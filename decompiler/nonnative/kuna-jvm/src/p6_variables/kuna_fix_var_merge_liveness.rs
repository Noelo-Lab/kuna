//! kuna fix `var_merge_liveness` (`kuna_fixes`): `VarDefinitionHelper.mergeVars`
//! merges a variable into an earlier one of the same original slot only when
//! no write of the later one (other than a copy of the earlier one) can be
//! followed by a read of the earlier one's value.
//! Vineflower checks at most the rest of the defining block, which holds for
//! slots whose versions keep their bytecode lifetimes, but not after a pass
//! extends one (the Kotlin schedule's `SimplifyStack` with `inlineRegularVars`
//! replaces a local copied from a stack variable by that variable), so the
//! merge made the later variable overwrite a value still in use.
//!
//! With the fix `finally_exit_versions`, a search from a write inside a
//! verified `finally` body also goes from the body's end to the code its
//! protected code leaves to normally (the flattened graph has no such edge).

use crate::direct_edge_type::DirectEdgeType;
use crate::direct_node::DirectNode;
use crate::direct_node_type::DirectNodeType;
use crate::exprent::Exprent;
use crate::flatten_statements_helper::FlattenStatementsHelper;
use crate::function_exprent::FunctionType;
use crate::kuna_fixes::{self, KunaFix};
use crate::p3_dataflow::kuna_fix_finally_exit_versions::FinallyExitVersions;
use crate::root_statement::RootStatement;
use crate::rt::{JHashMap, JHashSet};
use crate::var_exprent::VarExprent;
use crate::var_type::VarType;
use crate::var_version_pair::VarVersionPair;

/// Whether some path of `root`'s flow graph reads `target` after a write of
/// `key` that does not copy `target`, without writing `target` in between.
pub fn overwrites_live(root: RootStatement, key: VarVersionPair, target: VarVersionPair) -> bool {
    let graph = FlattenStatementsHelper::new().build_direct_graph(root);
    let nodes: Vec<(DirectNode, Vec<Vec<Event>>)> = graph
        .nodes()
        .into_iter()
        .map(|node| {
            let defines = defines_vars(node);
            let events = node
                .exprents()
                .to_vec()
                .into_iter()
                .map(|e| events(e, defines))
                .collect();
            (node, events)
        })
        .collect();
    let exits = if kuna_fixes::enabled(KunaFix::FinallyExitVersions) {
        FinallyExitVersions::exit_edges(&graph.nodes())
    } else {
        JHashMap::new()
    };
    read_after_write(&nodes, &exits, key, target)
}

/// What evaluating an exprent does to variables, in order.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Event {
    Read(VarVersionPair),
    /// A write; `true` when it may not happen (under `&&`, `||` or `?:`).
    Write(VarVersionPair, bool),
    /// `key = target`: leaves the two equal.
    Copy(VarVersionPair, VarVersionPair),
}

fn read_after_write(
    nodes: &[(DirectNode, Vec<Vec<Event>>)],
    exits: &JHashMap<String, Vec<DirectNode>>,
    key: VarVersionPair,
    target: VarVersionPair,
) -> bool {
    let mut by_id: JHashMap<String, usize> = JHashMap::new();
    for (i, (node, _)) in nodes.iter().enumerate() {
        by_id.put(node.id(), i);
    }
    let mut work: Vec<DirectNode> = Vec::new();
    for (node, exprents) in nodes {
        let flat: Vec<Event> = exprents.iter().flatten().copied().collect();
        let start = flat.iter().position(|e| {
            matches!(e, Event::Write(v, _) if *v == key)
                || matches!(e, Event::Copy(v, w) if *v == key && *w != target)
        });
        if let Some(i) = start {
            match scan(&flat[i + 1..], target) {
                Scan::Read => return true,
                Scan::Killed => {}
                Scan::Through => {
                    let bodies = finally_bodies(*node, exits);
                    if bodies.is_empty() {
                        work.extend(successors(*node, exits, &bodies));
                    } else if reaches_read(nodes, &by_id, exits, &bodies, *node, target) {
                        return true;
                    }
                }
            }
        }
    }
    walk(nodes, &by_id, exits, &[], work, target)
}

/// Fix `finally_exit_versions`: a write inside a verified `finally` body
/// also reaches the code its protected code leaves to normally.
fn reaches_read(
    nodes: &[(DirectNode, Vec<Vec<Event>>)],
    by_id: &JHashMap<String, usize>,
    exits: &JHashMap<String, Vec<DirectNode>>,
    bodies: &[String],
    node: DirectNode,
    target: VarVersionPair,
) -> bool {
    walk(
        nodes,
        by_id,
        exits,
        bodies,
        successors(node, exits, bodies),
        target,
    )
}

fn walk(
    nodes: &[(DirectNode, Vec<Vec<Event>>)],
    by_id: &JHashMap<String, usize>,
    exits: &JHashMap<String, Vec<DirectNode>>,
    bodies: &[String],
    mut work: Vec<DirectNode>,
    target: VarVersionPair,
) -> bool {
    let mut seen: JHashSet<String> = JHashSet::new();
    while let Some(node) = work.pop() {
        if !seen.add(node.id()) {
            continue;
        }
        let Some(&i) = by_id.get(&node.id()) else {
            continue;
        };
        let flat: Vec<Event> = nodes[i].1.iter().flatten().copied().collect();
        match scan(&flat, target) {
            Scan::Read => return true,
            Scan::Killed => {}
            Scan::Through => work.extend(successors(node, exits, bodies)),
        }
    }
    false
}

enum Scan {
    Read,
    Killed,
    Through,
}

/// The ids of the `FINALLY_END` nodes of the verified finally bodies `node`
/// lies in: the ones whose normal exits a walk from `node` may take.
fn finally_bodies(node: DirectNode, exits: &JHashMap<String, Vec<DirectNode>>) -> Vec<String> {
    let mut res = Vec::new();
    let mut try_finally = node.try_finally();
    while let Some(enclosing) = try_finally {
        if enclosing.type_() == DirectNodeType::FinallyEnd && exits.contains_key(&enclosing.id()) {
            res.push(enclosing.id());
        }
        try_finally = enclosing.try_finally();
    }
    res
}

fn successors(
    node: DirectNode,
    exits: &JHashMap<String, Vec<DirectNode>>,
    bodies: &[String],
) -> Vec<DirectNode> {
    let mut res: Vec<DirectNode> = DirectEdgeType::TYPES
        .iter()
        .flat_map(|&t| node.get_successors(t).to_vec())
        .map(|edge| edge.get_destination())
        .collect();
    let id = node.id();
    if bodies.contains(&id) {
        if let Some(after) = exits.get(&id) {
            res.extend(after.iter().copied());
        }
    }
    res
}

/// Whether `node`'s lone variables take a value: a `for`-each variable or a
/// caught exception.
fn defines_vars(node: DirectNode) -> bool {
    matches!(
        node.type_(),
        DirectNodeType::ForeachVardef | DirectNodeType::Catch | DirectNodeType::CombinedCatch
    )
}

/// The first effect of `events` on `var`.
fn scan(events: &[Event], var: VarVersionPair) -> Scan {
    for event in events {
        match *event {
            Event::Read(v) if v == var => return Scan::Read,
            Event::Write(v, false) | Event::Copy(v, _) if v == var => return Scan::Killed,
            _ => {}
        }
    }
    Scan::Through
}

/// The events of a node's exprent: a lone variable is a declaration, or a
/// write in a node that `defines_vars`.
fn events(exprent: Exprent, defines: bool) -> Vec<Event> {
    let mut out = Vec::new();
    match exprent.try_as_var() {
        Some(var) if defines => out.push(Event::Write(VarVersionPair::from_var(var), false)),
        Some(_) => {}
        None => evaluate(exprent, false, &mut out),
    }
    out
}

/// `exprent`'s events in evaluation order; `conditional` when it may not be
/// evaluated.
fn evaluate(exprent: Exprent, conditional: bool, out: &mut Vec<Event>) {
    if let Some(var) = exprent.try_as_var() {
        out.push(Event::Read(VarVersionPair::from_var(var)));
        return;
    }
    if let Some(assign) = exprent.try_as_assignment() {
        let left = assign.get_left();
        match left.try_as_var() {
            Some(var) => {
                let var = VarVersionPair::from_var(var);
                if assign.get_cond_type().is_some() {
                    out.push(Event::Read(var));
                }
                let right = assign.get_right();
                evaluate(right, conditional, out);
                match copied_var(right) {
                    Some(source) if assign.get_cond_type().is_none() && !conditional => {
                        out.push(Event::Copy(var, VarVersionPair::from_var(source)))
                    }
                    _ => out.push(Event::Write(var, conditional)),
                }
            }
            None => {
                evaluate(left, conditional, out);
                evaluate(assign.get_right(), conditional, out);
            }
        }
        return;
    }
    if let Some(func) = exprent.try_as_function() {
        let operands = func.get_lst_operands().to_vec();
        if func.get_func_type().is_ppmm() {
            if let Some(var) = operands.first().and_then(|o| o.try_as_var()) {
                let var = VarVersionPair::from_var(var);
                out.push(Event::Read(var));
                out.push(Event::Write(var, conditional));
                return;
            }
        }
        if matches!(
            func.get_func_type(),
            FunctionType::BooleanAnd | FunctionType::BooleanOr | FunctionType::Ternary
        ) {
            for (i, &operand) in operands.iter().enumerate() {
                evaluate(operand, conditional || i > 0, out);
            }
            return;
        }
    }
    for child in exprent.get_all_exprents() {
        evaluate(child, conditional, out);
    }
}

/// The variable whose value `exprent` is unchanged: the variable itself, or
/// a reference cast of it.
fn copied_var(exprent: Exprent) -> Option<VarExprent> {
    if let Some(var) = exprent.try_as_var() {
        return Some(var);
    }
    let func = exprent.try_as_function()?;
    if func.get_func_type() != FunctionType::Cast
        || func
            .get_expr_type_opt()
            .is_none_or(|t| VarType::is_primitive(&t))
    {
        return None;
    }
    func.get_lst_operands().get(0).try_as_var()
}
