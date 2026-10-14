//! A call completes before a separate snapshot reads its updated out-parameter.
use crate::common;

use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

fn required(command: &mut Command) {
    common::process::required_output(command);
}

fn decompile(image: &Path, stackalias: bool) -> serde_json::Value {
    let mut command = Command::new(env!("CARGO_BIN_EXE_kuna"));
    command
        .args([
            "decompile-all",
            image.to_str().unwrap(),
            "--json",
            "--assert-strict",
            "--functions",
            "outparam_branch_snapshot,outparam_expression_safe",
            "--option",
            "stackalias",
            if stackalias { "on" } else { "off" },
            "--sleighpath",
        ])
        .arg(common::repo_root().join("specs"));
    for assertion in [
        "prototype fill_line int8 fill_line(uint1 **line)",
        "prototype read_first uint8 read_first(const uint1 *line)",
        "prototype identity_pointer uint1 *identity_pointer(uint1 *buffer)",
        "prototype outparam_branch_snapshot uint8 outparam_branch_snapshot(int4 from_file, const uint1 *alternate)",
        "prototype outparam_expression_safe uint8 outparam_expression_safe(uint1 *buffer)",
    ] {
        command.args(["--assert", assertion]);
    }
    let output = common::process::output_with_timeout(
        &mut command,
        Duration::from_secs(30),
        Duration::from_millis(50),
    )
    .expect("bounded out-parameter snapshot decompilation");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let document: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert!(
        document["assertions"]
            .as_array()
            .unwrap()
            .iter()
            .all(|assertion| assertion["status"] == "applied"),
        "{document}"
    );
    document
}

fn code(document: &serde_json::Value, name: &str) -> String {
    document["functions"]
        .as_array()
        .unwrap()
        .iter()
        .find(|function| function["name"] == name)
        .unwrap()["code"]
        .as_str()
        .unwrap()
        .to_owned()
}

#[test]
fn updated_out_parameter_snapshots_round_trip_without_disabling_safe_folds() {
    if std::env::consts::ARCH != "x86_64" || std::env::consts::OS != "linux" {
        return;
    }
    let compilers: Vec<_> = ["gcc", "clang"]
        .into_iter()
        .filter(|compiler| {
            common::process::optional_output(Command::new(compiler).arg("--version")).is_some()
        })
        .collect();
    let Some(compiler) = compilers.first() else {
        eprintln!("skipping out-parameter snapshot: no native C compiler");
        return;
    };
    let image = common::scratch_file("call-outparam-snapshot", "elf");
    required(
        Command::new(compiler)
            .args(["-nostdlib", "-no-pie", "-Wl,-e,outparam_branch_snapshot"])
            .arg(fixture("call_outparam_snapshot.S"))
            .arg("-o")
            .arg(&image),
    );
    let native = common::scratch_file("call-outparam-native", "bin");
    required(
        Command::new(compiler)
            .args(["-std=c11", "-O2", "-DKUNA_NATIVE"])
            .arg(fixture("call_outparam_snapshot_driver.c"))
            .arg(fixture("call_outparam_snapshot.S"))
            .arg("-o")
            .arg(&native),
    );
    required(&mut Command::new(native));

    for stackalias in [false, true] {
        let document = decompile(&image, stackalias);
        let snapshot = code(&document, "outparam_branch_snapshot");
        let safe = code(&document, "outparam_expression_safe");
        assert!(
            snapshot.contains("= fill_line("),
            "the call must finish before the snapshot: {snapshot}"
        );
        assert_eq!(safe.matches("identity_pointer(").count(), 1, "{safe}");
        assert!(
            !safe.contains("= identity_pointer("),
            "the ordered single expression should still fold: {safe}"
        );
        let emitted = common::scratch_file("call-outparam-emitted", "c");
        let driver = std::fs::read_to_string(fixture("call_outparam_snapshot_driver.c")).unwrap();
        std::fs::write(
            &emitted,
            driver.replace("/* KUNA_EMITTED_CODE */", &(snapshot + "\n" + &safe)),
        )
        .unwrap();
        for compiler in &compilers {
            for optimization in ["-O0", "-O2"] {
                let binary = common::scratch_file("call-outparam-roundtrip", "bin");
                required(
                    Command::new(compiler)
                        .args([
                            "-std=c11",
                            optimization,
                            "-fstrict-aliasing",
                            "-Werror=int-conversion",
                            "-Werror=incompatible-pointer-types",
                        ])
                        .arg(&emitted)
                        .arg("-o")
                        .arg(&binary),
                );
                required(&mut Command::new(binary));
            }
        }
    }
}
