use crate::domain::model::*;
use crate::error::Result;
use async_trait::async_trait;

#[async_trait]
pub trait ITweetRepository {
    async fn find_by_id(&self, id: &TweetID) -> Result<Tweet>;
    async fn save(&self, tweet: Tweet) -> Result<()>;
    async fn search(&self, query: &str) -> Result<Vec<Tweet>>;
}
