#![allow(dead_code)]
use busybody::{helpers::service_container, *};

#[derive(Debug)]
struct DbClient {
    pub id: i32,
    pub active: bool,
}

#[derive(Debug)]
struct RedisClient(usize);

// Container is setup here
async fn setup_container() {
    let meaning_of_life = 44;
    let connection = DbClient {
        id: 9343434,
        active: true,
    };

    // 1. Using the service builder, register services
    _ = ServiceContainerBuilder::new()
        .register(meaning_of_life)
        .await
        .service(connection)
        .await
        .build();
}

#[tokio::main]
async fn main() {
    setup_container().await;

    // 2. Get things from the container by using the function `service_container`
    let meaning = service_container().get_type::<i32>().await; // `get_type` returns things set with `register`
    let client = service_container().get::<DbClient>().await;

    // 3. Add or update things in the container via the function `service_container`
    service_container().set(RedisClient(34343_usize)).await; // set Wraps value in a Service<T>

    println!("meaning of life: {:#?}", meaning.unwrap());
    println!("db connection: {:#?}", client.unwrap());
    println!(
        "redis client: {:#?}",
        service_container().get::<RedisClient>().await.unwrap()
    ) // Get back the wrapped type
}
