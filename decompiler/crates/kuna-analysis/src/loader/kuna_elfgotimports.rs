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

use kuna_decomp::kuna_symbolnamechars::{sanitize_symbol_name_bytes, symbolnamechars_mode, NameChars};
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

/// [`resolve`]'s slots with their `[slot, slot + ptr)` ranges. A slot whose word
/// would run past the end of the address space is dropped.
fn ranged(file: &object::File) -> Vec<((u64, u64), Vec<u8>)> {
    let width = if file.is_64() { 8 } else { 4 };
    resolve(file).into_iter().filter_map(|(slot, name)| Some(((slot, slot.checked_add(width)?), name))).collect()
}

/// The `[slot, slot + ptr)` ranges of [`resolve`]'s slots.
pub(crate) fn slots(file: &object::File) -> Vec<(u64, u64)> {
    ranged(file).into_iter().map(|(range, _)| range).collect()
}

/// The name a slot is minted under: the import's bytes through the same
/// `symbolnamechars` sanitizer every other name the loader mints goes through.
fn minted_name(name: &[u8], mode: NameChars) -> String {
    String::from_utf8_lossy(&sanitize_symbol_name_bytes(name, mode)).into_owned()
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
        let mode = symbolnamechars_mode();
        for ((slot, end), name) in ranged(ctx.file) {
            out.externref.push((slot, end));
            out.import_slot_names.push((slot, minted_name(&name, mode)));
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use object::read::{Object, ObjectSection};

    use super::{minted_name, ranged, resolve, slots, NameChars};

    fn fixture() -> Vec<u8> {
        std::fs::read(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/elf_gotdata_x86_64.so")).unwrap()
    }

    #[test]
    fn a_slot_name_is_sanitized_like_every_minted_name() {
        assert_eq!(minted_name(b"stdout", NameChars::Safe), "stdout");
        let hostile = minted_name(b"imported_c\nunter*/x", NameChars::Safe);
        assert!(!hostile.contains('\n') && !hostile.contains("*/"), "{hostile:?}");
    }

    #[test]
    fn a_slot_at_the_top_of_the_address_space_is_dropped() {
        let mut bytes = fixture();
        let (off, len) = {
            let file = object::File::parse(&*bytes).unwrap();
            assert_eq!(slots(&file).len(), 3);
            file.section_by_name(".rela.dyn").unwrap().file_range().unwrap()
        };
        let stdout_slot = 0x2690u64.to_le_bytes();
        let at = (off..off + len)
            .step_by(24)
            .find(|&at| bytes[at as usize..at as usize + 8] == stdout_slot)
            .unwrap() as usize;
        bytes[at..at + 8].copy_from_slice(&0xffff_ffff_ffff_fffcu64.to_le_bytes());
        let file = object::File::parse(&*bytes).unwrap();
        assert!(resolve(&file).iter().any(|&(slot, _)| slot == 0xffff_ffff_ffff_fffc));
        assert_eq!(slots(&file), [(0x2680, 0x2688), (0x2688, 0x2690)]);
        assert_eq!(ranged(&file).len(), 2);
    }
}
