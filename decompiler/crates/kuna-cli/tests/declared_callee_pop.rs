//! Declared callee cleanup keeps subsequent stack loads at their original addresses,
//! and a `__cdecl` callee declared without naming its convention is not credited
//! with popping its own arguments.
#![cfg(all(target_os = "linux", target_arch = "x86_64"))]
use crate::common;
use std::process::{Command, Output};

fn checked(command: &mut Command) -> Output {
    let output = command.output().expect("start fixture command");
    assert!(
        output.status.success(),
        "{command:?}: {:?}\n{}\n{}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    output
}

#[test]
fn declared_callee_pop_preserves_stack_addresses() {
    if !Command::new("clang")
        .arg("--version")
        .output()
        .is_ok_and(|output| output.status.success())
    {
        eprintln!("Clang unavailable; skipping cross-target fixture");
        return;
    }
    let root = common::scratch_file("declared-callee-pop", "dir");
    std::fs::create_dir_all(&root).unwrap();
    for (name, contents) in [
        (
            "callbacks.h",
            include_str!("fixtures/declared_callee_pop/callbacks.h"),
        ),
        (
            "sample.s",
            include_str!("fixtures/declared_callee_pop/sample.s"),
        ),
        (
            "support.c",
            include_str!("fixtures/declared_callee_pop/support.c"),
        ),
        (
            "start.s",
            include_str!("fixtures/declared_callee_pop/start.s"),
        ),
        (
            "driver.c",
            include_str!("fixtures/declared_callee_pop/driver.c"),
        ),
    ] {
        std::fs::write(root.join(name), contents).unwrap();
    }
    let image = root.join("sample.so");
    checked(
        Command::new("clang")
            .current_dir(&root)
            .args([
                "--target=i386-linux-gnu",
                "-shared",
                "-nostdlib",
                "-fPIC",
                "-Wl,-Bsymbolic-functions",
                "-DREFERENCE",
                "sample.s",
                "support.c",
                "-o",
            ])
            .arg(&image),
    );
    let specs = common::repo_root().join("specs");
    for (variant, observe) in [
        (
            "named",
            "prototype observe void __cdecl observe(float a,float b)",
        ),
        ("unnamed", "prototype observe void observe(float a,float b)"),
    ] {
        let project = root.join(format!("project-{variant}"));
        let (_, error, status) = common::run_kuna(&[
            "decompile-project",
            image.to_str().unwrap(),
            "--functions",
            "sample",
            "--sleighpath",
            specs.to_str().unwrap(),
            "--target",
            "x86:LE:32:default:windows",
            "--option",
            "stackviews",
            "on",
            "--assert",
            "typedef struct pair {float real;float imaginary;} pair;",
            "--assert",
            "prototype sample float __cdecl sample(float a,float b)",
            "--assert",
            observe,
            "--assert",
            "prototype fill void __stdcall fill(pair *output)",
            "--assert-strict",
            "-o",
            project.to_str().unwrap(),
        ]);
        assert_eq!(status, 0, "{variant}: {error}");
        for optimization in ["-O0", "-O2"] {
            let mut executables = Vec::new();
            for regenerated in [false, true] {
                let executable = root.join(format!(
                    "{}-{variant}{optimization}",
                    if regenerated { "rebuilt" } else { "original" }
                ));
                let mut command = Command::new("clang");
                command.current_dir(&root).args([
                    "--target=i386-linux-gnu",
                    "-nostdlib",
                    "-static",
                    "-fno-pic",
                    "-fno-pie",
                    "-fno-stack-protector",
                    optimization,
                    "start.s",
                    "support.c",
                    "driver.c",
                ]);
                if regenerated {
                    command
                        .args(["-DREGENERATED", "-include", "callbacks.h"])
                        .arg(project.join("sample.so.c"));
                } else {
                    command.arg("-DREFERENCE").arg("sample.s");
                }
                checked(command.arg("-o").arg(&executable));
                executables.push(executable);
            }
            let original = match Command::new(&executables[0]).output() {
                Ok(output) => output,
                Err(error) if error.raw_os_error() == Some(8) => {
                    eprintln!("Kernel cannot execute i386 ELF; generated C compiled successfully");
                    std::fs::remove_dir_all(root).unwrap();
                    return;
                }
                Err(error) => panic!("execute reference: {error}"),
            };
            assert!(
                original.status.success(),
                "reference fixture failed: {:?}",
                original.status
            );
            checked(&mut Command::new(&executables[1]));
        }
    }
    std::fs::remove_dir_all(root).unwrap();
}
