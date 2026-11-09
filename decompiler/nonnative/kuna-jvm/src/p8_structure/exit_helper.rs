//! Port of `org/jetbrains/java/decompiler/modules/decompiler/ExitHelper.java`.
#![allow(unused_variables, dead_code)]

use crate::do_statement::Type as DoType;
use crate::exit_exprent::Type as ExitType;
use crate::if_statement::IfStatement;
use crate::method_descriptor::MethodDescriptor;
use crate::root_statement::RootStatement;
use crate::sequence_helper::SequenceHelper;
use crate::stat_edge::StatEdge;
use crate::statement::{EdgeDirection, Statement};
use crate::unported;

pub struct ExitHelper;

impl ExitHelper {
    pub fn condense_exits(root: RootStatement) -> bool {
        let changed = Self::integrate_exits(root.0);

        if changed > 0 {
            Self::clean_up_unreachable_blocks(root.0);
            SequenceHelper::condense_sequences(root.0);
        }

        changed > 0
    }

    fn clean_up_unreachable_blocks(stat: Statement) {
        loop {
            let mut found = false;

            let mut i = 0;
            while i < stat.get_stats().borrow().len() {
                let st = stat.get_stats().borrow().get(i);

                Self::clean_up_unreachable_blocks(st);

                if st.is_sequence() && st.get_stats().borrow().len() > 1 {
                    let last = st.get_stats().borrow().get_last();
                    let secondlast = {
                        let stats = st.get_stats();
                        let stats = stats.borrow();
                        stats.get(stats.len() - 2)
                    };

                    if !secondlast.has_basic_succ_edge() {
                        let mut set = last.get_neighbours_set(
                            Statement::STATEDGE_DIRECT_ALL,
                            EdgeDirection::Backward,
                        );
                        set.remove(&secondlast);

                        if set.is_empty() {
                            last.get_exprents().unwrap().clear();
                            let last_id = last.with(|d| d.id);
                            st.get_stats().borrow_mut().remove_with_key(&last_id);
                            for succ_edge in last.get_all_successor_edges() {
                                succ_edge.remove();
                            }
                            for pred_edge in last.get_all_predecessor_edges() {
                                pred_edge.remove();
                            }
                            found = true;
                            break;
                        }
                    }
                }
                i += 1;
            }

            if !found {
                break;
            }
        }
    }

    fn integrate_exits(stat: Statement) -> i32 {
        unported!("ExitHelper.integrateExits")
    }

    fn is_exit_edge(edge: StatEdge) -> Option<Statement> {
        let dest = edge.get_destination();

        let (explicit, labeled, can_inline, closure) =
            edge.with(|d| (d.explicit, d.labeled, d.can_inline, d.closure));
        if edge.get_type() == StatEdge::TYPE_BREAK
            && dest.is_basic_block()
            && explicit
            && (labeled || Self::is_only_edge(edge))
            && can_inline
        {
            let mut parent = closure;
            while let Some(p) = parent {
                if p.try_as_switch().is_some_and(|sw| sw.is_phantom()) {
                    return None;
                }
                parent = p.get_parent();
            }

            let data = dest.get_exprents();

            if let Some(data) = data {
                if data.len() == 1 && data.get(0).is_exit() {
                    return Some(dest);
                }
            }
        }

        None
    }

    fn find_if_break_edge(ifst: IfStatement, edge: Option<StatEdge>) -> Option<Statement> {
        let edge = edge?;

        let dest = edge.get_destination();

        match ifst.get_basichead().get_exprents() {
            Some(exprents) if exprents.is_empty() => {}
            _ => return None,
        }

        let (explicit, labeled, can_inline, closure) =
            edge.with(|d| (d.explicit, d.labeled, d.can_inline, d.closure));
        if edge.get_type() != StatEdge::TYPE_BREAK
            || !dest.is_basic_block()
            || !explicit
            || !can_inline
        {
            return None;
        }

        if !labeled && !Self::is_only_edge(edge) && {
            let parent = ifst.get_parent().unwrap();
            Some(parent) != closure
                || !parent.has_any_successor()
                || edge.get_destination() != parent.get_first_successor().get_destination()
        } {
            return None;
        }

        let mut parent = closure;
        while let Some(p) = parent {
            if p.try_as_switch().is_some_and(|sw| sw.is_phantom()) {
                return None;
            }

            parent = p.get_parent();
        }

        let data = dest.get_exprents();
        match data {
            Some(data) if data.len() == 1 && data.get(0).is_exit() => {}
            _ => return None,
        }

        Some(dest)
    }

    fn is_only_edge(edge: StatEdge) -> bool {
        let stat = edge.get_destination();

        for ed in stat.get_all_predecessor_edges() {
            if ed != edge {
                if ed.get_type() == StatEdge::TYPE_REGULAR {
                    let source = ed.get_source();

                    if source.is_basic_block()
                        || source.try_as_if().is_some_and(|source_if| {
                            source_if.d(|d| d.iftype) == IfStatement::IFTYPE_IF
                        })
                        || source
                            .try_as_do()
                            .is_some_and(|source_do| source_do.get_looptype() != DoType::Infinite)
                    {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }

        true
    }

    pub fn remove_redundant_returns(root: RootStatement) -> bool {
        let mut res = false;
        let dummy_exit = root.get_dummy_exit();

        for edge in dummy_exit.get_all_predecessor_edges() {
            if !edge.with(|d| d.explicit) {
                let source = edge.get_source();
                let lst_expr = source.get_exprents();
                if let Some(lst_expr) = lst_expr.filter(|l| !l.is_empty()) {
                    let expr = lst_expr.get(lst_expr.len() - 1);
                    if let Some(ex) = expr.try_as_exit() {
                        if ex.get_exit_type() == ExitType::Return && ex.get_value().is_none() {
                            let bytecode = ex.get_bytecode();
                            dummy_exit.add_bytecode_offsets(bytecode.as_ref());
                            lst_expr.remove_at(lst_expr.len() - 1);
                            res = true;
                        }
                    }
                }
            }
        }

        res
    }

    pub fn adjust_return_type(root: RootStatement, desc: &MethodDescriptor) -> bool {
        let mut res = false;
        for ret_edge in root.get_dummy_exit().get_all_predecessor_edges() {
            let ret = ret_edge.get_source();

            let exprents = ret.get_exprents();
            if let Some(exprents) = exprents.filter(|e| !e.is_empty()) {
                let expr = exprents.get(exprents.len() - 1);
                if let Some(ex) = expr.try_as_exit() {
                    let exit_exprents = ex.get_all_exprents_rec(true);

                    for exprent in exit_exprents {
                        if exprent.is_const() {
                            exprent.as_const().adjust_const_type(&desc.ret);
                            res = true;
                        }
                    }
                }
            }
        }

        res
    }
}
