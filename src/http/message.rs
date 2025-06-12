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
                    .split(',')
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
    pub fn from(headers: &[(&str, &[&str])]) -> Self {
        Self {
            data: headers
                .iter()
                .map(|(s, v)| (s.to_string(), v.iter().map(|s| s.to_string()).collect()))
                .collect(),
        }
    }
    pub fn merge_with(&self, other: &Self) -> Self {
        let mut headers = self.clone();
        for (key, values) in other.data.iter() {
            headers.insert(key, values);
        }
        headers
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
    pub fn set(&mut self, key: &str, values: &[&str]) {
        self.data.remove(key);
        self.data.insert(
            key.to_string(),
            values.iter().map(|s| s.to_string()).collect(),
        );
    }
    pub fn get(&self, key: &str) -> Option<&Vec<String>> {
        self.data.get(key)
    }
    pub fn get_line(&self, key: &str) -> Option<String> {
        self.get(key).map(|v| v.join(","))
    }
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
        Headers::from(&[("A", &["1"]), ("B", &["2"])] )
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
        for (_, vals) in merged.iter() { count += vals.len(); }
        for (_, vals) in merged.clone().iter_mut() { let _ = vals.len(); }
        assert_eq!(count, 3);
    }

    #[test]
    fn test_message_methods() {
        let msg = Message::v1_1(sample_headers(), "body".into());
        assert!(msg.raw().contains("A: 1"));
        assert!(msg.raw().contains("B: 2"));
        assert!(msg.to_string().starts_with("HTTP/1.1"));
    }
}
