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

use kuna_base::types::uint4;

use crate::context::VarnodeId;
use crate::funcdata::Funcdata;
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
