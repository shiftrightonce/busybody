#![allow(dead_code)]
use busybody::*;
use std::thread;
use std::time::Duration;

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

    let local = container.clone();
    let handle = tokio::spawn(async move {
        for _ in 1..10 {
            // 2. Get things from the container
            let meaning = local.get_type::<i32>().await;
            let client = local.get::<DbClient>().await;

            // 3. Add or update things in the container
            local.set(RedisClient(34343_usize)).await; // set Wraps it's value in a Service<T>

            println!("meaning of life: {:#?}", meaning.unwrap());
            println!("db connection: {:#?}", client.unwrap());
            println!(
                "redis client: {:#?}",
                local.get::<RedisClient>().await.unwrap()
            ); // Get back the wrapped type

            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.await.unwrap();
}
