//! (kuna) `varargforward` — a declared value a function hands on unchanged, in
//! the register it already occupies, to the variable part of a variadic call is
//! an argument of that call.
//!
//! `int d2(const char *f, int a) { return pr(f, a); }` with `pr(const char *,
//! ...)` compiles to `xor %eax,%eax; jmp pr` on x86-64: `a` is still in `esi`,
//! the first variadic integer register. `double c2(int k, double x) { return
//! vr(k, x); }` is a bare `b vr` on AArch64, with `x` still in `d0`, and
//! `vr(k, g(k))` leaves `g`'s result in `d0`. Upstream refuses all three:
//! `AncestorRealistic` expects "active movement into the parameter" and fails a
//! trial whose value is the function's own input, and `ancestorOpUse` fails one
//! whose value is another call's output. The trial is inactive, and at the end
//! of the list it is dropped.
//!
//! The instructions are also what `pr(f)` compiles to when `a` is unused, and
//! `g(k); vr(k)` when `g`'s result is discarded, so the evidence is a declared
//! prototype and a value nothing else reads:
//!
//! * the value is the calling function's own input, its input is locked, and a
//!   declared parameter starts at the same storage; or
//! * the value is the output of a call whose return type is locked and not
//!   `void`;
//! * and the call slot is the only reader of the value.
//!
//! A declared value read nowhere else is there to be passed on. Either reading
//! compiles back to the same call, since the register holds the value when the
//! callee starts.
//!
//! A floating-point register is refused where the caller states how many it
//! filled (`al` on x86-64 SysV, CR bit 6 on 32-bit PowerPC: those registers are
//! [`crate::p4_calls::kuna_varargretreg`]'s) and where it only shadows a general
//! register (64-bit PowerPC).
use kuna_base::address::Address;
use kuna_base::error::KunaResult;
use kuna_base::types::int4;

use kuna_num::opcodes::OpCode;

use crate::context::{OpId, VarnodeId};
use crate::dtype::{type_class, type_metatype};
use crate::funcdata::Funcdata;
use crate::p0_knowledge::options::on_or_off;
use crate::p4_calls::kuna_varargretreg::{float_register_count, VarargFloats};

/// `option varargforward on|off`.
pub struct OptionVarargForward;

impl OptionVarargForward {
    /// The option name.
    pub const NAME: &'static str = "varargforward";

    /// Resolve the flag and its confirmation message; the caller writes it into
    /// `Architecture::vararg_forward`.
    pub fn apply(&self, p1: &str) -> KunaResult<(bool, String)> {
        let val = on_or_off(p1)?;
        let prop = if val { "on" } else { "off" };
        Ok((val, format!("Forwarded declared values at variadic calls turned {prop}")))
    }
}

/// Is the input trial `[addr, addr + size)` at `slot` of variadic call spec `idx`
/// a declared parameter of the calling function, or the declared result of
/// another call, that reaches the call unchanged and is read by nothing else?
pub fn forwards_declared_value(
    data: &Funcdata,
    idx: int4,
    slot: int4,
    addr: &Address,
    size: int4,
) -> bool {
    let arch = data.get_arch();
    if !arch.vararg_forward {
        return false;
    }
    let fc = data.get_call_specs(idx);
    if !fc.is_dotdotdot() || !fc.proto().has_model() {
        return false;
    }
    let op = fc.get_op();
    let Some(vn) = data.obank().get(op).and_then(|o| o.get_in(slot)) else { return false };
    let Some(v) = data.vbank().get(vn) else { return false };
    if !read_only_at(data, vn, op, slot, 0) {
        return false;
    }
    let float = fc.proto().model().input().get_entry().iter().any(|e| {
        e.get_type() == type_class::TYPECLASS_FLOAT && e.justified_contain(addr, size) >= 0
    });
    if float
        && (arch.vararg_floats == VarargFloats::Shadowed || float_register_count(data, op).is_some())
    {
        return false;
    }
    let (at, width) = (v.get_addr(), v.get_size());
    if v.is_input() {
        let proto = data.get_func_proto();
        return proto.is_input_locked()
            && (0..proto.num_params()).filter_map(|i| proto.get_param(i)).any(|p| {
                let (pa, ps) = (p.get_address(), p.get_size());
                at.justified_contain(width, &pa, ps, false) == 0
                    || pa.justified_contain(ps, at, width, false) == 0
            });
    }
    let Some(def) = v.get_def() else { return false };
    if !data.obank().get(def).is_some_and(|o| o.is_call()) {
        return false;
    }
    let Some(producer) = data.get_call_specs_index(def) else { return false };
    let proto = data.get_call_specs(producer).proto();
    let out = proto.get_output();
    proto.is_output_locked()
        && out.get_type().is_some_and(|t| t.get_metatype() != type_metatype::TYPE_VOID)
        && out.get_address() == *at
        && out.get_size() == width
}

/// Is `vn` read only by `call` at `slot`? An INDIRECT standing for some call's
/// possible effect on the storage is not a read, as long as its own output is
/// read by nothing else.
fn read_only_at(data: &Funcdata, vn: VarnodeId, call: OpId, slot: int4, depth: u32) -> bool {
    let Some(v) = data.vbank().get(vn) else { return false };
    v.descend_iter().all(|d| {
        let Some(o) = data.obank().get(d) else { return false };
        if d == call {
            return (0..o.num_input()).all(|i| i == slot || o.get_in(i) != Some(vn));
        }
        o.code() == OpCode::CPUI_INDIRECT
            && depth < 4
            && o.get_out().is_some_and(|out| read_only_at(data, out, call, slot, depth + 1))
    })
}
