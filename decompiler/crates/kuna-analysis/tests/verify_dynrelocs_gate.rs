//! (kuna `dynrelocs`) The dynamic-relocation application, in its own test binary,
//! driven against real fixtures rather than synthetic images.

use kuna_analysis::loader::kuna_dynrelocs::resolve;

/// A PIE yields its three `R_X86_64_RELATIVE` writes and the two
/// `PT_GNU_RELRO`-covered const slots.
#[test]
fn a_pie_applies_its_relative_slots() {
    let path = format!("{}/tests/fixtures/cet_pie_x86_64", env!("CARGO_MANIFEST_DIR"));
    let bytes = std::fs::read(&path).expect("PIE fixture");
    let file = object::File::parse(&bytes[..]).expect("parse");
    let on = resolve(&file, &bytes);
    assert_eq!(on.writes.len(), 3, "the three RELATIVE slots (imports are skipped)");
    assert_eq!(
        on.const_ranges,
        vec![(0x3d78u64, 0x3d7fu64), (0x3d80, 0x3d87)],
        "only the RELRO-covered slots"
    );
}

/// The Mach-O arm: each rebase is its target and the bind a zero, in Apple's
/// `DYLD_CHAINED_PTR_64_OFFSET` layout as in `ld64.lld`'s `DYLD_CHAINED_PTR_64`.
#[test]
fn macho_chained_fixups_are_unpacked() {
    for name in ["macho_ptrslots_arm64", "macho_ptrslots_offset_arm64"] {
        let path = format!("{}/tests/fixtures/{name}", env!("CARGO_MANIFEST_DIR"));
        let bytes = std::fs::read(&path).expect("Mach-O fixture");
        let file = object::File::parse(&bytes[..]).expect("parse");
        let on: Vec<(u64, u64, u8)> =
            resolve(&file, &bytes).writes.iter().map(|w| (w.vma, w.value, w.width)).collect();
        assert_eq!(
            on,
            vec![(0x100004000, 0x100000440, 8), (0x100004008, 0x1000003c8, 8), (0x100004010, 0, 8)],
            "{name}: two rebases and a zeroed bind"
        );
    }
}

/// The MIPS primary GOT, which the dynamic linker relocates with no relocation
/// records: the local entries after the module pointer and the defined symbols'
/// global entries are constant.
#[test]
fn mips_got_slots_are_constant() {
    let path = format!("{}/tests/fixtures/mipsgot_gcc_be.so", env!("CARGO_MANIFEST_DIR"));
    let bytes = std::fs::read(&path).expect("MIPS fixture");
    let file = object::File::parse(&bytes[..]).expect("parse");
    assert_eq!(kuna_analysis::loader::kuna_mipsgot::const_ranges(&file), vec![(0x10428u64, 0x10443u64)]);
}

/// A chained Mach-O's rebases are unpacked and its `__DATA_CONST` slots declared
/// constant.
#[test]
fn macho_chained_fixups_declare_data_const() {
    let path = format!("{}/tests/fixtures/macho_objc_refs_arm64", env!("CARGO_MANIFEST_DIR"));
    let bytes = std::fs::read(&path).expect("Mach-O fixture");
    let file = object::File::parse(&bytes[..]).expect("parse");
    let on = resolve(&file, &bytes);
    assert!(on.writes.iter().any(|w| w.vma == 0x1000081f8 && w.value == 0x100000948), "the first selref unpacked");
    assert!(on.const_ranges.contains(&(0x100004010, 0x100004017)), "__objc_classlist[0] in __DATA_CONST");
    assert!(!on.const_ranges.iter().any(|&(lo, _)| lo == 0x1000081f8), "a __DATA selref is not RELRO");
}
