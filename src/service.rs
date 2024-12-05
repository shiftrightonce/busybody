#![allow(dead_code)]

use crate::{container::ServiceContainer, injectable::Injectable};
use async_trait::async_trait;
use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
};

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
                panic!("Could not find service: {:?}", std::any::type_name::<T>())
            }
        }
    }
}

#[derive(Debug)]
pub struct RawType<T: Clone>(T);

impl<T: Clone> RawType<T> {
    pub fn new(the_type: T) -> Self {
        RawType(the_type)
    }
}

impl<T: Clone> RawType<T> {
    pub fn get_ref(&self) -> &T {
        &self.0
    }

    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T: Clone> Deref for RawType<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T: Clone> DerefMut for RawType<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Clone> From<T> for RawType<T> {
    fn from(the_type: T) -> Self {
        Self(the_type)
    }
}

#[async_trait]
impl<T: Default + Clone + 'static> Injectable for RawType<T> {
    async fn inject(container: &ServiceContainer) -> Self {
        match container.get_type::<T>() {
            Some(service) => Self::new(service),
            None => Self::new(T::default()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_creating_service() {
        let service = Service::new(99);
        assert_eq!(*service, 99);
    }

    #[tokio::test]
    async fn test_creating_from_arc() {
        let service: Service<i32> = Arc::new(7).into();
        assert_eq!(*service, 7);
    }

    #[tokio::test]
    async fn test_creating_from_t() {
        let service: Service<i32> = 9000.into();
        assert_eq!(*service, 9000);
    }

    #[tokio::test]
    async fn test_raw_type() {
        let var: RawType<String> = "Hello World".to_string().into();
        assert_eq!(var.as_str(), "Hello World");
    }
}
