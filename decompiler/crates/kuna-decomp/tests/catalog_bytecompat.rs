//! Catalog byte-compatibility gate (ADR 0006, item w4-kuna-p0-pack).
//!
//! The Rust catalog JSON emitter (`kuna_phases::emit_catalog_json`) must
//! reproduce `kuna_console.cc kunaEmitSettableJson`'s output **byte for byte**:
//! `tests/stages/kuna-catalog.xml` (a C++ datatest) string-matches the raw
//! emitter output, and `python -m kuna.catalog` parses it, so any drift in key
//! order, spacing, separators, pipe-splitting, the conditional `current` field,
//! or escaping would silently break the LLM control surface.
//!
//! ## What is compared
//!
//! The fixture `tests/fixtures/stage_catalog.json` is the EXACT bytes the C++
//! `decomp_dbg` binary emits for `stage catalog` **with no program loaded**
//! (so `kunaLiveValue` returns `""` for every option and no `current` field is
//! present). We compare against the no-program form deliberately: the `current`
//! field is architecture-dependent (it reads live `Architecture` flags), so it
//! is not a stable cross-build byte contract — but the surrounding bytes (key
//! order, `[\n  {...},\n  {...}\n]\n` framing, two-space indent, `, ` and `: `
//! separators, the pipe-split `values` array, and the JSON escaping) are. The
//! Rust emitter is driven with a `live = |_| None` closure to produce the same
//! no-`current` form.
//!
//! ## Regenerating the fixture
//!
//! The original fixture was captured from the (now-removed) C++ binary. Since the
//! C++ tree is gone and kuna is Rust-only, the Rust `emit_catalog_json` emitter is
//! the authoritative source; kuna-native settables (e.g. `realtypes`, DIV-6) have
//! no C++ origin. Recapture from the Rust `decomp_dbg` with the `openfile write`
//! trick (the same one `kuna decompile` uses, so console prompts never pollute the
//! bytes), with no program loaded so `kunaLiveValue` returns `""` (no `current`):
//!
//! ```sh
//! printf 'openfile write /tmp/cap.json\nstage catalog\nclosefile\nquit\n' \
//!   | decompiler/target/release/decomp_dbg -s "$PWD/specs" >/dev/null 2>&1
//! cp /tmp/cap.json decompiler/crates/kuna-decomp/tests/fixtures/stage_catalog.json
//! ```
//!
//! Regenerate it whenever `phases.toml` gains/loses a settable or a settable's
//! catalog text changes.

use kuna_decomp::kuna_phases::{emit_catalog_json, emit_catalog_json_one, lookup_settable};

/// The captured C++ `stage catalog` output (no program loaded -> no `current`).
const FIXTURE: &str = include_str!("fixtures/stage_catalog.json");

#[test]
fn full_catalog_matches_cpp_byte_for_byte() {
    // No live values -> the static, no-`current` form the fixture captures.
    let rust = emit_catalog_json(|_| None);
    if rust != FIXTURE {
        // Surface the first divergence for a readable failure.
        let rl: Vec<&str> = rust.lines().collect();
        let fl: Vec<&str> = FIXTURE.lines().collect();
        for (i, (r, f)) in rl.iter().zip(fl.iter()).enumerate() {
            assert_eq!(r, f, "first byte divergence at line {i}");
        }
        assert_eq!(
            rl.len(),
            fl.len(),
            "line count differs (rust {}, cpp {})",
            rl.len(),
            fl.len()
        );
        assert_eq!(rust, FIXTURE, "catalog bytes differ");
    }
    assert_eq!(rust, FIXTURE);
}

#[test]
fn fixture_has_no_current_field() {
    // Documents the comparison contract: the captured form has no live values.
    assert!(
        !FIXTURE.contains("\"current\""),
        "fixture must be the no-program (no `current`) form"
    );
}

#[test]
fn fixture_has_all_68_settables() {
    // One `"option":` per settable row: 38 stage-model knobs (incl. the `iteregion`
    // angr ITERegionConverter S8 assignment-diamond -> `?:` ternary gate, default-off,
    // a RUNTIME CHOICE, and the `foldcallret`
    // call-return variable-folding gate, the `dedupvardecls` duplicate-scalar-
    // declaration collapse gate, DIV-7, the `loopbreak_recovery` loop-exit-goto
    // break recovery gate, DIV-10, the `gotoreduce` angr SAILR return-tail
    // goto-reduction gate, the `crossjumprevert` angr SAILR CrossJumpReverter
    // cross-jump-tail duplication gate, the `switchguardbound` guard-bounded GCC PIC
    // jump-table recovery gate, angr test_decompiling_missing_function_call,
    // the `switchsharedcase` loop-carried-base PIC jump-table recovery gate,
    // angr test_switch_case_shared_case_nodes_b2sum_digest,
    // the `switchmultipred` multi-predecessor unrolled-guard jump-table recovery
    // gate, angr test_decompiling_abnormal_switch_case_case3,
    // the `tailcalljump` angr -O2 tail-jump S2 flow-classification gate,
    // the `branchflip` angr SAILR negated-guard S8 branch-flip gate,
    // the `regionstructure` region-based Phoenix/SAILR structurer, Inc 1,
    // the `regionlooprefine` region structurer multi-exit/irreducible
    // loop-successor refinement gate, default-off opt-in,
    // the `ifelseflatten` angr IfElseFlattener S8 terminating-if else-drop gate,
    // default-off opt-in,
    // the `noreturn_extern` undefined-extern name-based no-return S2
    // flow-classification gate, angr test_tail_tail_bytes_ret_dup, default-off opt-in,
    // and the `noreturn_externmatch` name-matched-extern no-return flow-seam gate,
    // angr incorrect-duplication-chcon, DIV-13)
    // + 20 analysis-tier gates (14 per-run
    // analysis-pass enablement — including the `mips_gp` $gp recovery gate, the
    // `mips_isa` MIPS16 ISA_MODE painting gate, the `eh_frame_full` `.eh_frame`
    // LSDA landing-pad discovery gate (GccExceptionAnalyzer, default-off), the
    // `operand_refs` scalar-operand reference-markup gate (default-off, !isElf), the
    // `funcstart_patterns` full byte-pattern function-start gate (Ghidra
    // FunctionStartAnalyzer, default-off), and the `dwarf_lines` DWARF .debug_line
    // source-line comment gate (default-off) — plus the `formatstring`
    // DecompilerDependent varargs-typing gate, the `listing` Listing/xref
    // disassembly tier gate, the `noreturn_disc` discovered-no-return Listing
    // consumer gate, the `noreturn_propagate` no-return propagation Listing consumer
    // gate, the `fid` FID fingerprint-matcher Listing consumer gate (re-identify a
    // stripped function by full-hash fingerprint, FUN_*/sub_* -> kuna_crc32,
    // default-off), the `aif` Aggressive Instruction Finder gap-walk Listing consumer
    // gate, and the `gopclntab` Go pclntab function-name recovery gate)
    // + 3 loader-tier capabilities (the `relocobjects` ET_REL relocatable-object
    // loader, DIV-8; the `i386_pie_plt` i386-PIE PLT-stub decode gate, DIV-9; and
    // the `macho-arm64e` Mach-O arm64e Apple-Silicon spec-selection gate, PR-8).
    // (+1 for the `taildup` angr SAILR ReturnDuplicatorLow return-tail-WITH-call
    // duplication gate — the gap between gotoreduce and crossjumprevert;
    // +1 for the `unrolledguard` angr optimized-memcpy interleaved unrolled-guard
    // jump-table partial-flow recovery gate;
    // +1 for the `dedupitetail` angr structurer ITE region-dedup gate — the INVERSE
    // of the duplication passes, merging a duplicated if/else leaf prefix/suffix into
    // one copy.
    // +1 for the `rtti` MSVC RTTI / vftable class-name recovery gate (the kuna analog
    // of Ghidra's RttiAnalyzer): on a Windows PE, parse the CompleteObjectLocator ->
    // RTTI3/2/1 -> RTTI0 graph in `.rdata`/`.data`, demangle each `.?A...@@` class
    // name, and emit `<Class>::vftable` / `<Class>::RTTI_*` labels (Box/Shape) —
    // PE-only, default-off, output-changing, so every ELF/XML parity gate is
    // byte-identical.
    // +1 for the `objc` Mach-O Objective-C metadata recovery gate (rename an IMP
    // function `-[Class sel]`/`+[Class sel]` from the `__objc_*` metadata,
    // ObjcTypeMetadataAnalyzer; Mach-O-only, default-off).
    // +1 for the `pdb` PE PDB metadata recovery gate (rename a stripped FUN_*/sub_*
    // function to its real name from the external `.pdb`, PdbUniversalAnalyzer:
    // CodeView fingerprint -> `kuna_pdb_path` locate -> guid/age fingerprint gate ->
    // S_PUB32/S_GPROC32 walk; PE-only, default-off, inert without a matching `.pdb`).)
    // +1 for the `regionedgeorder` region structurer last-resort edge-virtualization
    // ORDERING gate (SAILR P2: H2 post-dominator + dominance-tiered
    // crossing/secondary/other bucketing), default-off opt-in — only reorders which
    // goto is chosen when virtualizing, so OFF is byte-identical.
    // +1 for the `returndup` angr SAILR gotoless `ReturnDuplicatorHigh` gate (decbench F4).
    // +1 for the `noreturn_error` gate — conclude error(nonzero,...) wrappers no-return
    // (decbench F2, default-on but Listing-gated so byte-identical in the datatest path).
    // +1 for the `iteregion` angr ITERegionConverter assignment-diamond -> `?:` ternary
    // knob (decbench F5, default-off opt-in runtime choice).
    // +1 for the `earlyreturn` angr SAILR `ReturnDuplicatorHigh` per-edge const-guard
    // early-return hoisting gate (the mixed-diamond narrowing returndup cannot reach).
    // +1 for the `switchreturn` continuation of earlyreturn to the WIDE multi-way
    // switch-phi return (per-case `return K` past earlyreturn's 16-in-edge cap).
    assert_eq!(FIXTURE.matches("\"option\": ").count(), 75);
}

#[test]
fn single_settable_rows_match_their_slice_of_the_catalog() {
    // `stage catalog <option>` (one-argument form) must emit exactly the same
    // bytes as that option's row in the full catalog, minus the leading "  "
    // is preserved and minus the trailing comma (the full form joins with ',').
    for i in 0..kuna_decomp::kuna_phases::kuna_num_settables() {
        let st = kuna_decomp::kuna_phases::kuna_settable_by_index(i);
        let one = emit_catalog_json_one(st.option, None).expect("known option");
        // The one-row form is the bare object + a single newline.
        assert!(one.starts_with("  {\"option\": "));
        assert!(one.ends_with("}\n"));
        // It must appear verbatim inside the full catalog (the full form differs
        // only by an optional trailing comma before the newline).
        let body = one.trim_end_matches('\n');
        assert!(
            FIXTURE.contains(body),
            "one-row form for `{}` not found verbatim in the full catalog",
            st.option
        );
    }
}

#[test]
fn unknown_single_settable_is_none() {
    // C++ throws IfaceExecutionError for an unknown option; the port returns None.
    assert!(emit_catalog_json_one("not-a-real-option", None).is_none());
    assert!(lookup_settable("not-a-real-option").is_none());
}
