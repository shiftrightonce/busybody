# Busybody

**Busybody is a service container and dependency injector for rust application.**
---

## Busybody as a service container
<details>
<summary>
  Service container example
</summary>

```rust
use busybody::*;

#[derive(Debug)]
struct Config {
  hostname: String
}

#[tokio::main]
async fn main() {
  let container = ServiceContainerBuilder::new()
  .service(Config{ hostname: "http://localhost".into() }) // Will be wrapped in Service<T> ie: Arc<T>
  .await
  .register(600i32) // left as it is, i32
  .await
  .build();

  let config = container.get::<Config>().await.unwrap(); // When "some", will return Service<Config>
  let max_connection = container.get_type::<i32>().await.unwrap(); // When "some", will return i32

  println!("config: {:#?}", &config);
  println!("hostname: {:#?}", &config.hostname);
  println!("max connection: {}", max_connection);
}
```

</details>


## Busybody as a dependency injector
<details>
  <summary>
     Service Resolver
  </summary>

  ```rust
  use busybody::*;

  #[derive(Debug, Clone)]
  struct Config {
    hostname: String
  }

#[tokio::main]
async fn main() {
  // Whenever an instance of Config is needed
  // this closure will be called
  helpers::resolver(|_container| {
    Box::pin(async {
      Config {
         hostname: "127.0.0.1".to_string(),
      }
    })
  }).await;

  let _config: Config = helpers::get_type().await.unwrap(); // Resolve an instance of Config

  helpers::resolve_and_call(send_invoices).await; // Resolve all the parameters of "send_invoices" and call it.
}

async fn send_invoices(config: Config) {
  println!("sending invoices to: {}", &config.hostname);
}
```

</details>
<details>
  <summary>
    Dependency injection example
  </summary>

```rust
use busybody::*;

#[derive(Debug)]
struct Config {
  hostname: String
}

#[busybody::async_trait]
impl busybody::Injectable for Config { // implementing "Injectable" makes your type callable by the injector 

    async fn inject(_: &ServiceContainer) -> Self {
       Self {
           hostname: "localhost".into()
       }
    }
}


#[tokio::main]
async fn main() {
  let config = helpers::provide::<Config>().await;

  println!("config: {:#?}", &config);
  println!("hostname: {:#?}", &config.hostname);
}
```

</details>

<details>
  <summary>
    Dependency injection: singleton example
  </summary>

```rust
use busybody::*;

#[derive(Debug)]
struct Config {
  hostname: String
}

#[busybody::async_trait]
impl busybody::Injectable for Config { // implementing "Injectable" makes your type injectable by the injector

    async fn inject(_: &ServiceContainer) -> Self {
       Self {
           hostname: "localhost".into()
       }
    }
}


#[tokio::main]
async fn main() {
  let config = helpers::singleton::<Config>().await;

  println!("config: {:#?}", &config);
  println!("hostname: {:#?}", &config.hostname);
}
```

</details>


<details>
  <summary>
    Dependency injection: call a function/closure passing it all the require arguments 
  </summary>

```rust
use busybody::{helpers, Service, ServiceContainerBuilder};

#[tokio::main]
async fn main() {
    // 1. Setup the container
    _ = ServiceContainerBuilder::new()
        .register(200) // Register an i32 value that is not wrapped in Service<T>
        .await
        .service(400) // Register an i32 value that is wrapped in Service<T>
        .await
        .build();

    // 2. `inject_and_call` calls the provided function/closure, injecting all of it's required parameters
    //     inject_and_call takes a function/closure that expects 0 to 17 arguments
    //     The function **must** be async
    let double_result = helpers::inject_and_call(double).await;
    println!("200 double is: {}", double_result);

    // 3. Same as above but we are making use of a Service<T> ie Arc<T>
    //    it uses the `default` associate method to create a default instance of the Type.
    let sum = helpers::inject_and_call(|raw_i32: i32, service_i32: Service<i32>| async move {
        raw_i32 + *service_i32
    })
    .await;
    println!("Service<200> + 400 = {}", sum);
}

// 4. Function is taken an I32.
async fn double(count: i32) -> i32 {
    count * 2
}

```

</details>

## Examples
The [examples](https://github.com/shiftrightonce/busybody/tree/main/examples) folder contains simple and full examples. If none of the examples are helpful,
please reach out with your use case and I  try to provide one.


## Feedback
If you find this crate useful, please star the repository. Submit your issues and recommendations as well.

## License

### The MIT License (MIT)

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.