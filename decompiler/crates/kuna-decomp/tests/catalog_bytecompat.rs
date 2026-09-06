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
//! The fixture `tests/fixtures/phase_catalog.json` is the EXACT bytes the C++
//! `decomp_dbg` binary emits for `phase catalog` **with no program loaded**
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
//! printf 'openfile write /tmp/cap.json\nphase catalog\nclosefile\nquit\n' \
//!   | decompiler/target/release/decomp_dbg -s "$PWD/specs" >/dev/null 2>&1
//! cp /tmp/cap.json decompiler/crates/kuna-decomp/tests/fixtures/phase_catalog.json
//! ```
//!
//! Regenerate it whenever `phases.toml` gains/loses a settable or a settable's
//! catalog text changes.

use kuna_decomp::kuna_phases::{emit_catalog_json, emit_catalog_json_one, lookup_settable};

/// The captured `phase catalog` output (no program loaded -> no `current`).
const FIXTURE: &str = include_str!("fixtures/phase_catalog.json");

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
fn fixture_has_all_153_settables() {
    // One `"option":` per settable row; the authoritative per-option list is
    // phases.toml settableTable (counts asserted in kuna_phases/tests.rs).
    assert_eq!(FIXTURE.matches("\"option\": ").count(), 153);
    // Every row carries the tier field appended after change_kind.
    assert_eq!(FIXTURE.matches("\"tier\": ").count(), 153);
    // ... and the symptoms array appended after tier (C3).
    assert_eq!(FIXTURE.matches("\"symptoms\": ").count(), 153);
}

#[test]
fn single_settable_rows_match_their_slice_of_the_catalog() {
    // `phase catalog <option>` (one-argument form) must emit exactly the same
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
