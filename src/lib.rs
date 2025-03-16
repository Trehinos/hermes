pub mod concepts;
pub mod http;
pub mod security;

#[cfg(test)]
mod test_tdd {
    use crate::concepts::Parsable;
    use crate::http::{MessageTrait, Request};

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
        
        
    }
}
