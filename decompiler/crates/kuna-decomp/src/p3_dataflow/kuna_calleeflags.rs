//! Instruction-local scratch leading to Boolean flags can retain its frame origin.

use kuna_base::space::spacetype;
use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;

use super::kuna_calleememory::RawOp;

fn overlaps(left: &VarnodeData, right: &VarnodeData) -> bool {
    left.space.as_ref().map(|s| s.get_index()) == right.space.as_ref().map(|s| s.get_index())
        && left.offset < right.offset.saturating_add(u64::from(right.size))
        && right.offset < left.offset.saturating_add(u64::from(left.size))
}

fn boolean(code: OpCode) -> bool {
    matches!(
        code,
        OpCode::CPUI_INT_EQUAL
            | OpCode::CPUI_INT_NOTEQUAL
            | OpCode::CPUI_INT_LESS
            | OpCode::CPUI_INT_SLESS
            | OpCode::CPUI_INT_LESSEQUAL
            | OpCode::CPUI_INT_SLESSEQUAL
            | OpCode::CPUI_INT_CARRY
            | OpCode::CPUI_INT_SCARRY
            | OpCode::CPUI_INT_SBORROW
    )
}

fn scratch(code: OpCode) -> bool {
    matches!(
        code,
        OpCode::CPUI_COPY
            | OpCode::CPUI_CAST
            | OpCode::CPUI_SUBPIECE
            | OpCode::CPUI_INT_AND
            | OpCode::CPUI_INT_OR
            | OpCode::CPUI_INT_XOR
            | OpCode::CPUI_INT_LEFT
            | OpCode::CPUI_INT_RIGHT
            | OpCode::CPUI_INT_SRIGHT
            | OpCode::CPUI_INT_NEGATE
            | OpCode::CPUI_INT_2COMP
            | OpCode::CPUI_INT_ZEXT
            | OpCode::CPUI_INT_SEXT
            | OpCode::CPUI_POPCOUNT
    )
}

pub(super) fn flag_only_results(ops: &[RawOp]) -> Vec<bool> {
    let mut safe = vec![false; ops.len()];
    for (index, op) in ops.iter().enumerate().rev() {
        let Some(out) = op.out.as_ref() else { continue };
        if out.size == 1 && boolean(op.code) {
            safe[index] = true;
            continue;
        }
        if !scratch(op.code)
            || !out
                .space
                .as_ref()
                .is_some_and(|s| s.get_type() == spacetype::IPTR_INTERNAL)
        {
            continue;
        }
        let mut consumed = false;
        let mut flag_only = true;
        for (next, consumer) in ops.iter().enumerate().skip(index + 1) {
            for input in &consumer.inputs {
                if overlaps(out, input) {
                    consumed = true;
                    flag_only &= safe[next] && out.offset == input.offset && out.size == input.size;
                }
            }
            if let Some(replaced) = consumer.out.as_ref().filter(|v| overlaps(out, v)) {
                flag_only &= replaced.offset == out.offset && replaced.size == out.size;
                break;
            }
        }
        safe[index] = consumed && flag_only;
    }
    safe
}
