//! K5 direct graph, var-map holder, var processor and SSA constructors.
//!
//! These need `Statement`, `Exprent`, `StructMethod` and `SFormsFastMapDirect`
//! instances, which only other slices can build. Until they merge the file is
//! compiled only with `RUSTFLAGS='--cfg kuna_jvm_k5_stubs'` against K5's local
//! throw-away stubs; at integration, rebind the four constructors in the
//! `support` block to the real APIs and drop the `cfg`.
#![cfg(kuna_jvm_k5_stubs)]

use kuna_jvm::direct_edge::DirectEdge;
use kuna_jvm::direct_edge_type::DirectEdgeType;
use kuna_jvm::direct_graph::DirectGraph;
use kuna_jvm::direct_node::DirectNode;
use kuna_jvm::direct_node_type::DirectNodeType;
use kuna_jvm::exprent::Exprent;
use kuna_jvm::rt::{try_catch, JList, JavaEq, JavaHash, Shared};
use kuna_jvm::s_forms_fast_map_direct::SFormsFastMapDirect;
use kuna_jvm::ssa_constructor_sparse_ex::SSAConstructorSparseEx;
use kuna_jvm::ssau_constructor_sparse_ex::SSAUConstructorSparseEx;
use kuna_jvm::var_map_holder::VarMapHolder;
use kuna_jvm::var_names_collector::VarNamesCollector;
use kuna_jvm::var_processor::VarProcessor;
use kuna_jvm::var_type_processor::FinalType;
use kuna_jvm::var_version_node::State;
use kuna_jvm::var_version_pair::VarVersionPair;

mod support {
    use kuna_jvm::fast_sparse_set_factory::FastSparseSetFactory;
    use kuna_jvm::method_descriptor::MethodDescriptor;
    use kuna_jvm::rt::Shared;
    use kuna_jvm::statement::{Statement, StatementKind};
    use kuna_jvm::struct_method::StructMethod;
    use kuna_jvm::var_exprent::VarExprent;

    pub fn stat(id: i32) -> Statement {
        Statement::new_stub(id, StatementKind::General)
    }

    pub fn dummy_exit(id: i32) -> Statement {
        Statement::new_stub(
            id,
            StatementKind::DummyExit(kuna_jvm::dummy_exit_statement::DummyExitStatementData),
        )
    }

    pub fn var(id: i32, index: i32, version: i32) -> VarExprent {
        VarExprent::new_stub(id, index, version)
    }

    pub fn method() -> (StructMethod, MethodDescriptor) {
        (
            StructMethod::new_stub("m"),
            MethodDescriptor::new_stub("()V"),
        )
    }

    pub fn factory() -> Shared<FastSparseSetFactory<i32>> {
        Shared::new(FastSparseSetFactory::new((0..64).collect()))
    }
}

use support::{dummy_exit, stat, var};

const GOLDEN: &str = include_str!("data/k5/k5_golden.txt");

fn golden(tag: &str) -> Vec<Vec<&'static str>> {
    GOLDEN
        .lines()
        .filter_map(|l| l.strip_prefix(tag).and_then(|r| r.strip_prefix(' ')))
        .map(|r| r.split(' ').collect())
        .collect()
}

fn golden_line(tag: &str) -> String {
    golden(tag).into_iter().next().expect(tag).join(" ")
}

fn ids(nodes: &[DirectNode]) -> String {
    nodes.iter().map(|n| n.id()).collect::<Vec<_>>().join(" ")
}

#[test]
fn direct_node_hash_and_equals_match_java() {
    for row in golden("dnode") {
        let t = DirectNodeType::values()[row[0].parse::<usize>().unwrap()];
        let node = DirectNode::for_stat(t, stat(row[1].parse().unwrap()), None);
        assert_eq!(node.id(), row[2]);
        assert_eq!(node.to_string(), row[2]);
        let reduced = node
            .hash_code()
            .wrapping_sub(31i32.wrapping_mul(t.java_hash()));
        assert_eq!(reduced, row[3].parse::<i32>().unwrap(), "{node}");
        assert_eq!(node.java_hash(), node.hash_code());
    }
    let na = DirectNode::for_stat(DirectNodeType::Try, stat(5), None);
    let nb = DirectNode::for_stat(DirectNodeType::Try, stat(5), None);
    let nc = DirectNode::for_stat(DirectNodeType::Catch, stat(5), None);
    let nd = DirectNode::for_stat(DirectNodeType::Try, stat(6), None);
    let eq = format!(
        "{} {} {} {}",
        na.equals(nb),
        na.equals(nc),
        na.equals(nd),
        na.java_equals(&na)
    );
    assert_eq!(eq, golden_line("dnode_eq"));
    assert!(na != nb, "handle == is identity");
}

#[test]
fn direct_edge_hash_equals_and_text_match_java() {
    for row in golden("dedge") {
        let num = |i: usize| row[i].parse::<i32>().unwrap();
        let st = DirectNodeType::values()[num(0) as usize];
        let dt = DirectNodeType::values()[num(2) as usize];
        let et = DirectEdgeType::values()[num(4) as usize];
        let make = |et| {
            DirectEdge::new(
                DirectNode::for_stat(st, stat(num(1)), None),
                DirectNode::for_stat(dt, stat(num(3)), None),
                et,
            )
        };
        let edge = make(et);
        let enum_part = 29791i32
            .wrapping_mul(st.java_hash())
            .wrapping_add(961i32.wrapping_mul(dt.java_hash()))
            .wrapping_add(et.java_hash());
        assert_eq!(edge.hash_code().wrapping_sub(enum_part), num(5), "{edge}");
        assert_eq!(edge.equals(&make(et)), row[6] == "true");
        assert_eq!(
            edge == make(DirectEdgeType::values()[1 - num(4) as usize]),
            row[7] == "true"
        );
        assert_eq!(edge.to_string(), row[8..].join(" "));
    }
}

struct Graph {
    g: DirectGraph,
    n: Vec<DirectNode>,
}

fn build_graph() -> Graph {
    let g = DirectGraph::new();
    let mut n: Vec<DirectNode> = vec![DirectNode::for_stat(DirectNodeType::Direct, stat(0), None)];
    for i in 1..=8 {
        n.push(DirectNode::for_stat(DirectNodeType::Direct, stat(i), None));
    }
    n.push(DirectNode::for_stat(
        DirectNodeType::Direct,
        dummy_exit(9),
        None,
    ));
    for i in [3, 1, 9, 6, 2, 5, 4, 8, 7] {
        let node = n[i];
        g.with_mut(|d| d.nodes.add_with_key(node, node.id()));
    }
    g.set_first(Some(n[1]));
    for (a, b) in [
        (1, 2),
        (1, 3),
        (2, 4),
        (3, 4),
        (4, 2),
        (4, 5),
        (1, 3),
        (5, 7),
        (3, 7),
    ] {
        n[a].add_successor(DirectEdge::of(n[a], n[b]));
    }
    n[2].add_successor(DirectEdge::exception(n[2], n[6]));
    n[8].add_successor(DirectEdge::of(n[8], n[7]));
    Graph { g, n }
}

#[test]
fn direct_graph_adjacency_and_order_match_java() {
    let Graph { g, n } = build_graph();
    for row in golden("dg_adj") {
        let i: usize = row[0].parse().unwrap();
        let succ: Vec<DirectNode> = n[i]
            .get_successors(DirectEdgeType::Regular)
            .iter()
            .map(|e| e.get_destination())
            .collect();
        let pred: Vec<DirectNode> = n[i]
            .get_predecessors(DirectEdgeType::Regular)
            .iter()
            .map(|e| e.get_source())
            .collect();
        let line = format!(
            "{} [{}] [{}] {} {}",
            i,
            ids(&succ),
            ids(&pred),
            n[i].has_successors(DirectEdgeType::Exception),
            n[i].has_predecessors(DirectEdgeType::Exception)
        );
        assert_eq!(line, row.join(" "));
    }

    g.sort_reverse_post_order();
    assert_eq!(ids(&g.nodes()), golden_line("dg_rpo"));
    assert_eq!(ids(&g.extra_nodes()), golden_line("dg_extra"));
    assert_eq!(
        g.with(|d| d.nodes.get_with_key(&"4".to_string())),
        Some(n[4])
    );
}

#[test]
fn direct_graph_iterate_exprents_matches_java() {
    let Graph { g, n } = build_graph();
    g.sort_reverse_post_order();
    let mut next_id = 100;
    for node in n.iter().skip(1).take(8) {
        let i: i32 = node.id().parse().unwrap();
        let lst: JList<Exprent> = JList::new();
        for k in 0..3 {
            next_id += 1;
            lst.add(var(next_id, i * 10 + k, 0).0);
        }
        node.set_exprents(lst);
    }
    let index = |e: Exprent| e.as_var().get_index();

    let mut seen = Vec::new();
    let full = g.iterate_exprents(|e: Exprent| {
        let idx = index(e);
        seen.push(idx.to_string());
        if idx % 10 == 1 {
            2
        } else {
            0
        }
    });
    assert_eq!(
        format!("{} {}", full, seen.join(" ")),
        golden_line("dg_iter")
    );
    let left: Vec<String> = n[1..=8]
        .iter()
        .map(|x| x.exprents().len().to_string())
        .collect();
    assert_eq!(left.join(" "), golden_line("dg_left"));

    let mut seen2 = Vec::new();
    let full2 = g.iterate_exprents(|e: Exprent| {
        let idx = index(e);
        seen2.push(idx.to_string());
        if idx == 40 {
            1
        } else {
            0
        }
    });
    assert_eq!(
        format!("{} {}", full2, seen2.join(" ")),
        golden_line("dg_cancel")
    );

    let mut deep = 0;
    assert!(g.iterate_exprents_deep(|_e: Exprent| {
        deep += 1;
        0
    }));
    assert_eq!(deep, 12);
    let caught = try_catch(|| g.iterate_exprents_deep(|_e: Exprent| 2)).unwrap_err();
    assert!(caught.is("IllegalStateException"), "{}", caught.message);
    assert_eq!(
        caught.message,
        "IllegalStateException: Unsupported operation: 2"
    );
}

fn map() -> Shared<SFormsFastMapDirect> {
    Shared::new(SFormsFastMapDirect::new(support::factory()))
}

#[test]
fn var_map_holder_state_machine() {
    let base = map();
    base.borrow_mut().set_current_var(1, 1);
    let mut h = VarMapHolder::of_normal(&base);
    assert!(h.is_normal());
    assert!(!h.get_normal().ptr_eq(&base), "ofNormal copies");
    assert!(h.get_if_true().ptr_eq(&h.get_if_false()));

    h.make_fully_mutable();
    assert!(!h.is_normal());
    let (t, f) = (h.get_if_true(), h.get_if_false());
    assert!(!t.ptr_eq(&f));
    h.swap();
    assert!(h.get_if_true().ptr_eq(&f) && h.get_if_false().ptr_eq(&t));

    let other = map();
    other.borrow_mut().set_current_var(2, 3);
    h.merge_if_true(Some(&other));
    assert!(h.get_if_true().borrow().map.contains_key(&2));
    assert!(!h.get_if_false().borrow().map.contains_key(&2));
    h.merge_if_true(Some(&h.get_if_true()));
    h.merge_if_false(None);

    let merged = h.to_normal();
    assert!(h.is_normal());
    assert!(merged.ptr_eq(&h.get_normal()));
    assert!(merged.borrow().map.contains_key(&2));

    let mut h2 = VarMapHolder::of_normal(&base);
    h2.set(&h);
    assert!(h2.get_normal().ptr_eq(&h.get_normal()), "set aliases");

    h2.set_if_true(other.clone());
    assert!(!h2.is_normal());
    assert!(h2.get_if_false().ptr_eq(&merged));
    h2.set_if_true(merged.clone());
    assert!(h2.is_normal());

    h2.set_normal(other.clone());
    let caught = try_catch(|| {
        let mut bad = VarMapHolder::of_normal(&base);
        let t = bad.get_if_true();
        bad.set_if_false(t);
        let _ = bad.get_if_true();
    })
    .unwrap_err();
    assert!(caught
        .message
        .contains("VarMapHolder is in an illegal state"));

    let same = map();
    same.borrow_mut().set_current_var(-3, 1);
    let r = VarMapHolder::merge_maps(&same, Some(&same));
    assert!(r.ptr_eq(&same));
    h2.set_normal(same.clone());
    h2.remove_all_fields();
    assert!(same.borrow().is_empty());
}

#[test]
fn var_processor_accessors() {
    let (mt, md) = support::method();
    let vp = VarProcessor::new(mt, &md);
    let p = VarVersionPair::new(3, 0);

    assert_eq!(vp.get_var_name(p), None);
    vp.set_var_name(p, "x");
    vp.set_inherited_name(VarVersionPair::new(4, 0), "y");
    assert_eq!(vp.get_var_name(p).as_deref(), Some("x"));
    assert_eq!(vp.get_var_names(), vec!["y".to_string(), "x".to_string()]);
    assert_eq!(
        vp.get_used_var_versions(),
        vec![VarVersionPair::new(4, 0), p]
    );
    assert_eq!(vp.get_inherited_names().borrow().len(), 1);

    assert_eq!(vp.get_var_final(p), FinalType::Final);
    vp.set_var_final(p, FinalType::NonFinal);
    assert_eq!(
        vp.get_var_final(p),
        FinalType::Final,
        "no versions processor yet"
    );
    assert!(vp.get_var_type(p).is_none());
    assert_eq!(vp.get_var_original_index(3), None);
    assert!(vp.get_var_versions().is_none());

    vp.get_this_vars()
        .borrow_mut()
        .put(VarVersionPair::new(0, 0), "a.B".to_string());
    assert_eq!(
        vp.get_this_vars()
            .borrow()
            .get(&VarVersionPair::new(0, 0))
            .map(String::as_str),
        Some("a.B")
    );
    vp.get_external_vars().borrow_mut().add(p);
    assert!(vp.get_external_vars().borrow().contains(&p));
    vp.get_synthetic_semaphores().borrow_mut().add(7);
    assert!(vp.get_synthetic_semaphores().borrow().contains(&7));

    vp.mark_param(p);
    assert_eq!(vp.get_params(), vec![p]);
    vp.set_clashing_name(p, "x1");
    assert_eq!(vp.get_clashing_name(p).as_deref(), Some("x1"));
    assert_eq!(vp.clashing_names(), vec!["x1".to_string()]);

    vp.set_var_source(p, "m()V", VarVersionPair::new(1, 1));
    let src = vp.get_var_source(p).expect("source");
    assert_eq!((src.a.as_str(), src.b), ("m()V", VarVersionPair::new(1, 1)));

    assert!(!vp.has_lvt());
    assert!(vp.get_candidates(0).is_none());
    vp.find_lvt(var(1, 3, 0), 0);

    vp.get_var_names_collector().add_name("x");
    let vc = VarNamesCollector::new_with_names(["x".to_string()]);
    vp.refresh_var_names(vc);
    assert_eq!(vp.get_var_name(p).as_deref(), Some("xx"));
    assert_eq!(
        vp.get_var_name(VarVersionPair::new(4, 0)).as_deref(),
        Some("y")
    );
    assert!(!vp.nested_processed());
    vp.set_nested_processed(true);
    assert!(vp.with(|d| d.nested_processed));
}

#[test]
fn ssa_constructor_small_bodies() {
    let mut ssa = SSAConstructorSparseEx::new();
    assert!(ssa.is_ssa() && !ssa.track_field_vars && !ssa.track_direct_assignments);
    let p = VarVersionPair::new(5, 2);
    assert_eq!(ssa.get_or_create_phantom(p), p);
    ssa.mark_direct_assignment(p, p);
    assert!(ssa.get_phi().is_empty());

    let v = var(1, 5, 0);
    ssa.init_version(v, stat(1));
    assert_eq!(v.get_version(), 1);
    ssa.init_version(v, stat(1));
    assert_eq!(v.get_version(), 1, "already versioned");
    let w = var(2, 5, 0);
    ssa.init_version(w, stat(1));
    assert_eq!(w.get_version(), 2);
    assert_eq!(ssa.get_next_free_version(6, None), 1);

    let m = map();
    ssa.var_read_single_version(stat(1), false, w, &m, 7);
    assert_eq!(w.get_version(), 7);
    assert_eq!(ssa.get_read_count(p), 0);
    assert!(ssa.get_direct_graph().is_none());
}

#[test]
fn ssau_constructor_small_bodies() {
    let mut ssau = SSAUConstructorSparseEx::new();
    assert!(ssau.is_ssau() && ssau.track_field_vars && ssau.track_direct_assignments);
    ssau.factory = Some(support::factory());

    let a = VarVersionPair::new(1, 1);
    let b = VarVersionPair::new(2, 1);
    ssau.mark_direct_assignment(a, b);
    assert_eq!(ssau.get_var_assignment_map().get(&a), Some(&b));

    let v = var(1, 4, 0);
    ssau.init_version(v, stat(10));
    assert_eq!(v.get_version(), 1);
    let node = ssau.get_node(v).expect("node");
    assert_eq!(node.state(), Some(State::Write));
    assert_eq!(ssau.get_ssu_versions().nodes().len(), 1);

    let phantom = ssau.get_or_create_phantom(VarVersionPair::new(4, 1));
    assert_eq!(phantom, VarVersionPair::new(4, 2));
    assert_eq!(
        ssau.get_or_create_phantom(VarVersionPair::new(4, 1)),
        phantom
    );
    let pnode = ssau.get_ssu_versions().get_node_with_key(phantom).unwrap();
    assert_eq!(pnode.phantom_parent_node(), Some(node));
    assert_eq!(pnode.state(), Some(State::Phantom));

    let m = map();
    let r = var(2, 4, 0);
    ssau.var_read_single_version(stat(10), false, r, &m, 1);
    assert_eq!(r.get_version(), 3);
    let read = ssau.get_node(r).unwrap();
    assert_eq!(read.state(), Some(State::Read));
    assert_eq!(read.get_predecessors(), vec![node]);
    assert!(m.borrow().map.contains_key(&4));

    ssau.var_read_single_version(stat(10), true, r, &m, 1);
    let live = ssau
        .get_live_var_versions_map(VarVersionPair::new(4, 3))
        .expect("live");
    assert!(live.ptr_eq(&read.live().unwrap()), "the node's own map");
    assert!(ssau
        .get_live_var_versions_map(VarVersionPair::new(9, 9))
        .is_none());
    assert!(ssau
        .get_live_var_versions_map(VarVersionPair::new(4, 1))
        .is_some_and(|l| l.borrow().is_empty()));

    let p = map();
    ssau.init_parameter(0, &p, true);
    let param = ssau
        .get_ssu_versions()
        .get_node_with_key(VarVersionPair::new(0, 1))
        .unwrap();
    assert_eq!(param.state(), Some(State::Catch));
    assert!(ssau.get_map_version_first_range().is_empty());
    assert!(ssau.get_map_field_vars().is_empty());
    ssau.get_map_field_vars_mut().put(3, -1);
    assert_eq!(ssau.get_map_field_vars().get(&3), Some(&-1));
}
