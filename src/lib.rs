//! Busybody is a service container and dependency injector.
//! It is thread safe and async first. By default instances of types that are registered
//! are wrapped in a `Service` type. Service wraps it's inner type in a ARC.
//!
//! # Container setup example
//! ```rust
//! use busybody::*;
//!
//! #[derive(Debug)]
//! struct Config {
//!  hostname: String
//!}
//!
//! fn main() {
//!  let container = ServiceContainerBuilder::new()
//!  .service(Config{ hostname: "http://localhost".into() }) // Will be wrapped in Service<T> ie: Arc<T>
//!  .register(600i32) // left as it is, i32
//!  .build();
//!
//!  let config = container.get::<Config>().unwrap(); // When "some" will return Service<Config>
//!  let max_connection = container.get_type::<i32>().unwrap(); // When "some" will return i32
//!
//!  println!("config: {:#?}", &config);
//!  println!("hostname: {:#?}", &config.hostname);
//!  println!("max connection: {}", max_connection);
//! }
//! ```
//! # Dependency injection example
//!
//! ```rust
//! use busybody::*;
//! use async_trait::async_trait;
//!
//! #[derive(Debug)]
//! struct Config {
//!   hostname: String
//! }
//!
//! #[busybody::async_trait(?Send)]
//! impl busybody::Injectable for Config { // implementing "injector" makes your type injectable
//!
//!    async fn inject(_: &ServiceContainer) -> Self {
//!       Self {
//!           hostname: "localhost".into()
//!       }
//!    }
//!}
//!
//! #[tokio::main]
//! async fn main() {
//!  let config = helpers::provide::<Config>().await;
//!
//!   println!("config: {:#?}", &config);
//!   println!("hostname: {:#?}", &config.hostname);
//! }
//!
mod container;
mod handlers;
mod injectable;
mod service;
mod singleton;

pub mod helpers;

pub use container::ServiceContainer;
pub use container::ServiceContainerBuilder;
pub use handlers::*;
pub use injectable::Injectable;
pub use service::RawType;
pub use service::Service;
pub use singleton::Singleton;

pub use async_trait::async_trait;
