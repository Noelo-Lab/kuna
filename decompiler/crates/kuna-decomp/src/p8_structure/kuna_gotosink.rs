//! S8 goto sinking and backward-goto loops (kuna `gotosink`).
//!
//! Two print-tree rewrites of a structured list, run after every other
//! structuring pass:
//!
//! * **Sinking.** `if (c) goto L; S1..Sk; L: T` with the `if` and `L` in one
//!   list becomes `if (!c) { S1..Sk } L: T`: the jump skips exactly the
//!   statements between, which is what an `if` around them does.  At most
//!   [`MAX_SUNK`] components move, and the condition must be one basic block,
//!   whose branch is negated (`Funcdata::block_basic_negate_lastop`).
//! * **Backward goto.** `L: S1..Sk; if (c) goto L;` in one list becomes
//!   `do { S1..Sk } while (c);`, and `L: S1..Sk; goto L;` becomes
//!   `while( true ) { S1..Sk }` (the jump at the end is then the loop's own
//!   back edge, so its `BlockGoto` loses its target and prints nothing).  In the infinite
//!   loop every other jump to `L` whose innermost loop is the new one becomes
//!   `continue;`.  A region with a `break;` or `continue;` not enclosed by its
//!   own loop or switch is left alone, because the new loop would capture it.
//!
//! Both rewrites only move list members under a new node; a label no plain
//! goto reaches any more is released.

use crate::action::{Action, ActionBase, ActionContext, ActionGroupList, ApplyResult};
use crate::block::{block_flags, BlockGraph, BlockKind, BlockType};
use crate::context::BlockId;
use crate::funcdata::Funcdata;

/// The most components an `if` sinks over.
const MAX_SUNK: usize = 16;

fn if_goto(graph: &BlockGraph, id: BlockId) -> Option<BlockId> {
    let b = graph.block(id);
    (b.get_type() == BlockType::If && b.get_size() == 1 && b.get_if_goto_type() == block_flags::f_goto_goto)
        .then(|| b.get_if_goto_target())
        .flatten()
}

fn plain_goto(graph: &BlockGraph, id: BlockId) -> Option<BlockId> {
    let b = graph.block(id);
    (b.get_type() == BlockType::Goto && b.get_goto_type() == block_flags::f_goto_goto).then(|| b.get_goto_target()).flatten()
}

/// Is there a `break;` or `continue;` under `nodes` that no loop or switch
/// inside them encloses?
fn free_jumps(graph: &BlockGraph, nodes: &[BlockId]) -> bool {
    let mut stack: Vec<(BlockId, bool, bool)> = nodes.iter().map(|&n| (n, false, false)).collect();
    while let Some((id, in_loop, in_switch)) = stack.pop() {
        let b = graph.block(id);
        let kind = match b.get_type() {
            BlockType::Goto => b.get_goto_type(),
            BlockType::If => b.get_if_goto_type(),
            _ => 0,
        };
        let has_target = b.get_goto_target().is_some() || b.get_if_goto_target().is_some();
        if has_target && kind == block_flags::f_break_goto && !in_loop && !in_switch {
            return true;
        }
        if has_target && kind == block_flags::f_continue_goto && !in_loop {
            return true;
        }
        let (l, s) = match b.get_type() {
            BlockType::WhileDo | BlockType::DoWhile | BlockType::InfLoop => (true, in_switch),
            BlockType::Switch => (in_loop, true),
            _ => (in_loop, in_switch),
        };
        for i in 0..b.get_size() {
            stack.push((b.get_block(i), l, s));
        }
    }
    false
}

fn lists(graph: &BlockGraph, root: BlockId) -> Vec<BlockId> {
    let mut out = Vec::new();
    let mut stack = vec![root];
    while let Some(id) = stack.pop() {
        let b = graph.block(id);
        if matches!(b.get_type(), BlockType::Ls | BlockType::Graph) {
            out.push(id);
        }
        for i in 0..b.get_size() {
            stack.push(b.get_block(i));
        }
    }
    out
}

/// Retag every plain goto to `head` under `loop_id` whose innermost loop is
/// `loop_id` as `continue`.
fn continues_to(graph: &mut BlockGraph, loop_id: BlockId, head: BlockId) {
    let mut stack = vec![loop_id];
    let mut hits = Vec::new();
    while let Some(id) = stack.pop() {
        let b = graph.block(id);
        if id != loop_id && matches!(b.get_type(), BlockType::WhileDo | BlockType::DoWhile | BlockType::InfLoop) {
            continue;
        }
        let target = plain_goto(graph, id).or_else(|| if_goto(graph, id));
        if target.and_then(|t| graph.get_front_leaf(t)) == Some(head) {
            hits.push(id);
        }
        for i in 0..b.get_size() {
            stack.push(b.get_block(i));
        }
    }
    for id in hits {
        match graph.block_mut(id).kind_mut() {
            BlockKind::Goto { gototype, .. } | BlockKind::If { gototype, .. } => *gototype = block_flags::f_continue_goto,
            _ => {}
        }
    }
}

/// One backward-goto loop in `list`, if any; returns the leaf whose label it released.
fn loop_once(data: &mut Funcdata, list: BlockId) -> Option<BlockId> {
    let graph = data.sblocks_ref();
    let comps: Vec<BlockId> = graph.block(list).get_list().to_vec();
    for j in 0..comps.len() {
        let (target, conditional) = match (if_goto(graph, comps[j]), plain_goto(graph, comps[j])) {
            (Some(t), _) => (t, true),
            (None, Some(t)) => (t, false),
            _ => continue,
        };
        let head = graph.get_front_leaf(target)?;
        let Some(i) = comps[..j].iter().position(|&c| graph.get_front_leaf(c) == Some(head)) else { continue };
        if free_jumps(graph, &comps[i..j]) {
            continue;
        }
        if conditional && graph.block(graph.block(comps[j]).get_block(0)).get_type() != BlockType::Copy {
            continue;
        }
        let graph = data.sblocks_mut();
        if conditional {
            let cond = graph.block(comps[j]).get_block(0);
            graph.kuna_set_component(list, j, cond);
            graph.kuna_wrap_components(list, i, j, BlockKind::Ls);
            graph.kuna_wrap_components(list, i, i, BlockKind::DoWhile);
        } else {
            if let BlockKind::Goto { gototarget, .. } = graph.block_mut(comps[j]).kind_mut() {
                *gototarget = None;
            }
            graph.kuna_wrap_components(list, i, j, BlockKind::Ls);
            let inf = graph.kuna_wrap_components(list, i, i, BlockKind::InfLoop);
            continues_to(graph, inf, head);
        }
        return Some(head);
    }
    None
}

/// One sinking in `list`, if any; returns the leaf whose label it released.
fn sink_once(data: &mut Funcdata, list: BlockId) -> Option<BlockId> {
    let graph = data.sblocks_ref();
    let comps: Vec<BlockId> = graph.block(list).get_list().to_vec();
    for i in 0..comps.len() {
        let Some(target) = if_goto(graph, comps[i]) else { continue };
        let head = graph.get_front_leaf(target)?;
        let Some(j) = comps.iter().skip(i + 2).position(|&c| graph.get_front_leaf(c) == Some(head)).map(|p| p + i + 2) else { continue };
        if j - i - 1 > MAX_SUNK {
            continue;
        }
        let cond = graph.block(comps[i]).get_block(0);
        if graph.block(cond).get_type() != BlockType::Copy {
            continue;
        }
        let Some(bb) = graph.block(cond).get_copy() else { continue };
        let graph = data.sblocks_mut();
        if j - i - 1 > 1 {
            graph.kuna_wrap_components(list, i + 1, j - 1, BlockKind::Ls);
        }
        graph.kuna_move_component(list, i + 1, comps[i]);
        if let BlockKind::If { gototarget, gototype } = graph.block_mut(comps[i]).kind_mut() {
            *gototarget = None;
            *gototype = block_flags::f_goto_goto;
        }
        data.block_basic_negate_lastop(bb);
        return Some(head);
    }
    None
}

/// Apply both rewrites to a fixpoint; returns how many fired.
pub(crate) fn gotosink_apply(data: &mut Funcdata) -> i32 {
    let root = data.sblocks_root();
    data.sblocks_mut().kuna_flatten_lists(root);
    let mut count = 0;
    let mut released = Vec::new();
    for _ in 0..64 {
        let mut changed = false;
        for list in lists(data.sblocks_ref(), root) {
            while let Some(t) = loop_once(data, list).or_else(|| sink_once(data, list)) {
                released.push(t);
                count += 1;
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
    if !released.is_empty() {
        release_labels(data, &released);
        // A new loop prints its head's label above itself, as ActionFinalStructure
        // arranges for the loops collapse built.
        data.sblocks_mut().mark_label_bump_up(root, false);
    }
    count
}

/// Clear the label of each leaf in `heads` that no plain goto (not a `break`
/// or `continue`) reaches any more.
fn release_labels(data: &mut Funcdata, heads: &[BlockId]) {
    let graph = data.sblocks_ref();
    let mut still = std::collections::BTreeSet::new();
    let mut stack = vec![data.sblocks_root()];
    while let Some(id) = stack.pop() {
        let b = graph.block(id);
        stack.extend((0..b.get_size()).map(|i| b.get_block(i)));
        still.extend(if_goto(graph, id).or_else(|| plain_goto(graph, id)).and_then(|t| graph.get_front_leaf(t)));
        still.extend(b.switch_caseblocks().iter().filter(|c| c.gototype == block_flags::f_goto_goto).filter_map(|c| graph.get_front_leaf(c.block)));
        if let BlockKind::MultiGoto { gotoedges, .. } = b.kind() {
            still.extend(gotoedges.iter().filter_map(|&t| graph.get_front_leaf(t)));
        }
    }
    for &h in heads {
        if !still.contains(&h) {
            data.sblocks_mut().block_mut(h).clear_flag(block_flags::f_unstructured_targ);
        }
    }
}

/// Runs [`gotosink_apply`] when option `gotosink` is on.
pub struct ActionGotoSink {
    base: ActionBase,
}

impl ActionGotoSink {
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionGotoSink { base: ActionBase::new(0, "gotosink", g) })
    }
}

impl Action for ActionGotoSink {
    fn base(&self) -> &ActionBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ActionBase {
        &mut self.base
    }
    fn clone_filtered(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Action>> {
        grouplist
            .contains(self.get_group())
            .then(|| Box::new(ActionGotoSink { base: self.base.clone() }) as Box<dyn Action>)
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        if !data.get_arch().goto_sink || data.sblocks_get_size() == 0 {
            return 0;
        }
        self.base.count += gotosink_apply(data);
        0
    }
}
