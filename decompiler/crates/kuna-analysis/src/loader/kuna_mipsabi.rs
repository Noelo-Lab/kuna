//! MIPS architecture, instruction-word and standard ABI-record helpers shared
//! by the loader's target selection and call recovery.
use object::read::elf::{ElfFile, FileHeader, ProgramHeader};
use object::{Object, ObjectSection};

/// Whether the object reader's architecture tag is MIPS, including an ELF32
/// n32 image (`EF_MIPS_ABI2`), which the reader tags `Mips64_N32`.
pub(crate) fn is_mips(arch: object::Architecture) -> bool {
    matches!(arch, object::Architecture::Mips | object::Architecture::Mips64 | object::Architecture::Mips64_N32)
}

/// An unqualified name can denote multiple symbol versions, and different
/// GOT slots can name the same nonzero destination. Neither identifies a
/// unique library function, even when the image has no DWARF.
#[derive(Default)]
pub(crate) struct AmbiguousImports<'a> {
    pub names: std::collections::HashSet<&'a str>,
    pub destinations: std::collections::HashSet<u64>,
}

pub(crate) fn ambiguous_imports<'a>(file: &object::File<'a>) -> AmbiguousImports<'a> {
    use object::{ObjectSymbol, ObjectSymbolTable};
    use std::collections::{HashMap, HashSet};

    let mut result = AmbiguousImports::default();
    if !is_mips(file.architecture()) || file.format() != object::BinaryFormat::Elf {
        return result;
    }
    let Some(symbols) = file.dynamic_symbol_table() else { return result };
    let base_name = |s: &'a str| s.split_once('@').map_or(s, |(name, _)| name);
    let mut seen = HashSet::new();
    // A default definition and a non-default import may share the unversioned name.
    for name in file.dynamic_symbols().filter(|s| s.is_global()).filter_map(|s| s.name().ok()) {
        let name = base_name(name);
        if !seen.insert(name) { result.names.insert(name); }
    }
    let mut owners = HashMap::new();
    for (stub, index) in super::elf_plt::mips_stub_identities(file) {
        if stub == 0 { continue; }
        let name = symbols.symbol_by_index(object::SymbolIndex(index as usize))
            .ok().and_then(|s| s.name().ok()).map(base_name);
        if let Some(other) = owners.insert(stub, name) {
            result.destinations.insert(stub);
            result.names.extend(other);
            result.names.extend(name);
        }
    }
    result
}

/// The whole 32-bit instruction words at the start of `bytes`, in the image's
/// byte order.
pub(crate) fn words(bytes: &[u8], little: bool) -> impl Iterator<Item = u32> + '_ {
    bytes.chunks_exact(4).map(move |raw| {
        let raw = raw.try_into().unwrap();
        if little { u32::from_le_bytes(raw) } else { u32::from_be_bytes(raw) }
    })
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct AbiFlags(pub(super) [u8; 24]);

impl AbiFlags {
    fn read(bytes: &[u8]) -> Result<Self, &'static str> {
        if bytes.len() != 24 || bytes[..2] != [0, 0] {
            return Err("malformed or unsupported MIPS ABI-flags record");
        }
        Ok(Self(bytes.try_into().unwrap()))
    }

    pub(super) fn extension(self, little: bool) -> u32 {
        let bytes = self.0[8..12].try_into().unwrap();
        if little {
            u32::from_le_bytes(bytes)
        } else {
            u32::from_be_bytes(bytes)
        }
    }
}

fn merge_record(answer: &mut Option<AbiFlags>, bytes: &[u8]) -> Result<(), &'static str> {
    let value = AbiFlags::read(bytes)?;
    if answer.is_some_and(|old| old != value) {
        return Err("conflicting MIPS ABI-flags records");
    }
    *answer = Some(value);
    Ok(())
}

fn segment_records<Elf: FileHeader>(
    file: &ElfFile<'_, Elf>,
    answer: &mut Option<AbiFlags>,
) -> Result<(), &'static str> {
    for ph in file.elf_program_headers() {
        if ph.p_type(file.endian()) == object::elf::PT_MIPS_ABIFLAGS {
            let data = ph
                .data(file.endian(), file.data())
                .map_err(|_| "truncated MIPS ABI-flags segment")?;
            merge_record(answer, data)?;
        }
    }
    Ok(())
}

pub(super) fn abi_flags(file: &object::File<'_>) -> Result<Option<AbiFlags>, &'static str> {
    let mut answer = None;
    for section in file.sections().filter(|s| s.name() == Ok(".MIPS.abiflags")) {
        merge_record(
            &mut answer,
            section
                .data()
                .map_err(|_| "unreadable MIPS ABI-flags section")?,
        )?;
    }
    match file {
        object::File::Elf32(elf) => segment_records(elf, &mut answer)?,
        object::File::Elf64(elf) => segment_records(elf, &mut answer)?,
        _ => {}
    }
    Ok(answer)
}

/// GNU ANY (zero) leaves the convention unspecified; explicit declarations
/// must agree across the ABI record and GNU attributes.
pub(super) fn fp_abi(
    file: &object::File<'_>,
    record: Option<AbiFlags>,
) -> Result<Option<u32>, &'static str> {
    let attribute = super::kuna_floatabi::gnu_fp_abi(file)
        .ok_or("malformed or conflicting GNU floating-point attributes")?
        .filter(|v| *v != 0);
    let record_fp = record.map(|a| u32::from(a.0[7])).filter(|v| *v != 0);
    if matches!((attribute, record_fp), (Some(a), Some(b)) if a != b) {
        return Err("conflicting MIPS floating-point ABI declarations");
    }
    Ok(attribute.or(record_fp))
}
