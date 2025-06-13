# Changelog

This file documents all notable changes to Hermes.

## 0.1 — alpha

In this version, the API will not change a lot, but it will grow very fast.

### 0.1.2 (todo)

* Finalize the routing system.

### 0.1.1 (unreleased)

* Enhance the client:
  - New syntax `hermes-client [OPTIONS] <METHOD> <URL> [<BODY>]`.
  - Add `-H/--header` option to specify headers multiple times.
* Write a test: Test the server with multiple connections.
* Refactor the client and server structures to be real objects.
* Prototype a routing system.

### 0.1.0 — project bootstrap

```
+ Uri, Authority, UserInfo, Path, Query
+ Message, Version, Request, Method, Response, Status
+ Unit tests
+ Client and Server prototypes
```