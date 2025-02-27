use std::any::type_name;

use crate::{container::ServiceContainer, helpers::service_container};
use async_trait::async_trait;
use futures::join;

#[async_trait]
#[deprecated(note = "implement Resolver instead")]
pub trait Injectable {
    /// The required method that makes a type injectable
    async fn inject(container: &ServiceContainer) -> Self;

    /// Injects and return a concrete instance of the injectable type
    /// The global service container is used
    async fn instance() -> Self
    where
        Self: Sized,
    {
        let container = service_container();
        Self::inject(&container).await
    }

    /// Injects and returns a concrete instance of the injectable type
    /// The provided container will be used
    async fn instance_with(container: &ServiceContainer) -> Self
    where
        Self: Sized,
    {
        Self::inject(container).await
    }
}

// Zero argument
#[async_trait]
impl Injectable for () {
    async fn inject(_: &ServiceContainer) -> Self {}
}

// 1 arguments
#[async_trait]
impl<A> Injectable for (A,)
where
    A: Injectable + 'static,
{
    async fn inject(c: &ServiceContainer) -> Self {
        (A::inject(c).await,)
    }
}

/// This macro is repeating what is been done
/// for a tuple with one element but for tuples with two or more elements
macro_rules! tuple_from_injectable {
    ($($T: ident),*) => {
        #[async_trait]
        impl<$($T: Injectable + Send + Sync + 'static),+> Injectable for ($($T,)+) {
            async fn inject(c: &ServiceContainer) -> Self {
                join!($($T::inject(c)),+)
            }
        }
    };
}

// 2 arguments
tuple_from_injectable! { A, B }
// 3 arguments
tuple_from_injectable! { A, B, C }
// 4 arguments
tuple_from_injectable! { A, B, C, D }
// 5 arguments
tuple_from_injectable! { A, B, C, D, E }
// 6 arguments
tuple_from_injectable! { A, B, C, D, E, F }
// 7 arguments
tuple_from_injectable! { A, B, C, D, E, F, G }
// 8 arguments
tuple_from_injectable! { A, B, C, D, E, F, G, H }
// 9 arguments
tuple_from_injectable! { A, B, C, D, E, F, G, H, I }
// 10 arguments
tuple_from_injectable! { A, B, C, D, E, F, G, H, I, J }
// 11 arguments
tuple_from_injectable! { A, B, C, D, E, F, G, H, I, J, K }
// 12 arguments
tuple_from_injectable! { A, B, C, D, E, F, G, H, I, J, K, L }
// 13 arguments
tuple_from_injectable! { A, B, C, D, E, F, G, H, I, J, K, L,M }
// 14 arguments
tuple_from_injectable! { A, B, C, D, E, F, G, H, I, J, K, L,M, N }
// 15 arguments
tuple_from_injectable! { A, B, C, D, E, F, G, H, I, J, K, L,M, N, O }
// 16 arguments
tuple_from_injectable! { A, B, C, D, E, F, G, H, I, J, K, L,M, N, O, P }
// 17 arguments
tuple_from_injectable! { A, B, C, D, E, F, G, H, I, J, K, L,M, N, O, P, Q }

#[async_trait]
impl Injectable for u8 {
    async fn inject(c: &ServiceContainer) -> Self {
        c.get_type().await.unwrap_or_default()
    }
}

#[async_trait]
impl Injectable for i8 {
    async fn inject(c: &ServiceContainer) -> Self {
        c.get_type().await.unwrap_or_default()
    }
}

#[async_trait]
impl Injectable for u16 {
    async fn inject(c: &ServiceContainer) -> Self {
        c.get_type().await.unwrap_or_default()
    }
}

#[async_trait]
impl Injectable for i16 {
    async fn inject(c: &ServiceContainer) -> Self {
        c.get_type().await.unwrap_or_default()
    }
}

#[async_trait]
impl Injectable for i32 {
    async fn inject(c: &ServiceContainer) -> Self {
        c.get_type().await.unwrap_or_default()
    }
}

#[async_trait]
impl Injectable for u32 {
    async fn inject(c: &ServiceContainer) -> Self {
        c.get_type().await.unwrap_or_default()
    }
}

#[async_trait]
impl Injectable for i64 {
    async fn inject(c: &ServiceContainer) -> Self {
        c.get_type().await.unwrap_or_default()
    }
}

#[async_trait]
impl Injectable for u64 {
    async fn inject(c: &ServiceContainer) -> Self {
        c.get_type().await.unwrap_or_default()
    }
}

#[async_trait]
impl Injectable for f32 {
    async fn inject(c: &ServiceContainer) -> Self {
        c.get_type().await.unwrap_or_default()
    }
}

#[async_trait]
impl Injectable for f64 {
    async fn inject(c: &ServiceContainer) -> Self {
        c.get_type().await.unwrap_or_default()
    }
}

#[async_trait]
impl Injectable for usize {
    async fn inject(c: &ServiceContainer) -> Self {
        c.get_type().await.unwrap_or_default()
    }
}

#[async_trait]
impl Injectable for isize {
    async fn inject(c: &ServiceContainer) -> Self {
        c.get_type().await.unwrap_or_default()
    }
}

#[async_trait]
impl Injectable for i128 {
    async fn inject(c: &ServiceContainer) -> Self {
        c.get_type().await.unwrap_or_default()
    }
}

#[async_trait]
impl Injectable for u128 {
    async fn inject(c: &ServiceContainer) -> Self {
        c.get_type().await.unwrap_or_default()
    }
}

#[async_trait]
impl Injectable for String {
    async fn inject(c: &ServiceContainer) -> Self {
        c.get_type().await.unwrap_or_default()
    }
}

#[async_trait]
impl<T> Injectable for Option<T>
where
    T: Clone + Send + Sync + 'static,
{
    async fn inject(c: &ServiceContainer) -> Self {
        c.get_type().await
    }
}

#[async_trait]
impl<T> Injectable for Result<T, ()>
where
    T: Clone + Send + Sync + 'static,
{
    async fn inject(c: &ServiceContainer) -> Self {
        c.get_type().await.ok_or(())
    }
}

#[async_trait]
impl<T> Injectable for Result<T, String>
where
    T: Clone + Send + Sync + 'static,
{
    async fn inject(c: &ServiceContainer) -> Self {
        c.get_type()
            .await
            .ok_or(format!("could not inject: {:?}", type_name::<T>()))
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[derive(Clone, Debug)]
    struct Foo;

    #[async_trait::async_trait]
    impl Injectable for Foo {
        async fn inject(_: &ServiceContainer) -> Self {
            Self
        }
    }

    #[tokio::test]
    async fn test_getting_option() {
        let container = ServiceContainer::proxy();

        container
            .set_type(Some(container.provide::<Foo>().await))
            .await;

        let foo: Option<Option<Foo>> = container.get_type().await;

        assert_eq!(
            foo.is_some(),
            true,
            "An instance of Option<Option<Foo>> exist"
        );
        assert_eq!(
            foo.is_some(),
            true,
            "foo should have been wrapped in a Option<Foo>"
        );
    }

    #[tokio::test]
    async fn test_getting_result() {
        let container = ServiceContainer::proxy();

        container
            .set_type(Ok::<Foo, ()>(container.provide::<Foo>().await))
            .await;

        let foo: Option<Result<Foo, ()>> = container.get_type().await;

        assert_eq!(
            foo.is_some(),
            true,
            "An instance of Option<Result<Foo, ()>> exist"
        );

        assert_eq!(
            foo.unwrap().is_ok(),
            true,
            "foo should have been wrapped in a Result<Foo, ()>"
        );

        let an_int: Option<Result<i32, ()>> = container.get_type().await;
        assert_eq!(
            an_int.is_some(),
            false,
            "Option<Result<i32, ()>> instance does not exist"
        );
    }
}
