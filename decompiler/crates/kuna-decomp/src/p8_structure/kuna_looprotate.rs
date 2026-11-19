//! (kuna `looprotate`) A guarded do-while loop becomes a while loop.
//!
//! Compilers rotate `while (c) body` into `if (c) { do body while (c); }` so
//! the test runs at the bottom. Undoing it gives the loop back its head test,
//! and the `for` recovery then applies; kuna printed the guard and the do-while.
//!
//! [`ActionLoopRotate`] matches, on the basic blocks: a guard `G` branching to
//! the loop's entry side `P` (or straight to the header) and to an exit `E`; a
//! header `H` entered from `P` and from the tail `T`; `T` branching back to `H`
//! or out to the same `E`. The tail's condition `F` must read only loop
//! invariants and the values `T` hands to `H`'s MULTIEQUALs, through pure ops.
//! It rotates only when the guard is `F` on the values `P` hands to `H`
//! (operands compared op by op; a BOOL_NEGATE flips the sense), `P` has no
//! side effect, and each MULTIEQUAL of `E` reads, from `T`, a value it can
//! read at the new header instead (an `H` MULTIEQUAL's back-edge value or an
//! invariant) and, from `G`, the same value on entry. Then a new header `N`
//! takes `H`'s MULTIEQUALs and a copy of `F` over them, `P` and `T` jump to
//! `N`, `N` branches to `H` or `E`, and the guard's and the tail's own
//! branches go: `if (0 < n) { i = 0; do {...; i++} while (i < n); }` reads
//! `for (i = 0; i < n; i++) {...}`.

use std::collections::{BTreeMap, BTreeSet};

use kuna_base::types::{int4, uintb};
use kuna_num::opcodes::OpCode;

use crate::action::{Action, ActionBase, ActionContext, ActionGroupList, ApplyResult};
use crate::context::{BlockId, OpId, VarnodeId};
use crate::funcdata::Funcdata;
use crate::p3_dataflow::kuna_strlenscan::taken;

fn pure(code: OpCode) -> bool {
    use OpCode::*;
    matches!(
        code,
        CPUI_COPY | CPUI_INT_ADD | CPUI_INT_SUB | CPUI_INT_AND | CPUI_INT_OR | CPUI_INT_XOR | CPUI_INT_MULT
            | CPUI_INT_LEFT | CPUI_INT_RIGHT | CPUI_INT_SRIGHT | CPUI_INT_EQUAL | CPUI_INT_NOTEQUAL | CPUI_INT_LESS
            | CPUI_INT_LESSEQUAL | CPUI_INT_SLESS | CPUI_INT_SLESSEQUAL | CPUI_INT_ZEXT | CPUI_INT_SEXT
            | CPUI_SUBPIECE | CPUI_INT_2COMP | CPUI_INT_NEGATE | CPUI_BOOL_NEGATE | CPUI_BOOL_AND | CPUI_BOOL_OR
            | CPUI_BOOL_XOR | CPUI_PTRADD | CPUI_PTRSUB | CPUI_CAST | CPUI_INT_CARRY | CPUI_INT_SCARRY
            | CPUI_INT_SBORROW | CPUI_POPCOUNT
    )
}

fn commutative(code: OpCode) -> bool {
    use OpCode::*;
    matches!(code, CPUI_INT_ADD | CPUI_INT_AND | CPUI_INT_OR | CPUI_INT_XOR | CPUI_INT_MULT | CPUI_INT_EQUAL | CPUI_INT_NOTEQUAL | CPUI_BOOL_AND | CPUI_BOOL_OR | CPUI_BOOL_XOR)
}

/// A constant, or a COPY chain of one.
fn constant(data: &Funcdata, v: VarnodeId) -> Option<(u64, int4)> {
    let n = data.vbank().get(v)?;
    crate::p3_dataflow::kuna_strlenscan::copied_constant(data, v).map(|k| (k, n.get_size()))
}

struct Loop {
    guard: BlockId,
    entry: BlockId,
    header: BlockId,
    tail: BlockId,
    exit: BlockId,
    /// The guard's exit: `exit`, or a twin return block (see `twin_return`).
    guard_exit: BlockId,
    blocks: BTreeSet<BlockId>,
    /// `(phi, entry value, back-edge value)` per MULTIEQUAL of the header.
    phis: Vec<(OpId, VarnodeId, VarnodeId)>,
}

fn defined_in(data: &Funcdata, v: VarnodeId, set: &BTreeSet<BlockId>) -> bool {
    data.vbank().get(v).and_then(|n| n.get_def()).and_then(|d| data.obank().get(d)).and_then(|o| o.get_parent()).is_some_and(|b| set.contains(&b))
}

fn find_loop(data: &Funcdata, header: BlockId) -> Option<Loop> {
    let hb = data.bblocks_ref().block(header);
    if hb.size_in() != 2 {
        return None;
    }
    for back in 0..2 {
        let (tail, entry) = (hb.get_in(back), hb.get_in(1 - back));
        if tail == entry {
            continue;
        }
        let Some((t, f)) = taken(data, tail) else { continue };
        if t != header && f != header {
            continue;
        }
        let exit = if t == header { f } else { t };
        if exit == header {
            continue;
        }
        // The loop: the header and everything that reaches the tail without it.
        let mut blocks: BTreeSet<BlockId> = [header, tail].into_iter().collect();
        let mut work = vec![tail];
        while let Some(b) = work.pop() {
            if b == header {
                continue;
            }
            let bb = data.bblocks_ref().block(b);
            for i in 0..bb.size_in() {
                let p = bb.get_in(i);
                if blocks.len() > 512 {
                    return None;
                }
                if blocks.insert(p) {
                    work.push(p);
                }
            }
        }
        if blocks.contains(&entry) || blocks.contains(&exit) {
            continue;
        }
        // Only the tail leaves the loop, and only to the exit.
        let leaves = blocks.iter().any(|&b| {
            let bb = data.bblocks_ref().block(b);
            (0..bb.size_out()).any(|i| !blocks.contains(&bb.get_out(i)) && !(b == tail && bb.get_out(i) == exit))
        });
        if leaves {
            continue;
        }
        let eb = data.bblocks_ref().block(entry);
        let guard = if eb.size_out() == 2 {
            entry
        } else if eb.size_out() == 1 && eb.size_in() == 1 {
            eb.get_in(0)
        } else {
            continue;
        };
        let entry_side = if guard == entry { header } else { entry };
        let Some((gt, gf)) = taken(data, guard) else { continue };
        let guard_exit = if gt == entry_side { gf } else if gf == entry_side { gt } else { continue };
        if guard_exit == entry_side || blocks.contains(&guard_exit) {
            continue;
        }
        let mut phis = Vec::new();
        for op in data.bb_ops(header) {
            let o = data.obank().get(op)?;
            if o.code() != OpCode::CPUI_MULTIEQUAL {
                continue;
            }
            phis.push((op, o.get_in(1 - back as int4)?, o.get_in(back as int4)?));
        }
        return Some(Loop { guard, entry, header, tail, exit, guard_exit, blocks, phis });
    }
    None
}

/// The condition under which `b` takes its edge to `target`, as `(varnode,
/// sense)`, with BOOL_NEGATEs stripped.
fn condition(data: &Funcdata, b: BlockId, target: BlockId) -> Option<(VarnodeId, bool)> {
    let (t, _) = taken(data, b)?;
    let mut cond = data.obank().get(data.bb_op_tail(b)?)?.get_in(1)?;
    let mut sense = t == target;
    for _ in 0..4 {
        let d = data.vbank().get(cond)?.get_def().and_then(|d| data.obank().get(d));
        match d {
            Some(o) if o.code() == OpCode::CPUI_BOOL_NEGATE => {
                cond = o.get_in(0)?;
                sense = !sense;
            }
            _ => break,
        }
    }
    Some((cond, sense))
}

struct Rotation<'a> {
    l: &'a Loop,
    /// Back-edge value -> header MULTIEQUAL output.
    back: BTreeMap<VarnodeId, VarnodeId>,
    /// Back-edge value -> entry value.
    entry: BTreeMap<VarnodeId, VarnodeId>,
}

impl Rotation<'_> {
    /// The header MULTIEQUAL output standing for `v`: a back-edge value, or a
    /// COPY of one inside the loop.
    fn header_value(&self, data: &Funcdata, mut v: VarnodeId) -> Option<VarnodeId> {
        for _ in 0..4 {
            if let Some(&phi) = self.back.get(&v) {
                return Some(phi);
            }
            let d = data.obank().get(data.vbank().get(v)?.get_def()?)?;
            if d.code() != OpCode::CPUI_COPY || !d.get_parent().is_some_and(|b| self.l.blocks.contains(&b)) {
                return None;
            }
            v = d.get_in(0)?;
        }
        None
    }

    /// Can `v` be evaluated at the new header (a back-edge value, an invariant,
    /// or a pure op over those)? `ops` collects the ops to copy, leaves first.
    fn movable(&self, data: &Funcdata, v: VarnodeId, ops: &mut Vec<OpId>, depth: u32) -> bool {
        if self.back.contains_key(&v) || !defined_in(data, v, &self.l.blocks) {
            return true;
        }
        if depth > 8 {
            return false;
        }
        let Some(d) = data.vbank().get(v).and_then(|n| n.get_def()) else { return false };
        let Some(o) = data.obank().get(d) else { return false };
        if !pure(o.code()) {
            return false;
        }
        for i in 0..o.num_input() {
            let Some(x) = o.get_in(i) else { return false };
            if data.vbank().get(x).is_some_and(|n| n.is_constant()) {
                continue;
            }
            if !self.movable(data, x, ops, depth + 1) {
                return false;
            }
        }
        if !ops.contains(&d) {
            ops.push(d);
        }
        true
    }

    /// Is the guard's `g` the tail's `f` evaluated on the entry values?
    fn same(&self, data: &Funcdata, g: VarnodeId, f: VarnodeId, depth: u32) -> bool {
        let f = self.entry.get(&f).copied().unwrap_or(f);
        if g == f {
            return true;
        }
        if let (Some(a), Some(b)) = (constant(data, g), constant(data, f)) {
            return a == b;
        }
        if depth > 8 {
            return false;
        }
        let def = |v: VarnodeId| data.vbank().get(v).and_then(|n| n.get_def()).and_then(|d| data.obank().get(d));
        let (Some(a), Some(b)) = (def(g), def(f)) else { return false };
        if a.code() != b.code() || a.num_input() != b.num_input() || !pure(a.code()) {
            return false;
        }
        let ins = |o: &crate::op::PcodeOp| (0..o.num_input()).map(|i| o.get_in(i)).collect::<Option<Vec<_>>>();
        let (Some(ai), Some(bi)) = (ins(a), ins(b)) else { return false };
        let pairwise = |x: &[VarnodeId], y: &[VarnodeId]| x.iter().zip(y).all(|(&p, &q)| self.same(data, p, q, depth + 1));
        pairwise(&ai, &bi) || (commutative(a.code()) && ai.len() == 2 && pairwise(&ai, &[bi[1], bi[0]]))
    }
}

/// `(exit MULTIEQUAL, value read from the new header)` per MULTIEQUAL of the
/// exit, when each can move.
fn exit_values(data: &Funcdata, l: &Loop, back: &BTreeMap<VarnodeId, VarnodeId>, entry_of: &BTreeMap<VarnodeId, VarnodeId>) -> Option<Vec<(OpId, VarnodeId)>> {
    let eb = data.bblocks_ref().block(l.exit);
    let (gi, ti) = (eb.get_in_index(l.guard), eb.get_in_index(l.tail));
    if gi < 0 || ti < 0 {
        return None;
    }
    let mut out = Vec::new();
    for op in data.bb_ops(l.exit) {
        let o = data.obank().get(op)?;
        if o.code() != OpCode::CPUI_MULTIEQUAL {
            continue;
        }
        let (vg, vt) = (o.get_in(gi)?, o.get_in(ti)?);
        let (now, at_entry) = match back.get(&vt) {
            Some(&phi) => (phi, *entry_of.get(&vt)?),
            None if !defined_in(data, vt, &l.blocks) => (vt, vt),
            None => return None,
        };
        let same = vg == at_entry || matches!((constant(data, vg), constant(data, at_entry)), (Some(a), Some(b)) if a == b);
        if !same {
            return None;
        }
        out.push((op, now));
    }
    Some(out)
}

/// The exits are two blocks holding only COPYs and a RETURN, the tail's reached
/// from the tail alone and the guard's from the guard alone, and the guard's
/// returns what the tail's would on the entry values. `(op, slot, header
/// value)` for each input of the tail's block that reads a back-edge value.
fn twin_return(data: &Funcdata, l: &Loop, rot: &Rotation) -> Option<Vec<(OpId, int4, VarnodeId)>> {
    let only_return = |b: BlockId, from: BlockId| -> Option<(Vec<OpId>, OpId)> {
        let bb = data.bblocks_ref().block(b);
        let ops = data.bb_ops(b);
        let (&ret, copies) = ops.split_last()?;
        let ok = bb.size_in() == 1
            && bb.get_in(0) == from
            && data.obank().get(ret)?.code() == OpCode::CPUI_RETURN
            && copies.iter().all(|&c| data.obank().get(c).is_some_and(|o| o.code() == OpCode::CPUI_COPY));
        ok.then(|| (copies.to_vec(), ret))
    };
    let ((tcopies, rt), (_, rg)) = (only_return(l.exit, l.tail)?, only_return(l.guard_exit, l.guard)?);
    let (ot, og) = (data.obank().get(rt)?, data.obank().get(rg)?);
    if ot.num_input() != og.num_input() {
        return None;
    }
    for i in 1..ot.num_input() {
        if !rot.same(data, og.get_in(i)?, ot.get_in(i)?, 0) {
            return None;
        }
    }
    let mut subst = Vec::new();
    for op in tcopies.into_iter().chain([rt]) {
        let o = data.obank().get(op)?;
        for i in 0..o.num_input() {
            let v = o.get_in(i)?;
            match rot.header_value(data, v) {
                Some(phi) => subst.push((op, i, phi)),
                None if defined_in(data, v, &l.blocks) => return None,
                None => {}
            }
        }
    }
    Some(subst)
}

fn side_effect_free(data: &Funcdata, b: BlockId) -> bool {
    data.bb_ops(b).into_iter().all(|op| {
        data.obank().get(op).is_some_and(|o| {
            !matches!(o.code(), OpCode::CPUI_STORE | OpCode::CPUI_CALL | OpCode::CPUI_CALLIND | OpCode::CPUI_CALLOTHER | OpCode::CPUI_LOAD)
        })
    })
}

/// `(i, n, unsigned)` when `v` is `i < n` (`INT_SLESS`/`INT_LESS`), or `k <= n`
/// with a constant `k`, read as `k - 1 < n`.
fn below(data: &Funcdata, v: VarnodeId) -> Option<(Result<VarnodeId, (u64, int4)>, VarnodeId, bool)> {
    let o = data.obank().get(data.vbank().get(v)?.get_def()?)?;
    let (a, b) = (o.get_in(0)?, o.get_in(1)?);
    match o.code() {
        OpCode::CPUI_INT_SLESS | OpCode::CPUI_INT_LESS => {
            let left = constant(data, a).map_or(Ok(a), Err);
            Some((left, b, o.code() == OpCode::CPUI_INT_LESS))
        }
        OpCode::CPUI_INT_SLESSEQUAL | OpCode::CPUI_INT_LESSEQUAL => {
            let (k, size) = constant(data, a).filter(|&(_, s)| (1..=8).contains(&s))?;
            let signed = o.code() == OpCode::CPUI_INT_SLESSEQUAL;
            let min = if signed { 1u64 << (8 * size - 1) } else { 0 };
            (k & crate::p3_dataflow::kuna_strlenscan::mask(size) != min).then(|| (Err((k.wrapping_sub(1) & crate::p3_dataflow::kuna_strlenscan::mask(size), size)), b, !signed))
        }
        _ => None,
    }
}

/// A counting loop exits on `i + 1 == n`: `i` steps by one from `x0`, and the
/// guard entered the loop only when `x0 < n`. Then `i + 1 != n` is `i + 1 < n`
/// on every iteration (`i` runs from `x0` up to `n - 1` without wrapping), so
/// the tail's condition is replaced by that compare, which is the guard's on
/// the entry value.
fn counting(data: &Funcdata, rot: &Rotation, (f, f_sense): (VarnodeId, bool), (g, g_sense): (VarnodeId, bool)) -> Option<(VarnodeId, VarnodeId, bool)> {
    if !g_sense {
        return None;
    }
    let Some((x, n, unsigned)) = below(data, g) else { return None };
    let Some(fo) = data.vbank().get(f).and_then(|v| v.get_def()).and_then(|d| data.obank().get(d)) else { return None };
    let continue_if_ne = match (fo.code(), f_sense) {
        (OpCode::CPUI_INT_NOTEQUAL, true) | (OpCode::CPUI_INT_EQUAL, false) => true,
        _ => return None,
    };
    let (Some(a), Some(b)) = (fo.get_in(0), fo.get_in(1)) else { return None };
    let step_of = |v: VarnodeId| -> Option<VarnodeId> {
        rot.back.get(&v)?;
        let d = data.obank().get(data.vbank().get(v)?.get_def()?)?;
        let one = d.get_in(1).and_then(|c| data.vbank().get(c)).is_some_and(|c| c.is_constant() && c.get_offset() == 1);
        (d.code() == OpCode::CPUI_INT_ADD && one && rot.back.get(&v) == d.get_in(0).as_ref()).then_some(v)
    };
    let (i1, bound) = match (step_of(a), step_of(b)) {
        (Some(i1), None) => (i1, b),
        (None, Some(i1)) => (i1, a),
        _ => return None,
    };
    if !continue_if_ne || defined_in(data, bound, &rot.l.blocks) || !rot.same(data, n, bound, 0) {
        return None;
    }
    let x0 = rot.entry[&i1];
    let starts = match x {
        Ok(xv) => rot.same(data, xv, x0, 0),
        Err((k, size)) => constant(data, x0).is_some_and(|(c, s)| s == size && c & crate::p3_dataflow::kuna_strlenscan::mask(size) == k),
    };
    if !starts {
        return None;
    }
    Some((i1, bound, unsigned))
}

/// Replace the tail's condition by `i1 < bound` (see [`counting`]).
fn apply_counting(data: &mut Funcdata, l: &Loop, (i1, bound, unsigned): (VarnodeId, VarnodeId, bool)) -> kuna_base::error::KunaResult<()> {
    use kuna_base::error::KunaError;
    let branch = data.bb_op_tail(l.tail).ok_or_else(|| KunaError::lowlevel("looprotate: branch"))?;
    let at = data.obank().get(branch).map(|o| o.get_addr().clone()).ok_or_else(|| KunaError::lowlevel("looprotate: branch"))?;
    let code = if unsigned { OpCode::CPUI_INT_LESS } else { OpCode::CPUI_INT_SLESS };
    let cmp = data.new_op(2, at.clone());
    data.op_set_opcode_code(cmp, code);
    data.op_set_input(cmp, i1, 0)?;
    data.op_set_input(cmp, bound, 1)?;
    let mut c = data.new_unique_out(1, cmp)?;
    data.op_insert_before(cmp, branch);
    let continue_on_true = taken(data, l.tail).is_some_and(|(t, _)| l.header == t);
    if !continue_on_true {
        let neg = data.new_op(1, at);
        data.op_set_opcode_code(neg, OpCode::CPUI_BOOL_NEGATE);
        data.op_set_input(neg, c, 0)?;
        c = data.new_unique_out(1, neg)?;
        data.op_insert_before(neg, branch);
    }
    data.op_set_input(branch, c, 1)?;
    Ok(())
}

fn rotate(data: &mut Funcdata, header: BlockId) -> kuna_base::error::KunaResult<bool> {
    use kuna_base::error::KunaError;
    let Some(l) = find_loop(data, header) else { return Ok(false) };
    if l.phis.is_empty() || (l.entry != l.guard && !side_effect_free(data, l.entry)) {
        return Ok(false);
    }
    let back: BTreeMap<VarnodeId, VarnodeId> = l.phis.iter().filter_map(|&(phi, _, b)| Some((b, data.obank().get(phi)?.get_out()?))).collect();
    let entry_of: BTreeMap<VarnodeId, VarnodeId> = l.phis.iter().map(|&(_, e, b)| (b, e)).collect();
    let rot = Rotation { l: &l, back, entry: entry_of };
    let Some((f, f_sense)) = condition(data, l.tail, l.header) else { return Ok(false) };
    let entry_side = if l.guard == l.entry { l.header } else { l.entry };
    let Some((g, g_sense)) = condition(data, l.guard, entry_side) else { return Ok(false) };
    let plan = if f_sense == g_sense && rot.same(data, g, f, 0) {
        None
    } else {
        let Some(plan) = counting(data, &rot, (f, f_sense), (g, g_sense)) else { return Ok(false) };
        Some(plan)
    };
    let mut ops = Vec::new();
    let tail_cond = data.obank().get(data.bb_op_tail(l.tail).ok_or_else(|| KunaError::lowlevel("looprotate: tail"))?).and_then(|o| o.get_in(1)).ok_or_else(|| KunaError::lowlevel("looprotate: cond"))?;
    if plan.is_none() && !rot.movable(data, tail_cond, &mut ops, 0) {
        return Ok(false);
    }
    let (exits, twin) = if l.guard_exit == l.exit {
        let Some(exits) = exit_values(data, &l, &rot.back, &rot.entry) else { return Ok(false) };
        (exits, None)
    } else {
        let Some(twin) = twin_return(data, &l, &rot) else { return Ok(false) };
        (Vec::new(), Some(twin))
    };
    let back = rot.back.clone();
    let cond = match plan {
        Some(plan) => {
            apply_counting(data, &l, plan)?;
            let c = data.obank().get(data.bb_op_tail(l.tail).ok_or_else(|| KunaError::lowlevel("looprotate: tail"))?).and_then(|o| o.get_in(1)).ok_or_else(|| KunaError::lowlevel("looprotate: cond"))?;
            if !rot.movable(data, c, &mut ops, 0) {
                return Err(KunaError::lowlevel("looprotate: counting compare not movable"));
            }
            c
        }
        None => tail_cond,
    };

    // The new header and its edges, in the tail's out-slot order.
    let tail_op = data.bb_op_tail(l.tail).ok_or_else(|| KunaError::lowlevel("looprotate: tail op"))?;
    let at = data.obank().get(tail_op).map(|o| o.get_addr().clone()).ok_or_else(|| KunaError::lowlevel("looprotate: addr"))?;
    let flip = data.obank().get(tail_op).is_some_and(|o| o.is_boolean_flip());
    let tail_slots: Vec<BlockId> = (0..2).map(|i| data.bblocks_ref().block(l.tail).get_out(i)).collect();
    let graph = data.bblocks_root_pub();
    let n = data.bblocks_mut().new_block_basic(graph);
    data.set_basic_block_range(n, &at, &at);
    for &t in &tail_slots {
        data.bblocks_mut().add_edge(n, t);
    }
    for &(op, now) in &exits {
        let slot = data.obank().get(op).map_or(0, |o| o.num_input());
        data.op_insert_input(op, now, slot)?;
    }
    if let Some(subst) = &twin {
        for &(op, slot, phi) in subst {
            data.op_set_input(op, phi, slot)?;
        }
    }
    // Drop the guard's exit and the tail's exit.
    let ge = data.bblocks_ref().block(l.guard).get_out_index(l.guard_exit);
    data.remove_branch(l.guard, ge)?;
    let te = data.bblocks_ref().block(l.tail).get_out_index(l.exit);
    data.remove_branch(l.tail, te)?;
    // The header's MULTIEQUALs move to the new header, whose in edges are the
    // entry side and the tail.
    let from_entry = if l.guard == l.entry { l.guard } else { l.entry };
    for &(phi, e, b) in &l.phis {
        data.op_uninsert(phi);
        data.op_set_all_input(phi, &[e, b])?;
        data.op_insert_begin(phi, n);
    }
    data.bblocks_mut().switch_edge(from_entry, l.header, n);
    data.bblocks_mut().switch_edge(l.tail, l.header, n);
    // The condition, copied over the moved MULTIEQUAL outputs.
    let mut map: BTreeMap<VarnodeId, VarnodeId> = back;
    let branch = data.new_op(2, at.clone());
    data.op_set_opcode_code(branch, OpCode::CPUI_CBRANCH);
    let dest = data.new_code_ref(&at);
    data.op_set_input(branch, dest, 0)?;
    data.op_insert_end(branch, n);
    for op in ops {
        let (code, ins, size) = {
            let o = data.obank().get(op).ok_or_else(|| KunaError::lowlevel("looprotate: op"))?;
            let ins: Vec<VarnodeId> = (0..o.num_input()).filter_map(|i| o.get_in(i)).collect();
            let size = o.get_out().and_then(|v| data.vbank().get(v)).map_or(1, |v| v.get_size());
            (o.code(), ins, size)
        };
        let copy = data.new_op(ins.len() as int4, at.clone());
        data.op_set_opcode_code(copy, code);
        for (i, x) in ins.into_iter().enumerate() {
            let input = match map.get(&x) {
                Some(&m) => m,
                None => match data.vbank().get(x).filter(|v| v.is_constant()).map(|v| (v.get_size(), v.get_offset())) {
                    Some((s, k)) => data.new_constant(s, k as uintb),
                    None => x,
                },
            };
            data.op_set_input(copy, input, i as int4)?;
        }
        let out = data.new_unique_out(size, copy)?;
        data.op_insert_before(copy, branch);
        let orig = data.obank().get(op).and_then(|o| o.get_out()).ok_or_else(|| KunaError::lowlevel("looprotate: out"))?;
        map.insert(orig, out);
    }
    let c = *map.get(&cond).ok_or_else(|| KunaError::lowlevel("looprotate: cond copy"))?;
    data.op_set_input(branch, c, 1)?;
    if flip {
        if let Some(o) = data.obank_mut().get_mut(branch) {
            o.set_flag(crate::op::pcodeop_flags::boolean_flip);
        }
    }
    if twin.is_some() {
        data.remove_unreachable_blocks(false, true)?;
    }
    data.structure_reset();
    Ok(true)
}

/// Rotate guarded do-while loops into while loops (see the module docs).
pub struct ActionLoopRotate {
    base: ActionBase,
}

impl ActionLoopRotate {
    pub fn boxed(g: &str) -> Box<dyn Action> {
        Box::new(ActionLoopRotate { base: ActionBase::new(0, "looprotate", g) })
    }
}

impl Action for ActionLoopRotate {
    fn base(&self) -> &ActionBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ActionBase {
        &mut self.base
    }
    fn clone_filtered(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Action>> {
        grouplist.contains(self.get_group()).then(|| Box::new(ActionLoopRotate { base: self.base.clone() }) as Box<dyn Action>)
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        if !data.get_arch().loop_rotate {
            return 0;
        }
        let heads: Vec<BlockId> = (0..data.bblocks_get_size()).map(|i| data.bblocks_get_block(i)).collect();
        for h in heads {
            match rotate(data, h) {
                Ok(true) => {
                    self.base.count += 1;
                    return 1;
                }
                Ok(false) => {}
                Err(_) => return 0,
            }
        }
        0
    }
}
