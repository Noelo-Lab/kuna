//! Port of `org/jetbrains/java/decompiler/code/cfg/BasicBlock.java`.

use crate::decompiler_context::DecompilerContext;
use crate::i_graph_node::IGraphNode;
use crate::instruction::Instruction;
use crate::instruction_sequence::InstructionSequence;
use crate::rt::JList;
use crate::{arena_identity, arena_object};

pub struct BasicBlockData {
    pub id: i32,
    pub mark: i32,
    pub seq: InstructionSequence,
    pub preds: JList<BasicBlock>,
    pub succs: JList<BasicBlock>,
    pub instr_old_offsets: JList<i32>,
    pub pred_exceptions: JList<BasicBlock>,
    pub succ_exceptions: JList<BasicBlock>,
}

arena_object!(pub BasicBlock => BasicBlockData, Class);
arena_identity!(BasicBlock);

impl BasicBlock {
    pub fn new(id: i32) -> BasicBlock {
        let seq = InstructionSequence::new();
        BasicBlock::alloc(BasicBlockData {
            id,
            mark: 0,
            seq,
            preds: JList::new(),
            succs: JList::new(),
            instr_old_offsets: JList::new(),
            pred_exceptions: JList::new(),
            succ_exceptions: JList::new(),
        })
    }

    pub fn get_instruction(self, index: i32) -> Instruction {
        self.get_seq().get_instr(index)
    }

    pub fn get_last_instruction(self) -> Option<Instruction> {
        let seq = self.get_seq();
        if seq.is_empty() {
            None
        } else {
            Some(seq.get_last_instr())
        }
    }

    pub fn get_old_offset(self, index: i32) -> i32 {
        let offsets = self.get_instr_old_offsets();
        if index < offsets.len() as i32 {
            offsets.get(index as usize)
        } else {
            -1
        }
    }

    pub fn size(self) -> i32 {
        self.get_seq().length()
    }

    pub fn add_predecessor(self, block: BasicBlock) {
        self.get_preds().add(block);
    }

    pub fn remove_predecessor(self, block: BasicBlock) {
        let preds = self.get_preds();
        while preds.remove_item(&block) {}
    }

    pub fn add_successor(self, block: BasicBlock) {
        self.get_succs().add(block);
        block.add_predecessor(self);
    }

    pub fn remove_successor(self, block: BasicBlock) {
        let succs = self.get_succs();
        while succs.remove_item(&block) {}
        block.remove_predecessor(self);
    }

    pub fn replace_successor(self, old_block: BasicBlock, new_block: BasicBlock) {
        if old_block == new_block {
            return;
        }

        let old_id = old_block.get_id();

        let succs = self.get_succs();
        for i in 0..succs.len() {
            if succs.get(i).get_id() == old_id {
                succs.set(i, new_block);
                old_block.remove_predecessor(self);
                new_block.add_predecessor(self);
            }
        }

        let succ_exceptions = self.get_succ_exceptions();
        for i in 0..succ_exceptions.len() {
            if succ_exceptions.get(i).get_id() == old_id {
                succ_exceptions.set(i, new_block);
                old_block.remove_predecessor_exception(self);
                new_block.add_predecessor_exception(self);
            }
        }
    }

    pub fn add_predecessor_exception(self, block: BasicBlock) {
        self.get_pred_exceptions().add(block);
    }

    pub fn remove_predecessor_exception(self, block: BasicBlock) {
        let pred_exceptions = self.get_pred_exceptions();
        while pred_exceptions.remove_item(&block) {}
    }

    pub fn add_successor_exception(self, block: BasicBlock) {
        let succ_exceptions = self.get_succ_exceptions();
        if !succ_exceptions.contains(&block) {
            succ_exceptions.add(block);
            block.add_predecessor_exception(self);
        }
    }

    pub fn remove_successor_exception(self, block: BasicBlock) {
        let succ_exceptions = self.get_succ_exceptions();
        while succ_exceptions.remove_item(&block) {}
        block.remove_predecessor_exception(self);
    }

    /// Java: `toString(int)`.
    pub fn to_string_indent(self, indent: i32) -> String {
        let new_line_separator = DecompilerContext::get_new_line_separator().to_string();

        format!(
            "{}:{}{}",
            self.get_id(),
            new_line_separator,
            self.get_seq().to_string_indent(indent)
        )
    }

    pub fn is_successor(self, block: BasicBlock) -> bool {
        let id = block.get_id();
        self.get_succs().iter().any(|succ| succ.get_id() == id)
    }

    pub fn clone_block(self, id: i32) -> BasicBlock {
        let block = BasicBlock::new(id);

        block.set_seq(self.get_seq().java_clone());
        block
            .get_instr_old_offsets()
            .add_all(self.get_instr_old_offsets().to_vec());

        block
    }

    pub fn get_instr_old_offsets(self) -> JList<i32> {
        self.with(|d| d.instr_old_offsets.clone())
    }

    pub fn get_id(self) -> i32 {
        self.with(|d| d.id)
    }

    pub fn get_mark(self) -> i32 {
        self.with(|d| d.mark)
    }

    pub fn set_mark(self, mark: i32) {
        self.with_mut(|d| d.mark = mark)
    }

    pub fn get_preds(self) -> JList<BasicBlock> {
        self.with(|d| d.preds.clone())
    }

    pub fn get_seq(self) -> InstructionSequence {
        self.with(|d| d.seq)
    }

    pub fn set_seq(self, seq: InstructionSequence) {
        self.with_mut(|d| d.seq = seq)
    }

    pub fn get_succs(self) -> JList<BasicBlock> {
        self.with(|d| d.succs.clone())
    }

    pub fn get_succ_exceptions(self) -> JList<BasicBlock> {
        self.with(|d| d.succ_exceptions.clone())
    }

    pub fn get_pred_exceptions(self) -> JList<BasicBlock> {
        self.with(|d| d.pred_exceptions.clone())
    }

    pub fn get_start_instruction(self) -> i32 {
        if self.get_seq().is_empty() {
            return 0;
        }
        self.get_instr_old_offsets().get(0)
    }

    pub fn get_end_instruction(self) -> i32 {
        let seq = self.get_seq();
        if seq.is_empty() {
            return 0;
        }
        let end = seq.get_last_instr().with(|d| d.length);
        end + self.get_instr_old_offsets().get((self.size() - 1) as usize)
    }
}

impl IGraphNode for BasicBlock {
    fn get_predecessors(self) -> Vec<BasicBlock> {
        let mut lst = self.get_preds().to_vec();
        lst.extend(self.get_pred_exceptions().to_vec());
        lst
    }
}

impl std::fmt::Display for BasicBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string_indent(0))
    }
}
