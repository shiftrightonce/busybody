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

    pub fn proxy() -> Self {
        Self {
            services: RwLock::new(HashMap::new()),
            in_proxy_mode: true,
        }
    }

    pub fn is_proxy(&self) -> bool {
        self.in_proxy_mode
    }

    pub fn get<T: 'static>(&self) -> Option<Service<T>> {
        self.get_type::<Service<T>>()
    }

    pub fn get_type<T: Clone + 'static>(&self) -> Option<T> {
        if let Ok(services) = self.services.read() {
            let result: Option<&T> = services
                .get(&TypeId::of::<T>())
                .and_then(|b| b.downcast_ref());

            if let Some(service) = result {
                return Some(service.clone());
            }
        } else if self.in_proxy_mode {
            return service_container().get_type();
        }
        None
    }

    pub fn set_type<T: Clone + Send + Sync + 'static>(&self, ext: T) -> &Self {
        if let Ok(mut list) = self.services.write() {
            list.insert(TypeId::of::<T>(), Box::new(ext));
        }
        self
    }
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
    pub fn service<T: Send + Sync + 'static>(mut self, ext: T) -> Self {
        self.items
            .insert(TypeId::of::<Service<T>>(), Box::new(Service::new(ext)));
        self
    }

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
