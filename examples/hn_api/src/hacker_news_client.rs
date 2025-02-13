use crate::Config;
use async_trait::async_trait;
use std::fmt::Debug;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Story {
    by: String,
    id: u32,
    descendants: u32,
    kids: Vec<u32>,
    score: u32,
    time: u32,
    title: String,
    #[serde(rename(serialize = "type", deserialize = "type"))]
    type_: String,
    url: String,
}

#[async_trait]
pub trait HackerNewsClientTrait: Debug + Send + Sync {
    async fn fetch_top_stories(&self) -> Option<Vec<u32>>;
    async fn fetch_story(&self, id: u32) -> Option<Story>;
}

#[derive(Debug)]
pub struct HackerNewsClient {
    config: Config,
}

impl HackerNewsClient {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

#[busybody::async_trait]
impl busybody::Injectable for HackerNewsClient {
    async fn inject(container: &busybody::ServiceContainer) -> Self {
        let config = if let Some(config) = container.get_type::<Config>().await {
            config
        } else {
            busybody::helpers::provide::<Config>().await
        };

        Self::new(config)
    }
}

#[async_trait]
impl HackerNewsClientTrait for HackerNewsClient {
    async fn fetch_top_stories(&self) -> Option<Vec<u32>> {
        if let Ok(result) = reqwest::get(&self.config.top_stories_endpoint).await {
            result.json::<Vec<u32>>().await.ok()
        } else {
            None
        }
    }

    async fn fetch_story(&self, id: u32) -> Option<Story> {
        if let Ok(result) =
            reqwest::get(self.config.story_endpoint.replace(":id", &id.to_string())).await
        {
            result.json::<Story>().await.ok()
        } else {
            None
        }
    }
}
