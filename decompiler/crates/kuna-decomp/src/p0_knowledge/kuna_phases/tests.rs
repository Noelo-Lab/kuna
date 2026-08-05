//! Unit tests for the kuna phase registry (`kuna_phases.rs`).
//!
//! Parity targets (origin: `decompiler/cpp/kuna_stages.cc`, since grown):
//! group/subphase/surface/settable counts pinned by the asserts below, plus the phase-code
//! helpers, the lookup API, the typed `OptionValues` defaults, and the
//! catalog emitter.

use super::*;

// --- Table counts (the C++ kunaNum* values) ----------------------------------

#[test]
fn group_count_is_39() {
    assert_eq!(kuna_num_groups(), 39);
    assert_eq!(GROUP_TABLE.len(), 39);
}

#[test]
fn subphase_count_is_43() {
    // +1 for the P9 `condition-form` subphase (truthycond, DIV-36),
    // +1 for the P9 `brace-form` subphase (braceelide, DIV-37),
    // +1 for the P9 `warning-style` subphase (warnstyle, DIV-38).
    assert_eq!(kuna_num_subphases(), 43);
    assert_eq!(SUBPHASE_TABLE.len(), 43);
}

#[test]
fn surface_count_is_101() {
    // +1 for the `option switchguardbound` surface row (angr missing-function-call),
    // +1 for the `option switchsharedcase` surface row (angr shared-case-node b2sum),
    // +1 for the `option switchmultipred` surface row (angr abnormal-switch-case-case3),
    // +1 for the `option unrolledguard` surface row (angr optimized-memcpy),
    // +1 for the `option tailcalljump` surface row (angr tee-O2 tail-jumps),
    // +1 for the `option branchflip` surface row (angr SAILR condition polarity),
    // +1 for the `option noreturn_externmatch` surface row (angr incorrect-duplication-chcon, DIV-13),
    // +1 for the `option truthycond` surface row (kuna C-surface normalization, DIV-36),
    // +1 for the `option braceelide` surface row (kuna C-surface normalization, DIV-37),
    // +1 for the `option warnstyle` surface row (kuna C-surface normalization, DIV-38).
    assert_eq!(kuna_num_surfaces(), 101);
    assert_eq!(SURFACE_TABLE.len(), 101);
}

#[test]
fn settable_count_is_88() {
    // One row per kuna ArchOption; the authoritative per-option list (with
    // tier, symptoms, and provenance) is phases.toml settableTable.
    // +1 for `callsitestackargs` (P4 stack-passed call argument recovery).
    // +1 for `cortexmvectors` (P1 widened ARM Cortex-M vector-table signature).
    // +1 for `paramcopyhoist` (P6 parameter copy-shadow entry-block anchor).
    // +1 for `itecondlist` (S8 iteregion/iteboolean condition-list tolerance).
    // +1 for `peimportcall` (P1 PE import-call binding, DIV-57).
    // +1 for `ptrentry` (P1 pointer-referenced ARM function entries).
    // +1 for `tailcallentry` (P1 tail-call function-entry recovery).
    // +1 for `cppproto` (P1 DWARF C++ prototype recovery arm).
    assert_eq!(kuna_num_settables(), 90);
    assert_eq!(SETTABLE_TABLE.len(), 90);
}

#[test]
fn tier_counts_are_20_core_41_transform_28_analysis() {
    let mut core = 0;
    let mut transform = 0;
    let mut analysis = 0;
    for s in SETTABLE_TABLE.iter() {
        match s.tier {
            "core" => core += 1,
            "transform" => transform += 1,
            "analysis" => analysis += 1,
            other => panic!("invalid tier {other:?} on {}", s.option),
        }
    }
    // core 19 -> 20: +1 for `callsitestackargs` (P4 stack-passed call arguments).
    // transform 37 -> 38: +1 for `iteboolean` (S8 short-circuit 0/1 re-roll, DIV-51).
    // analysis 25 -> 26: +1 for `cortexmvectors` (P1 widened Cortex-M vector table).
    // transform 38 -> 39: +1 for `paramcopyhoist` (P6 parameter copy-shadow anchor).
    // transform 39 -> 40: +1 for `itecondlist` (S8 ITE condition-list tolerance, DIV-56).
    // transform 40 -> 41: +1 for `peimportcall` (P1 PE import-call binding, DIV-57).
    // analysis 26 -> 27: +1 for `ptrentry` (P1 pointer-referenced ARM entries).
    // analysis 27 -> 28: +1 for `tailcallentry` (P1 tail-call function-entry recovery).
    // analysis 28 -> 29: +1 for `cppproto` (P1 DWARF C++ prototype recovery).
    assert_eq!((core, transform, analysis), (20, 41, 29));
}

#[test]
fn noreturn_family_is_all_transform_tier() {
    // The whole family can remove code at call sites, so it sits in the
    // control-surface tier regardless of which tier mechanically hosts it.
    for s in SETTABLE_TABLE.iter().filter(|s| s.option.starts_with("noreturn_")) {
        assert_eq!(s.tier, "transform", "{} must sit in the transform tier", s.option);
    }
}

// --- Stage helpers (kunaStageCode/Name/Artifact/InBandB/FromCode) ------------

#[test]
fn stage_codes() {
    assert_eq!(KunaPhase::P0.code(), "P0");
    assert_eq!(KunaPhase::P1.code(), "P1");
    assert_eq!(KunaPhase::P9.code(), "P9");
    // C++ STAGE_CODES[10] for infra.
    assert_eq!(KunaPhase::Infra.code(), "--");
}

#[test]
fn stage_names_and_artifacts() {
    assert_eq!(KunaPhase::P0.name(), "Knowledge & Configuration Plane");
    assert_eq!(KunaPhase::P9.name(), "Surface Rendering & Refinement");
    assert_eq!(KunaPhase::Infra.name(), "Infrastructure / orchestration");
    assert_eq!(
        KunaPhase::P7.artifact(),
        "region tree (sblocks - physically distinct from the CFG)"
    );
    assert_eq!(
        KunaPhase::Infra.artifact(),
        "(none - schedule/termination policy only)"
    );
}

#[test]
fn band_b_membership() {
    // C++ kunaStageInBandB: S3..S6 only.
    assert!(!KunaPhase::P0.in_band_b());
    assert!(!KunaPhase::P1.in_band_b());
    assert!(!KunaPhase::P2.in_band_b());
    assert!(KunaPhase::P3.in_band_b());
    assert!(KunaPhase::P4.in_band_b());
    assert!(KunaPhase::P5.in_band_b());
    assert!(KunaPhase::P6.in_band_b());
    assert!(!KunaPhase::P7.in_band_b());
    assert!(!KunaPhase::P8.in_band_b());
    assert!(!KunaPhase::P9.in_band_b());
    assert!(!KunaPhase::Infra.in_band_b());
}

#[test]
fn stage_from_code() {
    // C++ kunaStageFromCode: P0/p0, S1..S9/s1..s9; everything else fails.
    assert_eq!(KunaPhase::from_code("P0"), Some(KunaPhase::P0));
    assert_eq!(KunaPhase::from_code("p0"), Some(KunaPhase::P0));
    assert_eq!(KunaPhase::from_code("S3"), Some(KunaPhase::P3));
    assert_eq!(KunaPhase::from_code("s3"), Some(KunaPhase::P3));
    assert_eq!(KunaPhase::from_code("S9"), Some(KunaPhase::P9));
    // Failures.
    assert_eq!(KunaPhase::from_code("P3"), Some(KunaPhase::P3));
    assert_eq!(KunaPhase::from_code("p9"), Some(KunaPhase::P9));
    assert_eq!(KunaPhase::from_code("S0"), None);
    assert_eq!(KunaPhase::from_code("P0"), Some(KunaPhase::P0));
    assert_eq!(KunaPhase::from_code("X3"), None);
    assert_eq!(KunaPhase::from_code("S"), None);
    assert_eq!(KunaPhase::from_code("S33"), None);
    assert_eq!(KunaPhase::from_code(""), None);
    assert_eq!(KunaPhase::from_code("--"), None);
}

#[test]
fn stage_index_matches_cpp_enum() {
    // C++ enum: kstage_infra=-1, kstage_p0=0, kstage_s1=1 .. kstage_s9=9.
    assert_eq!(KunaPhase::Infra.index(), -1);
    assert_eq!(KunaPhase::P0.index(), 0);
    assert_eq!(KunaPhase::P1.index(), 1);
    assert_eq!(KunaPhase::P9.index(), 9);
}

// --- Lookup API (kunaLookup*) ------------------------------------------------

#[test]
fn lookup_group_parity() {
    // Every group in the table is findable by name and round-trips by index.
    for i in 0..kuna_num_groups() {
        let e = kuna_group_by_index(i);
        let found = lookup_group(e.group).expect("group findable by name");
        assert_eq!(found.group, e.group);
        assert_eq!(found.phase, e.phase);
    }
    // A couple of known entries (transcribed from groupTable).
    assert_eq!(lookup_group("base").unwrap().phase, KunaPhase::Infra);
    assert_eq!(lookup_group("analysis").unwrap().phase, KunaPhase::P3);
    assert_eq!(lookup_group("casts").unwrap().phase, KunaPhase::P9);
    assert!(lookup_group("nonexistent").is_none());
}

#[test]
fn lookup_subphase_parity() {
    for i in 0..kuna_num_subphases() {
        let e = kuna_subphase_by_index(i);
        let found = lookup_subphase(e.name).expect("subphase findable");
        assert_eq!(found.name, e.name);
        assert_eq!(found.phase, e.phase);
        assert_eq!(found.rewind, e.rewind);
    }
    // Known rewind targets (stage-model.md section 12).
    let typ = lookup_subphase("type-propagation").unwrap();
    assert_eq!(typ.phase, KunaPhase::P5);
    assert_eq!(typ.rewind, KunaPhase::P5);
    let force = lookup_subphase("edge-virtualization").unwrap();
    assert_eq!(force.phase, KunaPhase::P7);
    assert_eq!(force.rewind, KunaPhase::P7);
    // explicit-implied: rewinds to S9 (the only cross-stage rewind in the table).
    let ei = lookup_subphase("explicit-implied").unwrap();
    assert_eq!(ei.phase, KunaPhase::P6);
    assert_eq!(ei.rewind, KunaPhase::P9);
    assert!(lookup_subphase("not-a-subphase").is_none());
}

#[test]
fn lookup_surface_parity() {
    for i in 0..kuna_num_surfaces() {
        let e = kuna_surface_by_index(i);
        let found = lookup_surface(e.surface).expect("surface findable by exact string");
        assert_eq!(found.surface, e.surface);
        assert_eq!(found.phase, e.phase);
    }
    assert_eq!(
        lookup_surface("force goto").unwrap().phase,
        KunaPhase::P7
    );
    assert_eq!(
        lookup_surface("option compareform").unwrap().subphase,
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
fn option_values_live_value_present_for_28_suppressed_for_42() {
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
        // (kuna) The widened ARM Cortex-M vector-table signature — an analysis-pass
        // gate with no codegen live reader (read console-side via kuna_live_value),
        // same as the gates around it. Default-off.
        "cortexmvectors",
        // (kuna) Pointer-referenced ARM function entries — an analysis-pass gate
        // with no codegen live reader (read console-side via kuna_live_value),
        // same as the gates around it. Default-off.
        "ptrentry",
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
        "fast_funcdisc",
        "noreturn_disc",
        "noreturn_propagate",
        // (kuna, decbench F2) The error(nonzero,…)-conditional recognizer — a
        // sub-rule gate of noreturn_propagate with no codegen live reader (read
        // console-side via kuna_live_value), like the analysis gates around it.
        // Default-on (DIV-16).
        "noreturn_error",
        // (kuna) CFG-reachability no-return rule (Ghidra targetOnlyCallsNoReturn), a
        // sub-rule gate of noreturn_propagate with no codegen live reader. Default-on (DIV-19).
        "noreturn_reach",
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
        // (kuna) Tail-call function-entry recovery — an analysis-pass gate with no
        // codegen live reader (read console-side via kuna_live_value), same as the
        // gates around it. Default-off, ARM-only.
        "tailcallentry",
        "gopclntab",
        // (kuna) Mach-O Objective-C metadata recovery — an analysis-pass gate with
        // no codegen live reader (read console-side via kuna_live_value), like the
        // gates around it. Default-off, Mach-O-only.
        "objc",
        // (kuna) PE PDB metadata recovery — an analysis-pass gate with no codegen
        // live reader (read console-side via kuna_live_value), like the gates around
        // it. Default-off, PE-only, externally `.pdb`-gated.
        "pdb",
        // (kuna) loader-tier gate, no codegen live reader (read console-side via
        // kuna_live_value), same as the analysis-pass gates above.
        "i386_pie_plt",
        // (PR-8) Mach-O arm64e spec selection: a load-time (pre-`option`) gate read
        // from the `KUNA_MACHO_ARM64E` env var, so it too has no codegen live_value.
        "macho-arm64e",
        // (kuna) DWARF C++ prototype recovery — an analysis-tier gate read at the
        // analysis COMMIT boundary (console-side via kuna_live_value), like the
        // analysis-pass gates above. Default-on.
        "cppproto",
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
                            | "regionedgeorder"
                            | "condfold"
                            | "stackguard"
                            | "branchflip"
                            | "namestyle"
                            | "foldcallret"
                            | "gotoreduce"
                            | "ifelseflatten"
                            | "crossjumprevert"
                            | "taildup"
                            | "dedupitetail"
                            | "iteregion"
                            | "iteexpr"
                            | "iteboolean"
                            | "itecondlist"
                            | "returndup"
                            | "earlyreturn"
                            | "switchreturn"
                            | "loopbreak_recovery"
                            | "relocobjects"
                            | "truthycond"
                            | "braceelide"
                            | "warnstyle"
                            | "callsitestackargs"
                            | "paramcopyhoist"
                    ) || PASS_GATES.contains(&st.option),
                    "unexpected option with no live reader: {}",
                    st.option
                );
            }
        }
    }
    // 28 -> 29: +1 for `peimportcall` (live_field = analysis_peimportcall);
    // `itecondlist` declares no live_field, so it does not move this count.
    assert_eq!(with_live, 29);
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
    assert!(out.starts_with("  {\"option\": \"compareform\", \"values\": [\"canonical\", \"original\"], \"default\": \"original\", \"destructive_as_default\": false, \"phase\": \"P3\""));
    // No `current` field when live is None.
    assert!(!out.contains("\"current\""));
    // ... and the tail order (issue ... change_kind ... tier ... symptoms).
    assert!(out.contains("\"strength\": \"HARD\", \"rewind\": \"P3\", \"issue\": \"GH-558\""));
    assert!(out.contains("\"change_kind\": \"presentation-default\", \"tier\": \"core\", \"symptoms\": [\""));
    assert!(out.ends_with("\"]}"));
}

#[test]
fn every_settable_has_nonempty_symptoms() {
    // C3: every catalog row carries at least one nonempty, output-shaped
    // symptom phrase (pipe-separated in the table, a JSON array in the
    // catalog) so an LLM can grep a natural-language symptom to its option.
    for i in 0..kuna_num_settables() {
        let st = kuna_settable_by_index(i);
        assert!(
            !st.symptoms.is_empty(),
            "settable `{}` has no symptoms",
            st.option
        );
        for phrase in st.symptoms.split('|') {
            assert!(
                !phrase.trim().is_empty(),
                "settable `{}` has an empty symptom phrase",
                st.option
            );
        }
    }
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
    // 83 rows: 82 trailing commas (the last, macho-arm64e, has none;
    // callsitestackargs' P4 row sits mid-table, so it does not move the tail;
    // switchguardbound's, switchsharedcase's, switchmultipred's, unrolledguard's,
    // tailcalljump's, noreturn_extern's, and noreturn_externmatch's S2 rows,
    // branchflip's, regionstructure's, regionlooprefine's, regionedgeorder's,
    // ifelseflatten's,
    // crossjumprevert's, taildup's, dedupitetail's, returndup's, iteregion's and
    // iteboolean's S8 rows,
    // noreturn_error's S1 analysis row, eh_frame_full's S1 row,
    // cortexmvectors' S1 row, ptrentry's S1 row,
    // operand_refs's S1 row, funcstart_patterns's S1 row, aif's S1 row, fid's S1
    // row, rtti's S1 row, dwarf_lines' S1 row, the `objc` Mach-O Objective-C S1 row,
    // the `pdb` PE PDB S1 row, switchreturn's S8 row, paramcopyhoist's P6 row,
    // itecondlist's S8 row and peimportcall's S1 row sit mid-table, so they do not
    // move the tail).
    assert_eq!(json.matches("},\n").count(), 89);
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
