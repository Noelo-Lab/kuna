//! Callee bodies, rather than const pointee declarations, bound frame observers.
mod common;

use std::process::Command;
use std::time::Duration;

fn decompile(function: &str, enabled: bool) -> String {
    decompile_fixture("lifetimes_callee_x86_64", function, enabled)
}

fn decompile_fixture(name: &str, function: &str, enabled: bool) -> String {
    let fixture = common::fixture(name);
    let output = common::process::output_with_timeout(
        Command::new(env!("CARGO_BIN_EXE_kuna"))
            .args([
                "decompile",
                &fixture,
                function,
                "--mode",
                "reliable",
                "--assert-strict",
                "--assert",
            ])
            .arg(format!("@{fixture}.kuna"))
            .args([
                "--option",
                "stackviews",
                if enabled { "on" } else { "off" },
                "--option",
                "stackalias",
                "off",
                "--option",
                "structdefs",
                "on",
                "--sleighpath",
            ])
            .arg(common::repo_root().join("specs")),
        Duration::from_secs(15),
        Duration::from_millis(25),
    )
    .expect("bounded callee lifetime decompilation");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap()
}

fn native(function: &str, code: &str, cases: &[(&str, i32)]) {
    native_abi(function, code, cases, None);
}

fn native_abi(function: &str, code: &str, cases: &[(&str, i32)], stack_abi: Option<bool>) {
    let prelude = r#"
typedef unsigned char uint1; typedef unsigned short uint2;
typedef unsigned int uint4; typedef unsigned long long uint8;
typedef signed char int1; typedef signed short int2;
typedef signed int int4; typedef signed long long int8;
typedef unsigned char undefined1;
struct CalleePair;
struct CalleePair *callee_saved_pointer;
struct CalleePair *stack_saved_pointer;
int callee_read_pair(const struct CalleePair *);
int callee_read_second(const struct CalleePair *);
int callee_no_memory(const struct CalleePair *);
int callee_hidden_read(const struct CalleePair *);
int callee_read_choice(const struct CalleePair *, int);
int callee_const_write(const struct CalleePair *);
"#;
    let helpers = r#"
int callee_read_pair(const struct CalleePair *p) { return p->first + p->second; }
int callee_read_second(const struct CalleePair *p) { return p->second; }
int callee_no_memory(const struct CalleePair *p) { return 3; }
int callee_hidden_read(const struct CalleePair *p) { return callee_saved_pointer->first; }
int callee_read_choice(const struct CalleePair *p, int c) { return c ? p->second : p->first; }
int callee_const_write(const struct CalleePair *p) { ((struct CalleePair *)p)->first = 9; return 0; }
"#;
    let checks = cases
        .iter()
        .map(|(arg, expected)| format!("{function}({arg}) != {expected}"))
        .collect::<Vec<_>>()
        .join(" || ");
    let (stack_prototypes, stack_helpers, entry) = if let Some(is32) = stack_abi {
        let args = if is32 {
            ""
        } else {
            "long a, long b, long c, long d, long e, long f, "
        };
        (
            format!("int stack_read_pair(const struct CalleePair *);\nint stack_read_second({args}const struct CalleePair *p);\nint stack_hidden_read({args}const struct CalleePair *p);\nint stack_const_write({args}const struct CalleePair *p);\nstruct CalleePair *stack_return_pointer({args}const struct CalleePair *p);"),
            format!("int stack_read_pair(const struct CalleePair *p) {{ return p->first + p->second; }}\nint stack_read_second({args}const struct CalleePair *p) {{ return p->second; }}\nint stack_hidden_read({args}const struct CalleePair *p) {{ return p->first; }}\nint stack_const_write({args}const struct CalleePair *p) {{ ((struct CalleePair *)p)->first = 9; return 0; }}\nstruct CalleePair *stack_return_pointer({args}const struct CalleePair *p) {{ return stack_saved_pointer; }}"),
            if is32 {
                r#"void _start(void) { int status = main(); __asm__ volatile("int $0x80" : : "a"(1), "b"(status) : "memory"); __builtin_unreachable(); }"#
            } else { "" },
        )
    } else {
        (String::new(), String::new(), "")
    };
    let source = common::scratch_file(function, "c");
    std::fs::write(
        &source,
        format!("{prelude}\n{stack_prototypes}\n{code}\n{helpers}\n{stack_helpers}\nint main(void) {{ return {checks}; }}\n{entry}\n"),
    )
    .unwrap();
    let compilers: Vec<_> = ["gcc", "clang"]
        .into_iter()
        .filter(|c| common::process::optional_output(Command::new(c).arg("--version")).is_some())
        .collect();
    if compilers.is_empty() {
        eprintln!("no C compiler available; skipping emitted-C execution");
        return;
    }
    for compiler in compilers {
        for level in ["-O0", "-O2"] {
            let binary = source.with_extension(format!("{compiler}{}", &level[1..]));
            let mut compile = Command::new(compiler);
            compile.args(["-std=c11", level, "-fstrict-aliasing"]);
            if stack_abi == Some(true) {
                compile.args([
                    "-m32",
                    "-nostdlib",
                    "-fno-pie",
                    "-no-pie",
                    "-fno-stack-protector",
                    "-Wl,-e,_start",
                ]);
            }
            common::process::required_output(compile.arg(&source).arg("-o").arg(&binary));
            common::process::required_output(&mut Command::new(&binary));
        }
    }
}

#[test]
fn proven_leaf_observers_allow_unread_bytes_to_be_overwritten() {
    common::process::required_output(&mut Command::new(common::fixture(
        "lifetimes_callee_x86_64",
    )));
    for (function, expected) in [
        ("overwrite_after_read_second", 23),
        ("overwrite_after_no_memory", 19),
    ] {
        let baseline = decompile(function, false);
        assert!(baseline.contains(" = 5;"), "{baseline}");
        let code = decompile(function, true);
        assert!(!code.contains(" = 5;"), "{code}");
        assert!(code.contains(" = 7;"), "{code}");
        assert!(code.contains(" = 9;"), "{code}");
        native(function, &code, &[("", expected)]);
    }
}

#[test]
fn hidden_aliases_all_branch_reads_and_const_writes_keep_native_behavior() {
    for (function, cases) in [
        ("overwrite_after_hidden_read", vec![("", 21)]),
        ("overwrite_after_branch_read", vec![("0", 21), ("1", 23)]),
        ("overwrite_after_const_write", vec![("", 16)]),
        ("read_after_const_write", vec![("", 16)]),
    ] {
        let code = decompile(function, true);
        if matches!(
            function,
            "overwrite_after_hidden_read" | "overwrite_after_branch_read"
        ) {
            assert!(code.contains(" = 5;"), "{code}");
        }
        native(function, &code, &cases);
    }
}

#[test]
fn stack_arguments_and_direct_frame_effects_preserve_native_cdecl_behavior() {
    stack_argument_cases("lifetimes_callee_stack_x86", true);
}

#[test]
fn stack_arguments_and_direct_frame_effects_preserve_native_sysv_behavior() {
    stack_argument_cases("lifetimes_callee_stack_x86_64", false);
}

fn stack_argument_cases(fixture: &str, is32: bool) {
    common::process::required_output(&mut Command::new(common::fixture(fixture)));
    let baseline = decompile_fixture(fixture, "overwrite_after_stack_read", false);
    assert!(baseline.contains(" = 5;"), "{baseline}");
    for (function, expected) in [
        ("overwrite_after_stack_read", 23),
        ("overwrite_after_stack_hidden", 21),
        ("read_after_stack_write", 16),
        ("returned_pointer_read_before_overwrite", 21),
        ("returned_pointer_read_after_overwrite", 25),
    ] {
        let code = decompile_fixture(fixture, function, true);
        if function == "overwrite_after_stack_read" {
            assert!(!code.contains(" = 5;"), "{code}");
            assert!(code.contains(" = 7;"), "{code}");
            assert!(code.contains(" = 9;"), "{code}");
        } else if matches!(
            function,
            "overwrite_after_stack_hidden" | "returned_pointer_read_before_overwrite"
        ) {
            assert!(code.contains(" = 5;"), "{code}");
        } else if function == "returned_pointer_read_after_overwrite" {
            assert!(!code.contains(" = 5;"), "{code}");
        }
        native_abi(function, &code, &[("", expected)], Some(is32));
    }
}
