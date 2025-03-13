use crate::concepts::{Dictionary, Value};
use crate::http::factory::{HttpRequest, HttpResponse};
use crate::http::uri::Uri;
use crate::http::{Headers, Method, Request, Response, Router, Version};
use std::net::IpAddr;
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct ServerConfiguration {
    pub http_version: Version,
    pub default_request_headers: Headers,
    pub default_response_headers: Headers,
    pub server_address: IpAddr,
    pub server_name: String,
    pub server_port: u16,
    pub remote_address: IpAddr,
    pub remote_name: String,
    pub remote_port: u16,
    pub request_method: Method,
    pub request_uri: Uri,
    pub request_time: SystemTime,
    pub document_root: String,
}

impl ServerConfiguration {
    pub const DEFAULT_RESPONSE_HEADERS: &'static [(&'static str, &'static [&'static str])] = &[
        ("Server", &["Hermes v0.1.0"]),
    ];
    pub const DEFAULT_REQUEST_HEADERS: &'static [(&'static str, &'static [&'static str])] = &[
        ("User-Agent", &["Hermes v0.1.0"]),
    ];

    pub fn path_info(&self) -> String {
        self.request_uri.path.to_string()
    }
    pub fn query_string(&self) -> String {
        self.request_uri.query.to_string()
    }
}

#[derive(Debug, Clone)]
pub struct ServerRequest {
    http_request: Request,
    pub configuration: ServerConfiguration,
    /// The parsed query.
    pub query: Dictionary<Value>,
    /// The parsed request body if `self.method().has_body()` is true.
    ///
    /// TODO : depends the MIMETYPE of ContentType.
    pub parsed_body: Dictionary<Value>,
}

impl ServerRequest {
    pub fn request(&self) -> &Request {
        &self.http_request
    }
}

pub trait Handler {
    fn check(&self, _: &ServerRequest) -> bool {
        true
    }
    fn handle(&mut self, request: &ServerRequest) -> Response;
}
pub trait MiddlewareTrait: Handler {
    fn process(&mut self, other: &mut dyn Handler, request: &ServerRequest) -> Response {
        if self.check(request) {
            self.handle(request)
        } else {
            other.handle(request)
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Middleware {
    handle: fn(request: &ServerRequest) -> Response,
    accept: fn(request: &ServerRequest) -> bool,
}

impl Middleware {
    pub fn new(
        handle: fn(request: &ServerRequest) -> Response,
        accept: fn(request: &ServerRequest) -> bool,
    ) -> Self {
        Self { handle, accept }
    }
}

impl Handler for Middleware {
    fn check(&self, request: &ServerRequest) -> bool {
        (self.accept)(request)
    }

    fn handle(&mut self, request: &ServerRequest) -> Response {
        (self.handle)(request)
    }
}

impl MiddlewareTrait for Middleware {}

#[derive(Debug)]
pub struct Server {
    pub configuration: ServerConfiguration,
    pub request: HttpRequest,
    pub response: HttpResponse,
    router: Router,
    middleware: Vec<Middleware>,
}

impl Server {
    pub fn new(configuration: ServerConfiguration, router: Router) -> Self {
        let request_builder = HttpRequest::new(
            configuration.http_version,
            configuration.default_request_headers.clone(),
        );
        let response_builder = HttpResponse::new(
            configuration.http_version,
            configuration.default_response_headers.clone(),
        );

        Self {
            configuration,
            router,
            request: request_builder,
            response: response_builder,
            middleware: vec![],
        }
    }
    pub fn add_middleware(&mut self, middleware: Middleware) {
        self.middleware.push(middleware);
    }
    pub fn route_request(&mut self, request: &ServerRequest) -> Option<Response> {
        if let Some(route) = self.router.route(request) {
            todo!()
        }
        None
    }
}

impl Handler for Server {
    fn handle(&mut self, request: &ServerRequest) -> Response {
        for middleware in self.middleware.iter_mut().rev() {
            if middleware.check(request) {
                return middleware.handle(request);
            }
        }
        if let Some(response) = self.route_request(request) {
            return response;
        }
        self.response
            .not_implemented("None of the middleware accepted the request.")
    }
}
