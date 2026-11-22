//! `--types <file.h|dir>` — C declarations loaded from a header for one run.
//!
//! A header is cut into its top-level declarations and each one becomes an
//! ordinary [`Directive`]: a function declaration is a `prototype` for the
//! function or import of that name, and a struct, union, enum or typedef is a
//! `typedef`. They are placed ahead of the caller's own `--assert` directives,
//! so a directive stated on the command line still wins. Each directive keeps
//! its `file:line` in [`Directive::raw`] behind [`ORIGIN`], which is how a
//! declaration the C grammar refuses is reported without stopping the others.
//!
//! The text is expected to be preprocessed C. Without `--types-cpp` kuna runs
//! no preprocessor: `#` lines are skipped (a `# <line> "<file>"` marker
//! re-anchors the reported position) and a macro left in a declaration is a
//! rejected declaration. Compiler decorations the grammar has no reading for
//! (`__attribute__((..))`, `__asm__(..)`, `__extension__`, `__restrict`) are
//! dropped; variable declarations are not facts about a type and are skipped.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use crate::assertions::{Body, Directive};

/// Prefix of [`Directive::raw`] for a declaration that came from `--types`.
pub const ORIGIN: &str = "--types ";

/// Largest header (after preprocessing) `--types` accepts.
const MAX_HEADER_BYTES: u64 = 64 << 20;

/// One top-level declaration and where it started.
#[derive(Debug, PartialEq)]
struct Chunk {
    file: String,
    line: usize,
    text: String,
}

/// Load every `--types` path, in order, as directives. A directory contributes
/// its `*.h` files in name order. `cpp` is the `--types-cpp` command.
pub fn load_types(paths: &[String], cpp: Option<&str>) -> Result<Vec<Directive>, String> {
    let mut out = Vec::new();
    let mut seen = BTreeSet::new();
    let mut known = BTreeMap::new();
    for path in paths {
        for file in header_files(path)? {
            let shown = file.to_string_lossy().into_owned();
            let text = match cpp {
                Some(cpp) => preprocess(cpp, &file)?,
                None => read_header(&file)?,
            };
            for chunk in split_declarations(&shown, &text) {
                let Some(body) = classify(&chunk.text, &mut known) else { continue };
                if seen.insert(body_key(&body)) {
                    let raw = format!("{ORIGIN}{}:{}: {}", chunk.file, chunk.line, chunk.text);
                    out.push(Directive { raw, body });
                }
            }
        }
    }
    Ok(out)
}

/// `file:line: declaration` for a directive `--types` produced.
pub fn origin(raw: &str) -> Option<&str> {
    raw.strip_prefix(ORIGIN)
}

fn body_key(body: &Body) -> String {
    match body {
        Body::Prototype { func, decl } => format!("p {func} {decl}"),
        Body::Typedef { decl } => format!("t {decl}"),
        _ => String::new(),
    }
}

fn header_files(path: &str) -> Result<Vec<PathBuf>, String> {
    let p = Path::new(path);
    if !p.is_dir() {
        return Ok(vec![p.to_path_buf()]);
    }
    let entries = std::fs::read_dir(p).map_err(|e| format!("--types {path}: {e}"))?;
    let mut files: Vec<PathBuf> = entries
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|f| f.extension().is_some_and(|x| x == "h") && f.is_file())
        .collect();
    files.sort();
    if files.is_empty() {
        return Err(format!("--types {path}: the directory holds no .h file"));
    }
    Ok(files)
}

fn read_header(file: &Path) -> Result<String, String> {
    let shown = file.display();
    let meta = std::fs::metadata(file).map_err(|e| format!("--types {shown}: {e}"))?;
    if meta.len() > MAX_HEADER_BYTES {
        return Err(format!("--types {shown}: larger than {} MiB", MAX_HEADER_BYTES >> 20));
    }
    let bytes = std::fs::read(file).map_err(|e| format!("--types {shown}: {e}"))?;
    Ok(String::from_utf8_lossy(&bytes).into_owned())
}

/// Run the caller's preprocessor over `file` and return what it printed.
/// `-E` is added when the command does not carry it.
fn preprocess(cpp: &str, file: &Path) -> Result<String, String> {
    let mut words = cpp.split_whitespace();
    let program = words.next().ok_or("--types-cpp: empty command")?;
    let args: Vec<&str> = words.collect();
    let mut cmd = std::process::Command::new(program);
    if !args.contains(&"-E") {
        cmd.arg("-E");
    }
    let out = cmd
        .args(&args)
        .arg(file)
        .stdin(std::process::Stdio::null())
        .output()
        .map_err(|e| format!("--types-cpp {cpp:?}: {e}"))?;
    if !out.status.success() {
        let stderr = String::from_utf8_lossy(&out.stderr);
        let first = stderr.lines().find(|l| !l.trim().is_empty()).unwrap_or("no diagnostic");
        return Err(format!(
            "--types-cpp {cpp:?} failed on {} ({}): {first}",
            file.display(),
            out.status
        ));
    }
    if out.stdout.len() as u64 > MAX_HEADER_BYTES {
        return Err(format!(
            "--types {}: larger than {} MiB once preprocessed",
            file.display(),
            MAX_HEADER_BYTES >> 20
        ));
    }
    Ok(String::from_utf8_lossy(&out.stdout).into_owned())
}

/// Blank out comments, keeping every newline so line numbers hold.
fn strip_comments(text: &str) -> String {
    let b = text.as_bytes();
    let mut out = Vec::with_capacity(b.len());
    let mut i = 0;
    while i < b.len() {
        match b[i] {
            b'/' if b.get(i + 1) == Some(&b'*') => {
                i += 2;
                while i < b.len() && !(b[i] == b'*' && b.get(i + 1) == Some(&b'/')) {
                    if b[i] == b'\n' {
                        out.push(b'\n');
                    }
                    i += 1;
                }
                i = (i + 2).min(b.len());
                out.push(b' ');
            }
            b'/' if b.get(i + 1) == Some(&b'/') => {
                while i < b.len() && b[i] != b'\n' {
                    i += 1;
                }
            }
            q @ (b'"' | b'\'') => {
                out.push(q);
                i += 1;
                while i < b.len() && b[i] != q && b[i] != b'\n' {
                    if b[i] == b'\\' && i + 1 < b.len() && b[i + 1] != b'\n' {
                        out.push(b[i]);
                        i += 1;
                    }
                    out.push(b[i]);
                    i += 1;
                }
                if i < b.len() && b[i] == q {
                    out.push(q);
                    i += 1;
                }
            }
            c => {
                out.push(c);
                i += 1;
            }
        }
    }
    String::from_utf8_lossy(&out).into_owned()
}

/// `# <line> "<file>"` or `#line <line> "<file>"`: the position of the NEXT line.
fn line_marker(directive: &str) -> Option<(usize, Option<String>)> {
    let rest = directive.strip_prefix('#')?.trim_start();
    let rest = rest.strip_prefix("line").unwrap_or(rest).trim_start();
    let digits = rest.find(|c: char| !c.is_ascii_digit()).unwrap_or(rest.len());
    let line = rest[..digits].parse().ok()?;
    let tail = rest[digits..].trim_start();
    let file = tail
        .strip_prefix('"')
        .and_then(|t| t.split_once('"'))
        .map(|(name, _)| name.to_string());
    Some((line, file))
}

/// Cut a header into top-level declarations. A function body is dropped and
/// ends its declaration; an `extern "C" {` wrapper is transparent.
fn split_declarations(file: &str, text: &str) -> Vec<Chunk> {
    let text = strip_comments(text);
    let mut out = Vec::new();
    let mut file = file.to_string();
    let mut line = 0usize;
    let mut cur = String::new();
    let mut start = (file.clone(), 0usize);
    let mut braces = 0usize;
    let mut parens = 0usize;
    let mut body = 0usize;
    let mut continued = false;
    let mut read = Tail::default();
    for raw in text.split('\n') {
        line = line.saturating_add(1);
        let directive = continued || raw.trim_start().starts_with('#');
        if directive {
            if !continued {
                if let Some((next, name)) = line_marker(raw.trim()) {
                    line = next.saturating_sub(1);
                    if let Some(name) = name {
                        file = name;
                    }
                }
            }
            continued = raw.trim_end().ends_with('\\');
            continue;
        }
        for c in raw.chars() {
            if body > 0 {
                match c {
                    '{' => body += 1,
                    '}' => body -= 1,
                    _ => {}
                }
                if body == 0 {
                    flush(&mut out, &mut cur, &start);
                }
                continue;
            }
            if cur.is_empty() {
                if c.is_whitespace() {
                    continue;
                }
                start = (file.clone(), line);
                read = Tail::default();
            }
            match c {
                '{' if braces == 0 && parens == 0 && read.opens_body(&cur) => body = 1,
                '{' if braces == 0 && is_linkage_block(&cur) => cur.clear(),
                '{' => {
                    braces += 1;
                    cur.push(c);
                }
                '}' if braces == 0 => {}
                '}' => {
                    braces -= 1;
                    cur.push(c);
                }
                '(' => {
                    parens += 1;
                    cur.push(c);
                }
                ')' => {
                    parens = parens.saturating_sub(1);
                    cur.push(c);
                }
                ';' if braces == 0 && parens == 0 => flush(&mut out, &mut cur, &start),
                _ if c.is_whitespace() && cur.ends_with(char::is_whitespace) => {}
                _ => cur.push(c),
            }
        }
        if !cur.is_empty() && !cur.ends_with(char::is_whitespace) {
            cur.push(' ');
        }
    }
    flush(&mut out, &mut cur, &start);
    out
}

/// How much of the declaration being cut [`Tail::opens_body`] has read, and
/// whether its last undecorated token there was `)`.
#[derive(Default)]
struct Tail {
    read: usize,
    paren: bool,
}

impl Tail {
    /// Does a `{` after `cur` open a function body (rather than a record that
    /// carries an attribute before its brace)? Asked only where every group is
    /// closed, so what an earlier call read keeps its reading and only the text
    /// added since is tokenized: a declaration is read once, not once per brace.
    fn opens_body(&mut self, cur: &str) -> bool {
        if !cur.trim_end().ends_with(')') {
            return false;
        }
        if let Some(last) = undecorated(&words(&cur[self.read..])).last() {
            self.paren = last == ")";
        }
        self.read = cur.len();
        self.paren
    }
}

/// `extern "C"` (or `"C++"`) before a `{`. Longer text is never one, which
/// keeps the check constant-time however long the declaration grows.
fn is_linkage_block(cur: &str) -> bool {
    let cur = cur.trim();
    if cur.len() > 256 {
        return false;
    }
    let mut words = cur.split_whitespace();
    words.next() == Some("extern")
        && words.next().is_some_and(|w| w.starts_with('"'))
        && words.next().is_none()
}

fn flush(out: &mut Vec<Chunk>, cur: &mut String, start: &(String, usize)) {
    let text = cur.split_whitespace().collect::<Vec<_>>().join(" ");
    cur.clear();
    if !text.is_empty() {
        out.push(Chunk { file: start.0.clone(), line: start.1, text });
    }
}

/// Identifier-shaped words and single punctuation characters.
fn words(text: &str) -> Vec<&str> {
    let mut out = Vec::new();
    let mut it = text.char_indices().peekable();
    while let Some((at, c)) = it.next() {
        if c.is_whitespace() {
            continue;
        }
        let mut end = at + c.len_utf8();
        if c.is_alphanumeric() || c == '_' {
            while let Some(&(next, d)) = it.peek() {
                if !(d.is_alphanumeric() || d == '_') {
                    break;
                }
                end = next + d.len_utf8();
                it.next();
            }
        }
        out.push(&text[at..end]);
    }
    out
}

/// Drop the compiler decorations the C grammar has no reading for, and spell
/// `__builtin_va_list` as the pointer every supported ABI passes it as.
fn undecorated(toks: &[&str]) -> Vec<String> {
    let mut out: Vec<String> = Vec::with_capacity(toks.len());
    let mut i = 0;
    while i < toks.len() {
        match toks[i] {
            "__attribute__" | "__attribute" | "__asm__" | "__asm" | "__declspec" | "_Alignas"
                if toks.get(i + 1) == Some(&"(") =>
            {
                i = matching(toks, i + 1, "(", ")").map_or(toks.len(), |close| close + 1);
                continue;
            }
            "__extension__" | "__restrict" | "__restrict__" | "restrict" | "_Noreturn"
            | "__inline" | "__inline__" | "__cdecl" => {}
            "__const" | "__const__" => out.push("const".into()),
            "__signed__" | "__signed" => out.push("signed".into()),
            "__volatile__" => out.push("volatile".into()),
            "__builtin_va_list" => out.extend(["void".to_string(), "*".to_string()]),
            tok => out.push(tok.to_string()),
        }
        i += 1;
    }
    out
}

/// Index of the token closing the group opened at `toks[open]`.
fn matching<S: AsRef<str>>(toks: &[S], open: usize, left: &str, right: &str) -> Option<usize> {
    let mut depth = 0usize;
    for (i, tok) in toks.iter().enumerate().skip(open) {
        if tok.as_ref() == left {
            depth += 1;
        } else if tok.as_ref() == right {
            depth = depth.checked_sub(1)?;
            if depth == 0 {
                return Some(i);
            }
        }
    }
    None
}

fn is_identifier(tok: &str) -> bool {
    tok.starts_with(|c: char| c.is_alphabetic() || c == '_')
}

fn is_word(tok: &str) -> bool {
    tok.starts_with(|c: char| c.is_alphanumeric() || c == '_')
}

fn join(toks: &[String]) -> String {
    let mut out = String::new();
    for (i, tok) in toks.iter().enumerate() {
        let after_word = i > 0 && is_word(&toks[i - 1]);
        let spaced = i > 0 && (matches!(toks[i - 1].as_str(), "," | "=") || tok == "=");
        if spaced || (after_word && (is_word(tok) || tok == "*")) {
            out.push(' ');
        }
        out.push_str(tok);
    }
    out
}

/// The type of an integer constant expression, as C's literal and conversion
/// rules give it. `long` is 32 bits wide on some targets and 64 on others, so a
/// `long` or `unsigned long` value is kept only while it means the same on both.
#[derive(Clone, Copy, PartialEq)]
enum Ty {
    Int,
    UInt,
    Long,
    ULong,
    LLong,
    ULLong,
}

impl Ty {
    /// `v` converted to this type as C converts it, or `None` where C leaves
    /// that undefined or the answer depends on the width of `long`.
    fn value(self, v: i128) -> Option<i128> {
        match self {
            Ty::Int | Ty::Long => i32::try_from(v).ok().map(i128::from),
            Ty::UInt => Some(v.rem_euclid(1 << 32)),
            Ty::ULong => u32::try_from(v).ok().map(i128::from),
            Ty::LLong => i64::try_from(v).ok().map(i128::from),
            Ty::ULLong => Some(v.rem_euclid(1 << 64)),
        }
    }

    fn unsigned(self) -> bool {
        matches!(self, Ty::UInt | Ty::ULong | Ty::ULLong)
    }

    fn rank(self) -> u8 {
        self as u8 / 2
    }

    /// The type the usual arithmetic conversions give `a` and `b`, where it is
    /// the same on every target.
    fn common(a: Ty, b: Ty) -> Option<Ty> {
        if a.unsigned() == b.unsigned() {
            return Some(if a.rank() >= b.rank() { a } else { b });
        }
        let (u, s) = if a.unsigned() { (a, b) } else { (b, a) };
        if u.rank() >= s.rank() {
            Some(u)
        } else {
            (s == Ty::LLong && u == Ty::UInt).then_some(s)
        }
    }

    /// The type named by a cast's keywords; `char` and `short` are not read.
    fn named(words: &[String]) -> Option<Ty> {
        let count = |w: &str| words.iter().filter(|t| *t == w).count();
        let unsigned = match (count("unsigned"), count("signed")) {
            (0, _) => false,
            (1, 0) => true,
            _ => return None,
        };
        if count("char") + count("short") > 0 || count("int") > 1 {
            return None;
        }
        Some(match (count("long"), unsigned) {
            (0, false) => Ty::Int,
            (0, true) => Ty::UInt,
            (1, false) => Ty::Long,
            (1, true) => Ty::ULong,
            (2, false) => Ty::LLong,
            (2, true) => Ty::ULLong,
            _ => return None,
        })
    }
}

/// A C integer literal or one-character constant, with the type C gives it.
fn literal(toks: &[String], at: &mut usize) -> Option<(i128, Ty)> {
    let tok = toks.get(*at)?.as_str();
    if tok == "'" {
        let c = toks.get(*at + 1)?;
        if c.len() != 1 || toks.get(*at + 2).map(String::as_str) != Some("'") {
            return None;
        }
        *at += 3;
        return Some((i128::from(c.as_bytes()[0]), Ty::Int));
    }
    let digits = tok.trim_end_matches(['u', 'U', 'l', 'L']);
    let lower = digits.to_ascii_lowercase();
    let (value, decimal) = if let Some(hex) = lower.strip_prefix("0x") {
        (i128::from_str_radix(hex, 16).ok()?, false)
    } else if let Some(bin) = lower.strip_prefix("0b") {
        (i128::from_str_radix(bin, 2).ok()?, false)
    } else if lower.len() > 1 && lower.starts_with('0') {
        (i128::from_str_radix(&lower[1..], 8).ok()?, false)
    } else {
        (lower.parse().ok()?, true)
    };
    let types: &[Ty] = match (tok[digits.len()..].to_ascii_lowercase().as_str(), decimal) {
        ("", true) => &[Ty::Int],
        ("", false) => &[Ty::Int, Ty::UInt],
        ("u", _) => &[Ty::UInt],
        ("l", _) => &[Ty::Long],
        ("ul" | "lu", _) => &[Ty::ULong],
        ("ll", true) => &[Ty::LLong],
        ("ll", false) => &[Ty::LLong],
        ("ull" | "llu", _) => &[Ty::ULLong],
        _ => return None,
    };
    *at += 1;
    let ty = types.iter().find(|t| t.value(value) == Some(value))?;
    Some((value, *ty))
}

/// An integer constant expression over literals and already-seen enumerators,
/// evaluated as C evaluates it, with a signed `<<` that wraps as GCC and Clang
/// define it. `None` for what C leaves undefined (signed overflow, a shift by
/// the width or more, a division by zero), for an answer that depends on the
/// target, and for a value the C grammar cannot read back (above `i64::MAX`).
/// `min` is the lowest binary precedence this call may consume.
fn constant(
    toks: &[String],
    at: &mut usize,
    min: u8,
    known: &BTreeMap<String, i128>,
    depth: u8,
) -> Option<(i128, Ty)> {
    if depth > 32 {
        return None;
    }
    let tok = toks.get(*at)?.as_str();
    let mut left = match tok {
        "-" | "+" | "~" | "!" => {
            *at += 1;
            let (v, ty) = constant(toks, at, 10, known, depth + 1)?;
            match tok {
                "-" => (ty.value(-v)?, ty),
                "~" => (ty.value(!v)?, ty),
                "!" => (i128::from(v == 0), Ty::Int),
                _ => (v, ty),
            }
        }
        "(" => {
            let close = matching(toks, *at, "(", ")")?;
            let cast = toks[*at + 1..close].iter().all(|t| {
                matches!(t.as_str(), "int" | "long" | "short" | "char" | "signed" | "unsigned")
            });
            if cast && close > *at + 1 {
                let ty = Ty::named(&toks[*at + 1..close])?;
                *at = close + 1;
                let (v, _) = constant(toks, at, 10, known, depth + 1)?;
                let v = match ty {
                    Ty::Int => i128::from(v as i32),
                    Ty::LLong => i128::from(v as i64),
                    _ => ty.value(v)?,
                };
                (v, ty)
            } else {
                *at += 1;
                let v = constant(toks, at, 0, known, depth + 1)?;
                if *at != close {
                    return None;
                }
                *at += 1;
                v
            }
        }
        name if is_identifier(name) => {
            *at += 1;
            (Ty::Int.value(*known.get(name)?)?, Ty::Int)
        }
        _ => literal(toks, at)?,
    };
    loop {
        let one = toks.get(*at).map(String::as_str);
        let two = toks.get(*at + 1).map(String::as_str);
        let (op, width, prec) = match (one, two) {
            (Some("<"), Some("<")) => ("<<", 2, 6),
            (Some(">"), Some(">")) => (">>", 2, 6),
            (Some("|"), next) if next != Some("|") => ("|", 1, 1),
            (Some("^"), _) => ("^", 1, 2),
            (Some("&"), next) if next != Some("&") => ("&", 1, 3),
            (Some("+"), _) => ("+", 1, 7),
            (Some("-"), _) => ("-", 1, 7),
            (Some("*"), _) => ("*", 1, 8),
            (Some("/"), _) => ("/", 1, 8),
            (Some("%"), _) => ("%", 1, 8),
            _ => return Some(left),
        };
        if prec < min {
            return Some(left);
        }
        *at += width;
        let right = constant(toks, at, prec + 1, known, depth + 1)?;
        let ((a, ty), (b, _)) = (left, right);
        left = if op == "<<" || op == ">>" {
            let bits = if matches!(ty, Ty::LLong | Ty::ULLong) { 64 } else { 32 };
            let n = u32::try_from(b).ok().filter(|n| *n < bits)?;
            let v = match (op, ty) {
                (">>", _) => a >> n,
                ("<<", Ty::Int) => i128::from((a as i32 as u32).wrapping_shl(n) as i32),
                ("<<", Ty::LLong) => i128::from((a as i64 as u64).wrapping_shl(n) as i64),
                _ => ty.value(a << n)?,
            };
            (v, ty)
        } else {
            let ty = Ty::common(ty, right.1)?;
            let (a, b) = (ty.value(a)?, ty.value(b)?);
            let v = match op {
                "|" => a | b,
                "^" => a ^ b,
                "&" => a & b,
                "+" => a + b,
                "-" => a - b,
                "*" => a.checked_mul(b)?,
                "/" => a.checked_div(b)?,
                _ => ty.value(a.checked_div(b)?).and(a.checked_rem(b))?,
            };
            (ty.value(v)?, ty)
        };
    }
}

/// The value of `toks` when all of it is one constant expression.
fn whole_constant(toks: &[String], known: &BTreeMap<String, i128>) -> Option<i128> {
    let mut at = 0;
    let (value, _) = constant(toks, &mut at, 0, known, 0)?;
    (at == toks.len() && i64::try_from(value).is_ok()).then_some(value)
}

/// Is `toks` one literal the C grammar reads as C does? A binary literal is
/// not (the grammar stops at its `b`), nor is an enumerator name.
fn read_as_written(toks: &[String]) -> bool {
    let [tok] = toks else { return false };
    tok.starts_with(|c: char| c.is_ascii_digit()) && !tok[1..].starts_with(['b', 'B'])
}

fn push_value(out: &mut Vec<String>, value: i128) {
    if value < 0 {
        out.push("-".into());
    }
    out.push(value.unsigned_abs().to_string());
}

/// Replace each enumerator value and array bound that is a constant
/// expression with its value, which is the only spelling the C grammar reads.
/// `known` carries enumerator values across the declarations of one load.
fn fold_constants(toks: Vec<String>, known: &mut BTreeMap<String, i128>) -> Vec<String> {
    let mut out = Vec::with_capacity(toks.len());
    let closers = closers(&toks);
    let mut i = 0;
    while i < toks.len() {
        let tok = toks[i].as_str();
        let close = match tok {
            "[" => closers[i],
            "{" if enum_opens(&toks, i) => closers[i],
            _ => None,
        };
        let Some(close) = close else {
            out.push(toks[i].clone());
            i += 1;
            continue;
        };
        out.push(toks[i].clone());
        let inner = &toks[i + 1..close];
        if tok == "{" {
            fold_enumerators(inner, known, &mut out);
        } else {
            match whole_constant(inner, known).filter(|_| !read_as_written(inner)) {
                Some(value) => push_value(&mut out, value),
                None => out.extend(inner.iter().cloned()),
            }
        }
        out.push(toks[close].clone());
        i = close + 1;
    }
    out
}

/// For each `[` and `{`, the index of the token that closes it: [`matching`]
/// for every group in one pass.
fn closers(toks: &[String]) -> Vec<Option<usize>> {
    let mut out = vec![None; toks.len()];
    let (mut squares, mut braces) = (Vec::new(), Vec::new());
    for (i, tok) in toks.iter().enumerate() {
        match tok.as_str() {
            "[" => squares.push(i),
            "{" => braces.push(i),
            "]" => {
                if let Some(open) = squares.pop() {
                    out[open] = Some(i);
                }
            }
            "}" => {
                if let Some(open) = braces.pop() {
                    out[open] = Some(i);
                }
            }
            _ => {}
        }
    }
    out
}

fn enum_opens(toks: &[String], brace: usize) -> bool {
    let before = |n: usize| brace.checked_sub(n).map(|i| toks[i].as_str());
    before(1) == Some("enum") || (before(2) == Some("enum") && before(1).is_some_and(is_identifier))
}

fn fold_enumerators(inner: &[String], known: &mut BTreeMap<String, i128>, out: &mut Vec<String>) {
    let mut next = Some(0i128);
    for (n, item) in inner.split(|t| t == ",").enumerate() {
        if n > 0 {
            out.push(",".into());
        }
        let Some(name) = item.first() else { continue };
        out.push(name.clone());
        let value = match item.get(1).map(String::as_str) {
            Some("=") => {
                let value = whole_constant(&item[2..], known);
                out.push("=".into());
                match value {
                    Some(value) if !read_as_written(&item[2..]) => push_value(out, value),
                    _ => out.extend(item[2..].iter().cloned()),
                }
                value
            }
            _ => {
                out.extend(item[1..].iter().cloned());
                next
            }
        };
        if let Some(value) = value {
            known.insert(name.clone(), value);
        }
        next = value.and_then(|v| v.checked_add(1));
    }
}

/// Spell an unsized array parameter (`char *argv[]`) as the pointer it is.
fn pointer_parameters(decl: Vec<String>) -> Vec<String> {
    let mut out: Vec<String> = Vec::with_capacity(decl.len());
    let mut toks = decl.into_iter().peekable();
    while let Some(tok) = toks.next() {
        let bare = tok == "[" && toks.peek().is_some_and(|t| t == "]");
        if bare && out.last().is_some_and(|t| is_identifier(t)) {
            toks.next();
            let name = out.pop().unwrap_or_default();
            out.extend(["*".to_string(), name]);
        } else {
            out.push(tok);
        }
    }
    out
}

/// The directive body one declaration states, or `None` for a declaration that
/// states nothing about a type or a signature (a variable).
fn classify(text: &str, known: &mut BTreeMap<String, i128>) -> Option<Body> {
    let toks = fold_constants(undecorated(&words(text)), known);
    if toks.first()? == "typedef" {
        return Some(Body::Typedef { decl: join(&toks) });
    }
    let lead = toks
        .iter()
        .take_while(|t| matches!(t.as_str(), "extern" | "static" | "inline"))
        .count();
    let decl = &toks[lead..];
    let mut braces = 0usize;
    for (i, tok) in decl.iter().enumerate() {
        match tok.as_str() {
            "{" => braces += 1,
            "}" => braces = braces.saturating_sub(1),
            "=" if braces == 0 => return None,
            "(" if braces == 0 => {
                let name = decl.get(i.checked_sub(1)?)?;
                if i < 2 || !is_identifier(name) {
                    return None;
                }
                let func = name.clone();
                return Some(Body::Prototype { func, decl: join(&pointer_parameters(decl.to_vec())) });
            }
            _ => continue,
        }
    }
    let tagged = matches!(decl.first()?.as_str(), "struct" | "union" | "enum");
    let defines = decl.last().is_some_and(|t| t == "}");
    (tagged && defines).then(|| Body::Typedef { decl: join(decl) })
}

#[cfg(test)]
mod tests;
