//! (kuna) `stackstoreguard` -- every stack byte a guarded indexed store may
//! reach is one local.
//!
//! The heritage guard of `p3_dataflow/kuna_stackstoreguard.rs` keeps an indexed
//! stack byte store's effect on the slots it may write. The store prints through
//! the local at its base address, `v1[i & 7] = j`, so every slot it may reach
//! must be part of that local: a slot mapped as its own local is a separate C
//! object the store never writes.
//!
//! When the store's pointer is the stack base plus constants plus indices whose
//! known-bits masks bound them, the reach is `[base, base + max + 1)`; a pointer
//! that chooses between stack addresses (`(j & 2) ? &u.b[0] : &u.b[4]`) reaches
//! from the lowest to the end of the highest. The hints inside a reach, widened
//! over any hint that crosses either edge (an open hint by its indexed
//! elements), are replaced by one open byte array hint whose index evidence
//! runs to the end of the reach. Its element is the most specific one-byte
//! integer type a byte access inside the reach carries (an unsigned one when a
//! byte is read zero-extended), or the unknown byte. A reach holding a
//! type-locked hint keeps its hints.
//!
//! A reach holding a float hint keeps its hints too, and its stores' bases stop
//! absorbing: a float read of a byte array would print as an integer piece. If
//! the final layout then maps such a store's bytes as more than one local, the
//! function is analyzed again without the guard
//! (`Funcdata::withdraw_split_stack_store_guard`), since a store into one local
//! that a read of another never sees is worse than no guard.
//!
//! A function with a LOAD or STORE whose pointer comes from the stack base but
//! does not resolve (`(&v1 | 4) + i`) keeps upstream's layout: that access may
//! write or read any slot near the reach, and a split there is a separate local
//! it no longer reaches. Once a pass has laid out a reach, later passes skip
//! this check (`Funcdata::store_reach_committed`): a pass that lays out a smaller
//! local than an earlier one strands the pointers the earlier pass resolved
//! against the larger local (`&v19[0x20]` into a `char v19[32]`).
//!
//! Whether or not the indices are bounded, the open range at a guarded store's
//! base then absorbs every hint that starts inside a slot it absorbed
//! (`MapState::set_absorbing_bases`), so a word read inside a constant slot the
//! range swallowed stays a piece of the same local.

use std::collections::BTreeSet;
use std::rc::Rc;

use kuna_base::address::sign_extend;
use kuna_base::space::{spacetype, AddrSpace};
use kuna_base::types::{int4, intb, uintb};
use kuna_num::opcodes::OpCode;

use crate::context::{OpId, VarnodeId};
use crate::dtype::{type_metatype, Datatype, TypeFactory};
use crate::funcdata::Funcdata;
use crate::varmap::{MapState, RangeHint, RangeType};

/// Widest reach, in bytes, that is folded into one local.
const MAX_REACH: intb = 0x100;

/// Most stack addresses one pointer may choose between.
const MAX_PIECES: usize = 8;

/// Coalesce each guarded byte store's bounded reach into one open array hint,
/// and let the open range at every guarded store's base absorb what starts
/// inside a slot it absorbed. Returns the bounded pieces of the guarded stores
/// in a reach left to the upstream layout because it holds a float.
pub(crate) fn prepare_hints(
    fd: &Funcdata,
    state: &mut MapState,
    space: &Rc<AddrSpace>,
    types: &dyn TypeFactory,
) -> Vec<(intb, intb)> {
    if !fd.stack_store_guard() {
        return Vec::new();
    }
    let Some(sb) = fd.find_spacebase_input(space) else {
        return Vec::new();
    };
    let mut stores: Vec<(OpId, Vec<(intb, Option<intb>)>)> = fd
        .obank()
        .iter_code(OpCode::CPUI_STORE)
        .filter_map(|store| store_pieces(fd, store, sb, space).map(|pieces| (store, pieces)))
        .filter(|(_, pieces)| store_reach(pieces).is_some())
        .collect();
    if stores.is_empty() {
        return Vec::new();
    }
    let guarded = guarded_stores(fd, space);
    stores.retain(|(store, _)| guarded.contains(store));
    if stores.is_empty() {
        return Vec::new();
    }
    let reaches: Vec<(intb, Option<intb>)> =
        stores.iter().filter_map(|(_, pieces)| store_reach(pieces)).collect();
    let mut bounded: Vec<(intb, intb)> = reaches
        .iter()
        .filter_map(|&(lo, hi)| hi.map(|hi| (lo, hi)))
        .collect();
    bounded.sort_unstable();
    let mut merged: Vec<(intb, intb)> = Vec::new();
    for (lo, hi) in bounded {
        match merged.last_mut() {
            Some(last) if lo <= last.1 => last.1 = last.1.max(hi),
            _ => merged.push((lo, hi)),
        }
    }
    if !fd.store_reach_committed() {
        if has_unresolved_frame_access(fd, sb) {
            return Vec::new();
        }
        fd.commit_store_reach();
    }
    let mut bases: Vec<intb> = reaches.iter().map(|&(lo, _)| lo).collect();
    let mut floats = Vec::new();
    for (lo, hi) in merged {
        match coalesce_range(state, space, types, lo, hi) {
            Coalesce::Array(start) => bases.push(start),
            Coalesce::Float(flo, fhi) => {
                bases.retain(|&b| b < flo || b >= fhi);
                floats.push((flo, fhi));
            }
            Coalesce::Kept => {}
        }
    }
    bases.sort_unstable();
    bases.dedup();
    state.set_absorbing_bases(bases);
    stores
        .iter()
        .flat_map(|(_, pieces)| pieces.iter().filter_map(|&(lo, hi)| Some((lo, hi?))))
        .filter(|&(lo, hi)| floats.iter().any(|&(flo, fhi)| lo < fhi && flo < hi))
        .collect()
}

/// Does the layout map one of `pieces` as more than one local? The store then
/// prints into one of them while a read of another sees only the initializer.
pub(crate) fn splits_pieces(fd: &Funcdata, space: &Rc<AddrSpace>, pieces: &[(intb, intb)]) -> bool {
    let Some(sl) = fd.get_scope_local() else {
        return false;
    };
    let symbols: Vec<(intb, intb)> = sl
        .database()
        .scope_space_local_var_specs(sl.scope_id(), space.get_index() as usize)
        .into_iter()
        .map(|(_, ct, addr, _)| {
            let off = sign_extend(addr.get_offset() as intb, space.get_addr_size() as int4 * 8 - 1);
            (off, off + ct.get_size() as intb)
        })
        .collect();
    pieces.iter().any(|&(lo, hi)| {
        symbols.iter().filter(|&&(s, e)| s < hi && lo < e).count() > 1
    })
}

/// What `coalesce_range` did with one reach.
enum Coalesce {
    /// The hints became one open byte array starting here.
    Array(intb),
    /// A float hint lies in this widened reach, so its hints were kept.
    Float(intb, intb),
    /// The hints were kept for another reason.
    Kept,
}

/// Replace the hints overlapping `[lo, hi)`, widened over crossing hints, with one
/// open byte array.
fn coalesce_range(
    state: &mut MapState,
    space: &Rc<AddrSpace>,
    types: &dyn TypeFactory,
    lo: intb,
    hi: intb,
) -> Coalesce {
    let overlaps = |h: &RangeHint, lo: intb, hi: intb| {
        h.range_type != RangeType::Endpoint && h.sstart < hi && lo < hint_end(h)
    };
    let (mut lo, mut hi) = (lo, hi);
    loop {
        let (nlo, nhi) = state
            .hints_mut()
            .iter()
            .filter(|h| overlaps(h, lo, hi))
            .fold((lo, hi), |(l, r), h| (l.min(h.sstart), r.max(hint_end(h))));
        if (nlo, nhi) == (lo, hi) {
            break;
        }
        (lo, hi) = (nlo, nhi);
    }
    let hints = state.hints_mut();
    if hints
        .iter()
        .any(|h| overlaps(h, lo, hi) && h.type_.get_metatype() == type_metatype::TYPE_FLOAT)
    {
        return Coalesce::Float(lo, hi);
    }
    let start = space.wrap_offset(lo as uintb);
    if hi - lo > MAX_REACH || !state.covers(start, (hi - lo) as int4) {
        return Coalesce::Kept;
    }
    let hints = state.hints_mut();
    if hints
        .iter()
        .any(|h| overlaps(h, lo, hi) && h.is_type_lock())
    {
        return Coalesce::Kept;
    }
    let Some(elem) =
        byte_type(hints, lo, hi).or_else(|| types.get_base(1, type_metatype::TYPE_UNKNOWN).ok())
    else {
        return Coalesce::Kept;
    };
    hints.retain(|h| !overlaps(h, lo, hi));
    hints.push(RangeHint::new(
        start,
        1,
        lo,
        elem,
        0,
        RangeType::Open,
        (hi - lo - 1) as int4,
    ));
    Coalesce::Array(lo)
}

/// The end of a hint, counting an open hint's indexed elements.
fn hint_end(h: &RangeHint) -> intb {
    let end = h.sstart.wrapping_add(h.size as intb);
    if h.range_type != RangeType::Open || h.highind < 0 {
        return end;
    }
    let elems = (h.highind as intb + 1).saturating_mul(h.type_.get_align_size().max(1) as intb);
    end.max(h.sstart.saturating_add(elems))
}

/// The most specific one-byte integer type a fixed hint in `[lo, hi)` carries.
fn byte_type(hints: &[RangeHint], lo: intb, hi: intb) -> Option<Rc<Datatype>> {
    let mut elem: Option<Rc<Datatype>> = None;
    for h in hints {
        if h.sstart < lo
            || h.sstart >= hi
            || h.type_.get_size() != 1
            || h.range_type != RangeType::Fixed
        {
            continue;
        }
        if !matches!(
            h.type_.get_metatype(),
            type_metatype::TYPE_INT | type_metatype::TYPE_UINT
        ) {
            continue;
        }
        if elem
            .as_ref()
            .is_none_or(|e| h.type_.type_order(e).is_ok_and(|o| o < 0))
        {
            elem = Some(Rc::clone(&h.type_));
        }
    }
    elem
}

/// The STOREs a guard INDIRECT on `space` names as its effect.
fn guarded_stores(fd: &Funcdata, space: &Rc<AddrSpace>) -> BTreeSet<OpId> {
    let mut stores = BTreeSet::new();
    for id in fd.obank().iter_alive() {
        let Some(op) = fd
            .obank()
            .get(id)
            .filter(|o| o.code() == OpCode::CPUI_INDIRECT)
        else {
            continue;
        };
        let on_stack = op
            .get_out()
            .and_then(|o| fd.vbank().get(o))
            .is_some_and(|o| o.get_space().get_index() == space.get_index());
        let Some(iop) = op.get_in(1).and_then(|v| fd.vbank().get(v)) else {
            continue;
        };
        if on_stack && iop.get_space().get_type() == spacetype::IPTR_IOP {
            stores.insert(crate::funcdata_varnode::op_iop_decode(
                iop.get_addr().get_offset(),
            ));
        }
    }
    stores
}

/// The signed stack bytes `[lo, hi)` a byte store may write, `hi` unknown when
/// an index is not bounded, one piece per stack address its pointer chooses
/// between.
fn store_pieces(
    fd: &Funcdata,
    store: OpId,
    sb: VarnodeId,
    space: &Rc<AddrSpace>,
) -> Option<Vec<(intb, Option<intb>)>> {
    let op = fd.obank().get(store).filter(|o| !o.is_dead())?;
    if fd.vbank().get(op.get_in(2)?)?.get_size() != 1 {
        return None;
    }
    let bits = space.get_addr_size() as int4 * 8 - 1;
    let pieces = pointer_pieces(fd, op.get_in(1)?, sb, 0)?;
    Some(
        pieces
            .into_iter()
            .map(|(off, extra)| {
                let lo = sign_extend(space.wrap_offset(off) as intb, bits);
                (lo, extra.map(|e| lo + e + 1))
            })
            .collect(),
    )
}

/// The base of an indexed store and, when its indices are bounded, the end of
/// the bytes it may write: the span of its pieces. A store to one fixed byte
/// has none.
fn store_reach(pieces: &[(intb, Option<intb>)]) -> Option<(intb, Option<intb>)> {
    if let [(lo, hi)] = pieces {
        return (*hi != Some(lo + 1)).then_some((*lo, *hi));
    }
    let lo = pieces.iter().map(|&(lo, _)| lo).min()?;
    let hi = pieces
        .iter()
        .map(|&(_, hi)| hi)
        .collect::<Option<Vec<_>>>()?
        .into_iter()
        .max()?;
    (hi - lo <= MAX_REACH).then_some((lo, Some(hi)))
}

/// `vn` as the stack base plus a constant plus a non-negative extra, which is
/// `None` when an index's known-bits mask does not bound it. The extra starts
/// at the address the pointer names, since the C prints the access from there.
/// A phi of different stack addresses gives one such piece per address.
fn pointer_pieces(
    fd: &Funcdata,
    vn: VarnodeId,
    sb: VarnodeId,
    depth: u32,
) -> Option<Vec<(uintb, Option<intb>)>> {
    if vn == sb {
        return Some(vec![(0, Some(0))]);
    }
    if depth > 12 {
        return None;
    }
    let op = fd.obank().get(fd.vbank().get(vn)?.get_def()?)?;
    match op.code() {
        OpCode::CPUI_COPY | OpCode::CPUI_CAST => pointer_pieces(fd, op.get_in(0)?, sb, depth + 1),
        OpCode::CPUI_MULTIEQUAL => {
            let mut walk = false;
            let mut pieces: Vec<(uintb, Option<intb>)> = Vec::new();
            for k in 0..op.num_input() {
                let input = op.get_in(k)?;
                if steps_from(fd, input, vn) {
                    walk = true;
                    continue;
                }
                for piece in pointer_pieces(fd, input, sb, depth + 1)? {
                    if !pieces.contains(&piece) {
                        pieces.push(piece);
                    }
                }
            }
            let &(first, _) = pieces.first()?;
            if pieces.iter().all(|&(off, _)| off == first) {
                return Some(vec![(first, None)]);
            }
            (!walk && pieces.len() <= MAX_PIECES).then_some(pieces)
        }
        OpCode::CPUI_PTRSUB => {
            let c = fd.vbank().get(op.get_in(1)?).filter(|c| c.is_constant())?.get_offset();
            let pieces = pointer_pieces(fd, op.get_in(0)?, sb, depth + 1)?;
            Some(pieces.into_iter().map(|(off, extra)| (off.wrapping_add(c), extra)).collect())
        }
        OpCode::CPUI_PTRADD | OpCode::CPUI_INT_ADD => {
            let scale = match op.code() {
                OpCode::CPUI_PTRADD => fd.vbank().get(op.get_in(2)?)?.get_offset(),
                _ => 1,
            };
            let (pieces, term) = match pointer_pieces(fd, op.get_in(0)?, sb, depth + 1) {
                Some(base) => (base, op.get_in(1)?),
                None if op.code() == OpCode::CPUI_INT_ADD => (
                    pointer_pieces(fd, op.get_in(1)?, sb, depth + 1)?,
                    op.get_in(0)?,
                ),
                None => return None,
            };
            let t = fd.vbank().get(term)?;
            if t.is_constant() {
                let c = t.get_offset().wrapping_mul(scale);
                return Some(pieces.into_iter().map(|(off, extra)| (off.wrapping_add(c), extra)).collect());
            }
            let (index, bias) = offset_index(fd, term).unwrap_or((term, 0));
            let span = fd
                .vbank()
                .get(index)?
                .get_nz_mask()
                .checked_add(bias)
                .and_then(|m| m.checked_mul(scale))
                .filter(|&s| s < MAX_REACH as uintb)
                .map(|s| s as intb);
            Some(
                pieces
                    .into_iter()
                    .map(|(off, extra)| (off, extra.zip(span).map(|(e, s)| e + s)))
                    .collect(),
            )
        }
        _ => None,
    }
}

/// `vn` as `index + c` for a small non-negative constant `c`, which is at most
/// `c` plus the index's known-bits mask.
fn offset_index(fd: &Funcdata, vn: VarnodeId) -> Option<(VarnodeId, uintb)> {
    let op = fd.obank().get(fd.vbank().get(vn)?.get_def()?)?;
    if op.code() != OpCode::CPUI_INT_ADD {
        return None;
    }
    let c = fd.vbank().get(op.get_in(1)?).filter(|c| c.is_constant())?;
    let c = sign_extend(c.get_offset() as intb, c.get_size() * 8 - 1);
    (0..MAX_REACH).contains(&c).then_some((op.get_in(0)?, c as uintb))
}

/// Does some LOAD or STORE address come from the stack base `sb` along a path
/// `pointer_pieces` cannot resolve?
fn has_unresolved_frame_access(fd: &Funcdata, sb: VarnodeId) -> bool {
    [OpCode::CPUI_LOAD, OpCode::CPUI_STORE]
        .into_iter()
        .any(|code| {
            fd.obank().iter_code(code).any(|id| {
                fd.obank()
                    .get(id)
                    .filter(|op| !op.is_dead())
                    .and_then(|op| op.get_in(1))
                    .is_some_and(|ptr| {
                        comes_from(fd, ptr, sb) && pointer_pieces(fd, ptr, sb, 0).is_none()
                    })
            })
        })
}

/// Does `vn` reach `sb` through address arithmetic? Undecided past 64 values,
/// which counts as yes.
fn comes_from(fd: &Funcdata, vn: VarnodeId, sb: VarnodeId) -> bool {
    let mut seen = BTreeSet::new();
    let mut work = vec![vn];
    while let Some(v) = work.pop() {
        if v == sb {
            return true;
        }
        if !seen.insert(v) {
            continue;
        }
        if seen.len() > 64 {
            return true;
        }
        let Some(op) = fd
            .vbank()
            .get(v)
            .and_then(|v| v.get_def())
            .and_then(|d| fd.obank().get(d))
        else {
            continue;
        };
        if matches!(
            op.code(),
            OpCode::CPUI_COPY
                | OpCode::CPUI_CAST
                | OpCode::CPUI_INDIRECT
                | OpCode::CPUI_MULTIEQUAL
                | OpCode::CPUI_PTRSUB
                | OpCode::CPUI_PTRADD
                | OpCode::CPUI_INT_ADD
                | OpCode::CPUI_INT_SUB
                | OpCode::CPUI_INT_OR
                | OpCode::CPUI_INT_XOR
                | OpCode::CPUI_INT_AND
                | OpCode::CPUI_INT_ZEXT
                | OpCode::CPUI_INT_SEXT
                | OpCode::CPUI_SUBPIECE
                | OpCode::CPUI_PIECE
                | OpCode::CPUI_SEGMENTOP
        ) {
            let inputs = if op.code() == OpCode::CPUI_INDIRECT {
                1
            } else {
                op.num_input()
            };
            work.extend((0..inputs).filter_map(|k| op.get_in(k)));
        }
    }
    false
}

/// Is `vn` the phi output `phi` plus a constant (a pointer walk's back edge)?
fn steps_from(fd: &Funcdata, vn: VarnodeId, phi: VarnodeId) -> bool {
    let Some(op) = fd
        .vbank()
        .get(vn)
        .and_then(|v| v.get_def())
        .and_then(|d| fd.obank().get(d))
    else {
        return false;
    };
    let constant = |k: int4| {
        op.get_in(k)
            .and_then(|v| fd.vbank().get(v))
            .is_some_and(|v| v.is_constant())
    };
    matches!(op.code(), OpCode::CPUI_INT_ADD | OpCode::CPUI_PTRADD)
        && op.get_in(0) == Some(phi)
        && constant(1)
}
