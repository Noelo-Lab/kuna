//! (kuna `bitidioms`) Byte swaps and rotates written out as shifts and ORs
//! become `bswap(x)` and `rotl(x, n)`.
//!
//! SLEIGH lifts `bswap`/`rev` and `rol`/`ror` into the shift-and-mask p-code
//! they mean, and compilers emit the same shape for the portable C, so the
//! decompiled C spells a byte swap as four masked shifts ORed together.
//! This rule reads the `INT_OR` tree back into the operation it spells:
//!
//! * rotate: `(x << n) | (x >> (W - n))` with constant `n` and the same `x`;
//! * byte swap: followed through byte-aligned shifts, byte masks, ORs of
//!   disjoint bytes, zero extensions and byte pieces, the OR moves byte `i` of
//!   one `x` to byte `N - 1 - i`, every byte exactly once. This also reads the
//!   staged swap AArch64 `rev` lifts to (bytes within halfwords, then
//!   halfwords, then the halves), which is why it runs before the rotate test.

use kuna_base::types::{int4, uintb};
use kuna_num::opcodes::OpCode;

use crate::action::{ActionGroupList, Rule};
use crate::context::{BlockId, OpId, VarnodeId};
use crate::funcdata::Funcdata;
use crate::userop::{BUILTIN_ABS, BUILTIN_LABS, BUILTIN_BITREV, BUILTIN_BSWAP, BUILTIN_CTZ, BUILTIN_ROTL};

/// `[x op c]` with `c` a constant input in slot 1 (or 0 for a commutative op).
fn const_operand(data: &Funcdata, op: OpId) -> Option<(VarnodeId, u64)> {
    let o = data.obank().get(op)?;
    let (a, b) = (o.get_in(0)?, o.get_in(1)?);
    let is_const = |v: VarnodeId| data.vbank().get(v).is_some_and(|v| v.is_constant());
    let value = |v: VarnodeId| data.vbank().get(v).map(|v| v.get_offset());
    if is_const(b) {
        return Some((a, value(b)?));
    }
    if o.code() == OpCode::CPUI_INT_AND && is_const(a) {
        return Some((b, value(a)?));
    }
    None
}

/// Where each bit of `vn` comes from: `Some(i)` is bit `i` of the source `x`,
/// `None` a zero bit. Followed through constant shifts, masks, ORs of disjoint
/// bits, zero extensions and pieces; any other definition is the source.
type Bits = (VarnodeId, Vec<Option<u32>>);

fn bits_of(data: &Funcdata, vn: VarnodeId, depth: u32, budget: &mut u32) -> Option<Bits> {
    let v = data.vbank().get(vn)?;
    let size = v.get_size() as usize;
    if v.is_constant() || size == 0 || size > 8 {
        return None;
    }
    let width = 8 * size;
    let source = Some((vn, (0..width as u32).map(Some).collect()));
    if depth == 0 || *budget == 0 {
        return source;
    }
    *budget -= 1;
    let Some(def) = v.get_def() else { return source };
    let Some(o) = data.obank().get(def) else { return source };
    let mut sub = |vn: VarnodeId| bits_of(data, vn, depth - 1, budget);
    match o.code() {
        OpCode::CPUI_INT_AND => {
            let (a, m) = const_operand(data, def)?;
            let (x, mut b) = sub(a)?;
            for (i, slot) in b.iter_mut().enumerate() {
                if m >> i & 1 == 0 {
                    *slot = None;
                }
            }
            Some((x, b))
        }
        OpCode::CPUI_INT_LEFT | OpCode::CPUI_INT_RIGHT => {
            let (a, c) = const_operand(data, def)?;
            let k = c as usize;
            let (x, b) = sub(a)?;
            let left = o.code() == OpCode::CPUI_INT_LEFT;
            let out = (0..width)
                .map(|i| if left { i.checked_sub(k).and_then(|j| b.get(j).copied().flatten()) } else { b.get(i + k).copied().flatten() })
                .collect();
            Some((x, out))
        }
        OpCode::CPUI_INT_OR => {
            let (x, a) = sub(o.get_in(0)?)?;
            let (y, b) = sub(o.get_in(1)?)?;
            if x != y {
                return None;
            }
            let mut out = Vec::with_capacity(width);
            for (p, q) in a.into_iter().zip(b) {
                out.push(match (p, q) {
                    (Some(_), Some(_)) => return None,
                    (p, None) => p,
                    (None, q) => q,
                });
            }
            Some((x, out))
        }
        OpCode::CPUI_INT_ZEXT => {
            let (x, mut b) = sub(o.get_in(0)?)?;
            b.resize(width, None);
            Some((x, b))
        }
        OpCode::CPUI_SUBPIECE => {
            let k = 8 * o.get_in(1).and_then(|c| data.vbank().get(c)).map(|c| c.get_offset() as usize)?;
            let (x, b) = sub(o.get_in(0)?)?;
            Some((x, (0..width).map(|i| b.get(i + k).copied().flatten()).collect()))
        }
        _ => source,
    }
}

/// What a full permutation of `x`'s bits is.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Reversal {
    Bytes,
    Bits,
}

/// `(x, kind)` when `vn` rotated left by `rot` bits reverses the bytes (or the
/// bits) of an `x` of its own size (`rot` is 0 for the INT_OR itself).
fn reversal_source(data: &Funcdata, vn: VarnodeId, rot: usize) -> Option<(VarnodeId, Reversal)> {
    let size = data.vbank().get(vn)?.get_size();
    if !matches!(size, 1 | 2 | 4 | 8) {
        return None;
    }
    let (x, b) = bits_of(data, vn, 12, &mut 96)?;
    if (x == vn && rot == 0) || data.vbank().get(x)?.get_size() != size {
        return None;
    }
    let w = 8 * size as usize;
    let at = |d: usize| b[(d + w - rot % w) % w];
    if size > 1 && (0..w).all(|d| at(d) == Some(((size as usize - 1 - d / 8) * 8 + d % 8) as u32)) {
        return Some((x, Reversal::Bytes));
    }
    (0..w).all(|d| at(d) == Some((w - 1 - d) as u32)).then_some((x, Reversal::Bits))
}

fn reversal_builtin(kind: Reversal) -> uintb {
    (match kind {
        Reversal::Bytes => BUILTIN_BSWAP,
        Reversal::Bits => BUILTIN_BITREV,
    }) as uintb
}

/// `(x, n)` when the two inputs are `x << n` and `x >> (W - n)`.
fn rotate_source(data: &Funcdata, a: VarnodeId, b: VarnodeId, size: int4) -> Option<(VarnodeId, u64)> {
    let shift = |vn: VarnodeId, code: OpCode| -> Option<(VarnodeId, u64)> {
        let def = data.vbank().get(vn)?.get_def()?;
        (data.obank().get(def)?.code() == code).then(|| const_operand(data, def)).flatten()
    };
    let width = 8 * size as u64;
    for (l, r) in [(a, b), (b, a)] {
        if let (Some((x, n)), Some((y, m))) = (shift(l, OpCode::CPUI_INT_LEFT), shift(r, OpCode::CPUI_INT_RIGHT)) {
            if x == y && n > 0 && n < width && n + m == width && data.vbank().get(x).is_some_and(|v| v.get_size() == size) {
                return Some((x, n));
            }
        }
    }
    None
}

/// Does a reader of the rotate mask away every bit of one of its two shifted
/// halves? Then it is a shift in disguise (PowerPC `rlwinm` writes shifts as
/// rotate-and-mask), and the AND-of-shift rules fold it further than `rotl`.
/// The AND may sit past a zero extension or a `PIECE` (the mask pushed onto a
/// joined pair), whose bits are read at the rotate's position. A mask not yet
/// folded to a constant waits for the next pass.
fn masked_to_one_half(data: &Funcdata, out: VarnodeId, n: u64, size: int4) -> bool {
    let full = if size >= 8 { u64::MAX } else { (1u64 << (8 * size)) - 1 };
    let low = (1u64 << n) - 1;
    let high = full & !low;
    let mut reads: Vec<(VarnodeId, u32)> = vec![(out, 0)];
    let mut depth = 0;
    while let Some((vn, at)) = reads.pop() {
        for d in data.descend_snapshot(vn) {
            let Some(o) = data.obank().get(d) else { continue };
            match o.code() {
                OpCode::CPUI_INT_AND => {
                    let Some((_, m)) = const_operand(data, d) else { return true };
                    let m = m.checked_shr(at).unwrap_or(0) & full;
                    if m & low == 0 || m & high == 0 {
                        return true;
                    }
                }
                OpCode::CPUI_INT_ZEXT if depth < 2 => reads.extend(o.get_out().map(|w| (w, at))),
                OpCode::CPUI_PIECE if depth < 2 => {
                    let shift = if o.get_in(0) == Some(vn) {
                        o.get_in(1).and_then(|l| data.vbank().get(l)).map_or(0, |l| 8 * l.get_size() as u32)
                    } else {
                        0
                    };
                    reads.extend(o.get_out().map(|w| (w, at + shift)));
                }
                _ => {}
            }
        }
        depth += 1;
    }
    false
}

/// `LZCOUNT(bitreverse(x))` (AArch64 `rbit` + `clz`, how compilers spell
/// `__builtin_ctz`) becomes `ctz(x)`.
fn count_trailing(op: OpId, data: &mut Funcdata) -> int4 {
    let Some(input) = data.obank().get(op).and_then(|o| o.get_in(0)) else { return 0 };
    let Some(def) = data.vbank().get(input).and_then(|v| v.get_def()) else { return 0 };
    let Some(d) = data.obank().get(def) else { return 0 };
    if d.code() != OpCode::CPUI_CALLOTHER || d.num_input() != 2 {
        return 0;
    }
    let (Some(id), Some(x)) = (d.get_in(0), d.get_in(1)) else { return 0 };
    if data.vbank().get(id).is_none_or(|v| !v.is_constant() || v.get_offset() != BUILTIN_BITREV as u64) {
        return 0;
    }
    let ctz = data.new_constant(4, BUILTIN_CTZ as uintb);
    if data.op_set_all_input(op, &[ctz, x]).is_err() {
        return 0;
    }
    data.op_set_opcode(op, crate::typeop::type_op_for(OpCode::CPUI_CALLOTHER));
    if data.descend_snapshot(input).is_empty() {
        data.op_destroy(def);
    }
    1
}

/// Turn `rotl(x, n)` back into `x << n | x >> (W - n)`, for a rotate a mask
/// found later (past a `PIECE` the AND was pushed onto) shows is a shift.
fn unrotate(op: OpId, x: VarnodeId, n: u64, size: int4, data: &mut Funcdata) -> kuna_base::error::KunaResult<()> {
    let at = data.obank().get(op).map(|o| o.get_addr().clone()).ok_or_else(|| kuna_base::error::KunaError::lowlevel("unrotate: stale op"))?;
    let half = |code: OpCode, by: u64, data: &mut Funcdata| -> kuna_base::error::KunaResult<VarnodeId> {
        let s = data.new_op(2, at.clone());
        data.op_set_opcode_code(s, code);
        data.op_set_input(s, x, 0)?;
        let c = data.new_constant(4, by as uintb);
        data.op_set_input(s, c, 1)?;
        let out = data.new_unique_out(size, s)?;
        data.op_insert_before(s, op);
        Ok(out)
    };
    let l = half(OpCode::CPUI_INT_LEFT, n, data)?;
    let r = half(OpCode::CPUI_INT_RIGHT, 8 * size as u64 - n, data)?;
    data.op_set_all_input(op, &[l, r])?;
    data.op_set_opcode(op, crate::typeop::type_op_for(OpCode::CPUI_INT_OR));
    Ok(())
}

/// Rewrite a byte swap or a rotate (see the module docs).
pub struct RuleBitIdioms {
    group: String,
}

impl RuleBitIdioms {
    pub fn new(group: &str) -> RuleBitIdioms {
        RuleBitIdioms { group: group.to_string() }
    }
}

impl RuleBitIdioms {
    /// A `rotl(v, 8k)` whose `v` is a partial byte swap the rotate completes
    /// (the last stage of AArch64 `rev`, when the rotate was folded before the
    /// stages under it were simplified) becomes `bswap(x)`.
    fn rotate_to_swap(&self, op: OpId, data: &mut Funcdata) -> int4 {
        let Some(o) = data.obank().get(op) else { return 0 };
        let (Some(id), Some(v), Some(n)) = (o.get_in(0), o.get_in(1), o.get_in(2)) else { return 0 };
        let constant = |vn: VarnodeId| data.vbank().get(vn).filter(|v| v.is_constant()).map(|v| v.get_offset());
        if constant(id) != Some(BUILTIN_ROTL as u64) {
            return 0;
        }
        let Some(n) = constant(n) else { return 0 };
        if let Some(out) = o.get_out() {
            let size = data.vbank().get(out).map_or(0, |w| w.get_size());
            if masked_to_one_half(data, out, n, size) {
                return unrotate(op, v, n, size, data).map_or(0, |()| 1);
            }
        }
        let Some((x, kind)) = reversal_source(data, v, n as usize) else { return 0 };
        let id = data.new_constant(4, reversal_builtin(kind));
        if data.op_set_all_input(op, &[id, x]).is_err() {
            return 0;
        }
        1
    }
}

impl Rule for RuleBitIdioms {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_INT_OR, OpCode::CPUI_CALLOTHER, OpCode::CPUI_LZCOUNT]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        grouplist.contains(&self.group).then(|| Box::new(RuleBitIdioms::new(&self.group)) as Box<dyn Rule>)
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        if !data.get_arch().bit_idioms {
            return 0;
        }
        let Some(o) = data.obank().get(op) else { return 0 };
        if o.code() == OpCode::CPUI_CALLOTHER {
            return self.rotate_to_swap(op, data);
        }
        if o.code() == OpCode::CPUI_LZCOUNT {
            return count_trailing(op, data);
        }
        let (Some(a), Some(b), Some(out)) = (o.get_in(0), o.get_in(1), o.get_out()) else { return 0 };
        let Some(size) = data.vbank().get(out).map(|v| v.get_size()) else { return 0 };
        if let Some((x, kind)) = reversal_source(data, out, 0) {
            let id = data.new_constant(4, reversal_builtin(kind));
            if data.op_set_all_input(op, &[id, x]).is_err() {
                return 0;
            }
            data.op_set_opcode(op, crate::typeop::type_op_for(OpCode::CPUI_CALLOTHER));
            return 1;
        }
        if let Some((x, n)) = rotate_source(data, a, b, size).filter(|&(_, n)| !masked_to_one_half(data, out, n, size)) {
            let id = data.new_constant(4, BUILTIN_ROTL as uintb);
            let count = data.new_constant(4, n as uintb);
            if data.op_set_all_input(op, &[id, x, count]).is_err() {
                return 0;
            }
            data.op_set_opcode(op, crate::typeop::type_op_for(OpCode::CPUI_CALLOTHER));
            return 1;
        }
        0
    }
}


/// `x` when `vn` is `-x` (`INT_2COMP(x)` or `INT_MULT(x, -1)`).
fn negated(data: &Funcdata, vn: VarnodeId) -> Option<VarnodeId> {
    let def = data.vbank().get(vn)?.get_def()?;
    let o = data.obank().get(def)?;
    match o.code() {
        OpCode::CPUI_INT_2COMP => o.get_in(0),
        OpCode::CPUI_INT_MULT => {
            let (x, c) = const_operand(data, def)?;
            let size = data.vbank().get(x)?.get_size();
            let full = if size >= 8 { u64::MAX } else { (1u64 << (8 * size)) - 1 };
            (c & full == full).then_some(x)
        }
        _ => None,
    }
}

/// The value of a signed compare of `x`, `-x` and constants at `x = v`, or
/// `None` when the condition reads anything else.
fn sign_test(data: &Funcdata, cond: VarnodeId, x: VarnodeId, negx: VarnodeId, v: i64) -> Option<bool> {
    let def = data.vbank().get(cond)?.get_def()?;
    let o = data.obank().get(def)?;
    let size = data.vbank().get(x)?.get_size();
    let operand = |vn: VarnodeId| -> Option<i64> {
        if vn == x {
            return Some(v);
        }
        if vn == negx || negated(data, vn) == Some(x) {
            return Some(-v);
        }
        let c = data.vbank().get(vn).filter(|c| c.is_constant())?.get_offset();
        let bits = 8 * size as u32;
        Some(if bits >= 64 { c as i64 } else { ((c << (64 - bits)) as i64) >> (64 - bits) })
    };
    match o.code() {
        OpCode::CPUI_BOOL_NEGATE => sign_test(data, o.get_in(0)?, x, negx, v).map(|b| !b),
        OpCode::CPUI_INT_SLESS => Some(operand(o.get_in(0)?)? < operand(o.get_in(1)?)?),
        OpCode::CPUI_INT_SLESSEQUAL => Some(operand(o.get_in(0)?)? <= operand(o.get_in(1)?)?),
        _ => None,
    }
}

/// `MULTIEQUAL(x, -x)` chosen by a sign test of `x` becomes `abs(x)` (or the
/// MULTIEQUAL stays when the choice is `-abs(x)`): the cmov / `cneg` lowering
/// of `x < 0 ? -x : x`.
pub struct RuleIntAbs {
    group: String,
}

impl RuleIntAbs {
    pub fn new(group: &str) -> RuleIntAbs {
        RuleIntAbs { group: group.to_string() }
    }
}

impl Rule for RuleIntAbs {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_MULTIEQUAL]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        grouplist.contains(&self.group).then(|| Box::new(RuleIntAbs::new(&self.group)) as Box<dyn Rule>)
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        if !data.get_arch().bit_idioms {
            return 0;
        }
        let Some(o) = data.obank().get(op) else { return 0 };
        if o.num_input() != 2 {
            return 0;
        }
        let (Some(in0), Some(in1), Some(out), Some(merge)) = (o.get_in(0), o.get_in(1), o.get_out(), o.get_parent()) else {
            return 0;
        };
        let (x, negx) = if negated(data, in1) == Some(in0) {
            (in0, in1)
        } else if negated(data, in0) == Some(in1) {
            (in1, in0)
        } else {
            return 0;
        };
        let size = data.vbank().get(out).map_or(0, |v| v.get_size());
        if !matches!(size, 1 | 2 | 4 | 8) {
            return 0;
        }
        // Which branch outcome reaches each MULTIEQUAL slot.
        let blocks = data.bblocks_ref();
        let m = blocks.block(merge);
        if m.size_in() != 2 {
            return 0;
        }
        let preds = [m.get_in(0), m.get_in(1)];
        let head = |p: BlockId| -> BlockId {
            let pb = blocks.block(p);
            if pb.size_out() == 1 && pb.size_in() == 1 { pb.get_in(0) } else { p }
        };
        let branch = head(preds[0]);
        if head(preds[1]) != branch || blocks.block(branch).size_out() != 2 {
            return 0;
        }
        let Some(cb) = data.bb_op_tail(branch) else { return 0 };
        let Some(cbo) = data.obank().get(cb) else { return 0 };
        if cbo.code() != OpCode::CPUI_CBRANCH {
            return 0;
        }
        let Some(cond) = cbo.get_in(1) else { return 0 };
        let flip = cbo.is_boolean_flip();
        let bb = blocks.block(branch);
        let (taken_true, taken_false) = if flip { (bb.get_false_out(), bb.get_true_out()) } else { (bb.get_true_out(), bb.get_false_out()) };
        let outcome_of = |slot: usize| -> Option<bool> {
            let p = preds[slot];
            let via = if p == branch {
                (0..bb.size_out()).map(|j| bb.get_out(j)).find(|&s| s == merge)?
            } else {
                p
            };
            if via == taken_true {
                Some(true)
            } else if via == taken_false {
                Some(false)
            } else {
                None
            }
        };
        let (Some(o0), Some(o1)) = (outcome_of(0), outcome_of(1)) else { return 0 };
        if o0 == o1 {
            return 0;
        }
        let (Some(neg_case), Some(pos_case)) = (sign_test(data, cond, x, negx, -5), sign_test(data, cond, x, negx, 5)) else {
            return 0;
        };
        if neg_case == pos_case {
            return 0;
        }
        let value_for = |c: bool| if (c == o0) == (in0 == negx) { negx } else { x };
        if value_for(neg_case) != negx || value_for(pos_case) != x {
            return 0;
        }
        let first = data
            .bb_ops(merge)
            .into_iter()
            .find(|&id| data.obank().get(id).is_some_and(|p| p.code() != OpCode::CPUI_MULTIEQUAL));
        let Some(first) = first else { return 0 };
        let addr = data.obank().get(first).map(|p| p.get_addr().clone());
        let Some(addr) = addr else { return 0 };
        let call = data.new_op(2, addr);
        data.op_set_opcode_code(call, OpCode::CPUI_CALLOTHER);
        let builtin = match size {
            4 => BUILTIN_ABS,
            8 => BUILTIN_LABS,
            _ => return 0,
        };
        let id = data.new_constant(4, builtin as uintb);
        if data.op_set_input(call, id, 0).is_err() || data.op_set_input(call, x, 1).is_err() {
            return 0;
        }
        let Ok(res) = data.new_unique_out(size, call) else { return 0 };
        let int = data.get_arch().types().and_then(|t| t.get_base(size, crate::dtype::type_metatype::TYPE_INT).ok());
        if let (Some(t), Some(n)) = (int, data.vbank_mut().get_mut(res)) {
            n.update_type(t);
        }
        data.op_insert_before(call, first);
        if data.total_replace(out, res).is_err() {
            return 0;
        }
        1
    }
}


/// What a single-instruction bit-scan loop computes.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum ScanKind {
    /// The index of the lowest set bit (x86 `bsf` / `tzcnt`).
    Trailing,
    /// The index of the highest set bit (x86 `bsr`).
    Highest,
}

struct BitScan {
    head: BlockId,
    to_body: i32,
    source: VarnodeId,
    counter: VarnodeId,
    kind: ScanKind,
}

/// Values a scan's source is checked with: every single bit, and mixtures.
fn scan_samples(width: i32) -> Vec<u64> {
    let bits = 8 * width as u32;
    let mut out: Vec<u64> = (0..bits).map(|b| 1u64 << b).collect();
    out.extend([3, 6, 0x60, 0x8001, 0xf0f0, 0x1234_5678, 0xffff_ffff, 0x8000_0000_0000_0001]);
    let m = crate::kuna_strlenscan::mask(width);
    out.into_iter().map(|v| v & m).filter(|&v| v != 0).collect()
}

/// The one-instruction loop SLEIGH lifts `bsf`/`bsr`/`tzcnt` to, proved by
/// running it: for every sample source the counter the head hands to the exit
/// is the lowest (or highest) set bit index.
fn match_bitscan(data: &Funcdata, head: BlockId) -> Option<BitScan> {
    use crate::kuna_strlenscan::{eval, op, size, taken};
    let hb = data.bblocks_ref().block(head);
    if hb.size_in() != 2 || hb.size_out() != 2 {
        return None;
    }
    let (t, f) = taken(data, head)?;
    let body = [t, f].into_iter().find(|&b| {
        let bb = data.bblocks_ref().block(b);
        b != head && bb.size_in() == 1 && bb.size_out() == 1 && bb.get_out(0) == head
    })?;
    let exit = if t == body { f } else { t };
    let region = [head, body];
    let ops: Vec<OpId> = data.bb_ops(head).into_iter().chain(data.bb_ops(body)).collect();
    let at = op(data, *ops.first()?)?.get_addr().clone();
    if !ops.iter().all(|&o| op(data, o).is_some_and(|p| *p.get_addr() == at)) {
        return None;
    }
    let back = data.bblocks_ref().block(head).get_in_index(body);
    let outside = 1 - back;
    let phis: Vec<(VarnodeId, VarnodeId, VarnodeId)> = data
        .bb_ops(head)
        .into_iter()
        .filter_map(|o| op(data, o).filter(|p| p.code() == OpCode::CPUI_MULTIEQUAL))
        .map(|p| Some((p.get_out()?, p.get_in(outside)?, p.get_in(back)?)))
        .collect::<Option<Vec<_>>>()?;
    if phis.is_empty() || phis.len() > 3 {
        return None;
    }
    let in_region = |v: VarnodeId| {
        data.vbank().get(v).and_then(|n| n.get_def()).and_then(|d| op(data, d)).and_then(|p| p.get_parent()).is_some_and(|b| region.contains(&b))
    };
    let is_const = |v: VarnodeId| crate::kuna_strlenscan::copied_constant(data, v).is_some();
    // The source: the one non-constant value entering the loop.
    let mut leaves: Vec<VarnodeId> = phis.iter().map(|p| p.1).filter(|&v| !is_const(v)).collect();
    for &o in &ops {
        let p = op(data, o)?;
        if p.code() == OpCode::CPUI_MULTIEQUAL {
            continue;
        }
        for i in 0..p.num_input() {
            let v = p.get_in(i)?;
            if !is_const(v) && !in_region(v) && !phis.iter().any(|ph| ph.0 == v) {
                leaves.push(v);
            }
        }
    }
    leaves.sort_unstable();
    leaves.dedup();
    let [source] = leaves.as_slice() else { return None };
    let source = *source;
    let width = size(data, source)?;
    if !matches!(width, 2 | 4 | 8) {
        return None;
    }
    let cond = op(data, data.bb_op_tail(head)?)?.get_in(1)?;
    let mut exits: Vec<Vec<u64>> = vec![Vec::new(); phis.len()];
    let samples = scan_samples(width);
    for &x in &samples {
        let mut state: Vec<u64> = phis
            .iter()
            .map(|p| if p.1 == source { Some(x) } else { crate::kuna_strlenscan::copied_constant(data, p.1) })
            .collect::<Option<Vec<_>>>()?;
        let mut done = false;
        for _ in 0..=8 * width + 2 {
            let env: Vec<(VarnodeId, u64)> = phis.iter().map(|p| p.0).zip(state.iter().copied()).chain([(source, x)]).collect();
            let c = eval(data, cond, &env, &region, 0)?;
            let to = if c != 0 { t } else { f };
            if to == exit {
                done = true;
                break;
            }
            state = phis.iter().map(|p| eval(data, p.2, &env, &region, 0)).collect::<Option<Vec<_>>>()?;
        }
        if !done {
            return None;
        }
        for (i, v) in state.into_iter().enumerate() {
            exits[i].push(v);
        }
    }
    let lowest: Vec<u64> = samples.iter().map(|x| x.trailing_zeros() as u64).collect();
    let highest: Vec<u64> = samples.iter().map(|x| 63 - x.leading_zeros() as u64).collect();
    let (counter, kind) = phis.iter().zip(&exits).find_map(|(p, e)| {
        if *e == lowest {
            Some((p.0, ScanKind::Trailing))
        } else if *e == highest {
            Some((p.0, ScanKind::Highest))
        } else {
            None
        }
    })?;
    // Nothing but the counter may leave the loop.
    for &o in &ops {
        let Some(out) = op(data, o)?.get_out() else { continue };
        let used_outside = data.descend_snapshot(out).into_iter().any(|d| op(data, d).and_then(|p| p.get_parent()).is_some_and(|b| !region.contains(&b)));
        if used_outside && out != counter {
            return None;
        }
    }
    Some(BitScan { head, to_body: data.bblocks_ref().block(head).get_out_index(body), source, counter, kind })
}

fn rewrite_bitscan(data: &mut Funcdata, s: &BitScan) -> kuna_base::error::KunaResult<()> {
    use kuna_base::error::KunaError;
    let head_ops = data.bb_ops(s.head);
    let first = head_ops
        .iter()
        .copied()
        .find(|&o| data.obank().get(o).is_some_and(|p| p.code() != OpCode::CPUI_MULTIEQUAL))
        .ok_or_else(|| KunaError::lowlevel("bitscan: empty head"))?;
    let at = data.obank().get(first).map(|p| p.get_addr().clone()).ok_or_else(|| KunaError::lowlevel("bitscan: stale op"))?;
    let csize = data.vbank().get(s.counter).map_or(4, |v| v.get_size());
    let width = data.vbank().get(s.source).map_or(4, |v| v.get_size());
    let result = match s.kind {
        ScanKind::Trailing => {
            let call = data.new_op(2, at);
            data.op_set_opcode_code(call, OpCode::CPUI_CALLOTHER);
            let id = data.new_constant(4, BUILTIN_CTZ as uintb);
            data.op_set_input(call, id, 0)?;
            data.op_set_input(call, s.source, 1)?;
            let res = data.new_unique_out(csize, call)?;
            data.op_insert_before(call, first);
            res
        }
        ScanKind::Highest => {
            let lz = data.new_op(1, at.clone());
            data.op_set_opcode_code(lz, OpCode::CPUI_LZCOUNT);
            data.op_set_input(lz, s.source, 0)?;
            let lzv = data.new_unique_out(csize, lz)?;
            data.op_insert_before(lz, first);
            let x = data.new_op(2, at);
            data.op_set_opcode_code(x, OpCode::CPUI_INT_XOR);
            data.op_set_input(x, lzv, 0)?;
            let top = data.new_constant(csize, (8 * width - 1) as uintb);
            data.op_set_input(x, top, 1)?;
            let res = data.new_unique_out(csize, x)?;
            data.op_insert_after(x, lz);
            res
        }
    };
    let region = [s.head, data.bblocks_ref().block(s.head).get_out(s.to_body)];
    for d in data.descend_snapshot(s.counter) {
        let outside = data.obank().get(d).and_then(|p| p.get_parent()).is_some_and(|b| !region.contains(&b));
        if !outside {
            continue;
        }
        let slots: Vec<i32> = (0..data.obank().get(d).map_or(0, |p| p.num_input())).filter(|&i| data.obank().get(d).and_then(|p| p.get_in(i)) == Some(s.counter)).collect();
        for i in slots {
            data.op_set_input(d, result, i)?;
        }
    }
    data.remove_branch(s.head, s.to_body)?;
    data.remove_unreachable_blocks(false, true)?;
    Ok(())
}

/// (kuna `bitidioms`) `ActionBitScanLoop` — the loop SLEIGH lifts x86
/// `bsf`/`tzcnt`/`bsr` to becomes `ctz(x)` / `LZCOUNT(x) ^ (W - 1)`.
pub struct ActionBitScanLoop {
    base: crate::action::ActionBase,
}

impl ActionBitScanLoop {
    pub fn new(g: impl Into<String>) -> Self {
        Self { base: crate::action::ActionBase::new(0, "bitscanloop", g) }
    }
}

impl crate::action::Action for ActionBitScanLoop {
    fn base(&self) -> &crate::action::ActionBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut crate::action::ActionBase {
        &mut self.base
    }
    fn clone_filtered(&self, grouplist: &ActionGroupList) -> Option<Box<dyn crate::action::Action>> {
        grouplist
            .contains(self.get_group())
            .then(|| Box::new(Self { base: self.base.clone() }) as Box<dyn crate::action::Action>)
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut crate::action::ActionContext) -> crate::action::ApplyResult {
        if !data.get_arch().bit_idioms {
            return 0;
        }
        let heads: Vec<BlockId> = (0..data.bblocks_get_size()).map(|i| data.bblocks_get_block(i)).collect();
        for h in heads {
            let Some(scan) = match_bitscan(data, h) else { continue };
            if rewrite_bitscan(data, &scan).is_ok() {
                self.base.count += 1;
                return 1;
            }
            return 0;
        }
        0
    }
}
