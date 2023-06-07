use crate::service_container::ServiceContainer;

pub trait Injectable: Sized {
    fn inject(container: &ServiceContainer) -> Self;
}

// Zero argument
impl Injectable for () {
    fn inject(_c: &ServiceContainer) -> Self {}
}

// 1 arguments
impl<A: Injectable + 'static> Injectable for (A,) {
    fn inject(c: &ServiceContainer) -> Self {
        (A::inject(c),)
    }
}

// 2 arguments
impl<A: Injectable + 'static, B: Injectable + 'static> Injectable for (A, B) {
    fn inject(c: &ServiceContainer) -> Self {
        (A::inject(c), B::inject(c))
    }
}

// 3 arguments
impl<A: Injectable + 'static, B: Injectable + 'static, C: Injectable + 'static> Injectable
    for (A, B, C)
{
    fn inject(c: &ServiceContainer) -> Self {
        (A::inject(c), B::inject(c), C::inject(c))
    }
}

// 4 arguments
impl<
        A: Injectable + 'static,
        B: Injectable + 'static,
        C: Injectable + 'static,
        D: Injectable + 'static,
    > Injectable for (A, B, C, D)
{
    fn inject(c: &ServiceContainer) -> Self {
        (A::inject(c), B::inject(c), C::inject(c), D::inject(c))
    }
}
