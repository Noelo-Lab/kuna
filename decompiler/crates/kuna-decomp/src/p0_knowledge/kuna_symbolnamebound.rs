//! (kuna) The `symbolnamebound` option gate — put a ceiling on what one
//! attacker-controlled symbol name can spend of the process.
//!
//! ## The failure it removes
//!
//! [`Database::find_create_scope_from_symbol_name`] splits a loader symbol name
//! on every `::` and nests **one `Scope` per component**, with no bound on the
//! component count, the per-component length, or the total. A `Scope` is not
//! cheap: a `RangeList`, three `BTreeMap`s (`children`, `nametree`,
//! `multi_entry_set`), two `String`s and a `maptable: Vec<Option<EntryMap>>`
//! sized to the address-space count — **~1.5 KB resident**, measured at 1,493
//! B/component over 200,000 components and 1,504 B/component over 400,000. The
//! interning key is `hash_scope_name(parent.unique_id, name)`, so a repeated
//! component name still allocates a fresh `Scope` at every level: `a::a::a::…`
//! is as expensive as a path of distinct names.
//!
//! That makes a symbol name a **~498x input-to-RSS amplifier**. Measured on
//! `kuna functions` over ELFs whose `.symtab` carries one function named
//! `("a::" * N) + "z"`:
//!
//! | N | file | maxRSS |
//! |---|---|---|
//! | 0 | 4,728 B | 52,932 KB |
//! | 20,000 | 64,728 B | 79,288 KB |
//! | 200,000 | 604,728 B | 345,132 KB |
//!
//! 600,001 bytes of `.strtab` buy 291,724 KB of RSS. The amplifier is strictly
//! **per-component**, not per byte: the same 600,000 bytes in ONE component cost
//! +1,168 KB (1.9x), because a long component is one `String`, not a chain of
//! `Scope`s. `decompile-all` is worse still — quadratic in depth (0.14 s at
//! N=0, 19.73 s at N=20,000, 71.85 s at N=40,000), so 60 KB of `.strtab` already
//! buys a 20-second stall. Nothing validates a name before it is split, so this
//! survives every header check the loader applies: a denial of analysis for the
//! price of a few `.strtab` bytes, on the same
//! `read_loader_symbols`-inside-`load file` seam that
//! [`crate::kuna_symbolnamerepair`] (GH-335) covers for a *different* mechanism
//! (a rejected name, not an unbounded one).
//!
//! ## What the bound does
//!
//! [`bound_scope_path`] rewrites a name only when it exceeds a limit, and
//! otherwise returns [`Cow::Borrowed`] after one allocation-free scan, so the
//! ordinary load pays nothing. Three limits, all applying to the **scope path**
//! (everything before the final component):
//!
//! * the scope-component **count**, the settable one — `symbolnamebound <n>`,
//!   default [`DEFAULT_SCOPE_DEPTH`]; this is the load-bearing bound;
//! * each scope component's **length** ([`MAX_COMPONENT_LEN`]);
//! * the whole scope path's **length** ([`MAX_SCOPE_PATH_LEN`]).
//!
//! Overflow **folds** rather than truncates: the dropped run of components
//! becomes one synthetic component `<24 sanitized bytes of the first dropped
//! component>__x<16 hex>`, where the hex is [`fnv1a64`] over the exact original
//! byte range that was folded away. Distinct names therefore stay distinct, and
//! two symbols that share a scope path but differ in their base name still share
//! the folded scope — the fold hashes the scope range, never the base name.
//!
//! The **base name is never rewritten.** It names no `Scope`, so it costs one
//! `String` (the measured 1.9x, i.e. no amplification at all), and rewriting it
//! would change the string a user has to type for a symbol that was never a
//! resource problem.
//!
//! [`bound_scope_path`] is **idempotent**: its output satisfies every limit, so
//! re-bounding an already-bounded name short-circuits to `Cow::Borrowed`. That
//! is what lets the read path
//! ([`Database::resolve_scope_from_symbol_name`]) and the create path share it
//! and still round-trip — a name installed under a folded path is findable both
//! by its original spelling and by the spelling the listing renders.
//!
//! ## Why the hash is written out here
//!
//! `std::collections::hash_map::DefaultHasher` and `RandomState` are seeded per
//! process. A folded name built on either would differ between runs and between
//! the `functions` and `decompile` surfaces, so every `stringmatch` and every
//! golden diff over such a binary would be noise. [`fnv1a64`] is FNV-1a 64 with
//! its constants spelled out, so the folded spelling is a function of the input
//! bytes alone.
//!
//! ## Why an env-var bridge, not an `Architecture` field
//!
//! The symbol table is installed inside `load file`, **upstream of every
//! per-function `option` command**, so an `Architecture` value would be read long
//! after the memory had already been spent. Same shape as `symbolnamerepair` /
//! `relocrebase` / `typedepth`: the live gate is a process environment variable
//! ([`SYMBOLNAMEBOUND_ENV`]), set by the `kuna` CLI before
//! `bootstrap_from_object` (and on the `decomp_dbg` subprocess). The companion
//! `Architecture::analysis_symbolnamebound` exists only so the option is
//! catalog-visible and `phase catalog`'s live `current` field reflects it.
//!
//! Default **256** — 3.2x the deepest `::` nesting found in any real binary
//! measured for this gate (see [`DEFAULT_SCOPE_DEPTH`]). `off` restores the
//! historical unbounded behavior exactly, for reproducing a report.
//!
//! ## What this bound does NOT close
//!
//! It caps what ONE name costs. It does not cap what a symbol TABLE costs, and
//! that limit is worth stating plainly rather than discovering later: the
//! amplifier is per-`Scope`, and an attacker who spends the same `.strtab` bytes
//! on many moderately deep names instead of one enormous one gets the same RSS.
//! Measured: 3,000 distinct 64-component names — 1.9 MB of ELF — cost **343 MB**
//! whether this gate is on or off, because none of those names exceeds the
//! ceiling and none of their `Scope`s can be shared. The general fix is a cap on
//! the total `Scope` population, or a cheaper `Scope`; both are larger changes
//! than this one, and both need their own measurement against real binaries
//! (libLLVM alone installs ~98,600 functions). What this gate does close is the
//! reported primitive — one name turning 600 KB of `.strtab` into 292 MB — and
//! the quadratic `decompile-all` blowup that rode on it, where 40,000 components
//! cost 86 s and 200,000 did not finish in 300 s.
//!
//! [`Database::find_create_scope_from_symbol_name`]: crate::database::Database::find_create_scope_from_symbol_name
//! [`Database::resolve_scope_from_symbol_name`]: crate::database::Database::resolve_scope_from_symbol_name

use std::borrow::Cow;

use kuna_base::error::{KunaError, KunaResult};

/// Environment variable carrying the scope-component bound (read by
/// `Database::find_create_scope_from_symbol_name` and its resolve twin).
/// Absent or unparseable ⇒ [`DEFAULT_SCOPE_DEPTH`]; an off-token ⇒ unbounded.
pub const SYMBOLNAMEBOUND_ENV: &str = "KUNA_SYMBOLNAMEBOUND";

/// The shipped scope-component ceiling.
///
/// 3.2x the deepest `::` nesting found in any real binary measured for this
/// gate, which is **79**. Two independent sweeps set that number:
///
/// * **1,683,515 demangled names** — the repo fixtures, fourteen large C++
///   objects (libLLVM, libnode, webkit2gtk, ICU, z3, libstdc++, leveldb), nine
///   rustc-built binaries including a 153 MB `librustc_driver`, and the sixty
///   largest ELF objects under `/usr/lib/x86_64-linux-gnu` + `/usr/bin`. Maximum
///   **21** (a Rust v0 tuple-impl name); C++ never exceeded **6**.
/// * **The DWARF names of one rustc binary** (`private/FakeCrypt`, 9,011
///   installed names). Maximum **79** — far higher, because the DWARF name path
///   does NOT strip template arguments, so every `::` *inside* `<…>` counts as a
///   scope component. That is the number that matters, and it is why this is not
///   32 or 64: at either, two real `addr2line`/`alloc::collections` names fold.
///
/// The value is a safety constant, not a tuned one: capping one name's cost at
/// 256 `Scope`s (~384 KB) instead of 200,000 (~300 MB) is a 780x reduction, and
/// any constant achieves the same shape. So the right constant is the smallest
/// one with a real margin over everything real — see the module docs on what
/// this bound does *not* close.
pub const DEFAULT_SCOPE_DEPTH: usize = 256;

/// The ceiling on a single **scope** component's length in bytes. Not
/// separately settable: the component COUNT is the amplifier, and one knob for
/// a resource bound is enough.
///
/// 4x the longest real scope component in those sweeps, which is **exactly
/// 256 bytes** — and structurally so, because that is where rustc's own symbol
/// mangler truncates a legacy path segment (it appends its own `__x` + 16 hex,
/// which is where this module's fold suffix gets its shape). A 256-byte ceiling
/// would clear the only structural producer of long components by a single
/// byte, so it is not a ceiling at all.
pub const MAX_COMPONENT_LEN: usize = 1024;

/// The ceiling on the whole scope path (every component plus its delimiter, i.e.
/// everything before the base name) in bytes.
///
/// Note this bounds the SCOPE PATH, not the whole name. The longest real
/// installed name in the sweep is 1,780 bytes — a Go generic instantiation in
/// `/usr/bin/dockerd` — but it carries no `::` at all, so its scope path is
/// empty and this never sees it. A long, FLAT name is one `String`, not a chain
/// of `Scope`s; capping it would clip real symbols for no benefit. The longest
/// real SCOPE PATH measured is 1,095 bytes (a templated `addr2line` DWARF name
/// in `private/FakeCrypt`), so 4096 clears it by 3.7x.
pub const MAX_SCOPE_PATH_LEN: usize = 4096;

/// The marker that opens a folded component's hash suffix.
const SUFFIX_TAG: &str = "__x";
/// `SUFFIX_TAG` plus 16 hex digits of [`fnv1a64`].
const SUFFIX_LEN: usize = SUFFIX_TAG.len() + 16;
/// How many bytes of the first folded component a synthetic name keeps, so the
/// fold stays readable rather than becoming a bare hash.
const SYNTH_PREFIX_LEN: usize = 24;

/// The scope-component ceiling for this process: `Some(n)` to bound at `n`,
/// `None` for the historical unbounded behavior.
///
/// Unset ⇒ [`DEFAULT_SCOPE_DEPTH`]. `off`/`0`/`false`/`none`/`unbounded` ⇒
/// `None`. Any other parseable positive integer ⇒ that bound. An unparseable
/// value falls back to the default rather than failing: this gate exists to keep
/// a load alive, and refusing the load over the spelling of its own safety limit
/// would defeat the point.
pub fn symbolnamebound_depth() -> Option<usize> {
    match std::env::var(SYMBOLNAMEBOUND_ENV) {
        Ok(v) => match parse_bound_token(v.trim()) {
            Ok(bound) => bound,
            Err(_) => Some(DEFAULT_SCOPE_DEPTH),
        },
        Err(_) => Some(DEFAULT_SCOPE_DEPTH),
    }
}

/// Parse one `symbolnamebound` token into a bound. Shared by the env reader
/// (which swallows the error) and `Architecture::set_kuna_option` (which
/// reports it).
pub fn parse_bound_token(tok: &str) -> Result<Option<usize>, String> {
    match tok.to_ascii_lowercase().as_str() {
        "off" | "0" | "false" | "none" | "unbounded" => Ok(None),
        "" | "on" | "true" | "default" => Ok(Some(DEFAULT_SCOPE_DEPTH)),
        other => match other.parse::<usize>() {
            Ok(n) if n > 0 => Ok(Some(n)),
            _ => Err(format!(
                "symbolnamebound wants a positive scope-component count or `off`, not `{tok}`"
            )),
        },
    }
}

/// `option symbolnamebound <n>|off` — parse and describe, for
/// `Architecture::set_kuna_option`.
pub fn parse_symbolnamebound(p1: &str) -> KunaResult<(Option<usize>, String)> {
    let bound = parse_bound_token(p1.trim()).map_err(|e| KunaError::parse(&e))?;
    let msg = match bound {
        None => "Symbol-name scope bound turned off (unbounded)".to_string(),
        Some(n) => format!("Symbol-name scope bound set to {n} components"),
    };
    Ok((bound, msg))
}

/// Bridge a `set_kuna_option("symbolnamebound", …)` choice to
/// [`SYMBOLNAMEBOUND_ENV`] so a later `load file` in the *same* process sees it
/// (the interactive `option symbolnamebound … ` before `load file` path, and the
/// in-process CLI drivers).
pub fn set_symbolnamebound_env(bound: Option<usize>) {
    match bound {
        None => std::env::set_var(SYMBOLNAMEBOUND_ENV, "off"),
        Some(n) => std::env::set_var(SYMBOLNAMEBOUND_ENV, n.to_string()),
    }
}

/// FNV-1a 64, constants written out.
///
/// The folded spelling of a symbol name must be a function of the input bytes
/// alone — `DefaultHasher`/`RandomState` are seeded per process and would make
/// the same binary decompile differently on two runs.
pub fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for &b in bytes {
        h ^= b as u64;
        h = h.wrapping_mul(0x0000_0100_0000_01b3);
    }
    h
}

/// The largest index `<= i` that starts a `char` in `s` (a stable stand-in for
/// the unstable `str::floor_char_boundary`).
fn floor_char_boundary(s: &str, i: usize) -> usize {
    if i >= s.len() {
        return s.len();
    }
    let mut i = i;
    while i > 0 && !s.is_char_boundary(i) {
        i -= 1;
    }
    i
}

/// The synthetic component standing in for a folded run: up to
/// [`SYNTH_PREFIX_LEN`] identifier-safe bytes of `lead` followed by
/// `__x<16 hex>` of `hashed`.
///
/// Sanitizing to `[A-Za-z0-9_]` is not cosmetic. A scope name is re-split on the
/// delimiter and reaches emitted C verbatim, so a surviving `:` would re-nest the
/// fold and a `(` would open a parameter list; and `load function` splits its
/// argument on whitespace, so a space makes the symbol unaddressable.
fn fold_component(lead: &str, hashed: &[u8]) -> String {
    let mut out = String::with_capacity(SYNTH_PREFIX_LEN + SUFFIX_LEN);
    for &b in lead.as_bytes().iter().take(SYNTH_PREFIX_LEN) {
        out.push(if b.is_ascii_alphanumeric() || b == b'_' { b as char } else { '_' });
    }
    out.push_str(SUFFIX_TAG);
    out.push_str(&format!("{:016x}", fnv1a64(hashed)));
    out
}

/// One kept scope component: the byte range of `fullname` it accounts for, and
/// the text it contributes (borrowed unless it was rewritten).
struct Kept<'a> {
    src: std::ops::Range<usize>,
    text: Cow<'a, str>,
}

/// Rewrite `fullname` so its **scope path** — every `delim`-separated component
/// before the final one — satisfies the count and length bounds. Returns
/// [`Cow::Borrowed`] unchanged when it already does, which is every real name.
///
/// The final component (the base name) is returned verbatim: it nests no
/// `Scope`, so it is not the amplifier, and folding it would only cost the user
/// the name they have to type.
///
/// Idempotent — the output satisfies every bound, so a second application
/// borrows.
pub fn bound_scope_path<'a>(fullname: &'a str, delim: &str) -> Cow<'a, str> {
    let cap = match symbolnamebound_depth() {
        None => return Cow::Borrowed(fullname),
        Some(n) => n.max(1),
    };
    if delim.is_empty() {
        return Cow::Borrowed(fullname);
    }

    // Pass 1: allocation-free scan. `bounds` are the component start offsets;
    // collecting them is O(components) and only happens when a limit is hit.
    let mut count = 0usize;
    let mut worst_len = 0usize;
    let mut mark = 0usize;
    loop {
        let Some(rel) = fullname[mark..].find(delim) else { break };
        let end = mark + rel;
        count += 1;
        worst_len = worst_len.max(end - mark);
        mark = end + delim.len();
    }
    let path_len = mark;
    if count <= cap && worst_len <= MAX_COMPONENT_LEN && path_len <= MAX_SCOPE_PATH_LEN {
        return Cow::Borrowed(fullname);
    }

    // Pass 2: materialize the component ranges (a bounded name never gets here).
    let mut ranges: Vec<std::ops::Range<usize>> = Vec::with_capacity(count);
    let mut mark = 0usize;
    loop {
        let Some(rel) = fullname[mark..].find(delim) else { break };
        let end = mark + rel;
        ranges.push(mark..end);
        mark = end + delim.len();
    }
    let basename_at = mark;

    // (1) Count. Keep `cap - 1` verbatim, fold the rest -- INCLUDING their
    // delimiters, so the hash covers the exact dropped byte range -- into one.
    let verbatim = if count > cap { cap - 1 } else { count };
    let mut kept: Vec<Kept<'a>> = Vec::with_capacity(verbatim + 1);
    for r in ranges.iter().take(verbatim) {
        kept.push(Kept { src: r.clone(), text: Cow::Borrowed(&fullname[r.clone()]) });
    }
    if count > cap {
        let from = ranges[verbatim].start;
        kept.push(Kept {
            src: from..basename_at,
            text: Cow::Owned(fold_component(
                &fullname[ranges[verbatim].clone()],
                fullname[from..basename_at].as_bytes(),
            )),
        });
    }

    // (2) Per-component length. Keep the head verbatim, fold the removed tail.
    for k in kept.iter_mut() {
        if k.text.len() <= MAX_COMPONENT_LEN {
            continue;
        }
        let head_len = floor_char_boundary(&k.text, MAX_COMPONENT_LEN - SUFFIX_LEN);
        let folded = {
            let (head, tail) = k.text.split_at(head_len);
            let mut s = String::with_capacity(MAX_COMPONENT_LEN);
            s.push_str(head);
            s.push_str(SUFFIX_TAG);
            s.push_str(&format!("{:016x}", fnv1a64(tail.as_bytes())));
            s
        };
        k.text = Cow::Owned(folded);
    }

    // (3) Total scope-path length. Fold from the HEAD -- the leading components
    // are the least identifying -- until the path fits. Terminates: one
    // component is at most MAX_COMPONENT_LEN + delim, well under the limit.
    while scope_path_len(&kept, delim) > MAX_SCOPE_PATH_LEN && kept.len() > 1 {
        let take = 2.max(kept.len() - foldable_tail(&kept, delim));
        let take = take.min(kept.len());
        let src = kept[0].src.start..kept[take - 1].src.end;
        let lead = kept[0].text.to_string();
        let folded = Kept {
            text: Cow::Owned(fold_component(&lead, fullname[src.clone()].as_bytes())),
            src,
        };
        kept.splice(0..take, std::iter::once(folded));
    }

    let mut out = String::with_capacity(scope_path_len(&kept, delim) + (fullname.len() - basename_at));
    for k in &kept {
        out.push_str(&k.text);
        out.push_str(delim);
    }
    out.push_str(&fullname[basename_at..]);
    Cow::Owned(out)
}

/// Bytes the kept components occupy once rejoined, delimiters included.
fn scope_path_len(kept: &[Kept<'_>], delim: &str) -> usize {
    kept.iter().map(|k| k.text.len() + delim.len()).sum()
}

/// How many trailing components fit under [`MAX_SCOPE_PATH_LEN`] once the head
/// has been replaced by one synthetic component. Drives how much step (3) folds
/// per round so it converges instead of shaving one component at a time.
fn foldable_tail(kept: &[Kept<'_>], delim: &str) -> usize {
    let budget = MAX_SCOPE_PATH_LEN.saturating_sub(SYNTH_PREFIX_LEN + SUFFIX_LEN + delim.len());
    let mut used = 0usize;
    let mut n = 0usize;
    for k in kept.iter().rev() {
        used += k.text.len() + delim.len();
        if used > budget {
            break;
        }
        n += 1;
    }
    n
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use super::*;

    /// The gate is a process-global env var and the tests run in parallel
    /// threads, so every arm is serialized here (the same shape
    /// `verify_symbolnamerepair.rs` uses around its bootstraps).
    static ENV: Mutex<()> = Mutex::new(());

    /// Set (or clear) the gate WITHOUT taking the lock — for a test that already
    /// holds it.
    fn set_env(tok: Option<&str>) {
        match tok {
            Some(v) => std::env::set_var(SYMBOLNAMEBOUND_ENV, v),
            None => std::env::remove_var(SYMBOLNAMEBOUND_ENV),
        }
    }

    fn with_bound<T>(tok: Option<&str>, f: impl FnOnce() -> T) -> T {
        let _guard = ENV.lock().unwrap_or_else(|e| e.into_inner());
        set_env(tok);
        let out = f();
        std::env::remove_var(SYMBOLNAMEBOUND_ENV);
        out
    }

    /// Env parsing, the default, and the off-tokens. One test, because the env
    /// is process-global.
    #[test]
    fn env_gate_defaults_to_the_shipped_bound_and_off_tokens_unbound() {
        // The guard is held for the WHOLE test: every line below reads or writes
        // the process-global gate, and a sibling test releasing it mid-way would
        // clear the var under us.
        let _guard = ENV.lock().unwrap_or_else(|e| e.into_inner());

        set_env(None);
        assert_eq!(symbolnamebound_depth(), Some(DEFAULT_SCOPE_DEPTH), "unset => the default");
        for off in ["off", "0", "false", "OFF", "None", "unbounded"] {
            set_env(Some(off));
            assert_eq!(symbolnamebound_depth(), None, "`{off}`");
        }
        for (tok, want) in [("8", 8usize), ("1", 1), ("1000", 1000), (" 12 ", 12)] {
            set_env(Some(tok));
            assert_eq!(symbolnamebound_depth(), Some(want), "`{tok}`");
        }
        // Unparseable never fails the load -- it falls back to the default.
        for bad in ["twelve", "-3", "3.5"] {
            set_env(Some(bad));
            assert_eq!(symbolnamebound_depth(), Some(DEFAULT_SCOPE_DEPTH), "`{bad}`");
            assert!(parse_bound_token(bad).is_err(), "`{bad}` still reports as a bad option value");
        }
        // The `set_kuna_option` bridge round-trips through the same var.
        set_symbolnamebound_env(None);
        assert_eq!(symbolnamebound_depth(), None);
        set_symbolnamebound_env(Some(7));
        assert_eq!(symbolnamebound_depth(), Some(7));
        set_env(None);
    }

    /// Every real-shaped name is returned BORROWED: the bound costs the ordinary
    /// load nothing.
    #[test]
    fn real_names_are_untouched_and_unallocated() {
        with_bound(None, || {
            for n in [
                "main",
                "std::filesystem::__cxx11::path::_List::_Impl_deleter::operator()",
                "core::str::traits::core::ops::range::RangeFrom::get",
                "llvm::orc::shared::detail::serializeViaSPSToWrapperFunctionResult",
                "outer::anonymous_namespace::Widget::emit",
                "a::::b",
                "::absolute",
            ] {
                assert!(matches!(bound_scope_path(n, "::"), Cow::Borrowed(_)), "{n} must borrow");
            }
        });
    }

    /// The count bound folds, and the fold is stable, idempotent, and
    /// distinctness-preserving.
    #[test]
    fn deep_paths_fold_to_the_cap() {
        with_bound(Some("8"), || {
            let deep = format!("{}z", "a::".repeat(1000));
            let out = bound_scope_path(&deep, "::");
            assert!(matches!(out, Cow::Owned(_)));
            let comps: Vec<&str> = out.split("::").collect();
            assert_eq!(comps.len(), 9, "8 scope components + the base name: {out}");
            assert_eq!(comps[8], "z", "the base name is verbatim");
            assert!(comps[7].starts_with("a__x"), "the fold keeps a readable lead: {}", comps[7]);

            // Deterministic across calls, and idempotent (the read path re-bounds
            // a name the listing already renders folded).
            assert_eq!(bound_scope_path(&deep, "::"), out);
            assert!(matches!(bound_scope_path(&out, "::"), Cow::Borrowed(_)));
            assert_eq!(bound_scope_path(&out, "::"), out);

            // Distinct originals stay distinct.
            let other = format!("{}z", "a::".repeat(1001));
            assert_ne!(bound_scope_path(&other, "::"), out);

            // Two base names under the same deep path share the folded SCOPE.
            let sib = format!("{}y", "a::".repeat(1000));
            let sib_out = bound_scope_path(&sib, "::");
            assert_eq!(
                sib_out.rsplit_once("::").unwrap().0,
                out.rsplit_once("::").unwrap().0,
                "the fold hashes the scope range, never the base name"
            );
        });
    }

    /// An over-long scope component folds; an over-long BASE name does not.
    #[test]
    fn component_length_folds_but_the_base_name_is_never_rewritten() {
        with_bound(None, || {
            let long = "q".repeat(4000);
            let name = format!("ns::{long}::base");
            let out = bound_scope_path(&name, "::");
            let comps: Vec<&str> = out.split("::").collect();
            assert_eq!(comps[0], "ns");
            assert_eq!(comps[1].len(), MAX_COMPONENT_LEN);
            assert!(comps[1].starts_with("qqq") && comps[1].contains(SUFFIX_TAG));
            assert_eq!(comps[2], "base");

            // A 4,000-byte BASE name (the real 928-byte D-language symbol shape,
            // exaggerated) is left exactly alone -- it nests no Scope.
            let basename_only = format!("ns::{long}");
            assert!(matches!(bound_scope_path(&basename_only, "::"), Cow::Borrowed(_)));
        });
    }

    /// The total-scope-path bound folds from the head and converges.
    #[test]
    fn total_scope_path_is_bounded() {
        with_bound(Some("128"), || {
            // 60 components x 200 bytes = 12,120 bytes of scope path: under both
            // the count cap and the per-component cap, over the total.
            let name = format!("{}base", format!("{}::", "w".repeat(200)).repeat(60));
            let out = bound_scope_path(&name, "::");
            let (path, base) = out.rsplit_once("::").unwrap();
            assert_eq!(base, "base");
            assert!(
                path.len() + 2 <= MAX_SCOPE_PATH_LEN,
                "scope path {} > {MAX_SCOPE_PATH_LEN}",
                path.len()
            );
            for c in path.split("::") {
                assert!(c.len() <= MAX_COMPONENT_LEN);
            }
            assert!(matches!(bound_scope_path(&out, "::"), Cow::Borrowed(_)), "idempotent");
        });
    }

    /// `off` restores the historical unbounded behavior byte for byte.
    #[test]
    fn off_is_the_historical_behavior() {
        with_bound(Some("off"), || {
            let deep = format!("{}z", "a::".repeat(50_000));
            assert!(matches!(bound_scope_path(&deep, "::"), Cow::Borrowed(_)));
            let long = format!("ns::{}::base", "q".repeat(4000));
            assert!(matches!(bound_scope_path(&long, "::"), Cow::Borrowed(_)));
        });
    }

    /// A folded component can never re-split, re-nest, or reach emitted C as
    /// punctuation.
    #[test]
    fn folded_components_are_identifier_safe() {
        with_bound(Some("2"), || {
            let name = "a(b)::c d::e:f::g::h::base";
            let out = bound_scope_path(name, "::");
            let comps: Vec<&str> = out.split("::").collect();
            assert_eq!(comps.len(), 3, "2 scope components + base: {out}");
            for c in &comps[..2] {
                assert!(
                    c.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'(' || b == b')' || b == b' '),
                    "{c}"
                );
            }
            // The SYNTHETIC one is strictly identifier-safe.
            assert!(comps[1].bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_'), "{}", comps[1]);
        });
    }

    /// FNV-1a 64 against its published vectors -- the spelling of a folded name
    /// is part of kuna's output contract, so the hash is pinned, not inherited.
    #[test]
    fn fnv1a64_matches_the_published_vectors() {
        assert_eq!(fnv1a64(b""), 0xcbf2_9ce4_8422_2325);
        assert_eq!(fnv1a64(b"a"), 0xaf63_dc4c_8601_ec8c);
        assert_eq!(fnv1a64(b"foobar"), 0x8594_4171_f739_67e8);
    }

    /// Non-ASCII bytes never cause a slice panic: the length fold lands on a
    /// char boundary and the sanitizer works per byte.
    #[test]
    fn multibyte_names_do_not_panic() {
        with_bound(Some("4"), || {
            let comp = "\u{4e16}\u{754c}".repeat(200); // 1,200 bytes, 3 per char
            let name = format!("{comp}::{comp}::{comp}::{comp}::{comp}::base");
            let out = bound_scope_path(&name, "::");
            assert!(out.ends_with("::base"));
            assert!(matches!(bound_scope_path(&out, "::"), Cow::Borrowed(_)));
        });
    }
}
