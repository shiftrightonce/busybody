//! # Busybody
//!
//! **Busybody is a service container and dependency injector for rust application.**
//! ---
//!
//! ## Busybody as a service container
//!
//! <details>
//! <summary>
//!   Service container example
//! </summary>
//!
//! ```rust
//! use busybody::*;
//!
//! #[derive(Debug)]
//! struct Config {
//!   hostname: String
//! }
//!
//! #[tokio::main]
//! async fn main() {
//!   let container = ServiceContainerBuilder::new()
//!   .service(Config{ hostname: "http://localhost".into() }) // Will be wrapped in Service<T> ie: Arc<T>
//!   .await
//!   .register(600i32) // left as it is, i32
//!   .await
//!   .build();
//!
//!   let config = container.get::<Config>().await.unwrap(); // Return Service<Config> ie: Arc<T>
//!   let max_connection = container.get_type::<i32>().await.unwrap(); // Return i32
//!
//!   println!("config: {:#?}", &config);
//!   println!("hostname: {:#?}", &config.hostname);
//!   println!("max connection: {}", max_connection);
//! }
//! ```
//!
//! </details>
//!
//! ## Busybody as a dependency injector
//!
//! <details>
//!   <summary>
//!      Service Resolver
//!   </summary>
//!
//!   ```rust
//! use std::sync::Arc;
//!
//! use busybody::*;
//!
//! #[derive(Debug, Clone)]
//! struct Config {
//!     hostname: String,
//! }
//!
//! #[tokio::main]
//! async fn main() {
//!     // Instantiate the http client once
//!     helpers::resolvable_once::<Arc<HttpClient>>().await; // HttpClient implements Resolver
//!
//!     // helpers::resolvable::<Arc<HttpClient>>().await; // Resolves the instance each time one is required
//!     // helpers::soft_resolvable::<Arc<HttpClient>>().await; // Register a resolver if one does not exist
//!
//!     // Whenever an instance of Config is needed
//!     // this closure will be called
//!     helpers::resolver(|_container| async move {
//!             Config {
//!                 hostname: "127.0.0.1".to_string(),
//!             }
//!     })
//!     .await;
//!
//!     let _config: Config = helpers::get_type().await.unwrap(); // Resolve an instance of Config
//!
//!     helpers::resolve_and_call(send_invoices).await; // Resolve all the parameters of "send_invoices" and call it.
//!
//!     // You can pass a closure and also return a value
//!     let result = helpers::resolve_and_call(async |client: Service<HttpClient>, config: Config| {
//!         println!("config: {:?}", config);
//!         client.get()
//!     })
//!     .await;
//!     println!("got invoices: {}", result);
//! }
//!
//! async fn send_invoices(http_client: Arc<HttpClient>) {
//!     http_client.post();
//! }
//!
//! struct HttpClient {
//!     config: Config,
//! }
//!
//! impl HttpClient {
//!     pub fn post(&self) {
//!         println!("sending invoices to: {}", self.config.hostname);
//!     }
//!
//!     pub fn get(&self) -> bool {
//!         println!("fetching invoices from: {}", self.config.hostname);
//!         true
//!     }
//! }
//!
//! #[async_trait::async_trait]
//! impl Resolver for HttpClient {
//!     async fn resolve(container: &ServiceContainer) -> Self {
//!         Self {
//!             config: container.get_type().await.unwrap(),
//!         }
//!     }
//! }
//!
//! ```
//!
//! </details>
//! <details>
//!   <summary>
//!     Dependency injection example
//!   </summary>
//!
//! ```rust
//! use busybody::*;
//!
//! #[derive(Debug, Clone)]
//! struct Config {
//!     hostname: String,
//! }
//!
//! #[busybody::async_trait]
//! impl busybody::Resolver for Config {
//!     async fn resolve(_: &ServiceContainer) -> Self {
//!         Self {
//!             hostname: "localhost".into(),
//!         }
//!     }
//! }
//!
//! #[tokio::main]
//! async fn main() {
//!     busybody::helpers::resolvable::<Config>().await;
//!     busybody::helpers::service_container()
//!         .resolvable::<Config>() // same as above
//!         .await
//!         .set_type(44_i64)
//!         .await
//!         .set_type(32_i32)
//!         .await
//!         .set_type(22.84_f32)
//!         .await;
//!
//!     let (float, int32, int64, config) = helpers::resolve_all::<(f32, i32, i64, Config)>().await;
//!
//!     println!("float: {}", float);
//!     println!("int32: {}", int32);
//!     println!("int64: {}", int64);
//!     println!("config: {:#?}", &config);
//!     println!("hostname: {:#?}", &config.hostname);
//! }
//! ```
//!
//! </details>
//!
//! <details>
//!   <summary>
//!     Dependency injection: singleton example
//!   </summary>
//!
//! ```rust
//! use std::time::SystemTime;
//!
//! use busybody::*;
//!
//! #[derive(Debug, Clone)]
//! struct Config {
//!     uptime: SystemTime,
//! }
//!
//! #[tokio::main]
//! async fn main() {
//!     helpers::resolver_once(|_| async {
//!             Config {
//!                 uptime: SystemTime::now(),
//!             }
//!     })
//!     .await;
//!
//!     for _ in 0..=20 {
//!         let config = busybody::helpers::get_type::<Config>().await.unwrap();
//!         println!("uptime: {:?}", config.uptime);
//!     }
//! }
//!
//! ```
//!
//! </details>
//!
//! <details>
//!   <summary>
//!     Dependency injection: call a function/closure passing it all the require arguments
//!   </summary>
//!
//! ```rust
//! use busybody::{helpers, Service, ServiceContainerBuilder};
//!
//! #[tokio::main]
//! async fn main() {
//!     // 1. Setup the container
//!     _ = ServiceContainerBuilder::new()
//!         .register(200) // Register an i32 value that is not wrapped in Service<T>
//!         .await
//!         .service(400) // Register an i32 value that is wrapped in Service<T>
//!         .await
//!         .build();
//!
//!     // 2. `resolve_and_call` calls the provided function/closure, injecting all of it's required parameters
//!     //     resolve_and_call takes a function/closure that expects 0 to 17 arguments
//!     //     The function **must** be async
//!     let double_result = helpers::resolve_and_call(double).await;
//!     println!("200 double is: {}", double_result);
//!
//!     // 3. Same as above but we are making use of a Service<T> ie Arc<T>
//!     //    it uses the `default` associate method to create a default instance of the Type.
//!     let sum = helpers::resolve_and_call(|raw_i32: i32, service_i32: Service<i32>| async move {
//!         raw_i32 + *service_i32
//!     })
//!     .await;
//!     println!("Service<200> + 400 = {}", sum);
//! }
//!
//! // 4. Function is taken an I32.
//! async fn double(count: i32) -> i32 {
//!     count * 2
//! }
//!
//! ```
//!
//! </details>
//!

mod container;
mod handlers;
mod resolver;
mod service;

pub mod helpers;

pub use container::ServiceContainer;
pub use container::ServiceContainerBuilder;
pub use handlers::*;
pub use resolver::Resolver;
pub use service::Service;

pub use async_trait::async_trait;
