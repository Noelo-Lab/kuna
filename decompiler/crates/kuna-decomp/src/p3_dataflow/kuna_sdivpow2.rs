//! Signed division by a power of two whose negative-operand correction is a
//! conditional add.  Compilers round `x / 2^n` toward zero by adding `2^n - 1`
//! to a negative `x` before the arithmetic shift; when that add is a conditional
//! move or a short branch (`lea 7(%rax),%rdx; test %rax,%rax; cmovs %rdx,%rax;
//! sar $3,%rax`, ARM `cmp r3,#0; blt L; ... L: add r3,#7`) the shift reads a
//! MULTIEQUAL of `x` and `x + (2^n - 1)` at the join of a branch on the sign of
//! `x`.  [`recover`] rewrites that shift to `INT_SDIV(x, 2^n)`; the add, the join
//! and the branch then fall to dead-code and empty-block removal.

use kuna_base::address::calc_mask;
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::context::{BlockId, OpId, VarnodeId};
use crate::funcdata::Funcdata;

fn def_op(data: &Funcdata, vn: VarnodeId, opc: OpCode) -> Option<OpId> {
    let v = data.vbank().get(vn)?;
    let op = v.get_def()?;
    (data.obank().get(op)?.code() == opc).then_some(op)
}

fn input(data: &Funcdata, op: OpId, slot: int4) -> Option<VarnodeId> {
    data.obank().get(op)?.get_in(slot)
}

fn constant(data: &Funcdata, vn: VarnodeId) -> Option<u64> {
    let v = data.vbank().get(vn)?;
    v.is_constant().then(|| v.get_offset())
}

/// `true` when `t` is `x + c`.
fn adds(data: &Funcdata, t: VarnodeId, x: VarnodeId, c: u64) -> bool {
    let Some(add) = def_op(data, t, OpCode::CPUI_INT_ADD) else { return false };
    let (Some(a), Some(b)) = (input(data, add, 0), input(data, add, 1)) else { return false };
    (a == x && constant(data, b) == Some(c)) || (b == x && constant(data, a) == Some(c))
}

/// Whether the boolean `cond` is true exactly when `x < 0` (`Some(true)`) or
/// exactly when `x >= 0` (`Some(false)`).
fn sign_test(data: &Funcdata, cond: VarnodeId, x: VarnodeId, mask: u64) -> Option<bool> {
    let op = data.vbank().get(cond)?.get_def()?;
    let o = data.obank().get(op)?;
    let (a, b) = (o.get_in(0)?, o.get_in(1));
    match o.code() {
        OpCode::CPUI_BOOL_NEGATE => sign_test(data, a, x, mask).map(|neg| !neg),
        OpCode::CPUI_INT_SLESS if a == x && constant(data, b?) == Some(0) => Some(true),
        OpCode::CPUI_INT_SLESS if b? == x && constant(data, a) == Some(mask) => Some(false),
        OpCode::CPUI_INT_SLESSEQUAL if a == x && constant(data, b?) == Some(mask) => Some(true),
        OpCode::CPUI_INT_SLESSEQUAL if b? == x && constant(data, a) == Some(0) => Some(false),
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

/// Rewrite `op = MULTIEQUAL(x, x + (2^n - 1)) s>> n`, the add taken exactly when
/// `x < 0`, to `op = x s/ 2^n`.
pub fn recover(op: OpId, data: &mut Funcdata) -> bool {
    let Some(o) = data.obank().get(op) else { return false };
    let (Some(m), Some(sh), Some(out)) = (o.get_in(0), o.get_in(1), o.get_out()) else { return false };
    let Some(n) = constant(data, sh) else { return false };
    let size = data.vbank().get(out).map_or(0, |v| v.get_size());
    if !(1..=8).contains(&size) || n == 0 || n > (8 * size - 2) as u64 {
        return false;
    }
    let mask = calc_mask(size);
    let Some(phi) = def_op(data, m, OpCode::CPUI_MULTIEQUAL) else { return false };
    let p = data.obank().get(phi).expect("sdivpow2: stale join");
    if p.num_input() != 2 {
        return false;
    }
    let Some(join) = p.get_parent() else { return false };
    let (Some(in0), Some(in1)) = (p.get_in(0), p.get_in(1)) else { return false };
    let c = (1u64 << n) - 1;
    let (x, add_slot) = if adds(data, in1, in0, c) {
        (in0, 1)
    } else if adds(data, in0, in1, c) {
        (in1, 0)
    } else {
        return false;
    };
    if data.vbank().get(x).is_none_or(|v| v.is_free()) {
        return false;
    }
    let Some((branch, add_edge)) = deciding_edge(data, join, add_slot) else { return false };
    let Some((branch_x, x_edge)) = deciding_edge(data, join, 1 - add_slot) else { return false };
    if branch != branch_x || add_edge == x_edge {
        return false;
    }
    let Some(cbranch) = data.bb_op_tail(branch) else { return false };
    let cb = data.obank().get(cbranch).expect("sdivpow2: stale branch");
    if cb.code() != OpCode::CPUI_CBRANCH {
        return false;
    }
    let flip = cb.is_boolean_flip();
    let Some(cond) = cb.get_in(1) else { return false };
    let Some(true_when_negative) = sign_test(data, cond, x, mask) else { return false };
    let negative_edge = if true_when_negative != flip { 1 } else { 0 };
    if add_edge != negative_edge {
        return false;
    }
    data.op_set_input(op, x, 0).expect("sdivpow2: dividend");
    let divisor = data.new_constant(size, 1u64 << n);
    data.op_set_input(op, divisor, 1).expect("sdivpow2: divisor");
    data.op_set_opcode(op, crate::typeop::seam_type_op_for(OpCode::CPUI_INT_SDIV));
    true
}
