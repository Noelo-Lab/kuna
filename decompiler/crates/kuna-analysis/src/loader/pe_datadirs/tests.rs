//! Data-directory tolerance: a count that fits is returned byte for byte, a
//! count that cannot fit is clamped to the directories that are really there,
//! and the clamped image keeps the real import table rather than a fabricated one.

use super::*;
use object::read::pe::{PeFile32, PeFile64};
use object::read::Object;

/// The count the reported Invius-packed image carried at file offset 0xf4.
const TRASHED: u32 = 1_531_532_893;

const DOS: usize = 0x40;
const OPT: usize = DOS + 4 + 20;
const DIRS: usize = 16;
const FILE_ALIGN: usize = 0x200;
const ENTRY_RVA: u32 = 0x1000;
/// Planted in the IMPORT directory slot so a clamp that fabricates an empty
/// table instead of keeping the real one is visible.
const IMPORT_RVA: u32 = 0x1234;

/// A minimal well-formed PE32 (`plus == false`) or PE32+ image: DOS stub, COFF
/// header, a full optional header with all 16 data directories present, one
/// `.text` section, and nothing else. `object` accepts it as it stands.
fn pe(plus: bool, mutate: impl FnOnce(&mut Vec<u8>)) -> Vec<u8> {
    let fixed = if plus { PE32PLUS_FIXED } else { PE32_FIXED };
    let optsize = fixed + DIRS * 8;
    let headers = OPT + optsize + 40;

    let mut b = vec![0u8; DOS];
    b[..2].copy_from_slice(b"MZ");
    b[0x3c..0x40].copy_from_slice(&(DOS as u32).to_le_bytes());
    b.extend_from_slice(b"PE\0\0");

    let machine: u16 = if plus { 0x8664 } else { 0x14c };
    b.extend_from_slice(&machine.to_le_bytes());
    b.extend_from_slice(&1u16.to_le_bytes()); // NumberOfSections
    b.extend_from_slice(&[0u8; 12]); // timestamp, symbol table, symbol count
    b.extend_from_slice(&(optsize as u16).to_le_bytes());
    b.extend_from_slice(&0x0102u16.to_le_bytes()); // EXECUTABLE_IMAGE | 32BIT_MACHINE

    let mut o: Vec<u8> = Vec::new();
    let magic: u16 = if plus { 0x20b } else { 0x10b };
    o.extend_from_slice(&magic.to_le_bytes());
    o.extend_from_slice(&[14, 0]); // linker version
    o.extend_from_slice(&0x10u32.to_le_bytes()); // SizeOfCode
    o.extend_from_slice(&[0u8; 8]); // SizeOfInitializedData, SizeOfUninitializedData
    o.extend_from_slice(&ENTRY_RVA.to_le_bytes());
    o.extend_from_slice(&ENTRY_RVA.to_le_bytes()); // BaseOfCode
    if plus {
        o.extend_from_slice(&0x1_4000_0000u64.to_le_bytes()); // ImageBase
    } else {
        o.extend_from_slice(&0u32.to_le_bytes()); // BaseOfData
        o.extend_from_slice(&0x40_0000u32.to_le_bytes()); // ImageBase
    }
    o.extend_from_slice(&0x1000u32.to_le_bytes()); // SectionAlignment
    o.extend_from_slice(&(FILE_ALIGN as u32).to_le_bytes()); // FileAlignment
    o.extend_from_slice(&[0u8; 16]); // OS/image/subsystem versions, Win32VersionValue
    o.extend_from_slice(&0x2000u32.to_le_bytes()); // SizeOfImage
    o.extend_from_slice(&(FILE_ALIGN as u32).to_le_bytes()); // SizeOfHeaders
    o.extend_from_slice(&0u32.to_le_bytes()); // CheckSum
    o.extend_from_slice(&3u16.to_le_bytes()); // Subsystem = CONSOLE
    o.extend_from_slice(&0u16.to_le_bytes()); // DllCharacteristics
    if plus {
        o.extend_from_slice(&[0u8; 32]); // stack/heap reserve + commit, u64 each
    } else {
        o.extend_from_slice(&[0u8; 16]); // stack/heap reserve + commit, u32 each
    }
    o.extend_from_slice(&0u32.to_le_bytes()); // LoaderFlags
    o.extend_from_slice(&(DIRS as u32).to_le_bytes()); // NumberOfRvaAndSizes
    assert_eq!(o.len(), fixed, "optional header fixed part");
    for i in 0..DIRS {
        let rva = if i == 1 { IMPORT_RVA } else { 0 };
        o.extend_from_slice(&rva.to_le_bytes());
        o.extend_from_slice(&0u32.to_le_bytes());
    }
    b.extend_from_slice(&o);

    let mut s: Vec<u8> = b".text\0\0\0".to_vec();
    s.extend_from_slice(&0x10u32.to_le_bytes()); // VirtualSize
    s.extend_from_slice(&ENTRY_RVA.to_le_bytes()); // VirtualAddress
    s.extend_from_slice(&(FILE_ALIGN as u32).to_le_bytes()); // SizeOfRawData
    s.extend_from_slice(&(FILE_ALIGN as u32).to_le_bytes()); // PointerToRawData
    s.extend_from_slice(&[0u8; 12]); // relocations, line numbers
    s.extend_from_slice(&0x6000_0020u32.to_le_bytes()); // CODE | EXECUTE | READ
    b.extend_from_slice(&s);
    assert_eq!(b.len(), headers);

    b.resize(FILE_ALIGN * 2, 0);
    b[FILE_ALIGN] = 0xc3; // ret, at the entry point

    mutate(&mut b);
    b
}

/// File offset of `NumberOfRvaAndSizes` in the images `pe` builds.
fn nrva_off(plus: bool) -> usize {
    OPT + if plus { PE32PLUS_FIXED } else { PE32_FIXED } - 4
}

fn set_nrva(b: &mut [u8], plus: bool, value: u32) {
    let off = nrva_off(plus);
    b[off..off + 4].copy_from_slice(&value.to_le_bytes());
}

#[test]
fn count_that_fits_is_returned_verbatim() {
    for plus in [false, true] {
        let bytes = pe(plus, |_| {});
        assert!(
            object::File::parse(bytes.as_slice()).is_ok(),
            "fixture must parse as-is (plus={plus})"
        );
        let (out, note) = tolerate_oversized_data_directories(bytes.clone());
        assert_eq!(out, bytes, "a parseable image must not be rewritten");
        assert!(note.is_none(), "no diagnostic for a healthy image, got {note:?}");
    }
}

/// The filed case: `NumberOfRvaAndSizes` is trashed while the 16 real
/// directories are physically present.
#[test]
fn trashed_count_is_clamped_and_the_real_import_directory_survives() {
    let bytes = pe(false, |b| set_nrva(b, false, TRASHED));
    assert!(
        object::File::parse(bytes.as_slice()).is_err(),
        "fixture must be rejected today"
    );

    let (out, note) = tolerate_oversized_data_directories(bytes);
    let note = note.expect("the clamp must report what it did");
    assert!(note.contains(&TRASHED.to_string()), "note names the bad count: {note}");
    assert!(note.contains("clamped to 16"), "note names the clamp: {note}");
    assert!(note.contains("0x401000"), "note reports the surviving entry: {note}");

    let file = PeFile32::parse(out.as_slice()).expect("clamped image parses");
    assert_eq!(file.entry(), 0x40_0000 + u64::from(ENTRY_RVA));
    assert_eq!(file.sections().count(), 1, "the section table is untouched");
    let dirs = file.data_directories();
    assert_eq!(dirs.len(), 16, "the directories that fit are all kept");
    assert_eq!(
        dirs.get(1).map(|d| d.virtual_address.get(object::LittleEndian)),
        Some(IMPORT_RVA),
        "the real import directory must be read, not fabricated"
    );
}

#[test]
fn trashed_count_is_clamped_on_pe32_plus() {
    let bytes = pe(true, |b| set_nrva(b, true, TRASHED));
    assert!(object::File::parse(bytes.as_slice()).is_err());
    let (out, _) = tolerate_oversized_data_directories(bytes);
    let file = PeFile64::parse(out.as_slice()).expect("clamped image parses");
    assert_eq!(file.data_directories().len(), 16);
}

/// One over the limit is still clamped; the limit itself is not.
#[test]
fn the_clamp_boundary_is_what_the_optional_header_holds() {
    let exact = pe(false, |b| set_nrva(b, false, 16));
    let (out, note) = tolerate_oversized_data_directories(exact.clone());
    assert_eq!(out, exact, "a count that fits exactly is left alone");
    assert!(note.is_none());

    let over = pe(false, |b| set_nrva(b, false, 17));
    let (out, note) = tolerate_oversized_data_directories(over);
    assert!(note.is_some(), "one directory too many is still a rejection");
    assert_eq!(read_u32(&out, nrva_off(false)), 16);
}

/// A non-PE and a too-short file are left exactly as they came in -- the clamp
/// must never invent a PE header to write into.
#[test]
fn non_pe_input_is_untouched() {
    let mut mz_only = vec![0u8; 0x40];
    mz_only[..2].copy_from_slice(b"MZ");
    for bytes in [b"\x7fELF\x02\x01\x01".to_vec(), vec![0u8; 8], mz_only] {
        let (out, note) = tolerate_oversized_data_directories(bytes.clone());
        assert_eq!(out, bytes);
        assert!(note.is_none());
    }
}

/// A ROM-image magic is one `object` rejects outright: the clamp has no legal
/// fixed-part size for it and must not guess.
#[test]
fn unknown_optional_header_magic_is_untouched() {
    let bytes = pe(false, |b| {
        b[OPT..OPT + 2].copy_from_slice(&0x107u16.to_le_bytes());
        set_nrva(b, false, TRASHED);
    });
    let (out, note) = tolerate_oversized_data_directories(bytes.clone());
    assert_eq!(out, bytes);
    assert!(note.is_none());
}

/// Corruption the clamp does not fix keeps the clamp anyway: the count was wrong
/// regardless, and dropping it back would hide the error that is actually left.
#[test]
fn an_unrepairable_image_keeps_the_clamp_and_reports_no_note() {
    let bytes = pe(false, |b| {
        set_nrva(b, false, TRASHED);
        b[DOS + 4 + 2..DOS + 4 + 4].copy_from_slice(&0x2000u16.to_le_bytes()); // sections past EOF
    });
    let (out, note) = tolerate_oversized_data_directories(bytes.clone());
    assert_ne!(out, bytes, "the clamp is kept");
    assert_eq!(read_u32(&out, nrva_off(false)), 16);
    assert!(note.is_none(), "no recovery claim for an image that still fails");
    assert!(object::File::parse(out.as_slice()).is_err());
}
