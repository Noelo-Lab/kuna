//! kuna fix `kt_string_literal_escape`: string literals in Kotlin output
//! are escaped as Kotlin strings.
//!
//! Vineflower writes a string constant with Java's escapes. In a Kotlin
//! string `$` followed by a name or `{` starts a template, so the constant
//! `"$this$read"` reads `this.toString()` and a variable `read`, and `"${x}"`
//! reads `x`; `\f` is not a Kotlin escape at all. In a string template the
//! plugin escapes the `$` of each constant piece but splices a `Char`
//! constant in raw, so `'"'`, `'$'` or a newline end or change the literal.
//! A variable is spliced in as `$name`, even when the next piece starts with
//! a letter or digit, so `"$key258EAF"` reads a variable `key258EAF`. With
//! the fix a `$` that would start a template is written `\$`, a form feed
//! `\u000c`, a `Char` piece of a template is escaped as a string, and a
//! variable is written `${name}` unless `$name` ends where the name does.

use crate::const_exprent::ConstExprent;
use crate::exprent::Exprent;
use crate::rt::jstring::from_utf16;
use crate::var_type::VarType;

/// `text`, a string escaped by `ConstExprent::convert_string_to_java`, as a
/// Kotlin string's text: a `$` that starts a template is escaped (`all`: every
/// `$`), and `\f` becomes `\u000c`.
pub fn kotlin_text(text: &str, all: bool) -> String {
    let chars: Vec<char> = text.chars().collect();
    let mut out = String::with_capacity(text.len() + 4);
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        if c == '\\' && i + 1 < chars.len() {
            if chars[i + 1] == 'f' {
                out.push_str("\\u000c");
            } else {
                out.push(c);
                out.push(chars[i + 1]);
            }
            i += 2;
            continue;
        }
        if c == '$' && (all || chars.get(i + 1).is_some_and(|n| starts_template(*n))) {
            out.push('\\');
        }
        out.push(c);
        i += 1;
    }
    out
}

fn starts_template(c: char) -> bool {
    c == '{' || c == '`' || c == '_' || c.is_alphabetic()
}

/// The text a `Char` constant piece of a string template is written as.
pub fn template_char(unit: u16, ascii: bool) -> String {
    kotlin_text(
        &ConstExprent::convert_string_to_java(&from_utf16(&[unit]), ascii),
        true,
    )
}

/// Whether the variable written `name` can be spliced in as `$name` when
/// `next` follows it in the template.
pub fn simple_reference(name: &str, next: Option<Exprent>) -> bool {
    let mut chars = name.chars();
    let plain = chars.next().is_some_and(|c| c == '_' || c.is_alphabetic())
        && chars.all(|c| c == '_' || c.is_alphanumeric());
    let backticked = name.len() > 2 && name.starts_with('`') && name.ends_with('`');
    if !plain && !backticked {
        return false;
    }
    let Some(constant) = next.and_then(|n| n.try_as_const()) else {
        return true;
    };
    let const_type = constant.get_const_type();
    let first = match constant.get_value() {
        Some(value) if VarType::VARTYPE_STRING().equals(&const_type) => {
            value.as_string().chars().next()
        }
        Some(value) if VarType::VARTYPE_CHAR().equals(&const_type) => {
            char::from_u32(value.as_int() as u32)
        }
        _ => None,
    };
    !first.is_some_and(|c| c == '_' || c.is_alphanumeric())
}
