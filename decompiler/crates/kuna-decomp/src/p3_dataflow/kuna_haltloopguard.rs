//! A loop the function never leaves reads global memory, as a RETURN does.
//!
//! Heritage reads every persistent range at each RETURN, halts included, with
//! an `addrforce` COPY (`Heritage::guard_returns`), and that read is what keeps
//! a store to a global alive through `ActionDeadCode`.
//! A function that ends in a loop with no exit edge (`b .`, `for (;;) __WFI();`,
//! a firmware super-loop) has no RETURN, so a global it stores before or inside
//! that loop has no reader and the store is deleted: the AIRCR write of
//! `NVIC_SystemReset` vanishes in front of its `while (true) {}`. Memory written
//! there stays observable to interrupt handlers, DMA and other cores for as
//! long as the loop runs.
//!
//! [`exitless_loops`] finds each cycle of the basic-block graph that has no edge
//! out of it. For each persistent range the function writes and such a loop does
//! not read, [`guard`] records a block of the loop, and when renaming reaches
//! that block's entry [`mark`] makes the range's current value `addrforce`, as
//! the RETURN guard's COPY output is: the store before the loop, or the
//! MULTIEQUAL that joins it with the loop's own stores. No operation is added,
//! so nothing prints and no block changes. A loop that reads the range already
//! keeps the stores that reach it alive.

use std::collections::HashMap;

use kuna_base::address::Address;
use kuna_base::types::{int4, uint4};
use kuna_num::opcodes::OpCode;

use crate::context::{BlockId, VarnodeId};
use crate::funcdata::Funcdata;
use crate::varnode::varnode_flags;

/// A strongly connected component of the basic-block graph that holds a cycle
/// and has no edge out of it.
#[derive(Debug)]
pub(crate) struct ExitlessLoop {
    /// The block the guard goes in: the first one the walk reaches that no
    /// cleanup can remove as doing nothing ([`keeps_block`]), else the one
    /// through which the walk entered.
    at: BlockId,
    /// The index of every block in the loop, sorted.
    members: Vec<usize>,
}

/// Each [`ExitlessLoop`] reached from the function's start block.
pub(crate) fn exitless_loops(fd: &Funcdata) -> Vec<ExitlessLoop> {
    let g = fd.bblocks_ref();
    let Some(root) = g.root else { return Vec::new() };
    let n = g.block(root).get_size() as usize;
    if n == 0 {
        return Vec::new();
    }
    let blocks: Vec<BlockId> = (0..n).map(|i| g.block(root).get_block(i as int4)).collect();
    if blocks.iter().enumerate().any(|(i, &b)| g.block(b).get_index() as usize != i) {
        return Vec::new();
    }
    let succ = |v: usize, e: int4| g.block(g.block(blocks[v]).get_out(e)).get_index() as usize;
    const UNSEEN: usize = usize::MAX;
    let mut order = vec![UNSEEN; n];
    let mut low = vec![0usize; n];
    let mut comp = vec![UNSEEN; n];
    let mut stack: Vec<usize> = Vec::new();
    let mut walk: Vec<(usize, int4)> = Vec::new();
    let mut heads = Vec::new();
    let mut next = 0usize;
    order[0] = next;
    low[0] = next;
    next += 1;
    stack.push(0);
    walk.push((0, 0));
    while let Some(&(v, e)) = walk.last() {
        if e < g.block(blocks[v]).size_out() {
            walk.last_mut().expect("walk top").1 += 1;
            let w = succ(v, e);
            if order[w] == UNSEEN {
                order[w] = next;
                low[w] = next;
                next += 1;
                stack.push(w);
                walk.push((w, 0));
            } else if comp[w] == UNSEEN {
                low[v] = low[v].min(order[w]);
            }
            continue;
        }
        walk.pop();
        if let Some(&(p, _)) = walk.last() {
            low[p] = low[p].min(low[v]);
        }
        if low[v] != order[v] {
            continue;
        }
        let mut members = Vec::new();
        while let Some(w) = stack.pop() {
            comp[w] = v;
            members.push(w);
            if w == v {
                break;
            }
        }
        let mut cyclic = members.len() > 1;
        let mut sink = true;
        for &m in &members {
            for e in 0..g.block(blocks[m]).size_out() {
                let w = succ(m, e);
                if comp[w] != v {
                    sink = false;
                } else if w == m {
                    cyclic = true;
                }
            }
        }
        if sink && cyclic {
            members.sort_by_key(|&m| order[m]);
            let kept = members.iter().copied().find(|&m| keeps_block(fd, blocks[m])).unwrap_or(v);
            members.sort_unstable();
            heads.push(ExitlessLoop { at: blocks[kept], members });
        }
    }
    heads
}

/// Can no cleanup remove block `bl` as one that does nothing? It branches more
/// than one way or back to itself, or holds an operation with an effect.
fn keeps_block(fd: &Funcdata, bl: BlockId) -> bool {
    let b = fd.bblocks_ref().block(bl);
    if b.size_out() != 1 || b.get_out(0) == bl {
        return true;
    }
    fd.bb_ops(bl).into_iter().any(|op| {
        fd.obank().get(op).is_some_and(|o| {
            matches!(
                o.code(),
                OpCode::CPUI_CALL | OpCode::CPUI_CALLIND | OpCode::CPUI_CALLOTHER | OpCode::CPUI_STORE
                    | OpCode::CPUI_BRANCHIND
            )
        })
    })
}

/// Record, for each loop in `heads` that none of the range's `read`s is in, the
/// block whose entry [`mark`] keeps the range alive at, when the range at `addr`
/// is persistent (`fl`) and the function stores to it.
pub(crate) fn guard(
    fd: &Funcdata,
    heads: &[ExitlessLoop],
    fl: uint4,
    addr: &Address,
    read: &[VarnodeId],
    write: &[VarnodeId],
    marks: &mut HashMap<usize, Vec<Address>>,
) {
    if heads.is_empty() || (fl & varnode_flags::persist) == 0 || !fd.get_arch().halt_loop_guard {
        return;
    }
    let stored = write.iter().any(|&vn| {
        fd.vbank()
            .get(vn)
            .and_then(|v| v.get_def())
            .and_then(|d| fd.obank().get(d))
            .is_some_and(|o| !o.is_marker() && !o.is_return_copy())
    });
    if !stored {
        return;
    }
    let read_blocks: Vec<usize> = read
        .iter()
        .filter_map(|&vn| fd.vbank().get(vn)?.descend_iter().next())
        .filter_map(|op| fd.obank().get(op)?.get_parent())
        .map(|bl| fd.bblocks_ref().block(bl).get_index() as usize)
        .collect();
    for lp in heads {
        if read_blocks.iter().any(|b| lp.members.binary_search(b).is_ok()) {
            continue;
        }
        let index = fd.bblocks_ref().block(lp.at).get_index() as usize;
        marks.entry(index).or_default().push(addr.clone());
    }
}

/// Make each value in `reaching`, the definitions renaming has current at a
/// marked block's entry, `addrforce`, which `ActionDeadCode` keeps alive with
/// the stores that reach it. An input the function never wrote needs nothing.
pub(crate) fn mark(fd: &mut Funcdata, reaching: &[VarnodeId]) {
    for &vn in reaching {
        if let Some(v) = fd.vbank_mut().get_mut(vn).filter(|v| v.is_written()) {
            v.set_addr_force();
        }
    }
}
