//! (kuna `switchcaseorder`) The order a switch prints its arms in.
//!
//! `BlockSwitch::finalizePrinting` sorts the arms by label, and the `default:`
//! arm sorts under whatever label its table entry carries, so it prints first or
//! in the middle. An arm that ends in a call that does not return prints no
//! statement that stops it, so whichever arm is printed next reads as the code it
//! falls into: an edge the program does not have.
//!
//! [`order_cases`] keeps the label order and every fall-through chain together,
//! and moves the arm holding `default:` after the others, and an arm whose code
//! ends without a statement that stops it after those. Of several such arms
//! only the last can end the switch, so each of the others prints a `break;`.

use crate::block::{BlockType, CaseOrder};
use crate::context::BlockId;
use crate::funcdata::Funcdata;
use crate::op::pcodeop_flags;
use kuna_num::opcodes::OpCode;

/// Reorder `cases` (sorted by label, each chain root followed by the cases it
/// falls into) for printing.
pub(crate) fn order_cases(fd: &Funcdata, cases: &mut Vec<CaseOrder>) {
    let mut groups: Vec<Vec<CaseOrder>> = Vec::new();
    for c in cases.iter() {
        match groups.last_mut() {
            Some(g) if c.depth > 0 => {
                if g.last().map(|p| p.depth + 1) != Some(c.depth) || g[0].label != c.label {
                    return;
                }
                g.push(c.clone());
            }
            _ => groups.push(vec![c.clone()]),
        }
    }
    let unstopped: Vec<bool> = groups.iter().map(|g| g.last().is_some_and(|c| ends_unstopped(fd, c))).collect();
    let has_default: Vec<bool> = groups.iter().map(|g| g.iter().any(|c| c.isdefault)).collect();
    let order = print_order(&unstopped, &has_default);
    let n = order.len();
    let mut out: Vec<CaseOrder> = Vec::with_capacity(cases.len());
    for (pos, &i) in order.iter().enumerate() {
        let mut g = std::mem::take(&mut groups[i]);
        if unstopped[i] && pos + 1 < n {
            if let Some(last) = g.last_mut() {
                last.isexit = true;
            }
        }
        out.extend(g);
    }
    *cases = out;
}

/// The printing order of arm groups: the label order, with the group holding
/// `default:` after the other groups that end in a stopping statement, and the
/// groups that do not after all of those (`default:` last among them too).
fn print_order(unstopped: &[bool], has_default: &[bool]) -> Vec<usize> {
    let mut order: Vec<usize> = (0..unstopped.len()).collect();
    order.sort_by_key(|&i| (unstopped[i], has_default[i]));
    order
}

/// True when `case` prints code that ends without a statement stopping it: its
/// last basic block has no successor and does not return (a call that does not
/// return).
fn ends_unstopped(fd: &Funcdata, case: &CaseOrder) -> bool {
    case.gototype == 0 && !case.isexit && case.chain == -1 && last_leaf_unstopped(fd, case.block)
}

fn last_leaf_unstopped(fd: &Funcdata, blk: BlockId) -> bool {
    let graph = fd.sblocks_ref();
    let b = graph.block(blk);
    match b.get_type() {
        BlockType::Ls => b.get_list().last().is_some_and(|&l| last_leaf_unstopped(fd, l)),
        BlockType::Copy => b.get_copy().is_some_and(|bb| {
            let halt = pcodeop_flags::halt | pcodeop_flags::noreturn;
            fd.bblocks_ref().block(bb).size_out() == 0
                && fd
                    .bb_ops(bb)
                    .into_iter()
                    .rev()
                    .filter_map(|op| fd.obank().get(op))
                    .find(|op| !op.not_printed() && !(op.code() == OpCode::CPUI_RETURN && op.get_halt_type() & halt != 0))
                    .is_some_and(|op| matches!(op.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND | OpCode::CPUI_CALLOTHER))
        }),
        _ => false,
    }
}

#[cfg(test)]
#[path = "kuna_switchcaseorder/tests.rs"]
mod tests;
