//! Literal-pool prefixes must preserve a disconnected ARM caller's input.
use crate::common;

use serde_json::Value;
use std::path::Path;
use std::process::Command;

fn decompile(path: &Path, frames: &str, pools: &str) -> Value {
    let out = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args(["decompile-all", path.to_str().unwrap(), "--json", "--jobs", "1",
               "--mode", "reliable", "--option", "armframes", frames,
               "--option", "aifpoolslide", pools])
        .output()
        .expect("run source-built kuna");
    assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
    serde_json::from_slice(&out.stdout).expect("decompiler JSON")
}

#[test]
fn loaded_literal_before_frame_does_not_replace_the_real_entry_or_its_input() {
    let path = common::scratch_file("arm-frame-pool", "elf");
    let generator = common::repo_root()
        .join("decompiler/crates/kuna-analysis/tests/fixtures/arm_frame_pool.py");
    assert!(Command::new("python3").arg(generator).arg(&path)
        .status().expect("generate authored fixture").success());

    for frames in ["off", "inventory", "on"] {
        let doc = decompile(&path, frames, "on");
        let rows = doc["functions"].as_array().unwrap();
        let caller = rows.iter().find(|f| f["address"] == 0x08000400u64)
            .unwrap_or_else(|| panic!("real caller absent with armframes {frames}: {doc}"));
        let code = caller["code"].as_str().unwrap();
        assert!(code.contains("return a0;"), "{code}");
        assert!(!rows.iter().any(|f| f["address"] == 0x080003fcu64), "literal became code: {doc}");
    }

    let uncorrected = decompile(&path, "on", "off");
    let rows = uncorrected["functions"].as_array().unwrap();
    let phantom = rows.iter().find(|f| f["address"] == 0x080003fcu64)
        .expect("pool check disabled must reproduce the literal prefix");
    assert!(phantom["code"].as_str().unwrap().contains("return 0;"), "{phantom}");
    assert!(!rows.iter().any(|f| f["address"] == 0x08000400u64));
    std::fs::remove_file(path).unwrap();
}
