//! YAML formatter for [`Value`].
//!
//! The formatter produces a minimal YAML representation. It is not intended
//! to be fully compliant with all YAML features but is sufficient for tests
//! and examples.

use crate::concepts::value::{Value, ValueFormatter};
use crate::concepts::Dictionary;
use serde_yaml::Value as YamlValue;

#[derive(Clone)]
pub struct YamlFormatter;

impl YamlFormatter {
    fn format_with_indent(&self, value: Value, indent: usize) -> String {
        match value {
            Value::Null => "null".to_string(),
            Value::Bool(b) => if b { "true" } else { "false" }.to_string(),
            Value::Int(i) => i.to_string(),
            Value::Number(f) => f.to_string(),
            Value::String(s) => format!("\"{}\"", s.replace('"', "\\\"")),
            Value::Array(a) => {
                let mut out = String::new();
                for v in a {
                    out.push_str(&" ".repeat(indent));
                    out.push_str("- ");
                    out.push_str(&self.format_with_indent(v, indent + 2));
                    out.push('\n');
                }
                if out.ends_with('\n') {
                    out.pop();
                }
                out
            }
            Value::Dictionary(d) => {
                let mut out = String::new();
                for (k, v) in d {
                    out.push_str(&" ".repeat(indent));
                    out.push_str(&format!("{}: ", k));
                    match v {
                        Value::Array(_) | Value::Dictionary(_) => {
                            out.push('\n');
                            out.push_str(&self.format_with_indent(v, indent + 2));
                            out.push('\n');
                        }
                        _ => {
                            out.push_str(&self.format_with_indent(v, indent));
                            out.push('\n');
                        }
                    }
                }
                if out.ends_with('\n') {
                    out.pop();
                }
                out
            }
        }
    }
}

impl ValueFormatter for YamlFormatter {
    fn format(&self, value: Value) -> String {
        self.format_with_indent(value, 0)
    }

    fn parse(&self, input: &str) -> Option<Value> {
        serde_yaml::from_str::<YamlValue>(input).ok().map(from_yaml)
    }
}

fn from_yaml(v: YamlValue) -> Value {
    match v {
        YamlValue::Null => Value::Null,
        YamlValue::Bool(b) => Value::Bool(b),
        YamlValue::Number(n) => {
            if let Some(i) = n.as_i64() {
                Value::Int(i)
            } else if let Some(f) = n.as_f64() {
                Value::Number(f)
            } else {
                Value::String(n.to_string())
            }
        }
        YamlValue::String(s) => Value::String(s),
        YamlValue::Sequence(seq) => Value::Array(seq.into_iter().map(from_yaml).collect()),
        YamlValue::Mapping(map) => {
            let mut d = Dictionary::new();
            for (k, v) in map {
                if let YamlValue::String(key) = k {
                    d.insert(key, from_yaml(v));
                }
            }
            Value::Dictionary(d)
        }
        YamlValue::Tagged(_) => Value::Null,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::concepts::Dictionary;
    use serde_yaml::Value as YamlValue;

    #[test]
    fn test_scalar() {
        let f = YamlFormatter;
        assert_eq!(f.format(Value::Bool(false)), "false");
    }

    #[test]
    fn test_array() {
        let f = YamlFormatter;
        let arr = Value::Array(vec![Value::Int(1), Value::Int(2)]);
        assert_eq!(f.format(arr), "- 1\n- 2");
    }

    #[test]
    fn test_dictionary() {
        let f = YamlFormatter;
        let mut d = Dictionary::new();
        d.insert("a".to_string(), Value::Int(1));
        assert_eq!(f.format(Value::Dictionary(d)), "a: 1");
    }

    #[test]
    fn test_nested_dictionary() {
        let f = YamlFormatter;

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

        let parsed: YamlValue = serde_yaml::from_str(&out).unwrap();
        let expected: YamlValue =
            serde_yaml::from_str("outer:\n  inner:\n    a: 1\n    b: 2\n  flag: true\ncount: 10")
                .unwrap();
        assert_eq!(parsed, expected);
    }
}
