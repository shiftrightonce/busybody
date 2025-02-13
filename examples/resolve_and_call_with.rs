#[tokio::main]
async fn main() {
    _ = busybody::ServiceContainerBuilder::new()
        .register(0) // 1. We are storing a counter that will be used in the resolver
        .await
        // 2. A resolver is a function or closure that returns a future
        .resolver(|container| {
            // 3. Your returned future must be pin
            //   wrap your return type in `Box::pin(async { ... })`
            Box::pin(async move {
                // - for this example, we are getting the current i32 value stored in the
                // container, adding one to it and re-setting it.
                let current = container.get_type::<i32>().await.unwrap_or_default() + 1;
                container.set_type(current).await;
                Id(current)
            })
        })
        .await
        .resolver(|_| Box::pin(async { Greeting(String::new()) })) // 4. Another resolver. This time for type Greeting
        .await
        .build();

    // 5. A second container that temporarily overrides the registered instances
    let container = busybody::helpers::make_builder()
        .register(Id(6000))
        .await
        .register(Greeting("Welcome :)".to_string()))
        .await
        .build();

    for _ in 0..=5 {
        container.resolve_and_call(greet_user).await; // 6. arguments to register_user will be resolved
    }

    println!(">>>>>>>>>>>>>>>> using helper function `resolve_and_call` <<<<<<<<<<<<<<<");
    for _ in 0..=5 {
        busybody::helpers::resolve_and_call_with(&container, greet_user).await;
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
