//! `--define-function` — telling kuna where a function starts and ends.
//!
//! # Why this exists
//!
//! Every function boundary kuna knows is one it *derived*: discovery finds the
//! entries, and the extent is reconstructed as the address-contiguous clip
//! `[entry, next_entry)` ([`kuna_console::funcextent`]) over an unbounded flow
//! follow.  That is the right default and it is wrong exactly where reverse
//! engineering is hard — an obfuscated, packed or hand-written image, where a
//! missed entry silently merges two functions and a phantom entry invents one.
//! Until this flag the `kuna` binary could not correct either: its generated
//! console script emitted a fixed vocabulary (`load file` / `option` /
//! `read symbols` / `load function|addr` / `kassert` / `decompile` / `print`),
//! and "function F spans `[start,end)`" was not expressible anywhere in the
//! engine (`docs/re-needs/no-cli-function-boundary-override.md`).
//!
//! # The declaration
//!
//! ```text
//!   --define-function 0x1400[-0x1480][=name]
//!   --define-function @bounds.txt
//! ```
//!
//! `start` declares the entry (the `map function` recipe: a `FunctionSymbol` so
//! call sites name it, and a name→address registration so it enumerates and
//! resolves by name).  `end` is EXCLUSIVE and declares the extent, which bounds
//! flow following for that function and replaces the clip the inventory reports.
//! Both halves are optional in the sense that `end` may be omitted — a bare
//! `--define-function 0x1400` asserts an entry and leaves the extent natural.
//!
//! The `@file` form is the durable half: an agent that has worked out the real
//! boundaries of a packed image keeps them in a file and passes the same
//! `@bounds.txt` to every later invocation.  kuna does not write the declarations
//! back into the image, so durability is caller-carried by design — the file is
//! the artifact, and it is plain text an agent can generate and diff.
//!
//! Both surfaces converge on the console `function bounds <start> [<end>] [as <name>]`
//! command: the script path emits it, the in-process path calls the
//! [`kuna_console::engine::ConsoleProgram::declare_function`] it is a wrapper for.

use std::fmt::Write as _;

/// One caller-declared function boundary.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct FuncDecl {
    /// Entry VMA.
    pub(crate) start: u64,
    /// Exclusive end VMA; `None` leaves the extent unbounded.
    pub(crate) end: Option<u64>,
    /// The name to give the entry; `None` takes kuna's generated `sub_<addr>`.
    pub(crate) name: Option<String>,
}

impl FuncDecl {
    /// The declared byte extent, or `0` for "unbounded" (the engine-wide
    /// `UNBOUNDED_SIZE` convention).
    pub(crate) fn size(&self) -> u64 {
        self.end.map(|end| end - self.start).unwrap_or(0)
    }

    /// The `function bounds` console line this declaration lowers to.
    pub(crate) fn console_line(&self) -> String {
        let mut line = format!("function bounds {:#x}", self.start);
        if let Some(end) = self.end {
            let _ = write!(line, " {end:#x}");
        }
        if let Some(name) = &self.name {
            let _ = write!(line, " as {name}");
        }
        line
    }
}

/// Parse one `--define-function` value: either a declaration or `@FILE`.
///
/// A file holds one declaration per line; blank lines and `#` comments are
/// skipped, so an agent can annotate what it worked out.
pub(crate) fn parse_flag(value: &str) -> Result<Vec<FuncDecl>, String> {
    let Some(path) = value.strip_prefix('@') else {
        return Ok(vec![parse_one(value)?]);
    };
    let text = std::fs::read_to_string(path)
        .map_err(|e| format!("--define-function @{path}: {e}"))?;
    let mut out = Vec::new();
    for (n, line) in text.lines().enumerate() {
        let line = line.split('#').next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }
        out.push(parse_one(line).map_err(|e| format!("{path}:{}: {e}", n + 1))?);
    }
    Ok(out)
}

/// Parse `START[-END][=NAME]`, hex with or without a `0x` prefix.
fn parse_one(spec: &str) -> Result<FuncDecl, String> {
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
    Ok(FuncDecl { start, end, name: name.map(str::to_string) })
}

/// A bare or `0x`-prefixed hexadecimal VMA.
fn parse_vma(tok: &str) -> Option<u64> {
    let tok = tok.trim();
    let body = tok.strip_prefix("0x").or_else(|| tok.strip_prefix("0X")).unwrap_or(tok);
    if body.is_empty() || !body.chars().all(|c| c.is_ascii_hexdigit()) {
        return None;
    }
    u64::from_str_radix(body, 16).ok()
}

/// Apply the declarations to a loaded program (the in-process surfaces).
///
/// Runs AFTER the analysis commit, so a declaration outranks whatever discovery
/// decided about the same address — that is the whole point of the flag.
pub(crate) fn apply(
    prog: &mut kuna_console::engine::ConsoleProgram,
    decls: &[FuncDecl],
) -> Result<(), String> {
    let Some(space) = prog.arch().manage().get_default_code_space().cloned() else {
        if decls.is_empty() {
            return Ok(());
        }
        return Err("--define-function: the loaded program has no default code space".into());
    };
    for decl in decls {
        let addr = kuna_base::address::Address::new(std::rc::Rc::clone(&space), decl.start);
        prog.declare_function(addr, decl.name.as_deref(), decl.size() as kuna_base::types::int4)
            .map_err(|e| {
                format!("--define-function {:#x}: {}", decl.start, e.explain())
            })?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn one(spec: &str) -> FuncDecl {
        parse_flag(spec).expect("parses").pop().expect("one declaration")
    }

    #[test]
    fn parses_start_end_and_name() {
        assert_eq!(
            one("0x1400-0x1480=decrypt"),
            FuncDecl { start: 0x1400, end: Some(0x1480), name: Some("decrypt".into()) }
        );
    }

    #[test]
    fn end_and_name_are_each_optional() {
        assert_eq!(one("0x1400"), FuncDecl { start: 0x1400, end: None, name: None });
        assert_eq!(
            one("0x1400-0x1480"),
            FuncDecl { start: 0x1400, end: Some(0x1480), name: None }
        );
        assert_eq!(
            one("0x1400=decrypt"),
            FuncDecl { start: 0x1400, end: None, name: Some("decrypt".into()) }
        );
    }

    #[test]
    fn a_bare_hex_address_needs_no_prefix() {
        assert_eq!(one("1400-1480"), FuncDecl { start: 0x1400, end: Some(0x1480), name: None });
    }

    #[test]
    fn the_extent_is_end_exclusive() {
        assert_eq!(one("0x1400-0x1480").size(), 0x80);
        assert_eq!(one("0x1400").size(), 0);
    }

    /// An inverted or empty range is a caller error, not a silently-unbounded
    /// function: a declaration kuna cannot honor must say so.
    #[test]
    fn an_end_at_or_below_the_start_is_rejected() {
        assert!(parse_flag("0x1480-0x1400").is_err());
        assert!(parse_flag("0x1400-0x1400").is_err());
    }

    #[test]
    fn a_non_hex_token_is_rejected() {
        assert!(parse_flag("main").is_err());
        // A trailing `-` leaves an empty tail, so it is not a separator and the
        // whole token must fail as hex rather than parse as a bare start.
        assert!(parse_flag("0x1400-").is_err());
    }

    #[test]
    fn lowers_to_the_console_command() {
        assert_eq!(
            one("0x1400-0x1480=decrypt").console_line(),
            "function bounds 0x1400 0x1480 as decrypt"
        );
        assert_eq!(one("0x1400-0x1480").console_line(), "function bounds 0x1400 0x1480");
        assert_eq!(one("0x1400").console_line(), "function bounds 0x1400");
        // A name with no extent: `as` keys the name, so it cannot be read as the
        // end address.
        assert_eq!(one("0x1400=decrypt").console_line(), "function bounds 0x1400 as decrypt");
    }

    #[test]
    fn a_file_holds_one_declaration_per_line_with_comments() {
        let dir = std::env::temp_dir().join(format!("kuna-funcdecl-{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("scratch dir");
        let path = dir.join("bounds.txt");
        std::fs::write(&path, "# worked out by hand\n0x1400-0x1480=decrypt\n\n0x1480-0x1500\n")
            .expect("write");
        let decls = parse_flag(&format!("@{}", path.display())).expect("parses");
        assert_eq!(decls.len(), 2);
        assert_eq!(decls[0].name.as_deref(), Some("decrypt"));
        assert_eq!(decls[1].start, 0x1480);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_missing_file_is_an_error_naming_the_path() {
        let err = parse_flag("@/nonexistent/kuna-bounds.txt").expect_err("refuses");
        assert!(err.contains("/nonexistent/kuna-bounds.txt"), "got {err:?}");
    }
}
