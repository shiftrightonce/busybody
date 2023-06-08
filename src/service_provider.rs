use crate::{injectables::Injectable, service::Service, service_container::SERVICE_CONTAINER};

pub struct ServiceProvider;

impl ServiceProvider {
    pub fn provide<T: Injectable + 'static>() -> T {
        T::inject(SERVICE_CONTAINER.get().unwrap())
    }

    pub fn service<T: 'static>() -> Service<T> {
        Service::inject(SERVICE_CONTAINER.get().unwrap())
    }
}

pub fn provide<T: Injectable + 'static>() -> T {
    ServiceProvider::provide()
}

pub fn service<T: 'static>() -> Service<T> {
    ServiceProvider::service()
}
