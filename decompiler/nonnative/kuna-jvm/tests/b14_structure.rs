//! Try-with-resources edge search and the try/assert passes on trees that
//! hold nothing to rewrite.

use kuna_jvm::assert_processor::AssertProcessor;
use kuna_jvm::basic_block_statement::BasicBlockStatement;
use kuna_jvm::byte_cursor::ByteCursor;
use kuna_jvm::class_wrapper::ClassWrapper;
use kuna_jvm::classes_processor::{ClassNode, ClassNodeType};
use kuna_jvm::decompiler_context::DecompilerContext;
use kuna_jvm::dummy_exit_statement::DummyExitStatement;
use kuna_jvm::i_fernflower_preferences::IFernflowerPreferences;
use kuna_jvm::root_statement::RootStatement;
use kuna_jvm::rt::{JHashSet, JLinkedHashSet};
use kuna_jvm::sequence_statement::SequenceStatement;
use kuna_jvm::stat_edge::StatEdge;
use kuna_jvm::statement::Statement;
use kuna_jvm::struct_class::StructClass;
use kuna_jvm::synchronized_helper::SynchronizedHelper;
use kuna_jvm::try_helper::TryHelper;
use kuna_jvm::try_with_resources_processor::TryWithResourcesProcessor;
use kuna_jvm::var_processor::VarProcessor;

use crate::common::{self, ContextGuard};

fn fixture_class() -> StructClass {
    let path = format!(
        "{}/tests/data/k2_graph_oracle/classes/K2Shapes.class",
        env!("CARGO_MANIFEST_DIR")
    );
    let bytes = std::fs::read(&path).unwrap_or_else(|e| panic!("{path}: {e}"));
    StructClass::create(&mut ByteCursor::new(bytes), true)
}

fn setup(options: &[(&str, &str)]) -> (ContextGuard, StructClass) {
    let context = common::install_context_with(options);
    let cl = fixture_class();
    let mt = cl.get_method("straight", "(II)I").expect("fixture method");
    DecompilerContext::start_method(VarProcessor::new(mt, &mt.method_descriptor()));
    (context, cl)
}

fn bb() -> Statement {
    BasicBlockStatement::create().0
}

fn edge(type_: i32, source: Statement, destination: Statement) -> StatEdge {
    let e = StatEdge::new(type_, source, destination);
    source.add_successor(e);
    e
}

#[test]
fn edges_leaving_skip_inner_and_exit_edges() {
    let _context = setup(&[]);
    let (a, b, out) = (bb(), bb(), bb());
    let exit = DummyExitStatement::new();
    let seq = SequenceStatement::new(vec![a, b]).0;
    seq.set_all_parent();
    edge(StatEdge::TYPE_REGULAR, a, b);
    let to_out = edge(StatEdge::TYPE_BREAK, b, out);
    let to_exit = edge(StatEdge::TYPE_BREAK, a, exit.0);

    let mut edges: JLinkedHashSet<StatEdge> = JLinkedHashSet::new();
    TryWithResourcesProcessor::find_edges_leaving(seq, seq, &mut edges);
    assert_eq!(edges.to_vec(), vec![to_out]);

    let mut with_exit: JLinkedHashSet<StatEdge> = JLinkedHashSet::new();
    TryWithResourcesProcessor::find_edges_leaving_exit(seq, seq, &mut with_exit, true);
    assert_eq!(with_exit.to_vec(), vec![to_exit, to_out]);
}

#[test]
fn edges_leaving_drop_implicit_edges_already_found_by_a_container() {
    let _context = setup(&[]);
    let (a, b, out, check) = (bb(), bb(), bb(), bb());
    let seq = SequenceStatement::new(vec![a, b]).0;
    seq.set_all_parent();
    let from_seq = edge(StatEdge::TYPE_REGULAR, seq, out);
    let implicit = edge(StatEdge::TYPE_BREAK, b, out);
    implicit.with_mut(|d| d.explicit = false);
    let explicit = edge(StatEdge::TYPE_BREAK, a, out);

    let mut edges: JHashSet<StatEdge> = JHashSet::new();
    TryWithResourcesProcessor::find_edges_leaving(seq, check, &mut edges);
    assert_eq!(edges.len(), 2);
    assert!(edges.contains(&from_seq) && edges.contains(&explicit));
    assert!(!edges.contains(&implicit));
}

#[test]
fn try_and_sync_passes_leave_plain_trees_alone() {
    let (_context, cl) = setup(&[]);
    let mt = cl.get_method("straight", "(II)I").unwrap();
    let (a, b) = (bb(), bb());
    let seq = SequenceStatement::new(vec![a, b]).0;
    seq.set_all_parent();
    let root = RootStatement::new(seq, DummyExitStatement::new(), mt);
    seq.set_parent(Some(root.0));

    assert!(!TryHelper::enhance_try_stats(root, cl));
    assert!(!SynchronizedHelper::clean_synchronized_var(root.0));
    let var_processor = DecompilerContext::get_var_processor();
    assert!(!SynchronizedHelper::insert_sink(
        root,
        var_processor,
        root.0
    ));
    assert_eq!(root.get_stats().borrow().to_vec(), vec![seq]);
}

#[test]
fn assertions_need_the_option_and_an_assert_field() {
    let (_context, cl) = setup(&[(IFernflowerPreferences::DECOMPILE_ASSERTIONS, "0")]);
    let mt = cl.get_method("straight", "(II)I").unwrap();
    let root = RootStatement::new(bb(), DummyExitStatement::new(), mt);
    assert!(!AssertProcessor::build_assertions(root));
    drop(_context);

    let (_context, cl) = setup(&[]);
    let mt = cl.get_method("straight", "(II)I").unwrap();
    let root = RootStatement::new(bb(), DummyExitStatement::new(), mt);
    let wrapper = ClassWrapper::new(ClassNode::new(ClassNodeType::Root, cl));
    DecompilerContext::set_current_class_wrapper(Some(wrapper));
    assert!(AssertProcessor::get_holding_class() == wrapper);
    assert!(!AssertProcessor::build_assertions(root));
}
