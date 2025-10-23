use axum::{Json, Router, extract::Path, routing::get};
use busybody::{Service, helpers::service_container};
use hacker_news_common::{
    Config, HackerNewsClientProvider,
    hacker_news_client::{self, HackerNewsClientTrait},
};

#[path = "../hacker_news_common/lib.rs"]
mod hacker_news_common;

#[tokio::main]
async fn main() {
    busybody::helpers::resolvable_once::<Service<HackerNewsClientProvider>>().await;

    // 1. By default, "enable_caching" is disabled in the config instance
    //    Passing "cache" when you starting the application will enable caching

    let mut config = Config::default();
    if let Some(arg) = std::env::args().nth(1) {
        if arg == "cache" {
            config = Config {
                enable_caching: true,
                ..Config::default()
            };
        }
    }
    service_container().set_type(config).await;

    // 2. build our application with a route
    let app = Router::new()
        .route("/", get(top_stories))
        .route("/item/{id}", get(get_story));

    println!("listening on port: 8080");

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn top_stories() -> Json<Vec<u32>> {
    // 3. Inject hacker news client provider
    let list = busybody::helpers::service::<HackerNewsClientProvider>()
        .await
        .fetch_top_stories()
        .await
        .unwrap();

    Json(list)
}

// basic handler that responds with a static string
async fn get_story(Path(id): Path<u32>) -> Json<hacker_news_client::Story> {
    // 3. Inject hacker news client provider
    let story = busybody::helpers::service::<HackerNewsClientProvider>()
        .await
        .fetch_story(id)
        .await
        .unwrap();
    Json(story)
}
