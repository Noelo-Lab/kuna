//! Catalog byte-compatibility gate (ADR 0006, item w4-kuna-p0-pack).
//!
//! The catalog JSON emitter (`kuna_phases::emit_catalog_json`) must keep
//! `kuna_console.cc kunaEmitSettableJson`'s byte format: `tests/stages/kuna-catalog.xml`
//! string-matches the raw emitter output, and `python -m kuna.catalog` parses it, so any
//! drift in key order, spacing, separators, pipe-splitting, the conditional `current`
//! field, or escaping would silently break the LLM control surface.
//!
//! ## What is compared
//!
//! The full catalog must be exactly `[\n`, one row per registry entry in registry
//! order joined by `,\n`, and `\n]\n`, with each row the bytes of
//! `phase catalog <option>`. The row format itself is pinned by
//! `tests/fixtures/phase_catalog_rows.json`: a handful of rows captured verbatim with
//! **no program loaded** (so `kunaLiveValue` returns `""` and no `current` field is
//! present), chosen to cover the first row, a destructive default, a `<n>` value, an
//! escaped quote, an escaped backslash and non-ASCII text. A new option needs no
//! fixture change; editing one of those six rows' text does. Recapture them with the
//! `openfile write` trick (console prompts never pollute the bytes):
//!
//! ```sh
//! printf 'openfile write /tmp/cap.json\nphase catalog\nclosefile\nquit\n' \
//!   | decompiler/target/release/decomp_dbg -s "$PWD/specs" >/dev/null 2>&1
//! grep -E '^  \{"option": "(compareform|returnpair|protoorder|cppsig|retsplitlimit|macho-arm64e)"' \
//!   /tmp/cap.json | sed 's/},$/}/' > decompiler/crates/kuna-decomp/tests/fixtures/phase_catalog_rows.json
//! ```

use kuna_decomp::kuna_phases::{
    emit_catalog_json, emit_catalog_json_one, kuna_num_settables, kuna_settable_by_index,
    lookup_settable,
};

/// Captured `phase catalog` rows (no program loaded -> no `current`), one per line.
const FIXTURE: &str = include_str!("fixtures/phase_catalog_rows.json");

fn rows() -> Vec<String> {
    (0..kuna_num_settables())
        .map(|i| {
            let st = kuna_settable_by_index(i);
            let one = emit_catalog_json_one(st.option, None).expect("known option");
            assert!(one.starts_with("  {\"option\": "), "{}", st.option);
            assert!(one.ends_with("}\n"), "{}", st.option);
            one.trim_end_matches('\n').to_string()
        })
        .collect()
}

#[test]
fn full_catalog_is_every_row_in_registry_order() {
    let full = emit_catalog_json(|_| None);
    assert_eq!(full, format!("[\n{}\n]\n", rows().join(",\n")));
}

#[test]
fn pinned_rows_match_byte_for_byte() {
    let full = emit_catalog_json(|_| None);
    let emitted: Vec<&str> = full.lines().map(|l| l.trim_end_matches(',')).collect();
    assert_eq!(FIXTURE.lines().count(), 6);
    for pinned in FIXTURE.lines() {
        assert!(emitted.contains(&pinned), "row drifted from its capture:\n{pinned}");
    }
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
fn every_row_carries_the_tier_and_symptoms_fields() {
    let full = emit_catalog_json(|_| None);
    for key in ["\"option\": ", "\"tier\": ", "\"symptoms\": "] {
        assert_eq!(full.matches(key).count(), kuna_num_settables(), "{key}");
    }
}

#[test]
fn unknown_single_settable_is_none() {
    // C++ throws IfaceExecutionError for an unknown option; the port returns None.
    assert!(emit_catalog_json_one("not-a-real-option", None).is_none());
    assert!(lookup_settable("not-a-real-option").is_none());
}
