//! (kuna `wrapadd`) Preserve word-width wrapping before a sign extension
//! observes the result.
use crate::architecture::Architecture;
use crate::context::{OpId, VarnodeId};
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;
use kuna_num::opcodes::OpCode;
use std::rc::Rc;

pub(crate) struct WidenedAddPlan {
    pub lhs: VarnodeId,
    pub rhs: VarnodeId,
    pub unsigned: Rc<Datatype>,
    pub signed: Option<Rc<Datatype>>,
}

pub(crate) fn plan(
    data: &Funcdata,
    arch: &Architecture,
    id: OpId,
    printed: impl Fn(VarnodeId) -> Rc<Datatype>,
) -> Option<WidenedAddPlan> {
    if !arch.wrap_add {
        return None;
    }
    let op = data.obank().get(id)?;
    if op.code() != OpCode::CPUI_INT_ADD {
        return None;
    }
    let output = data.vbank().get(op.get_out()?)?;
    let lhs = op.get_in(0)?;
    let rhs = op.get_in(1)?;
    // A U-suffixed literal already makes the original word expression wrap.
    if [lhs, rhs]
        .iter()
        .any(|&vn| data.vbank().get(vn).is_some_and(|v| v.is_unsigned_print()))
    {
        return None;
    }
    let signed = |vn: VarnodeId| {
        let Some(value) = data.vbank().get(vn) else {
            return false;
        };
        let ty = printed(vn);
        value.get_size() == 4
            && matches!(
                ty.get_metatype(),
                type_metatype::TYPE_INT | type_metatype::TYPE_ENUM_INT
            )
    };
    let operand = |v| {
        signed(v)
            || data
                .vbank()
                .get(v)
                .is_some_and(|v| v.is_constant() && v.get_size() == 4)
    };
    if output.get_size() != 4 || !operand(lhs) || !operand(rhs) || !(signed(lhs) || signed(rhs)) {
        return None;
    }
    let mut work: Vec<_> = output.descend_iter().collect();
    let mut seen = std::collections::HashSet::new();
    let mut extended = false;
    while let Some(reader) = work.pop() {
        if !seen.insert(reader) {
            continue;
        }
        if seen.len() > 16 {
            break;
        }
        let reader = data.obank().get(reader)?;
        let Some(out) = reader.get_out().and_then(|v| data.vbank().get(v)) else {
            continue;
        };
        if reader.code() == OpCode::CPUI_INT_SEXT && out.get_size() > 4 {
            extended = integer_observes(data, reader.get_out().unwrap());
            if extended {
                break;
            }
        }
        if out.get_size() == 4 && matches!(reader.code(), OpCode::CPUI_COPY | OpCode::CPUI_CAST) {
            work.extend(out.descend_iter());
        }
    }
    if !extended {
        return None;
    }
    Some(WidenedAddPlan {
        lhs,
        rhs,
        unsigned: arch.types().get_base(4, type_metatype::TYPE_UINT).ok()?,
        signed: matches!(
            output.get_type_def_facing().get_metatype(),
            type_metatype::TYPE_INT | type_metatype::TYPE_ENUM_INT
        )
        .then(|| arch.types().get_base(4, type_metatype::TYPE_INT).ok())
        .flatten(),
    })
}

/// Address generation and library lengths have separate pointer/domain rules.
/// Repair a widened integer value observed by a return, store or comparison;
/// following pointer arithmetic into a LOAD would conflate its loaded value
/// with the address bits. The bounded walk is only an emission guard.
fn integer_observes(data: &Funcdata, start: VarnodeId) -> bool {
    let mut work = vec![start];
    let mut seen = std::collections::HashSet::new();
    while let Some(vn) = work.pop() {
        if !seen.insert(vn) {
            continue;
        }
        if seen.len() > 64 {
            return false;
        }
        let Some(value) = data.vbank().get(vn) else {
            continue;
        };
        if value.get_size() < 8
            || matches!(
                value.get_type().get_metatype(),
                type_metatype::TYPE_PTR | type_metatype::TYPE_PTRREL | type_metatype::TYPE_FLOAT
            )
        {
            continue;
        }
        for id in value.descend_iter() {
            let Some(op) = data.obank().get(id) else {
                continue;
            };
            match op.code() {
                OpCode::CPUI_RETURN if (1..op.num_input()).any(|i| op.get_in(i) == Some(vn)) => {
                    return true
                }
                OpCode::CPUI_STORE if op.get_in(2) == Some(vn) => return true,
                OpCode::CPUI_INT_EQUAL
                | OpCode::CPUI_INT_NOTEQUAL
                | OpCode::CPUI_INT_LESS
                | OpCode::CPUI_INT_LESSEQUAL
                | OpCode::CPUI_INT_SLESS
                | OpCode::CPUI_INT_SLESSEQUAL => return true,
                OpCode::CPUI_COPY
                | OpCode::CPUI_CAST
                | OpCode::CPUI_MULTIEQUAL
                | OpCode::CPUI_INT_ADD
                | OpCode::CPUI_INT_SUB
                | OpCode::CPUI_INT_MULT
                | OpCode::CPUI_INT_LEFT
                | OpCode::CPUI_INT_RIGHT
                | OpCode::CPUI_INT_SRIGHT
                | OpCode::CPUI_INT_AND
                | OpCode::CPUI_INT_OR
                | OpCode::CPUI_INT_XOR => {
                    if let Some(out) = op.get_out() {
                        work.push(out);
                    }
                }
                _ => {}
            }
        }
    }
    false
}

#[cfg(test)]
mod tests;
