//! Expose Go write-barrier guards hidden behind jump-only blocks before SSA.

use crate::action::{Action, ActionBase, ActionContext, ActionGroupList, ApplyResult};
use crate::block::BlockKind;
use crate::context::BlockId;
use crate::funcdata::Funcdata;
use kuna_num::opcodes::OpCode;

fn removable(data: &Funcdata, block: BlockId) -> bool {
    let graph = data.bblocks_ref();
    let node = graph.block(block);
    if !matches!(node.kind(), BlockKind::Basic(_))
        || node.is_entry_point()
        || node.size_in() != 1
        || node.size_out() != 1
    {
        return false;
    }
    let predecessor = node.get_in(0);
    let successor = node.get_out(0);
    if predecessor == block || successor == block || predecessor == successor {
        return false;
    }
    if (0..graph.block(predecessor).size_out())
        .any(|edge| graph.block(predecessor).get_out(edge) == successor)
    {
        return false;
    }
    let Some(op) = data.bb_op_head(block) else {
        return false;
    };
    data.bb_op_tail(block) == Some(op)
        && data.obank().get(op).is_some_and(|op| op.code() == OpCode::CPUI_BRANCH)
}

fn follow(data: &Funcdata, mut block: BlockId) -> (BlockId, Vec<BlockId>) {
    let mut path = Vec::new();
    while removable(data, block) && !path.contains(&block) {
        path.push(block);
        block = data.bblocks_ref().block(block).get_out(0);
    }
    (block, path)
}

fn barrier_forwarders(data: &Funcdata) -> Option<Vec<BlockId>> {
    if !(0..data.num_calls()).any(|i| {
        crate::kuna_gowritebarrier::is_write_barrier_name(data.get_call_specs(i).get_name())
    }) {
        return None;
    }
    for index in 0..data.bblocks_get_size() {
        let guard = data.bblocks_get_block(index);
        if data.bblocks_ref().block(guard).size_out() != 2 {
            continue;
        }
        let Some(tail) = data.bb_op_tail(guard) else { continue };
        if data.obank().get(tail).expect("gojumpchain guard").code() != OpCode::CPUI_CBRANCH {
            continue;
        }
        let paths: Vec<_> = (0..2)
            .map(|edge| follow(data, data.bblocks_ref().block(guard).get_out(edge)))
            .collect();
        for edge in 0..2 {
            let cold = paths[edge].0;
            let node = data.bblocks_ref().block(cold);
            if cold == guard || node.size_in() != 1 || node.size_out() != 1 {
                continue;
            }
            let (join, after) = follow(data, node.get_out(0));
            if join != paths[1 - edge].0 || join == guard || join == cold {
                continue;
            }
            let calls: Vec<_> = (0..data.num_calls())
                .map(|i| data.get_call_specs(i))
                .filter(|call| data.obank().get(call.get_op()).and_then(|op| op.get_parent()) == Some(cold))
                .collect();
            if calls.len() != 1
                || !crate::kuna_gowritebarrier::is_write_barrier_name(calls[0].get_name())
            {
                continue;
            }
            let mut result = paths[0].1.clone();
            result.extend(paths[1].1.iter().copied());
            result.extend(after);
            if !result.is_empty() {
                return Some(result);
            }
        }
    }
    None
}

pub struct ActionGoJumpChain {
    base: ActionBase,
}

impl ActionGoJumpChain {
    pub fn boxed(group: impl Into<String>) -> Box<dyn Action> {
        Box::new(Self {
            base: ActionBase::new(0, "gojumpchain", group),
        })
    }
}

impl Action for ActionGoJumpChain {
    fn base(&self) -> &ActionBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ActionBase {
        &mut self.base
    }
    fn clone_filtered(&self, groups: &ActionGroupList) -> Option<Box<dyn Action>> {
        groups.contains(self.get_group()).then(|| {
            Box::new(Self {
                base: self.base.clone(),
            }) as Box<dyn Action>
        })
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        if !data.get_arch().go_jump_chain
            || !data.get_arch().source_lang.is_go()
            || data.get_heritage_pass() != 0
            || data.num_calls() == 0
            || !crate::kuna_gowritebarrier::live(data)
        {
            return 0;
        }
        let mut changed = 0;
        while let Some(blocks) = barrier_forwarders(data) {
            for block in blocks {
                if removable(data, block) {
                    data.remove_do_nothing_block(block)
                        .expect("gojumpchain remove branch-only block");
                    self.base.count += 1;
                    changed = 1;
                }
            }
        }
        changed
    }
}
