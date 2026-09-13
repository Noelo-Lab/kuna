//! (kuna) `endptrbound` -- a pointer walk over a stack buffer ends at the buffer.
//!
//! A loop that walks a stack buffer with a pointer usually stops on the address
//! one past its last element:
//!
//! ```text
//!   lea  rsi,[rbp+0x27]      ; p = buf
//! loop:
//!   movzx edx,byte ptr [rsi] ; *p
//!   ...
//!   inc  rsi
//!   lea  rax,[rbp+0x2f]      ; buf + 8
//!   cmp  rsi,rax
//!   jne  loop
//! ```
//!
//! That end address is a plain `sp + 0x2f` to the frame model, which binds a
//! stack address to whatever symbol lives there -- the neighbouring local -- so
//! the bound renders as the neighbour and the trip count is not recoverable from
//! the C.  The comparison also pulls the neighbour's pointer type into the
//! walking pointer, the walked bytes are re-typed after it, and the buffer splits
//! into unrelated scalars.
//!
//! With the option on, restructuring recognizes the walk: a `MULTIEQUAL`
//! induction pointer that starts at stack offset `start` and advances by a
//! constant `step`, is dereferenced with an access of exactly `step` bytes, and
//! is compared (`==`, `!=`, or `p < end`) against a stack address `end` a whole
//! number of steps beyond `start`.  Two things follow:
//!
//! - The frame hints inside `[start, end)` are replaced by one hint for an array
//!   of `step`-byte elements, so the buffer is one object.  The walk is left
//!   alone when a hint straddles either edge, is type-locked, or carries a
//!   typed non-constant value -- a separately used variable inside the range.
//! - Once the symbol at `start` is an array covering exactly `[start, end)`, the
//!   end address is rebuilt as `&buf + (end - start)`, which renders `&buf[n]`.
//!   The rewrite happens at the address's definition when nothing but the
//!   comparison consumes it (through copies, casts and phi plumbing), and in
//!   front of the comparison otherwise, so an address that also means the
//!   neighbour elsewhere keeps rendering as the neighbour there.
//!
//! The value compared is unchanged; only the base it is expressed on moves.

use std::rc::Rc;

use kuna_base::address::{calc_mask, sign_extend, Address};
use kuna_base::error::KunaResult;
use kuna_base::marshal::ElementId;
use kuna_base::space::AddrSpace;
use kuna_base::types::{int4, uintb};
use kuna_num::opcodes::OpCode;

use crate::context::{OpId, VarnodeId};
use crate::dtype::{type_metatype, Datatype, TypeFactory};
use crate::funcdata::Funcdata;
use crate::p0_knowledge::options::on_or_off;
use crate::varmap::{MapState, RangeHint, RangeType, COPY_CONSTANT};

/// Marshaling element `<endptrbound>` (kuna 4000+ range; 4165 was the previous
/// high-water mark).
pub const ELEM_ENDPTRBOUND: ElementId = ElementId::new("endptrbound", 4166);

/// (kuna) Bind a pointer walk's end address to the walked buffer: `endptrbound on|off`.
pub struct OptionEndPtrBound;

impl OptionEndPtrBound {
    /// The option name.
    pub const NAME: &'static str = "endptrbound";

    /// Resolve the flag and its confirmation message; the caller writes it into
    /// `Architecture::end_ptr_bound`.
    pub fn apply(&self, p1: &str) -> KunaResult<(bool, String)> {
        let val = on_or_off(p1)?;
        let prop = if val { "on" } else { "off" };
        Ok((val, format!("Pointer-walk end bound turned {prop}")))
    }
}

const WALK_DEPTH: u32 = 8;
const PHI_BUDGET: u32 = 32;
const MAX_ELEMENTS: i64 = 0x10000;

/// A loop pointer that starts at stack offset `start` and advances by `step`,
/// compared by `cmp` (bound in input `slot`) against the address `end`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PointerWalk {
    pub cmp: OpId,
    pub slot: int4,
    pub start: i64,
    pub end: i64,
    pub step: i64,
}

fn def_code(fd: &Funcdata, vn: VarnodeId) -> Option<(OpId, OpCode)> {
    let op = fd.vbank().get(vn)?.get_def()?;
    Some((op, fd.obank().get(op)?.code()))
}

fn input(fd: &Funcdata, op: OpId, slot: int4) -> Option<VarnodeId> {
    fd.obank().get(op)?.get_in(slot)
}

fn constant(fd: &Funcdata, vn: VarnodeId) -> Option<i64> {
    let v = fd.vbank().get(vn)?;
    if !v.is_constant() {
        return None;
    }
    Some(sign_extend(v.get_offset() as i64, v.get_size() * 8 - 1))
}

fn is_plumbing(code: OpCode) -> bool {
    matches!(code, OpCode::CPUI_CAST | OpCode::CPUI_COPY | OpCode::CPUI_INDIRECT)
}

fn strip(fd: &Funcdata, mut vn: VarnodeId) -> VarnodeId {
    while let Some((op, code)) = def_code(fd, vn) {
        if !is_plumbing(code) {
            break;
        }
        match input(fd, op, 0) {
            Some(i) => vn = i,
            None => break,
        }
    }
    vn
}

/// The address-forming op behind `vn`, looking through plumbing and through a
/// loop-invariant `MULTIEQUAL` whose inputs all lead back to it or to itself.
fn address_def(fd: &Funcdata, vn: VarnodeId, budget: &mut u32) -> Option<OpId> {
    let vn = strip(fd, vn);
    let (op, code) = def_code(fd, vn)?;
    if code != OpCode::CPUI_MULTIEQUAL {
        return Some(op);
    }
    *budget = budget.checked_sub(1)?;
    let n = fd.obank().get(op)?.num_input();
    let mut root = None;
    for i in 0..n {
        let s = strip(fd, input(fd, op, i)?);
        if s == vn {
            continue;
        }
        let r = address_def(fd, s, budget)?;
        if root.is_some_and(|prev| prev != r) {
            return None;
        }
        root = Some(r);
    }
    root
}

/// The stack offset `vn` holds when it is the stack pointer plus constants.
fn stack_offset(fd: &Funcdata, vn: VarnodeId, sp: VarnodeId, depth: u32) -> Option<i64> {
    if strip(fd, vn) == sp {
        return Some(0);
    }
    if depth == 0 {
        return None;
    }
    let op = address_def(fd, vn, &mut PHI_BUDGET.clone())?;
    let code = fd.obank().get(op)?.code();
    match code {
        OpCode::CPUI_INT_ADD | OpCode::CPUI_PTRSUB => {
            let (a, b) = (input(fd, op, 0)?, input(fd, op, 1)?);
            let (base, c) = match (constant(fd, a), constant(fd, b)) {
                (None, Some(c)) => (a, c),
                (Some(c), None) if code == OpCode::CPUI_INT_ADD => (b, c),
                _ => return None,
            };
            Some(stack_offset(fd, base, sp, depth - 1)?.wrapping_add(c))
        }
        OpCode::CPUI_PTRADD => {
            let i = constant(fd, input(fd, op, 1)?)?;
            let m = constant(fd, input(fd, op, 2)?)?;
            let base = stack_offset(fd, input(fd, op, 0)?, sp, depth - 1)?;
            Some(base.wrapping_add(i.wrapping_mul(m)))
        }
        _ => None,
    }
}

/// The constant step when `vn` computes `phi_out + step`.
fn increment_of(fd: &Funcdata, vn: VarnodeId, phi_out: VarnodeId) -> Option<i64> {
    let (op, code) = def_code(fd, vn)?;
    match code {
        OpCode::CPUI_INT_ADD => {
            let (a, b) = (input(fd, op, 0)?, input(fd, op, 1)?);
            if strip(fd, a) == phi_out {
                constant(fd, b)
            } else if strip(fd, b) == phi_out {
                constant(fd, a)
            } else {
                None
            }
        }
        OpCode::CPUI_PTRADD if strip(fd, input(fd, op, 0)?) == phi_out => {
            let i = constant(fd, input(fd, op, 1)?)?;
            let m = constant(fd, input(fd, op, 2)?)?;
            Some(i.wrapping_mul(m))
        }
        _ => None,
    }
}

/// `(start, step, step_out)` for `p = MULTIEQUAL(sp + start, p + step, ...)`.
fn induction(fd: &Funcdata, phi: OpId, sp: VarnodeId) -> Option<(i64, i64, VarnodeId)> {
    let o = fd.obank().get(phi)?;
    let phi_out = o.get_out()?;
    let n = o.num_input();
    let (mut start, mut step, mut step_out) = (None, None, None);
    for i in 0..n {
        let s = strip(fd, input(fd, phi, i)?);
        if s == phi_out {
            return None;
        }
        if let Some(k) = increment_of(fd, s, phi_out) {
            if step.is_some_and(|prev| prev != k) {
                return None;
            }
            step = Some(k);
            step_out = Some(s);
        } else {
            let a = stack_offset(fd, s, sp, WALK_DEPTH)?;
            if start.is_some_and(|prev| prev != a) {
                return None;
            }
            start = Some(a);
        }
    }
    let step = step.filter(|k| *k > 0)?;
    Some((start?, step, step_out?))
}

/// `vn` and every value that only re-labels it (casts, copies, indirect effects).
fn relabels(fd: &Funcdata, vn: VarnodeId) -> Vec<VarnodeId> {
    let mut out = Vec::new();
    let mut work = vec![vn];
    while let Some(v) = work.pop() {
        if out.contains(&v) {
            continue;
        }
        out.push(v);
        let Some(var) = fd.vbank().get(v) else { continue };
        for d in var.descend_iter() {
            let Some(o) = fd.obank().get(d) else { continue };
            if is_plumbing(o.code()) && o.get_in(0) == Some(v) {
                if let Some(next) = o.get_out() {
                    work.push(next);
                }
            }
        }
    }
    out
}

/// The walks `restructure_varnode` acts on: none unless the option is on.
pub fn gather_walks(fd: &Funcdata, space: &Rc<AddrSpace>) -> Vec<PointerWalk> {
    if !fd.get_arch().end_ptr_bound {
        return Vec::new();
    }
    match fd.find_spacebase_input(space) {
        Some(sp) => find_pointer_walks(fd, sp),
        None => Vec::new(),
    }
}

/// Every pointer walk over the stack frame addressed through `sp`.
pub fn find_pointer_walks(fd: &Funcdata, sp: VarnodeId) -> Vec<PointerWalk> {
    let mut walks: Vec<PointerWalk> = Vec::new();
    let mut seen: Vec<OpId> = Vec::new();
    let accesses: Vec<OpId> = fd
        .obank()
        .iter_code(OpCode::CPUI_LOAD)
        .chain(fd.obank().iter_code(OpCode::CPUI_STORE))
        .collect();
    for op in accesses {
        let Some(o) = fd.obank().get(op) else { continue };
        if o.is_dead() {
            continue;
        }
        let accessed = if o.code() == OpCode::CPUI_LOAD { o.get_out() } else { o.get_in(2) };
        let width = accessed.and_then(|v| fd.vbank().get(v)).map(|v| v.get_size());
        let (Some(width), Some(ptr)) = (width, o.get_in(1)) else { continue };
        let p = strip(fd, ptr);
        let Some((phi, OpCode::CPUI_MULTIEQUAL)) = def_code(fd, p) else { continue };
        if seen.contains(&phi) {
            continue;
        }
        seen.push(phi);
        let Some((start, step, step_out)) = induction(fd, phi, sp) else { continue };
        if width as i64 != step {
            continue;
        }
        let mut cands = relabels(fd, p);
        cands.extend(relabels(fd, step_out));
        for c in cands {
            let Some(var) = fd.vbank().get(c) else { continue };
            for use_op in var.descend_iter() {
                let Some(u) = fd.obank().get(use_op) else { continue };
                let slot = u.get_slot(c);
                let bound_slot = match u.code() {
                    OpCode::CPUI_INT_EQUAL | OpCode::CPUI_INT_NOTEQUAL => 1 - slot,
                    OpCode::CPUI_INT_LESS if slot == 0 => 1,
                    _ => continue,
                };
                let Some(bound) = u.get_in(bound_slot) else { continue };
                let Some(end) = stack_offset(fd, bound, sp, WALK_DEPTH) else { continue };
                let span = end.wrapping_sub(start);
                if span <= step || span % step != 0 || span / step > MAX_ELEMENTS {
                    continue;
                }
                let walk = PointerWalk { cmp: use_op, slot: bound_slot, start, end, step };
                if !walks.contains(&walk) {
                    walks.push(walk);
                }
            }
        }
    }
    walks
}

fn overlaps(h: &RangeHint, w: &PointerWalk) -> bool {
    let size = if h.range_type == RangeType::Open && h.highind >= 0 {
        (h.highind as i64 + 1).wrapping_mul(h.type_.get_align_size() as i64).max(h.size as i64)
    } else {
        h.size as i64
    };
    h.sstart.wrapping_add(size) > w.start && h.sstart < w.end
}

fn blocks(h: &RangeHint, w: &PointerWalk) -> bool {
    if h.range_type == RangeType::Endpoint || h.is_type_lock() {
        return true;
    }
    if h.range_type == RangeType::Open && h.highind >= 0 {
        let extent = (h.highind as i64 + 1).wrapping_mul(h.type_.get_align_size() as i64);
        let end = h.sstart.wrapping_add(extent.max(h.size as i64));
        if (h.sstart < w.start && end > w.start) || (h.sstart >= w.start && end > w.end) {
            return true;
        }
    }
    if h.sstart < w.start {
        return h.range_type != RangeType::Open;
    }
    if h.sstart.wrapping_add(h.size as i64) > w.end {
        return true;
    }
    h.range_type == RangeType::Fixed
        && (h.flags & COPY_CONSTANT) == 0
        && h.type_.get_metatype() != type_metatype::TYPE_UNKNOWN
}

fn element_type(hints: &[RangeHint], w: &PointerWalk) -> Option<Rc<Datatype>> {
    let mut elem: Option<Rc<Datatype>> = None;
    for h in hints {
        if h.sstart < w.start || h.sstart >= w.end || h.type_.get_size() as i64 != w.step {
            continue;
        }
        if !matches!(
            h.type_.get_metatype(),
            type_metatype::TYPE_INT
                | type_metatype::TYPE_UINT
                | type_metatype::TYPE_BOOL
                | type_metatype::TYPE_FLOAT
                | type_metatype::TYPE_PTR
                | type_metatype::TYPE_ENUM_INT
                | type_metatype::TYPE_ENUM_UINT
        ) {
            continue;
        }
        if elem.as_ref().is_none_or(|e| h.type_.type_order(e).is_ok_and(|o| o < 0)) {
            elem = Some(Rc::clone(&h.type_));
        }
    }
    elem
}

/// Replace the frame hints inside each walk's `[start, end)` with one array hint.
pub fn coalesce_hints(
    state: &mut MapState,
    walks: &[PointerWalk],
    space: &Rc<AddrSpace>,
    types: &dyn TypeFactory,
) {
    let mut ordered: Vec<&PointerWalk> = walks.iter().collect();
    ordered.sort_by_key(|w| (w.start, std::cmp::Reverse(w.end)));
    let mut done: Vec<(i64, i64)> = Vec::new();
    for w in ordered {
        if done.iter().any(|(s, e)| *s < w.end && w.start < *e) {
            continue;
        }
        let start = space.wrap_offset(w.start as uintb);
        let size = (w.end - w.start) as int4;
        if !state.covers(start, size) {
            continue;
        }
        let hints = state.hints_mut();
        if hints.iter().any(|h| overlaps(h, w) && blocks(h, w)) {
            continue;
        }
        let elem = match element_type(hints, w) {
            Some(e) => e,
            None => match types.get_base(w.step as int4, type_metatype::TYPE_UNKNOWN) {
                Ok(e) => e,
                Err(_) => continue,
            },
        };
        hints.retain(|h| h.sstart < w.start || h.sstart >= w.end);
        hints.push(RangeHint::new(start, size, w.start, elem, 0, RangeType::Fixed, -1));
        done.push((w.start, w.end));
    }
}

fn array_covers(fd: &Funcdata, w: &PointerWalk) -> bool {
    let Some(lm) = fd.get_scope_local() else { return false };
    let space = Rc::clone(lm.get_space_id());
    let addr = Address::new(Rc::clone(&space), space.wrap_offset(w.start as uintb));
    let size = (w.end - w.start) as int4;
    lm.query_container(&addr, size, &Address::new_invalid()).is_some_and(|e| {
        e.addr.get_offset() == addr.get_offset()
            && e.size == size
            && e.sym_type.get_metatype() == type_metatype::TYPE_ARRAY
            && e.sym_type.get_size() == size
    })
}

/// Does every use of `def`'s output end at `cmp`, through plumbing and phis?
fn feeds_only(fd: &Funcdata, def: OpId, cmp: OpId) -> bool {
    let Some(out) = fd.obank().get(def).and_then(|o| o.get_out()) else { return false };
    let mut seen = Vec::new();
    let mut work = vec![out];
    while let Some(v) = work.pop() {
        if seen.contains(&v) {
            continue;
        }
        seen.push(v);
        let Some(var) = fd.vbank().get(v) else { return false };
        for d in var.descend_iter() {
            if d == cmp {
                continue;
            }
            let Some(o) = fd.obank().get(d) else { return false };
            let passes = o.code() == OpCode::CPUI_MULTIEQUAL
                || (is_plumbing(o.code()) && o.get_in(0) == Some(v));
            match (passes, o.get_out()) {
                (true, Some(next)) => work.push(next),
                _ => return false,
            }
        }
    }
    true
}

/// Rebuild each walk's raw `sp + end` as `&buf + (end - start)` once the frame
/// holds the array; returns the number of rewrites.
pub fn rebase_bounds(fd: &mut Funcdata, space: &Rc<AddrSpace>, walks: &[PointerWalk]) -> int4 {
    if walks.is_empty() || !fd.has_type_recovery_started() {
        return 0;
    }
    let Some(sp) = fd.find_spacebase_input(space) else { return 0 };
    let mut count = 0;
    for w in walks {
        let Some(bound) = input(fd, w.cmp, w.slot) else { continue };
        let Some(def) = address_def(fd, bound, &mut PHI_BUDGET.clone()) else { continue };
        let raw = matches!(
            fd.obank().get(def).map(|o| o.code()),
            Some(OpCode::CPUI_INT_ADD | OpCode::CPUI_PTRSUB)
        ) && input(fd, def, 0).map(|b| strip(fd, b)) == Some(sp)
            && input(fd, def, 1).and_then(|c| constant(fd, c)) == Some(w.end);
        if !raw || !array_covers(fd, w) {
            continue;
        }
        let size = fd.vbank().get(sp).map(|v| v.get_size()).unwrap_or(0);
        let mask = calc_mask(size);
        let start_c = fd.new_constant(size, (w.start as uintb) & mask);
        let span_c = fd.new_constant(size, ((w.end - w.start) as uintb) & mask);
        if feeds_only(fd, def, w.cmp) {
            let base = fd.new_op_before(def, OpCode::CPUI_PTRSUB, sp, start_c, None);
            let Some(base_out) = fd.obank().get(base).and_then(|o| o.get_out()) else { continue };
            if let Some(o) = fd.obank_mut().get_mut(def) {
                o.clear_stop_type_propagation();
            }
            fd.op_set_opcode_code(def, OpCode::CPUI_INT_ADD);
            if fd.op_set_input(def, base_out, 0).is_ok() && fd.op_set_input(def, span_c, 1).is_ok() {
                count += 1;
            }
        } else {
            let base = fd.new_op_before(w.cmp, OpCode::CPUI_PTRSUB, sp, start_c, None);
            let Some(base_out) = fd.obank().get(base).and_then(|o| o.get_out()) else { continue };
            let add = fd.new_op_before(w.cmp, OpCode::CPUI_INT_ADD, base_out, span_c, None);
            let Some(add_out) = fd.obank().get(add).and_then(|o| o.get_out()) else { continue };
            if fd.op_set_input(w.cmp, add_out, w.slot).is_ok() {
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
#[path = "kuna_endptrbound/tests.rs"]
mod tests;
