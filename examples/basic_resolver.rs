use busybody::helpers;

#[tokio::main]
async fn main() {
    let container = busybody::ServiceContainerBuilder::new()
        // 1. Register a resolver that resolves requests for the i32 type
        .resolver(|_container| Box::pin(async { 200 }))
        .await
        .build();

    // 2. Using the helper function to register a resolver for f32 type
    helpers::resolver(|_| Box::pin(async { 10_000.00_f32 })).await;

    // 3. get an i32 value from the container
    let magic_number: i32 = container.get_type().await.unwrap();
    println!("the magic number is: {}", magic_number);

    // 3. get a f32 value from the container
    let bank_balance: f32 = container.get_type().await.unwrap();
    println!("bank balance: {}", bank_balance);
}
