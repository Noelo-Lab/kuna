//! Port of `org/jetbrains/java/decompiler/modules/decompiler/PPandMMHelper.java`.
#![allow(unused_variables, dead_code)]

use crate::direct_graph::DirectGraph;
use crate::exprent::Exprent;
use crate::if_statement::IfStatement;
use crate::root_statement::RootStatement;
use crate::rt::JList;
use crate::statement::Statement;
use crate::unported;
use crate::var_processor::VarProcessor;
use crate::var_version_pair::VarVersionPair;

/// Folds `x = x + 1` into `++x`; one instance per pass, never stored.
pub struct PPandMMHelper {
    exprent_replaced: bool,
    var_proc: VarProcessor,
    dgraph: Option<DirectGraph>,
}

impl PPandMMHelper {
    pub fn new(var_proc: VarProcessor) -> PPandMMHelper {
        PPandMMHelper {
            exprent_replaced: false,
            var_proc,
            dgraph: None,
        }
    }

    pub fn find_p_pand_mm(&mut self, root: RootStatement) -> bool {
        unported!("PPandMMHelper.findPPandMM")
    }

    fn process_exprent_list(&mut self, lst: &JList<Exprent>) -> bool {
        unported!("PPandMMHelper.processExprentList")
    }

    fn process_exprent_recursive(&mut self, exprent: Exprent) -> Option<Exprent> {
        unported!("PPandMMHelper.processExprentRecursive")
    }

    fn exprs_equal(&self, e1: Option<Exprent>, e2: Option<Exprent>) -> bool {
        if e1 == e2 {
            return true;
        }
        let (Some(e1), Some(e2)) = (e1, e2) else {
            return false;
        };
        if e1.is_var() {
            return self.vars_equal(e1, e2);
        }
        e1.equals(e2)
    }

    fn vars_equal(&self, e1: Exprent, e2: Exprent) -> bool {
        let Some(v1) = e1.try_as_var() else {
            return false;
        };
        let Some(v2) = e2.try_as_var() else {
            return false;
        };
        self.var_proc.get_var_original_index(v1.get_index())
            == self.var_proc.get_var_original_index(v2.get_index())
    }

    fn update_versions(
        &self,
        graph: DirectGraph,
        old_vvp: VarVersionPair,
        new_vvp: VarVersionPair,
    ) {
        unported!("PPandMMHelper.updateVersions")
    }

    pub fn inline_pp_iand_mmi_if(stat: RootStatement) -> bool {
        unported!("PPandMMHelper.inlinePPIandMMIIf")
    }

    fn inline_pp_iand_mmi_if_rec(stat: Statement) -> bool {
        unported!("PPandMMHelper.inlinePPIandMMIIfRec")
    }

    fn find_if_successor(stat: Statement) -> Option<IfStatement> {
        if let Some(parent) = stat.get_parent() {
            if let Some(if_parent) = parent.try_as_if() {
                if parent.get_first() == Some(stat) {
                    return Some(if_parent);
                }
            }
        }

        None
    }
}
