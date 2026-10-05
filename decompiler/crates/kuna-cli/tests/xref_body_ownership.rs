//! Caller ownership follows reachable blocks across independently established entries.

use serde_json::Value;
use std::path::PathBuf;
use std::process::Command;

#[test]
fn discontiguous_callers_keep_their_calls_in_both_query_directions() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let path = PathBuf::from(env!("CARGO_TARGET_TMPDIR"))
        .join(format!("xref-bodies-{}.elf", std::process::id()));
    for (endian, target) in [("little", "ARM:LE:32:v8"), ("big", "ARM:BE:32:v8")] {
        assert!(Command::new("python3")
            .arg(root.join("../kuna-analysis/tests/fixtures/arm_xref_bodies.py"))
            .arg(&path)
            .arg(endian)
            .status()
            .unwrap()
            .success());
        for (direction, entry) in [("--from", "0x1500"), ("--to", "0x1100")] {
            let output = Command::new(env!("CARGO_BIN_EXE_kuna"))
                .args([
                    "xrefs",
                    path.to_str().unwrap(),
                    direction,
                    entry,
                    "--json",
                    "--kind",
                    "call",
                    "--target",
                    target,
                    "--isa",
                    "arm",
                    "--option",
                    "aif",
                    "off",
                    "--option",
                    "funcstart_patterns",
                    "on",
                ])
                .output()
                .unwrap();
            assert!(
                output.status.success(),
                "{}",
                String::from_utf8_lossy(&output.stderr)
            );
            let doc: Value = serde_json::from_slice(&output.stdout).unwrap();
            let calls = doc["xrefs"].as_array().unwrap();
            assert_eq!(calls.len(), 1, "{endian} {direction}: {doc}");
            assert_eq!(calls[0]["from_address"], 0x1750);
            assert_eq!(calls[0]["to_address"], 0x1100);
            assert_eq!(calls[0]["from_function"]["address"], 0x1500);
        }
    }
    std::fs::remove_file(path).unwrap();
}
