#[tokio::main]
async fn main() {
    // set an i32 in the global container
    busybody::helpers::set_type(600).await;

    // create a task proxy container
    // since this is called outside of a task, an error will be returned
    let ci = busybody::helpers::make_task_proxy();
    assert_eq!(ci.is_err(), true);

    // spawn a task (task 1)
    _ = tokio::task::spawn(async {
        let ci = busybody::helpers::make_task_proxy().unwrap();
        let c2 = busybody::helpers::make_task_proxy().unwrap();

        // set 6 as the value for i32 in this task's context
        ci.set_type(6).await;

        println!("task 1 i32 value 1: {:?}", ci.get_type::<i32>().await);
        println!(
            "task 1 i32 value 1a: {:?}",
            busybody::helpers::get_type::<i32>().await
        );

        // Using another instance will still point to the same task instance
        c2.set_type(77).await;

        println!("task 1 i32 value 2: {:?}", ci.get_type::<i32>().await);

        // A normal proxy will fallback first to the task proxy before checking
        // the global container.
        println!(
            "task 1 i32 value 3 via proxy: {:?}",
            busybody::helpers::make_proxy().get_type::<i32>().await
        );

        // Values set on a proxy will be limited to that proxy
        let proxy = busybody::helpers::make_proxy();
        proxy.set_type(55).await;

        // proxy value will be 55
        println!(
            "task 1 i32 value 3 via proxy 2: {:?}",
            proxy.get_type::<i32>().await
        );

        // task context value is still 77
        println!("task 1 i32 value 5: {:?}", ci.get_type::<i32>().await);
    })
    .await;

    // another task (task 2)
    _ = tokio::task::spawn(async {
        let ci = busybody::helpers::make_task_proxy().unwrap();

        // value will be 600
        println!("task 2 i32 value: {:?}", ci.get_type::<i32>().await);
    })
    .await;
}
