//! Inputs that would end the run or never finish. The recursion guard
//! (`rt::stack`): input nested deeper than an engine thread's stack fails one
//! method or class with `StackOverflowError`, and the run goes on. The method
//! budget counts polls; a three-letter option name is its long name.
//! Fixtures in `tests/jvm/stackguard/`: `TestDeepExpression` (javac 21.0.12
//! `--release 17 -g`, a sum 300 terms deep), `TestDeepField` (javac 21.0.13.1
//! `--release 17 -g`, the same depth in a field initializer) and `TestLambdaCycle`
//! (asmtools-core 7.0.b10-ea jasm on JDK 17.0.20.1: two synthetic lambda
//! bodies that create each other's lambdas, which Vineflower nests without
//! end).

use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::Arc;

use kuna_jvm::fernflower::canonical_option;
use kuna_jvm::i_fernflower_logger::IFernflowerLogger;
use kuna_jvm::kuna_driver::{self, ClassResult, DriverConfig, Jobs};
use kuna_jvm::kuna_input::{self, LoadOptions};

use crate::kuna_support::{options, scratch, RecordingLogger};

fn fixture(dir: &str, classes: &[&str]) -> PathBuf {
    let from = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../tests/jvm/stackguard/classes/pkg");
    let to = scratch(dir).join("pkg");
    std::fs::create_dir_all(&to).unwrap();
    for class in classes {
        std::fs::copy(from.join(class), to.join(class)).unwrap();
    }
    to.parent().unwrap().to_path_buf()
}

fn run(dir: &Path, extra: &[(&str, &str)], stack_size: Option<usize>) -> Vec<ClassResult> {
    let input = kuna_input::load(dir, &LoadOptions::default()).unwrap();
    let mut config = DriverConfig::new(
        options(extra),
        Arc::new(|| Rc::new(RecordingLogger::new()) as Rc<dyn IFernflowerLogger>),
    );
    config.jobs = Jobs::Exactly(1);
    if let Some(bytes) = stack_size {
        config.stack_size = bytes;
    }
    let mut results = Vec::new();
    kuna_driver::run(&input, &config, &mut |r| results.push(r)).unwrap();
    results
}

fn text<'a>(results: &'a [ClassResult], name: &str) -> &'a str {
    let result = results.iter().find(|r| r.qualified_name == name).unwrap();
    &result.unit.as_ref().unwrap_or_else(|| panic!("{name}: {:?}", result.error)).content
}

#[test]
fn an_expression_deeper_than_the_stack_fails_only_its_method() {
    let dir = fixture("stackguard-deep", &["TestDeepExpression.class"]);
    let small = run(&dir, &[], Some(1 << 20));
    let out = text(&small, "pkg/TestDeepExpression");
    assert!(out.contains("// $VF: Couldn't be decompiled"), "{out}");
    assert!(out.contains("StackOverflowError"), "{out}");
    assert!(out.contains("return a + 1;"), "the shallow method still decompiles: {out}");

    let full = run(&dir, &[], None);
    let out = text(&full, "pkg/TestDeepExpression");
    assert!(!out.contains("Couldn't be decompiled"), "{out}");
    assert_eq!(out.matches("+ b").count(), 150, "{out}");
}

#[test]
fn an_initializer_deeper_than_the_stack_fails_its_class_and_the_run_goes_on() {
    let dir = fixture("stackguard-field", &["TestDeepField.class", "TestDeepExpression.class"]);
    let small = run(&dir, &[], Some(1 << 20));
    let out = text(&small, "pkg/TestDeepField");
    assert!(out.contains("$VF: Unable to decompile class"), "{out}");
    assert!(out.contains("StackOverflowError"), "{out}");
    let other = text(&small, "pkg/TestDeepExpression");
    assert!(other.contains("return a + 1;"), "{other}");

    let full = run(&dir, &[], None);
    let out = text(&full, "pkg/TestDeepField");
    assert!(!out.contains("decompile"), "{out}");
    assert_eq!(out.matches("A[0]").count(), 300, "{out}");
}

#[test]
fn a_lambda_cycle_does_not_nest_without_end() {
    let dir = fixture(
        "stackguard-cycle",
        &["TestLambdaCycle.class", "TestLambdaCycle$Task.class", "TestDeepExpression.class"],
    );
    let results = run(&dir, &[], Some(8 << 20));
    let cycle = text(&results, "pkg/TestLambdaCycle");
    assert!(!cycle.contains("StackOverflowError"), "{cycle}");
    let other = text(&results, "pkg/TestDeepExpression");
    assert!(other.contains("return a + 1;"), "{other}");
}

#[test]
fn a_three_letter_option_name_is_its_long_name() {
    assert_eq!(
        canonical_option("mpm", "120"),
        ("max-time-per-method".to_string(), "120".to_string())
    );
    assert_eq!(canonical_option("din", "true"), ("decompile-inner".to_string(), "1".to_string()));
    assert_eq!(
        canonical_option("max-time-per-method", "5"),
        ("max-time-per-method".to_string(), "5".to_string())
    );
}

#[test]
fn the_method_budget_counts_polls_not_time() {
    use kuna_jvm::cancelation_manager::{CancelationManager, MethodDeadlineExceeded};
    let previous = CancelationManager::set_method_poll_budget(Some(3));
    for _ in 0..3 {
        CancelationManager::check_canceled();
    }
    let caught = kuna_jvm::rt::try_catch(CancelationManager::check_canceled).unwrap_err();
    assert!(caught.is(MethodDeadlineExceeded::CLASS), "{}", caught.message);
    CancelationManager::set_method_poll_budget(previous);
    CancelationManager::check_canceled();
}
