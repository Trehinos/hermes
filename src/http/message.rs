use crate::concepts::Parsable;
use nom::bytes::complete::{tag, take_until, take_until1};
use nom::character::complete::{digit1, multispace0};
use nom::sequence::terminated;
use nom::{IResult, Parser};
use std::collections::HashMap;
use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum Version {
    Http0_9,
    Http1_0,
    Http1_1,
    Http2_0,
    Http3_0,
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "HTTP/{}",
            match self {
                Version::Http0_9 => "0.9",
                Version::Http1_0 => "1.0",
                Version::Http1_1 => "1.1",
                Version::Http2_0 => "2.0",
                Version::Http3_0 => "3.0",
            }
        )
    }
}

impl Parsable for Version {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, _) = tag("HTTP/")(input)?;
        let (input, major) = digit1(input)?;
        let (input, _) = tag(".")(input)?;
        let (input, minor) = digit1(input)?;

        let version = match (major, minor) {
            ("0", "9") => Version::Http0_9,
            ("1", "0") => Version::Http1_0,
            ("1", "1") => Version::Http1_1,
            ("2", "0") => Version::Http2_0,
            ("3", "0") => Version::Http3_0,
            _ => panic!("Invalid HTTP version: {}.{}", major, minor),
        };
        Ok((input, version))
    }
}

#[derive(Debug, Default, Clone)]
pub struct Headers {
    data: HashMap<String, Vec<String>>,
}

impl Headers {
    pub fn parse_header(input: &str) -> IResult<&str, (String, Vec<String>)> {
        let (header_value, header_name) = terminated(take_until(":"), tag(":")).parse(input)?;
        Ok((
            "",
            (
                header_name.to_string(),
                header_value
                    .trim()
                    .split(';')
                    .map(|s| s.trim().to_string())
                    .collect(),
            ),
        ))
    }
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
    pub fn iter(&self) -> impl Iterator<Item = (&String, &Vec<String>)> {
        self.data.iter()
    }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&String, &mut Vec<String>)> {
        self.data.iter_mut()
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    pub fn add(&mut self, key: &str, value: &str) {
        if let Some(values) = self.data.get_mut(key) {
            values.push(value.to_string());
            return;
        }
        self.data.insert(key.to_string(), vec![value.to_string()]);
    }
    pub fn insert(&mut self, key: &str, values: &[String]) {
        if let Some(v) = self.data.get_mut(key) {
            v.extend_from_slice(values);
            return;
        }
        self.data.insert(key.to_string(), values.to_vec());
    }
    pub fn set(&mut self, key: &str, values: &[String]) {
        self.data.remove(key);
        self.data.insert(key.to_string(), values.to_vec());
    }
    pub fn get(&self, key: &str) -> Option<&Vec<String>> {
        self.data.get(key)
    }
    pub fn get_line(&self, key: &str) -> Option<String> {
        self.get(key).map(|v| v.join(","))
    }
}

impl Display for Headers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self
            .data
            .keys()
            .map(|key| format!("{}: {}", key, self.get_line(key).unwrap()))
            .collect::<Vec<_>>()
            .join("\r\n");

        write!(f, "{}", s)
    }
}

impl Parsable for Headers {
    fn parse(input: &str) -> IResult<&str, Self> {
        let mut header_lines = input;
        let mut input = input;
        if input.contains("\r\n\r\n") {
            let (i, h) = take_until1("\r\n\r\n")(input)?;
            let (i, _) = tag("\r\n\r\n")(i)?;
            header_lines = h;
            input = i;
        } else {
            input = "";
        }
        let mut headers = HashMap::new();
        for line in header_lines.split("\r\n") {
            if line.is_empty() {
                continue;
            }
            let (_, (name, value)) = Self::parse_header(line)?;
            headers.insert(name, value);
        }
        Ok((input, Self { data: headers }))
    }
}

pub trait MessageTrait {
    fn protocol_version(&self) -> Version;
    fn with_protocol_version(self, version: Version) -> Self
    where
        Self: Sized;
    fn headers(&self) -> Headers;
    fn get_header_line(&self, key: &str) -> Option<String> {
        self.headers().get_line(key)
    }
    fn has_header(&self, key: &str) -> bool;
    fn with_headers(self, headers: Headers) -> Self
    where
        Self: Sized;
    fn with_added_header(self, key: &str, value: &[String]) -> Self
    where
        Self: Sized;
    fn without_header(self, key: &str) -> Self
    where
        Self: Sized;
    fn body(&self) -> String;
    fn with_body(self, body: &str) -> Self
    where
        Self: Sized;
}

#[derive(Debug, Clone)]
pub struct Message {
    pub version: Version,
    pub headers: Headers,
    pub body: String,
}

impl Message {
    pub fn v1_1(headers: Headers, body: String) -> Self {
        Self {
            version: Version::Http1_1,
            headers,
            body,
        }
    }
    pub fn v2_0(headers: Headers, body: String) -> Self {
        Self {
            version: Version::Http2_0,
            headers,
            body,
        }
    }
    pub fn v3_0(headers: Headers, body: String) -> Self {
        Self {
            version: Version::Http3_0,
            headers,
            body,
        }
    }

    pub fn parse_version(input: &str) -> IResult<&str, Option<Version>> {
        let mut input = input;
        if input.starts_with("HTTP") {
            let (i, version) = Version::parse(input)?;
            let (i, _) = multispace0(i)?;
            input = i;
            Ok((input, Some(version)))
        } else {
            Ok((input, None))
        }
    }

    pub fn raw(&self) -> String {
        format!("{}\r\n\r\n{}", self.headers, self.body)
    }
}

impl Parsable for Message {
    fn parse(input: &str) -> IResult<&str, Self>
    where
        Self: Sized,
    {
        let (input, version) = Self::parse_version(input)?;
        let (body, headers) = Headers::parse(input)?;

        Ok((
            "",
            Self {
                version: version.unwrap_or(Version::Http1_1),
                headers,
                body: body.to_string(),
            },
        ))
    }
}

impl MessageTrait for Message {
    fn protocol_version(&self) -> Version {
        self.version
    }

    fn with_protocol_version(self, version: Version) -> Self
    where
        Self: Sized,
    {
        Self {
            version,
            headers: self.headers,
            body: self.body,
        }
    }

    fn headers(&self) -> Headers {
        self.headers.clone()
    }
    fn has_header(&self, key: &str) -> bool {
        self.headers.data.contains_key(key)
    }

    fn with_headers(self, headers: Headers) -> Self
    where
        Self: Sized,
    {
        Self {
            version: self.version,
            headers,
            body: self.body,
        }
    }

    fn with_added_header(self, key: &str, value: &[String]) -> Self
    where
        Self: Sized,
    {
        let mut headers = self.headers;
        headers.insert(key, value);
        Self {
            version: self.version,
            headers,
            body: self.body,
        }
    }

    fn without_header(self, key: &str) -> Self
    where
        Self: Sized,
    {
        let mut headers = self.headers;
        headers.data.remove(key);
        Self {
            version: self.version,
            headers,
            body: self.body,
        }
    }

    fn body(&self) -> String {
        self.body.clone()
    }

    fn with_body(self, body: &str) -> Self
    where
        Self: Sized,
    {
        Self {
            version: self.version,
            headers: self.headers,
            body: body.to_string(),
        }
    }
}

impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\r\n{}", self.version, self.raw())
    }
}

#[cfg(test)]
mod parse_tests {
    use super::*;

    #[test]
    fn test_parse_version() {
        const VERSIONS: &[(&str, Version)] = &[
            ("HTTP/0.9", Version::Http0_9),
            ("HTTP/1.0", Version::Http1_0),
            ("HTTP/1.1", Version::Http1_1),
            ("HTTP/2.0", Version::Http2_0),
            ("HTTP/3.0", Version::Http3_0),
        ];
        for (str_v, version) in VERSIONS {
            let (input, parsed_version) = Version::parse(str_v).unwrap();
            assert_eq!(input, "");
            assert_eq!(*version, parsed_version);
        }
    }

    #[test]
    fn test_parse_header() {
        let input = "Content-Type: text/html";
        let (input, (key, value)) = Headers::parse_header(input).unwrap();
        assert_eq!(input, "");
        assert_eq!("Content-Type", key);
        assert_eq!("text/html", value[0]);
    }

    #[test]
    fn test_parse_headers() {
        let input = "Content-Type: text/html\r\nContent-Length: 123\r\n";
        let (input, headers) = Headers::parse(input).unwrap();
        assert_eq!(input, "");
        assert_eq!("text/html", headers.get_line("Content-Type").unwrap());
        assert_eq!("123", headers.get_line("Content-Length").unwrap());
    }

    #[test]
    fn test_parse_message() {
        let input = "HTTP/1.1\r\n\
            Content-Type: text/html\r\n\
            Context-Disposition: attachment; filename=\"file.html\"\r\n\
            Content-Length: 123\r\n\
            \r\n\
            <html><body>Hello world!</body></html>";
        let (input, message) = Message::parse(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(
            "text/html",
            message.get_header_line("Content-Type").unwrap()
        );
        assert_eq!("123", message.get_header_line("Content-Length").unwrap());
        assert_eq!("<html><body>Hello world!</body></html>", message.body());
    }
}
