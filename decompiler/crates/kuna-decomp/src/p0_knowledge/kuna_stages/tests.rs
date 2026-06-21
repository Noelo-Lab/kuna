//! Unit tests for the kuna stage registry (`kuna_stages.rs`).
//!
//! Parity targets transcribed from `decompiler/cpp/kuna_stages.cc`:
//! group=39, substage=40, surface=90, settable=23, plus the stage-code helpers,
//! the lookup API, the typed `OptionValues` defaults, and the catalog emitter.

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
fn surface_count_is_90() {
    assert_eq!(kuna_num_surfaces(), 90);
    assert_eq!(SURFACE_TABLE.len(), 90);
}

#[test]
fn settable_count_is_23() {
    assert_eq!(kuna_num_settables(), 23);
    assert_eq!(SETTABLE_TABLE.len(), 23);
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
fn option_values_live_value_present_for_20_suppressed_for_3() {
    let ov = OptionValues::default();
    // 20 options have a live reader (realtypes joins the field-backed group); the
    // live_value returns the current value for them and None for
    // loweredswitch/stackguard/namestyle.
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
                    matches!(st.option, "loweredswitch" | "stackguard" | "namestyle"),
                    "unexpected option with no live reader: {}",
                    st.option
                );
            }
        }
    }
    assert_eq!(with_live, 20);
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
    // 23 rows: 22 trailing commas (the last has none).
    assert_eq!(json.matches("},\n").count(), 22);
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
