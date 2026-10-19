//! Declared inputs remain real values when their spill slots are reused.
#![cfg(all(target_os = "linux", target_arch = "x86_64"))]
use crate::common;
use std::process::{Command, Output};

fn checked(command: &mut Command) -> Output {
    let output = command.output().expect("start fixture command");
    assert!(
        output.status.success(),
        "{command:?}\n{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    output
}

#[test]
fn declared_preserved_input_does_not_hide_a_reused_stack_local() {
    if !Command::new("clang")
        .arg("--version")
        .output()
        .is_ok_and(|output| output.status.success())
    {
        eprintln!("Clang unavailable; skipping cross-target fixture");
        return;
    }
    let root = common::scratch_file("parameter-spill-reuse", "dir");
    std::fs::create_dir_all(&root).unwrap();
    for (name, contents) in [
        (
            "callbacks.h",
            include_str!("fixtures/parameter_spill/callbacks.h"),
        ),
        (
            "sample.s",
            include_str!("fixtures/parameter_spill/sample.s"),
        ),
        (
            "support.c",
            include_str!("fixtures/parameter_spill/support.c"),
        ),
        ("start.s", include_str!("fixtures/parameter_spill/start.s")),
        (
            "driver.c",
            include_str!("fixtures/parameter_spill/driver.c"),
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
                "sample.s",
                "support.c",
                "-o",
            ])
            .arg(&image),
    );
    let project = root.join("project");
    let specs = common::repo_root().join("specs");
    let (_, error, status) = common::run_kuna(&[
        "decompile-project",
        image.to_str().unwrap(),
        "--functions",
        "_sample",
        "--sleighpath",
        specs.to_str().unwrap(),
        "--option",
        "stackviews",
        "on",
        "--assert",
        "prototype _sample float __cdecl _sample(float value,unsigned int control)",
        "--assert",
        "param _sample::1 EDI unsigned int control",
        "--assert",
        "prototype _logflags void __cdecl _logflags(unsigned int value)",
        "--assert",
        "prototype _read_value float __cdecl _read_value(float *value)",
        "--assert-strict",
        "-o",
        project.to_str().unwrap(),
    ]);
    assert_eq!(status, 0, "{error}");
    for optimization in ["-O0", "-O2"] {
        let mut executables = Vec::new();
        for regenerated in [false, true] {
            let executable = root.join(format!(
                "{}-{optimization}",
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
                command.arg("sample.s");
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
        assert_eq!(original.stdout.len(), 1024 * 16);
        let rebuilt = checked(&mut Command::new(&executables[1]));
        assert_eq!(
            rebuilt.stdout, original.stdout,
            "{optimization}: register observation or reused float differs"
        );
    }
    std::fs::remove_dir_all(root).unwrap();
}
