#![allow(dead_code)]

use crate::{container::ServiceContainer, injectable::Injectable, service::Service};
use async_trait::async_trait;
use std::ops::Deref;

pub struct Singleton<T: Injectable>(Service<T>);

#[async_trait(?Send)]
impl<T: Injectable + Send + Sync + 'static> Injectable for Singleton<T> {
    async fn inject(container: &ServiceContainer) -> Self {
        let content = match container.get::<T>() {
            Some(user) => user,
            None => {
                let content_service = Service::new(T::inject(container).await);
                container.set_type(content_service.clone());
                content_service
            }
        };
        Self(content)
    }
}

impl<T: Injectable + ?Sized> Singleton<T> {
    pub fn get_ref(&self) -> &T {
        self.0.as_ref()
    }

    pub fn into_inner(self) -> Service<T> {
        self.0
    }
}

impl<T: Injectable + ?Sized> Deref for Singleton<T> {
    type Target = Service<T>;

    fn deref(&self) -> &Service<T> {
        &self.0
    }
}

impl<T: Injectable + ?Sized> Clone for Singleton<T> {
    fn clone(&self) -> Self {
        Self(Service::clone(&self.0))
    }
}

impl<T: Injectable + ?Sized> From<T> for Singleton<T> {
    fn from(service: T) -> Self {
        Self(Service::new(service))
    }
}
