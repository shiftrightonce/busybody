use std::sync::Arc;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use busybody::ServiceContainer;
use chrono::prelude::*;
use rand::{self, Rng};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 1. Wrapping our shared resources in Arc
    //    Both of them implements Resolver
    busybody::helpers::resolvable_once::<Arc<ServerUptime>>().await;
    busybody::helpers::resolvable_once::<Arc<HandlerExecutionTime>>().await;

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(uptime))
            .route("/two", web::get().to(uptime2))
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}

#[derive(Debug)]
struct ServerUptime {
    started: DateTime<Utc>,
}

impl ServerUptime {
    pub fn new() -> Self {
        Self {
            started: Utc::now(),
        }
    }
    pub fn duration(&self) -> String {
        let now = Utc::now() - self.started;
        format!(
            "hours: {}, minutes: {}, seconds: {}",
            now.num_hours(),
            now.num_minutes() % 60,
            now.num_seconds() % 60
        )
    }
}

// 2. Implement "Resolver" for ServerUptime
#[busybody::async_trait]
impl busybody::Resolver for ServerUptime {
    async fn resolve(_: &ServiceContainer) -> Self {
        Self::new() // each time return a new instance
    }
}

// HandlerExecutionTime will log how long the server has been up
// and the time it took to complete a task in the request handler
struct HandlerExecutionTime {
    started: DateTime<Utc>,
    server_duration: String,
}

// 3. Implement "injectable" for HandlerExecutionTime
#[busybody::async_trait]
impl busybody::Resolver for HandlerExecutionTime {
    async fn resolve(container: &ServiceContainer) -> Self {
        // 4. Ask for a singleton instance of ServerUptime to be returned.
        //    If none exist, a new one will be created and returned
        let server_timer = container.get::<ServerUptime>().await.unwrap();
        Self {
            started: Utc::now(),
            server_duration: server_timer.duration(),
        }
    }
}

impl HandlerExecutionTime {
    pub fn duration(&self) -> String {
        let now = Utc::now() - self.started;
        format!(
            "server\'s uptime: {},<br/> handler execution time => hours: {}, minutes: {}, seconds: {}, milliseconds: {}",
            self.server_duration,
            now.num_hours(),
            now.num_minutes() % 60,
            now.num_seconds() % 60,
            now.num_milliseconds()
        )
    }
}

async fn uptime() -> impl Responder {
    // 5. Ask for an instance of HandlerExecutionTime to be created and provided
    println!(
        "has handler exec: {}",
        busybody::helpers::get_service::<HandlerExecutionTime>()
            .await
            .is_some()
    );
    let timer = busybody::helpers::get_service::<HandlerExecutionTime>()
        .await
        .unwrap();
    let mut rang = rand::rng();

    for _ in 0..rang.random_range(1..20000000) {
        // pretend we are doing something that could take some time....
    }

    HttpResponse::Ok()
        .content_type("text/html")
        .body(format!("<h1><center>{}</center></h1>", timer.duration()))
}
async fn uptime2() -> impl Responder {
    // 5. Ask for an instance of HandlerExecutionTime to be created and provided
    let timer = busybody::helpers::get_type::<Arc<HandlerExecutionTime>>()
        .await
        .unwrap();
    let mut rang = rand::rng();

    for _ in 0..rang.random_range(1..40000000) {
        // pretend we are doing something that could take some time....
    }

    HttpResponse::Ok()
        .content_type("text/html")
        .body(format!("<h1><center>{}</center></h1>", timer.duration()))
}
