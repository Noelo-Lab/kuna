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
//! from the lowest to the end of the highest, and a walk from such a choice has
//! one unbounded piece per address. A choice input that does not come from the
//! stack base (a global) is not a stack address, and copies and INDIRECTs pass
//! the pointer through. The hints inside a reach, widened over any hint that
//! crosses either edge (an open hint by its indexed elements), are replaced by
//! one open byte array hint whose index evidence runs to the end of the reach.
//! Its element is the most specific one-byte integer type a byte access inside
//! the reach carries (an unsigned one when a byte is read zero-extended), or the
//! unknown byte. A reach holding a type-locked hint keeps its hints.
//!
//! A reach holding a float hint keeps its hints too, and its stores' bases stop
//! absorbing: a float read of a byte array would print as an integer piece.
//!
//! A frame with a LOAD or STORE whose pointer comes from the stack base but
//! does not resolve (`(&v1 | 4) + i`, a walk with a variable step, a choice
//! between more than eight addresses) is not guarded at all: that access may
//! write or read any slot near a reach, so no layout of the frame is known to
//! be right (`p3_dataflow/kuna_stackstoreguard.rs (frame_unresolved)`). When a
//! layout pass finds one in a frame heritage already guarded, the analysis
//! stops and the drive analyzes a freshly built copy of the function without
//! the guard. The partial copy jump-table recovery analyzes is never analyzed
//! again, so there such a pass keeps the upstream layout unless an earlier pass
//! laid out a reach (`Funcdata::store_reach_committed`): a smaller local would
//! strand the pointers the earlier pass resolved against the larger one
//! (`&v19[0x20]` into a `char v19[32]`).
//!
//! Whether or not the indices are bounded, the open range at a guarded store's
//! base then absorbs every hint that starts inside a slot it absorbed
//! (`MapState::set_absorbing_bases`), so a word read inside a constant slot the
//! range swallowed stays a piece of the same local.
//!
//! The guard is worse than none when the final layout maps a slot it keeps
//! (the bytes of a guard INDIRECT whose value is read) as more than one local,
//! maps any slot a guard INDIRECT names inside a guarded store's reach (its
//! bounded reach, or everything at or above an unbounded piece's base),
//! whether read or not, outside the local holding the store's base, holds the
//! base in a local that is not an array of bytes or of 2-, 4- or 8-byte
//! integers or unknowns (a bounded reach may also lie wholly in a float
//! local), or splits or reads as an integer a float local a guarded piece lies
//! in: the drive then analyzes a freshly built copy of the function without
//! the guard (`withdraw_spoiled_guard`), which prints what `stackstoreguard
//! off` prints.

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

/// Widest bounded reach, in bytes; a wider index is unbounded.
const MAX_REACH: intb = 0x100;

/// Most stack addresses one pointer may choose between.
const MAX_PIECES: usize = 8;

/// What the latest layout pass asks of the final layout.
#[derive(Clone, Default)]
pub(crate) struct ReachChecks {
    /// `(base, end)`: the bytes a guarded store may write from `base`, to the
    /// end of its bounded reach or, for an unbounded piece, the top of the frame.
    reaches: Vec<(intb, intb)>,
    /// The guarded store pieces in a float reach.
    floats: Vec<(intb, intb)>,
}

/// Coalesce each guarded byte store's bounded reach into one open array hint,
/// and let the open range at every guarded store's base absorb what starts
/// inside a slot it absorbed. Returns what the final layout must satisfy for
/// the guard to stand (`withdraw_spoiled_guard`).
pub(crate) fn prepare_hints(
    fd: &Funcdata,
    state: &mut MapState,
    space: &Rc<AddrSpace>,
    types: &dyn TypeFactory,
) -> ReachChecks {
    let mut checks = ReachChecks::default();
    if !fd.stack_store_guard() {
        return checks;
    }
    let Some(sb) = fd.find_spacebase_input(space) else {
        return checks;
    };
    if !has_byte_store(fd) {
        return checks;
    }
    let guarded: Vec<OpId> = guard_effects(fd, space).stores.into_iter().collect();
    if guarded.is_empty() {
        return checks;
    }
    if frame_unresolved(fd, space, &guarded) {
        if !fd.is_jumptable_recovery_on() {
            fd.spoil_stack_store_guard();
            return checks;
        }
        if !fd.store_reach_committed() {
            return checks;
        }
    }
    let resolved: Vec<Vec<(intb, Option<intb>)>> = guarded
        .iter()
        .filter_map(|&store| store_pieces(fd, store, sb, space))
        .collect();
    let reaches: Vec<(Vec<intb>, Option<(intb, intb)>)> = resolved
        .iter()
        .filter_map(|pieces| store_reach(pieces))
        .collect();
    if reaches.is_empty() {
        return checks;
    }
    fd.commit_store_reach();
    let mut bounded: Vec<(intb, intb)> = reaches.iter().filter_map(|&(_, span)| span).collect();
    bounded.sort_unstable();
    let mut merged: Vec<(intb, intb)> = Vec::new();
    for (lo, hi) in bounded {
        match merged.last_mut() {
            Some(last) if lo <= last.1 => last.1 = last.1.max(hi),
            _ => merged.push((lo, hi)),
        }
    }
    let mut bases: Vec<intb> = reaches
        .iter()
        .flat_map(|(b, _)| b.iter().copied())
        .collect();
    let mut floats: Vec<(intb, intb)> = Vec::new();
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
    for (bases, span) in &reaches {
        match span {
            Some(span) => checks.reaches.push(*span),
            None => checks.reaches.extend(bases.iter().map(|&b| (b, intb::MAX))),
        }
    }
    checks.reaches.sort_unstable();
    checks.reaches.dedup();
    checks.floats = resolved
        .iter()
        .filter(|pieces| store_reach(pieces).is_some())
        .flatten()
        .filter_map(|&(lo, hi)| Some((lo, hi?)))
        .filter(|&(lo, hi)| floats.iter().any(|&(flo, fhi)| lo < fhi && flo < hi))
        .collect();
    checks.floats.sort_unstable();
    checks.floats.dedup();
    checks
}

/// If the final layout spoils what the guard needs, mark the function to be
/// analyzed again from scratch without the guard. The guard stands
/// only when every stack access in the frame resolved, every slot a guard
/// INDIRECT still keeps (and whose value is read) is one local, every slot a
/// guard INDIRECT names inside a guarded store's reach, read or not, lies in
/// the local holding the store's base (the C writes through that local and no
/// other), that local can hold the reach (`holds_reach`), and every guarded
/// store piece in a float reach is one local that is read only as a float: an
/// integer read of a float local prints as a cast, which converts its value.
pub(crate) fn withdraw_spoiled_guard(fd: &Funcdata) -> bool {
    if !fd.stack_store_guard() {
        return false;
    }
    if !has_byte_store(fd) {
        return false;
    }
    let Some(sl) = fd.get_scope_local() else {
        return false;
    };
    let space = Rc::clone(sl.get_space_id());
    let checks = fd.store_reach_checks();
    let effects = guard_effects(fd, &space);
    if checks.floats.is_empty() && checks.reaches.is_empty() && effects.read.is_empty() {
        return false;
    }
    let bits = space.get_addr_size() as int4 * 8 - 1;
    let symbols: Vec<(intb, intb, Rc<Datatype>)> = sl
        .database()
        .scope_space_local_var_specs(sl.scope_id(), space.get_index() as usize)
        .into_iter()
        .map(|(_, ct, addr, _)| {
            let off = sign_extend(addr.get_offset() as intb, bits);
            (off, off + ct.get_size() as intb, ct)
        })
        .collect();
    let locals = |lo: intb, hi: intb| -> Vec<(intb, intb)> {
        symbols
            .iter()
            .filter(|&&(s, e, _)| s < hi && lo < e)
            .map(|&(s, e, _)| (s, e))
            .collect()
    };
    let spoiled = effects
        .read
        .iter()
        .any(|&(lo, hi)| locals(lo, hi).len() > 1)
        || checks.reaches.iter().any(|&(base, end)| {
            let local = symbols.iter().find(|&&(s, e, _)| s <= base && base < e);
            local.is_some_and(|(s, e, ct)| !holds_reach(ct, *s, *e, base, end))
                || effects.all.iter().any(|&(lo, hi)| {
                    lo < end && base < hi && !local.is_some_and(|&(s, e, _)| s <= lo && hi <= e)
                })
        })
        || checks
            .floats
            .iter()
            .any(|&(lo, hi)| match locals(lo, hi).as_slice() {
                [(s, e)] => !read_only_as_float(fd, &space, *s, *e),
                _ => true,
            });
    if spoiled {
        fd.spoil_stack_store_guard();
    }
    spoiled
}

/// Can the local of type `ct` at `[s, e)` print the writes of a store reaching
/// `[base, end)` (`end` the top of the frame for an unbounded one)? An array of
/// bytes or of 2-, 4- or 8-byte integers or unknowns is laid out to the next
/// local and written through a byte pointer (`v1[i]`, `((char *)v1)[i]`); a
/// float local holding all of a bounded reach is written the same way and read
/// by the float checks. Any other local (a scalar, an `undefined16` array) may
/// end before the bytes the store writes, or print an index the C cannot
/// express (`v1[0][i]`).
fn holds_reach(ct: &Datatype, s: intb, e: intb, base: intb, end: intb) -> bool {
    let bounded = end != intb::MAX;
    if bounded && !(s <= base && end <= e) {
        return false;
    }
    let elem = ct.get_array_base();
    match elem.as_deref() {
        Some(el) if el.get_size() == 1 => true,
        Some(el)
            if matches!(el.get_size(), 2 | 4 | 8)
                && matches!(
                    el.get_metatype(),
                    type_metatype::TYPE_INT
                        | type_metatype::TYPE_UINT
                        | type_metatype::TYPE_UNKNOWN
                ) =>
        {
            true
        }
        el => bounded && el.unwrap_or(ct).get_metatype() == type_metatype::TYPE_FLOAT,
    }
}

/// Does every op that reads stack bytes in `[lo, hi)` print them as their
/// bits? A float op reads a float; a COPY, INDIRECT, MULTIEQUAL or PIECE into
/// `[lo, hi)` moves or assembles them; a SUBPIECE above the low end prints as a
/// piece (`v1._6_2_`). Any other read of a variable declared float prints as a
/// cast, which converts its value (`(unsigned short)v1` of a `double v1`).
fn read_only_as_float(fd: &Funcdata, space: &Rc<AddrSpace>, lo: intb, hi: intb) -> bool {
    let bits = space.get_addr_size() as int4 * 8 - 1;
    let inside = |vn: VarnodeId| {
        fd.vbank().get(vn).is_some_and(|v| {
            if v.get_space().get_index() != space.get_index() {
                return false;
            }
            let off = sign_extend(v.get_addr().get_offset() as intb, bits);
            off < hi && lo < off + v.get_size() as intb
        })
    };
    fd.obank().iter_alive().all(|id| {
        let Some(op) = fd.obank().get(id) else {
            return true;
        };
        let read: Vec<VarnodeId> = (0..op.num_input())
            .filter_map(|k| op.get_in(k))
            .filter(|&v| inside(v))
            .collect();
        if read.is_empty() {
            return true;
        }
        match op.code() {
            OpCode::CPUI_INDIRECT
            | OpCode::CPUI_MULTIEQUAL
            | OpCode::CPUI_COPY
            | OpCode::CPUI_PIECE
                if op.get_out().is_some_and(inside) =>
            {
                true
            }
            OpCode::CPUI_SUBPIECE
                if op
                    .get_in(1)
                    .and_then(|c| fd.vbank().get(c))
                    .is_some_and(|c| c.get_offset() != 0) =>
            {
                true
            }
            code if code != OpCode::CPUI_FLOAT_INT2FLOAT
                && (OpCode::CPUI_FLOAT_EQUAL as u32..=OpCode::CPUI_FLOAT_ROUND as u32)
                    .contains(&(code as u32)) =>
            {
                true
            }
            _ => !read.iter().any(|&v| declared_float(fd, v)),
        }
    })
}

/// Is `vn`, or another member of its variable, typed as a float?
fn declared_float(fd: &Funcdata, vn: VarnodeId) -> bool {
    let float = |v: VarnodeId| {
        fd.vbank()
            .get(v)
            .is_some_and(|v| v.get_type().get_metatype() == type_metatype::TYPE_FLOAT)
    };
    if float(vn) {
        return true;
    }
    let Some(high) = fd
        .vbank()
        .get(vn)
        .and_then(|v| v.get_high())
        .and_then(|h| fd.high_bank().get(h))
    else {
        return false;
    };
    (0..high.num_instances()).any(|i| float(high.get_instance(i)))
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

/// Does the function store a single byte anywhere?
fn has_byte_store(fd: &Funcdata) -> bool {
    fd.obank().iter_code(OpCode::CPUI_STORE).any(|id| {
        fd.obank()
            .get(id)
            .and_then(|op| op.get_in(2))
            .and_then(|v| fd.vbank().get(v))
            .is_some_and(|v| v.get_size() == 1)
    })
}

/// Does the pointer of one of the byte `stores`, or of another LOAD or STORE
/// in the frame, come from the stack base along a path `pointer_pieces` cannot
/// resolve, or choose between stack addresses too far apart to fold?
pub(crate) fn frame_unresolved(fd: &Funcdata, space: &Rc<AddrSpace>, stores: &[OpId]) -> bool {
    let Some(sb) = fd.find_spacebase_input(space) else {
        return false;
    };
    stores.iter().any(|&store| {
        store_pieces(fd, store, sb, space)
            .is_none_or(|pieces| pieces.len() > 1 && store_reach(&pieces).is_none())
    }) || has_unresolved_frame_access(fd, sb)
}

/// The slots a guard keeps byte stores' effects on.
#[derive(Default)]
struct GuardEffects {
    /// The byte STOREs a guard INDIRECT names as its effect.
    stores: BTreeSet<OpId>,
    /// The signed stack ranges `[lo, hi)` of those INDIRECTs whose value is read.
    read: Vec<(intb, intb)>,
    /// The signed stack ranges of all of them.
    all: Vec<(intb, intb)>,
}

/// The guard INDIRECTs on `space` whose effect is a byte STORE.
fn guard_effects(fd: &Funcdata, space: &Rc<AddrSpace>) -> GuardEffects {
    let bits = space.get_addr_size() as int4 * 8 - 1;
    let mut effects = GuardEffects::default();
    for id in fd.obank().iter_alive() {
        let Some(op) = fd
            .obank()
            .get(id)
            .filter(|o| o.code() == OpCode::CPUI_INDIRECT)
        else {
            continue;
        };
        let Some(out_id) = op.get_out() else {
            continue;
        };
        let Some(out) = fd
            .vbank()
            .get(out_id)
            .filter(|o| o.get_space().get_index() == space.get_index())
        else {
            continue;
        };
        let Some(iop) = op
            .get_in(1)
            .and_then(|v| fd.vbank().get(v))
            .filter(|v| v.get_space().get_type() == spacetype::IPTR_IOP)
        else {
            continue;
        };
        let store = crate::funcdata_varnode::op_iop_decode(iop.get_addr().get_offset());
        let byte_store = fd
            .obank()
            .get(store)
            .filter(|s| s.code() == OpCode::CPUI_STORE && !s.is_dead())
            .and_then(|s| s.get_in(2))
            .and_then(|v| fd.vbank().get(v))
            .is_some_and(|v| v.get_size() == 1);
        if byte_store {
            effects.stores.insert(store);
            let lo = sign_extend(out.get_addr().get_offset() as intb, bits);
            let range = (lo, lo + out.get_size() as intb);
            if is_read(fd, out_id) {
                effects.read.push(range);
            }
            effects.all.push(range);
        }
    }
    effects
}

/// Does an op other than an INDIRECT or MULTIEQUAL read `vn`, or the value of
/// one that carries it on? Undecided past 256 values, which counts as yes.
pub(crate) fn is_read(fd: &Funcdata, vn: VarnodeId) -> bool {
    let mut seen = BTreeSet::new();
    let mut work = vec![vn];
    while let Some(v) = work.pop() {
        if !seen.insert(v) {
            continue;
        }
        if seen.len() > 256 {
            return true;
        }
        let Some(value) = fd.vbank().get(v) else {
            continue;
        };
        for use_op in value.descend_iter() {
            let Some(op) = fd.obank().get(use_op) else {
                continue;
            };
            if !matches!(op.code(), OpCode::CPUI_INDIRECT | OpCode::CPUI_MULTIEQUAL) {
                return true;
            }
            work.extend(op.get_out());
        }
    }
    false
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

/// The bases of an indexed store's open ranges and, when its indices are
/// bounded, the span of the bytes it may write. A store to one fixed byte has
/// none.
fn store_reach(pieces: &[(intb, Option<intb>)]) -> Option<(Vec<intb>, Option<(intb, intb)>)> {
    if let [(lo, hi)] = pieces {
        return (*hi != Some(lo + 1)).then(|| (vec![*lo], hi.map(|hi| (*lo, hi))));
    }
    let lo = pieces.iter().map(|&(lo, _)| lo).min()?;
    match pieces.iter().map(|&(_, hi)| hi).collect::<Option<Vec<_>>>() {
        Some(his) => {
            let hi = his.into_iter().max()?;
            (hi - lo <= MAX_REACH).then(|| (vec![lo], Some((lo, hi))))
        }
        None => Some((pieces.iter().map(|&(lo, _)| lo).collect(), None)),
    }
}

/// `vn` as the stack base plus a constant plus a non-negative extra, which is
/// `None` when an index's known-bits mask does not bound it. The extra starts
/// at the address the pointer names, since the C prints the access from there.
/// A phi of different stack addresses gives one such piece per address.
pub(crate) fn pointer_pieces(
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
        OpCode::CPUI_COPY | OpCode::CPUI_CAST | OpCode::CPUI_INDIRECT => {
            pointer_pieces(fd, op.get_in(0)?, sb, depth + 1)
        }
        OpCode::CPUI_MULTIEQUAL => {
            let mut walk = false;
            let mut pieces: Vec<(uintb, Option<intb>)> = Vec::new();
            for k in 0..op.num_input() {
                let input = op.get_in(k)?;
                if steps_from(fd, input, vn) {
                    walk = true;
                    continue;
                }
                let Some(found) = pointer_pieces(fd, input, sb, depth + 1) else {
                    if comes_from(fd, input, sb) {
                        return None;
                    }
                    continue;
                };
                for piece in found {
                    if !pieces.contains(&piece) {
                        pieces.push(piece);
                    }
                }
            }
            let &(first, _) = pieces.first()?;
            if pieces.iter().all(|&(off, _)| off == first) {
                return Some(vec![(first, None)]);
            }
            if pieces.len() > MAX_PIECES {
                return None;
            }
            if walk {
                for piece in &mut pieces {
                    piece.1 = None;
                }
                pieces.sort_unstable();
                pieces.dedup();
            }
            Some(pieces)
        }
        OpCode::CPUI_PTRSUB => {
            let c = fd
                .vbank()
                .get(op.get_in(1)?)
                .filter(|c| c.is_constant())?
                .get_offset();
            let pieces = pointer_pieces(fd, op.get_in(0)?, sb, depth + 1)?;
            Some(
                pieces
                    .into_iter()
                    .map(|(off, extra)| (off.wrapping_add(c), extra))
                    .collect(),
            )
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
                return Some(
                    pieces
                        .into_iter()
                        .map(|(off, extra)| (off.wrapping_add(c), extra))
                        .collect(),
                );
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
    (0..MAX_REACH)
        .contains(&c)
        .then_some((op.get_in(0)?, c as uintb))
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
        .get(moved_from(fd, vn))
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
        && op.get_in(0).is_some_and(|base| moved_from(fd, base) == phi)
        && constant(1)
}

/// The value `vn` copies, through COPYs and the INDIRECTs a guard or a call
/// puts on a pointer kept in memory.
fn moved_from(fd: &Funcdata, vn: VarnodeId) -> VarnodeId {
    let mut cur = vn;
    for _ in 0..12 {
        let next = fd
            .vbank()
            .get(cur)
            .and_then(|v| v.get_def())
            .and_then(|d| fd.obank().get(d))
            .filter(|op| matches!(op.code(), OpCode::CPUI_COPY | OpCode::CPUI_INDIRECT))
            .and_then(|op| op.get_in(0));
        match next {
            Some(n) => cur = n,
            None => break,
        }
    }
    cur
}
