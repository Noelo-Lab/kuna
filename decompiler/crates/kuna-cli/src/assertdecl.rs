//! `--assert` — the text syntax of the override plane.
//!
//! # Why this exists
//!
//! `rename`, `retype`, `map param`, `map return`, `map address`,
//! `comment instruction` and `parse line extern` are all functional in the
//! console, and until this flag not one of them was reachable from the `kuna`
//! binary: the generated console script emitted only `option`, `kassert`,
//! `function bounds` and `decompile`.  For an agent that is the difference
//! between reading a decompilation once and actually working through one — a
//! rename it cannot state is a rename it does not have
//! (`docs/re-needs/no-cli-rename-or-prototype-override.md`).
//!
//! # The flag
//!
//! ```text
//!   --assert 'prototype authenticate int authenticate(char *user,char *pass)'
//!   --assert 'type v2 char[16]'
//!   --assert 'name v2 credbuf'
//!   --assert 'readonly 0x404028+8'
//!   --assert 'flow 0x1405 return'
//!   --assert 'volatile 0x50000000+4'
//!   --assert @notes/overrides.kuna
//! ```
//!
//! Repeatable, and `@FILE` holds one directive per line with `#` comments — the
//! `--define-function` file contract verbatim, because that is what makes an
//! override durable across invocations.  kuna does not write assertions back
//! into the image: the file is the artifact, and it is plain text an agent can
//! generate, diff and version.
//!
//! This module owns the SYNTAX only.  The vocabulary, where each directive lands
//! in the pipeline and what "applied" means for it live in
//! [`kuna_console::assertions`], which both the in-process surfaces and the
//! generated console script go through.
//!
//! # Console lowering
//!
//! `kuna decompile`'s text surface drives `decomp_dbg`, so each directive also
//! has to be expressible as console lines ([`ConsoleForm`]).  The two surfaces
//! emit the same facts in the same order; they differ only in whether the engine
//! is reached in-process or through a script.

use kuna_console::assertions::{Body, Directive};

/// Where a directive's console line goes in the generated script.
///
/// The slots are forced, not stylistic: a `readonly` range must precede `read
/// symbols` (the symbols absorb the property as they are mapped), an `option`
/// must precede it too (the analysis commit), a `map param` needs a loaded
/// function, and a `rename` of a local needs a function that has already been
/// decompiled — before that the console answers `No symbol named: v2`, which is
/// exactly the bug that makes today's `--kassert p9 naming-policy` a silent
/// no-op.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Slot {
    /// After `load file` and the `option` lines, before `read symbols`.  A range
    /// property has to be painted before the symbols over it are mapped: the
    /// mapping folds the property into each `SymbolEntry` and never consults the
    /// range again.
    Image,
    /// After `read symbols`, before `load function`.
    Program,
    /// After `load function`, before the first `decompile`.
    Function,
    /// After the first `decompile` (forcing a second one).
    Symbol,
}

/// One directive rendered for the console script.
#[derive(Debug)]
pub(crate) struct ConsoleForm {
    pub(crate) slot: Slot,
    pub(crate) line: String,
}

/// Parse one `--assert` value: either a directive or `@FILE`.
pub(crate) fn parse_flag(value: &str) -> Result<Vec<Directive>, String> {
    let Some(path) = value.strip_prefix('@') else {
        return Ok(vec![parse_one(value)?]);
    };
    let text = std::fs::read_to_string(path).map_err(|e| format!("--assert @{path}: {e}"))?;
    let mut out = Vec::new();
    for (n, line) in text.lines().enumerate() {
        let line = strip_comment(line);
        if line.is_empty() {
            continue;
        }
        out.push(parse_one(line).map_err(|e| format!("{path}:{}: {e}", n + 1))?);
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
pub(crate) fn parse_one(spec: &str) -> Result<Directive, String> {
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
            let decls = crate::funcdecl::parse_flag(rest)
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
                 type, readonly, volatile)"
            ))
        }
    };
    Ok(Directive { raw, body })
}

/// The function a directive names, when that is NOT the function this run
/// selected.  `target` is `None` for an `--addr` run, where the CLI does not
/// know the selected function's name until the script has run.
fn names_another(func: &Option<String>, target: Option<&str>) -> Option<String> {
    let f = func.as_deref()?;
    match target {
        Some(t) if t == f => None,
        _ => Some(f.to_string()),
    }
}

/// A qualified directive that cannot bind: it names a function this run did not
/// decompile, and its kind (a local, a comment, a flow edge) has no meaning
/// outside the function it belongs to.  Silently applying it to the SELECTED
/// function instead is what made `--assert 'param callee::0 ECX char *maze'`
/// rename the caller's parameters
/// (`docs/re-needs/qualified-parameter-assertions-modify.md`).
fn unbindable(func: &str) -> String {
    format!(
        "names {func}, which this run did not decompile (decompile {func}, or use \
         a directive that crosses functions: prototype, param, return)"
    )
}

/// The console line a directive lowers to, and the script slot it belongs in.
///
/// `target` is the function the run selected.  A `param`/`return` qualified with
/// a DIFFERENT function is a statement about that function's prototype, so it
/// lowers to the cross-function console spelling and moves to the program slot;
/// every other qualified directive that names a function this run does not
/// decompile is rejected rather than applied to the selected one.
pub(crate) fn console_form(
    d: &Directive,
    target: Option<&str>,
) -> Result<ConsoleForm, String> {
    let (slot, line) = match &d.body {
        Body::Function { start, end, name } => {
            let mut line = format!("function bounds {start:#x}");
            if let Some(end) = end {
                line.push_str(&format!(" {end:#x}"));
            }
            if let Some(name) = name {
                line.push_str(&format!(" as {name}"));
            }
            (Slot::Program, line)
        }
        Body::Typedef { decl } => (Slot::Program, format!("parse line {}", semicolon(decl))),
        // `parse line extern <decl>` binds the signature to the name INSIDE the
        // declaration, so a declaration that renames the function landed on a
        // fresh symbol and left the selected one untouched -- silently, since the
        // console reported no error (`docs/re-needs/text-output-silently-ignores.md`).
        // `map prototype` takes the target as its first token, as the in-process
        // surface always has.
        Body::Prototype { func, decl } => {
            (Slot::Program, format!("map prototype {func} {}", semicolon(decl)))
        }
        Body::Data { addr, decl } => (Slot::Program, format!("map address {addr:#x} {decl}")),
        Body::Param { func, index, storage, decl } => match names_another(func, target) {
            Some(f) => (Slot::Program, format!("map param {f}::{index} {storage} {decl}")),
            None => (Slot::Function, format!("map param {index} {storage} {decl}")),
        },
        Body::Return { func, storage, decl } => match names_another(func, target) {
            Some(f) => (Slot::Program, format!("map return {f}::{storage} {decl}")),
            None => (Slot::Function, format!("map return {storage} {decl}")),
        },
        Body::Comment { func, addr, text } => match names_another(func, target) {
            Some(f) => return Err(unbindable(&f)),
            None => (Slot::Function, format!("comment instruction {addr:#x} {text}")),
        },
        Body::Flow { func, addr, kind } => match names_another(func, target) {
            Some(f) => return Err(unbindable(&f)),
            None => (Slot::Function, format!("override flow {addr:#x} {kind}")),
        },
        Body::Readonly { addr, size } => (Slot::Image, format!("readonly {addr:#x} {size}")),
        Body::Volatile { addr, size } => (Slot::Image, format!("volatile {addr:#x} {size}")),
        Body::Name { func, symbol, newname } => match names_another(func, target) {
            Some(f) => return Err(unbindable(&f)),
            None => (Slot::Symbol, format!("rename {symbol} {newname}")),
        },
        Body::Type { func, symbol, decl } => match names_another(func, target) {
            Some(f) => return Err(unbindable(&f)),
            None => (Slot::Symbol, format!("retype {symbol} {decl}")),
        },
    };
    Ok(ConsoleForm { slot, line })
}

fn semicolon(decl: &str) -> String {
    let decl = decl.trim();
    if decl.ends_with(';') {
        decl.to_string()
    } else {
        format!("{decl};")
    }
}

/// Does any directive need the script's second `decompile`?
pub(crate) fn needs_second_pass(directives: &[Directive], target: Option<&str>) -> bool {
    directives
        .iter()
        .filter_map(|d| console_form(d, target).ok())
        .any(|form| form.slot == Slot::Symbol)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn one(spec: &str) -> Directive {
        parse_one(spec).expect("parses")
    }

    #[test]
    fn the_three_directives_of_the_acceptance_probe() {
        assert_eq!(
            one("prototype authenticate int4 authenticate(char *user,char *pass)").body,
            Body::Prototype {
                func: "authenticate".into(),
                decl: "int4 authenticate(char *user,char *pass)".into()
            }
        );
        assert_eq!(
            one("type v2 char[16]").body,
            Body::Type { func: None, symbol: "v2".into(), decl: "char[16]".into() }
        );
        assert_eq!(
            one("name v2 credbuf").body,
            Body::Name { func: None, symbol: "v2".into(), newname: "credbuf".into() }
        );
    }

    #[test]
    fn a_directive_may_name_the_function_it_binds_to() {
        assert_eq!(
            one("name authenticate::v2 credbuf").body,
            Body::Name {
                func: Some("authenticate".into()),
                symbol: "v2".into(),
                newname: "credbuf".into()
            }
        );
        // A C++ name is split at its LAST `::`, so the qualifier keeps its own.
        assert_eq!(
            one("type ns::cls::run::v2 char[8]").body,
            Body::Type {
                func: Some("ns::cls::run".into()),
                symbol: "v2".into(),
                decl: "char[8]".into()
            }
        );
    }

    #[test]
    fn param_and_return_carry_storage_and_a_type() {
        assert_eq!(
            one("param 0 RDI char *user").body,
            Body::Param {
                func: None,
                index: 0,
                storage: "%RDI".into(),
                decl: "char *user".into()
            }
        );
        assert_eq!(
            one("return main::RAX int4").body,
            Body::Return {
                func: Some("main".into()),
                storage: "%RAX".into(),
                decl: "int4".into()
            }
        );
    }

    #[test]
    fn addresses_are_hex_with_or_without_the_prefix() {
        assert_eq!(
            one("data 0x601040 char *sneaky").body,
            Body::Data { addr: 0x601040, decl: "char *sneaky".into() }
        );
        assert_eq!(
            one("comment 400699 open the file").body,
            Body::Comment { func: None, addr: 0x400699, text: "open the file".into() }
        );
    }

    #[test]
    fn function_is_the_define_function_spelling() {
        assert_eq!(
            one("function 0x1400-0x1480=decrypt").body,
            Body::Function { start: 0x1400, end: Some(0x1480), name: Some("decrypt".into()) }
        );
    }

    /// `flow` takes an address and one of the console's four flow words, and
    /// qualifies like every other function-scoped directive.
    #[test]
    fn flow_takes_an_address_and_one_of_four_words() {
        for kind in ["branch", "call", "callreturn", "return"] {
            assert_eq!(
                one(&format!("flow 0x1405 {kind}")).body,
                Body::Flow { func: None, addr: 0x1405, kind: kind.into() }
            );
        }
        // Bare hex is the `--define-function` convention, here too.
        assert_eq!(
            one("flow 1405 return").body,
            Body::Flow { func: None, addr: 0x1405, kind: "return".into() }
        );
        assert_eq!(
            one("flow sub_13c9::0x1405 callreturn").body,
            Body::Flow {
                func: Some("sub_13c9".into()),
                addr: 0x1405,
                kind: "callreturn".into()
            }
        );
    }

    /// A directive kuna cannot honor must say so rather than be dropped: an
    /// accepted-and-inert assertion is the failure mode this plane exists to end.
    #[test]
    fn a_malformed_directive_is_rejected_naming_itself() {
        for spec in [
            "",
            "rename v2 buf",
            "name v2",
            "name v2 buf extra",
            "type v2",
            "data notahex int4 x",
            "param x RDI int4",
            "prototype authenticate",
            "comment 0x400699",
            "flow 0x1405",
            "flow 0x1405 goto",
            "flow notahex return",
            "flow 0x1405 return extra",
        ] {
            let err = parse_one(spec).expect_err("refuses");
            assert!(err.starts_with("--assert"), "got {err:?} for {spec:?}");
        }
    }

    /// The console line names the function the directive TARGETS, not the one
    /// the declaration is written around.  `parse line extern <decl>` binds by
    /// the declared name, so a declaration that renames the function parked its
    /// signature on a fresh symbol and the selected function kept its recovered
    /// one, exiting 0 with nothing on stderr
    /// (`docs/re-needs/text-output-silently-ignores.md`).
    #[test]
    fn a_prototype_that_renames_the_function_still_names_its_target() {
        let d = one("prototype sub_1400055e0 void * sha256(void *out,void *input)");
        let form = console_form(&d, Some("sub_1400055e0")).expect("has a console form");
        assert_eq!(
            form.line,
            "map prototype sub_1400055e0 void * sha256(void *out,void *input);"
        );
        assert_eq!(form.slot, Slot::Program);
    }

    #[test]
    fn directives_lower_to_their_console_commands() {
        let lowered = |spec: &str| {
            let d = one(spec);
            let f = console_form(&d, Some("authenticate")).expect("has a console form");
            (f.slot, f.line)
        };
        assert_eq!(
            lowered("prototype authenticate int4 authenticate(char *u,char *p)"),
            (Slot::Program, "map prototype authenticate int4 authenticate(char *u,char *p);".into())
        );
        assert_eq!(
            lowered("typedef struct pt { int x; int y; };"),
            (Slot::Program, "parse line struct pt { int x; int y; };".into())
        );
        assert_eq!(
            lowered("data 0x601040 char *sneaky"),
            (Slot::Program, "map address 0x601040 char *sneaky".into())
        );
        assert_eq!(
            lowered("param 0 RDI char *user"),
            (Slot::Function, "map param 0 %RDI char *user".into())
        );
        assert_eq!(
            lowered("return RAX int4"),
            (Slot::Function, "map return %RAX int4".into())
        );
        assert_eq!(
            lowered("comment 0x400699 open the file"),
            (Slot::Function, "comment instruction 0x400699 open the file".into())
        );
        assert_eq!(
            lowered("flow 0x1405 return"),
            (Slot::Function, "override flow 0x1405 return".into())
        );
        assert_eq!(lowered("type v2 char[16]"), (Slot::Symbol, "retype v2 char[16]".into()));
        assert_eq!(lowered("name v2 credbuf"), (Slot::Symbol, "rename v2 credbuf".into()));
        assert_eq!(
            lowered("function 0x1400-0x1480=decrypt"),
            (Slot::Program, "function bounds 0x1400 0x1480 as decrypt".into())
        );
        assert_eq!(
            lowered("readonly 0x404028+8"),
            (Slot::Image, "readonly 0x404028 8".into())
        );
        assert_eq!(
            lowered("volatile 0x50000000+4"),
            (Slot::Image, "volatile 0x50000000 4".into())
        );
    }

    /// `readonly`/`volatile` take `<addr>+<size>`; a whitespace-separated size is
    /// accepted too, and the size may be decimal or hex.
    #[test]
    fn a_range_directive_takes_an_address_and_a_size() {
        assert_eq!(
            one("readonly 0x404028+8").body,
            Body::Readonly { addr: 0x404028, size: 8 }
        );
        assert_eq!(
            one("volatile 50000000 4").body,
            Body::Volatile { addr: 0x5000_0000, size: 4 }
        );
        assert_eq!(
            one("readonly 0x404028+0x10").body,
            Body::Readonly { addr: 0x404028, size: 16 }
        );
    }

    /// A range with no size, a zero size or a junk size is rejected: a property
    /// painted over nothing is the accepted-and-inert failure mode again.
    #[test]
    fn a_range_directive_without_a_usable_size_is_rejected() {
        for spec in [
            "readonly 0x404028",
            "readonly 0x404028+0",
            "readonly 0x404028+banana",
            "readonly +8",
            "volatile 0x50000000 4 8",
            "volatile",
        ] {
            let err = parse_one(spec).expect_err("refuses");
            assert!(err.starts_with("--assert"), "got {err:?} for {spec:?}");
        }
    }

    /// A `readonly` range implies read-only propagation for the run; nothing
    /// else does.
    #[test]
    fn only_a_readonly_range_implies_read_only_propagation() {
        use kuna_console::assertions::implies_readonly_propagation;
        assert!(implies_readonly_propagation(&[one("readonly 0x404028+8")]));
        assert!(!implies_readonly_propagation(&[one("volatile 0x50000000+4")]));
        assert!(!implies_readonly_propagation(&[one("name v2 credbuf")]));
    }

    /// A `param`/`return` qualified with a function OTHER than the selected one
    /// is a statement about that function's prototype, so it lowers to the
    /// cross-function console spelling and moves to the program slot.  Before
    /// this the qualifier was dropped and `param callee::0 ECX char *maze`
    /// retyped the CALLER
    /// (`docs/re-needs/qualified-parameter-assertions-modify.md`).
    #[test]
    fn a_qualified_param_lowers_against_the_function_it_names() {
        let d = one("param sub_401c50::0 ECX char *maze");
        let form = console_form(&d, Some("sub_402020")).expect("has a console form");
        assert_eq!(form.slot, Slot::Program);
        assert_eq!(form.line, "map param sub_401c50::0 %ECX char *maze");

        let form = console_form(&d, Some("sub_401c50")).expect("has a console form");
        assert_eq!(form.slot, Slot::Function);
        assert_eq!(form.line, "map param 0 %ECX char *maze");

        let d = one("return sub_401c50::EAX int4");
        let form = console_form(&d, Some("sub_402020")).expect("has a console form");
        assert_eq!(form.slot, Slot::Program);
        assert_eq!(form.line, "map return sub_401c50::%EAX int4");
    }

    /// `comment`, `flow`, `name` and `type` describe the inside of one function
    /// body, so naming a function this run did not decompile cannot be honoured.
    /// Rejecting says so; applying it to the SELECTED function silently is the
    /// bug.
    #[test]
    fn a_qualified_body_directive_naming_another_function_is_rejected() {
        for spec in [
            "name sub_401c50::v2 credbuf",
            "type sub_401c50::v2 char[16]",
            "comment sub_401c50::0x401c60 checks the maze",
            "flow sub_401c50::0x401c60 return",
        ] {
            let d = one(spec);
            let detail = console_form(&d, Some("sub_402020")).expect_err("does not bind");
            assert!(detail.contains("sub_401c50"), "{spec}: {detail}");
            // The same directive against its own function still lowers.
            assert!(console_form(&d, Some("sub_401c50")).is_ok(), "{spec}");
        }
    }

    /// The second `decompile` is emitted ONLY for a symbol-scoped directive, so
    /// every other invocation keeps its current cost.
    #[test]
    fn only_a_symbol_scoped_directive_forces_a_second_pass() {
        assert!(!needs_second_pass(&[one("prototype f int4 f(void)")], Some("f")));
        assert!(!needs_second_pass(&[one("param 0 RDI int4 a")], Some("f")));
        assert!(needs_second_pass(&[one("name v2 buf")], Some("f")));
        assert!(needs_second_pass(
            &[one("prototype f int4 f(void)"), one("type v2 char[4]")],
            Some("f"),
        ));
    }

    #[test]
    fn a_file_holds_one_directive_per_line_with_comments() {
        let dir = std::env::temp_dir().join(format!("kuna-assertdecl-{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("scratch dir");
        let path = dir.join("overrides.kuna");
        std::fs::write(
            &path,
            "# worked out by hand\nname v2 credbuf\n\ntype v2 char[16] # the read buffer\n",
        )
        .expect("write");
        let directives = parse_flag(&format!("@{}", path.display())).expect("parses");
        assert_eq!(directives.len(), 2);
        assert_eq!(directives[0].raw, "name v2 credbuf");
        assert_eq!(
            directives[1].body,
            Body::Type { func: None, symbol: "v2".into(), decl: "char[16]".into() }
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_missing_file_is_an_error_naming_the_path() {
        let err = parse_flag("@/nonexistent/kuna-asserts.txt").expect_err("refuses");
        assert!(err.contains("/nonexistent/kuna-asserts.txt"), "got {err:?}");
    }
}
