use busybody::{helpers, RawType, Service, ServiceContainerBuilder};

#[tokio::main]
async fn main() {
    // 1. Setup the container
    _ = ServiceContainerBuilder::new()
        .register(200) // Register an i32 value that is not wrapped in Service<T>
        .service(400) // Register an i32 value that is wrapped in Service<T>
        .build();

    // 2. `inject_and_call` calls the provided function/closure, injecting all of it's required parameters
    //     inject_and_call takes a function/closure that expects 0 to 17 arguments
    //     The function **must** be async
    let double_result = helpers::inject_and_call(double).await;
    println!("200 double is: {}", double_result);

    // 3. Same as above but we are making use of "RawType<T>"
    //    RawType<T> tries to find an instance of the specified type. If none exist,
    //    it uses the `default` associate method to create a default instance of the Type.
    //    This means, the "T" in RawType must implement the `Default` trait.
    let sum = helpers::inject_and_call(|raw_i32: RawType<i32>, service_i32: Service<i32>| async {
        raw_i32.into_inner() + *service_i32.into_inner()
    })
    .await;
    println!("Service<200> + RawType<400> = {}", sum);
}

// 4. Function is taken an I32.
async fn double(count: i32) -> i32 {
    count * 2
}
