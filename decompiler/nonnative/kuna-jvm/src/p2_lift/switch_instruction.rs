//! Port of `org/jetbrains/java/decompiler/code/SwitchInstruction.java`.

use crate::bytecode_version::BytecodeVersion;
use crate::code_constants::{OPC_LOOKUPSWITCH, OPC_TABLESWITCH};
use crate::full_instruction_sequence::FullInstructionSequence;
use crate::instruction::{Instruction, InstructionKind};
use crate::java_subclass;

#[derive(Default)]
pub struct SwitchInstructionData {
    pub destinations: Vec<i32>,
    pub values: Vec<i32>,
    pub default_destination: i32,
}

java_subclass!(pub SwitchInstruction: Instruction, InstructionKind::Switch(SwitchInstructionData), is_switch, as_switch, try_as_switch);

impl crate::rt::JavaEq for SwitchInstruction {
    fn java_equals(&self, other: &Self) -> bool {
        self == other
    }
}

impl crate::rt::JavaHash for SwitchInstruction {
    fn java_hash(&self) -> i32 {
        self.index() as i32
    }
}

impl SwitchInstruction {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        opcode: i32,
        group: i32,
        wide: bool,
        bytecode_version: BytecodeVersion,
        operands: Option<Vec<i32>>,
        start_offset: i32,
        length: i32,
    ) -> SwitchInstruction {
        SwitchInstruction(Instruction::new_kind(
            opcode,
            group,
            wide,
            bytecode_version,
            operands,
            start_offset,
            length,
            InstructionKind::Switch(SwitchInstructionData::default()),
        ))
    }

    pub fn init_instruction(self, seq: FullInstructionSequence) {
        let opcode = self.opcode();
        let operands = self.operands().expect("operands");
        let default_destination = seq.get_index_by_rel_offset(self.0, operands[0]);

        let prefix: usize = if opcode == OPC_TABLESWITCH { 3 } else { 2 };
        let mut len = operands.len() as i32 - prefix as i32;
        let mut low = 0;
        if opcode == OPC_LOOKUPSWITCH {
            len /= 2;
        } else {
            low = operands[1];
        }

        let mut destinations = vec![0; len as usize];
        let mut values = vec![0; len as usize];
        let mut k = 0usize;
        for i in 0..len as usize {
            if opcode == OPC_LOOKUPSWITCH {
                values[i] = operands[prefix + k];
                k += 1;
            } else {
                values[i] = low.wrapping_add(k as i32);
            }
            destinations[i] = seq.get_index_by_rel_offset(self.0, operands[prefix + k]);
            k += 1;
        }

        self.d_mut(|d| {
            d.default_destination = default_destination;
            d.destinations = destinations;
            d.values = values;
        });
    }

    pub fn get_destinations(self) -> Vec<i32> {
        self.d(|d| d.destinations.clone())
    }

    pub fn get_values(self) -> Vec<i32> {
        self.d(|d| d.values.clone())
    }

    pub fn get_default_destination(self) -> i32 {
        self.d(|d| d.default_destination)
    }

    /// Java: `clone()`.
    pub fn java_clone(self) -> SwitchInstruction {
        let copy = Instruction::java_clone_base(self.0).as_switch();
        let (default_destination, destinations, values) = self.d(|d| {
            (
                d.default_destination,
                d.destinations.clone(),
                d.values.clone(),
            )
        });
        copy.d_mut(|d| {
            d.default_destination = default_destination;
            d.destinations = destinations;
            d.values = values;
        });
        copy
    }
}
