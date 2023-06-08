#![allow(dead_code)]

use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::{Arc, OnceLock},
};

use crate::service::Service;

pub(crate) static SERVICE_CONTAINER: OnceLock<Arc<ServiceContainer>> = OnceLock::new();

pub struct ServiceContainer {
    items: HashMap<TypeId, Box<dyn Any + Send + Sync + 'static>>,
}

impl ServiceContainer {
    pub fn get<T: 'static>(&self) -> Option<&T> {
        self.items
            .get(&TypeId::of::<T>())
            .and_then(|b| b.downcast_ref())
    }

    pub fn service<T: 'static>(&self) -> Option<Service<T>> {
        self.items
            .get(&TypeId::of::<Service<T>>())
            .and_then(|b| b.downcast_ref().cloned())
    }
}

pub struct ServiceContainerBuilder {
    items: HashMap<TypeId, Box<dyn Any + Send + Sync + 'static>>,
}

impl ServiceContainerBuilder {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    pub fn register<T: Send + Sync + 'static>(mut self, ext: T) -> Self {
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
        let container = Arc::new(ServiceContainer { items: self.items });
        _ = SERVICE_CONTAINER.set(container.clone());

        container.clone()
    }
}
