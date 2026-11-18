//! (kuna) `arrayextent` -- an indexed stack array covers the slots its index
//! reaches past the four elements upstream assumes.
//!
//! `MapState::gatherOpen` gives the open range at an indexed stack base room
//! for four elements, and `RangeHint::attempt_join` refuses a fixed hint past
//! that, so an element the index reaches beyond the fourth is laid out as its
//! own scalar and the printed subscript runs off the declared array.
//!
//! At `bound`, an indexed base whose pointer is the stack base plus constants
//! plus indices whose known bits bound them (`kuna_storereach.rs
//! (pointer_pieces)`), and which the function only dereferences, gets room for
//! every element those dereferences may touch.
//!
//! At `on`, an indexed base with no such bound and no range-locked guard over
//! it also takes each following slot of its element size while that slot
//! starts exactly where the elements it covers end, starts no other open range,
//! holds only unlocked hints of its element size and of a type `attempt_join`
//! accepts, and is never read except through a pointer. A slot the function
//! writes and never reads directly is only observable through the index, so it
//! is an element of the array rather than a scalar of its own. When
//! upstream's merge (`ScopeLocal::restructure`), run over the same sorted hints
//! with the array at its old length, ends the array at or before the last slot
//! taken, that slot loses its constant-copy mark, so upstream's constant
//! absorption (`RangeHint::absorb`) cannot raise the array one element past it
//! and from there over each following constant-initialised local. Where
//! upstream's merge reaches further on its own, the array is upstream's.

use std::rc::Rc;

use kuna_base::address::{sign_extend, Address};
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::space::AddrSpace;
use kuna_base::types::{int4, intb, uintb};
use kuna_num::opcodes::OpCode;

use crate::context::VarnodeId;
use crate::dtype::{type_metatype, Datatype, TypeFactory};
use crate::funcdata::Funcdata;
use crate::varmap::{MapState, RangeHint, RangeType, COPY_CONSTANT};

/// Upstream's four-element assumption only.
pub const LEVEL_OFF: int4 = 0;
/// Also size an indexed base from the known bits of its indices.
pub const LEVEL_BOUND: int4 = 1;
/// Also extend an unbounded indexed base over contiguous write-only slots.
pub const LEVEL_ON: int4 = 2;

/// Most slots one unbounded base takes past the elements its layout pass
/// starts with, however many alias bases or guards share its start.
const MAX_FOLLOW: int4 = 256;

/// `option arrayextent off|bound|on`.
pub struct OptionArrayExtent;

impl OptionArrayExtent {
    /// The option name.
    pub const NAME: &'static str = "arrayextent";

    /// Parse the level and its confirmation message; the caller writes it into
    /// `Architecture::array_extent`.
    pub fn apply(&self, p1: &str) -> KunaResult<(int4, String)> {
        match p1 {
            "off" => Ok((LEVEL_OFF, "Indexed stack array extent turned off".to_string())),
            "bound" => Ok((
                LEVEL_BOUND,
                "Indexed stack array extent turned on, known-bits bound only".to_string(),
            )),
            "on" | "" => Ok((
                LEVEL_ON,
                "Indexed stack array extent turned on, bound plus write-only slots".to_string(),
            )),
            _ => Err(KunaError::parse("Must specify one of off, bound, on")),
        }
    }
}

/// An indexed base `extend_unbounded` may lengthen: its open hint's start and
/// element size.
#[derive(Clone, Copy)]
pub(crate) struct OpenBase {
    pub(crate) start: uintb,
    pub(crate) elem: int4,
}

/// How many `elem`-byte elements from `offset` the dereferences through the
/// indexed pointer `base` may touch, when every index on its path from the
/// stack base is bounded and `base` is used only as a LOAD or STORE address or
/// in further address arithmetic. The count stops at `next`, the closest other
/// pointer base above `offset`: a known-bits bound is loose (a byte index
/// reaches 256), and the address taken there is another object.
pub(crate) fn bounded_items(
    fd: &Funcdata,
    space: &Rc<AddrSpace>,
    base: VarnodeId,
    offset: uintb,
    next: Option<uintb>,
    elem: int4,
) -> Option<int4> {
    if elem <= 0 {
        return None;
    }
    let sb = fd.find_spacebase_input(space)?;
    let access = access_size(fd, base)?;
    let pieces = super::kuna_storereach::pointer_pieces(fd, base, sb, 0)?;
    let [(off, Some(extra))] = pieces.as_slice() else {
        return None;
    };
    let bits = space.get_addr_size() as int4 * 8 - 1;
    let signed = |o: uintb| sign_extend(space.wrap_offset(o) as intb, bits);
    let start = signed(offset);
    let mut reach = signed(*off) + extra + access as intb - start;
    if let Some(next) = next.map(signed).filter(|&n| n > start) {
        reach = reach.min(next - start);
    }
    if reach <= 0 {
        return None;
    }
    Some(((reach + elem as intb - 1) / elem as intb) as int4)
}

/// The closest pointer base in `offsets` above `offset`, in signed frame
/// order.
pub(crate) fn next_base(space: &Rc<AddrSpace>, offsets: &[uintb], offset: uintb) -> Option<uintb> {
    let bits = space.get_addr_size() as int4 * 8 - 1;
    let signed = |o: uintb| sign_extend(space.wrap_offset(o) as intb, bits);
    let start = signed(offset);
    offsets
        .iter()
        .copied()
        .filter(|&o| signed(o) > start)
        .min_by_key(|&o| signed(o))
}

/// The widest LOAD or STORE through `vn`, or `None` when `vn` also escapes
/// (passed, stored, compared, merged) or is never dereferenced.
fn access_size(fd: &Funcdata, vn: VarnodeId) -> Option<int4> {
    let mut size = 0;
    for use_op in fd.vbank().get(vn)?.descend_iter() {
        let op = fd.obank().get(use_op)?;
        let as_addr = op.get_in(1) == Some(vn);
        let width = match op.code() {
            OpCode::CPUI_LOAD if as_addr => fd.vbank().get(op.get_out()?)?.get_size(),
            OpCode::CPUI_STORE if as_addr && op.get_in(2) != Some(vn) => {
                fd.vbank().get(op.get_in(2)?)?.get_size()
            }
            OpCode::CPUI_COPY
            | OpCode::CPUI_INT_ADD
            | OpCode::CPUI_PTRADD
            | OpCode::CPUI_PTRSUB
            | OpCode::CPUI_SEGMENTOP => continue,
            OpCode::CPUI_INT_SUB if op.get_in(0) == Some(vn) => continue,
            _ => return None,
        };
        size = size.max(width);
    }
    (size > 0).then_some(size)
}

/// An array `extend_unbounded` lengthened: its open hint's offset, start and
/// element size, and its high index before and after.
#[derive(Clone, Copy)]
pub(crate) struct Extended {
    start: uintb,
    sstart: intb,
    elem: int4,
    from: int4,
    to: int4,
}

/// Lengthen the open hint of each unbounded indexed base over the contiguous
/// write-only slots that follow its elements, and record each array lengthened
/// for [`settle`].
pub(crate) fn extend_unbounded(
    fd: &Funcdata,
    state: &mut MapState,
    space: &Rc<AddrSpace>,
    bases: &[OpenBase],
) {
    if bases.is_empty() {
        return;
    }
    let Some(end) = state.end_sstart() else {
        return;
    };
    let mut order: Vec<(intb, usize)> = state
        .hints_mut()
        .iter()
        .enumerate()
        .map(|(i, h)| (h.sstart, i))
        .collect();
    order.sort_unstable();
    let mut extended: Vec<Extended> = Vec::new();
    for base in bases {
        if base.elem <= 0 || locked_guard_over(fd, base.start) {
            continue;
        }
        let hints = state.hints_mut();
        let Some(open) = hints
            .iter()
            .filter(|h| is_indexed_open(h, base))
            .max_by_key(|h| h.highind)
        else {
            continue;
        };
        let (sstart, highind, ty) = (open.sstart, open.highind, Rc::clone(&open.type_));
        let elem = base.elem as intb;
        let same = |e: &Extended| e.start == base.start && e.elem == base.elem;
        let first = extended.iter().find(|e| same(e)).map_or(highind, |e| e.from);
        let mut reach = highind;
        while reach < first + MAX_FOLLOW {
            let slot = sstart + (reach as intb + 1) * elem;
            if slot + elem > end
                || crosses(hints, &order, sstart, slot)
                || !takes_slot(fd, space, hints, &order, sstart, slot, elem, &ty)
            {
                break;
            }
            reach += 1;
        }
        if reach == highind {
            continue;
        }
        for h in hints.iter_mut().filter(|h| is_indexed_open(h, base)) {
            h.highind = h.highind.max(reach);
        }
        match extended.iter_mut().find(|e| same(e)) {
            Some(e) => e.to = reach,
            None => extended.push(Extended { start: base.start, sstart, elem: base.elem, from: highind, to: reach }),
        }
    }
    state.set_extended(extended);
}

/// Before `ScopeLocal::restructure` merges the sorted hints, clear
/// `COPY_CONSTANT` on the last slot each lengthened array took, unless
/// upstream's merge of the same hints, with the arrays at their old length,
/// carries the array past that slot anyway. A constant there would otherwise
/// let `RangeHint::absorb` raise the array one element past it and from there
/// over each following constant-initialised local.
pub(crate) fn settle(state: &mut MapState, space: &Rc<AddrSpace>, types: &dyn TypeFactory) {
    let extended = state.take_extended();
    if extended.is_empty() {
        return;
    }
    let mut list = state.hints_mut().clone();
    for h in list.iter_mut() {
        if let Some(e) = extended.iter().find(|e| is_lengthened(h, e)) {
            h.highind = e.from;
        }
    }
    let Some(ends) = upstream_ends(state, &list, &extended, space, types) else {
        return;
    };
    for (e, end) in extended.iter().zip(ends) {
        let last = e.sstart + e.to as intb * e.elem as intb;
        if end.is_some_and(|end| end <= last + e.elem as intb) {
            for h in state
                .hints_mut()
                .iter_mut()
                .filter(|h| h.range_type == RangeType::Fixed && h.sstart == last)
            {
                h.flags &= !COPY_CONSTANT;
            }
        }
    }
}

/// Is `h` an open hint `extend_unbounded` raised for `e`?
fn is_lengthened(h: &RangeHint, e: &Extended) -> bool {
    h.range_type == RangeType::Open
        && h.start == e.start
        && h.type_.get_align_size() == e.elem
        && h.highind == e.to
}

/// Where each array in `extended` ends when `ScopeLocal::restructure`'s merge
/// runs over the sorted `list`, the same steps on copies: `None` for one the
/// merge never closes, and `None` overall if a merge fails. Ends at the next
/// hint for an open range, at its own end for a fixed one.
fn upstream_ends(
    state: &MapState,
    list: &[RangeHint],
    extended: &[Extended],
    space: &Rc<AddrSpace>,
    types: &dyn TypeFactory,
) -> Option<Vec<Option<intb>>> {
    let mut ends = vec![None; extended.len()];
    let mut cur = list.first()?.clone();
    let mut held: Vec<usize> = bases_at(extended, &cur).collect();
    let mut cur_end = cur.sstart.wrapping_add(cur.size as intb);
    for next in &list[1..] {
        let next_end = next.sstart.wrapping_add(next.size as intb);
        let absorbing = state.is_absorbing(cur.sstart);
        if next.sstart < cur.sstart.wrapping_add(cur.size as intb) {
            let was_open = cur.range_type == RangeType::Open;
            cur.merge(next, space, types).ok()?;
            cur_end = cur_end.max(next_end);
            let aggregate = matches!(
                cur.type_.get_metatype(),
                type_metatype::TYPE_STRUCT | type_metatype::TYPE_UNION | type_metatype::TYPE_ARRAY
            );
            if absorbing && was_open && !cur.is_type_lock() && !aggregate {
                cur.range_type = RangeType::Open;
            }
        } else if cur.range_type == RangeType::Open
            && absorbing
            && next.range_type != RangeType::Endpoint
            && !next.is_type_lock()
            && next.sstart < cur_end
        {
            cur.absorb(next);
            cur_end = cur_end.max(next_end);
        } else if cur.attempt_join(next) {
            cur_end = cur_end.max(next_end);
        } else {
            let end = match cur.range_type {
                RangeType::Open => next.sstart,
                _ => cur.sstart.wrapping_add(cur.size as intb),
            };
            for &e in &held {
                ends[e] = Some(end);
            }
            held.clear();
            cur = next.clone();
            cur_end = next_end;
        }
        held.extend(bases_at(extended, next));
    }
    Some(ends)
}

/// The entries of `extended` whose array starts at the open hint `h`.
fn bases_at<'a>(extended: &'a [Extended], h: &'a RangeHint) -> impl Iterator<Item = usize> + 'a {
    (0..extended.len())
        .filter(move |&e| h.range_type == RangeType::Open && h.start == extended[e].start)
}

/// Is `h` an open hint with index evidence at `base`'s start and element size?
fn is_indexed_open(h: &RangeHint, base: &OpenBase) -> bool {
    h.range_type == RangeType::Open
        && h.highind >= 0
        && h.start == base.start
        && !h.is_type_lock()
        && h.type_.get_align_size() == base.elem
}

/// Does a valid guard whose range the value-set analysis locked cover `start`?
fn locked_guard_over(fd: &Funcdata, start: uintb) -> bool {
    let covers = |g: &crate::heritage::LoadGuard, opc: OpCode| {
        g.is_range_locked()
            && g.is_valid(fd, opc)
            && g.get_minimum() <= start
            && start <= g.get_maximum()
    };
    fd.get_load_guards().iter().any(|g| covers(g, OpCode::CPUI_LOAD))
        || fd.get_store_guards().iter().any(|g| covers(g, OpCode::CPUI_STORE))
}

/// Does a fixed hint starting in `[from, slot)` run past `slot`?
fn crosses(hints: &[RangeHint], order: &[(intb, usize)], from: intb, slot: intb) -> bool {
    let first = order.partition_point(|&(s, _)| s < from);
    order[first..]
        .iter()
        .take_while(|&&(s, _)| s < slot)
        .map(|&(_, i)| &hints[i])
        .any(|h| h.range_type == RangeType::Fixed && h.sstart + h.size as intb > slot)
}

/// May an open range of element type `ty` starting at `start` take the
/// `elem`-byte slot at `slot`?
#[allow(clippy::too_many_arguments)]
fn takes_slot(
    fd: &Funcdata,
    space: &Rc<AddrSpace>,
    hints: &[RangeHint],
    order: &[(intb, usize)],
    start: intb,
    slot: intb,
    elem: intb,
    ty: &Rc<Datatype>,
) -> bool {
    let first = order.partition_point(|&(s, _)| s < slot);
    let inside: Vec<&RangeHint> = order[first..]
        .iter()
        .take_while(|&&(s, _)| s < slot + elem)
        .map(|&(_, i)| &hints[i])
        .collect();
    let Some(_) = inside.first() else {
        return false;
    };
    inside.iter().all(|h| {
        h.sstart == slot
            && h.range_type == RangeType::Fixed
            && !h.is_type_lock()
            && h.size as intb == elem
            && joinable(ty, &h.type_)
    }) && !observed(fd, space, start, slot, elem)
}

/// Would `RangeHint::attempt_join` keep an element of type `a` across a hint
/// of type `b` without retyping the array: `b` is unknown, an integer joining
/// an unknown or opposite-signed integer, or `a` itself behind the same number
/// of pointers.
fn joinable(a: &Rc<Datatype>, b: &Rc<Datatype>) -> bool {
    if a.get_align_size() != b.get_align_size() {
        return false;
    }
    let int = |m| matches!(m, type_metatype::TYPE_INT | type_metatype::TYPE_UINT);
    let (am, bm) = (a.get_metatype(), b.get_metatype());
    if bm == type_metatype::TYPE_UNKNOWN || (int(bm) && (int(am) || am == type_metatype::TYPE_UNKNOWN)) {
        return true;
    }
    let (mut a, mut b) = (Rc::clone(a), Rc::clone(b));
    while a.get_metatype() == type_metatype::TYPE_PTR && b.get_metatype() == type_metatype::TYPE_PTR {
        match (a.get_ptr_to(), b.get_ptr_to()) {
            (Some(an), Some(bn)) => (a, b) = (an, bn),
            _ => break,
        }
    }
    Rc::ptr_eq(&a, &b)
}

/// Can the function observe the value in `[slot, slot + elem)` other than
/// through a pointer? A stack varnode there is read by an op other than a guard
/// INDIRECT or MULTIEQUAL, or a value copied into it is also used by anything
/// but copies into stack slots outside the array's `[start, slot + elem)`.
/// The second covers a scalar whose reads were propagated to the register it
/// was stored from (`fmt = f(); if (fmt == 2)` at -O0), and a value copied
/// into the array's own elements as well (`float t = ...; float a[3] = {t, t,
/// u};`); a parameter spilled to its home slot and copied into the array is
/// still an element.
fn observed(fd: &Funcdata, space: &Rc<AddrSpace>, start: intb, slot: intb, elem: intb) -> bool {
    let bits = space.get_addr_size() as int4 * 8 - 1;
    let frame_slot = |vn: VarnodeId| {
        fd.vbank()
            .get(vn)
            .filter(|v| v.get_space().get_index() == space.get_index())
            .map(|v| {
                let off = sign_extend(v.get_addr().get_offset() as intb, bits);
                (off, off + v.get_size() as intb)
            })
    };
    let lo = Address::new(Rc::clone(space), space.wrap_offset((slot - 16) as uintb));
    let hi = Address::new(Rc::clone(space), space.wrap_offset((slot + elem) as uintb));
    let end = slot + elem;
    fd.vbank().iter_loc_addr_range(&lo, &hi).any(|vn| {
        if !frame_slot(vn).is_some_and(|(s, e)| s < end && slot < e) {
            return false;
        }
        if super::kuna_storereach::is_read(fd, vn) {
            return true;
        }
        let Some(def) = fd.vbank().get(vn).and_then(|v| v.get_def()) else {
            return false;
        };
        let Some(src) = fd
            .obank()
            .get(def)
            .filter(|op| op.code() == OpCode::CPUI_COPY)
            .and_then(|op| op.get_in(0))
            .and_then(|v| fd.vbank().get(v))
            .filter(|v| !v.is_constant())
        else {
            return false;
        };
        src.descend_iter().any(|use_op| {
            use_op != def
                && !fd
                    .obank()
                    .get(use_op)
                    .filter(|op| op.code() == OpCode::CPUI_COPY)
                    .and_then(|op| op.get_out())
                    .and_then(frame_slot)
                    .is_some_and(|(s, e)| e <= start || end <= s)
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn option_levels_parse() {
        assert_eq!(OptionArrayExtent.apply("off").unwrap().0, LEVEL_OFF);
        assert_eq!(OptionArrayExtent.apply("bound").unwrap().0, LEVEL_BOUND);
        assert_eq!(OptionArrayExtent.apply("on").unwrap().0, LEVEL_ON);
        assert_eq!(OptionArrayExtent.apply("").unwrap().0, LEVEL_ON);
        assert!(OptionArrayExtent.apply("full").is_err());
    }
}
