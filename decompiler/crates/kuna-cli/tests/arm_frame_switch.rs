//! A32 switch case blocks and a shrink-wrapped `push` stay inside their
//! functions, and `--jobs` workers replay the parent's frame recovery.
use crate::common;

use serde_json::Value;
use std::path::{Path, PathBuf};
use std::process::Command;

const DISPATCHER: u64 = 0x0800_0400;
const CASES: [u64; 5] = [0x0800_0424, 0x0800_0434, 0x0800_0444, 0x0800_0454, 0x0800_0464];
const WRAPPED: u64 = 0x0800_0480;

fn fixture(tag: &str) -> PathBuf {
    let path = common::scratch_file(tag, "elf");
    let generator = common::repo_root()
        .join("decompiler/crates/kuna-analysis/tests/fixtures/arm_frame_switch.py");
    assert!(Command::new("python3").arg(generator).arg(&path)
        .status().expect("generate authored fixture").success());
    path
}

fn decompile_all(path: &Path, extra: &[&str]) -> Vec<u8> {
    let out = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args(["decompile-all", path.to_str().unwrap(), "--json"])
        .args(extra)
        .output()
        .expect("run source-built kuna");
    assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
    out.stdout
}

fn entries(doc: &[u8]) -> Vec<u64> {
    let doc: Value = serde_json::from_slice(doc).expect("decompiler JSON");
    doc["functions"].as_array().unwrap().iter().map(|f| f["address"].as_u64().unwrap()).collect()
}

#[test]
fn jump_table_cases_and_a_late_push_are_not_function_entries() {
    let path = fixture("arm-frame-switch");
    let doc = decompile_all(&path, &["--jobs", "1"]);
    let found = entries(&doc);
    assert!(found.contains(&DISPATCHER) && found.contains(&WRAPPED), "{found:x?}");
    for at in CASES.iter().chain([&(WRAPPED + 4)]) {
        assert!(!found.contains(at), "{at:#x} split its function: {found:x?}");
    }
    let doc: Value = serde_json::from_slice(&doc).unwrap();
    let code = doc["functions"].as_array().unwrap().iter()
        .find(|f| f["address"] == DISPATCHER).unwrap()["code"].as_str().unwrap().to_string();
    assert!(code.contains("switch") && code.contains("sub_8000480(0)"), "{code}");

    let prior = entries(&decompile_all(&path, &["--jobs", "1", "--option", "armframes", "off"]));
    assert!(!prior.contains(&DISPATCHER) && prior.contains(&CASES[0]), "{prior:x?}");
    std::fs::remove_file(path).unwrap();
}

#[test]
fn jobs_workers_replay_the_parents_frame_recovery() {
    let path = fixture("arm-frame-switch-jobs");
    let serial = decompile_all(&path, &["--jobs", "1", "--option", "protoorder", "off"]);
    assert!(entries(&serial).contains(&DISPATCHER));
    for full in [false, true] {
        let mut args = vec!["--jobs", "2", "--jobs-chunk", "4", "--option", "protoorder", "off"];
        if full {
            args.push("--jobs-full-load");
        }
        assert!(decompile_all(&path, &args) == serial, "--jobs 2 (full load {full}) differs from --jobs 1");
    }
    std::fs::remove_file(path).unwrap();
}
