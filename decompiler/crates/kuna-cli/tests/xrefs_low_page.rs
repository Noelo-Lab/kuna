//! `kuna xrefs` over an address formed from the program counter below 0x1000
//! (GH #1364). A zero-based image maps code and constants in its first page:
//! an arm64 Mach-O dylib's `__TEXT`, the `.rodata` ld.lld puts first in an ELF
//! shared object or PIE, a firmware image linked at 0. A PC-relative form there
//! (`adrp`/`adr`, `lea [rip]`, `auipc`, the i386 GOT pointer, A32 and Thumb
//! `adr`) is a reference however small its address; the same value built as a
//! plain immediate is a number and stays unreported.
//!
//! Fixtures: `lowpage_*` (see `lowpage.provenance`). Integration tests require
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

fn has(rows: &[(String, String, u64, u64)], kind: &str, from: &str) -> bool {
    rows.iter().any(|(k, f, _, _)| k == kind && f == from)
}

/// The issue's dylib: `adrp x0, 0x0; add x0, x0, #0x378` forms `_audit_target`.
#[test]
fn an_adrp_pair_into_a_dylibs_first_page_is_a_reference() {
    let bin = "lowpage_macho_arm64";
    let target = rows(bin, "--to", "_audit_target");
    assert!(has(&target, "call", "_audit_direct"), "{target:?}");
    assert!(target.iter().any(|(k, f, at, _)| k == "data" && f == "_audit_address" && *at == 0x398), "{target:?}");
    assert!(has(&target, "data", "_audit_adr"), "adr x0, _audit_target: {target:?}");
    let from = rows(bin, "--from", "_audit_address");
    assert_eq!(from.iter().map(|r| (r.0.as_str(), r.3)).collect::<Vec<_>>(), [("data", 0x378)], "{from:?}");
    let string = rows(bin, "--to", "0x3d4");
    assert!(has(&string, "data", "_audit_string"), "adrp + add of a __cstring literal: {string:?}");
    let word = rows(bin, "--to", "_audit_word");
    assert!(has(&word, "read", "_audit_read"), "adrp + ldr of a __TEXT,__const word: {word:?}");
}

#[test]
fn a_pc_relative_form_into_an_elf_first_page_is_a_reference() {
    let lea = rows("lowpage_x86_64", "--to", "audit_text");
    assert!(has(&lea, "data", "audit_string"), "lea rax,[rip+...]: {lea:?}");
    let table = rows("lowpage_x86_64", "--to", "audit_words");
    assert!(has(&table, "data", "audit_table"), "lea rcx,[rip+...] before an indexed load: {table:?}");
    let auipc = rows("lowpage_rv64", "--to", "audit_text");
    assert!(has(&auipc, "data", "audit_string"), "auipc + addi: {auipc:?}");
    let load = rows("lowpage_rv64", "--to", "audit_words");
    assert!(has(&load, "read", "audit_read"), "auipc + lw: {load:?}");
    let gotoff = rows("lowpage_i386", "--to", "audit_text");
    assert!(has(&gotoff, "data", "audit_string"), "lea eax,[ebx+GOTOFF]: {gotoff:?}");
    let read = rows("lowpage_i386", "--to", "audit_words");
    assert!(has(&read, "read", "audit_read"), "mov eax,[ebx+GOTOFF]: {read:?}");
}

#[test]
fn an_arm_adr_into_a_low_image_is_a_reference() {
    let text = rows("lowpage_arm", "--to", "audit_text");
    assert!(has(&text, "data", "audit_a32"), "A32 adr: {text:?}");
    assert!(has(&text, "data", "audit_thumb_wide"), "Thumb adr.w: {text:?}");
    let words = rows("lowpage_arm", "--to", "audit_words");
    assert!(has(&words, "data", "audit_thumb"), "Thumb adr: {words:?}");
    let field = rows("lowpage_arm", "--to", "0x13c");
    assert!(has(&field, "read", "audit_thumb"), "Thumb adr + ldr [r0,#4]: {field:?}");
}

/// Each `control_number(s)` builds the addresses the `audit_*` functions form,
/// as immediates (`mov`, `movw`, `li`, an `add` of a non-PC register): numbers.
#[test]
fn the_same_value_as_a_plain_immediate_is_not_a_reference() {
    for (bin, control) in [
        ("lowpage_macho_arm64", "_control_numbers"),
        ("lowpage_x86_64", "control_number"),
        ("lowpage_rv64", "control_number"),
        ("lowpage_i386", "control_number"),
        ("lowpage_arm", "control_numbers"),
    ] {
        let from = rows(bin, "--from", control);
        assert!(from.is_empty(), "{bin}: {from:?}");
    }
}
