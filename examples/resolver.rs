#[tokio::main]
async fn main() {
    let container = busybody::ServiceContainerBuilder::new()
        .register(0) // 1. We are storing a counter that will be used in the resolver
        // 2. A resolver is a function or closure that returns a future
        .resolver(|container| {
            // - for this example, we are getting the current i32 value stored in the
            // container, adding one to it and re-setting it.
            let current = container.get_type::<i32>().unwrap_or_default() + 1;
            container.set_type(current);

            // 3. Your returned future must be pin
            //   wrap your return type in `Box::pin(async { ... })`
            Box::pin(async move { Id(current) })
        })
        .build();

    for _ in 0..=5 {
        let id = container.get_type::<Id>().unwrap();
        println!("id: {id:?}, inner id: {}", id.0);
    }
}

#[derive(Debug, Clone)]
struct Id(i32);
