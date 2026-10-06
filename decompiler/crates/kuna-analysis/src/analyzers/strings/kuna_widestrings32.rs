//! (kuna `widestrings32`) The 4-byte width of the string-literal markup: a
//! `wchar_t` literal of an ELF or Mach-O target, where `wchar_t` is 4 bytes, or a
//! `char32_t` one.
//!
//! # The gap
//!
//! Read at 1-byte width `L"hellow"` is `68 00 00 00 65 00 00 00 ..`, a
//! one-character string, and at 2-byte width it is `68 00` and a terminator, so
//! neither existing width marks it up and a call that hands it to the image's own
//! function printed its address (`lenw(&dat_2004)`). [`scan_utf32_runs`] is the
//! 1-byte matcher widened to 4-byte code units, as [`super::kuna_widestrings`]
//! widens it to 2: a run of units that each hold a [`super::is_string_char`]
//! value, read in the image's byte order on 4-aligned addresses and closed by a
//! zero unit.
//!
//! # Why a run needs more than its bytes
//!
//! An `int` table of character codes is byte for byte a wide literal:
//! `{72, 101, 108, 108, 111, 0}` is `L"Hello"`, and a table of the weeks in each
//! year, `{52, 53, 52, ..}`, reads as `L"4544.."`. So [`wide_string32_facts`]
//! reads only read-only data, and plants a run only when
//!
//! - no sized data object of the symbol tables overlaps it: a declared array,
//!   whatever its element type, keeps printing as its name (an assembler-local
//!   `.L` label, which a relocatable object keeps for its own literals, names
//!   nothing the source declared); and
//! - it lies in a mergeable string section of 4-byte entries (a relocatable
//!   object's `.rodata.str4.4`), or it holds at least [`MIN_UNITS`] units of at
//!   least three distinct characters and either the image kept its local
//!   symbols (its symbol table names a source file), so every array it declares,
//!   `static` ones included, is a named object and only literals are left
//!   unnamed, or something points at its start: an operand the scalar scan
//!   found, a pointer-aligned slot, a dynamic relocation.
//!
//! A relocatable object is read only through the laid-out view the loader
//! builds; its raw sections all sit at address 0.
//!
//! A stripped image's anonymous `int` table that passes all of that, NUL unit
//! included, still prints as the literal its bytes spell; the values are the
//! same either way.

use object::read::{Object, ObjectSection, ObjectSymbol};
use object::{BinaryFormat, ObjectKind, SectionKind, SymbolKind};

use crate::operand_refs::{held_pointers, DataObjects};
use crate::pass::StringFact;

use super::is_string_char;

/// The fewest code units a run needs outside a declared string section, the
/// length the 2-byte width uses.
pub(crate) const MIN_UNITS: usize = super::DEFAULT_WIDE_MIN_LEN;

const SHF_WRITE: u64 = 0x1;
const SHF_ALLOC: u64 = 0x2;
const SHF_EXECINSTR: u64 = 0x4;
const SHF_MERGE: u64 = 0x10;
const SHF_STRINGS: u64 = 0x20;

/// One 4-byte run: its address, its code units (terminator excluded), and the
/// number of distinct characters among them.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Run32 {
    pub addr: u64,
    pub units: usize,
    pub distinct: usize,
}

impl Run32 {
    /// The `wchar4[N]` byte span, terminator included.
    pub(crate) fn len(&self) -> u32 {
        ((self.units + 1) * 4) as u32
    }
}

/// Every run of at least `min_units` 4-byte code units in `data` (mapped at
/// `vma`) whose values are [`is_string_char`] characters, read in the given byte
/// order on 4-aligned addresses and closed by a zero unit.
pub(crate) fn scan_utf32_runs(data: &[u8], vma: u64, little_endian: bool, min_units: usize) -> Vec<Run32> {
    let mut out = Vec::new();
    let mut start: Option<usize> = None;
    let mut seen: u128 = 0;
    let mut i = ((4 - vma % 4) % 4) as usize;
    while i + 4 <= data.len() {
        let raw = [data[i], data[i + 1], data[i + 2], data[i + 3]];
        let unit = if little_endian { u32::from_le_bytes(raw) } else { u32::from_be_bytes(raw) };
        if unit < 0x80 && is_string_char(unit as u8) {
            if start.is_none() {
                start = Some(i);
                seen = 0;
            }
            seen |= 1u128 << unit;
        } else if let Some(s) = start.take() {
            let units = (i - s) / 4;
            if unit == 0 && units >= min_units {
                out.push(Run32 { addr: vma + s as u64, units, distinct: seen.count_ones() as usize });
            }
        }
        i += 4;
    }
    out
}

/// A section the scan reads: allocated, initialized, read-only data that is not
/// one of the loader's tables, and whether it is a mergeable string section of
/// 4-byte entries.
fn readonly_data(file: &object::File, sec: &object::Section) -> Option<bool> {
    if sec.size() == 0 || matches!(sec.kind(), SectionKind::UninitializedData) {
        return None;
    }
    match sec.flags() {
        object::SectionFlags::Elf { sh_flags } => {
            let name = sec.name().unwrap_or("");
            let data = sh_flags & SHF_ALLOC != 0
                && sh_flags & (SHF_WRITE | SHF_EXECINSTR) == 0
                && !crate::loader::format::elf::is_loader_table(name, sec.kind());
            let strings4 = sh_flags & (SHF_MERGE | SHF_STRINGS) == SHF_MERGE | SHF_STRINGS
                && elf_entsize(file, sec.index()) == Some(4);
            data.then_some(strings4)
        }
        _ => matches!(sec.kind(), SectionKind::ReadOnlyData | SectionKind::ReadOnlyString).then_some(false),
    }
}

/// The `sh_entsize` of an ELF image's section `index`.
fn elf_entsize(file: &object::File, index: object::SectionIndex) -> Option<u64> {
    use object::read::elf::SectionHeader;
    match file {
        object::File::Elf32(f) => {
            Some(u64::from(f.section_by_index(index).ok()?.elf_section_header().sh_entsize(f.endian())))
        }
        object::File::Elf64(f) => Some(f.section_by_index(index).ok()?.elf_section_header().sh_entsize(f.endian())),
        _ => None,
    }
}

/// The `wchar4[N]` facts of `file`: every 4-byte run [`scan_utf32_runs`] finds in
/// its read-only data that the image backs as a string (see the module docs).
/// `targets` (sorted) are the read-only addresses the scalar scan found operands
/// pointing at, empty when it did not run.
pub fn wide_string32_facts(file: &object::File, targets: &[u64]) -> Vec<StringFact> {
    if matches!(file.format(), BinaryFormat::Pe | BinaryFormat::Coff) {
        return Vec::new();
    }
    let little_endian = file.is_little_endian();
    let relocatable = file.kind() == ObjectKind::Relocatable;
    let mut runs: Vec<(Run32, bool)> = Vec::new();
    for sec in file.sections() {
        let Some(strings4) = readonly_data(file, &sec).filter(|_| !relocatable || sec.address() != 0) else {
            continue;
        };
        let Ok(data) = sec.data() else {
            continue;
        };
        let min_units = if strings4 { 1 } else { MIN_UNITS };
        runs.extend(scan_utf32_runs(data, sec.address(), little_endian, min_units).into_iter().map(|r| (r, strings4)));
    }
    if runs.is_empty() {
        return Vec::new();
    }
    let declared = |s: &object::Symbol| {
        s.kind() == SymbolKind::Data
            && !s.is_undefined()
            && s.size() != 0
            && !s.name().is_ok_and(|n| n.starts_with(".L"))
    };
    let named = file.symbols().any(|s| s.kind() == SymbolKind::File);
    let objects = DataObjects::from_spans(
        file.symbols()
            .chain(file.dynamic_symbols())
            .filter(|s| declared(s))
            .map(|s| (s.address(), s.size()))
            .collect(),
    );
    let reach = objects.reach();
    runs.retain(|(r, strings4)| {
        !objects.overlaps(&reach, r.addr, u64::from(r.len())) && (*strings4 || r.distinct >= 3)
    });
    let mut unbacked: Vec<u64> = runs
        .iter()
        .filter(|(r, strings4)| !strings4 && !named && targets.binary_search(&r.addr).is_err())
        .map(|(r, _)| r.addr)
        .collect();
    if !unbacked.is_empty() {
        unbacked.sort_unstable();
        let held = held_pointers(file, &unbacked);
        unbacked.retain(|a| !held.contains(a));
    }
    runs.into_iter()
        .filter(|(r, _)| unbacked.binary_search(&r.addr).is_err())
        .map(|(r, _)| StringFact { addr: r.addr, len: r.len() })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn wide(s: &str, little_endian: bool) -> Vec<u8> {
        let mut d = Vec::new();
        for ch in s.chars().chain(std::iter::once('\0')) {
            let u = ch as u32;
            d.extend_from_slice(&if little_endian { u.to_le_bytes() } else { u.to_be_bytes() });
        }
        d
    }

    #[test]
    fn utf32_runs_in_either_byte_order() {
        for le in [true, false] {
            let data = wide("hellow", le);
            let runs = scan_utf32_runs(&data, 0x2004, le, MIN_UNITS);
            assert_eq!(runs, vec![Run32 { addr: 0x2004, units: 6, distinct: 5 }]);
            assert_eq!(runs[0].len(), 28);
            assert!(scan_utf32_runs(&data, 0x2004, !le, MIN_UNITS).is_empty());
        }
    }

    #[test]
    fn short_unterminated_and_misaligned_runs_are_not_runs() {
        assert!(scan_utf32_runs(&wide("abcd", true), 0x1000, true, MIN_UNITS).is_empty());
        assert_eq!(scan_utf32_runs(&wide("abcd", true), 0x1000, true, 1).len(), 1);
        let mut open = wide("hellow", true);
        open.truncate(24);
        open.extend_from_slice(&1000u32.to_le_bytes());
        assert!(scan_utf32_runs(&open, 0x1000, true, MIN_UNITS).is_empty());
        let mut shifted = vec![0xffu8, 0xff];
        shifted.extend(wide("hellow", true));
        assert!(scan_utf32_runs(&shifted, 0x1000, true, MIN_UNITS).is_empty());
        assert!(scan_utf32_runs(&shifted, 0x1002, true, MIN_UNITS).len() == 1);
    }

    #[test]
    fn narrow_and_utf16_text_is_never_a_utf32_run() {
        let narrow = b"NtQueryInformationProcess\0\0\0\0\0\0\0";
        assert!(scan_utf32_runs(narrow, 0x2000, true, MIN_UNITS).is_empty());
        assert_eq!(scan_utf32_runs(narrow, 0x2000, true, 1), vec![Run32 { addr: 0x2018, units: 1, distinct: 1 }]);
        let mut utf16 = Vec::new();
        for ch in "ntdll.dll\0".bytes() {
            utf16.extend_from_slice(&[ch, 0]);
        }
        assert!(scan_utf32_runs(&utf16, 0x2000, true, 1).is_empty());
    }

    fn fixture_facts(name: &str, targets: &[u64]) -> Vec<(u64, u32)> {
        let path = format!("{}/tests/fixtures/{name}", env!("CARGO_MANIFEST_DIR"));
        let bytes = std::fs::read(path).expect("read widestr32 fixture");
        let raw = object::File::parse(bytes.as_slice()).expect("parse widestr32 fixture");
        let view = crate::loader::kuna_relocrebase::rebased_view(&raw, &bytes);
        let (file, _) = crate::loader::kuna_relocrebase::select(raw, &bytes, &view);
        let mut facts: Vec<(u64, u32)> = wide_string32_facts(&file, targets).iter().map(|f| (f.addr, f.len)).collect();
        facts.sort_unstable();
        facts
    }

    /// `widestr32.c`: `L"hellow"`, `L"(NULL)"`, `L"xbind"` (whose tail `L"bind"`
    /// is passed too) and `U"char32-text"`, beside `codes` (`int {72, 101, 108,
    /// 108, 111, 0}`, which spells "Hello") and `weeks` (`int {52, 53, ..}`, two
    /// characters). With the symbol table the literals are planted and both
    /// tables keep their names; stripped, only an operand target is planted,
    /// `codes` with them and `weeks` never.
    #[test]
    fn widestr32_builds_plant_the_literals() {
        let literals = vec![(0x402004, 28), (0x402020, 28), (0x40203c, 24), (0x402058, 48)];
        assert_eq!(fixture_facts("widestr32_gcc_O2_x86_64", &[]), literals);
        assert!(fixture_facts("widestr32_gcc_O2_stripped_x86_64", &[]).is_empty());
        let targets = [0x402004, 0x402020, 0x40203c, 0x402040, 0x402058, 0x4020a0, 0x4020c0];
        let mut stripped = literals.clone();
        stripped.push((0x4020c0, 24));
        assert_eq!(fixture_facts("widestr32_gcc_O2_stripped_x86_64", &targets), stripped);
        assert_eq!(
            fixture_facts("widestr32_clang_O2_x86_64", &[]),
            vec![(0x2050, 28), (0x206c, 48), (0x209c, 28), (0x20b8, 24)]
        );
        assert_eq!(
            fixture_facts("widestr32_mips32_be_O2", &[]),
            vec![(0x4003a0, 28), (0x4003bc, 48), (0x4003ec, 28), (0x400408, 24)]
        );
    }

    /// A relocatable object's `.rodata.str4.4` holds only literals, so every run
    /// there is planted, while `codes` and `weeks` in `.rodata` keep their names.
    #[test]
    fn a_four_byte_string_section_backs_its_runs() {
        for name in ["widestr32_aarch64_O2.o", "widestr32_arm32_O2.o"] {
            let lens: Vec<u32> = fixture_facts(name, &[]).iter().map(|&(_, len)| len).collect();
            assert_eq!(lens.len(), 5, "{name}: {lens:?}");
            assert!([28, 48, 28, 24, 20].iter().all(|l| lens.contains(l)), "{name}: {lens:?}");
        }
    }

    #[test]
    fn a_pe_image_is_not_scanned() {
        assert!(fixture_facts("widestrings_x86_64.exe", &[]).is_empty());
    }

    #[test]
    fn distinct_characters_are_counted() {
        let weeks: Vec<u8> = [52u32, 53, 52, 52, 52, 53, 0].iter().flat_map(|u| u.to_le_bytes()).collect();
        let runs = scan_utf32_runs(&weeks, 0x3000, true, MIN_UNITS);
        assert_eq!(runs, vec![Run32 { addr: 0x3000, units: 6, distinct: 2 }]);
    }
}
