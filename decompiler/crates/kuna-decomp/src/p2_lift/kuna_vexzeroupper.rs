//! (kuna `vexzeroupper`) The VEX/EVEX zero-upper write, split at its width.
//!
//! A VEX- or EVEX-encoded instruction that writes an XMM or YMM register clears
//! the rest of the ZMM register. The x86 SLEIGH spec states that as one write of
//! the whole register, `ZmmReg1 = zext(result)`. Every such write then overlaps
//! every read of the same XMM/YMM register, so heritage widens the register's
//! storage to all 64 bytes: a function that reads `ymm0` and returns in `ymm0`
//! has a 64-byte `zmm0` input that no 16- or 32-byte parameter entry can hold,
//! and the result is read back as `SUB6432(zmm0, 0)`.
//!
//! This splits each such write into the result's own width and a zero fill of
//! the bytes above it: `ymm0 = result; zmm0[32..64] = 0`. The two pieces never
//! overlap a read of the low register, so heritage keeps `xmm`/`ymm` storage
//! at its real width.

use kuna_base::address::Address;
use kuna_base::error::KunaResult;
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::context::OpId;
use crate::funcdata::Funcdata;

/// Split the zero-extending register write `op`, if it is one: `INT_ZEXT` into a
/// 32- or 64-byte register from a 16- or 32-byte value. Returns the zero-fill op
/// inserted after it.
pub fn split(data: &mut Funcdata, op: OpId, insn: &[OpId]) -> KunaResult<Option<OpId>> {
    let Some(o) = data.obank().get(op) else { return Ok(None) };
    if o.code() != OpCode::CPUI_INT_ZEXT {
        return Ok(None);
    }
    let (Some(out), Some(input)) = (o.get_out(), o.get_in(0)) else { return Ok(None) };
    let (Some(ov), Some(iv)) = (data.vbank().get(out), data.vbank().get(input)) else { return Ok(None) };
    let (out_size, in_size) = (ov.get_size(), iv.get_size());
    let reg = ov.get_addr().clone();
    let is_register = reg.get_space().is_some_and(|s| s.get_name() == "register");
    if !is_register || !matches!(out_size, 32 | 64) || !matches!(in_size, 16 | 32) || in_size >= out_size {
        return Ok(None);
    }
    let low = reg.clone();
    // When the value is a temporary only this write reads, its producer writes
    // the low register itself, so no copy sits between the result and its uses.
    // At flow time a temporary's readers are not linked to its writer yet, so
    // the producer is the op of this instruction that writes the same storage.
    let in_addr = iv.get_addr().clone();
    let is_unique = in_addr.get_space().is_some_and(|s| s.get_name() == "unique");
    let writes = |o: OpId| {
        data.obank().get(o).and_then(|x| x.get_out()).and_then(|v| data.vbank().get(v)).is_some_and(|v| v.get_addr() == &in_addr)
    };
    let reads = |o: OpId| {
        data.obank().get(o).is_some_and(|x| {
            (0..x.num_input()).any(|k| x.get_in(k).and_then(|v| data.vbank().get(v)).is_some_and(|v| v.get_addr() == &in_addr))
        })
    };
    let producer = if is_unique && insn.iter().filter(|&&o| writes(o)).count() == 1 && insn.iter().filter(|&&o| reads(o)).count() == 1 {
        insn.iter().copied().find(|&o| writes(o))
    } else {
        None
    };
    if let Some(def) = producer {
        data.op_unset_output(def);
        data.new_varnode_out(in_size, &low, def)?;
        return zero_fill(data, op, true, &reg, in_size, out_size).map(Some);
    }
    // `zmm = zext(ymm)` of the register's own low part needs no low copy: the
    // op itself becomes the zero fill.
    if iv.get_addr() == &low {
        return zero_fill(data, op, true, &reg, in_size, out_size).map(Some);
    }
    data.op_unset_output(op);
    data.op_set_opcode_code(op, OpCode::CPUI_COPY);
    data.new_varnode_out(in_size, &low, op)?;
    zero_fill(data, op, false, &reg, in_size, out_size).map(Some)
}

/// Zero bytes `[from, to)` of the register at `reg`, one op per piece that ends
/// at the next YMM/ZMM boundary (so an XMM write leaves the YMM and ZMM upper
/// halves as separate storage). With `reuse`, `op` itself becomes the first
/// piece; otherwise the pieces follow it. Returns the last op.
fn zero_fill(data: &mut Funcdata, op: OpId, reuse: bool, reg: &Address, from: int4, to: int4) -> KunaResult<OpId> {
    let space = reg.get_space().expect("register space").clone();
    let pc = data.obank().get(op).map(|o| o.get_addr().clone()).expect("zero fill: op");
    let mut pieces = Vec::new();
    let mut at = from;
    while at < to {
        let end = if at < 32 { 32.min(to) } else { to };
        pieces.push((at, end - at));
        at = end;
    }
    let mut last = op;
    for (k, (start, size)) in pieces.into_iter().enumerate() {
        let target = if k == 0 && reuse {
            data.op_unset_output(op);
            op
        } else {
            let n = data.new_op(1, pc.clone());
            data.op_dead_insert_after(n, last);
            n
        };
        let zero = data.new_constant(size, 0);
        data.op_set_input(target, zero, 0)?;
        data.op_set_opcode_code(target, OpCode::CPUI_COPY);
        let addr = Address::new(space.clone(), reg.get_offset() + start as u64);
        data.new_varnode_out(size, &addr, target)?;
        last = target;
    }
    Ok(last)
}
