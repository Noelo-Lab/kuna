//! kuna fix `handler_copy_shared_body` (spec 10.16): the body half of
//! `ExceptionDeobfuscator::insert_dummy_exception_handler_blocks`.
//!
//! That pass copies only the first block of a handler with several ranges, so
//! the copies of a catch-any handler share the rest of it: the `finally` body
//! after the exception store, and the rethrow. `FinallyProcessor` takes each
//! catch-all's handler statement for the whole `finally` body; a shared body
//! belongs to no copy's statement, so the copies cannot be processed (and the
//! processing failed on the first copy's store, which has no offsets).
//! [`HandlerCopySharedBody::separate`] gives every live copy its own copy of
//! the blocks after the handler block when only the handler and its copies
//! run them: node splitting of code no other block reaches, with the same
//! successors and protection, so the method does what it did.

use crate::basic_block::BasicBlock;
use crate::control_flow_graph::ControlFlowGraph;
use crate::rt::{JHashMap, JHashSet};

pub struct HandlerCopySharedBody;

impl HandlerCopySharedBody {
    /// `copies` lists each copy of a catch-any handler with the handler it
    /// copies, in the order the pass made them.
    pub fn separate(graph: ControlFlowGraph, copies: &[(BasicBlock, BasicBlock)]) {
        let mut handlers: Vec<BasicBlock> = Vec::new();
        for &(_, handler) in copies {
            if !handlers.contains(&handler) {
                handlers.push(handler);
            }
        }

        for handler in handlers {
            let mut entries: Vec<BasicBlock> = Vec::new();
            if !handler.get_pred_exceptions().is_empty() {
                entries.push(handler);
            }
            for &(copy, original) in copies {
                if original == handler && !copy.get_pred_exceptions().is_empty() {
                    entries.push(copy);
                }
            }
            if entries.len() < 2 {
                continue;
            }

            let Some(body) = Self::private_body(graph, &entries) else {
                continue;
            };
            if body.is_empty() {
                continue;
            }
            for &entry in &entries[1..] {
                Self::give_body(graph, entry, &body);
            }
        }
    }

    /// Whether the handler block `block` has no instruction offsets, as a copy
    /// has none, and the blocks it runs before leaving the method are its own:
    /// the copy holds its whole handler.
    pub fn owns_body(graph: ControlFlowGraph, block: BasicBlock) -> bool {
        block.get_instr_old_offsets().is_empty() && Self::private_body(graph, &[block]).is_some()
    }

    /// The blocks reached from `entries` by regular edges, short of the exit,
    /// if they are entered only from `entries` and from each other, none is a
    /// handler, and every handler protecting one protects each entry.
    fn private_body(graph: ControlFlowGraph, entries: &[BasicBlock]) -> Option<Vec<BasicBlock>> {
        let last = graph.get_last();
        let mut protection: Option<JHashSet<BasicBlock>> = None;
        for &entry in entries {
            if !entry.get_preds().is_empty() {
                return None;
            }
            let handlers = JHashSet::from_collection(entry.get_succ_exceptions().to_vec());
            protection = Some(match protection {
                None => handlers,
                Some(mut common) => {
                    common.retain(|h| handlers.contains(h));
                    common
                }
            });
        }
        let protection = protection?;

        let mut body: Vec<BasicBlock> = Vec::new();
        let mut seen: JHashSet<BasicBlock> = JHashSet::from_collection(entries.to_vec());
        let mut next = 0;
        let mut frontier: Vec<BasicBlock> = entries.to_vec();
        while next < frontier.len() {
            let block = frontier[next];
            next += 1;
            for succ in block.get_succs().iter() {
                if succ == last || seen.contains(&succ) {
                    continue;
                }
                seen.add(succ);
                body.push(succ);
                frontier.push(succ);
            }
        }

        for &block in &body {
            if !block.get_pred_exceptions().is_empty()
                || !block.get_preds().iter().all(|pred| seen.contains(&pred))
                || !block
                    .get_succ_exceptions()
                    .iter()
                    .all(|h| protection.contains(&h))
            {
                return None;
            }
        }
        Some(body)
    }

    /// Points `entry` at its own copy of `body`.
    fn give_body(graph: ControlFlowGraph, entry: BasicBlock, body: &[BasicBlock]) {
        let mut map: JHashMap<BasicBlock, BasicBlock> = JHashMap::new();
        for &block in body {
            let id = graph.with_mut(|d| {
                d.last_id += 1;
                d.last_id
            });
            let clone = block.clone_block(id);
            graph.get_blocks().borrow_mut().add_with_key(clone, id);
            map.put(block, clone);
        }

        for &block in body {
            let clone = map.get_cloned(&block).expect("clone");
            for succ in block.get_succs().to_vec() {
                clone.add_successor(map.get_cloned(&succ).unwrap_or(succ));
            }
            for hd in block.get_succ_exceptions().to_vec() {
                clone.add_successor_exception(hd);
                graph
                    .get_exception_range(hd, block)
                    .expect("NullPointerException")
                    .get_protected_range()
                    .add(clone);
            }
        }

        for succ in entry.get_succs().to_vec() {
            if let Some(clone) = map.get_cloned(&succ) {
                entry.replace_successor(succ, clone);
            }
        }
    }
}
