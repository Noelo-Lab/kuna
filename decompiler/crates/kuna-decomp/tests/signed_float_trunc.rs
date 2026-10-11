//! Floating truncation must remain signed before destination coercion.
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, FspecSpace, IopSpace,
    UniqueSpace,
};
use kuna_decomp::context::ArchContext;
use kuna_decomp::dtype::{type_metatype, TypeFactory, TypeFactoryImpl};
use kuna_decomp::funcdata::Funcdata;
use kuna_num::opcodes::OpCode;

fn conversion(
    size: i32,
    destination: type_metatype,
    opcode: OpCode,
) -> (
    Funcdata,
    kuna_decomp::context::OpId,
    kuna_decomp::context::VarnodeId,
) {
    let mut spaces = AddrSpaceManager::new();
    spaces.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    spaces
        .insert_space(Rc::new(UniqueSpace::new(1, 0, false)))
        .unwrap();
    spaces.insert_space(Rc::new(IopSpace::new(2))).unwrap();
    spaces.insert_space(Rc::new(FspecSpace::new(3))).unwrap();
    spaces
        .insert_space(Rc::new(AddrSpace::new(
            spacetype::IPTR_PROCESSOR,
            "ram",
            false,
            8,
            1,
            4,
            addrspace_flags::hasphysical,
            1,
            1,
        )))
        .unwrap();
    let factory = Rc::new(TypeFactoryImpl::new());
    factory.setup_sizes(Some(4), 8, 8);
    factory.set_default_alignment_map();
    factory.set_max_basetype_size(8);
    let mut context = ArchContext::new(spaces);
    context.types = Some(Rc::clone(&factory));
    let context = Rc::new(context);
    let ram = Rc::clone(context.manage().get_space_by_name("ram").unwrap());
    let address = Address::new(Rc::clone(&ram), 0x1000);
    let mut fd = Funcdata::new(
        "convert",
        "convert",
        context,
        address.clone(),
        0x1000_0000,
        0x40,
    )
    .unwrap();
    let root = fd.bblocks_ref().root.unwrap();
    let block = fd.bblocks_mut().new_block_basic(root);
    let op = fd.new_op(1, address);
    fd.op_set_opcode(op, kuna_decomp::typeop::type_op_for(opcode));
    let input_size = if size == 1 { 4 } else { size };
    let input_metatype = if opcode == OpCode::CPUI_FLOAT_INT2FLOAT {
        type_metatype::TYPE_INT
    } else {
        type_metatype::TYPE_FLOAT
    };
    let input = fd.new_unique(
        input_size,
        Some(
            factory
                .get_base(input_size, input_metatype)
                .unwrap(),
        ),
    );
    fd.op_set_input(op, input, 0).unwrap();
    let destination_type = match destination {
        type_metatype::TYPE_ENUM_INT | type_metatype::TYPE_ENUM_UINT => factory
            .get_type_enum_sized("Result", size, destination)
            .unwrap(),
        _ => factory.get_base(size, destination).unwrap(),
    };
    let output = fd.new_varnode(size, &Address::new(ram, 0x3000), Some(destination_type));
    fd.op_set_output(op, output).unwrap();
    let output = fd.obank().get(op).unwrap().get_out().unwrap();
    fd.op_insert(op, block, None);
    (fd, op, output)
}

#[test]
fn truncation_precedes_unsigned_coercion() {
    for size in [4, 8] {
        for destination in [type_metatype::TYPE_UINT, type_metatype::TYPE_ENUM_UINT] {
            let (mut fd, op, output) = conversion(size, destination, OpCode::CPUI_FLOAT_TRUNC);
            let destination_type = Rc::clone(fd.vbank().get(output).unwrap().get_type());
            fd.action_set_casts();
            assert!(Rc::ptr_eq(
                fd.vbank().get(output).unwrap().get_type(),
                &destination_type
            ));
            let signed = fd.obank().get(op).unwrap().get_out().unwrap();
            let signed_type = fd.vbank().get(signed).unwrap().get_type();
            assert_eq!(signed_type.get_metatype(), type_metatype::TYPE_INT);
            assert_eq!(signed_type.get_size(), size);
            let cast = fd.vbank().get(output).unwrap().get_def().unwrap();
            assert_eq!(fd.obank().get(cast).unwrap().code(), OpCode::CPUI_CAST);
            assert_eq!(fd.obank().get(cast).unwrap().get_in(0), Some(signed));
            assert_eq!(
                fd.vbank().get(output).unwrap().get_type().get_metatype(),
                type_metatype::TYPE_UINT
            );
        }
    }
}

#[test]
fn signed_truncation_and_float_conversion_do_not_gain_casts() {
    for size in [4, 8] {
        for (opcode, destination) in [
            (OpCode::CPUI_FLOAT_TRUNC, type_metatype::TYPE_INT),
            (OpCode::CPUI_FLOAT_TRUNC, type_metatype::TYPE_ENUM_INT),
            (OpCode::CPUI_FLOAT_TRUNC, type_metatype::TYPE_UNKNOWN),
            (OpCode::CPUI_FLOAT_FLOAT2FLOAT, type_metatype::TYPE_FLOAT),
            (OpCode::CPUI_FLOAT_INT2FLOAT, type_metatype::TYPE_FLOAT),
        ] {
            let (mut fd, op, output) = conversion(size, destination, opcode);
            fd.action_set_casts();
            assert_eq!(fd.obank().get(op).unwrap().get_out(), Some(output), "{opcode:?} {destination:?} size={size}");
        }
    }
}

#[test]
fn truncation_precedes_one_byte_boolean_coercion() {
    let (mut fd, op, output) = conversion(1, type_metatype::TYPE_BOOL, OpCode::CPUI_FLOAT_TRUNC);
    fd.action_set_casts();
    let signed = fd.obank().get(op).unwrap().get_out().unwrap();
    assert_eq!(
        fd.vbank().get(signed).unwrap().get_type().get_metatype(),
        type_metatype::TYPE_INT
    );
    assert_eq!(fd.vbank().get(signed).unwrap().get_type().get_size(), 1);
    assert_eq!(
        fd.vbank().get(output).unwrap().get_type().get_metatype(),
        type_metatype::TYPE_BOOL
    );
    let cast = fd.vbank().get(output).unwrap().get_def().unwrap();
    assert_eq!(fd.obank().get(cast).unwrap().code(), OpCode::CPUI_CAST);
    assert_eq!(fd.obank().get(cast).unwrap().get_in(0), Some(signed));
}
