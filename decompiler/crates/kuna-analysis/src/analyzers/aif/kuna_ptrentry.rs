//! (kuna) `ptrentry` — pointer-referenced ARM function entries (P1 code/data
//! partition, ARM discovery Stage 3b).
//!
//! Step 2 of the ARM entry-recall sequence (`[PROPOSAL]` PR #239). That proposal
//! measured **2,026 of 2,061 live recall misses as function-entry granularity on
//! embedded ARM Cortex-M**, and found that its single largest sub-class — 1,671
//! addresses, 56.8% of the whole gap — is reachable only through a code-pointer
//! word that the shipped Stage-3 scan
//! ([`super::code_pointer_table_seeds`]) *already finds and then throws away*.
//!
//! # What the shipped Stage-3 scan rejects, and why it is wrong to
//!
//! Stage 3 accepts a code pointer's target only if it (a) lands in an undefined
//! gap, (b) opens with a stack-frame prologue
//! ([`super::is_thumb_function_prologue`]) and (c) disassembles into a
//! `check_valid_subroutine` of **more than two instructions**
//! (`MIN_SUBROUTINE_INSNS`). Predicates (b) and (c) reject 1,632 of the 1,671
//! missed addresses, because on this corpus
//!
//! * **93.3% of the missed entries have no canonical Thumb frame prologue** — a
//!   leaf callback establishes no frame at all, and
//! * **41% are ≤ 8-byte leaves** — `movs r0,#0 ; bx lr`, `mov r0,r2 ; bx lr`, or a
//!   bare `bx lr`. A one-instruction `bx lr` IS a valid Cortex-M exception handler
//!   (`nuttx`'s `devnull_read`, cleanflight's three distinct `NMI_Handler` /
//!   `PendSV_Handler` / `DebugMon_Handler` stubs), and every rival decompiler
//!   recovers them.
//!
//! Simply deleting the two predicates is measured and **not shippable**: 560 new
//! entries at 62% ground truth, 117 of them (21%) splitting a real function body.
//! decbench scores per ground-truth function, so those splits cost nothing on the
//! benchmark and everything in real use. The design work is therefore entirely the
//! precision model, and this module is that model.
//!
//! # The precision model: containment, not shape
//!
//! The 21% mid-body splits are dominated by `ldr pc,[pc,r]`-style **switch
//! tables**, whose slots point *into* the very function that holds the table.
//! Two containment facts separate them from a real vtable / fops struct / ISR
//! table, and between them they are decisive (measured over 3,462 validated
//! candidates on the 48-binary Cortex-M corpus):
//!
//! | guard | candidates | ground truth | splits a real body |
//! |---|---|---|---|
//! | validated candidates | 3,462 | 2,009 (58.0%) | 1,261 |
//! | + no site shares the target's function | 2,184 | 1,997 (91.4%) | 8 |
//! | + no site is instruction bytes | 2,143 | 1,992 (**93.0%**) | **0** |
//!
//! 1. **[`Guard::SameFunction`]** — reject a target when any pointer word that
//!    references it falls in the same discovered kuna function as the target
//!    itself. This is the switch table exactly: table and cases share a body. It
//!    removes 1,253 of the 1,261 splits and costs 12 ground-truth entries.
//! 2. **[`Guard::SiteIsInstruction`]** — reject a target when any referencing word
//!    overlaps a decoded instruction. Such a "pointer" is an instruction's operand
//!    bytes read four-aligned, not a table slot. It removes the remaining 8 splits
//!    and costs 5 ground-truth entries.
//!
//! This is the kuna form of the guard Ghidra draws between
//! `OperandReferenceAnalyzer` (creates functions, from **instruction operands**)
//! and `DataOperandReferenceAnalyzer`, which overrides `createFunctions` to a
//! no-op — *"don't ever create a function from a data pointer"*. kuna cannot use
//! Ghidra's version directly: the Listing tier records only control-flow
//! references (`RefKind::Call`/`Code`; the design leaves the data kinds
//! unpopulated), so "which instruction loaded this word" is not available. The
//! containment pair recovers the same discrimination from the code/data partition
//! the walk already leaves behind.
//!
//! **Table-run corroboration was measured and is dominated.** The proposal's other
//! candidate guard — require a run of ≥ 2 consecutive stride-4 code-pointer words
//! — was implemented and measured on the same candidates: it removes only 3 of the
//! 8 splits the two containment guards do not, and costs **196 ground-truth
//! entries** (1,796 vs 1,992). The switch tables it was meant to catch are runs
//! themselves, so a run test cannot separate them; the lone in-`.text` pointers it
//! rejects are ordinary `ldr rN,=func` literal-pool constants. It is therefore not
//! part of the shipped predicate — see `docs/features/ptrentry/analysis.md` §3.
//!
//! # Terminating-routine validity replaces the length floor
//!
//! [`check_terminating_routine`] is `check_valid_subroutine` **without
//! `MIN_SUBROUTINE_INSNS`**: follow the speculatively-decoded flow from the
//! target, reject an undecodable byte, a flow that leaves the executable image, or
//! an escape into another undecoded region, and require the walk to reach a clean
//! `RET` / computed jump (or a call into already-discovered code) inside the
//! `MAX_FOLLOW_INSNS` bound. A single `bx lr` passes; a random data word that
//! happens to decode does not.
//!
//! # Wiring: entry facts only, deliberately not walk seeds
//!
//! The accepted targets are emitted as an additive `entries` fact stream and are
//! **not** fed back into the recursive-descent walk, even though the shipped
//! Stages 2 and 3 do re-seed it. Both wirings were built and measured (analysis
//! §5): re-seeding gains 13 more ground-truth entries but drops 734 already
//! -recovered ones (15 of them ground truth) and splits 974 real function bodies,
//! taking precision from 95.2% to 59.5%. The mechanism is the one step 1
//! (`cortexmvectors`) traced —
//! `listing/walk.rs` treats an unconditional `B` as same-function flow, so seeding
//! a two-instruction ISR stub that tail-calls a shared handler makes the walk
//! absorb that handler, whose own entry then fails the `is_undefined` guard here
//! and in the AIF gap scan. That is proposal step 3's tail-call split, which has
//! not shipped. Until it does, the additive wiring is strictly better on every
//! axis, and it makes **"this option never removes an entry" a property of the
//! wiring rather than of a heuristic**.

use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

use kuna_base::space::AddrSpace;
use kuna_sleigh::translate::Translate;

use crate::listing::{CodeUnit, FlowKind, Listing};

use super::{GapDecoder, MAX_FOLLOW_INSNS};

/// Why a pointer-referenced candidate was rejected (the measured precision model;
/// see the module docs for the per-guard contribution).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Guard {
    /// The target is not in an undefined gap, or is inside a routine an earlier
    /// candidate already claimed.
    NotUndefined,
    /// A referencing word overlaps a decoded instruction — it is operand bytes
    /// read four-aligned, not a table slot.
    SiteIsInstruction,
    /// A referencing word lies in the same discovered function as the target — the
    /// `ldr pc,[pc,r]` switch-table shape, whose slots point into their own body.
    SameFunction,
    /// The speculative decode is not a terminating routine.
    NotTerminating,
}

/// Every 4-byte-aligned Thumb code-pointer word in the object's allocated
/// sections, as `target -> referencing site VMAs` (address-ordered).
///
/// A word qualifies when bit 0 is set (the Thumb bit) and the masked value points
/// into an executable range. Scanning *all* allocated sections — not just the
/// non-executable ones — is what reaches the literal pools and vector tables that
/// bare-metal firmware links inside `.text`.
fn code_pointer_sites(
    file: &object::File,
    exec_ranges: &[(u64, u64)],
) -> BTreeMap<u64, Vec<u64>> {
    use object::read::{Object, ObjectSection};
    let little = file.is_little_endian();
    let in_exec = |vma: u64| exec_ranges.iter().any(|&(lo, hi)| vma >= lo && vma < hi);

    let mut out: BTreeMap<u64, Vec<u64>> = BTreeMap::new();
    for sec in file.sections() {
        let sec_addr = sec.address();
        if sec_addr == 0 {
            continue;
        }
        let Ok(data) = sec.data() else { continue };
        let mut off = (4 - (sec_addr as usize & 3)) & 3;
        while off + 4 <= data.len() {
            let w = if little {
                u32::from_le_bytes([data[off], data[off + 1], data[off + 2], data[off + 3]])
            } else {
                u32::from_be_bytes([data[off], data[off + 1], data[off + 2], data[off + 3]])
            } as u64;
            if (w & 1) != 0 && in_exec(w & !1) {
                out.entry(w & !1).or_default().push(sec_addr + off as u64);
            }
            off += 4;
        }
    }
    out
}

/// The whole acceptance predicate for one candidate: the accepted routine's body
/// VMAs, or the [`Guard`] that rejected it.
fn probe_target(
    decoder: &mut GapDecoder,
    listing: &Listing,
    claimed: &BTreeSet<u64>,
    target: u64,
    sites: &[u64],
) -> Result<BTreeSet<u64>, Guard> {
    if !listing.is_undefined(target) || claimed.contains(&target) {
        return Err(Guard::NotUndefined);
    }
    let target_fn = listing.function_containing(target).map(|f| f.entry);
    for &site in sites {
        if matches!(listing.code_unit_at(site), CodeUnit::Instruction(_)) {
            return Err(Guard::SiteIsInstruction);
        }
        if let (Some(a), Some(b)) = (listing.function_containing(site).map(|f| f.entry), target_fn) {
            if a == b {
                return Err(Guard::SameFunction);
            }
        }
    }
    let gap_hi = listing.next_instruction_start_after(target).unwrap_or(u64::MAX);
    check_terminating_routine(decoder, listing, target, gap_hi).ok_or(Guard::NotTerminating)
}

/// [`super::check_valid_subroutine`] **without the `MIN_SUBROUTINE_INSNS` floor**:
/// follow the speculatively-decoded flow from `entry` and accept when it reaches a
/// clean `RET` / computed jump, or calls into already-discovered code, inside
/// `MAX_FOLLOW_INSNS`.
///
/// Rejects an undecodable byte, a flow leaving the executable image, and an escape
/// into an undecoded region outside `[entry, gap_hi)` — the strict escape policy
/// the pointer-root validator already uses, since an uncorroborated jump into
/// another dark region is the signature of a misread data word. Returns the body
/// VMAs so the caller can claim them.
pub(super) fn check_terminating_routine(
    decoder: &mut GapDecoder,
    listing: &Listing,
    entry: u64,
    gap_hi: u64,
) -> Option<BTreeSet<u64>> {
    let mut body: BTreeSet<u64> = BTreeSet::new();
    let mut worklist: Vec<u64> = vec![entry];
    let mut did_terminate = false;
    let mut adds_info = false;
    let mut steps = 0usize;

    while let Some(vma) = worklist.pop() {
        if body.contains(&vma) {
            continue;
        }
        steps += 1;
        if steps > MAX_FOLLOW_INSNS {
            return None;
        }
        if vma < entry || vma >= gap_hi {
            if listing.is_instruction_start(vma) {
                adds_info = true;
                continue;
            }
            return None;
        }
        let insn = decoder.probe(vma)?;
        body.insert(vma);
        if insn.is_terminal || matches!(insn.kind, FlowKind::ComputedJump) {
            did_terminate = true;
            continue;
        }
        for &target in &insn.flows {
            if !decoder.in_exec(target) {
                return None;
            }
            if insn.is_call {
                if listing.is_instruction_start(target) {
                    adds_info = true;
                }
            } else {
                worklist.push(target);
            }
        }
        if let Some(fall) = insn.fall_through {
            worklist.push(fall);
        }
    }

    if body.is_empty() || !(did_terminate || adds_info) {
        return None;
    }
    Some(body)
}

/// (kuna, ARM discovery Stage 3b) The `ptrentry` scan: the accepted
/// pointer-referenced function entries, address-sorted and deduped, ready to emit
/// as [`crate::pass::AnalysisOutput::entries`].
///
/// ARM-gated (Thumb code pointers are an ARM-only notion), so a strict no-op on
/// every other architecture. Purely additive: it reads the built Listing and emits
/// entries, and never re-seeds the walk — see the module docs.
pub fn pointer_entry_seeds(
    file: &object::File,
    listing: &Listing,
    translate: &dyn Translate,
    code_space: Rc<AddrSpace>,
    exec_ranges: &[(u64, u64)],
) -> Vec<u64> {
    use object::read::Object;
    if file.architecture() != object::Architecture::Arm {
        return Vec::new();
    }

    let sites = code_pointer_sites(file, exec_ranges);
    let mut decoder = GapDecoder::new(translate, code_space, exec_ranges);
    let mut accepted: BTreeSet<u64> = BTreeSet::new();
    let mut claimed: BTreeSet<u64> = BTreeSet::new();

    // Ascending target order, so the body-claim dedup sees a routine's entry before
    // any pointer into its interior (the same ordering guarantee Stages 2-3 use).
    for (&target, sites) in &sites {
        if let Ok(body) = probe_target(&mut decoder, listing, &claimed, target, sites) {
            accepted.insert(target);
            claimed.extend(body);
        }
    }

    accepted.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use object::read::Object;

    fn fixture(name: &str) -> Vec<u8> {
        let path = format!("{}/tests/fixtures/{}", env!("CARGO_MANIFEST_DIR"), name);
        std::fs::read(&path).unwrap_or_else(|_| panic!("read fixture {path}"))
    }

    /// The committed e2e fixture, read through the collector: every Thumb code
    /// pointer in it is found, in *both* the `.rodata` fops-struct slot and the
    /// in-`.text` switch slot, and each carries its referencing site. Pins the
    /// fixture so a regenerated file cannot silently drift.
    #[test]
    fn collects_every_code_pointer_with_its_site() {
        let bytes = fixture("cortexm_ptrentry_le32");
        let file = object::File::parse(bytes.as_slice()).expect("parse fixture");
        let sites = code_pointer_sites(&file, &[(0x0800_0000, 0x0800_0060)]);
        assert_eq!(
            sites.get(&0x0800_0048).map(Vec::as_slice),
            Some(&[0x0800_0060u64][..]),
            "the frameless leaf is reached only by the .rodata function pointer"
        );
        assert_eq!(
            sites.get(&0x0800_005c).map(Vec::as_slice),
            Some(&[0x0800_0058u64][..]),
            "the switch case is reached only by the in-.text slot before it"
        );
        assert_eq!(
            sites.get(&0x0800_0040).map(Vec::as_slice),
            Some(&[0x0800_0004u64, 0x0800_0008, 0x0800_000c][..]),
            "the reset vector's three table slots all resolve to one target"
        );
        // An even word, a zero slot, and a word pointing outside the executable
        // range are not code pointers.
        assert_eq!(sites.get(&0x2000_1000), None, "the initial SP is not a code pointer");
        assert_eq!(sites.len(), 3, "no other word in the image qualifies: {sites:?}");
    }

    /// The in-executable predicate, on the same bytes with a narrowed executable
    /// range: a pointer whose target falls outside the code is not a candidate.
    #[test]
    fn a_target_outside_the_executable_ranges_is_not_a_candidate() {
        let bytes = fixture("cortexm_ptrentry_le32");
        let file = object::File::parse(bytes.as_slice()).expect("parse fixture");
        let sites = code_pointer_sites(&file, &[(0x0800_0000, 0x0800_0050)]);
        assert!(sites.contains_key(&0x0800_0048));
        assert!(!sites.contains_key(&0x0800_005c), "0x800005c is now outside the code");
    }

    /// Strict no-op off ARM: Thumb code pointers are an ARM-only notion, so
    /// [`pointer_entry_seeds`] returns on the architecture gate before it scans
    /// anything. Retargeting the same bytes (`e_machine` is at offset 18 of an
    /// ELF32 header) leaves candidates the scan would still find, so the gate —
    /// not an empty scan — is what makes the option inert elsewhere.
    #[test]
    fn non_arm_is_gated_before_the_scan() {
        let mut bytes = fixture("cortexm_ptrentry_le32");
        bytes[18] = 62; // EM_X86_64
        bytes[19] = 0;
        let file = object::File::parse(bytes.as_slice()).expect("parse");
        assert_ne!(file.architecture(), object::Architecture::Arm, "the gate fires");
        assert!(
            !code_pointer_sites(&file, &[(0x0800_0000, 0x0800_0060)]).is_empty(),
            "the scan itself is architecture-blind, so the gate is load-bearing"
        );
    }
}
