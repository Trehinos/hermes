use crate::concepts::Dictionary;

#[derive(Debug, Clone)]
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
}

pub mod json;