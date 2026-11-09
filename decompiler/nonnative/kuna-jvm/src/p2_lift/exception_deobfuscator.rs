//! Port of `org/jetbrains/java/decompiler/modules/code/ExceptionDeobfuscator.java`.
//!
//! kuna fix `nested_handler_copies` (spec 10.16): the handler copies of
//! `insert_dummy_exception_handler_blocks` are nested in the outer-handler copy
//! of the range they serve.

use std::collections::VecDeque;

use crate::basic_block::BasicBlock;
use crate::bytecode_version::BytecodeVersion;
use crate::cancelation_manager::CancelationManager;
use crate::code_constants::{GROUP_GENERAL, OPC_ASTORE, OPC_BIPUSH, OPC_NEW, OPC_POP};
use crate::control_flow_graph::ControlFlowGraph;
use crate::decompiler_context::DecompilerContext;
use crate::exception_range_cfg::ExceptionRangeCFG;
use crate::generic_dominator_engine::GenericDominatorEngine;
use crate::i_fernflower_logger::Severity;
use crate::i_fernflower_preferences::IFernflowerPreferences;
use crate::i_graph::IGraph;
use crate::instruction::Instruction;
use crate::instruction_sequence::InstructionSequence;
use crate::kuna_fixes::{self, KunaFix};
use crate::p2_lift::kuna_fix_nested_handler_copies::NestedHandlerCopies;
use crate::rt::{objects_equals, JHashMap, JHashSet, JList, JavaListExt};

pub struct ExceptionDeobfuscator;

struct Range {
    handler: BasicBlock,
    unique_str: Option<String>,
    protected_range: JHashSet<BasicBlock>,
    range_cfg: ExceptionRangeCFG,
}

struct CfgGraph(ControlFlowGraph);

impl IGraph<BasicBlock> for CfgGraph {
    fn get_reverse_post_order_list(&self) -> Vec<BasicBlock> {
        self.0.get_reverse_post_order()
    }

    fn get_roots(&self) -> JHashSet<BasicBlock> {
        JHashSet::from_collection(vec![self.0.get_first()])
    }
}

impl ExceptionDeobfuscator {
    pub fn restore_pop_ranges(graph: ControlFlowGraph) {
        let mut lst_ranges: Vec<Range> = Vec::new();

        for range in graph.get_exceptions().iter() {
            let mut found = false;
            for arr in lst_ranges.iter_mut() {
                if arr.handler == range.get_handler()
                    && objects_equals(&range.get_unique_exceptions_string(), &arr.unique_str)
                {
                    arr.protected_range
                        .add_all(range.get_protected_range().to_vec());
                    found = true;
                    break;
                }
            }

            if !found {
                lst_ranges.push(Range {
                    handler: range.get_handler(),
                    unique_str: range.get_unique_exceptions_string(),
                    protected_range: JHashSet::from_collection(
                        range.get_protected_range().to_vec(),
                    ),
                    range_cfg: range,
                });
            }
        }

        for ri in 0..lst_ranges.len() {
            if lst_ranges[ri].unique_str.is_some() {
                let mut handler = lst_ranges[ri].handler;
                let mut seq = handler.get_seq();

                if seq.length() > 0 {
                    let firstinstr = seq.get_instr(0);

                    let first_opcode = firstinstr.with(|d| d.opcode);
                    if first_opcode == OPC_POP || first_opcode == OPC_ASTORE {
                        let setrange = lst_ranges[ri].protected_range.clone();

                        for si in 0..lst_ranges.len() {
                            if ri != si {
                                let range_super = &lst_ranges[si];
                                let mut setrange_super = range_super.protected_range.clone();

                                if !setrange.contains(&range_super.handler)
                                    && !setrange_super.contains(&handler)
                                    && (range_super.unique_str.is_none()
                                        || setrange_super.contains_all(&setrange))
                                {
                                    if range_super.unique_str.is_none() {
                                        setrange_super.retain_all(&setrange);
                                    } else {
                                        setrange_super.remove_all(&setrange);
                                    }

                                    if !setrange_super.is_empty() {
                                        let mut newblock = handler;

                                        if seq.length() > 1 {
                                            let id = graph.with_mut(|d| {
                                                d.last_id += 1;
                                                d.last_id
                                            });
                                            newblock = BasicBlock::new(id);
                                            let newseq = InstructionSequence::new();
                                            newseq.add_instruction(firstinstr.java_clone());

                                            newblock.set_seq(newseq);
                                            graph
                                                .get_blocks()
                                                .borrow_mut()
                                                .add_with_key(newblock, newblock.get_id());

                                            let mut lst_temp = handler.get_preds().to_vec();
                                            lst_temp.extend(handler.get_pred_exceptions().to_vec());

                                            for pred in lst_temp {
                                                pred.replace_successor(handler, newblock);
                                            }

                                            for range_ext in graph.get_exceptions().iter() {
                                                if range_ext.get_handler() == handler {
                                                    range_ext.set_handler(newblock);
                                                } else if range_ext
                                                    .get_protected_range()
                                                    .contains(&handler)
                                                {
                                                    newblock.add_successor_exception(
                                                        range_ext.get_handler(),
                                                    );
                                                    range_ext.get_protected_range().add(newblock);
                                                }
                                            }

                                            newblock.add_successor(handler);
                                            if graph.get_first() == handler {
                                                graph.set_first(newblock);
                                            }

                                            seq.remove_instruction(0);
                                        }

                                        newblock.add_successor_exception(range_super.handler);
                                        range_super.range_cfg.get_protected_range().add(newblock);

                                        handler = lst_ranges[ri].range_cfg.get_handler();
                                        seq = handler.get_seq();
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn insert_empty_exception_handler_blocks(graph: ControlFlowGraph) {
        let mut set_visited: JHashSet<BasicBlock> = JHashSet::new();

        for range in graph.get_exceptions().iter() {
            CancelationManager::check_canceled();
            let handler = range.get_handler();

            if set_visited.contains(&handler) {
                continue;
            }
            set_visited.add(handler);

            let id = graph.with_mut(|d| {
                d.last_id += 1;
                d.last_id
            });
            let emptyblock = BasicBlock::new(id);
            graph
                .get_blocks()
                .borrow_mut()
                .add_with_key(emptyblock, emptyblock.get_id());

            let lst_temp = handler.get_pred_exceptions().to_vec();

            for pred in lst_temp {
                pred.replace_successor(handler, emptyblock);
            }

            for range_ext in graph.get_exceptions().iter() {
                if range_ext.get_handler() == handler {
                    range_ext.set_handler(emptyblock);
                } else if range_ext.get_protected_range().contains(&handler) {
                    emptyblock.add_successor_exception(range_ext.get_handler());
                    range_ext.get_protected_range().add(emptyblock);
                }
            }

            emptyblock.add_successor(handler);
            if graph.get_first() == handler {
                graph.set_first(emptyblock);
            }
        }
    }

    pub fn remove_empty_ranges(graph: ControlFlowGraph) {
        let lst_ranges = graph.get_exceptions();
        for i in (0..lst_ranges.len()).rev() {
            let range = lst_ranges.get(i);

            let mut is_empty = true;
            for block in range.get_protected_range().iter() {
                if !block.get_seq().is_empty() {
                    is_empty = false;
                    break;
                }
            }

            if is_empty {
                for block in range.get_protected_range().iter() {
                    block.remove_successor_exception(range.get_handler());
                }

                lst_ranges.remove_at(i);
                graph.add_comment("$VF: Removed empty exception range");
            }
        }
    }

    pub fn remove_circular_ranges(graph: ControlFlowGraph) {
        let mut engine = GenericDominatorEngine::new(Box::new(CfgGraph(graph)));

        engine.initialize();

        let lst_ranges = graph.get_exceptions();
        for i in (0..lst_ranges.len()).rev() {
            let range = lst_ranges.get(i);

            let handler = range.get_handler();
            let range_list = range.get_protected_range();

            if range_list.contains(&handler) {
                let lst_rem_blocks =
                    Self::get_reachable_blocks_restricted(range.get_handler(), range, &engine);

                if lst_rem_blocks.len() < range_list.len() || range_list.len() == 1 {
                    for block in lst_rem_blocks {
                        block.remove_successor_exception(handler);
                        range_list.remove_item(&block);
                    }
                }

                if range_list.is_empty() {
                    lst_ranges.remove_at(i);
                }
            }
        }
    }

    fn get_reachable_blocks_restricted(
        start: BasicBlock,
        range: ExceptionRangeCFG,
        engine: &GenericDominatorEngine<BasicBlock>,
    ) -> Vec<BasicBlock> {
        let mut lst_res: Vec<BasicBlock> = Vec::new();

        let mut stack: VecDeque<BasicBlock> = VecDeque::new();
        let mut set_visited: JHashSet<BasicBlock> = JHashSet::new();

        stack.push_front(start);

        while let Some(block) = stack.pop_front() {
            set_visited.add(block);

            if range.get_protected_range().contains(&block) && engine.is_dominator(block, start) {
                lst_res.push(block);

                let mut lst_succs = block.get_succs().to_vec();
                lst_succs.extend(block.get_succ_exceptions().to_vec());

                for succ in lst_succs {
                    if !set_visited.contains(&succ) {
                        stack.push_back(succ);
                    }
                }
            }
        }

        lst_res
    }

    pub fn has_obfuscated_exceptions(graph: ControlFlowGraph) -> bool {
        let mut map_ranges: JHashMap<BasicBlock, JHashSet<BasicBlock>> = JHashMap::new();
        for range in graph.get_exceptions().iter() {
            map_ranges
                .compute_if_absent(range.get_handler(), JHashSet::new)
                .add_all(range.get_protected_range().to_vec());
        }

        let first = graph.get_first();

        for (_, value) in map_ranges.iter() {
            let mut set_entries: JHashSet<BasicBlock> = JHashSet::new();

            for block in value.iter() {
                let mut set_temp = JHashSet::from_collection(block.get_preds().to_vec());
                set_temp.remove_all(value);

                if !set_temp.is_empty() {
                    set_entries.add(*block);
                }
            }

            if value.contains(&first) {
                set_entries.add(first);
            }

            if !set_entries.is_empty() && set_entries.len() > 1 {
                return true;
            }
        }

        false
    }

    pub fn handle_multiple_entry_exception_ranges(graph: ControlFlowGraph) -> bool {
        let mut engine = GenericDominatorEngine::new(Box::new(CfgGraph(graph)));

        engine.initialize();

        let mut found;

        loop {
            found = false;
            let mut splitted = false;

            for range in graph.get_exceptions().iter() {
                let set_entries = Self::get_range_entries(range);

                if set_entries.len() > 1 {
                    found = true;

                    if Self::split_exception_range(range, &set_entries, graph, &engine) {
                        splitted = true;
                        graph.add_comment(
                            "$VF: Handled exception range with multiple entry points by splitting it",
                        );
                        break;
                    }
                }
            }

            if !splitted {
                break;
            }
        }

        !found
    }

    fn get_range_entries(range: ExceptionRangeCFG) -> JHashSet<BasicBlock> {
        let mut set_entries: JHashSet<BasicBlock> = JHashSet::new();
        let set_range = JHashSet::from_collection(range.get_protected_range().to_vec());

        for block in range.get_protected_range().iter() {
            let mut set_preds = JHashSet::from_collection(block.get_preds().to_vec());
            set_preds.remove_all(&set_range);

            if !set_preds.is_empty() {
                set_entries.add(block);
            }
        }

        set_entries
    }

    fn split_exception_range(
        range: ExceptionRangeCFG,
        set_entries: &JHashSet<BasicBlock>,
        graph: ControlFlowGraph,
        engine: &GenericDominatorEngine<BasicBlock>,
    ) -> bool {
        for entry in set_entries.to_vec() {
            let lst_subrange_blocks = Self::get_reachable_blocks_restricted(entry, range, engine);
            if !lst_subrange_blocks.is_empty()
                && lst_subrange_blocks.len() < range.get_protected_range().len()
            {
                let sub_range = ExceptionRangeCFG::new(
                    JList::from(lst_subrange_blocks.clone()),
                    range.get_handler(),
                    range.get_exception_types(),
                );
                graph.get_exceptions().add(sub_range);
                range
                    .get_protected_range()
                    .borrow_mut()
                    .retain(|b| !lst_subrange_blocks.j_contains(b));
                return true;
            } else {
                DecompilerContext::get_logger().write_message(
                    "Inconsistency found while splitting protected range",
                    Severity::Warn,
                );
            }
        }

        false
    }

    pub fn insert_dummy_exception_handler_blocks(
        graph: ControlFlowGraph,
        bytecode_version: BytecodeVersion,
    ) {
        let mut map_ranges: JHashMap<BasicBlock, Vec<ExceptionRangeCFG>> = JHashMap::new();
        for range in graph.get_exceptions().iter() {
            map_ranges
                .compute_if_absent(range.get_handler(), Vec::new)
                .push(range);
        }

        let nest_copies = kuna_fixes::enabled(KunaFix::NestedHandlerCopies);
        let separate_finally = kuna_fixes::enabled(KunaFix::HandlerCopyOffsets);
        let mut copies: JHashMap<BasicBlock, BasicBlock> = JHashMap::new();
        let mut finally_copies: JHashMap<BasicBlock, BasicBlock> = JHashMap::new();

        for (handler, ranges) in map_ranges.entry_vec() {
            if ranges.len() == 1 {
                continue;
            }

            if !DecompilerContext::get_option(IFernflowerPreferences::OLD_TRY_DEDUP) {
                for range in ranges.iter().skip(1).copied() {
                    let id = graph.with_mut(|d| {
                        d.last_id += 1;
                        d.last_id
                    });
                    let new_block = BasicBlock::new(id);
                    new_block.set_seq(handler.get_seq().java_clone());
                    if nest_copies {
                        copies.put(new_block, handler);
                    }
                    if separate_finally && range.get_exception_types().is_none() {
                        finally_copies.put(new_block, handler);
                    }

                    graph
                        .get_blocks()
                        .borrow_mut()
                        .add_with_key(new_block, new_block.get_id());

                    let protected_range = range.get_protected_range();
                    let mut lst_pred_exceptions = handler.get_pred_exceptions().to_vec();
                    lst_pred_exceptions.retain(|b| protected_range.contains(b));

                    for pred in lst_pred_exceptions {
                        pred.replace_successor(handler, new_block);
                    }
                    range.set_handler(new_block);

                    for succ in handler.get_succs().iter() {
                        new_block.add_successor(succ);
                    }

                    for succ in handler.get_succ_exceptions().iter() {
                        new_block.add_successor_exception(succ);
                        let exc_range = graph
                            .get_exception_range(succ, handler)
                            .expect("NullPointerException");
                        exc_range.get_protected_range().add(new_block);
                    }
                }

                if !Self::is_match_exception(handler) {
                    graph.add_comment(
                        "$VF: Duplicated exception handlers to handle obfuscated exceptions",
                    );
                }
            } else {
                for range in ranges.iter().copied() {
                    let seq = InstructionSequence::new();
                    seq.add_instruction(Instruction::create(
                        OPC_BIPUSH,
                        false,
                        GROUP_GENERAL,
                        bytecode_version,
                        Some(vec![0]),
                        -1,
                        1,
                    ));
                    seq.add_instruction(Instruction::create(
                        OPC_POP,
                        false,
                        GROUP_GENERAL,
                        bytecode_version,
                        None,
                        -1,
                        1,
                    ));

                    let id = graph.with_mut(|d| {
                        d.last_id += 1;
                        d.last_id
                    });
                    let dummy_block = BasicBlock::new(id);
                    dummy_block.set_seq(seq);

                    graph
                        .get_blocks()
                        .borrow_mut()
                        .add_with_key(dummy_block, dummy_block.get_id());

                    let protected_range = range.get_protected_range();
                    let mut lst_pred_exceptions = handler.get_pred_exceptions().to_vec();
                    lst_pred_exceptions.retain(|b| protected_range.contains(b));

                    for pred in lst_pred_exceptions.iter().copied() {
                        pred.replace_successor(handler, dummy_block);
                    }

                    range.set_handler(dummy_block);
                    let mut common_handlers =
                        JHashSet::from_collection(handler.get_succ_exceptions().to_vec());
                    for pred in lst_pred_exceptions.iter() {
                        let pred_succ_exceptions = pred.get_succ_exceptions();
                        common_handlers.retain(|h| pred_succ_exceptions.contains(h));
                    }
                    for common_handler in common_handlers.to_vec() {
                        let common_range = graph
                            .get_exception_range(common_handler, handler)
                            .expect("NullPointerException");

                        dummy_block.add_successor_exception(common_handler);
                        common_range.get_protected_range().add(dummy_block);
                    }

                    dummy_block.add_successor(handler);

                    graph.add_comment(
                        "$VF: Inserted dummy exception handlers to handle obfuscated exceptions",
                    );
                }
            }
        }

        if nest_copies && !copies.is_empty() && NestedHandlerCopies::nest(graph, &copies) {
            NestedHandlerCopies::separate_handler_bodies(graph, &copies);
        } else if !finally_copies.is_empty() {
            NestedHandlerCopies::separate_handler_bodies(graph, &finally_copies);
        }
    }

    fn is_match_exception(block: BasicBlock) -> bool {
        let cl = DecompilerContext::get_current_class();

        let seq = block.get_seq();
        for i in 0..seq.length() {
            let instr = seq.get_instr(i);
            if instr.with(|d| d.opcode) == OPC_NEW {
                let class_name = cl
                    .expect("NullPointerException")
                    .get_pool()
                    .get_primitive_constant(instr.operand(0))
                    .expect("NullPointerException")
                    .get_string_opt();
                if class_name.as_deref() == Some("java/lang/MatchException") {
                    return true;
                }
            }
        }

        false
    }
}
