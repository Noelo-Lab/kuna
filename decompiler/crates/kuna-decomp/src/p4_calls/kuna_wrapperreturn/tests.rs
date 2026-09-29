use super::*;
use crate::context::ArchContext;
use crate::kuna_passthrough::{returns_tail_result, PassThroughClaim};
use kuna_base::space::{spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace};
use slotmap::Key;
use std::rc::Rc;

fn function(big: bool) -> (Funcdata, Rc<AddrSpace>) {
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
        0,
        0,
        0,
    ));
    let reg = Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "register",
        big,
        8,
        1,
        3,
        0,
        0,
        0,
    ));
    manager.insert_space(ram.clone()).unwrap();
    manager.insert_space(reg.clone()).unwrap();
    let mut arch = ArchContext::new(manager);
    arch.pass_through = true;
    arch.wrapper_return = true;
    let data = Funcdata::new(
        "wrapper",
        "wrapper",
        Rc::new(arch),
        Address::new(ram, 0x1000),
        0x10000000,
        0x40,
    )
    .unwrap();
    (data, reg)
}
fn half(data: &mut Funcdata, addr: &Address, owner: OpId) -> VarnodeId {
    let op = data.new_op(2, data.get_address().clone());
    data.op_set_opcode_code(op, OpCode::CPUI_INDIRECT);
    let zero = data.new_constant(4, 0);
    let reference = data.new_constant(8, owner.data().as_ffi());
    data.op_set_input(op, zero, 0).unwrap();
    data.op_set_input(op, reference, 1).unwrap();
    let value = data.new_varnode_out(4, addr, op).unwrap();
    data.mark_indirect_creation(op, false).unwrap();
    value
}
#[test]
fn split_returns_require_all_bytes_from_the_same_claimed_call() {
    for big in [false, true] {
        let (mut data, reg) = function(big);
        let addr = Address::new(reg.clone(), 0x300);
        let call = data.new_op(0, data.get_address().clone());
        data.op_set_opcode_code(call, OpCode::CPUI_CALL);
        let other = data.new_op(0, data.get_address().clone());
        data.op_set_opcode_code(other, OpCode::CPUI_CALL);
        data.kuna_set_passthrough_claims(vec![PassThroughClaim {
            addr: addr.clone(),
            size: 8,
            arg_owners: Vec::new(),
            ret_owners: vec![call],
            body_touches: false,
        }]);
        let hi_addr = Address::new(reg.clone(), 0x300 + if big { 0 } else { 4 });
        let lo_addr = Address::new(reg, 0x300 + if big { 4 } else { 0 });
        let hi = half(&mut data, &hi_addr, call);
        let lo = half(&mut data, &lo_addr, call);
        let op = data.new_op(2, data.get_address().clone());
        data.op_set_opcode_code(op, OpCode::CPUI_PIECE);
        data.op_set_input(op, hi, 0).unwrap();
        data.op_set_input(op, lo, 1).unwrap();
        let value = data.new_varnode_out(8, &addr, op).unwrap();
        assert!(returns_tail_result(&data, value, &addr, 8));
        let wrong_owner = half(&mut data, &lo_addr, other);
        data.op_set_input(op, wrong_owner, 1).unwrap();
        assert!(!returns_tail_result(&data, value, &addr, 8));
        data.op_set_input(op, lo, 0).unwrap();
        data.op_set_input(op, hi, 1).unwrap();
        assert!(!returns_tail_result(&data, value, &addr, 8));
        data.op_set_opcode_code(op, OpCode::CPUI_COPY);
        data.op_remove_input(op, 1);
        data.op_set_input(op, value, 0).unwrap();
        assert!(!returns_tail_result(&data, value, &addr, 8));
    }
}
