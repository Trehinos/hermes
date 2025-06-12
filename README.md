# Hermes

Hermes is the seed of a complete HTTP framework written in Rust. While it
currently focuses on clean abstractions for HTTP messages, the long term goal
is to offer a fully fledged web framework capable of replacing PHP stacks. The
project will gradually grow to include:

* controllers to route requests and invoke business logic
* security features for authentication and authorization
* wrappers around database connections
* an ORM layer for working with persistent data
* a template system for rendering responses

## Building

```bash
cargo build
```

## Running tests

```bash
cargo test
```

At this stage the crate offers utilities for parsing and generating HTTP
messages. All core types are available under the `http` module.
