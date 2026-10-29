use super::*;

#[test]
fn option_name_and_message() {
    assert_eq!(OptionCalleePopFrame::NAME, "calleepopframe");
    let (on, msg) = OptionCalleePopFrame.apply("on").unwrap();
    assert!(on);
    assert_eq!(msg, "Fixed-frame callee-pop reading turned on");
    let (off, msg) = OptionCalleePopFrame.apply("off").unwrap();
    assert!(!off);
    assert_eq!(msg, "Fixed-frame callee-pop reading turned off");
    assert!(OptionCalleePopFrame.apply("maybe").is_err());
}


fn frame_guess(stores: &[(i32, i32)], adjustment: i32) -> Option<i32> {
    use std::rc::Rc;
    use kuna_base::space::{spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace};
    use crate::context::ArchContext;
    use crate::dtype::{type_metatype, Datatype};

    let mut spaces = AddrSpaceManager::new();
    spaces.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    spaces.insert_space(Rc::new(UniqueSpace::new(1, 0, false))).unwrap();
    let ram = Rc::new(AddrSpace::new(spacetype::IPTR_PROCESSOR, "ram", false, 4, 1, 2, 0, 1, 1));
    let reg = Rc::new(AddrSpace::new(spacetype::IPTR_PROCESSOR, "register", false, 4, 1, 3, 0, 1, 1));
    spaces.insert_space(ram.clone()).unwrap();
    spaces.insert_space(reg.clone()).unwrap();
    let arch = Rc::new(ArchContext::new(spaces));
    let mut fd = Funcdata::new("frame", "frame", arch, Address::new(ram.clone(), 0x1000), 0, 0x100).unwrap();
    let root = fd.bblocks_ref().root.unwrap();
    let block = fd.bblocks_mut().new_block_basic(root);
    let sp = Address::new(reg, 0x20);
    let ty = Rc::new(Datatype::new(4, type_metatype::TYPE_UNKNOWN));
    let base = fd.vbank_mut().create(4, sp.clone(), ty.clone());
    let base = fd.vbank_mut().set_input(base, &mut |_, _, _| Ok(())).unwrap();
    let mut address = 0x1000;
    let mut op = |fd: &mut Funcdata, code, inputs: &[VarnodeId], output: Option<Address>| {
        address += 4;
        let op = fd.new_op(inputs.len() as i32, Address::new(ram.clone(), address));
        fd.op_set_opcode(op, crate::typeop::type_op_for(code));
        for (slot, &input) in inputs.iter().enumerate() {
            fd.op_set_input(op, input, slot as i32).unwrap();
        }
        let out = output.map(|at| {
            let v = fd.vbank_mut().create(4, at, ty.clone());
            fd.op_set_output(op, v).unwrap();
            v
        });
        fd.op_insert(op, block, None);
        (op, out)
    };
    for &(offset, width) in stores {
        let k = fd.new_constant(4, offset as u32 as u64);
        let (_, pointer) = op(&mut fd, OpCode::CPUI_INT_ADD, &[base, k], Some(sp.clone()));
        let space = fd.new_constant(4, 2);
        let value = fd.new_constant(width, 0);
        op(&mut fd, OpCode::CPUI_STORE, &[space, pointer.unwrap(), value], None);
    }
    let k = fd.new_constant(4, (-4i32) as u32 as u64);
    let (_, call_sp) = op(&mut fd, OpCode::CPUI_INT_ADD, &[base, k], Some(sp.clone()));
    let call_sp = call_sp.unwrap();
    let marker = fd.new_constant(4, 0);
    let (_, call_out) = op(&mut fd, OpCode::CPUI_INDIRECT, &[call_sp, marker], Some(sp.clone()));
    let call_out = call_out.unwrap();
    let target = fd.new_constant(4, 0x2000);
    let (call, _) = op(&mut fd, OpCode::CPUI_CALL, &[target], None);
    let k = fd.new_constant(4, adjustment as u32 as u64);
    op(&mut fd, OpCode::CPUI_INT_ADD, &[call_out, k], Some(sp.clone()));
    guess_extra_pop(&fd, &sp, call, call_sp, call_out)
}

#[test]
fn a_narrow_store_writes_its_slot() {
    assert_eq!(frame_guess(&[(0, 1)], -4), Some(8));
    assert_eq!(frame_guess(&[(0, 2)], -4), Some(8));
    assert_eq!(frame_guess(&[(0, 5)], -8), Some(12));
    assert_eq!(frame_guess(&[(0, 2), (3, 1)], -4), Some(8));
    assert_eq!(frame_guess(&[(2, 1)], -4), Some(8));
}

#[test]
fn padded_small_aggregates_prove_their_slots() {
    assert_eq!(frame_guess(&[(0, 1), (4, 4)], -8), Some(12));
    assert_eq!(frame_guess(&[(0, 2), (4, 4)], -8), Some(12));
    assert_eq!(frame_guess(&[(2, 1), (0, 2), (4, 4)], -8), Some(12));
    assert_eq!(frame_guess(&[(0, 4), (4, 2), (8, 4)], -12), Some(16));
}

#[test]
fn complete_and_split_stores_prove_their_slots() {
    assert_eq!(frame_guess(&[(0, 4)], -4), Some(8));
    assert_eq!(frame_guess(&[(0, 8)], -8), Some(12));
    assert_eq!(frame_guess(&[(0, 2), (2, 2)], -4), Some(8));
    assert_eq!(frame_guess(&[(0, 2), (1, 3)], -4), Some(8));
    assert_eq!(frame_guess(&[(-1, 5)], -4), Some(8));
}

#[test]
fn an_overlapping_aggregate_copy_proves_its_slots() {
    assert_eq!(frame_guess(&[(3, 4), (0, 4), (8, 4)], -12), Some(16));
}

#[test]
fn an_unwritten_slot_leaves_the_pop_unknown() {
    assert_eq!(frame_guess(&[], -4), None);
    assert_eq!(frame_guess(&[(0, 4)], -8), None);
    assert_eq!(frame_guess(&[(4, 4)], -8), None);
    assert_eq!(frame_guess(&[(0, 1), (8, 4)], -12), None);
    assert_eq!(frame_guess(&[(-4, 4)], -4), None);
    assert_eq!(frame_guess(&[(4, 1)], -4), None);
}

#[test]
fn implausible_stack_adjustments_decline_without_overflow() {
    for adjustment in [i32::MIN, -260, -3, 0, 4, i32::MAX] {
        assert_eq!(frame_guess(&[(0, 4)], adjustment), None);
    }
}
