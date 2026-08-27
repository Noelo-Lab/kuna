//! (kuna `entrythumbflow`) Entry-reachable Thumb decode context for a mixed
//! ARM image whose container entry carries the Thumb bit but whose machine word
//! makes no whole-image mode claim (PE `IMAGE_FILE_MACHINE_ARM` and
//! `ARMTHUMB_MIXED`, as a UEFI TE carries them).
//!
//! The entry bit proves the mode at one address. Ghidra's disassembler carries
//! that context along the flow it follows; kuna's decompiler reads context per
//! address from the `ContextDatabase` and follows nothing, so the same walk is
//! run here once, at the analysis commit, and its result is published as
//! bounded [`ContextPaint`] ranges: exactly the instruction bytes the walk
//! decoded as Thumb, never a whole section. Flow is followed through
//! fall-through, direct branches, and direct mode-preserving `BL` calls; an
//! interworking `BLX` target keeps the mode its own encoding selects.
//!
//! The walk is bounded by [`MAX_INSTRUCTIONS`]. Reaching the bound is not an
//! error: the ranges walked so far are still published, the caller is told
//! ([`EntryThumbFlow::truncated`]) so it can say so once, and the code beyond
//! the walked ranges decodes with the language default (A32), which is what an
//! even entry would have produced anyway; `--isa thumb` covers a wholly Thumb
//! image.
//!
//! The walk covers only the bytes the file backs. A mapped tail the loader
//! zero-fills decodes as a run of two-byte Thumb no-ops, which would march the
//! walk to the section end and paint it, so the caller hands it file-backed
//! extents; and a direct call to a callee the load-time facts know never
//! returns has no fall-through, so the bytes after it keep their own mode.
//!
//! Decoding needs the context set before the bytes are read, so the walk seeds
//! `TMode=1` over every executable range as one region write per range, decodes,
//! and then restores every run of values it found there. That keeps the context
//! partition at a handful of split points instead of two per visited
//! instruction, and restores exactly what was recorded, so arming the pass over
//! a range another producer has already painted does not flatten it. The
//! restore runs on every path out of the walk, including the one the
//! instruction budget cuts short, because a range left seeded would decode as
//! Thumb for the rest of the session. What the
//! seed cannot undo is the "definitively set" mark `set_variable_region` leaves
//! on the words it touches; the database exposes no way to clear it, so a later
//! point-form `set_variable` over the same range stops at the seed's split
//! points rather than the next real change point.

use std::collections::BTreeSet;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::KunaResult;
use kuna_base::space::AddrSpace;
use kuna_decomp::architecture::Architecture;

use crate::listing::classify::classify;
use crate::listing::decode::{decode_one, mnemonic_at};
use crate::pass::ContextPaint;

/// The instruction budget of one walk.
pub const MAX_INSTRUCTIONS: usize = 4096;

const TMODE: &[u8] = b"TMode";

/// What the walk found.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct EntryThumbFlow {
    /// The merged, bounded `TMode=1` ranges the walk decoded.
    pub paints: Vec<ContextPaint>,
    /// The walk stopped at [`MAX_INSTRUCTIONS`] with work remaining.
    pub truncated: bool,
}

/// Walk the flow reachable from the (already even) `entry` decoding as Thumb,
/// inside the executable `ranges` (`(start, size)`), and report the decoded
/// instruction ranges as `TMode=1` paints. A language without a `TMode`
/// context variable yields an empty result.
pub fn entry_thumb_flow(
    arch: &Architecture,
    code_space: &Rc<AddrSpace>,
    ranges: &[(u64, u64)],
    entry: u64,
    noreturn: &[u64],
) -> KunaResult<EntryThumbFlow> {
    let ranges: Vec<(u64, u64)> = ranges
        .iter()
        .filter_map(|&(start, size)| {
            let end = start.checked_add(size)?;
            (size != 0).then_some((start, end))
        })
        .collect();
    if range_holding(&ranges, entry).is_none() {
        return Ok(EntryThumbFlow::default());
    }
    let addr_at = |offset: u64| Address::new(Rc::clone(code_space), offset);

    // Every fallible READ happens before the first write, so a language that
    // registers no `TMode` leaves the database exactly as it found it.
    let mut saved: Vec<(u64, u64, u32)> = Vec::new();
    for &(start, end) in &ranges {
        let Some(runs) = context_runs(arch, code_space, start, end) else {
            return Ok(EntryThumbFlow::default());
        };
        saved.extend(runs);
    }

    // From the first seed onward every exit path restores: a walk abandoned
    // partway must not leave a range decoding as Thumb for the rest of the
    // session, and it must not report success while it does. Decoding is also
    // kept from writing context of its own: a Thumb `blx` runs the language's
    // `globalset(TMode=0)` at its target, which would flatten every Thumb
    // address above that target for the rest of the walk. The seed already
    // answers the mode question for the whole range.
    let translate = arch.translate();
    let walked = ranges
        .iter()
        .try_for_each(|&(start, end)| {
            arch.with_context_db_mut(|db| {
                db.set_variable_region(TMODE, &addr_at(start), &addr_at(end), 1)
            })
        })
        .map(|()| {
            translate.allow_context_set(false);
            let walked = walk_from_entry(arch, code_space, &ranges, entry, noreturn);
            translate.allow_context_set(true);
            walked
        });
    for &(start, end, value) in &saved {
        arch.with_context_db_mut(|db| {
            db.set_variable_region(TMODE, &addr_at(start), &addr_at(end), value)
        })?;
    }
    let (mut painted, truncated) = walked?;

    painted.sort_unstable();
    let mut merged: Vec<(u64, u64)> = Vec::new();
    for (start, end) in painted {
        match merged.last_mut() {
            Some(last) if start <= last.1 => last.1 = last.1.max(end),
            _ => merged.push((start, end)),
        }
    }
    Ok(EntryThumbFlow {
        paints: merged
            .into_iter()
            .map(|(addr, end)| ContextPaint { addr, end: Some(end), var: "TMode", value: 1 })
            .collect(),
        truncated,
    })
}

/// The end of the executable range holding `addr`, if one does.
fn range_holding(ranges: &[(u64, u64)], addr: u64) -> Option<u64> {
    ranges
        .iter()
        .find_map(|&(start, end)| (addr >= start && addr < end).then_some(end))
}

/// Decode from `entry` with the seed in place, following fall-through, direct
/// branches, and direct mode-preserving calls, and report the instruction
/// ranges decoded plus whether the budget stopped the walk.
///
/// Infallible by construction: an address that will not decode, or whose
/// instruction leaves its range, ends that path rather than the walk, so the
/// caller's restore is never skipped.
fn walk_from_entry(
    arch: &Architecture,
    code_space: &Rc<AddrSpace>,
    ranges: &[(u64, u64)],
    entry: u64,
    noreturn: &[u64],
) -> (Vec<(u64, u64)>, bool) {
    let translate = arch.translate();
    let mut pending = vec![entry];
    let mut visited = BTreeSet::new();
    let mut painted: Vec<(u64, u64)> = Vec::new();
    while let Some(addr) = pending.pop() {
        if visited.contains(&addr) {
            continue;
        }
        let Some(end) = range_holding(ranges, addr) else {
            continue;
        };
        if visited.len() >= MAX_INSTRUCTIONS {
            return (painted, true);
        }
        visited.insert(addr);
        let Ok(decoded) = decode_one(translate, addr, code_space, false) else {
            continue;
        };
        let Some(instruction_end) = addr.checked_add(u64::from(decoded.len)) else {
            continue;
        };
        if decoded.len == 0 || instruction_end > end {
            continue;
        }
        painted.push((addr, instruction_end));
        let classified = classify(&decoded.ops, addr, decoded.len);
        let follow_targets = if classified.flow.is_call {
            !classified.flow.is_indirect && is_mode_preserving_call(translate, addr, code_space)
        } else {
            true
        };
        // A direct call to a callee the load-time facts know never returns
        // has no fall-through: the bytes after it are padding, a pool, or the
        // next function, in whatever mode that is.
        let returns = !classified.flow.is_call
            || !classified.flows.iter().any(|target| noreturn.contains(&(target & !1)));
        if follow_targets {
            pending.extend(classified.flows.into_iter().map(|target| target & !1));
        }
        if let Some(fall_through) = classified.fall_through.filter(|_| returns) {
            pending.push(fall_through);
        }
    }
    (painted, false)
}

/// The `TMode` values already recorded across `[start, end)`, as
/// `(start, end, value)` runs, so the seed can be undone exactly rather than
/// flattened to whatever held at the range start. `None` when the active
/// language registers no `TMode` at all.
fn context_runs(
    arch: &Architecture,
    code_space: &Rc<AddrSpace>,
    start: u64,
    end: u64,
) -> Option<Vec<(u64, u64, u32)>> {
    let mut runs = Vec::new();
    let mut addr = start;
    while addr < end {
        let probe = Address::new(Rc::clone(code_space), addr);
        let (value, bound) = arch.with_context_db_mut(|db| {
            let value = db.get_variable_value(TMODE, &probe).ok()?;
            let (_, _, last) = db.get_context_bounds(&probe);
            Some((value, last))
        })?;
        // `bound` is the inclusive last offset the region covers; a region that
        // runs to the top of the space, or one that somehow ends at `addr`,
        // must still advance the cursor.
        let run_end = bound.saturating_add(1).max(addr + 1).min(end);
        runs.push((addr, run_end, value));
        addr = run_end;
    }
    Some(runs)
}

/// A direct Thumb `BL` keeps the callee in Thumb state; `BLX` interworks to
/// A32. The shipped ARM language spells both as their own mnemonics, so the
/// mnemonic is the encoding's own statement of which one it is.
fn is_mode_preserving_call(
    translate: &dyn kuna_sleigh::translate::Translate,
    addr: u64,
    code_space: &Rc<AddrSpace>,
) -> bool {
    let mnemonic = mnemonic_at(translate, addr, code_space).to_ascii_lowercase();
    mnemonic.starts_with("bl") && !mnemonic.starts_with("blx")
}
