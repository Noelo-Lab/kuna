#![cfg(all(target_arch = "x86_64", target_os = "linux"))]

use crate::common;
use common::process;
use kuna_analysis::loadimage_object::ObjectLoadImage;
use std::path::{Path, PathBuf};
use std::process::Command;

fn fixture(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/relocated_table_c")
        .join(name)
}

#[test]
fn relocated_tables_preserve_values_after_independent_relayout() {
    let compilers: Vec<_> = ["gcc", "clang"]
        .into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    if compilers.is_empty() {
        eprintln!("skipping native table round trip: no GCC or Clang");
        return;
    }
    let dir = common::scratch_file("relocated-table-c", "dir");
    std::fs::create_dir_all(&dir).unwrap();
    let object = dir.join("access.o");
    process::required_output(
        Command::new(compilers[0])
            .args(["-fno-pic", "-fno-pie", "-c"])
            .arg(fixture("access.S"))
            .arg("-o")
            .arg(&object),
    );
    let native = dir.join("native");
    process::required_output(
        Command::new(compilers[0])
            .args(["-O2", "-no-pie"])
            .arg(&object)
            .arg(fixture("driver.c"))
            .arg("-o")
            .arg(&native),
    );
    process::required_output(&mut Command::new(&native));

    let image =
        ObjectLoadImage::from_bytes(object.to_str().unwrap(), &std::fs::read(&object).unwrap())
            .unwrap();
    let half_address = image
        .data_symbols()
        .into_iter()
        .find(|(_, name, _)| name == "sample_halves")
        .expect("the assembly defines the halfword control")
        .0;
    let prototypes = [
        ("lookup17", "int lookup17(long slot)"),
        ("lookup53", "int lookup53(long slot)"),
        ("lookup211", "int lookup211(long slot)"),
        ("lookup32", "int lookup32(unsigned long slot)"),
        ("lookup64", "int lookup64(unsigned long slot)"),
        ("lookup63", "int lookup63(unsigned long slot)"),
        ("lookup_half", "int lookup_half(long slot)"),
        ("store53", "void store53(long slot, signed char value)"),
    ];
    let names = prototypes.map(|(name, _)| name).join(",");
    let runner =
        std::env::var_os("KUNA_TEST_RUNNER").unwrap_or_else(|| env!("CARGO_BIN_EXE_kuna").into());
    let mut command = Command::new(runner);
    command
        .env(
            "KUNA_BINARY",
            std::env::var_os("KUNA_BINARY").unwrap_or_else(|| env!("CARGO_BIN_EXE_kuna").into()),
        )
        .env("KUNA_SPECS", common::repo_root().join("specs"))
        .args(["decompile-all"])
        .arg(&object)
        .args([
            "--functions",
            &names,
            "--mode",
            "aggressive",
            "--language",
            "c",
            "--jobs",
            "1",
            "--option",
            "globalarrayanchor",
            "on",
            "--assert-strict",
            "--json",
        ])
        .args([
            "--assert",
            &format!("data {half_address:#x} short sample_halves[11]"),
        ]);
    for (name, declaration) in prototypes {
        command.args(["--assert", &format!("prototype {name} {declaration}")]);
    }
    let result = process::required_output(&mut command);
    let result: serde_json::Value = serde_json::from_slice(&result.stdout).unwrap();
    let emitted = dir.join("emitted.c");
    let mut code = String::from(
        "extern unsigned char sample_prefix[256];\nextern unsigned char sample_table[23];\nextern short sample_halves[11];\n",
    );
    for function in result["functions"].as_array().expect("function results") {
        assert!(function["error"].is_null(), "{function}");
        code.push_str(function["code"].as_str().expect("emitted C"));
        code.push('\n');
    }
    std::fs::write(&emitted, code).unwrap();
    for cc in compilers {
        for level in ["-O0", "-O2"] {
            let executable = dir.join(format!("relayout-{cc}-{level}"));
            process::required_output(
                Command::new(cc)
                    .args([
                        level,
                        "-fstrict-aliasing",
                        "-fsanitize=undefined",
                        "-fno-sanitize-recover=all",
                        "-Werror",
                        "-no-pie",
                    ])
                    .arg(&emitted)
                    .arg(fixture("driver.c"))
                    .arg(fixture("layout.c"))
                    .arg("-o")
                    .arg(&executable),
            );
            process::required_output(&mut Command::new(&executable));
        }
    }
    std::fs::remove_dir_all(dir).unwrap();
}
