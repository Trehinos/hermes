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
