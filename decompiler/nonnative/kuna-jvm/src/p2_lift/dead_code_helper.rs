//! Port of `org/jetbrains/java/decompiler/modules/code/DeadCodeHelper.java`.

use std::collections::VecDeque;

use crate::basic_block::BasicBlock;
use crate::cancelation_manager::CancelationManager;
use crate::code_constants::{
    GROUP_SWITCH, OPC_ACONST_NULL, OPC_ALOAD, OPC_ATHROW, OPC_BIPUSH, OPC_DCONST_0, OPC_DCONST_1,
    OPC_DLOAD, OPC_FCONST_0, OPC_FCONST_1, OPC_FCONST_2, OPC_FLOAD, OPC_GOTO, OPC_ILOAD,
    OPC_IRETURN, OPC_LCONST_0, OPC_LCONST_1, OPC_LDC, OPC_LDC2_W, OPC_LDC_W, OPC_LLOAD,
    OPC_MONITORENTER, OPC_MONITOREXIT, OPC_RETURN, OPC_SIPUSH,
};
use crate::control_flow_graph::ControlFlowGraph;
use crate::decompiler_context::DecompilerContext;
use crate::i_fernflower_preferences::IFernflowerPreferences;
use crate::jthrow;
use crate::p8_structure::kuna_javac_layout;
use crate::rt::{JHashSet, JavaListExt};

pub struct DeadCodeHelper;

fn block_list(graph: ControlFlowGraph) -> Vec<BasicBlock> {
    graph.get_blocks().borrow().iter().copied().collect()
}

fn last_opcode_is(block: BasicBlock, opcode: i32) -> bool {
    let seq = block.get_seq();
    !seq.is_empty() && seq.get_last_instr().with(|d| d.opcode) == opcode
}

impl DeadCodeHelper {
    pub fn remove_dead_blocks(graph: ControlFlowGraph) {
        let mut stack: VecDeque<BasicBlock> = VecDeque::new();
        let mut set_stacked: JHashSet<BasicBlock> = JHashSet::new();

        stack.push_back(graph.get_first());
        set_stacked.add(graph.get_first());

        while let Some(block) = stack.pop_front() {
            let mut lst_succs = block.get_succs().to_vec();
            lst_succs.extend(block.get_succ_exceptions().to_vec());

            for succ in lst_succs {
                if !set_stacked.contains(&succ) {
                    stack.push_back(succ);
                    set_stacked.add(succ);
                }
            }
        }

        let mut set_all_blocks = JHashSet::from_collection(block_list(graph));
        set_all_blocks.remove_all(&set_stacked);

        for block in set_all_blocks.to_vec() {
            graph.remove_block(block);
        }
    }

    pub fn remove_empty_blocks(graph: ControlFlowGraph) {
        let blocks = graph.get_blocks();

        loop {
            let mut cont = false;

            let mut i = blocks.borrow().len() as i32 - 1;
            while i >= 0 {
                let block = blocks.borrow().get(i as usize);

                if Self::remove_empty_block(graph, block, false) {
                    cont = true;
                    break;
                }
                i -= 1;
            }

            if !cont {
                break;
            }
        }
    }

    fn remove_empty_block(graph: ControlFlowGraph, block: BasicBlock, merging: bool) -> bool {
        let mut deleted_ranges = false;

        if block.get_seq().is_empty() {
            if block.get_succs().len() > 1 {
                if block.get_preds().len() > 1 {
                    jthrow!("ERROR: empty block with multiple predecessors and successors found");
                } else if !merging {
                    jthrow!("ERROR: empty block with multiple successors found");
                }
            }

            let set_exits = JHashSet::from_collection(graph.get_last().get_preds().to_vec());

            if block.get_pred_exceptions().is_empty()
                && (!set_exits.contains(&block) || block.get_preds().len() == 1)
            {
                if set_exits.contains(&block) {
                    let pred = block.get_preds().get(0);

                    let pred_seq = pred.get_seq();
                    if pred.get_succs().len() != 1
                        || (!pred_seq.is_empty()
                            && pred_seq.get_last_instr().with(|d| d.group) == GROUP_SWITCH)
                    {
                        return false;
                    }
                }

                let set_preds = JHashSet::from_collection(block.get_preds().to_vec());
                let set_succs = JHashSet::from_collection(block.get_succs().to_vec());

                let mut set_common_exception_handlers: Option<JHashSet<BasicBlock>> = None;
                for i in 0..2 {
                    let side = if i == 0 { &set_preds } else { &set_succs };
                    for pred in side.iter() {
                        let succ_exceptions = pred.get_succ_exceptions().to_vec();
                        match set_common_exception_handlers.as_mut() {
                            None => {
                                set_common_exception_handlers =
                                    Some(JHashSet::from_collection(succ_exceptions));
                            }
                            Some(common) => {
                                common.retain(|h| succ_exceptions.j_contains(h));
                            }
                        }
                    }
                }

                if let Some(common) = &set_common_exception_handlers {
                    if !common.is_empty() {
                        let block_succ_exceptions = block.get_succ_exceptions();
                        for handler in common.iter() {
                            if !block_succ_exceptions.contains(handler) {
                                return false;
                            }
                        }
                    }
                }

                let lst_ranges = graph.get_exceptions();
                for i in (0..lst_ranges.len()).rev() {
                    let range = lst_ranges.get(i);
                    let lst = range.get_protected_range();

                    if lst.len() == 1 && lst.get(0) == block {
                        if DecompilerContext::get_option(
                            IFernflowerPreferences::REMOVE_EMPTY_RANGES,
                        ) {
                            block.remove_successor_exception(range.get_handler());
                            lst_ranges.remove_at(i);

                            deleted_ranges = true;
                        } else {
                            return false;
                        }
                    }
                }

                if merging {
                    let pred = block.get_preds().get(0);
                    pred.remove_successor(block);

                    let lst_succs = block.get_succs().to_vec();
                    for succ in lst_succs {
                        block.remove_successor(succ);
                        pred.add_successor(succ);
                    }
                } else {
                    for pred in set_preds.to_vec() {
                        for succ in set_succs.to_vec() {
                            pred.replace_successor(block, succ);
                        }
                    }
                }

                let set_finally_exits = graph.get_finally_exits();
                if set_finally_exits.borrow().contains(&block) {
                    set_finally_exits.borrow_mut().remove(&block);
                    let replacement = *set_preds.iter().next().expect("NoSuchElementException");
                    set_finally_exits.borrow_mut().add(replacement);
                }

                if graph.get_first() == block {
                    if set_succs.len() != 1 {
                        jthrow!("multiple or no entry blocks!");
                    } else {
                        graph.set_first(*set_succs.iter().next().expect("NoSuchElementException"));
                    }
                }

                graph.remove_block(block);

                if deleted_ranges {
                    Self::remove_dead_blocks(graph);
                }
            }
        }

        deleted_ranges
    }

    pub fn is_dominator(graph: ControlFlowGraph, block: BasicBlock, dom: BasicBlock) -> bool {
        let mut marked: JHashSet<BasicBlock> = JHashSet::new();

        if block == dom {
            return true;
        }

        let first = graph.get_first();

        let mut lst_nodes: VecDeque<BasicBlock> = VecDeque::new();
        lst_nodes.push_back(block);

        while let Some(node) = lst_nodes.pop_front() {
            if marked.contains(&node) {
                continue;
            } else {
                marked.add(node);
            }

            if node == first {
                return false;
            }

            let preds = node.get_preds();
            for i in 0..preds.len() {
                let pred = preds.get(i);
                if pred != dom && !marked.contains(&pred) {
                    lst_nodes.push_back(pred);
                }
            }

            let pred_exceptions = node.get_pred_exceptions();
            for i in 0..pred_exceptions.len() {
                let pred = pred_exceptions.get(i);
                if pred != dom && !marked.contains(&pred) {
                    lst_nodes.push_back(pred);
                }
            }
        }

        true
    }

    pub fn remove_gotos(graph: ControlFlowGraph) {
        let ignored = Self::find_circular_jumps(graph);

        for block in block_list(graph) {
            let instr = block.get_last_instruction();

            if let Some(instr) = instr {
                if instr.with(|d| d.opcode) == OPC_GOTO {
                    if ignored.contains(&block) {
                        continue;
                    }

                    block.get_seq().remove_last();
                }
            }
        }

        Self::remove_empty_blocks(graph);
    }

    fn find_circular_jumps(graph: ControlFlowGraph) -> JHashSet<BasicBlock> {
        let mut ret: JHashSet<BasicBlock> = JHashSet::new();

        let mut blocks: JHashSet<BasicBlock> = JHashSet::new();

        for block in block_list(graph) {
            if ret.contains(&block) {
                continue;
            }

            blocks.add(block);

            let mut check = block;
            loop {
                let instr = check.get_last_instruction();

                match instr {
                    Some(instr) if instr.with(|d| d.opcode) == OPC_GOTO => {
                        if check.get_succs().len() == 1 {
                            check = check.get_succs().get(0);

                            if blocks.contains(&check) {
                                ret.add_all(blocks.to_vec());
                                break;
                            }

                            blocks.add(check);
                        } else {
                            break;
                        }
                    }
                    _ => break,
                }
            }

            blocks.clear();
        }

        ret
    }

    pub fn connect_dummy_exit_block(graph: ControlFlowGraph) {
        let exit = graph.get_last();
        for block in JHashSet::from_collection(exit.get_preds().to_vec()).to_vec() {
            exit.remove_predecessor(block);
            block.add_successor(exit);
        }
    }

    pub fn extend_synchronized_range_to_monitorexit(graph: ControlFlowGraph) {
        loop {
            let mut range_extended = false;

            for range in graph.get_exceptions().iter() {
                let protected_range = range.get_protected_range();

                let mut set_preds: JHashSet<BasicBlock> = JHashSet::new();
                for block in protected_range.iter() {
                    set_preds.add_all(block.get_preds().to_vec());
                }
                for basic_block in protected_range.iter() {
                    set_preds.remove(&basic_block);
                }

                if set_preds.len() != 1 {
                    continue;
                }

                let pred_block = *set_preds.iter().next().expect("NoSuchElementException");
                if !last_opcode_is(pred_block, OPC_MONITORENTER) {
                    continue;
                }

                let mut monitorexit_in_range = false;
                let mut set_protected_blocks = JHashSet::from_collection(protected_range.to_vec());
                set_protected_blocks.add(range.get_handler());

                for block in set_protected_blocks.to_vec() {
                    let block_seq = block.get_seq();
                    for i in 0..block_seq.length() {
                        if block_seq.get_instr(i).with(|d| d.opcode) == OPC_MONITOREXIT {
                            monitorexit_in_range = true;
                            break;
                        }
                    }

                    if monitorexit_in_range {
                        break;
                    }
                }

                if monitorexit_in_range {
                    continue;
                }

                let mut set_succs: JHashSet<BasicBlock> = JHashSet::new();
                for block in protected_range.iter() {
                    set_succs.add_all(block.get_succs().to_vec());
                }
                for basic_block in protected_range.iter() {
                    set_succs.remove(&basic_block);
                }

                if set_succs.len() != 1 {
                    continue;
                }

                let succ_block = *set_succs.iter().next().expect("NoSuchElementException");
                let succ_seq = succ_block.get_seq();

                let mut succ_monitorexit_index = -1;
                for i in 0..succ_seq.length() {
                    if succ_seq.get_instr(i).with(|d| d.opcode) == OPC_MONITOREXIT {
                        succ_monitorexit_index = i;
                        break;
                    }
                }

                if succ_monitorexit_index < 0 {
                    continue;
                }

                let handler_block = range.get_handler();
                if handler_block.get_succs().len() != 1 {
                    continue;
                }
                let succ_handler = handler_block.get_succs().get(0);
                let succ_handler_seq = succ_handler.get_seq();
                if !last_opcode_is(succ_handler, OPC_ATHROW) {
                    continue;
                }

                let mut handler_monitorexit_index = -1;
                for i in 0..succ_handler_seq.length() {
                    if succ_handler_seq.get_instr(i).with(|d| d.opcode) == OPC_MONITOREXIT {
                        handler_monitorexit_index = i;
                        break;
                    }
                }

                if handler_monitorexit_index < 0 {
                    continue;
                }

                let range_exit_block = succ_block.get_preds().get(0);
                let range_exit_exceptions = range_exit_block.get_succ_exceptions();
                let mut j = 0;
                while j < range_exit_exceptions.len() {
                    let hd = range_exit_exceptions.get(j);
                    succ_block.add_successor_exception(hd);

                    let rng = graph
                        .get_exception_range(hd, range_exit_block)
                        .expect("NullPointerException");
                    rng.get_protected_range().add(succ_block);
                    j += 1;
                }

                let handler_seq = handler_block.get_seq();
                for _counter in 0..handler_monitorexit_index {
                    handler_seq.add_instruction(succ_handler_seq.get_instr(0));
                    succ_handler_seq.remove_instruction(0);
                }

                range_extended = true;
                graph.add_comment("$VF: Extended synchronized range to monitorexit");
                break;
            }

            if !range_extended {
                break;
            }
        }

        for range in graph.get_exceptions().iter() {
            let protected_range = range.get_protected_range();
            let mut predecessors: JHashSet<BasicBlock> = JHashSet::new();

            let mut exceptions_ok = true;
            for block in protected_range.iter() {
                if block.get_succ_exceptions().len() > 1 {
                    exceptions_ok = false;
                    break;
                }
            }

            if !exceptions_ok {
                continue;
            }

            for block in protected_range.iter() {
                predecessors.add_all(block.get_preds().to_vec());
            }

            for block in protected_range.iter() {
                predecessors.remove(&block);
            }

            if predecessors.len() != 1 {
                continue;
            }

            let pred = *predecessors.iter().next().expect("NoSuchElementException");
            if !last_opcode_is(pred, OPC_MONITORENTER) {
                continue;
            }

            let mut successors: JHashSet<BasicBlock> = JHashSet::new();
            for block in protected_range.iter() {
                successors.add_all(block.get_succs().to_vec());
            }

            for block in protected_range.iter() {
                successors.remove(&block);
            }

            for successor in successors.to_vec() {
                if last_opcode_is(successor, OPC_MONITOREXIT)
                    && !protected_range.contains(&successor)
                {
                    protected_range.add(successor);

                    successor.add_successor_exception(range.get_handler());
                }
            }
        }
    }

    pub fn incorporate_value_returns(graph: ControlFlowGraph) {
        for block in block_list(graph) {
            let seq = block.get_seq();

            let len = seq.length();
            if len > 0 && len < 3 {
                let mut ok = false;

                let last_opcode = seq.get_last_instr().with(|d| d.opcode);
                if (OPC_IRETURN..=OPC_RETURN).contains(&last_opcode) {
                    if len == 1 {
                        ok = true;
                    } else if last_opcode != OPC_RETURN {
                        match seq.get_instr(0).with(|d| d.opcode) {
                            OPC_ILOAD | OPC_LLOAD | OPC_FLOAD | OPC_DLOAD | OPC_ALOAD
                            | OPC_ACONST_NULL | OPC_BIPUSH | OPC_SIPUSH | OPC_LCONST_0
                            | OPC_LCONST_1 | OPC_FCONST_0 | OPC_FCONST_1 | OPC_FCONST_2
                            | OPC_DCONST_0 | OPC_DCONST_1 | OPC_LDC | OPC_LDC_W | OPC_LDC2_W => {
                                ok = true;
                            }
                            _ => {}
                        }
                    }
                }

                if ok
                    && last_opcode != OPC_RETURN
                    && kuna_javac_layout::active()
                    && block
                        .get_preds()
                        .iter()
                        .all(|pred| kuna_javac_layout::jumped_to(pred, block))
                {
                    ok = false;
                }

                if ok {
                    if !block.get_preds().is_empty() {
                        let mut set_pred_handlers_union: JHashSet<BasicBlock> = JHashSet::new();
                        let mut set_pred_handlers_intersection: JHashSet<BasicBlock> =
                            JHashSet::new();

                        let mut firstpred = true;
                        for pred in block.get_preds().iter() {
                            let pred_succ_exceptions = pred.get_succ_exceptions().to_vec();
                            if firstpred {
                                set_pred_handlers_intersection
                                    .add_all(pred_succ_exceptions.clone());
                                firstpred = false;
                            } else {
                                set_pred_handlers_intersection
                                    .retain(|h| pred_succ_exceptions.j_contains(h));
                            }

                            set_pred_handlers_union.add_all(pred_succ_exceptions);
                        }

                        for basic_block in block.get_succ_exceptions().iter() {
                            set_pred_handlers_intersection.remove(&basic_block);
                        }
                        let predecessor = block.get_preds().get(0);

                        for handler in set_pred_handlers_intersection.to_vec() {
                            let range = graph
                                .get_exception_range(handler, predecessor)
                                .expect("NullPointerException");

                            range.get_protected_range().add(block);
                            block.add_successor_exception(handler);
                        }

                        let mut set_ranges_to_be_removed =
                            JHashSet::from_collection(block.get_succ_exceptions().to_vec());
                        set_ranges_to_be_removed.remove_all(&set_pred_handlers_union);

                        for handler in set_ranges_to_be_removed.to_vec() {
                            let range = graph
                                .get_exception_range(handler, block)
                                .expect("NullPointerException");

                            if range.get_protected_range().len() > 1 {
                                range.get_protected_range().remove_item(&block);
                                block.remove_successor_exception(handler);
                            }
                        }
                    }

                    if block.get_preds().len() == 1 && block.get_pred_exceptions().is_empty() {
                        let bpred = block.get_preds().get(0);
                        if bpred.get_succs().len() == 1 {
                            for succ in bpred.get_succ_exceptions().iter() {
                                if !block.get_succ_exceptions().contains(&succ) {
                                    let range = graph
                                        .get_exception_range(succ, bpred)
                                        .expect("NullPointerException");

                                    range.get_protected_range().add(block);
                                    block.add_successor_exception(succ);
                                }
                            }

                            let succs_copy =
                                JHashSet::from_collection(block.get_succ_exceptions().to_vec());
                            for succ in succs_copy.to_vec() {
                                if !bpred.get_succ_exceptions().contains(&succ) {
                                    let range = graph
                                        .get_exception_range(succ, block)
                                        .expect("NullPointerException");

                                    if range.get_protected_range().len() > 1 {
                                        range.get_protected_range().remove_item(&block);
                                        block.remove_successor_exception(succ);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn merge_basic_blocks(graph: ControlFlowGraph) {
        loop {
            let mut merged = false;
            let origin_blocks_count = graph.get_blocks().borrow().len();

            for block in block_list(graph) {
                CancelationManager::check_canceled();
                let seq = block.get_seq();

                if block.get_succs().len() == 1 {
                    let next = block.get_succs().get(0);

                    if next != graph.get_last()
                        && (seq.is_empty()
                            || seq.get_last_instr().with(|d| d.group) != GROUP_SWITCH)
                        && next.get_preds().len() == 1
                        && next.get_pred_exceptions().is_empty()
                        && next != graph.get_first()
                    {
                        let mut same_ranges = true;
                        for range in graph.get_exceptions().iter() {
                            let protected_range = range.get_protected_range();
                            if protected_range.contains(&block) ^ protected_range.contains(&next) {
                                same_ranges = false;
                                break;
                            }
                        }

                        if same_ranges {
                            seq.add_sequence(next.get_seq());
                            block
                                .get_instr_old_offsets()
                                .add_all(next.get_instr_old_offsets().to_vec());
                            next.get_seq().clear();

                            Self::remove_empty_block(graph, next, true);

                            merged = true;
                            break;
                        }
                    }
                }
            }

            if !merged || graph.get_blocks().borrow().len() == origin_blocks_count {
                break;
            }
        }
    }
}
