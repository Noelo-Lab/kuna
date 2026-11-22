//! Rust identifiers for functional machine operations.

const FALLBACK_PREFIX: &str = "__kuna_userop_";

pub(crate) fn operator_name(name: Option<&[u8]>, index: u64) -> String {
    let name = String::from_utf8_lossy(name.unwrap_or_default());
    if raw_keyword(&name) {
        return format!("r#{name}");
    }
    if is_ascii_identifier(&name) && !name.starts_with(FALLBACK_PREFIX) {
        return name.into_owned();
    }

    let mut fallback = format!("{FALLBACK_PREFIX}{index:x}");
    if !name.is_empty() {
        fallback.push('_');
        fallback.extend(name.chars().take(64).map(
            |c| {
                if c.is_ascii_alphanumeric() {
                    c
                } else {
                    '_'
                }
            },
        ));
    }
    fallback
}

fn is_ascii_identifier(name: &str) -> bool {
    if matches!(name, "_" | "crate" | "self" | "Self" | "super") {
        return false;
    }
    let mut chars = name.bytes();
    chars
        .next()
        .is_some_and(|c| c == b'_' || c.is_ascii_alphabetic())
        && chars.all(|c| c == b'_' || c.is_ascii_alphanumeric())
}

pub(crate) fn raw_keyword(name: &str) -> bool {
    matches!(
        name,
        "as" | "async" | "await" | "break" | "const" | "continue" | "dyn" | "else"
            | "enum" | "extern" | "false" | "fn" | "for" | "if" | "impl" | "in"
            | "let" | "loop" | "match" | "mod" | "move" | "mut" | "pub" | "ref"
            | "return" | "static" | "struct" | "trait" | "true" | "type" | "unsafe"
            | "use" | "where" | "while" | "abstract" | "become" | "box" | "do"
            | "final" | "gen" | "macro" | "override" | "priv" | "try" | "typeof"
            | "unsized" | "virtual" | "yield"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rust_spellings_preserve_identifiers_and_escape_unrepresentable_names() {
        for name in ["input", "union", "_name", "name42"] {
            assert_eq!(operator_name(Some(name.as_bytes()), 0xa), name);
        }
        assert_eq!(operator_name(Some(b"in"), 0xa), "r#in");
        assert_eq!(operator_name(Some(b"gen"), 0xa), "r#gen");
        assert_eq!(operator_name(Some(b"r#in"), 0xa), "__kuna_userop_a_r_in");
        assert_eq!(operator_name(Some(b""), 0xa), "__kuna_userop_a");
        assert_eq!(operator_name(Some(b"\xff"), 0xa), "__kuna_userop_a__");
        assert_eq!(operator_name(Some(b"2name"), 0xa), "__kuna_userop_a_2name");
        for name in ["_", "crate", "self", "Self", "super"] {
            assert_eq!(
                operator_name(Some(name.as_bytes()), 0xa),
                format!("__kuna_userop_a_{name}")
            );
        }
        for joiner in ['\u{200c}', '\u{200d}'] {
            let long_name = format!("a{joiner}{}", "é".repeat(80));
            assert_eq!(
                operator_name(Some(long_name.as_bytes()), 0xa),
                format!("__kuna_userop_a_a{}", "_".repeat(63))
            );
        }
    }

    #[test]
    fn unicode_names_keep_their_operation_identity_after_rust_normalization() {
        assert_eq!(operator_name(Some("é".as_bytes()), 0), "__kuna_userop_0__");
        assert_eq!(operator_name(Some("e\u{301}".as_bytes()), 1), "__kuna_userop_1_e_");
        assert_eq!(operator_name(Some("ação".as_bytes()), 2), "__kuna_userop_2_a__o");
        assert_eq!(operator_name(Some("東京".as_bytes()), 3), "__kuna_userop_3___");
        assert_eq!(operator_name(Some("_я".as_bytes()), 4), "__kuna_userop_4___");
    }

    #[test]
    fn raw_spelling_is_limited_to_representable_keywords() {
        for name in ["in", "break", "yield", "async", "gen", "type"] {
            assert!(raw_keyword(name));
        }
        for name in [
            "input", "breakpoint", "yield_now", "in_", "r#in", "CALLOTHER[0x1]",
            "union", "raw", "safe", "_", "crate", "self", "Self", "super",
        ] {
            assert!(!raw_keyword(name));
        }
    }
}
