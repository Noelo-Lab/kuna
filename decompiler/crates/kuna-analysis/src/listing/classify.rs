//! Flow classification of a decoded instruction's p-code (design doc §4.3).
//!
//! [`classify`] is a faithful transliteration of the decompiler's single source
//! of truth, `p2_lift/flow.rs::xref_control_flow` (`flow.rs:1039-1185`). It maps
//! the flat p-code op list the SLEIGH decoder emits for one machine instruction
//! into a [`FlowType`], the list of static control targets, and the fall-through
//! VMA. It performs no decode and needs no engine — it is pure over the ops, so
//! it is unit-tested directly (see the tests at the bottom).
//!
//! Three load-bearing rules carried over from `flow.rs` (the design's gotchas):
//!
//!  1. **constant-space `in0` is p-code-relative, not a VMA.** A BRANCH/CBRANCH
//!     whose `in0` is in the constant space is an *intra-instruction* relative
//!     branch (a multi-op instruction branching within itself); for
//!     single-instruction flow it resolves to fall-through and is never a real
//!     target. Only a non-constant `in0` carries an absolute target VMA
//!     (`flow.rs:1067-1109`). Always test `is_constant()` first.
//!  2. **fall-through is decided by the LAST op, not the first.** An
//!     instruction's *targets* come from every op, but its *fall-through* is the
//!     last-op test only: it falls through UNLESS the last op is
//!     BRANCH / BRANCHIND / RETURN (`flow.rs:1170-1183`). An instruction that
//!     emits NO ops falls through.
//!  3. **delay slots are already folded into `len`** by `one_instruction`, so
//!     `fall_vma = vma + len` is correct — never re-decode the delay slot.

use kuna_num::opcodes::OpCode;

use super::model::{FlowKind, FlowType, RawOp};

/// The result of classifying one instruction's p-code.
pub struct Classified {
    /// The instruction's flow type (predicate set + kind).
    pub flow: FlowType,
    /// Static control targets (absolute VMAs): branch/call destinations.
    pub flows: Vec<u64>,
    /// The fall-through VMA, or `None` for a terminal instruction.
    pub fall_through: Option<u64>,
}

/// Classify the p-code `ops` emitted for the instruction at `vma` of byte length
/// `len`. Faithful to `xref_control_flow` (`flow.rs:1039-1185`).
///
/// `flows` collects only *statically derivable* absolute targets: constant-space
/// (p-code-relative) branch operands and indirect (BRANCHIND/CALLIND) ops never
/// contribute a target. The fall-through is the last-op test (gotcha 2).
pub fn classify(ops: &[RawOp], vma: u64, len: u32) -> Classified {
    let fall_vma = vma.wrapping_add(len as u64);
    let mut flows: Vec<u64> = Vec::new();
    let mut flow = FlowType::default();

    // Per-op target collection + kind/predicate accumulation. The op order is
    // observable in flow.rs; we walk every op so a multi-op instruction's
    // targets are all collected (the last-op test below decides fall-through).
    for op in ops {
        match op.opcode {
            OpCode::CPUI_CBRANCH => {
                // Conditional branch: target unless the in0 is p-code-relative
                // (constant space), which resolves to fall-through (gotcha 1).
                if let Some(target) = absolute_branch_target(op) {
                    flows.push(target);
                }
                flow.kind = FlowKind::ConditionalBranch;
                flow.is_jump = true;
                flow.is_conditional = true;
            }
            OpCode::CPUI_BRANCH => {
                if let Some(target) = absolute_branch_target(op) {
                    flows.push(target);
                }
                flow.kind = FlowKind::UnconditionalBranch;
                flow.is_jump = true;
            }
            OpCode::CPUI_BRANCHIND => {
                // Indirect/computed jump: no static target (deferred jump-table
                // resolution, design §8). Recorded as computed/indirect.
                flow.kind = FlowKind::ComputedJump;
                flow.is_jump = true;
                flow.is_computed = true;
                flow.is_indirect = true;
            }
            OpCode::CPUI_CALL => {
                // CALL target == in0 (flow.rs CALL case; the in0 address is the
                // callee entry). It is a code address, never constant-relative.
                if let Some(target) = absolute_branch_target(op) {
                    flows.push(target);
                }
                flow.kind = FlowKind::Call;
                flow.is_call = true;
            }
            OpCode::CPUI_CALLIND => {
                // Indirect call: no static target, but it still falls through.
                flow.kind = FlowKind::ComputedCall;
                flow.is_call = true;
                flow.is_computed = true;
                flow.is_indirect = true;
            }
            OpCode::CPUI_CALLOTHER => {
                flow.kind = FlowKind::CallOther;
            }
            OpCode::CPUI_RETURN => {
                flow.kind = FlowKind::Return;
                flow.is_terminal = true;
            }
            _ => {}
        }
    }

    // Fall-through is the LAST-op test (gotcha 2): falls through unless the last
    // op is BRANCH / BRANCHIND / RETURN. No ops at all ⇒ falls through.
    let has_fallthrough = match ops.last() {
        None => true,
        Some(last) => !matches!(
            last.opcode,
            OpCode::CPUI_BRANCH | OpCode::CPUI_BRANCHIND | OpCode::CPUI_RETURN
        ),
    };
    flow.has_fallthrough = has_fallthrough;
    let fall_through = has_fallthrough.then_some(fall_vma);

    Classified { flow, flows, fall_through }
}

/// The absolute target VMA carried by a branch/call op's `in0`, or `None` if the
/// `in0` is constant-space (p-code-relative, intra-instruction — gotcha 1) or
/// absent.
fn absolute_branch_target(op: &RawOp) -> Option<u64> {
    let in0 = op.in0.as_ref()?;
    let addr = in0.get_addr();
    if addr.is_constant() {
        // p-code-relative: resolves to fall-through, never a real target.
        None
    } else {
        Some(addr.get_offset())
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use kuna_base::space::{spacetype, AddrSpace};
    use kuna_num::opcodes::OpCode;
    use kuna_num::pcoderaw::VarnodeData;

    use super::*;
    use crate::listing::model::FlowKind;

    /// A throwaway processor (RAM) space and the constant space, so we can build
    /// `VarnodeData` whose `in0` is either an absolute code address (RAM) or a
    /// p-code-relative constant.
    fn spaces() -> (Rc<AddrSpace>, Rc<AddrSpace>) {
        let ram = Rc::new(AddrSpace::new_for_decode(spacetype::IPTR_PROCESSOR));
        let cst = Rc::new(AddrSpace::new_for_decode(spacetype::IPTR_CONSTANT));
        (ram, cst)
    }

    /// A varnode at `offset` in the given space, 8 bytes wide.
    fn vn(space: &Rc<AddrSpace>, offset: u64) -> VarnodeData {
        VarnodeData { space: Some(Rc::clone(space)), offset, size: 8 }
    }

    fn op(opcode: OpCode, in0: Option<VarnodeData>) -> RawOp {
        RawOp { opcode, in0 }
    }

    /// CBRANCH to an absolute target: two successors (target + fall-through),
    /// ConditionalBranch, is_conditional set, falls through.
    #[test]
    fn cbranch_target_and_fallthrough() {
        let (ram, _cst) = spaces();
        let ops = vec![op(OpCode::CPUI_CBRANCH, Some(vn(&ram, 0x2000)))];
        let c = classify(&ops, 0x1000, 4);
        assert_eq!(c.flow.kind, FlowKind::ConditionalBranch);
        assert!(c.flow.is_conditional);
        assert!(c.flow.is_jump);
        assert!(c.flow.has_fallthrough);
        assert_eq!(c.flows, vec![0x2000]);
        assert_eq!(c.fall_through, Some(0x1004));
    }

    /// Unconditional BRANCH: the target, NO fall-through.
    #[test]
    fn unconditional_branch_no_fallthrough() {
        let (ram, _cst) = spaces();
        let ops = vec![op(OpCode::CPUI_BRANCH, Some(vn(&ram, 0x3000)))];
        let c = classify(&ops, 0x1000, 4);
        assert_eq!(c.flow.kind, FlowKind::UnconditionalBranch);
        assert!(!c.flow.has_fallthrough);
        assert_eq!(c.flows, vec![0x3000]);
        assert_eq!(c.fall_through, None);
    }

    /// CALL: the callee target, is_call, falls through (a call returns).
    #[test]
    fn call_target_falls_through() {
        let (ram, _cst) = spaces();
        let ops = vec![op(OpCode::CPUI_CALL, Some(vn(&ram, 0x4000)))];
        let c = classify(&ops, 0x1000, 5);
        assert_eq!(c.flow.kind, FlowKind::Call);
        assert!(c.flow.is_call);
        assert!(c.flow.has_fallthrough);
        assert_eq!(c.flows, vec![0x4000]);
        assert_eq!(c.fall_through, Some(0x1005));
    }

    /// RETURN: terminal — no fall-through, no target.
    #[test]
    fn return_is_terminal() {
        let ops = vec![op(OpCode::CPUI_RETURN, None)];
        let c = classify(&ops, 0x1000, 1);
        assert_eq!(c.flow.kind, FlowKind::Return);
        assert!(c.flow.is_terminal);
        assert!(!c.flow.has_fallthrough);
        assert!(c.flows.is_empty());
        assert_eq!(c.fall_through, None);
    }

    /// BRANCHIND: computed/indirect, NO static target, no fall-through.
    #[test]
    fn branchind_is_computed_no_target() {
        let (ram, _cst) = spaces();
        // in0 of a BRANCHIND is a register/temp, not a code address; classify
        // must NOT treat it as a target regardless.
        let ops = vec![op(OpCode::CPUI_BRANCHIND, Some(vn(&ram, 0x9999)))];
        let c = classify(&ops, 0x1000, 2);
        assert_eq!(c.flow.kind, FlowKind::ComputedJump);
        assert!(c.flow.is_computed);
        assert!(c.flow.is_indirect);
        assert!(c.flow.is_jump);
        assert!(c.flows.is_empty(), "BRANCHIND yields no static target");
        assert_eq!(c.fall_through, None);
    }

    /// CALLIND: computed/indirect call, NO static target, but FALLS THROUGH.
    #[test]
    fn callind_is_computed_falls_through() {
        let (ram, _cst) = spaces();
        let ops = vec![op(OpCode::CPUI_CALLIND, Some(vn(&ram, 0x9999)))];
        let c = classify(&ops, 0x1000, 2);
        assert_eq!(c.flow.kind, FlowKind::ComputedCall);
        assert!(c.flow.is_call);
        assert!(c.flow.is_computed);
        assert!(c.flow.is_indirect);
        assert!(c.flows.is_empty(), "CALLIND yields no static target");
        assert_eq!(c.fall_through, Some(0x1002));
    }

    /// Gotcha 1: a constant-space BRANCH `in0` is p-code-relative (intra-insn),
    /// NOT a real target. The instruction still classifies as a branch (so the
    /// last-op test drops the fall-through), but it contributes no `flows` entry.
    #[test]
    fn constant_space_branch_in0_is_not_a_target() {
        let (_ram, cst) = spaces();
        // A relative BRANCH inside a multi-op instruction (offset 1 in the
        // constant space ⇒ "the next p-code op", not VMA 0x1).
        let ops = vec![op(OpCode::CPUI_BRANCH, Some(vn(&cst, 1)))];
        let c = classify(&ops, 0x1000, 4);
        assert_eq!(c.flow.kind, FlowKind::UnconditionalBranch);
        assert!(
            c.flows.is_empty(),
            "a constant-space (p-code-relative) BRANCH in0 must not be a VMA target"
        );
        // Last op is BRANCH ⇒ still no fall-through.
        assert_eq!(c.fall_through, None);
    }

    /// An ordinary multi-op instruction (no flow break): falls through, no
    /// targets, Fallthrough kind.
    #[test]
    fn plain_instruction_falls_through() {
        let (ram, _cst) = spaces();
        let ops = vec![
            op(OpCode::CPUI_COPY, Some(vn(&ram, 0x10))),
            op(OpCode::CPUI_INT_ADD, Some(vn(&ram, 0x10))),
        ];
        let c = classify(&ops, 0x1000, 3);
        assert_eq!(c.flow.kind, FlowKind::Fallthrough);
        assert!(c.flow.has_fallthrough);
        assert!(c.flows.is_empty());
        assert_eq!(c.fall_through, Some(0x1003));
    }

    /// Gotcha 2: a CBRANCH followed by trailing non-flow ops still falls through
    /// (the LAST op is not a flow break), AND keeps its absolute target.
    #[test]
    fn cbranch_then_trailing_op_keeps_fallthrough() {
        let (ram, _cst) = spaces();
        let ops = vec![
            op(OpCode::CPUI_CBRANCH, Some(vn(&ram, 0x2000))),
            op(OpCode::CPUI_COPY, Some(vn(&ram, 0x10))),
        ];
        let c = classify(&ops, 0x1000, 4);
        assert_eq!(c.flow.kind, FlowKind::ConditionalBranch);
        assert_eq!(c.flows, vec![0x2000]);
        assert_eq!(c.fall_through, Some(0x1004));
    }
}
