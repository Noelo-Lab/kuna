//! `GenericDominatorEngine` on hand-built graphs.

use kuna_jvm::generic_dominator_engine::GenericDominatorEngine;
use kuna_jvm::i_graph::IGraph;
use kuna_jvm::i_graph_node::IGraphNode;
use kuna_jvm::rt::{try_catch, JHashSet};
use kuna_jvm::{arena_identity, arena_object};

struct NodeData {
    name: char,
    preds: Vec<Node>,
    succs: Vec<Node>,
}
arena_object!(Node => NodeData, Class);
arena_identity!(Node);

impl IGraphNode for Node {
    fn get_predecessors(&self) -> Vec<Node> {
        self.with(|d| d.preds.clone())
    }
}

struct Graph {
    nodes: Vec<Node>,
    roots: Vec<Node>,
}

impl Graph {
    fn node(&self, name: char) -> Node {
        *self
            .nodes
            .iter()
            .find(|n| n.with(|d| d.name) == name)
            .unwrap_or_else(|| panic!("no node {name}"))
    }

    /// Reverse post-order from the roots, successors in edge order.
    fn rpo(&self) -> Vec<Node> {
        let mut visited: Vec<Node> = Vec::new();
        let mut post: Vec<Node> = Vec::new();
        fn dfs(n: Node, visited: &mut Vec<Node>, post: &mut Vec<Node>) {
            visited.push(n);
            for s in n.with(|d| d.succs.clone()) {
                if !visited.contains(&s) {
                    dfs(s, visited, post);
                }
            }
            post.push(n);
        }
        for &r in &self.roots {
            if !visited.contains(&r) {
                dfs(r, &mut visited, &mut post);
            }
        }
        post.reverse();
        post
    }
}

struct EngineGraph {
    rpo: Vec<Node>,
    roots: Vec<Node>,
}

impl IGraph<Node> for EngineGraph {
    fn get_reverse_post_order_list(&self) -> Vec<Node> {
        self.rpo.clone()
    }

    fn get_roots(&self) -> JHashSet<Node> {
        JHashSet::from_collection(self.roots.clone())
    }
}

/// `edges` like "AB AC BD": nodes are created in first-mention order.
fn graph(edges: &str, roots: &str) -> Graph {
    let mut nodes: Vec<Node> = Vec::new();
    let get = |c: char, nodes: &mut Vec<Node>| -> Node {
        if let Some(n) = nodes.iter().find(|n| n.with(|d| d.name) == c) {
            return *n;
        }
        let n = Node::alloc(NodeData {
            name: c,
            preds: Vec::new(),
            succs: Vec::new(),
        });
        nodes.push(n);
        n
    };
    for e in edges.split_whitespace() {
        let mut cs = e.chars();
        let (a, b) = (cs.next().unwrap(), cs.next().unwrap());
        let (a, b) = (get(a, &mut nodes), get(b, &mut nodes));
        a.with_mut(|d| d.succs.push(b));
        b.with_mut(|d| d.preds.push(a));
    }
    let roots = roots.chars().map(|c| get(c, &mut nodes)).collect();
    Graph { nodes, roots }
}

fn engine(g: &Graph) -> GenericDominatorEngine<Node> {
    let mut e = GenericDominatorEngine::new(Box::new(EngineGraph {
        rpo: g.rpo(),
        roots: g.roots.clone(),
    }));
    e.initialize();
    e
}

/// Every node's dominators, as "X:dominators" sorted by name.
fn dominators(g: &Graph, e: &GenericDominatorEngine<Node>) -> String {
    let mut names: Vec<char> = g.rpo().iter().map(|n| n.with(|d| d.name)).collect();
    names.sort();
    names
        .iter()
        .map(|&x| {
            let doms: String = names
                .iter()
                .filter(|&&d| e.is_dominator(g.node(x), g.node(d)))
                .collect();
            format!("{x}:{doms}")
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[test]
fn diamond() {
    let g = graph("AB AC BD CD", "A");
    let e = engine(&g);
    assert_eq!(dominators(&g, &e), "A:A B:AB C:AC D:AD");
}

#[test]
fn loop_with_exit() {
    let g = graph("AB BC CB CD DE BE", "A");
    let e = engine(&g);
    assert_eq!(dominators(&g, &e), "A:A B:AB C:ABC D:ABCD E:ABE");
}

#[test]
fn irreducible_loop() {
    let g = graph("AB AC BC CB BD", "A");
    let e = engine(&g);
    assert_eq!(dominators(&g, &e), "A:A B:AB C:AC D:ABD");
}

#[test]
fn nested_loops_and_self_loop() {
    let g = graph("AB BC CC CD DB DE BF EF", "A");
    let e = engine(&g);
    assert_eq!(dominators(&g, &e), "A:A B:AB C:ABC D:ABCD E:ABCDE F:ABF");
}

#[test]
fn two_roots_merge_without_idom() {
    let g = graph("RX SX XY SZ", "RS");
    let e = engine(&g);
    assert_eq!(dominators(&g, &e), "R:R S:S X:X Y:XY Z:SZ");
}

#[test]
fn predecessors_outside_the_order_are_ignored() {
    let g = graph("AB UB BC", "A");
    let e = GenericDominatorEngine::new(Box::new(EngineGraph {
        rpo: vec![g.node('A'), g.node('B'), g.node('C')],
        roots: vec![g.node('A')],
    }));
    let mut e = e;
    e.initialize();
    assert!(e.is_dominator(g.node('C'), g.node('A')));
    assert!(e.is_dominator(g.node('B'), g.node('A')));
    assert!(!e.is_dominator(g.node('B'), g.node('U')));
}

#[test]
fn misuse_throws_like_vineflower() {
    let g = graph("AB", "A");
    let mut e = GenericDominatorEngine::new(Box::new(EngineGraph {
        rpo: g.rpo(),
        roots: g.roots.clone(),
    }));
    let (a, b) = (g.node('A'), g.node('B'));
    let caught = try_catch(|| e.is_dominator(b, a)).unwrap_err();
    assert!(caught.is("IllegalStateException"));
    assert_eq!(
        caught.message,
        "IllegalStateException: GenericDominatorEngine not initialized!"
    );

    e.initialize();
    let caught = try_catch(|| e.initialize()).unwrap_err();
    assert!(caught.is("IllegalStateException"));
    assert!(caught
        .message
        .ends_with("Dominator Engine was already initialized"));

    let stranger = graph("ZY", "Z").node('Y');
    let caught = try_catch(|| e.is_dominator(stranger, a)).unwrap_err();
    assert!(caught.is("RuntimeException"));
    assert!(caught
        .message
        .ends_with("Inconsistent idom sequence discovered or node not found in dom graph!"));
}
