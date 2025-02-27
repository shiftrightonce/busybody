use std::any::type_name;

use crate::ServiceContainer;

#[async_trait::async_trait]
pub trait Resolver {
    async fn resolve(container: &ServiceContainer) -> Self;
}

// Zero argument
#[async_trait::async_trait]
impl Resolver for () {
    async fn resolve(_: &ServiceContainer) -> Self {}
}

// 1 argument

#[async_trait::async_trait]
impl<A> Resolver for (A,)
where
    A: Clone + Send + Sync + 'static,
{
    async fn resolve(c: &ServiceContainer) -> Self {
        (c.get_type::<A>()
            .await
            .expect(&format!("could not resolve: {}", type_name::<A>())),)
    }
}

/// This macro is repeating what is been done
/// for a tuple with one element but for tuples with two or more elements
macro_rules! tuple_from_resolvable {
    ($($T: ident),*) => {
        #[async_trait::async_trait]
        impl<$($T: Clone + Send + Sync  + 'static),+> Resolver for ($($T,)+) {
            async fn resolve(c: &ServiceContainer) -> Self {
                ($(c.get_type::<$T>().await.expect(&format!("could not resolve: {}", type_name::<A>()))),+)
            }
        }
    };
}

// 2 arguments
tuple_from_resolvable! { A, B }
// 3 arguments
tuple_from_resolvable! { A, B, C }
// 4 arguments
tuple_from_resolvable! { A, B, C, D }
// 5 arguments
tuple_from_resolvable! { A, B, C, D, E }
// 6 arguments
tuple_from_resolvable! { A, B, C, D, E, F }
// 7 arguments
tuple_from_resolvable! { A, B, C, D, E, F, G }
// 8 arguments
tuple_from_resolvable! { A, B, C, D, E, F, G, H }
// 9 arguments
tuple_from_resolvable! { A, B, C, D, E, F, G, H, I }
// 10 arguments
tuple_from_resolvable! { A, B, C, D, E, F, G, H, I, J }
// 11 arguments
tuple_from_resolvable! { A, B, C, D, E, F, G, H, I, J, K }
// 12 arguments
tuple_from_resolvable! { A, B, C, D, E, F, G, H, I, J, K, L }
// 13 arguments
tuple_from_resolvable! { A, B, C, D, E, F, G, H, I, J, K, L,M }
// 14 arguments
tuple_from_resolvable! { A, B, C, D, E, F, G, H, I, J, K, L,M, N }
// 15 arguments
tuple_from_resolvable! { A, B, C, D, E, F, G, H, I, J, K, L,M, N, O }
// 16 arguments
tuple_from_resolvable! { A, B, C, D, E, F, G, H, I, J, K, L,M, N, O, P }
// 17 arguments
tuple_from_resolvable! { A, B, C, D, E, F, G, H, I, J, K, L,M, N, O, P, Q }

#[async_trait::async_trait]
impl Resolver for u8 {
    async fn resolve(c: &ServiceContainer) -> Self {
        c.instance().await.unwrap_or_default()
    }
}

#[async_trait::async_trait]
impl Resolver for i8 {
    async fn resolve(c: &ServiceContainer) -> Self {
        c.instance().await.unwrap_or_default()
    }
}

#[async_trait::async_trait]
impl Resolver for u16 {
    async fn resolve(c: &ServiceContainer) -> Self {
        c.instance().await.unwrap_or_default()
    }
}

#[async_trait::async_trait]
impl Resolver for i16 {
    async fn resolve(c: &ServiceContainer) -> Self {
        c.instance().await.unwrap_or_default()
    }
}

#[async_trait::async_trait]
impl Resolver for i32 {
    async fn resolve(c: &ServiceContainer) -> Self {
        c.instance().await.unwrap_or_default()
    }
}

#[async_trait::async_trait]
impl Resolver for u32 {
    async fn resolve(c: &ServiceContainer) -> Self {
        c.instance().await.unwrap_or_default()
    }
}

#[async_trait::async_trait]
impl Resolver for i64 {
    async fn resolve(c: &ServiceContainer) -> Self {
        c.instance().await.unwrap_or_default()
    }
}

#[async_trait::async_trait]
impl Resolver for u64 {
    async fn resolve(c: &ServiceContainer) -> Self {
        c.instance().await.unwrap_or_default()
    }
}

#[async_trait::async_trait]
impl Resolver for f32 {
    async fn resolve(c: &ServiceContainer) -> Self {
        c.instance().await.unwrap_or_default()
    }
}

#[async_trait::async_trait]
impl Resolver for f64 {
    async fn resolve(c: &ServiceContainer) -> Self {
        c.instance().await.unwrap_or_default()
    }
}

#[async_trait::async_trait]
impl Resolver for usize {
    async fn resolve(c: &ServiceContainer) -> Self {
        c.instance().await.unwrap_or_default()
    }
}

#[async_trait::async_trait]
impl Resolver for isize {
    async fn resolve(c: &ServiceContainer) -> Self {
        c.instance().await.unwrap_or_default()
    }
}

#[async_trait::async_trait]
impl Resolver for i128 {
    async fn resolve(c: &ServiceContainer) -> Self {
        c.instance().await.unwrap_or_default()
    }
}

#[async_trait::async_trait]
impl Resolver for u128 {
    async fn resolve(c: &ServiceContainer) -> Self {
        c.instance().await.unwrap_or_default()
    }
}

#[async_trait::async_trait]
impl Resolver for String {
    async fn resolve(c: &ServiceContainer) -> Self {
        c.instance().await.unwrap_or_default()
    }
}

#[async_trait::async_trait]
impl<T> Resolver for Option<T>
where
    T: Clone + Send + Sync + 'static,
{
    async fn resolve(c: &ServiceContainer) -> Self {
        c.instance().await
    }
}

#[async_trait::async_trait]
impl<T> Resolver for Result<T, ()>
where
    T: Clone + Send + Sync + 'static,
{
    async fn resolve(c: &ServiceContainer) -> Self {
        c.instance().await.ok_or(())
    }
}

#[async_trait::async_trait]
impl<T> Resolver for Result<T, String>
where
    T: Clone + Send + Sync + 'static,
{
    async fn resolve(c: &ServiceContainer) -> Self {
        c.instance()
            .await
            .ok_or(format!("could not inject: {:?}", type_name::<T>()))
    }
}
