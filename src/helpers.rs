#![allow(dead_code)]

use crate::{
    handlers::Handler, injectables::Injectable, service::Service,
    service_container::SERVICE_CONTAINER, service_provider::ServiceProvider,
};

pub fn inject_service<F, Args>(handler: F)
where
    F: Handler<Args>,
    Args: Injectable + 'static,
{
    let args = Args::inject(SERVICE_CONTAINER.get().unwrap());
    handler.call(args)
}

pub fn provide<T: Injectable + 'static>() -> T {
    ServiceProvider::provide()
}

pub fn service<T: 'static>() -> Service<T> {
    ServiceProvider::service()
}
