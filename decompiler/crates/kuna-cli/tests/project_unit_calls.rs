//! A project export declares the p-code helpers (`SCARRY4`, `SUB168`) and
//! user-defined operations (`invalidInstructionException`) its C calls, and its
//! `undefined16` takes arithmetic, so the export compiles as C11 and as C2x,
//! where `T f();` stops meaning "any arguments".
use crate::common;
use common::process;
use std::path::PathBuf;
use std::process::Command;

fn export(fixture: &str) -> PathBuf {
    let dir = common::scratch_file(fixture, "kuna");
    let specs = common::repo_root().join("specs");
    let (_stdout, stderr, code) = common::run_kuna(&[
        "decompile-project",
        &common::fixture(fixture),
        "-o",
        dir.to_str().unwrap(),
        "--sleighpath",
        specs.to_str().unwrap(),
    ]);
    assert_eq!(code, 0, "kuna decompile-project {fixture}: {stderr}");
    dir
}

fn declares(header: &str, name: &str) -> bool {
    header.lines().any(|line| {
        line.starts_with("extern ")
            && line.split(|c: char| !c.is_ascii_alphanumeric() && c != '_').any(|word| word == name)
    })
}

#[test]
fn an_export_that_calls_helpers_and_operations_compiles() {
    if process::optional_output(Command::new("cc").arg("--version")).is_none() {
        eprintln!("an_export_that_calls_helpers_and_operations_compiles: skipping (no `cc`)");
        return;
    }
    // `checked_arith_x86_64`'s `scale` checks its add with SCARRY4 and traps
    // with `ud1`; `argclobber_pair_x86_64`'s `mk` returns a 16-byte rax:rdx
    // pair, which its caller splits with SUB168.
    for (fixture, names) in [
        ("checked_arith_x86_64", &["SCARRY4", "invalidInstructionException"][..]),
        ("argclobber_pair_x86_64", &["SUB168"][..]),
    ] {
        let dir = export(fixture);
        let c = dir.join(format!("{fixture}.c"));
        let header = std::fs::read_to_string(dir.join(format!("{fixture}.h"))).unwrap();
        let body = std::fs::read_to_string(&c).unwrap();
        for name in names {
            assert!(body.contains(&format!("{name}(")), "{fixture} calls no {name}:\n{body}");
            assert!(declares(&header, name), "{fixture}'s header does not declare {name}:\n{header}");
        }
        for standard in ["-std=c11", "-std=c2x"] {
            let out = Command::new("cc")
                .args([standard, "-fsyntax-only", "-Werror=implicit-function-declaration", "-Werror=undef"])
                .arg(&c)
                .output()
                .expect("spawn cc");
            assert!(
                out.status.success(),
                "{fixture}'s export failed cc {standard}:\n{}",
                String::from_utf8_lossy(&out.stderr)
            );
        }
    }
}
