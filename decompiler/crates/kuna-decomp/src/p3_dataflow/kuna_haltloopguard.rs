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
//! out of it and the block through which it is entered. For each persistent
//! range the function writes, [`guard`] records that block, and renaming records
//! the range's value at the block's entry. Once renaming is done, [`mark`] makes
//! the writes behind that value `addrforce`, as the RETURN guard's COPY output
//! is: the store before the loop and the loop's own stores that the MULTIEQUALs
//! there join. No operation is added, so nothing prints and no block changes.
//! A read inside the loop is no substitute: it may only feed the loop's own
//! stores (`for (;;) *odr ^= 0x2000;`), a cycle with no other reader, or fold
//! into a constant once the store reaches it.

use std::collections::BTreeMap;

use kuna_base::address::Address;
use kuna_base::types::{int4, uint4};
use kuna_num::opcodes::OpCode;

use crate::context::{BlockId, VarnodeId};
use crate::funcdata::Funcdata;
use crate::kuna_fasthash::FxHashSet;
use crate::varnode::varnode_flags;

/// The entry block of each strongly connected component of the basic-block
/// graph reached from the function's start block that holds a cycle and has no
/// edge out of it: the block through which a depth-first walk entered it.
pub(crate) fn exitless_loops(fd: &Funcdata) -> Vec<BlockId> {
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
            heads.push(blocks[v]);
        }
    }
    heads
}

/// Record each loop entry in `heads` as a block whose entry [`mark`] keeps the
/// range at `addr` alive at, when the range is persistent (`fl`) and the
/// function stores to it.
pub(crate) fn guard(
    fd: &Funcdata,
    heads: &[BlockId],
    fl: uint4,
    addr: &Address,
    write: &[VarnodeId],
    marks: &mut BTreeMap<usize, Vec<Address>>,
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
    for &head in heads {
        let index = fd.bblocks_ref().block(head).get_index() as usize;
        marks.entry(index).or_default().push(addr.clone());
    }
}

/// Make the writes behind each value in `reaching`, the definitions renaming
/// had current at a marked block's entry, `addrforce`, which `ActionDeadCode`
/// keeps alive with what they read. A MULTIEQUAL is looked through to the
/// writes it joins rather than marked itself: the block holding it may do
/// nothing once dead code is gone, and removing that block drops it. An input
/// the function never wrote needs nothing.
pub(crate) fn mark(fd: &mut Funcdata, reaching: &[VarnodeId]) {
    let mut seen: FxHashSet<VarnodeId> = FxHashSet::default();
    let mut work: Vec<VarnodeId> = reaching.to_vec();
    while let Some(vn) = work.pop() {
        if !seen.insert(vn) {
            continue;
        }
        let Some(def) = fd.vbank().get(vn).and_then(|v| v.get_def()) else { continue };
        let Some(op) = fd.obank().get(def) else { continue };
        if op.code() == OpCode::CPUI_MULTIEQUAL {
            work.extend((0..op.num_input()).filter_map(|i| op.get_in(i)));
            continue;
        }
        if let Some(v) = fd.vbank_mut().get_mut(vn) {
            v.set_addr_force();
        }
    }
}
