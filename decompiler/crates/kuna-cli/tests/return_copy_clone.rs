//! A return block cloned per predecessor must not print a second store to a global (#1355).
//!
//! The fixture stores `audit_recorded` once and picks the result with a CMOV. The emitted C is
//! recompiled with `audit_recorded` counting its own evaluations and run for both choices.
#![cfg(all(target_arch = "x86_64", target_os = "linux"))]

use crate::common;

use common::process;
use std::path::Path;
use std::process::Command;

const FUNCTIONS: [&str; 2] = ["audit_select", "audit_select_gcc"];

const PRELUDE: &str = r#"
#include <stdio.h>
typedef void code;
static int writes;
static void *recorded;
#define audit_recorded (*(++writes, &recorded))
int audit_choice;
int audit_fast(int x) { return x + 7; }
int audit_safe(int x) { return x + 11; }
int audit_decoy(int x) { return x + 19; }
"#;

fn driver(function: &str) -> String {
    format!(
        r#"
int main(void) {{
    for (int choice = 0; choice < 2; choice++) {{
        audit_choice = choice;
        writes = 0;
        recorded = 0;
        void *result = (void *){function}();
        void *expected = choice ? (void *)audit_fast : (void *)audit_safe;
        if (writes != 1 || recorded != (void *)audit_decoy || result != expected) {{
            fprintf(stderr, "choice %d: %d writes\n", choice, writes);
            return 1;
        }}
    }}
    return 0;
}}
"#
    )
}

fn compiler() -> Option<&'static str> {
    ["gcc", "clang"]
        .into_iter()
        .find(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
}

fn decompile(image: &Path, extra: &[&str]) -> Vec<String> {
    let specs = common::repo_root().join("specs");
    let functions = FUNCTIONS.join(",");
    let mut args = vec![
        "decompile-all",
        image.to_str().unwrap(),
        "--functions",
        &functions,
        "--json",
        "--sleighpath",
        specs.to_str().unwrap(),
    ];
    args.extend_from_slice(extra);
    let (stdout, stderr, rc) = common::run_kuna(&args);
    assert_eq!(rc, 0, "{stderr}\n{stdout}");
    let doc: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    FUNCTIONS
        .iter()
        .map(|name| {
            let record = doc["functions"]
                .as_array()
                .unwrap()
                .iter()
                .find(|f| f["name"] == *name)
                .unwrap_or_else(|| panic!("{name} missing: {doc}"));
            assert!(record["error"].is_null(), "{record}");
            record["code"].as_str().unwrap().to_owned()
        })
        .collect()
}

#[test]
fn split_return_writes_the_global_once() {
    let Some(cc) = compiler() else {
        eprintln!("skipping return copy clone round trip: no GCC or Clang");
        return;
    };
    let dir = common::scratch_file("return-copy-clone", "dir");
    std::fs::create_dir_all(&dir).unwrap();
    let image = dir.join("return_copy_clone.so");
    process::required_output(
        Command::new(cc)
            .args(["-shared", "-nostdlib", "-Wl,--build-id=none"])
            .arg(Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/return_copy_clone.S"))
            .arg("-o")
            .arg(&image),
    );
    let runs: [(&str, &[&str]); 4] = [
        ("auto", &[]),
        ("reliable", &["--mode", "reliable"]),
        ("fast", &["--mode", "fast"]),
        ("upstream", &["--mode", "reliable", "--option", "returndup", "off"]),
    ];
    for (label, extra) in runs {
        for (function, code) in FUNCTIONS.iter().zip(decompile(&image, extra)) {
            assert!(code.contains("return audit_fast;") && code.contains("return audit_safe;"), "{label}:\n{code}");
            assert_eq!(code.matches("audit_recorded = ").count(), 1, "{label}:\n{code}");
            let source = dir.join(format!("{function}-{label}.c"));
            std::fs::write(&source, format!("{PRELUDE}\n{code}\n{}", driver(function))).unwrap();
            let executable = source.with_extension("");
            process::required_output(Command::new(cc).args(["-O0", "-w"]).arg(&source).arg("-o").arg(&executable));
            let run = Command::new(&executable).output().unwrap();
            assert!(
                run.status.success(),
                "{label} {function}: printed C writes audit_recorded differently from the binary\n{}\n{code}",
                String::from_utf8_lossy(&run.stderr)
            );
        }
    }
    std::fs::remove_dir_all(dir).unwrap();
}
