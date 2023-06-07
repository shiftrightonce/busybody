use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::OnceLock,
};

pub(crate) static SERVICE_CONTAINER: OnceLock<ServiceContainer> = OnceLock::new();

pub struct ServiceContainer {
    items: HashMap<TypeId, Box<dyn Any + Send + Sync + 'static>>,
}

impl ServiceContainer {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    fn new_with(items: HashMap<TypeId, Box<dyn Any + Send + Sync + 'static>>) -> Self {
        Self { items }
    }

    pub fn register<T: Send + Sync + 'static>(mut self, ext: T) -> Self {
        self.items.insert(TypeId::of::<T>(), Box::new(ext));
        self
    }

    pub fn get<T: 'static>(&self) -> Option<&T> {
        self.items
            .get(&TypeId::of::<T>())
            .and_then(|b| b.downcast_ref())
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

    pub fn build(self) {
        _ = SERVICE_CONTAINER.set(ServiceContainer::new_with(self.items));
    }
}
