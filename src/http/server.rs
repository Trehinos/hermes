use crate::concepts::Dictionary;
use crate::http::uri::Uri;
use crate::http::{Method, Request, Response, Router, Version};
use std::net::IpAddr;
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub enum Value {
    Null,
    Bool(bool),
    Int(i64),
    Number(f64),
    String(String),
    Array(Vec<Value>),
    Dictionary(Dictionary<Value>),
}

#[derive(Debug, Clone)]
pub struct ServerConfiguration {
    pub http_version: Version,
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
    /// The parsed request body if `self.method().has_body()` is true.
    ///
    /// TODO : depends the MIMETYPE of ContentType.
    pub request: Dictionary<Value>,
    /// The parsed query.
    pub query: Dictionary<Value>,
}

impl ServerRequest {
    pub fn http(&self) -> &Request {
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
    pub router: Router,
    pub middleware: Vec<Middleware>,
}
