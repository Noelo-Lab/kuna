//! JSON layout and ASCII-safe escaping shared by all CLI output modes.

use super::Json;
use std::fmt::{self, Write as _};

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

/// How much of a streamed document is buffered before it is written.
const STREAM_CHUNK: usize = 1 << 16;

/// [`super::write_indent2_streamed`].
pub(super) fn write_streamed(
    out: &mut dyn std::io::Write,
    head: &[(String, Json)],
    key: &str,
    items: impl Iterator<Item = Json>,
    tail: &[(String, Json)],
) -> std::io::Result<()> {
    let field = |buf: &mut String, first: bool, name: &str| {
        if !first {
            buf.push(',');
        }
        Format::Pretty.line(buf, 2);
        write_string(buf, name);
        buf.push_str(": ");
    };
    let mut buf = String::from("{");
    for (i, (name, value)) in head.iter().enumerate() {
        field(&mut buf, i == 0, name);
        write_value(&mut buf, value, 2, Format::Pretty);
    }
    field(&mut buf, head.is_empty(), key);
    buf.push('[');
    let mut any = false;
    for item in items {
        if any {
            buf.push(',');
        }
        any = true;
        Format::Pretty.line(&mut buf, 4);
        write_value(&mut buf, &item, 4, Format::Pretty);
        if buf.len() >= STREAM_CHUNK {
            out.write_all(buf.as_bytes())?;
            buf.clear();
        }
    }
    if any {
        Format::Pretty.line(&mut buf, 2);
    }
    buf.push(']');
    for (name, value) in tail {
        field(&mut buf, false, name);
        write_value(&mut buf, value, 2, Format::Pretty);
    }
    Format::Pretty.line(&mut buf, 0);
    buf.push('}');
    out.write_all(buf.as_bytes())
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

/// JSON Unicode escapes, shared by string and I/O renderers.
pub(crate) struct UnicodeEscape(pub(crate) char);

impl fmt::Display for UnicodeEscape {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        let cp = self.0 as u32;
        if cp <= 0xFFFF {
            write!(out, "\\u{cp:04x}")
        } else {
            let v = cp - 0x10000;
            let hi = 0xD800 + (v >> 10);
            let lo = 0xDC00 + (v & 0x3FF);
            write!(out, "\\u{hi:04x}\\u{lo:04x}")
        }
    }
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
                let _ = write!(out, "{}", UnicodeEscape(c));
            }
            c if (c as u32) < 0x7F => out.push(c),
            c => {
                let _ = write!(out, "{}", UnicodeEscape(c));
            }
        }
    }
    out.push('"');
}
