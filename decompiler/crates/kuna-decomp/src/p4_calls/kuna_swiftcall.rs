//! (kuna `swiftcall`) Swift calls: the ones that may throw, the `self` a
//! method hands on, and the `self` a call to a struct member does not pass.
//!
//! A throwing Swift function returns its error in the `swifterror` register
//! (x21 on arm64, r12 on x86-64), which every other convention preserves:
//! the caller zeroes the register, makes the call and tests it, so under the
//! C model the test reads the zero it just stored and the error path folds
//! away. A call whose block writes the constant zero to that register as its
//! last write before the call, and whose next read of the register compares
//! it with zero, is such a call, direct or indirect, and is given
//! `__swiftcall_throws`, which hands the register back to the caller
//! ([`throwing_call`]). Without that test the register is an ordinary
//! callee-saved variable (a loop counter a C or Swift function zeroes, then
//! reads back across the call) and the call leaves it alone.
//!
//! A struct member takes `self` in `swiftself` only when it is mutating or
//! its value is passed indirectly; a non-mutating member of a loadable struct
//! gets it in the trailing argument registers and leaves `swiftself` alone.
//! Its name does not say which, so a call to one keeps a `swiftself`
//! argument only when the caller set the register for that call: a value
//! that reaches it across an earlier call is that call's, not this one's,
//! unless that call is to a C function (a runtime helper), which preserves
//! the register and reads none of it, to a body that does not read it, or to
//! a Swift function of a module rather than a member ([`stale_self`],
//! [`seed_self_readers`], [`SwiftAbi::no_self`]).
//!
//! A method hands its own `self` on to the methods it calls without writing
//! the register again; that value is the caller's input in a preserved
//! register, which ancestor scoring takes for no argument, so it is passed
//! explicitly ([`passes_own_self`]). An indirect call through a target its
//! block loads takes `__swiftcall` ([`indirect_call`]), and passes
//! `swiftself` exactly when it dispatches on the value the register holds,
//! through that value's isa ([`indirect_self`]).
//!
//! Inert unless the Swift metadata pass bound the convention in this image.
//!
//! The two models are derived from the compiler spec's own default model
//! when the spec declares neither ([`derive_model`]), so any AArch64 or
//! x86-64 spec carries them, including one a Ghidra host sends: `__swiftcall`
//! is the default with `swiftself` added as an input of storage class
//! `class1`, its own resource section, so it is a parameter exactly when the
//! body reads it and never forces the argument registers around it;
//! `__swiftcall_throws` also moves the error register from the preserved
//! registers to the killed ones.

use std::collections::BTreeSet;
use std::rc::Rc;

use kuna_base::xml::Element;

use kuna_base::address::Address;
use kuna_base::space::{AddrSpace, spacetype};
use kuna_base::types::{int4, uintb};
use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;
use kuna_sleigh::translate::{PcodeEmit, Translate};

use crate::context::OpId;
use crate::fspec::ProtoModel;
use crate::funcdata::Funcdata;

/// The Swift registers of the image and the model that returns the error.
#[derive(Debug)]
pub struct SwiftAbi {
    pub self_reg: (Address, int4),
    pub error_reg: (Address, int4),
    pub call: Rc<ProtoModel>,
    pub throws: Rc<ProtoModel>,
    /// The bound callees that may or may not take `swiftself`, by entry
    /// `(space index, offset)`.
    pub maybe_self: BTreeSet<(int4, uintb)>,
    /// The Swift functions a symbol names as a module's, not a type's
    /// members: they take no `swiftself` and preserve it.
    pub no_self: BTreeSet<(int4, uintb)>,
}

fn overlaps(a: &Address, asize: int4, b: &Address, bsize: int4) -> bool {
    a.overlap(0, b, bsize) >= 0 || b.overlap(0, a, asize) >= 0
}

/// The names of the two models.
pub const SWIFTCALL: &str = "__swiftcall";
pub const SWIFTCALL_THROWS: &str = "__swiftcall_throws";

/// How many ops before the call the zeroing store is looked for.
const MAX_SCAN: usize = 64;

/// Is the op at `id` a write of zero to all of `(addr, size)`: a constant
/// copy, an `xor` of the register with itself, or the zero extension of one
/// of those into it?
fn zeroes(data: &Funcdata, id: OpId, depth: u32) -> bool {
    let Some(op) = data.obank().get(id) else { return false };
    let input = |i: i32| op.get_in(i).and_then(|v| data.vbank().get(v));
    match op.code() {
        OpCode::CPUI_COPY => input(0).is_some_and(|v| v.is_constant() && v.get_offset() == 0),
        OpCode::CPUI_INT_XOR => match (input(0), input(1)) {
            (Some(a), Some(b)) => !a.is_constant() && a.get_addr() == b.get_addr() && a.get_size() == b.get_size(),
            _ => false,
        },
        OpCode::CPUI_INT_ZEXT if depth == 0 => {
            let Some(low) = input(0) else { return false };
            let (addr, size) = (low.get_addr().clone(), low.get_size());
            let mut cur = data.op_previous_op(id);
            while let Some(prev) = cur {
                let written = data.obank().get(prev).and_then(|o| o.get_out()).and_then(|o| data.vbank().get(o));
                if let Some(out) = written {
                    if overlaps(out.get_addr(), out.get_size(), &addr, size) {
                        return out.get_addr() == &addr && out.get_size() == size && zeroes(data, prev, 1);
                    }
                }
                cur = data.op_previous_op(prev);
            }
            false
        }
        _ => false,
    }
}

/// `__swiftcall_throws` when `call` is a throwing Swift call: the error
/// register is zeroed just before it and compared with zero after it.
pub fn throwing_call(data: &Funcdata, call: OpId) -> Option<Rc<ProtoModel>> {
    let abi = data.get_arch().swift_abi.as_ref()?;
    let (reg, size) = &abi.error_reg;
    (zeroed_before(data, call, reg, *size) && tested_after(data, call, reg, *size))
        .then(|| Rc::clone(&abi.throws))
}

/// Is `(reg, size)` written with zero as the last write before `call`?
fn zeroed_before(data: &Funcdata, call: OpId, reg: &Address, size: int4) -> bool {
    let mut cur = data.op_previous_op(call);
    for _ in 0..MAX_SCAN {
        let Some(id) = cur else { return false };
        let Some(op) = data.obank().get(id) else { return false };
        if matches!(op.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND | OpCode::CPUI_CALLOTHER) {
            return false;
        }
        if let Some(out) = op.get_out().and_then(|o| data.vbank().get(o)) {
            if overlaps(out.get_addr(), out.get_size(), reg, size) {
                return out.get_addr() == reg && out.get_size() == size && zeroes(data, id, 0);
            }
        }
        cur = data.op_previous_op(id);
    }
    false
}

/// How many places the error register's value is followed through after a
/// call: the register itself, the copies made of it and the stack slots it is
/// spilled to before the test.
const MAX_TRACKED: usize = 4;

/// Is `(addr, size)` within one of `set`'s places?
fn within(set: &[(Address, int4)], addr: &Address, size: int4) -> bool {
    set.iter().any(|(a, w)| overlaps(addr, size, a, *w))
}

/// The `(base register, offset)` an op computes into its output: the
/// register itself for a copy of one, `base + c` for an add or subtract of a
/// constant (`rsp + 8`, `x29 - 0x18`); `None` for anything else.
fn address_of(code: OpCode, inputs: &[&crate::varnode::Varnode]) -> Option<(Address, int4, uintb)> {
    let register = |v: &crate::varnode::Varnode| {
        !v.is_constant() && v.get_addr().get_space().is_some_and(|s| s.get_type() == kuna_base::space::spacetype::IPTR_PROCESSOR)
    };
    let (base, off) = match (code, inputs) {
        (OpCode::CPUI_COPY, [a]) => (*a, 0),
        (OpCode::CPUI_INT_ADD, [a, b]) if a.is_constant() => (*b, a.get_offset()),
        (OpCode::CPUI_INT_ADD, [a, b]) if b.is_constant() => (*a, b.get_offset()),
        (OpCode::CPUI_INT_SUB, [a, b]) if b.is_constant() => (*a, b.get_offset().wrapping_neg()),
        _ => return None,
    };
    register(base).then(|| (base.get_addr().clone(), base.get_size(), off))
}

/// Does the code after `call` examine `(reg, size)` the way a caller of a
/// throwing function does: its next read compares it with zero? The read may
/// be of the register or of a copy of it (an arm64 debug build copies x21 and
/// stores the copy, then tests x21 itself), or of a value reloaded from a
/// stack slot it was spilled to while the slot's base register is unchanged;
/// any other store of either is passed over, and the zero may be a temporary
/// the instruction loaded first (arm64 `cmp x21, #0`).
fn tested_after(data: &Funcdata, call: OpId, reg: &Address, size: int4) -> bool {
    let mut tracked = vec![(reg.clone(), size)];
    let mut slots: Vec<(Address, int4, uintb)> = Vec::new();
    let mut pointers: Vec<(Address, int4, (Address, int4, uintb))> = Vec::new();
    let mut zeros: Vec<(Address, int4)> = Vec::new();
    let mut cur = data.op_next_op(call);
    for _ in 0..MAX_SCAN {
        let Some(id) = cur else { return false };
        let Some(op) = data.obank().get(id) else { return false };
        let inputs: Vec<_> = (0..op.num_input())
            .filter_map(|i| op.get_in(i).and_then(|v| data.vbank().get(v)))
            .collect();
        let out = op
            .get_out()
            .and_then(|o| data.vbank().get(o))
            .map(|o| (o.get_addr().clone(), o.get_size()));
        let slot = match (op.code(), inputs.get(1)) {
            (OpCode::CPUI_LOAD | OpCode::CPUI_STORE, Some(p)) if !p.is_constant() => {
                pointers.iter().find(|(a, w, _)| a == p.get_addr() && *w == p.get_size()).map(|(_, _, s)| s.clone()).or_else(
                    || p.get_addr().get_space().is_some_and(|s| s.get_type() == kuna_base::space::spacetype::IPTR_PROCESSOR)
                        .then(|| (p.get_addr().clone(), p.get_size(), 0)),
                )
            }
            _ => None,
        };
        if let Some((addr, width)) = &out {
            pointers.retain(|(a, w, (b, bw, _))| !overlaps(addr, *width, a, *w) && !overlaps(addr, *width, b, *bw));
            if let Some(target) = address_of(op.code(), &inputs) {
                pointers.push((addr.clone(), *width, target));
            }
        }
        let read = inputs.iter().any(|v| !v.is_constant() && within(&tracked, v.get_addr(), v.get_size()));
        if read {
            match op.code() {
                OpCode::CPUI_INT_EQUAL
                | OpCode::CPUI_INT_NOTEQUAL
                | OpCode::CPUI_INT_LESS
                | OpCode::CPUI_INT_LESSEQUAL
                | OpCode::CPUI_INT_SLESS
                | OpCode::CPUI_INT_SLESSEQUAL
                | OpCode::CPUI_INT_CARRY
                | OpCode::CPUI_INT_SCARRY
                | OpCode::CPUI_INT_SBORROW => {
                    return inputs.iter().any(|v| {
                        (v.is_constant() && v.get_offset() == 0)
                            || (!v.is_constant() && within(&zeros, v.get_addr(), v.get_size()))
                    });
                }
                OpCode::CPUI_INT_AND => {
                    return inputs.len() == 2 && inputs[0].get_addr() == inputs[1].get_addr();
                }
                OpCode::CPUI_COPY if tracked.len() + slots.len() < MAX_TRACKED => {
                    let Some(place) = out.clone() else { return false };
                    tracked.push(place);
                }
                OpCode::CPUI_STORE => {
                    let value = inputs.get(2).is_some_and(|v| within(&tracked, v.get_addr(), v.get_size()));
                    if let Some(slot) = slot.filter(|_| value && tracked.len() + slots.len() < MAX_TRACKED) {
                        slots.push(slot);
                    }
                }
                _ => return false,
            }
        } else {
            if let Some(slot) = &slot {
                if op.code() == OpCode::CPUI_STORE {
                    slots.retain(|s| s != slot);
                }
            }
            if let Some((addr, width)) = &out {
                tracked.retain(|(a, w)| !overlaps(addr, *width, a, *w));
                slots.retain(|(a, w, _)| !overlaps(addr, *width, a, *w));
                if op.code() == OpCode::CPUI_LOAD && slot.as_ref().is_some_and(|s| slots.contains(s)) {
                    tracked.push((addr.clone(), *width));
                }
                if tracked.is_empty() && slots.is_empty() {
                    return false;
                }
            }
        }
        if let Some((addr, width)) = out {
            zeros.retain(|(a, w)| !overlaps(&addr, width, a, *w));
            if !read && zeroes(data, id, 1) {
                zeros.push((addr, width));
            }
        }
        cur = data.op_next_op(id);
    }
    false
}

/// Is the `swiftself` trial of call `idx` (input `slot`, in `addr`/`size`) a
/// value the caller did not set for a callee that may not take it?
pub fn stale_self(data: &Funcdata, idx: int4, slot: int4, addr: &Address, size: int4) -> bool {
    let Some(abi) = data.get_arch().swift_abi.as_ref() else { return false };
    if addr != &abi.self_reg.0 || size != abi.self_reg.1 {
        return false;
    }
    let call = data.get_call_specs(idx);
    let entry = call.get_entry_address();
    let Some(sp) = entry.get_space() else { return false };
    if !abi.maybe_self.contains(&(sp.get_index(), entry.get_offset())) {
        return false;
    }
    let op = call.get_op();
    let Some(vn) = data.obank().get(op).and_then(|o| o.get_in(slot)) else { return false };
    let def = data.vbank().get(vn).and_then(|v| v.get_def());
    if def.and_then(|d| data.obank().get(d)).is_some_and(|d| d.code() == OpCode::CPUI_INDIRECT) {
        return true;
    }
    call_since(data, def, op)
}

/// Is `(addr, size)` the image's `swiftself` register?
pub fn is_self_storage(data: &Funcdata, addr: &Address, size: int4) -> bool {
    data.get_arch().swift_abi.as_ref().is_some_and(|abi| &abi.self_reg.0 == addr && abi.self_reg.1 == size)
}

/// Is the model one of the two Swift models?
fn is_swift(model: &ProtoModel) -> bool {
    matches!(model.get_name(), SWIFTCALL | SWIFTCALL_THROWS)
}

/// Is the `swiftself` trial of call `idx` (input `slot`, in `addr`/`size`) the
/// caller's own `self`, handed on untouched or through copies (a value saved
/// in another callee-saved register and moved back)? A Swift method decompiled under
/// the convention passes the `self` it received to a callee that takes one
/// without writing the register again; its input is preserved by every call,
/// so it reads as no argument at all. [`stale_self`] has already refused it
/// for a callee that may not take `self`. A direct call only; an indirect
/// one is [`indirect_self`]'s.
pub fn passes_own_self(data: &Funcdata, idx: int4, slot: int4, addr: &Address, size: int4) -> bool {
    let Some(abi) = data.get_arch().swift_abi.as_ref() else { return false };
    if addr != &abi.self_reg.0 || size != abi.self_reg.1 {
        return false;
    }
    let own = data.get_func_proto();
    let call = data.get_call_specs(idx);
    if !own.has_model() || !is_swift(own.model()) || !call.proto().has_model() || !is_swift(call.proto().model()) {
        return false;
    }
    let op = call.get_op();
    let Some(vn) = data.obank().get(op).and_then(|o| o.get_in(slot)) else { return false };
    let root = copy_root(data, vn);
    let own = data.vbank().get(root).is_some_and(|v| v.is_input() && v.get_addr() == addr && v.get_size() == size);
    own && data.obank().get(op).is_some_and(|o| o.code() == OpCode::CPUI_CALL)
}

/// The Varnode `vn` copies, through any chain of copies.
fn copy_root(data: &Funcdata, mut vn: crate::context::VarnodeId) -> crate::context::VarnodeId {
    for _ in 0..8 {
        let def = data.vbank().get(vn).and_then(|v| v.get_def()).and_then(|d| data.obank().get(d));
        match def {
            Some(d) if d.code() == OpCode::CPUI_COPY => match d.get_in(0) {
                Some(a) => vn = a,
                None => break,
            },
            _ => break,
        }
    }
    vn
}

/// The `swiftself` trial of the indirect call `idx`, when the call is under
/// a Swift model: an argument exactly when the call dispatches on the value
/// the register holds, through that value's own isa (`obj->isa[k](obj, …)`).
/// A value witness, a witness-table entry or a closure reached any other way
/// is not told what `swiftself` holds, so the register is not its argument.
pub fn indirect_self(data: &Funcdata, idx: int4, slot: int4, addr: &Address, size: int4) -> Option<bool> {
    let abi = data.get_arch().swift_abi.as_ref()?;
    if addr != &abi.self_reg.0 || size != abi.self_reg.1 {
        return None;
    }
    let call = data.get_call_specs(idx);
    let op = call.get_op();
    if !data.obank().get(op).is_some_and(|o| o.code() == OpCode::CPUI_CALLIND)
        || !call.proto().has_model()
        || !is_swift(call.proto().model())
    {
        return None;
    }
    let vn = data.obank().get(op).and_then(|o| o.get_in(slot))?;
    Some(dispatches_on(data, op, vn))
}

/// Is the target of the indirect call `op` a slot loaded through `obj`'s
/// own first word, its isa: `obj->isa[k]`, a method dispatched on `obj`?
fn dispatches_on(data: &Funcdata, op: OpId, obj: crate::context::VarnodeId) -> bool {
    let def = |v: crate::context::VarnodeId| {
        data.vbank().get(v).and_then(|v| v.get_def()).and_then(|d| data.obank().get(d))
    };
    let root = |v| copy_root(data, v);
    let obj = root(obj);
    let mut v = match data.obank().get(op).and_then(|o| o.get_in(0)) {
        Some(v) => v,
        None => return false,
    };
    let mut loads = 0;
    for _ in 0..8 {
        if loads == 2 {
            return root(v) == obj;
        }
        let Some(d) = def(v) else { return false };
        v = match d.code() {
            OpCode::CPUI_LOAD => {
                loads += 1;
                match d.get_in(1) {
                    Some(a) => a,
                    None => return false,
                }
            }
            OpCode::CPUI_COPY | OpCode::CPUI_INT_ADD | OpCode::CPUI_PTRADD | OpCode::CPUI_PTRSUB => {
                match d.get_in(0) {
                    Some(a) => a,
                    None => return false,
                }
            }
            _ => return false,
        };
    }
    false
}

/// `__swiftcall` for an indirect call whose target its block loads from
/// memory, in an image the convention is bound in, when the call shows it is
/// Swift's: the slot is reached through another loaded pointer (an isa's
/// vtable, a witness table read out of a container), or the caller sets
/// `swiftself` for it (a method's `self`, a witness's, a closure's context).
/// The model only makes room for `swiftself`; [`indirect_self`] decides
/// whether the call passes it. A C function pointer loaded from a global or
/// a field of a pointer the caller holds, with `swiftself` left alone, keeps
/// the C model, as does a trap's target or a coroutine continuation the
/// previous call returned, which is not a load.
pub fn indirect_call(data: &Funcdata, call: OpId) -> Option<Rc<ProtoModel>> {
    let abi = data.get_arch().swift_abi.as_ref()?;
    let op = data.obank().get(call)?;
    if op.code() != OpCode::CPUI_CALLIND {
        return None;
    }
    let target = data.vbank().get(op.get_in(0)?)?;
    let load = defining_op(data, call, target.get_addr().clone(), target.get_size(), &[OpCode::CPUI_COPY])?;
    let slot = data.obank().get(load).filter(|o| o.code() == OpCode::CPUI_LOAD)?;
    let ptr = data.vbank().get(slot.get_in(1)?)?;
    let through_memory = !ptr.is_constant()
        && defining_op(
            data,
            load,
            ptr.get_addr().clone(),
            ptr.get_size(),
            &[OpCode::CPUI_COPY, OpCode::CPUI_INT_ADD, OpCode::CPUI_PTRADD, OpCode::CPUI_PTRSUB],
        )
        .and_then(|d| data.obank().get(d))
        .is_some_and(|d| d.code() == OpCode::CPUI_LOAD);
    (through_memory || sets_self(data, call, abi)).then(|| Rc::clone(&abi.call))
}

/// The op before `from` in its block that last wrote `(addr, size)`, after
/// following back through any of `through` (each to its first non-constant
/// input), or `None` when another op writes it first or nothing does.
fn defining_op(data: &Funcdata, from: OpId, mut addr: Address, mut size: int4, through: &[OpCode]) -> Option<OpId> {
    let mut cur = data.op_previous_op(from);
    for _ in 0..MAX_SCAN {
        let id = cur?;
        let prev = data.obank().get(id)?;
        if let Some(out) = prev.get_out().and_then(|o| data.vbank().get(o)) {
            if overlaps(out.get_addr(), out.get_size(), &addr, size) {
                if !through.contains(&prev.code()) {
                    return Some(id);
                }
                let src = (0..prev.num_input())
                    .filter_map(|i| prev.get_in(i).and_then(|v| data.vbank().get(v)))
                    .find(|v| !v.is_constant())?;
                (addr, size) = (src.get_addr().clone(), src.get_size());
            }
        }
        cur = data.op_previous_op(id);
    }
    None
}

/// Does the code before `call`, back to the previous call, write `swiftself`?
fn sets_self(data: &Funcdata, call: OpId, abi: &SwiftAbi) -> bool {
    let (reg, size) = &abi.self_reg;
    let mut cur = data.op_previous_op(call);
    for _ in 0..MAX_SCAN {
        let Some(id) = cur else { return false };
        let Some(op) = data.obank().get(id) else { return false };
        if matches!(op.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND | OpCode::CPUI_CALLOTHER) {
            return false;
        }
        if op.get_out().and_then(|o| data.vbank().get(o)).is_some_and(|o| overlaps(o.get_addr(), o.get_size(), reg, *size)) {
            return true;
        }
        cur = data.op_previous_op(id);
    }
    false
}

/// How many dominating blocks [`call_since`] walks.
const MAX_BLOCKS: usize = 64;

/// Can the call `id` take `swiftself`? Not when it is under a C model and
/// its callee is named as a C function (`swift_retain`, `objc_msgSend`, a
/// `@_cdecl` export): one preserves the register and reads none of it; nor
/// when its body was seen not to read the register ([`seed_self_readers`]),
/// or a symbol names it a Swift function of a module rather than a member
/// ([`SwiftAbi::no_self`]). Any other Swift function, unnamed one or
/// indirect target may.
fn may_take_self(data: &Funcdata, id: OpId) -> bool {
    let Some(i) = data.get_call_specs_index(id) else { return true };
    let call = data.get_call_specs(i);
    let entry = call.get_entry_address();
    if data.kuna_swift_self_unread(entry) {
        return false;
    }
    let free = data.get_arch().swift_abi.as_ref().is_some_and(|abi| {
        entry.get_space().is_some_and(|sp| abi.no_self.contains(&(sp.get_index(), entry.get_offset())))
    });
    if free {
        return false;
    }
    let proto = call.proto();
    (proto.has_model() && is_swift(proto.model())) || counts_as_swift(call.get_name())
}

/// Is there a call that may take `swiftself` ([`may_take_self`]) on the dominator path from
/// `def` (the function entry when `None`) down to the call `op`?
fn call_since(data: &Funcdata, def: Option<OpId>, op: OpId) -> bool {
    let Some(mut bl) = data.obank().get(op).and_then(|o| o.get_parent()) else { return false };
    let mut cur = data.op_previous_op(op);
    for _ in 0..MAX_BLOCKS {
        while let Some(id) = cur {
            if Some(id) == def {
                return false;
            }
            let Some(o) = data.obank().get(id) else { return false };
            if matches!(o.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND) && may_take_self(data, id) {
                return true;
            }
            cur = data.op_previous_op(id);
        }
        let Some(up) = data.bblocks_ref().block(bl).get_immed_dom() else { return false };
        bl = up;
        cur = data.bb_op_tail(bl);
    }
    false
}

/// Whether the body at `f` reads the register `reg` before it writes it:
/// `Some(true)` on a read first (a `self` that arrives there), `Some(false)`
/// on a write first (the register is scratch), and `None` when the first
/// sixty-four instructions leave it alone; and whether that verdict was
/// reached on the one path from the entry: before any branch or call out of
/// the straight line, and, for `None`, at a return rather than the scan
/// limit. The scan is linear, so a store of the register through the stack
/// pointer, or through an address the same instruction computes from it,
/// does not count as a read, nor a load into it as a write; a store through
/// any other address does (it hands the value on), as does the copy into a
/// temporary a `push` or `stp` makes first. Decodes through `translate`, so
/// its load image must be attached.
fn first_touch(translate: &dyn Translate, code_space: &Rc<AddrSpace>, reg: &str, f: u64) -> (Option<bool>, bool) {
    struct Ops(Vec<(OpCode, Option<VarnodeData>, Vec<VarnodeData>)>);
    impl PcodeEmit for Ops {
        fn dump(&mut self, _: &Address, opc: OpCode, out: Option<&VarnodeData>, vars: &[VarnodeData]) {
            self.0.push((opc, out.cloned(), vars.to_vec()));
        }
    }
    let Ok(want) = translate.get_register_data(reg) else { return (None, false) };
    let sp = translate.get_register_data("RSP").or_else(|_| translate.get_register_data("sp")).ok();
    let index = |v: &VarnodeData| v.space.as_ref().map(|s| s.get_index());
    let same = |v: &VarnodeData| {
        index(v) == index(&want)
            && v.offset < want.offset + u64::from(want.size)
            && want.offset < v.offset + u64::from(v.size)
    };
    let is_sp = |v: &VarnodeData| sp.as_ref().is_some_and(|sp| index(v) == index(sp) && v.offset == sp.offset);
    let mut vma = f;
    for _ in 0..64 {
        let mut ops = Ops(Vec::new());
        let Ok(len) = translate.one_instruction(&mut ops, &Address::new(Rc::clone(code_space), vma)) else {
            return (None, false);
        };
        let mut sp_temps: Vec<VarnodeData> = Vec::new();
        for (opc, out, vars) in &ops.0 {
            match opc {
                OpCode::CPUI_RETURN => return (None, true),
                OpCode::CPUI_BRANCH | OpCode::CPUI_CBRANCH
                    if vars.first().and_then(|v| v.space.as_ref()).is_some_and(|s| s.get_type() == spacetype::IPTR_CONSTANT) => {}
                OpCode::CPUI_BRANCH | OpCode::CPUI_CBRANCH | OpCode::CPUI_BRANCHIND | OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => {
                    return (None, false);
                }
                _ => {}
            }
            let from_sp = |p: &VarnodeData| is_sp(p) || sp_temps.iter().any(|t| index(t) == index(p) && t.offset == p.offset);
            let spill = *opc == OpCode::CPUI_STORE && vars.get(2).is_some_and(same) && vars.get(1).is_some_and(from_sp);
            let reload = *opc == OpCode::CPUI_LOAD && out.as_ref().is_some_and(same);
            if spill || reload {
                continue;
            }
            if vars.iter().any(same) {
                return (Some(true), true);
            }
            if out.as_ref().is_some_and(same) {
                return (Some(false), true);
            }
            let derived = matches!(opc, OpCode::CPUI_COPY | OpCode::CPUI_INT_ADD | OpCode::CPUI_INT_SUB)
                && vars.first().is_some_and(from_sp);
            if let Some(o) = out.as_ref().filter(|o| derived && o.space.as_ref().is_some_and(|s| s.get_type() == spacetype::IPTR_INTERNAL)) {
                sp_temps.push(o.clone());
            }
        }
        let Some(next) = u64::try_from(len).ok().and_then(|l| vma.checked_add(l)) else { return (None, false) };
        vma = next;
    }
    (None, false)
}

/// Record, for each direct callee [`may_take_self`] would otherwise count (a
/// Swift function or an unnamed one), a body that does not take `swiftself`: on its one straight path from the
/// entry it writes the register first or returns without touching it. The
/// per-function `ArchContext` carries no translator, so the bodies are read
/// here, once per image. Inert outside an image the convention is bound in.
pub fn seed_self_readers(arch: &mut crate::architecture::Architecture, data: &mut Funcdata) {
    let Some(abi) = arch.loader_swift_abi.clone() else { return };
    if abi.maybe_self.is_empty() || data.num_calls() < 2 {
        return;
    }
    let Some(space) = abi.self_reg.0.get_space().cloned() else { return };
    let reg = arch.translate().get_register_name(&space, abi.self_reg.0.get_offset(), abi.self_reg.1);
    if reg.is_empty() {
        return;
    }
    for i in 0..data.num_calls() {
        let call = data.get_call_specs(i);
        let entry = call.get_entry_address().clone();
        let Some(code) = entry.get_space().cloned().filter(|_| !entry.is_invalid()) else { continue };
        if !(call.proto().has_model() && is_swift(call.proto().model())) && !counts_as_swift(call.get_name()) {
            continue;
        }
        let key = (code.get_index(), entry.get_offset());
        let unread = match arch.kuna_swift_self_first_cache.get(&key) {
            Some(&v) => v,
            None => {
                let (first, straight) = first_touch(arch.translate(), &code, &reg, entry.get_offset());
                let v = straight && first != Some(true);
                arch.kuna_swift_self_first_cache.insert(key, v);
                v
            }
        };
        if unread {
            data.kuna_set_swift_self_unread(&entry);
        }
    }
}

/// Is `name` one a Swift function may have: not a C identifier, or the
/// engine's placeholder for an unnamed one?
fn counts_as_swift(name: &str) -> bool {
    let c_name = !name.is_empty() && name.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_');
    !c_name || name.starts_with("sub_") || name.starts_with("FUN_")
}

/// The `swiftself` and `swifterror` register names of a language Swift
/// targets: AArch64's x20 / x21 and x86-64's r13 / r12.
pub fn swift_registers(has_register: impl Fn(&str) -> Option<int4>) -> Option<(&'static str, &'static str)> {
    [("x20", "x21"), ("R13", "R12")]
        .into_iter()
        .find(|(s, e)| has_register(s) == Some(8) && has_register(e) == Some(8))
}

fn register(name: &str) -> Rc<Element> {
    let mut r = Element::new();
    r.set_name("register");
    r.add_attribute("name", name.as_bytes());
    Rc::new(r)
}

fn shell(from: &Element) -> Element {
    let mut el = Element::new();
    el.set_name(from.get_name());
    for i in 0..from.get_num_attributes() {
        el.add_attribute(from.get_attribute_name(i), from.get_attribute_value_at(i));
    }
    el
}

/// The `<prototype>` named `name`: the default model `proto` with
/// `self_reg` as a `class1` input ahead of every other, and with `error_reg`
/// killed by the call rather than preserved across it.
pub fn derive_model(proto: &Element, name: &str, self_reg: &str, error_reg: Option<&str>) -> Element {
    let mut out = Element::new();
    out.set_name("prototype");
    for i in 0..proto.get_num_attributes() {
        let attr = proto.get_attribute_name(i);
        let value = if attr == "name" { name.as_bytes() } else { proto.get_attribute_value_at(i) };
        out.add_attribute(attr, value);
    }
    let names = |el: &Element, reg: &str| {
        el.get_attribute_value("name").is_ok_and(|n| n.eq_ignore_ascii_case(reg.as_bytes()))
    };
    for child in proto.get_children() {
        match (child.get_name(), error_reg) {
            ("input", _) => {
                let mut input = shell(child);
                let mut entry = Element::new();
                entry.set_name("pentry");
                entry.add_attribute("minsize", b"1");
                entry.add_attribute("maxsize", b"8");
                entry.add_attribute("storage", b"class1");
                entry.add_child(register(self_reg));
                input.add_child(Rc::new(entry));
                for c in child.get_children() {
                    input.add_child(Rc::clone(c));
                }
                out.add_child(Rc::new(input));
            }
            ("unaffected", Some(err)) => {
                let mut list = shell(child);
                for c in child.get_children().iter().filter(|c| !names(c, err)) {
                    list.add_child(Rc::clone(c));
                }
                out.add_child(Rc::new(list));
            }
            ("killedbycall", Some(err)) => {
                let mut list = shell(child);
                list.add_child(register(err));
                for c in child.get_children() {
                    list.add_child(Rc::clone(c));
                }
                out.add_child(Rc::new(list));
            }
            _ => out.add_child(Rc::clone(child)),
        }
    }
    if let Some(err) = error_reg {
        if !proto.get_children().iter().any(|c| c.get_name() == "killedbycall") {
            let mut list = Element::new();
            list.set_name("killedbycall");
            list.add_child(register(err));
            out.add_child(Rc::new(list));
        }
    }
    out
}
