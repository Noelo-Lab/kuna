//! Unit tests for the kuna phase registry (`kuna_phases.rs`).
//!
//! Parity targets (origin: `decompiler/cpp/kuna_stages.cc`, since grown):
//! group/subphase/surface/settable counts pinned by the asserts below, plus the phase-code
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
fn group_count_is_39() {
    assert_eq!(kuna_num_groups(), 39);
    assert_eq!(GROUP_TABLE.len(), 39);
}

#[test]
fn subphase_count_is_47() {
    assert_eq!(kuna_num_subphases(), 47);
    assert_eq!(SUBPHASE_TABLE.len(), 47);
}

#[test]
fn surface_count_is_120() {
    assert_eq!(kuna_num_surfaces(), 120);
    assert_eq!(SURFACE_TABLE.len(), 120);
}

#[test]
fn settable_count_is_246() {
    assert_eq!(kuna_num_settables(), 246);
    assert_eq!(SETTABLE_TABLE.len(), 246);
}

#[test]
fn tier_counts_are_77_core_103_transform_66_analysis() {
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
    assert_eq!((core, transform, analysis), (77, 103, 66));
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
fn option_values_live_value_present_for_105() {
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
    // 19 analysis/loader-tier gates (which have no `live_field` — their live state
    // is read console-side via the hand-written `kuna_live_value` / an env gate,
    // not the codegen `live_value`; +1 for `funcstart_patterns`, the full
    // byte-pattern function-start pass). `relocobjects` (DIV-8) gates the loader,
    // not a printer/engine flag, so it too has no codegen live reader.
    const PASS_GATES: &[&str] = &[
        // (kuna) Linked-image dynamic-relocation application (DIV-84): a load-time
        // gate read via the `kuna_dynrelocs` env var (the relocations are applied
        // while the loader snapshots the image), like `relocrebase`.
        "dynrelocs",
        "noreturn_known",
        "libproto",
        // (kuna) The measured libc signature extension — an analysis-pass gate with
        // no codegen live reader (read console-side via kuna_live_value), same as
        // `libproto` above. Default-ON (DIV-65).
        "libcsigs",
        // (kuna) The named libc/POSIX aggregate types — a load-time analysis gate
        // read through the `KUNA_LIBCTYPES` env bridge (the named shells are
        // interned while the prototype pass builds its signatures, upstream of
        // every `option` command), so no codegen live reader. Default-ON
        // (`opaque`).
        "libctypes",
        // (kuna) The declared-name libc prototype lookup — read console-side at
        // declaration time (`ConsoleProgram::declare_function`), so it has no
        // codegen live reader either. Default-ON (DIV-139).
        "declaredlibcproto",
        // (kuna) The built-in Win32 API signature table — an analysis-pass gate
        // with no codegen live reader (read console-side via kuna_live_value),
        // same as `libcsigs` above. Default-ON (DIV-141).
        "win32sigs",
        "strings",
        // (kuna) The 2-byte (UTF-16LE) width of the string-literal pass — an
        // analysis-pass gate read at the commit boundary (console-side via
        // kuna_live_value), same as `strings` above. Default-ON (DIV-110).
        "widestrings",
        "entry_disc",
        // (kuna) `.eh_frame` LSDA landing-pad discovery sub-feature of entry_disc
        // (GccExceptionAnalyzer), default-off; analysis-tier, no codegen live reader.
        "eh_frame_full",
        // (kuna) `.eh_frame` FDE-interior entry suppression — an analysis-pass gate
        // with no codegen live reader (read console-side via kuna_live_value), same
        // as the gates around it. Default-ON (DIV-61).
        "fdeinterior",
        // (kuna) `.pdata` RUNTIME_FUNCTION-interior entry suppression — the PE half
        // of `fdeinterior`, an analysis-pass gate with no codegen live reader.
        // Default-ON.
        "pdatainterior",
        // (kuna) PDB-procedure-interior entry suppression — the PDB half of
        // `pdatainterior`, an analysis-pass gate with no codegen live reader.
        // Default-ON.
        "pdbinterior",
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
        // (kuna) ELF data-symbol (`STT_OBJECT`) naming — the loader-collected,
        // commit-gated data twin of the funcsym stream, with no codegen live
        // reader (read console-side via kuna_live_value), same as the gates
        // around it. Default-ON (DIV-76).
        "datasyms",
        "dwarf_lines",
        "callfixup",
        "addrtable",
        "operand_refs",
        "formatstring",
        "listing",
        "fast_funcdisc",
        // (kuna, DIV-185) Headerless-image function discovery -- an analysis-tier
        // gate read console-side via kuna_live_value, like the gates around it.
        // Default-ON, raw-container only.
        "rawdiscover",
        "noreturn_disc",
        // (kuna, GH-312) The positive-evidence-only tally for `noreturn_disc` -- a
        // sub-rule gate with no codegen live reader (read console-side via
        // kuna_live_value), like the analysis gates around it. Default-ON (DIV-92).
        "noreturn_discstrict",
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
        // (kuna) PE CRT entry-function prototype recovery -- an analysis-tier gate
        // with no codegen live reader (read console-side via kuna_live_value), like
        // the discovery gates around it. Default-ON.
        "entrymainproto",
        // (kuna) Mach-O `LC_MAIN` entry naming + prototype -- the Mach-O counterpart
        // of `entrymainproto` above and the same seam: an analysis-tier gate with no
        // codegen live reader, read console-side via kuna_live_value. Default-ON.
        "machomain",
        // (kuna) PE user-entry naming -- the PE counterpart of `machomain` above
        // and the same seam. Default-ON.
        "pemain",
        // (kuna) non-PIE ARM crt1 `_start`->`main` recovery -- the ARM ELF counterpart
        // of `machomain` above and the same seam: an analysis-tier gate with no
        // codegen live reader, read console-side via kuna_live_value. Default-ON.
        "armlibcmain",
        // (kuna) ELF libc-start `main` naming + prototype -- the ELF counterpart of
        // `machomain` above and the same seam: an analysis-tier gate with no
        // codegen live reader, read console-side via kuna_live_value. Default-ON.
        "elfmain",
        // (kuna) Unmapped-CALL-target entry suppression -- an analysis-tier gate with
        // no codegen live reader (read console-side via kuna_live_value), like the
        // discovery gates around it. Default-ON.
        "unmappedentry",
        // (kuna) PPC64 ELFv2 local-entry entry suppression -- an analysis-tier gate
        // with no codegen live reader (read console-side via kuna_live_value), like
        // `unmappedentry` above. Default-ON.
        "ppclocalentry",
        // (kuna, GH-403) PE chained-`UNWIND_INFO` `.pdata` entry suppression -- an
        // analysis-tier gate with no codegen live reader (read console-side via
        // kuna_live_value), like `ppclocalentry` above. Default-ON.
        "pdatachained",
        // (kuna) x86-64 PE REX-prefixed import-thunk rejection -- a LOAD-time gate
        // read from the `KUNA_REXTHUNK` env var, like `pdatachained` above.
        // Default-ON.
        "rexthunk",
        // (kuna) PE import-by-ordinal naming -- a LOAD-time gate read from the
        // `KUNA_PEORDINAL` env var, like `rexthunk` above. Default-ON.
        "peordinal",
        "aif",
        // (kuna, GH-299) The AIF gap-cursor aligned slide — an analysis-tier gate
        // with no codegen live reader (read console-side via kuna_live_value), like
        // `aif` above. Default-OFF, carried by the `aggressive` preset.
        "aifstrict",
        // (kuna, GH-313) The AIF accept corroboration test — an analysis-tier gate
        // with no codegen live reader (read console-side via kuna_live_value), like
        // `aifstrict` above. Default-OFF, carried by the `aggressive` preset.
        "aifcorroborate",
        // (kuna) Tail-call function-entry recovery — an analysis-pass gate with no
        // codegen live reader (read console-side via kuna_live_value), same as the
        // gates around it. Default-off, ARM-only.
        "tailcallentry",
        // (kuna) ARM literal-pool inference — an analysis-pass gate with no codegen
        // live reader (read console-side via kuna_live_value), same as
        // `tailcallentry` above. Default-off, ARM-only.
        "poolentry",
        "gopclntab",
        // (kuna) Mach-O Objective-C metadata recovery — an analysis-pass gate with
        // no codegen live reader (read console-side via kuna_live_value), like the
        // gates around it. Default-off, Mach-O-only.
        "objc",
        // (kuna) PE PDB metadata recovery — an analysis-pass gate with no codegen
        // live reader (read console-side via kuna_live_value), like the gates around
        // it. Default-on (DIV-129), PE-only, `.pdb`-sidecar-gated.
        "pdb",
        // (kuna) loader-tier gate, no codegen live reader (read console-side via
        // kuna_live_value), same as the analysis-pass gates above.
        "i386_pie_plt",
        // (kuna) x86-64 IFUNC PLT-stub naming: a load-time gate read via the
        // `kuna_ifuncfpret` env var (no codegen live reader), like `i386_pie_plt`.
        "ifuncfpret",
        // (kuna) Relocatable-object analysis rebase (GH-289): a load-time gate read
        // via the `kuna_relocrebase` env var (the analyzer tier runs inside `load
        // file`), like `i386_pie_plt`. Default-ON (DIV-79).
        "relocrebase",
        // (kuna) Degenerate-symbol-name repair: a load-time gate read via the
        // `kuna_symbolnamerepair` env var (the symbol table is installed inside
        // `load file`), like `relocrebase`. Default-ON.
        "symbolnamerepair",
        // (kuna, GH-340) Symbol-name character sanitizing: a load-time gate read
        // via the `kuna_symbolnamechars` env var (names are minted inside `load
        // file`), like `symbolnamerepair`. Three-valued, default `safe`.
        "symbolnamechars",
        // (kuna, GH-338) Symbol-name scope resource bound: the same load-time
        // seam, read via the `kuna_symbolnamebound` env var. VALUED (the scope
        // ceiling is a number), so its live `current` is read console-side via
        // kuna_live_value. Default 32.
        "symbolnamebound",
        // (kuna) MSVC `__real@` FP-constant recovery (DIV-96): a load-time gate
        // read via the `kuna_msvcfpconst` env var (the decoded bytes are
        // materialised while the loader lays the object out), like
        // `symbolnamerepair`. Default-ON.
        "msvcfpconst",
        // (PR-8) Mach-O arm64e spec selection: a load-time (pre-`option`) gate read
        // from the `KUNA_MACHO_ARM64E` env var, so it too has no codegen live_value.
        "macho-arm64e",
        // (kuna) DWARF C++ prototype recovery — an analysis-tier gate read at the
        // analysis COMMIT boundary (console-side via kuna_live_value), like the
        // analysis-pass gates above. Default-on.
        "cppproto",
        // (kuna) Demangled C++ signature application — an analysis-tier gate read
        // at the analysis COMMIT boundary (console-side via kuna_live_value), like
        // `cppproto` above. Three-valued, default `proven`.
        "cppsig",
        // (kuna) Full-depth DWARF type resolution — a LOAD-time gate read from the
        // `KUNA_TYPEDEPTH` env var (the types are mapped inside `load file`), so
        // like `macho-arm64e` above it has no codegen live_value. Default-on.
        "typedepth",
        // (kuna) Itanium (GCC/Clang) RTTI + vtable recovery — an analysis-tier gate
        // read at the analysis COMMIT boundary (console-side via kuna_live_value),
        // like the analysis-pass gates above. Default-off, ELF-only.
        "itaniumrtti",
        // (kuna) DWARF aggregate-layout import — a LOAD-time gate read from the
        // `KUNA_DWARFSTRUCTS` env var (the layout is installed on the interned type
        // inside `load file`), so like `typedepth` above it has no codegen
        // live_value. Default-on.
        "dwarfstructs",
        // (kuna) DWARF variant-part import -- a LOAD-time gate read from the
        // `KUNA_DWARFVARIANTS` env var (the overlay is installed on the interned
        // type inside `load file`), the same seam as `dwarfstructs` above.
        // Default-on.
        "dwarfvariants",
        // (kuna) PIC base-register folding in the cross-reference index -- an
        // analysis-tier gate read by the read-only xref query, console-side via
        // kuna_live_value like the analysis-pass gates above, so it has no
        // codegen live_value. Default-on.
        "picbase",
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
                            | "loweredswitch"
                            | "regionstructure"
                            | "regionlooprefine"
                            | "regionedgeorder"
                            | "condfold"
                            | "stackguard"
                            | "msvcstackguard"
                            | "securitycheck"
                            | "branchflip"
                            | "namestyle"
                            | "foldcallret"
                            // (kuna) `foldcallretphi` extends `foldcallret`'s
                            // decision and shares its seam, so it reports its
                            // live state the same way its sibling does.
                            | "foldcallretphi"
                            // (kuna) `indirectonly` gates an engine-side
                            // Varnode flag read by `HighVariable::has_name` and
                            // `Merge::merge_test_adjacent`, not a printer flag;
                            // like `foldcallret` above it declares no
                            // `live_field`, so the catalog suppresses `current`.
                            | "indirectonly"
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
                            | "orchain"
                            | "earlyreturn"
                            | "switchreturn"
                            | "loopbreak_recovery"
                            | "relocobjects"
                            | "truthycond"
                            | "braceelide"
                            | "warnstyle"
                            | "int3pad"
                            | "x64syscall"
                            | "pebnames"
                            // (kuna) `ptrfromuse` takes `off|byte|void`, which
                            // the codegen live reader (a bool
                            // `live_true`/`live_false` pair) cannot express --
                            // the same reason `pebnames` is here. Its live value
                            // is `Architecture::ptr_from_use`.
                            | "ptrfromuse"
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
                            // (kuna `castwiden`) takes a MODE
                            // (`off|on|literal`) over an enum field, for the
                            // same reason.  Its live value is
                            // `Architecture::cast_widen`.
                            | "castwiden"
                            | "arraycoverwidth"
                            | "emptystrconst"
                            // (kuna) `structdefs` is a PrintC option like
                            // `arraycoverwidth`/`emptystrconst` above: its live
                            // state is `PrintC::options.struct_defs`, which the
                            // codegen live reader (an `Architecture` bool
                            // member) cannot reach.
                            | "structdefs"
                            // (kuna) `globalref` is a PrintC option too:
                            // `PrintC::options.global_ref`.
                            | "globalref"
                            | "callsitestackargs"
                            | "varargstackargs"
                            | "calleearity"
                            | "calleearityfwd"
                            | "calleearitylive"
                            | "calleearitybody"
                            | "calleearitycut"
                            | "calleearityscratch"
                            | "calleedeadarg"
                            | "calleepreserves"
                            | "calleeretpreserves"
                            | "calleescratchbody"
                            | "indirectanchor"
                            | "calloverlap"
                            | "indexaliasguard"
                            | "spillargtrial"
                            | "paramcopyhoist"
                            | "guardarm"
                            | "loopcondhoist"
                            | "rustabi"
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
    // (kuna) `int3pad` joins the suppressed group (96 -> 97): it is a THREE-valued
    // option (`off|warn|halt`) whose live field is an enum, which the codegen
    // live reader (a bool `live_true`/`live_false` pair) cannot express -- the
    // same reason `warnstyle` and `namestyle` are there.
    // 28 -> 29: +1 for `peimportcall` (live_field = analysis_peimportcall);
    // `itecondlist` declares no live_field, so it does not move this count.
    // 29 -> 30: +1 for `funcboundflow` (live_field = funcbound_flow).
    // 30 -> 31: +1 for `evalcurrentproto` (live_field = evalcurrentproto).
    // 31 -> 32: +1 for `msvcftol` (live_field = msvc_ftol).
    // 32 -> 33: +1 for `ctypes` (live_field = ctypes).
    // 33 -> 34: +1 for `loadguardrange` (live_field = load_guard_range).
    // 34 -> 35: +1 for `cleanupcode` (live_field = remove_cleanup_code).
    // 35 -> 36: +1 for `retinputhalf` (live_field = ret_input_half).
    // 36 -> 37: +1 for `framelayout` (live_field = framelayout, DIV-97).
    // 38 -> 39: +1 for `cortexmpriv` (live_field = cortexmpriv, DIV-99).
    // 39 -> 40: +1 for `linuxsyscall` (live_field = linux_syscall).
    // 40 -> 41: +1 for `switchselector` (its own live_field).
    // 41 -> 42: +1 for `tiedstorekeep` (live_field = tied_store_keep, DIV-105).
    // 42 -> 43: +1 for `overlapbranch` (live_field = overlap_branch, DIV-106).
    // 43 -> 44: +1 for `ptrdepthcap` (live_field = ptrdepthcap, DIV-108).
    // 44 -> 45: +1 for `tailcallframe` (live_field = tail_call_frame, DIV-109).
    // 45 -> 46: +1 for `jtsharepartial` (live_field = jumptable_share_partial).
    // 46 -> 47: +1 for `rodatastring` (live_field = rodata_string, DIV-113).
    // 47 -> 48: +1 for `inputparamgap` (live_field = input_param_gap, DIV-114).
    // 48 -> 49: +1 for `simdlane` (live_field = simd_lane_fold, DIV-115).
    // 49 -> 50: +1 for `retsplitglobal` (live_field = ret_split_global, DIV-116).
    // 50 -> 51: +1 for `noreturnretuse` (live_field = noreturn_ret_use, DIV-118).
    // 51 -> 52: +1 for `fastfailnoreturn` (live_field = fastfail_noreturn, DIV-119).
    // 53 -> 54: +1 for `constselectjump` (live_field = const_select_jump).
    // 54 -> 55: +1 for `calleepop` (live_field = callee_pop).
    // 55 -> 56: +1 for `litpoolconst` (live_field = litpoolconst, DIV-136).
    // 56 -> 57: +1 for `codescalar` (live_field = codescalar, DIV-138).
    // 57 -> 58: +1 for `stackarggap` (live_field = stack_arg_gap, DIV-140).
    // 58 -> 59: +1 for `calleeprotostack` (live_field = callee_proto_stack,
    // DIV-142).
    // 59 -> 60: +1 for `paramrefdecl` (live_field = param_ref_decl, DIV-143).
    // 60 -> 61: +1 for `calltrampoline` (live_field = call_trampoline, DIV-144).
    // 61 -> 62: +1 for `loopcounterstore` (live_field = loop_counter_store,
    // DIV-146).
    // 62 -> 63: +1 for `zeroidiomuse` (live_field = zero_idiom_use).
    // 63 -> 64: +1 for `decodehalt` (live_field = decode_halt, DIV-151).
    // 64 -> 65: +1 for `splitstorekeep` (live_field = split_store_keep,
    // DIV-153).
    // 65 -> 66: +1 for `entrythumbflow` (live_field = analysis_entrythumbflow).
    // 66 -> 67: +1 for `retpushedhalf` (live_field = ret_pushed_half, DIV-156).
    // 67 -> 68: +1 for `tailcallsaved` (live_field = tail_call_saved, DIV-157).
    // 68 -> 69: +1 for `constspaceload` (live_field = const_space_load_fold,
    // DIV-158).
    // 69 -> 70: +1 for `declhightype` (live_field = decl_high_type, DIV-160).
    // 70 -> 71: +1 for `exclusivearguse` (live_field = exclusive_arg_use,
    // DIV-161).
    // 71 -> 72: +1 for `callretpair` (live_field = call_ret_pair, DIV-162).
    // 72 -> 73: +1 for `callpopret` (live_field = call_pop_ret, DIV-163).
    // 73 -> 74: +1 for `entryretdispatch` (live_field = entry_ret_dispatch,
    // DIV-168).
    // 74 -> 75: +1 for `pushimmediateret` (live_field = push_immediate_ret,
    // DIV-170).
    // 75 -> 76: +1 for `loweredswitchlabels` (live_field = lowered_switch_labels,
    // DIV-173).
    // 76 -> 77: +1 for `cancelbytearithmetic` (live_field =
    // cancel_byte_arithmetic, DIV-174).
    // 78 -> 79: +1 for `nulterminator` (live_field = nul_terminator, opt-in).
    // 79 -> 80: +1 for `endptrbound` (live_field = end_ptr_bound, DIV-177).
    // 80 -> 81: +1 for `msvcstrappend` (live_field = msvc_str_append).
    // 81 -> 82: +1 for `loweredswitchvalue` (live_field =
    // lowered_switch_value_check, DIV-181).
    // 82 -> 83: +1 for `tiedphitrim` (live_field = tied_phi_trim, DIV-182).
    // 83 -> 85: +1 for `loweredswitchexact` (P2 re-rolled switch matches its compare tree, DIV-183) and +1 for `loweredswitchheads` (P2 every lowered-switch cascade head, DIV-184).
    // 85 -> 86: +1 for `bytehonest` (live_field = byte_honest).
    // 86 -> 87: +1 for `argclobber` (live_field = arg_clobber, opt-in).
    // 87 -> 88: +1 for `hideshadow` (live_field = hide_shadow, default-on).
    // 88 -> 89: +1 for `boolbyte` (live_field = bool_byte, opt-in).
    // 89 -> 90: +1 for `mulblob` (live_field = mul_blob, default-on).
    // 90 -> 91: +1 for `charbyte` (live_field = char_byte, default-on).
    // 92 -> 93: +1 for `charptr` (live_field = char_ptr, opt-in).
    // 93 -> 94: +1 for `slotptr` (live_field = slot_ptr, default-on).
    // 94 -> 95: +1 for `passthrough` (live_field = pass_through).
    // 95 -> 96: +1 for `castimplied` (live_field = cast_implied).
    // 97 -> 98: +1 for `castsign` (live_field = cast_sign).
    // 98 -> 99: +1 for `castindex` (live_field = cast_index).
    // 99 -> 100: +1 for `castternary` (live_field = cast_ternary).
    // 100 -> 101: +1 for `callpush` (live_field = drop_call_push).
    // 101 -> 102: +1 for `callrettype` (live_field = call_ret_type).
    // 102 -> 103: +1 for `castobject` (live_field = cast_object).
    // 103 -> 104: +1 for `elemptr` (live_field = elem_ptr).
    // 104 -> 105: +1 for `condexeret` (live_field = cond_exe_ret).
    assert_eq!(with_live, 105);
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
    // itecondlist's S8 row, peimportcall's S1 row, cppproto's S1 row,
    // fdeinterior's S1 row, cppsig's S1 row, typedepth's S1 row, itaniumrtti's S1
    // row, libcsigs' S1 row, funcboundflow's S2 row, poolentry's S1 row, the
    // two P8 ifNoExit rows, calloverlap's P3 row, orchain's S8 row,
    // evalcurrentproto's P4 row, ifuncfpret's P1 row, msvcftol's P2 row and
    // datasyms' P1 row, loadguardrange's P3 row, relocrebase's P1 row and
    // aifstrict's P1 row, spillargtrial's P4 row, dynrelocs' P1 row and
    // retinputhalf's P4, dwarfstructs' P1, dwarfvariants' P1, rustabi's P4 and
    // symbolnamerepair's P1 rows sit mid-table, so they do not move the tail).
    // noreturn_discstrict's P1 row is mid-table too, so it only bumps the count.
    // retinputhalf's P4, dwarfstructs' P1, dwarfvariants' P1, rustabi's P4,
    // symbolnamerepair's P1 and aifcorroborate's P1 rows sit mid-table, so they do
    // not move the tail).
    // symbolnamerepair's P1 and symbolnamechars' P1 rows sit mid-table, so they
    // do not move the tail).
    // symbolnamerepair's P1 and symbolnamebound's P1 rows sit mid-table, so they
    // do not move the tail).
    // symbolnamerepair's P1 and msvcfpconst's P1 rows sit mid-table, so they do
    // not move the tail).
    // +1 for `pdatainterior` (a P1 row beside `fdeinterior`, mid-table, so it only
    // bumps the count).
    // 123 -> 124: +1 for `framelayout` (DIV-97; its P6 row sits mid-table ahead of
    // `ctypes`, so it does not move the tail either).
    // 127 -> 129: +1 for `unmappedentry` and +1 for `entrymainproto` (both P1 rows
    // mid-table, beside `fdeinterior`, so neither moves the tail either); the
    // count is one less than the settable total, since the last row has no comma.
    // +1 for `ppclocalentry` (another P1 row beside `unmappedentry`, mid-table).
    // +1 for `pdatachained`, appended at the TAIL, so the previous last row gains
    // its comma and the count moves with the total.
    // +1 for `varargstackargs` and +1 for `calleearity`; both P4 rows sit
    // mid-table beside `callsitestackargs`, so the tail does not move either.
    // 148 -> 149: +1 for `noreturnretuse` (DIV-118); its P4 row is appended after
    // the last one, so the previous tail row gains a comma and it becomes the tail.
    // 149 -> 150: +1 for `msvcstackguard` (GH-468); its P7 row sits mid-table
    // beside `securitycheck`, so the tail does not move.
    // 180 -> 181: +1 for `entrythumbflow` (DIV-154); its P1 row sits mid-table
    // beside `tailcallentry`, so the tail does not move.
    // +1 for `cancelbytearithmetic` (DIV-174); its P3 row sits mid-table, so it
    // increments the comma-terminated catalog-row count.
    // 193 -> 194: +1 for `pebnames` (DIV-175); its P5 row sits mid-table beside
    // `codescalar`, so it increments the comma-terminated catalog-row count.
    // 194 -> 195: +1 for `mappedflowboundary` (DIV-176); its P2 row sits mid-table
    // beside `funcboundflow`.
    // 195 -> 196: +1 for `nulterminator` (opt-in); its P6 row sits mid-table beside
    // `cookiescramble`, so it increments the comma-terminated catalog-row count.
    // 196 -> 197: +1 for `endptrbound` (DIV-177); its P6 row sits mid-table.
    // 197 -> 198: +1 for `rexthunk`; its P1 row sits mid-table beside `pdatachained`,
    // so it increments the comma-terminated catalog-row count.
    // 198 -> 199: +1 for `pdbinterior` (DIV-180); its P1 row sits mid-table beside
    // `pdatainterior`.
    // +1 for `msvcstrappend`; its P2 row sits mid-table.
    // 200 -> 201: +1 for `loweredswitchvalue` (DIV-181); its P2 row sits
    // mid-table, so it increments the comma-terminated catalog-row count.
    // +1 for `tiedphitrim` (DIV-182); its P6 row sits mid-table beside
    // `paramcopyhoist`, so it increments the comma-terminated catalog-row count.
    // 202 -> 204: +1 for `loweredswitchexact` (P2 re-rolled switch matches its compare tree, DIV-183) and +1 for `loweredswitchheads` (P2 every lowered-switch cascade head, DIV-184).
    // 209 -> 210: +1 for `libctypes`; its P1 row sits mid-table, so it
    // increments the comma-terminated catalog-row count.
    // 210 -> 211: +1 for `foldcallretphi`.
    // 211 -> 212: +1 for `ptrfromuse`; its P5 row sits mid-table, so it
    // increments the comma-terminated catalog-row count.
    // 214 -> 215: +1 for `structsynth`.
    // 215 -> 216: +1 for `signedness`; its P9 row sits mid-table, so it
    // increments the comma-terminated catalog-row count.
    // 216 -> 217: +1 for `boolbyte`; its P5 row sits mid-table, so it
    // increments it again.
    // 217 -> 218: +1 for `mulblob`; its P3 row sits mid-table, so it
    // increments it again.
    // 224 -> 225: +1 for `slotptr`; its P6 row sits mid-table.
    // 225 -> 226: +1 for `calleevote`.
    // 226 -> 227: +1 for `structmerge`; its P5 row sits mid-table.
    // 232 -> 233: +1 for `structheadless`; its P5 row sits mid-table.
    // 235 -> 236: +1 for `callpush`; its P4 row sits mid-table.
    // 237 -> 238: +1 for `peordinal`.
    // 238 -> 239: +1 for `jumptablemax`.
    // 240 -> 241: +1 for `callrettype`; its P4 row sits mid-table.
    // 241 -> 242: +1 for `castobject`.
    // 242 -> 243: +1 for `castwiden`.
    // 243 -> 244: +1 for `elemptr`.
    // 244 -> 245: +1 for `condexeret`; its P4 row sits mid-table.
    assert_eq!(json.matches("},\n").count(), 245);
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
