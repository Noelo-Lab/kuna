//! KT8: classes written as Kotlin under kuna's per-method deadline and method
//! guard, the polls of the Kotlin pass schedule, the stop signals the Kotlin
//! writer passes on, and the spans of a Kotlin class (vendored corpus class
//! `pkg/TestTrySynchronized`: two functions, a property with a getter, a companion).

use std::cell::Cell;
use std::rc::Rc;
use std::time::{Duration, Instant};

use kuna_jvm::byte_cursor::ByteCursor;
use kuna_jvm::cancelation_manager::{
    CancelationManager, CanceledException, MethodAbortedException, MethodDeadlineExceeded,
};
use kuna_jvm::decompile_record::DecompileRecord;
use kuna_jvm::fernflower::DecompiledClass;
use kuna_jvm::i_fernflower_logger::Severity;
use kuna_jvm::kotlin_writer::is_stop_signal;
use kuna_jvm::kuna_input::{self, InputSet, LoadOptions};
use kuna_jvm::looping_pass_builder::LoopingPassBuilder;
use kuna_jvm::pass_context::PassContext;
use kuna_jvm::rt::{throw, try_catch};
use kuna_jvm::struct_class::StructClass;
use kuna_jvm::var_processor::VarProcessor;

use crate::common;
use crate::kuna_support::{corpus, decompile_with, options, scratch, Logged};

const CLASSES: [&str; 2] = [
    "TestTrySynchronized.class",
    "TestTrySynchronized$Companion.class",
];

fn input(test: &str) -> InputSet {
    let dir = scratch(&format!("kt8-{test}")).join("pkg");
    std::fs::create_dir_all(&dir).unwrap();
    for name in CLASSES {
        std::fs::copy(corpus(&format!("kotlin/kt/pkg/{name}")), dir.join(name)).unwrap();
    }
    kuna_input::load(dir.parent().unwrap(), &LoadOptions::default()).unwrap()
}

fn method_text<'a>(unit: &'a DecompiledClass, name: &str) -> &'a str {
    let m = unit.methods.iter().find(|m| m.name == name).unwrap();
    &unit.content[m.start..m.end]
}

fn has(log: &[Logged], severity: Severity, text: &str) -> bool {
    log.iter()
        .any(|l| l.severity == severity && l.message == text)
}

#[test]
fn a_kotlin_class_gets_class_and_method_spans() {
    let (units, _) = decompile_with(&input("spans"), &options(&[]), |_| {});
    assert_eq!(units.len(), 1);
    let unit = &units[0];
    assert_eq!(unit.path, "pkg/TestTrySynchronized.kt");
    assert_eq!(unit.language(), "kotlin");
    assert!(unit.error.is_none());

    let classes: Vec<&str> = unit.classes.iter().map(|c| c.internal.as_str()).collect();
    assert_eq!(
        classes,
        [
            "pkg/TestTrySynchronized",
            "pkg/TestTrySynchronized$Companion"
        ]
    );
    let outer = &unit.classes[0];
    assert!(unit.content[outer.start..outer.end].starts_with("public class TestTrySynchronized {"));
    let companion = &unit.classes[1];
    assert!(unit.content[companion.start..companion.end]
        .trim_start()
        .starts_with("public companion object {"));
    assert!(outer.start <= companion.start && companion.end <= outer.end);

    let methods: Vec<(&str, &str)> = unit
        .methods
        .iter()
        .map(|m| (m.name.as_str(), m.descriptor.as_str()))
        .collect();
    assert_eq!(
        methods,
        [
            ("case01", "()V"),
            ("case02", "()V"),
            ("getInputStream", "()Ljava/io/InputStream;")
        ]
    );
    for m in &unit.methods {
        assert!(m.error.is_none(), "{}: {:?}", m.name, m.error);
        assert!(outer.start <= m.start && m.end <= outer.end, "{}", m.name);
        assert!(!m.lines.is_empty(), "{} has no line mapping", m.name);
    }
    let case01 = method_text(unit, "case01");
    assert!(case01.starts_with("   @Throws(Exception::class)\n   public fun case01() {"));
    assert!(case01.trim_end().ends_with('}'));
    assert!(method_text(unit, "getInputStream")
        .trim_start()
        .starts_with("private final val inputStream: InputStream?"));
}

#[test]
fn with_the_kotlin_plugin_off_the_class_is_written_as_java() {
    for (key, value) in [("jvm_plugins", "0"), ("kt-enable", "0")] {
        let (units, _) = decompile_with(&input(key), &options(&[(key, value)]), |_| {});
        assert_eq!(units[0].path, "pkg/TestTrySynchronized.java", "{key}");
        assert_eq!(units[0].language(), "java");
        assert!(units[0].content.contains("@Metadata("));
        assert!(units[0]
            .methods
            .iter()
            .any(|m| m.name == "case01" && m.error.is_none()));
    }
}

#[test]
fn a_kotlin_method_past_its_deadline_fails_like_a_vineflower_timeout() {
    let input = input("deadline");
    let opts = options(&[("max-time-per-method", "1")]);
    let (reference, _) = decompile_with(&input, &opts, |_| {});
    let (units, log) = decompile_with(&input, &opts, |method| {
        CancelationManager::set_method_guard(Some(Rc::new(move || {
            if method.borrow().as_str() == "case02 ()V" {
                std::thread::sleep(Duration::from_millis(400));
            }
            None
        })));
    });
    let unit = &units[0];
    assert_eq!(unit.path, "pkg/TestTrySynchronized.kt");
    let span = unit.methods.iter().find(|m| m.name == "case02").unwrap();
    assert_eq!(span.error.as_deref(), Some("TimeoutException"));
    let text = method_text(unit, "case02");
    assert!(text.contains("public fun case02() {"), "{text}");
    assert!(text.contains("// $VF: Couldn't be decompiled"), "{text}");
    assert!(text.contains("// TimeoutException"), "{text}");
    assert!(has(
        &log,
        Severity::Error,
        "Processing time limit exceeded for method case02, execution interrupted."
    ));
    assert!(!log.iter().any(|l| l.message.contains("couldn't be")));
    for name in ["case01", "getInputStream"] {
        assert_eq!(method_text(unit, name), method_text(&reference[0], name));
    }
}

#[test]
fn a_guard_reason_fails_only_that_kotlin_method() {
    let input = input("guard");
    let opts = options(&[]);
    let (reference, _) = decompile_with(&input, &opts, |_| {});
    let (units, log) = decompile_with(&input, &opts, |method| {
        CancelationManager::set_method_guard(Some(Rc::new(move || {
            (method.borrow().as_str() == "case01 ()V").then(|| "over budget".to_string())
        })));
    });
    let unit = &units[0];
    let span = unit.methods.iter().find(|m| m.name == "case01").unwrap();
    assert_eq!(
        span.error.as_deref(),
        Some("MethodAbortedException: over budget")
    );
    assert!(method_text(unit, "case01").contains("// $VF: Couldn't be decompiled"));
    assert!(has(
        &log,
        Severity::Warn,
        "Method case01 ()V in class pkg/TestTrySynchronized couldn't be decompiled."
    ));
    for name in ["case02", "getInputStream"] {
        assert_eq!(method_text(unit, name), method_text(&reference[0], name));
    }
}

/// The Kotlin schedule's `Main`/`Merge` loops poll the method deadline at the
/// head of every round, as the Java schedule's loops do.
#[test]
fn looping_passes_poll_the_method_deadline() {
    let _guard = common::install_default_context();
    let bytes = std::fs::read(corpus("kotlin/kt/pkg/TestTrySynchronized.class")).unwrap();
    let cl = StructClass::create(&mut ByteCursor::new(bytes), true);
    let mt = cl.get_method("case01", "()V").unwrap();
    let mut ctx = PassContext::new(
        None,
        None,
        mt,
        cl,
        VarProcessor::new(mt, &mt.method_descriptor()),
        DecompileRecord::new(mt),
    );
    let rounds = Rc::new(Cell::new(0));
    let counted = rounds.clone();
    let pass = LoopingPassBuilder::new("Main")
        .add_looping_pass(
            "Again",
            Rc::new(move |_: &mut PassContext| {
                counted.set(counted.get() + 1);
                counted.get() < 1000
            }),
        )
        .build();

    assert!(pass.run(&mut ctx));
    assert_eq!(rounds.get(), 1000);

    rounds.set(0);
    let previous = CancelationManager::set_method_deadline(Some(Instant::now()));
    let caught = try_catch(|| pass.run(&mut ctx)).unwrap_err();
    CancelationManager::set_method_deadline(previous);
    assert!(
        caught.is(MethodDeadlineExceeded::CLASS),
        "{}",
        caught.message
    );
    assert_eq!(rounds.get(), 0);
}

#[test]
fn the_kotlin_writer_passes_on_only_kunas_stop_signals() {
    let caught = |class: &'static str| try_catch(|| throw(class, "")).unwrap_err();
    assert!(is_stop_signal(
        &try_catch(|| CanceledException::throw()).unwrap_err()
    ));
    assert!(is_stop_signal(&caught(MethodDeadlineExceeded::CLASS)));
    assert!(!is_stop_signal(&caught(MethodAbortedException::CLASS)));
    assert!(!is_stop_signal(&caught("NullPointerException")));
    assert!(!is_stop_signal(&try_catch(|| panic!("index")).unwrap_err()));
}
