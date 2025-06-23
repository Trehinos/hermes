//! Error types for HTTP parsing operations.
use std::fmt::{self, Display};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    /// An invalid HTTP version was encountered.
    InvalidHttpVersion(String),
    /// A status code was malformed.
    InvalidStatusCode(String),
    /// A port number could not be parsed.
    InvalidPort(String),
    /// A header line could not be parsed.
    InvalidHeaderFormat(String),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::InvalidHttpVersion(v) => {
                write!(f, "Invalid HTTP version: {}", v)
            }
            ParseError::InvalidStatusCode(code) => write!(f, "Invalid status code: {}", code),
            ParseError::InvalidPort(port) => write!(f, "Invalid port: {}", port),
            ParseError::InvalidHeaderFormat(line) => {
                write!(f, "Invalid header format: {}", line)
            }
        }
    }
}

impl std::error::Error for ParseError {}
