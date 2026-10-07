//! (kuna `syscallregs`) Give the system-call instruction of ARM, AArch64,
//! RISC-V, MIPS and PowerPC the registers it reads and writes.
//!
//! # The defect
//!
//! The SLEIGH specs lower `svc` (ARM, AArch64), `ecall` (RISC-V), `syscall`
//! (MIPS) and `sc` (PowerPC) to a `CALLOTHER` with no register inputs and no
//! output. The Linux and BSD kernels these instructions trap into return their
//! result in a register (`r0`, `x0`, `a0`, `v0`, `r3`), but kuna is told nothing
//! changes, so whatever the function put in that register to set the call up is
//! still there afterwards:
//!
//! ```text
//! mov r0, r4 ; mov r1, r5 ; mov r7, #4 ; svc 0 ; pop {.., pc}
//!   ->  software_interrupt(0); return CONCAT44(a0,a1);
//! li v0, 4004 ; syscall ; jr ra
//!   ->  syscall(0); return 0xfa4;
//! ```
//!
//! The first returns its arguments as a register pair, the second the syscall
//! number. Any other read of the result register after the instruction is the
//! same false value (`if (r0 < 0)` tests the argument), and the setup writes
//! have no reader, so the arguments are erased from the call.
//!
//! # The fix
//!
//! Before heritage, the `CALLOTHER` is rewritten in place, as
//! [`crate::kuna_x64syscall`] does for x86-64:
//!
//! - it writes the result register;
//! - it reads the number register when the function writes it on every path to
//!   the call with no call in between ([`written_on_every_path`]), and then the
//!   argument registers in order, stopping at the first one the function does
//!   not write that way. A printed argument is therefore always in its own
//!   position, and every read is of a value the function placed there, so no
//!   undefined register becomes a parameter.
//!
//! The pair goes away with it: its low half is now the call's output, and its
//! high half is an argument the call reads. Return recovery treats the call as
//! a call site ([`reads_as_argument`]), so that argument is not also a returned
//! half.
//!
//! MIPS and PowerPC kernels also report a failure outside the result: MIPS sets
//! `a3` to 0 or 1, PowerPC the summary-overflow bit of `cr0`, and the C library
//! tests it right after the instruction. A new op after the call defines that
//! register with the opaque `syscall_error()` ([`define_error_flag`]), so the
//! test reads the kernel's flag instead of the function's entry `a3`, which
//! otherwise becomes a phantom parameter. It is an ordinary write, so heritage
//! treats it as any other, and dead-code removal drops it where nothing reads
//! the register. Each flag is its own value: two are never merged or folded
//! into one, and a copy of the op keeps it free of side effects.
//!
//! That phantom `a3` used to pull a MIPS wrapper's real parameters in with it,
//! `a0`..`a2` taking the positions before it, although the call read none of
//! them. So a MIPS system call also reads, written or not, as many argument
//! registers as the kernel's entry point for its constant number takes
//! ([`mips_args`]), and all four of `a0`..`a3` when the table does not know the
//! number: a wrapper that hands its parameters to the kernel in place keeps
//! them.
//!
//! # When it acts
//!
//! Not every handler returns a result. A bare-metal RTOS uses the same
//! instruction for a context switch or a privilege change and leaves every
//! register alone, and its compiler keeps live values in the result register
//! across the instruction (FreeRTOS's Cortex-M `xPortRaisePrivilege` returns
//! the `r0 = 0` it set before `svc 2`). So `auto`, the default, acts only on an
//! image the loader identified as a program for an operating system's user
//! space ([`SyscallRegsMode::fires`]); `on` acts on any image.
//!
//! # What it does not model
//!
//! Outside MIPS, a wrapper that hands its own incoming arguments to the kernel
//! untouched shows only the leading registers it sets: nothing says how many
//! of them the kernel reads, and taking all of them would give a zero-argument
//! call phantom parameters. A MIPS call with no known number takes `a0`..`a3`
//! even when it reads fewer, so a test of its flag still reads as a fourth
//! parameter there, and arguments past `a3` that o32 passes on the stack are
//! not read. The MIPS `v1` a few calls return a second value in is
//! not modelled.
//!
//! A `CALLOTHER` a compiler spec has specialized with its own
//! `<callotherfixup>` is left alone.

use kuna_base::address::Address;
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::types::{int4, uint4};
use kuna_num::opcodes::OpCode;
use std::collections::HashMap;

use crate::action::{Action, ActionBase, ActionContext, ActionGroupList, ApplyResult};
use crate::context::{BlockId, OpId};
use crate::funcdata::Funcdata;
use crate::kuna_x64syscall::overlaps;
use crate::op::{pcodeop_flags, PcodeOpBank};
use crate::userop::BUILTIN_SYSCALL_ERROR;
use crate::varnode::VarnodeBank;

/// How many basic blocks one register's walk may visit before it gives up and
/// leaves the register unread.
const BLOCK_WALK_LIMIT: usize = 256;

/// The argument registers a MIPS system call with no known number reads:
/// `a0`..`a3`.
const MIPS_UNKNOWN_ARGS: usize = 4;

/// (kuna) When the system call gets its register effects: `syscallregs
/// off|auto|on`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SyscallRegsMode {
    /// The vendored model: no register inputs, no output.
    #[default]
    Off,
    /// Only on an image the loader identified as an operating system's
    /// user-space program.
    Auto,
    /// On any image.
    On,
}

impl SyscallRegsMode {
    /// The option token for this mode.
    pub fn as_str(self) -> &'static str {
        match self {
            SyscallRegsMode::Off => "off",
            SyscallRegsMode::Auto => "auto",
            SyscallRegsMode::On => "on",
        }
    }

    /// Does the pass act on an image with this loader fact?
    pub fn fires(self, image_os_userland: bool) -> bool {
        match self {
            SyscallRegsMode::Off => false,
            SyscallRegsMode::Auto => image_os_userland,
            SyscallRegsMode::On => true,
        }
    }
}

/// (kuna) Parse `option syscallregs off|auto|on`; the caller writes the live
/// field.
pub struct OptionSyscallRegs;

impl OptionSyscallRegs {
    /// The option name.
    pub const NAME: &'static str = "syscallregs";

    /// Parse + validate the value.
    pub fn apply(&self, p1: &str) -> KunaResult<(SyscallRegsMode, String)> {
        let mode = match p1 {
            "off" => SyscallRegsMode::Off,
            "auto" => SyscallRegsMode::Auto,
            "on" => SyscallRegsMode::On,
            other => {
                return Err(KunaError::parse(format!(
                    "Unknown syscallregs value: {other} (expected off|auto|on)"
                )))
            }
        };
        Ok((mode, format!("System-call register effects set to {p1}")))
    }
}

/// The processor families whose system-call user-op this pass models.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyscallFamily {
    Arm,
    AArch64,
    RiscV,
    Mips,
    PowerPc,
}

impl SyscallFamily {
    /// The family a language id names, from its processor field.
    pub fn from_archid(archid: &str) -> Option<Self> {
        match archid.split(':').next()? {
            "ARM" => Some(SyscallFamily::Arm),
            "AARCH64" => Some(SyscallFamily::AArch64),
            "RISCV" => Some(SyscallFamily::RiscV),
            "MIPS" => Some(SyscallFamily::Mips),
            "PowerPC" => Some(SyscallFamily::PowerPc),
            _ => None,
        }
    }

    /// The user-op the family's system-call constructor emits.
    pub fn userop(self) -> &'static str {
        match self {
            SyscallFamily::Arm => "software_interrupt",
            SyscallFamily::AArch64 => "CallSupervisor",
            SyscallFamily::RiscV => "ecall",
            SyscallFamily::Mips | SyscallFamily::PowerPc => "syscall",
        }
    }

    /// The register the kernel returns its result in.
    pub fn result(self) -> &'static str {
        match self {
            SyscallFamily::Arm => "r0",
            SyscallFamily::AArch64 => "x0",
            SyscallFamily::RiscV => "a0",
            SyscallFamily::Mips => "v0",
            SyscallFamily::PowerPc => "r3",
        }
    }

    /// The register the kernel flags a failure in besides the result: `a3` is 0
    /// or 1 on MIPS, and the summary-overflow bit of `cr0` is set on PowerPC.
    pub fn error_flag(self) -> Option<&'static str> {
        match self {
            SyscallFamily::Mips => Some("a3"),
            SyscallFamily::PowerPc => Some("cr0"),
            _ => None,
        }
    }

    /// The registers the call may read, number register first. `wide` selects
    /// the 64-bit MIPS ABIs, whose fifth and sixth arguments are `t0`/`t1`.
    pub fn inputs(self, wide: bool) -> &'static [&'static str] {
        match self {
            SyscallFamily::Arm => &["r7", "r0", "r1", "r2", "r3", "r4", "r5", "r6"],
            SyscallFamily::AArch64 => &["x8", "x0", "x1", "x2", "x3", "x4", "x5", "x6", "x7"],
            SyscallFamily::RiscV => &["a7", "a0", "a1", "a2", "a3", "a4", "a5", "a6"],
            SyscallFamily::Mips if wide => &["v0", "a0", "a1", "a2", "a3", "t0", "t1"],
            SyscallFamily::Mips => &["v0", "a0", "a1", "a2", "a3"],
            SyscallFamily::PowerPc => &["r0", "r3", "r4", "r5", "r6", "r7", "r8"],
        }
    }
}

/// The family's registers, resolved against the loaded language.
struct SyscallStorage {
    family: SyscallFamily,
    result: (Address, int4),
    inputs: Vec<(Address, int4)>,
    error: Option<(Address, int4)>,
}

/// Resolve every register `family` names, or `None` when one is missing.
fn resolve(data: &Funcdata, family: SyscallFamily) -> Option<SyscallStorage> {
    let lookup = data.get_arch().manage().register_lookup()?;
    let reg = |nm: &str| -> Option<(Address, int4)> {
        let st = lookup.probe_register(nm)?;
        Some((Address::new(st.space.clone()?, st.offset), st.size as int4))
    };
    let result = reg(family.result())?;
    let wide = result.1 == 8;
    let inputs = family
        .inputs(wide)
        .iter()
        .map(|nm| reg(nm))
        .collect::<Option<Vec<_>>>()?;
    let error = family.error_flag().and_then(reg);
    Some(SyscallStorage { family, result, inputs, error })
}

/// Is `op` a `CALLOTHER` of the family's system-call user-op?
fn is_syscall(data: &Funcdata, op: OpId) -> bool {
    let Some(oth) = data.obank().get(op) else {
        return false;
    };
    if oth.code() != OpCode::CPUI_CALLOTHER {
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
    u32::try_from(index).is_ok_and(|i| data.get_arch().syscall_regs_userops.contains(&i))
}

/// Is `op` the `syscall_error()` this pass placed after a system call?
pub fn is_error_flag(data: &Funcdata, op: OpId) -> bool {
    is_error_flag_op(data.obank(), data.vbank(), op)
}

/// [`is_error_flag`] over the op and varnode banks alone.
pub fn is_error_flag_op(obank: &PcodeOpBank, vbank: &VarnodeBank, op: OpId) -> bool {
    obank.get(op).is_some_and(|o| {
        o.code() == OpCode::CPUI_CALLOTHER
            && o.get_in(0)
                .and_then(|v| vbank.get(v))
                .is_some_and(|v| v.is_constant() && v.get_offset() == BUILTIN_SYSCALL_ERROR as u64)
    })
}

/// Is `op` a system call this pass has not rewritten: the user-op index, at
/// most the instruction's immediate, and no output?
fn is_bare_syscall(data: &Funcdata, op: OpId) -> bool {
    is_syscall(data, op)
        && data
            .obank()
            .get(op)
            .is_some_and(|o| o.num_input() <= 2 && o.get_out().is_none())
}

/// How the ops of one block, read backward from a point, leave `reg`.
enum Scan {
    Written,
    Clobbered,
    Reached,
}

/// Scan `ops` backward for the last write of `reg`, stopping at a call, whose
/// effect on the register is not visible here. Another system call writes the
/// result register, and its error flag as a value the function did not set,
/// whether or not it has been rewritten yet.
fn scan_back(
    data: &Funcdata,
    ops: &[OpId],
    reg: &Address,
    size: int4,
    regs: &SyscallStorage,
) -> Scan {
    for &op in ops.iter().rev() {
        let Some(o) = data.obank().get(op) else {
            continue;
        };
        if matches!(o.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND) {
            return Scan::Clobbered;
        }
        if is_syscall(data, op) {
            if overlaps(&regs.result.0, regs.result.1, reg, size) {
                return Scan::Written;
            }
            if regs.error.as_ref().is_some_and(|(a, s)| overlaps(a, *s, reg, size)) {
                return Scan::Clobbered;
            }
            continue;
        }
        if is_error_flag(data, op) {
            continue;
        }
        let written = o
            .get_out()
            .and_then(|out| data.vbank().get(out))
            .is_some_and(|v| overlaps(v.get_addr(), v.get_size(), reg, size));
        if written {
            return Scan::Written;
        }
    }
    Scan::Reached
}

/// Does every path from the function entry to `op` write `reg`, with no call
/// between the write and `op`?
///
/// A must-define dataflow over the raw blocks that reach `op`: a block decides
/// for itself when it writes the register or makes a call, and otherwise
/// inherits from all of its predecessors; the entry decides no. Every block
/// starts at yes and only falls to no, so a loop is decided by the paths into
/// it. More than [`BLOCK_WALK_LIMIT`] blocks answers no.
fn written_on_every_path(
    data: &Funcdata,
    op: OpId,
    reg: &Address,
    size: int4,
    regs: &SyscallStorage,
) -> bool {
    let Some(start) = data.obank().get(op).and_then(|o| o.get_parent()) else {
        return false;
    };
    let ops = data.bb_ops(start);
    let at = ops.iter().position(|&o| o == op).unwrap_or(ops.len());
    match scan_back(data, &ops[..at], reg, size, regs) {
        Scan::Written => return true,
        Scan::Clobbered => return false,
        Scan::Reached => {}
    }
    let graph = data.bblocks_ref();
    let preds = |b: BlockId| (0..graph.block(b).size_in()).map(move |i| graph.block(b).get_in(i));
    let mut blocks: Vec<BlockId> = Vec::new();
    let mut index: HashMap<BlockId, usize> = HashMap::new();
    let mut work: Vec<BlockId> = preds(start).collect();
    while let Some(b) = work.pop() {
        if index.contains_key(&b) {
            continue;
        }
        if blocks.len() == BLOCK_WALK_LIMIT {
            return false;
        }
        index.insert(b, blocks.len());
        blocks.push(b);
        work.extend(preds(b));
    }
    if blocks.is_empty() {
        return false;
    }
    let local: Vec<Scan> = blocks
        .iter()
        .map(|&b| scan_back(data, &data.bb_ops(b), reg, size, regs))
        .collect();
    let mut out = vec![true; blocks.len()];
    let mut changed = true;
    while changed {
        changed = false;
        for i in 0..blocks.len() {
            if !out[i] {
                continue;
            }
            let now = match local[i] {
                Scan::Written => true,
                Scan::Clobbered => false,
                Scan::Reached => {
                    let mut ps = preds(blocks[i]).peekable();
                    ps.peek().is_some() && ps.all(|p| out[index[&p]])
                }
            };
            if !now {
                out[i] = false;
                changed = true;
            }
        }
    }
    preds(start).all(|p| out[index[&p]])
}

/// How the ops of one block, read backward from a point, define `reg`: a
/// constant, something else, or nothing.
enum Def {
    Constant(u64),
    Other,
    None,
}

/// Scan `ops` backward for the last write of `reg`; a call is [`Def::Other`].
fn last_def(data: &Funcdata, ops: &[OpId], reg: &Address, size: int4) -> Def {
    for &o in ops.iter().rev() {
        let Some(oo) = data.obank().get(o) else {
            continue;
        };
        if oo.is_call() {
            return Def::Other;
        }
        let Some(out) = oo.get_out().and_then(|v| data.vbank().get(v)) else {
            continue;
        };
        if !overlaps(out.get_addr(), out.get_size(), reg, size) {
            continue;
        }
        if out.get_addr() != reg || out.get_size() != size {
            return Def::Other;
        }
        let ins = (0..oo.num_input())
            .map(|i| {
                oo.get_in(i)
                    .and_then(|v| data.vbank().get(v))
                    .filter(|v| v.is_constant())
                    .map(|v| v.get_offset())
            })
            .collect::<Option<Vec<u64>>>();
        let value = match (oo.code(), ins.as_deref()) {
            (OpCode::CPUI_COPY, Some([c])) => *c,
            (OpCode::CPUI_INT_ADD, Some([a, b])) => a.wrapping_add(*b),
            (OpCode::CPUI_INT_OR, Some([a, b])) => a | b,
            _ => return Def::Other,
        };
        return Def::Constant(value & kuna_base::address::calc_mask(size));
    }
    Def::None
}

/// Fold one block's [`Def`] into the constant seen so far: `Some(true)` when it
/// writes that constant, `Some(false)` when it does not write the register,
/// `None` when it writes something else.
fn agree(value: &mut Option<u64>, def: Def) -> Option<bool> {
    match def {
        Def::Constant(c) if value.is_none_or(|v| v == c) => {
            *value = Some(c);
            Some(true)
        }
        Def::None => Some(false),
        _ => None,
    }
}

/// The constant `reg` holds at `op`: every write of it that reaches `op`, with
/// no call in between, writes the same constant (`li`, or `addiu`/`ori` from
/// the zero register). More than [`BLOCK_WALK_LIMIT`] blocks answers no.
fn constant_at(data: &Funcdata, op: OpId, reg: &Address, size: int4) -> Option<u64> {
    let start = data.obank().get(op)?.get_parent()?;
    let ops = data.bb_ops(start);
    let at = ops.iter().position(|&o| o == op)?;
    let mut value = None;
    if agree(&mut value, last_def(data, &ops[..at], reg, size))? {
        return value;
    }
    let graph = data.bblocks_ref();
    let preds = |b: BlockId| (0..graph.block(b).size_in()).map(move |i| graph.block(b).get_in(i));
    let mut seen: Vec<BlockId> = Vec::new();
    let mut work: Vec<BlockId> = preds(start).collect();
    if work.is_empty() {
        return None;
    }
    while let Some(b) = work.pop() {
        if seen.contains(&b) {
            continue;
        }
        if seen.len() == BLOCK_WALK_LIMIT {
            return None;
        }
        seen.push(b);
        if !agree(&mut value, last_def(data, &data.bb_ops(b), reg, size))? {
            let mut ps = preds(b).peekable();
            ps.peek()?;
            work.extend(ps);
        }
    }
    value
}

/// How many argument registers a MIPS system call at `op` reads at least: the
/// kernel's count for its constant number ([`mips_args`]), or all of
/// `a0`..`a3` when the table does not know the number (one computed at run
/// time, as in a dispatcher like the C library's `syscall`), since such a
/// function hands its own `a3` to the kernel.
fn mips_floor(data: &Funcdata, op: OpId, number: &(Address, int4)) -> usize {
    constant_at(data, op, &number.0, number.1)
        .and_then(mips_args::mips_arg_count)
        .unwrap_or(MIPS_UNKNOWN_ARGS)
}

/// The registers the call at `op` reads: the number register, then each
/// argument register in order up to the first one the function does not write
/// on every path to it. Nothing when the number register is not written so.
/// A MIPS call reads at least [`mips_floor`] argument registers, written or
/// not: a wrapper hands its own parameters to the kernel in place.
fn read_set(data: &Funcdata, op: OpId, regs: &SyscallStorage) -> Vec<(Address, int4)> {
    let written =
        |(addr, size): &&(Address, int4)| written_on_every_path(data, op, addr, *size, regs);
    let Some((number, args)) = regs.inputs.split_first() else {
        return Vec::new();
    };
    if !written(&number) {
        return Vec::new();
    }
    let mut count = args.iter().take_while(written).count();
    if regs.family == SyscallFamily::Mips {
        count = count.max(mips_floor(data, op, number).min(args.len()));
    }
    std::iter::once(number).chain(&args[..count]).cloned().collect()
}

/// Give one matched `CALLOTHER` its register effects.
fn rewrite(data: &mut Funcdata, op: OpId, regs: &SyscallStorage) -> KunaResult<()> {
    let reads = read_set(data, op, regs);
    let base = data.obank().get(op).map(|o| o.num_input()).unwrap_or(1);
    for (i, (addr, size)) in reads.iter().enumerate() {
        let vn = data.new_varnode(*size, addr, None);
        data.op_insert_input(op, vn, base + i as int4)?;
    }
    data.new_varnode_out(regs.result.1, &regs.result.0, op)?;
    if let Some((addr, size)) = &regs.error {
        define_error_flag(data, op, addr, *size)?;
    }
    Ok(())
}

/// Write the error-flag register right after the system call at `op` with an
/// opaque `syscall_error()`. Without the call flag the op has no side effect,
/// so it dies with its last reader.
fn define_error_flag(data: &mut Funcdata, op: OpId, addr: &Address, size: int4) -> KunaResult<()> {
    let pc = data
        .obank()
        .get(op)
        .map(|o| o.get_addr().clone())
        .ok_or_else(|| KunaError::lowlevel("syscallregs: stale system call"))?;
    let flag = data.new_op(1, pc);
    data.op_set_opcode_code(flag, OpCode::CPUI_CALLOTHER);
    if let Some(o) = data.obank_mut().get_mut(flag) {
        o.clear_flag(pcodeop_flags::call);
    }
    let id = data.new_constant(4, BUILTIN_SYSCALL_ERROR as u64);
    data.op_set_input(flag, id, 0)?;
    data.new_varnode_out(size, addr, flag)?;
    data.op_insert_after(flag, op);
    Ok(())
}

/// (kuna) `ActionSyscallRegs`: give the ARM, AArch64, RISC-V, MIPS and PowerPC
/// system-call user-op its register effects (option `syscallregs`).
pub struct ActionSyscallRegs {
    base: ActionBase,
}

impl ActionSyscallRegs {
    /// Construct the action in the given group.
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionSyscallRegs {
            base: ActionBase::new(crate::action::ruleflags::rule_onceperfunc, "syscallregs", g),
        })
    }
}

impl Action for ActionSyscallRegs {
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
        Some(Box::new(ActionSyscallRegs {
            base: self.base.clone(),
        }))
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        let arch = data.get_arch();
        if !arch.syscall_regs || arch.syscall_regs_userops.is_empty() {
            return 0;
        }
        let Some(family) = arch.syscall_regs_family else {
            return 0;
        };
        let found: Vec<OpId> = data
            .obank()
            .iter_code(OpCode::CPUI_CALLOTHER)
            .filter(|op| is_bare_syscall(data, *op))
            .collect();
        if found.is_empty() {
            return 0;
        }
        let Some(regs) = resolve(data, family) else {
            return 0;
        };
        let mut changed = 0;
        for op in found {
            if rewrite(data, op, &regs).is_ok() {
                changed = 1;
            }
        }
        changed
    }
}

/// The enabled system calls whose opaque handlers may read or write memory.
pub fn is_memory_call(data: &Funcdata, op: OpId) -> bool {
    data.get_arch().syscall_regs
        && data.obank().get(op).is_some_and(|o| !o.is_dead())
        && is_syscall(data, op)
}

/// The enabled system calls whose opaque handlers may read or write memory.
pub fn memory_calls(data: &Funcdata) -> Vec<OpId> {
    if !data.get_arch().syscall_regs || data.get_arch().syscall_regs_userops.is_empty() {
        return Vec::new();
    }
    data.obank()
        .iter_code(OpCode::CPUI_CALLOTHER)
        .filter(|&op| is_memory_call(data, op))
        .collect()
}

/// Guard writable memory across system calls using the same image policy as
/// their register effects. Registers and unique temporaries remain unchanged.
pub fn guard_memory(
    data: &mut Funcdata,
    flags: uint4,
    addr: &Address,
    size: int4,
    write: &mut Vec<crate::context::VarnodeId>,
) {
    use crate::varnode::varnode_flags;
    use kuna_base::space::spacetype;

    if !data.get_arch().syscall_regs
        || flags & varnode_flags::readonly != 0
        || !addr.get_space().is_some_and(|s| {
            s.get_type() == spacetype::IPTR_SPACEBASE
                || data.get_arch().manage().get_default_data_space()
                    .is_some_and(|ram| ram.get_index() == s.get_index())
        })
    {
        return;
    }
    for op in memory_calls(data) {
        let guard = data.new_indirect_op(op, addr, size, 0);
        if let Some(input) = data.obank().get(guard).and_then(|o| o.get_in(0)) {
            data.vbank_mut().get_mut(input)
                .expect("syscall memory input").set_active_heritage();
        }
        if let Some(output) = data.obank().get(guard).and_then(|o| o.get_out()) {
            let v = data.vbank_mut().get_mut(output).expect("syscall memory output");
            v.set_active_heritage();
            if flags & varnode_flags::addrtied != 0 {
                v.set_addr_force();
            }
            write.push(output);
        }
    }
}

/// Does the rewritten system call `op` read `vn` as one of its registers?
///
/// `Funcdata::only_op_use` asks this while it follows a trial's value forward.
/// The call is a use of the value, not a step it flows through, since its
/// result is the kernel's. A value the kernel is handed as an argument is
/// therefore not also the high half of a returned pair. The kernel preserves
/// the argument registers, so a call whose result the function discards
/// (`svc; mov r0,#7; bx lr`) leaves the argument in `r1` at the RETURN. It stays
/// free to be a later call's argument too (`svc; bl use` with `r1` kept).
pub fn reads_as_argument(data: &Funcdata, op: OpId, vn: crate::context::VarnodeId) -> bool {
    data.get_arch().syscall_regs
        && is_syscall(data, op)
        && data.obank().get(op).is_some_and(|o| {
            o.get_out().is_some() && (1..o.num_input()).any(|i| o.get_in(i) == Some(vn))
        })
}

/// The user-op ids `family`'s system-call constructor emits, leaving out one a
/// compiler spec specialized with its own `<callotherfixup>`.
pub fn userop_ids(
    userops: &crate::userop::UserOpManage,
    family: Option<SyscallFamily>,
) -> Vec<uint4> {
    family
        .and_then(|f| userops.get_op_by_name(f.userop().as_bytes()))
        .filter(|u| u.get_inject_id().is_none())
        .map(|u| vec![u.get_index() as uint4])
        .unwrap_or_default()
}

mod mips_args;

#[cfg(test)]
mod tests;
