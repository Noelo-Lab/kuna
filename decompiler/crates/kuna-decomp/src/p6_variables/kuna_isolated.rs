//! Carrying an isolated local to the next decompile of its function.
//!
//! An isolated Symbol (`isolate <name>`, C++ `Symbol::setIsolated`) is not
//! merged with other variables by choice: `Merge::mergeTestAdjacent` refuses a
//! variable whose Symbol is isolated. The isolation is stated on a decompile
//! that has finished, and the next decompile builds its Symbols again. A
//! register local's Symbol is seeded with its use point and keeps the flag;
//! a stack local and a parameter are rebuilt by the analysis, the parameter
//! after the merges have run. Those two are carried here by storage and
//! width, and the merge reads the carried set beside the Symbols.

use std::collections::BTreeSet;

use kuna_base::address::Address;
use kuna_base::types::int4;

use crate::context::HighVariableId;
use crate::database::{symbol_category, EntryRef};
use crate::funcdata::Funcdata;

/// The storage and width of each isolated Symbol of `fd` that is a parameter
/// or holds its storage for the whole function, with the ones `fd` was itself given.
pub fn carried(fd: &Funcdata) -> BTreeSet<(Address, int4)> {
    let mut out = fd.kuna_prior_names().isolated.clone();
    let Some(lm) = fd.get_scope_local().filter(|lm| lm.has_isolated_symbols()) else { return out };
    for (sym, addr, size, _) in lm.named_symbol_storage() {
        let symbol = lm.database().symbol(sym);
            let whole = lm.symbol_category(sym) == symbol_category::FUNCTION_PARAMETER
            || symbol.mapentry.iter().any(|&entry| {
                matches!(entry, EntryRef::Mapped { .. }) && lm.database().entry(lm.scope_id(), entry).uselimit.empty()
            });
        if symbol.is_isolated() && whole {
            out.insert((addr, size));
        }
    }
    out
}

/// Is `high` a variable the previous decompile isolated ([`carried`]): does
/// it hold the function input or the address-tied storage of one?
pub fn holds(fd: &Funcdata, high: HighVariableId) -> bool {
    let isolated = &fd.kuna_prior_names().isolated;
    if isolated.is_empty() {
        return false;
    }
    let Some(h) = fd.high_bank().get(high) else { return false };
    (0..h.num_instances()).filter_map(|i| fd.vbank().get(h.get_instance(i))).any(|v| {
        (v.is_input() || v.is_addr_tied()) && isolated.contains(&(v.get_addr().clone(), v.get_size()))
    })
}
