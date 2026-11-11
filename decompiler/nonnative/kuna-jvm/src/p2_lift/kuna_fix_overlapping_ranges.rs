//! kuna fix `overlapping_ranges` (spec 10.16): protected ranges that overlap
//! without nesting.
//!
//! `DomHelper` builds a `try` from the blocks a handler protects, so the ranges
//! of different handlers must nest. kotlinc 1.3 breaks that: the range of a
//! `catch` inside a `try`/`finally` (or `use`) also covers the `finally` copy
//! inlined before a `return` of the `try`, which the `finally`'s own ranges
//! leave out, while those ranges protect the `catch`'s handler. The method
//! fails with `parsing failure!`.
//!
//! [`OverlappingRanges::split`] gives the part of such an inner range that its
//! handler's protectors leave out its own copy of the handler, then splits the
//! ranges of every handler protecting it, innermost first, into pieces that
//! are entered at one block, each with its own copy of the handler. A copy
//! takes the blocks only its original runs (unprotected, entered from nowhere
//! else) along: exact node splitting. Before a split, blocks of `nop`s leave
//! the ranges: they cannot throw. Every block keeps the handlers it had, so the
//! method does what it did. `FinallyProcessor` then works per piece; in such a
//! method it refuses a `finally` copy that another path also runs and a `try`
//! re-entered at its head from inside (see `finally_processor.rs`).

use crate::basic_block::BasicBlock;
use crate::code_constants::{OPC_ASTORE, OPC_IRETURN, OPC_NOP, OPC_POP, OPC_RETURN};
use crate::control_flow_graph::ControlFlowGraph;
use crate::exception_range_cfg::ExceptionRangeCFG;
use crate::instruction_sequence::InstructionSequence;
use crate::rt::{JHashMap, JHashSet, JList};

pub struct OverlappingRanges;

impl OverlappingRanges {
    /// Splits the ranges of `graph` when one overlaps a range of a handler
    /// that protects its handler without nesting in it; whether it did.
    pub fn split(graph: ControlFlowGraph) -> bool {
        if graph.get_exceptions().len() < 2 {
            return false;
        }
        let handlers = Self::handlers(graph);
        let covers: Vec<JHashSet<BasicBlock>> = handlers
            .iter()
            .map(|&h| JHashSet::from_collection(Self::cover(graph, h)))
            .collect();
        let protectors = |handler: BasicBlock| -> Vec<usize> {
            (0..handlers.len())
                .filter(|&i| handlers[i] != handler && covers[i].contains(&handler))
                .collect()
        };

        let mut targets: Vec<(BasicBlock, Vec<BasicBlock>)> = Vec::new();
        for &handler in &handlers {
            let around = protectors(handler);
            if around.is_empty() {
                continue;
            }
            let mut inside = false;
            let mut outside: Vec<BasicBlock> = Vec::new();
            for block in Self::cover(graph, handler) {
                match around
                    .iter()
                    .filter(|&&i| covers[i].contains(&block))
                    .count()
                {
                    0 => outside.push(block),
                    n if n == around.len() => inside = true,
                    _ => return false,
                }
            }
            if inside && !outside.is_empty() {
                let seq = handler.get_seq();
                if seq.is_empty() || ![OPC_ASTORE, OPC_POP].contains(&seq.get_instr(0).opcode()) {
                    return false;
                }
                targets.push((handler, outside));
            }
        }
        if targets.is_empty() {
            return false;
        }

        let mut dirty: Vec<BasicBlock> = Vec::new();
        let mut next = 0;
        for (handler, _) in &targets {
            for i in protectors(*handler) {
                if !dirty.contains(&handlers[i]) {
                    dirty.push(handlers[i]);
                }
            }
        }
        while next < dirty.len() {
            for i in protectors(dirty[next]) {
                if !dirty.contains(&handlers[i]) {
                    dirty.push(handlers[i]);
                }
            }
            next += 1;
        }
        let protected_by: Vec<Vec<BasicBlock>> = dirty
            .iter()
            .map(|&h| protectors(h).into_iter().map(|i| handlers[i]).collect())
            .collect();
        let mut order: Vec<BasicBlock> = Vec::new();
        while order.len() < dirty.len() {
            let pick = (0..dirty.len()).find(|&i| {
                !order.contains(&dirty[i])
                    && (0..dirty.len()).all(|j| {
                        j == i || order.contains(&dirty[j]) || !protected_by[j].contains(&dirty[i])
                    })
            });
            match pick {
                Some(i) => order.push(dirty[i]),
                None => return false,
            }
        }

        if targets
            .iter()
            .map(|(h, _)| *h)
            .chain(order.iter().copied())
            .any(|h| Self::catch_any(graph, h) && !Self::owns_body(graph, h))
        {
            return false;
        }

        for (handler, outside) in &targets {
            let copy = Self::copy_handler(graph, *handler);
            Self::detach_store(graph, copy);
            Self::move_blocks(graph, *handler, copy, outside);
        }
        for handler in order {
            Self::split_entries(graph, handler);
        }
        graph.with_mut(|d| d.kuna_overlapping_ranges = true);
        true
    }

    /// Whether `split` changed the ranges of `graph`.
    pub fn split_method(graph: ControlFlowGraph) -> bool {
        graph.with(|d| d.kuna_overlapping_ranges)
    }

    /// Whether, in a split method, a `try` whose blocks are `try_blocks` and
    /// whose first block is `head` must stay a catch-all: an edge from inside
    /// it enters its head (the loop around it would leave it there, which runs
    /// a `finally`), or it returns, which runs a `finally` the method did not.
    pub fn keeps_catch_all(try_blocks: &[BasicBlock], head: BasicBlock) -> bool {
        head.get_preds().iter().any(|p| try_blocks.contains(&p))
            || try_blocks.iter().any(|b| {
                b.get_last_instruction()
                    .is_some_and(|i| (OPC_IRETURN..=OPC_RETURN).contains(&i.opcode()))
            })
    }

    /// Whether, in a split method, the `finally` copy `area` after a `try`
    /// whose blocks are `try_blocks` is also entered from elsewhere, so that
    /// removing it would remove it from another path.
    pub fn shared_copy(area: &JHashSet<BasicBlock>, try_blocks: &[BasicBlock]) -> bool {
        area.iter().any(|b| {
            b.get_preds()
                .iter()
                .chain(b.get_pred_exceptions().iter())
                .any(|p| !area.contains(&p) && !try_blocks.contains(&p))
        })
    }

    /// Whether every range of `handler` catches any exception.
    fn catch_any(graph: ControlFlowGraph, handler: BasicBlock) -> bool {
        graph
            .get_exceptions()
            .iter()
            .filter(|r| r.get_handler() == handler)
            .all(|r| r.get_exception_types().is_none())
    }

    /// Whether every block `handler` reaches by regular edges, short of the
    /// exit, is in its private body, so that a copy owns all of its code.
    fn owns_body(graph: ControlFlowGraph, handler: BasicBlock) -> bool {
        let last = graph.get_last();
        let body = Self::private_body(graph, handler);
        let mut seen: Vec<BasicBlock> = vec![handler];
        let mut i = 0;
        while i < seen.len() {
            for succ in seen[i].get_succs().to_vec() {
                if succ != last && !seen.contains(&succ) {
                    if !body.contains(&succ) {
                        return false;
                    }
                    seen.push(succ);
                }
            }
            i += 1;
        }
        true
    }

    /// The handlers of `graph`, in range order.
    fn handlers(graph: ControlFlowGraph) -> Vec<BasicBlock> {
        let mut res: Vec<BasicBlock> = Vec::new();
        for range in graph.get_exceptions().iter() {
            let handler = range.get_handler();
            if !res.contains(&handler) {
                res.push(handler);
            }
        }
        res
    }

    /// The blocks the ranges of `handler` protect, in range order.
    fn cover(graph: ControlFlowGraph, handler: BasicBlock) -> Vec<BasicBlock> {
        let mut seen: JHashSet<BasicBlock> = JHashSet::new();
        let mut blocks: Vec<BasicBlock> = Vec::new();
        for range in graph.get_exceptions().iter() {
            if range.get_handler() == handler {
                for block in range.get_protected_range().iter() {
                    if seen.add(block) {
                        blocks.push(block);
                    }
                }
            }
        }
        blocks
    }

    /// Moves `blocks` from the ranges of `handler` to ranges of the same
    /// exception types of `copy`, each next to the range it leaves.
    fn move_blocks(
        graph: ControlFlowGraph,
        handler: BasicBlock,
        copy: BasicBlock,
        blocks: &[BasicBlock],
    ) {
        let exceptions = graph.get_exceptions();
        for range in exceptions.to_vec() {
            if range.get_handler() != handler {
                continue;
            }
            let protected = range.get_protected_range();
            let part: Vec<BasicBlock> = protected.iter().filter(|b| blocks.contains(b)).collect();
            if part.is_empty() {
                continue;
            }
            protected.borrow_mut().retain(|b| !part.contains(b));
            for &block in &part {
                Self::replace_handler(block, handler, copy);
            }
            let at = exceptions.index_of(&range) as usize + 1;
            exceptions.add_at(
                at,
                ExceptionRangeCFG::new(JList::from(part), copy, range.get_exception_types()),
            );
            if protected.is_empty() {
                exceptions.remove_item(&range);
            }
        }
    }

    /// Splits the ranges of `handler` into pieces entered at one block each,
    /// every piece after the first with its own copy of the handler.
    fn split_entries(graph: ControlFlowGraph, handler: BasicBlock) {
        if Self::entries(graph, handler).len() < 2 {
            return;
        }
        let protected_handlers: Vec<BasicBlock> = Self::cover(graph, handler)
            .into_iter()
            .filter(|b| !b.get_pred_exceptions().is_empty())
            .collect();
        let exceptions = graph.get_exceptions();
        for range in exceptions.to_vec() {
            if range.get_handler() != handler {
                continue;
            }
            let protected = range.get_protected_range();
            for block in protected.to_vec() {
                if Self::cannot_throw(block)
                    && block
                        .get_succ_exceptions()
                        .iter()
                        .all(|h| !protected_handlers.contains(&h))
                {
                    protected.remove_item(&block);
                    if !Self::cover(graph, handler).contains(&block) {
                        block.remove_successor_exception(handler);
                    }
                }
            }
            if protected.is_empty() {
                exceptions.remove_item(&range);
            }
        }

        let entries = Self::entries(graph, handler);
        if entries.len() < 2 {
            return;
        }
        let union = Self::cover(graph, handler);
        let in_union = JHashSet::from_collection(union.clone());
        let preds = |b: BasicBlock| -> Vec<BasicBlock> {
            let mut all = b.get_preds().to_vec();
            all.extend(b.get_pred_exceptions().to_vec());
            all
        };

        let mut seeds = entries;
        let owner = loop {
            let mut owner: JHashMap<BasicBlock, BasicBlock> = JHashMap::new();
            let mut queue: Vec<BasicBlock> = Vec::new();
            for &seed in &seeds {
                owner.put(seed, seed);
                queue.push(seed);
            }
            let mut next = 0;
            while next < queue.len() {
                let block = queue[next];
                next += 1;
                let own = owner.get_cloned(&block).expect("owner");
                let mut succs = block.get_succs().to_vec();
                succs.extend(block.get_succ_exceptions().to_vec());
                for succ in succs {
                    if in_union.contains(&succ) && owner.get_cloned(&succ).is_none() {
                        owner.put(succ, own);
                        queue.push(succ);
                    }
                }
            }

            let mut grown = false;
            for &block in &union {
                if seeds.contains(&block) {
                    continue;
                }
                let own = owner.get_cloned(&block);
                if own.is_none()
                    || preds(block)
                        .iter()
                        .any(|p| in_union.contains(p) && owner.get_cloned(p) != own)
                {
                    seeds.push(block);
                    grown = true;
                }
            }
            if !grown {
                break owner;
            }
        };

        let keep = owner.get_cloned(&union[0]).expect("owner");
        let mut heads: Vec<BasicBlock> = Vec::new();
        let mut pieces: Vec<Vec<BasicBlock>> = Vec::new();
        for &block in &union {
            let own = owner.get_cloned(&block).expect("owner");
            if own == keep {
                continue;
            }
            match heads.iter().position(|&h| h == own) {
                Some(i) => pieces[i].push(block),
                None => {
                    heads.push(own);
                    pieces.push(vec![block]);
                }
            }
        }
        for piece in pieces {
            let copy = Self::copy_handler(graph, handler);
            Self::move_blocks(graph, handler, copy, &piece);
        }
    }

    /// The blocks the ranges of `handler` protect that are entered from
    /// outside them, by a regular or an exception edge, or start the method.
    fn entries(graph: ControlFlowGraph, handler: BasicBlock) -> Vec<BasicBlock> {
        let union = Self::cover(graph, handler);
        let in_union = JHashSet::from_collection(union.clone());
        let first = graph.get_first();
        union
            .into_iter()
            .filter(|&b| {
                b == first
                    || b.get_preds().iter().any(|p| !in_union.contains(&p))
                    || b.get_pred_exceptions()
                        .iter()
                        .any(|p| !in_union.contains(&p))
            })
            .collect()
    }

    /// Whether `block` holds only `nop`s, which cannot throw.
    fn cannot_throw(block: BasicBlock) -> bool {
        let seq = block.get_seq();
        seq.length() > 0
            && (0..seq.length()).all(|i| seq.get_instr(i).opcode() == OPC_NOP)
            && block.get_pred_exceptions().is_empty()
    }

    /// A copy of the handler block `handler`, protected as it is, with copies
    /// of the blocks only it runs.
    fn copy_handler(graph: ControlFlowGraph, handler: BasicBlock) -> BasicBlock {
        let body = Self::private_body(graph, handler);
        let mut map: JHashMap<BasicBlock, BasicBlock> = JHashMap::new();
        let mut originals = vec![handler];
        originals.extend(body);
        for &block in &originals {
            let id = graph.with_mut(|d| {
                d.last_id += 1;
                d.last_id
            });
            let clone = block.clone_block(id);
            graph.get_blocks().borrow_mut().add_with_key(clone, id);
            map.put(block, clone);
        }
        for &block in &originals {
            let clone = map.get_cloned(&block).expect("clone");
            for succ in block.get_succs().to_vec() {
                clone.add_successor(map.get_cloned(&succ).unwrap_or(succ));
            }
            for hd in block.get_succ_exceptions().to_vec() {
                clone.add_successor_exception(hd);
                for range in graph.get_exceptions().iter() {
                    if range.get_handler() == hd && range.get_protected_range().contains(&block) {
                        range.get_protected_range().add(clone);
                    }
                }
            }
        }
        map.get_cloned(&handler).expect("clone")
    }

    /// Moves the protection of the handler copy `block` past its first
    /// instruction, the store of the exception, which cannot throw: the copy's
    /// `catch` then holds the `try`s of the pieces protecting it.
    fn detach_store(graph: ControlFlowGraph, block: BasicBlock) {
        let handlers = block.get_succ_exceptions().to_vec();
        if handlers.is_empty() {
            return;
        }
        let seq = block.get_seq();
        if seq.length() > 1 {
            let id = graph.with_mut(|d| {
                d.last_id += 1;
                d.last_id
            });
            let rest = BasicBlock::new(id);
            let rest_seq = InstructionSequence::new();
            for i in 1..seq.length() {
                rest_seq.add_instruction(seq.get_instr(i));
            }
            rest.set_seq(rest_seq);
            let offsets = block.get_instr_old_offsets();
            rest.get_instr_old_offsets()
                .add_all(offsets.to_vec().into_iter().skip(1));
            while seq.length() > 1 {
                seq.remove_last();
            }
            while offsets.len() > 1 {
                offsets.remove_at(offsets.len() - 1);
            }
            graph.get_blocks().borrow_mut().add_with_key(rest, id);
            for succ in block.get_succs().to_vec() {
                block.remove_successor(succ);
                rest.add_successor(succ);
            }
            block.add_successor(rest);
            for &hd in &handlers {
                rest.add_successor_exception(hd);
                for range in graph.get_exceptions().iter() {
                    if range.get_handler() == hd && range.get_protected_range().contains(&block) {
                        range.get_protected_range().add(rest);
                    }
                }
            }
        }
        for hd in handlers {
            for range in graph.get_exceptions().iter() {
                if range.get_handler() == hd {
                    range.get_protected_range().remove_item(&block);
                }
            }
            block.remove_successor_exception(hd);
        }
    }

    /// The blocks reached from `head` by regular edges that only `head` and
    /// each other enter and no handler protects.
    fn private_body(graph: ControlFlowGraph, head: BasicBlock) -> Vec<BasicBlock> {
        let last = graph.get_last();
        let first = graph.get_first();
        let mut region: Vec<BasicBlock> = vec![head];
        loop {
            let mut grown = false;
            let mut i = 0;
            while i < region.len() {
                for succ in region[i].get_succs().to_vec() {
                    if succ != last
                        && succ != first
                        && !region.contains(&succ)
                        && succ.get_pred_exceptions().is_empty()
                        && succ.get_succ_exceptions().is_empty()
                        && succ.get_preds().iter().all(|p| region.contains(&p))
                    {
                        region.push(succ);
                        grown = true;
                    }
                }
                i += 1;
            }
            if !grown {
                break;
            }
        }
        region.remove(0);
        region
    }

    /// Replaces `old` by `new` among the exception successors of `block`.
    fn replace_handler(block: BasicBlock, old: BasicBlock, new: BasicBlock) {
        let succs = block.get_succ_exceptions();
        if succs.contains(&new) {
            return;
        }
        for i in 0..succs.len() {
            if succs.get(i) == old {
                succs.set(i, new);
            }
        }
        old.remove_predecessor_exception(block);
        new.add_predecessor_exception(block);
    }
}
