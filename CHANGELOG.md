# Changelog

This file documents all notable changes to Hermes.

## 0.1 — alpha

In this version, the API will not change a lot, but it will grow very fast.

### 0.1.2 (unreleased)

* Finalize the routing system.
* Move the HTTP client and server under the `http::services` namespace.
* Reorganize the HTTP modules into separate namespaces.
* Provide request helpers for common HTTP methods and JSON formatting utilities.
* Add route groups with generic controllers and middleware defaults.
* Reorder middleware parameters for consistency.
* Fix host header preservation in `Request::with_uri`.
* Reduce required Tokio features.
* Standardize American English in documentation and update dependencies.

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
