use busybody::{
    Resolver, ServiceContainer, ServiceContainerBuilder,
    helpers::{self},
};
use rand::Rng;

#[tokio::main]
async fn main() {
    // Point 1. Setup the service container
    _ = ServiceContainerBuilder::new()
        // Point 2. Register an instance of the application configuration as a `service` (thread safe)
        .service(AppConfig {
            api_token: "super_secret_token".into(),
        })
        .await
        .resolvable_once::<DailyInvoicesFetcher>()
        .await
        .build();

    // Point 3. Auto instantiate an instance of `DailyInvoicesFetcher`
    //          We are able to `build` an instance because `DailyInvoicesFetcher` implements `Injectable`
    let invoice_fetcher = helpers::get_type::<DailyInvoicesFetcher>().await.unwrap();
    println!(
        "client id: {}. invoices fetched: {:#?}",
        invoice_fetcher.id,
        invoice_fetcher.fetch().await
    );

    // Point 4. Subsequent calls to `singleton` returns the same instance
    let invoice_fetcher = helpers::get_type::<DailyInvoicesFetcher>().await.unwrap();
    println!(
        "client id: {}. invoices fetched: {:#?}",
        invoice_fetcher.id,
        invoice_fetcher.fetch().await
    );

    // Point 5: Using `inject_all` you can inject multiple injectable(s)
    let (fetcher, singleton_fetcher) =
        helpers::resolve_all::<(DailyInvoicesFetcher, DailyInvoicesFetcher)>().await; // inject one or more injectable

    println!(
        "new client id: {}. invoices fetched: {:#?}",
        fetcher.id,
        fetcher.fetch().await
    );

    println!(
        "singleton client id: {}. invoices fetched: {:#?}",
        singleton_fetcher.id,
        singleton_fetcher.fetch().await
    );

    // inject a singleton
    helpers::resolve_and_call(|fetcher: DailyInvoicesFetcher| async move {
        use_fetcher(&fetcher).await;
    })
    .await;
}

#[derive(Debug)]
struct AppConfig {
    pub api_token: String,
}

#[derive(Debug, Clone)]
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
impl Resolver for DailyInvoicesFetcher {
    async fn resolve(container: &ServiceContainer) -> Self {
        // let mut rng = rand::rng(); // for random numbers generation
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

async fn use_fetcher(fetcher: &DailyInvoicesFetcher) {
    fetcher.fetch().await;
}
