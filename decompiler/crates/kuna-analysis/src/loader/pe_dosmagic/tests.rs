//! DOS-magic tolerance: a healthy image and a non-PE file are returned byte for
//! byte, a trashed `e_magic` in front of a real PE signature is rewritten to `MZ`
//! and parses, and nothing else in the image moves.

use super::*;
use object::read::Object;

const LFANEW: usize = 0x80;

/// A minimal PE32 with a 0x80-byte DOS stub: COFF header, a 224-byte optional
/// header with 16 empty data directories, one `.text` section holding `ret`.
fn pe() -> Vec<u8> {
    let mut b = vec![0u8; LFANEW];
    b[..2].copy_from_slice(b"MZ");
    b[0x3c..0x40].copy_from_slice(&(LFANEW as u32).to_le_bytes());
    b.extend_from_slice(b"PE\0\0");
    b.extend_from_slice(&0x14cu16.to_le_bytes());
    b.extend_from_slice(&1u16.to_le_bytes());
    b.extend_from_slice(&[0u8; 12]);
    b.extend_from_slice(&224u16.to_le_bytes());
    b.extend_from_slice(&0x0102u16.to_le_bytes());
    let mut o = Vec::new();
    o.extend_from_slice(&0x10bu16.to_le_bytes());
    o.extend_from_slice(&[14, 0]);
    o.extend_from_slice(&[0u8; 12]);
    o.extend_from_slice(&0x1000u32.to_le_bytes());
    o.extend_from_slice(&0x1000u32.to_le_bytes());
    o.extend_from_slice(&0u32.to_le_bytes());
    o.extend_from_slice(&0x40_0000u32.to_le_bytes());
    o.extend_from_slice(&0x1000u32.to_le_bytes());
    o.extend_from_slice(&0x200u32.to_le_bytes());
    o.extend_from_slice(&[0u8; 16]);
    o.extend_from_slice(&0x2000u32.to_le_bytes());
    o.extend_from_slice(&0x200u32.to_le_bytes());
    o.extend_from_slice(&0u32.to_le_bytes());
    o.extend_from_slice(&3u16.to_le_bytes());
    o.extend_from_slice(&0u16.to_le_bytes());
    o.extend_from_slice(&[0u8; 16]);
    o.extend_from_slice(&0u32.to_le_bytes());
    o.extend_from_slice(&16u32.to_le_bytes());
    assert_eq!(o.len(), 96);
    o.resize(224, 0);
    b.extend_from_slice(&o);
    b.extend_from_slice(b".text\0\0\0");
    for v in [0x10u32, 0x1000, 0x200, 0x200, 0, 0] {
        b.extend_from_slice(&v.to_le_bytes());
    }
    b.extend_from_slice(&[0u8; 4]);
    b.extend_from_slice(&0x6000_0020u32.to_le_bytes());
    b.resize(0x400, 0);
    b[0x200] = 0xc3;
    b
}

#[test]
fn a_healthy_image_is_returned_verbatim() {
    let bytes = pe();
    assert!(object::File::parse(bytes.as_slice()).is_ok(), "fixture must parse as-is");
    let (out, note) = tolerate_corrupt_dos_magic(bytes.clone());
    assert_eq!(out, bytes);
    assert!(note.is_none(), "{note:?}");
}

/// The filed case: the first byte is `0x15` instead of `M`.
#[test]
fn a_trashed_magic_in_front_of_a_pe_signature_is_repaired() {
    let mut bytes = pe();
    bytes[0] = 0x15;
    assert!(object::File::parse(bytes.as_slice()).is_err(), "the trashed image must not parse");
    let (out, note) = tolerate_corrupt_dos_magic(bytes.clone());
    assert_eq!(&out[..2], b"MZ");
    assert_eq!(out[2..], bytes[2..], "only e_magic may change");
    let note = note.expect("the repair must report what it did");
    assert!(note.contains("e_magic is 0x5a15"), "{note}");
    assert!(note.contains("e_lfanew 0x80"), "{note}");
    let file = object::File::parse(out.as_slice()).expect("the repaired copy parses");
    assert_eq!(file.entry(), 0x40_1000);
    assert_eq!(file.sections().count(), 1);
}

#[test]
fn a_file_with_no_pe_signature_at_e_lfanew_is_left_alone() {
    let mut bytes = pe();
    bytes[0] = 0x15;
    bytes[LFANEW] = b'X';
    let (out, note) = tolerate_corrupt_dos_magic(bytes.clone());
    assert_eq!(out, bytes);
    assert!(note.is_none(), "{note:?}");

    let mut wild = pe();
    wild[0] = 0x15;
    wild[0x3c..0x40].copy_from_slice(&u32::MAX.to_le_bytes());
    assert!(tolerate_corrupt_dos_magic(wild).1.is_none(), "an out-of-file e_lfanew");

    assert!(tolerate_corrupt_dos_magic(vec![0x15; 0x3f]).1.is_none(), "shorter than a DOS header");
}

#[test]
fn another_recognized_format_is_never_rewritten() {
    let mut elf = vec![0u8; 0x100];
    elf[..4].copy_from_slice(b"\x7fELF");
    elf[4] = 1;
    elf[5] = 1;
    elf[0x3c..0x40].copy_from_slice(&0x80u32.to_le_bytes());
    elf[0x80..0x84].copy_from_slice(b"PE\0\0");
    let (out, note) = tolerate_corrupt_dos_magic(elf.clone());
    assert_eq!(out, elf);
    assert!(note.is_none(), "{note:?}");
}
