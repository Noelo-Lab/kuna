//! Name ownership for linked ELF object labels without a data extent.

use kuna_decomp::kuna_symbolnamechars::{sanitize_symbol_name_bytes, symbolnamechars_mode};
use object::read::{Object, ObjectSection, ObjectSegment, ObjectSymbol};
use object::{BinaryFormat, ObjectKind, SectionFlags, SymbolFlags};

/// Collect names for the signature rename guard without constructing data symbols.
pub fn zero_size_data_names(bytes: &[u8]) -> Vec<String> {
    let Ok(file) = object::File::parse(bytes) else {
        return Vec::new();
    };
    if file.format() != BinaryFormat::Elf
        || !matches!(file.kind(), ObjectKind::Executable | ObjectKind::Dynamic)
    {
        return Vec::new();
    }
    let mode = symbolnamechars_mode();
    file.symbols()
        .chain(file.dynamic_symbols())
        .filter_map(|sym| {
            if sym.size() != 0
                || !sym.is_definition()
                || !matches!(sym.flags(), SymbolFlags::Elf { st_info, .. } if st_info & 0xf == object::elf::STT_OBJECT)
            {
                return None;
            }
            let section = file.section_by_index(sym.section_index()?).ok()?;
            let address = sym.address();
            if !matches!(section.flags(), SectionFlags::Elf { sh_flags } if sh_flags & u64::from(object::elf::SHF_ALLOC) != 0)
                || !inside(address, section.address(), section.size())
                || !file.segments().any(|segment| inside(address, segment.address(), segment.size()))
            {
                return None;
            }
            let name = crate::loader::elf_plt::strip_version(sym.name_bytes().ok()?);
            let name = sanitize_symbol_name_bytes(&name, mode);
            (!name.is_empty()).then(|| String::from_utf8_lossy(&name).into_owned())
        })
        .collect()
}

fn inside(address: u64, start: u64, size: u64) -> bool {
    address
        .checked_sub(start)
        .is_some_and(|offset| offset < size)
}
