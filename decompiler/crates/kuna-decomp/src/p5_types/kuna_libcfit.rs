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
//! walks the value the call reads with the walkers `protoorder` holds its own
//! recovered votes to (`kuna_protoorder::accesses_through`), and looks for a
//! load, a store or a derived address at a constant offset outside the
//! aggregate. One is enough: nothing reads an N-byte object at offset N or past
//! it, or at a negative offset. The vote then falls back to `void *`, the
//! width-stable spelling the shipped tables give the same slot. The declared
//! type is still what the call's cast is measured against
//! (`declared_input_type_local`), so a value that settles on another type
//! prints `_obstack_newchunk((obstack *)a0,n)`, and a `void *` needs no cast.
//!
//! Only what is proven is refused: a variable index and a walk that runs out of
//! budget keep the vote. The rule applies to the names `libctypes` owns
//! ([`crate::kuna_libctypes::AGGREGATE_NAMES`]) and only while that option is on,
//! so `libctypes off` is untouched.

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
    use crate::kuna_protoorder::{accesses_through, value_family, with_sibling_loads};
    let family = with_sibling_loads(data, &value_family(data, vn));
    let Some((accesses, places)) = accesses_through(data, &family) else { return false };
    places.iter().any(|&(at, stride)| stride == 0 && !(0..=size).contains(&at))
        || accesses
            .iter()
            .any(|a| a.stride == 0 && (a.at < 0 || a.at + a.size as i64 > size))
}

#[cfg(test)]
mod tests;
