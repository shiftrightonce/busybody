#![allow(dead_code)]
use busybody::{helpers::service_container, *};

#[derive(Debug)]
struct DbClient {
    pub id: i32,
    pub active: bool,
}

#[derive(Debug)]
struct RedisClient(usize);

fn main() {
    let meaning_of_life = 44;
    let connection = DbClient {
        id: 9343434,
        active: true,
    };

    // 1. Use helper functions
    helpers::register_type(meaning_of_life);
    helpers::register_service(connection);

    // 2. Get things from the container by using the function `service_container`
    let meaning: Option<i32> = service_container().get_type(); // `get_type` returns things set with `register`
    let client = service_container().get::<DbClient>();

    // 3. Add or update things in the container via the function `service_container`
    service_container().set(RedisClient(34343_usize)); // set Wraps value in a Service<T>

    println!("meaning of life: {:#?}", meaning.unwrap());
    println!("db connection: {:#?}", client.unwrap());
    println!(
        "redis client: {:#?}",
        service_container().get::<RedisClient>().unwrap()
    ) // Get back the wrapped type
}
