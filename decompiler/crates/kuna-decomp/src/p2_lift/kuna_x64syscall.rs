//! (kuna `x64syscall`) Give the x86-64 `SYSCALL` user-op the Linux ABI's
//! register effects, so a syscall wrapper stops returning its own first
//! argument.
//!
//! # The defect
//!
//! `specs/Ghidra/Processors/x86/data/languages/ia.sinc:4836` lowers the
//! instruction as
//!
//! ```text
//! RCX = inst_next; R11 = rflags; syscall();
//! ```
//!
//! — a `CALLOTHER` with **no inputs and no output**. That single fact produces
//! both halves of the damage on the ordinary three-argument wrapper
//!
//! ```text
//! MOV RAX,[RBP-0x18]   ; number
//! MOV RDX,[RBP-0x20]
//! MOV RSI,[RBP-0x28]
//! MOV RDI,RDX
//! SYSCALL
//! MOV [RBP-0x8],RAX
//! ```
//!
//! No inputs, so the four register writes that set the call up have no reader
//! and the dead-code fixpoint collects them: the number and every argument are
//! erased and the op prints as `syscall()` with empty parens. No output, so
//! nothing redefines `RAX` across the instruction and the `RAX` the store reads
//! is still the one loaded from `number` — the function is emitted as
//! `syscall(); return number;`. The return value is not an invented one; it is
//! correct copy propagation over an operation kuna has been told writes nothing,
//! which makes the emitted C a *false statement about the program*.
//!
//! `linuxsyscall` does not cover this and cannot be extended to: it resolves the
//! number to a constant and retargets a `CALLIND`, and here there is no `CALLIND`
//! (a bare `CALLOTHER`, not the `int 0x80` `intloc = swi(0x80); call [intloc]`
//! pair) and no constant (the number is an incoming parameter). It is also gated
//! to x86-32. The fix has to be number-independent, which is what this is.
//!
//! # The fix
//!
//! Rewrite the `CALLOTHER` in place, before heritage, giving it the storage the
//! x86-64 Linux kernel entry actually reads and writes: `RAX` in as the number,
//! the argument registers `RDI, RSI, RDX, R10, R8, R9` in, and `RAX` back out.
//! The reads keep the setup stores alive, and the write redefines `RAX` so the
//! `return number;` dies. `RCX`/`R11` are already written by the SLEIGH
//! constructor itself and are left alone.
//!
//! # Which arguments
//!
//! With a runtime number, nothing can say how many of the six the callee reads,
//! and the two defensible answers differ, so the option carries both:
//!
//! - **`on`** takes an argument register only where a bounded backward walk of
//!   the `CALLOTHER`'s own basic block finds an op writing it before the
//!   instruction — the setup the assembler wrote, read syntactically, since
//!   pre-SSA there is no def-use edge to follow. Every read it adds is of a value
//!   the function demonstrably places there, so no undefined register is ever
//!   introduced; the cost is that a wrapper whose setup sits in a predecessor
//!   block keeps today's rendering for that argument.
//! - **`abi`** takes all six unconditionally, which is what the kernel contract
//!   permits it to read. It is the sound over-approximation and the answer for a
//!   wrapper built out of several blocks, but a register the function never
//!   writes then reads as an undefined input, which can add a parameter the
//!   source has not got.
//!
//! `RAX` is read and written under both: the number is in it by definition, and
//! the output is the half that retires the false return.
//!
//! # Where it sits
//!
//! An `Action` in the once-per-function block beside
//! [`ActionLinuxSyscall`](crate::kuna_linuxsyscall::ActionLinuxSyscall), for the
//! same reason — the pattern is read off the raw p-code, and adding a register
//! read or write is only legal before heritage builds SSA. A `CALLOTHER` a
//! compiler spec has already specialized with a `<callotherfixup>` is left alone;
//! a spec-declared model wins, exactly as it does for `cortexmpriv`.

use kuna_base::address::Address;
use kuna_base::marshal::ElementId;
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::action::{Action, ActionBase, ActionContext, ActionGroupList, ApplyResult};
use crate::context::OpId;
use kuna_base::error::{KunaError, KunaResult};
use crate::funcdata::Funcdata;

/// Marshaling element `<x64syscall>` (kuna). ElementIds live in the 4000+ range;
/// 4160 was the previous max.
pub const ELEM_X64SYSCALL: ElementId = ElementId::new("x64syscall", 4161);

/// The vendored x86 user-op the 64-bit `SYSCALL` constructor emits
/// (`ia.sinc:4838`).
pub const USEROP_NAME: &[u8] = b"syscall";

/// The register carrying the syscall number on the way in and the result on the
/// way out.
pub const NUM_REGISTER: &str = "RAX";

/// The x86-64 Linux syscall argument registers, in ABI order. `RCX` is absent on
/// purpose: the instruction clobbers it with the return address, which is why
/// the kernel ABI uses `R10` where the C ABI would use `RCX`.
pub const ARG_REGISTERS: [&str; 6] = ["RDI", "RSI", "RDX", "R10", "R8", "R9"];

/// Every ABI register is read and written at this width.
const REGISTER_SIZE: int4 = 8;

/// How far back through the block the setup walk looks for one argument
/// register. Generous next to the four-instruction setup a wrapper actually
/// emits, and bounded so a long block cannot make the pass quadratic.
const SETUP_WALK_LIMIT: usize = 64;

/// (kuna) What effects to give the `SYSCALL` user-op: `x64syscall off|on|abi`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum X64SyscallMode {
    /// Upstream: a zero-input, zero-output `CALLOTHER`.
    #[default]
    Off,
    /// Read `RAX` and every argument register the block provably sets up; write
    /// `RAX`.
    On,
    /// Read `RAX` and all six argument registers; write `RAX`.
    Abi,
}

impl X64SyscallMode {
    /// Is the rewrite applied at all?
    pub fn rewrites(self) -> bool {
        !matches!(self, X64SyscallMode::Off)
    }

    /// Are all six argument registers taken without asking the block?
    pub fn takes_all_args(self) -> bool {
        matches!(self, X64SyscallMode::Abi)
    }
}

/// (kuna) Choose the `SYSCALL` effect model: `x64syscall off|on|abi`.
///
/// Following the kuna-option idiom
/// ([`OptionInt3Pad`](crate::kuna_int3pad::OptionInt3Pad)), this struct owns only
/// the parse; the caller (`Architecture::set_kuna_option`) writes the live field.
pub struct OptionX64Syscall;

impl OptionX64Syscall {
    /// The option name.
    pub const NAME: &'static str = "x64syscall";

    /// Parse + validate the `off`/`on`/`abi` value.
    pub fn apply(&self, p1: &str) -> KunaResult<(X64SyscallMode, String)> {
        let mode = match p1 {
            "off" => X64SyscallMode::Off,
            "on" => X64SyscallMode::On,
            "abi" => X64SyscallMode::Abi,
            other => {
                return Err(KunaError::parse(format!(
                    "Unknown x64syscall value: {other} (expected off|on|abi)"
                )))
            }
        };
        Ok((mode, format!("x86-64 syscall effect model set to {p1}")))
    }
}

/// The storage the x86-64 Linux syscall ABI uses.
pub struct SyscallAbi64 {
    /// `RAX` — the number on the way in, the result on the way out.
    pub num: Address,
    /// `RDI, RSI, RDX, R10, R8, R9`.
    pub args: Vec<Address>,
}

/// Resolve the x86-64 syscall ABI against this language, or `None` when the
/// language is not x86-64.
///
/// Every ABI register must resolve at its full 8-byte width and the default code
/// space must be 8 bytes wide. x86-32 has no `RAX` at all, and the address-size
/// test excludes any other 64-bit language that happens to name one.
pub fn resolve_abi(data: &Funcdata) -> Option<SyscallAbi64> {
    let manage = data.get_arch().manage();
    if manage.get_default_code_space()?.get_addr_size() != REGISTER_SIZE as u32 {
        return None;
    }
    let lookup = manage.register_lookup()?;
    let reg = |nm: &str| -> Option<Address> {
        let st = lookup.probe_register(nm)?;
        if st.size != REGISTER_SIZE as u32 {
            return None;
        }
        Some(Address::new(st.space.clone()?, st.offset))
    };
    let num = reg(NUM_REGISTER)?;
    let mut args = Vec::with_capacity(ARG_REGISTERS.len());
    for nm in ARG_REGISTERS {
        args.push(reg(nm)?);
    }
    Some(SyscallAbi64 { num, args })
}

/// Do two storage locations overlap?
fn overlaps(a: &Address, asz: int4, b: &Address, bsz: int4) -> bool {
    match (a.get_space(), b.get_space()) {
        (Some(x), Some(y)) if std::rc::Rc::ptr_eq(x, y) => {
            let (ao, bo) = (a.get_offset(), b.get_offset());
            ao < bo + bsz as u64 && bo < ao + asz as u64
        }
        _ => false,
    }
}

/// Does the `CALLOTHER`'s own basic block write `reg` before it?
///
/// A backward walk of the raw op list from `from`, stopping — declining — at the
/// first call or branch (whose effect on the register is not visible here), at
/// the top of the block, and at [`SETUP_WALK_LIMIT`] ops. Any output overlapping
/// the register counts: a 32-bit `MOV ESI,...` sets up `RSI` just as a 64-bit one
/// does. Pre-SSA there is no def-use edge to follow, which is the point — this is
/// a local, syntactic read of the instruction sequence the assembler wrote.
fn block_sets_up(data: &Funcdata, from: OpId, reg: &Address) -> bool {
    let Some(block) = data.obank().get(from).and_then(|o| o.get_parent()) else {
        return false;
    };
    let mut cur = from;
    for _ in 0..SETUP_WALK_LIMIT {
        let Some(prev) = data.op_previous_op(cur) else { return false };
        let Some(op) = data.obank().get(prev) else { return false };
        if op.get_parent() != Some(block) {
            return false;
        }
        match op.code() {
            OpCode::CPUI_CALL
            | OpCode::CPUI_CALLIND
            | OpCode::CPUI_CALLOTHER
            | OpCode::CPUI_BRANCH
            | OpCode::CPUI_CBRANCH
            | OpCode::CPUI_BRANCHIND
            | OpCode::CPUI_RETURN => return false,
            _ => {}
        }
        if let Some(outid) = op.get_out() {
            if let Some(out) = data.vbank().get(outid) {
                if overlaps(out.get_addr(), out.get_size(), reg, REGISTER_SIZE) {
                    return true;
                }
            }
        }
        cur = prev;
    }
    false
}

/// Is `op` the bare `SYSCALL` user-op this pass models?
///
/// The signature is exact: a `CALLOTHER` whose only input is the user-op index,
/// with no output, naming [`USEROP_NAME`]. `SYSENTER`/`SYSRET` are separate
/// user-ops and the `int 0x80` lowering is a two-input `swi`, so neither can
/// match. The index is checked against the ids the architecture resolved for
/// [`USEROP_NAME`] into the ArchSeam, which is also where a user-op a compiler
/// spec specialized with its own `<callotherfixup>` is dropped — the spec's model
/// wins.
fn is_bare_syscall(data: &Funcdata, op: OpId) -> bool {
    let Some(oth) = data.obank().get(op) else { return false };
    if oth.code() != OpCode::CPUI_CALLOTHER || oth.num_input() != 1 || oth.get_out().is_some() {
        return false;
    }
    let Some(index) = oth
        .get_in(0)
        .and_then(|v| data.vbank().get(v))
        .filter(|v| v.is_constant())
        .map(|v| v.get_offset())
    else {
        return false;
    };
    u32::try_from(index).is_ok_and(|i| data.get_arch().x64_syscall_userops.contains(&i))
}

/// Give one matched `CALLOTHER` its ABI effects. Returns whether it changed.
fn rewrite(
    data: &mut Funcdata,
    op: OpId,
    abi: &SyscallAbi64,
    mode: X64SyscallMode,
) -> KunaResult<bool> {
    let mut reads = vec![abi.num.clone()];
    for reg in &abi.args {
        if mode.takes_all_args() || block_sets_up(data, op, reg) {
            reads.push(reg.clone());
        }
    }
    // Slot 0 is the user-op index and stays put; the effects follow it in ABI
    // order, which is the order they print in.
    for (i, reg) in reads.iter().enumerate() {
        let vn = data.new_varnode(REGISTER_SIZE, reg, None);
        data.op_insert_input(op, vn, (i + 1) as int4)?;
    }
    data.new_varnode_out(REGISTER_SIZE, &abi.num, op)?;
    Ok(true)
}

/// (kuna) `ActionX64Syscall` — give the x86-64 `SYSCALL` user-op the Linux ABI's
/// register effects (option `x64syscall`).
pub struct ActionX64Syscall {
    base: ActionBase,
}

impl ActionX64Syscall {
    /// Construct the action in the given group.
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionX64Syscall {
            base: ActionBase::new(
                crate::action::ruleflags::rule_onceperfunc,
                "x64syscall",
                g,
            ),
        })
    }
}

impl Action for ActionX64Syscall {
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
        Some(Box::new(ActionX64Syscall { base: self.base.clone() }))
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        let mode = data.get_arch().x64_syscall;
        if !mode.rewrites() || data.get_arch().x64_syscall_userops.is_empty() {
            return 0;
        }
        // The op-bank scan is the cheap gate: a function with no `SYSCALL` never
        // reaches the register probes.
        let found: Vec<OpId> = data
            .obank()
            .iter_code(OpCode::CPUI_CALLOTHER)
            .filter(|op| is_bare_syscall(data, *op))
            .collect();
        if found.is_empty() {
            return 0;
        }
        let Some(abi) = resolve_abi(data) else { return 0 };
        let mut changed = 0;
        for op in found {
            if rewrite(data, op, &abi, mode).unwrap_or(false) {
                changed += 1;
            }
        }
        if changed != 0 {
            1
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests;
