use busybody::helpers::{make_task_proxy, service_container};

#[tokio::main]
async fn main() {
    // 1. Global discount
    service_container().set_type(0.25_f64).await;

    // 2. Task 1
    _ = tokio::task::spawn(async {
        // 2.1. Create a task proxy container that will be dropped when this task ends
        let container = make_task_proxy();
        container.set_type(0.50_f64).await;
        container.set_type(150_i64).await;

        let container = make_task_proxy();

        println!(
            "task 1 discounted to : {}",
            container.resolve_and_call(apply_discount).await
        );
    })
    .await;

    // 3. Task 3
    _ = tokio::task::spawn(async {
        let container = make_task_proxy();
        container.set_type(0.65_f64).await;
        container.set_type(150_i64).await;

        println!(
            "task 2 discounted to : {}",
            container.resolve_and_call(apply_discount).await
        );
    })
    .await;
}

async fn apply_discount(amount: i64, dis: f64) -> i64 {
    if dis > 0.0 {
        amount - (((amount as f64) * dis) as i64)
    } else {
        amount
    }
}
