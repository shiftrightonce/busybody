use crate::service_container::ServiceContainer;
mod handlers;
mod service;
mod service_container;
use service::Service;
use std::sync::OnceLock;

static SERVICE_CONTAINER: OnceLock<ServiceContainer> = OnceLock::new();

#[derive(Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
}

fn main() {
    let user = User {
        id: 545656,
        name: "John doe".into(),
    };

    let mut services = ServiceContainer::new();
    services = services
        .register(300i32)
        .register(user)
        .register(Service::new(User {
            id: 33,
            name: "Jibao".to_owned(),
        }));

    _ = SERVICE_CONTAINER.set(services);

    let number = SERVICE_CONTAINER.get().unwrap().get::<i32>().unwrap();
    dbg!(number);

    let the_user = SERVICE_CONTAINER.get().unwrap().get::<User>().unwrap();
    dbg!(the_user);

    println!("Hello, world!");

    inject_service(|a: i32| {
        dbg!(a * 2);
    });

    inject_service(|user: Service<User>| {
        dbg!(user.get_ref());
    });

    inject_service(|| {
        dbg!("this is really cool");
    });
}

pub trait Injectable: Sized {
    fn inject(container: &ServiceContainer) -> Self;
}

impl Injectable for () {
    fn inject(_c: &ServiceContainer) -> Self {
        ()
    }
}

impl Injectable for (i32,) {
    fn inject(_c: &ServiceContainer) -> Self {
        (44,)
    }
}

impl<T: 'static> Injectable for (Service<T>,) {
    fn inject(container: &ServiceContainer) -> Self {
        (container.get::<Service<T>>().as_mut().unwrap().clone(),)
    }
}

impl<A: Injectable + 'static, B: Injectable + 'static> Injectable for (A, B) {
    fn inject(c: &ServiceContainer) -> Self {
        (A::inject(c), B::inject(c))
    }
}

pub trait Handler<Args> {
    // type Output;
    fn call(&self, args: Args);
}

impl<Func> Handler<()> for Func
where
    Func: Fn(),
{
    fn call(&self, _arg1: ()) {
        (self)();
    }
}

impl<Func, Arg1> Handler<(Arg1,)> for Func
where
    Func: Fn(Arg1),
{
    fn call(&self, (arg1,): (Arg1,)) {
        (self)(arg1);
    }
}

impl<Func, Arg1, Arg2> Handler<(Arg1, Arg2)> for Func
where
    Func: Fn(Arg1, Arg2),
{
    fn call(&self, (arg1, arg2): (Arg1, Arg2)) {
        (self)(arg1, arg2);
    }
}

pub fn inject_service<F, Args>(handler: F)
where
    F: Handler<Args>,
    Args: Injectable + 'static,
{
    let args = Args::inject(SERVICE_CONTAINER.get().unwrap());
    handler.call(args)
}
