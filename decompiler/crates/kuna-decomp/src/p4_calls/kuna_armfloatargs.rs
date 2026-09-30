//! Scalar VFP input recovery from typed reads and proven callee contracts.

use crate::action::{ruleflags, Action, ActionBase, ActionContext, ActionGroupList, ApplyResult};
use crate::{
    context::VarnodeId,
    dtype::{type_class, type_metatype, Datatype},
    fspec::{FuncCallSpecs, ParameterPieces, PrototypePieces},
    funcdata::Funcdata,
    infra::architecture::Architecture,
    varnode::varnode_flags,
};
use kuna_base::address::Address;
use kuna_num::opcodes::OpCode;

pub struct ActionArmFloatArgs {
    base: ActionBase,
}

impl ActionArmFloatArgs {
    pub fn boxed(group: impl Into<String>) -> Box<dyn Action> {
        Box::new(Self {
            base: ActionBase::new(ruleflags::rule_onceperfunc, "armfloatargs", group),
        })
    }
}

impl Action for ActionArmFloatArgs {
    fn base(&self) -> &ActionBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ActionBase {
        &mut self.base
    }
    fn clone_filtered(&self, groups: &ActionGroupList) -> Option<Box<dyn Action>> {
        groups
            .contains(self.get_group())
            .then(|| Self::boxed(self.get_group()))
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        prepare_inputs(data);
        0
    }
}

pub fn applies(arch: &Architecture) -> bool {
    arch.arm_float_args
        && arch.archid.starts_with("ARM:")
        && arch.translate().loader_rc().borrow().arm_vfp_args()
}

/// Keep a proven double argument independent of an overlapping float call result.
/// Run after passthrough claims, which must inspect the function's original reads.
pub fn link_call_inputs(data: &mut Funcdata, index: i32) {
    if !data.get_arch().arm_float_args {
        return;
    }
    let call = data.get_call_specs(index);
    if call.is_input_locked() || call.is_dotdotdot() {
        return;
    }
    let op = call.get_op();
    if !data
        .obank()
        .get(op)
        .is_some_and(|o| o.code() == OpCode::CPUI_CALL)
    {
        return;
    }
    let Some(stated) = data.kuna_protoorder_types(call.get_entry_address()) else {
        return;
    };
    if !stated.arity_sound {
        return;
    }
    let Some((result_addr, result_size, result_type)) = &stated.output else {
        return;
    };
    if *result_size != 4 || result_type.get_metatype() != type_metatype::TYPE_FLOAT {
        return;
    }
    let inputs: Vec<_> = stated
        .inputs
        .iter()
        .filter(|(a, s, t)| {
            *s == 8
                && t.get_metatype() == type_metatype::TYPE_FLOAT
                && a == result_addr
                && call.proto().model().input().get_entry().iter().any(|e| {
                    e.get_type() == type_class::TYPECLASS_FLOAT && e.justified_contain(a, *s) == 0
                })
        })
        .map(|(a, s, _)| (a.clone(), *s))
        .collect();
    for (addr, size) in inputs {
        if data
            .get_call_specs(index)
            .active_input()
            .which_trial(&addr, size)
            >= 0
        {
            continue;
        }
        data.get_call_specs_mut(index)
            .get_active_input()
            .register_trial(&addr, size);
        let vn = data.new_varnode(size, &addr, None);
        let slot = data.obank().get(op).map_or(1, |o| o.num_input());
        let _ = data.op_insert_input(op, vn, slot);
    }
}

/// The s-register words the callee's recovered prototype states.
fn stated_words(data: &Funcdata, call: &FuncCallSpecs) -> Vec<Address> {
    if !data.get_arch().arm_float_args || call.is_input_locked() || !call.proto().has_model() {
        return Vec::new();
    }
    let Some(stated) = data
        .kuna_protoorder_types(call.get_entry_address())
        .filter(|s| s.arity_sound)
    else {
        return Vec::new();
    };
    let entries = call.proto().model().input().get_entry();
    stated
        .inputs
        .iter()
        .filter(|(a, s, _)| {
            *s == 4
                && entries.iter().any(|e| {
                    e.get_type() == type_class::TYPECLASS_FLOAT
                        && e.get_size() == 4
                        && e.justified_contain(a, 4) == 0
                })
        })
        .map(|(a, _, _)| a.clone())
        .collect()
}

/// The callee's stated s-register parameters inside a d-register range the call
/// may not take whole, so the call reads each word it states.
pub fn stated_singles(
    data: &Funcdata,
    call: &FuncCallSpecs,
    addr: &Address,
    size: i32,
) -> Vec<Address> {
    if size != 8 {
        return Vec::new();
    }
    stated_words(data, call)
        .into_iter()
        .filter(|a| addr.justified_contain(size, a, 4, false) >= 0)
        .collect()
}

/// Keep a hole the callee states as s-register words from taking a whole
/// d-register.
pub fn mark_single_floats(call: &mut FuncCallSpecs, data: &Funcdata) {
    if data.get_arch().arm_float_args {
        let words = stated_words(data, call);
        call.get_active_input().set_single_floats(words);
    }
}

/// A word of a stated floating parameter is not that parameter.
pub fn partial_float(data: &Funcdata, stated_size: i32, stated: &Datatype, size: i32) -> bool {
    data.get_arch().arm_float_args
        && size < stated_size
        && stated.get_metatype() == type_metatype::TYPE_FLOAT
}

fn single_entry(data: &Funcdata, addr: &Address) -> bool {
    data.get_func_proto()
        .model()
        .input()
        .get_entry()
        .iter()
        .any(|e| {
            e.get_type() == type_class::TYPECLASS_FLOAT
                && e.get_size() == 4
                && e.justified_contain(addr, 4) == 0
        })
}

/// Replace an eight-byte VFP input by its two s-register words.
fn split_input(data: &mut Funcdata, vn: VarnodeId) -> Option<()> {
    let addr = data.vbank().get(vn)?.get_addr().clone();
    let high = &addr + 4;
    let (lo_addr, hi_addr) = if addr.is_big_endian() {
        (high, addr.clone())
    } else {
        (addr.clone(), high)
    };
    let block = data.bblocks_get_block(0);
    let start = data.bblocks_block_start(block);
    let piece = data.new_op(2, start);
    data.op_set_opcode_code(piece, OpCode::CPUI_PIECE);
    let whole = data.new_varnode_out(8, &addr, piece).ok()?;
    data.op_insert_begin(piece, block);
    data.total_replace(vn, whole).ok()?;
    data.vbank_mut().destroy(vn).ok()?;
    let lo = data.new_varnode(4, &lo_addr, None);
    let lo = data.set_input_varnode(lo).ok()?;
    let hi = data.new_varnode(4, &hi_addr, None);
    let hi = data.set_input_varnode(hi).ok()?;
    data.op_set_input(piece, hi, 0).ok()?;
    data.op_set_input(piece, lo, 1).ok()?;
    read_words(data, whole, [lo, hi], 0, 4);
    if data.vbank().get(whole).is_some_and(|w| w.has_no_descend()) {
        data.op_destroy(piece);
    }
    Some(())
}

/// Point each word read of `vn` (shifted right by `shift` bytes) at its input.
fn read_words(data: &mut Funcdata, vn: VarnodeId, words: [VarnodeId; 2], shift: u64, depth: u32) {
    if depth == 0 {
        return;
    }
    for reader in data.descend_snapshot(vn) {
        let Some(op) = data.obank().get(reader) else {
            continue;
        };
        let Some(out) = op.get_out() else {
            continue;
        };
        let constant = op
            .get_in(1)
            .and_then(|c| data.vbank().get(c))
            .filter(|c| c.is_constant())
            .map(|c| c.get_offset());
        let out_size = data.vbank().get(out).map_or(0, |o| o.get_size());
        match (op.code(), constant) {
            (OpCode::CPUI_SUBPIECE, Some(offset)) if out_size == 4 => {
                let word = match offset + shift {
                    0 => words[0],
                    4 => words[1],
                    _ => continue,
                };
                data.op_remove_input(reader, 1);
                if data.op_set_input(reader, word, 0).is_ok() {
                    data.op_set_opcode_code(reader, OpCode::CPUI_COPY);
                }
                continue;
            }
            (OpCode::CPUI_INT_RIGHT, Some(32)) if shift == 0 && op.get_in(0) == Some(vn) => {
                read_words(data, out, words, 4, depth - 1);
            }
            (OpCode::CPUI_COPY | OpCode::CPUI_CAST, _) => {
                read_words(data, out, words, shift, depth - 1);
            }
            _ => continue,
        }
        if data.vbank().get(out).is_some_and(|o| o.has_no_descend()) {
            data.op_destroy(reader);
        }
    }
}

/// A d-register input read only as its two words holds the two s-register inputs.
fn split_word_inputs(data: &mut Funcdata) {
    let inputs: Vec<VarnodeId> = data
        .vbank()
        .iter_def_flag(varnode_flags::input)
        .filter(|&vn| {
            data.vbank().get(vn).is_some_and(|v| {
                v.get_size() == 8
                    && !v.has_no_descend()
                    && single_entry(data, v.get_addr())
                    && single_entry(data, &(v.get_addr() + 4))
            }) && crate::kuna_armfloatreturn::halves_only(data, vn, 4)
        })
        .collect();
    for vn in inputs {
        split_input(data, vn);
    }
}

fn input_storage(data: &Funcdata, vn: VarnodeId, budget: &mut usize) -> Option<(Address, i32)> {
    if *budget == 0 {
        return None;
    }
    *budget -= 1;
    let value = data.vbank().get(vn)?;
    if value.is_input() {
        return Some((value.get_addr().clone(), value.get_size()));
    }
    let op = data.obank().get(value.get_def()?)?;
    match op.code() {
        OpCode::CPUI_COPY | OpCode::CPUI_CAST => {
            let source = input_storage(data, op.get_in(0)?, budget)?;
            (source.1 == value.get_size()).then_some(source)
        }
        OpCode::CPUI_SUBPIECE => {
            let source = input_storage(data, op.get_in(0)?, budget)?;
            let offset = data.vbank().get(op.get_in(1)?)?;
            if !offset.is_constant()
                || offset.get_offset().checked_add(value.get_size() as u64)? > source.1 as u64
            {
                return None;
            }
            let displacement = if source.0.is_big_endian() {
                source.1 as u64 - offset.get_offset() - value.get_size() as u64
            } else {
                offset.get_offset()
            };
            Some((&source.0 + displacement as i64, value.get_size()))
        }
        OpCode::CPUI_PIECE => {
            let hi = input_storage(data, op.get_in(0)?, budget)?;
            let lo = input_storage(data, op.get_in(1)?, budget)?;
            let (first, second) = if lo.0.is_big_endian() {
                (hi, lo)
            } else {
                (lo, hi)
            };
            (first.1 + second.1 == value.get_size() && &first.0 + i64::from(first.1) == second.0)
                .then_some((first.0, value.get_size()))
        }
        _ => None,
    }
}

/// Heritage can split an incoming d-register around an overlapping s-register result.
pub fn is_forwarded_input(data: &Funcdata, vn: VarnodeId, addr: &Address, size: i32) -> bool {
    data.get_arch().arm_float_args
        && size == 8
        && data.get_func_proto().has_model()
        && data
            .get_func_proto()
            .model()
            .input()
            .get_entry()
            .iter()
            .any(|e| {
                e.get_type() == type_class::TYPECLASS_FLOAT && e.justified_contain(addr, size) == 0
            })
        && input_storage(data, vn, &mut 32).is_some_and(|(a, s)| a == *addr && s == size)
}

fn typed_inputs(data: &Funcdata) -> Vec<(Address, i32)> {
    let mut out = Vec::new();
    for op in data
        .obank()
        .iter_alive()
        .filter_map(|id| data.obank().get(id))
    {
        let floating = matches!(
            op.code(),
            OpCode::CPUI_FLOAT_EQUAL
                | OpCode::CPUI_FLOAT_NOTEQUAL
                | OpCode::CPUI_FLOAT_LESS
                | OpCode::CPUI_FLOAT_LESSEQUAL
                | OpCode::CPUI_FLOAT_NAN
                | OpCode::CPUI_FLOAT_ADD
                | OpCode::CPUI_FLOAT_DIV
                | OpCode::CPUI_FLOAT_MULT
                | OpCode::CPUI_FLOAT_SUB
                | OpCode::CPUI_FLOAT_NEG
                | OpCode::CPUI_FLOAT_ABS
                | OpCode::CPUI_FLOAT_SQRT
                | OpCode::CPUI_FLOAT_FLOAT2FLOAT
                | OpCode::CPUI_FLOAT_TRUNC
                | OpCode::CPUI_FLOAT_CEIL
                | OpCode::CPUI_FLOAT_FLOOR
                | OpCode::CPUI_FLOAT_ROUND
        );
        if floating {
            for slot in 0..op.num_input() {
                if let Some(storage) = op
                    .get_in(slot)
                    .and_then(|vn| input_storage(data, vn, &mut 32))
                {
                    out.push(storage);
                }
            }
        }
    }
    for i in 0..data.num_calls() {
        let call = data.get_call_specs(i);
        let Some(op) = data.obank().get(call.get_op()).filter(|o| !o.is_dead()) else {
            continue;
        };
        for slot in 1..op.num_input() {
            if let Some(storage) = op
                .get_in(slot)
                .and_then(|vn| input_storage(data, vn, &mut 32))
            {
                let declared = call.proto().get_param(slot - 1).is_some_and(|p| {
                    p.get_size() == storage.1
                        && p.get_type()
                            .is_some_and(|t| t.get_metatype() == type_metatype::TYPE_FLOAT)
                });
                let recovered = data
                    .kuna_protoorder_types(call.get_entry_address())
                    .is_some_and(|s| {
                        s.arity_sound
                            && s.inputs.iter().any(|(a, z, t)| {
                                a == &storage.0
                                    && *z == storage.1
                                    && t.get_metatype() == type_metatype::TYPE_FLOAT
                            })
                    });
                if declared || recovered {
                    out.push(storage);
                }
            }
        }
    }
    out
}

/// Rejoin word-sized input pieces only when a floating reader consumes their whole value.
pub fn prepare_inputs(data: &mut Funcdata) {
    if !data.get_arch().arm_float_args
        || data.get_func_proto().is_input_locked()
        || !data.get_func_proto().has_model()
    {
        return;
    }
    split_word_inputs(data);
    let candidates = typed_inputs(data);
    for (addr, size) in candidates {
        if !matches!(size, 4 | 8)
            || !data
                .get_func_proto()
                .model()
                .input()
                .get_entry()
                .iter()
                .any(|e| {
                    e.get_type() == type_class::TYPECLASS_FLOAT
                        && e.justified_contain(&addr, size) == 0
                })
        {
            continue;
        }
        let mut input = data.find_varnode_input(size, &addr);
        if input.is_none() && size == 8 {
            let high = &addr + 4;
            if let (Some(first), Some(second)) = (
                data.find_varnode_input(4, &addr),
                data.find_varnode_input(4, &high),
            ) {
                let (hi, lo) = if addr.is_big_endian() {
                    (first, second)
                } else {
                    (second, first)
                };
                if data.combine_input_varnodes(hi, lo).is_ok() {
                    input = data.find_varnode_input(size, &addr);
                }
            }
        }
        if let Some(vn) = input {
            if let Some(ty) = data
                .get_arch()
                .types()
                .and_then(|t| t.get_base(size, type_metatype::TYPE_FLOAT).ok())
            {
                data.vn_update_type(vn, ty);
            }
        }
    }
}

/// A resolved variadic format call states its parameters in base AAPCS storage.
pub fn base_inputs(arch: &Architecture, pieces: &mut PrototypePieces) -> bool {
    if !applies(arch) {
        return false;
    }
    let Some(model) = arch.get_model("__stdcall_softfp") else {
        return false;
    };
    let mut storage: Vec<ParameterPieces> = Vec::new();
    if model
        .assign_parameter_storage(pieces, &mut storage, true, arch.types(), arch.manage())
        .is_err()
        || storage.len() != pieces.intypes.len() + 1
    {
        return false;
    }
    for parameter in storage.iter_mut().skip(1) {
        if parameter
            .addr
            .get_space()
            .is_some_and(|s| s.get_type() == kuna_base::space::spacetype::IPTR_JOIN)
        {
            let Ok(join) = arch.manage().find_join(parameter.addr.get_offset()) else {
                return false;
            };
            if join.num_pieces() != 2 {
                return false;
            }
            let high = join.get_piece(0);
            let low = join.get_piece(1);
            let Some(space) = high
                .space
                .as_ref()
                .filter(|s| s.get_type() == kuna_base::space::spacetype::IPTR_PROCESSOR)
            else {
                return false;
            };
            if low.space.as_ref().map(|s| s.get_index()) != Some(space.get_index())
                || high.size != 4
                || low.size != 4
            {
                return false;
            }
            let (first, second) = if space.is_big_endian() {
                (high, low)
            } else {
                (low, high)
            };
            if first.offset.checked_add(4) != Some(second.offset) {
                return false;
            }
            parameter.addr = Address::new(space.clone(), first.offset);
        }
        parameter.flags |= crate::fspec::parameter_pieces_flags::CUSTOM_STORAGE;
    }
    pieces.input_storage = storage
        .into_iter()
        .skip(1)
        .enumerate()
        .map(|(i, p)| (i as i32, p))
        .collect();
    true
}
