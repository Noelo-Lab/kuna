//! (kuna) `msvcbyval` — size an MSVC `__cdecl` import's by-value class argument
//! at the call site, so every parameter after it is placed.
//!
//! # The gap
//!
//! An MSVC name states each parameter's type but not the size of a class passed
//! by value, so `msvcsig` applies a `__cdecl` import's parameters up to that class
//! and leaves the rest open. The call-site recovery cannot take the open slots:
//! MSVC builds the class in its outgoing slot through a pointer (a copy
//! constructor's `this`), and `checkInputTrialUse` refuses every stack slot at or
//! above an address the caller takes. `?MakeStack@@YA?AVArr@@V1@AAV1@1@Z`
//! (`Arr MakeStack(Arr, Arr &, Arr &)`) kept only its hidden return pointer.
//!
//! # The size
//!
//! The caller pops a `__cdecl` callee's arguments, so an `add esp,k` that is the
//! very next instruction states `k` argument bytes, and the class's slot is `k`
//! less the declared parameters' 4-byte-aligned slots. The call's prototype
//! becomes the full declaration, the class an `undefined` of that width, before
//! `ActionFuncLink` reads it. Every recovered call site is read on its own.
//!
//! A cleanup can say more than the call's arguments: a compiler may defer an
//! earlier call's pop into it, or fold the frame's own release into it. So the
//! raw p-code of the call's block is walked back from the call, tracking the
//! stack pointer, until it stands at the top of the argument area, and the size
//! is declined unless:
//!
//! * every declared parameter's slot is written by a push made after the slot
//!   was last reserved (the class's own slot may be reserved by `push ecx` or
//!   `sub esp,n`), and no address is taken of one while it is live, which is
//!   what a local the cleanup also releases would show;
//! * every earlier call on the way states its pop exactly (its model or its
//!   locked prototype) and, when it pops only the return address, is cleaned by
//!   the instruction after it or declares no stack parameter;
//! * the stack pointer changes only by a constant, rises only as such a
//!   cleanup, and no single reservation straddles the top of the area;
//! * the walk does not leave the block.
//!
//! # The object
//!
//! A slot whose address the caller forms is an object as well as an argument,
//! so `ActionRestrictLocal` leaves it mapped, and whatever fills it through that
//! address and the argument print as one local. When the slot's address is the
//! object a call builds -- the `this` (`ecx`) of a `__thiscall` MSVC constructor
//! (`??0`), or the hidden return pointer (first stack argument) of an MSVC
//! function returning a class by value -- Heritage gives the slot to that call
//! as a value it creates rather than one it may modify, so the `ecx` value a
//! `push ecx` stored to reserve the slot is dead.

use std::collections::BTreeMap;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::KunaResult;
use kuna_base::space::{spacetype, AddrSpace};
use kuna_base::types::{int4, uintb};
use kuna_num::opcodes::OpCode;

use crate::context::OpId;
use crate::dtype::{type_metatype, Datatype, TypeFactory};
use crate::fspec::{FuncCallSpecs, FuncProto, PrototypePieces, EXTRAPOP_UNKNOWN, OFFSET_UNKNOWN};
use crate::funcdata::Funcdata;
use crate::p0_knowledge::options::on_or_off;

/// `option msvcbyval on|off` (C++ `ArchOption` shape).
pub struct OptionMsvcByval;

impl OptionMsvcByval {
    /// The option name.
    pub const NAME: &'static str = "msvcbyval";

    /// Resolve the flag and its confirmation message; the caller writes it into
    /// `Architecture::msvc_byval`.
    pub fn apply(&self, p1: &str) -> KunaResult<(bool, String)> {
        let val = on_or_off(p1)?;
        let prop = if val { "on" } else { "off" };
        Ok((val, format!("MSVC by-value argument sizing turned {prop}")))
    }
}

/// An MSVC `__cdecl` declaration with one by-value class parameter of unstated
/// size: the full parameter list, `pieces.intypes[at]` a placeholder for the
/// class that a call site replaces with its slot's width.
#[derive(Clone, Debug)]
pub struct ByvalDecl {
    pub pieces: PrototypePieces,
    pub at: usize,
}

/// Declarations by entry `(space index, offset)`.
pub type ByvalDecls = BTreeMap<(int4, uintb), ByvalDecl>;

/// Where a call takes the address of the object it builds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Builder {
    /// A `__thiscall` constructor: `this`, in `ecx`.
    This,
    /// A function returning a class by value: the hidden return pointer, its
    /// first stack argument.
    ReturnSlot,
}

/// MSVC-declared builders by entry `(space index, offset)`.
pub type Builders = BTreeMap<(int4, uintb), Builder>;

/// One call whose by-value slot was sized: the call that builds the argument in
/// that slot, if one was found, and whether the slot's address was formed.
#[derive(Clone, Debug)]
pub struct ByvalSite {
    pub call: OpId,
    pub param: usize,
    pub builder: Option<OpId>,
    pub mapped: bool,
}

/// Most argument bytes a single cleanup is read as, as for `calleepop`.
const MAX_ARG_BYTES: int4 = 256;

fn align4(n: int4) -> int4 {
    (n + 3) & !3
}

fn key(addr: &Address) -> Option<(int4, uintb)> {
    Some((addr.get_space()?.get_index(), addr.get_offset()))
}

/// The stack pointer register, as an address and size.
fn stack_pointer(data: &Funcdata) -> Option<(Address, int4)> {
    let stack = data.get_arch().manage().get_stack_space()?;
    let point = stack.get_spacebase(0).ok()?;
    Some((Address::new(point.space.clone()?, point.offset), point.size as int4))
}

/// Size the by-value slot of every call to a [`ByvalDecl`] callee, replacing the
/// call's open prototype with the full one; run at the end of
/// `ActionDefaultParams`, on the raw p-code.
pub fn size_slots(data: &mut Funcdata) {
    data.kuna_set_msvc_byval_sites(Vec::new());
    let arch = data.get_arch().clone();
    if !arch.msvc_byval || arch.msvc_byval_decls.is_empty() {
        return;
    }
    let Some(sp) = stack_pointer(data) else { return };
    let Some(types) = arch.types() else { return };
    let Ok(void_ty) = types.get_type_void() else { return };
    let mut sites = Vec::new();
    for i in 0..data.num_calls() {
        let Some(decl) = key(data.get_call_specs(i).get_entry_address())
            .and_then(|k| arch.msvc_byval_decls.get(&k))
        else {
            continue;
        };
        let fc = data.get_call_specs(i);
        if !fc.proto().has_model() || !fc.proto().is_input_locked() || !fc.proto().is_dotdotdot() {
            continue;
        }
        if fc.proto().get_extra_pop() != 4 || decl.at >= decl.pieces.intypes.len() {
            continue;
        }
        let model = Rc::clone(fc.proto().model());
        let call = fc.get_op();
        let Some(k) = cleanup_after(data, call, &sp) else { continue };
        let top = 4 + k;
        let widths: Vec<int4> = decl.pieces.intypes.iter().map(|t| align4(t.get_size())).collect();
        let known: int4 =
            widths.iter().enumerate().filter(|(j, _)| *j != decl.at).map(|(_, w)| *w).sum();
        if known >= k {
            continue;
        }
        // The class slot starts above the parameters below it, all known-width.
        let lo: int4 = 4 + widths[..decl.at].iter().sum::<int4>();
        let Some(walk) = walk_back(data, call, &sp, top, lo, &arch.msvc_builders) else { continue };
        // The class slot's size, cross-checked against the cleanup so a padding
        // reservation or a local released by the same `add esp` is declined.
        let slot = match walk.builder {
            // A copy constructor (or by-value return) fills the slot through a
            // pointer, so its bytes are not pushes: take the reservation it
            // captured, and require the cleanup to account for exactly it plus
            // the declared parameters.
            Some(_) => {
                let Some(r) = walk.slot_reserved else { continue };
                if r <= 0 || r % 4 != 0 || known + r != k {
                    continue;
                }
                r
            }
            // A trivially copyable aggregate is stored directly, so every byte
            // of the argument area is a push: a reservation nothing writes is a
            // padding slot or a local, and the sizing is declined.
            None => {
                if (4..top).any(|b| !walk.written[b as usize]) {
                    continue;
                }
                k - known
            }
        };
        if slot <= 0 || slot % 4 != 0 {
            continue;
        }
        // An address formed into a live byte of the argument area outside the
        // class slot is a local the cleanup also releases.
        if walk.pointers.iter().any(|t| (4..top).contains(t) && !(lo..lo + slot).contains(t)) {
            continue;
        }
        let mut layout = Vec::with_capacity(widths.len());
        let mut off = 4;
        for (j, w) in widths.iter().enumerate() {
            let w = if j == decl.at { slot } else { *w };
            layout.push((off, w));
            off += w;
        }
        let pushed = layout
            .iter()
            .enumerate()
            .filter(|(j, _)| *j != decl.at)
            .all(|(_, (o, w))| (*o..*o + *w).all(|b| walk.written[b as usize]));
        if !pushed {
            continue;
        }
        let Some(slot_ty) = slot_type(types, slot) else { continue };
        let mut pieces = decl.pieces.clone();
        pieces.intypes[decl.at] = slot_ty;
        pieces.first_var_arg_slot = -1;
        let mut fp = FuncProto::new();
        if fp.seed_locked_from_pieces(&pieces, model, void_ty.clone(), types, arch.manage()).is_err() {
            continue;
        }
        if !laid_out(&fp, &layout) {
            continue;
        }
        data.get_call_specs_mut(i).proto_mut().copy(&fp);
        let mapped =
            walk.builder.is_some() || walk.pointers.iter().any(|p| (lo..lo + slot).contains(p));
        sites.push(ByvalSite { call, param: decl.at, builder: walk.builder, mapped });
    }
    data.kuna_set_msvc_byval_sites(sites);
}

/// The type of a by-value slot of `size` bytes.
fn slot_type(types: &dyn TypeFactory, size: int4) -> Option<Rc<Datatype>> {
    if size <= 8 {
        return types.get_base(size, type_metatype::TYPE_UNKNOWN).ok();
    }
    let byte = types.get_base(1, type_metatype::TYPE_UNKNOWN).ok()?;
    types.get_type_array(size, byte).ok()
}

/// Did the model lay every parameter out on the stack where `layout` puts it?
fn laid_out(fp: &FuncProto, layout: &[(int4, int4)]) -> bool {
    if fp.num_params() as usize != layout.len() {
        return false;
    }
    layout.iter().enumerate().all(|(j, (off, w))| {
        fp.get_param(j as int4).is_some_and(|p| {
            let addr = p.get_address();
            addr.get_space().is_some_and(|s| s.get_type() == spacetype::IPTR_SPACEBASE)
                && addr.get_offset() == *off as uintb
                && align4(p.get_size()) == *w
        })
    })
}

fn is_sp(data: &Funcdata, vn: Option<crate::context::VarnodeId>, sp: &(Address, int4)) -> bool {
    vn.and_then(|v| data.vbank().get(v))
        .is_some_and(|v| v.get_addr() == &sp.0 && v.get_size() == sp.1)
}

fn constant(data: &Funcdata, vn: Option<crate::context::VarnodeId>) -> Option<int4> {
    let v = vn.and_then(|v| data.vbank().get(v))?;
    if !v.is_constant() || v.get_size() > 8 || v.get_size() <= 0 {
        return None;
    }
    let shift = 64 - (v.get_size() as u32) * 8;
    i32::try_from(((v.get_offset() << shift) as i64) >> shift).ok()
}

/// How an op changes the stack pointer: `Some(Some(delta))` for `sp = sp ± #c`,
/// `Some(None)` for any other write of it, `None` when it leaves it alone.
fn sp_delta(data: &Funcdata, op: OpId, sp: &(Address, int4)) -> Option<Option<int4>> {
    let o = data.obank().get(op)?;
    if !is_sp(data, o.get_out(), sp) {
        return None;
    }
    let delta = match o.code() {
        OpCode::CPUI_INT_ADD if is_sp(data, o.get_in(0), sp) => constant(data, o.get_in(1)),
        OpCode::CPUI_INT_SUB if is_sp(data, o.get_in(0), sp) => {
            constant(data, o.get_in(1)).and_then(|c| c.checked_neg())
        }
        _ => None,
    };
    Some(delta)
}

/// The offset from the stack pointer of an address `op` forms from it into a
/// register or temporary (`mov r,esp`, `lea r,[esp+c]`, a `[esp+c]` operand).
fn sp_pointer(data: &Funcdata, op: OpId, sp: &(Address, int4)) -> Option<int4> {
    let o = data.obank().get(op)?;
    if o.get_out().is_none() || is_sp(data, o.get_out(), sp) {
        return None;
    }
    match o.code() {
        OpCode::CPUI_COPY if is_sp(data, o.get_in(0), sp) => Some(0),
        OpCode::CPUI_INT_ADD if is_sp(data, o.get_in(0), sp) => constant(data, o.get_in(1)),
        OpCode::CPUI_INT_ADD if is_sp(data, o.get_in(1), sp) => constant(data, o.get_in(0)),
        _ => None,
    }
}

/// The ops of the instruction after `op`, in its block.
fn next_instruction(data: &Funcdata, op: OpId) -> Vec<OpId> {
    let Some(first) = data.obank().get(op).and_then(|o| o.basic_neighbours().1) else {
        return Vec::new();
    };
    let Some(at) = data.obank().get(first).map(|o| o.get_addr().clone()) else {
        return Vec::new();
    };
    let mut ops = vec![first];
    let mut cur = first;
    while let Some(next) = data.obank().get(cur).and_then(|o| o.basic_neighbours().1) {
        if data.obank().get(next).map(|o| o.get_addr() != &at).unwrap_or(true) {
            break;
        }
        ops.push(next);
        cur = next;
    }
    ops
}

/// The argument bytes the caller pops after the call, when the instruction
/// right after it is a single `add esp,k`.
///
/// Only an `add esp,k` is read: a `pop` raises the stack pointer the same way
/// but also reads the stack, and a run of pops (MSVC `/O1` argument cleanup)
/// cannot be told apart from a `pop ebp` epilogue locally, so the sizing is
/// declined there rather than risk a wrong count.
fn cleanup_after(data: &Funcdata, op: OpId, sp: &(Address, int4)) -> Option<int4> {
    let mut k = None;
    let ops = next_instruction(data, op);
    for &o in &ops {
        let code = data.obank().get(o).map(|o| o.code());
        if matches!(code, Some(OpCode::CPUI_LOAD | OpCode::CPUI_STORE))
            && touches_sp(data, o, sp)
        {
            return None; // a pop/push, not a bare `add esp,k`
        }
        match sp_delta(data, o, sp) {
            None => {}
            Some(Some(c)) if c > 0 && k.is_none() => k = Some(c),
            _ => return None,
        }
    }
    if split_cleanup(data, *ops.last()?, sp) {
        return None;
    }
    k.filter(|k| *k % 4 == 0 && *k <= MAX_ARG_BYTES)
}

/// Does another bare `add esp,c` follow `last` (the cleanup's last op) before
/// the next call, branch or return, or the end of the block? Then the cleanup
/// may be split (`add esp,8; add esp,4`) and the first add is not all of it. A
/// `pop` (which also reads the stack) is an epilogue and does not count.
fn split_cleanup(data: &Funcdata, last: OpId, sp: &(Address, int4)) -> bool {
    let mut at: Option<Address> = None;
    let mut reads_sp = false;
    let mut cur = data.obank().get(last).and_then(|o| o.basic_neighbours().1);
    while let Some(op) = cur {
        let Some(o) = data.obank().get(op) else { return false };
        if matches!(
            o.code(),
            OpCode::CPUI_CALL
                | OpCode::CPUI_CALLIND
                | OpCode::CPUI_BRANCH
                | OpCode::CPUI_CBRANCH
                | OpCode::CPUI_BRANCHIND
                | OpCode::CPUI_RETURN
        ) {
            return false;
        }
        if at.as_ref() != Some(o.get_addr()) {
            at = Some(o.get_addr().clone());
            reads_sp = false;
        }
        reads_sp |= touches_sp(data, op, sp);
        if matches!(sp_delta(data, op, sp), Some(Some(c)) if c > 0) && !reads_sp {
            return true;
        }
        cur = o.basic_neighbours().1;
    }
    false
}

/// Does a LOAD or STORE address the stack through `sp` (`sp` or `sp + #c`)?
fn touches_sp(data: &Funcdata, op: OpId, sp: &(Address, int4)) -> bool {
    let Some(o) = data.obank().get(op) else { return false };
    let addr = match o.code() {
        OpCode::CPUI_LOAD | OpCode::CPUI_STORE => o.get_in(1),
        _ => return false,
    };
    let Some(vn) = addr else { return false };
    if is_sp(data, Some(vn), sp) {
        return true;
    }
    data.vbank()
        .get(vn)
        .and_then(|v| v.get_def())
        .and_then(|d| data.obank().get(d))
        .is_some_and(|d| {
            d.code() == OpCode::CPUI_INT_ADD
                && (is_sp(data, d.get_in(0), sp) || is_sp(data, d.get_in(1), sp))
        })
}

/// What the walk back from a call saw of its argument area.
struct Walk {
    /// Per byte of `[0, top)` from the call's stack pointer: written by a push
    /// after it was last reserved.
    written: Vec<bool>,
    /// The call that builds the object at the by-value slot's start.
    builder: Option<OpId>,
    /// The bytes the one reservation the builder captured its pointer at took
    /// (`push ecx` = 4, `sub esp,n` = n): the class slot's real size.
    slot_reserved: Option<int4>,
    /// Every offset at which the walk saw an address formed from the stack
    /// pointer while that offset was still live.
    pointers: Vec<int4>,
}

/// The builder kind of the call `op`, when its prototype fits it.
fn builder_kind(data: &Funcdata, op: OpId, builders: &Builders) -> Option<Builder> {
    let fc = data.get_call_specs(data.get_call_specs_index(op)?);
    let kind = *builders.get(&key(fc.get_entry_address())?)?;
    let model_ok = fc.proto().has_model()
        && match kind {
            Builder::This => fc.proto().get_model_name() == "__thiscall",
            Builder::ReturnSlot => fc.proto().is_input_locked(),
        };
    model_ok.then_some(kind)
}

/// Do two storage ranges in one space overlap?
fn overlaps(a: &Address, asz: int4, b: &Address, bsz: int4) -> bool {
    match (a.get_space(), b.get_space()) {
        (Some(x), Some(y)) if Rc::ptr_eq(x, y) => {
            let (ao, bo) = (a.get_offset(), b.get_offset());
            ao < bo.wrapping_add(bsz as uintb) && bo < ao.wrapping_add(asz as uintb)
        }
        _ => false,
    }
}

/// Walk the raw p-code back from `call` until the stack pointer stands at `top`
/// bytes above its value at the call (see the module header); `None` when the
/// walk declines. `lo` is the by-value slot's bottom, from the call's stack
/// pointer; its size is not yet known and is what the walk measures.
fn walk_back(
    data: &Funcdata,
    call: OpId,
    sp: &(Address, int4),
    top: int4,
    lo: int4,
    builders: &Builders,
) -> Option<Walk> {
    let n = top as usize;
    let mut claimed = vec![false; n];
    let mut written = vec![false; n];
    let mut d: int4 = 0;
    let mut cleaned: Option<OpId> = None;
    // (builder call, storage whose latest write is the object address it gets)
    let mut pending: Vec<(OpId, Address, int4)> = Vec::new();
    // (return-slot builder call, offset of the push of its first stack argument)
    let mut want: Option<(OpId, int4)> = None;
    let mut builder = None;
    let mut slot_reserved = None;
    let mut after_capture = false;
    let mut pointers = Vec::new();
    let ecx = this_register(data);
    let mut cur = data.op_previous_op(call)?;
    loop {
        let o = data.obank().get(cur)?;
        if let Some(out) = o.get_out().and_then(|v| data.vbank().get(v)) {
            let mut resolved = Vec::new();
            pending.retain_mut(|(c, a, sz)| {
                if !overlaps(out.get_addr(), out.get_size(), a, *sz) {
                    return true;
                }
                if out.get_addr() != a || out.get_size() != *sz {
                    return false;
                }
                match o.code() {
                    OpCode::CPUI_COPY if is_sp(data, o.get_in(0), sp) => resolved.push((*c, d)),
                    OpCode::CPUI_INT_ADD if is_sp(data, o.get_in(0), sp) => {
                        if let Some(k) = constant(data, o.get_in(1)) {
                            resolved.push((*c, d + k));
                        }
                    }
                    OpCode::CPUI_COPY => {
                        if let Some(v) = o.get_in(0).and_then(|v| data.vbank().get(v)) {
                            if !v.is_constant() {
                                *a = v.get_addr().clone();
                                *sz = v.get_size();
                                return true;
                            }
                        }
                    }
                    _ => {}
                }
                false
            });
            for (c, ptr) in resolved {
                let fresh = !claimed[lo as usize] && !written[lo as usize];
                if ptr == lo && fresh && builder.is_none() {
                    builder = Some(c);
                    after_capture = true;
                }
            }
        }
        match o.code() {
            OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => {
                let idx = data.get_call_specs_index(cur)?;
                let fc = data.get_call_specs(idx);
                let e = fc.get_extra_pop();
                if e == EXTRAPOP_UNKNOWN || e < 4 {
                    return None;
                }
                if e == 4 && cleaned != Some(cur) && !declares_no_stack_input(fc) {
                    return None;
                }
                d -= e;
                cleaned = None;
                pending.clear();
                want = None;
                match builder_kind(data, cur, builders) {
                    Some(Builder::This) => {
                        if let Some(r) = &ecx {
                            pending.push((cur, r.0.clone(), r.1));
                        }
                    }
                    Some(Builder::ReturnSlot) => want = Some((cur, d + 4)),
                    None => {}
                }
            }
            OpCode::CPUI_STORE if is_sp(data, o.get_in(1), sp) => {
                let v = o.get_in(2).and_then(|v| data.vbank().get(v))?;
                for b in d.max(0)..(d + v.get_size()).min(top) {
                    if !claimed[b as usize] {
                        written[b as usize] = true;
                    }
                }
                if let Some((c, _)) = want.filter(|(_, at)| *at == d) {
                    want = None;
                    if !v.is_constant() {
                        pending.push((c, v.get_addr().clone(), v.get_size()));
                    }
                }
            }
            _ => {}
        }
        if let Some(t) = sp_pointer(data, cur, sp).map(|c| d + c) {
            if (0..top).contains(&t) && !claimed[t as usize] {
                pointers.push(t);
            }
        }
        match sp_delta(data, cur, sp) {
            None => {}
            Some(None) => return None,
            Some(Some(delta)) if delta < 0 => {
                if after_capture && slot_reserved.is_none() {
                    slot_reserved = Some(-delta);
                    after_capture = false;
                }
                let from = d;
                d -= delta;
                for b in from.max(0)..d.min(top) {
                    claimed[b as usize] = true;
                }
                if from < top && d > top {
                    return None;
                }
                if d == top {
                    return Some(Walk { written, builder, slot_reserved, pointers });
                }
            }
            Some(Some(delta)) => {
                cleaned = Some(cleanup_of(data, cur)?);
                d -= delta;
            }
        }
        cur = data.op_previous_op(cur)?;
    }
}

/// The call whose cleanup the stack-pointer raise `op` is: the instruction
/// before `op`'s ends with that call.
fn cleanup_of(data: &Funcdata, op: OpId) -> Option<OpId> {
    let at = data.obank().get(op)?.get_addr().clone();
    let mut cur = data.op_previous_op(op)?;
    while data.obank().get(cur)?.get_addr() == &at {
        cur = data.op_previous_op(cur)?;
    }
    matches!(data.obank().get(cur)?.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND).then_some(cur)
}

/// A locked, non-variadic prototype with no parameter on the stack.
fn declares_no_stack_input(fc: &FuncCallSpecs) -> bool {
    let p = fc.proto();
    if !p.is_input_locked() || p.is_dotdotdot() {
        return false;
    }
    (0..p.num_params()).all(|j| {
        p.get_param(j).is_some_and(|param| {
            param.get_address().get_space().is_some_and(|s| s.get_type() != spacetype::IPTR_SPACEBASE)
        })
    })
}

/// The `__thiscall` `this` register (`ECX`), as an address and size.
fn this_register(data: &Funcdata) -> Option<(Address, int4)> {
    let reg = data.get_arch().manage().register_lookup()?.probe_register("ECX")?;
    Some((Address::new(reg.space.clone()?, reg.offset), reg.size as int4))
}

/// The caller-frame stack range each recorded builder call builds, as
/// `(builder call, offset, size)` in `stack`. `qlst` is the function's call
/// specs, lifted out by `Heritage::guardCalls`.
pub fn constructed_ranges(
    data: &Funcdata,
    qlst: &[FuncCallSpecs],
    stack: &Rc<AddrSpace>,
) -> Vec<(OpId, uintb, int4)> {
    let mut out = Vec::new();
    for site in data.kuna_msvc_byval_sites() {
        let Some(builder) = site.builder else { continue };
        let Some(fc) = qlst.iter().find(|fc| fc.get_op() == site.call) else { continue };
        let base = fc.get_spacebase_offset();
        if base == OFFSET_UNKNOWN {
            continue;
        }
        let Some(param) = fc.proto().get_param(site.param as int4) else { continue };
        let addr = param.get_address();
        if !addr.get_space().is_some_and(|s| Rc::ptr_eq(s, stack)) {
            continue;
        }
        out.push((builder, stack.wrap_offset(addr.get_offset().wrapping_add(base)), param.get_size()));
    }
    out
}

/// Is `[off, off+size)` inside the range the call `op` constructs?
pub fn constructs(ranges: &[(OpId, uintb, int4)], op: OpId, stack: &AddrSpace, off: uintb, size: int4) -> bool {
    ranges
        .iter()
        .any(|(c, lo, sz)| *c == op && within(stack.wrap_offset(off.wrapping_sub(*lo)), size, *sz))
}

/// Does a range `size` bytes long, starting `rel` bytes into an object of
/// `object` bytes, lie wholly inside it?
fn within(rel: uintb, size: int4, object: int4) -> bool {
    size > 0 && rel.checked_add(size as uintb).is_some_and(|end| end <= object as uintb)
}

/// Does `ActionRestrictLocal` leave parameter `param` of the call `op` mapped?
/// True for a by-value slot whose address the caller formed: that slot is an
/// object, not just an argument.
pub fn keeps_slot_mapped(data: &Funcdata, op: OpId, param: int4) -> bool {
    data.kuna_msvc_byval_sites()
        .iter()
        .any(|s| s.call == op && s.param as int4 == param && s.mapped)
}

#[cfg(test)]
mod tests;
