# Change logs

## Version 0.3.0
- Add the ability to inject Option<T> and Result<T, E> where T is injectable

## Version 0.2.0
- Implementor of the `injectable` trait must now be `Send`

```rust
#[busybody::async_trait] // Note: This is no longer #[busybody::async_trait(?Send)] 
impl busybody::Injectable for SomeType {
    async fn inject(container: &ServiceContainer) -> Self {
      // You can do one or all of the following:

      // 1. Using the container fetch a registered service
      // 2. Use "provide", "service" or "singleton" helpers to inject an instance of a type
      // 3. Fetch an instance of this type from the service container

       Self::new();// return an instance of the type
    }
}
```