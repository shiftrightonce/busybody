use crate::service_container::ServiceContainer;

pub trait Injectable: Sized {
    fn inject(container: &ServiceContainer) -> Self;
}

// Zero argument
impl Injectable for () {
    fn inject(_c: &ServiceContainer) -> Self {}
}

// 2 arguments
impl<A: Injectable + 'static, B: Injectable + 'static> Injectable for (A, B) {
    fn inject(c: &ServiceContainer) -> Self {
        (A::inject(c), B::inject(c))
    }
}
