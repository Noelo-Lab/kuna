//! The trace file `DecompileRecord::add` writes when tracing is on: step numbering across records,
//! repeat counters, unchanged-step back-references and method headers must be byte-identical to
//! what the Java tracer writes for the same call sequence (`data/trace/record.vft`, from
//! `data/trace/GenTraceVectors.java`).

use std::path::PathBuf;

use kuna_jvm::basic_block::BasicBlock;
use kuna_jvm::basic_block_statement::BasicBlockStatement;
use kuna_jvm::byte_cursor::ByteCursor;
use kuna_jvm::const_exprent::ConstExprent;
use kuna_jvm::counter_container::CounterContainer;
use kuna_jvm::decompile_record::DecompileRecord;
use kuna_jvm::decompiler_context::DecompilerContext;
use kuna_jvm::dummy_exit_statement::DummyExitStatement;
use kuna_jvm::kuna_trace;
use kuna_jvm::root_statement::RootStatement;
use kuna_jvm::rt::JList;
use kuna_jvm::struct_class::StructClass;
use kuna_jvm::struct_method::StructMethod;
use kuna_jvm::var_processor::VarProcessor;

use crate::common;

const RECORD: &str = include_str!("data/trace/record.vft");
const K2SHAPES: &[u8] = include_bytes!("data/k2_graph_oracle/classes/K2Shapes.class");

fn shapes_method(name: &str, descriptor: &str) -> StructMethod {
    StructClass::create(&mut ByteCursor::new(K2SHAPES.to_vec()), true)
        .get_method(name, descriptor)
        .expect("K2Shapes method")
}

fn trace_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!("{name}-{}.vft", std::process::id()))
}

fn start(mt: StructMethod) {
    DecompilerContext::start_method(VarProcessor::new(mt, &mt.method_descriptor()));
}

fn tiny(mt: StructMethod) -> RootStatement {
    let id = DecompilerContext::get_counter_container()
        .get_counter_and_increment(CounterContainer::STATEMENT_COUNTER);
    let b = BasicBlockStatement::new(BasicBlock::new(id)).0;
    RootStatement::new(b, DummyExitStatement::new(), mt)
}

fn body(text: &str) -> &str {
    text.split_once('\n').map_or("", |(_, rest)| rest)
}

#[test]
fn trace_file_matches_the_java_tracer() {
    let _context = common::install_default_context();
    let mt = shapes_method("straight", "(II)I");
    let mt2 = shapes_method("ifElse", "(I)I");
    let path = trace_path("record");
    kuna_trace::start(&path, None).expect("trace file");
    assert!(kuna_trace::enabled());

    start(mt);
    let root = tiny(mt);
    let rec = DecompileRecord::new(mt);
    rec.add("Initial", Some(root));
    rec.add("CondenseSequences", Some(root));
    let first = root.get_first().expect("first");
    first.set_exprents(JList::from(vec![ConstExprent::new_int(1, false, None).0]));
    rec.increment_main_loop();
    rec.add("Start", Some(root));
    rec.add("Start", Some(root));
    rec.increment_merge_loop();
    rec.add("MergeLoopStart", Some(root));
    rec.reset_merge_loop();
    rec.reset_main_loop();
    rec.add("MainLoopEnd", Some(root));
    rec.add("NoTree", None);
    start(mt);
    let root2 = tiny(mt2);
    DecompileRecord::new(mt2).add("Initial", Some(root2));
    DecompileRecord::new(mt).add("FinalSimplifySwitches", Some(root));
    DecompileRecord::new(mt2).add("FinalPrettifyIfs", Some(root));

    let summary = kuna_trace::finish().expect("tracing was on");
    assert!(!kuna_trace::enabled());
    assert_eq!((summary.methods, summary.steps), (2, 9));
    let written = std::fs::read_to_string(&path).expect("trace written");
    assert_eq!(summary.bytes, written.len() as u64);
    assert!(written.starts_with("#vftrace 1 engine=kuna"));
    assert_eq!(body(&written), body(RECORD));
}

#[test]
fn trace_filter_skips_other_methods() {
    let _context = common::install_default_context();
    let mt = shapes_method("straight", "(II)I");
    let path = trace_path("filter");
    kuna_trace::start(&path, Some("#ifElse(")).expect("trace file");
    start(mt);
    DecompileRecord::new(mt).add("Initial", Some(tiny(mt)));
    let summary = kuna_trace::finish().expect("tracing was on");
    assert_eq!((summary.methods, summary.steps), (0, 0));
    let written = std::fs::read_to_string(&path).expect("trace written");
    assert_eq!(written.lines().count(), 1);
}

/// Off by default; `KUNA_JVM_TRACE=<file> cargo test ... trace_env_` exercises the env switch.
#[test]
fn trace_env_switch_is_read_once_per_thread() {
    let _context = common::install_default_context();
    let mt = shapes_method("straight", "(II)I");
    let from_env = std::env::var_os("KUNA_JVM_TRACE").filter(|p| !p.is_empty());
    start(mt);
    DecompileRecord::new(mt).add("Initial", Some(tiny(mt)));
    assert_eq!(kuna_trace::enabled(), from_env.is_some());
    if from_env.is_some() {
        let summary = kuna_trace::finish().expect("env tracing");
        assert_eq!(summary.steps, 1);
    } else {
        assert!(kuna_trace::finish().is_none());
    }
}
