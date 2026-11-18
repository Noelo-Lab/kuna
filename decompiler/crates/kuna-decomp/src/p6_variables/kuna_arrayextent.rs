//! (kuna) `arrayextent` recovers indexed frame-storage extents and preserves
//! existing byte views through runtime-stack allocation recovery.
//!
//! `bound` extends indexed storage using known index bits. `on` also includes
//! contiguous, unlocked, same-width slots only observed through a pointer.
//! Separately addressed or directly read slots stop extension. Indexed bases
//! retain a wider extent already produced by upstream layout.

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

/// Preserve a pre-recovery byte-array view only over its current written prefix.
/// Return false before changing hints if a prefix would exclude live side effects.
pub(crate) fn preserve_stackalloc_bytes(
    fd: &Funcdata,
    state: &mut MapState,
    space: &Rc<AddrSpace>,
    previous: &[(uintb, Rc<Datatype>, bool)],
    types: &dyn TypeFactory,
) -> bool {
    let bits = space.get_addr_size() as int4 * 8 - 1;
    let mut planned = Vec::new();
    for (start, array, locked) in previous {
        if *locked || array.get_metatype() != type_metatype::TYPE_ARRAY
            || array.get_size() > MAX_FOLLOW + 1 {
            continue;
        }
        let Some(byte) = array.get_array_base().filter(|ty| {
            ty.get_size() == 1 && ty.get_align_size() == 1
                && matches!(ty.get_metatype(), type_metatype::TYPE_UNKNOWN
                    | type_metatype::TYPE_INT | type_metatype::TYPE_UINT)
        }) else {
            continue;
        };
        let sstart = sign_extend(space.wrap_offset(*start) as intb, bits);
        let old_end = sstart + array.get_size() as intb;
        if state.hints_mut().iter().any(|h| h.range_type == RangeType::Open
            && h.highind >= 0 && h.sstart < old_end
            && sstart < h.sstart + (h.highind as intb + 1) * h.type_.get_align_size() as intb)
        {
            continue;
        }
        let compatible = |ty: &Datatype| ty.get_size() == 1
            && (ty.get_metatype() == byte.get_metatype()
                || ty.get_metatype() == type_metatype::TYPE_UNKNOWN);
        let mut count = 0;
        let mut escaped = false;
        for index in 0..array.get_size() {
            let slot = sstart + index as intb;
            let offset = space.wrap_offset(slot as uintb);
            if !state.covers(offset, 1) || locked_guard_over(fd, offset)
                || previous.iter().any(|(other, _, _)| *other != *start
                    && sign_extend(space.wrap_offset(*other) as intb, bits) == slot)
            {
                break;
            }
            let mut open = false;
            if !state.hints_mut().iter().filter(|h| h.sstart <= slot
                && slot < h.sstart + h.size as intb).all(|h| {
                    if h.sstart != slot || h.size != 1 || h.is_type_lock()
                        || !compatible(&h.type_) { return false; }
                    match h.range_type {
                        RangeType::Fixed => true,
                        RangeType::Open if h.highind < 0 => { open = true; true }
                        _ => false,
                    }
                })
            {
                break;
            }
            let at = Address::new(Rc::clone(space), offset);
            let end = Address::new(Rc::clone(space), space.wrap_offset(offset.wrapping_add(1)));
            let mut written = false;
            let mut affected = false;
            let invalid = fd.vbank().iter_loc_addr_range(&at, &end).any(|vn| {
                let Some(value) = fd.vbank().get(vn) else { return false };
                if value.is_free() { return false; }
                if value.get_size() != 1 || value.is_type_lock()
                    || super::kuna_storereach::is_read(fd, vn) { return true; }
                written |= value.get_def().and_then(|id| fd.obank().get(id))
                    .is_some_and(|op| {
                        if op.is_dead() { return false; }
                        affected |= op.code() == OpCode::CPUI_INDIRECT;
                        if !op.is_marker() { return true; }
                        op.code() == OpCode::CPUI_INDIRECT && !op.is_indirect_creation()
                            && op.get_in(0).and_then(|input| fd.vbank().get(input)).is_some_and(|source|
                                !source.is_constant() && source.get_size() == 1
                                    && (source.get_space().get_index() != value.get_space().get_index()
                                        || source.get_offset() != value.get_offset()))
                    });
                false
            });
            if invalid { break; }
            if !written {
                if count >= 2 && escaped && affected && state.hints_mut().iter().any(|h|
                    h.sstart == slot && h.size == 1 && h.range_type == RangeType::Fixed
                        && !h.is_type_lock() && compatible(&h.type_))
                {
                    return false;
                }
                break;
            }
            escaped |= open;
            count += 1;
        }
        if count < 2 || !escaped || !state.covers(*start, count) {
            continue;
        }
        planned.push((*start, count, byte));
    }
    for (start, count, byte) in planned {
        if let Ok(array) = types.get_type_array(count, byte) {
            let sstart = sign_extend(space.wrap_offset(start) as intb, bits);
            let end = sstart + count as intb;
            state.hints_mut().retain(|h| h.sstart >= end
                || h.sstart + (h.size as intb) <= sstart);
            state.add_range_pub(start, Some(array), 0, RangeType::Fixed, -1);
        }
    }
    true
}

/// Retain wider indexed extents already produced by the original layout.
pub(crate) fn settle(state: &mut MapState, space: &Rc<AddrSpace>, types: &dyn TypeFactory) {
    let extended = state.take_extended();
    if extended.is_empty() { return; }
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
    fn stackalloc_callee_tail_withdrawal_restarts_with_consumer_coverage() {
        use crate::action::ActionContext;
        use crate::context::ArchContext;
        use crate::dtype::TypeFactoryImpl;
        use crate::p9_emit::coreaction_render::ActionRestructureVarnode;
        use kuna_base::address::{calc_mask, RangeList};
        use kuna_base::space::{addrspace_flags, spacetype, AddrSpaceManager, ConstantSpace,
            IopSpace, SpacebaseSpace, UniqueSpace, VarnodeStorage};

        let build = |withdrawn: bool| {
            let mut manager = AddrSpaceManager::new();
            manager.insert_space(Rc::new(ConstantSpace::new())).unwrap();
            manager.insert_space(Rc::new(UniqueSpace::new(1, 0, false))).unwrap();
            manager.insert_space(Rc::new(IopSpace::new(2))).unwrap();
            let ram = Rc::new(AddrSpace::new(
                spacetype::IPTR_PROCESSOR, "ram", false, 8, 1, 3, addrspace_flags::hasphysical, 1, 1,
            ));
            manager.insert_space(Rc::clone(&ram)).unwrap();
            manager.set_default_code_space(3).unwrap();
            manager.set_default_data_space(3).unwrap();
            let register = Rc::new(AddrSpace::new(
                spacetype::IPTR_PROCESSOR, "register", false, 8, 1, 4, addrspace_flags::hasphysical, 0, 0,
            ));
            manager.insert_space(Rc::clone(&register)).unwrap();
            let stack = Rc::new(SpacebaseSpace::new("stack", 5, 8, &ram, 1, true, false));
            manager.add_spacebase_pointer(&stack, &VarnodeStorage {
                space: Some(Rc::clone(&register)), offset: 0, size: 8,
            }, 8, true).unwrap();
            manager.insert_space(stack).unwrap();
            let stack = Rc::clone(manager.get_stack_space().unwrap());
            let types = Rc::new(TypeFactoryImpl::new());
            types.set_default_alignment_map();
            types.set_max_basetype_size(8);
            let byte = types.get_base(1, type_metatype::TYPE_UNKNOWN).unwrap();
            let mut context = ArchContext::new(manager);
            context.types = Some(types.clone());
            context.stack_alloc = true;
            context.array_extent = LEVEL_ON;
            let mut fd = Funcdata::new("tail", "tail", Rc::new(context),
                Address::new(ram, 0x1000), 0x10000000, 0x40).unwrap();
            let mut model = crate::fspec::ProtoModel::new(fd.get_arch().manage());
            model.build_param_list("standard").unwrap();
            fd.get_func_proto_mut().set_model(Some(Rc::new(model)));
            if withdrawn { fd.withdraw_stack_alloc(); }
            let mut range = RangeList::new();
            range.insert_range(Rc::clone(&stack), (-64_i64) as u64, u64::MAX);
            let params = RangeList::new();
            let start = (-32_i64) as u64;
            let scope = fd.get_scope_local_mut().unwrap();
            scope.reset_local_window(&range, &params, false);
            scope.add_symbol("", types.get_type_array(12, byte).unwrap(),
                &Address::new(Rc::clone(&stack), start), &Address::new_invalid()).unwrap();
            let root = fd.bblocks_ref().root.unwrap();
            let block = fd.bblocks_mut().new_block_basic(root);
            let entry = fd.new_varnode(8, &Address::new(register, 0), None);
            let entry = fd.set_input_varnode(entry).unwrap();
            if !withdrawn {
                fd.vbank_mut().get_mut(entry).unwrap().set_auto_live_hold();
                fd.kuna_stack_scope_holds.push(entry);
            }
            let base_op = fd.new_op(2, fd.get_address().clone());
            fd.op_set_opcode_code(base_op, OpCode::CPUI_INT_ADD);
            fd.op_set_input(base_op, entry, 0).unwrap();
            let offset = fd.new_constant(8, start);
            fd.op_set_input(base_op, offset, 1).unwrap();
            let base = fd.new_unique_out(8, base_op).unwrap();
            fd.op_insert_end(base_op, block);
            let tail_op = fd.new_op(2, fd.get_address().clone());
            fd.op_set_opcode_code(tail_op, OpCode::CPUI_INT_ADD);
            fd.op_set_input(tail_op, base, 0).unwrap();
            let two = fd.new_constant(8, 2);
            fd.op_set_input(tail_op, two, 1).unwrap();
            let tail = fd.new_unique_out(8, tail_op).unwrap();
            fd.op_insert_end(tail_op, block);
            let fill = fd.new_op(3, fd.get_address().clone());
            fd.op_set_opcode_code(fill, OpCode::CPUI_CALL);
            let target = fd.new_constant(8, 0x2000);
            fd.op_set_input(fill, target, 0).unwrap();
            fd.op_set_input(fill, tail, 1).unwrap();
            let four = fd.new_constant(8, 4);
            fd.op_set_input(fill, four, 2).unwrap();
            for index in 0..6 {
                let marker = index >= 2;
                let write = fd.new_op(if marker { 2 } else { 1 }, fd.get_address().clone());
                fd.op_set_opcode_code(write, if marker { OpCode::CPUI_INDIRECT } else { OpCode::CPUI_COPY });
                let input = fd.new_constant(1, if marker { 0 } else { index });
                fd.op_set_input(write, input, 0).unwrap();
                if marker {
                    let effect = fd.new_varnode_iop(fill);
                    fd.op_set_input(write, effect, 1).unwrap();
                    fd.obank_mut().get_mut(write).unwrap().set_flag(crate::op::pcodeop_flags::indirect_creation);
                }
                fd.new_varnode_out(1, &Address::new(Rc::clone(&stack), start.wrapping_add(index)), write).unwrap();
                fd.op_insert_end(write, block);
            }
            fd.op_insert_end(fill, block);
            let consume = fd.new_op(3, fd.get_address().clone());
            fd.op_set_opcode_code(consume, OpCode::CPUI_CALL);
            let target = fd.new_constant(8, 0x3000);
            fd.op_set_input(consume, target, 0).unwrap();
            fd.op_set_input(consume, base, 1).unwrap();
            let six = fd.new_constant(8, 6);
            fd.op_set_input(consume, six, 2).unwrap();
            fd.op_insert_end(consume, block);
            let reserve = fd.new_op(2, fd.get_address().clone());
            fd.op_set_opcode_code(reserve, OpCode::CPUI_INT_SUB);
            let dynamic_base = if withdrawn { base } else { fd.new_unique(8, None) };
            fd.op_set_input(reserve, dynamic_base, 0).unwrap();
            let amount = fd.new_unique(8, None);
            fd.op_set_input(reserve, amount, 1).unwrap();
            let dynamic = fd.new_unique_out(8, reserve).unwrap();
            fd.op_insert_end(reserve, block);
            let sink = fd.new_op(2, fd.get_address().clone());
            fd.op_set_opcode_code(sink, OpCode::CPUI_CALL);
            let target = fd.new_constant(8, 0x4000);
            fd.op_set_input(sink, target, 0).unwrap();
            fd.op_set_input(sink, dynamic, 1).unwrap();
            fd.op_insert_end(sink, block);
            (fd, stack, start & calc_mask(8))
        };
        let (mut fd, _, _) = build(false);
        let mut ctx = ActionContext::new();
        ActionRestructureVarnode::boxed("localrecovery").apply(&mut fd, &mut ctx);
        assert!(ctx.abandon, "side-effect tail must stop the recovery analysis");
        assert!(!fd.stack_alloc_on());
        assert!(fd.stack_alloc_spoiled().is_some());
        fd.clear();
        assert!(!fd.stack_alloc_on());
        assert!(fd.stack_alloc_spoiled().is_some(), "clear lost the outer withdrawal request");
        let (mut restarted, stack, start) = build(true);
        let mut ctx = ActionContext::new();
        ActionRestructureVarnode::boxed("localrecovery").apply(&mut restarted, &mut ctx);
        assert!(!ctx.abandon);
        let scope = restarted.get_scope_local().unwrap();
        let id = scope.scope_id();
        for index in 0..6 {
            let entry = scope.database().find_overlap(id,
                &Address::new(Rc::clone(&stack), start.wrapping_add(index)), 1).unwrap();
            let entry = scope.database().entry(id, entry);
            assert_eq!(entry.get_addr().get_offset(), start);
            assert!(entry.get_size() >= 6, "restarted consumer lost its six-byte storage");
        }
    }

    #[test]
    fn option_levels_parse() {
        assert_eq!(OptionArrayExtent.apply("off").unwrap().0, LEVEL_OFF);
        assert_eq!(OptionArrayExtent.apply("bound").unwrap().0, LEVEL_BOUND);
        assert_eq!(OptionArrayExtent.apply("on").unwrap().0, LEVEL_ON);
        assert_eq!(OptionArrayExtent.apply("").unwrap().0, LEVEL_ON);
        assert!(OptionArrayExtent.apply("full").is_err());
    }

    #[test]
    fn stackalloc_byte_view_preserves_packet_without_padding_or_independent_slots() {
        use kuna_base::address::RangeList;
        use kuna_base::space::{addrspace_flags, spacetype, AddrSpaceManager, ConstantSpace, UniqueSpace};
        use crate::context::ArchContext;
        use crate::dtype::TypeFactoryImpl;
        use crate::varmap::ScopeLocal;

        for control in ["packet", "padding", "old-end", "read", "locked", "gap",
            "width", "outside-alias", "no-view", "indexed", "independent", "frame", "signed", "large-view",
            "propagated", "same-slot", "created", "created-value", "callee-tail"] {
            let mut manager = AddrSpaceManager::new();
            manager.insert_space(Rc::new(ConstantSpace::new())).unwrap();
            manager.insert_space(Rc::new(UniqueSpace::new(1, 0, false))).unwrap();
            let ram = Rc::new(AddrSpace::new(
                spacetype::IPTR_PROCESSOR, "ram", false, 8, 1, 2, addrspace_flags::hasphysical, 1, 1,
            ));
            manager.insert_space(Rc::clone(&ram)).unwrap();
            let stack = Rc::new(AddrSpace::new(
                spacetype::IPTR_PROCESSOR, "stack", false, 8, 1, 3, addrspace_flags::hasphysical, 1, 1,
            ));
            manager.insert_space(Rc::clone(&stack)).unwrap();
            let arch = Rc::new(ArchContext::new(manager));
            let mut fd = Funcdata::new("packet", "packet", arch, Address::new(ram, 0x1000), 0x10000000, 0x40).unwrap();
            let root = fd.bblocks_ref().root.unwrap();
            let block = fd.bblocks_mut().new_block_basic(root);
            let types = TypeFactoryImpl::new();
            types.set_default_alignment_map();
            types.set_max_basetype_size(8);
            let byte = Rc::new(Datatype::new_with_align(1, 1, type_metatype::TYPE_UNKNOWN));
            let start = 0x10;
            let mut range = RangeList::new();
            range.insert_range(Rc::clone(&stack), 0, if control == "frame" { start + 2 } else { 0x3f });
            let params = RangeList::new();
            let mut state = MapState::new(Rc::clone(&stack), &range, &params, Rc::clone(&byte));
            let mut previous = vec![(start, types.get_type_array(
                if control == "large-view" { 512 } else if matches!(control, "old-end" | "outside-alias") { 3 } else { 12 }, Rc::clone(&byte),
            ).unwrap(), false)];
            if control == "no-view" { previous.clear(); }
            if control == "independent" {
                previous.push((start + 3, Rc::clone(&byte), false));
            }
            for index in 0..6 {
                if (control == "padding" && index >= 2) || (control == "gap" && index == 3)
                    || (control == "width" && index > 3) { continue; }
                let width = if control == "width" && index == 3 { 4 } else { 1 };
                let ty = if control == "signed" && index == 3 {
                    Rc::new(Datatype::new_with_align(1, 1, type_metatype::TYPE_INT))
                } else if width != 1 {
                    Rc::new(Datatype::new_with_align(width, width, type_metatype::TYPE_UINT))
                } else { Rc::clone(&byte) };
                let runtime = matches!(index, 2 | 4);
                let write = fd.new_op(if runtime { 2 } else { 1 }, fd.get_address().clone());
                fd.op_set_opcode_code(write, if runtime { OpCode::CPUI_SUBPIECE } else { OpCode::CPUI_COPY });
                let input = if runtime {
                    fd.new_unique(4, None)
                } else { fd.new_constant(width, index) };
                fd.op_set_input(write, input, 0).unwrap();
                if runtime {
                    let zero = fd.new_constant(4, 0);
                    fd.op_set_input(write, zero, 1).unwrap();
                }
                let address = Address::new(Rc::clone(&stack), start + index);
                let output = if (runtime && matches!(control, "propagated" | "same-slot" | "created" | "created-value"))
                    || (control == "callee-tail" && index >= 2) {
                    let source = fd.new_unique_out(width, write).unwrap();
                    fd.op_insert_end(write, block);
                    let marker = fd.new_op(2, fd.get_address().clone());
                    fd.op_set_opcode_code(marker, OpCode::CPUI_INDIRECT);
                    let source = if control == "same-slot" {
                        fd.new_varnode(width, &address, None)
                    } else if matches!(control, "created" | "callee-tail") { fd.new_constant(width, 0) } else { source };
                    fd.op_set_input(marker, source, 0).unwrap();
                    let effect = fd.new_constant(8, 0);
                    fd.op_set_input(marker, effect, 1).unwrap();
                    if matches!(control, "created" | "created-value" | "callee-tail") {
                        fd.obank_mut().get_mut(marker).unwrap().set_flag(crate::op::pcodeop_flags::indirect_creation);
                    }
                    let output = fd.new_varnode_out(width, &address, marker).unwrap();
                    fd.op_insert_end(marker, block);
                    output
                } else {
                    let output = fd.new_varnode_out(width, &address, write).unwrap();
                    fd.op_insert_end(write, block);
                    output
                };
                if control == "read" && index == 3 {
                    let read = fd.new_op(2, fd.get_address().clone());
                    fd.op_set_opcode_code(read, OpCode::CPUI_INT_ADD);
                    fd.op_set_input(read, output, 0).unwrap();
                    let one = fd.new_constant(1, 1);
                    fd.op_set_input(read, one, 1).unwrap();
                    fd.new_unique_out(1, read).unwrap();
                    fd.op_insert_end(read, block);
                }
                state.add_range_pub(start + index, Some(ty),
                    if control == "locked" && index == 3 { crate::varmap::TYPELOCK } else { 0 },
                    RangeType::Fixed, -1);
            }
            let alias = start + if control == "outside-alias" { 4 } else { 1 };
            state.add_range_pub(alias, Some(Rc::clone(&byte)), 0, RangeType::Open,
                if control == "indexed" { 3 } else { -1 });
            if control == "callee-tail" {
                state.add_range_pub(start, Some(Rc::clone(&byte)), 0, RangeType::Open, -1);
            }
            state.add_range_pub(start + 8,
                Some(Rc::new(Datatype::new_with_align(4, 4, type_metatype::TYPE_UINT))),
                0, RangeType::Fixed, -1);
            let original = state.hints_mut().clone();
            let safe = preserve_stackalloc_bytes(&fd, &mut state, &stack, &previous, &types);
            let refused = matches!(control, "same-slot" | "created" | "created-value" | "callee-tail");
            assert_eq!(safe, !refused, "{control}: live side-effect tail decision");
            let expected = match control {
                "packet" | "propagated" => Some(6),
                "padding" => Some(2),
                "outside-alias" | "no-view" | "indexed" | "large-view"
                    | "same-slot" | "created" | "created-value" | "callee-tail" => None,
                _ => Some(3),
            };
            let retained = state.hints_mut().iter().find(|h| h.start == start
                && h.range_type == RangeType::Fixed && h.type_.get_metatype() == type_metatype::TYPE_ARRAY)
                .map(|h| h.size);
            assert_eq!(retained, expected, "{control}");
            let end = start + expected.unwrap_or(0) as uintb;
            for hint in original.iter().filter(|h| h.start >= end) {
                assert!(state.hints_mut().iter().any(|h| h.start == hint.start
                    && h.size == hint.size && h.range_type == hint.range_type
                    && h.highind == hint.highind && h.flags == hint.flags
                    && Rc::ptr_eq(&h.type_, &hint.type_)),
                    "{control}: independent hint at {} changed", hint.start);
            }
            if matches!(control, "packet" | "propagated") {
                let mut scope = ScopeLocal::new(1, Rc::clone(&stack), "packet", 4).unwrap();
                scope.reset_local_window(&range, &params, false);
                scope.restructure(&mut state, &types).unwrap();
                let id = scope.scope_id();
                for index in 0..6 {
                    let entry = scope.database().find_overlap(id, &Address::new(Rc::clone(&stack), start + index), 1).unwrap();
                    let entry = scope.database().entry(id, entry);
                    assert_eq!(entry.get_addr().get_offset(), start);
                    assert_eq!(entry.get_size(), 6, "{control}: consumer requires six contiguous bytes");
                }
                assert!(scope.database().find_overlap(id, &Address::new(Rc::clone(&stack), start + 6), 1).is_none());
            }
        }
    }

}
