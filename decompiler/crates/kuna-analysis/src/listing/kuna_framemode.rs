//! Retract heuristic bodies contradicted by a subsequent direct call's ISA mode.

use std::collections::{BTreeMap, BTreeSet, HashSet};
pub(crate) struct FrameModes<'a> {
    roots: HashSet<u64>,
    spans: BTreeMap<u64, Span>,
    stale: BTreeSet<u64>,
    established: Option<&'a super::Listing>,
}

struct Span {
    end: u64,
    root: u64,
    mode: u32,
}

impl<'a> FrameModes<'a> {
    pub fn new(roots: &[u64]) -> Self {
        Self {
            roots: roots.iter().copied().collect(),
            spans: BTreeMap::new(),
            stale: BTreeSet::new(),
            established: None,
        }
    }

    /// Inventory rebuilds retain independently decoded code and references.
    pub fn preserving(roots: &[u64], established: &'a super::Listing) -> Self {
        Self {
            established: Some(established),
            ..Self::new(roots)
        }
    }

    pub fn established(&self) -> Option<&super::Listing> {
        self.established
    }

    pub fn record(&mut self, root: u64, at: u64, len: u32, mode: Option<u32>) {
        if !self.roots.contains(&root) {
            return;
        }
        let Some(mode) = mode else { return };
        let end = at.saturating_add(u64::from(len));
        if let Some((_, span)) = self.spans.range_mut(..at).next_back() {
            if span.end == at && span.root == root && span.mode == mode {
                span.end = end;
                return;
            }
        }
        self.spans.insert(at, Span { end, root, mode });
    }

    pub fn called(&mut self, target: u64, mode: Option<u32>) {
        let Some((_, span)) = self.spans.range(..=target).next_back() else {
            return;
        };
        if target < span.end && mode.is_some_and(|mode| mode != span.mode) {
            self.stale.insert(span.root);
        }
    }

    pub fn decoded_as_arm(&self, replacements: &BTreeMap<u64, u64>) -> bool {
        self.spans
            .values()
            .all(|span| !replacements.contains_key(&span.root) || span.mode == 0)
            && replacements.keys().all(|root| {
                self.spans
                    .get(root)
                    .is_some_and(|span| span.root == *root && span.mode == 0)
            })
    }

    pub fn replace(&mut self, replacements: &BTreeMap<u64, u64>) {
        self.roots.retain(|root| !replacements.contains_key(root));
        self.roots.extend(replacements.values().copied());
        self.spans
            .retain(|_, span| !replacements.contains_key(&span.root));
    }

    pub fn in_replaced_body(&self, at: u64, replacements: &BTreeMap<u64, u64>) -> bool {
        self.spans
            .range(..=at)
            .next_back()
            .is_some_and(|(_, span)| at < span.end && replacements.contains_key(&span.root))
    }

    pub fn stale(&self) -> &BTreeSet<u64> {
        &self.stale
    }
}
