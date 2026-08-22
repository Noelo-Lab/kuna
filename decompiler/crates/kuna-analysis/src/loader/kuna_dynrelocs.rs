//! (kuna) `dynrelocs` — apply a **linked** ELF image's dynamic relocations into
//! the loaded image, and report the slots `PT_GNU_RELRO` freezes as constant.
//!
//! ## The gap
//!
//! [`crate::loadimage_object::ObjectLoadImage`] maps the `PT_LOAD` bytes as the
//! linker wrote them. That is the *file*, not the run-time image: every slot an
//! `R_*_RELATIVE` / `R_*_GLOB_DAT` / `R_*_JUMP_SLOT` relocation fills is left at
//! **0** for the dynamic loader. In a PIE — which every modern toolchain emits by
//! default — that includes the whole `.got`, so an indirect call through a
//! function pointer the linker itself resolved reads a null target and renders
//! `(*dat_e0dc8)(…)` instead of the callee's name.
//!
//! ## What this module does
//!
//! Two facts, both derived from the same walk of `.rela.dyn`/`.rel.dyn`/
//! `.rela.plt`:
//!
//! 1. **The writes.** One `(vma, value, width)` per relocation whose result is
//!    knowable without a dynamic loader — `RELATIVE` (the image's own load bias
//!    plus the addend) and `GLOB_DAT`/`JUMP_SLOT` **whose symbol is defined in
//!    this same image**. An *undefined* symbol is an import: there is nothing to
//!    write, the run-time value comes from another object, and the existing
//!    PLT/import naming ([`crate::loader::elf_plt`]) already covers the call. It
//!    is skipped, so that path is untouched.
//! 2. **The constant slots.** Those same written slots, intersected with the
//!    `PT_GNU_RELRO` segment. `.got` is `SHF_WRITE`, so nothing downstream would
//!    trust its contents — but `PT_GNU_RELRO` is exactly the linker's statement
//!    that the range is `mprotect`ed read-only once startup relocation is done.
//!    That is what makes the relocated value foldable, and the two halves are
//!    useless apart: applying the relocation without the constancy leaves the
//!    load unfolded, and declaring constancy without applying the relocation
//!    would fold the call target to *zero*.
//!
//! Both are returned to the loader, which patches its own segment bytes and adds
//! the constant slots to `ObjectLoadImage::dynreloc_const` (see
//! [`crate::loadimage_object`]).
//!
//! ## Scope
//!
//! Linked images only (`ET_EXEC`/`ET_DYN`); an `ET_REL` `.o` takes the separate
//! `relocobjects` layout path, which applies its own relocations. Architectures:
//! x86-64, AArch64, i386 and ARM — the four whose `RELATIVE`/`GLOB_DAT`/
//! `JUMP_SLOT` triple is all that is needed. Both `RELA` (explicit addend) and
//! `REL` (addend in the slot) tables are handled. Relocation kinds outside the
//! triple (`COPY`, `IRELATIVE`, `TLS*`, …) are not knowable statically and are
//! skipped.

use object::read::{Object, ObjectSymbol};
use object::{RelocationFlags, RelocationTarget};

/// One relocation whose run-time value is statically knowable.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DynRelocWrite {
    /// Virtual address of the slot being filled in.
    pub vma: u64,
    /// The value the dynamic loader would store there.
    pub value: u64,
    /// Slot width in bytes (the image's pointer size).
    pub width: u8,
}

/// The two fact streams [`resolve`] produces. Empty whenever the gate is off,
/// the image is not a linked ELF, or the machine is not one of the four handled.
#[derive(Debug, Default)]
pub struct DynRelocs {
    /// Slots to fill in, in relocation-table order.
    pub writes: Vec<DynRelocWrite>,
    /// `[start, stop]` **inclusive** byte ranges of the written slots that
    /// `PT_GNU_RELRO` covers — the ones a reader may treat as constant.
    pub const_ranges: Vec<(u64, u64)>,
}

/// The `RELATIVE` / `GLOB_DAT` / `JUMP_SLOT` `r_type` triple for `machine`, or
/// `None` if kuna does not handle this architecture's dynamic relocations.
fn reloc_triple(arch: object::Architecture) -> Option<(u32, u32, u32)> {
    use object::elf;
    match arch {
        object::Architecture::X86_64 => Some((
            elf::R_X86_64_RELATIVE,
            elf::R_X86_64_GLOB_DAT,
            elf::R_X86_64_JUMP_SLOT,
        )),
        object::Architecture::Aarch64 => Some((
            elf::R_AARCH64_RELATIVE,
            elf::R_AARCH64_GLOB_DAT,
            elf::R_AARCH64_JUMP_SLOT,
        )),
        object::Architecture::I386 => {
            Some((elf::R_386_RELATIVE, elf::R_386_GLOB_DAT, elf::R_386_JMP_SLOT))
        }
        object::Architecture::Arm => {
            Some((elf::R_ARM_RELATIVE, elf::R_ARM_GLOB_DAT, elf::R_ARM_JUMP_SLOT))
        }
        _ => None,
    }
}

/// ELF `p_type` of the RELRO program header (`PT_GNU_RELRO`).
const PT_GNU_RELRO: u32 = 0x6474_e552;

/// `[start, end)` of the image's `PT_GNU_RELRO` segment, read straight off the
/// program-header table (the generic `object` segment iterator yields `PT_LOAD`
/// only, so the raw walk is the only route). `None` when the image declares no
/// RELRO segment — a `-z norelro` link, or a static non-PIE.
fn relro_range(bytes: &[u8]) -> Option<(u64, u64)> {
    if bytes.len() < 64 || &bytes[..4] != b"\x7fELF" {
        return None;
    }
    let is64 = bytes[4] == 2;
    let le = bytes[5] == 1;
    let u16at = |o: usize| -> Option<u16> {
        let b: [u8; 2] = bytes.get(o..o + 2)?.try_into().ok()?;
        Some(if le { u16::from_le_bytes(b) } else { u16::from_be_bytes(b) })
    };
    let u32at = |o: usize| -> Option<u32> {
        let b: [u8; 4] = bytes.get(o..o + 4)?.try_into().ok()?;
        Some(if le { u32::from_le_bytes(b) } else { u32::from_be_bytes(b) })
    };
    let u64at = |o: usize| -> Option<u64> {
        let b: [u8; 8] = bytes.get(o..o + 8)?.try_into().ok()?;
        Some(if le { u64::from_le_bytes(b) } else { u64::from_be_bytes(b) })
    };

    let (phoff, phentsize, phnum) = if is64 {
        (u64at(0x20)?, u16at(0x36)? as usize, u16at(0x38)? as usize)
    } else {
        (u32at(0x1c)? as u64, u16at(0x2a)? as usize, u16at(0x2c)? as usize)
    };
    let min = if is64 { 56 } else { 32 };
    if phentsize < min {
        return None;
    }
    for i in 0..phnum {
        let ph = (phoff as usize).checked_add(i.checked_mul(phentsize)?)?;
        if u32at(ph)? != PT_GNU_RELRO {
            continue;
        }
        let (vaddr, memsz) =
            if is64 { (u64at(ph + 0x10)?, u64at(ph + 0x28)?) } else { (u32at(ph + 8)? as u64, u32at(ph + 20)? as u64) };
        if memsz == 0 {
            return None;
        }
        return Some((vaddr, vaddr.wrapping_add(memsz)));
    }
    None
}

/// Read the `width`-byte slot at `vma` out of the mapped `PT_LOAD` bytes — the
/// implicit addend a `REL`-style table (i386/ARM `.rel.dyn`) stores in the field
/// being patched rather than in the entry.
fn implicit_addend(file: &object::File, vma: u64, width: usize, le: bool) -> Option<u64> {
    use object::read::ObjectSegment;
    for seg in file.segments() {
        let base = seg.address();
        if vma < base {
            continue;
        }
        let data = seg.data().ok()?;
        let off = (vma - base) as usize;
        let end = off.checked_add(width)?;
        if end > data.len() {
            continue;
        }
        let mut v: u64 = 0;
        if le {
            for b in data[off..end].iter().rev() {
                v = (v << 8) | *b as u64;
            }
        } else {
            for b in &data[off..end] {
                v = (v << 8) | *b as u64;
            }
        }
        return Some(v);
    }
    None
}

/// Walk the image's dynamic relocation tables and produce the statically
/// knowable writes plus the RELRO-frozen subset of their slots.
///
/// Returns an empty result — the byte-identical pre-fix behaviour — whenever
/// `--option dynrelocs off` (env `KUNA_DYNRELOCS`), the file is not an ELF, the
/// file is an `ET_REL` object (the `relocobjects` path owns those), or the
/// machine is not one of the four handled. Never errors: a malformed entry is
/// skipped, never applied on a guess.
pub fn resolve(file: &object::File, bytes: &[u8]) -> DynRelocs {
    let mut out = DynRelocs::default();
    if !kuna_decomp::kuna_dynrelocs::dynrelocs_enabled() {
        return out;
    }
    if file.format() != object::BinaryFormat::Elf {
        return out;
    }
    // A pre-link object is the `relocobjects` layout path's business; it applies
    // its own relocations against a synthetic layout, and the tables here name a
    // different address space.
    if file.kind() == object::ObjectKind::Relocatable {
        return out;
    }
    let Some((r_relative, r_glob_dat, r_jump_slot)) = reloc_triple(file.architecture()) else {
        return out;
    };
    let Some(relocs) = file.dynamic_relocations() else {
        return out;
    };

    let width: usize = if file.is_64() { 8 } else { 4 };
    let le = file.is_little_endian();

    // kuna maps a linked image at the vaddrs the ELF itself declares, so the load
    // bias every `RELATIVE` relocation adds is zero. Named rather than folded
    // away so the formula stays the ABI's `B + A`.
    let load_bias: u64 = 0;

    // `.dynsym` by index — the index a dynamic relocation's `r_sym` names. The
    // generic `symbol_by_index` reads `.symtab`, which is a different table (and
    // is absent from a stripped image), so the map is built here.
    let mut dynsyms: std::collections::HashMap<usize, (u64, bool)> = std::collections::HashMap::new();
    for s in file.dynamic_symbols() {
        dynsyms.insert(s.index().0, (s.address(), s.is_undefined()));
    }

    let relro = relro_range(bytes);
    let mask = if width == 8 { u64::MAX } else { u32::MAX as u64 };

    for (offset, reloc) in relocs {
        let RelocationFlags::Elf { r_type } = reloc.flags() else {
            continue;
        };
        let addend = if reloc.has_implicit_addend() {
            match implicit_addend(file, offset, width, le) {
                Some(a) => a,
                None => continue,
            }
        } else {
            reloc.addend() as u64
        };

        let value = if r_type == r_relative {
            load_bias.wrapping_add(addend)
        } else if r_type == r_glob_dat || r_type == r_jump_slot {
            let RelocationTarget::Symbol(idx) = reloc.target() else {
                continue;
            };
            match dynsyms.get(&idx.0) {
                // An undefined symbol is an import: the value lives in another
                // image. Nothing to write, and the PLT/import naming already
                // covers the call — leave the slot exactly as the linker left it.
                Some(&(_, true)) | None => continue,
                Some(&(addr, false)) => {
                    // A `REL`-style GLOB_DAT/JUMP_SLOT ignores the in-place word
                    // (the ABI is `S`, not `S + A`); only a RELA addend counts.
                    let a = if reloc.has_implicit_addend() { 0 } else { addend };
                    addr.wrapping_add(a)
                }
            }
        } else {
            continue;
        };

        let value = value & mask;
        out.writes.push(DynRelocWrite { vma: offset, value, width: width as u8 });

        if let Some((lo, hi)) = relro {
            let last = offset.wrapping_add(width as u64 - 1);
            if offset >= lo && last < hi {
                out.const_ranges.push((offset, last));
            }
        }
    }

    out
}

#[cfg(test)]
mod tests;
