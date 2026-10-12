//! Call evidence reachable independently of provisional frame bodies.

use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::rc::Rc;

use super::ProbedInsn;
use crate::listing::Listing;

/// Follow calls and control flow from established roots and validated prefixes.
/// A cycle with no path from these roots cannot corroborate its own entries.
pub(super) fn independently_called(
    corpus: &Listing,
    roots: impl IntoIterator<Item = u64>,
    prefixes: &BTreeMap<u64, Rc<ProbedInsn>>,
) -> BTreeSet<u64> {
    let mut pending: Vec<_> = roots.into_iter().collect();
    let mut seen = HashSet::new();
    let mut called = BTreeSet::new();
    while let Some(at) = pending.pop() {
        if !seen.insert(at) {
            continue;
        }
        let (fall, flows, is_call) = if let Some(insn) = prefixes.get(&at) {
            (insn.fall_through, &insn.flows, insn.is_call)
        } else if let Some(insn) = corpus.instruction_at(at) {
            (insn.fall_through, &insn.flows, insn.flow.is_call)
        } else {
            continue;
        };
        pending.extend(fall);
        pending.extend(flows.iter().copied());
        if is_call {
            called.extend(flows.iter().copied());
        }
    }
    called
}
