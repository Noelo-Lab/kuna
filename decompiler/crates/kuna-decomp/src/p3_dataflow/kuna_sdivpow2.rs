//! Signed division by a power of two whose negative-operand correction is a
//! conditional add.  Compilers round `x / 2^n` toward zero by adding `2^n - 1`
//! to a negative `x` before the arithmetic shift; when that add is a conditional
//! move or a short branch (`lea 7(%rax),%rdx; test %rax,%rax; cmovs %rdx,%rax;
//! sar $3,%rax`, ARM `cmp r3,#0; blt L; ... L: add r3,#7`) the shift reads a
//! MULTIEQUAL of `x` and `x + (2^n - 1)` at the join of a branch on the sign of
//! `x`.  [`recover`] rewrites the reads of that join that are exactly a function
//! of `x / 2^n` to read an `INT_SDIV(x, 2^n)` instead; once none is left, the
//! add, the join and the branch fall to dead-code and empty-block removal.

use kuna_base::address::calc_mask;
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::context::{BlockId, OpId, VarnodeId};
use crate::funcdata::Funcdata;

fn def_op(data: &Funcdata, vn: VarnodeId, opc: OpCode) -> Option<OpId> {
    let op = data.vbank().get(vn)?.get_def()?;
    (data.obank().get(op)?.code() == opc).then_some(op)
}

fn input(data: &Funcdata, op: OpId, slot: int4) -> Option<VarnodeId> {
    data.obank().get(op)?.get_in(slot)
}

fn constant(data: &Funcdata, vn: VarnodeId) -> Option<u64> {
    let v = data.vbank().get(vn)?;
    v.is_constant().then(|| v.get_offset())
}

fn size(data: &Funcdata, vn: VarnodeId) -> int4 {
    data.vbank().get(vn).map_or(0, |v| v.get_size())
}

/// `vn` as `root + k`, through one INT_ADD of a constant.
fn offset(data: &Funcdata, vn: VarnodeId) -> (VarnodeId, u64) {
    if let Some(add) = def_op(data, vn, OpCode::CPUI_INT_ADD) {
        if let (Some(a), Some(b)) = (input(data, add, 0), input(data, add, 1)) {
            if let Some(k) = constant(data, b) {
                return (a, k);
            }
            if let Some(k) = constant(data, a) {
                return (b, k);
            }
        }
    }
    (vn, 0)
}

/// Whether `a` and `b` are the same value: one Varnode, or the same constant
/// added to one root.
fn same(data: &Funcdata, a: VarnodeId, b: VarnodeId) -> bool {
    a == b || (offset(data, a) == offset(data, b) && offset(data, a).1 != 0)
}

/// Whether the boolean `cond` is true exactly when `x < 0` (`Some(true)`) or
/// exactly when `x >= 0` (`Some(false)`).
fn sign_test(data: &Funcdata, cond: VarnodeId, x: VarnodeId, mask: u64) -> Option<bool> {
    let op = data.vbank().get(cond)?.get_def()?;
    let o = data.obank().get(op)?;
    let a = o.get_in(0)?;
    if o.code() == OpCode::CPUI_BOOL_NEGATE {
        return sign_test(data, a, x, mask).map(|negative| !negative);
    }
    let b = o.get_in(1)?;
    let (ka, kb) = (constant(data, a), constant(data, b));
    match o.code() {
        OpCode::CPUI_INT_SLESS if kb == Some(0) && same(data, a, x) => Some(true),
        OpCode::CPUI_INT_SLESS if ka == Some(mask) && same(data, b, x) => Some(false),
        OpCode::CPUI_INT_SLESSEQUAL if kb == Some(mask) && same(data, a, x) => Some(true),
        OpCode::CPUI_INT_SLESSEQUAL if ka == Some(0) && same(data, b, x) => Some(false),
        _ => None,
    }
}

/// The block ending in the branch that decides the `slot` input of a join in
/// `join`, and which of its out edges leads there: the join's predecessor
/// itself, or the predecessor of a block on that edge alone.
fn deciding_edge(data: &Funcdata, join: BlockId, slot: int4) -> Option<(BlockId, int4)> {
    let graph = data.bblocks_ref();
    let j = graph.block(join);
    let pred = j.get_in(slot);
    let p = graph.block(pred);
    if p.size_out() == 2 {
        return Some((pred, j.get_in_rev_index(slot)));
    }
    if p.size_out() != 1 || p.size_in() != 1 {
        return None;
    }
    let root = p.get_in(0);
    (graph.block(root).size_out() == 2).then(|| (root, p.get_in_rev_index(0)))
}

/// The dividend `x` and the power `n` when `m` is `x < 0 ? x + (2^n - 1) : x`:
/// a two-input MULTIEQUAL whose inputs are `x` and `x + (2^n - 1)`, each decided
/// by one edge of the same branch, the add's edge being taken exactly when
/// `x < 0`, and `x` defined where it reaches the join's reads (not in its own
/// arm).  `n` stops at `8 * size - 2`, the largest power whose divisor is
/// positive at the operand's size.
fn corrected(data: &Funcdata, m: VarnodeId) -> Option<(VarnodeId, u32)> {
    let phi = def_op(data, m, OpCode::CPUI_MULTIEQUAL)?;
    let p = data.obank().get(phi)?;
    if p.num_input() != 2 {
        return None;
    }
    let join = p.get_parent()?;
    let (in0, in1) = (p.get_in(0)?, p.get_in(1)?);
    let sz = size(data, m);
    if !(1..=8).contains(&sz) {
        return None;
    }
    let mask = calc_mask(sz);
    let ((r0, k0), (r1, k1)) = (offset(data, in0), offset(data, in1));
    if r0 != r1 {
        return None;
    }
    let power = |c: u64| (c != 0 && c & (c + 1) == 0).then(|| c.trailing_ones()).filter(|&n| n <= 8 * sz as u32 - 2);
    let (x, add_slot, n) = if let Some(n) = power(k1.wrapping_sub(k0) & mask) {
        (in0, 1, n)
    } else {
        let n = power(k0.wrapping_sub(k1) & mask)?;
        (in1, 0, n)
    };
    if data.vbank().get(x).is_none_or(|v| v.is_free()) {
        return None;
    }
    let (branch, add_edge) = deciding_edge(data, join, add_slot)?;
    let (branch_x, x_edge) = deciding_edge(data, join, 1 - add_slot)?;
    if branch != branch_x || add_edge == x_edge {
        return None;
    }
    let x_arm = data.bblocks_ref().block(join).get_in(1 - add_slot);
    let x_def = data.vbank().get(x)?.get_def().and_then(|d| data.obank().get(d)?.get_parent());
    if x_arm != branch && x_def == Some(x_arm) {
        return None;
    }
    let cb = data.obank().get(data.bb_op_tail(branch)?)?;
    if cb.code() != OpCode::CPUI_CBRANCH {
        return None;
    }
    let negative_when_true = sign_test(data, cb.get_in(1)?, x, mask)?;
    let negative_edge = if negative_when_true != cb.is_boolean_flip() { 1 } else { 0 };
    (add_edge == negative_edge).then_some((x, n))
}

/// Whether every read of `vn` truncates it to at most `bits` low bits.
fn read_below(data: &Funcdata, vn: VarnodeId, bits: int4) -> bool {
    let Some(v) = data.vbank().get(vn) else { return false };
    if v.is_addr_tied() || v.is_persist() {
        return false;
    }
    let mut reads = v.descend_iter().peekable();
    reads.peek().is_some()
        && reads.all(|r| {
            data.obank().get(r).is_some_and(|o| {
                o.code() == OpCode::CPUI_SUBPIECE
                    && o.get_in(1).and_then(|k| constant(data, k)) == Some(0)
                    && o.get_out().is_some_and(|out| 8 * size(data, out) <= bits)
            })
        })
}

/// A new `INT_SDIV(x, 2^n)` just before `op`.
fn quotient_before(data: &mut Funcdata, op: OpId, x: VarnodeId, n: u32) -> Option<VarnodeId> {
    let sz = size(data, x);
    let addr = data.obank().get(op)?.get_addr().clone();
    let div = data.new_op(2, addr);
    data.op_set_opcode(div, crate::typeop::seam_type_op_for(OpCode::CPUI_INT_SDIV));
    let q = data.new_unique_out(sz, div).ok()?;
    data.op_set_input(div, x, 0).ok()?;
    let d = data.new_constant(sz, 1u64 << n);
    data.op_set_input(div, d, 1).ok()?;
    data.op_insert_before(div, op);
    Some(q)
}

/// Rewrite a read of `m = x < 0 ? x + (2^n - 1) : x` that is exactly a function
/// of `x / 2^n`: `m s>> n` becomes `x / 2^n`, as does `m >> n` whose reads keep
/// only bits the two shifts agree on; byte `n / 8` of `m` becomes the low bytes
/// of `x / 2^n`, and `m & -2^n` becomes `(x / 2^n) * 2^n`.
pub fn recover(op: OpId, data: &mut Funcdata) -> bool {
    let Some(o) = data.obank().get(op) else { return false };
    let opc = o.code();
    if !matches!(
        opc,
        OpCode::CPUI_INT_SRIGHT | OpCode::CPUI_INT_RIGHT | OpCode::CPUI_SUBPIECE | OpCode::CPUI_INT_AND
    ) {
        return false;
    }
    let (Some(m), Some(c), Some(out)) = (o.get_in(0), o.get_in(1), o.get_out()) else { return false };
    let Some(c) = constant(data, c) else { return false };
    let Some((x, n)) = corrected(data, m) else { return false };
    let sz = size(data, m);
    let divisor = 1u64 << n;
    match opc {
        OpCode::CPUI_INT_SRIGHT | OpCode::CPUI_INT_RIGHT => {
            if c != n as u64 || (opc == OpCode::CPUI_INT_RIGHT && !read_below(data, out, 8 * sz - n as int4)) {
                return false;
            }
            data.op_set_input(op, x, 0).expect("sdivpow2: dividend");
            let d = data.new_constant(sz, divisor);
            data.op_set_input(op, d, 1).expect("sdivpow2: divisor");
            data.op_set_opcode(op, crate::typeop::seam_type_op_for(OpCode::CPUI_INT_SDIV));
        }
        OpCode::CPUI_SUBPIECE => {
            if 8 * c != n as u64 || size(data, out) as u64 + c > sz as u64 {
                return false;
            }
            let Some(q) = quotient_before(data, op, x, n) else { return false };
            data.op_set_input(op, q, 0).expect("sdivpow2: truncated quotient");
            let zero = data.new_constant(4, 0);
            data.op_set_input(op, zero, 1).expect("sdivpow2: low bytes");
        }
        _ => {
            if c != calc_mask(sz) & !(divisor - 1) {
                return false;
            }
            let Some(q) = quotient_before(data, op, x, n) else { return false };
            data.op_set_input(op, q, 0).expect("sdivpow2: rounded quotient");
            let d = data.new_constant(sz, divisor);
            data.op_set_input(op, d, 1).expect("sdivpow2: scale");
            data.op_set_opcode(op, crate::typeop::seam_type_op_for(OpCode::CPUI_INT_MULT));
        }
    }
    true
}

#[cfg(test)]
#[path = "kuna_sdivpow2/tests.rs"]
mod tests;
