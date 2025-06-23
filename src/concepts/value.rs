//! Representation of loosely typed values and associated formatters.
//!
//! The [`Value`] enum can model typical JSON or YAML compatible values. It is
//! accompanied by the [`ValueFormatter`] trait which allows serialising a
//! `Value` into various textual formats.

use crate::concepts::Dictionary;

#[derive(Debug, Clone, PartialEq)]
/// Represents a generic JSON-like value used when parsing parameters.
pub enum Value {
    Null,
    Bool(bool),
    Int(i64),
    Number(f64),
    String(String),
    Array(Vec<Value>),
    Dictionary(Dictionary<Value>),
}

pub trait ValueFormatter {
    fn format(&self, value: Value) -> String;
    fn parse(&self, input: &str) -> Option<Value>;
}

pub mod json;
pub mod yaml;
