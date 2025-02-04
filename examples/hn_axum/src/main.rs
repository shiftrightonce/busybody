use axum::{extract::Path, routing::get, Json, Router};
use busybody::helpers::service_container;
use hn_api::{hacker_news_client::HackerNewsClientTrait, HackerNewsClientProvider};

#[tokio::main]
async fn main() {
    // 1. By default, "enable_caching" is disabled in the config instance
    //    Passing "cache" when you starting the application will enable caching
    if let Some(arg) = std::env::args().nth(1) {
        if arg == "cache" {
            let config = hn_api::Config {
                enable_caching: true,
                ..hn_api::Config::default()
            };
            service_container().set_type(config);
        }
    }

    // 2. build our application with a route
    let app = Router::new()
        .route("/", get(top_stories))
        .route("/item/{id}", get(get_story));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn top_stories() -> Json<Vec<u32>> {
    // 3. Inject hacker news client provider
    let list = busybody::helpers::singleton::<HackerNewsClientProvider>()
        .await
        .fetch_top_stories()
        .await
        .unwrap();

    Json(list)
}

// basic handler that responds with a static string
async fn get_story(Path(id): Path<u32>) -> Json<hn_api::hacker_news_client::Story> {
    // 3. Inject hacker news client provider
    let story = busybody::helpers::singleton::<HackerNewsClientProvider>()
        .await
        .fetch_story(id)
        .await
        .unwrap();
    Json(story)
}
