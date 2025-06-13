//! Core library exposing HTTP primitives, an asynchronous client and server.
//!
//! The crate is organized in a few top level modules:
//! - [`http`] re-exports the types used to parse and build HTTP messages.
//! - [`client`] provides a minimal asynchronous HTTP client.
//! - [`server`] contains an extremely small asynchronous server used in
//!   examples and tests.
//!
//! The [`concepts`] module houses utility traits and data structures shared
//! across the crate.

pub mod client;
pub mod concepts;
pub mod controller;
pub mod http;
pub mod router;
pub mod server;
