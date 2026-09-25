//! The text syntax of the `--assert` override plane: one directive per line,
//! `@FILE` for a file of them, and the `--define-function` boundary spelling
//! the `function` directive shares. Shared by the `kuna` CLI and the in-browser
//! front-end; the vocabulary and where each directive lands live in
//! [`crate::assertions`].

use crate::assertions::{Body, Directive};

/// One caller-declared function boundary: `START[-END][=NAME]`, `END` exclusive.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FunctionSpec {
    pub start: u64,
    pub end: Option<u64>,
    pub name: Option<String>,
}

/// Parse one `--define-function` value: either a declaration or `@FILE`, whose
/// lines are declarations with blank lines and `#` comments skipped.
pub fn parse_function_flag(value: &str) -> Result<Vec<FunctionSpec>, String> {
    let Some(path) = value.strip_prefix('@') else {
        return Ok(vec![parse_function_spec(value)?]);
    };
    let text = std::fs::read_to_string(path)
        .map_err(|e| format!("--define-function @{path}: {e}"))?;
    let mut out = Vec::new();
    for (n, line) in text.lines().enumerate() {
        let line = line.split('#').next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }
        out.push(parse_function_spec(line).map_err(|e| format!("{path}:{}: {e}", n + 1))?);
    }
    Ok(out)
}

/// Parse `START[-END][=NAME]`, hex with or without a `0x` prefix.
pub fn parse_function_spec(spec: &str) -> Result<FunctionSpec, String> {
    let spec = spec.trim();
    let (range, name) = match spec.split_once('=') {
        Some((range, name)) if !name.trim().is_empty() => (range.trim(), Some(name.trim())),
        Some((range, _)) => (range.trim(), None),
        None => (spec, None),
    };
    // Split on the LAST '-': a range separator can never be the first character
    // of a hex literal, but `0x-` cannot occur either, so the last one is the
    // separator whenever there is a non-empty tail after it.
    let (start, end) = match range.rsplit_once('-') {
        Some((start, end)) if !start.trim().is_empty() && !end.trim().is_empty() => {
            (start.trim(), Some(end.trim()))
        }
        _ => (range, None),
    };
    let start = parse_vma(start)
        .ok_or_else(|| format!("--define-function {spec:?}: {start:?} is not a hex address"))?;
    let end = match end {
        None => None,
        Some(end) => Some(
            parse_vma(end)
                .ok_or_else(|| format!("--define-function {spec:?}: {end:?} is not a hex address"))?,
        ),
    };
    if let Some(end) = end {
        if end <= start {
            return Err(format!(
                "--define-function {spec:?}: end {end:#x} must be above start {start:#x}"
            ));
        }
    }
    Ok(FunctionSpec { start, end, name: name.map(str::to_string) })
}

/// Parse one `--assert` value: either a directive or `@FILE`.
pub fn parse_directive_flag(value: &str) -> Result<Vec<Directive>, String> {
    let Some(path) = value.strip_prefix('@') else {
        return Ok(vec![parse_directive(value)?]);
    };
    let text = std::fs::read_to_string(path).map_err(|e| format!("--assert @{path}: {e}"))?;
    let mut out = Vec::new();
    for (n, line) in text.lines().enumerate() {
        let line = strip_comment(line);
        if line.is_empty() {
            continue;
        }
        out.push(parse_directive(line).map_err(|e| format!("{path}:{}: {e}", n + 1))?);
    }
    Ok(out)
}

/// A `#` starts a comment, so an agent can annotate what it worked out.  Only at
/// the start of a token: a `#` inside a `comment` directive's text is text.
fn strip_comment(line: &str) -> &str {
    match line.find(" #") {
        Some(at) => line[..at].trim(),
        None => line.trim().strip_prefix('#').map(|_| "").unwrap_or(line.trim()),
    }
}

/// Split `<func>::<operand>` into its two halves.  Unqualified means "the
/// function under decompile"; a C++ name is split at its LAST `::`, so
/// `ns::cls::fn::v2` qualifies `v2` with `ns::cls::fn`.
fn split_qualifier(tok: &str) -> (Option<String>, String) {
    match tok.rsplit_once("::") {
        Some((func, operand)) if !func.is_empty() && !operand.is_empty() => {
            (Some(func.to_string()), operand.to_string())
        }
        _ => (None, tok.to_string()),
    }
}

/// Normalise a storage operand to the console's machine-address syntax.
///
/// Storage is spelled the way the console spells it — `%RDI`, `[register,0x38,8]`,
/// `[stack,-0x18,8]`, `s0x10` — because that is the grammar the engine parses
/// (`parse_machaddr`).  A bare register NAME is the one spelling an agent will
/// reach for and the one the console rejects outright (`RDI` reads as the space
/// shortcut `R`, "Bad address: R"), so it is rewritten to `%RDI` here: a token
/// that starts with a letter and carries no `0x` cannot be any of the other
/// forms.
fn normalize_storage(tok: &str) -> String {
    let bare_name = tok.starts_with(|c: char| c.is_ascii_alphabetic())
        && !tok.contains("0x")
        && !tok.contains("0X")
        && tok.chars().all(|c| c.is_ascii_alphanumeric() || c == '_');
    if bare_name {
        format!("%{tok}")
    } else {
        tok.to_string()
    }
}

/// A bare or `0x`-prefixed hexadecimal VMA (the `--define-function` convention).
fn parse_vma(tok: &str) -> Option<u64> {
    let tok = tok.trim();
    let body = tok.strip_prefix("0x").or_else(|| tok.strip_prefix("0X")).unwrap_or(tok);
    if body.is_empty() || !body.chars().all(|c| c.is_ascii_hexdigit()) {
        return None;
    }
    u64::from_str_radix(body, 16).ok()
}

/// A memory range: `<addr>+<size>` (canonical) or `<addr> <size>`.  The address
/// is hex with or without `0x`; the size is decimal unless it carries a `0x`.
fn parse_range(rest: &str) -> Option<(u64, i32)> {
    let (addr_tok, size_tok) = match rest.split_once('+') {
        Some((a, b)) => (a, b),
        None => {
            let (a, b) = take_token(rest);
            (a, b)
        }
    };
    let addr = parse_vma(addr_tok)?;
    let size_tok = size_tok.trim();
    if size_tok.is_empty() || size_tok.split_whitespace().count() != 1 {
        return None;
    }
    let size = match size_tok.strip_prefix("0x").or_else(|| size_tok.strip_prefix("0X")) {
        Some(hex) => u64::from_str_radix(hex, 16).ok()?,
        None => size_tok.parse::<u64>().ok()?,
    };
    if size == 0 || size > i32::MAX as u64 {
        return None;
    }
    Some((addr, size as i32))
}

/// Split off the leading whitespace-delimited token, returning `(token, rest)`.
fn take_token(s: &str) -> (&str, &str) {
    let s = s.trim_start();
    match s.find(char::is_whitespace) {
        Some(at) => (&s[..at], s[at..].trim_start()),
        None => (s, ""),
    }
}

/// Parse one directive line.
pub fn parse_directive(spec: &str) -> Result<Directive, String> {
    let raw = spec.trim().to_string();
    let (keyword, rest) = take_token(&raw);
    // The WHOLE directive is one argument. Unquoted,
    //   --assert prototype main int main(void)
    // hands us just "prototype", `rest` comes back empty, and the old message
    // ("prototype needs <func> then a C declaration") named the CONTENT we wanted while
    // saying nothing about the shape -- so when the spec is a bare keyword, say what is
    // actually wrong and show the fix.
    // This comment used to blame five round-3 testers for writing it unquoted and to
    // claim the type rejection they filed did not exist. Both halves were wrong (#418):
    // their directives were correctly quoted, and `int` really was rejected, because the
    // C-declaration grammar knew only Ghidra's `int4` vocabulary. That is the defect
    // `grammar.rs (CParse::scalar_specifier)` closes; this hint is the unrelated half.
    let bad = |what: &str| {
        if rest.is_empty() && !raw.is_empty() {
            format!(
                "--assert {raw:?}: {what}\n\
                 hint: the whole assertion is ONE argument -- quote it. You probably wrote\n\
                 \x20      --assert {kw} <rest of the declaration>\n\
                 \x20  and the shell split it. Write:\n\
                 \x20      --assert \"{kw} <rest of the declaration>\"\n\
                 \x20  e.g. --assert \"prototype main int main(int argc, char **argv)\"",
                kw = keyword
            )
        } else {
            format!("--assert {raw:?}: {what}")
        }
    };
    let body = match keyword {
        "function" => {
            let decls = parse_function_flag(rest)
                .map_err(|e| format!("--assert {raw:?}: {}", e.trim_start_matches("--define-function ")))?;
            let decl = decls.into_iter().next().ok_or_else(|| bad("missing <start>"))?;
            Body::Function { start: decl.start, end: decl.end, name: decl.name }
        }
        "typedef" => {
            if rest.is_empty() {
                return Err(bad("typedef needs a C declaration"));
            }
            Body::Typedef { decl: rest.to_string() }
        }
        "prototype" => {
            let (func, decl) = take_token(rest);
            if func.is_empty() || decl.is_empty() {
                return Err(bad("prototype needs <func> then a C declaration"));
            }
            Body::Prototype { func: func.to_string(), decl: decl.to_string() }
        }
        "data" => {
            let (addr, decl) = take_token(rest);
            let addr = parse_vma(addr).ok_or_else(|| bad("data needs a hex <addr>"))?;
            if decl.is_empty() {
                return Err(bad("data needs a C type declaration naming the symbol"));
            }
            Body::Data { addr, decl: decl.to_string() }
        }
        "param" => {
            let (index, rest) = take_token(rest);
            let (func, index) = split_qualifier(index);
            let index: i32 = index.parse().map_err(|_| bad("param needs a decimal index"))?;
            let (storage, decl) = take_token(rest);
            if storage.is_empty() || decl.is_empty() {
                return Err(bad("param needs <i> <storage> then a C type declaration"));
            }
            Body::Param {
                func,
                index,
                storage: normalize_storage(storage),
                decl: decl.to_string(),
            }
        }
        "return" => {
            let (storage, decl) = take_token(rest);
            let (func, storage) = split_qualifier(storage);
            if storage.is_empty() || decl.is_empty() {
                return Err(bad("return needs <storage> then a C type declaration"));
            }
            Body::Return { func, storage: normalize_storage(&storage), decl: decl.to_string() }
        }
        "comment" => {
            let (addr, text) = take_token(rest);
            let (func, addr) = split_qualifier(addr);
            let addr = parse_vma(&addr).ok_or_else(|| bad("comment needs a hex <addr>"))?;
            if text.is_empty() {
                return Err(bad("comment needs text"));
            }
            Body::Comment { func, addr, text: text.to_string() }
        }
        "name" => {
            let (symbol, newname) = take_token(rest);
            let (func, symbol) = split_qualifier(symbol);
            let (newname, tail) = take_token(newname);
            if symbol.is_empty() || newname.is_empty() {
                return Err(bad("name needs <symbol> <newname>"));
            }
            if !tail.is_empty() {
                return Err(bad("name takes exactly <symbol> <newname>"));
            }
            Body::Name { func, symbol, newname: newname.to_string() }
        }
        "type" => {
            let (symbol, decl) = take_token(rest);
            let (func, symbol) = split_qualifier(symbol);
            if symbol.is_empty() || decl.is_empty() {
                return Err(bad("type needs <symbol> then a C type"));
            }
            Body::Type { func, symbol, decl: decl.to_string() }
        }
        "readonly" | "volatile" => {
            let (addr, size) = parse_range(rest)
                .ok_or_else(|| bad("needs <addr>+<size> (e.g. 0x404028+8)"))?;
            if keyword == "readonly" {
                Body::Readonly { addr, size }
            } else {
                Body::Volatile { addr, size }
            }
        }
        "bytes" => {
            let (addr, payload) = take_token(rest);
            let addr = parse_vma(addr).ok_or_else(|| bad("bytes needs a hex <addr>"))?;
            let (payload, tail) = take_token(payload);
            if payload.is_empty() {
                return Err(bad("bytes needs <addr> then hex, or @FILE of raw bytes"));
            }
            if !tail.is_empty() {
                return Err(bad("bytes takes exactly <addr> <hex|@FILE>"));
            }
            let data = match payload.strip_prefix('@') {
                Some(path) => std::fs::read(path)
                    .map_err(|e| format!("--assert {raw:?}: {path}: {e}"))?,
                None => crate::assertions::parse_hex_bytes(payload)
                    .map_err(|e| bad(&e))?,
            };
            if data.is_empty() {
                return Err(bad("bytes needs at least one byte"));
            }
            Body::Bytes { addr, data }
        }
        "flow" => {
            let (addr, kind) = take_token(rest);
            let (func, addr) = split_qualifier(addr);
            let addr = parse_vma(&addr).ok_or_else(|| bad("flow needs a hex <addr>"))?;
            let (kind, tail) = take_token(kind);
            if !tail.is_empty() {
                return Err(bad("flow takes exactly <addr> <flowkind>"));
            }
            // The vocabulary is closed and four words wide, so a misspelling is a
            // usage error here rather than a rejected outcome three phases later.
            if !matches!(kind, "branch" | "call" | "callreturn" | "return") {
                return Err(bad(
                    "flow needs <addr> then one of branch, call, callreturn, return",
                ));
            }
            Body::Flow { func, addr, kind: kind.to_string() }
        }
        "" => return Err("--assert: empty directive".into()),
        other => {
            return Err(format!(
                "--assert {raw:?}: unknown directive {other:?} (want one of \
                 function, typedef, prototype, data, param, return, comment, flow, name, \
                 type, readonly, volatile, bytes)"
            ))
        }
    };
    Ok(Directive { raw, body })
}
