#[tokio::main]
async fn main() {
    busybody::helpers::set_type(300.0_f32).await;
    let x = busybody::helpers::resolve_all::<f32>().await;
    println!("ready...{}", x);
}
