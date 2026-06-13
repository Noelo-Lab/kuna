//! Tests for the kuna stage-model console commands (W9, `kuna_console.rs`).
//!
//! The kuna registry (`kuna_decomp::kuna_stages`) is pure static data, so the
//! registry-only commands (`stage list`/`map`/`catalog`, the `kassert`
//! validation core) produce their exact bytes with no program loaded — that is
//! what these tests pin. The `stage catalog` test compares against the **W4
//! byte-compatible emitter** directly (`emit_catalog_json` /
//! `emit_catalog_json_one`), which `tests/stages/kuna-catalog.xml`
//! string-matches against the C++ binary, so reproducing the emitter output is
//! the load-bearing contract.
//!
//! The engine-dependent kuna commands (`stage status`, `restarts`, `pipeline`
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
use kuna_decomp::kuna_stages::{emit_catalog_json, emit_catalog_json_one};

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
fn registers_all_eleven_kuna_commands() {
    // register_decomp_commands registers 105 (see ifacedecomp/tests.rs); the
    // kuna capability adds 11: stage list/map/status/catalog, kassert, restarts,
    // pipeline, quality, region tree/blocks/walk.
    let only_kuna = {
        let mut st = ConsoleCommands::into_status(vec![]);
        register_kuna_commands(&mut st);
        st.num_commands()
    };
    assert_eq!(only_kuna, 11);
    assert_eq!(console(&[]).num_commands(), 105 + 11);
}

#[test]
fn kuna_command_prefixes_expand() {
    let mut status = console(&[]);
    // The abbreviations the kuna datatests / kuna.catalog drive.
    assert_eq!(status.resolve("stage list").unwrap(), vec!["stage", "list"]);
    assert_eq!(status.resolve("stage map").unwrap(), vec!["stage", "map"]);
    assert_eq!(status.resolve("stage status").unwrap(), vec!["stage", "status"]);
    assert_eq!(status.resolve("stage catalog").unwrap(), vec!["stage", "catalog"]);
    assert_eq!(status.resolve("stage cat").unwrap(), vec!["stage", "catalog"]);
    assert_eq!(status.resolve("kassert list").unwrap(), vec!["kassert"]);
    assert_eq!(status.resolve("pipeline list").unwrap(), vec!["pipeline"]);
    assert_eq!(status.resolve("quality").unwrap(), vec!["quality"]);
    assert_eq!(status.resolve("region tree").unwrap(), vec!["region", "tree"]);
    assert_eq!(status.resolve("region blocks").unwrap(), vec!["region", "blocks"]);
    assert_eq!(status.resolve("region walk").unwrap(), vec!["region", "walk"]);
    assert_eq!(status.resolve("restarts").unwrap(), vec!["restarts"]);
}

// ---------------------------------------------------------------------------
// `stage list` — pure static registry data.
// ---------------------------------------------------------------------------

#[test]
fn stage_list_prints_the_stage_model_header_and_all_stages() {
    let out = run_one("stage list");
    assert!(
        out.starts_with("Stages (kuna stage model, docs/stages.md / docs/stage-model.md):\n"),
        "out: {out:?}"
    );
    // P0 is the orthogonal plane; S3..S6 are Band B.
    assert!(out.contains("  P0  Knowledge & Configuration Plane  [orthogonal plane]\n"));
    assert!(out.contains("  S3  Definition Web  [Band B]\n"));
    assert!(out.contains("  S6  Variable & Storage Model  [Band B]\n"));
    // S1/S2/S7..S9 carry no bracket tag.
    assert!(out.contains("  S1  Image & Code Partition\n"));
    assert!(out.contains("  S9  Surface Rendering & Refinement\n"));
    // The artifact line follows each stage.
    assert!(out.contains("        artifact: text + position maps\n"));
    // The sub-stage section header.
    assert!(out.contains(
        "Sub-stages (named decision points; LATENT = no override surface today):\n"
    ));
    // Sub-stage rows carry decision/assertion/rewind/exposure lines.
    assert!(out.contains("        decision: "));
    assert!(out.contains("   rewind: "));
    assert!(out.contains("        exposure: "));
}

#[test]
fn stage_list_marks_strength_and_latent() {
    let out = run_one("stage list");
    // At least one HARD assertion and the LATENT marker appear in the catalog.
    assert!(out.contains(" (HARD)"), "out: {out:?}");
    assert!(out.contains("  (LATENT)"), "out: {out:?}");
}

// ---------------------------------------------------------------------------
// `stage map` — group/surface/sub-stage resolution.
// ---------------------------------------------------------------------------

#[test]
fn stage_map_no_arg_dumps_both_tables() {
    let out = run_one("stage map");
    assert!(out.contains(
        "Action/rule groups -> stage (dominant artifact; see stage-model.md s15 for straddlers):\n"
    ));
    assert!(out.contains("Console surfaces -> stage:\n"));
}

#[test]
fn stage_map_resolves_a_known_substage() {
    // compareform's sub-stage is comparison-canonicalization (S3) in the registry.
    let out = run_one("stage map comparison-canonicalization");
    assert!(
        out.contains("sub-stage comparison-canonicalization -> S3 (Definition Web)"),
        "out: {out:?}"
    );
    assert!(out.contains("  decision: "), "out: {out:?}");
    assert!(out.contains("  rewind: "), "out: {out:?}");
    assert!(out.contains("  exposure: "), "out: {out:?}");
}

#[test]
fn stage_map_unknown_token_is_execution_error() {
    let out = run_one("stage map definitely-not-a-real-thing");
    assert!(
        out.contains(
            "Execution error: Unknown group/surface/sub-stage: definitely-not-a-real-thing"
        ),
        "out: {out:?}"
    );
}

// ---------------------------------------------------------------------------
// `stage catalog` — must reproduce the W4 byte-compatible emitter exactly.
// ---------------------------------------------------------------------------

#[test]
fn stage_catalog_full_matches_w4_emitter_byte_for_byte() {
    // No program loaded -> the static, no-`current` form (kuna_live_value is
    // never called). The command must equal the W4 emitter exactly.
    let out = run_one("stage catalog");
    let expected = emit_catalog_json(|_| None);
    assert_eq!(out, expected);
    // Spot-check the JSON framing the catalog parser depends on.
    assert!(out.starts_with("[\n  {\"option\": "), "out head: {:?}", &out[..40.min(out.len())]);
    assert!(out.ends_with("}\n]\n"), "out tail differs");
}

#[test]
fn stage_catalog_single_option_matches_w4_emitter() {
    let out = run_one("stage catalog returnpair");
    let expected = emit_catalog_json_one("returnpair", None).expect("returnpair is a settable");
    assert_eq!(out, expected);
    assert!(out.contains("\"option\": \"returnpair\""), "out: {out:?}");
    assert!(out.contains("\"destructive_as_default\": true"), "out: {out:?}");
}

#[test]
fn stage_catalog_unknown_option_is_execution_error() {
    let out = run_one("stage catalog nosuchoption");
    assert!(
        out.contains("Execution error: Unknown settable option: nosuchoption (try `stage catalog`)"),
        "out: {out:?}"
    );
}

// ---------------------------------------------------------------------------
// `kassert` — validation + routing core (engine-independent).
// ---------------------------------------------------------------------------

#[test]
fn kassert_without_image_is_no_load_image_present() {
    // dcp->conf is null with no program loaded.
    let out = run_one("kassert S3 comparison-canonicalization canonical");
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
    let out = run_one("stage status");
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
        "Named pipeline variants (group filters over the universal action; P0 pipeline-variant sub-stage):\n"
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
// builds one). Its `current`-field contribution to `stage catalog` is exercised
// at the integration level by `tests/stages/kuna-catalog.xml` (which loads a
// real program and string-matches the `current` values); the function itself is
// a direct field-read transcription of the C++ `kunaLiveValue` switch.
