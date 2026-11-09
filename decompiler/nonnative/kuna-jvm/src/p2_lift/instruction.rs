//! Port of `org/jetbrains/java/decompiler/code/Instruction.java`.
//!
//! `JumpInstruction` and `SwitchInstruction` are views over [`InstructionKind`].

use crate::bytecode_version::BytecodeVersion;
use crate::code_constants::*;
use crate::full_instruction_sequence::FullInstructionSequence;
use crate::jump_instruction::{JumpInstruction, JumpInstructionData};
use crate::switch_instruction::{SwitchInstruction, SwitchInstructionData};
use crate::text_util::TextUtil;
use crate::{arena_identity, arena_object};

pub struct InstructionData {
    pub opcode: i32,
    pub group: i32,
    pub wide: bool,
    pub bytecode_version: BytecodeVersion,
    pub start_offset: i32,
    pub length: i32,
    pub operands: Option<Vec<i32>>,
    pub kind: InstructionKind,
}

pub enum InstructionKind {
    Plain,
    Jump(JumpInstructionData),
    Switch(SwitchInstructionData),
}

arena_object!(pub Instruction => InstructionData, Class);
arena_identity!(Instruction);

impl Instruction {
    #[allow(clippy::too_many_arguments)]
    pub fn create(
        opcode: i32,
        wide: bool,
        group: i32,
        bytecode_version: BytecodeVersion,
        operands: Option<Vec<i32>>,
        start_offset: i32,
        length: i32,
    ) -> Instruction {
        if (OPC_IFEQ..=OPC_IF_ACMPNE).contains(&opcode)
            || opcode == OPC_IFNULL
            || opcode == OPC_IFNONNULL
            || opcode == OPC_JSR
            || opcode == OPC_JSR_W
            || opcode == OPC_GOTO
            || opcode == OPC_GOTO_W
        {
            JumpInstruction::new(
                opcode,
                group,
                wide,
                bytecode_version,
                operands,
                start_offset,
                length,
            )
            .0
        } else if opcode == OPC_TABLESWITCH || opcode == OPC_LOOKUPSWITCH {
            SwitchInstruction::new(
                opcode,
                group,
                wide,
                bytecode_version,
                operands,
                start_offset,
                length,
            )
            .0
        } else {
            Instruction::new(
                opcode,
                group,
                wide,
                bytecode_version,
                operands,
                start_offset,
                length,
            )
        }
    }

    /// Java: static `equals(Instruction, Instruction)`.
    pub fn equals(i1: Option<Instruction>, i2: Option<Instruction>) -> bool {
        match (i1, i2) {
            (Some(i1), Some(i2)) => {
                i1 == i2
                    || i1.opcode() == i2.opcode()
                        && i1.wide() == i2.wide()
                        && i1.operands_count() == i2.operands_count()
            }
            _ => false,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new(
        opcode: i32,
        group: i32,
        wide: bool,
        bytecode_version: BytecodeVersion,
        operands: Option<Vec<i32>>,
        start_offset: i32,
        length: i32,
    ) -> Instruction {
        Instruction::new_kind(
            opcode,
            group,
            wide,
            bytecode_version,
            operands,
            start_offset,
            length,
            InstructionKind::Plain,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new_kind(
        opcode: i32,
        group: i32,
        wide: bool,
        bytecode_version: BytecodeVersion,
        operands: Option<Vec<i32>>,
        start_offset: i32,
        length: i32,
        kind: InstructionKind,
    ) -> Instruction {
        Instruction::alloc(InstructionData {
            opcode,
            group,
            wide,
            bytecode_version,
            start_offset,
            length,
            operands,
            kind,
        })
    }

    pub fn opcode(self) -> i32 {
        self.with(|d| d.opcode)
    }

    pub fn group(self) -> i32 {
        self.with(|d| d.group)
    }

    pub fn wide(self) -> bool {
        self.with(|d| d.wide)
    }

    pub fn bytecode_version(self) -> BytecodeVersion {
        self.with(|d| d.bytecode_version)
    }

    pub fn start_offset(self) -> i32 {
        self.with(|d| d.start_offset)
    }

    pub fn length(self) -> i32 {
        self.with(|d| d.length)
    }

    /// A copy of the raw operand array (`null` when the instruction has none).
    pub fn operands(self) -> Option<Vec<i32>> {
        self.with(|d| d.operands.clone())
    }

    pub fn init_instruction(self, seq: FullInstructionSequence) {
        if let Some(j) = self.try_as_jump() {
            JumpInstruction::init_instruction(j, seq)
        } else if let Some(s) = self.try_as_switch() {
            SwitchInstruction::init_instruction(s, seq)
        }
    }

    pub fn operands_count(self) -> i32 {
        self.with(|d| d.operands.as_ref().map_or(0, |o| o.len() as i32))
    }

    pub fn operand(self, index: i32) -> i32 {
        self.with(|d| d.operands.as_ref().expect("operands")[index as usize])
    }

    pub fn can_fall_through(self) -> bool {
        let opcode = self.opcode();
        opcode != OPC_GOTO
            && opcode != OPC_GOTO_W
            && opcode != OPC_RET
            && !(OPC_IRETURN..=OPC_RETURN).contains(&opcode)
            && opcode != OPC_ATHROW
            && opcode != OPC_JSR
            && opcode != OPC_TABLESWITCH
            && opcode != OPC_LOOKUPSWITCH
    }

    /// Java: `clone()`.
    pub fn java_clone(self) -> Instruction {
        if let Some(j) = self.try_as_jump() {
            JumpInstruction::java_clone(j).0
        } else if let Some(s) = self.try_as_switch() {
            SwitchInstruction::java_clone(s).0
        } else {
            Instruction::java_clone_base(self)
        }
    }

    /// `Instruction.clone()`'s own body, for `super.clone()`.
    pub fn java_clone_base(self) -> Instruction {
        let (opcode, wide, group, version, operands, start_offset, length) = self.with(|d| {
            (
                d.opcode,
                d.wide,
                d.group,
                d.bytecode_version,
                d.operands.clone(),
                d.start_offset,
                d.length,
            )
        });
        Instruction::create(opcode, wide, group, version, operands, start_offset, length)
    }
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (wide, opcode, operands) = self.with(|d| (d.wide, d.opcode, d.operands.clone()));
        let mut res = String::new();
        if wide {
            res.push_str("@wide ");
        }
        res.push('@');
        res.push_str(&TextUtil::get_instruction_name(opcode));
        for op in operands.unwrap_or_default() {
            if op < 0 {
                res.push_str(&format!(" -{:x}", op.wrapping_neg() as u32));
            } else {
                res.push_str(&format!(" {:x}", op as u32));
            }
        }
        f.write_str(&res)
    }
}
