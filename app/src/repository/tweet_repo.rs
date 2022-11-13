use crate::domain::interface::*;
use crate::domain::model::*;
use crate::error::*;
use crate::infra::DBConnector;
use crate::schema::tweet_records;
use async_trait::async_trait;
use diesel::dsl::*;
use diesel::prelude::*;

#[derive(Queryable, Insertable, Identifiable)]
pub struct TweetRecord {
    id: String,
    text: String,
    author_id: String,
    created_at: String,
    entities: String,
    geo: Option<String>,
    in_reply_to_user_id: Option<String>,
    lang: String,
    possibly_sensitive: Option<bool>,
    referenced_tweets: Option<String>,
    source: String,
    withheld: Option<String>,
}

impl TweetRecord {
    pub fn to_model(self) -> Result<Tweet> {
        let tweet = Tweet {
            id: TweetID(self.id),
            text: self.text,
            author_id: self.author_id,
            created_at: self.created_at,
            entities: self.entities,
            geo: self.geo,
            in_reply_to_user_id: self.in_reply_to_user_id,
            lang: self.lang,
            possibly_sensitive: self.possibly_sensitive,
            referenced_tweets: self.referenced_tweets,
            source: self.source,
            withheld: self.withheld,
        };
        Ok(tweet)
    }

    pub fn from_model(tweet: Tweet) -> Result<Self> {
        let tweet_record = TweetRecord {
            id: tweet.id.0.clone(),
            text: tweet.text.clone(),
            author_id: tweet.author_id.clone(),
            created_at: tweet.created_at.clone(),
            entities: tweet.entities.clone(),
            geo: tweet.geo.clone(),
            in_reply_to_user_id: tweet.in_reply_to_user_id.clone(),
            lang: tweet.lang.clone(),
            possibly_sensitive: tweet.possibly_sensitive.clone(),
            referenced_tweets: tweet.referenced_tweets.clone(),
            source: tweet.source.clone(),
            withheld: tweet.withheld.clone(),
        };
        Ok(tweet_record)
    }
}

pub struct TweetRepository {
    db: DBConnector,
}

impl TweetRepository {
    pub fn new(db: DBConnector) -> Self {
        Self { db }
    }

    pub fn get_table() -> tweet_records::table {
        tweet_records::table
    }
}

#[async_trait]
impl ITweetRepository for TweetRepository {
    async fn find_by_id(&self, id: &TweetID) -> Result<Tweet> {
        let record = self
            .db
            .first::<TweetRecord, _>(
                tweet_records::table.filter(tweet_records::id.eq(id.0.clone())),
            )
            .await?;
        record.to_model()
    }

    async fn save(&self, tweet: Tweet) -> Result<()> {
        let record = TweetRecord::from_model(tweet)?;
        self.db
            .execute(replace_into(tweet_records::table).values::<TweetRecord>(record))
            .await?;
        Ok(())
    }

    async fn search(&self, query: &str) -> Result<Vec<Tweet>> {
        let records = self
            .db
            .load::<TweetRecord, _>(
                tweet_records::table.filter(tweet_records::text.like(format!("%{}%", query))),
            )
            .await?;
        records
            .into_iter()
            .map(|record| record.to_model())
            .collect::<Result<Vec<Tweet>>>()
    }
}
