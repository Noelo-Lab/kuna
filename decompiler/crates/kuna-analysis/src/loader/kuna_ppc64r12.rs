//! (kuna `ppc64r12`) PPC64 ELFv2 global entries hold their own address in `r12`.
//!
//! The OpenPOWER ELFv2 ABI requires a caller that enters a function at its
//! *global* entry (an indirect call, a PLT stub, a cross-module call) to pass the
//! entry address in `r12`. The global entry derives the TOC pointer from it,
//! `addis r2,r12,hi; addi r2,r2,lo`, before falling into the local entry. Without
//! that fact `r2` is an unknown function input and every TOC-relative global
//! access prints as an offset from an uninitialised register
//! (`*(T *)(r12 + 0x22068) = p`), which also does not recompile.
//!
//! The pass seeds `r12 = entry` at each defined function whose symbol declares a
//! distinct local entry (`st_other`). That annotation is what says the function
//! begins with the `r12`-based TOC setup; a function without one keeps `r2` as
//! received. The kuna analog of the MIPS `t9 = entry` seed in
//! [`super::mips_markers`].

use object::read::{Object, ObjectSymbol};
use object::{SymbolFlags, SymbolKind};

use crate::pass::{AnalysisCtx, AnalysisOutput, AnalysisPass, Phase, TrackedRegFact};

/// The ELFv2 global-entry address register.
const R12: &str = "r12";

/// One `r12 = entry` fact per PPC64 function symbol with a distinct local entry.
fn scan_global_entries(file: &object::File) -> AnalysisOutput {
    let mut out = AnalysisOutput::default();
    // An image whose e_flags declare ELFv1 (1) gives st_other no entry meaning.
    if file.architecture() != object::Architecture::PowerPc64
        || !matches!(file.format(), object::BinaryFormat::Elf)
        || matches!(file.flags(), object::FileFlags::Elf { e_flags, .. } if e_flags & 3 == 1)
    {
        return out;
    }
    for sym in file.symbols().chain(file.dynamic_symbols()) {
        if sym.kind() != SymbolKind::Text || sym.is_undefined() || sym.address() == 0 {
            continue;
        }
        let SymbolFlags::Elf { st_other, .. } = sym.flags() else { continue };
        // Bits 5-7 of st_other: 2..=6 name a local entry 4..64 bytes in.
        if !matches!((st_other >> 5) & 7, 2..=6) {
            continue;
        }
        out.entry_tracks.push(TrackedRegFact { func_addr: sym.address(), reg: R12, value: sym.address() });
    }
    out
}

/// Seeds `r12` at PPC64 ELFv2 global entries; empty on every other image.
#[derive(Clone, Copy, Default)]
pub struct Ppc64GlobalEntryPass;

impl AnalysisPass for Ppc64GlobalEntryPass {
    fn phase(&self) -> Phase {
        Phase::P1
    }

    fn id(&self) -> &'static str {
        "ppc64r12"
    }

    fn run(&self, ctx: &AnalysisCtx) -> AnalysisOutput {
        if ctx.arch.get_register_varnode(R12.as_bytes()).is_err() {
            return AnalysisOutput::default();
        }
        scan_global_entries(ctx.file)
    }
}
