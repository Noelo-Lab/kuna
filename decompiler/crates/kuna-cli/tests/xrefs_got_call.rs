//! `kuna xrefs` and `kuna decompile-graph` over a call through an import's GOT
//! slot (GH #1376). `adrp x16,_malloc@GOTPAGE; ldr x16,[x16,..]; blr x16`
//! reads the slot and calls through the pointer it loaded, as x86-64's `call
//! *slot(%rip)` does in one instruction and `mov slot(%rip),%rax; call *%rax`
//! in two. A Mach-O slot lives in `__DATA_CONST,__got`, a section `object` names
//! no kind for, and the reference walk used to drop every row into it.
//!
//! Fixtures: `gotcall_*` (see `gotcall.provenance`). Integration tests require
//! the built processor specs under `specs/`.

use crate::common;

/// Every row `kuna xrefs <binary> <direction> <target> --json` answers, as
/// `(kind, referencing function, referencing address, referenced address)`.
fn rows(binary: &str, direction: &str, target: &str) -> Vec<(String, String, u64, u64)> {
    let path = common::fixture(binary);
    let (stdout, stderr, rc) = common::run_kuna(&["xrefs", &path, direction, target, "--json"]);
    assert_eq!(rc, 0, "{stderr}\n{stdout}");
    let doc: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    doc["xrefs"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| {
            (
                r["kind"].as_str().unwrap().to_owned(),
                r["from_function"]["name"].as_str().unwrap_or("-").to_owned(),
                r["from_address"].as_u64().unwrap(),
                r["to_address"].as_u64().unwrap(),
            )
        })
        .collect()
}

fn at(rows: &[(String, String, u64, u64)], kind: &str, from: u64, to: u64) -> bool {
    rows.iter().any(|(k, _, f, t)| k == kind && *f == from && *t == to)
}

/// The `(caller, callee, kind)` edges of the graph document.
fn edges(binary: &str) -> Vec<(u64, u64, String)> {
    let path = common::fixture(binary);
    let (stdout, stderr, rc) = common::run_kuna(&["decompile-graph", &path, "--functions", ""]);
    assert_eq!(rc, 0, "{stderr}\n{stdout}");
    let doc: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    doc["edges"]
        .as_array()
        .unwrap()
        .iter()
        .map(|e| {
            (
                e["callerAddress"].as_u64().unwrap(),
                e["calleeAddress"].as_u64().unwrap(),
                e["kind"].as_str().unwrap().to_owned(),
            )
        })
        .collect()
}

/// The issue's dylib: the `ldr` reads `_malloc`'s slot and the `blr` calls
/// through it, from both directions and under the import's own name.
#[test]
fn an_arm64_macho_got_load_reads_the_slot_and_the_blr_calls_it() {
    let bin = "gotcall_macho_arm64";
    let from = rows(bin, "--from", "_audit_import_call");
    assert!(at(&from, "read", 0x131c, 0x4000), "ldr x16,[x16]: {from:?}");
    assert!(at(&from, "call", 0x1320, 0x4000), "blr x16: {from:?}");
    for target in ["0x4000", "malloc"] {
        let to = rows(bin, "--to", target);
        assert!(at(&to, "read", 0x131c, 0x4000), "--to {target}: {to:?}");
        assert!(at(&to, "call", 0x1320, 0x4000), "--to {target}: {to:?}");
    }
    assert_eq!(edges(bin), [(0x1310, 0x4000, "call".to_owned())], "_audit_import_call -> malloc");
}

#[test]
fn both_x86_64_macho_spellings_call_the_slot() {
    let bin = "gotcall_macho_x86_64";
    let to = rows(bin, "--to", "malloc");
    assert!(at(&to, "call", 0x301, 0x1000), "call *_malloc@GOTPCREL(%rip): {to:?}");
    assert!(at(&to, "read", 0x30a, 0x1000), "mov _malloc@GOTPCREL(%rip),%rax: {to:?}");
    assert!(at(&to, "call", 0x311, 0x1000), "call *%rax: {to:?}");
    let mut graph = edges(bin);
    graph.sort();
    assert_eq!(graph, [(0x300, 0x1000, "call".to_owned()), (0x309, 0x1000, "call".to_owned())]);
}

/// The ELF controls name no import at the slot, but the rows are the same.
#[test]
fn an_elf_got_load_and_call_read_and_call_the_slot() {
    let a64 = rows("gotcall_aarch64", "--to", "0x20348");
    assert!(at(&a64, "read", 0x10294, 0x20348), "{a64:?}");
    assert!(at(&a64, "call", 0x10298, 0x20348), "{a64:?}");
    let x64 = rows("gotcall_x86_64", "--to", "0x2370");
    assert!(at(&x64, "call", 0x12b9, 0x2370), "{x64:?}");
    assert!(at(&x64, "read", 0x12c2, 0x2370), "{x64:?}");
    assert!(at(&x64, "call", 0x12c9, 0x2370), "{x64:?}");
}
