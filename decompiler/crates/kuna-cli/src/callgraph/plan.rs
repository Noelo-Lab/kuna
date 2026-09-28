//! Deterministic callee-first scheduling and recursive-component policy.

use std::collections::{BTreeMap, BTreeSet};

use kuna_console::engine::FunctionEntry;

use super::CallGraph;

/// Return `(target index, may park recovered types)` in callee-first order.
/// Tarjan walks roots and neighbors in target-index order and emits components
/// after their callees. Recursive functions park only under `cycles`, which
/// orders each recursive component with [`cycle_order`].
pub(crate) fn callee_first_plan(
    graph: &CallGraph,
    targets: &[FunctionEntry],
    cycles: bool,
) -> Vec<(usize, bool)> {
    let mut index_of: BTreeMap<u64, usize> = BTreeMap::new();
    for (i, t) in targets.iter().enumerate() {
        index_of.entry(t.addr.get_offset()).or_insert(i);
    }
    // Callees first: an edge u -> v means "u calls v", and Tarjan pops v's
    // component before u's.
    let mut edges: Vec<Vec<usize>> = vec![Vec::new(); targets.len()];
    let mut self_recursive: Vec<bool> = vec![false; targets.len()];
    for (i, t) in targets.iter().enumerate() {
        let mut out: Vec<usize> = Vec::new();
        for (callee, _kind) in graph.callees_of(t.addr.get_offset()) {
            let Some(&j) = index_of.get(&callee) else {
                continue;
            };
            if j == i {
                self_recursive[i] = true;
                continue;
            }
            out.push(j);
        }
        out.sort_unstable();
        out.dedup();
        edges[i] = out;
    }
    plan_from_components(&edges, &self_recursive, cycles)
}

/// [`callee_first_plan`] over the call edges (self-calls removed and recorded
/// in `self_recursive`).
///
/// With `cycles`, every function parks.  The members of a component with more
/// than one are decompiled once each, in [`cycle_order`], and each states its
/// types as it finishes, so a member decompiled later and every caller outside
/// the component read them.  An earlier member does not see a later one's
/// statement: a second round was measured and costs far more than it recovers
/// (docs/spec/04-calls-and-prototypes.md).
fn plan_from_components(
    edges: &[Vec<usize>],
    self_recursive: &[bool],
    cycles: bool,
) -> Vec<(usize, bool)> {
    let (order, component_size) = tarjan_scc(edges);
    if !cycles {
        return order
            .into_iter()
            .map(|i| (i, component_size[i] == 1 && !self_recursive[i]))
            .collect();
    }
    let mut component = vec![0usize; edges.len()];
    let mut at = 0;
    while at < order.len() {
        let n = component_size[order[at]];
        for &m in &order[at..at + n] {
            component[m] = at;
        }
        at += n;
    }
    let mut entered = vec![false; edges.len()];
    for (u, out) in edges.iter().enumerate() {
        for &v in out {
            if component[u] != component[v] {
                entered[v] = true;
            }
        }
    }
    let mut plan: Vec<(usize, bool)> = Vec::with_capacity(order.len());
    let mut at = 0;
    while at < order.len() {
        let n = component_size[order[at]];
        let members = &order[at..at + n];
        at += n;
        if n == 1 {
            plan.push((members[0], true));
        } else {
            plan.extend(
                cycle_order(members, edges, &entered)
                    .into_iter()
                    .map(|m| (m, true)),
            );
        }
    }
    plan
}

/// The decompile order inside one recursive component (`members` ascending).
///
/// A depth-first walk over the component's own edges that emits each member
/// after the partners it reaches, started first from the members something
/// outside the component calls.  Only a call back to a member already on the
/// walk is left unanswered, and a member called from outside -- the one the
/// component's callers read -- decompiles after the partners it reaches.
fn cycle_order(members: &[usize], edges: &[Vec<usize>], entered: &[bool]) -> Vec<usize> {
    let inside: BTreeSet<usize> = members.iter().copied().collect();
    let roots = members
        .iter()
        .filter(|&&m| entered[m])
        .chain(members.iter().filter(|&&m| !entered[m]));
    let mut seen: BTreeSet<usize> = BTreeSet::new();
    let mut out: Vec<usize> = Vec::with_capacity(members.len());
    for &root in roots {
        if !seen.insert(root) {
            continue;
        }
        let mut work: Vec<(usize, usize)> = vec![(root, 0)];
        while let Some(&mut (v, ref mut edge)) = work.last_mut() {
            if *edge < edges[v].len() {
                let w = edges[v][*edge];
                *edge += 1;
                if inside.contains(&w) && seen.insert(w) {
                    work.push((w, 0));
                }
                continue;
            }
            work.pop();
            out.push(v);
        }
    }
    out
}

/// Tarjan's SCC over `edges`, iteratively (a 100k-function binary would
/// overflow the stack recursing).  Returns the emission order — every component
/// after the components it points at — and each node's component size.
fn tarjan_scc(edges: &[Vec<usize>]) -> (Vec<usize>, Vec<usize>) {
    let n = edges.len();
    const UNVISITED: usize = usize::MAX;
    let mut index = vec![UNVISITED; n];
    let mut low = vec![0usize; n];
    let mut on_stack = vec![false; n];
    let mut stack: Vec<usize> = Vec::new();
    let mut next_index = 0usize;
    let mut order: Vec<usize> = Vec::with_capacity(n);
    let mut component_size = vec![1usize; n];
    for root in 0..n {
        if index[root] != UNVISITED {
            continue;
        }
        // (node, next edge to walk)
        let mut work: Vec<(usize, usize)> = vec![(root, 0)];
        index[root] = next_index;
        low[root] = next_index;
        next_index += 1;
        stack.push(root);
        on_stack[root] = true;
        while let Some(&mut (v, ref mut edge)) = work.last_mut() {
            if *edge < edges[v].len() {
                let w = edges[v][*edge];
                *edge += 1;
                if index[w] == UNVISITED {
                    index[w] = next_index;
                    low[w] = next_index;
                    next_index += 1;
                    stack.push(w);
                    on_stack[w] = true;
                    work.push((w, 0));
                } else if on_stack[w] {
                    low[v] = low[v].min(index[w]);
                }
                continue;
            }
            work.pop();
            if low[v] == index[v] {
                let mut members: Vec<usize> = Vec::new();
                while let Some(w) = stack.pop() {
                    on_stack[w] = false;
                    members.push(w);
                    if w == v {
                        break;
                    }
                }
                members.sort_unstable();
                for &w in &members {
                    component_size[w] = members.len();
                }
                order.extend(members);
            }
            if let Some(&mut (parent, _)) = work.last_mut() {
                low[parent] = low[parent].min(low[v]);
            }
        }
    }
    (order, component_size)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_chain_decompiles_callees_first_and_every_step_parks() {
        let edges = vec![vec![1], vec![2], vec![]];
        for cycles in [false, true] {
            assert_eq!(
                plan_from_components(&edges, &[false; 3], cycles),
                vec![(2, true), (1, true), (0, true)]
            );
        }
    }

    #[test]
    fn a_function_that_calls_itself_parks_only_under_cycles() {
        let edges = vec![vec![1], vec![]];
        let recursive = [false, true];
        assert_eq!(
            plan_from_components(&edges, &recursive, false),
            vec![(1, false), (0, true)]
        );
        assert_eq!(
            plan_from_components(&edges, &recursive, true),
            vec![(1, true), (0, true)]
        );
    }

    #[test]
    fn a_cycle_member_called_from_outside_decompiles_after_its_partner() {
        // 2 calls 0; 0 and 1 call each other.
        let edges = vec![vec![1], vec![0], vec![0]];
        assert_eq!(
            plan_from_components(&edges, &[false; 3], false),
            vec![(0, false), (1, false), (2, true)]
        );
        assert_eq!(
            plan_from_components(&edges, &[false; 3], true),
            vec![(1, true), (0, true), (2, true)]
        );
    }

    #[test]
    fn every_member_of_a_larger_cycle_decompiles_once_before_its_callers() {
        // 3 calls 0; 0 -> 1 -> 2 -> 0, and 2 also calls 1.
        let edges = vec![vec![1], vec![2], vec![0, 1], vec![0]];
        assert_eq!(
            plan_from_components(&edges, &[false; 4], true),
            vec![(2, true), (1, true), (0, true), (3, true)]
        );
    }

    #[test]
    fn the_cycle_order_starts_from_the_members_called_from_outside() {
        let edges = vec![vec![1], vec![0], vec![]];
        assert_eq!(
            cycle_order(&[0, 1], &edges, &[false, true, false]),
            vec![0, 1]
        );
        assert_eq!(
            cycle_order(&[0, 1], &edges, &[true, false, false]),
            vec![1, 0]
        );
        assert_eq!(
            cycle_order(&[0, 1], &edges, &[false, false, false]),
            vec![1, 0]
        );
    }

    #[test]
    fn small_graphs_keep_recursive_components_and_callee_order() {
        for nodes in 0..=4 {
            let pairs: Vec<_> = (0..nodes)
                .flat_map(|from| {
                    (0..nodes)
                        .filter(move |&to| from != to)
                        .map(move |to| (from, to))
                })
                .collect();
            for bits in 0usize..(1 << pairs.len()) {
                let mut edges = vec![Vec::new(); nodes];
                let mut reaches = vec![vec![false; nodes]; nodes];
                for node in 0..nodes {
                    reaches[node][node] = true;
                }
                for (bit, &(from, to)) in pairs.iter().enumerate() {
                    if bits & (1 << bit) != 0 {
                        edges[from].push(to);
                        reaches[from][to] = true;
                    }
                }
                for via in 0..nodes {
                    for from in 0..nodes {
                        for to in 0..nodes {
                            reaches[from][to] |= reaches[from][via] && reaches[via][to];
                        }
                    }
                }
                for cycles in [false, true] {
                    let plan = plan_from_components(&edges, &vec![false; nodes], cycles);
                    assert_eq!(plan.len(), nodes);
                    let mut position = vec![usize::MAX; nodes];
                    for (at, &(node, parks)) in plan.iter().enumerate() {
                        assert!(node < nodes);
                        assert_eq!(position[node], usize::MAX, "duplicate node");
                        position[node] = at;
                        let recursive = (0..nodes).any(|other| {
                            other != node && reaches[node][other] && reaches[other][node]
                        });
                        assert_eq!(parks, cycles || !recursive);
                    }
                    for from in 0..nodes {
                        for &to in &edges[from] {
                            if !reaches[to][from] {
                                assert!(position[to] < position[from], "callee follows caller");
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn a_hundred_thousand_function_chain_does_not_recurse() {
        let count = 100_000;
        let edges: Vec<Vec<usize>> = (0..count)
            .map(|node| {
                if node + 1 < count {
                    vec![node + 1]
                } else {
                    Vec::new()
                }
            })
            .collect();
        let expected: Vec<_> = (0..count).rev().map(|node| (node, true)).collect();
        for cycles in [false, true] {
            assert_eq!(
                plan_from_components(&edges, &vec![false; count], cycles),
                expected
            );
        }
    }
}
