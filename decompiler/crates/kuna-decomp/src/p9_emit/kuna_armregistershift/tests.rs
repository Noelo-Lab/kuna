//! Repeatability gates for ARM register-controlled shift emission.

use super::{operand_is_repeatable, plan_arm_register_lsl};
use crate::architecture::Architecture;
use crate::context::{ArchContext, OpId, TypeOp, VarnodeId};
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;
use kuna_base::address::Address;
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;
use kuna_sleigh::globalcontext::ContextInternal;
use kuna_sleigh::loadimage::LoadImage;
use kuna_sleigh::sleigh::Sleigh;
use std::rc::Rc;

struct DummyImg;

impl LoadImage for DummyImg {
    fn get_file_name(&self) -> &str {
        "dummy"
    }

    fn load_fill(&mut self, _ptr: &mut [u8], _addr: &Address) -> KunaResult<()> {
        Err(KunaError::data_unavail("dummy"))
    }

    fn get_arch_type(&self) -> Vec<u8> {
        Vec::new()
    }

    fn adjust_vma(&mut self, _adjust: i64) {}
}

fn arm_arch() -> Architecture {
    let mut arch = Architecture::new(
        "ARM:LE:32:v5t:default",
        Sleigh::new(Box::new(DummyImg), Box::new(ContextInternal::new())),
    );
    arch.build_typegrp();
    arch.types_impl().setup_sizes(Some(4), 4, 4);
    arch.types_impl().set_default_alignment_map();
    arch.build_core_types().unwrap();
    arch.arm_shift_guard = crate::p9_emit::kuna_armshiftguard::ShiftGuardMode::Always;
    arch
}

fn build_fd() -> (Funcdata, Rc<AddrSpace>) {
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
        addrspace_flags::hasphysical,
        1,
        1,
    ));
    manager.insert_space(Rc::clone(&ram)).unwrap();
    let register = Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "register",
        false,
        8,
        1,
        3,
        addrspace_flags::hasphysical,
        1,
        1,
    ));
    manager.insert_space(Rc::clone(&register)).unwrap();
    let global = Rc::new(ArchContext::new(manager));
    let fd = Funcdata::new(
        "shift",
        "shift",
        global,
        Address::new(ram, 0x1000),
        0x1000,
        0x100,
    )
    .unwrap();
    (fd, register)
}

fn unknown4() -> Rc<Datatype> {
    Rc::new(Datatype::new(4, type_metatype::TYPE_UNKNOWN))
}

fn new_value(fd: &mut Funcdata, register: &Rc<AddrSpace>, offset: u64) -> VarnodeId {
    fd.new_varnode(
        4,
        &Address::new(Rc::clone(register), offset),
        Some(unknown4()),
    )
}

fn ram(fd: &Funcdata) -> Rc<AddrSpace> {
    Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap())
}

fn new_op(fd: &mut Funcdata, inputs: int4, offset: u64, code: OpCode) -> OpId {
    let ram = ram(fd);
    let op = fd.new_op(inputs, Address::new(ram, offset));
    fd.obank_mut()
        .change_opcode(op, TypeOp::new(code, 0, format!("{code:?}")));
    op
}

fn produced_value(
    fd: &mut Funcdata,
    register: &Rc<AddrSpace>,
    code: OpCode,
    num_inputs: int4,
    op_offset: u64,
    value_offset: u64,
    implied: bool,
    explicit: bool,
) -> VarnodeId {
    let op = new_op(fd, num_inputs, op_offset, code);
    for slot in 0..num_inputs {
        let input = new_value(fd, register, 0x100 + value_offset + slot as u64 * 4);
        fd.op_set_input(op, input, slot).unwrap();
    }
    let output = new_value(fd, register, value_offset);
    fd.op_set_output(op, output).unwrap();
    let output = fd.obank().get(op).unwrap().get_out().unwrap();
    let varnode = fd.vbank_mut().get_mut(output).unwrap();
    if implied {
        varnode.set_implied();
    }
    if explicit {
        varnode.set_explicit();
    }
    output
}

fn plan_for_count(
    fd: &mut Funcdata,
    arch: &Architecture,
    register: &Rc<AddrSpace>,
    count: VarnodeId,
    op_offset: u64,
) -> bool {
    let lhs = new_value(fd, register, 0x800 + op_offset);
    let op = new_op(fd, 2, op_offset, OpCode::CPUI_INT_LEFT);
    fd.op_set_input(op, lhs, 0).unwrap();
    fd.op_set_input(op, count, 1).unwrap();
    let output = new_value(fd, register, 0x900 + op_offset);
    fd.op_set_output(op, output).unwrap();
    plan_arm_register_lsl(fd, arch, op).is_some()
}

#[test]
fn volatile_unwritten_persistent_leaf_is_rejected() {
    let (mut fd, register) = build_fd();
    let arch = arm_arch();
    let count = new_value(&mut fd, &register, 0x20);
    fd.vbank_mut()
        .get_mut(count)
        .unwrap()
        .set_flags_pub(
            crate::varnode::varnode_flags::persist | crate::varnode::varnode_flags::volatil,
        );

    let value = fd.vbank().get(count).unwrap();
    assert!(!value.is_written());
    assert!(value.is_persist());
    assert!(value.is_volatile());
    assert!(!operand_is_repeatable(&fd, count, 16));
    assert!(!plan_for_count(&mut fd, &arch, &register, count, 0x10));
}

#[test]
fn implied_load_and_call_counts_are_rejected() {
    for (code, input_count) in [(OpCode::CPUI_LOAD, 2), (OpCode::CPUI_CALL, 1)] {
        let (mut fd, register) = build_fd();
        let arch = arm_arch();
        let count = produced_value(
            &mut fd,
            &register,
            code,
            input_count,
            0x10,
            0x20,
            true,
            false,
        );

        assert!(!operand_is_repeatable(&fd, count, 16), "{code:?}");
        assert!(
            !plan_for_count(&mut fd, &arch, &register, count, 0x30),
            "{code:?}"
        );
    }
}

#[test]
fn explicit_materialized_call_and_pure_arithmetic_counts_are_accepted() {
    let (mut fd, register) = build_fd();
    let arch = arm_arch();
    let explicit_call = produced_value(
        &mut fd,
        &register,
        OpCode::CPUI_CALL,
        1,
        0x10,
        0x20,
        false,
        true,
    );
    assert!(operand_is_repeatable(&fd, explicit_call, 16));
    assert!(plan_for_count(
        &mut fd,
        &arch,
        &register,
        explicit_call,
        0x30,
    ));

    let arithmetic = produced_value(
        &mut fd,
        &register,
        OpCode::CPUI_INT_ADD,
        2,
        0x40,
        0x50,
        true,
        false,
    );
    assert!(operand_is_repeatable(&fd, arithmetic, 16));
    assert!(plan_for_count(
        &mut fd,
        &arch,
        &register,
        arithmetic,
        0x60,
    ));
}

fn masked_count(fd: &mut Funcdata, register: &Rc<AddrSpace>, mask: u64, offset: u64) -> VarnodeId {
    let op = new_op(fd, 2, offset, OpCode::CPUI_INT_AND);
    let value = new_value(fd, register, 0x100 + offset);
    fd.op_set_input(op, value, 0).unwrap();
    let constant = fd.new_constant(4, mask);
    fd.op_set_input(op, constant, 1).unwrap();
    let output = new_value(fd, register, offset);
    fd.op_set_output(op, output).unwrap();
    let output = fd.obank().get(op).unwrap().get_out().unwrap();
    fd.vbank_mut().get_mut(output).unwrap().set_implied();
    output
}

#[test]
fn armshiftguard_guards_only_a_count_that_can_reach_32() {
    use crate::p9_emit::kuna_armshiftguard::ShiftGuardMode::{Always, Needed, Never, Wide};
    for (mode, mask, guarded) in [
        (Always, 0x1f, true),
        (Needed, 0x1f, false),
        (Needed, 0x7, false),
        (Needed, 0x20, true),
        (Needed, 0x3f, true),
        (Needed, 0xff, true),
        (Wide, 0x1f, false),
        (Wide, 0xff, false),
        (Never, 0xff, false),
    ] {
        let (mut fd, register) = build_fd();
        let mut arch = arm_arch();
        arch.arm_shift_guard = mode;
        let count = masked_count(&mut fd, &register, mask, 0x40);
        assert_eq!(
            plan_for_count(&mut fd, &arch, &register, count, 0x60),
            guarded,
            "{mode:?} & {mask:#x}"
        );
    }
}

#[test]
fn armshiftguard_values_parse() {
    use crate::p9_emit::kuna_armshiftguard::{OptionArmShiftGuard, ShiftGuardMode};
    for (text, mode) in [
        ("always", ShiftGuardMode::Always),
        ("needed", ShiftGuardMode::Needed),
        ("wide", ShiftGuardMode::Wide),
        ("never", ShiftGuardMode::Never),
    ] {
        assert_eq!(OptionArmShiftGuard.apply(text).unwrap().0, mode);
        assert_eq!(mode.as_str(), text);
    }
    assert!(OptionArmShiftGuard.apply("on").is_err());
}
