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

fn main() {
  let container = ServiceContainerBuilder::new()
  .service(Config{ hostname: "http://localhost".into() }) // Will be wrapped in Service<T> ie: Arc<T>
  .register(600i32) // left as it is, i32
  .build();

  let config = container.get::<Config>().unwrap(); // When "some", will return Service<Config>
  let max_connection = container.get_type::<i32>().unwrap(); // When "some", will return i32

  println!("config: {:#?}", &config);
  println!("hostname: {:#?}", &config.hostname);
  println!("max connection: {}", max_connection);
}
```

</details>


## Busybody as a dependency injector
<details>
  <summary>
    Dependency injection example
  </summary>

```rust
use busybody::*;
use async_trait::async_trait;

#[derive(Debug)]
struct Config {
  hostname: String
}

#[busybody::async_trait(?Send)]
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
    Dependency injection singleton example
  </summary>

```rust
use busybody::*;
use async_trait::async_trait;

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
use busybody::{helpers, RawType, Service, ServiceContainerBuilder};

#[tokio::main]
async fn main() {
    // 1. Setup the container
    _ = ServiceContainerBuilder::new()
        .register(200) // Register an i32 value that is not wrapped in Service<T>
        .service(400) // Register an i32 value that is wrapped in Service<T>
        .build();

    // 2. `inject_and_call` calls the provided function/closure, injecting all of it's required parameters
    //     inject_and_call takes a function/closure that expects 0 to 17 arguments
    //     The function **must** be async
    let double_result = helpers::inject_and_call(double).await;
    println!("200 double is: {}", double_result);

    // 3. Same as above but we are making use of "RawType<T>"
    //    RawType<T> trys to find an instance of the speicified type. If none exist,
    //    it uses the `default` associate method to create a default instance of the Type.
    //    This means, the "T" in RawType must implement the `Default` trait.
    let sum = helpers::inject_and_call(|raw_i32: RawType<i32>, service_i32: Service<i32>| async {
        raw_i32.into_inner() + *service_i32.into_inner()
    })
    .await;
    println!("Service<200> + RawType<400> = {}", sum);
}

// 4. Function is taken an I32.
//    RawType<T> trys to find an instance of the speicified type. If none exist,
//    it uses the `default` associate method to create a default instance of the Type.
//    This means, the "T" in RawType must implement the `Default` trait.
async fn double(count: RawType<i32>) -> i32 {
    *count * 2
}

```

</details>

## Examples
The [examples](https://github.com/shiftrightonce/busybody/tree/main/examples) folder contains simple and full examples. If none of the examples are helpful,
please reach out with your use case and I  try to provide one.

## License

### The MIT License (MIT)

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.