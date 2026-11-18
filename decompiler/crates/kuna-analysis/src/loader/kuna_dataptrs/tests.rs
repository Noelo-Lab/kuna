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
        assert_eq!(got.coverage, Coverage::Complete(Source::ChainedFixups), "{name}");
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
    assert_eq!(got.coverage, Coverage::Complete(Source::RebaseOpcodes));
    assert_eq!(got.pointers.len(), 10, "{:x?}", got.pointers);
    assert_eq!(got.pointers[0], DataPointer { slot: 0x100002000, target: 0x100003000, width: 8 });
}

/// A PIE's `R_X86_64_RELATIVE` relocations, and a PE's base relocations.
#[test]
fn elf_and_pe_relocations_list_their_pointers() {
    let pie = pointers("cet_pie_x86_64");
    assert_eq!(pie.coverage, Coverage::Complete(Source::DynamicRelocations));
    assert_eq!(pie.pointers.len(), 3, "{:x?}", pie.pointers);
    let pe = pointers("msvc_rtti_x64.exe");
    assert_eq!(pe.coverage, Coverage::Complete(Source::BaseRelocations));
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
    let (got, damage) = interpret_rebase(&ops, &[0x1000, 0x4000], 8);
    assert_eq!(got, vec![0x4010, 0x4018, 0x4020, 0x4030, 0x4040]);
    assert_eq!(damage, None);
}

/// A shared object's table of its own exported function is an absolute
/// relocation against the preemptible symbol (`R_X86_64_64`), listed beside the
/// `RELATIVE` words and the `.got` slot of a data symbol.
#[test]
fn a_shared_objects_absolute_relocations_are_data_pointers() {
    let got = pointers("dataptrs_so_x86_64.so");
    assert_eq!(got.coverage, Coverage::Complete(Source::DynamicRelocations));
    assert_eq!(
        got.pointers,
        vec![
            DataPointer { slot: 0x3ff8, target: 0x4030, width: 8 },
            DataPointer { slot: 0x4020, target: 0x2000, width: 8 },
            DataPointer { slot: 0x4030, target: 0x1000, width: 8 },
            DataPointer { slot: 0x4038, target: 0x1010, width: 8 },
        ]
    );
}

fn word(slot: u64, target: u64, width: u8) -> DataPointer {
    DataPointer { slot, target, width }
}

/// GH #1502: RISC-V has no `GLOB_DAT`, so its GOT slot and its table entry for
/// an exported symbol are both absolute words, beside the `RELATIVE` slot and
/// the `JUMP_SLOT` (`llvm-readelf -r`, `dataptrs_arch.provenance`).
#[test]
fn risc_v_dynamic_relocations_are_data_pointers() {
    let rv64 = pointers("dataptrs_arch_riscv64.so");
    assert_eq!(rv64.coverage, Coverage::Complete(Source::DynamicRelocations));
    assert_eq!(rv64.reason, None);
    assert_eq!(
        rv64.pointers,
        vec![word(0x2558, 0x3560, 8), word(0x3568, 0x13d0, 8), word(0x3570, 0x13ea, 8), word(0x3588, 0x13ea, 8)]
    );
    let rv32 = pointers("dataptrs_arch_riscv32.so");
    assert_eq!(rv32.coverage, Coverage::Complete(Source::DynamicRelocations));
    assert_eq!(
        rv32.pointers,
        vec![word(0x2394, 0x3398, 4), word(0x339c, 0x1280, 4), word(0x33a0, 0x129a, 4), word(0x33ac, 0x129a, 4)]
    );
}

/// PowerPC's `JMP_SLOT` stores a word only under the 32-bit secure PLT
/// (`DT_PPC_GOT`) and ELFv2; a BSS-PLT slot is patched code and an ELFv1 slot a
/// copied function descriptor, so neither is listed. An ELFv1 function pointer
/// holds its descriptor, whose first word is a pointer to the code.
#[test]
fn powerpc_dynamic_relocations_are_data_pointers() {
    let secure = pointers("dataptrs_arch_ppc32.so");
    assert_eq!(secure.coverage, Coverage::Complete(Source::DynamicRelocations));
    assert_eq!(
        secure.pointers,
        vec![word(0x304d4, 0x304d8, 4), word(0x304dc, 0x10338, 4), word(0x304e0, 0x1035c, 4), word(0x304e4, 0x1035c, 4)]
    );
    let bss = pointers("dataptrs_arch_ppc32_bssplt.so");
    assert_eq!(bss.coverage, Coverage::Complete(Source::DynamicRelocations));
    assert_eq!(bss.pointers, vec![word(0x1f6c, 0x2010, 4), word(0x2014, 0x230, 4), word(0x2018, 0x25c, 4)]);
    let v2 = pointers("dataptrs_arch_ppc64le.so");
    assert_eq!(
        v2.pointers,
        vec![word(0x20690, 0x30698, 8), word(0x306a0, 0x10480, 8), word(0x306a8, 0x104a0, 8), word(0x306c0, 0x104a0, 8)]
    );
    let v1 = pointers("dataptrs_arch_ppc64_elfv1.so");
    assert_eq!(v1.coverage, Coverage::Complete(Source::DynamicRelocations));
    for listed in [word(0x1eb8, 0x43c, 8), word(0x2038, 0x1ea0, 8), word(0x2040, 0x1eb8, 8)] {
        assert!(v1.pointers.contains(&listed), "{listed:x?} in {:x?}", v1.pointers);
    }
    assert!(!v1.pointers.iter().any(|p| p.slot == 0x2018), "the descriptor-copy PLT slot: {:x?}", v1.pointers);
}

/// `-z pack-relative-relocs` moves the `RELATIVE` relocations to `SHT_RELR`,
/// whose slots hold their targets, listed beside the `.rela.dyn` words.
#[test]
fn relr_slots_are_data_pointers() {
    let got = pointers("relr_so_x86_64.so");
    assert_eq!(got.coverage, Coverage::Complete(Source::DynamicRelocations));
    assert_eq!(
        got.pointers,
        vec![word(0x3ff8, 0x4030, 8), word(0x4020, 0x2000, 8), word(0x4030, 0x1000, 8), word(0x4038, 0x1010, 8)]
    );
}

/// Metadata kuna does not read is `Unsupported`, never `None`: a MIPS image
/// relocates its local GOT without relocations, and a relocatable object's
/// relocations name sections.
#[test]
fn unread_metadata_is_unsupported() {
    for name in ["mipsstubs_noreturn_el.so", "armfloatargs_calls_arm-O0.o"] {
        let got = pointers(name);
        assert_eq!(got.coverage, Coverage::Unsupported, "{name}");
        assert!(got.reason.is_some() && got.pointers.is_empty(), "{name}: {got:?}");
    }
    assert!(pointers("mipsstubs_noreturn_el.so").reason.unwrap().starts_with("MIPS"));
}

/// Metadata that exists and lists no pointer is a complete, empty answer: a
/// chained-fixup image that only binds, and a slid image whose `LC_DYLD_INFO`
/// rebases nothing (`llvm-objdump --macho --dyld-info` lists no rebase in
/// either).
#[test]
fn metadata_that_lists_nothing_is_complete() {
    let binds = pointers("macho_bind_only_arm64");
    assert_eq!(binds.coverage, Coverage::Complete(Source::ChainedFixups));
    assert!(binds.pointers.is_empty() && binds.reason.is_none());
    let classic = pointers("macho_import_slots");
    assert_eq!(classic.coverage, Coverage::Complete(Source::RebaseOpcodes));
    assert!(classic.pointers.is_empty());
}

/// GH #1502: GNU ld stores a pointer at an unaligned offset under PowerPC's
/// `UADDR` relocations, the 32-bit one against a local section symbol whose
/// value binutils folds into the addend; Android's `SHT_ANDROID_RELR` packs
/// `RELATIVE` slots as `SHT_RELR` does (`dataptrs_arch.provenance`).
#[test]
fn unaligned_and_android_packed_relocations_are_data_pointers() {
    let ppc32 = pointers("dataptrs_arch_ppc32_uaddr.so");
    assert_eq!(ppc32.coverage, Coverage::Complete(Source::DynamicRelocations));
    assert_eq!(ppc32.pointers, vec![word(0x2001, 0x190, 4), word(0x2008, 0x190, 4)]);
    let ppc64 = pointers("dataptrs_arch_ppc64_uaddr.so");
    assert_eq!(ppc64.pointers, vec![word(0x1ee8, 0x278, 8), word(0x2001, 0x1ee8, 8), word(0x2010, 0x1ee8, 8)]);
    let android = pointers("dataptrs_arch_x86_64_androidrelr.so");
    assert_eq!(android.coverage, Coverage::Complete(Source::DynamicRelocations));
    assert_eq!(
        android.pointers,
        vec![word(0x2620, 0x3648, 8), word(0x2640, 0x14a0, 8), word(0x3650, 0x1490, 8), word(0x3658, 0x14a0, 8)]
    );
}

fn patched(name: &str, patch: impl FnOnce(&mut Vec<u8>)) -> DataPointers {
    let mut bytes = load(name);
    patch(&mut bytes);
    let file = object::File::parse(&bytes[..]).expect("parse");
    data_pointers(&file, &bytes)
}

fn u32_at(bytes: &[u8], at: usize) -> usize {
    u32::from_le_bytes(bytes[at..at + 4].try_into().unwrap()) as usize
}

/// A relocation section whose size is not a whole number of entries is one
/// the relocation walk skips, so the list is partial. A PE base-relocation
/// directory of size 0 is none, and an image base at the top of the address
/// space wraps its slots as it wraps its sections' addresses instead of
/// overflowing.
#[test]
fn ragged_and_empty_tables_are_never_complete() {
    let ragged = patched("dataptrs_so_x86_64.so", |bytes| {
        let header = u64::from_le_bytes(bytes[0x28..0x30].try_into().unwrap()) as usize + 4 * 0x40;
        assert_eq!(u32_at(bytes, header + 0x28), 2, "section 4 is .rela.dyn, linked to .dynsym");
        let size = u64::from_le_bytes(bytes[header + 0x20..header + 0x28].try_into().unwrap());
        bytes[header + 0x20..header + 0x28].copy_from_slice(&(size + 1).to_le_bytes());
    });
    assert_eq!(ragged.coverage, Coverage::Partial(Source::DynamicRelocations));
    assert!(ragged.reason.as_deref().unwrap_or_default().contains("is not a whole number of entries"), "{ragged:?}");
    let pe_directory = |bytes: &[u8]| {
        let optional = u32_at(bytes, 0x3c) + 24;
        (optional, optional + if bytes[optional..optional + 2] == [0x0b, 0x02] { 112 } else { 96 })
    };
    let empty = patched("dataptrs_textptr_i386.exe", |bytes| {
        let (_, directories) = pe_directory(bytes);
        bytes[directories + 5 * 8 + 4..directories + 5 * 8 + 8].copy_from_slice(&0u32.to_le_bytes());
    });
    assert_eq!(empty.coverage, Coverage::None, "{empty:?}");
    let top = patched("msvc_rtti_x64.exe", |bytes| {
        let (optional, _) = pe_directory(bytes);
        bytes[optional + 24..optional + 32].copy_from_slice(&0xffff_ffff_ffff_f000u64.to_le_bytes());
    });
    assert_eq!(top.coverage, Coverage::Complete(Source::BaseRelocations), "{:?}", top.reason);
}

/// A chained-fixup image whose chains use a pointer format the walk does not
/// decode lists what it read, as partial, never as complete.
#[test]
fn an_unread_chain_makes_the_list_partial() {
    let got = patched("macho_ptrslots_arm64", |bytes| {
        let mut at = 32;
        for _ in 0..u32_at(bytes, 16) {
            if u32_at(bytes, at) == 0x8000_0034 {
                let blob = u32_at(bytes, at + 8);
                let starts = blob + u32_at(bytes, blob + 4);
                let first = (0..u32_at(bytes, starts)).map(|i| u32_at(bytes, starts + 4 + 4 * i)).find(|&o| o != 0);
                let format = starts + first.expect("a segment with chains") + 6;
                bytes[format..format + 2].copy_from_slice(&13u16.to_le_bytes());
                return;
            }
            at += u32_at(bytes, at + 4);
        }
        panic!("no LC_DYLD_CHAINED_FIXUPS");
    });
    assert_eq!(got.coverage, Coverage::Partial(Source::ChainedFixups));
    let why = "the chained fixups were not all read: chained pointer format 13 is not decoded";
    assert_eq!(got.reason.as_deref(), Some(why));
}
