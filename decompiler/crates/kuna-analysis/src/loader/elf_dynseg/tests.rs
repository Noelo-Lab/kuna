//! The dynamic segment as the only source of import markup.
//!
//! `dynseg_plt_x86_64` (generator alongside it) is a section-table-free ELF64
//! PIE importing `puts` and `memcpy` through `.plt.sec`-shaped stubs — the
//! reduction of round-4 crackme `KataVM_L1`. Nothing in it is reachable by
//! section name, so any name these tests see came through `PT_DYNAMIC`.

use super::*;

fn fixture(name: &str) -> Vec<u8> {
    let path = format!("{}/tests/fixtures/{}", env!("CARGO_MANIFEST_DIR"), name);
    std::fs::read(&path).unwrap_or_else(|e| panic!("read {path}: {e}"))
}

#[test]
fn names_both_got_slots_from_pt_dynamic() {
    let bytes = fixture("dynseg_plt_x86_64");
    let imports = dynamic_imports(&bytes).expect("PT_DYNAMIC present");
    assert_eq!(imports.pltgot, Some(0x300));
    assert_eq!(imports.got_to_name.get(&0x318).map(Vec::as_slice), Some(&b"puts"[..]));
    assert_eq!(imports.got_to_name.get(&0x320).map(Vec::as_slice), Some(&b"memcpy"[..]));
    assert_eq!(imports.got_to_name.len(), 2);
}

/// The section table is what `object` reads, and it is genuinely empty here —
/// so the fixture really does isolate the segment path.
#[test]
fn fixture_has_no_sections_for_object_to_read() {
    use object::read::Object;
    let bytes = fixture("dynseg_plt_x86_64");
    let file = object::File::parse(&*bytes).expect("parses");
    assert_eq!(file.sections().count(), 0);
    assert!(file.dynamic_relocations().map(|r| r.count()).unwrap_or(0) == 0);
}

/// Only the `PF_X` `PT_LOAD` is a stub window: the read-only segment holds the
/// relocation and symbol tables, and the read-write one is the GOT.
#[test]
fn exec_segments_are_the_executable_loads_only() {
    let bytes = fixture("dynseg_plt_x86_64");
    let segs = exec_segments(&bytes);
    assert_eq!(segs.len(), 1);
    assert_eq!(segs[0].0, 0x240);
    assert_eq!(&segs[0].1[..4], &[0xf3, 0x0f, 0x1e, 0xfa]); // endbr64
}

/// End to end: both stubs are named at their own entry addresses, and `main` —
/// a third 16-byte-aligned run of code immediately after them — is not. A reader
/// that guessed a PLT extent out of the executable segment would name it too.
#[test]
fn resolves_stub_names_without_naming_the_code_after_them() {
    let bytes = fixture("dynseg_plt_x86_64");
    let file = object::File::parse(&*bytes).expect("parses");
    let mut syms = super::super::elf_plt::resolve_plt_imports(&file, &bytes);
    syms.sort_by_key(|s| s.addr);
    let got: Vec<(u64, String)> =
        syms.iter().map(|s| (s.addr, String::from_utf8_lossy(&s.name).into_owned())).collect();
    assert_eq!(got, vec![(0x240, "puts".to_string()), (0x250, "memcpy".to_string())]);
}

/// A sectionless *static* image has no `PT_DYNAMIC` at all: no tags, no names,
/// and no panic on the way to saying so.
#[test]
fn sectionless_static_image_yields_nothing() {
    let bytes = fixture("noshdr_x86_64");
    assert!(dynamic_imports(&bytes).is_none());
    let file = object::File::parse(&*bytes).expect("parses");
    assert!(super::super::elf_plt::resolve_plt_imports(&file, &bytes).is_empty());
}

/// Truncation at every length must be answered, not crashed on. The dynamic
/// tables point at offsets the prefix no longer contains, which is exactly the
/// shape a hostile file uses.
#[test]
fn truncated_images_never_panic() {
    let bytes = fixture("dynseg_plt_x86_64");
    for n in 0..bytes.len() {
        let head = &bytes[..n];
        let _ = dynamic_imports(head);
        let _ = exec_segments(head);
    }
}

/// A `DT_PLTRELSZ` far larger than the file must not turn into an unbounded
/// walk, and the entries that *are* readable still resolve.
#[test]
fn oversized_relocation_table_size_is_bounded() {
    let mut bytes = fixture("dynseg_plt_x86_64");
    // DT_PLTRELSZ is the 6th tag (0-based 5) of the dynamic segment at 0x120.
    let at = 0x120 + 5 * 16;
    assert_eq!(u64::from_le_bytes(bytes[at..at + 8].try_into().unwrap()), 2);
    bytes[at + 8..at + 16].copy_from_slice(&u64::MAX.to_le_bytes());
    let imports = dynamic_imports(&bytes).expect("PT_DYNAMIC present");
    assert_eq!(imports.got_to_name.get(&0x318).map(Vec::as_slice), Some(&b"puts"[..]));
}

/// A non-ELF input is not this module's business and is refused up front.
#[test]
fn non_elf_is_refused() {
    assert!(dynamic_imports(b"MZ\x90\x00not an elf at all........").is_none());
    assert!(exec_segments(b"MZ\x90\x00not an elf at all........").is_empty());
}
