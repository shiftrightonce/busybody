use busybody::helpers::service_container;

#[tokio::main]
async fn main() {
    let container_1 = busybody::ServiceContainer::proxy(); // 2. Create a proxy container will first try to resolve types and then fallback to the global container
    container_1.set_type(1).await; // 3. Store an i32 value

    let container_2 = busybody::ServiceContainer::proxy();
    container_2.set_type(2).await; // 4. Store an i32 value in container 2

    // 5. Store the instance of `Life` in the global container
    service_container().set_type(Life(42)).await;

    // 6. Both container have a local instance of an i32 value
    assert_ne!(
        container_1.get_type::<i32>().await.unwrap(),
        container_2.get_type::<i32>().await.unwrap(),
        "container 1 and 2 should be completely independent"
    );

    // 7. Both container don't have an instance of `Life`
    assert_eq!(
        container_1.get_type::<Life>().await.unwrap().meaning(),
        container_2.get_type::<Life>().await.unwrap().meaning(),
        "expected 42"
    );
}

#[derive(Debug, Clone)]
struct Life(i32);

impl Life {
    pub fn meaning(&self) -> i32 {
        self.0
    }
}
