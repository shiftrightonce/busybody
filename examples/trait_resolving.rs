use std::ops::Deref;

use busybody::Service;

#[tokio::main]
async fn main() {
    // Register resolver
    busybody::helpers::resolvable::<MyGreeterProvider>().await;

    //1.  By default MyGreeterProvider uses "MyGreeter" instance
    let greeter: MyGreeterProvider = busybody::helpers::get_type().await.unwrap();
    println!("my greeter greet: {}", greeter.greet());

    //2. An instance of MyGreeterProvider is created that uses a third party greeter
    busybody::helpers::service_container()
        .set_type(MyGreeterProvider::new(ThirdPartyGreeter))
        .await;

    //3. The following call to get a "greeter provider" will use the third party greeter
    let third_party_greeter: MyGreeterProvider = busybody::helpers::get_type().await.unwrap();
    println!("third party greeter greet: {}", third_party_greeter.greet());
}

trait GreetTrait: Send + Sync {
    fn greet(&self) -> String;
}

#[derive(Clone)]
struct MyGreeter;

impl GreetTrait for MyGreeter {
    fn greet(&self) -> String {
        String::from("Hello from my greeter")
    }
}

struct ThirdPartyGreeter;

impl GreetTrait for ThirdPartyGreeter {
    fn greet(&self) -> String {
        String::from("Hello from third party")
    }
}

#[derive(Clone)]
struct MyGreeterProvider(Service<Box<dyn GreetTrait>>);

impl MyGreeterProvider {
    pub fn new<T: GreetTrait + 'static>(concret: T) -> Self {
        Self(Service::new(Box::new(concret)))
    }
}

// Makes MyGreeterProvider appears as GreetTrait``
impl Deref for MyGreeterProvider {
    type Target = Service<Box<dyn GreetTrait>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[busybody::async_trait]
impl busybody::Resolver for MyGreeterProvider {
    async fn resolve(container: &busybody::ServiceContainer) -> Self {
        let instance = Self(Service::new(Box::new(MyGreeter)));
        container.set_type(instance).await.get_type().await.unwrap()
    }
}
