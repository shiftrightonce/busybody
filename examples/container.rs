#![allow(dead_code)]
use busybody::*;

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

    // 1. Using the service builder, register services
    let container = ServiceContainerBuilder::new()
        .register(meaning_of_life)
        .service(connection)
        .build();

    // 2. Get things from the container
    let meaning = container.get_type::<i32>();
    let client = container.get::<DbClient>();

    // 3. Add or update things in the container
    container.set(RedisClient(34343_usize)); // set Wraps value in a Service<T>

    println!("meaning of life: {:#?}", meaning.unwrap());
    println!("db connection: {:#?}", client.unwrap());
    println!(
        "redis client: {:#?}",
        container.get::<RedisClient>().unwrap()
    ) // Get back the wrapped type
}
