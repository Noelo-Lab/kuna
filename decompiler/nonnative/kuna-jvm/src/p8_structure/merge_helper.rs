//! Port of `org/jetbrains/java/decompiler/modules/decompiler/MergeHelper.java`.
#![allow(unused_variables, dead_code)]

use crate::do_statement::{DoStatement, Type as LoopType};
use crate::exprent::Exprent;
use crate::function_exprent::FunctionType;
use crate::invocation_exprent::InvocationExprent;
use crate::root_statement::RootStatement;
use crate::rt::JHashSet;
use crate::sequence_helper::SequenceHelper;
use crate::sequence_statement::SequenceStatement;
use crate::stat_edge::StatEdge;
use crate::statement::Statement;
use crate::unported;
use crate::validation_helper::ValidationHelper;
use crate::var_exprent::VarExprent;

pub struct MergeHelper;

impl MergeHelper {
    pub fn enhance_loops(root: Statement) -> bool {
        let mut res = false;
        while Self::enhance_loops_rec(root) {
            res = true;
        }

        if res {
            SequenceHelper::condense_sequences(root);
        }
        res
    }

    fn enhance_loops_rec(stat: Statement) -> bool {
        let mut res = false;

        let stats = stat.get_stats().borrow().to_vec();

        for st in stats {
            if st.get_exprents().is_none() {
                res |= Self::enhance_loops_rec(st);
            }
        }

        if let Some(dostat) = stat.try_as_do() {
            res |= Self::enhance_loop(dostat);
        }

        res
    }

    fn enhance_loop(stat: DoStatement) -> bool {
        let oldloop = stat.get_looptype();

        match oldloop {
            LoopType::Infinite => {
                if Self::match_while(stat) && !Self::match_for_each(stat) {
                    Self::match_for(stat);
                }
            }
            LoopType::While => {
                if !Self::match_for_each(stat) {
                    Self::match_for(stat);
                }
            }
            _ => {}
        }

        stat.get_looptype() != oldloop
    }

    fn match_do_while(stat: DoStatement) {
        unported!("MergeHelper.matchDoWhile")
    }

    fn is_last_in_loop(stat: Statement) -> bool {
        let mut stat = stat;
        while let Some(parent) = stat.get_parent().filter(|p| p.is_sequence()) {
            let stats = parent.get_stats();
            if stats.borrow().index_of(&stat) != stats.borrow().len() as i32 - 1 {
                return false;
            }

            stat = parent;
        }

        stat.get_parent().is_some_and(|p| p.is_do())
    }

    pub fn match_while(stat: DoStatement) -> bool {
        unported!("MergeHelper.matchWhile")
    }

    pub fn is_direct_path(stat: Statement, endstat: Statement) -> bool {
        unported!("MergeHelper.isDirectPath")
    }

    pub fn match_for(stat: DoStatement) {
        unported!("MergeHelper.matchFor")
    }

    /// Java: getFinalVariables(Statement, Set)
    fn get_final_variables(
        stat: Statement,
        variables: JHashSet<VarExprent>,
    ) -> JHashSet<VarExprent> {
        let mut variables = variables;
        match stat.get_exprents() {
            None => {
                let stats = stat.get_stats().borrow().to_vec();
                for s in stats {
                    variables = Self::get_final_variables(s, variables);
                }

                for exp in stat.get_stat_exprents() {
                    variables = Self::get_final_variables_exprent(exp, variables);
                }
            }
            Some(exprents) => {
                for exp in exprents.iter() {
                    variables = Self::get_final_variables_exprent(exp, variables);
                }
            }
        }
        variables
    }

    /// Java: getFinalVariables(Exprent, Set)
    fn get_final_variables_exprent(
        exp: Exprent,
        variables: JHashSet<VarExprent>,
    ) -> JHashSet<VarExprent> {
        let mut variables = variables;
        for e in exp.get_all_exprents_rec_self(true, true) {
            if let Some(assignment) = e.try_as_assignment() {
                if let Some(var_exprent) = assignment.get_left().try_as_var() {
                    if var_exprent.is_effectively_final() {
                        variables.add(var_exprent);
                    }
                }
            }
        }
        variables
    }

    pub fn clean_empty_statements(dostat: DoStatement, stat: Option<Statement>) {
        let Some(stat) = stat else {
            return;
        };
        if stat.get_exprents().unwrap().is_empty() {
            let lst = stat.get_all_successor_edges();
            if !lst.is_empty() {
                stat.remove_successor(lst[0]);
            }

            if stat
                .get_parent()
                .is_some_and(|p| p.is_catch() || p.is_catch_all() || p.is_synchronized())
            {
                return;
            }

            Self::remove_last_empty_statement(dostat, stat);
        }
    }

    pub fn remove_last_empty_statement(dostat: DoStatement, stat: Statement) {
        unported!("MergeHelper.removeLastEmptyStatement")
    }

    pub fn get_last_direct_data(stat: Statement) -> Option<Statement> {
        if stat.get_exprents().is_some() {
            return Some(stat);
        }

        let stats = stat.get_stats();
        let size = stats.borrow().len();
        for i in (0..size).rev() {
            let st = stats.borrow().get(i);
            let tmp = Self::get_last_direct_data(st);
            match tmp {
                None => return None,
                Some(t) if !t.get_exprents().unwrap().is_empty() => return tmp,
                Some(_) => {}
            }
        }
        None
    }

    pub fn match_for_each(stat: DoStatement) -> bool {
        unported!("MergeHelper.matchForEach")
    }

    pub fn is_var_used_before(var: VarExprent, st: Statement) -> bool {
        unported!("MergeHelper.isVarUsedBefore")
    }

    fn is_var_used_after(var: VarExprent, this_st: DoStatement) -> bool {
        let Some(parent) = this_st.get_parent().filter(|p| p.is_sequence()) else {
            return false;
        };
        let idx = parent.get_stats().borrow().index_of(&this_st.0);
        let mut i = idx + 1;
        while (i as usize) < parent.get_stats().borrow().len() {
            let st = parent.get_stats().borrow().get(i as usize);
            if Self::is_var_used_in(var, st) {
                return true;
            }
            i += 1;
        }

        false
    }

    fn is_var_used_in(var: VarExprent, stat: Statement) -> bool {
        let stats = stat.get_stats().borrow().to_vec();
        for st in stats {
            if Self::is_var_used_in(var, st) {
                return true;
            }
        }

        for ex in stat.get_stat_exprents() {
            if Self::contains_var(var, ex) {
                return true;
            }
        }

        if let Some(exprents) = stat.get_exprents() {
            for ex in exprents.iter() {
                if Self::contains_var(var, ex) {
                    return true;
                }
            }
        }

        false
    }

    fn contains_var(var: VarExprent, ex: Exprent) -> bool {
        for e in ex.get_all_exprents_rec_self(true, true) {
            if let Some(v) = e.try_as_var() {
                if v.get_index() == var.get_index() {
                    return true;
                }
            }
        }
        false
    }

    pub fn drill_nots(exp: Exprent) -> Exprent {
        let mut exp = exp;
        loop {
            let Some(fun) = exp.try_as_function() else {
                return exp;
            };
            let func_type = fun.get_func_type();
            if func_type == FunctionType::BoolNot {
                exp = fun.get_lst_operands().get(0);
            } else if func_type == FunctionType::Eq || func_type == FunctionType::Ne {
                return fun.get_lst_operands().get(0);
            } else {
                return exp;
            }
        }
    }

    pub fn get_first_direct_data(stat: Statement) -> Option<Statement> {
        if stat.get_exprents().is_some_and(|e| !e.is_empty()) {
            return Some(stat);
        }

        let stats = stat.get_stats().borrow().to_vec();

        for tmp in stats {
            let ret = Self::get_first_direct_data(tmp);
            if ret.is_some() {
                return ret;
            }
        }
        None
    }

    pub fn get_uncast(exp: Exprent) -> Exprent {
        if let Some(func) = exp.try_as_function() {
            if func.get_func_type() == FunctionType::Cast {
                return Self::get_uncast(func.get_lst_operands().get(0));
            }
        }
        exp
    }

    pub fn as_invocation_exprent(exp: Exprent) -> Option<InvocationExprent> {
        Self::get_uncast(exp).try_as_invocation()
    }

    pub fn is_iterator_call(exp: Exprent) -> bool {
        unported!("MergeHelper.isIteratorCall")
    }

    pub fn is_has_next_call(exp: Exprent) -> bool {
        unported!("MergeHelper.isHasNextCall")
    }

    pub fn is_next_call(exp: Exprent) -> bool {
        unported!("MergeHelper.isNextCall")
    }

    pub fn is_next_unboxing(exprent: Exprent) -> bool {
        let Some(inv) = Self::get_uncast(exprent).try_as_invocation() else {
            return false;
        };
        inv.is_unboxing_call() && inv.get_instance().is_some_and(Self::is_next_call)
    }

    pub fn make_do_while_loops(root: RootStatement) -> bool {
        if Self::make_do_while_rec(root.0) {
            SequenceHelper::condense_sequences(root.0);
            return true;
        }

        false
    }

    fn make_do_while_rec(stat: Statement) -> bool {
        let mut ret = false;

        let stats = stat.get_stats().borrow().to_vec();

        for st in stats {
            ret |= Self::make_do_while_rec(st);
        }

        if let Some(dostat) = stat.try_as_do() {
            if dostat.get_looptype() == LoopType::Infinite {
                Self::match_do_while(dostat);
                if dostat.get_looptype() != LoopType::Infinite {
                    ret = true;
                    ValidationHelper::validate_statement(stat.get_top_parent());
                }
            }
        }

        ret
    }

    fn add_continue_or_break(stat: DoStatement, ifedge: StatEdge) -> bool {
        unported!("MergeHelper.addContinueOrBreak")
    }

    pub fn condense_infinite_loops_with_return(root: Statement) -> bool {
        let ret = Self::condense_infinite_loops_with_return_rec(root);

        if ret {
            SequenceHelper::condense_sequences(root);
        }

        ret
    }

    fn condense_infinite_loops_with_return_rec(stat: Statement) -> bool {
        let mut res = false;

        if let Some(loop_) = stat.try_as_do() {
            if loop_.get_looptype() == LoopType::Infinite {
                res = Self::condense_loop(loop_);
            }
        }

        let stats = stat.get_stats().borrow().to_vec();

        for st in stats {
            res |= Self::condense_infinite_loops_with_return_rec(st);
        }

        res
    }

    fn condense_loop(stat: DoStatement) -> bool {
        unported!("MergeHelper.condenseLoop")
    }

    fn extractable_from_loop(seq: SequenceStatement, loop_: DoStatement) -> i32 {
        unported!("MergeHelper.extractableFromLoop")
    }
}
