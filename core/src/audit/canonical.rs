use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CanonicalJson {
    Null,
    Bool(bool),
    Number(i64),
    String(String),
    Array(Vec<CanonicalJson>),
    Object(BTreeMap<String, CanonicalJson>),
}

impl CanonicalJson {
    pub fn object() -> Self {
        CanonicalJson::Object(BTreeMap::new())
    }

    pub fn insert(&mut self, k: impl Into<String>, v: CanonicalJson) {
        if let CanonicalJson::Object(ref mut m) = self {
            m.insert(k.into(), v);
        }
    }

    pub fn encode(&self) -> String {
        match self {
            CanonicalJson::Null => "null".to_string(),
            CanonicalJson::Bool(b) => {
                if *b {
                    "true".to_string()
                } else {
                    "false".to_string()
                }
            }
            CanonicalJson::Number(n) => n.to_string(),
            CanonicalJson::String(s) => format!("\"{}\"", escape_json(s)),
            CanonicalJson::Array(arr) => {
                let mut out = String::from("[");
                for (i, v) in arr.iter().enumerate() {
                    if i > 0 {
                        out.push(',');
                    }
                    out.push_str(&v.encode());
                }
                out.push(']');
                out
            }
            CanonicalJson::Object(obj) => {
                let mut out = String::from("{");
                for (i, (k, v)) in obj.iter().enumerate() {
                    if i > 0 {
                        out.push(',');
                    }
                    out.push('"');
                    out.push_str(&escape_json(k));
                    out.push_str("\":");
                    out.push_str(&v.encode());
                }
                out.push('}');
                out
            }
        }
    }
}

impl fmt::Display for CanonicalJson {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.encode())
    }
}

fn escape_json(s: &str) -> String {
    let mut out = String::new();
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if c.is_control() => {
                out.push_str(&format!("\\u{:04x}", c as u32));
            }
            _ => out.push(c),
        }
    }
    out
}
