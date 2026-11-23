//! (kuna) `varargsave` — the x86-64 System V register-save prologue of a
//! variadic function is how `va_start` is implemented, not program logic.
//!
//! A function that calls `va_start` stores the argument registers a caller may
//! have filled past the named parameters into a register save area `R` in its
//! frame, so `va_arg` can read them through the `va_list`:
//!
//! ```text
//!   mov  %rsi,0x28(%rsp)   ; general registers from the first unnamed one
//!   ...                    ; to r9, at R + 8*k
//!   mov  %r9,0x48(%rsp)
//!   test %al,%al           ; the caller's count of vector registers used
//!   je   1f
//!   movaps %xmm0,0x50(%rsp)  ; xmm registers at R + 48 + 16*j
//!   ...
//!   movaps %xmm7,0xc0(%rsp)
//! 1:
//! ```
//!
//! Printed as code it is an `if (al) { … }` the source does not have, eight
//! `xmm` parameters and five more integer ones: `mprintf(const char *fmt, ...)`
//! decompiled with fourteen parameters.
//!
//! [`fold`] runs right after the first heritage pass, when the registers are
//! in SSA form but the stack is not yet, so every store is still the raw
//! `STORE` of an incoming register to the entry stack pointer plus a constant.
//! It matches the prologue exactly:
//!
//! * a `CBRANCH` on `(al & al) == 0`, where `al` is the incoming value of the
//!   register (also after the `push %rax; pop %rax` gcc puts in front of a
//!   no-return variadic function), jumping over a block that holds nothing but stores of the
//!   incoming `xmm` registers `m..m+c-1`, each to the slot `R + 48 + 16*j` of
//!   one save area `R`, and rejoins the jump target;
//! * the stores of the incoming general argument registers `n..n+g-1`
//!   (`rdi`..`r9` are 0..5) to `R + 8*k`, a contiguous run, in the block that
//!   tests `al` or the straight line of blocks before it.
//!
//! The fold is for a function whose own body never reads the save area: one
//! that hands its `va_list` to a callee (`vfprintf(f, fmt, ap)`). [`frame`]
//! follows the entry stack pointer forward to every value that holds it plus a
//! constant, and the fold is refused when
//!
//! * a load reads a saved slot (gcc folds a `va_arg` whose offset it knows into
//!   `mov 0x28(%rsp),%eax`), or a value pointing into a saved slot is loaded
//!   through, passed on or turned into another 64-bit value ([`carries_pointer`];
//!   a flag or the `eax` half `xor %eax,%eax` reads is neither);
//! * the save area's address `R` is used that way other than as the value a
//!   store puts in a frame slot (the `va_list`'s `reg_save_area` field), such as
//!   indexed by a `gp_offset` held in a register;
//! * a load reads a `va_list` whose `reg_save_area` field is filled with `R`
//!   (an unoptimized `va_arg` reloads `gp_offset` and `reg_save_area`, and the
//!   constants it reloads resolve the read to a saved slot);
//! * the stack pointer is realigned (`and $-16,%rsp`), so a frame offset no
//!   longer names one slot.
//!
//! The edit removes the jump's edge into the `xmm` block (the block becomes
//! unreachable and goes with its stores) and destroys the general-register
//! stores. The flag computation and the stored registers then have no reader,
//! so `al`, the `xmm` registers and the unnamed general registers are not
//! parameters. [`mark_variadic`] gives the recovered prototype its `...`.
//!
//! x86-64 System V only (`VarargFloats::VectorCount`): Windows x64 spills its
//! four argument registers to the caller's home area without an `al` test, and
//! no other target has this prologue.

use std::collections::{HashMap, HashSet};

use kuna_base::space::spacetype;
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::context::{BlockId, OpId, VarnodeId};
use crate::funcdata::Funcdata;
use crate::kuna_varargretreg::VarargFloats;

/// The general argument registers in System V order.
const GP_ARGS: [&str; 6] = ["RDI", "RSI", "RDX", "RCX", "R8", "R9"];
/// The vector argument registers in System V order.
const XMM_ARGS: [&str; 8] = [
    "XMM0", "XMM1", "XMM2", "XMM3", "XMM4", "XMM5", "XMM6", "XMM7",
];
/// Bytes of the general-register part of the save area.
const GP_AREA: i64 = 48;
/// Bytes of a `va_list` before its `reg_save_area` field, and of the field.
const VA_LIST_HEAD: i64 = 16;
const VA_LIST_FIELD: i64 = 8;
/// Most frame offsets one value may hold before it counts as a walk.
const OFFSET_CAP: usize = 8;
/// Most values [`frame`] follows before it gives up.
const VALUE_CAP: usize = 20_000;

#[derive(Clone)]
struct Reg {
    space: usize,
    offset: u64,
    size: int4,
}

struct Plan {
    guard: BlockId,
    edge: int4,
    gp_stores: Vec<OpId>,
}

/// Fold the prologue of `fd` when it matches; called once, right after the
/// first heritage pass.
pub(crate) fn fold(fd: &mut Funcdata) -> bool {
    if fd.get_heritage_pass() != 1
        || !fd.get_arch().vararg_save
        || fd.get_arch().vararg_floats != VarargFloats::VectorCount
        || fd.kuna_vararg_save
    {
        return false;
    }
    let Some(plan) = find(fd) else { return false };
    if fd.remove_branch(plan.guard, plan.edge).is_err()
        || fd.remove_unreachable_blocks(false, true).is_err()
    {
        return false;
    }
    for op in plan.gp_stores {
        if fd.obank().get(op).is_some_and(|o| o.get_parent().is_some()) {
            fd.op_destroy(op);
        }
    }
    fd.kuna_vararg_save = true;
    true
}

/// Give the recovered prototype of a folded function its `...`.
pub(crate) fn mark_variadic(fd: &mut Funcdata) {
    if fd.kuna_vararg_save && !fd.get_func_proto().is_input_locked() {
        fd.get_func_proto_mut().set_dotdotdot(true);
    }
}

fn register(fd: &Funcdata, name: &str) -> Option<Reg> {
    let r = fd
        .get_arch()
        .manage()
        .register_lookup()?
        .probe_register(name)?;
    Some(Reg {
        space: r.space?.get_index() as usize,
        offset: r.offset,
        size: r.size as int4,
    })
}

fn is_reg(fd: &Funcdata, vn: VarnodeId, reg: &Reg) -> bool {
    fd.vbank().get(vn).is_some_and(|v| {
        v.get_space().get_index() as usize == reg.space
            && v.get_offset() == reg.offset
            && v.get_size() == reg.size
    })
}

fn def(fd: &Funcdata, vn: VarnodeId) -> Option<&crate::op::PcodeOp> {
    fd.vbank()
        .get(vn)?
        .get_def()
        .and_then(|d| fd.obank().get(d))
}

fn constant(fd: &Funcdata, vn: VarnodeId) -> Option<u64> {
    fd.vbank()
        .get(vn)
        .filter(|v| v.is_constant())
        .map(|v| v.get_offset())
}

/// The incoming register `vn` holds, through copies and a reload of the
/// slot an earlier store of its block wrote (`push %rax; pop %rax`, which gcc
/// puts in front of a no-return variadic function's prologue).
fn incoming(fd: &Funcdata, mut vn: VarnodeId) -> Option<VarnodeId> {
    for _ in 0..4 {
        let v = fd.vbank().get(vn)?;
        if v.is_input() {
            return Some(vn);
        }
        let op = v.get_def()?;
        let o = fd.obank().get(op)?;
        vn = match o.code() {
            OpCode::CPUI_COPY => o.get_in(0)?,
            OpCode::CPUI_LOAD => reloaded(fd, op)?,
            _ => return None,
        };
    }
    None
}

/// The value the `LOAD` `op` reads back: the value of the last `STORE` before
/// it in its block through the same address value, with no call between.
fn reloaded(fd: &Funcdata, op: OpId) -> Option<VarnodeId> {
    let addr = fd.obank().get(op)?.get_in(1)?;
    let mut cur = fd.op_previous_op(op);
    while let Some(prev) = cur {
        let o = fd.obank().get(prev)?;
        match o.code() {
            OpCode::CPUI_STORE => {
                return if o.get_in(1)? == addr {
                    o.get_in(2)
                } else {
                    None
                }
            }
            OpCode::CPUI_CALL | OpCode::CPUI_CALLIND | OpCode::CPUI_CALLOTHER => return None,
            _ => {}
        }
        cur = fd.op_previous_op(prev);
    }
    None
}

/// Is `vn` the incoming `al`: the input itself, or the low byte of the
/// incoming `rax`/`eax`?
fn incoming_al(fd: &Funcdata, vn: VarnodeId, al: &Reg) -> bool {
    if incoming(fd, vn).is_some_and(|v| is_reg(fd, v, al)) {
        return true;
    }
    let Some(op) = def(fd, vn) else { return false };
    if op.code() != OpCode::CPUI_SUBPIECE || op.get_in(1).and_then(|c| constant(fd, c)) != Some(0) {
        return false;
    }
    op.get_in(0)
        .and_then(|w| incoming(fd, w))
        .and_then(|w| fd.vbank().get(w))
        .is_some_and(|w| {
            w.get_space().get_index() as usize == al.space && w.get_offset() == al.offset
        })
}

/// Is `cond` the zero flag of `test %al,%al` on the incoming `al`?
fn al_is_zero(fd: &Funcdata, cond: VarnodeId, al: &Reg) -> bool {
    let Some(eq) = def(fd, cond) else {
        return false;
    };
    if eq.code() != OpCode::CPUI_INT_EQUAL {
        return false;
    }
    let (Some(a), Some(b)) = (eq.get_in(0), eq.get_in(1)) else {
        return false;
    };
    let tested = match (constant(fd, a), constant(fd, b)) {
        (_, Some(0)) => a,
        (Some(0), _) => b,
        _ => return false,
    };
    let Some(and) = def(fd, tested) else {
        return false;
    };
    and.code() == OpCode::CPUI_INT_AND
        && (0..2).all(|i| and.get_in(i).is_some_and(|x| incoming_al(fd, x, al)))
}

/// The frame offsets a value may hold, as the entry stack pointer plus each.
/// `wide` marks a value that took more than [`OFFSET_CAP`] of them (a pointer
/// stepped around a loop); `at` keeps the ones seen before that.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct Offsets {
    at: Vec<i64>,
    wide: bool,
}

impl Offsets {
    fn shifted(&self, by: i64) -> Offsets {
        Offsets {
            at: self.at.iter().map(|a| a.wrapping_add(by)).collect(),
            wide: self.wide,
        }
    }

    fn merge(&mut self, other: &Offsets) -> bool {
        let mut changed = false;
        if other.wide && !self.wide {
            self.wide = true;
            changed = true;
        }
        if self.wide {
            return changed;
        }
        for &a in &other.at {
            if !self.at.contains(&a) {
                self.at.push(a);
                changed = true;
            }
        }
        if self.at.len() > OFFSET_CAP {
            self.wide = true;
        }
        changed
    }

    fn single(&self) -> Option<i64> {
        match self.at.as_slice() {
            [a] if !self.wide => Some(*a),
            _ => None,
        }
    }
}

/// Every value holding the entry stack pointer plus a constant, followed
/// forward from the stack pointer through copies, constant additions and
/// subtractions, phis and indirect effects, and every other read of one:
/// `(value, op, input slot)`.
struct Frame {
    values: HashMap<VarnodeId, Offsets>,
    uses: Vec<(VarnodeId, OpId, int4)>,
}

fn frame(fd: &Funcdata, sp: VarnodeId) -> Option<Frame> {
    let mut values: HashMap<VarnodeId, Offsets> = HashMap::new();
    values.insert(
        sp,
        Offsets {
            at: vec![0],
            wide: false,
        },
    );
    let mut uses: HashSet<(VarnodeId, OpId, int4)> = HashSet::new();
    let mut work = vec![sp];
    while let Some(vn) = work.pop() {
        if values.len() > VALUE_CAP {
            return None;
        }
        let cur = values.get(&vn)?.clone();
        let readers: Vec<OpId> = fd.vbank().get(vn)?.descend_iter().collect();
        for op in readers {
            let Some(o) = fd.obank().get(op) else {
                continue;
            };
            for slot in (0..o.num_input()).filter(|&i| o.get_in(i) == Some(vn)) {
                let other = |i: int4| o.get_in(i).and_then(|x| constant(fd, x));
                let next = match o.code() {
                    OpCode::CPUI_COPY | OpCode::CPUI_MULTIEQUAL => Some(cur.clone()),
                    OpCode::CPUI_INDIRECT if slot == 0 => Some(cur.clone()),
                    OpCode::CPUI_INT_ADD => other(1 - slot).map(|c| cur.shifted(c as i64)),
                    OpCode::CPUI_INT_SUB if slot == 0 => {
                        other(1).map(|c| cur.shifted((c as i64).wrapping_neg()))
                    }
                    _ => None,
                };
                match (next, o.get_out()) {
                    (Some(next), Some(out)) => {
                        if values.entry(out).or_default().merge(&next) {
                            work.push(out);
                        }
                    }
                    _ => {
                        uses.insert((vn, op, slot));
                    }
                }
            }
        }
    }
    let mut uses: Vec<_> = uses.into_iter().collect();
    uses.sort_unstable();
    Some(Frame { values, uses })
}

/// A `STORE` of an incoming register: its address operand and the register.
fn store_of_incoming(fd: &Funcdata, op: OpId) -> Option<(VarnodeId, VarnodeId)> {
    let o = fd.obank().get(op)?;
    if o.code() != OpCode::CPUI_STORE {
        return None;
    }
    Some((o.get_in(1)?, incoming(fd, o.get_in(2)?)?))
}

/// The xmm save block: `(register index, frame offset)` of each store, when
/// the block holds nothing else.
fn xmm_saves(fd: &Funcdata, fr: &Frame, block: BlockId, xmm: &[Reg]) -> Option<Vec<(u32, i64)>> {
    let mut saves = Vec::new();
    for op in fd.bb_ops(block) {
        let o = fd.obank().get(op)?;
        match o.code() {
            OpCode::CPUI_STORE => {
                let (addr, reg) = store_of_incoming(fd, op)?;
                let j = xmm.iter().position(|r| is_reg(fd, reg, r))? as u32;
                saves.push((j, fr.values.get(&addr)?.single()?));
            }
            OpCode::CPUI_INDIRECT => {}
            OpCode::CPUI_COPY | OpCode::CPUI_INT_ADD => {
                let out = fd.vbank().get(o.get_out()?)?;
                if out.get_space().get_type() != spacetype::IPTR_INTERNAL {
                    return None;
                }
            }
            _ => return None,
        }
    }
    Some(saves)
}

/// Does the guard's skipped successor look like the xmm save block?  Checked
/// before [`frame`] runs, so a function without the prologue pays nothing.
fn xmm_block_shape(fd: &Funcdata, block: BlockId, xmm: &[Reg]) -> bool {
    let ops = fd.bb_ops(block);
    !ops.is_empty()
        && ops
            .iter()
            .all(|&op| match fd.obank().get(op).map(|o| o.code()) {
                Some(OpCode::CPUI_STORE) => store_of_incoming(fd, op)
                    .is_some_and(|(_, reg)| xmm.iter().any(|r| is_reg(fd, reg, r))),
                Some(OpCode::CPUI_INDIRECT | OpCode::CPUI_COPY | OpCode::CPUI_INT_ADD) => true,
                _ => false,
            })
}

/// The blocks whose stores may be the general-register saves: the guard and
/// the straight line of blocks that reaches it from the entry.
fn prologue_blocks(fd: &Funcdata, guard: BlockId) -> Vec<BlockId> {
    let blocks = fd.bblocks_ref();
    let mut chain = vec![guard];
    let mut cur = guard;
    while blocks.block(cur).size_in() == 1 && chain.len() < 8 {
        let pred = blocks.block(cur).get_in(0);
        if blocks.block(pred).size_out() != 1 || chain.contains(&pred) {
            break;
        }
        chain.push(pred);
        cur = pred;
    }
    chain
}

/// Does the body read the save area at `area`, whose saved slots are
/// `saved`, itself?  See the module notes for what counts.
fn body_reads_save_area(
    fd: &Funcdata,
    fr: &Frame,
    sp: VarnodeId,
    area: i64,
    saved: &[(i64, i64)],
) -> bool {
    let sp_addr = fd.vbank().get(sp).map(|v| v.get_addr().clone());
    let mut fields: Vec<(i64, i64)> = Vec::new();
    let mut loads: Vec<(&Offsets, i64)> = Vec::new();
    for &(vn, op, slot) in &fr.uses {
        let Some(o) = fd.obank().get(op) else {
            continue;
        };
        let offsets = &fr.values[&vn];
        let to_base = offsets.at.contains(&area);
        let to_saved = offsets.at.iter().any(|&a| overlaps(a, 1, saved));
        match (o.code(), slot) {
            (OpCode::CPUI_STORE, 1) => continue,
            (OpCode::CPUI_STORE, 2) if to_base && !to_saved => {
                let field = o
                    .get_in(1)
                    .and_then(|a| fr.values.get(&a))
                    .and_then(|a| a.single());
                match field {
                    Some(f) => fields.push((f - VA_LIST_HEAD, f + VA_LIST_FIELD)),
                    None => return true,
                }
                continue;
            }
            (OpCode::CPUI_INT_AND, _)
                if o.get_out()
                    .and_then(|v| fd.vbank().get(v))
                    .map(|v| v.get_addr().clone())
                    == sp_addr =>
            {
                return true;
            }
            _ => {}
        }
        if (to_base || to_saved) && carries_pointer(fd, o) {
            return true;
        }
        if o.code() == OpCode::CPUI_LOAD && slot == 1 {
            let size = o
                .get_out()
                .and_then(|v| fd.vbank().get(v))
                .map_or(1, |v| v.get_size()) as i64;
            loads.push((offsets, size));
        }
    }
    loads.into_iter().any(|(offsets, size)| {
        offsets
            .at
            .iter()
            .any(|&a| overlaps(a, size, saved) || overlaps(a, size, &fields))
    })
}

/// Can `op` read memory through, hand on or compute a further pointer from
/// a pointer input?  A flag, a comparison or a narrow piece of a 64-bit stack
/// address (`xor %eax,%eax` reads the `eax` half of a `rax` that holds one)
/// cannot.
fn carries_pointer(fd: &Funcdata, op: &crate::op::PcodeOp) -> bool {
    matches!(
        op.code(),
        OpCode::CPUI_LOAD
            | OpCode::CPUI_STORE
            | OpCode::CPUI_CALL
            | OpCode::CPUI_CALLIND
            | OpCode::CPUI_CALLOTHER
            | OpCode::CPUI_RETURN
            | OpCode::CPUI_BRANCHIND
    ) || op
        .get_out()
        .and_then(|v| fd.vbank().get(v))
        .is_some_and(|v| v.get_size() >= 8)
}

/// The save area `R` of the sorted xmm stores `(j, frame offset)`, when they
/// are the registers `m..m+c-1` at `R + 48 + 16*j`.
fn save_area(saves: &[(u32, i64)]) -> Option<i64> {
    let &(first, at) = saves.first()?;
    let area = at - GP_AREA - 16 * first as i64;
    contiguous(saves.iter().map(|&(j, _)| j))
        .then_some(area)
        .filter(|_| {
            saves
                .iter()
                .all(|&(j, off)| off == area + GP_AREA + 16 * j as i64)
        })
}

/// Are the sorted register indices consecutive?
fn contiguous(regs: impl Iterator<Item = u32>) -> bool {
    let regs: Vec<u32> = regs.collect();
    regs.windows(2).all(|w| w[1] == w[0] + 1)
}

/// Does `[at, at + size)` overlap one of the `[lo, hi)` slots?
fn overlaps(at: i64, size: i64, slots: &[(i64, i64)]) -> bool {
    slots.iter().any(|&(lo, hi)| at < hi && lo < at + size)
}

fn find(fd: &Funcdata) -> Option<Plan> {
    let al = register(fd, "AL")?;
    let xmm: Vec<Reg> = XMM_ARGS
        .iter()
        .map(|n| register(fd, n))
        .collect::<Option<_>>()?;
    let blocks = fd.bblocks_ref();
    let candidate = (0..fd.bblocks_get_size())
        .map(|i| fd.bblocks_get_block(i))
        .find(|&guard| {
            let g = blocks.block(guard);
            if g.size_out() != 2 || g.get_flip_path() {
                return false;
            }
            let Some(tail) = fd.bb_op_tail(guard).and_then(|t| fd.obank().get(t)) else {
                return false;
            };
            if tail.code() != OpCode::CPUI_CBRANCH
                || !tail.get_in(1).is_some_and(|c| al_is_zero(fd, c, &al))
            {
                return false;
            }
            let (save_block, join) = (g.get_false_out(), g.get_true_out());
            let s = blocks.block(save_block);
            s.size_in() == 1
                && s.size_out() == 1
                && s.get_out(0) == join
                && blocks.block(join).size_in() == 2
                && xmm_block_shape(fd, save_block, &xmm)
        })?;
    let gp: Vec<Reg> = GP_ARGS
        .iter()
        .map(|n| register(fd, n))
        .collect::<Option<_>>()?;
    let stack = fd.get_arch().manage().get_stack_space()?.clone();
    let sp = fd.find_spacebase_input(&stack)?;
    if fd.vbank().get(sp)?.get_size() != 8 {
        return None;
    }
    let fr = frame(fd, sp)?;
    let save_block = blocks.block(candidate).get_false_out();
    let mut xmm_saves = xmm_saves(fd, &fr, save_block, &xmm)?;
    xmm_saves.sort_unstable();
    let area = save_area(&xmm_saves)?;
    let mut gp_saves: Vec<(u32, OpId)> = Vec::new();
    for block in prologue_blocks(fd, candidate) {
        for op in fd.bb_ops(block) {
            let Some((addr, reg)) = store_of_incoming(fd, op) else {
                continue;
            };
            let Some(k) = gp.iter().position(|r| is_reg(fd, reg, r)) else {
                continue;
            };
            if fr.values.get(&addr).and_then(|a| a.single()) == Some(area + 8 * k as i64) {
                gp_saves.push((k as u32, op));
            }
        }
    }
    gp_saves.sort_unstable();
    if !contiguous(gp_saves.iter().map(|&(k, _)| k)) {
        return None;
    }
    let mut saved: Vec<(i64, i64)> = gp_saves
        .iter()
        .map(|&(k, _)| (area + 8 * k as i64, area + 8 * k as i64 + 8))
        .collect();
    saved.extend(xmm_saves.iter().map(|&(_, off)| (off, off + 16)));
    if body_reads_save_area(fd, &fr, sp, area, &saved) {
        return None;
    }
    Some(Plan {
        guard: candidate,
        edge: blocks.block(candidate).get_out_index(save_block),
        gp_stores: gp_saves.into_iter().map(|(_, op)| op).collect(),
    })
}

#[cfg(test)]
mod tests;
