//! (kuna) `unflaggedcode` — memory the image maps executable in a section whose
//! own flags do not say it holds instructions.
//!
//! The reference walk ([`super::xrefs`]) decodes the sections the linker marked
//! as code ([`crate::entry::executable_sections`]): ELF `SHF_EXECINSTR`, Mach-O
//! `S_ATTR_PURE_INSTRUCTIONS`/`S_ATTR_SOME_INSTRUCTIONS`, PE
//! `IMAGE_SCN_MEM_EXECUTE`. The loader obeys the segment instead, so a section
//! can be executable at run time without being marked: an Apple toolchain links
//! a function placed in `__TEXT,__const` with no instruction attribute, a
//! bare-metal link script leaves `.ramfunc` or `.isr_vector` flagged `WA` in an
//! `RWE` `PT_LOAD`, and GNU ld puts `.rodata` in the `R E` segment on AArch64.
//! The walk does not decode such memory on its own account (most of it is
//! strings and tables), but an explicit query about one address there has to
//! be able to ask, and to say why it was not answered when it is not.
//!
//! A PE section is its own mapping unit, so a PE section is either marked
//! executable (and walked) or not executable at all; only ELF and Mach-O have
//! the gap this module describes.

use object::{
    Object, ObjectSection, ObjectSegment, ObjectSymbol, SectionFlags, SectionKind, SegmentFlags, SymbolKind,
};

/// One section of executable memory the walk does not decode on its own.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnflaggedSection {
    /// `__TEXT,__const` (Mach-O segment and section) or `.rodata`.
    pub name: String,
    pub start: u64,
    pub end: u64,
}

/// Where an address sits, as the reference walk sees it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Placement {
    /// In memory the walk decodes: a section marked as code (or, for an image
    /// with no section table, an executable segment). `section` is `None` in
    /// the second case.
    Code { section: Option<String> },
    /// In executable memory, in a section that is not marked as code and whose
    /// type does not rule instructions out.
    Unflagged(UnflaggedSection),
    /// In a section of data: memory the image does not map executable
    /// (`executable: false`), or executable memory in a section whose type
    /// holds only data (`__TEXT,__cstring`, `.dynstr`).
    Data { section: String, executable: bool },
    /// In no section the image maps: a header, a gap, or unmapped.
    NoSection,
    /// The object file's addresses are not the run-time ones (a relocatable
    /// object, which the loader lays out itself), so nothing can be said.
    Unknown,
}

/// Every [`UnflaggedSection`] of the image, in address order. Empty for a PE,
/// a relocatable object, and almost every ELF and Mach-O image.
pub fn unflagged_sections(file: &object::File) -> Vec<UnflaggedSection> {
    if file.kind() == object::ObjectKind::Relocatable {
        return Vec::new();
    }
    let segments = executable_segments(file);
    if segments.is_empty() {
        return Vec::new();
    }
    let mut out: Vec<UnflaggedSection> = file
        .sections()
        .filter_map(|sec| match classify(&sec, &segments)? {
            Placement::Unflagged(section) => Some(section),
            _ => None,
        })
        .collect();
    out.sort_by_key(|s| s.start);
    out
}

/// Where `vma` sits. A section marked as code wins over any other section that
/// also covers the address, the way the walk's own gate decides.
pub fn placement(file: &object::File, vma: u64) -> Placement {
    if file.kind() == object::ObjectKind::Relocatable {
        return Placement::Unknown;
    }
    let segments = executable_segments(file);
    if file.sections().next().is_none() {
        return if file.format() == object::BinaryFormat::Elf && in_any(&segments, vma) {
            Placement::Code { section: None }
        } else {
            Placement::NoSection
        };
    }
    let mut found = Placement::NoSection;
    for sec in file.sections() {
        let start = sec.address();
        if vma < start || vma - start >= sec.size() {
            continue;
        }
        let Some(here) = classify(&sec, &segments) else { continue };
        let rank = |p: &Placement| match p {
            Placement::Code { .. } => 3,
            Placement::Unflagged(_) => 2,
            Placement::Data { .. } => 1,
            _ => 0,
        };
        if rank(&here) > rank(&found) {
            found = here;
        }
    }
    found
}

/// The name of the data object an ELF symbol table declares at `vma`
/// (`STT_OBJECT`, `STT_COMMON`, `STT_TLS`). A Mach-O `nlist` and a PE image
/// declare no such type, so the answer there is always `None`.
pub fn declared_data(file: &object::File, vma: u64) -> Option<String> {
    declared(file, vma, |kind| matches!(kind, SymbolKind::Data | SymbolKind::Tls))
}

/// The name of the function an ELF symbol table declares at `vma`
/// (`STT_FUNC`, `STT_GNU_IFUNC`); `None` on every other format, for the same
/// reason as [`declared_data`].
pub fn declared_function(file: &object::File, vma: u64) -> Option<String> {
    declared(file, vma, |kind| kind == SymbolKind::Text)
}

fn declared(file: &object::File, vma: u64, kind: impl Fn(SymbolKind) -> bool) -> Option<String> {
    if file.format() != object::BinaryFormat::Elf {
        return None;
    }
    file.symbols()
        .chain(file.dynamic_symbols())
        .find(|sym| sym.address() == vma && sym.is_definition() && kind(sym.kind()))
        .map(|sym| sym.name().unwrap_or_default().to_string())
}

/// `(start, end)` of every segment the image maps executable: ELF `PF_X`,
/// Mach-O `initprot` `VM_PROT_EXECUTE`, PE `IMAGE_SCN_MEM_EXECUTE`.
fn executable_segments(file: &object::File) -> Vec<(u64, u64)> {
    file.segments()
        .filter(|seg| match seg.flags() {
            SegmentFlags::Elf { p_flags } => p_flags & object::elf::PF_X != 0,
            SegmentFlags::MachO { initprot, .. } => initprot & object::macho::VM_PROT_EXECUTE != 0,
            SegmentFlags::Coff { characteristics } => {
                characteristics & object::pe::IMAGE_SCN_MEM_EXECUTE != 0
            }
            _ => false,
        })
        .map(|seg| (seg.address(), seg.address().saturating_add(seg.size())))
        .filter(|&(lo, hi)| hi > lo)
        .collect()
}

fn in_any(ranges: &[(u64, u64)], vma: u64) -> bool {
    ranges.iter().any(|&(lo, hi)| vma >= lo && vma < hi)
}

/// What one mapped section is; `None` for a section that occupies no memory
/// (an ELF section without `SHF_ALLOC`, a Mach-O debug section, an empty one).
fn classify(sec: &object::Section, segments: &[(u64, u64)]) -> Option<Placement> {
    const SHF_ALLOC: u64 = 0x2;
    const S_ATTR_DEBUG: u32 = 0x0200_0000;
    let (start, size) = (sec.address(), sec.size());
    if size == 0 {
        return None;
    }
    let mapped = match sec.flags() {
        SectionFlags::Elf { sh_flags } => sh_flags & SHF_ALLOC != 0,
        SectionFlags::MachO { flags } => flags & S_ATTR_DEBUG == 0,
        _ => true,
    };
    if !mapped {
        return None;
    }
    let name = section_name(sec);
    if crate::entry::is_executable(sec) {
        return Some(Placement::Code { section: Some(name) });
    }
    let end = start.saturating_add(size);
    let executable = segments.iter().any(|&(lo, hi)| start >= lo && end <= hi);
    if executable && may_hold_code(sec) {
        return Some(Placement::Unflagged(UnflaggedSection { name, start, end }));
    }
    Some(Placement::Data { section: name, executable })
}

/// Can this section's type hold instructions at all? ELF: allocated
/// `SHT_PROGBITS` that is neither TLS nor a merged-string section. Mach-O:
/// `S_REGULAR`, `S_SYMBOL_STUBS` or `S_COALESCED`; the literal, pointer,
/// zero-fill and thread-local types hold data by definition.
fn may_hold_code(sec: &object::Section) -> bool {
    const SECTION_TYPE: u32 = 0xff;
    const S_REGULAR: u32 = 0x0;
    const S_SYMBOL_STUBS: u32 = 0x8;
    const S_COALESCED: u32 = 0xb;
    match sec.flags() {
        SectionFlags::Elf { .. } => matches!(sec.kind(), SectionKind::Data | SectionKind::ReadOnlyData),
        SectionFlags::MachO { flags } => {
            matches!(flags & SECTION_TYPE, S_REGULAR | S_SYMBOL_STUBS | S_COALESCED)
        }
        _ => false,
    }
}

/// `__TEXT,__const` for a Mach-O section (the spelling `nm -m` uses), the
/// section's own name otherwise, and its address when it has none.
fn section_name(sec: &object::Section) -> String {
    let name = sec.name().unwrap_or_default();
    match sec.segment_name() {
        Ok(Some(segment)) if matches!(sec.flags(), SectionFlags::MachO { .. }) => {
            format!("{segment},{name}")
        }
        _ if name.is_empty() => format!("the unnamed section at 0x{:x}", sec.address()),
        _ => name.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture(name: &str) -> Vec<u8> {
        std::fs::read(format!("{}/tests/fixtures/{name}", env!("CARGO_MANIFEST_DIR"))).unwrap()
    }

    /// `__TEXT,__const` without an instruction attribute in the `r-x` `__TEXT`
    /// is unflagged code memory; a `__cstring` beside it is data by its type.
    #[test]
    fn a_macho_const_section_in_text_is_unflagged() {
        let bytes = fixture("unflagged_macho_arm64");
        let file = object::File::parse(&*bytes).unwrap();
        let section = UnflaggedSection { name: "__TEXT,__const".into(), start: 0x1000003fc, end: 0x100000414 };
        assert_eq!(unflagged_sections(&file), [section.clone()]);
        assert_eq!(placement(&file, 0x1000003d8), Placement::Code { section: Some("__TEXT,__text".into()) });
        assert_eq!(placement(&file, 0x10000040c), Placement::Unflagged(section));
        assert_eq!(
            placement(&file, 0x100000414),
            Placement::Data { section: "__TEXT,__cstring".into(), executable: true }
        );
        assert_eq!(
            placement(&file, 0x100004000),
            Placement::Data { section: "__DATA,__data".into(), executable: false }
        );
        assert_eq!(placement(&file, 0x100000000), Placement::NoSection);
        assert_eq!((declared_data(&file, 0x1000003fc), declared_function(&file, 0x1000003fc)), (None, None));
    }

    /// `.rodata` in the `R E` `PT_LOAD` is unflagged; the symbol table declares
    /// `table` data and `lonely` a function.
    #[test]
    fn an_elf_rodata_in_the_text_segment_is_unflagged() {
        let bytes = fixture("unflagged_elf_x86_64");
        let file = object::File::parse(&*bytes).unwrap();
        let section = UnflaggedSection { name: ".rodata".into(), start: 0x4000c2, end: 0x4000d8 };
        assert_eq!(placement(&file, 0x4000d3), Placement::Unflagged(section));
        assert_eq!(placement(&file, 0x4000b0), Placement::Code { section: Some(".text".into()) });
        assert_eq!(placement(&file, 0x401000), Placement::Data { section: ".data".into(), executable: false });
        assert_eq!(declared_data(&file, 0x4000ce).as_deref(), Some("table"));
        assert_eq!(declared_function(&file, 0x4000c8).as_deref(), Some("lonely"));
        assert_eq!((declared_data(&file, 0x4000d3), declared_function(&file, 0x4000d3)), (None, None));
    }

    /// A PE and a separate-code ELF have no gap. An older GNU ld image keeps its
    /// read-only data in the `R E` segment, and every Mach-O its unwind tables
    /// in `__TEXT`.
    #[test]
    fn the_gap_is_a_layout_property() {
        for name in ["pe_imports.exe", "cet_pie_x86_64"] {
            let bytes = fixture(name);
            let file = object::File::parse(&*bytes).unwrap();
            assert!(unflagged_sections(&file).is_empty(), "{name}: {:?}", unflagged_sections(&file));
        }
        for (name, expect) in [
            ("fauxware", &[".interp", ".rodata", ".eh_frame_hdr", ".eh_frame"][..]),
            ("macho_datasyms_arm64", &["__TEXT,__unwind_info"][..]),
        ] {
            let bytes = fixture(name);
            let file = object::File::parse(&*bytes).unwrap();
            let names: Vec<String> = unflagged_sections(&file).into_iter().map(|s| s.name).collect();
            assert_eq!(names, expect, "{name}");
        }
    }
}
