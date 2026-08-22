//! **cleanupcode** — delete the Rust drop/deallocate call sites a reader never
//! wants to see (port of the idea in SEFCOM Oxidizer's `CleanupCodeRemover`,
//! `angr/rust/optimization_passes/cleanup_code_remover.py`).
//!
//! ```text
//!   core::ptr::drop_in_place<std::fs::File>(v13);       -->  (gone)
//!   core::ptr::drop_in_place<alloc::vec::Vec<u8>>(&v8); -->  (gone)
//! ```
//!
//! Rust's automatic resource management emits a drop glue call at every scope
//! exit and every `?` early return.  None of it is in the source, none of it
//! carries program meaning for a reader, and on a real binary it is the single
//! largest source of emitted lines in a Rust function.  Oxidizer removes those
//! calls; this is the same transform on kuna's p-code.
//!
//! # The name list, and the one entry that is deliberately missing
//!
//! Oxidizer's `CLEANUP_FUNCTIONS` also lists `free`, `close` and `_close`.
//! kuna's primary corpus is C binaries, where deleting `free()` would be a
//! catastrophically wrong answer, so this port carries only names that **cannot**
//! occur in a C program:
//!
//! ```text
//!   core::ptr::drop_in_place
//!   core::ops::drop::Drop::drop
//!   alloc::raw_vec::RawVecInner::deallocate
//!   __rust_dealloc
//!   __rustc::__rust_dealloc
//! ```
//!
//! (rustc's v0 mangling renders the drop shim without its `core::ptr::` prefix,
//! so a bare `drop_in_place<T>` is matched too — but only when the name really
//! carries generic arguments, which no C symbol does.)
//!
//! That restriction is what makes the pass **structurally inert on a C binary**:
//! no C ELF resolves a call to any of those names, so nothing is removed, and no
//! compiler-detection channel from the loader to the engine is needed (there is
//! none today).  It is why the option can ship default-ON.
//!
//! Oxidizer's `smallvec::deallocate` is NOT carried: it names one third-party
//! crate's helper, the shipped rustc runtime never produces that path, and no
//! binary available here witnesses it — an unverifiable entry in a delete list is
//! worse than a missing one.  `<A as core::alloc::Allocator>::deallocate` (which
//! IS present in real Rust binaries) is likewise left out, faithfully to
//! Oxidizer: it is the inner callee of `RawVecInner::deallocate`, so at a normal
//! call site the listed outer name is what appears, and matching the bare trait
//! method would also delete a hand-written `Allocator` impl's own body call.
//!
//! # Matching: normalize, then compare exactly
//!
//! The recovered display name is whatever kuna's demangler produced, which for
//! the legacy rustc mangling keeps the generic arguments in their escaped form:
//!
//! ```text
//!   core::ptr::drop_in_place$LT$core..result..Result$LT$usize$C$std..io..error..Error$GT$$GT$
//!   _$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$::drop
//!   alloc::raw_vec::RawVecInner$LT$A$GT$::deallocate
//! ```
//!
//! so a string equality test against the list would match nothing, and a
//! `starts_with` test would match `FakeCrypt::fileops::drop_ransom_note` against
//! `...::drop`.  [`normalize_rust_name`] instead reproduces Oxidizer's
//! `normalize(monopolize=True, use_trait_name=True)`: un-escape the `$..$`
//! sequences, then repeatedly collapse the **innermost** angle-bracket group —
//! deleting a plain generic argument list, and replacing a `<T as Trait>`
//! qualified path with the **trait** name.  The three examples above normalize to
//! `core::ptr::drop_in_place`, `core::ops::drop::Drop::drop` and
//! `alloc::raw_vec::RawVecInner::deallocate`; the match is then exact.
//!
//! Normalization only ever *shrinks* a name, and the comparison is exact against
//! five Rust-only paths, so a C or C++ symbol cannot be turned into a match:
//! `<T as crossbeam_epoch::atomic::Pointable>::drop` (a different trait's `drop`)
//! and `std::vector<int>::push_back` both normalize to something that is not in
//! the list.
//!
//! # The seam: pre-heritage, so the argument setup dies too
//!
//! Registered at the top of `mainloop` and self-gated on
//! `get_heritage_pass() == 0` — the documented pre-SSA window
//! ([`crate::p8_structure::kuna_outline`] and
//! [`crate::p2_lift::kuna_loweredswitch`] use the same one).  This mirrors
//! Oxidizer's `STAGE = BEFORE_VARIABLE_RECOVERY`, and the timing is the whole
//! point: before SSA there are no `INDIRECT` call-effect ops to unpick and no
//! `MULTIEQUAL` to patch, the CALL is not a control-transfer op so the CFG is
//! untouched, and — the payoff — the register/stack writes that existed only to
//! set up the drop's arguments lose their last reader and are collected by the
//! ordinary `ActionDeadCode` fixpoint.  Deleting the call later would leave that
//! setup behind as unexplained assignments.
//!
//! # One place the pass declines: inside the glue itself
//!
//! A cleanup routine's own body is mostly calls to *other* cleanup routines —
//! `drop_in_place<Foo>` drops each field, `<RawVec<T,A> as Drop>::drop` is a
//! bare tail call to `RawVecInner::deallocate`. Deleting those would gut the
//! very function a reader has explicitly navigated into, and in the tail-call
//! case it leaves the argument registers looking like a return value (measured:
//! 25 of the 26 functions that GREW under the unguarded pass were exactly this
//! shape). So when the analyzed function is itself one of the listed names, the
//! pass declines outright. The rule stays simple: cleanup calls are removed from
//! their *callers*, never from the glue.
//!
//! The removal is the stock pair used by `Funcdata::block_remove_internal` for a
//! CALL inside a deleted block: [`Funcdata::delete_call_specs`] to drop the
//! `FuncCallSpecs` record, then [`Funcdata::op_destroy`].  A call whose output
//! Varnode already has a reader is left alone (`drop_in_place` returns `()`, so
//! in practice it never does) — declining is always safe, destroying a read
//! Varnode is not.
//!
//! # The option
//!
//! `option cleanupcode on|off`, default **on** (DIV-81), live flag
//! [`ArchContext::remove_cleanup_code`](crate::context::ArchContext).  Marked
//! destructive: it deletes real calls with real side effects, which is the
//! declared intent.

use kuna_base::marshal::ElementId;

use crate::action::{Action, ActionBase, ActionContext, ActionGroupList, ApplyResult};
use crate::context::OpId;
use crate::funcdata::Funcdata;

/// Marshaling element `<cleanupcode>` (kuna). ElementIds live in the 4000+
/// range; 4130 is the previous max.
pub const ELEM_CLEANUPCODE: ElementId = ElementId::new("cleanupcode", 4131);

/// The cleanup-call name list, in **normalized** form (see
/// [`normalize_rust_name`]).
///
/// Oxidizer's `CLEANUP_FUNCTIONS` minus `free`/`close`/`_close` (which a C
/// binary really does call) and minus `smallvec::deallocate` (unwitnessed).
pub const CLEANUP_FUNCTIONS: &[&str] = &[
    "core::ptr::drop_in_place",
    "core::ops::drop::Drop::drop",
    "alloc::raw_vec::RawVecInner::deallocate",
    "__rust_dealloc",
    "__rustc::__rust_dealloc",
];

/// The v0-mangling spelling of the drop shim.
///
/// rustc's `_R..` (v0) mangling gives `drop_in_place` a shim path that the
/// demangler renders **without** its `core::ptr::` prefix — `drop_in_place<T>`
/// — so 23 of the 903 drop routines in the showcase binary never reach
/// [`CLEANUP_FUNCTIONS`].  A bare `drop_in_place` is matched only when the raw
/// name carried generic arguments (a real `<` or an escaped `$LT$`), which no C
/// symbol can, so the inertness argument that lets this option default on is
/// preserved.
pub const CLEANUP_SHIM_FUNCTIONS: &[&str] = &["drop_in_place"];

/// Cheap pre-filter: every entry of [`CLEANUP_FUNCTIONS`] contains one of these
/// substrings *before* normalization, so a name holding neither can be rejected
/// without allocating.
fn could_be_cleanup(raw: &str) -> bool {
    raw.contains("drop") || raw.contains("dealloc")
}

/// Is `s[i..]` a legacy-rustc `$..$` escape?  Returns `(replacement, len)`.
///
/// The escapes rustc's legacy (`_ZN..`) mangling uses for punctuation that is not
/// legal in a symbol name.  `$uXX$` is the generic hex-codepoint form; the named
/// ones are spelled out because they are the common cases.
fn legacy_escape_at(s: &str, i: usize) -> Option<(char, usize)> {
    let b = s.as_bytes();
    if b[i] != b'$' {
        return None;
    }
    // Find the closing '$' within a short window (the longest form is `$u10FFFF$`).
    let end = b[i + 1..].iter().take(8).position(|&c| c == b'$')? + i + 1;
    let body = &s[i + 1..end];
    let ch = match body {
        "LT" => '<',
        "GT" => '>',
        "LP" => '(',
        "RP" => ')',
        "C" => ',',
        "RF" => '&',
        "BP" => '*',
        "SP" => '@',
        _ => {
            // `$uXXXX$` — a raw Unicode codepoint.
            let hex = body.strip_prefix('u')?;
            if hex.is_empty() || !hex.bytes().all(|c| c.is_ascii_hexdigit()) {
                return None;
            }
            char::from_u32(u32::from_str_radix(hex, 16).ok()?)?
        }
    };
    Some((ch, end + 1 - i))
}

/// Un-escape the legacy rustc mangling: `$LT$`/`$GT$`/`$u20$`/... become real
/// punctuation and the `..` path separator becomes `::`.
///
/// A leading `_` is dropped when it introduces a `<`-qualified path
/// (`_$LT$T$u20$as$u20$Trait$GT$::m`), which is how rustc keeps such a symbol
/// from starting with a non-identifier character.
fn unescape_legacy(name: &str) -> String {
    let name = match name.strip_prefix("_$LT$") {
        Some(rest) => {
            let mut s = String::from("$LT$");
            s.push_str(rest);
            std::borrow::Cow::Owned(s)
        }
        None => std::borrow::Cow::Borrowed(name),
    };
    let mut out = String::with_capacity(name.len());
    let mut i = 0;
    let b = name.as_bytes();
    while i < b.len() {
        if let Some((ch, len)) = legacy_escape_at(&name, i) {
            out.push(ch);
            i += len;
            continue;
        }
        if b[i] == b'.' && i + 1 < b.len() && b[i + 1] == b'.' {
            out.push_str("::");
            i += 2;
            continue;
        }
        // Not an escape: copy one whole UTF-8 character.
        let ch = name[i..].chars().next().expect("unescape_legacy: char boundary");
        out.push(ch);
        i += ch.len_utf8();
    }
    out
}

/// Strip the legacy 17-character disambiguator (`::h` + 16 hex digits) some
/// demanglers leave on the tail.
fn strip_hash_suffix(name: &str) -> &str {
    match name.rfind("::h") {
        Some(i) => {
            let tail = &name[i + 3..];
            if tail.len() == 16 && tail.bytes().all(|c| c.is_ascii_hexdigit()) {
                &name[..i]
            } else {
                name
            }
        }
        None => name,
    }
}

/// Collapse angle-bracket groups the way Oxidizer's
/// `normalize(monopolize=True, use_trait_name=True)` does, innermost first:
/// a plain generic list `<A, B>` (and an immediately preceding `::`) is deleted;
/// a qualified path `<T as Trait>` becomes `Trait`.
///
/// Innermost-first means the inner groups of a nested `<T as Trait>` are already
/// gone by the time the `as` split runs, so the split can never straddle a
/// bracket.  The iteration is bounded — a malformed name (unbalanced brackets)
/// leaves the remainder as-is rather than looping.
fn collapse_generics(mut s: String) -> String {
    for _ in 0..64 {
        // The innermost group: the first '<' whose next bracket of either kind
        // is its own '>'.
        let mut open = None;
        let mut found = None;
        for (i, c) in s.char_indices() {
            match c {
                '<' => open = Some(i),
                '>' => {
                    // A '>' with no '<' before it is malformed; leave the rest.
                    let Some(o) = open else { return s };
                    found = Some((o, i));
                    break;
                }
                _ => {}
            }
        }
        // No complete group left (no '<', or a '<' that never closes).
        let Some((o, close)) = found else { return s };
        let inner = &s[o + 1..close];
        let (start, replacement) = match inner.split_once(" as ") {
            // `<T as Trait>` / `<impl T as Trait>` -> the trait path.
            Some((_, trait_path)) => (o, trait_path.trim().to_string()),
            // A plain generic argument list: delete it, and the `::` of a
            // turbofish `::<A>` with it.
            None => (if s[..o].ends_with("::") { o - 2 } else { o }, String::new()),
        };
        s.replace_range(start..close + 1, &replacement);
    }
    s
}

/// Reduce a recovered callee display name to the canonical Rust path Oxidizer
/// matches against (its `utils/demangler.py::normalize`).
pub fn normalize_rust_name(name: &str) -> String {
    collapse_generics(unescape_legacy(strip_hash_suffix(name)))
}

/// Is this callee one of the cleanup routines?
pub fn is_cleanup_call_name(name: &str) -> bool {
    if !could_be_cleanup(name) {
        return false;
    }
    let normalized = normalize_rust_name(name);
    if CLEANUP_FUNCTIONS.contains(&normalized.as_str()) {
        return true;
    }
    // The v0 shim spelling, and only for a real generic instantiation.
    CLEANUP_SHIM_FUNCTIONS.contains(&normalized.as_str())
        && (name.contains('<') || name.contains("$LT$"))
}

/// Collect the CALL ops whose callee is a cleanup routine and that are safe to
/// destroy (alive, in a block, and with no reader of the call's output).
fn cleanup_call_ops(data: &Funcdata) -> Vec<OpId> {
    let mut victims = Vec::new();
    for i in 0..data.num_calls() {
        let fc = data.get_call_specs(i);
        if !is_cleanup_call_name(fc.get_name()) {
            continue;
        }
        let op = fc.get_op();
        let Some(o) = data.obank().get(op) else { continue };
        if o.get_parent().is_none() {
            continue;
        }
        // A read output would be left dangling by destroyVarnode; a cleanup
        // routine returns `()`, so declining here costs nothing.
        if let Some(out) = o.get_out() {
            let live = data.vbank().get(out).map(|v| !v.has_no_descend()).unwrap_or(true);
            if live {
                continue;
            }
        }
        victims.push(op);
    }
    victims
}

/// (kuna) `ActionRemoveCleanupCode` — delete Rust drop/deallocate call sites
/// (option `cleanupcode`, default on, DIV-81).
pub struct ActionRemoveCleanupCode {
    base: ActionBase,
}

impl ActionRemoveCleanupCode {
    /// Construct the action in the given group (`deadcode`).
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionRemoveCleanupCode { base: ActionBase::new(0, "cleanupcode", g) })
    }
}

impl Action for ActionRemoveCleanupCode {
    fn base(&self) -> &ActionBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ActionBase {
        &mut self.base
    }
    fn clone_filtered(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Action>> {
        if !grouplist.contains(self.get_group()) {
            return None;
        }
        Some(Box::new(ActionRemoveCleanupCode { base: self.base.clone() }))
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        if !data.get_arch().remove_cleanup_code {
            return 0;
        }
        // The pre-SSA window: no INDIRECT call effects to unpick, and the
        // argument setup still dies to the following dead-code fixpoint.
        if data.get_heritage_pass() != 0 || data.num_calls() == 0 {
            return 0;
        }
        // Never gut the glue itself: a cleanup routine's body is calls to other
        // cleanup routines, and that IS its content.
        if is_cleanup_call_name(data.get_name()) {
            return 0;
        }
        let victims = cleanup_call_ops(data);
        if victims.is_empty() {
            return 0;
        }
        for op in victims {
            data.delete_call_specs(op);
            data.op_destroy(op);
        }
        1
    }
}

#[cfg(test)]
mod tests;
