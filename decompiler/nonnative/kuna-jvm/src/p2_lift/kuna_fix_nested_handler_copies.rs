//! kuna fix `nested_handler_copies` (spec 10.16): the nesting half of
//! `ExceptionDeobfuscator::insert_dummy_exception_handler_blocks`.
//!
//! That pass gives every range of a handler with several ranges its own copy of
//! the handler block and puts each copy into the ranges that protect the
//! handler, under the handler those ranges have. When the handler is itself
//! protected, as the `catch` of a `try`/`catch`/`finally` is by the `finally`'s
//! handler, and that outer handler was duplicated too, a catch and the try body
//! it serves end up under different copies of the outer handler, and
//! `DomHelper` can never build the statement: the method fails with
//! `parsing failure!`. [`NestedHandlerCopies::nest`] restores the nesting. An
//! outer handler whose ranges together have a single entry did not need its
//! copies, so its ranges go back to the handler itself; otherwise each catch
//! moves to the copy of the outer handler that protects its try body, and every
//! copy gets its own copy of the handler's further blocks, which the copies
//! otherwise share and the finally processing would edit once per copy. Copies
//! run the same code with the same successors, so none of this alters what the
//! method does.

use crate::basic_block::BasicBlock;
use crate::control_flow_graph::ControlFlowGraph;
use crate::dead_code_helper::DeadCodeHelper;
use crate::exception_range_cfg::ExceptionRangeCFG;
use crate::rt::{JHashMap, JHashSet};

pub struct NestedHandlerCopies;

/// A handler whose protection by `outer` disagrees with its try body's, which
/// `target`, a copy of the same handler as `outer`, protects.
struct Mismatch {
    handler: BasicBlock,
    body: Vec<BasicBlock>,
    outer: BasicBlock,
    target: BasicBlock,
}

impl NestedHandlerCopies {
    /// `copies` maps each handler copy made by the pass to the handler it copies.
    /// Whether a nesting needed repair.
    pub fn nest(graph: ControlFlowGraph, copies: &JHashMap<BasicBlock, BasicBlock>) -> bool {
        let original = |block: BasicBlock| copies.get_cloned(&block).unwrap_or(block);

        let mut families: Vec<BasicBlock> = Vec::new();
        for mismatch in Self::mismatches(graph, &original) {
            let family = original(mismatch.outer);
            if !families.contains(&family) {
                families.push(family);
            }
        }
        if families.is_empty() {
            return false;
        }

        for family in families {
            Self::merge_if_single_entry(graph, family, &original);
        }

        for mismatch in Self::mismatches(graph, &original) {
            let Some(outer_range) = graph.get_exception_range(mismatch.outer, mismatch.handler)
            else {
                continue;
            };
            let Some(catch_body) = Self::private_catch_body(mismatch.handler, outer_range) else {
                continue;
            };
            if catch_body.iter().any(|block| {
                mismatch.body.contains(block)
                    || *block == mismatch.target
                    || block.get_succ_exceptions().contains(&mismatch.target)
            }) {
                continue;
            }
            let Some(target_range) = graph.get_exception_range(mismatch.target, mismatch.body[0])
            else {
                continue;
            };

            for block in catch_body {
                outer_range.get_protected_range().remove_item(&block);
                Self::replace_handler(block, mismatch.outer, mismatch.target);
                target_range.get_protected_range().add(block);
            }
        }

        let exceptions = graph.get_exceptions();
        for i in (0..exceptions.len()).rev() {
            if exceptions.get(i).get_protected_range().is_empty() {
                exceptions.remove_at(i);
            }
        }
        DeadCodeHelper::remove_dead_blocks(graph);
        true
    }

    fn mismatches(
        graph: ControlFlowGraph,
        original: &impl Fn(BasicBlock) -> BasicBlock,
    ) -> Vec<Mismatch> {
        let mut res = Vec::new();
        for range in graph.get_exceptions().to_vec() {
            let handler = range.get_handler();
            let body = range.get_protected_range().to_vec();
            if body.is_empty() || !handler.get_preds().is_empty() {
                continue;
            }

            for outer in handler.get_succ_exceptions().to_vec() {
                let Some(outer_range) = graph.get_exception_range(outer, handler) else {
                    continue;
                };
                let types = outer_range.get_unique_exceptions_string();
                if let Some(target) =
                    Self::body_handler(graph, &body, original(outer), &types, original)
                {
                    if target != outer {
                        res.push(Mismatch {
                            handler,
                            body: body.clone(),
                            outer,
                            target,
                        });
                    }
                }
            }
        }
        res
    }

    /// The one handler copied from `family` that protects every block of `body`
    /// against `types`.
    fn body_handler(
        graph: ControlFlowGraph,
        body: &[BasicBlock],
        family: BasicBlock,
        types: &Option<String>,
        original: &impl Fn(BasicBlock) -> BasicBlock,
    ) -> Option<BasicBlock> {
        let mut common: Option<Vec<BasicBlock>> = None;
        for &block in body {
            let here: Vec<BasicBlock> = block
                .get_succ_exceptions()
                .iter()
                .filter(|&h| {
                    original(h) == family
                        && graph
                            .get_exception_range(h, block)
                            .is_some_and(|r| r.get_unique_exceptions_string() == *types)
                })
                .collect();
            common = Some(match common {
                None => here,
                Some(c) => c.into_iter().filter(|h| here.contains(h)).collect(),
            });
        }
        match common?.as_slice() {
            [only] => Some(*only),
            _ => None,
        }
    }

    /// Hands every range of `family`'s copies back to `family` when the ranges
    /// together are entered at a single block.
    fn merge_if_single_entry(
        graph: ControlFlowGraph,
        family: BasicBlock,
        original: &impl Fn(BasicBlock) -> BasicBlock,
    ) {
        let ranges: Vec<ExceptionRangeCFG> = graph
            .get_exceptions()
            .iter()
            .filter(|r| original(r.get_handler()) == family)
            .collect();
        if !ranges.iter().any(|r| r.get_handler() == family) {
            return;
        }

        let mut union: JHashSet<BasicBlock> = JHashSet::new();
        for range in &ranges {
            for block in range.get_protected_range().iter() {
                if !union.add(block) {
                    return;
                }
            }
        }

        let first = graph.get_first();
        let entries = union
            .iter()
            .filter(|&&block| {
                block == first || block.get_preds().iter().any(|pred| !union.contains(&pred))
            })
            .count();
        if entries > 1 {
            return;
        }

        for range in ranges {
            let copy = range.get_handler();
            if copy == family {
                continue;
            }
            for block in range.get_protected_range().iter() {
                Self::replace_handler(block, copy, family);
            }
            range.set_handler(family);
        }
    }

    /// Gives every live copy of a handler its own copy of the blocks after the
    /// handler block that only the handler and its copies run, so that no two
    /// copies share code the finally processing edits per handler.
    pub fn separate_handler_bodies(
        graph: ControlFlowGraph,
        copies: &JHashMap<BasicBlock, BasicBlock>,
    ) {
        let mut handlers: Vec<BasicBlock> = Vec::new();
        for (_, handler) in copies.entry_vec() {
            if !handlers.contains(&handler) {
                handlers.push(handler);
            }
        }

        for handler in handlers {
            let live: Vec<BasicBlock> = copies
                .entry_vec()
                .into_iter()
                .filter(|(copy, h)| *h == handler && !copy.get_pred_exceptions().is_empty())
                .map(|(copy, _)| copy)
                .collect();
            if live.is_empty() {
                continue;
            }
            let mut entries = live.clone();
            entries.push(handler);
            let body = Self::shared_body(graph, handler, &entries);
            if body.len() == 1 {
                continue;
            }

            for copy in live {
                let mut map: Vec<(BasicBlock, BasicBlock)> = vec![(handler, copy)];
                for &block in &body[1..] {
                    let id = graph.with_mut(|d| {
                        d.last_id += 1;
                        d.last_id
                    });
                    let clone = block.clone_block(id);
                    graph.get_blocks().borrow_mut().add_with_key(clone, id);
                    map.push((block, clone));
                }
                let mapped =
                    |b: BasicBlock| map.iter().find(|(from, _)| *from == b).map(|(_, to)| *to);

                for succ in copy.get_succs().to_vec() {
                    if let Some(to) = mapped(succ) {
                        copy.replace_successor(succ, to);
                    }
                }
                for &(block, clone) in &map[1..] {
                    for succ in block.get_succs().to_vec() {
                        clone.add_successor(mapped(succ).unwrap_or(succ));
                    }
                    for hd in block.get_succ_exceptions().to_vec() {
                        clone.add_successor_exception(hd);
                        if let Some(range) = graph.get_exception_range(hd, block) {
                            range.get_protected_range().add(clone);
                        }
                    }
                }
            }
        }
    }

    /// `handler` and the blocks after it that only the blocks of `entries`
    /// and of the body itself lead to.
    fn shared_body(
        graph: ControlFlowGraph,
        handler: BasicBlock,
        entries: &[BasicBlock],
    ) -> Vec<BasicBlock> {
        let last = graph.get_last();
        let mut body = vec![handler];
        loop {
            let mut grown = false;
            for i in 0..body.len() {
                for succ in body[i].get_succs().to_vec() {
                    if succ == last
                        || body.contains(&succ)
                        || !succ.get_pred_exceptions().is_empty()
                        || !succ
                            .get_preds()
                            .iter()
                            .all(|p| body.contains(&p) || entries.contains(&p))
                    {
                        continue;
                    }
                    body.push(succ);
                    grown = true;
                }
            }
            if !grown {
                break;
            }
        }
        body
    }

    /// The blocks of `outer_range` the handler's code runs through, when no
    /// other block enters them.
    fn private_catch_body(
        handler: BasicBlock,
        outer_range: ExceptionRangeCFG,
    ) -> Option<Vec<BasicBlock>> {
        let range = outer_range.get_protected_range();
        let mut blocks = vec![handler];
        let mut i = 0;
        while i < blocks.len() {
            let mut next = blocks[i].get_succs().to_vec();
            next.extend(blocks[i].get_succ_exceptions().to_vec());
            for succ in next {
                if range.contains(&succ) && !blocks.contains(&succ) {
                    blocks.push(succ);
                }
            }
            i += 1;
        }

        let entered_from_outside = blocks[1..].iter().any(|block| {
            block
                .get_preds()
                .iter()
                .chain(block.get_pred_exceptions().iter())
                .any(|pred| !blocks.contains(&pred))
        });
        (!entered_from_outside).then_some(blocks)
    }

    /// Replaces `old` by `new` among the exception successors of `block`, in place.
    fn replace_handler(block: BasicBlock, old: BasicBlock, new: BasicBlock) {
        let succs = block.get_succ_exceptions();
        for i in 0..succs.len() {
            if succs.get(i) == old {
                succs.set(i, new);
            }
        }
        old.remove_predecessor_exception(block);
        new.add_predecessor_exception(block);
    }
}
