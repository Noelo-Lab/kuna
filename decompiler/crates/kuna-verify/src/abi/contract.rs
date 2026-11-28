//! Typed contracts: the function's real return type, read from C source.
//!
//! Only what the return comparison needs is parsed: the declared return type of
//! the named function, through the file's typedefs.

use std::collections::BTreeMap;

use regex::Regex;

/// The sizes of C's integer types on the target.
#[derive(Debug, Clone, Copy)]
pub struct CSizes {
    pub int: u32,
    pub long: u32,
    pub long_long: u32,
    pub pointer: u32,
    /// Plain `char` is signed: x86 yes; ARM, AArch64, PowerPC, RISC-V and
    /// s390 System V ABIs no. Spell `signed char` or `unsigned char` where a
    /// target's compiler differs (Apple's and Windows' ARM ABIs sign it).
    pub char_signed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CKind {
    Integer,
    Boolean,
    Pointer,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CType {
    /// The type as written in the source.
    pub spelling: String,
    /// `0` for `void`.
    pub size: u32,
    pub signed: bool,
    pub kind: CKind,
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

/// A contract that names the compiler's `-fsigned-char` or
/// `-funsigned-char` (in a comment) overrides the target's plain `char`.
fn char_override(src: &str, sizes: CSizes) -> CSizes {
    let signed = src.contains("-fsigned-char");
    let unsigned = src.contains("-funsigned-char");
    match (signed, unsigned) {
        (true, false) => CSizes { char_signed: true, ..sizes },
        (false, true) => CSizes { char_signed: false, ..sizes },
        _ => sizes,
    }
}

fn strip_comments(src: &str) -> String {
    let block = Regex::new(r"(?s)/\*.*?\*/").expect("regex");
    let line = Regex::new(r"//[^\n]*").expect("regex");
    line.replace_all(&block.replace_all(src, " "), "").into_owned()
}

/// Typedefs by name. A name defined twice with different types (one per
/// preprocessor branch, say) maps to `None`: which one the binary was built
/// with is not known here.
fn typedefs(src: &str) -> BTreeMap<String, Option<String>> {
    let re = Regex::new(r"typedef\s+([^;{}()]+?)\s+(\w+)\s*;").expect("regex");
    let mut defs: BTreeMap<String, Option<String>> = BTreeMap::new();
    for c in re.captures_iter(src) {
        let ty = c[1].split_whitespace().collect::<Vec<_>>().join(" ");
        defs.entry(c[2].to_string())
            .and_modify(|d| {
                if d.as_deref() != Some(ty.as_str()) {
                    *d = None;
                }
            })
            .or_insert(Some(ty));
    }
    defs
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
    let plain_char = has("char") && !unsigned && !has("signed");
    Some((size, if plain_char { sizes.char_signed } else { !unsigned }))
}

/// The size, signedness and kind of a spelled type, through typedefs; an error
/// names a typedef defined more than one way.
fn resolve(spelling: &str, defs: &BTreeMap<String, Option<String>>, sizes: CSizes, depth: u32) -> Result<Option<(u32, bool, CKind)>, String> {
    if spelling.contains('*') {
        return Ok(Some((sizes.pointer, false, CKind::Pointer)));
    }
    let words: Vec<&str> = spelling
        .split_whitespace()
        .filter(|w| !matches!(*w, "const" | "volatile" | "static" | "inline" | "extern" | "register"))
        .collect();
    if words.len() == 1 && depth < 8 {
        match defs.get(words[0]) {
            Some(Some(inner)) => return resolve(inner, defs, sizes, depth + 1),
            Some(None) => return Err(format!("typedef `{}` is defined more than one way", words[0])),
            None => {}
        }
    }
    if matches!(words.as_slice(), ["_Bool" | "bool"]) {
        return Ok(Some((1, false, CKind::Boolean)));
    }
    Ok(base_type(&words, sizes).map(|(size, signed)| (size, signed, CKind::Integer)))
}

/// Read `function`'s return type from C source.
pub fn from_source(src: &str, function: &str, source: &str, sizes: CSizes) -> Result<Contract, String> {
    let sizes = char_override(src, sizes);
    let clean = strip_comments(src);
    let defs = typedefs(&clean);
    let re = Regex::new(&format!(
        r"(?m)(^|[;{{}}])\s*((?:[A-Za-z_]\w*[\s*]+)+?)\b{}\s*\(([^)]*)\)",
        regex::escape(function)
    ))
    .expect("regex");
    let found: Vec<_> = re
        .captures_iter(&clean)
        .filter(|c| !c[2].split_whitespace().any(|w| matches!(w, "return" | "else" | "case" | "goto")))
        .collect();
    let caps = found.first().ok_or_else(|| format!("{source}: no declaration of {function}"))?;
    let words = |c: &regex::Captures| c[2].split_whitespace().filter(|w| !matches!(*w, "static" | "inline" | "extern")).collect::<Vec<_>>().join(" ");
    let spelling = caps[2].split_whitespace().collect::<Vec<_>>().join(" ");
    let first = words(caps);
    if let Some(other) = found.iter().map(words).find(|o| *o != first) {
        return Err(format!("{source}: {function} is declared returning both `{first}` and `{other}`"));
    }
    let (size, signed, kind) = resolve(&spelling, &defs, sizes, 0)
        .map_err(|e| format!("{source}: return type of {function}: {e}"))?
        .ok_or_else(|| format!("{source}: cannot size return type `{spelling}` of {function}"))?;
    let declaration = format!("{} {}({})", spelling, function, caps[3].split_whitespace().collect::<Vec<_>>().join(" "));
    Ok(Contract { ret: CType { spelling, size, signed, kind }, declaration, source: source.to_string() })
}

#[cfg(test)]
mod tests {
    use super::*;

    const ARM: CSizes = CSizes { int: 4, long: 4, long_long: 8, pointer: 4, char_signed: false };

    #[test]
    fn return_types_resolve_through_typedefs() {
        let src = "typedef unsigned long long u64;\n/* u32 f() */\nu64 ins16(unsigned a, unsigned short b) { return 0; }\n";
        let c = from_source(src, "ins16", "ins16.c", ARM).unwrap();
        assert_eq!(c.ret, CType { spelling: "u64".into(), size: 8, signed: false, kind: CKind::Integer });
        assert_eq!(c.declaration, "u64 ins16(unsigned a, unsigned short b)");
    }

    #[test]
    fn builtin_and_pointer_returns() {
        let src = "static int f(void);\nconst char *g(int x);\nvoid h(void) {}\nlong k(void);";
        assert_eq!(from_source(src, "f", "s", ARM).unwrap().ret.size, 4);
        assert!(from_source(src, "f", "s", ARM).unwrap().ret.signed);
        assert_eq!(from_source(src, "g", "s", ARM).unwrap().ret.size, 4);
        assert_eq!(from_source(src, "g", "s", ARM).unwrap().ret.kind, CKind::Pointer);
        assert_eq!(from_source(src, "f", "s", ARM).unwrap().ret.kind, CKind::Integer);
        assert!(from_source(src, "h", "s", ARM).unwrap().ret.is_void());
        assert_eq!(from_source(src, "k", "s", ARM).unwrap().ret.size, 4);
        assert!(from_source(src, "missing", "s", ARM).is_err());
        let wide = "unsigned __int128 m(unsigned long a);\nNI unsigned q(int x);";
        assert_eq!(from_source(wide, "m", "s", ARM).unwrap().ret.size, 16);
        assert!(from_source(wide, "q", "s", ARM).is_err());
    }

    /// A typedef each preprocessor branch defines its own way, or a function
    /// declared with two return types, gives no contract: which one the binary
    /// was built with is not known.
    #[test]
    fn ambiguous_return_types_are_refused() {
        let src = "#ifdef TARGET_64\ntypedef uint64_t result_t;\n#else\ntypedef uint32_t result_t;\n#endif\nresult_t f(void);\n";
        let e = from_source(src, "f", "s", ARM).unwrap_err();
        assert!(e.contains("typedef `result_t` is defined more than one way"), "{e}");
        let same = "typedef unsigned int u32;\ntypedef unsigned  int u32;\nu32 g(void);\n";
        assert_eq!(from_source(same, "g", "s", ARM).unwrap().ret.size, 4);
        let two = "#if A\nlong h(void);\n#else\nshort h(void);\n#endif\n";
        assert!(from_source(two, "h", "s", ARM).unwrap_err().contains("declared returning both"));
        let decl_and_def = "static int k(void);\nint k(void) { return 1; }\n";
        assert_eq!(from_source(decl_and_def, "k", "s", ARM).unwrap().ret.size, 4);
    }

    /// Plain `char` takes the target's signedness; spelled out, it does not.
    #[test]
    fn plain_char_follows_the_target() {
        let src = "char a(void);
signed char b(void);
unsigned char c(void);";
        let x86 = CSizes { char_signed: true, ..ARM };
        assert!(!from_source(src, "a", "s", ARM).unwrap().ret.signed);
        assert!(from_source(src, "a", "s", x86).unwrap().ret.signed);
        assert!(from_source(src, "b", "s", ARM).unwrap().ret.signed);
        assert!(!from_source(src, "c", "s", x86).unwrap().ret.signed);
        let flagged = format!("/* built with -fsigned-char */\n{src}");
        assert!(from_source(&flagged, "a", "s", ARM).unwrap().ret.signed);
        let flagged = format!("// -funsigned-char\n{src}");
        assert!(!from_source(&flagged, "a", "s", x86).unwrap().ret.signed);
    }
}
