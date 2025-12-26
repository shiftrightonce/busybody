#![allow(dead_code)]

use futures::future::BoxFuture;
use tokio::sync::{Mutex, RwLock};

use crate::{Handler, Resolver, helpers::service_container, service::Service};
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    fmt::{Debug, Display},
    sync::{Arc, OnceLock, atomic::AtomicUsize},
};

pub(crate) static SERVICE_CONTAINER: OnceLock<ServiceContainer> = OnceLock::new();
pub(crate) const GLOBAL_INSTANCE_ID: u64 = 0;
pub(crate) static TASK_SERVICE_CONTAINER: OnceLock<
    std::sync::Mutex<HashMap<u64, (AtomicUsize, Container)>>,
> = OnceLock::new();

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

    pub(crate) async fn forget<T: 'static>(&self, ci: ServiceContainer) -> Option<Box<T>> {
        let mut lock = self.services.write().await;
        if let Some(raw) = lock.remove(&TypeId::of::<T>()) {
            self.resolvers.write().await.remove(&TypeId::of::<T>());
            return raw.downcast().ok();
        }

        let mut lock = self.resolvers.write().await;
        if let Some(mutex) = lock.remove(&TypeId::of::<T>()) {
            drop(lock);
            let mut callback = mutex.lock().await;
            return callback(ci).await.downcast::<T>().ok();
        }

        None
    }

    pub(crate) async fn remove_resolver<T: 'static>(&self) -> bool {
        if self.has_resolver::<T>().await {
            let mut lock = self.resolvers.write().await;
            lock.remove(&TypeId::of::<T>());
            true
        } else {
            false
        }
    }

    pub(crate) async fn resolver<T: Send + Sync + 'static, F>(
        &self,
        mut callback: impl FnMut(ServiceContainer) -> F + Send + Sync + 'static,
    ) -> &Self
    where
        F: Future<Output = T> + Send + 'static,
    {
        let mut lock = self.resolvers.write().await;
        lock.insert(
            TypeId::of::<T>(),
            Arc::new(Mutex::new(Box::new(move |c| {
                let f = (callback)(c);
                Box::pin(async move {
                    //
                    Box::new(f.await) as Box<dyn Any + Send + Sync + 'static>
                })
            }))),
        );
        self
    }

    pub(crate) async fn soft_resolver<T: Clone + Send + Sync + 'static, F>(
        &self,
        callback: impl FnMut(ServiceContainer) -> F + Send + Sync + 'static,
    ) -> &Self
    where
        F: Future<Output = T> + Send + 'static,
    {
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
    is_task_mode: bool,
    is_reference: bool,
    container: Container,
    id: u64,
}

impl Debug for ServiceContainer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self, f)
    }
}

impl Display for ServiceContainer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "service container:: id: {}, proxy_mode: {}, task mode: {}",
            self.id,
            self.is_proxy(),
            self.is_task_proxy()
        )
    }
}

impl Default for ServiceContainer {
    fn default() -> Self {
        Self::new()
    }
}

impl ServiceContainer {
    pub fn id(&self) -> u64 {
        self.id
    }
    pub(crate) fn new() -> Self {
        _ = TASK_SERVICE_CONTAINER.get_or_init(std::sync::Mutex::default);

        Self {
            id: GLOBAL_INSTANCE_ID,
            in_proxy_mode: false,
            is_task_mode: false,
            is_reference: false,
            container: Default::default(),
        }
    }

    pub(crate) fn make_reference(&self) -> Self {
        Self {
            is_reference: true,
            id: self.id.clone(),
            in_proxy_mode: self.in_proxy_mode,
            is_task_mode: self.is_task_mode,
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
        let mut ci = Self::default();
        ci.id = ulid::Ulid::new().0 as u64;
        ci.in_proxy_mode = true;

        ci
    }

    /// Returns a new proxy service container that is tie to the current task
    ///
    /// If this method is called outside of a task context, an error will be return
    pub fn make_task_proxy() -> Result<Self, String> {
        if let Some(ci) = Self::get_task_instance() {
            return Ok(ci);
        }

        let id = if let Some(id) = tokio::task::try_id() {
            id.to_string()
                .parse::<u64>()
                .unwrap_or_else(|_| ulid::Ulid::new().0 as u64)
        } else {
            return Err("Task proxy requires a async task process".to_string());
        };

        let mut ci = Self::default();
        ci.id = id;
        ci.in_proxy_mode = true;
        ci.is_task_mode = true;

        if let Some(mutex) = TASK_SERVICE_CONTAINER.get()
            && let Ok(mut lock) = mutex.lock()
        {
            let counter = AtomicUsize::new(1);
            lock.insert(id, (counter, ci.container.clone()));
            drop(lock);
        }

        Ok(ci)
    }

    /// Returns the task proxy state of the current container
    pub fn is_task_proxy(&self) -> bool {
        self.is_task_mode
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

    /// Tries to find the "raw" instance of the type
    pub async fn get_type<T: Clone + 'static>(&self) -> Option<T> {
        let value = self.container.get::<T>(self.make_reference()).await;
        if value.is_some() {
            return value;
        }

        if !self.is_task_proxy()
            && self.is_proxy()
            && let Some(sc) = Self::get_task_instance()
        {
            let result = Box::pin(sc.get_type()).await;
            if result.is_some() {
                return result;
            }
        }

        if self.is_proxy() {
            let value = Box::pin(
                service_container()
                    .container
                    .get::<T>(self.make_reference()),
            )
            .await;

            if value.is_some() {
                return value;
            }

            return Box::pin(service_container().get_type()).await;
        }

        None
    }

    pub(crate) async fn instance<T: Clone + 'static>(&self) -> Option<T> {
        self.container.get::<T>(self.make_reference()).await
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

    pub(crate) fn get_task_instance() -> Option<ServiceContainer> {
        let id = if let Some(id) = tokio::task::try_id() {
            id.to_string()
                .parse::<u64>()
                .unwrap_or_else(|_| ulid::Ulid::new().0 as u64)
        } else {
            return None;
        };

        let mutex = TASK_SERVICE_CONTAINER.get_or_init(std::sync::Mutex::default);
        if let Ok(mut lock) = mutex.lock()
            && let Some((counter, c)) = lock.get_mut(&id)
        {
            counter.fetch_add(1, std::sync::atomic::Ordering::Acquire);
            let mut instance = Self::proxy();
            instance.id = id;
            instance.container = c.clone();
            instance.is_task_mode = true;
            return Some(instance);
        }

        None
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
    pub async fn resolver<T: Send + Sync + 'static, F>(
        &self,
        callback: impl FnMut(ServiceContainer) -> F + Send + Sync + 'static,
    ) -> &Self
    where
        F: Future<Output = T> + Send + 'static,
    {
        self.container.resolver(callback).await;
        self
    }

    pub async fn resolvable<T: Resolver + Clone + Send + Sync + 'static>(&self) -> &Self {
        self.container
            .resolver(|c| async move { T::resolve(&c).await })
            .await;
        self
    }

    pub async fn resolvable_once<T: Resolver + Clone + Send + Sync + 'static>(&self) -> &Self {
        self.resolver_once(|c| Box::pin(async move { T::resolve(&c).await }))
            .await;
        self
    }

    pub async fn soft_resolvable<T: Resolver + Clone + Send + Sync + 'static>(&self) -> &Self {
        self.soft_resolver(|c| Box::pin(async move { T::resolve(&c).await }))
            .await;
        self
    }

    /// Registers a closure that will be call each time
    /// an instance of the specified type is requested
    /// This closure will be ignored if the type already has a registered resolver
    ///
    pub async fn soft_resolver<T: Clone + Send + Sync + 'static, F>(
        &self,
        callback: impl Fn(ServiceContainer) -> F + Send + Sync + 'static,
    ) -> &Self
    where
        F: Future<Output = T> + Send + 'static,
    {
        self.container.soft_resolver(callback).await;
        self
    }

    /// Registers a closure that will be call the first time
    /// an instance of the specified type is requested
    /// This closure will override existing closure for this type
    ///
    pub async fn resolver_once<T: Clone + Send + Sync + 'static, F>(
        &self,
        // callback: impl Fn(ServiceContainer) -> BoxFuture<'static, T> + Send + Sync + Copy + 'static,
        callback: impl Fn(ServiceContainer) -> F + Send + Sync + 'static,
    ) -> &Self
    where
        F: Future<Output = T> + Send + 'static,
    {
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
    pub async fn soft_resolver_once<T: Clone + Send + Sync + 'static, F>(
        &self,
        callback: impl Fn(ServiceContainer) -> F + Send + Sync + 'static,
    ) -> &Self
    where
        F: Future<Output = T> + Send + 'static,
    {
        if !self.container.has_resolver::<T>().await {
            self.resolver_once(callback).await;
        }

        self
    }

    /// Takes an async function or closure and execute it
    /// Require arguments are resolve either by a resolver or sourced from the service container
    ///
    /// This method will use an existing if one exist.
    pub async fn resolve_and_call<F, Args>(&self, mut handler: F) -> F::Output
    where
        F: Handler<Args>,
        Args: Clone + Resolver + 'static,
    {
        let args = self.resolve_all().await;
        handler.call(args).await
    }

    /// Given a tuple of types, this method will try to resolve them
    /// by using a resolver or cloning an existing instance in the container
    ///
    pub async fn resolve_all<Args>(&self) -> Args
    where
        Args: Clone + Resolver + 'static,
    {
        if let Some(a) = self.get_type::<Args>().await {
            return a;
        }

        Args::resolve(self).await
    }
}

impl Drop for ServiceContainer {
    fn drop(&mut self) {
        if self.is_task_proxy()
            && let Some(mutex) = TASK_SERVICE_CONTAINER.get()
            && let Ok(mut lock) = mutex.lock()
        {
            if let Some((counter, sc)) = lock.remove(&self.id)
                && counter.fetch_sub(1, std::sync::atomic::Ordering::Acquire) > 0
            {
                lock.insert(self.id, (counter, sc));
            }
            drop(lock);
        }
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
            service_container: SERVICE_CONTAINER.get_or_init(ServiceContainer::new).clone(),
        }
    }

    /// Create a new proxy container
    ///
    /// All resolving will be executed on this container before fallback to the
    /// global container
    pub fn new_proxy() -> Self {
        Self {
            service_container: ServiceContainer::proxy(),
        }
    }

    /// Registers an instance of a type
    pub async fn register<T: Clone + Send + Sync + 'static>(self, ext: T) -> Self {
        self.service_container.set_type(ext).await;
        // Makes it easy for this type to be resolvable
        self.resolver(|ci| async move { ci.get_type::<T>().await })
            .await
    }

    /// Registers a closure that will be call each time
    ///
    /// an instance of the specified type is requested
    /// This closure will override existing closure for this type
    pub async fn resolver<T: Clone + Send + Sync + 'static, F>(
        self,
        callback: impl FnMut(ServiceContainer) -> F + Send + Sync + 'static,
    ) -> Self
    where
        F: Future<Output = T> + Send + 'static,
    {
        self.service_container.resolver(callback).await;
        self
    }

    /// Registers type T as resolvable
    ///
    /// This call will override existing resolver for this type
    pub async fn resolvable<T: Resolver + Clone + Send + Sync + 'static>(self) -> Self {
        self.service_container.resolvable::<T>().await;
        self
    }

    /// Registers type T as resolvable
    ///
    /// This call will override existing resolver for this type
    /// The returned instance will be cache and use fro subsequent resolving
    pub async fn resolvable_once<T: Resolver + Clone + Send + Sync + 'static>(self) -> Self {
        self.service_container.resolvable_once::<T>().await;
        self
    }

    /// Registers type T as resolvable
    ///
    /// If a resolver already exist, this call will gracefully fail
    pub async fn soft_resolvable<T: Resolver + Clone + Send + Sync + 'static>(self) -> Self {
        self.service_container.soft_resolvable::<T>().await;
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
    pub async fn soft_resolver<T: Clone + Send + Sync + 'static, F>(
        self,
        callback: impl Fn(ServiceContainer) -> F + Send + Sync + 'static,
    ) -> Self
    where
        F: Future<Output = T> + Send + 'static,
    {
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
    pub async fn soft_resolver_once<T: Clone + Send + Sync + 'static, F>(
        self,
        callback: impl Fn(ServiceContainer) -> F + Send + Sync + 'static,
    ) -> Self
    where
        F: Future<Output = T> + Send + 'static,
    {
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
    pub fn build(self) -> ServiceContainer {
        self.service_container
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
    impl Resolver for Counter {
        async fn resolve(container: &ServiceContainer) -> Self {
            container.set_type(Counter { start_point: 44 }).await;
            container.get_type().await.unwrap()
        }
    }

    #[derive(Debug, Clone)]
    struct User {
        id: i32,
    }

    #[async_trait]
    impl Resolver for User {
        async fn resolve(_: &ServiceContainer) -> Self {
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
        let counter = container.resolve_all::<Counter>().await;

        assert_eq!(counter.start_point, 44usize);
    }

    #[tokio::test]
    async fn test_injecting_stored_instance() {
        let container = ServiceContainer::proxy();
        container.set_type(Counter { start_point: 6000 }).await;

        let counter = container.resolve_all::<Counter>().await;
        assert_eq!(counter.start_point, 6000usize);
    }

    #[tokio::test]
    async fn test_inject_and_call() {
        let container = ServiceContainer::proxy();
        container.resolvable::<User>().await;
        container.resolvable::<Counter>().await;

        let result = container
            .resolve_and_call(|user: User, counter: Counter| async move {
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

        container
            .resolver(|c| async move {
                //
                User::resolve(&c).await
            })
            .await;

        let a_user = container
            .get_type::<User>()
            .await
            .expect("instance of User was expected");
        let a_user2 = container.get_type::<User>().await;

        assert_eq!(a_user.id, 1000);
        assert_eq!(a_user2.is_some(), true);
        assert_eq!(a_user2.unwrap().id, a_user.id);
    }

    #[tokio::test]
    async fn test_get_or_inject_service_type() {
        let container = ServiceContainer::proxy();

        assert_eq!(container.get::<User>().await.is_none(), true);

        container.resolvable_once::<User>().await;

        let a_user = container.resolve_all::<User>().await;
        let a_user2 = container.get_type::<User>().await;

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

        container.resolver(|_| async { "foo".to_string() }).await;

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
            .resolver_once(|c| async move {
                let counter: i32 = c.get_type().await.unwrap_or_default();
                c.set_type(counter + 1).await;
                Special(format!("id:{counter}"))
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

        container.resolver(|_| async { SoftCounter(1) }).await;
        container
            .soft_resolver(|_| async { SoftCounter(100) })
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

    #[tokio::test]
    async fn test_forgetting_resolver() {
        let container = ServiceContainer::proxy();
        container.resolver(|_| Box::pin(async { 100 })).await;

        let number = container.get_type::<i32>().await;
        assert_eq!(number.is_some(), true);

        container.forget_resolver::<i32>().await;
        let number2 = container.get_type::<i32>().await;
        assert_eq!(number2.is_none(), true);
    }
}
