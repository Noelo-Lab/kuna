//! A minimal JSON value model + a pretty-printer byte-identical to CPython's
//! `json.dumps(obj, indent=2)` (the default `ensure_ascii=True`, key order
//! preserved — NOT `sort_keys`).
//!
//! `kuna/catalog.py` produced its `--json` output by running `decomp_dbg stage
//! catalog`, `json.loads`-ing the embedded array, then `json.dumps(..., indent=2)`.
//! To match it byte-for-byte the Rust CLI parses the binary's JSON into this
//! model (preserving object key order) and re-emits with the exact CPython
//! indent-2 layout.  No serde dependency: the catalog JSON is the binary's own
//! controlled, ASCII, escape-free output (verified), so a small hand parser is
//! both sufficient and the lightest faithful reproduction.

use std::fmt::Write as _;

/// A parsed JSON value.  Object key order is preserved (a `Vec` of pairs), which
/// is what `json.loads` does (Python dicts are insertion-ordered) and what the
/// catalog re-emission must reproduce.
#[derive(Debug, Clone, PartialEq)]
pub enum Json {
    Null,
    Bool(bool),
    /// The raw numeric token (preserved verbatim, as CPython would re-emit an
    /// int unchanged; the catalog only ever uses bools/strings/arrays, but
    /// numbers are supported for completeness).
    Number(String),
    Str(String),
    Array(Vec<Json>),
    Object(Vec<(String, Json)>),
}

/// Parse a single JSON value from `s`, returning it (trailing whitespace is
/// permitted).  Returns `None` on any malformed input.
pub fn parse(s: &str) -> Option<Json> {
    let bytes = s.as_bytes();
    let mut p = Parser { bytes, pos: 0 };
    p.skip_ws();
    let v = p.value()?;
    p.skip_ws();
    // Allow trailing content (the console transcript), since callers extract the
    // balanced span first; here we only require a complete value was parsed.
    Some(v)
}

struct Parser<'a> {
    bytes: &'a [u8],
    pos: usize,
}

impl Parser<'_> {
    fn peek(&self) -> Option<u8> {
        self.bytes.get(self.pos).copied()
    }

    fn skip_ws(&mut self) {
        while let Some(c) = self.peek() {
            if c == b' ' || c == b'\t' || c == b'\n' || c == b'\r' {
                self.pos += 1;
            } else {
                break;
            }
        }
    }

    fn value(&mut self) -> Option<Json> {
        self.skip_ws();
        match self.peek()? {
            b'{' => self.object(),
            b'[' => self.array(),
            b'"' => Some(Json::Str(self.string()?)),
            b't' => self.literal("true", Json::Bool(true)),
            b'f' => self.literal("false", Json::Bool(false)),
            b'n' => self.literal("null", Json::Null),
            b'-' | b'0'..=b'9' => self.number(),
            _ => None,
        }
    }

    fn literal(&mut self, kw: &str, val: Json) -> Option<Json> {
        if self.bytes[self.pos..].starts_with(kw.as_bytes()) {
            self.pos += kw.len();
            Some(val)
        } else {
            None
        }
    }

    fn number(&mut self) -> Option<Json> {
        let start = self.pos;
        if self.peek() == Some(b'-') {
            self.pos += 1;
        }
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() || c == b'.' || c == b'e' || c == b'E' || c == b'+' || c == b'-' {
                self.pos += 1;
            } else {
                break;
            }
        }
        if self.pos == start {
            return None;
        }
        Some(Json::Number(
            std::str::from_utf8(&self.bytes[start..self.pos]).ok()?.to_string(),
        ))
    }

    fn string(&mut self) -> Option<String> {
        // Consume the opening quote.
        if self.peek() != Some(b'"') {
            return None;
        }
        self.pos += 1;
        let mut out = String::new();
        loop {
            let c = self.peek()?;
            self.pos += 1;
            match c {
                b'"' => return Some(out),
                b'\\' => {
                    let e = self.peek()?;
                    self.pos += 1;
                    match e {
                        b'"' => out.push('"'),
                        b'\\' => out.push('\\'),
                        b'/' => out.push('/'),
                        b'b' => out.push('\u{0008}'),
                        b'f' => out.push('\u{000C}'),
                        b'n' => out.push('\n'),
                        b'r' => out.push('\r'),
                        b't' => out.push('\t'),
                        b'u' => {
                            let hex = self.bytes.get(self.pos..self.pos + 4)?;
                            self.pos += 4;
                            let cp = u32::from_str_radix(std::str::from_utf8(hex).ok()?, 16).ok()?;
                            out.push(char::from_u32(cp)?);
                        }
                        _ => return None,
                    }
                }
                _ => {
                    // A raw byte; reconstruct the UTF-8 char (the input is valid
                    // UTF-8, so push the byte sequence for this code point).
                    if c < 0x80 {
                        out.push(c as char);
                    } else {
                        // Multi-byte: back up and decode via str slicing.
                        let start = self.pos - 1;
                        let s = std::str::from_utf8(&self.bytes[start..]).ok()?;
                        let ch = s.chars().next()?;
                        out.push(ch);
                        self.pos = start + ch.len_utf8();
                    }
                }
            }
        }
    }

    fn array(&mut self) -> Option<Json> {
        self.pos += 1; // '['
        let mut items = Vec::new();
        self.skip_ws();
        if self.peek() == Some(b']') {
            self.pos += 1;
            return Some(Json::Array(items));
        }
        loop {
            let v = self.value()?;
            items.push(v);
            self.skip_ws();
            match self.peek()? {
                b',' => {
                    self.pos += 1;
                }
                b']' => {
                    self.pos += 1;
                    return Some(Json::Array(items));
                }
                _ => return None,
            }
        }
    }

    fn object(&mut self) -> Option<Json> {
        self.pos += 1; // '{'
        let mut pairs = Vec::new();
        self.skip_ws();
        if self.peek() == Some(b'}') {
            self.pos += 1;
            return Some(Json::Object(pairs));
        }
        loop {
            self.skip_ws();
            let key = self.string()?;
            self.skip_ws();
            if self.peek() != Some(b':') {
                return None;
            }
            self.pos += 1;
            let v = self.value()?;
            pairs.push((key, v));
            self.skip_ws();
            match self.peek()? {
                b',' => {
                    self.pos += 1;
                }
                b'}' => {
                    self.pos += 1;
                    return Some(Json::Object(pairs));
                }
                _ => return None,
            }
        }
    }
}

/// Render `v` exactly as CPython's `json.dumps(v, indent=2)` would (ASCII-safe
/// string escaping, `", "`/`": "` item/key separators collapsed to the indent
/// form: items separated by `,\n`, keys by `": "`).
pub fn dumps_indent2(v: &Json) -> String {
    let mut out = String::new();
    write_value_opts(&mut out, v, 0, false);
    out
}

/// As [`dumps_indent2`] but with `sort_keys=True` (object keys emitted in
/// ascending byte order) — matches `kuna/run_tests.py`'s
/// `json.dumps(..., indent=2, sort_keys=True)`.
pub fn dumps_indent2_sorted(v: &Json) -> String {
    let mut out = String::new();
    write_value_opts(&mut out, v, 0, true);
    out
}

fn write_value_opts(out: &mut String, v: &Json, indent: usize, sort_keys: bool) {
    match v {
        Json::Null => out.push_str("null"),
        Json::Bool(true) => out.push_str("true"),
        Json::Bool(false) => out.push_str("false"),
        Json::Number(n) => out.push_str(n),
        Json::Str(s) => write_string(out, s),
        Json::Array(items) => {
            if items.is_empty() {
                out.push_str("[]");
                return;
            }
            out.push('[');
            let child = indent + 2;
            for (i, item) in items.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                out.push('\n');
                pad(out, child);
                write_value_opts(out, item, child, sort_keys);
            }
            out.push('\n');
            pad(out, indent);
            out.push(']');
        }
        Json::Object(pairs) => {
            if pairs.is_empty() {
                out.push_str("{}");
                return;
            }
            out.push('{');
            let child = indent + 2;
            // sort_keys=True orders by the (Unicode code point) key, like CPython.
            let mut order: Vec<usize> = (0..pairs.len()).collect();
            if sort_keys {
                order.sort_by(|&a, &b| pairs[a].0.cmp(&pairs[b].0));
            }
            for (i, &idx) in order.iter().enumerate() {
                let (k, val) = &pairs[idx];
                if i > 0 {
                    out.push(',');
                }
                out.push('\n');
                pad(out, child);
                write_string(out, k);
                out.push_str(": ");
                write_value_opts(out, val, child, sort_keys);
            }
            out.push('\n');
            pad(out, indent);
            out.push('}');
        }
    }
}

fn pad(out: &mut String, n: usize) {
    for _ in 0..n {
        out.push(' ');
    }
}

/// Escape a string the way CPython's json encoder does with `ensure_ascii=True`:
/// the JSON control escapes, then `\uXXXX` for any character >= 0x7F (and for
/// remaining control characters < 0x20).
fn write_string(out: &mut String, s: &str) {
    out.push('"');
    for ch in s.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\u{0008}' => out.push_str("\\b"),
            '\u{000C}' => out.push_str("\\f"),
            c if (c as u32) < 0x20 => {
                let _ = write!(out, "\\u{:04x}", c as u32);
            }
            c if (c as u32) < 0x7F => out.push(c),
            c => {
                // ensure_ascii: emit \uXXXX (surrogate pair for astral planes).
                let cp = c as u32;
                if cp <= 0xFFFF {
                    let _ = write!(out, "\\u{:04x}", cp);
                } else {
                    let v = cp - 0x10000;
                    let hi = 0xD800 + (v >> 10);
                    let lo = 0xDC00 + (v & 0x3FF);
                    let _ = write!(out, "\\u{:04x}\\u{:04x}", hi, lo);
                }
            }
        }
    }
    out.push('"');
}

/// Extract the JSON array/object span out of a `decomp_dbg` console transcript.
///
/// Port of `kuna/catalog.py::_extract_json`: drop the fixed `[decomp]>` prompt
/// (whose `[`/`]` are not JSON), scan to the first `[`/`{`, then balance
/// brackets ignoring those inside string literals, and return the spanned text.
pub fn extract_json_span(transcript: &str) -> Option<&str> {
    // The prompt's brackets are not JSON; remove only the exact prompt token so
    // byte offsets into the cleaned string still map onto real JSON.  We operate
    // on the cleaned owned string only for scanning, then re-find the span in the
    // original by content — simpler: clean into an owned String and return owned.
    // To keep a &str return, we instead scan the original but skip the literal
    // "[decomp]>" prompt occurrences.
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
        let mut s = String::new();
        write_string(&mut s, match &v {
            Json::Str(x) => x,
            _ => unreachable!(),
        });
        assert_eq!(s, "\"\\u26a0\\ufe0f opt-in\"");
    }
}
