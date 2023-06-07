#![allow(dead_code)]

use std::{ops::Deref, sync::Arc};

use crate::{injectables::Injectable, service_container::ServiceContainer};

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
    type Target = Arc<T>;

    fn deref(&self) -> &Arc<T> {
        &self.0
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

impl<T: 'static> Injectable for Service<T> {
    fn inject(container: &ServiceContainer) -> Self {
        container.get::<Service<T>>().as_mut().unwrap().clone()
    }
}
