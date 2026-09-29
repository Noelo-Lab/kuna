//! Ordered JSON values and the CLI's CPython-compatible ASCII-safe rendering.

#[path = "jsonfmt/parser.rs"]
mod parser;

#[path = "jsonfmt/writer.rs"]
mod writer;

use writer::Format;

#[cfg(test)]
#[path = "jsonfmt/writer_tests.rs"]
mod writer_tests;

/// Parse one complete JSON value, allowing trailing whitespace.
pub fn parse(text: &str) -> Option<Json> {
    parser::parse(text)
}

#[derive(Debug, Clone, PartialEq)]
pub enum Json {
    Null,
    Bool(bool),
    /// The original numeric token, without rounding or normalization.
    Number(String),
    Str(String),
    Array(Vec<Json>),
    Object(Vec<(String, Json)>),
}

/// Render `v` exactly as CPython's `json.dumps(v, indent=2)` would (ASCII-safe
/// string escaping, `", "`/`": "` item/key separators collapsed to the indent
/// form: items separated by `,\n`, keys by `": "`).
pub fn dumps_indent2(v: &Json) -> String {
    writer::render(v, Format::Pretty)
}

/// As [`dumps_indent2`] but with `sort_keys=True` (object keys emitted in
/// ascending byte order) — matches `kuna/run_tests.py`'s
/// `json.dumps(..., indent=2, sort_keys=True)`.
pub fn dumps_indent2_sorted(v: &Json) -> String {
    writer::render(v, Format::Sorted)
}

/// Render `v` on ONE line, with no spaces — the `index.jsonl` / `.streaming`
/// form, where a record has to be a single line a reader can consume as it
/// lands.  String escaping is the same `ensure_ascii` rendering the pretty
/// printers use.
pub fn dumps_compact(v: &Json) -> String {
    writer::render(v, Format::Compact)
}

/// Extract the JSON array/object span out of a `decomp_dbg` console transcript.
///
/// Port of `kuna/catalog.py::_extract_json`: drop the fixed `[decomp]>` prompt
/// (whose `[`/`]` are not JSON), scan to the first `[`/`{`, then balance
/// brackets ignoring those inside string literals, and return the spanned text.
pub fn extract_json_span(transcript: &str) -> Option<&str> {
    let bytes = transcript.as_bytes();
    let prompt = b"[decomp]>";
    let mut i = 0usize;
    let mut start = None;
    while i < bytes.len() {
        if bytes[i..].starts_with(prompt) {
            i += prompt.len();
            continue;
        }
        let c = bytes[i];
        if c == b'[' || c == b'{' {
            start = Some(i);
            break;
        }
        i += 1;
    }
    let start = start?;
    let mut depth = 0i32;
    let mut in_str = false;
    let mut escape = false;
    let mut j = start;
    while j < bytes.len() {
        // A prompt can appear mid-output; skip it so its brackets never count.
        if !in_str && bytes[j..].starts_with(prompt) {
            j += prompt.len();
            continue;
        }
        let c = bytes[j];
        if in_str {
            if escape {
                escape = false;
            } else if c == b'\\' {
                escape = true;
            } else if c == b'"' {
                in_str = false;
            }
            j += 1;
            continue;
        }
        match c {
            b'"' => in_str = true,
            b'[' | b'{' => depth += 1,
            b']' | b'}' => {
                depth -= 1;
                if depth == 0 {
                    return transcript.get(start..=j);
                }
            }
            _ => {}
        }
        j += 1;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_object_preserves_key_order() {
        let v = parse(r#"{"b": 1, "a": "x", "c": [true, false]}"#).unwrap();
        let out = dumps_indent2(&v);
        assert_eq!(
            out,
            "{\n  \"b\": 1,\n  \"a\": \"x\",\n  \"c\": [\n    true,\n    false\n  ]\n}"
        );
    }

    #[test]
    fn empty_containers() {
        assert_eq!(dumps_indent2(&parse("[]").unwrap()), "[]");
        assert_eq!(dumps_indent2(&parse("{}").unwrap()), "{}");
    }

    #[test]
    fn extract_span_skips_prompt_brackets() {
        let t = "[decomp]> stage catalog\n[ {\"x\": 1} ]\n[decomp]> quit\n";
        let span = extract_json_span(t).unwrap();
        assert_eq!(span, "[ {\"x\": 1} ]");
    }

    #[test]
    fn ascii_escape_high_chars() {
        let v = Json::Str("\u{26a0}\u{fe0f} opt-in".to_string());
        // warning sign + variation selector -> ⚠️
        assert_eq!(dumps_compact(&v), "\"\\u26a0\\ufe0f opt-in\"");
    }
}
