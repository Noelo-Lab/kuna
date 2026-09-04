//! P3 marker copy-propagation brake — the `tiedstorekeep` decision point.
//!
//! # The gap
//!
//! A `MOV [frame+off],REG` that stores a call's return value into a local lowers
//! to `stackvn = COPY(REG)` with an address-tied output.  `RulePropagateCopy`
//! (`ruleaction.cc:3943`) then rewrites every reader of `stackvn` to read `REG`
//! directly.  Ordinary readers are fine — the value is the same and the store
//! stays alive for whoever still reads the location.  A *marker* reader
//! (`INDIRECT` across a later call, `MULTIEQUAL` at a join) is not: markers do
//! not print, so once the last ordinary reader has been rewritten and the marker
//! has swallowed the register too, the `COPY` has no descendants at all and dies
//! to dead-code elimination.  The store is then nowhere in the emitted C.
//!
//! That is usually invisible, because `Merge` then merges the register's
//! HighVariable into the tied location's (the cross-space MULTIEQUAL is a
//! copy-shadow) and both print under one name.  When the merge is DECLINED —
//! the register high already covers other values whose covers intersect the
//! local's — nothing repairs it, and the local's last *printed* assignment is
//! whatever came before the store.  In obfuscated Win32 code that reads:
//!
//! ```text
//! v108 = NULL;
//! v12 = (void *)GetProcAddress(v107,"NtQueryInformationProcess");
//! if (v12) {
//!   v128 = v108;
//!   v7 = (*v108)(v113,0,v27,0x30);      // calls NULL
//! ```
//!
//! The dataflow underneath is correct — the CALLIND's target Varnode really is
//! the `GetProcAddress` result — but the C says the opposite, and an analyst
//! reading it concludes the import is never used.
//!
//! # The brake
//!
//! Upstream already declines this propagation for an `addrforce` output ("Don't
//! propagate if we are keeping the COPY anyway", `ruleaction.cc`), but
//! `addrforce` is set only on heritage's own guard outputs, never on an ordinary
//! frame store.  [`declines`] widens that refusal by exactly one case: a marker
//! is about to consume the LAST reader of an address-tied `COPY` output that
//! holds a **call's return value**.  Propagating there buys nothing (the marker
//! is invisible either way) and costs the store.
//!
//! The predicate is deliberately narrow, and each clause was measured:
//!
//! - **the stored value must come from a call** (`CALL`/`CALLIND`/`CALLOTHER`,
//!   or the `INDIRECT` that carries the return register across the call site
//!   before `ActionActiveReturn` promotes it to the CALL's output).  Without
//!   this clause the brake also keeps ordinary arithmetic stores alive, which
//!   turns 3 upstream datatests' if/else diamonds into `?:` ternaries and
//!   degrades union field resolution: 7/675 assertions move.  With it, 0/675.
//! - **the tied output must not be `persist`.**  A global already has heritage's
//!   own RETURN-COPY (`addrforce`) keeping its last store printed, so the brake
//!   buys nothing there; letting it fire on globals made the `copytrim` datatest
//!   run for minutes instead of 0.16 s.
//! - **the COPY's input must not itself be address-tied**, which is upstream's
//!   own "we must not allow merging of different addrtieds" territory.
//! - **the marker must be the LAST reader.**  While an ordinary reader survives,
//!   the store is alive anyway and upstream's propagation is pure gain.

use kuna_num::opcodes::OpCode;

use crate::context::{OpId, VarnodeId};
use crate::funcdata::Funcdata;

/// Would propagating `invn` into marker `op` (in place of `vn`) orphan the
/// address-tied `COPY` that stores a call's return value into a frame local?
///
/// Called from `RulePropagateCopy`'s marker arm, after its own constant /
/// `addrforce` / differing-addrtied guards have declined to fire.  `vn` is the
/// marker's input being considered and is known to be the output of a
/// `CPUI_COPY` whose input is `invn`.
pub fn declines(data: &Funcdata, op: OpId, vn: VarnodeId, invn: VarnodeId) -> bool {
    if !data.get_arch().tied_store_keep {
        return false;
    }
    let (Some(v), Some(iv)) = (data.vbank().get(vn), data.vbank().get(invn)) else {
        return false;
    };
    if !v.is_addr_tied() || v.is_persist() || iv.is_addr_tied() {
        return false;
    }
    if data.lone_descend(vn) != Some(op) {
        return false;
    }
    let Some(def) = iv.get_def() else {
        return false;
    };
    matches!(
        data.obank().get(def).map(|o| o.code()),
        Some(
            OpCode::CPUI_CALL
                | OpCode::CPUI_CALLIND
                | OpCode::CPUI_CALLOTHER
                | OpCode::CPUI_INDIRECT
        )
    )
}
