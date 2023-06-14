use async_trait::async_trait;
use hacker_news_client::{HackerNewsClientTrait, Story};

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
impl busybody::Injectable for Config {
    async fn inject(container: &busybody::ServiceContainer) -> Self {
        if let Some(config) = container.get_type::<Self>() {
            config
        } else {
            let conf = Self::default();
            container.set_type(conf.clone());
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
impl busybody::Injectable for HackerNewsClientProvider {
    async fn inject(_: &busybody::ServiceContainer) -> Self {
        let config = busybody::helpers::provide::<Config>().await;
        if config.enable_caching {
            Self(Box::new(
                busybody::helpers::provide::<hacker_news_cache::HackerNewsCacheClient>().await,
            ))
        } else {
            Self(Box::new(
                busybody::helpers::provide::<hacker_news_client::HackerNewsClient>().await,
            ))
        }
    }
}
