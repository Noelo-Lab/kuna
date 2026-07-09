//! Port of `decompiler/cpp/kuna_sparcstructret.{cc,hh}` — SPARC struct-return
//! `unimp N` post-call fall-through (kuna GH-6882, S2 flow-classification).
//!
//! The SPARC ABI for a struct-returning function plants an `unimp <structsize>`
//! after the call site; the SPARC SLEIGH spec lifts `unimp`/`illtrap` to
//! `dest = IllegalInstructionTrap(N); goto [dest];` — a `CPUI_CALLOTHER`
//! (the trap user-op) feeding a `CPUI_BRANCHIND`.  Jump-table recovery cannot
//! resolve the target, so `flow.cc::truncateIndirectJump` turns the BRANCHIND
//! into a non-returning CALLIND and the function loses its tail.  When
//! `option sparcstructret on`, at flow-classification time this exact idiom is
//! recognized and converted to a fall-through (the dead BRANCHIND is removed).
//!
//! ## What this module ports
//!
//! [`kuna_is_sparc_struct_ret_trap`] is a faithful free-function transcription
//! of the C++ `kunaIsSparcStructRetTrap(Funcdata&, const PcodeOp*)` predicate,
//! including its **positional** (pre-SSA) backward walk over the dead/insert
//! list to locate the trap CALLOTHER in the same instruction.  The
//! fall-through rewrite itself is driven by `flow.rs` at
//! `FlowInfo::xrefControlFlow`; this module owns only the *decision*.
//!
//! ## STUB(W4): Rule/Action + ArchOption + user-op wrappers
//!
//! Two architecture-side pieces live on the W4 `Architecture`:
//!
//!   - the **gate** `glb->sparc_struct_return` (default \b false, shipped
//!     `option sparcstructret off` — `architecture.cc:1434`,
//!     `kuna_stages.cc` settableTable): modelled by [`SparcStructRetOption`].
//!   - the **user-op name resolution** `glb->userops.getOp((uint4)id)->getName()`
//!     (the `UserOpManage` table, W4): supplied as a closure
//!     `userop_name: Fn(u32) -> Option<&str>` (`None` == the C++ null
//!     `UserPcodeOp *`).  W4 wires the real `userops.getOp` lookup.
//!
//! The dead/insert-list backward walk (the C++ `op->getInsertIter()` /
//! `data.beginOpDead()` / `--iter`) is ported directly against the op bank's
//! `iter_dead()` ordering (ADR 0001's intrusive insert-list).

use crate::funcdata::Funcdata;
use crate::context::OpId;
use kuna_num::opcodes::OpCode;

/// (kuna) Toggle SPARC struct-return `unimp` BRANCHIND -> fall-through
/// (C++ `OptionSparcStructRet`, GH-6882).
///
/// STUB(W4): the C++ `OptionSparcStructRet::apply` flips
/// `glb->sparc_struct_return`; here the flag is carried as a plain `bool` whose
/// [`Default`] is the *shipped* default (`option sparcstructret off`, i.e.
/// \b false — upstream byte-identical; `architecture.cc:1434`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SparcStructRetOption {
    /// True if the SPARC struct-return `unimp` after a call falls through
    /// instead of becoming a non-returning CALLIND
    /// (C++ `Architecture::sparc_struct_return`).
    pub enabled: bool,
}

impl Default for SparcStructRetOption {
    /// Shipped default: `option sparcstructret off` (upstream byte-identical;
    /// `architecture.cc:1434` sets `sparc_struct_return = false`).
    fn default() -> Self {
        SparcStructRetOption { enabled: false }
    }
}

impl SparcStructRetOption {
    /// (kuna) Set the gate (C++ `OptionSparcStructRet::apply`).
    pub fn apply(&mut self, val: bool) -> &'static str {
        self.enabled = val;
        if val {
            "SPARC struct-return (post-call unimp) fall-through turned on"
        } else {
            "SPARC struct-return (post-call unimp) fall-through turned off"
        }
    }

    /// Read the gate (C++ `glb->sparc_struct_return`).
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

/// The user-op name the SPARC `unimp`/`illtrap` lifting emits.
const ILLEGAL_INSTRUCTION_TRAP: &str = "IllegalInstructionTrap";

/// (kuna) Is `op` a SPARC struct-return `unimp` BRANCHIND that should fall
/// through? (C++ `kunaIsSparcStructRetTrap`, kuna_sparcstructret.cc:14-48, GH-6882).
///
/// `gate` is the resolved `glb->sparc_struct_return` (see
/// [`SparcStructRetOption`]).  `userop_name` resolves a user-op id (the C++
/// `(uint4)idvn->getOffset()`) to `glb->userops.getOp(id)->getName()`
/// (`None` == a null `UserPcodeOp *`).  // STUB(W4)
///
/// The C++ backward dead-list iteration (`op->getInsertIter()` walked with
/// `--iter` until `beginOpDead()`) is ported by locating `op` in the op bank's
/// `iter_dead()` order (the intrusive insert list, ADR 0001) and walking the
/// preceding entries — semantically identical to the C++ `const_iterator`
/// decrement.
pub fn kuna_is_sparc_struct_ret_trap<F>(data: &Funcdata, op: OpId, gate: bool, userop_name: F) -> bool
where
    F: Fn(u32) -> Option<String>,
{
    // default-off gate
    if !gate {
        return false;
    }
    let opref = match data.obank().get(op) {
        Some(o) => o,
        None => return false,
    };
    if opref.code() != OpCode::CPUI_BRANCHIND {
        return false;
    }

    // The dead list in `iter_dead()` order is the C++ `beginOpDead()..` range;
    // `op` sits at some index `pos` in it (the C++ `getInsertIter()`).  We walk
    // `cur` back from `pos` toward `beg` (index 0), exactly as `--iter`.
    let deadlist: Vec<OpId> = data.obank().iter_dead().collect();
    let mut pos = match deadlist.iter().position(|&o| o == op) {
        Some(p) => p,
        // If `op` is not on the dead list the C++ premise (raw flow stage) does
        // not hold; conservatively no match.
        None => return false,
    };

    loop {
        let cur = deadlist[pos];
        let curref = match data.obank().get(cur) {
            Some(o) => o,
            None => return false,
        };
        if curref.code() == OpCode::CPUI_CALLOTHER {
            // first input = user-op id
            if let Some(idvn) = curref.get_in(0) {
                if let Some(vnref) = data.vbank().get(idvn) {
                    if vnref.is_constant() {
                        // (uint4) truncation: the user-op id is a 32-bit index;
                        // the C++ casts the 64-bit constant offset to uint4.
                        let userop_id = vnref.get_offset() as u32;
                        if let Some(name) = userop_name(userop_id) {
                            if name == ILLEGAL_INSTRUCTION_TRAP {
                                return true;
                            }
                        }
                    }
                }
            }
        }
        // reached the first op of this insn
        if curref.is_instruction_start() {
            break;
        }
        // reached the dead-list head
        if pos == 0 {
            break;
        }
        pos -= 1;
    }
    false
}

#[cfg(test)]
#[path = "kuna_sparcstructret/tests.rs"]
mod tests;
