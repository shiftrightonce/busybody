use std::{sync::OnceLock, thread};

use crate::{helpers::inject_service, service_container::ServiceContainerBuilder};
mod handlers;
mod helpers;
mod injectables;
mod service;
mod service_container;
use injectables::Injectable;
use service::Service;

fn main() {
    ServiceContainerBuilder::new()
        .register(Service::new(User {
            id: 65,
            name: "Jibao".into(),
        }))
        .register(Service::new(Grade { id: 5000 }))
        .register(Service::new(LoggedInUser::new()))
        .build();

    // Inject the User instance and call the closure
    inject_service(|user: Service<User>| {
        println!(
            "we go the injected user: {:#?}, name: {:#?}",
            user.as_ref().id,
            user.as_ref().name
        );
    });

    // Inject the Grade instance and call the closure
    inject_service(|grade: Service<Grade>| {
        println!("we go the injected grade: {:#?}", grade.as_ref().id);
    });
    // Inject both User and Grade instance and call the closure
    inject_service(|grade: Service<Grade>, user: Service<User>| {
        println!(
            "we go the injected grade: {:#?} and user: {:#?}",
            grade.id, &user.name
        );
    });

    inject_service(|logged_in: Service<LoggedInUser>| {
        dbg!(&logged_in.user());
    });

    let handle = thread::spawn(|| {
        inject_service(|logged_in: Service<LoggedInUser>| {
            dbg!(&logged_in.user());
        });
    });

    handle.join().unwrap();
}

struct LoggedInUser {
    user: OnceLock<User>,
}

impl LoggedInUser {
    pub fn new() -> Self {
        Self {
            user: OnceLock::new(),
        }
    }

    pub fn user(&self) -> &User {
        return self.user.get_or_init(|| {
            println!("creating user for the first time");
            User {
                id: 20000,
                name: "Logged in User".into(),
            }
        });
    }
}

impl Injectable for LoggedInUser {
    fn inject(_container: &service_container::ServiceContainer) -> Self {
        Self::new()
    }
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
