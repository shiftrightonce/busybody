#![allow(dead_code)]

use crate::{helpers::service_container, service::Service, Handler, Injectable, Singleton};
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::{Arc, OnceLock, RwLock},
};

pub(crate) static SERVICE_CONTAINER: OnceLock<Arc<ServiceContainer>> = OnceLock::new();

pub struct ServiceContainer {
    services: RwLock<HashMap<TypeId, Box<dyn Any + Send + Sync + 'static>>>,
    in_proxy_mode: bool,
}

impl Default for ServiceContainer {
    fn default() -> Self {
        Self::new()
    }
}

impl ServiceContainer {
    pub(crate) fn new() -> Self {
        Self {
            services: RwLock::new(HashMap::new()),
            in_proxy_mode: false,
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
        Self {
            services: RwLock::new(HashMap::new()),
            in_proxy_mode: true,
        }
    }

    /// Returns the proxy state of the current container
    pub fn is_proxy(&self) -> bool {
        self.in_proxy_mode
    }

    /// Checks if the current container is in proxy mode.
    /// If that is the case, it tries to find the instance of the
    /// type, falls back to the main service container
    pub fn proxy_value<T: Clone + 'static>(&self) -> Option<T> {
        if self.is_proxy() {
            self.get_type::<T>()
        } else {
            None
        }
    }

    /// Tries to find the instance of the type wrapped in Service<T>
    pub fn get<T: 'static>(&self) -> Option<Service<T>> {
        self.get_type::<Service<T>>()
    }

    /// Tries to find the "raw" instance of the type
    pub fn get_type<T: Clone + 'static>(&self) -> Option<T> {
        if let Ok(services) = self.services.read() {
            let result: Option<&T> = services
                .get(&TypeId::of::<T>())
                .and_then(|b| b.downcast_ref());

            if let Some(service) = result {
                return Some(service.clone());
            } else if self.is_proxy() {
                return service_container().get_type();
            }
        }
        None
    }

    /// Stores the instance
    pub fn set_type<T: Clone + Send + Sync + 'static>(&self, ext: T) -> &Self {
        if let Ok(mut list) = self.services.write() {
            list.insert(TypeId::of::<T>(), Box::new(ext));
        }
        self
    }

    /// Stores the instance as Service<T>
    /// You need to use "get" in order to retrieve the instance
    pub fn set<T: Send + Sync + 'static>(&self, ext: T) -> &Self {
        self.set_type(Service::new(ext))
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
    pub async fn provide<T: Injectable + Send + Sync + 'static>(&self) -> T {
        T::inject(self).await
    }

    /// Given a type, this method will try to find an instance of the type
    /// wrapped in a `Service<T>` that is currently registered in the service
    /// container.
    pub async fn service<T: 'static>(&self) -> Service<T> {
        Service::inject(self).await
    }

    /// Given a type, this method will try to find an existing instance of the
    /// type. If that fails, an instance of the type is
    /// initialized, wrapped in a `Service`, stored and
    /// a copy is returned. Subsequent call requesting instance of that type will
    /// returned. If the this is a proxy container, the instance will be dropped with
    /// this container goes out of scope.
    pub async fn singleton<T: Injectable + Sized + Send + Sync + 'static>(&self) -> Singleton<T> {
        Singleton::inject(self).await
    }
}
pub struct ServiceContainerBuilder {
    items: HashMap<TypeId, Box<dyn Any + Send + Sync + 'static>>,
}

impl Default for ServiceContainerBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ServiceContainerBuilder {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    pub fn register<T: Clone + Send + Sync + 'static>(mut self, ext: T) -> Self {
        self.items.insert(TypeId::of::<T>(), Box::new(ext));
        self
    }

    /// T is wrapped in a `Service`
    /// This means to get T back you need to specify `Service<T>`
    ///  or use the "get" method on the container
    pub fn service<T: Send + Sync + 'static>(mut self, ext: T) -> Self {
        self.items
            .insert(TypeId::of::<Service<T>>(), Box::new(Service::new(ext)));
        self
    }

    /// Instantiate and returns the service container
    pub fn build(self) -> Arc<ServiceContainer> {
        let container = SERVICE_CONTAINER.get_or_init(|| Arc::new(ServiceContainer::default()));
        if let Ok(mut services) = container.services.write() {
            for (k, v) in self.items {
                services.insert(k, v);
            }
        }
        container.clone()
    }
}

#[cfg(test)]
mod test {
    use async_trait::async_trait;

    use super::*;

    #[derive(Debug, Clone)]
    struct Counter {
        start_point: usize,
    }

    #[async_trait]
    impl Injectable for Counter {
        async fn inject(container: &ServiceContainer) -> Self {
            container
                .get_type()
                .unwrap_or_else(|| Counter { start_point: 44 })
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
        let container = ServiceContainerBuilder::new()
            .service(5usize)
            .register(true)
            .build();

        assert_eq!(*container.get::<usize>().unwrap(), 5usize);
        assert_eq!(container.get_type::<bool>(), Some(true));
    }

    #[tokio::test]
    async fn test_empty_container() {
        let container = ServiceContainer::new();

        assert_eq!(container.get::<i32>().is_none(), true);
        assert_eq!(container.get_type::<i32>(), None);
    }

    #[tokio::test]
    async fn test_getting_raw_type() {
        let container = ServiceContainer::new();
        container.set_type(400);
        container.set_type(300f32);
        container.set_type(true);

        assert_eq!(container.get_type::<i32>(), Some(400));
        assert_eq!(container.get_type::<f32>(), Some(300f32));
        assert_eq!(container.get_type::<bool>(), Some(true));
    }

    #[tokio::test]
    async fn test_getting_service_type() {
        let container = ServiceContainer::new();
        container.set(400);
        container.set(300f32);
        container.set(true);

        assert_eq!(*container.get::<i32>().unwrap(), 400);
        assert_eq!(*container.get::<f32>().unwrap(), 300f32);
        assert_eq!(*container.get::<bool>().unwrap(), true);
    }

    #[tokio::test]
    async fn test_proxy_service() {
        service_container().set_type(true);
        let container = ServiceContainer::proxy();

        let is_true: Option<bool> = container.get_type();
        let an_i32: Option<i32> = container.get_type();

        assert_eq!(is_true, Some(true));
        assert_eq!(an_i32, None);

        container.set_type(30000);
        let rate_per_hour: Option<i32> = container.get_type();
        assert_eq!(rate_per_hour, Some(30000));
    }

    #[tokio::test]
    async fn test_injecting() {
        let container = ServiceContainer::new();
        let counter = container.inject_all::<Counter>().await;

        assert_eq!(counter.start_point, 44usize);
    }

    #[tokio::test]
    async fn test_injecting_stored_instance() {
        let container = ServiceContainer::new();
        container.set_type(Counter { start_point: 6000 });

        let counter = container.inject_all::<Counter>().await;
        assert_eq!(counter.start_point, 6000usize);
    }

    #[tokio::test]
    async fn test_singleton() {
        let container = ServiceContainer::new();

        let user = container.singleton::<User>().await;
        assert_eq!(user.id, 1000);

        container.set_type(User { id: 88 });
        let user = container.singleton::<User>().await;
        assert_eq!(user.id, 1000);
    }

    #[tokio::test]
    async fn test_inject_and_call() {
        let container = ServiceContainer::new();

        let result = container
            .inject_and_call(|user: User, counter: Counter| async move {
                assert_eq!(user.id, 1000);
                assert_eq!(counter.start_point, 44);
                (1, 2, 3)
            })
            .await;

        assert_eq!(result, (1, 2, 3));
    }
}
