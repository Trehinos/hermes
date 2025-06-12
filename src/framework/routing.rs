use crate::concepts::Parsable;
use crate::http::{
    Method, Request, Response, Version,
};
use regex::Regex;
use std::io::Read;
use std::net::{IpAddr, TcpListener};
use std::time::SystemTime;
use crate::framework::handler::Handler;
use crate::framework::server::{ServerConfiguration, ServerRequest};

pub struct Route {
    pub name: String,
    methods: Vec<Method>,
    pattern: Regex,
    target: Box<dyn Handler>,
}

impl Route {
    pub fn new<T: Handler + 'static>(
        name: &str,
        methods: Vec<Method>,
        pattern: String,
        target: T,
    ) -> Self {
        Self {
            name: name.to_string(),
            methods,
            pattern: Regex::new(&pattern).unwrap(),
            target: Box::new(target),
        }
    }

    pub fn check_target(&self, request: &ServerRequest) -> bool {
        self.target.check(request)
    }

    pub fn matches(&self, request: &ServerRequest) -> bool {
        self.methods.contains(request.method())
            && self
                .pattern
                .is_match(&request.request().target.path.to_string())
            && self.check_target(request)
    }
}

impl Handler for Route {
    fn check(&self, request: &ServerRequest) -> bool {
        self.matches(request)
    }
    fn handle(&mut self, request: &ServerRequest) -> Response {
        self.target.handle(request)
    }
}

#[cfg(test)]
mod test {
    use crate::concepts::Parsable;
    use crate::framework::routing::Route;
    use crate::http::{
        Headers, Method, RequestFactory, ResponseFactory, Uri, Version,
    };
    use std::net::{IpAddr, Ipv4Addr};
    use std::time::SystemTime;
    use crate::framework::handler::Controller;
    use crate::framework::server::{ServerConfiguration, ServerRequest};

    #[test]
    fn test_route() {
        let response = ResponseFactory::new(Version::Http2_0, Headers::new());
        let request = RequestFactory::new(Version::Http2_0, Headers::new());
        let route = Route::new(
            "test",
            vec![Method::Get],
            r"^/test/(\d+)$".to_string(),
            Controller::new(
                move |_| response.ok(Headers::new(), "Hello world!".to_string()),
                |_| true,
            ),
        );

        let (_, test_uri) = Uri::parse("http://example.com/test/25").unwrap();
        let server_request = ServerRequest::from(
            request.get(test_uri, Headers::new()),
            ServerConfiguration {
                http_version: Version::Http2_0,
                default_request_headers: Default::default(),
                default_response_headers: Default::default(),
                server_address: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
                server_name: "example.com".to_string(),
                server_port: 80,
                remote_address: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
                remote_name: "".to_string(),
                remote_port: 0,
                request_method: Method::Get,
                request_uri: Default::default(),
                request_time: SystemTime::now(),
                document_root: "".to_string(),
            },
        );
        assert!(route.matches(&server_request));
    }
}

pub struct Host {
    host_name: String,
    server_address: IpAddr,
    listen_address: IpAddr,
    listen_port: u16,
    listener: TcpListener,
    routes: Vec<Route>,
}

impl Host {
    pub fn new(
        host_name: &str,
        routes: Vec<Route>,
        server_address: IpAddr,
        listen_address: IpAddr,
        listen_port: u16,
    ) -> Self {
        Self {
            host_name: host_name.to_string(),
            server_address,
            listen_address,
            listen_port,
            listener: TcpListener::bind(format!("{}:{}", listen_address, listen_port)).unwrap(),
            routes,
        }
    }

    pub fn create_configuration(
        &self
    ) -> ServerConfiguration {
        ServerConfiguration {
            http_version: Version::Http2_0,
            default_request_headers: Default::default(),
            default_response_headers: Default::default(),
            server_address: self.server_address,
            server_name: self.host_name.clone(),
            server_port: 0,
            remote_address: self.listen_address,
            remote_name: "".to_string(),
            remote_port: self.listen_port,
            request_method: Method::Get,
            request_uri: Default::default(),
            request_time: SystemTime::now(),
            document_root: "".to_string(),
        }
    }

    pub fn accept(&mut self) {
        for stream in self.listener.incoming() {
            if let Ok(s) = stream {
                let addr = s.peer_addr().unwrap();
                let local = s.local_addr().unwrap();
                let ip = addr.ip();
                let port = addr.port();
                let request_str =
                    String::from_utf8(s.bytes().map(|b| b.unwrap()).collect::<Vec<u8>>()).unwrap();
                let (_, _request) = Request::parse(&request_str).unwrap();
                println!(
                    "New connection from {}:{} on {}:{}\n{}",
                    ip,
                    port,
                    local.ip(),
                    local.port(),
                    request_str
                );
            } else {
                panic!("TcpStream error");
            }
        }
    }
}

impl Handler for Host {
    fn check(&self, request: &ServerRequest) -> bool {
        if request.request().target.authority.host != self.host_name {
            return false;
        }
        self.routes.iter().any(|route| route.matches(request))
    }
    fn handle(&mut self, request: &ServerRequest) -> Response {
        self.routes
            .iter_mut()
            .find(|route| route.matches(request))
            .unwrap()
            .handle(request)
    }
}
