#[tokio::main]
async fn main() {
    let container = busybody::ServiceContainerBuilder::new()
        .register(0) // 1. We are storing a counter that will be used in the resolver
        .await
        // 2. A resolver is a function or closure that returns a future
        .resolver(|container| async move {
            // 3. Your returned future must be pin
            // - for this example, we are getting the current i32 value stored in the
            // container, adding one to it and re-setting it.
            let current = container.get_type::<i32>().await.unwrap_or_default() + 1;
            container.set_type(current).await;
            Id(current)
        })
        .await
        .resolver(|_| async { Greeting(String::new()) }) // 4. Another resolver. This time for type Greeting
        .await
        .build();

    for _ in 0..=5 {
        container.resolve_and_call(greet_user).await; // 5. arguments to register_user will be resolved
    }
}

async fn greet_user(id: Id, mut greeting: Greeting) {
    greeting.0 = format!("Hello user: {}", id.0);
    println!("id: {id:?} // {greeting:?}");
}

#[derive(Debug, Clone)]
struct Id(i32);

#[derive(Debug, Clone)]
struct Greeting(String);
