//! JSON text for the front-end's documents: string escaping, the small field
//! encoders the hand-formatted `list`/`decompile`/`project` documents share,
//! and a compact object/array writer for the study-view documents.

use kuna_console::assertions::Outcome;
use kuna_console::engine::ObjectLocation;

/// Encode a Rust string as a JSON string literal (RFC 8259 escaping).
pub fn json_str(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\u{08}' => out.push_str("\\b"),
            '\u{0c}' => out.push_str("\\f"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

pub fn json_opt_str(v: Option<&str>) -> String {
    match v {
        Some(s) => json_str(s),
        None => "null".to_string(),
    }
}

pub fn json_opt_num(v: Option<i64>) -> String {
    match v {
        Some(n) => n.to_string(),
        None => "null".to_string(),
    }
}

/// (kuna, issue #197) A JSON array of strings — the `aliases` field: every
/// OTHER name the reported entry carries.  Always present (`[]` when the entry
/// has exactly one name).
pub fn json_str_array(items: &[String]) -> String {
    let mut out = String::from("[");
    for (i, s) in items.iter().enumerate() {
        if i > 0 {
            out.push_str(", ");
        }
        out.push_str(&json_str(s));
    }
    out.push(']');
    out
}

pub fn json_object_location(location: Option<&ObjectLocation>) -> String {
    match location {
        Some(location) => format!(
            "{{\"section_index\": {}, \"section\": {}, \"offset\": {}, \"offset_hex\": {}}}",
            location.section_index,
            json_str(&location.section),
            location.offset,
            json_str(&format!("0x{:x}", location.offset))
        ),
        None => "null".to_string(),
    }
}

/// `0x<hex>`, the `_hex` twin of every address field.
pub fn hex_str(value: u64) -> String {
    json_str(&format!("0x{value:x}"))
}

/// A compact JSON object built field by field; values are JSON text.
pub struct Obj(String);

impl Obj {
    pub fn new() -> Self {
        Obj(String::from("{"))
    }

    pub fn raw(mut self, key: &str, value: &str) -> Self {
        if self.0.len() > 1 {
            self.0.push(',');
        }
        self.0.push_str(&json_str(key));
        self.0.push(':');
        self.0.push_str(value);
        self
    }

    pub fn str(self, key: &str, value: &str) -> Self {
        self.raw(key, &json_str(value))
    }

    pub fn opt_str(self, key: &str, value: Option<&str>) -> Self {
        self.raw(key, &json_opt_str(value))
    }

    pub fn num(self, key: &str, value: impl std::fmt::Display) -> Self {
        self.raw(key, &value.to_string())
    }

    pub fn opt_num(self, key: &str, value: Option<impl std::fmt::Display>) -> Self {
        match value {
            Some(v) => self.raw(key, &v.to_string()),
            None => self.raw(key, "null"),
        }
    }

    pub fn bool(self, key: &str, value: bool) -> Self {
        self.raw(key, if value { "true" } else { "false" })
    }

    /// An address and its `<key>_hex` twin.
    pub fn addr(self, key: &str, value: u64) -> Self {
        let hex = hex_str(value);
        self.num(key, value).raw(&format!("{key}_hex"), &hex)
    }

    pub fn end(mut self) -> String {
        self.0.push('}');
        self.0
    }
}

impl Default for Obj {
    fn default() -> Self {
        Obj::new()
    }
}

/// A compact JSON array of already-encoded values.
pub fn arr<I: IntoIterator<Item = String>>(items: I) -> String {
    let mut out = String::from("[");
    for (i, item) in items.into_iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        out.push_str(&item);
    }
    out.push(']');
    out
}

/// The per-directive report, the fields `kuna decompile-all --json` gives it.
pub fn assertions_json(outcomes: &[Outcome]) -> String {
    arr(outcomes.iter().map(|o| {
        Obj::new()
            .str("directive", &o.directive)
            .str("kind", o.kind)
            .str("phase", o.phase)
            .str("subphase", o.subphase)
            .str("status", o.status)
            .opt_str("detail", o.detail.as_deref())
            .bool("fatal", o.fatal)
            .end()
    }))
}

/// A minimal JSON reader for this crate's tests: enough to walk the documents
/// the front-end writes.
#[cfg(test)]
pub(crate) mod read {
    use std::collections::BTreeMap;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Value {
        Null,
        Bool(bool),
        Num(f64),
        Str(String),
        Arr(Vec<Value>),
        Obj(BTreeMap<String, Value>),
    }

    impl Value {
        pub fn get(&self, key: &str) -> &Value {
            match self {
                Value::Obj(map) => map.get(key).unwrap_or(&Value::Null),
                _ => &Value::Null,
            }
        }
        pub fn arr(&self) -> &[Value] {
            match self {
                Value::Arr(items) => items,
                _ => &[],
            }
        }
        pub fn str(&self) -> Option<&str> {
            match self {
                Value::Str(s) => Some(s),
                _ => None,
            }
        }
        pub fn num(&self) -> Option<f64> {
            match self {
                Value::Num(n) => Some(*n),
                _ => None,
            }
        }
        pub fn u64(&self) -> Option<u64> {
            self.num().map(|n| n as u64)
        }
        pub fn has(&self, key: &str) -> bool {
            matches!(self, Value::Obj(map) if map.contains_key(key))
        }
    }

    pub fn parse(text: &str) -> Value {
        let chars: Vec<char> = text.chars().collect();
        let mut at = 0;
        let v = value(&chars, &mut at);
        skip(&chars, &mut at);
        assert_eq!(at, chars.len(), "trailing text after JSON value");
        v
    }

    fn skip(c: &[char], at: &mut usize) {
        while *at < c.len() && c[*at].is_whitespace() {
            *at += 1;
        }
    }

    fn value(c: &[char], at: &mut usize) -> Value {
        skip(c, at);
        match c[*at] {
            '{' => {
                *at += 1;
                let mut map = BTreeMap::new();
                loop {
                    skip(c, at);
                    if c[*at] == '}' {
                        *at += 1;
                        return Value::Obj(map);
                    }
                    let Value::Str(key) = value(c, at) else { panic!("object key") };
                    skip(c, at);
                    assert_eq!(c[*at], ':');
                    *at += 1;
                    map.insert(key, value(c, at));
                    skip(c, at);
                    if c[*at] == ',' {
                        *at += 1;
                    }
                }
            }
            '[' => {
                *at += 1;
                let mut items = Vec::new();
                loop {
                    skip(c, at);
                    if c[*at] == ']' {
                        *at += 1;
                        return Value::Arr(items);
                    }
                    items.push(value(c, at));
                    skip(c, at);
                    if c[*at] == ',' {
                        *at += 1;
                    }
                }
            }
            '"' => {
                *at += 1;
                let mut s = String::new();
                while c[*at] != '"' {
                    if c[*at] == '\\' {
                        *at += 1;
                        match c[*at] {
                            'n' => s.push('\n'),
                            't' => s.push('\t'),
                            'r' => s.push('\r'),
                            'b' => s.push('\u{8}'),
                            'f' => s.push('\u{c}'),
                            'u' => {
                                let hex: String = c[*at + 1..*at + 5].iter().collect();
                                s.push(char::from_u32(u32::from_str_radix(&hex, 16).unwrap()).unwrap());
                                *at += 4;
                            }
                            other => s.push(other),
                        }
                    } else {
                        s.push(c[*at]);
                    }
                    *at += 1;
                }
                *at += 1;
                Value::Str(s)
            }
            't' => {
                *at += 4;
                Value::Bool(true)
            }
            'f' => {
                *at += 5;
                Value::Bool(false)
            }
            'n' => {
                *at += 4;
                Value::Null
            }
            _ => {
                let start = *at;
                while *at < c.len() && matches!(c[*at], '-' | '+' | '.' | 'e' | 'E' | '0'..='9') {
                    *at += 1;
                }
                let text: String = c[start..*at].iter().collect();
                Value::Num(text.parse().unwrap())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn objects_and_arrays_are_compact_and_escaped() {
        let o = Obj::new()
            .str("name", "a\"b\n")
            .num("n", 3)
            .opt_num("none", None::<i64>)
            .bool("ok", true)
            .addr("address", 0x1161)
            .raw("list", &arr(["1".to_string(), "2".to_string()]))
            .end();
        assert_eq!(
            o,
            "{\"name\":\"a\\\"b\\n\",\"n\":3,\"none\":null,\"ok\":true,\"address\":4449,\
             \"address_hex\":\"0x1161\",\"list\":[1,2]}"
        );
        assert_eq!(arr(Vec::<String>::new()), "[]");
        let back = read::parse(&o);
        assert_eq!(back.get("name").str(), Some("a\"b\n"));
        assert_eq!(back.get("list").arr().len(), 2);
        assert_eq!(back.get("address_hex").str(), Some("0x1161"));
    }
}
