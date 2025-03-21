#![allow(dead_code)]

use std::{any::TypeId, sync::Arc};

use futures::future::BoxFuture;

use crate::{
    container::SERVICE_CONTAINER, handlers::Handler, injectable::Injectable, service::Service,
    Resolver, ServiceContainer, ServiceContainerBuilder, Singleton,
};

/// Takes an async function or closure and executes it
/// Require arguments are injected during the call
/// The global service container is used for any resolving
#[deprecated(note = "use `resolve_and_call`")]
pub async fn inject_and_call<F, Args>(mut handler: F) -> F::Output
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
#[deprecated(note = "use `resolve_and_call_with`")]
pub async fn inject_and_call_with<F, Args>(ci: &ServiceContainer, mut handler: F) -> F::Output
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
#[deprecated(note = "use `resolve_all`")]
pub async fn inject_all<Args>() -> Args
where
    Args: Injectable + 'static,
{
    Args::inject(&service_container()).await
}

/// Given a tuple of types, this function will try to resolve them
/// and return a tuple of instances.
/// The service container to used is provided by the caller.
#[deprecated(note = "resolve_all_with")]
pub async fn inject_all_with<Args>(container: &ServiceContainer) -> Args
where
    Args: Injectable + 'static,
{
    Args::inject(container).await
}

/// Takes an async function or closure and execute it
/// Require arguments are resolve either by a resolver or sourced from the service container
///
/// This function will use an existing if one exist.
/// The service container to used is provided by the caller.
pub async fn resolve_and_call<F, Args>(handler: F) -> F::Output
where
    F: Handler<Args>,
    Args: Resolver,
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
    Args: Resolver,
{
    ci.resolve_and_call(handler).await
}

/// Given a type, this function will try to call the `inject` method
/// implemented by the type.
/// This function uses the global container
#[deprecated(note = "use `get_type`")]
pub async fn provide<T: Injectable + Send + Sync + 'static>() -> T {
    service_container().provide().await
}

/// Given a type, this function will try to find an instance of the type
/// wrapped in a `Service<T>` that is currently registered in the service
/// container.
/// The global service container is used as the resolver.
pub async fn service<T: Send + Sync + 'static>() -> Service<T> {
    service_container().service().await
}

/// Given a type, this function will try to find an existing instance of the
/// type in the service container. If that fails, an instance of the type is
/// initialized, wrapped in a `Service`, stored in the service container and
/// a copy is returned. Subsequent call requesting instance of that type will
/// return the instance in the service container.
/// The global service container is used as the resolver.
#[deprecated(note = "register a `resolve_once`` handler and then use `get` or `get_type`")]
pub async fn singleton<T: Injectable + Sized + Send + Sync + 'static>() -> Singleton<T> {
    service_container().singleton().await
}

/// Returns the global service container instance
pub fn service_container() -> Arc<ServiceContainer> {
    if let Some(container) = SERVICE_CONTAINER.get() {
        return container.clone();
    }
    ServiceContainerBuilder::new().build()
}

/// Returns an instance of the service builder
pub fn make_builder() -> ServiceContainerBuilder {
    ServiceContainerBuilder::new()
}

/// Tries to get an instance of the type if one exist in the container.
/// If one does not exist, it tries to do an injection
#[deprecated(note = "use `get_type`")]
pub async fn get_type_or_inject<T: Injectable + Clone + Send + Sync + 'static>() -> T {
    service_container().get_type_or_inject().await
}

/// Tries to get an instance of the type if one exist in the container.
/// If one does not exist, it tries to do an injection.
/// The container to used is provided by the caller
#[deprecated(note = "use a proxy container and call `get_type`")]
pub async fn get_type_or_inject_with<T: Injectable + Clone + Send + Sync + 'static>(
    container: &ServiceContainer,
) -> T {
    container.get_type_or_inject().await
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

/// Tries to get an instance of the type wrapped in a `Service<T>` from the container.
/// If one does not exist, it tries to do an injection
/// This function uses the global container
#[deprecated(note = "use `get`")]
pub async fn get_or_inject<T: Injectable + Send + Sync + 'static>() -> Service<T> {
    service_container().get_or_inject().await
}

/// Tries to get an instance of the type wrapped in a `Service<T>` from the container.
/// If one does not exist, it tries to do an injection.
/// The container to used is provided by the caller
/// This function uses the global container
#[deprecated(note = "call `get` on the container you are about to pass")]
pub async fn get_or_inject_with<T: Injectable + Clone + Send + Sync + 'static>(
    container: &ServiceContainer,
) -> Service<T> {
    container.get_or_inject().await
}

/// Register a service instance
/// The instance is registered with the global service container
/// This function uses the global container
pub async fn register_service<T: Send + Sync + 'static>(ext: T) -> Arc<ServiceContainer> {
    let container = service_container();
    container.set(ext).await;

    container
}

/// Register a type instance
/// The instance is registered with the global service container
/// This function uses the global container
pub async fn register_type<T: Clone + Send + Sync + 'static>(ext: T) -> Arc<ServiceContainer> {
    let container = service_container();
    container.set_type(ext).await;

    container
}

/// Register a type instance
/// Same as `register_type`
/// The instance is registered with the global service container
/// This function uses the global container
pub async fn set_type<T: Clone + Send + Sync + 'static>(ext: T) -> Arc<ServiceContainer> {
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
) -> Arc<ServiceContainer> {
    let c = service_container();
    c.resolver(callback).await;
    c
}

/// Registers a type as resolvable
/// Existing resolver for this type will be replaced
/// This function uses the global container
///
pub async fn resolvable<T: Resolver + Clone + Send + Sync + 'static>() -> Arc<ServiceContainer> {
    let c = service_container();
    c.resolvable::<T>().await;
    c
}

pub async fn resolvable_once<T: Resolver + Clone + Send + Sync + 'static>() -> Arc<ServiceContainer>
{
    let c = service_container();
    c.resolvable_once::<T>().await;
    c
}

pub async fn soft_resolvable<T: Resolver + Clone + Send + Sync + 'static>() -> Arc<ServiceContainer>
{
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
) -> Arc<ServiceContainer> {
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
) -> Arc<ServiceContainer> {
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
) -> Arc<ServiceContainer> {
    let c = service_container();
    c.soft_resolver_once(callback).await;
    c
}

/// Returns a new proxy service container
pub fn make_proxy() -> ServiceContainer {
    ServiceContainer::proxy()
}
