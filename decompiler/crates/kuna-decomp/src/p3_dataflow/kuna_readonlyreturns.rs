//! (kuna `readonlyreturns`) No RETURN guard for read-only memory nothing writes.
//!
//! `Heritage::guardReturns` puts a `glob = glob` COPY in front of every RETURN
//! for every persistent range a heritage pass meets. Its addrforced output is
//! what keeps a global's last write alive through dead-code removal, and its
//! input is whatever value reaches the return: the function's own write, or
//! the INDIRECT a call left on the range. The cost is the pass's persistent
//! ranges times the function's RETURN ops, and every RETURN block that
//! `ActionReturnSplit` later clones copies its COPYs with it.
//!
//! On a pass where `readonlycalls` leaves read-only memory unguarded at calls,
//! a read-only range the function never writes has no write and no INDIRECT
//! anywhere: the value that reaches every return is the one it had at entry.
//! Its return COPY keeps nothing alive, and the printer never shows it. Such a
//! range gets none. Any write in the pass (an explicit store, a syscall's
//! memory effect) keeps the COPYs, as does every range of a pass that keeps its
//! read-only call guards, whose chains the return COPYs are the reader of, and
//! every range under `indexaliasguard full`, whose STORE guards are placed after
//! the return guards and would be read only by them.
//!
//! The COPYs also keep the range's entry value live to every RETURN, so its
//! Cover spans the function and `Merge` never joins another value into the
//! global: such a join prints the value's definition as a store to read-only
//! memory (`dat_400050 ^= a1;`). The heritage pass records each range it skips
//! ([`Returnless`]), and [`keeps_apart`] refuses those joins in their place.

use std::collections::BTreeSet;

use kuna_base::address::Address;
use kuna_base::types::{int4, uint4};

use crate::context::{HighVariableId, VarnodeId};
use crate::funcdata::Funcdata;
use crate::merge::MergeContext;
use crate::varnode::varnode_flags;

/// Does the RETURN guard of a range with properties `fl` go unbuilt: the
/// option is on, the pass leaves read-only ranges unguarded at calls
/// (`readonly_unguarded`), the range is read-only, nothing in the pass writes
/// it (`write`, after the call and syscall guards), and no STORE guard can
/// follow (`indexaliasguard` below `full`).
pub(crate) fn skips(fd: &Funcdata, fl: uint4, readonly_unguarded: bool, write: &[VarnodeId]) -> bool {
    fd.get_arch().readonly_returns
        && readonly_unguarded
        && (fl & varnode_flags::readonly) != 0
        && write.is_empty()
        && fd.get_arch().index_alias_guard < crate::kuna_indexaliasguard::LEVEL_FULL
}

/// The ranges this function's heritage passes gave no RETURN guard, as
/// (space index, offset, size).
#[derive(Debug, Clone, Default)]
pub struct Returnless(BTreeSet<(int4, u64, int4)>);

impl Returnless {
    pub(crate) fn record(&mut self, addr: &Address, size: int4) {
        if let Some(space) = addr.get_space() {
            self.0.insert((space.get_index(), addr.get_offset(), size));
        }
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub(crate) fn contains(&self, space: int4, offset: u64, size: int4) -> bool {
        self.0.contains(&(space, offset, size))
    }

    pub(crate) fn clear(&mut self) {
        self.0.clear();
    }
}

/// Must `Merge` keep `a` and `b` apart because one is a global whose entry
/// value lost its RETURN guards and the other holds a value that is not a copy
/// of that entry value?
pub(crate) fn keeps_apart(ctx: &mut dyn MergeContext, a: HighVariableId, b: HighVariableId) -> bool {
    if a == b || !ctx.kuna_has_returnless() {
        return false;
    }
    for (global, other) in [(a, b), (b, a)] {
        if !ctx.high_is_persist(global) || ctx.high_is_addr_tied(other) {
            continue;
        }
        let Some(entry) = ctx.bank_input_varnode(global) else {
            continue;
        };
        if !ctx.kuna_returnless_entry(entry) {
            continue;
        }
        let n = ctx.high_num_instances(other);
        if (0..n).any(|i| !ctx.vn_copy_shadow(ctx.high_get_instance(other, i), entry)) {
            return true;
        }
    }
    false
}
