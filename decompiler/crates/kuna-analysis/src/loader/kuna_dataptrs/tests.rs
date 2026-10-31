//! The data-pointer list per format, against vendored images.

use super::*;

fn load(name: &str) -> Vec<u8> {
    let path = format!("{}/tests/fixtures/{name}", env!("CARGO_MANIFEST_DIR"));
    std::fs::read(&path).unwrap_or_else(|e| panic!("read {path}: {e}"))
}

fn pointers(name: &str) -> DataPointers {
    let bytes = load(name);
    let file = object::File::parse(&bytes[..]).expect("parse");
    data_pointers(&file, &bytes)
}

/// GH #1333: the two initialized pointer slots are chained-fixup rebases, in
/// `ld64.lld`'s layout and Apple `ld64`'s; the bound import slot is no address
/// of this image.
#[test]
fn chained_fixup_rebases_are_data_pointers() {
    for name in ["macho_ptrslots_arm64", "macho_ptrslots_offset_arm64"] {
        let got = pointers(name);
        assert_eq!(got.coverage, Coverage::ChainedFixups, "{name}");
        assert_eq!(
            got.pointers,
            vec![
                DataPointer { slot: 0x100004000, target: 0x100000440, width: 8 },
                DataPointer { slot: 0x100004008, target: 0x1000003c8, width: 8 },
            ],
            "{name}"
        );
    }
}

/// A classic `LC_DYLD_INFO` image lists its pointers in rebase opcodes:
/// `macho_objc`'s ten (`llvm-objdump --macho --rebase`), the first the
/// `__objc_classlist` entry for `class_t Greeter`.
#[test]
fn rebase_opcodes_list_a_classic_images_pointers() {
    let got = pointers("macho_objc");
    assert_eq!(got.coverage, Coverage::RebaseOpcodes);
    assert_eq!(got.pointers.len(), 10, "{:x?}", got.pointers);
    assert_eq!(got.pointers[0], DataPointer { slot: 0x100002000, target: 0x100003000, width: 8 });
}

/// A PIE's `R_X86_64_RELATIVE` relocations, and a PE's base relocations.
#[test]
fn elf_and_pe_relocations_list_their_pointers() {
    let pie = pointers("cet_pie_x86_64");
    assert_eq!(pie.coverage, Coverage::DynamicRelocations);
    assert_eq!(pie.pointers.len(), 3, "{:x?}", pie.pointers);
    let pe = pointers("msvc_rtti_x64.exe");
    assert_eq!(pe.coverage, Coverage::BaseRelocations);
    assert!(!pe.pointers.is_empty() && pe.pointers.iter().all(|p| p.width == 8));
}

/// A non-PIE ELF stores its pointers as plain words: none are enumerable.
#[test]
fn a_non_pie_elf_lists_none() {
    let got = pointers("fauxware");
    assert_eq!(got.coverage, Coverage::None);
    assert!(got.pointers.is_empty());
}

/// The opcode machine: segment/offset set, an immediate run, a ULEB run with a
/// skip, and an add-after rebase, on a 64-bit image.
#[test]
fn rebase_opcodes_interpret() {
    let ops = [
        0x11, // SET_TYPE_IMM pointer
        0x21, 0x10, // SET_SEGMENT_AND_OFFSET_ULEB seg 1, +0x10
        0x52, // DO_REBASE_IMM_TIMES 2
        0x82, 0x02, 0x08, // DO_REBASE_ULEB_TIMES_SKIPPING_ULEB 2, skip 8
        0x71, 0x00, // DO_REBASE_ADD_ADDR_ULEB 0
        0x00,
    ];
    let got = interpret_rebase(&ops, &[0x1000, 0x4000], 8);
    assert_eq!(got, vec![0x4010, 0x4018, 0x4020, 0x4030, 0x4040]);
}
