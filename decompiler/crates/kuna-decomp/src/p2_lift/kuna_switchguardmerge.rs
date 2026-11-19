//! (kuna `switchguardmerge`) An equality test a range guard sends out-of-range
//! values to becomes a case of the switch.
//!
//! gcc and clang lower `switch` with a far outlier (`case 100:` beside
//! `case 0 ... 6:`) to a range guard around the jump table and an equality
//! test on the guard's other side:
//!
//! ```text
//! if (a0 <= 6) { switch (a0) { case 0: ... default: return -1; } }
//! if (a0 != 100) return -1;
//! return f8(0x65);
//! ```
//!
//! The source had one switch. [`ActionSwitchGuardMerge`] walks up from a labelled
//! switch through the blocks that lead into it alone (each either a guard
//! whose other side is the switch's default, a folded guard whose branch is a
//! constant, or a pass-through block), to a guard `G` whose other side is a
//! chain of single-entry blocks each testing the switch variable against one
//! constant, the last falling through to the default. It is rewritten only
//! when it provably means the same switch: the guard and every block between
//! it and the switch are evaluated on the chain constants (each must reach the
//! chain, then pass straight to the switch) and on every case label (each must
//! stay on the switch side), the blocks on the way compute into values nothing
//! outside the function reads, and the default's MULTIEQUALs read the same
//! value from the chain and the switch. Each constant then becomes a case
//! (`add_block_to_switch` + `push_branch`, the guard-folding primitives), and
//! the guard's edge to the chain is removed, which leaves the chain
//! unreachable.
//!
//! gcc also repeats the range check unsigned (`x >u 6`) right before the
//! table, landing on case 0, and the recovery folds that check as the default,
//! so the switch's default is one of its cases. Then the chain's last miss
//! becomes the default instead, provided the guards still standing and `G`
//! together admit only case labels (`JumpTable::kuna_standing_guard_range`),
//! so no value the switch sees reached the folded default, and every guard on
//! the way rejects to that same miss.

use kuna_num::opcodes::OpCode;

use crate::action::{Action, ActionBase, ActionContext, ActionGroupList, ApplyResult};
use crate::context::{BlockId, VarnodeId};
use crate::funcdata::Funcdata;
use crate::jumptable::NO_LABEL;
use crate::p3_dataflow::kuna_strlenscan::{eval, taken};

/// Ops that only compute, into values nothing outside the function can see:
/// a block made of them can run on more paths.
fn pure_block(data: &Funcdata, b: BlockId) -> bool {
    use OpCode::*;
    data.bb_ops(b).into_iter().all(|o| {
        data.obank().get(o).is_some_and(|p| {
            let computes = matches!(
                p.code(),
                CPUI_COPY | CPUI_CBRANCH | CPUI_BRANCH | CPUI_INT_ZEXT | CPUI_INT_SEXT | CPUI_SUBPIECE | CPUI_PIECE
                    | CPUI_INT_ADD | CPUI_INT_SUB | CPUI_INT_AND | CPUI_INT_OR | CPUI_INT_XOR | CPUI_INT_EQUAL
                    | CPUI_INT_NOTEQUAL | CPUI_INT_LESS | CPUI_INT_LESSEQUAL | CPUI_INT_SLESS | CPUI_INT_SLESSEQUAL
                    | CPUI_INT_CARRY | CPUI_INT_SCARRY | CPUI_INT_SBORROW | CPUI_BOOL_NEGATE | CPUI_BOOL_AND
                    | CPUI_BOOL_OR | CPUI_BOOL_XOR | CPUI_INT_2COMP | CPUI_INT_NEGATE | CPUI_CAST
            );
            let local_out = p.get_out().and_then(|v| data.vbank().get(v)).is_none_or(|v| !v.is_persist() && !v.is_addr_tied());
            computes && local_out
        })
    })
}

/// The block a CBRANCH block `b` goes to when the switch variable `v` is `x`.
fn branch_for(data: &Funcdata, b: BlockId, v: VarnodeId, x: u64, all: &[BlockId]) -> Option<BlockId> {
    let (t, f) = taken(data, b)?;
    let cond = data.obank().get(data.bb_op_tail(b)?)?.get_in(1)?;
    Some(if eval(data, cond, &[(v, x)], all, 0)? != 0 { t } else { f })
}

/// The constant an equality block `t` compares `v` with.
fn compared_constant(data: &Funcdata, t: BlockId, v: VarnodeId) -> Option<u64> {
    let mut cond = data.obank().get(data.bb_op_tail(t)?)?.get_in(1)?;
    for _ in 0..2 {
        let d = data.obank().get(data.vbank().get(cond)?.get_def()?)?;
        match d.code() {
            OpCode::CPUI_BOOL_NEGATE => cond = d.get_in(0)?,
            OpCode::CPUI_INT_EQUAL | OpCode::CPUI_INT_NOTEQUAL => {
                let (a, b) = (d.get_in(0)?, d.get_in(1)?);
                let k = |x: VarnodeId| data.vbank().get(x).filter(|n| n.is_constant()).map(|n| n.get_offset());
                return if a == v { k(b) } else if b == v { k(a) } else { None };
            }
            _ => return None,
        }
    }
    None
}

/// Do `b`'s MULTIEQUALs read the same value from `x` as from `y`? `None`
/// when either is not a predecessor.
fn same_phi_inputs(data: &Funcdata, b: BlockId, x: BlockId, y: BlockId) -> Option<bool> {
    let bb = data.bblocks_ref().block(b);
    let (i, j) = (bb.get_in_index(x), bb.get_in_index(y));
    if i < 0 || j < 0 {
        return None;
    }
    Some(data.bb_ops(b).into_iter().all(|o| {
        data.obank().get(o).is_some_and(|op| op.code() != OpCode::CPUI_MULTIEQUAL || op.get_in(i) == op.get_in(j))
    }))
}

/// The equality chain starting at `head` (entered only from `from`): each
/// block tests the switch variable `v` against one constant not already a case.
/// With `end`, the chain must fall through to it; without, it stops at the
/// first block that is not such a test. Returns the cases and the last miss.
#[allow(clippy::too_many_arguments)]
fn equality_chain(
    data: &Funcdata,
    head: BlockId,
    from: BlockId,
    v: VarnodeId,
    end: Option<BlockId>,
    labels: &[u64],
    all: &[BlockId],
    avoid: &[BlockId],
) -> Option<(Vec<(u64, BlockId, i32)>, BlockId)> {
    let mut cases: Vec<(u64, BlockId, i32)> = Vec::new();
    let (mut t, mut from) = (head, from);
    loop {
        let tb = data.bblocks_ref().block(t);
        let test = tb.size_in() == 1 && tb.get_in(0) == from && tb.size_out() == 2 && pure_block(data, t) && cases.len() < 8;
        let Some(c) = test.then(|| compared_constant(data, t, v)).flatten() else {
            return (end.is_none() && !cases.is_empty()).then_some((cases, t));
        };
        let hit = branch_for(data, t, v, c, all)?;
        let miss = branch_for(data, t, v, c.wrapping_add(1), all)?;
        if hit == miss || avoid.contains(&hit) || cases.iter().any(|&(k, b, _)| k == c || b == hit) || labels.contains(&c) {
            return None;
        }
        let slot = if tb.get_out(0) == hit { 0 } else { 1 };
        cases.push((c, t, slot));
        if Some(miss) == end {
            return Some((cases, miss));
        }
        from = t;
        t = miss;
    }
}

/// One merge: the guard and its edge to the chain (none when the chain is the
/// switch's default), and the cases
/// `(constant, chain block, its out-edge to the case)`.
struct Merge {
    guard: Option<(BlockId, i32)>,
    cases: Vec<(u64, BlockId, i32)>,
    /// The default out-edge and the case out-edge it folds into.
    default_into: Option<(i32, i32)>,
    /// The chain's last miss, which becomes the default when the switch's
    /// default was a case.
    new_default: Option<BlockId>,
}

fn find_merge(data: &Funcdata, table: i32) -> Option<Merge> {
    let jt = data.get_jump_table(table);
    if !jt.is_labelled() {
        return None;
    }
    let ind = data.obank().get(jt.get_indirect_op()?)?;
    let (s, v) = (ind.get_parent()?, ind.get_in(0)?);
    let sb = data.bblocks_ref().block(s);
    let def = jt.get_default_block();
    if def < 0 || def >= sb.size_out() {
        return None;
    }
    let d = sb.get_out(def);
    let labels: Vec<u64> = (0..jt.num_entries()).map(|i| jt.get_label_by_index(i)).filter(|&l| l != NO_LABEL).collect();
    let all: Vec<BlockId> = (0..data.bblocks_get_size()).map(|i| data.bblocks_get_block(i)).collect();
    // The default itself is the chain (the guard was folded onto it).
    let db = data.bblocks_ref().block(d);
    if db.size_in() == 1 {
        let mut avoid: Vec<BlockId> = vec![s];
        avoid.extend((0..sb.size_out()).map(|i| sb.get_out(i)));
        if let Some((cases, miss)) = equality_chain(data, d, s, v, None, &labels, &all, &avoid) {
            // A last miss that is already a case becomes the default itself.
            let into = sb.get_out_index(miss);
            if into < 0 {
                return Some(Merge { guard: None, cases, default_into: None, new_default: None });
            }
            if same_phi_inputs(data, miss, cases.last()?.1, s)? {
                return Some(Merge { guard: None, cases, default_into: Some((def, into)), new_default: None });
            }
        }
    }
    // A default that is also a case came from a range check the recovery folded
    // whose other side was that case; the chain's last miss becomes the real
    // default once the standing guards are known to admit only case labels.
    let labelled_default = jt.get_index_by_block(data, d, 0).is_ok_and(|i| jt.get_label_by_index(i) != NO_LABEL);
    let exit = (!labelled_default).then_some(d);
    // Walk up through the blocks that lead into the switch alone.
    let mut path: Vec<BlockId> = Vec::new();
    let mut rejects: Vec<BlockId> = Vec::new();
    let mut cur = s;
    for _ in 0..6 {
        let cb = data.bblocks_ref().block(cur);
        if cb.size_in() != 1 {
            return None;
        }
        let p = cb.get_in(0);
        if p == s || !pure_block(data, p) {
            return None;
        }
        let pb = data.bblocks_ref().block(p);
        if pb.size_out() == 1 {
            path.push(p);
            cur = p;
            continue;
        }
        if pb.size_out() != 2 {
            return None;
        }
        let off = if pb.get_out(0) == cur { 1 } else { 0 };
        let o = pb.get_out(off);
        // A folded guard's branch is a constant: its other edge is dead.
        let dead_branch = data.bb_op_tail(p).and_then(|t| data.obank().get(t)).and_then(|t| t.get_in(1)).and_then(|c| data.vbank().get(c)).is_some_and(|c| c.is_constant());
        if Some(o) == exit || dead_branch {
            path.push(p);
            cur = p;
            continue;
        }
        // `p` is the candidate guard; `o` the head of the equality chain.
        let mut avoid: Vec<BlockId> = path.clone();
        avoid.extend([s, p]);
        avoid.extend((0..sb.size_out()).map(|i| sb.get_out(i)));
        let Some((cases, miss)) = equality_chain(data, o, p, v, exit, &labels, &all, &avoid) else {
            if exit.is_some() {
                return None;
            }
            rejects.push(o);
            path.push(p);
            cur = p;
            continue;
        };
        let new_default = if exit.is_some() {
            // The default reads the same value whether reached from the chain or the switch.
            if !same_phi_inputs(data, d, cases.last()?.1, s)? {
                return None;
            }
            None
        } else {
            // Every value the standing guards and this one let through is a case
            // label, so no value the switch sees goes to the folded default; a
            // guard that rejects a value sends it where the chain's miss goes.
            let standing = jt.kuna_standing_guard_range(data, v, &[(p, cur)])?;
            if standing.get_size() > labels.len() as u64 || !range_values(&standing).iter().all(|x| labels.contains(x)) {
                return None;
            }
            if avoid.contains(&miss) || rejects.iter().any(|&r| r != miss) {
                return None;
            }
            Some(miss)
        };
        // Each constant reaches the chain at `p`, then passes straight to the switch.
        let mut seq: Vec<BlockId> = path.iter().rev().copied().collect();
        seq.push(s);
        for &(c, ..) in &cases {
            if branch_for(data, p, v, c, &all)? != o {
                return None;
            }
            for w in seq.windows(2) {
                if data.bblocks_ref().block(w[0]).size_out() == 2 && branch_for(data, w[0], v, c, &all)? != w[1] {
                    return None;
                }
            }
        }
        // Every case label stays on the switch side of the guard.
        for &l in &labels {
            if branch_for(data, p, v, l, &all)? != cur {
                return None;
            }
        }
        return Some(Merge { guard: Some((p, off)), cases, default_into: None, new_default });
    }
    None
}

/// The values of a range small enough to list.
fn range_values(r: &crate::rangeutil::CircleRange) -> Vec<u64> {
    let mut out = Vec::new();
    let mut x = r.get_min();
    out.push(x);
    while out.len() < 4096 && r.get_next(&mut x) && x != r.get_min() {
        out.push(x);
    }
    out
}

/// Merge equality guards into their switch (see the module docs).
pub struct ActionSwitchGuardMerge {
    base: ActionBase,
}

impl ActionSwitchGuardMerge {
    pub fn boxed(g: &str) -> Box<dyn Action> {
        Box::new(ActionSwitchGuardMerge { base: ActionBase::new(0, "switchguardmerge", g) })
    }
}

impl Action for ActionSwitchGuardMerge {
    fn base(&self) -> &ActionBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ActionBase {
        &mut self.base
    }
    fn clone_filtered(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Action>> {
        grouplist.contains(self.get_group()).then(|| Box::new(ActionSwitchGuardMerge { base: self.base.clone() }) as Box<dyn Action>)
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        if !data.get_arch().switch_guard_merge {
            return 0;
        }
        for i in 0..data.num_jump_tables() {
            let Some(m) = find_merge(data, i) else { continue };
            let Some(s) = data.get_jump_table(i).get_indirect_op().and_then(|o| data.obank().get(o)).and_then(|o| o.get_parent()) else { continue };
            let placeholder = crate::jumptable::JumpTable::new(data.get_address().clone());
            let mut jt = std::mem::replace(data.get_jump_table_mut(i), placeholder);
            let mut ok = true;
            for &(c, t, slot) in &m.cases {
                let hit = data.bblocks_ref().block(t).get_out(slot);
                if jt.add_block_to_switch(data, hit, c).is_err() || data.push_branch(t, slot, s).is_err() {
                    ok = false;
                    break;
                }
            }
            if let (true, Some(miss), Some(&(_, last, _))) = (ok, m.new_default, m.cases.last()) {
                // The last chain block's one remaining edge is its miss.
                ok = jt.add_block_to_switch(data, miss, NO_LABEL).is_ok();
                if ok {
                    data.bblocks_mut().move_out_edge(last, 0, s);
                    jt.set_last_as_default();
                }
            }
            if let (true, Some((from, into))) = (ok, m.default_into) {
                let to = data.bblocks_ref().block(s).get_out(into);
                let at = jt.get_index_by_block(data, to, 0).ok().map(|k| jt.get_address_by_index(k));
                match at {
                    Some(at) => {
                        jt.kuna_merge_out_edge(from, into, &at);
                        ok = data.remove_branch(s, from).is_ok() && data.remove_unreachable_blocks(false, true).is_ok();
                    }
                    None => ok = false,
                }
            }
            *data.get_jump_table_mut(i) = jt;
            if !ok {
                return 0;
            }
            if let Some((g, off)) = m.guard {
                if data.remove_branch(g, off).is_err() || data.remove_unreachable_blocks(false, true).is_err() {
                    return 0;
                }
            }
            data.sblocks_clear();
            self.base_mut().count += 1;
        }
        0
    }
}
