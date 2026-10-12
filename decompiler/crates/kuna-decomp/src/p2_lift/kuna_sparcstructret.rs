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
//! recognized and converted to a fall-through: the trap producer becomes an
//! inert COPY and the dead BRANCHIND is removed.
//!
//! ## What this module ports
//!
//! [`kuna_sparc_struct_ret_trap_producer`] returns the producer recognized by
//! the C++ `kunaIsSparcStructRetTrap(Funcdata&, const PcodeOp*)` predicate,
//! including its **positional** (pre-SSA) backward walk over the dead/insert
//! list to locate the trap CALLOTHER in the same instruction.  The
//! fall-through rewrite itself is driven by `flow.rs` at
//! `FlowInfo::xrefControlFlow`; this module also neutralizes the trap producer.
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

/// Neutralize a recognized producer when its output is available; otherwise leave it intact.
pub fn neutralize_trap_producer(data: &mut Funcdata, producer: OpId) -> kuna_base::error::KunaResult<()> {
    let Some(op) = data.obank().get(producer) else {
        return Ok(());
    };
    if op.code() != OpCode::CPUI_CALLOTHER {
        return Ok(());
    }
    let Some(size) = op.get_out().and_then(|v| data.vbank().get(v)).map(|v| v.get_size()) else {
        return Ok(());
    };
    let zero = data.new_constant(size, 0);
    data.op_set_opcode_code(producer, OpCode::CPUI_COPY);
    data.op_set_all_input(producer, &[zero])?;
    Ok(())
}

/// Find the IllegalInstructionTrap producer within the branch's instruction.
pub fn kuna_sparc_struct_ret_trap_producer<F>(data: &Funcdata, branch: OpId, gate: bool, userop_name: F) -> Option<OpId>
where
    F: Fn(u32) -> Option<String>,
{
    if !gate || !data.obank().on_dead_list(branch) {
        return None;
    }
    if data.obank().get(branch)?.code() != OpCode::CPUI_BRANCHIND {
        return None;
    }
    let mut current = Some(branch);
    while let Some(id) = current {
        let op = data.obank().get(id)?;
        if op.code() == OpCode::CPUI_CALLOTHER {
            if let Some(input) = op.get_in(0).and_then(|v| data.vbank().get(v)) {
                if input.is_constant()
                    && userop_name(input.get_offset() as u32).as_deref() == Some(ILLEGAL_INSTRUCTION_TRAP)
                {
                    return Some(id);
                }
            }
        }
        if op.is_instruction_start() {
            break;
        }
        current = data.obank().dead_prev(id);
    }
    None
}

#[cfg(test)]
#[path = "kuna_sparcstructret/tests.rs"]
mod tests;
