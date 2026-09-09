//! P3 loop-counter store brake — the `loopcounterstore` decision point.
//!
//! # The gap
//!
//! A counter that the compiler keeps in a frame slot is read, bumped in a
//! register and written back:
//!
//! ```text
//! 0x1400035f6  MOV EAX,dword ptr [RSP + 0x84]
//! 0x1400035fd  INC EAX
//! 0x1400035ff  MOV dword ptr [RSP + 0x84],EAX
//! 0x140003606  CMP dword ptr [RSP + 0x84],0x5
//! ```
//!
//! After heritage the write-back is an address-tied `stackvn = COPY(EAX)` whose
//! only reader is the loop header's `MULTIEQUAL`, and that `MULTIEQUAL`'s output
//! is the same frame slot.  `RulePropagateCopy` (`ruleaction.cc:3943`) rewrites
//! the marker to read `EAX` directly, the `COPY` loses its last descendant and
//! dies to dead-code elimination, and the loop header is left as
//! `slot = MULTIEQUAL(init, EAX)` with `EAX = slot + 1`.
//!
//! Nothing then prints the store *at the slot*.  What prints is the register's
//! own def, under the register's HighVariable — so the C reads
//!
//! ```text
//! for (v55 = 0; v55 <= 4; v7 = v55 + 1) { ... }
//! ```
//!
//! which is not merely mis-named: as written the induction variable is never
//! updated and the loop does not terminate.  `Merge` usually hides this by
//! merging the register high into the tied high, but in obfuscated code where
//! one register carries several counters and a few call returns those covers
//! intersect, the merge is declined, and five loops in one function come out
//! this way.
//!
//! # The brake
//!
//! [`declines`] refuses that one propagation.  The shape it recognises is the
//! self-updating tied phi and nothing else:
//!
//! - the marker is a `MULTIEQUAL` whose output is address-tied at **exactly the
//!   storage of the `COPY`'s output** — same address, same size.  The phi
//!   already names the frame slot, so propagating a register into it cannot
//!   make the slot's def printable; it can only take the store away.
//! - the stored value is **computed from that phi output**: some input of
//!   `invn`'s defining op is the `MULTIEQUAL`'s own output.  This is what makes
//!   it a counter rather than a value that merely happens to land in the slot,
//!   and it is the clause that keeps the brake off ordinary stores — without it
//!   a call return stored to a frame local also gets kept, which re-prints
//!   `v134 = _time64(0)` as two statements and un-folds `v43 -= 0x2a` into
//!   `v43 = v43 - 0x2a` on the witness alone.
//! - the `COPY`'s output is address-tied and not `persist`, its input is not
//!   address-tied, and the marker is the **last** reader — the same three
//!   guards [`crate::p3_dataflow::kuna_tiedstorekeep`] carries, for the same
//!   reasons (a global keeps its last store printed by heritage's own
//!   `addrforce` RETURN-COPY, and while an ordinary reader survives the store is
//!   alive anyway and upstream's propagation is pure gain).
//!
//! `tiedstorekeep` does not cover this: its predicate requires the stored value
//! to come from a `CALL`/`CALLIND`/`CALLOTHER`/`INDIRECT`, and a counter bump is
//! an `INT_ADD`.  Flipping it either way leaves the loops above byte-identical.
//!
//! # The printer half
//!
//! Where `Merge` would have succeeded anyway, the brake keeps a `COPY` the
//! output did not need, and that `COPY` becomes the statement's root op with the
//! arithmetic hanging off it as an implied expression.  `PrintC::emitInplaceOp`
//! matches only a bare two-input op, so 85 statements across the arena corpus
//! came out as `v = v + 1` instead of `v += 1` — correct C, and exactly what
//! upstream Ghidra prints, but a silent loss of kuna's DIV-36 sugar in loops
//! that were never broken.  `PrintC::inplace_through_copy` closes that: a `COPY`
//! of an **implied** two-input value is rendered from the inner op.  An explicit
//! input is left alone, because there the statement really is `out = <name>`.

use kuna_num::opcodes::OpCode;
use kuna_base::space::spacetype;

use crate::context::{OpId, VarnodeId};
use crate::funcdata::Funcdata;

/// Would propagating `invn` into marker `op` (in place of `vn`) delete the
/// write-back of a loop counter that lives in an address-tied frame slot?
///
/// Called from `RulePropagateCopy`'s marker arm, after its own constant /
/// `addrforce` / differing-addrtied guards and after
/// [`crate::p3_dataflow::kuna_tiedstorekeep::declines`] have declined to fire.
/// `vn` is the marker's input under consideration and is known to be the output
/// of a `CPUI_COPY` whose input is `invn`.
pub fn declines(data: &Funcdata, op: OpId, vn: VarnodeId, invn: VarnodeId) -> bool {
    if !data.get_arch().loop_counter_store {
        return false;
    }
    let (Some(v), Some(iv)) = (data.vbank().get(vn), data.vbank().get(invn)) else {
        return false;
    };
    if !v.is_addr_tied() || v.is_persist() || iv.is_addr_tied() {
        return false;
    }
    if iv.get_space().get_type() != spacetype::IPTR_PROCESSOR {
        return false;
    }
    let Some(marker) = data.obank().get(op) else {
        return false;
    };
    if marker.code() != OpCode::CPUI_MULTIEQUAL {
        return false;
    }
    if data.lone_descend(vn) != Some(op) {
        return false;
    }
    let Some(out) = marker.get_out() else {
        return false;
    };
    let Some(ov) = data.vbank().get(out) else {
        return false;
    };
    if !ov.is_addr_tied() || ov.get_addr() != v.get_addr() || ov.get_size() != v.get_size() {
        return false;
    }
    let Some(def) = iv.get_def().and_then(|d| data.obank().get(d)) else {
        return false;
    };
    (0..def.num_input())
        .filter_map(|i| def.get_in(i))
        .any(|x| resolves_to(data, x, out))
}

/// Does `x` reach `target` through a chain of `CPUI_COPY`s?
///
/// `RulePropagateCopy` fires in an order kuna does not control, so at the moment
/// the counter's write-back is under consideration the load side may or may not
/// have been folded yet: the increment reads either the phi output directly or a
/// register `COPY` of it.  Both are the same counter, so the predicate must see
/// through the chain, or whether the brake fires depends on which of the two
/// propagations the rule pool reached first.
fn resolves_to(data: &Funcdata, x: VarnodeId, target: VarnodeId) -> bool {
    let mut cur = x;
    for _ in 0..COPY_CHAIN_LIMIT {
        if cur == target {
            return true;
        }
        let Some(def) = data.vbank().get(cur).and_then(|v| v.get_def()) else {
            return false;
        };
        let Some(op) = data.obank().get(def) else {
            return false;
        };
        if op.code() != OpCode::CPUI_COPY {
            return false;
        }
        let Some(next) = op.get_in(0) else {
            return false;
        };
        cur = next;
    }
    false
}

/// Depth bound on [`resolves_to`]'s `COPY` walk.  A counter's load side is one
/// `COPY` in every form seen so far; the slack is for a lifter that routes the
/// read through a cast temporary as well.
const COPY_CHAIN_LIMIT: usize = 4;
