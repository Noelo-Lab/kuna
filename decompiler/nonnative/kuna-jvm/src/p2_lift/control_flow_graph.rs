//! Port of `org/jetbrains/java/decompiler/code/cfg/ControlFlowGraph.java`.

use std::collections::VecDeque;

use crate::basic_block::BasicBlock;
use crate::code_constants::{
    GROUP_FIELDACCESS, GROUP_GENERAL, GROUP_INVOCATION, GROUP_JUMP, GROUP_RETURN, GROUP_SWITCH,
    OPC_ASTORE, OPC_JSR, OPC_POP, OPC_RET,
};
use crate::code_type::CodeType;
use crate::constant_pool::ConstantPool;
use crate::data_point::DataPoint;
use crate::dead_code_helper::DeadCodeHelper;
use crate::decompiler_context::DecompilerContext;
use crate::exception_range_cfg::ExceptionRangeCFG;
use crate::full_instruction_sequence::FullInstructionSequence;
use crate::instruction_impact::InstructionImpact;
use crate::rt::{JHashMap, JHashSet, JLinkedHashMap, JLinkedHashSet, JList, Shared};
use crate::struct_class::StructClass;
use crate::struct_method::StructMethod;
use crate::var_type::VarType;
use crate::vb_style_collection::VBStyleCollection;
use crate::{arena_identity, arena_object, jthrow};

pub struct ControlFlowGraphData {
    pub last_id: i32,
    pub blocks: Shared<VBStyleCollection<BasicBlock, i32>>,
    pub first: Option<BasicBlock>,
    pub last: Option<BasicBlock>,
    pub exceptions: JList<ExceptionRangeCFG>,
    pub subroutines: JLinkedHashMap<BasicBlock, BasicBlock>,
    pub finally_exits: Shared<JHashSet<BasicBlock>>,
    pub sequence: FullInstructionSequence,
    pub comment_lines: Option<JLinkedHashSet<String>>,
    pub add_error_comment: bool,
}

arena_object!(pub ControlFlowGraph => ControlFlowGraphData, Class);
arena_identity!(ControlFlowGraph);

struct JsrRecord {
    jsr: BasicBlock,
    range: JHashSet<BasicBlock>,
    ret: BasicBlock,
}

impl ControlFlowGraph {
    pub fn new(seq: FullInstructionSequence) -> ControlFlowGraph {
        let graph = ControlFlowGraph::alloc(ControlFlowGraphData {
            last_id: 0,
            blocks: Shared::new(VBStyleCollection::new()),
            first: None,
            last: None,
            exceptions: JList::new(),
            subroutines: JLinkedHashMap::new(),
            finally_exits: Shared::new(JHashSet::new()),
            sequence: seq,
            comment_lines: None,
            add_error_comment: false,
        });

        graph.build_blocks(seq);

        graph
    }

    fn block_list(self) -> Vec<BasicBlock> {
        self.get_blocks().borrow().iter().copied().collect()
    }

    pub fn remove_markers(self) {
        for block in self.block_list() {
            block.set_mark(0);
        }
    }

    pub fn inline_jsr(self, cl: StructClass, mt: StructMethod) {
        self.process_jsr();
        self.remove_jsr(cl, mt);

        self.remove_markers();

        DeadCodeHelper::remove_empty_blocks(self);
    }

    pub fn remove_block(self, block: BasicBlock) {
        while !block.get_succs().is_empty() {
            block.remove_successor(block.get_succs().get(0));
        }

        while !block.get_succ_exceptions().is_empty() {
            block.remove_successor_exception(block.get_succ_exceptions().get(0));
        }

        while !block.get_preds().is_empty() {
            block.get_preds().get(0).remove_successor(block);
        }

        while !block.get_pred_exceptions().is_empty() {
            block
                .get_pred_exceptions()
                .get(0)
                .remove_successor_exception(block);
        }

        self.get_last().remove_predecessor(block);

        self.get_blocks()
            .borrow_mut()
            .remove_with_key(&block.get_id());

        let exceptions = self.get_exceptions();
        for i in (0..exceptions.len()).rev() {
            let range = exceptions.get(i);
            if range.get_handler() == block {
                exceptions.remove_at(i);
            } else {
                let lst_range = range.get_protected_range();
                lst_range.remove_item(&block);

                if lst_range.is_empty() {
                    exceptions.remove_at(i);
                }
            }
        }

        self.with_mut(|d| d.subroutines.retain(|k, v| *k != block && *v != block));
    }

    pub fn get_exception_range(
        self,
        handler: BasicBlock,
        block: BasicBlock,
    ) -> Option<ExceptionRangeCFG> {
        let exceptions = self.get_exceptions();
        for i in (0..exceptions.len()).rev() {
            let range = exceptions.get(i);
            if range.get_handler() == handler && range.get_protected_range().contains(&block) {
                return Some(range);
            }
        }

        None
    }

    fn build_blocks(self, instrseq: FullInstructionSequence) {
        let mut states = Self::find_start_instructions(instrseq);

        let map_instr_blocks = self.create_basic_blocks(&mut states, instrseq);

        self.connect_blocks(&map_instr_blocks);

        self.set_exception_edges(instrseq, &map_instr_blocks);

        self.set_subroutine_edges();
    }

    fn find_start_instructions(seq: FullInstructionSequence) -> Vec<i16> {
        let len = seq.length();
        let mut inststates = vec![0i16; len as usize];

        for handler in seq.exception_table().get_handlers().to_vec() {
            inststates[handler.from() as usize] = 1;
            inststates[handler.handler() as usize] = 1;

            if handler.to() < len {
                inststates[handler.to() as usize] = 1;
            }
        }

        for i in 0..len {
            let instr = seq.get_instr(i);
            let group = instr.with(|d| d.group);
            match group {
                GROUP_JUMP | GROUP_RETURN => {
                    if group == GROUP_JUMP {
                        inststates[instr.as_jump().d(|d| d.destination) as usize] = 1;
                    }
                    if i + 1 < len {
                        inststates[(i + 1) as usize] = 1;
                    }
                }
                GROUP_SWITCH => {
                    let swinstr = instr.as_switch();
                    let dests = swinstr.get_destinations();
                    for j in (0..dests.len()).rev() {
                        inststates[dests[j] as usize] = 1;
                    }
                    inststates[swinstr.get_default_destination() as usize] = 1;
                    if i + 1 < len {
                        inststates[(i + 1) as usize] = 1;
                    }
                }
                _ => {}
            }
        }

        inststates[0] = 1;

        inststates
    }

    fn create_basic_blocks(
        self,
        startblock: &mut [i16],
        instr_seq: FullInstructionSequence,
    ) -> JHashMap<i32, BasicBlock> {
        let mut map_instr_blocks: JHashMap<i32, BasicBlock> = JHashMap::new();

        let blocks: Shared<VBStyleCollection<BasicBlock, i32>> =
            Shared::new(VBStyleCollection::new());
        self.with_mut(|d| d.blocks = blocks.clone());

        let mut currseq = None;
        let mut lst_offs: Option<JList<i32>> = None;

        let len = startblock.len();
        let mut counter: i16 = 0;

        let mut current_block: Option<BasicBlock> = None;
        for (i, state) in startblock.iter_mut().enumerate() {
            if *state == 1 {
                counter = counter.wrapping_add(1);
                let block = BasicBlock::new(counter as i32);

                currseq = Some(block.get_seq());
                lst_offs = Some(block.get_instr_old_offsets());

                blocks.borrow_mut().add_with_key(block, block.get_id());
                current_block = Some(block);
            }

            *state = counter;
            map_instr_blocks.put(i as i32, current_block.expect("NullPointerException"));

            let instr = instr_seq.get_instr(i as i32);
            currseq
                .expect("NullPointerException")
                .add_instruction(instr);
            lst_offs
                .as_ref()
                .expect("NullPointerException")
                .add(instr.with(|d| d.start_offset));
        }

        let first = blocks.borrow().get(0);
        counter = counter.wrapping_add(1);
        let last = BasicBlock::new(counter as i32);
        map_instr_blocks.put(len as i32, last);

        self.with_mut(|d| {
            d.first = Some(first);
            d.last = Some(last);
            d.last_id = counter as i32;
        });

        map_instr_blocks
    }

    fn connect_blocks(self, map_instr_blocks: &JHashMap<i32, BasicBlock>) {
        let blocks = self.get_blocks();
        let last = self.get_last();
        let size = blocks.borrow().len() as i32;

        for i in 0..size {
            let block = blocks.borrow().get(i as usize);
            let instr = block.get_last_instruction().expect("NullPointerException");

            match instr.with(|d| d.group) {
                GROUP_JUMP => {
                    let dest = instr.as_jump().d(|d| d.destination);

                    block.add_successor(
                        map_instr_blocks
                            .get_cloned(&dest)
                            .expect("NullPointerException"),
                    );

                    if instr.can_fall_through() {
                        if i >= size - 1 {
                            continue;
                        }
                        block.add_successor(blocks.borrow().get((i + 1) as usize));
                    }
                }
                GROUP_SWITCH => {
                    let sw_instr = instr.as_switch();

                    let default_destination = sw_instr.get_default_destination();
                    block.add_successor(
                        map_instr_blocks
                            .get_cloned(&default_destination)
                            .expect("NullPointerException"),
                    );

                    for destination in sw_instr.get_destinations() {
                        block.add_successor(
                            map_instr_blocks
                                .get_cloned(&destination)
                                .expect("NullPointerException"),
                        );
                    }
                }
                GROUP_RETURN => {
                    if instr.with(|d| d.opcode) != OPC_RET {
                        last.add_predecessor(block);
                    }
                }
                GROUP_GENERAL | GROUP_INVOCATION | GROUP_FIELDACCESS => {
                    if i >= size - 1 {
                        continue;
                    }
                    let next_block = blocks.borrow().get((i + 1) as usize);
                    block.add_successor(next_block);
                }
                _ => jthrow!(IllegalStateException; "Unknown instruction group"),
            }
        }
    }

    fn set_exception_edges(
        self,
        instrseq: FullInstructionSequence,
        instr_blocks: &JHashMap<i32, BasicBlock>,
    ) {
        let exceptions: JList<ExceptionRangeCFG> = JList::new();
        self.with_mut(|d| d.exceptions = exceptions.clone());

        let mut map_ranges: JHashMap<String, ExceptionRangeCFG> = JHashMap::new();
        let blocks = self.get_blocks();

        for handler in instrseq.exception_table().get_handlers().to_vec() {
            let from = instr_blocks
                .get_cloned(&handler.from())
                .expect("NullPointerException");
            let to = instr_blocks
                .get_cloned(&handler.to())
                .expect("NullPointerException");
            let handle = instr_blocks
                .get_cloned(&handler.handler())
                .expect("NullPointerException");

            let key = format!("{}:{}:{}", from.get_id(), to.get_id(), handle.get_id());

            if map_ranges.contains_key(&key) {
                let range = map_ranges.get_cloned(&key).expect("range");
                range.add_exception_type(handler.exception_class());
            } else {
                let protected_range: JList<BasicBlock> = JList::new();
                for j in from.get_id()..to.get_id() {
                    let block = blocks
                        .borrow()
                        .get_with_key(&j)
                        .expect("NullPointerException");
                    protected_range.add(block);
                    block.add_successor_exception(handle);
                }

                let range = ExceptionRangeCFG::new_single(
                    protected_range,
                    handle,
                    handler.exception_class(),
                );
                map_ranges.put(key, range);

                exceptions.add(range);
            }
        }
    }

    fn set_subroutine_edges(self) {
        let mut subroutines: JLinkedHashMap<BasicBlock, BasicBlock> = JLinkedHashMap::new();
        let blocks = self.get_blocks();

        for block in self.block_list() {
            if block.get_seq().get_last_instr().with(|d| d.opcode) == OPC_JSR {
                let mut stack: VecDeque<BasicBlock> = VecDeque::new();
                let mut stack_jsr_stacks: VecDeque<Vec<BasicBlock>> = VecDeque::new();

                let mut set_visited: JHashSet<BasicBlock> = JHashSet::new();

                stack.push_back(block);
                stack_jsr_stacks.push_back(Vec::new());

                while let Some(node) = stack.pop_front() {
                    let mut jsrstack = stack_jsr_stacks
                        .pop_front()
                        .expect("NoSuchElementException");

                    set_visited.add(node);

                    match node.get_seq().get_last_instr().with(|d| d.opcode) {
                        OPC_JSR => jsrstack.push(node),
                        OPC_RET => {
                            let enter = *jsrstack.last().expect("NoSuchElementException");
                            let exit = blocks.borrow().get_with_key(&(enter.get_id() + 1));

                            match exit {
                                Some(exit) => {
                                    if !node.is_successor(exit) {
                                        node.add_successor(exit);
                                    }
                                    jsrstack.pop();
                                    subroutines.put(enter, exit);
                                }
                                None => jthrow!("ERROR: last instruction jsr"),
                            }
                        }
                        _ => {}
                    }

                    if !jsrstack.is_empty() {
                        for succ in node.get_succs().iter() {
                            if !set_visited.contains(&succ) {
                                stack.push_back(succ);
                                stack_jsr_stacks.push_back(jsrstack.clone());
                            }
                        }
                    }
                }
            }
        }

        self.with_mut(|d| d.subroutines = subroutines);
    }

    fn process_jsr(self) {
        loop {
            if self.process_jsr_ranges() == 0 {
                break;
            }
        }
    }

    fn process_jsr_ranges(self) -> i32 {
        let mut lst_jsr_all: Vec<JsrRecord> = Vec::new();

        for (jsr, ret) in self.with(|d| d.subroutines.entry_vec()) {
            lst_jsr_all.push(JsrRecord {
                jsr,
                range: self.get_jsr_range(jsr, ret),
                ret,
            });
        }

        let mut lst_jsr: Vec<JsrRecord> = Vec::new();
        for arr in lst_jsr_all {
            let mut i = 0;
            while i < lst_jsr.len() {
                if lst_jsr[i].range.contains(&arr.jsr) {
                    break;
                }
                i += 1;
            }
            lst_jsr.insert(i, arr);
        }

        for i in 0..lst_jsr.len() {
            for j in (i + 1)..lst_jsr.len() {
                let (arr, arr1) = (&lst_jsr[i], &lst_jsr[j]);
                let set = &arr.range;
                let set1 = &arr1.range;

                if !set.contains(&arr1.jsr) && !set1.contains(&arr.jsr) {
                    let mut setc = set.clone();
                    setc.retain_all(set1);

                    if !setc.is_empty() {
                        let (jsr, ret) = (arr.jsr, arr.ret);
                        self.split_jsr_range(jsr, ret, &setc);
                        return 1;
                    }
                }
            }
        }

        0
    }

    fn get_jsr_range(self, jsr: BasicBlock, ret: BasicBlock) -> JHashSet<BasicBlock> {
        let mut blocks: JHashSet<BasicBlock> = JHashSet::new();

        let mut lst_nodes: VecDeque<BasicBlock> = VecDeque::new();
        lst_nodes.push_back(jsr);

        let dom = jsr.get_succs().get(0);
        let last = self.get_last();

        while let Some(node) = lst_nodes.pop_front() {
            for j in 0..2 {
                let lst = if j == 0 {
                    if node
                        .get_last_instruction()
                        .expect("NullPointerException")
                        .with(|d| d.opcode)
                        == OPC_RET
                        && node.get_succs().contains(&ret)
                    {
                        continue;
                    }
                    node.get_succs()
                } else {
                    if node == jsr {
                        continue;
                    }
                    node.get_succ_exceptions()
                };

                'child: for i in (0..lst.len()).rev() {
                    let child = lst.get(i);
                    if !blocks.contains(&child) {
                        if node != jsr {
                            let preds = child.get_preds();
                            for k in 0..preds.len() {
                                if !DeadCodeHelper::is_dominator(self, preds.get(k), dom) {
                                    continue 'child;
                                }
                            }

                            let pred_exceptions = child.get_pred_exceptions();
                            for k in 0..pred_exceptions.len() {
                                if !DeadCodeHelper::is_dominator(self, pred_exceptions.get(k), dom)
                                {
                                    continue 'child;
                                }
                            }
                        }

                        if child != last {
                            blocks.add(child);
                        }

                        lst_nodes.push_back(child);
                    }
                }
            }
        }

        blocks
    }

    fn split_jsr_range(
        self,
        jsr: BasicBlock,
        ret: BasicBlock,
        common_blocks: &JHashSet<BasicBlock>,
    ) {
        let mut lst_nodes: VecDeque<BasicBlock> = VecDeque::new();
        let mut map_new_nodes: JHashMap<i32, BasicBlock> = JHashMap::new();

        lst_nodes.push_back(jsr);
        map_new_nodes.put(jsr.get_id(), jsr);

        let last = self.get_last();
        let blocks = self.get_blocks();

        while let Some(node) = lst_nodes.pop_front() {
            for j in 0..2 {
                let lst = if j == 0 {
                    if node
                        .get_last_instruction()
                        .expect("NullPointerException")
                        .with(|d| d.opcode)
                        == OPC_RET
                        && node.get_succs().contains(&ret)
                    {
                        continue;
                    }
                    node.get_succs()
                } else {
                    if node == jsr {
                        continue;
                    }
                    node.get_succ_exceptions()
                };

                let mut i = lst.len() as i32 - 1;
                while i >= 0 {
                    let child = lst.get(i as usize);
                    let childid = child.get_id();

                    if let Some(mapped) = map_new_nodes.get_cloned(&childid) {
                        node.replace_successor(child, mapped);
                    } else if common_blocks.contains(&child) {
                        let id = self.with_mut(|d| {
                            d.last_id += 1;
                            d.last_id
                        });
                        let copy = child.clone_block(id);

                        if copy
                            .get_last_instruction()
                            .expect("NullPointerException")
                            .with(|d| d.opcode)
                            == OPC_RET
                            && child.get_succs().contains(&ret)
                        {
                            copy.add_successor(ret);
                            child.remove_successor(ret);
                        } else {
                            let child_succs = child.get_succs();
                            let mut k = 0;
                            while k < child_succs.len() {
                                copy.add_successor(child_succs.get(k));
                                k += 1;
                            }
                        }
                        let child_succ_exceptions = child.get_succ_exceptions();
                        let mut k = 0;
                        while k < child_succ_exceptions.len() {
                            copy.add_successor_exception(child_succ_exceptions.get(k));
                            k += 1;
                        }

                        lst_nodes.push_back(copy);
                        map_new_nodes.put(childid, copy);

                        if last.get_preds().contains(&child) {
                            last.add_predecessor(copy);
                        }

                        node.replace_successor(child, copy);
                        blocks.borrow_mut().add_with_key(copy, copy.get_id());
                    } else {
                        map_new_nodes.put(childid, child);
                    }

                    i -= 1;
                }
            }
        }

        self.split_jsr_exception_ranges(common_blocks, &map_new_nodes);
    }

    fn split_jsr_exception_ranges(
        self,
        common_blocks: &JHashSet<BasicBlock>,
        map_new_nodes: &JHashMap<i32, BasicBlock>,
    ) {
        let exceptions = self.get_exceptions();
        for i in (0..exceptions.len()).rev() {
            let range = exceptions.get(i);
            let lst_range = range.get_protected_range();

            let mut set_both = common_blocks.clone();
            set_both.retain(|b| lst_range.contains(b));

            if !set_both.is_empty() {
                let lst_new_range;

                if set_both.len() == lst_range.len() {
                    lst_new_range = JList::new();
                    let new_range = ExceptionRangeCFG::new(
                        lst_new_range.clone(),
                        map_new_nodes
                            .get_cloned(&range.get_handler().get_id())
                            .expect("NullPointerException"),
                        range.get_exception_types(),
                    );
                    exceptions.add(new_range);
                } else {
                    lst_new_range = lst_range;
                }

                for block in set_both.to_vec() {
                    lst_new_range.add(
                        map_new_nodes
                            .get_cloned(&block.get_id())
                            .expect("NullPointerException"),
                    );
                }
            }
        }
    }

    fn remove_jsr(self, cl: StructClass, mt: StructMethod) {
        let pool = cl.get_pool();
        let first = self.get_first();
        let data = DataPoint::get_initial_data_point(cl, mt);
        Self::remove_jsr_instructions(pool, first, data);
    }

    fn remove_jsr_instructions(pool: ConstantPool, block: BasicBlock, mut data: DataPoint) {
        let seq = block.get_seq();
        let mut i = 0;
        while i < seq.length() {
            let instr = seq.get_instr(i);
            let opcode = instr.with(|d| d.opcode);

            let mut var: Option<VarType> = None;
            if opcode == OPC_ASTORE || opcode == OPC_POP {
                var = Some(data.get_stack().get_by_offset(-1));
            }

            InstructionImpact::step_types(&mut data, instr, pool);

            match opcode {
                OPC_JSR | OPC_RET => {
                    seq.remove_instruction(i);
                    i -= 1;
                }
                OPC_ASTORE | OPC_POP => {
                    if var.expect("NullPointerException").type_ == CodeType::Address {
                        seq.remove_instruction(i);
                        i -= 1;
                    }
                }
                _ => {}
            }

            i += 1;
        }

        block.set_mark(1);

        let mut i = 0;
        while i < block.get_succs().len() {
            let suc = block.get_succs().get(i);
            if suc.get_mark() != 1 {
                Self::remove_jsr_instructions(pool, suc, data.copy());
            }
            i += 1;
        }

        let mut i = 0;
        while i < block.get_succ_exceptions().len() {
            let suc = block.get_succ_exceptions().get(i);
            if suc.get_mark() != 1 {
                let mut point = DataPoint::new();
                point.set_local_variables(data.get_local_variables().clone());
                point.get_stack().push(VarType::new_value(
                    CodeType::Object,
                    0,
                    Some("java/lang/Object"),
                ));

                Self::remove_jsr_instructions(pool, suc, point);
            }
            i += 1;
        }
    }

    pub fn get_reverse_post_order(self) -> Vec<BasicBlock> {
        let mut res: VecDeque<BasicBlock> = VecDeque::new();
        Self::add_to_reverse_post_order_list_iterative(self.get_first(), &mut res);

        res.into_iter().collect()
    }

    fn add_to_reverse_post_order_list_iterative(root: BasicBlock, lst: &mut VecDeque<BasicBlock>) {
        let mut stack_node: Vec<BasicBlock> = Vec::new();
        let mut stack_index: Vec<i32> = Vec::new();

        let mut set_visited: JHashSet<BasicBlock> = JHashSet::new();

        stack_node.push(root);
        stack_index.push(0);

        while let Some(&node) = stack_node.last() {
            let mut index = stack_index.pop().expect("NoSuchElementException");

            set_visited.add(node);

            let mut lst_succs = node.get_succs().to_vec();
            lst_succs.extend(node.get_succ_exceptions().to_vec());

            while (index as usize) < lst_succs.len() {
                let succ = lst_succs[index as usize];

                if !set_visited.contains(&succ) {
                    stack_index.push(index + 1);

                    stack_node.push(succ);
                    stack_index.push(0);

                    break;
                }
                index += 1;
            }

            if index as usize == lst_succs.len() {
                lst.push_front(node);

                stack_node.pop();
            }
        }
    }

    pub fn get_blocks(self) -> Shared<VBStyleCollection<BasicBlock, i32>> {
        self.with(|d| d.blocks.clone())
    }

    pub fn get_first(self) -> BasicBlock {
        self.with(|d| d.first.expect("NullPointerException"))
    }

    pub fn set_first(self, first: BasicBlock) {
        self.with_mut(|d| d.first = Some(first))
    }

    pub fn get_exceptions(self) -> JList<ExceptionRangeCFG> {
        self.with(|d| d.exceptions.clone())
    }

    pub fn get_last(self) -> BasicBlock {
        self.with(|d| d.last.expect("NullPointerException"))
    }

    pub fn get_finally_exits(self) -> Shared<JHashSet<BasicBlock>> {
        self.with(|d| d.finally_exits.clone())
    }

    pub fn get_sequence(self) -> FullInstructionSequence {
        self.with(|d| d.sequence)
    }

    pub fn add_comment(self, comment: &str) {
        self.with_mut(|d| {
            d.comment_lines
                .get_or_insert_with(JLinkedHashSet::new)
                .add(comment.to_string());
        })
    }
}

impl std::fmt::Display for ControlFlowGraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let graph = *self;
        let new_line_separator = DecompilerContext::get_new_line_separator().to_string();

        let mut buf = String::new();

        for block in graph.block_list() {
            let id = block.get_id();
            buf.push_str(&format!("----- Block {id} -----"));
            buf.push_str(&new_line_separator);
            buf.push_str(&block.to_string());
            buf.push_str("----- Edges -----");
            buf.push_str(&new_line_separator);

            for a_suc in block.get_succs().iter() {
                buf.push_str(&format!(">>>>>>>>(regular) Block {}", a_suc.get_id()));
                buf.push_str(&new_line_separator);
            }
            for handler in block.get_succ_exceptions().iter() {
                let range = graph.get_exception_range(handler, block);

                match range {
                    None => {
                        buf.push_str(&format!(
                            ">>>>>>>>(exception) Block {}\tERROR: range not found!",
                            handler.get_id()
                        ));
                        buf.push_str(&new_line_separator);
                    }
                    Some(range) => match range.get_exception_types() {
                        None => {
                            buf.push_str(&format!(
                                ">>>>>>>>(exception) Block {}\tNULL",
                                handler.get_id()
                            ));
                            buf.push_str(&new_line_separator);
                        }
                        Some(exception_types) => {
                            for exception_type in exception_types {
                                buf.push_str(&format!(
                                    ">>>>>>>>(exception) Block {}\t{}",
                                    handler.get_id(),
                                    exception_type
                                ));
                                buf.push_str(&new_line_separator);
                            }
                        }
                    },
                }
            }
            buf.push_str("----- ----- -----");
            buf.push_str(&new_line_separator);
        }

        f.write_str(&buf)
    }
}
