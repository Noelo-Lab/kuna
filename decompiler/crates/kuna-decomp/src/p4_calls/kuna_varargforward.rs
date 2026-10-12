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
use kuna_base::space::spacetype;
use kuna_base::types::int4;

use kuna_num::opcodes::OpCode;

use crate::context::{OpId, VarnodeId};
use crate::dtype::{type_class, type_metatype};
use crate::fspec::{FuncCallSpecs, ParameterPieces, PrototypePieces};
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
    if !read_only_at(data, vn, op, slot, 0) || !declared_value(data, vn) {
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
    !format_stops_before(data, fc, addr, size, float)
}

/// Narrow register trial `i` of variadic call spec `idx` to the int width when
/// only its low int bytes are defined and every byte above them is a part of
/// the caller's own incoming register that its locked prototype declares no
/// parameter in. The trial may be active, or inactive and later gap-filled.
/// `int d4(const char *f, int a) { pr(f, a); return pr(f, a); }` hands `a` on
/// in `esi`; the 8-byte `rsi` trial would otherwise print `CONCAT44(v1,a)` with
/// `v1` never set.
pub fn narrow_undeclared_upper(data: &mut Funcdata, idx: int4, i: int4) {
    let arch = data.get_arch();
    if !arch.vararg_forward {
        return;
    }
    let Some(width) = arch.types().map(|t| t.get_size_of_int()) else { return };
    let fc = data.get_call_specs(idx);
    if !fc.is_dotdotdot() || !fc.proto().has_model() {
        return;
    }
    let trial = fc.active_input().get_trial(i);
    let (addr, size, slot) = (trial.get_address().clone(), trial.get_size(), trial.get_slot());
    if size <= width
        || addr.get_space().is_none_or(|s| s.get_type() != spacetype::IPTR_PROCESSOR)
        || fc.proto().model().input().get_entry().iter().any(|e| {
            e.get_type() == type_class::TYPECLASS_FLOAT && e.justified_contain(&addr, size) >= 0
        })
    {
        return;
    }
    let low = if addr.is_big_endian() { &addr + (size - width) as i64 } else { addr.clone() };
    if !fc.active_input().test_shrink(i, &low, width) {
        return;
    }
    let Some(vn) = data.obank().get(fc.get_op()).and_then(|o| o.get_in(slot)) else { return };
    if !undeclared_bytes(data, vn, width, size - width, &mut 64)
        || !super::kuna_varargformat::defined_bytes(data, vn, 0, width, &mut 64)
    {
        return;
    }
    data.get_call_specs_mut(idx).get_active_input().shrink(i, low, width);
}

/// Are bytes `[offset, offset + width)` of `vn`, counted from its least
/// significant byte, copied from an incoming register that no parameter of the
/// calling function's locked prototype overlaps?
fn undeclared_bytes(
    data: &Funcdata,
    vn: VarnodeId,
    offset: int4,
    width: int4,
    budget: &mut u32,
) -> bool {
    if *budget == 0 {
        return false;
    }
    *budget -= 1;
    let Some(v) = data.vbank().get(vn) else { return false };
    let size = v.get_size();
    if offset < 0 || width <= 0 || offset > size - width {
        return false;
    }
    if v.is_input() {
        let proto = data.get_func_proto();
        let at = v.get_addr();
        return proto.is_input_locked()
            && at.get_space().is_some_and(|s| s.get_type() == spacetype::IPTR_PROCESSOR)
            && (0..proto.num_params()).filter_map(|i| proto.get_param(i)).all(|p| {
                let (pa, ps) = (p.get_address(), p.get_size());
                pa.get_space().is_some_and(|s| s.get_type() != spacetype::IPTR_JOIN)
                    && at.overlap(0, &pa, ps) < 0
                    && pa.overlap(0, at, size) < 0
            });
    }
    let Some(op) = v.get_def().and_then(|id| data.obank().get(id)) else { return false };
    let mut input = |slot, off, len| {
        op.get_in(slot).is_some_and(|id| undeclared_bytes(data, id, off, len, budget))
    };
    match op.code() {
        OpCode::CPUI_COPY => input(0, offset, width),
        OpCode::CPUI_PIECE => {
            let Some(low) = op.get_in(1).and_then(|id| data.vbank().get(id)) else {
                return false;
            };
            let low_size = low.get_size();
            if offset >= low_size {
                input(0, offset - low_size, width)
            } else if offset + width <= low_size {
                input(1, offset, width)
            } else {
                input(1, offset, low_size - offset) && input(0, 0, offset + width - low_size)
            }
        }
        OpCode::CPUI_SUBPIECE => {
            let Some(skip) = op.get_in(1).and_then(|id| data.vbank().get(id)) else {
                return false;
            };
            let Some(source) = op.get_in(0).and_then(|id| data.vbank().get(id)) else {
                return false;
            };
            skip.is_constant()
                && skip.get_offset() <= source.get_size() as u64
                && input(0, offset + skip.get_offset() as int4, width)
        }
        OpCode::CPUI_MULTIEQUAL => (0..op.num_input()).all(|slot| input(slot, offset, width)),
        _ => false,
    }
}

/// Is `vn` the calling function's own input where it declares a parameter, or
/// the output of a call whose declared return it is?
pub(super) fn declared_value(data: &Funcdata, vn: VarnodeId) -> bool {
    let Some(v) = data.vbank().get(vn) else { return false };
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

/// Does the call pass a printf format as its last fixed argument that consumes
/// fewer arguments than the variadic slot `[addr, addr + size)` would be?
/// `sqlite3_mprintf("... %s ...", zName)` from a callback that declares an
/// unused `NotUsed2` in the next register is not handed `NotUsed2`.
fn format_stops_before(
    data: &Funcdata,
    fc: &FuncCallSpecs,
    addr: &Address,
    size: int4,
    float: bool,
) -> bool {
    let fixed = fc.proto().num_params();
    let Some(format) = fixed.checked_sub(1).and_then(|i| fc.proto().get_param(i)) else {
        return false;
    };
    let active = fc.active_input();
    let index = active.which_trial(&format.get_address(), format.get_size());
    if index < 0 {
        return false;
    }
    let op = fc.get_op();
    let Some(call) = data.obank().get(op) else { return false };
    let Some(vn) = call.get_in(active.get_trial(index).get_slot()) else { return false };
    let Some((value, width)) = constant_value(data, vn, 0) else { return false };
    let Some(count) =
        crate::p4_calls::kuna_formatwitness::pointed_format_count(data, value, width, call.get_addr())
    else {
        return false;
    };
    vararg_position(data, fc, addr, size, float).is_some_and(|position| position > count)
}

/// The constant `vn` holds, with its size: through COPYs, an INT_ADD (an
/// `adrp`/`add` pair, ARM's `add r0,pc,r0`) and a LOAD from read-only memory (a
/// literal pool), before the simplification rules have folded them.
pub(super) fn constant_value(data: &Funcdata, vn: VarnodeId, depth: u32) -> Option<(u64, int4)> {
    let v = data.vbank().get(vn)?;
    let size = v.get_size();
    if v.is_constant() {
        return Some((v.get_offset(), size));
    }
    if depth >= 6 {
        return None;
    }
    let def = data.obank().get(v.get_def()?)?;
    let operand = |slot| constant_value(data, def.get_in(slot)?, depth + 1);
    match def.code() {
        OpCode::CPUI_COPY => operand(0).map(|(value, _)| (value, size)),
        OpCode::CPUI_INT_ADD => {
            let (a, b) = (operand(0)?.0, operand(1)?.0);
            let mask = if size >= 8 { u64::MAX } else { (1u64 << (8 * size)) - 1 };
            Some((a.wrapping_add(b) & mask, size))
        }
        OpCode::CPUI_LOAD => {
            let manage = data.get_arch().manage();
            let index = data.vbank().get(def.get_in(0)?)?.get_offset();
            if index >= manage.num_spaces() as u64 || !(1..=8).contains(&size) {
                return None;
            }
            let space = manage.get_space(index as i32)?.clone();
            let at = Address::new(space.clone(), operand(1)?.0);
            let flags = data.get_arch().query_global_properties(&at, size, def.get_addr());
            if flags & crate::varnode::varnode_flags::readonly == 0 {
                return None;
            }
            let mut bytes = [0u8; 8];
            data.get_arch().loader_fill(&mut bytes[..size as usize], &at).ok()?;
            let bytes = &bytes[..size as usize];
            let fold = |acc: u64, b: &u8| (acc << 8) | *b as u64;
            let value = if space.is_big_endian() {
                bytes.iter().fold(0, fold)
            } else {
                bytes.iter().rev().fold(0, fold)
            };
            Some((value, size))
        }
        _ => None,
    }
}

/// Which variadic argument of its class, counting from 1, the call's model puts
/// in `[addr, addr + size)`.
fn vararg_position(
    data: &Funcdata,
    fc: &FuncCallSpecs,
    addr: &Address,
    size: int4,
    float: bool,
) -> Option<usize> {
    let arch = data.get_arch();
    let types = arch.types()?;
    let metatype = if float { type_metatype::TYPE_FLOAT } else { type_metatype::TYPE_INT };
    let vararg = types.get_base(size, metatype).ok()?;
    let mut pieces = PrototypePieces::default();
    fc.proto().get_pieces(&mut pieces);
    if pieces.outtype.is_none() || pieces.first_var_arg_slot != pieces.intypes.len() as int4 {
        return None;
    }
    for position in 1..=16 {
        pieces.intypes.push(vararg.clone());
        pieces.innames.push(String::new());
        let mut res: Vec<ParameterPieces> = Vec::new();
        fc.proto()
            .model()
            .assign_parameter_storage(&pieces, &mut res, true, types, arch.manage())
            .ok()?;
        if res.last()?.addr == *addr {
            return Some(position);
        }
    }
    None
}
