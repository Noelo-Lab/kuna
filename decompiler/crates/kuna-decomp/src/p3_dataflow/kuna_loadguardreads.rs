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
//! upstream's COPYs mark, and it is propagated away like them. The `COPY` of a
//! pair that a later read would see on top of the stack is built when renaming
//! meets that read, which reads it as it reads upstream's: the steps between
//! renaming and propagation see the same reader lists, and propagating the
//! COPYs in upstream's order leaves every Varnode's readers in upstream's order.

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
    behind: std::collections::BTreeMap<Address, (u32, Vec<(u32, u32, OpId)>)>,
    copies: FxHashMap<u32, VarnodeId>,
    built: Vec<(u32, OpId)>,
}

/// A guard read a later read of the range sees on top of the stack.
pub(crate) struct Behind {
    pub(crate) pair: u32,
    pub(crate) range: u32,
    pub(crate) load: OpId,
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
        self.behind
            .entry(key)
            .or_insert_with(|| (range, Vec::new()))
            .1
            .push((depth as u32, pair, op));
    }

    /// The block that pushed a [`Reads::reach`] entry for `key` is done.
    pub(crate) fn pop(&mut self, key: &Address) {
        if let Some((_, entries)) = self.behind.get_mut(key) {
            entries.pop();
        }
    }

    /// Renaming links a read of `key` to the entry `depth` of its stack holds
    /// from below. The guard read upstream's COPY for it would read instead,
    /// when one stands between them.
    pub(crate) fn behind(&self, key: &Address, depth: usize) -> Option<Behind> {
        let (range, entries) = self.behind.get(key)?;
        let &(d, pair, load) = entries
            .iter()
            .rev()
            .find(|(d, _, _)| *d as usize <= depth)?;
        (d as usize == depth).then_some(Behind {
            pair,
            range: *range,
            load,
        })
    }

    /// The output of the COPY built for `pair`, if any.
    pub(crate) fn copy(&self, pair: u32) -> Option<VarnodeId> {
        self.copies.get(&pair).copied()
    }

    /// The COPY `op` writing `out` was built for `pair`.
    pub(crate) fn built(&mut self, pair: u32, out: VarnodeId, op: OpId) {
        self.copies.insert(pair, out);
        self.built.push((pair, op));
    }

    pub(crate) fn take_reached(&mut self) -> Vec<(VarnodeId, OpId, u32)> {
        self.seen.clear();
        std::mem::take(&mut self.reached)
    }

    /// The COPYs [`Reads::built`] recorded, in upstream's propagation order.
    pub(crate) fn take_built(&mut self) -> Vec<OpId> {
        let mut built = std::mem::take(&mut self.built);
        built.sort_unstable_by_key(|&(pair, _)| pair);
        built.into_iter().map(|(_, op)| op).collect()
    }

    pub(crate) fn clear(&mut self) {
        self.ranges.clear();
        self.by_op.clear();
        self.pairs = 0;
        self.reached.clear();
        self.seen.clear();
        self.behind.clear();
        self.copies.clear();
        self.built.clear();
    }
}
