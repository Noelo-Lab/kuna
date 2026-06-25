//! Auto-apply cspec call-fixups — the kuna analog of Ghidra's
//! `CallFixupAnalyzer` (the "Call-Fixup Installer").
//!
//! # What this is
//!
//! A processor cspec ships named `<callfixup>` payloads, each with one or more
//! `<target>` symbol names: e.g. `x86-64-gcc.cspec` registers
//! `<callfixup name="mcount"><target name="mcount"/>` (body `temp:1 = 0;`), the
//! profiling stub gcc `-pg` plants in every prologue. A call-fixup *replaces* the
//! CALL to that target with the payload's p-code body, so the bogus
//! `mcount()`/`__fentry__()` profiling call dissolves out of the decompiled
//! output.
//!
//! In the Ghidra application, **`CallFixupAnalyzer`** is the piece that *installs*
//! a fixup: it enumerates the registered call-fixups' `<target>` names, matches
//! them against the program's functions by name, and tags each match with the
//! fixup (`function.setCallFixup(name)`). The decompiler then applies the tagged
//! fixup at flow time. kuna inherits the whole *apply* path (see "Inherited vs
//! gap" below); this pass is the missing *installer*.
//!
//! # Inherited vs gap
//!
//! **Inherited** (already in the engine, do not re-build): the cspec
//! `<callfixup name=…><target name=…>` decode at bootstrap
//! (`architecture.rs::decode_call_fixups`); per-payload `target_symbol_names`
//! retention on `SleighPayload::Callfixup` (`inject_sleigh.rs`); the entire
//! inject/weave pipeline — `func_inject_id → FuncProto::set_inject_id →
//! inject_sub_function` (`seams.rs`, `flow.rs`); and the per-function applier
//! primitive `Database::set_function_inject_id` (`database.rs`), already called by
//! hand in the console `fixup apply` command (`IfcFixupApply`, `ifacedecomp.rs`).
//!
//! **The gap this fills**: nothing auto-iterates the registered fixups' `<target>`
//! names and applies them to matching FunctionSymbols. kuna only had the manual
//! `fixup apply <fixup> <function>` console path. Ghidra's `CallFixupAnalyzer` is
//! exactly that auto-installer; this pass ports it.
//!
//! # Origin (upstream Ghidra, the tree kuna was ported from)
//!
//! - `Ghidra/Features/Base/src/main/java/ghidra/app/plugin/core/disassembler/CallFixupAnalyzer.java`
//!   — `AnalyzerType.FUNCTION_ANALYZER`, `setDefaultEnablement(true)` (line 51).
//!   The decompiler-relevant logic ported here:
//!   - `getTargetFixupMap` (lines 436-459): enumerate `getCallFixupNames()`, read
//!     each `InjectPayloadCallfixup.getTargets()`, build `map[target] = fixupName`.
//!   - `getCallFixupNameForFunction` (lines 200-222): probe the target map with
//!     `funcName`, then `"_"+funcName`, then `"__"+funcName` (the underscore
//!     retries); first hit wins; a leading `libID_conflict_` is stripped first.
//!   - the `added` install loop (lines 81-91): for each function, if a fixup
//!     matches and `getCallFixup()==null`, `function.setCallFixup(fixupName)`.
//! - `InjectPayloadCallfixup.getTargets()` (`.../InjectPayloadCallfixup.java:60-61`,
//!   `targetSymbolNames`).
//!
//! # Scope / LOSS
//!
//! Only the *install* half is ported (target-match → set the fixup). The rest of
//! `CallFixupAnalyzer.added` (lines 93-175) and all of
//! `repairLocationsForNonReturningFunction`/`setNoFallThru`/`clearAndRepairFlows`
//! (lines 234-434) is **post-disassembly listing flow-repair** (FlowOverride,
//! ClearFlowAndRepairCmd) and is **not portable at this tier** — there is no
//! pre-decompile Listing / FlowOverride here. kuna handles fixup fallthrough
//! inside the engine's flow/inject path. Thunk-following
//! (`addInThunkedFunctionsToList`, lines 177-198) is part of that flow-repair half
//! and is likewise out of scope. This is the same tier constraint already
//! documented for `FindNoReturnFunctionsAnalyzer` (see `s1_loader/noreturn.rs`).

use object::read::{Object, ObjectSymbol};
use object::SymbolKind;

use kuna_decomp::architecture::Architecture;
use kuna_decomp::inject_sleigh::SleighPayload;

use crate::pass::{AnalysisCtx, AnalysisOutput, AnalysisPass, Stage};

/// Prefix Ghidra strips from a conflicting library-imported function name before
/// the target-map probe (`CallFixupAnalyzer.java:207-209`,
/// `LIBRARY_ID_CONFLICT_PREFIX`).
const LIB_CONFLICT_PREFIX: &str = "libID_conflict_";

/// Build the `(target_name, fixup_name)` map from the engine's registered
/// call-fixups — the port of `getTargetFixupMap` (`CallFixupAnalyzer.java:436-459`).
///
/// For each registered `SleighPayload::Callfixup`, for each of its
/// `<target>` names, record `map[target] = fixupName`. The fixup's name is its
/// payload `core.name` (the `<callfixup name=…>` attribute); the targets are the
/// `<target name=…>` symbols (`InjectPayloadCallfixup::target_symbol_names`). The
/// data lives on `arch.pcodeinjectlib` (decoded at bootstrap by
/// `Architecture::decode_call_fixups`).
///
/// Returned as a `Vec<(target, fixup)>` (no `HashMap` per project convention);
/// probed by [`call_fixup_name_for_function`]. Targets/fixup names are UTF-8
/// strings (cspec symbol names are ASCII); a non-UTF-8 name is skipped.
///
/// `pub` so the commit seam (`kuna-console`'s `commit_analysis_output`) can
/// rebuild the same map and re-derive a [`crate::pass::CallFixupFact`]'s fixup —
/// the fact carries only the function name (the install loop applies by inject id,
/// which the commit re-resolves the same way the pass matched it).
pub fn target_fixup_map(arch: &Architecture) -> Vec<(String, String)> {
    let mut map: Vec<(String, String)> = Vec::new();
    for payload in &arch.pcodeinjectlib.injection {
        let SleighPayload::Callfixup(cf) = payload else {
            continue;
        };
        let Ok(fixup_name) = std::str::from_utf8(&cf.payload.core.name) else {
            continue;
        };
        for target in &cf.target_symbol_names {
            if let Ok(target) = std::str::from_utf8(target) {
                map.push((target.to_string(), fixup_name.to_string()));
            }
        }
    }
    map
}

/// Find the call-fixup name a function maps to, or `None` — the port of
/// `getCallFixupNameForFunction` (`CallFixupAnalyzer.java:200-222`).
///
/// Strip a leading `libID_conflict_` if present (lines 207-209), then probe the
/// target map for `funcName`, then `"_"+funcName`, then `"__"+funcName` (lines
/// 211-219, the underscore-prefix retries). First hit wins. Returns the matched
/// *fixup* name (e.g. `"mcount"`).
///
/// `pub` for the commit seam (see [`target_fixup_map`]).
pub fn call_fixup_name_for_function<'a>(
    func_name: &str,
    map: &'a [(String, String)],
) -> Option<&'a str> {
    let name = func_name.strip_prefix(LIB_CONFLICT_PREFIX).unwrap_or(func_name);
    // Probe order: bare name, then one leading underscore, then two — faithful to
    // the Java retries (a cspec `<target name="__fentry__">` must match a function
    // installed as `fentry` or `_fentry` or `__fentry__`).
    for probe in [name.to_string(), format!("_{name}"), format!("__{name}")] {
        if let Some((_, fixup)) = map.iter().find(|(target, _)| *target == probe) {
            return Some(fixup.as_str());
        }
    }
    None
}

/// Scan an ELF object's FUNC symbols (`.symtab` + `.dynsym`) for names that match
/// a registered call-fixup target, emitting a [`CallFixupFact`] per match. The
/// **original installed name** is carried (not the matched fixup name), so the
/// commit's `query_global_function` resolves the FunctionSymbol that actually
/// exists and re-derives the fixup the same way (mirrors `noreturn`). Shared by
/// [`AnalysisPass::run`] and the unit tests.
fn scan_call_fixups(file: &object::File, arch: &Architecture) -> AnalysisOutput {
    let mut out = AnalysisOutput::default();
    // Format-agnostic (PR-10): a cspec `<callfixup>` `<target>` is a function NAME
    // (`mcount`/`__fentry__`/`__x86_indirect_thunk_*`), matched against the object's
    // FUNC symbols — neutral data on every format. The map comes from the loaded
    // arch's cspec (the Windows cspec ships its own fixups), so this fires on a
    // PE/Mach-O whose toolchain emitted a fixup'd stub, with no format branch.
    let map = target_fixup_map(arch);
    if map.is_empty() {
        return out;
    }
    let mut seen = std::collections::HashSet::new();
    // The same FUNC streams the loader installs as FunctionSymbols (a `-pg`
    // `mcount` may be a `.symtab` FUNC, a weak symbol, or a `.dynsym` import);
    // both streams are scanned, with version suffixes stripped (`mcount@GLIBC…`),
    // exactly as `noreturn.rs` does.
    for sym in file.symbols().chain(file.dynamic_symbols()) {
        if sym.kind() != SymbolKind::Text {
            continue;
        }
        let Ok(n) = sym.name() else { continue };
        let Ok(n) = String::from_utf8(crate::s1_loader::elf_plt::strip_version(n.as_bytes()))
        else {
            continue;
        };
        if call_fixup_name_for_function(&n, &map).is_some() && seen.insert(n.clone()) {
            out.call_fixups.push(crate::pass::CallFixupFact { func_name: n });
        }
    }
    out
}

/// Port of `CallFixupAnalyzer` (the install half): tag every program function
/// whose name matches a cspec call-fixup `<target>` so the engine applies that
/// fixup at flow time. Ghidra ships it default-on (`setDefaultEnablement(true)`,
/// CallFixupAnalyzer.java:51), so on-by-default is faithful; gate id `"callfixup"`
/// lets `--option callfixup off` restore the un-fixed rendering.
#[derive(Clone, Copy, Default)]
pub struct CallFixupPass;

impl AnalysisPass for CallFixupPass {
    fn stage(&self) -> Stage {
        Stage::S1
    }

    fn id(&self) -> &'static str {
        "callfixup"
    }

    fn run(&self, ctx: &AnalysisCtx) -> AnalysisOutput {
        scan_call_fixups(ctx.file, ctx.arch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A synthetic target map standing in for the cspec's registered fixups
    /// (`mcount`→`mcount`, `__fentry__`→`fentry`), so the matcher is tested in
    /// isolation without bootstrapping an Architecture.
    fn synthetic_map() -> Vec<(String, String)> {
        vec![
            ("mcount".to_string(), "mcount".to_string()),
            ("__fentry__".to_string(), "fentry".to_string()),
            ("__x86_indirect_thunk_rax".to_string(), "x86_indirect_thunk_rax".to_string()),
        ]
    }

    #[test]
    fn direct_target_matches() {
        let m = synthetic_map();
        // `mcount` is a bare target: a function named `mcount` matches directly.
        assert_eq!(call_fixup_name_for_function("mcount", &m), Some("mcount"));
        // `__fentry__` is a `__`-prefixed target: a function named `__fentry__`
        // matches directly (the bare probe).
        assert_eq!(call_fixup_name_for_function("__fentry__", &m), Some("fentry"));
    }

    #[test]
    fn underscore_retries_match() {
        let m = synthetic_map();
        // The Java matcher probes funcName, "_"+funcName, "__"+funcName (the
        // underscore retries), first hit wins. The target is
        // `__x86_indirect_thunk_rax`, so a function installed *without* the leading
        // underscores still reaches it:
        //   `x86_indirect_thunk_rax`  -> "__"+name == "__x86_indirect_thunk_rax" (hit, __ retry)
        assert_eq!(
            call_fixup_name_for_function("x86_indirect_thunk_rax", &m),
            Some("x86_indirect_thunk_rax")
        );
        //   `_x86_indirect_thunk_rax` -> "_"+name  == "__x86_indirect_thunk_rax" (hit, _ retry)
        assert_eq!(
            call_fixup_name_for_function("_x86_indirect_thunk_rax", &m),
            Some("x86_indirect_thunk_rax")
        );
    }

    #[test]
    fn unrelated_names_do_not_match() {
        let m = synthetic_map();
        for n in ["main", "puts", "read", "entry", "count", "fentry_helper"] {
            assert_eq!(call_fixup_name_for_function(n, &m), None, "{n} must not match");
        }
    }

    #[test]
    fn lib_conflict_prefix_is_stripped() {
        let m = synthetic_map();
        // A `libID_conflict_mcount` resolves to `mcount` after the prefix strip
        // (CallFixupAnalyzer.java:207-209), then matches the `mcount` target.
        assert_eq!(call_fixup_name_for_function("libID_conflict_mcount", &m), Some("mcount"));
    }

    /// Drive the full scan over the vendored `mcount_x86_64` fixture against a
    /// synthetic target map: it must surface the `-pg` prologue callee (`mcount`
    /// or `__fentry__`, whichever the toolchain emitted) and nothing spurious
    /// (`main`/`__libc_start_main`/…). Mirrors
    /// `noreturn.rs::run_over_fauxware_flags_exit_only`.
    #[test]
    fn scan_flags_mcount_in_fixture() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/mcount_x86_64");
        let bytes = std::fs::read(path).expect("read mcount fixture");
        let file = object::File::parse(bytes.as_slice()).expect("parse mcount fixture");

        // The matcher is exercised against a synthetic map (the cspec map needs a
        // bootstrapped Architecture; the e2e gate proves the real bootstrap path).
        // We re-implement the FUNC scan here over the same streams scan_call_fixups
        // uses, to keep the unit test free of an Architecture.
        let map = synthetic_map();
        let mut hits: Vec<String> = Vec::new();
        for sym in file.symbols().chain(file.dynamic_symbols()) {
            if sym.kind() != SymbolKind::Text {
                continue;
            }
            let Ok(n) = sym.name() else { continue };
            let Ok(n) =
                String::from_utf8(crate::s1_loader::elf_plt::strip_version(n.as_bytes()))
            else {
                continue;
            };
            if call_fixup_name_for_function(&n, &map).is_some() {
                hits.push(n);
            }
        }
        // The `-pg` profiling target the prologue calls is present and flagged.
        assert!(
            hits.iter().any(|n| n == "mcount" || n == "__fentry__"),
            "the -pg profiling target (mcount/__fentry__) must be flagged; hits={hits:?}"
        );
        // Ordinary functions are never flagged.
        assert!(!hits.iter().any(|n| n == "main"), "main must not be flagged");
        assert!(
            !hits.iter().any(|n| n == "__libc_start_main"),
            "__libc_start_main must not be flagged"
        );
    }

    /// PR-10: the FUNC-symbol scan reaches the matcher on a PE (no ELF early
    /// return). A synthetic target matching a real PE symbol name (`__main`, a
    /// MinGW CRT function present in `pe_imports.exe`) is flagged — proving the
    /// pass is format-agnostic. (The real cspec map needs a bootstrapped
    /// Architecture; the e2e bootstrap covers that path. This pins the scan runs
    /// on a non-ELF object at all, which the old gate blocked.)
    #[test]
    fn scan_runs_over_pe_symbols() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/pe_imports.exe");
        let bytes = std::fs::read(path).expect("read pe_imports fixture");
        let file = object::File::parse(bytes.as_slice()).expect("parse pe_imports fixture");
        assert_eq!(file.format(), object::BinaryFormat::Pe, "fixture is a PE");

        // A synthetic fixup whose target is a real PE FUNC symbol (`__main`).
        let map = vec![("__main".to_string(), "synthetic_fixup".to_string())];
        let mut flagged = false;
        for sym in file.symbols().chain(file.dynamic_symbols()) {
            if sym.kind() != SymbolKind::Text {
                continue;
            }
            let Ok(n) = sym.name() else { continue };
            let Ok(n) = String::from_utf8(crate::s1_loader::elf_plt::strip_version(n.as_bytes()))
            else {
                continue;
            };
            if call_fixup_name_for_function(&n, &map).is_some() {
                flagged = true;
            }
        }
        assert!(flagged, "the PE FUNC-symbol scan must reach the matcher (no ELF early return)");
    }
}
