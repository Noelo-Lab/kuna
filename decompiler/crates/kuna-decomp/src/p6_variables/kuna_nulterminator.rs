//! (kuna) `nulterminator` — a zero store past an open stack array's index
//! floor is that array's terminator, not a separate local.
//!
//! # The gap
//!
//! `ScopeLocal::restructure` (`varmap.cc:1294`) sweeps the sorted `RangeHint`s
//! and lets an *open* hint (a pointer into the frame) swallow a following hint
//! only through `RangeHint::attemptJoin` (`varmap.cc:170`): the follower must
//! lie within the open hint's `highind`, or be a constant COPY no further than
//! that bound (`isConstAbsorbable`, `varmap.cc:30`).  For a pointer with an
//! index Varnode and no refined guard `highind` is the `gatherOpen` floor of 3,
//! so every other hint ends the array at its own start.
//!
//! The bounded-copy idiom is the common victim:
//!
//! ```c
//! char buf[64];
//! strncpy(buf, in, sizeof(buf) - 1);
//! buf[sizeof(buf) - 1] = 0;
//! for (int i = 0; buf[i]; i++) ...
//! ```
//!
//! The terminator's index is a constant, so heritage folds its STORE into a
//! direct one-byte frame store at `buf + 63`, and the scan loop's index is
//! unbounded, so `loadguardrange` has no range to lock.  The layout becomes
//! `char buf[63]` plus a separate `char` that is written once and never read,
//! and the emitted C clears a variable nothing uses while the string it
//! terminates has no terminator.
//!
//! # The mechanism
//!
//! `Funcdata::gather_varnodes` records, for each `(offset, size)` a
//! constant COPY writes, whether *every* such write is a zero that no op reads
//! directly (`MapState::note_terminator_store`).  In the restructure sweep,
//! after `attempt_join` has declined, [`absorbs_terminator`] lets the open
//! range continue over the store when it is:
//!
//! - a *fixed* constant-COPY hint with no type lock, exactly one element of an
//!   open range whose elements are one- or two-byte integers (`char`, a UTF-16
//!   code unit), at a whole-element distance;
//! - a zero on every write, and unread except through markers.
//!
//! The only reader such a byte can have is a pointer into the open range, so
//! it is the range's element.  Wider elements are left alone: a zeroed
//! eight-byte slot after a two-word buffer is the length field of an MSVC
//! `std::string`, not a terminator.
//!
//! The index floor is not raised, so an adjacent read local still starts a new
//! variable, and any other hint at the store's own offset (a read of a later
//! version, a wider access, an address taken there) is not a terminator hint,
//! so it ends the range at that offset exactly as upstream would.  The absorbed
//! range fills to the next hint like any open range.
//!
//! Upstream's layout is restored with `option nulterminator off`.

use kuna_base::types::int4;

use crate::dtype::type_metatype;
use crate::varmap::{MapState, RangeHint, RangeType, COPY_CONSTANT};

/// Is `meta` an integer element type a terminator can belong to?
fn is_integer(meta: type_metatype) -> bool {
    meta == type_metatype::TYPE_INT || meta == type_metatype::TYPE_UINT
}

/// Can an element of `align` bytes be a string terminator (a `char` or a
/// UTF-16 code unit)?
fn is_character_width(align: int4) -> bool {
    align == 1 || align == 2
}

/// Should the open range `cur` continue over the non-intersecting hint `next`
/// as its zero terminator element?
pub fn absorbs_terminator(cur: &RangeHint, next: &RangeHint, state: &MapState) -> bool {
    if cur.range_type != RangeType::Open || cur.is_type_lock() {
        return false;
    }
    if next.range_type != RangeType::Fixed || next.flags != COPY_CONSTANT {
        return false;
    }
    if !is_integer(cur.type_.get_metatype()) {
        return false;
    }
    let next_meta = next.type_.get_metatype();
    if next_meta != type_metatype::TYPE_UNKNOWN && !is_integer(next_meta) {
        return false;
    }
    let align = cur.type_.get_align_size();
    if !is_character_width(align) || next.size != align {
        return false;
    }
    let dist = next.sstart.wrapping_sub(cur.sstart);
    if dist <= 0 || dist % (align as i64) != 0 {
        return false;
    }
    state.is_terminator_store(next.start, next.size)
}

#[cfg(test)]
#[path = "kuna_nulterminator/tests.rs"]
mod tests;
