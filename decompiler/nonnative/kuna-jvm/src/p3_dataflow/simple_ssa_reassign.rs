//! Port of `org/jetbrains/java/decompiler/modules/decompiler/sforms/SimpleSSAReassign.java`.

use crate::counter_container::CounterContainer;
use crate::decompiler_context::DecompilerContext;
use crate::exprent::Exprent;
use crate::instruction::Instruction;
use crate::root_statement::RootStatement;
use crate::rt::{JHashMap, JHashSet};
use crate::ssa_constructor_sparse_ex::SSAConstructorSparseEx;
use crate::statement::Statement;
use crate::var_exprent::VarExprent;
use crate::var_version_pair::VarVersionPair;

pub struct SimpleSSAReassign;

impl SimpleSSAReassign {
    pub fn reassign_ssa_form(
        ssa: &SSAConstructorSparseEx,
        root: RootStatement,
    ) -> JHashMap<Instruction, i32> {
        let mut vers: JHashSet<i32> = JHashSet::new();
        let version_lookup = ssa.get_simple_reverse_phi_lookup();

        Self::find_all_vars(root.into(), &mut |var: VarExprent| {
            let index = var.get_index();

            if index < VarExprent::STACK_BASE {
                vers.add(index);
            }
        });

        let max_ver =
            DecompilerContext::get_counter_container().get_counter(CounterContainer::VAR_COUNTER);

        if vers.is_empty() {
            return JHashMap::new();
        }

        let mut counter = max_ver;

        let mut new_vers: JHashMap<VarVersionPair, i32> = JHashMap::new();
        Self::find_all_vars(root.into(), &mut |var: VarExprent| {
            let vvp = var.get_var_version_pair();
            let version = version_lookup.get(&vvp).copied().unwrap_or(vvp.version);

            if version > 1 && vers.contains(&vvp.var) {
                let mut base = vvp;
                let base_is_new = vvp.version != version;
                if base_is_new {
                    base = VarVersionPair::new(vvp.var, version);
                }
                if !new_vers.contains_key(&base) {
                    counter += 1;
                    new_vers.put(base, counter);
                }

                if base_is_new {
                    let value = new_vers.get(&base).copied().expect("newVers.get(base)");
                    new_vers.put(vvp, value);
                }
            }
        });

        let mut rewrite_map: JHashMap<Instruction, i32> = JHashMap::new();
        Self::find_all_vars(root.into(), &mut |var: VarExprent| {
            let vvp = var.get_var_version_pair();

            if let Some(&new_idx) = new_vers.get(&vvp) {
                var.set_index(new_idx);
                var.set_version(1);

                if let Some(instr) = var.get_backing_instr() {
                    rewrite_map.put(instr, new_idx);
                }
            }
        });

        rewrite_map
    }

    /// Java: findAllVars(Statement, Consumer)
    fn find_all_vars(stat: Statement, action: &mut dyn FnMut(VarExprent)) {
        match stat.get_exprents() {
            None => {
                for st in stat.get_stats().borrow().to_vec() {
                    Self::find_all_vars(st, action);
                }

                for exprent in stat.get_stat_exprents().to_vec() {
                    Self::find_all_vars_exprent(exprent, action);
                }
            }
            Some(exprents) => {
                for exprent in exprents.to_vec() {
                    Self::find_all_vars_exprent(exprent, action);
                }
            }
        }
    }

    /// Java: findAllVars(Exprent, Consumer)
    fn find_all_vars_exprent(exprent: Exprent, action: &mut dyn FnMut(VarExprent)) {
        let lst = exprent.get_all_exprents_rec_self(true, true).to_vec();

        for expr in lst {
            if let Some(var) = expr.try_as_var() {
                action(var);
            }
        }
    }
}
