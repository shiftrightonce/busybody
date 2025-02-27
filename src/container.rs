#![allow(dead_code)]

use futures::future::BoxFuture;
use tokio::sync::{Mutex, RwLock};

use crate::{
    helpers::service_container, service::Service, Handler, Injectable, Resolver, Singleton,
};
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::{Arc, OnceLock},
};

pub(crate) static SERVICE_CONTAINER: OnceLock<Arc<ServiceContainer>> = OnceLock::new();
pub(crate) const GLOBAL_INSTANCE_ID: &str = "_global_ci";

type ResolverCollection = HashMap<
    TypeId,
    Arc<
        Mutex<
            Box<
                dyn FnMut(
                        ServiceContainer,
                    )
                        -> BoxFuture<'static, Box<dyn Any + Send + Sync + 'static>>
                    + Sync
                    + Send
                    + 'static,
            >,
        >,
    >,
>;

#[derive(Default, Clone)]
pub(crate) struct Container {
    services: Arc<RwLock<HashMap<TypeId, Box<dyn Any + Send + Sync + 'static>>>>,
    resolvers: Arc<RwLock<ResolverCollection>>,
}

impl Container {
    pub(crate) async fn get<T: Clone + 'static>(&self, ci: ServiceContainer) -> Option<T> {
        let lock = self.services.read().await;
        if let Some(raw) = lock.get(&TypeId::of::<T>()) {
            return raw.downcast_ref().cloned();
        }
        drop(lock);

        let lock = self.resolvers.read().await;

        if let Some(mutex) = lock.get(&TypeId::of::<T>()).cloned() {
            drop(lock);
            let mut callback = mutex.lock().await;
            return callback(ci).await.downcast_ref::<T>().cloned();
        }

        None
    }

    pub(crate) async fn set<T: Send + Sync + 'static>(&self, value: T) -> &Self {
        let mut lock = self.services.write().await;
        lock.insert(
            TypeId::of::<T>(),
            Box::new(value) as Box<dyn Any + Send + Sync + 'static>,
        );
        drop(lock);

        self
    }

    pub(crate) async fn forget<T: 'static>(&self) -> Option<Box<T>> {
        let mut lock = self.services.write().await;
        if let Some(raw) = lock.remove(&TypeId::of::<T>()) {
            return raw.downcast().ok();
        }
        None
    }

    pub(crate) async fn resolver<T: Clone + Send + Sync + 'static>(
        &self,
        mut callback: impl FnMut(ServiceContainer) -> BoxFuture<'static, T>
            + Send
            + Sync
            + Clone
            + 'static,
    ) -> &Self {
        let mut lock = self.resolvers.write().await;
        lock.insert(
            TypeId::of::<T>(),
            Arc::new(Mutex::new(Box::new(move |c| {
                let f = (callback)(c);
                Box::pin(async move { Box::new(f.await) as Box<dyn Any + Send + Sync + 'static> })
            }))),
        );
        self
    }

    pub(crate) async fn soft_resolver<T: Clone + Send + Sync + 'static>(
        &self,
        callback: impl Fn(ServiceContainer) -> BoxFuture<'static, T> + Send + Sync + Clone + 'static,
    ) -> &Self {
        if self.has_resolver::<T>().await {
            return self;
        }

        self.resolver(callback).await
    }

    pub(crate) async fn has_resolver<T: 'static>(&self) -> bool {
        let lock = self.resolvers.read().await;
        lock.get(&TypeId::of::<T>()).is_some()
    }
}

#[derive(Clone)]
pub struct ServiceContainer {
    in_proxy_mode: bool,
    is_reference: bool,
    container: Container,
    id: Arc<String>,
}

impl Default for ServiceContainer {
    fn default() -> Self {
        Self::new()
    }
}

impl ServiceContainer {
    pub(crate) fn new() -> Self {
        let id = GLOBAL_INSTANCE_ID.to_string();
        Self {
            id: Arc::new(id),
            in_proxy_mode: false,
            is_reference: false,
            container: Default::default(),
        }
    }

    pub(crate) fn make_reference(&self) -> Self {
        Self {
            is_reference: true,
            id: self.id.clone(),
            in_proxy_mode: self.in_proxy_mode,
            container: self.container.clone(),
        }
    }

    /// Create an instance of the container in proxy mode
    /// A proxy container is a container that creates a
    /// limited scope but will reach out to the global service
    /// container when an instance of a type does not exist locally.
    ///
    /// This allows a new instance of a type to be created and use in
    /// a specific scope
    pub fn proxy() -> Self {
        let id = ulid::Ulid::new().to_string().to_lowercase();

        let mut ci = Self::default();
        ci.id = Arc::new(id);
        ci.in_proxy_mode = true;
        ci
    }

    /// Returns the proxy state of the current container
    pub fn is_proxy(&self) -> bool {
        self.in_proxy_mode
    }

    /// Checks if the current container is in proxy mode.
    /// If that is the case, it tries to find the instance of the
    /// type, falls back to the main service container
    pub async fn proxy_value<T: Clone + 'static>(&self) -> Option<T> {
        if self.is_proxy() {
            self.get_type::<T>().await
        } else {
            None
        }
    }

    /// Tries to find the instance of the type wrapped in `Service<T>`
    pub async fn get<T: 'static>(&self) -> Option<Service<T>> {
        self.get_type::<Service<T>>().await
    }

    pub async fn forget_type<T: 'static>(&self) -> Option<Box<T>> {
        self.container.forget::<T>(self.make_reference()).await
    }

    pub async fn forget_resolver<T: 'static>(&self) -> bool {
        self.container.remove_resolver::<T>().await
    }

    pub async fn forget<T: 'static>(&self) -> Option<Box<Service<T>>> {
        self.forget_type().await
    }

    /// Tries to find the instance of the type wrapped in `Service<T>`
    /// if an instance does not exist, one will be injected
    pub async fn get_or_inject<T: Injectable + Send + Sync + 'static>(&self) -> Service<T> {
        let result = self.get::<T>().await;

        if result.is_none() {
            let instance = T::inject(self).await;
            return self.set(instance).await.get::<T>().await.unwrap();
        }

        result.unwrap()
    }

    /// Tries to find the instance of the type T
    /// if an instance does not exist, one will be injected
    pub async fn get_type_or_inject<T: Injectable + Clone + Send + Sync + 'static>(&self) -> T {
        let result = self.get_type::<T>().await;
        if result.is_none() {
            let instance = T::inject(self).await;
            self.set_type(instance.clone()).await;
            return instance;
        }

        result.unwrap()
    }

    /// Tries to find the "raw" instance of the type
    pub async fn get_type<T: Clone + 'static>(&self) -> Option<T> {
        let value = self.container.get::<T>(self.make_reference()).await;
        if value.is_some() {
            return value;
        }

        if self.is_proxy() {
            return Box::pin(async { service_container().get_type().await }).await;
        }

        None
    }

    /// Stores the instance
    pub async fn set_type<T: Clone + Send + Sync + 'static>(&self, value: T) -> &Self {
        self.resolver(move |_| {
            let c = value.clone();
            Box::pin(async move { c })
        })
        .await;
        self
    }

    pub(crate) async fn remember<T: Clone + Send + Sync + 'static>(&self, value: T) -> &Self {
        self.container.set(value).await;
        self
    }

    /// Stores the instance as `Service<T>`
    /// You need to use "get" in order to retrieve the instance
    pub async fn set<T: Send + Sync + 'static>(&self, ext: T) -> &Self {
        self.set_type(Service::new(ext)).await
    }

    /// Registers a closure that will be call each time
    /// an instance of the specified type is requested
    /// This closure will override existing closure for this type
    ///
    ///       
    pub async fn resolver<T: Clone + Send + Sync + 'static>(
        &self,
        callback: impl FnMut(ServiceContainer) -> BoxFuture<'static, T> + Send + Sync + Clone + 'static,
    ) -> &Self {
        self.container.resolver(callback).await;

        self
    }

    /// Registers a closure that will be call each time
    /// an instance of the specified type is requested
    /// This closure will be ignored if the type already has a registered resolver
    ///
    pub async fn soft_resolver<T: Clone + Send + Sync + 'static>(
        &self,
        callback: impl Fn(ServiceContainer) -> BoxFuture<'static, T> + Send + Sync + Clone + 'static,
    ) -> &Self {
        self.container.soft_resolver(callback).await;
        self
    }

    /// Registers a closure that will be call the first time
    /// an instance of the specified type is requested
    /// This closure will override existing closure for this type
    ///
    pub async fn resolver_once<T: Clone + Send + Sync + 'static>(
        &self,
        callback: impl Fn(ServiceContainer) -> BoxFuture<'static, T> + Send + Sync + Copy + 'static,
    ) -> &Self {
        self.container
            .resolver(move |container| {
                let f = (callback)(container.clone());
                Box::pin(async move {
                    let value = f.await;
                    container.set_type(value.clone()).await;
                    value
                })
            })
            .await;

        self
    }

    /// Registers a closure that will be call each time
    /// an instance of the specified type is requested
    /// This closure will be ignored if the type already has a registered resolver
    ///
    /// The returned instance will be store in the service container
    /// and subsequent request for this type will resolve to that copy.
    ///
    pub async fn soft_resolver_once<T: Clone + Send + Sync + 'static>(
        &self,
        callback: impl Fn(ServiceContainer) -> BoxFuture<'static, T> + Send + Sync + Copy + 'static,
    ) -> &Self {
        if !self.container.has_resolver::<T>().await {
            self.resolver_once(callback).await;
        }

        self
    }

    /// Takes an async function or closure and executes it
    /// Require arguments are injected during the call. All arguments must implement
    /// Injectable.
    ///
    /// This method does not check for existing instance
    pub async fn inject_and_call<F, Args>(&self, handler: F) -> F::Output
    where
        F: Handler<Args>,
        Args: Injectable + 'static,
    {
        let args = Args::inject(self).await;
        handler.call(args).await
    }

    /// Takes an async function or closure and execute it
    /// Require arguments are resolve either by a resolver or sourced from the service container
    ///
    /// This method will use an existing if one exist.
    pub async fn resolve_and_call<F, Args>(&self, handler: F) -> F::Output
    where
        F: Handler<Args>,
        Args: Resolver,
    {
        let args = Args::resolve(self).await;
        handler.call(args).await
    }

    /// Given a tuple of types, this method will try to resolve them
    /// by using a resolver or cloning an existing instance in the container
    ///
    pub async fn resolve_all<Args>(&self) -> Args
    where
        Args: Resolver,
    {
        Args::resolve(self).await
    }

    /// Given a tuple of types, this method will try to resolve them
    /// and return a tuple of instances
    /// The types must implement Injectable.
    ///
    /// This method does not check for existing instance of the types.
    pub async fn inject_all<Args>(&self) -> Args
    where
        Args: Injectable + 'static,
    {
        Args::inject(self).await
    }

    /// Given a type, this method will try to call the `inject` method
    /// implemented on the type. It does not check the container for existing
    /// instance.
    pub async fn provide<T: Injectable + 'static>(&self) -> T {
        T::inject(self).await
    }

    /// Given a type, this method will try to find an instance of the type
    /// wrapped in a `Service<T>` that is currently registered in the service
    /// container.
    pub async fn service<T: Send + Sync + 'static>(&self) -> Service<T> {
        Service::inject(self).await
    }

    /// Given a type, this method will try to find an existing instance of the
    /// type. If that fails, an instance of the type is
    /// initialized, wrapped in a `Service`, stored and
    /// a copy is returned. Subsequent calls requesting instance of that type will
    /// returned the stored copy. If the this is a proxy container, the instance will be dropped when
    /// this container goes out of scope.
    pub async fn singleton<T: Injectable + Sized + Send + Sync + 'static>(&self) -> Singleton<T> {
        Singleton::inject(self).await
    }
}

pub struct ServiceContainerBuilder {
    service_container: ServiceContainer,
}

impl Default for ServiceContainerBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ServiceContainerBuilder {
    pub fn new() -> Self {
        Self {
            service_container: ServiceContainer::new(),
        }
    }

    pub fn new_proxy() -> Self {
        Self {
            service_container: ServiceContainer::proxy(),
        }
    }

    pub async fn register<T: Clone + Send + Sync + 'static>(self, ext: T) -> Self {
        self.service_container.set_type(ext).await;
        self
    }

    /// Registers a closure that will be call each time
    /// an instance of the specified type is requested
    /// This closure will override existing closure for this type
    ///
    pub async fn resolver<T: Clone + Send + Sync + 'static>(
        self,
        callback: impl FnMut(ServiceContainer) -> BoxFuture<'static, T> + Send + Sync + Copy + 'static,
    ) -> Self {
        self.service_container.resolver(callback).await;
        self
    }

    /// Registers a closure that will be call each time
    /// an instance of the specified type is requested
    /// This closure will override existing closure for this type
    ///
    /// The returned instance will be store in the service container
    /// and subsequent request for this type will resolve to that copy.
    ///
    pub async fn resolver_once<T: Clone + Send + Sync + 'static>(
        self,
        callback: impl Fn(ServiceContainer) -> BoxFuture<'static, T> + Send + Sync + Copy + 'static,
    ) -> Self {
        self.service_container.resolver_once(callback).await;
        self
    }

    /// Registers a closure that will be call each time
    /// an instance of the specified type is requested
    /// If a closure already registered for this type, this one will be ignore
    ///
    ///
    pub async fn soft_resolver<T: Clone + Send + Sync + 'static>(
        self,
        callback: impl Fn(ServiceContainer) -> BoxFuture<'static, T> + Send + Sync + Clone + 'static,
    ) -> Self {
        self.service_container.soft_resolver(callback).await;
        self
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
        self,
        callback: impl Fn(ServiceContainer) -> BoxFuture<'static, T> + Send + Sync + Copy + 'static,
    ) -> Self {
        self.service_container.soft_resolver_once(callback).await;
        self
    }

    /// T is wrapped in a `Service`
    /// This means to get T back you need to specify `Service<T>`
    ///  or use the "get" method on the container
    pub async fn service<T: Send + Sync + 'static>(self, ext: T) -> Self {
        self.service_container.set(ext).await;
        self
    }

    /// Instantiate and returns the service container
    pub fn build(self) -> Arc<ServiceContainer> {
        if self.service_container.id.as_str() == GLOBAL_INSTANCE_ID {
            SERVICE_CONTAINER
                .get_or_init(|| Arc::new(self.service_container))
                .clone()
        } else {
            Arc::new(self.service_container)
        }
    }
}

#[cfg(test)]
mod test {
    use async_trait::async_trait;

    use crate::helpers::service_container;

    use super::*;

    #[derive(Debug, Clone)]
    struct Counter {
        start_point: usize,
    }

    #[async_trait]
    impl Injectable for Counter {
        async fn inject(container: &ServiceContainer) -> Self {
            let mut result = container.get_type().await;
            if result.is_none() {
                result = container
                    .set_type(Counter { start_point: 44 })
                    .await
                    .get_type()
                    .await;
            }
            result.unwrap()
        }
    }

    #[derive(Debug, Clone)]
    struct User {
        id: i32,
    }

    #[async_trait]
    impl Injectable for User {
        async fn inject(_: &ServiceContainer) -> Self {
            Self { id: 1000 }
        }
    }

    #[tokio::test]
    async fn test_builder() {
        let container = ServiceContainerBuilder::new_proxy()
            .service(5usize)
            .await
            .register(true)
            .await
            .build();

        assert_eq!(*container.get::<usize>().await.unwrap(), 5usize);
        assert_eq!(container.get_type::<bool>().await, Some(true));
    }

    #[tokio::test]
    async fn test_empty_container() {
        let container = ServiceContainer::proxy();

        assert_eq!(container.get::<i32>().await.is_none(), true);
        assert_eq!(container.get_type::<i32>().await, None);
    }

    #[tokio::test]
    async fn test_getting_raw_type() {
        let container = ServiceContainer::proxy();
        container.set_type(400).await;
        container.set_type(300f32).await;
        container.set_type(true).await;

        assert_eq!(container.get_type::<i32>().await, Some(400));
        assert_eq!(container.get_type::<f32>().await, Some(300f32));
        assert_eq!(container.get_type::<bool>().await, Some(true));
    }

    #[tokio::test]
    async fn test_getting_service_type() {
        let container = ServiceContainer::proxy();
        container.set(400).await;
        container.set(300f32).await;
        container.set(true).await;

        assert_eq!(*container.get::<i32>().await.unwrap(), 400);
        assert_eq!(*container.get::<f32>().await.unwrap(), 300f32);
        assert_eq!(*container.get::<bool>().await.unwrap(), true);
    }

    #[tokio::test]
    async fn test_proxy_service() {
        service_container().set_type(true).await;
        let container = ServiceContainer::proxy();

        let is_true: Option<bool> = container.get_type().await;
        let an_i32: Option<i32> = container.get_type().await;

        assert_eq!(is_true, Some(true));
        assert_eq!(an_i32, None);

        container.set_type(30000).await;
        let rate_per_hour: Option<i32> = container.get_type().await;
        assert_eq!(rate_per_hour, Some(30000));
    }

    #[tokio::test]
    async fn test_injecting() {
        let container = ServiceContainer::proxy();
        let counter = container.inject_all::<Counter>().await;

        assert_eq!(counter.start_point, 44usize);
    }

    #[tokio::test]
    async fn test_injecting_stored_instance() {
        let container = ServiceContainer::proxy();
        container.set_type(Counter { start_point: 6000 }).await;

        let counter = container.inject_all::<Counter>().await;
        assert_eq!(counter.start_point, 6000usize);
    }

    #[tokio::test]
    async fn test_singleton() {
        let container = ServiceContainer::proxy();

        let user = container.singleton::<User>().await;
        assert_eq!(user.id, 1000);

        container.set_type(User { id: 88 }).await;
        let user = container.singleton::<User>().await;
        assert_eq!(user.id, 1000);
    }

    #[tokio::test]
    async fn test_inject_and_call() {
        let container = ServiceContainer::proxy();

        let result = container
            .inject_and_call(|user: User, counter: Counter| async move {
                assert_eq!(user.id, 1000);
                assert_eq!(counter.start_point, 44);
                (1, 2, 3)
            })
            .await;

        assert_eq!(result, (1, 2, 3));
    }

    #[tokio::test]
    async fn test_get_or_inject_raw_type() {
        let container = ServiceContainer::proxy();
        assert_eq!(container.get_type::<User>().await.is_none(), true);

        let a_user = container.get_type_or_inject::<User>().await;
        let a_user2 = container.get_type::<User>().await;

        assert_eq!(a_user.id, 1000);
        assert_eq!(a_user2.is_some(), true);
        assert_eq!(a_user2.unwrap().id, a_user.id);
    }

    #[tokio::test]
    async fn test_get_or_inject_service_type() {
        let container = ServiceContainer::proxy();

        assert_eq!(container.get::<User>().await.is_none(), true);

        let a_user = container.get_or_inject::<User>().await;
        let a_user2 = container.get::<User>().await;

        assert_eq!(a_user.id, 1000);
        assert_eq!(a_user2.is_some(), true);
        assert_eq!(a_user2.unwrap().id, a_user.id);
    }

    #[tokio::test]
    async fn test_forgetting_a_type() {
        let container = ServiceContainer::proxy();

        assert_eq!(container.get_type::<usize>().await, None);

        container.set_type(300_usize).await;
        assert_eq!(container.get_type::<usize>().await, Some(300_usize));

        let value = container.forget_type::<usize>().await;
        assert_eq!(value.is_some(), true);

        assert_eq!(container.get_type::<usize>().await, None);
    }

    #[tokio::test]
    async fn test_forgetting_service_a_type() {
        let container = ServiceContainer::proxy();

        assert_eq!(container.get::<usize>().await.is_none(), true);

        container.set(300_usize).await;
        assert_eq!(*container.get::<usize>().await.unwrap(), 300_usize);

        let value = container.forget::<usize>().await;
        assert_eq!(value.is_some(), true);

        assert_eq!(container.get::<usize>().await.is_none(), true);
    }

    #[tokio::test]
    async fn test_service_without_clone_type() {
        struct UserName(String);

        let container = ServiceContainer::proxy();
        container.set(UserName("foobar".to_string())).await;

        let result: Option<Service<_>> = container.get::<UserName>().await;

        assert_eq!(true, result.is_some());
        assert_eq!("foobar", result.unwrap().as_ref().0);
    }

    #[tokio::test]
    async fn test_resolver() {
        let container = ServiceContainer::proxy();

        container
            .resolver::<String>(|_| Box::pin(async { "foo".to_string() }))
            .await;

        assert_eq!(
            container.get_type::<String>().await,
            Some("foo".to_string()),
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_resolving_once() {
        let container = ServiceContainer::proxy();

        #[derive(Debug, Clone, PartialEq)]
        struct Special(String);

        container
            .resolver_once::<Special>(|c| {
                Box::pin(async move {
                    let counter: i32 = c.get_type().await.unwrap_or_default();
                    c.set_type(counter + 1).await;
                    Special(format!("id:{counter}"))
                })
            })
            .await;

        assert_eq!(
            container.get_type::<Special>().await,
            Some(Special("id:0".to_string()))
        );
        assert_eq!(
            container.get_type::<Special>().await,
            Some(Special("id:0".to_string())),
            "ID should have been zero (0)"
        );
    }

    #[tokio::test]
    async fn test_soft_resolving() {
        let container = ServiceContainer::proxy();

        container
            .resolver(|_| Box::pin(async { SoftCounter(1) }))
            .await;
        container
            .soft_resolver(|_| Box::pin(async { SoftCounter(100) }))
            .await;

        #[derive(Debug, Clone, PartialEq)]
        struct SoftCounter(i32);

        let counter: SoftCounter = container.get_type().await.unwrap();
        assert_eq!(counter.0, 1);

        let counter: SoftCounter = container.get_type().await.unwrap();
        assert_ne!(counter.0, 100);
    }

    #[tokio::test]
    async fn test_soft_resolving2() {
        let container = ServiceContainer::proxy();

        container
            .soft_resolver(|_| Box::pin(async { SoftCounter(100) }))
            .await;

        #[derive(Debug, Clone, PartialEq)]
        struct SoftCounter(i32);

        let counter: SoftCounter = container.get_type().await.unwrap();
        assert_eq!(counter.0, 100);
    }
}
