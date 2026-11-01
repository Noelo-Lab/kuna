//! Raw Rust identifiers for keyword-named machine operations.

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
