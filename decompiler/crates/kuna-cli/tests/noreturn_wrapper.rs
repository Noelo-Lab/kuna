//! Explicit no-return facts seed wrapper propagation before bounded caller flow.
#![cfg(all(target_os = "linux", target_arch = "x86_64"))]
mod common;

use std::path::{Path, PathBuf};
use std::process::Command;

use object::{Object, ObjectSymbol, SymbolKind};
use serde_json::Value;

struct Fixture {
    image: PathBuf,
    assertions: PathBuf,
    native: PathBuf,
}

fn fixture() -> Option<Fixture> {
    common::process::optional_output(Command::new("cc").arg("--version"))?;
    let source = common::scratch_file("noreturn-wrapper", "S");
    let driver = common::scratch_file("noreturn-wrapper-driver", "c");
    let image = common::scratch_file("noreturn-wrapper", "elf");
    let native = common::scratch_file("noreturn-wrapper-native", "elf");
    let assertions = common::scratch_file("noreturn-wrapper", "kuna");
    std::fs::write(&source, include_str!("fixtures/noreturn_wrapper.S")).unwrap();
    std::fs::write(&driver, include_str!("fixtures/noreturn_wrapper_driver.c")).unwrap();
    common::process::required_output(
        Command::new("cc")
            .args([
                "-nostdlib",
                "-no-pie",
                "-Wl,--build-id=none",
                "-Wl,-e,bounded_tail",
            ])
            .arg(&source)
            .arg("-o")
            .arg(&image),
    );
    common::process::required_output(
        Command::new("cc")
            .arg("-no-pie")
            .arg(&source)
            .arg(&driver)
            .arg("-o")
            .arg(&native),
    );
    let bytes = std::fs::read(&image).unwrap();
    let file = object::File::parse(bytes.as_slice()).unwrap();
    let mut facts = String::new();
    for symbol in file
        .symbols()
        .filter(|s| s.kind() == SymbolKind::Text && s.is_definition())
    {
        let name = symbol.name().unwrap();
        let start = symbol.address();
        let end = start + symbol.size();
        facts.push_str(&format!("function {start:#x}-{end:#x}={name}\n"));
        let declaration = match name {
            "bounded_tail" | "returning_caller" | "bounded_missing" => {
                format!("int {name}(int fail)")
            }
            "returning_wrapper" | "missing_wrapper" => format!("void {name}(int fail)"),
            "unknown_wrapper" => format!("void {name}(int fail, void *target)"),
            _ => format!("void {name}(void)"),
        };
        facts.push_str(&format!("prototype {start:#x} {declaration}\n"));
    }
    std::fs::write(&assertions, facts).unwrap();
    Some(Fixture {
        image,
        assertions,
        native,
    })
}

fn decompile(fixture: &Fixture, options: &[&str], single: bool) -> Value {
    let assertion = format!("@{}", fixture.assertions.display());
    let mut args = vec![
        if single { "decompile" } else { "decompile-all" },
        fixture.image.to_str().unwrap(),
    ];
    if single {
        args.push("bounded_tail");
    } else {
        args.extend([
            "--functions",
            "bounded_tail,outer_wrapper,returning_caller,bounded_missing",
        ]);
    }
    args.extend([
        "--mode",
        "reliable",
        "--json",
        "--assert-strict",
        "--assert",
        &assertion,
    ]);
    args.extend(options);
    let (out, err, rc) = common::run_kuna(&args);
    assert_eq!(rc, 0, "{out}\n{err}");
    let doc: Value = serde_json::from_str(&out).expect("valid JSON");
    for outcome in doc["assertions"].as_array().expect("assertion outcomes") {
        assert_eq!(outcome["status"], "applied", "{outcome}");
    }
    doc
}

fn code<'a>(doc: &'a Value, name: &str) -> &'a str {
    let function = doc["functions"]
        .as_array()
        .unwrap()
        .iter()
        .find(|f| f["name"] == name)
        .unwrap();
    assert!(function["error"].is_null(), "{function}");
    function["code"].as_str().unwrap()
}

fn native_exit(native: &Path, argument: Option<&str>, expected: i32) {
    let mut command = Command::new(native);
    if let Some(argument) = argument {
        command.arg(argument);
    }
    assert_eq!(command.status().unwrap().code(), Some(expected));
}

fn check_emitted(doc: &Value) {
    let source = common::scratch_file("noreturn-wrapper-emitted", "c");
    let helpers = common::scratch_file("noreturn-wrapper-helpers", "S");
    let driver = common::scratch_file("noreturn-wrapper-emitted-driver", "c");
    let executable = common::scratch_file("noreturn-wrapper-emitted", "elf");
    let mut assembly = include_str!("fixtures/noreturn_wrapper.S").to_owned();
    for name in ["bounded_tail", "returning_caller"] {
        let start = assembly.find(&format!(".globl {name}\n")).unwrap();
        let end_marker = format!(".size {name}, .-{name}\n");
        let end = assembly.find(&end_marker).unwrap() + end_marker.len();
        assembly.replace_range(start..end, "");
    }
    std::fs::write(&helpers, assembly).unwrap();
    std::fs::write(&driver, include_str!("fixtures/noreturn_wrapper_driver.c")).unwrap();
    std::fs::write(
        &source,
        format!(
            "typedef int int4;\nvoid fail_wrapper(void);\nvoid returning_wrapper(int);\n{}\n{}\n",
            code(doc, "bounded_tail"),
            code(doc, "returning_caller"),
        ),
    )
    .unwrap();
    common::process::required_output(
        Command::new("cc")
            .arg("-no-pie")
            .arg(&source)
            .arg(&helpers)
            .arg(&driver)
            .arg("-o")
            .arg(&executable),
    );
    native_exit(&executable, None, 0);
    native_exit(&executable, Some("fail"), 73);
    native_exit(&executable, Some("control-fail"), 73);
}

#[test]
fn leaf_marking_propagates_through_call_and_trap_wrappers() {
    let Some(fixture) = fixture() else { return };
    native_exit(&fixture.native, None, 0);
    native_exit(&fixture.native, Some("fail"), 73);
    native_exit(&fixture.native, Some("control-fail"), 73);

    let default = decompile(&fixture, &[], false);
    assert!(code(&default, "bounded_tail").contains("halt_missing"));
    let marked = decompile(&fixture, &["--option", "noreturn", "fail_leaf"], false);
    let caller = code(&marked, "bounded_tail");
    assert!(!caller.contains("halt_missing"), "{caller}");
    assert!(caller.contains("return 5;"), "{caller}");
    assert!(caller.contains("fail_wrapper(); // no-return"), "{caller}");
    assert!(code(&marked, "outer_wrapper").contains("fail_wrapper(); // no-return"));
    let control = code(&marked, "returning_caller");
    assert!(control.contains("return 9;"), "{control}");
    assert!(!control.contains("// no-return"), "{control}");
    let missing = code(&marked, "bounded_missing");
    assert!(missing.contains("halt_missing"), "{missing}");
    check_emitted(&marked);

    for gate in ["listing", "noreturn_propagate", "noreturn_reach"] {
        let off = decompile(
            &fixture,
            &["--option", "noreturn", "fail_leaf", "--option", gate, "off"],
            false,
        );
        assert!(
            code(&off, "bounded_tail").contains("halt_missing"),
            "{gate}: {off}"
        );
    }
    let immediate = decompile(&fixture, &["--option", "noreturn", "fail_wrapper"], false);
    assert!(!code(&immediate, "bounded_tail").contains("halt_missing"));
    let explicit_only = decompile(
        &fixture,
        &[
            "--option",
            "noreturn",
            "fail_leaf",
            "--option",
            "noreturn_known",
            "off",
        ],
        false,
    );
    assert!(!code(&explicit_only, "bounded_tail").contains("halt_missing"));

    let single = decompile(&fixture, &["--option", "noreturn", "fail_leaf"], true);
    let caller = code(&single, "bounded_tail");
    assert!(!caller.contains("halt_missing"), "{single}");
    assert!(caller.contains("fail_wrapper(); // no-return"), "{single}");

    let assertion = format!("@{}", fixture.assertions.display());
    let (plain, err, rc) = common::run_kuna(&[
        "decompile",
        fixture.image.to_str().unwrap(),
        "bounded_tail",
        "--mode",
        "reliable",
        "--assert-strict",
        "--assert",
        &assertion,
        "--option",
        "noreturn",
        "fail_leaf",
    ]);
    assert_eq!(rc, 0, "{plain}\n{err}");
    assert!(!plain.contains("halt_missing"), "{plain}");
    assert!(plain.contains("fail_wrapper(); // no-return"), "{plain}");
}

#[test]
fn propagation_keeps_returning_unknown_and_incomplete_paths() {
    let Some(fixture) = fixture() else { return };
    let specs = vec![common::repo_root()
        .join("specs")
        .to_str()
        .unwrap()
        .to_owned()];
    let mut program =
        kuna_console::engine::bootstrap_from_object(fixture.image.to_str().unwrap(), "", &specs)
            .unwrap();
    program.arch_mut().set_kuna_option("listing", "on").unwrap();
    use kuna_decomp::options::ArchOptionContext;
    program
        .arch_mut()
        .set_function_no_return("fail_leaf", true)
        .unwrap();
    program.commit_pending_analysis().unwrap();
    for (name, expected) in [
        ("fail_leaf", true),
        ("fail_wrapper", true),
        ("outer_wrapper", true),
        ("bounded_tail", false),
        ("returning_wrapper", false),
        ("returning_caller", false),
        ("unknown_wrapper", false),
        ("missing_wrapper", false),
        ("ordinary_leaf", false),
        ("bounded_missing", false),
        ("trap_only", false),
    ] {
        let (_, addr) = program
            .function_entries()
            .find(|(n, _)| *n == name)
            .unwrap();
        assert_eq!(
            program
                .arch()
                .symboltab
                .function_is_no_return_across_scopes(addr),
            expected,
            "{name}"
        );
    }
}
