//! `Singleton<T>` is use to fetch an existing instance in the service container or
//! instantiate a new instance, wrap it in a `Service`, stores a copy and returns a copy
//!
#![allow(dead_code)]
use crate::{container::ServiceContainer, injectable::Injectable, service::Service};
use async_trait::async_trait;
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct Singleton<T: Injectable>(Service<T>);

#[async_trait]
impl<T: Injectable + Send + Sync + 'static> Injectable for Singleton<T> {
    async fn inject(container: &ServiceContainer) -> Self {
        let content = match container.get::<T>() {
            Some(instance) => instance,
            None => {
                let content_service = Service::new(T::inject(container).await);
                container.set_type(content_service.clone());
                content_service
            }
        };
        Self(content)
    }
}

impl<T: Injectable + Sized> Singleton<T> {
    pub fn get_ref(&self) -> &T {
        &self.0
    }

    pub fn into_inner(self) -> Service<T> {
        self.0
    }
}

impl<T: Injectable + Sized> Deref for Singleton<T> {
    type Target = Service<T>;

    fn deref(&self) -> &Service<T> {
        &self.0
    }
}

impl<T: Injectable + Sized> DerefMut for Singleton<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Injectable + Sized> Clone for Singleton<T> {
    fn clone(&self) -> Self {
        Self(Service::clone(&self.0))
    }
}
