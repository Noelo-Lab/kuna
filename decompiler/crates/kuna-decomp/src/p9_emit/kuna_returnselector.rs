//! Materialize finite caller-return selectors after recovery and type analysis.
use crate::action::{Action, ActionBase, ActionContext, ActionGroupList, ApplyResult};
use crate::funcdata::Funcdata;

pub struct ActionReturnSelector {
    base: ActionBase,
}
impl ActionReturnSelector {
    pub fn boxed(group: impl Into<String>) -> Box<dyn Action> {
        Box::new(Self {
            base: ActionBase::new(0, "returnselector", group),
        })
    }
}
impl Action for ActionReturnSelector {
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
    fn apply(&mut self, data: &mut Funcdata, ctx: &mut ActionContext) -> ApplyResult {
        let mut changed = false;
        for i in 0..data.num_jump_tables() {
            let table = data.get_jump_table(i);
            let live = table.get_indirect_op().is_some_and(|id| {
                data.obank()
                    .get(id)
                    .is_some_and(|op| !op.is_dead() && op.get_parent().is_some())
            });
            if !live || !table.is_labelled() || table.kuna_return_default().is_none() {
                continue;
            }
            let placeholder = crate::jumptable::JumpTable::new(data.get_address().clone());
            let table = std::mem::replace(data.get_jump_table_mut(i), placeholder);
            if crate::kuna_indirectreturn::normalize_return_selector(data, &table) {
                self.base.count += 1;
                changed = true;
            } else {
                data.warning("Indirect jump caller-return selector could not be normalized",
                    table.get_op_address());
            }
            *data.get_jump_table_mut(i) = table;
        }
        if changed {
            crate::p9_emit::coreaction_render::ActionDeadCode::boxed("deadcode").apply(data, ctx);
        }
        0
    }
}
