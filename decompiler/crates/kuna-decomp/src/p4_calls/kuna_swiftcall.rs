//! (kuna `swiftcall`) Swift calls: the ones that may throw, and the `self`
//! a call to a struct member does not pass.
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
//! Inert unless the Swift metadata pass bound the convention in this image.

use std::collections::BTreeSet;
use std::rc::Rc;

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
    pub throws: Rc<ProtoModel>,
    /// The bound callees that may or may not take `swiftself`, by entry
    /// `(space index, offset)`.
    pub maybe_self: BTreeSet<(int4, uintb)>,
}

fn overlaps(a: &Address, asize: int4, b: &Address, bsize: int4) -> bool {
    a.overlap(0, b, bsize) >= 0 || b.overlap(0, a, asize) >= 0
}

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
