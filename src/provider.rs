#![allow(dead_code)]
use crate::{helpers::service_container, injectable::Injectable, service::Service};

pub struct ServiceProvider;

impl ServiceProvider {
    pub async fn provide<T: Injectable + Send + Sync + 'static>() -> T {
        T::inject(service_container().as_ref()).await
    }

    pub async fn service<T: 'static>() -> Service<T> {
        Service::inject(service_container().as_ref()).await
    }
}
