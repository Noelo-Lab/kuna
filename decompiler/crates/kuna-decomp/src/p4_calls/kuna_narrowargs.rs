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
//! bits it read before, so no printed value moves, and the parameter is
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

/// Is `vn` an input this function may narrow: a register that is not
/// type-locked, that no `PIECE` joins into a wider value, and whose narrow
/// consumption does not come from a shift? A left shift drops the high bits of
/// any width of value, and a register shift amount is read through its low
/// byte (`lsl r6, r7, lr` is `r7 << (lr & 0xff)`), so neither says anything
/// about the value's type.
fn narrowable(data: &Funcdata, vn: VarnodeId) -> bool {
    let Some(v) = data.vbank().get(vn) else { return false };
    if !v.is_input() || v.is_type_lock() || v.is_addr_tied() || v.is_persist() {
        return false;
    }
    if v.get_addr().get_space().is_none_or(|s| s.get_type() != kuna_base::space::spacetype::IPTR_PROCESSOR) {
        return false;
    }
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
            match o.code() {
                OpCode::CPUI_PIECE | OpCode::CPUI_INT_LEFT => return false,
                OpCode::CPUI_INT_RIGHT | OpCode::CPUI_INT_SRIGHT if slot == 1 => return false,
                OpCode::CPUI_INT_AND if shifts_by(data, o.get_out()) => return false,
                OpCode::CPUI_COPY | OpCode::CPUI_MULTIEQUAL | OpCode::CPUI_INDIRECT | OpCode::CPUI_CAST => {
                    if let Some(out) = o.get_out().filter(|out| !family.contains(out)) {
                        family.push(out);
                    }
                }
                _ => {}
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
    let inputs: Vec<VarnodeId> = data.vbank().iter_def_flag(crate::varnode::varnode_flags::input).collect();
    let mut count = 0;
    for vn in inputs {
        if !narrowable(data, vn) {
            continue;
        }
        let (addr, size, consume) = {
            let v = data.vbank().get(vn).expect("narrow_inputs: stale input");
            (v.get_addr().clone(), v.get_size(), v.get_consume())
        };
        let Some(w) = narrow_width(consume, size) else { continue };
        if !data.get_func_proto().possible_input_param(&addr, size) {
            continue;
        }
        let Some(part) = low_part(data, &addr, size, w) else { continue };
        if replace(data, vn, &addr, size, &part, w).is_ok() {
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
    count
}

fn replace(
    data: &mut Funcdata,
    vn: VarnodeId,
    addr: &Address,
    size: int4,
    part: &Address,
    w: int4,
) -> kuna_base::error::KunaResult<()> {
    let topbl = data.bblocks_get_block(0);
    let start = data.bblocks_block_start(topbl);
    let op = data.new_op(1, start);
    let out = data.new_varnode_out(size, addr, op)?;
    data.op_set_opcode_code(op, OpCode::CPUI_INT_ZEXT);
    data.total_replace(vn, out)?;
    data.delete_varnode(vn)?;
    let narrow = data.new_varnode(w, part, None);
    let narrow = data.set_input_varnode(narrow)?;
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
