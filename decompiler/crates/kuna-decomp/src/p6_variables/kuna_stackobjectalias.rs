//! Named pointer views preserve existing physical frame layouts.

use std::rc::Rc;

use crate::context::VarnodeId;
use crate::dtype::{type_metatype, TypeFactory};
use crate::funcdata::Funcdata;
use crate::kuna_stackobjectasserts::{ObjectAssertion, UseAnchor};
use crate::varmap::LinkEntryInfo;
use kuna_num::opcodes::OpCode;

pub(crate) fn create(
    fd: &mut Funcdata,
    assertion: &ObjectAssertion,
    use_: &UseAnchor,
    info: &LinkEntryInfo,
    width: i32,
    factory: &dyn TypeFactory,
) -> Option<VarnodeId> {
    let scope = fd.get_scope_local()?;
    let space = assertion.address.get_space()?.clone();
    let offset = space.wrap_offset(
        assertion
            .address
            .get_offset()
            .wrapping_sub(info.entry_addr.get_offset()),
    );
    let backing = info.sym_type.as_ref()?;
    if backing.is_incomplete()
        || info.entry_size <= 0
        || backing.get_size() <= 0
        || info.sym_off < 0
        || info.sym_off as u64 != offset
        || offset.checked_add(assertion.size as u64)? > info.entry_size as u64
        || offset.checked_add(assertion.size as u64)? > backing.get_size() as u64
    {
        return None;
    }
    let ty = assertion.datatype.as_ref().unwrap_or(&use_.datatype);
    let target = factory
        .get_type_pointer(width, Rc::clone(ty), space.get_word_size())
        .ok()?;
    let physical = factory
        .get_type_pointer_strip_array(width, Rc::clone(backing), space.get_word_size())
        .ok()?;
    let byte = factory.get_base(1, type_metatype::TYPE_UINT).ok()?;
    let byte_pointer = factory
        .get_type_pointer(width, byte, space.get_word_size())
        .ok()?;
    let stem = format!("{}_view", assertion.view_name(ty));
    let mut name = scope.make_local_name_unique(&stem);
    let mut suffix = 0;
    while fd
        .high_bank()
        .iter()
        .any(|(_, high)| high.kuna_name() == Some(name.as_str()))
    {
        suffix += 1;
        name = format!("{stem}_{suffix}");
    }
    let block = fd.bblocks_get_block(0);
    let point = fd.bblocks_block_start(block);
    let base = fd.construct_spacebase_input(&space).ok()?;
    let mut operations = Vec::new();
    let root = fd.new_op(2, point.clone());
    fd.op_set_opcode_code(root, OpCode::CPUI_PTRSUB);
    let mut value = fd.new_unique_out(width, root).ok()?;
    fd.vn_update_type(value, physical);
    let at = fd.new_constant(width, info.entry_addr.get_offset());
    fd.op_set_all_input(root, &[base, at]).ok()?;
    fd.vbank_mut().get_mut(value)?.set_implied();
    operations.push(root);
    if offset != 0 {
        let cast = fd.new_op(1, point.clone());
        fd.op_set_opcode_code(cast, OpCode::CPUI_CAST);
        let byte_value = fd.new_unique_out(width, cast).ok()?;
        fd.vn_update_type(byte_value, Rc::clone(&byte_pointer));
        fd.op_set_input(cast, value, 0).ok()?;
        fd.vbank_mut().get_mut(byte_value)?.set_implied();
        operations.push(cast);
        let add = fd.new_op(3, point.clone());
        fd.op_set_opcode_code(add, OpCode::CPUI_PTRADD);
        value = fd.new_unique_out(width, add).ok()?;
        fd.vn_update_type(value, byte_pointer);
        let displacement = fd.new_constant(width, offset);
        let scale = fd.new_constant(width, 1);
        fd.op_set_all_input(add, &[byte_value, displacement, scale])
            .ok()?;
        fd.vbank_mut().get_mut(value)?.set_implied();
        operations.push(add);
    }
    let cast = fd.new_op(1, point);
    fd.op_set_opcode_code(cast, OpCode::CPUI_CAST);
    let alias = fd.new_unique_out(width, cast).ok()?;
    fd.vn_update_type_locked(alias, Rc::clone(&target), true, true);
    fd.op_set_input(cast, value, 0).ok()?;
    fd.vbank_mut().get_mut(alias)?.set_explicit();
    fd.vbank_mut()
        .get_mut(alias)?
        .set_flags_pub(crate::varnode::varnode_flags::namelock);
    let high = fd.vbank().get(alias)?.get_high()?;
    fd.high_bank_mut().get_mut(high)?.set_kuna_name(name);
    fd.high_bank_mut().get_mut(high)?.set_symbol_type(target);
    operations.push(cast);
    for op in operations.into_iter().rev() {
        fd.op_insert_begin(op, block);
    }
    let reference = fd.vbank().get(at)?.get_high()?;
    let high = fd.high_bank_mut().get_mut(reference)?;
    high.set_kuna_name(info.display_name.clone());
    high.set_symbol_offset(0);
    high.set_symbol_type(Rc::clone(backing));
    high.set_kuna_ref_symbol(info.symbol);
    Some(alias)
}

pub(crate) fn matches(
    fd: &Funcdata,
    input: VarnodeId,
    assertion: &ObjectAssertion,
    use_: &UseAnchor,
) -> bool {
    let Some(value) = fd.vbank().get(input) else {
        return false;
    };
    let Some(high) = value.get_high().and_then(|high| fd.high_bank().get(high)) else {
        return false;
    };
    let ty = assertion.datatype.as_ref().unwrap_or(&use_.datatype);
    let stem = format!("{}_view", assertion.view_name(ty));
    let named = high
        .kuna_name()
        .is_some_and(|name| name == stem || name.starts_with(&(stem + "_")));
    if !named || !value.is_explicit() {
        return false;
    }
    let Some(pointer) = value.get_type().get_ptr_to() else {
        return false;
    };
    if !Rc::ptr_eq(&pointer, ty) {
        return false;
    }
    let Some(address) = crate::kuna_stackranges::frame_address(fd, input) else {
        return false;
    };
    let Some(space) = assertion.address.get_space() else {
        return false;
    };
    address.first == address.last
        && address.terms.is_empty()
        && space.wrap_offset(address.first as u64) == assertion.address.get_offset()
}
