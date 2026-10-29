//! ELF `GLOB_DAT` GOT slots of undefined symbols — the ELF analog of a Mach-O
//! `__got` import slot (`elfgotimports`, committed under `importdata`).
//!
//! A `-fPIC` reference to data another image defines (`stdout`, `environ`, a
//! library's global) loads a GOT slot the dynamic loader fills with the symbol's
//! address (`R_*_GLOB_DAT`), then loads through it. kuna names PLT stubs and paints
//! PE/Mach-O import slots `externref`, but an ELF GOT data slot was neither, so the
//! read printed as `*dat_<slot>`. [`resolve`] lists those slots with the symbol each
//! one holds. [`crate::loader::format::elf::ElfFormat`] reports them as import slots
//! (pointer words, never function bodies), and [`ElfGotImportPass`] names them and
//! paints them `externref`, so `importdata` renders the read as the import exactly
//! as it does for a Mach-O `__got` slot.
//!
//! Only undefined symbols are taken. A `GLOB_DAT` naming a symbol the image defines
//! is filled in and folded by `dynrelocs`, which already renders the object itself
//! (`exported_value`, `&exported_value`); painting it `externref` would stop that
//! fold. The relocation types are `dynrelocs`' (x86-64, AArch64, i386, ARM).

use object::read::{Object, ObjectSymbol};
use object::{RelocationFlags, RelocationTarget};

use crate::pass::{AnalysisCtx, AnalysisOutput, AnalysisPass, Phase};

/// `(slot VMA, symbol name)` for every `GLOB_DAT` slot whose symbol is undefined
/// in this image. Empty for a non-ELF input, a relocatable object, or a machine
/// `dynrelocs` does not handle.
pub(crate) fn resolve(file: &object::File) -> Vec<(u64, Vec<u8>)> {
    let mut out = Vec::new();
    if file.format() != object::BinaryFormat::Elf || file.kind() == object::ObjectKind::Relocatable {
        return out;
    }
    let Some((_, r_glob_dat, _)) = crate::loader::kuna_dynrelocs::reloc_triple(file.architecture()) else {
        return out;
    };
    let Some(relocs) = file.dynamic_relocations() else {
        return out;
    };
    let imports: std::collections::HashMap<usize, Vec<u8>> = file
        .dynamic_symbols()
        .filter(|s| s.is_undefined())
        .filter_map(|s| {
            let name = crate::loader::elf_plt::strip_version(s.name_bytes().ok()?);
            (!name.is_empty()).then_some((s.index().0, name))
        })
        .collect();
    for (offset, reloc) in relocs {
        if reloc.flags() != (RelocationFlags::Elf { r_type: r_glob_dat }) {
            continue;
        }
        if let RelocationTarget::Symbol(idx) = reloc.target() {
            if let Some(name) = imports.get(&idx.0) {
                out.push((offset, name.clone()));
            }
        }
    }
    out
}

/// The `[slot, slot + ptr)` ranges of [`resolve`]'s slots.
pub(crate) fn slots(file: &object::File) -> Vec<(u64, u64)> {
    let width = if file.is_64() { 8 } else { 4 };
    resolve(file).into_iter().map(|(slot, _)| (slot, slot + width)).collect()
}

/// Names each undefined-symbol `GLOB_DAT` slot for its import (a symbol-table
/// name, not a function-inventory entry) and paints it `externref`. Its output is
/// committed only when `importdata` is on.
pub struct ElfGotImportPass;

impl AnalysisPass for ElfGotImportPass {
    fn phase(&self) -> Phase {
        Phase::P1
    }

    fn id(&self) -> &'static str {
        "elfgotimports"
    }

    fn run(&self, ctx: &AnalysisCtx) -> AnalysisOutput {
        let mut out = AnalysisOutput::default();
        let width = if ctx.file.is_64() { 8 } else { 4 };
        for (slot, name) in resolve(ctx.file) {
            out.externref.push((slot, slot + width));
            out.import_slot_names.push((slot, String::from_utf8_lossy(&name).into_owned()));
        }
        out
    }
}
