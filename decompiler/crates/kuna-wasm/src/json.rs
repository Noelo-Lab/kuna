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
        let address = u64::MAX - 1;
        let wide = Obj::new().addr("address", address).end();
        let wide: serde_json::Value = serde_json::from_str(&wide).expect("valid JSON");
        assert_eq!(wide["address"].as_u64(), Some(address));
        assert_eq!(wide["address_hex"].as_str(), Some("0xfffffffffffffffe"));
        let back: serde_json::Value = serde_json::from_str(&o).expect("valid JSON");
        assert_eq!(back["name"].as_str(), Some("a\"b\n"));
        assert_eq!(back["list"].as_array().expect("list").len(), 2);
        assert_eq!(back["address_hex"].as_str(), Some("0x1161"));
    }
}
