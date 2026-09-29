//! JSON layout and ASCII-safe escaping shared by all CLI output modes.

use super::Json;
use std::fmt::Write as _;

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Format {
    Compact,
    Pretty,
    Sorted,
}

pub(super) fn render(value: &Json, format: Format) -> String {
    let mut out = String::new();
    write_value(&mut out, value, 0, format);
    out
}

impl Format {
    fn line(self, out: &mut String, indent: usize) {
        if self != Self::Compact {
            out.push('\n');
            for _ in 0..indent {
                out.push(' ');
            }
        }
    }
}

fn write_value(out: &mut String, value: &Json, indent: usize, format: Format) {
    match value {
        Json::Null => out.push_str("null"),
        Json::Bool(true) => out.push_str("true"),
        Json::Bool(false) => out.push_str("false"),
        Json::Number(number) => out.push_str(number),
        Json::Str(text) => write_string(out, text),
        Json::Array(items) => {
            out.push('[');
            for (i, item) in items.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                format.line(out, indent + 2);
                write_value(out, item, indent + 2, format);
            }
            if !items.is_empty() {
                format.line(out, indent);
            }
            out.push(']');
        }
        Json::Object(pairs) => {
            if format == Format::Sorted && pairs.len() > 1 {
                let mut ordered: Vec<_> = pairs.iter().collect();
                ordered.sort_by(|a, b| a.0.cmp(&b.0));
                write_pairs(out, ordered.into_iter(), indent, format);
            } else {
                write_pairs(out, pairs.iter(), indent, format);
            }
        }
    }
}

fn write_pairs<'a>(
    out: &mut String,
    pairs: impl ExactSizeIterator<Item = &'a (String, Json)>,
    indent: usize,
    format: Format,
) {
    let nonempty = pairs.len() != 0;
    out.push('{');
    for (i, (key, value)) in pairs.enumerate() {
        if i > 0 {
            out.push(',');
        }
        format.line(out, indent + 2);
        write_string(out, key);
        out.push(':');
        if format != Format::Compact {
            out.push(' ');
        }
        write_value(out, value, indent + 2, format);
    }
    if nonempty {
        format.line(out, indent);
    }
    out.push('}');
}

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
