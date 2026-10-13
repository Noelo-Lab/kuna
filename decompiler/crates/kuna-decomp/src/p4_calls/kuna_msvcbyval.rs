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
//! When `ecx` is loaded with the slot's address and the next call is an MSVC
//! constructor (`??0`), that call builds the argument. Heritage gives the slot to
//! it as a value it creates rather than one it may modify, so the `ecx` value a
//! `push ecx` stored to reserve the slot is dead, and `ActionRestrictLocal` leaves
//! the slot mapped, so the constructor's `this` and the argument print as one
//! local.

use std::collections::{BTreeMap, BTreeSet};
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

/// Entries `(space index, offset)` declared as MSVC constructors.
pub type CtorEntries = BTreeSet<(int4, uintb)>;

/// One call whose by-value slot was sized, and the constructor call that builds
/// the argument in that slot, if one was found.
#[derive(Clone, Debug)]
pub struct ByvalSite {
    pub call: OpId,
    pub param: usize,
    pub ctor: Option<OpId>,
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
        let widths: Vec<int4> = decl.pieces.intypes.iter().map(|t| align4(t.get_size())).collect();
        let known: int4 =
            widths.iter().enumerate().filter(|(j, _)| *j != decl.at).map(|(_, w)| *w).sum();
        let slot = k - known;
        if slot <= 0 || slot % 4 != 0 {
            continue;
        }
        let mut layout = Vec::with_capacity(widths.len());
        let mut off = 4;
        for (j, w) in widths.iter().enumerate() {
            let w = if j == decl.at { slot } else { *w };
            layout.push((off, w));
            off += w;
        }
        let (lo, _) = layout[decl.at];
        let Some(walk) = walk_back(data, call, &sp, 4 + k, (lo, lo + slot)) else { continue };
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
        let ctor = walk.this_call.filter(|c| {
            data.get_call_specs_index(*c)
                .and_then(|ci| key(data.get_call_specs(ci).get_entry_address()))
                .is_some_and(|k| arch.msvc_ctors.contains(&k))
        });
        sites.push(ByvalSite { call, param: decl.at, ctor });
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

/// The bytes the instruction right after the call `op` raises the stack pointer
/// by, when that instruction is a single `sp = sp + #k` (`add esp,k`).
fn cleanup_after(data: &Funcdata, op: OpId, sp: &(Address, int4)) -> Option<int4> {
    let mut k = None;
    for o in next_instruction(data, op) {
        match sp_delta(data, o, sp) {
            None => {}
            Some(Some(c)) if c > 0 && k.is_none() => k = Some(c),
            _ => return None,
        }
    }
    k.filter(|k| *k % 4 == 0 && *k <= MAX_ARG_BYTES)
}

/// What the walk back from a call saw of its argument area.
struct Walk {
    /// Per byte of `[0, top)` from the call's stack pointer: written by a push
    /// after it was last reserved.
    written: Vec<bool>,
    /// The call that `ecx` was pointed at the by-value slot for.
    this_call: Option<OpId>,
}

/// Walk the raw p-code back from `call` until the stack pointer stands at `top`
/// bytes above its value at the call (see the module header); `None` when the
/// walk declines.
fn walk_back(
    data: &Funcdata,
    call: OpId,
    sp: &(Address, int4),
    top: int4,
    slot: (int4, int4),
) -> Option<Walk> {
    let n = top as usize;
    let mut claimed = vec![false; n];
    let mut written = vec![false; n];
    let mut d: int4 = 0;
    let mut cleaned: Option<OpId> = None;
    let mut next_call: Option<OpId> = None;
    let mut ecx_set = false;
    let mut this_call = None;
    let ecx = this_register(data);
    let mut cur = data.op_previous_op(call)?;
    loop {
        let o = data.obank().get(cur)?;
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
                next_call = Some(cur);
                ecx_set = false;
            }
            OpCode::CPUI_STORE if is_sp(data, o.get_in(1), sp) => {
                let size = o.get_in(2).and_then(|v| data.vbank().get(v)).map(|v| v.get_size())?;
                for b in d.max(0)..(d + size).min(top) {
                    if !claimed[b as usize] {
                        written[b as usize] = true;
                    }
                }
            }
            _ => {}
        }
        if let Some(t) = sp_pointer(data, cur, sp).map(|c| d + c) {
            if (4..top).contains(&t) && !claimed[t as usize] && !(slot.0..slot.1).contains(&t) {
                return None;
            }
        }
        if let (Some(r), Some(out)) = (&ecx, o.get_out().and_then(|v| data.vbank().get(v))) {
            if out.get_addr() == &r.0 && !ecx_set {
                ecx_set = true;
                let points = match o.code() {
                    OpCode::CPUI_COPY if is_sp(data, o.get_in(0), sp) => Some(d),
                    OpCode::CPUI_INT_ADD if is_sp(data, o.get_in(0), sp) => {
                        constant(data, o.get_in(1)).map(|c| d + c)
                    }
                    _ => None,
                };
                let fresh = |b: int4| !claimed[b as usize] && !written[b as usize];
                if points == Some(slot.0) && (slot.0..slot.1).all(fresh) {
                    this_call = next_call;
                }
            }
        }
        match sp_delta(data, cur, sp) {
            None => {}
            Some(None) => return None,
            Some(Some(delta)) if delta < 0 => {
                let from = d;
                d -= delta;
                for b in from.max(0)..d.min(top) {
                    claimed[b as usize] = true;
                }
                if from < top && d > top {
                    return None;
                }
                if d == top {
                    return Some(Walk { written, this_call });
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

/// The caller-frame stack range each recorded constructor call builds, as
/// `(constructor call, offset, size)` in `stack`. `qlst` is the function's call
/// specs, lifted out by `Heritage::guardCalls`.
pub fn constructed_ranges(
    data: &Funcdata,
    qlst: &[FuncCallSpecs],
    stack: &Rc<AddrSpace>,
) -> Vec<(OpId, uintb, int4)> {
    let mut out = Vec::new();
    for site in data.kuna_msvc_byval_sites() {
        let Some(ctor) = site.ctor else { continue };
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
        out.push((ctor, stack.wrap_offset(addr.get_offset().wrapping_add(base)), param.get_size()));
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
/// True for a by-value slot a recorded constructor builds: that slot is an
/// object, not just an argument.
pub fn keeps_slot_mapped(data: &Funcdata, op: OpId, param: int4) -> bool {
    data.kuna_msvc_byval_sites()
        .iter()
        .any(|s| s.call == op && s.param as int4 == param && s.ctor.is_some())
}

#[cfg(test)]
mod tests;
