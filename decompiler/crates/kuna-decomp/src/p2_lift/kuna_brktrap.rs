//! **brktrap** — an AArch64 `brk` that a compiler emits as a trap ends the flow,
//! instead of printing as a call through `SoftwareBreakpoint` that returns.
//!
//! ```text
//!   void f(long a0,long a1)                  long f(long a0,long a1)
//!   {                                        {
//!     if (!SCARRY8(a0,a1))                     if (!SCARRY8(a0,a1))
//!       return;                       -->        return a0 + a1;
//!     (*(void (*)(long))                       SoftwareBreakpoint(1,0x100010);
//!       SoftwareBreakpoint(1,0x100010))      }
//!       (a0 + a1); // jump-as-call
//!   }
//! ```
//!
//! # The defect
//!
//! AArch64 SLEIGH lowers `BRK #imm16` to a user-op whose result is jumped
//! through (`AARCH64base.sinc`, `:brk`):
//!
//! ```text
//!   pc = SoftwareBreakpoint(imm16, inst_next);
//!   goto [pc];
//! ```
//!
//! Jump-table recovery cannot resolve that `BRANCHIND`, so
//! `FlowInfo::truncate_indirect_jump` turns it into a `CALLIND` followed by an
//! ordinary return, and the trap path returns whatever that call leaves in
//! `x0`. Return recovery then either drops the function's return value (it
//! prints as `void`, with the value passed to the trap as an argument) or
//! returns a value fabricated from the call (`return (*v1)();`).
//!
//! # The fix
//!
//! A compiler emits a trap only where control must not continue: `llvm.trap`
//! (clang's `__builtin_trap`, and Swift's overflow and bounds checks) is
//! `brk #1`, `llvm.ubsantrap` (`-fsanitize-trap`, `-ftrapv`) is `brk #0x55xx`,
//! and GCC's `__builtin_trap` is `brk #1000`. Under
//! `option brktrap on` (the default), [`is_brk_trap_branchind`] matches the
//! lowered shape with one of those immediates and
//! `flow.rs::xref_control_flow` replaces the `BRANCHIND` with the artificial
//! no-return halt a call that never returns gets. The `SoftwareBreakpoint`
//! user-op stays as a statement, and the trap path contributes no return value.
//!
//! # What it will not do
//!
//! * **Only those immediates.** `brk #0xf000` is `__builtin_debugtrap`, after
//!   which execution continues (clang puts the function's `ret` behind it),
//!   `brk #0` is a debugger breakpoint, and `brk #0xc470`..`#0xc473` are the
//!   arm64e pointer-authentication failure traps. They keep the jump-as-call.
//! * **Only the exact lifted shape.** The `BRANCHIND` must jump through the
//!   storage that a `SoftwareBreakpoint` `CALLOTHER` with a two-byte constant
//!   immediate wrote in the same instruction, with nothing in between writing
//!   it. Only the AArch64 specs define that user-op, so no language gate is
//!   needed.

use crate::context::OpId;
use crate::funcdata::Funcdata;
use kuna_base::types::uintb;
use kuna_num::opcodes::OpCode;

/// The user-op AArch64 SLEIGH lowers `brk` through (`AARCH64instructions.sinc`).
pub const BREAKPOINT_USEROP: &str = "SoftwareBreakpoint";

/// `brk #1`: `llvm.trap`.
pub const LLVM_TRAP: uintb = 0x1;

/// `brk #1000`: GCC's `trap` pattern (`gcc/config/aarch64/aarch64.md`).
pub const GCC_TRAP: uintb = 0x3e8;

/// `brk #0x5500 | kind`: `llvm.ubsantrap`.
pub const UBSAN_TRAP: std::ops::RangeInclusive<uintb> = 0x5500..=0x55ff;

/// Is `imm` a `brk` immediate a compiler emits for a trap that does not return?
pub fn is_compiler_trap_immediate(imm: uintb) -> bool {
    imm == LLVM_TRAP || imm == GCC_TRAP || UBSAN_TRAP.contains(&imm)
}

/// Is `op` the `BRANCHIND` half of an AArch64 `brk` with a compiler trap
/// immediate?
pub fn is_brk_trap_branchind<F>(data: &Funcdata, op: OpId, userop_name: F) -> bool
where
    F: Fn(u32) -> Option<String>,
{
    brk_immediate(data, op, userop_name).is_some_and(is_compiler_trap_immediate)
}

/// The immediate of the `brk` whose `BRANCHIND` half `op` is, or `None` when
/// `op` is not one.
///
/// Runs at flow-classification time, before basic blocks exist, so the
/// `CALLOTHER` is found by walking back over the instruction's raw ops, as
/// [`kuna_sparcstructret`](crate::kuna_sparcstructret) does for the same
/// lowering of SPARC's `unimp`.
pub fn brk_immediate<F>(data: &Funcdata, op: OpId, userop_name: F) -> Option<uintb>
where
    F: Fn(u32) -> Option<String>,
{
    let branch = data.obank().get(op)?;
    if branch.code() != OpCode::CPUI_BRANCHIND || !data.obank().on_dead_list(op) {
        return None;
    }
    let target = branch.get_in(0).and_then(|v| data.vbank().get(v))?;
    let insn = branch.get_addr();
    let mut cur = data.obank().dead_prev(op);
    while let Some(id) = cur {
        let prev = data.obank().get(id)?;
        if prev.get_addr() != insn {
            return None;
        }
        let out = prev.get_out().and_then(|v| data.vbank().get(v));
        if let Some(out) = out.filter(|v| v.intersects(target)) {
            let exact = out.get_addr() == target.get_addr() && out.get_size() == target.get_size();
            if !exact || prev.code() != OpCode::CPUI_CALLOTHER || prev.num_input() != 3 {
                return None;
            }
            let constant = |slot| prev.get_in(slot).and_then(|v| data.vbank().get(v)).filter(|v| v.is_constant());
            let userop = constant(0).and_then(|v| userop_name(v.get_offset() as u32))?;
            let imm = constant(1).filter(|v| v.get_size() == 2)?;
            return (userop == BREAKPOINT_USEROP).then(|| imm.get_offset());
        }
        if prev.is_instruction_start() {
            return None;
        }
        cur = data.obank().dead_prev(id);
    }
    None
}

#[cfg(test)]
#[path = "kuna_brktrap/tests.rs"]
mod tests;
