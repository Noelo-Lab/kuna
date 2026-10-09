//! Physical writes remain observable through an escaped stack pointer (#810).
mod common;

use std::process::Command;
use std::time::Duration;

fn decompile(function: &str, enabled: bool) -> String {
    let fixture = common::fixture("lifetimes_stack_x86_64");
    let assertions = format!("@{fixture}.kuna");
    let specs = common::repo_root().join("specs");
    let mut command = Command::new(env!("CARGO_BIN_EXE_kuna"));
    command
        .args([
            "decompile",
            &fixture,
            function,
            "--mode",
            "reliable",
            "--assert-strict",
            "--assert",
            &assertions,
            "--option",
            "stackalias",
            if enabled { "on" } else { "off" },
            "--sleighpath",
        ])
        .arg(specs);
    let output = common::process::output_with_timeout(
        &mut command,
        Duration::from_secs(15),
        Duration::from_millis(25),
    )
    .expect("bounded stack decompilation");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap()
}

#[test]
fn escaped_pointer_read_keeps_its_physical_writes() {
    let off = decompile("escaped_alias", false);
    assert!(
        !off.contains(" = 7;"),
        "baseline control no longer reproduces: {off}"
    );
    let on = decompile("escaped_alias", true);
    assert!(on.contains(" = 7;"), "{on}");
    assert!(on.contains("return *escaped_pointer;"), "{on}");

    if common::process::optional_output(Command::new("clang").arg("--version")).is_none() {
        eprintln!("clang unavailable; skipping native emitted-C control");
        return;
    }
    let source = common::scratch_file("escaped-stack-alias", "c");
    std::fs::write(&source, format!(
        "#include <stdint.h>\ntypedef int32_t int4;\nint4 *escaped_pointer;\n{on}\nint main(void) {{ return escaped_alias() != 7; }}\n"
    )).unwrap();
    for optimization in ["-O0", "-O2"] {
        let binary = source.with_extension(&optimization[1..]);
        common::process::required_output(
            Command::new("clang")
                .args(["-std=c11", optimization])
                .arg(&source)
                .arg("-o")
                .arg(&binary),
        );
        common::process::required_output(&mut Command::new(binary));
    }
    common::process::required_output(&mut Command::new(common::fixture("lifetimes_stack_x86_64")));
}

#[test]
fn calls_partial_writes_branches_loops_and_indirect_writers_stay_bounded() {
    for function in [
        "escaped_two_reads",
        "escaped_partial",
        "escaped_branch",
        "escaped_loop",
        "callback_rewrite",
    ] {
        let on = decompile(function, true);
        assert!(!on.contains("WARNING:"), "{function}: {on}");
        if function == "callback_rewrite" {
            assert!(on.contains("(*callback)"), "{on}");
        } else {
            assert!(on.contains("read_escaped()"), "{on}");
        }
    }
}

#[test]
fn indirect_writer_keeps_its_declared_function_pointer() {
    let code = decompile("callback_rewrite", true);
    assert!(code.contains("void (*callback)(int4 *)"), "{code}");
    if common::process::optional_output(Command::new("clang").arg("--version")).is_none() {
        eprintln!("clang unavailable; skipping emitted callback execution");
        return;
    }
    let source = common::scratch_file("typed-stack-callback", "c");
    std::fs::write(&source, format!(
        "#include <stdint.h>\ntypedef int32_t int4;\n{code}\nvoid set_seven(int4 *p) {{ *p = 7; }}\nint main(void) {{ return callback_rewrite(set_seven) != 7; }}\n"
    )).unwrap();
    for optimization in ["-O0", "-O2"] {
        let binary = source.with_extension(&optimization[1..]);
        common::process::required_output(
            Command::new("clang").args(["-std=c11", optimization]).arg(&source).arg("-o").arg(&binary),
        );
        common::process::required_output(&mut Command::new(binary));
    }
}
