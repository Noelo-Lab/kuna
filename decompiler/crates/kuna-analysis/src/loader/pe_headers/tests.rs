//! The header page's extent: what a well-formed image publishes, and the two
//! clamps that keep a malformed `SizeOfHeaders` from shadowing a real section.

use super::*;

const DOS: usize = 0x40;
const DIRS: usize = 16;
const PE32_FIXED: usize = 96;
const FILE_ALIGN: u32 = 0x200;
const IMAGE_BASE: u64 = 0x40_0000;

/// A minimal PE32: DOS stub, COFF header, a full optional header, and one
/// `.text` section at `text_rva`. `size_of_headers` is written verbatim so a
/// test can declare one that is a lie.
fn pe(size_of_headers: u32, text_rva: u32) -> Vec<u8> {
    let optsize = PE32_FIXED + DIRS * 8;

    let mut b = vec![0u8; DOS];
    b[..2].copy_from_slice(b"MZ");
    b[0x3c..0x40].copy_from_slice(&(DOS as u32).to_le_bytes());
    b.extend_from_slice(b"PE\0\0");
    b.extend_from_slice(&0x14cu16.to_le_bytes()); // Machine = i386
    b.extend_from_slice(&1u16.to_le_bytes()); // NumberOfSections
    b.extend_from_slice(&[0u8; 12]);
    b.extend_from_slice(&(optsize as u16).to_le_bytes());
    b.extend_from_slice(&0x010fu16.to_le_bytes());

    let mut o: Vec<u8> = Vec::new();
    o.extend_from_slice(&0x10bu16.to_le_bytes()); // PE32
    o.extend_from_slice(&[14, 0]);
    o.extend_from_slice(&0x10u32.to_le_bytes()); // SizeOfCode
    o.extend_from_slice(&[0u8; 8]);
    o.extend_from_slice(&text_rva.to_le_bytes()); // AddressOfEntryPoint
    o.extend_from_slice(&text_rva.to_le_bytes()); // BaseOfCode
    o.extend_from_slice(&0u32.to_le_bytes()); // BaseOfData
    o.extend_from_slice(&(IMAGE_BASE as u32).to_le_bytes());
    o.extend_from_slice(&0x1000u32.to_le_bytes()); // SectionAlignment
    o.extend_from_slice(&FILE_ALIGN.to_le_bytes());
    o.extend_from_slice(&[0u8; 16]);
    o.extend_from_slice(&0x2000u32.to_le_bytes()); // SizeOfImage
    o.extend_from_slice(&size_of_headers.to_le_bytes());
    o.extend_from_slice(&0u32.to_le_bytes()); // CheckSum
    o.extend_from_slice(&3u16.to_le_bytes()); // Subsystem
    o.extend_from_slice(&0u16.to_le_bytes());
    o.extend_from_slice(&[0u8; 16]); // stack/heap reserve + commit
    o.extend_from_slice(&0u32.to_le_bytes()); // LoaderFlags
    o.extend_from_slice(&(DIRS as u32).to_le_bytes());
    assert_eq!(o.len(), PE32_FIXED);
    o.extend_from_slice(&vec![0u8; DIRS * 8]);
    b.extend_from_slice(&o);

    let mut s: Vec<u8> = b".text\0\0\0".to_vec();
    s.extend_from_slice(&0x10u32.to_le_bytes()); // VirtualSize
    s.extend_from_slice(&text_rva.to_le_bytes()); // VirtualAddress
    s.extend_from_slice(&FILE_ALIGN.to_le_bytes()); // SizeOfRawData
    s.extend_from_slice(&FILE_ALIGN.to_le_bytes()); // PointerToRawData
    s.extend_from_slice(&[0u8; 12]);
    s.extend_from_slice(&0x6000_0020u32.to_le_bytes()); // CODE | EXECUTE | READ
    b.extend_from_slice(&s);

    // Long enough that the file-length clamp never binds unless a test asks
    // for it by truncating.
    b.resize(0x4000, 0);
    b[FILE_ALIGN as usize] = 0xc3; // ret, at the entry
    b
}

fn region_of(bytes: &[u8]) -> Option<HeaderRegion> {
    let file = object::File::parse(bytes).expect("fixture parses");
    header_region(&file, bytes)
}

#[test]
fn well_formed_pe_publishes_size_of_headers_at_image_base() {
    let region = region_of(&pe(FILE_ALIGN, 0x1000)).expect("a PE has a header page");
    assert_eq!(region.vma, IMAGE_BASE);
    assert_eq!(region.len, FILE_ALIGN as usize);
}

/// The guard the witness needs: a `SizeOfHeaders` reaching past the first
/// section must not shadow it.
#[test]
fn size_of_headers_is_clamped_to_the_first_section() {
    let region = region_of(&pe(0x4000, 0x1000)).expect("a PE has a header page");
    assert_eq!(region.vma, IMAGE_BASE);
    assert_eq!(region.len, 0x1000, "clamped to the first section's RVA");
}

/// A section claiming RVA 0 leaves no room at all, and no region is published
/// rather than an empty one.
#[test]
fn a_section_at_rva_zero_leaves_no_header_page() {
    assert!(region_of(&pe(FILE_ALIGN, 0)).is_none());
}

#[test]
fn size_of_headers_is_clamped_to_the_file_length() {
    let mut bytes = pe(FILE_ALIGN, 0x1000);
    bytes.truncate(0x180);
    let file = object::File::parse(bytes.as_slice()).expect("a truncated image still parses");
    let region = header_region(&file, &bytes).expect("a PE has a header page");
    assert_eq!(region.len, 0x180, "only file-backed bytes are mapped");
}

/// Every other format keeps a section/segment-derived map: the `ObjectFormat`
/// default answers `None`, and so does this function on a non-PE input.
#[test]
fn a_non_pe_has_no_header_page() {
    let elf = std::fs::read(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/cet_pie_x86_64"
    ))
    .expect("vendored ELF fixture");
    assert!(object::File::parse(elf.as_slice()).is_ok());
    assert!(region_of(&elf).is_none());
}
