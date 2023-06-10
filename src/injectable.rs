use crate::container::ServiceContainer;
use async_trait::async_trait;
use futures::join;

#[async_trait(?Send)]
pub trait Injectable: Sized {
    async fn inject(container: &ServiceContainer) -> Self;
}

// Zero argument
#[async_trait(?Send)]
impl Injectable for () {
    async fn inject(_: &ServiceContainer) -> Self {}
}

// 1 arguments
#[async_trait(?Send)]
impl<A: Injectable> Injectable for (A,) {
    async fn inject(c: &ServiceContainer) -> Self {
        (A::inject(c).await,)
    }
}

/// This macro is repeating what is been done
/// for a tuple with one element but for tuples with two or more elements
macro_rules! tuple_from_injectable {
    ($($T: ident),*) => {
        #[async_trait(?Send)]
        impl<$($T: Injectable + 'static),+> Injectable for ($($T,)+) {
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
