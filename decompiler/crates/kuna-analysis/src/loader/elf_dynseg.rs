//! (kuna) ELF import markup read out of `PT_DYNAMIC` — the segment-only twin of
//! the section-driven inputs [`crate::loader::elf_plt`] normally uses.
//!
//! ## The gap
//!
//! Every route by which kuna learns an import's name is keyed on the section
//! table: `object`'s `dynamic_relocations()` builds its iterator by scanning the
//! `SectionTable` for `SHT_REL`/`SHT_RELA`, `dynamic_symbols()` reads `.dynsym`
//! as a section, and [`elf_plt`](crate::loader::elf_plt) locates the stubs
//! themselves by `.plt`/`.plt.sec`/`.plt.got` section name. An ELF with no
//! usable section table therefore gets its bytes and none of its import markup,
//! and a call to `puts@plt` renders `(*dat_7f98)()`.
//!
//! Nothing about that information is section-bound: `PT_DYNAMIC` names the
//! symbol table, the string table and both relocation tables by virtual address,
//! which is exactly how the run-time loader finds them. This module reads them
//! that way.
//!
//! ## What it produces
//!
//! [`dynamic_imports`] returns `got_slot_vma → import_name` built from every
//! symbol-bearing entry of `DT_JMPREL` and `DT_RELA`/`DT_REL` — the same map
//! `elf_plt::build_got_name_map` builds from `dynamic_relocations()`, from the
//! same bytes by a different route — plus `DT_PLTGOT`, which is the i386
//! `_GLOBAL_OFFSET_TABLE_` anchor its PIE stub decoder needs.
//!
//! [`exec_segments`] returns the executable `PT_LOAD` windows the stubs live in.
//! The PLT is *not* reconstructed as a range: there is no section name left to
//! find it by, and guessing a sub-range of an executable segment is how a stub
//! gets named off by one entry. `elf_plt`'s decoders are already
//! self-correcting — they emit a name only for a stub whose *decoded* GOT target
//! is in the map — so the executable segment is handed to them whole and the
//! relocation slots decide which instructions were stubs.
//!
//! Total and non-panicking: a truncated, hostile or simply sectionless-and-static
//! file yields `None`/empty and leaves the caller's behavior unchanged.

use std::collections::HashMap;

const PT_LOAD: u32 = 1;
const PT_DYNAMIC: u32 = 2;
const PF_X: u32 = 1;

const DT_NULL: u64 = 0;
const DT_PLTRELSZ: u64 = 2;
const DT_PLTGOT: u64 = 3;
const DT_STRTAB: u64 = 5;
const DT_SYMTAB: u64 = 6;
const DT_RELA: u64 = 7;
const DT_RELASZ: u64 = 8;
const DT_SYMENT: u64 = 11;
const DT_REL: u64 = 17;
const DT_RELSZ: u64 = 18;
const DT_PLTREL: u64 = 20;
const DT_JMPREL: u64 = 23;

/// The import facts recoverable from the dynamic segment alone.
pub(crate) struct DynImports {
    /// GOT slot VMA → imported symbol name (`@VERSION` stripped), from every
    /// symbol-bearing dynamic relocation.
    pub got_to_name: HashMap<u64, Vec<u8>>,
    /// `DT_PLTGOT` — the GOT base, i.e. `_GLOBAL_OFFSET_TABLE_`. `None` when the
    /// image declares no PLT GOT.
    pub pltgot: Option<u64>,
}

/// A parsed ELF header + program-header table, enough to translate a VMA to a
/// file offset. Built by [`Image::parse`]; every accessor is bounds-checked.
struct Image<'a> {
    bytes: &'a [u8],
    is64: bool,
    le: bool,
    /// `(p_type, p_flags, p_offset, p_vaddr, p_filesz)` per program header.
    phdrs: Vec<(u32, u32, u64, u64, u64)>,
}

impl<'a> Image<'a> {
    fn parse(bytes: &'a [u8]) -> Option<Self> {
        if bytes.len() < 64 || &bytes[..4] != b"\x7fELF" {
            return None;
        }
        let is64 = match bytes[4] {
            1 => false,
            2 => true,
            _ => return None,
        };
        let le = match bytes[5] {
            1 => true,
            2 => false,
            _ => return None,
        };
        let mut img = Image { bytes, is64, le, phdrs: Vec::new() };
        let (phoff, phentsize, phnum) = if is64 {
            (img.u64at(0x20)?, img.u16at(0x36)? as usize, img.u16at(0x38)? as usize)
        } else {
            (img.u32at(0x1c)? as u64, img.u16at(0x2a)? as usize, img.u16at(0x2c)? as usize)
        };
        if phentsize < if is64 { 56 } else { 32 } {
            return None;
        }
        for i in 0..phnum {
            let ph = (phoff as usize).checked_add(i.checked_mul(phentsize)?)?;
            let p_type = img.u32at(ph)?;
            let (flags, off, vaddr, filesz) = if is64 {
                (img.u32at(ph + 4)?, img.u64at(ph + 8)?, img.u64at(ph + 0x10)?, img.u64at(ph + 0x20)?)
            } else {
                (
                    img.u32at(ph + 24)?,
                    img.u32at(ph + 4)? as u64,
                    img.u32at(ph + 8)? as u64,
                    img.u32at(ph + 16)? as u64,
                )
            };
            img.phdrs.push((p_type, flags, off, vaddr, filesz));
        }
        Some(img)
    }

    fn u16at(&self, o: usize) -> Option<u16> {
        let b: [u8; 2] = self.bytes.get(o..o + 2)?.try_into().ok()?;
        Some(if self.le { u16::from_le_bytes(b) } else { u16::from_be_bytes(b) })
    }

    fn u32at(&self, o: usize) -> Option<u32> {
        let b: [u8; 4] = self.bytes.get(o..o + 4)?.try_into().ok()?;
        Some(if self.le { u32::from_le_bytes(b) } else { u32::from_be_bytes(b) })
    }

    fn u64at(&self, o: usize) -> Option<u64> {
        let b: [u8; 8] = self.bytes.get(o..o + 8)?.try_into().ok()?;
        Some(if self.le { u64::from_le_bytes(b) } else { u64::from_be_bytes(b) })
    }

    /// Pointer-width word (`Elf32_Addr`/`Elf64_Addr`) at a file offset.
    fn wordat(&self, o: usize) -> Option<u64> {
        if self.is64 {
            self.u64at(o)
        } else {
            self.u32at(o).map(u64::from)
        }
    }

    /// File offset of a virtual address, per the `PT_LOAD` map. `None` when no
    /// loadable segment has that address backed by file bytes.
    fn off_of(&self, vma: u64) -> Option<usize> {
        for &(p_type, _, off, vaddr, filesz) in &self.phdrs {
            if p_type != PT_LOAD || filesz == 0 {
                continue;
            }
            if vma >= vaddr && vma - vaddr < filesz {
                return usize::try_from(off.checked_add(vma - vaddr)?).ok();
            }
        }
        None
    }

    /// The NUL-terminated string at `vma`, capped at 4 KiB.
    fn cstr_at(&self, vma: u64) -> Option<&'a [u8]> {
        let off = self.off_of(vma)?;
        let tail = self.bytes.get(off..)?;
        let end = tail.iter().take(4096).position(|&b| b == 0)?;
        Some(&tail[..end])
    }
}

/// Executable `PT_LOAD` windows as `(vma, file bytes)`, in program-header order.
///
/// Empty for a non-ELF or header-truncated input. `p_memsz` beyond `p_filesz` is
/// zero-fill, so only the file-backed part is returned — a stub decoder has
/// nothing to read past it.
pub(crate) fn exec_segments(bytes: &[u8]) -> Vec<(u64, &[u8])> {
    let Some(img) = Image::parse(bytes) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for &(p_type, flags, off, vaddr, filesz) in &img.phdrs {
        if p_type != PT_LOAD || flags & PF_X == 0 || filesz == 0 {
            continue;
        }
        let (Ok(off), Ok(filesz)) = (usize::try_from(off), usize::try_from(filesz)) else {
            continue;
        };
        if let Some(data) = bytes.get(off..off.saturating_add(filesz)) {
            out.push((vaddr, data));
        }
    }
    out
}

/// Read the dynamic segment's import markup: `got_slot → name` plus `DT_PLTGOT`.
///
/// `None` when the file is not an ELF, has no `PT_DYNAMIC`, or names no symbol
/// and string table (a statically linked image). An empty `got_to_name` on a
/// present dynamic segment is a legitimate answer — the image imports nothing.
pub(crate) fn dynamic_imports(bytes: &[u8]) -> Option<DynImports> {
    let img = Image::parse(bytes)?;
    let tags = dynamic_tags(&img)?;

    let get = |t: u64| tags.iter().rev().find(|&&(tag, _)| tag == t).map(|&(_, v)| v);
    let pltgot = get(DT_PLTGOT);
    let (Some(symtab), Some(strtab)) = (get(DT_SYMTAB), get(DT_STRTAB)) else {
        return Some(DynImports { got_to_name: HashMap::new(), pltgot });
    };
    // `DT_SYMENT` is mandatory alongside `DT_SYMTAB`, but a hand-written image may
    // omit it; the ABI size is the only sane reading of the table without it.
    let syment = get(DT_SYMENT).filter(|&s| s >= 8).unwrap_or(if img.is64 { 24 } else { 16 });

    let mut got_to_name: HashMap<u64, Vec<u8>> = HashMap::new();
    // `DT_JMPREL` (the PLT relocations) first: its `JUMP_SLOT` entries are the
    // stub targets, and `DT_RELA`/`DT_REL` may overlap it in a `-z now` image
    // that folded the two tables.
    // `DT_PLTREL` is mandatory alongside `DT_JMPREL` and says which of the two
    // entry layouts it uses; without it the table is not readable, and guessing
    // reads a `r_offset` out of the middle of a different field.
    let jmprel = match get(DT_PLTREL) {
        Some(DT_RELA) => (get(DT_JMPREL), get(DT_PLTRELSZ), true),
        Some(DT_REL) => (get(DT_JMPREL), get(DT_PLTRELSZ), false),
        _ => (None, None, true),
    };
    for (base, size, rela) in
        [jmprel, (get(DT_RELA), get(DT_RELASZ), true), (get(DT_REL), get(DT_RELSZ), false)]
    {
        let (Some(base), Some(size)) = (base, size) else {
            continue;
        };
        read_reloc_table(&img, base, size, rela, symtab, syment, strtab, &mut got_to_name);
    }

    Some(DynImports { got_to_name, pltgot })
}

/// The `(d_tag, d_un)` pairs of the `PT_DYNAMIC` segment, up to `DT_NULL`.
fn dynamic_tags(img: &Image) -> Option<Vec<(u64, u64)>> {
    let (_, _, off, _, filesz) = *img.phdrs.iter().find(|&&(t, ..)| t == PT_DYNAMIC)?;
    let off = usize::try_from(off).ok()?;
    let filesz = usize::try_from(filesz).ok()?;
    let entsz = if img.is64 { 16 } else { 8 };
    let mut out = Vec::new();
    let mut at = off;
    while at + entsz <= off.checked_add(filesz)? && at + entsz <= img.bytes.len() {
        let tag = img.wordat(at)?;
        let val = img.wordat(at + entsz / 2)?;
        if tag == DT_NULL {
            break;
        }
        out.push((tag, val));
        at += entsz;
    }
    Some(out)
}

/// Walk one `RELA`/`REL` table, inserting `r_offset → symbol name` for every
/// entry that names a symbol. Silently stops at the first unreadable entry.
#[allow(clippy::too_many_arguments)]
fn read_reloc_table(
    img: &Image,
    base: u64,
    size: u64,
    rela: bool,
    symtab: u64,
    syment: u64,
    strtab: u64,
    out: &mut HashMap<u64, Vec<u8>>,
) {
    let entsz: u64 = match (img.is64, rela) {
        (true, true) => 24,
        (true, false) => 16,
        (false, true) => 12,
        (false, false) => 8,
    };
    let n = size / entsz;
    // A corrupt `DT_*SZ` must not turn into an unbounded walk; no real table has
    // more entries than the file has bytes to hold them.
    let n = n.min(img.bytes.len() as u64 / entsz.max(1));
    for i in 0..n {
        let Some(off) = i.checked_mul(entsz).and_then(|d| base.checked_add(d)).and_then(|v| img.off_of(v))
        else {
            return;
        };
        let (Some(r_offset), Some(r_info)) =
            (img.wordat(off), img.wordat(off + if img.is64 { 8 } else { 4 }))
        else {
            return;
        };
        let sym = if img.is64 { r_info >> 32 } else { r_info >> 8 };
        if sym == 0 {
            continue;
        }
        let Some(st_off) =
            sym.checked_mul(syment).and_then(|d| symtab.checked_add(d)).and_then(|v| img.off_of(v))
        else {
            continue;
        };
        let Some(st_name) = img.u32at(st_off) else {
            continue;
        };
        if st_name == 0 {
            continue;
        }
        let Some(name) = strtab.checked_add(u64::from(st_name)).and_then(|v| img.cstr_at(v)) else {
            continue;
        };
        if name.is_empty() {
            continue;
        }
        out.entry(r_offset).or_insert_with(|| super::elf_plt::strip_version(name));
    }
}

#[cfg(test)]
mod tests;
