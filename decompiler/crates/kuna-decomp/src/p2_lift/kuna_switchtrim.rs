//! (kuna) `switchtrim` — recover a jump table no guard bounds, and end it where
//! its layout ends.
//!
//! A switch whose default is unreachable (`__builtin_unreachable()`, an enum
//! the compiler trusts, a `switch (x & 7)` the mask was folded out of) indexes
//! its table with no range check. `JumpBasic` then sees the full width of the
//! index, the range exceeds `jumptablemax`, and the `BRANCHIND` stays a computed
//! jump-as-call. The table's own layout still bounds it: a single-`LOAD` table
//! whose range is unbounded is opened to `[0, jumptablemax)`; emulation stops at the first row
//! it cannot read, and the sanity check keeps the rows before the first one
//! that
//!
//! * is zero or far away with no image bytes behind it (upstream's rule),
//! * points into the table's own bytes (its first entry up to this one's end),
//! * is read from at or above the lowest code address an earlier row jumps to
//!   (a table cannot run into the code it dispatches to), or
//! * on an object load, points outside the image's executable memory.
//!
//! Each row must read its own entry, above the previous row's, so a range
//! opened on a value that is not the table index reads no table at all. Fewer
//! than two rows is no table.

use kuna_base::address::Address;
use kuna_base::types::int4;

use crate::funcdata::Funcdata;
use crate::jumptable::LoadTable;

/// How many leading rows of `addresstable` survive the layout rules above.
/// `loadcounts[i]` is the number of loads read through row `i`, so row `i`'s
/// table entry is `loadpoints[loadcounts[i] - 1]`.
pub fn trimmed_rows(
    fd: &Funcdata,
    addresstable: &[Address],
    loadpoints: &[LoadTable],
    loadcounts: Option<&Vec<int4>>,
) -> usize {
    let entry = |i: usize| -> Option<&LoadTable> {
        let n = *loadcounts?.get(i)?;
        loadpoints.get(usize::try_from(n).ok()?.checked_sub(1)?)
    };
    let exec = &fd.get_arch().litpool_const;
    let Some(lo) = entry(0).map(|e| e.addr.get_offset()) else { return 0 };
    let mut prev: Option<u64> = None;
    let mut lowest_target_above: Option<u64> = None;
    for (i, t) in addresstable.iter().enumerate() {
        let off = t.get_offset();
        if !exec.is_empty() && !crate::kuna_litpoolconst::contains(exec, off, 1) {
            return i;
        }
        let Some(e) = entry(i) else { return i };
        let at = e.addr.get_offset();
        let end = at.saturating_add(e.size.max(0) as u64);
        if prev.is_some_and(|p| at <= p) || (lo..end).contains(&off) || lowest_target_above.is_some_and(|m| at >= m) {
            return i;
        }
        prev = Some(at);
        if off >= end {
            lowest_target_above = Some(lowest_target_above.map_or(off, |m| m.min(off)));
        }
    }
    addresstable.len()
}
