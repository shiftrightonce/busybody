#![allow(dead_code)]

use std::sync::Arc;

use crate::{
    container::SERVICE_CONTAINER, handlers::Handler, injectable::Injectable,
    provider::ServiceProvider, service::Service, ServiceContainer, ServiceContainerBuilder,
    Singleton,
};

/// Takes an async function or closure and executes it
/// Require arguments are injected during the call
pub async fn inject_and_call<F, Args>(handler: F) -> F::Output
where
    F: Handler<Args>,
    Args: Injectable + 'static,
{
    let args = Args::inject(&service_container()).await;
    handler.call(args).await
}

/// Given a tuple of types, this function will try to resolve them
/// and return a tuple of instances
pub async fn inject_all<Args>() -> Args
where
    Args: Injectable + 'static,
{
    Args::inject(&service_container()).await
}

/// Given a type, this function will try to call the `inject` method
/// implemented by the type
pub async fn provide<T: Injectable + Send + Sync + 'static>() -> T {
    ServiceProvider::provide().await
}

/// Given a type, this function will try to find an instance of the type
/// wrapped in a `Service<T>` that is currently registered in the service
/// container
///
/// Example
pub async fn service<T: 'static>() -> Service<T> {
    ServiceProvider::service().await
}

/// Given a type, this function will try to find an existing instance of the
/// type in the service container. If that fails, an instance of the type is
/// initialized, wrapped in a `Service`, stored in the service container and
/// a copy is returned. Subsequent call requesting instance of that type will
/// return the instance in the service container.
pub async fn singleton<T: Injectable + Sized + Send + Sync + 'static>() -> Singleton<T> {
    Singleton::inject(&service_container()).await
}

/// Returns the service container instance
pub fn service_container() -> Arc<ServiceContainer> {
    if let Some(container) = SERVICE_CONTAINER.get() {
        container.clone()
    } else {
        ServiceContainerBuilder::new().build()
    }
}
