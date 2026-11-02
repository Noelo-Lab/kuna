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
//! The skipped guards' values still flow through their calls to the next
//! reader, so the range stays live over the same code, which is what merging
//! tests. That holds wherever something reads the range later; before a full
//! write or the end of a path with no return, upstream's last guard is the
//! final reader. So a last walk keeps the last call before each such point:
//! a call nothing reads after (no read, return or kept guard before a full
//! write) keeps its guard.
//!
//! The reads are the range's free reads, its partial writes (which read the
//! bytes they keep) and every LOAD from the range's space that is not through
//! the stack pointer, since a LOAD whose pointer later folds to a constant
//! becomes a read of the range. The writes are its full and partial writes and
//! every STORE into the space. Returns are not reads for the first walk: their
//! persist COPYs read whichever guard or write is last, which needs no guard of
//! its own.

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
/// read may still be live after the call), or nothing that reads the range (a
/// read, a return, a kept guard) follows it at all before a full write. `read`
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
        push(fd, op, USE, &mut events);
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
        USE | UDEF => Step::Gen,
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
        USE | UDEF => Step::Gen,
        KILL => Step::Kill,
        CALL => Step::ObserveKill,
        _ => Step::Skip,
    }, |op, reach| {
        if reach {
            keep.insert(op);
            read_kept.insert(op);
        }
    });
    let mut unread = Vec::new();
    solve(fd, n, &events, true, |kind, _| match kind {
        USE | UDEF | RET => Step::Gen,
        KILL => Step::Kill,
        CALL => Step::ObserveGen,
        _ => Step::Skip,
    }, |op, live| {
        if !live && !keep.contains(&op) {
            unread.push(op);
        }
    });
    keep.extend(unread);
    (keep, read_kept)
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

/// Does the new read `vn` overlap a range of `pruned` (space index, offset,
/// size) some of whose calls were left unguarded, at an op [`guarded`] does not
/// clear?
pub(crate) fn crosses_skipped(fd: &Funcdata, vn: VarnodeId, pruned: &[(int4, u64, int4)]) -> bool {
    let Some(v) = fd.vbank().get(vn) else { return false };
    let (space, lo) = (v.get_space(), v.get_offset());
    let hi = lo.saturating_add(v.get_size() as u64);
    pruned.iter().any(|&(index, offset, size)| {
        index == space.get_index()
            && offset < hi
            && lo < offset.saturating_add(size as u64)
            && {
                let addr = Address::new(std::rc::Rc::clone(&space), offset);
                v.descend_iter().any(|op| !guarded(fd, op, &addr, size))
            }
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
/// stop or start it.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Step {
    Gen,
    Kill,
    Skip,
    ObserveKill,
    ObserveGen,
}

/// One walk over the CFG, backward (`back`: is a read reachable from here?)
/// or forward (does a write reach here?), calling `observe` with the answer at
/// each `ObserveKill` or `ObserveGen` event.
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
            Some(Step::Gen | Step::ObserveGen) => {
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
            Step::ObserveGen => {
                observe(op, live);
                live = true;
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
