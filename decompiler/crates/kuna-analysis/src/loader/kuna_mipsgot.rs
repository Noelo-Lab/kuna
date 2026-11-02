//! (kuna `dynrelocs`) The MIPS primary-GOT entries a linked image's dynamic
//! linker fills with an address the image itself defines, reported constant so
//! a load from one folds: `lw $25,%call16(add64)($gp); jalr $25` becomes a call
//! to `add64`, and `lw $2,%got(counter)($gp); lw $2,%lo(counter)($2)` a read of
//! `counter`.
//!
//! A MIPS dynamic image relocates its primary GOT with no relocation records
//! (glibc's `elf_machine_got_rel`). Its first `DT_MIPS_LOCAL_GOTNO` entries are
//! local: page or full link-time addresses, to which the run-time loader adds
//! the load bias. Entry 0 is the lazy resolver's address and entry 1, when its
//! top bit is set, the module pointer (a GNU extension); both are written at
//! run time and stay variable. The global entries that follow map one-to-one
//! onto `.dynsym` from `DT_MIPS_GOTSYM` to `DT_MIPS_SYMTABNO`; for a symbol the
//! image defines, the loader stores the address its lookup binds the name to,
//! which the linker wrote as the symbol's value. kuna maps the image at its
//! link-time addresses, so each of these slots already holds its run-time
//! value; none is written by program code.
//!
//! A default-visibility definition in a shared object can be interposed: the
//! lookup may bind the name to another object's definition first. The fold
//! still names the symbol the call or access binds by, and gives it the body
//! this image carries, as `dynrelocs` does for a `GLOB_DAT` slot of a defined
//! symbol and `pltlocal` for a PLT stub. `--option dynrelocs off` leaves every
//! such slot variable.
//!
//! Undefined symbols are imports, whose stub slots `elf_plt` folds; common,
//! absolute, TLS and `STT_GNU_IFUNC` symbols, and a slot whose contents are not
//! the symbol's value, are skipped. Secondary GOTs of a multi-GOT image are
//! relocated by `R_MIPS_REL32` records instead and are not covered.

use object::read::{Object, ObjectSection, ObjectSymbol};
use object::{Architecture, SymbolFlags, SymbolSection};

/// `STT_TLS`.
const STT_TLS: u8 = 6;
/// `STT_GNU_IFUNC`.
const STT_GNU_IFUNC: u8 = 10;

/// The primary GOT's layout, read off `.dynamic`.
struct Layout {
    base: u64,
    local: u64,
    gotsym: u64,
    symtabno: Option<u64>,
}

/// `[start, stop]` (inclusive) of every primary-GOT slot whose run-time value is
/// an address the image defines, ascending and coalesced. Empty off-MIPS, for a
/// relocatable object, without the `DT_MIPS_*` tags, or with `dynrelocs` off.
pub fn const_ranges(file: &object::File) -> Vec<(u64, u64)> {
    if !matches!(file.architecture(), Architecture::Mips | Architecture::Mips64 | Architecture::Mips64_N32)
        || file.kind() == object::ObjectKind::Relocatable
        || !kuna_decomp::kuna_dynrelocs::dynrelocs_enabled()
    {
        return Vec::new();
    }
    let Some(layout) = layout(file) else { return Vec::new() };
    let ptr = if file.is_64() { 8u64 } else { 4u64 };
    let le = file.is_little_endian();
    let word = |index: u64| -> Option<u64> {
        read_word(file, layout.base.checked_add(index.checked_mul(ptr)?)?, ptr as usize, le)
    };
    let slot_of = |index: u64| layout.base.wrapping_add(index * ptr);

    let mut slots: Vec<u64> = Vec::new();
    if layout.local > 1 {
        let module_ptr = word(1).is_some_and(|w| w >> (ptr * 8 - 1) != 0);
        let first = if module_ptr { 2 } else { 1 };
        slots.extend((first..layout.local).filter(|&i| word(i).is_some()).map(slot_of));
    }

    let mut defined: Vec<(u64, u64)> = Vec::new();
    for sym in file.dynamic_symbols() {
        let index = sym.index().0 as u64;
        if index < layout.gotsym || layout.symtabno.is_some_and(|n| index >= n) {
            continue;
        }
        let SymbolFlags::Elf { st_info, .. } = sym.flags() else { continue };
        if !matches!(sym.section(), SymbolSection::Section(_)) || matches!(st_info & 0xf, STT_TLS | STT_GNU_IFUNC) {
            continue;
        }
        defined.push((layout.local + (index - layout.gotsym), sym.address()));
    }
    for (got_index, value) in defined {
        if word(got_index) == Some(value) {
            slots.push(slot_of(got_index));
        }
    }

    slots.sort_unstable();
    slots.dedup();
    let mut out: Vec<(u64, u64)> = Vec::new();
    for slot in slots {
        let stop = slot + (ptr - 1);
        match out.last_mut() {
            Some(last) if last.1.wrapping_add(1) == slot => last.1 = stop,
            _ => out.push((slot, stop)),
        }
    }
    out
}

/// `DT_PLTGOT`, `DT_MIPS_LOCAL_GOTNO`, `DT_MIPS_GOTSYM` and `DT_MIPS_SYMTABNO`
/// from the `.dynamic` section; `None` without the first three.
fn layout(file: &object::File) -> Option<Layout> {
    const DT_NULL: u64 = 0;
    const DT_PLTGOT: u64 = 3;
    const DT_MIPS_LOCAL_GOTNO: u64 = 0x7000_000a;
    const DT_MIPS_SYMTABNO: u64 = 0x7000_0011;
    const DT_MIPS_GOTSYM: u64 = 0x7000_0013;

    let data = file.section_by_name(".dynamic")?.data().ok()?;
    let width = if file.is_64() { 8 } else { 4 };
    let le = file.is_little_endian();
    let (mut base, mut local, mut gotsym, mut symtabno) = (None, None, None, None);
    for entry in data.chunks_exact(width * 2) {
        let (tag, val) = (uint(&entry[..width], le), uint(&entry[width..], le));
        match tag {
            DT_NULL => break,
            DT_PLTGOT => base = Some(val),
            DT_MIPS_LOCAL_GOTNO => local = Some(val),
            DT_MIPS_GOTSYM => gotsym = Some(val),
            DT_MIPS_SYMTABNO => symtabno = Some(val),
            _ => {}
        }
    }
    Some(Layout { base: base?, local: local?, gotsym: gotsym?, symtabno })
}

/// The `width`-byte word at `vma`, from the section holding all of it.
fn read_word(file: &object::File, vma: u64, width: usize, le: bool) -> Option<u64> {
    let sec = file.sections().find(|s| {
        s.size() != 0 && vma >= s.address() && vma - s.address() + width as u64 <= s.size()
    })?;
    let data = sec.data().ok()?;
    let at = usize::try_from(vma - sec.address()).ok()?;
    Some(uint(data.get(at..at + width)?, le))
}

fn uint(bytes: &[u8], le: bool) -> u64 {
    let fold = |acc: u64, b: &u8| (acc << 8) | u64::from(*b);
    if le {
        bytes.iter().rev().fold(0, fold)
    } else {
        bytes.iter().fold(0, fold)
    }
}

#[cfg(test)]
mod tests;
