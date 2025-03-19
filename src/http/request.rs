use crate::concepts::{Dictionary, Parsable};
use crate::http::{Headers, Message, MessageTrait, Uri, Version};
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{alpha1, space0, space1};
use nom::IResult;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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
}

impl Parsable for Method {
    fn parse(input: &str) -> IResult<&str, Self>
    where
        Self: Sized,
    {
        let (input, method) = alpha1(input)?;
        Ok((
            input,
            match method.to_uppercase().as_str() {
                "GET" => Method::Get,
                "POST" => Method::Post,
                "PUT" => Method::Put,
                "DELETE" => Method::Delete,
                "PATCH" => Method::Patch,
                "OPTIONS" => Method::Options,
                "HEAD" => Method::Head,
                "CONNECT" => Method::Connect,
                "TRACE" => Method::Trace,
                &_ => panic!("Invalid HTTP method: {}", method),
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
            }
        )
    }
}

impl Method {
    pub fn has_body(&self) -> bool {
        matches!(self, Method::Post | Method::Put | Method::Connect)
    }
    pub fn response_has_body(&self) -> bool {
        matches!(
            self,
            Method::Get | Method::Post | Method::Connect | Method::Options
        )
    }
    pub fn is_safe(&self) -> bool {
        matches!(
            self,
            Method::Get | Method::Head | Method::Options | Method::Trace
        )
    }
    pub fn is_idempotent(&self) -> bool {
        matches!(self, Method::Post | Method::Connect)
    }
    pub fn is_cacheable(&self) -> bool {
        matches!(self, Method::Get | Method::Head | Method::Post)
    }
    pub fn is_html_compatible(&self) -> bool {
        matches!(self, Method::Get | Method::Post)
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
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
    pub fn add(&mut self, key: &str, value: &str) {
        self.data.insert(key.to_string(), value.to_string());
    }
    pub fn set(&mut self, key: &str, value: &str) {
        self.data.remove(key);
        self.data.insert(key.to_string(), value.to_string());
    }
    pub fn remove(&mut self, key: &str) {
        self.data.remove(key);
    }
    pub fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }
    pub fn get_line(&self, key: &str) -> Option<String> {
        self.get(key).map(|v| v.to_string())
    }
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
        self.method
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
    #[test]
    fn test_parse_query() {
        let query1 = "simple_query";
        let query2 = "variable=value";
        let query3 = "array[]=value&array[]=value2";
        let query4 = "map[a]=value1&map[b]=value2";
        let (_, query) = Query::parse(query1).unwrap();
        assert_eq!(query.data.len(), 1);
        assert_eq!(query.get("simple_query"), Some(&"".to_string()));

        let (_, query) = Query::parse(query2).unwrap();
        assert_eq!(query.data.len(), 1);
        assert_eq!(query.get("variable"), Some(&"value".to_string()));
        
        let (_, query) = Query::parse(query3).unwrap();
        assert_eq!(query.data.len(), 1);
        assert_eq!(query.get("array[]"), Some(&"value".to_string()));
        
        println!("{:?}", Query::parse(query4));
        let query1 = "simple_query#fragment";
        let query2 = "variable=value#fragment";
        let query3 = "array[]=value&array[]=value2#fragment";
        let query4 = "map[a]=value1&map[b]=value2#fragment";
        println!("{:?}", Query::parse(query1));
        println!("{:?}", Query::parse(query2));
        println!("{:?}", Query::parse(query3));
        println!("{:?}", Query::parse(query4));
        //todo!("Convert to tests")
    }

    #[test]
    fn test_parse_request() {
        let request_str = "POST /test.php HTTP/2.0\r\n\
            Host: localhost\r\n\
            \r\nvalue=v";
        let (_, request) = Request::parse(request_str).unwrap();
        println!("{:#?}", request);
    }
}