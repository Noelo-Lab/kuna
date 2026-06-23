//! MIPS `$gp` (global-pointer) recovery via per-function `t9` register-value
//! tracking — the kuna analog of Ghidra's `MipsAddressAnalyzer`.
//!
//! Unlike ARM `TMode` ([`crate::s1_loader::arm_markers`]), which paints a SLEIGH
//! decode-mode context *bit*, MIPS `$gp` recovery is a **tracked register VALUE**:
//! a constant seeded at a register at a function entry that the decompiler's
//! constant-value analysis then propagates. The seam it uses is kuna's
//! `TrackedSet`/`TrackedContext` (`globalcontext.rs`), driven exactly like the
//! console `set track <reg> <val> <start> <end>` command (`IfcSettrackedrange`):
//! the commit seam (`engine.rs::commit_analysis_output`) calls
//! `create_set(func_addr, func_addr+1)` + pushes `TrackedContext{loc, val}`, the
//! per-function `build_arch_handle` snapshots the track base into the seam, and
//! `ActionConstbase` (S3) emits `COPY #val -> reg` at the entry block so constant
//! propagation can resolve `$gp`-relative GOT/`.sdata` loads to real addresses.
//!
//! ## Why `t9`, not `gp`
//!
//! Ghidra's `MipsAddressAnalyzer` recovers `$gp` with **two** register-value
//! mechanisms (`MipsAddressAnalyzer.java`):
//!
//! 1. **`gp` global-assumption** (`checkForGlobalGP:137-191` + `flowConstants:243`):
//!    derive a program-wide `gp_assumption_value` from the `_mips_gp_value` /
//!    `_gp` / `_GP` symbol and `setRegisterValue(func.getEntryPoint(), …, gp=val)`
//!    at every function entry (default-on via `discoverGlobalGPSetting`).
//! 2. **`t9` entry-address assumption** (`assumeT9EntryAddress`, **default true**,
//!    `OPTION_DEFAULT_ASSUME_T9_ENTRY:64`): in the symbolic propagator's
//!    `unknownValue:481-494` callback, an *unknown* `t9` read is assumed to be the
//!    function entry — `setRegisterValue(funcAddr, funcAddr, t9=funcAddr)` — the
//!    MIPS PIC ABI convention (a function is reached via `jalr t9`, so on entry
//!    `t9` holds the callee's own entry address). A PIC prologue then computes
//!    `gp = _gp_disp + t9` (`lui gp; addiu gp; addu gp,gp,t9`), which folds to the
//!    real `$gp`.
//!
//! kuna ports mechanism **(2) `t9 = func_entry`** as the broadly-correct,
//! parity-safe choice for this tier. mechanism (1) — seeding `gp` directly — is
//! **deliberately NOT** ported here, because kuna's `ActionConstbase` is the
//! *unconditional* `COPY #val -> reg` form (it lacks Ghidra's *lazy* `unknownValue`
//! propagator that only assumes a value when the register is genuinely unknown and
//! `clearRegister`s it after a PIC call). An unconditional `COPY #_gp -> gp` at
//! entry is **counterproductive on a PIC function**: such a function recomputes
//! `gp` from `t9` (`addu gp,gp,t9`), and a pre-seeded `gp` interacts with the
//! recompute to poison the value (empirically the `$gp`-relative load then fails to
//! resolve), whereas seeding `t9 = entry` resolves it cleanly. Seeding `t9` is
//! always correct (the ABI guarantees `t9 == entry` on entry) and never poisons a
//! non-PIC function (which ignores `t9`). The `gp`-direct seam (for a non-PIC
//! function that loads `$gp` off the stack and never recomputes it) is a documented
//! follow-up requiring the lazy-`unknownValue` propagator — see LOSS below.
//!
//! ## Recovering the GP value (documentation / cross-check)
//!
//! [`recover_gp_value`] recovers the program-wide `$gp` from the ELF symbol table,
//! the faithful analog of `checkForGlobalGP`: prefer `_mips_gp_value`
//! (`MIPS_ElfExtension.MIPS_GP_VALUE_SYMBOL`), else `_gp`/`_GP`. The MIPS ABI
//! convention is `_gp = .got + 0x7ff0` (the GP bias centring the 16-bit signed
//! `$gp`-relative offset window over the GOT). It is exposed for the `t9`-path's
//! documentation/tests and for a future `gp`-direct seam; the seeded fact uses
//! `t9 = func_entry` (above), not this value.

use object::read::{Object, ObjectSymbol};
use object::SymbolKind;

use crate::pass::{AnalysisCtx, AnalysisOutput, AnalysisPass, Stage, TrackedRegFact};

/// The MIPS PIC global-pointer setup register. A function is reached via
/// `jalr t9`, so on entry `t9` holds the callee's own entry address; the PIC
/// prologue computes `gp = _gp_disp + t9`. Faithful to `MipsAddressAnalyzer`'s
/// `t9 = program.getRegister("t9")` (`MipsAddressAnalyzer.java:103`).
const T9: &str = "t9";

/// The MIPS GP bias: `_gp` sits `0x7ff0` past the GOT base so the signed 16-bit
/// `$gp`-relative offset window `[-0x8000, 0x7fff]` is centred over the GOT. A
/// MIPS ABI invariant (`_gp = .got + 0x7ff0`); used by [`recover_gp_value`] only
/// for the documented cross-check, never to seed the `t9` fact.
pub const MIPS_GP_BIAS: u64 = 0x7ff0;

/// Recover the program-wide `$gp` value from the ELF symbol table — the kuna
/// analog of `MipsAddressAnalyzer.checkForGlobalGP` (`:137-191`). Prefers
/// `_mips_gp_value` (`MIPS_ElfExtension.MIPS_GP_VALUE_SYMBOL`), then `_gp`/`_GP`.
/// Returns `None` if no such symbol is present (a stripped binary). This is the
/// global GP; per the module note the seeded fact uses `t9 = func_entry`, not this
/// value, but it is recovered for cross-checking and the future `gp`-direct seam.
pub fn recover_gp_value(file: &object::File) -> Option<u64> {
    let mut gp: Option<u64> = None;
    for sym in file.symbols().chain(file.dynamic_symbols()) {
        let Ok(name) = sym.name() else { continue };
        match name {
            // _mips_gp_value wins outright (the loader-provided value).
            "_mips_gp_value" => return Some(sym.address()),
            "_gp" | "_GP" if gp.is_none() => gp = Some(sym.address()),
            _ => {}
        }
    }
    gp
}

/// Port of `MipsAddressAnalyzer`'s per-function `t9 = func_entry` register-value
/// seeding (the `assumeT9EntryAddress` default-on convention). Emits one
/// [`TrackedRegFact`] (`t9 = addr`) for each MIPS function entry, so a PIC
/// `addu gp,gp,t9` prologue folds to the real `$gp` and `$gp`-relative loads
/// resolve. MIPS-only: empty output on every other language (the analog of
/// `MipsAddressAnalyzer.canAnalyze == processor==MIPS`).
#[derive(Clone, Copy, Default)]
pub struct MipsMarkerPass;

/// Scan an ELF object's FUNC symbols and emit a `t9 = entry` [`TrackedRegFact`]
/// per defined function. Shared by [`AnalysisPass::run`] and the unit tests.
///
/// Mirrors `MipsAddressAnalyzer`'s gate + the `t9 = func.getEntryPoint()`
/// assumption: gated on a MIPS ELF object, one fact per STT_FUNC with a defined
/// (non-zero, non-UND) address. Undefined imports (`.dynsym` address 0) are
/// skipped — a stub has no body to seed. Idempotent against the funcsym stream:
/// the commit seam keys the tracked set by the entry address.
fn scan_mips_markers(file: &object::File) -> AnalysisOutput {
    let mut out = AnalysisOutput::default();
    // canAnalyze gate: MIPS only. On any other language the pass contributes
    // nothing (== canAnalyze false). MIPS32 and MIPS64 both report Architecture::Mips
    // (object) / Mips64 — accept both.
    if !matches!(file.architecture(), object::Architecture::Mips | object::Architecture::Mips64) {
        return out;
    }
    // Only ELF carries these conventions (the only format kuna loads for MIPS).
    if !matches!(file.format(), object::BinaryFormat::Elf) {
        return out;
    }

    for sym in file.symbols().chain(file.dynamic_symbols()) {
        // Only defined functions: STT_FUNC with a real (non-zero, defined)
        // address. An undefined import (`.dynsym` UND, address 0) has no body to
        // seed; a $-prefixed local/section symbol is not a function entry.
        if sym.kind() != SymbolKind::Text {
            continue;
        }
        if sym.is_undefined() {
            continue;
        }
        let addr = sym.address();
        if addr == 0 {
            continue;
        }
        // assumeT9EntryAddress: t9 == the function's own entry address on entry
        // (the MIPS PIC `jalr t9` convention). MipsAddressAnalyzer.unknownValue:492
        // setRegisterValue(funcAddr, funcAddr, t9=funcAddr).
        out.tracked_regs.push(TrackedRegFact { func_addr: addr, reg: T9, value: addr });
    }

    out
}

impl AnalysisPass for MipsMarkerPass {
    fn stage(&self) -> Stage {
        Stage::S1
    }

    fn id(&self) -> &'static str {
        "mips_gp"
    }

    fn run(&self, ctx: &AnalysisCtx) -> AnalysisOutput {
        scan_mips_markers(ctx.file)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The dynamic MIPS LE fixture (a linked ET_DYN with PT_LOAD, so the decode
    /// e2e works — unlike the ARM `.o`, this host HAS a MIPS toolchain).
    fn mips_gp() -> Vec<u8> {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/mips_gp_le32");
        std::fs::read(path).expect("read mips_gp_le32 fixture")
    }

    fn t9_seed_at(out: &AnalysisOutput, addr: u64) -> Option<u64> {
        out.tracked_regs
            .iter()
            .find(|t| t.func_addr == addr && t.reg == T9)
            .map(|t| t.value)
    }

    /// The merge-blocking gate: the pass over the MIPS fixture must emit a
    /// `t9 = entry` fact for each defined function, with the value EQUAL to the
    /// entry address (the PIC `jalr t9` convention). Pins the fixture's VMAs.
    #[test]
    fn mips_markers_emit_t9_facts() {
        let bytes = mips_gp();
        let file = object::File::parse(bytes.as_slice()).expect("parse mips_gp_le32");
        let out = scan_mips_markers(&file);

        assert!(!out.tracked_regs.is_empty(), "must emit at least one t9 seed");
        // Every fact seeds t9 to its own func_addr (t9 == entry).
        for t in &out.tracked_regs {
            assert_eq!(t.reg, T9, "only t9 is seeded");
            assert_eq!(t.value, t.func_addr, "t9 == function entry (PIC convention)");
            assert_ne!(t.func_addr, 0, "no UND/zero-address function is seeded");
        }

        // Pin the load-bearing entries: `_init`@0x4004cc (PIC `addu gp,gp,t9`),
        // `main`@0x400704, `bump`@0x4006f0.
        assert_eq!(t9_seed_at(&out, 0x4004cc), Some(0x4004cc), "_init t9 = entry");
        assert_eq!(t9_seed_at(&out, 0x400704), Some(0x400704), "main t9 = entry");
        assert_eq!(t9_seed_at(&out, 0x4006f0), Some(0x4006f0), "bump t9 = entry");
    }

    /// `_gp` recovery cross-check (the global GP, used for documentation, not the
    /// seeded fact): `recover_gp_value` reads `_gp` = 0x419030, and the MIPS GP
    /// bias invariant `_gp == .got + 0x7ff0` holds (`.got` = 0x411040).
    #[test]
    fn mips_gp_value_recovered_and_bias_holds() {
        let bytes = mips_gp();
        let file = object::File::parse(bytes.as_slice()).expect("parse mips_gp_le32");
        let gp = recover_gp_value(&file).expect("fixture carries a _gp symbol");
        assert_eq!(gp, 0x419030, "_gp value from the symbol table");
        // _gp = .got + 0x7ff0 (the MIPS GP-bias convention).
        assert_eq!(gp, 0x411040 + MIPS_GP_BIAS, "_gp == .got + 0x7ff0 (GP bias)");
    }

    /// A non-MIPS object must emit ZERO facts — the `canAnalyze == processor==MIPS`
    /// gate. Driven over the x86-64 fauxware fixture: without this gate a `t9` seed
    /// on a non-MIPS language would reach the commit seam (which must also no-op,
    /// but the pass-level gate is the first line of defence).
    #[test]
    fn non_mips_object_emits_nothing() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/fauxware");
        let bytes = std::fs::read(path).expect("read fauxware fixture");
        let file = object::File::parse(bytes.as_slice()).expect("parse fauxware");
        assert_ne!(file.architecture(), object::Architecture::Mips);
        let out = scan_mips_markers(&file);
        assert!(
            out.tracked_regs.is_empty(),
            "a non-MIPS object must emit no t9 seeds (canAnalyze gate)"
        );
    }
}
