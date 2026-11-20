//! Typed contracts: the function's real return type, read from C source.
//!
//! Only what the return comparison needs is parsed: the declared return type of
//! the named function, through the file's typedefs, as a size and signedness.

use std::collections::BTreeMap;

use regex::Regex;

/// The sizes of C's integer types on the target.
#[derive(Debug, Clone, Copy)]
pub struct CSizes {
    pub int: u32,
    pub long: u32,
    pub long_long: u32,
    pub pointer: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CType {
    /// The type as written in the source.
    pub spelling: String,
    /// `0` for `void`.
    pub size: u32,
    pub signed: bool,
    /// A pointer, which some conventions return in an address register.
    pub pointer: bool,
}

impl CType {
    pub fn is_void(&self) -> bool {
        self.size == 0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Contract {
    pub ret: CType,
    /// The declaration line the contract was read from.
    pub declaration: String,
    pub source: String,
}

fn strip_comments(src: &str) -> String {
    let block = Regex::new(r"(?s)/\*.*?\*/").expect("regex");
    let line = Regex::new(r"//[^\n]*").expect("regex");
    line.replace_all(&block.replace_all(src, " "), "").into_owned()
}

fn typedefs(src: &str) -> BTreeMap<String, String> {
    let re = Regex::new(r"typedef\s+([^;{}()]+?)\s+(\w+)\s*;").expect("regex");
    re.captures_iter(src).map(|c| (c[2].to_string(), c[1].trim().to_string())).collect()
}

fn base_type(words: &[&str], sizes: CSizes) -> Option<(u32, bool)> {
    let has = |w: &str| words.contains(&w);
    let unsigned = has("unsigned");
    if has("__int128") {
        return Some((16, !unsigned));
    }
    let longs = words.iter().filter(|&&w| w == "long").count();
    let fixed = |w: &str| -> Option<(u32, bool)> {
        Some(match w {
            "void" => (0, false),
            "_Bool" | "bool" => (1, false),
            "int8_t" => (1, true),
            "uint8_t" => (1, false),
            "int16_t" => (2, true),
            "uint16_t" => (2, false),
            "int32_t" => (4, true),
            "uint32_t" => (4, false),
            "int64_t" => (8, true),
            "uint64_t" => (8, false),
            "__int128_t" => (16, true),
            "__uint128_t" => (16, false),
            "size_t" | "uintptr_t" => (sizes.pointer, false),
            "ssize_t" | "intptr_t" | "ptrdiff_t" => (sizes.pointer, true),
            _ => return None,
        })
    };
    if let Some(found) = words.iter().find_map(|w| fixed(w)) {
        return Some(found);
    }
    if words.iter().any(|w| !matches!(*w, "unsigned" | "signed" | "long" | "short" | "int" | "char")) {
        return None;
    }
    let size = if has("char") {
        1
    } else if has("short") {
        2
    } else if longs >= 2 {
        sizes.long_long
    } else if longs == 1 {
        sizes.long
    } else if has("int") || unsigned || has("signed") {
        sizes.int
    } else {
        return None;
    };
    Some((size, !unsigned))
}

/// `(size, signed, pointer)` of a spelled type, through typedefs.
fn resolve(spelling: &str, defs: &BTreeMap<String, String>, sizes: CSizes, depth: u32) -> Option<(u32, bool, bool)> {
    if spelling.contains('*') {
        return Some((sizes.pointer, false, true));
    }
    let words: Vec<&str> = spelling
        .split_whitespace()
        .filter(|w| !matches!(*w, "const" | "volatile" | "static" | "inline" | "extern" | "register"))
        .collect();
    if words.len() == 1 && depth < 8 {
        if let Some(inner) = defs.get(words[0]) {
            return resolve(inner, defs, sizes, depth + 1);
        }
    }
    base_type(&words, sizes).map(|(size, signed)| (size, signed, false))
}

/// Read `function`'s return type from C source.
pub fn from_source(src: &str, function: &str, source: &str, sizes: CSizes) -> Result<Contract, String> {
    let clean = strip_comments(src);
    let defs = typedefs(&clean);
    let re = Regex::new(&format!(
        r"(?m)(^|[;{{}}])\s*((?:[A-Za-z_]\w*[\s*]+)+?)\b{}\s*\(([^)]*)\)",
        regex::escape(function)
    ))
    .expect("regex");
    let caps = re
        .captures_iter(&clean)
        .find(|c| !c[2].split_whitespace().any(|w| matches!(w, "return" | "else" | "case" | "goto")))
        .ok_or_else(|| format!("{source}: no declaration of {function}"))?;
    let spelling = caps[2].trim().to_string();
    let (size, signed, pointer) = resolve(&spelling, &defs, sizes, 0)
        .ok_or_else(|| format!("{source}: cannot size return type `{spelling}` of {function}"))?;
    let declaration = format!("{} {}({})", spelling, function, caps[3].split_whitespace().collect::<Vec<_>>().join(" "));
    Ok(Contract { ret: CType { spelling, size, signed, pointer }, declaration, source: source.to_string() })
}

#[cfg(test)]
mod tests {
    use super::*;

    const ARM: CSizes = CSizes { int: 4, long: 4, long_long: 8, pointer: 4 };

    #[test]
    fn return_types_resolve_through_typedefs() {
        let src = "typedef unsigned long long u64;\n/* u32 f() */\nu64 ins16(unsigned a, unsigned short b) { return 0; }\n";
        let c = from_source(src, "ins16", "ins16.c", ARM).unwrap();
        assert_eq!(c.ret, CType { spelling: "u64".into(), size: 8, signed: false, pointer: false });
        assert_eq!(c.declaration, "u64 ins16(unsigned a, unsigned short b)");
    }

    #[test]
    fn builtin_and_pointer_returns() {
        let src = "static int f(void);\nconst char *g(int x);\nvoid h(void) {}\nlong k(void);";
        assert_eq!(from_source(src, "f", "s", ARM).unwrap().ret.size, 4);
        assert!(from_source(src, "f", "s", ARM).unwrap().ret.signed);
        assert_eq!(from_source(src, "g", "s", ARM).unwrap().ret.size, 4);
        assert!(from_source(src, "g", "s", ARM).unwrap().ret.pointer);
        assert!(!from_source(src, "f", "s", ARM).unwrap().ret.pointer);
        assert!(from_source(src, "h", "s", ARM).unwrap().ret.is_void());
        assert_eq!(from_source(src, "k", "s", ARM).unwrap().ret.size, 4);
        assert!(from_source(src, "missing", "s", ARM).is_err());
        let wide = "unsigned __int128 m(unsigned long a);\nNI unsigned q(int x);";
        assert_eq!(from_source(wide, "m", "s", ARM).unwrap().ret.size, 16);
        assert!(from_source(wide, "q", "s", ARM).is_err());
    }
}
