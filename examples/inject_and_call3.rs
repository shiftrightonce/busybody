#![allow(dead_code)]
use busybody::{helpers, Injectable, ServiceContainerBuilder, Singleton};

#[tokio::main]
async fn main() {
    // 1. Setup the container
    _ = ServiceContainerBuilder::new().build();

    // 2. Inject and call the function "make_sales_order"
    let new_sales_order = helpers::inject_and_call(make_sales_order).await;
    println!("first sales order created: {:#?}", new_sales_order);

    // 2. Inject and call the function "make_sales_order"
    //    make_sales_order should receive the same IdGenerator instance
    let new_sales_order = helpers::inject_and_call(make_sales_order).await;
    println!("second sales order created: {:#?}", new_sales_order);
}

// 3. make_sales_order expect an instance of SalesOrder and the IdGenerator as a singleton
//   Note that the SalesOrder instance is not wrapped in  Singleton<T> or Service<T>.
//   It is a plan old instance of the type
async fn make_sales_order(mut so: SalesOrder, id_generator: Singleton<IdGenerator>) -> SalesOrder {
    so.id = id_generator.make();

    println!("generated ID for sales order {:#?}", so.id);

    so
}

#[derive(Debug)]
struct SalesOrder {
    id: String,
    line_items: Vec<SalesOrderLineItem>,
}

#[busybody::async_trait]
impl Injectable for SalesOrder {
    async fn inject(_container: &busybody::ServiceContainer) -> Self {
        Self {
            id: "".into(),
            line_items: Vec::new(),
        }
    }
}

struct IdGenerator;
impl IdGenerator {
    pub fn make(&self) -> String {
        uuid::Uuid::new_v4().to_string()
    }
}

#[busybody::async_trait]
impl busybody::Injectable for IdGenerator {
    async fn inject(_: &busybody::ServiceContainer) -> Self {
        println!("creating a new instance of the IdGenerator");
        Self
    }
}

#[derive(Debug)]
struct SalesOrderLineItem {
    id: String,
    item: String,
    amount: String,
}
