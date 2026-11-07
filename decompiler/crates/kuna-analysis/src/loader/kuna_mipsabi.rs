//! Standard MIPS ABI records shared by target selection and call recovery.
use object::read::elf::{ElfFile, FileHeader, ProgramHeader};
use object::{Object, ObjectSection};

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
