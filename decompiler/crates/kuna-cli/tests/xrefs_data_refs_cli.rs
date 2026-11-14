//! (GH #1333) `kuna xrefs` answers a pointer the image stores in its data as a
//! separate `pointer` row, from the image's own fixup metadata, and says which
//! metadata it read, so an empty answer is never ambiguous. (GH #1502) It also
//! says when it read only part of that metadata, or none of a kind it does not
//! read, and why.

use std::path::PathBuf;
use std::process::Command;

use crate::common;

fn fixture(name: &str) -> String {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../kuna-analysis/tests/fixtures")
        .join(name)
        .to_str()
        .unwrap()
        .to_string()
}

fn xrefs(args: &[&str]) -> String {
    let out = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .arg("xrefs")
        .args(args)
        .output()
        .expect("run kuna xrefs");
    assert!(out.status.success(), "kuna xrefs {args:?}: {}", String::from_utf8_lossy(&out.stderr));
    String::from_utf8_lossy(&out.stdout).into_owned()
}

fn json(args: &[&str]) -> serde_json::Value {
    let mut argv = args.to_vec();
    argv.push("--json");
    serde_json::from_str(&xrefs(&argv)).expect("xrefs JSON")
}

/// The function-pointer slot `audit_func_ptr` is a `pointer` row beside the one
/// instruction call, in both chained-fixup layouts; `count` and `xrefs` keep
/// the instruction rows alone.
#[test]
fn a_stored_function_pointer_is_a_pointer_row() {
    for name in ["macho_ptrslots_arm64", "macho_ptrslots_offset_arm64"] {
        let doc = json(&[&fixture(name), "--to", "_audit_target"]);
        assert_eq!(doc["count"], 1, "{name}: the bl in audit_direct_call");
        assert_eq!(doc["xrefs"][0]["kind"], "call", "{name}");
        assert_eq!(doc["data_refs_coverage"], "chained-fixups", "{name}");
        assert_eq!(doc["data_refs_count"], 1, "{name}");
        let row = &doc["data_refs"][0];
        assert_eq!(row["kind"], "pointer");
        assert_eq!(row["from_address_hex"], "0x100004008");
        assert_eq!(row["to_address_hex"], "0x1000003c8");
        assert_eq!(row["width"], 8);
        assert_eq!(row["source"], "chained-fixups");

        let label = json(&[&fixture(name), "--to", "0x100000440"]);
        assert_eq!(label["data_refs"][0]["from_address_hex"], "0x100004000", "{name}: audit_label_ptr");
    }
}

/// `--kind` selects between the two: `pointer` alone drops the instruction rows,
/// an instruction kind alone drops the pointer rows. The text surface prints
/// them as their own block.
#[test]
fn kind_filters_pointer_rows() {
    let bin = fixture("macho_ptrslots_offset_arm64");
    let only = json(&[&bin, "--to", "_audit_target", "--kind", "pointer"]);
    assert_eq!((only["count"].as_u64(), only["data_refs_count"].as_u64()), (Some(0), Some(1)));
    let calls = json(&[&bin, "--to", "_audit_target", "--kind", "call"]);
    assert_eq!((calls["count"].as_u64(), calls["data_refs_count"].as_u64()), (Some(1), Some(0)));
    let text = xrefs(&[&bin, "--to", "_audit_target"]);
    assert!(
        text.contains("# 1 data-held pointer to _audit_target @ 0x1000003c8 (chained-fixups)\n0x100004008\tpointer\t"),
        "{text}"
    );
}

/// `--from` a data slot answers what it holds; a classic Mach-O lists its
/// pointers in rebase opcodes; a non-PIE ELF lists none and says so.
#[test]
fn coverage_is_reported_per_image() {
    let slot = json(&[&fixture("macho_ptrslots_offset_arm64"), "--from", "0x100004008"]);
    assert_eq!(slot["data_refs"][0]["to_address_hex"], "0x1000003c8");
    let classic = json(&[&fixture("macho_objc"), "--to", "0x100003000"]);
    assert_eq!(classic["data_refs_coverage"], "rebase-opcodes");
    assert_eq!(classic["data_refs"][0]["from_address_hex"], "0x100002000", "the __objc_classlist entry");
    let plain = json(&[&fixture("fauxware"), "--to", "main"]);
    assert_eq!(plain["data_refs_coverage"], "none");
    assert_eq!(plain["data_refs_count"], 0);
}

/// A shared object stores its exported function in a table through an absolute
/// relocation against the symbol; that word is a pointer row too.
#[test]
fn a_shared_objects_exported_function_table_is_listed() {
    let doc = json(&[&fixture("dataptrs_so_x86_64.so"), "--to", "audit_target"]);
    assert_eq!(doc["data_refs_coverage"], "dynamic-relocations");
    assert_eq!(doc["data_refs_count"], 1, "{doc}");
    assert_eq!(doc["data_refs"][0]["from_address_hex"], "0x4030");
    assert_eq!(doc["data_refs"][0]["from_name"], "exported_tab");
}

/// On x86 a relocated word inside a decoded instruction that names the same
/// address is that instruction's operand, not a stored pointer; a pointer table
/// in `.text` that starts just past such an instruction is still listed.
#[test]
fn an_x86_operand_is_the_instructions_row_and_a_text_table_is_a_pointer() {
    let exe = fixture("dataptrs_textptr_i386.exe");
    let rows = |doc: &serde_json::Value| -> Vec<String> {
        doc["data_refs"].as_array().unwrap().iter().map(|r| r["from_address_hex"].as_str().unwrap().to_string()).collect()
    };
    let dispatch = json(&[&exe, "--to", "0x401013"]);
    assert_eq!(dispatch["data_refs_coverage"], "base-relocations");
    assert_eq!(rows(&dispatch), ["0x401058", "0x403004"], "the mov's operand 0x40104f is not a pointer row: {dispatch}");
    assert!(dispatch["xrefs"].as_array().unwrap().iter().any(|r| r["from_address_hex"] == "0x40104e"), "{dispatch}");
    let helper = json(&[&exe, "--to", "0x40104e"]);
    assert_eq!(rows(&helper), ["0x401054", "0x403000"], "the push's operand 0x401001 is not a pointer row: {helper}");
}

/// A copy of fixture `name` with `patch` applied, under cargo's scratch
/// directory.
fn patched(name: &str, patch: impl FnOnce(&mut Vec<u8>)) -> String {
    let mut bytes = std::fs::read(fixture(name)).expect("read fixture");
    patch(&mut bytes);
    let path = common::scratch_file(name, "patched");
    std::fs::write(&path, &bytes).expect("write patched fixture");
    path.to_str().unwrap().to_string()
}

/// The file offset of a little-endian ELF64's section header named `wanted`.
fn elf64_section_header(bytes: &[u8], wanted: &str) -> usize {
    let u16at = |at: usize| u16::from_le_bytes(bytes[at..at + 2].try_into().unwrap()) as usize;
    let u32at = |at: usize| u32::from_le_bytes(bytes[at..at + 4].try_into().unwrap()) as usize;
    let u64at = |at: usize| u64::from_le_bytes(bytes[at..at + 8].try_into().unwrap()) as usize;
    let (shoff, shentsize, shnum, shstrndx) = (u64at(0x28), u16at(0x3a), u16at(0x3c), u16at(0x3e));
    let strings = u64at(shoff + shstrndx * shentsize + 0x18);
    (0..shnum)
        .map(|i| shoff + i * shentsize)
        .find(|&header| {
            let name = &bytes[strings + u32at(header)..];
            &name[..name.iter().position(|&b| b == 0).unwrap()] == wanted.as_bytes()
        })
        .unwrap_or_else(|| panic!("no section {wanted}"))
}

/// GH #1502: a RISC-V shared object's `R_RISCV_RELATIVE` slot is a pointer row
/// under complete coverage.
#[test]
fn a_risc_v_relative_relocation_is_a_pointer_row() {
    let doc = json(&[&fixture("dataptrs_arch_riscv64.so"), "--to", "audit_target"]);
    assert_eq!(doc["data_refs_coverage"], "dynamic-relocations", "{doc}");
    assert_eq!(doc["data_refs_reason"], serde_json::Value::Null);
    assert_eq!(doc["data_refs_count"], 1, "{doc}");
    assert_eq!(doc["data_refs"][0]["from_address_hex"], "0x3568");
    assert_eq!(doc["data_refs"][0]["from_name"], "audit_slot");
    assert_eq!(doc["data_refs"][0]["to_address_hex"], "0x13d0");
}

/// A `.rela.dyn` whose `sh_link` is not the dynamic symbol table is a table the
/// relocation walk skips: the list is `partial`, says why, and the text says
/// so too, where the intact image lists its pointer completely.
#[test]
fn an_unreadable_relocation_section_makes_the_list_partial() {
    let broken = patched("dataptrs_so_x86_64.so", |bytes| {
        let header = elf64_section_header(bytes, ".rela.dyn");
        bytes[header + 0x28..header + 0x2c].copy_from_slice(&999u32.to_le_bytes());
    });
    let doc = json(&[&broken, "--to", "audit_target"]);
    assert_eq!(doc["data_refs_coverage"], "partial", "{doc}");
    assert_eq!(doc["data_refs_count"], 0, "{doc}");
    assert_eq!(
        doc["data_refs_reason"],
        ".rela.dyn (section 4): sh_link 999 is not the dynamic symbol table",
        "{doc}"
    );
    let text = xrefs(&[&broken, "--to", "audit_target"]);
    assert!(
        text.contains(
            "# 0 data-held pointers to audit_target @ 0x1000 (dynamic-relocations, partial: .rela.dyn (section 4): \
             sh_link 999 is not the dynamic symbol table)"
        ),
        "{text}"
    );
}

/// A PE whose base-relocation directory lies outside every section lists
/// nothing, as `partial` with the directory named, not as `none`.
#[test]
fn an_unreadable_base_relocation_directory_makes_the_list_partial() {
    let broken = patched("dataptrs_textptr_i386.exe", |bytes| {
        let pe = u32::from_le_bytes(bytes[0x3c..0x40].try_into().unwrap()) as usize;
        let optional = pe + 24;
        let directories = optional + if bytes[optional..optional + 2] == [0x0b, 0x02] { 112 } else { 96 };
        let basereloc = directories + 5 * 8;
        bytes[basereloc..basereloc + 4].copy_from_slice(&0x7fff_0000u32.to_le_bytes());
    });
    let doc = json(&[&broken, "--to", "0x401013"]);
    assert_eq!(doc["data_refs_coverage"], "partial", "{doc}");
    assert_eq!(doc["data_refs_count"], 0, "{doc}");
    let reason = doc["data_refs_reason"].as_str().unwrap_or_default();
    assert!(reason.starts_with("the base-relocation directory (RVA 0x7fff0000,"), "{doc}");
}

/// Metadata kuna does not read is `unsupported`, not `none`: a MIPS image's
/// dynamic relocations (its local GOT is relocated without one) and a
/// relocatable object's relocations. A non-PIE ELF keeps none, and says so.
#[test]
fn metadata_kuna_does_not_read_is_unsupported() {
    let mips = json(&[&fixture("mipsstubs_noreturn_el.so"), "--to", "say"]);
    assert_eq!(mips["data_refs_coverage"], "unsupported", "{mips}");
    assert!(mips["data_refs_reason"].as_str().unwrap_or_default().starts_with("MIPS"), "{mips}");
    let text = xrefs(&[&fixture("mipsstubs_noreturn_el.so"), "--to", "say"]);
    assert!(text.contains("# data-held pointers to say @ 0x494 not listed (unsupported: MIPS"), "{text}");
    let object = json(&[&fixture("armfloatargs_calls_arm-O0.o"), "--to", "k7"]);
    assert_eq!(object["data_refs_coverage"], "unsupported", "{object}");
    let plain = json(&[&fixture("fauxware"), "--to", "main"]);
    assert_eq!(plain["data_refs_coverage"], "none");
    assert!(plain["data_refs_reason"].as_str().unwrap_or_default().contains("ET_EXEC"), "{plain}");
}

/// Packed `SHT_RELR` relocations list their slots like any `RELATIVE` one.
#[test]
fn relr_relocations_are_pointer_rows() {
    let doc = json(&[&fixture("relr_so_x86_64.so"), "--to", "0x1010"]);
    assert_eq!(doc["data_refs_coverage"], "dynamic-relocations", "{doc}");
    assert_eq!(doc["data_refs_count"], 1, "{doc}");
    assert_eq!(doc["data_refs"][0]["from_address_hex"], "0x4038", "exported_tab[1]: {doc}");
}

/// An `MH_FILESET` entry's pointers are relocated by the collection's chained
/// fixups, which kuna applies when it cuts the entry out: they are
/// `unsupported`, not `none`, whichever entry is loaded.
#[test]
fn a_fileset_entrys_pointers_are_unsupported_not_none() {
    let collection = fixture("kernelcache/fileset_calls.macho");
    for entry in [&[][..], &["--option", "kernelcache", "com.example.driver.PMGR"][..]] {
        let mut args = vec![collection.as_str(), "--to", "0xfffffe0007007840"];
        args.extend_from_slice(entry);
        let doc = json(&args);
        assert_eq!(doc["data_refs_coverage"], "unsupported", "{entry:?}: {doc}");
        assert!(doc["data_refs_reason"].as_str().unwrap_or_default().contains("MH_FILESET"), "{doc}");
    }
}
