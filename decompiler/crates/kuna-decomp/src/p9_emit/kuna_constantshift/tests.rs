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
    arch.arm_constant_shift = crate::p9_emit::kuna_armconstantshift::ConstShiftMode::All;
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
fn widened_shift_plans_preserve_shared_operands_and_result_sign() {
    let mut arch = architecture();
    let mut fd = function();
    let address = Address::new(fd.get_address().get_space().unwrap().clone(), 0x100);
    let narrow = fd.new_varnode(
        4,
        &address,
        Some(Rc::new(Datatype::new(4, type_metatype::TYPE_INT))),
    );
    let narrow = fd.set_input_varnode(narrow).unwrap();
    let (extension, wide) = operation(&mut fd, OpCode::CPUI_INT_SEXT, &[narrow], 8);
    fd.vbank_mut().get_mut(wide).unwrap().set_implied();
    let count = fd.new_constant(4, 32);
    let (shift, _) = operation(&mut fd, OpCode::CPUI_INT_LEFT, &[wide, count], 8);
    operation(&mut fd, OpCode::CPUI_COPY, &[wide], 8);
    let readers = fd.vbank().get(wide).unwrap().descend_iter().count();
    assert_eq!(readers, 2);
    for bits in [0, 31, 32, 63, 64] {
        let count = fd.new_constant(4, bits);
        fd.op_set_input(shift, count, 1).unwrap();
        let plan = plan_fixed64_lsl(&fd, &arch, shift, |vn| fd.vn_type_read_facing(vn, shift));
        assert_eq!(plan.is_some(), bits < 64);
        if let Some(plan) = plan {
            assert_eq!(plan.lhs, wide);
            assert_eq!(plan.count, count);
            assert_eq!(plan.unsigned_type.get_size(), 8);
            assert_eq!(plan.unsigned_type.get_metatype(), type_metatype::TYPE_UINT);
            assert_eq!(
                plan.signed_result_type.unwrap().get_metatype(),
                type_metatype::TYPE_INT
            );
        }
        assert_eq!(
            fd.vbank().get(wide).unwrap().descend_iter().count(),
            readers
        );
        assert_eq!(
            fd.obank().get(extension).unwrap().code(),
            OpCode::CPUI_INT_SEXT
        );
    }
    let count = fd.new_constant(4, 32);
    fd.op_set_input(shift, count, 1).unwrap();
    arch.archid = "ARM:LE:64:v8A:default".into();
    assert!(plan_fixed64_lsl(&fd, &arch, shift, |vn| fd.vn_type_read_facing(vn, shift)).is_some());
    arch.archid = "MIPS:BE:64:default".into();
    fd.op_set_opcode(extension, crate::typeop::type_op_for(OpCode::CPUI_INT_ZEXT));
    assert!(plan_fixed64_lsl(&fd, &arch, shift, |vn| fd.vn_type_read_facing(vn, shift)).is_some());
    fd.op_set_opcode(extension, crate::typeop::type_op_for(OpCode::CPUI_INT_SEXT));
    fd.op_set_input(shift, narrow, 1).unwrap();
    assert!(plan_fixed64_lsl(&fd, &arch, shift, |vn| fd.vn_type_read_facing(vn, shift)).is_none());
}

#[test]
fn direct_signed_printed_inputs_do_not_require_a_sign_extension() {
    let arch = architecture();
    let mut fd = function();
    let input = fd.new_unique(8, Some(Rc::new(Datatype::new(8, type_metatype::TYPE_UINT))));
    let count = fd.new_constant(4, 63);
    let (shift, _) = operation(&mut fd, OpCode::CPUI_INT_LEFT, &[input, count], 8);
    let as_signed = |_| Rc::new(Datatype::new(8, type_metatype::TYPE_INT));
    assert!(plan_fixed64_lsl(&fd, &arch, shift, as_signed).is_some());
    assert!(
        plan_fixed64_lsl(&fd, &arch, shift, |_| Rc::new(Datatype::new(
            8,
            type_metatype::TYPE_UINT
        )))
        .is_none()
    );
    let count = fd.new_constant(4, 64);
    fd.op_set_input(shift, count, 1).unwrap();
    assert!(plan_fixed64_lsl(&fd, &arch, shift, as_signed).is_none());
}

#[test]
fn signed_shifts_with_proven_safe_known_bits_keep_the_compact_form() {
    let arch = architecture();
    let mut fd = function();
    let address = Address::new(fd.get_address().get_space().unwrap().clone(), 0x100);
    let value = fd.new_varnode(
        8,
        &address,
        Some(Rc::new(Datatype::new(8, type_metatype::TYPE_INT))),
    );
    let value = fd.set_input_varnode(value).unwrap();
    fd.vbank_mut().get_mut(value).unwrap().set_nz_mask(0xffff);
    for (count, repair) in [(4, false), (47, false), (48, true), (63, true)] {
        let amount = fd.new_constant(4, count);
        let (shift, _) = operation(&mut fd, OpCode::CPUI_INT_LEFT, &[value, amount], 8);
        assert_eq!(
            plan_fixed64_lsl(&fd, &arch, shift, |vn| fd.vn_type_read_facing(vn, shift)).is_some(),
            repair
        );
    }
}

#[test]
fn only_the_all_value_plans_a_64_bit_shift() {
    use crate::p9_emit::kuna_armconstantshift::ConstShiftMode;
    let mut arch = architecture();
    let mut fd = function();
    let input = fd.new_unique(8, Some(Rc::new(Datatype::new(8, type_metatype::TYPE_INT))));
    let count = fd.new_constant(4, 31);
    let (shift, _) = operation(&mut fd, OpCode::CPUI_INT_LEFT, &[input, count], 8);
    let as_signed = |_| Rc::new(Datatype::new(8, type_metatype::TYPE_INT));
    for (mode, planned) in [(ConstShiftMode::Off, false), (ConstShiftMode::On, false), (ConstShiftMode::All, true)] {
        arch.arm_constant_shift = mode;
        assert_eq!(plan_fixed64_lsl(&fd, &arch, shift, as_signed).is_some(), planned, "{mode:?}");
    }
}

/// The two-valued spellings keep their meaning, including the bare `on`.
#[test]
fn option_values_keep_the_two_valued_spellings() {
    use crate::p9_emit::kuna_armconstantshift::{ConstShiftMode, OptionArmConstShift};
    for (value, mode) in [("off", ConstShiftMode::Off), ("on", ConstShiftMode::On), ("", ConstShiftMode::On), ("all", ConstShiftMode::All)] {
        assert_eq!(OptionArmConstShift.apply(value).unwrap().0, mode, "{value:?}");
    }
    assert!(OptionArmConstShift.apply("yes").is_err());
}
