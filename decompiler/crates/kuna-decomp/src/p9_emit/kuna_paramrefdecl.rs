//! (kuna `paramrefdecl`) A `&parameter` reference must not re-declare the
//! parameter as a body local.
//!
//! `ActionNameVars::linkSpacebaseSymbol` (`coreaction.cc:3005`) binds the offset
//! constant of a `PTRSUB(spacebase, off)` — the `&symbol` markup — to the Symbol
//! that owns the referenced frame slot, and kuna carries the Symbol identity onto
//! the constant's HighVariable (`HighVariable::kuna_ref_symbol`, the port of C++
//! `Varnode::setSymbolReference`).  When the referenced slot is an incoming stack
//! parameter, that Symbol is the parameter's own `function_parameter` Symbol.
//!
//! Upstream declares a function body's locals by walking the ScopeLocal Symbol
//! table for `no_category` Symbols only (`PrintC::emitScopeVarDecls`,
//! `printc.cc:2667`), so a `function_parameter` Symbol is rendered in the
//! signature and never again.  kuna's printer instead walks HighVariables and
//! recognises a parameter by testing whether a `function_parameter` Symbol
//! *contains the storage* of one of the high's member Varnodes.  An `&parameter`
//! reference high has no such member: every instance of it is the PTRSUB offset
//! CONSTANT, which lives in the constant space.  So the storage test says "not a
//! parameter", the high is declared in the body, and — because it took its name
//! from the parameter Symbol it is bound to — the declaration re-declares the
//! parameter under its own name with the constant high's data type:
//!
//! ```text
//! void sub_401571(unsigned int a0,int4 a1,int4 a2)
//! {
//!   int4 a0;                              // the same storage, a second time
//!   ...
//!   v1 = CryptDeriveKey(v4,&a0,4,0);      // LEA ECX,[EBP + 0x8]
//! ```
//!
//! Two objects of the same name and different types in one scope is not valid C,
//! so a `decompile-project` export of such a function cannot be recompiled.
//!
//! This module supplies the Symbol-identity test the storage test cannot make:
//! a high bound to a `function_parameter` Symbol IS that parameter, wherever its
//! Varnodes live.  The printer skips its declaration
//! ([`references_parameter_symbol`]) and the `variables` JSON surface attributes
//! the reference to the parameter it names ([`parameter_reference_varrefs`]).

use crate::context::HighVariableId;
use crate::database::SymbolId;
use crate::funcdata::Funcdata;
use crate::varmap::ScopeLocal;
use kuna_base::address::Address;
use std::collections::BTreeSet;

/// Is `sym` a Symbol upstream would render in the signature rather than in the
/// body (`Symbol::function_parameter`, the category `emitScopeVarDecls` excludes)?
pub fn is_parameter_symbol(lm: &ScopeLocal, sym: SymbolId) -> bool {
    lm.symbol_category(sym) == crate::database::symbol_category::FUNCTION_PARAMETER
}

/// The `function_parameter` Symbol that owns the storage at `addr`, or `None`
/// when that storage is unclaimed or claimed by a plain local.
///
/// The container query is the one `linkSymbol` binds a reference with (the
/// smallest entry covering the base byte, use-point ignored), so this resolves
/// the same Symbol [`references_parameter_symbol`] reads off the reference.
pub fn parameter_symbol_at(lm: &ScopeLocal, addr: &Address) -> Option<SymbolId> {
    let (sym, _) = lm.containing_symbol_for_storage(addr)?;
    is_parameter_symbol(lm, sym).then_some(sym)
}

/// Is `high` an `&symbol` reference bound to a `function_parameter` Symbol?
///
/// The C++ `emitScopeVarDecls(scope, no_category)` predicate, asked of a high
/// whose only Varnodes are the constant PTRSUB offset that carries the reference.
pub fn references_parameter_symbol(fd: &Funcdata, high: HighVariableId) -> bool {
    let Some(sym) = fd.high_bank().get(high).and_then(|h| h.kuna_ref_symbol()) else {
        return false;
    };
    fd.get_scope_local().map(|lm| is_parameter_symbol(lm, sym)).unwrap_or(false)
}

/// The markup varrefs of every `&symbol` reference bound to the `function_parameter`
/// Symbol that owns `[addr, addr+size)`.
///
/// The `variables` JSON surface collects a parameter's uses by matching Varnodes
/// against the parameter's own storage, which an address-taken parameter has none
/// of once the frame slot is only ever referenced through `PTRSUB`.  Reported as
/// an unused parameter, that contradicts the emitted C, which shows the reference.
/// Returns the create indices the markup keys line evidence on.
pub fn parameter_reference_varrefs(fd: &Funcdata, addr: &Address) -> BTreeSet<u64> {
    let mut varrefs = BTreeSet::new();
    let Some(lm) = fd.get_scope_local() else { return varrefs };
    let Some(sym) = parameter_symbol_at(lm, addr) else { return varrefs };
    for (_, high) in fd.high_bank().iter() {
        if high.kuna_ref_symbol() != Some(sym) {
            continue;
        }
        for index in 0..high.num_instances() {
            if let Some(vn) = fd.vbank().get(high.get_instance(index)) {
                varrefs.insert(vn.get_create_index() as u64);
            }
        }
    }
    varrefs
}

#[cfg(test)]
#[path = "kuna_paramrefdecl/tests.rs"]
mod tests;
