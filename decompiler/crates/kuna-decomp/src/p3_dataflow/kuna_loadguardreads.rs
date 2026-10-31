//! (kuna `loadguardreads`) Index-alias LOAD guards read while renaming
//! instead of through one COPY per guarded range per LOAD.
//!
//! Upstream `Heritage::guardLoads` puts an address-forced `COPY` of every
//! guarded range in front of every LOAD that may alias it, renames them with
//! the rest of the pass, lets `handleNewLoadCopies` mark the writes reaching
//! them address forced, and propagates every one away before the pass ends.
//! A new guard covers the whole stack until value-set analysis bounds it after
//! renaming, so the pass that first meets the indexed LOADs builds one `COPY`
//! and two Varnodes per LOAD per address-tied range. Here the pass records the
//! pairs, renaming reads the top of each pair's stack at the LOAD exactly as it
//! would read the `COPY`'s input, and each distinct Varnode reached gets one
//! guard `COPY` after renaming; the writes the walk from it marks are the ones
//! upstream's COPYs mark, and it is propagated away like them. A read that
//! upstream renames to a guard `COPY`'s output moves, at the end of the pass,
//! to the back of its Varnode's readers, where propagating that `COPY` away
//! puts it, so later passes visit readers in the same order.

use crate::context::{OpId, VarnodeId};
use crate::funcdata::Funcdata;
use crate::kuna_fasthash::{FxHashMap, FxHashSet};
use kuna_base::address::Address;
use kuna_base::types::int4;

/// The LOAD guard reads of one heritage pass.
#[derive(Debug, Default)]
pub(crate) struct Reads {
    ranges: Vec<(Address, int4)>,
    by_op: FxHashMap<OpId, Vec<(u32, u32)>>,
    pairs: u32,
    reached: Vec<(VarnodeId, OpId, u32)>,
    seen: FxHashSet<VarnodeId>,
    behind: std::collections::BTreeMap<Address, Vec<(usize, u32)>>,
    later: Vec<(u32, OpId, int4)>,
}

impl Reads {
    pub(crate) fn enabled(fd: &Funcdata) -> bool {
        fd.get_arch().load_guard_reads
    }

    /// Record the range `addr`/`size` and return its index for [`Reads::guard`].
    pub(crate) fn add_range(&mut self, addr: &Address, size: int4) -> u32 {
        self.ranges.push((addr.clone(), size));
        (self.ranges.len() - 1) as u32
    }

    /// The LOAD `op` reads range `range` just before it executes. Pairs are
    /// numbered in the order upstream builds their COPYs.
    pub(crate) fn guard(&mut self, op: OpId, range: u32) {
        self.by_op.entry(op).or_default().push((range, self.pairs));
        self.pairs += 1;
    }

    /// Did this pass guard any LOAD?
    pub(crate) fn active(&self) -> bool {
        self.pairs > 0
    }

    /// Is a guarded LOAD still to be renamed?
    pub(crate) fn has_guards(&self) -> bool {
        !self.by_op.is_empty()
    }

    /// The ranges the LOAD `op` reads and their pair numbers, in the order
    /// they were guarded.
    pub(crate) fn take(&mut self, op: OpId) -> Option<Vec<(u32, u32)>> {
        self.by_op.remove(&op)
    }

    pub(crate) fn range(&self, index: u32) -> &(Address, int4) {
        &self.ranges[index as usize]
    }

    /// Renaming linked the read of range `range` at the LOAD `op` to `vn`, the
    /// entry `depth` of its stack holds from below; reads of that entry until
    /// it is popped read upstream's COPY for `pair` instead.
    pub(crate) fn reach(&mut self, vn: VarnodeId, op: OpId, range: u32, pair: u32, depth: usize) {
        if self.seen.insert(vn) {
            self.reached.push((vn, op, range));
        }
        let key = self.ranges[range as usize].0.clone();
        self.behind.entry(key).or_default().push((depth, pair));
    }

    /// The block that pushed a [`Reads::reach`] entry for `key` is done.
    pub(crate) fn pop(&mut self, key: &Address) {
        if let Some(entries) = self.behind.get_mut(key) {
            entries.pop();
        }
    }

    /// Renaming linked `slot` of `op` to the entry `depth` of the stack for
    /// `key` holds from below. When an upstream guard COPY would have stood
    /// between them, the read moves to the back of the Varnode's readers at
    /// the end of the pass, where propagating that COPY away puts it.
    pub(crate) fn read(&mut self, key: &Address, depth: usize, op: OpId, slot: int4) {
        let Some(entries) = self.behind.get(key) else {
            return;
        };
        if let Some(&(d, pair)) = entries.iter().rev().find(|(d, _)| *d <= depth) {
            if d == depth {
                self.later.push((pair, op, slot));
            }
        }
    }

    pub(crate) fn take_reached(&mut self) -> Vec<(VarnodeId, OpId, u32)> {
        self.seen.clear();
        std::mem::take(&mut self.reached)
    }

    /// The reads [`Reads::read`] deferred, in upstream's propagation order.
    pub(crate) fn take_later(&mut self) -> Vec<(u32, OpId, int4)> {
        let mut later = std::mem::take(&mut self.later);
        later.sort_by_key(|&(pair, _, _)| pair);
        later
    }

    pub(crate) fn clear(&mut self) {
        self.ranges.clear();
        self.by_op.clear();
        self.pairs = 0;
        self.reached.clear();
        self.seen.clear();
        self.behind.clear();
        self.later.clear();
    }
}
