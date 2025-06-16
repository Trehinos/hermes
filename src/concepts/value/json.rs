//! JSON formatter for [`Value`].
//!
//! This module defines [`JsonFormatter`], a basic formatter converting a
//! [`Value`] into its JSON string representation.

use crate::concepts::value::{Value, ValueFormatter};

pub struct JsonFormatter;

impl ValueFormatter for JsonFormatter {
    fn format(&self, value: Value) -> String {
        match value {
            Value::Null => "null".to_string(),
            Value::Bool(b) => if b { "true" } else { "false" }.to_string(),
            Value::Int(i) => i.to_string(),
            Value::Number(f) => f.to_string(),
            Value::String(s) => format!("\"{}\"", s.replace("\\", "\\\\").replace("\"", "\\\"")),
            Value::Array(a) => {
                let mut s = "[".to_string();
                for v in a {
                    s.push_str(&self.format(v));
                    s.push(',');
                }
                s.pop();
                s.push(']');
                s
            }
            Value::Dictionary(d) => {
                let mut s = "{".to_string();
                for (k, v) in d {
                    s.push_str(&format!("\"{}\": ", k));
                    s.push_str(&self.format(v));
                    s.push(',');
                }
                s.pop();
                s.push('}');
                s
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::concepts::Dictionary;
    use serde_json::json;

    #[test]
    fn test_basic_types() {
        let f = JsonFormatter;
        assert_eq!(f.format(Value::Null), "null");
        assert_eq!(f.format(Value::Bool(true)), "true");
        assert_eq!(f.format(Value::Int(3)), "3");
        assert_eq!(f.format(Value::String("a".to_string())), "\"a\"");
    }

    #[test]
    fn test_array_and_dict() {
        let f = JsonFormatter;
        let arr = Value::Array(vec![Value::Int(1), Value::Int(2)]);
        assert_eq!(f.format(arr), "[1,2]");

        let mut d = Dictionary::new();
        d.insert("k".to_string(), Value::Bool(false));
        assert_eq!(f.format(Value::Dictionary(d)), "{\"k\": false}");
    }

    #[test]
    fn test_nested_dictionary() {
        let f = JsonFormatter;

        let mut inner = Dictionary::new();
        inner.insert("a".to_string(), Value::Int(1));
        inner.insert("b".to_string(), Value::Int(2));

        let mut outer = Dictionary::new();
        outer.insert("inner".to_string(), Value::Dictionary(inner));
        outer.insert("flag".to_string(), Value::Bool(true));

        let mut root = Dictionary::new();
        root.insert("outer".to_string(), Value::Dictionary(outer));
        root.insert("count".to_string(), Value::Int(10));

        let out = f.format(Value::Dictionary(root));

        let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
        let expected = json!({"outer": {"inner": {"a": 1, "b": 2}, "flag": true}, "count": 10});
        assert_eq!(parsed, expected);
    }
}
