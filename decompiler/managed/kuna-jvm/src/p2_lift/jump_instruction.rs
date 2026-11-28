//! Port of `org/jetbrains/java/decompiler/code/JumpInstruction.java`.

use crate::bytecode_version::BytecodeVersion;
use crate::full_instruction_sequence::FullInstructionSequence;
use crate::instruction::{Instruction, InstructionKind};
use crate::java_subclass;

pub struct JumpInstructionData {
    pub destination: i32,
}

java_subclass!(pub JumpInstruction: Instruction, InstructionKind::Jump(JumpInstructionData), is_jump, as_jump, try_as_jump);

impl crate::rt::JavaEq for JumpInstruction {
    fn java_equals(&self, other: &Self) -> bool {
        self == other
    }
}

impl crate::rt::JavaHash for JumpInstruction {
    fn java_hash(&self) -> i32 {
        self.index() as i32
    }
}

impl JumpInstruction {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        opcode: i32,
        group: i32,
        wide: bool,
        bytecode_version: BytecodeVersion,
        operands: Option<Vec<i32>>,
        start_offset: i32,
        length: i32,
    ) -> JumpInstruction {
        JumpInstruction(Instruction::new_kind(
            opcode,
            group,
            wide,
            bytecode_version,
            operands,
            start_offset,
            length,
            InstructionKind::Jump(JumpInstructionData { destination: 0 }),
        ))
    }

    pub fn destination(self) -> i32 {
        self.d(|d| d.destination)
    }

    pub fn set_destination(self, destination: i32) {
        self.d_mut(|d| d.destination = destination)
    }

    pub fn init_instruction(self, seq: FullInstructionSequence) {
        let destination = seq.get_index_by_rel_offset(self.0, self.operand(0));
        self.set_destination(destination);
    }

    /// Java: `clone()`.
    pub fn java_clone(self) -> JumpInstruction {
        let copy = Instruction::java_clone_base(self.0).as_jump();
        copy.set_destination(self.destination());
        copy
    }
}
