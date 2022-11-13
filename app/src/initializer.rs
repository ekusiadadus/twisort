use crate::domain::service;
use crate::infra;
use crate::repository;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Config {
    pub db_url: String,
    pub db_pool_size: u32,
    pub tweets_table_name: String,
    pub bearer_token: String,
}

#[derive(Clone)]
pub struct Infras {
    pub db: infra::DBConnector,
}
impl Infras {
    pub async fn ensure_initialized(&self) -> Option<()> {
        if self.db.ensure_initialized().await.is_ok() {
            Some(())
        } else {
            None
        }
    }
}

pub async fn infras(config: &Config) -> Infras {
    let db_executor = infra::DBExecutor::new(config.db_url.clone(), config.db_pool_size);
    let db_connector = infra::DBConnector::new(db_executor);
    Infras { db: db_connector }
}

#[derive(Clone)]

pub struct Repository {
    pub tweet: Arc<repository::TweetRepository>,
}

pub fn repository(config: &Config, infras: &Infras) -> Repository {
    let tweet = Arc::new(repository::TweetRepository::new(infras.db.clone()));
    Repository { tweet }
}

#[derive(Clone)]
pub struct Services {
    pub tweet: service::TweetService,
}

#[derive(Clone)]
pub struct AppContext {
    pub infras: Infras,
    pub repository: Repository,
    pub services: Services,
}

pub async fn new(config: Config) -> AppContext {
    let infras = infras(&config).await;
    let repository = repository(&config, &infras);
    let services = Services {
        tweet: service::TweetService::new(repository.tweet.clone()),
    };
    AppContext {
        infras,
        repository,
        services,
    }
}
