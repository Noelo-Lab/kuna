//! **kernelbug** — a Linux kernel `WARN`/`BUG` site prints as the statement it
//! is, instead of as a call through `invalidInstructionException()`.
//!
//! ```text
//!   if (a0 < 0) {                               if (a0 < 0)
//!     (*(void (*)(void))                          __WARN("kwarn_bugsite.c",9);
//!       invalidInstructionException())();  -->  if (a0 <= 100)
//!     return;                                     return kwarn_counter + a0;
//!   }                                           BUG();
//!   ...
//!   (*(void (*)(void))invalidInstructionException())();
//! ```
//!
//! # The defect
//!
//! The kernel plants a trap instruction at every `WARN_ON`/`BUG_ON` site — `ud2`
//! on x86, `brk #0x800` on AArch64 — and describes the site in a `__bug_table`
//! entry (`struct bug_entry`: the trap's address, the source file and line, and a
//! flags word whose bit 0, `BUGFLAG_WARNING`, says the handler resumes after the
//! trap). The trap handler looks the faulting address up and either prints the
//! warning and skips the instruction, or panics. SLEIGH lowers both traps to a
//! jump through a user-op's result (`ia.sinc`, `:UD2`: `target =
//! invalidInstructionException(); goto [target];`), jump-table recovery cannot
//! resolve that BRANCHIND, and `truncate_indirect_jump` makes it a call followed
//! by a return. Every site then prints as a call through a function pointer that
//! returns, which is false both ways: a `WARN` site falls through to the next
//! instruction and never returns, and a `BUG` site never continues at all.
//!
//! # The fix
//!
//! The analysis tier reads `__bug_table` ([`kuna_analysis::kernelbug`]) and parks
//! one [`KernelBugSite`] per entry on the architecture. At flow time
//! `FlowInfo::xref_control_flow` asks [`site_producer`] whether a BRANCHIND is
//! the jump half of a trap at a parked address, and [`rewrite`] turns the
//! producing CALLOTHER into one of two built-in user-ops: `__WARN(file,line)` —
//! the kernel's own spelling, with the file as a `char *` constant so it prints
//! as the literal — after which the flow falls through to the next instruction,
//! or `BUG()`, after which the flow plants the artificial no-return halt a call
//! that never returns gets ([`kuna_brktrap`](crate::kuna_brktrap)). A table
//! without `CONFIG_DEBUG_BUGVERBOSE` has no file or line, and the statement
//! prints bare.
//!
//! # What it will not do
//!
//! * **Only table entries.** A `ud2` or `brk #0x800` the table does not name
//!   (an `__builtin_trap`, a KGDB or kprobes breakpoint) keeps its rendering.
//! * **Only the exact lifted shape.** The BRANCHIND must jump through exactly
//!   the storage the trap's CALLOTHER wrote in the same instruction
//!   (`kuna_brktrap::brk_site`'s walk).

use crate::context::OpId;
use crate::funcdata::Funcdata;
use crate::userop::{BUILTIN_KERNEL_BUG, BUILTIN_KERNEL_WARN};
use kuna_base::error::KunaResult;
use kuna_base::types::int4;

/// The AArch64 `brk` immediate Linux uses for `BUG`/`WARN` (`BUG_BRK_IMM`).
pub const LINUX_BUG_BRK_IMM: u64 = 0x800;

/// The x86 trap user-op `ud0`/`ud1`/`ud2` jump through.
pub const INVALID_INSTRUCTION_USEROP: &[u8] = b"invalidInstructionException";

/// One `__bug_table` entry: the trap at `addr` and what the handler does there.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KernelBugSite {
    /// The trap instruction's address.
    pub addr: u64,
    /// `BUGFLAG_WARNING`: the handler resumes after the trap.
    pub warning: bool,
    /// The source file name's address, when the table carries one.
    pub file: Option<u64>,
    /// The source line, when the table carries one.
    pub line: Option<u32>,
}

/// The parked site at `addr`, if any; `sites` is sorted by address.
pub fn site_at(sites: &[KernelBugSite], addr: u64) -> Option<&KernelBugSite> {
    sites.binary_search_by_key(&addr, |s| s.addr).ok().map(|i| &sites[i])
}

/// The CALLOTHER whose output the BRANCHIND `op` jumps through, when both
/// belong to the same instruction and the CALLOTHER is the trap user-op:
/// `invalidInstructionException` (`invalid`) or an AArch64 `SoftwareBreakpoint`
/// (`breakpoint`) with the kernel's `brk` immediate.
pub fn site_producer(
    data: &Funcdata,
    op: OpId,
    invalid: Option<u32>,
    breakpoint: Option<u32>,
) -> Option<OpId> {
    if let Some(bp) = breakpoint {
        if let Some((callother, imm)) = crate::kuna_brktrap::brk_site(data, op, bp) {
            return (imm == LINUX_BUG_BRK_IMM).then_some(callother);
        }
    }
    let invalid = invalid?;
    let id = crate::kuna_brktrap::trap_producer(data, op)?;
    let prev = data.obank().get(id)?;
    if prev.num_input() != 1 {
        return None;
    }
    let index = prev.get_in(0).and_then(|v| data.vbank().get(v)).filter(|v| v.is_constant())?;
    (index.get_offset() == invalid as u64).then_some(id)
}

/// Turn the trap's CALLOTHER into `BUG()` or `__WARN(file,line)`: the user-op
/// index becomes the built-in's, the trap's own inputs go, and a `WARN` site
/// gains its file (a pointer constant as wide as the jump target the BRANCHIND
/// `branch` reads) and line.
pub fn rewrite(data: &mut Funcdata, branch: OpId, callother: OpId, site: &KernelBugSite) -> KunaResult<()> {
    let ptr_size: int4 = data
        .obank()
        .get(branch)
        .and_then(|o| o.get_in(0))
        .and_then(|v| data.vbank().get(v))
        .map_or(8, |v| v.get_size());
    let n = data.obank().get(callother).expect("kernelbug: stale callother").num_input();
    for slot in (1..n).rev() {
        data.op_remove_input(callother, slot);
    }
    let builtin = if site.warning { BUILTIN_KERNEL_WARN } else { BUILTIN_KERNEL_BUG };
    let index = data.new_constant(4, builtin as u64);
    data.op_set_input(callother, index, 0)?;
    if site.warning {
        if let Some(file) = site.file {
            let vn = data.new_constant(ptr_size, file);
            data.op_insert_input(callother, vn, 1)?;
        }
        if let Some(line) = site.line {
            let slot = data.obank().get(callother).expect("kernelbug: stale callother").num_input();
            let vn = data.new_constant(4, line as u64);
            data.op_insert_input(callother, vn, slot)?;
        }
    }
    Ok(())
}

#[cfg(test)]
#[path = "kuna_kernelbug/tests.rs"]
mod tests;
