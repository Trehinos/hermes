# Hermes

Hermes is the seed of a complete HTTP framework written in Rust. While it
currently focuses on clean abstractions for HTTP messages and minimal
networking components, the long‑term goal is to offer a fully fledged web
framework capable of replacing typical PHP stacks.

## Features

- Utilities for parsing and generating HTTP messages exposed under the `http`
  module.
- A minimal asynchronous client for performing requests, available under
  the `http::services` module.
- A lightweight asynchronous server used in examples and tests, also under
  `http::services`.
- A simple router and `Controller` trait to handle incoming requests.

## Building

```bash
cargo build
```

## Running tests

```bash
cargo test
```

At this stage the crate offers utilities for parsing and generating HTTP
messages. All core types are available under the `http` module. It also ships
with a minimal asynchronous client and server used in the tests and examples.

## Quick example

```rust
use hermes::http::services::client::Client;
use hermes::http::ResponseTrait;

# tokio_test::block_on(async {
let resp = Client::get("http://example.com").await.unwrap();
println!("Status: {}", resp.code());
# });
```

## Roadmap

The project will evolve into a complete backend framework. Upcoming milestones
include an asynchronous server built on Tokio and Hyper, a richer routing
system with middleware and dependency injection, session and security
mechanisms, database access through an ORM, a template engine, CLI tools and
continuous integration. Advanced features like form handling, background tasks
and optional WebSocket support are also planned. See `ROADMAP.md` for more
details.
