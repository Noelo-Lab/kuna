//! A return block cloned per predecessor must not print a second store to a global (#1355).
//!
//! The fixture stores each global once on every path. The emitted C is recompiled with the
//! globals counting their own evaluations and run on every path.
#![cfg(all(target_arch = "x86_64", target_os = "linux"))]

use crate::common;

use common::process;
use std::path::Path;
use std::process::Command;

const FUNCTIONS: [&str; 3] = ["audit_select", "audit_select_gcc", "audit_retry"];

const PRELUDE: &str = r#"
#include <stdio.h>
typedef void code;
typedef int int4;
typedef unsigned int uint4;
static int recorded_writes, count_writes;
static void *recorded;
static int count;
#define audit_recorded (*(++recorded_writes, &recorded))
#define audit_count (*(++count_writes, &count))
int audit_choice;
int audit_fast(int x) { return x + 7; }
int audit_safe(int x) { return x + 11; }
int audit_decoy(int x) { return x + 19; }
void audit_tick(void) { }
"#;

const DRIVER: &str = r#"
static int select_once(const char *name, void *(*select)(void)) {
    for (int choice = 0; choice < 2; choice++) {
        audit_choice = choice;
        recorded_writes = 0;
        recorded = 0;
        void *result = select();
        void *expected = choice ? (void *)audit_fast : (void *)audit_safe;
        if (recorded_writes != 1 || recorded != (void *)audit_decoy || result != expected) {
            fprintf(stderr, "%s choice %d: %d writes\n", name, choice, recorded_writes);
            return 1;
        }
    }
    return 0;
}

int main(void) {
    int bad = select_once("audit_select", (void *(*)(void))audit_select)
        | select_once("audit_select_gcc", (void *(*)(void))audit_select_gcc);
    for (int a = 0; a < 2; a++) {
        for (int b = 0; b < 2; b++) {
            count_writes = 0;
            count = 0;
            int result = (int)audit_retry(a, b);
            if (result != (a ? (b ? 1 : 2) : 3) || count_writes != a || count != a) {
                fprintf(stderr, "audit_retry(%d,%d): %d writes\n", a, b, count_writes);
                bad = 1;
            }
        }
    }
    return bad;
}
"#;

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
fn split_return_writes_each_global_once() {
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
        let code = decompile(&image, extra);
        for (function, global) in FUNCTIONS.iter().zip(["audit_recorded", "audit_recorded", "audit_count"]) {
            let text = &code[FUNCTIONS.iter().position(|f| f == function).unwrap()];
            assert_eq!(text.matches(&format!("{global} = ")).count(), 1, "{label}:\n{text}");
        }
        let printed = code.join("\n");
        let source = dir.join(format!("printed-{label}.c"));
        std::fs::write(&source, format!("{PRELUDE}\n{printed}\n{DRIVER}")).unwrap();
        let executable = source.with_extension("");
        process::required_output(Command::new(cc).args(["-O0", "-w"]).arg(&source).arg("-o").arg(&executable));
        let run = Command::new(&executable).output().unwrap();
        assert!(
            run.status.success(),
            "{label}: printed C writes a global more often than the binary\n{}\n{printed}",
            String::from_utf8_lossy(&run.stderr)
        );
    }
    std::fs::remove_dir_all(dir).unwrap();
}
