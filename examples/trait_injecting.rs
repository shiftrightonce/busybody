use std::ops::Deref;

use busybody::Service;

#[tokio::main]
async fn main() {
    //1.  By default MyGreeterProvider uses "MyGreeter" instance
    let greeter: MyGreeterProvider = busybody::helpers::provide().await;
    println!("my greeter greet: {}", greeter.greet());

    //2. An instance of MyGreeterProvider is created that uses a third party greeter
    busybody::helpers::service_container().set_type(MyGreeterProvider::new(ThirdPartyGreeter));

    //3. The following call to provide a "greeter provider" will use the third party greeter
    let third_party_greeter: MyGreeterProvider = busybody::helpers::provide().await;
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
impl busybody::Injectable for MyGreeterProvider {
    async fn inject(container: &busybody::ServiceContainer) -> Self {
        // Get a registered instance or create and register a new one
        match container.get_type::<Self>() {
            Some(exiting) => exiting,
            None => {
                let instance = Self(Service::new(Box::new(MyGreeter)));
                container.set_type(instance).get_type().unwrap()
            }
        }
    }
}
