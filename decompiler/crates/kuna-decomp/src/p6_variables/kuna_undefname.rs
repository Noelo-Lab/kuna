//! (kuna) The `ActionNameVars` tail that keeps Ghidra's `$$undefNNNNNNNN`
//! placeholder out of every kuna surface (P6, `coreaction.cc:3079`).
//!
//! `Scope::buildUndefinedName` (`database.cc:2854`) hands a Symbol created with an
//! empty name the placeholder `$$undef` + 8 hex digits.  Upstream that string is
//! never observable: `ActionNameVars::apply` renames every still-undefined Symbol
//! before it returns, and `PrintC` reads `sym->getDisplayName()` *live* at emit
//! time, so the render always sees the final name.  kuna's naming model differs on
//! both counts — the name is cached per HighVariable (`HighVariable::kuna_name`)
//! and the catch-all rename was ported but never wired — which let the placeholder
//! reach emitted C, the `decompile-all --json` `variables` array, and the
//! `decompile-project` `.asm` stack-frame comments.  `$$` is not a legal C
//! identifier character, and the two failure modes below additionally split ONE
//! stack Symbol across TWO identifiers, so the body reads a name nothing ever
//! writes.
//!
//! Two producers, both closed here, at the tail of `name_local_highs_angr`:
//!
//! * **Stale cache.** `resolve_default_name_override` (`varmap.rs`) renames the
//!   covering Symbol only for a whole-symbol cover (the C++ `namerec` gate
//!   `sym->isNameUndefined() && high->getSymbolOffset() < 0`), and otherwise
//!   returns the Symbol's *current* display name.  When a partial-cover high of a
//!   stack slot is visited before its whole-cover sibling, the partial caches the
//!   placeholder and the sibling then renames the shared Symbol to `vN`; nothing
//!   back-fills the first cache.  [`refresh_placeholder_high_names`] re-reads the
//!   Symbol once at the end of the pass — kuna's stand-in for upstream's live
//!   `getDisplayName()` read.
//! * **Never renamed.** A Symbol no high covers wholly is never renamed at all and
//!   keeps the placeholder in the Symbol table itself.
//!   [`assign_scope_default_names`] wires the already-ported
//!   `Database::assign_default_names` (`ScopeInternal::assignDefaultNames`), the
//!   last statement of upstream's `ActionNameVars::apply`.
//!
//! Neither step can renumber an existing local: both run after every `vN` has been
//! handed out, so a Symbol named here takes an index above every other local, and
//! the refresh consumes no `base` at all.

use kuna_base::types::int4;

use crate::context::HighVariableId;
use crate::funcdata::Funcdata;

/// C++ `Symbol::isNameUndefined` (`database.cc:2860`): the `buildUndefinedName`
/// shape, `$$undef` + exactly 8 hex digits.
pub fn is_undefined_name(name: &str) -> bool {
    name.len() == 15 && name.starts_with("$$undef")
}

/// The `ActionNameVars` tail: name every Symbol the pass left undefined, then
/// re-read the Symbol names the HighVariable caches froze too early.  `base` is the
/// running `int4 base` of the naming pass, already advanced past every local it
/// named.
pub fn finish_undefined_names(data: &mut Funcdata, base: &mut int4) {
    assign_scope_default_names(data, base);
    refresh_placeholder_high_names(data);
}

/// C++ `localmap->assignDefaultNames(base)` — the final statement of
/// `ActionNameVars::apply` (`coreaction.cc:3079`).  Renames every remaining
/// undefined-named Symbol in the local scope to its `buildDefaultName`.
fn assign_scope_default_names(data: &mut Funcdata, base: &mut int4) {
    let arch = data.get_arch().clone();
    if let Some(lm) = data.get_scope_local_mut() {
        let _ = lm.assign_default_names(base, arch.as_ref());
    }
}

/// Re-resolve every HighVariable still holding a `$$undef` placeholder against the
/// local scope, the stand-in for upstream `PrintC` reading
/// `sym->getDisplayName()` at emit time.
///
/// The re-query is the same `queryProperties(addr, 1, usepoint)` containment lookup
/// (`Funcdata::linkSymbol`, `funcdata_varnode.cc:1190`) that bound the stale name in
/// the first place, so it returns the same SymbolEntry — now carrying the Symbol's
/// final name.  Only the name is rewritten: the in-symbol offset and symbol type the
/// naming pass bound came from this same entry and are already correct.  A high whose
/// placeholder did not come from a local Symbol (no containing entry) is left alone
/// rather than given an invented name.
fn refresh_placeholder_high_names(data: &mut Funcdata) {
    let stale: Vec<HighVariableId> = data
        .high_bank()
        .iter()
        .filter(|(_, h)| h.kuna_name().map(is_undefined_name).unwrap_or(false))
        .map(|(id, _)| id)
        .collect();
    for high in stale {
        let rep = match data.high_name_representative(high) {
            Some(v) => v,
            None => continue,
        };
        let addr = match data.vbank().get(rep) {
            Some(v) => v.get_addr().clone(),
            None => continue,
        };
        let usepoint = data.vn_use_point(rep);
        let resolved = data
            .get_scope_local()
            .and_then(|lm| lm.query_container_for_link(&addr, &usepoint))
            .filter(|info| !info.is_name_undefined)
            .map(|info| info.display_name);
        if let Some(name) = resolved {
            if let Some(h) = data.high_bank_mut().get_mut(high) {
                h.set_kuna_name(name);
            }
        }
    }
}
