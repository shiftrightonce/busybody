#[tokio::main]
async fn main() {
    _ = busybody::ServiceContainerBuilder::new()
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

    // 4. Here we are creating another container that has instances of the types we are interested in
    //    This feature allows you  to override a registered instance of a type or it's resolver
    let container = busybody::ServiceContainerBuilder::new()
        .register(Id(200))
        .await
        .register(Greeting("Welcome to this big world".to_string()))
        .await
        .build();

    for _ in 0..=5 {
        let (id, greeting): (Id, Greeting) = container.resolve_all().await; // 6. Using our second container directly

        // let (id, greeting) = container.resolve_all::<(Id, Greeting)>(); // The above line could be written like this

        println!("id: {id:?} // {greeting:?}");
    }

    println!(">>>>>>>>>>>>> using 'resolve_all_with' <<<<<<<<<<<<<");

    for _ in 0..5 {
        let (id, greeting): (Id, Greeting) = busybody::helpers::resolve_all_with(&container).await; // 7. Using the helper function `resolve_all_with`  and passing a reference of the container
        println!("id: {id:?} // {greeting:?}");
    }
}

#[derive(Debug, Clone)]
#[allow(unused)]
struct Id(i32);

#[derive(Debug, Clone)]
#[allow(unused)]
struct Greeting(String);
