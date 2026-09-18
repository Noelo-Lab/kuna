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
//! `--option rawdiscover off` restores the seeds-only inventory, and
//! `--define-function` remains the way to assert a boundary neither oracle
//! found.

use std::rc::Rc;

use kuna_base::space::AddrSpace;
use kuna_decomp::architecture::Architecture;
use kuna_sleigh::translate::Translate;

use crate::listing::classify::classify;
use crate::listing::decode::decode_one;
use crate::listing::{Listing, ListingDetail};

/// Every function entry a raw image yields: the seeds, the targets of every
/// direct call in the executable bytes, and the direct-call closure of both.
///
/// A seed is never dropped, so the inventory can only grow; an image whose
/// seeds reach no call and whose gaps hold no code returns exactly what the
/// caller passed.
///
/// `exec_ranges` bounds every decode. An address outside it is not decoded and
/// contributes no successor, so a call into unmapped space is dropped rather
/// than walked into the void.
pub fn discover(
    arch: &Architecture,
    translate: &dyn Translate,
    code_space: &Rc<AddrSpace>,
    exec_ranges: &[(u64, u64)],
    seeds: &[u64],
) -> Vec<u64> {
    if seeds.is_empty() || exec_ranges.is_empty() {
        return seeds.to_vec();
    }
    let mut roots: Vec<u64> = seeds.to_vec();
    roots.extend(sweep_call_targets(translate, code_space, exec_ranges));
    roots.sort_unstable();
    roots.dedup();

    // One descent over the union of the seeds and the swept roots. Only the
    // function partition is read back, so the disassembly text and the
    // reference model are not built.
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
    out
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
fn sweep_call_targets(
    translate: &dyn Translate,
    code_space: &Rc<AddrSpace>,
    exec_ranges: &[(u64, u64)],
) -> Vec<u64> {
    // An instruction can never start off the language's alignment, so that is
    // both the smallest honest resynchronization step and the one that keeps a
    // misaligned probe from inventing a call. `1` on a language with no
    // requirement, which is what `get_alignment` documents.
    let step = translate.translate_base().get_alignment().max(1) as u64;
    let mut out: Vec<u64> = Vec::new();
    for &(lo, hi) in exec_ranges {
        let mut vma = lo;
        while vma < hi {
            let Ok(decoded) = decode_one(translate, vma, code_space, false, false) else {
                vma = next_aligned(vma, step);
                continue;
            };
            if decoded.len == 0 {
                vma = next_aligned(vma, step);
                continue;
            }
            let c = classify(&decoded.ops, vma, decoded.len);
            if c.flow.is_call {
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
