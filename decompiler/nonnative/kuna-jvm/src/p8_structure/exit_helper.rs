//! Port of `org/jetbrains/java/decompiler/modules/decompiler/ExitHelper.java`.
//!
//! kuna option `jvm_javac_layout` (`kuna_javac_layout`): an if without a body
//! outside a switch keeps its if-edge in `integrate_exits` (the exit after
//! the code that follows it is not the if's).

use crate::basic_block_statement::BasicBlockStatement;
use crate::dec_helper::DecHelper;
use crate::do_statement::Type as DoType;
use crate::exit_exprent::Type as ExitType;
use crate::exprent::Exprent;
use crate::function_exprent::FunctionType;
use crate::if_statement::IfStatement;
use crate::merge_helper::MergeHelper;
use crate::p8_structure::kuna_javac_layout;
use crate::method_descriptor::MethodDescriptor;
use crate::root_statement::RootStatement;
use crate::rt::JList;
use crate::sequence_helper::SequenceHelper;
use crate::sequence_statement::SequenceStatement;
use crate::stat_edge::StatEdge;
use crate::statement::{EdgeDirection, Statement};
use crate::switch_statement::SwitchStatement;

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
        let mut ret = 0;

        if stat.get_exprents().is_none() {
            loop {
                let mut changed = 0;

                let stats = stat.get_stats().borrow().to_vec();
                for st in stats {
                    changed = Self::integrate_exits(st);
                    if changed > 0 {
                        ret = 1;
                        break;
                    }
                }

                if changed == 0 {
                    break;
                }
            }

            if let Some(ifst) = stat.try_as_if().filter(|ifst| ifst.get_ifstat().is_none()) {
                let ifedge = ifst.get_if_edge().unwrap();
                let mut dest = Self::is_exit_edge(ifedge);

                let mut if_break_edge: Option<StatEdge> = None;
                if dest.is_none() && !kuna_javac_layout::keeps_if_edge(ifst) {
                    if ifst.has_successor(StatEdge::TYPE_BREAK) {
                        if_break_edge = Some(ifst.get_successor_edges(StatEdge::TYPE_BREAK)[0]);
                        dest = Self::find_if_break_edge(ifst, if_break_edge);
                    } else if !ifst.get_successor_edges(StatEdge::TYPE_REGULAR).is_empty() {
                        let adjacent_stat =
                            ifst.get_successor_edges(StatEdge::TYPE_REGULAR)[0].get_destination();

                        if adjacent_stat.has_successor(StatEdge::TYPE_BREAK) {
                            if_break_edge =
                                Some(adjacent_stat.get_successor_edges(StatEdge::TYPE_BREAK)[0]);
                            dest = Self::find_if_break_edge(ifst, if_break_edge);
                        }
                    }
                }

                if let Some(dest) = dest {
                    let new_block = BasicBlockStatement::create();
                    new_block.set_exprents(copy_exprents(&dest.get_exprents().unwrap()));

                    let oldexitedge = dest.get_first_successor();

                    if let Some(if_break_edge) = if_break_edge {
                        let ifst_in_seqence = ifst.get_parent().is_some_and(|p| p.is_sequence());
                        if ifst_in_seqence {
                            if let Some(source_exprents) = if_break_edge.get_source().get_exprents()
                            {
                                new_block.get_exprents().unwrap().add_all_at(
                                    0,
                                    DecHelper::copy_exprent_list(&source_exprents.to_vec()),
                                );
                            }
                        }

                        let mut switch_stat: Option<SwitchStatement> = None;
                        if let Some(s) = ifst.get_parent().and_then(|p| p.try_as_switch()) {
                            switch_stat = Some(s);
                        } else if ifst_in_seqence {
                            if let Some(s) = ifst
                                .get_parent()
                                .unwrap()
                                .get_parent()
                                .and_then(|p| p.try_as_switch())
                            {
                                switch_stat = Some(s);
                            }
                        }

                        if let Some(switch_stat) = switch_stat {
                            if let Some(func_expr) = ifst
                                .get_headexprent()
                                .get_condition()
                                .try_as_function()
                                .filter(|f| f.get_func_type() == FunctionType::BoolNot)
                            {
                                ifst.get_headexprent()
                                    .set_condition(func_expr.get_lst_operands().get(0));

                                if let Some(default_edge) = switch_stat.get_default_edge() {
                                    let default_case = default_edge
                                        .get_destination()
                                        .get_first_successor()
                                        .get_destination();
                                    if ifedge.get_destination() == default_case {
                                        if ifst_in_seqence {
                                            ifst.get_successor_edges(StatEdge::TYPE_REGULAR)[0]
                                                .change_source(ifst.get_parent().unwrap());

                                            if_break_edge.get_source().replace_with_empty();
                                            if_break_edge.get_destination().replace_with_empty();
                                        } else {
                                            ifst.remove_successor(if_break_edge);
                                        }

                                        ifst.add_successor(StatEdge::new_closure(
                                            StatEdge::TYPE_BREAK,
                                            ifst.0,
                                            default_case,
                                            ifedge.closure(),
                                        ));
                                    }
                                }
                            }
                        }
                    }

                    let newedge = StatEdge::new(
                        StatEdge::TYPE_REGULAR,
                        ifst.get_first().unwrap(),
                        new_block.0,
                    );
                    ifst.get_first().unwrap().remove_successor(ifedge);
                    ifst.get_first().unwrap().add_successor(newedge);
                    ifst.set_if_edge(Some(newedge));
                    ifst.set_ifstat(Some(new_block.0));
                    let new_block_id = new_block.id();
                    ifst.get_stats()
                        .borrow_mut()
                        .add_with_key(new_block.0, new_block_id);
                    new_block.set_parent(Some(ifst.0));

                    let newexitedge = StatEdge::new(
                        StatEdge::TYPE_BREAK,
                        new_block.0,
                        oldexitedge.get_destination(),
                    );
                    new_block.add_successor(newexitedge);
                    oldexitedge
                        .closure()
                        .expect("closure")
                        .add_labeled_edge(newexitedge);

                    ret = 1;
                }
            }
        }

        if stat.get_all_successor_edges().len() == 1
            && stat.get_all_successor_edges()[0].get_type() == StatEdge::TYPE_BREAK
            && stat.get_label_edges().borrow().is_empty()
        {
            let parent = stat.get_parent().expect("parent");
            if Some(stat) == parent.get_first() && (parent.is_if() || parent.is_switch()) {
                return ret;
            }

            let destedge = stat.get_all_successor_edges()[0];
            let dest = match Self::is_exit_edge(destedge) {
                Some(dest) => dest,
                None => return ret,
            };

            stat.remove_successor(destedge);

            let bstat = BasicBlockStatement::create();
            bstat.set_exprents(copy_exprents(&dest.get_exprents().unwrap()));

            let oldexitedge = dest.get_all_successor_edges()[0];
            let newexitedge =
                StatEdge::new(StatEdge::TYPE_BREAK, bstat.0, oldexitedge.get_destination());
            bstat.add_successor(newexitedge);
            oldexitedge
                .closure()
                .expect("closure")
                .add_labeled_edge(newexitedge);

            let block = SequenceStatement::new(vec![stat, bstat.0]);
            block.set_all_parent();

            parent.replace_statement(stat, block.0);
            for prededge in block.get_predecessor_edges(StatEdge::TYPE_CONTINUE) {
                block.remove_predecessor(prededge);
                prededge
                    .get_source()
                    .change_edge_node(EdgeDirection::Forward, prededge, stat);
                stat.add_predecessor(prededge);
                stat.add_labeled_edge(prededge);
            }

            stat.add_successor(StatEdge::new(StatEdge::TYPE_REGULAR, stat, bstat.0));

            for edge in dest.get_all_predecessor_edges() {
                if edge.explicit()
                    || !stat.contains_statement_strict(edge.get_source())
                    || !MergeHelper::is_direct_path(
                        edge.get_source().get_parent().expect("parent"),
                        bstat.0,
                    )
                {
                    continue;
                }

                dest.remove_predecessor(edge);
                edge.get_source()
                    .change_edge_node(EdgeDirection::Forward, edge, bstat.0);
                bstat.add_predecessor(edge);

                if !edge
                    .closure()
                    .is_some_and(|closure| stat.contains_statement_strict(closure))
                {
                    stat.add_labeled_edge(edge);
                }
            }

            ret = 2;
        }

        ret
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

/// `DecHelper.copyExprentList(lst)` as the fresh list `setExprents` takes.
fn copy_exprents(lst: &JList<Exprent>) -> JList<Exprent> {
    JList::from(DecHelper::copy_exprent_list(&lst.to_vec()))
}
