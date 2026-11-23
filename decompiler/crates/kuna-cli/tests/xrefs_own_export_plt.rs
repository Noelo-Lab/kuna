//! `kuna xrefs --to <name>` on a shared library that calls its own export through
//! `name@plt` (issue #1031): the name answers for the export, so the calls that
//! reach it through the stub and its GOT slot are its callers.

use crate::common;
use serde_json::Value;

const BODY: u64 = 0x1120;
const STUB: u64 = 0x1050;
const SLOT: u64 = 0x4018;

fn xrefs_to(spec: &str) -> (Value, String) {
    let lib = common::fixture("selfplt_x86_64.so");
    let (out, err, code) = common::run_kuna(&["xrefs", &lib, "--to", spec, "--json"]);
    assert_eq!(code, 0, "{err}");
    (serde_json::from_str(&out).expect("xrefs JSON"), err)
}

fn aliases(doc: &Value) -> Vec<u64> {
    doc["target"]["aliases"]
        .as_array()
        .unwrap()
        .iter()
        .map(|a| a["address"].as_u64().unwrap())
        .collect()
}

#[test]
fn a_name_counts_the_calls_through_the_exports_own_plt_stub() {
    let (doc, err) = xrefs_to("bump");
    assert!(!err.contains("warning"), "{err}");
    assert_eq!(doc["target"]["address"].as_u64(), Some(BODY));
    assert_eq!(aliases(&doc), vec![STUB, SLOT]);
    let sites: Vec<(u64, &str, u64)> = doc["xrefs"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| {
            (
                r["from_address"].as_u64().unwrap(),
                r["kind"].as_str().unwrap(),
                r["to_address"].as_u64().unwrap(),
            )
        })
        .collect();
    assert_eq!(
        sites,
        vec![
            (0x1137, "call", STUB),
            (0x1140, "jump", STUB),
            (0x1158, "call", STUB)
        ]
    );
    assert_eq!(doc["count"].as_u64(), Some(3));
}

#[test]
fn an_address_still_answers_for_that_address() {
    let (body, err) = xrefs_to("0x1120");
    assert!(err.is_empty(), "{err}");
    assert_eq!((body["count"].as_u64(), aliases(&body)), (Some(0), vec![]));
    let (stub, err) = xrefs_to("0x1050");
    assert!(err.is_empty(), "{err}");
    assert_eq!(
        (stub["count"].as_u64(), aliases(&stub)),
        (Some(3), vec![SLOT])
    );
}

#[test]
fn a_name_follows_only_the_slot_bound_to_its_definition() {
    for (fixture, name, stub, slot, caller) in [
        ("pltlocal_aarch64.so", "timer_helper", 0x10380, 0x30468, 0x1032c),
        ("dataptrs_arch_ppc32_bssplt.so", "audit_export", 0x2064, 0x2064, 0x2f4),
        ("dataptrs_arch_ppc64_elfv1.so", "audit_export", 0x3e0, 0x2018, 0x4d4),
    ] {
        let file = common::fixture(fixture);
        let (out, err, code) = common::run_kuna(&["xrefs", &file, "--to", name, "--kind", "call", "--json"]);
        assert_eq!(code, 0, "{err}");
        let doc: Value = serde_json::from_str(&out).unwrap();
        let expected = if stub == slot { vec![stub] } else { vec![stub, slot] };
        assert_eq!(aliases(&doc), expected, "{fixture}");
        assert_eq!(doc["count"], 1, "{doc}");
        assert_eq!(doc["xrefs"][0]["from_address"], caller);
        assert_eq!(doc["xrefs"][0]["to_address"], stub);
    }
}

#[test]
fn an_unbound_name_or_a_stub_without_a_forwarding_jump_is_not_an_alias() {
    use object::{Object, ObjectSection};
    let original = std::fs::read(common::fixture("pltlocal_aarch64.so")).unwrap();
    let file = object::File::parse(original.as_slice()).unwrap();
    let section = |name| file.section_by_name(name).unwrap().file_range().unwrap().0 as usize;
    let symbol = section(".dynsym") + 2 * 24;
    let plt = section(".plt");
    let rela = section(".rela.plt");
    let path = common::scratch_file("export-plt-control", "so");
    for (offset, before, after) in [
        // Undefined dynsym entry still names the stub; .symtab retains the named body.
        (symbol + 6, vec![6, 0], vec![0, 0]),
        // The jump slot binds the other definition, which shares the displayed name.
        (rela + 12, 2u32.to_le_bytes().to_vec(), 3u32.to_le_bytes().to_vec()),
        // Loading the correct slot and returning instead of jumping through it.
        (plt + 44, 0xd61f0220u32.to_le_bytes().to_vec(), 0xd65f03c0u32.to_le_bytes().to_vec()),
    ] {
        let mut bytes = original.clone();
        assert_eq!(bytes[offset..offset + before.len()], before);
        bytes[offset..offset + after.len()].copy_from_slice(&after);
        if offset == rela + 12 {
            bytes.copy_within(symbol..symbol + 4, symbol + 24);
        }
        std::fs::write(&path, bytes).unwrap();
        let (out, err, code) = common::run_kuna(&["xrefs", path.to_str().unwrap(), "--to", "timer_helper",
            "--kind", "call", "--json"]);
        assert_eq!(code, 0, "{err}");
        let doc: Value = serde_json::from_str(&out).unwrap();
        assert_eq!(doc["target"]["address"], 0x10344, "{doc}");
        assert!(aliases(&doc).is_empty(), "{doc}");
        assert_eq!(doc["count"], 0, "{doc}");
    }
    std::fs::remove_file(path).unwrap();
}

#[test]
fn ppc32_absolute_stubs_keep_same_named_definitions_separate_in_each_mode() {
    let mut elf = std::fs::read(common::fixture("dataptrs_arch_ppc32.so")).unwrap();
    // Replace the four-word PIC stub by lis/lwz/mtctr/bctr through its fixed slot.
    for (at, word) in elf[0x3fc..0x40c].chunks_exact_mut(4).zip([
        0x3d600003u32, 0x816b04e4, 0x7d6903a6, 0x4e800420,
    ]) {
        at.copy_from_slice(&word.to_be_bytes());
    }
    let path = common::scratch_file("absolute-export-plt", "so");
    for bound in [true, false] {
        if !bound {
            // Select dynsym[2] in the JMP_SLOT; copy dynsym[1]'s name to it.
            elf[0x278..0x27c].copy_from_slice(&0x215u32.to_be_bytes());
            elf.copy_within(0x164..0x168, 0x174);
        }
        std::fs::write(&path, &elf).unwrap();
        for mode in ["off", "on", "abi"] {
            let (out, err, code) = common::run_kuna(&["xrefs", path.to_str().unwrap(), "--to",
                "audit_export", "--kind", "call", "--option", "ppc32picimports", mode, "--json"]);
            assert_eq!(code, 0, "{err}");
            let doc: Value = serde_json::from_str(&out).unwrap();
            assert_eq!(doc["target"]["address"], 0x1035c);
            assert_eq!(doc["count"], u64::from(bound), "{doc}");
            if bound {
                assert_eq!(aliases(&doc), [0x103fc, 0x304e4]);
                assert_eq!(doc["xrefs"][0]["from_address"], 0x103e4);
            } else {
                assert!(aliases(&doc).is_empty(), "{doc}");
            }
        }
    }
    std::fs::remove_file(path).unwrap();
}

#[test]
fn a_value_equal_to_the_stub_address_is_not_a_reference_to_the_export() {
    let mut elf = std::fs::read(common::fixture("dataptrs_arch_ppc32_bssplt.so")).unwrap();
    // audit_target's `addi r9,r9,7` becomes `li r9,0x2064`, the BSS PLT entry's address.
    assert_eq!(elf[0x244..0x248], 0x39290007u32.to_be_bytes());
    elf[0x244..0x248].copy_from_slice(&0x39202064u32.to_be_bytes());
    let path = common::scratch_file("export-plt-immediate", "so");
    std::fs::write(&path, &elf).unwrap();
    let rows = |spec: &str| -> Vec<(u64, String)> {
        let (out, err, code) = common::run_kuna(&["xrefs", path.to_str().unwrap(), "--to", spec, "--json"]);
        assert_eq!(code, 0, "{err}");
        let doc: Value = serde_json::from_str(&out).unwrap();
        doc["xrefs"].as_array().unwrap().iter()
            .map(|r| (r["from_address"].as_u64().unwrap(), r["kind"].as_str().unwrap().to_string()))
            .collect()
    };
    assert_eq!(rows("audit_export"), [(0x2f4, "call".to_string())]);
    assert_eq!(rows("0x2064"), [(0x244, "data".to_string()), (0x2f4, "call".to_string())]);
    std::fs::remove_file(path).unwrap();
}
