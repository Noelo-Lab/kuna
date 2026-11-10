//! kuna fix `split_constructor_call` (`kuna_fixes`): an object creation whose
//! constructor arguments hold control flow is rebuilt as one `new X(args)`.
//!
//! The bytecode of `new X(args)` is `new X; dup; <args>; invokespecial
//! X.<init>`. When evaluating the arguments branches (Kotlin's `a!!` and
//! `a ?: throw ..`, an inlined `require`, an overflow check), the stack is
//! spilled at the branch and the allocation and the constructor call land in
//! different statements: `v = new X` and, later, `v.<init>(args)`.
//! `SimplifyExprentsHelper.isConstructorInvocationRemote` joins the two only
//! within one exprent list, so Vineflower prints the `<init>` call with
//! `$VF: Unable to resugar constructor`, which is not Java (or Kotlin).
//! With the fix the pair is joined across statements, `v = new X(args)` at
//! the constructor call, when the allocation is the only definition of `v`,
//! the call is the only `<init>` on it, nothing reads `v` on a path from the
//! allocation that has not passed the call, and the same handlers protect
//! both. The verifier allows nothing but `<init>` on the uninitialised
//! object, so what moves is the allocation itself, as the same-list join
//! already moves it past the statements in between.

use crate::code_type::CodeType;
use crate::direct_edge_type::DirectEdgeType;
use crate::direct_graph::DirectGraph;
use crate::direct_node::DirectNode;
use crate::exprent::Exprent;
use crate::flatten_statements_helper::FlattenStatementsHelper;
use crate::invocation_exprent::Type as InvocationFuncType;
use crate::root_statement::RootStatement;
use crate::rt::{JHashSet, JList};
use crate::statement::Statement;
use crate::var_version_pair::VarVersionPair;

/// Joins every split `v = new X` / `v.<init>(args)` pair of `root`; whether
/// any was joined.
pub fn join_split_constructors(root: RootStatement) -> bool {
    let mut lists = Vec::new();
    collect_lists(root.0, &mut lists);
    let mut news = Vec::new();
    let mut inits = Vec::new();
    for list in &lists {
        for e in list.to_vec() {
            if let Some(pair) = allocation(e) {
                news.push((pair, e));
            } else if let Some(pair) = init_call(e) {
                inits.push((pair, e));
            }
        }
    }
    let candidates: Vec<(Exprent, Exprent)> = news
        .iter()
        .filter_map(|(pair, a)| {
            let mut calls = inits.iter().filter(|(p, _)| p == pair);
            match (calls.next(), calls.next()) {
                (Some((_, i)), None) => Some((*a, *i)),
                _ => None,
            }
        })
        .collect();
    if candidates.is_empty() {
        return false;
    }

    let graph = FlattenStatementsHelper::new().build_direct_graph(root);
    let mut joined = false;
    for (a, i) in candidates {
        if joinable(graph, a, i) {
            join(&lists, a, i);
            joined = true;
        }
    }
    joined
}

fn collect_lists(stat: Statement, out: &mut Vec<JList<Exprent>>) {
    match stat.get_exprents() {
        Some(list) => out.push(list),
        None => {
            for st in stat.get_stats().borrow().to_vec() {
                collect_lists(st, out);
            }
        }
    }
}

/// `v = new X` with no constructor call yet: `v`.
fn allocation(e: Exprent) -> Option<VarVersionPair> {
    let assignment = e.try_as_assignment()?;
    let var = assignment.get_left().try_as_var()?;
    let new = assignment.get_right().try_as_new()?;
    let ty = new.get_new_type();
    (ty.type_ == CodeType::Object && ty.array_dim == 0 && new.get_constructor().is_none())
        .then(|| VarVersionPair::from_var(var))
}

/// `v.<init>(..)` as a statement: `v`.
fn init_call(e: Exprent) -> Option<VarVersionPair> {
    let call = e.try_as_invocation()?;
    if call.get_functype() != InvocationFuncType::Init {
        return None;
    }
    let var = call.get_instance()?.try_as_var()?;
    Some(VarVersionPair::from_var(var))
}

fn reads(e: Exprent, pair: VarVersionPair) -> bool {
    e.get_all_variables().contains(&pair)
}

fn node_of(graph: DirectGraph, e: Exprent) -> Option<(DirectNode, usize)> {
    graph.nodes().into_iter().find_map(|node| {
        node.exprents()
            .to_vec()
            .iter()
            .position(|x| *x == e)
            .map(|at| (node, at))
    })
}

fn handlers(node: DirectNode) -> Vec<String> {
    let mut ids: Vec<String> = node
        .get_successors(DirectEdgeType::Exception)
        .to_vec()
        .iter()
        .map(|edge| edge.get_destination().id())
        .collect();
    ids.sort();
    ids
}

fn joinable(graph: DirectGraph, a: Exprent, i: Exprent) -> bool {
    let pair = allocation(a).expect("allocation");
    let definitions = graph
        .nodes()
        .iter()
        .flat_map(|node| node.exprents().to_vec())
        .flat_map(|e| {
            let mut all = e.get_all_exprents_rec(true);
            all.push(e);
            all
        })
        .filter(|e| {
            e.try_as_assignment()
                .and_then(|s| s.get_left().try_as_var())
                .is_some_and(|v| VarVersionPair::from_var(v) == pair)
        })
        .count();
    if definitions != 1 {
        return false;
    }
    let (Some((node_a, at_a)), Some((node_i, at_i))) = (node_of(graph, a), node_of(graph, i))
    else {
        return false;
    };
    if node_a == node_i || handlers(node_a) != handlers(node_i) {
        return false;
    }
    let call = i.try_as_invocation().expect("init call");
    if call.get_lst_parameters().iter().any(|p| reads(p, pair))
        || node_i.exprents().to_vec()[..at_i]
            .iter()
            .any(|e| reads(*e, pair))
        || node_a.exprents().to_vec()[at_a + 1..]
            .iter()
            .any(|e| reads(*e, pair))
    {
        return false;
    }

    let mut seen: JHashSet<String> = JHashSet::new();
    let mut work: Vec<DirectNode> = successors(node_a);
    let mut reached = false;
    while let Some(node) = work.pop() {
        if !seen.add(node.id()) {
            continue;
        }
        if node == node_i {
            reached = true;
            continue;
        }
        if node.exprents().to_vec().iter().any(|e| reads(*e, pair)) {
            return false;
        }
        work.extend(successors(node));
    }
    reached
}

fn successors(node: DirectNode) -> Vec<DirectNode> {
    DirectEdgeType::TYPES
        .iter()
        .flat_map(|t| node.get_successors(*t).to_vec())
        .map(|edge| edge.get_destination())
        .collect()
}

fn join(lists: &[JList<Exprent>], a: Exprent, i: Exprent) {
    let assignment = a.as_assignment();
    let new = assignment.get_right().as_new();
    let call = i.as_invocation();
    new.set_constructor(Some(call));
    call.set_instance(None);
    for list in lists {
        if let Some(at) = list.to_vec().iter().position(|e| *e == i) {
            list.set(at, assignment.copy());
        }
    }
    for list in lists {
        if let Some(at) = list.to_vec().iter().position(|e| *e == a) {
            list.remove_at(at);
        }
    }
}
