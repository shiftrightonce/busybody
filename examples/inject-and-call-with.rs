#![allow(dead_code)]
use busybody::{helpers, Injectable, ServiceContainer, Singleton};

#[tokio::main]
async fn main() {
    // 1. Inject and call the function "make_sales_order"
    //    Shipping and handling cost will be 5.50 since that is the default
    let new_sales_order = helpers::inject_and_call(make_sales_order).await;
    println!("second sales order created: {:#?}", new_sales_order);

    // 2. A proxy container is created here
    let container = ServiceContainer::proxy();
    // An instance of the shipping and handling struct is registered
    container.set_type(ShippingAndHandling(10.50));

    // 3. Using `inject_and_call_with`, the proxy container is used as the first source
    //    of dependencies resolving
    let new_sales_order = helpers::inject_and_call_with(&container, make_sales_order).await;
    println!("first proxy sales order created: {:#?}", new_sales_order);

    // 4. Or use the proxy container directly.
    //    Anywhere a service container is required, a proxy container can be used.
    container.inject_and_call(make_sales_order).await;
    println!("second proxy sales order created: {:#?}", new_sales_order);
}

// 3. make_sales_order expect an instance of SalesOrder, the IdGenerator as a singleton and an instance of ShippingAndHandling
async fn make_sales_order(
    mut so: SalesOrder,
    id_generator: Singleton<IdGenerator>,
    shipping_cost: ShippingAndHandling,
) -> SalesOrder {
    so.id = id_generator.make();

    println!(
        "generated ID for sales order {:#?}, number: {:?}",
        so.id, shipping_cost
    );

    so
}

#[derive(Debug, Clone)]
struct ShippingAndHandling(f32);
#[busybody::async_trait]
impl Injectable for ShippingAndHandling {
    async fn inject(c: &busybody::ServiceContainer) -> Self {
        c.get_type().unwrap_or(Self(5.50))
    }
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
