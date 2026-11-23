//! (kuna) `falseentry` — a discovered entry that its neighbour's own code runs
//! into or branches to is not a function entry (P1 code/data partition).
//!
//! The speculative entry oracles of a stripped image (the `<patternpairs>`
//! prologue scan, the ARM frame scan, the `aif` gap walk) judge a candidate by
//! its first bytes. A shrink-wrapped function pushes its frame after an early
//! test (`cmp; bgt out; push {r4,r5,lr}`), a variadic one saves its argument
//! registers before the frame push, a switch case or a block moved past the
//! return opens with a push, and a function whose prologue fingerprint is rare
//! is skipped by the gap walk while the next block inside it is accepted. Each
//! plants an entry in the middle of a real function, and the decompiler stops
//! that function's flow there (`funcboundflow`).
//!
//! [`interior_entries`] reads the finished partition. An unattested candidate is
//! interior when
//!
//! 1. **run-in:** a live instruction that is not a call, a user operation,
//!    padding or zero fill ends exactly at it and continues there, by falling
//!    through or as the skip of a predicated instruction (a `b` to the next
//!    address is a tail call and does not count);
//! 2. **local branch:** the flow of the surviving entry before it, entering no
//!    callee and stepping onto no other survivor, has a conditional branch or a
//!    switch case to it; or
//! 3. **run-in past:** that flow runs into it and also branches past it, at or
//!    before the next survivor (a shrink-wrapped function whose real entry only a
//!    speculative oracle found).
//!
//! Facts 2 and 3 read the preceding entry's own flow, so a function whose first
//! instruction heads a loop keeps its entry. Code is live when an attested entry
//! reaches it, or else a code pointer, over static targets and fall-throughs
//! that provably continue (the [`super::kuna_poolextents`] trust notion, taking
//! an indirect call and a register tail jump to return), which keeps a pool
//! after a call to an unproven `panic` from running into the next function. Attested entries (a symbol, an oracle reading the image, a direct
//! call from any candidate's code) are never dropped. A code pointer no
//! candidate names whose code runs into an interior entry becomes the entry in
//! its place.

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::rc::Rc;

use kuna_base::space::AddrSpace;
use kuna_sleigh::translate::Translate;

use super::{FlowKind, Listing, RefKind};

/// Instruction cap on one entry's local-branch flow (`followSubFlows`' 4000).
const MAX_REGION_INSNS: usize = 4000;

/// One decoded instruction, as the interior test reads it.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Step {
    pub len: u32,
    pub kind: FlowKind,
    pub is_call: bool,
    pub fall_through: Option<u64>,
    pub flows: Vec<u64>,
    /// Flow provably continues into `fall_through` (see [`super::kuna_poolextents`]).
    pub continues: bool,
    /// Alignment padding or zero fill.
    pub filler: bool,
    /// The case targets of a switch dispatch.
    pub cases: Vec<u64>,
}

impl Step {
    fn end(&self, at: u64) -> u64 {
        at.wrapping_add(u64::from(self.len))
    }

    /// The address this instruction runs into without a transfer, if any.
    fn runs_into(&self, at: u64) -> Option<u64> {
        if self.is_call || self.kind == FlowKind::CallOther || self.filler || self.len == 0 {
            return None;
        }
        let end = self.end(at);
        let skips = self.kind != FlowKind::UnconditionalBranch && self.flows.contains(&end);
        (self.fall_through == Some(end) || skips).then_some(end)
    }

    /// Live successors: static targets of branches and calls, and the proven
    /// fall-through. `calls` says whether call targets are followed.
    fn successors(&self, calls: bool) -> impl Iterator<Item = u64> + '_ {
        let targets = (calls || !self.is_call).then_some(self.flows.iter().copied());
        self.fall_through
            .filter(|_| self.continues)
            .into_iter()
            .chain(targets.into_iter().flatten())
            .chain(self.cases.iter().copied())
    }
}

/// Which fact made an entry interior, with the instruction that shows it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reason {
    RunIn(u64),
    LocalBranch(u64),
    RunInPast(u64),
}

/// What [`interior_entries`] decided.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Verdict {
    /// The candidates that are another function's code, with the evidence.
    pub interior: BTreeMap<u64, Reason>,
    /// Pointer targets no candidate names whose code runs into an interior
    /// entry: the start of the function that entry was the middle of.
    pub entries: BTreeSet<u64>,
}

/// The entries of `candidates` that another function's live code runs into or
/// branches to locally. `step` reads one instruction (`None` where nothing
/// decodes); `vouched` names the entries the image itself attests. Code is live
/// when a vouched candidate reaches it, or else one of the code pointers
/// `pointed` does.
pub fn interior_entries(
    candidates: &BTreeSet<u64>,
    pointed: &BTreeSet<u64>,
    vouched: impl Fn(u64) -> bool,
    mut step: impl FnMut(u64) -> Option<Step>,
) -> Verdict {
    let mut cache: BTreeMap<u64, Option<Step>> = BTreeMap::new();
    let mut read = |at: u64| -> Option<Step> { cache.entry(at).or_insert_with(|| step(at)).clone() };

    let mut called: BTreeSet<u64> = BTreeSet::new();
    let mut seen: BTreeSet<u64> = BTreeSet::new();
    let mut work: Vec<u64> = candidates.iter().copied().collect();
    while let Some(at) = work.pop() {
        if !seen.insert(at) {
            continue;
        }
        let Some(insn) = read(at) else { continue };
        if insn.is_call {
            called.extend(insn.flows.iter().copied());
        }
        work.extend(insn.fall_through.into_iter().chain(insn.flows.iter().copied()).chain(insn.cases.iter().copied())
            .filter(|t| !seen.contains(t)));
    }
    let vouched = |entry: u64| vouched(entry) || called.contains(&entry);

    let mut interior: BTreeMap<u64, Reason> = BTreeMap::new();
    let mut via: BTreeMap<u64, u64> = BTreeMap::new();
    let mut ran_into: BTreeSet<u64> = BTreeSet::new();
    let mut live: BTreeSet<u64> = BTreeSet::new();
    let attested_roots: Vec<u64> = candidates.iter().copied().filter(|&e| vouched(e)).collect();
    for root in attested_roots.into_iter().chain(pointed.iter().copied()) {
        let mut work = vec![root];
        while let Some(at) = work.pop() {
            if !live.insert(at) {
                continue;
            }
            let Some(insn) = read(at) else { continue };
            if let Some(next) = insn.runs_into(at) {
                ran_into.insert(next);
                if next != at && candidates.contains(&next) && !vouched(next) && !interior.contains_key(&next) {
                    interior.insert(next, Reason::RunIn(at));
                    via.insert(next, root);
                }
            }
            work.extend(insn.successors(true).filter(|t| !live.contains(t)));
        }
    }
    let entries: BTreeSet<u64> =
        via.values().copied().filter(|r| !candidates.contains(r) && !ran_into.contains(r)).collect();
    let vouched = |entry: u64| vouched(entry) || entries.contains(&entry);

    let mut survivors: BTreeSet<u64> =
        candidates.iter().chain(&entries).copied().filter(|e| !interior.contains_key(e)).collect();
    let owners: Vec<u64> = survivors.iter().copied().collect();
    for owner in owners {
        if !survivors.contains(&owner) {
            continue;
        }
        let mut region = Region::new(owner);
        region.grow(&survivors, &mut read);
        while let Some(&entry) = survivors.range(owner + 1..).next() {
            if vouched(entry) {
                break;
            }
            let upper = survivors.range(entry + 1..).next().copied().unwrap_or(u64::MAX);
            let reason = if let Some(&site) = region.branches_to.get(&entry) {
                Reason::LocalBranch(site)
            } else if let Some(&site) = region.runs_into.get(&entry).filter(|_| region.passes(entry, upper)) {
                Reason::RunInPast(site)
            } else {
                break;
            };
            interior.insert(entry, reason);
            survivors.remove(&entry);
            region.work.push_back(entry);
            region.grow(&survivors, &mut read);
        }
    }
    Verdict { interior, entries }
}

/// One entry's own flow: it enters no callee and steps onto no other survivor.
/// An entry that turns out to be its code is absorbed and the flow resumes.
///
/// The entry right after the owner is its code when the flow branches to it
/// conditionally, or runs into it while also branching past it into its region
/// `(entry, upper]`. The second form is a shrink-wrapped function whose real
/// entry only a speculative oracle found: the early exit jumps over the frame
/// push. A function whose first instruction heads a loop keeps its entry, since
/// its loop branch is reached only through that entry.
struct Region {
    owner: u64,
    seen: BTreeSet<u64>,
    work: VecDeque<u64>,
    branches_to: BTreeMap<u64, u64>,
    runs_into: BTreeMap<u64, u64>,
    targets: BTreeSet<u64>,
}

impl Region {
    fn new(owner: u64) -> Region {
        Region {
            owner,
            seen: BTreeSet::new(),
            work: VecDeque::from([owner]),
            branches_to: BTreeMap::new(),
            runs_into: BTreeMap::new(),
            targets: BTreeSet::new(),
        }
    }

    fn grow(&mut self, survivors: &BTreeSet<u64>, read: &mut impl FnMut(u64) -> Option<Step>) {
        while let Some(at) = self.work.pop_front() {
            if self.seen.len() >= MAX_REGION_INSNS || !self.seen.insert(at) {
                continue;
            }
            let Some(insn) = read(at) else { continue };
            if !insn.is_call {
                if insn.kind == FlowKind::ConditionalBranch {
                    for &t in &insn.flows {
                        self.branches_to.entry(t).or_insert(at);
                    }
                }
                for &t in &insn.cases {
                    self.branches_to.entry(t).or_insert(at);
                }
                self.targets.extend(insn.flows.iter().chain(&insn.cases).copied());
            }
            if let Some(next) = insn.runs_into(at) {
                self.runs_into.entry(next).or_insert(at);
            }
            let owner = self.owner;
            self.work.extend(insn.successors(false).filter(|&next| next == owner || !survivors.contains(&next)));
        }
    }

    fn passes(&self, entry: u64, upper: u64) -> bool {
        self.targets.range(entry + 1..=upper).next().is_some()
    }
}

/// The interior entries of `candidates` over the finished `listing`.
///
/// `vouched` holds the entries an oracle read from the image (symbols, entry
/// oracles, code pointers, call tables); a direct call the listing decoded
/// vouches for its target as well. `pools` are the literal pools the listing
/// proves and `no_return` names callees whose call does not continue.
///
/// A switch dispatch's cases are the targets the walk resolved for it, or the
/// run of Thumb code pointers stored right after it (`ldr.w pc,[r2,r3,lsl #2]`
/// followed by its table).
#[allow(clippy::too_many_arguments)]
pub(crate) fn scan(
    file: &object::File,
    listing: &Listing,
    translate: &dyn Translate,
    code_space: &Rc<AddrSpace>,
    candidates: &BTreeSet<u64>,
    vouched: &BTreeSet<u64>,
    pools: &[(u64, u64)],
    no_return: &BTreeSet<u64>,
) -> Verdict {
    let _probe = crate::aif::arm_gap_probe_scope(translate);
    let wide = code_space.get_addr_size() > 4;
    let sites = code_pointers(file, listing);
    let pointed = pointer_roots(listing, &sites, pools, candidates);
    let words: BTreeMap<u64, u64> =
        sites.iter().flat_map(|(&target, at)| at.iter().map(move |&site| (site, target))).collect();
    let inline_table = |at: u64, len: u32| -> Vec<u64> {
        let mut slot = at.wrapping_add(u64::from(len)).wrapping_add(3) & !3;
        let mut cases = Vec::new();
        while let Some(&target) = words.get(&slot) {
            cases.push(target);
            slot += 4;
        }
        cases
    };
    let dispatch = |is_computed: bool, is_jump: bool, is_call: bool| is_computed && is_jump && !is_call;
    let mut returns = super::kuna_poolextents::returning_entries_with(listing, true);
    returns.retain(|callee| !no_return.contains(callee));
    let attested = |entry: u64| {
        vouched.contains(&entry)
            || listing.function_at(entry).is_some_and(|f| f.from_symbol)
            || listing.refs_to(entry).iter().any(|r| r.kind == RefKind::Call)
    };
    let step = |at: u64| -> Option<Step> {
        if let Some(insn) = listing.instruction_at(at) {
            let mut cases = Vec::new();
            if dispatch(insn.flow.is_computed, insn.flow.is_jump, insn.flow.is_call) {
                cases.extend(listing.refs_from(at).iter()
                    .filter(|r| r.kind == RefKind::Code && Some(r.to) != insn.fall_through && !insn.flows.contains(&r.to))
                    .map(|r| r.to));
                cases.extend(inline_table(at, insn.len));
            }
            return Some(Step {
                len: insn.len,
                kind: insn.flow.kind,
                is_call: insn.flow.is_call,
                fall_through: insn.fall_through,
                flows: insn.flows.clone(),
                continues: super::kuna_poolextents::falls_through_with(insn, &returns, true),
                filler: filler(insn.mnemonic.as_str(), &insn.operands, wide),
                cases,
            });
        }
        if !listing.is_undefined(at) {
            return None;
        }
        let decoded = super::decode::decode_one(translate, at, code_space, true, false).ok()?;
        if decoded.len == 0 {
            return None;
        }
        let c = super::classify::classify(&decoded.ops, at, decoded.len);
        let continues = c.fall_through.is_some()
            && (!(c.flow.is_call || c.flow.kind == FlowKind::CallOther)
                || (c.flow.is_call && (c.flow.is_conditional || c.flow.is_computed))
                || (c.flow.is_call && c.flows.first().is_some_and(|t| returns.contains(t))));
        let cases = if dispatch(c.flow.is_computed, c.flow.is_jump, c.flow.is_call) {
            inline_table(at, decoded.len)
        } else {
            Vec::new()
        };
        Some(Step {
            len: decoded.len,
            kind: c.flow.kind,
            is_call: c.flow.is_call,
            fall_through: c.fall_through,
            flows: c.flows,
            continues,
            filler: filler(&decoded.mnemonic, &decoded.operands, wide),
            cases,
        })
    };
    interior_entries(candidates, &pointed, attested, step)
}

/// Every Thumb code-pointer word of an ARM image, `target -> sites`.
fn code_pointers(file: &object::File, listing: &Listing) -> BTreeMap<u64, Vec<u64>> {
    use object::read::Object;
    if file.architecture() != object::Architecture::Arm {
        return BTreeMap::new();
    }
    crate::aif::kuna_ptrentry::code_pointer_sites(file, listing.exec_ranges())
}

/// The targets of Thumb code pointers that may root live code: a pointer
/// word in data or in a literal pool `pools` proves, or any word the listing
/// did not decode as an instruction when its target is already a candidate
/// entry. A word in undecoded code bytes (`mov.w r8,#1` spells `0x0801f04f`)
/// is not a pointer, so it alone never invents a root.
fn pointer_roots(
    listing: &Listing,
    sites: &BTreeMap<u64, Vec<u64>>,
    pools: &[(u64, u64)],
    candidates: &BTreeSet<u64>,
) -> BTreeSet<u64> {
    let exec = listing.exec_ranges();
    let in_data = |site: u64| {
        !exec.iter().any(|&(lo, hi)| (lo..hi).contains(&site)) || pools.iter().any(|&(lo, hi)| (lo..hi).contains(&site))
    };
    let decoded = |site: u64| matches!(listing.code_unit_at(site), super::CodeUnit::Instruction(_));
    sites
        .iter()
        .filter(|(target, sites)| {
            sites.iter().any(|&site| in_data(site) || (candidates.contains(target) && !decoded(site)))
        })
        .map(|(&target, _)| target)
        .collect()
}

fn filler(mnemonic: &str, operands: &str, wide: bool) -> bool {
    crate::aif::kuna_aifnoppad::is_padding_text(mnemonic, operands, wide)
        || crate::aif::kuna_aifnoppad::is_zero_fill_text(mnemonic, operands)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn insn(len: u32, kind: FlowKind, fall: bool, flows: &[u64]) -> Step {
        Step {
            len,
            kind,
            is_call: kind == FlowKind::Call,
            fall_through: fall.then_some(0),
            flows: flows.to_vec(),
            continues: fall,
            filler: false,
            cases: Vec::new(),
        }
    }

    /// A program as `(address, step)` rows; a fall-through is `address + len`.
    fn program(rows: &[(u64, Step)]) -> BTreeMap<u64, Step> {
        rows.iter()
            .map(|(at, s)| {
                let mut s = s.clone();
                s.fall_through = s.fall_through.map(|_| at + u64::from(s.len));
                (*at, s)
            })
            .collect()
    }

    fn run(prog: &BTreeMap<u64, Step>, entries: &[u64], vouched: &[u64]) -> Vec<u64> {
        let candidates: BTreeSet<u64> = entries.iter().copied().collect();
        interior_entries(&candidates, &BTreeSet::new(), |e| vouched.contains(&e), |at| prog.get(&at).cloned())
            .interior
            .into_keys()
            .collect()
    }

    fn plain() -> Step { insn(2, FlowKind::Fallthrough, true, &[]) }
    fn ret() -> Step { insn(2, FlowKind::Return, false, &[]) }
    fn cbranch(to: u64) -> Step { insn(2, FlowKind::ConditionalBranch, true, &[to]) }
    fn branch(to: u64) -> Step { insn(2, FlowKind::UnconditionalBranch, false, &[to]) }
    fn call(to: u64) -> Step { insn(4, FlowKind::Call, true, &[to]) }

    /// The `uartTotalRxBytesWaiting` shape: `ldr; cbz r3, L; <block>; bx lr;
    /// L: <block>; bx lr`, with both the cbz fall-through and its target
    /// discovered as entries.
    #[test]
    fn the_fall_through_and_the_moved_block_of_a_test_are_interior() {
        let prog = program(&[
            (0x100, plain()), (0x102, cbranch(0x10a)),
            (0x104, plain()), (0x106, plain()), (0x108, ret()),
            (0x10a, plain()), (0x10c, ret()),
            (0x10e, plain()), (0x110, ret()),
        ]);
        assert_eq!(run(&prog, &[0x100, 0x104, 0x10a, 0x10e], &[0x100]), vec![0x104, 0x10a]);
    }

    /// A shrink-wrapped frame push after an early test is reached by the test's
    /// fall-through.
    #[test]
    fn a_frame_pushed_after_an_early_test_is_interior() {
        let prog = program(&[
            (0x100, plain()), (0x102, cbranch(0x10a)),
            (0x104, plain()), (0x106, call(0x200)), (0x10a, ret()),
            (0x200, ret()),
        ]);
        assert_eq!(run(&prog, &[0x100, 0x104, 0x200], &[0x200]), vec![0x104]);
    }

    /// The image's word keeps an entry that code runs into.
    #[test]
    fn a_vouched_entry_is_kept_even_when_code_runs_into_it() {
        let prog = program(&[(0x100, plain()), (0x102, plain()), (0x104, ret())]);
        assert!(run(&prog, &[0x100, 0x102], &[0x102]).is_empty());
    }

    /// Padding, zero fill, a call, and code reached only past a call that is not
    /// proven to return all leave the next entry alone.
    #[test]
    fn padding_calls_and_unproven_fall_throughs_separate_functions() {
        let mut nop = plain();
        nop.filler = true;
        let mut unproven = call(0x300);
        unproven.continues = false;
        let prog = program(&[
            (0x100, plain()), (0x102, ret()), (0x104, nop), (0x106, plain()), (0x108, ret()),
            (0x200, plain()), (0x202, call(0x300)), (0x206, plain()), (0x208, ret()),
            (0x280, unproven), (0x284, plain()), (0x286, plain()), (0x288, ret()),
            (0x300, ret()),
        ]);
        assert!(run(&prog, &[0x100, 0x104, 0x106, 0x200, 0x206, 0x280, 0x286, 0x300], &[0x300])
            .iter()
            .all(|e| ![0x106, 0x206, 0x286].contains(e)));
    }

    /// A function whose first instruction heads a loop is reached by its own
    /// backward branch only; the entry before it never reaches that branch.
    #[test]
    fn a_loop_head_at_an_entry_is_not_a_local_branch_target() {
        let prog = program(&[
            (0x100, plain()), (0x102, ret()),
            (0x104, plain()), (0x106, cbranch(0x104)), (0x108, ret()),
        ]);
        assert!(run(&prog, &[0x100, 0x104], &[]).is_empty());
    }

    /// A tail call, including one to the very next address, keeps its target.
    #[test]
    fn a_tail_call_target_is_not_interior() {
        let prog = program(&[
            (0x100, plain()), (0x102, branch(0x104)),
            (0x104, plain()), (0x106, ret()),
        ]);
        assert!(run(&prog, &[0x100, 0x104], &[0x100]).is_empty());
    }

    /// A switch case that opens like a prologue is the dispatcher's code.
    #[test]
    fn a_switch_case_is_interior() {
        let mut dispatch = insn(4, FlowKind::ComputedJump, false, &[]);
        dispatch.cases = vec![0x108, 0x10c];
        let prog = program(&[
            (0x100, plain()), (0x102, cbranch(0x110)), (0x104, dispatch),
            (0x108, plain()), (0x10a, ret()),
            (0x10c, plain()), (0x10e, ret()),
            (0x110, ret()),
        ]);
        assert_eq!(run(&prog, &[0x100, 0x108, 0x10c], &[0x100]), vec![0x108, 0x10c]);
    }

    /// Two unattested entries: the first runs into the second and its early exit
    /// branches past it, which is the shrink-wrapped shape; without the branch
    /// past, the second stays (the first could be data running into it).
    #[test]
    fn an_unattested_owner_takes_a_frame_push_only_when_it_branches_past_it() {
        let shrink = program(&[
            (0x100, plain()), (0x102, cbranch(0x10c)),
            (0x104, plain()), (0x106, plain()), (0x108, plain()), (0x10a, ret()),
            (0x10c, ret()),
        ]);
        assert_eq!(run(&shrink, &[0x100, 0x104], &[]), vec![0x104]);
        let data = program(&[
            (0x100, plain()), (0x102, plain()),
            (0x104, plain()), (0x106, ret()),
        ]);
        assert!(run(&data, &[0x100, 0x104], &[]).is_empty());
    }

    /// A code pointer no candidate names, whose code runs into a planted entry,
    /// becomes the entry in its place.
    #[test]
    fn a_pointer_target_replaces_the_entry_it_runs_into() {
        let prog = program(&[
            (0x100, plain()), (0x102, cbranch(0x10c)),
            (0x104, plain()), (0x106, plain()), (0x108, plain()), (0x10a, ret()),
            (0x10c, ret()),
        ]);
        let candidates: BTreeSet<u64> = [0x104].into();
        let verdict = interior_entries(&candidates, &[0x100].into(), |_| false, |at| prog.get(&at).cloned());
        assert_eq!(verdict.interior.keys().copied().collect::<Vec<_>>(), vec![0x104]);
        assert_eq!(verdict.entries.into_iter().collect::<Vec<_>>(), vec![0x100]);
    }

    /// A call from any candidate's code attests its target.
    #[test]
    fn a_called_candidate_is_kept() {
        let prog = program(&[
            (0x100, plain()), (0x102, plain()), (0x104, ret()),
            (0x200, call(0x102)), (0x204, ret()),
        ]);
        assert!(run(&prog, &[0x100, 0x102, 0x200], &[0x100]).is_empty());
    }
}
