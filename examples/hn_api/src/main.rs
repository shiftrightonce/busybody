use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use busybody::helpers::service_container;
use hn_api::{hacker_news_client::HackerNewsClientTrait, Config, HackerNewsClientProvider};

mod hacker_news_cache;
mod hacker_news_client;

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
