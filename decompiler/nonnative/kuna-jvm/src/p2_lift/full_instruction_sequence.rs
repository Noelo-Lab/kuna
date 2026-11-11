//! Port of `org/jetbrains/java/decompiler/code/FullInstructionSequence.java`.
//!
//! A Java record; component accessors keep the component names. Its handle
//! compares by identity (no caller relies on the record's `equals`).

use crate::decompiler_context::DecompilerContext;
use crate::exception_table::ExceptionTable;
use crate::instruction::Instruction;
use crate::rt::JHashMap;
use crate::text_util::TextUtil;
use crate::{arena_identity, arena_object};

pub struct FullInstructionSequenceData {
    pub instructions: Vec<Instruction>,
    pub offset_to_index: JHashMap<i32, i32>,
    pub exception_table: ExceptionTable,
}

arena_object!(pub FullInstructionSequence => FullInstructionSequenceData, Class);
arena_identity!(FullInstructionSequence);

impl FullInstructionSequence {
    pub fn new(
        instructions: Vec<Instruction>,
        offset_to_index: JHashMap<i32, i32>,
        exception_table: ExceptionTable,
    ) -> FullInstructionSequence {
        FullInstructionSequence::alloc(FullInstructionSequenceData {
            instructions,
            offset_to_index,
            exception_table,
        })
    }

    pub fn instructions(self) -> Vec<Instruction> {
        self.with(|d| d.instructions.clone())
    }

    pub fn offset_to_index(self) -> JHashMap<i32, i32> {
        self.with(|d| d.offset_to_index.clone())
    }

    pub fn exception_table(self) -> ExceptionTable {
        self.with(|d| d.exception_table.clone())
    }

    pub fn get_instr(self, index: i32) -> Instruction {
        self.with(|d| d.instructions[index as usize])
    }

    pub fn length(self) -> i32 {
        self.with(|d| d.instructions.len() as i32)
    }

    pub fn is_empty(self) -> bool {
        self.with(|d| d.instructions.is_empty())
    }

    pub fn get_index_by_rel_offset(self, instruction: Instruction, offset: i32) -> i32 {
        let absolute_offset = instruction.start_offset().wrapping_add(offset);
        self.with(|d| {
            d.offset_to_index
                .get(&absolute_offset)
                .copied()
                .unwrap_or(-1)
        })
    }

    /// Java: `iterator()`, over a snapshot.
    pub fn iter(self) -> std::vec::IntoIter<Instruction> {
        self.instructions().into_iter()
    }

    /// Java: `toString(int)`.
    pub fn to_string_indent(self, indent: i32) -> String {
        let new_line_separator = DecompilerContext::get_new_line_separator();
        let mut buf = String::new();
        for instr in self.instructions() {
            buf.push_str(&TextUtil::get_indent_string(indent));
            buf.push_str(&instr.start_offset().to_string());
            buf.push_str(": ");
            buf.push_str(&format!("{}{}", instr, new_line_separator));
        }
        buf
    }

    pub fn get_last(self) -> Instruction {
        self.with(|d| d.instructions[d.instructions.len().wrapping_sub(1)])
    }
}

impl std::fmt::Display for FullInstructionSequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string_indent(0))
    }
}
