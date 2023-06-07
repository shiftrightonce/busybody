use crate::{helpers::inject_service, service_container::ServiceContainerBuilder};
mod handlers;
mod helpers;
mod injectables;
mod service;
mod service_container;
use service::Service;

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

    ServiceContainerBuilder::new()
        .register(300i32)
        .register(user)
        .register(Service::new(User {
            id: 33,
            name: "Jibao".to_owned(),
        }))
        .build();

    inject_service(|user: Service<User>| {
        dbg!(user.get_ref());
    });

    inject_service(|| {
        dbg!("this is really cool");
    });
}
