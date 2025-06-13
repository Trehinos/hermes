//! Utilities for HTTP messages and headers.
use crate::concepts::Parsable;
use crate::http::ParseError;
use nom::bytes::complete::{tag, take_until1};
use nom::character::complete::digit1;
use nom::IResult;
use std::collections::HashMap;
use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
/// Supported versions of the HTTP protocol.
///
/// The [`Version::parse`] method can be used to read the protocol version
/// from the start of a request or response line.
///
/// # Examples
///
/// ```
/// use hermes::http::Version;
/// use hermes::concepts::Parsable;
///
/// let (rest, ver) = Version::parse("HTTP/1.1 rest").unwrap();
/// assert_eq!(ver, Version::Http1_1);
/// assert_eq!(rest, " rest");
/// assert_eq!(ver.to_string(), "HTTP/1.1");
/// ```
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
            _ => {
                return Err(nom::Err::Failure(nom::error::Error::new(
                    input,
                    nom::error::ErrorKind::Fail,
                )));
            }
        };
        Ok((input, version))
    }
}

#[derive(Debug, Default, Clone)]
/// Collection of HTTP header fields.
///
/// The structure stores a mapping between header names and the associated
/// values.  Individual helper methods are provided to insert and retrieve
/// headers in a convenient way.
///
/// # Examples
///
/// ```
/// use hermes::http::Headers;
///
/// let mut headers = Headers::new();
/// headers.add("Content-Type", "text/plain");
/// headers.add("Content-Type", "charset=utf8");
/// assert_eq!(headers.get_line("Content-Type"), Some("text/plain,charset=utf8".to_string()));
/// ```
pub struct Headers {
    data: HashMap<String, Vec<String>>,
}

impl Headers {
    /// Parse a single header line into a key and list of values.
    pub fn parse_header(input: &str) -> Result<(&str, (String, Vec<String>)), ParseError> {
        if let Some((name, rest)) = input.split_once(':') {
            let values = rest
                .trim()
                .split(',')
                .map(|s| s.trim().to_string())
                .collect();
            Ok(("", (name.trim().to_string(), values)))
        } else {
            Err(ParseError::InvalidHeaderFormat(input.to_string()))
        }
    }
    /// Create an empty `Headers` map.
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
    /// Build a `Headers` collection from a slice of key/value pairs.
    pub fn from(headers: &[(&str, &[&str])]) -> Self {
        Self {
            data: headers
                .iter()
                .map(|(s, v)| (s.to_string(), v.iter().map(|s| s.to_string()).collect()))
                .collect(),
        }
    }
    /// Return a new `Headers` containing values from `self` and `other`.
    pub fn merge_with(&self, other: &Self) -> Self {
        let mut headers = self.clone();
        for (key, values) in other.data.iter() {
            headers.insert(key, values);
        }
        headers
    }
    /// Iterate over header entries as key/value pairs.
    pub fn iter(&self) -> impl Iterator<Item = (&String, &Vec<String>)> {
        self.data.iter()
    }
    /// Iterate over mutable header values.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&String, &mut Vec<String>)> {
        self.data.iter_mut()
    }
    /// Return the number of stored headers.
    pub fn len(&self) -> usize {
        self.data.len()
    }
    /// Check whether no headers are stored.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    /// Add a new header value without removing existing ones.
    pub fn add(&mut self, key: &str, value: &str) {
        if let Some(values) = self.data.get_mut(key) {
            values.push(value.to_string());
            return;
        }
        self.data.insert(key.to_string(), vec![value.to_string()]);
    }
    /// Insert several values for the specified header key.
    pub fn insert(&mut self, key: &str, values: &[String]) {
        if let Some(v) = self.data.get_mut(key) {
            v.extend_from_slice(values);
            return;
        }
        self.data.insert(key.to_string(), values.to_vec());
    }
    /// Replace the header with a new set of values.
    pub fn set(&mut self, key: &str, values: &[&str]) {
        self.data.remove(key);
        self.data.insert(
            key.to_string(),
            values.iter().map(|s| s.to_string()).collect(),
        );
    }
    /// Retrieve the stored values for `key` if present.
    pub fn get(&self, key: &str) -> Option<&Vec<String>> {
        self.data.get(key)
    }
    /// Return the values joined by commas as one line.
    pub fn get_line(&self, key: &str) -> Option<String> {
        self.get(key).map(|v| v.join(","))
    }
    /// Return the values joined with new lines for display.
    pub fn get_value(&self, key: &str) -> Option<String> {
        self.get(key).map(|v| v.join(",\n\t"))
    }
}

impl Display for Headers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self
            .data
            .keys()
            .map(|key| format!("{}: {}", key, self.get_value(key).unwrap_or("".to_string())))
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
            let (_, (name, value)) = Self::parse_header(line).map_err(|_| {
                nom::Err::Error(nom::error::Error::new(line, nom::error::ErrorKind::Fail))
            })?;
            headers.insert(name, value);
        }
        Ok((input, Self { data: headers }))
    }
}

/// Common interface implemented by HTTP request and response types.
pub trait MessageTrait {
    fn protocol_version(&self) -> Version;
    fn with_protocol_version(self, version: Version) -> Self
    where
        Self: Sized;
    fn headers(&self) -> &Headers;
    fn headers_mut(&mut self) -> &mut Headers;
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
/// Generic HTTP message used as the building block of requests and responses.
///
/// The struct simply stores the protocol [`Version`], a set of [`Headers`] and
/// the message body.  Convenience constructors like [`Message::v1_1`] help
/// create instances with a fixed version.
///
/// # Examples
/// ```
/// use hermes::http::{Headers, Message, Version};
/// use hermes::http::MessageTrait;
///
/// let msg = Message::v1_1(Headers::new(), "body".into());
/// assert_eq!(msg.protocol_version(), Version::Http1_1);
/// assert_eq!(msg.body(), "body");
/// ```
pub struct Message {
    pub version: Version,
    pub headers: Headers,
    pub body: String,
}

impl Message {
    /// Build a HTTP/1.1 message from the given headers and body.
    pub fn v1_1(headers: Headers, body: String) -> Self {
        Self {
            version: Version::Http1_1,
            headers,
            body,
        }
    }
    /// Build a HTTP/2.0 message from the given headers and body.
    pub fn v2_0(headers: Headers, body: String) -> Self {
        Self {
            version: Version::Http2_0,
            headers,
            body,
        }
    }
    /// Build a HTTP/3.0 message from the given headers and body.
    pub fn v3_0(headers: Headers, body: String) -> Self {
        Self {
            version: Version::Http3_0,
            headers,
            body,
        }
    }

    /// Parse a protocol version at the start of `input` if present.
    pub fn parse_version(input: &str) -> Result<(&str, Option<Version>), ParseError> {
        let mut input = input;
        if input.starts_with("HTTP") {
            let (i, version) = Version::parse(input)
                .map_err(|_| ParseError::InvalidHttpVersion(input.to_string()))?;
            let i = i.trim_start();
            input = i;
            Ok((input, Some(version)))
        } else {
            Ok((input, None))
        }
    }

    /// Render the message body preceded by headers as raw text.
    pub fn raw(&self) -> String {
        format!("{}\r\n\r\n{}", self.headers, self.body)
    }
}

impl Parsable for Message {
    fn parse(input: &str) -> IResult<&str, Self>
    where
        Self: Sized,
    {
        let (input, version) = Self::parse_version(input).map_err(|_| {
            nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Fail))
        })?;
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

    fn headers(&self) -> &Headers {
        &self.headers
    }
    fn headers_mut(&mut self) -> &mut Headers {
        &mut self.headers
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
mod tests {
    use super::*;

    fn sample_headers() -> Headers {
        Headers::from(&[("A", &["1"]), ("B", &["2"])])
    }

    #[test]
    fn test_version_display_and_parse_version() {
        for (s, v) in [
            ("HTTP/1.1", Version::Http1_1),
            ("HTTP/2.0", Version::Http2_0),
        ] {
            let (rest, parsed) = Version::parse(s).unwrap();
            assert_eq!(rest, "");
            assert_eq!(parsed, v);
            assert_eq!(parsed.to_string(), s);
        }
        let (rest, none) = Message::parse_version("NoHTTP").unwrap();
        assert_eq!(rest, "NoHTTP");
        assert!(none.is_none());
    }

    #[test]
    fn test_headers_basic() {
        let mut h = Headers::new();
        assert!(h.is_empty());
        h.add("A", "1");
        h.add("A", "2");
        assert_eq!(h.len(), 1);
        assert_eq!(h.get_line("A"), Some("1,2".to_string()));
        h.insert("A", &["3".to_string()]);
        assert_eq!(h.get_line("A"), Some("1,2,3".to_string()));
        h.set("A", &["x"]);
        assert_eq!(h.get_line("A"), Some("x".to_string()));
    }

    #[test]
    fn test_headers_merge_and_display_iter() {
        let h1 = sample_headers();
        let h2 = Headers::from(&[("B", &["3"])]);
        let merged = h1.merge_with(&h2);
        assert_eq!(merged.get_line("A"), Some("1".to_string()));
        assert_eq!(merged.get_line("B"), Some("2,3".to_string()));
        let out = merged.to_string();
        assert!(out.contains("A: 1"));
        assert!(out.contains("B: 2,\n\t3"));

        let mut count = 0;
        for (_, vals) in merged.iter() {
            count += vals.len();
        }
        for (_, vals) in merged.clone().iter_mut() {
            let _ = vals.len();
        }
        assert_eq!(count, 3);
    }

    #[test]
    fn test_message_methods() {
        let msg = Message::v1_1(sample_headers(), "body".into());
        assert!(msg.raw().contains("A: 1"));
        assert!(msg.raw().contains("B: 2"));
        assert!(msg.to_string().starts_with("HTTP/1.1"));
    }

    #[test]
    fn test_parse_header_error() {
        let err = Headers::parse_header("BadHeader").unwrap_err();
        assert_eq!(
            err,
            ParseError::InvalidHeaderFormat("BadHeader".to_string())
        );
    }

    #[test]
    fn test_parse_version_error() {
        let err = Message::parse_version("HTTP/9.9").unwrap_err();
        assert_eq!(err, ParseError::InvalidHttpVersion("HTTP/9.9".to_string()));
    }
}
