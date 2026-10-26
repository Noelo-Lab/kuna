//! Unit tests for the kuna phase registry (`kuna_phases.rs`).
//!
//! Registry membership, public count relationships, valid tiers, phase-code
//! helpers, lookup behavior, typed `OptionValues` defaults, and catalog APIs.

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
fn registry_count_apis_match_their_tables() {
    assert_eq!(kuna_num_groups(), GROUP_TABLE.len());
    assert_eq!(kuna_num_subphases(), SUBPHASE_TABLE.len());
    assert_eq!(kuna_num_surfaces(), SURFACE_TABLE.len());
    assert_eq!(kuna_num_settables(), SETTABLE_TABLE.len());
}

#[test]
fn every_settable_has_a_valid_tier() {
    for s in SETTABLE_TABLE.iter() {
        assert!(
            matches!(s.tier, "core" | "transform" | "analysis"),
            "invalid tier {:?} on {}",
            s.tier,
            s.option
        );
    }
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
fn option_values_live_defaults_match_shipped_values() {
    let ov = OptionValues::default();
    // Field-backed options expose their current value through the generated
    // reader. Loader gates and options with hand-written live readers are
    // checked separately below.
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
        // (kuna) Discriminant-value naming -- a load-time env bridge
        // (`KUNA_VARIANTNAMES`, the dwarfstructs/dwarfvariants pattern) whose
        // live state is read console-side via `kuna_live_value`, so it has no
        // codegen live reader.
        "variantnames",
        // (kuna) The source-language override — a loadtime env bridge
        // (`KUNA_SOURCELANG`, the rexthunk/relocobjects pattern) whose live
        // state is read console-side via `kuna_live_value` (it reports the
        // option, `auto` by default, not the detected fact), so it has no
        // codegen live reader.
        "sourcelang",
        // (kuna) Swift demangling - a loadtime env bridge (`KUNA_SWIFTDEMANGLE`,
        // the rexthunk pattern); live state read console-side.
        "swiftdemangle",
        // (kuna) golang cspec selection - a loadtime env bridge (`KUNA_GOABI`,
        // the rexthunk/sourcelang pattern) whose live state is read
        // console-side via `kuna_live_value` (auto|off token), so it has no
        // codegen live reader.
        "goabi",
        // (kuna) The goroutine-register naming — a load-time env bridge
        // (`KUNA_GOREG`, the goabi seam), read console-side via kuna_live_value.
        "goreg",
        // (kuna) The Go pclntab no-return pass gate - an analysis-pass gate
        // with no codegen live reader (read console-side via kuna_live_value),
        // same as `gopclntab`. Default-ON (Go-only pass).
        "gonoreturn",
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
        // (kuna) The 4-byte width of the string-literal markup -- read at the
        // commit boundary like `widestrings`. Default-off (aggressive on).
        "widestrings32",
        "entry_disc",
        // (kuna) `.eh_frame` LSDA landing-pad discovery sub-feature of entry_disc
        // (GccExceptionAnalyzer), default-off; analysis-tier, no codegen live reader.
        "eh_frame_full",
        // (kuna) `.eh_frame` FDE-interior entry suppression — an analysis-pass gate
        // with no codegen live reader (read console-side via kuna_live_value), same
        // as the gates around it. Default-ON (DIV-61).
        "fdeinterior",
        // (kuna) Multi-entry `.cold` fragment entries — an analysis-pass gate with
        // no codegen live reader. Default-ON.
        "coldentry",
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
        "armframes",
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
        // (kuna, GH-299) Thumb tbb/tbh switch-table read in the Listing walk --
        // an analysis-tier gate with no codegen live reader, like `unmappedentry`
        // above. Default-ON.
        "thumbswitch",
        "x86listingswitch",
        // (kuna) Register-chain jump-table reach in the reference walk -- an
        // analysis-tier gate with no codegen live reader, like `thumbswitch`
        // above. Default-ON.
        "switchreach",
        // (kuna) Jump-thunk target function entries -- an analysis-tier gate with
        // no codegen live reader (read console-side via kuna_live_value), like
        // `unmappedentry` above. Default-ON.
        "thunkentry",
        // (kuna) Address-taken code function entries -- an analysis-tier gate with
        // no codegen live reader (read console-side via kuna_live_value), like
        // `thunkentry` above. Default-ON.
        "addrtakenentry",
        // (kuna) PPC64 ELFv2 local-entry entry suppression -- an analysis-tier gate
        // with no codegen live reader (read console-side via kuna_live_value), like
        // `unmappedentry` above. Default-ON.
        "ppclocalentry",
        // (kuna, GH-780) ARM flow-proven decode-mode paints -- an analysis-tier
        // gate read console-side via kuna_live_value, like `ppclocalentry` above.
        // Default-ON.
        "flowmode",
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
        // (kuna, GH-299) The AIF bracketed-candidate reject — an analysis-tier gate
        // with no codegen live reader (read console-side via kuna_live_value), like
        // `aifcorroborate` above. Default-ON, inert without `aif`.
        "aifbracket",
        // (kuna, GH-299) The AIF filler reject, read console-side like `aifbracket`.
        "aifnoppad",
        // (kuna, GH-299) The AIF literal-pool slide, read console-side like `aifnoppad`.
        "aifpoolslide",
        // (kuna) Tail-call function-entry recovery — an analysis-pass gate with no
        // codegen live reader (read console-side via kuna_live_value), same as the
        // gates around it. Default-off, ARM-only.
        "tailcallentry",
        // (kuna) ARM literal-pool inference — an analysis-pass gate with no codegen
        // live reader (read console-side via kuna_live_value), same as
        // `tailcallentry` above. Default-off, ARM-only.
        "poolentry",
        "gopclntab",
        // (kuna) The Go runtime-type harvest - a loadtime env bridge
        // (KUNA_GOTYPES, the dwarfstructs pattern); no codegen live reader.
        "gotypes",
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
        // (kuna) The MSVC declaration arm of the demangled signatures -- the same
        // commit-boundary gate as `cppsig`, read console-side. Default-on.
        "msvcsig",
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
        // (kuna `lazyfuncs`) The deferred entry commit -- read console-side at
        // `read symbols` (the commit either creates a Function per entry or parks
        // the starts), so it has no codegen live reader. Default-off.
        "lazyfuncs",
    ];
    for i in 0..kuna_num_settables() {
        let st = kuna_settable_by_index(i);
        match ov.live_value(st.option) {
            Some(v) => {
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
                            | "rettriangle"
                            | "earlyreturn"
                            | "switchreturn"
                            | "loopbreak_recovery"
                            | "relocobjects"
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
                            // (kuna `castwiden`) takes a MODE
                            // (`off|on|literal`) over an enum field, for the
                            // same reason.  Its live value is
                            // `Architecture::cast_widen`.
                            | "castwiden"
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
                            // (kuna) `bejoin` takes a MODE (`off|evidence|on`)
                            // over an enum field, for the same reason.  Its live
                            // value is `Architecture::be_join`.
                            | "bejoin"
                            | "arraycoverwidth"
                            | "bytearrayc"
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
                            | "protoranges"
                            | "varargstackargs"
                            | "varargforward"
                            | "calleearity"
                            | "calleearityfwd"
                            | "calleearitylive"
                            | "calleearitybody"
                            | "calleearitycut"
                            | "calleearityscratch"
                            | "calleedeadarg"
                            | "hiddenretarg"
                            | "mixedtailret"
                            | "calleereadarg"
                            | "zerofillreturn"
                            | "zerocallregs"
                            | "calleepreserves"
                            | "calleeretpreserves"
                            | "calleescratchbody"
                            | "indirectanchor"
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
                            // (kuna) `gobool` takes the same MODE shape; its
                            // live value is `Architecture::go_bool`.
                            | "gobool"
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
fn emit_catalog_json_one_unknown_is_none() {
    assert!(emit_catalog_json_one("bogus", None).is_none());
    assert!(emit_catalog_json_one("namestyle", None).is_some());
}

#[test]
fn json_string_escaping() {
    // Direct check of the escape rules (quote, backslash, newline, control).
    let mut out = String::new();
    json_string(&mut out, "a\"b\\c\nd\te");
    // tab (0x09) is a control char < 0x20 and is NOT '\n' -> collapses to space.
    assert_eq!(out, "\"a\\\"b\\\\c\\nd e\"");
}
