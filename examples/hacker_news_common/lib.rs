use async_trait::async_trait;
use hacker_news_cache::HackerNewsCacheClient;
use hacker_news_client::{HackerNewsClient, HackerNewsClientTrait, Story};
pub mod hacker_news_cache;
pub mod hacker_news_client;

#[derive(Debug, Clone)]
pub struct Config {
    pub top_stories_endpoint: String,
    pub story_endpoint: String,
    pub enable_caching: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            top_stories_endpoint:
                "https://hacker-news.firebaseio.com/v0/topstories.json?print=pretty".into(),
            story_endpoint: "https://hacker-news.firebaseio.com/v0/item/:id.json?print=pretty"
                .into(),
            enable_caching: false,
        }
    }
}

#[busybody::async_trait]
impl busybody::Resolver for Config {
    async fn resolve(container: &busybody::ServiceContainer) -> Self {
        if let Some(config) = container.get_type::<Self>().await {
            config
        } else {
            let conf = Self::default();
            container.set_type(conf.clone()).await;
            conf
        }
    }
}

#[derive(Debug)]
pub struct HackerNewsClientProvider(Box<dyn HackerNewsClientTrait>);

impl HackerNewsClientProvider {
    pub fn new<T: HackerNewsClientTrait + 'static>(client: T) -> Self {
        Self(Box::new(client))
    }
}

#[async_trait]
impl HackerNewsClientTrait for HackerNewsClientProvider {
    async fn fetch_top_stories(&self) -> Option<Vec<u32>> {
        self.0.fetch_top_stories().await
    }

    async fn fetch_story(&self, id: u32) -> Option<Story> {
        self.0.fetch_story(id).await
    }
}

#[busybody::async_trait]
impl busybody::Resolver for HackerNewsClientProvider {
    async fn resolve(container: &busybody::ServiceContainer) -> Self {
        let config = container.get_type::<Config>().await.unwrap();
        if config.enable_caching {
            Self::new(HackerNewsCacheClient::resolve(container).await)
        } else {
            Self::new(HackerNewsClient::resolve(container).await)
        }
    }
}
