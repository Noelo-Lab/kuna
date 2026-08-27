//! (kuna) Relocatable-object (`.o` / `.obj`) load-layout + relocation engine —
//! the loader-tier capability angr's CLE `ELF` backend has and kuna lacked.
//!
//! ## Why this exists
//!
//! A linked image (`ET_EXEC`/`ET_DYN`, a PE) says where its bytes live, so the
//! faithful [`crate::loadimage_object`] backend builds its byte map from
//! `file.segments()`.  A **relocatable object** — the `.o`/`.obj` a compiler
//! emits before link — does not, and each format fails that differently:
//!
//! - an ELF `ET_REL` has *no program headers* (`readelf -l ptx.o` → "There are no
//!   program headers in this file"), so `file.segments()` is empty, the byte map
//!   is empty, and **every** function fails to lift (`Unable to load N bytes`);
//! - a COFF `.obj` *does* present its sections as segments, but every one of them
//!   at VMA 0 — so the byte map is whichever section happens to sort first, and
//!   with MSVC function-level linking (`/Gy`, one COMDAT `.text` per function)
//!   all but one function collapse onto address 0 and vanish.
//!
//! angr's CLE loader instead lays the memory-resident sections out at a synthetic
//! base, applies the relocations, and binds the symbols, so an object decompiles
//! fully.  This module reproduces exactly that, format-neutrally: which sections
//! are memory-resident is the [`ObjectFormat::is_alloc_section`] question, and
//! everything below it is shared.
//!
//! ## What it does (the angr CLE relocatable path, distilled)
//!
//! 1. **Section layout.** Each memory-resident section (`.text`, `.text.startup`,
//!    `.rodata*`, `.data*`, `.bss`, and every COFF COMDAT `.text$mn`) is assigned
//!    a non-overlapping load VMA above [`RELOC_BASE`] (`0x400000`, matching angr's
//!    default so the testcase's `0x400660` lines up), respecting each section's
//!    alignment.  Initialized bytes are snapshotted; `NOBITS` (`.bss`) is
//!    zero-filled.
//! 2. **Relocations.** For each laid-out section, every relocation that applies
//!    to it is resolved and patched into the snapshotted bytes. Generic
//!    absolute, relative, PLT-relative, and image-offset fields support
//!    8/16/32/64-bit widths in the object's byte order. [`super::reloc_apply`]
//!    additionally decodes ARM branch/data fields, AArch64 branch/page/low-12
//!    fields, and PowerPC64 `REL24`/TOC fields without overwriting opcode bits.
//!    `A` is the entry's addend for a RELA table and the in-place field value for
//!    a REL-style one (COFF, 32-bit ELF). Entries that cannot be applied are left
//!    untouched and grouped by architecture, relocation type, and failure
//!    reason. One bounded report per public load carries exact counts, at most
//!    eight groups, and at most three samples per group.
//! 3. **Symbol rebasing + externs.** Each defined function symbol is shifted from
//!    its section-relative `st_value` to `section_load_vma + st_value`; each
//!    *undefined* referenced symbol (an external like `xmalloc`/`strlen`) is
//!    assigned a synthetic address in an "extern" area above the laid-out
//!    sections (the angr extern-object analog) and, when it is a function,
//!    registered as a function symbol so the call renders by name.
//!
//! The result is fed back into [`crate::loadimage_object::ObjectLoadImage`] as the
//! same `(segments, sections, funcsyms)` triple the linked path produces, so the
//! rest of the loader (`load_fill`, `getNextSection`, `getReadonly`,
//! `getNextSymbol`) is unchanged.

use std::collections::HashMap;

use object::read::{Object, ObjectSection, ObjectSymbol};
use object::{
    Architecture, RelocationFlags, RelocationTarget, SectionIndex, SectionKind, SymbolIndex,
    SymbolKind, SymbolSection,
};

use super::format::ObjectFormat;
use super::reloc_apply::{self, RelocationFailure};

/// Synthetic load base for the first `SHF_ALLOC` section (angr's CLE default for
/// a relocatable object, so a `.o` lifted by kuna and by angr share addresses).
pub const RELOC_BASE: u64 = 0x40_0000;

/// One section in the synthetic relocatable-object address space.
///
/// The object-file coordinates are retained alongside the synthetic VMA so
/// front-ends can present stable selectors such as `.text+0x24` without
/// exposing the loader's implementation address as though it came from the
/// input file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelocSectionInfo {
    pub index: usize,
    pub name: String,
    pub vma: u64,
    pub size: u64,
}

/// Provenance for a function symbol in a relocatable object.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelocSymbolInfo {
    pub name: String,
    pub vma: u64,
    pub section_index: Option<usize>,
    pub section_name: Option<String>,
    pub section_offset: Option<u64>,
    pub binding: String,
    pub undefined: bool,
}

const DIAGNOSTIC_SAMPLE_LIMIT: usize = 3;
const DIAGNOSTIC_GROUP_LIMIT: usize = 8;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct RelocDiagnosticKey {
    architecture: Architecture,
    r_type: Option<u32>,
    reason: RelocationFailure,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RelocDiagnosticGroup {
    key: RelocDiagnosticKey,
    count: usize,
    samples: Vec<(u64, u64)>,
}

/// Bounded-detail relocation failures. Counts are exact, while only a few
/// addresses per architecture/type/reason group are retained for display.
#[derive(Debug, Clone, Default)]
pub struct RelocDiagnostics {
    groups: HashMap<RelocDiagnosticKey, RelocDiagnosticGroup>,
    total: usize,
}

impl RelocDiagnostics {
    fn record(
        &mut self,
        architecture: Architecture,
        r_type: Option<u32>,
        reason: RelocationFailure,
        section_vma: u64,
        offset: u64,
    ) {
        self.total += 1;
        let key = RelocDiagnosticKey {
            architecture,
            r_type,
            reason,
        };
        let group = self
            .groups
            .entry(key.clone())
            .or_insert_with(|| RelocDiagnosticGroup {
                key,
                count: 0,
                samples: Vec::new(),
            });
        group.count += 1;
        if group.samples.len() < DIAGNOSTIC_SAMPLE_LIMIT {
            group.samples.push((section_vma, offset));
        }
    }

    pub fn is_empty(&self) -> bool {
        self.total == 0
    }

    pub fn total(&self) -> usize {
        self.total
    }

    /// Render a fixed-size diagnostic report. The number of relocation records
    /// affects counts only, never the number of emitted stderr lines.
    pub fn report_lines(&self) -> Vec<String> {
        if self.is_empty() {
            return Vec::new();
        }
        let mut groups: Vec<&RelocDiagnosticGroup> = self.groups.values().collect();
        groups.sort_by_key(|group| {
            (
                format!("{:?}", group.key.architecture),
                group.key.r_type,
                group.key.reason,
            )
        });
        let shown = groups.len().min(DIAGNOSTIC_GROUP_LIMIT);
        let hidden = groups.len() - shown;
        let mut lines = vec![format!(
            "{} relocation(s) skipped in {} group(s); showing {} group(s), {} suppressed",
            self.total,
            groups.len(),
            shown,
            hidden
        )];
        for group in groups.into_iter().take(DIAGNOSTIC_GROUP_LIMIT) {
            let kind = group
                .key
                .r_type
                .map(|r_type| {
                    format!(
                        "{} ({r_type})",
                        relocation_name(group.key.architecture, r_type)
                    )
                })
                .unwrap_or_else(|| "non-ELF".to_string());
            let samples = group
                .samples
                .iter()
                .map(|(base, offset)| format!("{base:#x}+{offset:#x}"))
                .collect::<Vec<_>>()
                .join(", ");
            lines.push(format!(
                "arch={:?} type={kind} reason={} count={} samples=[{}] sample(s)-suppressed={}",
                group.key.architecture,
                group.key.reason.label(),
                group.count,
                samples,
                group.count.saturating_sub(group.samples.len())
            ));
        }
        lines
    }
}

/// The laid-out image of a relocatable object: the same three streams the linked
/// `PT_LOAD` path produces, ready to drop into `ObjectLoadImage`.
pub struct RelocLayout {
    /// Loadable regions `(vma, bytes)` — one per laid-out `SHF_ALLOC` section,
    /// relocations already applied.  Feeds `ObjectLoadImage::segments`.
    pub segments: Vec<(u64, Vec<u8>)>,
    /// `(vma, size, flags)` per laid-out section, for the `getNextSection` /
    /// `getReadonly` info walks.  Feeds `ObjectLoadImage::sections`.
    pub sections: Vec<(u64, u64, u32)>,
    /// `(addr, name)` function symbols — defined functions rebased to their load
    /// VMA plus each external call target.  Feeds `ObjectLoadImage::funcsyms`
    /// (demangle + dedup happen there, as on the linked path).
    pub funcsyms: Vec<(u64, Vec<u8>)>,
    /// Non-fatal relocation diagnostics, aggregated with bounded samples. The
    /// caller prints one report for the load; analysis-side layout reuse stays
    /// silent.
    pub diagnostics: RelocDiagnostics,
    /// The section -> load-VMA map the layout assigned, keyed by the parsed
    /// object's own [`SectionIndex`].  A section absent from this map was NOT
    /// laid out (a `.debug_*`/`.rela.*` table, or an empty `SHF_ALLOC`
    /// placeholder) and has no address in the loaded image.  Exposed so the
    /// analysis tier can rebase a pre-link fact through its OWN section's delta
    /// (see [`crate::loader::kuna_relocrebase`]) — sections are laid out
    /// non-contiguously, so there is no single global offset.
    pub section_vma: HashMap<SectionIndex, u64>,
    /// The undefined-symbol -> synthetic extern-slot map, keyed by the parsed
    /// object's own [`SymbolIndex`].  Same purpose as [`Self::section_vma`], for
    /// the externs.
    pub extern_addr: HashMap<SymbolIndex, u64>,
    /// The half-open `[lo, hi)` extent of the synthetic extern area, `None` when
    /// the object referenced no undefined symbol.
    pub extern_range: Option<(u64, u64)>,
    /// Public object-coordinate map for selectors and machine-readable output.
    pub section_info: Vec<RelocSectionInfo>,
    /// Defined and undefined callable symbols with their original provenance.
    pub symbol_info: Vec<RelocSymbolInfo>,
}

/// One laid-out `SHF_ALLOC` section: its load VMA and a *mutable* byte buffer the
/// relocation pass patches in place.
struct LaidSection {
    index: SectionIndex,
    vma: u64,
    size: u64,
    flags: u32,
    data: Vec<u8>,
}

#[derive(Debug, Clone, Copy)]
struct ResolvedRelocationTarget {
    address: u64,
    /// Defined `STT_FUNC` symbols carry their ARM/Thumb state in bit zero.
    /// Undefined extern slots and section targets deliberately remain unknown:
    /// their synthetic address must not invent an execution state.
    thumb: Option<bool>,
}

/// Lay a relocatable object (an ELF `ET_REL` or a COFF `.obj`) out into a
/// loadable image: synthesize the section layout, apply the relocations, and
/// rebase / extern-bind the symbols.  The kuna analog of angr CLE's `ELF`
/// relocatable backend.
///
/// `fmt` supplies the two per-format questions — which sections are memory-
/// resident ([`ObjectFormat::is_alloc_section`]) and the section-flag translation
/// (`section_bits`, identical to the linked path, so `.rodata` lands read-only
/// (string literals fold) and `.text` lands as code).
pub fn layout_relocatable(file: &object::File, fmt: &dyn ObjectFormat) -> RelocLayout {
    let mut diagnostics = RelocDiagnostics::default();
    let architecture = file.architecture();
    let little_endian = file.is_little_endian();

    // --- Pass 1: assign a load VMA to every SHF_ALLOC section, snapshot bytes.
    let mut laid: Vec<LaidSection> = Vec::new();
    let mut vma_of: HashMap<SectionIndex, u64> = HashMap::new();
    let mut section_info: Vec<RelocSectionInfo> = Vec::new();
    let mut cursor = RELOC_BASE;
    for sec in file.sections() {
        if !fmt.is_alloc_section(sec.kind(), sec.flags()) {
            continue; // not mapped at run time (.rela.*, .symtab, .debug_*, …)
        }
        let size = sec.size();
        if size == 0 {
            // An empty section occupies no memory; laying it out would alias the
            // next section's VMA (a COFF object carries empty `.text`/`.data`/`.bss`
            // placeholders ahead of its per-function COMDATs) for no gain.
            continue;
        }
        let align = sec.align().max(1);
        let vma = align_up(cursor, align);
        // PROGBITS bytes verbatim; NOBITS (.bss) zero-filled to its RAM size.
        let data: Vec<u8> = if matches!(sec.kind(), SectionKind::UninitializedData) {
            vec![0u8; size as usize]
        } else {
            match sec.data() {
                Ok(d) => {
                    let mut v = d.to_vec();
                    // A short file extent (rare) is zero-padded to the RAM size.
                    if (v.len() as u64) < size {
                        v.resize(size as usize, 0);
                    }
                    v
                }
                Err(_) => vec![0u8; size as usize],
            }
        };
        vma_of.insert(sec.index(), vma);
        section_info.push(RelocSectionInfo {
            index: sec.index().0,
            name: sec.name().unwrap_or("").to_string(),
            vma,
            size,
        });
        laid.push(LaidSection {
            index: sec.index(),
            vma,
            size,
            flags: fmt.section_bits(sec.kind(), sec.flags()),
            data,
        });
        cursor = vma.wrapping_add(size);
    }

    // Externs (undefined referenced symbols) live in a synthetic area above the
    // laid-out sections — the angr extern-object analog.  Allocated on demand.
    let mut extern_cursor = align_up(cursor.wrapping_add(0x1000), 0x1000);
    let mut extern_of: HashMap<SymbolIndex, u64> = HashMap::new();
    let mut extern_order: Vec<(SymbolIndex, u64)> = Vec::new();
    // Externs reached through a PLT-relative relocation (`R_X86_64_PLT32`) are
    // definitely *call targets* — even when the undefined symbol is `STT_NOTYPE`
    // (object `SymbolKind::Unknown`), which gcc emits for plain `extern` calls.
    // Used to register them as function symbols so the call renders by name.
    let mut call_externs: std::collections::HashSet<SymbolIndex> = std::collections::HashSet::new();

    // ELFv2 defines `.TOC.` as `.toc + 0x8000`. Split TOC relocations use that
    // base rather than the target section's own VMA.
    let toc = file
        .sections()
        .find(|section| section.name().ok() == Some(".toc"))
        .and_then(|section| vma_of.get(&section.index()).copied())
        .map(|base| base.wrapping_add(0x8000));

    // --- Pass 2: apply relocations, patching each laid-out section's bytes.
    for li in 0..laid.len() {
        let sec_index = laid[li].index;
        let sec_vma = laid[li].vma;
        let Ok(sec) = file.section_by_index(sec_index) else {
            continue;
        };
        // Collect first so the `laid` buffer can be borrowed mutably below.
        let relocs: Vec<(u64, object::Relocation)> = sec.relocations().collect();
        for (offset, reloc) in relocs {
            let r_type = match reloc.flags() {
                RelocationFlags::Elf { r_type } => Some(r_type),
                _ => None,
            };
            let spec = match reloc_apply::classify(architecture, &reloc) {
                Ok(Some(spec)) => spec,
                Ok(None) => continue,
                Err(reason) => {
                    diagnostics.record(architecture, r_type, reason, sec_vma, offset);
                    continue;
                }
            };
            let off = offset as usize;
            let Some(end) = off.checked_add(spec.width()) else {
                diagnostics.record(
                    architecture,
                    r_type,
                    RelocationFailure::PastSectionEnd,
                    sec_vma,
                    offset,
                );
                continue;
            };
            if end > laid[li].data.len() {
                diagnostics.record(
                    architecture,
                    r_type,
                    RelocationFailure::PastSectionEnd,
                    sec_vma,
                    offset,
                );
                continue;
            }
            let spec = spec.refine_call_from_field(&laid[li].data[off..end], little_endian);
            // S — the resolved value of the relocation's target.
            let target = if !spec.requires_symbol() {
                Some(ResolvedRelocationTarget {
                    address: 0,
                    thumb: None,
                })
            } else {
                match reloc.target() {
                    RelocationTarget::Symbol(sym_idx) => {
                        // Instruction calls mark even STT_NOTYPE undefined symbols
                        // as callable externs, including r_types object leaves Unknown.
                        if spec.is_call() {
                            call_externs.insert(sym_idx);
                        }
                        resolve_symbol(
                            file,
                            sym_idx,
                            &vma_of,
                            &mut extern_of,
                            &mut extern_order,
                            &mut extern_cursor,
                            architecture,
                        )
                    }
                    RelocationTarget::Section(sidx) => {
                        vma_of.get(&sidx).map(|address| ResolvedRelocationTarget {
                            address: *address,
                            thumb: None,
                        })
                    }
                    _ => None,
                }
            };
            let Some(target) = target else {
                diagnostics.record(
                    architecture,
                    r_type,
                    RelocationFailure::UnresolvedTarget,
                    sec_vma,
                    offset,
                );
                continue;
            };
            let place = sec_vma.wrapping_add(offset);
            if let Err(reason) = reloc_apply::apply(
                spec,
                &reloc,
                &mut laid[li].data[off..end],
                little_endian,
                target.address,
                place,
                target.thumb,
                toc,
            ) {
                diagnostics.record(architecture, r_type, reason, sec_vma, offset);
            }
        }
    }

    // --- Function symbols: defined (rebased) + extern call targets.
    let mut funcsyms: Vec<(u64, Vec<u8>)> = Vec::new();
    let mut symbol_info: Vec<RelocSymbolInfo> = Vec::new();
    for sym in file.symbols() {
        if sym.kind() != SymbolKind::Text {
            continue;
        }
        let SymbolSection::Section(sec_idx) = sym.section() else {
            continue;
        };
        let Some(&base) = vma_of.get(&sec_idx) else {
            continue;
        };
        let name = match sym.name_bytes() {
            Ok(n) if !n.is_empty() => n.to_vec(),
            _ => continue,
        };
        let vma = base.wrapping_add(sym.address());
        let section_name = file
            .section_by_index(sec_idx)
            .ok()
            .and_then(|section| section.name().ok())
            .unwrap_or("")
            .to_string();
        symbol_info.push(RelocSymbolInfo {
            name: String::from_utf8_lossy(&name).into_owned(),
            vma,
            section_index: Some(sec_idx.0),
            section_name: Some(section_name),
            section_offset: Some(sym.address()),
            binding: symbol_binding(&sym),
            undefined: false,
        });
        funcsyms.push((vma, name));
    }
    // Extern functions: name each external *call target* at its synthetic slot
    // so `call ext` renders by name instead of a bare address. A call target is
    // an undefined symbol reached through a PLT-relative reloc (reliable even for
    // the `STT_NOTYPE` symbols gcc emits for `extern` calls) or one typed
    // `STT_FUNC`. Pure data externs (e.g. `stdout`, referenced by PC32 to an
    // `STT_OBJECT`) are addressed but deliberately left unnamed here.
    for (sym_idx, addr) in &extern_order {
        let Ok(sym) = file.symbol_by_index(*sym_idx) else {
            continue;
        };
        let is_func = call_externs.contains(sym_idx) || sym.kind() == SymbolKind::Text;
        if !is_func {
            continue;
        }
        if let Ok(n) = sym.name_bytes() {
            if !n.is_empty() {
                symbol_info.push(RelocSymbolInfo {
                    name: String::from_utf8_lossy(n).into_owned(),
                    vma: *addr,
                    section_index: None,
                    section_name: None,
                    section_offset: None,
                    binding: symbol_binding(&sym),
                    undefined: true,
                });
                funcsyms.push((*addr, n.to_vec()));
            }
        }
    }

    let segments = laid.iter().map(|l| (l.vma, l.data.clone())).collect();
    let sections = laid.iter().map(|l| (l.vma, l.size, l.flags)).collect();
    let extern_range = extern_order
        .iter()
        .map(|(_, a)| *a)
        .min()
        .map(|lo| (lo, extern_cursor));
    RelocLayout {
        segments,
        sections,
        funcsyms,
        diagnostics,
        section_vma: vma_of,
        extern_addr: extern_of,
        extern_range,
        section_info,
        symbol_info,
    }
}

fn symbol_binding<'data, S: ObjectSymbol<'data>>(sym: &S) -> String {
    if sym.is_weak() {
        "weak"
    } else if sym.is_local() {
        "local"
    } else if sym.is_global() {
        "global"
    } else {
        "unknown"
    }
    .to_string()
}

/// Resolve a relocation's target symbol to a load address.  A defined symbol maps
/// to `section_load_vma + st_value`; an *undefined* symbol is bound to a fresh
/// synthetic extern slot (allocated on first reference and reused after).
/// Absolute symbols pass through.  `None` for a symbol in a non-laid section
/// (e.g. a `.debug_*`-only symbol) — the caller warns and skips.
fn resolve_symbol(
    file: &object::File,
    idx: SymbolIndex,
    vma_of: &HashMap<SectionIndex, u64>,
    extern_of: &mut HashMap<SymbolIndex, u64>,
    extern_order: &mut Vec<(SymbolIndex, u64)>,
    extern_cursor: &mut u64,
    architecture: Architecture,
) -> Option<ResolvedRelocationTarget> {
    let sym = file.symbol_by_index(idx).ok()?;
    match sym.section() {
        SymbolSection::Section(sec_idx) => {
            vma_of.get(&sec_idx).map(|base| ResolvedRelocationTarget {
                address: base.wrapping_add(sym.address()),
                thumb: (architecture == Architecture::Arm && sym.kind() == SymbolKind::Text)
                    .then(|| sym.address() & 1 != 0),
            })
        }
        SymbolSection::Undefined | SymbolSection::Common => {
            if let Some(&a) = extern_of.get(&idx) {
                return Some(ResolvedRelocationTarget {
                    address: a,
                    thumb: None,
                });
            }
            let a = *extern_cursor;
            *extern_cursor = extern_cursor.wrapping_add(16);
            extern_of.insert(idx, a);
            extern_order.push((idx, a));
            Some(ResolvedRelocationTarget {
                address: a,
                thumb: None,
            })
        }
        SymbolSection::Absolute => Some(ResolvedRelocationTarget {
            address: sym.address(),
            thumb: (architecture == Architecture::Arm && sym.kind() == SymbolKind::Text)
                .then(|| sym.address() & 1 != 0),
        }),
        _ => None,
    }
}

/// Whether this object is one the loader lays out synthetically — i.e. whether
/// the addresses the raw [`object`] view reports are **pre-link** ones, in a
/// different address space than every address the engine holds.
///
/// The analysis tier parses the object independently of the loader, so a pass
/// that reads a raw section/symbol address and hands it to a tier that decodes
/// through the *loaded* image is making a category error: on a relocatable object
/// it seeds `.text`+0x60 rather than the rebased `0x400060`, inventing a phantom
/// `sub_60` beside the real function. Passes that would do that ask here first.
pub fn is_synthetically_laid_out(file: &object::File) -> bool {
    crate::loadimage_object::reloc_objects_enabled()
        && crate::loader::format::detect(file).is_ok_and(|f| f.relocatable_layout(file))
}

fn relocation_name(architecture: Architecture, r_type: u32) -> &'static str {
    match (architecture, r_type) {
        (Architecture::Arm, object::elf::R_ARM_CALL) => "R_ARM_CALL",
        (Architecture::Arm, object::elf::R_ARM_JUMP24) => "R_ARM_JUMP24",
        (Architecture::Arm, object::elf::R_ARM_THM_PC22) => "R_ARM_THM_CALL",
        (Architecture::Arm, object::elf::R_ARM_THM_JUMP24) => "R_ARM_THM_JUMP24",
        (Architecture::Arm, object::elf::R_ARM_REL32) => "R_ARM_REL32",
        (Architecture::Arm, object::elf::R_ARM_PREL31) => "R_ARM_PREL31",
        (Architecture::Aarch64, object::elf::R_AARCH64_CALL26) => "R_AARCH64_CALL26",
        (Architecture::Aarch64, object::elf::R_AARCH64_JUMP26) => "R_AARCH64_JUMP26",
        (Architecture::Aarch64, object::elf::R_AARCH64_ADR_PREL_PG_HI21) => {
            "R_AARCH64_ADR_PREL_PG_HI21"
        }
        (Architecture::Aarch64, object::elf::R_AARCH64_ADD_ABS_LO12_NC) => {
            "R_AARCH64_ADD_ABS_LO12_NC"
        }
        (Architecture::PowerPc64, object::elf::R_PPC64_REL24) => "R_PPC64_REL24",
        (Architecture::PowerPc64, object::elf::R_PPC64_TOC16_HA) => "R_PPC64_TOC16_HA",
        (Architecture::PowerPc64, object::elf::R_PPC64_TOC16_LO_DS) => "R_PPC64_TOC16_LO_DS",
        (Architecture::PowerPc64, object::elf::R_PPC64_TOC) => "R_PPC64_TOC",
        _ => "ELF relocation",
    }
}

/// Round `value` up to a multiple of `align` (a power of two ≥ 1).
fn align_up(value: u64, align: u64) -> u64 {
    debug_assert!(align >= 1);
    let mask = align - 1;
    value.wrapping_add(mask) & !mask
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::loader::format::elf::ElfFormat;
    use object::write;
    use object::{
        Architecture, BinaryFormat, Endianness, RelocationFlags, SectionKind, SymbolFlags,
        SymbolKind, SymbolScope,
    };

    /// Hand-assemble a minimal little-endian ELF64 **ET_REL** (no program
    /// headers) with one `.text` (ALLOC|EXEC), one `.data` (ALLOC|WRITE), a
    /// `.symtab` (`myfunc`@.text, `datum`@.data, undefined `ext_call`), and a
    /// `.rela.text` exercising one of each relocation kind the corpus uses:
    ///   off 4  : R_X86_64_PC32  -> datum   (addend -4)
    ///   off 10 : R_X86_64_PLT32 -> ext_call(addend -4)
    ///   off 16 : R_X86_64_64    -> datum   (addend  0)
    ///   off 24 : R_X86_64_32    -> datum   (addend  0)
    /// Needs no external toolchain.
    fn build_et_rel() -> Vec<u8> {
        // Section header indices (must match shndx / link / info below):
        //   0 null, 1 .text, 2 .data, 3 .symtab, 4 .strtab, 5 .rela.text, 6 .shstrtab
        const TEXT: usize = 32;
        let text = vec![0u8; TEXT];
        let data = vec![0u8; 8];

        // --- .strtab (symbol names) -------------------------------------
        let mut strtab = vec![0u8];
        let push_str = |s: &mut Vec<u8>, n: &str| {
            let off = s.len() as u32;
            s.extend_from_slice(n.as_bytes());
            s.push(0);
            off
        };
        let n_myfunc = push_str(&mut strtab, "myfunc");
        let n_datum = push_str(&mut strtab, "datum");
        let n_extcall = push_str(&mut strtab, "ext_call");

        // --- .symtab (Elf64_Sym = 24 bytes) -----------------------------
        let mut symtab: Vec<u8> = vec![0u8; 24]; // [0] null
        let push_sym = |s: &mut Vec<u8>, name: u32, info: u8, shndx: u16, value: u64| {
            s.extend_from_slice(&name.to_le_bytes()); // st_name
            s.push(info); // st_info
            s.push(0); // st_other
            s.extend_from_slice(&shndx.to_le_bytes()); // st_shndx
            s.extend_from_slice(&value.to_le_bytes()); // st_value
            s.extend_from_slice(&0u64.to_le_bytes()); // st_size
        };
        // STB_GLOBAL<<4 | STT_FUNC(2) = 0x12 ; STT_OBJECT(1) = 0x11
        push_sym(&mut symtab, n_myfunc, 0x12, 1, 0); // [1] myfunc @ .text+0
        push_sym(&mut symtab, n_datum, 0x11, 2, 0); // [2] datum  @ .data+0
        push_sym(&mut symtab, n_extcall, 0x12, 0, 0); // [3] ext_call UNDEF

        // --- .rela.text (Elf64_Rela = 24 bytes) -------------------------
        let mut rela: Vec<u8> = Vec::new();
        let push_rela = |s: &mut Vec<u8>, off: u64, sym: u64, ty: u64, addend: i64| {
            s.extend_from_slice(&off.to_le_bytes());
            s.extend_from_slice(&((sym << 32) | ty).to_le_bytes());
            s.extend_from_slice(&addend.to_le_bytes());
        };
        push_rela(&mut rela, 4, 2, 2, -4); // PC32  datum
        push_rela(&mut rela, 10, 3, 4, -4); // PLT32 ext_call
        push_rela(&mut rela, 16, 2, 1, 0); // 64    datum
        push_rela(&mut rela, 24, 2, 10, 0); // 32    datum

        // --- .shstrtab (section names) ----------------------------------
        let mut shstr = vec![0u8];
        let s_text = push_str(&mut shstr, ".text");
        let s_data = push_str(&mut shstr, ".data");
        let s_symtab = push_str(&mut shstr, ".symtab");
        let s_strtab = push_str(&mut shstr, ".strtab");
        let s_rela = push_str(&mut shstr, ".rela.text");
        let s_shstr = push_str(&mut shstr, ".shstrtab");

        // --- file body offsets ------------------------------------------
        let mut buf: Vec<u8> = Vec::new();
        let ehdr_size = 64u64;
        buf.resize(ehdr_size as usize, 0); // Ehdr filled in last
        let off_text = buf.len() as u64;
        buf.extend_from_slice(&text);
        let off_data = buf.len() as u64;
        buf.extend_from_slice(&data);
        let off_symtab = buf.len() as u64;
        buf.extend_from_slice(&symtab);
        let off_strtab = buf.len() as u64;
        buf.extend_from_slice(&strtab);
        let off_rela = buf.len() as u64;
        buf.extend_from_slice(&rela);
        let off_shstr = buf.len() as u64;
        buf.extend_from_slice(&shstr);
        // align section headers to 8
        while buf.len() % 8 != 0 {
            buf.push(0);
        }
        let sh_off = buf.len() as u64;

        // --- section headers (Elf64_Shdr = 64 bytes) --------------------
        #[allow(clippy::too_many_arguments)]
        let push_shdr = |b: &mut Vec<u8>,
                         name: u32,
                         sh_type: u32,
                         sh_flags: u64,
                         addr: u64,
                         offset: u64,
                         size: u64,
                         link: u32,
                         info: u32,
                         align: u64,
                         entsize: u64| {
            b.extend_from_slice(&name.to_le_bytes());
            b.extend_from_slice(&sh_type.to_le_bytes());
            b.extend_from_slice(&sh_flags.to_le_bytes());
            b.extend_from_slice(&addr.to_le_bytes());
            b.extend_from_slice(&offset.to_le_bytes());
            b.extend_from_slice(&size.to_le_bytes());
            b.extend_from_slice(&link.to_le_bytes());
            b.extend_from_slice(&info.to_le_bytes());
            b.extend_from_slice(&align.to_le_bytes());
            b.extend_from_slice(&entsize.to_le_bytes());
        };
        // SHF_WRITE=1, SHF_ALLOC=2, SHF_EXECINSTR=4, SHF_INFO_LINK=0x40
        push_shdr(&mut buf, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0); // [0] null
        push_shdr(
            &mut buf,
            s_text,
            1,
            0x2 | 0x4,
            0,
            off_text,
            TEXT as u64,
            0,
            0,
            16,
            0,
        ); // [1] .text
        push_shdr(&mut buf, s_data, 1, 0x2 | 0x1, 0, off_data, 8, 0, 0, 8, 0); // [2] .data
        push_shdr(
            &mut buf,
            s_symtab,
            2,
            0,
            0,
            off_symtab,
            symtab.len() as u64,
            4,
            1,
            8,
            24,
        ); // [3] .symtab
        push_shdr(
            &mut buf,
            s_strtab,
            3,
            0,
            0,
            off_strtab,
            strtab.len() as u64,
            0,
            0,
            1,
            0,
        ); // [4] .strtab
        push_shdr(
            &mut buf,
            s_rela,
            4,
            0x40,
            0,
            off_rela,
            rela.len() as u64,
            3,
            1,
            8,
            24,
        ); // [5] .rela.text
        push_shdr(
            &mut buf,
            s_shstr,
            3,
            0,
            0,
            off_shstr,
            shstr.len() as u64,
            0,
            0,
            1,
            0,
        ); // [6] .shstrtab

        // --- Ehdr -------------------------------------------------------
        let mut e: Vec<u8> = Vec::new();
        e.extend_from_slice(&[0x7f, b'E', b'L', b'F']);
        e.push(2); // ELFCLASS64
        e.push(1); // ELFDATA2LSB
        e.push(1); // version
        e.extend_from_slice(&[0u8; 9]); // OSABI + pad
        e.extend_from_slice(&1u16.to_le_bytes()); // e_type = ET_REL
        e.extend_from_slice(&62u16.to_le_bytes()); // e_machine = EM_X86_64
        e.extend_from_slice(&1u32.to_le_bytes()); // e_version
        e.extend_from_slice(&0u64.to_le_bytes()); // e_entry
        e.extend_from_slice(&0u64.to_le_bytes()); // e_phoff = 0 (no phdrs)
        e.extend_from_slice(&sh_off.to_le_bytes()); // e_shoff
        e.extend_from_slice(&0u32.to_le_bytes()); // e_flags
        e.extend_from_slice(&64u16.to_le_bytes()); // e_ehsize
        e.extend_from_slice(&0u16.to_le_bytes()); // e_phentsize
        e.extend_from_slice(&0u16.to_le_bytes()); // e_phnum
        e.extend_from_slice(&64u16.to_le_bytes()); // e_shentsize
        e.extend_from_slice(&7u16.to_le_bytes()); // e_shnum
        e.extend_from_slice(&6u16.to_le_bytes()); // e_shstrndx
        buf[..64].copy_from_slice(&e[..64]);
        buf
    }

    fn le32(b: &[u8]) -> u32 {
        u32::from_le_bytes([b[0], b[1], b[2], b[3]])
    }
    fn le64(b: &[u8]) -> u64 {
        let mut a = [0u8; 8];
        a.copy_from_slice(&b[..8]);
        u64::from_le_bytes(a)
    }

    #[test]
    fn et_rel_layout_relocations_and_symbols() {
        let bytes = build_et_rel();
        let file = object::File::parse(&*bytes).expect("parse ET_REL");
        assert_eq!(file.kind(), object::ObjectKind::Relocatable);
        assert!(file.segments().next().is_none(), "ET_REL has no PT_LOAD");

        let layout = layout_relocatable(&file, &ElfFormat);
        assert!(
            layout.diagnostics.is_empty(),
            "unexpected diagnostics: {:?}",
            layout.diagnostics
        );

        // .text is the first SHF_ALLOC section -> RELOC_BASE; .data follows it.
        let (text_vma, text) = layout
            .segments
            .iter()
            .find(|(v, _)| *v == RELOC_BASE)
            .map(|(v, d)| (*v, d.clone()))
            .expect(".text at RELOC_BASE");
        assert_eq!(text_vma, 0x40_0000);
        let data_vma = layout
            .segments
            .iter()
            .map(|(v, _)| *v)
            .find(|v| *v != RELOC_BASE)
            .expect(".data laid out");
        assert!(data_vma > text_vma, ".data above .text");

        // Defined function `myfunc` rebased to .text load VMA; `datum` (data) is
        // NOT a function symbol; extern `ext_call` bound above the sections.
        let func = |name: &str| {
            layout
                .funcsyms
                .iter()
                .find(|(_, n)| n == name.as_bytes())
                .map(|(a, _)| *a)
        };
        assert_eq!(func("myfunc"), Some(text_vma), "myfunc rebased");
        assert_eq!(func("datum"), None, "data symbol is not a funcsym");
        let ext = func("ext_call").expect("ext_call bound");
        assert!(ext >= data_vma, "extern above the laid-out sections");

        // PC32 @4 : S + A - P = data_vma - 4 - (text_vma+4)
        assert_eq!(
            le32(&text[4..8]) as i32,
            (data_vma as i64 - 4 - (text_vma as i64 + 4)) as i32
        );
        // PLT32 @10: ext - 4 - (text_vma+10)
        assert_eq!(
            le32(&text[10..14]) as i32,
            (ext as i64 - 4 - (text_vma as i64 + 10)) as i32
        );
        // R_X86_64_64 @16 : S + A = data_vma (8 bytes)
        assert_eq!(le64(&text[16..24]), data_vma);
        // R_X86_64_32 @24 : S + A = data_vma (4 bytes)
        assert_eq!(le32(&text[24..28]) as u64, data_vma);
    }

    fn add_symbol(
        object: &mut write::Object<'_>,
        name: &[u8],
        section: write::SymbolSection,
        value: u64,
        kind: SymbolKind,
    ) -> write::SymbolId {
        object.add_symbol(write::Symbol {
            name: name.to_vec(),
            value,
            size: 0,
            kind,
            scope: SymbolScope::Linkage,
            weak: false,
            section,
            flags: SymbolFlags::None,
        })
    }

    fn add_elf_relocation(
        object: &mut write::Object<'_>,
        section: write::SectionId,
        offset: u64,
        symbol: write::SymbolId,
        r_type: u32,
    ) {
        object
            .add_relocation(
                section,
                write::Relocation {
                    offset,
                    symbol,
                    addend: 0,
                    flags: RelocationFlags::Elf { r_type },
                },
            )
            .expect("add synthetic ELF relocation");
    }

    #[test]
    fn arm_calls_bind_local_and_untyped_external_targets() {
        let mut object =
            write::Object::new(BinaryFormat::Elf, Architecture::Arm, Endianness::Little);
        let text = object.section_id(write::StandardSection::Text);
        object.append_section_data(
            text,
            &[
                0xfe, 0xff, 0xff, 0xeb, // bl, implicit addend -8
                0xfe, 0xff, 0xff, 0xeb, // bl, implicit addend -8
                0x1e, 0xff, 0x2f, 0xe1, // bx lr
            ],
            4,
        );
        let local = add_symbol(
            &mut object,
            b"local_status",
            write::SymbolSection::Section(text),
            8,
            SymbolKind::Text,
        );
        let external = add_symbol(
            &mut object,
            b"external_status",
            write::SymbolSection::Undefined,
            0,
            SymbolKind::Unknown,
        );
        add_elf_relocation(&mut object, text, 0, local, object::elf::R_ARM_CALL);
        add_elf_relocation(&mut object, text, 4, external, object::elf::R_ARM_CALL);

        let bytes = object.write().expect("write synthetic ARM ET_REL");
        let file = object::File::parse(&*bytes).expect("parse synthetic ARM ET_REL");
        let layout = layout_relocatable(&file, &ElfFormat);
        assert!(layout.diagnostics.is_empty(), "{:?}", layout.diagnostics);
        let code = &layout.segments[0].1;
        assert_eq!(le32(&code[..4]), 0xeb00_0000, "local BL reaches .text+8");
        assert_ne!(le32(&code[4..8]) & 0x00ff_ffff, 0x00ff_fffe);
        assert!(
            layout
                .funcsyms
                .iter()
                .any(|(_, name)| name == b"external_status"),
            "untyped CALL target is named as an extern function"
        );
    }

    #[test]
    fn arm_jump24_link_instruction_binds_untyped_external_call() {
        let mut object =
            write::Object::new(BinaryFormat::Elf, Architecture::Arm, Endianness::Little);
        let text = object.section_id(write::StandardSection::Text);
        object.append_section_data(
            text,
            &[
                0xfe, 0xff, 0xff, 0x1b, // blne, implicit addend -8
                0x1e, 0xff, 0x2f, 0xe1, // bx lr
            ],
            4,
        );
        let external = add_symbol(
            &mut object,
            b"external_status",
            write::SymbolSection::Undefined,
            0,
            SymbolKind::Unknown,
        );
        add_elf_relocation(
            &mut object,
            text,
            0,
            external,
            object::elf::R_ARM_JUMP24,
        );

        let bytes = object.write().expect("write synthetic ARM ET_REL");
        let file = object::File::parse(&*bytes).expect("parse synthetic ARM ET_REL");
        let layout = layout_relocatable(&file, &ElfFormat);
        assert!(layout.diagnostics.is_empty(), "{:?}", layout.diagnostics);
        assert_eq!(
            le32(&layout.segments[0].1[..4]) & 0xff00_0000,
            0x1b00_0000,
            "conditional BL opcode must be retained"
        );
        assert!(
            layout
                .funcsyms
                .iter()
                .any(|(_, name)| name == b"external_status"),
            "link-bit JUMP24 target must retain callable external identity"
        );
    }

    #[test]
    fn arm_typed_calls_interwork_in_both_directions() {
        let mut object =
            write::Object::new(BinaryFormat::Elf, Architecture::Arm, Endianness::Little);
        let text = object.section_id(write::StandardSection::Text);
        object.append_section_data(
            text,
            &[
                0x00, 0x00, 0x00, 0xeb, // ARM BL to Thumb target
                0x00, 0xf0, 0x00, 0xf8, // Thumb BL to ARM target
                0x70, 0x47, 0x00, 0xbf, // Thumb target: bx lr; nop
                0x1e, 0xff, 0x2f, 0xe1, // ARM target: bx lr
            ],
            4,
        );
        let thumb_target = add_symbol(
            &mut object,
            b"thumb_target",
            write::SymbolSection::Section(text),
            9,
            SymbolKind::Text,
        );
        let arm_target = add_symbol(
            &mut object,
            b"arm_target",
            write::SymbolSection::Section(text),
            12,
            SymbolKind::Text,
        );
        add_elf_relocation(&mut object, text, 0, thumb_target, object::elf::R_ARM_CALL);
        add_elf_relocation(
            &mut object,
            text,
            4,
            arm_target,
            object::elf::R_ARM_THM_PC22,
        );

        let bytes = object.write().expect("write mixed-state ARM ET_REL");
        let file = object::File::parse(&*bytes).expect("parse mixed-state ARM ET_REL");
        let layout = layout_relocatable(&file, &ElfFormat);
        assert!(layout.diagnostics.is_empty(), "{:?}", layout.diagnostics);
        let code = &layout.segments[0].1;
        assert_eq!(le32(&code[..4]), 0xfa00_0002, "ARM BL became BLX");
        assert_eq!(
            &code[4..8],
            &[0x00, 0xf0, 0x04, 0xe8],
            "Thumb BL became BLX"
        );
    }

    #[test]
    fn aarch64_calls_and_low12_relocations_apply_together() {
        let mut object =
            write::Object::new(BinaryFormat::Elf, Architecture::Aarch64, Endianness::Little);
        let text = object.section_id(write::StandardSection::Text);
        let data = object.section_id(write::StandardSection::Data);
        object.append_section_data(
            text,
            &[
                0x00, 0x00, 0x00, 0x94, // bl local
                0x00, 0x00, 0x00, 0x94, // bl external
                0x00, 0x00, 0x00, 0x90, // adrp x0, data
                0x00, 0x00, 0x00, 0x91, // add x0, x0, data@lo12
                0xc0, 0x03, 0x5f, 0xd6, // ret
            ],
            4,
        );
        object.append_section_data(data, &[0; 16], 16);
        let local = add_symbol(
            &mut object,
            b"local_status",
            write::SymbolSection::Section(text),
            16,
            SymbolKind::Text,
        );
        let external = add_symbol(
            &mut object,
            b"external_status",
            write::SymbolSection::Undefined,
            0,
            SymbolKind::Unknown,
        );
        let datum = add_symbol(
            &mut object,
            b"datum",
            write::SymbolSection::Section(data),
            0,
            SymbolKind::Data,
        );
        add_elf_relocation(&mut object, text, 0, local, object::elf::R_AARCH64_CALL26);
        add_elf_relocation(
            &mut object,
            text,
            4,
            external,
            object::elf::R_AARCH64_CALL26,
        );
        add_elf_relocation(
            &mut object,
            text,
            8,
            datum,
            object::elf::R_AARCH64_ADR_PREL_PG_HI21,
        );
        add_elf_relocation(
            &mut object,
            text,
            12,
            datum,
            object::elf::R_AARCH64_ADD_ABS_LO12_NC,
        );

        let bytes = object.write().expect("write synthetic AArch64 ET_REL");
        let file = object::File::parse(&*bytes).expect("parse synthetic AArch64 ET_REL");
        let layout = layout_relocatable(&file, &ElfFormat);
        assert!(layout.diagnostics.is_empty(), "{:?}", layout.diagnostics);
        let code = &layout.segments[0].1;
        assert_eq!(le32(&code[..4]), 0x9400_0004);
        assert!(layout
            .funcsyms
            .iter()
            .any(|(_, name)| name == b"external_status"));
        assert_eq!(le32(&code[12..16]) & 0x003f_fc00, 0x0000_8000);
    }

    #[test]
    fn ppc64_big_endian_calls_and_toc_family_apply() {
        let mut object =
            write::Object::new(BinaryFormat::Elf, Architecture::PowerPc64, Endianness::Big);
        let text = object.section_id(write::StandardSection::Text);
        let toc_section = object.add_section(Vec::new(), b".toc".to_vec(), SectionKind::Data);
        object.append_section_data(
            text,
            &[
                0x48, 0x00, 0x00, 0x01, // bl local
                0x48, 0x00, 0x00, 0x01, // bl external
                0x4e, 0x80, 0x00, 0x20, // blr
                0x3c, 0x42, 0x00, 0x00, // addis ..., .toc@ha
                0xe8, 0x42, 0x00, 0x02, // ld ..., .toc@l (DS low bits kept)
            ],
            4,
        );
        object.append_section_data(toc_section, &[0; 8], 8);
        let local = add_symbol(
            &mut object,
            b".local_status",
            write::SymbolSection::Section(text),
            8,
            SymbolKind::Text,
        );
        let external = add_symbol(
            &mut object,
            b".external_status",
            write::SymbolSection::Undefined,
            0,
            SymbolKind::Unknown,
        );
        let toc_symbol = add_symbol(
            &mut object,
            b".toc",
            write::SymbolSection::Section(toc_section),
            0,
            SymbolKind::Data,
        );
        add_elf_relocation(&mut object, text, 0, local, object::elf::R_PPC64_REL24);
        add_elf_relocation(&mut object, text, 4, external, object::elf::R_PPC64_REL24);
        add_elf_relocation(
            &mut object,
            text,
            14,
            toc_symbol,
            object::elf::R_PPC64_TOC16_HA,
        );
        add_elf_relocation(
            &mut object,
            text,
            18,
            toc_symbol,
            object::elf::R_PPC64_TOC16_LO_DS,
        );

        let bytes = object.write().expect("write synthetic PPC64 ET_REL");
        let file = object::File::parse(&*bytes).expect("parse synthetic PPC64 ET_REL");
        let layout = layout_relocatable(&file, &ElfFormat);
        assert!(layout.diagnostics.is_empty(), "{:?}", layout.diagnostics);
        let code = &layout.segments[0].1;
        assert_eq!(
            u32::from_be_bytes(code[..4].try_into().unwrap()),
            0x4800_0009
        );
        assert_eq!(&code[14..16], &[0x00, 0x00]);
        assert_eq!(&code[18..20], &[0x80, 0x02]);
        assert!(layout
            .funcsyms
            .iter()
            .any(|(_, name)| name == b".external_status"));
    }

    #[test]
    fn ppc64_rel24_link_bit_controls_external_call_identity() {
        let mut object =
            write::Object::new(BinaryFormat::Elf, Architecture::PowerPc64, Endianness::Big);
        let text = object.section_id(write::StandardSection::Text);
        object.append_section_data(
            text,
            &[
                0x48, 0x00, 0x00, 0x00, // b branch_target
                0x48, 0x00, 0x00, 0x01, // bl call_target
                0x4e, 0x80, 0x00, 0x20, // blr
            ],
            4,
        );
        let branch_target = add_symbol(
            &mut object,
            b"branch_target",
            write::SymbolSection::Undefined,
            0,
            SymbolKind::Unknown,
        );
        let call_target = add_symbol(
            &mut object,
            b"call_target",
            write::SymbolSection::Undefined,
            0,
            SymbolKind::Unknown,
        );
        add_elf_relocation(
            &mut object,
            text,
            0,
            branch_target,
            object::elf::R_PPC64_REL24,
        );
        add_elf_relocation(
            &mut object,
            text,
            4,
            call_target,
            object::elf::R_PPC64_REL24,
        );

        let bytes = object.write().expect("write synthetic PPC64 ET_REL");
        let file = object::File::parse(&*bytes).expect("parse synthetic PPC64 ET_REL");
        let layout = layout_relocatable(&file, &ElfFormat);
        assert!(layout.diagnostics.is_empty(), "{:?}", layout.diagnostics);
        assert!(!layout
            .funcsyms
            .iter()
            .any(|(_, name)| name == b"branch_target"));
        assert!(layout
            .funcsyms
            .iter()
            .any(|(_, name)| name == b"call_target"));
    }

    #[test]
    fn past_section_relocations_are_counted_without_touching_bytes() {
        let mut object =
            write::Object::new(BinaryFormat::Elf, Architecture::Arm, Endianness::Little);
        let text = object.section_id(write::StandardSection::Text);
        object.append_section_data(text, &0xe12f_ff1eu32.to_le_bytes(), 4);
        let target = add_symbol(
            &mut object,
            b"target",
            write::SymbolSection::Section(text),
            0,
            SymbolKind::Text,
        );
        add_elf_relocation(&mut object, text, 4, target, object::elf::R_ARM_CALL);

        let bytes = object.write().expect("write out-of-bounds relocation");
        let file = object::File::parse(&*bytes).expect("parse out-of-bounds relocation");
        let layout = layout_relocatable(&file, &ElfFormat);
        assert_eq!(layout.diagnostics.total(), 1);
        assert!(layout
            .diagnostics
            .report_lines()
            .iter()
            .any(|line| { line.contains(RelocationFailure::PastSectionEnd.label()) }));
        assert_eq!(layout.segments[0].1, 0xe12f_ff1eu32.to_le_bytes());
    }

    #[test]
    fn relocation_diagnostic_detail_is_bounded_but_counts_are_exact() {
        let mut diagnostics = RelocDiagnostics::default();
        for offset in 0..4_000 {
            diagnostics.record(
                Architecture::Arm,
                Some(255),
                RelocationFailure::Unsupported,
                RELOC_BASE,
                offset * 4,
            );
        }
        let lines = diagnostics.report_lines();
        assert_eq!(diagnostics.total(), 4_000);
        assert_eq!(lines.len(), 2);
        assert!(
            lines[0].contains("4000 relocation(s) skipped"),
            "{:?}",
            lines
        );
        assert!(lines[1].contains("count=4000"), "{:?}", lines);
        assert!(
            lines[1].contains("sample(s)-suppressed=3997"),
            "{:?}",
            lines
        );
        assert!(lines.iter().map(String::len).sum::<usize>() < 512);
    }

    #[test]
    fn relocation_diagnostics_render_every_reason_and_cap_groups_deterministically() {
        let reasons = [
            RelocationFailure::Unsupported,
            RelocationFailure::UnresolvedTarget,
            RelocationFailure::MissingToc,
            RelocationFailure::PastSectionEnd,
            RelocationFailure::RequiresVeneer,
            RelocationFailure::Misaligned,
            RelocationFailure::OutOfRange,
            RelocationFailure::InvalidEncoding,
        ];
        let mut diagnostics = RelocDiagnostics::default();
        for (r_type, reason) in reasons.into_iter().enumerate() {
            diagnostics.record(
                Architecture::Arm,
                Some(r_type as u32),
                reason,
                RELOC_BASE,
                0,
            );
        }
        let lines = diagnostics.report_lines();
        assert_eq!(lines.len(), reasons.len() + 1);
        for reason in reasons {
            assert!(
                lines.iter().any(|line| line.contains(reason.label())),
                "missing reason {reason:?}: {lines:?}"
            );
        }

        let mut capped = RelocDiagnostics::default();
        for r_type in 0..10 {
            capped.record(
                Architecture::Arm,
                Some(r_type),
                RelocationFailure::Unsupported,
                RELOC_BASE,
                0,
            );
        }
        let lines = capped.report_lines();
        assert_eq!(lines.len(), DIAGNOSTIC_GROUP_LIMIT + 1);
        assert!(lines[0].contains("showing 8 group(s), 2 suppressed"));
        for (r_type, line) in lines.iter().skip(1).enumerate() {
            assert!(line.contains(&format!("({r_type})")), "{lines:?}");
        }
        assert!(!lines
            .iter()
            .any(|line| line.contains("(8)") || line.contains("(9)")));
    }
}
