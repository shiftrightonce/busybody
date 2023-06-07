use crate::{helpers::inject_service, service_container::ServiceContainerBuilder};
mod handlers;
mod helpers;
mod injectables;
mod service;
mod service_container;
use service::Service;

fn main() {
    ServiceContainerBuilder::new()
        .register(Service::new(User {
            id: 65,
            name: "Jibao".into(),
        }))
        .register(Service::new(Grade { id: 5000 }))
        .build();

    inject_service(|user: Service<User>| {
        println!(
            "we go the injected user: {:#?}, name: {:#?}",
            user.as_ref().id,
            user.as_ref().name
        );
    });

    inject_service(|grade: Service<Grade>| {
        println!("we go the injected grade: {:#?}", grade.as_ref().id);
    });
    inject_service(|grade: Service<Grade>, user: Service<User>| {
        println!(
            "we go the injected grade: {:#?} and user: {:#?}",
            grade.id, &user.name
        );
    });
}

#[derive(Debug)]
struct User {
    id: i32,
    name: String,
}

#[derive(Debug)]
struct Grade {
    id: i32,
}
