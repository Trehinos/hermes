//! Structures and utilities for HTTP requests.
use crate::concepts::{Dictionary, Parsable};
use crate::http::{Headers, Message, MessageTrait, Uri, Version};
use nom::bytes::complete::{tag, take_until, take_while1};
use nom::character::complete::{space0, space1};
use nom::IResult;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
/// Standard HTTP request methods.
pub enum Method {
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Options,
    Head,
    Connect,
    Trace,
    Custom(String),
}

impl Parsable for Method {
    fn parse(input: &str) -> IResult<&str, Self>
    where
        Self: Sized,
    {
        let (input, method) =
            take_while1(|c: char| c.is_ascii_alphanumeric() || c == '-')(input)?;
        let upper = method.to_uppercase();
        Ok((
            input,
            match upper.as_str() {
                "GET" => Method::Get,
                "POST" => Method::Post,
                "PUT" => Method::Put,
                "DELETE" => Method::Delete,
                "PATCH" => Method::Patch,
                "OPTIONS" => Method::Options,
                "HEAD" => Method::Head,
                "CONNECT" => Method::Connect,
                "TRACE" => Method::Trace,
                _ => Method::Custom(method.to_string()),
            },
        ))
    }
}

impl Display for Method {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}",
            match self {
                Method::Get => "GET",
                Method::Post => "POST",
                Method::Put => "PUT",
                Method::Delete => "DELETE",
                Method::Patch => "PATCH",
                Method::Options => "OPTIONS",
                Method::Head => "HEAD",
                Method::Connect => "CONNECT",
                Method::Trace => "TRACE",
                Method::Custom(s) => s,
            }
        )
    }
}

impl Method {

    /// Returns `true` if requests with this method usually contain a body.
    pub fn request_has_body(&self) -> bool {
        match self {
            Method::Post | Method::Delete | Method::Patch | Method::Put => true,
            Method::Custom(_) => false,
            _ => false,
        }
    }

    /// Returns `true` if responses to this method are expected to contain a body.
    pub fn response_has_body(&self) -> bool {
        match self {
            Method::Get
            | Method::Post
            | Method::Put
            | Method::Delete
            | Method::Patch
            | Method::Options
            | Method::Trace => true,
            Method::Custom(_) => false,
            _ => false,
        }
    }

    /// Checks whether the method is defined as safe by the HTTP specification.
    pub fn is_safe(&self) -> bool {
        match self {
            Method::Get | Method::Head | Method::Options | Method::Trace => true,
            _ => false,
        }
    }
    /// Indicates if repeated requests using this method are idempotent.
    pub fn is_idempotent(&self) -> bool {
        match self {
            Method::Put | Method::Delete => true,
            Method::Custom(_) => false,
            _ => self.is_safe(),
        }
    }
    /// Indicates if responses to this method can be cached.
    pub fn is_cacheable(&self) -> bool {
        match self {
            Method::Get | Method::Head | Method::Post | Method::Patch => true,
            _ => false,
        }
    }
    /// Returns `true` if browsers commonly use this method in HTML forms.
    pub fn is_html_compatible(&self) -> bool {
        match self {
            Method::Get | Method::Post => true,
            _ => false,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Query {
    data: Dictionary<String>,
}

impl Parsable for Query {
    fn parse(input: &str) -> IResult<&str, Self>
    where
        Self: Sized,
    {
        let mut map = Dictionary::new();
        let mut query = input;
        let mut input = input;
        if input.contains('#') {
            let (i, q) = take_until("#")(input)?;
            input = i;
            query = q;
        } else {
            input = "";
        }
        for pair in query.split('&') {
            let mut parts = pair.splitn(2, '=');
            let key = parts.next().unwrap_or_default();
            let value = parts.next().unwrap_or_default();
            map.insert(key.to_string(), value.to_string());
        }

        Ok((input, Self { data: map }))
    }
}

impl Query {
    /// Create an empty query string representation.
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
    /// Append a key/value pair to the query.
    pub fn add(&mut self, key: &str, value: &str) {
        self.data.insert(key.to_string(), value.to_string());
    }
    /// Replace any existing value for `key` with `value`.
    pub fn set(&mut self, key: &str, value: &str) {
        self.data.remove(key);
        self.data.insert(key.to_string(), value.to_string());
    }
    /// Remove the value associated with `key`.
    pub fn remove(&mut self, key: &str) {
        self.data.remove(key);
    }
    /// Get a reference to the stored value for `key` if any.
    pub fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }
    /// Return the value for `key` as an owned string.
    pub fn get_line(&self, key: &str) -> Option<String> {
        self.get(key).map(|v| v.to_string())
    }
    /// Returns `true` if the query contains `key`.
    pub fn has(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }
}

impl Display for Query {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut strings = Vec::new();
        for (key, value) in self.data.iter() {
            strings.push(format!("{}={}", key, value));
        }
        write!(f, "{}", strings.join("&"))
    }
}

/// Behaviour shared by HTTP request types.
pub trait RequestTrait: MessageTrait {
    fn get_target(&self) -> String;
    fn get_method(&self) -> Method;
    fn with_method(self, method: Method) -> Self
    where
        Self: Sized;
    fn get_uri(&self) -> Uri;
    fn with_uri(self, uri: Uri, preserve_host: bool) -> Self
    where
        Self: Sized;
}

#[derive(Debug, Clone)]
/// Representation of an HTTP request message.
pub struct Request {
    pub method: Method,
    pub target: Uri,
    pub message: Message,
}

impl Parsable for Request {
    fn parse(input: &str) -> IResult<&str, Self>
    where
        Self: Sized,
    {
        let (input, method) = Method::parse(input)?;
        let (input, _) = space1(input)?;
        let (input, uri) = take_until(" ")(input)?;
        let (_, target) = Uri::parse(uri)?;
        let (input, _) = space1(input)?;
        let (input, version) = Version::parse(input)?;
        let (input, _) = space0(input)?;
        let (input, _) = tag("\r\n")(input)?;
        let (_, mut message) = Message::parse(input)?;
        message = message.with_protocol_version(version);
        Ok((
            "",
            Self {
                method,
                target,
                message,
            },
        ))
    }
}

impl MessageTrait for Request {
    fn protocol_version(&self) -> Version {
        self.message.protocol_version()
    }

    fn with_protocol_version(self, version: Version) -> Self
    where
        Self: Sized,
    {
        Self {
            method: self.method,
            target: self.target,
            message: self.message.with_protocol_version(version),
        }
    }

    fn headers(&self) -> &Headers {
        self.message.headers()
    }

    fn headers_mut(&mut self) -> &mut Headers {
        self.message.headers_mut()
    }

    fn has_header(&self, key: &str) -> bool {
        self.message.has_header(key)
    }

    fn with_headers(self, headers: Headers) -> Self
    where
        Self: Sized,
    {
        Self {
            method: self.method,
            target: self.target,
            message: self.message.with_headers(headers),
        }
    }

    fn with_added_header(self, key: &str, value: &[String]) -> Self
    where
        Self: Sized,
    {
        Self {
            method: self.method,
            target: self.target,
            message: self.message.with_added_header(key, value),
        }
    }

    fn without_header(self, key: &str) -> Self
    where
        Self: Sized,
    {
        Self {
            method: self.method,
            target: self.target,
            message: self.message.without_header(key),
        }
    }

    fn body(&self) -> String {
        self.message.body()
    }

    fn with_body(self, body: &str) -> Self
    where
        Self: Sized,
    {
        Self {
            method: self.method,
            target: self.target,
            message: self.message.with_body(body),
        }
    }
}

impl RequestTrait for Request {
    fn get_target(&self) -> String {
        self.target.to_string()
    }

    fn get_method(&self) -> Method {
        self.method.clone()
    }

    fn with_method(self, method: Method) -> Self {
        Self {
            method,
            target: self.target,
            message: self.message,
        }
    }

    fn get_uri(&self) -> Uri {
        self.target.clone()
    }

    fn with_uri(self, uri: Uri, preserve_host: bool) -> Self {
        let mut headers = self.message.headers().clone();
        if preserve_host {
            headers.set("Host", &[&uri.authority.host]);
        }
        Self {
            method: self.method,
            target: uri,
            message: self.message.with_headers(headers),
        }
    }
}

impl Display for Request {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}\r\n",
            self.method,
            self.target,
            self.protocol_version()
        )?;
        write!(f, "{}", self.message.raw())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::http::{Authority, Path};
    #[test]
    fn test_parse_query() {
        let query1 = "simple_query";
        let query2 = "variable=value";
        let query3 = "array[]=value&array[]=value2";
        
        let (_, query) = Query::parse(query1).unwrap();
        assert_eq!(query.data.len(), 1);
        assert_eq!(query.get("simple_query"), Some(&"".to_string()));

        let (_, query) = Query::parse(query2).unwrap();
        assert_eq!(query.data.len(), 1);
        assert_eq!(query.get("variable"), Some(&"value".to_string()));
        
        let (_, query) = Query::parse(query3).unwrap();
        assert_eq!(query.data.len(), 1);
        assert_eq!(query.get("array[]"), Some(&"value2".to_string()));
        
        let query1 = "simple_query#fragment";
        let query2 = "variable=value#fragment";
        let query3 = "array[]=value&array[]=value2#fragment";
        let query4 = "map[a]=value1&map[b]=value2#fragment";
        
        let (_, query) = Query::parse(query1).unwrap();
        assert_eq!(query.data.len(), 1);
        assert_eq!(query.get("simple_query"), Some(&"".to_string()));
        let (_, query) = Query::parse(query2).unwrap();
        assert_eq!(query.data.len(), 1);
        assert_eq!(query.get("variable"), Some(&"value".to_string()));
        let (_, query) = Query::parse(query3).unwrap();
        assert_eq!(query.data.len(), 1);
        assert_eq!(query.get("array[]"), Some(&"value2".to_string()));
        let (_, query) = Query::parse(query4).unwrap();
        assert_eq!(query.data.len(), 2);
        assert_eq!(query.get("map[a]"), Some(&"value1".to_string()));
        assert_eq!(query.get("map[b]"), Some(&"value2".to_string()));
    }

    #[test]
    fn test_parse_request() {
        let request_str = "POST /test.php HTTP/2.0\r\n\
            Host: localhost\r\n\
            \r\nvalue=v";
        let (_, request) = Request::parse(request_str).unwrap();
        println!("{:#?}", request);
    }
    #[test]
    fn test_method_properties() {
        for (s, m) in [("GET", Method::Get), ("POST", Method::Post)] {
            assert_eq!(Method::parse(s).unwrap().1, m);
            assert!(m.to_string().contains(s));
        }
        assert!(Method::Post.request_has_body());
        assert!(Method::Get.response_has_body());
        assert!(Method::Get.is_safe());
        assert!(Method::Put.is_idempotent());
        assert!(Method::Get.is_cacheable());
        assert!(Method::Post.is_html_compatible());
    }

    #[test]
    fn test_custom_method_parse() {
        assert_eq!(
            Method::parse("FOO").unwrap().1,
            Method::Custom("FOO".to_string())
        );
        assert_eq!(
            Method::parse("CUSTOM-METH").unwrap().1,
            Method::Custom("CUSTOM-METH".to_string())
        );
    }

    #[test]
    fn test_query_methods() {
        let mut q = Query::new();
        q.add("a", "1");
        assert_eq!(q.get_line("a"), Some("1".to_string()));
        assert!(q.has("a"));
        q.set("a", "2");
        assert_eq!(q.get("a"), Some(&"2".to_string()));
        q.remove("a");
        assert!(!q.has("a"));
        q.add("x", "1");
        q.add("y", "2");
        assert!(q.to_string().contains("x=1"));
    }

    #[test]
    fn test_request_methods() {
        let uri = Uri::new("http".into(), Authority::new("host".into(), None, None, None), Path::new("/".into(), None), Query::new(), None);
        let req = Request {
            method: Method::Get,
            target: uri.clone(),
            message: Message::v1_1(Headers::from(&[("Host", &["host"])]), String::new()),
        };
        assert!(req.get_target().starts_with("http://host"));
        assert_eq!(req.get_method(), Method::Get);
        let req2 = req.clone().with_method(Method::Post);
        assert_eq!(req2.get_method(), Method::Post);
        let uri2 = Uri::new("http".into(), Authority::new("example".into(), None, None, None), Path::new("/".into(), None), Query::new(), None);
        let req3 = req2.with_uri(uri2, true);
        assert_eq!(req3.get_header_line("Host"), Some("example".to_string()));
        assert!(req3.get_target().contains("http://example"));
    }
}
