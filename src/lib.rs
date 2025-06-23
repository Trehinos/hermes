//! Core library exposing HTTP primitives, an asynchronous client and server.
//!
//! The crate is organised in a few top level modules:
//! - [`http`] exposes all HTTP primitives. The [`http::services`] module
//!   contains the asynchronous client and server used in examples and tests.
//! - [`concepts`] houses utility traits and data structures shared across the
//!   crate.

pub mod concepts;
pub mod container;
pub mod http;
