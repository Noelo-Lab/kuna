//! (kuna) The lane traces `ActionLaneDivide` may skip when it rescans a
//! storage location after a split.
//!
//! Upstream restarts the scan of a laned storage location from its first
//! Varnode after every successful split, so every Varnode that failed before is
//! traced again; a function with a thousand splits traces each failing Varnode
//! a thousand times. A failed trace changes nothing, and it reads only the
//! Varnodes it reached and the ops that define and read them. A split rewrites
//! only the Varnodes its own trace reached and those ops. So a failed trace
//! that read none of the Varnodes a later split's trace read or touched fails
//! again in the same way, and is not traced again.

use crate::context::VarnodeId;
use crate::funcdata::Funcdata;
use crate::kuna_fasthash::{FxHashMap, FxHashSet};

/// The Varnodes a trace from `root` that reached `reached` read: `root`,
/// every Varnode reached, and every input and output of the ops defining and
/// reading them.
pub(crate) fn footprint(
    data: &Funcdata,
    root: VarnodeId,
    reached: impl IntoIterator<Item = VarnodeId>,
    out: &mut FxHashSet<VarnodeId>,
) {
    for vn in std::iter::once(root).chain(reached) {
        if !out.insert(vn) {
            continue;
        }
        let Some(v) = data.vbank().get(vn) else {
            continue;
        };
        for op in v.get_def().into_iter().chain(v.descend_iter()) {
            let Some(o) = data.obank().get(op) else {
                continue;
            };
            out.extend(o.get_out());
            out.extend((0..o.num_input()).filter_map(|i| o.get_in(i)));
        }
    }
}

/// The failed traces of one storage location in one mode, and the Varnodes
/// each one read.
#[derive(Default)]
pub(crate) struct FailedTraces {
    failed: FxHashSet<VarnodeId>,
    readers: FxHashMap<VarnodeId, Vec<VarnodeId>>,
}

impl FailedTraces {
    /// Would a trace from `root` fail as it did before?
    pub(crate) fn holds(&self, root: VarnodeId) -> bool {
        self.failed.contains(&root)
    }

    /// The trace from `root` failed after reading `footprint`.
    pub(crate) fn record(&mut self, root: VarnodeId, footprint: FxHashSet<VarnodeId>) {
        self.failed.insert(root);
        for vn in footprint {
            self.readers.entry(vn).or_default().push(root);
        }
    }

    /// The IR changed at `footprint`: forget every failure that read it.
    pub(crate) fn invalidate(&mut self, footprint: &FxHashSet<VarnodeId>) {
        for vn in footprint {
            if let Some(roots) = self.readers.remove(vn) {
                for root in roots {
                    self.failed.remove(&root);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use slotmap::KeyData;

    fn vn(n: u64) -> VarnodeId {
        VarnodeId::from(KeyData::from_ffi(n))
    }

    fn set(ids: &[u64]) -> FxHashSet<VarnodeId> {
        ids.iter().map(|&n| vn(n)).collect()
    }

    #[test]
    fn a_failure_holds_until_a_split_touches_what_it_read() {
        let mut failed = FailedTraces::default();
        failed.record(vn(1), set(&[1, 2, 3]));
        failed.record(vn(4), set(&[4, 5]));
        assert!(failed.holds(vn(1)) && failed.holds(vn(4)));
        failed.invalidate(&set(&[6, 7]));
        assert!(failed.holds(vn(1)) && failed.holds(vn(4)));
        failed.invalidate(&set(&[7, 3]));
        assert!(!failed.holds(vn(1)));
        assert!(failed.holds(vn(4)));
    }

    #[test]
    fn a_retraced_failure_is_held_again() {
        let mut failed = FailedTraces::default();
        failed.record(vn(1), set(&[1, 2]));
        failed.invalidate(&set(&[2]));
        assert!(!failed.holds(vn(1)));
        failed.record(vn(1), set(&[1, 8]));
        failed.invalidate(&set(&[2]));
        assert!(failed.holds(vn(1)));
        failed.invalidate(&set(&[8]));
        assert!(!failed.holds(vn(1)));
    }
}
