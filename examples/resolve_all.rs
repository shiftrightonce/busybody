use busybody::{Resolver, ServiceContainer};

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
        // 4. Another resolver. This time for type Greeting
        .resolvable::<Greeting>()
        .await
        .build();

    for _ in 0..=5 {
        let (id, mut greeting): (Id, Greeting) = container.resolve_all().await; // 5. Using a tuple, we can resolve one or more types

        // let (id, mut greeting) = container.resolve_all::<(Id, Greeting)>(); // The above line could be written like this

        greeting.0 = format!("Hello user: {}", id.0);
        println!("id: {id:?} // {greeting:?}");
    }
}

#[derive(Debug, Clone)]
struct Id(i32);

#[derive(Debug, Clone)]
struct Greeting(String);

#[async_trait::async_trait]
impl Resolver for Greeting {
    async fn resolve(_: &ServiceContainer) -> Self {
        Self(String::new())
    }
}
