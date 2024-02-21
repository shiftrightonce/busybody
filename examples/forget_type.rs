#[tokio::main]
async fn main() {
    // 1. Register i32 type
    busybody::helpers::register_type(42);

    // 2. Get a cloned of the i32 type
    let value = busybody::helpers::get_type::<i32>();
    println!("i32 type value is: {:#?}", value);

    // 3. Forget the i32 type
    let value = busybody::helpers::forget_type::<i32>();
    println!("forgotten i32: {:#?}", value);

    let value = busybody::helpers::forget_type::<i32>();

    println!("No registered i32 type: {:#?}", value);
}
