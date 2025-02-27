#![allow(dead_code)]

use crate::{container::ServiceContainer, injectable::Injectable, Resolver};
use async_trait::async_trait;
use std::sync::Arc;

pub type Service<T> = Arc<T>;

#[async_trait]
impl<T: Send + Sync + 'static> Injectable for Service<T> {
    async fn inject(container: &ServiceContainer) -> Self {
        match container.get::<T>().await {
            Some(service) => service,
            None => {
                panic!("Could not find service: {:?}", std::any::type_name::<T>())
            }
        }
    }
}

#[async_trait]
impl<T: Send + Sync + 'static> Resolver for Service<T>
where
    T: Resolver,
{
    async fn resolve(container: &ServiceContainer) -> Self {
        Service::new(T::resolve(container).await)
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
}
