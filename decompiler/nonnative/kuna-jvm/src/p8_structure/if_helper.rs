//! Port of `org/jetbrains/java/decompiler/modules/decompiler/IfHelper.java`.
//!
//! kuna fix `jvm_fix_if_denest_fallthrough` (`kuna_fixes`): `ifElseChainDenesting`
//! leaves an if alone when its body still reaches the statements it would move
//! into the then-branch.

use crate::assert_processor::AssertProcessor;
use crate::decompiler_context::DecompilerContext;
use crate::exprent::Exprent;
use crate::function_exprent::{FunctionExprent, FunctionType};
use crate::i_fernflower_preferences::IFernflowerPreferences;
use crate::if_node::{EdgeType, IfNode};
use crate::if_statement::IfStatement;
use crate::jthrow;
use crate::kuna_fixes::{self, KunaFix};
use crate::merge_helper::MergeHelper;
use crate::root_statement::RootStatement;
use crate::rt::JHashSet;
use crate::sequence_helper::SequenceHelper;
use crate::sequence_statement::SequenceStatement;
use crate::stat_edge::StatEdge;
use crate::statement::{Statement, StatementType};
use crate::validation_helper::ValidationHelper;

pub struct IfHelper;

impl IfHelper {
    pub fn merge_all_ifs(root: RootStatement) -> bool {
        let res = Self::merge_all_ifs_rec(root.0, &mut JHashSet::new());
        if res {
            SequenceHelper::condense_sequences(root.0);
        }
        res
    }

    fn merge_all_ifs_rec(stat: Statement, set_reordered_ifs: &mut JHashSet<i32>) -> bool {
        let mut res = false;

        if stat.get_exprents().is_none() {
            loop {
                let mut changed = false;

                let stats = stat.get_stats().borrow().to_vec();
                for st in stats {
                    res |= Self::merge_all_ifs_rec(st, set_reordered_ifs);

                    if Self::merge_ifs(st, set_reordered_ifs) {
                        changed = true;
                        break;
                    }
                }

                res |= changed;

                if !changed {
                    break;
                }
            }
        }

        res
    }

    pub fn merge_ifs(statement: Statement, set_reordered_ifs: &mut JHashSet<i32>) -> bool {
        if !statement.is_if() && !statement.is_sequence() {
            return false;
        }

        let mut res = false;

        'loop_: loop {
            let lst: Vec<Statement> = if statement.is_if() {
                vec![statement]
            } else {
                statement.get_stats().borrow().to_vec()
            };

            let stsingle = lst.len() == 1;

            for stat in lst {
                if let Some(ifstat) = stat.try_as_if() {
                    let rtnode = IfNode::build(ifstat, stsingle);

                    if Self::collapse_if_if(&rtnode) {
                        res = true;
                        ValidationHelper::validate_statement(stat.get_top_parent());
                        continue 'loop_;
                    }

                    if !set_reordered_ifs.contains(&stat.id()) {
                        if Self::collapse_if_else(&rtnode) {
                            res = true;
                            ValidationHelper::validate_statement(stat.get_top_parent());
                            continue 'loop_;
                        }

                        if Self::collapse_else(&rtnode) {
                            res = true;
                            ValidationHelper::validate_statement(stat.get_top_parent());
                            continue 'loop_;
                        }

                        if DecompilerContext::get_option(IFernflowerPreferences::TERNARY_CONDITIONS)
                            && Self::collapse_ternary(&rtnode)
                        {
                            res = true;
                            ValidationHelper::validate_statement(stat.get_top_parent());
                            continue 'loop_;
                        }

                        if Self::if_else_chain_denesting(&rtnode) {
                            res = true;
                            ValidationHelper::validate_statement(stat.get_top_parent());
                            continue 'loop_;
                        }
                    }

                    if Self::reorder_if(ifstat) {
                        res = true;
                        ValidationHelper::validate_statement(stat.get_top_parent());
                        set_reordered_ifs.add(stat.id());
                        continue 'loop_;
                    }
                }
            }

            return res;
        }
    }

    fn collapse_if_if(rtnode: &IfNode) -> bool {
        if rtnode.inner_type == Some(EdgeType::Direct) {
            let ifbranch = node(&rtnode.inner_node);
            if let Some(ifbranch_inner) = ifbranch.inner_node.as_deref() {
                if node(&rtnode.successor_node).value == node(&ifbranch.successor_node).value {
                    let ifparent = rtnode.value.as_if();
                    let ifchild = ifbranch.value.as_if();
                    let ifinner = ifbranch_inner.value;

                    if first_exprents_empty(ifchild.0)
                        && !ifchild.has_ppmm()
                        && !Self::has_assert_field(ifchild.get_headexprent().0)
                    {
                        ifparent.get_if_edge().unwrap().remove();
                        ifchild.get_first_successor().remove();
                        let ifchild_id = ifchild.id();
                        ifparent
                            .get_stats()
                            .borrow_mut()
                            .remove_with_key(&ifchild_id);

                        if ifbranch.inner_type == Some(EdgeType::Indirect) {
                            ifparent.set_ifstat(None);

                            let ifedge = ifchild.get_if_edge().unwrap();

                            ifedge.change_source(ifparent.get_first().unwrap());

                            if ifedge.closure() == Some(ifchild.0) {
                                ifedge.with_mut(|d| d.closure = None);
                            }

                            ifparent.set_if_edge(Some(ifedge));
                        } else {
                            ifchild.get_if_edge().unwrap().remove();

                            let ifedge = StatEdge::new(
                                StatEdge::TYPE_REGULAR,
                                ifparent.get_first().unwrap(),
                                ifinner,
                            );
                            ifparent.get_first().unwrap().add_successor(ifedge);
                            ifparent.set_if_edge(Some(ifedge));
                            ifparent.set_ifstat(Some(ifinner));

                            let ifinner_id = ifinner.id();
                            ifparent
                                .get_stats()
                                .borrow_mut()
                                .add_with_key(ifinner, ifinner_id);
                            ifinner.set_parent(Some(ifparent.0));

                            if ifinner.has_any_successor() {
                                let edge = ifinner.get_first_successor();
                                if edge.closure() == Some(ifchild.0) {
                                    edge.with_mut(|d| d.closure = Some(ifparent.0));
                                }
                            }
                        }

                        let statexpr = ifparent.get_headexprent();

                        let lst_operands: Vec<Exprent> = vec![
                            statexpr.get_condition(),
                            ifchild.get_headexprent().get_condition(),
                        ];

                        statexpr.set_condition(
                            FunctionExprent::new(FunctionType::BooleanAnd, lst_operands, None).0,
                        );
                        statexpr.add_bytecode_offsets(
                            ifchild.get_headexprent().get_bytecode().as_ref(),
                        );

                        return true;
                    }
                }
            }
        }

        false
    }

    fn collapse_if_else(rtnode: &IfNode) -> bool {
        if rtnode.inner_type == Some(EdgeType::Direct) {
            let ifbranch = node(&rtnode.inner_node);
            if let Some(ifbranch_inner) = ifbranch.inner_node.as_deref() {
                if node(&rtnode.successor_node).value == ifbranch_inner.value {
                    let ifparent = rtnode.value.as_if();
                    let ifchild = ifbranch.value.as_if();

                    if first_exprents_empty(ifchild.0) {
                        ifparent.get_if_edge().unwrap().remove();
                        ifchild.get_if_edge().unwrap().remove();
                        let ifchild_id = ifchild.id();
                        ifparent
                            .get_stats()
                            .borrow_mut()
                            .remove_with_key(&ifchild_id);

                        ifparent.set_ifstat(None);

                        let ifedge = ifchild.get_first_successor();
                        ifedge.change_source(ifparent.get_first().unwrap());
                        ifparent.set_if_edge(Some(ifedge));

                        let statexpr = ifparent.get_headexprent();

                        let lst_operands: Vec<Exprent> = vec![
                            statexpr.get_condition(),
                            FunctionExprent::new_single(
                                FunctionType::BoolNot,
                                ifchild.get_headexprent().get_condition(),
                                None,
                            )
                            .0,
                        ];
                        statexpr.set_condition(
                            FunctionExprent::new(FunctionType::BooleanAnd, lst_operands, None).0,
                        );
                        statexpr.add_bytecode_offsets(
                            ifchild.get_headexprent().get_bytecode().as_ref(),
                        );

                        return true;
                    }
                }
            }
        }

        false
    }

    fn collapse_else(rtnode: &IfNode) -> bool {
        if rtnode.successor_type == Some(EdgeType::Direct) {
            let elsebranch = node(&rtnode.successor_node);
            if let Some(elsebranch_inner) = elsebranch.inner_node.as_deref() {
                let path =
                    if node(&elsebranch.successor_node).value == node(&rtnode.inner_node).value {
                        2
                    } else if elsebranch_inner.value == node(&rtnode.inner_node).value {
                        1
                    } else {
                        0
                    };

                if path > 0 {
                    let firstif = rtnode.value.as_if();
                    let secondif = elsebranch.value.as_if();
                    let parent = firstif.get_parent();

                    if first_exprents_empty(secondif.0) {
                        firstif.get_if_edge().unwrap().remove();

                        firstif.remove_all_successors(secondif.0);

                        for edge in firstif.get_all_predecessor_edges() {
                            if !firstif.contains_statement_strict(edge.get_source()) {
                                edge.change_destination(secondif.0);
                            }
                        }

                        let parent = parent.unwrap();
                        let firstif_id = firstif.id();
                        parent.get_stats().borrow_mut().remove_with_key(&firstif_id);
                        if parent.get_first() == Some(firstif.0) {
                            parent.set_first(secondif.0);
                        }

                        let statexpr = secondif.get_headexprent();

                        let mut lst_operands: Vec<Exprent> = Vec::new();
                        lst_operands.push(firstif.get_headexprent().get_condition());

                        if path == 2 {
                            lst_operands[0] = FunctionExprent::new_single(
                                FunctionType::BoolNot,
                                lst_operands[0],
                                None,
                            )
                            .0;
                        }

                        lst_operands.push(statexpr.get_condition());

                        statexpr.set_condition(
                            FunctionExprent::new(
                                if path == 1 {
                                    FunctionType::BooleanOr
                                } else {
                                    FunctionType::BooleanAnd
                                },
                                lst_operands,
                                None,
                            )
                            .0,
                        );

                        if first_exprents_empty(secondif.0) && !first_exprents_empty(firstif.0) {
                            secondif.0.replace_statement(
                                secondif.get_first().unwrap(),
                                firstif.get_first().unwrap(),
                            );
                        }

                        return true;
                    }
                }
            } else if let Some(elsebranch_successor) = elsebranch.successor_node.as_deref() {
                if elsebranch_successor.value == node(&rtnode.inner_node).value {
                    let firstif = rtnode.value.as_if();
                    let second = elsebranch.value;

                    firstif.remove_all_successors(second);

                    for edge in second.get_all_successor_edges() {
                        edge.change_source(firstif.0);
                    }

                    let ifedge = firstif.get_if_edge().unwrap();
                    ifedge.remove();

                    second.add_successor(StatEdge::new_closure(
                        ifedge.get_type(),
                        second,
                        ifedge.get_destination(),
                        ifedge.closure(),
                    ));

                    let newifedge =
                        StatEdge::new(StatEdge::TYPE_REGULAR, firstif.get_first().unwrap(), second);
                    firstif.get_first().unwrap().add_successor(newifedge);
                    firstif.set_if_edge(Some(newifedge));
                    firstif.set_ifstat(Some(second));

                    let second_id = second.id();
                    firstif
                        .get_stats()
                        .borrow_mut()
                        .add_with_key(second, second_id);
                    second.set_parent(Some(firstif.0));

                    firstif
                        .get_parent()
                        .unwrap()
                        .get_stats()
                        .borrow_mut()
                        .remove_with_key(&second_id);

                    let statexpr = firstif.get_headexprent();
                    statexpr.set_condition(
                        FunctionExprent::new_single(
                            FunctionType::BoolNot,
                            statexpr.get_condition(),
                            None,
                        )
                        .0,
                    );

                    return true;
                }
            }
        }
        false
    }

    fn collapse_ternary(rtnode: &IfNode) -> bool {
        if rtnode.inner_type == Some(EdgeType::Direct)
            && rtnode.successor_type == Some(EdgeType::Else)
        {
            let if_branch = node(&rtnode.inner_node);
            let else_branch = node(&rtnode.successor_node);

            if if_branch.inner_type == Some(EdgeType::Indirect)
                && if_branch.successor_type == Some(EdgeType::Indirect)
                && else_branch.inner_type == Some(EdgeType::Indirect)
                && else_branch.successor_type == Some(EdgeType::Indirect)
                && first_exprents_empty(if_branch.value)
                && first_exprents_empty(else_branch.value)
            {
                let inverted;
                if node(&if_branch.inner_node).value == node(&else_branch.inner_node).value
                    && node(&if_branch.successor_node).value
                        == node(&else_branch.successor_node).value
                {
                    inverted = false;
                } else if node(&if_branch.inner_node).value
                    == node(&else_branch.successor_node).value
                    && node(&if_branch.successor_node).value == node(&else_branch.inner_node).value
                {
                    inverted = true;
                } else {
                    return false;
                }

                let main_if = rtnode.value.as_if();
                let first_if = if_branch.value.as_if();
                let second_if = else_branch.value.as_if();

                let first_if_id = first_if.id();
                main_if
                    .get_stats()
                    .borrow_mut()
                    .remove_with_key(&first_if_id);
                main_if.get_if_edge().unwrap().remove();
                main_if.set_ifstat(None);

                let second_if_id = second_if.id();
                main_if
                    .get_stats()
                    .borrow_mut()
                    .remove_with_key(&second_if_id);
                main_if.get_else_edge().unwrap().remove();
                main_if.set_elsestat(None);

                first_if.get_if_edge().unwrap().remove();
                first_if.get_first_successor().remove();

                main_if.set_if_edge(second_if.get_if_edge());
                main_if
                    .get_if_edge()
                    .unwrap()
                    .change_source(main_if.get_first().unwrap());

                if main_if.has_any_successor() {
                    main_if.get_first_successor().remove();
                }

                if second_if.get_first_successor().closure() == Some(second_if.0) {
                    second_if.get_first_successor().remove_closure();
                }

                second_if.get_first_successor().change_source(main_if.0);
                if main_if.get_first_successor().closure() == Some(main_if.0) {
                    if main_if.get_first_successor().get_source() != main_if.0 {
                        main_if.get_first_successor().remove_closure();
                    }

                    main_if
                        .get_first_successor()
                        .change_type(StatEdge::TYPE_REGULAR);
                }

                main_if.d_mut(|d| d.iftype = IfStatement::IFTYPE_IF);
                main_if.set_else_edge(None);

                let statexpr = main_if.get_headexprent();

                let mut lst_operands: Vec<Exprent> = Vec::new();
                lst_operands.push(main_if.get_headexprent().get_condition());
                lst_operands.push(first_if.get_headexprent().get_condition());

                if inverted {
                    lst_operands[1] =
                        FunctionExprent::new_single(FunctionType::BoolNot, lst_operands[1], None).0;
                }

                lst_operands.push(second_if.get_headexprent().get_condition());

                statexpr.set_condition(
                    FunctionExprent::new(FunctionType::Ternary, lst_operands, None).0,
                );
                return true;
            }
        }

        false
    }

    fn if_else_chain_denesting(rtnode: &IfNode) -> bool {
        if rtnode.inner_type == Some(EdgeType::Direct)
            && rtnode.successor_type != Some(EdgeType::Else)
        {
            let outer_if = rtnode.value.as_if();
            if let Some(parent) = outer_if.get_parent().and_then(|p| p.try_as_sequence()) {
                let nested_stat = node(&rtnode.inner_node).value;

                let ifdirect = Self::has_direct_end_edge(nested_stat, parent.0);
                let parent_last = parent.get_stats().borrow().get_last();
                let elsedirect = Self::has_direct_end_edge(parent_last, parent.0);
                if ifdirect && elsedirect {
                    let nested_if = Self::if_or_leading_if(nested_stat);
                    if let Some(nested_if) = nested_if {
                        if first_exprents_empty(nested_if.0)
                            && nested_if.get_ifstat().is_some()
                            && Self::has_direct_end_edge(nested_if.get_ifstat().unwrap(), parent.0)
                        {
                            let successors = outer_if.get_all_successor_edges();
                            let next_stat = if !successors.is_empty()
                                && successors[0].get_type() == StatEdge::TYPE_REGULAR
                            {
                                Some(successors[0].get_destination())
                            } else {
                                None
                            };
                            let next_if_stat = next_stat.and_then(Self::if_or_leading_if);
                            if next_stat.is_some()
                                && next_if_stat.is_none_or(|nis| !first_exprents_empty(nis.0))
                            {
                                if kuna_fixes::enabled(KunaFix::IfDenestFallthrough)
                                    && Self::reaches_following(outer_if, parent)
                                {
                                    return false;
                                }
                                let condition_exprent = outer_if.get_headexprent();
                                condition_exprent.set_condition(
                                    FunctionExprent::new_single(
                                        FunctionType::BoolNot,
                                        condition_exprent.get_condition(),
                                        None,
                                    )
                                    .0,
                                );
                                Self::swap_branches(outer_if, false, parent);
                                return true;
                            }
                        }
                    }
                }
            }
        }

        false
    }

    /// Whether an edge from inside `outer_if` reaches a statement after it in
    /// `parent`: the statements `swap_branches` moves into the then-branch.
    fn reaches_following(outer_if: IfStatement, parent: SequenceStatement) -> bool {
        let stats = parent.get_stats().borrow().to_vec();
        let Some(index) = stats.iter().position(|st| *st == outer_if.0) else {
            return false;
        };
        stats[index + 1..]
            .iter()
            .any(|st| Self::exists_path(outer_if.0, *st))
    }

    /// `stat` itself when it is an if, or its first statement when `stat` is a
    /// sequence starting with an if (the shape both ternaries in
    /// `ifElseChainDenesting` test).
    fn if_or_leading_if(stat: Statement) -> Option<IfStatement> {
        if let Some(ifstat) = stat.try_as_if() {
            Some(ifstat)
        } else if stat.is_sequence() {
            stat.get_first().and_then(|f| f.try_as_if())
        } else {
            None
        }
    }

    fn reorder_if(ifstat: IfStatement) -> bool {
        if ifstat.iftype() == IfStatement::IFTYPE_IFELSE {
            return false;
        }

        let ifdirect;
        let mut noifstat = false;
        let mut ifdirectpath = false;
        let mut elsedirectpath = false;

        let parent = ifstat.get_parent();
        let parent_is_seq = parent.is_some_and(|p| p.is_sequence());
        let from = if parent_is_seq {
            parent.unwrap()
        } else {
            ifstat.0
        };

        let next = Self::get_next_statement(from);

        match ifstat.get_ifstat() {
            None => {
                noifstat = true;

                ifdirect = ifstat.get_if_edge().unwrap().get_type() == StatEdge::TYPE_FINALLYEXIT
                    || MergeHelper::is_direct_path(
                        from,
                        ifstat.get_if_edge().unwrap().get_destination(),
                    );
            }
            Some(if_body) => {
                let lst_succs = if_body.get_all_successor_edges();
                ifdirect = !lst_succs.is_empty()
                    && lst_succs[0].get_type() == StatEdge::TYPE_FINALLYEXIT
                    || Self::has_direct_end_edge(if_body, from);
            }
        }

        let last = if parent_is_seq {
            parent.unwrap().get_stats().borrow().get_last()
        } else {
            ifstat.0
        };
        let noelsestat = last == ifstat.0;

        let last_succs = last.get_all_successor_edges();
        let elsedirect = !last_succs.is_empty()
            && last_succs[0].get_type() == StatEdge::TYPE_FINALLYEXIT
            || Self::has_direct_end_edge(last, from);

        let successors = ifstat.get_all_successor_edges();

        if successors.is_empty() {
            jthrow!(IllegalStateException; "If statement {} has no successors!", ifstat.0);
        }

        if !noelsestat && Self::exists_path(ifstat.0, successors[0].get_destination()) {
            return false;
        }

        if !ifdirect && !noifstat {
            ifdirectpath = Self::exists_path(ifstat.0, next);
        }

        if !elsedirect && !noelsestat {
            let sequence = parent.unwrap().as_sequence();

            let mut i = sequence.get_stats().borrow().len() as i32 - 1;
            while i >= 0 {
                let sttemp = sequence.get_stats().borrow().get(i as usize);
                if sttemp == ifstat.0 {
                    break;
                } else if Self::exists_path(sttemp, next) {
                    elsedirectpath = true;
                    break;
                }
                i -= 1;
            }
        }

        if (ifdirect || ifdirectpath) && (elsedirect || elsedirectpath) && !noifstat && !noelsestat
        {
            let sequence = parent.unwrap().as_sequence();

            let mut lst: Vec<Statement> = Vec::new();
            let mut i = sequence.get_stats().borrow().len() as i32 - 1;
            while i >= 0 {
                let sttemp = sequence.get_stats().borrow().get(i as usize);
                if sttemp == ifstat.0 {
                    break;
                } else {
                    lst.insert(0, sttemp);
                }
                i -= 1;
            }

            let stelse = if lst.len() == 1 {
                lst[0]
            } else {
                let seq = SequenceStatement::new(lst.clone());
                seq.set_all_parent();
                seq.0
            };

            ifstat.remove_successor(ifstat.get_first_successor());
            for st in &lst {
                let st_id = st.id();
                sequence.get_stats().borrow_mut().remove_with_key(&st_id);
            }

            let elseedge =
                StatEdge::new(StatEdge::TYPE_REGULAR, ifstat.get_first().unwrap(), stelse);
            ifstat.get_first().unwrap().add_successor(elseedge);
            ifstat.set_elsestat(Some(stelse));
            ifstat.set_else_edge(Some(elseedge));

            let stelse_id = stelse.id();
            ifstat
                .get_stats()
                .borrow_mut()
                .add_with_key(stelse, stelse_id);
            stelse.set_parent(Some(ifstat.0));

            ifstat.d_mut(|d| d.iftype = IfStatement::IFTYPE_IFELSE);
        } else if ifdirect
            && (!elsedirect || (noifstat && !noelsestat))
            && !ifstat.get_all_successor_edges().is_empty()
        {
            let statexpr = ifstat.get_headexprent();
            statexpr.set_condition(
                FunctionExprent::new_single(FunctionType::BoolNot, statexpr.get_condition(), None)
                    .0,
            );

            if noelsestat {
                let ifedge = ifstat.get_if_edge();
                let elseedge = ifstat.get_first_successor();

                if noifstat {
                    remove_successor_nullable(ifstat.get_first().unwrap(), ifedge);
                    ifstat.remove_successor(elseedge);

                    let ifedge = ifedge.expect("if edge");
                    ifedge.set_source(ifstat.0);
                    elseedge.set_source(ifstat.get_first().unwrap());

                    ifstat.add_successor(ifedge);
                    ifstat.get_first().unwrap().add_successor(elseedge);

                    ifstat.set_if_edge(Some(elseedge));
                } else {
                    let ifbranch = ifstat.get_ifstat().unwrap();
                    let newseq = SequenceStatement::new(vec![ifstat.0, ifbranch]);

                    remove_successor_nullable(ifstat.get_first().unwrap(), ifedge);
                    let ifbranch_id = ifbranch.id();
                    ifstat
                        .get_stats()
                        .borrow_mut()
                        .remove_with_key(&ifbranch_id);
                    ifstat.set_ifstat(None);

                    ifstat.remove_successor(elseedge);
                    elseedge.set_source(ifstat.get_first().unwrap());
                    ifstat.get_first().unwrap().add_successor(elseedge);

                    ifstat.set_if_edge(Some(elseedge));

                    ifstat
                        .get_parent()
                        .unwrap()
                        .replace_statement(ifstat.0, newseq.0);
                    newseq.set_all_parent();

                    ifstat.add_successor(StatEdge::new(StatEdge::TYPE_REGULAR, ifstat.0, ifbranch));
                }
            } else {
                Self::swap_branches(ifstat, noifstat, parent.unwrap().as_sequence());
            }
        } else {
            return false;
        }

        true
    }

    fn swap_branches(ifstat: IfStatement, noifstat: bool, parent: SequenceStatement) {
        ValidationHelper::validate_true(
            ifstat.iftype() == IfStatement::IFTYPE_IF,
            "This method is meant for swapping the branches of non if-else IfStatements",
        );
        let mut lst: Vec<Statement> = Vec::new();
        let mut i = parent.get_stats().borrow().len() as i32 - 1;
        while i >= 0 {
            let sttemp = parent.get_stats().borrow().get(i as usize);
            if sttemp == ifstat.0 {
                break;
            } else {
                lst.insert(0, sttemp);
            }
            i -= 1;
        }

        let stelse = if lst.len() == 1 {
            lst[0]
        } else {
            let seq = SequenceStatement::new(lst.clone());
            seq.set_all_parent();
            seq.0
        };

        ifstat.remove_successor(ifstat.get_first_successor());
        for st in &lst {
            let st_id = st.id();
            parent.get_stats().borrow_mut().remove_with_key(&st_id);
        }

        if noifstat {
            let ifedge = ifstat.get_if_edge();

            remove_successor_nullable(ifstat.get_first().unwrap(), ifedge);
            let ifedge = ifedge.expect("if edge");
            ifedge.set_source(ifstat.0);
            ifstat.add_successor(ifedge);
        } else {
            let ifbranch = ifstat.get_ifstat().unwrap();

            remove_successor_nullable(ifstat.get_first().unwrap(), ifstat.get_if_edge());
            let ifbranch_id = ifbranch.id();
            ifstat
                .get_stats()
                .borrow_mut()
                .remove_with_key(&ifbranch_id);

            ifstat.add_successor(StatEdge::new(StatEdge::TYPE_REGULAR, ifstat.0, ifbranch));

            parent
                .get_stats()
                .borrow_mut()
                .add_with_key(ifbranch, ifbranch_id);
            ifbranch.set_parent(Some(parent.0));
        }

        let newifedge = StatEdge::new(StatEdge::TYPE_REGULAR, ifstat.get_first().unwrap(), stelse);
        ifstat.get_first().unwrap().add_successor(newifedge);
        ifstat.set_ifstat(Some(stelse));
        ifstat.set_if_edge(Some(newifedge));

        let stelse_id = stelse.id();
        ifstat
            .get_stats()
            .borrow_mut()
            .add_with_key(stelse, stelse_id);
        stelse.set_parent(Some(ifstat.0));
    }

    fn has_direct_end_edge(stat: Statement, from: Statement) -> bool {
        for edge in stat.get_all_successor_edges() {
            if MergeHelper::is_direct_path(from, edge.get_destination()) {
                return true;
            }
        }

        if stat.get_exprents().is_none() {
            match stat.type_() {
                StatementType::Sequence => {
                    let last = stat.get_stats().borrow().get_last();
                    return Self::has_direct_end_edge(last, from);
                }
                StatementType::CatchAll | StatementType::TryCatch => {
                    let stats = stat.get_stats().borrow().to_vec();
                    for st in stats {
                        if Self::has_direct_end_edge(st, from) {
                            return true;
                        }
                    }
                }
                StatementType::If => {
                    let ifstat = stat.as_if();
                    if ifstat.iftype() == IfStatement::IFTYPE_IFELSE {
                        return Self::has_direct_end_edge(ifstat.get_ifstat().unwrap(), from)
                            || Self::has_direct_end_edge(ifstat.get_elsestat().unwrap(), from);
                    }
                }
                StatementType::Synchronized => {
                    let body = stat.get_stats().borrow().get(1);
                    return Self::has_direct_end_edge(body, from);
                }
                StatementType::Switch => {
                    let stats = stat.get_stats().borrow().to_vec();
                    for st in stats {
                        if Self::has_direct_end_edge(st, from) {
                            return true;
                        }
                    }
                }
                _ => {}
            }
        }

        false
    }

    fn get_next_statement(stat: Statement) -> Statement {
        let parent = stat.get_parent().unwrap();
        match parent.type_() {
            StatementType::Root => return parent.as_root().get_dummy_exit().0,
            StatementType::Do => return parent,
            StatementType::Sequence => {
                let stats = parent.as_sequence().get_stats().borrow().to_vec();
                if stats[stats.len() - 1] != stat {
                    for i in (0..stats.len()).rev() {
                        if stats[i] == stat {
                            return stats[i + 1];
                        }
                    }
                }
            }
            _ => {}
        }

        Self::get_next_statement(parent)
    }

    fn exists_path(from: Statement, to: Statement) -> bool {
        for edge in to.get_all_predecessor_edges() {
            if from.contains_statement_strict(edge.get_source()) {
                return true;
            }
        }

        false
    }

    pub fn prettify_ifs(stat: Statement) -> bool {
        let mut changed = false;
        let stats = stat.get_stats().borrow().to_vec();
        for st in stats {
            if st.is_switch() {
                continue;
            }

            Self::prettify_ifs(st);

            if let Some(if_st) = st.try_as_if() {
                changed |= Self::prettify_if(if_st);
            }
        }

        changed
    }

    fn prettify_if(if_st: IfStatement) -> bool {
        if if_st.iftype() == IfStatement::IFTYPE_IFELSE && !if_st.is_pattern_matched() {
            let if_stat = if_st.get_ifstat();
            let else_stat = if_st.get_elsestat();
            if let Some(if_stat) = if_stat.filter(|&s| Self::is_exit(s)) {
                let force = if_stat.get_exprents().unwrap().len() == 1
                    && else_stat
                        .unwrap()
                        .get_exprents()
                        .is_some_and(|e| e.len() > 1);
                if !force {
                    if Self::is_exit(else_stat.unwrap()) {
                        return false;
                    }

                    if let Some(else_if) = else_stat.and_then(|s| s.try_as_if()) {
                        if else_if.get_ifstat().is_some_and(Self::is_exit) {
                            return false;
                        }
                    }
                }

                if_st.d_mut(|d| d.iftype = IfStatement::IFTYPE_IF);
                let else_stat = else_stat.unwrap();
                let seq = SequenceStatement::new_varargs(&[if_st.0, else_stat]);

                let else_edge = if_st.get_else_edge();
                let else_stat_id = else_stat.id();
                if_st
                    .get_stats()
                    .borrow_mut()
                    .remove_with_key(&else_stat_id);

                if_st.set_elsestat(None);
                if_st.set_else_edge(None);
                if_st.replace_with(seq.0);
                else_edge.unwrap().change_source(if_st.0);
                seq.set_all_parent();

                return true;
            }
        }

        false
    }

    fn is_exit(stat: Statement) -> bool {
        if !stat.has_any_direct_successor() {
            return false;
        }

        let suc = stat.get_first_direct_successor();

        if !(suc.get_type() == StatEdge::TYPE_BREAK && suc.get_destination().is_dummy_exit()) {
            return false;
        }

        match stat.try_as_basic_block() {
            Some(bb) => {
                let exprents = bb.get_exprents().unwrap();
                !exprents.is_empty() && exprents.get(exprents.len() - 1).is_exit()
            }
            None => false,
        }
    }

    fn has_assert_field(expr: Exprent) -> bool {
        let wrapper = AssertProcessor::get_holding_class();
        let prop = wrapper.get_method_properties("<clinit>", "()V");

        if let Some(fd) = prop.and_then(|p| p.assert_field) {
            for ex in expr.get_all_exprents_rec_self(true, true) {
                if let Some(field) = ex.try_as_field() {
                    if field.get_name() == fd.get_name()
                        && field.get_classname() == wrapper.get_class_struct().qualified_name()
                    {
                        return true;
                    }
                }
            }
        }

        false
    }
}

/// A child slot of an `IfNode` that the Java dereferences unconditionally.
fn node(slot: &Option<Box<IfNode>>) -> &IfNode {
    slot.as_deref().expect("IfNode child")
}

/// `stat.getFirst().getExprents().isEmpty()`.
fn first_exprents_empty(stat: Statement) -> bool {
    stat.get_first()
        .expect("first")
        .get_exprents()
        .expect("exprents")
        .is_empty()
}

/// `stat.removeSuccessor(edge)`, which Java tolerates for a `null` edge.
fn remove_successor_nullable(stat: Statement, edge: Option<StatEdge>) {
    if let Some(edge) = edge {
        stat.remove_successor(edge);
    }
}
