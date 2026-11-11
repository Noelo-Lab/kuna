//! `NestedClassProcessor` and `NestedMemberAccess` over the `class_model`
//! fixture with wrappers whose methods carry no decompiled code, and
//! `NestedMemberAccess` over hand-built direct graphs for one synthetic static
//! accessor and its caller, and the local class definition placed into a
//! hand-built statement tree, against Vineflower 1.12.0 doing the same
//! (`tests/data/b20_nested/gen.sh`).

use std::path::PathBuf;

use kuna_jvm::assignment_exprent::AssignmentExprent;
use kuna_jvm::class_wrapper::ClassWrapper;
use kuna_jvm::classes_processor::{ClassNode, ClassNodeType};
use kuna_jvm::const_exprent::ConstExprent;
use kuna_jvm::counter_container::CounterContainer;
use kuna_jvm::decompiler_context::DecompilerContext;
use kuna_jvm::direct_edge::DirectEdge;
use kuna_jvm::direct_graph::DirectGraph;
use kuna_jvm::direct_node::DirectNode;
use kuna_jvm::direct_node_type::DirectNodeType;
use kuna_jvm::dummy_exit_statement::DummyExitStatement;
use kuna_jvm::exit_exprent::{ExitExprent, Type as ExitType};
use kuna_jvm::exprent::{Exprent, ExprentType};
use kuna_jvm::field_descriptor::FieldDescriptor;
use kuna_jvm::field_exprent::FieldExprent;
use kuna_jvm::function_exprent::{FunctionExprent, FunctionType};
use kuna_jvm::i_fernflower_logger::Severity;
use kuna_jvm::i_fernflower_preferences::IFernflowerPreferences;
use kuna_jvm::interpreter_util::InterpreterUtil;
use kuna_jvm::invocation_exprent::InvocationExprent;
use kuna_jvm::method_descriptor::MethodDescriptor;
use kuna_jvm::method_wrapper::MethodWrapper;
use kuna_jvm::nested_class_processor::NestedClassProcessor;
use kuna_jvm::nested_member_access::NestedMemberAccess;
use kuna_jvm::new_exprent::NewExprent;
use kuna_jvm::root_statement::RootStatement;
use kuna_jvm::rt::{JHashMap, JList};
use kuna_jvm::sequence_statement::SequenceStatement;
use kuna_jvm::statement::Statement;
use kuna_jvm::struct_class::StructClass;
use kuna_jvm::struct_context::StructContext;
use kuna_jvm::var_exprent::VarExprent;
use kuna_jvm::var_processor::VarProcessor;
use kuna_jvm::var_type::VarType;

use crate::common::{self, ContextGuard};

const ACCESSOR: &str = "lambda$new$0 ()V";
const CALLER: &str = "<init> (Lp/Outer;)V";
const RUN: &str = "run ()V";

const NAMES: [&str; 5] = [
    "p/Outer",
    "p/Outer$1Local",
    "p/Outer$A",
    "p/Outer$A$1",
    "p/Outer$B",
];

fn data_dir(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/data")
        .join(name)
}

fn fixture() -> (ContextGuard, StructContext) {
    let guard = common::install_context_with(&[
        (IFernflowerPreferences::INCLUDE_JAVA_RUNTIME, "0"),
        (IFernflowerPreferences::DUMP_EXCEPTION_ON_ERROR, "0"),
    ]);
    let sc = guard.struct_context();
    sc.add_space_file(&data_dir("class_model").join("classes"), true);
    sc.add_space_file(&data_dir("class_model").join("lib"), false);
    (guard, sc)
}

fn class(sc: StructContext, name: &str) -> StructClass {
    sc.get_class(name)
        .unwrap_or_else(|| panic!("fixture class {name}"))
}

/// The node tree `ClassesProcessor.loadClasses` builds for the fixture (as in `class_model.rs`).
fn load_nodes(guard: &ContextGuard, sc: StructContext) -> ClassNode {
    let roots = guard.class_processor().get_map_root_classes();
    let mut nodes = Vec::new();
    for name in NAMES {
        let cl = class(sc, name);
        let node = ClassNode::new(ClassNodeType::Root, cl);
        node.with_mut(|d| d.access = cl.get_access_flags());
        roots.borrow_mut().put(name.to_string(), node);
        nodes.push(node);
    }
    let [outer, local, a, a1, b] = nodes[..] else {
        unreachable!()
    };

    let nest = |parent: ClassNode,
                child: ClassNode,
                type_: ClassNodeType,
                simple_name: Option<&str>,
                access: i32,
                enclosing: &[&str]| {
        child.with_mut(|d| {
            d.simple_name = simple_name.map(str::to_string);
            d.type_ = type_;
            if type_ == ClassNodeType::Anonymous {
                d.enclosing_method = None;
            }
            d.access = access;
            d.parent = Some(parent);
        });
        if type_ == ClassNodeType::Anonymous {
            let cl = child.with(|d| d.class_struct);
            let type_ = VarType::new_signature_cl(&cl.get_interface(0), true);
            child.with_mut(|d| d.anonymous_class_type = Some(type_));
        }
        parent.with(|d| d.nested.clone()).add(child);
        let set = child.with(|d| d.enclosing_classes.clone());
        for name in enclosing {
            set.borrow_mut().add(name.to_string());
        }
    };
    nest(
        outer,
        local,
        ClassNodeType::Local,
        Some("Local"),
        0,
        &["p/Outer"],
    );
    nest(
        outer,
        a,
        ClassNodeType::Member,
        Some("A"),
        0x0001,
        &["p/Outer"],
    );
    nest(
        outer,
        b,
        ClassNodeType::Member,
        Some("B"),
        0x0009,
        &["p/Outer"],
    );
    nest(
        a,
        a1,
        ClassNodeType::Anonymous,
        None,
        0,
        &["p/Outer", "p/Outer$A"],
    );
    for node in [outer, a] {
        let nested = node.with(|d| d.nested.clone());
        let mut sorted = nested.to_vec();
        sorted.sort_by(|x, y| x.compare_to(*y));
        nested.set_all(sorted);
    }
    outer
}

fn esc(s: Option<&str>) -> String {
    s.unwrap_or("\\N").to_string()
}

fn name(n: Option<ClassNode>) -> String {
    n.map_or("\\N".to_string(), |n| {
        n.with(|d| d.class_struct).qualified_name()
    })
}

fn node_line(n: ClassNode) -> String {
    let nested: Vec<String> = n
        .with(|d| d.nested.to_vec())
        .into_iter()
        .map(|c| format!("{}:{}", c.with(|d| d.type_).name(), name(Some(c))))
        .collect();
    let mut enclosing = n.with(|d| d.enclosing_classes.borrow().to_vec());
    enclosing.sort();
    let (type_, simple_name, parent, enclosing_method, fields, super_invocation) = n.with(|d| {
        (
            d.type_,
            d.simple_name.clone(),
            d.parent,
            d.enclosing_method.clone(),
            d.map_fields_to_vars.len(),
            d.super_invocation.is_some(),
        )
    });
    [
        "node".to_string(),
        type_.name().to_string(),
        name(Some(n)),
        esc(simple_name.as_deref()),
        name(parent),
        nested.join(","),
        esc(enclosing_method.as_deref()),
        enclosing.join(","),
        fields.to_string(),
        super_invocation.to_string(),
    ]
    .join("\t")
}

fn wrapper_lines(n: ClassNode, out: &mut Vec<String>) {
    let w = n.get_wrapper().unwrap();
    let mut hidden = w.get_hidden_members().borrow().to_vec();
    hidden.sort();
    out.push(format!("hidden\t{}\t{}", name(Some(n)), hidden.join(",")));
    for mw in w.get_methods().borrow().to_vec() {
        let (mt, comments, error, synth, varproc) = mw.with(|d| {
            (
                d.method_struct,
                d.comment_lines.as_ref().map(|c| c.to_vec()),
                d.add_error_comment,
                d.synth_parameters.clone(),
                d.varproc,
            )
        });
        let synth = synth.map_or("\\N".to_string(), |s| {
            s.iter()
                .map(|p| p.map_or("null".to_string(), |p| p.to_string()))
                .collect::<Vec<_>>()
                .join(",")
        });
        out.push(
            [
                "method".to_string(),
                name(Some(n)),
                format!("{} {}", mt.get_name(), mt.get_descriptor()),
                comments.map_or("\\N".to_string(), |c| c.join("|")),
                error.to_string(),
                synth,
                varproc.nested_processed().to_string(),
            ]
            .join("\t"),
        );
    }
}

/// Wraps the fixture classes with code-less methods; for `kind` "access" the
/// accessor and the caller, for "localdef" `p/Outer.run`, get a root statement
/// and a two-node direct graph. Returns the graphs' first nodes by `class.key`
/// and the two statements of `run`'s sequence.
fn wrap(guard: &ContextGuard, kind: &str) -> (JHashMap<String, DirectNode>, Vec<Statement>) {
    let roots = guard.class_processor().get_map_root_classes();
    let mut firsts = JHashMap::new();
    let mut stats = Vec::new();
    for name in NAMES {
        let node = roots.borrow().get_cloned(&name.to_string()).unwrap();
        let cl = node.with(|d| d.class_struct);
        let wrapper = ClassWrapper::new(node);
        for mt in cl.get_methods().to_vec() {
            let md = MethodDescriptor::parse_descriptor(&mt.get_descriptor());
            let key = InterpreterUtil::make_unique_key(&mt.get_name(), &mt.get_descriptor());
            let special = kind == "access"
                && ((name == "p/Outer" && key == ACCESSOR)
                    || (name == "p/Outer$A" && key == CALLER))
                || kind == "localdef" && name == "p/Outer" && key == RUN;
            let mw = if special {
                let head = if kind == "localdef" {
                    let s1 = DummyExitStatement::new().0;
                    let s2 = DummyExitStatement::new().0;
                    stats.push(s1);
                    stats.push(s2);
                    SequenceStatement::new(vec![s1, s2]).0
                } else {
                    DummyExitStatement::new().0
                };
                let exit = DummyExitStatement::new();
                let root = RootStatement::new(head, exit, mt);
                let mw = MethodWrapper::new(
                    Some(root),
                    VarProcessor::new(mt, &md),
                    mt,
                    cl,
                    CounterContainer::new(),
                );
                let graph = DirectGraph::new();
                let first = DirectNode::for_stat(DirectNodeType::Direct, head, None);
                let last = DirectNode::for_stat(DirectNodeType::Direct, exit.0, None);
                let (first_id, last_id) = (first.id(), last.id());
                graph.with_mut(|d| {
                    d.nodes.add_with_key(first, first_id);
                    d.nodes.add_with_key(last, last_id);
                });
                graph.set_first(Some(first));
                first.add_successor(DirectEdge::of(first, last));
                mw.with_mut(|d| d.graph = Some(graph));
                firsts.put(format!("{name}.{key}"), first);
                mw
            } else {
                MethodWrapper::new(
                    None,
                    VarProcessor::new(mt, &md),
                    mt,
                    cl,
                    CounterContainer::new(),
                )
            };
            wrapper.get_methods().borrow_mut().add_with_key(mw, key);
        }
        node.with_mut(|d| d.wrapper = Some(wrapper));
    }
    (firsts, stats)
}

fn desc(e: Option<Exprent>) -> String {
    let Some(e) = e else {
        return "-".to_string();
    };
    let id = format!("#{}", e.get_id());
    match e.get_type() {
        ExprentType::Field => {
            let f = e.as_field();
            format!(
                "Field{id}({}.{},{},{})",
                f.get_classname(),
                f.get_name(),
                f.is_static(),
                desc(f.get_instance())
            )
        }
        ExprentType::Var => {
            let v = e.as_var();
            format!(
                "Var{id}({}{}{})",
                v.get_index(),
                if v.is_definition() { ",def" } else { "" },
                if v.is_class_def() { ",classdef" } else { "" }
            )
        }
        ExprentType::New => format!("New{id}({})", e.as_new().get_new_type().value_or_null()),
        ExprentType::Const => format!("Const{id}"),
        ExprentType::Invocation => {
            let i = e.as_invocation();
            let ps: Vec<String> = i
                .get_lst_parameters()
                .to_vec()
                .into_iter()
                .map(|p| desc(Some(p)))
                .collect();
            format!(
                "Inv{id}({}.{}{},{},{},[{}])",
                i.get_classname(),
                i.get_name(),
                i.get_string_descriptor(),
                i.is_static(),
                desc(i.get_instance()),
                ps.join(",")
            )
        }
        ExprentType::Assignment => {
            let a = e.as_assignment();
            format!(
                "Assign{id}({},{})",
                desc(Some(a.get_left())),
                desc(Some(a.get_right()))
            )
        }
        ExprentType::Function => {
            let f = e.as_function();
            let ps: Vec<String> = f
                .get_lst_operands()
                .to_vec()
                .into_iter()
                .map(|p| desc(Some(p)))
                .collect();
            format!("Func{id}({},[{}])", f.get_func_type().name(), ps.join(","))
        }
        ExprentType::Exit => {
            let x = e.as_exit();
            format!(
                "Exit{id}({},{})",
                x.get_exit_type().name(),
                desc(x.get_value())
            )
        }
        t => format!("{}{id}", t.name()),
    }
}

fn call() -> InvocationExprent {
    let inv = InvocationExprent::new();
    inv.set_name("lambda$new$0".to_string());
    inv.set_classname("p/Outer".to_string());
    inv.set_string_descriptor("()V".to_string());
    inv.set_descriptor(MethodDescriptor::parse_descriptor("()V"));
    inv.set_static(true);
    inv.set_lst_parameters(Vec::new());
    inv
}

fn static_field() -> Exprent {
    FieldExprent::new(
        "S".to_string(),
        "p/Outer".to_string(),
        true,
        None,
        FieldDescriptor::parse_descriptor("I"),
        None,
    )
    .0
}

fn warnings(guard: &ContextGuard, from: usize, out: &mut Vec<String>) {
    let records = guard.logger().records();
    for record in records.borrow()[from..].iter() {
        if record.severity >= Severity::Warn {
            out.push(format!("{}\t{}", record.severity.name(), record.message));
        }
    }
}

fn golden_section(header: &str) -> Vec<String> {
    let golden = std::fs::read_to_string(data_dir("b20_nested").join("golden.txt")).unwrap();
    let mut lines = golden.lines().skip_while(|l| *l != header);
    let mut section = vec![lines.next().expect("golden section").to_string()];
    section.extend(
        lines
            .take_while(|l| !l.starts_with("== access") && !l.starts_with("== localdef"))
            .map(str::to_string),
    );
    section
}

#[test]
fn nested_processing_without_code_matches_vineflower() {
    let (guard, sc) = fixture();
    let outer = load_nodes(&guard, sc);
    let roots = guard.class_processor().get_map_root_classes();
    let node = |name: &str| roots.borrow().get_cloned(&name.to_string()).unwrap();

    let cl = class(sc, "p/Outer");
    let lambda = ClassNode::new_lambda(
        "p/Outer",
        "lambda$new$0",
        "()V",
        6,
        "java/lang/Runnable",
        "run",
        "()V",
        cl,
    );
    lambda.with_mut(|d| {
        d.simple_name = Some("p/Outer##Lambda_0_1".to_string());
        d.enclosing_method = Some("<init> ()V".to_string());
        d.parent = Some(outer);
    });
    outer.with(|d| d.nested.clone()).add(lambda);

    node("p/Outer$B").with_mut(|d| d.simple_name = None);

    wrap(&guard, "plain");

    let logged_before = guard.logger().records().borrow().len();

    NestedClassProcessor::new().process_class(outer, outer);
    NestedMemberAccess::new().propagate_member_access(outer);

    let mut out = vec!["== log".to_string()];
    warnings(&guard, logged_before, &mut out);
    out.push("== nodes".to_string());
    for name in NAMES {
        out.push(node_line(node(name)));
    }
    out.push(node_line(lambda));
    out.push("== wrappers".to_string());
    for name in NAMES {
        wrapper_lines(node(name), &mut out);
    }

    assert_eq!(out, golden_section("== log"));
}

fn access(variant: &str) -> Vec<String> {
    let (guard, sc) = fixture();
    let outer = load_nodes(&guard, sc);
    let counters = DecompilerContext::get_counter_container();
    counters.set_counter(CounterContainer::STATEMENT_COUNTER, 50);
    counters.set_counter(CounterContainer::EXPRESSION_COUNTER, 100);
    let (firsts, _) = wrap(&guard, "access");

    let value = match variant {
        "field" => static_field(),
        "method" => {
            let inv = InvocationExprent::new();
            inv.set_name("m".to_string());
            inv.set_classname("q/Other".to_string());
            inv.set_string_descriptor("()I".to_string());
            inv.set_descriptor(MethodDescriptor::parse_descriptor("()I"));
            inv.set_static(true);
            inv.set_lst_parameters(Vec::new());
            inv.0
        }
        _ => {
            let field = static_field();
            FunctionExprent::new(FunctionType::Ipp, vec![field], None).0
        }
    };
    let accessor = firsts.get_cloned(&format!("p/Outer.{ACCESSOR}")).unwrap();
    accessor.exprents().add(
        ExitExprent::new(
            ExitType::Return,
            Some(value),
            Some(VarType::VARTYPE_INT()),
            None,
            None,
        )
        .0,
    );

    let caller_method = guard
        .class_processor()
        .get_map_root_classes()
        .borrow()
        .get_cloned(&"p/Outer$A".to_string())
        .unwrap()
        .get_wrapper()
        .unwrap()
        .get_methods()
        .borrow()
        .get_with_key(&CALLER.to_string())
        .unwrap();
    let caller = firsts.get_cloned(&format!("p/Outer$A.{CALLER}")).unwrap();
    let first = call();
    let var = VarExprent::new(
        5,
        Some(VarType::VARTYPE_INT()),
        Some(caller_method.with(|d| d.varproc)),
    );
    let second = call();
    caller.exprents().add(first.0);
    caller
        .exprents()
        .add(AssignmentExprent::new(var.0, second.0, None).0);

    let logged_before = guard.logger().records().borrow().len();
    NestedMemberAccess::new().propagate_member_access(outer);

    let mut out = vec![format!("== access {variant}")];
    warnings(&guard, logged_before, &mut out);
    let mut hidden = outer
        .get_wrapper()
        .unwrap()
        .get_hidden_members()
        .borrow()
        .to_vec();
    hidden.sort();
    out.push(format!("hidden\t{}", hidden.join(",")));
    for e in accessor.exprents().to_vec() {
        out.push(format!("accessor\t{}", desc(Some(e))));
    }
    for e in caller.exprents().to_vec() {
        out.push(format!("caller\t{}", desc(Some(e))));
    }
    out
}

#[test]
fn accessor_calls_are_inlined_like_vineflower() {
    for variant in ["field", "method", "function"] {
        let header = format!("== access {variant}");
        assert_eq!(access(variant), golden_section(&header), "{variant}");
    }
}

#[test]
fn member_access_skips_a_root_without_nested_classes() {
    let (guard, sc) = fixture();
    let outer = load_nodes(&guard, sc);
    let b = guard
        .class_processor()
        .get_map_root_classes()
        .borrow()
        .get_cloned(&"p/Outer$B".to_string())
        .unwrap();
    assert!(b.with(|d| d.nested.is_empty()));
    assert!(outer.get_wrapper().is_none());
    NestedMemberAccess::new().propagate_member_access(b);
    NestedClassProcessor::new().process_class(b, b);
}

fn localdef(variant: &str) -> Vec<String> {
    let (guard, sc) = fixture();
    let outer = load_nodes(&guard, sc);
    let counters = DecompilerContext::get_counter_container();
    counters.set_counter(CounterContainer::STATEMENT_COUNTER, 50);
    counters.set_counter(CounterContainer::EXPRESSION_COUNTER, 100);
    let (_, stats) = wrap(&guard, "localdef");

    let run = outer
        .get_wrapper()
        .unwrap()
        .get_methods()
        .borrow()
        .get_with_key(&RUN.to_string())
        .unwrap();
    let varproc = run.with(|d| d.varproc);
    let local = VarType::new_signature_cl("p/Outer$1Local", true);
    let mut first: Vec<Exprent> = Vec::new();
    let mut second: Vec<Exprent> = Vec::new();
    let var1 = VarExprent::new(1, Some(VarType::VARTYPE_INT()), Some(varproc));
    let zero = ConstExprent::new_int(0, false, None);
    first.push(AssignmentExprent::new(var1.0, zero.0, None).0);
    if variant == "single" {
        second.push(NewExprent::new(local.clone(), Vec::new(), None).0);
    } else {
        let var2 = VarExprent::new(2, Some(local.clone()), Some(varproc));
        let created = NewExprent::new(local.clone(), Vec::new(), None);
        first.push(AssignmentExprent::new(var2.0, created.0, None).0);
        second.push(
            FieldExprent::new(
                "x".to_string(),
                "p/Outer$1Local".to_string(),
                true,
                None,
                FieldDescriptor::parse_descriptor("I"),
                None,
            )
            .0,
        );
    }
    stats[0].set_exprents(JList::from(first));
    stats[1].set_exprents(JList::from(second));

    let logged_before = guard.logger().records().borrow().len();
    NestedClassProcessor::new().process_class(outer, outer);

    let mut out = vec![format!("== localdef {variant}")];
    warnings(&guard, logged_before, &mut out);
    for (name, stat) in [("s1", stats[0]), ("s2", stats[1])] {
        for e in stat.get_exprents().unwrap().to_vec() {
            out.push(format!("{name}\t{}", desc(Some(e))));
        }
        out.push(format!(
            "{name}\tvardefs\t{}",
            stat.get_var_definitions().len()
        ));
    }
    let comments = run.with(|d| d.comment_lines.as_ref().map(|c| c.to_vec()));
    out.push(format!(
        "comments\t{}",
        comments.map_or("\\N".to_string(), |c| c.join("|"))
    ));
    out
}

#[test]
fn local_class_definitions_are_placed_like_vineflower() {
    for variant in ["single", "both"] {
        let header = format!("== localdef {variant}");
        assert_eq!(localdef(variant), golden_section(&header), "{variant}");
    }
}
