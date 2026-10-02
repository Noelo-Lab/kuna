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
//! `$t`/STT_FUNC-LSB → `TMode=1` (Thumb), `$a` → `TMode=0` (ARM). The commit boundary
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
//! Belt-and-suspenders: the commit boundary additionally swallows the
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
//! - **The Thumb-FUNC address shift is now re-homed.** Ghidra *moves* a Thumb
//!   `FUNC` symbol from `entry|1` to `entry` (`moveFunction`/`setElfSymbolAddress`)
//!   and renames it; kuna's `read_loader_symbols` installs the `FUNC` at the odd
//!   address. This pass additionally emits a [`SymFact`] (`SymKind::Function`) at
//!   the normalized (even) entry — the address the `TMode=1` paint and the engine's
//!   decode use — so `load function <name>` and a CALL resolve there. The commit
//!   boundary's symbol arm is idempotent (it skips an already-installed function), so
//!   the odd- and even-address installs coexist without a double-symbol collision.
//! - **MIPS `$gp` / MIPS16 `ISA_MODE` are out of scope.** MIPS `$gp` is not a
//!   decode-mode context paint at all — it is a *tracked register value* set at
//!   each function entry (`MipsAddressAnalyzer.flowConstants`,
//!   `MIPS_ElfExtension.java:508-515` sources `_mips_gp_value`) and consumed by
//!   constant propagation (S3). MIPS16 `ISA_MODE` is the exact `$t`/STT_FUNC-LSB
//!   analog (`MIPS_ElfExtension.applyIsaMode:412-432`) and could be added here
//!   with the identical mechanism (var `"ISA_MODE"`) given a MIPS16 fixture.
//!   Both are deferred — see `docs/history/analysis-port-plan.md` (`arch-markers` risks).

use object::read::{Object, ObjectSymbol};
use object::SymbolKind;

use crate::loader::kuna_armfloatabi;
use crate::pass::{AnalysisCtx, AnalysisOutput, AnalysisPass, ContextPaint, Phase, SymFact, SymKind};

/// The SLEIGH processor-context variable steering ARM/Thumb instruction decode.
/// Defined by the ARM `.pspec` (`ARMt.pspec` `<context_data>`); `0` = ARM (A32),
/// `1` = Thumb (T16/T32). Faithful to `ARM_ElfExtension.java`'s `TMode`.
const TMODE: &str = "TMode";

/// Port of the ARM mapping-symbol + STT_FUNC-LSB → `TMode` decode-mode painting
/// (`ARM_ElfExtension.evaluateElfSymbol` + `ArmSymbolAnalyzer`). Additive,
/// never-failing: emits [`ContextPaint`] facts the commit boundary applies to the
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
///
/// Without mapping symbols, on an image that has a Thumb STT_FUNC and whose
/// build attributes allow A32 code ([`kuna_armfloatabi::thumb_only`] is false),
/// the address bit is read both ways: an even STT_FUNC paints `TMode=0` over
/// exactly `[st_value, st_value + st_size)`, so a Thumb function's paint skips
/// the A32 function instead of running across it, and resumes after it over code
/// no symbol names. One without a size (crt `_init`/`_fini`, assembly without
/// `.size`) paints up to the next function symbol, or to the next change point
/// when it is the last. An image with no Thumb STT_FUNC has no such paint to
/// bound, and keeps the plain scan. Mapping symbols, where present, already delimit every
/// mode run and take precedence. An image whose odd `e_entry` is its only Thumb
/// evidence ([`unmarked_thumb_entry`]) keeps the plain scan, as it keeps the
/// plain Listing walk.
pub(crate) fn scan_arm_markers(file: &object::File) -> AnalysisOutput {
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
    let mut arm_functions = Vec::new();
    let mut thumb_functions = false;
    let mut mapped = false;

    for sym in file.symbols().chain(file.dynamic_symbols()) {
        let Ok(name) = sym.name() else { continue };
        let addr = sym.address();

        // (1) ARM mapping symbols — name-matched (STT_NOTYPE local symbols, so
        // `kind()` is not Text; do NOT filter on kind here).
        if is_thumb_marker(name) {
            // ARM_ElfExtension.java:166-175 — setValue(TMode, addr, addr, 1).
            out.context_paints.push(ContextPaint { addr, end: None, var: TMODE, value: 1 });
            mapped = true;
            continue;
        }
        if is_arm_marker(name) {
            // ARM_ElfExtension.java:176-185 — setValue(TMode, addr, addr, 0).
            out.context_paints.push(ContextPaint { addr, end: None, var: TMODE, value: 0 });
            mapped = true;
            continue;
        }
        // `$d`/`$d.` (data, :189) and `$b` (:186): LOSS — no createUndefinedData
        // equivalent at this tier; emit nothing.

        // (2) STT_FUNC with the Thumb odd-address (LSB) convention. ARM records a
        // Thumb function symbol at `entry|1`. ARM_ElfExtension.java:197-204 (the
        // STT_FUNC fallback) + creatingFunction:117-141 mask the LSB and set
        // TMode=1 at the even entry, AND *move* the FUNC symbol from `entry|1` to
        // `entry` (moveFunction/setElfSymbolAddress).
        if sym.kind() == SymbolKind::Text && (addr & 1) != 0 {
            let even = addr & !1;
            out.context_paints.push(ContextPaint { addr: even, end: None, var: TMODE, value: 1 });
            thumb_functions |= sym.is_definition();
            // Thumb-FUNC re-home: emit the FunctionSymbol at the EVEN entry so
            // `load function <name>` / a CALL resolves at the address the `TMode`
            // paint uses (the engine decodes at the even address). Ghidra moves the
            // symbol; kuna's `read_loader_symbols` installs it at the ODD address,
            // so this additive SymFact makes the even entry known too. The commit
            // boundary's symbol arm is idempotent (it skips an already-installed
            // function), so the odd-address install and this even-address one
            // coexist without a double-symbol collision. Named symbols only — an
            // unnamed `.symtab` entry has no name to re-home (a NUL name is skipped
            // by the `is_empty` guard, exactly as Ghidra leaves an unnamed FUNC).
            if !name.is_empty() {
                out.symbols.push(SymFact {
                    addr: even,
                    name: name.to_string(),
                    kind: SymKind::Function,
                });
            }
        } else if sym.kind() == SymbolKind::Text && sym.is_definition() {
            arm_functions.push((addr, sym.size()));
        }
    }

    if mapped || !thumb_functions || kuna_armfloatabi::thumb_only(file) || unmarked_thumb_entry(file)
    {
        return out;
    }
    let mut starts: Vec<u64> = file
        .symbols()
        .chain(file.dynamic_symbols())
        .filter(|sym| sym.kind() == SymbolKind::Text && sym.is_definition())
        .map(|sym| sym.address() & !1)
        .collect();
    starts.sort_unstable();
    starts.dedup();
    for (addr, size) in arm_functions {
        let end = match size {
            0 => starts.get(starts.partition_point(|&at| at <= addr)).copied(),
            size => Some(addr.saturating_add(size)),
        };
        out.context_paints.push(ContextPaint { addr, end, var: TMODE, value: 0 });
    }

    out
}

/// Whether `file` carries an ARM mapping symbol (`$a` or `$t`).
pub(crate) fn has_mapping_symbols(file: &object::File) -> bool {
    file.symbols().chain(file.dynamic_symbols()).any(|sym| {
        let name = sym.name().unwrap_or_default();
        is_thumb_marker(name) || is_arm_marker(name)
    })
}

/// Whether `file` is an ARM ELF whose odd `e_entry` is its only Thumb evidence:
/// no mapping symbol anywhere and no function symbol at the entry. Nothing in
/// such an image says where its Thumb code ends, so it keeps the plain marker
/// scan and the plain Listing walk.
pub(crate) fn unmarked_thumb_entry(file: &object::File) -> bool {
    let entry = file.entry();
    file.architecture() == object::Architecture::Arm
        && matches!(file.format(), object::BinaryFormat::Elf)
        && entry & 1 != 0
        && !file.symbols().chain(file.dynamic_symbols()).any(|sym| {
            let name = sym.name().unwrap_or_default();
            is_thumb_marker(name)
                || is_arm_marker(name)
                || (sym.kind() == SymbolKind::Text && sym.address() == entry)
        })
}

impl AnalysisPass for ArmMarkerPass {
    fn phase(&self) -> Phase {
        Phase::P1
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

        // Thumb-FUNC re-home: each LSB-set STT_FUNC also surfaces as a Function
        // SymFact at the NORMALIZED even entry (thumb_add@0x1 -> 0x0, _start@0x15 ->
        // 0x14), so `load function <name>` resolves at the even decode address.
        let homed = |a: u64, n: &str| {
            out.symbols.iter().any(|s| {
                s.addr == a && s.name == n && s.kind == SymKind::Function
            })
        };
        assert!(homed(0x0, "thumb_add"), "thumb_add re-homed to even 0x0");
        assert!(homed(0x14, "_start"), "_start re-homed to even 0x14");
        // Every re-homed function symbol is at an even address.
        assert!(
            out.symbols.iter().all(|s| s.addr & 1 == 0),
            "re-homed function symbols must be at even addresses"
        );
    }

    /// A non-ARM object must emit ZERO paints — the `canAnalyze == processor==ARM`
    /// gate. This is the regression guard: without it, a `TMode` paint on a
    /// non-ARM language would reach the commit boundary (which must also no-op, but the
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

    /// An ARM ELF with a Thumb FUNC at 0x41, an A32 FUNC at 0x80, the given
    /// `e_entry` and, optionally, a `Tag_CPU_arch_profile` attribute.
    fn interwork_elf(entry: u32, profile: Option<u8>) -> Vec<u8> {
        functions_elf(&[("thumb_helper", 0x41), ("arm_helper", 0x80)], entry, profile)
    }

    /// An ARM ELF with the given FUNC symbols, `e_entry` and, optionally, a
    /// `Tag_CPU_arch_profile` attribute.
    fn functions_elf(functions: &[(&str, u64)], entry: u32, profile: Option<u8>) -> Vec<u8> {
        use object::write::{Object, Symbol, SymbolSection};
        use object::{Architecture, BinaryFormat, Endianness, SectionKind, SymbolFlags, SymbolScope};
        let mut obj = Object::new(BinaryFormat::Elf, Architecture::Arm, Endianness::Little);
        let text = obj.add_section(Vec::new(), b".text".to_vec(), SectionKind::Text);
        obj.append_section_data(text, &[0; 0x110], 4);
        for &(name, value) in functions {
            obj.add_symbol(Symbol {
                name: name.as_bytes().to_vec(),
                value,
                size: 4,
                kind: SymbolKind::Text,
                scope: SymbolScope::Linkage,
                weak: false,
                section: SymbolSection::Section(text),
                flags: SymbolFlags::None,
            });
        }
        if let Some(profile) = profile {
            let attrs = obj.add_section(Vec::new(), b".ARM.attributes".to_vec(), SectionKind::Other);
            let mut data = vec![b'A', 17, 0, 0, 0];
            data.extend(b"aeabi\0");
            data.extend([1, 7, 0, 0, 0, 7, profile]);
            obj.append_section_data(attrs, &data, 1);
        }
        let mut bytes = obj.write().unwrap();
        bytes[24..28].copy_from_slice(&entry.to_le_bytes());
        bytes
    }

    /// Where A32 code may exist, an even FUNC paints A32 over its own extent
    /// only, whether the entry is A32 or a named Thumb function.
    #[test]
    fn an_even_function_is_arm_evidence_over_its_size() {
        for entry in [0x80, 0x41] {
            let bytes = interwork_elf(entry, Some(b'A'));
            let file = object::File::parse(bytes.as_slice()).unwrap();
            assert!(!unmarked_thumb_entry(&file));
            let out = scan_arm_markers(&file);
            assert_eq!(paints_at(&out, 0x40), vec![1]);
            assert_eq!(paints_at(&out, 0x80), vec![0]);
            let ends: Vec<_> = out.context_paints.iter().map(|p| (p.addr, p.end)).collect();
            assert!(ends.contains(&(0x40, None)));
            assert!(ends.contains(&(0x80, Some(0x84))));
        }
    }

    /// An even FUNC without a size paints up to the next function symbol, or
    /// to the next change point when no function symbol follows it.
    #[test]
    fn an_even_function_without_a_size_paints_up_to_the_next_function() {
        use object::write::{Object, Symbol, SymbolSection};
        use object::{Architecture, BinaryFormat, Endianness, SectionKind, SymbolFlags, SymbolScope};
        let mut obj = Object::new(BinaryFormat::Elf, Architecture::Arm, Endianness::Little);
        let text = obj.add_section(Vec::new(), b".text".to_vec(), SectionKind::Text);
        obj.append_section_data(text, &[0; 0x110], 4);
        for (name, value, size) in [
            ("thumb_helper", 0x41, 4),
            ("arm_helper", 0x80, 0),
            ("thumb_after", 0xc1, 4),
            ("arm_last", 0x100, 0),
        ] {
            obj.add_symbol(Symbol {
                name: name.as_bytes().to_vec(),
                value,
                size,
                kind: SymbolKind::Text,
                scope: SymbolScope::Linkage,
                weak: false,
                section: SymbolSection::Section(text),
                flags: SymbolFlags::None,
            });
        }
        let mut bytes = obj.write().unwrap();
        bytes[24..28].copy_from_slice(&0x41u32.to_le_bytes());
        let out = scan_arm_markers(&object::File::parse(bytes.as_slice()).unwrap());
        assert_eq!(paints_at(&out, 0x40), vec![1]);
        let ends: Vec<_> = out.context_paints.iter().map(|p| (p.addr, p.end, p.value)).collect();
        assert!(ends.contains(&(0x80, Some(0xc0), 0)));
        assert!(ends.contains(&(0x100, None, 0)));
    }

    /// With no Thumb FUNC there is no Thumb paint for an even FUNC to stop, so
    /// an image of A32 functions keeps the plain scan.
    #[test]
    fn an_image_without_a_thumb_function_paints_nothing() {
        let bytes = functions_elf(&[("first", 0x40), ("second", 0x80)], 0x40, Some(b'A'));
        let out = scan_arm_markers(&object::File::parse(bytes.as_slice()).unwrap());
        assert!(out.context_paints.is_empty());
    }

    /// An odd entry that no symbol names gives no extent for its Thumb code, so
    /// the scan paints only what the symbols say and nothing at the entry.
    #[test]
    fn an_unmarked_thumb_entry_keeps_the_plain_scan() {
        let bytes = interwork_elf(0x101, Some(b'A'));
        let file = object::File::parse(bytes.as_slice()).unwrap();
        assert!(unmarked_thumb_entry(&file));
        let out = scan_arm_markers(&file);
        assert_eq!(paints_at(&out, 0x40), vec![1]);
        assert!(paints_at(&out, 0x80).is_empty());
        assert!(paints_at(&out, 0x100).is_empty());
    }

    /// Mapping symbols delimit every run themselves, so they suppress the
    /// address-bit inference: a `$t` at an even FUNC keeps that function Thumb.
    #[test]
    fn mapping_symbols_take_precedence_over_the_address_bit() {
        use object::write::{Object, Symbol, SymbolSection};
        use object::{Architecture, BinaryFormat, Endianness, SectionKind, SymbolFlags, SymbolScope};
        let mut obj = Object::new(BinaryFormat::Elf, Architecture::Arm, Endianness::Little);
        let text = obj.add_section(Vec::new(), b".text".to_vec(), SectionKind::Text);
        obj.append_section_data(text, &[0; 0x10], 4);
        for (name, kind, scope) in [
            ("$t", SymbolKind::Label, SymbolScope::Compilation),
            ("thumb_entry", SymbolKind::Text, SymbolScope::Linkage),
        ] {
            obj.add_symbol(Symbol {
                name: name.as_bytes().to_vec(),
                value: 8,
                size: 0,
                kind,
                scope,
                weak: false,
                section: SymbolSection::Section(text),
                flags: SymbolFlags::None,
            });
        }
        let mut bytes = obj.write().unwrap();
        bytes[24..28].copy_from_slice(&0x1u32.to_le_bytes());
        let out = scan_arm_markers(&object::File::parse(bytes.as_slice()).unwrap());
        assert_eq!(paints_at(&out, 8), vec![1]);
        assert!(paints_at(&out, 0).is_empty());
    }

    /// An M-profile image has no A32 state, so only the Thumb FUNC paints.
    #[test]
    fn a_thumb_only_image_paints_only_its_thumb_functions() {
        let bytes = interwork_elf(0x41, Some(b'M'));
        let out = scan_arm_markers(&object::File::parse(bytes.as_slice()).unwrap());
        assert_eq!(paints_at(&out, 0x40), vec![1]);
        assert!(paints_at(&out, 0x80).is_empty());
    }
}
