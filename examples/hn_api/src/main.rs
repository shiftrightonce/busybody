use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use async_trait::async_trait;
use busybody::helpers::service_container;
use hacker_news_client::{HackerNewsClientTrait, Story};

mod hacker_news_cache;
mod hacker_news_client;

#[derive(Debug)]
pub struct HackerNewsClientProvider(Box<dyn HackerNewsClientTrait>);

impl HackerNewsClientProvider {
    pub fn new<T: HackerNewsClientTrait + 'static>(client: T) -> Self {
        Self(Box::new(client))
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Config {
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

#[busybody::async_trait(?Send)]
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

#[async_trait]
impl HackerNewsClientTrait for HackerNewsClientProvider {
    async fn fetch_top_stories(&self) -> Option<Vec<u32>> {
        self.0.fetch_top_stories().await
    }

    async fn fetch_story(&self, id: u32) -> Option<Story> {
        self.0.fetch_story(id).await
    }
}

#[busybody::async_trait(?Send)]
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 1. By default, "enable_caching" is disabled in the config instance
    //    Passing "cache" when you starting the application will enable caching
    if let Some(arg) = std::env::args().nth(1) {
        if arg == "cache" {
            let config = Config {
                enable_caching: true,
                ..Config::default()
            };
            service_container().set_type(config);
        }
    }

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(top_stories))
            .route("/item/{id}", web::get().to(a_story))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

async fn top_stories() -> impl Responder {
    // 2. Inject the client provider
    let client = busybody::helpers::singleton::<HackerNewsClientProvider>().await;

    HttpResponse::Ok().json(client.fetch_top_stories().await)
}
async fn a_story(id: web::Path<u32>) -> impl Responder {
    // 2. Inject the client provider
    let client = busybody::helpers::singleton::<HackerNewsClientProvider>().await;

    HttpResponse::Ok().json(client.fetch_story(*id).await)
}
