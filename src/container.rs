//! Dependency injection container.
//!
//! The `Container` stores services indexed by their type and returns them as
//! shared references. Multiple instances of the same type can be registered and
//! retrieved. Each instance is associated with a name to allow resolving a
//! particular service. Controllers can receive the container as context to
//! obtain the required dependencies.
//!
//! # Example
//!
//! ```
//! use hermes::container::Container;
//! use hermes::http::routing::router::{Route, Router};
//! use hermes::http::{Headers, Method, Request, Response, ResponseFactory, Status, Version};
//!
//! fn ping(ctx: &Container, _req: &mut Request) -> Response {
//!     let value = ctx.resolve_named::<u32>("answer").unwrap();
//!     assert_eq!(*value, 42);
//!     ResponseFactory::version(Version::Http1_1).with_status(Status::OK, Headers::new())
//! }
//!
//! let mut container = Container::new();
//! container.register_named("answer", 42u32);
//!
//! let mut router: Router<Container> = Router::new();
//! router.add_route(Route::new("/ping", vec![Method::Get], Headers::new(), Box::new(ping)));
//! ```
//!
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::Arc;

/// Simple service container holding one or more instances per type.
#[derive(Default)]
pub struct Container {
    services: HashMap<TypeId, HashMap<String, Arc<dyn Any + Send + Sync>>>,
}

impl Container {
    /// Create an empty container.
    pub fn new() -> Self {
        Self {
            services: HashMap::new(),
        }
    }

    /// Register a service instance of type `T` with a custom name.
    pub fn register_named<T: Any + Send + Sync>(&mut self, name: impl Into<String>, service: T) {
        self.services
            .entry(TypeId::of::<T>())
            .or_insert_with(HashMap::new)
            .insert(name.into(), Arc::new(service));
    }

    /// Register a service instance of type `T` using the default name `"default"`.
    pub fn register<T: Any + Send + Sync>(&mut self, service: T) {
        self.register_named("default", service);
    }

    /// Retrieve a service of type `T` if present.
    pub fn resolve<T: Any + Send + Sync>(&self) -> Option<Arc<T>> {
        self.services
            .get(&TypeId::of::<T>())
            .and_then(|map| {
                map.get("default")
                    .cloned()
                    .or_else(|| map.values().next().cloned())
            })
            .and_then(|s| s.downcast::<T>().ok())
    }

    /// Retrieve a named service of type `T` if present.
    pub fn resolve_named<T: Any + Send + Sync>(&self, name: &str) -> Option<Arc<T>> {
        self.services
            .get(&TypeId::of::<T>())
            .and_then(|map| map.get(name).cloned())
            .and_then(|s| s.downcast::<T>().ok())
    }

    /// Retrieve all services of type `T`.
    ///
    /// ```
    /// use hermes::container::Container;
    ///
    /// let mut c = Container::new();
    /// c.register_named::<u32>("one", 1);
    /// c.register_named::<u32>("two", 2);
    /// let values = c.resolve_all::<u32>();
    /// assert_eq!(values.len(), 2);
    /// ```
    pub fn resolve_all<T: Any + Send + Sync>(&self) -> Vec<Arc<T>> {
        self.services
            .get(&TypeId::of::<T>())
            .map(|map| {
                map.values()
                    .filter_map(|s| s.clone().downcast::<T>().ok())
                    .collect()
            })
            .unwrap_or_default()
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

    #[test]
    fn named_registration() {
        let mut c = Container::new();
        c.register_named::<u32>("first", 1);
        c.register_named::<u32>("second", 2);
        let one = c.resolve_named::<u32>("first").unwrap();
        let two = c.resolve_named::<u32>("second").unwrap();
        assert_eq!(*one, 1);
        assert_eq!(*two, 2);
    }

    #[test]
    fn multiple_instances() {
        let mut c = Container::new();
        c.register_named::<u32>("one", 1);
        c.register_named::<u32>("two", 2);
        let all = c.resolve_all::<u32>();
        assert_eq!(all.len(), 2);
        let mut values = all.iter().map(|v| **v).collect::<Vec<_>>();
        values.sort();
        assert_eq!(values, vec![1, 2]);
    }
}
