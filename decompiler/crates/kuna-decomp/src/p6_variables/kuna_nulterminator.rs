//! (kuna) `nulterminator` — the zero element that ends a stack character
//! array belongs to the array, and nothing after it does.
//!
//! # The gap
//!
//! `ScopeLocal::restructure` (`varmap.cc:1294`) sweeps the sorted `RangeHint`s.
//! An *open* hint (a pointer into the frame) swallows a following hint only
//! through `RangeHint::attemptJoin` (`varmap.cc:170`): within its `highind`, or
//! a constant COPY no further than that bound (`isConstAbsorbable`,
//! `varmap.cc:30`).  Otherwise the open range ends at the following hint's
//! start.  For a pointer with an index Varnode and no refined guard, `highind`
//! is the `gatherOpen` floor of 3.
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
//! `char buf[63]` plus a separate `char` that is written once and never read.
//!
//! # The rule
//!
//! After `attempt_join` has declined `next`, [`close_at_terminator`] turns the
//! open range `cur` into a closed range ending exactly one element past `next`
//! when all of these hold:
//!
//! 1. `cur` is open and unlocked, and its elements are one- or two-byte
//!    integers (`char`, a UTF-16 code unit).
//! 2. `next` is a fixed, unlocked constant-COPY hint of exactly one element,
//!    at a whole-element offset past `cur`'s start.  `next` is the hint the
//!    open range would otherwise end at, so its slot is the element immediately
//!    after the array's current end.
//! 3. Every constant COPY into that exact slot writes zero and none is read
//!    directly (`MapState::is_terminator_store`).  A read of any other version
//!    of the slot, and any non-constant write, is a hint of its own at the same
//!    offset that sorts first and ends the array before the slot.
//! 4. The hint after `next` starts at or past the slot's end: nothing else (an
//!    address taken at the slot, a wider access over it) touches the element.
//! 5. No plain address into the frame (an open hint with no index) starts
//!    strictly inside the array.  Such an address marks a second object that
//!    upstream merged only because the array's index bound reached it, and the
//!    zero may terminate that object instead:
//!    `strcoll((char[]){i, 0}, (char[]){i + 1, 0})` is two `char[2]` that
//!    upstream already prints as one `char[3]`.  An indexed hint inside the
//!    array is ordinary element arithmetic (`buf[i + 1]` is recorded at
//!    `buf + 1`) and does not count.
//!
//! Such a byte has no reader except through a pointer into the array, so it is
//! the array's element.  The result is a *fixed* range, so nothing after the
//! terminator joins it: alignment padding stays unmapped, and a second zero
//! store, a neighbouring local or any later hint starts its own variable
//! exactly as upstream.  Compared with upstream the one change is the owner of
//! the terminator element.  Wider elements are left alone: a zeroed eight-byte
//! slot after a two-word buffer is the length field of an MSVC `std::string`,
//! not a terminator.
//!
//! Upstream's layout is restored with `option nulterminator off`.

use kuna_base::error::KunaResult;
use kuna_base::marshal::ElementId;
use kuna_base::types::{int4, intb};

use crate::dtype::type_metatype;
use crate::p0_knowledge::options::on_or_off;
use crate::varmap::{MapState, RangeHint, RangeType, COPY_CONSTANT};

/// Marshalling element id for the option (kuna 4000+ range; 4167 = the
/// previous high-water mark).
pub const ELEM_NULTERMINATOR: ElementId = ElementId::new("nulterminator", 4168);

/// The `nulterminator on|off` option.
pub struct OptionNulTerminator;

impl OptionNulTerminator {
    /// Parse the option and return its confirmation message.
    pub fn apply(&self, p1: &str) -> KunaResult<(bool, String)> {
        let val = on_or_off(p1)?;
        let prop = if val { "on" } else { "off" };
        Ok((val, format!("Stack array terminator absorption turned {prop}")))
    }
}

fn is_integer(meta: type_metatype) -> bool {
    meta == type_metatype::TYPE_INT || meta == type_metatype::TYPE_UINT
}

fn is_character_width(align: int4) -> bool {
    align == 1 || align == 2
}

/// The closed range the open array `cur` becomes when the hint `next` it would
/// end at is its zero terminator, and `after` is the hint sorted after `next`.
/// `None` leaves the sweep to upstream.
pub fn close_at_terminator(
    cur: &RangeHint,
    next: &RangeHint,
    after: Option<&RangeHint>,
    state: &MapState,
) -> Option<RangeHint> {
    if cur.range_type != RangeType::Open || cur.is_type_lock() {
        return None;
    }
    if !is_integer(cur.type_.get_metatype()) {
        return None;
    }
    let align = cur.type_.get_align_size();
    if !is_character_width(align) {
        return None;
    }
    if next.range_type != RangeType::Fixed || next.flags != COPY_CONSTANT || next.size != align {
        return None;
    }
    let next_meta = next.type_.get_metatype();
    if next_meta != type_metatype::TYPE_UNKNOWN && !is_integer(next_meta) {
        return None;
    }
    let dist = next.sstart.checked_sub(cur.sstart)?;
    if dist < cur.size as intb || dist % align as intb != 0 {
        return None;
    }
    if !state.is_terminator_store(next.start, next.size) {
        return None;
    }
    if state.swept_address_inside(cur.sstart, next.sstart) {
        return None;
    }
    let end = next.sstart.checked_add(align as intb)?;
    if after?.sstart < end {
        return None;
    }
    let size = int4::try_from(end - cur.sstart).ok()?;
    let mut closed = cur.clone();
    closed.size = size;
    closed.range_type = RangeType::Fixed;
    Some(closed)
}

#[cfg(test)]
#[path = "kuna_nulterminator/tests.rs"]
mod tests;
