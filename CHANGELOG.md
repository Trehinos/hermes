# Changelog

This file documents all notable changes to Hermes.

## 0.1 — alpha

In this version, the API will not change a lot, but it will grow very fast.

### 0.1.3 (unreleased)
* Introduce a `session` module exposing a `SessionStore` trait and a
  default file-based implementation.
* Document the default backend and customisation points in the roadmap and README.
* Move the `session` module under `http` for better organisation.
* Support storing `Value` objects in sessions and allow choosing a
  `ValueFormatter` for the file-based backend (JSON by default).
* Provide cookie parsing utilities and helpers to set cookies on responses.
* Add `generate_id` to create secure session identifiers.
* Integrate a router and session/cookie handling into the asynchronous server.
* The HTTP client now stores and sends cookies automatically.

### 0.1.2

* Finalize the routing system.
* Move the HTTP client and server under the `http::services` namespace.
* Reorganize the HTTP modules into separate namespaces.
* Provide request helpers for common HTTP methods and JSON formatting utilities.
* Add route groups with generic controllers and middleware defaults.
* Reorder middleware parameters for consistency.
* Fix host header preservation in `Request::with_uri`.
* Reduce required Tokio features.
* Standardize American English in documentation and update dependencies.
* Introduce a simple dependency injection container.
* Allow registering multiple instances of the same type in the container.
* Name each instance with a string to retrieve a specific one.

### 0.1.1

* Enhance the client:
  - New syntax `hermes-client [OPTIONS] <METHOD> <URL> [<BODY>]`.
  - Add `-H/--header` option to specify headers multiple times.
* Write a test: Test the server with multiple connections.
* Refactor the client and server modules into dedicated objects.
* Prototype of a routing system.


### 0.1.0 — project bootstrap

* Uri, Authority, UserInfo, Path, Query
* Message, Version, Request, Method, Response, Status
* Unit tests
* Client and Server prototypes
