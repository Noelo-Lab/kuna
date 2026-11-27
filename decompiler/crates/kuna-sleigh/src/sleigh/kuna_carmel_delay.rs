//! Carmel's partial issue composition rejects effects whose timing is not modeled.

use std::ops::Range;

use kuna_base::address::Address;
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::space::spacetype::IPTR_CONSTANT;
use kuna_num::opcodes::OpCode;

use super::{ContextDatabase, ParserContext, PcodeCacher, PcodeData, Sleigh};

impl Sleigh {
    pub(super) fn uses_carmel_issues(&self) -> bool {
        self.base.base.get_alignment() == 3
            && self.base.manager.get_default_code_space().is_some_and(|space| {
                space.get_word_size() == 3 && space.get_name() == "code"
            })
            && self.context_db.borrow().get_variable(b"carmel_issue").is_ok()
    }

    pub(super) fn carmel_counted_delay(&self, pos: &ParserContext) -> bool {
        self.context_db.borrow().get_variable(b"carmel_counted_delay")
            .is_ok_and(|field| field.get_value(&pos.context) != 0)
    }

    pub(super) fn carmel_split_delay(&self, pos: &ParserContext) -> bool {
        self.context_db.borrow().get_variable(b"carmel_split_delay")
            .is_ok_and(|field| field.get_value(&pos.context) != 0)
    }

    pub(super) fn check_carmel_fetch(&self, addr: &Address) -> KunaResult<()> {
        if self.base.base.get_alignment() == 3
            && matches!(self.context_db.borrow().get_variable_value(b"carmel_bad_fetch", addr), Ok(1))
        {
            return Err(KunaError::bad_data("Carmel fetch rejected before issue"));
        }
        Ok(())
    }
}

/// Decode just the delayed issue, not the fused two-issue DATA schedule.
pub(super) fn select_single_issue(db: &dyn ContextDatabase, pos: &mut ParserContext) {
    if let Ok(field) = db.get_variable(b"carmel_single_issue") {
        field.set_value(&mut pos.context, 1);
    }
}

/// The one memory-bearing delay range that may share a simple transfer.
#[derive(Default)]
pub(super) struct DelayUops {
    memory_slot: Option<Range<usize>>,
}

impl DelayUops {
    pub(super) fn record(
        &mut self,
        engine: &Sleigh,
        cache: &PcodeCacher,
        start: usize,
        multiple_issues: bool,
        length: i32,
    ) -> KunaResult<()> {
        let slot = &cache.issued[start..];
        let external_flow = |op: &PcodeData| match op.opc {
            OpCode::CPUI_BRANCH | OpCode::CPUI_CBRANCH => op
                .invar
                .and_then(|i| cache.pool[i].space.as_ref())
                .is_none_or(|space| space.get_type() != IPTR_CONSTANT),
            OpCode::CPUI_BRANCHIND | OpCode::CPUI_CALL | OpCode::CPUI_CALLIND
            | OpCode::CPUI_RETURN => true,
            OpCode::CPUI_CALLOTHER => !delay_admission(engine, cache, op),
            _ => false,
        };
        if slot.iter().any(external_flow) {
            return Err(unsupported(length));
        }
        if slot.iter().any(memory) {
            if multiple_issues || self.memory_slot.is_some() {
                return Err(unsupported(length));
            }
            self.memory_slot = Some(start..cache.issued.len());
        }
        Ok(())
    }

    /// After the parent template is complete, late stores and repeat backedges
    /// are visible. They have no modeled DATA ordering relative to the slot.
    pub(super) fn validate(&self, cache: &PcodeCacher, length: i32) -> KunaResult<()> {
        let Some(slot) = &self.memory_slot else { return Ok(()); };
        if cache.issued[..slot.start].iter().chain(&cache.issued[slot.end..]).any(memory)
            || cache.label_refs.iter().any(|reference| {
                let from = reference.calling_index;
                let label = cache.pool[reference.dataptr].offset as usize;
                let Some(&to) = cache.labels.get(label) else { return false; };
                let to = to as usize;
                matches!(cache.issued[from].opc, OpCode::CPUI_BRANCH | OpCode::CPUI_CBRANCH)
                    && to <= from && to < slot.end && from >= slot.start
                    && (!slot.contains(&from) || !slot.contains(&to))
            })
        {
            return Err(unsupported(length));
        }
        Ok(())
    }
}

fn delay_admission(engine: &Sleigh, cache: &PcodeCacher, op: &PcodeData) -> bool {
    if op.outvar.is_some() || op.isize != 2 {
        return false;
    }
    let Some(input) = op.invar else { return false; };
    let selector = &cache.pool[input];
    cache.pool[input + 1].size == 1
        && selector.space.as_ref().is_some_and(|space| space.get_type() == IPTR_CONSTANT)
        && engine.base.get_user_op_names().get(selector.offset as usize)
            .is_some_and(|name| name.as_slice() == b"carmel_delay_slot_admission")
}

fn memory(op: &PcodeData) -> bool {
    matches!(op.opc, OpCode::CPUI_LOAD | OpCode::CPUI_STORE)
}

fn unsupported(length: i32) -> KunaError {
    KunaError::unimpl("Unsupported Carmel delay issue effects", length)
}
