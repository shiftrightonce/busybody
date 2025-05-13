#![allow(dead_code)]

use futures::future::BoxFuture;

use crate::{
    Resolver, ServiceContainer, ServiceContainerBuilder, container::SERVICE_CONTAINER,
    handlers::Handler, service::Service,
};

/// Takes an async function or closure and execute it
/// Require arguments are resolve either by a resolver or sourced from the service container
///
/// This function will use an existing if one exist.
/// The service container to used is provided by the caller.
pub async fn resolve_and_call<F, Args>(handler: F) -> F::Output
where
    F: Handler<Args>,
    Args: Clone + Resolver + 'static,
{
    service_container().resolve_and_call(handler).await
}

/// Given a tuple of types, this function will try to resolve them
/// by using a resolver or cloning an existing instance in the container
///
/// The global service container is used.
pub async fn resolve_all<Args>() -> Args
where
    Args: Resolver,
{
    Args::resolve(&service_container()).await
}

/// Given a tuple of types, this function will try to resolve them
/// by using a resolver or cloning an existing instance in the container
///
pub async fn resolve_all_with<Args>(ci: &ServiceContainer) -> Args
where
    Args: Resolver,
{
    Args::resolve(ci).await
}

/// Takes an async function or closure, a reference to the service container and execute it
/// Require arguments are resolve either by a resolver or sourced from the service container
///
/// This function will use an existing if one exist.
/// This  function will use the provided service container before fallback back to the global one
pub async fn resolve_and_call_with<F, Args>(ci: &ServiceContainer, handler: F) -> F::Output
where
    F: Handler<Args>,
    Args: Clone + Resolver + 'static,
{
    ci.resolve_and_call(handler).await
}

/// Given a type, this function will try to find an instance of the type
/// wrapped in a `Service<T>` that is currently registered in the service
/// container.
/// The global service container is used as the resolver.
pub async fn service<T: Send + Sync + 'static>() -> Service<T> {
    service_container().get_type().await.unwrap()
}

/// Returns the global service container instance
pub fn service_container() -> ServiceContainer {
    if let Some(container) = SERVICE_CONTAINER.get() {
        return container.clone();
    }
    ServiceContainerBuilder::new().build()
}

/// Returns an instance of the service builder
pub fn make_builder() -> ServiceContainerBuilder {
    ServiceContainerBuilder::new()
}

/// Tries to get an instance of the type if one exist in the container
/// This function uses the global container
pub async fn get_type<T: Clone + 'static>() -> Option<T> {
    service_container().get_type().await
}

/// Tries to get an instance of the type's service if one exist in the container
/// This function uses the global container
pub async fn get_service<T: 'static>() -> Option<Service<T>> {
    service_container().get::<T>().await
}

/// Removes the registered instance of the type specified and returns it
/// This function uses the global container
pub async fn forget_type<T: 'static>() -> Option<Box<T>> {
    service_container().forget_type().await
}

/// Removes the registered service instance of the type specified and returns it
/// This function uses the global container
pub async fn forget<T: 'static>() -> Option<Box<Service<T>>> {
    service_container().forget().await
}

/// Register a service instance
/// The instance is registered with the global service container
/// This function uses the global container
pub async fn register_service<T: Send + Sync + 'static>(ext: T) -> ServiceContainer {
    let container = service_container();
    container.set(ext).await;

    container
}

/// Register a type instance
/// The instance is registered with the global service container
/// This function uses the global container
pub async fn register_type<T: Clone + Send + Sync + 'static>(ext: T) -> ServiceContainer {
    let container = service_container();
    container.set_type(ext).await;

    container
}

/// Register a type instance
/// Same as `register_type`
/// The instance is registered with the global service container
/// This function uses the global container
pub async fn set_type<T: Clone + Send + Sync + 'static>(ext: T) -> ServiceContainer {
    let container = service_container();
    container.set_type(ext).await;

    container
}

/// Registers a closure that will be call each time
/// an instance of the specified type is requested
/// This closure will override existing closure for this type
/// This function uses the global container
///
pub async fn resolver<T: Clone + Send + Sync + 'static>(
    callback: impl Fn(ServiceContainer) -> BoxFuture<'static, T> + Send + Sync + Copy + 'static,
) -> ServiceContainer {
    let c = service_container();
    c.resolver(callback).await;
    c
}

/// Registers a type as resolvable
/// Existing resolver for this type will be replaced
/// This function uses the global container
///
pub async fn resolvable<T: Resolver + Clone + Send + Sync + 'static>() -> ServiceContainer {
    let c = service_container();
    c.resolvable::<T>().await;
    c
}

pub async fn resolvable_once<T: Resolver + Clone + Send + Sync + 'static>() -> ServiceContainer {
    let c = service_container();
    c.resolvable_once::<T>().await;
    c
}

pub async fn soft_resolvable<T: Resolver + Clone + Send + Sync + 'static>() -> ServiceContainer {
    let c = service_container();
    c.soft_resolvable::<T>().await;
    c
}

/// Registers a closure that will be call each time
/// an instance of the specified type is requested
/// This closure will override existing closure for this type
///
/// The returned instance will be store in the global service container
/// and subsequent request for this type will resolve to that copy.
///
/// Note: The service container passed to your callback is the instance
///       of the global service container
pub async fn resolver_once<T: Clone + Send + Sync + 'static>(
    callback: impl Fn(ServiceContainer) -> BoxFuture<'static, T> + Send + Sync + Copy + 'static,
) -> ServiceContainer {
    let c = service_container();
    c.resolver_once(callback).await;
    c
}

/// Registers a closure that will be call each time
/// an instance of the specified type is requested
/// If a closure already registered for this type, this one will be ignore
///
///
/// Note: The service container passed to your callback is the instance
///       of the global service container
pub async fn soft_resolver<T: Clone + Send + Sync + 'static>(
    callback: impl Fn(ServiceContainer) -> BoxFuture<'static, T> + Send + Sync + Copy + 'static,
) -> ServiceContainer {
    let c = service_container();
    c.soft_resolver(callback).await;
    c
}

/// Registers a closure that will be call each time
/// an instance of the specified type is requested
/// If a closure already registered for this type, this one will be ignore
///
/// The returned instance will be store in the global service container
/// and subsequent request for this type will resolve to that copy.
///
/// Note: The service container passed to your callback is the instance
///       of the global service container
pub async fn soft_resolver_once<T: Clone + Send + Sync + 'static>(
    callback: impl Fn(ServiceContainer) -> BoxFuture<'static, T> + Send + Sync + Copy + 'static,
) -> ServiceContainer {
    let c = service_container();
    c.soft_resolver_once(callback).await;
    c
}

/// Returns a new proxy service container
pub fn make_proxy() -> ServiceContainer {
    ServiceContainer::proxy()
}

/// Returns a new proxy service container
/// The container is tie/link to the current process
pub fn make_task_proxy() -> ServiceContainer {
    ServiceContainer::make_task_proxy()
}
