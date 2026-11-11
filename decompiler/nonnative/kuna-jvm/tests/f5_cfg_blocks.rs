//! `BasicBlock`, `ExceptionRangeCFG` and `ControlFlowGraph` on hand-built instruction sequences.

use kuna_jvm::basic_block::BasicBlock;
use kuna_jvm::bytecode_version::BytecodeVersion;
use kuna_jvm::code_constants::{
    GROUP_GENERAL, GROUP_INVOCATION, GROUP_JUMP, GROUP_RETURN, GROUP_SWITCH, OPC_ATHROW,
    OPC_BIPUSH, OPC_GOTO, OPC_IFEQ, OPC_ILOAD, OPC_INVOKESTATIC, OPC_IRETURN, OPC_NOP, OPC_POP,
    OPC_RETURN, OPC_TABLESWITCH,
};
use kuna_jvm::control_flow_graph::ControlFlowGraph;
use kuna_jvm::dead_code_helper::DeadCodeHelper;
use kuna_jvm::exception_deobfuscator::ExceptionDeobfuscator;
use kuna_jvm::exception_handler::ExceptionHandler;
use kuna_jvm::exception_range_cfg::ExceptionRangeCFG;
use kuna_jvm::exception_table::ExceptionTable;
use kuna_jvm::full_instruction_sequence::FullInstructionSequence;
use kuna_jvm::i_graph_node::IGraphNode;
use kuna_jvm::instruction::Instruction;
use kuna_jvm::rt::{reset_class_scope, try_catch, JHashMap, JList};

/// (opcode, group, operands, length); start offsets accumulate.
type Op = (i32, i32, &'static [i32], i32);

fn sequence(ops: &[Op], handlers: &[(i32, i32, i32, Option<&str>)]) -> FullInstructionSequence {
    let version = BytecodeVersion::new(65, 0);
    let mut instructions = Vec::new();
    let mut offset_to_index: JHashMap<i32, i32> = JHashMap::new();
    let mut offset = 0;
    for &(opcode, group, operands, length) in ops {
        let operands = (!operands.is_empty()).then(|| operands.to_vec());
        instructions.push(Instruction::create(
            opcode, false, group, version, operands, offset, length,
        ));
        offset_to_index.put(offset, instructions.len() as i32 - 1);
        offset += length;
    }
    let handlers = handlers
        .iter()
        .map(|&(from, to, handler, class)| {
            ExceptionHandler::new(from, to, handler, class.map(str::to_string))
        })
        .collect();
    let seq =
        FullInstructionSequence::new(instructions, offset_to_index, ExceptionTable::new(handlers));
    for i in 0..seq.length() {
        seq.get_instr(i).init_instruction(seq);
    }
    seq
}

fn ids(blocks: JList<BasicBlock>) -> Vec<i32> {
    blocks.iter().map(|b| b.get_id()).collect()
}

fn block(graph: ControlFlowGraph, id: i32) -> BasicBlock {
    graph
        .get_blocks()
        .borrow()
        .get_with_key(&id)
        .expect("block")
}

/// `if (x == 0) return 2; return 1;` with an explicit goto-free layout.
fn branch() -> FullInstructionSequence {
    sequence(
        &[
            (OPC_ILOAD, GROUP_GENERAL, &[0], 1),
            (OPC_IFEQ, GROUP_JUMP, &[6], 3),
            (OPC_BIPUSH, GROUP_GENERAL, &[1], 2),
            (OPC_IRETURN, GROUP_RETURN, &[], 1),
            (OPC_BIPUSH, GROUP_GENERAL, &[2], 2),
            (OPC_IRETURN, GROUP_RETURN, &[], 1),
        ],
        &[],
    )
}

#[test]
fn construction_splits_at_jumps_and_orders_successors() {
    let _context = crate::common::install_default_context();
    reset_class_scope();
    let graph = ControlFlowGraph::new(branch());
    assert_eq!(graph.get_blocks().borrow().len(), 3);
    let (b1, b2, b3) = (block(graph, 1), block(graph, 2), block(graph, 3));
    assert_eq!(graph.get_first(), b1);
    assert_eq!(
        ids(b1.get_succs()),
        vec![3, 2],
        "jump target first, then the fall-through"
    );
    assert_eq!(ids(b2.get_preds()), vec![1]);
    assert_eq!(graph.get_last().get_id(), 4);
    assert_eq!(graph.with(|d| d.last_id), 4);
    assert_eq!(ids(graph.get_last().get_preds()), vec![2, 3]);
    assert!(graph.get_last().get_succs().is_empty());
    assert_eq!(b1.get_instr_old_offsets().to_vec(), vec![0, 1]);
    assert_eq!(
        (b3.get_start_instruction(), b3.get_end_instruction()),
        (7, 10)
    );
    assert_eq!(b2.get_old_offset(1), 6);
    assert_eq!(b2.get_old_offset(2), -1);
    assert_eq!(
        b1.get_last_instruction().unwrap().with(|d| d.opcode),
        OPC_IFEQ
    );

    DeadCodeHelper::connect_dummy_exit_block(graph);
    assert_eq!(ids(b2.get_succs()), vec![4]);
    assert_eq!(ids(graph.get_last().get_preds()), vec![2, 3]);
    let rpo: Vec<i32> = graph
        .get_reverse_post_order()
        .iter()
        .map(|b| b.get_id())
        .collect();
    assert_eq!(rpo, vec![1, 2, 3, 4]);
}

#[test]
fn switch_successors_put_default_first() {
    let _context = crate::common::install_default_context();
    reset_class_scope();
    let graph = ControlFlowGraph::new(sequence(
        &[
            (OPC_ILOAD, GROUP_GENERAL, &[0], 1),
            (OPC_TABLESWITCH, GROUP_SWITCH, &[22, 0, 1, 19, 20], 19),
            (OPC_NOP, GROUP_GENERAL, &[], 1),
            (OPC_RETURN, GROUP_RETURN, &[], 1),
            (OPC_RETURN, GROUP_RETURN, &[], 1),
            (OPC_RETURN, GROUP_RETURN, &[], 1),
        ],
        &[],
    ));
    let b1 = block(graph, 1);
    assert_eq!(ids(b1.get_succs()), vec![5, 2, 3]);
    assert_eq!(
        b1.get_last_instruction().unwrap().as_switch().get_values(),
        vec![0, 1]
    );
}

#[test]
fn gotos_and_empty_blocks_are_removed() {
    let _context = crate::common::install_default_context();
    reset_class_scope();
    let graph = ControlFlowGraph::new(sequence(
        &[
            (OPC_INVOKESTATIC, GROUP_INVOCATION, &[1], 3),
            (OPC_GOTO, GROUP_JUMP, &[6], 3),
            (OPC_GOTO, GROUP_JUMP, &[-3], 3),
            (OPC_RETURN, GROUP_RETURN, &[], 1),
        ],
        &[],
    ));
    DeadCodeHelper::remove_dead_blocks(graph);
    assert_eq!(
        graph.get_blocks().borrow().len(),
        3,
        "the goto at 6 is unreachable"
    );
    DeadCodeHelper::connect_dummy_exit_block(graph);
    DeadCodeHelper::remove_gotos(graph);
    let blocks: Vec<BasicBlock> = graph.get_blocks().borrow().iter().copied().collect();
    assert_eq!(
        blocks.iter().map(|b| b.get_id()).collect::<Vec<_>>(),
        vec![1, 4]
    );
    assert_eq!(ids(blocks[0].get_succs()), vec![4]);
    assert_eq!(ids(blocks[1].get_preds()), vec![1]);
    assert_eq!(blocks[0].size(), 1);
}

#[test]
fn circular_goto_is_kept() {
    let _context = crate::common::install_default_context();
    reset_class_scope();
    let graph = ControlFlowGraph::new(sequence(&[(OPC_GOTO, GROUP_JUMP, &[0], 3)], &[]));
    DeadCodeHelper::connect_dummy_exit_block(graph);
    DeadCodeHelper::remove_gotos(graph);
    let b1 = block(graph, 1);
    assert_eq!(b1.size(), 1);
    assert_eq!(ids(b1.get_succs()), vec![1]);
    assert_eq!(ids(b1.get_preds()), vec![1]);
}

#[test]
fn merge_joins_a_straight_chain() {
    let _context = crate::common::install_default_context();
    reset_class_scope();
    let graph = ControlFlowGraph::new(sequence(
        &[
            (OPC_ILOAD, GROUP_GENERAL, &[0], 1),
            (OPC_IFEQ, GROUP_JUMP, &[3], 3),
            (OPC_NOP, GROUP_GENERAL, &[], 1),
            (OPC_RETURN, GROUP_RETURN, &[], 1),
        ],
        &[],
    ));
    DeadCodeHelper::connect_dummy_exit_block(graph);
    assert_eq!(graph.get_blocks().borrow().len(), 2);
    assert_eq!(ids(block(graph, 1).get_succs()), vec![2, 2]);
    DeadCodeHelper::merge_basic_blocks(graph);
    assert_eq!(
        graph.get_blocks().borrow().len(),
        2,
        "two edges into block 2, so nothing merges"
    );

    reset_class_scope();
    let graph = ControlFlowGraph::new(sequence(
        &[
            (OPC_NOP, GROUP_GENERAL, &[], 1),
            (OPC_GOTO, GROUP_JUMP, &[3], 3),
            (OPC_NOP, GROUP_GENERAL, &[], 1),
            (OPC_RETURN, GROUP_RETURN, &[], 1),
        ],
        &[],
    ));
    DeadCodeHelper::connect_dummy_exit_block(graph);
    DeadCodeHelper::remove_gotos(graph);
    DeadCodeHelper::merge_basic_blocks(graph);
    let blocks: Vec<BasicBlock> = graph.get_blocks().borrow().iter().copied().collect();
    assert_eq!(blocks.len(), 1);
    let b = blocks[0];
    assert_eq!(b.get_id(), 1);
    assert_eq!(b.size(), 3);
    assert_eq!(
        b.get_instr_old_offsets().to_vec(),
        vec![0, 1, 4, 5],
        "removed goto keeps its offset"
    );
    assert_eq!(ids(b.get_succs()), vec![graph.get_last().get_id()]);
}

#[test]
fn exception_ranges_and_handler_blocks() {
    let _context = crate::common::install_default_context();
    reset_class_scope();
    let graph = ControlFlowGraph::new(sequence(
        &[
            (OPC_INVOKESTATIC, GROUP_INVOCATION, &[1], 3),
            (OPC_RETURN, GROUP_RETURN, &[], 1),
            (OPC_POP, GROUP_GENERAL, &[], 1),
            (OPC_RETURN, GROUP_RETURN, &[], 1),
        ],
        &[
            (0, 1, 2, Some("java/lang/Exception")),
            (0, 1, 2, Some("java/lang/Error")),
        ],
    ));
    let exceptions = graph.get_exceptions();
    assert_eq!(
        exceptions.len(),
        1,
        "same from/to/handler merges into one range"
    );
    let range = exceptions.get(0);
    assert_eq!(range.get_handler().get_id(), 3);
    assert_eq!(ids(range.get_protected_range()), vec![1]);
    assert_eq!(
        range.get_exception_types(),
        Some(vec![
            "java/lang/Exception".to_string(),
            "java/lang/Error".to_string()
        ])
    );
    assert_eq!(ids(block(graph, 1).get_succ_exceptions()), vec![3]);
    assert_eq!(ids(block(graph, 3).get_pred_exceptions()), vec![1]);
    assert_eq!(
        graph.get_exception_range(block(graph, 3), block(graph, 1)),
        Some(range)
    );
    assert_eq!(
        graph.get_exception_range(block(graph, 3), block(graph, 2)),
        None
    );

    let text = graph.to_string();
    assert!(
        text.contains(">>>>>>>>(exception) Block 3\tjava/lang/Exception\n"),
        "{text}"
    );
    assert!(
        text.contains(">>>>>>>>(exception) Block 3\tjava/lang/Error\n"),
        "{text}"
    );
    assert!(text.starts_with("----- Block 1 -----\n1:\n"), "{text}");

    DeadCodeHelper::connect_dummy_exit_block(graph);
    ExceptionDeobfuscator::insert_empty_exception_handler_blocks(graph);
    let empty = range.get_handler();
    assert_eq!(empty.get_id(), 5);
    assert!(empty.get_seq().is_empty());
    assert_eq!(ids(empty.get_succs()), vec![3]);
    assert_eq!(ids(empty.get_pred_exceptions()), vec![1]);
    assert!(block(graph, 3).get_pred_exceptions().is_empty());

    graph.remove_block(block(graph, 1));
    assert!(
        graph.get_exceptions().is_empty(),
        "a range whose last block goes is dropped"
    );
}

#[test]
fn block_edge_operations() {
    let _context = crate::common::install_default_context();
    reset_class_scope();
    let (a, b, c) = (BasicBlock::new(1), BasicBlock::new(2), BasicBlock::new(3));
    a.add_successor(b);
    a.add_successor(b);
    a.add_successor(c);
    assert_eq!(ids(a.get_succs()), vec![2, 2, 3]);
    assert_eq!(ids(b.get_preds()), vec![1, 1]);
    assert!(a.is_successor(c));

    a.replace_successor(b, c);
    assert_eq!(ids(a.get_succs()), vec![3, 3, 3]);
    assert!(b.get_preds().is_empty());
    assert_eq!(ids(c.get_preds()), vec![1, 1, 1]);

    let twin = BasicBlock::new(3);
    a.replace_successor(twin, b);
    assert_eq!(
        ids(a.get_succs()),
        vec![2, 2, 2],
        "replaceSuccessor matches by id"
    );
    assert_eq!(
        ids(c.get_preds()),
        vec![1, 1, 1],
        "but unlinks the block it was given"
    );

    a.remove_successor(b);
    assert!(a.get_succs().is_empty());
    assert!(b.get_preds().is_empty());

    a.add_successor_exception(c);
    a.add_successor_exception(c);
    assert_eq!(ids(a.get_succ_exceptions()), vec![3]);
    assert_eq!(ids(c.get_pred_exceptions()), vec![1]);
    a.add_predecessor(c);
    assert_eq!(
        c.get_predecessors()
            .iter()
            .map(|x| x.get_id())
            .collect::<Vec<_>>(),
        vec![1, 1, 1, 1]
    );
    assert_eq!(
        a.get_predecessors()
            .iter()
            .map(|x| x.get_id())
            .collect::<Vec<_>>(),
        vec![3]
    );
    a.remove_successor_exception(c);
    assert!(a.get_succ_exceptions().is_empty() && c.get_pred_exceptions().is_empty());

    a.get_seq().add_instruction(Instruction::create(
        OPC_ATHROW,
        false,
        GROUP_RETURN,
        BytecodeVersion::new(65, 0),
        None,
        7,
        1,
    ));
    a.get_instr_old_offsets().add(7);
    let copy = a.clone_block(9);
    assert_eq!(copy.get_id(), 9);
    assert_ne!(copy.get_seq(), a.get_seq());
    assert_eq!(copy.get_instruction(0), a.get_instruction(0));
    assert_eq!(copy.get_instr_old_offsets().to_vec(), vec![7]);
    assert!(copy.get_succs().is_empty() && copy.get_preds().is_empty());
    assert_eq!(copy.to_string_indent(0).lines().next(), Some("9:"));
}

#[test]
fn exception_range_types() {
    let _context = crate::common::install_default_context();
    reset_class_scope();
    let (h, b) = (BasicBlock::new(5), BasicBlock::new(2));
    let range = ExceptionRangeCFG::new_single(JList::from(vec![b]), h, Some("a/X".to_string()));
    range.add_exception_type(Some("a/Y".to_string()));
    range.add_exception_type(Some("a/X".to_string()));
    assert_eq!(
        range.get_unique_exceptions_string().as_deref(),
        Some("a/X:a/Y")
    );
    assert!(!range.is_circular());
    assert_eq!(
        range.to_string(),
        "exceptionType: a/X a/Y a/X\nhandler: 5\nrange: 2 \n"
    );
    range.add_exception_type(None);
    assert_eq!(range.get_exception_types(), None);
    range.add_exception_type(Some("a/Z".to_string()));
    assert_eq!(
        range.get_unique_exceptions_string(),
        None,
        "a catch-all stays catch-all"
    );
    assert_eq!(
        range.to_string(),
        "exceptionType: null\nhandler: 5\nrange: 2 \n"
    );

    let shared = JList::from(vec![b]);
    let other = ExceptionRangeCFG::new(shared.clone(), b, Some(vec![]));
    shared.add(h);
    assert_eq!(
        ids(other.get_protected_range()),
        vec![2, 5],
        "the range list is aliased"
    );
    assert!(other.is_circular());
    assert_eq!(other.get_unique_exceptions_string().as_deref(), Some(""));
}

#[test]
fn empty_block_with_two_successors_is_an_error() {
    let _context = crate::common::install_default_context();
    reset_class_scope();
    let graph = ControlFlowGraph::new(branch());
    let b2 = block(graph, 2);
    b2.get_seq().clear();
    b2.add_successor(block(graph, 3));
    b2.add_successor(block(graph, 1));
    let caught = try_catch(|| DeadCodeHelper::remove_empty_blocks(graph)).unwrap_err();
    assert!(caught.is("RuntimeException"));
    assert!(caught
        .message
        .ends_with("ERROR: empty block with multiple successors found"));
}
