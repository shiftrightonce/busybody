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
        (c.get_type::<A>().await.unwrap(),)
    }
}

/// This macro is repeating what is been done
/// for a tuple with one element but for tuples with two or more elements
macro_rules! tuple_from_resolvable {
    ($($T: ident),*) => {
        #[async_trait::async_trait]
        impl<$($T: Clone + Send + Sync  + 'static),+> Resolver for ($($T,)+) {
            async fn resolve(c: &ServiceContainer) -> Self {
                ($(c.get_type::<$T>().await.unwrap()),+)
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
