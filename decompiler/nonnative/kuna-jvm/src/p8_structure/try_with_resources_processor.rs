//! Port of `org/jetbrains/java/decompiler/modules/decompiler/TryWithResourcesProcessor.java`.
#![allow(unused_variables, dead_code)]

use crate::assignment_exprent::AssignmentExprent;
use crate::basic_block_statement::BasicBlockStatement;
use crate::catch_all_statement::CatchAllStatement;
use crate::catch_statement::CatchStatement;
use crate::decompiler_context::DecompilerContext;
use crate::exprent::Exprent;
use crate::function_exprent::{FunctionExprent, FunctionType};
use crate::rt::{JHashMap, JHashSet, JLinkedHashSet};
use crate::stat_edge::StatEdge;
use crate::statement::Statement;
use crate::unported;
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
        unported!("TryWithResourcesProcessor.makeTryWithResource")
    }

    pub fn make_try_with_resource_j11(try_statement: CatchStatement) -> bool {
        unported!("TryWithResourcesProcessor.makeTryWithResourceJ11")
    }

    fn is_valid(stat: Statement, closeable: VarExprent, nullable: bool) -> bool {
        unported!("TryWithResourcesProcessor.isValid")
    }

    fn remove_close(statement: Statement, nullable: bool) {
        unported!("TryWithResourcesProcessor.removeClose")
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
                if edge.with(|d| d.explicit)
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
        unported!("TryWithResourcesProcessor.removeRedundantThrow")
    }
}
