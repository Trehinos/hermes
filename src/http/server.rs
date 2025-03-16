use crate::{
    concepts::{Dictionary, Value},
    http::{
        Headers, MessageTrait, Method, Request, RequestFactory, Response, ResponseFactory, Uri,
        Version,
    },
};
use std::net::IpAddr;
use std::rc::Rc;
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
        ("Server", &["Hermes/0.1.0"]),
        ("Connection", &["keep-alive"]),
        ("Content-Type", &["charset=utf-8"]),
    ];
    pub const DEFAULT_REQUEST_HEADERS: &'static [(&'static str, &'static [&'static str])] = &[
        ("User-Agent", &["Hermes/0.1.0"]),
        ("Content-Type", &["charset=utf-8"]),
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
    pub fn from(http_request: Request, configuration: ServerConfiguration) -> Self {
        // TODO
        Self {
            http_request,
            configuration,
            query: Dictionary::new(),
            parsed_body: Dictionary::new(),
        }
    }

    pub fn method(&self) -> &Method {
        &self.http_request.method
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

#[derive(Debug, Clone)]
pub struct Middleware {
    server: Rc<Server>,
    handle: fn(server: &Server, request: &ServerRequest) -> Response,
    accept: fn(request: &ServerRequest) -> bool,
}

impl Middleware {
    pub fn new(
        server: Rc<Server>,
        handle: fn(server: &Server, request: &ServerRequest) -> Response,
        accept: fn(request: &ServerRequest) -> bool,
    ) -> Self {
        Self {
            server,
            handle,
            accept,
        }
    }
}

impl Handler for Middleware {
    fn check(&self, request: &ServerRequest) -> bool {
        (self.accept)(request)
    }

    fn handle(&mut self, request: &ServerRequest) -> Response {
        (self.handle)(&self.server, request)
    }
}

impl MiddlewareTrait for Middleware {}

#[derive(Debug)]
pub struct Server {
    pub configuration: ServerConfiguration,
    pub request: RequestFactory,
    pub response: ResponseFactory,
    pub response_headers: Headers,
    middleware: Vec<Middleware>,
}

impl Server {
    pub fn new(configuration: ServerConfiguration) -> Self {
        let request_builder = RequestFactory::new(
            configuration.http_version,
            configuration.default_request_headers.clone(),
        );
        let response_builder = ResponseFactory::new(
            configuration.http_version,
            configuration.default_response_headers.clone(),
        );

        Self {
            configuration,
            request: request_builder,
            response: response_builder,
            response_headers: Headers::new(),
            middleware: vec![],
        }
    }
    pub fn add_middleware(&mut self, middleware: Middleware) {
        self.middleware.push(middleware);
    }
}

impl Handler for Server {
    fn handle(&mut self, request: &ServerRequest) -> Response {
        let mut response = {
            for middleware in self.middleware.iter_mut().rev() {
                if middleware.check(request) {
                    return middleware.handle(request);
                }
            }
            self.response
                .not_implemented("None of the middleware accepted the request.")
        };
        let mut headers = self.configuration.default_response_headers.merge_with(
            &self
                .response_headers
                .merge_with(&response.headers().clone()),
        );
        headers.set(
            "Content-Length",
            &[&format!("{}", response.message.body.len())],
        );
        response = response.with_headers(headers);
        response
    }
}
