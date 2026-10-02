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
//! known-bits masks bound them, the reach is `[base, base + max + 1)`. The
//! hints inside it, widened over any hint that crosses either edge, are replaced
//! by one open byte array hint whose index evidence runs to the end of the
//! reach. Its element is the most specific one-byte integer type a byte access
//! inside the reach carries (an unsigned one when a byte is read zero-extended),
//! or the unknown byte. A reach holding a type-locked hint keeps its hints, and
//! one holding a float hint keeps them and its stores' bases stop absorbing: a
//! float read of a byte array would print as an integer piece.
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

/// Coalesce each guarded byte store's bounded reach into one open array hint,
/// and let the open range at every guarded store's base absorb what starts
/// inside a slot it absorbed.
pub(crate) fn prepare_hints(
    fd: &Funcdata,
    state: &mut MapState,
    space: &Rc<AddrSpace>,
    types: &dyn TypeFactory,
) {
    if !fd.get_arch().stack_store_guard {
        return;
    }
    let Some(sb) = fd.find_spacebase_input(space) else {
        return;
    };
    let mut reaches: Vec<(OpId, intb, Option<intb>)> = fd
        .obank()
        .iter_code(OpCode::CPUI_STORE)
        .filter_map(|store| store_reach(fd, store, sb, space).map(|(lo, hi)| (store, lo, hi)))
        .collect();
    if reaches.is_empty() {
        return;
    }
    let guarded = guarded_stores(fd, space);
    reaches.retain(|(store, _, _)| guarded.contains(store));
    if reaches.is_empty() {
        return;
    }
    if !fd.store_reach_committed() {
        if has_unresolved_frame_access(fd, sb) {
            return;
        }
        fd.commit_store_reach();
    }
    let mut bases: Vec<intb> = reaches.iter().map(|&(_, lo, _)| lo).collect();
    let mut bounded: Vec<(intb, intb)> = reaches
        .iter()
        .filter_map(|&(_, lo, hi)| hi.map(|hi| (lo, hi)))
        .collect();
    bounded.sort_unstable();
    let mut merged: Vec<(intb, intb)> = Vec::new();
    for (lo, hi) in bounded {
        match merged.last_mut() {
            Some(last) if lo <= last.1 => last.1 = last.1.max(hi),
            _ => merged.push((lo, hi)),
        }
    }
    for (lo, hi) in merged {
        match coalesce_range(state, space, types, lo, hi) {
            Coalesce::Array(start) => bases.push(start),
            Coalesce::Float(flo, fhi) => bases.retain(|&b| b < flo || b >= fhi),
            Coalesce::Kept => {}
        }
    }
    bases.sort_unstable();
    bases.dedup();
    state.set_absorbing_bases(bases);
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
        h.range_type != RangeType::Endpoint
            && h.sstart < hi
            && lo < h.sstart.wrapping_add(h.size as intb)
    };
    let (mut lo, mut hi) = (lo, hi);
    loop {
        let (nlo, nhi) = state
            .hints_mut()
            .iter()
            .filter(|h| overlaps(h, lo, hi))
            .fold((lo, hi), |(l, r), h| {
                (
                    l.min(h.sstart),
                    r.max(h.sstart.wrapping_add(h.size as intb)),
                )
            });
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

/// The signed base of an indexed stack byte store and, when its indices are
/// bounded, the end of the bytes it may write.
fn store_reach(
    fd: &Funcdata,
    store: OpId,
    sb: VarnodeId,
    space: &Rc<AddrSpace>,
) -> Option<(intb, Option<intb>)> {
    let op = fd.obank().get(store).filter(|o| !o.is_dead())?;
    if fd.vbank().get(op.get_in(2)?)?.get_size() != 1 {
        return None;
    }
    let (off, extra) = pointer_reach(fd, op.get_in(1)?, sb, 0)?;
    if extra == Some(0) {
        return None;
    }
    let lo = sign_extend(
        space.wrap_offset(off) as intb,
        space.get_addr_size() as int4 * 8 - 1,
    );
    Some((lo, extra.map(|e| lo + e + 1)))
}

/// `vn` as the stack base plus a constant plus a non-negative extra, which is
/// `None` when an index's known-bits mask does not bound it.
fn pointer_reach(
    fd: &Funcdata,
    vn: VarnodeId,
    sb: VarnodeId,
    depth: u32,
) -> Option<(uintb, Option<intb>)> {
    if vn == sb {
        return Some((0, Some(0)));
    }
    if depth > 12 {
        return None;
    }
    let op = fd.obank().get(fd.vbank().get(vn)?.get_def()?)?;
    match op.code() {
        OpCode::CPUI_COPY | OpCode::CPUI_CAST => pointer_reach(fd, op.get_in(0)?, sb, depth + 1),
        OpCode::CPUI_MULTIEQUAL => {
            let mut base = None;
            for k in 0..op.num_input() {
                let input = op.get_in(k)?;
                if steps_from(fd, input, vn) {
                    continue;
                }
                let (off, _) = pointer_reach(fd, input, sb, depth + 1)?;
                if base.is_some_and(|b| b != off) {
                    return None;
                }
                base = Some(off);
            }
            base.map(|off| (off, None))
        }
        OpCode::CPUI_PTRSUB => {
            let (off, extra) = pointer_reach(fd, op.get_in(0)?, sb, depth + 1)?;
            let c = fd.vbank().get(op.get_in(1)?).filter(|c| c.is_constant())?;
            Some((off.wrapping_add(c.get_offset()), extra))
        }
        OpCode::CPUI_PTRADD | OpCode::CPUI_INT_ADD => {
            let scale = match op.code() {
                OpCode::CPUI_PTRADD => fd.vbank().get(op.get_in(2)?)?.get_offset(),
                _ => 1,
            };
            let ((off, extra), term) = match pointer_reach(fd, op.get_in(0)?, sb, depth + 1) {
                Some(base) => (base, op.get_in(1)?),
                None if op.code() == OpCode::CPUI_INT_ADD => (
                    pointer_reach(fd, op.get_in(1)?, sb, depth + 1)?,
                    op.get_in(0)?,
                ),
                None => return None,
            };
            let t = fd.vbank().get(term)?;
            if t.is_constant() {
                return Some((off.wrapping_add(t.get_offset().wrapping_mul(scale)), extra));
            }
            let span = t
                .get_nz_mask()
                .checked_mul(scale)
                .filter(|&s| s < MAX_REACH as uintb);
            Some((off, extra.zip(span).map(|(e, s)| e + s as intb)))
        }
        _ => None,
    }
}

/// Does some LOAD or STORE address come from the stack base `sb` along a path
/// `pointer_reach` cannot resolve?
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
                        comes_from(fd, ptr, sb) && pointer_reach(fd, ptr, sb, 0).is_none()
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
