//! The toolchain a compile unit names: `DW_AT_language` and `DW_AT_producer`
//! of the first compile units in `.debug_info`. A fallback for an image whose
//! `.comment` was stripped but whose debug info was kept.

use super::Compiler;
use object::{Object, ObjectSection};

const UNITS: usize = 16;

fn section<'a>(file: &'a object::File<'a>, name: &str) -> &'a [u8] {
    let macho = format!("__{}", name.trim_start_matches('.'));
    file.section_by_name(name)
        .or_else(|| file.section_by_name(&macho))
        .and_then(|s| s.data().ok())
        .unwrap_or(&[])
}

/// The compiler the debug info names, if any unit says.
pub fn dwarf_compiler(file: &object::File) -> Option<Compiler> {
    let endian = if file.is_little_endian() { gimli::RunTimeEndian::Little } else { gimli::RunTimeEndian::Big };
    let dwarf = gimli::Dwarf::load(|id| -> Result<_, gimli::Error> {
        Ok(gimli::EndianSlice::new(section(file, id.name()), endian))
    })
    .ok()?;
    let mut units = dwarf.units();
    let mut verdict = None;
    for _ in 0..UNITS {
        let Ok(Some(header)) = units.next() else { break };
        let Ok(unit) = dwarf.unit(header) else { continue };
        let mut entries = unit.entries();
        let Ok(Some(root)) = entries.next_dfs() else { continue };
        if let Some(gimli::AttributeValue::Language(lang)) = root.attr_value(gimli::DW_AT_language) {
            match lang {
                gimli::DW_LANG_Rust => return Some(Compiler::Rustc),
                gimli::DW_LANG_Go => return Some(Compiler::Go),
                gimli::DW_LANG_Swift => return Some(Compiler::Swift),
                _ => {}
            }
        }
        let producer = root
            .attr_value(gimli::DW_AT_producer)
            .and_then(|v| dwarf.attr_string(&unit, v).ok())
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_default();
        if producer.contains("rustc") {
            return Some(Compiler::Rustc);
        }
        if verdict.is_none() {
            if producer.contains("clang") {
                verdict = Some(Compiler::Clang);
            } else if producer.starts_with("GNU ") {
                verdict = Some(Compiler::Gcc);
            }
        }
    }
    verdict
}

#[cfg(test)]
mod tests {
    use super::*;

    fn detect(name: &str) -> Option<Compiler> {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures").join(name);
        let bytes = std::fs::read(path).unwrap();
        let file = object::File::parse(&*bytes).unwrap();
        dwarf_compiler(&file)
    }

    #[test]
    fn producer_and_language_name_the_toolchain() {
        assert_eq!(detect("dwarfglobaltypes_gcc_O1_x86_64"), Some(Compiler::Gcc));
        assert_eq!(detect("narrowload_dwarf_clang_O0_x86_64"), Some(Compiler::Clang));
        assert_eq!(detect("dwarfvariants_x86_64"), Some(Compiler::Rustc));
    }
}
