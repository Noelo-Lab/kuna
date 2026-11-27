use object::{
    elf,
    read::elf::{ElfFile64, FileHeader, ProgramHeader, SectionHeader},
    LittleEndian,
};

use super::{MAX_MAPPED, PAGE};
use crate::state::Region;

const READ_WRITE: u32 = elf::PF_R | elf::PF_W;
const READ_EXECUTE: u32 = elf::PF_R | elf::PF_X;

pub(super) fn regions(image: &[u8]) -> Result<Vec<Region>, String> {
    let file = ElfFile64::<LittleEndian>::parse(image)
        .map_err(|e| format!("expected a little-endian ELF64: {e}"))?;
    let endian = file.endian();
    let header = file.elf_header();
    if header.e_machine(endian) != elf::EM_X86_64 || header.e_type(endian) != elf::ET_EXEC {
        return Err("oracle-case requires a fixed-address x86-64 ELF executable".into());
    }
    if file.elf_section_table().iter().any(|s| {
        matches!(
            s.sh_type(endian),
            elf::SHT_REL | elf::SHT_RELA | elf::SHT_RELR
        ) && s.sh_size(endian) != 0
            && s.sh_flags(endian) & u64::from(elf::SHF_ALLOC) != 0
    }) {
        return Err("ELF contains allocated relocation entries".into());
    }
    let mut regions = Vec::new();
    let mut mapped_size = 0;
    for ph in file.elf_program_headers() {
        match ph.p_type(endian) {
            elf::PT_DYNAMIC | elf::PT_INTERP | elf::PT_TLS => {
                return Err(
                    "dynamic loading, relocations and TLS are outside the ELF case model".into(),
                );
            }
            elf::PT_LOAD => {}
            _ => continue,
        }
        let size = ph.p_memsz(endian);
        if size == 0 && ph.p_filesz(endian) == 0 {
            continue;
        }
        let at = ph.p_vaddr(endian);
        let base = at & !(PAGE - 1);
        let prefix = at - base;
        let end = at
            .checked_add(size)
            .and_then(|end| end.checked_add(PAGE - 1))
            .map(|end| end & !(PAGE - 1))
            .filter(|&end| end <= 1 << 47)
            .ok_or("ELF load segment is outside the low canonical half")?;
        mapped_size += end - base;
        if mapped_size > MAX_MAPPED {
            return Err("ELF load segments exceed the explicit case size limit".into());
        }
        let (offset, file_size) = ph.file_range(endian);
        if file_size > size || offset % PAGE != prefix {
            return Err("invalid ELF load segment extent or alignment".into());
        }
        let bytes = if file_size == 0 {
            Vec::new()
        } else {
            let start = (offset - prefix) as usize;
            let end = offset
                .checked_add(file_size)
                .and_then(|end| usize::try_from(end).ok())
                .ok_or("ELF load segment file range overflows")?;
            image
                .get(start..end)
                .ok_or("truncated ELF load segment")?
                .to_vec()
        };
        let permissions = match ph.p_flags(endian) {
            elf::PF_R => "r",
            READ_WRITE => "rw",
            READ_EXECUTE => "rx",
            elf::PF_X => "x",
            _ => return Err("unsupported ELF load segment permissions".into()),
        };
        regions.push(Region {
            address: base,
            size: end - base,
            bytes,
            permissions,
            observe: permissions == "rw",
        });
    }
    if regions.is_empty() {
        return Err("ELF has no loadable pages".into());
    }
    Ok(regions)
}
