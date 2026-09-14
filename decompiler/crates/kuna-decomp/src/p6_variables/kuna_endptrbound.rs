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
//! With the option on, restructuring accepts a walk only when it can prove that
//! every dereference through the pointer lands in `[start, end)`:
//!
//! - The pointer is a `MULTIEQUAL` in the header of a natural loop.  Every input
//!   arriving on an edge from outside the loop is the stack address `start`;
//!   every input arriving on a back edge is the pointer plus one positive
//!   constant `step`.
//! - A comparison of the pointer (`t = p`) or of the value every back edge
//!   carries (`t = p + step`) against a stack address `end` a whole number of
//!   steps, and at least two, beyond `start` controls a loop exit: `==`, `!=`,
//!   `t < end` or `end <= t` feeds (through `BOOL_NEGATE`/`COPY`) the `CBRANCH`
//!   of a loop block whose one out-edge leaves the loop exactly when `t` reaches
//!   `end`, and that block dominates every back edge.  When `t` is `p` itself,
//!   every dereference also sits in a loop block the branch strictly dominates.
//!   Every comparison that controls an exit this way names the same `end`.
//! - Every use of the pointer is accounted for: dereferences through it plus a
//!   constant fit inside one `step` and one of them is exactly `step` bytes at
//!   offset 0; it may be compared, stepped, or turned into an integer that is
//!   never dereferenced.  A variable offset that reaches a dereference, a call
//!   argument, a stored or returned pointer, or a second phi declines the walk.
//!
//! Two things follow:
//!
//! - The frame hints inside `[start, end)` are replaced by one open hint for an
//!   array of `step`-byte elements with index evidence through the last one, so
//!   the buffer is one object that extends no further than the replaced hints
//!   would have (to the next frame reference).  The walk is left alone when a
//!   hint straddles either edge or is type-locked, when a non-constant fixed hint
//!   inside is typed or is not exactly one element wide -- a separately used
//!   variable, or the range read as a wider value -- when typed index evidence
//!   inside uses another element width (a byte walk over an `int` array), and
//!   when another walk overlaps it without one containing the other.
//! - Once the symbol at `start` is an array covering exactly `[start, end)`, the
//!   end address is rebuilt as `&buf + (end - start)`, which renders `&buf[n]`.
//!   The rewrite happens at the address's definition when nothing but the
//!   comparison consumes it (through copies, casts and phi plumbing), and in
//!   front of the comparison otherwise, so an address that also means the
//!   neighbour elsewhere keeps rendering as the neighbour there.  The rebuilt
//!   `PTRSUB(sp, start) + span` is still an additive stack reference at `end`,
//!   so later layouts see the same frame reference and the rewrite never
//!   changes the frame.
//!
//! The value compared is unchanged; only the base it is expressed on moves.

use std::collections::HashSet;
use std::rc::Rc;

use kuna_base::address::{calc_mask, sign_extend, Address};
use kuna_base::error::KunaResult;
use kuna_base::marshal::ElementId;
use kuna_base::space::AddrSpace;
use kuna_base::types::{int4, uintb};
use kuna_num::opcodes::OpCode;

use crate::context::{BlockId, OpId, VarnodeId};
use crate::dtype::{type_metatype, Datatype, TypeFactory};
use crate::funcdata::Funcdata;
use crate::p0_knowledge::options::on_or_off;
use crate::varmap::{MapState, RangeHint, RangeType, COPY_CONSTANT};

/// Marshaling element `<endptrbound>` (kuna 4000+ range; 4168 was the previous
/// high-water mark).
pub const ELEM_ENDPTRBOUND: ElementId = ElementId::new("endptrbound", 4169);

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
const USE_BUDGET: usize = 256;
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

/// The natural loop a candidate pointer phi heads.
struct WalkLoop {
    phi: OpId,
    ptr: VarnodeId,
    start: i64,
    step: i64,
    /// The value every back edge carries, when they all carry the same one.
    stepped: Option<VarnodeId>,
    latches: Vec<BlockId>,
    body: HashSet<BlockId>,
}

/// A dereference through the walking pointer plus `off`, `width` bytes wide.
#[derive(Debug, Clone, Copy)]
struct Deref {
    op: OpId,
    off: i64,
    width: i64,
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
    matches!(code, OpCode::CPUI_CAST | OpCode::CPUI_COPY)
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

/// `(base, delta)` when `op` computes `base + delta` for a constant `delta`.
fn constant_step(fd: &Funcdata, op: OpId) -> Option<(VarnodeId, i64)> {
    let o = fd.obank().get(op)?;
    let (a, b) = (o.get_in(0)?, o.get_in(1)?);
    match o.code() {
        OpCode::CPUI_INT_ADD => match (constant(fd, a), constant(fd, b)) {
            (None, Some(c)) => Some((a, c)),
            (Some(c), None) => Some((b, c)),
            _ => None,
        },
        OpCode::CPUI_PTRSUB => Some((a, constant(fd, b)?)),
        OpCode::CPUI_PTRADD => {
            let m = constant(fd, o.get_in(2)?)?;
            Some((a, constant(fd, b)?.checked_mul(m)?))
        }
        OpCode::CPUI_INT_SUB => Some((a, constant(fd, b)?.checked_neg()?)),
        _ => None,
    }
}

/// `(phi output, offset)` when `vn` is a `MULTIEQUAL` output plus constants.
fn phi_offset(fd: &Funcdata, vn: VarnodeId) -> Option<(VarnodeId, i64)> {
    let mut vn = strip(fd, vn);
    let mut off = 0i64;
    for _ in 0..WALK_DEPTH {
        let (op, code) = def_code(fd, vn)?;
        if code == OpCode::CPUI_MULTIEQUAL {
            return Some((vn, off));
        }
        let (base, delta) = constant_step(fd, op)?;
        off = off.checked_add(delta)?;
        vn = strip(fd, base);
    }
    None
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
    let (base, delta) = constant_step(fd, op)?;
    stack_offset(fd, base, sp, depth - 1)?.checked_add(delta)
}

/// `vn` and every value that only re-labels it (casts and copies).
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

/// The pointer phis some dereference reads through, in first-use order.
fn candidate_phis(fd: &Funcdata) -> Vec<OpId> {
    let mut phis = Vec::new();
    let accesses = fd.obank().iter_code(OpCode::CPUI_LOAD).chain(fd.obank().iter_code(OpCode::CPUI_STORE));
    for op in accesses {
        let Some(o) = fd.obank().get(op) else { continue };
        if o.is_dead() {
            continue;
        }
        let Some((ptr, _)) = o.get_in(1).and_then(|p| phi_offset(fd, p)) else { continue };
        let Some(phi) = fd.vbank().get(ptr).and_then(|v| v.get_def()) else { continue };
        if !phis.contains(&phi) {
            phis.push(phi);
        }
    }
    phis
}

/// The blocks of the natural loop with header `head` and back-edge sources `latches`.
fn natural_loop(fd: &Funcdata, head: BlockId, latches: &[BlockId]) -> HashSet<BlockId> {
    let graph = fd.bblocks_ref();
    let mut body = HashSet::from([head]);
    let mut work = latches.to_vec();
    while let Some(b) = work.pop() {
        if !body.insert(b) {
            continue;
        }
        let fb = graph.block(b);
        for i in 0..fb.size_in() {
            work.push(fb.get_in(i));
        }
    }
    body
}

/// The induction loop `phi` heads: stack-address inputs on entry edges and one
/// positive constant step on every back edge.
fn walk_loop(fd: &Funcdata, phi: OpId, sp: VarnodeId) -> Option<WalkLoop> {
    let o = fd.obank().get(phi)?;
    let (head, ptr, n) = (o.get_parent()?, o.get_out()?, o.num_input());
    let graph = fd.bblocks_ref();
    let hb = graph.block(head);
    if n < 2 || hb.size_in() != n {
        return None;
    }
    let (mut start, mut step) = (None, None);
    let mut carried: Vec<VarnodeId> = Vec::new();
    let mut latches = Vec::new();
    for i in 0..n {
        let src = hb.get_in(i);
        let val = input(fd, phi, i)?;
        if graph.dominates(head, Some(src)) {
            let (root, k) = phi_offset(fd, val)?;
            if root != ptr || k <= 0 || step.is_some_and(|s| s != k) {
                return None;
            }
            step = Some(k);
            let v = strip(fd, val);
            if !carried.contains(&v) {
                carried.push(v);
            }
            latches.push(src);
        } else {
            let a = stack_offset(fd, val, sp, WALK_DEPTH)?;
            if start.is_some_and(|s| s != a) {
                return None;
            }
            start = Some(a);
        }
    }
    let (start, step) = (start?, step?);
    let stepped = if carried.len() == 1 { Some(carried[0]) } else { None };
    let body = natural_loop(fd, head, &latches);
    Some(WalkLoop { phi, ptr, start, step, stepped, latches, body })
}

/// Every dereference reachable from the walking pointer, or `None` when a use
/// could reach memory at an offset the walk does not bound.  A value narrower
/// than a pointer, or the difference of the pointer and a stack address, is an
/// integer and is not followed.
fn pointer_derefs(fd: &Funcdata, sp: VarnodeId, lp: &WalkLoop) -> Option<Vec<Deref>> {
    let ptr_size = fd.vbank().get(lp.ptr)?.get_size();
    let mut derefs = Vec::new();
    let mut seen: HashSet<VarnodeId> = HashSet::new();
    let mut work: Vec<(VarnodeId, Option<i64>)> = vec![(lp.ptr, Some(0))];
    while let Some((v, off)) = work.pop() {
        if !seen.insert(v) {
            continue;
        }
        if seen.len() > USE_BUDGET {
            return None;
        }
        for d in fd.vbank().get(v)?.descend_iter() {
            let o = fd.obank().get(d)?;
            let code = o.code();
            match code {
                OpCode::CPUI_LOAD | OpCode::CPUI_STORE => {
                    if o.get_in(1) != Some(v) || (code == OpCode::CPUI_STORE && o.get_in(2) == Some(v)) {
                        return None;
                    }
                    let accessed = if code == OpCode::CPUI_LOAD { o.get_out() } else { o.get_in(2) };
                    let width = fd.vbank().get(accessed?)?.get_size() as i64;
                    derefs.push(Deref { op: d, off: off?, width });
                }
                OpCode::CPUI_MULTIEQUAL if d == lp.phi => {}
                OpCode::CPUI_INT_EQUAL
                | OpCode::CPUI_INT_NOTEQUAL
                | OpCode::CPUI_INT_LESS
                | OpCode::CPUI_INT_LESSEQUAL
                | OpCode::CPUI_INT_SLESS
                | OpCode::CPUI_INT_SLESSEQUAL => {}
                OpCode::CPUI_MULTIEQUAL
                | OpCode::CPUI_INDIRECT
                | OpCode::CPUI_CALL
                | OpCode::CPUI_CALLIND
                | OpCode::CPUI_CALLOTHER
                | OpCode::CPUI_RETURN
                | OpCode::CPUI_BRANCH
                | OpCode::CPUI_CBRANCH
                | OpCode::CPUI_BRANCHIND
                | OpCode::CPUI_CPOOLREF
                | OpCode::CPUI_NEW => return None,
                _ => {
                    let out = o.get_out()?;
                    if fd.vbank().get(out)?.get_size() < ptr_size {
                        continue;
                    }
                    let other = if o.get_in(0) == Some(v) { o.get_in(1) } else { o.get_in(0) };
                    if code == OpCode::CPUI_INT_SUB
                        && other.is_some_and(|y| y != v && stack_offset(fd, y, sp, WALK_DEPTH).is_some())
                    {
                        continue;
                    }
                    let next = match code {
                        OpCode::CPUI_CAST | OpCode::CPUI_COPY => off,
                        _ => off.zip(constant_step(fd, d)).and_then(|(a, (_, c))| a.checked_add(c)),
                    };
                    work.push((out, next));
                }
            }
        }
    }
    Some(derefs)
}

/// The loop block whose `CBRANCH` leaves the loop exactly when `cmp` evaluates
/// to `exit_when`, and which dominates every back edge.
fn exit_branch(fd: &Funcdata, lp: &WalkLoop, cmp: OpId, exit_when: bool) -> Option<BlockId> {
    let graph = fd.bblocks_ref();
    let mut seen: Vec<VarnodeId> = Vec::new();
    let mut work = vec![(fd.obank().get(cmp)?.get_out()?, exit_when)];
    while let Some((v, when)) = work.pop() {
        if seen.contains(&v) || seen.len() > WALK_DEPTH as usize {
            continue;
        }
        seen.push(v);
        let Some(var) = fd.vbank().get(v) else { continue };
        for d in var.descend_iter() {
            let Some(o) = fd.obank().get(d) else { continue };
            match o.code() {
                OpCode::CPUI_BOOL_NEGATE => work.extend(o.get_out().map(|out| (out, !when))),
                OpCode::CPUI_COPY => work.extend(o.get_out().map(|out| (out, when))),
                OpCode::CPUI_CBRANCH if o.get_in(1) == Some(v) => {
                    let Some(b) = o.get_parent() else { continue };
                    let fb = graph.block(b);
                    if !lp.body.contains(&b) || fb.size_out() != 2 {
                        continue;
                    }
                    let exit = if when != o.is_boolean_flip() { 1 } else { 0 };
                    if lp.body.contains(&fb.get_out(exit)) || !lp.body.contains(&fb.get_out(1 - exit)) {
                        continue;
                    }
                    if lp.latches.iter().all(|l| graph.dominates(b, Some(*l))) {
                        return Some(b);
                    }
                }
                _ => {}
            }
        }
    }
    None
}

/// Comparisons of `t` against a stack address that control an exit of `lp`,
/// each with whether it alone keeps every dereference inside the walk.
fn controlling_bounds(
    fd: &Funcdata,
    sp: VarnodeId,
    lp: &WalkLoop,
    t: VarnodeId,
    derefs: &[Deref],
    out: &mut Vec<(PointerWalk, bool)>,
) {
    let graph = fd.bblocks_ref();
    let tests_ptr = strip(fd, t) == lp.ptr;
    let Some(var) = fd.vbank().get(t) else { return };
    for cmp in var.descend_iter() {
        let Some(o) = fd.obank().get(cmp) else { continue };
        let (Some(a), Some(b)) = (o.get_in(0), o.get_in(1)) else { continue };
        if a == b {
            continue;
        }
        let (bound_slot, exit_when) = match o.code() {
            OpCode::CPUI_INT_EQUAL => (if a == t { 1 } else { 0 }, true),
            OpCode::CPUI_INT_NOTEQUAL => (if a == t { 1 } else { 0 }, false),
            OpCode::CPUI_INT_LESS if a == t => (1, false),
            OpCode::CPUI_INT_LESSEQUAL if b == t => (0, true),
            _ => continue,
        };
        let Some(bound) = o.get_in(bound_slot) else { continue };
        let Some(end) = stack_offset(fd, bound, sp, WALK_DEPTH) else { continue };
        let Some(branch) = exit_branch(fd, lp, cmp, exit_when) else { continue };
        let guarded = !tests_ptr
            || derefs.iter().all(|r| {
                fd.obank().get(r.op).and_then(|ro| ro.get_parent()).is_some_and(|rb| {
                    rb != branch && lp.body.contains(&rb) && graph.dominates(branch, Some(rb))
                })
            });
        let walk = PointerWalk { cmp, slot: bound_slot, start: lp.start, end, step: lp.step };
        if !out.iter().any(|(w, _)| *w == walk) {
            out.push((walk, guarded));
        }
    }
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

/// Every proven pointer walk over the stack frame addressed through `sp`.
pub fn find_pointer_walks(fd: &Funcdata, sp: VarnodeId) -> Vec<PointerWalk> {
    let mut walks: Vec<PointerWalk> = Vec::new();
    for phi in candidate_phis(fd) {
        let Some(lp) = walk_loop(fd, phi, sp) else { continue };
        let Some(derefs) = pointer_derefs(fd, sp, &lp) else { continue };
        if !derefs.iter().any(|r| r.off == 0 && r.width == lp.step)
            || derefs.iter().any(|r| r.off < 0 || r.off + r.width > lp.step)
        {
            continue;
        }
        let mut found = Vec::new();
        for t in relabels(fd, lp.ptr) {
            controlling_bounds(fd, sp, &lp, t, &derefs, &mut found);
        }
        if let Some(stepped) = lp.stepped {
            for t in relabels(fd, stepped) {
                controlling_bounds(fd, sp, &lp, t, &derefs, &mut found);
            }
        }
        let Some(&(first, _)) = found.first() else { continue };
        let span = first.end.wrapping_sub(first.start);
        if found.iter().any(|(w, _)| w.end != first.end)
            || !found.iter().any(|(_, guarded)| *guarded)
            || span < 2 * lp.step
            || span % lp.step != 0
            || span / lp.step > MAX_ELEMENTS
        {
            continue;
        }
        for (w, _) in found {
            if !walks.contains(&w) {
                walks.push(w);
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
        return h.range_type != RangeType::Open || h.sstart.wrapping_add(h.size as i64) > w.start;
    }
    if h.sstart.wrapping_add(h.size as i64) > w.end {
        return true;
    }
    if h.range_type == RangeType::Open
        && h.highind >= 0
        && h.type_.get_metatype() != type_metatype::TYPE_UNKNOWN
        && h.type_.get_align_size() as i64 != w.step
    {
        return true;
    }
    h.range_type == RangeType::Fixed
        && (h.flags & COPY_CONSTANT) == 0
        && (h.type_.get_metatype() != type_metatype::TYPE_UNKNOWN || h.size as i64 != w.step)
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
/// The hint is open, with index evidence through the last element, so like the
/// open hints it replaces it runs on to the next hint: the frame reference at
/// `end` stops it there, and without one the array keeps whatever extent the
/// frame already gave it.  A walk nested in one already coalesced is vetoed by
/// that hint's index evidence, so the outermost walk wins.
pub fn coalesce_hints(
    state: &mut MapState,
    walks: &[PointerWalk],
    space: &Rc<AddrSpace>,
    types: &dyn TypeFactory,
) {
    let mut ordered: Vec<&PointerWalk> = walks.iter().collect();
    ordered.sort_by_key(|w| (w.start, std::cmp::Reverse(w.end)));
    for w in ordered.iter().copied() {
        let crossed = ordered.iter().any(|o| {
            let nested = (o.start <= w.start && w.end <= o.end) || (w.start <= o.start && o.end <= w.end);
            o.start < w.end && w.start < o.end && !nested
        });
        if crossed {
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
        let last = ((w.end - w.start) / w.step - 1) as int4;
        hints.push(RangeHint::new(start, w.step as int4, w.start, elem, 0, RangeType::Open, last));
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
