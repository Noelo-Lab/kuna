//! String queries use the same relocatable address space as the loader.

use object::write::{Object, Relocation, Symbol, SymbolSection};
use object::{
    Architecture, BinaryFormat, Endianness, RelocationFlags, SectionFlags, SectionKind,
    SymbolFlags, SymbolKind, SymbolScope,
};
use serde_json::Value;

mod common;

fn image() -> Vec<u8> {
    let mut obj = Object::new(BinaryFormat::Coff, Architecture::X86_64, Endianness::Little);
    let text = obj.add_section(Vec::new(), b".text".to_vec(), SectionKind::Text);
    obj.append_section_data(text, b"\x48\x8d\x05\0\0\0\0\xc3\0text_literal\0", 16);
    obj.add_symbol(Symbol {
        name: b"entry".to_vec(),
        value: 0,
        size: 8,
        kind: SymbolKind::Text,
        scope: SymbolScope::Linkage,
        weak: false,
        section: SymbolSection::Section(text),
        flags: SymbolFlags::None,
    });
    let data = obj.add_section(Vec::new(), b".rdata".to_vec(), SectionKind::ReadOnlyData);
    obj.append_section_data(data, b"synthetic_object_literal\0", 16);
    let literal = obj.add_symbol(Symbol {
        name: b"literal".to_vec(),
        value: 0,
        size: 25,
        kind: SymbolKind::Data,
        scope: SymbolScope::Linkage,
        weak: false,
        section: SymbolSection::Section(data),
        flags: SymbolFlags::None,
    });
    obj.add_relocation(
        text,
        Relocation {
            offset: 3,
            symbol: literal,
            addend: 0,
            flags: RelocationFlags::Coff {
                typ: object::pe::IMAGE_REL_AMD64_REL32,
            },
        },
    )
    .unwrap();
    let second = obj.add_section(Vec::new(), b".rdata".to_vec(), SectionKind::ReadOnlyData);
    obj.append_section_data(second, "second_literal\0unicode_λ_literal\0".as_bytes(), 16);
    let wide = obj.add_section(Vec::new(), b".wide".to_vec(), SectionKind::ReadOnlyData);
    let wide_bytes: Vec<u8> = "wide_literal\0"
        .encode_utf16()
        .flat_map(u16::to_le_bytes)
        .collect();
    obj.append_section_data(wide, &wide_bytes, 16);
    let debug = obj.add_section(Vec::new(), b".debug$S".to_vec(), SectionKind::ReadOnlyData);
    obj.section_mut(debug).flags = SectionFlags::Coff {
        characteristics: object::pe::IMAGE_SCN_CNT_INITIALIZED_DATA
            | object::pe::IMAGE_SCN_MEM_READ
            | object::pe::IMAGE_SCN_MEM_DISCARDABLE,
    };
    obj.append_section_data(debug, b"unmapped_debug_literal\0", 1);
    obj.write().unwrap()
}

fn fixture() -> std::path::PathBuf {
    let path = common::scratch_file("coff-strings", "obj");
    std::fs::write(&path, image()).unwrap();
    path
}

fn query(path: &std::path::Path, extra: &[&str]) -> Value {
    let mut args = vec![
        "strings",
        path.to_str().unwrap(),
        "--json",
        "--termination",
        "nul",
    ];
    args.extend_from_slice(extra);
    let (out, err, status) = common::run_kuna(&args);
    assert_eq!(status, 0, "{args:?}: {err}");
    serde_json::from_str(&out).unwrap()
}

#[test]
fn zero_vma_sections_keep_their_own_literals_at_loader_addresses() {
    let path = fixture();
    let root = common::repo_root();
    let prog = kuna_console::engine::bootstrap_from_object(
        path.to_str().unwrap(),
        "",
        &[root.join("specs").to_str().unwrap().to_owned()],
    )
    .unwrap();
    let found = query(&path, &["--no-xrefs"]);
    let rows = found["strings"].as_array().unwrap();
    for (literal, section) in [
        ("text_literal", ".text"),
        ("synthetic_object_literal", ".rdata"),
        ("second_literal", ".rdata"),
    ] {
        let row = rows.iter().find(|r| r["text"] == literal).expect(literal);
        assert_eq!(row["section"], section);
        let addr = row["address"].as_u64().unwrap();
        assert!(addr >= 0x400000, "{row}");
        let bytes = prog.read_bytes(addr, literal.len() + 1).unwrap();
        assert_eq!(bytes, format!("{literal}\0").as_bytes());
    }
    assert!(!rows.iter().any(|r| r["text"] == "unmapped_debug_literal"));
}

#[test]
fn relocated_code_references_the_mapped_string() {
    let path = fixture();
    let found = query(&path, &["--filter", "^synthetic_object_literal$"]);
    let row = &found["strings"][0];
    assert_eq!(found["count"], 1);
    assert_eq!(row["xrefs_count"], 1, "{found}");
    assert_eq!(row["functions"][0]["name"], "entry", "{found}");
}

#[test]
fn encodings_and_section_filters_use_the_mapped_regions() {
    let path = fixture();
    for (encoding, section, literal) in [
        ("utf8", "rdata", "unicode_λ_literal"),
        ("utf16", "wide", "wide_literal"),
    ] {
        let found = query(
            &path,
            &["--no-xrefs", "--encoding", encoding, "--section", section],
        );
        let row = found["strings"]
            .as_array()
            .unwrap()
            .iter()
            .find(|r| r["text"] == literal)
            .expect(literal);
        assert!(row["address"].as_u64().unwrap() >= 0x400000);
        assert_eq!(row["encoding"], encoding);
    }
}

#[test]
fn explicit_rebase_choices_are_applied_before_scanning() {
    let path = fixture();
    let enabled = query(&path, &["--no-xrefs", "--option", "relocrebase", "on"]);
    assert!(enabled["strings"]
        .as_array()
        .unwrap()
        .iter()
        .all(|r| r["address"].as_u64().unwrap() >= 0x400000));
    let disabled = query(&path, &["--no-xrefs", "--option", "relocrebase", "off"]);
    assert!(disabled["strings"]
        .as_array()
        .unwrap()
        .iter()
        .all(|r| r["address"].as_u64().unwrap() < 0x400000));
    let restored = query(
        &path,
        &[
            "--no-xrefs",
            "--option",
            "relocrebase",
            "off",
            "--option",
            "relocrebase",
            "on",
        ],
    );
    assert_eq!(restored, enabled);
}
