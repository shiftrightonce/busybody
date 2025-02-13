use busybody::{helpers, Injectable, ServiceContainer, ServiceContainerBuilder};
use rand::Rng;

#[tokio::main]
async fn main() {
    // Point 1. Setup the service container
    _ = ServiceContainerBuilder::new()
        // Point 2. Register an instance of the application configuration as a `service` (thread safe)
        .service(AppConfig {
            api_token: "token12345".into(),
        })
        .await
        .build();

    // Point 3. Auto instantiate an instance of `DailyInvoicesFetcher`
    //          We are able to `build` an instance because `DailyInvoicesFetcher` implements `Injectable`
    let invoice_fetcher = helpers::provide::<DailyInvoicesFetcher>().await;
    println!(
        "client id: {}. invoices fetched: {:#?}",
        invoice_fetcher.id,
        invoice_fetcher.fetch().await
    );

    // Point 4. Each call to provide by default returns a new instance of the type
    let invoice_fetcher = helpers::provide::<DailyInvoicesFetcher>().await;
    println!(
        "client id: {}. invoices fetched: {:#?}",
        invoice_fetcher.id,
        invoice_fetcher.fetch().await
    );

    // Point 5. Another way to get an instance of the type.
    //          We can call the static method "instance" on the type/struct
    let invoice_fetcher2 = DailyInvoicesFetcher::instance().await;
    println!(
        "client id: {}. invoices fetched: {:#?}",
        invoice_fetcher2.id,
        invoice_fetcher2.fetch().await
    );
}

#[derive(Debug)]
struct AppConfig {
    pub api_token: String,
}

#[derive(Debug)]
struct DailyInvoicesFetcher {
    api_token: String,
    id: u32,
}

impl DailyInvoicesFetcher {
    pub async fn fetch(&self) -> Vec<String> {
        let mut rng = rand::rng(); // for random numbers generation

        println!("fetching invoices using token: {:#?}", self.api_token);
        let total = rng.random_range(0..15);
        let mut invoices = Vec::with_capacity(total);

        for _ in 0..total {
            invoices.push(format!("Invoice: {}", rng.random::<u32>()));
        }

        invoices
    }
}

#[busybody::async_trait]
impl Injectable for DailyInvoicesFetcher {
    async fn inject(container: &ServiceContainer) -> Self {
        let api_token = container
            .get::<AppConfig>()
            .await
            .unwrap()
            .api_token
            .clone(); // Using the container, we are plucking the registered AppConfig's instance
        Self {
            api_token,
            id: rand::random(),
        } // Create a new instance for each call
    }
}
