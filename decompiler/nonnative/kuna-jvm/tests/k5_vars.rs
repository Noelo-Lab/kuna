//! K5 values and variable graphs against vectors from the real Vineflower
//! 1.12.0 classes (`data/k5/K5Golden.java` -> `data/k5/k5_golden.txt`).

use kuna_jvm::direct_edge_type::DirectEdgeType;
use kuna_jvm::direct_node_type::DirectNodeType;
use kuna_jvm::rt::{JHashSet, JavaEq, JavaHash};
use kuna_jvm::s_forms_constructor::SFormsConstructor;
use kuna_jvm::var_names_collector::VarNamesCollector;
use kuna_jvm::var_type_processor::FinalType;
use kuna_jvm::var_version_node::{State, VarVersionNode};
use kuna_jvm::var_version_pair::VarVersionPair;
use kuna_jvm::var_versions_graph::VarVersionsGraph;

const GOLDEN: &str = include_str!("data/k5/k5_golden.txt");

fn golden(tag: &str) -> Vec<Vec<&'static str>> {
    GOLDEN
        .lines()
        .filter_map(|l| l.strip_prefix(tag).and_then(|r| r.strip_prefix(' ')))
        .map(|r| r.split(' ').collect())
        .collect()
}

fn golden_line(tag: &str) -> &'static str {
    GOLDEN
        .lines()
        .find_map(|l| l.strip_prefix(tag).and_then(|r| r.strip_prefix(' ')))
        .unwrap_or_else(|| panic!("no golden line {tag}"))
}

fn parse_pair(s: &str) -> VarVersionPair {
    let (a, b) = s.split_once(',').unwrap();
    VarVersionPair::new(a.parse().unwrap(), b.parse().unwrap())
}

#[test]
fn var_version_pair_matches_java() {
    let rows = golden("vvp");
    assert_eq!(rows.len(), 8);
    for row in rows {
        let var: i32 = row[0].parse().unwrap();
        let version: i32 = row[1].parse().unwrap();
        let p = VarVersionPair::new(var, version);
        assert_eq!(p.hash_code(), row[2].parse::<i32>().unwrap(), "{p}");
        assert_eq!(p.java_hash(), p.hash_code());
        assert_eq!(p.to_string(), row[3]);
        assert_eq!(
            p.java_equals(&VarVersionPair::new(var, version)),
            row[4] == "true"
        );
        assert_eq!(
            p.equals(&VarVersionPair::new(var, version.wrapping_add(1))),
            row[5] == "true"
        );
        assert!(p.is(var, version));
    }
}

#[test]
fn var_version_pair_set_order_matches_java() {
    let mut set: JHashSet<VarVersionPair> = JHashSet::new();
    for s in golden_line("vvpset_insert").split(' ') {
        set.add(parse_pair(s));
    }
    set.remove(&VarVersionPair::new(-9, 0));
    let order: Vec<String> = set
        .iter()
        .map(|p| format!("{},{}", p.var, p.version))
        .collect();
    assert_eq!(order.join(" "), golden_line("vvpset_order"));
}

#[test]
fn var_names_collector_matches_java() {
    let vc = VarNamesCollector::new_with_names(["var1", "this", "var1x"].map(String::from));
    let free = [
        vc.get_free_name_index(1),
        vc.get_free_name("this"),
        vc.get_free_name_index(2),
        vc.get_free_name("var1"),
        vc.get_free_name("var2"),
    ];
    vc.add_name("a");
    vc.add_names(["b", "c", "a"].map(String::from));
    for i in 0..30 {
        vc.get_free_name_index(i % 7);
    }
    assert_eq!(free.join(" "), golden_line("vnc_free"));
    assert_eq!(vc.get_used_names().join(" "), golden_line("vnc_used"));
}

#[test]
fn direct_node_type_ids_match_java() {
    let rows = golden("dnode");
    assert_eq!(rows.len(), 48);
    for row in rows {
        let t = DirectNodeType::values()[row[0].parse::<usize>().unwrap()];
        let sid: i32 = row[1].parse().unwrap();
        assert_eq!(t.make_id(sid), row[2], "{t}");
    }
    assert_eq!(DirectNodeType::ForeachVardef.name(), "FOREACH_VARDEF");
    assert_eq!(DirectNodeType::CombinedCatch.ordinal(), 7);
    assert_eq!(
        DirectEdgeType::TYPES,
        [DirectEdgeType::Regular, DirectEdgeType::Exception]
    );
    assert_eq!(DirectEdgeType::Exception.to_string(), "EXCEPTION");
    assert_eq!(FinalType::ExplicitFinal.name(), "EXPLICIT_FINAL");
    assert_eq!(State::DeadRead.java_hash(), 3);
}

fn version_graph() -> (VarVersionsGraph, impl Fn(i32, i32) -> VarVersionNode) {
    let vg = VarVersionsGraph::new();
    for var in 1..=2 {
        for version in 1..=5 {
            vg.create_node(VarVersionPair::new(var, version));
        }
    }
    let get = move |var: i32, version: i32| {
        vg.get_node_with_key(VarVersionPair::new(var, version))
            .expect("node")
    };
    for var in 1..=2 {
        for (from, to) in [(1, 2), (1, 3), (2, 4), (3, 4), (4, 5)] {
            SFormsConstructor::make_read_edge(get(var, to), get(var, from));
        }
    }
    (vg, get)
}

#[test]
fn var_versions_graph_matches_java() {
    let (vg, get) = version_graph();

    let order: Vec<String> = VarVersionsGraph::get_reversed_post_order(&[get(1, 1)])
        .iter()
        .map(|n| n.to_string())
        .collect();
    assert_eq!(order.join(" "), golden_line("vg_rpo"));

    assert_eq!(
        vg.are_vars_analogous(1, 2).to_string(),
        golden_line("vg_analog")
    );
    let n31 = vg.create_node(VarVersionPair::new(3, 1));
    vg.create_node(VarVersionPair::new(3, 2));
    SFormsConstructor::make_read_edge(get(1, 2), n31);
    assert_eq!(
        format!(
            "{} {}",
            vg.are_vars_analogous(1, 3),
            vg.are_vars_analogous(1, 2)
        ),
        golden_line("vg_analog2")
    );

    let dom = JHashSet::from_collection(vec![get(1, 2), get(1, 3)]);
    let dom2 = JHashSet::from_collection(vec![get(1, 2), get(2, 3)]);
    assert_eq!(
        format!(
            "{} {} {}",
            vg.is_dominator_set(get(1, 5), &dom),
            vg.is_dominator_set(get(1, 5), &dom2),
            vg.is_dominator_set(get(1, 2), &dom)
        ),
        golden_line("vg_domset")
    );

    let (n14, n15) = (get(1, 4), get(1, 5));
    assert_eq!(
        format!(
            "{} {} {} {} {}",
            n14,
            n14.has_single_predecessor(),
            n15.has_single_predecessor(),
            n15.get_single_predecessor(),
            n15.has_any_successors()
        ),
        golden_line("vg_node")
    );
}

#[test]
fn var_version_node_links_and_reachability() {
    let (vg, get) = version_graph();
    assert_eq!(get(1, 4).get_predecessors(), vec![get(1, 2), get(1, 3)]);
    assert_eq!(get(1, 1).successors(), vec![get(1, 2), get(1, 3)]);
    assert_eq!(get(1, 3).as_pair(), VarVersionPair::new(1, 3));

    let roots = JHashSet::from_collection(vec![get(1, 1)]);
    let reached = VarVersionsGraph::root_reachability(&roots);
    assert_eq!(reached.len(), 5);
    assert!((1..=5).all(|v| reached.contains(&get(1, v))));
    assert!(!reached.contains(&get(2, 1)));

    get(1, 4).remove_predecessor(get(1, 2));
    get(1, 2).remove_successor(get(1, 4));
    assert_eq!(get(1, 4).get_predecessors(), vec![get(1, 3)]);
    assert!(!get(1, 2).has_any_successors());

    let phantom = vg.create_node(VarVersionPair::new(1, 6));
    get(1, 5).set_phantom_node(Some(phantom));
    phantom.set_phantom_parent_node(Some(get(1, 5)));
    phantom.set_state(Some(State::Phantom));
    assert_eq!(get(1, 5).phantom_node(), Some(phantom));
    assert_eq!(phantom.state(), Some(State::Phantom));
    assert_eq!(vg.nodes().len(), 11);
}
