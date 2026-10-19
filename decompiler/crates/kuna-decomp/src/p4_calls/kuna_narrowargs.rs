//! (kuna `narrowargs`) A register parameter whose body reads only its low bytes
//! is declared that narrow.
//!
//! An ARM AAPCS or x86-64 caller hands a `uint8_t` or an `int` over in a whole
//! register, so a callee that computes with the register at its full width
//! (`and.w r2, r1, #7`, `mov %edi,%eax` beside a forwarded `%rdi`) leaves the
//! input Varnode register-wide and the recovered parameter `unsigned int` or
//! `unsigned long`. `ActionDeadCode` already knows which bits of that input
//! any use can observe. When they all lie in a low part of 1, 2 or 4 bytes and
//! reach that part's top bit, the input is replaced by the part, zero-extended
//! back to the register at the function's entry: every use reads the same
//! bits it read before, so no value the function computes moves, and the parameter is
//! recovered at the narrow width. In the callee-first whole-binary run the
//! parameter a callee declares that way caps how much of the argument its
//! callers' dead-code pass treats as consumed, so a value forwarded to it is
//! narrowed in the caller too.

use kuna_base::address::{calc_mask, Address};
use kuna_base::types::{int4, uintb};
use kuna_num::opcodes::OpCode;

use crate::context::VarnodeId;
use crate::dtype::type_class;
use crate::fspec::FuncCallSpecs;
use crate::funcdata::Funcdata;

/// The narrowest of 1, 2 and 4 bytes whose low part holds every bit of
/// `consume`, when the bits reach that part's top bit and it is narrower than
/// `size`.
fn narrow_width(consume: uintb, size: int4) -> Option<int4> {
    if consume == 0 {
        return None;
    }
    let w = [1, 2, 4].into_iter().find(|&w| consume & !calc_mask(w) == 0)?;
    (w < size && (consume >> (8 * w - 1)) & 1 == 1).then_some(w)
}

/// The integer argument register entry of the function's model that is exactly
/// `addr:size` and holds its low `w` bytes justified, and where the low part
/// starts.
fn low_part(data: &Funcdata, addr: &Address, size: int4, w: int4) -> Option<Address> {
    let proto = data.get_func_proto();
    if !proto.has_model() {
        return None;
    }
    let part = if addr.get_space()?.is_big_endian() { addr + (size - w) as i64 } else { addr.clone() };
    proto
        .model()
        .input()
        .get_entry()
        .iter()
        .any(|e| {
            e.get_type() == type_class::TYPECLASS_GENERAL
                && e.get_size() == size
                && e.justified_contain(addr, size) == 0
                && e.justified_contain(&part, w) == 0
        })
        .then_some(part)
}

/// Is `vn` an input this function may narrow to `w` bytes: a register that is
/// not type-locked, that no `PIECE` joins into a wider value, and whose narrow
/// consumption does not come from a shift? A left shift (or a multiply by a
/// multiple of 256) drops the high bits of any width of value, and a register
/// shift amount is read through its low byte (`lsl r6, r7, lr` is
/// `r7 << (lr & 0xff)`), so neither says anything about the value's type. The
/// value is followed through copies, merges, casts and the arithmetic and
/// bitwise operations whose low bits depend only on its low bits
/// (`(a0 ^ 0x8000) << 0x10`), up to a mask that is itself no wider than `w`.
fn narrowable(data: &Funcdata, vn: VarnodeId, w: int4) -> bool {
    let Some(v) = data.vbank().get(vn) else { return false };
    if !v.is_input() || v.is_type_lock() || v.is_addr_tied() || v.is_persist() {
        return false;
    }
    if v.get_addr().get_space().is_none_or(|s| s.get_type() != kuna_base::space::spacetype::IPTR_PROCESSOR) {
        return false;
    }
    let constant = |data: &Funcdata, vn: Option<VarnodeId>| {
        vn.and_then(|vn| data.vbank().get(vn)).filter(|c| c.is_constant()).map(|c| c.get_offset())
    };
    let mut family = vec![vn];
    let mut i = 0;
    while i < family.len() {
        if family.len() > 64 {
            return false;
        }
        let id = family[i];
        i += 1;
        let Some(cur) = data.vbank().get(id) else { return false };
        for op in cur.descend_iter() {
            let Some(o) = data.obank().get(op) else { return false };
            let slot = (0..o.num_input()).find(|&k| o.get_in(k) == Some(id)).unwrap_or(0);
            let other = o.get_in(1 - slot.min(1));
            let follow = match o.code() {
                OpCode::CPUI_PIECE | OpCode::CPUI_INT_LEFT => return false,
                OpCode::CPUI_INT_RIGHT | OpCode::CPUI_INT_SRIGHT => {
                    if slot == 1 {
                        return false;
                    }
                    false
                }
                OpCode::CPUI_INT_MULT => {
                    if constant(data, other).is_some_and(|c| c != 0 && c.trailing_zeros() >= 8) {
                        return false;
                    }
                    true
                }
                OpCode::CPUI_INT_AND => {
                    if shifts_by(data, o.get_out()) {
                        return false;
                    }
                    !constant(data, other).is_some_and(|c| c & !calc_mask(w) == 0)
                }
                OpCode::CPUI_COPY
                | OpCode::CPUI_MULTIEQUAL
                | OpCode::CPUI_INDIRECT
                | OpCode::CPUI_CAST
                | OpCode::CPUI_INT_XOR
                | OpCode::CPUI_INT_OR
                | OpCode::CPUI_INT_ADD
                | OpCode::CPUI_INT_SUB
                | OpCode::CPUI_INT_NEGATE
                | OpCode::CPUI_INT_2COMP => true,
                _ => false,
            };
            if follow {
                if let Some(out) = o.get_out().filter(|out| !family.contains(out)) {
                    family.push(out);
                }
            }
        }
    }
    true
}

/// Is `vn` the amount of a shift?
fn shifts_by(data: &Funcdata, vn: Option<VarnodeId>) -> bool {
    let Some(v) = vn.and_then(|vn| data.vbank().get(vn)) else { return false };
    v.descend_iter().any(|op| {
        data.obank().get(op).is_some_and(|o| {
            matches!(o.code(), OpCode::CPUI_INT_LEFT | OpCode::CPUI_INT_RIGHT | OpCode::CPUI_INT_SRIGHT)
                && o.get_in(1) == vn
        })
    })
}

/// Replace every register input whose consumed bits fit a narrower low part by
/// that part, zero-extended at the entry. Returns how many were replaced.
pub fn narrow_inputs(data: &mut Funcdata) -> int4 {
    if !data.get_arch().narrow_args || data.get_func_proto().is_input_locked() || data.bblocks_get_size() == 0 {
        return 0;
    }
    let stated = crate::kuna_callerargs::narrow_stated(data);
    let inputs: Vec<VarnodeId> = data.vbank().iter_def_flag(crate::varnode::varnode_flags::input).collect();
    let mut count = 0;
    for vn in inputs {
        let Some((addr, size, consume)) = data.vbank().get(vn).map(|v| (v.get_addr().clone(), v.get_size(), v.get_consume()))
        else {
            continue;
        };
        let Some(w) = narrow_width(consume, size) else { continue };
        if !narrowable(data, vn, w) {
            continue;
        }
        if !data.get_func_proto().possible_input_param(&addr, size) {
            continue;
        }
        let Some(part) = low_part(data, &addr, size, w) else { continue };
        if replace(data, vn, &addr, size, &part, w, OpCode::CPUI_INT_ZEXT).is_ok() {
            if trace() {
                eprintln!(
                    "[narrowargs] 0x{:x} input {}:{:x}:{size} consume 0x{consume:x} -> {w}",
                    data.get_address().get_offset(),
                    addr.get_space().map(|s| s.get_name()).unwrap_or("?"),
                    addr.get_offset()
                );
            }
            count += 1;
        }
    }
    count + stated
}

/// Replace the register input `vn` by its low `w` bytes, extended back to the
/// register by `ext` at the entry, when the model passes an integer argument
/// there; whether it was.
pub(crate) fn narrow_input_to(data: &mut Funcdata, vn: VarnodeId, w: int4, ext: OpCode) -> bool {
    let Some((addr, size)) = data.vbank().get(vn).map(|v| (v.get_addr().clone(), v.get_size())) else { return false };
    let Some(part) = low_part(data, &addr, size, w) else { return false };
    replace(data, vn, &addr, size, &part, w, ext).is_ok()
}

/// Does an input other than `vn` overlap `addr:size`?
fn overlaps_another_input(data: &Funcdata, vn: VarnodeId, addr: &Address, size: int4) -> bool {
    let space = addr.get_space().map(|s| s.get_index());
    let (lo, hi) = (addr.get_offset(), addr.get_offset() + size as u64);
    data.vbank().iter_def_flag(crate::varnode::varnode_flags::input).any(|other| {
        other != vn
            && data.vbank().get(other).is_some_and(|o| {
                let a = o.get_addr();
                a.get_space().map(|s| s.get_index()) == space
                    && a.get_offset() < hi
                    && lo < a.get_offset() + o.get_size() as u64
            })
    })
}

/// Replace input `vn` by its low `w` bytes at `part`, extended by `ext` at the
/// entry. Nothing changes when another input overlaps the register, the one
/// case in which the narrow input could not be made; past that check a failure
/// puts the register-wide input back.
fn replace(
    data: &mut Funcdata,
    vn: VarnodeId,
    addr: &Address,
    size: int4,
    part: &Address,
    w: int4,
    ext: OpCode,
) -> kuna_base::error::KunaResult<()> {
    if overlaps_another_input(data, vn, addr, size) {
        return Err(kuna_base::error::KunaError::lowlevel("narrowargs: overlapping input varnodes".to_string()));
    }
    let topbl = data.bblocks_get_block(0);
    let start = data.bblocks_block_start(topbl);
    let op = data.new_op(1, start);
    let out = data.new_varnode_out(size, addr, op)?;
    data.op_set_opcode_code(op, ext);
    data.total_replace(vn, out)?;
    data.delete_varnode(vn)?;
    let narrow = data.new_varnode(w, part, None);
    let narrow = match data.set_input_varnode(narrow) {
        Ok(narrow) => narrow,
        Err(e) => {
            let wide = data.new_varnode(size, addr, None);
            let wide = data.set_input_varnode(wide)?;
            data.total_replace(out, wide)?;
            let zero = data.new_constant(w, 0);
            data.op_set_input(op, zero, 0)?;
            data.op_destroy_raw(op)?;
            return Err(e);
        }
    };
    data.op_set_input(op, narrow, 0)?;
    data.op_insert_begin(op, topbl);
    Ok(())
}

/// How many bytes of argument `slot` (0-based) of the call `fc` its callee
/// reads, when the callee's recovered parameter in that position is the low
/// part of the integer register the argument is passed in; 0 when nothing is
/// stated.
pub fn stated_bytes_consumed(data: &Funcdata, fc: &FuncCallSpecs, slot: usize) -> int4 {
    if !data.get_arch().narrow_args {
        return 0;
    }
    let Some(stated) = data.kuna_protoorder_types(fc.get_entry_address()) else { return 0 };
    let inputs = data.obank().get(fc.get_op()).map(|o| o.num_input());
    if inputs != Some(fc.final_input_storage().len() as int4 + 1) {
        return 0;
    }
    let (Some((aaddr, asize)), Some((paddr, psize, ct))) =
        (fc.final_input_storage().get(slot), stated.inputs.get(slot))
    else {
        return 0;
    };
    if psize >= asize || ct.get_metatype() == crate::dtype::type_metatype::TYPE_FLOAT {
        return 0;
    }
    let Some(space) = aaddr.get_space() else { return 0 };
    let low = if space.is_big_endian() { aaddr + (asize - psize) as i64 } else { aaddr.clone() };
    if paddr != &low {
        return 0;
    }
    let general = fc.proto().has_model()
        && fc.proto().model().input().get_entry().iter().any(|e| {
            e.get_type() == type_class::TYPECLASS_GENERAL && e.justified_contain(aaddr, *asize) == 0
        });
    if general {
        *psize
    } else {
        0
    }
}

/// Is the decision trace on (`KUNA_NARROWARGS_TRACE=1`)?
fn trace() -> bool {
    std::env::var("KUNA_NARROWARGS_TRACE").is_ok_and(|v| v != "0" && !v.is_empty())
}
