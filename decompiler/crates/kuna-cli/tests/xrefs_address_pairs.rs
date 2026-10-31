//! `kuna xrefs` over an address a fixed-width ISA forms in two instructions:
//! AArch64 `adrp` + `add`/`ldr`/`str` (ELF and Mach-O), RISC-V `lui`/`auipc` +
//! `addi`/`lw`/`sw`/`ld` (RV32 and RV64). Forming an address is `data`, reading
//! through it `read`, writing through it `write`; a pair that a join, or a call
//! clobbering the register, separates forms nothing.
//!
//! Fixtures: `xrefpairs_{aarch64,rv64,rv32,macho_arm64}` (see their
//! `.provenance`). Integration tests require the built processor specs under
//! `specs/`.

use crate::common;

/// Every row `kuna xrefs <binary> <direction> <target> --json` answers, as
/// `(kind, referencing function, referenced address)`.
fn rows(binary: &str, direction: &str, target: &str) -> Vec<(String, String, u64)> {
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
                r["to_address"].as_u64().unwrap(),
            )
        })
        .collect()
}

fn has(rows: &[(String, String, u64)], kind: &str, from: &str) -> bool {
    rows.iter().any(|(k, f, _)| k == kind && f == from)
}

#[test]
fn an_aarch64_adrp_pair_is_a_reference_from_the_instruction_that_completes_it() {
    let bin = "xrefpairs_aarch64";
    let label = rows(bin, "--to", "audit_label");
    assert!(has(&label, "data", "audit_direct_label"), "adrp + add returning the label: {label:?}");
    assert!(has(&label, "data", "audit_pass_label"), "adrp + add passed to a call: {label:?}");
    let byte = rows(bin, "--to", "0x20103b");
    assert!(has(&byte, "read", "audit_first_byte"), "adrp + ldrb [x8,#off]: {byte:?}");
    let base = rows(bin, "--to", "audit_vlabel");
    assert!(has(&base, "data", "audit_indexed_byte"), "adrp + add + indexed ldrb: {base:?}");
    let global = rows(bin, "--to", "audit_global");
    assert!(has(&global, "read", "audit_leaf") && has(&global, "write", "audit_store"), "{global:?}");
    let slot = rows(bin, "--to", "audit_func_ptr");
    assert!(has(&slot, "read", "audit_indirect_call"), "the function pointer's load: {slot:?}");
}

#[test]
fn an_aarch64_pair_holds_across_scheduling_a_fall_through_and_a_preserved_call() {
    let bin = "xrefpairs_aarch64";
    let word = rows(bin, "--to", "edge_word");
    assert!(has(&word, "read", "edge_sched"), "two instructions between the halves: {word:?}");
    assert!(has(&word, "read", "edge_branch"), "the load on a cbz's fall-through: {word:?}");
    let text = rows(bin, "--to", "edge_text");
    assert!(has(&text, "data", "edge_saved"), "x19 survives the bl: {text:?}");
    assert!(!has(&text, "data", "edge_clobbered"), "x8 does not survive the bl: {text:?}");
}

#[test]
fn two_bases_meeting_at_a_join_form_no_address() {
    for (bin, join) in [("xrefpairs_aarch64", "edge_join"), ("xrefpairs_rv64", "edge_join"), ("xrefpairs_rv32", "edge_join")] {
        let from = rows(bin, "--from", join);
        let formed: Vec<_> = from.iter().filter(|(_, _, to)| to & 0xfff == 0x10).collect();
        assert!(formed.is_empty(), "{bin}: the add after the join has two possible bases: {from:?}");
    }
}

#[test]
fn a_riscv_lui_or_auipc_pair_is_a_reference_on_rv32_and_rv64() {
    for bin in ["xrefpairs_rv64", "xrefpairs_rv32"] {
        let global = rows(bin, "--to", "audit_global");
        assert!(has(&global, "read", "audit_leaf"), "{bin}: lui + lw: {global:?}");
        assert!(has(&global, "write", "audit_store"), "{bin}: lui + sw: {global:?}");
        let label = rows(bin, "--to", "audit_label");
        assert!(has(&label, "data", "audit_direct_label"), "{bin}: lui + addi: {label:?}");
        let text = rows(bin, "--to", "edge_text");
        assert!(has(&text, "data", "edge_lui_addi"), "{bin}: lui + addi: {text:?}");
        assert!(has(&text, "data", "edge_auipc_addi"), "{bin}: auipc + addi: {text:?}");
        let word = rows(bin, "--to", "edge_word");
        assert!(has(&word, "read", "edge_lui_lw") && has(&word, "write", "edge_lui_sw"), "{bin}: {word:?}");
        let ptr = rows(bin, "--to", "edge_ptr");
        assert!(has(&ptr, "read", "edge_auipc_load"), "{bin}: auipc + ld/lw: {ptr:?}");
    }
}

#[test]
fn an_arm64_macho_adrp_pair_is_a_reference() {
    let bin = "xrefpairs_macho_arm64";
    let label = rows(bin, "--to", "0x1000004d5");
    assert!(has(&label, "data", "_audit_direct_label"), "{label:?}");
    assert!(has(&label, "data", "_audit_pass_label"), "{label:?}");
    let byte = rows(bin, "--to", "0x1000004eb");
    assert!(has(&byte, "read", "_audit_first_byte"), "adrp + ldrsb [x8,#off]: {byte:?}");
    let base = rows(bin, "--to", "0x1000004e8");
    assert!(has(&base, "data", "_audit_indexed_byte"), "adrp + add + indexed ldrsb: {base:?}");
    let global = rows(bin, "--to", "0x100004000");
    assert!(has(&global, "read", "_audit_leaf") && has(&global, "write", "_audit_store"), "{global:?}");
}

#[test]
fn forming_an_address_is_data_not_a_read() {
    let from = rows("xrefpairs_aarch64", "--from", "audit_direct_label");
    let label = 0x201025;
    assert_eq!(
        from.iter().filter(|(k, _, to)| k == "data" && *to == label).count(),
        1,
        "exactly one address formation of the label, at the add: {from:?}"
    );
    assert!(from.iter().all(|(k, _, _)| k == "data"), "forming an address is not reading it: {from:?}");
}
