use super::*;
use crate::context::ArchContext;
use kuna_base::address::Address;
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::space::{spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace};
use kuna_sleigh::globalcontext::ContextInternal;
use kuna_sleigh::loadimage::LoadImage;
use kuna_sleigh::sleigh::Sleigh;

struct Image;
impl LoadImage for Image {
    fn get_file_name(&self) -> &str {
        "unit"
    }
    fn load_fill(&mut self, _: &mut [u8], _: &Address) -> KunaResult<()> {
        Err(KunaError::data_unavail("unit"))
    }
    fn get_arch_type(&self) -> Vec<u8> {
        Vec::new()
    }
    fn adjust_vma(&mut self, _: i64) {}
}
fn architecture() -> Architecture {
    let mut arch = Architecture::new(
        "MIPS:BE:64:default",
        Sleigh::new(Box::new(Image), Box::new(ContextInternal::new())),
    );
    arch.build_typegrp();
    arch.types_impl().setup_sizes(Some(4), 8, 8);
    arch.types_impl().set_default_alignment_map();
    arch.build_core_types().unwrap();
    arch.wrap_add = true;
    arch
}
fn function() -> Funcdata {
    let mut manager = AddrSpaceManager::new();
    manager.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    manager
        .insert_space(Rc::new(UniqueSpace::new(1, 0, false)))
        .unwrap();
    let ram = Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "ram",
        false,
        8,
        1,
        2,
        0,
        1,
        1,
    ));
    manager.insert_space(ram.clone()).unwrap();
    Funcdata::new(
        "shift",
        "shift",
        Rc::new(ArchContext::new(manager)),
        Address::new(ram, 0x1000),
        0x10000000,
        0x100,
    )
    .unwrap()
}
fn operation(
    fd: &mut Funcdata,
    code: OpCode,
    inputs: &[VarnodeId],
    size: i32,
) -> (OpId, VarnodeId) {
    let op = fd.new_op(inputs.len() as i32, fd.get_address().clone());
    fd.op_set_opcode(op, crate::typeop::type_op_for(code));
    for (slot, &value) in inputs.iter().enumerate() {
        fd.op_set_input(op, value, slot as i32).unwrap();
    }
    let out = fd.new_unique(
        size,
        Some(Rc::new(Datatype::new(size, type_metatype::TYPE_INT))),
    );
    fd.op_set_output(op, out).unwrap();
    (op, out)
}
#[test]
fn word_wrapping_requires_a_wider_sign_extension_and_signed_printing() {
    let arch = architecture();
    let mut fd = function();
    let value = fd.new_unique(4, Some(Rc::new(Datatype::new(4, type_metatype::TYPE_UINT))));
    let one = fd.new_constant(4, 1);
    let (add, sum) = operation(&mut fd, OpCode::CPUI_INT_ADD, &[value, one], 4);
    let signed = |_| Rc::new(Datatype::new(4, type_metatype::TYPE_INT));
    assert!(plan(&fd, &arch, add, signed).is_none());
    let (_, copied) = operation(&mut fd, OpCode::CPUI_COPY, &[sum], 4);
    let (extension, extended) = operation(&mut fd, OpCode::CPUI_INT_SEXT, &[copied], 8);
    let zero = fd.new_constant(8, 0);
    let ret = fd.new_op(2, fd.get_address().clone());
    fd.op_set_opcode_code(ret, OpCode::CPUI_RETURN);
    fd.op_set_input(ret, zero, 0).unwrap();
    fd.op_set_input(ret, extended, 1).unwrap();
    assert!(plan(&fd, &arch, add, signed).is_some());
    assert!(
        plan(&fd, &arch, add, |_| Rc::new(Datatype::new(
            4,
            type_metatype::TYPE_UINT
        )))
        .is_none()
    );
    fd.op_set_opcode(extension, crate::typeop::type_op_for(OpCode::CPUI_INT_ZEXT));
    assert!(plan(&fd, &arch, add, signed).is_none());
}

#[test]
fn widened_addresses_and_already_unsigned_words_keep_their_existing_expression() {
    let arch = architecture();
    let mut fd = function();
    let value = fd.new_unique(4, Some(Rc::new(Datatype::new(4, type_metatype::TYPE_INT))));
    let one = fd.new_constant(4, 1);
    let (add, sum) = operation(&mut fd, OpCode::CPUI_INT_ADD, &[value, one], 4);
    let (_, extended) = operation(&mut fd, OpCode::CPUI_INT_SEXT, &[sum], 8);
    let ram = fd.get_address().get_space().unwrap().clone();
    let space = fd.new_varnode_space(&ram);
    let (_, loaded) = operation(&mut fd, OpCode::CPUI_LOAD, &[space, extended], 8);
    let zero = fd.new_constant(8, 0);
    let ret = fd.new_op(2, fd.get_address().clone());
    fd.op_set_opcode_code(ret, OpCode::CPUI_RETURN);
    fd.op_set_input(ret, zero, 0).unwrap();
    fd.op_set_input(ret, loaded, 1).unwrap();
    let signed = |_| Rc::new(Datatype::new(4, type_metatype::TYPE_INT));
    assert!(plan(&fd, &arch, add, signed).is_none());
    fd.op_set_input(ret, extended, 1).unwrap();
    assert!(plan(&fd, &arch, add, signed).is_some());
    fd.vbank_mut().get_mut(one).unwrap().set_unsigned_print();
    assert!(plan(&fd, &arch, add, signed).is_none());
}

#[test]
fn the_option_gates_the_wrapping_form() {
    let mut arch = architecture();
    let mut fd = function();
    let value = fd.new_unique(4, Some(Rc::new(Datatype::new(4, type_metatype::TYPE_INT))));
    let one = fd.new_constant(4, 1);
    let (add, sum) = operation(&mut fd, OpCode::CPUI_INT_ADD, &[value, one], 4);
    let (_, extended) = operation(&mut fd, OpCode::CPUI_INT_SEXT, &[sum], 8);
    let zero = fd.new_constant(8, 0);
    let ret = fd.new_op(2, fd.get_address().clone());
    fd.op_set_opcode_code(ret, OpCode::CPUI_RETURN);
    fd.op_set_input(ret, zero, 0).unwrap();
    fd.op_set_input(ret, extended, 1).unwrap();
    let signed = |_| Rc::new(Datatype::new(4, type_metatype::TYPE_INT));
    assert!(plan(&fd, &arch, add, signed).is_some());
    arch.wrap_add = false;
    assert!(plan(&fd, &arch, add, signed).is_none());
}
