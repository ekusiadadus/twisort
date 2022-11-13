use crate::domain::model::*;
use crate::error::IServiceError;
use serde::*;

#[derive(Debug)]
pub enum TweetError {
    AccessDenied,
}

impl IServiceError for TweetError {
    fn error_type(&self) -> String {
        use TweetError::*;
        match self {
            AccessDenied => "access_denied",
        }
        .to_string()
    }

    fn status_code(&self) -> http::StatusCode {
        use TweetError::*;
        match self {
            AccessDenied => http::StatusCode::FORBIDDEN,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Tweet {
    pub id: TweetID,
    pub text: String,
    pub author_id: String,
    pub created_at: String,
    pub entities: String,
    pub geo: Option<String>,
    pub in_reply_to_user_id: Option<String>,
    pub lang: String,
    pub possibly_sensitive: Option<bool>,
    pub referenced_tweets: Option<String>,
    pub source: String,
    pub withheld: Option<String>,
}

impl Tweet {
    pub fn new(
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
    ) -> Self {
        Self {
            id: TweetID(id),
            text,
            author_id,
            created_at,
            entities,
            geo,
            in_reply_to_user_id,
            lang,
            possibly_sensitive,
            referenced_tweets,
            source,
            withheld,
        }
    }
}
