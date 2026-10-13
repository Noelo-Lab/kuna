//! A frame `armframes` recovers keeps the routine its tail branch reaches.
use serde_json::Value;
use std::path::PathBuf;
use std::process::Command;

fn entries(path: &PathBuf, extra: &[&str]) -> Vec<u64> {
    let out = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args(["functions", path.to_str().unwrap(), "--json", "--jobs", "1", "--mode", "reliable"])
        .args(extra)
        .output()
        .expect("run source-built kuna");
    assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
    let doc: Value = serde_json::from_slice(&out.stdout).expect("functions JSON");
    doc["functions"]
        .as_array()
        .expect("function list")
        .iter()
        .map(|f| f["address"].as_u64().expect("function address"))
        .collect()
}

#[test]
fn frame_recovery_keeps_the_tail_branch_target_the_default_inventory_finds() {
    let path = PathBuf::from(env!("CARGO_TARGET_TMPDIR"))
        .join(format!("arm-frame-retain-{}.elf", std::process::id()));
    let generator = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../kuna-analysis/tests/fixtures/arm_frame_retain.py");
    assert!(Command::new("python3")
        .arg(generator)
        .arg(&path)
        .status()
        .expect("run authored fixture generator")
        .success());
    let default = entries(&path, &[]);
    assert!(default.contains(&0x1600) && !default.contains(&0x1500), "{default:x?}");
    let frames = entries(&path, &["--option", "armframes", "on"]);
    assert!(frames.contains(&0x1500) && frames.contains(&0x1600), "{frames:x?}");
    std::fs::remove_file(path).unwrap();
}
