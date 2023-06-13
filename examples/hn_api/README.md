# HN API
---

**An application that fetches hacker news top stories and has the ability to fetch a story. Further requests can be cached**

## Endpoints
 - `/` : List of top stories
 - `item/id` : Fetches an item using it's ID

## Running the application
- `Cargo run` : Run application without caching
- `Cargo run -- cache` : Run application with caching
