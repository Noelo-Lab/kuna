//! Remove redundant ordered tests following the same Go floating-point equality.

use kuna_num::opcodes::OpCode;

use crate::architecture::Architecture;
use crate::context::{BlockId, VarnodeId};
use crate::dtype::type_metatype;
use crate::funcdata::Funcdata;
use crate::op::PcodeOp;

fn branch_condition(fd: &Funcdata, block: BlockId) -> Option<(BlockId, VarnodeId)> {
    let copy = fd.sblocks_ref().block(block);
    if copy.is_unstructured_target() {
        return None;
    }
    let basic = copy.get_copy()?;
    let branch = fd.obank().get(fd.bb_op_tail(basic)?)?;
    if branch.code() != OpCode::CPUI_CBRANCH || branch.is_boolean_flip() {
        return None;
    }
    Some((basic, branch.get_in(1)?))
}

fn implied_definition(fd: &Funcdata, value: VarnodeId) -> Option<&PcodeOp> {
    let value = fd.vbank().get(value)?;
    if !value.is_implied() || value.is_volatile() {
        return None;
    }
    fd.obank().get(value.get_def()?)
}

fn regular_float(fd: &Funcdata, arch: &Architecture, value: VarnodeId) -> bool {
    let Some(node) = fd.vbank().get(value) else {
        return false;
    };
    if node.is_implied()
        || node.is_volatile()
        || node.is_annotation()
        || !matches!(node.get_size(), 4 | 8)
    {
        return false;
    }
    if node.is_constant() {
        return true;
    }
    if node.get_type().get_metatype() != type_metatype::TYPE_FLOAT {
        return false;
    }
    let register = node
        .get_addr()
        .get_space()
        .is_some_and(|space| space.get_name() == "register");
    (node.is_input() && register)
        || (node.is_explicit() && !node.is_free() && !node.is_addr_tied() && !node.is_persist())
        || (node.is_input()
            && (node.is_read_only() || arch.is_global_read_only(node.get_addr(), node.get_size())))
}

fn same_operand(fd: &Funcdata, a: VarnodeId, b: VarnodeId) -> bool {
    if a == b {
        return true;
    }
    let (Some(a), Some(b)) = (fd.vbank().get(a), fd.vbank().get(b)) else {
        return false;
    };
    a.is_constant()
        && b.is_constant()
        && a.get_size() == b.get_size()
        && a.get_offset() == b.get_offset()
}

fn matching_guard(fd: &Funcdata, guard: VarnodeId, operands: [VarnodeId; 2]) -> Option<()> {
    let negate = implied_definition(fd, guard)?;
    if negate.code() != OpCode::CPUI_BOOL_NEGATE {
        return None;
    }
    let nan = implied_definition(fd, negate.get_in(0)?)?;
    if nan.code() != OpCode::CPUI_FLOAT_NAN {
        return None;
    }
    let tested = nan.get_in(0)?;
    operands
        .iter()
        .any(|&operand| same_operand(fd, operand, tested))
        .then_some(())
}

/// The right clause contains only tests already implied by the left equality.
pub(crate) fn redundant_right(
    fd: &Funcdata,
    arch: &Architecture,
    left: BlockId,
    right: BlockId,
) -> Option<()> {
    let (left_basic, equality) = branch_condition(fd, left)?;
    let compare = implied_definition(fd, equality)?;
    if compare.code() != OpCode::CPUI_FLOAT_EQUAL {
        return None;
    }
    let (right_basic, guard) = branch_condition(fd, right)?;
    let block = fd.bblocks_ref().block(right_basic);
    if block.size_in() != 1 || block.get_in(0) != left_basic {
        return None;
    }
    let operands = [compare.get_in(0)?, compare.get_in(1)?];
    if !operands
        .iter()
        .all(|&operand| regular_float(fd, arch, operand))
    {
        return None;
    }
    if fd.vbank().get(operands[0])?.get_size() != fd.vbank().get(operands[1])?.get_size() {
        return None;
    }
    let definition = implied_definition(fd, guard)?;
    if definition.code() == OpCode::CPUI_BOOL_AND {
        matching_guard(fd, definition.get_in(0)?, operands)?;
        matching_guard(fd, definition.get_in(1)?, operands)
    } else {
        matching_guard(fd, guard, operands)
    }
}
