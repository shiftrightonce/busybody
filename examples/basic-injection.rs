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
        let mut rng = rand::thread_rng(); // for random numbers generation

        println!("fetching invoices using token: {:#?}", self.api_token);
        let total = rng.gen_range(0..15);
        let mut invoices = Vec::with_capacity(total);

        for _ in 0..total {
            invoices.push(format!("Invoice: {}", rng.gen::<u32>()));
        }

        invoices
    }
}

#[busybody::async_trait(?Send)]
impl Injectable for DailyInvoicesFetcher {
    async fn inject(container: &ServiceContainer) -> Self {
        let mut rng = rand::thread_rng(); // for random numbers generation
        let api_token = container.get::<AppConfig>().unwrap().api_token.clone(); // Using the container, we are plucking the registered AppConfig's instance
        Self {
            api_token,
            id: rng.gen(),
        } // Create a new instance for each call
    }
}
