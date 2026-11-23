//! (kuna `loweredswitch`) Install a re-rolled switch along the raw paths of the
//! compare tree.
//!
//! gcc -O2 often loads each case's value before the compare that selects it:
//! `mov eax,K1 ; cmp edi,c1 ; je R ; mov eax,K2 ; cmp edi,c2 ; je R ; ...`, with
//! one shared return `R`. The simplified graph shows one return block per case
//! (the shared epilogue duplicated with each constant), but on the raw p-code the
//! install edits every case reaches the same block `R`, and what tells the cases
//! apart is an assignment inside the compare tree. Branching from the head
//! straight to `R` would lose those assignments.
//!
//! [`plan`] evaluates the raw compare tree, starting at the head with the switch
//! value known and everything else unknown, for every value that can change an
//! outcome (each constant of the tree and its neighbours, and the wrap points of
//! the value's width). Each value follows the raw branches through the compare
//! blocks the detection recorded until it leaves them; values that run the same
//! compare blocks to the same block form one arm. Every value without a label
//! must take one path, the default's. An arm that runs compare blocks whose
//! writes are read again gets a new block holding copies of their operations,
//! in order, ahead of the block it leaves to, so each value still runs exactly
//! the operations it ran in the tree. A condition that cannot be decided, a
//! block on a path that is entered twice, or a second default path declines
//! the install.

use std::collections::{BTreeMap, BTreeSet};

use kuna_base::address::{calc_int_min, calc_mask, sign_extend_sized, Address};
use kuna_base::error::KunaResult;
use kuna_base::types::{int4, uintb};
use kuna_num::opcodes::OpCode;

use crate::context::{BlockId, OpId, VarnodeId};
use crate::funcdata::Funcdata;
use crate::p2_lift::kuna_loweredswitchexact::read_before_written;
use crate::p2_lift::kuna_loweredswitchvalue::{head_compare_operand, HeadOperand};

/// What the head compare reads: a storage location that holds the switch value
/// when the head block starts, or the output of a load.
pub(crate) enum Selector {
    Storage(Address, int4),
    Value(VarnodeId),
}

/// One arm of the planned switch: the raw compare blocks it runs after the head,
/// the block it leaves to, its labels (none for the default), and the operations
/// of `via` whose results are used again, which its own block repeats.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct PathArm {
    pub via: Vec<BlockId>,
    pub target: BlockId,
    pub labels: Vec<uintb>,
    pub ops: Vec<OpId>,
}

/// The arms (cases first, default last) of the switch the raw tree at `head`
/// dispatches, or `None` when the tree cannot be followed for every value.
pub(crate) fn plan(
    data: &Funcdata,
    head: BlockId,
    spine: &[Address],
    selector: &Selector,
    size: int4,
    case_vals: &[uintb],
    returns: &[(Address, int4)],
) -> Option<Vec<PathArm>> {
    if !(1..=8).contains(&size) {
        return None;
    }
    let mask = calc_mask(size);
    let blocks: BTreeSet<BlockId> = (0..data.bblocks_get_size())
        .map(|i| data.bblocks_get_block(i))
        .filter(|&b| b == head || tail_cbranch_at(data, b).is_some_and(|a| spine.contains(&a)))
        .collect();
    let mut points: BTreeSet<uintb> = BTreeSet::new();
    let int_min = calc_int_min(size);
    for p in [0, mask, int_min, int_min.wrapping_sub(1)] {
        points.insert(p & mask);
    }
    for &b in &blocks {
        for op in data.bb_ops(b) {
            let o = data.obank().get(op)?;
            for i in 0..o.num_input() {
                if let Some(v) = o.get_in(i).and_then(|v| data.vbank().get(v)).filter(|v| v.is_constant()) {
                    let c = v.get_offset();
                    for p in [c.wrapping_sub(1), c, c.wrapping_add(1)] {
                        points.insert(p & mask);
                    }
                }
            }
        }
    }
    let labels: BTreeSet<uintb> = case_vals.iter().map(|v| v & mask).collect();
    points.extend(labels.iter().copied());

    let mut default: Option<(Vec<BlockId>, BlockId)> = None;
    let mut arms: Vec<PathArm> = Vec::new();
    for &v in &points {
        let (via, target) = follow(data, head, &blocks, selector, size, v)?;
        if !labels.contains(&v) {
            match &default {
                None => default = Some((via, target)),
                Some(d) if *d == (via.clone(), target) => {}
                Some(_) => return None,
            }
            continue;
        }
        match arms.iter_mut().find(|a| a.via == via && a.target == target) {
            Some(a) => a.labels.push(v),
            None => arms.push(PathArm { via, target, labels: vec![v], ops: Vec::new() }),
        }
    }
    let (dvia, dtarget) = default?;
    arms.retain(|a| !(a.via == dvia && a.target == dtarget));
    arms.push(PathArm { via: dvia, target: dtarget, labels: Vec::new(), ops: Vec::new() });
    for arm in arms.iter_mut() {
        arm.ops = used_ops(data, &blocks, arm, returns);
    }
    let default = arms.pop()?;
    let mut merged: Vec<PathArm> = Vec::new();
    for arm in arms {
        if arm.ops.is_empty() && default.ops.is_empty() && arm.target == default.target {
            continue;
        }
        match merged.iter_mut().find(|m| m.ops.is_empty() && arm.ops.is_empty() && m.target == arm.target) {
            Some(m) => m.labels.extend(arm.labels),
            None => merged.push(arm),
        }
    }
    merged.push(default);
    Some(merged)
}

/// The storage the returns of the (simplified) function read.
pub(crate) fn returned_storage(data: &Funcdata) -> Vec<(Address, int4)> {
    let mut out: Vec<(Address, int4)> = Vec::new();
    for i in 0..data.bblocks_get_size() {
        let b = data.bblocks_get_block(i);
        let Some(op) = data.bb_op_tail(b).and_then(|op| data.obank().get(op)) else { continue };
        if op.code() != OpCode::CPUI_RETURN {
            continue;
        }
        for k in 1..op.num_input() {
            if let Some(v) = op.get_in(k).and_then(|v| data.vbank().get(v)).filter(|v| !v.is_constant()) {
                let loc = (v.get_addr().clone(), v.get_size());
                if !out.contains(&loc) {
                    out.push(loc);
                }
            }
        }
    }
    out
}

/// The arms of the switch to install at `head` along the raw paths, reading
/// what the install will read: the head compare's own operand when it is
/// preferred and found (it must be the load at `load_at` for a selector the
/// detection graph held in a temporary), the recorded storage otherwise.
#[allow(clippy::too_many_arguments)]
pub(crate) fn plan_install(
    data: &Funcdata,
    head: BlockId,
    var_addr: &Address,
    var_size: int4,
    case_vals: &[uintb],
    prefer_head_operand: bool,
    load_at: Option<&Address>,
    spine: &[Address],
    returns: &[(Address, int4)],
) -> Option<Vec<PathArm>> {
    let cbranch = data.bb_op_tail(head)?;
    if data.obank().get(cbranch)?.code() != OpCode::CPUI_CBRANCH {
        return None;
    }
    let operand = if prefer_head_operand {
        head_compare_operand(data, head, cbranch, var_addr, var_size).filter(|op| *op != HeadOperand::Storage(var_addr.clone()))
    } else {
        None
    };
    if let Some(at) = load_at {
        let loaded = match &operand {
            Some(HeadOperand::Value(v)) => data.vbank().get(*v).and_then(|v| v.get_def()).and_then(|d| data.obank().get(d)),
            _ => None,
        };
        if !loaded.is_some_and(|op| op.code() == OpCode::CPUI_LOAD && op.get_addr() == at) {
            return None;
        }
    }
    let selector = match operand {
        Some(HeadOperand::Value(v)) => Selector::Value(v),
        Some(HeadOperand::Storage(a)) => Selector::Storage(a, var_size),
        None => Selector::Storage(var_addr.clone(), var_size),
    };
    plan(data, head, spine, &selector, var_size, case_vals, returns)
}

/// What a block of the simplified graph does, for telling apart copies of one
/// block: each op's code, its output storage and its inputs (a constant by value,
/// anything else by storage).
pub(crate) fn body_signature(data: &Funcdata, b: BlockId) -> Vec<(OpCode, Vec<(bool, int4, uintb, int4)>)> {
    let vn = |v: VarnodeId| {
        data.vbank().get(v).map(|x| {
            let space = x.get_addr().get_space().map(|s| s.get_index()).unwrap_or(-1);
            (x.is_constant(), space, x.get_offset(), x.get_size())
        })
    };
    data.bb_ops(b)
        .into_iter()
        .filter_map(|op| data.obank().get(op))
        .map(|o| {
            let mut sig: Vec<(bool, int4, uintb, int4)> = o.get_out().and_then(vn).into_iter().collect();
            sig.extend((0..o.num_input()).filter_map(|i| o.get_in(i)).filter_map(vn));
            (o.code(), sig)
        })
        .collect()
}

/// Build the new block for each arm that runs compare blocks: copies of their
/// operations other than branches, in order, with one edge to the arm's target.
/// Returns the block each arm leaves the switch head for.
pub(crate) fn build_arm_blocks(data: &mut Funcdata, arms: &[PathArm]) -> KunaResult<Vec<BlockId>> {
    let mut out = Vec::with_capacity(arms.len());
    for arm in arms {
        if arm.ops.is_empty() {
            out.push(arm.target);
            continue;
        }
        let ops = arm.ops.clone();
        let at = match ops.first().and_then(|&op| data.obank().get(op)) {
            Some(o) => o.get_addr().clone(),
            None => crate::block::block_get_start(&data.bblocks_ref().arena, arm.via[0]),
        };
        let graph = data.bblocks_root_pub();
        let nb = data.bblocks_mut().new_block_basic(graph);
        data.set_basic_block_range(nb, &at, &at);
        for op in ops {
            let copy = clone_raw_op(data, op)?;
            data.op_insert_end(copy, nb);
        }
        data.bblocks_mut().add_edge(nb, arm.target);
        out.push(nb);
    }
    Ok(out)
}

/// A copy of raw op `op` with fresh, unlinked varnodes for heritage to connect.
fn clone_raw_op(data: &mut Funcdata, op: OpId) -> KunaResult<OpId> {
    let (code, addr, ins, out) = {
        let o = data.obank().get(op).expect("clone_raw_op: stale op");
        let ins: Vec<(bool, int4, Address, uintb)> = (0..o.num_input())
            .filter_map(|i| o.get_in(i).and_then(|v| data.vbank().get(v)))
            .map(|v| (v.is_constant(), v.get_size(), v.get_addr().clone(), v.get_offset()))
            .collect();
        let out = o.get_out().and_then(|v| data.vbank().get(v)).map(|v| (v.get_size(), v.get_addr().clone()));
        (o.code(), o.get_addr().clone(), ins, out)
    };
    let copy = data.new_op(ins.len() as int4, addr);
    data.op_set_opcode_code(copy, code);
    for (slot, (constant, size, a, off)) in ins.into_iter().enumerate() {
        let vn = if constant { data.new_constant(size, off) } else { data.new_varnode(size, &a, None) };
        data.op_set_input(copy, vn, slot as int4)?;
    }
    if let Some((size, a)) = out {
        data.new_varnode_out(size, &a, copy)?;
    }
    Ok(copy)
}

/// The operations of an arm's compare blocks whose results are used: read by a
/// later one of them that is used, or, past the arm, read before they are
/// written again (a return reads only `returns`).
fn used_ops(data: &Funcdata, spine: &BTreeSet<BlockId>, arm: &PathArm, returns: &[(Address, int4)]) -> Vec<OpId> {
    let ops: Vec<OpId> = arm
        .via
        .iter()
        .flat_map(|&b| data.bb_ops(b))
        .filter(|&op| data.obank().get(op).is_some_and(|o| !is_branch_code(o.code())))
        .collect();
    let loc = |vn: VarnodeId| data.vbank().get(vn).filter(|v| !v.is_constant()).map(|v| (v.get_addr().clone(), v.get_size()));
    let overlaps = |a: &(Address, int4), b: &(Address, int4)| {
        a.0.get_space().map(|s| s.get_index()) == b.0.get_space().map(|s| s.get_index())
            && a.0.get_offset() < b.0.get_offset().wrapping_add(b.1 as uintb)
            && b.0.get_offset() < a.0.get_offset().wrapping_add(a.1 as uintb)
    };
    let covers = |a: &(Address, int4), b: &(Address, int4)| {
        a.0.get_space().map(|s| s.get_index()) == b.0.get_space().map(|s| s.get_index())
            && a.0.get_offset() <= b.0.get_offset()
            && b.0.get_offset().wrapping_add(b.1 as uintb) <= a.0.get_offset().wrapping_add(a.1 as uintb)
    };
    let mut keep = vec![false; ops.len()];
    for i in (0..ops.len()).rev() {
        let Some(out) = data.obank().get(ops[i]).and_then(|o| o.get_out()).and_then(loc) else { continue };
        let mut used = None;
        for j in i + 1..ops.len() {
            let Some(o) = data.obank().get(ops[j]) else { continue };
            if keep[j] && (0..o.num_input()).filter_map(|k| o.get_in(k)).filter_map(loc).any(|l| overlaps(&l, &out)) {
                used = Some(true);
                break;
            }
            if o.get_out().and_then(loc).is_some_and(|w| covers(&w, &out)) {
                used = Some(false);
                break;
            }
        }
        keep[i] = used.unwrap_or_else(|| read_before_written(data, spine, &[arm.target], &out.0, out.1, Some(returns)));
    }
    ops.into_iter().zip(keep).filter(|(_, k)| *k).map(|(op, _)| op).collect()
}

fn is_branch_code(code: OpCode) -> bool {
    matches!(code, OpCode::CPUI_BRANCH | OpCode::CPUI_CBRANCH | OpCode::CPUI_BRANCHIND)
}

/// The address of `b`'s trailing `CBRANCH`.
fn tail_cbranch_at(data: &Funcdata, b: BlockId) -> Option<Address> {
    let op = data.obank().get(data.bb_op_tail(b)?)?;
    (op.code() == OpCode::CPUI_CBRANCH && data.bblocks_ref().block(b).size_out() == 2).then(|| op.get_addr().clone())
}

/// The compare blocks after `head` that switch value `v` runs, and the block it
/// leaves them for.
fn follow(
    data: &Funcdata,
    head: BlockId,
    blocks: &BTreeSet<BlockId>,
    selector: &Selector,
    size: int4,
    v: uintb,
) -> Option<(Vec<BlockId>, BlockId)> {
    let mut st = State::default();
    let load = match selector {
        Selector::Storage(a, s) => {
            st.write(data, a, *s, Some(v & calc_mask(*s)));
            None
        }
        Selector::Value(vn) => {
            let x = data.vbank().get(*vn)?;
            st.write(data, &x.get_addr().clone(), x.get_size(), Some(v & calc_mask(size)));
            Some(*vn)
        }
    };
    let mut cur = head;
    let mut via: Vec<BlockId> = Vec::new();
    for _ in 0..=blocks.len() {
        let mut taken: Option<bool> = None;
        for op in data.bb_ops(cur) {
            let o = data.obank().get(op)?;
            if o.code() == OpCode::CPUI_CBRANCH {
                let c = st.read(data, o.get_in(1)?)?;
                taken = Some((c != 0) != o.is_boolean_flip());
                continue;
            }
            if is_branch_code(o.code())
                || matches!(
                    o.code(),
                    OpCode::CPUI_CALL | OpCode::CPUI_CALLIND | OpCode::CPUI_RETURN | OpCode::CPUI_STORE | OpCode::CPUI_CALLOTHER
                )
            {
                return None;
            }
            let Some(out) = o.get_out() else { continue };
            let val = if load == Some(out) { Some(v & calc_mask(size)) } else { eval(data, &st, op) };
            let x = data.vbank().get(out)?;
            st.write(data, &x.get_addr().clone(), x.get_size(), val);
        }
        let blk = data.bblocks_ref().block(cur);
        let next = if taken? { blk.get_true_out() } else { blk.get_false_out() };
        if next != head && blocks.contains(&next) && !via.contains(&next) {
            via.push(next);
            cur = next;
            continue;
        }
        if next == head || via.contains(&next) {
            return None;
        }
        return Some((via, next));
    }
    None
}

/// Known bytes of storage, by space index and byte offset.
#[derive(Default)]
struct State {
    bytes: BTreeMap<(int4, uintb), u8>,
}

impl State {
    fn write(&mut self, data: &Funcdata, a: &Address, size: int4, val: Option<uintb>) {
        let Some(spc) = a.get_space() else { return };
        let (idx, big) = (spc.get_index(), spc.is_big_endian());
        let _ = data;
        for i in 0..size as uintb {
            let key = (idx, a.get_offset().wrapping_add(i));
            match val {
                Some(v) => {
                    let shift = if big { (size as uintb - 1 - i) * 8 } else { i * 8 };
                    self.bytes.insert(key, ((v >> shift) & 0xff) as u8);
                }
                None => {
                    self.bytes.remove(&key);
                }
            }
        }
    }

    fn read(&self, data: &Funcdata, vn: VarnodeId) -> Option<uintb> {
        let v = data.vbank().get(vn)?;
        let size = v.get_size();
        if v.is_constant() {
            return Some(v.get_offset() & calc_mask(size));
        }
        if !(1..=8).contains(&size) {
            return None;
        }
        let spc = v.get_addr().get_space()?;
        let (idx, big) = (spc.get_index(), spc.is_big_endian());
        let mut out: uintb = 0;
        for i in 0..size as uintb {
            let b = *self.bytes.get(&(idx, v.get_offset().wrapping_add(i)))? as uintb;
            let shift = if big { (size as uintb - 1 - i) * 8 } else { i * 8 };
            out |= b << shift;
        }
        Some(out)
    }
}

/// The value op `op` computes from what `st` knows, when it is decided.
fn eval(data: &Funcdata, st: &State, op: OpId) -> Option<uintb> {
    let o = data.obank().get(op)?;
    let outsize = data.vbank().get(o.get_out()?)?.get_size();
    if !(1..=8).contains(&outsize) {
        return None;
    }
    let omask = calc_mask(outsize);
    let ins: Vec<VarnodeId> = (0..o.num_input()).filter_map(|i| o.get_in(i)).collect();
    let insize = |i: usize| ins.get(i).and_then(|&v| data.vbank().get(v)).map(|v| v.get_size()).unwrap_or(0);
    let get = |i: usize| ins.get(i).and_then(|&v| st.read(data, v));
    let a = get(0);
    let b = get(1);
    let sz = insize(0);
    if !(1..=8).contains(&sz) {
        return None;
    }
    let m = calc_mask(sz);
    let sx = |x: uintb| sign_extend_sized(x & m, sz, 8) as i64;
    let flag = |c: bool| Some(c as uintb);
    let r = match o.code() {
        OpCode::CPUI_COPY => a,
        OpCode::CPUI_INT_ZEXT => a,
        OpCode::CPUI_INT_SEXT => a.map(|x| sign_extend_sized(x, sz, outsize)),
        OpCode::CPUI_SUBPIECE => match (a, b) {
            (Some(x), Some(k)) if k < 8 => Some(x >> (k * 8)),
            (_, Some(k)) if k >= 8 => Some(0),
            _ => None,
        },
        OpCode::CPUI_PIECE => match (a, b) {
            (Some(h), Some(l)) => Some((h << (insize(1) * 8)) | l),
            _ => None,
        },
        OpCode::CPUI_INT_ADD => Some(a?.wrapping_add(b?)),
        OpCode::CPUI_INT_SUB => Some(a?.wrapping_sub(b?)),
        OpCode::CPUI_INT_MULT => match (a, b) {
            (Some(0), _) | (_, Some(0)) => Some(0),
            (Some(x), Some(y)) => Some(x.wrapping_mul(y)),
            _ => None,
        },
        OpCode::CPUI_INT_AND => match (a, b) {
            (Some(0), _) | (_, Some(0)) => Some(0),
            (Some(x), Some(y)) => Some(x & y),
            _ => None,
        },
        OpCode::CPUI_INT_OR => Some(a? | b?),
        OpCode::CPUI_INT_XOR => Some(a? ^ b?),
        OpCode::CPUI_INT_NEGATE => Some(!a?),
        OpCode::CPUI_INT_2COMP => Some(a?.wrapping_neg()),
        OpCode::CPUI_INT_LEFT => {
            let s = b?;
            if s >= 64 {
                Some(0)
            } else {
                Some(a? << s)
            }
        }
        OpCode::CPUI_INT_RIGHT => {
            let s = b?;
            if s >= 64 {
                Some(0)
            } else {
                Some((a? & m) >> s)
            }
        }
        OpCode::CPUI_INT_SRIGHT => Some((sx(a?) >> b?.min(63)) as uintb),
        OpCode::CPUI_INT_EQUAL => flag((a? & m) == (b? & m)),
        OpCode::CPUI_INT_NOTEQUAL => flag((a? & m) != (b? & m)),
        OpCode::CPUI_INT_LESS => flag((a? & m) < (b? & m)),
        OpCode::CPUI_INT_LESSEQUAL => flag((a? & m) <= (b? & m)),
        OpCode::CPUI_INT_SLESS => flag(sx(a?) < sx(b?)),
        OpCode::CPUI_INT_SLESSEQUAL => flag(sx(a?) <= sx(b?)),
        OpCode::CPUI_INT_CARRY => {
            let (x, y) = (a? & m, b? & m);
            flag((x.wrapping_add(y) & m) < x)
        }
        OpCode::CPUI_INT_SCARRY => {
            let (x, y) = (sx(a?), sx(b?));
            let r = sx((x as uintb).wrapping_add(y as uintb));
            flag((x < 0) == (y < 0) && (r < 0) != (x < 0))
        }
        OpCode::CPUI_INT_SBORROW => {
            let (x, y) = (sx(a?), sx(b?));
            let r = sx((x as uintb).wrapping_sub(y as uintb));
            flag((x < 0) != (y < 0) && (r < 0) != (x < 0))
        }
        OpCode::CPUI_BOOL_NEGATE => flag(a? == 0),
        OpCode::CPUI_BOOL_AND => match (a, b) {
            (Some(0), _) | (_, Some(0)) => Some(0),
            (Some(_), Some(_)) => Some(1),
            _ => None,
        },
        OpCode::CPUI_BOOL_OR => match (a, b) {
            (Some(x), _) if x != 0 => Some(1),
            (_, Some(y)) if y != 0 => Some(1),
            (Some(_), Some(_)) => Some(0),
            _ => None,
        },
        OpCode::CPUI_BOOL_XOR => flag((a? != 0) != (b? != 0)),
        OpCode::CPUI_POPCOUNT => Some(((a? & m).count_ones()) as uintb),
        _ => None,
    };
    r.map(|x| x & omask)
}

#[cfg(test)]
#[path = "kuna_loweredswitchpath/tests.rs"]
mod tests;
