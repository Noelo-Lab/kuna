//! Declaration deduplication and function-scope name allocation.
//!
//! The printer uses rendered signatures to collapse identical declaration lines
//! when `dedupvardecls` is enabled. Name allocation reserves existing identifiers
//! and future local spellings, then assigns collision suffixes in caller order.
//! Symbol and overlap-group collapsing remain in [`crate::printc`].

use kuna_base::error::KunaResult;
use kuna_base::marshal::ElementId;

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
        Ok((
            val,
            format!("Duplicate local-declaration collapse turned {prop}"),
        ))
    }
}

/// The rendered signature of one local declaration: the bytes that distinguish it
/// from another declaration.  Two declarations with the same signature render
/// character-for-character identical lines.
///
/// * `decl_type` — the final declarator type string (after the composite/array
///   relabel), e.g. `"char *"`, `"int4"`, `"undefined8"`.
/// * `decl_back` — the declarator suffix printed after the name, e.g. `")[16]"`
///   for `char (*p)[16]`; empty for every other declaration.
/// * `name` — the variable name.
/// * `array` — the `(base-type, count)` array adornment, when the symbol is an array.
/// * `comment` — the `(text, offset)` storage comment (`// stack - 0x3c`), present
///   only under angr naming.  Two locals at *different* slots carry different
///   comments and so are NOT collapsed.
pub type DeclSignature = (
    String,
    String,
    String,
    Option<(String, i32)>,
    Option<(String, u64)>,
);

/// Tracks the rendered signatures already emitted so duplicates can be suppressed.
///
/// Used by `emit_local_var_decls` only when `Architecture::dedup_var_decls` is set.
#[derive(Debug, Default)]
pub struct DeclDedup {
    #[expect(
        clippy::disallowed_types,
        reason = "Only signature membership is observed; caller order determines declarations"
    )]
    seen: std::collections::HashSet<DeclSignature>,
}

/// Allocates declaration identifiers that are unique in one function scope.
///
/// Reserved spellings cannot become generated suffixes. Assigned spellings also
/// carry the next suffix to try when another declaration requests that name.
#[derive(Debug, Default)]
pub struct DeclNameUniquifier {
    #[expect(
        clippy::disallowed_types,
        reason = "Hash iteration cannot affect naming; assignment follows caller order"
    )]
    names: std::collections::HashMap<String, NameState>,
}

#[derive(Debug)]
enum NameState {
    Reserved,
    Assigned {
        next_suffix: u32,
    },
}

impl DeclNameUniquifier {
    /// Construct an allocator for `locals`, reserving `occupied` identifiers such
    /// as parameter names which body declarations may not redeclare.
    pub fn new<'a>(
        locals: impl IntoIterator<Item = &'a str>,
        occupied: impl IntoIterator<Item = &'a str>,
    ) -> Self {
        let mut allocator = Self {
            names: locals
                .into_iter()
                .map(|name| (name.to_owned(), NameState::Reserved))
                .collect(),
        };
        for name in occupied {
            allocator
                .names
                .insert(name.to_owned(), NameState::Assigned { next_suffix: 1 });
        }
        allocator
    }

    /// Return `base` when it is free, otherwise the first free `<base>_<n>`.
    pub fn unique(&mut self, base: &str) -> String {
        match self.names.get_mut(base) {
            Some(state @ NameState::Reserved) => {
                *state = NameState::Assigned { next_suffix: 1 };
                return base.to_owned();
            }
            None => {
                self.names
                    .insert(base.to_owned(), NameState::Assigned { next_suffix: 1 });
                return base.to_owned();
            }
            Some(NameState::Assigned { .. }) => {}
        }
        loop {
            let candidate = {
                let NameState::Assigned { next_suffix } = self.names.get_mut(base).unwrap() else {
                    unreachable!("base was assigned above")
                };
                let candidate = format!("{base}_{next_suffix}");
                *next_suffix += 1;
                candidate
            };
            if let std::collections::hash_map::Entry::Vacant(entry) = self.names.entry(candidate) {
                let name = entry.key().clone();
                entry.insert(NameState::Assigned { next_suffix: 1 });
                return name;
            }
        }
    }
}

impl DeclDedup {
    /// A fresh deduper (nothing seen yet).
    pub fn new() -> Self {
        Self::default()
    }

    /// Record `sig` and report whether it was **already** present — i.e. whether the
    /// caller should SKIP emitting this (duplicate) declaration.  The first
    /// occurrence of a signature returns `false` (emit it); every later identical
    /// one returns `true` (suppress it).
    pub fn is_duplicate(&mut self, sig: DeclSignature) -> bool {
        !self.seen.insert(sig)
    }
}

#[cfg(test)]
#[path = "kuna_dedupvardecls/tests.rs"]
mod tests;
