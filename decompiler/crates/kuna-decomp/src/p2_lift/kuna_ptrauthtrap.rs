//! **ptrauthtrap** — drop the pointer-authentication failure checks an arm64e
//! compiler plants after an `aut*` instruction.
//!
//! ```text
//!   ldp   x29, x30, [sp], #16
//!   autibsp                          ;  if ((x30 ^ x30 << 1) >> 0x3e & 1) {
//!   eor   x16, x30, x30, lsl #1      ;    (*SoftwareBreakpoint(0xc471,..))();
//!   tbz   x16, #62, ok               ;    return;                        -->  helper2();
//!   brk   #0xc471                    ;  }
//! ok:                                ;  helper2();
//!   b     helper2
//! ```
//!
//! On a CPU without `FEAT_FPAC` a failed `aut*` does not trap: it returns the
//! pointer with an error code in its top bits and the fault only comes at the
//! next use. Apple's clang therefore checks the result inline: before a tail
//! call it tests that bits 62 and 61 of the authenticated `x30` agree (the
//! `eor`/`tbz` pair), and after a data `aut*` it compares the result with its
//! `xpac`-stripped copy. The failure arm is a `brk #0xc470 + key`, an immediate
//! LLVM reserves for exactly this trap. SLEIGH hides the `aut*` itself (the
//! `ShowPAC` context is off), so a statically decompiled pointer always
//! authenticates, and the check is noise: the `xpac` form folds away on its own,
//! but the bit test reads the incoming return address, which nothing can fold.
//! Apple binaries carry one such check before nearly every tail call (205 in
//! iOS 16's `trustd`, 1,017 in `AuthKit`).
//!
//! # Matching
//!
//! The trap block is the anchor: a basic block lifted entirely from one `brk`
//! whose `SoftwareBreakpoint` `CALLOTHER` carries an immediate in
//! `0xc470..=0xc473`. Every in-edge that is one arm of a `CBRANCH` is cut; the
//! condition feeding it dies to dead-code elimination. A trap block entered any
//! other way keeps that entry.
//!
//! # The edit, and the window
//!
//! `remove_branch` plus the unreachable-block sweep, pre-SSA
//! (`get_heritage_pass() == 0`), the `gomorestack` window.
//!
//! # The option
//!
//! `option ptrauthtrap on|off`, default **on**. Off keeps the checks for
//! auditing. Destructive by declared intent: it deletes a real branch.

use kuna_base::types::{int4, uint4};
use kuna_num::opcodes::OpCode;

use crate::action::{Action, ActionBase, ActionContext, ActionGroupList, ApplyResult};
use crate::block::BlockKind;
use crate::context::BlockId;
use crate::funcdata::Funcdata;

/// The SLEIGH user-op `brk` lowers to (`AARCH64base.sinc`, `:brk`).
pub const BREAKPOINT_USEROP: &[u8] = b"SoftwareBreakpoint";

/// Is `imm` one of the `brk` immediates LLVM reserves for a pointer-auth
/// failure (`0xc470 | key`, keys IA, IB, DA, DB)?
pub fn is_ptrauth_trap_immediate(imm: u64) -> bool {
    (0xc470..=0xc473).contains(&imm)
}

/// Is `bl` a basic block lifted from nothing but one pointer-auth trap `brk`?
fn is_trap_block(data: &Funcdata, bl: BlockId, userop: uint4) -> bool {
    if !matches!(data.bblocks_ref().block(bl).kind(), BlockKind::Basic(_)) {
        return false;
    }
    let mut cur = data.bb_op_head(bl);
    let mut insn = None;
    let mut trap = false;
    while let Some(id) = cur {
        let Some(op) = data.obank().get(id) else { return false };
        match &insn {
            None => insn = Some(op.get_addr().clone()),
            Some(a) if a != op.get_addr() => return false,
            Some(_) => {}
        }
        if op.code() == OpCode::CPUI_CALLOTHER && op.num_input() >= 2 {
            let constant = |slot| {
                op.get_in(slot)
                    .and_then(|v| data.vbank().get(v))
                    .filter(|v| v.is_constant())
                    .map(|v| v.get_offset())
            };
            trap |= constant(0) == Some(userop as u64) && constant(1).is_some_and(is_ptrauth_trap_immediate);
        }
        cur = data.bb_op_next(id);
    }
    trap
}

/// The `(guard block, out-edge)` pairs of every `CBRANCH` that branches into a
/// pointer-auth trap block. A guard whose both arms trap keeps both: there is
/// no surviving arm to fall through to.
fn trap_edges(data: &Funcdata, userop: uint4) -> Vec<(BlockId, int4)> {
    let blocks = data.bblocks_ref();
    let mut out = Vec::new();
    let Some(root) = blocks.root else { return out };
    for &trap in blocks.block(root).get_list() {
        if !is_trap_block(data, trap, userop) {
            continue;
        }
        for k in 0..blocks.block(trap).size_in() {
            let guard = blocks.block(trap).get_in(k);
            if blocks.block(guard).size_out() != 2 {
                continue;
            }
            let is_cbranch = data
                .bb_op_tail(guard)
                .and_then(|t| data.obank().get(t))
                .is_some_and(|t| t.code() == OpCode::CPUI_CBRANCH);
            let edges: Vec<int4> = (0..2).filter(|&j| blocks.block(guard).get_out(j) == trap).collect();
            if is_cbranch && edges.len() == 1 {
                out.push((guard, edges[0]));
            }
        }
    }
    let mut per_guard: std::collections::HashMap<BlockId, usize> = std::collections::HashMap::new();
    for &(g, _) in &out {
        *per_guard.entry(g).or_default() += 1;
    }
    out.retain(|(g, _)| per_guard[g] == 1);
    out
}

/// (kuna) `ActionPtrAuthTrap` — cut the branch into every pointer-auth
/// failure trap (option `ptrauthtrap`, default on).
pub struct ActionPtrAuthTrap {
    base: ActionBase,
}

impl ActionPtrAuthTrap {
    /// Construct the action in the given group.
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionPtrAuthTrap { base: ActionBase::new(0, "ptrauthtrap", g) })
    }
}

impl Action for ActionPtrAuthTrap {
    fn base(&self) -> &ActionBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ActionBase {
        &mut self.base
    }
    fn clone_filtered(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Action>> {
        if !grouplist.contains(self.get_group()) {
            return None;
        }
        Some(Box::new(ActionPtrAuthTrap { base: self.base.clone() }))
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        let Some(userop) = data.get_arch().ptrauth_trap_userop else { return 0 };
        if data.get_heritage_pass() != 0 {
            return 0;
        }
        let edges = trap_edges(data, userop);
        if edges.is_empty() {
            return 0;
        }
        for (guard, edge) in edges {
            data.remove_branch(guard, edge).expect("ActionPtrAuthTrap: removeBranch");
            self.base.count += 1;
        }
        data.remove_unreachable_blocks(false, true)
            .expect("ActionPtrAuthTrap: removeUnreachableBlocks");
        1
    }
}

#[cfg(test)]
#[path = "kuna_ptrauthtrap/tests.rs"]
mod tests;
