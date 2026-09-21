//! (kuna `libctypes`) A libc aggregate is not the type of a pointer the caller
//! reaches past the end of.
//!
//! `libctypes` declares `void _obstack_newchunk(obstack *, size_t)`, and
//! `TypeOpCall::getInputLocal` (typeop.cc:689) turns that declaration into a vote
//! about the Varnode the call reads. The vote is right when the Varnode points at
//! an obstack and wrong when it points at an object whose FIRST member is one:
//! `&kwset->obstack` and `kwset` are the same address, so grep's
//! `kwsincr(kwset_t, char const *, idx_t)` was declared `obstack *a0`, and every
//! other member it touches rendered as an element of an obstack array
//! (`&v1[0x1b].field_0x38` for offset 0x980 of the 88-byte struct). A
//! `pthread_mutex_t` at the head of a larger record is the same shape.
//!
//! What the caller does through the pointer says which one it is. [`overruns`]
//! walks the Varnode the call reads, through its descendants, with the walker
//! `protoorder` holds its own recovered votes to
//! (`kuna_protoorder::accesses_through`), and looks for a
//! load, a store or a derived address outside the aggregate: at a constant
//! offset at or past its end or before its start, or stepped by a constant at
//! least as large as the aggregate but not equal to it (coreutils `wc` walks
//! `&fstatus[i].st`, a `stat` 8 bytes into a 152-byte record, and reads
//! `failed` at -8). One is enough. The vote then falls back to `void *`,
//! the width-stable spelling the shipped tables give the same slot. The declared
//! type is still what the call's cast is measured against
//! (`declared_input_type_local`), so a value that settles on another type
//! prints `_obstack_newchunk((obstack *)a0,n)`, and a `void *` needs no cast.
//!
//! Only what is proven is refused. A step smaller than the aggregate keeps the
//! vote (an unknown index, a word-at-a-time struct copy, a phi between two
//! fields), so does an array of the aggregate itself (sdiff's `struct sigaction`
//! table, stepped by exactly 152), and so does a walk that runs out of budget.
//!
//! The same question guards type PROPAGATION ([`refuses`]): the type is not
//! carried onto a Varnode whose own descendants reach past the end, whichever
//! call it came from. Walking descendants rather than the whole COPY family is
//! what keeps the answer per Varnode: at `-O0` a macro local holding a copy of
//! the pointer and the pointer itself are different Varnodes.
//! The rule applies to the names `libctypes` owns
//! ([`crate::kuna_libctypes::AGGREGATE_NAMES`]) and only while that option is
//! on, so `libctypes off` is untouched.

use std::rc::Rc;

use crate::context::{OpId, VarnodeId};
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;
use kuna_base::types::int4;

/// Does the value call `op` reads in `slot` get used outside the libc
/// aggregate `ct` points at?
pub(crate) fn overruns(data: &Funcdata, op: OpId, slot: int4, ct: &Datatype) -> bool {
    if !data.get_arch().libctypes {
        return false;
    }
    let Some(size) = libc_pointee_size(ct) else { return false };
    let Some(vn) = data.obank().get(op).and_then(|o| o.get_in(slot)) else { return false };
    reached_outside(data, vn, size)
}

/// Should type propagation refuse to carry `ct` onto `vn`? The same question
/// [`overruns`] asks at a call, asked of the Varnode the type would land on.
pub(crate) fn refuses(data: &Funcdata, vn: VarnodeId, ct: &Datatype) -> bool {
    if !data.get_arch().libctypes {
        return false;
    }
    let Some(size) = libc_pointee_size(ct) else { return false };
    reached_outside(data, vn, size)
}

/// The `void *` the vote falls back to, as wide as the declared pointer.
pub(crate) fn width_stable_vote(data: &Funcdata, ct: &Datatype) -> Option<Rc<Datatype>> {
    let tlst = data.get_arch().types()?;
    let void = tlst.get_type_void().ok()?;
    let ws = ct.get_word_size().unwrap_or(1);
    tlst.get_type_pointer(ct.get_size(), void, ws).ok()
}

fn libc_pointee_size(ct: &Datatype) -> Option<i64> {
    if ct.get_metatype() != type_metatype::TYPE_PTR {
        return None;
    }
    let pointee = ct.get_ptr_to()?;
    if pointee.get_metatype() != type_metatype::TYPE_STRUCT
        || !crate::kuna_libctypes::AGGREGATE_NAMES.contains(&pointee.get_name())
    {
        return None;
    }
    Some(pointee.get_size() as i64).filter(|&s| s > 0)
}

fn reached_outside(data: &Funcdata, vn: VarnodeId, size: i64) -> bool {
    let Some((accesses, places)) = crate::kuna_protoorder::accesses_through(data, &[vn]) else {
        return false;
    };
    places.iter().any(|&(at, stride)| !fits(at, stride, 1, size))
        || accesses.iter().any(|a| !fits(a.at, a.stride, a.size as i64, size))
}

/// Can `width` bytes at `at` (plus any multiple of `stride`) lie inside an
/// object of `size` bytes, or inside one element of an array of them? A derived
/// address is measured as one byte, so a pointer one past the end (`&v1[1]`
/// handed to `pthread_cond_init`) is outside. A step smaller than the aggregate
/// proves nothing: it is an unknown index (one byte), a copy loop walking the
/// object a word at a time, or a phi choosing between two of its fields. An
/// array of the aggregate is stepped by exactly its size; any other step at least
/// that large is a record the aggregate sits inside (libselinux indexes 120-byte
/// records holding a 40-byte mutex at +0x38).
fn fits(at: i64, stride: i64, width: i64, size: i64) -> bool {
    match stride.abs() {
        0 => at >= 0 && at + width <= size,
        s if s < size => true,
        s if s == size => at.rem_euclid(size) + width <= size,
        _ => false,
    }
}

#[cfg(test)]
mod tests;
