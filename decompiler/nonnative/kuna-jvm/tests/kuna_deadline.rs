//! The cooperative per-method deadline and the method guard.

use std::rc::Rc;
use std::time::Duration;

use kuna_jvm::cancelation_manager::CancelationManager;
use kuna_jvm::fernflower::DecompiledClass;
use kuna_jvm::i_fernflower_logger::Severity;
use kuna_jvm::kuna_input::{self, InputSet, LoadOptions};

use crate::kuna_support::{corpus, decompile_with, options, Logged};

fn input() -> InputSet {
    kuna_input::load(
        &corpus("java8/pkg/TestClassLoop.class"),
        &LoadOptions::default(),
    )
    .unwrap()
}

fn method_text<'a>(unit: &'a DecompiledClass, name: &str) -> &'a str {
    let m = unit.methods.iter().find(|m| m.name == name).unwrap();
    &unit.content[m.start..m.end]
}

fn others_equal(unit: &DecompiledClass, reference: &DecompiledClass, except: &str) {
    for m in unit.methods.iter().filter(|m| m.name != except) {
        assert!(m.error.is_none(), "{} failed: {:?}", m.name, m.error);
        assert_eq!(
            method_text(unit, &m.name),
            method_text(reference, &m.name),
            "{}",
            m.name
        );
    }
}

fn has(log: &[Logged], severity: Severity, text: &str) -> bool {
    log.iter()
        .any(|l| l.severity == severity && l.message == text)
}

#[test]
fn a_method_past_its_deadline_fails_like_a_vineflower_timeout() {
    let input = input();
    let opts = options(&[("max-time-per-method", "1")]);
    let (reference, _) = decompile_with(&input, &opts, |_| {});
    let (units, log) = decompile_with(&input, &opts, |method| {
        CancelationManager::set_method_guard(Some(Rc::new(move || {
            if method.borrow().as_str() == "testFinally ()V" {
                std::thread::sleep(Duration::from_millis(400));
            }
            None
        })));
    });
    let unit = &units[0];
    let span = unit
        .methods
        .iter()
        .find(|m| m.name == "testFinally")
        .unwrap();
    assert_eq!(span.error.as_deref(), Some("TimeoutException"));
    let text = method_text(unit, "testFinally");
    assert!(text.contains("// $VF: Couldn't be decompiled"), "{text}");
    assert!(text.contains("// TimeoutException"), "{text}");
    assert!(text.contains("// Bytecode:"), "{text}");
    assert!(has(
        &log,
        Severity::Error,
        "Processing time limit exceeded for method testFinally, execution interrupted."
    ));
    assert!(!log
        .iter()
        .any(|l| l.message.contains("testFinally ()V in class")));
    others_equal(unit, &reference[0], "testFinally");
}

#[test]
fn without_a_limit_a_slow_method_still_completes() {
    let input = input();
    let opts = options(&[]);
    let (reference, _) = decompile_with(&input, &opts, |_| {});
    let (units, _) = decompile_with(&input, &opts, |method| {
        CancelationManager::set_method_guard(Some(Rc::new(move || {
            if method.borrow().as_str() == "testCatch ()V" {
                std::thread::sleep(Duration::from_millis(20));
            }
            None
        })));
    });
    assert_eq!(units[0].content, reference[0].content);
}

#[test]
fn a_guard_reason_fails_only_that_method() {
    let input = input();
    let opts = options(&[]);
    let (reference, _) = decompile_with(&input, &opts, |_| {});
    let (units, log) = decompile_with(&input, &opts, |method| {
        CancelationManager::set_method_guard(Some(Rc::new(move || {
            (method.borrow().as_str() == "testCatch ()V").then(|| "over budget".to_string())
        })));
    });
    let unit = &units[0];
    let span = unit.methods.iter().find(|m| m.name == "testCatch").unwrap();
    assert_eq!(
        span.error.as_deref(),
        Some("MethodAbortedException: over budget")
    );
    assert!(log.iter().any(|l| l.severity == Severity::Warn
        && l.message == "Method testCatch ()V in class pkg/TestClassLoop couldn't be decompiled."));
    others_equal(unit, &reference[0], "testCatch");
}
