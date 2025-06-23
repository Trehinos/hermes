//! Simple utilities for working with HTTP cookies.
//!
//! The module provides a [`Cookie`] type and a [`CookieJar`] collection
//! to parse and generate `Cookie` and `Set-Cookie` header values.

use crate::concepts::Dictionary;

/// Representation of a single HTTP cookie.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Cookie {
    /// Name of the cookie.
    pub name: String,
    /// Value stored in the cookie.
    pub value: String,
}

impl Cookie {
    /// Create a new [`Cookie`] with the provided name and value.
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }
}

#[derive(Debug, Default, Clone)]
/// Collection of cookies typically stored in the `Cookie` header.
pub struct CookieJar {
    cookies: Dictionary<String>,
}

impl CookieJar {
    /// Create an empty [`CookieJar`].
    pub fn new() -> Self {
        Self {
            cookies: Dictionary::new(),
        }
    }

    /// Insert or update a cookie.
    pub fn insert(&mut self, name: impl Into<String>, value: impl Into<String>) {
        self.cookies.insert(name.into(), value.into());
    }

    /// Get a cookie value by name.
    pub fn get(&self, name: &str) -> Option<&String> {
        self.cookies.get(name)
    }

    /// Remove a cookie from the jar.
    pub fn remove(&mut self, name: &str) {
        self.cookies.remove(name);
    }

    /// Parse cookies from a `Cookie` header string.
    pub fn parse(header: &str) -> Self {
        let mut jar = Self::new();
        for pair in header.split(';') {
            let trimmed = pair.trim();
            if let Some((n, v)) = trimmed.split_once('=') {
                jar.insert(n.trim(), v.trim());
            }
        }
        jar
    }

    /// Render the cookies as a `Cookie` header line.
    pub fn to_header(&self) -> String {
        self.cookies
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join("; ")
    }

    /// Iterate over stored cookie name/value pairs.
    pub fn iter(&self) -> impl Iterator<Item = (&String, &String)> {
        self.cookies.iter()
    }
}
