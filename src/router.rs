//! Simple request router.
//!
//! The router stores a list of [`Route`] definitions and can match an incoming
//! [`Request`](crate::http::Request) to the first route that satisfies all
//! conditions (path, method and required headers).

use crate::concepts::Dictionary;
use crate::http::{Headers, Method, Request};

/// A single route definition used by the [`Router`].
#[derive(Debug, Clone)]
pub struct Route {
    /// URL pattern of the form `/path/{parameter}`.
    pub pattern: String,
    /// Allowed HTTP methods for this route.
    pub methods: Vec<Method>,
    /// Required headers that must be present with matching values.
    pub headers: Headers,
}

impl Route {
    /// Create a new [`Route`].
    pub fn new(pattern: &str, methods: Vec<Method>, headers: Headers) -> Self {
        Self {
            pattern: pattern.to_string(),
            methods,
            headers,
        }
    }

    /// Check whether `req` matches this route.
    fn matches(&self, req: &Request) -> Option<Dictionary<String>> {
        // check method
        if !self.methods.is_empty() && !self.methods.contains(&req.method) {
            return None;
        }
        // check headers
        for (key, values) in self.headers.iter() {
            if let Some(v) = req.message.headers.get(key) {
                if v != values {
                    return None;
                }
            } else {
                return None;
            }
        }
        self.match_path(&req.target.path.to_string())
    }

    /// Match the path part of the URL and extract parameters.
    fn match_path(&self, path: &str) -> Option<Dictionary<String>> {
        let mut params = Dictionary::new();
        let pattern_parts: Vec<&str> = self.pattern.trim_matches('/').split('/').collect();
        let path_parts: Vec<&str> = path.trim_matches('/').split('/').collect();
        if pattern_parts.len() != path_parts.len() {
            return None;
        }
        for (p, val) in pattern_parts.iter().zip(path_parts.iter()) {
            if p.starts_with('{') && p.ends_with('}') {
                let name = &p[1..p.len() - 1];
                params.insert(name.to_string(), val.to_string());
            } else if p != val {
                return None;
            }
        }
        Some(params)
    }
}

/// Result of a successful route match.
#[derive(Debug)]
pub struct RouteMatch<'a> {
    /// Reference to the matched route.
    pub route: &'a Route,
    /// Captured parameters from the URL pattern.
    pub params: Dictionary<String>,
}

/// Collection of [`Route`]s able to select one for a given request.
#[derive(Debug, Default)]
pub struct Router {
    routes: Vec<Route>,
}

impl Router {
    /// Create an empty [`Router`].
    pub fn new() -> Self {
        Self { routes: Vec::new() }
    }

    /// Register a new route.
    pub fn add_route(&mut self, route: Route) {
        self.routes.push(route);
    }

    /// Iterate over registered routes.
    pub fn iter(&self) -> impl Iterator<Item = &Route> {
        self.routes.iter()
    }

    /// Attempt to match `req` against registered routes.
    pub fn match_request(&self, req: &Request) -> Option<RouteMatch> {
        for route in &self.routes {
            if let Some(params) = route.matches(req) {
                return Some(RouteMatch { route, params });
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http::{Authority, Path, Query};
    use crate::http::{Headers, Message, Request, Uri};

    fn request(method: Method, path: &str) -> Request {
        let uri = Uri::new(
            String::new(),
            Authority::default(),
            Path::new(path.to_string(), None),
            Query::new(),
            None,
        );
        Request {
            method,
            target: uri,
            message: Message::v1_1(Headers::new(), String::new()),
        }
    }

    #[test]
    fn test_basic_match() {
        let mut router = Router::new();
        router.add_route(Route::new("/foo/{id}", vec![Method::Get], Headers::new()));

        let req = request(Method::Get, "/foo/42");
        let result = router.match_request(&req).unwrap();
        assert_eq!(result.params.get("id"), Some(&"42".to_string()));
    }

    #[test]
    fn test_method_and_header_mismatch() {
        let mut headers = Headers::new();
        headers.add("X-Test", "1");
        let route = Route::new("/a/{b}", vec![Method::Post], headers.clone());
        let mut router = Router::new();
        router.add_route(route);

        let req = request(Method::Get, "/a/val");
        assert!(router.match_request(&req).is_none());

        let mut req = request(Method::Post, "/a/val");
        req.message.headers = Headers::new();
        assert!(router.match_request(&req).is_none());

        req.message.headers = headers;
        assert!(router.match_request(&req).is_some());
    }
}
