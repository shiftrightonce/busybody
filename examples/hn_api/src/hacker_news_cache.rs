use crate::hacker_news_client::{HackerNewsClient, HackerNewsClientTrait, Story};
use async_trait::async_trait;
use std::{collections::HashMap, sync::RwLock};

#[derive(Debug)]
pub(crate) struct HackerNewsCacheClient {
    top_stories: RwLock<Vec<u32>>,
    stories: RwLock<HashMap<u32, Story>>,
    client: busybody::Singleton<HackerNewsClient>,
}

impl HackerNewsCacheClient {
    pub fn new(client: busybody::Singleton<HackerNewsClient>) -> Self {
        Self {
            top_stories: RwLock::new(Vec::new()),
            stories: RwLock::new(HashMap::new()),
            client,
        }
    }
}

#[busybody::async_trait(?Send)]
impl busybody::Injectable for HackerNewsCacheClient {
    async fn inject(_: &busybody::ServiceContainer) -> Self {
        Self::new(busybody::helpers::singleton::<HackerNewsClient>().await)
    }
}

#[async_trait]
impl HackerNewsClientTrait for HackerNewsCacheClient {
    async fn fetch_top_stories(&self) -> Option<Vec<u32>> {
        if let Ok(stories) = self.top_stories.read() {
            if !stories.is_empty() {
                return Some(stories.clone());
            }
        }

        if let Some(mut stories) = self.client.fetch_top_stories().await {
            if let Ok(mut cache) = self.top_stories.write() {
                cache.append(&mut stories);
                return Some(cache.clone());
            }
        }

        None
    }

    async fn fetch_story(&self, id: u32) -> Option<Story> {
        if let Ok(cache) = self.stories.read() {
            if let Some(story) = cache.get(&id) {
                return Some(story.clone());
            }
        }

        match self.client.fetch_story(id).await {
            Some(story) => {
                if let Ok(mut cache) = self.stories.write() {
                    cache.insert(id, story.clone());
                }
                Some(story)
            }
            None => None,
        }
    }
}
