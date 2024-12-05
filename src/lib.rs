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
//! #[derive(Debug, Clone)]
//! struct Config {
//!   hostname: String
//! }
//!
//! #[busybody::async_trait]
//! impl busybody::Injectable for Config { // implementing "injector" makes your type injectable
//!
//!    async fn inject(_: &ServiceContainer) -> Self {
//!       Self {
//!           hostname: "localhost".to_string()
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
//! ```
//!
//! # Dependency injection: call a function/closure passing it all the require arguments
//!
//! ```rust
//! use busybody::{helpers, RawType, Service, ServiceContainerBuilder};
//!
//! #[tokio::main]
//! async fn main() {
//!    // 1. Setup the container
//!    _ = ServiceContainerBuilder::new()
//!        .register(200) // Register an i32 value that is not wrapped in Service<T>
//!        .service(400) // Register an i32 value that is wrapped in Service<T>
//!        .build();
//!
//!    // 2. `inject_and_call` calls the provided function/closure, injecting all of it's required parameters
//!     //     inject_and_call takes a function/closure that expects 0 to 17 arguments
//!     //     The function **must** be async
//!     let double_result = helpers::inject_and_call(double).await;
//!     println!("200 double is: {}", double_result);
//!
//!     // 3. Same as above but we are making use of "RawType<T>"
//!     //    RawType<T> tries to find an instance of the specified type. If none exist,
//!     //    it uses the `default` associate method to create a default instance of the Type.
//!     //    This means, the "T" in RawType must implement the `Default` trait.
//!     let sum = helpers::inject_and_call(|raw_i32: RawType<i32>, service_i32: Service<i32>| async {
//!        raw_i32.into_inner() + *service_i32.into_inner()
//!     })
//!     .await;
//!     println!("Service<200> + RawType<400> = {}", sum);
//! }
//!
//! // 4. Function is taken an I32.
//! async fn double(count: i32) -> i32 {
//!     count * 2
//! }
//!
//! ```
//!
//! # Dependency injection: singleton example
//!
//! ```rust
//! use busybody::*;
//!
//! #[derive(Debug)]
//! struct Config {
//!   hostname: String
//! }
//!
//! #[busybody::async_trait]
//! impl busybody::Injectable for Config {
//!
//!     async fn inject(_: &ServiceContainer) -> Self {
//!        Self {
//!            hostname: "localhost".into()
//!        }
//!     }
//! }
//!
//! #[tokio::main]
//! async fn main() {
//!   let config = helpers::singleton::<Config>().await;
//!
//!   println!("config: {:#?}", &config);
//!   println!("hostname: {:#?}", &config.hostname);
//! }
//!
//! ```
//!
mod container;
mod handlers;
mod injectable;
mod resolver;
mod service;
mod singleton;

pub mod helpers;

pub use container::ServiceContainer;
pub use container::ServiceContainerBuilder;
pub use handlers::*;
pub use injectable::Injectable;
pub use resolver::Resolver;
pub use service::RawType;
pub use service::Service;
pub use singleton::Singleton;

pub use async_trait::async_trait;
