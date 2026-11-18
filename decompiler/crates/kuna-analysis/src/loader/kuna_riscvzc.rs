//! RISC-V Zcmp/Zcmt language selection from the ELF `Tag_RISCV_arch`
//! attribute. The Zc code-size extensions reuse the `c.fsdsp` encoding space
//! and exclude Zcd, so an image that declares `zcmp` or `zcmt` takes the
//! `RISCV:LE:<size>:Zcmp` language, whose processor spec sets the `Zcmp`
//! context bit; every other RISC-V image keeps the default selection.
use super::macho_trie::uleb;
use object::read::elf::{ElfFile, FileHeader, ProgramHeader};
use object::{Architecture, BinaryFormat, Object, ObjectSection};

const PT_RISCV_ATTRIBUTES: u32 = 0x7000_0003;
const TAG_FILE: u8 = 1;
const TAG_RISCV_ARCH: u64 = 5;
const MALFORMED: &str = "malformed .riscv.attributes";

fn ntbs<'a>(data: &'a [u8], at: &mut usize, end: usize) -> Option<&'a str> {
    let rest = data.get(*at..end)?;
    let len = rest.iter().position(|&b| b == 0)?;
    *at += len + 1;
    std::str::from_utf8(&rest[..len]).ok()
}

fn u32_at(data: &[u8], at: usize, little: bool) -> Option<usize> {
    let raw: [u8; 4] = data.get(at..at + 4)?.try_into().ok()?;
    let value = if little { u32::from_le_bytes(raw) } else { u32::from_be_bytes(raw) };
    usize::try_from(value).ok()
}

/// The `Tag_RISCV_arch` value of one `Tag_File` attribute list in
/// `data[at..end]`: odd tags carry a string, even tags a ULEB128.
fn file_arch(data: &[u8], mut at: usize, end: usize) -> Option<Option<&str>> {
    while at < end {
        let tag = uleb(data, &mut at, end)?;
        if tag % 2 == 1 {
            let text = ntbs(data, &mut at, end)?;
            if tag == TAG_RISCV_ARCH {
                return Some(Some(text));
            }
        } else {
            uleb(data, &mut at, end)?;
        }
    }
    Some(None)
}

/// The `Tag_RISCV_arch` string of one attributes blob (`'A'`, then
/// length-prefixed vendor sections of tagged, length-prefixed sub-sections).
fn arch_attribute(blob: &[u8], little: bool) -> Result<Option<String>, &'static str> {
    if blob.first() != Some(&b'A') {
        return Err(MALFORMED);
    }
    let mut at = 1;
    while at < blob.len() {
        let length = u32_at(blob, at, little).filter(|n| *n >= 4).ok_or(MALFORMED)?;
        let section_end = at.checked_add(length).filter(|e| *e <= blob.len()).ok_or(MALFORMED)?;
        let mut cursor = at + 4;
        let vendor = ntbs(blob, &mut cursor, section_end).ok_or(MALFORMED)?;
        while vendor == "riscv" && cursor < section_end {
            let tag = blob[cursor];
            let size = u32_at(blob, cursor + 1, little).filter(|n| *n >= 5).ok_or(MALFORMED)?;
            let sub_end = cursor.checked_add(size).filter(|e| *e <= section_end).ok_or(MALFORMED)?;
            if tag == TAG_FILE {
                if let Some(arch) = file_arch(blob, cursor + 5, sub_end).ok_or(MALFORMED)? {
                    return Ok(Some(arch.to_ascii_lowercase()));
                }
            }
            cursor = sub_end;
        }
        at = section_end;
    }
    Ok(None)
}

fn segment_blobs<'a, Elf: FileHeader>(file: &'a ElfFile<'a, Elf>) -> Vec<&'a [u8]> {
    file.elf_program_headers()
        .iter()
        .filter(|ph| ph.p_type(file.endian()) == PT_RISCV_ATTRIBUTES)
        .filter_map(|ph| ph.data(file.endian(), file.data()).ok())
        .collect()
}

/// The declared ISA string of a RISC-V ELF, from its `.riscv.attributes`
/// section or, without sections, its `PT_RISCV_ATTRIBUTES` segment; `Ok(None)`
/// when the image declares none.
pub(crate) fn arch_string(file: &object::File<'_>) -> Result<Option<String>, &'static str> {
    let mut blobs: Vec<&[u8]> = file
        .sections()
        .filter(|s| s.name() == Ok(".riscv.attributes"))
        .filter_map(|s| s.data().ok())
        .collect();
    if blobs.is_empty() {
        blobs = match file {
            object::File::Elf32(elf) => segment_blobs(elf),
            object::File::Elf64(elf) => segment_blobs(elf),
            _ => Vec::new(),
        };
    }
    let mut answer = None;
    for blob in blobs {
        if let Some(arch) = arch_attribute(blob, file.is_little_endian())? {
            if answer.as_ref().is_some_and(|old| *old != arch) {
                return Err("conflicting Tag_RISCV_arch declarations");
            }
            answer = Some(arch);
        }
    }
    Ok(answer)
}

/// The extension names of an ISA string with versions dropped:
/// `rv32i2p1_m2p0_c2p0_zcmp1p0` lists `i`, `m`, `c`, `zcmp`; the base
/// `rv32imac` lists each letter.
fn digits(s: &str) -> &str {
    s.trim_end_matches(|c: char| c.is_ascii_digit())
}

fn strip_version(part: &str) -> &str {
    let major = digits(part);
    if major.len() == part.len() {
        return part;
    }
    match major.strip_suffix('p') {
        Some(stem) if digits(stem).len() < stem.len() => digits(stem),
        _ => major,
    }
}

fn extensions(arch: &str) -> Vec<String> {
    let mut names = Vec::new();
    for (index, part) in arch.split('_').enumerate() {
        let part = strip_version(part);
        if index == 0 {
            let base = part.strip_prefix("rv32").or_else(|| part.strip_prefix("rv64")).unwrap_or(part);
            names.extend(base.chars().filter(|c| c.is_ascii_alphabetic() && *c != 'p').map(String::from));
        } else if !part.is_empty() {
            names.push(part.to_string());
        }
    }
    names
}

/// `None` leaves non-Zc RISC-V selection alone, including an image whose
/// attributes are malformed or conflicting; a declaration of both Zcmp and
/// Zcd has no language and requires an explicit target.
pub fn language_id(file: &object::File<'_>) -> Result<Option<String>, &'static str> {
    if file.format() != BinaryFormat::Elf
        || !matches!(file.architecture(), Architecture::Riscv32 | Architecture::Riscv64)
    {
        return Ok(None);
    }
    let Ok(Some(arch)) = arch_string(file) else { return Ok(None) };
    let names = extensions(&arch);
    let has = |name: &str| names.iter().any(|n| n == name);
    if !has("zcmp") && !has("zcmt") {
        return Ok(None);
    }
    if has("zcd") || (has("c") && (has("d") || has("g") || has("q"))) {
        return Err("Tag_RISCV_arch declares both Zcmp/Zcmt and Zcd");
    }
    let size = if file.is_64() { "64" } else { "32" };
    let endian = if file.is_little_endian() { "LE" } else { "BE" };
    Ok(Some(format!("RISCV:{endian}:{size}:Zcmp:gcc")))
}

pub fn language_ids() -> Vec<String> {
    vec!["RISCV:LE:32:Zcmp:gcc".to_string(), "RISCV:LE:64:Zcmp:gcc".to_string()]
}

#[cfg(test)]
mod tests {
    use super::*;
    use object::write::Object as Writer;
    use object::{Endianness, SectionKind};

    fn attributes(arch: &str) -> Vec<u8> {
        let mut file = vec![4, 16, 5];
        file.extend(arch.as_bytes());
        file.push(0);
        let mut sub = vec![TAG_FILE];
        sub.extend((file.len() as u32 + 5).to_le_bytes());
        sub.extend(file);
        let mut section = Vec::new();
        section.extend((sub.len() as u32 + 4 + 6).to_le_bytes());
        section.extend(b"riscv\0");
        section.extend(sub);
        let mut blob = vec![b'A'];
        blob.extend(section);
        blob
    }

    fn elf(elf64: bool, sections: &[(&[u8], Vec<u8>)]) -> Vec<u8> {
        let arch = if elf64 { Architecture::Riscv64 } else { Architecture::Riscv32 };
        let mut object = Writer::new(BinaryFormat::Elf, arch, Endianness::Little);
        for (name, bytes) in sections {
            let section = object.add_section(Vec::new(), name.to_vec(), SectionKind::Other);
            object.append_section_data(section, bytes, 1);
        }
        object.write().unwrap()
    }

    fn select(bytes: &[u8]) -> Result<Option<String>, &'static str> {
        language_id(&object::File::parse(bytes).unwrap())
    }

    #[test]
    fn versions_are_stripped_without_eating_the_name() {
        assert_eq!(
            extensions("rv32i2p1_m2p0_a2p1_c2p0_zmmul1p0_zca1p0_zcmp1p0"),
            ["i", "m", "a", "c", "zmmul", "zca", "zcmp"]
        );
        assert_eq!(extensions("rv64imac_zcmt"), ["i", "m", "a", "c", "zcmt"]);
        assert_eq!(strip_version("zcmp"), "zcmp");
        assert_eq!(strip_version("zcmp1p0"), "zcmp");
        assert_eq!(strip_version("zca1"), "zca");
    }

    #[test]
    fn zcmp_and_zcmt_select_the_zcmp_language_per_class() {
        for (elf64, id) in [(false, "RISCV:LE:32:Zcmp:gcc"), (true, "RISCV:LE:64:Zcmp:gcc")] {
            for arch in ["rv32i2p1_m2p0_c2p0_zca1p0_zcmp1p0", "rv32imac_zcmt1p0", "RV32IMAC_ZCA_ZCMP_ZCMT"] {
                let bytes = elf(elf64, &[(b".riscv.attributes", attributes(arch))]);
                assert_eq!(select(&bytes), Ok(Some(id.to_string())), "{arch}");
            }
        }
    }

    #[test]
    fn plain_and_absent_declarations_keep_the_default() {
        for arch in ["rv32i2p1_m2p0_a2p1_f2p2_d2p2_c2p0", "rv64gc", "rv32imac_zca1p0_zcb1p0"] {
            let bytes = elf(false, &[(b".riscv.attributes", attributes(arch))]);
            assert_eq!(select(&bytes), Ok(None), "{arch}");
        }
        assert_eq!(select(&elf(false, &[])), Ok(None));
        assert_eq!(select(&elf(true, &[])), Ok(None));
    }

    #[test]
    fn zcd_conflicts_require_an_explicit_target() {
        for arch in ["rv32imafdc_zcmp", "rv32imac_zcd1p0_zcmp1p0", "rv64gc_zcmp"] {
            let bytes = elf(false, &[(b".riscv.attributes", attributes(arch))]);
            assert!(select(&bytes).is_err(), "{arch}");
        }
    }

    #[test]
    fn malformed_and_conflicting_blobs_keep_the_default() {
        let parse = |bytes: &[u8]| arch_string(&object::File::parse(bytes).unwrap());
        let good = attributes("rv32imac_zcmp");
        let mut other = good.clone();
        other[0] = b'B';
        let mut blobs: Vec<Vec<u8>> = [0, 5, 12, 20, good.len() - 1].iter().map(|cut| good[..*cut].to_vec()).collect();
        blobs.push(other);
        for blob in blobs {
            let bytes = elf(false, &[(b".riscv.attributes", blob)]);
            assert_eq!(parse(&bytes), Err(MALFORMED));
            assert_eq!(select(&bytes), Ok(None));
        }
        let two = [(b".riscv.attributes".as_slice(), good.clone()), (b".riscv.attributes".as_slice(), attributes("rv32imac"))];
        let bytes = elf(false, &two);
        assert!(parse(&bytes).is_err());
        assert_eq!(select(&bytes), Ok(None));
    }
}
