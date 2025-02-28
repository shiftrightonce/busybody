use busybody::{Resolver, ServiceContainer};

#[tokio::main]
async fn main() {
    let container = busybody::ServiceContainerBuilder::new()
        .register(0) // 1. We are storing a counter that will be used in the resolver
        .await
        // 2. A resolver is a function or closure that returns a future
        .resolver(|container| {
            // 3. Your returned future must be pin
            //   wrap your return type in `Box::pin(async { ... })`
            Box::pin(async move {
                // - For this example, we are getting the current i32 value stored in the
                // container, adding one to it and re-setting it.
                let current = container.get_type::<i32>().await.unwrap_or_default() + 1;
                container.set_type(current).await;
                Id(current)
            })
        })
        .await
        // 4. Or a type that implements Resolver
        .resolvable::<Bonus>()
        .await
        .build();

    for _ in 0..=5 {
        let id = container.get_type::<Id>().await.unwrap();
        let bonus = container.get_type::<Bonus>().await.unwrap();
        println!("id: {}, bonus: {}", id.0, bonus.0);
    }
}

#[derive(Debug, Clone)]
struct Id(i32);

#[derive(Debug, Clone)]
struct Bonus(i32);
#[async_trait::async_trait]
impl Resolver for Bonus {
    async fn resolve(container: &ServiceContainer) -> Self {
        let current = container.get_type::<i32>().await.unwrap_or_default();
        Self(current * 2)
    }
}
