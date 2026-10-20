//! A declared data range supersedes strings inferred from float bytes inside it,
//! including one that starts inside the range and runs past its end.
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
fn an_asserted_float_table_displaces_an_inferred_interior_string() {
    if !Command::new("clang")
        .arg("--version")
        .output()
        .is_ok_and(|o| o.status.success())
    {
        eprintln!("Clang unavailable; skipping ELF fixture");
        return;
    }
    let contained = include_str!("fixtures/declared_data_range/sample.s");
    let straddling = contained.replace(
        " .long 0x42640000,0x40414243,0x42700000\n",
        " .long 0x42640000,0x40414243,0x44434241\n .byte 0\n",
    );
    assert_ne!(contained, straddling);
    check_table(contained);
    check_table(&straddling);
}

fn check_table(assembly: &str) {
    let root = common::scratch_file("declared-data-range", "dir");
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(root.join("sample.s"), assembly).unwrap();
    std::fs::write(
        root.join("driver.c"),
        include_str!("fixtures/declared_data_range/driver.c"),
    )
    .unwrap();
    let image = root.join("sample.so");
    checked(
        Command::new("clang")
            .current_dir(&root)
            .args([
                "-shared",
                "-nostdlib",
                "-Wl,--section-start=.text=0x1000",
                "-Wl,--section-start=.rodata=0x2000",
                "sample.s",
                "-o",
            ])
            .arg(&image),
    );
    let project = root.join("project");
    let specs = common::repo_root().join("specs");
    let (_, stderr, status) = common::run_kuna(&[
        "decompile-project",
        image.to_str().unwrap(),
        "--functions",
        "sample",
        "--sleighpath",
        specs.to_str().unwrap(),
        "--option",
        "symbolnamechars",
        "ident",
        "--assert",
        "prototype sample float sample(int choice)",
        "--assert",
        "data 0x2000 float values[3]",
        "--assert-strict",
        "-o",
        project.to_str().unwrap(),
    ]);
    assert_eq!(status, 0, "{stderr}");
    let code = std::fs::read_to_string(project.join("sample.so.c")).unwrap();
    let header = std::fs::read_to_string(project.join("sample.so.h")).unwrap();
    assert!(
        code.contains("values[0]") && code.contains("values[1]"),
        "{code}"
    );
    assert!(header.contains("extern float values[3];"), "{header}");
    assert!(!code.contains("s_2002"), "{code}");
    for compiler in ["gcc", "clang"] {
        if !Command::new(compiler)
            .arg("--version")
            .output()
            .is_ok_and(|o| o.status.success())
        {
            continue;
        }
        for optimization in ["-O0", "-O2"] {
            for regenerated in [false, true] {
                let exe = root.join(format!("{compiler}-{optimization}-{regenerated}"));
                let mut compile = Command::new(compiler);
                compile
                    .current_dir(&root)
                    .args(["-std=c11", optimization, "driver.c"]);
                if regenerated {
                    compile
                        .arg("-DREGENERATED")
                        .arg(project.join("sample.so.c"));
                } else {
                    compile.arg(&image);
                }
                checked(compile.arg("-o").arg(&exe));
                checked(&mut Command::new(&exe));
            }
        }
    }
    std::fs::remove_dir_all(root).unwrap();
}
