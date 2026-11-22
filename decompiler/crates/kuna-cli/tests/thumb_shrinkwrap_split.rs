//! A Thumb `push` a pattern scan finds is not a function when the routine before
//! it falls into it: GCC shrink-wraps, returning before it saves LR.
use serde_json::Value;
use std::path::PathBuf;
use std::process::Command;

#[test]
fn shrink_wrapped_push_stays_inside_its_routine() {
    let path = PathBuf::from(env!("CARGO_TARGET_TMPDIR"))
        .join(format!("thumb-shrinkwrap-split-{}.elf", std::process::id()));
    let generator = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../kuna-analysis/tests/fixtures/thumb_shrinkwrap_split.py");
    assert!(Command::new("python3")
        .arg(generator)
        .arg(&path)
        .status()
        .expect("run authored fixture generator")
        .success());
    let out = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args(["functions", path.to_str().unwrap(), "--json", "--jobs", "1", "--mode", "reliable"])
        .args(["--isa", "thumb"])
        .output()
        .expect("run source-built kuna");
    assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
    let doc: Value = serde_json::from_slice(&out.stdout).expect("functions JSON");
    let entries: Vec<u64> = doc["functions"]
        .as_array()
        .expect("function list")
        .iter()
        .map(|f| f["address"].as_u64().expect("function address"))
        .collect();
    assert!(entries.contains(&0x1100) && entries.contains(&0x1200), "{entries:x?}");
    assert!(!entries.contains(&0x1104), "{entries:x?}");
    std::fs::remove_file(path).unwrap();
}
