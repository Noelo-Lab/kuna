//! Bind a dynamic local's identity and locked type through early and late recovery.
use std::rc::Rc;

use crate::context::VarnodeId;
use crate::database::SymbolEntry;
use crate::funcdata::Funcdata;
use crate::varnode::varnode_flags;
use kuna_base::error::KunaResult;

pub fn bind(fd: &mut Funcdata, vn: VarnodeId, entry: &SymbolEntry) -> KunaResult<bool> {
    let (name, unnamed, locked, dtype, flags) = {
        let Some(scope) = fd.get_scope_local() else {
            return Ok(false);
        };
        let symbol = scope.database().symbol(entry.symbol);
        (
            symbol.name.clone(),
            symbol.is_name_undefined(),
            symbol.is_type_locked(),
            symbol.dtype.clone(),
            symbol.get_flags(),
        )
    };
    let Some(dtype) = dtype else { return Ok(false) };
    let Some(v) = fd.vbank().get(vn) else {
        return Ok(false);
    };
    if v.get_size() != entry.get_size() || dtype.get_size() != v.get_size() {
        return Ok(false);
    }
    let old = v.kuna_symbol_entry();
    if old.is_some_and(|symbol| symbol != entry.symbol) {
        return Ok(false);
    }
    let high = v.get_high();
    let binding = high
        .and_then(|id| fd.high_bank().get(id))
        .and_then(|h| h.kuna_dynamic_symbol());
    if binding.is_some_and(|symbol| symbol != entry.symbol) {
        return Ok(false);
    }
    let attached = binding == Some(entry.symbol);
    if old == Some(entry.symbol) && (high.is_none() || attached) {
        return Ok(false);
    }
    let changed = locked && fd.vn_update_type_locked(vn, Rc::clone(&dtype), true, true);
    if let Some(v) = fd.vbank_mut().get_mut(vn) {
        v.set_kuna_symbol_entry(entry.symbol);
        v.set_flags_pub(flags & varnode_flags::namelock);
    }
    if let Some(h) = high.and_then(|id| fd.high_bank_mut().get_mut(id)) {
        if !unnamed {
            h.set_kuna_name(name);
        }
        h.set_kuna_dynamic_symbol(entry.symbol);
        if locked {
            h.set_symbol_type(dtype);
        }
    }
    Ok(changed || old != Some(entry.symbol) || (high.is_some() && !attached))
}
