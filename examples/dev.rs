#[tokio::main]
async fn main() {
    busybody::helpers::service_container().set(100);

    println!("ready...");
}
