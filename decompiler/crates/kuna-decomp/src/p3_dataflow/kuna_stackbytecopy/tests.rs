use std::rc::Rc;

use kuna_num::opbehavior::{
    OpBehavior, OpBehaviorCopy, OpBehaviorDefault, OpBehaviorIntAdd, OpBehaviorIntSub,
    OpBehaviorNotEqual,
};

use crate::context::ArchContext;
use crate::p6_variables::funcdata_spacebase::spacebase_tests::{build_fd, make_sp_in, reg_addr};

use super::*;

fn emit(
    fd: &mut Funcdata,
    block: BlockId,
    code: OpCode,
    inputs: &[VarnodeId],
    output: Option<(i32, u64)>,
) -> OpId {
    let op = fd.new_op(inputs.len() as i32, reg_addr(fd, 0x1000));
    fd.op_set_opcode_code(op, code);
    for (slot, &input) in inputs.iter().enumerate() {
        fd.op_set_input(op, input, slot as i32).unwrap();
    }
    if let Some((size, offset)) = output {
        fd.new_varnode_out(size, &reg_addr(fd, offset), op).unwrap();
    }
    fd.op_insert(op, block, None);
    op
}

fn free_register(fd: &mut Funcdata, size: i32, offset: u64) -> VarnodeId {
    fd.new_varnode(size, &reg_addr(fd, offset), None)
}

#[test]
fn loop_rewrite_reports_a_change_and_preserves_per_iteration_dataflow() {
    let original = build_fd();
    let manager = Rc::clone(&original.get_arch().manage);
    let entry_address = original.get_address().clone();
    drop(original);
    let mut arch = ArchContext::new_shared(manager);
    arch.stack_store_guard = true;
    arch.opbehaviors = vec![None; OpCode::CPUI_MAX as usize];
    let behaviors: Vec<Rc<dyn OpBehavior>> = vec![
        Rc::new(OpBehaviorCopy),
        Rc::new(OpBehaviorIntAdd),
        Rc::new(OpBehaviorIntSub),
        Rc::new(OpBehaviorNotEqual),
        Rc::new(OpBehaviorDefault::new_special(
            OpCode::CPUI_LOAD,
            false,
            true,
        )),
    ];
    for behavior in behaviors {
        let code = behavior.get_opcode();
        arch.opbehaviors[code as usize] = Some(behavior);
    }
    let mut fd = Funcdata::new("copy", "copy", Rc::new(arch), entry_address, 0x1000, 0x40).unwrap();
    let sp = make_sp_in(&mut fd);
    let root = fd.bblocks_root_pub();
    let entry = fd.bblocks_mut().new_block_basic(root);
    let body = fd.bblocks_mut().new_block_basic(root);
    let exit = fd.bblocks_mut().new_block_basic(root);
    for (index, block) in [entry, body, exit].into_iter().enumerate() {
        fd.bblocks_mut().block_mut(block).set_index(index as i32);
    }
    fd.bblocks_mut().add_edge(entry, body);
    fd.bblocks_mut().add_edge(body, exit);
    fd.bblocks_mut().add_edge(body, body);
    fd.bblocks_mut().set_start_block(root, entry);

    let two = fd.new_constant(8, 2);
    emit(&mut fd, entry, OpCode::CPUI_COPY, &[two], Some((8, 0x100)));
    emit(&mut fd, entry, OpCode::CPUI_COPY, &[sp], Some((8, 0x108)));
    let source = free_register(&mut fd, 8, 0x110);
    let source = fd.set_input_varnode(source).unwrap();
    let memory_index = fd
        .get_arch()
        .manage()
        .get_stack_space()
        .unwrap()
        .get_contain()
        .unwrap()
        .get_index();
    let memory = fd.new_constant(4, memory_index as u64);
    let load = emit(
        &mut fd,
        body,
        OpCode::CPUI_LOAD,
        &[memory, source],
        Some((1, 0x118)),
    );
    let loaded = fd.obank().get(load).unwrap().get_out().unwrap();
    let destination = free_register(&mut fd, 8, 0x108);
    emit(
        &mut fd,
        body,
        OpCode::CPUI_STORE,
        &[memory, destination, loaded],
        None,
    );
    let one = fd.new_constant(8, 1);
    let destination = free_register(&mut fd, 8, 0x108);
    emit(
        &mut fd,
        body,
        OpCode::CPUI_INT_ADD,
        &[destination, one],
        Some((8, 0x108)),
    );
    let counter = free_register(&mut fd, 8, 0x100);
    let decrement = emit(
        &mut fd,
        body,
        OpCode::CPUI_INT_SUB,
        &[counter, one],
        Some((8, 0x100)),
    );
    let decremented = fd.obank().get(decrement).unwrap().get_out().unwrap();
    let zero = fd.new_constant(8, 0);
    let compare = emit(
        &mut fd,
        body,
        OpCode::CPUI_INT_NOTEQUAL,
        &[decremented, zero],
        Some((1, 0x120)),
    );
    let condition = fd.obank().get(compare).unwrap().get_out().unwrap();
    let target = fd.new_constant(8, 0x1000);
    emit(
        &mut fd,
        body,
        OpCode::CPUI_CBRANCH,
        &[target, condition],
        None,
    );
    let final_use = emit(
        &mut fd,
        exit,
        OpCode::CPUI_COPY,
        &[decremented],
        Some((8, 0x128)),
    );

    assert_eq!(trip_count(&fd, body), Some(2));
    let mut action = ActionStackByteCopy::boxed();
    assert_eq!(action.perform(&mut fd, &mut ActionContext::default()), 1);
    assert_eq!(fd.bblocks_ref().block(body).size_in(), 1);
    assert_eq!(fd.bblocks_ref().block(body).size_out(), 1);
    assert_eq!(fd.bblocks_ref().block(body).get_out(0), exit);
    let operations = fd.bb_ops(body);
    assert_eq!(operations.len(), 10);
    for iteration in operations.chunks_exact(5) {
        let load = fd.obank().get(iteration[0]).unwrap();
        let store = fd.obank().get(iteration[1]).unwrap();
        assert_eq!(load.code(), OpCode::CPUI_LOAD);
        assert_eq!(store.code(), OpCode::CPUI_STORE);
        assert_eq!(store.get_in(2), load.get_out());
        let decrement = fd.obank().get(iteration[3]).unwrap();
        let compare = fd.obank().get(iteration[4]).unwrap();
        assert_eq!(compare.get_in(0), decrement.get_out());
    }
    let final_decrement = fd.obank().get(operations[8]).unwrap().get_out();
    assert_eq!(
        fd.obank().get(final_use).unwrap().get_in(0),
        final_decrement
    );
    assert_eq!(action.perform(&mut fd, &mut ActionContext::default()), 0);
}
