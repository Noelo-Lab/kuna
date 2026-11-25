//! Unit tests for the kuna phase registry (`kuna_phases.rs`).
//!
//! Parity targets (origin: `decompiler/cpp/kuna_stages.cc`, since grown):
//! group/subphase/surface/settable counts checked against the registry sources, the phase-code
//! helpers, the lookup API, the typed `OptionValues` defaults, and the
//! catalog emitter.

use super::*;

#[test]
fn catalog_matches_implemented_options() {
    use std::collections::BTreeSet;
    use crate::options::KUNA_OPTION_NAMES;

    let registered: BTreeSet<_> = KUNA_OPTION_NAMES.iter().copied().collect();
    let documented: BTreeSet<_> = SETTABLE_TABLE.iter().map(|row| row.option).collect();
    assert_eq!(registered.len(), KUNA_OPTION_NAMES.len(), "duplicate option handler");
    assert_eq!(documented.len(), SETTABLE_TABLE.len(), "duplicate catalog option");
    assert_eq!(registered, documented);
}

#[test]
fn github_anchor_matches_rendered_heading_text() {
    let cases = [
        ("`objc` -- on | off, default `off`", "objc----on--off-default-off"),
        (
            "`returnpair` -- pair | single, default `pair` (destructive opt-in)",
            "returnpair----pair--single-default-pair-destructive-opt-in",
        ),
        (
            "`fooopt` -- off | 0 | 1 | <n>, default `2`",
            "fooopt----off--0--1---default-2",
        ),
    ];
    for (heading, anchor) in cases {
        assert_eq!(github_anchor(heading), anchor, "{heading}");
    }
}

/// The lines of the registry sources: `phases.toml` and every `phases.d/*.toml`.
fn registry_lines() -> Vec<String> {
    let crate_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut files = vec![crate_dir.join("phases.toml")];
    for entry in std::fs::read_dir(crate_dir.join("phases.d")).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().is_some_and(|x| x == "toml") {
            files.push(path);
        }
    }
    let text: Vec<String> = files.iter().map(|f| std::fs::read_to_string(f).unwrap()).collect();
    text.iter().flat_map(|t| t.lines()).map(str::to_string).collect()
}

fn registry_rows(table: &str) -> usize {
    let header = format!("[[{table}]]");
    registry_lines().iter().filter(|l| l.trim_end() == header).count()
}

#[test]
fn table_counts_match_the_registry_sources() {
    assert_eq!(kuna_num_groups(), registry_rows("group"));
    assert_eq!(GROUP_TABLE.len(), kuna_num_groups());
    assert_eq!(kuna_num_subphases(), registry_rows("subphase"));
    assert_eq!(SUBPHASE_TABLE.len(), kuna_num_subphases());
    assert_eq!(kuna_num_surfaces(), registry_rows("surface"));
    assert_eq!(SURFACE_TABLE.len(), kuna_num_surfaces());
    assert_eq!(kuna_num_settables(), registry_rows("settable"));
    assert_eq!(SETTABLE_TABLE.len(), kuna_num_settables());
}

#[test]
fn tier_counts_match_the_registry_sources() {
    let mut compiled = [0usize; 3];
    let mut sourced = [0usize; 3];
    let slot = |tier: &str| ["core", "transform", "analysis"].iter().position(|t| *t == tier);
    for s in SETTABLE_TABLE.iter() {
        let i = slot(s.tier).unwrap_or_else(|| panic!("invalid tier {:?} on {}", s.tier, s.option));
        compiled[i] += 1;
    }
    for tier in registry_lines().iter().filter_map(|l| l.strip_prefix("tier = \"")) {
        sourced[slot(tier.trim_end_matches('"')).unwrap()] += 1;
    }
    assert_eq!(compiled, sourced);
    assert_eq!(compiled.iter().sum::<usize>(), kuna_num_settables());
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
fn option_values_live_value_present_for_every_live_field_row() {
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
    // loweredswitch/stackguard/namestyle/foldcallret PLUS the
    // analysis/loader-tier gates (which have no `live_field` — their live state
    // is read console-side via the hand-written `kuna_live_value` / an env gate,
    // not the codegen `live_value`; +1 for `funcstart_patterns`, the full
    // byte-pattern function-start pass).
    const PASS_GATES: &[&str] = &[
        "noreturn_known",
        // (kuna) The source-language override — a loadtime env bridge
        // (`KUNA_SOURCELANG`) whose live
        // state is read console-side via `kuna_live_value` (it reports the
        // option, `auto` by default, not the detected fact), so it has no
        // codegen live reader.
        "sourcelang",
        // (kuna) dyld shared cache image selection and cross-image naming -
        // loadtime env bridges (`KUNA_DYLD_IMAGE`, `KUNA_DYLDCROSSREFS`).
        "dyldimage",
        "dyldcrossrefs",
        // (kuna) golang cspec selection - a loadtime env bridge (`KUNA_GOABI`)
        // whose live state is read
        // console-side via `kuna_live_value` (auto|off token), so it has no
        // codegen live reader.
        "goabi",
        // (kuna) The length-carried literal rule — three-valued, read
        // console-side via `kuna_live_value` (its token), no codegen live
        // reader.
        "fatstr",
        // (kuna) The named libc/POSIX aggregate types — a load-time analysis gate
        // read through the `KUNA_LIBCTYPES` env bridge (the named shells are
        // interned while the prototype pass builds its signatures, upstream of
        // every `option` command), so no codegen live reader. Default-ON
        // (`opaque`).
        "libctypes",
        // (kuna) The libgcc `__divdi3` contract -- an analysis-pass gate read
        // console-side via kuna_live_value, like `libcsigs`. Default-off.
        "libgccsigs",
        // (kuna) The Go `_type` descriptor globals -- an analysis-pass commit
        // gate (`analysis_go_type_globals`) read console-side via
        // kuna_live_value, like `libgccsigs`. Default-on.
        "gotypeglobals",
        // (kuna) The Go newobject result typing (the sitetypelock client) --
        // an on/off engine gate read console-side via kuna_live_value, like
        // `gotypeglobals`. Default-on.
        "gonewtype",
        // (kuna) The MS-DOS data segment in DS -- an on/off engine gate
        // (`Architecture::mz_dgroup_on`) read console-side via kuna_live_value,
        // like `gonewtype`. Default-on, MZ loads only.
        "mzdgroup",
        "strings",
        // (kuna) The 2-byte (UTF-16LE) width of the string-literal pass — an
        // analysis-pass gate read at the commit boundary (console-side via
        // kuna_live_value), same as `strings` above. Default-ON (DIV-110).
        "widestrings",
        // (kuna) The 4-byte width of the string-literal markup -- read at the
        // commit boundary like `widestrings`. Default-off (aggressive on).
        "widestrings32",
        "entry_disc",
        // (kuna) `.eh_frame` LSDA landing-pad discovery sub-feature of entry_disc
        // (GccExceptionAnalyzer), default-off; analysis-tier, no codegen live reader.
        "eh_frame_full",
        // (kuna) Mach-O `__unwind_info` function-start discovery — an analysis-pass
        // gate with no codegen live reader (read console-side via kuna_live_value),
        // same as `eh_frame_full` above. Default-off.
        "unwindstarts",
        // (kuna) The full byte-pattern function-start pass — an analysis-pass gate
        // with no codegen live reader (read console-side via kuna_live_value), same
        // as the gates around it. Default-off.
        "funcstart_patterns",
        "armframes",
        // (kuna) The widened ARM Cortex-M vector-table signature — an analysis-pass
        // gate with no codegen live reader (read console-side via kuna_live_value),
        // same as the gates around it. Default-off.
        "cortexmvectors",
        // (kuna) Pointer-referenced ARM function entries — an analysis-pass gate
        // with no codegen live reader (read console-side via kuna_live_value),
        // same as the gates around it. Default-off.
        "ptrentry",
        "dwarf",
        "dwarf_lines",
        "cvftrace",
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
        // (kuna) FLIRT signature matcher, the same Listing-consumer gate as `fid`;
        // its signatures come from `KUNA_FLIRT_SIGS`. Default-off.
        "flirt",
        // (kuna) MSVC RTTI / vftable class-name recovery — a PE-only analysis-pass
        // gate (no `live_field`); its live state is read console-side via
        // kuna_live_value, like the analysis-pass gates around it. Default-off.
        "rtti",
        // (kuna) ELF libc-start `main` naming + prototype -- the ELF counterpart of
        // `machomain` above and the same seam: an analysis-tier gate with no
        // codegen live reader, read console-side via kuna_live_value. Default-ON.
        "elfmain",
        "x86listingswitch",
        // (kuna) Address-taken code function entries -- an analysis-tier gate with
        // no codegen live reader (read console-side via kuna_live_value), like
        // `thunkentry` above. Default-ON.
        "addrtakenentry",
        // (kuna, GH-780) ARM flow-proven decode-mode paints -- an analysis-tier
        // gate read console-side via kuna_live_value, like `ppclocalentry` above.
        // Default-ON.
        "flowmode",
        "aif",
        // (kuna, GH-299) The AIF literal-pool slide, read console-side like `aif`.
        "aifpoolslide",
        // (kuna) Tail-call function-entry recovery — an analysis-pass gate with no
        // codegen live reader (read console-side via kuna_live_value), same as the
        // gates around it. Default-off, ARM-only.
        "tailcallentry",
        // (kuna) Call tables: an analysis-pass gate read console-side, like
        // `tailcallentry` above. Default-on.
        "calltable",
        // (kuna) Stripped Mach-O stub naming: an analysis-pass gate read
        // console-side, like `calltable` above. Default-on.
        "stubbinds",
        // (kuna) Jump-thunk naming: a commit gate read console-side, like
        // `stubbinds` above. Default-on.
        "thunknames",
        // (kuna) ARM literal-pool inference — an analysis-pass gate with no codegen
        // live reader (read console-side via kuna_live_value), same as
        // `tailcallentry` above. Default-off, ARM-only.
        "poolentry",
        // (kuna) Mach-O Objective-C metadata recovery — an analysis-pass gate with
        // no codegen live reader (read console-side via kuna_live_value), like the
        // gates around it. Default-off, Mach-O-only.
        "objc",
        // (kuna) The Swift runtime-metadata pass gate, the objc precedent.
        "swiftmetadata",
        // (kuna) Mach-O __cfstring constant-string recovery — an analysis-pass gate
        // with no codegen live reader (read console-side via kuna_live_value), like
        // the gates around it. Default-on, Mach-O-only.
        "cfstrings",
        // (kuna) Embedded image and archive data symbols: a commit-side gate.
        "blobs",
        // (kuna) The Swift type-layout arm's gate, read inside that pass.
        "swifttypes",
        // (kuna) The Swift helper arm's gate, read inside the metadata pass.
        "swifthelpers",
        // (kuna) Objective-C instance layouts — same commit-side gate as `objc`.
        "objcivars",
        // (kuna) The Swift calling-convention arm's gate, read inside that pass.
        "swiftcall",
        // (kuna) The Swift @objc-thunk naming arm's gate, read at the commit.
        "swiftobjcthunk",
        // (kuna) The Swift value-witness arm's gate, read inside that pass and
        // by the decompiler's witness-table vote (`ArchContext`).
        "valuewitness",
        // (kuna) The Objective-C SDK header directory — a load-time env bridge
        // (`KUNA_OBJCSDK`, the sourcelang pattern); no codegen live reader.
        "objcsdk",
        // (kuna) PE PDB metadata recovery — an analysis-pass gate with no codegen
        // live reader (read console-side via kuna_live_value), like the gates around
        // it. Default-on (DIV-129), PE-only, `.pdb`-sidecar-gated.
        "pdb",
        "ppc32picimports",
        // (kuna) x86-64 IFUNC PLT-stub naming: a load-time gate read via the
        // `kuna_ifuncfpret` env var (no codegen live reader), like `i386_pie_plt`.
        "ifuncfpret",
        // (kuna) PowerPC APU-note language selection: a load-time gate read via
        // the `kuna_ppcapu` env var, like `ifuncfpret`.
        "ppcapu",
        // (kuna, GH-340) Symbol-name character sanitizing: a load-time gate read
        // via the `kuna_symbolnamechars` env var (names are minted inside `load
        // file`), like `symbolnamerepair`. Three-valued, default `safe`.
        "symbolnamechars",
        // (PR-8) Mach-O arm64e spec selection: a load-time (pre-`option`) gate read
        // from the `KUNA_MACHO_ARM64E` env var, so it too has no codegen live_value.
        "macho-arm64e",
        "godwarfsig",
        // (kuna) Demangled C++ signature application — an analysis-tier gate read
        // at the analysis COMMIT boundary (console-side via kuna_live_value), like
        // `cppproto` above. Three-valued, default `proven`.
        "cppsig",
        // (kuna) Itanium (GCC/Clang) RTTI + vtable recovery — an analysis-tier gate
        // read at the analysis COMMIT boundary (console-side via kuna_live_value),
        // like the analysis-pass gates above. Default-on, ELF-only.
        "itaniumrtti",
        // (kuna `lazyfuncs`) The deferred entry commit -- read console-side at
        // `read symbols` (the commit either creates a Function per entry or parks
        // the starts), so it has no codegen live reader. Default-off.
        "lazyfuncs",
        // (kuna) Apple kernelcache unwrapping + MH_FILESET entry choice -- a
        // LOAD-time gate read from `KUNA_KERNELCACHE`. Default auto.
        "kernelcache",
        // (kuna) Where a kernel collection's linked collections are found -- a
        // LOAD-time gate read from `KUNA_KERNELCACHE_BOOT`. Default auto.
        "kernelcache-boot",
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
                        "outline"
                            | "outlang"
                            | "loweredswitch"
                            | "regionstructure"
                            | "regionlooprefine"
                            | "loopcontinue"
                            | "regionedgeorder"
                            | "condfold"
                            // (kuna) `condstmts` takes `off` or an INTEGER cap;
                            // its live value is `Architecture::cond_stmts`.
                            | "condstmts"
                            | "stackguard"
                            | "msvcstackguard"
                            | "securitycheck"
                            | "branchflip"
                            | "namestyle"
                            | "foldcallret"
                            // (kuna) `foldcallretphi` extends `foldcallret`'s
                            // decision and shares its seam, so it reports its
                            // live state the same way its sibling does.
                            // (kuna) `indirectonly` gates an engine-side
                            // Varnode flag read by `HighVariable::has_name` and
                            // `Merge::merge_test_adjacent`, not a printer flag;
                            // like `foldcallret` above it declares no
                            // `live_field`, so the catalog suppresses `current`.
                            | "indirectonly"
                            | "gotoreduce"
                            | "ifelseflatten"
                            | "crossjumprevert"
                            | "dedupitetail"
                            | "iteregion"
                            | "iteexpr"
                            | "iteboolean"
                            | "returndup"
                            | "rettriangle"
                            | "earlyreturn"
                            | "switchreturn"
                            | "loopbreak_recovery"
                            | "truthycond"
                            | "braceelide"
                            | "warnstyle"
                            | "infloopstyle"
                            | "int3pad"
                            | "x64syscall"
                            | "pebnames"
                            // (kuna) `ptrfromuse` takes `off|byte|void`, which
                            // the codegen live reader (a bool
                            // `live_true`/`live_false` pair) cannot express --
                            // the same reason `pebnames` is here. Its live value
                            // is `Architecture::ptr_from_use`.
                            | "ptrfromuse"
                            // (kuna) `readonlycalls` takes `off|on|<n>`, a guard
                            // count; its live value is `Architecture::readonly_calls`.
                            | "readonlycalls"
                            // (kuna) `persistcalls` takes `off|on|<n>`, a guard
                            // count; its live value is `Architecture::persist_calls`.
                            | "persistcalls"
                            // (kuna) `deadcascade` takes `off|on|<n>`, a pass
                            // count; its live value is `Architecture::dead_cascade`.
                            | "deadcascade"
                            // (kuna) `retsplitlimit` takes `off|<n>`, an op count;
                            // its live value is `Architecture::ret_split_limit`.
                            | "retsplitlimit"
                            // (kuna) `tailform` takes `off|on|<n>|<void>,<value>`;
                            // its live value is `Architecture::tail_form`.
                            | "tailform"
                            // (kuna) `narrowargs` takes `off|on|callers`; its
                            // live value is the pair `Architecture::narrow_args`
                            // and `Architecture::narrow_args_callers`.
                            | "narrowargs"
                            // (kuna) `structsynth` takes a MODE (`off|param|nest`),
                            // for the same reason.  Its live value is
                            // `Architecture::struct_synth`.
                            | "structsynth"
                            // (kuna) `structmerge` takes a MODE
                            // (`off|siblings`) over an enum field, for the same
                            // reason.  Its live value is
                            // `Architecture::struct_merge`.
                            | "structmerge"
                            // (kuna) `structheadless` takes a MODE
                            // (`off|closed`) over an enum field, for the same
                            // reason.  Its live value is
                            // `Architecture::struct_headless`.
                            | "structheadless"
                            // (kuna) `protoorder` takes a MODE
                            // (`off|types|cycles|lock`) over an enum field, for the
                            // same reason.  Its live value is
                            // `Architecture::protoorder`.
                            | "protoorder"
                            // (kuna) `calleevote` takes a MODE
                            // (`off|types|fields`) over an enum field, for the
                            // same reason.  Its live value is
                            // `Architecture::calleevote`.
                            | "calleevote"
                            // (kuna) `callbacktype` is an enum field the same
                            // way; its live value is `Architecture::callbacktype`.
                            | "callbacktype"
                            // (kuna `calleereginputs`) three-valued
                            // (`off|on|custom`), so its live value is read
                            // console-side from its two engine flags, not a
                            // single codegen `live_field`.
                            | "calleereginputs"
                            // (kuna `castwiden`) takes a MODE
                            // (`off|on|literal`) over an enum field, for the
                            // same reason.  Its live value is
                            // `Architecture::cast_widen`.
                            | "castwiden"
                            // (kuna) `armconstshift` takes a MODE
                            // (`off|on|all`) over an enum field, for the same
                            // reason.  Its live value is
                            // `Architecture::arm_constant_shift`.
                            | "armconstshift"
                            // (kuna) `armshiftguard` takes a MODE
                            // (`always|needed|wide|never`) over an enum field; its
                            // live value is `Architecture::arm_shift_guard`.
                            | "armshiftguard"
                            // (kuna) `constselectjump` takes a MODE
                            // (`off|reentry|on`) over an enum field, for the
                            // same reason.  Its live value is
                            // `Architecture::const_select_jump`.
                            | "constselectjump"
                            // (kuna) `looptest` takes a MODE
                            // (`binary|proven|assume`) over an enum field; its
                            // live value is `Architecture::loop_test`.
                            | "looptest"
                            // (kuna) `declorder` takes a MODE (`natural|name`)
                            // over an enum field, for the same reason.  Its live
                            // value is `Architecture::decl_order`.
                            | "declorder"
                            // (kuna) `narrowext` takes a MODE
                            // (`off|abi|compiler`) over an enum field, for the
                            // same reason.  Its live value is
                            // `Architecture::narrow_ext`.
                            | "narrowext"
                            // (kuna) `syscallregs` takes a MODE
                            // (`off|auto|on`) over an enum field, for the
                            // same reason.  Its live value is
                            // `Architecture::syscall_regs`.
                            | "syscallregs"
                            // (kuna) `linuxsyscall` takes the same MODE
                            // (`off|auto|on`) over an enum field; its live
                            // value is `Architecture::linux_syscall`.
                            | "linuxsyscall"
                            // (kuna) `callmechanisms` takes a MODE
                            // (`off|auto|on`) over an enum field, for the
                            // same reason.  Its live value is
                            // `Architecture::call_mechanism_mode`.
                            | "callmechanisms"
                            // (kuna) `bejoin` takes a MODE (`off|evidence|on`)
                            // over an enum field, for the same reason.  Its live
                            // value is `Architecture::be_join`.
                            | "bejoin"
                            // (kuna) `bejoinpass` likewise (`off|whole|high`);
                            // its live value is `Architecture::be_join_pass`.
                            | "bejoinpass"
                            // (kuna) `octeoneffects` likewise
                            // (`off|atomic|ordered`); its live value is
                            // `Architecture::octeon_effects`.
                            | "octeoneffects"
                            | "bytearrayc"
                            // (kuna) `structdefs` is a PrintC option: its live
                            // state is `PrintC::options.struct_defs`, which the
                            // codegen live reader (an `Architecture` bool
                            // member) cannot reach.
                            | "structdefs"
                            // (kuna) `globalref` is a PrintC option too:
                            // `PrintC::options.global_ref`.
                            | "globalref"
                            | "mixedtailret"
                            | "calloverlap"
                            | "indexaliasguard"
                            // (kuna) `arrayextent` takes a MODE
                            // (`off|bound|on`) over an int field; its live
                            // value is `Architecture::array_extent`.
                            | "arrayextent"
                            // (kuna) `arraysize` takes a MODE (`off|on|calls`)
                            // over an int field; its live value is
                            // `Architecture::array_size`.
                            | "arraysize"
                            | "spillargtrial"
                            | "paramcopyhoist"
                            | "guardarm"
                            | "loopcondhoist"
                            | "rustabi"
                            // (kuna) `gomorestack` takes a MODE
                            // (`off|auto|always`) over a u8 field, the
                            // rustabi shape; its live value is
                            // `Architecture::go_morestack`.
                            | "gomorestack"
                            | "rustprobestack"
                            | "rustfmt"
                            // (kuna) `gobool` takes the same MODE shape; its
                            // live value is `Architecture::go_bool`.
                            | "gobool"
                            // (kuna) `fpuemu` takes the same MODE shape; its
                            // live value is `Architecture::fpu_emu`.
                            | "fpuemu"
                            // (kuna) `gowritebarrier`: the same MODE shape;
                            // its live value is `Architecture::go_write_barrier`.
                            | "gowritebarrier"
                            // (kuna) `impliedrefs`/`termdup` take an INTEGER
                            // (`2|3|4|<n>`), which the codegen live reader (a
                            // bool `live_true`/`live_false` pair) cannot
                            // express -- the same reason `int3pad`, `warnstyle`
                            // and `namestyle` are here. Their live values are
                            // `Architecture::max_implied_ref` /
                            // `max_term_duplication`.
                            | "impliedrefs"
                            | "termdup"
                            // (kuna) `jumptablemax` takes an INTEGER; its
                            // live value is `Architecture::max_jumptable_size`.
                            | "jumptablemax"
                            // (kuna) `signedness` takes FOUR values
                            // (`upstream|auto|prefer-signed|prefer-unsigned`)
                            // over an enum field, which the codegen live reader
                            // (a bool `live_true`/`live_false` pair) cannot
                            // express -- the same reason `namestyle`,
                            // `warnstyle` and `int3pad` are here.  Its live
                            // value is `Architecture::signedness`.
                            | "signedness"
                    ) || PASS_GATES.contains(&st.option),
                    "unexpected option with no live reader: {}",
                    st.option
                );
            }
        }
    }
    let rows_with_live = registry_lines().iter().filter(|l| l.starts_with("live_field = ")).count();
    assert_eq!(with_live, rows_with_live);
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
    assert_eq!(OptionValues::live_from_arch("memsetrecover", on), Some("on"));
    assert_eq!(OptionValues::live_from_arch("memsetrecover", off), Some("off"));
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
    assert_eq!(json.matches("},\n").count(), kuna_num_settables() - 1);
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
