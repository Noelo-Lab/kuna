//! (kuna `rawdiscover`) Function discovery for a headerless raw image.
//!
//! `--raw-image` loads a file that carries no object header, so the whole
//! discovery tier declines: [`crate::passes::run_listing_consumers`] opens with
//! `parse_object(bytes)` and every seed oracle under it reads an `object::File`
//! — a symbol table, an exception table, a section list. A raw image has none
//! of them, and the inventory is therefore exactly the `--entry` addresses the
//! caller typed. On a 147 KiB firmware image with one entry that is a single
//! "function" spanning the whole file.
//!
//! Neither of the two things that actually find functions needs that metadata:
//!
//! 1. the recursive descent ([`crate::listing::walk`]) follows direct call
//!    targets from the seeds and promotes each to an entry;
//! 2. a **linear call-target sweep** over the executable bytes, which decodes
//!    straight through the image and collects the target of every direct call
//!    it sees, then hands those to the descent as additional roots.
//!
//! The descent alone recovers only the seeds' direct-call closure. On firmware
//! that dispatches through pointer tables and interrupt vectors that is a small
//! fraction of the image — on the 147 KiB ATWINC1500 image this was written
//! against, one entry reaches 21 of about 555 functions. The sweep is what
//! reaches the rest, and a call target is a strong claim: it is an address some
//! instruction names as a subroutine, not a byte pattern that resembles one.
//!
//! The sweep decodes bytes that may be data, and data decodes as *something*,
//! so a junk "call" can mint a junk entry. Three things bound that: a target
//! must land inside the executable ranges, the sweep resynchronizes on the
//! instruction alignment after a failed decode rather than cascading, and every
//! surviving root is then re-walked by the descent, which decodes it for real.
//! An MS-DOS executable, whose load module is mostly data, also hands the
//! sweep its relocations ([`super::kuna_mzsweep`]). `--define-function`
//! remains the way to assert a boundary neither oracle found.

use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::AddrSpace;
use kuna_decomp::architecture::Architecture;
use kuna_decomp::kuna_fasthash::FxHashSet;
use kuna_sleigh::translate::Translate;

use crate::listing::classify::classify;
use crate::listing::decode::decode_one;
use crate::listing::kuna_dosexit::{self, DosExits};
use crate::listing::kuna_mzsweep::{self, MzEvidence, SweepRun};
use crate::listing::kuna_rawextent;
use crate::listing::{Listing, ListingDetail};

/// What [`discover`] finds in a raw image.
pub struct RawDiscovery {
    /// Sorted entries to install now, excluding assertion-only roots.
    pub entries: Vec<u64>,
    /// The DOS exits of the code the descent decoded ([`kuna_dosexit`]).
    pub dos_exits: DosExits,
    /// Proven flow bounds from [`kuna_rawextent`]; absent entries keep the clip.
    pub flow_ends: Option<BTreeMap<u64, u64>>,
    /// Direct Thumb calls as (callee, continuation, containing caller entry), sorted.
    pub call_returns: Vec<(u64, u64, u64)>,
}

/// Every function entry a raw image yields: the seeds, the targets of every
/// direct call in the executable bytes, and the direct-call closure of both,
/// with the DOS exits and any flow ends the raw Listing can measure.
///
/// A seed is never dropped, so the inventory can only grow; an image whose
/// seeds reach no call and whose gaps hold no code returns exactly what the
/// caller passed.
///
/// `exec_ranges` bounds every decode. An address outside it is not decoded and
/// contributes no successor, so a call into unmapped space is dropped rather
/// than walked into the void. `mz`, an MS-DOS executable's evidence, holds the
/// sweep to the calls [`super::kuna_mzsweep`] keeps and the targets that open
/// with code.
///
/// `assertion_roots` also seed descent and extent measurement but do not supply
/// synthetic symbols unless ordinary discovery would find them too; they lead
/// the roots so that the Listing's last-in-first-out walk decodes the ordinary
/// roots first. The sweep decodes data as well as code, so its context writes
/// are rolled back before the descent and an explicit ARM `--isa` mode is
/// protected inside it; the descent's own writes (a real `blx`) persist. Under
/// an explicit ARM `--isa` the seeds follow the swept roots, so that their
/// closure, with the mode switch of each real `blx` in it, decodes before a
/// swept root does, and each seed's mode is fenced ([`fence_seed_modes`]).
/// Under `--isa arm` a swept call target off a word boundary (a `blx` decoded
/// in data) is not a root; a Thumb entry the descent reaches through a real
/// `blx` stays an entry.
pub fn discover(
    arch: &Architecture,
    translate: &dyn Translate,
    code_space: &Rc<AddrSpace>,
    exec_ranges: &[(u64, u64)],
    seeds: &[u64],
    assertion_roots: BTreeSet<u64>,
    mz: Option<&MzEvidence>,
) -> RawDiscovery {
    if seeds.is_empty() || exec_ranges.is_empty() {
        return RawDiscovery {
            entries: seeds.to_vec(),
            dos_exits: DosExits::default(),
            flow_ends: None,
            call_returns: Vec::new(),
        };
    }
    let real_mode = kuna_dosexit::is_real_mode(&arch.archid);
    let explicit_arm_isa = !real_mode && arch.input_arm_isa_override;
    let thumb = explicit_arm_isa
        && arch.with_context_db(|db| {
            db.get_variable_value(b"TMode", &Address::new(Rc::clone(code_space), seeds[0])).ok()
        }) == Some(1);
    let word_aligned = |target: &u64| !explicit_arm_isa || thumb || target % 4 == 0;
    let mut swept: Vec<u64> = crate::loader::kuna_realmodecs::without_commits(arch, translate, || {
        let _scope = if real_mode {
            None
        } else {
            translate.decode_context_scope().inspect(|scope| {
                if explicit_arm_isa {
                    let _ = scope.protect_variable(b"TMode");
                }
            })
        };
        let swept = sweep_call_targets(translate, code_space, exec_ranges, mz);
        match mz {
            Some(mz) => swept
                .into_iter()
                .filter(|&target| {
                    seeds.contains(&target)
                        || kuna_mzsweep::opens_as_code(translate, code_space, exec_ranges, mz, target)
                })
                .collect(),
            None => swept.into_iter().filter(word_aligned).collect::<Vec<u64>>(),
        }
    });
    let mut seed_roots = seeds.to_vec();
    seed_roots.sort_unstable();
    seed_roots.dedup();
    swept.sort_unstable();
    swept.dedup();
    swept.retain(|target| seed_roots.binary_search(target).is_err());
    let mut deferred = assertion_roots;
    deferred.retain(|entry| seed_roots.binary_search(entry).is_err() && swept.binary_search(entry).is_err());
    let ordinary_start = deferred.len();
    if !explicit_arm_isa {
        seed_roots.extend(swept.drain(..));
        seed_roots.sort_unstable();
    }
    let roots: Vec<u64> = deferred.iter().copied().chain(swept).chain(seed_roots).collect();
    if explicit_arm_isa {
        fence_seed_modes(arch, code_space, exec_ranges, seeds);
    }

    // One descent over all roots. PARTITION_ONLY retains instruction lengths
    // and flow edges without disassembly text or cross-reference indexes.
    let listing = Listing::build_raw(
        arch,
        translate,
        code_space,
        exec_ranges,
        &roots,
        ListingDetail::PARTITION_ONLY,
    );

    let mut out: Vec<u64> = seeds.to_vec();
    out.extend(listing.functions().map(|(&vma, _)| vma));
    out.sort_unstable();
    out.dedup();
    let dos_exits = kuna_dosexit::analyze(arch, translate, code_space, &listing);
    let flow_ends = kuna_rawextent::measure(arch, translate, code_space, &listing, &out, &dos_exits);
    if !deferred.is_empty() {
        remove_discovered_roots(&listing, &roots[ordinary_start..], &mut deferred);
        out.retain(|entry| !deferred.contains(entry));
    }
    let call_returns = if thumb {
        super::kuna_callerreads::raw_call_returns(&listing)
    } else {
        Vec::new()
    };
    RawDiscovery { entries: out, dos_exits, flow_ends, call_returns }
}

/// Paint each seed's `TMode` over `[seed, next seed)`, its own value, so that
/// a real `blx` the descent follows switches the mode of its target up to the
/// next seed and no further: a Thumb function after an ARM one keeps its mode.
fn fence_seed_modes(
    arch: &Architecture,
    code_space: &Rc<AddrSpace>,
    exec_ranges: &[(u64, u64)],
    seeds: &[u64],
) {
    let mut seeds = seeds.to_vec();
    seeds.sort_unstable();
    seeds.dedup();
    for (i, &seed) in seeds.iter().enumerate() {
        let Some(&(_, hi)) = exec_ranges.iter().find(|&&(lo, hi)| (lo..hi).contains(&seed)) else { continue };
        let end = seeds.get(i + 1).map_or(hi, |&next| next.min(hi));
        let at = Address::new(Rc::clone(code_space), seed);
        arch.with_context_db_mut(|db| {
            if let Ok(mode) = db.get_variable_value(b"TMode", &at) {
                let _ = db.set_variable_region(b"TMode", &at, &Address::new(Rc::clone(code_space), end), mode);
            }
        });
    }
}

/// Drop from `deferred` every assertion root an ordinary root's flow calls, so
/// that a callee the sweep missed (at another alignment, say) is still
/// installed as discovered.
fn remove_discovered_roots(listing: &Listing, roots: &[u64], deferred: &mut BTreeSet<u64>) {
    let mut pending = roots.to_vec();
    let mut seen = FxHashSet::default();
    while let Some(vma) = pending.pop() {
        if !seen.insert(vma) {
            continue;
        }
        let Some(insn) = listing.instruction_at(vma) else { continue };
        if insn.flow.is_call {
            for target in &insn.flows {
                deferred.remove(target);
            }
            if deferred.is_empty() {
                break;
            }
        }
        pending.extend(insn.flows.iter().copied());
        pending.extend(insn.fall_through);
    }
}

/// Decode `exec_ranges` end to end and return the target of every direct call.
///
/// This is a linear sweep, not a flow walk: it starts at each range's low
/// address and steps by the decoded instruction length, so it reads code and
/// data alike. A byte that does not decode, or decodes to nothing, advances the
/// cursor to the next instruction-alignment boundary instead of stopping, which
/// is what lets the sweep resynchronize after a run of data rather than
/// abandoning the rest of the range - and, on a language that aligns its
/// instructions, keeps a misaligned address from decoding to a phantom call.
///
/// Only *statically* derivable targets count: an indirect call through a
/// register contributes nothing, because there is no address to contribute.
/// Targets outside `exec_ranges` are dropped.
///
/// [`discover`] isolates the sweep's context writes so that decodes from data
/// cannot change the context the descent starts in. The real-mode path instead
/// keeps its existing `realCS` write mask ([`crate::loader::kuna_realmodecs`]).
fn sweep_call_targets(
    translate: &dyn Translate,
    code_space: &Rc<AddrSpace>,
    exec_ranges: &[(u64, u64)],
    mz: Option<&MzEvidence>,
) -> Vec<u64> {
    // An instruction can never start off the language's alignment, so that is
    // both the smallest honest resynchronization step and the one that keeps a
    // misaligned probe from inventing a call. `1` on a language with no
    // requirement, which is what `get_alignment` documents.
    let step = translate.translate_base().get_alignment().max(1) as u64;
    let mut out: Vec<u64> = Vec::new();
    for &(lo, hi) in exec_ranges {
        let mut vma = lo;
        let mut run = SweepRun::default();
        while vma < hi {
            let Ok(decoded) = decode_one(translate, vma, code_space, false, false) else {
                run.undecodable();
                vma = next_aligned(vma, step);
                continue;
            };
            if decoded.len == 0 {
                run.undecodable();
                vma = next_aligned(vma, step);
                continue;
            }
            let c = classify(&decoded.ops, vma, decoded.len);
            let counts = match mz {
                Some(mz) => run.step(mz, vma, decoded.len, &c.flow),
                None => c.flow.is_call,
            };
            if counts {
                out.extend(
                    c.flows.iter().copied().filter(|t| in_ranges(exec_ranges, *t)),
                );
            }
            vma = vma.saturating_add(u64::from(decoded.len));
        }
    }
    out.sort_unstable();
    out.dedup();
    out
}

/// The next address strictly after `vma` that is a multiple of `step`.
///
/// A decode that fails part-way into an instruction leaves the cursor off the
/// alignment, so advancing by `step` alone would keep it off for the rest of
/// the range.
fn next_aligned(vma: u64, step: u64) -> u64 {
    let next = vma.saturating_add(step);
    next - (next % step)
}

/// Whether `vma` lies in any half-open executable range.
fn in_ranges(ranges: &[(u64, u64)], vma: u64) -> bool {
    ranges.iter().any(|&(lo, hi)| vma >= lo && vma < hi)
}
