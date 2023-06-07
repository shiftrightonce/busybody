use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

pub struct ServiceContainer {
    items: HashMap<TypeId, Box<dyn Any + Send + Sync + 'static>>,
}

impl ServiceContainer {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    pub fn register<T: Send + Sync + 'static>(mut self, ext: T) -> Self {
        dbg!("type: {:?}", TypeId::of::<T>());
        self.items.insert(TypeId::of::<T>(), Box::new(ext));
        self
    }

    pub fn get<T: 'static>(&self) -> Option<&T> {
        dbg!("getting ID", TypeId::of::<T>());
        self.items
            .get(&TypeId::of::<T>())
            .and_then(|b| b.downcast_ref())
    }
}
