//! Section-table tolerance: a usable table is returned byte for byte, an
//! unusable one is cleared, and the recovered image keeps its program headers.

use super::*;
use object::read::{Object, ObjectSegment};

/// A minimal well-formed ELF32 LSB executable: header, one `PF_X` `PT_LOAD`
/// program header covering the whole file, and a real two-entry section table
/// (the null section plus a `.shstrtab`) so `e_shoff` is genuinely in use and
/// `object` accepts the file as it stands.
fn elf32(mutate: impl FnOnce(&mut Vec<u8>)) -> Vec<u8> {
    const PHOFF: usize = 52;
    const SHOFF: usize = 84;
    const STROFF: usize = 164;
    const STRTAB: &[u8] = b"\0.shstrtab\0";

    let mut b = vec![0u8; STROFF];
    b[..4].copy_from_slice(b"\x7fELF");
    b[4] = 1; // ELFCLASS32
    b[5] = 1; // ELFDATA2LSB
    b[6] = 1; // EV_CURRENT
    b[16..18].copy_from_slice(&2u16.to_le_bytes()); // e_type = ET_EXEC
    b[18..20].copy_from_slice(&3u16.to_le_bytes()); // e_machine = EM_386
    b[20..24].copy_from_slice(&1u32.to_le_bytes()); // e_version
    b[24..28].copy_from_slice(&0x8048054u32.to_le_bytes()); // e_entry
    b[28..32].copy_from_slice(&(PHOFF as u32).to_le_bytes()); // e_phoff
    b[32..36].copy_from_slice(&(SHOFF as u32).to_le_bytes()); // e_shoff
    b[40..42].copy_from_slice(&52u16.to_le_bytes()); // e_ehsize
    b[42..44].copy_from_slice(&32u16.to_le_bytes()); // e_phentsize
    b[44..46].copy_from_slice(&1u16.to_le_bytes()); // e_phnum
    b[46..48].copy_from_slice(&40u16.to_le_bytes()); // e_shentsize
    b[48..50].copy_from_slice(&2u16.to_le_bytes()); // e_shnum
    b[50..52].copy_from_slice(&1u16.to_le_bytes()); // e_shstrndx

    b[PHOFF..PHOFF + 4].copy_from_slice(&1u32.to_le_bytes()); // p_type = PT_LOAD
    b[PHOFF + 8..PHOFF + 12].copy_from_slice(&0x8048000u32.to_le_bytes()); // p_vaddr
    b[PHOFF + 12..PHOFF + 16].copy_from_slice(&0x8048000u32.to_le_bytes()); // p_paddr
    b[PHOFF + 16..PHOFF + 20].copy_from_slice(&(STROFF as u32).to_le_bytes()); // p_filesz
    b[PHOFF + 20..PHOFF + 24].copy_from_slice(&(STROFF as u32).to_le_bytes()); // p_memsz
    b[PHOFF + 24..PHOFF + 28].copy_from_slice(&5u32.to_le_bytes()); // p_flags = R|X
    b[PHOFF + 28..PHOFF + 32].copy_from_slice(&0x1000u32.to_le_bytes()); // p_align

    // Section 0 is the all-zero null header; section 1 is the `.shstrtab`.
    let s1 = SHOFF + 40;
    b[s1..s1 + 4].copy_from_slice(&1u32.to_le_bytes()); // sh_name -> ".shstrtab"
    b[s1 + 4..s1 + 8].copy_from_slice(&3u32.to_le_bytes()); // sh_type = SHT_STRTAB
    b[s1 + 16..s1 + 20].copy_from_slice(&(STROFF as u32).to_le_bytes()); // sh_offset
    b[s1 + 20..s1 + 24].copy_from_slice(&(STRTAB.len() as u32).to_le_bytes()); // sh_size
    b.extend_from_slice(STRTAB);

    mutate(&mut b);
    b
}

#[test]
fn usable_section_table_is_returned_verbatim() {
    let bytes = elf32(|_| {});
    assert!(object::File::parse(bytes.as_slice()).is_ok(), "fixture must parse as-is");
    let (out, note) = tolerate_unusable_section_table(bytes.clone());
    assert_eq!(out, bytes, "a parseable image must not be rewritten");
    assert!(note.is_none(), "no diagnostic for a healthy image, got {note:?}");
}

#[test]
fn no_section_table_at_all_is_returned_verbatim() {
    let bytes = elf32(|b| {
        b[32..36].copy_from_slice(&0u32.to_le_bytes()); // e_shoff = 0
        b[48..50].copy_from_slice(&0u16.to_le_bytes()); // e_shnum = 0
    });
    let (out, note) = tolerate_unusable_section_table(bytes.clone());
    assert_eq!(out, bytes);
    assert!(note.is_none());
}

/// The filed case: `e_shoff`/`e_shnum`/`e_shstrndx` are garbage while the
/// program headers are intact.
#[test]
fn out_of_range_section_table_is_cleared_and_program_headers_survive() {
    let bytes = elf32(|b| {
        b[32..36].copy_from_slice(&57005u32.to_le_bytes()); // e_shoff = 0xDEAD
        b[48..50].copy_from_slice(&57007u16.to_le_bytes()); // e_shnum
        b[50..52].copy_from_slice(&47806u16.to_le_bytes()); // e_shstrndx
    });
    assert!(object::File::parse(bytes.as_slice()).is_err(), "fixture must be rejected today");

    let (out, note) = tolerate_unusable_section_table(bytes);
    let note = note.expect("the repair must report what it dropped");
    assert!(note.contains("past the end"), "note should name the cause: {note}");
    assert!(note.contains("0x8048054"), "note should report the surviving entry: {note}");

    let file = object::File::parse(out.as_slice()).expect("repaired image parses");
    assert_eq!(file.entry(), 0x8048054);
    let segs: Vec<_> = file.segments().map(|s| s.address()).collect();
    assert_eq!(segs, vec![0x8048000], "the PT_LOAD map must be intact");
    assert_eq!(file.sections().count(), 0, "the unusable table is gone, not guessed at");
}

/// A section table whose name-string index was zeroed in place: `object` refuses
/// the whole file over a name table nothing in the decompiler reads.
#[test]
fn zeroed_shstrndx_is_cleared() {
    let bytes = elf32(|b| b[50..52].copy_from_slice(&0u16.to_le_bytes()));
    assert!(object::File::parse(bytes.as_slice()).is_err());
    let (out, note) = tolerate_unusable_section_table(bytes);
    assert!(note.expect("diagnosed").contains("e_shstrndx is 0"));
    assert!(object::File::parse(out.as_slice()).is_ok());
}

#[test]
fn wrong_entry_size_is_cleared() {
    let bytes = elf32(|b| b[46..48].copy_from_slice(&41u16.to_le_bytes()));
    assert!(object::File::parse(bytes.as_slice()).is_err());
    let (out, note) = tolerate_unusable_section_table(bytes);
    assert!(note.expect("diagnosed").contains("e_shentsize is 41"));
    assert!(object::File::parse(out.as_slice()).is_ok());
}

#[test]
fn out_of_range_shstrndx_is_cleared() {
    let bytes = elf32(|b| b[50..52].copy_from_slice(&9u16.to_le_bytes()));
    assert!(object::File::parse(bytes.as_slice()).is_err());
    let (out, note) = tolerate_unusable_section_table(bytes);
    assert!(note.expect("diagnosed").contains("e_shstrndx is 9"));
    assert!(object::File::parse(out.as_slice()).is_ok());
}

/// A non-ELF and a too-short file are both left exactly as they came in -- the
/// repair must never invent an ELF header to write into.
#[test]
fn non_elf_input_is_untouched() {
    for bytes in [b"MZ\x90\x00".to_vec(), vec![0u8; 8], b"\x7fELF".to_vec()] {
        let (out, note) = tolerate_unusable_section_table(bytes.clone());
        assert_eq!(out, bytes);
        assert!(note.is_none());
    }
}

/// Corruption the clear does not fix (here: a program header table past EOF)
/// must leave the original bytes so the caller reports `object`'s real error
/// instead of one about the section table.
#[test]
fn unrepairable_image_keeps_its_original_bytes() {
    let bytes = elf32(|b| {
        b[32..36].copy_from_slice(&57005u32.to_le_bytes()); // unusable section table
        b[28..32].copy_from_slice(&0xffff_0000u32.to_le_bytes()); // e_phoff past EOF
    });
    let (out, note) = tolerate_unusable_section_table(bytes.clone());
    assert_eq!(out, bytes, "a repair that does not make the file parse is discarded");
    assert!(note.is_none());
}
