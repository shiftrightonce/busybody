# Busybody

**Busybody is a service container and dependency injector for rust application.**
---

```toml
[dependencies]
busybody = "0.1.0"
async-trait = "0.1.68"# For ease of implementing async function in a trait,
```


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

  let config = container.get::<Config>().unwrap(); // When "some" will return Service<Config>
  let max_connection = container.get_type::<i32>().unwrap(); // When "some" will return i32

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

#[async_trait(?Send)]
impl busybody::Injectable for Config { // implementing "injector" makes your type injectable

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

#[async_trait(?Send)]
impl busybody::Injectable for Config { // implementing "injector" makes your type injectable

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