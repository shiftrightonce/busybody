#![allow(dead_code)]

use std::sync::Arc;

use crate::{
    container::SERVICE_CONTAINER, handlers::Handler, injectable::Injectable, service::Service,
    ServiceContainer, ServiceContainerBuilder, Singleton,
};

/// Takes an async function or closure and executes it
/// Require arguments are injected during the call
/// The global service container is used for any resolving
pub async fn inject_and_call<F, Args>(handler: F) -> F::Output
where
    F: Handler<Args>,
    Args: Injectable + 'static,
{
    let args = Args::inject(&service_container()).await;
    handler.call(args).await
}

/// Takes an async function or closure and executes it.
/// Require arguments are injected during the call.
/// The container to use is provided by the caller.
pub async fn inject_and_call_with<F, Args>(ci: &ServiceContainer, handler: F) -> F::Output
where
    F: Handler<Args>,
    Args: Injectable + 'static,
{
    let args = Args::inject(ci).await;
    handler.call(args).await
}

/// Given a tuple of types, this function will try to resolve them
/// and return a tuple of instances.
/// The global service container is used.
pub async fn inject_all<Args>() -> Args
where
    Args: Injectable + 'static,
{
    Args::inject(&service_container()).await
}

/// Given a tuple of types, this function will try to resolve them
/// and return a tuple of instances.
/// The service container to used is provided by the caller.
pub async fn inject_all_with<Args>(container: &ServiceContainer) -> Args
where
    Args: Injectable + 'static,
{
    Args::inject(container).await
}

/// Given a type, this function will try to call the `inject` method
/// implemented by the type.
/// This function uses the global container
pub async fn provide<T: Injectable + Send + Sync + 'static>() -> T {
    service_container().provide().await
}

/// Given a type, this function will try to find an instance of the type
/// wrapped in a `Service<T>` that is currently registered in the service
/// container.
/// The global service container is used as the resolver.
pub async fn service<T: 'static>() -> Service<T> {
    service_container().service().await
}

/// Given a type, this function will try to find an existing instance of the
/// type in the service container. If that fails, an instance of the type is
/// initialized, wrapped in a `Service`, stored in the service container and
/// a copy is returned. Subsequent call requesting instance of that type will
/// return the instance in the service container.
/// The global service container is used as the resolver.
pub async fn singleton<T: Injectable + Sized + Send + Sync + 'static>() -> Singleton<T> {
    service_container().singleton().await
}

/// Returns the global service container instance
pub fn service_container() -> Arc<ServiceContainer> {
    if let Some(container) = SERVICE_CONTAINER.get() {
        container.clone()
    } else {
        ServiceContainerBuilder::new().build()
    }
}

/// Tries to get an instance of the type if one exist in the container.
/// If one does not exist, it tries to do an injection
pub async fn get_type_or_inject<T: Injectable + Clone + Send + Sync + 'static>() -> T {
    service_container().get_type_or_inject().await
}

/// Tries to get an instance of the type if one exist in the container.
/// If one does not exist, it tries to do an injection.
/// The container to used is provided by the caller
pub async fn get_type_or_inject_with<T: Injectable + Clone + Send + Sync + 'static>(
    container: &ServiceContainer,
) -> T {
    container.get_type_or_inject().await
}

/// Tries to get an instance of the type if one exist in the container
/// This function uses the global container
pub fn get_type<T: Clone + 'static>() -> Option<T> {
    service_container().get_type()
}

/// Tries to get an instance of the type's service if one exist in the container
/// This function uses the global container
pub fn get_service<T: 'static>() -> Option<Service<T>> {
    service_container().get_type()
}

/// Removes the registered instance of the type specified and returns it
/// This function uses the global container
pub fn forget_type<T: 'static>() -> Option<Box<T>> {
    service_container().forget_type()
}

/// Removes the registered service instance of the type specified and returns it
/// This function uses the global container
pub fn forget<T: 'static>() -> Option<Box<Service<T>>> {
    service_container().forget()
}

/// Tries to get an instance of the type wrapped in a `Service<T>` from the container.
/// If one does not exist, it tries to do an injection
/// This function uses the global container
pub async fn get_or_inject<T: Injectable + Send + Sync + 'static>() -> Service<T> {
    service_container().get_or_inject().await
}

/// Tries to get an instance of the type wrapped in a `Service<T>` from the container.
/// If one does not exist, it tries to do an injection.
/// The container to used is provided by the caller
/// This function uses the global container
pub async fn get_or_inject_with<T: Injectable + Clone + Send + Sync + 'static>(
    container: &ServiceContainer,
) -> Service<T> {
    container.get_or_inject().await
}

/// Register a service instance
/// The instance is registered with the global service container
/// This function uses the global container
pub fn register_service<T: Send + Sync + 'static>(ext: T) -> Arc<ServiceContainer> {
    let container = service_container();
    container.set(ext);

    container
}

/// Register a type instance
/// The instance is registered with the global service container
/// This function uses the global container
pub fn register_type<T: Clone + Send + Sync + 'static>(ext: T) -> Arc<ServiceContainer> {
    let container = service_container();
    container.set_type(ext);

    container
}

/// Register a type instance
/// Same as `register_type`
/// The instance is registered with the global service container
/// This function uses the global container
pub fn set_type<T: Clone + Send + Sync + 'static>(ext: T) -> Arc<ServiceContainer> {
    let container = service_container();
    container.set_type(ext);

    container
}

/// Returns a new proxy service container
pub fn make_proxy() -> ServiceContainer {
    ServiceContainer::proxy()
}
