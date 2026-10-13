//! Keep the inventory that `armframes` starts from. Frame recovery re-walks the
//! Listing from new roots, and a root's walk absorbs a routine it reaches by a
//! tail branch; its AIF prefix scan can also trade a root for literal-pool words
//! in front of it. Neither is evidence against an entry the frameless inventory
//! reports, so that inventory is computed first and survives unless the
//! frame-expanded Listing contradicts an entry.

use std::collections::BTreeSet;
use std::rc::Rc;

use kuna_base::space::AddrSpace;
use kuna_decomp::architecture::Architecture;
use kuna_sleigh::translate::Translate;

use super::{kuna_aifbracket::BracketEvidence, kuna_framecache::ArmFrames, GapDecoder, MAX_FOLLOW_INSNS};
use crate::listing::{FlowKind, Listing, RefKind};

/// The frameless inventory: the walk after the Thumb prologue scan (`walked`)
/// plus the AIF, pool and pointer entries over it (`entries`, a superset).
pub(crate) struct Prior {
    walked: BTreeSet<u64>,
    entries: BTreeSet<u64>,
}

/// Compute the inventory the default pipeline reports, without keeping its walk
/// or decode context. `rebuild` walks the Listing from a new seed set.
#[allow(clippy::too_many_arguments)]
pub(crate) fn prior_inventory(
    file: &object::File,
    listing: &Listing,
    seeds: &[u64],
    rebuild: impl FnOnce(&[u64]) -> Listing,
    arch: &Architecture,
    translate: &dyn Translate,
    code_space: &Rc<AddrSpace>,
    bracket: Option<&BracketEvidence>,
) -> Prior {
    let _scope = translate.context_scope();
    let mut thumb = seeds.to_vec();
    thumb.extend(super::raw_thumb_prologue_seeds(
        file, listing, translate, Rc::clone(code_space), listing.exec_ranges(),
    ));
    thumb.sort_unstable();
    thumb.dedup();
    let rebuilt = (thumb.len() != seeds.len()).then(|| rebuild(&thumb));
    let base = rebuilt.as_ref().unwrap_or(listing);
    let walked: BTreeSet<u64> = base.functions().map(|(&entry, _)| entry).collect();
    let mut entries = walked.clone();
    if arch.analysis_aif {
        let pointers = if arch.analysis_ptrentry {
            super::kuna_ptrentry::pointer_entry_seeds(
                file, base, translate, Rc::clone(code_space), base.exec_ranges(),
            )
        } else {
            Vec::new()
        };
        let checked = super::kuna_entrychecks::run(
            base, None, arch, translate, Rc::clone(code_space), pointers, bracket,
        );
        entries.extend(checked.entries);
        entries.extend(checked.pool_entries);
        entries.extend(checked.pointer_entries);
    }
    Prior { walked, entries }
}

impl Prior {
    /// True iff a walked `root` keeps its place against `prefix`. A prefix in
    /// front of a walked entry is no stronger evidence than the walk that found
    /// it, and dense Thumb encodings decode literal-pool words as such a prefix;
    /// only a root inside one of the prefix's instructions yields.
    pub(crate) fn shields(&self, root: u64, prefix: u64, frames: &ArmFrames) -> bool {
        self.walked.contains(&root) && !frames.splits(root, prefix)
    }

    /// Undo the prefix replacements of `frames`' first reconciliation that
    /// [`Self::shields`] refuses.
    pub(crate) fn restore(&self, roots: &mut BTreeSet<u64>, frames: &ArmFrames) {
        for (&root, &prefix) in &frames.replaced {
            if !self.shields(root, prefix, frames) {
                continue;
            }
            roots.insert(root);
            let orphan = frames.replaced.iter()
                .all(|(&other, &to)| to != prefix || self.shields(other, to, frames));
            if orphan && !self.entries.contains(&prefix) {
                roots.remove(&prefix);
            }
        }
    }

    /// The prior entries missing from `found` that the final Listing does not
    /// contradict, that do not start predicated, that the post-frame passes did
    /// not refute (`refuted`: a contradicted decode mode, or an accepted AIF body
    /// running into the entry or a refused one covering it), and from which no
    /// flow joins a walked function's interior.
    pub(crate) fn retained(
        &self,
        found: &BTreeSet<u64>,
        refuted: &BTreeSet<u64>,
        listing: &Listing,
        translate: &dyn Translate,
        code_space: Rc<AddrSpace>,
    ) -> Vec<u64> {
        let _probe = super::arm_gap_probe_scope(translate);
        let mut decoder = GapDecoder::new(translate, code_space, listing.exec_ranges());
        let known: BTreeSet<u64> = self.entries.union(found).copied().collect();
        self.entries
            .iter()
            .copied()
            .filter(|&entry| {
                !found.contains(&entry)
                    && !refuted.contains(&entry)
                    && listing.function_at(entry).is_none()
                    && !contradicted(listing, entry)
                    && !predicated(&mut decoder, listing, entry)
                    && !(listing.is_undefined(entry) && joins_interior(&mut decoder, listing, entry, &known))
            })
            .collect()
    }
}

/// True iff the frame-expanded Listing shows `entry` is not a function start: it
/// is data, it lies inside a decoded instruction, or straight-line flow (a
/// fall-through or a conditional branch) runs into it.
fn contradicted(listing: &Listing, entry: u64) -> bool {
    if listing.is_undefined(entry) {
        return false;
    }
    if !listing.is_instruction_start(entry) {
        return true;
    }
    if listing.instruction_before(entry).is_some_and(|insn| insn.fall_through == Some(entry)) {
        return true;
    }
    listing.refs_to(entry).iter().any(|r| {
        r.kind == RefKind::Code
            && listing.instruction_at(r.from).is_some_and(|insn| {
                insn.fall_through == Some(entry)
                    || (insn.flows.contains(&entry) && insn.flow.kind != FlowKind::UnconditionalBranch)
            })
    })
}

/// True iff the instruction at `entry` is predicated: one of its flows is its own
/// fall-through, a conditional skip of itself. Condition flags are undefined on
/// entry, so no function starts there; literal-pool words before a frame decode so.
fn predicated(decoder: &mut GapDecoder, listing: &Listing, entry: u64) -> bool {
    if let Some(insn) = listing.instruction_at(entry) {
        return !insn.flow.is_call && insn.fall_through.is_some_and(|next| insn.flows.contains(&next));
    }
    decoder.probe(entry).is_some_and(|insn| {
        !insn.is_call && insn.fall_through.is_some_and(|next| insn.flows.contains(&next))
    })
}

/// True iff non-call flow from the undefined `entry` reaches decoded code that is
/// no function entry: the bytes are a fragment of a walked function, such as a
/// switch case its unresolved computed jump leaves undecoded.
fn joins_interior(
    decoder: &mut GapDecoder,
    listing: &Listing,
    entry: u64,
    known: &BTreeSet<u64>,
) -> bool {
    let mut seen = BTreeSet::new();
    let mut work = vec![entry];
    while let Some(at) = work.pop() {
        if seen.len() >= MAX_FOLLOW_INSNS || !seen.insert(at) {
            continue;
        }
        if !listing.is_undefined(at) {
            if !listing.is_data(at) && listing.function_at(at).is_none() && !known.contains(&at) {
                return true;
            }
            continue;
        }
        let Some(insn) = decoder.probe(at) else { continue };
        work.extend(insn.fall_through);
        if !insn.is_call {
            work.extend(insn.flows.iter().copied());
        }
    }
    false
}
