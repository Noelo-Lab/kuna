//! Decode JSON without normalizing number tokens or merging duplicate keys.

use serde::de::{Deserialize, Deserializer, MapAccess, Visitor};
use serde_json::value::RawValue;

use super::Json;

const MAX_DEPTH: usize = 128;

pub(super) fn parse(text: &str) -> Option<Json> {
    value(serde_json::from_str(text).ok()?, 0)
}

fn value(raw: &RawValue, depth: usize) -> Option<Json> {
    let text = raw.get();
    if depth >= MAX_DEPTH && matches!(text.as_bytes()[0], b'[' | b'{') {
        return None;
    }
    match text.as_bytes()[0] {
        b'n' => Some(Json::Null),
        b't' => Some(Json::Bool(true)),
        b'f' => Some(Json::Bool(false)),
        b'"' => serde_json::from_str(text).ok().map(Json::Str),
        b'[' => serde_json::from_str::<Vec<&RawValue>>(text)
            .ok()?
            .into_iter()
            .map(|raw| value(raw, depth + 1))
            .collect::<Option<_>>()
            .map(Json::Array),
        b'{' => serde_json::from_str::<Object>(text)
            .ok()?
            .0
            .into_iter()
            .map(|(key, raw)| Some((key, value(raw, depth + 1)?)))
            .collect::<Option<_>>()
            .map(Json::Object),
        _ => Some(Json::Number(text.into())),
    }
}

struct Object<'a>(Vec<(String, &'a RawValue)>);

impl<'de> Deserialize<'de> for Object<'de> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct ObjectVisitor;

        impl<'de> Visitor<'de> for ObjectVisitor {
            type Value = Object<'de>;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str("an object")
            }

            fn visit_map<M: MapAccess<'de>>(self, mut map: M) -> Result<Self::Value, M::Error> {
                let mut pairs = Vec::new();
                while let Some(pair) = map.next_entry()? {
                    pairs.push(pair);
                }
                Ok(Object(pairs))
            }
        }

        deserializer.deserialize_map(ObjectVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::jsonfmt::dumps_compact;

    #[test]
    fn preserves_numeric_tokens_and_duplicate_key_order() {
        let text = r#"{"b":0,"a":-0,"b":[1.0,1e0,1E+2,18446744073709551616,1e9999]}"#;
        assert_eq!(dumps_compact(&parse(text).unwrap()), text);
    }

    #[test]
    fn decodes_unicode_and_all_string_escapes() {
        let text = r#"{"\ud83d\ude80":"\b\f\n\r\t\/\\\""}"#;
        assert_eq!(
            parse(text),
            Some(Json::Object(vec![(
                "🚀".into(),
                Json::Str("\u{8}\u{c}\n\r\t/\\\"".into())
            )]))
        );
        assert_eq!(parse("\"🚀\""), parse(r#""\ud83d\ude80""#));
    }

    #[test]
    fn rejects_malformed_values_and_trailing_content() {
        for text in [
            "",
            "-",
            "01",
            "1.",
            "1e",
            "1e+",
            "1-2",
            "true false",
            "[] garbage",
            "[1,]",
            "{\"a\":1,}",
            "{\"a\" 1}",
            "\"raw\nnewline\"",
            "\"\u{0}\"",
            r#""\ud800""#,
            r#""\udc00""#,
            r#""\x41""#,
        ] {
            assert!(parse(text).is_none(), "accepted {text:?}");
        }
        assert_eq!(parse(" \n\ttrue\r\n"), Some(Json::Bool(true)));
    }

    #[test]
    fn bounds_recursive_conversion() {
        let nested = |depth, leaf| format!("{}{leaf}{}", "[".repeat(depth), "]".repeat(depth));
        assert!(parse(&nested(MAX_DEPTH, "null")).is_some());
        assert!(parse(&nested(MAX_DEPTH + 1, "null")).is_none());
        for empty in ["[]", "{}"] {
            assert!(parse(&nested(MAX_DEPTH - 1, empty)).is_some());
            assert!(parse(&nested(MAX_DEPTH, empty)).is_none());
        }
    }
}
