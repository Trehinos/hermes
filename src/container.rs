//! Dependency injection container.
//!
//! The `Container` stores services indexed by their type and returns them as
//! shared references. Services are registered as singletons using
//! [`register`]. Controllers can receive the container as context to retrieve
//! required dependencies.
//!
//! # Example
//!
//! ```
//! use hermes::container::Container;
//! use hermes::http::routing::router::{Route, Router};
//! use hermes::http::{Headers, Method, Request, Response, ResponseFactory, Status, Version};
//!
//! fn ping(ctx: &Container, _req: &mut Request) -> Response {
//!     let value = ctx.resolve::<u32>().unwrap();
//!     assert_eq!(*value, 42);
//!     ResponseFactory::version(Version::Http1_1).with_status(Status::OK, Headers::new())
//! }
//!
//! let mut container = Container::new();
//! container.register(42u32);
//!
//! let mut router: Router<Container> = Router::new();
//! router.add_route(Route::new("/ping", vec![Method::Get], Headers::new(), Box::new(ping)));
//! ```
//!
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::Arc;

/// Simple service container holding singleton instances.
#[derive(Default)]
pub struct Container {
    services: HashMap<TypeId, Arc<dyn Any + Send + Sync>>,
}

impl Container {
    /// Create an empty container.
    pub fn new() -> Self {
        Self {
            services: HashMap::new(),
        }
    }

    /// Register a service instance of type `T`.
    pub fn register<T: Any + Send + Sync>(&mut self, service: T) {
        self.services.insert(TypeId::of::<T>(), Arc::new(service));
    }

    /// Retrieve a service of type `T` if present.
    pub fn resolve<T: Any + Send + Sync>(&self) -> Option<Arc<T>> {
        self.services
            .get(&TypeId::of::<T>())
            .and_then(|s| s.clone().downcast::<T>().ok())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_and_resolve() {
        let mut c = Container::new();
        c.register::<u32>(1);
        let value = c.resolve::<u32>().unwrap();
        assert_eq!(*value, 1);
    }
}
