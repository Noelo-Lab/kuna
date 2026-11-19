//! (kuna `mapglobalsyms`) Keep the data Symbols `Funcdata::mapGlobals` creates.
//!
//! Upstream `mapGlobals` adds a `DAT_` Symbol to the global scope for every
//! group of global storage the function touches that no Symbol holds, typed by
//! the widest access when it spans the group and `undefined<N>` otherwise
//! (funcdata_varnode.cc). The scope outlives the function, so every function
//! decompiled after it sees that Symbol: an access inside it prints as a piece
//! of it, and its storage is no longer a free-standing `dat_<addr>` the next
//! function types from scratch. `coverVarnodes` adds the names it gives the
//! interior of a group that overruns its Symbol (`small_4`) to the same scope. Kuna's per-function `GlobalQuery` is a snapshot
//! the actions cannot write, so `mapGlobals` records the groups on the
//! `Funcdata` and the driver adds them here once the function is done; the next
//! `build_arch_handle` snapshots them like any other global Symbol.

use kuna_base::address::Address;

use crate::architecture::Architecture;
use crate::funcdata::Funcdata;

/// Add every global `fd`'s `mapGlobals` recorded to the global scope under the
/// name kuna already prints it with, skipping storage some Symbol now holds:
/// the `DAT_` Symbols, and the `coverVarnodes` names (`small_4`) given to the
/// interior of a group that runs past its Symbol. Returns how many Symbols were
/// added.
pub fn persist(arch: &mut Architecture, fd: &Funcdata) -> usize {
    if !arch.mapglobalsyms || (fd.new_globals().is_empty() && fd.global_covers().is_empty()) {
        return 0;
    }
    let Some(global) = arch.symboltab.get_global_scope() else { return 0 };
    let style = arch.kuna_name_style();
    let mut added = 0;
    for (addr, size, name) in fd.global_covers() {
        let held = arch
            .symboltab
            .find_container_matching(global, addr, *size, &Address::new_invalid(), |_| true)
            .is_some();
        let Ok(ct) = arch.types().get_base(*size, crate::dtype::type_metatype::TYPE_UNKNOWN) else { continue };
        if !held && arch.symboltab.add_symbol_mapped(global, name, ct, addr, &Address::new_invalid()).is_ok() {
            added += 1;
        }
    }
    for (addr, ct) in fd.new_globals() {
        let held = arch
            .symboltab
            .find_container_matching(global, addr, 1, &Address::new_invalid(), |_| true)
            .is_some()
            || arch.symboltab.find_function(global, addr).is_some();
        if held {
            continue;
        }
        let name = crate::printc::kuna_global_data_name(style, addr.get_offset());
        if arch
            .symboltab
            .add_symbol_mapped(global, &name, std::rc::Rc::clone(ct), addr, &Address::new_invalid())
            .is_ok()
        {
            added += 1;
        }
    }
    added
}
