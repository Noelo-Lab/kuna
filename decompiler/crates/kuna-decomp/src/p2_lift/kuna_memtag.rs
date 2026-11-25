//! Lift the AArch64 memory-tagging instructions with their tag effect.
//!
//! `AARCH64.pspec` leaves the `ShowMemTag` context at 0, and with it off
//! `irg`, `addg`, `subg`, `gmi`, `ldg` and the `stg` family lift as the plain
//! pointer arithmetic or store they would be on a core without tags. The
//! `acleintrinsics` option sets the context to 1 over the whole code space,
//! which selects the constructors whose tag operation is a user op producing
//! the tagged pointer. No other instruction reads the context.

use std::rc::Rc;

use kuna_base::address::Address;

use crate::architecture::Architecture;

const SHOW_MEMTAG: &[u8] = b"ShowMemTag";

/// Make the `ShowMemTag` context at `entry` agree with the option.
pub fn paint(arch: &mut Architecture, entry: &Address) {
    if !arch.archid.starts_with("AARCH64:") {
        return;
    }
    let want = u32::from(arch.acle_intrinsics);
    if arch.with_context_db(|db| db.get_variable_value(SHOW_MEMTAG, entry).map_or(true, |v| v == want)) {
        return;
    }
    let Some(code) = arch.manage().get_default_code_space().cloned() else { return };
    arch.with_context_db_mut(|db| {
        let begin = Address::new(Rc::clone(&code), 0);
        let _ = db.set_variable_default(SHOW_MEMTAG, want);
        let _ = db.set_variable_region(SHOW_MEMTAG, &begin, &Address::default(), want);
    });
}
