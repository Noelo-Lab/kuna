//! Port of `org/jetbrains/java/decompiler/modules/decompiler/TryWithResourcesProcessor.java`.
//!
//! kuna fix `twr_unary_condition` (`kuna_fixes`): in
//! `make_try_with_resource_j11`, a handler whose first `if` tests a unary
//! condition (`!flag`) is not a resource's null check, where Vineflower reads
//! the condition's missing second operand.
use crate::assignment_exprent::AssignmentExprent;
use crate::basic_block_statement::BasicBlockStatement;
use crate::catch_all_statement::CatchAllStatement;
use crate::catch_statement::CatchStatement;
use crate::decompiler_context::DecompilerContext;
use crate::exit_exprent;
use crate::exprent::Exprent;
use crate::function_exprent::{FunctionExprent, FunctionType};
use crate::kuna_fixes::{self, KunaFix};
use crate::rt::{JHashMap, JHashSet, JLinkedHashSet};
use crate::sequence_helper::SequenceHelper;
use crate::stat_edge::StatEdge;
use crate::statement::Statement;
use crate::var_exprent::VarExprent;
use crate::var_type::VarType;

pub struct TryWithResourcesProcessor;

/// The Java `Set<StatEdge>` the edge searches fill: callers pass a `HashSet`,
/// `findExitpoints` a `LinkedHashSet`.
pub trait StatEdgeSet {
    fn add(&mut self, edge: StatEdge) -> bool;
}

impl StatEdgeSet for JHashSet<StatEdge> {
    fn add(&mut self, edge: StatEdge) -> bool {
        JHashSet::add(self, edge)
    }
}

impl StatEdgeSet for JLinkedHashSet<StatEdge> {
    fn add(&mut self, edge: StatEdge) -> bool {
        JLinkedHashSet::add(self, edge)
    }
}

impl TryWithResourcesProcessor {
    pub fn make_try_with_resource(finally_stat: CatchAllStatement) -> bool {
        let handler = finally_stat.get_handler();

        if handler.get_stats().borrow().len() != 2 {
            return false;
        }

        let Some(to_check) = finally_stat.get_handler().get_first() else {
            return false;
        };
        let Some(to_check) = to_check
            .try_as_if()
            .and_then(|i| i.get_ifstat())
            .and_then(|s| s.try_as_if())
        else {
            return false;
        };

        let Some(else_block) = to_check.get_elsestat() else {
            return false;
        };

        let mut var: Option<VarExprent> = None;

        if let Some(exprents) = else_block.get_exprents().filter(|e| e.len() == 1) {
            let exp = exprents.get(0);

            if Self::is_closeable(exp) {
                var = Some(exp.as_invocation().get_instance().unwrap().as_var());
            }
        }

        if let Some(var) = var {
            let mut ass: Option<AssignmentExprent> = None;
            let mut init_block: Option<BasicBlockStatement> = None;
            for edge in finally_stat.get_all_predecessor_edges() {
                if edge.get_destination() == finally_stat.0 && edge.get_source().is_basic_block() {
                    ass = Self::find_resource_def(var, edge.get_source());
                    if ass.is_some() {
                        init_block = Some(edge.get_source().as_basic_block());
                        break;
                    }
                }
            }

            if let Some(ass) = ass {
                let init_block = init_block.unwrap();
                let stat = finally_stat.get_parent();
                let stat2 = finally_stat.get_first();

                if let Some(child) = stat2.and_then(|s| s.try_as_catch()) {
                    let resource_def = ass.copy().as_assignment();
                    if ass
                        .get_right()
                        .get_expr_type()
                        .equals(&VarType::VARTYPE_NULL())
                    {
                        if let Some(child_first) = child.get_first() {
                            Self::fix_resource_assignment(resource_def, child_first);
                        }
                    }

                    if resource_def
                        .get_right()
                        .get_expr_type()
                        .equals(&VarType::VARTYPE_NULL())
                    {
                        return false;
                    }

                    init_block.get_exprents().unwrap().remove_item(&ass.0);
                    child.get_resources().add_at(0, resource_def.0);

                    if !finally_stat.get_var_definitions().is_empty() {
                        child
                            .get_var_definitions()
                            .add_all_at(0, finally_stat.get_var_definitions().to_vec());
                    }

                    stat.unwrap().replace_statement(finally_stat.0, child.0);

                    SequenceHelper::destroy_and_flatten_statement(finally_stat.get_handler());

                    Self::remove_redundant_throw(init_block, child);
                    return true;
                }
            }
        }

        false
    }

    pub fn make_try_with_resource_j11(try_statement: CatchStatement) -> bool {
        if try_statement.get_stats().borrow().len() < 2 {
            return false;
        }

        if try_statement.get_vars().get(0).get_var_type().value_str() != "java/lang/Throwable" {
            return false;
        }

        let mut inner = try_statement.get_stats().borrow().get(1);

        let mut closeable: Option<VarExprent> = None;

        let mut nullable = false;

        if inner.is_sequence() {
            inner = inner.get_stats().borrow().get(0);

            if let Some(inner_try) = inner.try_as_catch() {
                if inner.get_stats().borrow().is_empty() {
                    return false;
                }

                if inner_try.get_vars().get(0).get_var_type().value_str() != "java/lang/Throwable" {
                    return false;
                }

                let in_try = inner.get_stats().borrow().get(0);

                if in_try.is_basic_block() && !in_try.get_exprents().unwrap().is_empty() {
                    let first = in_try.get_exprents().unwrap().get(0);

                    if Self::is_closeable(first) {
                        closeable = Some(first.as_invocation().get_instance().unwrap().as_var());
                    }
                }
            }

            if let Some(inner_if) = inner.try_as_if() {
                let if_case = inner_if.get_headexprent().get_condition();

                if let Some(if_func) = if_case.try_as_function() {
                    let func = Self::unwrap_negations(if_func);

                    let check = func.get_lst_operands().get(0);

                    let Some(check) = check.try_as_var() else {
                        return false;
                    };

                    if func.get_lst_operands().len() < 2
                        && kuna_fixes::enabled(KunaFix::TwrUnaryCondition)
                    {
                        return false;
                    }

                    if func
                        .get_lst_operands()
                        .get(1)
                        .get_expr_type()
                        .equals(&VarType::VARTYPE_NULL())
                    {
                        let Some(if_stat) = inner_if.get_ifstat() else {
                            return false;
                        };
                        inner = if_stat;

                        if inner.is_catch() && !inner.get_stats().borrow().is_empty() {
                            if inner.get_stats().borrow().is_empty() {
                                return false;
                            }

                            let inner_try = inner.as_catch();
                            if inner_try.get_vars().get(0).get_var_type().value_str()
                                != "java/lang/Throwable"
                            {
                                return false;
                            }

                            let in_try = inner.get_stats().borrow().get(0);

                            if in_try.is_basic_block() && !in_try.get_exprents().unwrap().is_empty()
                            {
                                let first = in_try.get_exprents().unwrap().get(0);

                                if Self::is_closeable(first) {
                                    let found =
                                        first.as_invocation().get_instance().unwrap().as_var();
                                    closeable = Some(found);
                                    nullable = true;

                                    if found.get_var_version_pair() != check.get_var_version_pair()
                                    {
                                        closeable = None;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        let Some(closeable) = closeable else {
            return false;
        };

        let mut destinations = Self::find_exitpoints(try_statement.0);
        if destinations.is_empty() {
            return false;
        }

        let mut check = Some(try_statement.0);
        let mut preds: Vec<StatEdge> = Vec::new();
        while let Some(c) = check.filter(|_| preds.is_empty()) {
            preds = c.get_predecessor_edges(StatEdge::TYPE_REGULAR);
            check = c.get_parent();
        }

        if preds.is_empty() {
            return false;
        }

        let edge = preds[0];
        if edge.get_source().is_basic_block() {
            let Some(assignment) = Self::find_resource_def(closeable, edge.get_source()) else {
                return false;
            };

            for destination in destinations.to_vec() {
                if !Self::is_valid(destination, closeable, nullable) {
                    return false;
                }
            }

            for destination in destinations.to_vec() {
                Self::remove_close(destination, nullable);
            }

            edge.get_source()
                .get_exprents()
                .unwrap()
                .remove_item(&assignment.0);

            try_statement.get_resources().add_at(0, assignment.0);

            let remove = try_statement.get_stats().borrow().get(1);

            SequenceHelper::destroy_and_flatten_statement(remove);

            try_statement.get_stats().borrow_mut().remove_at(1);

            if let Some(declaration) = assignment.get_left().try_as_var() {
                if (assignment.get_right().is_var() || assignment.get_right().is_field())
                    && !declaration.is_var_referenced(try_statement.0, &[])
                {
                    try_statement.get_resources().set(0, assignment.get_right());
                }
            }

            destinations = Self::find_exitpoints(try_statement.0);

            for destination in destinations.to_vec() {
                let mut exit_stat = destination;

                while let Some(block) = exit_stat.try_as_basic_block() {
                    let succs = block.get_all_successor_edges();
                    if !(block.get_exprents().unwrap().is_empty() && succs.len() == 1) {
                        break;
                    }
                    exit_stat = succs[0].get_destination();
                }

                let Some(block) = exit_stat.try_as_basic_block() else {
                    continue;
                };
                let block_exprents = block.get_exprents().unwrap();
                if block_exprents.len() != 1 {
                    continue;
                }
                let Some(exit) = block_exprents
                    .get(0)
                    .try_as_exit()
                    .filter(|e| e.get_exit_type() == exit_exprent::Type::Return)
                else {
                    continue;
                };

                let break_preds = destination.get_predecessor_edges(StatEdge::TYPE_BREAK);
                if break_preds.len() != 1 {
                    continue;
                }
                let Some(set_exit) = break_preds[0].get_source().try_as_basic_block() else {
                    continue;
                };

                let set_exit_exprents = set_exit.get_exprents().unwrap();
                if set_exit_exprents.is_empty() {
                    continue;
                }
                let Some(assign) = set_exit_exprents
                    .get(set_exit_exprents.len() - 1)
                    .try_as_assignment()
                else {
                    continue;
                };
                let Some(variable) = assign.get_left().try_as_var() else {
                    continue;
                };

                if exit
                    .get_value()
                    .is_some_and(|v| variable.equals_versions(v))
                    && !variable.is_var_referenced(
                        try_statement.get_top_parent().0,
                        &[exit.get_value().unwrap().as_var()],
                    )
                {
                    exit.replace_exprent(exit.get_value().unwrap(), assign.get_right());
                    set_exit_exprents.set(set_exit_exprents.len() - 1, exit.0);
                    exit_stat.replace_with_empty();
                }
            }

            return true;
        }

        false
    }

    fn is_valid(stat: Statement, closeable: VarExprent, nullable: bool) -> bool {
        if nullable {
            if let Some(if_stat) = stat.try_as_if() {
                let condition = if_stat.get_headexprent().get_condition();

                if let Some(cond_func) = condition.try_as_function() {
                    let func = Self::unwrap_negations(cond_func);

                    if func.get_func_type() == FunctionType::Ne
                        && func.get_lst_operands().get(0).is_var()
                        && func
                            .get_lst_operands()
                            .get(1)
                            .get_expr_type()
                            .equals(&VarType::VARTYPE_NULL())
                    {
                        if let Some(var) = func.get_lst_operands().get(0).try_as_var() {
                            if var.get_var_version_pair() == closeable.get_var_version_pair() {
                                return true;
                            }
                        }
                    }
                }
            }
        } else if stat.is_basic_block() {
            if let Some(exprents) = stat.get_exprents().filter(|e| !e.is_empty()) {
                let exprent = exprents.get(0);

                if let Some(invocation) = exprent.try_as_invocation() {
                    let inst = invocation.get_instance();

                    if let Some(inst) = inst {
                        if inst.is_var() && inst.equals(closeable.0) && Self::is_closeable(exprent)
                        {
                            return true;
                        }
                    }
                }
            }
        }

        false
    }

    fn remove_close(statement: Statement, nullable: bool) {
        if nullable {
            let edges = statement.get_all_successor_edges();
            if !edges.is_empty() && edges[0].closure() == statement.get_parent() {
                SequenceHelper::destroy_and_flatten_statement(statement);
            } else {
                for edge in statement.get_first().unwrap().get_all_successor_edges() {
                    edge.get_destination().remove_predecessor(edge);
                }

                for edge in statement
                    .as_if()
                    .get_ifstat()
                    .unwrap()
                    .get_all_successor_edges()
                {
                    edge.get_destination().remove_predecessor(edge);

                    if let Some(closure) = edge.closure() {
                        closure.get_label_edges().borrow_mut().remove(&edge);
                    }
                }

                statement.replace_with_empty();
            }
        } else {
            statement.get_exprents().unwrap().remove_at(0);
        }
    }

    fn find_exitpoints(stat: Statement) -> JHashSet<Statement> {
        let mut edges: JLinkedHashSet<StatEdge> = JLinkedHashSet::new();
        Self::find_edges_leaving(stat.get_first().unwrap(), stat, &mut edges);

        let mut destinations = JHashSet::new();
        for edge in edges.iter() {
            destinations.add(edge.get_destination());
        }
        destinations
    }

    /// Java: findEdgesLeaving(Statement, Statement, Set)
    pub fn find_edges_leaving(curr: Statement, check: Statement, edges: &mut impl StatEdgeSet) {
        Self::find_edges_leaving_exit(curr, check, edges, false);
    }

    /// Java: findEdgesLeaving(Statement, Statement, Set, boolean)
    pub fn find_edges_leaving_exit(
        curr: Statement,
        check: Statement,
        edges: &mut impl StatEdgeSet,
        allow_exit: bool,
    ) {
        Self::find_edges_leaving_found(curr, check, edges, allow_exit, &mut JHashMap::new());
    }

    /// Java: findEdgesLeaving(Statement, Statement, Set, boolean, Map)
    fn find_edges_leaving_found(
        curr: Statement,
        check: Statement,
        edges: &mut impl StatEdgeSet,
        allow_exit: bool,
        found: &mut JHashMap<Statement, JHashSet<Statement>>,
    ) {
        for edge in curr.get_all_successor_edges() {
            let destination = edge.get_destination();
            if !check.contains_statement(destination)
                && (allow_exit || !destination.is_dummy_exit())
            {
                if edge.explicit()
                    || found
                        .iter()
                        .all(|(k, v)| !k.contains_statement(curr) || !v.contains(&destination))
                {
                    edges.add(edge);
                    found
                        .compute_if_absent(curr, JHashSet::new)
                        .add(destination);
                }
            }
        }

        let stats = curr.get_stats().borrow().to_vec();
        for stat in stats {
            Self::find_edges_leaving_found(stat, check, edges, allow_exit, found);
        }
    }

    fn unwrap_negations(func: FunctionExprent) -> FunctionExprent {
        let mut func = func;
        while func.get_func_type() == FunctionType::BoolNot {
            let expr = func.get_lst_operands().get(0);

            match expr.try_as_function() {
                Some(inner) => func = inner,
                None => break,
            }
        }

        func
    }

    fn find_resource_def(var: VarExprent, prev_statement: Statement) -> Option<AssignmentExprent> {
        for exp in prev_statement.get_exprents().unwrap().iter() {
            if let Some(ass) = exp.try_as_assignment() {
                if let Some(left) = ass.get_left().try_as_var() {
                    if left.get_var_version_pair() == var.get_var_version_pair() {
                        return Some(ass);
                    }
                }
            }
        }

        None
    }

    fn is_closeable(exp: Exprent) -> bool {
        if let Some(invoc_exp) = exp.try_as_invocation() {
            if invoc_exp.get_name() == "close" && invoc_exp.get_string_descriptor() == "()V" {
                if invoc_exp.get_instance().is_some_and(|i| i.is_var()) {
                    return DecompilerContext::get_struct_context()
                        .instance_of(&invoc_exp.get_classname(), "java/lang/AutoCloseable");
                }
            }
        }

        false
    }

    fn fix_resource_assignment(ass: AssignmentExprent, statement: Statement) {
        if let Some(exprents) = statement.get_exprents() {
            for exp in exprents.iter() {
                if let Some(to_remove) = exp.try_as_assignment() {
                    if ass.get_left().equals(to_remove.get_left())
                        && !to_remove
                            .get_right()
                            .get_expr_type()
                            .equals(&VarType::VARTYPE_NULL())
                    {
                        ass.set_right(to_remove.get_right());
                        exprents.remove_item(&exp);
                        break;
                    }
                }
            }
        }
    }

    fn remove_redundant_throw(init_block: BasicBlockStatement, catch_stat: CatchStatement) -> bool {
        if catch_stat.get_stats().borrow().len() > 1 {
            let mut removed = false;
            let mut temp: Option<Statement> = None;
            let mut i = 1;
            while i < catch_stat.get_stats().borrow().len() {
                let t = catch_stat.get_stats().borrow().get(i);
                temp = Some(t);

                if let Some(exprents) = t.get_exprents().filter(|_| t.is_basic_block()) {
                    if exprents.len() >= 2
                        && catch_stat.get_vars().get(i - 1).get_var_type().value_str()
                            == "java/lang/Throwable"
                    {
                        if let Some(exit) = exprents.get(exprents.len() - 1).try_as_exit() {
                            if exit.get_exit_type() == exit_exprent::Type::Throw
                                && exit
                                    .get_value()
                                    .unwrap()
                                    .equals(catch_stat.get_vars().get(i - 1).0)
                            {
                                catch_stat.get_exct_strings().remove_at(i - 1);
                                catch_stat.get_vars().remove_at(i - 1);
                                catch_stat.get_stats().borrow_mut().remove_at(i);

                                for edge in t.get_all_successor_edges() {
                                    edge.get_source().remove_successor(edge);
                                }

                                removed = true;
                                break;
                            }
                        }
                    }
                }
                i += 1;
            }

            if removed {
                let temp_exprents = temp.unwrap().get_exprents().unwrap();
                if let Some(assignment_exp) = temp_exprents
                    .get(temp_exprents.len() - 2)
                    .try_as_assignment()
                {
                    if assignment_exp.get_left().get_expr_type().value_str()
                        == "java/lang/Throwable"
                    {
                        let init_exprents = init_block.get_exprents().unwrap();
                        for exprent in init_exprents.iter() {
                            if let Some(to_remove) = exprent.try_as_assignment() {
                                if to_remove.get_left().equals(assignment_exp.get_left()) {
                                    init_exprents.remove_item(&to_remove.0);
                                    return true;
                                }
                            }
                        }
                    }
                }
            }
        }
        false
    }
}
