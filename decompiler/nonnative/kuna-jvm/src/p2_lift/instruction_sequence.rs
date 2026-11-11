//! Port of `org/jetbrains/java/decompiler/code/InstructionSequence.java`.

use crate::decompiler_context::DecompilerContext;
use crate::instruction::Instruction;
use crate::text_util::TextUtil;
use crate::{arena_identity, arena_object};

pub struct InstructionSequenceData {
    pub instructions: Vec<Instruction>,
}

arena_object!(pub InstructionSequence => InstructionSequenceData, Class);
arena_identity!(InstructionSequence);

impl InstructionSequence {
    /// Java: `InstructionSequence()`.
    pub fn new() -> InstructionSequence {
        InstructionSequence::new_from(Vec::new())
    }

    /// Java: `InstructionSequence(Collection<Instruction>)` (takes a copy).
    pub fn new_from(instructions: Vec<Instruction>) -> InstructionSequence {
        InstructionSequence::alloc(InstructionSequenceData { instructions })
    }

    /// Java: `clone()`.
    pub fn java_clone(self) -> InstructionSequence {
        InstructionSequence::new_from(self.instructions())
    }

    pub fn clear(self) {
        self.with_mut(|d| d.instructions.clear())
    }

    pub fn add_instruction(self, inst: Instruction) {
        self.with_mut(|d| d.instructions.push(inst))
    }

    /// Java: `addInstruction(int, Instruction)`.
    pub fn add_instruction_at(self, index: i32, inst: Instruction) {
        self.with_mut(|d| d.instructions.insert(index as usize, inst))
    }

    pub fn add_sequence(self, seq: InstructionSequence) {
        let other = seq.instructions();
        self.with_mut(|d| d.instructions.extend(other))
    }

    pub fn remove_instruction(self, index: i32) {
        self.with_mut(|d| {
            d.instructions.remove(index as usize);
        })
    }

    pub fn remove_last(self) {
        self.with_mut(|d| {
            d.instructions.pop();
        })
    }

    pub fn get_instr(self, index: i32) -> Instruction {
        self.with(|d| d.instructions[index as usize])
    }

    pub fn get_last_instr(self) -> Instruction {
        self.with(|d| d.instructions[d.instructions.len().wrapping_sub(1)])
    }

    pub fn length(self) -> i32 {
        self.with(|d| d.instructions.len() as i32)
    }

    pub fn is_empty(self) -> bool {
        self.with(|d| d.instructions.is_empty())
    }

    /// A snapshot of the instruction list.
    pub fn instructions(self) -> Vec<Instruction> {
        self.with(|d| d.instructions.clone())
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
}

impl Default for InstructionSequence {
    fn default() -> Self {
        InstructionSequence::new()
    }
}

impl std::fmt::Display for InstructionSequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string_indent(0))
    }
}
