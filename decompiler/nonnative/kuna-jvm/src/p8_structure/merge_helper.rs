//! Port of `org/jetbrains/java/decompiler/modules/decompiler/MergeHelper.java`.
//!
//! kuna fix `statement_as_expression` (`kuna_fixes`): in `match_for`,
//! a loop whose last exprent is statement-only (an `assert`) is not made a
//! `for` loop with it as the update, where Vineflower excludes only an exit.
//!
//! kuna option `jvm_source_locals` (`kuna_source_locals`): `match_for_each`
//! does not turn a loop whose iterator or index is a local of the source into
//! a for-each, nor fold a local of the source into the for-each's iterable.
//!
//! kuna option `jvm_javac_layout` (`kuna_javac_layout`): `condense_loop`
//! leaves a loop whose body has more than its first `if` before the
//! statements it would move out (that `if` becomes the whole body, so the
//! others would be lost): javac's layout keeps `if (c) { ..; continue; }`
//! statements that Vineflower nests in else-branches.
//! In `match_for` the option decides a `for` header from the
//! LocalVariableTable and the LineNumberTable (`kuna_javac_layout::for_header`).

use std::collections::VecDeque;

use crate::assignment_exprent::AssignmentExprent;
use crate::basic_block_statement::BasicBlockStatement;
use crate::decompiler_context::DecompilerContext;
use crate::direct_edge_type::DirectEdgeType;
use crate::direct_node::DirectNode;
use crate::do_statement::{DoStatement, Type as LoopType};
use crate::exprent::Exprent;
use crate::flatten_statements_helper::FlattenStatementsHelper;
use crate::function_exprent::FunctionType;
use crate::if_statement::IfStatement;
use crate::invocation_exprent::InvocationExprent;
use crate::kuna_fix_statement_as_expression;
use crate::kuna_fixes::{self, KunaFix};
use crate::label_helper::LabelHelper;
use crate::p6_variables::kuna_source_locals;
use crate::p8_structure::kuna_javac_layout;
use crate::root_statement::RootStatement;
use crate::rt::JHashSet;
use crate::s_forms_constructor::SFormsConstructor;
use crate::sequence_helper::SequenceHelper;
use crate::sequence_statement::SequenceStatement;
use crate::stat_edge::StatEdge;
use crate::statement::{EdgeDirection, Statement, StatementType};
use crate::try_with_resources_processor::TryWithResourcesProcessor;
use crate::validation_helper::ValidationHelper;
use crate::var_exprent::VarExprent;

pub struct MergeHelper;

/// Java `source.getBytecodeRange(target.bytecode)`: the range is OR-ed into
/// `target`'s own (live) offset set, and nothing happens while it is `null`.
fn measure_into(source: Exprent, target: Exprent) {
    let mut values = target.get_bytecode();
    if values.is_some() {
        source.get_bytecode_range(values.as_mut());
        target.set_bytecode(values);
    }
}

/// `DecompilerContext.getStructContext().instanceOf(valclass, refclass)` with
/// the Java `null` guard on `valclass`.
fn struct_instance_of(valclass: Option<&str>, refclass: &str) -> bool {
    match valclass {
        Some(valclass) => DecompilerContext::get_struct_context().instance_of(valclass, refclass),
        None => false,
    }
}

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
        let mut last = stat.get_first();
        while let Some(seq) = last.filter(|l| l.is_sequence()) {
            last = Some(seq.get_stats().borrow().get_last());
        }

        let Some(lastif) = last.and_then(|l| l.try_as_if()) else {
            return;
        };
        if lastif.iftype() != IfStatement::IFTYPE_IF || lastif.get_ifstat().is_some() {
            return;
        }

        let ifedge = lastif.get_if_edge();
        let elseedge = lastif.get_first_successor();
        let ifedge = ifedge.expect("IfStatement.ifedge");

        let (if_type, else_type) = (ifedge.get_type(), elseedge.get_type());
        let matched = (if_type == StatEdge::TYPE_BREAK
            && else_type == StatEdge::TYPE_CONTINUE
            && elseedge.closure() == Some(stat.0)
            && Self::is_direct_path(stat.0, ifedge.get_destination()))
            || (if_type == StatEdge::TYPE_CONTINUE
                && else_type == StatEdge::TYPE_BREAK
                && ifedge.closure() == Some(stat.0)
                && Self::is_direct_path(stat.0, elseedge.get_destination()))
            || (if_type == StatEdge::TYPE_CONTINUE
                && else_type == StatEdge::TYPE_CONTINUE
                && elseedge.closure() == Some(stat.0)
                && Self::is_last_in_loop(stat.0)
                && Self::is_direct_path(stat.0, ifedge.get_destination()));
        if !matched {
            return;
        }

        let mut set = stat.get_neighbours_set(StatEdge::TYPE_CONTINUE, EdgeDirection::Backward);
        set.remove(&lastif.0);

        if !set.is_empty() {
            return;
        }

        stat.set_looptype(LoopType::DoWhile);

        let ifexpr = lastif.get_headexprent().copy().as_if();
        if elseedge.get_type() == StatEdge::TYPE_CONTINUE && elseedge.closure() == Some(stat.0) {
            ifexpr.negate_if();
        }

        if let Some(condition) = stat.get_condition_exprent() {
            ifexpr
                .get_condition()
                .add_bytecode_offsets(condition.get_bytecode().as_ref());
        }
        ifexpr
            .get_condition()
            .add_bytecode_offsets(lastif.get_headexprent().get_bytecode().as_ref());

        stat.set_condition_exprent(ifexpr.get_condition());
        lastif.remove_successor(elseedge);

        let lastif_first = lastif.get_first().expect("IfStatement.first");
        if lastif_first
            .get_exprents()
            .expect("if head exprents")
            .is_empty()
        {
            lastif_first.remove_successor(ifedge);

            Self::remove_last_empty_statement(stat, lastif.0);
        } else if stat.get_stats().borrow().len() == 1
            && ifedge.get_type() == StatEdge::TYPE_CONTINUE
        {
            lastif.replace_with(lastif_first);
        } else {
            lastif.set_exprents(lastif_first.get_exprents().expect("if head exprents"));

            let newedge = StatEdge::new(StatEdge::TYPE_CONTINUE, lastif.0, stat.0);
            lastif.add_successor(newedge);
            stat.add_labeled_edge(newedge);
        }

        if stat.get_all_successor_edges().is_empty() {
            let edge = if elseedge.get_type() == StatEdge::TYPE_CONTINUE {
                ifedge
            } else {
                elseedge
            };

            edge.set_source(stat.0);
            if edge.closure() == Some(stat.0) {
                let parent = stat.get_parent();
                edge.with_mut(|d| d.closure = parent);
            }
            stat.add_successor(edge);
        }
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
        let mut first = stat.get_first();
        while let Some(seq) = first.filter(|f| f.is_sequence()) {
            first = seq.get_first();
        }

        let Some(first) = first else {
            return false;
        };
        let Some(firstif) = first.try_as_if() else {
            return false;
        };

        if !firstif
            .get_first()
            .expect("IfStatement.first")
            .get_exprents()
            .expect("if head exprents")
            .is_empty()
        {
            return false;
        }

        if firstif.iftype() != IfStatement::IFTYPE_IF {
            return false;
        }

        if firstif.get_ifstat().is_none() {
            let ifedge = firstif.get_if_edge().expect("IfStatement.ifedge");

            if Self::is_direct_path(stat.0, ifedge.get_destination())
                || Self::add_continue_or_break(stat, ifedge)
            {
                stat.set_looptype(LoopType::While);

                let ifexpr = firstif.get_headexprent().copy().as_if();
                ifexpr.negate_if();

                if let Some(condition) = stat.get_condition_exprent() {
                    ifexpr
                        .get_condition()
                        .add_bytecode_offsets(condition.get_bytecode().as_ref());
                }
                ifexpr
                    .get_condition()
                    .add_bytecode_offsets(firstif.get_headexprent().get_bytecode().as_ref());

                stat.set_condition_exprent(ifexpr.get_condition());

                firstif
                    .get_first()
                    .expect("IfStatement.first")
                    .remove_successor(ifedge);
                firstif.remove_successor(firstif.get_all_successor_edges()[0]);

                if stat.get_all_successor_edges().is_empty() {
                    ifedge.set_source(stat.0);
                    if ifedge.closure() == Some(stat.0) {
                        let parent = stat.get_parent();
                        ifedge.with_mut(|d| d.closure = parent);
                    }
                    stat.add_successor(ifedge);
                }

                if Some(firstif.0) == stat.get_first() {
                    let bstat = BasicBlockStatement::create();
                    stat.replace_statement(firstif.0, bstat.0);
                    bstat.add_successor(StatEdge::new_closure(
                        StatEdge::TYPE_CONTINUE,
                        bstat.0,
                        stat.0,
                        Some(stat.0),
                    ));
                } else {
                    let sequence = firstif.get_parent().expect("parent sequence");
                    sequence
                        .get_stats()
                        .borrow_mut()
                        .remove_with_key(&firstif.id());

                    let new_first = {
                        let stats = sequence.get_stats();
                        let stats = stats.borrow();
                        (!stats.is_empty()).then(|| stats.get(0))
                    };
                    match new_first {
                        Some(new_first) => sequence.set_first(new_first),
                        None => {
                            SequenceHelper::destroy_and_flatten_statement(sequence);
                        }
                    }
                }

                return true;
            }
        }

        let else_edge = firstif.get_first_successor();
        if Self::is_direct_path(stat.0, else_edge.get_destination()) {
            stat.set_looptype(LoopType::While);

            let ifexpr = firstif.get_headexprent().copy().as_if();
            if let Some(condition) = stat.get_condition_exprent() {
                ifexpr
                    .get_condition()
                    .add_bytecode_offsets(condition.get_bytecode().as_ref());
            }
            ifexpr
                .get_condition()
                .add_bytecode_offsets(firstif.get_headexprent().get_bytecode().as_ref());
            stat.set_condition_exprent(ifexpr.get_condition());

            let ifedge = firstif.get_if_edge().expect("IfStatement.ifedge");
            firstif
                .get_first()
                .expect("IfStatement.first")
                .remove_successor(ifedge);
            firstif.remove_successor(else_edge);

            if stat.get_all_successor_edges().is_empty() {
                else_edge.set_source(stat.0);
                if else_edge.closure() == Some(stat.0) {
                    let parent = stat.get_parent();
                    else_edge.with_mut(|d| d.closure = parent);
                }
                stat.add_successor(else_edge);
            }

            match firstif.get_ifstat() {
                None => {
                    let bstat = BasicBlockStatement::create();

                    ifedge.set_source(bstat.0);
                    bstat.add_successor(ifedge);

                    stat.replace_statement(firstif.0, bstat.0);
                }
                Some(ifstat) => {
                    first
                        .get_parent()
                        .expect("parent")
                        .replace_statement(first, ifstat);

                    for prededge in else_edge
                        .get_destination()
                        .get_predecessor_edges(StatEdge::TYPE_BREAK)
                    {
                        if prededge
                            .closure()
                            .is_some_and(|closure| stat.contains_statement_strict(closure))
                        {
                            stat.add_labeled_edge(prededge);
                        }
                    }

                    LabelHelper::low_closures(stat.0);
                }
            }

            return true;
        }

        false
    }

    pub fn is_direct_path(stat: Statement, endstat: Statement) -> bool {
        let forward_edges =
            stat.get_neighbours_set(Statement::STATEDGE_DIRECT_ALL, EdgeDirection::Forward);

        if !forward_edges.is_empty() {
            return forward_edges.contains(&endstat);
        }

        let Some(parent) = stat.get_parent() else {
            return endstat.is_dummy_exit();
        };

        match parent.type_() {
            StatementType::Root => endstat.is_dummy_exit(),
            StatementType::Do => endstat == parent,
            StatementType::Switch => {
                let swst = parent.as_switch();
                let case_statements = swst.get_case_statements();

                for i in 0..case_statements.len().saturating_sub(1) {
                    let case_statement = case_statements.get(i);

                    if case_statement == stat {
                        let mut next_case = case_statements.get(i + 1);

                        if next_case.get_exprents().is_some_and(|e| e.is_empty()) {
                            next_case = next_case.get_all_successor_edges()[0].get_destination();
                        }

                        return endstat == next_case;
                    }
                }

                Self::is_direct_path(parent, endstat)
            }
            _ => Self::is_direct_path(parent, endstat),
        }
    }

    pub fn match_for(stat: DoStatement) {
        let mut pre_data: Option<Statement> = None;

        let last_data = Self::get_last_direct_data(stat.get_first().expect("DoStatement.first"));
        let Some(last_data) = last_data else {
            return;
        };
        if last_data.get_exprents().expect("exprents").is_empty() {
            return;
        }

        let last_prot = SFormsConstructor::get_first_protected_range(last_data);
        if last_prot.is_some_and(|p| stat.contains_statement(p)) {
            return;
        }

        let lst_expr = last_data.get_exprents().expect("exprents");
        let last_do_exprent = lst_expr.get(lst_expr.len() - 1);

        let mut issingle = false;
        if lst_expr.len() == 1 && last_data.get_all_predecessor_edges().len() > 1 {
            issingle = true;
        }

        let haslast = issingle || last_do_exprent.is_assignment() || last_do_exprent.is_function();
        if !haslast {
            return;
        }

        let mut hasinit = false;

        let mut current = stat.0;
        while let Some(parent) = current.get_parent() {
            if !parent.is_sequence() {
                break;
            }

            if Some(current) == parent.get_first() {
                current = parent;
            } else {
                let pd = current.get_neighbours(StatEdge::TYPE_REGULAR, EdgeDirection::Backward)[0];
                pre_data = Some(pd);
                if !pd.is_basic_block() {
                    break;
                }

                pre_data = Self::get_last_direct_data(pd);
                if let Some(pd) = pre_data {
                    let exprents = pd.get_exprents().expect("exprents");
                    if !exprents.is_empty() {
                        let init_do_exprent = exprents.get(exprents.len() - 1);
                        if init_do_exprent.is_assignment() {
                            hasinit = true;
                        }
                    }
                }
                break;
            }
        }

        let mut header_update = false;
        if kuna_javac_layout::active() && pre_data.is_some_and(|pd| pd.get_exprents().is_some()) {
            let init = pre_data
                .filter(|_| hasinit)
                .and_then(|pd| pd.get_exprents())
                .and_then(|e| (!e.is_empty()).then(|| e.get(e.len() - 1)));
            if let Some((init_line, update_line)) =
                kuna_javac_layout::for_header(stat, init, last_do_exprent)
            {
                hasinit &= init_line;
                issingle &= update_line;
                header_update = update_line;
            }
        }

        if hasinit || issingle || header_update {
            let mut set = stat.get_neighbours_set(StatEdge::TYPE_CONTINUE, EdgeDirection::Backward);
            set.remove(&last_data);

            if !set.is_empty() {
                return;
            }

            let mut first_stat = stat.get_first();
            while let Some(fs) =
                first_stat.filter(|f| f.is_sequence() && f.get_stats().borrow().len() == 1)
            {
                let Some(fst) = fs.get_first() else {
                    break;
                };

                first_stat = Some(fst);
            }

            if let Some(fs) = first_stat.filter(|f| f.is_basic_block()) {
                if fs.get_exprents().expect("exprents").len() == 1 {
                    return;
                }
            }

            let last_exprents = last_data.get_exprents().expect("exprents");
            let last_exp = last_exprents.get(last_exprents.len() - 1);

            let statement_only = if kuna_fixes::enabled(KunaFix::StatementAsExpression) {
                kuna_fix_statement_as_expression::is_statement_only(last_exp)
            } else {
                last_exp.is_exit()
            };
            if statement_only {
                return;
            }

            let final_variables = Self::get_final_variables(stat.0, JHashSet::new());
            if final_variables
                .to_vec()
                .into_iter()
                .any(|exp| exp.is_var_referenced_exp(last_exp, &[]))
            {
                return;
            }

            for e in last_exp.get_all_exprents_rec_self(true, true) {
                let Some(var) = e.try_as_var() else {
                    continue;
                };

                if !Self::is_var_used_before(var, stat.0) {
                    return;
                }
            }

            if hasinit {
                let pd_exprents = pre_data.expect("preData").get_exprents().expect("exprents");
                if let Some(assign) = pd_exprents.get(pd_exprents.len() - 1).try_as_assignment() {
                    if let Some(var) = assign.get_left().try_as_var() {
                        if !stat
                            .get_condition_exprent()
                            .expect("DoStatement.conditionExprent")
                            .contains_var(var.get_var_version_pair())
                            && !last_exp.contains_var(var.get_var_version_pair())
                        {
                            return;
                        }
                    }
                }
            }

            stat.set_looptype(LoopType::For);
            if hasinit {
                let pd_exprents = pre_data.expect("preData").get_exprents().expect("exprents");
                let exp = pd_exprents.remove_at(pd_exprents.len() - 1);
                if let Some(init) = stat.get_init_exprent() {
                    exp.add_bytecode_offsets(init.get_bytecode().as_ref());
                }
                stat.set_init_exprent(exp);
            }
            last_data
                .get_exprents()
                .expect("exprents")
                .remove_item(&last_exp);
            if let Some(inc) = stat.get_inc_exprent() {
                last_exp.add_bytecode_offsets(inc.get_bytecode().as_ref());
            }
            stat.set_inc_exprent(last_exp);
        }

        Self::clean_empty_statements(stat, Some(last_data));
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
        if stat.get_exprents().expect("exprents").is_empty() {
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
        if Some(stat) == dostat.get_first() {
            let bstat = BasicBlockStatement::create();
            dostat.replace_statement(stat, bstat.0);
        } else {
            for edge in stat.get_all_predecessor_edges() {
                edge.get_source().change_edge_type(
                    EdgeDirection::Forward,
                    edge,
                    StatEdge::TYPE_CONTINUE,
                );

                stat.remove_predecessor(edge);

                edge.get_source()
                    .change_edge_node(EdgeDirection::Forward, edge, dostat.0);
                dostat.add_predecessor(edge);

                dostat.add_labeled_edge(edge);
            }

            stat.get_parent()
                .expect("parent")
                .get_stats()
                .borrow_mut()
                .remove_with_key(&stat.id());

            if let Some(parent) = stat.get_parent().and_then(|p| p.try_as_if()) {
                if parent.get_ifstat() == Some(stat) {
                    parent.set_ifstat(None);
                } else if parent.get_elsestat() == Some(stat) {
                    parent.set_elsestat(None);
                }
            }
        }
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
                Some(t) if !t.get_exprents().expect("exprents").is_empty() => return tmp,
                Some(_) => {}
            }
        }
        None
    }

    pub fn match_for_each(stat: DoStatement) -> bool {
        let mut first_do_exprent: Option<AssignmentExprent> = None;
        let mut init_exprents: [Option<AssignmentExprent>; 3] = [None; 3];
        let mut pre_data: Option<Statement> = None;
        let mut last_exprent: Option<Exprent> = None;

        let mut current = stat.0;
        while let Some(parent) = current.get_parent() {
            if !parent.is_sequence() {
                break;
            }

            if Some(current) == parent.get_first() {
                current = parent;
            } else {
                let pd = current.get_neighbours(StatEdge::TYPE_REGULAR, EdgeDirection::Backward)[0];
                pre_data = Self::get_last_direct_data(pd);
                if let Some(pd) = pre_data {
                    let exprents = pd.get_exprents().expect("exprents");
                    if !exprents.is_empty() {
                        let size = exprents.len();
                        for (x, slot) in init_exprents.iter_mut().enumerate() {
                            if size > x {
                                let exprent = exprents.get(size - 1 - x);
                                if let Some(assignment) = exprent.try_as_assignment() {
                                    *slot = Some(assignment);
                                }
                            }
                        }
                    }
                }
                break;
            }
        }

        let first_data = Self::get_first_direct_data(stat.get_first().expect("DoStatement.first"));
        if let Some(fd) = first_data {
            if let Some(assignment) = fd
                .get_exprents()
                .expect("exprents")
                .get(0)
                .try_as_assignment()
            {
                first_do_exprent = Some(assignment);
            }
        }
        let last_data = Self::get_last_direct_data(stat.get_first().expect("DoStatement.first"));
        if let Some(ld) = last_data {
            let exprents = ld.get_exprents().expect("exprents");
            if !exprents.is_empty() {
                last_exprent = Some(exprents.get(exprents.len() - 1));
            }
        }

        if stat.get_looptype() != LoopType::While {
            return false;
        }
        let (Some(init0), Some(first_do_exprent)) = (init_exprents[0], first_do_exprent) else {
            return false;
        };

        if Self::is_iterator_call(init0.get_right()) {
            let invc = Self::get_uncast(init0.get_right()).as_invocation();
            if invc.get_classname().contains("java/util/stream") {
                return false;
            }
            if init0
                .get_left()
                .try_as_var()
                .is_some_and(kuna_source_locals::is_named_loop_local)
            {
                return false;
            }

            match stat.get_condition_exprent() {
                Some(condition) if Self::is_has_next_call(Self::drill_nots(condition)) => {}
                _ => return false,
            }

            let ass = first_do_exprent;
            if (!Self::is_next_call(ass.get_right()) && !Self::is_next_unboxing(ass.get_right()))
                || !ass.get_left().is_var()
            {
                return false;
            }

            let mut next = Self::get_uncast(ass.get_right()).as_invocation();
            if Self::is_next_unboxing(next.0) {
                next = Self::get_uncast(next.get_instance().expect("instance")).as_invocation();
            }

            let hnext = Self::get_uncast(Self::drill_nots(
                stat.get_condition_exprent()
                    .expect("DoStatement.conditionExprent"),
            ))
            .as_invocation();
            let next_instance = next.get_instance();
            let hnext_instance = hnext.get_instance();
            if !next_instance.is_some_and(|i| i.is_var())
                || !hnext_instance.is_some_and(|i| i.is_var())
                || init0.get_left().as_var().is_var_referenced(
                    stat.0,
                    &[
                        next_instance.expect("instance").as_var(),
                        hnext_instance.expect("instance").as_var(),
                    ],
                )
            {
                return false;
            }

            if init0.get_left().as_var().get_var_version_pair()
                != next_instance
                    .expect("instance")
                    .as_var()
                    .get_var_version_pair()
            {
                return false;
            }

            let mut right = init0.get_right();
            if let Some(f_right) = right.try_as_function() {
                if f_right.get_func_type() == FunctionType::Cast {
                    right = f_right.get_lst_operands().get(0);
                }

                if right.is_invocation() {
                    return false;
                }
            }

            if Self::is_var_used_before(ass.get_left().as_var(), stat.0) {
                return false;
            }

            if Self::is_var_used_after(init0.get_left().as_var(), stat) {
                return false;
            }

            let holder = right.as_invocation();
            let Some(holder_instance) = holder.get_instance() else {
                return false;
            };

            measure_into(init0.0, holder_instance);
            measure_into(holder.0, holder_instance);
            measure_into(first_do_exprent.0, ass.get_left());
            measure_into(ass.get_right(), ass.get_left());
            if let Some(inc) = stat.get_inc_exprent() {
                measure_into(inc, holder_instance);
            }
            if let Some(init) = stat.get_init_exprent() {
                measure_into(init, ass.get_left());
            }

            stat.set_looptype(LoopType::ForEach);
            stat.set_init_exprent(ass.get_left());
            stat.set_inc_exprent(holder_instance);
            let pre_data = pre_data.expect("preData");
            pre_data
                .get_exprents()
                .expect("exprents")
                .remove_item(&init0.0);
            first_data
                .expect("firstData")
                .get_exprents()
                .expect("exprents")
                .remove_item(&first_do_exprent.0);

            if let Some(init1) = init_exprents[1] {
                if init1.get_left().is_var() && holder_instance.is_var() {
                    let copy = init1.get_left().as_var();
                    let inc = holder_instance.as_var();
                    if copy.get_index() == inc.get_index()
                        && copy.get_version() == inc.get_version()
                        && !inc.is_var_referenced(stat.get_top_parent().0, &[copy])
                        && !Self::is_next_call(init1.get_right())
                        && !kuna_source_locals::is_source_local(copy, inc)
                    {
                        pre_data
                            .get_exprents()
                            .expect("exprents")
                            .remove_item(&init1.0);
                        measure_into(init1.0, init1.get_right());
                        measure_into(
                            stat.get_inc_exprent().expect("DoStatement.incExprent"),
                            init1.get_right(),
                        );
                        stat.set_inc_exprent(init1.get_right());
                    }
                }
            }

            if let Some(type_res) = ass.0.check_expr_type_bounds() {
                if !type_res.get_lower_bounds().is_empty() {
                    let bound_type = type_res.get_lower_bounds()[0].type_.clone();
                    let var = ass.get_left().as_var();
                    var.set_bound_type(Some(bound_type));
                }
            }

            return true;
        } else if let Some(init1) = init_exprents[1] {
            if !first_do_exprent.get_right().is_array() || !first_do_exprent.get_left().is_var() {
                return false;
            }

            let Some(last_exprent) = last_exprent.filter(|e| e.is_function()) else {
                return false;
            };

            if !init0.get_right().is_const()
                || !init1.get_right().is_function()
                || !stat
                    .get_condition_exprent()
                    .is_some_and(|c| c.is_function())
            {
                return false;
            }
            if [init0, init1].iter().any(|init| {
                init.get_left()
                    .try_as_var()
                    .is_some_and(kuna_source_locals::is_named_loop_local)
            }) {
                return false;
            }

            let func_right = init1.get_right().as_function();
            let func_inc = last_exprent.as_function();
            let arr = first_do_exprent.get_right().as_array();
            let inc_type = func_inc.get_func_type();

            if func_right.get_func_type() != FunctionType::ArrayLength
                || (inc_type != FunctionType::Ppi && inc_type != FunctionType::Ipp)
                || !arr.get_index().is_var()
                || !arr.get_array().is_var()
            {
                return false;
            }

            let index = arr.get_index().as_var();
            let array = arr.get_array().as_var();
            let count_expr = func_inc.get_lst_operands().get(0);

            if let Some(counter) = count_expr.try_as_var() {
                if counter.get_index() != index.get_index()
                    || counter.get_version() != index.get_version()
                {
                    return false;
                }

                if counter.is_var_referenced(stat.get_first().expect("DoStatement.first"), &[index])
                {
                    return false;
                }
            }

            if !array.equals_versions(func_right.get_lst_operands().get(0)) {
                return false;
            }

            if Self::is_var_used_before(first_do_exprent.get_left().as_var(), stat.0) {
                return false;
            }

            let length_of = func_right.get_lst_operands().get(0);
            length_of.add_bytecode_offsets(init0.get_bytecode().as_ref());
            length_of.add_bytecode_offsets(init1.get_bytecode().as_ref());
            length_of.add_bytecode_offsets(last_exprent.get_bytecode().as_ref());
            first_do_exprent
                .get_left()
                .add_bytecode_offsets(first_do_exprent.get_bytecode().as_ref());
            first_do_exprent
                .get_left()
                .add_bytecode_offsets(init0.get_bytecode().as_ref());

            stat.set_looptype(LoopType::ForEach);
            stat.set_init_exprent(first_do_exprent.get_left());
            stat.set_inc_exprent(func_right.get_lst_operands().get(0));
            let pre_data = pre_data.expect("preData");
            let pre_exprents = pre_data.get_exprents().expect("exprents");
            pre_exprents.remove_item(&init0.0);
            pre_exprents.remove_item(&init1.0);
            first_data
                .expect("firstData")
                .get_exprents()
                .expect("exprents")
                .remove_item(&first_do_exprent.0);
            last_data
                .expect("lastData")
                .get_exprents()
                .expect("exprents")
                .remove_item(&last_exprent);

            if let Some(init2) = init_exprents[2] {
                if init2.get_left().is_var() {
                    if let Some(for_array) = stat.get_inc_exprent().and_then(|e| e.try_as_var()) {
                        let copy = init2.get_left().as_var();

                        if copy.get_index() == array.get_index()
                            && copy.get_version() == array.get_version()
                            && !copy
                                .is_var_referenced(stat.get_parent().expect("parent"), &[for_array])
                        {
                            pre_data
                                .get_exprents()
                                .expect("exprents")
                                .remove_item(&init2.0);
                            init2
                                .get_right()
                                .add_bytecode_offsets(init2.get_bytecode().as_ref());
                            init2.get_right().add_bytecode_offsets(
                                stat.get_inc_exprent()
                                    .expect("DoStatement.incExprent")
                                    .get_bytecode()
                                    .as_ref(),
                            );
                            stat.set_inc_exprent(init2.get_right());
                        }
                    }
                }
            }

            if let Some(type_res) = first_do_exprent.0.check_expr_type_bounds() {
                if !type_res.get_lower_bounds().is_empty() {
                    let bound_type = type_res.get_lower_bounds()[0].type_.clone();
                    let var = first_do_exprent.get_left().as_var();
                    var.set_bound_type(Some(bound_type));
                }
            }

            return true;
        }

        false
    }

    pub fn is_var_used_before(var: VarExprent, st: Statement) -> bool {
        let mut flatten = FlattenStatementsHelper::new();
        flatten.build_direct_graph(st.get_top_parent());

        let stnd = flatten
            .get_direct_node(st)
            .expect("FlattenStatementsHelper.getDirectNode");

        let mut stack: VecDeque<DirectNode> = VecDeque::new();
        for pred in stnd.get_predecessors(DirectEdgeType::Regular).iter() {
            stack.push_back(pred.get_source());
        }
        let mut visited: JHashSet<DirectNode> = JHashSet::new();

        while let Some(node) = stack.pop_front() {
            if st.contains_statement(node.statement()) {
                continue;
            }

            for exprent in node.exprents().iter() {
                if Self::contains_var(var, exprent) {
                    return true;
                }
            }

            for pred in node.get_predecessors(DirectEdgeType::Regular).iter() {
                if visited.add(pred.get_source()) {
                    stack.push_front(pred.get_source());
                }
            }
        }

        false
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
        let Some(iexp) = Self::as_invocation_exprent(exp) else {
            return false;
        };
        let descriptor = iexp.get_descriptor();
        if !struct_instance_of(descriptor.ret.value.as_deref(), "java/util/Iterator") {
            return false;
        }
        let name = iexp.get_name();
        name == "iterator" || name == "listIterator"
    }

    pub fn is_has_next_call(exp: Exprent) -> bool {
        let Some(iexp) = Self::as_invocation_exprent(exp) else {
            return false;
        };
        if !struct_instance_of(Some(iexp.get_classname().as_str()), "java/util/Iterator") {
            return false;
        }
        iexp.get_name() == "hasNext" && iexp.get_string_descriptor() == "()Z"
    }

    pub fn is_next_call(exp: Exprent) -> bool {
        let Some(iexp) = Self::as_invocation_exprent(exp) else {
            return false;
        };
        if !struct_instance_of(Some(iexp.get_classname().as_str()), "java/util/Iterator") {
            return false;
        }
        iexp.get_name() == "next" && iexp.get_string_descriptor() == "()Ljava/lang/Object;"
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
        let mut outer = stat.get_parent();
        while let Some(o) = outer.filter(|o| !o.is_switch() && !o.is_do()) {
            outer = o.get_parent();
        }

        let Some(outer) = outer else {
            return false;
        };
        if !(outer.is_switch() || outer.as_do().get_looptype() != LoopType::Infinite) {
            return false;
        }

        let parent = stat.get_parent();
        let is_last = match parent.filter(|p| p.is_sequence()) {
            None => true,
            Some(seq) => seq.get_stats().borrow().get_last() == stat.0,
        };
        if is_last {
            if ifedge.get_destination() == outer {
                stat.add_successor(StatEdge::new_closure(
                    StatEdge::TYPE_CONTINUE,
                    stat.0,
                    ifedge.get_destination(),
                    Some(outer),
                ));
                return true;
            } else if MergeHelper::is_direct_path(outer, ifedge.get_destination()) {
                stat.add_successor(StatEdge::new_closure(
                    StatEdge::TYPE_BREAK,
                    stat.0,
                    ifedge.get_destination(),
                    Some(outer),
                ));
                return true;
            }
        }

        false
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
        let Some(first) = stat.get_first().filter(|f| f.is_sequence()) else {
            return false;
        };
        if !stat.get_successor_edges(StatEdge::TYPE_REGULAR).is_empty() {
            return false;
        }

        let extract_start = Self::extractable_from_loop(first.as_sequence(), stat);

        let first_stats = first.get_stats();
        if first_stats.borrow().is_empty() || extract_start <= 0 {
            return false;
        }
        if extract_start != 1 && kuna_javac_layout::active() {
            return false;
        }

        let (first_body, last_body, pre_extract, mut extract) = {
            let stats = first_stats.borrow();
            (
                stats.get(0),
                stats.get_last(),
                stats.get(extract_start as usize - 1),
                stats.sub_vec(extract_start as usize, stats.len()),
            )
        };

        let Some(first_if) = first_body.try_as_if() else {
            return false;
        };
        if first_if.iftype() != IfStatement::IFTYPE_IF
            || !first_body
                .get_basichead()
                .get_exprents()
                .expect("exprents")
                .is_empty()
        {
            return false;
        }

        let breaks = last_body.get_successor_edges(StatEdge::TYPE_BREAK);

        if breaks.is_empty() || !breaks[0].get_destination().is_dummy_exit() {
            return false;
        }

        let mut edges: JHashSet<StatEdge> = JHashSet::new();
        TryWithResourcesProcessor::find_edges_leaving(first_body, first_body, &mut edges);

        let mut continue_found = false;
        for edge in edges.to_vec() {
            if edge.get_destination() == stat.0 && edge.get_type() == StatEdge::TYPE_CONTINUE {
                continue_found = true;
                break;
            }
        }

        if !continue_found {
            return false;
        }

        for st in &extract {
            for edge in st.get_all_predecessor_edges() {
                if edge.get_type() == StatEdge::TYPE_REGULAR && edge.get_source() == pre_extract {
                    pre_extract.remove_successor(edge);
                }

                if edge.get_type() == StatEdge::TYPE_BREAK
                    && stat.contains_statement_strict(edge.get_source())
                {
                    stat.add_labeled_edge(edge);
                }
            }
        }

        extract.insert(0, stat.0);
        let seq = SequenceStatement::new(extract.clone());

        for st in extract.clone() {
            stat.get_first()
                .expect("DoStatement.first")
                .get_stats()
                .borrow_mut()
                .remove_with_key(&st.id());
        }

        stat.replace_with(seq.0);

        let seq_label_edges = seq.get_label_edges().borrow().to_vec();
        for edge in seq_label_edges {
            stat.add_labeled_edge(edge);
        }

        for edge in seq.get_predecessor_edges(StatEdge::TYPE_CONTINUE) {
            if stat.contains_statement_strict(edge.get_source()) {
                seq.remove_predecessor(edge);
                edge.get_source()
                    .change_edge_node(EdgeDirection::Forward, edge, stat.0);
                stat.add_predecessor(edge);
            }
        }

        seq.set_all_parent();

        let next = extract[1];
        stat.add_successor(StatEdge::new(StatEdge::TYPE_REGULAR, stat.0, next));

        let ifstat = first_if.get_ifstat().expect("IfStatement.ifstat");
        first.replace_with(ifstat);
        let ifedge = first_if.get_if_edge().expect("IfStatement.ifedge");
        ifedge.get_destination().remove_successor(ifedge);

        stat.set_first(ifstat);

        stat.set_all_parent();

        stat.set_looptype(LoopType::While);

        stat.set_condition_exprent(first_if.get_headexprent().get_condition());

        stat.get_first()
            .expect("DoStatement.first")
            .set_all_parent();

        true
    }

    fn extractable_from_loop(seq: SequenceStatement, loop_: DoStatement) -> i32 {
        let mut no_continue: Vec<Statement> = Vec::new();
        let stats = seq.get_stats();
        let mut i = 0;
        while i < stats.borrow().len() {
            let stat = stats.borrow().get(i);

            let mut edges: JHashSet<StatEdge> = JHashSet::new();
            TryWithResourcesProcessor::find_edges_leaving(stat, stat, &mut edges);

            let mut continue_found = false;
            for edge in edges.to_vec() {
                if edge.get_destination() == loop_.0 && edge.get_type() == StatEdge::TYPE_CONTINUE {
                    continue_found = true;
                    break;
                }
            }

            if !continue_found && i > 0 {
                let last = stats.borrow().get(i - 1);

                let succs = last.get_all_successor_edges();
                if succs.len() == 1 {
                    let edge = succs[0];

                    if edge.get_type() == StatEdge::TYPE_REGULAR && edge.get_destination() == stat {
                        no_continue.push(stat);
                    }
                }
            }
            i += 1;
        }

        if no_continue.is_empty() {
            return -1;
        }

        let index = stats.borrow().index_of(&no_continue[0]);
        index
    }
}
