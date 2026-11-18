//! nanoMIPS ELF objects (`e_machine` 249, `EM_NANOMIPS`) select the nanoMIPS
//! language. The neutral parser has no architecture for the machine, so the id
//! is read from the ELF header here; every other object is left alone.
use object::read::elf::FileHeader;

pub(crate) const EM_NANOMIPS: u16 = 249;

pub(crate) fn language_ids() -> impl Iterator<Item = String> {
    ["LE", "BE"]
        .into_iter()
        .map(|endian| format!("nanoMIPS:{endian}:32:default:default"))
}

pub(crate) fn language_id(file: &object::File<'_>) -> Option<String> {
    let object::File::Elf32(elf) = file else {
        return None;
    };
    let header = elf.elf_header();
    if header.e_machine(elf.endian()) != EM_NANOMIPS {
        return None;
    }
    let endian = if header.is_little_endian() { "LE" } else { "BE" };
    Some(format!("nanoMIPS:{endian}:32:default:default"))
}
