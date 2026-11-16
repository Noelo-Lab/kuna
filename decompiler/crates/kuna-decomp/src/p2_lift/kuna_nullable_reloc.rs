//! Keep an undefined weak address symbolic at its exact relocated operand.
use crate::context::{OpId, VarnodeId};
use crate::dtype::TypeFactory;
use crate::funcdata::Funcdata;
use kuna_base::address::{calc_mask, Address};
use kuna_num::opcodes::OpCode;

#[derive(Clone, Copy, Debug)]
pub enum Operand {
    Unsupported,
    CodeReference,
    DataPointer,
    MipsLui,
    MipsSigned,
    MipsUnsigned,
    Absolute,
    PcRelative,
}

#[derive(Clone, Debug)]
pub struct NullableRelocation {
    pub field: u64,
    pub slot: u64,
    pub symbol: u64,
    pub addend: i64,
    pub pointer_size: i32,
    pub operand: Operand,
    pub shift: u32,
    pub bias: u64,
    pub patched: u64,
}

/// Run before xrefs or SSA discard the instruction/operand provenance. The
/// supported operand forms are explicit; an unmatched form remains diagnosed.
pub(crate) fn materialize(data: &mut Funcdata, address: &Address, step: i32, ops: &[OpId]) {
    // SLEIGH can emit a direct memory varnode for a fixed address rather than
    // a LOAD (notably x86 RIP-relative reads). Preserve those reads here;
    // computed addresses are handled when RuleLoadVarnode materializes them.
    for &id in ops {
        let count = data.obank().get(id).unwrap().num_input();
        for slot in 0..count {
            let Some(vn) = data
                .obank()
                .get(id)
                .unwrap()
                .get_in(slot)
                .and_then(|v| data.vbank().get(v))
            else {
                continue;
            };
            if address
                .get_space()
                .is_none_or(|s| s.get_index() != vn.get_space().get_index())
            {
                continue;
            }
            let record = data
                .get_arch()
                .nullable_relocations
                .get(&vn.get_offset())
                .and_then(|records| {
                    records.iter().find(|r| {
                        r.pointer_size == vn.get_size() && matches!(r.operand, Operand::DataPointer)
                    })
                })
                .cloned();
            let Some(record) = record else { continue };
            let space = address.get_space().unwrap().clone();
            let space_id = data.new_varnode_space(&space);
            let location = data.new_constant(space.get_addr_size() as i32, record.slot);
            data.vbank_mut()
                .get_mut(location)
                .unwrap()
                .set_synthetic_literal(false);
            let mut value = emit(
                data,
                id,
                OpCode::CPUI_LOAD,
                record.pointer_size,
                &[space_id, location],
            );
            if record.addend != 0 {
                let addend = data.new_constant(record.pointer_size, record.addend as u64);
                value = emit(
                    data,
                    id,
                    OpCode::CPUI_INT_ADD,
                    record.pointer_size,
                    &[value, addend],
                );
            }
            data.op_set_input(id, value, slot)
                .expect("read-only weak pointer");
        }
    }
    let end = address.get_offset().saturating_add(step as u64);
    let records: Vec<_> = data
        .get_arch()
        .nullable_relocations
        .range(address.get_offset()..end)
        .flat_map(|(_, refs)| refs.iter().cloned())
        .collect();
    let alone = records.len() == 1;
    for record in records {
        if matches!(record.operand, Operand::CodeReference) {
            let mut found = false;
            for &id in ops {
                let op = data.obank().get(id).unwrap();
                if matches!(
                    op.code(),
                    OpCode::CPUI_CALL | OpCode::CPUI_BRANCH | OpCode::CPUI_CBRANCH
                ) {
                    if let Some(vn) = op.get_in(0).filter(|&vn| {
                        data.vbank()
                            .get(vn)
                            .is_some_and(|v| v.get_addr().get_offset() == record.symbol)
                    }) {
                        data.vbank_mut()
                            .get_mut(vn)
                            .unwrap()
                            .set_synthetic_literal(false);
                        found = true;
                    }
                }
            }
            if !found {
                data.warning(
                    "Nullable weak code relocation could not be modeled",
                    address,
                );
            }
            continue;
        }
        let mut matches = Vec::new();
        for &id in ops {
            let Some(op) = data.obank().get(id) else {
                continue;
            };
            for slot in 0..op.num_input() {
                let Some(vn) = op.get_in(slot).and_then(|v| data.vbank().get(v)) else {
                    continue;
                };
                if !vn.is_constant() || !(1..=8).contains(&vn.get_size()) {
                    continue;
                }
                let expected = match record.operand {
                    Operand::MipsLui
                        if op.code() == OpCode::CPUI_INT_LEFT
                            && slot == 0
                            && op
                                .get_in(1)
                                .and_then(|v| data.vbank().get(v))
                                .is_some_and(|v| v.is_constant() && v.get_offset() == 16) =>
                    {
                        Some(record.patched)
                    }
                    Operand::MipsSigned
                        if slot == 1
                            && matches!(
                                op.code(),
                                OpCode::CPUI_INT_ADD
                                    | OpCode::CPUI_INT_LESS
                                    | OpCode::CPUI_INT_SLESS
                            ) =>
                    {
                        Some(record.patched as u16 as i16 as i64 as u64)
                    }
                    Operand::MipsUnsigned
                        if slot == 1
                            && matches!(
                                op.code(),
                                OpCode::CPUI_INT_OR | OpCode::CPUI_INT_AND | OpCode::CPUI_INT_XOR
                            ) =>
                    {
                        Some(record.patched)
                    }
                    Operand::Absolute
                        if op.code() == OpCode::CPUI_COPY && slot == 0
                            || alone && vn.get_size() >= 4 =>
                    {
                        let value = record.symbol.wrapping_add(record.addend as u64);
                        (op.code() == OpCode::CPUI_COPY
                            || value != end && value != address.get_offset())
                        .then_some(value)
                    }
                    Operand::PcRelative
                        if slot == 0 && op.code() == OpCode::CPUI_COPY
                            || matches!(record.operand, Operand::PcRelative)
                                && slot == 1
                                && matches!(op.code(), OpCode::CPUI_LOAD | OpCode::CPUI_STORE) =>
                    {
                        Some(
                            record
                                .symbol
                                .wrapping_add(record.addend as u64)
                                .wrapping_add(end - record.field),
                        )
                    }
                    _ => None,
                };
                if expected
                    .is_some_and(|expected| expected & calc_mask(vn.get_size()) == vn.get_offset())
                {
                    matches.push((id, slot, vn.get_size()));
                }
            }
        }
        // An instruction's one immediate can reach several p-code operations
        // (a compare's subtraction and flags). With no other weak field in the
        // instruction they are that one operand, read once before all of them.
        let shared = matches.len() > 1
            && alone
            && matches!(record.operand, Operand::Absolute)
            && matches.iter().all(|m| m.2 == matches[0].2);
        if matches.len() != 1 && !shared {
            data.warning("Nullable weak relocation operand could not be modeled; address guards may be incomplete", address);
            continue;
        }
        let (before, slot, size) = matches[0];
        let before = if shared { ops[0] } else { before };
        let space = address.get_space().unwrap().clone();
        let space_id = data.new_varnode_space(&space);
        let location = data.new_constant(space.get_addr_size() as i32, record.slot);
        data.vbank_mut()
            .get_mut(location)
            .unwrap()
            .set_synthetic_literal(false);
        let mut value = emit(
            data,
            before,
            OpCode::CPUI_LOAD,
            record.pointer_size,
            &[space_id, location],
        );
        let mut addend = record.addend as u64;
        if matches!(record.operand, Operand::PcRelative) {
            addend = addend.wrapping_add(end - record.field);
        }
        addend = addend.wrapping_add(record.bias);
        if addend != 0 {
            let amount = data.new_constant(record.pointer_size, addend);
            value = emit(
                data,
                before,
                OpCode::CPUI_INT_ADD,
                record.pointer_size,
                &[value, amount],
            );
        }
        if record.shift != 0 {
            let count = data.new_constant(4, record.shift as u64);
            value = emit(
                data,
                before,
                OpCode::CPUI_INT_RIGHT,
                record.pointer_size,
                &[value, count],
            );
        }
        if matches!(
            record.operand,
            Operand::MipsLui | Operand::MipsSigned | Operand::MipsUnsigned
        ) {
            let zero = data.new_constant(4, 0);
            value = emit(data, before, OpCode::CPUI_SUBPIECE, 2, &[value, zero]);
            if size > 2 {
                let code = if matches!(record.operand, Operand::MipsSigned) {
                    OpCode::CPUI_INT_SEXT
                } else {
                    OpCode::CPUI_INT_ZEXT
                };
                value = emit(data, before, code, size, &[value]);
            }
        } else if size < record.pointer_size {
            let zero = data.new_constant(4, 0);
            value = emit(data, before, OpCode::CPUI_SUBPIECE, size, &[value, zero]);
        } else if size > record.pointer_size {
            value = emit(data, before, OpCode::CPUI_INT_ZEXT, size, &[value]);
        }
        if shared {
            for &(id, slot, _) in &matches {
                data.op_set_input(id, value, slot).expect("relocation operand");
            }
            continue;
        }
        data.op_set_input(before, value, slot)
            .expect("relocation operand");
        if matches!(
            record.operand,
            Operand::MipsLui | Operand::MipsSigned | Operand::MipsUnsigned
        ) {
            for &id in ops {
                let op = data.obank().get(id).unwrap();
                if !matches!(
                    op.code(),
                    OpCode::CPUI_INT_LEFT
                        | OpCode::CPUI_INT_SEXT
                        | OpCode::CPUI_INT_ZEXT
                        | OpCode::CPUI_INT_ADD
                        | OpCode::CPUI_COPY
                ) {
                    continue;
                }
                if let Some(out) = op.get_out() {
                    if let Some(types) = data.get_arch().types_rc() {
                        let size = data.vbank().get(out).unwrap().get_size();
                        if let Ok(ty) = types.get_base(size, crate::dtype::type_metatype::TYPE_UINT)
                        {
                            data.vbank_mut()
                                .get_mut(out)
                                .unwrap()
                                .update_type_locked(ty, true, true);
                        }
                    }
                }
            }
        }
    }
}

fn emit(
    data: &mut Funcdata,
    before: OpId,
    code: OpCode,
    size: i32,
    inputs: &[VarnodeId],
) -> VarnodeId {
    let op = data.new_op(
        inputs.len() as i32,
        data.obank().get(before).unwrap().get_addr().clone(),
    );
    data.op_set_opcode_code(op, code);
    data.op_set_all_input(op, inputs)
        .expect("symbolic expression");
    let out = data.new_unique_out(size, op).expect("symbolic output");
    if code != OpCode::CPUI_LOAD {
        if let Some(types) = data.get_arch().types_rc() {
            let meta = crate::dtype::type_metatype::TYPE_UINT;
            if let Ok(ty) = types.get_base(size, meta) {
                data.vbank_mut()
                    .get_mut(out)
                    .unwrap()
                    .update_type_locked(ty, true, true);
            }
        }
    }
    data.obank_mut().insert_before_dead(op, before);
    out
}
