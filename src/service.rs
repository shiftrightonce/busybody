#![allow(dead_code)]

use crate::{container::ServiceContainer, injectable::Injectable};
use async_trait::async_trait;
use std::{ops::Deref, sync::Arc};

#[derive(Debug)]
pub struct Service<T: ?Sized>(Arc<T>);

impl<T> Service<T> {
    pub fn new(the_service: T) -> Self {
        Service(Arc::new(the_service))
    }
}

impl<T: ?Sized> Service<T> {
    pub fn get_ref(&self) -> &T {
        self.0.as_ref()
    }

    pub fn into_inner(self) -> Arc<T> {
        self.0
    }
}

impl<T: ?Sized> Deref for Service<T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.0.as_ref()
    }
}

impl<T: ?Sized> Clone for Service<T> {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

impl<T: ?Sized> From<Arc<T>> for Service<T> {
    fn from(arc: Arc<T>) -> Self {
        Self(arc)
    }
}

impl<T: Sized> From<T> for Service<T> {
    fn from(arc: T) -> Self {
        Self(Arc::new(arc))
    }
}

#[async_trait]
impl<T: 'static> Injectable for Service<T> {
    async fn inject(container: &ServiceContainer) -> Self {
        match container.get::<T>() {
            Some(service) => service,
            None => {
                panic!("Could not find service")
            }
        }
    }
}
