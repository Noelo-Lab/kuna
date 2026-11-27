//! The live structuring components as a dense graph, for SAILR edge ordering.
//!
//! Node `i` is the `i`-th live component and node `n` a synthetic head with an
//! edge to every component that has no predecessor (angr's temporary
//! `postorder_head`). Parallel edges count once and self-loops count, as in a
//! `networkx.DiGraph`. Construction computes the post-order of the deterministic
//! walk from the head and the forward dominator tree as pre-order intervals.
//! A dominator tree is unique, so any exact algorithm gives the same relation.

use kuna_base::types::{int4, uintb};

const ABSENT: u32 = u32::MAX;

pub(super) struct CompGraph {
    addr: Vec<uintb>,
    succ_at: Vec<u32>,
    succs: Vec<u32>,
    pred_at: Vec<u32>,
    preds: Vec<u32>,
    seq: Vec<u32>,
    span: Vec<(u32, u32)>,
}

impl CompGraph {
    /// The graph of `addr.len()` components joined by the `(src, dst)` edges;
    /// `addr[i]` orders component `i` among siblings, ties broken by index.
    pub(super) fn new(addr: Vec<uintb>, mut edges: Vec<(u32, u32)>) -> CompGraph {
        let n = addr.len();
        edges.sort_unstable();
        edges.dedup();
        let mut indeg = vec![0u32; n];
        for &(_, d) in &edges {
            indeg[d as usize] += 1;
        }
        let head = n as u32;
        edges.extend((0..head).filter(|&v| indeg[v as usize] == 0).map(|v| (head, v)));
        let csr = |key: fn(&(u32, u32)) -> u32| {
            let mut at = vec![0u32; n + 2];
            for e in &edges {
                at[key(e) as usize + 1] += 1;
            }
            for v in 0..=n {
                at[v + 1] += at[v];
            }
            at
        };
        let succ_at = csr(|e| e.0);
        let pred_at = csr(|e| e.1);
        let mut fill = pred_at.clone();
        let mut preds = vec![0u32; edges.len()];
        for &(s, d) in &edges {
            preds[fill[d as usize] as usize] = s;
            fill[d as usize] += 1;
        }
        let succs = edges.iter().map(|e| e.1).collect();
        let mut g = CompGraph { addr, succ_at, succs, pred_at, preds, seq: vec![0; n], span: Vec::new() };
        let post = postorder(n + 1, head, |v, kids| {
            kids.extend_from_slice(g.succ(v));
            kids.sort_unstable_by_key(|&s| (g.addr[s as usize], s));
        });
        let idom = immediate_dominators(n + 1, &post, |v, ps| ps.extend_from_slice(g.pred(v)));
        for (i, &v) in post.iter().enumerate() {
            if v != head {
                g.seq[v as usize] = i as u32 + 1;
            }
        }
        g.span = dominator_spans(head, &idom);
        g
    }

    fn succ(&self, v: u32) -> &[u32] {
        &self.succs[self.succ_at[v as usize] as usize..self.succ_at[v as usize + 1] as usize]
    }

    fn pred(&self, v: u32) -> &[u32] {
        &self.preds[self.pred_at[v as usize] as usize..self.pred_at[v as usize + 1] as usize]
    }

    /// Components plus the head.
    pub(super) fn num_nodes(&self) -> int4 {
        self.addr.len() as int4 + 1
    }

    pub(super) fn size_in(&self, v: u32) -> int4 {
        self.pred(v).len() as int4
    }

    pub(super) fn size_out(&self, v: u32) -> int4 {
        self.succ(v).len() as int4
    }

    /// One plus `v`'s position in the post-order from the head (angr's
    /// `node_seq`: larger is closer to the head), 0 when the head cannot reach it.
    pub(super) fn node_seq(&self, v: u32) -> int4 {
        self.seq[v as usize] as int4
    }

    /// Whether `a` dominates `b` from the head; a node unreachable from the head
    /// dominates only itself.
    pub(super) fn dominates(&self, a: u32, b: u32) -> bool {
        if a == b {
            return true;
        }
        let (begin, end) = self.span[a as usize];
        let at = self.span[b as usize].0;
        begin != ABSENT && at != ABSENT && begin <= at && at < end
    }

    /// How many components have an immediate post-dominator other than the
    /// exit once the `s -> d` edge is gone (SAILR H2). Post-dominators are the
    /// dominators of the reversed component graph rooted at an exit that every
    /// component without successors leads to; the head takes no part.
    pub(super) fn postdominated_without(&self, s: u32, d: u32) -> usize {
        let n = self.addr.len();
        let exit = n as u32;
        let out = |v: u32| self.succ(v).iter().copied().filter(move |&w| (v, w) != (s, d));
        let post = postorder(n + 1, exit, |v, kids| {
            if v == exit {
                kids.extend((0..exit).filter(|&u| out(u).next().is_none()));
            } else {
                kids.extend(self.pred(v).iter().copied().filter(|&u| u != exit && (u, v) != (s, d)));
            }
        });
        let idom = immediate_dominators(n + 1, &post, |v, ps| {
            ps.extend(out(v));
            if ps.is_empty() {
                ps.push(exit);
            }
        });
        idom[..n].iter().filter(|&&p| p != ABSENT && p != exit).count()
    }
}

/// The post-order of a depth-first walk from `root` over `nodes` nodes, where
/// `succ` lists a node's successors and the last one listed is explored first.
fn postorder(nodes: usize, root: u32, succ: impl Fn(u32, &mut Vec<u32>)) -> Vec<u32> {
    let mut seen = vec![false; nodes];
    let mut post = Vec::with_capacity(nodes);
    let mut stack = vec![(root, true)];
    let mut kids = Vec::new();
    while let Some((v, pre)) = stack.pop() {
        if !pre {
            post.push(v);
            continue;
        }
        if seen[v as usize] {
            continue;
        }
        seen[v as usize] = true;
        stack.push((v, false));
        kids.clear();
        succ(v, &mut kids);
        stack.extend(kids.iter().filter(|&&k| !seen[k as usize]).map(|&k| (k, true)));
    }
    post
}

/// Each node's immediate dominator (Cooper-Harvey-Kennedy), `ABSENT` for a node
/// `post` does not reach; `post` is a depth-first post-order ending at the root,
/// which dominates itself, and `pred` lists a node's predecessors.
fn immediate_dominators(nodes: usize, post: &[u32], pred: impl Fn(u32, &mut Vec<u32>)) -> Vec<u32> {
    let mut at = vec![ABSENT; nodes];
    for (i, &v) in post.iter().enumerate() {
        at[v as usize] = i as u32;
    }
    let root = post.len() - 1;
    let mut idom = vec![ABSENT; post.len()];
    idom[root] = root as u32;
    let mut ps = Vec::new();
    let mut changed = true;
    while changed {
        changed = false;
        for i in (0..root).rev() {
            ps.clear();
            pred(post[i], &mut ps);
            let mut new = ABSENT;
            for &p in &ps {
                let mut f1 = at[p as usize];
                if f1 == ABSENT || idom[f1 as usize] == ABSENT {
                    continue;
                }
                if new != ABSENT {
                    let mut f2 = new;
                    while f1 != f2 {
                        while f1 < f2 {
                            f1 = idom[f1 as usize];
                        }
                        while f2 < f1 {
                            f2 = idom[f2 as usize];
                        }
                    }
                }
                new = f1;
            }
            if new != ABSENT && idom[i] != new {
                idom[i] = new;
                changed = true;
            }
        }
    }
    let mut res = vec![ABSENT; nodes];
    for (i, &v) in post.iter().enumerate() {
        res[v as usize] = post[idom[i] as usize];
    }
    res
}

/// Pre-order `(entry, exit)` clocks of the dominator tree rooted at `root`, so
/// `a` dominates `b` exactly when `b`'s entry lies in `a`'s interval.
fn dominator_spans(root: u32, idom: &[u32]) -> Vec<(u32, u32)> {
    let mut first = vec![ABSENT; idom.len()];
    let mut next = vec![ABSENT; idom.len()];
    for (v, &p) in idom.iter().enumerate() {
        if p != ABSENT && p as usize != v {
            next[v] = first[p as usize];
            first[p as usize] = v as u32;
        }
    }
    let mut span = vec![(ABSENT, ABSENT); idom.len()];
    let mut clock = 0;
    let mut stack = vec![(root, false)];
    while let Some((v, exiting)) = stack.pop() {
        if exiting {
            span[v as usize].1 = clock;
            continue;
        }
        span[v as usize] = (clock, clock);
        clock += 1;
        stack.push((v, true));
        let mut c = first[v as usize];
        while c != ABSENT {
            stack.push((c, false));
            c = next[c as usize];
        }
    }
    span
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::p7_regions::kuna_regiongraph::{
        kuna_dfs_postorder_deterministic, kuna_immediate_dominators, KunaIncrementalDominators, KunaNodeId,
        KunaNodePool, KunaRegionGraph, NodeKind,
    };

    /// The component graph as the region-graph library builds it: a pool node
    /// per component keyed `(addr, index)`, then the head.
    fn region_graph(addr: &[uintb], edges: &[(u32, u32)]) -> (KunaNodePool, KunaRegionGraph, Vec<KunaNodeId>, KunaNodeId) {
        let mut pool = KunaNodePool::new();
        let mut graph = KunaRegionGraph::new();
        let ids: Vec<_> = addr.iter().enumerate().map(|(i, &a)| pool.make(NodeKind::Block, a, i as u32)).collect();
        for &id in &ids {
            graph.add_node(&pool, id);
        }
        for &(s, d) in edges {
            graph.add_edge(&pool, ids[s as usize], ids[d as usize]);
        }
        let head = pool.make(NodeKind::Dummy, 0, u32::MAX);
        graph.add_node(&pool, head);
        for &id in &ids {
            if graph.size_in(id).unwrap() == 0 {
                graph.add_edge(&pool, head, id);
            }
        }
        (pool, graph, ids, head)
    }

    /// SAILR H2 over the region graph: rebuild the reversed graph without
    /// `s -> d`, rooted at an exit every sink leads to, and count the
    /// components whose immediate post-dominator is not the exit.
    fn region_postdominated(pool: &KunaNodePool, graph: &KunaRegionGraph, ids: &[KunaNodeId], s: u32, d: u32) -> usize {
        let mut lpool = pool.clone();
        let exit = lpool.make(NodeKind::Dummy, u64::MAX, u32::MAX);
        let mut rev = KunaRegionGraph::new();
        for &id in ids {
            rev.add_node(&lpool, id);
        }
        rev.add_node(&lpool, exit);
        for &src in ids {
            let mut has_succ = false;
            for &dst in graph.get_succs(src).unwrap() {
                if (src, dst) != (ids[s as usize], ids[d as usize]) {
                    has_succ = true;
                    rev.add_edge(&lpool, dst, src);
                }
            }
            if !has_succ {
                rev.add_edge(&lpool, exit, src);
            }
        }
        let mut idom = std::collections::BTreeMap::new();
        kuna_immediate_dominators(&lpool, &rev, exit, &mut idom).unwrap();
        ids.iter().filter(|id| idom.get(id).is_some_and(|&p| p != **id && p != exit)).count()
    }

    #[test]
    fn agrees_with_the_region_graph_algorithms() {
        let mut seed = 0x2545_f491_4f6c_dd1du64;
        let mut next = move |m: u64| {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            seed % m
        };
        for _ in 0..4000 {
            let n = 1 + next(12) as usize;
            let addr: Vec<uintb> = (0..n).map(|_| next(4) * 0x10).collect();
            let edges: Vec<(u32, u32)> =
                (0..next(3 * n as u64 + 1)).map(|_| (next(n as u64) as u32, next(n as u64) as u32)).collect();
            let g = CompGraph::new(addr.clone(), edges.clone());
            let (pool, graph, ids, head) = region_graph(&addr, &edges);
            assert_eq!(g.num_nodes(), graph.num_nodes());
            let doms = KunaIncrementalDominators::new(&pool, &graph, head, false).unwrap();
            let mut post = Vec::new();
            kuna_dfs_postorder_deterministic(&pool, &graph, head, &mut post).unwrap();
            for v in 0..n as u32 {
                let id = ids[v as usize];
                assert_eq!(g.size_in(v), graph.size_in(id).unwrap(), "{addr:?} {edges:?}");
                assert_eq!(g.size_out(v), graph.size_out(id).unwrap(), "{addr:?} {edges:?}");
                let seq = post.iter().position(|&p| p == id).map_or(0, |i| i as int4 + 1);
                assert_eq!(g.node_seq(v), seq, "{addr:?} {edges:?}");
                for w in 0..n as u32 {
                    assert_eq!(g.dominates(v, w), doms.dominates(id, ids[w as usize]), "{addr:?} {edges:?} {v} {w}");
                    if v != w && graph.has_edge(id, ids[w as usize]) {
                        assert_eq!(
                            g.postdominated_without(v, w),
                            region_postdominated(&pool, &graph, &ids, v, w),
                            "{addr:?} {edges:?} {v} {w}"
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn deep_chain_uses_iterative_walks() {
        let n = 10_000u32;
        let edges: Vec<_> = (1..n).map(|v| (v - 1, v)).collect();
        let g = CompGraph::new(vec![0; n as usize], edges);
        for v in [0, 1, n / 2, n - 1] {
            assert!(g.dominates(0, v));
            assert!(g.dominates(v, n - 1));
            assert_eq!(g.dominates(v, 0), v == 0);
            assert_eq!(g.node_seq(v), (n - v) as int4);
        }
        assert_eq!(g.postdominated_without(0, 1), n as usize - 2);
    }
}
