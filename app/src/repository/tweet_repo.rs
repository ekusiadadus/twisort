use crate::domain::interface::*;
use crate::domain::model::*;
use crate::error::*;
use crate::infra::DBConnector;
use crate::schema::tweet_records;
use async_trait::async_trait;
use diesel::dsl::*;
use diesel::prelude::*;
use serde::*;
use std::sync::Arc;

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
    bigquery: bool,
}

impl TweetRecord {
    pub fn to_model(&self) -> Result<Tweet> {
        Ok(Tweet::new(
            self.id.clone(),
            self.text.clone(),
            self.author_id.clone(),
            self.created_at.clone(),
            serde_json::from_str(&self.entities.clone()).unwrap_or_default(),
            self.geo
                .clone()
                .map(|geo| serde_json::from_str(&geo).unwrap()),
            self.in_reply_to_user_id.clone(),
            Some(self.lang.clone()),
            self.possibly_sensitive,
            self.referenced_tweets
                .clone()
                .map(|referenced_tweets| serde_json::from_str(&referenced_tweets).unwrap()),
            Some(self.source.clone()),
            self.withheld
                .clone()
                .map(|withheld| serde_json::from_str(&withheld).unwrap()),
        ))
    }

    pub fn from_model(tweet: Tweet) -> Result<Self> {
        let entities = serde_json::to_string(&tweet.entities).unwrap();
        let geo = tweet.geo.map(|geo| serde_json::to_string(&geo).unwrap());
        let referenced_tweets = tweet
            .referenced_tweets
            .map(|referenced_tweets| serde_json::to_string(&referenced_tweets).unwrap());
        let withheld = tweet
            .withheld
            .map(|withheld| serde_json::to_string(&withheld).unwrap());
        Ok(TweetRecord {
            id: tweet.id,
            text: tweet.text,
            author_id: tweet.author_id,
            created_at: tweet.created_at,
            entities,
            geo,
            in_reply_to_user_id: tweet.in_reply_to_user_id,
            lang: tweet.lang.unwrap(),
            possibly_sensitive: tweet.possibly_sensitive,
            referenced_tweets,
            source: tweet.source.unwrap_or_default(),
            withheld,
            bigquery: false,
        })
    }
}

pub struct TweetRepository {
    db: DBConnector,
    http_client: Arc<dyn IHttpClient + Sync + Send>,
    bearer_token: String,
}

impl TweetRepository {
    pub fn new(
        db: DBConnector,
        http_client: Arc<dyn IHttpClient + Sync + Send>,
        bearer_token: String,
    ) -> Self {
        Self {
            db,
            http_client,
            bearer_token,
        }
    }
}

#[derive(Clone, Deserialize, Serialize, Default)]

pub struct TweetResponse {
    data: Vec<Tweet>,
    meta: Option<TweetResponseMeta>,
}

#[derive(Clone, Deserialize, Serialize, Default)]
pub struct TweetResponseMeta {
    pub newest_id: String,
    pub oldest_id: String,
    pub result_count: i64,
    pub next_token: Option<String>,
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

    async fn get_tweets_by_hashtag(&self, hashtag: &str) -> Result<Vec<Tweet>> {
        // remove retweets
        let tweet_fileds = "tweet.fields=author_id,created_at,entities,geo,in_reply_to_user_id,lang,possibly_sensitive,referenced_tweets,source,text,withheld&max_results=10";
        let uri = "https://api.twitter.com/2/tweets/search/recent?query=ekusiadadus -is: retweet"
            .to_string()
            + "&"
            + tweet_fileds;
        let uri = uri.replace("ekusiadadus", hashtag);
        let mut headers = reqwest::header::HeaderMap::new();
        // add bearer_token
        let bearer_token = format!("Bearer {}", self.bearer_token);
        headers.insert(
            reqwest::header::AUTHORIZATION,
            bearer_token.parse().unwrap(),
        );
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            "application/json".parse().unwrap(),
        );
        let response = self.http_client.get(&uri, Some(headers)).await.unwrap();

        let body = response.text().await.unwrap();

        print!("{}", body);

        let tweets = serde_json::from_str::<TweetResponse>(&body).unwrap();

        print!("{:?}", tweets.data);

        Ok(tweets.data)
    }

    async fn save(&self, tweet: Tweet) -> Result<()> {
        let record = TweetRecord::from_model(tweet)?;
        self.db
            .execute(replace_into(tweet_records::table).values::<TweetRecord>(record))
            .await?;
        Ok(())
    }

    async fn save_tweets(&self, tweets: Vec<Tweet>) -> Result<()> {
        let records = tweets
            .into_iter()
            .map(TweetRecord::from_model)
            .collect::<Result<Vec<TweetRecord>>>()?;
        for record in records {
            self.db
                .execute(replace_into(tweet_records::table).values::<TweetRecord>(record))
                .await?;
        }
        Ok(())
    }

    async fn delete(&self, id: &TweetID) -> Result<()> {
        self.db
            .execute(delete(tweet_records::table).filter(tweet_records::id.eq(id.0.clone())))
            .await?;
        Ok(())
    }

    async fn delete_tweet(&self, id: &TweetID) -> Result<()> {
        // delete tweet from twitter
        let uri = format!("https://api.twitter.com/2/tweets/{}", id.0);
        let mut headers = reqwest::header::HeaderMap::new();
        // add bearer_token

        let bearer_token = format!(
            "
        Bearer {}",
            self.bearer_token
        );
        headers.insert(
            reqwest::header::AUTHORIZATION,
            bearer_token.parse().unwrap(),
        );
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            "application/json".parse().unwrap(),
        );
        let response = self.http_client.delete(&uri, Some(headers)).await.unwrap();

        let body = response.text().await.unwrap();

        print!("{}", body);

        Ok(())
    }

    async fn favorite_tweet(&self, id: &TweetID) -> Result<()> {
        // favorite tweet from twitter
        let uri = format!("https://api.twitter.com/2/tweets/{}/like", id.0);
        let mut headers = reqwest::header::HeaderMap::new();
        // add bearer_token

        let bearer_token = format!(
            "
        Bearer {}",
            self.bearer_token
        );
        headers.insert(
            reqwest::header::AUTHORIZATION,
            bearer_token.parse().unwrap(),
        );
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            "application/json".parse().unwrap(),
        );

        let response = self
            .http_client
            .post(&uri, Some(headers), None)
            .await
            .unwrap();

        let body = response.text().await.unwrap();

        print!("{}", body);

        Ok(())
    }

    async fn post(&self, tweet: Tweet) -> Result<()> {
        let url = "https://api.twitter.com/2/tweets".to_string();
        let mut headers = reqwest::header::HeaderMap::new();
        let bearer_token = format!(
            "
            Bearer {}",
            self.bearer_token
        );

        headers.insert(
            reqwest::header::AUTHORIZATION,
            bearer_token.parse().unwrap(),
        );

        headers.insert(
            reqwest::header::CONTENT_TYPE,
            "application/json".parse().unwrap(),
        );

        let body = serde_json::to_string(&tweet).unwrap();

        let response = self
            .http_client
            .post(&url, Some(headers), Some(body))
            .await
            .unwrap();

        let body = response.text().await.unwrap();

        print!("{}", body);

        Ok(())
    }
}
