//! (kuna `alloca`) A dynamic stack allocation becomes `alloca(size)`.
//!
//! `alloca(n)` and a variable-length array compile to a subtraction of the
//! rounded size from the stack pointer:
//!
//! ```text
//!   lea  rax,[n+15]          ; or n+0x17 / n+0x1b with alignment slack
//!   and  rax,-16             ; or shr 4; shl 4
//!   sub  rsp,rax
//!   lea  rax,[rsp+15]        ; optional re-alignment of the buffer
//!   and  rax,-16
//! ```
//!
//! The decompiled C kept that arithmetic on a stack symbol,
//! `use(&(&Stackfffffffffffffff8)[v1],a0)` with
//! `v1 = -((long)a0 + 0xfU & 0xfffffffffffffff0)`. [`RuleAlloca`]
//! rewrites the subtraction itself: an `INT_ADD` of a stack-pointer value and
//! the negation of a non-constant size becomes `alloca(n)`, where `n` is the
//! size before the round-up when the round-up is recognized, and the
//! re-alignment of that result is dropped. Under stack-clash protection the
//! allocation is split into a page part, which a probe loop takes, and a
//! remainder `(n + c) & 0xff0`; the remainder's subtraction is the one that
//! becomes `alloca(n)`, and [`RuleAllocaProbe`] removes the loop and probes.
//! A stack move neither of them caught before types were in is still the
//! typed `PTRADD(stack pointer, -n, 1)` written back to the stack-pointer
//! register; [`RuleAllocaTyped`] turns that into `alloca(n)` as well.

use kuna_base::types::{int4, uintb};
use kuna_num::opcodes::OpCode;

use crate::action::{ActionGroupList, Rule};
use crate::context::{OpId, VarnodeId};
use crate::funcdata::Funcdata;
use crate::userop::BUILTIN_ALLOCA;

const MAX_SLACK: u64 = 0x40;

fn def(data: &Funcdata, vn: VarnodeId) -> Option<(OpCode, Vec<VarnodeId>)> {
    let o = data.obank().get(data.vbank().get(vn)?.get_def()?)?;
    Some((o.code(), (0..o.num_input()).filter_map(|i| o.get_in(i)).collect()))
}

fn constant(data: &Funcdata, vn: VarnodeId) -> Option<u64> {
    data.vbank().get(vn).filter(|v| v.is_constant()).map(|v| v.get_offset())
}

fn signed(data: &Funcdata, vn: VarnodeId) -> Option<i64> {
    let v = data.vbank().get(vn).filter(|v| v.is_constant())?;
    sign_extend(v.get_offset(), v.get_size())
}

/// `offset` read as a signed `size`-byte value; `None` past 8 bytes.
fn sign_extend(offset: u64, size: int4) -> Option<i64> {
    let bits = 64u32.checked_sub(8 * u32::try_from(size).ok()?).filter(|&b| b < 64)?;
    Some(((offset << bits) as i64) >> bits)
}

/// Is `vn` the stack pointer: its entry value, or an earlier `alloca` result,
/// plus or minus constants?
fn stack_pointer(data: &Funcdata, vn: VarnodeId) -> bool {
    let mut vn = vn;
    for _ in 0..16 {
        let Some(v) = data.vbank().get(vn) else { return false };
        if (v.is_spacebase() && v.is_input()) || is_alloca(data, vn) {
            return true;
        }
        match def(data, vn) {
            Some((OpCode::CPUI_INT_ADD, ins)) if ins.len() == 2 && constant(data, ins[1]).is_some() => vn = ins[0],
            Some((OpCode::CPUI_COPY, ins)) if ins.len() == 1 => vn = ins[0],
            _ => return false,
        }
    }
    false
}

/// The negated amount in `vn`: `-(x)`, `x * -1`.
fn negated(data: &Funcdata, vn: VarnodeId) -> Option<VarnodeId> {
    match def(data, vn)? {
        (OpCode::CPUI_INT_2COMP, ins) if ins.len() == 1 => Some(ins[0]),
        (OpCode::CPUI_INT_MULT, ins) if ins.len() == 2 && signed(data, ins[1]) == Some(-1) => Some(ins[0]),
        _ => None,
    }
}

/// An alignment mask `-a`, `a` a power of two from 4 to 64.
fn align_mask(m: i64) -> bool {
    (-64..=-4).contains(&m) && m.unsigned_abs().is_power_of_two()
}

/// A page mask `-p`, `p` a power of two of at least 4096: the part of a
/// stack-clash allocation the probe loop moves the stack pointer by.
fn page_mask(m: i64) -> bool {
    m <= -4096 && m.unsigned_abs().is_power_of_two()
}

/// The remainder mask `p - a` that a stack-clash allocation subtracts after
/// its probe loop (`and $0xff0`).
fn remainder_mask(m: i64) -> bool {
    if m <= 0 {
        return false;
    }
    let low = m & -m;
    (4..=64).contains(&low) && m.checked_add(low).is_some_and(|s| s >= 4096 && (s as u64).is_power_of_two())
}

/// `(n + c) & -4096`: the page part a stack-clash probe loop allocates.
fn is_page_part(data: &Funcdata, vn: VarnodeId) -> bool {
    matches!(def(data, vn), Some((OpCode::CPUI_INT_AND, a)) if a.len() == 2 && signed(data, a[1]).is_some_and(page_mask))
}

/// Look through the width changes a 32-bit `and` puts around a 64-bit size.
fn strip_extension(data: &Funcdata, vn: VarnodeId) -> VarnodeId {
    let mut vn = vn;
    for _ in 0..3 {
        match def(data, vn) {
            Some((OpCode::CPUI_INT_ZEXT | OpCode::CPUI_INT_SEXT, ins)) if ins.len() == 1 => vn = ins[0],
            Some((OpCode::CPUI_SUBPIECE, ins)) if ins.len() == 2 && constant(data, ins[1]) == Some(0) => vn = ins[0],
            _ => break,
        }
    }
    vn
}

/// The size before a 16-byte round-up `(n + c) & -16` (or `>> 4 << 4`), or
/// before the remainder part `(n + c) & 0xff0` of a stack-clash allocation,
/// whose page part the probe loop has already taken.
fn unrounded(data: &Funcdata, vn: VarnodeId) -> Option<VarnodeId> {
    let vn = strip_extension(data, vn);
    let sum = match def(data, vn)? {
        (OpCode::CPUI_INT_AND, ins) if ins.len() == 2 && signed(data, ins[1]).is_some_and(|m| align_mask(m) || remainder_mask(m)) => ins[0],
        (OpCode::CPUI_INT_LEFT, ins) if ins.len() == 2 => {
            let k = constant(data, ins[1])?;
            match def(data, ins[0])? {
                (OpCode::CPUI_INT_RIGHT, r) if r.len() == 2 && constant(data, r[1]) == Some(k) => r[0],
                _ => return None,
            }
        }
        _ => return None,
    };
    match def(data, strip_extension(data, sum))? {
        (OpCode::CPUI_INT_ADD, ins) if ins.len() == 2 && constant(data, ins[1]).is_some_and(|c| c >= 3 && c < MAX_SLACK) => Some(ins[0]),
        _ => None,
    }
}

fn is_alloca(data: &Funcdata, vn: VarnodeId) -> bool {
    match def(data, vn) {
        Some((OpCode::CPUI_CALLOTHER, ins)) => ins.first().and_then(|&i| constant(data, i)) == Some(BUILTIN_ALLOCA as u64),
        _ => false,
    }
}


/// Turn `sp = sp - reg` into `sp = alloca(reg)` in the raw p-code, before
/// heritage, so the allocation is one definition of the stack pointer that
/// later rules cannot duplicate. Only a size the same block rounded to the
/// stack alignment (`and reg,-16`, `shl reg,4`, or the stack-clash remainder
/// `and reg,0xff0`) counts, and not one the block computed from constants:
/// `mov x9,#N; bl ___chkstk_darwin; sub sp,sp,x9,lsl #4` is a fixed frame.
pub fn rewrite_raw(data: &mut Funcdata) {
    if !data.get_arch().alloca_calls {
        return;
    }
    let Some(base) = data.get_arch().manage().get_stack_space().and_then(|s| s.get_spacebase(0).ok()) else { return };
    let Some(space) = base.space.as_ref() else { return };
    let sp = (space.get_index(), base.offset, base.size as int4);
    let key = |data: &Funcdata, vn: VarnodeId| {
        data.vbank().get(vn).and_then(|v| Some((v.get_addr().get_space()?.get_index(), v.get_offset(), v.get_size())))
    };
    let mut rewrites = Vec::new();
    for i in 0..data.bblocks_get_size() {
        let block = data.bblocks_get_block(i);
        let mut last: std::collections::HashMap<(int4, u64, int4), OpId> = std::collections::HashMap::new();
        let mut rounded: std::collections::HashSet<(int4, u64, int4)> = std::collections::HashSet::new();
        let mut constants: std::collections::HashSet<(int4, u64, int4)> = std::collections::HashSet::new();
        for id in data.bb_ops(block) {
            let Some(op) = data.obank().get(id) else { continue };
            let out = op.get_out().and_then(|o| key(data, o));
            let sub_into = |sub: OpId| -> Option<OpId> {
                let s = data.obank().get(sub)?;
                if s.code() != OpCode::CPUI_INT_SUB || key(data, s.get_in(0)?) != Some(sp) {
                    return None;
                }
                let amount = s.get_in(1)?;
                let k = key(data, amount);
                if data.vbank().get(amount)?.is_constant()
                    || !k.is_some_and(|k| rounded.contains(&k))
                    || k.is_some_and(|k| constants.contains(&k))
                {
                    return None;
                }
                Some(sub)
            };
            if out == Some(sp) {
                let target = match op.code() {
                    OpCode::CPUI_INT_SUB => sub_into(id),
                    OpCode::CPUI_COPY => op.get_in(0).and_then(|v| key(data, v)).and_then(|k| last.get(&k).copied()).and_then(sub_into),
                    _ => None,
                };
                rewrites.extend(target);
            }
            if let Some(k) = out {
                last.insert(k, id);
                let mask = op.get_in(1).and_then(|c| signed(data, c));
                let carried = || op.get_in(0).and_then(|v| key(data, v)).is_some_and(|k| rounded.contains(&k));
                let is_rounded = match op.code() {
                    OpCode::CPUI_INT_AND => mask.is_some_and(|m| align_mask(m) || remainder_mask(m)),
                    OpCode::CPUI_INT_LEFT => mask.is_some_and(|k| (2..=6).contains(&k)),
                    OpCode::CPUI_COPY | OpCode::CPUI_INT_ZEXT => carried(),
                    _ => false,
                };
                if is_rounded {
                    rounded.insert(k);
                } else {
                    rounded.remove(&k);
                }
                let known = |slot: int4| {
                    op.get_in(slot).is_some_and(|v| {
                        data.vbank().get(v).is_some_and(|x| x.is_constant()) || key(data, v).is_some_and(|k| constants.contains(&k))
                    })
                };
                let is_constant = match op.code() {
                    OpCode::CPUI_COPY | OpCode::CPUI_INT_ZEXT => known(0),
                    OpCode::CPUI_INT_AND | OpCode::CPUI_INT_LEFT | OpCode::CPUI_INT_ADD | OpCode::CPUI_INT_OR => known(0) && known(1),
                    _ => false,
                };
                if is_constant {
                    constants.insert(k);
                } else {
                    constants.remove(&k);
                }
            }
        }
    }
    for op in rewrites {
        let Some(amount) = data.obank().get(op).and_then(|o| o.get_in(1)) else { continue };
        let id = data.new_constant(4, BUILTIN_ALLOCA as uintb);
        if data.op_set_all_input(op, &[id, amount]).is_ok() {
            data.op_set_opcode(op, crate::typeop::type_op_for(OpCode::CPUI_CALLOTHER));
        }
    }
}

pub struct RuleAlloca {
    group: String,
}

impl RuleAlloca {
    pub fn new(group: &str) -> RuleAlloca {
        RuleAlloca { group: group.to_string() }
    }
}

impl Rule for RuleAlloca {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_INT_ADD, OpCode::CPUI_INT_AND, OpCode::CPUI_CALLOTHER]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        grouplist.contains(&self.group).then(|| Box::new(RuleAlloca::new(&self.group)) as Box<dyn Rule>)
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        if !data.get_arch().alloca_calls {
            return 0;
        }
        let Some(o) = data.obank().get(op) else { return 0 };
        let (Some(a), Some(b)) = (o.get_in(0), o.get_in(1)) else { return 0 };
        if o.code() == OpCode::CPUI_CALLOTHER {
            // `alloca((n + 15) & -16)` from the raw rewrite: the size as written.
            if constant(data, a) != Some(BUILTIN_ALLOCA as u64) {
                return 0;
            }
            let Some(size) = unrounded(data, b) else { return 0 };
            return i32::from(data.op_set_input(op, size, 1).is_ok());
        }
        if o.code() == OpCode::CPUI_INT_AND {
            // `(alloca(n) + 15) & -16`: the buffer alloca returns is aligned.
            let aligned = signed(data, b).is_some_and(|m| m < 0 && m.unsigned_abs().is_power_of_two())
                && matches!(def(data, a), Some((OpCode::CPUI_INT_ADD, ins)) if ins.len() == 2 && is_alloca(data, ins[0]) && constant(data, ins[1]).is_some_and(|c| c < MAX_SLACK));
            if !aligned {
                return 0;
            }
            let Some((_, ins)) = def(data, a) else { return 0 };
            if data.op_set_all_input(op, &[ins[0]]).is_err() {
                return 0;
            }
            data.op_set_opcode(op, crate::typeop::type_op_for(OpCode::CPUI_COPY));
            return 1;
        }
        let (sp, neg) = if stack_pointer(data, a) { (a, b) } else if stack_pointer(data, b) { (b, a) } else { return 0 };
        let _ = sp;
        let Some(amount) = negated(data, neg) else { return 0 };
        let Some(size) = unrounded(data, amount) else { return 0 };
        // `p = n > k ? alloca(n) : buf`: the frame array is laid out from the
        // stack arithmetic the merge sees, so that shape keeps it.
        let Some(out) = o.get_out() else { return 0 };
        let merges_frame = data.vbank().get(out).is_some_and(|v| {
            v.descend_iter().any(|d| {
                data.obank().get(d).is_some_and(|m| {
                    m.code() == OpCode::CPUI_MULTIEQUAL
                        && (0..m.num_input()).filter_map(|i| m.get_in(i)).any(|i| i != out && stack_derived(data, i, 8))
                })
            })
        });
        if merges_frame {
            return 0;
        }
        let id = data.new_constant(4, BUILTIN_ALLOCA as uintb);
        if data.op_set_all_input(op, &[id, size]).is_err() {
            return 0;
        }
        data.op_set_opcode(op, crate::typeop::type_op_for(OpCode::CPUI_CALLOTHER));
        1
    }
}

/// Is `vn` an address into the stack or into an `alloca` buffer?
fn stack_derived(data: &Funcdata, vn: VarnodeId, depth: u32) -> bool {
    let Some(v) = data.vbank().get(vn) else { return false };
    if v.is_spacebase() && v.is_input() {
        return true;
    }
    if depth == 0 {
        return false;
    }
    match def(data, vn) {
        Some((OpCode::CPUI_INT_ADD | OpCode::CPUI_PTRADD | OpCode::CPUI_PTRSUB, ins)) => {
            ins.iter().any(|&i| constant(data, i).is_none() && stack_derived(data, i, depth - 1))
        }
        Some((OpCode::CPUI_COPY | OpCode::CPUI_CAST, ins)) => ins.first().is_some_and(|&i| stack_derived(data, i, depth - 1)),
        Some((OpCode::CPUI_CALLOTHER, _)) => is_alloca(data, vn),
        _ => false,
    }
}

/// Do two address Varnodes compute the same address (the same Varnode, or the
/// same op over the same inputs)?
fn same_address(data: &Funcdata, a: VarnodeId, b: VarnodeId) -> bool {
    if a == b {
        return true;
    }
    match (def(data, a), def(data, b)) {
        (Some((oa, ia)), Some((ob, ib))) if oa == ob && ia.len() == ib.len() => ia.iter().zip(&ib).all(|(&x, &y)| {
            x == y || (constant(data, x).is_some() && constant(data, x) == constant(data, y) && data.vbank().get(x).map(|v| v.get_size()) == data.vbank().get(y).map(|v| v.get_size()))
        }),
        _ => false,
    }
}

/// The limit of a stack-clash allocation's probe loop: an `alloca` result, or
/// the stack pointer less the page part `(n + c) & -4096`.
fn is_limit(data: &Funcdata, vn: VarnodeId) -> bool {
    if is_alloca(data, vn) {
        return true;
    }
    let Some((OpCode::CPUI_INT_ADD, ins)) = def(data, vn) else { return false };
    if ins.len() != 2 || !stack_pointer(data, ins[0]) {
        return false;
    }
    negated(data, ins[1]).is_some_and(|amount| is_page_part(data, amount))
}

/// The comparison a CBRANCH tests, as its two operands.
fn compared(data: &Funcdata, cbranch: OpId) -> Option<(VarnodeId, VarnodeId)> {
    let cond = data.obank().get(cbranch)?.get_in(1)?;
    match def(data, cond)? {
        (
            OpCode::CPUI_INT_EQUAL
            | OpCode::CPUI_INT_NOTEQUAL
            | OpCode::CPUI_INT_LESS
            | OpCode::CPUI_INT_SLESS
            | OpCode::CPUI_INT_LESSEQUAL
            | OpCode::CPUI_INT_SLESSEQUAL,
            ins,
        ) if ins.len() == 2 => Some((ins[0], ins[1])),
        _ => None,
    }
}

/// Does `block` end in a CBRANCH that compares against `limit`?
fn tests_limit(data: &Funcdata, block: crate::context::BlockId, limit: VarnodeId) -> bool {
    let Some(&last) = data.bb_ops(block).last() else { return false };
    data.obank().get(last).is_some_and(|o| o.code() == OpCode::CPUI_CBRANCH)
        && compared(data, last).is_some_and(|(a, b)| a == limit || b == limit)
}

/// The probe of a stack-clash allocation: a store back of the word just loaded
/// from the same stack address (`or qword [rsp+k],0`).
fn is_probe(data: &Funcdata, store: OpId) -> bool {
    let Some(o) = data.obank().get(store) else { return false };
    let (Some(addr), Some(value)) = (o.get_in(1), o.get_in(2)) else { return false };
    let loaded = match def(data, value) {
        Some((OpCode::CPUI_LOAD, ins)) => ins.get(1).copied(),
        Some((OpCode::CPUI_INT_OR, ins)) if ins.len() == 2 && constant(data, ins[1]) == Some(0) => match def(data, ins[0]) {
            Some((OpCode::CPUI_LOAD, l)) => l.get(1).copied(),
            _ => None,
        },
        _ => None,
    };
    loaded.is_some_and(|l| same_address(data, l, addr)) && stack_derived(data, addr, 8)
}

/// Remove the probe loop and probes of a stack-clash dynamic allocation.
///
/// gcc `-fstack-clash-protection` (the Ubuntu default) and clang
/// `-fstack-clash-protection` allocate `alloca(n)` a page at a time, touching
/// each page: a guard `sp == limit` skips a loop that lowers the stack
/// pointer by 4096 and stores back the word it loads until it reaches the
/// limit, and gcc probes the remainder once more. The allocation itself is
/// `alloca(n)` (RuleAlloca); the probes are stores of a value back to where
/// it was loaded and change nothing, so they are removed, and the guard is
/// pinned to skip the loop, which leaves the loop unreachable.
pub struct RuleAllocaProbe {
    group: String,
}

impl RuleAllocaProbe {
    pub fn new(group: &str) -> RuleAllocaProbe {
        RuleAllocaProbe { group: group.to_string() }
    }
}

impl Rule for RuleAllocaProbe {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_CBRANCH, OpCode::CPUI_STORE]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        grouplist.contains(&self.group).then(|| Box::new(RuleAllocaProbe::new(&self.group)) as Box<dyn Rule>)
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        if !data.get_arch().alloca_calls {
            return 0;
        }
        let Some(o) = data.obank().get(op) else { return 0 };
        if o.code() == OpCode::CPUI_STORE {
            if !is_probe(data, op) {
                return 0;
            }
            data.op_destroy(op);
            return 1;
        }
        let Some((a, b)) = compared(data, op) else { return 0 };
        let limit = if is_limit(data, a) && stack_pointer(data, b) {
            a
        } else if is_limit(data, b) && stack_pointer(data, a) {
            b
        } else {
            return 0;
        };
        let flip = o.is_boolean_flip();
        let Some(block) = o.get_parent() else { return 0 };
        let bb = data.bblocks_ref().block(block);
        if bb.size_out() != 2 {
            return 0;
        }
        let (false_out, true_out) = (bb.get_false_out(), bb.get_true_out());
        let (loop_side, skip) = match (tests_limit(data, false_out, limit), tests_limit(data, true_out, limit)) {
            (true, false) => (false_out, true_out),
            (false, true) => (true_out, false_out),
            _ => return 0,
        };
        let _ = loop_side;
        let taken = (skip == true_out) != flip;
        let value = data.new_constant(1, u64::from(taken));
        if data.op_set_input(op, value, 1).is_err() {
            return 0;
        }
        1
    }
}

/// `n` when `v` is `-n` with `n` not a constant.
fn negated_amount(data: &Funcdata, v: VarnodeId) -> Option<VarnodeId> {
    let d = data.obank().get(data.vbank().get(v)?.get_def()?)?;
    let n = match d.code() {
        OpCode::CPUI_INT_2COMP => d.get_in(0)?,
        OpCode::CPUI_INT_MULT => {
            let (a, b) = (d.get_in(0)?, d.get_in(1)?);
            let minus_one = |x: VarnodeId| data.vbank().get(x).is_some_and(|c| {
                let m = if c.get_size() >= 8 { u64::MAX } else { (1u64 << (8 * c.get_size())) - 1 };
                c.is_constant() && c.get_offset() & m == m
            });
            if minus_one(b) { a } else if minus_one(a) { b } else { return None }
        }
        _ => return None,
    };
    (!data.vbank().get(n)?.is_constant()).then_some(n)
}

/// The stack-pointer storage `v` is based on (its spacebase input, or a
/// `PTRSUB` off it), or `None`.
fn stack_pointer_storage(data: &Funcdata, v: VarnodeId) -> Option<kuna_base::address::Address> {
    let n = data.vbank().get(v)?;
    if n.is_spacebase() {
        return Some(n.get_addr().clone());
    }
    let d = data.obank().get(n.get_def()?)?;
    let b = data.vbank().get(d.get_in(0)?)?;
    (d.code() == OpCode::CPUI_PTRSUB && b.is_spacebase()).then(|| b.get_addr().clone())
}

/// Rewrite the typed `PTRADD(sp, -n, 1)` with a run-time `n`, written back to
/// the stack-pointer register, into `alloca(n)`.
pub struct RuleAllocaTyped {
    group: String,
}

impl RuleAllocaTyped {
    pub fn new(group: &str) -> Self {
        Self { group: group.to_string() }
    }
}

impl Rule for RuleAllocaTyped {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_PTRADD]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        grouplist.contains(&self.group).then(|| Box::new(Self::new(&self.group)) as Box<dyn Rule>)
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        if !data.get_arch().alloca_calls {
            return 0;
        }
        let Some(o) = data.obank().get(op) else { return 0 };
        let (Some(base), Some(off), Some(scale)) = (o.get_in(0), o.get_in(1), o.get_in(2)) else { return 0 };
        if data.vbank().get(scale).is_none_or(|s| !s.is_constant() || s.get_offset() != 1) {
            return 0;
        }
        // Only a move OF the stack pointer: the result is written back to the
        // stack-pointer register itself.
        let Some(sp) = stack_pointer_storage(data, base) else { return 0 };
        if o.get_out().and_then(|v| data.vbank().get(v)).is_none_or(|v| *v.get_addr() != sp) {
            return 0;
        }
        let Some(n) = negated_amount(data, off) else { return 0 };
        let id = data.new_constant(4, BUILTIN_ALLOCA as uintb);
        if data.op_set_all_input(op, &[id, n]).is_err() {
            return 0;
        }
        data.op_set_opcode(op, crate::typeop::type_op_for(OpCode::CPUI_CALLOTHER));
        1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wide_constants_do_not_sign_extend() {
        assert_eq!(sign_extend(0xfff0, 2), Some(-16));
        assert_eq!(sign_extend(0xffff_fff0, 8), Some(0xffff_fff0));
        assert_eq!(sign_extend(1, 16), None);
        assert_eq!(sign_extend(1, 0), None);
    }

    #[test]
    fn masks_at_the_i64_extremes() {
        for m in [i64::MIN, i64::MIN + 1, i64::MAX, i64::MAX - 0xf] {
            assert!(!align_mask(m));
            assert!(!remainder_mask(m));
        }
        assert!(page_mask(i64::MIN));
        assert!(align_mask(-16) && page_mask(-4096) && remainder_mask(0xff0));
    }
}
