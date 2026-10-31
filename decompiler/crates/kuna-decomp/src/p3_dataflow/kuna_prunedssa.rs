//! (kuna `prunedssa`) Heritage markers only where the range is still read.
//!
//! Heritage builds minimal SSA: a MULTIEQUAL at every join in the iterated
//! dominance frontier of a range's writes, and an INDIRECT at every call with
//! an unknown effect on it. For a range that is neither persistent nor
//! address-tied, a marker's output is consumed only by a later read of the
//! range, through the chain of later markers, so `ActionDeadCode` deletes
//! every marker no read can follow. A huge function builds that cross product
//! in full (stack slots or reused unique temporaries times calls or joins)
//! and throws almost all of it away. Answering the liveness question first
//! keeps only the markers `ActionDeadCode` would keep: a backward walk over
//! the CFG from the range's reads, stopped by its full writes, gives whether
//! the range is live at each join and after each call.
//!
//! An address-tied range is left alone: its call guards' outputs are
//! `addrforce`d, and once a direct write taints them they survive dead-code
//! removal, so which of them `ActionDeadCode` keeps is not a liveness
//! question. Every stack slot of a function with a local scope is address-tied;
//! the stack slots of a jump-table partial, which has none yet, are not.
//!
//! The joins are pruned from the reads present after `Heritage::guard`, in the
//! unique and stack spaces. Register joins are left whole: the prototype
//! passes that run between heritage and `ActionDeadCode` read a register's
//! dead joins (a clobber on one path of an argument register's join, a pad
//! push). The calls are pruned before their guards exist, so the reads the
//! guard pass adds at a call (input trials, overlapping outputs) or at a
//! return (output trials) count as reads there; this half is limited to the
//! stack space.

use kuna_base::address::Address;
use kuna_base::space::spacetype;
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::context::{BlockId, OpId, VarnodeId};
use crate::fspec::{Containment, FuncCallSpecs, OFFSET_UNKNOWN};
use crate::funcdata::Funcdata;
use crate::kuna_fasthash::FxHashSet;

const CALL: u8 = 0;
const USE: u8 = 1;
const KILL: u8 = 2;

type Event = (usize, u32, u8, OpId);

/// The calls that keep their guard for one stack range.
pub(crate) struct LiveCalls {
    keep: FxHashSet<OpId>,
}

impl LiveCalls {
    /// Can a read of the range follow `call`?
    pub(crate) fn needs(&self, call: OpId) -> bool {
        self.keep.contains(&call)
    }
}

/// May the markers of the range `addr` with properties `fl` be pruned on this
/// heritage pass: the option is on, the range is neither persistent nor
/// address-tied, and the `ActionDeadCode` after this pass may remove from its
/// space?
pub(crate) fn applies(fd: &Funcdata, fl: u32, addr: &Address, pass: int4, deadcode_delay: int4) -> bool {
    use crate::varnode::varnode_flags;
    fd.get_arch().pruned_ssa
        && pass >= deadcode_delay
        && (fl & (varnode_flags::addrtied | varnode_flags::persist)) == 0
        && addr.get_space().is_some()
}

/// Do the call guards of the range qualify for [`live_calls`]?
pub(crate) fn applies_to_calls(fd: &Funcdata, addr: &Address) -> bool {
    fd.num_calls() >= 2 && addr.get_space().is_some_and(|s| s.get_type() == spacetype::IPTR_SPACEBASE)
}

/// The calls of `calls` after which a read of the stack range `addr:size` can
/// follow. `read` are its free reads, `full` the ops writing all of it and
/// `partial` the ops writing part of it (each reads the rest first).
pub(crate) fn live_calls(
    fd: &Funcdata,
    calls: &[FuncCallSpecs],
    addr: &Address,
    size: int4,
    read: &[VarnodeId],
    full: &[OpId],
    partial: &[OpId],
) -> LiveCalls {
    let mut events: Vec<Event> = Vec::with_capacity(read.len() + full.len() + calls.len());
    for &vn in read {
        if let Some(v) = fd.vbank().get(vn) {
            for op in v.descend_iter() {
                push(fd, op, USE, &mut events);
            }
        }
    }
    for &op in partial {
        push(fd, op, USE, &mut events);
    }
    for &op in full {
        push(fd, op, KILL, &mut events);
    }
    let spc = addr.get_space().expect("live_calls: stack range has a space");
    for fc in calls {
        let op = fc.get_op();
        push(fd, op, CALL, &mut events);
        if call_reads(fd, fc, addr, size, spc) {
            push(fd, op, USE, &mut events);
        }
    }
    if fd.get_active_output().is_some()
        && fd.get_func_proto().characterize_as_output(addr, size) != Containment::NoContainment
    {
        for op in fd.obank().iter_code(OpCode::CPUI_RETURN) {
            push(fd, op, USE, &mut events);
        }
    }
    let live = Liveness::solve(fd, &mut events);
    let mut keep = FxHashSet::default();
    live.scan(&events, |kind, op, is_live| {
        if kind == CALL && is_live {
            keep.insert(op);
        }
    });
    LiveCalls { keep }
}

/// Drop from `merges` every join at which the range `addr:size` is not live,
/// judged from its reads and writes as `Heritage::guard` left them. Returns
/// whether any join was dropped.
pub(crate) fn prune_merges(fd: &Funcdata, addr: &Address, size: int4, merges: &mut Vec<BlockId>) -> bool {
    let joins_prune = addr
        .get_space()
        .is_some_and(|s| matches!(s.get_type(), spacetype::IPTR_INTERNAL | spacetype::IPTR_SPACEBASE));
    if merges.is_empty() || !joins_prune {
        return false;
    }
    let mut events: Vec<Event> = Vec::new();
    let end = addr + i64::from(size);
    for vn in fd.vbank().iter_loc_addr_range(addr, &end) {
        let Some(v) = fd.vbank().get(vn) else { continue };
        if v.is_written() {
            let Some(def) = v.get_def() else { continue };
            let Some(op) = fd.obank().get(def) else { continue };
            if op.code() == OpCode::CPUI_INDIRECT && !op.is_indirect_creation() {
                continue;
            }
            let full = v.get_addr() == addr && v.get_size() == size;
            push(fd, def, if full { KILL } else { USE }, &mut events);
        } else if !v.is_input() {
            for op in v.descend_iter() {
                let transparent = fd
                    .obank()
                    .get(op)
                    .is_some_and(|o| o.code() == OpCode::CPUI_INDIRECT && o.get_in(0) == Some(vn));
                if !transparent {
                    push(fd, op, USE, &mut events);
                }
            }
        }
    }
    let live = Liveness::solve(fd, &mut events);
    let bblocks = fd.bblocks_ref();
    let before = merges.len();
    merges.retain(|&bl| live.live_in[bblocks.block(bl).get_index() as usize]);
    merges.len() != before
}

fn push(fd: &Funcdata, op: OpId, kind: u8, events: &mut Vec<Event>) {
    if let Some(o) = fd.obank().get(op) {
        if let Some(bl) = o.get_parent() {
            events.push((fd.bblocks_ref().block(bl).get_index() as usize, o.get_seq_num().get_order(), kind, op));
        }
    }
}

/// Block-level liveness of one range over `events`, which [`Liveness::solve`]
/// sorts into block and op order.
struct Liveness {
    live_in: Vec<bool>,
    live_out: Vec<bool>,
}

impl Liveness {
    fn solve(fd: &Funcdata, events: &mut [Event]) -> Liveness {
        events.sort_unstable_by_key(|&(bl, order, kind, _)| (bl, order, kind));
        let bblocks = fd.bblocks_ref();
        let root = bblocks.root.expect("prunedssa: blocks built");
        let nblocks = bblocks.block(root).get_size() as usize;
        let mut live_in = vec![false; nblocks];
        let mut live_out = vec![false; nblocks];
        let mut transparent = vec![true; nblocks];
        let mut work: Vec<usize> = Vec::new();
        for block in events.chunk_by(|a, b| a.0 == b.0) {
            let bl = block[0].0;
            if block.iter().find(|e| e.2 != CALL).is_some_and(|e| e.2 == USE) {
                live_in[bl] = true;
                work.push(bl);
            }
            if block.iter().any(|e| e.2 == KILL) {
                transparent[bl] = false;
            }
        }
        while let Some(bl) = work.pop() {
            let block = bblocks.block(root).get_block(bl as int4);
            for k in 0..bblocks.block(block).size_in() {
                let pred = bblocks.block(bblocks.block(block).get_in(k)).get_index() as usize;
                live_out[pred] = true;
                if transparent[pred] && !live_in[pred] {
                    live_in[pred] = true;
                    work.push(pred);
                }
            }
        }
        Liveness { live_in, live_out }
    }

    /// Visit every event backward through its block, with whether the range
    /// is live just after it.
    fn scan(&self, events: &[Event], mut visit: impl FnMut(u8, OpId, bool)) {
        for block in events.chunk_by(|a, b| a.0 == b.0) {
            let mut live = self.live_out[block[0].0];
            for &(_, _, kind, op) in block.iter().rev() {
                match kind {
                    USE => live = true,
                    KILL => live = false,
                    _ => {}
                }
                visit(kind, op, live);
            }
        }
    }
}

/// Does the guard pass add a read of the range at `fc` itself: an input trial,
/// a call output overlapping it, or a locked stack output?
fn call_reads(
    fd: &Funcdata,
    fc: &FuncCallSpecs,
    addr: &Address,
    size: int4,
    spc: &std::rc::Rc<kuna_base::space::AddrSpace>,
) -> bool {
    let own_output = fd
        .obank()
        .get(fc.get_op())
        .and_then(|o| o.get_out())
        .and_then(|out| fd.vbank().get(out))
        .is_some_and(|v| {
            v.get_space().get_index() == spc.get_index()
                && v.get_offset() < addr.get_offset().wrapping_add(size as u64)
                && addr.get_offset() < v.get_offset().wrapping_add(v.get_size() as u64)
        });
    if own_output {
        return true;
    }
    if fc.get_spacebase_offset() == OFFSET_UNKNOWN {
        return false;
    }
    let off = spc.wrap_offset(addr.get_offset().wrapping_sub(fc.get_spacebase_offset()));
    let trans = Address::new(std::rc::Rc::clone(spc), off);
    (fc.is_input_active() && fc.proto().characterize_as_input_param(&trans, size) != Containment::NoContainment)
        || ((fc.is_output_active() || fc.is_stack_output_lock())
            && fc.proto().characterize_as_output(&trans, size) != Containment::NoContainment)
}
