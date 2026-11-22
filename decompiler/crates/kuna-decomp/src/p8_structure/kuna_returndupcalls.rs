//! (kuna `returndup calls`) Split a cross-jumped call tail per predecessor.
//!
//! An optimizing compiler shares the tail of two paths that end in the same
//! call (gcc `-fcrossjumping`): each path sets the arguments and jumps to one
//! `call f; ret` block, so the decompiler sees MULTIEQUALs feeding the call's
//! arguments and prints `if (c) { x = a; } else { x = b; } return f(x);`.
//! [`ActionReturnDupCalls`] splits such a block once per predecessor with
//! [`Funcdata::node_split_calls`], giving each path its own call and return:
//! `if (c) return f(a); return f(b);`.
//!
//! A block qualifies when it has no successors, two to four distinct
//! predecessors, at most [`MAX_OPS`] ops, exactly one direct CALL, an argument
//! of that call computed from a MULTIEQUAL in the block, and no INDIRECT that
//! still creates the call's output (prototype recovery has not settled it yet;
//! a later pass of the loop retries).  Each predecessor runs exactly one copy,
//! so the split preserves behaviour.

use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::action::{Action, ActionBase, ActionContext, ActionGroupList, ApplyResult};
use crate::context::{BlockId, OpId};
use crate::funcdata::Funcdata;

/// Largest block (in p-code ops) this pass duplicates.
pub const MAX_OPS: usize = 16;
/// Maximum splits per function.
pub const MAX_SPLITS: int4 = 16;

/// Split a cross-jumped call tail per predecessor (`option returndup calls`).
pub struct ActionReturnDupCalls {
    base: ActionBase,
}

impl ActionReturnDupCalls {
    /// Construct in group `g`.
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionReturnDupCalls { base: ActionBase::new(0, "returndupcalls", g) })
    }
}

fn block_ops(data: &Funcdata, b: BlockId) -> Vec<OpId> {
    let mut ops = Vec::new();
    let mut cur = data.bb_op_head(b);
    while let Some(op) = cur {
        ops.push(op);
        cur = data.obank().get(op).and_then(|o| o.basic_neighbours().1);
    }
    ops
}

/// Does `op`'s value depend, within `ops`, on a MULTIEQUAL?
fn reads_merge(data: &Funcdata, ops: &[OpId], op: OpId, depth: u32) -> bool {
    let Some(o) = data.obank().get(op) else { return false };
    (0..o.num_input()).any(|slot| {
        let Some(def) = o.get_in(slot).and_then(|v| data.vbank().get(v)).and_then(|v| v.get_def()) else { return false };
        if !ops.contains(&def) {
            return false;
        }
        let code = data.obank().get(def).map(|d| d.code());
        code == Some(OpCode::CPUI_MULTIEQUAL) || (depth < 4 && reads_merge(data, ops, def, depth + 1))
    })
}

fn is_candidate(data: &Funcdata, b: BlockId) -> bool {
    let blk = data.bblocks_ref().block(b);
    let size_in = blk.size_in();
    if blk.size_out() != 0 || !(2..=4).contains(&size_in) {
        return false;
    }
    let mut preds: Vec<BlockId> = (0..size_in).map(|i| blk.get_in(i)).collect();
    preds.sort_unstable();
    preds.dedup();
    if preds.len() as int4 != size_in {
        return false;
    }
    let ops = block_ops(data, b);
    if ops.len() > MAX_OPS {
        return false;
    }
    let mut call = None;
    for &op in &ops {
        let Some(o) = data.obank().get(op) else { return false };
        if (o.is_branch() && o.code() != OpCode::CPUI_RETURN) || o.is_indirect_creation() {
            return false;
        }
        if o.is_call() {
            if call.is_some() || o.code() != OpCode::CPUI_CALL {
                return false;
            }
            call = Some(op);
        }
        let free_input = (0..o.num_input())
            .any(|s| o.get_in(s).and_then(|v| data.vbank().get(v)).is_some_and(|v| v.is_free() && !v.is_constant() && !v.is_annotation()));
        if free_input {
            return false;
        }
    }
    let Some(call) = call else { return false };
    let ends_in_return = ops.last().and_then(|&op| data.obank().get(op)).is_some_and(|o| o.code() == OpCode::CPUI_RETURN);
    ends_in_return && data.get_call_specs_index(call).is_some() && reads_merge(data, &ops, call, 0)
}

impl Action for ActionReturnDupCalls {
    fn base(&self) -> &ActionBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ActionBase {
        &mut self.base
    }
    fn clone_filtered(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Action>> {
        if !grouplist.contains(self.get_group()) {
            return None;
        }
        Some(Box::new(ActionReturnDupCalls { base: self.base.clone() }))
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        if !data.get_arch().duplicate_call_returns {
            return 0;
        }
        let root = data.bblocks_root_pub();
        let cands: Vec<BlockId> =
            data.bblocks_ref().block(root).get_list().iter().copied().filter(|&b| is_candidate(data, b)).collect();
        let mut count = 0;
        for b in cands {
            while count < MAX_SPLITS && data.bblocks_ref().block(b).size_in() > 1 {
                let last = data.bblocks_ref().block(b).size_in() - 1;
                if data.node_split_calls(b, last).is_err() {
                    break;
                }
                count += 1;
            }
        }
        self.base_mut().count += count;
        0
    }
}
