//! (kuna) The `symbolnamechars` option gate — stop a symbol name's raw bytes
//! from restructuring the C document they are printed into, and stop two
//! distinct names from collapsing onto one `String`.
//!
//! ## The failure it removes
//!
//! A symbol name travels from `.strtab`/`.dynstr` to emitted C with nothing in
//! between that validates a single byte of it. Whatever the binary claims is
//! what `printc` prints — into a `// Function: <name> @ <addr>` header, a `.h`
//! prototype, a call site and an `.asm` label. Three shapes break the document
//! rather than merely look odd:
//!
//! * `*/` closes the header comment early and the rest of the header becomes
//!   code (`// Function: a*/b @ 0x401006` → the `@ 0x401006` is now a statement);
//! * a raw `0x0a` splits the header, the prototype, the definition, the call
//!   site and the `.asm` label across two lines each — `void a\nb(void)` is not
//!   a declaration of anything;
//! * `//` comments out the remainder of the line it lands on, taking the
//!   parameter list with it.
//!
//! A fourth is not a print problem but an identity one: the name is decoded with
//! `String::from_utf8_lossy`, so every invalid byte becomes U+FFFD and two
//! genuinely different symbols — `a\x80b` at one address and `a\x81b` at
//! another — become the *same* `String`. The export then carries two definitions
//! and two prototypes with one name, which no compiler accepts and no reader can
//! tell apart.
//!
//! Symbol-name bytes are attacker-controlled data, and none of the above needs a
//! malformed header or an exotic format: a few `.strtab` bytes buy a corrupted
//! export.
//!
//! ## Why the option is VALUED and not a bool
//!
//! The most common name in the wild that is not valid C is not hostile at all —
//! it is gcc's clone suffix (`err_fatal.constprop.0`, `main.part.1`,
//! `add_fdes.cold`, `__x86.get_pc_thunk.bx`), which appears on most `-O2`
//! binaries and on six fixtures checked into this repo. Folding those to `_` is
//! a legitimate want for someone who intends to recompile the export, but it
//! rewrites names on nearly every real optimized binary and would change the key
//! that `kuna decompile <name>` is passed. So the two wants are two values:
//!
//! | value | behavior |
//! |---|---|
//! | `off` | verbatim — what the binary literally claims, for someone auditing exactly that |
//! | `safe` | **default**. Only the structural set below. A no-op on every clone suffix |
//! | `ident` | the full identifier reduction ([`sanitize_ident_chain`]) — every non-`[A-Za-z0-9_]` character per `::` component folded away |
//!
//! `ident` never DELETES a name: the reduction drops empty components, so a name
//! made only of non-identifier characters (`***`) would reduce to nothing and put
//! every such symbol under one empty name — the very collapse this option exists
//! to prevent. Such a name is escaped whole instead (`_x2a_x2a_x2a`), which is
//! still an identifier and still injective.
//!
//! ## The exact `safe` set
//!
//! Applied to `&[u8]`, **before** any UTF-8 decode. A byte is rewritten iff:
//!
//! * it is an ASCII control byte (`0x00`–`0x1F`, `0x7F`) — covers `\n`, `\r`,
//!   `\t`, `\0`;
//! * it is `"`, `'` or `\` — the quote/escape characters of every quoted context
//!   the name reaches;
//! * it is a `*` or `/` that forms `*/`, `/*` or `//` with its neighbor (BOTH
//!   characters of the pair are rewritten). A LONE `*` or `/` is left alone: it
//!   is not a comment delimiter, so rewriting it buys nothing structural and
//!   costs collateral on real names;
//! * it is part of an invalid UTF-8 sequence.
//!
//! Everything else — `.` `$` `@` `:` `-` `+` `<` `>` `(` `)` `;` `{` `}` and all
//! valid multi-byte UTF-8 — is left alone. None of `;{}()` can escape a `//`
//! comment or a `"` string, so rewriting them buys nothing structural; they are
//! what `ident` is for. `::` survives both modes intact, because the scope
//! splitter (`Database::find_create_scope_from_symbol_name`) reads it.
//!
//! Two deviations from the shape this was scoped with, both deliberate:
//!
//! 1. **`//` is in the set.** It was not in the original character list, but
//!    `void a//b(void)` comments out the parameter list and the `;`, which is
//!    exactly the structural corruption the other three entries exist to stop.
//!    No gcc clone suffix, mangled symbol or Rust path contains `//`, so it
//!    costs no collateral.
//! 2. **Every rewritten byte becomes `_x<hh>` (its lowercase hex), not `_`.**
//!    `_` is not injective: `a"b` and `a'b` and `a\nb` all collapse to `a_b`,
//!    which reproduces the *exact* defect the invalid-UTF-8 half of this option
//!    exists to fix — two addresses, one name, a C redefinition — merely with a
//!    different trigger. One escape scheme for the whole set keeps the rewrite
//!    injective over every name that does not already contain the escape form,
//!    and costs nothing legible because `safe` fires on no real name at all.
//!
//! ## Mint time, not print time
//!
//! The sanitizer runs where the name string is MINTED — the loader's symbol
//! walks in `kuna-analysis` — not in the printer. `kuna decompile <name>`, the
//! console's `load function`, `kuna functions` and the DB scope path all key on
//! the string in `prog.symbols`; if the printer sanitized, the name in the `.c`
//! would no longer be the name you can hand back to the CLI, which for an
//! agent-first tool trades one bug for a worse one. Sanitizing at the mint makes
//! `prog.symbols`, `kuna functions`, `kuna decompile <name>`, the `.c`, the `.h`
//! and the `.asm` all carry one string.
//!
//! ## Why an env-var bridge, not an `Architecture` flag
//!
//! The symbol table is installed inside `load file`, upstream of every
//! per-function `option` command, so an `Architecture` field would be read long
//! after the names had been minted. This is the same bridge
//! `symbolnamerepair` / `relocrebase` / `typedepth` use:
//! [`SYMBOLNAMECHARS_ENV`], set by the `kuna` CLI before `bootstrap_from_object`
//! (and on the `decomp_dbg` subprocess). The companion
//! `Architecture::analysis_symbolnamechars` exists only so the option is
//! catalog-visible and `phase catalog`'s live `current` field reflects it.

use std::borrow::Cow;

/// Environment variable that carries the `symbolnamechars` mode to the loader.
/// Absent or unrecognized ⇒ [`NameChars::Safe`] (the default).
pub const SYMBOLNAMECHARS_ENV: &str = "KUNA_SYMBOLNAMECHARS";

/// How much of a raw symbol name's byte content is rewritten on the way in.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum NameChars {
    /// Verbatim: the name reaches emitted C exactly as the binary spells it.
    Off,
    /// **Default.** Only the structural set (see the module header): control
    /// bytes, quote/escape characters, comment delimiters, invalid UTF-8.
    #[default]
    Safe,
    /// The full identifier reduction ([`sanitize_ident_chain`]) applied per `::`
    /// component, on top of `safe`'s byte pass.
    Ident,
}

impl NameChars {
    /// Parse an `option symbolnamechars <value>` token. `on` is accepted as a
    /// synonym for the default so the usual on/off habit does something sane.
    pub fn parse(value: &str) -> Option<NameChars> {
        match value.trim().to_ascii_lowercase().as_str() {
            "off" | "0" | "false" | "no" | "verbatim" => Some(NameChars::Off),
            "safe" | "on" | "1" | "true" => Some(NameChars::Safe),
            "ident" | "identifier" => Some(NameChars::Ident),
            _ => None,
        }
    }

    /// The canonical catalog spelling.
    pub fn as_str(self) -> &'static str {
        match self {
            NameChars::Off => "off",
            NameChars::Safe => "safe",
            NameChars::Ident => "ident",
        }
    }
}

/// The mode this process's loader is running in, from [`SYMBOLNAMECHARS_ENV`].
/// Unset or unrecognized ⇒ [`NameChars::Safe`].
///
/// Read ONCE per symbol walk and passed down rather than consulted per symbol:
/// a large image carries a few hundred thousand names and `std::env::var` is not
/// free.
pub fn symbolnamechars_mode() -> NameChars {
    match std::env::var(SYMBOLNAMECHARS_ENV) {
        Ok(v) => NameChars::parse(&v).unwrap_or_default(),
        Err(_) => NameChars::default(),
    }
}

/// Bridge a `set_kuna_option("symbolnamechars", val)` choice to
/// [`SYMBOLNAMECHARS_ENV`] so a later `load file` in the *same* process sees it
/// (the interactive `option symbolnamechars … ` before `load file` path, and the
/// in-process CLI drivers).
pub fn set_symbolnamechars_env(mode: NameChars) {
    std::env::set_var(SYMBOLNAMECHARS_ENV, mode.as_str());
}

/// Whether `a`,`b` (adjacent, in that order) open or close a C comment.
fn is_comment_delim(a: u8, b: u8) -> bool {
    matches!((a, b), (b'*', b'/') | (b'/', b'*') | (b'/', b'/'))
}

/// Whether byte `i` of `name` is one of the structurally dangerous bytes (the
/// `safe` set, minus the UTF-8 validity half which is computed separately).
fn is_structural(name: &[u8], i: usize) -> bool {
    let b = name[i];
    if b < 0x20 || b == 0x7f || b == b'"' || b == b'\'' || b == b'\\' {
        return true;
    }
    if b != b'*' && b != b'/' {
        return false;
    }
    let after = name.get(i + 1).is_some_and(|&n| is_comment_delim(b, n));
    let before = i > 0 && is_comment_delim(name[i - 1], b);
    after || before
}

/// Mark every byte of `name` that is part of an invalid UTF-8 sequence.
fn mark_invalid_utf8(name: &[u8], bad: &mut [bool]) {
    let mut rest = name;
    let mut base = 0usize;
    loop {
        match std::str::from_utf8(rest) {
            Ok(_) => return,
            Err(e) => {
                let good = e.valid_up_to();
                // `error_len() == None` means "unexpected end of input": every
                // remaining byte is part of one truncated sequence.
                let len = e.error_len().unwrap_or(rest.len() - good);
                for b in bad.iter_mut().skip(base + good).take(len) {
                    *b = true;
                }
                let step = good + len;
                if step >= rest.len() {
                    return;
                }
                base += step;
                rest = &rest[step..];
            }
        }
    }
}

/// Apply the `safe` byte pass: every offending byte becomes `_x<hh>`.
///
/// Returns the input borrowed when nothing offends, which is the case for every
/// name a real toolchain emits — the common path allocates nothing.
pub fn sanitize_safe_bytes(name: &[u8]) -> Cow<'_, [u8]> {
    // Cheap pre-scan first: a name with nothing wrong is the overwhelmingly
    // common case (every name a real toolchain emits), and it must allocate
    // nothing at all — a large image carries a few hundred thousand of them.
    let clean = !(0..name.len()).any(|i| is_structural(name, i))
        && (name.is_ascii() || std::str::from_utf8(name).is_ok());
    if clean {
        return Cow::Borrowed(name);
    }
    let mut bad = vec![false; name.len()];
    for (i, flag) in bad.iter_mut().enumerate() {
        *flag = is_structural(name, i);
    }
    mark_invalid_utf8(name, &mut bad);
    let mut out: Vec<u8> = Vec::with_capacity(name.len() + 8);
    for (i, &b) in name.iter().enumerate() {
        if bad[i] {
            out.extend_from_slice(format!("_x{b:02x}").as_bytes());
        } else {
            out.push(b);
        }
    }
    Cow::Owned(out)
}

/// Escape EVERY byte of `name` as `_x<hh>` — the `ident` fallback for a name
/// whose identifier reduction is empty. Always a valid C identifier (it starts
/// with `_`), never empty for a non-empty input, and injective.
fn escape_all_bytes(name: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(name.len() * 4);
    for b in name {
        out.extend_from_slice(format!("_x{b:02x}").as_bytes());
    }
    out
}

/// Sanitize one raw symbol name under `mode`.
///
/// The result is always valid UTF-8 under `safe`/`ident` (that is the point of
/// the invalid-byte escape); under `off` the bytes are returned untouched and
/// the caller's `from_utf8_lossy` keeps its historical behavior.
pub fn sanitize_symbol_name_bytes(name: &[u8], mode: NameChars) -> Cow<'_, [u8]> {
    match mode {
        NameChars::Off => Cow::Borrowed(name),
        NameChars::Safe => sanitize_safe_bytes(name),
        NameChars::Ident => {
            let safe = sanitize_safe_bytes(name);
            // `safe` guarantees valid UTF-8, so this decode never loses a byte.
            let text = String::from_utf8_lossy(&safe);
            let ident = sanitize_ident_chain(&text);
            // A name made only of non-identifier characters (`***`, `...`)
            // reduces to nothing, and `ident` must never DELETE a name — that
            // would put every such symbol under one empty name, which is the
            // collapse this option exists to prevent. Escape it whole instead:
            // still an identifier, still injective, and never empty.
            if ident.is_empty() && !name.is_empty() {
                return Cow::Owned(escape_all_bytes(name));
            }
            if ident.as_bytes() == name {
                Cow::Borrowed(name)
            } else {
                Cow::Owned(ident.into_bytes())
            }
        }
    }
}

/// [`sanitize_symbol_name_bytes`] for a name that is already a `str` (an
/// analysis pass's recovered name, which cannot be invalid UTF-8 — only the
/// structural and identifier halves can fire).
pub fn sanitize_symbol_name(name: &str, mode: NameChars) -> Cow<'_, str> {
    match sanitize_symbol_name_bytes(name.as_bytes(), mode) {
        Cow::Borrowed(_) => Cow::Borrowed(name),
        // `safe`/`ident` only ever emit ASCII in place of a bad byte, so the
        // result is valid UTF-8 by construction.
        Cow::Owned(v) => Cow::Owned(String::from_utf8(v).unwrap_or_else(|e| {
            String::from_utf8_lossy(e.as_bytes()).into_owned()
        })),
    }
}

/// Reduce `name` to a chain of C identifiers joined by `::` — the `ident` mode's
/// core, and the shared implementation behind `kuna-analysis`'s RTTI class-name
/// sanitizer.
///
/// A `::` at bracket depth 0 splits a component; `<`/`(`/`[` and `>`/`)`/`]`
/// increase/decrease depth and fold to `_`, so a `::` INSIDE a template argument
/// list is not mistaken for a scope boundary. Each component then has every
/// non-`[A-Za-z0-9_]` character folded to `_`, `_` runs collapsed, the edges
/// trimmed, and empty components dropped.
///
/// ```text
/// Vec<int>                          -> Vec_int
/// Vec<double>                       -> Vec_double
/// a::b::c::Deep                     -> a::b::c::Deep
/// (anonymous namespace)::Hidden     -> anonymous_namespace::Hidden
/// std::vector<std::pair<int, int> > -> std::vector_std_pair_int_int
/// ```
///
/// Template arguments are FOLDED IN, never deleted: `Vec<int>` and `Vec<double>`
/// must not both reduce to `Vec`, or two classes share one vtable name.
///
/// This is applied UNCONDITIONALLY by the RTTI recovery (a class name reaches
/// emitted C as a type and a scope, and the recovery invented the string in the
/// first place); the `symbolnamechars` gate decides only whether the *loader's*
/// symbol names go through it too.
pub fn sanitize_ident_chain(name: &str) -> String {
    let mut components: Vec<String> = Vec::new();
    let mut current = String::new();
    let mut depth: i32 = 0;
    let chars: Vec<char> = name.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        match c {
            '<' | '(' | '[' => {
                depth += 1;
                current.push('_');
            }
            '>' | ')' | ']' => {
                depth -= 1.min(depth);
                current.push('_');
            }
            ':' if depth == 0 && chars.get(i + 1) == Some(&':') => {
                components.push(std::mem::take(&mut current));
                i += 2;
                continue;
            }
            _ if c.is_ascii_alphanumeric() || c == '_' => current.push(c),
            _ => current.push('_'),
        }
        i += 1;
    }
    components.push(current);
    components
        .into_iter()
        .map(|c| {
            // Collapse `_` runs and trim the edges, so `vector<std::pair<int, int> >`
            // becomes `vector_std_pair_int_int` rather than a wall of underscores.
            let mut out = String::with_capacity(c.len());
            for ch in c.chars() {
                if ch == '_' && out.ends_with('_') {
                    continue;
                }
                out.push(ch);
            }
            out.trim_matches('_').to_string()
        })
        .filter(|c| !c.is_empty())
        .collect::<Vec<_>>()
        .join("::")
}

#[cfg(test)]
mod tests;
