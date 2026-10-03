//! Preserve secondary natural-loop latches before collapse can turn them into
//! nested retry loops. Only dominated, single-entry loops with ordinary branch
//! latches qualify. Head-tested and switch-bodied loops retain their existing
//! structuring, as do loops whose retained latch has a condition or whose body
//! contains another loop head.

use crate::block::{block_flags, BlockGraph, BlockKind, BlockType};
use crate::context::BlockId;

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

/// A continue in an infinite loop resumes its first leaf. Conditional loops
/// reset the scope: their continue can evaluate a test or iterator instead.
pub(crate) fn recover_continues(graph: &mut BlockGraph, root: BlockId) {
    let mut stack = vec![(root, None)];
    while let Some((id, mut head)) = stack.pop() {
        match graph.block(id).get_type() {
            BlockType::InfLoop => head = graph.get_front_leaf(id),
            BlockType::WhileDo | BlockType::DoWhile => head = None,
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
        recover_continues(&mut graph, root);
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
        recover_continues(&mut graph, root);
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
        recover_continues(&mut graph, root);
        assert_eq!(graph.block(restart).get_goto_type(), block_flags::f_goto_goto);
    }
}
