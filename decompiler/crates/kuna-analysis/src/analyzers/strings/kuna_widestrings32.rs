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
//! `{72, 101, 108, 108, 111, 0}` is `L"Hello"`, a table of the weeks in each
//! year, `{52, 53, 52, ..}`, reads as `L"4544.."`, and a table such as
//! `{97, 98, 99, 100, 101, 0, 7, 8}` or a switch lookup table of character
//! codes runs on past its "terminator", so code reading it would read past the
//! literal. [`wide_string32_facts`] reads only read-only data, and plants a run
//! only when
//!
//! - no sized data object of the symbol tables overlaps it: a declared array,
//!   whatever its element type, keeps printing as its name (an assembler-local
//!   `.L` label of exactly the run's extent, which a relocatable object keeps for
//!   its own literals, does not count; `.Lswitch.table.f` does); and
//! - it lies in a mergeable string section of 4-byte entries (a relocatable
//!   object's `.rodata.str4.4`), where every NUL-terminated run is a literal, or
//!   all of the following hold:
//!   - it holds at least [`MIN_UNITS`] units, three of them distinct;
//!   - something points at its start: an operand the scalar scan found, a
//!     pointer-aligned slot of a data section, a dynamic relocation, or an
//!     entry of a table of relative offsets at an operand (clang's `reltable`);
//!   - what follows its terminator ([`follower`]) is the next literal or
//!     object, not the table's next element: the end of the section, or, past
//!     zero padding no longer than the next unit's alignment asks for, either a
//!     table base or a symbol's start whose first unit is no character (at or
//!     above U+110000) or opens a string, or an address something points at
//!     that opens a string (a zero-terminated wide run, or a narrow string of
//!     four characters or more) -- an operand naming the field after a struct's
//!     codes (a negative count, a pointer) names no next object;
//!   - no code adds a computed index to an address from its start to its
//!     terminator ([`crate::operand_refs`]' table uses: `lea rcx,t` then
//!     `mov eax,[rcx+rax*4]`).
//!
//! A literal laid out right after another object's last printable unit (a
//! switch table ending in `'q'`) is the tail of a longer run; when nothing
//! points at that run's start, an operand at one of its units starts the run
//! there instead, under the same tests.
//!
//! An ARM literal pool slot backs nothing, since the scan cannot see code index
//! the address it loads (a switch table's sits in one too), and an address code
//! builds in two instructions (AArch64 `adrp`/`add`, MIPS `lui`/`addiu`) is no
//! operand, so a linked image of those targets plants only what a data slot
//! holds. What is left are the tables the bytes cannot tell from literals: an
//! anonymous table that ends at its zero prints as the literal its elements
//! spell, the same values; the rows of a 2-D table of codes (`{{97, .., 0},
//! {102, .., 0}}`), adjacent tables each ending in a zero, or a code table
//! followed by a string (a `char name[8]` field, `{.., 0, 233, 120, 0}`),
//! print as one literal each, so code reading across them reads past a
//! literal; and a
//! fixed-size table whose
//! codes are followed by zero padding (`int t[8] = {97, 98, 99, 100, 101}`)
//! prints as the shorter literal, so code reading past its first zero reads
//! zeros in the binary and past the literal in the printed C.
//!
//! A relocatable object is read only through the laid-out view the loader
//! builds; its raw sections all sit at address 0.

use object::read::{Object, ObjectSection, ObjectSymbol};
use object::{BinaryFormat, ObjectKind, SectionKind, SymbolKind};

use crate::operand_refs::{held_pointers, relative_string_table, DataObjects};
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
/// pointing at and `tables` (sorted) the ones it saw code index as arrays; both
/// are empty when it did not run.
pub fn wide_string32_facts(file: &object::File, targets: &[u64], tables: &[u64]) -> Vec<StringFact> {
    if matches!(file.format(), BinaryFormat::Pe | BinaryFormat::Coff) {
        return Vec::new();
    }
    let little_endian = file.is_little_endian();
    let relocatable = file.kind() == ObjectKind::Relocatable;
    let mut runs: Vec<(Run32, bool, &[u8], u64)> = Vec::new();
    for sec in file.sections() {
        let Some(strings4) = readonly_data(file, &sec).filter(|_| !relocatable || sec.address() != 0) else {
            continue;
        };
        let Ok(data) = sec.data() else {
            continue;
        };
        let min_units = if strings4 { 1 } else { MIN_UNITS };
        let vma = sec.address();
        runs.extend(scan_utf32_runs(data, vma, little_endian, min_units).into_iter().map(|r| (r, strings4, data, vma)));
    }
    if runs.is_empty() {
        return Vec::new();
    }
    let follows: Vec<Option<Option<Follower>>> = runs
        .iter()
        .map(|(r, strings4, data, vma)| (!strings4).then(|| follower(r, data, *vma, little_endian)).flatten())
        .collect();
    let mut unpointed: Vec<u64> = runs
        .iter()
        .zip(&follows)
        .filter(|((_, strings4, ..), _)| !strings4)
        .flat_map(|((r, ..), next)| [Some(r.addr), next.flatten().map(|f| f.addr)])
        .flatten()
        .filter(|a| targets.binary_search(a).is_err())
        .collect();
    unpointed.sort_unstable();
    unpointed.dedup();
    let mut held = if unpointed.is_empty() { Vec::new() } else { held_pointers(file, &unpointed, false) };
    if !unpointed.is_empty() && file.format() == BinaryFormat::Elf {
        held.extend(
            targets
                .iter()
                .flat_map(|&t| relative_string_table(file, t, little_endian))
                .filter(|a| unpointed.binary_search(a).is_ok()),
        );
    }
    held.sort_unstable();
    let pointed = |a: u64| targets.binary_search(&a).is_ok() || held.binary_search(&a).is_ok();
    let (labels, objects): (Vec<_>, Vec<_>) = file
        .symbols()
        .chain(file.dynamic_symbols())
        .filter(|s| s.kind() == SymbolKind::Data && !s.is_undefined() && s.size() != 0)
        .map(|s| (s.name().is_ok_and(|n| n.starts_with(".L")), (s.address(), s.size())))
        .partition(|&(label, _)| label);
    let objects = DataObjects::from_spans(objects.into_iter().map(|(_, span)| span).collect());
    let labels = DataObjects::from_spans(labels.into_iter().map(|(_, span)| span).collect());
    let (reach, label_reach) = (objects.reach(), labels.reach());
    let declared = |a: u64| tables.binary_search(&a).is_ok() || objects.starts_at(a) || labels.starts_at(a);
    let next_object = |f: Follower| declared(f.addr) || (f.string && pointed(f.addr));
    let literal = |r: Run32, next: Option<Option<Follower>>, data: &[u8], vma: u64| -> Option<Run32> {
        if !next?.is_none_or(next_object) {
            return None;
        }
        let terminator = r.addr + 4 * r.units as u64;
        if tables.get(tables.partition_point(|&t| t < r.addr)).is_some_and(|&t| t <= terminator) {
            return None;
        }
        if pointed(r.addr) {
            return Some(r);
        }
        let at = *targets.get(targets.partition_point(|&t| t <= r.addr))?;
        if at >= terminator || (at - r.addr) % 4 != 0 {
            return None;
        }
        let from = usize::try_from(at - vma).ok()?;
        let to = usize::try_from(terminator + 4 - vma).ok()?;
        let tail = scan_utf32_runs(data.get(from..to)?, at, little_endian, MIN_UNITS);
        tail.into_iter().next().filter(|t| t.addr == at)
    };
    runs.into_iter()
        .zip(follows)
        .filter_map(|((r, strings4, data, vma), next)| {
            let r = if strings4 { r } else { literal(r, next, data, vma).filter(|r| r.distinct >= 3)? };
            let len = u64::from(r.len());
            (!objects.overlaps(&reach, r.addr, len) && !labels.overlaps_other(&label_reach, r.addr, len))
                .then_some(StringFact { addr: r.addr, len: r.len() })
        })
        .collect()
}

/// The most zero bytes [`follower`] reads as padding.
const MAX_PADDING: u64 = 64;

/// The first nonzero unit after a run's terminator, and whether a string
/// starts there (a zero-terminated wide run or a narrow C string); otherwise its
/// unit is no character, which only data an object starts at can be.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Follower {
    pub addr: u64,
    pub string: bool,
}

/// What comes after `r`'s terminator in its section (`data`, mapped at `vma`):
/// `Some(None)` the section's end, `Some(Some(f))` the first nonzero unit past
/// zero padding, which a literal's next literal or object starts at, or `None`
/// when it cannot be one: zeros that are no padding, more than [`MAX_PADDING`]
/// bytes or more than the alignment of the unit after them asks for
/// (`{97, 98, 99, 100, 101, 0, 0, 7}` is one table), or a unit that is a
/// character but opens no string (`{.., 101, 0, 7, 8}` goes on with the codes 7
/// and 8, and `{.., 101, 0}, 5` with a count). A string is a zero-terminated run
/// of characters below [`CODE_POINTS`] and outside the surrogates, the control
/// codes but tab, CR and LF excluded, with at least one printable ASCII unit, or
/// a narrow C string of four characters or more (fewer are the bytes of a
/// wide unit or of a pointer such as `0x402039`, `"9 @"`).
fn follower(r: &Run32, data: &[u8], vma: u64, little_endian: bool) -> Option<Option<Follower>> {
    let next = r.addr + u64::from(r.len());
    let mut at = next;
    loop {
        let Some(unit) = usize::try_from(at - vma).ok().and_then(|off| data.get(off..off + 4)) else {
            return Some(None);
        };
        let raw = [unit[0], unit[1], unit[2], unit[3]];
        if (if little_endian { u32::from_le_bytes(raw) } else { u32::from_be_bytes(raw) }) != 0 {
            break;
        }
        at += 4;
        if at - next > MAX_PADDING {
            return None;
        }
    }
    let align = 1u64 << at.trailing_zeros().min(MAX_PADDING.trailing_zeros());
    if at - next >= align && at != next {
        return None;
    }
    let off = usize::try_from(at - vma).ok()?;
    let unit = |k: usize| {
        let raw: [u8; 4] = data.get(off + 4 * k..off + 4 * k + 4)?.try_into().ok()?;
        Some(if little_endian { u32::from_le_bytes(raw) } else { u32::from_be_bytes(raw) })
    };
    let ascii = |u: u32| u < 0x80 && is_string_char(u as u8);
    let wide_char = |u: u32| ascii(u) || ((0xa0..CODE_POINTS).contains(&u) && !(0xd800..0xe000).contains(&u));
    let mut k = 0;
    let mut printable = false;
    while let Some(u) = unit(k).filter(|&u| wide_char(u)) {
        printable |= ascii(u);
        k += 1;
    }
    let wide = k > 0 && printable && unit(k) == Some(0);
    let string = wide || crate::operand_refs::string_len(&data[off..]).is_some_and(|len| len > 4);
    (string || unit(0)? >= CODE_POINTS).then_some(Some(Follower { addr: at, string }))
}

/// One past the largest Unicode code point: a unit at or above it is no
/// character, so data that starts with one is no next element of a code table.
const CODE_POINTS: u32 = 0x11_0000;

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

    fn fixture_facts(name: &str, targets: &[u64], tables: &[u64]) -> Vec<(u64, u32)> {
        let path = format!("{}/tests/fixtures/{name}", env!("CARGO_MANIFEST_DIR"));
        let bytes = std::fs::read(path).expect("read widestr32 fixture");
        let raw = object::File::parse(bytes.as_slice()).expect("parse widestr32 fixture");
        let view = crate::loader::kuna_relocrebase::rebased_view(&raw, &bytes);
        let (file, _) = crate::loader::kuna_relocrebase::select(raw, &bytes, &view);
        let mut facts: Vec<(u64, u32)> =
            wide_string32_facts(&file, targets, tables).iter().map(|f| (f.addr, f.len)).collect();
        facts.sort_unstable();
        facts
    }

    /// `widestr32_gcc_O2_x86_64`: the literals `L"hellow"` (0x402004),
    /// `L"(NULL)"`, `L"xbind"` (whose tail is passed too), `U"char32-text"`,
    /// `L"first-msg"` and `L"second-msg"` (both held by the pointer table `msgs`),
    /// beside the switch table `CSWTCH.19` (`L"helpz"` and then `'q'`), `rows`
    /// (`L"alpha"`, `L"bravo"`), `weeks` (two characters) and `codes`
    /// (`L"Hello"`, the last object of `.rodata`). With no operand only the
    /// pointer table backs a literal; with the operands the literals are planted
    /// and every declared table keeps its name; stripped, the operand at `codes`
    /// plants it, while the switch table, whose `'q'` nothing points at, `rows`
    /// and the two-character `weeks` never are.
    #[test]
    fn gcc_builds_plant_what_points_at_a_literal() {
        let held = vec![(0x402088, 40), (0x4020b0, 44)];
        assert_eq!(fixture_facts("widestr32_gcc_O2_x86_64", &[], &[]), held);
        let targets = [0x402004, 0x402020, 0x40203c, 0x402040, 0x402058, 0x402088, 0x4020e0, 0x402100, 0x402160, 0x402180];
        let tables = [0x4020e0, 0x402100];
        let literals = vec![(0x402004, 28), (0x402020, 28), (0x40203c, 24), (0x402058, 48), (0x402088, 40), (0x4020b0, 44)];
        assert_eq!(fixture_facts("widestr32_gcc_O2_x86_64", &targets, &tables), literals);
        let mut stripped = literals.clone();
        stripped.push((0x402180, 24));
        for tables in [&tables[..], &[]] {
            assert_eq!(fixture_facts("widestr32_gcc_O2_stripped_x86_64", &targets, tables), stripped);
        }
    }

    /// `widestr32_clang_O2_x86_64` keeps no symbol for its switch table
    /// (`L"helpz"` at 0x20a0, then `'q'`), so `L"hellow"` right after it is the
    /// tail of the run `L"qhellow"`: the operand at 0x20bc plants the literal
    /// from there. `L"first-msg"` is followed by `L"second-msg"`, which only
    /// clang's table of relative offsets `reltable.pick` (0x2050) reaches: with
    /// the operand at that table both are planted.
    #[test]
    fn an_operand_inside_a_run_plants_the_literal_it_points_at() {
        assert!(fixture_facts("widestr32_clang_O2_x86_64", &[], &[]).is_empty());
        let targets = [0x20a0, 0x20bc, 0x20d8, 0x2108, 0x2124, 0x2128, 0x213c];
        let literals = vec![(0x20bc, 28), (0x20d8, 48), (0x2108, 28), (0x2124, 24)];
        assert_eq!(fixture_facts("widestr32_clang_O2_x86_64", &targets, &[0x2060, 0x20a0]), literals);
        let mut explained = targets.to_vec();
        explained.insert(0, 0x2050);
        let mut first = literals.clone();
        first.extend([(0x213c, 40), (0x2164, 44)]);
        assert_eq!(fixture_facts("widestr32_clang_O2_x86_64", &explained, &[0x2060, 0x20a0]), first);
    }

    /// `widestr32_tables_gcc_O2_stripped`: int tables of character codes that
    /// run on past their zero, and `L"control"` (0x2008), followed by the format
    /// string `"%ld %u\n"`, which `printf`'s operand points at. A run whose next
    /// unit is a code that opens no literal is refused, whatever points at it
    /// (`passed`, 0x2100, then `7`; `keys`, 0x2040, held by the struct `s`, then
    /// `'z'`, `'x'`, 27); so is a run holding a table base, even from an operand
    /// inside it.
    #[test]
    fn a_table_past_its_zero_is_no_literal() {
        let name = "widestr32_tables_gcc_O2_stripped";
        assert!(fixture_facts(name, &[0x2008], &[]).is_empty());
        assert_eq!(fixture_facts(name, &[0x2008, 0x2028], &[]), [(0x2008, 32)]);
        assert!(fixture_facts(name, &[0x2100, 0x2118], &[]).is_empty());
        assert!(fixture_facts(name, &[], &[]).iter().all(|&(addr, _)| addr != 0x2040));
        assert!(fixture_facts(name, &[0x200c, 0x2028], &[0x2008]).is_empty());
        assert_eq!(fixture_facts(name, &[0x200c, 0x2028], &[]), [(0x200c, 28)]);
    }

    #[test]
    fn what_follows_a_literal_is_the_next_literal_or_no_code() {
        let mut d = wide("abcde", true);
        d.extend_from_slice(&[0; 8]);
        let run = scan_utf32_runs(&d, 0x1000, true, MIN_UNITS)[0];
        assert_eq!(follower(&run, &d, 0x1000, true), Some(None));
        d.extend(wide("fg", true));
        assert_eq!(follower(&run, &d, 0x1000, true), Some(Some(Follower { addr: 0x1020, string: true })));
        assert_eq!(follower(&run, &d, 0x1004, true), None);
        let mut named = wide("abcde", true);
        named.extend_from_slice(&7u32.to_le_bytes());
        let run = scan_utf32_runs(&named, 0x1000, true, MIN_UNITS)[0];
        assert_eq!(follower(&run, &named, 0x1000, true), None);
        let mut next = wide("abcde", true);
        next.extend(wide("xy", true));
        assert_eq!(follower(&run, &next, 0x1000, true), Some(Some(Follower { addr: 0x1018, string: true })));
        next.truncate(next.len() - 4);
        assert_eq!(follower(&run, &next, 0x1000, true), None);
        let mut other = wide("abcde", true);
        other.extend_from_slice(b"%ld %u\n\0");
        assert_eq!(follower(&run, &other, 0x1000, true), Some(Some(Follower { addr: 0x1018, string: true })));
        let mut pointer = wide("abcde", true);
        pointer.extend_from_slice(&0x40_2039u64.to_le_bytes());
        assert_eq!(follower(&run, &pointer, 0x1000, true), Some(Some(Follower { addr: 0x1018, string: false })));
        let mut field = wide("abcde", true);
        field.extend_from_slice(&(-5i32).to_le_bytes());
        assert_eq!(follower(&run, &field, 0x1000, true), Some(Some(Follower { addr: 0x1018, string: false })));
        let mut cafe = wide("abcde", true);
        cafe.extend(wide("caf\u{e9}", true));
        assert_eq!(follower(&run, &cafe, 0x1000, true), Some(Some(Follower { addr: 0x1018, string: true })));
        let mut long = wide("abcde", true);
        long.extend_from_slice(&[0; 68]);
        long.extend_from_slice(&7u32.to_le_bytes());
        let run = scan_utf32_runs(&long, 0x1000, true, MIN_UNITS)[0];
        assert_eq!(follower(&run, &long, 0x1000, true), None);
    }

    /// A relocatable object's `.rodata.str4.4` holds only literals, so every run
    /// there is planted, while the tables in `.rodata` are not, nothing pointing
    /// at them.
    #[test]
    fn a_four_byte_string_section_backs_its_runs() {
        for name in ["widestr32_aarch64_O2.o", "widestr32_arm32_O2.o"] {
            let lens: Vec<u32> = fixture_facts(name, &[], &[]).iter().map(|&(_, len)| len).collect();
            assert_eq!(lens, [28, 48, 28, 24, 20, 40, 44], "{name}");
        }
    }

    /// The big-endian MIPS image reaches its literals through `lui`/`addiu`
    /// pairs, which no operand names, so only the two `msgs` holds are planted.
    #[test]
    fn a_big_endian_image_reads_its_units_in_its_byte_order() {
        assert_eq!(fixture_facts("widestr32_mips32_be_O2", &[], &[]), [(0x400520, 40), (0x400548, 44)]);
    }

    #[test]
    fn a_pe_image_is_not_scanned() {
        assert!(fixture_facts("widestrings_x86_64.exe", &[], &[]).is_empty());
    }

    #[test]
    fn distinct_characters_are_counted() {
        let weeks: Vec<u8> = [52u32, 53, 52, 52, 52, 53, 0].iter().flat_map(|u| u.to_le_bytes()).collect();
        let runs = scan_utf32_runs(&weeks, 0x3000, true, MIN_UNITS);
        assert_eq!(runs, vec![Run32 { addr: 0x3000, units: 6, distinct: 2 }]);
    }
}
