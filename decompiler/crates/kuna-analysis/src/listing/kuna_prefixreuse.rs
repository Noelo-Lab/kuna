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
    state.carry.discard(&removed);
    state.skips.retain(|(from, _, _, _)| !removed.contains(from));
    state
        .switches
        .retain(|table| !removed.contains(&table.dispatch));
    if let Some(flow) = &mut state.flow {
        flow.insns.retain(|(at, _, _)| !removed.contains(at));
        flow.jumps.retain(|(at, _)| !removed.contains(at));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::listing::kuna_framemode::FrameModes;

    /// A condition skip held by a body the reconciliation discards goes with
    /// that body, as its filed rows do.
    #[test]
    fn a_discarded_body_takes_its_held_skips() {
        let mut modes = FrameModes::new(&[0x1000]);
        modes.record(0x1000, 0x1000, 4, Some(0));
        modes.record(0x1000, 0x1004, 4, Some(0));
        let mut state = State::new(&BTreeSet::from([0x1000, 0x2000]), false);
        state.skips.push((0x1004, 0x1008, 0x1000, Some(0)));
        state.skips.push((0x2004, 0x2008, 0x2000, Some(0)));
        let mut partition = vec![(0x1000, 4), (0x1004, 4), (0x2004, 4)];
        let replacements = BTreeMap::from([(0x1000, 0xff0)]);
        discard_bodies(&mut state, &mut partition, &mut BTreeMap::new(), &mut None, &modes, &replacements);
        assert_eq!(partition, vec![(0x2004, 4)]);
        assert_eq!(state.skips, vec![(0x2004, 0x2008, 0x2000, Some(0))]);
    }
}
