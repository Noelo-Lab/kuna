//! Tests for the kuna stage-model console commands (W9, `kuna_console.rs`).
//!
//! The kuna registry (`kuna_decomp::kuna_phases`) is pure static data, so the
//! registry-only commands (`phase list`/`map`/`catalog`, the `kassert`
//! validation core) produce their exact bytes with no program loaded — that is
//! what these tests pin. The `phase catalog` test compares against the **W4
//! byte-compatible emitter** directly (`emit_catalog_json` /
//! `emit_catalog_json_one`), which `tests/stages/kuna-catalog.xml`
//! string-matches against the C++ binary, so reproducing the emitter output is
//! the load-bearing contract.
//!
//! The engine-dependent kuna commands (`phase status`, `restarts`, `pipeline`
//! running a variant, `quality`, the `region *` commands, and a *routable*
//! `kassert`) read accessors the merged `rust-port` tree does not yet expose;
//! they are smoke-driven to confirm they *resolve and dispatch* — emitting their
//! faithful pre-engine guard text or the self-describing `engine_unavailable`
//! execution error — never an "Invalid command" (which would mean the
//! registration/prefix surface drifted).

use super::*;
use crate::ifacedecomp::register_decomp_commands;
use crate::ifaceterm::ConsoleCommands;
use crate::interface::IfaceStatus;
use kuna_decomp::kuna_phases::{emit_catalog_json, emit_catalog_json_one};

/// A console wired like the datatest runner: a `ConsoleCommands` feed with the
/// full decompiler command set **and** the kuna stage commands registered.
fn console(commands: &[&str]) -> IfaceStatus {
    let cmds: Vec<String> = commands.iter().map(|s| s.to_string()).collect();
    let mut status = ConsoleCommands::into_status(cmds);
    register_decomp_commands(&mut status);
    register_kuna_commands(&mut status);
    status
}

/// Run a single queued command via `run_command` and return the captured
/// command/bulk output (`optr`). No prompt is written, so the output is exactly
/// the command's own bytes (the bulk `file_out` stream lands in `optr` when no
/// `openfile` redirect is open). On a dispatched error the message is returned
/// with its console prefix (mirroring the driver's `execute`).
fn run_one(line: &str) -> String {
    let mut status = console(&[line]);
    match status.run_command() {
        Ok(_) => {}
        Err(err) => {
            if err.is_parse() {
                status.out(&format!("Command parsing error: {err}\n"));
            } else if err.is_execution() {
                status.out(&format!("Execution error: {err}\n"));
            } else {
                status.out(&format!("ERROR: {err}\n"));
            }
        }
    }
    status.optr
}

// ---------------------------------------------------------------------------
// Registration / prefix surface.
// ---------------------------------------------------------------------------

#[test]
fn registers_all_nineteen_kuna_commands() {
    // register_decomp_commands registers 105 (see ifacedecomp/tests.rs); the
    // kuna capability adds 19: phase list/map/status/catalog (plus the four
    // deprecated `stage ...` alias registrations), kassert, restarts,
    // pipeline, mode, quality, functions, region tree/blocks/walk,
    // `function bounds` and `map prototype`.
    let only_kuna = {
        let mut st = ConsoleCommands::into_status(vec![]);
        register_kuna_commands(&mut st);
        st.num_commands()
    };
    assert_eq!(only_kuna, 19);
    assert_eq!(console(&[]).num_commands(), 105 + 19);
}

#[test]
fn kuna_command_prefixes_expand() {
    let mut status = console(&[]);
    // The abbreviations the kuna datatests / kuna.catalog drive.
    assert_eq!(status.resolve("phase list").unwrap(), vec!["phase", "list"]);
    assert_eq!(status.resolve("phase map").unwrap(), vec!["phase", "map"]);
    assert_eq!(status.resolve("phase status").unwrap(), vec!["phase", "status"]);
    assert_eq!(status.resolve("phase catalog").unwrap(), vec!["phase", "catalog"]);
    assert_eq!(status.resolve("phase cat").unwrap(), vec!["phase", "catalog"]);
    // Deprecated `stage ...` aliases keep resolving.
    assert_eq!(status.resolve("stage list").unwrap(), vec!["stage", "list"]);
    assert_eq!(status.resolve("stage catalog").unwrap(), vec!["stage", "catalog"]);
    assert_eq!(status.resolve("kassert list").unwrap(), vec!["kassert"]);
    assert_eq!(status.resolve("pipeline list").unwrap(), vec!["pipeline"]);
    assert_eq!(status.resolve("quality").unwrap(), vec!["quality"]);
    assert_eq!(status.resolve("functions").unwrap(), vec!["functions"]);
    assert_eq!(status.resolve("region tree").unwrap(), vec!["region", "tree"]);
    assert_eq!(status.resolve("region blocks").unwrap(), vec!["region", "blocks"]);
    assert_eq!(status.resolve("region walk").unwrap(), vec!["region", "walk"]);
    assert_eq!(status.resolve("restarts").unwrap(), vec!["restarts"]);
}

// ---------------------------------------------------------------------------
// `map prototype <func> <C declaration>` — the console spelling of
// `--assert 'prototype <func> <decl>'`.
// ---------------------------------------------------------------------------

/// The new command must not have made the upstream `map ...` set ambiguous:
/// `map param` and `map address` are what the datatest corpus drives, `map fun`
/// among them as an abbreviation.
#[test]
fn map_prototype_does_not_shadow_the_upstream_map_commands() {
    let mut status = console(&[]);
    assert_eq!(status.resolve("map prototype f void f(void)").unwrap(), vec!["map", "prototype"]);
    assert_eq!(status.resolve("map param 0 %RDI int4 x").unwrap(), vec!["map", "param"]);
    assert_eq!(status.resolve("map addr 0x1000 int4 g").unwrap(), vec!["map", "address"]);
    assert_eq!(status.resolve("map fun 0x1000").unwrap(), vec!["map", "function"]);
}

/// Both operands are required, and the guards read like the rest of the module
/// (`function bounds`' "Missing ..." parse errors) rather than as a C syntax
/// error three layers down.
#[test]
fn map_prototype_names_a_missing_operand() {
    assert!(
        run_one("map prototype").contains("Missing function name"),
        "out: {:?}",
        run_one("map prototype")
    );
    assert!(
        run_one("map prototype authenticate").contains("Missing C declaration"),
        "out: {:?}",
        run_one("map prototype authenticate")
    );
}

/// With both operands but no image, the guard is the module's own
/// ("No load image present") — the same one `parse line` gives.
#[test]
fn map_prototype_without_image_is_no_load_image_present() {
    let out = run_one("map prototype authenticate void *hashit(void *out,void *input)");
    assert!(out.contains("No load image present"), "out: {out:?}");
}

// ---------------------------------------------------------------------------
// `phase list` — pure static registry data.
// ---------------------------------------------------------------------------

#[test]
fn stage_list_prints_the_stage_model_header_and_all_stages() {
    let out = run_one("phase list");
    assert!(
        out.starts_with("Phases (kuna phase model, docs/phases.md):\n"),
        "out: {out:?}"
    );
    // P0 is the orthogonal plane; P3..P6 are Band B.
    assert!(out.contains("  P0  Knowledge & Configuration Plane  [orthogonal plane]\n"));
    assert!(out.contains("  P3  Definition Web  [Band B]\n"));
    assert!(out.contains("  P6  Variable & Storage Model  [Band B]\n"));
    // P1/P2/P7..P9 carry no bracket tag.
    assert!(out.contains("  P1  Image & Code Partition\n"));
    assert!(out.contains("  P9  Surface Rendering & Refinement\n"));
    // The artifact line follows each phase.
    assert!(out.contains("        artifact: text + position maps\n"));
    // The sub-phase section header.
    assert!(out.contains(
        "Sub-phases (named decision points; LATENT = no override surface today):\n"
    ));
    // Sub-phase rows carry decision/assertion/rewind/exposure lines.
    assert!(out.contains("        decision: "));
    assert!(out.contains("   rewind: "));
    assert!(out.contains("        exposure: "));
}

#[test]
fn stage_list_marks_strength_and_latent() {
    let out = run_one("phase list");
    // At least one HARD assertion and the LATENT marker appear in the catalog.
    assert!(out.contains(" (HARD)"), "out: {out:?}");
    assert!(out.contains("  (LATENT)"), "out: {out:?}");
}

// ---------------------------------------------------------------------------
// `phase map` — group/surface/sub-phase resolution.
// ---------------------------------------------------------------------------

#[test]
fn stage_map_no_arg_dumps_both_tables() {
    let out = run_one("phase map");
    assert!(out.contains(
        "Action/rule groups -> phase (dominant artifact; see docs/history/stage-model.md s15 for straddlers):\n"
    ));
    assert!(out.contains("Console surfaces -> phase:\n"));
}

#[test]
fn stage_map_resolves_a_known_substage() {
    // compareform's sub-phase is comparison-canonicalization (P3) in the registry.
    let out = run_one("phase map comparison-canonicalization");
    assert!(
        out.contains("sub-phase comparison-canonicalization -> P3 (Definition Web)"),
        "out: {out:?}"
    );
    assert!(out.contains("  decision: "), "out: {out:?}");
    assert!(out.contains("  rewind: "), "out: {out:?}");
    assert!(out.contains("  exposure: "), "out: {out:?}");
}

#[test]
fn stage_map_unknown_token_is_execution_error() {
    let out = run_one("phase map definitely-not-a-real-thing");
    assert!(
        out.contains(
            "Execution error: Unknown group/surface/sub-phase: definitely-not-a-real-thing"
        ),
        "out: {out:?}"
    );
}

// ---------------------------------------------------------------------------
// `phase catalog` — must reproduce the W4 byte-compatible emitter exactly.
// ---------------------------------------------------------------------------

#[test]
fn stage_catalog_full_matches_w4_emitter_byte_for_byte() {
    // No program loaded -> the static, no-`current` form (kuna_live_value is
    // never called). The command must equal the W4 emitter exactly.
    let out = run_one("phase catalog");
    let expected = emit_catalog_json(|_| None);
    assert_eq!(out, expected);
    // Spot-check the JSON framing the catalog parser depends on.
    assert!(out.starts_with("[\n  {\"option\": "), "out head: {:?}", &out[..40.min(out.len())]);
    assert!(out.ends_with("}\n]\n"), "out tail differs");
}

#[test]
fn stage_catalog_single_option_matches_w4_emitter() {
    let out = run_one("phase catalog returnpair");
    let expected = emit_catalog_json_one("returnpair", None).expect("returnpair is a settable");
    assert_eq!(out, expected);
    assert!(out.contains("\"option\": \"returnpair\""), "out: {out:?}");
    assert!(out.contains("\"destructive_as_default\": true"), "out: {out:?}");
}

#[test]
fn stage_catalog_unknown_option_is_execution_error() {
    let out = run_one("phase catalog nosuchoption");
    assert!(
        out.contains("Execution error: Unknown settable option: nosuchoption (try `phase catalog`)"),
        "out: {out:?}"
    );
}

// ---------------------------------------------------------------------------
// `kassert` — validation + routing core (engine-independent).
// ---------------------------------------------------------------------------

#[test]
fn kassert_without_image_is_no_load_image_present() {
    // dcp->conf is null with no program loaded.
    let out = run_one("kassert P3 comparison-canonicalization canonical");
    assert!(out.contains("Execution error: No load image present"), "out: {out:?}");
}

#[test]
fn kassert_list_with_no_image_still_guards_on_image() {
    // C++ checks dcp->conf BEFORE the `list` branch, so `kassert list` with no
    // program is "No load image present", not an empty list.
    let out = run_one("kassert list");
    assert!(out.contains("Execution error: No load image present"), "out: {out:?}");
}

// ---------------------------------------------------------------------------
// Engine-dependent commands: resolve & dispatch (never "Invalid command").
// ---------------------------------------------------------------------------

#[test]
fn stage_status_without_image_is_no_load_image_present() {
    let out = run_one("phase status");
    assert!(out.contains("Execution error: No load image present"), "out: {out:?}");
}

#[test]
fn restarts_without_function_is_no_function_selected() {
    let out = run_one("restarts");
    assert!(out.contains("Execution error: No function selected"), "out: {out:?}");
}

#[test]
fn quality_without_function_is_no_function_selected() {
    let out = run_one("quality");
    assert!(out.contains("Execution error: No function selected"), "out: {out:?}");
}

#[test]
fn pipeline_list_prints_the_variants_with_no_program() {
    // `pipeline list` is fully expressible with no program (no `(current)` mark).
    let out = run_one("pipeline list");
    assert!(out.contains(
        "Named pipeline variants (group filters over the universal action; P0 pipeline-variant sub-phase):\n"
    ), "out: {out:?}");
    for v in ["decompile", "jumptable", "normalize", "paramid", "register", "firstpass"] {
        assert!(out.contains(&format!("  {v}\n")), "missing variant {v}: {out:?}");
    }
    // No program loaded -> no `(current)` marker.
    assert!(!out.contains("(current)"), "out: {out:?}");
}

#[test]
fn pipeline_bare_word_lists_like_list() {
    // C++: name.empty() || name=="list" both list. A bare `pipeline` lists.
    let out = run_one("pipeline");
    assert!(out.contains("Named pipeline variants"), "out: {out:?}");
}

#[test]
fn pipeline_unknown_variant_is_parse_error() {
    let out = run_one("pipeline bogusvariant");
    assert!(
        out.contains("Command parsing error: Unknown pipeline variant: bogusvariant (try `pipeline list`)"),
        "out: {out:?}"
    );
}

#[test]
fn region_commands_without_function_are_no_function_selected() {
    for cmd in ["region tree", "region blocks", "region walk"] {
        let out = run_one(cmd);
        assert!(
            out.contains("Execution error: No function selected"),
            "{cmd} out: {out:?}"
        );
    }
}

// NOTE: `kuna_live_value` (the `kunaLiveValue` port) reads a live
// `&Architecture`'s option flags; constructing a real `Architecture` requires a
// loaded `Sleigh` spec (impractical in a console unit test, and no console test
// builds one). Its `current`-field contribution to `phase catalog` is exercised
// at the integration level by `tests/stages/kuna-catalog.xml` (which loads a
// real program and string-matches the `current` values); the function itself is
// a direct field-read transcription of the C++ `kunaLiveValue` switch.

// ===========================================================================
// VERIFIER adversarial tests (w9-con-kuna-console, round 1).
//
// Targeting the spots the hunt list flagged as most fragile for this item:
// the multi-token `phase map` join loop (which carries the datatest
// KUNA-CONSOLE #5 contract `stage map force goto`), the `kassert` guard
// precedence (image check BEFORE tokenizing), and the `phase catalog`
// single-option live-value join vs the no-program path (exact bytes the
// `kuna.catalog` parser + `tests/stages/kuna-catalog.xml` depend on).
// ===========================================================================

/// `stage map force goto` — the C++ token-join loop
/// (`while(!s.eof()){ s>>word>>ws; if(empty)break; if(!token.empty())token+=' ';
/// token+=word; }`) must rebuild the SPACE-JOINED surface key "force goto", not
/// resolve on the first token alone. This is the exact datatest KUNA-CONSOLE #5
/// surface (`surface "force goto" -> P7 (Region Hierarchy) sub-phase
/// edge-virtualization`); a join bug would silently drop the second word and
/// report "Unknown group/surface/sub-phase: force".
#[test]
fn w9_con_kuna_console_stage_map_joins_multiword_surface_key() {
    let out = run_one("phase map force goto");
    assert!(
        out.contains(
            "surface \"force goto\" -> P7 (Region Hierarchy) sub-phase edge-virtualization"
        ),
        "multi-word surface key join lost: {out:?}"
    );
    // It must NOT have resolved on the first token alone and errored on "force".
    assert!(
        !out.contains("Unknown group/surface/sub-phase: force"),
        "join collapsed to first token only: {out:?}"
    );
}

/// `stage map   force   goto  ` with irregular interior/trailing whitespace must
/// collapse to the same single-space "force goto" key. The C++ `>> word >> ws`
/// discards every run of whitespace; the Rust `read_token()` + `skip_ws()` pair
/// must too (a stray double-space in the rebuilt token would miss the table).
#[test]
fn w9_con_kuna_console_stage_map_collapses_irregular_whitespace() {
    let out = run_one("phase map   force   goto   ");
    assert!(
        out.contains("surface \"force goto\" -> P7"),
        "irregular whitespace not collapsed to single-space key: {out:?}"
    );
}

/// The `kassert` image guard is checked BEFORE any tokenizing in C++
/// (`if (dcp->conf==0) throw ...` is the first statement). So a `kassert` with a
/// trailing `hard`/`hint` and a *valid* stage/substage still reports
/// "No load image present" with no program — the strength-pop and
/// validate_assertion path must NOT run first. A reordering that validated
/// before the image guard would surface a different (or no) error.
#[test]
fn w9_con_kuna_console_kassert_image_guard_precedes_validation() {
    // Valid S3 substage + trailing `hint`: were validation to run first this
    // would route/parse; the image guard must short-circuit it.
    // Legacy S-code input alias driven on purpose.
    let out = run_one("kassert S3 comparison-canonicalization original hint");
    assert!(
        out.contains("Execution error: No load image present"),
        "image guard did not precede validation: {out:?}"
    );
    // And a *bogus* stage code likewise must hit the image guard first (not a
    // "Bad phase code" parse error), proving the ordering.
    let out2 = run_one("kassert ZZ nosuch arg");
    assert!(
        out2.contains("Execution error: No load image present"),
        "bogus-stage kassert bypassed the image guard: {out2:?}"
    );
    assert!(
        !out2.contains("Bad phase code"),
        "validation ran before the image guard: {out2:?}"
    );
}

/// `stage catalog <option>` with a leading run of whitespace must still extract
/// the option token (`s >> ws >> option`) and emit the single-row form that
/// byte-matches the W4 emitter — not the full-array form. A failure to skip the
/// leading whitespace would read an empty option and dump the whole catalog
/// (the `kuna.catalog --json` single-option probe would then break).
#[test]
fn w9_con_kuna_console_stage_catalog_single_option_skips_leading_ws() {
    let out = run_one("phase catalog    returnpair");
    let expected = emit_catalog_json_one("returnpair", None).expect("returnpair is a settable");
    assert_eq!(out, expected, "leading-ws single-option form drifted");
    // Single-row form: starts with the object brace, not the array bracket.
    assert!(out.starts_with("  {\"option\": \"returnpair\""), "not single-row: {out:?}");
    assert!(!out.starts_with('['), "emitted the full array instead of one row: {out:?}");
}

/// A bare `phase catalog` (no option, no program) must equal the full W4 emitter
/// array with the no-`current` closure — the load-bearing contract the
/// `kuna.catalog` parser greps. Pinned here independently of the existing test
/// to guard the empty-option branch selection (option.is_empty() -> full form).
#[test]
fn w9_con_kuna_console_stage_catalog_bare_is_full_array() {
    // Drives the deprecated `phase catalog` alias on purpose: alias smoke test.
    let out = run_one("stage catalog");
    assert_eq!(out, emit_catalog_json(|_| None));
    assert!(out.starts_with("[\n"), "full form must open with the JSON array: {out:?}");
}
