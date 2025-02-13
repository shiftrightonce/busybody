use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use busybody::{helpers::service_container, ServiceContainerBuilder};
use chrono::prelude::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 1. Initialize the service container by using the builder
    _ = ServiceContainerBuilder::new()
        // 2. Register an instance of "ServerUptime" as a service ie: Service<ServerUptime>
        .service(ServerUptime::new())
        .await
        .build();

    // 3. Setup actix web application
    HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(uptime))
            .route("/two", web::get().to(uptime2))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

async fn uptime() -> impl Responder {
    // 4. Get the service container via the function `service_container`
    let time_keeper = service_container().get::<ServerUptime>().await.unwrap();
    HttpResponse::Ok().content_type("text/html").body(format!(
        "<h1><center>Up time<br/>{}</center></h1>",
        time_keeper.duration()
    ))
}
async fn uptime2() -> impl Responder {
    // 5. Or get the ServerUptime service by using the helper function "service"
    let time_keeper = busybody::helpers::service::<ServerUptime>().await;

    HttpResponse::Ok().content_type("text/html").body(format!(
        "<h1><center>Up time<br/>{}</center></h1>",
        time_keeper.duration()
    ))
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
