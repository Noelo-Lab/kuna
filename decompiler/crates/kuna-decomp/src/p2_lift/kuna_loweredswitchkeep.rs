//! (kuna `loweredswitch`) Keep a re-rolled switch only when the function reads
//! no worse with it.
//!
//! A re-rolled switch is the same program as the compare tree it replaces, but
//! the rest of the pipeline sees a different graph. A switch whose arms continue
//! an enclosing loop can leave the structurer without the loop, printed as
//! `goto`s around the switch; a register the compare tree read and the switch
//! does not can be taken for one more argument of a call or of the function.
//!
//! [`ActionLowerSwitchKeep`] runs at the end of every pass, once the structure and
//! the prototypes are final. On the pass that recorded a switch, before it is
//! installed, it records the function's [`Shape`]: the gotos the structured
//! tree prints, the function's parameter count and return size, and
//! the argument count of every call. On the pass with the switch installed it
//! measures again; a switch printed as a loop or `if` condition, or one that
//! changes the parameter list, the width of a returned value or any call's
//! arguments, is withdrawn and the function restarts without it, exactly as
//! `option loweredswitch off` renders it. So is a switch that prints more
//! `goto`s (a loop the structurer loses prints its back edge as one), unless it
//! is the binary-search tree with a compare-only head over a register selector
//! that the recogniser has always taken, which reads better than its compare
//! tree even with a goto or two.

use std::collections::{BTreeMap, BTreeSet};

use kuna_base::address::Address;
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::action::{Action, ActionBase, ActionContext, ActionGroupList, ApplyResult};
use crate::block::{block_flags, BlockType};
use crate::context::BlockId;
use crate::funcdata::Funcdata;
use crate::p2_lift::kuna_loweredswitch::SharedLoweredSwitchStore;
use crate::p2_lift::kuna_loweredswitchvalue::ValueState;

/// What a function prints, as far as a re-rolled switch may change it.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Shape {
    pub gotos: int4,
    /// Conditions (of an `if` or a loop) whose code ends in a switch: the
    /// structurer's way of losing an arm that continues a loop, printed as
    /// `} while switch (...)`, which is not C.
    pub switch_conditions: int4,
    pub params: int4,
    pub output: int4,
    pub calls: BTreeMap<Address, BTreeSet<int4>>,
}

impl Shape {
    /// The shape of `data` as structured and prototyped now.
    pub fn measure(data: &Funcdata) -> Shape {
        let mut shape = Shape {
            gotos: printed_gotos(data),
            params: data.get_func_proto().num_params(),
            output: data.get_func_proto().get_output().get_size(),
            ..Shape::default()
        };
        let graph = data.sblocks_ref();
        if let Some(root) = graph.root {
            let mut stack: Vec<BlockId> = vec![root];
            while let Some(b) = stack.pop() {
                let blk = graph.block(b);
                if matches!(blk.get_type(), BlockType::If | BlockType::WhileDo | BlockType::DoWhile)
                    && blk.get_size() > 0
                    && ends_in_switch(data, blk.get_block(0))
                {
                    shape.switch_conditions += 1;
                }
                stack.extend(blk.get_list().iter().copied());
            }
        }
        for op in data.obank().iter_alive() {
            let Some(o) = data.obank().get(op) else { continue };
            if matches!(o.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND) {
                shape.calls.entry(o.get_addr().clone()).or_default().insert(o.num_input() - 1);
            }
        }
        shape
    }

    /// True when `self` prints worse than `before`: a switch printed as a
    /// condition, another parameter list, another width of a value the function
    /// returned, or another argument list of a call both have; and with
    /// `structure`, more gotos.
    pub fn worse_than(&self, before: &Shape, structure: bool) -> bool {
        self.switch_conditions > before.switch_conditions
            || self.params != before.params
            || (before.output != 0 && self.output != before.output)
            || self.calls.iter().any(|(at, args)| before.calls.get(at).is_some_and(|b| b != args))
            || (structure && self.gotos > before.gotos)
    }
}

/// The `goto` statements the structured tree prints: the jumps of goto blocks,
/// goto-`if`s and bare-goto switch arms that the print-time census
/// ([`goto_rewrites`](crate::p8_structure::kuna_gotonext::goto_rewrites)) neither
/// leaves out nor prints as `break;`, and the edges of multi-goto blocks.
pub(crate) fn printed_gotos(data: &Funcdata) -> int4 {
    let graph = data.sblocks_ref();
    let Some(root) = graph.root else { return 0 };
    let rw = crate::p8_structure::kuna_gotonext::goto_rewrites(
        graph,
        root,
        data.get_arch().recover_loop_break,
        &crate::p8_structure::kuna_switchmerge::SwitchPlans::default(),
    );
    let mut n = 0;
    let mut stack: Vec<BlockId> = vec![root];
    while let Some(b) = stack.pop() {
        let blk = graph.block(b);
        match blk.get_type() {
            BlockType::Goto => {
                if blk.get_goto_type() == block_flags::f_goto_goto && !rw.silent.contains(&b) && !rw.breaks.contains(&b) {
                    n += 1;
                }
            }
            BlockType::MultiGoto => {
                if let crate::block::BlockKind::MultiGoto { gotoedges, .. } = blk.kind() {
                    n += gotoedges.len() as int4;
                }
            }
            BlockType::Switch => {
                n += blk
                    .switch_caseblocks()
                    .iter()
                    .filter(|c| c.gototype == block_flags::f_goto_goto && !rw.case_breaks.contains(&(b, c.block)))
                    .count() as int4;
            }
            _ => {
                if blk.get_if_goto_target().is_some()
                    && blk.get_if_goto_type() == block_flags::f_goto_goto
                    && !rw.breaks.contains(&b)
                {
                    n += 1;
                }
            }
        }
        stack.extend(blk.get_list().iter().copied());
    }
    n
}

/// True when the last block `blk` runs, through nested lists, is a switch.
fn ends_in_switch(data: &Funcdata, blk: BlockId) -> bool {
    let graph = data.sblocks_ref();
    let mut b = blk;
    for _ in 0..64 {
        let node = graph.block(b);
        match node.get_type() {
            BlockType::Switch => return true,
            BlockType::Ls | BlockType::Graph => match node.get_list().last() {
                Some(&last) => b = last,
                None => return false,
            },
            _ => return false,
        }
    }
    false
}

/// Measure each recorded switch's function without and with it, and withdraw a
/// switch the function prints worse with (see the module docs).
pub struct ActionLowerSwitchKeep {
    base: ActionBase,
    store: SharedLoweredSwitchStore,
}

impl ActionLowerSwitchKeep {
    pub fn with_store(g: impl Into<String>, store: SharedLoweredSwitchStore) -> ActionLowerSwitchKeep {
        ActionLowerSwitchKeep { base: ActionBase::new(0, "lowerswitchkeep", g), store }
    }

    /// The decision, exposed for testing without the Action state machine.
    /// Returns true when a switch was withdrawn.
    pub fn keep(&mut self, data: &mut Funcdata) -> bool {
        if !data.get_arch().recover_lowered_switch || data.is_jumptable_recovery_on() {
            return false;
        }
        let n = self.store.borrow().records(data).len();
        if n == 0 {
            return false;
        }
        let shape = Shape::measure(data);
        let mut withdrawn = false;
        for i in 0..n {
            let installed = {
                let store = self.store.borrow();
                let rec = &store.records(data)[i];
                (0..data.num_jump_tables())
                    .map(|j| data.get_jump_table(j))
                    .any(|jt| jt.kuna_lowered_var().is_some() && *jt.get_op_address() == rec.branch_addr)
            };
            let mut store = self.store.borrow_mut();
            let Some(rec) = store.record_mut(data, i) else { continue };
            if rec.value.state == ValueState::Withdrawn {
                continue;
            }
            match (&rec.shape_without, installed) {
                (None, false) => rec.shape_without = Some(shape.clone()),
                (Some(before), true) if shape.worse_than(before, !rec.base_shape) => {
                    rec.value.state = ValueState::Withdrawn;
                    withdrawn = true;
                }
                _ => {}
            }
        }
        if withdrawn {
            data.set_restart_pending(true);
        }
        withdrawn
    }
}

impl Action for ActionLowerSwitchKeep {
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
        Some(Box::new(ActionLowerSwitchKeep { base: self.base.clone(), store: self.store.clone() }))
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        let _ = self.keep(data);
        0
    }
}

#[cfg(test)]
#[path = "kuna_loweredswitchkeep/tests.rs"]
mod tests;
