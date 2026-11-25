//! Preserve secondary natural-loop latches before collapse can turn them into
//! nested retry loops. Only dominated, single-entry loops with ordinary branch
//! latches qualify. Head-tested and switch-bodied loops retain their existing
//! structuring, as do loops whose retained latch has a condition or whose body
//! contains another loop head.

use std::collections::BTreeMap;

use crate::block::{block_flags, BlockGraph, BlockKind, BlockType};
use crate::context::BlockId;
use crate::funcdata::Funcdata;
use kuna_base::error::KunaResult;

/// The `loopcontinue` option: `on`, `off`, or `auto` (`on`, and in an
/// unoptimized function the jump statements the binary records keep their
/// tests apart, [`crate::p8_structure::kuna_o0jumps`]).
pub struct OptionLoopContinue;

impl OptionLoopContinue {
    /// Parse `on`/`off`/`auto`, returning whether continues are recovered,
    /// whether the recorded jump statements decide unoptimized functions, and
    /// a confirmation message.
    pub fn apply(&self, p1: &str) -> KunaResult<(bool, bool, String)> {
        if p1 == "auto" {
            return Ok((
                true,
                true,
                "Secondary loop-latch continue recovery turned on, recorded jump statements in unoptimized functions".to_string(),
            ));
        }
        let val = crate::options::on_or_off(p1)?;
        let prop = if val { "on" } else { "off" };
        Ok((val, false, format!("Secondary loop-latch continue recovery turned {prop}")))
    }
}

pub(crate) fn refine_latches(graph: &mut BlockGraph, root: BlockId) {
    let mut edges = Vec::new();
    for i in 0..graph.block(root).get_size() {
        let head = graph.block(root).get_block(i);
        let hb = graph.block(head);
        if hb.is_switch_out() {
            continue;
        }
        let mut latches = Vec::new();
        let mut entries = 0;
        let mut eligible = true;
        for j in 0..hb.size_in() {
            if !hb.is_back_edge_in(j) {
                entries += 1;
                continue;
            }
            let src = hb.get_in(j);
            let sb = graph.block(src);
            if !graph.dominates(head, Some(src)) || sb.is_switch_out() {
                eligible = false;
                break;
            }
            for e in 0..sb.size_out() {
                if sb.get_out(e) == head && sb.is_back_edge_out(e) && !sb.is_goto_out(e) {
                    latches.push((src, e));
                }
            }
        }
        if !eligible || entries != 1 || latches.len() < 2 {
            continue;
        }
        latches.sort_by_key(|&(src, _)| graph.block(src).get_index());
        let primary = latches.pop().unwrap();
        if graph.block(primary.0).size_out() != 1 {
            continue;
        }
        let mut body = std::collections::BTreeSet::from([head]);
        let mut pending: Vec<_> = latches.iter().map(|&(src, _)| src).collect();
        pending.push(primary.0);
        while let Some(node) = pending.pop() {
            if !body.insert(node) {
                continue;
            }
            for j in 0..graph.block(node).size_in() {
                let predecessor = graph.block(node).get_in(j);
                if graph.dominates(head, Some(predecessor)) {
                    pending.push(predecessor);
                }
            }
        }
        if (0..hb.size_out()).any(|e| !body.contains(&hb.get_out(e)))
            || body.iter().any(|&node| graph.block(node).is_switch_out())
            || body.iter().any(|&node| {
                let block = graph.block(node);
                (0..block.size_in()).any(|j| {
                    block.is_irreducible_in(j)
                        || (node != head && !body.contains(&block.get_in(j)))
                }) || (0..block.size_out()).any(|e| block.is_irreducible_out(e))
            })
            || body.iter().any(|&node| {
                node != head
                    && (0..graph.block(node).size_in())
                        .any(|j| graph.block(node).is_back_edge_in(j))
            })
        {
            continue;
        }
        edges.extend(latches);
    }
    for (src, edge) in edges {
        let _ = graph.set_goto_branch(src, edge);
    }
}

/// A continue in an infinite loop resumes its first leaf. A conditional loop
/// resumes the leaf `points` gives it ([`continue_points`]), or resets the
/// scope when it has none: its continue can evaluate a test or iterator instead.
pub(crate) fn recover_continues(graph: &mut BlockGraph, root: BlockId, points: &BTreeMap<BlockId, BlockId>) {
    let mut stack = vec![(root, None)];
    while let Some((id, mut head)) = stack.pop() {
        match graph.block(id).get_type() {
            BlockType::InfLoop => head = graph.get_front_leaf(id),
            BlockType::WhileDo | BlockType::DoWhile => head = points.get(&id).copied(),
            _ => {}
        }
        let target = match graph.block(id).get_type() {
            BlockType::Goto => graph.block(id).get_goto_target(),
            BlockType::If => graph.block(id).get_if_goto_target(),
            _ => None,
        };
        if let (Some(head), Some(target)) = (head, target) {
            if graph.get_front_leaf(target) == Some(head) {
                match graph.block_mut(id).kind_mut() {
                    BlockKind::Goto { gototype, .. } | BlockKind::If { gototype, .. } => {
                        *gototype = block_flags::f_continue_goto;
                    }
                    _ => {}
                }
            }
        }
        for i in 0..graph.block(id).get_size() {
            stack.push((graph.block(id).get_block(i), head));
        }
    }
}

/// The leaf a `continue;` resumes at in each while-do and do-while loop under
/// `root`: a while-do's condition block, unless it prints as a `for` whose
/// iterate statement a `continue;` runs first, and a do-while's last leaf when
/// that leaf prints nothing but the loop test and the output language's
/// do-while runs that test on `continue;`.
pub(crate) fn continue_points(data: &Funcdata, root: BlockId) -> BTreeMap<BlockId, BlockId> {
    let graph = data.sblocks_ref();
    let mut points = BTreeMap::new();
    let mut stack = vec![root];
    while let Some(id) = stack.pop() {
        let block = graph.block(id);
        for i in 0..block.get_size() {
            stack.push(block.get_block(i));
        }
        let point = match block.get_type() {
            BlockType::WhileDo if block.get_iterate_op().is_none() => {
                graph.sub_block(id, 0).and_then(|cond| graph.get_front_leaf(cond))
            }
            BlockType::WhileDo if data.kuna_o0jumps.is_some() => step_leaf(data, id),
            BlockType::DoWhile if data.get_arch().do_while_continue => graph
                .sub_block(id, 0)
                .and_then(|body| graph.get_exit_leaf(body))
                .filter(|&leaf| prints_only_its_branch(data, leaf)),
            _ => None,
        };
        if let Some(leaf) = point {
            points.insert(id, leaf);
        }
    }
    points
}

/// A `for` loop's last leaf when it prints nothing but the loop's iterate
/// statement: a jump to it runs exactly what `continue;` runs.
fn step_leaf(data: &Funcdata, id: BlockId) -> Option<BlockId> {
    let graph = data.sblocks_ref();
    let step = graph.block(id).get_iterate_op()?;
    let leaf = graph.sub_block(id, 1).and_then(|body| graph.get_exit_leaf(body))?;
    let bb = graph.block(leaf).get_copy()?;
    if data.obank().get(step).and_then(|o| o.get_parent()) != Some(bb) {
        return None;
    }
    prints_only(data, leaf, Some(step)).then_some(leaf)
}

/// Does the structured leaf `leaf` print no statement besides its branch?
fn prints_only_its_branch(data: &Funcdata, leaf: BlockId) -> bool {
    prints_only(data, leaf, None)
}

/// Does the structured leaf `leaf` print nothing besides its branch and `except`?
fn prints_only(data: &Funcdata, leaf: BlockId, except: Option<crate::context::OpId>) -> bool {
    let Some(bb) = data.sblocks_ref().block(leaf).get_copy() else { return false };
    let BlockKind::Basic(basic) = data.bblocks_ref().block(bb).kind() else { return false };
    let mut cur = basic.op_head;
    while let Some(op) = cur {
        cur = data.bb_op_next(op);
        let Some(o) = data.obank().get(op) else { return false };
        if o.is_branch() || o.not_printed() || o.is_marker() || Some(op) == except {
            continue;
        }
        let implied = o.get_out().and_then(|v| data.vbank().get(v)).is_some_and(|v| v.is_implied());
        if !implied {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::block::FlowBlock;

    fn leaf(graph: &mut BlockGraph, root: BlockId) -> BlockId {
        let basic = graph.arena.insert(FlowBlock::new());
        graph.new_block_copy(root, basic)
    }

    fn jump(graph: &mut BlockGraph, root: BlockId, target: BlockId) -> BlockId {
        let source = leaf(graph, root);
        graph.add_edge(source, target);
        graph.new_block_goto(root, source)
    }

    #[test]
    fn only_unbounded_loops_without_nested_cycles_are_refined() {
        for variant in ["retry", "head-test", "latch-test", "nested", "switch", "multi-entry", "irreducible"] {
            let mut graph = BlockGraph::new();
            let root = graph.arena.insert(FlowBlock::new_kind(BlockKind::Graph));
            let entry = graph.new_block(root);
            let head = graph.new_block(root);
            let condition = graph.new_block(root);
            let retry = graph.new_block(root);
            let tail = graph.new_block(root);
            let exit = graph.new_block(root);
            graph.add_edge(entry, head);
            graph.add_edge(head, condition);
            graph.add_edge(condition, retry);
            graph.add_edge(condition, tail);
            graph.add_edge(retry, head);
            graph.add_edge(tail, head);
            match variant {
                "head-test" => graph.add_edge(head, exit),
                "latch-test" => graph.add_edge(tail, exit),
                "nested" => graph.add_edge(condition, condition),
                "switch" => graph.block_mut(condition).set_flag(block_flags::f_switch_out),
                "multi-entry" => {
                    let extra = graph.new_block(root);
                    graph.add_edge(entry, extra);
                    graph.add_edge(extra, head);
                }
                _ => {}
            }
            graph.set_start_block(root, entry);
            let mut roots = Vec::new();
            graph.structure_loops(root, &mut roots);
            graph.calc_forward_dominator(root, &roots);
            if variant == "irreducible" {
                graph.set_out_edge_flag(condition, 0, crate::block::edge_flags::f_irreducible);
            }
            for (index, id) in [entry, head, condition, retry, tail, exit].into_iter().enumerate() {
                graph.block_mut(id).set_index(index as i32);
            }
            refine_latches(&mut graph, root);
            assert_eq!(graph.block(retry).is_goto_out(0), variant == "retry", "{variant}");
            assert!(!graph.block(tail).is_goto_out(0), "{variant}");
        }
    }

    #[test]
    fn continue_requires_the_infinite_loop_head() {
        let mut graph = BlockGraph::new();
        let root = graph.arena.insert(FlowBlock::new_kind(BlockKind::Graph));
        let head = leaf(&mut graph, root);
        let middle = leaf(&mut graph, root);
        let restart = jump(&mut graph, root, head);
        let skip = jump(&mut graph, root, middle);
        let body = graph.new_block_list(root, &[head, middle, restart, skip]).unwrap();
        graph.new_block_inf_loop(root, body);
        recover_continues(&mut graph, root, &BTreeMap::new());
        assert_eq!(graph.block(restart).get_goto_type(), block_flags::f_continue_goto);
        assert_eq!(graph.block(skip).get_goto_type(), block_flags::f_goto_goto);
    }

    #[test]
    fn nested_loop_does_not_continue_the_outer_loop() {
        let mut graph = BlockGraph::new();
        let root = graph.arena.insert(FlowBlock::new_kind(BlockKind::Graph));
        let outer_head = leaf(&mut graph, root);
        let inner_head = leaf(&mut graph, root);
        let outer_jump = jump(&mut graph, root, outer_head);
        let inner_jump = jump(&mut graph, root, inner_head);
        let inner_body = graph.new_block_list(root, &[inner_head, outer_jump, inner_jump]).unwrap();
        let inner = graph.new_block_inf_loop(root, inner_body);
        let outer_body = graph.new_block_list(root, &[outer_head, inner]).unwrap();
        graph.new_block_inf_loop(root, outer_body);
        recover_continues(&mut graph, root, &BTreeMap::new());
        assert_eq!(graph.block(outer_jump).get_goto_type(), block_flags::f_goto_goto);
        assert_eq!(graph.block(inner_jump).get_goto_type(), block_flags::f_continue_goto);
    }

    #[test]
    fn conditional_loop_head_jump_keeps_its_condition_semantics() {
        let mut graph = BlockGraph::new();
        let root = graph.arena.insert(FlowBlock::new_kind(BlockKind::Graph));
        let head = leaf(&mut graph, root);
        let restart = jump(&mut graph, root, head);
        let body = graph.new_block_list(root, &[head, restart]).unwrap();
        graph.new_block_do_while(root, body);
        recover_continues(&mut graph, root, &BTreeMap::new());
        assert_eq!(graph.block(restart).get_goto_type(), block_flags::f_goto_goto);
    }
}
