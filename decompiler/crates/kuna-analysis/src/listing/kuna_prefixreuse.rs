//! Rewalk an extended ARM caller without decoding unrelated functions again.

use super::*;

pub(super) fn discard_bodies(
    state: &mut State,
    partition: &mut Vec<(u64, u32)>,
    spans: &mut BTreeMap<u64, u32>,
    contexts: &mut Option<crate::listing::kuna_fingerprintcontext::FingerprintContexts>,
    modes: &crate::listing::kuna_framemode::FrameModes,
    replacements: &BTreeMap<u64, u64>,
) {
    let removed: HashSet<_> = partition
        .iter()
        .filter_map(|&(at, _)| modes.in_replaced_body(at, replacements).then_some(at))
        .collect();
    if let Some(contexts) = contexts {
        contexts.retain(partition.iter().map(|(at, _)| !removed.contains(at)));
    }
    partition.retain(|(at, _)| !removed.contains(at));
    spans.retain(|at, _| !removed.contains(at));
    state.decoded.retain(|at| !removed.contains(at));
    state.by_source.retain(|at, _| !removed.contains(at));
    state.by_target.retain(|_, refs| {
        refs.retain(|r| !removed.contains(&r.from));
        !refs.is_empty()
    });
    state.indirect_call_sites.retain(|at| !removed.contains(at));
    state
        .switches
        .retain(|table| !removed.contains(&table.dispatch));
    if let Some(flow) = &mut state.flow {
        flow.insns.retain(|(at, _, _)| !removed.contains(at));
        flow.jumps.retain(|(at, _)| !removed.contains(at));
    }
}
