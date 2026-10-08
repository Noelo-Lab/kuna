//! (kuna `armfuncmode`) An even ARM function symbol starts A32 code.
//!
//! Bit 0 of an ARM ELF `STT_FUNC` value is set for a Thumb function and clear
//! for an A32 one (AAELF32 "Symbol Values"). `arm_markers` paints the Thumb
//! half as a point paint, which fills up to the next point where the mode was
//! set; with no such point at the A32 function after it, the Thumb mode of the
//! function before it (or of a `blx` target before it) reaches it. Mapping
//! symbols (`$a`/`$t`) supply that point when the image has them; this pass
//! supplies it from the function symbol when the image has none: `TMode=0` over
//! each defined even function symbol's `[value, value + size)` in an executable
//! section that a Thumb function symbol's paint would otherwise reach, i.e. one
//! after the first odd function symbol. The extent is cut at the next function
//! symbol and at the section's end (a linker moves an exported Thumb function's
//! symbol onto its A32 interworking stub and keeps the function's size). Past
//! the extent the Thumb mode resumes, as the Thumb symbol's paint gave it
//! before, so unsymbolized Thumb code after an A32 function keeps its mode. A
//! symbol with size 0 says nothing about its extent and is skipped, and an
//! image whose function symbols are all even keeps the language default it
//! already has.
//!
//! Nothing is painted for an image with `$a`/`$t` mapping symbols, a
//! relocatable object (its section addresses are not where the loader places
//! the code), an image whose build attributes rule out A32, a Cortex-M image
//! with a vector table, or any non-ARM or non-ELF object. An address that also
//! carries an odd (Thumb) function symbol is left to `arm_markers`.

use std::collections::{BTreeMap, BTreeSet};

use object::read::{Object, ObjectSection, ObjectSymbol};
use object::{ObjectKind, SectionIndex, SymbolKind};

use crate::pass::{AnalysisCtx, AnalysisOutput, AnalysisPass, ContextPaint, Phase};

const TMODE: &str = "TMode";
const SHF_EXECINSTR: u64 = 0x4;

pub struct ArmFuncModePass;

fn is_mapping_mode_symbol(name: &str) -> bool {
    matches!(name, "$a" | "$t") || name.starts_with("$a.") || name.starts_with("$t.")
}

fn executable_end(file: &object::File, index: SectionIndex) -> Option<u64> {
    let section = file.section_by_index(index).ok()?;
    let object::SectionFlags::Elf { sh_flags } = section.flags() else {
        return None;
    };
    (sh_flags & SHF_EXECINSTR != 0).then(|| section.address().saturating_add(section.size()))
}

/// The `TMode` paints for the image's even function symbols (see the module
/// docs); empty whenever the image is out of scope.
pub fn arm_func_mode_paints(file: &object::File) -> Vec<ContextPaint> {
    if file.architecture() != object::Architecture::Arm
        || file.format() != object::BinaryFormat::Elf
        || file.kind() == ObjectKind::Relocatable
    {
        return Vec::new();
    }
    let symbols = || file.symbols().chain(file.dynamic_symbols());
    if symbols().any(|sym| sym.name().is_ok_and(is_mapping_mode_symbol)) {
        return Vec::new();
    }
    let functions: Vec<(u64, u64, u64)> = symbols()
        .filter(|sym| sym.kind() == SymbolKind::Text && sym.is_definition() && sym.address() != 0)
        .filter_map(|sym| Some((sym.address(), sym.size(), executable_end(file, sym.section_index()?)?)))
        .collect();
    let thumb: BTreeSet<u64> =
        functions.iter().filter(|(addr, ..)| addr & 1 != 0).map(|(addr, ..)| addr & !1).collect();
    let Some(&first_thumb) = thumb.first() else {
        return Vec::new();
    };
    let starts: BTreeSet<u64> = functions.iter().map(|(addr, ..)| addr & !1).collect();
    let next_start = |addr: u64| starts.range(addr + 1..).next().copied().unwrap_or(u64::MAX);
    let mut arm: BTreeMap<u64, u64> = BTreeMap::new();
    for &(addr, size, section_end) in functions.iter() {
        if addr & 1 != 0 || addr < first_thumb || size == 0 || thumb.contains(&addr) {
            continue;
        }
        let end = addr.saturating_add(size).min(next_start(addr)).min(section_end);
        let extent = arm.entry(addr).or_insert(end);
        *extent = (*extent).max(end);
    }
    if arm.is_empty()
        || super::kuna_armfloatabi::thumb_only(file)
        || !crate::analyzers::entry::cortexm_thumb_paints(file, true).is_empty()
    {
        return Vec::new();
    }
    let mut paints = Vec::new();
    for (addr, end) in arm.into_iter().filter(|(addr, end)| end > addr) {
        paints.push(ContextPaint { addr, end: Some(end), var: TMODE, value: 0 });
        if end < next_start(addr) {
            paints.push(ContextPaint { addr: end, end: None, var: TMODE, value: 1 });
        }
    }
    paints
}

impl AnalysisPass for ArmFuncModePass {
    fn phase(&self) -> Phase {
        Phase::P1
    }

    fn id(&self) -> &'static str {
        "armfuncmode"
    }

    fn run(&self, ctx: &AnalysisCtx) -> AnalysisOutput {
        AnalysisOutput { context_paints: arm_func_mode_paints(ctx.file), ..AnalysisOutput::default() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn paints(fixture: &str) -> Vec<(u64, Option<u64>, u32)> {
        let path = format!("{}/tests/fixtures/{fixture}", env!("CARGO_MANIFEST_DIR"));
        let bytes = std::fs::read(&path).expect("read fixture");
        let file = object::File::parse(bytes.as_slice()).expect("parse fixture");
        let out = arm_func_mode_paints(&file);
        assert!(out.iter().all(|p| p.var == TMODE));
        out.iter().map(|p| (p.addr, p.end, p.value)).collect()
    }

    #[test]
    fn even_function_symbols_after_a_thumb_symbol_are_a32_over_their_size() {
        let expected = vec![(0x0200_0080, Some(0x0200_0088), 0), (0x0200_0088, None, 1)];
        assert_eq!(paints("arm_funcmode_le32"), expected);
        assert_eq!(paints("arm_funcmode_nocall_le32"), expected);
    }

    #[test]
    fn size_zero_symbols_are_skipped() {
        assert!(paints("arm_funcmode_sizes_le32").is_empty());
    }

    #[test]
    fn all_even_function_symbols_paint_nothing() {
        assert!(paints("arm_interwork_stubs_o0_le32").is_empty());
    }

    #[test]
    fn mapping_symbols_relocatable_objects_and_other_architectures_paint_nothing() {
        assert!(paints("arm_funcmode_mapsyms_le32").is_empty());
        assert!(paints("arm_thumb_le32.o").is_empty());
        assert!(paints("fauxware").is_empty());
    }
}
