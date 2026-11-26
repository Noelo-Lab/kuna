//! Engine state that must not leak between methods, classes, threads or
//! batches: what `catch (RuntimeException)` catches, the thread-local marks of
//! two fixes, the counter the `class$` patterns draw from, and the members a
//! class's writing marks; and the one test of a kotlinc class that the
//! options reading javac's code shapes share.

use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;

use kuna_jvm::byte_cursor::ByteCursor;
use kuna_jvm::class_reference14_processor::ClassReference14Processor;
use kuna_jvm::classes_processor::{ClassNode, ClassNodeType};
use kuna_jvm::counter_container::CounterContainer;
use kuna_jvm::decompiler_context::DecompilerContext;
use kuna_jvm::i_fernflower_logger::IFernflowerLogger;
use kuna_jvm::testing::kt_static_initializer as kuna_fix_kt_static_initializer;
use kuna_jvm::kuna_driver::{self, DriverConfig, Jobs};
use kuna_jvm::kuna_input::{self, LoadOptions};
use kuna_jvm::testing::disjoint_generic_cast as kuna_fix_disjoint_generic_cast;
use kuna_jvm::rt::{throw, try_catch};
use kuna_jvm::struct_class::StructClass;
use kuna_jvm::text_buffer::TextBuffer;

use crate::common;
use crate::kuna_support::{decompile, options, scratch, RecordingLogger};

fn class(rel: &str) -> StructClass {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(rel);
    StructClass::create(&mut ByteCursor::new(std::fs::read(path).unwrap()), true)
}

#[test]
fn errors_and_method_stops_are_not_runtime_exceptions() {
    let caught = |class: &'static str| try_catch(|| throw(class, "")).unwrap_err();
    for class in ["IllegalArgumentException", "NullPointerException", "CanceledException"] {
        assert!(caught(class).is_runtime_exception(), "{class}");
    }
    for class in [
        "StackOverflowError",
        "UnportedError",
        "MethodDeadlineExceeded",
        "MethodAbortedException",
    ] {
        assert!(!caught(class).is_runtime_exception(), "{class}");
    }
    let panic = try_catch(|| -> () { panic!("index out of bounds") }).unwrap_err();
    assert!(panic.is_runtime_exception());
}

#[test]
fn a_declared_target_mark_does_not_outlive_a_throw() {
    let thrown = try_catch(|| {
        let _declared = kuna_fix_disjoint_generic_cast::mark_declared_target();
        throw("NullPointerException", "value")
    });
    assert!(thrown.is_err());
    assert!(!kuna_fix_disjoint_generic_cast::take_declared_target());
}

#[test]
fn an_init_block_not_taken_is_forgotten_with_its_class() {
    let _guard = common::install_default_context();
    let cl = class("tests/data/k2_graph_oracle/classes/K2Shapes.class");
    let companion = ClassNode::new(ClassNodeType::Member, cl);
    let thrown = try_catch(|| {
        let _moved = kuna_fix_kt_static_initializer::scope();
        kuna_fix_kt_static_initializer::move_to(companion, TextBuffer::new());
        throw("IllegalStateException", "the outer class's write")
    });
    assert!(thrown.is_err());
    assert!(kuna_fix_kt_static_initializer::take(cl).is_none());
    {
        let _moved = kuna_fix_kt_static_initializer::scope();
        kuna_fix_kt_static_initializer::move_to(companion, TextBuffer::new());
    }
    assert!(kuna_fix_kt_static_initializer::take(cl).is_none());
}

#[test]
fn the_class_reference_patterns_take_no_expression_ids() {
    let _guard = common::install_default_context();
    let node = ClassNode::new(
        ClassNodeType::Root,
        class("tests/data/k2_graph_oracle/classes/K2Shapes.class"),
    );
    let counter = DecompilerContext::get_counter_container();
    let before = counter.get_counter(CounterContainer::EXPRESSION_COUNTER);
    let _ = try_catch(|| ClassReference14Processor::process_class_references(node));
    assert_eq!(
        counter.get_counter(CounterContainer::EXPRESSION_COUNTER),
        before
    );
}

/// `TestKtCompanionOverride`'s companion overrides its `size` and `label`;
/// writing the class marks their getters as companion items, which a later
/// batch on the same engine read while processing `User` (`r.getSize()` in
/// place of the single run's `r.size`, Vineflower's own output).
#[test]
fn a_batch_leaves_no_marks_for_the_next_one() {
    let dir = scratch("companion-marks");
    let fixes = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../tests/jvm/kuna-fixes/classes/pkg");
    let data = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/data/kt_companion_marks/pkg");
    std::fs::create_dir_all(dir.join("pkg")).unwrap();
    for name in [
        "TestKtCompanionOverride.class",
        "TestKtCompanionOverride$Default.class",
        "TestKtCompanionOverride$Shape.class",
        "TestKtCompanionOverride$Shape$Companion.class",
    ] {
        std::fs::copy(fixes.join(name), dir.join("pkg").join(name)).unwrap();
    }
    std::fs::copy(data.join("User.class"), dir.join("pkg/User.class")).unwrap();
    let input = kuna_input::load(&dir, &LoadOptions::default()).unwrap();
    let opts = options(&[]);
    let single: Vec<(String, String)> = decompile(&input, &opts)
        .into_iter()
        .map(|u| (u.qualified_name, u.content))
        .collect();
    assert!(single.iter().any(|(n, c)| n == "pkg/User" && c.contains("r.size")));
    let mut config = DriverConfig::new(
        opts,
        Arc::new(|| Rc::new(RecordingLogger::new()) as Rc<dyn IFernflowerLogger>),
    );
    config.jobs = Jobs::Exactly(1);
    let mut results = Vec::new();
    let report = kuna_driver::run(&input, &config, &mut |r| results.push(r)).unwrap();
    assert!(report.runs > 1 && report.instances == 1, "{report:?}");
    let driven: Vec<(String, String)> = results
        .into_iter()
        .map(|r| {
            let unit = r.unit.unwrap();
            (unit.qualified_name, unit.content)
        })
        .collect();
    assert_eq!(driven, single);
}

/// `KtSource.java` compiled by javac 17, and the same class naming a `.kt`
/// source file with no `kotlin.Metadata`, which every option that reads
/// javac's code shapes takes for kotlinc's (`jvm_javac_layout` keeps
/// Vineflower's `return c ? 1 : 2` there).
#[test]
fn a_class_named_after_a_kotlin_source_keeps_vineflower_layout() {
    let data = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/data/kotlinc_source/KtSource.class");
    let javac = std::fs::read(&data).unwrap();
    let name = |s: &str| {
        let mut entry = vec![1u8];
        entry.extend((s.len() as u16).to_be_bytes());
        entry.extend(s.as_bytes());
        entry
    };
    let (from, to) = (name("KtSource.java"), name("KtSource.kt"));
    let at = javac.windows(from.len()).position(|w| w == from.as_slice()).unwrap();
    let kotlinc = [&javac[..at], to.as_slice(), &javac[at + from.len()..]].concat();
    let text = |bytes: &[u8], dir: &str| {
        let dir = scratch(dir);
        std::fs::write(dir.join("KtSource.class"), bytes).unwrap();
        let input = kuna_input::load(&dir.join("KtSource.class"), &LoadOptions::default()).unwrap();
        decompile(&input, &options(&[])).remove(0).content
    };
    assert!(!text(&javac, "kotlinc-source-java").contains("return var0 ? 1 : 2;"));
    assert!(text(&kotlinc, "kotlinc-source-kt").contains("return var0 ? 1 : 2;"));
}
