//! (kuna) Collapse duplicate local-variable *declarations* — the scalar analogue
//! of the existing composite-symbol declaration collapse in
//! [`crate::printc`]'s `emit_local_var_decls`.
//!
//! This is a kuna-owned presentation policy (NOT upstream Ghidra), inspired by the
//! angr decompiler.  angr's variable recovery yields exactly one variable per
//! storage location, so its declaration block lists each local once.  kuna's
//! printer instead walks **HighVariables** (the documented "W4 ScopeLocal symbol
//! walk is the missing surface"), so when the angr-style naming maps many distinct
//! scalar HighVariables that share one stack slot to the *same* name + type +
//! storage, kuna emits one declaration **per high** — a wall of textually
//! identical lines (e.g. `int4 option_index; // stack - 0x3c` repeated ~200×, seen
//! on x86_64/cvs `main`).  The body refers to all of them by the one shared name,
//! so the duplicate declaration *lines* are pure noise (strictly, invalid C
//! re-declarations).
//!
//! Ghidra walks the `ScopeLocal` *Symbol* table once per Symbol
//! (`emitScopeVarDecls`, printc.cc:2667/2696), so it never repeats a declaration.
//! `emit_local_var_decls` already reproduces this for **composite** (array/struct/
//! union) mapped symbols (`printc.rs`, the `seen_sym` retain) but explicitly keeps
//! the per-high behavior for scalars.  This module supplies the scalar collapse,
//! gated by `option dedupvardecls` so default output stays byte-identical.
//!
//! # What is collapsed
//!
//! A declaration is suppressed ONLY when its **fully-rendered signature** — final
//! declarator type, variable name, array adornment, and storage comment — is
//! byte-identical to one already emitted.  This is provably lossless: the emitted
//! C bytes would be character-for-character the same, so no information is removed.
//! Two highs that render the same name but a *different* type or storage have
//! different signatures and both survive (a genuine collision the reader must see).
//!
//! # Apply seam
//!
//! Following the kuna-option idiom ([`crate::kuna_arraynotation::OptionArrayNotation`]),
//! `apply` parses + validates the `on`/`off` value and returns the resolved flag
//! plus the confirmation message; the caller flips the architecture flag
//! (`Architecture::dedup_var_decls`), which the printer reads via the ArchContext.

use kuna_base::marshal::ElementId;
use kuna_base::error::KunaResult;

use crate::options::on_or_off;

/// Marshaling element `<dedupvardecls>` (kuna, 4000+ id range; next free after
/// `hideextensions` = 4090).
pub const ELEM_DEDUPVARDECLS: ElementId = ElementId::new("dedupvardecls", 4091);

/// (kuna) Toggle the scalar duplicate-declaration collapse: `dedupvardecls on|off`.
///
/// "off" keeps the per-high behavior (one declaration per HighVariable, so shared
/// names repeat); "on" collapses declarations whose rendered signature is identical
/// to one already emitted ([`crate::printc::PrintC`]'s `emit_local_var_decls`).
#[derive(Debug, Clone, Copy, Default)]
pub struct OptionDedupVarDecls;

impl OptionDedupVarDecls {
    /// The option name.
    pub const NAME: &'static str = "dedupvardecls";

    /// Parse + validate the `on`/`off` value and return the resolved flag plus the
    /// confirmation message (the caller flips `Architecture::dedup_var_decls`).
    pub fn apply(&self, p1: &str) -> KunaResult<(bool, String)> {
        let val = on_or_off(p1)?;
        let prop = if val { "on" } else { "off" };
        Ok((val, format!("Duplicate local-declaration collapse turned {prop}")))
    }
}

/// The rendered signature of one local declaration: the bytes that distinguish it
/// from another declaration.  Two declarations with the same signature render
/// character-for-character identical lines.
///
/// * `decl_type` — the final declarator type string (after the composite/array
///   relabel), e.g. `"char *"`, `"int4"`, `"undefined8"`.
/// * `name` — the variable name.
/// * `array` — the `(base-type, count)` array adornment, when the symbol is an array.
/// * `comment` — the `(text, offset)` storage comment (`// stack - 0x3c`), present
///   only under angr naming.  Two locals at *different* slots carry different
///   comments and so are NOT collapsed.
pub type DeclSignature = (String, String, Option<(String, i32)>, Option<(String, u64)>);

/// Tracks the rendered signatures already emitted so duplicates can be suppressed.
///
/// Used by `emit_local_var_decls` only when `Architecture::dedup_var_decls` is set;
/// when unset the printer never constructs one, so default output is byte-identical.
#[derive(Debug, Default)]
pub struct DeclDedup {
    seen: std::collections::HashSet<DeclSignature>,
}

impl DeclDedup {
    /// A fresh deduper (nothing seen yet).
    pub fn new() -> Self {
        Self { seen: std::collections::HashSet::new() }
    }

    /// Record `sig` and report whether it was **already** present — i.e. whether the
    /// caller should SKIP emitting this (duplicate) declaration.  The first
    /// occurrence of a signature returns `false` (emit it); every later identical
    /// one returns `true` (suppress it).
    pub fn is_duplicate(&mut self, sig: DeclSignature) -> bool {
        // `insert` returns false when the value was already present.
        !self.seen.insert(sig)
    }
}

#[cfg(test)]
#[path = "kuna_dedupvardecls/tests.rs"]
mod tests;
