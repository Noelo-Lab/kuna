//! Port of `org/jetbrains/java/decompiler/modules/decompiler/FinallyProcessor.java`.
//!
//! kuna fix `handler_copy_offsets` (`kuna_fixes`): on a handler copy made by
//! `ExceptionDeobfuscator::insert_dummy_exception_handler_blocks` that holds the
//! whole handler, `remove_exception_instructions_ex` removes only the instruction
//! offsets the copy has (none).
//!
//! kuna fix `copied_finally` (spec 10.16): `iterate_graph` marks the copies
//! irreducible-flow splitting made of a processed `finally` as `finally`.

use std::collections::VecDeque;

use crate::basic_block::BasicBlock;
use crate::basic_block_statement::BasicBlockStatement;
use crate::bytecode_version::BytecodeVersion;
use crate::catch_all_statement::CatchAllStatement;
use crate::code_constants::{
    GROUP_GENERAL, GROUP_JUMP, GROUP_SWITCH, OPC_ALOAD, OPC_ARETURN, OPC_ASTORE, OPC_BIPUSH,
    OPC_DLOAD, OPC_DRETURN, OPC_DSTORE, OPC_FLOAD, OPC_FRETURN, OPC_FSTORE, OPC_IINC, OPC_ILOAD,
    OPC_IRETURN, OPC_ISTORE, OPC_LLOAD, OPC_LRETURN, OPC_LSTORE, OPC_POP, OPC_SASTORE,
};
use crate::control_flow_graph::ControlFlowGraph;
use crate::counter_container::CounterContainer;
use crate::dead_code_helper::DeadCodeHelper;
use crate::decompiler_context::DecompilerContext;
use crate::direct_edge_type::DirectEdgeType;
use crate::direct_node::DirectNode;
use crate::exception_range_cfg::ExceptionRangeCFG;
use crate::exit_exprent::Type as ExitType;
use crate::expr_processor::ExprProcessor;
use crate::exprent::Exprent;
use crate::flatten_statements_helper::FlattenStatementsHelper;
use crate::i_fernflower_preferences::IFernflowerPreferences;
use crate::instruction::Instruction;
use crate::instruction_sequence::InstructionSequence;
use crate::interpreter_util::InterpreterUtil;
use crate::kuna_fixes::{self, KunaFix};
use crate::method_descriptor::MethodDescriptor;
use crate::root_statement::RootStatement;
use crate::rt::{JHashMap, JHashSet, JLinkedHashMap, JLinkedHashSet, JavaEq};
use crate::simple_ssa_reassign::SimpleSSAReassign;
use crate::ssa_constructor_sparse_ex::SSAConstructorSparseEx;
use crate::ssau_constructor_sparse_ex::SSAUConstructorSparseEx;
use crate::stack_vars_processor::StackVarsProcessor;
use crate::stat_edge::StatEdge;
use crate::statement::Statement;
use crate::struct_class::StructClass;
use crate::struct_method::StructMethod;
use crate::validation_helper::ValidationHelper;
use crate::var_exprent::VarExprent;
use crate::var_processor::VarProcessor;
use crate::var_type::VarType;
use crate::var_version_pair::VarVersionPair;
use crate::var_versions_graph::VarVersionsGraph;

/// Inlines `finally` handlers, or guards them with a semaphore variable when the
/// copies cannot be verified. Plain owned value: `MethodProcessor` creates one per
/// method and calls `iterate_graph` until it returns false.
pub struct FinallyProcessor {
    finally_blocks: JHashMap<BasicBlock, Option<i32>>,
    catchall_blocks: JHashSet<BasicBlock>,
    #[allow(dead_code)]
    mt: StructMethod,
    method_descriptor: MethodDescriptor,
    var_processor: VarProcessor,
    ssuversions: Option<VarVersionsGraph>,
    instr_rewrites: Option<JHashMap<Instruction, i32>>,
}

struct Record {
    first_code: i32,
    exception_offset: i32,
    map_last: JLinkedHashMap<BasicBlock, bool>,
}

impl Record {
    fn new(
        first_code: i32,
        exception_offset: i32,
        map_last: JLinkedHashMap<BasicBlock, bool>,
    ) -> Record {
        Record {
            first_code,
            exception_offset,
            map_last,
        }
    }
}

struct Area {
    start: BasicBlock,
    sample: JHashSet<BasicBlock>,
    next: Option<BasicBlock>,
    side_exits: JHashSet<BasicBlock>,
}

impl Area {
    fn new(
        start: BasicBlock,
        sample: JHashSet<BasicBlock>,
        next: Option<BasicBlock>,
        side_exits: JHashSet<BasicBlock>,
    ) -> Area {
        Area {
            start,
            sample,
            next,
            side_exits,
        }
    }
}

/// The local class `BlockStackEntry` of `compareSubgraphsEx`.
struct BlockStackEntry {
    pub block_catch: BasicBlock,
    pub block_sample: BasicBlock,
    pub lst_store_vars: Vec<[i32; 2]>,
}

impl BlockStackEntry {
    fn new(
        block_catch: BasicBlock,
        block_sample: BasicBlock,
        lst_store_vars: &[[i32; 2]],
    ) -> BlockStackEntry {
        BlockStackEntry {
            block_catch,
            block_sample,
            lst_store_vars: lst_store_vars.to_vec(),
        }
    }
}

/// One `BasicBlock[]{from, to, trueExit}` value of `compareSubgraphsEx`'s `mapNext`.
type NextEdge = (BasicBlock, BasicBlock, Option<BasicBlock>);

const STORE_CODES: [i32; 5] = [OPC_ISTORE, OPC_LSTORE, OPC_FSTORE, OPC_DSTORE, OPC_ASTORE];

const NEXT_CODES: [[i32; 2]; 5] = [
    [OPC_ILOAD, OPC_IRETURN],
    [OPC_LLOAD, OPC_LRETURN],
    [OPC_FLOAD, OPC_FRETURN],
    [OPC_DLOAD, OPC_DRETURN],
    [OPC_ALOAD, OPC_ARETURN],
];

impl FinallyProcessor {
    pub fn new(
        mt: StructMethod,
        md: &MethodDescriptor,
        var_proc: VarProcessor,
    ) -> FinallyProcessor {
        FinallyProcessor {
            finally_blocks: JHashMap::new(),
            catchall_blocks: JHashSet::new(),
            mt,
            method_descriptor: md.clone(),
            var_processor: var_proc,
            ssuversions: None,
            instr_rewrites: None,
        }
    }

    pub fn iterate_graph(
        &mut self,
        cl: StructClass,
        mt: StructMethod,
        root: RootStatement,
        graph: ControlFlowGraph,
    ) -> bool {
        self.ssuversions = None;
        let bytecode_version = mt.get_bytecode_version();

        let mut stack: Vec<Statement> = vec![root.0];

        while let Some(stat) = stack.pop() {
            let parent = stat.get_parent();
            if let Some(fin) = parent.and_then(|p| p.try_as_catch_all()) {
                if Some(stat) == fin.get_first() && !fin.is_copied() {
                    let head = fin.get_basichead().get_block();
                    let handler = fin.get_handler().get_basichead().get_block();

                    if self.catchall_blocks.contains(&handler) {
                    } else if self.finally_blocks.contains_key(&handler) {
                        fin.set_finally(true);

                        let var = self.finally_blocks.get(&handler).copied().flatten();
                        fin.set_monitor(var.map(|var| {
                            VarExprent::new(
                                var,
                                Some(VarType::VARTYPE_INT()),
                                Some(self.var_processor),
                            )
                        }));
                    } else {
                        let inf = self.get_finally_information(cl, mt, root, fin);

                        match inf {
                            None => {
                                self.catchall_blocks.add(handler);
                                root.add_comment_error(
                                    "$VF: Could not inline inconsistent finally blocks",
                                    true,
                                );
                            }
                            Some(inf) => {
                                if DecompilerContext::get_option(
                                    IFernflowerPreferences::FINALLY_DEINLINE,
                                ) && self.verify_finally_ex(graph, fin, &inf)
                                {
                                    Self::inline_return_var(graph, handler, &inf);

                                    self.finally_blocks.put(handler, None);
                                } else {
                                    let var_index = DecompilerContext::get_counter_container()
                                        .get_counter_and_increment(CounterContainer::VAR_COUNTER);
                                    self.var_processor
                                        .get_synthetic_semaphores()
                                        .borrow_mut()
                                        .add(var_index);
                                    Self::insert_semaphore(
                                        graph,
                                        &Self::get_all_basic_blocks(
                                            fin.get_first().expect("NullPointerException"),
                                        ),
                                        head,
                                        handler,
                                        var_index,
                                        &inf,
                                        &bytecode_version,
                                    );

                                    self.finally_blocks.put(handler, Some(var_index));

                                    if DecompilerContext::get_option(
                                        IFernflowerPreferences::DECOMPILER_COMMENTS,
                                    ) {
                                        root.add_comment_error("$VF: Could not verify finally blocks. A semaphore variable has been added to preserve control flow.", true);
                                    }
                                }

                                DeadCodeHelper::remove_dead_blocks(graph);
                                DeadCodeHelper::remove_empty_blocks(graph);
                                DeadCodeHelper::merge_basic_blocks(graph);
                            }
                        }

                        return true;
                    }
                } else if Some(stat) == fin.get_first()
                    && kuna_fixes::enabled(KunaFix::CopiedFinally)
                {
                    self.mark_copied_finally(fin);
                }
            }

            let stats = stat.get_stats().borrow().to_vec();
            stack.extend(stats);
        }

        false
    }

    /// A copy of a catch-all that was processed as a `finally` is a `finally` too.
    fn mark_copied_finally(&self, fin: CatchAllStatement) {
        let mut handler = fin.get_handler().get_basichead().get_block();
        while let Some(original) = handler.get_copy_of() {
            handler = original;
        }

        if let Some(var) = self.finally_blocks.get(&handler).copied() {
            fin.set_finally(true);
            fin.set_monitor(var.map(|var| {
                VarExprent::new(var, Some(VarType::VARTYPE_INT()), Some(self.var_processor))
            }));
        }
    }

    fn get_finally_information(
        &mut self,
        cl: StructClass,
        mt: StructMethod,
        root: RootStatement,
        fstat: CatchAllStatement,
    ) -> Option<Record> {
        let proc = ExprProcessor::new(&self.method_descriptor, self.var_processor);
        proc.process_statement(root, cl);

        if self.ssuversions.is_none() {
            let mut ssa = SSAConstructorSparseEx::new();
            ssa.split_variables(root, mt);

            self.instr_rewrites = Some(SimpleSSAReassign::reassign_ssa_form(&ssa, root));

            StackVarsProcessor::set_versions_to_null(root.0);

            let mut ssau = SSAUConstructorSparseEx::new();
            ssau.split_variables(root, mt);

            self.ssuversions = Some(ssau.get_ssu_versions());
            StackVarsProcessor::set_versions_to_null(root.0);
        }

        let mut map_last: JLinkedHashMap<BasicBlock, bool> = JLinkedHashMap::new();

        let first_block_statement = fstat.get_handler().get_basichead();
        let first_basic_block = first_block_statement.get_block();

        if first_basic_block.get_seq().is_empty() {
            return None;
        }

        let instr_first = first_basic_block.get_instruction(0);

        let mut firstcode = 0;

        match instr_first.opcode() {
            OPC_POP => firstcode = 1,
            OPC_ASTORE => firstcode = 2,
            _ => {}
        }

        let mut ssa = SSAConstructorSparseEx::new();
        ssa.split_variables(root, mt);

        let lst_exprents = first_block_statement
            .get_exprents()
            .expect("NullPointerException");

        let varpaar = VarVersionPair::from_var(
            lst_exprents
                .get(if firstcode == 2 { 1 } else { 0 })
                .as_assignment()
                .get_left()
                .as_var(),
        );

        let mut flatthelper = FlattenStatementsHelper::new();
        let dgraph = flatthelper.build_direct_graph(root);

        let mut stack: VecDeque<DirectNode> = VecDeque::new();
        stack.push_back(dgraph.first().expect("NullPointerException"));

        let mut set_visited: JHashSet<DirectNode> = JHashSet::new();

        while let Some(node) = stack.pop_front() {
            if set_visited.contains(&node) {
                continue;
            }
            set_visited.add(node);

            let mut block_statement: Option<BasicBlockStatement> = None;
            if node.block().is_some() {
                block_statement = node.block();
            } else if node.get_predecessors(DirectEdgeType::Regular).len() == 1 {
                block_statement = node
                    .get_predecessors(DirectEdgeType::Regular)
                    .get(0)
                    .get_source()
                    .block();
            }

            let mut is_true_exit = true;

            if firstcode != 1 {
                is_true_exit = false;

                let mut start_idx = 0;
                if Some(first_block_statement) == node.block() {
                    start_idx = if firstcode == 2 { 2 } else { 1 };
                }

                let node_exprents = node.exprents();
                let mut i = start_idx;
                while i < node_exprents.len() {
                    let exprent = node_exprents.get(i);

                    if firstcode == 0 {
                        let mut lst = exprent.get_all_exprents();
                        lst.push(exprent);

                        let mut found = false;
                        for expr in lst {
                            if let Some(var) = expr.try_as_var() {
                                if VarVersionPair::from_var(var).equals(&varpaar) {
                                    found = true;
                                    break;
                                }
                            }
                        }

                        if found {
                            found = false;
                            if let Some(exexpr) = exprent.try_as_exit() {
                                if exexpr.get_exit_type() == ExitType::Throw
                                    && exexpr.get_value().is_some_and(|v| v.is_var())
                                {
                                    found = true;
                                }
                            }

                            if !found {
                                return None;
                            } else {
                                is_true_exit = true;
                            }
                        }
                    } else if firstcode == 2 {
                        if let Some(assexpr) = exprent.try_as_assignment() {
                            if let Some(right) = assexpr.get_right().try_as_var() {
                                if VarVersionPair::from_var(right).equals(&varpaar) {
                                    let mut next: Option<Exprent> = None;
                                    if i == node_exprents.len() - 1 {
                                        if node.get_successors(DirectEdgeType::Regular).len() == 1 {
                                            let nd = node
                                                .get_successors(DirectEdgeType::Regular)
                                                .get(0)
                                                .get_destination();
                                            if !nd.exprents().is_empty() {
                                                next = Some(nd.exprents().get(0));
                                            }
                                        }
                                    } else {
                                        next = Some(node_exprents.get(i + 1));
                                    }

                                    let mut found = false;
                                    if let Some(exexpr) = next.and_then(|n| n.try_as_exit()) {
                                        if exexpr.get_exit_type() == ExitType::Throw
                                            && exexpr.get_value().is_some_and(|v| v.is_var())
                                            && assexpr
                                                .get_left()
                                                .equals(exexpr.get_value().unwrap())
                                        {
                                            found = true;
                                        }
                                    }

                                    if !found {
                                        return None;
                                    } else {
                                        is_true_exit = true;
                                    }
                                }
                            }
                        }
                    }
                    i += 1;
                }
            }

            if let Some(block_statement) = block_statement {
                let handler = fstat.get_handler();
                for edge in block_statement.get_successor_edges(Statement::STATEDGE_DIRECT_ALL) {
                    if edge.get_type() != StatEdge::TYPE_REGULAR
                        && handler.contains_statement(block_statement.0)
                        && !handler.contains_statement(edge.get_destination())
                    {
                        let existing_flag = map_last.get(&block_statement.get_block()).copied();
                        if existing_flag.is_none() || !existing_flag.unwrap() {
                            map_last.put(block_statement.get_block(), is_true_exit);
                            break;
                        }
                    }
                }
            }

            for suc in node.get_successors(DirectEdgeType::Regular).iter() {
                stack.push_back(suc.get_destination());
            }
        }

        if fstat.get_handler().is_basic_block() {
            let mut is_empty = false;
            let is_first_last = map_last.contains_key(&first_basic_block);
            let seq = first_basic_block.get_seq();

            match firstcode {
                0 => is_empty = is_first_last && seq.length() == 1,
                1 => is_empty = seq.length() == 1,
                2 => {
                    is_empty = if is_first_last {
                        seq.length() == 3
                    } else {
                        seq.length() == 1
                    }
                }
                _ => {}
            }

            if is_empty {
                firstcode = 3;
            }
        }

        Some(Record::new(firstcode, instr_first.start_offset(), map_last))
    }

    fn next_block_id(graph: ControlFlowGraph) -> i32 {
        graph.with_mut(|d| {
            d.last_id += 1;
            d.last_id
        })
    }

    fn semaphore_store(
        value: i32,
        var: i32,
        store_length: i32,
        bytecode_version: BytecodeVersion,
    ) -> InstructionSequence {
        let seq = InstructionSequence::new();
        seq.add_instruction(Instruction::create(
            OPC_BIPUSH,
            false,
            GROUP_GENERAL,
            bytecode_version,
            Some(vec![value]),
            -1,
            1,
        ));
        seq.add_instruction(Instruction::create(
            OPC_ISTORE,
            false,
            GROUP_GENERAL,
            bytecode_version,
            Some(vec![var]),
            -1,
            store_length,
        ));
        seq
    }

    fn insert_semaphore(
        graph: ControlFlowGraph,
        set_try: &JLinkedHashSet<BasicBlock>,
        head: BasicBlock,
        handler: BasicBlock,
        var: i32,
        information: &Record,
        bytecode_version: &BytecodeVersion,
    ) {
        let mut set_copy: JHashSet<BasicBlock> = JHashSet::from_collection(set_try.to_vec());

        let finallytype = information.first_code;
        let map_last = &information.map_last;

        Self::remove_exception_instructions_ex(graph, handler, 1, finallytype);
        for (last, value) in map_last.entry_vec() {
            if value {
                Self::remove_exception_instructions_ex(graph, last, 2, finallytype);
                graph.get_finally_exits().borrow_mut().add(last);
            }
        }

        let store_length = if var <= 3 {
            1
        } else if var <= 128 {
            2
        } else {
            4
        };
        for block in set_try.to_vec() {
            let lst_succ = block.get_succs();

            let mut k = 0;
            while k < lst_succ.len() {
                let dest = lst_succ.get(k);
                k += 1;

                if dest != graph.get_last() && !set_copy.contains(&dest) {
                    let seq = Self::semaphore_store(0, var, store_length, *bytecode_version);

                    let newblock = BasicBlock::new(Self::next_block_id(graph));
                    newblock.set_seq(seq);

                    block.replace_successor(dest, newblock);
                    newblock.add_successor(dest);
                    set_copy.add(newblock);
                    graph
                        .get_blocks()
                        .borrow_mut()
                        .add_with_key(newblock, newblock.get_id());

                    let succ_exceptions = block.get_succ_exceptions();
                    let mut j = 0;
                    while j < succ_exceptions.len() {
                        let hd = succ_exceptions.get(j);
                        newblock.add_successor_exception(hd);

                        let range = graph
                            .get_exception_range(hd, block)
                            .expect("NullPointerException");
                        range.get_protected_range().add(newblock);
                        j += 1;
                    }
                }
            }
        }

        let seq = Self::semaphore_store(1, var, store_length, *bytecode_version);

        let newhead = BasicBlock::new(Self::next_block_id(graph));
        newhead.set_seq(seq);

        Self::insert_block_before(graph, head, newhead);

        let seq = Self::semaphore_store(0, var, store_length, *bytecode_version);

        let newheadinit = BasicBlock::new(Self::next_block_id(graph));
        newheadinit.set_seq(seq);

        Self::insert_block_before(graph, newhead, newheadinit);

        set_copy.add(newhead);
        set_copy.add(newheadinit);

        let handlers = JHashSet::from_collection(newheadinit.get_succ_exceptions().to_vec());
        for hd in handlers.to_vec() {
            let range = graph
                .get_exception_range(hd, newheadinit)
                .expect("NullPointerException");

            let protected_range = range.get_protected_range();
            if protected_range
                .iter()
                .all(|block| set_copy.contains(&block))
            {
                newheadinit.remove_successor_exception(hd);
                protected_range.remove_item(&newheadinit);
            }
        }
    }

    fn insert_block_before(graph: ControlFlowGraph, oldblock: BasicBlock, newblock: BasicBlock) {
        let mut lst_temp: Vec<BasicBlock> = Vec::new();
        lst_temp.extend(oldblock.get_preds().to_vec());
        lst_temp.extend(oldblock.get_pred_exceptions().to_vec());

        for pred in lst_temp {
            pred.replace_successor(oldblock, newblock);
        }

        for hd in oldblock.get_succ_exceptions().to_vec() {
            newblock.add_successor_exception(hd);

            let range = graph
                .get_exception_range(hd, oldblock)
                .expect("NullPointerException");
            range.get_protected_range().add(newblock);
        }

        for range in graph.get_exceptions().to_vec() {
            if range.get_handler() == oldblock {
                range.set_handler(newblock);
            }
        }

        newblock.add_successor(oldblock);
        graph
            .get_blocks()
            .borrow_mut()
            .add_with_key(newblock, newblock.get_id());
        if graph.with(|d| d.first) == Some(oldblock) {
            graph.set_first(newblock);
        }
    }

    fn get_all_basic_blocks(stat: Statement) -> JLinkedHashSet<BasicBlock> {
        let mut lst: Vec<Statement> = vec![stat];

        let mut index = 0;
        loop {
            let st = lst[index];

            if st.is_basic_block() {
                index += 1;
            } else {
                let stats = st.get_stats().borrow().to_vec();
                lst.extend(stats);
                lst.remove(index);
            }

            if index >= lst.len() {
                break;
            }
        }

        let mut res = JLinkedHashSet::new();

        for st in lst {
            res.add(st.as_basic_block().get_block());
        }

        res
    }

    fn verify_finally_ex(
        &self,
        graph: ControlFlowGraph,
        fstat: CatchAllStatement,
        information: &Record,
    ) -> bool {
        let try_blocks =
            Self::get_all_basic_blocks(fstat.get_first().expect("NullPointerException"));
        let catch_blocks = Self::get_all_basic_blocks(fstat.get_handler());

        let finallytype = information.first_code;
        let map_last = &information.map_last;

        let mut first = fstat.get_handler().get_basichead().get_block();
        let mut skipped_first = false;

        if finallytype == 3 {
            Self::remove_exception_instructions_ex(graph, first, 3, finallytype);

            if map_last.contains_key(&first) {
                graph.get_finally_exits().borrow_mut().add(first);
            }

            return true;
        } else if first.get_seq().length() == 1 && finallytype > 0 {
            let firstsuc = first.get_succs().get(0);
            if catch_blocks.contains(&firstsuc) {
                first = firstsuc;
                skipped_first = true;
            }
        }

        let mut start_blocks: JLinkedHashSet<BasicBlock> = JLinkedHashSet::new();
        for block in try_blocks.to_vec() {
            start_blocks.add_all(block.get_succs().to_vec());
        }
        start_blocks.remove(&graph.get_last());
        start_blocks.remove_all(&try_blocks);

        let mut lst_areas: Vec<Area> = Vec::new();
        let mut side_exits: Option<JHashSet<BasicBlock>> = None;
        for start in start_blocks.to_vec() {
            let Some(arr) = self.compare_subgraphs_ex(
                graph,
                start,
                &catch_blocks,
                first,
                finallytype,
                map_last,
                skipped_first,
            ) else {
                return false;
            };

            lst_areas.push(arr);
            let arr = lst_areas.last().expect("area");

            match &side_exits {
                None => side_exits = Some(arr.side_exits.clone()),
                Some(side_exits) => ValidationHelper::validate_true(
                    side_exits.java_equals(&arr.side_exits),
                    "Side exits are not equal",
                ),
            }
        }

        for area in &lst_areas {
            Self::delete_area(graph, area);
        }

        let mut true_exit: Option<BasicBlock> = None;
        for (last, value) in map_last.entry_vec() {
            if value {
                Self::remove_exception_instructions_ex(graph, last, 2, finallytype);
                graph.get_finally_exits().borrow_mut().add(last);
                ValidationHelper::validate_true(true_exit.is_none(), "More than one true exit");
                true_exit = Some(last);
            }
        }

        Self::remove_exception_instructions_ex(
            graph,
            fstat.get_handler().get_basichead().get_block(),
            1,
            finallytype,
        );

        true
    }

    #[allow(clippy::too_many_arguments)]
    fn compare_subgraphs_ex(
        &self,
        graph: ControlFlowGraph,
        start_sample: BasicBlock,
        catch_blocks: &JLinkedHashSet<BasicBlock>,
        start_catch: BasicBlock,
        finallytype: i32,
        map_last: &JLinkedHashMap<BasicBlock, bool>,
        skipped_first: bool,
    ) -> Option<Area> {
        let mut stack: VecDeque<BlockStackEntry> = VecDeque::new();

        let mut set_sample: JHashSet<BasicBlock> = JHashSet::new();

        let mut map_next: JHashMap<String, NextEdge> = JHashMap::new();

        stack.push_back(BlockStackEntry::new(start_catch, start_sample, &[]));

        while let Some(mut entry) = stack.pop_front() {
            let block_catch = entry.block_catch;
            let block_sample = entry.block_sample;

            let is_first_block = !skipped_first && block_catch == start_catch;
            let is_last_block = map_last.contains_key(&block_catch);
            let is_true_last_block =
                is_last_block && *map_last.get(&block_catch).expect("NullPointerException");

            if !self.compare_basic_blocks_ex(
                graph,
                block_catch,
                block_sample,
                (if is_first_block { 1 } else { 0 }) | (if is_true_last_block { 2 } else { 0 }),
                finallytype,
                &mut entry.lst_store_vars,
            ) {
                return None;
            }

            if block_sample.get_succs().len() != block_catch.get_succs().len() {
                return None;
            }

            set_sample.add(block_sample);

            let mut i = 0;
            while i < block_catch.get_succs().len() {
                let suc_catch = block_catch.get_succs().get(i);
                let suc_sample = block_sample.get_succs().get(i);

                if catch_blocks.contains(&suc_catch) && !set_sample.contains(&suc_sample) {
                    stack.push_back(BlockStackEntry::new(
                        suc_catch,
                        suc_sample,
                        &entry.lst_store_vars,
                    ));
                }
                i += 1;
            }

            if !(is_last_block && block_sample.get_seq().is_empty()) {
                if block_catch.get_succ_exceptions().len()
                    == block_sample.get_succ_exceptions().len()
                {
                    let mut i = 0;
                    while i < block_catch.get_succ_exceptions().len() {
                        let suc_catch = block_catch.get_succ_exceptions().get(i);
                        let suc_sample = block_sample.get_succ_exceptions().get(i);

                        let exc_catch = graph
                            .get_exception_range(suc_catch, block_catch)
                            .expect("NullPointerException")
                            .get_unique_exceptions_string();
                        let exc_sample = graph
                            .get_exception_range(suc_sample, block_sample)
                            .expect("NullPointerException")
                            .get_unique_exceptions_string();

                        let equalexc = exc_catch == exc_sample;

                        if equalexc {
                            if catch_blocks.contains(&suc_catch)
                                && !set_sample.contains(&suc_sample)
                            {
                                let mut lst: &[[i32; 2]] = &entry.lst_store_vars;
                                let extended: Vec<[i32; 2]>;

                                if suc_catch.get_seq().length() > 0
                                    && suc_sample.get_seq().length() > 0
                                {
                                    let instr_catch = suc_catch.get_seq().get_instr(0);
                                    let instr_sample = suc_sample.get_seq().get_instr(0);

                                    if instr_catch.opcode() == OPC_ASTORE
                                        && instr_sample.opcode() == OPC_ASTORE
                                    {
                                        let mut copy = lst.to_vec();
                                        copy.push([
                                            instr_catch.operand(0),
                                            instr_sample.operand(0),
                                        ]);
                                        extended = copy;
                                        lst = &extended;
                                    }
                                }

                                stack.push_back(BlockStackEntry::new(suc_catch, suc_sample, lst));
                            }
                        } else {
                            return None;
                        }
                        i += 1;
                    }
                } else {
                    return None;
                }
            }

            if is_last_block {
                let mut set_succs = JHashSet::from_collection(block_sample.get_succs().to_vec());
                set_succs.remove_all(&set_sample);

                for stackent in &stack {
                    set_succs.remove(&stackent.block_sample);
                }

                for succ in set_succs.to_vec() {
                    if graph.get_last() != succ {
                        map_next.put(
                            format!("{}#{}", block_sample.get_id(), succ.get_id()),
                            (
                                block_sample,
                                succ,
                                if is_true_last_block { Some(succ) } else { None },
                            ),
                        );
                    }
                }
            }
        }

        let next = Self::get_unique_next(graph, &map_next.value_vec());
        let side_exits = Self::get_side_exits(&map_next.value_vec());
        Some(Area::new(start_sample, set_sample, next, side_exits))
    }

    fn get_side_exits(set_next: &[NextEdge]) -> JHashSet<BasicBlock> {
        let mut set = JHashSet::new();

        for next in set_next {
            if next.2.is_none() {
                set.add(next.1);
            }
        }

        set
    }

    /// Java iterates a `HashSet<BasicBlock[]>` (identity-hashed arrays); this
    /// walks `set_next` in `mapNext`'s value order instead.
    fn get_unique_next(graph: ControlFlowGraph, set_next: &[NextEdge]) -> Option<BasicBlock> {
        let mut next: Option<BasicBlock> = None;
        let mut multiple = false;

        for arr in set_next {
            if arr.2.is_some() {
                next = Some(arr.1);
                multiple = false;
                break;
            } else {
                if next.is_none() {
                    next = Some(arr.1);
                } else if next != Some(arr.1) {
                    multiple = true;
                }

                if arr.1.get_preds().len() == 1 {
                    next = Some(arr.1);
                }
            }
        }

        if multiple {
            let next = next.expect("NullPointerException");
            for arr in set_next {
                let block = arr.1;

                if block != next {
                    if InterpreterUtil::equal_sets(
                        Some(&next.get_succs().to_vec()),
                        Some(&block.get_succs().to_vec()),
                    ) {
                        let seq_next = next.get_seq();
                        let seq_block = block.get_seq();

                        if seq_next.length() == seq_block.length() {
                            for i in 0..seq_next.length() {
                                let instr_next = seq_next.get_instr(i);
                                let instr_block = seq_block.get_instr(i);

                                if !Instruction::equals(Some(instr_next), Some(instr_block)) {
                                    return None;
                                }
                                for j in 0..instr_next.operands_count() {
                                    if instr_next.operand(j) != instr_block.operand(j) {
                                        return None;
                                    }
                                }
                            }
                        } else {
                            return None;
                        }
                    } else {
                        return None;
                    }
                }
            }

            for arr in set_next {
                if arr.1 != next {
                    arr.0.remove_successor(arr.1);
                    arr.0.add_successor(next);
                }
            }

            DeadCodeHelper::remove_dead_blocks(graph);
        }

        next
    }

    fn compare_basic_blocks_ex(
        &self,
        graph: ControlFlowGraph,
        pattern: BasicBlock,
        sample: BasicBlock,
        type_: i32,
        finallytype: i32,
        lst_store_vars: &mut Vec<[i32; 2]>,
    ) -> bool {
        let mut seq_pattern = pattern.get_seq();
        let seq_sample = sample.get_seq();
        let instr_old_offsets_sample = sample.get_instr_old_offsets();

        if type_ != 0 {
            seq_pattern = seq_pattern.java_clone();

            if (type_ & 1) > 0 && finallytype > 0 {
                seq_pattern.remove_instruction(0);
            }

            if (type_ & 2) > 0 {
                if finallytype == 0 || finallytype == 2 {
                    seq_pattern.remove_last();
                }

                if finallytype == 2 {
                    seq_pattern.remove_last();
                }
            }
        }

        if seq_pattern.length() > seq_sample.length() {
            return false;
        }

        for i in 0..seq_pattern.length() {
            let instr_pattern = seq_pattern.get_instr(i);
            let instr_sample = seq_sample.get_instr(i);

            if !self.equal_instructions(instr_pattern, instr_sample, lst_store_vars) {
                return false;
            }
        }

        if seq_pattern.length() < seq_sample.length() {
            let seq = InstructionSequence::new();
            let mut old_offsets: VecDeque<i32> = VecDeque::new();
            let mut i = seq_sample.length() - 1;
            while i >= seq_pattern.length() {
                seq.add_instruction_at(0, seq_sample.get_instr(i));
                old_offsets.push_front(sample.get_old_offset(i));
                seq_sample.remove_instruction(i);
                if (i as usize) < instr_old_offsets_sample.len() {
                    instr_old_offsets_sample.remove_at(i as usize);
                }
                i -= 1;
            }

            let newblock = BasicBlock::new(Self::next_block_id(graph));
            newblock.set_seq(seq);
            newblock.get_instr_old_offsets().add_all(old_offsets);

            let lst_temp = sample.get_succs().to_vec();

            for suc in lst_temp {
                sample.remove_successor(suc);
                newblock.add_successor(suc);
            }

            sample.add_successor(newblock);

            graph
                .get_blocks()
                .borrow_mut()
                .add_with_key(newblock, newblock.get_id());

            let set_finally_exits = graph.get_finally_exits();
            let is_finally_exit = set_finally_exits.borrow().contains(&sample);
            if is_finally_exit {
                set_finally_exits.borrow_mut().remove(&sample);
                set_finally_exits.borrow_mut().add(newblock);
            }

            let succ_exceptions = sample.get_succ_exceptions();
            let mut j = 0;
            while j < succ_exceptions.len() {
                let hd = succ_exceptions.get(j);
                newblock.add_successor_exception(hd);

                let range = graph
                    .get_exception_range(hd, sample)
                    .expect("NullPointerException");
                range.get_protected_range().add(newblock);
                j += 1;
            }
        }

        true
    }

    pub fn equal_instructions(
        &self,
        first: Instruction,
        second: Instruction,
        lst_store_vars: &mut Vec<[i32; 2]>,
    ) -> bool {
        if !Instruction::equals(Some(first), Some(second)) {
            return false;
        }

        if first.group() != GROUP_JUMP && first.group() != GROUP_SWITCH {
            for i in 0..first.operands_count() {
                let mut first_op = first.operand(i);
                let mut second_op = second.operand(i);
                if first_op != second_op {
                    if first.opcode() == OPC_ALOAD {
                        for arr in lst_store_vars.iter() {
                            if arr[0] == first_op && arr[1] == second_op {
                                return true;
                            }
                        }
                    } else if first.opcode() == OPC_ASTORE {
                        lst_store_vars.push([first_op, second_op]);
                        return true;
                    }

                    let mut ok = false;
                    if Self::is_opc_var(first.opcode()) {
                        let instr_rewrites =
                            self.instr_rewrites.as_ref().expect("NullPointerException");
                        if let Some(&rewritten) = instr_rewrites.get(&first) {
                            first_op = rewritten;
                        }
                        if let Some(&rewritten) = instr_rewrites.get(&second) {
                            second_op = rewritten;
                        }

                        if self
                            .ssuversions
                            .expect("NullPointerException")
                            .are_vars_analogous(first_op, second_op)
                        {
                            ok = true;
                        }
                    }

                    if !ok {
                        return false;
                    }
                }
            }
        }

        true
    }

    fn is_opc_var(opc: i32) -> bool {
        (OPC_ILOAD..=OPC_SASTORE).contains(&opc) || opc == OPC_IINC
    }

    fn delete_area(graph: ControlFlowGraph, area: &Area) {
        let start = area.start;
        let mut next = area.next;

        if Some(start) == next {
            return;
        }

        if next.is_none() {
            next = Some(graph.get_last());
        }
        let next = next.expect("next");

        let mut set_common_exception_handlers: JHashSet<BasicBlock> =
            JHashSet::from_collection(next.get_succ_exceptions().to_vec());
        for pred in start.get_preds().to_vec() {
            let pred_exceptions = pred.get_succ_exceptions();
            set_common_exception_handlers.retain(|hd| pred_exceptions.contains(hd));
        }

        let mut is_outside_range = false;

        let set_predecessors: JHashSet<BasicBlock> =
            JHashSet::from_collection(start.get_preds().to_vec());

        for pred in set_predecessors.to_vec() {
            pred.replace_successor(start, next);
        }

        let set_blocks = &area.sample;

        let mut set_common_removed_exception_ranges: Option<JHashSet<Option<ExceptionRangeCFG>>> =
            None;

        for block in set_blocks.to_vec() {
            let in_graph = graph.get_blocks().borrow().contains_key(&block.get_id());
            if in_graph {
                let succ_exceptions = block.get_succ_exceptions();
                if !set_common_exception_handlers
                    .iter()
                    .all(|hd| succ_exceptions.contains(hd))
                {
                    is_outside_range = true;
                }

                let mut set_removed_exception_ranges: JHashSet<Option<ExceptionRangeCFG>> =
                    JHashSet::new();
                for handler in block.get_succ_exceptions().to_vec() {
                    set_removed_exception_ranges.add(graph.get_exception_range(handler, block));
                }

                if let Some(common) = set_common_removed_exception_ranges.as_mut() {
                    common.retain_all(&set_removed_exception_ranges);
                } else {
                    set_common_removed_exception_ranges = Some(set_removed_exception_ranges);
                }

                if block.get_seq().is_empty() && block.get_succs().len() == 1 {
                    let succs = block.get_succs().get(0);
                    for pred in block.get_preds().to_vec() {
                        if !set_blocks.contains(&pred) {
                            pred.replace_successor(block, succs);
                        }
                    }

                    if graph.with(|d| d.first) == Some(block) {
                        graph.set_first(succs);
                    }
                }

                graph.remove_block(block);
            }
        }

        if is_outside_range {
            let emptyblock = BasicBlock::new(Self::next_block_id(graph));

            graph
                .get_blocks()
                .borrow_mut()
                .add_with_key(emptyblock, emptyblock.get_id());

            let ranges = set_common_removed_exception_ranges
                .as_ref()
                .expect("NullPointerException")
                .to_vec();
            for range in ranges {
                let range = range.expect("NullPointerException");
                emptyblock.add_successor_exception(range.get_handler());
                range.get_protected_range().add(emptyblock);
            }

            emptyblock.add_successor(next);
            for pred in set_predecessors.to_vec() {
                pred.replace_successor(next, emptyblock);
            }
        }
    }

    /// kuna: whether `block` is a handler copy (no offsets) that holds the whole
    /// handler, i.e. it leaves the method only by throwing. A copy of the first
    /// block of a longer handler shares the rest with the other copies.
    fn is_whole_handler_copy(graph: ControlFlowGraph, block: BasicBlock) -> bool {
        let last = graph.get_last();
        block.get_instr_old_offsets().is_empty()
            && block.get_succs().iter().all(|succ| succ == last)
    }

    fn remove_exception_instructions_ex(
        graph: ControlFlowGraph,
        block: BasicBlock,
        blocktype: i32,
        finallytype: i32,
    ) {
        let seq = block.get_seq();
        let instr_old_offsets = block.get_instr_old_offsets();
        let missing_ok = kuna_fixes::enabled(KunaFix::HandlerCopyOffsets)
            && Self::is_whole_handler_copy(graph, block);
        let remove_offset = |index: i32| {
            if missing_ok {
                if (0..instr_old_offsets.len() as i32).contains(&index) {
                    instr_old_offsets.remove_at(index as usize);
                }
            } else {
                instr_old_offsets.remove_at(index as usize);
            }
        };

        if finallytype == 3 {
            let mut i = seq.length() - 1;
            while i >= 0 {
                seq.remove_instruction(i);
                remove_offset(i);
                i -= 1;
            }
        } else {
            if (blocktype & 1) > 0 && (finallytype == 2 || finallytype == 1) {
                seq.remove_instruction(0);
                remove_offset(0);
            }

            if (blocktype & 2) > 0 {
                if finallytype == 2 || finallytype == 0 {
                    seq.remove_last();
                    remove_offset(instr_old_offsets.len() as i32 - 1);
                }

                if finallytype == 2 {
                    seq.remove_last();
                    remove_offset(instr_old_offsets.len() as i32 - 1);
                }
            }
        }
    }

    fn inline_return_var(graph: ControlFlowGraph, handler: BasicBlock, inf: &Record) {
        let mut ranges: Vec<ExceptionRangeCFG> = Vec::new();

        for ex in graph.get_exceptions().to_vec() {
            if ex.get_handler() == handler {
                ranges.push(ex);
            }
        }

        let mut exits: JHashSet<BasicBlock> = JHashSet::new();

        for ex in &ranges {
            for block in ex.get_protected_range().to_vec() {
                let block_ex = block.get_succ_exceptions().to_vec();

                for suc in block.get_succs().to_vec() {
                    if suc.get_succ_exceptions().to_vec() != block_ex {
                        exits.add(block);
                    }
                }
            }
        }

        for exit in exits.to_vec() {
            if exit.get_succs().len() == 1 && !exit.get_seq().is_empty() {
                let instr = exit.get_last_instruction().expect("NullPointerException");

                let index = Self::index_of(instr);

                if index >= 0 {
                    let succ = exit.get_succs().get(0);
                    if succ.get_preds().len() != 1 || !succ.get_pred_exceptions().is_empty() {
                        continue;
                    }

                    let next_seq = succ.get_seq();
                    if next_seq.length() == 2
                        && next_seq.get_instr(0).start_offset() < inf.exception_offset
                        && next_seq.get_instr(0).opcode() == NEXT_CODES[index as usize][0]
                        && next_seq.get_instr(1).opcode() == NEXT_CODES[index as usize][1]
                        && instr.operand(0) == next_seq.get_instr(0).operand(0)
                    {
                        exit.get_seq().remove_last();
                        exit.get_seq().add_instruction(next_seq.get_instr(1));

                        next_seq.clear();
                    }
                }
            }
        }
    }

    fn index_of(instr: Instruction) -> i32 {
        let opcode = instr.opcode();
        for (i, &code) in STORE_CODES.iter().enumerate() {
            if opcode == code {
                return i as i32;
            }
        }

        -1
    }
}
