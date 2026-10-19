//! A restored frame base does not make pointer-written locals immutable.
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
fn declared_pointer_writes_survive_a_restored_frame_base() {
    if !Command::new("clang")
        .arg("--version")
        .output()
        .is_ok_and(|output| output.status.success())
    {
        eprintln!("Clang unavailable; skipping i386 fixture");
        return;
    }
    let root = common::scratch_file("declared-pointer-frame-alias", "dir");
    std::fs::create_dir_all(&root).unwrap();
    for (name, contents) in [
        (
            "sample.s",
            include_str!("fixtures/declared_pointer_frame_alias/sample.s"),
        ),
        (
            "start.s",
            include_str!("fixtures/declared_pointer_frame_alias/start.s"),
        ),
        (
            "support.c",
            include_str!("fixtures/declared_pointer_frame_alias/support.c"),
        ),
        (
            "callbacks.h",
            include_str!("fixtures/declared_pointer_frame_alias/callbacks.h"),
        ),
        (
            "driver.c",
            include_str!("fixtures/declared_pointer_frame_alias/driver.c"),
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
        "sample",
        "--sleighpath",
        specs.to_str().unwrap(),
        "--option",
        "stackviews",
        "on",
        "--assert",
        "typedef struct pair {float real;float imaginary;} pair;",
        "--assert",
        "prototype sample float __cdecl sample(float a,float b)",
        "--assert",
        "prototype helper void __cdecl helper(void)",
        "--assert",
        "prototype fill void __cdecl fill(pair *output)",
        "--assert-strict",
        "-o",
        project.to_str().unwrap(),
    ]);
    assert_eq!(status, 0, "{error}");
    for compiler in ["gcc", "clang"] {
        if !Command::new(compiler)
            .arg("--version")
            .output()
            .is_ok_and(|output| output.status.success())
        {
            continue;
        }
        for optimization in ["-O0", "-O2"] {
            for regenerated in [false, true] {
                let executable = root.join(format!("{compiler}-{optimization}-{regenerated}"));
                let mut command = Command::new(compiler);
                command.current_dir(&root).args([
                    "-m32",
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
                match Command::new(&executable).output() {
                    Ok(output) => assert!(
                        output.status.success(),
                        "{compiler} {optimization} regenerated={regenerated}: {:?}",
                        output.status
                    ),
                    Err(error) if error.raw_os_error() == Some(8) => {
                        eprintln!("Kernel cannot execute i386 ELF; fixture compiled successfully");
                        std::fs::remove_dir_all(root).unwrap();
                        return;
                    }
                    Err(error) => panic!("execute fixture: {error}"),
                }
            }
        }
    }
    std::fs::remove_dir_all(root).unwrap();
}
