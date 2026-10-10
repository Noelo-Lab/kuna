use super::*;
use crate::context::ArchContext;
use crate::dtype::Datatype;
use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};
use std::rc::Rc;

fn function(big: bool) -> Funcdata {
    let mut manager = AddrSpaceManager::new();
    manager.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    manager
        .insert_space(Rc::new(UniqueSpace::new(1, 0, big)))
        .unwrap();
    let ram = Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "ram",
        big,
        8,
        1,
        2,
        addrspace_flags::hasphysical,
        1,
        1,
    ));
    manager.insert_space(ram.clone()).unwrap();
    Funcdata::new(
        "test",
        "test",
        Rc::new(ArchContext::new(manager)),
        Address::new(ram, 0x1000),
        0x10000000,
        0x40,
    )
    .unwrap()
}

fn operation(
    fd: &mut Funcdata,
    code: OpCode,
    inputs: &[VarnodeId],
    size: i32,
) -> (OpId, VarnodeId) {
    let address = fd.get_address().clone();
    let op = fd.new_op(inputs.len() as i32, address);
    fd.op_set_opcode_code(op, code);
    fd.op_set_all_input(op, inputs).unwrap();
    let out = fd.new_unique_out(size, op).unwrap();
    (op, out)
}

fn input(fd: &mut Funcdata, meta: type_metatype, size: i32) -> VarnodeId {
    let ram = fd
        .get_arch()
        .manage()
        .get_space_by_name("ram")
        .unwrap()
        .clone();
    let value = fd.vbank_mut().create(
        size,
        Address::new(ram, 0x2000),
        Rc::new(Datatype::new(size, meta)),
    );
    fd.vbank_mut()
        .set_input(value, &mut |_, _, _| Ok(()))
        .unwrap()
}

fn pattern(
    fd: &mut Funcdata,
    unary: OpCode,
    high_offset: u64,
    low_offset: u64,
    different: bool,
    meta: type_metatype,
) -> (OpId, VarnodeId) {
    let source = input(fd, meta, 8);
    let other = if different {
        fd.new_unique(8, None)
    } else {
        source
    };
    let high_shift = fd.new_constant(4, high_offset);
    let low_shift = fd.new_constant(4, low_offset);
    let (_, high) = operation(fd, OpCode::CPUI_SUBPIECE, &[source, high_shift], 4);
    let (_, low) = operation(fd, OpCode::CPUI_SUBPIECE, &[other, low_shift], 4);
    let (_, negated) = operation(fd, unary, &[high], 4);
    let (piece, _) = operation(fd, OpCode::CPUI_PIECE, &[negated, low], 8);
    (piece, source)
}

#[test]
fn restores_the_double_negation_in_both_byte_orders() {
    for big in [false, true] {
        let mut fd = function(big);
        let (piece, source) = pattern(
            &mut fd,
            OpCode::CPUI_FLOAT_NEG,
            4,
            0,
            false,
            type_metatype::TYPE_FLOAT,
        );
        assert!(recover(&mut fd, piece));
        let op = fd.obank().get(piece).unwrap();
        assert_eq!(op.code(), OpCode::CPUI_FLOAT_NEG);
        assert_eq!(op.num_input(), 1);
        assert_eq!(op.get_in(0), Some(source));
        assert!(!recover(&mut fd, piece));
    }
}

#[test]
fn refuses_different_sources_wrong_slices_and_nonfloat_values() {
    for (high, low, different, meta) in [
        (0, 0, false, type_metatype::TYPE_FLOAT),
        (4, 4, false, type_metatype::TYPE_FLOAT),
        (4, 0, true, type_metatype::TYPE_FLOAT),
        (4, 0, false, type_metatype::TYPE_UINT),
        (4, 0, false, type_metatype::TYPE_UNKNOWN),
    ] {
        let mut fd = function(false);
        let (piece, _) = pattern(&mut fd, OpCode::CPUI_FLOAT_NEG, high, low, different, meta);
        assert!(!recover(&mut fd, piece));
        assert_eq!(fd.obank().get(piece).unwrap().code(), OpCode::CPUI_PIECE);
    }
}

#[test]
fn refuses_a_float_type_narrower_than_its_storage() {
    let mut fd = function(false);
    let (piece, source) = pattern(
        &mut fd,
        OpCode::CPUI_FLOAT_NEG,
        4,
        0,
        false,
        type_metatype::TYPE_FLOAT,
    );
    fd.vbank_mut()
        .get_mut(source)
        .unwrap()
        .update_type(Rc::new(Datatype::new(4, type_metatype::TYPE_FLOAT)));
    assert!(!recover(&mut fd, piece));
    assert_eq!(fd.obank().get(piece).unwrap().code(), OpCode::CPUI_PIECE);
}

#[test]
fn restores_absolute_value_in_both_byte_orders() {
    for big in [false, true] {
        let mut fd = function(big);
        let (piece, source) = pattern(
            &mut fd,
            OpCode::CPUI_FLOAT_ABS,
            4,
            0,
            false,
            type_metatype::TYPE_FLOAT,
        );
        assert!(recover(&mut fd, piece));
        let op = fd.obank().get(piece).unwrap();
        assert_eq!(op.code(), OpCode::CPUI_FLOAT_ABS);
        assert_eq!(op.num_input(), 1);
        assert_eq!(op.get_in(0), Some(source));
        assert!(!recover(&mut fd, piece));
    }
}

#[test]
fn absolute_value_refuses_mismatched_shapes_in_both_byte_orders() {
    for big in [false, true] {
        for (high, low, different, meta) in [
            (0, 0, false, type_metatype::TYPE_FLOAT),
            (4, 4, false, type_metatype::TYPE_FLOAT),
            (4, 0, true, type_metatype::TYPE_FLOAT),
            (4, 0, false, type_metatype::TYPE_UINT),
            (4, 0, false, type_metatype::TYPE_UNKNOWN),
        ] {
            let mut fd = function(big);
            let (piece, _) = pattern(&mut fd, OpCode::CPUI_FLOAT_ABS, high, low, different, meta);
            assert!(!recover(&mut fd, piece));
            assert_eq!(fd.obank().get(piece).unwrap().code(), OpCode::CPUI_PIECE);
        }
        let mut fd = function(big);
        let (piece, source) = pattern(
            &mut fd,
            OpCode::CPUI_FLOAT_ABS,
            4,
            0,
            false,
            type_metatype::TYPE_FLOAT,
        );
        fd.vbank_mut()
            .get_mut(source)
            .unwrap()
            .update_type(Rc::new(Datatype::new(4, type_metatype::TYPE_FLOAT)));
        assert!(!recover(&mut fd, piece));
        assert_eq!(fd.obank().get(piece).unwrap().code(), OpCode::CPUI_PIECE);
    }
}

#[test]
fn other_unary_float_operations_are_not_sign_operations() {
    for opcode in [
        OpCode::CPUI_FLOAT_SQRT,
        OpCode::CPUI_FLOAT_CEIL,
        OpCode::CPUI_FLOAT_FLOOR,
    ] {
        let mut fd = function(false);
        let (piece, _) = pattern(&mut fd, opcode, 4, 0, false, type_metatype::TYPE_FLOAT);
        assert!(!recover(&mut fd, piece));
        assert_eq!(fd.obank().get(piece).unwrap().code(), OpCode::CPUI_PIECE);
    }
}

#[test]
fn refuses_slices_that_are_not_the_sign_word_of_a_wider_float() {
    for unary in [OpCode::CPUI_FLOAT_NEG, OpCode::CPUI_FLOAT_ABS] {
        let mut fd = function(false);
        let source = input(&mut fd, type_metatype::TYPE_FLOAT, 16);
        let high_shift = fd.new_constant(4, 4);
        let low_shift = fd.new_constant(4, 0);
        let (_, high) = operation(&mut fd, OpCode::CPUI_SUBPIECE, &[source, high_shift], 4);
        let (_, low) = operation(&mut fd, OpCode::CPUI_SUBPIECE, &[source, low_shift], 4);
        let (_, changed) = operation(&mut fd, unary, &[high], 4);
        let (piece, _) = operation(&mut fd, OpCode::CPUI_PIECE, &[changed, low], 8);
        assert!(!recover(&mut fd, piece));
        assert_eq!(fd.obank().get(piece).unwrap().code(), OpCode::CPUI_PIECE);
    }
}
