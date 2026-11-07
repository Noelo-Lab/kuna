//! Loader-proven lazy-binding transfers preserve the caller's return in t7.
use object::{Object, ObjectSection, SectionFlags};

/// Bytes of the one stub form validated: `lw/ld t9,-0x7ff0(gp)`, `move t7,ra`,
/// `jalr t9` and the dynamic-symbol index in the delay slot.
const STUB_BYTES: u64 = 16;

/// Offset of the resolver call (`jalr t9`) within a validated stub.
const RESOLVER_CALL: u64 = 8;

/// Whether `bytes` begin with a complete lazy-binding stub for dynamic symbol
/// `index`, in the image's byte order.
fn is_resolver_stub(bytes: &[u8], little: bool, index: u64) -> bool {
    let Some(raw) = bytes.get(..STUB_BYTES as usize) else { return false };
    let words: Vec<u32> = super::kuna_mipsabi::words(raw, little).collect();
    matches!(words[0], 0x8f998010 | 0xdf998010)
        && matches!(words[1], 0x03e07825 | 0x03e0782d | 0x03e07821)
        && words[2] == 0x0320f809
        && matches!(words[3] & 0xffff0000, 0x24180000 | 0x64180000 | 0x34180000)
        && u64::from(words[3] & 0xffff) == index
        && (words[3] >> 26 == 13 || index < 0x8000)
}

/// The MIPS ELF linkage facts a loader proves once per image.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MipsLinkage {
    /// The ELF32 class: its pointers sign-extend in a 64-bit register.
    pub elf32: bool,
    pub little: bool,
    /// Resolver call site and end of each complete, symbol/GOT-backed
    /// lazy-binding stub.
    pub transfers: Vec<(u64, u64)>,
}

/// The class and byte order of a MIPS ELF, with no stubs: what a relocatable
/// object, which has none, contributes. `None` off MIPS ELF.
pub fn class_linkage(file: &object::File<'_>) -> Option<MipsLinkage> {
    (file.format() == object::BinaryFormat::Elf && super::kuna_mipsabi::is_mips(file.architecture()))
        .then(|| MipsLinkage {
            elf32: !file.is_64(),
            little: file.is_little_endian(),
            ..MipsLinkage::default()
        })
}

/// The linkage of a linked MIPS ELF; `None` off MIPS ELF.
pub fn linkage(file: &object::File<'_>) -> Option<MipsLinkage> {
    let mut linkage = class_linkage(file)?;
    linkage.transfers = scan_transfers(file);
    Some(linkage)
}

/// Resolver instruction and end of each complete, symbol/GOT-backed import stub.
fn scan_transfers(file: &object::File<'_>) -> Vec<(u64, u64)> {
    let Some(section) = file.section_by_name(".MIPS.stubs") else { return Vec::new(); };
    if !matches!(section.flags(), SectionFlags::Elf { sh_flags } if sh_flags & u64::from(object::elf::SHF_EXECINSTR) != 0) {
        return Vec::new();
    }
    let Ok(bytes) = section.data() else { return Vec::new(); };
    super::elf_plt::mips_stub_identities(file).into_iter().filter_map(|(entry, index)| {
        if entry & 3 != 0 { return None; }
        let offset = usize::try_from(entry.checked_sub(section.address())?).ok()?;
        if !is_resolver_stub(bytes.get(offset..)?, file.is_little_endian(), index) {
            return None;
        }
        Some((entry.checked_add(RESOLVER_CALL)?, entry.checked_add(STUB_BYTES)?))
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compiler_stubs_require_linkage_and_executable_complete_bytes() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
        for name in ["flow-mips32r2-le-local-ssp-stripped.so", "flow-mips32r2-be-local-ssp.so",
            "flow-octeon3-n32.so", "flow-octeon3-64.so"] {
            let original = std::fs::read(root.join("tests/fixtures/mips-import-flow").join(name)).unwrap();
            let file = object::File::parse(original.as_slice()).unwrap();
            let expected = scan_transfers(&file);
            assert!(expected.len() >= 4, "{name}: {expected:?}");
            let section = file.section_by_name(".MIPS.stubs").unwrap();
            let little = file.is_little_endian();
            let word = |bytes: &[u8]| -> u64 {
                if little { bytes.iter().rev().fold(0, |v, b| (v << 8) | u64::from(*b)) }
                else { bytes.iter().fold(0, |v, b| (v << 8) | u64::from(*b)) }
            };
            let (shoff, shsize, flag_size) = if file.is_64() {
                (word(&original[40..48]), word(&original[58..60]), 8)
            } else {
                (word(&original[32..36]), word(&original[46..48]), 4)
            };
            let flag_offset = (shoff + shsize * section.index().0 as u64 + 8) as usize;
            let mut nonexec = original.clone();
            let flag_byte = flag_offset + if little { 0 } else { flag_size - 1 };
            nonexec[flag_byte] &= !(object::elf::SHF_EXECINSTR as u8);
            assert!(scan_transfers(&object::File::parse(nonexec.as_slice()).unwrap()).is_empty());

            let got = file.section_by_name(".got").unwrap();
            let pointer_size = if file.is_64() { 8 } else { 4 };
            let got_index = got.data().unwrap().chunks_exact(pointer_size)
                .position(|bytes| word(bytes) == expected[0].0 - 8).unwrap();
            let got_offset = got.file_range().unwrap().0 as usize + got_index * pointer_size;
            let mut unlinked = original.clone();
            unlinked[got_offset..got_offset + pointer_size].fill(0);
            assert_eq!(scan_transfers(&object::File::parse(unlinked.as_slice()).unwrap()).len() + 1,
                expected.len());
            let (offset, _) = section.file_range().unwrap();
            let entry = expected[0].0 - 8;
            let offset = (offset + entry - section.address()) as usize;
            let mut bad = original.clone();
            bad[offset..offset + 4].fill(0);
            let file = object::File::parse(bad.as_slice()).unwrap();
            let actual = scan_transfers(&file);
            assert_eq!(actual.len() + 1, expected.len());
            assert!(!actual.contains(&expected[0]));
        }
    }

    #[test]
    fn complete_stub_requires_saved_return_resolver_and_matching_symbol() {
        for little in [false, true] {
            for (load, immediate) in [(0x8f998010, 0x24180007), (0xdf998010, 0x64180007)] {
                let words: [u32; 4] = [load, 0x03e07825, 0x0320f809, immediate];
                let encode = |w: &[u32]| -> Vec<u8> { w.iter().flat_map(|v|
                    if little { v.to_le_bytes() } else { v.to_be_bytes() }).collect() };
                let bytes = encode(&words);
                assert!(is_resolver_stub(&bytes, little, 7));
                assert!(!is_resolver_stub(&bytes, little, 8));
                for n in 0..16 { assert!(!is_resolver_stub(&bytes[..n], little, 7)); }
                for i in 0..4 {
                    let mut bad = words; bad[i] ^= 1;
                    assert!(!is_resolver_stub(&encode(&bad), little, 7));
                }
            }
        }
    }
}
