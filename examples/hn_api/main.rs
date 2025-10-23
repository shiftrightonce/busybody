use std::sync::Arc;

#[path = "../hacker_news_common/lib.rs"]
mod hacker_news_common;

use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use busybody::helpers::service_container;
use hacker_news_common::{
    Config, HackerNewsClientProvider, hacker_news_client::HackerNewsClientTrait,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    busybody::helpers::resolvable_once::<Arc<HackerNewsClientProvider>>().await;

    // 1. By default, "enable_caching" is disabled in the config instance
    //    Pass "cache" when starting to enable caching
    let mut config = Config::default();
    if let Some(arg) = std::env::args().nth(1) {
        if arg == "cache" {
            config = Config {
                enable_caching: true,
                ..Config::default()
            };
        }
    }

    //
    service_container().set_type(config).await;

    println!("listening on port: 8080");

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
    let client = busybody::helpers::get_service::<HackerNewsClientProvider>()
        .await
        .unwrap();

    HttpResponse::Ok().json(client.fetch_top_stories().await)
}
async fn a_story(id: web::Path<u32>) -> impl Responder {
    // 2. Inject the client provider
    let client = busybody::helpers::get_service::<HackerNewsClientProvider>()
        .await
        .unwrap();

    HttpResponse::Ok().json(client.fetch_story(*id).await)
}
