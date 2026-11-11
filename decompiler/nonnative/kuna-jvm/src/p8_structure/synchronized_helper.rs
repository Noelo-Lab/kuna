//! Port of `org/jetbrains/java/decompiler/modules/decompiler/SynchronizedHelper.java`.
//!
//! kuna fix `synchronized_lock_assignment`: `cleanSynchronizedVar` takes only
//! the assignment right before the monitor for the compiler's lock variable
//! (`kuna_fix_synchronized_lock_assignment`).

use crate::assignment_exprent::AssignmentExprent;
use crate::counter_container::CounterContainer;
use crate::decompiler_context::DecompilerContext;
use crate::kuna_fixes::{self, KunaFix};
use crate::p8_structure::kuna_fix_synchronized_lock_assignment::is_lock_variable;
use crate::root_statement::RootStatement;
use crate::statement::Statement;
use crate::var_exprent::VarExprent;
use crate::var_processor::VarProcessor;
use crate::var_type::VarType;

pub struct SynchronizedHelper;

impl SynchronizedHelper {
    pub fn clean_synchronized_var(stat: Statement) -> bool {
        let mut res = false;
        let stats = stat.get_stats().borrow().to_vec();
        for st in stats {
            res |= Self::clean_synchronized_var(st);
        }

        if let Some(sync) = stat.try_as_synchronized() {
            let head = sync.get_headexprent_list().first();
            if let Some(mon) = head.and_then(|h| h.try_as_monitor()) {
                let first_exprents = sync
                    .get_first()
                    .expect("first")
                    .get_exprents()
                    .expect("exprents");
                for e in first_exprents.iter() {
                    if let Some(ass) = e.try_as_assignment() {
                        if let Some(var) = ass.get_left().try_as_var() {
                            let fixed = kuna_fixes::enabled(KunaFix::SynchronizedLockAssignment);
                            if var.equals(mon.get_value())
                                && (!fixed
                                    || is_lock_variable(&first_exprents, e, ass.get_right(), true))
                                && !var.is_var_referenced(
                                    stat.get_parent().expect("parent"),
                                    &[mon.get_value().as_var()],
                                )
                            {
                                first_exprents.remove_item(&e);
                                let bytecode = mon.get_value().get_bytecode();
                                mon.add_bytecode_offsets(bytecode.as_ref());
                                mon.replace_exprent(mon.get_value(), ass.get_right());
                                res = true;
                                break;
                            } else if ass.get_right().equals(mon.get_value())
                                && (!fixed
                                    || is_lock_variable(&first_exprents, e, ass.get_right(), false))
                                && !var.is_var_referenced(stat.get_parent().expect("parent"), &[])
                            {
                                first_exprents.remove_item(&e);
                                res = true;
                                break;
                            }
                        }
                    }
                }
            }
        }

        res
    }

    pub fn insert_sink(root: RootStatement, var_processor: VarProcessor, stat: Statement) -> bool {
        let mut res = false;
        let stats = stat.get_stats().borrow().to_vec();
        for st in stats {
            res |= Self::insert_sink(root, var_processor, st);
        }

        if let Some(sync) = stat.try_as_synchronized() {
            let mon = sync.get_headexprent().as_monitor();
            let value = mon.get_value();

            if value.try_as_const().is_some_and(|c| {
                !c.get_const_type().ptr_eq(&VarType::VARTYPE_STRING())
                    && !c.get_const_type().is_generic_type()
            }) {
                let var = DecompilerContext::get_counter_container()
                    .get_counter_and_increment(CounterContainer::VAR_COUNTER);

                let var_ex =
                    VarExprent::new(var, Some(VarType::VARTYPE_OBJECT()), Some(var_processor));
                var_processor
                    .set_var_type(var_ex.get_var_version_pair(), &VarType::VARTYPE_OBJECT());

                let assign = AssignmentExprent::new(var_ex.0, value, None);
                mon.replace_exprent(value, assign.0);
                assign.add_bytecode_offsets(value.get_bytecode().as_ref());
                root.add_comment("$VF: Added assignment to ensure synchronized validity");
            } else if let Some(inv) = value.try_as_invocation() {
                if inv.is_boxing_call() {
                    inv.mark_using_boxing_result();
                }
            }
        }

        res
    }
}
