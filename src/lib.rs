//! Busybody is a service container and dependency injector.
//! It is thread safe and async first. By default instances of types that are registered
//! are wrapped in a `Service` type. Service wraps it's inner type in a ARC.
//!
mod container;
mod handlers;
mod injectable;
mod provider;
mod service;
mod singleton;

pub mod helpers;

pub use container::ServiceContainer;
pub use container::ServiceContainerBuilder;
pub use handlers::*;
pub use injectable::Injectable;
pub use service::Service;
pub use singleton::Singleton;
