//! ARM/Thumb mapping-symbol → decode-mode (`TMode`) painting — the kuna analog
//! of Ghidra's ARM ELF-loader extension + `ArmSymbolAnalyzer`.
//!
//! Ghidra recovers the ARM/Thumb decode mode from **two** distinct ELF
//! conventions, and the named analyzer is not the only (or even the primary)
//! source:
//!
//! - **ARM mapping symbols** (`$t`/`$a`/`$d`, ARM ELF ABI): `$t` marks the start
//!   of a Thumb run, `$a` an ARM (A32) run, `$d` a data run. Ghidra paints the
//!   `TMode` processor-context variable per range:
//!   `Ghidra/Processors/ARM/.../elf/extend/ARM_ElfExtension.java:166-196`
//!   `evaluateElfSymbol`: the `$t`/`$t.` branch (`:166`) does
//!   `programContext.setValue(TMode, addr, addr, 1)`, `$a`/`$a.` (`:176`) sets
//!   `TMode=0`, `$d`/`$d.` (`:189`) lays undefined data (listing-only markup).
//! - **The STT_FUNC odd-address (LSB) convention**: a Thumb function symbol is
//!   recorded at `entry|1`. `ARM_ElfExtension.java:197-204` (the STT_FUNC
//!   fallback) and `creatingFunction:117-141` mask the LSB off and set
//!   `TMode=1` at the (even) entry. `Features/Base/.../analysis/ArmSymbolAnalyzer.java:48-100`
//!   is a separate later pass that re-applies only this LSB convention
//!   (`getProgramContext().setValue(TMode, newAddr, newAddr, 1)`); its
//!   `canAnalyze:172-177` requires `processor==ARM && getRegister("TMode")!=null`.
//!
//! This pass ports BOTH mechanisms as additive [`ContextPaint`] facts:
//! `$t`/STT_FUNC-LSB → `TMode=1` (Thumb), `$a` → `TMode=0` (ARM). The commit seam
//! (`kuna-console`'s `commit_analysis_output`) paints each over the engine's
//! `ContextDatabase`, the exact analog of `programContext.setValue(TMode,…)`.
//! `TMode` defaults to 0 (ARM) — registered by the ARM `.pspec`
//! (`ARMt.pspec` `<context_data><set name="TMode" val="0">`, applied by
//! `Architecture::parse_processor_config`), so painting 1 = Thumb.
//!
//! ## Gating (faithful to `ArmSymbolAnalyzer.canAnalyze`)
//!
//! The pass only fires on an ARM object (`object::Architecture::Arm`); on any
//! other language it returns an empty output, mirroring `canAnalyze == false`.
//! Belt-and-suspenders: the commit seam additionally swallows the
//! "`TMode` not registered" error so painting can never regress a non-ARM
//! decompile (see `commit_analysis_output`).
//!
//! ## LOSS / divergences from upstream
//!
//! - **`$d` data runs are a no-op.** Ghidra's `$d` branch lays *undefined data*
//!   (listing markup that protects the bytes from disassembly); kuna has no
//!   `createUndefinedData` equivalent at this tier, and the protection has
//!   negligible decompiler payoff (it only matters for the listing view). We do
//!   not emit a fact for `$d`/`$b`. Documented LOSS.
//! - **The Thumb-FUNC address shift is paint-only (v1).** Ghidra *moves* a Thumb
//!   `FUNC` symbol from `entry|1` to `entry` (`moveFunction`/`setElfSymbolAddress`)
//!   and renames it; kuna's `read_loader_symbols` installs the `FUNC` at the odd
//!   address. The conservative v1 here emits only the `TMode=1` paint at the
//!   normalized (even) address and does NOT re-home the function symbol (that
//!   risks double-symbols). The function-address normalization is a documented
//!   follow-up.
//! - **MIPS `$gp` / MIPS16 `ISA_MODE` are out of scope.** MIPS `$gp` is not a
//!   decode-mode context paint at all — it is a *tracked register value* set at
//!   each function entry (`MipsAddressAnalyzer.flowConstants`,
//!   `MIPS_ElfExtension.java:508-515` sources `_mips_gp_value`) and consumed by
//!   constant propagation (S3). MIPS16 `ISA_MODE` is the exact `$t`/STT_FUNC-LSB
//!   analog (`MIPS_ElfExtension.applyIsaMode:412-432`) and could be added here
//!   with the identical mechanism (var `"ISA_MODE"`) given a MIPS16 fixture.
//!   Both are deferred — see `docs/analysis-port-plan.md` (`arch-markers` risks).

use object::read::{Object, ObjectSymbol};
use object::SymbolKind;

use crate::pass::{AnalysisCtx, AnalysisOutput, AnalysisPass, ContextPaint, Stage};

/// The SLEIGH processor-context variable steering ARM/Thumb instruction decode.
/// Defined by the ARM `.pspec` (`ARMt.pspec` `<context_data>`); `0` = ARM (A32),
/// `1` = Thumb (T16/T32). Faithful to `ARM_ElfExtension.java`'s `TMode`.
const TMODE: &str = "TMode";

/// Port of the ARM mapping-symbol + STT_FUNC-LSB → `TMode` decode-mode painting
/// (`ARM_ElfExtension.evaluateElfSymbol` + `ArmSymbolAnalyzer`). Additive,
/// never-failing: emits [`ContextPaint`] facts the commit seam applies to the
/// engine's `ContextDatabase`. Fires only on an ARM object (the
/// `canAnalyze == processor==ARM` gate); empty on any other language.
#[derive(Clone, Copy, Default)]
pub struct ArmMarkerPass;

/// `true` if `name` is an ARM Thumb mapping symbol: exactly `$t` or a `$t.`
/// prefix (`$t.0`, `$t.1`, …). Faithful to `ARM_ElfExtension.java:166`
/// (`symName.equals("$t") || symName.startsWith("$t.")`).
fn is_thumb_marker(name: &str) -> bool {
    name == "$t" || name.starts_with("$t.")
}

/// `true` if `name` is an ARM (A32) mapping symbol: `$a` or `$a.` prefix.
/// Faithful to `ARM_ElfExtension.java:176`.
fn is_arm_marker(name: &str) -> bool {
    name == "$a" || name.starts_with("$a.")
}

/// Scan an ELF object's symbols for the two ARM decode-mode conventions and emit
/// [`ContextPaint`] facts. Shared by [`AnalysisPass::run`] and the unit tests.
///
/// Two passes over the symbol streams (`.symtab` + `.dynsym`, the same chain
/// `noreturn.rs` uses):
/// 1. **Mapping symbols** (`$t`/`$a`), matched by name regardless of `st_type`
///    (the ARM ABI records them as `STT_NOTYPE` local symbols). `$d`/`$b` are a
///    documented no-op (no `createUndefinedData` at this tier).
/// 2. **STT_FUNC with `st_value & 1`** (the Thumb odd-address convention): emit
///    `TMode=1` at the normalized even address `value & !1`.
fn scan_arm_markers(file: &object::File) -> AnalysisOutput {
    let mut out = AnalysisOutput::default();
    // canAnalyze gate: ARM only. Mirrors ArmSymbolAnalyzer.canAnalyze:172-177
    // (processor==ARM && getRegister("TMode")!=null). On any other language the
    // pass contributes nothing (== canAnalyze false).
    if file.architecture() != object::Architecture::Arm {
        return out;
    }
    // Only ARM ELF objects carry these conventions (the only format kuna loads).
    if !matches!(file.format(), object::BinaryFormat::Elf) {
        return out;
    }

    for sym in file.symbols().chain(file.dynamic_symbols()) {
        let Ok(name) = sym.name() else { continue };
        let addr = sym.address();

        // (1) ARM mapping symbols — name-matched (STT_NOTYPE local symbols, so
        // `kind()` is not Text; do NOT filter on kind here).
        if is_thumb_marker(name) {
            // ARM_ElfExtension.java:166-175 — setValue(TMode, addr, addr, 1).
            out.context_paints.push(ContextPaint { addr, end: None, var: TMODE, value: 1 });
            continue;
        }
        if is_arm_marker(name) {
            // ARM_ElfExtension.java:176-185 — setValue(TMode, addr, addr, 0).
            out.context_paints.push(ContextPaint { addr, end: None, var: TMODE, value: 0 });
            continue;
        }
        // `$d`/`$d.` (data, :189) and `$b` (:186): LOSS — no createUndefinedData
        // equivalent at this tier; emit nothing.

        // (2) STT_FUNC with the Thumb odd-address (LSB) convention. ARM records a
        // Thumb function symbol at `entry|1`. ARM_ElfExtension.java:197-204 (the
        // STT_FUNC fallback) + creatingFunction:117-141 mask the LSB and set
        // TMode=1 at the even entry. We paint only (the function-symbol re-home
        // is a documented follow-up).
        if sym.kind() == SymbolKind::Text && (addr & 1) != 0 {
            out.context_paints.push(ContextPaint {
                addr: addr & !1,
                end: None,
                var: TMODE,
                value: 1,
            });
        }
    }

    out
}

impl AnalysisPass for ArmMarkerPass {
    fn stage(&self) -> Stage {
        Stage::S1
    }

    fn id(&self) -> &'static str {
        "arm_markers"
    }

    fn run(&self, ctx: &AnalysisCtx) -> AnalysisOutput {
        scan_arm_markers(ctx.file)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The bare ARM Thumb `.o` fixture (ET_REL parses fine; the e2e on a LINKED
    /// exe is a documented follow-up — no ARM linker on the build host).
    fn arm_thumb_o() -> Vec<u8> {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/arm_thumb_le32.o");
        std::fs::read(path).expect("read arm_thumb_le32.o fixture")
    }

    fn paints_at(out: &AnalysisOutput, addr: u64) -> Vec<u32> {
        out.context_paints
            .iter()
            .filter(|p| p.addr == addr && p.var == TMODE)
            .map(|p| p.value)
            .collect()
    }

    #[test]
    fn marker_name_classification() {
        assert!(is_thumb_marker("$t"));
        assert!(is_thumb_marker("$t.0"));
        assert!(is_thumb_marker("$t.17"));
        assert!(!is_thumb_marker("$td"));
        assert!(is_arm_marker("$a"));
        assert!(is_arm_marker("$a.1"));
        // data / non-markers classify as neither.
        assert!(!is_thumb_marker("$d"));
        assert!(!is_arm_marker("$d.0"));
        assert!(!is_thumb_marker("thumb_add"));
        assert!(!is_arm_marker("main"));
    }

    /// The merge-blocking gate: the pass over the bare ARM `.o` must emit a Thumb
    /// (`TMode=1`) paint for the `$t.0` mapping symbol AND for each STT_FUNC whose
    /// `st_value & 1` (`thumb_add`@0x1, `_start`@0x15), with the function address
    /// normalized to even (0x0, 0x14). Pins the fixture's VMAs.
    #[test]
    fn arm_markers_emit_thumb_facts() {
        let bytes = arm_thumb_o();
        let file = object::File::parse(bytes.as_slice()).expect("parse arm_thumb_le32.o");
        let out = scan_arm_markers(&file);

        // Every paint is TMode and on this all-Thumb fixture is value 1.
        assert!(!out.context_paints.is_empty(), "must emit at least one TMode paint");
        for p in &out.context_paints {
            assert_eq!(p.var, TMODE, "only TMode is painted");
            assert_eq!(p.value, 1, "all-Thumb fixture paints TMode=1 everywhere");
            assert_eq!(p.end, None, "v1 emits point sets (paint-to-next-change-point)");
        }

        // (1) The `$t.0` mapping symbol at .text+0x0 → TMode=1.
        assert!(paints_at(&out, 0x0).contains(&1), "$t.0 must paint TMode=1 at 0x0");

        // (2) STT_FUNC LSB convention: thumb_add@0x1 → normalized 0x0, _start@0x15
        // → normalized 0x14, both TMode=1.
        assert!(
            paints_at(&out, 0x0).contains(&1),
            "thumb_add@0x1 must paint TMode=1 at normalized 0x0"
        );
        assert!(
            paints_at(&out, 0x14).contains(&1),
            "_start@0x15 must paint TMode=1 at normalized 0x14"
        );

        // No odd address is ever painted (LSB always masked off).
        assert!(
            out.context_paints.iter().all(|p| p.addr & 1 == 0),
            "function addresses must be normalized to even (LSB masked)"
        );
    }

    /// A non-ARM object must emit ZERO paints — the `canAnalyze == processor==ARM`
    /// gate. This is the regression guard: without it, a `TMode` paint on a
    /// non-ARM language would reach the commit seam (which must also no-op, but the
    /// pass-level gate is the first line). Driven over the x86-64 fauxware fixture.
    #[test]
    fn non_arm_object_emits_nothing() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/fauxware");
        let bytes = std::fs::read(path).expect("read fauxware fixture");
        let file = object::File::parse(bytes.as_slice()).expect("parse fauxware");
        assert_ne!(file.architecture(), object::Architecture::Arm);
        let out = scan_arm_markers(&file);
        assert!(
            out.context_paints.is_empty(),
            "a non-ARM object must emit no TMode paints (canAnalyze gate)"
        );
    }
}
