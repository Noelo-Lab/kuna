//! (kuna `callerptr`) A parameter every caller passes a pointer is a pointer.
//!
//! `calleevote` states a callee parameter's type only where every direct call
//! passes the same committed pointer: a record with a layout, a `char *`, a
//! `char **`. A caller that holds a pointer to nothing in particular -- a
//! `void *` from an allocator, the `undefined1 *` an untyped buffer gets, the
//! address of a frame object -- or that passes a null at one of the sites,
//! states nothing, so a callee that only forwards, stores or compares the value
//! keeps the pointer-width integer its own body recovered (`long a1` where every
//! caller passes a string, `unsigned long a1` for a hash-table entry its callers
//! hand over as two different records).
//!
//! Under this option such a parameter is a pointer when every recorded call
//! passes, in exactly the storage the callee recovered the parameter in, either
//! a value its caller typed as a pointer of the parameter's width or the
//! constant zero, and at least one call passes a pointer. It takes the callers'
//! type where every pointer they pass is the same type, and `void *` where they
//! differ. The statement is `calleevote`'s: it is decided over the same closed
//! caller set and taken through the same vote, so the callee's own uses still
//! refuse it (`kuna_protoorder::input_refuses` -- an integer operation on the
//! value, a load or store the pointee does not cover, a family member that is
//! declared, global or frame memory).

use std::collections::VecDeque;
use crate::kuna_fasthash::FxHashSet;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_num::opcodes::OpCode;

use crate::context::{OpId, VarnodeId};
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;
use crate::infra::architecture::Architecture;
use crate::kuna_calleevote::{committed, same_type, trace, CallSite, Typed};

/// How far [`dereferenced`] follows a value through copies and literal offsets.
const HOP_CAP: usize = 10;

/// The target's `void *`, or `None` when nothing is decided: the option or
/// `calleevote` is off.
pub fn void_pointer(arch: &Architecture) -> Option<Rc<Datatype>> {
    if !arch.caller_ptr || !arch.calleevote.is_on() {
        return None;
    }
    let tlst = arch.types();
    let spc = arch.manage().get_default_data_space()?;
    let void = tlst.get_type_void().ok()?;
    tlst.get_type_pointer(tlst.get_size_of_pointer(), void, spc.get_word_size()).ok()
}

/// What the callers state about the callee parameter `p` at position `j`, or
/// `None`: `p` already says it is a pointer, a call passes something other than
/// a pointer or a null in `p`'s storage, or every call passes a null.
pub fn pointer_vote(sites: &[&CallSite], j: usize, p: &Typed, void_ptr: &Rc<Datatype>) -> Option<Typed> {
    if p.ct.get_metatype() == type_metatype::TYPE_PTR || p.size != void_ptr.get_size() {
        return None;
    }
    let mut passed: Vec<&Typed> = Vec::new();
    for s in sites {
        let a = s.args.get(j)?.as_ref()?;
        if a.addr != p.addr || a.size != p.size {
            return None;
        }
        if a.null {
            continue;
        }
        if a.ct.get_metatype() != type_metatype::TYPE_PTR || a.ct.get_size() != p.size {
            return None;
        }
        passed.push(a);
    }
    if passed.iter().any(|a| !a.proven) {
        return None;
    }
    let first = passed.first()?;
    let agreed = committed(&first.ct) && passed.iter().all(|a| same_type(&a.ct, &first.ct));
    let (ct, frame) = if agreed {
        (Rc::clone(&first.ct), passed.iter().any(|a| a.frame))
    } else {
        (Rc::clone(void_ptr), false)
    };
    if trace() {
        eprintln!(
            "[callerptr] param {j} @{:x}:{} -> {} ({} callers)",
            p.addr.get_offset(),
            p.size,
            if agreed { "callers' type" } else { "void *" },
            sites.len()
        );
    }
    Some(Typed { addr: p.addr.clone(), size: p.size, ct, frame, null: false, proven: false, by_class: true })
}

/// Does anything the callee does with its input `vn` outrank the callers'
/// pointer `ct`? `calleevote`'s refusals ([`crate::kuna_protoorder::input_refuses`]),
/// except that a call the value is handed to speaks only through a declared
/// prototype: another recovery's integer for the same slot says no more than
/// "a pointer-width value", and a function that converts the pointer it holds
/// to a number for a callee still holds a pointer. An address the callee
/// derives from the value by a literal offset and then masks, shifts, divides
/// or multiplies refuses it too (`(p + 7) & ~7`, an alignment helper over a
/// `uintptr_t`).
pub(crate) fn refuses(data: &Funcdata, vn: VarnodeId, ct: &Datatype) -> bool {
    crate::kuna_protoorder::input_refuses_declared(data, vn, ct)
        || masks_a_derived_address(data, &crate::kuna_protoorder::value_family(data, vn))
}

/// Does the callee derive an address from a member of `family` by a literal
/// offset and then treat it as a number: mask, shift, divide or multiply it?
fn masks_a_derived_address(data: &Funcdata, family: &[VarnodeId]) -> bool {
    let constant = |v: Option<VarnodeId>| v.and_then(|v| data.vbank().get(v)).is_some_and(|c| c.is_constant());
    let mut seen: FxHashSet<VarnodeId> = family.iter().copied().collect();
    let mut work: VecDeque<(VarnodeId, usize)> = family.iter().map(|&v| (v, 0)).collect();
    while let Some((v, hops)) = work.pop_front() {
        let Some(node) = data.vbank().get(v) else { continue };
        for r in node.descend_iter() {
            let Some(o) = data.obank().get(r) else { continue };
            let derived = match o.code() {
                OpCode::CPUI_INT_ADD => constant(if o.get_in(0) == Some(v) { o.get_in(1) } else { o.get_in(0) }),
                OpCode::CPUI_INT_SUB => o.get_in(0) == Some(v) && constant(o.get_in(1)),
                OpCode::CPUI_COPY | OpCode::CPUI_CAST => true,
                OpCode::CPUI_INT_AND
                | OpCode::CPUI_INT_OR
                | OpCode::CPUI_INT_XOR
                | OpCode::CPUI_INT_MULT
                | OpCode::CPUI_INT_DIV
                | OpCode::CPUI_INT_SDIV
                | OpCode::CPUI_INT_REM
                | OpCode::CPUI_INT_SREM
                | OpCode::CPUI_INT_LEFT
                | OpCode::CPUI_INT_RIGHT
                | OpCode::CPUI_INT_SRIGHT => {
                    if hops > 0 {
                        return true;
                    }
                    false
                }
                _ => false,
            };
            if !derived || hops + 1 >= HOP_CAP {
                continue;
            }
            if let Some(out) = o.get_out() {
                if seen.insert(out) {
                    work.push_back((out, hops + 1));
                }
            }
        }
    }
    false
}

/// Does the caller hold the argument `vn` as an address for a reason of its own,
/// rather than because a type was voted onto it? Its value -- `vn` and every
/// varnode a COPY, CAST or INDIRECT joins to it, never a MULTIEQUAL, which
/// merges other values -- must be read or written through ([`dereferenced`]),
/// be the address of a frame object or a string, come from or go to a declared
/// pointer (a library's `void *malloc(size_t)`, `free(void *)`), or be a
/// parameter the function's own callers stated a pointer for. A HighVariable's
/// type alone is not enough: a register the function reuses for a size and a
/// pointer is one variable, typed as the pointer, at every use, and the merge
/// that joins the two values is where the size would pick up the pointer's
/// dereference.
pub fn provably_a_pointer(data: &Funcdata, vn: VarnodeId) -> bool {
    let value = same_value(data, vn);
    if dereferenced(data, &value) {
        return true;
    }
    value.iter().any(|&v| {
        let Some(node) = data.vbank().get(v) else { return false };
        if frame_address(data, v) || addresses_a_string(data, v) {
            return true;
        }
        if node.is_input() && stated_pointer(data, node.get_addr(), node.get_size()) {
            return true;
        }
        if let Some(def) = node.get_def() {
            if declared_pointer_result(data, def) {
                return true;
            }
        }
        node.descend_iter().any(|r| declared_pointer_argument(data, r, v))
    })
}

/// `vn` and every varnode a COPY, CAST or INDIRECT joins to it, in either
/// direction: the one value, without the values a MULTIEQUAL merges with it.
fn same_value(data: &Funcdata, vn: VarnodeId) -> Vec<VarnodeId> {
    let mut out = vec![vn];
    let mut seen: FxHashSet<VarnodeId> = FxHashSet::from_iter([vn]);
    let mut at = 0;
    while at < out.len() && out.len() < 64 {
        let v = out[at];
        at += 1;
        let Some(node) = data.vbank().get(v) else { continue };
        let mut next: Vec<VarnodeId> = Vec::new();
        if let Some(o) = node.get_def().and_then(|d| data.obank().get(d)) {
            if matches!(o.code(), OpCode::CPUI_COPY | OpCode::CPUI_CAST | OpCode::CPUI_INDIRECT) {
                next.extend(o.get_in(0));
            }
        }
        for r in node.descend_iter() {
            let Some(o) = data.obank().get(r) else { continue };
            let joins = match o.code() {
                OpCode::CPUI_COPY | OpCode::CPUI_CAST => true,
                OpCode::CPUI_INDIRECT => o.get_in(0) == Some(v),
                _ => false,
            };
            if joins {
                next.extend(o.get_out());
            }
        }
        for n in next {
            if data.vbank().get(n).is_some_and(|x| !x.is_constant()) && seen.insert(n) {
                out.push(n);
            }
        }
    }
    out
}

/// Is `v` computed from the stack pointer by copies and literal offsets alone
/// -- the address of a frame object on every path?
fn frame_address(data: &Funcdata, v: VarnodeId) -> bool {
    let constant = |x: Option<VarnodeId>| x.and_then(|x| data.vbank().get(x)).is_some_and(|c| c.is_constant());
    let mut at = Some(v);
    for _ in 0..16 {
        let Some(node) = at.and_then(|x| data.vbank().get(x)) else { return false };
        if node.is_spacebase() && !node.is_constant() {
            return true;
        }
        let Some(o) = node.get_def().and_then(|d| data.obank().get(d)) else { return false };
        at = match o.code() {
            OpCode::CPUI_COPY | OpCode::CPUI_CAST | OpCode::CPUI_PTRSUB | OpCode::CPUI_PTRADD => o.get_in(0),
            OpCode::CPUI_INT_ADD if constant(o.get_in(1)) => o.get_in(0),
            OpCode::CPUI_INT_ADD if constant(o.get_in(0)) => o.get_in(1),
            OpCode::CPUI_INT_SUB if constant(o.get_in(1)) => o.get_in(0),
            _ => return false,
        };
    }
    false
}

/// Is some member of `family` the address of a LOAD or STORE, directly or
/// through copies, literal offsets and the base slot of the engine's pointer
/// arithmetic? An add of two variables says nothing: either operand could be
/// the base. Nor does a MULTIEQUAL: its output holds other values on other
/// paths, and the dereference may be the one those paths make.
fn dereferenced(data: &Funcdata, family: &[VarnodeId]) -> bool {
    let mut seen: FxHashSet<VarnodeId> = family.iter().copied().collect();
    let mut work: VecDeque<(VarnodeId, usize)> = family.iter().map(|&v| (v, 0)).collect();
    while let Some((v, hops)) = work.pop_front() {
        let Some(node) = data.vbank().get(v) else { continue };
        for r in node.descend_iter() {
            let Some(o) = data.obank().get(r) else { continue };
            let carries = match o.code() {
                OpCode::CPUI_LOAD | OpCode::CPUI_STORE => {
                    if o.get_in(1) == Some(v) {
                        return true;
                    }
                    false
                }
                OpCode::CPUI_COPY | OpCode::CPUI_CAST => true,
                OpCode::CPUI_INDIRECT | OpCode::CPUI_PTRSUB | OpCode::CPUI_PTRADD => o.get_in(0) == Some(v),
                OpCode::CPUI_INT_ADD => {
                    let other = if o.get_in(0) == Some(v) { o.get_in(1) } else { o.get_in(0) };
                    other.and_then(|c| data.vbank().get(c)).is_some_and(|c| c.is_constant())
                }
                _ => false,
            };
            if !carries || hops + 1 >= HOP_CAP {
                continue;
            }
            if let Some(out) = o.get_out() {
                if seen.insert(out) {
                    work.push_back((out, hops + 1));
                }
            }
        }
    }
    false
}

/// Is `v` a string literal: a constant, or the engine's `PTRSUB` off the
/// constant zero, that resolves to a character array kuna recovered
/// (`kuna_charptr::constant_is_string`)? Any other constant is a number as
/// often as an address -- `0x2000` is a buffer size and the address of
/// `.init` -- so it proves nothing.
fn addresses_a_string(data: &Funcdata, v: VarnodeId) -> bool {
    let Some(node) = data.vbank().get(v) else { return false };
    let (reader, constant) = if node.is_constant() {
        (node.descend_iter().next(), Some(v))
    } else {
        let def = node.get_def();
        let base = def
            .and_then(|d| data.obank().get(d))
            .filter(|o| o.code() == OpCode::CPUI_PTRSUB)
            .filter(|o| o.get_in(0).and_then(|b| data.vbank().get(b)).is_some_and(|b| b.is_constant() && b.get_offset() == 0))
            .and_then(|o| o.get_in(1));
        (def, base)
    };
    match (reader, constant) {
        (Some(op), Some(c)) => crate::p5_types::kuna_charptr::constant_is_string(data, op, c),
        _ => false,
    }
}

/// Did the function's callers state a pointer for its input in this storage?
fn stated_pointer(data: &Funcdata, addr: &Address, size: i32) -> bool {
    data.kuna_calleevote_inputs()
        .and_then(|s| s.at(addr, size))
        .is_some_and(|t| t.ct.get_metatype() == type_metatype::TYPE_PTR)
}

/// Is `def` a call whose declared prototype returns a pointer?
fn declared_pointer_result(data: &Funcdata, def: OpId) -> bool {
    let Some(o) = data.obank().get(def) else { return false };
    if !matches!(o.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND) {
        return false;
    }
    let Some(i) = data.get_call_specs_index(def) else { return false };
    let proto = data.get_call_specs(i).proto();
    proto.is_output_locked() && proto.get_output_type().is_some_and(|t| t.get_metatype() == type_metatype::TYPE_PTR)
}

/// Does the call `r` hand `v` to a parameter its declared prototype types as a
/// pointer?
fn declared_pointer_argument(data: &Funcdata, r: OpId, v: VarnodeId) -> bool {
    let Some(o) = data.obank().get(r) else { return false };
    if !matches!(o.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND) {
        return false;
    }
    let Some(i) = data.get_call_specs_index(r) else { return false };
    let proto = data.get_call_specs(i).proto();
    (1..o.num_input()).filter(|&s| o.get_in(s) == Some(v)).any(|s| {
        proto.get_param(s - 1).is_some_and(|p| {
            p.is_type_locked() && p.get_type().is_some_and(|t| t.get_metatype() == type_metatype::TYPE_PTR)
        })
    })
}
