//! Keep statements out of condition slots.
//!
//! An `if`, a loop and a short-circuit condition print their condition block
//! twice: once for its statements, once for the branch that ends it. A block
//! that does not end in a two-way branch (a folded switch, a loop, an `if` with
//! a body) has no branch to print, and printing it again where the condition
//! belongs gives C such as `if switch (x) { ... }`. The region structurer folds
//! only blocks that pass [`renders_branch`] into a condition slot, refuses a
//! finished tree that [`misplaced_condition`] rejects (the caller then falls
//! back to `CollapseStructure`), and the printer writes a warning in place of
//! any such block it is still handed.

use crate::block::{BlockGraph, BlockType};
use crate::context::BlockId;

pub(crate) const MISPLACED_CONDITION_WARNING: &str =
    "WARNING: structuring placed a statement where a condition belongs";

/// False when `bl`, printed where a condition belongs, would print a statement:
/// a switch, an `if`, a loop or a goto, at the end of any nesting of lists.
pub(crate) fn renders_branch(graph: &BlockGraph, bl: BlockId) -> bool {
    let mut b = bl;
    loop {
        let node = graph.block(b);
        match node.get_type() {
            BlockType::Ls | BlockType::MultiGoto => match node.get_list().last() {
                Some(&last) => b = last,
                None => return false,
            },
            BlockType::Switch
            | BlockType::If
            | BlockType::WhileDo
            | BlockType::DoWhile
            | BlockType::InfLoop
            | BlockType::Goto => return false,
            _ => return true,
        }
    }
}

/// The first `if`, loop or short-circuit node under `root` whose condition slot
/// holds a block that does not end in a two-way branch.
pub(crate) fn misplaced_condition(graph: &BlockGraph, root: BlockId) -> Option<BlockId> {
    let mut stack = vec![root];
    while let Some(b) = stack.pop() {
        let node = graph.block(b);
        let slots = match node.get_type() {
            BlockType::If | BlockType::WhileDo | BlockType::DoWhile => 1,
            BlockType::Condition => 2,
            _ => 0,
        };
        if (0..slots.min(node.get_size())).any(|i| !renders_branch(graph, node.get_block(i))) {
            return Some(b);
        }
        stack.extend(node.get_list().iter().copied());
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::block::{BlockKind, FlowBlock};

    fn graph(n: usize, edges: &[(usize, usize)]) -> (BlockGraph, BlockId, Vec<BlockId>) {
        let mut g = BlockGraph::new();
        let root = g.arena.insert(FlowBlock::new_kind(BlockKind::Graph));
        g.root = Some(root);
        let b: Vec<BlockId> = (0..n).map(|_| g.new_block(root)).collect();
        for &block in &b {
            let original = g.arena.insert(FlowBlock::new_kind(BlockKind::Graph));
            *g.block_mut(block).kind_mut() = BlockKind::Copy { copy: Some(original) };
        }
        for &(s, d) in edges {
            g.add_edge(b[s], b[d]);
        }
        (g, root, b)
    }

    #[test]
    fn only_a_block_ending_in_a_branch_is_a_condition() {
        let (mut g, root, b) = graph(4, &[(0, 1), (1, 2), (1, 3), (2, 3)]);
        assert!(renders_branch(&g, b[1]));
        let ls = g.new_block_list(root, &[b[0], b[1]]).unwrap();
        assert!(renders_branch(&g, ls));
        let (mut g, root, b) = graph(4, &[(0, 1), (1, 2), (1, 3), (2, 3)]);
        let ifb = g.new_block_if(root, b[1], b[2]);
        assert!(!renders_branch(&g, ifb));
        let ls = g.new_block_list(root, &[b[0], ifb]).unwrap();
        assert!(!renders_branch(&g, ls));
    }

    #[test]
    fn a_loop_in_a_condition_slot_is_found() {
        let (mut g, root, b) = graph(3, &[(0, 1), (1, 0), (0, 2)]);
        let w = g.new_block_while_do(root, b[0], b[1]);
        assert_eq!(misplaced_condition(&g, root), None);
        let bad = g.new_block_if(root, w, b[2]);
        assert_eq!(misplaced_condition(&g, root), Some(bad));
        let (mut g, root, b) = graph(3, &[(0, 1), (0, 2), (1, 2)]);
        g.new_block_if(root, b[0], b[1]);
        assert_eq!(misplaced_condition(&g, root), None);
    }
}
