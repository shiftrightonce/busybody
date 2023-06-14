# Basic2 webapp
---

**This example illustrates how instances of types can be automatically injected/provided**

The application is displaying the server uptime like in `basic_webapp` example and the time it took for routes handler to execute a "task".

Server start time starts when we handle the `first` request.

This example shows you how to make your type `injectable`. All that is require
is to implement `injectable` as follows:

```rust
#[busybody::async_trait]
impl busybody::Injectable for HandlerExecutionTime {
    async fn inject(container: &ServiceContainer) -> Self {
      // You can do one or all of the following:

      // 1. Using the container fetch a registered service
      // 2. Use "provide", "service" or "singleton" helpers to inject an instance of a type
      // 3. Fetch an instance of this type from the service container

       Self::new();// return an instance of the type
    }
}
```
