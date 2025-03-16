pub mod concepts;
pub mod http;
pub mod security;

#[cfg(test)]
mod test_tdd {
    use crate::concepts::Parsable;
    use crate::http::{
        Handler, Headers, MessageTrait, Request, Server, ServerConfiguration, ServerRequest,
        Version,
    };

    fn request() -> Request {
        let request_text = "GET http://example.server.test/index?name=world#second HTTP/1.1\r\n\
            Host: example.server.test\r\n\
            Accept: *\r\n\
            Connection: keep-alive\r\n\r\n";
        let (_, request) = Request::parse(request_text).unwrap();
        assert_eq!(request.target.authority.host, "example.server.test");
        assert_eq!(request.get_header_line("Connection").unwrap(), "keep-alive");

        request
    }

    #[test]
    fn test_cycle() {
        let request = request();
        println!("Handle a GET request to {}...", request.target);

        let mut server = Server::new(ServerConfiguration {
            http_version: Version::Http1_1,
            default_request_headers: Headers::new(),
            default_response_headers: Headers::new(),
            server_address: "127.0.0.1".parse().unwrap(),
            server_name: "server.test".to_string(),
            server_port: 80,
            remote_address: "127.0.0.1".parse().unwrap(),
            remote_name: "client.test".to_string(),
            remote_port: 80,
            request_method: request.method,
            request_uri: request.target.clone(),
            request_time: std::time::SystemTime::now(),
            document_root: "index".to_string(),
        });
        let server_request = ServerRequest::from(request.clone(), server.configuration.clone());
        let response = server.handle(&server_request);
        println!("Response: {}", response);
    }
}
