//! (kuna `swiftcall`) Swift calls: the ones that may throw, the `self` a
//! method hands on, and the `self` a call to a struct member does not pass.
//!
//! A throwing Swift function returns its error in the `swifterror` register
//! (x21 on arm64, r12 on x86-64), which every other convention preserves:
//! the caller zeroes the register, makes the call and tests it, so under the
//! C model the test reads the zero it just stored and the error path folds
//! away. A call whose block writes the constant zero to that register as its
//! last write before the call is such a call, direct or indirect, and is
//! given `__swiftcall_throws`, which hands the register back to the caller
//! ([`throwing_call`]).
//!
//! A struct member takes `self` in `swiftself` only when it is mutating or
//! its value is passed indirectly; a non-mutating member of a loadable struct
//! gets it in the trailing argument registers and leaves `swiftself` alone.
//! Its name does not say which, so a call to one keeps a `swiftself`
//! argument only when the caller set the register for that call: a value
//! that reaches it across an earlier call is that call's, not this one's
//! ([`stale_self`]).
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
use kuna_base::types::{int4, uintb};
use kuna_num::opcodes::OpCode;

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

/// `__swiftcall_throws` when `call` is a throwing Swift call.
pub fn throwing_call(data: &Funcdata, call: OpId) -> Option<Rc<ProtoModel>> {
    let abi = data.get_arch().swift_abi.as_ref()?;
    let (reg, size) = &abi.error_reg;
    let mut cur = data.op_previous_op(call);
    for _ in 0..MAX_SCAN {
        let id = cur?;
        let op = data.obank().get(id)?;
        if matches!(op.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND | OpCode::CPUI_CALLOTHER) {
            return None;
        }
        if let Some(out) = op.get_out().and_then(|o| data.vbank().get(o)) {
            if overlaps(out.get_addr(), out.get_size(), reg, *size) {
                let whole = out.get_addr() == reg && out.get_size() == *size;
                return (whole && zeroes(data, id, 0)).then(|| Rc::clone(&abi.throws));
            }
        }
        cur = data.op_previous_op(id);
    }
    None
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
/// memory, in an image the convention is bound in: a vtable slot, a witness
/// entry or a closure's function, which take it. The model only makes room
/// for `swiftself`; [`indirect_self`] decides whether the call passes it. A
/// trap's target or a coroutine continuation the previous call returned is
/// not a load and keeps the C model.
pub fn indirect_call(data: &Funcdata, call: OpId) -> Option<Rc<ProtoModel>> {
    let abi = data.get_arch().swift_abi.as_ref()?;
    let op = data.obank().get(call)?;
    if op.code() != OpCode::CPUI_CALLIND {
        return None;
    }
    let target = data.vbank().get(op.get_in(0)?)?;
    let (mut addr, mut size) = (target.get_addr().clone(), target.get_size());
    let mut cur = data.op_previous_op(call);
    for _ in 0..MAX_SCAN {
        let id = cur?;
        let prev = data.obank().get(id)?;
        if let Some(out) = prev.get_out().and_then(|o| data.vbank().get(o)) {
            if overlaps(out.get_addr(), out.get_size(), &addr, size) {
                match prev.code() {
                    OpCode::CPUI_LOAD => return Some(Rc::clone(&abi.call)),
                    OpCode::CPUI_COPY => {
                        let src = data.vbank().get(prev.get_in(0)?)?;
                        if src.is_constant() {
                            return None;
                        }
                        (addr, size) = (src.get_addr().clone(), src.get_size());
                    }
                    _ => return None,
                }
            }
        }
        cur = data.op_previous_op(id);
    }
    None
}

/// How many dominating blocks [`call_since`] walks.
const MAX_BLOCKS: usize = 64;

/// Is there a call on the dominator path from `def` (the function entry when
/// `None`) down to the call `op`?
fn call_since(data: &Funcdata, def: Option<OpId>, op: OpId) -> bool {
    let Some(mut bl) = data.obank().get(op).and_then(|o| o.get_parent()) else { return false };
    let mut cur = data.op_previous_op(op);
    for _ in 0..MAX_BLOCKS {
        while let Some(id) = cur {
            if Some(id) == def {
                return false;
            }
            let Some(o) = data.obank().get(id) else { return false };
            if matches!(o.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND) {
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
