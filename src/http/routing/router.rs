//! Simple request router.
//!
//! The router stores a list of [`Route`] definitions and can match an incoming
//! [`Request`] to the first route that satisfies all
//! conditions (path, method and required headers).

use crate::concepts::Dictionary;
use crate::http::routing::controller::Controller;
use crate::http::{Headers, Method, Request, RequestTrait, Response, ResponseTrait};

/// A single route definition used by the [`Router`].
pub struct Route<Ctx, Req: RequestTrait = Request, Res: ResponseTrait = Response> {
    /// URL pattern of the form `/path/{parameter}`.
    pub pattern: String,
    /// Allowed HTTP methods for this route.
    pub methods: Vec<Method>,
    /// Required headers that must be present with matching values.
    pub headers: Headers,
    /// Controller handling the request when this route matches.
    pub controller: Box<dyn Controller<Ctx, Req, Res>>,
}

impl<Ctx, Req: RequestTrait, Res: ResponseTrait> core::fmt::Debug for Route<Ctx, Req, Res> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Route")
            .field("pattern", &self.pattern)
            .field("methods", &self.methods)
            .field("headers", &self.headers)
            .finish()
    }
}

impl<Ctx, Req: RequestTrait, Res: ResponseTrait> Route<Ctx, Req, Res> {
    /// Create a new [`Route`].
    pub fn new(
        pattern: &str,
        methods: Vec<Method>,
        headers: Headers,
        controller: Box<dyn Controller<Ctx, Req, Res>>,
    ) -> Self {
        Self {
            pattern: pattern.to_string(),
            methods,
            headers,
            controller,
        }
    }

    /// Check whether `req` matches this route.
    pub fn matches(&self, req: &Req) -> Option<Dictionary<String>> {
        // check method
        if !self.methods.is_empty() && !self.methods.contains(&req.get_method()) {
            return None;
        }
        // check headers
        for (key, values) in self.headers.iter() {
            if let Some(v) = req.headers().get(key) {
                if v != values {
                    return None;
                }
            } else {
                return None;
            }
        }
        self.match_path(&req.get_uri().path.to_string())
    }

    /// Invoke the controller for this route.
    pub fn handle(&mut self, context: &Ctx, req: &mut Req) -> Res {
        self.controller.handle(context, req)
    }

    /// Match the path part of the URL and extract parameters.
    pub fn match_path(&self, path: &str) -> Option<Dictionary<String>> {
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
pub struct RouteMatch<'a, Ctx, Req: RequestTrait = Request, Res: ResponseTrait = Response> {
    /// Reference to the matched route.
    pub route: &'a Route<Ctx, Req, Res>,
    /// Captured parameters from the URL pattern.
    pub params: Dictionary<String>,
}

/// Collection of [`Route`]s able to select one for a given request.
#[derive(Debug, Default)]
pub struct Router<Ctx, Req: RequestTrait = Request, Res: ResponseTrait = Response> {
    routes: Vec<Route<Ctx, Req, Res>>,
}

impl<Ctx, Req: RequestTrait, Res: ResponseTrait> Router<Ctx, Req, Res> {
    /// Create an empty [`Router`].
    pub fn new() -> Self {
        Self { routes: Vec::new() }
    }

    /// Register a new route.
    pub fn add_route(&mut self, route: Route<Ctx, Req, Res>) {
        self.routes.push(route);
    }

    /// Iterate over registered routes.
    pub fn iter(&self) -> impl Iterator<Item = &Route<Ctx, Req, Res>> {
        self.routes.iter()
    }

    /// Attempt to match `req` against registered routes.
    pub fn match_request(&self, req: &Req) -> Option<RouteMatch<Ctx, Req, Res>> {
        for route in &self.routes {
            if let Some(params) = route.matches(req) {
                return Some(RouteMatch { route, params });
            }
        }
        None
    }

    /// Handle `req` and return the generated [`Response`] if a route matches.
    pub fn handle_request(&mut self, context: &Ctx, req: &mut Req) -> Option<Res> {
        for route in &mut self.routes {
            if route.matches(req).is_some() {
                return Some(route.handle(context, req));
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http::{
        Authority, Headers, Message, MessageTrait, Path, Query, Request, RequestTrait, Response,
        ResponseFactory, ResponseTrait, Status, Uri, Version,
    };

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

    #[derive(Clone)]
    struct CustomRequest(Request);

    impl MessageTrait for CustomRequest {
        fn protocol_version(&self) -> Version {
            self.0.protocol_version()
        }

        fn with_protocol_version(self, version: Version) -> Self {
            Self(self.0.with_protocol_version(version))
        }

        fn headers(&self) -> &Headers {
            self.0.headers()
        }

        fn headers_mut(&mut self) -> &mut Headers {
            self.0.headers_mut()
        }

        fn has_header(&self, key: &str) -> bool {
            self.0.has_header(key)
        }

        fn with_headers(self, headers: Headers) -> Self {
            Self(self.0.with_headers(headers))
        }

        fn with_added_header(self, key: &str, value: &[String]) -> Self {
            Self(self.0.with_added_header(key, value))
        }

        fn without_header(self, key: &str) -> Self {
            Self(self.0.without_header(key))
        }

        fn body(&self) -> String {
            self.0.body()
        }

        fn with_body(self, body: &str) -> Self {
            Self(self.0.with_body(body))
        }
    }

    impl RequestTrait for CustomRequest {
        fn get_target(&self) -> String {
            self.0.get_target()
        }

        fn get_method(&self) -> Method {
            self.0.get_method()
        }

        fn with_method(self, method: Method) -> Self {
            Self(self.0.with_method(method))
        }

        fn get_uri(&self) -> Uri {
            self.0.get_uri()
        }

        fn with_uri(self, uri: Uri, preserve_host: bool) -> Self {
            Self(self.0.with_uri(uri, preserve_host))
        }
    }

    #[derive(Clone)]
    struct CustomResponse(Response);

    impl MessageTrait for CustomResponse {
        fn protocol_version(&self) -> Version {
            self.0.protocol_version()
        }

        fn with_protocol_version(self, version: Version) -> Self {
            Self(self.0.with_protocol_version(version))
        }

        fn headers(&self) -> &Headers {
            self.0.headers()
        }

        fn headers_mut(&mut self) -> &mut Headers {
            self.0.headers_mut()
        }

        fn has_header(&self, key: &str) -> bool {
            self.0.has_header(key)
        }

        fn with_headers(self, headers: Headers) -> Self {
            Self(self.0.with_headers(headers))
        }

        fn with_added_header(self, key: &str, value: &[String]) -> Self {
            Self(self.0.with_added_header(key, value))
        }

        fn without_header(self, key: &str) -> Self {
            Self(self.0.without_header(key))
        }

        fn body(&self) -> String {
            self.0.body()
        }

        fn with_body(self, body: &str) -> Self {
            Self(self.0.with_body(body))
        }
    }

    impl ResponseTrait for CustomResponse {
        fn status(&self) -> Status {
            self.0.status()
        }

        fn with_status(self, status: Status) -> Self {
            Self(self.0.with_status(status))
        }
    }

    #[test]
    fn test_basic_match() {
        let mut router = Router::new();
        let factory = ResponseFactory::version(Version::Http1_1);
        router.add_route(Route::new(
            "/foo/{id}",
            vec![Method::Get],
            Headers::new(),
            Box::new(move |_: &(), _req: &mut Request| factory.no_content(Headers::new())),
        ));

        let req = request(Method::Get, "/foo/42");
        let result = router.match_request(&req).unwrap();
        assert_eq!(result.params.get("id"), Some(&"42".to_string()));
    }

    #[test]
    fn test_method_and_header_mismatch() {
        let mut headers = Headers::new();
        headers.add("X-Test", "1");
        let factory = ResponseFactory::version(Version::Http1_1);
        let route = Route::new(
            "/a/{b}",
            vec![Method::Post],
            headers.clone(),
            Box::new(move |_: &(), _req: &mut Request| factory.no_content(Headers::new())),
        );
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

    #[test]
    fn test_controller_is_invoked() {
        let mut router = Router::new();
        let factory = ResponseFactory::version(Version::Http1_1);
        router.add_route(Route::new(
            "/ping",
            vec![Method::Get],
            Headers::new(),
            Box::new(move |_: &(), _req: &mut Request| {
                factory.with_status(Status::OK, Headers::new())
            }),
        ));

        let mut req = request(Method::Get, "/ping");
        let resp = router.handle_request(&(), &mut req).unwrap();
        assert_eq!(resp.status(), Status::OK);
    }

    #[test]
    fn test_generic_request_response() {
        let mut router: Router<(), CustomRequest, CustomResponse> = Router::new();
        router.add_route(Route::new(
            "/custom",
            vec![Method::Get],
            Headers::new(),
            Box::new(|_: &(), req: &mut CustomRequest| {
                let factory = ResponseFactory::version(req.protocol_version());
                CustomResponse(factory.with_status(Status::NoContent, Headers::new()))
            }),
        ));

        let mut req = CustomRequest(request(Method::Get, "/custom"));
        let resp = router.handle_request(&(), &mut req).unwrap();
        assert_eq!(resp.status(), Status::NoContent);
    }
}
