//! Unit tests for the kuna stage registry (`kuna_stages.rs`).
//!
//! Parity targets transcribed from `decompiler/cpp/kuna_stages.cc`:
//! group=39, substage=40, surface=91, settable=46 (28 stage-model knobs + 15
//! kuna analysis-tier gates + 3 loader-tier capabilities
//! `relocobjects`/`i386_pie_plt`/`macho-arm64e`), plus the stage-code helpers,
//! the lookup API, the
//! typed `OptionValues` defaults, and the catalog emitter.

use super::*;

// --- Table counts (the C++ kunaNum* values) ----------------------------------

#[test]
fn group_count_is_39() {
    assert_eq!(kuna_num_groups(), 39);
    assert_eq!(GROUP_TABLE.len(), 39);
}

#[test]
fn substage_count_is_40() {
    assert_eq!(kuna_num_substages(), 40);
    assert_eq!(SUBSTAGE_TABLE.len(), 40);
}

#[test]
fn surface_count_is_98() {
    // +1 for the `option switchguardbound` surface row (angr missing-function-call),
    // +1 for the `option switchsharedcase` surface row (angr shared-case-node b2sum),
    // +1 for the `option switchmultipred` surface row (angr abnormal-switch-case-case3),
    // +1 for the `option unrolledguard` surface row (angr optimized-memcpy),
    // +1 for the `option tailcalljump` surface row (angr tee-O2 tail-jumps),
    // +1 for the `option branchflip` surface row (angr SAILR condition polarity),
    // +1 for the `option noreturn_externmatch` surface row (angr incorrect-duplication-chcon, DIV-13).
    assert_eq!(kuna_num_surfaces(), 98);
    assert_eq!(SURFACE_TABLE.len(), 98);
}

#[test]
fn settable_count_is_66() {
    // 32 stage-model knobs (incl. `foldcallret` + `dedupvardecls` + `loopbreak_recovery`
    // + `gotoreduce`
    // + `switchguardbound`, angr test_decompiling_missing_function_call
    // + `tailcalljump`, angr tee-O2 tail-jumps
    // + `branchflip`, angr SAILR negated-guard branch flip, S8 readability
    // + `regionstructure`, region-based Phoenix/SAILR structurer, Inc 1
    // + `regionlooprefine`, region structurer multi-exit/irreducible loop-successor
    //   refinement, default-off opt-in)
    // + 20 analysis-tier
    // gates: 14 per-run analysis-pass
    // enablement (noreturn_known/libproto/strings/entry_disc/eh_frame_full/
    // funcstart_patterns/arm_markers/mips_gp/mips_isa/dwarf/dwarf_lines/callfixup/addrtable/operand_refs;
    // `eh_frame_full` is the `.eh_frame` LSDA landing-pad discovery sub-feature of the
    // always-on entry_disc pass, GccExceptionAnalyzer, default-off; `funcstart_patterns`
    // is the full Ghidra byte-pattern function-start set, FunctionStartAnalyzer, default-off)
    // + the `formatstring` DecompilerDependent
    // varargs-typing gate + the `listing` Listing/xref disassembly tier gate +
    // the `noreturn_disc` discovered-no-return Listing consumer gate + the
    // `noreturn_propagate` no-return propagation Listing consumer gate + the
    // `fid` FID fingerprint-matcher Listing consumer gate + the
    // `aif` Aggressive Instruction Finder gap-walk Listing consumer gate + the
    // `gopclntab` Go pclntab function-name recovery gate.
    // (mips_isa added with MIPS16 ISA_MODE painting, Increment 21; mips_gp with
    // MIPS $gp recovery; formatstring with half B; listing with the Listing/xref
    // tier, Increment 29; noreturn_disc with the first Listing consumer, Increment 33;
    // gopclntab with Go pclntab name recovery, Increment 34;
    // dwarf_lines with DWARF .debug_line source-line comments, default-off opt-in;
    // dedupvardecls with duplicate-scalar-declaration collapse, DIV-7;
    // loopbreak_recovery with loop-exit-goto break recovery, DIV-10;
    // gotoreduce with the angr SAILR return-tail goto-reduction pass;
    // switchguardbound with guard-bounded GCC PIC jump-table recovery;
    // switchsharedcase with loop-carried-base PIC jump-table recovery
    // (angr test_switch_case_shared_case_nodes_b2sum_digest);
    // regionstructure with the region-based Phoenix/SAILR structurer (Inc 1);
    // noreturn_propagate with angr-style structural no-return propagation;
    // operand_refs with the ScalarOperandAnalyzer scalar-operand reference markup,
    // default-off — Ghidra getDefaultEnablement = !isElf;
    // funcstart_patterns with the FULL Ghidra byte-pattern function-start set
    //   (FunctionStartAnalyzer), default-off opt-in, output-changing;
    // aif with the AggressiveInstructionFinder gap-walk consumer.)
    // + 3 loader-tier capabilities: the `relocobjects` ET_REL relocatable-object
    // loader (DIV-8), the `i386_pie_plt` i386-PIE PLT-stub decode gate
    // (DIV-9, angr test_decompiling_nl_i386_pie), and the `macho-arm64e` Mach-O
    // arm64e Apple-Silicon spec-selection gate (multi-format loader PR-8) —
    // settables that gate the loader rather than a per-function pass.
    // (+1 for `tailcalljump`, the angr tee-O2 tail-jump S2 flow-classification
    // knob, default-off opt-in; +1 for `branchflip`, the angr SAILR negated-guard
    // S8 branch-flip readability knob, default-off opt-in; +1 for `regionstructure`,
    // the region-based Phoenix/SAILR structurer Inc 1 knob, default-off opt-in;
    // +1 for `funcstart_patterns`, the full byte-pattern function-start set,
    // default-off opt-in; +1 for `aif`, the AggressiveInstructionFinder gap-walk
    // Listing consumer, default-off opt-in; +1 for `dwarf_lines`, the DWARF
    // .debug_line source-line mapping, default-off opt-in;
    // +1 for `noreturn_extern`, the undefined-extern name-based no-return S2
    // flow-classification knob, angr test_tail_tail_bytes_ret_dup, default-off opt-in;
    // +1 for `noreturn_externmatch`, the angr incorrect-duplication-chcon S2
    // name-matched-extern no-return knob, DIV-13 default-on;
    // +1 for `crossjumprevert`, the angr SAILR CrossJumpReverter cross-jump-tail
    // duplication knob, default-off opt-in;
    // +1 for `switchsharedcase`, the loop-carried-base PIC jump-table recovery S2
    // knob (angr test_switch_case_shared_case_nodes_b2sum_digest), default-off opt-in;
    // +1 for `switchmultipred`, the multi-predecessor unrolled-guard jump-table
    // recovery S2 knob (angr test_decompiling_abnormal_switch_case_case3),
    // default-off opt-in;
    // +1 for `regionlooprefine`, the region structurer multi-exit/irreducible
    // loop-successor refinement knob, default-off opt-in;
    // +1 for `ifelseflatten`, the angr IfElseFlattener S8 terminating-if else-drop
    // knob, default-off opt-in;
    // +1 for `fid`, the FID fingerprint-matcher Listing consumer that re-identifies
    // a stripped function by full-hash fingerprint (FUN_*/sub_* -> kuna_crc32),
    // default-off opt-in, real-ELF path only;
    // +1 for `taildup`, the angr SAILR ReturnDuplicatorLow return-tail-WITH-call
    // duplication knob (the gap between gotoreduce and crossjumprevert), default-off
    // opt-in;
    // +1 for `unrolledguard`, the interleaved unrolled-guard jump-table
    // partial-flow recovery S2 knob (angr test_decompiling_optimized_memcpy MSVC
    // memcpy): the partial-clone collectEdges skips a recovered sibling table's
    // undecoded case-target edge instead of throwing, default-off opt-in;
    // +1 for `dedupitetail`, the angr structurer ITE region-dedup knob — the INVERSE
    // of the duplication passes: it merges a duplicated if/else leaf prefix/suffix
    // (one copy instead of two), default-off opt-in.
    // +1 for `rtti`, the MSVC RTTI / vftable class-name recovery pass (the kuna
    // analog of Ghidra's RttiAnalyzer): on a Windows PE, parse the
    // CompleteObjectLocator -> RTTI3/2/1 -> RTTI0 graph in `.rdata`/`.data`,
    // demangle each `.?A...@@` class name, and emit `<Class>::vftable` /
    // `<Class>::RTTI_*` labels (Box/Shape) — PE-only, default-off opt-in,
    // output-changing, so every ELF/XML parity gate is byte-identical.)
    assert_eq!(kuna_num_settables(), 66);
    assert_eq!(SETTABLE_TABLE.len(), 66);
}

// --- Stage helpers (kunaStageCode/Name/Artifact/InBandB/FromCode) ------------

#[test]
fn stage_codes() {
    assert_eq!(KunaStage::P0.code(), "P0");
    assert_eq!(KunaStage::S1.code(), "S1");
    assert_eq!(KunaStage::S9.code(), "S9");
    // C++ STAGE_CODES[10] for infra.
    assert_eq!(KunaStage::Infra.code(), "--");
}

#[test]
fn stage_names_and_artifacts() {
    assert_eq!(KunaStage::P0.name(), "Knowledge & Configuration Plane");
    assert_eq!(KunaStage::S9.name(), "Surface Rendering & Refinement");
    assert_eq!(KunaStage::Infra.name(), "Infrastructure / orchestration");
    assert_eq!(
        KunaStage::S7.artifact(),
        "region tree (sblocks - physically distinct from the CFG)"
    );
    assert_eq!(
        KunaStage::Infra.artifact(),
        "(none - schedule/termination policy only)"
    );
}

#[test]
fn band_b_membership() {
    // C++ kunaStageInBandB: S3..S6 only.
    assert!(!KunaStage::P0.in_band_b());
    assert!(!KunaStage::S1.in_band_b());
    assert!(!KunaStage::S2.in_band_b());
    assert!(KunaStage::S3.in_band_b());
    assert!(KunaStage::S4.in_band_b());
    assert!(KunaStage::S5.in_band_b());
    assert!(KunaStage::S6.in_band_b());
    assert!(!KunaStage::S7.in_band_b());
    assert!(!KunaStage::S8.in_band_b());
    assert!(!KunaStage::S9.in_band_b());
    assert!(!KunaStage::Infra.in_band_b());
}

#[test]
fn stage_from_code() {
    // C++ kunaStageFromCode: P0/p0, S1..S9/s1..s9; everything else fails.
    assert_eq!(KunaStage::from_code("P0"), Some(KunaStage::P0));
    assert_eq!(KunaStage::from_code("p0"), Some(KunaStage::P0));
    assert_eq!(KunaStage::from_code("S3"), Some(KunaStage::S3));
    assert_eq!(KunaStage::from_code("s3"), Some(KunaStage::S3));
    assert_eq!(KunaStage::from_code("S9"), Some(KunaStage::S9));
    // Failures.
    assert_eq!(KunaStage::from_code("S0"), None);
    assert_eq!(KunaStage::from_code("P1"), None);
    assert_eq!(KunaStage::from_code("X3"), None);
    assert_eq!(KunaStage::from_code("S"), None);
    assert_eq!(KunaStage::from_code("S33"), None);
    assert_eq!(KunaStage::from_code(""), None);
    assert_eq!(KunaStage::from_code("--"), None);
}

#[test]
fn stage_index_matches_cpp_enum() {
    // C++ enum: kstage_infra=-1, kstage_p0=0, kstage_s1=1 .. kstage_s9=9.
    assert_eq!(KunaStage::Infra.index(), -1);
    assert_eq!(KunaStage::P0.index(), 0);
    assert_eq!(KunaStage::S1.index(), 1);
    assert_eq!(KunaStage::S9.index(), 9);
}

// --- Lookup API (kunaLookup*) ------------------------------------------------

#[test]
fn lookup_group_parity() {
    // Every group in the table is findable by name and round-trips by index.
    for i in 0..kuna_num_groups() {
        let e = kuna_group_by_index(i);
        let found = lookup_group(e.group).expect("group findable by name");
        assert_eq!(found.group, e.group);
        assert_eq!(found.stage, e.stage);
    }
    // A couple of known entries (transcribed from groupTable).
    assert_eq!(lookup_group("base").unwrap().stage, KunaStage::Infra);
    assert_eq!(lookup_group("analysis").unwrap().stage, KunaStage::S3);
    assert_eq!(lookup_group("casts").unwrap().stage, KunaStage::S9);
    assert!(lookup_group("nonexistent").is_none());
}

#[test]
fn lookup_substage_parity() {
    for i in 0..kuna_num_substages() {
        let e = kuna_substage_by_index(i);
        let found = lookup_substage(e.name).expect("substage findable");
        assert_eq!(found.name, e.name);
        assert_eq!(found.stage, e.stage);
        assert_eq!(found.rewind, e.rewind);
    }
    // Known rewind targets (stage-model.md section 12).
    let typ = lookup_substage("type-propagation").unwrap();
    assert_eq!(typ.stage, KunaStage::S5);
    assert_eq!(typ.rewind, KunaStage::S5);
    let force = lookup_substage("edge-virtualization").unwrap();
    assert_eq!(force.stage, KunaStage::S7);
    assert_eq!(force.rewind, KunaStage::S7);
    // explicit-implied: rewinds to S9 (the only cross-stage rewind in the table).
    let ei = lookup_substage("explicit-implied").unwrap();
    assert_eq!(ei.stage, KunaStage::S6);
    assert_eq!(ei.rewind, KunaStage::S9);
    assert!(lookup_substage("not-a-substage").is_none());
}

#[test]
fn lookup_surface_parity() {
    for i in 0..kuna_num_surfaces() {
        let e = kuna_surface_by_index(i);
        let found = lookup_surface(e.surface).expect("surface findable by exact string");
        assert_eq!(found.surface, e.surface);
        assert_eq!(found.stage, e.stage);
    }
    assert_eq!(
        lookup_surface("force goto").unwrap().stage,
        KunaStage::S7
    );
    assert_eq!(
        lookup_surface("option compareform").unwrap().substage,
        "comparison-canonicalization"
    );
    assert!(lookup_surface("nope").is_none());
}

#[test]
fn lookup_settable_parity() {
    for i in 0..kuna_num_settables() {
        let e = kuna_settable_by_index(i);
        let found = lookup_settable(e.option).expect("settable findable by option");
        assert_eq!(found.option, e.option);
        assert_eq!(found.shipped, e.shipped);
    }
    assert!(lookup_settable("compareform").is_some());
    assert!(lookup_settable("namestyle").is_some());
    assert!(lookup_settable("not-an-option").is_none());
}

// --- Typed OptionValues defaults == settableTable shipped values -------------

#[test]
fn option_values_defaults_match_shipped() {
    let ov = OptionValues::default();
    for i in 0..kuna_num_settables() {
        let st = kuna_settable_by_index(i);
        let live = ov
            .get(st.option)
            .unwrap_or_else(|| panic!("OptionValues field missing for {}", st.option));
        assert_eq!(
            live, st.shipped,
            "default for {} must equal shipped value",
            st.option
        );
    }
}

#[test]
fn option_values_set_validates_against_values() {
    let mut ov = OptionValues::default();
    // compareform default is "original"; "canonical" is allowed.
    assert_eq!(ov.get("compareform"), Some("original"));
    assert!(ov.set("compareform", "canonical"));
    assert_eq!(ov.get("compareform"), Some("canonical"));
    // An out-of-vocabulary value is rejected and leaves the field unchanged.
    assert!(!ov.set("compareform", "bogus"));
    assert_eq!(ov.get("compareform"), Some("canonical"));
    // Unknown option.
    assert!(!ov.set("not-an-option", "on"));
    assert_eq!(ov.get("not-an-option"), None);
}

#[test]
fn option_values_live_value_present_for_28_suppressed_for_36() {
    let ov = OptionValues::default();
    // 28 options have a codegen live reader (realtypes + dedupvardecls join the
    // field-backed group; switchguardbound is field-backed via switch_guard_bound;
    // switchsharedcase is field-backed via switch_shared_case;
    // switchmultipred is field-backed via switch_multi_pred;
    // unrolledguard is field-backed via unrolled_guard;
    // +1 for `tailcalljump`, whose `live_field` is `tail_call_jumps`; +1 for
    // `noreturn_extern`, whose `live_field` is `noreturn_extern_calls`, opt-in;
    // +1 for `noreturn_externmatch`, field-backed via `noreturn_extern_match`, DIV-13); the
    // live_value returns the current value for them and None for
    // loweredswitch/stackguard/namestyle/foldcallret/relocobjects PLUS the
    // 18 analysis/loader-tier gates (which have no `live_field` — their live state
    // is read console-side via the hand-written `kuna_live_value` / an env gate,
    // not the codegen `live_value`; +1 for `funcstart_patterns`, the full
    // byte-pattern function-start pass). `relocobjects` (DIV-8) gates the loader,
    // not a printer/engine flag, so it too has no codegen live reader.
    const PASS_GATES: &[&str] = &[
        "noreturn_known",
        "libproto",
        "strings",
        "entry_disc",
        // (kuna) `.eh_frame` LSDA landing-pad discovery sub-feature of entry_disc
        // (GccExceptionAnalyzer), default-off; analysis-tier, no codegen live reader.
        "eh_frame_full",
        // (kuna) The full byte-pattern function-start pass — an analysis-pass gate
        // with no codegen live reader (read console-side via kuna_live_value), same
        // as the gates around it. Default-off.
        "funcstart_patterns",
        "arm_markers",
        "mips_gp",
        "mips_isa",
        "dwarf",
        "dwarf_lines",
        "callfixup",
        "addrtable",
        "operand_refs",
        "formatstring",
        "listing",
        "noreturn_disc",
        "noreturn_propagate",
        // (kuna) FID fingerprint-matcher Listing consumer — an analysis-pass gate
        // whose DB source is a load-time env var (`kuna_fid_db`); no codegen
        // live_value reader (read console-side via kuna_live_value), like the gates
        // around it. Default-off.
        "fid",
        // (kuna) MSVC RTTI / vftable class-name recovery — a PE-only analysis-pass
        // gate (no `live_field`); its live state is read console-side via
        // kuna_live_value, like the analysis-pass gates around it. Default-off.
        "rtti",
        "aif",
        "gopclntab",
        // (kuna) loader-tier gate, no codegen live reader (read console-side via
        // kuna_live_value), same as the analysis-pass gates above.
        "i386_pie_plt",
        // (PR-8) Mach-O arm64e spec selection: a load-time (pre-`option`) gate read
        // from the `KUNA_MACHO_ARM64E` env var, so it too has no codegen live_value.
        "macho-arm64e",
    ];
    let mut with_live = 0;
    for i in 0..kuna_num_settables() {
        let st = kuna_settable_by_index(i);
        match ov.live_value(st.option) {
            Some(v) => {
                with_live += 1;
                assert_eq!(v, st.shipped, "live default == shipped for {}", st.option);
            }
            None => {
                assert!(
                    matches!(
                        st.option,
                        "loweredswitch"
                            | "regionstructure"
                            | "regionlooprefine"
                            | "stackguard"
                            | "branchflip"
                            | "namestyle"
                            | "foldcallret"
                            | "gotoreduce"
                            | "ifelseflatten"
                            | "crossjumprevert"
                            | "taildup"
                            | "dedupitetail"
                            | "loopbreak_recovery"
                            | "relocobjects"
                    ) || PASS_GATES.contains(&st.option),
                    "unexpected option with no live reader: {}",
                    st.option
                );
            }
        }
    }
    assert_eq!(with_live, 28);
}

#[test]
fn live_from_arch_matches_cpp_ternaries() {
    // compareform = present_lessequal ? "original" : "canonical"
    let on = |_f: &str| Some(true);
    let off = |_f: &str| Some(false);
    assert_eq!(
        OptionValues::live_from_arch("compareform", on),
        Some("original")
    );
    assert_eq!(
        OptionValues::live_from_arch("compareform", off),
        Some("canonical")
    );
    // returnpair = return_single ? "single" : "pair"
    assert_eq!(OptionValues::live_from_arch("returnpair", on), Some("single"));
    assert_eq!(OptionValues::live_from_arch("returnpair", off), Some("pair"));
    // a plain on/off option
    assert_eq!(OptionValues::live_from_arch("thumbfuncptr", on), Some("on"));
    assert_eq!(OptionValues::live_from_arch("thumbfuncptr", off), Some("off"));
    // no live reader -> None even with a value-producing closure
    assert_eq!(OptionValues::live_from_arch("namestyle", on), None);
    assert_eq!(OptionValues::live_from_arch("stackguard", on), None);
    assert_eq!(OptionValues::live_from_arch("loweredswitch", on), None);
    // unknown flag -> None propagates
    assert_eq!(
        OptionValues::live_from_arch("compareform", |_f: &str| None),
        None
    );
}

// --- Catalog JSON emitter byte-shape (full byte-compat is in the integration
//     test catalog_bytecompat.rs against the C++ binary's captured output) ----

#[test]
fn emit_settable_json_first_row_shape() {
    // The first settable is `compareform`. Emit it with no live value (no
    // program loaded form) and check the leading bytes match kunaEmitSettableJson.
    let st = lookup_settable("compareform").unwrap();
    let mut out = String::new();
    emit_settable_json(&mut out, st, None);
    assert!(out.starts_with("  {\"option\": \"compareform\", \"values\": [\"canonical\", \"original\"], \"default\": \"original\", \"destructive_as_default\": false, \"stage\": \"S3\""));
    // No `current` field when live is None.
    assert!(!out.contains("\"current\""));
    // ... and the tail order (issue ... change_kind).
    assert!(out.contains("\"strength\": \"HARD\", \"rewind\": \"S3\", \"issue\": \"GH-558\""));
    assert!(out.ends_with("\"change_kind\": \"presentation-default\"}"));
}

#[test]
fn emit_settable_json_includes_current_when_live() {
    let st = lookup_settable("compareform").unwrap();
    let mut out = String::new();
    emit_settable_json(&mut out, st, Some("canonical"));
    // C++ inserts "current" right after "default".
    assert!(out.contains("\"default\": \"original\", \"current\": \"canonical\", \"destructive_as_default\""));
}

#[test]
fn emit_catalog_json_static_form_brackets_and_commas() {
    let json = emit_catalog_json(|_| None);
    assert!(json.starts_with("[\n  {\"option\": \"compareform\""));
    assert!(json.ends_with("}\n]\n"));
    // 66 rows: 65 trailing commas (the last, macho-arm64e, has none;
    // switchguardbound's, switchsharedcase's, switchmultipred's, unrolledguard's,
    // tailcalljump's, noreturn_extern's, and noreturn_externmatch's S2 rows,
    // branchflip's, regionstructure's, regionlooprefine's, ifelseflatten's,
    // crossjumprevert's, taildup's, and dedupitetail's S8 rows, eh_frame_full's S1 row,
    // operand_refs's S1 row, funcstart_patterns's S1 row, aif's S1 row, fid's S1
    // row, rtti's S1 row, and dwarf_lines' S1 row sit mid-table, so they do not move
    // the tail).
    assert_eq!(json.matches("},\n").count(), 65);
}

#[test]
fn emit_catalog_json_one_unknown_is_none() {
    assert!(emit_catalog_json_one("bogus", None).is_none());
    let one = emit_catalog_json_one("namestyle", None).unwrap();
    assert!(one.starts_with("  {\"option\": \"namestyle\""));
    assert!(one.ends_with("}\n"));
}

#[test]
fn json_string_escaping() {
    // Direct check of the escape rules (quote, backslash, newline, control).
    let mut out = String::new();
    json_string(&mut out, "a\"b\\c\nd\te");
    // tab (0x09) is a control char < 0x20 and is NOT '\n' -> collapses to space.
    assert_eq!(out, "\"a\\\"b\\\\c\\nd e\"");
}
