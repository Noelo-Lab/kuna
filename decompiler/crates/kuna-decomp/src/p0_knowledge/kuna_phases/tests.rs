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
fn subphase_count_is_45() {
    // +1 for the P9 `condition-form` subphase (truthycond, DIV-36),
    // +1 for the P9 `brace-form` subphase (braceelide, DIV-37),
    // +1 for the P9 `warning-style` subphase (warnstyle, DIV-38),
    // +1 for the P9 `array-cover-width` subphase (arraycoverwidth, DIV-122).
    // +1 for the P9 `empty-string-constant` subphase (emptystrconst, DIV-125).
    assert_eq!(kuna_num_subphases(), 45);
    assert_eq!(SUBPHASE_TABLE.len(), 45);
}

#[test]
fn surface_count_is_110() {
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
    // +1 for the `option funcboundflow` surface row (kuna cross-function-merge fix).
    // +1 for the `option securitycheck` surface row (kuna rustc panic-branch
    // stripping, DIV-82) -- the P7 edge-virtualization sibling of `stackguard`.
    // +1 for the `option retinputhalf` surface row (kuna P4 output-prototype:
    // keep a returned register half that is a placed input parameter).
    // +1 for the `option rustabi` surface row (kuna P4 output-prototype: keep the
    // two-register rustc ScalarPair return and connect it at the call).
    // +1 for the `option overlapbranch` surface row (kuna P2 flow-classification:
    // a conditional branch target strictly inside its own fall-through instruction).
    // +1 for the `option tailcallframe` surface row (kuna P2 flow-classification:
    // a direct jmp preceded by a teardown of exactly the entry block's frame).
    // +1 for the `option rodatastring` surface row (kuna P5 constsequence: collapse
    // a read-only string block copy into builtin_strncpy).
    // +1 for the `option arraycoverwidth` surface row (kuna P9 array-cover-width:
    // render a multi-element array cover at its real width, DIV-122).
    // +1 for the `option emptystrconst` surface row (kuna P9 empty-string-constant:
    // keep the address when the string literal would be empty, DIV-125).
    assert_eq!(kuna_num_surfaces(), 110);
    assert_eq!(SURFACE_TABLE.len(), 110);
}

#[test]
fn settable_count_is_156() {
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
    // +1 for `fdeinterior` (P1 `.eh_frame` FDE-interior entry suppression, DIV-61).
    // +1 for `cppsig` (P1 demangled C++ signature application).
    // +1 for `typedepth` (P1 full-depth DWARF type resolution, DIV-63).
    // +1 for `itaniumrtti` (P1 Itanium GCC/Clang RTTI + vtable recovery, DIV-64).
    // +1 for `libcsigs` (P1 measured libc signature extension, DIV-65).
    // +1 for `funcboundflow` (P2 fall-through bound at function entries).
    // +1 for `poolentry` (P1 ARM literal-pool inference).
    // +1 for `guardarm` (P8 ruleBlockIfNoExit arm tie-break).
    // +1 for `loopcondhoist` (P8 deferred-scan loop-head deferral).
    // +1 for `calloverlap` (P3 partial-range call-overlap guards, GH-275).
    // +1 for `orchain` (S8 returndup short-circuit operand-chain protection).
    // +1 for `evalcurrentproto` (P4 compiler-spec current-function prototype model).
    // +1 for `ifuncfpret` (P1 x86-64 IFUNC IRELATIVE PLT-stub naming).
    // +1 for `outline` (S8 region excision into a synthesized pseudofunction).
    // +1 for `msvcftol` (P2 MSVC __ftol-family call-fixup, DIV-74).
    // +1 for `ctypes` (P9 valid per-architecture C type spelling, DIV-75).
    // +1 for `datasyms` (P1 ELF data-symbol naming gate, DIV-76, GH-184).
    // +1 for `loadguardrange` (P3 indexed-stack guard ValueSet range refinement, GH-182).
    // +1 for `relocrebase` (P1 relocatable-object analysis rebase, DIV-79, GH-289).
    // +1 for `aifstrict` (P1 AIF gap-cursor aligned slide, GH-299).
    // +1 for `spillargtrial` (P4 caller-save spill tolerance in input-trial scoring, GH-275).
    // +1 for `securitycheck` (P7 rustc panic-branch stripping, DIV-82).
    // +1 for `cleanupcode` (P2 Rust drop/deallocate call removal, DIV-81).
    // +1 for `dynrelocs` (P1 linked-image dynamic-relocation application, DIV-84).
    // +1 for `retinputhalf` (P4 returned input-parameter half retention, DIV-85).
    // +1 for `dwarfstructs` (P1 DWARF aggregate-layout import, DIV-86).
    // +1 for `rustabi` (P4 rustc two-register ScalarPair return recovery).
    // +1 for `dwarfvariants` (P1 DWARF variant-part import, DIV-87).
    // +1 for `symbolnamerepair` (P1 degenerate-symbol-name repair, DIV-88).
    // +1 for `noreturn_discstrict` (P1 discovered-no-return positive-evidence-only
    // tally, DIV-92, GH-312).
    // +1 for `aifcorroborate` (P1 AIF accept corroboration test, GH-313).
    // +1 for `symbolnamechars` (P1 symbol-name character sanitizing, DIV-94).
    // +1 for `symbolnamebound` (P1 symbol-name scope resource bound, DIV-95, GH-338).
    // +1 for `msvcfpconst` (P1 MSVC `__real@` FP-constant recovery, DIV-96).
    // +1 for `cortexmpriv` (P2 Cortex-M privileged-mode guard folding, DIV-99).
    // +1 for `linuxsyscall` (P2 32-bit Linux int 0x80 syscall naming).
    // +1 for `unmappedentry` (P1 unmapped-CALL-target entry suppression).
    // +1 for `entrymainproto` (P1 PE CRT entry-function prototype recovery).
    // +1 for `ppclocalentry` (P1 PPC64 ELFv2 local-entry entry suppression).
    // +1 for `pdatachained` (P1 PE chained-`UNWIND_INFO` `.pdata` entry
    // suppression, DIV-117, GH-403).
    // +1 for `noreturnretuse` (P4 terminal no-return call use in return trials,
    // DIV-118).
    assert_eq!(kuna_num_settables(), 156);
    assert_eq!(SETTABLE_TABLE.len(), 156);
}

#[test]
fn tier_counts_are_41_core_61_transform_54_analysis() {
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
    // analysis 29 -> 30: +1 for `fdeinterior` (P1 FDE-interior entry suppression, DIV-61).
    // analysis 30 -> 31: +1 for `cppsig` (P1 demangled C++ signature application).
    // analysis 31 -> 32: +1 for `typedepth` (P1 full-depth DWARF types, DIV-63).
    // analysis 32 -> 33: +1 for `itaniumrtti` (P1 Itanium GCC/Clang RTTI + vtable
    // recovery, DIV-64).
    // analysis 33 -> 34: +1 for `libcsigs` (P1 measured libc signature extension, DIV-65).
    // transform 41 -> 42: +1 for `funcboundflow` (P2 fall-through bound at function entries).
    // analysis 34 -> 35: +1 for `poolentry` (P1 ARM literal-pool inference).
    // transform 42 -> 43: +1 for `guardarm` (P8 ruleBlockIfNoExit arm tie-break).
    // transform 43 -> 44: +1 for `loopcondhoist` (P8 deferred-scan loop-head deferral).
    // core 20 -> 21: +1 for `calloverlap` (P3 partial-range call-overlap guards, GH-275).
    // transform 44 -> 45: +1 for `orchain` (S8 returndup short-circuit chain gate).
    // core 21 -> 22: +1 for `evalcurrentproto` (P4 compiler-spec current-function
    // prototype model, DIV-71).
    // analysis 35 -> 36: +1 for `ifuncfpret` (P1 x86-64 IFUNC PLT-stub naming).
    // transform 45 -> 46: +1 for `outline` (deletes blocks, synthesizes a call).
    // transform 46 -> 47: +1 for `msvcftol` (P2 MSVC __ftol call-fixup, DIV-74).
    // core 22 -> 23: +1 for `ctypes` (P9 valid C type spelling, DIV-75).
    // analysis 36 -> 37: +1 for `datasyms` (P1 ELF data-symbol naming, DIV-76).
    // core 23 -> 24: +1 for `loadguardrange` (P3 guard ValueSet range refinement, GH-182).
    // analysis 37 -> 38: +1 for `relocrebase` (P1 relocatable-object analysis rebase, DIV-79).
    // analysis 38 -> 39: +1 for `aifstrict` (P1 AIF gap-cursor aligned slide, GH-299).
    // transform 47 -> 48: +1 for `spillargtrial` (P4 caller-save spill tolerance, GH-275)
    // -- transform, not core: it INSERTS a call argument, and is right on the spill/reload
    // shape and wrong on an ordinary frame store, which is the transform tier's definition.
    // transform 48 -> 50: +1 for `securitycheck` (P7 rustc panic-branch stripping,
    // DIV-82) -- transform, like its `stackguard` sibling: it deletes real
    // instructions on a name trigger -- and +1 for `cleanupcode` (P2 Rust
    // drop/deallocate call removal, DIV-81).
    // analysis 39 -> 40: +1 for `dynrelocs` (P1 linked-image dynamic relocations, DIV-84).
    // core 24 -> 25: +1 for `retinputhalf` (P4 returned input-parameter half
    // retention, DIV-85) -- core, not transform: it narrows a classification the
    // engine already makes, and never rewrites anything the narrowing does not
    // reach.
    // analysis 40 -> 41: +1 for `dwarfstructs` (P1 DWARF aggregate-layout import, DIV-86).
    // core 25 -> 26: +1 for `rustabi` (P4 rustc ScalarPair return) -- core, not
    // transform: it keeps a value the engine already recovered instead of
    // introducing a new rewrite.
    // analysis 41 -> 42: +1 for `dwarfvariants` (P1 DWARF variant-part import,
    // DIV-87).
    // analysis 42 -> 43: +1 for `symbolnamerepair` (P1 degenerate-symbol-name
    // repair, DIV-88).
    // analysis 43 -> 44: +1 for `aifcorroborate` (P1 AIF accept corroboration test,
    // GH-313) -- analysis, like its `aifstrict` sibling: it shapes which entries the
    // discovery tier emits, and rewrites nothing.
    // analysis 44 -> 45: +1 for `symbolnamechars` (P1 symbol-name character
    // sanitizing, DIV-94).
    // transform 50 -> 51: +1 for `noreturn_discstrict` (P1 discovered-no-return
    // positive-evidence-only tally, DIV-92) -- transform, not analysis: it changes
    // which callees are marked no-return, so it changes emitted C at every caller.
    // analysis 43 -> 44: +1 for `symbolnamebound` (P1 symbol-name scope resource
    // bound, DIV-95, GH-338).
    // analysis 43 -> 44: +1 for `msvcfpconst` (P1 MSVC `__real@` FP-constant
    // recovery, DIV-96).
    // core 26 -> 27: +1 for `framelayout` (P6 recovered-stack-frame reporting on
    // the `decompile-all --json` `variables` surface, DIV-97) -- core, not
    // transform: it changes no p-code and no emitted C, only what the JSON
    // surface reports about the frame the analysis already recovered.
    // transform 51 -> 52: +1 for `cortexmpriv` (P2 Cortex-M privileged-mode guard
    // folding, DIV-99).
    // transform 52 -> 53: +1 for `linuxsyscall` (P2 32-bit Linux int 0x80 syscall
    // naming) -- transform, not core: it renames a call and locks a prototype,
    // which is the judgement an operator flips.
    // analysis 47 -> 48: +1 for `unmappedentry` (P1 unmapped-CALL-target entry
    // suppression).
    // analysis 48 -> 49: +1 for `entrymainproto` (P1 PE CRT entry-function
    // prototype recovery).
    // analysis 50 -> 51: +1 for `ppclocalentry` (P1 PPC64 ELFv2 local-entry entry
    // suppression).
    // analysis 53 -> 54: +1 for `pdatachained` (P1 PE chained-`UNWIND_INFO`
    // `.pdata` entry suppression, DIV-117).
    // core 35 -> 36: +1 for `noreturnretuse` (P4 terminal no-return call use in
    // return trials, DIV-118) -- core, not transform: it narrows which competing
    // uses veto an output trial, changing no p-code of its own.
    assert_eq!((core, transform, analysis), (41, 61, 54));
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
fn option_values_live_value_present_for_53_suppressed_for_96() {
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
        // it. Default-off, PE-only, externally `.pdb`-gated.
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
                            | "securitycheck"
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
                            | "orchain"
                            | "earlyreturn"
                            | "switchreturn"
                            | "loopbreak_recovery"
                            | "relocobjects"
                            | "truthycond"
                            | "braceelide"
                            | "warnstyle"
                            | "arraycoverwidth"
                            | "emptystrconst"
                            | "callsitestackargs"
                            | "varargstackargs"
                            | "calleearity"
                            | "calleearityfwd"
                            | "calleearitylive"
                            | "calleedeadarg"
                            | "calleepreserves"
                            | "calloverlap"
                            | "spillargtrial"
                            | "paramcopyhoist"
                            | "guardarm"
                            | "loopcondhoist"
                            | "rustabi"
                    ) || PASS_GATES.contains(&st.option),
                    "unexpected option with no live reader: {}",
                    st.option
                );
            }
        }
    }
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
    assert_eq!(with_live, 53);
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
    assert_eq!(json.matches("},\n").count(), 155);
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
