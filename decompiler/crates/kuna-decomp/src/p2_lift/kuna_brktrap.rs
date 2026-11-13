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
//! returns a value fabricated from the call (`return (*v1)();`). That return is
//! false for every `brk`: the exception returns to the `brk` itself, never to
//! the caller.
//!
//! # The fix
//!
//! A compiler emits a trap only where control must not continue: `llvm.trap`
//! (clang's `__builtin_trap`, Rust's `abort`, Swift's overflow and bounds
//! checks, and the trap a Darwin or Windows target plants for
//! `__builtin_unreachable`) is `brk #1`, `llvm.ubsantrap` (`-fsanitize-trap`,
//! `-ftrapv`) is `brk #0x55xx`, GCC's `__builtin_trap` is `brk #1000`, the
//! arm64e pointer-authentication failure traps are `brk #0xc470`..`#0xc473`
//! (see [`kuna_ptrauthtrap`](crate::kuna_ptrauthtrap)) and MSVC's `__fastfail`
//! is `brk #0xf003`. [`is_brk_trap_branchind`] matches the lowered shape with
//! one of those immediates and `flow.rs::xref_control_flow` replaces the
//! `BRANCHIND` with the artificial no-return halt a call that never returns
//! gets. The `SoftwareBreakpoint` user-op stays as a statement, and the trap
//! path contributes no return value.
//!
//! # What it will not do
//!
//! * **Only those immediates.** `brk #0xf000` is `__builtin_debugtrap` and
//!   `__debugbreak`, which a debugger steps past (clang puts the function's
//!   `ret` behind it); `brk #0` is a debugger's or Go's `runtime.breakpoint`;
//!   Linux's `brk #0x800` (`BUG`/`WARN`), `#0x400`/`#0x401` (KGDB),
//!   `#0x004`/`#0x005` (kprobes, uprobes), `#0x900`..`#0x9ff` (KASAN) and
//!   `#0x8000`.. (KCFI) can resume. They keep the jump-as-call.
//! * **Only the exact lifted shape.** The `BRANCHIND` must jump through the
//!   storage that a `SoftwareBreakpoint` `CALLOTHER` with a two-byte constant
//!   immediate wrote in the same instruction, with nothing in between writing
//!   it. Only the AArch64 specs define that user-op; elsewhere the lookup of its
//!   index fails and nothing is walked.

use crate::context::OpId;
use crate::funcdata::Funcdata;
use kuna_base::types::uintb;
use kuna_num::opcodes::OpCode;

/// `brk #1`: `llvm.trap`.
pub const LLVM_TRAP: uintb = 0x1;

/// `brk #1000`: GCC's `trap` pattern (`gcc/config/aarch64/aarch64.md`).
pub const GCC_TRAP: uintb = 0x3e8;

/// `brk #0x5500 | kind`: `llvm.ubsantrap`.
pub const UBSAN_TRAP: std::ops::RangeInclusive<uintb> = 0x5500..=0x55ff;

/// `brk #0xf003`: MSVC's `__fastfail`.
pub const WINDOWS_FASTFAIL: uintb = 0xf003;

/// Is `imm` a `brk` immediate a compiler emits for a trap that does not return?
pub fn is_compiler_trap_immediate(imm: uintb) -> bool {
    imm == LLVM_TRAP
        || imm == GCC_TRAP
        || imm == WINDOWS_FASTFAIL
        || UBSAN_TRAP.contains(&imm)
        || crate::kuna_ptrauthtrap::is_ptrauth_trap_immediate(imm)
}

/// Is `op` the `BRANCHIND` half of an AArch64 `brk` with a compiler trap
/// immediate? `breakpoint` is the `SoftwareBreakpoint` user-op's index.
pub fn is_brk_trap_branchind(data: &Funcdata, op: OpId, breakpoint: u32) -> bool {
    brk_immediate(data, op, breakpoint).is_some_and(is_compiler_trap_immediate)
}

/// The immediate of the `brk` whose `BRANCHIND` half `op` is, or `None` when
/// `op` is not one.
///
/// Runs at flow-classification time, before basic blocks exist, so the
/// `CALLOTHER` is found by walking back over the instruction's raw ops, as
/// [`kuna_sparcstructret`](crate::kuna_sparcstructret) does for the same
/// lowering of SPARC's `unimp`.
pub fn brk_immediate(data: &Funcdata, op: OpId, breakpoint: u32) -> Option<uintb> {
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
            if constant(0)?.get_offset() != breakpoint as uintb {
                return None;
            }
            return constant(1).filter(|v| v.get_size() == 2).map(|v| v.get_offset());
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
