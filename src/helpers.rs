#![allow(dead_code)]

use crate::{
    container::SERVICE_CONTAINER, handlers::Handler, injectable::Injectable,
    provider::ServiceProvider, service::Service, service_container, Singleton,
};

pub async fn inject_service<F, Args>(handler: F)
where
    F: Handler<Args>,
    Args: Injectable + 'static,
{
    let args = Args::inject(SERVICE_CONTAINER.get().unwrap()).await;
    handler.call(args);
}

pub async fn inject_all<Args>() -> Args
where
    Args: Injectable + 'static,
{
    Args::inject(SERVICE_CONTAINER.get().unwrap()).await
}

pub async fn provide<T: Injectable + Send + Sync + 'static>() -> T {
    ServiceProvider::provide().await
}

pub async fn service<T: 'static>() -> Service<T> {
    ServiceProvider::service().await
}

pub async fn singleton<T: Injectable + Sized + Send + Sync + 'static>() -> Singleton<T> {
    Singleton::inject(&service_container()).await
}
