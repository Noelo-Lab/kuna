//! `funcchunks`: the inventory size of `kuna functions --json` is the flow body
//! (main chunk plus tails), not the `[entry, next)` clip.

use std::process::Command;

fn sizes(extra: &[&str]) -> serde_json::Value {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap();
    let bin = root.join("decompiler/crates/kuna-analysis/tests/fixtures/funcchunks_x86_64");
    let out = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args(["functions", bin.to_str().unwrap(), "--json", "--option", "listing", "on",
               "--sleighpath", root.join("specs").to_str().unwrap()])
        .args(extra)
        .output()
        .expect("run source-built kuna");
    assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
    serde_json::from_slice(&out.stdout).expect("functions JSON")
}

fn size_of(doc: &serde_json::Value, addr: u64) -> u64 {
    doc["functions"]
        .as_array()
        .unwrap()
        .iter()
        .find(|f| f["address"] == addr)
        .unwrap_or_else(|| panic!("{addr:#x} missing: {doc}"))["size"]
        .as_u64()
        .unwrap()
}

/// `f`@0x401019 and `g`@0x401024 jump into the shared tail at 0x401036, which
/// lies after `h`@0x401030: the clip hands the tail to `h`, the chunk body gives
/// it to both jumpers.
#[test]
fn the_inventory_size_is_the_chunk_body_with_the_option_on() {
    let clip = sizes(&[]);
    assert_eq!(size_of(&clip, 0x401030), 12, "{clip}");

    let body = sizes(&["--option", "funcchunks", "on"]);
    assert_eq!(size_of(&body, 0x401030), 6, "{body}");
    assert_eq!(size_of(&body, 0x401019), 11 + 6, "{body}");
    assert_eq!(size_of(&body, 0x401024), 12 + 6, "{body}");
}
