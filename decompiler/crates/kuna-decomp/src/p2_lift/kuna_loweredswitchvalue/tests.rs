//! Tests for the lowered-switch dispatch value check: naming a value across a
//! restart, the proof that a `BRANCHIND` input is the recorded value, the
//! verification state machine, and the raw head-compare operand walk.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace};
use kuna_num::opcodes::OpCode;

use super::*;
use crate::context::{ArchContext, BlockId, TypeOp, VarnodeId};
use crate::jumptable::JumpTable;
use crate::p2_lift::kuna_loweredswitch::{new_shared_store, KunaLoweredSwitchRecord};

fn build_manager() -> AddrSpaceManager {
    let mut m = AddrSpaceManager::new();
    m.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    m.insert_space(Rc::new(UniqueSpace::new(1, 0, false))).unwrap();
    m.insert_space(Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "ram",
        false,
        8,
        1,
        2,
        addrspace_flags::hasphysical,
        1,
        1,
    )))
    .unwrap();
    m.insert_space(Rc::new(AddrSpace::new(spacetype::IPTR_PROCESSOR, "regbe", true, 8, 1, 3, 0, 1, 1)))
        .unwrap();
    m
}

fn build_fd_with(check: bool) -> Funcdata {
    let mut ctx = ArchContext::new(build_manager());
    ctx.lowered_switch_value_check = check;
    let glb = Rc::new(ctx);
    let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
    Funcdata::new("func", "func", glb, Address::new(ram, 0x1000), 0x10000000, 0x40).unwrap()
}

fn build_fd() -> Funcdata {
    build_fd_with(true)
}

fn addr(fd: &Funcdata, space: &str, off: u64) -> Address {
    Address::new(Rc::clone(fd.get_arch().manage().get_space_by_name(space).unwrap()), off)
}

fn ram(fd: &Funcdata, off: u64) -> Address {
    addr(fd, "ram", off)
}

fn new_block(fd: &mut Funcdata, off: u64) -> BlockId {
    let root = fd.bblocks_root_pub();
    let bl = fd.bblocks_mut().new_block_basic(root);
    let a = ram(fd, off);
    fd.set_basic_block_range(bl, &a, &a);
    bl
}

fn input(fd: &mut Funcdata, space: &str, off: u64, size: int4) -> VarnodeId {
    let a = addr(fd, space, off);
    let vn = fd.new_varnode(size, &a, None);
    fd.set_input_varnode(vn).unwrap()
}

/// `out = code(inputs)` at `at`, appended to `bl`; `out` is `(space, offset, size)`
/// or a temporary of `size` when `space` is empty.
fn op(fd: &mut Funcdata, bl: BlockId, at: u64, code: OpCode, inputs: &[VarnodeId], out: (&str, u64, int4)) -> VarnodeId {
    let o = fd.new_op(inputs.len() as int4, ram(fd, at));
    fd.op_set_opcode(o, TypeOp::new(code, 0, "op"));
    for (i, &vn) in inputs.iter().enumerate() {
        fd.op_set_input(o, vn, i as int4).unwrap();
    }
    let res = if out.0.is_empty() {
        fd.new_unique_out(out.2, o).unwrap()
    } else {
        let a = addr(fd, out.0, out.1);
        fd.new_varnode_out(out.2, &a, o).unwrap()
    };
    fd.op_insert_end(o, bl);
    res
}

fn named(fd: &Funcdata, vn: VarnodeId) -> ValueName {
    match observe(fd, vn) {
        Observed::Named(n) => n,
        other => panic!("expected a name, got {other:?}"),
    }
}

#[test]
fn an_input_read_through_copies_and_low_truncation_is_named_by_its_storage() {
    let mut fd = build_fd();
    let bl = new_block(&mut fd, 0x100);
    let rdi = input(&mut fd, "ram", 0x38, 8);
    let zero = fd.new_constant(4, 0);
    let edi = op(&mut fd, bl, 0x100, OpCode::CPUI_SUBPIECE, &[rdi, zero], ("ram", 0x38, 4));
    let copy = op(&mut fd, bl, 0x104, OpCode::CPUI_COPY, &[edi], ("ram", 0x0, 4));
    let name = named(&fd, copy);
    assert_eq!(name.def, ValueDef::Input);
    assert_eq!((name.offset, name.size), (0x38, 8));
    let narrow = ValueName { size: 4, ..name.clone() };
    assert!(same_low_bytes(&narrow, &name, 4));
}

#[test]
fn a_zero_extended_narrower_value_is_not_the_wide_value() {
    let mut fd = build_fd();
    let bl = new_block(&mut fd, 0x100);
    let ecx = input(&mut fd, "ram", 0x8, 4);
    let zero = fd.new_constant(4, 0);
    let cl = op(&mut fd, bl, 0x100, OpCode::CPUI_SUBPIECE, &[ecx, zero], ("", 0, 1));
    let widened = op(&mut fd, bl, 0x104, OpCode::CPUI_INT_ZEXT, &[cl], ("ram", 0x0, 4));
    assert_eq!(observe(&fd, widened), Observed::Opaque);
    let recorded = named(&fd, ecx);
    assert!(!input_is_recorded_value(&fd, widened, &recorded, 4));
    let wide_of_whole = op(&mut fd, bl, 0x108, OpCode::CPUI_INT_ZEXT, &[ecx], ("ram", 0x10, 8));
    let low = op(&mut fd, bl, 0x10c, OpCode::CPUI_SUBPIECE, &[wide_of_whole, zero], ("ram", 0x18, 4));
    assert!(input_is_recorded_value(&fd, low, &recorded, 4), "the low bytes of a widened value are the value");
}

#[test]
fn a_constant_is_never_the_recorded_value() {
    let mut fd = build_fd();
    let bl = new_block(&mut fd, 0x100);
    let edi = input(&mut fd, "ram", 0x38, 4);
    let recorded = named(&fd, edi);
    let six = fd.new_constant(4, 6);
    assert_eq!(observe(&fd, six), Observed::Constant);
    assert!(!input_is_recorded_value(&fd, six, &recorded, 4));
    let copied = op(&mut fd, bl, 0x100, OpCode::CPUI_COPY, &[six], ("ram", 0x38, 4));
    assert!(!input_is_recorded_value(&fd, copied, &recorded, 4));
}

#[test]
fn a_temporary_root_has_no_name() {
    let mut fd = build_fd();
    let bl = new_block(&mut fd, 0x100);
    let a = input(&mut fd, "ram", 0x0, 4);
    let b = input(&mut fd, "ram", 0x8, 4);
    let sum = op(&mut fd, bl, 0x100, OpCode::CPUI_INT_ADD, &[a, b], ("", 0, 4));
    assert_eq!(observe(&fd, sum), Observed::Opaque);
}

#[test]
fn a_phi_defined_value_is_not_named() {
    // A value merged by SSA construction (a spilled argument's phi) is not
    // restart-stable: the same storage reads as a raw argument after re-heritage,
    // so naming the phi would false-withdraw a correct switch.
    let mut fd = build_fd();
    let bl = new_block(&mut fd, 0x100);
    let a = input(&mut fd, "ram", 0x8, 4);
    let b = input(&mut fd, "ram", 0x10, 4);
    let phi = op(&mut fd, bl, 0x100, OpCode::CPUI_MULTIEQUAL, &[a, b], ("ram", 0x18, 4));
    assert_eq!(observe(&fd, phi), Observed::Merge(phi));
    assert_eq!(ValueCheck::for_value(&fd, phi).name, None);
    let recorded = named(&fd, a);
    assert!(!input_is_recorded_value(&fd, phi, &recorded, 4));
}

#[test]
fn a_phi_is_the_value_only_when_every_input_is() {
    let mut fd = build_fd();
    let bl = new_block(&mut fd, 0x100);
    let edi = input(&mut fd, "ram", 0x38, 4);
    let copy = op(&mut fd, bl, 0x100, OpCode::CPUI_COPY, &[edi], ("ram", 0x0, 4));
    let phi = op(&mut fd, bl, 0x104, OpCode::CPUI_MULTIEQUAL, &[edi, copy], ("ram", 0x0, 4));
    let recorded = named(&fd, edi);
    assert_eq!(classify_input(&fd, phi, &recorded, 4), InputVerdict::Matches);
    let looped = op(&mut fd, bl, 0x108, OpCode::CPUI_MULTIEQUAL, &[edi, edi], ("ram", 0x0, 4));
    let back = fd.vbank().get(looped).and_then(|v| v.get_def()).unwrap();
    fd.op_set_input(back, looped, 1).unwrap();
    assert_eq!(classify_input(&fd, looped, &recorded, 4), InputVerdict::Matches, "a loop back-edge adds nothing");
}

#[test]
fn a_phi_with_one_different_input_is_a_wrong_dispatch() {
    // gcc -O0: `if (a & 1) hv(a); else hv(a + 1); switch (x)` passes each call
    // its argument in EDI, so at the switch EDI is a phi of two call effects.
    let mut fd = build_fd();
    let bl = new_block(&mut fd, 0x100);
    let edi = input(&mut fd, "ram", 0x38, 4);
    let iop = fd.new_constant(8, 0);
    let first = op(&mut fd, bl, 0x200, OpCode::CPUI_INDIRECT, &[edi, iop], ("ram", 0x38, 4));
    let second = op(&mut fd, bl, 0x300, OpCode::CPUI_INDIRECT, &[edi, iop], ("ram", 0x38, 4));
    let phi = op(&mut fd, bl, 0x400, OpCode::CPUI_MULTIEQUAL, &[first, second], ("ram", 0x38, 4));
    let recorded = named(&fd, edi);
    assert_eq!(classify_input(&fd, phi, &recorded, 4), InputVerdict::Differs);
    let mixed = op(&mut fd, bl, 0x404, OpCode::CPUI_MULTIEQUAL, &[edi, first], ("ram", 0x38, 4));
    assert_eq!(classify_input(&fd, mixed, &recorded, 4), InputVerdict::Differs);
    let rbx = input(&mut fd, "ram", 0x18, 4);
    let four = fd.new_constant(4, 4);
    let sum = op(&mut fd, bl, 0x408, OpCode::CPUI_INT_ADD, &[rbx, four], ("unique", 0x500, 4));
    let opaque = op(&mut fd, bl, 0x40c, OpCode::CPUI_MULTIEQUAL, &[edi, sum], ("ram", 0x38, 4));
    assert_eq!(classify_input(&fd, opaque, &recorded, 4), InputVerdict::Unknown, "an unnameable input proves nothing");
}

#[test]
fn values_left_by_calls_at_different_addresses_are_different_values() {
    let mut fd = build_fd();
    let bl = new_block(&mut fd, 0x100);
    let r0 = input(&mut fd, "ram", 0x20, 4);
    let iop = fd.new_constant(8, 0);
    let first = op(&mut fd, bl, 0x200, OpCode::CPUI_INDIRECT, &[r0, iop], ("ram", 0x20, 4));
    let second = op(&mut fd, bl, 0x300, OpCode::CPUI_INDIRECT, &[r0, iop], ("ram", 0x20, 4));
    let recorded = named(&fd, first);
    assert!(!same_low_bytes(&recorded, &named(&fd, second), 4));
    assert_eq!(classify_input(&fd, second, &recorded, 4), InputVerdict::Differs);
    assert_eq!(classify_input(&fd, first, &recorded, 4), InputVerdict::Matches);
}

#[test]
fn a_phi_of_different_values_over_the_recorded_storage_is_withdrawn() {
    let mut fd = build_fd();
    let edi = input(&mut fd, "ram", 0x38, 4);
    let bl = new_block(&mut fd, 0x900);
    let iop = fd.new_constant(8, 0);
    let first = op(&mut fd, bl, 0x900, OpCode::CPUI_INDIRECT, &[edi, iop], ("ram", 0x38, 4));
    let second = op(&mut fd, bl, 0x910, OpCode::CPUI_INDIRECT, &[edi, iop], ("ram", 0x38, 4));
    let phi = op(&mut fd, bl, 0x920, OpCode::CPUI_MULTIEQUAL, &[first, second], ("ram", 0x38, 4));
    let store = install_fake_switch(&mut fd, phi, false);
    store.borrow_mut().record_mut(&fd, 0).unwrap().value.name = Some(named(&fd, edi));
    verify_installed_switch_values(&mut fd, &store);
    assert_eq!(state(&fd, &store).state, ValueState::Withdrawn);
}

#[test]
fn only_a_fallback_or_a_missing_name_reads_the_recorded_storage() {
    let mut fd = build_fd();
    let edi = input(&mut fd, "ram", 0x38, 4);
    let unnamed = ValueCheck::default();
    assert!(unnamed.reads_home(), "nothing could verify a head-operand read");
    let named_check = ValueCheck { name: Some(named(&fd, edi)), ..ValueCheck::default() };
    assert!(!named_check.reads_home());
    assert!(ValueCheck { read_home: true, ..named_check }.reads_home());
}

#[test]
fn a_call_effect_is_named_by_the_call_address_and_storage() {
    let mut fd = build_fd();
    let bl = new_block(&mut fd, 0x100);
    let edi = input(&mut fd, "ram", 0x38, 4);
    let iop = fd.new_constant(8, 0);
    let after = op(&mut fd, bl, 0x200, OpCode::CPUI_INDIRECT, &[edi, iop], ("ram", 0x38, 4));
    let name = named(&fd, after);
    assert_eq!(name.def, ValueDef::Effect);
    assert_eq!(name.at, ram(&fd, 0x200));
    assert!(!same_low_bytes(&named(&fd, edi), &name, 4), "the value after a call is not the input");
}

#[test]
fn big_endian_storage_keeps_its_low_bytes_at_the_end() {
    let mut fd = build_fd();
    let bl = new_block(&mut fd, 0x100);
    let r3 = input(&mut fd, "regbe", 0x18, 8);
    let zero = fd.new_constant(4, 0);
    let low = op(&mut fd, bl, 0x100, OpCode::CPUI_SUBPIECE, &[r3, zero], ("regbe", 0x1c, 4));
    let low_input = ValueName { offset: 0x1c, size: 4, ..named(&fd, r3) };
    assert!(input_is_recorded_value(&fd, low, &low_input, 4));
    let high_bytes = ValueName { offset: 0x18, size: 4, ..named(&fd, r3) };
    assert!(!input_is_recorded_value(&fd, low, &high_bytes, 4));
}

#[test]
fn two_writes_of_one_storage_by_one_instruction_leave_the_name_ambiguous() {
    let mut fd = build_fd();
    let bl = new_block(&mut fd, 0x100);
    let a = input(&mut fd, "ram", 0x0, 4);
    let one = fd.new_constant(4, 1);
    let first = op(&mut fd, bl, 0x100, OpCode::CPUI_INT_ADD, &[a, one], ("ram", 0x10, 4));
    let name = named(&fd, first);
    assert!(name_is_unique(&fd, &name, 4));
    let second = op(&mut fd, bl, 0x100, OpCode::CPUI_INT_ADD, &[first, one], ("ram", 0x10, 4));
    assert!(!name_is_unique(&fd, &name, 4));
    assert!(!input_is_recorded_value(&fd, second, &name, 4));
}

#[test]
fn a_value_of_another_size_than_the_switch_is_not_proven() {
    let mut fd = build_fd();
    let rdi = input(&mut fd, "ram", 0x38, 8);
    let recorded = ValueName { size: 4, ..named(&fd, rdi) };
    assert!(!input_is_recorded_value(&fd, rdi, &recorded, 4));
}

#[test]
fn only_a_withdrawn_record_is_not_installed() {
    // An unnamed switch and an unchecked named one both install (best effort);
    // only a switch a restart proved wrong is withheld.
    let mut check = ValueCheck::default();
    assert!(check.allows_install(), "an unnamed switch installs best-effort");
    let mut fd = build_fd();
    let edi = input(&mut fd, "ram", 0x38, 4);
    check.name = Some(named(&fd, edi));
    assert!(check.allows_install());
    check.state = ValueState::Verified;
    assert!(check.allows_install());
    check.state = ValueState::Withdrawn;
    assert!(!check.allows_install());
}

/// A switch installed at `0x1001` reading `input_vn`, with `rec` as its record.
fn install_fake_switch(fd: &mut Funcdata, input_vn: VarnodeId, read_head_operand: bool) -> crate::p2_lift::kuna_loweredswitch::SharedLoweredSwitchStore {
    let bl = new_block(fd, 0x1000);
    let ind = fd.new_op(1, ram(fd, 0x1001));
    fd.op_set_opcode(ind, TypeOp::new(OpCode::CPUI_BRANCHIND, 0, "BRANCHIND"));
    fd.op_set_input(ind, input_vn, 0).unwrap();
    fd.op_insert_end(ind, bl);
    let mut jt = JumpTable::new(ram(fd, 0x1001));
    jt.set_indirect_op_addr(ind, ram(fd, 0x1001));
    jt.kuna_set_lowered_var(ram(fd, 0x38), 4);
    fd.jumpvec_mut().push(jt);
    let store = new_shared_store();
    let a = |off| ram(fd, off);
    let rec = KunaLoweredSwitchRecord {
        branch_addr: a(0x1001),
        var_addr: a(0x38),
        var_size: 4,
        case_vals: vec![1, 2, 3],
        case_targets: vec![a(0x2000), a(0x2100), a(0x2200)],
        default_target: a(0x3000),
        signed_labels: false,
        value: ValueCheck { read_head_operand, ..ValueCheck::default() },
    };
    store.borrow_mut().push(fd, rec);
    store
}

fn state(fd: &Funcdata, store: &crate::p2_lift::kuna_loweredswitch::SharedLoweredSwitchStore) -> ValueCheck {
    store.borrow().records(fd)[0].value.clone()
}

#[test]
fn a_switch_reading_the_recorded_value_is_verified() {
    let mut fd = build_fd();
    let edi = input(&mut fd, "ram", 0x38, 4);
    let store = install_fake_switch(&mut fd, edi, true);
    store.borrow_mut().record_mut(&fd, 0).unwrap().value.name = Some(named(&fd, edi));
    verify_installed_switch_values(&mut fd, &store);
    let v = state(&fd, &store);
    assert_eq!(v.state, ValueState::Verified);
    assert!(!fd.has_restart_pending());
}

#[test]
fn a_head_operand_that_is_not_the_value_falls_back_to_the_recorded_storage() {
    let mut fd = build_fd();
    let edi = input(&mut fd, "ram", 0x38, 4);
    let esi = input(&mut fd, "ram", 0x30, 4);
    let store = install_fake_switch(&mut fd, esi, true);
    store.borrow_mut().record_mut(&fd, 0).unwrap().value.name = Some(named(&fd, edi));
    verify_installed_switch_values(&mut fd, &store);
    let v = state(&fd, &store);
    assert_eq!(v.state, ValueState::Unchecked);
    assert!(v.read_home);
    assert!(fd.has_restart_pending());
}

#[test]
fn a_second_look_before_the_restart_keeps_the_pending_fallback() {
    // Detection runs again before the restart its first look requested; the
    // same head-operand read must not spend the home-storage retry.
    let mut fd = build_fd();
    let edi = input(&mut fd, "ram", 0x38, 4);
    let esi = input(&mut fd, "ram", 0x30, 4);
    let store = install_fake_switch(&mut fd, esi, true);
    store.borrow_mut().record_mut(&fd, 0).unwrap().value.name = Some(named(&fd, edi));
    verify_installed_switch_values(&mut fd, &store);
    verify_installed_switch_values(&mut fd, &store);
    let v = state(&fd, &store);
    assert_eq!(v.state, ValueState::Unchecked);
    assert!(v.read_home);
    assert!(fd.has_restart_pending());
}

#[test]
fn a_recorded_storage_read_that_is_not_the_value_is_withdrawn() {
    let mut fd = build_fd();
    let edi = input(&mut fd, "ram", 0x38, 4);
    let bl = new_block(&mut fd, 0x900);
    let six = fd.new_constant(4, 6);
    let clobbered = op(&mut fd, bl, 0x900, OpCode::CPUI_COPY, &[six], ("ram", 0x38, 4));
    let store = install_fake_switch(&mut fd, clobbered, false);
    store.borrow_mut().record_mut(&fd, 0).unwrap().value.name = Some(named(&fd, edi));
    verify_installed_switch_values(&mut fd, &store);
    assert_eq!(state(&fd, &store).state, ValueState::Withdrawn);
    assert!(fd.has_restart_pending());
}

#[test]
fn an_unnameable_input_is_left_installed_not_withdrawn() {
    // The BRANCHIND ended up reading a phi whose other input has no name: not
    // proof of a wrong dispatch, so the switch stays as loweredswitch alone would
    // leave it.
    let mut fd = build_fd();
    let edi = input(&mut fd, "ram", 0x38, 4);
    let b = input(&mut fd, "ram", 0x10, 4);
    let store = install_fake_switch(&mut fd, edi, true);
    // point the BRANCHIND at a phi
    let bl = fd.get_jump_table(0).get_indirect_op().and_then(|o| fd.obank().get(o)).and_then(|o| o.get_parent()).unwrap();
    let four = fd.new_constant(4, 4);
    let sum = op(&mut fd, bl, 0x0ff8, OpCode::CPUI_INT_ADD, &[b, four], ("unique", 0x500, 4));
    let phi = op(&mut fd, bl, 0x1000, OpCode::CPUI_MULTIEQUAL, &[edi, sum], ("ram", 0x40, 4));
    let ind = fd.get_jump_table(0).get_indirect_op().unwrap();
    fd.op_set_input(ind, phi, 0).unwrap();
    store.borrow_mut().record_mut(&fd, 0).unwrap().value.name = Some(named(&fd, edi));
    verify_installed_switch_values(&mut fd, &store);
    assert_eq!(state(&fd, &store).state, ValueState::Unchecked, "a phi input is not withdrawn");
    assert!(!fd.has_restart_pending());
}

#[test]
fn a_later_match_in_the_run_restores_the_read_it_verified() {
    // A head-operand read first looks wrong (falls back to home), then matches
    // later in the same run: the verified read must stay the head operand, or the
    // next run would install over the home storage while claiming verification.
    let mut fd = build_fd();
    let edi = input(&mut fd, "ram", 0x38, 4);
    let esi = input(&mut fd, "ram", 0x30, 4);
    let store = install_fake_switch(&mut fd, esi, true);
    store.borrow_mut().record_mut(&fd, 0).unwrap().value.name = Some(named(&fd, edi));
    verify_installed_switch_values(&mut fd, &store);
    assert!(state(&fd, &store).read_home);
    let ind = fd.get_jump_table(0).get_indirect_op().unwrap();
    fd.op_set_input(ind, edi, 0).unwrap();
    verify_installed_switch_values(&mut fd, &store);
    let v = state(&fd, &store);
    assert_eq!(v.state, ValueState::Verified);
    assert!(!v.read_home, "keep reading the head operand that was verified");
    assert_eq!(v.verified_read, Some(true));
}

#[test]
fn an_identity_add_is_the_value_it_copies() {
    // Thumb `adds r5,r2,#0` copies a parameter while setting flags.
    let mut fd = build_fd();
    let bl = new_block(&mut fd, 0x100);
    let r2 = input(&mut fd, "ram", 0x8, 4);
    let zero = fd.new_constant(4, 0);
    let r5 = op(&mut fd, bl, 0x100, OpCode::CPUI_INT_ADD, &[r2, zero], ("ram", 0x14, 4));
    assert!(input_is_recorded_value(&fd, r5, &named(&fd, r2), 4));
    let six = fd.new_constant(4, 6);
    let r6 = op(&mut fd, bl, 0x104, OpCode::CPUI_INT_ADD, &[r2, six], ("ram", 0x18, 4));
    assert!(!input_is_recorded_value(&fd, r6, &named(&fd, r2), 4), "an addend is a different value");
}

#[test]
fn a_constant_input_is_withdrawn_and_a_later_proof_restores_the_switch() {
    let mut fd = build_fd();
    let edi = input(&mut fd, "ram", 0x38, 4);
    let six = fd.new_constant(4, 6);
    let store = install_fake_switch(&mut fd, six, false);
    store.borrow_mut().record_mut(&fd, 0).unwrap().value.name = Some(named(&fd, edi));
    verify_installed_switch_values(&mut fd, &store);
    assert_eq!(state(&fd, &store).state, ValueState::Withdrawn);
    let ind = fd.get_jump_table(0).get_indirect_op().unwrap();
    fd.op_set_input(ind, edi, 0).unwrap();
    verify_installed_switch_values(&mut fd, &store);
    assert_eq!(state(&fd, &store).state, ValueState::Verified);
}

#[test]
fn an_unnamed_switch_is_left_installed_and_never_verified() {
    // No recorded name (a phi selector): the switch dispatches on the head
    // operand best-effort and the value check leaves it alone.
    let mut fd = build_fd();
    let edi = input(&mut fd, "ram", 0x38, 4);
    let store = install_fake_switch(&mut fd, edi, false);
    verify_installed_switch_values(&mut fd, &store);
    assert_eq!(state(&fd, &store).state, ValueState::Unchecked);
    assert!(!fd.has_restart_pending(), "an unnamed switch requests no restart");
}

#[test]
fn the_check_is_inert_when_the_option_is_off() {
    let mut fd = build_fd_with(false);
    let esi = input(&mut fd, "ram", 0x30, 4);
    let store = install_fake_switch(&mut fd, esi, false);
    verify_installed_switch_values(&mut fd, &store);
    assert_eq!(state(&fd, &store).state, ValueState::Unchecked);
    assert!(!fd.has_restart_pending());
}

// -----------------------------------------------------------------------------
// head_compare_operand: raw p-code, reads not linked to writes
// -----------------------------------------------------------------------------

fn free(fd: &mut Funcdata, space: &str, off: u64, size: int4) -> VarnodeId {
    let a = addr(fd, space, off);
    fd.new_varnode(size, &a, None)
}

/// Append `CBRANCH dest, cond` at `at` and return the op.
fn cbranch(fd: &mut Funcdata, bl: BlockId, at: u64, cond: VarnodeId) -> OpId {
    let dest = fd.new_constant(8, 0);
    let cb = fd.new_op(2, ram(fd, at));
    fd.op_set_opcode(cb, TypeOp::new(OpCode::CPUI_CBRANCH, 0, "CBRANCH"));
    fd.op_set_input(cb, dest, 0).unwrap();
    fd.op_set_input(cb, cond, 1).unwrap();
    fd.op_insert_end(cb, bl);
    cb
}

/// `CMP <reg>,5 ; JZ` over `reg` at `ram:reg_off`: a temporary difference, ZF
/// from it, and the branch reading ZF back.
fn cmp_jz(fd: &mut Funcdata, bl: BlockId, reg_off: u64) -> OpId {
    let reg = free(fd, "ram", reg_off, 4);
    let five = fd.new_constant(4, 5);
    let diff = op(fd, bl, 0x1000, OpCode::CPUI_INT_SUB, &[reg, five], ("unique", 0x100, 4));
    let _ = diff;
    let diff_read = free(fd, "unique", 0x100, 4);
    let zero = fd.new_constant(4, 0);
    op(fd, bl, 0x1000, OpCode::CPUI_INT_EQUAL, &[diff_read, zero], ("ram", 0x206, 1));
    let zf = free(fd, "ram", 0x206, 1);
    cbranch(fd, bl, 0x1002, zf)
}

#[test]
fn the_head_operand_is_the_register_the_compare_reads() {
    let mut fd = build_fd();
    let bl = new_block(&mut fd, 0x1000);
    let cb = cmp_jz(&mut fd, bl, 0x18);
    let home = ram(&fd, 0x08);
    assert_eq!(head_compare_operand(&fd, bl, cb, &home, 4), Some(HeadOperand::Storage(ram(&fd, 0x18))));
}

#[test]
fn a_register_written_after_the_compare_is_not_the_head_operand() {
    let mut fd = build_fd();
    let bl = new_block(&mut fd, 0x1000);
    let reg = free(&mut fd, "ram", 0x18, 4);
    let five = fd.new_constant(4, 5);
    op(&mut fd, bl, 0x1000, OpCode::CPUI_INT_SUB, &[reg, five], ("unique", 0x100, 4));
    let diff_read = free(&mut fd, "unique", 0x100, 4);
    let zero = fd.new_constant(4, 0);
    op(&mut fd, bl, 0x1000, OpCode::CPUI_INT_EQUAL, &[diff_read, zero], ("ram", 0x206, 1));
    let nine = fd.new_constant(8, 9);
    op(&mut fd, bl, 0x1001, OpCode::CPUI_COPY, &[nine], ("ram", 0x18, 8));
    let zf = free(&mut fd, "ram", 0x206, 1);
    let cb = cbranch(&mut fd, bl, 0x1002, zf);
    assert_eq!(head_compare_operand(&fd, bl, cb, &ram(&fd, 0x08), 4), None);
}

#[test]
fn a_compare_that_rewrites_its_register_reads_the_rewritten_value() {
    // SUB EAX,5 ; JZ: ZF is computed from EAX after the subtraction, so the
    // operand is EAX as it stands at the branch.
    let mut fd = build_fd();
    let bl = new_block(&mut fd, 0x1000);
    let eax = free(&mut fd, "ram", 0x0, 4);
    let five = fd.new_constant(4, 5);
    op(&mut fd, bl, 0x1000, OpCode::CPUI_INT_SUB, &[eax, five], ("ram", 0x0, 4));
    let eax_after = free(&mut fd, "ram", 0x0, 4);
    let zero = fd.new_constant(4, 0);
    op(&mut fd, bl, 0x1000, OpCode::CPUI_INT_EQUAL, &[eax_after, zero], ("ram", 0x206, 1));
    let zf = free(&mut fd, "ram", 0x206, 1);
    let cb = cbranch(&mut fd, bl, 0x1002, zf);
    assert_eq!(head_compare_operand(&fd, bl, cb, &ram(&fd, 0x38), 4), Some(HeadOperand::Storage(ram(&fd, 0x0))));
}

#[test]
fn a_compare_of_a_zero_extended_register_reads_the_low_register() {
    // MOV EAX,EDI writes all of RAX; CMP EAX,5 reads its low half.
    let mut fd = build_fd();
    let bl = new_block(&mut fd, 0x1000);
    let edi = free(&mut fd, "ram", 0x38, 4);
    op(&mut fd, bl, 0x0ff0, OpCode::CPUI_INT_ZEXT, &[edi], ("ram", 0x0, 8));
    let cb = cmp_jz(&mut fd, bl, 0x0);
    assert_eq!(head_compare_operand(&fd, bl, cb, &ram(&fd, 0x38), 4), Some(HeadOperand::Storage(ram(&fd, 0x0))));
}

#[test]
fn a_loaded_compare_operand_is_the_load() {
    let mut fd = build_fd();
    let bl = new_block(&mut fd, 0x1000);
    let spc = fd.new_constant(8, 2);
    let rbp = free(&mut fd, "ram", 0x28, 8);
    let disp = fd.new_constant(8, 0xffff_ffff_ffff_fffc);
    op(&mut fd, bl, 0x1000, OpCode::CPUI_INT_ADD, &[rbp, disp], ("unique", 0x200, 8));
    let ptr = free(&mut fd, "unique", 0x200, 8);
    let loaded = op(&mut fd, bl, 0x1000, OpCode::CPUI_LOAD, &[spc, ptr], ("unique", 0x300, 4));
    let val = free(&mut fd, "unique", 0x300, 4);
    let three = fd.new_constant(4, 3);
    op(&mut fd, bl, 0x1000, OpCode::CPUI_INT_LESS, &[val, three], ("ram", 0x200, 1));
    let cf = free(&mut fd, "ram", 0x200, 1);
    let cb = cbranch(&mut fd, bl, 0x1002, cf);
    assert_eq!(head_compare_operand(&fd, bl, cb, &ram(&fd, 0x38), 4), Some(HeadOperand::Value(loaded)));
}

#[test]
fn a_compare_of_a_widened_load_reads_the_load() {
    // gcc -O0 `movzwl -4(%rbp),%eax ; cmp $0xea60,%eax` over an unsigned short:
    // the compare reads four bytes that zero-extend the two-byte load.
    let mut fd = build_fd();
    let bl = new_block(&mut fd, 0x1000);
    let spc = fd.new_constant(8, 2);
    let rbp = free(&mut fd, "ram", 0x28, 8);
    let loaded = op(&mut fd, bl, 0x0ff0, OpCode::CPUI_LOAD, &[spc, rbp], ("unique", 0x300, 2));
    let val = free(&mut fd, "unique", 0x300, 2);
    op(&mut fd, bl, 0x0ff0, OpCode::CPUI_INT_ZEXT, &[val], ("ram", 0x0, 8));
    let cb = cmp_jz(&mut fd, bl, 0x0);
    assert_eq!(head_compare_operand(&fd, bl, cb, &ram(&fd, 0x38), 2), Some(HeadOperand::Value(loaded)));
}

#[test]
fn a_compare_of_a_widened_register_reads_the_narrow_register() {
    // `movzwl %di,%eax ; cmp $5,%eax` over a two-byte value reads DI.
    let mut fd = build_fd();
    let bl = new_block(&mut fd, 0x1000);
    let di = free(&mut fd, "ram", 0x38, 2);
    op(&mut fd, bl, 0x0ff0, OpCode::CPUI_INT_SEXT, &[di], ("ram", 0x0, 8));
    let cb = cmp_jz(&mut fd, bl, 0x0);
    assert_eq!(head_compare_operand(&fd, bl, cb, &ram(&fd, 0x38), 2), Some(HeadOperand::Storage(ram(&fd, 0x38))));
    let mut fd = build_fd();
    let bl = new_block(&mut fd, 0x1000);
    let wide = free(&mut fd, "ram", 0x38, 4);
    op(&mut fd, bl, 0x0ff0, OpCode::CPUI_INT_ZEXT, &[wide], ("ram", 0x0, 8));
    let cb = cmp_jz(&mut fd, bl, 0x0);
    assert_eq!(
        head_compare_operand(&fd, bl, cb, &ram(&fd, 0x38), 2),
        None,
        "four compared bytes extended from four bytes are not a two-byte value"
    );
}

#[test]
fn two_compares_of_different_registers_have_no_single_operand() {
    let mut fd = build_fd();
    let bl = new_block(&mut fd, 0x1000);
    let a = free(&mut fd, "ram", 0x0, 4);
    let b = free(&mut fd, "ram", 0x8, 4);
    let five = fd.new_constant(4, 5);
    op(&mut fd, bl, 0x1000, OpCode::CPUI_INT_EQUAL, &[a, five], ("ram", 0x206, 1));
    op(&mut fd, bl, 0x1000, OpCode::CPUI_INT_EQUAL, &[b, five], ("ram", 0x207, 1));
    let zf = free(&mut fd, "ram", 0x206, 1);
    let sf = free(&mut fd, "ram", 0x207, 1);
    let both = op(&mut fd, bl, 0x1001, OpCode::CPUI_BOOL_AND, &[zf, sf], ("unique", 0x400, 1));
    let _ = both;
    let cond = free(&mut fd, "unique", 0x400, 1);
    let cb = cbranch(&mut fd, bl, 0x1002, cond);
    assert_eq!(head_compare_operand(&fd, bl, cb, &ram(&fd, 0x38), 4), None);
}
