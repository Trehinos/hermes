use crate::concepts::{Dictionary, Value};
use crate::http::{Headers, Method, Request, Uri, Version};
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
        ("Server", &[concat!("HermesServer/", env!("CARGO_PKG_VERSION"))]),
        ("Connection", &["keep-alive"]),
        ("Content-Type", &["charset=utf-8"]),
    ];
    pub const DEFAULT_REQUEST_HEADERS: &'static [(&'static str, &'static [&'static str])] = &[
        ("User-Agent", &[concat!("HermesAgent/", env!("CARGO_PKG_VERSION"))]),
        ("Content-Type", &["charset=utf-8"]),
    ];
    
    pub fn from(server_address: IpAddr, server_name: String, server_port: u16) -> Self {
        Self {
            http_version: Version::Http2_0,
            default_request_headers: Headers::from(Self::DEFAULT_REQUEST_HEADERS),
            default_response_headers: Headers::from(Self::DEFAULT_RESPONSE_HEADERS),
            server_address,
            server_name,
            server_port,
            remote_address: server_address,
            remote_name: "".to_string(),
            remote_port: server_port,
            request_method: Method::Get,
            request_uri: Default::default(),
            request_time: SystemTime::now(),
            document_root: "".to_string(),
        }
    }

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
