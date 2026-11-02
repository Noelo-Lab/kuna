//! (kuna `persistcalls`) Guard a persistent global only at the calls next to
//! its reads and writes.
//!
//! Upstream gives every heritaged global an INDIRECT at every call whose effect
//! on it is unknown, and because the range is persistent and address-tied the
//! outputs survive dead-code removal. A guard does two things: its input lets
//! the call read the value the function last stored, and its output is the
//! value a later read sees once the call may have changed it. Neither needs
//! every call. Between a write and its first following call, that call's input
//! keeps the write live; every later call on the path reads only another
//! call's output. Between a read and the last call before it, that call's
//! output is what the read sees; every earlier call feeds only another call.
//! So a call keeps its guard when a write reaches it with no call in between,
//! or when a read follows it with no call or full write in between. Liveness
//! is a walk over the CFG in each direction from the range's accesses, stopped
//! by the calls themselves.
//!
//! A value read before a call can still be live after it, in a register or an
//! expression. Upstream's guard at that call is a second definition of the
//! global live alongside it, which is what keeps the printer from moving the
//! read past the call and makes the value a variable of its own. The first
//! call after each read keeps its guard for that: past it, the read value is
//! live alongside that guard's output.
//!
//! Every guard reads the value before it, so with every guard built the range
//! is live wherever a call, a read or a return follows before a full write,
//! and merging tests that cover: a value held in a register across calls the
//! guards cover cannot become the global. A skipped call keeps that cover only
//! if every path after it meets something that really reads the range before a
//! full write or the end of the path: a free read, a partial write, a return or
//! a kept guard. Every path, not some, because a later pass can remove the
//! branch whose reader the call relied on. So a last walk, backward from the
//! exits, keeps each call with a path to a full write or an end that meets no
//! such reader, deciding as it meets each call: a run of calls before a full
//! write keeps only its last, and a loop of calls before one keeps a guard
//! inside the loop.
//!
//! The reads are the range's free reads, its partial writes (which read the
//! bytes they keep) and every LOAD and STORE in the range's space that is not
//! through the stack pointer, since a LOAD whose pointer later folds to a
//! constant becomes a read of the range and a STORE a write of part of it. The
//! writes are its full and partial writes and every such STORE. For the last
//! walk a LOAD reads nothing and a STORE may write all of the range, which is
//! what each is until it folds. A return reads the range too (its persist
//! COPY), so the call before it keeps its guard.
//!
//! What is left is a value that crosses skipped calls only into a kept guard's
//! input, which nothing prints. A join is the other place such a value could
//! land: a MULTIEQUAL that takes, on one edge, the value from before a skipped
//! call is a stale definition merging may copy out where it is made. So the
//! last skipped call on each such edge keeps its guard, until no join of the
//! range takes one ([`stale_calls`]).

use kuna_base::address::Address;
use kuna_base::space::spacetype;
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::context::{BlockId, OpId, VarnodeId};
use crate::funcdata::Funcdata;
use crate::kuna_fasthash::FxHashSet;

const USE: u8 = 0;
const KILL: u8 = 1;
const DEF: u8 = 2;
const UDEF: u8 = 3;
const RET: u8 = 4;
const LOAD: u8 = 5;
const CALL: u8 = 9;

type Event = (usize, u32, u8, OpId);

/// The LOAD and STORE ops of one space that can reach a global, and the
/// function's returns, collected once per heritage pass.
#[derive(Debug, Default)]
pub(crate) struct MemoryOps {
    space: Option<i32>,
    loads: Vec<OpId>,
    stores: Vec<OpId>,
    returns: Vec<OpId>,
}

impl MemoryOps {
    pub(crate) fn clear(&mut self) {
        self.space = None;
        self.loads.clear();
        self.stores.clear();
        self.returns.clear();
    }

    fn collect(&mut self, fd: &Funcdata, space: i32) {
        if self.space == Some(space) {
            return;
        }
        self.space = Some(space);
        self.loads.clear();
        self.stores.clear();
        self.returns = fd
            .obank()
            .iter_code(OpCode::CPUI_RETURN)
            .filter(|&op| fd.obank().get(op).is_some_and(|o| !o.is_dead()))
            .collect();
        let manage = fd.get_arch().manage();
        for (code, out) in [(OpCode::CPUI_LOAD, &mut self.loads), (OpCode::CPUI_STORE, &mut self.stores)] {
            for op in fd.obank().iter_code(code) {
                let Some(o) = fd.obank().get(op) else { continue };
                if o.is_dead() || o.uses_spacebase_ptr() {
                    continue;
                }
                let into = o
                    .get_in(0)
                    .and_then(|v| fd.vbank().get(v))
                    .and_then(|v| manage.get_space(v.get_addr().get_offset() as i32))
                    .is_some_and(|s| s.get_index() == space);
                if into {
                    out.push(op);
                }
            }
        }
    }
}

/// Does the range `addr` with properties `fl` take sparse call guards: it is
/// persistent, writable (read-only memory is `readonlycalls`'), not volatile,
/// and in the default data space of a function that can restart (not a
/// jump-table partial)?
pub(crate) fn applies(fd: &Funcdata, fl: u32, addr: &Address) -> bool {
    use crate::varnode::varnode_flags;
    !fd.is_jumptable_recovery_on()
        && (fl & varnode_flags::persist) != 0
        && (fl & (varnode_flags::volatil | varnode_flags::readonly)) == 0
        && addr.get_space().is_some_and(|s| {
            s.get_type() == spacetype::IPTR_PROCESSOR
                && fd.get_arch().manage().get_default_data_space().is_some_and(|d| d.get_index() == s.get_index())
        })
}

/// The calls of `calls` not in `keep` whose value reaches a join of `joins`
/// with no definition of the range after them: on each path back from an edge
/// into a join, the first such call met before a definition (`defs`, a kept
/// call), another join or the entry. A MULTIEQUAL taking one of their values
/// would hold the value from before the call.
pub(crate) fn stale_calls(
    fd: &Funcdata,
    calls: &[OpId],
    keep: &FxHashSet<OpId>,
    defs: &[OpId],
    joins: &[BlockId],
) -> Vec<OpId> {
    let n = num_blocks(fd);
    let mut events: Vec<Event> = Vec::with_capacity(calls.len() + defs.len());
    for &op in defs {
        push(fd, op, KILL, &mut events);
    }
    for &op in calls {
        push(fd, op, if keep.contains(&op) { KILL } else { CALL }, &mut events);
    }
    events.sort_unstable_by_key(|&(bl, order, kind, _)| (bl, order, kind));
    let bblocks = fd.bblocks_ref();
    let Some(root) = bblocks.root else { return Vec::new() };
    let rootb = bblocks.block(root);
    let index = |b: BlockId| bblocks.block(b).get_index() as usize;
    let mut last: Vec<Option<(u8, OpId)>> = vec![None; n];
    for block in events.chunk_by(|a, b| a.0 == b.0) {
        let e = block[block.len() - 1];
        last[e.0] = Some((e.2, e.3));
    }
    let mut join = vec![false; n];
    for &bl in joins {
        join[index(bl)] = true;
    }
    let mut seen = vec![false; n];
    let mut stale = Vec::new();
    for &bl in joins {
        let block = bblocks.block(bl);
        let mut work: Vec<usize> = (0..block.size_in()).map(|k| index(block.get_in(k))).collect();
        while let Some(at) = work.pop() {
            if seen[at] {
                continue;
            }
            seen[at] = true;
            match last[at] {
                Some((CALL, op)) => stale.push(op),
                Some(_) => {}
                None if join[at] => {}
                None => {
                    let b = bblocks.block(rootb.get_block(at as i32));
                    work.extend((0..b.size_in()).map(|k| index(b.get_in(k))));
                }
            }
        }
    }
    stale
}

/// Restart the function with every persistent global guarded at every call:
/// a read or a guard removal reached a range across a call it left unguarded.
pub(crate) fn restart_full(fd: &mut Funcdata) {
    if fd.get_override().persist_calls_off() {
        return;
    }
    fd.get_override_mut().set_persist_calls_off();
    fd.set_restart_pending(true);
    fd.kuna_set_restart_now();
}

/// Take the op and Varnode numbers a skipped guard would have used, so every
/// later op and Varnode is numbered as with every guard built.
pub(crate) fn skip_guard(fd: &mut Funcdata) {
    fd.vbank_mut().reserve_create_index(3);
    fd.obank_mut().reserve_uniq(1);
}

/// Take the op and Varnode numbers a MULTIEQUAL with `slots` inputs would have
/// used at a join only skipped guards needed.
pub(crate) fn skip_join(fd: &mut Funcdata, slots: u32) {
    fd.obank_mut().reserve_uniq(1);
    fd.vbank_mut().reserve_create_index(1 + slots);
}

/// The calls of `calls` that keep their guard on the range: a write reaches
/// the call with no call between, a read follows it with no call or full write
/// between, a read reaches it with no call or full write between (the value
/// read may still be live after the call), or some path after it meets a full
/// write or its end before anything that reads the range (a read, a partial
/// write, a return, a kept guard). `read`
/// are the range's free reads, `full` and `partial` the ops writing all or part
/// of it. The second set holds the calls kept for a read, before or after.
pub(crate) fn keep(
    fd: &Funcdata,
    memory: &mut MemoryOps,
    space: i32,
    calls: &[OpId],
    read: &[VarnodeId],
    full: &[OpId],
    partial: &[OpId],
) -> (FxHashSet<OpId>, FxHashSet<OpId>) {
    memory.collect(fd, space);
    let mut events: Vec<Event> =
        Vec::with_capacity(calls.len() + read.len() + memory.loads.len() + memory.stores.len() + memory.returns.len());
    for &vn in read {
        if let Some(v) = fd.vbank().get(vn) {
            for op in v.descend_iter() {
                push(fd, op, USE, &mut events);
            }
        }
    }
    for &op in &memory.loads {
        push(fd, op, LOAD, &mut events);
    }
    for &op in full {
        push(fd, op, KILL, &mut events);
    }
    for &op in partial {
        push(fd, op, UDEF, &mut events);
    }
    for &op in &memory.stores {
        push(fd, op, DEF, &mut events);
    }
    for &op in &memory.returns {
        push(fd, op, RET, &mut events);
    }
    for &op in calls {
        push(fd, op, CALL, &mut events);
    }
    events.sort_unstable_by_key(|&(bl, order, kind, _)| (bl, order, kind));
    let n = num_blocks(fd);
    let mut read_kept = FxHashSet::default();
    solve(fd, n, &events, true, |kind, _| match kind {
        USE | UDEF | LOAD | DEF | RET => Step::Gen,
        KILL => Step::Kill,
        CALL => Step::ObserveKill,
        _ => Step::Skip,
    }, |op, live| {
        if live {
            read_kept.insert(op);
        }
    });
    let mut keep = read_kept.clone();
    solve(fd, n, &events, false, |kind, _| match kind {
        KILL | DEF | UDEF => Step::Gen,
        CALL => Step::ObserveKill,
        _ => Step::Skip,
    }, |op, reach| {
        if reach {
            keep.insert(op);
        }
    });
    solve(fd, n, &events, false, |kind, _| match kind {
        USE | UDEF | LOAD => Step::Gen,
        KILL => Step::Kill,
        CALL => Step::ObserveKill,
        _ => Step::Skip,
    }, |op, reach| {
        if reach {
            keep.insert(op);
            read_kept.insert(op);
        }
    });
    ground(fd, n, &events, &mut keep);
    (keep, read_kept)
}

/// Keep each call from which some path meets a full write of the range, or the
/// end of the path, before a free read, a partial write, a return or a kept
/// guard. A LOAD reads nothing until it folds and a STORE may fold into a full
/// write. Walked backward,
/// successors first, deciding at each call as it is met: a call with such a
/// path after it is kept, and either way the calls before it reach a reader
/// there. A block is walked again when such a path opens from one of its
/// successors; a call once kept stays kept.
fn ground(fd: &Funcdata, n: usize, events: &[Event], keep: &mut FxHashSet<OpId>) {
    let bblocks = fd.bblocks_ref();
    let Some(root) = bblocks.root else { return };
    let rootb = bblocks.block(root);
    let id = |i: usize| rootb.get_block(i as i32);
    let index = |b: BlockId| bblocks.block(b).get_index() as usize;
    let mut span: Vec<(usize, usize)> = vec![(0, 0); n];
    let mut at = 0;
    for block in events.chunk_by(|a, b| a.0 == b.0) {
        span[block[0].0] = (at, at + block.len());
        at += block.len();
    }
    let mut order = Vec::with_capacity(n);
    let mut seen = vec![false; n];
    for start in 0..n {
        if seen[start] {
            continue;
        }
        seen[start] = true;
        let mut stack = vec![(start, 0i32)];
        while let Some(&mut (bl, ref mut next)) = stack.last_mut() {
            let block = bblocks.block(id(bl));
            if *next < block.size_out() {
                let succ = index(block.get_out(*next));
                *next += 1;
                if !seen[succ] {
                    seen[succ] = true;
                    stack.push((succ, 0));
                }
            } else {
                order.push(bl);
                stack.pop();
            }
        }
    }
    let mut entry = vec![false; n];
    let mut queued = vec![true; n];
    let mut work: Vec<usize> = order.into_iter().rev().collect();
    while let Some(bl) = work.pop() {
        queued[bl] = false;
        let block = bblocks.block(id(bl));
        let mut open = block.size_out() == 0 || (0..block.size_out()).any(|k| entry[index(block.get_out(k))]);
        let (lo, hi) = span[bl];
        for &(_, _, kind, op) in events[lo..hi].iter().rev() {
            match kind {
                USE | UDEF | RET => open = false,
                KILL | DEF => open = true,
                CALL => {
                    if open {
                        keep.insert(op);
                    }
                    open = false;
                }
                _ => {}
            }
        }
        if open && !entry[bl] {
            entry[bl] = true;
            for k in 0..block.size_in() {
                let pred = index(block.get_in(k));
                if !queued[pred] {
                    queued[pred] = true;
                    work.push(pred);
                }
            }
        }
    }
}

/// Does every path into `op` meet a guard of the range `addr:size`, another
/// definition of it or the function entry before it meets a call without one,
/// and every path out of it meet a definition or the end before such a call?
/// A read heritage links into the range's chain after the pass that guarded it
/// sees the right value, and a value read there stays apart from the range
/// past the next call, exactly when this holds.
pub(crate) fn guarded(fd: &Funcdata, op: OpId, addr: &Address, size: int4) -> bool {
    walk(fd, op, addr, size, true) && walk(fd, op, addr, size, false)
}

/// Does the new Varnode `vn` overlap a range of `pruned` (space index, offset,
/// size) some of whose calls were left unguarded, so that linking it could
/// read a value across such a call? A read is cleared by [`guarded`] at each
/// op reading it and a write of the whole range needs nothing; a partial write,
/// which reads the bytes it keeps, must reach a guard or definition first. A
/// Varnode reaching past the range (`partial`, a pass that grows it) is never
/// cleared: refining or revisiting the range rebuilds its value from the
/// skipped calls' stale inputs.
pub(crate) fn crosses_skipped(fd: &Funcdata, vn: VarnodeId, pruned: &[(int4, u64, int4)], partial: bool) -> bool {
    let Some(v) = fd.vbank().get(vn) else { return false };
    let (space, lo) = (v.get_space(), v.get_offset());
    let hi = lo.saturating_add(v.get_size() as u64);
    pruned.iter().any(|&(index, offset, size)| {
        index == space.get_index()
            && offset < hi
            && lo < offset.saturating_add(size as u64)
            && (partial || {
                let addr = Address::new(std::rc::Rc::clone(&space), offset);
                if v.is_written() {
                    v.get_size() < size && v.get_def().is_some_and(|op| !walk(fd, op, &addr, size, true))
                } else {
                    v.descend_iter().any(|op| !guarded(fd, op, &addr, size))
                }
            })
    })
}

fn walk(fd: &Funcdata, op: OpId, addr: &Address, size: int4, back: bool) -> bool {
    let Some(start) = fd.obank().get(op).and_then(|o| o.get_parent()) else {
        return false;
    };
    let defines = |o: &crate::op::PcodeOp| {
        o.code() != OpCode::CPUI_MULTIEQUAL
            && o.get_out().and_then(|w| fd.vbank().get(w)).is_some_and(|w| w.get_addr() == addr && w.get_size() == size)
    };
    let step = |o: &crate::op::PcodeOp| if back { o.basic_neighbours().0 } else { o.basic_neighbours().1 };
    let mut seen: FxHashSet<BlockId> = FxHashSet::default();
    let mut work = vec![(start, fd.obank().get(op).and_then(step))];
    'paths: while let Some((bl, mut cur)) = work.pop() {
        while let Some(at) = cur {
            let Some(o) = fd.obank().get(at) else { return false };
            if defines(o) {
                continue 'paths;
            }
            if matches!(o.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND) {
                if !back {
                    return false;
                }
                let mut prev = o.basic_neighbours().0;
                while let Some(p) = prev.and_then(|p| fd.obank().get(p)) {
                    if p.code() != OpCode::CPUI_INDIRECT {
                        break;
                    }
                    if defines(p) {
                        continue 'paths;
                    }
                    prev = p.basic_neighbours().0;
                }
                return false;
            }
            cur = step(o);
        }
        let block = fd.bblocks_ref().block(bl);
        let count = if back { block.size_in() } else { block.size_out() };
        for k in 0..count {
            let next = if back { block.get_in(k) } else { block.get_out(k) };
            if seen.insert(next) {
                work.push((next, if back { fd.bb_op_tail(next) } else { fd.bb_op_head(next) }));
            }
        }
    }
    true
}

fn push(fd: &Funcdata, op: OpId, kind: u8, events: &mut Vec<Event>) {
    if let Some(o) = fd.obank().get(op) {
        if let Some(bl) = o.get_parent() {
            events.push((fd.bblocks_ref().block(bl).get_index() as usize, o.get_seq_num().get_order(), kind, op));
        }
    }
}

fn num_blocks(fd: &Funcdata) -> usize {
    let bblocks = fd.bblocks_ref();
    bblocks.root.map_or(0, |r| bblocks.block(r).get_size() as usize)
}

/// What one access does to the walk: start or stop the range being tracked,
/// let it through, or tell `observe` whether it is tracked there and then
/// stop it.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Step {
    Gen,
    Kill,
    Skip,
    ObserveKill,
}

/// One walk over the CFG, backward (`back`: is a read reachable from here?)
/// or forward (does a write reach here?), calling `observe` with the answer at
/// each `ObserveKill` event.
fn solve(
    fd: &Funcdata,
    n: usize,
    events: &[Event],
    back: bool,
    class: impl Fn(u8, OpId) -> Step,
    mut observe: impl FnMut(OpId, bool),
) {
    let mut entry = vec![false; n];
    let mut exit = vec![false; n];
    let mut transparent = vec![true; n];
    let mut work = Vec::new();
    for block in events.chunk_by(|a, b| a.0 == b.0) {
        let bl = block[0].0;
        let first = |e: &&Event| !matches!(class(e.2, e.3), Step::Skip);
        let step = if back { block.iter().find(first) } else { block.iter().rev().find(first) };
        match step.map(|e| class(e.2, e.3)) {
            Some(Step::Gen) => {
                entry[bl] = true;
                transparent[bl] = false;
                work.push(bl);
            }
            Some(_) => transparent[bl] = false,
            None => {}
        }
    }
    let bblocks = fd.bblocks_ref();
    let Some(root) = bblocks.root else { return };
    while let Some(bl) = work.pop() {
        let block = bblocks.block(bblocks.block(root).get_block(bl as i32));
        let count = if back { block.size_in() } else { block.size_out() };
        for k in 0..count {
            let next = if back { block.get_in(k) } else { block.get_out(k) };
            let next = bblocks.block(next).get_index() as usize;
            exit[next] = true;
            if transparent[next] && !entry[next] {
                entry[next] = true;
                work.push(next);
            }
        }
    }
    for block in events.chunk_by(|a, b| a.0 == b.0) {
        let mut live = exit[block[0].0];
        let mut visit = |&(_, _, kind, op): &Event| match class(kind, op) {
            Step::Gen => live = true,
            Step::Kill => live = false,
            Step::Skip => {}
            Step::ObserveKill => {
                observe(op, live);
                live = false;
            }
        };
        if back {
            block.iter().rev().for_each(&mut visit);
        } else {
            block.iter().for_each(&mut visit);
        }
    }
}

#[cfg(test)]
mod tests;
