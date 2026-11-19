//! P3 value-range folding (kuna `rangefold`).
//!
//! A conditional branch on `V cmp K` bounds the value of `V` in every block that
//! one of its out-edges controls: the edge's target dominates the block and is
//! entered only through that edge (`FlowBlock::restrictedByConditional`, the test
//! `ActionConditionalConst` uses for `V == K`).  Upstream propagates only the
//! equality case.  This pass keeps a `CircleRange` per such edge, pulled back
//! through the comparison and then through the `COPY`, extension and
//! add-constant chain that defined `V`, and intersects every range whose edge
//! controls a block.  In that block it folds
//!
//! * a comparison of `V` against a constant whose outcome the range fixes into
//!   the constant `true` or `false` (a branch on it is then removed by
//!   `ActionDeterminedBranch`), and
//! * `V & M`, where `M` is `2^k - 1` and the range lies within `[0, M]`, into
//!   `V`.
//!
//! `V` is an SSA value, so the range holds for its every read in the block.

use std::collections::HashMap;

use kuna_base::types::{int4, uintb};
use kuna_num::opcodes::OpCode;

use crate::action::{Action, ActionBase, ActionContext, ActionGroupList, ApplyResult};
use crate::context::{BlockId, OpId, VarnodeId};
use crate::funcdata::Funcdata;
use crate::rangeutil::CircleRange;

/// The most `COPY`/extension/add-constant steps a range is pulled back through.
const CHAIN_DEPTH: usize = 3;

/// The range a varnode holds in every block `edge_block` dominates.
struct Fact {
    edge_block: BlockId,
    range: CircleRange,
}

fn is_compare(opc: OpCode) -> bool {
    matches!(
        opc,
        OpCode::CPUI_INT_EQUAL
            | OpCode::CPUI_INT_NOTEQUAL
            | OpCode::CPUI_INT_LESS
            | OpCode::CPUI_INT_LESSEQUAL
            | OpCode::CPUI_INT_SLESS
            | OpCode::CPUI_INT_SLESSEQUAL
    )
}

fn constant(data: &Funcdata, vn: VarnodeId) -> Option<uintb> {
    let v = data.vbank().get(vn)?;
    v.is_constant().then(|| v.get_offset())
}

/// The variable, the constant and the variable's slot of a comparison against
/// a constant.
fn const_compare(data: &Funcdata, op: OpId) -> Option<(VarnodeId, uintb, int4)> {
    let o = data.obank().get(op)?;
    if !is_compare(o.code()) || o.num_input() != 2 {
        return None;
    }
    let (a, b) = (o.get_in(0)?, o.get_in(1)?);
    match (constant(data, a), constant(data, b)) {
        (None, Some(k)) => Some((a, k, 0)),
        (Some(k), None) => Some((b, k, 1)),
        _ => None,
    }
}

/// The range of the variable compared in `op` for which the comparison is `value`.
fn compare_range(data: &Funcdata, op: OpId, value: bool) -> Option<(VarnodeId, CircleRange)> {
    let (vn, k, slot) = const_compare(data, op)?;
    let size = data.vbank().get(vn)?.get_size();
    let mut range = CircleRange::new_bool(value);
    let opc = data.obank().get(op)?.code();
    range.pull_back_binary(opc, k, slot, size, 1).then_some((vn, range))
}

/// The input a `COPY`, extension or add-constant op defining `vn` reads, and the
/// range that input holds when `vn` holds `range`.
fn pull_back_def(data: &Funcdata, vn: VarnodeId, range: &CircleRange) -> Option<(VarnodeId, CircleRange)> {
    let def = data.vbank().get(vn)?.get_def()?;
    let o = data.obank().get(def)?;
    let input = o.get_in(0)?;
    if constant(data, input).is_some() {
        return None;
    }
    let in_size = data.vbank().get(input)?.get_size();
    let out_size = data.vbank().get(vn)?.get_size();
    let mut r = range.clone();
    let ok = match o.code() {
        OpCode::CPUI_COPY | OpCode::CPUI_INT_ZEXT | OpCode::CPUI_INT_SEXT => {
            r.pull_back_unary(o.code(), in_size, out_size)
        }
        OpCode::CPUI_INT_ADD | OpCode::CPUI_INT_SUB if o.num_input() == 2 => {
            let k = constant(data, o.get_in(1)?)?;
            r.pull_back_binary(o.code(), k, 0, in_size, out_size)
        }
        _ => false,
    };
    (ok && !r.is_empty() && !r.is_full()).then_some((input, r))
}

fn last_op(data: &Funcdata, bl: BlockId) -> Option<OpId> {
    match data.bblocks_ref().block(bl).kind() {
        crate::block::BlockKind::Basic(bd) => bd.op_tail,
        _ => None,
    }
}

/// Every range a conditional branch establishes, keyed by the varnode it bounds.
fn collect_facts(data: &Funcdata) -> HashMap<VarnodeId, Vec<Fact>> {
    let mut facts: HashMap<VarnodeId, Vec<Fact>> = HashMap::new();
    let graph = data.bblocks_ref();
    for i in 0..data.bblocks_get_size() {
        let bl = data.bblocks_get_block(i);
        let Some(cbranch) = last_op(data, bl) else { continue };
        let Some(op) = data.obank().get(cbranch) else { continue };
        if op.code() != OpCode::CPUI_CBRANCH || graph.block(bl).size_out() != 2 {
            continue;
        }
        if graph.block(bl).get_out(0) == graph.block(bl).get_out(1) {
            continue;
        }
        let flip = op.is_boolean_flip();
        let Some(mut cond) = op.get_in(1) else { continue };
        let mut negated = false;
        while let Some(def) = data.vbank().get(cond).and_then(|v| v.get_def()) {
            let d = data.obank().get(def).unwrap();
            if d.code() != OpCode::CPUI_BOOL_NEGATE {
                break;
            }
            negated = !negated;
            cond = d.get_in(0).unwrap();
        }
        let Some(cmp) = data.vbank().get(cond).and_then(|v| v.get_def()) else { continue };
        for edge in 0..2 {
            let target = graph.block(bl).get_out(edge);
            if !graph.restricted_by_conditional(target, bl) || graph.dominates(target, Some(bl)) {
                continue;
            }
            let value = ((edge == 1) != flip) != negated;
            let Some((mut vn, mut range)) = compare_range(data, cmp, value) else { continue };
            for _ in 0..=CHAIN_DEPTH {
                if range.is_full() || range.is_empty() {
                    break;
                }
                facts.entry(vn).or_default().push(Fact { edge_block: target, range: range.clone() });
                match pull_back_def(data, vn, &range) {
                    Some((input, r)) => (vn, range) = (input, r),
                    None => break,
                }
            }
        }
    }
    facts
}

/// The range `vn` holds throughout `bl`, if any branch bounds it there.
fn range_at(data: &Funcdata, facts: &HashMap<VarnodeId, Vec<Fact>>, vn: VarnodeId, bl: BlockId) -> Option<CircleRange> {
    let mut out: Option<CircleRange> = None;
    for f in facts.get(&vn)? {
        if !data.bblocks_ref().dominates(f.edge_block, Some(bl)) {
            continue;
        }
        match &mut out {
            None => out = Some(f.range.clone()),
            Some(cur) => {
                let mut r = cur.clone();
                if r.intersect(&f.range) == 0 {
                    *cur = r;
                }
            }
        }
    }
    out.filter(|r| !r.is_empty())
}

enum Fold {
    Bool(bool),
    Identity(VarnodeId),
}

fn fold_for(data: &Funcdata, facts: &HashMap<VarnodeId, Vec<Fact>>, op: OpId) -> Option<Fold> {
    let o = data.obank().get(op)?;
    let bl = o.get_parent()?;
    if o.code() == OpCode::CPUI_INT_AND && o.num_input() == 2 {
        let vn = o.get_in(0)?;
        let mask = constant(data, o.get_in(1)?)?;
        if mask == 0 || mask & mask.wrapping_add(1) != 0 {
            return None;
        }
        let size = data.vbank().get(vn)?.get_size();
        let range = range_at(data, facts, vn, bl)?;
        let low = CircleRange::new(0, mask.wrapping_add(1), size, 1);
        return low.contains(&range).then_some(Fold::Identity(vn));
    }
    let (vn, _, _) = const_compare(data, op)?;
    let range = range_at(data, facts, vn, bl)?;
    let (_, when_true) = compare_range(data, op, true)?;
    let (_, when_false) = compare_range(data, op, false)?;
    if when_true.contains(&range) {
        Some(Fold::Bool(true))
    } else if when_false.contains(&range) {
        Some(Fold::Bool(false))
    } else {
        None
    }
}

/// Fold every comparison and mask a dominating branch decides; returns the count.
pub(crate) fn rangefold_apply(data: &mut Funcdata) -> int4 {
    let facts = collect_facts(data);
    if facts.is_empty() {
        return 0;
    }
    let candidates: Vec<OpId> = facts
        .keys()
        .flat_map(|&vn| data.vbank().get(vn).map(|v| v.descend_iter().collect::<Vec<_>>()).unwrap_or_default())
        .collect();
    let mut count = 0;
    for op in candidates {
        let Some(fold) = fold_for(data, &facts, op) else { continue };
        match fold {
            Fold::Bool(value) => {
                let k = data.new_constant(1, value as uintb);
                data.op_remove_input(op, 1);
                data.op_set_input(op, k, 0).expect("rangefold: set constant");
            }
            Fold::Identity(vn) => {
                data.op_remove_input(op, 1);
                data.op_set_input(op, vn, 0).expect("rangefold: set input");
            }
        }
        data.op_set_opcode(op, crate::typeop::type_op_for(OpCode::CPUI_COPY));
        count += 1;
    }
    count
}

/// Runs [`rangefold_apply`] when option `rangefold` is on.  Registered in group
/// `unreachable` with `ActionDeterminedBranch`, which removes the branches it
/// decides, so the reduced `jumptable` pipeline never sees a guard folded away.
pub struct ActionRangeFold {
    base: ActionBase,
}

impl ActionRangeFold {
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionRangeFold { base: ActionBase::new(0, "rangefold", g) })
    }
}

impl Action for ActionRangeFold {
    fn base(&self) -> &ActionBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ActionBase {
        &mut self.base
    }
    fn clone_filtered(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Action>> {
        grouplist
            .contains(self.get_group())
            .then(|| Box::new(ActionRangeFold { base: self.base.clone() }) as Box<dyn Action>)
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        if !data.get_arch().range_fold {
            return 0;
        }
        self.base.count += rangefold_apply(data);
        0
    }
}
