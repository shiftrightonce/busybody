#![allow(dead_code)]
use busybody::*;

#[derive(Debug)]
struct DbClient {
    pub id: i32,
    pub active: bool,
}

#[derive(Debug)]
struct RedisClient(usize);

#[tokio::main]
async fn main() {
    let meaning_of_life = 44;
    let connection = DbClient {
        id: 9343434,
        active: true,
    };

    // 1. Using the service builder, register services
    let container = ServiceContainerBuilder::new()
        .register(meaning_of_life)
        .await
        .service(connection)
        .await
        .build();

    //1b. Or user the helper function
    helpers::service_container().set_type(99usize).await;

    // 2. Get things from the container
    let meaning = container.get_type::<i32>().await;
    let client = container.get::<DbClient>().await;

    // 3. Add or update things in the container
    container.set(RedisClient(34343_usize)).await; // set Wraps it's value in a Service<T>

    println!("meaning of life: {:#?}", meaning.unwrap());
    println!("db connection: {:#?}", client.unwrap());
    println!(
        "redis client: {:#?}",
        container.get::<RedisClient>().await.unwrap()
    ); // Get back the wrapped type
    println!(
        "get back the usize: {:#?}",
        helpers::service_container().get_type::<usize>().await
    );
}
