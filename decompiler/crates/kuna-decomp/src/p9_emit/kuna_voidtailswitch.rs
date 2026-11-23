//! (kuna `voidtailreturn`) The `default:` arm of the switch a void function ends
//! with, when all that arm does is return.
//!
//! A switch at the end of a void function whose source had no `default:` (or a
//! `default: break;`) leaves the switch for every other value and falls off the
//! end of the body. Once the shared epilogue has been duplicated into the arms,
//! kuna prints that path as `default: return;`, a statement the source does not
//! have. When the switch is the last statement the function prints, leaving it
//! is returning, so the arm says nothing and is left out, exactly as
//! `voidtailreturn` leaves out the function's own trailing `return;`.

use crate::block::BlockType;
use crate::context::{BlockId, OpId};
use crate::dtype::type_metatype;
use crate::funcdata::Funcdata;
use kuna_num::opcodes::OpCode;

/// The switch block and the index of its `default:` case that prints only a
/// bare `return;`, when the switch is the last statement of a void function.
/// `tail_return` is the trailing `return;` the printer already elides: a leaf
/// printing only that op after the switch prints nothing.
pub(crate) fn elidable_tail_default(fd: &Funcdata, tail_return: Option<OpId>) -> Option<(BlockId, usize)> {
    let out = fd.get_func_proto().get_output_type()?;
    if out.get_metatype() != type_metatype::TYPE_VOID {
        return None;
    }
    let graph = fd.sblocks_ref();
    let mut cur = graph.root?;
    let sw = loop {
        let blk = graph.block(cur);
        match blk.get_type() {
            BlockType::Graph | BlockType::Ls => {
                let list = blk.get_list();
                let mut last = *list.last()?;
                if list.len() > 1 && tail_return.is_some() && printed_ops(fd, last) == Some(tail_return.into_iter().collect()) {
                    last = list[list.len() - 2];
                }
                cur = last;
            }
            BlockType::Switch => break cur,
            _ => return None,
        }
    };
    let cases = graph.block(sw).switch_caseblocks();
    let index = cases.iter().position(|c| c.isdefault)?;
    let case = &cases[index];
    if case.gototype != 0 || case.chain != -1 || case.depth != 0 || cases.iter().any(|c| c.chain == index as i32) {
        return None;
    }
    let leaf = graph.block(case.block);
    if !matches!(leaf.get_type(), BlockType::Basic | BlockType::Copy) || leaf.is_unstructured_target() {
        return None;
    }
    let ops = printed_ops(fd, case.block)?;
    let ret = fd.obank().get(*ops.first()?)?;
    if ops.len() != 1
        || ret.code() != OpCode::CPUI_RETURN
        || ret.num_input() > 1
        || crate::kuna_decodehalt::halt_call_name(ret.get_halt_type()).is_some()
    {
        return None;
    }
    Some((sw, index))
}

/// The ops a structured leaf prints, other than branches; `None` for a block
/// that is not a leaf.
fn printed_ops(fd: &Funcdata, leaf: BlockId) -> Option<Vec<OpId>> {
    let graph = fd.sblocks_ref();
    let blk = graph.block(leaf);
    if !matches!(blk.get_type(), BlockType::Basic | BlockType::Copy) {
        return None;
    }
    let mut cur = match blk.get_copy() {
        Some(under) => fd.bb_op_head(under),
        None => match blk.kind() {
            crate::block::BlockKind::Basic(b) => b.op_head,
            _ => None,
        },
    };
    let mut ops = Vec::new();
    while let Some(op) = cur {
        cur = fd.bb_op_next(op);
        let Some(o) = fd.obank().get(op) else { continue };
        if o.not_printed() || o.code() == OpCode::CPUI_BRANCH {
            continue;
        }
        if o.get_out().and_then(|v| fd.vbank().get(v)).is_some_and(|v| v.is_implied()) {
            continue;
        }
        ops.push(op);
    }
    Some(ops)
}
