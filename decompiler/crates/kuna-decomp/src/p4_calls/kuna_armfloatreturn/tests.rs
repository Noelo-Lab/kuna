use super::*;
use crate::context::ArchContext;
use kuna_base::space::{spacetype, AddrSpace, ConstantSpace, UniqueSpace};

fn function() -> (Funcdata, Address) {
    let mut manager = AddrSpaceManager::new();
    manager.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    manager
        .insert_space(Rc::new(UniqueSpace::new(1, 0, false)))
        .unwrap();
    let ram = Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "ram",
        false,
        4,
        1,
        2,
        0,
        0,
        0,
    ));
    let reg = Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "register",
        false,
        4,
        1,
        3,
        0,
        0,
        0,
    ));
    manager.insert_space(ram.clone()).unwrap();
    manager.insert_space(reg.clone()).unwrap();
    let data = Funcdata::new(
        "narrow",
        "narrow",
        Rc::new(ArchContext::new(manager)),
        Address::new(ram, 0x1000),
        0x10000000,
        0x40,
    )
    .unwrap();
    (data, Address::new(reg, 0x300))
}

fn output(
    data: &mut Funcdata,
    code: OpCode,
    inputs: &[VarnodeId],
    size: i32,
    addr: &Address,
) -> VarnodeId {
    let op = data.new_op(inputs.len() as i32, data.get_address().clone());
    data.op_set_opcode_code(op, code);
    for (i, &vn) in inputs.iter().enumerate() {
        data.op_set_input(op, vn, i as i32).unwrap();
    }
    data.new_varnode_out(size, addr, op).unwrap()
}

fn partial(data: &mut Funcdata, addr: &Address) -> VarnodeId {
    let old = data.new_varnode(8, addr, None);
    let old = data.set_input_varnode(old).unwrap();
    let offset = data.new_constant(4, 4);
    let high = output(
        data,
        OpCode::CPUI_SUBPIECE,
        &[old, offset],
        4,
        &Address::new(addr.get_space().unwrap().clone(), addr.get_offset() + 4),
    );
    let low = output(data, OpCode::CPUI_FLOAT_FLOAT2FLOAT, &[old], 4, addr);
    output(data, OpCode::CPUI_PIECE, &[high, low], 8, addr)
}

#[test]
fn narrowing_requires_a_low_write_and_stale_upper_bytes() {
    let (mut data, addr) = function();
    let value = partial(&mut data, &addr);
    assert!(partial_return(&data, value, &addr, &mut 64));
    assert!(!partial_return(
        &data,
        value,
        &Address::new(addr.get_space().unwrap().clone(), addr.get_offset() + 8),
        &mut 64
    ));
    let piece = data.vbank().get(value).unwrap().get_def().unwrap();
    let high = data.obank().get(piece).unwrap().get_in(0).unwrap();
    let low = data.obank().get(piece).unwrap().get_in(1).unwrap();
    data.op_set_input(piece, low, 0).unwrap();
    data.op_set_input(piece, high, 1).unwrap();
    assert!(!partial_return(&data, value, &addr, &mut 64));
    data.op_set_input(piece, high, 0).unwrap();
    let integer = data.new_constant(4, 7);
    let integer = output(
        &mut data,
        OpCode::CPUI_INT_ADD,
        &[integer, integer],
        4,
        &addr,
    );
    data.op_set_input(piece, integer, 1).unwrap();
    assert!(partial_return(&data, value, &addr, &mut 64));
}

#[test]
fn complete_doubles_and_mixed_exit_widths_do_not_narrow() {
    let (mut data, addr) = function();
    let value = partial(&mut data, &addr);
    let other = partial(&mut data, &addr);
    let merged = output(
        &mut data,
        OpCode::CPUI_MULTIEQUAL,
        &[value, other],
        8,
        &addr,
    );
    assert!(partial_return(&data, merged, &addr, &mut 64));
    let input = data.new_varnode(4, &addr, None);
    let double = output(
        &mut data,
        OpCode::CPUI_FLOAT_FLOAT2FLOAT,
        &[input],
        8,
        &addr,
    );
    assert!(!partial_return(&data, double, &addr, &mut 64));
    let phi = data.vbank().get(merged).unwrap().get_def().unwrap();
    data.op_set_input(phi, double, 1).unwrap();
    assert!(!partial_return(&data, merged, &addr, &mut 64));
}

#[test]
fn narrowing_proof_is_bounded_and_declines_cycles() {
    let (mut data, addr) = function();
    let value = partial(&mut data, &addr);
    let copy = output(&mut data, OpCode::CPUI_COPY, &[value], 8, &addr);
    assert!(partial_return(&data, copy, &addr, &mut 64));
    assert!(!partial_return(&data, copy, &addr, &mut 1));
    let op = data.vbank().get(copy).unwrap().get_def().unwrap();
    data.op_set_input(op, copy, 0).unwrap();
    assert!(!partial_return(&data, copy, &addr, &mut 64));
}

#[test]
fn constants_loads_and_inputs_do_not_need_float_evidence() {
    let (mut data, addr) = function();
    let floating = partial(&mut data, &addr);
    let opaque = partial(&mut data, &addr);
    let piece = data.vbank().get(opaque).unwrap().get_def().unwrap();
    let constant = data.new_constant(4, 0x3f800000);
    let input_addr = Address::new(addr.get_space().unwrap().clone(), addr.get_offset() + 8);
    let input = data.new_varnode(4, &input_addr, None);
    let input = data.set_input_varnode(input).unwrap();
    let copy = output(&mut data, OpCode::CPUI_COPY, &[input], 4, &addr);
    let space = data.new_constant(4, 2);
    let pointer = data.new_constant(4, 0x1000);
    let load = output(&mut data, OpCode::CPUI_LOAD, &[space, pointer], 4, &addr);
    let merged = output(
        &mut data,
        OpCode::CPUI_MULTIEQUAL,
        &[floating, opaque],
        8,
        &addr,
    );
    for low in [constant, input, copy, load] {
        data.op_set_input(piece, low, 1).unwrap();
        assert!(partial_return(&data, opaque, &addr, &mut 64));
        assert!(partial_return(&data, merged, &addr, &mut 64));
    }
    let phi = data.vbank().get(merged).unwrap().get_def().unwrap();
    data.op_set_input(phi, opaque, 0).unwrap();
    assert!(partial_return(&data, merged, &addr, &mut 64));
    let integer = output(
        &mut data,
        OpCode::CPUI_INT_ADD,
        &[input, constant],
        4,
        &addr,
    );
    data.op_set_input(piece, integer, 1).unwrap();
    data.op_set_input(phi, floating, 0).unwrap();
    assert!(partial_return(&data, merged, &addr, &mut 64));
}

#[test]
fn low_half_joins_check_every_path() {
    let (mut data, addr) = function();
    let value = partial(&mut data, &addr);
    let piece = data.vbank().get(value).unwrap().get_def().unwrap();
    let low = data.obank().get(piece).unwrap().get_in(1).unwrap();
    let constant = data.new_constant(4, 0x3f800000);
    let merged = output(
        &mut data,
        OpCode::CPUI_MULTIEQUAL,
        &[constant, low],
        4,
        &addr,
    );
    data.op_set_input(piece, merged, 1).unwrap();
    assert!(partial_return(&data, value, &addr, &mut 64));
    let phi = data.vbank().get(merged).unwrap().get_def().unwrap();
    data.op_set_input(phi, low, 0).unwrap();
    data.op_set_input(phi, constant, 1).unwrap();
    assert!(partial_return(&data, value, &addr, &mut 64));
    let integer = output(
        &mut data,
        OpCode::CPUI_INT_ADD,
        &[constant, constant],
        4,
        &addr,
    );
    data.op_set_input(phi, integer, 1).unwrap();
    assert!(partial_return(&data, value, &addr, &mut 64));
    data.op_set_input(phi, merged, 1).unwrap();
    assert!(!partial_return(&data, value, &addr, &mut 64));
}

#[test]
fn reassembling_the_same_double_does_not_prove_a_narrow_write() {
    let (mut data, addr) = function();
    let value = partial(&mut data, &addr);
    let piece = data.vbank().get(value).unwrap().get_def().unwrap();
    let high = data.obank().get(piece).unwrap().get_in(0).unwrap();
    let sub = data.vbank().get(high).unwrap().get_def().unwrap();
    let whole = data.obank().get(sub).unwrap().get_in(0).unwrap();
    let zero = data.new_constant(4, 0);
    let low = output(&mut data, OpCode::CPUI_SUBPIECE, &[whole, zero], 4, &addr);
    let copy = output(&mut data, OpCode::CPUI_COPY, &[low], 4, &addr);
    data.op_set_input(piece, high, 1).unwrap();
    assert!(partial_return(&data, value, &addr, &mut 64));
    data.op_set_input(piece, copy, 1).unwrap();
    assert!(!partial_return(&data, value, &addr, &mut 64));
    let constant = data.new_constant(4, 0);
    let merged = output(
        &mut data,
        OpCode::CPUI_MULTIEQUAL,
        &[constant, copy],
        4,
        &addr,
    );
    data.op_set_input(piece, merged, 1).unwrap();
    assert!(partial_return(&data, value, &addr, &mut 64));
}

#[test]
fn predicated_writes_can_carry_the_older_double_through_a_join() {
    let (mut data, addr) = function();
    let value = partial(&mut data, &addr);
    let piece = data.vbank().get(value).unwrap().get_def().unwrap();
    let high = data.obank().get(piece).unwrap().get_in(0).unwrap();
    let sub = data.vbank().get(high).unwrap().get_def().unwrap();
    let old = data.obank().get(sub).unwrap().get_in(0).unwrap();
    let merged = output(&mut data, OpCode::CPUI_MULTIEQUAL, &[old, value], 8, &addr);
    for bits in [
        0, 0x80000000, 0x3f800000, 0x40000000, 0x7fc00000, 0xbf800000,
    ] {
        let low = data.new_constant(4, bits);
        data.op_set_input(piece, low, 1).unwrap();
        assert!(partial_return(&data, merged, &addr, &mut 64));
    }
    let copy = output(&mut data, OpCode::CPUI_COPY, &[merged], 8, &addr);
    assert!(partial_return(&data, copy, &addr, &mut 64));
    let unrelated = data.new_constant(8, 0x3ff0000000000000);
    let op = data.vbank().get(merged).unwrap().get_def().unwrap();
    data.op_set_input(op, unrelated, 0).unwrap();
    assert!(!partial_return(&data, merged, &addr, &mut 64));
}

#[test]
fn a_double_input_read_only_in_halves_holds_two_floats() {
    let (mut data, addr) = function();
    let whole = data.new_varnode(8, &addr, None);
    let whole = data.set_input_varnode(whole).unwrap();
    let zero = data.new_constant(4, 0);
    output(&mut data, OpCode::CPUI_SUBPIECE, &[whole, zero], 4, &addr);
    let cast = output(&mut data, OpCode::CPUI_CAST, &[whole], 8, &addr);
    let shift = data.new_constant(4, 32);
    let high = output(&mut data, OpCode::CPUI_INT_RIGHT, &[cast, shift], 8, &addr);
    let zero = data.new_constant(4, 0);
    output(&mut data, OpCode::CPUI_SUBPIECE, &[high, zero], 4, &addr);
    assert!(halves_only(&data, whole, 4));
    output(&mut data, OpCode::CPUI_FLOAT_ADD, &[whole, whole], 8, &addr);
    assert!(!halves_only(&data, whole, 4));
}

#[test]
fn only_a_value_computed_to_be_returned_retires_a_call_leftover() {
    let (mut data, addr) = function();
    let target = data.new_constant(4, 0x2000);
    let left = output(&mut data, OpCode::CPUI_CALL, &[target], 8, &addr);
    assert!(left_by_call(&data, left));
    let one = data.new_constant(4, 1);
    let computed = output(&mut data, OpCode::CPUI_INT_ADD, &[one, one], 4, &addr);
    assert!(!left_by_call(&data, computed));
    let ret = data.new_op(2, data.get_address().clone());
    data.op_set_opcode_code(ret, OpCode::CPUI_RETURN);
    let code = data.new_constant(4, 0);
    data.op_set_input(ret, code, 0).unwrap();
    data.op_set_input(ret, computed, 1).unwrap();
    assert!(computed_result(&data, computed));
    assert!(!computed_result(&data, left));
    let store = data.new_constant(4, 0x3000);
    output(&mut data, OpCode::CPUI_INT_ADD, &[computed, store], 4, &addr);
    assert!(!computed_result(&data, computed));
}
