//! CLI end-to-end gate for `kuna decompile-all` / `kuna functions` — drives the
//! built `kuna` binary over the real vendored `fauxware` ELF and asserts the
//! machine-readable JSON surface decbench and an LLM driver consume.
//!
//! Integration tests require the built processor specs under `specs/`.

mod common;

use common::process;

#[cfg(unix)]
#[path = "common/compiler_probe_tests.rs"]
mod compiler_probe_tests;

use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn fauxware() -> String {
    repo_root()
        .join("decompiler/crates/kuna-analysis/tests/fixtures/fauxware")
        .to_str()
        .unwrap()
        .to_string()
}

/// The checked-in stripped-ELF hang repro (`tests/hang-repro/README.md`): a
/// fully-stripped x86-64 openssh `ssh-sk-helper` whose `sub_1bd04` @ 0x1bd04
/// never converges in the decompile pipeline (the `--max-fn-seconds` watchdog's
/// raison d'être).
fn hang_repro() -> String {
    repo_root().join("tests/hang-repro/ssh-sk-helper").to_str().unwrap().to_string()
}

fn specs() -> String {
    repo_root().join("specs").to_str().unwrap().to_string()
}

/// A small **ARM 32-bit** (Thumb) ELF fixture — the non-x86-64 discovery surface
/// where `decompile-all` defaults `funcstart_patterns` ON (DIV-20).
fn arm_thumb() -> String {
    repo_root()
        .join("decompiler/crates/kuna-analysis/tests/fixtures/arm_thumb_linked_le32")
        .to_str()
        .unwrap()
        .to_string()
}

fn arm_thumb_pe() -> String {
    repo_root()
        .join("decompiler/crates/kuna-analysis/tests/fixtures/armv4t_thumb_pe.exe")
        .to_str()
        .unwrap()
        .to_string()
}

fn dialog_callbacks_pe() -> String {
    repo_root()
        .join("decompiler/crates/kuna-analysis/tests/fixtures/stdcallpop_pe_i386.exe")
        .to_str()
        .unwrap()
        .to_string()
}

/// An executable callback pointer passed to DialogBoxParamA must seed the dialog
/// procedure, and rebuilding from that procedure must discover its nested dialog
/// callback in the same run.
#[test]
fn inventory_and_decompile_all_include_nested_dialog_callbacks() {
    let bin = dialog_callbacks_pe();
    let sp = specs();
    let (inventory, stderr, ok) =
        run_kuna(&["functions", &bin, "--json", "--sleighpath", &sp]);
    assert!(ok, "kuna functions failed: {stderr}");
    assert!(
        inventory.contains("\"address_hex\": \"0x401000\"")
            && inventory.contains("\"address_hex\": \"0x401410\""),
        "nested dialog callbacks are absent from the inventory: {inventory}"
    );
    let parent = inventory
        .find("\"address_hex\": \"0x4013e0\"")
        .map(|start| &inventory[start..inventory.len().min(start + 240)])
        .expect("parent function 0x4013e0 is absent");
    assert!(
        (41..=48).any(|size| parent.contains(&format!("\"size\": {size}"))),
        "the parent extent still covers its callback: {parent}"
    );

    let (without, stderr, ok) = run_kuna(&[
        "functions",
        &bin,
        "--json",
        "--sleighpath",
        &sp,
        "--option",
        "fast_funcdisc",
        "off",
    ]);
    assert!(ok, "kuna functions with fast_funcdisc off failed: {stderr}");
    assert!(
        !without.contains("\"address_hex\": \"0x401000\"")
            && !without.contains("\"address_hex\": \"0x401410\""),
        "fast_funcdisc off no longer restores the prior inventory: {without}"
    );

    let (whole, stderr, ok) =
        run_kuna(&["decompile-all", &bin, "--json", "--sleighpath", &sp]);
    assert!(ok, "kuna decompile-all failed: {stderr}");
    for (address, body) in [("0x401000", "sub_401000"), ("0x401410", "sub_401410")] {
        assert!(
            whole.contains(&format!("\"address_hex\": \"{address}\""))
                && whole.contains(body),
            "decompile-all omitted callback {address}: {whole}"
        );
    }
}

fn write_thumb_te() -> PathBuf {
    let bytes = kuna_analysis::loadimage_te::synthetic::TeImage::thumb(&[0x07, 0x20, 0x70, 0x47]).build();
    let path = common::scratch_file("cli-thumb", "te");
    std::fs::write(&path, bytes).unwrap();
    path
}

/// A larger **ARM 32-bit** ELF fixture with functions the prologue-`<patternpairs>`
/// matcher genuinely finds and the entry oracles do not (`0x3e0`, `0x410`,
/// `0x3c520`) — the fixture the DIV-20 `funcstart_patterns` assertion needs.
///
/// `arm_thumb()` cannot serve that role: it holds exactly two functions, both
/// already named by `.symtab`, so `funcstart_patterns` adds no real entry there.
/// Before issue #197 the assertion appeared to pass on it only because the pass's
/// extra "discoveries" were duplicate records for those same two functions (a
/// `sub_<addr>` alias plus an odd-address Thumb `entry|1` phantom).
fn arm_entrymain() -> String {
    repo_root()
        .join("decompiler/crates/kuna-analysis/tests/fixtures/entrymain_arm")
        .to_str()
        .unwrap()
        .to_string()
}

/// A C++ ELF whose `main` calls a **namespaced** member, `foo::Bar::baz`. A name
/// is installed into the scope its `::` path names, so this is the fixture where
/// a second function symbol at one address lands in a different scope from the
/// first and the across-scopes display lookup starts answering with the wrong
/// one.
fn cpp_mangled() -> String {
    repo_root()
        .join("decompiler/crates/kuna-analysis/tests/fixtures/cpp_mangled_x86_64")
        .to_str()
        .unwrap()
        .to_string()
}

fn json_count(stdout: &str) -> Option<usize> {
    let document: serde_json::Value = serde_json::from_str(stdout).expect("valid CLI JSON");
    document
        .get("count")
        .and_then(serde_json::Value::as_u64)
        .and_then(|value| usize::try_from(value).ok())
}

fn json_addresses(stdout: &str) -> Vec<u64> {
    let document: serde_json::Value = serde_json::from_str(stdout).expect("valid CLI JSON");
    document
        .get("functions")
        .and_then(serde_json::Value::as_array)
        .expect("function array")
        .iter()
        .map(|function| {
            function.get("address").and_then(serde_json::Value::as_u64)
                .expect("numeric function address")
        })
        .collect()
}

#[test]
fn json_helpers_use_function_fields_and_preserve_raw_records() {
    let document = r#"{"metadata":{"count":99,"address":99,"size":99},"count":2,"functions":[
  {"name":"a\"b","address":1,"size":4,"variables":[{"size":32}]},
  {"name":"second","address":2,"size":8}
]}"#;
    assert_eq!(json_count(document), Some(2));
    assert_eq!(json_addresses(document), [1, 2]);
    assert_eq!(json_sizes(document), [4, 8]);
    let records = json_records(document);
    assert_eq!(records.len(), 2);
    assert_eq!(record_name(records[0]), "a\"b");
    assert_eq!(
        records[0],
        r#"{"name":"a\"b","address":1,"size":4,"variables":[{"size":32}]}"#
    );
}

#[test]
fn json_helpers_reject_malformed_documents() {
    for document in [r#"{"count":2garbage}"#, r#"{"count":2} trailing"#] {
        assert!(std::panic::catch_unwind(|| json_count(document)).is_err());
    }
}

/// Run `kuna <cmd> <bin> --mode reliable --json` and return its entry addresses.
fn run_json_addrs(cmd: &str, bin: &str, sp: &str, extra: &[&str]) -> Vec<u64> {
    let mut args = vec![cmd, bin, "--json", "--sleighpath", sp, "--mode", "reliable"];
    args.extend_from_slice(extra);
    let (stdout, stderr, ok) = run_kuna(&args);
    if !ok {
        panic!("kuna {cmd} failed on {bin}: {stderr}");
    }
    json_addresses(&stdout)
}

/// The `error(nonzero,…)` boundary-overrun fixture (`noreturn_error_x86_64`):
/// `err_fatal.constprop.0` @ 0x4011c0 ends in `call error(2,…)` and is immediately
/// followed by `compute` @ 0x4011f0.
fn noreturn_error_fixture() -> String {
    repo_root()
        .join("decompiler/crates/kuna-analysis/tests/fixtures/noreturn_error_x86_64")
        .to_str()
        .unwrap()
        .to_string()
}

/// The decode-lane knobs (`kuna_pdecode`). Cleared from every child here so a
/// suite run under `KUNA_DECODE_JOBS=8 KUNA_DECODE_MIN_BYTES=0` -- which is how
/// DIV-169 says to exercise the gates -- cannot move a baseline these tests take
/// for granted; each test then sets only what it means to.
const DECODE_ENV: [&str; 6] = [
    "KUNA_DECODE_JOBS",
    "KUNA_DECODE_MIN_BYTES",
    "KUNA_DECODE_INTERVALS",
    "KUNA_DECODE_SELFCHECK",
    "KUNA_DECODE_STATS",
    "KUNA_DECODE_FAULT",
];

/// A `kuna` invocation with this suite's environment hygiene, plus `env`.
fn kuna_command(env: &[(&str, &str)]) -> Command {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_kuna"));
    cmd.env_remove("KUNA_DECOMP_DBG").env_remove("KUNA_DECOMP_TEST").env_remove("KUNA_SLACOMP");
    // A lane fault reports itself in one line; `RUST_BACKTRACE` deliberately
    // turns the runtime's panic block back on, so it must not reach a child
    // whose whole assertion is that the block is absent.
    cmd.env_remove("RUST_BACKTRACE");
    cmd.env_remove("KUNA_JOBS_FAULT");
    for name in DECODE_ENV {
        cmd.env_remove(name);
    }
    for (name, value) in env {
        cmd.env(name, value);
    }
    cmd
}

/// Run the built `kuna` binary, returning `(stdout, stderr, success)`.
fn run_kuna(args: &[&str]) -> (String, String, bool) {
    run_kuna_env(args, &[])
}

/// Run the built `kuna` binary with a hard outer wall-clock `cap`, returning
/// `Some((stdout, stderr, success))` if it exited in time, `None` if it had to
/// be killed.  The outer cap is the regression guard for the watchdog itself:
/// without `--max-fn-seconds` the hang-repro invocation would spin forever.
fn run_kuna_with_timeout(args: &[&str], cap: Duration) -> Option<(String, String, bool)> {
    run_kuna_env_with_timeout(args, &[], cap)
}

/// [`run_kuna_with_timeout`] with extra environment. Every fault-injecting
/// invocation goes through this: the failure the fallback exists to prevent is a
/// deadlock, and a wedged test binary reports nothing at all.
fn run_kuna_env_with_timeout(
    args: &[&str],
    env: &[(&str, &str)],
    cap: Duration,
) -> Option<(String, String, bool)> {
    process::output_with_timeout(kuna_command(env).args(args), cap, Duration::from_millis(200))
        .map(|output| {
            (
                String::from_utf8_lossy(&output.stdout).into_owned(),
                String::from_utf8_lossy(&output.stderr).into_owned(),
                output.status.success(),
            )
        })
}

/// A filtered whole-binary run is still a body-lifting surface. Selecting a
/// mapped IAT word must fail before it can become a result row.
#[test]
fn decompile_all_refuses_an_executable_section_iat_slot() {
    let bin = repo_root()
        .join("decompiler/crates/kuna-analysis/tests/fixtures/pe_iatincode_i386.exe")
        .to_str()
        .unwrap()
        .to_string();
    let (stdout, stderr, ok) = run_kuna(&[
        "decompile-all",
        &bin,
        "--addr",
        "0x401000",
        "--json",
        "--sleighpath",
        &specs(),
    ]);
    assert!(!ok, "an IAT slot unexpectedly decompiled: {stdout}");
    assert!(stdout.trim().is_empty(), "an IAT result row escaped: {stdout}");
    assert_eq!(
        stderr,
        "error: selector \"0x401000\" identifies import VirtualAlloc at 0x401000; \
         the IAT slot contains a loader-written pointer, not a function body\n"
    );
}

#[test]
fn decompile_all_emits_json_for_main() {
    let bin = fauxware();
    let (stdout, stderr, ok) = run_kuna(&[
        "decompile-all",
        &bin,
        "--functions",
        "main,authenticate",
        "--json",
        "--sleighpath",
        &specs(),
    ]);
    if !ok {
        panic!("kuna decompile-all failed: {stderr}");
    }
    // Shape assertions (no JSON dep): two functions, both with non-null code.
    assert!(stdout.trim_start().starts_with('{'), "output is not a JSON object:\n{stdout}");
    assert!(stdout.contains("\"count\": 2"), "expected count 2:\n{stdout}");
    assert!(stdout.contains("\"name\": \"main\""), "missing function `main`:\n{stdout}");
    assert!(stdout.contains("\"name\": \"authenticate\""), "missing `authenticate`:\n{stdout}");
    assert!(stdout.contains("\"variables\""), "missing variables array:\n{stdout}");
    assert!(stdout.contains("\"line_mappings\""), "missing line mappings:\n{stdout}");
    assert!(stdout.contains("\"line_number\":"), "line mappings are empty:\n{stdout}");
    assert!(stdout.contains("\"line_numbers\""), "missing variable line evidence:\n{stdout}");
    assert!(stdout.contains("\"addresses\""), "missing provenance addresses:\n{stdout}");
    let has_variable_lines = stdout.match_indices("\"line_numbers\": [").any(|(i, key)| {
        stdout[i + key.len()..]
            .trim_start()
            .chars()
            .next()
            .is_some_and(|c| c.is_ascii_digit())
    });
    assert!(has_variable_lines, "all variable-use mappings are empty:\n{stdout}");
    let authenticate = stdout
        .split("\"name\": \"authenticate\"")
        .nth(1)
        .expect("authenticate result must be present");
    let array_local = authenticate
        .split("\"name\": \"v2\"")
        .nth(1)
        .expect("authenticate must report its recovered array local");
    let array_lines = array_local
        .split("\"line_numbers\":")
        .nth(1)
        .expect("the array local must carry the additive provenance field");
    assert!(
        !array_lines.trim_start().starts_with("[]"),
        "the fragmented array-local varrefs must retain use evidence:\n{stdout}"
    );
    // `authenticate(const char *, const char *)` ⇒ a parameter with arg_index 0.
    assert!(
        stdout.contains("\"kind\": \"arg\"") && stdout.contains("\"arg_index\": 0"),
        "expected a parameter with arg_index 0:\n{stdout}"
    );
}

#[test]
fn fast_mode_matches_explicit_options_and_user_override_wins() {
    let bin = arm_entrymain();
    let sp = specs();
    let run = |extra: &[&str]| -> String {
        let mut args =
            vec!["decompile-all", bin.as_str(), "--json", "--no-vars", "--sleighpath", sp.as_str()];
        args.extend_from_slice(extra);
        let (stdout, stderr, ok) = run_kuna(&args);
        if !ok {
            panic!("kuna decompile-all failed on the ARM fixture: {stderr}");
        }
        stdout
    };

    let fast = run(&["--mode", "fast"]);
    let explicit = run(&[
        "--mode",
        "reliable",
        "--option",
        "listing",
        "off",
        "--option",
        "funcstart_patterns",
        "off",
        "--option",
        "aif",
        "off",
        "--option",
        "fast_funcdisc",
        "on",
    ]);
    assert_eq!(fast, explicit, "fast must equal its four explicit option overrides");

    let noreturn = noreturn_fixture();
    let base = [
        "decompile-all",
        noreturn.as_str(),
        "--functions",
        "compute",
        "--json",
        "--no-vars",
        "--sleighpath",
        sp.as_str(),
    ];
    let mut fast_args = base.to_vec();
    fast_args.extend_from_slice(&["--mode", "fast"]);
    let (fast_out, stderr, ok) = run_kuna(&fast_args);
    assert!(ok, "fast no-return control failed: {stderr}");
    assert!(
        // (kuna DIV-39) the no-return warning renders as the `// no-return`
        // slug under the default inline warnstyle.
        !code_field(&fast_out).contains("// no-return"),
        "fast must keep the Listing/no-return consumer disabled"
    );

    let mut restored_args = base.to_vec();
    restored_args.extend_from_slice(&["--mode", "fast", "--option", "listing", "on"]);
    let (restored_out, stderr, ok) = run_kuna(&restored_args);
    assert!(ok, "fast with Listing restored failed: {stderr}");
    assert!(
        code_field(&restored_out).contains("// no-return"),
        "an explicit option after fast must win with last-write precedence"
    );
}

#[test]
fn modes_command_lists_auto_policy_and_fast_preset() {
    let (stdout, stderr, ok) = run_kuna(&["modes", "--json"]);
    assert!(ok, "kuna modes failed: {stderr}");
    let auto = stdout
        .split("\"name\": \"auto\"")
        .nth(1)
        .expect("modes JSON must list auto after its name");
    assert!(
        auto.contains("\"automatic\": true"),
        "auto mode JSON must identify a dynamic policy: {stdout}"
    );
    let fast = stdout
        .split("\"name\": \"fast\"")
        .nth(1)
        .expect("modes JSON must list fast after its name");
    for option in ["listing", "funcstart_patterns", "aif", "fast_funcdisc"] {
        assert!(
            fast.contains(&format!("\"option\": \"{option}\"")),
            "fast mode JSON missing {option}: {stdout}"
        );
    }
}

#[test]
fn omitted_and_explicit_auto_match_aggressive_on_a_small_binary() {
    let bin = fauxware();
    let sp = specs();
    let run = |mode: Option<&str>| -> String {
        let mut args = vec![
            "functions",
            bin.as_str(),
            "--json",
            "--sleighpath",
            sp.as_str(),
        ];
        if let Some(mode) = mode {
            args.extend_from_slice(&["--mode", mode]);
        }
        let (stdout, stderr, ok) = run_kuna(&args);
        if !ok {
            panic!("kuna functions failed for mode {mode:?}: {stderr}");
        }
        stdout
    };

    let omitted = run(None);
    assert_eq!(omitted, run(Some("auto")));
    assert_eq!(omitted, run(Some("aggressive")));
}

#[test]
fn decompile_mode_requires_a_value() {
    let bin = fauxware();
    let (_stdout, stderr, ok) = run_kuna(&["decompile", bin.as_str(), "main", "--mode"]);
    assert!(!ok, "missing --mode value must fail");
    assert!(stderr.contains("--mode requires a value"), "unexpected error: {stderr}");
}

/// DIV-20: in `reliable` mode on a **non-x86-64** binary, `decompile-all` defaults
/// `funcstart_patterns` ON — the primary function-discovery source when oracle 5
/// (the x86-64-only prologue scan) does not apply. Without it a stripped ARM binary
/// discovers only the ELF entry; with it the prologue `<patternpairs>` matcher finds
/// more. The reliable driver fallback must match an explicit
/// `--option funcstart_patterns on` and beat `off`.
///
/// Runs on `entrymain_arm`, where the pass finds three functions nothing else does
/// (`0x3e0`, `0x410`, `0x3c520`): 10 entries `off` vs 12 by default — 13 canonical,
/// of which `0x3c520` falls outside every CODE section and so is listed by `kuna
/// functions` but not decompiled. It used to run
/// on the two-function `arm_thumb()` fixture, where the "extra" entries the
/// assertion counted were in fact duplicate records for functions already found —
/// so the fixture swap is what keeps this assertion meaningful once issue #197
/// stops the enumeration reporting one function more than once.
#[test]
fn arm_decompile_all_defaults_funcstart_patterns_on() {
    let bin = arm_entrymain();
    let sp = specs();
    let run = |extra: &[&str]| -> usize {
        let mut args = vec![
            "decompile-all", bin.as_str(), "--json", "--sleighpath", sp.as_str(),
            "--mode", "reliable",
        ];
        args.extend_from_slice(extra);
        let (stdout, stderr, ok) = run_kuna(&args);
        if !ok {
            panic!("kuna decompile-all failed on the ARM fixture: {stderr}");
        }
        json_count(&stdout).expect("count in json")
    };
    let default_cnt = run(&[]);
    let off_cnt = run(&["--option", "funcstart_patterns", "off"]);
    let on_cnt = run(&["--option", "funcstart_patterns", "on"]);
    // The non-x86-64 default injects the pass: it discovers strictly more than `off`,
    // and matches the explicit `on`.
    assert!(
        default_cnt > off_cnt,
        "ARM decompile-all default should discover MORE than funcstart_patterns off \
         (default={default_cnt}, off={off_cnt}) — the DIV-20 injection did not fire"
    );
    assert_eq!(
        default_cnt, on_cnt,
        "ARM default must equal explicit `funcstart_patterns on` (default={default_cnt}, on={on_cnt})"
    );
}

/// `decompile-all --mode reliable` on a non-x86-64 binary ALSO defaults the Aggressive Instruction
/// Finder (`aif`) ON — the gap-walk that seeds the disconnected call-graph components
/// (functions reached only via indirect calls / function-pointer tables, preceded by
/// data/literal-pools so the `funcstart_patterns` `<patternpairs>` epilogue-prepattern
/// never matches) that the prologue matcher + recursive-descent walk structurally miss
/// (crazyflie cf2.elf 1430 -> 2700 functions, 45% -> 82% of angr's set).  This small
/// fixture is too sparse for AIF's prologue-fingerprint histogram (`FINGERPRINT_THRESHOLD`)
/// to add anything — the coverage win is on real firmware, verified on the decbench ARM
/// projects — so here we assert the injection is WIRED and NON-DESTRUCTIVE: the default
/// path equals an explicit `--option aif on` and never discovers fewer than `aif off`.
#[test]
fn arm_decompile_all_defaults_aif_on() {
    let bin = arm_thumb();
    let sp = specs();
    let run = |extra: &[&str]| -> usize {
        let mut args = vec![
            "decompile-all", bin.as_str(), "--json", "--sleighpath", sp.as_str(),
            "--mode", "reliable",
        ];
        args.extend_from_slice(extra);
        let (stdout, stderr, ok) = run_kuna(&args);
        if !ok {
            panic!("kuna decompile-all failed on the ARM fixture: {stderr}");
        }
        json_count(&stdout).expect("count in json")
    };
    let default_cnt = run(&[]);
    let off_cnt = run(&["--option", "aif", "off"]);
    let on_cnt = run(&["--option", "aif", "on"]);
    assert_eq!(
        default_cnt, on_cnt,
        "ARM default must equal explicit `aif on` (default={default_cnt}, on={on_cnt}) — the injection did not fire"
    );
    assert!(
        default_cnt >= off_cnt,
        "AIF must never discover FEWER than off (default={default_cnt}, off={off_cnt})"
    );
}

/// Stage 2 (angr-parity ARM discovery): reliable `decompile-all` on a non-x86-64 binary also
/// runs the **raw, UNPAIRED Thumb-prologue** gap seed
/// (`aif::raw_thumb_prologue_seeds`, the mirror of angr `CFGFast`'s
/// `_func_addrs_from_prologues()` over `ArchARMCortexM.thumb_prologs`). It scans for
/// canonical LR-saving Thumb prologues (`PUSH {..,lr}` `0xB5xx` / `PUSH.W {..,lr}`
/// `0xE92D..`) that fell in an UNDEFINED gap (never `<patternpairs>` epilogue-paired,
/// never reached by a direct BL, and skipped by AIF's cursor-advancing gap-walk),
/// validates each with `check_valid_subroutine`, and re-seeds the recursive-descent
/// walk with the survivors. It is folded into the existing `funcstart_patterns`
/// (`analysis_funcstart_patterns`) discovery gate — no new stage-model option — so
/// there is nothing extra to toggle here.
///
/// This tiny fixture has no dense literal-pool-separated prologue clusters, so the
/// raw scan adds nothing on it (the coverage win is on real Cortex-M firmware:
/// betaflight STM32F405 recovers the ~483 PUSH-prologue functions the
/// `<patternpairs>` matcher structurally misses, crazyflie 82% -> ~95% of angr's
/// discovered set — verified in the decbench ARM parity harness). Here we assert the
/// wiring is NON-DESTRUCTIVE: the default path (raw seed active) still succeeds and
/// never discovers FEWER functions than `funcstart_patterns off` (which disables the
/// whole recursive-discovery tier, raw seed included), and turning the gate off does
/// not error.
#[test]
fn arm_decompile_all_raw_thumb_prologue_seed_non_destructive() {
    let bin = arm_thumb();
    let sp = specs();
    let run = |extra: &[&str]| -> usize {
        let mut args = vec![
            "decompile-all", bin.as_str(), "--json", "--sleighpath", sp.as_str(),
            "--mode", "reliable",
        ];
        args.extend_from_slice(extra);
        let (stdout, stderr, ok) = run_kuna(&args);
        if !ok {
            panic!("kuna decompile-all failed on the ARM fixture: {stderr}");
        }
        json_count(&stdout).expect("count in json")
    };
    let default_cnt = run(&[]);
    // `funcstart_patterns off` disables the whole recursive-discovery tier (the raw
    // Thumb-prologue seed is gated on the same flag), so the default (with the raw
    // seed active) must never discover fewer.
    let off_cnt = run(&["--option", "funcstart_patterns", "off"]);
    assert!(
        default_cnt >= off_cnt,
        "raw Thumb-prologue seed must never discover FEWER than funcstart_patterns off \
         (default={default_cnt}, off={off_cnt})"
    );
}

/// DIV-68: `kuna functions` takes the same discovery defaults as `kuna decompile-all`,
/// so the inventory can never omit an entry the whole-binary run decompiles.
///
/// `decompile-all` reports the CODE-backed SUBSET of the canonical inventory
/// `functions` prints, so every address the former decompiles must appear in the
/// latter.  Under `--mode reliable` on a non-x86-64 binary that invariant used to be
/// inverted: the DIV-20 `funcstart_patterns`/`aif` defaults (and the Listing that
/// gates them) were bundled behind the same flag as the DIV-15 Listing default, which
/// `functions` deliberately declined — so `entrymain_arm` listed 10 entries while
/// `decompile-all` decompiled 12, `0x3e0` and `0x410` among them.  On real firmware
/// the same hole read as `1` of `5,797` (stripped betaflight STM32F405).
#[test]
fn arm_functions_inventory_covers_every_decompile_all_entry() {
    let bin = arm_entrymain();
    let sp = specs();
    let inventory = run_json_addrs("functions", &bin, &sp, &[]);
    let decompiled =
        run_json_addrs("decompile-all", &bin, &sp, &["--no-vars"]);

    let missing: Vec<u64> =
        decompiled.iter().copied().filter(|a| !inventory.contains(a)).collect();
    assert!(
        missing.is_empty(),
        "`kuna functions` must list every entry `decompile-all` decompiles; missing {:x?} \
         (inventory={}, decompiled={})",
        missing,
        inventory.len(),
        decompiled.len()
    );
    // The two entries only the prologue matcher finds — the concrete pre-fix miss.
    for want in [0x3e0u64, 0x410] {
        assert!(
            inventory.contains(&want),
            "the ARM inventory is missing the funcstart_patterns discovery 0x{want:x}: {:x?}",
            inventory
        );
    }
    // The injection fired: the default inventory equals the explicit bundle.
    let explicit = run_json_addrs(
        "functions",
        &bin,
        &sp,
        &["--option", "listing", "on", "--option", "funcstart_patterns", "on", "--option",
          "aif", "on"],
    );
    assert_eq!(
        inventory, explicit,
        "the non-x86-64 `functions` default must equal the explicit discovery bundle"
    );
}

/// DIV-68, the other side: x86-64 enumeration is untouched.
///
/// The discovery bundle is non-x86-64-only and the Listing is measured entry-neutral
/// on x86-64, so `kuna functions` there must still inject nothing — same inventory as
/// an explicit `listing off`, and still a superset of what `decompile-all` decompiles.
#[test]
fn x86_64_functions_inventory_is_unchanged_and_covers_decompile_all() {
    let bin = fauxware();
    let sp = specs();
    let inventory = run_json_addrs("functions", &bin, &sp, &[]);
    let no_listing =
        run_json_addrs("functions", &bin, &sp, &["--option", "listing", "off"]);
    assert_eq!(
        inventory, no_listing,
        "x86-64 `kuna functions` must not build the Listing — the DIV-15 default is the \
         decompiling surfaces'"
    );
    let decompiled =
        run_json_addrs("decompile-all", &bin, &sp, &["--no-vars"]);
    let missing: Vec<u64> =
        decompiled.iter().copied().filter(|a| !inventory.contains(a)).collect();
    assert!(
        missing.is_empty(),
        "`kuna functions` must list every entry `decompile-all` decompiles; missing {missing:x?}"
    );
}

/// Issue #197: a whole-binary run reports each function ENTRY exactly once.
///
/// `arm_thumb_linked_le32` holds exactly two functions (`compute` @ 0x100b8 and
/// `_start` @ 0x100d6 — see the fixture's `.c`), but `decompile-all` used to emit
/// **six** records for them: one per name the entry carried (`compute` +
/// `sub_100b8`), plus one per ARM Thumb `entry|1` twin (`sub_100b9`, `sub_100d7`)
/// — the ELF `.symtab` stores these functions at the ODD `st_value` 0x100b9 /
/// 0x100d7, the mode bit, and the unmasked value was being seeded as a function
/// start.  The odd twins are not merely redundant: 0x100b9 is not an instruction
/// boundary, so it decompiled to a bogus empty `void sub_100b9(void)`.
///
/// Asserts the canonical shape: two entries, at the two even addresses, named by
/// their real symbols, with the generated `sub_<addr>` name kept in `aliases` (so
/// nothing that could be looked up before stops resolving) and no odd address
/// anywhere in the output.
#[test]
fn decompile_all_reports_each_entry_once() {
    let bin = arm_thumb();
    let sp = specs();
    let (stdout, stderr, ok) =
        run_kuna(&["decompile-all", bin.as_str(), "--json", "--sleighpath", sp.as_str()]);
    if !ok {
        panic!("kuna decompile-all failed on the ARM fixture: {stderr}");
    }
    assert_eq!(
        json_count(&stdout),
        Some(2),
        "the 2-function ARM fixture must report 2 entries, not one per name/twin:\n{stdout}"
    );
    // The real symbols win the `name` slot ...
    for want in ["\"name\": \"compute\"", "\"name\": \"_start\""] {
        assert!(stdout.contains(want), "expected {want} in:\n{stdout}");
    }
    // ... the generated placeholders survive as aliases, not as extra records ...
    for want in ["\"sub_100b8\"", "\"sub_100d6\""] {
        assert!(stdout.contains(want), "expected the alias {want} in:\n{stdout}");
    }
    assert!(
        !stdout.contains("\"name\": \"sub_100b8\"") && !stdout.contains("\"name\": \"sub_100d6\""),
        "a generic `sub_<addr>` alias must not be a function's reported name:\n{stdout}"
    );
    // ... and the Thumb `entry|1` phantoms are gone entirely (address AND name).
    for gone in ["0x100b9", "0x100d7", "sub_100b9", "sub_100d7"] {
        assert!(
            !stdout.contains(gone),
            "the ARM Thumb `entry|1` twin {gone} must not be reported at all:\n{stdout}"
        );
    }
}

/// Issue #197, the companion guarantee: collapsing the enumeration must not make a
/// name that used to select a function stop working.  `--functions <alias>` still
/// resolves an entry through its alias list — the lookup decbench's name-narrowing
/// relies on — and reports it under its canonical name.
#[test]
fn decompile_all_functions_filter_resolves_an_alias() {
    let bin = arm_thumb();
    let sp = specs();
    let (stdout, stderr, ok) = run_kuna(&[
        "decompile-all",
        bin.as_str(),
        "--json",
        "--sleighpath",
        sp.as_str(),
        "--functions",
        "sub_100b8",
    ]);
    if !ok {
        panic!("kuna decompile-all failed on the ARM fixture: {stderr}");
    }
    assert_eq!(
        json_count(&stdout),
        Some(1),
        "`--functions sub_100b8` must still select exactly one function:\n{stdout}"
    );
    assert!(
        stdout.contains("\"name\": \"compute\"") && stdout.contains("\"0x100b8\""),
        "the alias must resolve to `compute` @ 0x100b8:\n{stdout}"
    );
}

/// Issue #197, `--addr` on an ARM/Thumb `entry|1` address.
///
/// An ARM caller legitimately holds odd addresses — an ELF `st_value`, a DWARF
/// entry PC, a benchmark case address all carry the Thumb mode bit. Asking for
/// `--addr 0x100b9` used to decompile literally there, landing mid-`push {r7}`
/// and returning an empty `void compute(void) { return; }`. It now resolves to the
/// real entry, and the odd address must NOT fold on a byte-aligned ISA, where an
/// odd function address is genuine (`cet_pie_x86_64` really has
/// `elaborate_debug_symbol` at 0x1357).
#[test]
fn decompile_all_addr_tolerates_the_arm_thumb_bit() {
    let sp = specs();
    let arm = arm_thumb();
    let (stdout, stderr, ok) = run_kuna(&[
        "decompile-all", arm.as_str(), "--json", "--sleighpath", sp.as_str(), "--addr", "0x100b9",
    ]);
    if !ok {
        panic!("kuna decompile-all failed on the ARM fixture: {stderr}");
    }
    assert!(
        stdout.contains("\"address_hex\": \"0x100b8\"") && stdout.contains("\"name\": \"compute\""),
        "--addr 0x100b9 must resolve to `compute` at its real entry 0x100b8:\n{stdout}"
    );
    assert!(
        stdout.contains("a0 * 3 + 7"),
        "--addr 0x100b9 must decompile the real body, not an empty phantom:\n{stdout}"
    );

    // The x86-64 guardrail: an odd address there is a real entry, never folded.
    let x86 = repo_root()
        .join("decompiler/crates/kuna-analysis/tests/fixtures/cet_pie_x86_64")
        .to_str()
        .unwrap()
        .to_string();
    let (stdout, stderr, ok) = run_kuna(&[
        "decompile-all", x86.as_str(), "--json", "--sleighpath", sp.as_str(), "--addr", "0x1357",
    ]);
    if !ok {
        panic!("kuna decompile-all failed on the x86-64 fixture: {stderr}");
    }
    assert!(
        stdout.contains("\"address_hex\": \"0x1357\""),
        "an odd x86-64 address is a REAL entry and must not be Thumb-masked:\n{stdout}"
    );
}

#[test]
fn arm_thumb_pe_functions_and_address_decompile() {
    let binary = arm_thumb_pe();
    let sp = specs();
    let target = "ARM:LE:32:v4t:default";

    let (stdout, stderr, ok) = run_kuna(&[
        "functions",
        &binary,
        "--json",
        "--sleighpath",
        &sp,
    ]);
    if !ok {
        panic!("kuna functions failed on synthetic ARM PE: {stderr}");
    }
    assert!(
        stdout.contains("\"address_hex\": \"0x401000\""),
        "odd Thumb entry was not normalized:\n{stdout}"
    );
    // The name follows the normalized address, not the raw entry word.
    assert!(
        stdout.contains("\"name\": \"sub_401000\"") && !stdout.contains("sub_401001"),
        "the entry function must be named at its even address:\n{stdout}"
    );

    let (stdout, stderr, ok) = run_kuna(&[
        "decompile-all",
        &binary,
        "--addr",
        "0x401001",
        "--target",
        target,
        "--sleighpath",
        &sp,
    ]);
    assert!(ok, "automatic PE Thumb mode failed: {stderr}");
    assert!(stdout.contains("return 7;"), "wrong ARM/Thumb decode:\n{stdout}");

    // An endian-conflicting --target is reported, not refused: --target is the
    // flag that overrides what the container declares, and a byte-swapped decode
    // of a mislabeled image is a legitimate use of it.
    let (stdout, stderr, ok) = run_kuna(&[
        "functions",
        &binary,
        "--target",
        "ARM:BE:32:v4t:default",
        "--sleighpath",
        &sp,
    ]);
    assert!(ok, "endian-conflicting target must still load: {stderr}");
    assert!(
        stderr.contains("BE-endian") && stderr.contains("LE-endian"),
        "the mismatch must still be reported: {stderr}"
    );
    assert!(stdout.contains("0x401000"), "{stdout}");
}

#[test]
fn te_image_auto_detects_entry_mapping_and_thumb_context() {
    let path = write_thumb_te();
    let binary = path.to_string_lossy().into_owned();
    let sp = specs();

    let (stdout, stderr, ok) =
        run_kuna(&["functions", &binary, "--json", "--sleighpath", &sp]);
    assert!(ok, "TE functions failed: {stderr}");
    assert!(stdout.contains("\"count\": 1"), "{stdout}");
    assert!(
        stdout.contains("\"address_hex\": \"0x401000\""),
        "{stdout}"
    );
    // An inventory of one is reported, not left to be inferred — on a plain
    // run, with no options asking for the discovery tier.
    assert!(
        stderr.contains("no object-file view, so function discovery cannot run"),
        "a TE inventory must say why it is only the entry: {stderr}"
    );

    let (stdout, stderr, ok) = run_kuna(&[
        "functions",
        &binary,
        "--json",
        "--target",
        "default",
        "--sleighpath",
        &sp,
    ]);
    assert!(ok, "TE --target default failed: {stderr}");
    assert!(
        stdout.contains("\"address_hex\": \"0x401000\""),
        "{stdout}"
    );

    let (stdout, stderr, ok) = run_kuna(&[
        "functions",
        &binary,
        "--json",
        "--option",
        "namestyle",
        "ghidra",
        "--filter",
        "^func_",
        "--sleighpath",
        &sp,
    ]);
    assert!(ok, "TE functions with ghidra names failed: {stderr}");
    assert!(stdout.contains("\"count\": 1"), "{stdout}");
    assert!(stdout.contains("\"name\": \"func_0x00401000\""), "{stdout}");

    let (stdout, stderr, ok) = run_kuna(&[
        "read",
        &binary,
        "0x401003",
        "--addr",
        "--bytes",
        "16",
        "--json",
        "--sleighpath",
        &sp,
    ]);
    assert!(ok, "TE boundary read failed: {stderr}");
    assert!(stdout.contains("\"end\": 4198404"), "{stdout}");
    assert!(stdout.contains("\"bytes\": 1"), "{stdout}");
    assert!(stdout.contains("\"hex\": \"47\""), "{stdout}");

    // `--slice` names a Mach-O fat slice; a thin image ignores it, and a TE is
    // a thin image, so it is accepted on every surface rather than rejected on
    // some.
    let (stdout, stderr, ok) =
        run_kuna(&["functions", &binary, "--json", "--slice", "arm64", "--sleighpath", &sp]);
    assert!(ok, "TE --slice must be ignored like any thin image: {stderr}");
    assert!(stdout.contains("\"count\": 1"), "{stdout}");

    let (stdout, stderr, ok) = run_kuna(&[
        "decompile-all",
        &binary,
        "--addr",
        "0x401001",
        "--sleighpath",
        &sp,
    ]);
    assert!(ok, "TE decompile failed: {stderr}");
    assert!(stdout.contains("return 7;"), "unexpected TE body:\n{stdout}");

    let (stdout, stderr, ok) = run_kuna(&[
        "decompile-all",
        &binary,
        "--addr",
        "0x401001",
        "--assert",
        "bytes 0x401000 2a207047",
        "--assert-strict",
        "--sleighpath",
        &sp,
    ]);
    assert!(ok, "strict TE byte overlay failed: {stderr}");
    assert!(
        stdout.contains("return 0x2a;"),
        "TE decompiled stale image bytes:\n{stdout}"
    );

    let project_dir = common::scratch_file("te-project", "dir");
    let (_stdout, stderr, ok) = run_kuna(&[
        "decompile-project",
        &binary,
        "-o",
        project_dir.to_str().unwrap(),
        "--sleighpath",
        &sp,
    ]);
    assert!(ok, "TE project export failed: {stderr}");
    let readme = std::fs::read_to_string(project_dir.join("README.md")).unwrap();
    assert!(
        readme.contains("| Entry point | `0x401000` |"),
        "TE README omitted its entry point:\n{readme}"
    );
    assert!(readme.contains("## Sections"), "TE README omitted its sections:\n{readme}");
    assert!(
        readme.contains("| `.text` | `0x401000` | `0x4` | Text |"),
        "TE README omitted its named code section:\n{readme}"
    );
    std::fs::remove_dir_all(project_dir).unwrap();

    std::fs::remove_file(path).unwrap();
}

#[test]
fn te_object_view_commands_report_capability_errors() {
    let path = write_thumb_te();
    let binary = path.to_string_lossy().into_owned();
    let sp = specs();

    // Every object-view consumer answers with the same capability error, never
    // the object crate's own parse failure.
    for (command, flag) in [
        ("functions", vec!["--summary"]),
        ("functions", vec!["--reachable-from", "0x401000"]),
        ("decompile-all", vec!["--summary"]),
        ("decompile-all", vec!["--reachable-from", "0x401000"]),
        ("xrefs", vec!["--to", "0x401000"]),
        ("decompile-graph", vec![]),
        ("strings", vec!["--no-xrefs"]),
    ] {
        let mut args = vec![command, &binary, "--sleighpath", &sp];
        args.extend(flag);
        let (_stdout, stderr, ok) = run_kuna(&args);
        assert!(!ok, "TE {command} unexpectedly succeeded");
        assert!(
            stderr.contains("UEFI TE input has no object-file view"),
            "unexpected TE diagnostic from {command}: {stderr}"
        );
        assert!(!stderr.contains("Unknown file magic"), "leaked object parser error: {stderr}");
    }
    std::fs::remove_file(path).unwrap();
}

#[test]
fn te_input_errors_preserve_format_and_target_diagnostics() {
    let path = write_thumb_te();
    let binary = path.to_string_lossy().into_owned();
    let sp = specs();

    // A TE for a machine kuna has no binding for is still a TE: the user is
    // told which machine, not handed the headerless-image guidance.
    let ebc = common::scratch_file("ebc", "te");
    std::fs::write(
        &ebc,
        kuna_analysis::loadimage_te::synthetic::TeImage::thumb(&[0; 4]).machine(0x0ebc).build(),
    )
    .unwrap();
    let ebc_path = ebc.to_string_lossy().into_owned();
    let (_stdout, stderr, ok) = run_kuna(&["functions", &ebc_path, "--json", "--sleighpath", &sp]);
    assert!(!ok, "an EBC TE unexpectedly loaded");
    assert!(
        stderr.contains("unsupported machine value 0x0ebc"),
        "an unsupported TE machine must be named: {stderr}"
    );
    std::fs::remove_file(ebc).unwrap();

    // A file that merely opens with the two signature letters is not a TE
    // image: it keeps the unrecognized-input guidance rather than being routed
    // into the TE parser or refused as one.
    let prose = common::scratch_file("not-a-te", "bin");
    std::fs::write(&prose, b"VZ: a note about the build, not a container").unwrap();
    let prose_path = prose.to_string_lossy().into_owned();
    let (_stdout, stderr, ok) =
        run_kuna(&["functions", &prose_path, "--json", "--sleighpath", &sp]);
    assert!(!ok, "a non-container unexpectedly loaded");
    assert!(
        stderr.contains("--raw-image") && !stderr.contains("TE"),
        "a `VZ`-prefixed non-container must keep the raw-image guidance: {stderr}"
    );
    let (_stdout, stderr, ok) = run_kuna(&["strings", &prose_path, "--no-xrefs"]);
    assert!(!ok, "a non-container unexpectedly scanned");
    assert!(
        !stderr.contains("UEFI TE"),
        "a `VZ`-prefixed non-container must not be diagnosed as TE: {stderr}"
    );
    std::fs::remove_file(prose).unwrap();

    let (_stdout, stderr, ok) = run_kuna(&[
        "functions",
        &binary,
        "--target",
        "ARM:BE:32:v4t:default",
        "--sleighpath",
        &sp,
    ]);
    assert!(!ok, "endianness-conflicting TE target unexpectedly loaded");
    assert!(stderr.contains("BE-endian") && stderr.contains("LE-endian"));
    std::fs::remove_file(path).unwrap();
}

/// The past-pathological function of the stripped-ELF hang repro now
/// CONVERGES: `sub_1bd04` @ 0x1bd04 used to spin forever (100% CPU, no output)
/// in a condconst↔lowered-switch-repair fixpoint tug-of-war
/// (`kuna_repair_lowered_switch_inputs` mis-classified the constant that
/// `ActionConditionalConst` legitimately installed on the synthetic BRANCHIND
/// as a broken input, re-pointing it at the register def every heritage pass).
/// With the repair's healthy-input test accepting heritage-known Varnodes the
/// pipeline converges, so the DEFAULT watchdog budget must never fire here:
/// the function decompiles with non-null `code` and null `error`.
///
/// This is the convergence-regression gate: if the fixpoint bug returns, the
/// default 120s budget turns it into a per-function error (failing the
/// `"error": null` assertion) inside the generous 300s outer cap — visible,
/// never a hung CI.  The watchdog *mechanism* stays covered deterministically
/// by `kuna-decomp`'s `repeatapply_deadline_bounds_nonconverging_action` unit
/// test (an already-expired deadline bounding a never-converging repeatapply
/// loop).
#[test]
fn decompile_all_converges_on_past_pathological_function() {
    let bin = hang_repro();
    let res = run_kuna_with_timeout(
        &[
            "decompile-all", &bin, "--addr", "0x1bd04", "--json", "--sleighpath",
            &specs(), "--mode", "reliable",
        ],
        Duration::from_secs(300),
    );
    let (stdout, stderr, ok) = match res {
        Some(t) => t,
        None => panic!(
            "kuna decompile-all did not terminate within the 300s outer bound — \
             the 0x1bd04 convergence fix has regressed AND the default \
             --max-fn-seconds watchdog is not firing"
        ),
    };
    if !ok {
        panic!("kuna decompile-all failed: {stderr}");
    }
    // Shape assertions (no JSON dep): a well-formed single-function document
    // whose one record decompiled cleanly (non-null code, null error).
    let trimmed = stdout.trim();
    assert!(trimmed.starts_with('{') && trimmed.ends_with('}'), "output is not a JSON object:\n{stdout}");
    assert!(stdout.contains("\"count\": 1"), "expected count 1:\n{stdout}");
    assert!(stdout.contains("\"address_hex\": \"0x1bd04\""), "missing the 0x1bd04 record:\n{stdout}");
    assert!(
        stdout.contains("\"error\": null"),
        "sub_1bd04 must decompile cleanly now (the convergence fix regressed?):\n{stdout}"
    );
    assert!(
        stdout.contains("\"code\": \""),
        "sub_1bd04 must emit code (the convergence fix regressed?):\n{stdout}"
    );
    assert!(
        !stdout.contains("budget exceeded"),
        "the watchdog must not fire on the fixed function:\n{stdout}"
    );
}

/// Watchdog control: a healthy function in the SAME hang-repro binary
/// decompiles normally under the default budget — `code` non-null, `error`
/// null — so the watchdog demonstrably fires only on pathological input.
#[test]
fn decompile_all_watchdog_quiet_on_healthy_function() {
    let bin = hang_repro();
    // 0x5020 is a tiny PLT-style thunk (`sub_5020`) that decompiles in
    // milliseconds on a release build; the default 120s budget applies.
    let res = run_kuna_with_timeout(
        &[
            "decompile-all", &bin, "--addr", "0x5020", "--json", "--sleighpath",
            &specs(), "--mode", "reliable",
        ],
        Duration::from_secs(300),
    );
    let (stdout, stderr, ok) = match res {
        Some(t) => t,
        None => panic!("kuna decompile-all on a healthy function did not terminate in 300s"),
    };
    if !ok {
        panic!("kuna decompile-all failed: {stderr}");
    }
    assert!(stdout.contains("\"count\": 1"), "expected count 1:\n{stdout}");
    assert!(stdout.contains("\"error\": null"), "healthy function must have null error:\n{stdout}");
    assert!(stdout.contains("\"code\": \""), "healthy function must emit code:\n{stdout}");
    assert!(
        !stdout.contains("budget exceeded"),
        "watchdog must not fire on a healthy function:\n{stdout}"
    );
}

/// The `noreturn_propagate` fixture (`kuna-analysis/tests/fixtures/`): a
/// non-PIE x86-64 ELF whose custom no-return wrapper `my_die` (ending in
/// `call abort` + NOP padding, called from a SINGLE site) is only concluded
/// no-return by the call-graph propagation fixpoint — the mechanism the
/// decompile-all Listing default (decbench F1, DIV-15) exists to activate.
fn noreturn_fixture() -> String {
    repo_root()
        .join("decompiler/crates/kuna-analysis/tests/fixtures/noreturn_propagate_x86_64")
        .to_str()
        .unwrap()
        .to_string()
}

/// Extract the JSON-escaped `code` string of the first function record (shape
/// assertion helper — no JSON dep, mirrors the other raw-substring checks).
fn code_field(stdout: &str) -> &str {
    let start = stdout.find("\"code\": \"").expect("record has a code field") + 9;
    let rest = &stdout[start..];
    // The code string ends at the first unescaped quote.
    let mut end = 0;
    let bytes = rest.as_bytes();
    while end < bytes.len() {
        match bytes[end] {
            b'\\' => end += 2,
            b'"' => break,
            _ => end += 1,
        }
    }
    &rest[..end]
}

/// decbench F1 (DIV-15), the two-pass gate at the exact benchmark surface:
///
/// - **reliable fallback** (`listing` injected on ⇒ the default-on `noreturn_propagate`
///   fixpoint fires): `compute`'s single `call my_die` is concluded no-return —
///   the no-return terminator appears and the post-call dead fall-through is
///   gone (the "collapsed" form);
/// - **`--option listing off`** (the explicit opt-out = the pre-F1 behavior):
///   `my_die` is treated as returning, the dead fall-through survives, and the
///   output is the inflated form (on real stripped binaries this is the
///   swallow-the-next-function shape, e.g. coreutils `xalloc_die`,
///   118 LOC / 2 gotos for a 4-instruction body).
#[test]
fn decompile_all_listing_default_collapses_noreturn_wrapper() {
    let bin = noreturn_fixture();
    let sleigh = specs();
    let base = [
        "decompile-all", bin.as_str(), "--functions", "compute", "--json",
        "--sleighpath", sleigh.as_str(), "--mode", "reliable",
    ];

    // Pass 1: reliable has no listing override, so the driver fallback fires.
    let (on_out, stderr, ok) = run_kuna(&base);
    if !ok {
        panic!("kuna decompile-all (default) failed: {stderr}");
    }
    let on_code = code_field(&on_out).to_string();
    assert!(
        on_code.contains("// no-return"),
        "default decompile-all must mark the my_die() wrapper call no-return \
         (the Listing default is not reaching noreturn_propagate):\n{on_code}"
    );

    // Pass 2: the opt-out — `--option listing off` restores the old behavior.
    let mut off_args = base.to_vec();
    off_args.extend_from_slice(&["--option", "listing", "off"]);
    let (off_out, stderr, ok) = run_kuna(&off_args);
    assert!(ok, "kuna decompile-all --option listing off failed: {stderr}");
    let off_code = code_field(&off_out).to_string();
    assert!(
        !off_code.contains("// no-return"),
        "listing-off output must NOT mark my_die() no-return (the opt-out must \
         restore the pre-F1 rendering):\n{off_code}"
    );
    assert_ne!(
        on_code, off_code,
        "the Listing default must change compute's decompilation"
    );
    assert!(
        on_code.len() < off_code.len(),
        "the no-return collapse must SHRINK the function (dead fall-through \
         eliminated):\n--- default ({} bytes) ---\n{on_code}\n--- listing off ({} bytes) ---\n{off_code}",
        on_code.len(),
        off_code.len()
    );

    // An EXPLICIT `--option listing on` must be byte-identical to the default
    // (the injection only fills the unset case; it never double-applies).
    let mut expl_args = base.to_vec();
    expl_args.extend_from_slice(&["--option", "listing", "on"]);
    let (expl_out, stderr, ok) = run_kuna(&expl_args);
    assert!(ok, "kuna decompile-all --option listing on failed: {stderr}");
    assert_eq!(
        code_field(&expl_out),
        on_code,
        "explicit `--option listing on` must match the injected default"
    );
}

#[test]
fn functions_lists_main() {
    let bin = fauxware();
    let (stdout, stderr, ok) = run_kuna(&["functions", &bin, "--json", "--sleighpath", &specs()]);
    if !ok {
        panic!("kuna functions failed: {stderr}");
    }
    assert!(stdout.contains("\"name\": \"main\""), "enumeration missing `main`:\n{stdout}");
    assert!(stdout.contains("\"address\""), "enumeration missing addresses:\n{stdout}");
}

/// (kuna, Ghidra-gap) The `error(nonzero,…)` boundary-overrun fix. `err_fatal`
/// (0x4011c0) ends in `call error(2,…)` — glibc `error()` with a nonzero status
/// never returns — so the decompile-all seam must prune its fall-through (a
/// `CALL_RETURN` flow override) exactly as Ghidra does ("Subroutine does not
/// return"). Without the prune the flow-follower walks past the call into the
/// following function `compute` (0x4011f0) and absorbs it, inflating the CFG —
/// the single biggest cause of kuna losing to Ghidra proper on the benchmark
/// (~50% of the ghidra-beats-kuna GED cases were this boundary overrun).
///
/// The test isolates exactly the fix: `--option noreturn_error off` (no error
/// recognizer ⇒ no prune ⇒ err_fatal absorbs `compute`) must yield a LARGER
/// function byte-extent than the default (`noreturn_error on`, the prune fires).
#[test]
fn decompile_all_error_nonzero_does_not_absorb_next_function() {
    let bin = noreturn_error_fixture();
    let sp = specs();
    let code = |extra: &[&str]| -> String {
        let mut a: Vec<&str> =
            vec!["decompile-all", &bin, "--addr", "0x4011c0", "--json", "--sleighpath", &sp];
        a.extend_from_slice(extra);
        let (stdout, stderr, ok) = run_kuna(&a);
        assert!(ok, "decompile-all failed: {stderr}");
        stdout
    };
    // OFF: err_fatal's flow walks past `call error(2)` into the following functions.
    // `funcboundflow` (default-on, DIV-67) is a SECOND, name-independent bound that
    // stops the same overrun at `compute`'s entry, so it must also be off to expose
    // the pre-fix overrun this test isolates.
    let off = code(&["--option", "noreturn_error", "off", "--option", "funcboundflow", "off"]);
    // ON (default): the CALL_RETURN prune stops err_fatal at the no-return call.
    let on = code(&[]);
    // `err_warn` belongs to `compute_warn` — a DIFFERENT function two hops after
    // err_fatal. It can only appear in err_fatal's decompilation if the flow-follower
    // overran `call error(2)` and absorbed the following functions. OFF must show the
    // overrun; ON (the prune) must not.
    assert!(
        off.contains("err_warn"),
        "with noreturn_error off, err_fatal should overrun and absorb the following \
         functions (the pre-fix behaviour):\n{off}"
    );
    assert!(
        !on.contains("err_warn"),
        "noreturn_error must prune the `call error(2)` fall-through so err_fatal does \
         NOT absorb `compute`/`compute_warn`:\n{on}"
    );
}

/// (kuna, Ghidra-gap) The SINGLE-function `kuna decompile` path must also prune the
/// `error(nonzero)` fall-through — not just `decompile-all`. It now builds the Listing by
/// default (like decompile-all) and `IfcDecompile` applies the CALL_RETURN overrides, so
/// `err_fatal` @ 0x4011c0 does not overrun into the following `compute`/`compute_warn`.
/// `--option noreturn_error off` disables the recognizer ⇒ the overrun returns (control).
#[test]
fn kuna_decompile_single_error_nonzero_does_not_absorb_next_function() {
    let bin = noreturn_error_fixture();
    let sp = specs();
    let code = |extra: &[&str]| -> String {
        let mut a: Vec<&str> = vec!["decompile", &bin, "0x4011c0", "--addr", "--sleighpath", &sp];
        a.extend_from_slice(extra);
        let (stdout, stderr, ok) = run_kuna(&a);
        assert!(ok, "kuna decompile failed: {stderr}");
        stdout
    };
    // `err_warn` belongs to `compute_warn`, a DIFFERENT function — it appears in err_fatal's
    // output ONLY if the flow overran past `call error(2)`.  `funcboundflow` (default-on,
    // DIV-67) is a second, name-independent bound at `compute`'s entry, so it too must be
    // off to expose the pre-fix overrun.
    let off = code(&["--option", "noreturn_error", "off", "--option", "funcboundflow", "off"]);
    let on = code(&[]);
    assert!(
        off.contains("err_warn"),
        "noreturn_error off: single-function err_fatal should overrun (pre-fix):\n{off}"
    );
    assert!(
        !on.contains("err_warn"),
        "the single-function `kuna decompile` path must prune the error(2) fall-through:\n{on}"
    );
}

/// `dwarf_lines` must stay a per-run opt-in even under `--mode aggressive`.
///
/// `auto` (the file-frontend default since DIV-40) resolves to `aggressive`
/// below 500 KiB, so while `aggressive` carried `dwarf_lines on` every small
/// `-g` binary rendered its whole body interleaved with `/* src.c:NNN */`
/// comments by default. `cet_pie_x86_64` (20 KiB, DWARF, resolves to
/// `aggressive`) is the repro: annotated only when the option is named.
#[test]
fn dwarf_source_line_comments_stay_opt_in_under_every_mode() {
    let bin = repo_root()
        .join("decompiler/crates/kuna-analysis/tests/fixtures/cet_pie_x86_64")
        .to_str()
        .unwrap()
        .to_string();
    let sp = specs();
    let code = |extra: &[&str]| -> String {
        let mut a: Vec<&str> =
            vec!["decompile", &bin, "elaborate_debug_symbol", "--sleighpath", &sp];
        a.extend_from_slice(extra);
        let (stdout, stderr, ok) = run_kuna(&a);
        if !ok {
            panic!("kuna decompile failed for {extra:?}: {stderr}");
        }
        stdout
    };

    let default = code(&[]);
    assert!(
        default.contains("elaborate_debug_symbol"),
        "expected the function body, got:\n{default}"
    );
    assert!(
        !default.contains("/* debug_symbol.c:"),
        "the default (auto -> aggressive here) must NOT annotate source lines:\n{default}"
    );

    let aggressive = code(&["--mode", "aggressive"]);
    assert!(
        !aggressive.contains("/* debug_symbol.c:"),
        "--mode aggressive must NOT annotate source lines:\n{aggressive}"
    );

    // Named explicitly, the pass still works — and outranks the mode.
    let opted_in = code(&["--option", "dwarf_lines", "on"]);
    assert!(
        opted_in.contains("/* debug_symbol.c:124 */"),
        "`--option dwarf_lines on` must still annotate source lines:\n{opted_in}"
    );
}

#[test]
fn raw_image_supported_surfaces_share_seed_and_base_semantics() {
    let path = common::scratch_file("raw thumb image", "bin");
    std::fs::write(&path, [0x07, 0x20, 0x70, 0x47]).unwrap();
    let binary = path.to_string_lossy().into_owned();
    let sp = specs();
    let target = "ARM:LE:32:v4t:default";
    let spec = PathBuf::from(&sp).join("Ghidra/Processors/ARM/data/languages/ARM8_le.sla");
    assert!(spec.exists(), "required processor spec missing; build specs before running integration tests");

    let (stdout, stderr, ok) = run_kuna(&[
        "functions", &binary, "--json", "--raw-image", "--target", target, "--base",
        "0x4000", "--entry", "0x4001", "--isa", "thumb", "--sleighpath", &sp,
    ]);
    assert!(ok, "raw functions failed: {stderr}");
    assert!(stdout.contains("\"count\": 1"), "{stdout}");
    assert!(stdout.contains("\"address_hex\": \"0x4000\""), "{stdout}");

    let (stdout, stderr, ok) = run_kuna(&[
        "functions", &binary, "--json", "--raw-image", "--target", target, "--base",
        "0x4000", "--entry", "0x4001", "--isa", "thumb", "--option", "namestyle",
        "ghidra", "--filter", "^func_", "--sleighpath", &sp,
    ]);
    assert!(ok, "raw functions with ghidra names failed: {stderr}");
    assert!(stdout.contains("\"count\": 1"), "{stdout}");
    assert!(stdout.contains("\"name\": \"func_"), "{stdout}");

    let (stdout, stderr, ok) = run_kuna(&[
        "decompile-all", &binary, "--raw-image", "--target", target, "--base", "0x4000",
        "--addr", "0x4001", "--isa", "thumb", "--sleighpath", &sp,
    ]);
    assert!(ok, "raw decompile-all failed: {stderr}");
    assert!(stdout.contains("return 7;"), "unexpected raw body:\n{stdout}");

    let (stdout, stderr, ok) = run_kuna(&[
        "decompile", &binary, "0x4001", "--json", "--raw-image", "--target", target,
        "--base", "0x4000", "--isa", "thumb", "--sleighpath", &sp,
    ]);
    assert!(ok, "raw decompile --json failed: {stderr}");
    assert!(stdout.contains("\"address\": 16384"), "{stdout}");
    assert!(stdout.contains("return 7;"), "{stdout}");

    let out_dir = common::scratch_file("raw-project", "dir");
    let (stdout, stderr, ok) = run_kuna(&[
        "decompile-project", &binary, "-o", out_dir.to_str().unwrap(), "--raw-image",
        "--target", target, "--base", "0x4000", "--entry", "0x4001", "--isa",
        "thumb", "--assert", "data 0x4001 char odd_data", "--sleighpath", &sp,
    ]);
    assert!(ok, "raw project export failed: {stderr}");
    assert!(stdout.contains("functions: 1 ok, 0 failed"), "{stdout}");
    let artifacts: Vec<String> = std::fs::read_dir(&out_dir)
        .unwrap()
        .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
        .collect();
    assert!(artifacts.iter().any(|name| name.ends_with(".c")), "{artifacts:?}");
    assert!(artifacts.iter().any(|name| name.ends_with(".asm")), "{artifacts:?}");
    assert!(artifacts.iter().any(|name| name == "README.md"), "{artifacts:?}");
    let asm_name = artifacts.iter().find(|name| name.ends_with(".asm")).unwrap();
    let asm = std::fs::read_to_string(out_dir.join(asm_name)).unwrap();
    let data_tail = asm.split("; --- data ---").nth(1).expect("project data tail");
    assert!(data_tail.contains("odd_data:  ; 0x4001"), "{data_tail}");
    assert!(data_tail.contains("  00004001:"), "{data_tail}");
    assert!(!data_tail.contains("odd_data:  ; 0x4000"), "{data_tail}");

    std::fs::remove_dir_all(out_dir).unwrap();
    std::fs::remove_file(path).unwrap();
}

/// (kuna `rawdiscover`) A headerless image's inventory is its seeds plus what
/// the executable bytes call, not just what the caller typed.
///
/// The fixture is 36 bytes of Cortus APS3 laid out so the two discovery halves
/// are distinguishable. `0x80000010` is reached by a direct call from the entry,
/// so the recursive descent alone would find it; `0x80000020` is called only
/// from `0x80000018`, which sits past the entry function's `ret` with nothing
/// branching to it, so only the linear call-target sweep reaches it. An
/// unfiltered `decompile-all` must then emit all three bodies, because on a raw
/// image `--entry` seeds the load without selecting.
#[test]
fn raw_image_discovers_called_functions_beyond_its_seeds() {
    let path = common::scratch_file("raw-aps3-calls", "bin");
    #[rustfmt::skip]
    let image: [u8; 36] = [
        0x04, 0x21,                          // 0x00 mov r2,1
        0x8b, 0x00, 0x00, 0x00,              // 0x02 call 0x80000010
        0xe1, 0xf0,                          // 0x06 ret
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x26, 0x22,                          // 0x10 add r2,r2
        0xe1, 0xf0,                          // 0x12 ret
        0x00, 0x00, 0x00, 0x00,
        0x0b, 0x01, 0x00, 0x00,              // 0x18 call 0x80000020 (no flow reaches here)
        0xe1, 0xf0,                          // 0x1c ret
        0x00, 0x00,
        0x04, 0x27,                          // 0x20 mov r2,7
        0xe1, 0xf0,                          // 0x22 ret
    ];
    std::fs::write(&path, image).unwrap();
    let binary = path.to_string_lossy().into_owned();
    let sp = specs();
    let target = "Cortus:LE:32:APS3:default";
    let spec = PathBuf::from(&sp).join("Ghidra/Processors/Cortus/data/languages/aps3.sla");
    assert!(spec.exists(), "required processor spec missing; build specs before running integration tests");

    // Off: the inventory is exactly the seed, as it was before the option.
    let (stdout, stderr, ok) = run_kuna(&[
        "functions", &binary, "--json", "--raw-image", "--target", target, "--base",
        "0x80000000", "--entry", "0x80000000", "--option", "rawdiscover", "off",
        "--sleighpath", &sp,
    ]);
    assert!(ok, "raw functions with rawdiscover off failed: {stderr}");
    assert!(stdout.contains("\"count\": 1"), "{stdout}");

    // On (the default): the seed, its direct callee, and the sweep-only callee.
    let (stdout, stderr, ok) = run_kuna(&[
        "functions", &binary, "--json", "--raw-image", "--target", target, "--base",
        "0x80000000", "--entry", "0x80000000", "--sleighpath", &sp,
    ]);
    assert!(ok, "raw functions failed: {stderr}");
    assert!(stdout.contains("\"count\": 3"), "{stdout}");
    assert!(stdout.contains("\"address_hex\": \"0x80000010\""), "{stdout}");
    assert!(
        stdout.contains("\"address_hex\": \"0x80000020\""),
        "the sweep-only callee must be discovered:\n{stdout}"
    );

    // `--entry` seeds without selecting, so the whole inventory is decompiled.
    let (stdout, stderr, ok) = run_kuna(&[
        "decompile-all", &binary, "--json", "--raw-image", "--target", target,
        "--base", "0x80000000", "--entry", "0x80000000", "--sleighpath", &sp,
    ]);
    assert!(ok, "raw decompile-all failed: {stderr}");
    assert!(stdout.contains("\"count\": 3"), "{stdout}");
    assert!(stdout.contains("return 7;"), "the sweep-only body must decompile:\n{stdout}");
    // A raw image has no call graph, so the default `protoorder` stays out of the
    // way silently; naming the option is what makes it say it had nothing to order.
    assert!(!stderr.contains("protoorder"), "the default spoke on a raw image:\n{stderr}");
    let (named, stderr, ok) = run_kuna(&[
        "decompile-all", &binary, "--json", "--raw-image", "--target", target,
        "--base", "0x80000000", "--entry", "0x80000000", "--option", "protoorder", "types",
        "--sleighpath", &sp,
    ]);
    assert!(ok, "raw decompile-all --option protoorder types failed: {stderr}");
    assert!(stderr.contains("--option protoorder: no call graph"), "{stderr}");
    assert_eq!(named, stdout, "naming protoorder moved a raw image's output");

    // `--addr` still narrows a raw run to the addresses named.
    let (stdout, stderr, ok) = run_kuna(&[
        "decompile-all", &binary, "--json", "--raw-image", "--target", target,
        "--base", "0x80000000", "--addr", "0x80000020", "--sleighpath", &sp,
    ]);
    assert!(ok, "raw decompile-all --addr failed: {stderr}");
    assert!(stdout.contains("\"count\": 1"), "{stdout}");
    assert!(stdout.contains("return 7;"), "{stdout}");

    std::fs::remove_file(path).unwrap();
}

#[test]
fn raw_image_decompile_scales_word_addressed_selector() {
    let path = common::scratch_file("raw-avr-return", "bin");
    std::fs::write(&path, [0, 0, 0x08, 0x95]).unwrap();
    let binary = path.to_string_lossy().into_owned();
    let sp = specs();
    let spec = PathBuf::from(&sp).join("Ghidra/Processors/Atmel/data/languages/avr8.sla");
    assert!(spec.exists(), "required processor spec missing; build specs before running integration tests");

    let (stdout, stderr, ok) = run_kuna(&[
        "functions", &binary, "--json", "--raw-image", "--target",
        "avr8:LE:16:default", "--base", "0x100", "--entry", "0x101",
        "--sleighpath", &sp,
    ]);
    assert!(ok, "word-addressed raw inventory failed: {stderr}");
    assert!(stdout.contains("\"address\": 257"), "{stdout}");
    assert!(stdout.contains("\"address_hex\": \"0x101\""), "{stdout}");
    assert!(!stdout.contains("\"address_hex\": \"0x202\""), "{stdout}");

    let (stdout, stderr, ok) = run_kuna(&[
        "functions", &binary, "--raw-image", "--target", "avr8:LE:16:default",
        "--base", "0x100", "--entry", "0x101", "--sleighpath", &sp,
    ]);
    assert!(ok, "word-addressed raw text inventory failed: {stderr}");
    assert!(stdout.contains("0x101\tsub_101"), "{stdout}");

    let (stdout, stderr, ok) = run_kuna(&[
        "decompile-all", &binary, "--raw-image", "--target", "avr8:LE:16:default",
        "--base", "0x100", "--entry", "0x101", "--sleighpath", &sp,
    ]);
    assert!(ok, "word-addressed raw selector failed: {stderr}");
    assert!(stdout.contains("sub_101"), "{stdout}");
    assert!(stdout.contains("@ 0x101"), "{stdout}");

    let (stdout, stderr, ok) = run_kuna(&[
        "decompile", &binary, "0x101", "--raw-image", "--target", "avr8:LE:16:default",
        "--base", "0x100", "--sleighpath", &sp,
    ]);
    assert!(ok, "word-addressed raw text decompile failed: {stderr}");
    assert!(stdout.contains("sub_101"), "{stdout}");

    let (stdout, stderr, ok) = run_kuna(&[
        "decompile", &binary, "0x101", "--regions", "--raw-image", "--target",
        "avr8:LE:16:default", "--base", "0x100", "--sleighpath", &sp,
    ]);
    assert!(ok, "word-addressed raw regions failed: {stderr}");
    assert!(stdout.contains("[0x101]"), "{stdout}");
    assert!(stdout.contains("region head=0x101"), "{stdout}");
    assert!(!stdout.contains("[0x202]"), "{stdout}");
    assert!(!stdout.contains("region head=0x202"), "{stdout}");

    let (stdout, stderr, ok) = run_kuna(&[
        "decompile", &binary, "0x101", "--json", "--raw-image", "--target",
        "avr8:LE:16:default", "--base", "0x100", "--sleighpath", &sp,
    ]);
    assert!(ok, "word-addressed raw JSON decompile failed: {stderr}");
    assert!(stdout.contains("\"address\": 257"), "{stdout}");
    assert!(stdout.contains("\"address_hex\": \"0x101\""), "{stdout}");
    assert!(stdout.contains("\"addresses\": [\n            257"), "{stdout}");

    let out_dir = common::scratch_file("raw-avr-project", "dir");
    let (stdout, stderr, ok) = run_kuna(&[
        "decompile-project", &binary, "-o", out_dir.to_str().unwrap(), "--raw-image",
        "--target", "avr8:LE:16:default", "--base", "0x100", "--entry", "0x101",
        "--assert", "data 0x101 int foo", "--sleighpath", &sp,
    ]);
    assert!(ok, "word-addressed raw project failed: {stderr}");
    assert!(stdout.contains("functions: 1 ok, 0 failed"), "{stdout}");
    let files: Vec<_> = std::fs::read_dir(&out_dir)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .collect();
    let c_path = files
        .iter()
        .find(|path| path.extension().is_some_and(|ext| ext == "c"))
        .expect("project C file");
    let asm_path = files
        .iter()
        .find(|path| path.extension().is_some_and(|ext| ext == "asm"))
        .expect("project asm file");
    let c = std::fs::read_to_string(c_path).unwrap();
    let asm = std::fs::read_to_string(asm_path).unwrap();
    assert!(c.contains("// Function: sub_101 @ 0x101"), "{c}");
    assert!(asm.contains("sub_101:  ; 0x101"), "{asm}");
    assert!(asm.contains("00000101:"), "{asm}");
    let data_tail = asm.split("; --- data ---").nth(1).expect("project data tail");
    assert!(data_tail.contains("foo:  ; 0x101"), "{data_tail}");
    assert!(data_tail.contains("  00000101:"), "{data_tail}");
    assert!(!data_tail.contains("foo:  ; 0x202"), "{data_tail}");
    assert!(!data_tail.contains("  00000202:"), "{data_tail}");
    std::fs::remove_dir_all(out_dir).unwrap();
    std::fs::remove_file(path).unwrap();
}

#[test]
fn raw_project_preserves_byte_addressed_data_coordinates() {
    let path = common::scratch_file("raw-avr-data-reference", "bin");
    std::fs::write(&path, [0x80, 0x91, 0x00, 0x01, 0x08, 0x95]).unwrap();
    let binary = path.to_string_lossy().into_owned();
    let sp = specs();
    let spec = PathBuf::from(&sp).join("Ghidra/Processors/Atmel/data/languages/avr8.sla");
    assert!(spec.exists(), "required processor spec missing; build specs before running integration tests");

    let out_dir = common::scratch_file("raw-avr-data-project", "dir");
    let (stdout, stderr, ok) = run_kuna(&[
        "decompile-project", &binary, "-o", out_dir.to_str().unwrap(), "--raw-image",
        "--target", "avr8:LE:16:default", "--base", "0", "--entry", "0",
        "--assert", "data 0x80 int foo", "--sleighpath", &sp,
    ]);
    assert!(ok, "word-addressed raw data project failed: {stderr}");
    assert!(stdout.contains("functions: 1 ok, 0 failed"), "{stdout}");
    let files: Vec<_> = std::fs::read_dir(&out_dir)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .collect();
    let c = std::fs::read_to_string(
        files.iter().find(|path| path.extension().is_some_and(|ext| ext == "c")).unwrap(),
    )
    .unwrap();
    let asm = std::fs::read_to_string(
        files.iter().find(|path| path.extension().is_some_and(|ext| ext == "asm")).unwrap(),
    )
    .unwrap();
    assert!(c.contains("dat_100"), "{c}");
    let data_tail = asm.split("; --- data ---").nth(1).expect("project data tail");
    assert!(data_tail.contains("foo:  ; 0x80"), "{data_tail}");
    assert!(data_tail.contains("dat_100:  ; 0x100"), "{data_tail}");
    assert!(data_tail.contains("  00000100:"), "{data_tail}");
    assert!(!data_tail.contains("foo:  ; 0x80 = dat_100"), "{data_tail}");
    assert!(!data_tail.contains("dat_100:  ; 0x80"), "{data_tail}");
    std::fs::remove_dir_all(out_dir).unwrap();
    std::fs::remove_file(path).unwrap();
}

#[test]
fn raw_address_directives_use_target_units() {
    let sp = specs();
    let avr_spec = PathBuf::from(&sp).join("Ghidra/Processors/Atmel/data/languages/avr8.sla");
    let arm_spec = PathBuf::from(&sp).join("Ghidra/Processors/ARM/data/languages/ARM8_le.sla");
    assert!(avr_spec.exists() && arm_spec.exists(), "required processor spec missing; build specs before running integration tests");

    let avr_path = common::scratch_file("raw-avr-directives", "bin");
    std::fs::write(&avr_path, [0, 0, 0x08, 0x95]).unwrap();
    let avr = avr_path.to_string_lossy().into_owned();
    let (stdout, stderr, ok) = run_kuna(&[
        "decompile", &avr, "0x101", "--json", "--raw-image", "--target",
        "avr8:LE:16:default", "--base", "0x100", "--define-function",
        "0x101-0x102=bounded", "--assert", "comment 0x101 WORD_COMMENT", "--sleighpath",
        &sp,
    ]);
    assert!(ok, "word-addressed raw directives failed: {stderr}");
    assert!(stdout.contains("\"name\": \"bounded\""), "{stdout}");
    assert!(stdout.contains("\"address\": 257"), "{stdout}");
    assert!(stdout.contains("\"size\": 2"), "{stdout}");
    assert!(stdout.contains("/* WORD_COMMENT */"), "{stdout}");

    let (stdout, stderr, ok) = run_kuna(&[
        "decompile", &avr, "0x101", "--raw-image", "--target", "avr8:LE:16:default",
        "--base", "0x100", "--define-function", "0x101-0x102=bounded", "--assert",
        "comment 0x101 WORD_COMMENT", "--sleighpath", &sp,
    ]);
    assert!(ok, "word-addressed raw text directives failed: {stderr}");
    assert!(stdout.contains("bounded"), "{stdout}");
    assert!(stdout.contains("/* WORD_COMMENT */"), "{stdout}");
    std::fs::remove_file(avr_path).unwrap();

    let arm_path = common::scratch_file("raw-thumb-directives", "bin");
    std::fs::write(&arm_path, [0x07, 0x20, 0x70, 0x47]).unwrap();
    let arm = arm_path.to_string_lossy().into_owned();
    let (stdout, stderr, ok) = run_kuna(&[
        "decompile", &arm, "0x4001", "--json", "--raw-image", "--target",
        "ARM:LE:32:v4t:default", "--base", "0x4000", "--isa", "thumb", "--assert",
        "function 0x4001-0x4003=thumb_bounded", "--assert",
        "comment 0x4001 THUMB_COMMENT", "--sleighpath", &sp,
    ]);
    assert!(ok, "odd-Thumb raw directives failed: {stderr}");
    assert!(stdout.contains("\"name\": \"thumb_bounded\""), "{stdout}");
    assert!(stdout.contains("\"address\": 16384"), "{stdout}");
    assert!(stdout.contains("\"size\": 2"), "{stdout}");
    assert!(stdout.contains("/* THUMB_COMMENT */"), "{stdout}");

    let (stdout, stderr, ok) = run_kuna(&[
        "decompile", &arm, "0x4001", "--raw-image", "--target", "ARM:LE:32:v4t:default",
        "--base", "0x4000", "--isa", "thumb", "--assert",
        "function 0x4001-0x4003=thumb_bounded", "--assert",
        "comment 0x4001 THUMB_COMMENT", "--sleighpath", &sp,
    ]);
    assert!(ok, "odd-Thumb raw text directives failed: {stderr}");
    assert!(stdout.contains("thumb_bounded"), "{stdout}");
    assert!(stdout.contains("/* THUMB_COMMENT */"), "{stdout}");
    std::fs::remove_file(arm_path).unwrap();
}

#[test]
fn raw_text_decode_failure_is_not_reported_as_an_external() {
    let path = common::scratch_file("raw-truncated-x86", "bin");
    std::fs::write(&path, [0x90]).unwrap();
    let binary = path.to_string_lossy().into_owned();
    let sp = specs();
    let spec = PathBuf::from(&sp).join("Ghidra/Processors/x86/data/languages/x86-64.sla");
    assert!(spec.exists(), "required processor spec missing; build specs before running integration tests");

    let (stdout, stderr, ok) = run_kuna(&[
        "decompile", &binary, "0", "--raw-image", "--target", "x86:LE:64:default",
        "--base", "0", "--sleighpath", &sp,
    ]);
    assert!(!ok, "truncated mapped raw entry unexpectedly succeeded");
    assert!(!stdout.contains("external symbol"), "{stdout}");
    assert!(stderr.contains("Unable to load"), "{stderr}");
    std::fs::remove_file(path).unwrap();
}

#[test]
fn raw_text_unknown_format_hint_handles_a_leading_lt_byte() {
    let path = common::scratch_file("raw-leading-lt", "bin");
    std::fs::write(&path, [0x3c, 0x00, 0xc3]).unwrap();
    let binary = path.to_string_lossy().into_owned();
    let (_stdout, stderr, ok) = run_kuna(&["decompile", &binary, "0"]);
    assert!(!ok, "headerless input without raw flags unexpectedly loaded");
    assert!(
        stderr.contains("--raw-image") && stderr.contains("--target") && stderr.contains("--base"),
        "leading-< diagnostic omitted raw guidance: {stderr}"
    );
    std::fs::remove_file(path).unwrap();
}

#[test]
fn raw_image_rejects_missing_metadata_and_object_only_surfaces() {
    let path = common::scratch_file("raw-parser", "bin");
    std::fs::write(&path, [0x07, 0x20, 0x70, 0x47]).unwrap();
    let binary = path.to_string_lossy().into_owned();
    let target = "ARM:LE:32:v4t:default";
    let cases: &[(&[&str], &str)] = &[
        (&["functions", &binary, "--raw-image", "--base", "0", "--entry", "0"],
         "--raw-image requires --target"),
        (&["functions", &binary, "--raw-image", "--target", target, "--entry", "0"],
         "--raw-image requires --base"),
        (&["functions", &binary, "--raw-image", "--target", target, "--base", "0"],
         "--raw-image requires at least one"),
        (&["functions", &binary, "--base", "0"], "--base requires --raw-image"),
        (&["functions", &binary, "--entry", "0"], "--entry requires --raw-image"),
        (&["functions", &binary, "--raw-image", "--target", target, "--base", "0",
           "--functions", "main"], "not --functions"),
        (&["functions", &binary, "--raw-image", "--target", target, "--base", "0",
           "--addr", "main"], "invalid address"),
        (&["functions", &binary, "--raw-image", "--target", target, "--base", "0",
           "--addr", ".text+0"], "raw image entries must be numeric"),
        (&["functions", &binary, "--raw-image", "--target", target, "--base", "0",
           "--entry", "0", "--slice", "arm"], "--slice does not apply"),
        (&["decompile-all", &binary, "--raw-image", "--target", target, "--base", "0",
           "--entry", "0", "--summary"], "require object-file metadata"),
        (&["decompile-graph", &binary, "--raw-image", "--target", target, "--base", "0",
           "--entry", "0"], "not supported by decompile-graph"),
        (&["disassemble", &binary, "0", "--raw-image", "--target", target, "--base", "0"],
         "unknown option --raw-image"),
    ];
    for (args, expected) in cases {
        let (_stdout, stderr, ok) = run_kuna(args);
        assert!(!ok, "{args:?} unexpectedly succeeded");
        assert!(stderr.contains(expected), "{args:?}: expected {expected:?}, got {stderr:?}");
    }

    let (_stdout, stderr, ok) = run_kuna(&["functions", &binary]);
    assert!(!ok, "headerless input without raw flags unexpectedly loaded");
    assert!(stderr.contains("--raw-image") && stderr.contains("--base"),
            "unknown-format diagnostic omitted raw guidance: {stderr}");

    let (_stdout, stderr, ok) = run_kuna(&[
        "decompile", &binary, "main", "--raw-image", "--target", target, "--base", "0",
    ]);
    assert!(!ok, "named raw decompile entry unexpectedly succeeded");
    assert!(stderr.contains("requires a numeric entry"), "{stderr}");
    std::fs::remove_file(path).unwrap();
}

fn json_sizes(stdout: &str) -> Vec<u64> {
    let document: serde_json::Value = serde_json::from_str(stdout).expect("valid CLI JSON");
    document
        .get("functions")
        .and_then(serde_json::Value::as_array)
        .expect("function array")
        .iter()
        .map(|function| {
            function.get("size").and_then(serde_json::Value::as_u64).expect("numeric function size")
        })
        .collect()
}

/// (kuna, `functions-json-size`) The cheap inventory call must carry an extent,
/// so a caller can rank a binary's functions by weight without decompiling it.
///
/// The regression this pins is the *absence*: `functions --json` records used to
/// be `name`/`address`/`address_hex`/`aliases` only, so "decompile the three
/// biggest functions" cost a whole `decompile-all`. Vendored acceptance probe:
/// `tests/cli/functions-json-size.json`.
///
/// `aif_gap_x86_64` is the fixture the need was filed against — stripped, so its
/// extents come from the clip alone and not from any ELF `st_size`.
#[test]
fn functions_json_carries_a_ranking_extent() {
    let bin = repo_root()
        .join("decompiler/crates/kuna-analysis/tests/fixtures/aif_gap_x86_64")
        .to_str()
        .unwrap()
        .to_string();
    let (stdout, stderr, ok) =
        run_kuna(&["functions", &bin, "--json", "--sleighpath", &specs()]);
    if !ok {
        panic!("kuna functions failed: {stderr}");
    }
    let sizes = json_sizes(&stdout);
    let count = json_count(&stdout).expect("the inventory must report a count");
    assert_eq!(
        sizes.len(),
        count,
        "every one of the {count} inventory records must carry `size`:\n{stdout}"
    );
    // The point of the field: it must DISCRIMINATE. An all-zero (or all-equal)
    // column would satisfy "the key exists" while leaving the caller exactly as
    // unable to rank as before — which is how this shipped broken on
    // `decompile-all`, where `size` came from the requested flow bound and so was
    // 0 on every record.
    assert!(
        sizes.iter().any(|&s| s > 0),
        "the inventory extents are all zero, so nothing can be ranked:\n{stdout}"
    );
    assert!(
        sizes.iter().collect::<std::collections::BTreeSet<_>>().len() > 1,
        "the inventory extents are all equal, so nothing can be ranked:\n{stdout}"
    );
    // The `.plt.got` thunk at 0x1030 is 8 bytes and the big `.text` tail at
    // 0x13c9 is 682: a thunk must not read as heavy as a real function.
    assert!(
        stdout.contains("\"address_hex\": \"0x1030\"") && sizes.contains(&8),
        "the 8-byte `.plt.got` thunk must report its real extent:\n{stdout}"
    );
    assert!(
        sizes.iter().any(|&s| s > 512),
        "the large `.text` function must outrank the thunks:\n{stdout}"
    );
}

/// (kuna, `functions-json-size`) `functions` and `decompile-all` must report the
/// SAME extent for the same entry — one field name, one meaning.
///
/// `decompile-all`'s `size` used to come from `Funcdata::get_size()`, which is
/// the *requested* flow bound (always "unbounded", i.e. 0, on a whole-binary
/// run), so the field was structurally dead on every record. Copying that into
/// the inventory would have satisfied the letter of the need and none of it.
#[test]
fn functions_and_decompile_all_agree_on_size() {
    let bin = repo_root()
        .join("decompiler/crates/kuna-analysis/tests/fixtures/aif_gap_x86_64")
        .to_str()
        .unwrap()
        .to_string();
    let sp = specs();
    let (inventory, stderr, ok) =
        run_kuna(&["functions", &bin, "--json", "--sleighpath", &sp]);
    if !ok {
        panic!("kuna functions failed: {stderr}");
    }
    let (decompiled, stderr, ok) =
        run_kuna(&["decompile-all", &bin, "--json", "--sleighpath", &sp]);
    if !ok {
        panic!("kuna decompile-all failed: {stderr}");
    }
    // Both documents are address-ordered over the same entry set, so the size
    // columns line up positionally.
    let want = json_sizes(&inventory);
    let got = json_sizes(&decompiled);
    assert_eq!(
        want, got,
        "the inventory and the whole-binary run disagree on function extents"
    );
}

/// (DIV-120) A function past the instruction budget reports the body kuna DID
/// decode, not nothing.  `--option maxinstruction 5` puts `fauxware`'s `main` in
/// the state the 1.8M-instruction obfuscated checker of
/// `docs/re-needs/checker-exceeds-instruction-ceiling.md` is in by default: the
/// decompiling surfaces clear `error_toomanyinstructions`, so the overrun
/// truncates the flow under a warning header that names the knob instead of
/// failing the function with `code: null`.  Naming the option explicitly still
/// restores the upstream hard failure — that is the second pass.
#[test]
fn instruction_budget_overrun_truncates_instead_of_failing() {
    let bin = fauxware();
    let sp = specs();
    let budget = ["decompile-all", &bin, "--functions", "main", "--json", "--sleighpath", &sp,
                  "--option", "maxinstruction", "5"];
    let (truncated, stderr, ok) = run_kuna(&budget);
    if !ok {
        panic!("kuna decompile-all failed: {stderr}");
    }
    assert!(
        truncated.contains("Exceeded the 5 instruction budget"),
        "the truncated body must carry the budget warning header:\n{truncated}"
    );
    assert!(
        truncated.contains("--option maxinstruction N"),
        "the warning must name the knob that raises the budget:\n{truncated}"
    );
    assert!(
        !truncated.contains("Flow exceeded maximum allowable instructions"),
        "the overrun must not be reported as a failure:\n{truncated}"
    );

    // Same run, upstream's policy named back on: the function fails outright and
    // carries no code, which is what every CLI decompile used to do.
    let mut fatal = budget.to_vec();
    fatal.extend_from_slice(&["--option", "errortoomanyinstructions", "on"]);
    let (failed, stderr, ok) = run_kuna(&fatal);
    assert!(!ok, "an all-failed batch must exit nonzero: {failed}");
    assert!(
        failed.contains("Flow exceeded maximum allowable instructions")
            && failed.contains("\"code\": null")
            && failed.contains("\"error\": \"decompilation produced zero function bodies"),
        "`--option errortoomanyinstructions on` must restore the hard failure:\n{failed}"
    );
    assert!(
        stderr.contains("decompilation produced zero function bodies")
            && stderr.contains("per-function error record"),
        "the run-level failure was not reported on stderr: {stderr}"
    );
}

/// A selected set with no body is a failed RUN, after its complete per-function
/// records have been emitted. One usable body keeps the batch recoverable even
/// when another function failed.
#[test]
fn aggregate_exit_distinguishes_all_failed_from_partial_success() {
    let bin = fauxware();
    let sp = specs();
    let fatal = [
        "--option",
        "maxinstruction",
        "5",
        "--option",
        "errortoomanyinstructions",
        "on",
        "--sleighpath",
        &sp,
    ];

    let mut text_args = vec!["decompile-all", &bin, "--functions", "main"];
    text_args.extend_from_slice(&fatal);
    let (stdout, stderr, ok) = run_kuna(&text_args);
    assert!(!ok, "an all-failed text batch exited zero");
    assert!(
        stdout.contains("// Function: main @ 0x40071d")
            && stdout.contains("Flow exceeded maximum allowable instructions"),
        "the failed function record was not preserved: {stdout}"
    );
    assert!(stderr.contains("zero function bodies"), "no run-level diagnostic: {stderr}");

    let mut mixed_args = vec![
        "decompile-all",
        &bin,
        "--functions",
        "main,__libc_csu_fini",
        "--json",
    ];
    mixed_args.extend_from_slice(&fatal);
    let (stdout, stderr, ok) = run_kuna(&mixed_args);
    assert!(ok, "a partial-success batch must remain recoverable: {stderr}");
    assert!(stdout.contains("\"error\": null"), "partial run gained a top-level error: {stdout}");
    assert!(
        stdout.contains("Flow exceeded maximum allowable instructions")
            && stdout.contains("void __libc_csu_fini(void)"),
        "the mixed control needs one error and one body: {stdout}"
    );
}

/// Fast discovery must not depend on the Listing carrying disassembly text.
///
/// `--mode fast` builds the Listing for `fast_funcdisc` alone, and that walk
/// captures no assembly text — nothing left in the mode reads it except AIF's
/// prologue fingerprint, which re-decodes the two instructions it needs. Drop that
/// fallback and the fingerprint histogram comes back empty, which silently takes
/// every pointer-validated function with it: on `aif_gap_x86_64` the target
/// reachable only through a function-pointer table (`0x13ae`) simply stops being
/// enumerated.
#[test]
fn fast_discovery_finds_the_pointer_only_target() {
    let bin = repo_root()
        .join("decompiler/crates/kuna-analysis/tests/fixtures/aif_gap_x86_64")
        .to_str()
        .unwrap()
        .to_string();
    let sp = specs();
    let args = ["functions", &bin, "--json", "--sleighpath", &sp, "--mode", "fast"];
    let (stdout, stderr, ok) = run_kuna(&args);
    if !ok {
        panic!("kuna functions failed: {stderr}");
    }
    let fast = json_addresses(&stdout);
    assert!(
        fast.contains(&0x13ae),
        "`--mode fast` must enumerate the pointer-only function 0x13ae: {fast:x?}"
    );

    // The control: with the fast walk off, nothing finds it.
    let mut off = args.to_vec();
    off.extend_from_slice(&["--option", "fast_funcdisc", "off"]);
    let (stdout, stderr, ok) = run_kuna(&off);
    assert!(ok, "kuna functions failed: {stderr}");
    assert!(
        !json_addresses(&stdout).contains(&0x13ae),
        "0x13ae must come from the fast walk alone:\n{stdout}"
    );
}

// --- `--jobs N`: the worker pool ---------------------------------------------

fn protoorder_fixture() -> String {
    repo_root()
        .join("decompiler/crates/kuna-analysis/tests/fixtures/protoorder_x86_64")
        .to_str()
        .unwrap()
        .to_string()
}

/// The pool's whole contract: the document must not depend on how many processes
/// produced it, or on how the work was cut up between them.  Every job count and
/// chunk size here has to agree with the serial run byte for byte — dispatch
/// order is longest-first, which is deliberately not output order, so a
/// positional merge is the only thing that can make this hold.
///
/// Both runs pass `--option protoorder off`: the serial default decompiles
/// callees first and a worker cannot see another worker's callees
/// ([`jobs_notes_that_the_default_callee_first_order_is_serial_only`]).  The
/// protoorder fixture is one where the default does move the output, so the
/// pin is not vacuous.
#[test]
fn jobs_output_is_byte_identical_to_serial() {
    let sp = specs();
    for bin in [fauxware(), protoorder_fixture()] {
        let base = ["decompile-all", &bin, "--json", "--max-fn-seconds", "0", "--sleighpath", &sp,
            "--option", "protoorder", "off"];
        let (want, stderr, ok) = run_kuna(&base);
        if !ok {
            panic!("kuna decompile-all failed: {stderr}");
        }
        assert!(want.contains("\"code\""), "the serial run decompiled nothing:\n{want}");

        for (jobs, chunk) in [("2", None), ("3", Some("1")), ("4", Some("7")), ("8", Some("1000"))] {
            let mut args = base.to_vec();
            args.extend_from_slice(&["--jobs", jobs]);
            if let Some(chunk) = chunk {
                args.extend_from_slice(&["--jobs-chunk", chunk]);
            }
            let (got, stderr, ok) = run_kuna(&args);
            assert!(ok, "kuna decompile-all --jobs {jobs} failed: {stderr}");
            assert_eq!(got, want, "--jobs {jobs} (chunk {chunk:?}) moved the document of {bin}");
            // A run that can take an hour has to say where it is, and it has to say
            // it on stderr — stdout is the document, byte-compared just above.
            assert!(
                stderr.contains("[kuna --jobs]") && stderr.contains("worker process(es)"),
                "--jobs {jobs} reported no plan on stderr:\n{stderr}"
            );
            assert!(
                stderr.contains("[kuna --jobs] done:"),
                "--jobs {jobs} never reported completion:\n{stderr}"
            );
            assert!(!stderr.contains("callee-first"), "protoorder off still noted:\n{stderr}");
        }
    }
}

/// With the default `protoorder cycles` (on this acyclic fixture the same as
/// `types`), a serial run types `caller`'s argument
/// from `callee`'s own recovery and a pool run cannot: the pool says so on
/// stderr instead of silently producing a different document.
#[test]
fn jobs_notes_that_the_default_callee_first_order_is_serial_only() {
    let bin = protoorder_fixture();
    let sp = specs();
    let base = ["decompile-all", &bin, "--max-fn-seconds", "0", "--sleighpath", &sp];
    let (serial, stderr, ok) = run_kuna(&base);
    if !ok {
        panic!("kuna decompile-all failed: {stderr}");
    }
    assert!(!stderr.contains("callee-first"), "a serial run printed the pool note:\n{stderr}");
    assert!(serial.contains("caller(unsigned char *a0,int a1)"), "{serial}");
    let mut pooled = base.to_vec();
    pooled.extend_from_slice(&["--jobs", "2"]);
    let (got, stderr, ok) = run_kuna(&pooled);
    assert!(ok, "kuna decompile-all --jobs 2 failed: {stderr}");
    assert!(
        stderr.contains("--jobs decompiles without the callee-first order"),
        "the pool did not say its output can differ:\n{stderr}"
    );
    assert!(got.contains("caller(unsigned long a0,int a1)"), "{got}");
}

/// A narrowed run skips the callee-first order (and its call-graph build) by
/// default, silently; naming the option orders the selection and says that a
/// callee outside it states nothing.
#[test]
fn a_narrowed_run_orders_callees_first_only_when_asked() {
    let bin = protoorder_fixture();
    let sp = specs();
    let base = ["decompile-all", &bin, "--functions", "caller,callee", "--sleighpath", &sp];
    let (plain, stderr, ok) = run_kuna(&base);
    if !ok {
        panic!("kuna decompile-all failed: {stderr}");
    }
    assert!(!stderr.contains("protoorder"), "a default narrowed run printed a note:\n{stderr}");
    assert!(plain.contains("caller(unsigned long a0,int a1)"), "{plain}");
    let mut asked = base.to_vec();
    asked.extend_from_slice(&["--option", "protoorder", "types"]);
    let (got, stderr, ok) = run_kuna(&asked);
    assert!(ok, "{stderr}");
    assert!(stderr.contains("note: --option protoorder: 2 of this binary's entries selected"), "{stderr}");
    assert!(got.contains("caller(unsigned char *a0,int a1)"), "{got}");
}

/// (kuna `protoorder cycles`) A function that calls itself, or sits in a
/// two-member cycle, states its recovered types under `cycles` and nothing under
/// `types`.  The one call whose arity moves is `argclobber`'s drop of a clobbered
/// trailing argument at a recursive callee whose stated list and body both say
/// the register is free; a callee that forwards the register into its own
/// recursion keeps the argument under both values. `calleevote` is off: by
/// default it gives `wrap` and `wrap2` the `char *` their one caller passes,
/// the other direction.
#[test]
fn recursive_callees_state_their_types_under_cycles() {
    let bin = repo_root()
        .join("decompiler/crates/kuna-analysis/tests/fixtures/protoorder_cycles_x86_64")
        .to_str()
        .unwrap()
        .to_string();
    let sp = specs();
    for (value, param, rcall) in [
        ("types", "(unsigned long a0)", "rtarget(a0,5,v3);"),
        ("cycles", "(char *a0)", "rtarget(a0,5);"),
    ] {
        let (got, stderr, ok) =
            run_kuna(&[
                "decompile-all", &bin, "--sleighpath", &sp, "--option", "protoorder", value, "--option",
                "calleevote", "off",
            ]);
        if !ok {
            panic!("kuna decompile-all --option protoorder {value} failed: {stderr}");
        }
        for f in ["wrap", "wrap2"] {
            assert!(got.contains(&format!(" {f}{param}")), "{value}: {f}{param} missing:\n{got}");
        }
        assert!(got.contains(rcall), "{value}: {rcall} missing:\n{got}");
        assert!(got.contains("rkeep(a0,5,v3);"), "{value}: rkeep lost its forwarded argument:\n{got}");
    }
}

/// (kuna `calleevote`) A caller's frame record whose first member is a
/// `char *` is passed as a `char **`. `add` writes a node through it whose word
/// stores a `char *` would print one character at a time, and `drop` reads past
/// the first member, so neither takes it; `advance(&cursor)` does. `mkpipe`
/// keeps the `int *` that `pipe` declares instead of a one-field record.
#[test]
fn a_frame_records_char_pointer_pointer_is_not_its_type() {
    let bin = repo_root()
        .join("decompiler/crates/kuna-analysis/tests/fixtures/calleevote_frame_x86_64")
        .to_str()
        .unwrap()
        .to_string();
    let sp = specs();
    for value in ["types", "fields"] {
        let (got, stderr, ok) =
            run_kuna(&["decompile-all", &bin, "--sleighpath", &sp, "--option", "calleevote", value]);
        if !ok {
            panic!("kuna decompile-all --option calleevote {value} failed: {stderr}");
        }
        assert!(got.contains("v2[1] = 0x506070801020304;"), "{value}: the node's word store split:\n{got}");
        assert!(!got.contains("] = '\\x"), "{value}: a character store:\n{got}");
        assert!(!got.contains("drop(char **a0)"), "{value}: drop took the frame char **:\n{got}");
        assert!(got.contains("int advance(char **a0)"), "{value}: advance lost its char **:\n{got}");
        assert!(got.contains("int mkpipe(int *a0)"), "{value}: mkpipe lost pipe's int *:\n{got}");
    }
}

/// (kuna `protoorder cycles` + `structsynth`) The convergence sweep decompiles a
/// self-recursive function again once a later layout supersedes the structure
/// its first decompile minted.  The redo must not read the statement that first
/// decompile made: at its own recursive call it typed the child pointer as the
/// superseded `struct_0` while its parameter took the survivor.  `walk` names
/// one structure, the one `look` names too, under both values.
#[test]
fn a_redone_recursive_function_reads_no_statement_of_its_own() {
    let bin = repo_root()
        .join("decompiler/crates/kuna-analysis/tests/fixtures/protoorder_cyclestruct_x86_64")
        .to_str()
        .unwrap()
        .to_string();
    let sp = specs();
    for value in ["types", "cycles"] {
        let (got, stderr, ok) =
            run_kuna(&["decompile-all", &bin, "--sleighpath", &sp, "--option", "protoorder", value]);
        if !ok {
            panic!("kuna decompile-all --option protoorder {value} failed: {stderr}");
        }
        let chunk = |name: &str| -> String {
            got.split("// Function: ")
                .find(|c| c.starts_with(&format!("{name} @")))
                .unwrap_or_else(|| panic!("{value}: no `{name}` in:\n{got}"))
                .to_string()
        };
        let structs = |text: &str| -> std::collections::BTreeSet<String> {
            let mut out = std::collections::BTreeSet::new();
            let mut rest = text;
            while let Some(at) = rest.find("struct_") {
                let digits: String =
                    rest[at + 7..].chars().take_while(|c| c.is_ascii_digit()).collect();
                if !digits.is_empty() {
                    out.insert(format!("struct_{digits}"));
                }
                rest = &rest[at + 7..];
            }
            out
        };
        let walk = structs(&chunk("walk"));
        let look = structs(&chunk("look"));
        assert_eq!(walk.len(), 1, "{value}: walk names more than one structure:\n{}", chunk("walk"));
        assert_eq!(walk, look, "{value}: walk and look name different structures:\n{got}");
        assert!(!chunk("walk").contains(" *)"), "{value}: walk casts a pointer:\n{}", chunk("walk"));
    }
}

/// The functions of a `decompile-all` document whose headers start with one of
/// `names`, each `struct_N` typedef and definition that `structdefs` printed
/// above them kept once and hoisted, so the set compiles as one file.
fn printed_functions(stdout: &str, names: &[&str]) -> String {
    let mut defs: Vec<String> = Vec::new();
    let mut bodies = String::new();
    for chunk in stdout.split("// Function: ").filter(|c| names.iter().any(|n| c.starts_with(n))) {
        let mut lines = chunk.lines();
        bodies.push_str("// ");
        while let Some(line) = lines.next() {
            if line.starts_with("typedef struct struct_") {
                let def = format!("{line}\n");
                if !defs.contains(&def) {
                    defs.insert(0, def);
                }
            } else if line.starts_with("struct struct_") && line.ends_with('{') {
                let mut def = format!("{line}\n");
                for l in lines.by_ref() {
                    def.push_str(l);
                    def.push('\n');
                    if l == "};" {
                        break;
                    }
                }
                if !defs.contains(&def) {
                    defs.push(def);
                }
            } else {
                bodies.push_str(line);
                bodies.push('\n');
            }
        }
        bodies.push('\n');
    }
    format!("{}{bodies}", defs.concat())
}

/// On MIPS o32 `h(int, float)` takes its float in a general register, and each
/// `g*` hands it a word's bits while also adding, comparing, truncating or
/// storing that word as an integer.  A float vote there printed `(int)v1 + 3`,
/// `(short)((unsigned int)v1 >> 0x10)` and `a2[1] = (int)v1` -- value conversions
/// where the machine moves bits -- so it is refused.  The round trip compiles the
/// seven printed callers (`-no-pie`, so the array's address survives the printed
/// 32-bit `(int)a1 + 0xc`) against a bit-preserving `h` and compares them with the
/// source.
#[test]
fn a_float_in_a_general_register_keeps_its_integer_uses_round_trip() {
    let bin = repo_root()
        .join("decompiler/crates/kuna-analysis/tests/fixtures/protoorder_floatgpr_mipsel")
        .to_str()
        .unwrap()
        .to_string();
    let sp = specs();
    let (stdout, stderr, ok) =
        run_kuna(&["decompile-all", &bin, "--option", "structdefs", "on", "--sleighpath", &sp]);
    assert!(ok, "kuna decompile-all failed: {stderr}");
    for want in [
        "return (int)h(a0,v1) + v1 + 3;",
        "(unsigned int)(v1 < 0x3fc00000)",
        "if (v1 == 0x3fc00001)",
        "(unsigned int)(0x3fc00000 < v1)",
        "*a2 = (short)((unsigned int)v1 >> 0x10);",
        "a2[1] = (char)((unsigned int)v1 >> 0x10);",
    ] {
        assert!(stdout.contains(want), "missing `{want}`:\n{stdout}");
    }
    assert!(
        stdout.contains("a2[1] = v1;") || stdout.contains("a2->field_0x4 = v1;"),
        "missing the bitwise store of v1:\n{stdout}"
    );
    for bad in ["float v1;", "1.5000001"] {
        assert!(!stdout.contains(bad), "`{bad}` printed:\n{stdout}");
    }
    let converts_v1 = |cast: &str, shift_ok: bool| {
        stdout.match_indices(cast).any(|(i, m)| {
            let rest = &stdout[i + m.len()..];
            !rest.starts_with(|c: char| c.is_ascii_digit()) && !(shift_ok && rest.starts_with(" >>"))
        })
    };
    assert!(!converts_v1("(int)v1", false), "`(int)v1` printed:\n{stdout}");
    assert!(!converts_v1("(unsigned int)v1", true), "`(unsigned int)v1` printed:\n{stdout}");

    let compilers: Vec<&str> = ["gcc", "clang"]
        .into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    assert!(!compilers.is_empty(), "protoorder float-in-GPR round trip requires a C compiler");
    let printed = printed_functions(&stdout, &["g3 ", "g5 ", "g6 ", "g9 ", "g20 ", "g22 ", "g24 "]);
    let dir = std::env::temp_dir().join(format!("kuna-protoorder-floatgpr-rt-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let src = dir.join("rt.c");
    let exe = dir.join("rt");
    std::fs::write(
        &src,
        format!(
            "#include <stdio.h>\n#include <string.h>\n\
             static float hs(int k, float x) {{ return x * 2.5f + (float)k; }}\n\
             static float h(int k, int bits) {{ float x; memcpy(&x, &bits, 4); return hs(k, x); }}\n\
             {printed}\n\
             static float fb(int b) {{ float f; memcpy(&f, &b, 4); return f; }}\n\
             static int s3(int k, int *p) {{ int b = p[3]; return (int)hs(k, fb(b)) + b + 3; }}\n\
             static int s5(int k, int *p) {{ int b = p[3]; return (int)hs(k, fb(b)) + (b < 0x3fc00000); }}\n\
             static int s6(int k, int *p) {{ int b = p[3]; return (int)hs(k, fb(b)) + (b == 0x3fc00001) * 100; }}\n\
             static unsigned s9(int k, unsigned *p) {{ unsigned b = p[3]; \
             return (unsigned)hs(k, fb((int)b)) + (b > 0x3fc00000u); }}\n\
             static int s20(int k, int *p, unsigned short *q) {{ int b = p[3]; int r = (int)hs(k, fb(b)); \
             *q = (short)(b >> 16); return r; }}\n\
             static int s22(int k, int *p, char *q) {{ int b = p[3]; int r = (int)hs(k, fb(b)); \
             q[0] = (char)(b >> 8); q[1] = (char)(b >> 16); return r; }}\n\
             static unsigned s24(int k, int *p, int *q) {{ int b = p[3]; float r = hs(k, fb(b)); \
             q[1] = b; q[0] = (int)r; return 0; }}\n\
             static int arr[4] = {{1, 2, 3, 0x3fc00001}};\n\
             static int bits[4] = {{1, 2, 3, 0x3fc01234}};\n\
             static void run(int use_printed) {{\n  \
             void *a = arr, *b = bits;\n  \
             unsigned short s = 0; char c[2] = {{0, 0}}; int q[2] = {{0, 0}}; int r20, r22, r24;\n  \
             if (use_printed) {{\n    \
             printf(\"%d %d %d %u \", g3(7, a), g5(7, a), g6(7, a), (unsigned)g9(7, a));\n    \
             r20 = g20(7, b, &s); r22 = g22(7, b, c); r24 = (int)g24(7, b, (void *)q);\n  \
             }} else {{\n    \
             printf(\"%d %d %d %u \", s3(7, arr), s5(7, arr), s6(7, arr), s9(7, (unsigned *)arr));\n    \
             r20 = s20(7, bits, &s); r22 = s22(7, bits, c); r24 = (int)s24(7, bits, q);\n  \
             }}\n  \
             printf(\"%d %04x %d %02x %02x %d %d %x\\n\", r20, s, r22, (unsigned char)c[0], (unsigned char)c[1], \
             r24, q[0], q[1]);\n\
             }}\n\
             int main(void) {{\n  run(1);\n  run(0);\n  return 0;\n}}\n"
        ),
    )
    .unwrap();
    for cc in &compilers {
        for level in ["-O0", "-O2"] {
            let compiled = Command::new(cc)
                .args(["-std=gnu11", "-w", "-fno-pie", "-no-pie", level])
                .args(["-o", exe.to_str().unwrap(), src.to_str().unwrap()])
                .output()
                .expect("spawn the C compiler");
            assert!(
                compiled.status.success(),
                "{cc} {level} rejected the printed callers:\n{}",
                String::from_utf8_lossy(&compiled.stderr)
            );
            let run = process::required_output(&mut Command::new(&exe));
            let got = String::from_utf8_lossy(&run.stdout);
            let lines: Vec<&str> = got.lines().collect();
            assert_eq!(lines.len(), 2, "{cc} {level}: {got}");
            assert_eq!(lines[0], lines[1], "{cc} {level}: printed callers compute different values:\n{printed}");
        }
    }
    let _ = std::fs::remove_dir_all(&dir);
}

/// The compilers a round trip is run with: every one of `gcc` and `clang` that
/// runs here, or `cc` when neither does.
fn round_trip_compilers() -> Vec<&'static str> {
    let runs = |cc: &str| process::optional_output(Command::new(cc).arg("--version")).is_some();
    let found: Vec<&'static str> = ["gcc", "clang"].into_iter().filter(|cc| runs(cc)).collect();
    if found.is_empty() && runs("cc") { vec!["cc"] } else { found }
}

/// Compile `src` with each of [`round_trip_compilers`] (a pointer handed to an
/// integer, or an integer to a pointer, is an error, as it is under CI's gcc 13
/// and clang 18), run it, and return each compiler's stdout.
fn compile_and_run_each(tag: &str, src: &str) -> Vec<(String, String)> {
    let dir = std::env::temp_dir().join(format!("kuna-{tag}-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let file = dir.join("rt.c");
    std::fs::write(&file, src).unwrap();
    let mut out = Vec::new();
    for cc in round_trip_compilers() {
        let exe = dir.join(format!("rt-{cc}"));
        let built = Command::new(cc)
            .args(["-std=gnu11", "-w", "-Werror=int-conversion", "-Werror=implicit-function-declaration", "-o"])
            .arg(&exe)
            .arg(&file)
            .output()
            .expect("spawn the compiler");
        assert!(built.status.success(), "{cc} rejected the printed C:\n{}\n{src}", String::from_utf8_lossy(&built.stderr));
        let run = Command::new(&exe).output().expect("run the round trip");
        out.push((cc.to_string(), String::from_utf8_lossy(&run.stdout).trim().to_string()));
    }
    let _ = std::fs::remove_dir_all(&dir);
    out
}

/// `size` bytes of the fixture image at virtual address `vaddr`.
fn image_bytes(path: &str, vaddr: u64, size: usize) -> Vec<u8> {
    use object::{Object, ObjectSection};
    let data = std::fs::read(path).unwrap();
    let file = object::File::parse(data.as_slice()).unwrap();
    let section = file
        .sections()
        .find(|s| s.address() <= vaddr && vaddr + size as u64 <= s.address() + s.size())
        .unwrap_or_else(|| panic!("no section holds 0x{vaddr:x}"));
    let at = (vaddr - section.address()) as usize;
    section.data().unwrap()[at..at + size].to_vec()
}

/// The bits of a value of any printed type, for a round trip that compares
/// what a function hands back whatever type it was printed with.
const BITS: &str = "#define BITS(e) ({ __typeof__(e) r_ = (e); unsigned long long u_ = 0; \
                    memcpy(&u_, &r_, sizeof r_ < 8 ? sizeof r_ : 8); u_; })\n";

/// `floatret_x86_64` (clang -O0): `qnan` returns `nanf("")` through the import
/// stub, `pick` returns `x < 0 ? qnan() : x * 2`, `wrapd` and `wrapi` hand back
/// what their callee returned (`call; ret`), and `getf`/`getd` return a global
/// in `xmm0`. Before, `qnan`, `wrapd` and `wrapi` were `void` while their callers
/// read the result -- `v1 = (float)sub_1150()`, which no compiler accepts. The
/// stub's jump returns the stub's float without a conversion, and `getf` stays
/// `unsigned int`: one declaration of the global serves every function, and
/// another may read it as an integer. `idf` returns its argument and stays an
/// `unsigned int` of one: what it receives is its callers' to type. The printed
/// functions, the stub included, are compiled against the fixture's own data
/// and hand back the bits the fixture computes.
#[test]
fn a_float_register_return_and_a_read_void_result_round_trip() {
    let bin = repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/floatret_x86_64").to_str().unwrap().to_string();
    let sp = specs();
    let (stdout, stderr, ok) = run_kuna(&["decompile-all", &bin, "--sleighpath", &sp]);
    assert!(ok, "kuna decompile-all failed: {stderr}");
    for want in [
        "float nanf(void)",
        "float sub_1150(void)",
        "return nanf((char *)0x203d);",
        "v1 = sub_1150();",
        "unsigned int sub_11c0(void)",
        "unsigned long sub_11d0(void)",
        "unsigned long sub_11e0(void)",
        "return sub_11d0();",
        "unsigned int sub_11f0(unsigned int a0)",
        "int sub_1210(void)",
        "return sub_1200();",
    ] {
        assert!(stdout.contains(want), "missing `{want}`:\n{stdout}");
    }
    for bad in ["(float)(*dat_4018)", "(float)sub_1150"] {
        assert!(!stdout.contains(bad), "`{bad}` printed:\n{stdout}");
    }
    let names = ["nanf ", "sub_1150 ", "sub_1170 ", "sub_11c0 ", "sub_11d0 ", "sub_11e0 ", "sub_11f0 ", "sub_1200 ", "sub_1210 "];
    let printed = printed_functions(&stdout, &names);
    let mut globals = String::new();
    let mut init = String::new();
    for (name, ty, addr, size) in [
        ("dat_2004", "float", 0x2004u64, 4usize),
        ("dat_4038", "unsigned int", 0x4038, 4),
        ("dat_4040", "unsigned long", 0x4040, 8),
        ("dat_4048", "int", 0x4048, 4),
    ] {
        let bytes: Vec<String> = image_bytes(&bin, addr, size).iter().map(|b| b.to_string()).collect();
        globals.push_str(&format!("{ty} {name};\nstatic const unsigned char {name}_b[{size}] = {{{}}};\n", bytes.join(",")));
        init.push_str(&format!("  memcpy(&{name}, {name}_b, sizeof {name});\n"));
    }
    // The stub takes no parameters (its jump reads none); its caller passes nanf's.
    let src = format!(
        "#include <stdio.h>\n#include <string.h>\n{BITS}#define nanf(...) nanf_stub()\n\
         static float target(void) {{ return __builtin_nanf(\"\"); }}\nfloat (*dat_4018)(void) = target;\n{globals}{printed}\n\
         int main(void) {{\n{init}  printf(\"%llx %llx %llx %llx %llx %llx %llx\\n\", BITS(sub_1170(3.0f)), BITS(sub_1170(-3.0f)), \
         BITS(sub_11c0()), BITS(sub_11d0()), BITS(sub_11e0()), BITS(sub_11f0({})), BITS(sub_1210()));\n  return 0;\n}}\n",
        arg_of_bits(&printed, "sub_11f0", 0, 0x3fa0_0000)
    );
    for (cc, got) in compile_and_run_each("floatret-x86_64", &src) {
        assert_eq!(
            got, "40c00000 7fc00000 3fc00000 4002000000000000 4002000000000000 3fa00000 2a",
            "{cc}: the printed C computes something else:\n{printed}"
        );
    }
}

/// `floatret_wrap_{gcc,clang}_O0`: `set_tz` and `restore_cwd` return what one
/// of two calls returns in `eax`, `gi_as_f`/`set_gi_bits` move a float's bits
/// through the `int` global `gi` that `use_gi` computes with, and `wrapneg`,
/// `wrapabs`, `wrapnegd` and `twice` hand back the result of a callee whose
/// recovery computes on the bits (`xorps`, `andps`) and returns an integer.
/// Before, `set_tz` returned the rest of `rax` it never set (`unsigned long`,
/// `return v2;`), `gi_as_f` returned `gi` converted to a float, and the wrappers
/// returned `(float)absf(a0)` -- each a value the binary does not compute. The
/// printed functions are compiled and must hand back the fixture's bits.
#[test]
fn a_wrapper_returns_its_callees_result_round_trip() {
    let sp = specs();
    let fixture = |name: &str| repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures").join(name).to_str().unwrap().to_string();
    let bin = fixture("floatret_wrap_gcc_O0");
    let (stdout, stderr, ok) = run_kuna(&["decompile-all", &bin, "--sleighpath", &sp]);
    assert!(ok, "kuna decompile-all failed: {stderr}");
    for want in ["int set_tz(char *a0)", "int restore_cwd(int a0,char *a1)", "unsigned int gi_as_f(void)", "void set_gi_bits(unsigned int a0)"] {
        assert!(stdout.contains(want), "missing `{want}`:\n{stdout}");
    }
    let printed = printed_functions(&stdout, &["set_tz ", "chdir_long ", "restore_cwd ", "gi_as_f ", "set_gi_bits ", "use_gi "]);
    assert!(!printed.contains("CONCAT"), "a return pieces in bits no path sets:\n{printed}");
    let src = format!(
        "#include <stdio.h>\n#include <stdlib.h>\n#include <string.h>\n#include <unistd.h>\n{BITS}int gi = 0x3fc00000;\n{printed}\n\
         int main(void) {{\n  int r = set_tz(NULL), s = restore_cwd(-1, \"/tmp\");\n  unsigned long long g = BITS(gi_as_f());\n  \
         set_gi_bits(0xc0f00000u);\n  printf(\"%d %d %llx %x\\n\", r, s, g, use_gi());\n  return 0;\n}}\n"
    );
    for (cc, got) in compile_and_run_each("floatret-wrap-gcc", &src) {
        assert_eq!(got, "0 3 3fc00000 c0f00001", "{cc}: the printed C computes something else:\n{printed}");
    }

    let bin = fixture("floatret_wrap_clang_O0");
    let (stdout, stderr, ok) = run_kuna(&["decompile-all", &bin, "--sleighpath", &sp]);
    assert!(ok, "kuna decompile-all failed: {stderr}");
    for want in [
        "unsigned int wrapneg(unsigned int a0)",
        "unsigned int wrapabs(unsigned int a0)",
        "unsigned long wrapnegd(unsigned long a0)",
        "unsigned int twice(unsigned int a0)",
    ] {
        assert!(stdout.contains(want), "missing `{want}`:\n{stdout}");
    }
    let names = ["negf ", "absf ", "negd ", "wrapneg ", "wrapabs ", "wrapnegd ", "twice "];
    let printed = printed_functions(&stdout, &names);
    for bad in ["(float)", "(double)"] {
        assert!(!printed.contains(bad), "`{bad}` converts a callee's bits:\n{printed}");
    }
    let mut globals = String::new();
    let mut init = String::new();
    for line in printed.lines() {
        for word in line.split(|c: char| !c.is_ascii_alphanumeric() && c != '_') {
            let Some(addr) = word.strip_prefix("dat_").and_then(|h| u64::from_str_radix(h, 16).ok()) else { continue };
            if globals.contains(&format!(" {word};")) {
                continue;
            }
            let bytes: Vec<String> = image_bytes(&bin, addr, 8).iter().map(|b| b.to_string()).collect();
            globals.push_str(&format!("unsigned long {word};\nstatic const unsigned char {word}_b[8] = {{{}}};\n", bytes.join(",")));
            init.push_str(&format!("  memcpy(&{word}, {word}_b, 8);\n"));
        }
    }
    let src = format!(
        "#include <stdio.h>\n#include <string.h>\n{BITS}{globals}{printed}\n\
         int main(void) {{\n{init}  printf(\"%llx %llx %llx %llx\\n\", BITS(wrapneg(0x3fc00000u)) & 0xffffffff, \
         BITS(wrapabs(0xbfa00000u)) & 0xffffffff, BITS(wrapnegd(0x3ff8000000000000ul)), BITS(twice(0x40400000u)) & 0xffffffff);\n  return 0;\n}}\n"
    );
    for (cc, got) in compile_and_run_each("floatret-wrap-clang", &src) {
        assert_eq!(got, "bfc00000 3fa00000 bff8000000000000 40400000", "{cc}: the printed C computes something else:\n{printed}");
    }

    // gcc -O2 tail-jumps to the callee; the wrapper takes no parameter of its own
    // (a separate gap), so this one is checked as text: the callee's integer bits
    // are returned as they are, not converted.
    let bin = fixture("floatret_wrap_gcc_O2");
    let (stdout, stderr, ok) = run_kuna(&["decompile-all", &bin, "--sleighpath", &sp]);
    assert!(ok, "kuna decompile-all failed: {stderr}");
    for want in ["unsigned long wrapneg(void)", "return negf(); // tail-call", "unsigned long wrapnegd(void)", "return negd(); // tail-call"] {
        assert!(stdout.contains(want), "missing `{want}`:\n{stdout}");
    }
    let printed = printed_functions(&stdout, &["wrapneg ", "wrapabs ", "wrapnegd ", "twice "]);
    for bad in ["(float)", "(double)"] {
        assert!(!printed.contains(bad), "`{bad}` converts a callee's bits:\n{printed}");
    }
}

/// `floatret_stale_gcc_O0` (gcc -O0): `find` and `slot` hand back what `lookup`
/// and `slot_of` return (`call; ret`), and `set_e`, `get_c`, `put` and `take`
/// reach a field through that result. `find` was `void`, so its callers printed
/// `*(unsigned int *)(find(a0) + 0x20) = a1`, which does not compile; once the
/// redo made `find` return `long *`, callers decompiled before it kept that
/// text, which C scales by the pointee and writes 0x100 bytes past the record.
/// Every reader of a function whose return a redo changed is decompiled again,
/// and a caller that keeps the result as another class converts it. The
/// printed functions, compiled against the fixture's own `lookup` and
/// `slot_of`, compute what the fixture prints.
#[test]
fn a_reader_of_a_redone_wrapper_round_trips() {
    let bin = repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/floatret_stale_gcc_O0").to_str().unwrap().to_string();
    let sp = specs();
    let (stdout, stderr, ok) = run_kuna(&["decompile-all", &bin, "--option", "structdefs", "on", "--sleighpath", &sp]);
    assert!(ok, "kuna decompile-all failed: {stderr}");
    for want in ["long * find(unsigned int a0)", "long * slot(unsigned int a0)"] {
        assert!(stdout.contains(want), "missing `{want}`:\n{stdout}");
    }
    let printed = printed_functions(&stdout, &["find ", "set_e ", "get_c ", "slot ", "put ", "take "]);
    for bad in ["find(a0) + ", "slot(a0) + "] {
        assert!(!printed.contains(bad), "`{bad}` is arithmetic C scales by the pointee:\n{printed}");
    }
    let src = format!(
        "#include <stdio.h>\n#include <string.h>\n\
         struct rec {{ long a, b, c, d; unsigned int e, f; }};\nstruct rec table[4];\nlong *slots[4];\nstatic long store[4][4];\n\
         long *lookup(unsigned int k) {{ struct rec *r = &table[k & 3]; r->d = r->a + r->b; return (long *)r; }}\n\
         long *slot_of(unsigned int k) {{ long *p = slots[k & 3]; p[1] = p[0] + 1; return p; }}\n{printed}\n\
         int main(void) {{\n  for (int i = 0; i < 4; i++) slots[i] = store[i];\n  \
         set_e(1, 7); put(1, 11); store[1][3] = 5; table[1].c = 13;\n  \
         printf(\"%u %ld %ld %ld\\n\", table[1].e, store[1][2], take(1), get_c(1));\n  return 0;\n}}\n"
    );
    for (cc, got) in compile_and_run_each("floatret-stale", &src) {
        assert_eq!(got, "7 11 5 13", "{cc}: the printed C computes something else:\n{printed}");
    }
}

/// `floatret_cm4.o` (Cortex-M4F, clang -O2): `qnanf_` loads 0x7fc00000 into
/// `s0` and returns it, and `logish` tail-calls it on its error path -- the
/// shape of crazyflie's `logf`. Before, `qnanf_` was `unsigned int` returning
/// `0x7fc00000` and `logish` returned `(float)qnanf_()`, a value conversion of
/// the NaN's bits that evaluates to 2143289344.0. `qp`, `sn` and `nn` return NaNs
/// `NAN` cannot spell (a payload, a signalling NaN, a negative payload) and keep
/// their bits, and `third` returns a `double` in `d0` whose low half is not a
/// float. `put` and `put2` store `core`'s float through an untyped pointer,
/// which printed `((unsigned int *)a1)[2] = core(a0)`, a conversion by value.
/// The printed functions, compiled on the host, must compute what the source
/// computes.
#[test]
fn a_nan_returned_in_s0_round_trips() {
    let bin = repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/floatret_cm4.o").to_str().unwrap().to_string();
    let sp = specs();
    let (stdout, stderr, ok) = run_kuna(&["decompile-all", &bin, "--sleighpath", &sp]);
    assert!(ok, "kuna decompile-all failed: {stderr}");
    for want in [
        "float qnanf_(void)",
        "return NAN;",
        "v1 = qnanf_();",
        "unsigned int qp(void)",
        "return 0x7fc00123;",
        "return 0x7fa00000;",
        "return 0xffc00005;",
        "unsigned int third(void)",
    ] {
        assert!(stdout.contains(want), "missing `{want}`:\n{stdout}");
    }
    for bad in ["0x7fc00000", "(float)qnanf_", "float third", "float qp", "float sn", "float nn", "(unsigned int *)"] {
        assert!(!stdout.contains(bad), "`{bad}` printed:\n{stdout}");
    }
    assert!(stdout.contains("*(float *)(a1 + a2 * 4) = core(a0);"), "put stores core's float:\n{stdout}");
    let printed = printed_functions(&stdout, &["qnanf_ ", "core ", "logish ", "use ", "qp ", "sn ", "nn ", "put2 "]);
    let src = format!(
        "#include <stdio.h>\n#include <string.h>\n#include <math.h>\n{BITS}int dat_0;\n{printed}\n\
         int main(void) {{\n  float out[4] = {{0}};\n  put2(3.0f, out);\n  \
         printf(\"%f %f %f %f %llx %llx %llx %llx\\n\", use(-3.0f), use(0.0f), use(4.0f), logish(-3.0f), \
         BITS(qp()), BITS(sn()), BITS(nn()), BITS(out[2]));\n  return 0;\n}}\n"
    );
    for (cc, got) in compile_and_run_each("floatret-cm4", &src) {
        assert_eq!(
            got, "nan -inf 3.000000 nan 7fc00123 7fa00000 ffc00005 3fc00000",
            "{cc}: the printed C computes something else:\n{printed}"
        );
    }
}

/// A C expression of the type the printed `name` declares its parameter `index`
/// as, holding the bits `bits`: a round trip hands each function the bits the
/// binary does, whichever type the listing gave the parameter.
fn arg_of_bits(printed: &str, name: &str, index: usize, bits: u64) -> String {
    let ty = param_type(printed, name, index);
    format!("({{ {ty} t_; unsigned long long u_ = {bits:#x}ULL; memcpy(&t_, &u_, sizeof t_); t_; }})")
}

/// An argument for parameter `index` of the printed function `name` holding
/// the address `expr`, whether the listing declares a pointer or a `long`.
fn arg_of_pointer(printed: &str, name: &str, index: usize, expr: &str) -> String {
    let ty = param_type(printed, name, index);
    format!("({{ {ty} t_; const void *p_ = {expr}; memcpy(&t_, &p_, sizeof t_); t_; }})")
}

/// The type the printed function `name` declares for parameter `index`.
fn param_type(printed: &str, name: &str, index: usize) -> String {
    let head = printed
        .lines()
        .find(|l| !l.starts_with("//") && l.contains(&format!(" {name}(")))
        .unwrap_or_else(|| panic!("no signature for {name}:\n{printed}"));
    let params = &head[head.find('(').unwrap() + 1..head.rfind(')').unwrap()];
    let param = params.split(',').nth(index).unwrap_or_else(|| panic!("{name} has no parameter {index}: {head}")).trim();
    param.trim_end_matches(|c: char| c.is_ascii_alphanumeric() || c == '_').trim().to_string()
}

/// `floatret_calls_{clang,gcc}_O0`: `signbit_` hands its float to `f2u`, which
/// keeps the bits as an `unsigned int`; `fetch` writes `p[1] = p[0] + 1` and
/// hands `*(float *)p` on to `pass`, which returns it; `use` passes two floats to
/// `mk` and `mk`'s `struct { float, float }` to `first_of`. A float vote on a
/// value in `xmm0` printed `signbit_(float a0) { .. f2u(a0) .. }` beside
/// `f2u(unsigned int)`, `a0[1] = (float)((int)*a0 + 1)` in `fetch`, and
/// `use(float a0,float a1)` handing both to `mk(unsigned int,unsigned int)` --
/// each a conversion by value where the binary moves bits. The printed functions
/// are compiled with gcc and clang and must compute the fixture's bits. At gcc
/// -O0, `call_f2u` hands `f2u`'s result back (`call; leave; ret`), but the
/// argument joined from two registers keeps the call from having an output: it
/// was printed returning a local nothing assigns, and stays `void`.
#[test]
fn a_float_crossing_an_integer_call_round_trips() {
    let sp = specs();
    let fixture = |name: &str| repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures").join(name).to_str().unwrap().to_string();
    let bin = fixture("floatret_calls_clang_O0");
    let (stdout, stderr, ok) = run_kuna(&["decompile-all", &bin, "--sleighpath", &sp]);
    assert!(ok, "kuna decompile-all failed: {stderr}");
    let printed = printed_functions(&stdout, &["f2u ", "signbit_ ", "pass ", "fetch ", "mk ", "first_of ", "use "]);
    let src = format!(
        "#include <stdio.h>\n#include <string.h>\n{BITS}\
         #define CONCAT44(h, l) ((unsigned long)(unsigned int)(h) << 32 | (unsigned int)(l))\nunsigned int g_out;\n{printed}\n\
         int main(void) {{\n  unsigned a[2] = {{0x3fc00000u, 0}};\n  fetch((void *)a);\n  \
         use({}, {});\n  printf(\"%llx %x %x\\n\", BITS(signbit_({})) & 0xff, a[1], g_out);\n  return 0;\n}}\n",
        arg_of_bits(&printed, "use", 0, 0x3fc0_0000),
        arg_of_bits(&printed, "use", 1, 0x4020_0000),
        arg_of_bits(&printed, "signbit_", 0, 0xbf00_0000),
    );
    for (cc, got) in compile_and_run_each("floatret-calls", &src) {
        assert_eq!(got, "1 3fc00001 3fc00000", "{cc}: the printed C computes something else:\n{printed}");
    }

    let bin = fixture("floatret_calls_gcc_O0");
    let (stdout, stderr, ok) = run_kuna(&["decompile-all", &bin, "--sleighpath", &sp]);
    assert!(ok, "kuna decompile-all failed: {stderr}");
    let body = printed_functions(&stdout, &["call_f2u "]);
    for line in body.lines() {
        let Some(var) = line.trim().strip_prefix("return ").and_then(|r| r.strip_suffix(';')) else { continue };
        if var.starts_with('v') && var[1..].chars().all(|c| c.is_ascii_digit()) {
            assert!(body.contains(&format!("{var} = ")), "call_f2u returns `{var}`, which nothing assigns:\n{body}");
        }
    }
}

/// Compile `lib` and `main` as two translation units with each of
/// [`round_trip_compilers`], run the program, and return each compiler's stdout.
fn compile_pair_and_run_each(tag: &str, lib: &str, main: &str) -> Vec<(String, String)> {
    let dir = std::env::temp_dir().join(format!("kuna-{tag}-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let (lib_file, main_file) = (dir.join("lib.c"), dir.join("main.c"));
    std::fs::write(&lib_file, lib).unwrap();
    std::fs::write(&main_file, main).unwrap();
    let mut out = Vec::new();
    for cc in round_trip_compilers() {
        let exe = dir.join(format!("rt-{cc}"));
        let built = Command::new(cc)
            .args(["-std=gnu11", "-w", "-Werror=int-conversion", "-Werror=implicit-function-declaration", "-o"])
            .arg(&exe)
            .arg(&main_file)
            .arg(&lib_file)
            .output()
            .expect("spawn the compiler");
        assert!(built.status.success(), "{cc} rejected the printed C:\n{}\n{lib}", String::from_utf8_lossy(&built.stderr));
        let run = Command::new(&exe).output().expect("run the round trip");
        out.push((cc.to_string(), String::from_utf8_lossy(&run.stdout).trim().to_string()));
    }
    let _ = std::fs::remove_dir_all(&dir);
    out
}

/// `floatparam_x86_64{_O0,}.o` (gcc -O0, clang -O2): `fs`, `ff` and `sink`
/// receive a `double` or `float`s in `xmm0`/`xmm1` and only store them through
/// a pointer, and each parameter printed as an integer, which the convention
/// passes in a general register: `fs(unsigned long a0,unsigned int *a1,..)`
/// read the pointer from `rsi`. The printed functions, compiled on the host
/// beside a caller that declares the source prototypes, must store what the
/// source stores. `fy` copies its double into a global nothing else in the
/// object touches, which `floatglobals` finds is only ever a float.
/// `floatparam_a64.o` (AArch64,
/// clang -O2) and `floatparam_x86_64_clang_O0.o`: `negsink` hands `sink` the
/// bits of an integer in `s0`/`xmm0`, and `k3b` hands `use` the bits `iget3`
/// returns in an integer register, beside an `iget3` printed returning
/// `unsigned int`; each is a reinterpretation the printed call must spell as
/// one, not as the conversion C applies to an integer argument of a `float`
/// parameter. `twicetrunc` reads the float `trunc16` computes on its bits and
/// returns in `s0`, beside a `trunc16` printed returning `unsigned int`:
/// `(float)trunc16(a0)` converted.
#[test]
fn a_float_register_parameter_round_trips() {
    let sp = specs();
    let fixture = |name: &str| repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures").join(name).to_str().unwrap().to_string();
    let callers = "#include <stdio.h>\nvoid fs(int *p, double b, double *q);\nvoid ff(int *p, float b, float *q);\n\
                   void sink(float *o, float a, float b);\n\
                   int main(void) {\n  int k1 = 0, k2 = 0;\n  double q = 0;\n  float f = 0, o[2] = {0};\n  \
                   fs(&k1, 2.5, &q);\n  ff(&k2, 1.25f, &f);\n  sink(o, -1.5f, 2.0f);\n  \
                   printf(\"%d %d %g %g %g %g\\n\", k1, k2, q, f, o[0], o[1]);\n  return 0;\n}\n";
    for (tag, name) in [
        ("gcc-O0", "floatparam_x86_64_O0.o"),
        ("clang-O0", "floatparam_x86_64_clang_O0.o"),
        ("clang-O2", "floatparam_x86_64.o"),
    ] {
        let (stdout, stderr, ok) = run_kuna(&["decompile-all", &fixture(name), "--sleighpath", &sp]);
        assert!(ok, "kuna decompile-all failed: {stderr}");
        for want in [
            "void fs(double a0,unsigned int *a1,double *a2)",
            "void ff(float a0,unsigned int *a1,float *a2)",
            "void fr(double a0,",
            "void sink(float a0,float a1,float *a2)",
            "void fy(double a0,unsigned int *a1)",
        ] {
            assert!(stdout.contains(want), "{tag}: missing `{want}`:\n{stdout}");
        }
        let lib = format!("#include <string.h>\n{}", printed_functions(&stdout, &["fs ", "ff ", "sink "]));
        for (cc, got) in compile_pair_and_run_each(&format!("floatparam-{tag}"), &lib, callers) {
            assert_eq!(got, "1 1 2.5 1.25 -1.5 2", "{tag} {cc}: the printed C stores something else:\n{lib}");
        }
    }

    for (tag, name) in [("a64", "floatparam_a64.o"), ("x86-clang-O0", "floatparam_x86_64_clang_O0.o")] {
        let (stdout, stderr, ok) = run_kuna(&["decompile-all", &fixture(name), "--sleighpath", &sp]);
        assert!(ok, "kuna decompile-all failed: {stderr}");
        for want in ["void sink(float a0,float a1,float *a2)", "void use(float a0,unsigned int *a1,float *a2)"] {
            assert!(stdout.contains(want), "{tag}: missing `{want}`:\n{stdout}");
        }
        let printed = printed_functions(&stdout, &["sink ", "negsink ", "iget3 ", "use ", "k3b "]);
        let global = printed
            .lines()
            .find_map(|l| l.trim().strip_prefix("return ").and_then(|r| r.strip_suffix(';')).filter(|g| g.starts_with("dat_")))
            .unwrap_or_else(|| panic!("{tag}: iget3 returns no global:\n{printed}"));
        let constants: String = printed
            .split(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
            .filter(|w| w.starts_with("dat_") && *w != global)
            .map(|w| format!("const float {w} = 2.0f;\n"))
            .collect::<std::collections::BTreeSet<_>>()
            .into_iter()
            .collect();
        let src = format!(
            "#include <stdio.h>\n#include <string.h>\n{BITS}unsigned int {global} = 0x40490fdbu;\n{constants}{printed}\n\
             int main(void) {{\n  unsigned p[2] = {{0, 0x3fc00000u}};\n  float o[2] = {{0}}, q = 0;\n  int k = 0;\n  \
             negsink((void *)p, o);\n  k3b((void *)&k, &q);\n  \
             printf(\"%llx %llx %d %llx\\n\", BITS(o[0]), BITS(o[1]), k, BITS(q));\n  return 0;\n}}\n"
        );
        for (cc, got) in compile_and_run_each(&format!("floatparam-{tag}"), &src) {
            assert_eq!(got, "bfc00000 40000000 1 40490fdb", "{tag} {cc}: the printed C computes something else:\n{printed}");
        }
    }

    let (stdout, stderr, ok) = run_kuna(&["decompile-all", &fixture("floatparam_a64.o"), "--sleighpath", &sp]);
    assert!(ok, "kuna decompile-all failed: {stderr}");
    assert!(!stdout.contains("(float)trunc16("), "trunc16's bits are converted:\n{stdout}");
    let printed = printed_functions(&stdout, &["trunc16 ", "twicetrunc "]);
    let src = format!(
        "#include <stdio.h>\n#include <string.h>\n{BITS}{printed}\n\
         int main(void) {{\n  printf(\"%llx\\n\", BITS(twicetrunc({})));\n  return 0;\n}}\n",
        arg_of_bits(&printed, "twicetrunc", 0, 0x4049_0fd0)
    );
    for (cc, got) in compile_and_run_each("floatparam-trunc-a64", &src) {
        assert_eq!(got, "40c90000", "{cc}: the printed C computes something else:\n{printed}");
    }
}

/// `floatglobal_{x86_64_gcc_O2,x86_64_clang_O0,a64_O2,armhf_O2}` (linked,
/// `floatglobal.c`): `fy`, `fw` and `ff` copy a float parameter into a global
/// the program only moves through float registers, and `pass`/`passf` hand one
/// to a float parameter. Each parameter printed as an integer the convention
/// passes in a general register (`fy(unsigned long a0,unsigned int *a1)` read
/// the pointer from `rsi`), and each argument as a union reinterpretation of an
/// integer. `gpun` takes a float's bits and is added to as an integer, so
/// `set_gpun` keeps its integer parameter; `gz` is zeroed by an integer store at
/// -O2 (`setz` keeps `unsigned long`) and through `xmm0` at clang -O0 (`setz`
/// takes the `double`). The printed functions, compiled on the host beside a
/// caller that declares the source prototypes, must store what the source
/// stores. Hard-float ARM drops `fy`'s and `fw`'s `d0` parameter
/// (`armfloatargs` off), so only `ff` is compiled there.
#[test]
fn a_global_only_moved_through_float_registers_round_trips() {
    let sp = specs();
    let fixture = |name: &str| repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures").join(name).to_str().unwrap().to_string();
    for (tag, name, doubles, setz) in [
        ("x86-gcc-O2", "floatglobal_x86_64_gcc_O2", true, "void setz(unsigned long a0)"),
        ("x86-clang-O0", "floatglobal_x86_64_clang_O0", true, "void setz(double a0)"),
        ("a64-O2", "floatglobal_a64_O2", true, "void setz(unsigned long a0)"),
        ("armhf-O2", "floatglobal_armhf_O2", false, "void ff(float a0,unsigned int *a1)"),
    ] {
        let (stdout, stderr, ok) = run_kuna(&["decompile-all", &fixture(name), "--sleighpath", &sp]);
        assert!(ok, "kuna decompile-all failed: {stderr}");
        let mut want = vec!["void ff(float a0,unsigned int *a1)", "sinkf(gf2)", "void set_gpun(unsigned int a0)", setz];
        if doubles {
            want.extend(["void fy(double a0,unsigned int *a1)", "void fw(double a0,unsigned int a1)", "sink(gd2)"]);
        }
        for w in want {
            assert!(stdout.contains(w), "{tag}: missing `{w}`:\n{stdout}");
        }
        assert!(!stdout.contains(".from = gf2") && !stdout.contains(".from = gd2"), "{tag}: a float global reinterpreted:\n{stdout}");
        let (names, calls, expect): (&[&str], &str, &str) = if doubles {
            (&["fy ", "fw ", "ff "], "fy(&k1, 2.5);\n  fw(3, 4.5);\n  ff(&k2, 1.25f);", "1 2 4.5 1.25 3")
        } else {
            (&["ff "], "k1 = 1;\n  gd = 4.5;\n  gi = 3;\n  ff(&k2, 1.25f);", "1 2 4.5 1.25 3")
        };
        let lib = format!("double gd;\nfloat gf;\nunsigned int gi;\n{}", printed_functions(&stdout, names));
        let callers = format!(
            "#include <stdio.h>\nvoid fy(int *p, double b);\nvoid fw(int a, double b);\nvoid ff(int *p, float b);\n\
             extern double gd;\nextern float gf;\nextern unsigned int gi;\n\
             int main(void) {{\n  int k1 = 0, k2 = 0;\n  {calls}\n  printf(\"%d %d %g %g %u\\n\", k1, k2, gd, gf, gi);\n  return 0;\n}}\n"
        );
        for (cc, got) in compile_pair_and_run_each(&format!("floatglobal-{tag}"), &lib, &callers) {
            assert_eq!(got, expect, "{tag} {cc}: the printed C stores something else:\n{lib}");
        }
    }
}

/// `floatglobal_index_x86_64_gcc_O2` (`floatglobal_index.c`): `ixa`, `lv0`,
/// `fld`, `neg` and `pas` read a double global's bits as an integer through an
/// index from its address, which the whole-program scan cannot see. A float
/// taken there prints `(&gda)[a1]` as a double element, a value conversion
/// where the machine adds to the bits, so the parameter and argument votes
/// refuse. `put` indexes a double array only as doubles and keeps its double.
#[test]
fn an_integer_access_through_an_index_from_a_float_global_refuses_the_float() {
    let bin = repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/floatglobal_index_x86_64_gcc_O2").to_str().unwrap().to_string();
    let sp = specs();
    let (stdout, stderr, ok) = run_kuna(&["decompile-all", &bin, "--sleighpath", &sp]);
    assert!(ok, "kuna decompile-all failed: {stderr}");
    for w in [
        "void ixa(unsigned long a0,int a1)",
        "void lv0(unsigned long a0,int a1)",
        "void fld(unsigned long a0,int a1)",
        "void neg(unsigned long a0,int a1)",
        "void put(double a0,int a1)",
        ".from = gdp }",
    ] {
        assert!(stdout.contains(w), "missing `{w}`:\n{stdout}");
    }
    assert!(!stdout.contains("sink(gdp)"), "gdp handed on as a float its own index reads as an integer:\n{stdout}");
}

/// `floatparam_retreg_clang_O2` (clang -O2, stripped): `v1` (`sub_11c0`) returns
/// `(float)geti()` in `xmm0`, but its recovery returns `rax`, and `main` reads
/// the call's `xmm0`. The return a callee states in another register says
/// nothing of what the caller reads, so the read is not printed as a
/// reinterpretation of `sub_11c0`'s integer.
#[test]
fn a_return_stated_in_another_register_is_not_reinterpreted() {
    let bin = repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/floatparam_retreg_clang_O2").to_str().unwrap().to_string();
    let sp = specs();
    let (stdout, stderr, ok) = run_kuna(&["decompile-all", &bin, "--sleighpath", &sp]);
    assert!(ok, "kuna decompile-all failed: {stderr}");
    assert!(stdout.contains("unsigned long sub_11c0("), "sub_11c0 no longer returns rax; re-pick the case:\n{stdout}");
    assert!(!stdout.contains(".from = sub_11c0("), "a return in rax reinterpreted as xmm0's float:\n{stdout}");
}

/// `floatret_put_{cm4,a64}.o` (Cortex-M4F and AArch64, clang -O2): `putf2` moves
/// its floats from `s0`..`s2` into `r0`..`r2` / `w0`..`w2` and tail-calls
/// `put3`, which stores them as `u32`. A float vote on the parameters printed
/// `putf2(float a0,..) { put3(a0,..); }` beside `put3(unsigned int,..)`, which
/// stores 1, 2 and 7 where the binary stores the bits. The printed functions,
/// compiled on the host, must store the bits.
#[test]
fn a_float_handed_on_to_an_integer_parameter_round_trips() {
    let sp = specs();
    for (tag, name) in [("cm4", "floatret_put_cm4.o"), ("a64", "floatret_put_a64.o")] {
        let bin = repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures").join(name).to_str().unwrap().to_string();
        let (stdout, stderr, ok) = run_kuna(&["decompile-all", &bin, "--sleighpath", &sp]);
        assert!(ok, "kuna decompile-all failed: {stderr}");
        let printed = printed_functions(&stdout, &["put3 ", "putf2 "]);
        let src = format!(
            "#include <stdio.h>\n#include <string.h>\n{printed}\n\
             int main(void) {{\n  unsigned s[3] = {{0}};\n  putf2({}, {}, {}, (void *)s);\n  \
             printf(\"%x %x %x\\n\", s[0], s[1], s[2]);\n  return 0;\n}}\n",
            arg_of_bits(&printed, "putf2", 0, 0x3fc0_0000),
            arg_of_bits(&printed, "putf2", 1, 0x4020_0000),
            arg_of_bits(&printed, "putf2", 2, 0x40e0_0000),
        );
        for (cc, got) in compile_and_run_each(&format!("floatret-put-{tag}"), &src) {
            assert_eq!(got, "3fc00000 40200000 40e00000", "{tag} {cc}: the printed C stores something else:\n{printed}");
        }
    }
}

/// `floatret_pair_gcc_O2` (gcc -O2, stripped): `k2`, `k3` and `kc` return
/// `{1.0f, 2.0f}` in `xmm0` as a `struct { float, float }`, the first two
/// floats of three, and a `float _Complex`, and each reader copies the eight
/// bytes into a `uint64_t` global and shifts out the upper half. A float vote
/// typed the callees `double` and the readers printed `dat_4040 =
/// (unsigned long)sub_11d0()`, which converts 2.0000004 to 2. The callees keep
/// their bits and the printed functions, compiled with gcc and clang, store
/// what the fixture stores.
#[test]
fn a_float_pair_held_as_an_integer_round_trips() {
    let bin = repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/floatret_pair_gcc_O2").to_str().unwrap().to_string();
    let sp = specs();
    let (stdout, stderr, ok) = run_kuna(&["decompile-all", &bin, "--sleighpath", &sp]);
    assert!(ok, "kuna decompile-all failed: {stderr}");
    for want in ["unsigned long sub_11d0(void)", "unsigned long sub_11f0(void)", "unsigned long sub_1210(void)"] {
        assert!(stdout.contains(want), "missing `{want}`:\n{stdout}");
    }
    let printed = printed_functions(&stdout, &["sub_11d0 ", "sub_11f0 ", "sub_1210 ", "sub_1230 ", "sub_1260 ", "sub_1290 "]);
    for bad in ["double", "(unsigned long)sub_"] {
        assert!(!printed.contains(bad), "`{bad}` printed:\n{printed}");
    }
    let mut globals = String::new();
    for line in printed.lines() {
        for word in line.split(|c: char| !c.is_ascii_alphanumeric() && c != '_') {
            if word.starts_with("dat_") && !globals.contains(&format!(" {word};")) {
                globals.push_str(&format!("unsigned long {word};\n"));
            }
        }
    }
    let src = format!(
        "#include <stdio.h>\n{globals}{printed}\n\
         int main(void) {{\n  sub_1230(1);\n  sub_1260();\n  sub_1290();\n  \
         printf(\"%lx %lx %lx %lx %lx %lx %lx\\n\", dat_4040, dat_4048, dat_4050, dat_4058, dat_4060, dat_4068, dat_4070);\n  return 0;\n}}\n"
    );
    for (cc, got) in compile_and_run_each("floatret-pair", &src) {
        assert_eq!(
            got, "400000003f800000 40000000 1 400000003f800000 40000000 400000003f800000 40000000",
            "{cc}: the printed C stores something else:\n{printed}"
        );
    }
}

/// `floatret_chain_gcc_O1` (gcc -O1, stripped): `wrapd` hands on the `double`
/// `getd` returns in `xmm0` (`call; ret`), `wrap2` hands on `wrapd`'s, and
/// `wrapp` the `struct { float, float }` of `getp`; each reader copies the eight
/// bytes into a `uint64_t` global. A reader keeping the bits as an integer
/// withdrew the wrapper's float return, but the float came from the getter's
/// own, which stayed: the wrapper still returned `double`, and the reader's
/// `dat_40a0 = sub_1156(a0,a1)` converted 2.25 to 2. The getters are withdrawn
/// with the wrappers, and the printed functions, compiled with gcc and clang,
/// store the fixture's bits. `floatret_chain_mips_O0` (mipsel gcc -O0) hands a
/// `float` in `$f0` through `wrapf` the same way: nothing on that chain is a
/// float either, and the printed reader, run on the fixture's array mapped at a
/// 32-bit address, stores the bits of 2.25f, where a float chain stored 2.
#[test]
fn a_float_handed_on_through_wrappers_to_an_integer_round_trips() {
    let sp = specs();
    let fixture = |name: &str| repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures").join(name).to_str().unwrap().to_string();
    let bin = fixture("floatret_chain_gcc_O1");
    let (stdout, stderr, ok) = run_kuna(&["decompile-all", &bin, "--sleighpath", &sp]);
    assert!(ok, "kuna decompile-all failed: {stderr}");
    let names = ["sub_1149 ", "sub_1156 ", "sub_1160 ", "sub_1181 ", "sub_118e ", "sub_1198 ", "sub_11ba ", "sub_11ff "];
    let printed = printed_functions(&stdout, &names);
    for want in ["unsigned long sub_1156(long a0,int a1)", "unsigned long sub_1160(long a0,int a1)", "unsigned long sub_118e(long a0,int a1)"] {
        assert!(printed.contains(want), "missing `{want}`:\n{printed}");
    }
    assert!(!printed.contains("double"), "a function of the chain returns a float:\n{printed}");
    let globals: String =
        ["dat_40a0", "dat_40a8", "dat_40b0", "dat_40b8", "dat_40d0", "dat_40d8"].iter().map(|g| format!("unsigned long {g};\n")).collect();
    let src = format!(
        "#include <stdio.h>\n#include <string.h>\n\
         double darr[4] = {{1.5, -0.0, 2.25, 3.0}};\n\
         struct {{ float x, y; }} parr[4] = {{{{1.0f, 2.0f}}, {{3.0f, 4.0f}}, {{5.0f, 6.0f}}, {{7.0f, 8.0f}}}};\n\
         {globals}{printed}\n\
         int main(void) {{\n  sub_1198({}, 2);\n  sub_11ba({}, 3);\n  sub_11ff({}, 1);\n  \
         printf(\"%lx %lx %lx %lx %lx %lx\\n\", dat_40a0, dat_40a8, dat_40b0, dat_40b8, dat_40d0, dat_40d8);\n  return 0;\n}}\n",
        arg_of_pointer(&printed, "sub_1198", 0, "darr"),
        arg_of_pointer(&printed, "sub_11ba", 0, "darr"),
        arg_of_pointer(&printed, "sub_11ff", 0, "parr"),
    );
    for (cc, got) in compile_and_run_each("floatret-chain", &src) {
        assert_eq!(
            got, "4002000000000000 40020000 4008000000000000 40080000 4080000040400000 40800000",
            "{cc}: the printed C stores something else:\n{printed}"
        );
    }
    let mips = fixture("floatret_chain_mips_O0");
    let (stdout, stderr, ok) = run_kuna(&["decompile-all", &mips, "--sleighpath", &sp]);
    assert!(ok, "kuna decompile-all failed: {stderr}");
    let printed = printed_functions(&stdout, &["sub_40085c ", "sub_400898 ", "sub_400b78 "]);
    for want in ["unsigned int sub_40085c(int a0,int a1)", "unsigned int sub_400898(int a0,int a1)"] {
        assert!(printed.contains(want), "missing `{want}`:\n{printed}");
    }
    assert!(!printed.contains("float"), "a function of the chain returns a float:\n{printed}");
    let src = format!(
        "#include <stdio.h>\n#include <string.h>\n#include <sys/mman.h>\n\
         void __stack_chk_fail(void);\nint guard_;\nint *dat_412094 = &guard_;\n\
         unsigned int dat_4120e0, dat_4120e4, dat_4120e8, dat_4120ec;\n{printed}\n\
         int main(void) {{\n  float farr[4] = {{1.5f, -0.0f, 2.25f, 3.0f}};\n  \
         void *base = mmap((void *)0x20000000, 4096, PROT_READ | PROT_WRITE, MAP_PRIVATE | MAP_ANONYMOUS | MAP_FIXED_NOREPLACE, -1, 0);\n  \
         if (base != (void *)0x20000000) return 2;\n  memcpy(base, farr, sizeof farr);\n  sub_400b78({}, 2);\n  \
         printf(\"%x %x %x %x\\n\", dat_4120e0, dat_4120e4, dat_4120e8, dat_4120ec);\n  return 0;\n}}\n",
        arg_of_pointer(&printed, "sub_400b78", 0, "base"),
    );
    for (cc, got) in compile_and_run_each("floatret-chain-mips", &src) {
        assert_eq!(got, "40100000 0 4010 0", "{cc}: the printed MIPS reader stores something else:\n{printed}");
    }
}

/// `floatret_pair32_{i386_O2,arm_O0}.o`: `use` reads a 64-bit result in two
/// registers from `hi` (`call zsum; or $0xff,%edx`) and from `wrap` (`call
/// ins16; ret`). Returned in `eax` alone, `hi` dropped the `0xff` its high
/// register carries, and `wrap` returned the low half of a 64-bit value. A
/// return the callers read in two registers is returned whole or not at all.
#[test]
fn a_pair_return_handed_on_stays_whole_round_trips() {
    let sp = specs();
    let pairs: [(u32, u32); 4] = [(1, 2), (0x7fff, 0x10), (0x1000000, 0x2345), (0, 0)];
    let want: Vec<String> = pairs.iter().map(|(a, b)| format!("{:x}", (a.wrapping_add(*b) as u64) | 0xff00000000)).collect();
    for name in ["floatret_pair32_i386_O2.o", "floatret_pair32_arm_O0.o"] {
        let bin = repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures").join(name).to_str().unwrap().to_string();
        let (stdout, stderr, ok) = run_kuna(&["decompile-all", &bin, "--sleighpath", &sp]);
        assert!(ok, "kuna decompile-all failed: {stderr}");
        assert!(!stdout.contains("unsigned int wrap("), "{name}: wrap returns half of a 64-bit value:\n{stdout}");
        let printed = printed_functions(&stdout, &["zsum ", "hi "]);
        assert!(printed.contains("unsigned long long hi(int a0,int a1)"), "{name}:\n{printed}");
        let calls: String = pairs.iter().map(|(a, b)| format!("  printf(\"%llx\\n\", hi({a:#x},{b:#x}));\n")).collect();
        let src = format!("#include <stdio.h>\n{printed}\nint main(void) {{\n{calls}  return 0;\n}}\n");
        for (cc, got) in compile_and_run_each("floatret-pair32", &src) {
            assert_eq!(got.lines().collect::<Vec<_>>(), want, "{name} {cc}: the printed hi returns something else:\n{printed}");
        }
    }
}

/// `floatret_beside_gcc_O2` (gcc -O2, stripped): `getf` returns a packed
/// global's float on one path and `0.0f` on the other, each with its own `ret`.
/// A value read from a global refuses the float-register vote, but the constant
/// on the other path was voted alone, and the function returned `float` with
/// the global in it. Every value the function returns in the register now
/// answers for the vote.
#[test]
fn a_float_return_beside_a_global_read_stays_an_integer() {
    let bin = repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/floatret_beside_gcc_O2").to_str().unwrap().to_string();
    let sp = specs();
    let (stdout, stderr, ok) = run_kuna(&["decompile-all", &bin, "--sleighpath", &sp]);
    assert!(ok, "kuna decompile-all failed: {stderr}");
    let printed = printed_functions(&stdout, &["sub_1180 "]);
    for want in ["unsigned int sub_1180(int a0)", "return 0;", "return dat_4017;"] {
        assert!(printed.contains(want), "missing `{want}`:\n{printed}");
    }
}

/// The entry hands `f`'s `eax` to `ExitProcess`, so `f` is decompiled again to
/// return it, and that decompile restarts once its second indirect call
/// resolves to `ExitProcess`. On its one returning path `f` returns the handle
/// the comparison also reads: `return -1;`. Patched to also load through the
/// handle on that path (`je` to a `mov ecx,[eax]` the `ExitProcess` path jumps
/// over), the pass before the restart returned `eax`, joined with what the
/// unresolved call left, and the pass after it refused the handle; the first
/// pass's output survived as `int sub_401020(..)` around a bare `return;`, and
/// the function must be `void` again. No function that declares a return type
/// prints a bare `return;`.
#[test]
fn a_return_refused_after_a_restart_leaves_the_function_void() {
    let fixture = repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/floatret_restart_pe_i386.exe");
    let sp = specs();
    let decompile = |bin: &str| {
        let (stdout, stderr, ok) = run_kuna(&["decompile-all", bin, "--sleighpath", &sp]);
        assert!(ok, "kuna decompile-all failed: {stderr}");
        for chunk in stdout.split("// Function: ") {
            let mut lines = chunk.lines().skip(1);
            let Some(sig) = lines.next() else { continue };
            if !sig.starts_with("void ") {
                assert!(!lines.any(|l| l.trim() == "return;"), "`{sig}` returns nothing:\n{stdout}");
            }
        }
        printed_functions(&stdout, &["sub_401020 "])
    };
    let printed = decompile(fixture.to_str().unwrap());
    assert!(printed.contains("int sub_401020(unsigned int a0)") && printed.contains("return -1;"), "{printed}");

    let mut image = std::fs::read(&fixture).unwrap();
    let at = |image: &[u8], pattern: &[u8]| image.windows(pattern.len()).position(|w| w == pattern).unwrap();
    let je = at(&image, &[0x74, 0x0e, 0xc7, 0x04, 0x24]);
    image[je + 1] = 0x10;
    let leave = at(&image, &[0xff, 0xd0, 0xc9, 0xc3]) + 2;
    image[leave..leave + 12]
        .copy_from_slice(&[0xeb, 0x08, 0x8b, 0x08, 0x89, 0x0d, 0x00, 0x21, 0x40, 0x00, 0xc9, 0xc3]);
    let patched = common::scratch_file("floatret-restart-load", "exe");
    std::fs::write(&patched, &image).unwrap();
    let printed = decompile(patched.to_str().unwrap());
    assert!(printed.contains("void sub_401020(unsigned int a0)"), "{printed}");
}

/// `floatret_handon_gcc_O1`: `key_ssh_name` passes `key_type_plain`'s `int` to
/// `type_name` and returns `type_name`'s pointer (`call; call; ret`), and `main`
/// reads all of `rax`. The function names only `eax`, the first call's result,
/// so its return was as narrow: `unsigned int key_ssh_name(..)`, which cuts the
/// pointer to 32 bits. The printed functions are compiled against a key table
/// whose string lies above 4 GiB and must return that string's address.
#[test]
fn a_pointer_handed_on_past_an_int_call_round_trips() {
    let bin = repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/floatret_handon_gcc_O1").to_str().unwrap().to_string();
    let sp = specs();
    let (stdout, stderr, ok) = run_kuna(&["decompile-all", &bin, "--option", "structdefs", "on", "--sleighpath", &sp]);
    assert!(ok, "kuna decompile-all failed: {stderr}");
    let printed = printed_functions(&stdout, &["type_name ", "key_type_plain ", "key_ssh_name "]);
    let src = format!(
        "#include <stdio.h>\n#include <string.h>\n#include <sys/mman.h>\n{BITS}unsigned long keytypes[8];\n{printed}\n\
         int main(void) {{\n  char *ed = mmap((void *)0x7e0000000000ul, 4096, PROT_READ | PROT_WRITE, MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);\n  \
         strcpy(ed, \"ssh-ed25519\");\n  keytypes[0] = (unsigned long)\"ssh-rsa\"; keytypes[2] = (unsigned long)ed; keytypes[3] = 3;\n  \
         keytypes[4] = (unsigned long)\"ecdsa\"; keytypes[5] = 2 | (415ul << 32); keytypes[7] = (unsigned long)-1;\n  \
         unsigned char key[64] = {{3}};\n  unsigned long long r = BITS(key_ssh_name((void *)key));\n  \
         printf(\"%d %d %s\\n\", (unsigned long)ed > 0xfffffffful, r == (unsigned long)ed, r == (unsigned long)ed ? ed : \"?\");\n  return 0;\n}}\n"
    );
    for (cc, got) in compile_and_run_each("floatret-handon", &src) {
        assert_eq!(got, "1 1 ssh-ed25519", "{cc}: the printed C returns something else:\n{printed}");
    }
}

/// `floatret_twopath_gcc_O1`: `wrap` tests `pick`'s `int` and returns one of
/// two `getname` calls' pointers, both paths ending at one `ret`, and `main`
/// reads all of `rax`. Heritage refines the planted `rax` at the `eax` the body
/// names and merges each half on its own, which no rule joins again: `wrap`
/// returned `CONCAT44(dat_4,v2)` and stored the upper half of `getname`'s
/// pointer into `dat_4`, a register piece printed as a global the binary never
/// writes. It stays `void`, as before a caller asked for its result.
#[test]
fn a_return_merged_in_pieces_stays_void() {
    let bin = repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/floatret_twopath_gcc_O1").to_str().unwrap().to_string();
    let sp = specs();
    let (stdout, stderr, ok) = run_kuna(&["decompile-all", &bin, "--sleighpath", &sp]);
    assert!(ok, "kuna decompile-all failed: {stderr}");
    let printed = printed_functions(&stdout, &["wrap "]);
    assert!(printed.contains("void wrap("), "`wrap` returns a value assembled from separately merged pieces:\n{printed}");
    for bad in ["dat_4 =", "CONCAT44(dat_4,"] {
        assert!(!printed.contains(bad), "`wrap` writes a register piece as a global (`{bad}`):\n{printed}");
    }
}

/// `floatret_parse_gcc_O1.o` and `floatret_parse_armel.o` (soft-float ARM):
/// `parse`, `parse2` and `parsef` return what `strtod` and `strtof` return, and
/// `rp` keeps `parse`'s bits as an integer, `rp2` stores `parse2`'s `double`, and
/// `rpf` keeps `parsef`'s bits. A relocatable object declares neither callee, so
/// a return through them printed `unsigned long parse(..) { return strtod(..); }`,
/// which converts the `double` by value once compiled against `<stdlib.h>`. They
/// stay `void`, as before a caller asked for their result.
#[test]
fn a_return_of_an_undeclared_float_callee_stays_void() {
    let sp = specs();
    for name in ["floatret_parse_gcc_O1.o", "floatret_parse_armel.o"] {
        let bin = repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures").join(name).to_str().unwrap().to_string();
        let (stdout, stderr, ok) = run_kuna(&["decompile-all", &bin, "--sleighpath", &sp]);
        assert!(ok, "kuna decompile-all failed on {name}: {stderr}");
        let printed = printed_functions(&stdout, &["parse ", "parse2 ", "parsef ", "rp ", "rp2 ", "rpf "]);
        for want in ["void parse(", "void parse2(", "void parsef("] {
            assert!(printed.contains(want), "{name}: missing `{want}`:\n{printed}");
        }
        for bad in ["return strtod(", "return strtof("] {
            assert!(!printed.contains(bad), "{name}: `{bad}` converts the callee's float by value:\n{printed}");
        }
    }
}

/// `floatret_softfp_armv7` states the base procedure-call standard and has an
/// FPU (`-mfloat-abi=softfp`, as Android `armeabi-v7a` builds do), so its
/// imports take and return doubles in core registers: `pw` passes `r0`-`r3` to
/// `pow`, and `rd` stores `strtod`'s `r0:r1` and passes it to `ceil`. The libc
/// table's `double` rows laid those calls out in VFP registers and dropped the
/// arguments and results (`pow(v1,v2)` of unset `d0`/`d1`); they are not seeded
/// on such an image.
#[test]
fn soft_float_imports_keep_their_core_register_values() {
    let bin = repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/floatret_softfp_armv7").to_str().unwrap().to_string();
    let sp = specs();
    let (stdout, stderr, ok) = run_kuna(&["decompile-all", &bin, "--sleighpath", &sp]);
    assert!(ok, "kuna decompile-all failed: {stderr}");
    for want in ["pow(a2,a3,a0,a1);", "= strtod(a0,0);", "= strtof(a0,0);", "= ceil("] {
        assert!(stdout.contains(want), "missing `{want}`:\n{stdout}");
    }
    for bad in ["double strtod(", "double pow(", "pow(v1,v2)"] {
        assert!(!stdout.contains(bad), "`{bad}` reads VFP registers the code never sets:\n{stdout}");
    }
}

/// `floatret_armrows_soft` and `floatret_armrows_hard` (soft-float and
/// hard-float ARM): `clampd` returns `strtod`'s result or 0, and `rd` copies
/// its bits into a `uint64_t`. With the libc table's `double` rows the soft
/// image printed `double sub_10224(..)` beside `rd`'s `unsigned long long v1 =
/// sub_10224()`, which converts 2.25 to 2, where the rows' absence returns the
/// right `CONCAT44(v2,v1)`; the hard image printed `unsigned int sub_101f0(..)`
/// returning `SUB84(v1,0)`, the low half of `d0`. ARM gets none of the rows.
#[test]
fn arm_images_get_no_double_libc_rows() {
    let sp = specs();
    for name in ["floatret_armrows_soft", "floatret_armrows_hard"] {
        let bin = repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures").join(name).to_str().unwrap().to_string();
        let (stdout, stderr, ok) = run_kuna(&["decompile-all", &bin, "--sleighpath", &sp]);
        assert!(ok, "kuna decompile-all failed on {name}: {stderr}");
        for bad in ["double", "SUB84("] {
            assert!(!stdout.contains(bad), "{name}: `{bad}` from a libc row ARM does not return whole:\n{stdout}");
        }
        if name.ends_with("soft") {
            assert!(
                stdout.contains("unsigned long long sub_10224(") && stdout.contains("return CONCAT44(v2,v1);"),
                "{name}: `clampd` no longer returns its r0:r1 bits:\n{stdout}"
            );
        }
    }
}

/// `dsum`, `norm` and `use` read their argument as `double *`, `struct P *` and
/// `struct M *`, and each caller writes that memory with integer bits first. A
/// pointer vote from the callee printed those stores as value conversions
/// (`a0->field_0x0 = (double)(a1 + 1)`, `a0->field_0x4 = (float)v2`), the payload
/// NaN as `NAN`, and `s3`'s integer-register parameters as `double`. Both passes,
/// the default and `--option protoorder off`, compile the six printed callers
/// against recording callees and must leave the same bytes as the source.
#[test]
fn a_float_pointee_keeps_the_callers_integer_stores_round_trip() {
    let bin = repo_root()
        .join("decompiler/crates/kuna-analysis/tests/fixtures/protoorder_floatpointee_x86_64")
        .to_str()
        .unwrap()
        .to_string();
    let sp = specs();
    let have_cc = process::optional_output(Command::new("cc").arg("--version")).is_some();
    for off in [false, true] {
        let mut args = vec!["decompile-all", bin.as_str(), "--option", "structdefs", "on", "--sleighpath", &sp];
        if off {
            args.extend_from_slice(&["--option", "protoorder", "off"]);
        }
        let (stdout, stderr, ok) = run_kuna(&args);
        assert!(ok, "kuna decompile-all failed: {stderr}");
        let names = ["u1 ", "u2 ", "s3 ", "cp1 ", "cp3 ", "cp5 "];
        let chunks: Vec<&str> =
            stdout.split("// Function: ").filter(|c| names.iter().any(|n| c.starts_with(n))).collect();
        assert_eq!(chunks.len(), names.len(), "{stdout}");
        for bad in ["= (double)(", "= (float)", "NAN", "double a1", "double a2"] {
            assert!(!chunks.iter().any(|c| c.contains(bad)), "`{bad}` printed (off={off}):\n{stdout}");
        }
        if !have_cc {
            eprintln!("protoorder float pointee round trip: no `cc`, spelling checked only");
            continue;
        }
        let printed = printed_functions(&stdout, &names);
        let dir = std::env::temp_dir().join(format!("kuna-protoorder-pointee-rt-{}-{off}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let src = dir.join("rt.c");
        let exe = dir.join("rt");
        std::fs::write(
            &src,
            format!(
                "#include <stdio.h>\n#include <string.h>\n#include <math.h>\n\
                 static unsigned char seen[32];\n\
                 double dsum(void *p) {{ memcpy(seen, p, 32); return 0; }}\n\
                 double norm(void *p) {{ memcpy(seen, p, 16); return 0; }}\n\
                 double use(void *p) {{ memcpy(seen, p, 16); return 0; }}\n\
                 {printed}\n\
                 union U {{ long l[4]; double d[4]; }};\n\
                 struct P {{ double x, y; }};\n\
                 struct M {{ int i; float f; double d; }};\n\
                 static void s_u1(union U *u, long v) {{ u->l[0] = v + 1; u->l[1] = v >> 1; dsum(u->d); }}\n\
                 static void s_u2(union U *u) {{ long t = u->l[2]; u->l[0] = t * 3; \
                 u->l[1] = 0x7ff0000000000001L; dsum(u->d); }}\n\
                 static void s_s3(struct P *d, long a, long b) {{ memcpy(&d->x, &a, 8); memcpy(&d->y, &b, 8); \
                 norm(d); }}\n\
                 static void s_cp1(struct M *d, const struct M *s) {{ *d = *s; use(d); }}\n\
                 static void s_cp3(struct M *d, long a, long b) {{ memcpy(d, &a, 8); memcpy(&d->d, &b, 8); \
                 use(d); }}\n\
                 static void s_cp5(struct M *d, unsigned v) {{ unsigned w = v * 2 + 1; memcpy(&d->f, &w, 4); \
                 d->i = v; d->d = 0; use(d); }}\n\
                 static void show(const char *tag, long *b) {{\n  printf(\"%s\", tag);\n  \
                 for (int i = 0; i < 4; i++) printf(\" %016lx\", b[i]);\n  \
                 for (int i = 0; i < 32; i++) printf(\"%02x\", seen[i]);\n  printf(\"\\n\");\n  \
                 memset(seen, 0, 32);\n}}\n\
                 static void run(int p) {{\n  \
                 long b[4], s[4] = {{0x40000000L << 32 | 1, 0x4008000000000000L, 0, 0}};\n  \
                 memcpy(b, (long[4]){{1, 2, 3, 4}}, 32);\n  \
                 if (p) ((void (*)(void *, long))u1)(b, 7); else s_u1((void *)b, 7);\n  show(\"u1\", b);\n  \
                 memcpy(b, (long[4]){{1, 2, 3, 4}}, 32);\n  \
                 if (p) ((void (*)(void *))u2)(b); else s_u2((void *)b);\n  show(\"u2\", b);\n  \
                 memset(b, 0, 32);\n  \
                 if (p) ((void (*)(void *, long, long))s3)(b, 0x4008000000000001L, 0x4010000000000000L);\n  \
                 else s_s3((void *)b, 0x4008000000000001L, 0x4010000000000000L);\n  show(\"s3\", b);\n  \
                 memset(b, 0, 32);\n  \
                 if (p) ((void (*)(void *, void *))cp1)(b, s); else s_cp1((void *)b, (void *)s);\n  \
                 show(\"cp1\", b);\n  memset(b, 0, 32);\n  \
                 if (p) ((void (*)(void *, long, long))cp3)(b, 0x4000000000000001L, 0x3ff0000000000000L);\n  \
                 else s_cp3((void *)b, 0x4000000000000001L, 0x3ff0000000000000L);\n  show(\"cp3\", b);\n  \
                 memset(b, 0, 32);\n  \
                 if (p) ((void (*)(void *, unsigned))cp5)(b, 0x1fc00007u); else s_cp5((void *)b, 0x1fc00007u);\n  \
                 show(\"cp5\", b);\n}}\n\
                 int main(void) {{\n  run(1);\n  printf(\"--\\n\");\n  run(0);\n  return 0;\n}}\n"
            ),
        )
        .unwrap();
        let cc = Command::new("cc")
            .args(["-std=gnu11", "-w", "-o", exe.to_str().unwrap(), src.to_str().unwrap()])
            .output()
            .expect("spawn cc");
        assert!(
            cc.status.success(),
            "the printed callers did not compile (off={off}):\n{}\n{printed}",
            String::from_utf8_lossy(&cc.stderr)
        );
        let run = process::required_output(&mut Command::new(&exe));
        let got = String::from_utf8_lossy(&run.stdout).to_string();
        let _ = std::fs::remove_dir_all(&dir);
        let (printed_run, source_run) = got.split_once("--\n").expect("both runs printed");
        assert_eq!(printed_run, source_run, "the printed callers store different bytes (off={off}):\n{printed}");
    }
}

/// `fill` stores the eight bytes of `"ustar  "` through the buffer it hands
/// `peek`, whose recovered parameter is `unsigned char *`. Taken as a vote, that
/// type made `fill`'s parameter a byte pointer and `SplitDatatype` printed the
/// store as eight byte stores; the vote is refused because the caller writes
/// wider than the pointee. Checked at the default, which also types `fill`'s
/// parameter from its own dereferences, and with `--option ptrfromuse off`.
#[test]
fn a_byte_pointee_vote_keeps_the_callers_wide_stores() {
    let bin = repo_root()
        .join("decompiler/crates/kuna-analysis/tests/fixtures/protoorder_narrowvote_x86_64")
        .to_str()
        .unwrap()
        .to_string();
    let sp = specs();
    for off in [false, true] {
        let mut args = vec!["decompile-all", bin.as_str(), "--sleighpath", &sp];
        if off {
            args.extend_from_slice(&["--option", "ptrfromuse", "off"]);
        }
        let (stdout, stderr, ok) = run_kuna(&args);
        assert!(ok, "kuna decompile-all failed: {stderr}");
        let fill = stdout.split("// Function: ").find(|c| c.starts_with("fill ")).expect("fill is printed");
        assert!(fill.contains("= 0x2020726174737575;"), "the eight-byte store was split (off={off}):\n{fill}");
        assert!(!fill.contains("unsigned char *a0"), "the byte-pointer vote was taken (off={off}):\n{fill}");
    }
}

/// The callee-first order runs the batch's `structsynth` convergence sweep too:
/// `fc` supersedes the structure `fb` minted, and `fb` is decompiled again onto
/// `fc`'s. The three readers make no direct calls, so the callee-first order is
/// the address order and the whole document equals the `protoorder off` one.
#[test]
fn callee_first_runs_the_structsynth_convergence_sweep() {
    let bin = repo_root()
        .join("decompiler/crates/kuna-analysis/tests/fixtures/structsynthchain_x86_64")
        .to_str()
        .unwrap()
        .to_string();
    let sp = specs();
    let base = ["decompile-all", bin.as_str(), "--sleighpath", sp.as_str(), "--option", "structsynth", "param"];
    let (default, stderr, ok) = run_kuna(&base);
    assert!(ok, "kuna decompile-all failed: {stderr}");
    let mut off_args = base.to_vec();
    off_args.extend_from_slice(&["--option", "protoorder", "off"]);
    let (off, stderr, ok) = run_kuna(&off_args);
    assert!(ok, "kuna decompile-all --option protoorder off failed: {stderr}");
    let proto = |name: &str| {
        default.lines().find(|l| l.starts_with(&format!("long {name}("))).map(str::to_string).unwrap_or_default()
    };
    assert_eq!(proto("fb").replace("fb", "f"), proto("fc").replace("fc", "f"), "fb was not moved onto fc's structure:\n{default}");
    assert_eq!(default, off, "the callee-first run differs from the address-order run");
}

/// `fill_words` stores an eight-byte constant at each word of the buffer it hands
/// the byte-reading `peek`, and `fill_many` stores 520 of them at fixed places,
/// more addresses than the vote's access walk follows. Taken as a vote, `peek`'s
/// `unsigned char *` printed every one of those stores as eight byte stores; both
/// votes are refused, at the default and with `--option ptrfromuse off`.
#[test]
fn a_byte_pointee_vote_keeps_word_fills_and_long_callers_whole() {
    let bin = repo_root()
        .join("decompiler/crates/kuna-analysis/tests/fixtures/protoorder_widefill_x86_64")
        .to_str()
        .unwrap()
        .to_string();
    let sp = specs();
    for off in [false, true] {
        let mut args = vec!["decompile-all", bin.as_str(), "--sleighpath", &sp];
        if off {
            args.extend_from_slice(&["--option", "ptrfromuse", "off"]);
        }
        let (stdout, stderr, ok) = run_kuna(&args);
        assert!(ok, "kuna decompile-all failed: {stderr}");
        for (name, store) in [("fill_words", "= 0x102030405060708;"), ("fill_many", "= 0x2020726174737575;")] {
            let body = stdout
                .split("// Function: ")
                .find(|c| c.starts_with(&format!("{name} ")))
                .unwrap_or_else(|| panic!("{name} is printed"));
            assert!(body.contains(store), "{name}'s eight-byte store was split (off={off}):\n{body}");
            assert!(!body.contains("unsigned char *a0"), "{name} took the byte-pointer vote (off={off}):\n{body}");
        }
    }
}

/// `--jobs-full-load` gives every worker the parent's own load instead of the
/// inventory hand-off.  It exists as the paranoid option, so it has to agree with
/// the hand-off, not merely with itself.
#[test]
fn jobs_full_load_agrees_with_the_inventory_handoff() {
    let bin = fauxware();
    let sp = specs();
    let base = [
        "decompile-all",
        bin.as_str(),
        "--json",
        "--max-fn-seconds",
        "0",
        "--sleighpath",
        sp.as_str(),
    ];
    let (want, stderr, ok) = run_kuna(&base);
    if !ok {
        panic!("kuna decompile-all failed: {stderr}");
    }
    let mut args = base.to_vec();
    args.extend_from_slice(&["--jobs", "3", "--jobs-full-load"]);
    let (got, stderr, ok) = run_kuna(&args);
    assert!(ok, "kuna decompile-all --jobs-full-load failed: {stderr}");
    assert_eq!(got, want, "--jobs-full-load moved the document");
}

/// The sharp case for the inventory hand-off, and the one the byte-identity
/// tests above cannot reach: a **namespaced C++ callee**.  Seeding the parent's
/// inventory into a worker has to be strictly additive, because a name is
/// installed into the scope its `::` path names — re-register an address the
/// worker's own load already named and that address ends up with two function
/// symbols in different scopes, at which point the across-scopes display lookup
/// answers with the other one and `main` prints `sub_401136(...)` where the
/// serial run printed `foo::Bar::baz(...)`.  Preserving callee names is what the
/// hand-off exists for, so it gets a fixture where it can actually fail.
#[test]
fn jobs_preserves_namespaced_cpp_callee_names() {
    let bin = cpp_mangled();
    let sp = specs();
    let base =
        ["decompile-all", bin.as_str(), "--max-fn-seconds", "0", "--sleighpath", sp.as_str()];
    let (want, stderr, ok) = run_kuna(&base);
    if !ok {
        panic!("kuna decompile-all failed: {stderr}");
    }
    assert!(
        want.contains("foo::Bar::baz("),
        "the fixture no longer calls a namespaced member, so this pins nothing:\n{want}"
    );

    for chunk in ["1", "2", "3"] {
        let mut args = base.to_vec();
        args.extend_from_slice(&["--jobs", "6", "--jobs-chunk", chunk]);
        let (got, stderr, ok) = run_kuna(&args);
        assert!(ok, "kuna decompile-all --jobs 6 --jobs-chunk {chunk} failed: {stderr}");
        assert_eq!(got, want, "--jobs-chunk {chunk} moved a namespaced callee name");
    }
}

/// The other half of the surface: `--jobs auto` resolves the worker count from
/// the machine (cores, capped, then trimmed to what free memory holds) instead
/// of the command line, and the plain concatenated-C output has no record
/// framing to hide a mis-ordered merge.  Both have to land on the serial
/// document exactly.
#[test]
fn jobs_auto_and_the_plain_c_surface_match_serial() {
    let bin = fauxware();
    let sp = specs();
    let base =
        ["decompile-all", bin.as_str(), "--max-fn-seconds", "0", "--sleighpath", sp.as_str()];
    let (want, stderr, ok) = run_kuna(&base);
    if !ok {
        panic!("kuna decompile-all failed: {stderr}");
    }
    assert!(want.matches("// Function:").count() > 1, "the fixture must hold several functions");

    for extra in [vec!["--jobs", "auto"], vec!["--jobs", "3", "--jobs-chunk", "1"]] {
        let mut args = base.to_vec();
        args.extend_from_slice(&extra);
        let (got, stderr, ok) = run_kuna(&args);
        assert!(ok, "kuna decompile-all {extra:?} failed: {stderr}");
        // Agreeing with the serial document is also what a silent fall back to
        // the serial path would do, so the pool has to be seen coming up.
        assert!(stderr.contains("worker process(es)"), "{extra:?} ran no pool:\n{stderr}");
        assert_eq!(got, want, "{extra:?} moved the concatenated-C document");
    }
}

/// A sharded run names every synthesized structure as the serial run does, on
/// both surfaces and at every job count. `structsynthchain_x86_64` takes the
/// convergence sweep (`fb` is decided again onto `fc`'s structure while `fa`
/// keeps the superseded one), `itaniumrtti_x86_64.so` mints five structures,
/// the i386 PE is a second architecture and loader, and in
/// `structsynth_teb_pe_x86_64.exe` the `TEB` type comes from a function that
/// synthesizes nothing. Every path lands on the same document: the renamed
/// first decompiles, the second decompile with the replayed names that
/// `synth:force` makes every function take, and the one-worker serial path.
///
/// Both sides run `--option protoorder off`: the serial default decompiles
/// callees first and runs the batch convergence sweep, an order a pool cannot
/// take ([`jobs_notes_that_the_default_callee_first_order_is_serial_only`]).
#[test]
fn jobs_names_synthesized_structs_as_the_serial_run_does() {
    let sp = specs();
    for (fixture, pinned) in [
        ("structsynthchain_x86_64", "long fb(struct_1 *a0)"),
        ("itaniumrtti_x86_64.so", "struct_4 *"),
        ("explicit_branch_assertion_pe_i386.exe", "struct_0 *"),
        ("structsynth_teb_pe_x86_64.exe", "TEB *teb;"),
    ] {
        let bin = repo_root()
            .join("decompiler/crates/kuna-analysis/tests/fixtures")
            .join(fixture)
            .to_str()
            .unwrap()
            .to_string();
        for json in [false, true] {
            let mut base = vec![
                "decompile-all", bin.as_str(), "--max-fn-seconds", "0", "--sleighpath", &sp,
                "--option", "protoorder", "off",
            ];
            if json {
                base.push("--json");
            }
            let (want, stderr, ok) = run_kuna(&base);
            if !ok {
                panic!("kuna decompile-all {fixture} failed: {stderr}");
            }
            assert!(want.contains(pinned), "{fixture} stopped synthesizing {pinned:?}");
            assert!(want.contains("struct_0"), "{fixture} stopped synthesizing");
            for pool in [&["--jobs", "2", "--jobs-chunk", "1"][..], &["--jobs", "4"][..]] {
                let (got, stderr, ok) = run_kuna(&[&base[..], pool].concat());
                assert!(ok, "{fixture} {pool:?} failed: {stderr}");
                assert!(
                    stderr.contains("named as --jobs 1 names them"),
                    "{fixture} {pool:?} never named the structures:\n{stderr}"
                );
                assert!(!stderr.contains("[kuna --jobs] note:"), "{fixture} {pool:?} fell back:\n{stderr}");
                assert_eq!(got, want, "{fixture} {pool:?} (json {json}) moved the document");
            }
            for (fault, says) in [
                ("synth:force", "0 renamed"),
                ("synth:serial", "decompiled again in order by one worker process"),
            ] {
                let (got, stderr, ok) =
                    run_kuna_env(&[&base[..], &["--jobs", "3", "--jobs-chunk", "1"]].concat(), &[("KUNA_JOBS_FAULT", fault)]);
                assert!(ok, "{fixture} {fault} failed: {stderr}");
                assert!(stderr.contains(says), "{fixture} {fault}: {stderr}");
                assert_eq!(got, want, "{fixture} {fault} moved the document");
            }
        }
    }
}

/// A worker that cannot install the replayed structures takes its chunk down
/// with it. The functions it was given are the record pool's, not lost work:
/// the run decompiles them again by the one-worker serial path instead of
/// keeping the dead worker's `error` records.
#[test]
fn jobs_falls_back_when_a_worker_cannot_install_the_replayed_structures() {
    let sp = specs();
    let bin = repo_root()
        .join("decompiler/crates/kuna-analysis/tests/fixtures/itaniumrtti_x86_64.so")
        .to_str()
        .unwrap()
        .to_string();
    // The serial side runs `--option protoorder off`: the callee-first order is
    // the serial run a pool cannot take, here as everywhere else.
    let base = ["decompile-all", bin.as_str(), "--max-fn-seconds", "0", "--sleighpath", sp.as_str(),
        "--option", "protoorder", "off"];
    let (want, stderr, ok) = run_kuna(&base);
    if !ok {
        panic!("kuna decompile-all failed: {stderr}");
    }
    assert!(want.contains("struct_4 *"), "the fixture stopped synthesizing");
    let (got, stderr, ok) = run_kuna_env(
        &[&base[..], &["--jobs", "2", "--jobs-chunk", "1"]].concat(),
        &[("KUNA_JOBS_FAULT", "synth:force,synth:noinstall")],
    );
    assert!(ok, "the run failed: {stderr}");
    assert!(
        stderr.contains("a function failed when decompiled again with the serial names"),
        "no fallback: {stderr}"
    );
    assert_eq!(got, want, "a worker that could not install the structures moved the document");
}

/// `pebnames` creates the `PEB` type the first time a function reads the PEB,
/// so only the worker that decompiled one holds it, and a synthesized
/// structure with a `PEB *` field cannot be installed in another. The run
/// names its structures by the one-worker serial path instead, and loses no
/// function.
#[test]
fn jobs_falls_back_when_a_structure_holds_a_type_other_workers_lack() {
    let sp = specs();
    let bin = repo_root()
        .join("decompiler/crates/kuna-analysis/tests/fixtures/structsynth_peb_pe_x86_64.exe")
        .to_str()
        .unwrap()
        .to_string();
    let base = ["decompile-all", bin.as_str(), "--max-fn-seconds", "0", "--sleighpath", sp.as_str(),
        "--option", "protoorder", "off"];
    let (want, stderr, ok) = run_kuna(&base);
    if !ok {
        panic!("kuna decompile-all failed: {stderr}");
    }
    assert!(want.contains("void store_a(struct_0 *a0"), "the fixture stopped synthesizing:\n{want}");
    assert!(!want.contains("error"), "{want}");
    for pool in [&["--jobs", "2"][..], &["--jobs", "4", "--jobs-chunk", "1"][..]] {
        let (got, stderr, ok) = run_kuna(&[&base[..], pool].concat());
        assert!(ok, "{pool:?} failed: {stderr}");
        assert!(stderr.contains("a field type another process cannot rebuild"), "{pool:?}: {stderr}");
        assert_eq!(got, want, "{pool:?} moved the document");
    }
}

/// `--option structsynth off` still reaches every worker, and a run that asked
/// for it hears nothing about structures.
#[test]
fn jobs_structsynth_off_is_the_serial_structsynth_off_document() {
    let bin = repo_root()
        .join("decompiler/crates/kuna-analysis/tests/fixtures/itaniumrtti_x86_64.so")
        .to_str()
        .unwrap()
        .to_string();
    let sp = specs();
    let off = [
        "decompile-all",
        bin.as_str(),
        "--max-fn-seconds",
        "0",
        "--sleighpath",
        sp.as_str(),
        "--option",
        "protoorder",
        "off",
        "--option",
        "structsynth",
        "off",
    ];
    let (want, stderr, ok) = run_kuna(&off);
    if !ok {
        panic!("kuna decompile-all failed: {stderr}");
    }
    assert!(!want.contains("struct_0"), "structsynth off still synthesized");
    let (got, stderr, ok) = run_kuna(&[&off[..], &["--jobs", "2", "--jobs-chunk", "1"]].concat());
    assert!(ok, "--jobs 2 structsynth-off decompile-all failed: {stderr}");
    assert!(!stderr.contains("structsynth"), "a structure line for a run that asked for off: {stderr}");
    assert_eq!(got, want, "--jobs 2 --option structsynth off moved the document");
}

/// A pool cannot honour a policy it cannot express, so the ones it cannot are
/// refused up front rather than silently dropped in the shards — as is a worker
/// or chunk count that is not a count at all.
#[test]
fn jobs_refuses_what_a_pool_cannot_carry() {
    let bin = fauxware();
    let (_, stderr, ok) = run_kuna(&[
        "decompile-all",
        &bin,
        "--json",
        "--jobs",
        "4",
        "--assert",
        "name v1 flag",
        "--sleighpath",
        &specs(),
    ]);
    assert!(!ok, "--assert with --jobs must be refused");
    assert!(stderr.contains("--assert and --jobs"), "the refusal must say why:\n{stderr}");

    let (_, stderr, ok) =
        run_kuna(&["decompile-all", &bin, "--json", "--jobs", "0", "--sleighpath", &specs()]);
    assert!(!ok, "--jobs 0 must be refused");
    assert!(stderr.contains("--jobs"), "the refusal must name the flag:\n{stderr}");

    let (_, stderr, ok) = run_kuna(&[
        "decompile-all",
        &bin,
        "--json",
        "--jobs",
        "2",
        "--jobs-chunk",
        "0",
        "--sleighpath",
        &specs(),
    ]);
    assert!(!ok, "--jobs-chunk 0 must be refused, not rounded up to a real chunk");
    assert!(stderr.contains("--jobs-chunk"), "the refusal must name the flag:\n{stderr}");
}

/// The pool's scratch directory carries the whole program's symbol inventory and
/// every function's decompiled C, so it must not outlive the run.  Its name
/// carries the parent's pid, which is what makes this checkable while sibling
/// tests are running pools of their own.
#[test]
fn jobs_leaves_no_scratch_directory_behind() {
    let bin = fauxware();
    let mut child = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .env_remove("KUNA_DECOMP_DBG")
        .env_remove("KUNA_DECOMP_TEST")
        .env_remove("KUNA_SLACOMP")
        .args([
            "decompile-all",
            &bin,
            "--json",
            "--max-fn-seconds",
            "0",
            "--jobs",
            "4",
            "--sleighpath",
            &specs(),
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("failed to spawn the kuna binary");
    let pid = child.id();
    let status = child.wait().expect("wait on the kuna binary");
    assert!(status.success(), "pooled decompilation failed: {status}");
    let mine = format!("kuna-jobs-{pid}-");
    let left: Vec<String> = std::fs::read_dir(std::env::temp_dir())
        .into_iter()
        .flatten()
        .flatten()
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .filter(|n| n.starts_with(&mine))
        .collect();
    assert!(left.is_empty(), "a finished --jobs run left {left:?} behind");
}

/// Killing the parent must take the pool with it.  `--jobs` is for hour-long
/// runs, so the parent being cancelled or timed out is a normal event, and it
/// used to leave every worker reparented to init with a scratch directory
/// holding the program's symbol inventory and every function's C.  SIGKILL is
/// the case that decides the design: no handler in the parent can cover it, so
/// each worker watches the pipe whose only write end its parent holds.
#[cfg(target_os = "linux")]
#[test]
fn killing_the_parent_takes_the_workers_and_the_scratch_dir_with_it() {
    // Every thread's list: the workers are spawned by the pool threads, so the
    // main thread's own `children` file is empty for the whole run.
    fn children_of(pid: u32) -> Vec<u32> {
        std::fs::read_dir(format!("/proc/{pid}/task"))
            .into_iter()
            .flatten()
            .flatten()
            .filter_map(|task| std::fs::read_to_string(task.path().join("children")).ok())
            .flat_map(|list| {
                list.split_whitespace().filter_map(|s| s.parse().ok()).collect::<Vec<u32>>()
            })
            .collect()
    }
    fn scratch_of(pid: u32) -> Vec<PathBuf> {
        let mine = format!("kuna-jobs-{pid}-");
        std::fs::read_dir(std::env::temp_dir())
            .into_iter()
            .flatten()
            .flatten()
            .map(|e| e.path())
            .filter(|p| {
                p.file_name().is_some_and(|n| n.to_string_lossy().starts_with(&mine))
            })
            .collect()
    }

    let bin = hang_repro();
    let mut child = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .env_remove("KUNA_DECOMP_DBG")
        .env_remove("KUNA_DECOMP_TEST")
        .env_remove("KUNA_SLACOMP")
        .args([
            "decompile-all",
            &bin,
            "--json",
            "--max-fn-seconds",
            "0",
            "--jobs",
            "4",
            "--sleighpath",
            &specs(),
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("failed to spawn the kuna binary");
    let pid = child.id();

    // Wait for the pool to be genuinely up: a worker running and the scratch
    // directory on disk. Nothing to prove until both exist.
    let deadline = Instant::now() + Duration::from_secs(120);
    let workers = loop {
        let workers = children_of(pid);
        if !workers.is_empty() && !scratch_of(pid).is_empty() {
            break workers;
        }
        if Instant::now() >= deadline || child.try_wait().expect("try_wait").is_some() {
            let _ = child.kill();
            let _ = child.wait();
            panic!("the worker pool did not start before the deadline");
        }
        std::thread::sleep(Duration::from_millis(100));
    };

    child.kill().expect("SIGKILL the pool parent");
    let _ = child.wait();

    let deadline = Instant::now() + Duration::from_secs(60);
    loop {
        let alive: Vec<u32> = workers
            .iter()
            .copied()
            .filter(|w| PathBuf::from(format!("/proc/{w}")).exists())
            .collect();
        let left = scratch_of(pid);
        if alive.is_empty() && left.is_empty() {
            return;
        }
        assert!(
            Instant::now() < deadline,
            "a SIGKILLed parent left workers {alive:?} and scratch {left:?} behind"
        );
        std::thread::sleep(Duration::from_millis(200));
    }
}

// --- `--jobs N`: a dead worker costs one function --------------------------

/// Parse records while retaining their original bytes for equality checks.
fn json_records(doc: &str) -> Vec<&str> {
    #[derive(serde::Deserialize)]
    struct Records<'a> {
        #[serde(borrow)]
        functions: Vec<&'a serde_json::value::RawValue>,
    }
    let document: Records<'_> = serde_json::from_str(doc).expect("valid function records");
    document.functions.into_iter().map(|record| record.get()).collect()
}

fn record_name(record: &str) -> String {
    let record: serde_json::Value = serde_json::from_str(record).expect("valid function record");
    record.get("name").and_then(serde_json::Value::as_str).expect("function name").to_owned()
}

/// The count a `[kuna <tag>] N function(s) left unfinished ... recovered.` line
/// reports, or `None` when the run printed no such line.
fn recovered_count(stderr: &str) -> Option<usize> {
    let line = stderr.lines().find(|l| l.contains("left unfinished by a failed worker process"))?;
    line.split("] ").nth(1)?.split(' ').next()?.parse().ok()
}

/// The serial `decompile-all --json` document of `fauxware`.
fn fauxware_serial_json() -> String {
    let (want, stderr, ok) = run_kuna(&[
        "decompile-all",
        &fauxware(),
        "--json",
        "--max-fn-seconds",
        "0",
        "--sleighpath",
        &specs(),
    ]);
    if !ok {
        panic!("kuna decompile-all failed: {stderr}");
    }
    want
}

/// A pooled `decompile-all --json` of `fauxware` in ONE chunk (so one worker
/// serves every target and the dispatch order is the planner's longest-first
/// order), under `KUNA_JOBS_FAULT=fault`.
fn fauxware_pooled_with_fault(fault: &str, max_fn_seconds: &str) -> (String, String, bool) {
    let bin = fauxware();
    let sp = specs();
    let args = [
        "decompile-all",
        bin.as_str(),
        "--json",
        "--max-fn-seconds",
        max_fn_seconds,
        "--sleighpath",
        sp.as_str(),
        "--jobs",
        "2",
        "--jobs-chunk",
        "64",
    ];
    run_kuna_env_with_timeout(&args, &[("KUNA_JOBS_FAULT", fault)], Duration::from_secs(240))
        .unwrap_or_else(|| panic!("KUNA_JOBS_FAULT={fault} wedged the pool"))
}

/// Every record but those named in `lost` must be the serial run's, byte for
/// byte.
fn assert_only_lost_differ(serial: &str, pooled: &str, lost: &[&str]) {
    let want = json_records(serial);
    let got = json_records(pooled);
    assert_eq!(got.len(), want.len(), "one record per target:\n{pooled}");
    for (w, g) in want.iter().zip(&got) {
        assert_eq!(record_name(w), record_name(g), "target order moved");
        if !lost.contains(&record_name(w).as_str()) {
            assert_eq!(g, w, "{} is not the serial record", record_name(w));
        }
    }
}

/// The issue this exists for: one function panicking its worker used to turn
/// every function of its chunk (26..512 wide on a large binary) into an `error`
/// record.  The functions the dead worker never delivered are re-run on their
/// own, so the document is the serial one except for the function that
/// panicked, whose record is the crash it repeats when run alone -- once, not
/// in a loop.  `main` is the fixture's largest function, so the planner puts it
/// first and the worker dies before delivering anything (the worker had not
/// finished a chunk yet, which is exactly when a death could also have been its
/// load); `authenticate` comes third, after a delivered prefix.
#[test]
fn jobs_a_worker_panic_loses_only_the_function_that_panicked() {
    let serial = fauxware_serial_json();
    let total = json_records(&serial).len();
    for (name, addr) in [("main", "0x40071d"), ("authenticate", "0x400664")] {
        let (got, stderr, ok) = fauxware_pooled_with_fault(&format!("panic:{addr}"), "0");
        assert!(ok, "one panicking function must not fail the run:\n{stderr}");
        assert_only_lost_differ(&serial, &got, &[name]);
        let lost = json_records(&got).into_iter().find(|r| record_name(r) == name).unwrap();
        assert!(
            lost.contains("\"error\": \"worker chunk failed (worker exited: exit status: 101)\""),
            "{name} must carry its own crash:\n{lost}"
        );
        assert_eq!(
            stderr.matches(&format!("KUNA_JOBS_FAULT: injected panic at {addr}")).count(),
            2,
            "{name} is run once in its chunk and once alone, never again:\n{stderr}"
        );
        let recovered =
            recovered_count(&stderr).unwrap_or_else(|| panic!("no recovery line:\n{stderr}"));
        if name == "main" {
            assert_eq!(recovered, total - 1, "every other function was re-run:\n{stderr}");
        } else {
            assert!(recovered >= 1, "{name} was not first, so it had bystanders:\n{stderr}");
        }
        assert!(
            stderr.contains(
                "warning: 1 function(s) have no result because their worker process failed"
            ) && stderr.contains("1 of them failed again when re-run on their own."),
            "the warning must count only the function that failed alone:\n{stderr}"
        );
    }
}

/// A death that is not the function's own -- an OOM kill, a signal -- is
/// recovered too: the function that was running is re-run first, and when it
/// succeeds alone the document is the serial one and nothing is reported lost.
#[test]
fn jobs_a_transient_worker_death_loses_nothing() {
    let serial = fauxware_serial_json();
    let (got, stderr, ok) = fauxware_pooled_with_fault("panic-once:0x40071d", "0");
    assert!(ok, "{stderr}");
    assert_eq!(got, serial, "a recovered run must be the serial document");
    assert_eq!(stderr.matches("KUNA_JOBS_FAULT: injected panic").count(), 1, "{stderr}");
    assert_eq!(recovered_count(&stderr), Some(json_records(&serial).len()), "{stderr}");
    assert!(!stderr.contains("have no result"), "nothing was lost:\n{stderr}");
}

/// The stall watchdog's kill is the other way a worker dies.  Its bystanders
/// are re-run like a crash's, but the function it was running is not: it has
/// already run four times past the per-function budget, and a second attempt
/// would cost the same stall window again.  `rejected` comes after a delivered
/// prefix, so the warm (4 s) stall window applies.
///
/// A bystander that stalls when re-run costs itself and the chunk re-runs on:
/// stalls are common on large binaries, and a stall in a planned chunk costs
/// one function too.  A second stalled re-run in the same chunk stops it, which
/// caps the extra stall windows a chunk can wait out.  `read` and `strcmp` are
/// the third and sixth re-runs, each after a recovered one, so both windows are
/// warm too.
#[test]
fn jobs_a_stalled_worker_loses_only_the_function_that_stalled() {
    let serial = fauxware_serial_json();
    let (got, stderr, ok) = fauxware_pooled_with_fault("stall:0x4006fd", "1");
    assert!(ok, "{stderr}");
    assert_only_lost_differ(&serial, &got, &["rejected"]);
    let lost = json_records(&got).into_iter().find(|r| record_name(r) == "rejected").unwrap();
    assert!(
        lost.contains("\"error\": \"worker stalled past the per-function watchdog (1s)"),
        "{lost}"
    );
    assert_eq!(
        stderr.matches("KUNA_JOBS_FAULT: injected stall at 0x4006fd").count(),
        1,
        "a stalled function is not re-run:\n{stderr}"
    );
    assert!(recovered_count(&stderr).is_some_and(|n| n >= 1), "{stderr}");
    assert!(!stderr.contains("failed again"), "{stderr}");

    let (got, stderr, ok) = fauxware_pooled_with_fault("stall:0x4006fd,stall:0x400530", "1");
    assert!(ok, "{stderr}");
    assert_only_lost_differ(&serial, &got, &["rejected", "read"]);
    assert!(!got.contains("not re-run"), "one stalled re-run stops nothing:\n{got}");
    assert_eq!(recovered_count(&stderr), Some(11), "{stderr}");

    let stalls = "stall:0x4006fd,stall:0x400530,stall:0x400550";
    let (got, stderr, ok) = fauxware_pooled_with_fault(stalls, "1");
    assert!(ok, "{stderr}");
    let not_rerun = ["sub_400500", "accepted", "__libc_start_main", "printf", "_fini", "open"];
    let mut lost = vec!["rejected", "read", "strcmp"];
    lost.extend(not_rerun);
    assert_only_lost_differ(&serial, &got, &lost);
    let stalled =
        "\"error\": \"worker stalled past the per-function watchdog (1s); the worker was killed";
    for r in json_records(&got).into_iter().filter(|r| lost.contains(&record_name(r).as_str())) {
        assert!(r.contains(stalled), "{r}");
        let marked = r.contains("; not re-run: two functions re-run from its chunk stalled\"");
        assert_eq!(marked, not_rerun.contains(&record_name(r).as_str()), "{r}");
    }
    for addr in ["0x4006fd", "0x400530", "0x400550"] {
        let fired = stderr.matches(&format!("KUNA_JOBS_FAULT: injected stall at {addr}")).count();
        assert_eq!(fired, 1, "{addr} stalled once:\n{stderr}");
    }
    assert_eq!(recovered_count(&stderr), Some(4), "{stderr}");
    assert!(
        stderr.contains("warning: 9 function(s) have no result")
            && stderr.contains("2 of them failed again when re-run on their own."),
        "{stderr}"
    );
}

/// The failure #578 exists for, at its worst: the planner cuts chunks from a
/// size-sorted order, so functions that crash alike sit side by side.  Five
/// crashers in the first five places of the chunk (the largest functions)
/// must cost those five and nothing else, not the chunk.
#[test]
fn jobs_neighbouring_crashers_do_not_forfeit_their_chunk() {
    let serial = fauxware_serial_json();
    let crashers = [
        ("main", "0x40071d"),
        ("__libc_csu_init", "0x4007e0"),
        ("authenticate", "0x400664"),
        ("__do_global_dtors_aux", "0x4005d0"),
        ("__do_global_ctors_aux", "0x400880"),
    ];
    let fault: Vec<String> = crashers.iter().map(|(_, a)| format!("panic:{a}")).collect();
    let (got, stderr, ok) = fauxware_pooled_with_fault(&fault.join(","), "0");
    assert!(ok, "sixteen functions came back, so the run succeeded:\n{stderr}");
    let names: Vec<&str> = crashers.iter().map(|(n, _)| *n).collect();
    assert_only_lost_differ(&serial, &got, &names);
    assert!(!got.contains("not re-run"), "no function was given up:\n{got}");
    assert_eq!(recovered_count(&stderr), Some(16), "{stderr}");
    assert_eq!(
        stderr.matches("KUNA_JOBS_FAULT: injected panic").count(),
        6,
        "the chunk's death, then one re-run apiece for the five crashers:\n{stderr}"
    );
    assert!(stderr.contains("5 of them failed again when re-run on their own."), "{stderr}");
}

/// Re-running must never become a loop, nor pay a load for every function of a
/// run whose workers die on everything.  A spawn the OS refuses is not re-run at
/// all; a re-run that cannot spawn ends its chunk's re-runs; a chunk whose
/// re-run bystanders mostly fail stops after eight of them; and a run whose
/// re-runs fail more often than its workers deliver stops re-running.  Each
/// case must end, with exactly one record per target, and every function left
/// behind says it was not re-run.
#[test]
fn jobs_rerunning_a_dead_worker_never_loops() {
    let serial = fauxware_serial_json();
    let total = json_records(&serial).len();
    let every_record_failed = |doc: &str, prefix: &str| {
        let records = json_records(doc);
        assert_eq!(records.len(), total, "{doc}");
        for r in records {
            assert!(r.contains(&format!("\"error\": \"{prefix}")), "{}:\n{r}", record_name(r));
        }
    };
    let crashed = "worker chunk failed (worker exited: exit status: 101)";
    let panics = |stderr: &str| stderr.matches("KUNA_JOBS_FAULT: injected panic").count();

    let (got, stderr, ok) = fauxware_pooled_with_fault("spawn:0", "0");
    assert!(!ok, "a run that decompiled nothing must fail:\n{stderr}");
    every_record_failed(&got, "cannot spawn worker");
    assert!(recovered_count(&stderr).is_none() && !stderr.contains("failed again"), "{stderr}");

    let (got, stderr, ok) = fauxware_pooled_with_fault("panic:0x40071d,spawn:1", "0");
    assert!(!ok, "{stderr}");
    every_record_failed(&got, crashed);
    let could_not_start = "; not re-run: its re-run could not start (cannot spawn";
    assert_eq!(got.matches(could_not_start).count(), total, "{got}");
    assert_eq!(panics(&stderr), 1, "no re-run could start, so nothing ran twice:\n{stderr}");

    let (got, stderr, ok) = fauxware_pooled_with_fault("panic:*", "0");
    assert!(!ok, "{stderr}");
    every_record_failed(&got, crashed);
    assert_eq!(panics(&stderr), 10, "the chunk's death, main, then eight bystanders:\n{stderr}");
    assert!(stderr.contains("9 of them failed again when re-run on their own."), "{stderr}");
    let given_up = "; not re-run: most functions re-run from its chunk failed again";
    assert_eq!(got.matches(given_up).count(), total - 9, "{got}");
    assert!(!stderr.contains("not re-run while that lasts"), "{stderr}");

    // One function per chunk on two threads: every chunk dies and so does its
    // re-run, nothing is ever delivered, so no re-run starts after 16 have
    // failed (17 when the other thread had one under way).
    let bin = fauxware();
    let sp = specs();
    let args = [
        "decompile-all",
        bin.as_str(),
        "--json",
        "--max-fn-seconds",
        "0",
        "--sleighpath",
        sp.as_str(),
        "--jobs",
        "2",
        "--jobs-chunk",
        "1",
    ];
    let cap = Duration::from_secs(240);
    let (got, stderr, ok) = run_kuna_env_with_timeout(&args, &[("KUNA_JOBS_FAULT", "panic:*")], cap)
        .expect("workers that die on everything wedged the pool");
    assert!(!ok, "{stderr}");
    every_record_failed(&got, crashed);
    let reruns = panics(&stderr) - total;
    assert!((16..=17).contains(&reruns), "16 or 17 failed re-runs, not {reruns}:\n{stderr}");
    assert_eq!(stderr.matches("are not re-run while that lasts").count(), 1, "{stderr}");
    let run_stopped = "; not re-run: re-runs in this run had failed more often than workers had";
    assert_eq!(got.matches(run_stopped).count(), total - reruns, "{got}");
}

// --- `--jobs N`: the decode lanes --------------------------------------------

/// Run `kuna` with extra environment, for the decode-lane knobs.
fn run_kuna_env(args: &[&str], env: &[(&str, &str)]) -> (String, String, bool) {
    let out = kuna_command(env).args(args).output().expect("failed to spawn the kuna binary");
    (
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
        out.status.success(),
    )
}

/// Everything the walk says about itself -- the plan line, the stats line, the
/// self-check -- removed, so the rest of stderr can be compared byte for byte
/// against a serial run's.
fn without_the_plan_line(stderr: &str) -> String {
    stderr
        .lines()
        .filter(|l| !l.starts_with("[kuna --jobs]"))
        .map(|l| format!("{l}\n"))
        .collect()
}

/// The plan line is printed BEFORE the walk and is not withdrawn when the lanes
/// fall back, so on its own it proves only that the flag was parsed. What proves
/// the document came off the lanes is the absence of a fallback line plus the
/// stats line, which only a completed parallel walk reaches.
fn assert_lanes_produced_it(stderr: &str, what: &str) {
    assert!(
        !stderr.contains("[kuna --jobs] decode: serial ("),
        "{what} fell back to the serial walk, so the comparison above is vacuous:\n{stderr}"
    );
    let stats = stderr
        .lines()
        .find(|l| l.starts_with("[kuna --jobs] decode stats:"))
        .unwrap_or_else(|| panic!("{what} printed no stats line:\n{stderr}"));
    assert!(stats.contains(" rounds="), "{what}: no round count in `{stats}`");
    let decodes: usize = stats
        .split_whitespace()
        .find_map(|f| f.strip_prefix("decodes="))
        .and_then(|n| n.parse().ok())
        .unwrap_or_else(|| panic!("{what}: no decode count in `{stats}`"));
    assert!(decodes > 0, "{what}: the lanes decoded nothing -- `{stats}`");
}

/// The decode lanes' whole contract: the inventory must not depend on how many
/// threads decoded it.  The size floor is lowered so the lanes really run on an
/// in-repo fixture -- without it every assertion here would be a serial run
/// agreeing with a serial run -- and the plan line is asserted, so a silent
/// refusal cannot read as a pass.
#[test]
fn jobs_decode_lanes_are_byte_identical_to_serial() {
    let bin = fauxware();
    let sp = specs();
    let lanes_on = [("KUNA_DECODE_MIN_BYTES", "0"), ("KUNA_DECODE_STATS", "1")];
    let (want, want_err, ok) = run_kuna(&["functions", &bin, "--json", "--sleighpath", &sp]);
    if !ok {
        panic!("kuna functions failed: {want_err}");
    }
    assert!(want.contains("\"name\""), "the serial run enumerated nothing:\n{want}");
    assert!(
        !want_err.contains("[kuna --jobs]"),
        "`--jobs` absent must print no plan line at all:\n{want_err}"
    );

    for jobs in ["1", "2", "3", "4", "8", "auto"] {
        let args = ["functions", &bin, "--json", "--sleighpath", &sp, "--jobs", jobs];
        let (got, stderr, ok) = run_kuna_env(&args, &lanes_on);
        assert!(ok, "kuna functions --jobs {jobs} failed: {stderr}");
        assert_eq!(got, want, "--jobs {jobs} moved the inventory");
        assert_eq!(
            without_the_plan_line(&stderr),
            without_the_plan_line(&want_err),
            "--jobs {jobs} moved stderr beyond its own plan line"
        );
        if jobs == "1" {
            assert!(
                !stderr.contains("[kuna --jobs]"),
                "`--jobs 1` is the serial path and says nothing:\n{stderr}"
            );
        } else {
            assert!(
                stderr.contains("[kuna --jobs] decode:") && stderr.contains(" lanes, "),
                "--jobs {jobs} never reached the lanes:\n{stderr}"
            );
            assert_lanes_produced_it(&stderr, &format!("--jobs {jobs}"));
        }
    }

    // The floor itself: without the override the same flag declines, and still
    // produces the same document.
    let (got, stderr, ok) =
        run_kuna(&["functions", &bin, "--json", "--sleighpath", &sp, "--jobs", "8"]);
    assert!(ok, "kuna functions --jobs 8 failed: {stderr}");
    assert_eq!(got, want, "the refused path moved the inventory");
    assert!(
        stderr.contains("[kuna --jobs] decode: serial (executable image too small)"),
        "a fixture under the size floor must say why it declined:\n{stderr}"
    );
}

/// A language whose constructors carry `globalset` must decline: a decode there
/// writes the shared context database at another address.  Byte-identity on ARM
/// rests entirely on this, so the decline is asserted, not assumed.
#[test]
fn jobs_decode_lanes_decline_on_a_context_committing_language() {
    let bin = arm_thumb();
    let sp = specs();
    let args = ["functions", &bin, "--json", "--sleighpath", &sp];
    let (want, stderr, ok) = run_kuna(&args);
    if !ok {
        panic!("kuna functions failed: {stderr}");
    }
    let mut with_jobs = args.to_vec();
    with_jobs.extend_from_slice(&["--jobs", "8"]);
    let (got, stderr, ok) = run_kuna_env(&with_jobs, &[("KUNA_DECODE_MIN_BYTES", "0")]);
    assert!(ok, "kuna functions --jobs 8 failed on ARM: {stderr}");
    assert_eq!(got, want, "the ARM decline moved the inventory");
    assert!(
        stderr.contains("[kuna --jobs] decode: serial (language commits context)"),
        "ARM must decline with the language reason:\n{stderr}"
    );
}

/// How long a fault-injected run may take before it is a deadlock rather than a
/// slow fixture. fauxware walks in well under a second either way.
const FAULT_CAP: Duration = Duration::from_secs(180);

/// A lane that dies must cost speed, not correctness: the fallback re-walks
/// serially and says so.
#[test]
fn a_dead_lane_falls_back_to_the_serial_walk() {
    let bin = fauxware();
    let sp = specs();
    let args = ["functions", &bin, "--json", "--sleighpath", &sp];
    let (want, stderr, ok) = run_kuna(&args);
    if !ok {
        panic!("kuna functions failed: {stderr}");
    }
    let mut with_jobs = args.to_vec();
    with_jobs.extend_from_slice(&["--jobs", "4"]);
    for lane in ["0", "2"] {
        let (got, stderr, ok) = run_kuna_env_with_timeout(
            &with_jobs,
            &[("KUNA_DECODE_MIN_BYTES", "0"), ("KUNA_DECODE_FAULT", lane)],
            FAULT_CAP,
        )
        .unwrap_or_else(|| panic!("lane {lane}'s fault deadlocked the run"));
        assert!(ok, "a lane panic must not fail the run (lane {lane}): {stderr}");
        assert_eq!(got, want, "the serial fallback moved the inventory (lane {lane})");
        assert!(
            stderr.contains("[kuna --jobs] decode: serial (lane fault)"),
            "lane {lane}'s panic must be reported as a fallback:\n{stderr}"
        );
        assert!(
            !stderr.contains("panicked at"),
            "the fallback is the report; the runtime's panic block must not reach the user:\n\
             {stderr}"
        );
    }
}

/// `decompile-all --jobs N` sizes both the decode lanes (in the parent's load)
/// and the worker pool (afterwards).  The document must survive both.
#[test]
fn jobs_decode_lanes_agree_with_serial_on_decompile_all() {
    let bin = fauxware();
    let sp = specs();
    let base =
        ["decompile-all", bin.as_str(), "--json", "--max-fn-seconds", "0", "--sleighpath", &sp];
    let (want, stderr, ok) = run_kuna(&base);
    if !ok {
        panic!("kuna decompile-all failed: {stderr}");
    }
    let mut args = base.to_vec();
    args.extend_from_slice(&["--jobs", "4"]);
    let (got, stderr, ok) =
        run_kuna_env(&args, &[("KUNA_DECODE_MIN_BYTES", "0"), ("KUNA_DECODE_STATS", "1")]);
    assert!(ok, "kuna decompile-all --jobs 4 failed: {stderr}");
    assert_eq!(got, want, "--jobs 4 moved the document");
    assert!(
        stderr.contains("[kuna --jobs] decode:") && stderr.contains(" lanes, "),
        "the parent's load must have run on lanes:\n{stderr}"
    );
    assert_lanes_produced_it(&stderr, "decompile-all --jobs 4");
}


/// A spawn the OS refuses is the one lane failure that cannot report itself: the
/// lanes already parked at the first barrier are joined by `thread::scope`
/// BEFORE the panic resumes, so an unguarded spawn hangs the process forever.
/// It must instead be a refusal like any other.
#[test]
fn a_refused_lane_spawn_falls_back_to_the_serial_walk() {
    let bin = fauxware();
    let sp = specs();
    let args = ["functions", &bin, "--json", "--sleighpath", &sp];
    let (want, stderr, ok) = run_kuna(&args);
    if !ok {
        panic!("kuna functions failed: {stderr}");
    }
    let mut with_jobs = args.to_vec();
    with_jobs.extend_from_slice(&["--jobs", "4"]);
    // Lane 1 is the first spawn (lane 0 is the calling thread), lane 3 the last:
    // the refusal has to release however many lanes are already at the barrier.
    for lane in ["1", "3"] {
        let (got, stderr, ok) = run_kuna_env_with_timeout(
            &with_jobs,
            &[("KUNA_DECODE_MIN_BYTES", "0"), ("KUNA_DECODE_FAULT", &format!("spawn:{lane}"))],
            FAULT_CAP,
        )
        .unwrap_or_else(|| panic!("a refused spawn at lane {lane} deadlocked the run"));
        assert!(ok, "a refused spawn must not fail the run (lane {lane}): {stderr}");
        assert_eq!(got, want, "the serial fallback moved the inventory (lane {lane})");
        assert!(
            stderr.contains("[kuna --jobs] decode: 4 lanes, "),
            "the plan was announced before the spawn failed:\n{stderr}"
        );
        assert!(
            stderr.contains("[kuna --jobs] decode: serial (thread spawn failed)"),
            "a refused spawn at lane {lane} must be reported as a fallback:\n{stderr}"
        );
    }
}

/// `--raw-image` and `--assert` are worker-POOL policy: `kuna functions` never
/// spawns one, so `--jobs` there is only the decode lanes and neither
/// combination may be refused. A raw image's discovery (`rawdiscover`) is a
/// serial sweep and descent with no lanes to hand work to, so `--jobs` is
/// simply inert.
#[test]
fn functions_takes_jobs_with_a_raw_image() {
    let bin = fauxware();
    let sp = specs();
    let args = [
        "functions",
        &bin,
        "--json",
        "--sleighpath",
        &sp,
        "--raw-image",
        "--target",
        "x86:LE:64:default",
        "--base",
        "0x400000",
        "--entry",
        "0x400580",
        "--jobs",
        "4",
    ];
    let (got, stderr, ok) = run_kuna_env(&args, &[("KUNA_DECODE_MIN_BYTES", "0")]);
    assert!(ok, "functions --raw-image --jobs 4 must not be refused: {stderr}");
    assert!(got.contains("\"functions\""), "the raw image enumerated nothing:\n{got}");
    assert!(
        !stderr.contains("does not apply to --raw-image"),
        "the pool's raw-image refusal must not fire on a surface with no pool:\n{stderr}"
    );
    assert!(
        stderr.contains("[kuna --jobs] decode: serial (a raw image's discovery walk has no lanes)"),
        "a raw image's discovery walk takes no decode lanes, and must say so rather than \
         accept the flag and do nothing:\n{stderr}"
    );

    // The pool surface keeps the refusal: there a raw image really is a policy
    // the workers cannot carry.
    let (_, stderr, ok) = run_kuna(&[
        "decompile-all",
        &bin,
        "--json",
        "--sleighpath",
        &sp,
        "--raw-image",
        "--target",
        "x86:LE:64:default",
        "--base",
        "0x400000",
        "--entry",
        "0x400580",
        "--jobs",
        "4",
    ]);
    assert!(!ok && stderr.contains("does not apply to --raw-image"), "{stderr}");
}

/// `--assert bytes` patches the loader's own segment bytes before the plan takes
/// its share of them, so the lanes read the overlay through the very `Arc` the
/// parent patched -- which is what the shared-bytes gate exists to guarantee.
/// The answer must match a serial run of the same overlaid load, and differ from
/// the unpatched one.
#[test]
fn functions_takes_jobs_with_an_assert_overlay() {
    let bin = fauxware();
    let sp = specs();
    // `main`'s `CALL authenticate` at 0x4007ae, re-pointed into the middle of
    // `authenticate` -- so `authenticate` loses its only caller and the
    // orientation document's `no callers` count moves.
    let overlay = "bytes 0x4007af bdfeffff";
    let args =
        ["functions", &bin, "--summary", "--sleighpath", &sp, "--assert", overlay, "--jobs", "4"];
    let (got, stderr, ok) =
        run_kuna_env(&args, &[("KUNA_DECODE_MIN_BYTES", "0"), ("KUNA_DECODE_STATS", "1")]);
    assert!(ok, "functions --assert --jobs 4 must not be refused: {stderr}");
    assert!(
        stderr.contains("[kuna --jobs] decode: 4 lanes, "),
        "the overlaid load must still reach the lanes:\n{stderr}"
    );
    assert_lanes_produced_it(&stderr, "functions --assert --jobs 4");

    let serial = ["functions", &bin, "--summary", "--sleighpath", &sp, "--assert", overlay];
    let (want, want_err, ok) = run_kuna(&serial);
    assert!(ok, "the serial overlaid run failed: {want_err}");
    assert_eq!(got, want, "--jobs 4 moved the overlaid answer");

    // ...and the overlay really did something, or the equality above is empty.
    let (plain, _, ok) = run_kuna(&["functions", &bin, "--summary", "--sleighpath", &sp]);
    assert!(ok, "the unpatched serial run failed");
    assert_ne!(got, plain, "the `{overlay}` overlay changed nothing, so it pins nothing");
}

/// A synthesized field commits to a signedness only when every read of its width
/// does.  `f` reads its 16-bit field into a signed comparison and, after a call,
/// zero-extends it into `sink`'s 32-bit argument; a `short field_0xc` would make
/// the printed call sign-extend, handing `sink` 4294941372 where the binary hands
/// it 0x9abc.  The round trip compiles `f` with the definition `structdefs`
/// prints above it.
#[test]
fn a_sign_contested_synthesized_field_round_trips_through_the_printed_c() {
    let bin = repo_root()
        .join("decompiler/crates/kuna-analysis/tests/fixtures/signfield_zext_x86_64")
        .to_str()
        .unwrap()
        .to_string();
    let sp = specs();
    let (stdout, stderr, ok) = run_kuna(&[
        "decompile-all", &bin, "--functions", "f", "--option", "structdefs", "on", "--sleighpath", &sp,
    ]);
    assert!(ok, "kuna decompile-all failed: {stderr}");
    assert!(stdout.contains("unsigned short field_0xc;"), "{stdout}");
    assert!(stdout.contains("sink(a0->field_0xc);"), "{stdout}");

    if process::optional_output(Command::new("cc").arg("--version")).is_none() {
        eprintln!("signfield round trip: no `cc`, spelling checked only");
        return;
    }
    let dir = std::env::temp_dir().join(format!("kuna-signfield-rt-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let src = dir.join("rt.c");
    let exe = dir.join("rt");
    std::fs::write(
        &src,
        format!(
            "#include <stdio.h>\nstatic unsigned int got;\nvoid touch(void *s) {{ (void)s; }}\n\
             void sink(unsigned int x) {{ got = x; }}\n{stdout}\n\
             int main(void) {{\n  static unsigned long s[2];\n  s[0] = 1;\n  \
             ((unsigned short *)s)[6] = 0x9abc;\n  int (*fp)() = (int (*)())f;\n  fp((void *)s);\n  \
             printf(\"%u\\n\", got);\n  return 0;\n}}\n"
        ),
    )
    .unwrap();
    let cc = Command::new("cc")
        .args(["-std=gnu11", "-w", "-o", exe.to_str().unwrap(), src.to_str().unwrap()])
        .output()
        .expect("spawn cc");
    assert!(cc.status.success(), "the printed f did not compile:\n{}", String::from_utf8_lossy(&cc.stderr));
    let run = process::required_output(&mut Command::new(&exe));
    let got = String::from_utf8_lossy(&run.stdout).trim().to_string();
    let _ = std::fs::remove_dir_all(&dir);
    assert_eq!(got, (0x9abcu32).to_string(), "the printed f hands sink a different value:\n{stdout}");
}

/// An 8-byte union member read both as a `double` and as a `long` is raw bytes,
/// so every read casts the address. A `long field_0x8` made the `movsd` read
/// print `(double)a0->field_0x8`, a value conversion: with 2.5 stored, tag 2
/// returned 4612811918334230528.0. The round trip compiles `vread` with the
/// definition `structdefs` prints above it and compares every tag against the
/// union read directly, over two payloads.
#[test]
fn a_float_and_integer_union_field_round_trips_through_the_printed_c() {
    let bin = repo_root()
        .join("decompiler/crates/kuna-analysis/tests/fixtures/unionfield_fp_x86_64")
        .to_str()
        .unwrap()
        .to_string();
    let sp = specs();
    let (stdout, stderr, ok) = run_kuna(&[
        "decompile-all", &bin, "--functions", "vread", "--option", "structdefs", "on", "--sleighpath", &sp,
    ]);
    assert!(ok, "kuna decompile-all failed: {stderr}");
    assert!(stdout.contains("char field_0x8[8];"), "{stdout}");
    assert!(stdout.contains("return *(double *)a0->field_0x8;"), "{stdout}");

    if process::optional_output(Command::new("cc").arg("--version")).is_none() {
        eprintln!("unionfield round trip: no `cc`, spelling checked only");
        return;
    }
    let dir = std::env::temp_dir().join(format!("kuna-unionfield-rt-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let src = dir.join("rt.c");
    let exe = dir.join("rt");
    std::fs::write(
        &src,
        format!(
            "#include <stdio.h>\n#include <string.h>\n{stdout}\n\
             static double truth(const unsigned char *b) {{\n  int tag; memcpy(&tag, b, 4);\n  \
             union {{ int i; float f; double d; long l; unsigned char c[8]; }} u; memcpy(&u, b + 8, 8);\n  \
             switch (tag) {{ case 0: return u.i; case 1: return u.f; case 2: return u.d;\n  \
             case 3: return (double)u.l; default: return u.c[1]; }}\n}}\n\
             int main(void) {{\n  static unsigned long s[2];\n  unsigned char *b = (unsigned char *)s;\n  \
             int bad = 0;\n  for (int k = 0; k < 2; k++)\n    for (int tag = 0; tag < 5; tag++) {{\n      \
             double d = 2.5;\n      if (k) for (int i = 8; i < 16; i++) b[i] = (unsigned char)(i * 37 + 0x81);\n      \
             else memcpy(b + 8, &d, 8);\n      memcpy(b, &tag, 4);\n      \
             double (*fp)() = (double (*)())vread;\n      double got = fp((void *)s), want = truth(b);\n      \
             if (memcmp(&got, &want, 8)) {{ printf(\"payload %d tag %d: %a != %a\\n\", k, tag, got, want); bad++; }}\n    }}\n  \
             printf(\"%d\\n\", bad);\n  return 0;\n}}\n"
        ),
    )
    .unwrap();
    let cc = Command::new("cc")
        .args(["-std=gnu11", "-w", "-o", exe.to_str().unwrap(), src.to_str().unwrap()])
        .output()
        .expect("spawn cc");
    assert!(cc.status.success(), "the printed vread did not compile:\n{}", String::from_utf8_lossy(&cc.stderr));
    let run = process::required_output(&mut Command::new(&exe));
    let got = String::from_utf8_lossy(&run.stdout).to_string();
    let _ = std::fs::remove_dir_all(&dir);
    assert_eq!(got.lines().last(), Some("0"), "the printed vread reads the union differently:\n{got}\n{stdout}");
}

/// A zero-extended 16-bit field read through a pointer typed `unsigned int *`
/// keeps its width and its zero extension.  `RuleExpandLoad` used to print it as
/// `(short)a0[0x1a]`; compiled, that hands `sink` 4294941372 where the binary
/// hands it 0x9abc.  The round trip compiles `f` exactly as printed, both with
/// `structsynth off` (the raw pointer) and at the default, where the read goes
/// through a synthesized `unsigned short field_0x68` whose definition
/// `structdefs` prints above `f`.
#[test]
fn a_zero_extended_narrow_load_round_trips_through_the_printed_c() {
    let bin = repo_root()
        .join("decompiler/crates/kuna-analysis/tests/fixtures/expandload_zext_x86_64")
        .to_str()
        .unwrap()
        .to_string();
    let sp = specs();
    let arms: [(&[&str], &str); 2] = [
        (&["--option", "structsynth", "off"], "sink(*(unsigned short *)&a0[0x1a]);"),
        (&["--option", "structdefs", "on"], "sink(a0->field_0x68);"),
    ];
    for (extra, call) in arms {
        let mut args = vec!["decompile-all", bin.as_str(), "--functions", "f", "--sleighpath", sp.as_str()];
        args.extend_from_slice(extra);
        let (stdout, stderr, ok) = run_kuna(&args);
        assert!(ok, "kuna decompile-all failed: {stderr}");
        assert!(stdout.contains(call), "{stdout}");

        if process::optional_output(Command::new("cc").arg("--version")).is_none() {
            eprintln!("expandload round trip: no `cc`, spelling checked only");
            continue;
        }
        let dir = std::env::temp_dir()
            .join(format!("kuna-expandload-rt-{}-{}", std::process::id(), extra[1]));
        std::fs::create_dir_all(&dir).unwrap();
        let src = dir.join("rt.c");
        let exe = dir.join("rt");
        std::fs::write(
            &src,
            format!(
                "#include <stdio.h>\nstatic unsigned int got;\nvoid sink(unsigned int x) {{ got = x; }}\n{stdout}\n\
                 int main(void) {{\n  static unsigned int s[27];\n  s[0] = 1; s[1] = 2;\n  \
                 ((unsigned short *)s)[0x34] = 0x9abc;\n  void (*fp)() = (void (*)())f;\n  fp((void *)s);\n  \
                 printf(\"%u\\n\", got);\n  return 0;\n}}\n"
            ),
        )
        .unwrap();
        let cc = Command::new("cc")
            .args(["-std=gnu11", "-w", "-o", exe.to_str().unwrap(), src.to_str().unwrap()])
            .output()
            .expect("spawn cc");
        assert!(cc.status.success(), "the printed f did not compile:\n{}", String::from_utf8_lossy(&cc.stderr));
        let run = process::required_output(&mut Command::new(&exe));
        let got = String::from_utf8_lossy(&run.stdout).trim().to_string();
        let _ = std::fs::remove_dir_all(&dir);
        assert_eq!(got, (0x9abcu32).to_string(), "the printed f hands sink a different value:\n{stdout}");
    }
}

/// (kuna `castimplied`) A cast C's own conversion already performs is left out,
/// and a cast that changes the value stays.  The fixture widens values into libc
/// arguments, into variables of the wider type, out through `return`, and under
/// another conversion; its last functions change the value (a sign change under
/// a widening, a varargs argument) or pass a type C would convert differently.
/// The round trip compiles the printed functions, option off and on, with gcc
/// and clang, and checks every build prints what the original binary prints.
/// `castwiden` is held off: it leaves out some of the widenings pinned here,
/// which `an_implied_widening_round_trips_through_the_printed_c` covers.
#[test]
fn an_implied_cast_round_trips_through_the_printed_c() {
    const FUNCS: &str = "arg_memchr,arg_toupper,arg_strchr,asg_char,asg_uint,asg_short,asg_uchar,\
                         ret_int,ret_uint,ret_char,ret_less,to_uchar,lookup,keep_inner,keep_inner2,\
                         keep_size,keep_vararg";
    const WANT: &str = "2 2 -1 82 251 1\n1648 936 1300 50\n-7 4294967280 -100 1 0\n-89 27\n\
                        2147483645 -2 2147483647 1\n-42\n";
    const MAIN: &str = r#"
#define F(ret, f) ((ret (*)())(void (*)())f)
int main(void) {
  static const char neg[] = {-5, 7, -40, 3};
  static const unsigned int big[] = {0xfffffff0u, 5, 0x80000001u};
  static const short sh[] = {-300, 1300, -2};
  static const unsigned char uc[] = {250, 3, 128};
  char table[256];
  for (int i = 0; i < 256; i++) table[i] = (char)(i ^ 0x5a);
  printf("%ld %ld %ld %d %d %ld\n", F(long, arg_memchr)("ab\xfb" "c", -5, 4),
         F(long, arg_memchr)("abc", 'c', 3), F(long, arg_memchr)("abc", 'z', 3),
         F(int, arg_toupper)('q'), F(int, arg_toupper)(250), F(long, arg_strchr)("x\xfey", -2));
  printf("%ld %ld %d %d\n", F(long, asg_char)(neg, 4), F(long, asg_uint)(big, 3),
         F(int, asg_short)(sh, 3), F(int, asg_uchar)(uc, 3));
  printf("%ld %lu %d %d %d\n", F(long, ret_int)(-7), (unsigned long)F(unsigned int, ret_uint)(0xfffffff0u),
         F(int, ret_char)(-100), (int)F(_Bool, ret_less)(-1, 1), (int)F(_Bool, ret_less)(2, 1));
  printf("%d %d\n", F(char, lookup)(table, -3), F(char, lookup)(table, 'A'));
  unsigned int half;
  long k2 = F(long, keep_inner2)(0xfffffffeu, &half);
  printf("%ld %ld %u %ld\n", F(long, keep_inner)(0xfffffffeu), k2, half, F(long, keep_size)("abc", 'b', 3));
  F(void, keep_vararg)(-42);
  return 0;
}
"#;
    let changed: [(&str, &str); 6] = [
        ("memchr(a0,(int)a1,(unsigned long)a2);", "memchr(a0,a1,a2);"),
        ("strchr(a0,(int)a1);", "strchr(a0,a1);"),
        ("v1 = (long)a0[v2];", "v1 = a0[v2];"),
        (
            "(int)(unsigned int)(unsigned char)to_uchar((int)a1)",
            "(int)(unsigned char)to_uchar((int)a1)",
        ),
        ("long ret_int(int a0)\n{\n  return (long)a0;", "long ret_int(int a0)\n{\n  return a0;"),
        ("*a1 = a0 >> 1;\n  return (long)(int)a0;", "*a1 = a0 >> 1;\n  return (int)a0;"),
    ];
    let kept: [&str; 3] = ["(long)(int)a0", "(long)a2);", "printf(\"%ld\\n\",(long)a0);"];
    let sp = specs();
    let compilers: Vec<&str> = ["gcc", "clang"]
        .into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    for fixture in ["castimplied_gcc_O0_x86_64", "castimplied_clang_O0_x86_64"] {
        let bin = repo_root()
            .join("decompiler/crates/kuna-analysis/tests/fixtures")
            .join(fixture)
            .to_str()
            .unwrap()
            .to_string();
        for opt in ["off", "on"] {
            let args = [
                "decompile-all", bin.as_str(), "--functions", FUNCS, "--sleighpath", sp.as_str(),
                "--option", "castimplied", opt, "--option", "castwiden", "off",
            ];
            let (stdout, stderr, ok) = run_kuna(&args);
            assert!(ok, "kuna decompile-all failed: {stderr}");
            for want in kept {
                assert!(stdout.contains(want), "{fixture} option {opt} lost `{want}`:\n{stdout}");
            }
            let semantic = without_variable_sources(&stdout);
            for (off, on) in changed {
                let want = if opt == "on" { on } else { off };
                assert!(semantic.contains(&without_variable_sources(want)), "{fixture} option {opt} does not print `{want}`:\n{stdout}");
            }
            for cc in &compilers {
                let dir = std::env::temp_dir()
                    .join(format!("kuna-castimplied-rt-{}-{fixture}-{opt}-{cc}", std::process::id()));
                std::fs::create_dir_all(&dir).unwrap();
                let src = dir.join("rt.c");
                let exe = dir.join("rt");
                std::fs::write(
                    &src,
                    format!(
                        "#include <ctype.h>\n#include <stdbool.h>\n#include <stdio.h>\n#include <string.h>\n\
                         {stdout}\n{MAIN}"
                    ),
                )
                .unwrap();
                let out = Command::new(cc)
                    .args(["-std=gnu11", "-w", "-o", exe.to_str().unwrap(), src.to_str().unwrap()])
                    .output()
                    .expect("spawn the C compiler");
                assert!(
                    out.status.success(),
                    "{cc} rejected the printed C ({fixture}, option {opt}):\n{}",
                    String::from_utf8_lossy(&out.stderr)
                );
                let run = process::required_output(&mut Command::new(&exe));
                let _ = std::fs::remove_dir_all(&dir);
                assert_eq!(
                    String::from_utf8_lossy(&run.stdout),
                    WANT,
                    "{fixture} printed with option {opt} and built by {cc} computes a different value:\n{stdout}"
                );
            }
        }
    }
}

/// (kuna `fieldtype`) A synthesized field some access holds as a pointer is
/// declared as that pointer, a field whose value is really a number stays one,
/// and the printed C still computes what the binary does.  The fixture's records
/// hold a buffer added to before `memmove` gets it, a context compared with a
/// sentinel before `strlen` and `free` get it, two walking pointers and the two
/// end pointers they are compared with, a function pointer compared with zero
/// before it is called, a union member read as a double and as a pointer, an
/// index read at a table and then handed to `write` as its buffer, and a number
/// that clang -O2 passes to one `fprintf` through the register a string also
/// takes.  The printed functions are compiled with the option off and on, with
/// gcc and clang, and each build must print what the fixture binary prints.
/// `ops_run` is checked by spelling only: kuna spells a code pointer `void *`,
/// so a call through one is not C in either arm.  `cell_val` is left out of the
/// clang builds (at -O0 its double conversion prints as partial-variable
/// assignments, at -O2 its double return is lost, in either arm), and
/// `pctx_print` out of gcc -O2, whose jump table prints as a call in either arm.
#[test]
fn a_pointer_field_round_trips_through_the_printed_c() {
    const PRELUDE: &str = "#include <stdbool.h>\n#include <stdio.h>\n#include <stdlib.h>\n#include <string.h>\n\
                           #include <unistd.h>\nchar unknown_ctx[] = \"?\";\n";
    const MAIN: &str = r#"
#define F(ret, f) ((ret (*)())(void (*)())f)
static long twice(long x) { return 2 * x + 1; }
struct buf_ { char *data; long used; int left; int flags; };
struct ent_ { char *name; int size; char *ctx; int flags; };
struct range_ { long *lo; long *hi; long *end_lo; long *end_hi; int nlo; int nhi; };
struct cell_ { long tag; union { double d; char *s; long l; } u; };
struct pctx_ { unsigned long num; char *str; unsigned int ino; unsigned long blk; };
struct slot_ { int kind; long idx; char *name; };
int main(void) {
  char text[] = "hello, fieldtype world";
  struct buf_ b = { text, 10, 4, 0 };
  long s1 = F(long, buf_shift)(&b);
  printf("buf %ld %ld %d %.*s\n", s1, b.used, b.flags, (int)b.used, b.data);
  char *heap = malloc(8);
  strcpy(heap, "context");
  struct ent_ e1 = { "a", 3, heap, 1 }, e2 = { "b", 5, unknown_ctx, 0 }, e3 = { "c", 7, "static", 0 }, e4 = { 0, 1, "x", 0 };
  long r1 = F(long, ent_release)(&e1), r2 = F(long, ent_release)(&e2), r3 = F(long, ent_release)(&e3);
  long r4 = F(long, ent_release)(&e4);
  printf("ent %ld %ld %ld %ld\n", r1, r2, r3, r4);
  long lo[] = { 9, 8, 7, 6, 5 }, hi[] = { 1, 2, 3, 4, 5 };
  struct range_ r = { lo, hi + 4, lo + 5, hi, 0, 0 };
  long w = F(long, range_walk)(&r);
  printf("range %ld %d %d\n", w, r.nlo, r.nhi);
#ifndef NO_CELL
  struct cell_ c1 = { 1, { .s = "four" } }, c2 = { 2, { .l = -12 } }, c3 = { 3, { .d = 2.5 } };
  printf("cell %g %g %g\n", F(double, cell_val)(&c1), F(double, cell_val)(&c2), F(double, cell_val)(&c3));
#endif
#ifndef NO_PCTX
  struct pctx_ pc = { 42, "path", 7, 99 };
  for (const char *p = "nbisxz"; *p; p++) {
    F(void, pctx_print)(stdout, *p, 3, &pc);
    putchar('|');
  }
  pc.str = 0;
  pc.num = 0;
  F(void, pctx_print)(stdout, 's', 1, &pc);
  F(void, pctx_print)(stdout, 'n', 1, &pc);
  putchar('\n');
#endif
  long tab[] = { 3, 1, 4, 1, 5, 9, 2, 6 };
  struct slot_ sl = { 2, 5, "five" };
  printf("slot %ld\n", F(long, slot_post)(tab, &sl));
  return 0;
}
"#;
    const CELL: &str = "cell 4 -12 2.5\n";
    const PCTX: &str = " 42| 99|  7|path| 2a|z|(none) NULL(none) 0\n";
    const WANT: &str = "buf 1054690 4 1  fie\nent 10 -1 13 -2\nrange 1600 1 0\ncell 4 -12 2.5\n 42| 99|  7|path| 2a|z|(none) NULL(none) 0\nslot 10\n";
    let changed: [(&str, &str); 6] = [
        ("    long field_0x0;\n    long field_0x8;", "    void *field_0x0;\n    long field_0x8;"),
        (
            "memmove((void *)a0->field_0x0,(void *)(a0->field_0x0 + (a0->field_0x8 - a0->field_0x10)),a0->field_0x10);",
            "memmove(a0->field_0x0,(void *)((long)a0->field_0x0 + (a0->field_0x8 - a0->field_0x10)),a0->field_0x10);",
        ),
        ("v2 = strlen((char *)a0->field_0x10);", "v2 = strlen(a0->field_0x10);"),
        ("    long field_0x10;\n    long field_0x18;", "    long *field_0x10;\n    long *field_0x18;"),
        ("a0->field_0x0 = a0->field_0x0 + 8;", "a0->field_0x0 = &a0->field_0x0[1];"),
        ("v1 = (*(long (*)(unsigned long))a0->field_0x8)(a0->field_0x10);", "v1 = (*a0->field_0x8)(a0->field_0x10);"),
    ];
    let index_stays_a_number = "    int field_0x0;\n    char field_0x4[4];\n    long field_0x8;";
    let number_stays_a_number = "    long field_0x0;\n    char *field_0x8;\n    unsigned int field_0x10;";
    let sp = specs();
    let compilers: Vec<&str> = ["gcc", "clang"]
        .into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    for fixture in
        ["fieldtype_gcc_O0_x86_64", "fieldtype_clang_O0_x86_64", "fieldtype_gcc_O2_x86_64", "fieldtype_clang_O2_x86_64"]
    {
        let bin = repo_root()
            .join("decompiler/crates/kuna-analysis/tests/fixtures")
            .join(fixture)
            .to_str()
            .unwrap()
            .to_string();
        let clang = fixture.contains("clang");
        let gcc_o2 = fixture == "fieldtype_gcc_O2_x86_64";
        let funcs = if clang {
            "buf_shift,ent_release,range_walk,pctx_print,slot_post"
        } else if gcc_o2 {
            "buf_shift,ent_release,range_walk,cell_val,slot_post"
        } else {
            "buf_shift,ent_release,range_walk,cell_val,pctx_print,slot_post"
        };
        for opt in ["off", "on"] {
            let args = [
                "decompile-all", bin.as_str(), "--functions", funcs, "--sleighpath", sp.as_str(),
                "--option", "structdefs", "on", "--option", "fieldtype", opt,
            ];
            let (stdout, stderr, ok) = run_kuna(&args);
            assert!(ok, "kuna decompile-all failed: {stderr}");
            if fixture == "fieldtype_gcc_O0_x86_64" {
                let args = [
                    "decompile-all", bin.as_str(), "--functions", "buf_shift,ent_release,ops_run,range_walk",
                    "--sleighpath", sp.as_str(), "--option", "structdefs", "on", "--option", "fieldtype", opt,
                ];
                let (all, stderr, ok) = run_kuna(&args);
                assert!(ok, "kuna decompile-all failed: {stderr}");
                for (off, on) in changed {
                    let want = if opt == "on" { on } else { off };
                    assert!(all.contains(want), "{fixture} option {opt} does not print `{want}`:\n{all}");
                }
            }
            if !clang {
                assert!(
                    stdout.contains("    char field_0x8[8];"),
                    "{fixture} option {opt}: a union member read as a double and a pointer must stay raw bytes:\n{stdout}"
                );
            }
            if fixture.contains("_O0_") {
                assert!(
                    stdout.contains(index_stays_a_number),
                    "{fixture} option {opt}: an index read at a table and handed to write must stay a number:\n{stdout}"
                );
            }
            if fixture == "fieldtype_clang_O2_x86_64" {
                assert!(
                    stdout.contains(number_stays_a_number),
                    "{fixture} option {opt}: a number merged with a string on its way to fprintf must stay a number:\n{stdout}"
                );
            }
            let printed: String =
                stdout.lines().filter(|l| !l.ends_with("/* opaque */")).map(|l| format!("{l}\n")).collect();
            for cc in &compilers {
                let dir = std::env::temp_dir()
                    .join(format!("kuna-fieldtype-rt-{}-{fixture}-{opt}-{cc}", std::process::id()));
                std::fs::create_dir_all(&dir).unwrap();
                let src = dir.join("rt.c");
                let exe = dir.join("rt");
                std::fs::write(&src, format!("{PRELUDE}{printed}\n{MAIN}")).unwrap();
                let mut cmd = Command::new(cc);
                cmd.args(["-std=gnu11", "-w", "-Werror=int-conversion", "-o", exe.to_str().unwrap()]);
                if clang {
                    cmd.arg("-DNO_CELL");
                }
                if gcc_o2 {
                    cmd.arg("-DNO_PCTX");
                }
                let out = cmd.arg(src.to_str().unwrap()).output().expect("spawn the C compiler");
                assert!(
                    out.status.success(),
                    "{cc} rejected the printed C ({fixture}, option {opt}):\n{}",
                    String::from_utf8_lossy(&out.stderr)
                );
                let run = process::required_output(&mut Command::new(&exe));
                let _ = std::fs::remove_dir_all(&dir);
                let mut want = WANT.to_string();
                if clang {
                    want = want.replace(CELL, "");
                }
                if gcc_o2 {
                    want = want.replace(PCTX, "");
                }
                assert_eq!(
                    String::from_utf8_lossy(&run.stdout),
                    want,
                    "{fixture} printed with option {opt} and built by {cc} computes a different value:\n{stdout}"
                );
            }
        }
    }
}

/// (kuna `castternary`) A widening on an arm of `c ? a : b` that the
/// conditional performs itself is left out, and a cast whose removal would give
/// the conditional another type stays.  The fixture holds a textbook base64
/// decoder (`in[i] != '=' ? table[in[i]] : 0`, four times per quantum) and arms
/// of char, unsigned char, short and int against int, unsigned, long and
/// negative constants and a second cast, fed bytes 0x80..0xff and negative
/// values.  The printed functions are compiled with the option off and on, with
/// gcc and clang, and each build must print what the fixture binary prints.
/// gcc -O0 keeps the result of each conditional in a register, so its build
/// prints conditionals; clang -O0 spills it, and most of its diamonds print as
/// if/else, where `castimplied` already leaves the widening out.
/// `castwiden` is held off: it leaves out some of the widenings pinned here,
/// which `an_implied_widening_round_trips_through_the_printed_c` covers.
#[test]
fn a_conditional_arm_cast_round_trips_through_the_printed_c() {
    const FUNCS: &str = "b64_decode,arm_char,arm_uchar,arm_short,arm_char_uint,arm_long,arm_char_long,\
                         arm_ulong,arm_uint_long,arm_uint_max,arm_both,arm_wide_literal,arm_all_ones,\
                         keep_narrow,keep_less,keep_float";
    const WANT: &str = "13 72 101 108 108 111 32 119 111 114 108 100 63 251\n\
                        -1 -128 127 -1 0 255 128 -1\n\
                        -300 7 4294967295 4294967168 0\n\
                        -1 -7 -9 -128 -1 -9\n\
                        4294967295 5 4294967295 3000000000 -9 4294967295 18446744073709551615\n\
                        -5 4294967291 -1 -5 -7 3000000000\n\
                        4294967168 4294967295 1 4294967295 -3.5\n";
    const MAIN: &str = r#"
#define F(ret, f) ((ret (*)())(void (*)())f)
int main(void) {
  signed char t[256];
  static const char alphabet[] = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
  memset(t, -1, 256);
  for (int i = 0; i < 64; i++) t[(unsigned char)alphabet[i]] = (signed char)i;
  static const char txt[] = "SGVsbG8gd29ybGQ/+w==";
  unsigned char out[16];
  unsigned long n = F(unsigned long, b64_decode)(t, txt, strlen(txt), out);
  static const char bytes[] = {0, -1, -128, 0x7f, (char)0xff, 'H'};
  static const unsigned char ub[] = {0, 0xff, 0x80, 0xde};
  static const short sh[] = {0, -300, 7};
  static const int in[] = {0, -1, -7};
  static const unsigned int ui[] = {0, 0xffffffffu, 3000000000u};
  static const long lg[] = {-9, 5, -1};
  static const unsigned long ul[] = {5, 0xfffffffffffffff0UL, 1};
  printf("%lu", n);
  for (unsigned long i = 0; i < n; i++) printf(" %u", out[i]);
  printf("\n%d %d %d %d %d %d %d %d\n", F(int, arm_char)(bytes, 1), F(int, arm_char)(bytes, 2),
         F(int, arm_char)(bytes, 3), F(int, arm_char)(bytes, 4), F(int, arm_char)(bytes, 0),
         F(int, arm_uchar)(ub, 1), F(int, arm_uchar)(ub, 2), F(int, arm_uchar)(ub, 0));
  printf("%d %d %u %u %u\n", F(int, arm_short)(sh, 1), F(int, arm_short)(sh, 0),
         F(unsigned int, arm_char_uint)(bytes, 1), F(unsigned int, arm_char_uint)(bytes, 2),
         F(unsigned int, arm_char_uint)(bytes, 0));
  printf("%ld %ld %ld %ld %ld %ld\n", F(long, arm_long)(in, lg, 1), F(long, arm_long)(in, lg, 2),
         F(long, arm_long)(in, lg, 0), F(long, arm_char_long)(bytes, lg, 2), F(long, arm_char_long)(bytes, lg, 4),
         F(long, arm_char_long)(bytes, lg, 0));
  printf("%lu %lu %ld %ld %ld %lu %lu\n", F(unsigned long, arm_ulong)(ui, ul, 1),
         F(unsigned long, arm_ulong)(ui, ul, 0), F(long, arm_uint_long)(ui, lg, 1), F(long, arm_uint_long)(ui, lg, 2),
         F(long, arm_uint_long)(ui, lg, 0), F(unsigned long, arm_uint_max)(ui, 1), F(unsigned long, arm_uint_max)(ui, 0));
  printf("%ld %ld %ld %ld %ld %ld\n", F(long, arm_both)(-5, 7u, 1), F(long, arm_both)(-5, 0xfffffffbu, 0),
         F(long, keep_narrow)(in, 1), F(long, keep_narrow)(in, 0), F(long, arm_wide_literal)(in, 2),
         F(long, arm_wide_literal)(in, 0));
  printf("%u %u %u %u %.1f\n", F(unsigned int, arm_all_ones)(bytes, 2), F(unsigned int, arm_all_ones)(bytes, 0),
         F(unsigned int, keep_less)(1, 2, 1), F(unsigned int, keep_less)(1, 2, 0),
         ((double (*)(float, double, int))(void (*)())keep_float)(-3.5f, 2.0, 1));
  return 0;
}
"#;
    let both = [("v1 = (a2) ? (unsigned long)a0 : (unsigned long)a1;", "v1 = (a2) ? (unsigned long)a0 : a1;")];
    let gcc_only: [(&str, &str); 8] = [
        (
            "v2 = (*(char *)(v7 + a1) != '=') ? (int)*(char *)(a0 + (unsigned long)*(unsigned char *)(v7 + a1)) : 0;",
            "v2 = (*(char *)(v7 + a1) != '=') ? *(char *)(a0 + (unsigned long)*(unsigned char *)(v7 + a1)) : 0;",
        ),
        ("v1 = (a1) ? (int)*(char *)(a0 + a1) : 0;", "v1 = (a1) ? *(char *)(a0 + a1) : 0;"),
        (
            "v1 = (a1) ? (unsigned int)*(unsigned char *)(a0 + a1) : 0xffffffff;",
            "v1 = (a1) ? *(unsigned char *)(a0 + a1) : 0xffffffff;",
        ),
        ("v1 = (a1) ? (int)*(short *)(a0 + (long)a1 * 2) : 7;", "v1 = (a1) ? *(short *)(a0 + (long)a1 * 2) : 7;"),
        (
            "v1 = (a1) ? (unsigned long)*(unsigned int *)(a0 + (long)a1 * 4) : 0xffffffffffffffff;",
            "v1 = (a1) ? *(unsigned int *)(a0 + (long)a1 * 4) : 0xffffffffffffffff;",
        ),
        (
            "v1 = (a1) ? (long)*(int *)(a0 + (long)a1 * 4) : 3000000000;",
            "v1 = (a1) ? *(int *)(a0 + (long)a1 * 4) : 3000000000;",
        ),
        ("v1 = (a1) ? (int)*(char *)(a0 + a1) : -1;", "v1 = (a1) ? *(char *)(a0 + a1) : -1;"),
        ("v5 = (*(char *)(a1 + v7 + 3) != '=') ? (int)*(char", "v5 = (*(char *)(a1 + v7 + 3) != '=') ? *(char"),
    ];
    let gcc_kept = ["v1 = (a1) ? (long)*(int *)(a0 + (long)a1 * 4) : -5;", "v1 = (a2) ? (unsigned int)(a0 < a1) : 0xffffffff;"];
    let sp = specs();
    let compilers: Vec<&str> = ["gcc", "clang"]
        .into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    for fixture in ["castternary_gcc_O0_x86_64", "castternary_clang_O0_x86_64"] {
        let bin = repo_root()
            .join("decompiler/crates/kuna-analysis/tests/fixtures")
            .join(fixture)
            .to_str()
            .unwrap()
            .to_string();
        let gcc = fixture.contains("gcc");
        // The spellings are pinned with `elemptr` off, which leaves the table
        // bases integers; with it on (the default) they are subscripts of
        // declared pointers, and the printed C must still compute the same values.
        for (elem, opt) in [("off", "off"), ("off", "on"), ("on", "off"), ("on", "on")] {
            let args = [
                "decompile-all", bin.as_str(), "--functions", FUNCS, "--sleighpath", sp.as_str(),
                "--option", "castternary", opt, "--option", "castwiden", "off", "--option", "elemptr", elem,
            ];
            let (stdout, stderr, ok) = run_kuna(&args);
            assert!(ok, "kuna decompile-all failed: {stderr}");
            let changed: Vec<(&str, &str)> =
                both.iter().chain(if gcc { gcc_only.iter() } else { [].iter() }).copied().collect();
            if elem == "off" {
                for (off, on) in changed {
                    let want = if opt == "on" { on } else { off };
                    assert!(stdout.contains(want), "{fixture} option {opt} does not print `{want}`:\n{stdout}");
                }
                if gcc {
                    for want in gcc_kept {
                        assert!(stdout.contains(want), "{fixture} option {opt} lost `{want}`:\n{stdout}");
                    }
                }
            }
            for cc in &compilers {
                for level in ["-O0", "-O2"] {
                    let dir = std::env::temp_dir().join(format!(
                        "kuna-castternary-rt-{}-{fixture}-{elem}-{opt}-{cc}{level}",
                        std::process::id()
                    ));
                    std::fs::create_dir_all(&dir).unwrap();
                    let src = dir.join("rt.c");
                    let exe = dir.join("rt");
                    std::fs::write(
                        &src,
                        format!("#include <stdbool.h>\n#include <stdio.h>\n#include <string.h>\n{stdout}\n{MAIN}"),
                    )
                    .unwrap();
                    let out = Command::new(cc)
                        .args(["-std=gnu11", level, "-w", "-Wno-error=int-conversion", "-o", exe.to_str().unwrap(), src.to_str().unwrap()])
                        .output()
                        .expect("spawn the C compiler");
                    assert!(
                        out.status.success(),
                        "{cc} rejected the printed C ({fixture}, option {opt}):\n{}",
                        String::from_utf8_lossy(&out.stderr)
                    );
                    let run = process::required_output(&mut Command::new(&exe));
                    let _ = std::fs::remove_dir_all(&dir);
                    assert_eq!(
                        String::from_utf8_lossy(&run.stdout),
                        WANT,
                        "{fixture} printed with option {opt} and built by {cc} {level} computes a different value:\n{stdout}"
                    );
                }
            }
        }
    }
}

/// (kuna `castwiden`) A 64-bit widening C performs by itself keeps no cast: an
/// operand of `+ - * / % & | ^` beside a 64-bit operand of the cast's type, beside
/// a literal the `literal` value prints with its `L`/`UL` suffix, and a widening
/// into an assignment, a store or a prototyped argument of the cast's type or
/// width.  A shift, a comparison, an unsigned widening beside a signed operand, a
/// negated unsigned literal and one widened value read by both operands of an op
/// (`(long)i * (long)i`) keep their cast, and a 32-bit sum or product beside a long
/// of the same operator keeps its parentheses.  Beside a chain of its own operator,
/// which prints without parentheses and which C regroups (`(long)a + ((long)b + x)`
/// prints `(long)a + b + x`), the left operand keeps its cast unless the chain's
/// first leaf is 64-bit.  The functions of
/// `castwiden_x86_64.c`, built with gcc and clang at -O0 and -O2, are printed with
/// the option off, on and literal, compiled with gcc and clang at -O0 and -O2
/// (`-fwrapv`, so the 32-bit arithmetic kuna prints as `int` wraps as the machine's
/// does), and every build must print what the fixture binary prints for negative
/// values, 0x80000000..0xffffffff, sums past 32 bits and squares past 2^32.
#[test]
fn an_implied_widening_round_trips_through_the_printed_c() {
    const FUNCS: &str = "add_load,minus_load,add_uload,mix_uint,add_char,div_load,udiv_load,mul_two,lit_mul,lit_add,\
                         lit_umul,lit_index,lit_mask,lit_neg,ret_ulong,store_long,store_ulong,store_uchar,assign_ulong,\
                         assign_loop,sink,assign_call,assign_size,field_add,field_umix,find_len,keep_shift,keep_less,\
                         keep_mixed,keep_neglit,sq,usq,sq_diff,dist,par_add,par_mul,chain_add,chain_mul,chain_add3,\
                         chain_xor";
    const WANT: &str = "2147483647 2147483647 5 0 0 7 1 0 -3\n\
0 1 0\n\
0 0\n\
2147483646 2147483648 4 1 3 -5 0 -1664 -4\n\
18446744073709551615 1 18446744073709551613\n\
5\n\
-1 18446744073709551615\n\
4294967294 0 2147483652 4611686014132420609 -6442450941 25769803771 2147483648 3573412788608 2147483644\n\
2147483647 1 18446744071562067965\n\
0\n\
2147483647 2147483647\n\
-1 4294967295 18446744071562067973 4611686018427387904 6442450944 -25769803769 -2147483647 -3573412790272 -2147483651\n\
18446744071562067968 0 2147483648\n\
2147483647\n\
-2147483648 18446744071562067968\n\
2147483640 2147483654 18446744073709551614 49 21 -77 -6 -11648 -10\n\
18446744073709551609 0 18446744073709551607\n\
1\n\
-7 18446744073709551609\n\
2147483650 2147483644 8 9 -9 43 4 4992 0\n\
3 1 18446744073709551609\n\
0\n\
3 3\n\
18446744073709551600 0 4294967296 0\n\
18446744071562067983 34359738360 8589934591 18446742974197923840 4294967296\n\
18446744073709551600 17179869184 6442450944 0 8589934591\n\
18446744073709551603 24 4294967299 3298534883328 6148914691236517200\n\
-133 122 255 250 2147483775\n\
3 -1 -1\n\
38654705538 4294967284\n\
6148914691236517202 715827882 6148914690520689323\n\
4294967292 2 10\n\
-6 18446744069414584334\n\
4294967294 18446744071562067968\n\
-2147483647 0\n\
0 0 0 0 2147483647 2147483648 0 0\n\
-5 0 2147483647 18446744073709551603\n\
1 18446744065119617025 0 2 2147483645 2147483647 2147483647 -6442450941\n\
-7 -9 2147483644 18446744069414584332\n\
4611686014132420609 4611686014132420609 1152921504606846976 5764607516591783938 2147483645 -1 2147483647 4611686009837453315\n\
4294967289 19327352823 8589934588 18446744071562067980\n\
4611686018427387904 4611686018427387904 1152921504606846976 5764607523034234880 2147483647 0 0 -4611686016279904256\n\
-4294967301 -19327352832 -4294967297 18446744071562067955\n\
4294967296 4294967296 1073741824 5368709120 2147614719 2147549184 0 422212464869376\n\
131067 589824 2147680255 18446744073709486067\n\
4295098369 18446181119461294081 1073741824 5368905730 2147352573 2147418111 281477124063231 -422218907320317\n\
-131079 -589833 2147287036 18446744069414649868\n\
2147488281 2147488281 536895241 2684337181 2147576329 2147529989 -4611676066988167705 298549619056881\n\
92677 417069 2147622670 18446744073709505270\n";
    const MAIN: &str = r#"
#define F(ret, f) ((ret (*)())(void (*)())f)
struct rec {
  long a;
  unsigned long b;
  int c;
  unsigned int d;
};
int main(void) {
  long lp[4] = {-5, 0x7fffffff, -1, 3};
  unsigned long up[4] = {0xfffffffffffffff0UL, 0x80000000UL, 5, 0};
  static const signed char sc[] = {1, -128, 127};
  static const unsigned char uc[] = {0xff, 0x80};
  int ints[] = {0, -1, 0x7fffffff, (int)0x80000000, -7, 3};
  unsigned int uints[] = {0, 0xffffffffu, 0x80000000u, 3};
  for (int k = 0; k < 6; k++) {
    int i = ints[k];
    printf("%ld %ld %lu %ld %ld %ld %ld %ld %ld\n", F(long, add_load)(lp, i), F(long, minus_load)(lp, i),
           F(unsigned long, add_uload)(up, i), F(long, mul_two)(i, i), F(long, mul_two)(i, -3), F(long, lit_mul)(i),
           F(long, lit_add)(i), F(long, lit_index)(i, i), F(long, lit_neg)(i));
    printf("%lu %d %lu\n", F(unsigned long, ret_ulong)(i), (int)F(bool, keep_less)(lp, i),
           F(unsigned long, assign_ulong)(i, (int)(0u - (unsigned int)i)));
    if (i != 0)
      printf("%ld\n", F(long, div_load)(lp, i));
    long sl[4] = {0};
    unsigned long su[4] = {0};
    F(void, store_long)(sl, i);
    F(void, store_ulong)(su, i);
    printf("%ld %lu\n", sl[1], su[2]);
  }
  for (int k = 0; k < 4; k++) {
    unsigned int u = uints[k];
    printf("%lu %lu %lu %lu", F(unsigned long, mix_uint)(up, u), F(unsigned long, lit_umul)(u),
           F(unsigned long, lit_mask)(u), F(unsigned long, keep_shift)(u));
    if (u != 0)
      printf(" %lu", F(unsigned long, udiv_load)(up, u));
    printf("\n");
  }
  unsigned long su[4] = {0};
  F(void, store_uchar)(su, uc);
  printf("%ld %ld %lu %lu %lu\n", F(long, add_char)(lp, sc), F(long, add_char)(lp, sc + 1), su[3],
         F(unsigned long, keep_mixed)(lp, uc), F(unsigned long, keep_mixed)(lp + 1, uc + 1));
  printf("%ld %ld %ld\n", F(long, find_len)("abcxdef", 7), F(long, find_len)("abcxdef", 3),
         F(long, find_len)("abcdef", 6));
  printf("%lu %lu\n", F(unsigned long, assign_loop)(ints, 6), F(unsigned long, assign_loop)(ints + 1, 3));
  printf("%lu %lu %lu\n", F(unsigned long, assign_size)(-7), F(unsigned long, assign_size)(0x7fffffff),
         F(unsigned long, assign_size)((int)0x80000000));
  printf("%lu %lu %lu\n", F(unsigned long, assign_call)(-1, 0xffffffffu), F(unsigned long, assign_call)((int)0x80000000, 0x80000000u),
         F(unsigned long, assign_call)(5, 7));
  struct rec rs[3] = {{-5, 7, -1, 0xffffffffu}, {0x7fffffff, 0xfffffffffffffff0UL, 0x7fffffff, 1},
                      {1, 2, (int)0x80000000, 0x80000000u}};
  for (int k = 0; k < 3; k++)
    printf("%ld %lu\n", F(long, field_add)(&rs[k]), F(unsigned long, field_umix)(&rs[k]));
  int big[] = {0, -1, 0x7fffffff, (int)0x80000000, 0x10000, -0x10001, 46341};
  for (int k = 0; k < 7; k++) {
    int i = big[k];
    printf("%ld %lu %ld %ld %ld %ld %ld %ld\n", F(long, sq)(i), F(unsigned long, usq)((unsigned int)i),
           F(long, sq_diff)(i, i >> 1), F(long, dist)(0, 0, i, i >> 1), F(long, par_add)(lp, i, i),
           F(long, par_add)(lp, i, 1), F(long, par_mul)(lp, i, i), F(long, par_mul)(lp, i, 3));
    printf("%ld %ld %ld %lu\n", F(long, chain_add)(lp, i, i), F(long, chain_mul)(lp + 3, i, 3),
           F(long, chain_add3)(lp, i, i, i), F(unsigned long, chain_xor)(up, (unsigned int)i, 3u));
  }
  return 0;
}
"#;
    // (function, text with the option off, with `on`, with `literal`) per fixture.
    type Pins = &'static [(&'static str, &'static str, &'static str, &'static str)];
    let gcc_o0: Pins = &[
        ("add_load", "(long)a1 + ((long *)a0)[1]", "return a1 + ((long *)a0)[1];", "return a1 + ((long *)a0)[1];"),
        ("mul_two", "(long)a1 * (long)a0", "return (long)a1 * a0;", "return (long)a1 * a0;"),
        ("lit_mul", "return (long)a0 * 0xc + 7;", "return (long)a0 * 0xc + 7;", "return a0 * 0xcL + 7;"),
        ("lit_mask", "(unsigned long)a0 | 0x100000000;", "(unsigned long)a0 | 0x100000000;", "return a0 | 0x100000000UL;"),
        ("store_ulong", "((long *)a0)[2] = (long)a1;", "((long *)a0)[2] = a1;", "((long *)a0)[2] = a1;"),
        ("assign_size", "v1 = (unsigned long)a0;", "v1 = a0;", "v1 = a0;"),
        ("find_len", "memchr(a0,0x78,(long)a1)", "memchr(a0,0x78,a1)", "memchr(a0,0x78,a1)"),
        ("keep_mixed", "*a0 + (unsigned long)*a1", "*a0 + (unsigned long)*a1", "*a0 + (unsigned long)*a1"),
        ("sq", "return (long)a0 * (long)a0;", "return (long)a0 * a0;", "return (long)a0 * a0;"),
        ("sq_diff", "(long)(a0 - a1) * (long)(a0 - a1)", "(long)(a0 - a1) * (long)(a0 - a1)", "(long)(a0 - a1) * (long)(a0 - a1)"),
        ("par_add", "return (long)(a2 + a1) + ((long *)a0)[1];", "return (a2 + a1) + ((long *)a0)[1];", "return (a2 + a1) + ((long *)a0)[1];"),
    ];
    let clang_o0: Pins = &[
        ("add_load", "((long *)a0)[1] + (long)a1", "return ((long *)a0)[1] + a1;", "return ((long *)a0)[1] + a1;"),
        ("mul_two", "(long)a0 * (long)a1", "return (long)a0 * a1;", "return (long)a0 * a1;"),
        ("lit_index", "((long)a0 * 0xc + (long)a1) * 0x80", "((long)a0 * 0xc + a1) * 0x80", "(a0 * 0xcL + a1) * 0x80"),
        ("assign_size", "v1 = (unsigned long)a0;", "v1 = a0;", "v1 = a0;"),
        ("keep_mixed", "(unsigned long)*a1 + *a0", "(unsigned long)*a1 + *a0", "(unsigned long)*a1 + *a0"),
        ("par_add", "((long *)a0)[1] + (long)(a1 + a2);", "((long *)a0)[1] + (a1 + a2);", "((long *)a0)[1] + (a1 + a2);"),
        ("par_mul", "((long *)a0)[1] * (long)(a1 * a2);", "((long *)a0)[1] * (a1 * a2);", "((long *)a0)[1] * (a1 * a2);"),
        ("chain_add", "return (long)a1 + (long)a2 + *a0;", "return (long)a1 + a2 + *a0;", "return (long)a1 + a2 + *a0;"),
        ("chain_add3", "(long)a1 + (long)a2 + (long)a3 + ", "return a1 + (long)a2 + a3 + ", "return a1 + (long)a2 + a3 + "),
    ];
    let gcc_o2: Pins = &[
        ("add_load", "(long)a1 + ((long *)a0)[1]", "return a1 + ((long *)a0)[1];", "return a1 + ((long *)a0)[1];"),
        ("lit_add", "return (long)a0 + 1;", "return (long)a0 + 1;", "return a0 + 1L;"),
        ("store_long", "((long *)a0)[1] = (long)a1;", "((long *)a0)[1] = a1;", "((long *)a0)[1] = a1;"),
        ("assign_size", "v1 = (unsigned long)a0;", "v1 = a0;", "v1 = a0;"),
        ("sq", "return (long)a0 * (long)a0;", "return (long)a0 * (long)a0;", "return (long)a0 * (long)a0;"),
        ("usq", "(unsigned long)a0 * (unsigned long)a0;", "(unsigned long)a0 * (unsigned long)a0;", "(unsigned long)a0 * (unsigned long)a0;"),
    ];
    let clang_o2: Pins = &[
        ("sq", "return (long)a0 * (long)a0;", "return (long)a0 * (long)a0;", "return (long)a0 * (long)a0;"),
        ("usq", "(unsigned long)a0 * (unsigned long)a0;", "(unsigned long)a0 * (unsigned long)a0;", "(unsigned long)a0 * (unsigned long)a0;"),
        ("par_mul", "return (long)(a1 * a2) * ((long *)a0)[1];", "return (a1 * a2) * ((long *)a0)[1];", "return (a1 * a2) * ((long *)a0)[1];"),
        ("chain_mul", "return (long)a2 * (long)a1 * *a0;", "return (long)a2 * a1 * *a0;", "return (long)a2 * a1 * *a0;"),
    ];
    // Kept by every value: a comparison operand, and a widening beside a negated
    // unsigned literal (`keep_neglit`, whose constant kuna prints with the option
    // off too in a form C reads as +2^31, so it is not called below).
    let kept = [("keep_less", "*a0 < (long)a1"), ("keep_neglit", "return (long)a0 + ")];
    let sp = specs();
    let compilers: Vec<&str> = ["gcc", "clang"]
        .into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    for (fixture, pins) in [
        ("castwiden_gcc_O0_x86_64", gcc_o0),
        ("castwiden_clang_O0_x86_64", clang_o0),
        ("castwiden_gcc_O2_x86_64", gcc_o2),
        ("castwiden_clang_O2_x86_64", clang_o2),
    ] {
        let bin = repo_root()
            .join("decompiler/crates/kuna-analysis/tests/fixtures")
            .join(fixture)
            .to_str()
            .unwrap()
            .to_string();
        for (arm, opt) in ["off", "on", "literal"].into_iter().enumerate() {
            let args = [
                "decompile-all", bin.as_str(), "--functions", FUNCS, "--sleighpath", sp.as_str(),
                "--option", "castwiden", opt, "--option", "structdefs", "on",
            ];
            let (stdout, stderr, ok) = run_kuna(&args);
            assert!(ok, "kuna decompile-all failed: {stderr}");
            for &(func, off, on, literal) in pins {
                let want = [off, on, literal][arm];
                let text = castsign_function(&stdout, func);
                assert!(text.contains(want), "{fixture} option {opt}: {func} does not print `{want}`:\n{text}");
            }
            for (func, want) in kept {
                let text = castsign_function(&stdout, func);
                assert!(text.contains(want), "{fixture} option {opt}: {func} lost `{want}`:\n{text}");
            }
            for cc in &compilers {
                for level in ["-O0", "-O2"] {
                    let dir = std::env::temp_dir().join(format!(
                        "kuna-castwiden-rt-{}-{fixture}-{opt}-{cc}{level}",
                        std::process::id()
                    ));
                    std::fs::create_dir_all(&dir).unwrap();
                    let src = dir.join("rt.c");
                    let exe = dir.join("rt");
                    std::fs::write(
                        &src,
                        format!("#include <stdbool.h>\n#include <stdio.h>\n#include <string.h>\n{stdout}\n{MAIN}"),
                    )
                    .unwrap();
                    let out = Command::new(cc)
                        .args([
                            "-std=gnu11", level, "-w", "-fno-strict-aliasing", "-fwrapv", "-Wno-error=int-conversion",
                            "-o", exe.to_str().unwrap(), src.to_str().unwrap(),
                        ])
                        .output()
                        .expect("spawn the C compiler");
                    assert!(
                        out.status.success(),
                        "{cc} rejected the printed C ({fixture}, option {opt}):\n{}",
                        String::from_utf8_lossy(&out.stderr)
                    );
                    let run = process::required_output(&mut Command::new(&exe));
                    let _ = std::fs::remove_dir_all(&dir);
                    assert_eq!(
                        String::from_utf8_lossy(&run.stdout),
                        WANT,
                        "{fixture} printed with option {opt} and built by {cc} {level} computes a different value:\n{stdout}"
                    );
                }
            }
        }
    }
}

/// The text of one function in a `decompile-all` listing, from its `// Function:`
/// header to the next.
fn castsign_function<'a>(listing: &'a str, name: &str) -> &'a str {
    let head = format!("// Function: {name} @");
    let start = listing.find(&head).unwrap_or_else(|| panic!("{name} not printed:\n{listing}"));
    let rest = &listing[start + head.len()..];
    &listing[start..start + head.len() + rest.find("// Function: ").unwrap_or(rest.len())]
}

/// Ignore variable-storage diagnostics when checking types and executable C.
fn without_variable_sources(output: &str) -> String {
    use regex::Regex;
    static PATTERNS: std::sync::OnceLock<(Regex, Regex, Regex, Regex)> = std::sync::OnceLock::new();
    let (home, declaration, prototype, parameter) = PATTERNS.get_or_init(|| {
        let register = r"(?i:acc|(?:r|e)?(?:ax|bx|cx|dx|sp|bp|di|si)|[abcd][lh]|(?:[sb]p|[sd]i)l|r[0-9]+[bwd]?|(?:[xyz]mm|st)[0-9]+(?:_[a-z]+)?|[cdefgs]s|[fg]s_offset|[csozapd]f)";
        (
            Regex::new(&format!(r"^(?:tmp|{register}(?::{register})*|stack [+-] 0x[0-9a-fA-F]+|[A-Za-z_][A-Za-z0-9_]* 0x[0-9a-fA-F]+)$")).unwrap(),
            Regex::new(r"^(?P<decl>  (?:[A-Za-z_][A-Za-z0-9_]*[ \t]+|\*[ \t]*)+[A-Za-z_][A-Za-z0-9_]*(?: \[[0-9]+\])?;) // (?P<source>.+)$").unwrap(),
            Regex::new(r"^(?:[A-Za-z_][A-Za-z0-9_]*[ \t]+|\*[ \t]*)+[A-Za-z_][A-Za-z0-9_]*\((?P<params>[^()]*)\)$").unwrap(),
            Regex::new(r"^(?:[A-Za-z_][A-Za-z0-9_]*[ \t]+|\*[ \t]*)+(?P<name>[A-Za-z_][A-Za-z0-9_]*)(?: \[[0-9]+\])?$").unwrap(),
        )
    });
    let storage = |source: &str| source.split(" | ").all(|part| home.is_match(part));
    let mut parameters = std::collections::BTreeSet::new();
    output.split_inclusive('\n').filter_map(|line| {
        let content = line.strip_suffix('\n').unwrap_or(line);
        let signature = content.split(" // ").next().unwrap();
        if let Some(captures) = prototype.captures(signature) {
            parameters = captures["params"].split(',').filter_map(|part| {
                parameter.captures(part.trim()).map(|p| p["name"].to_string())
            }).collect();
        } else if content == "}" {
            parameters.clear();
        }
        if let Some((name, source)) = content.strip_prefix("  // ").and_then(|s| s.split_once(": ")) {
            if parameters.contains(name) && storage(source) {
                return None;
            }
        }
        if let Some(captures) = declaration.captures(content) {
            let decl = &captures["decl"];
            let statement = matches!(decl.trim_start().split_whitespace().next(), Some("return" | "goto" | "throw"));
            if !statement && storage(&captures["source"]) {
                return Some(format!("{decl}{}", if line.ends_with('\n') { "\n" } else { "" }));
            }
        }
        Some(line.to_string())
    }).collect()
}

#[test]
fn variable_source_normalization_preserves_statement_and_note_boundaries() {
    let text = "uint1 f(int4 x) // warn: bad data\n{\n  uint1 v1; // ram 0x52 | acc\n  // x: edi | stack - 0xc\n  // note: rax\n  int4 v2; // warning\n  v1 = 1; // acc\n  return v1; // acc\n}\nuint1 g(int4 y)\n{\n  // x: edi\n  // y: esi\n  return y;\n}\n";
    let expected = "uint1 f(int4 x) // warn: bad data\n{\n  uint1 v1;\n  // note: rax\n  int4 v2; // warning\n  v1 = 1; // acc\n  return v1; // acc\n}\nuint1 g(int4 y)\n{\n  // x: edi\n  return y;\n}\n";
    assert_eq!(without_variable_sources(text), expected);
}

/// (kuna `castsign`) A variable the program only compares signed is declared
/// signed, and the `(long)v` it cost at each comparison goes.  A variable that
/// `+ - *` reads keeps its unsigned declaration: that arithmetic wraps, and the
/// signed form would overflow at the edge of the range, which gcc and clang fold
/// on.  `castsign_wrap_x86_64.c` passes its functions 2^63 - 1, 2^63, 2^63 + 1
/// and the 32-bit edges; `castsign_x86_64.c` keeps lengths and indexes taken from
/// `strlen` and from unsigned tables.  Each fixture is decompiled with the option
/// off and on, the printed functions are compiled with gcc and clang at -O0 and
/// -O2, and every build must print what the fixture binary prints.  The -O1
/// build of the wrap source is checked on `sign_of` and `peek` only: its
/// arithmetic shapes are register locals, which `signedness` decides and this
/// option leaves alone.  `castsign_eq_x86_64.c` also compares the value for
/// equality with `3000000000` or `10000000000000000000`, decimal literals whose C
/// type is wider than the declaration, so those declarations stay unsigned.
/// `castwiden` is held off: it leaves out some of the widenings pinned here,
/// which `an_implied_widening_round_trips_through_the_printed_c` covers.
#[test]
fn a_signed_only_variable_round_trips_through_the_printed_c() {
    const WRAP_FUNCS: &str = "dec_neg,cnt_wrap,spin,count_down,dec_neg32,sign_of,sign_of32,peek";
    const WRAP_WANT: &str = "dec_neg      0 1 1 5 5 0 0\ncnt_wrap     5 5 5 0 5 5 5\nspin         3 0 5\n\
count_down   0 1 2 5 5 0 0\ndec_neg32    0 13 15 15 0\nsign_of      1 2 0 1\nsign_of32    5 6 4 5\n\
peek         99 -1 -2 100\n";
    const WRAP_MAIN: &str = r#"
#define F(ret, f) ((ret (*)())(void (*)())f)
static const char *const W64[] = {"0", "1", "2", "0x7fffffffffffffff", "0x8000000000000000",
                                  "0x8000000000000001", "0xffffffffffffffff"};
static const char *const W32[] = {"0", "5", "0x7fffffff", "0x80000000", "0x80000001"};
int main(void) {
  printf("dec_neg     ");
  for (int i = 0; i < 7; i++)
    printf(" %ld", F(long, dec_neg)(W64[i]));
  printf("\ncnt_wrap    ");
  for (int i = 0; i < 7; i++)
    printf(" %ld", F(long, cnt_wrap)(W64[i]));
  printf("\nspin         %ld %ld %ld\ncount_down  ", F(long, spin)("0x7ffffffffffffffe", "0x8000000000000001"),
         F(long, spin)("0x8000000000000001", "0x7ffffffffffffffe"), F(long, spin)("0xfffffffffffffffe", "0x10"));
  for (int i = 0; i < 7; i++)
    printf(" %ld", F(long, count_down)(W64[i]));
  printf("\ndec_neg32   ");
  for (int i = 0; i < 5; i++)
    printf(" %ld", F(long, dec_neg32)(W32[i]));
  printf("\nsign_of      %ld %ld %ld %ld\nsign_of32    %ld %ld %ld %ld", F(long, sign_of)(W64[3], W64[0]),
         F(long, sign_of)(W64[4], "0xfffffffffffffffa"), F(long, sign_of)(W64[6], "0xfffffffffffffffe"),
         F(long, sign_of)(W64[5], W64[3]), F(long, sign_of32)(W32[2], W32[0]),
         F(long, sign_of32)(W32[3], "0xfffffffa"), F(long, sign_of32)("0xffffffff", "0xfffffffe"),
         F(long, sign_of32)(W32[4], "0x7ffffff0"));
  printf("\npeek         %ld %ld %ld %ld\n", F(long, peek)("abcd", "2"), F(long, peek)("abcd", W64[4]),
         F(long, peek)("abcd", W64[3]), F(long, peek)("abcd", "3"));
  return 0;
}
"#;
    const REG_FUNCS: &str = "sign_of,peek";
    const REG_WANT: &str = "sign_of      1 2 0 1\npeek         99 -1 -2 100\n";
    const REG_MAIN: &str = r#"
#define F(ret, f) ((ret (*)())(void (*)())f)
int main(void) {
  printf("sign_of      %ld %ld %ld %ld\n", F(long, sign_of)("0x7fffffffffffffff", "0"),
         F(long, sign_of)("0x8000000000000000", "0xfffffffffffffffa"),
         F(long, sign_of)("0xffffffffffffffff", "0xfffffffffffffffe"),
         F(long, sign_of)("0x8000000000000001", "0x7fffffffffffffff"));
  printf("peek         %ld %ld %ld %ld\n", F(long, peek)("abcd", "2"), F(long, peek)("abcd", "0x8000000000000000"),
         F(long, peek)("abcd", "0x7fffffffffffffff"), F(long, peek)("abcd", "3"));
  return 0;
}
"#;
    const FUNCS: &str = "trim_right,count_down,run_len,word_end,pick_len,zext_walk,both_ways,halved";
    const WANT: &str = "-7 23 13\n2 7\n2 125\n-3 7 0\n-1 3\n6 9 3 0 -1\n";
    const MAIN: &str = r#"
#define F(ret, f) ((ret (*)())(void (*)())f)
int main(void) {
  char a[] = "   ", b[] = "abc  ", c[] = "xy ";
  static const int arr[] = {-4, 5, -6, 7, 0, -1};
  static unsigned char tab[256];
  for (int i = 0; i < 256; i++)
    tab[i] = (unsigned char)(i % 7);
  printf("%ld %ld %ld\n", F(long, trim_right)(a), F(long, trim_right)(b), F(long, trim_right)(c));
  printf("%ld %ld\n", F(long, count_down)(arr, "abcd"), F(long, count_down)(arr, "abcdef"));
  printf("%ld %ld\n", F(long, run_len)(tab, "hello world", 3L), F(long, run_len)(tab, "zzzzzzz", 100L));
  printf("%ld %ld %ld\n", F(long, word_end)("ab cd", 0u), F(long, both_ways)("abcdefghij", 99UL),
         F(long, both_ways)("abc", 1UL));
  printf("%ld %ld\n", F(long, halved)("a"), F(long, halved)("abcdefgh"));
  static const unsigned int lens[] = {0, 4, 9, 2};
  static const unsigned int at[] = {6, 0};
  printf("%ld %ld %ld %ld %ld\n", F(long, pick_len)(lens, "ab  ", 0), F(long, pick_len)(lens, "abc ", 1),
         F(long, pick_len)(lens, "a ", 3), F(long, zext_walk)(at, "aqbbbbbb"), F(long, zext_walk)(at + 1, "q"));
  return 0;
}
"#;
    const EQ_FUNCS: &str = "d_eq32,c_eq64,c_ne64,c_or64,c_eq7";
    const EQ_WANT: &str = "d_eq32   2 1 0\nc_eq64   2 1 0 1 0\nc_ne64   0 1 0 1 0\nc_or64   2 1 0 1 0\n\
c_eq7    1 2 0 1 0\n";
    const EQ_MAIN: &str = r#"
#define F(ret, f) ((ret (*)())(void (*)())f)
static const char *const E[] = {"10000000000000000000", "0x8000000000000001", "5", "0xffffffffffffffff", "0"};
int main(void) {
  unsigned int a[2] = {htonl(3000000000u), 0}, b[2] = {htonl(0x80000001u), 0}, c[2] = {htonl(5), 0};
  printf("d_eq32   %ld %ld %ld\n", F(long, d_eq32)(a, "y"), F(long, d_eq32)(b, "y"), F(long, d_eq32)(c, "y"));
  printf("c_eq64  ");
  for (int i = 0; i < 5; i++)
    printf(" %ld", F(long, c_eq64)(E[i], "y"));
  printf("\nc_ne64  ");
  for (int i = 0; i < 5; i++)
    printf(" %ld", F(long, c_ne64)(E[i], "y"));
  printf("\nc_or64  ");
  for (int i = 0; i < 5; i++)
    printf(" %ld", F(long, c_or64)(E[i], "y"));
  printf("\nc_eq7    %ld %ld %ld %ld %ld\n", F(long, c_eq7)("0x8000000000000000", "y"), F(long, c_eq7)("7", "y"),
         F(long, c_eq7)("6", "y"), F(long, c_eq7)("x", "x0xffffffffffffffff"), F(long, c_eq7)("0", "y"));
  return 0;
}
"#;
    const WIDE: &[&str] = &["d_eq32", "c_eq64", "c_ne64", "c_or64"];
    const WRAPS: &[&str] = &["dec_neg", "cnt_wrap", "spin", "count_down", "dec_neg32"];
    const OLD: &[&str] = &["trim_right", "count_down", "run_len", "word_end", "zext_walk", "both_ways", "halved"];
    // (fixture, functions, main, output, the lines option off prints and what
    // option on prints instead, the functions option on must print unchanged)
    type Case<'a> = (&'a str, &'a str, &'a str, &'a str, &'a [(&'a str, &'a str)], &'a [&'a str]);
    let cases: [Case; 9] = [
        (
            "castsign_eq_gcc_O0_x86_64",
            EQ_FUNCS,
            EQ_MAIN,
            EQ_WANT,
            &[
                ("\n  unsigned long v1; // stack - 0x10\n  \n  v1 = strtoul(a0,NULL,0);\n  if (*a1 == 'x')\n    v1 = strtoul(&a1[1],NULL,0);\n  if (v1 == 7)",
                 "\n  long v1; // stack - 0x10\n  \n  v1 = strtoul(a0,NULL,0);\n  if (*a1 == 'x')\n    v1 = strtoul(&a1[1],NULL,0);\n  if (v1 == 7)"),
                ("if ((long)v1 <= -1)", "if (v1 <= -1)"),
            ],
            WIDE,
        ),
        (
            "castsign_eq_clang_O0_x86_64",
            EQ_FUNCS,
            EQ_MAIN,
            EQ_WANT,
            &[
                ("\n  unsigned long v1; // stack - 0x28\n  \n  v1 = strtoul(a0,NULL,0);\n  if (*a1 == 'x')\n    v1 = strtoul(&a1[1],NULL,0);\n  if (v1 == 7)",
                 "\n  long v1; // stack - 0x28\n  \n  v1 = strtoul(a0,NULL,0);\n  if (*a1 == 'x')\n    v1 = strtoul(&a1[1],NULL,0);\n  if (v1 == 7)"),
                ("if ((long)v1 <= -1)", "if (v1 <= -1)"),
            ],
            WIDE,
        ),
        (
            "castsign_wrap_gcc_O0_x86_64",
            WRAP_FUNCS,
            WRAP_MAIN,
            WRAP_WANT,
            &[
                ("\n  unsigned long v1; // stack - 0x10", "\n  long v1; // stack - 0x10"),
                ("if (0 <= (long)v1)\n    return 1;", "if (0 <= v1)\n    return 1;"),
                ("\n  unsigned int v1; // stack - 0x14", "\n  int v1; // stack - 0x14"),
                ("if (0 <= (int)v1)", "if (0 <= v1)"),
                ("if ((long)strlen(a0) <= (long)v1)", "if ((long)strlen(a0) <= v1)"),
            ],
            WRAPS,
        ),
        (
            "castsign_wrap_clang_O0_x86_64",
            WRAP_FUNCS,
            WRAP_MAIN,
            WRAP_WANT,
            &[
                ("\n  unsigned long v1; // stack - 0x28", "\n  long v1; // stack - 0x28"),
                ("\n  unsigned int v2; // stack - 0x1c", "\n  int v2; // stack - 0x1c"),
                ("if (0 <= (int)v2)", "if (0 <= v2)"),
                ("if ((long)strlen(a0) <= (long)v1) // branch-flip", "if ((long)strlen(a0) <= v1) // branch-flip"),
            ],
            WRAPS,
        ),
        (
            "castsign_wrap_gcc_O1_x86_64",
            REG_FUNCS,
            REG_MAIN,
            REG_WANT,
            &[
                ("\n  unsigned long v1; // rax", "\n  long v1; // rax"),
                ("if ((long)v1 < (long)strlen(a0))", "if (v1 < (long)strlen(a0))"),
            ],
            &["sign_of"],
        ),
        ("castsign_gcc_O0_x86_64", FUNCS, MAIN, WANT, &[], OLD),
        ("castsign_clang_O0_x86_64", FUNCS, MAIN, WANT, &[], OLD),
        ("castsign_gcc_O1_x86_64", FUNCS, MAIN, WANT, &[], OLD),
        (
            "castsign_clang_O1_x86_64",
            FUNCS,
            MAIN,
            WANT,
            &[("v1 = (unsigned long)a0[a2];", "v1 = a0[a2];")],
            OLD,
        ),
    ];
    let sp = specs();
    let compilers: Vec<&str> = ["gcc", "clang"]
        .into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    for (fixture, funcs, main, want, lines, same) in cases {
        let bin = repo_root()
            .join("decompiler/crates/kuna-analysis/tests/fixtures")
            .join(fixture)
            .to_str()
            .unwrap()
            .to_string();
        let mut printed: Vec<String> = Vec::new();
        for opt in ["off", "on"] {
            let args = [
                "decompile-all", bin.as_str(), "--functions", funcs, "--sleighpath", sp.as_str(),
                "--option", "castsign", opt, "--option", "castwiden", "off",
            ];
            let (stdout, stderr, ok) = run_kuna(&args);
            assert!(ok, "kuna decompile-all failed: {stderr}");
            let semantic = without_variable_sources(&stdout);
            for (off, on) in lines {
                let want = if opt == "on" { on } else { off };
                assert!(semantic.contains(&without_variable_sources(want)), "{fixture} option {opt} does not print `{want}`:\n{stdout}");
            }
            for cc in &compilers {
                for level in ["-O0", "-O2"] {
                    let dir = std::env::temp_dir()
                        .join(format!("kuna-castsign-rt-{}-{fixture}-{opt}-{cc}{level}", std::process::id()));
                    std::fs::create_dir_all(&dir).unwrap();
                    let src = dir.join("rt.c");
                    let exe = dir.join("rt");
                    std::fs::write(
                        &src,
                        format!(
                            "#include <arpa/inet.h>\n#include <stdbool.h>\n#include <stdio.h>\n#include <stdlib.h>\n#include <string.h>\n{stdout}\n{main}"
                        ),
                    )
                    .unwrap();
                    let out = Command::new(cc)
                        .args(["-std=gnu11", "-w", "-Wno-error=int-conversion", level])
                        .args(["-o", exe.to_str().unwrap(), src.to_str().unwrap()])
                        .output()
                        .expect("spawn the C compiler");
                    assert!(
                        out.status.success(),
                        "{cc} {level} rejected the printed C ({fixture}, option {opt}):\n{}",
                        String::from_utf8_lossy(&out.stderr)
                    );
                    let run = process::required_output(&mut Command::new(&exe));
                    let _ = std::fs::remove_dir_all(&dir);
                    assert_eq!(
                        String::from_utf8_lossy(&run.stdout),
                        want,
                        "{fixture} printed with option {opt} and built by {cc} {level} computes a different value:\n{stdout}"
                    );
                }
            }
            printed.push(semantic);
        }
        for name in same {
            assert_eq!(
                castsign_function(&printed[0], name),
                castsign_function(&printed[1], name),
                "{fixture}: option castsign changed {name}"
            );
        }
    }
    // Rust has no implicit integer conversions, so the option leaves Rust output alone.
    let bin = repo_root()
        .join("decompiler/crates/kuna-analysis/tests/fixtures/castsign_wrap_gcc_O0_x86_64")
        .to_str()
        .unwrap()
        .to_string();
    let rust: Vec<String> = ["off", "on"]
        .iter()
        .map(|opt| {
            let args = [
                "decompile-all", bin.as_str(), "--functions", WRAP_FUNCS, "--sleighpath", sp.as_str(),
                "--language", "rust", "--option", "castsign", opt,
            ];
            let (stdout, stderr, ok) = run_kuna(&args);
            assert!(ok, "kuna decompile-all --language rust failed: {stderr}");
            stdout
        })
        .collect();
    assert_eq!(rust[0], rust[1], "castsign changed Rust output");
}

/// (kuna `castobject`) A local whose address only fills declared `int *`
/// parameters is declared `int` when no reader wants it unsigned, and the call
/// stops casting its address.  `castobject_x86_64.c` reaps children that exit
/// with 0, 7 and 255 or die of a signal, so every status bit the tests read is
/// exercised, and runs the `init_*`, `gid_mixed` and `stored_value` objects,
/// which start with a parameter's value that `waitpid` on a pid with no child
/// (or `getgroups` with a size of 0) leaves in place, over values with the top
/// bit set.  `init_signed` is read signed only and moves at `-O0`; the others
/// have a reader that wants the other signedness (a logical shift, an unsigned
/// compare, a zero-extension, a signed compare of a `gid_t`, or a logical shift
/// of the value stored into the object), and they and the other readers that
/// disagree (a logical shift at `-O2`, an address kept in a pointer, a byte read
/// of one half, wrapping arithmetic) must print exactly as they did.  Each
/// fixture is decompiled with the option off and on, the printed functions are
/// compiled with gcc and clang at -O0 and -O2, and every build must print what
/// the fixture binary prints.  `clang -O2` allocates the status slot with
/// `push %rax` and kuna reads the pushed register back after the call, and it
/// prints `escaped` and `two_widths` through a piece accessor, which is not C;
/// both are defects of their own, the same in either arm, so the functions that
/// reap children are not built from that fixture.
#[test]
fn an_out_parameter_local_round_trips_through_the_printed_c() {
    const ALL: &str = "exit_code,status_order,reaped,escaped,two_widths,plus_one,cancel_state,\
init_signed,init_ushr,init_ult,init_zext,gid_mixed,stored_value";
    const CLANG_O2: &str =
        "plus_one,cancel_state,init_signed,init_ushr,init_ult,init_zext,gid_mixed,stored_value";
    const UNCHANGED: &[&str] = &[
        "status_order", "reaped", "escaped", "two_widths", "plus_one", "cancel_state", "init_ushr", "init_ult",
        "init_zext", "gid_mixed", "stored_value",
    ];
    const EVERY: &[&str] = &[
        "exit_code", "status_order", "reaped", "escaped", "two_widths", "plus_one", "cancel_state", "init_signed",
        "init_ushr", "init_ult", "init_zext", "gid_mixed", "stored_value",
    ];
    const INIT: &str = "init 00000000 0 0 0 0 0 ff800000\ninit 00000020 0 0 2 0 12 8\n\
init 00000100 0 0 1 0 101 ff800000\ninit 00007f00 0 3 127 0 12869 ff80003f\n\
init 80000000 -2048 800 0 ffffffff00000000 -1 ffc00000\ninit 80000001 0 0 0 0 -1 ffc00000\n\
init fffffffe 255 ff 255 ff -1 f\ninit ffffffff 255 ff 255 ff -1 f\ninit 87654321 67 43 101 43 -1 c\n\
init ffff0000 -1 fff 0 ffffffff00000000 -1 ffffff80\ninit 800000ff 0 0 0 0 -1 c\n";
    const REAP: &str = "exit_code    0 7 255 -2\nstatus_order 0 3 9\nreaped       5 -15\nescaped      9 0\n\
two_widths   9 3840\n";
    const TAIL: &str = "plus_one     257 16\ncancel_state 7 7\n";
    let want = format!("{REAP}{TAIL}{INIT}");
    let want_clang_o2 = format!("{TAIL}{INIT}");
    const MAIN: &str = r#"
#define F(ret, f) ((ret (*)())(void (*)())f)
static int child(int code) {
  int pid = fork();
  if (pid == 0) {
    if (code < 0)
      raise(-code);
    _exit(code);
  }
  return pid;
}
int main(void) {
#ifndef SUBSET
  int a = F(int, exit_code)(child(0)), b = F(int, exit_code)(child(7)), c = F(int, exit_code)(child(255)),
      d = F(int, exit_code)(child(-SIGTERM));
  printf("exit_code    %d %d %d %d\n", a, b, c, d);
  long e = F(long, status_order)(child(0)), f = F(long, status_order)(child(3)),
       g = F(long, status_order)(child(-SIGKILL));
  printf("status_order %ld %ld %ld\n", e, f, g);
  child(5);
  long h = F(long, reaped)();
  child(-SIGTERM);
  long i = F(long, reaped)();
  printf("reaped       %ld %ld\n", h, i);
  int j = F(int, escaped)(child(9)), k = F(int, escaped)(child(-SIGTERM));
  printf("escaped      %d %d\n", j, k);
  int l = F(int, two_widths)(child(9)), m = F(int, two_widths)(child(-SIGTERM));
  printf("two_widths   %d %d\n", l, m);
#endif
  unsigned long n = F(unsigned long, plus_one)(child(1)), o = F(unsigned long, plus_one)(child(-SIGTERM));
  printf("plus_one     %lu %lu\n", n, o);
  int q = F(int, cancel_state)();
  pthread_setcancelstate(PTHREAD_CANCEL_DISABLE, NULL);
  int r = F(int, cancel_state)();
  printf("cancel_state %d %d\n", q, r);
  unsigned vals[] = {0, 0x20, 0x100, 0x7f00, 0x80000000u, 0x80000001u, 0xfffffffeu, 0xffffffffu, 0x87654321u,
                     0xffff0000u, 0x800000ffu};
  for (unsigned t = 0; t < sizeof vals / sizeof *vals; t++)
    printf("init %08x %d %x %d %lx %ld %x\n", vals[t], F(int, init_signed)(vals[t]), F(unsigned, init_ushr)(vals[t]),
           F(int, init_ult)(vals[t]), F(long, init_zext)(vals[t]), F(long, gid_mixed)((int)vals[t]),
           F(unsigned, stored_value)(vals[t]));
  return 0;
}
"#;
    // (fixture, functions, output, the lines option off prints and what option on
    // prints instead, the functions option on must print unchanged)
    type Case<'a> = (&'a str, &'a str, &'a str, &'a [(&'a str, &'a str)], &'a [&'a str]);
    let cases: [Case; 4] = [
        (
            "castobject_gcc_O0_x86_64",
            ALL,
            want.as_str(),
            &[
                ("\n  unsigned int v1; // stack - 0x14\n  \n  if (waitpid(a0,(int *)&v1,0) <= -1)",
                 "\n  int v1; // stack - 0x14\n  \n  if (waitpid(a0,&v1,0) <= -1)"),
                ("    return (int)v1 >> 8 & 0xff;", "    return v1 >> 8 & 0xff;"),
                ("\n  unsigned int v1; // stack - 0x14\n  \n  v1 = a0;\n  waitpid(0x7ffffff0,(int *)&v1,1);",
                 "\n  int v1; // stack - 0x14\n  \n  v1 = a0;\n  waitpid(0x7ffffff0,&v1,1);"),
                ("(int)v1 >> 8 & 0xff : (int)v1 >> 0x14;", "v1 >> 8 & 0xff : v1 >> 0x14;"),
            ],
            UNCHANGED,
        ),
        (
            "castobject_clang_O0_x86_64",
            ALL,
            want.as_str(),
            &[
                ("\n  unsigned int v1; // stack - 0x14", "\n  int v1; // stack - 0x14"),
                ("if (0 <= waitpid(a0,(int *)&v1,0)) {", "if (0 <= waitpid(a0,&v1,0)) {"),
                ("  waitpid(0x7ffffff0,(int *)&v1,1);\n  v3 = (v1 & 0x7f) ? (int)v1 >> 8",
                 "  waitpid(0x7ffffff0,&v1,1);\n  v3 = (v1 & 0x7f) ? v1 >> 8"),
            ],
            UNCHANGED,
        ),
        ("castobject_gcc_O2_x86_64", ALL, want.as_str(), &[("(int *)&v1", "(int *)&v1")], EVERY),
        ("castobject_clang_O2_x86_64", CLANG_O2, want_clang_o2.as_str(), &[], &EVERY[5..]),
    ];
    let sp = specs();
    let compilers: Vec<&str> = ["gcc", "clang"]
        .into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    for (fixture, funcs, want, lines, same) in cases {
        let bin = repo_root()
            .join("decompiler/crates/kuna-analysis/tests/fixtures")
            .join(fixture)
            .to_str()
            .unwrap()
            .to_string();
        let mut printed: Vec<String> = Vec::new();
        for opt in ["off", "on"] {
            let args = [
                "decompile-all", bin.as_str(), "--functions", funcs, "--sleighpath", sp.as_str(),
                "--option", "castobject", opt,
            ];
            let (stdout, stderr, ok) = run_kuna(&args);
            assert!(ok, "kuna decompile-all failed: {stderr}");
            let semantic = without_variable_sources(&stdout);
            for (off, on) in lines {
                let line = if opt == "on" { on } else { off };
                assert!(semantic.contains(&without_variable_sources(line)), "{fixture} option {opt} does not print `{line}`:\n{stdout}");
            }
            for cc in &compilers {
                for level in ["-O0", "-O2"] {
                    let dir = std::env::temp_dir()
                        .join(format!("kuna-castobject-rt-{}-{fixture}-{opt}-{cc}{level}", std::process::id()));
                    std::fs::create_dir_all(&dir).unwrap();
                    let src = dir.join("rt.c");
                    let exe = dir.join("rt");
                    let subset = if funcs == CLANG_O2 { "#define SUBSET\n" } else { "" };
                    std::fs::write(
                        &src,
                        format!(
                            "{subset}#include <pthread.h>\n#include <signal.h>\n#include <stdbool.h>\n#include <stdio.h>\n#include <stdlib.h>\n#include <sys/wait.h>\n#include <unistd.h>\n{stdout}\n{MAIN}"
                        ),
                    )
                    .unwrap();
                    let out = Command::new(cc)
                        .args(["-std=gnu11", "-w", "-Wno-error=int-conversion", level])
                        .args(["-o", exe.to_str().unwrap(), src.to_str().unwrap()])
                        .output()
                        .expect("spawn the C compiler");
                    assert!(
                        out.status.success(),
                        "{cc} {level} rejected the printed C ({fixture}, option {opt}):\n{}",
                        String::from_utf8_lossy(&out.stderr)
                    );
                    let run = process::required_output(&mut Command::new(&exe));
                    let _ = std::fs::remove_dir_all(&dir);
                    assert_eq!(
                        String::from_utf8_lossy(&run.stdout),
                        want,
                        "{fixture} printed with option {opt} and built by {cc} {level} computes a different value:\n{stdout}"
                    );
                }
            }
            printed.push(semantic);
        }
        for name in same {
            assert_eq!(
                castsign_function(&printed[0], name),
                castsign_function(&printed[1], name),
                "{fixture}: option castobject changed {name}"
            );
        }
    }
}

/// (kuna `castsign`) A declaration whose type is locked is never re-signed.  A
/// `--assert type` on a stack local and on a register local, and a DWARF local
/// the source declares `unsigned long`, keep that type and the `(long)` their
/// signed comparison needs, option on as off.  Unlocked, the same variables are
/// declared `long`, so each case also checks that the option fires there.
#[test]
fn castsign_leaves_a_locked_declaration_alone() {
    let sp = specs();
    let fixture = |name: &str| {
        repo_root()
            .join("decompiler/crates/kuna-analysis/tests/fixtures")
            .join(name)
            .to_str()
            .unwrap()
            .to_string()
    };
    let unsigned_stack: &[&str] = &["\n  unsigned long v1; // stack - 0x10", "if (0 <= (long)v1)\n    return 1;"];
    let signed_stack: &[&str] = &["\n  long v1; // stack - 0x10", "if (0 <= v1)\n    return 1;"];
    let unsigned_reg: &[&str] = &["\n  unsigned long v1; // rax", "if ((long)v1 < (long)strlen(a0))"];
    let signed_reg: &[&str] = &["\n  long v1; // rax", "if (v1 < (long)strlen(a0))"];
    // (fixture, function, assertion, printed unlocked with the option on, printed locked)
    let asserted = [
        ("castsign_wrap_gcc_O0_x86_64", "sign_of", "type v1 unsigned long", signed_stack, unsigned_stack),
        ("castsign_wrap_gcc_O1_x86_64", "peek", "type v1 unsigned long", signed_reg, unsigned_reg),
    ];
    for (name, func, assertion, unlocked, locked) in asserted {
        let bin = fixture(name);
        let (stdout, stderr, ok) =
            run_kuna(&["decompile", &bin, func, "--sleighpath", &sp, "--option", "castsign", "on"]);
        assert!(ok, "kuna decompile failed: {stderr}");
        let semantic = without_variable_sources(&stdout);
        for want in unlocked {
            assert!(semantic.contains(&without_variable_sources(want)), "{name} {func} unlocked does not print `{want}`:\n{stdout}");
        }
        for opt in ["on", "off"] {
            let args = [
                "decompile", bin.as_str(), func, "--sleighpath", sp.as_str(), "--assert", assertion,
                "--option", "castsign", opt,
            ];
            let (stdout, stderr, ok) = run_kuna(&args);
            assert!(ok, "kuna decompile --assert failed: {stderr}");
            let semantic = without_variable_sources(&stdout);
            for want in locked {
                assert!(
                    semantic.contains(&without_variable_sources(want)),
                    "{name} {func} under `{assertion}`, option {opt}, does not print `{want}`:\n{stdout}"
                );
            }
        }
    }
    let dwarf = fixture("castsign_dwarf_gcc_O0_x86_64");
    let locked: &[&str] = &["\n  unsigned long n; // stack - 0x10", "if (0 <= (long)n)\n    return 1;"];
    for opt in ["on", "off"] {
        let args = [
            "decompile-all", dwarf.as_str(), "--functions", "sign_of", "--sleighpath", sp.as_str(),
            "--option", "castsign", opt,
        ];
        let (stdout, stderr, ok) = run_kuna(&args);
        assert!(ok, "kuna decompile-all failed: {stderr}");
        let semantic = without_variable_sources(&stdout);
        for want in locked {
            assert!(semantic.contains(&without_variable_sources(want)), "the DWARF local, option {opt}, does not print `{want}`:\n{stdout}");
        }
    }
    let dir = std::env::temp_dir().join(format!("kuna-castsign-lock-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let stripped = dir.join("sign_of");
    let strip = process::optional_output(
        Command::new("objcopy").args(["--strip-debug", dwarf.as_str(), stripped.to_str().unwrap()]),
    );
    if strip.is_some() {
        let args = [
            "decompile-all", stripped.to_str().unwrap(), "--functions", "sign_of", "--sleighpath",
            sp.as_str(), "--option", "castsign", "on",
        ];
        let (stdout, stderr, ok) = run_kuna(&args);
        assert!(ok, "kuna decompile-all (debug info stripped) failed: {stderr}");
        let semantic = without_variable_sources(&stdout);
        for want in signed_stack {
            assert!(semantic.contains(&without_variable_sources(want)), "stripped, the slot does not print `{want}`:\n{stdout}");
        }
    } else {
        eprintln!("castsign stripped-DWARF check: no `objcopy`");
    }
    let _ = std::fs::remove_dir_all(&dir);
}

/// A pointer plus a constant that is a whole number of elements prints as
/// pointer arithmetic under `castarith`, `((unsigned int *)a0)[0x2b]` with one
/// cast, instead of the integer round trip `*(unsigned int *)((long)a0 +
/// 0xac)`.  The round trip compiles every function between the `tested`
/// markers of `castarith_x86_64.c` exactly as printed, between the fixture's
/// own prelude and `main`, for the gcc -O0, clang -O0 and gcc -O2 builds with
/// the option on and off, and checks the program prints what the binary does:
/// loads and stores of 1, 2, 4 and 8 bytes signed and unsigned, a double, a
/// negative offset, an offset that is not whole elements (kept), a pointer
/// passed on, compared, and stepped in a loop, a base typed as another pointer,
/// a record base, which keeps its fields, loaded bytes and words widened under
/// the subscript, an integer base, which keeps the integer form, and negative
/// indexes of 2^31 elements or more, which keep it too: C reads the literal
/// `0x80000000` as an `unsigned int`, so `p[-0x80000000]` would point forward.
/// `main` reads those through a 48 GiB `MAP_NORESERVE` map, and prints the same
/// line in the binary and the round trip when the map is refused.
#[test]
fn a_pointer_plus_whole_elements_round_trips_through_the_printed_c() {
    let fx = repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures");
    let src = std::fs::read_to_string(fx.join("castarith_x86_64.c")).unwrap();
    let section = |from: &str, to: &str| -> String {
        let tail = src.split(from).nth(1).unwrap();
        tail.split(to).next().unwrap().to_string()
    };
    let prelude = section("/* prelude */", "/* tested */");
    let tested_src = section("/* tested */", "/* main */");
    let main = src.split("/* main */").nth(1).unwrap().to_string();
    let tested: Vec<&str> = tested_src
        .lines()
        .filter_map(|l| l.strip_prefix("KEEP void "))
        .filter_map(|l| l.split('(').next())
        .collect();
    assert!(tested.len() >= 24, "{tested:?}");
    let sp = specs();
    let runs_here = cfg!(all(target_os = "linux", target_arch = "x86_64"));
    let have_cc = process::optional_output(Command::new("cc").arg("--version")).is_some();
    for build in ["gcc_O0", "clang_O0", "gcc_O2"] {
        let bin = fx.join(format!("castarith_{build}_x86_64"));
        let bin = bin.to_str().unwrap();
        for arm in ["on", "off"] {
            let args = [
                "decompile-all", bin, "--sleighpath", sp.as_str(),
                "--option", "structdefs", "on", "--option", "castarith", arm,
            ];
            let (stdout, stderr, ok) = run_kuna(&args);
            assert!(ok, "kuna decompile-all failed: {stderr}");
            let mut body = String::new();
            let mut seen = 0;
            for part in stdout.split("// Function: ").skip(1) {
                let name = part.split(' ').next().unwrap_or("");
                if tested.contains(&name) {
                    body.push_str(part.split_once('\n').map(|(_, b)| b).unwrap_or(""));
                    seen += 1;
                }
            }
            assert_eq!(seen, tested.len(), "{build} {arm}: missing a tested function\n{stdout}");
            let want: &[&str] = if arm == "on" {
                &[
                    "((short *)a0)[-0xd]",
                    "take(&((unsigned int *)a0)[4]);",
                    "((long *)a0)[0x11] = a1 * 7;",
                    "((unsigned int *)a0)[-1] = 0xfeed;",
                    "((unsigned short *)a0)[3]",
                    "&((char *)",
                    "*(unsigned int *)((long)a0 + 0x6a)",
                    "a1 <= (int)((unsigned char *)a0)[0x11]",
                    "((unsigned short *)a0)[0x24] << 4",
                    "long *)a0)[-0x7fffffff]",
                    "long *)a0)[0x80000000]",
                    "long *)((long)a0 + -0x400000000)",
                    "long *)((long)a0 + -0x7fffffff8)",
                    "*(int *)((long)a0 + -0x200000000)",
                    "*(short *)((long)a0 + -0x100000000)",
                ]
            } else {
                &["take((unsigned int *)((long)a0 + 0x10));", "*(unsigned int *)((long)a0 + 0x6a)"]
            };
            for w in want {
                assert!(body.contains(w), "{build} castarith {arm}: expected `{w}`\n{body}");
            }
            for wide in ["[-0x80000000]", "[-0xffffffff]"] {
                assert!(
                    !body.contains(wide),
                    "{build} castarith {arm}: C reads the index `{wide}` as unsigned, so it points forward\n{body}"
                );
            }
            assert!(
                !body.contains("(unsigned int)((unsigned char *)"),
                "{build} castarith {arm}: a widening castimplied leaves out came back over a subscript\n{body}"
            );
            if !runs_here || !have_cc {
                eprintln!("castarith round trip: no x86-64 host or no `cc`, spelling checked only");
                continue;
            }
            let expected = process::required_output(&mut Command::new(bin));
            let dir = std::env::temp_dir()
                .join(format!("kuna-castarith-rt-{}-{build}-{arm}", std::process::id()));
            std::fs::create_dir_all(&dir).unwrap();
            let c = dir.join("rt.c");
            let exe = dir.join("rt");
            std::fs::write(
                &c,
                format!("#include <stdio.h>\n#include <string.h>\n#include <stdbool.h>\n{prelude}\n{body}\n{main}"),
            )
            .unwrap();
            let cc = Command::new("cc")
                .args(["-std=gnu11", "-w", "-o", exe.to_str().unwrap(), c.to_str().unwrap()])
                .output()
                .expect("spawn cc");
            assert!(
                cc.status.success(),
                "{build} castarith {arm}: the printed functions did not compile:\n{}\n{body}",
                String::from_utf8_lossy(&cc.stderr)
            );
            let got = process::required_output(&mut Command::new(&exe));
            let _ = std::fs::remove_dir_all(&dir);
            assert_eq!(
                String::from_utf8_lossy(&got.stdout),
                String::from_utf8_lossy(&expected.stdout),
                "{build} castarith {arm}: the printed C computes something else\n{body}"
            );
        }
    }
}

/// `castindex`: a pointer plus a variable index of whole elements prints as a
/// subscript, and the difference of two `char *` as `p - q`.  The round trip
/// compiles the prelude of `castindex_x86_64.c`, kuna's printing of every
/// function between its markers and its `main`, with the option on and off, and
/// checks the program prints what the binary does.  The indexes are `int`,
/// `unsigned int`, `short`, `signed char`, `unsigned char` and `long`, negative
/// where signed, and `main` reads an `unsigned int` index with its top bit set
/// through a 24 GiB `MAP_NORESERVE` map, which a sign extension would read 2^31
/// elements backward; the differences are divided, shifted, compared signed and
/// unsigned, and passed as a length.  A scale that is not the element's size, a
/// byte offset read at 8 bytes and a `long *` difference keep the integer form.
/// A base64 decoder indexes its `malloc`ed global table by input bytes of 0x80
/// and up, into a filler with the sign bit set, and checksums every quad.
/// `castwiden` is held off: it leaves out some of the widenings pinned here,
/// which `an_implied_widening_round_trips_through_the_printed_c` covers.
#[test]
fn a_variable_index_and_a_byte_pointer_difference_round_trip_through_the_printed_c() {
    let fx = repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures");
    let src = std::fs::read_to_string(fx.join("castindex_x86_64.c")).unwrap();
    let section = |from: &str, to: &str| -> String {
        let tail = src.split(from).nth(1).unwrap();
        tail.split(to).next().unwrap().to_string()
    };
    let prelude = section("/* prelude */", "/* tested */");
    let tested_src = section("/* tested */", "/* main */");
    let main = src.split("/* main */").nth(1).unwrap().to_string();
    let tested: Vec<&str> = tested_src
        .lines()
        .filter_map(|l| l.strip_prefix("KEEP void "))
        .filter_map(|l| l.split('(').next())
        .collect();
    assert!(tested.len() >= 20, "{tested:?}");
    let sp = specs();
    let runs_here = cfg!(all(target_os = "linux", target_arch = "x86_64"));
    let have_cc = process::optional_output(Command::new("cc").arg("--version")).is_some();
    for build in ["gcc_O0", "clang_O0", "gcc_O2"] {
        let bin = fx.join(format!("castindex_{build}_x86_64"));
        let bin = bin.to_str().unwrap();
        // The spellings are pinned with `elemptr` off, which leaves the bases
        // `void *` for this option to rewrite; with it on (the default) P5 types
        // them first, and the printed C must still compute the same values.
        for (elem, arm) in [("off", "on"), ("off", "off"), ("on", "on"), ("on", "off")] {
            let args = [
                "decompile-all", bin, "--sleighpath", sp.as_str(), "--option", "castindex", arm, "--option",
                "castwiden", "off", "--option", "elemptr", elem,
            ];
            let (stdout, stderr, ok) = run_kuna(&args);
            assert!(ok, "kuna decompile-all failed: {stderr}");
            let mut body = String::new();
            let mut seen = 0;
            for part in stdout.split("// Function: ").skip(1) {
                let name = part.split(' ').next().unwrap_or("");
                if tested.contains(&name) {
                    body.push_str(part.split_once('\n').map(|(_, b)| b).unwrap_or(""));
                    seen += 1;
                }
            }
            assert_eq!(seen, tested.len(), "{build} {arm}: missing a tested function\n{stdout}");
            let kept: &[&str] = &[
                "*(long *)((long)a0 + (long)a1 * 0x10 + 8)",
                "*(int *)((long)a0 + (long)a1 * 0xc)",
                "*(long *)((long)a0 + a1)",
                "(long)a1 - (long)a0 >> 3",
            ];
            let want: &[&str] = if arm == "on" {
                &[
                    "((char *)a0)[a1]",
                    "((short *)a0)[a1]",
                    "((unsigned short *)a0)[a1]",
                    "((unsigned int *)a0)[a1]",
                    "((unsigned long *)a0)[a1]",
                    "((double *)a0)[a1]",
                    "strchr(a0,a1) - a0",
                    "((char *)b64_table)[",
                ]
            } else {
                &[
                    "(long)strchr(a0,a1) - (long)a0",
                    "*(short *)((long)a0 + (long)a1 * 2)",
                    "(long)b64_table",
                ]
            };
            if elem == "off" {
                for w in want.iter().chain(kept) {
                    assert!(body.contains(w), "{build} castindex {arm}: expected `{w}`\n{body}");
                }
                if arm == "on" {
                    assert!(!body.contains("(long)b64_table"), "{build}: the table lookup kept its round trip\n{body}");
                }
            }
            if !runs_here || !have_cc {
                eprintln!("castindex round trip: no x86-64 host or no `cc`, spelling checked only");
                continue;
            }
            let expected = process::required_output(&mut Command::new(bin));
            let dir = std::env::temp_dir()
                .join(format!("kuna-castindex-rt-{}-{build}-{elem}-{arm}", std::process::id()));
            std::fs::create_dir_all(&dir).unwrap();
            let c = dir.join("rt.c");
            let exe = dir.join("rt");
            std::fs::write(
                &c,
                format!("#include <stdio.h>\n#include <string.h>\n#include <stdbool.h>\n{prelude}\n{body}\n{main}"),
            )
            .unwrap();
            let cc = Command::new("cc")
                .args(["-std=gnu11", "-w", "-o", exe.to_str().unwrap(), c.to_str().unwrap()])
                .output()
                .expect("spawn cc");
            assert!(
                cc.status.success(),
                "{build} castindex {arm}: the printed functions did not compile:\n{}\n{body}",
                String::from_utf8_lossy(&cc.stderr)
            );
            let got = process::required_output(&mut Command::new(&exe));
            let _ = std::fs::remove_dir_all(&dir);
            assert_eq!(
                String::from_utf8_lossy(&got.stdout),
                String::from_utf8_lossy(&expected.stdout),
                "{build} castindex {arm}: the printed C computes something else\n{body}"
            );
        }
    }
}

/// An enum element keeps the integer form under `castarith`.  kuna prints every
/// enum as a plain `enum`, which C sizes as an `int`, while the fixture's
/// packed enums are 1 and 2 bytes in the binary (as are `-fshort-enums` enums
/// and a C++ `enum class : uint8_t`), so `((color *)p)[3]` would read 12 bytes
/// past `p` instead of 3.  The round trip compiles kuna's own enum typedefs,
/// the prelude of `castarith_enum_x86_64.c` (whose callees read the width the
/// binary passes), the printed functions between its markers and its `main`,
/// with the option on and off, and checks the program prints what the binary
/// does.  The C++ build is checked for spelling only.
#[test]
fn an_enum_element_keeps_the_integer_form_and_round_trips() {
    let fx = repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures");
    let src = std::fs::read_to_string(fx.join("castarith_enum_x86_64.c")).unwrap();
    let section = |from: &str, to: &str| -> String {
        let tail = src.split(from).nth(1).unwrap();
        tail.split(to).next().unwrap().to_string()
    };
    let prelude = section("/* prelude */", "/* tested */");
    let tested_src = section("/* tested */", "/* main */");
    let main = src.split("/* main */").nth(1).unwrap().to_string();
    let tested: Vec<&str> = tested_src
        .lines()
        .filter_map(|l| l.strip_prefix("KEEP void "))
        .filter_map(|l| l.split('(').next())
        .collect();
    assert_eq!(tested.len(), 4, "{tested:?}");
    let sp = specs();
    let runs_here = cfg!(all(target_os = "linux", target_arch = "x86_64"));
    let have_cc = process::optional_output(Command::new("cc").arg("--version")).is_some();
    let decompile = |bin: &str, arm: &str| -> String {
        let args = [
            "decompile-all", bin, "--sleighpath", sp.as_str(),
            "--option", "structdefs", "on", "--option", "castarith", arm,
        ];
        let (stdout, stderr, ok) = run_kuna(&args);
        assert!(ok, "kuna decompile-all failed: {stderr}");
        stdout
    };
    let cxx = fx.join("castarith_enumclass_gpp_O2_x86_64");
    let out = decompile(cxx.to_str().unwrap(), "on");
    let rd = out.split("// Function: rd_enum_class ").nth(1).expect("rd_enum_class printed");
    let rd = rd.split("// Function: ").next().unwrap();
    for w in ["use_kind(*(Kind *)((long)p + 5));", "use_op(*(Op *)((long)p + 6));"] {
        assert!(rd.contains(w), "enum class: expected `{w}`\n{rd}");
    }
    for build in ["gcc_O0", "gcc_O2"] {
        let bin = fx.join(format!("castarith_enum_{build}_x86_64"));
        let bin = bin.to_str().unwrap();
        for arm in ["on", "off"] {
            let stdout = decompile(bin, arm);
            let mut types: Vec<String> = Vec::new();
            let mut body = String::new();
            let mut seen = 0;
            for part in stdout.split("// Function: ").skip(1) {
                let name = part.split(' ').next().unwrap_or("");
                if !tested.contains(&name) {
                    continue;
                }
                seen += 1;
                let mut block: Option<String> = None;
                for line in part.lines().skip(1) {
                    if let Some(b) = block.as_mut() {
                        b.push_str(line);
                        b.push('\n');
                        if line.starts_with('}') && line.ends_with(';') {
                            let b = block.take().unwrap();
                            if !types.contains(&b) {
                                types.push(b);
                            }
                        }
                    } else if line.starts_with("typedef") && line.ends_with('{') {
                        block = Some(format!("{line}\n"));
                    } else {
                        body.push_str(line);
                        body.push('\n');
                    }
                }
            }
            assert_eq!(seen, tested.len(), "{build} {arm}: missing a tested function\n{stdout}");
            assert_eq!(types.len(), 3, "{build} {arm}: expected kuna's color, mark and level\n{stdout}");
            for w in [
                "use_color(*(color *)((long)p + 3));",
                "use_mark(*(mark *)((long)p + 6));",
                "use_level(*(level *)((long)p + 8));",
                "set_color((color *)((long)p + 5));",
            ] {
                assert!(body.contains(w), "{build} castarith {arm}: expected `{w}`\n{body}");
            }
            if !runs_here || !have_cc {
                eprintln!("castarith enum round trip: no x86-64 host or no `cc`, spelling checked only");
                continue;
            }
            let expected = process::required_output(&mut Command::new(bin));
            let dir = std::env::temp_dir()
                .join(format!("kuna-castarith-enum-rt-{}-{build}-{arm}", std::process::id()));
            std::fs::create_dir_all(&dir).unwrap();
            let c = dir.join("rt.c");
            let exe = dir.join("rt");
            std::fs::write(
                &c,
                format!("#include <stdio.h>\n#define KUNA_RT\n{}\n{prelude}\n{body}\n{main}", types.concat()),
            )
            .unwrap();
            let cc = Command::new("cc")
                .args(["-std=gnu11", "-w", "-o", exe.to_str().unwrap(), c.to_str().unwrap()])
                .output()
                .expect("spawn cc");
            assert!(
                cc.status.success(),
                "{build} castarith {arm}: the printed functions did not compile:\n{}\n{body}",
                String::from_utf8_lossy(&cc.stderr)
            );
            let got = process::required_output(&mut Command::new(&exe));
            let _ = std::fs::remove_dir_all(&dir);
            assert_eq!(
                String::from_utf8_lossy(&got.stdout),
                String::from_utf8_lossy(&expected.stdout),
                "{build} castarith {arm}: the printed C computes something else\n{body}"
            );
        }
    }
}

/// `globalref`: a constant address used as a pointer prints as the global it
/// names (`put(&dat_30004070)`), and the project header declares it at the
/// type the function uses it at (`extern struct_0 dat_30004070;`). The round
/// trip exports the fixture both ways, compiles each witness caller exactly as
/// printed against the export's own header, links it with every `dat_<addr>`
/// placed at `<addr>` and the fixture's data mapped where the binary keeps it,
/// and runs it: both arms must print what the binary prints. The witnesses
/// cover a record, two scalars, a table and its one-past-the-end, a `void *`
/// libc argument, a pointer compare and a `char *` that is not a string; four
/// controls keep the cast (storage also read directly at another width or
/// type, twice, and a value also ordered or divided as a number, twice).
#[test]
fn a_constant_address_named_as_a_global_round_trips_through_the_printed_c() {
    check_globalref_round_trip(cfg!(all(target_os = "linux", target_arch = "x86_64")));
}

#[test]
fn globalref_spellings_are_checked_without_native_execution() {
    check_globalref_round_trip(false);
}

fn check_globalref_round_trip(run_native: bool) {
    let fixtures = repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures");
    let bin = fixtures.join("globalref_x86_64");
    let sp = specs();
    let witnesses = [
        "w_struct", "w_scalar", "w_range", "w_buffer", "w_compare", "w_glyph", "w_direct", "w_numeric", "w_width",
        "w_count",
    ];
    let arms: [(&str, &[&str], &[&str]); 2] = [
        (
            "on",
            &[
                "return put(&dat_30004070) + 1;",
                "return setbits(&dat_30004090,&dat_30004098) + 1;",
                "return sum(&dat_30002020,&dat_30002030) + 1;",
                "memset(&dat_300040c0,0x78,8);",
                "a0 == &dat_30004060",
                "return strlen(&dat_30002034) + 1;",
                "return put((struct_0 *)0x30004060) + v1;",
                "setbits((unsigned int *)0x30004090,&dat_30004098);",
                "return dat_300040a0 + strlen((char *)0x300040a0);",
                "memset((void *)0x300040c0,0,4);",
            ],
            &["extern struct_0 dat_30004070;", "extern unsigned int dat_30004090;", "extern int dat_30002030;"],
        ),
        (
            "off",
            &[
                "return put((struct_0 *)0x30004070) + 1;",
                "return setbits((unsigned int *)0x30004090,(long *)0x30004098) + 1;",
                "memset((void *)0x300040c0,0x78,8);",
                "a0 == (long *)0x30004060",
            ],
            &[],
        ),
    ];
    let expected = run_native.then(|| {
        let output = process::required_output(&mut Command::new(&bin));
        let text = String::from_utf8_lossy(&output.stdout).trim().to_string();
        assert_eq!(text, "210 21 27 121 1 0 4 229 1 -8608764254683430263 2", "the fixture itself");
        text
    });
    if !run_native {
        eprintln!("globalref round trip: native execution disabled; checking all spellings");
    }
    let dir = common::scratch_file("globalref-round-trip", "dir");
    std::fs::create_dir(&dir).unwrap();
    let harness = dir.join("main.c");
    std::fs::write(&harness, GLOBALREF_HARNESS.replace("@FIXTURE@", bin.to_str().unwrap())).unwrap();
    for (arm, want, decls) in arms {
        let out = dir.join(arm);
        let (_, stderr, ok) = run_kuna(&[
            "decompile-project",
            bin.to_str().unwrap(),
            "-o",
            out.to_str().unwrap(),
            "--sleighpath",
            sp.as_str(),
            "--option",
            "globalref",
            arm,
        ]);
        assert!(ok, "kuna decompile-project failed: {stderr}");
        let header = std::fs::read_to_string(out.join("globalref_x86_64.h")).unwrap();
        let code = std::fs::read_to_string(out.join("globalref_x86_64.c")).unwrap();
        for w in want {
            assert!(code.contains(w), "{arm}: missing `{w}`:\n{code}");
        }
        for d in decls {
            assert!(header.contains(d), "{arm}: the header does not declare `{d}`:\n{header}");
        }
        if arm == "off" {
            assert!(!header.contains("globals the code names by address"), "{header}");
        }
        let mut printed = String::from("#include \"globalref_x86_64.h\"\n");
        for w in witnesses {
            let head = format!("// Function: {w} @ ");
            let at = code.find(&head).unwrap_or_else(|| panic!("{arm}: no `{w}` in the export"));
            let end = code[at + head.len()..].find("// Function: ").map_or(code.len(), |e| at + head.len() + e);
            printed.push_str(&code[at..end]);
        }
        let mut names: Vec<String> = Vec::new();
        for (i, _) in printed.match_indices("dat_") {
            let hex: String = printed[i + 4..].chars().take_while(|c| c.is_ascii_hexdigit()).collect();
            let name = format!("dat_{hex}");
            if !hex.is_empty() && !names.contains(&name) {
                names.push(name);
            }
        }
        let undeclared: String = names
            .iter()
            .filter(|n| !header.contains(&format!(" {n};")))
            .map(|n| match n.as_str() {
                "dat_300040c3" => format!("extern char {n};\n"),
                _ => format!("extern long {n};\n"),
            })
            .collect();
        printed.insert_str(printed.find('\n').unwrap() + 1, &undeclared);
        std::fs::write(out.join("printed.c"), &printed).unwrap();
        let Some(expected) = &expected else { continue };
        for cc in ["gcc", "clang"] {
            if process::optional_output(Command::new(cc).arg("--version")).is_none() {
                eprintln!("globalref round trip: no `{cc}`");
                continue;
            }
            let exe = out.join(format!("rt-{cc}"));
            let mut args: Vec<String> = [
                "-std=gnu11",
                "-w",
                "-O0",
                "-fno-builtin",
                "-no-pie",
                "-DGLOBALREF_CALLEES_ONLY",
                "-o",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect();
            args.push(exe.to_str().unwrap().to_string());
            args.push(harness.to_str().unwrap().to_string());
            args.push(out.join("printed.c").to_str().unwrap().to_string());
            args.push(fixtures.join("globalref_x86_64.c").to_str().unwrap().to_string());
            for n in &names {
                args.push(format!("-Wl,--defsym,{n}=0x{}", &n[4..]));
            }
            let built = Command::new(cc).args(&args).current_dir(&out).output().expect("spawn cc");
            assert!(
                built.status.success(),
                "{arm}/{cc}: the printed callers did not compile:\n{}\n{printed}",
                String::from_utf8_lossy(&built.stderr)
            );
            let run = process::required_output(&mut Command::new(&exe));
            let got = String::from_utf8_lossy(&run.stdout).trim().to_string();
            assert_eq!(got, *expected, "{arm}/{cc}: the printed callers compute something else:\n{printed}");
        }
    }
    let _ = std::fs::remove_dir_all(&dir);
}

/// The `globalref` round trip's `main`: map the fixture's non-executable load
/// segments at their own addresses, then call the printed witnesses in the
/// order the fixture's own `main` does.
const GLOBALREF_HARNESS: &str = r#"#include <elf.h>
#include <fcntl.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/mman.h>
#include <unistd.h>
long w_struct(void); long w_scalar(void); long w_range(void); int w_buffer(void);
unsigned long w_compare(void *); long w_glyph(void); long w_direct(void); _Bool w_numeric(unsigned long);
long w_width(void); unsigned long w_count(unsigned long);
int main(void) {
  int fd = open("@FIXTURE@", O_RDONLY);
  Elf64_Ehdr eh; pread(fd, &eh, sizeof eh, 0);
  for (int i = 0; i < eh.e_phnum; i++) {
    Elf64_Phdr ph; pread(fd, &ph, sizeof ph, eh.e_phoff + i * sizeof ph);
    if (ph.p_type != PT_LOAD || (ph.p_flags & PF_X)) continue;
    unsigned long lo = ph.p_vaddr & ~0xfffUL, hi = (ph.p_vaddr + ph.p_memsz + 0xfff) & ~0xfffUL;
    if (mmap((void *)lo, hi - lo, PROT_READ | PROT_WRITE, MAP_PRIVATE | MAP_ANONYMOUS | MAP_FIXED_NOREPLACE, -1, 0) != (void *)lo) return 2;
    pread(fd, (void *)ph.p_vaddr, ph.p_filesz, ph.p_offset);
  }
  long a = w_struct(); long b = w_scalar(); long c = w_range(); int d = w_buffer();
  int e = (int)w_compare((void *)0x30004060); int f = (int)w_compare((void *)0x30004070);
  long g = w_glyph(); long h = w_direct(); long i = w_numeric(1); long j = w_width();
  long k = (long)w_count(0x7fffffff);
  printf("%ld %ld %ld %d %d %d %ld %ld %ld %ld %ld\n", a, b, c, d, e, f, g, h, i, j, k);
  return 0;
}
"#;

/// A call whose result meets a comparison through a non-short-circuit `&` is
/// always made by the binary.  `foldcallret` used to fold it into the right-hand
/// operand of the `&&`/`||` the printer emits, `if (a0 <= 5 || tick(a0))`, so
/// the printed C skipped it whenever the left-hand side decided (GH-684).  The
/// round trip compiles `w1f` and `w4f` as printed and counts the calls: each
/// fixture prints `2 0` (`calls`, `gflag`) for an argument of 1.  clang -O0 puts
/// the call on the left, where it is always evaluated and still folds.
#[test]
fn a_call_in_a_short_circuit_operand_round_trips_through_the_printed_c() {
    let sp = specs();
    let fixtures: [(&str, &[&str], &[&str]); 3] = [
        (
            "foldcallret_sc_gcc_O0_x86_64",
            &["v1 = tick(a0);", "if (a0 <= 5 || v1)", "gflag = (unsigned int)(5 < a0 && !v1);"],
            &["|| tick(", "&& !tick("],
        ),
        (
            "foldcallret_sc_clang_O0_x86_64",
            &["if (tick(a0) || a0 <= 5)", "gflag = (unsigned int)(!tick(a0) && 5 < a0);"],
            &[],
        ),
        (
            "foldcallret_sc_clang_O2_x86_64",
            &["v1 = tick(a0);", "gflag = (unsigned int)(6 <= a0 && !v1);"],
            &["&& !tick("],
        ),
    ];
    for (name, want, never) in fixtures {
        let bin = repo_root()
            .join("decompiler/crates/kuna-analysis/tests/fixtures")
            .join(name)
            .to_str()
            .unwrap()
            .to_string();
        let (stdout, stderr, ok) =
            run_kuna(&["decompile-all", bin.as_str(), "--functions", "w1f,w4f", "--sleighpath", sp.as_str()]);
        assert!(ok, "kuna decompile-all failed on {name}: {stderr}");
        for w in want {
            assert!(stdout.contains(w), "{name}: missing `{w}`:\n{stdout}");
        }
        for n in never {
            assert!(!stdout.contains(n), "{name}: the call was folded into a right-hand operand (`{n}`):\n{stdout}");
        }

        if process::optional_output(Command::new("cc").arg("--version")).is_none() {
            eprintln!("foldcallret short-circuit round trip: no `cc`, spelling checked only");
            continue;
        }
        let dir = std::env::temp_dir().join(format!("kuna-foldcallret-sc-{}-{name}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let src = dir.join("rt.c");
        let exe = dir.join("rt");
        std::fs::write(
            &src,
            format!(
                "#include <stdio.h>\nint calls;\nint gflag;\nint tick(int x) {{ calls++; return x - 3; }}\n{stdout}\n\
                 int main(void) {{\n  w1f(1);\n  w4f(1);\n  printf(\"%d %d\\n\", calls, gflag);\n  return 0;\n}}\n"
            ),
        )
        .unwrap();
        let cc = Command::new("cc")
            .args(["-std=gnu11", "-w", "-o", exe.to_str().unwrap(), src.to_str().unwrap()])
            .output()
            .expect("spawn cc");
        assert!(cc.status.success(), "{name}: the printed C did not compile:\n{}", String::from_utf8_lossy(&cc.stderr));
        let run = process::required_output(&mut Command::new(&exe));
        let got = String::from_utf8_lossy(&run.stdout).trim().to_string();
        let _ = std::fs::remove_dir_all(&dir);
        assert_eq!(got, "2 0", "{name}: the printed C makes a different number of calls than the binary:\n{stdout}");
    }
}

/// A load whose bytes a later store overwrites keeps its own statement ahead of
/// the store.  Whether a load may print after a store was decided by comparing
/// the two pointers alone, and one base plus two different constants counted as
/// two objects whatever the access widths: `inside` (a 4-byte read at `p+7`, a
/// byte store at `p+8`) printed the store first and then `return *(unsigned int
/// *)(a0 + 7);`, which returns the new byte.  The parameter is declared
/// `void *` by `ptrfromuse`, so the accesses through it carry a `(long)` cast.  `below` stores below the read,
/// `indexed` one element into an 8-byte read; `after`, `before` and `next` store
/// next to the read bytes, not into them, and keep the read folded.  The round
/// trip compiles the six printed functions and checks each against its source.
#[test]
fn a_load_is_not_printed_after_a_store_into_its_bytes() {
    let bin = repo_root()
        .join("decompiler/crates/kuna-analysis/tests/fixtures/aliasoverlap_x86_64")
        .to_str()
        .unwrap()
        .to_string();
    let sp = specs();
    let (stdout, stderr, ok) = run_kuna(&[
        "decompile-all",
        bin.as_str(),
        "--functions",
        "inside,below,after,before,indexed,next",
        "--sleighpath",
        sp.as_str(),
    ]);
    assert!(ok, "kuna decompile-all failed: {stderr}");
    let body = |name: &str| -> String {
        let at = stdout.find(&format!("// Function: {name} @")).unwrap_or_else(|| panic!("no {name}:\n{stdout}"));
        let rest = &stdout[at + 1..];
        rest[..rest.find("// Function:").unwrap_or(rest.len())].to_string()
    };
    let ordered = [
        ("inside", "*(unsigned int *)((long)a0 + 7);", "((char *)a0)[8] = "),
        ("below", "*(unsigned int *)((long)a0 + 7);", "*(unsigned int *)((long)a0 + 5) = "),
        ("indexed", "*(unsigned long *)(a0 + a1 * 4);", "*(unsigned int *)(a0 + 4 + a1 * 4) = "),
        ("after", "((char *)a0)[0xb] = ", "return *(unsigned int *)((long)a0 + 7);"),
        ("before", "((char *)a0)[6] = ", "return *(unsigned int *)((long)a0 + 7);"),
        ("next", "a0[a1 + 1] = ", "return a0[a1];"),
    ];
    for (name, first, second) in ordered {
        let b = body(name);
        let (i, j) = (b.find(first), b.find(second));
        assert!(i.is_some() && j.is_some() && i < j, "{name}: `{first}` must print before `{second}`:\n{b}");
    }

    if process::optional_output(Command::new("cc").arg("--version")).is_none() {
        eprintln!("aliasoverlap round trip: no `cc`, order checked only");
        return;
    }
    let dir = std::env::temp_dir().join(format!("kuna-aliasoverlap-rt-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let src = dir.join("rt.c");
    let exe = dir.join("rt");
    let harness = r#"#include <stdio.h>
#include <string.h>
@PRINTED@
static unsigned ref_inside(unsigned char *p, unsigned w) { unsigned v; memcpy(&v, p + 7, 4); p[8] = w; return v; }
static unsigned ref_below(unsigned char *p, unsigned w) { unsigned v; memcpy(&v, p + 7, 4); memcpy(p + 5, &w, 4); return v; }
static unsigned ref_after(unsigned char *p, unsigned w) { unsigned v; memcpy(&v, p + 7, 4); p[11] = w; return v; }
static unsigned ref_before(unsigned char *p, unsigned w) { unsigned v; memcpy(&v, p + 7, 4); p[6] = w; return v; }
static unsigned long ref_indexed(unsigned *a, long i, unsigned w) { unsigned long v; memcpy(&v, a + i, 8); a[i + 1] = w; return v; }
static unsigned ref_next(unsigned *a, long i, unsigned w) { unsigned v = a[i]; a[i + 1] = w; return v; }
static void fill(void *p, void *q, int n) {
  for (int i = 0; i < n; i++) ((unsigned char *)p)[i] = ((unsigned char *)q)[i] = (unsigned char)(i * 37 + 0x81);
}
int main(void) {
  int bad = 0;
  unsigned char b[32], r[32];
#define CHECK(T, f, ...) \
  fill(b, r, 32); \
  { T got = ((T (*)())f)((void *)b, __VA_ARGS__), want = ref_##f((void *)r, __VA_ARGS__); \
    if (got != want || memcmp(b, r, 32)) { printf(#f " %lx != %lx\n", (unsigned long)got, (unsigned long)want); bad++; } }
  CHECK(unsigned, inside, 0x5au)
  CHECK(unsigned, below, 0x5au)
  CHECK(unsigned, after, 0x5au)
  CHECK(unsigned, before, 0x5au)
  CHECK(unsigned long, indexed, 2L, 0x5au)
  CHECK(unsigned, next, 4L, 0x5au)
  printf("%d\n", bad);
  return 0;
}
"#;
    std::fs::write(&src, harness.replace("@PRINTED@", &stdout)).unwrap();
    let cc = Command::new("cc")
        .args(["-std=gnu11", "-w", "-o", exe.to_str().unwrap(), src.to_str().unwrap()])
        .output()
        .expect("spawn cc");
    assert!(cc.status.success(), "the printed functions did not compile:\n{}", String::from_utf8_lossy(&cc.stderr));
    let run = process::required_output(&mut Command::new(&exe));
    let got = String::from_utf8_lossy(&run.stdout).to_string();
    let _ = std::fs::remove_dir_all(&dir);
    assert_eq!(got.lines().last(), Some("0"), "the printed functions read different bytes:\n{got}\n{stdout}");
}

/// A typed read that spans several fields is not split at a COPY that lies past
/// a store or a call.  `SplitDatatype::split_load` built the per-field reads at
/// the read's lone COPY into the return register, so `intospan` printed
/// `s->c9 = (char)w;` and then read `s->c7`..`s->c10`, returning the new byte;
/// `otherptr` (called with `t == s`) and `acrosscall` (whose `sink` bumps
/// `s->c8`) did the same.  `plain`, with nothing between the read and its COPY,
/// still splits.  The round trip rewrites the printed partial writes
/// (`v1._0_1_ = ...`) as byte stores, compiles the four functions and checks
/// each against its source.
#[test]
fn a_split_load_is_not_moved_past_a_store_or_a_call() {
    let bin = repo_root()
        .join("decompiler/crates/kuna-analysis/tests/fixtures/splitload_x86_64")
        .to_str()
        .unwrap()
        .to_string();
    let sp = specs();
    let (stdout, stderr, ok) = run_kuna(&[
        "decompile-all",
        bin.as_str(),
        "--functions",
        "intospan,otherptr,acrosscall,plain",
        "--sleighpath",
        sp.as_str(),
    ]);
    assert!(ok, "kuna decompile-all failed: {stderr}");
    let body = |name: &str| -> String {
        let at = stdout.find(&format!("// Function: {name} @")).unwrap_or_else(|| panic!("no {name}:\n{stdout}"));
        let rest = &stdout[at + 1..];
        rest[..rest.find("// Function:").unwrap_or(rest.len())].to_string()
    };
    let read = "v1 = *(unsigned int *)&s->c7;";
    for (name, after) in [("intospan", "s->c9 = (char)w;"), ("otherptr", "t->c9 = (char)w;"), ("acrosscall", "sink(s);")] {
        let b = body(name);
        let (i, j) = (b.find(read), b.find(after));
        assert!(i.is_some() && j.is_some() && i < j, "{name}: `{read}` must print before `{after}`:\n{b}");
    }
    assert!(body("plain").contains("v1._0_1_ = s->c7;"), "plain no longer splits:\n{}", body("plain"));

    if process::optional_output(Command::new("cc").arg("--version")).is_none() {
        eprintln!("splitload round trip: no `cc`, order checked only");
        return;
    }
    let partial = regex::Regex::new(r"(\w+)\._(\d+)_(\d+)_ = ").unwrap();
    let printed = partial.replace_all(&stdout, |c: &regex::Captures| {
        let ty = match &c[3] {
            "1" => "unsigned char",
            "2" => "unsigned short",
            "4" => "unsigned int",
            _ => "unsigned long",
        };
        format!("*({ty} *)((char *)&{} + {}) = ", &c[1], &c[2])
    });
    let dir = std::env::temp_dir().join(format!("kuna-splitload-rt-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let src = dir.join("rt.c");
    let exe = dir.join("rt");
    let harness = r#"#include <stdio.h>
#include <string.h>
typedef struct S { int i0; char c4, c5, c6, c7, c8, c9, c10, c11; } S;
void sink(S *s) { s->c8 = (char)(s->c8 + 0x11); }
@PRINTED@
static unsigned ref_intospan(S *s, int w) { unsigned v; memcpy(&v, &s->c7, 4); s->c9 = w; return v; }
static unsigned ref_otherptr(S *s, S *t, int w) { unsigned v; memcpy(&v, &s->c7, 4); t->c9 = w; return v; }
static unsigned ref_acrosscall(S *s) { unsigned v; memcpy(&v, &s->c7, 4); sink(s); return v; }
static unsigned ref_plain(S *s) { unsigned v; memcpy(&v, &s->c7, 4); return v; }
static void fill(S *p, S *q) {
  for (int i = 0; i < (int)sizeof(S); i++) ((unsigned char *)p)[i] = ((unsigned char *)q)[i] = (unsigned char)(i * 37 + 0x81);
}
int main(void) {
  int bad = 0;
  S a, r;
  unsigned got, want;
#define CHECK(name, call, ref) \
  fill(&a, &r); got = call; want = ref; \
  if (got != want || memcmp(&a, &r, sizeof a)) { printf(name " %08x != %08x\n", got, want); bad++; }
  CHECK("intospan", intospan(&a, 0x5a), ref_intospan(&r, 0x5a))
  CHECK("otherptr", otherptr(&a, &a, 0x5a), ref_otherptr(&r, &r, 0x5a))
  CHECK("acrosscall", acrosscall(&a), ref_acrosscall(&r))
  CHECK("plain", plain(&a), ref_plain(&r))
  printf("%d\n", bad);
  return 0;
}
"#;
    std::fs::write(&src, harness.replace("@PRINTED@", &printed)).unwrap();
    let cc = Command::new("cc")
        .args(["-std=gnu11", "-w", "-o", exe.to_str().unwrap(), src.to_str().unwrap()])
        .output()
        .expect("spawn cc");
    assert!(cc.status.success(), "the printed functions did not compile:\n{}", String::from_utf8_lossy(&cc.stderr));
    let run = process::required_output(&mut Command::new(&exe));
    let got = String::from_utf8_lossy(&run.stdout).to_string();
    let _ = std::fs::remove_dir_all(&dir);
    assert_eq!(got.lines().last(), Some("0"), "the printed functions read different bytes:\n{got}\n{printed}");
}

/// A call's own return-address push is part of the call under `callpush`.
/// Every function between the `tested` markers of `callpush_x86_64.c` calls
/// `alloca`, so its stack pointer is the entry value minus a run-time size and
/// each later call stored its return address through that pointer as a
/// statement, `*(unsigned long *)&v9[v5 + -8] = 0x14e7;`.  For the gcc -O0,
/// gcc -O2, clang -O0 and clang -O2 builds, with the option on and off: off
/// prints a store of every listed return address (each is the address after a
/// `call` in the tested functions, from `objdump -d`), on prints none of them,
/// both arms make the same calls, `stacked` keeps the same number of other
/// stores (the stack-passed arguments of `spread` and the stack probes), and the
/// push of `call 1f` in `pc_here`, whose destination is the stored address and
/// which the function reads back, prints the same in both.  The printed alloca
/// frames are not compilable C in either arm (the alloca itself prints as
/// stack-pointer arithmetic, and `spread`'s stack arguments do not reach the
/// call), so the round trip here is the statement set, not a run: the removed
/// store writes the slot below the stack pointer that only the callee's `ret`
/// reads.
#[test]
fn a_calls_own_return_address_push_is_part_of_the_call() {
    let fx = repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures");
    let src = std::fs::read_to_string(fx.join("callpush_x86_64.c")).unwrap();
    let tested_src = src.split("/* tested */").nth(1).unwrap().split("/* main */").next().unwrap();
    let tested: Vec<&str> = tested_src
        .lines()
        .filter_map(|l| l.strip_prefix("KEEP long "))
        .filter_map(|l| l.split('(').next())
        .collect();
    assert_eq!(tested, ["joined", "stacked", "twice", "pc_here"]);
    let builds: [(&str, &[&str], Option<&str>); 4] = [
        (
            "gcc_O0",
            &["0x139d", "0x13d1", "0x13ee", "0x14e7", "0x1557", "0x1637", "0x165c", "0x1708", "0x172d"],
            Some("0x1802"),
        ),
        (
            "gcc_O2",
            &["0x1397", "0x13b1", "0x13bd", "0x148b", "0x14c4", "0x1583", "0x1593", "0x15f3", "0x1602"],
            Some("0x16b0"),
        ),
        (
            "clang_O0",
            &[
                "0x1249", "0x1256", "0x1289", "0x12b2", "0x12c3", "0x131b", "0x1390", "0x13d6", "0x13ef",
                "0x1427", "0x144d",
            ],
            None,
        ),
        (
            "clang_O2",
            &[
                "0x11f9", "0x1204", "0x1231", "0x124d", "0x1259", "0x12bb", "0x12f7", "0x133e", "0x134e",
                "0x1378",
            ],
            Some("0x13c5"),
        ),
    ];
    let sp = specs();
    let calls = |body: &str| -> Vec<String> {
        let mut v: Vec<String> = ["weigh(", "spread(", "memcpy(", "memset(", "strlen("]
            .iter()
            .flat_map(|c| std::iter::repeat(c.to_string()).take(body.matches(c).count()))
            .collect();
        v.sort();
        v
    };
    let stores = |body: &str, pushes: &[&str]| -> usize {
        body.lines()
            .map(str::trim)
            .filter(|l| (l.starts_with("*(") || l.starts_with("((")) && l.contains(" = "))
            .filter(|l| !pushes.iter().any(|p| l.ends_with(&format!("= {p};"))))
            .count()
    };
    for (build, pushes, call_next) in builds {
        let bin = fx.join(format!("callpush_{build}_x86_64"));
        let mut arms: Vec<String> = Vec::new();
        for arm in ["off", "on"] {
            let args = ["decompile-all", bin.to_str().unwrap(), "--sleighpath", sp.as_str(), "--option", "callpush", arm];
            let (stdout, stderr, ok) = run_kuna(&args);
            assert!(ok, "kuna decompile-all failed: {stderr}");
            let mut body = String::new();
            for part in stdout.split("// Function: ").skip(1) {
                if tested.contains(&part.split(' ').next().unwrap_or("")) {
                    body.push_str(part);
                }
            }
            for name in &tested {
                assert!(body.starts_with(name) || body.contains(&format!("\n{name} ")), "{build} {arm}: no `{name}`\n{stdout}");
            }
            arms.push(body);
        }
        let (off, on) = (&arms[0], &arms[1]);
        for p in pushes {
            let stored = format!("= {p};");
            assert!(off.contains(&stored), "{build} off: the push of return address {p} is not printed\n{off}");
            assert!(!on.contains(&stored), "{build} on: the push of return address {p} is still printed\n{on}");
        }
        if let Some(p) = call_next {
            let stored = format!("= {p};");
            assert!(off.contains(&stored) && on.contains(&stored), "{build}: the push `call 1f` reads back is lost\n{on}");
        }
        assert_eq!(calls(off), calls(on), "{build}: a call was lost or gained\n{off}\n{on}");
        let stacked = |b: &str| b.split("stacked").nth(1).unwrap().split("// Function: ").next().unwrap().to_string();
        assert_eq!(
            stores(&stacked(off), pushes),
            stores(&stacked(on), pushes),
            "{build}: a store other than a push changed in `stacked`\n{off}\n{on}"
        );
    }
}

/// The printed text of the functions `names` in a `decompile-all` listing, in
/// listing order, each from its `// Function:` header to the next.
fn callrettype_functions(listing: &str, names: &[&str]) -> String {
    let mut out = String::new();
    for part in listing.split("// Function: ").skip(1) {
        let name = part.split_whitespace().next().unwrap_or("");
        if names.contains(&name) {
            out.push_str("// Function: ");
            out.push_str(part);
        }
    }
    out
}

/// Every call in `listing`, keyed by the function printing it: the callee's
/// name and how many arguments it is passed, sorted.  A header counts as a
/// call to itself, so a function whose own parameter list moves shows too.
fn callrettype_calls(listing: &str) -> std::collections::BTreeMap<String, Vec<(String, usize)>> {
    const NOT_CALLS: &[&str] = &["if", "while", "for", "switch", "return", "sizeof"];
    let mut out = std::collections::BTreeMap::new();
    for part in listing.split("// Function: ").skip(1) {
        let name = part.split_whitespace().next().unwrap_or("").to_string();
        let b = part.as_bytes();
        let mut calls = Vec::new();
        let mut i = 0;
        while i < b.len() {
            if b[i].is_ascii_alphabetic() || b[i] == b'_' {
                let start = i;
                while i < b.len() && (b[i].is_ascii_alphanumeric() || b[i] == b'_') {
                    i += 1;
                }
                let word = &part[start..i];
                if i < b.len() && b[i] == b'(' && !NOT_CALLS.contains(&word) {
                    let (mut depth, mut args, mut j, mut empty) = (0i32, 1usize, i, true);
                    while j < b.len() {
                        match b[j] {
                            b'(' => depth += 1,
                            b')' => {
                                depth -= 1;
                                if depth == 0 {
                                    break;
                                }
                            }
                            b',' if depth == 1 => args += 1,
                            b' ' | b'\n' => {}
                            _ if depth >= 1 => empty = false,
                            _ => {}
                        }
                        j += 1;
                    }
                    calls.push((word.to_string(), if empty { 0 } else { args }));
                }
                continue;
            }
            i += 1;
        }
        calls.sort();
        out.insert(name, calls);
    }
    out
}

/// (kuna `callrettype`) A call's result takes the return type its callee's own
/// recovery gave it earlier in the same run, so `(char *)skip_blanks(a0)` under
/// a `char * skip_blanks(char *a0)` declaration prints without the conversion,
/// and the same for a `char *` found by `strchr`, a `FILE *` handed back from a
/// global and a `long` compared signed.  The controls keep their casts: a callee
/// whose unsigned result shares a variable with `strcmp`'s signed one, the
/// `unsigned long` shift a `long` result is read through, a callee recovered
/// `void`, which states nothing, and a `long *` result the merge ties into one
/// variable with the `-1` and the count the function returns (`cached`, which
/// must stay `unsigned long` rather than turn into a pointer), and two callers
/// that zero-extend a callee's `short` and `int` result in place before
/// returning it (`unsigned short use_s16_as_u`, `unsigned int
/// use_neg_as_unsigned`, called through their printed prototypes, where a
/// statement at the callee's sign would hand back -15536 and
/// 18446744073709551613), and two that keep an `int` result in an `unsigned
/// int` and hand it back as `unsigned long` through a reload or a move from
/// the register it was kept in across another call (`keep_widened`,
/// `keep_across`, read whole and shifted, which prints them `unsigned long` as
/// declared, where `int` would print 9223372036854775806), and one that passes such a result to an `unsigned
/// long` parameter (`pass_widened`, where an `int` argument would hand `halve`
/// a sign-extended value and print 9223372036854775807).  Every fixture is
/// decompiled with the option
/// off and on; every printed function compiled with gcc and clang at -O0 and
/// -O2 must print what the binary prints, and no call in the whole listing may
/// gain or lose an argument or a result.
#[test]
fn a_call_result_typed_by_its_callee_round_trips_through_the_printed_c() {
    const ALL: &[&str] = &[
        "skip_blanks", "count_upper", "upper_after_blanks", "upper_of_rest", "first_of", "first_char",
        "signed_delta", "is_behind", "clamp_delta", "hash_of", "pick", "after_colon", "fallback_name", "name_len",
        "pick_stream", "stream_no", "s16", "use_s16_as_u", "neg32", "use_neg_as_unsigned", "widen_signed", "tick",
        "keep_widened", "keep_across", "halve", "pass_widened", "mark", "marked_len",
    ];
    const O2: &[&str] = &[
        "first_of", "first_char", "signed_delta", "is_behind", "clamp_delta", "hash_of", "pick", "after_colon",
        "fallback_name", "name_len", "pick_stream", "stream_no", "s16", "use_s16_as_u", "neg32",
        "use_neg_as_unsigned", "widen_signed", "tick", "keep_widened", "keep_across", "halve", "pass_widened", "mark",
        "marked_len",
    ];
    const WANT: &str =
        "202 205 2\n104 -1\n1 0 -2\n1 -1 0\n7 6 6\n10 21 9\n50000 4294967293 -12\n2147483646 2147483646 2147483647\n";
    const MAIN: &str = r#"
int main(void) {
  char buf[] = "  AbC";
  char buf2[] = "xyzw";
  char buf3[] = "Hi!Hi!!";
  printf("%ld %ld %ld\n", (long)upper_after_blanks((long)buf), (long)upper_of_rest(buf),
         (long)((char *)skip_blanks(buf) - buf));
  printf("%d %d\n", (int)first_char((unsigned long)(buf2 + 1), 3L) - 17, (int)first_char((unsigned long)buf2, 0L));
  printf("%d %d %ld\n", (int)is_behind(3L, 9L), (int)is_behind(9L, 3L), (long)clamp_delta(1L, 9L));
  printf("%d %d %d\n", (int)pick("ab", "cd", 1), (int)pick("ab", "cd", 0), (int)pick("ab", "ab", 0));
  printf("%ld %ld %ld\n", (long)name_len(NULL), (long)name_len("key:value"), (long)name_len("plain"));
  printf("%d %d %ld\n", (int)stream_no(0), (int)stream_no(1), (long)marked_len(buf3));
  printf("%ld %lu %ld\n", (long)use_s16_as_u(50), (unsigned long)use_neg_as_unsigned(1), (long)widen_signed(4));
  printf("%lu %lu %lu\n", (unsigned long)keep_widened(1) >> 1, (unsigned long)keep_across(1) >> 1,
         (unsigned long)pass_widened(1));
  return 0;
}
"#;
    const O2_WANT: &str =
        "104 -1\n1 0 -2\n1 -1 0\n7 6 6\n10 21 9\n50000 4294967293 -12\n2147483646 2147483646 2147483647\n";
    const O2_MAIN: &str = r#"
int main(void) {
  char buf2[] = "xyzw";
  char buf3[] = "Hi!Hi!!";
  printf("%d %d\n", (int)first_char((unsigned long)(buf2 + 1), 3L) - 17, (int)first_char((unsigned long)buf2, 0L));
  printf("%d %d %ld\n", (int)is_behind(3L, 9L), (int)is_behind(9L, 3L), (long)clamp_delta(1L, 9L));
  printf("%d %d %d\n", (int)pick("ab", "cd", 1), (int)pick("ab", "cd", 0), (int)pick("ab", "ab", 0));
  printf("%ld %ld %ld\n", (long)name_len(NULL), (long)name_len("key:value"), (long)name_len("plain"));
  printf("%d %d %ld\n", (int)stream_no(0), (int)stream_no(1), (long)marked_len(buf3));
  printf("%ld %lu %ld\n", (long)use_s16_as_u(50), (unsigned long)use_neg_as_unsigned(1), (long)widen_signed(4));
  printf("%lu %lu %lu\n", (unsigned long)keep_widened(1) >> 1, (unsigned long)keep_across(1) >> 1,
         (unsigned long)pass_widened(1));
  return 0;
}
"#;
    // (fixture, printed functions, main, output, what option off prints and what
    // option on prints instead, what both print)
    type Case<'a> = (&'a str, &'a [&'a str], &'a str, &'a str, &'a [(&'a str, &'a str)], &'a [&'a str]);
    let cases: [Case; 3] = [
        (
            "callrettype_gcc_O0_x86_64",
            ALL,
            MAIN,
            WANT,
            &[
                ("v1 = (char *)skip_blanks(a0);", "v1 = skip_blanks(a0);"),
                ("v1 = (FILE *)pick_stream(a0);", "v1 = pick_stream(a0);"),
                (
                    "v1 = (a0) ? (char *)after_colon(a0) : (char *)fallback_name();",
                    "    v1 = after_colon(a0);\n  else {\n    v1 = fallback_name();",
                ),
                ("return (int)neg32(a0);", "return neg32(a0);"),
            ],
            &[
                "return (unsigned long)signed_delta(a0,a1) >> 0x3f;",
                "(unsigned int)hash_of(a0) % 7",
                "  mark(a0);\n",
                "unsigned long cached(long *a0,unsigned long a1)",
                "        v1 = 0xffffffffffffffff;",
                "    v1 = lookup((long *)*a0,a1);",
                "unsigned short use_s16_as_u(",
                "unsigned int use_neg_as_unsigned(",
                "unsigned long keep_widened(",
                "unsigned long keep_across(",
            ],
        ),
        (
            "callrettype_clang_O0_x86_64",
            ALL,
            MAIN,
            WANT,
            &[
                ("v1 = (char *)skip_blanks(a0);", "v1 = skip_blanks(a0);"),
                ("v1 = (FILE *)pick_stream(a0);", "v1 = pick_stream(a0);"),
                ("return (long)signed_delta(a0,a1) < 0;", "return signed_delta(a0,a1) < 0;"),
                (
                    "v1 = (a0) ? (char *)after_colon(a0) : (char *)fallback_name();",
                    "v1 = (a0) ? after_colon(a0) : fallback_name();",
                ),
                ("return (int)neg32(a0);", "return neg32(a0);"),
            ],
            &[
                "  mark(a0);\n",
                "unsigned short use_s16_as_u(",
                "unsigned int use_neg_as_unsigned(",
                "unsigned long keep_widened(",
                "unsigned long keep_across(",
            ],
        ),
        (
            "callrettype_gcc_O2_x86_64",
            O2,
            O2_MAIN,
            O2_WANT,
            &[
                ("v1 = (char *)after_colon(a0);", "v1 = after_colon(a0);"),
                ("v1 = (char *)fallback_name();", "v1 = fallback_name();"),
                ("v1 = (FILE *)pick_stream(a0);", "v1 = pick_stream(a0);"),
                ("return (int)neg32(a0);", "return neg32(a0);"),
            ],
            &[
                "v1 = (char *)skip_blanks(a0);",
                "return (unsigned long)signed_delta(a0,a1) >> 0x3f;",
                "unsigned short use_s16_as_u(short a0)",
                "unsigned int use_neg_as_unsigned(int a0)",
                "unsigned long keep_widened(",
                "unsigned long keep_across(",
            ],
        ),
    ];
    let sp = specs();
    let compilers: Vec<&str> = ["gcc", "clang"]
        .into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    assert!(!compilers.is_empty(), "callrettype round trip requires a C compiler");
    for (fixture, funcs, main, want, lines, kept) in cases {
        let bin = repo_root()
            .join("decompiler/crates/kuna-analysis/tests/fixtures")
            .join(fixture)
            .to_str()
            .unwrap()
            .to_string();
        let mut listings: Vec<String> = Vec::new();
        for opt in ["off", "on"] {
            let args = ["decompile-all", bin.as_str(), "--sleighpath", sp.as_str(), "--option", "callrettype", opt];
            let (stdout, stderr, ok) = run_kuna(&args);
            assert!(ok, "kuna decompile-all failed: {stderr}");
            for (off, on) in lines {
                let want = if opt == "on" { on } else { off };
                assert!(stdout.contains(want), "{fixture} option {opt} does not print `{want}`:\n{stdout}");
            }
            for k in kept {
                assert!(stdout.contains(k), "{fixture} option {opt} lost `{k}`:\n{stdout}");
            }
            let printed = callrettype_functions(&stdout, funcs);
            for cc in &compilers {
                for level in ["-O0", "-O2"] {
                    let dir = std::env::temp_dir()
                        .join(format!("kuna-callrettype-rt-{}-{fixture}-{opt}-{cc}{level}", std::process::id()));
                    std::fs::create_dir_all(&dir).unwrap();
                    let src = dir.join("rt.c");
                    let exe = dir.join("rt");
                    std::fs::write(
                        &src,
                        format!(
                            "#include <stdbool.h>\n#include <stdio.h>\n#include <string.h>\n\
                             #define stderr_ptr (&stderr)\n#define stdout_ptr (&stdout)\n\
                             #define CONCAT22(h, l) ((unsigned int)(unsigned short)(h) << 16 | (unsigned short)(l))\n\
                             char *g_fallback = \"fallback\";\nvolatile int g_ticks;\n{printed}\n{main}"
                        ),
                    )
                    .unwrap();
                    let out = Command::new(cc)
                        .args(["-std=gnu11", "-w", "-Wno-error=int-conversion", level])
                        .args(["-o", exe.to_str().unwrap(), src.to_str().unwrap()])
                        .output()
                        .expect("spawn the C compiler");
                    assert!(
                        out.status.success(),
                        "{cc} {level} rejected the printed C ({fixture}, option {opt}):\n{}",
                        String::from_utf8_lossy(&out.stderr)
                    );
                    let run = process::required_output(&mut Command::new(&exe));
                    let _ = std::fs::remove_dir_all(&dir);
                    assert_eq!(
                        String::from_utf8_lossy(&run.stdout),
                        want,
                        "{fixture} printed with option {opt} and built by {cc} {level} computes a different value:\n{printed}"
                    );
                }
            }
            listings.push(stdout);
        }
        assert_eq!(
            callrettype_calls(&listings[0]),
            callrettype_calls(&listings[1]),
            "{fixture}: option callrettype moved a call's arguments"
        );
        let reads = |listing: &str, callee: &str| {
            let call = format!("{callee}(");
            listing.lines().filter(|l| l.find(&call).is_some_and(|at| l[..at].contains('='))).count()
        };
        for header in listings.iter().flat_map(|l| l.lines()).filter(|l| l.starts_with("void ")) {
            let callee = header[5..].split('(').next().unwrap_or("").trim();
            assert_eq!(
                reads(&listings[0], callee),
                reads(&listings[1], callee),
                "{fixture}: option callrettype changed how often the result of the void function {callee} is read"
            );
        }
    }
}

/// `elemptr`: a pointer the program uses only as an array of one element type
/// is declared as that pointer, so a textbook base64 decoder reads
/// `dat_30004060[v2]`, `a0[v7]` and `v6[v8]` instead of three integer sums behind
/// casts. The round trip exports `elemptr_x86_64.c`'s gcc -O0, clang -O0 and
/// gcc -O2 builds with the option on and off, compiles the witnesses exactly as
/// printed against the export's own header with gcc and clang, links every
/// `dat_<addr>` at `<addr>` with the fixture's data mapped where the binary keeps
/// it, and runs them: both arms must print what the binary prints. The inputs
/// read bytes at and above 0x80 signed and unsigned, use them as indexes both
/// ways, index backwards from the end of an `int` array, read a `.data` table
/// of `int`s, fill a table through a global the program allocated, and return
/// an allocated buffer to the caller. Two controls keep their integer form: a
/// record walked by a stride, and one pointer read at two widths. A second line
/// reads tables whose elements have the top bit set: a `unsigned short` and an
/// `unsigned int` element returned to a caller that widens them (declared
/// signed, the callers would sign-extend; they add to the whole register, so
/// the functions return `unsigned long`, as declared), one shifted and one only compared,
/// and a byte table one function zero-extends and another sign-extends (the
/// header can declare it at one sign only, so neither indexes it). At -O2, gcc's
/// `w_rev` returns the `malloc` result it never copies out of `rax`, and kuna
/// declares it `void` in both arms (a return-recovery gap outside this option),
/// so that build's round trip does not compare the reversed string. A third
/// line copies a string into buffers bounded by a length the function compares
/// a pointer difference against (the length stays `unsigned long`, never a
/// `char *` base), and stores an `int` counter into an `unsigned` table and
/// indexes a second table with its elements (the counter stays `int`). A
/// fourth line reads a global `int *` two functions index and two others step
/// by bytes, a table one function indexes and two others name the first
/// element of, and a 2-byte field at the end of a readable page through a
/// pointer a callee reads as `unsigned int *`: no function may type the global
/// or the table (the batch's one declaration would rescale the others' byte
/// arithmetic, or make the scalar the array), and the field is read 2 bytes
/// wide (a 4-byte element read would fault). A global the header declines as
/// read at two types is compiled at the pointer that header comment lists, the
/// type the batch would commit, so a disagreement cannot hide behind `char *`.
/// A fifth line returns 8-byte elements above 2^32 from a function that loads
/// them through an address computed in the register it returns them in
/// (`w_nexttab`, coreutils `expand`'s `get_next_tab_column`), which is never
/// declared to return that address's type. A sixth line compares and hashes a
/// 15-byte table with a zero byte inside it through parameters the option
/// types `char *` (`w_chk`, `w_hash`) and through a parameter merged with it
/// (`w_pick`): the table is passed as its address, never as a string literal
/// that ends at the zero byte while the reader takes all fifteen.
#[test]
fn an_element_pointer_round_trips_through_the_printed_c() {
    check_elemptr_round_trip(cfg!(all(target_os = "linux", target_arch = "x86_64")));
}

#[test]
fn elemptr_spellings_are_checked_without_native_execution() {
    check_elemptr_round_trip(false);
}

fn check_elemptr_round_trip(run_native: bool) {
    let fixtures = repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures");
    let sp = specs();
    let witnesses = [
        "w_build", "w_decode", "w_sbytes", "w_ubytes", "w_words", "w_back", "w_sidx", "w_table", "w_rev",
        "w_record", "w_mixed", "w_wu", "w_wucall", "w_iu", "w_iucall", "w_iu2", "w_srch", "w_xu", "w_xs", "w_put",
        "w_ctr", "w_gpinit", "w_gpbump", "w_gpadv", "w_gpread", "w_tidx", "w_tfirst", "w_tset", "w_sum4", "w_hdr",
        "w_nexttab", "w_tabinit", "w_bcmp", "w_chk", "w_hsum", "w_hash", "w_pick",
    ];
    let on: &[&str] = &[
        "char * w_decode(char *a0,unsigned long a1,unsigned long *a2)",
        "long w_sbytes(char *a0,int a1)",
        "long w_ubytes(unsigned char *a0,int a1)",
        "long w_sidx(char *a0,int a1,int *a2)",
        "long w_mixed(char *a0,int a1)",
        "unsigned long w_wu(unsigned int a0)",
        "unsigned long w_iu(unsigned int a0)",
        "w_put(char *a0,unsigned long a1,char *a2)",
        "w_ctr(unsigned int *a0,",
        "long w_hsum(char *a0,long a1)",
        "long w_pick(char *a0,long a1,int a2)",
    ];
    let off: &[&str] = &["void * w_decode(long a0,unsigned long a1,unsigned long *a2)"];
    if !run_native {
        eprintln!("elemptr round trip: native execution disabled; checking all spellings");
    }
    let dir = common::scratch_file("elemptr-round-trip", "dir");
    std::fs::create_dir(&dir).unwrap();
    for build in ["gcc_O0", "clang_O0", "gcc_O2"] {
        let stem = format!("elemptr_{build}_x86_64");
        let bin = fixtures.join(&stem);
        let rev_broken = build == "gcc_O2";
        let expected = run_native.then(|| {
            let output = process::required_output(&mut Command::new(&bin));
            let mut text = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if rev_broken {
                let mut fields: Vec<&str> = text.split(' ').collect();
                fields[3] = "-";
                text = fields.join(" ");
            }
            text
        });
        for arm in ["on", "off"] {
            let harness = dir.join(format!("main-{build}-{arm}.c"));
            std::fs::write(
                &harness,
                ELEMPTR_HARNESS
                    .replace("@FIXTURE@", bin.to_str().unwrap())
                    .replace("@REV@", if rev_broken { "0" } else { "1" }),
            )
            .unwrap();
            let out = dir.join(format!("{build}-{arm}"));
            let (_, stderr, ok) = run_kuna(&[
                "decompile-project",
                bin.to_str().unwrap(),
                "-o",
                out.to_str().unwrap(),
                "--sleighpath",
                sp.as_str(),
                "--option",
                "elemptr",
                arm,
            ]);
            assert!(ok, "kuna decompile-project failed: {stderr}");
            let header = std::fs::read_to_string(out.join(format!("{stem}.h"))).unwrap();
            let code = std::fs::read_to_string(out.join(format!("{stem}.c"))).unwrap();
            for w in if arm == "on" { on } else { off } {
                assert!(code.contains(w), "{build} {arm}: missing `{w}`:\n{code}");
            }
            assert!(code.contains("long w_table(int a0)"), "{build} {arm}:\n{code}");
            assert!(!code.contains("w_put(char *a0,char *a1"), "{build} {arm}: the length is a number:\n{code}");
            assert!(!code.contains("(unsigned short)a0[1]"), "{build} {arm}: the 2-byte field is read 4 wide:\n{code}");
            assert!(
                code.contains("unsigned long w_nexttab(unsigned long a0,"),
                "{build} {arm}: the returned element is a number:\n{code}"
            );
            assert!(
                !code.contains("\"0!0"),
                "{build} {arm}: the table with a zero byte inside is a string literal:\n{code}"
            );
            if arm == "on" {
                assert!(header.contains("extern unsigned char dat_"), "{build}: the encoding table:\n{header}");
                assert!(header.contains("extern int dat_"), "{build}: the weights table:\n{header}");
                assert!(!code.contains("(long)v6 + (long)v8"), "{build}: the decoded buffer:\n{code}");
                assert!(header.contains("extern unsigned short dat_"), "{build}: the word table:\n{header}");
                assert!(
                    !header.lines().any(|l| l.contains("[];") && l.contains("also used as")),
                    "{build}: one table declared at two elements:\n{header}"
                );
            }
            let mut printed = format!("#include <stddef.h>\n#include <stdlib.h>\n#include \"{stem}.h\"\n");
            let mut bodies = String::new();
            for w in witnesses {
                let head = format!("// Function: {w} @ ");
                let at = code.find(&head).unwrap_or_else(|| panic!("{build} {arm}: no `{w}` in the export"));
                let end = code[at + head.len()..].find("// Function: ").map_or(code.len(), |e| at + head.len() + e);
                bodies.push_str(&code[at..end]);
            }
            let mut names: Vec<String> = Vec::new();
            for (i, _) in bodies.match_indices("dat_") {
                let hex: String = bodies[i + 4..].chars().take_while(|c| c.is_ascii_hexdigit()).collect();
                let name = format!("dat_{hex}");
                if !hex.is_empty() && !names.contains(&name) {
                    names.push(name);
                }
            }
            for n in &names {
                if header.contains(&format!(" {n};")) || header.contains(&format!(" {n}[];")) {
                    continue;
                }
                let listed = header.lines().find_map(|l| {
                    let why = l.split_once(&format!("/* {n} is "))?.1;
                    let decls = why.split_once("so it is not declared: ")?.1.trim_end_matches(" */");
                    decls.split(", ").find(|d| d.contains('*') || d.ends_with("[]")).map(str::to_string)
                });
                // A name the header leaves out is main's integer global; one the
                // bodies subscript is a pointer.
                let guess = if bodies.contains(&format!("{n}[")) { format!("char *{n}") } else { format!("long {n}") };
                printed.push_str(&format!("extern {};\n", listed.unwrap_or(guess)));
            }
            printed.push_str(&bodies);
            std::fs::write(out.join("printed.c"), &printed).unwrap();
            let Some(expected) = &expected else { continue };
            for cc in ["gcc", "clang"] {
                if process::optional_output(Command::new(cc).arg("--version")).is_none() {
                    eprintln!("elemptr round trip: no `{cc}`");
                    continue;
                }
                let exe = out.join(format!("rt-{cc}"));
                // Clang 16+ and gcc 14 make these errors, which `-w` does not
                // silence; a guessed declaration of an undeclared global trips them.
                let mut args: Vec<String> = [
                    "-std=gnu11",
                    "-w",
                    "-Wno-error=int-conversion",
                    "-Wno-error=incompatible-pointer-types",
                    "-O0",
                    "-fno-builtin",
                    "-no-pie",
                    "-o",
                ]
                    .iter()
                    .map(|s| s.to_string())
                    .collect();
                args.push(exe.to_str().unwrap().to_string());
                args.push(harness.to_str().unwrap().to_string());
                args.push(out.join("printed.c").to_str().unwrap().to_string());
                for n in &names {
                    args.push(format!("-Wl,--defsym,{n}=0x{}", &n[4..]));
                }
                let built = Command::new(cc).args(&args).current_dir(&out).output().expect("spawn cc");
                assert!(
                    built.status.success(),
                    "{build} {arm}/{cc}: the printed witnesses did not compile:\n{}\n{printed}",
                    String::from_utf8_lossy(&built.stderr)
                );
                let run = process::required_output(&mut Command::new(&exe));
                let mut got = String::from_utf8_lossy(&run.stdout).trim().to_string();
                let mut want = expected.clone();
                // gcc -O2 with the option off (main): `w_tabinit` fills the table
                // through `unsigned long *` and `w_nexttab` reads it as an integer
                // sum, and the header declares neither, so no one declaration
                // computes both; with the option on both read `dat_<addr>[i]`.
                if build == "gcc_O2" && arm == "off" {
                    let four = |t: &str| t.lines().take(4).collect::<Vec<_>>().join("\n");
                    got = four(&got);
                    want = four(&want);
                }
                assert_eq!(got, want, "{build} {arm}/{cc}: the printed witnesses compute something else:\n{printed}");
            }
        }
    }
    let _ = std::fs::remove_dir_all(&dir);
}

/// A read or a write narrower than the field or element its pointer is typed to
/// keeps its own width.  `narrowload_x86_64.c` reads the low bytes, one byte and
/// a masked byte of a record's 4-byte field and of its 8-byte field on a path
/// that never reads the whole field, the low half of an element one past a
/// `long` walk, and two bytes past a callee's `unsigned int` element, and writes
/// one, two and four bytes the same ways.  `main` passes each an object that
/// ends at the last byte the function touches, at the end of a readable page.
/// kuna printed `(unsigned short)a0->field_0x8`, `(a0->field_0x8 & 0x2000) != 0`
/// and `(int)a0[a1]`, which read the whole field or element and fault there.
/// The round trip exports the gcc and clang -O0 and -O2 builds with `elemptr` on
/// and off, compiles every function between the fixture's markers exactly as
/// printed against the export's header, links it with the fixture's own prelude
/// and `main` compiled on their own, and runs it: it must print what the binary
/// prints.
#[test]
fn a_narrow_read_round_trips_through_the_printed_c() {
    check_narrowload_round_trip("narrowload", 17, cfg!(all(target_os = "linux", target_arch = "x86_64")), stripped_wide);
}

#[test]
fn narrowload_spellings_are_checked_without_native_execution() {
    check_narrowload_round_trip("narrowload", 17, false, stripped_wide);
}

/// The DWARF twin: `narrowload_dwarf_x86_64.c`, built with `-g`, masks one byte
/// of a declared record's 4-byte field through a call's result, a loop's phi
/// and a pointer read out of another record, each ending at that byte at the
/// end of a page.  Main kept these narrow only with `elemptr` on; a widening
/// gated on the record alone printed `(src(k)->flags & 0x8100) == 0x8000` and
/// `(r_1->flags & 0x81) != 0x80`, which fault.
#[test]
fn a_narrow_read_of_a_declared_record_round_trips_through_the_printed_c() {
    check_narrowload_round_trip("narrowload_dwarf", 3, cfg!(all(target_os = "linux", target_arch = "x86_64")), dwarf_wide);
}

#[test]
fn narrowload_dwarf_spellings_are_checked_without_native_execution() {
    check_narrowload_round_trip("narrowload_dwarf", 3, false, dwarf_wide);
}

fn stripped_wide(_: &str, body: &str) -> Option<&'static str> {
    ["(unsigned short)a0->", "(short)a0->", "(unsigned char)a0->", "(int)a0->", "(int)a0[", "->field_0x8 & 0x"]
        .into_iter()
        .find(|w| body.contains(w))
}

fn dwarf_wide(name: &str, body: &str) -> Option<&'static str> {
    if body.contains("0x8100") {
        return Some("0x8100");
    }
    let loop_test = "->flags & 0x81) != 0x80";
    let widened = body.match_indices(loop_test).any(|(i, _)| {
        body[..i].trim_end_matches(|c: char| c.is_ascii_alphanumeric() || c == '_').ends_with('(')
    });
    (name == "d_loop" && widened).then_some(loop_test)
}

fn check_narrowload_round_trip(
    fixture: &str,
    n_tested: usize,
    run_native: bool,
    wide: fn(&str, &str) -> Option<&'static str>,
) {
    let fx = repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures");
    let src = std::fs::read_to_string(fx.join(format!("{fixture}_x86_64.c"))).unwrap();
    let prelude = src.split("/* prelude */").nth(1).unwrap().split("/* tested */").next().unwrap();
    let tested_src = src.split("/* tested */").nth(1).unwrap().split("/* main */").next().unwrap();
    let main = src.split("/* main */").nth(1).unwrap();
    let tested: Vec<&str> = tested_src
        .lines()
        .filter_map(|l| l.strip_prefix("KEEP "))
        .filter_map(|l| l.split('(').next())
        .filter_map(|l| l.rsplit([' ', '*']).next())
        .collect();
    assert_eq!(tested.len(), n_tested, "{tested:?}");
    let sp = specs();
    if !run_native {
        eprintln!("narrowload round trip: native execution disabled; checking all spellings");
    }
    let dir = common::scratch_file(&format!("{fixture}-round-trip"), "dir");
    std::fs::create_dir(&dir).unwrap();
    for build in ["gcc_O0", "clang_O0", "gcc_O2", "clang_O2"] {
        let stem = format!("{fixture}_{build}_x86_64");
        let bin = fx.join(&stem);
        let expected = run_native.then(|| process::required_output(&mut Command::new(&bin)));
        for elem in ["on", "off"] {
            let out = dir.join(format!("{build}-{elem}"));
            let (_, stderr, ok) = run_kuna(&[
                "decompile-project",
                bin.to_str().unwrap(),
                "-o",
                out.to_str().unwrap(),
                "--sleighpath",
                sp.as_str(),
                "--option",
                "elemptr",
                elem,
            ]);
            assert!(ok, "kuna decompile-project failed: {stderr}");
            let code = std::fs::read_to_string(out.join(format!("{stem}.c"))).unwrap();
            let mut bodies = String::new();
            for w in &tested {
                let head = format!("// Function: {w} @ ");
                let at = code.find(&head).unwrap_or_else(|| panic!("{build} {elem}: no `{w}` in the export"));
                let end = code[at + head.len()..].find("// Function: ").map_or(code.len(), |e| at + head.len() + e);
                let body = &code[at..end];
                if let Some(w) = wide(w, body) {
                    panic!("{build} {elem}: a narrow read printed wide, as `{w}`:\n{body}");
                }
                bodies.push_str(body);
            }
            let Some(expected) = &expected else { continue };
            std::fs::write(
                out.join("printed.c"),
                format!("#include <stddef.h>\n#include <stdlib.h>\n#include \"{stem}.h\"\n{bodies}"),
            )
            .unwrap();
            std::fs::write(out.join("harness.c"), format!("{prelude}{main}")).unwrap();
            for cc in ["gcc", "clang"] {
                if process::optional_output(Command::new(cc).arg("--version")).is_none() {
                    eprintln!("narrowload round trip: no `{cc}`");
                    continue;
                }
                let exe = out.join(format!("rt-{cc}"));
                let built = Command::new(cc)
                    .args([
                        "-std=gnu11",
                        "-w",
                        "-Wno-error=int-conversion",
                        "-Wno-error=incompatible-pointer-types",
                        "-O0",
                        "-fno-builtin",
                        "-o",
                        exe.to_str().unwrap(),
                        "harness.c",
                        "printed.c",
                    ])
                    .current_dir(&out)
                    .output()
                    .expect("spawn cc");
                assert!(
                    built.status.success(),
                    "{build} {elem}/{cc}: the printed functions did not compile:\n{}\n{bodies}",
                    String::from_utf8_lossy(&built.stderr)
                );
                let run = Command::new(&exe).output().expect("run the round trip");
                assert_eq!(
                    (String::from_utf8_lossy(&run.stdout), run.status.code()),
                    (String::from_utf8_lossy(&expected.stdout), expected.status.code()),
                    "{build} {elem}/{cc}: the printed functions compute something else:\n{bodies}"
                );
            }
        }
    }
    let _ = std::fs::remove_dir_all(&dir);
}

/// `elemptr` under `--jobs N`: a global or a table is an array only where
/// every function of a batch agrees, and a worker sees a share of the
/// functions, so a pool of more than one function types neither -- exactly as
/// a serial run that does not take the callee-first order. With `--option
/// protoorder off` on both, `decompile-all` and `decompile-project` print with
/// `--jobs 4` what they print with `--jobs 1`, and neither declares the global
/// `gp` an `int *` that `w_gpbump` steps by 4 bytes (as `int *`, `gp += 4`
/// would move 16). The default serial run still types it where the batch agrees.
#[test]
fn element_pointers_under_jobs_match_the_serial_run() {
    let bin = repo_root()
        .join("decompiler/crates/kuna-analysis/tests/fixtures/elemptr_gcc_O0_x86_64")
        .to_str()
        .unwrap()
        .to_string();
    let sp = specs();
    let off = ["--sleighpath", sp.as_str(), "--option", "protoorder", "off"];
    let serial: Vec<&str> = ["decompile-all", bin.as_str()].iter().chain(off.iter()).copied().collect();
    let (want, stderr, ok) = run_kuna(&serial);
    assert!(ok, "kuna decompile-all failed: {stderr}");
    let mut pooled = serial.clone();
    pooled.extend_from_slice(&["--jobs", "4", "--jobs-chunk", "1"]);
    let (got, stderr, ok) = run_kuna(&pooled);
    assert!(ok, "kuna decompile-all --jobs 4 failed: {stderr}");
    assert_eq!(got, want, "--jobs 4 moved the elemptr fixture's document");
    assert!(got.contains("dat_300053e8 += 4;"), "the byte step on gp:\n{got}");
    assert!(!got.contains("dat_300053e8["), "gp is indexed as an array without a batch:\n{got}");
    let (callee_first, _, ok) = run_kuna(&["decompile-all", bin.as_str(), "--sleighpath", sp.as_str()]);
    assert!(ok);
    assert!(callee_first.contains("dat_300053e0[dat_30005080[v2]] = (char)v2;"), "{callee_first}");

    let dir = std::env::temp_dir().join(format!("kuna-elemptr-jobs-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let mut exports = Vec::new();
    for jobs in ["1", "4"] {
        let out = dir.join(format!("j{jobs}"));
        let mut args = vec!["decompile-project", bin.as_str(), "-o", out.to_str().unwrap()];
        args.extend_from_slice(&off);
        args.extend_from_slice(&["--jobs", jobs, "--jobs-chunk", "1"]);
        let (_, stderr, ok) = run_kuna(&args);
        assert!(ok, "kuna decompile-project --jobs {jobs} failed: {stderr}");
        let read = |ext: &str| std::fs::read_to_string(out.join(format!("elemptr_gcc_O0_x86_64.{ext}"))).unwrap();
        exports.push((read("c"), read("h")));
    }
    let _ = std::fs::remove_dir_all(&dir);
    assert_eq!(exports[0].0, exports[1].0, "decompile-project --jobs 4 moved the .c");
    assert_eq!(exports[0].1, exports[1].1, "decompile-project --jobs 4 moved the .h");
    assert!(!exports[1].1.contains("int *dat_300053e8"), "the .h declares gp an int *:\n{}", exports[1].1);
}

/// The `elemptr` round trip's `main`: map the fixture's non-executable load
/// segments at their own addresses, then call the printed witnesses with the
/// fixture's own inputs and print its line.
const ELEMPTR_HARNESS: &str = r#"#include <elf.h>
#include <fcntl.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/mman.h>
#include <unistd.h>
unsigned char *w_decode(const char *, unsigned long, unsigned long *);
long w_sbytes(const char *, int); long w_ubytes(const unsigned char *, int);
long w_words(const int *, int); long w_back(const int *, int);
long w_sidx(const signed char *, int, const int *); long w_table(int); char *w_rev(const char *, int);
long w_record(const long *, int); long w_mixed(const char *, int);
unsigned long w_wucall(unsigned int); unsigned long w_iucall(unsigned int); unsigned long w_iu2(unsigned int);
long w_srch(unsigned int); unsigned long w_xu(const unsigned char *, int); long w_xs(const unsigned char *, int);
void w_put(char *, unsigned long, const char *); void w_ctr(unsigned int *, unsigned int *, int);
void w_gpinit(long); void w_gpbump(void); void w_gpadv(long); long w_gpread(long);
long w_tidx(unsigned int); long w_tfirst(void); void w_tset(long); long w_hdr(const unsigned int *);
void w_tabinit(void); unsigned long w_nexttab(unsigned long, unsigned long *, _Bool *);
int w_chk(const char *); long w_hash(void); long w_pick(const char *, long, int);
int main(void) {
  int fd = open("@FIXTURE@", O_RDONLY);
  Elf64_Ehdr eh; pread(fd, &eh, sizeof eh, 0);
  for (int i = 0; i < eh.e_phnum; i++) {
    Elf64_Phdr ph; pread(fd, &ph, sizeof ph, eh.e_phoff + i * sizeof ph);
    if (ph.p_type != PT_LOAD || (ph.p_flags & PF_X)) continue;
    unsigned long lo = ph.p_vaddr & ~0xfffUL, hi = (ph.p_vaddr + ph.p_memsz + 0xfff) & ~0xfffUL;
    if (mmap((void *)lo, hi - lo, PROT_READ | PROT_WRITE, MAP_PRIVATE | MAP_ANONYMOUS | MAP_FIXED_NOREPLACE, -1, 0) != (void *)lo) return 2;
    pread(fd, (void *)ph.p_vaddr, ph.p_filesz, ph.p_offset);
  }
  static const char hi[] = "\x81\x7f\xfe\x01\x80\x10";
  static const int wide[] = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16};
  static int span[256];
  for (int i = 0; i < 256; i++)
    span[i] = i * 3 - 384;
  static const long recs[] = {2, 3, 5, 7, 11, 13};
  unsigned long n = 0;
  unsigned char *dec = w_decode("aGVsbG8gd29ybGQ=", 16, &n);
  char *rev = @REV@ ? w_rev("kuna", 4) : (w_rev("kuna", 4), "-");
  long a = w_sbytes(hi, 6);
  long b = w_ubytes((const unsigned char *)hi, 6);
  long c = w_words(wide, 16);
  long d = w_back(wide + 16, 16);
  long e = w_sidx((const signed char *)hi, 6, span + 128);
  long f = w_table(8);
  long g = w_record(recs, 3);
  long h = w_mixed("abcdefgh", 2);
  printf("%s %lu %s %ld %ld %ld %ld %ld %ld %ld %ld\n", (char *)dec, n, rev, a, b, c, d, e, f, g, h);
  static const unsigned char ix[] = {0, 1, 2, 3, 4, 5};
  printf("%lu %lu %lu %lu %lu %lu %ld %ld %lu %ld\n", w_wucall(1), w_wucall(6), w_iucall(1), w_iucall(3), w_iu2(1),
         w_iu2(2), w_srch(0xffffffffu), w_srch(5), w_xu(ix, 6), w_xs(ix, 6));
  char put8[8], put4[4];
  static unsigned int fmap[8], eclass[8] = {5, 6, 7};
  w_put(put8, 8, "abc");
  w_put(put4, 4, "abcdef");
  w_ctr(fmap, eclass, 8);
  printf("%s %s %u %u %u\n", put8, put4, eclass[0], eclass[3], eclass[7]);
  w_gpinit(16);
  long g1 = w_gpread(1);
  w_gpbump();
  long g2 = w_gpread(1);
  w_gpadv(2);
  long g3 = w_gpread(1);
  long t1 = w_tidx(3), t2 = w_tfirst();
  w_tset(-7);
  long t3 = w_tidx(8), t4 = w_tfirst();
  unsigned char *pg = mmap(0, 8192, PROT_READ | PROT_WRITE, MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
  mprotect(pg + 4096, 4096, PROT_NONE);
  unsigned char *o = pg + 4096 - 6;
  o[0] = 1, o[1] = 2, o[2] = 3, o[3] = 4, o[4] = 0x34, o[5] = 0x92;
  printf("%ld %ld %ld %ld %ld %ld %ld %ld\n", g1, g2, g3, t1, t2, t3, t4, w_hdr((const unsigned int *)o));
  w_tabinit();
  unsigned long ti = 0;
  _Bool last = 0;
  unsigned long n1 = w_nexttab(9, &ti, &last), n2 = w_nexttab(0x100000005ul, &ti, &last), n3 = w_nexttab(0x200000000ul, &ti, &last);
  printf("%lu %lu %lu %lu %d\n", n1, n2, n3, ti, (int)last);
  static const unsigned char der[15] = {0x30, 0x21, 0x30, 0x09, 0x06, 0x05, 0x2b, 0x0e, 0x03, 0x02, 0x1a, 0x05, 0x00, 0x04, 0x14};
  char ok[15], bad[15];
  memcpy(ok, der, 15);
  memcpy(bad, der, 15);
  bad[14] = 0;
  printf("%d %d %ld %ld %ld\n", w_chk(ok), w_chk(bad), w_hash(), w_pick("abcdefghijklmno", 15, 1), w_pick(0, 15, 0));
  return 0;
}
"#;

/// A 64-bit value a function builds in its one return register from two 32-bit
/// halves is returned whole, and both arguments that feed it stay parameters.
/// The return-pair repair read an argument register in the returned value as
/// whatever the caller had left there: `join_lo_hi` (`((u64)hi << 32) | lo`)
/// printed `unsigned int join_lo_hi(unsigned int a0) { return a0; }` at -O0,
/// and `join_hi_sum` (`((u64)(a + 1) << 32) | b`) printed `return a0 + 1;` at
/// -O2.  The round trip compiles the printed functions with gcc and clang at -O0
/// and -O2 and checks each build prints what the fixture prints.
#[test]
fn a_value_built_in_one_return_register_round_trips_through_the_printed_c() {
    const FUNCS: &str =
        "join_lo_hi,join_third,join_sixth,join_signed,join_hi_sum,join_lo_sum,join_after_call,join_hi_lo";
    const ARITY: [(&str, usize); 8] = [
        ("join_lo_hi", 2),
        ("join_third", 3),
        ("join_sixth", 6),
        ("join_signed", 2),
        ("join_hi_sum", 2),
        ("join_lo_sum", 2),
        ("join_after_call", 2),
        ("join_hi_lo", 2),
    ];
    const WANT: &str = "1234567800000005 fedcba98ffffffff\n2222222211111111 800000007fffffff\n\
                        6666666655555555 fffffffe00000001\nfffffffe00000005 ffffffff80000000\n\
                        1234567900000005 ffffffff00000000\n1234567800000006 fedcba9800000000\n\
                        1234567800000005 fedcba98ffffffff\n0000000512345678 fffffffffedcba98\n";
    const MAIN: &str = r#"
#define F(ret, f) ((ret (*)())(void (*)())f)
typedef unsigned long u64;
int main(void) {
  printf("%016lx %016lx\n", F(u64, join_lo_hi)(5u, 0x12345678u), F(u64, join_lo_hi)(0xffffffffu, 0xfedcba98u));
  printf("%016lx %016lx\n", F(u64, join_third)(9u, 0x11111111u, 0x22222222u), F(u64, join_third)(0u, 0x7fffffffu, 0x80000000u));
  printf("%016lx %016lx\n", F(u64, join_sixth)(1u, 2u, 3u, 4u, 0x55555555u, 0x66666666u),
         F(u64, join_sixth)(1u, 2u, 3u, 4u, 1u, 0xfffffffeu));
  printf("%016lx %016lx\n", (u64)F(long, join_signed)(5, -2), (u64)F(long, join_signed)((int)0x80000000u, -1));
  printf("%016lx %016lx\n", F(u64, join_hi_sum)(0x12345678u, 5u), F(u64, join_hi_sum)(0xfffffffeu, 0u));
  printf("%016lx %016lx\n", F(u64, join_lo_sum)(5u, 0x12345678u), F(u64, join_lo_sum)(0xffffffffu, 0xfedcba98u));
  printf("%016lx %016lx\n", F(u64, join_after_call)(5u, 0x12345678u), F(u64, join_after_call)(0xffffffffu, 0xfedcba98u));
  printf("%016lx %016lx\n", F(u64, join_hi_lo)(5u, 0x12345678u), F(u64, join_hi_lo)(0xffffffffu, 0xfedcba98u));
  return 0;
}
"#;
    let sp = specs();
    let compilers: Vec<&str> = ["gcc", "clang"]
        .into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    for fixture in ["piecehi_gcc_O0_x86_64", "piecehi_clang_O0_x86_64", "piecehi_gcc_O2_x86_64", "piecehi_clang_O2_x86_64"] {
        let bin = repo_root()
            .join("decompiler/crates/kuna-analysis/tests/fixtures")
            .join(fixture)
            .to_str()
            .unwrap()
            .to_string();
        let (stdout, stderr, ok) =
            run_kuna(&["decompile-all", bin.as_str(), "--functions", FUNCS, "--sleighpath", sp.as_str()]);
        assert!(ok, "kuna decompile-all failed on {fixture}: {stderr}");
        for (name, arity) in ARITY {
            let decl = stdout
                .lines()
                .find(|l| !l.starts_with(' ') && l.contains(&format!(" {name}(")) && l.ends_with(')'))
                .unwrap_or_else(|| panic!("{fixture}: no declaration of {name}:\n{stdout}"));
            let params = decl.split_once('(').unwrap().1.trim_end_matches(')');
            assert_eq!(params.split(',').count(), arity, "{fixture}: {name} lost an argument: `{decl}`");
            assert!(
                decl.starts_with("unsigned long ") || decl.starts_with("long "),
                "{fixture}: {name} returns less than the eight bytes it computes: `{decl}`"
            );
        }
        for cc in &compilers {
            for level in ["-O0", "-O2"] {
                let dir = std::env::temp_dir()
                    .join(format!("kuna-piecehi-rt-{}-{fixture}-{cc}{level}", std::process::id()));
                std::fs::create_dir_all(&dir).unwrap();
                let src = dir.join("rt.c");
                let exe = dir.join("rt");
                std::fs::write(
                    &src,
                    format!(
                        "#include <stdio.h>\n#include <sys/wait.h>\n\
                         #define CONCAT44(h, l) ((unsigned long)(unsigned int)(h) << 32 | (unsigned int)(l))\n\
                         {stdout}\n{MAIN}"
                    ),
                )
                .unwrap();
                let out = Command::new(cc)
                    .args(["-std=gnu11", "-w", level, "-o", exe.to_str().unwrap(), src.to_str().unwrap()])
                    .output()
                    .expect("spawn the C compiler");
                assert!(
                    out.status.success(),
                    "{cc} {level} rejected the printed C ({fixture}):\n{}",
                    String::from_utf8_lossy(&out.stderr)
                );
                let run = process::required_output(&mut Command::new(&exe));
                let _ = std::fs::remove_dir_all(&dir);
                assert_eq!(
                    String::from_utf8_lossy(&run.stdout),
                    WANT,
                    "{fixture} printed and built by {cc} {level} computes a different value:\n{stdout}"
                );
            }
        }
    }
}

/// A value a sign-sensitive operation reads keeps its own variable rather
/// than merging into a global it is stored to, and the store stays where the
/// binary makes it.  `else { u = init * 3; sink = u; r = (int)(u >> 4); }` used
/// to print `sink = init * 3; r = sink >> 4;` — the shift re-read the global, so
/// a reader who gives `sink` its real type (`volatile int`) turned the logical
/// shift arithmetic.  The witnesses cover both signedness directions across a
/// branch merge, a straight line, a loop, a compare, a divide/remainder, a
/// widening (`w_sext`, `w_first`), a conversion to `double` (`w_i2f`) and a byte
/// compare (`w_eqc`), and `w_alias` stores before a pointer store that may alias
/// the global.  `w_sadd`, `w_xordiv`, `w_f`, `w_sidx` and `w_cond` reach the
/// operation through `+` or `^`, which C types after the global too
/// (`usink = a0 * 3; v1 = usink + 1 >> 4;`).  `w_fold16`, `w_fold8`, `w_neg16`,
/// `w_sub16`, `w_not16` and `w_xor16` compare a short or a byte computed with
/// `+`, `-`, `~` or `^` against a constant, which a fold later moves across that
/// operator (`hsink = a0 * 3; v1 = hsink == -1;`), `w_carry16` tests a carry,
/// and `w_meld16` ors two compares that merge into `shsink < 2`.  `w_realias`,
/// `w_rephi`, `w_reboth` and `w_recopy` store to a plain `int` and then through a
/// pointer that the harness points at it, so the load the binary makes afterwards
/// must still print as the global.  `w_ohoist`, `w_oif` and `w_oloop` load a
/// global after a pointer store that may change it, store a new value to it and
/// still use the value they loaded, which must not be copied out above the
/// pointer store.  Each is decompiled and, with the globals given their real
/// types, compiled by gcc and clang against the fixture's own `main` and must
/// print what the binary prints.  The functions a build keeps from the source
/// (clang -O2's `w_first`, gcc -O0's `w_rephi`, and the `w_o*` witnesses outside
/// the builds they witness) have printed forms with defects outside this test.
#[test]
fn a_value_read_by_a_sign_sensitive_op_is_not_re_read_from_a_global() {
    const FUNCS: &str = "w_branch,w_line,w_loop,w_less,w_div,w_signed,w_order,w_once,w_alias,w_sext,w_i2f,w_eqc,\
                         w_sadd,w_xordiv,w_f,w_sidx,w_cond,w_fold16,w_fold8,w_neg16,w_sub16,w_not16,w_xor16,w_carry16,w_meld16,\
                         w_realias,w_reboth,w_recopy";
    const DECLS: &str = "#include <stdio.h>\n#include <string.h>\n#include <stdlib.h>\n#include <stdbool.h>\n\
                         extern volatile int sink;\nextern volatile int sink2;\nextern volatile unsigned int usink;\n\
                         extern volatile unsigned char csink;\nextern volatile unsigned short hsink;\n\
                         extern volatile short shsink;\nextern unsigned long res;\nextern int retsel;\n\
                         extern char words[];\nextern int gre;\nextern int gre2;\nextern unsigned int uold;\n\
                         extern int gold;\n";
    let sp = specs();
    let fixtures_dir = repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures");
    let harness = fixtures_dir.join("globalstore_x86_64.c");
    let compilers: Vec<&str> = ["gcc", "clang"]
        .into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    for (fixture, extra, kept) in [
        ("globalstore_gcc_O0_x86_64", ",w_first,w_oif,w_oloop", &["REPHI", "OHOIST"][..]),
        ("globalstore_gcc_O2_x86_64", ",w_first,w_rephi,w_ohoist", &["OIF", "OLOOP"][..]),
        ("globalstore_clang_O2_x86_64", ",w_rephi,w_oif", &["FIRST", "OHOIST", "OLOOP"][..]),
    ] {
        let bin = fixtures_dir.join(fixture).to_str().unwrap().to_string();
        let want = match process::optional_output(&mut Command::new(&bin)) {
            Some(o) => String::from_utf8_lossy(&o.stdout).to_string(),
            None => {
                eprintln!("globalstore round trip: the fixture {fixture} does not run here, spelling checked only");
                continue;
            }
        };
        let funcs = format!("{FUNCS}{extra}");
        let (stdout, stderr, ok) =
            run_kuna(&["decompile-all", bin.as_str(), "--functions", funcs.as_str(), "--sleighpath", sp.as_str()]);
        assert!(ok, "kuna decompile-all failed on {fixture}: {stderr}");
        assert!(
            !stdout.contains("v1 = sink >> 4")
                && !stdout.contains("v1 = usink >> 4")
                && !stdout.contains("usink + 1")
                && !stdout.contains("(sink ^ 0x10")
                && !stdout.contains("hsink == ")
                && !stdout.contains("csink == ")
                && !stdout.contains("shsink < "),
            "{fixture}: the store's value is still re-read from the global:\n{stdout}"
        );
        for cc in &compilers {
            let dir = std::env::temp_dir().join(format!("kuna-globalstore-rt-{}-{fixture}-{cc}", std::process::id()));
            let _ = std::fs::remove_dir_all(&dir);
            std::fs::create_dir_all(&dir).unwrap();
            let printed = dir.join("printed.c");
            std::fs::write(&printed, format!("{DECLS}{stdout}\n")).unwrap();
            let exe = dir.join("rt");
            let mut args = vec!["-std=gnu11".to_string(), "-w".into(), "-O0".into(), "-DGLOBALSTORE_HARNESS".into()];
            args.extend(kept.iter().map(|k| format!("-DGLOBALSTORE_KEEP_{k}")));
            let out = Command::new(cc)
                .args(&args)
                .args(["-o", exe.to_str().unwrap(), printed.to_str().unwrap(), harness.to_str().unwrap()])
                .output()
                .expect("spawn the C compiler");
            assert!(
                out.status.success(),
                "{cc} rejected the printed C ({fixture}):\n{}\n{stdout}",
                String::from_utf8_lossy(&out.stderr)
            );
            let run = process::required_output(&mut Command::new(&exe));
            let got = String::from_utf8_lossy(&run.stdout).to_string();
            let _ = std::fs::remove_dir_all(&dir);
            assert_eq!(got, want, "{fixture} built by {cc} computes a different value than the binary:\n{stdout}");
        }
    }
}

/// A 64-bit value a 32-bit ABI returns in two registers joins with the high
/// word where the ABI puts it. Every big-endian ABI here returns the high word
/// in the FIRST register (PowerPC r3, MIPS o32 `$2`, SPARC `%o0`, ARM r0), and
/// return recovery and the call-output pair joined that register as the LOW
/// word, so `wide_mul` printed `CONCAT44(low, high)` and returned the two halves
/// swapped. `same` and `keep_zero` return one register. `bejoin.c`'s big-endian
/// builds and two little-endian controls are printed, every function is
/// compiled with gcc and clang at -O0 and -O2, and each is run against the
/// source's own arithmetic: the cross-built objects cannot run on this host, so
/// the source stands in for the binary. The arguments of `add_one` go in the
/// ABI's register order: high word first on a big-endian target. MIPS
/// `add_one` and `triple_plus` are left out: kuna drops a half of the first
/// (on either endianness), and in the second cannot follow MIPS's unrelocated
/// call. SPARC `triple_plus` reads the `%o0:%o1` pair its call to `triple`
/// returns (#862). SPARC `keep_zero` is left out: `restore` hands the
/// second argument back in `%o1`, and the pair still prints that argument
/// shifted into the high word, as it did before the join order was fixed.
#[test]
fn a_big_endian_register_pair_round_trips_through_the_printed_c() {
    const ALL: &[&str] = &["wide_mul", "add_one", "triple", "triple_plus", "same", "keep_zero"];
    let cases: [(&str, bool, &[&str]); 6] = [
        ("bejoin_ppc32_be.o", true, ALL),
        ("bejoin_arm32_be.o", true, ALL),
        ("bejoin_sparc32_be.o", true, &["wide_mul", "add_one", "triple", "triple_plus", "same"]),
        ("bejoin_mips32_be.o", true, &["wide_mul", "triple", "same", "keep_zero"]),
        ("bejoin_ppc32_le.o", false, ALL),
        ("bejoin_arm32_le.o", false, ALL),
    ];
    let sp = specs();
    for (fixture, big_endian, funcs) in cases {
        let bin = repo_root()
            .join("decompiler/crates/kuna-analysis/tests/fixtures")
            .join(fixture)
            .to_str()
            .unwrap()
            .to_string();
        let (stdout, stderr, ok) = run_kuna(&["decompile-all", &bin, "--sleighpath", &sp, "--option", "bejoin", "on"]);
        assert!(ok, "kuna decompile-all {fixture} failed: {stderr}");
        let printed = callrettype_functions(&stdout, funcs);
        let decls: String = printed
            .split("// Function: ")
            .skip(1)
            .filter_map(|part| part.lines().nth(1))
            .map(|sig| format!("{sig};\n"))
            .collect();
        let mut checks = String::from("  for (int i = 0; i < 10; i++)\n    for (int j = 0; j < 10; j++)\n");
        checks.push_str("      bad += (unsigned long long)wide_mul(xs[i], xs[j]) != (unsigned long long)((long long)xs[i] * xs[j]);\n");
        checks.push_str("  for (int i = 0; i < 10; i++) {\n");
        checks.push_str("    bad += (unsigned long long)triple(xs[i]) != (unsigned long long)((long long)xs[i] * 3);\n");
        if funcs.contains(&"triple_plus") {
            checks.push_str("    bad += (unsigned long long)triple_plus(xs[i]) != (unsigned long long)((long long)xs[i] * 3 + 7);\n");
        }
        checks.push_str("  }\n");
        checks.push_str("  for (int i = 0; i < 10; i++) {\n    int cell = 0;\n");
        if funcs.contains(&"keep_zero") {
            checks.push_str("    bad += keep_zero((void *)&cell, xs[i]) != 0 || cell != xs[i];\n");
        }
        checks.push_str("    for (int j = 0; j < 10; j++)\n");
        checks.push_str("      bad += (unsigned int)same(xs[i], xs[j]) != (unsigned int)(xs[i] == xs[j]);\n  }\n");
        if funcs.contains(&"add_one") {
            let (first, second) = if big_endian { ("hi", "lo") } else { ("lo", "hi") };
            checks.push_str(&format!(
                "  for (int i = 0; i < 6; i++) {{\n    int hi = (int)(vs[i] >> 32); unsigned int lo = (unsigned int)vs[i];\n    \
                 bad += (unsigned long long)add_one({first}, {second}) != (unsigned long long)(vs[i] + 1);\n  }}\n"
            ));
        }
        let src_text = format!(
            "#include <stdbool.h>\n#include <stdio.h>\n\
             #define CONCAT44(h, l) ((unsigned long long)(unsigned int)(h) << 32 | (unsigned int)(l))\n\
             {decls}{printed}\n\
             int main(void) {{\n  static const int xs[10] = {{0, 1, -1, 7, -7, 0x7fffffff, (int)0x80000000, 0x55555553, -0x55555553, 123456789}};\n  \
             static const long long vs[6] = {{0, 0xffffffffLL, -1, 0x7ffffffffffffffeLL, -0x100000000LL, 0x12345678ffffffffLL}};\n  \
             int bad = 0;\n{checks}  printf(\"%d\\n\", bad);\n  return 0;\n}}\n"
        );
        bejoin_round_trip(fixture, &src_text, &printed);
    }
}

/// Compile `src_text` -- the printed functions `printed` and a `main` that
/// prints how many of its checks disagree with the source -- with gcc and clang
/// at -O0 and -O2, run it, and require `0`. The cross-built objects cannot run
/// on this host, so the source's own arithmetic stands in for the binary.
/// The printed C is the machine's arithmetic, which wraps, so it is built with
/// `-fwrapv`: gcc -O2 otherwise folds `-a0 < 0` for `a0 == INT_MIN`.
fn bejoin_round_trip(label: &str, src_text: &str, printed: &str) {
    let compilers: Vec<&str> = ["gcc", "clang"]
        .into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    for cc in &compilers {
        for level in ["-O0", "-O2"] {
            let dir = std::env::temp_dir().join(format!("kuna-bejoin-rt-{}-{label}-{cc}{level}", std::process::id()));
            std::fs::create_dir_all(&dir).unwrap();
            let src = dir.join("rt.c");
            let exe = dir.join("rt");
            std::fs::write(&src, src_text).unwrap();
            let out = Command::new(cc)
                .args(["-std=gnu11", "-w", "-fwrapv", "-Wno-error=int-conversion", level])
                .args(["-o", exe.to_str().unwrap(), src.to_str().unwrap()])
                .output()
                .expect("spawn the C compiler");
            assert!(
                out.status.success(),
                "{cc} {level} rejected the printed C ({label}):\n{}\n{src_text}",
                String::from_utf8_lossy(&out.stderr)
            );
            let run = process::required_output(&mut Command::new(&exe));
            let _ = std::fs::remove_dir_all(&dir);
            assert_eq!(
                String::from_utf8_lossy(&run.stdout),
                "0\n",
                "{label} printed and built by {cc} {level} computes a different value:\n{printed}"
            );
        }
    }
}

/// How many parameters the printed header `sig` declares.
fn bejoin_arity(sig: &str) -> usize {
    let inner = sig.split_once('(').and_then(|(_, rest)| rest.rsplit_once(')')).map(|(p, _)| p.trim()).unwrap_or("");
    if inner.is_empty() || inner == "void" {
        0
    } else {
        inner.split(',').count()
    }
}

/// A big-endian ARM ABI returns a `long long` in r0:r1 with the high word in r0,
/// and r1 is also where the second argument arrives. `bejoin_carry.c` carries
/// that argument across a call into the low word (`mov r4,r1; bl ext; mov r0,#0;
/// mov r1,r4`). Joined first register low, all three printed `return (unsigned
/// long long)a1 << 0x20;`. The move back into r1 makes the argument a value the
/// function returns, not the register left as it arrived, so the pair joins in
/// the ABI's order even though the call might, for all return recovery knows
/// yet, read r1. The little-endian build is the control and returns the
/// argument in r0 as `a1`. Every function is compiled with gcc and clang at -O0
/// and -O2 and run against the source; `carry_low`'s argument goes in the ABI's
/// register order.
#[test]
fn an_argument_carried_across_a_call_into_the_low_word_is_returned() {
    let sp = specs();
    for (fixture, big_endian) in [("bejoin_carry_arm32_be.o", true), ("bejoin_carry_arm32_le.o", false)] {
        let bin = repo_root()
            .join("decompiler/crates/kuna-analysis/tests/fixtures")
            .join(fixture)
            .to_str()
            .unwrap()
            .to_string();
        let (stdout, stderr, ok) = run_kuna(&["decompile-all", &bin, "--sleighpath", &sp, "--option", "bejoin", "on"]);
        assert!(ok, "kuna decompile-all {fixture} failed: {stderr}");
        let printed = callrettype_functions(&stdout, &["carry_b", "carry_if", "carry_low"]);
        let sigs: Vec<&str> = printed.split("// Function: ").skip(1).filter_map(|part| part.lines().nth(1)).collect();
        assert_eq!(sigs.len(), 3, "{fixture}: all three functions print:\n{stdout}");
        if !big_endian {
            let carry_b = callrettype_functions(&stdout, &["carry_b"]);
            assert!(carry_b.contains("return a1;"), "the little-endian control returns r1 as it arrived:\n{carry_b}");
        }
        let decls: String = sigs.iter().map(|sig| format!("{sig};\n")).collect();
        let low_arity = sigs.iter().find(|s| s.contains("carry_low(")).map(|s| bejoin_arity(s)).unwrap_or(0);
        let words = if big_endian { ["hi", "lo"] } else { ["lo", "hi"] };
        let low_args = words[..low_arity.min(2)].join(", ");
        let src_text = format!(
            "#include <stdio.h>\n\
             void ext(void) {{}}\n\
             {decls}{printed}\n\
             int main(void) {{\n  static const int xs[10] = {{0, 1, -1, 7, -7, 0x7fffffff, (int)0x80000000, 0x55555553, -0x55555553, 123456789}};\n  \
             static const long long vs[6] = {{0, 0xffffffffLL, -1, 0x7ffffffffffffffeLL, -0x100000000LL, 0x12345678ffffffffLL}};\n  \
             int bad = 0;\n  \
             for (int i = 0; i < 10; i++)\n    for (int j = 0; j < 10; j++) {{\n      \
             bad += (unsigned long long)carry_b(xs[i], xs[j]) != (unsigned long long)(unsigned int)xs[j];\n      \
             bad += (unsigned long long)carry_if(xs[i], xs[j]) != (unsigned long long)(unsigned int)xs[j];\n    }}\n  \
             for (int i = 0; i < 6; i++) {{\n    unsigned int hi = (unsigned int)(vs[i] >> 32), lo = (unsigned int)vs[i];\n    \
             (void)hi;\n    bad += (unsigned long long)carry_low({low_args}) != (unsigned long long)lo;\n  }}\n  \
             printf(\"%d\\n\", bad);\n  return 0;\n}}\n"
        );
        bejoin_round_trip(fixture, &src_text, &printed);
    }
}

/// AVR's gcc ABI returns an `int` in R25:R24 with the high byte in R25, the
/// first register its `<join reversesignif="true">` rule consumes, although the
/// target is little-endian. The pair joined R25 as the low byte, so `negate`
/// printed `CONCAT11(a1,a0)`. With the order right, the two registers are
/// contiguous and name `R25R24` -- a global, since AVR maps its register file
/// into data memory -- and a value built there printed as `return R25R24;`, a
/// read of a register the printed C never assigns. `bejoin_avr.bin` is the raw
/// `.text` of `bejoin_avr.c`; both functions are compiled with gcc and clang at
/// -O0 and -O2 against the source's 16-bit arithmetic, the arguments in ABI
/// register order (R25, the high byte, first). The printed C writes the zero
/// register `R1`, which the driver declares.
#[test]
fn an_avr_register_pair_joins_high_byte_first_as_a_value() {
    let bin = repo_root()
        .join("decompiler/crates/kuna-analysis/tests/fixtures/bejoin_avr.bin")
        .to_str()
        .unwrap()
        .to_string();
    let sp = specs();
    let (stdout, stderr, ok) = run_kuna(&[
        "decompile-all",
        &bin,
        "--raw-image",
        "--target",
        "avr8:LE:16:default:gcc",
        "--base",
        "0",
        "--addr",
        "0x0",
        "--addr",
        "0x4",
        "--sleighpath",
        &sp,
        "--option",
        "bejoin",
        "on",
    ]);
    assert!(ok, "kuna decompile-all bejoin_avr.bin failed: {stderr}");
    let printed = callrettype_functions(&stdout, &["sub_0", "sub_4"]);
    let sigs: Vec<&str> = printed.split("// Function: ").skip(1).filter_map(|part| part.lines().nth(1)).collect();
    assert_eq!(sigs.len(), 2, "both functions print:\n{stdout}");
    assert!(!printed.contains("return R25R24;"), "the pair is a value, not the global register:\n{printed}");
    let decls: String = sigs.iter().map(|sig| format!("{sig};\n")).collect();
    let src_text = format!(
        "#include <stdbool.h>\n#include <stdio.h>\n\
         char R1;\n\
         #define CONCAT11(h, l) ((unsigned short)((unsigned char)(h) << 8 | (unsigned char)(l)))\n\
         {decls}{printed}\n\
         int main(void) {{\n  static const int xs[12] = {{0, 1, -1, 0x7f, 0x80, 0xff, 0x100, 0x1234, 0x7fff, -0x8000, -2, 0x55aa}};\n  \
         int bad = 0;\n  \
         for (int i = 0; i < 12; i++) {{\n    unsigned short x = (unsigned short)xs[i];\n    \
         bad += (unsigned short)sub_0((char)(x >> 8), (char)x) != (unsigned short)(0u - x);\n    \
         bad += (unsigned short)sub_4((char)(x >> 8), (unsigned char)x) != (unsigned short)(x + 0x1234u);\n  }}\n  \
         printf(\"%d\\n\", bad);\n  return 0;\n}}\n"
    );
    bejoin_round_trip("bejoin_avr.bin", &src_text, &printed);
}

/// The external calls `printed` makes -- the fixtures' `ext*` callees, or
/// `sub_*` where a call is not relocated -- declared and defined as functions
/// returning 0.
fn bejoin_stubs(printed: &str) -> (String, String) {
    let ident = |c: char| c.is_ascii_alphanumeric() || c == '_';
    let mut stubs: Vec<&str> = Vec::new();
    let mut at = 0;
    while at < printed.len() {
        let Some(start) = printed[at..].find(|c: char| ident(c)).map(|n| at + n) else { break };
        let end = printed[start..].find(|c: char| !ident(c)).map_or(printed.len(), |n| start + n);
        let name = &printed[start..end];
        if (name.starts_with("sub_") || name.starts_with("ext")) && printed[end..].trim_start().starts_with('(') {
            stubs.push(name);
        }
        at = end;
    }
    stubs.sort_unstable();
    stubs.dedup();
    (
        stubs.iter().map(|s| format!("int {s}();\n")).collect(),
        stubs.iter().map(|s| format!("int {s}() {{ return 0; }}\n")).collect(),
    )
}

/// On a big-endian ABI a function returning one `int` often leaves something
/// else in the second return register, the low word of a returned `long
/// long`: clang -O0 on MIPS materializes a zero in `$3` it never reads
/// (`realeof`, `both`), and SPARC's `restore` hands back in `%o1` whatever the
/// function left in `%i1` -- the second argument (`bound`'s first path, the
/// byte readers), a value it computed and also used (`bound`'s other paths,
/// `back4`'s loop), a byte it stored (`mark`). Joined first register low, such
/// a pair narrows back to the first register (the uncomputed-half repair, a
/// bool or byte return, C's truncation in `bound`'s narrow prototype); joined
/// in the ABI's order it keeps the wrong one, and printed `both` as `v1 <<
/// 0x20`, `bound`'s pair returns with their halves swapped, and the byte
/// readers as `char`. These stay joined as before and keep the first
/// register. Every function is compiled with gcc and clang at -O0 and -O2 and
/// run against the source. Left out: `hi_only` returns `(u64)x << 32` with the
/// same zero in `$3` as the `int` functions and prints as its high word, as
/// before; `pgetc_like` and `expand` print a pair of the value and `%i1`, as
/// before, since the window's leftover folds before it can be dropped.
#[test]
fn a_big_endian_function_returning_one_register_keeps_it() {
    let sp = specs();
    let cases: [(&str, &[&str]); 5] = [
        ("bejoin_zero_mips32_O0.o", &["realeof", "both"]),
        ("bejoin_window_sparc32_O0.o", &["mark"]),
        ("bejoin_window_sparc32_O2.o", &["back4"]),
        ("bejoin_narrow_sparc32_O0.o", &["bound", "ibyte", "ubyte", "ihalf", "ibyte_leaf"]),
        ("bejoin_narrow_sparc32_O2.o", &["bound", "ibyte", "ubyte", "ihalf", "ibyte_leaf"]),
    ];
    for (fixture, funcs) in cases {
        let bin = repo_root()
            .join("decompiler/crates/kuna-analysis/tests/fixtures")
            .join(fixture)
            .to_str()
            .unwrap()
            .to_string();
        let (stdout, stderr, ok) = run_kuna(&["decompile-all", &bin, "--sleighpath", &sp, "--option", "bejoin", "on"]);
        assert!(ok, "kuna decompile-all {fixture} failed: {stderr}");
        let printed = callrettype_functions(&stdout, funcs);
        let sigs: Vec<&str> = printed.split("// Function: ").skip(1).filter_map(|part| part.lines().nth(1)).collect();
        assert_eq!(sigs.len(), funcs.len(), "{fixture}: every function prints:\n{stdout}");
        assert!(!printed.contains("<< 0x20"), "{fixture}: no value is shifted into the high word:\n{printed}");
        let (stub_decls, stub_defs) = bejoin_stubs(&printed);
        let decls: String = sigs.iter().map(|sig| format!("{sig};\n")).collect();
        let mut checks = String::new();
        for f in funcs {
            checks.push_str(match *f {
                "realeof" => "  for (int i = 0; i < 10; i++)\n    bad += (long long)realeof(xs[i]) != (long long)ref_realeof(xs[i]);\n",
                "both" => "  for (int i = 0; i < 10; i++)\n    for (int j = 0; j < 10; j++)\n      \
                           bad += (long long)both(xs[i], xs[j]) != (long long)(xs[i] && xs[j]);\n",
                "mark" => "  for (int n = 0; n < 8; n++) {\n    char buf[8] = \"abcdefg\";\n    \
                           bad += (long long)mark(buf, n) != (long long)(n + 1) || buf[n] != 0 || buf[0] != 0;\n  }\n",
                "back4" => "  for (int i = 5; i < 29; i++)\n    bad += (long long)back4(text + i) != (long long)ref_back4(text + i);\n",
                "bound" => "  for (int i = 0; i < 16; i++)\n    for (int n = 0; n < 10; n++) {\n      \
                            int s[4] = {(i & 1) ? 15 : 3, (i >> 1) & 1, 1000 + i, (i >> 2) & 1};\n      \
                            unsigned int v = (unsigned int)xs[n] >> 2;\n      \
                            bad += (unsigned int)bound(s, v) != ref_bound(s, v);\n    }\n",
                "ibyte" | "ubyte" | "ihalf" | "ibyte_leaf" => "",
                other => panic!("no check for {other}"),
            });
        }
        if funcs.contains(&"ibyte") {
            checks.push_str(
                "  for (int b = 0; b < 256; b++) {\n    unsigned char c = (unsigned char)b;\n    unsigned short h = (unsigned short)(b * 0x101);\n    \
                 bad += (long long)ibyte(&c) != (long long)b;\n    bad += (long long)ubyte(&c) != (long long)b;\n    \
                 bad += (long long)ibyte_leaf(&c) != (long long)b;\n    bad += (long long)ihalf(&h) != (long long)h;\n  }\n",
            );
        }
        let src_text = format!(
            "#include <stdbool.h>\n#include <stdio.h>\n\
             #define CONCAT44(h, l) ((unsigned long long)(unsigned int)(h) << 32 | (unsigned int)(l))\n\
             {stub_decls}{decls}{printed}\n{stub_defs}\
             static int ref_realeof(unsigned a) {{ return a != 0 && a != 1; }}\n\
             static int ref_back4(const char *p) {{ while (p[-1] || p[-2] || p[-3] || p[-4]) p--; return p[-5] + 100; }}\n\
             static unsigned ref_bound(int *s, unsigned n) {{\n  unsigned a = n + (n >> 3) + 4, b = n + (n >> 5) + 7;\n  \
             if (s[3]) return (a > b ? a : b) + 6;\n  if (s[0] != 15) return (s[1] ? a : b) + s[2];\n  \
             return n + (n >> 12) + 13 + s[2];\n}}\n\
             int main(void) {{\n  static const int xs[10] = {{0, 1, -1, 7, -7, 0x7fffffff, (int)0x80000000, 0x55555553, -0x55555553, 123456789}};\n  \
             static const char text[29] = {{0, 0, 0, 0, 0, 9, 8, 7, 6, 5, 0, 0, 0, 0, 1, 2, 3, 4, 5, 6, 7, 0, 0, 0, 0, 11, 12, 13, 14}};\n  \
             int bad = 0;\n  (void)xs; (void)text;\n{checks}  printf(\"%d\\n\", bad);\n  return 0;\n}}\n"
        );
        bejoin_round_trip(fixture, &src_text, &printed);
    }
}

/// SPARC returns a `long long` in `%o0:%o1`, the high word in `%o0`, and each
/// function in `bejoin_window64.c` puts its low word in `%o1` on purpose:
/// through `restore`'s destination register (`restore %g0,1,%o1`), or in `%i1`
/// for `restore` to hand back (`mov 10,%i1`, `mix2`'s product), read by nothing
/// but the RETURNs. Joined first register low, `k_one`, `status64` and `bool64`
/// printed `return 0x100000000;`, `sel_const` `return (unsigned long long)v1 <<
/// 0x20;`, `mix2`'s shifted return `return a0;` and its product
/// `CONCAT44(low,high)`, and `mix3`'s `0x500000000` `return 5;`. Both builds
/// are printed; every function keeps its source's parameter count and is
/// compiled with gcc and clang at -O0 and -O2 and run against the source,
/// called as `long long`. The driver defines the calls to `ext` and `ext_i` as
/// returning 0.
#[test]
fn a_low_word_a_sparc_function_returns_on_purpose_is_part_of_the_value() {
    const FUNCS: [(&str, usize); 7] =
        [("neg_one", 0), ("k_one", 0), ("status64", 1), ("bool64", 2), ("sel_const", 1), ("mix2", 2), ("mix3", 2)];
    let names: Vec<&str> = FUNCS.iter().map(|(f, _)| *f).collect();
    let sp = specs();
    for fixture in ["bejoin_window64_sparc32_O0.o", "bejoin_window64_sparc32_O2.o"] {
        let bin = repo_root()
            .join("decompiler/crates/kuna-analysis/tests/fixtures")
            .join(fixture)
            .to_str()
            .unwrap()
            .to_string();
        let (stdout, stderr, ok) = run_kuna(&["decompile-all", &bin, "--sleighpath", &sp, "--option", "bejoin", "on"]);
        assert!(ok, "kuna decompile-all {fixture} failed: {stderr}");
        let printed = callrettype_functions(&stdout, &names);
        let sigs: Vec<&str> = printed
            .split("// Function: ")
            .skip(1)
            .filter_map(|part| part.lines().nth(1))
            .map(|sig| sig.split("//").next().unwrap_or("").trim())
            .collect();
        assert_eq!(sigs.len(), FUNCS.len(), "{fixture}: every function prints:\n{stdout}");
        for (f, arity) in FUNCS {
            let sig = sigs.iter().find(|s| s.contains(&format!(" {f}("))).expect("printed header");
            assert_eq!(bejoin_arity(sig), arity, "{fixture}: {f} keeps its parameters: {sig}");
        }
        let (stub_decls, stub_defs) = bejoin_stubs(&printed);
        let decls: String = sigs.iter().map(|sig| format!("{sig};\n")).collect();
        let src_text = format!(
            "#include <stdbool.h>\n#include <stdio.h>\n\
             #define CONCAT44(h, l) ((unsigned long long)(unsigned int)(h) << 32 | (unsigned int)(l))\n\
             {stub_decls}{decls}{printed}\n{stub_defs}\
             int main(void) {{\n  static const int xs[10] = {{0, 1, -1, 7, -7, 0x7fffffff, (int)0x80000000, 0x55555553, -0x55555553, 123456789}};\n  \
             int bad = (long long)neg_one() != -1LL;\n  \
             bad += (long long)k_one() != 1LL;\n  \
             for (int i = 0; i < 10; i++) {{\n    \
             bad += (long long)status64(xs[i]) != (long long)(xs[i] != 0);\n    \
             bad += (long long)sel_const(xs[i]) != (long long)(xs[i] ? 10 : 20);\n    \
             for (int j = 0; j < 10; j++) {{\n      \
             unsigned int x = (unsigned int)xs[i];\n      \
             bad += (unsigned long long)bool64(xs[i], xs[j]) != (unsigned long long)(xs[i] == xs[j]);\n      \
             bad += (unsigned long long)mix2(x, xs[j]) != (xs[j] ? (unsigned long long)x << 32 : (unsigned long long)x * 3);\n      \
             bad += (unsigned long long)mix3(x, xs[j]) != (xs[j] ? 0x500000000ULL : (unsigned long long)x * 3);\n    }}\n  }}\n  \
             printf(\"%d\\n\", bad);\n  return 0;\n}}\n"
        );
        bejoin_round_trip(fixture, &src_text, &printed);
    }
}

/// A 64-bit comparison on big-endian ARM computes the difference of the low
/// words into r1 for its flags and returns its `int` result in r0, so r1 holds
/// what reads as the low word of a returned `long long`: `big` (the e2fsprogs
/// `ext2fs_needs_large_file_feature` test), `ge64`, `lt64` and the saturating
/// `sat`. Joined in the ABI's order they printed `CONCAT14(flag,0x7fffffff -
/// a1)`, which C truncates to the leftover. A first register holding a
/// comparison's result, or a difference no borrow ties the first register to,
/// keeps the first-register-low join, which truncates to r0. Both builds are
/// compiled with gcc and clang at -O0 and -O2 and run against the source.
#[test]
fn a_64_bit_comparison_keeps_its_int_result_in_the_first_register() {
    const FUNCS: [&str; 4] = ["big", "ge64", "lt64", "sat"];
    let sp = specs();
    for fixture in ["bejoin_cmp_arm32_be_O0.o", "bejoin_cmp_arm32_be_O2.o"] {
        let bin = repo_root()
            .join("decompiler/crates/kuna-analysis/tests/fixtures")
            .join(fixture)
            .to_str()
            .unwrap()
            .to_string();
        let (stdout, stderr, ok) = run_kuna(&["decompile-all", &bin, "--sleighpath", &sp, "--option", "bejoin", "on"]);
        assert!(ok, "kuna decompile-all {fixture} failed: {stderr}");
        let printed = callrettype_functions(&stdout, &FUNCS);
        let sigs: Vec<&str> = printed.split("// Function: ").skip(1).filter_map(|part| part.lines().nth(1)).collect();
        assert_eq!(sigs.len(), FUNCS.len(), "{fixture}: every function prints:\n{stdout}");
        for sig in &sigs {
            let want = if sig.contains("big(") || sig.contains("sat(") { 2 } else { 4 };
            assert_eq!(bejoin_arity(sig), want, "{fixture}: {sig} keeps the source's word count");
        }
        let decls: String = sigs.iter().map(|sig| format!("{sig};\n")).collect();
        let src_text = format!(
            "#include <stdbool.h>\n#include <stdio.h>\n\
             typedef unsigned long long u64; typedef long long i64; typedef unsigned u32;\n\
             #define V(h, l) (((u64)(u32)(h) << 32) | (u32)(l))\n\
             #define CONCAT44(h, l) ((unsigned long long)(unsigned int)(h) << 32 | (unsigned int)(l))\n\
             static inline int SBORROW4(int a, int b) {{ int r; return __builtin_sub_overflow(a, b, &r); }}\n\
             {decls}{printed}\n\
             static int ref_big(u32 h, u32 l) {{ return V(h, l) >= 0x80000000ULL; }}\n\
             static int ref_ge64(u32 ah, u32 al, u32 bh, u32 bl) {{ return V(ah, al) >= V(bh, bl); }}\n\
             static int ref_lt64(int ah, u32 al, int bh, u32 bl) {{ return (i64)V(ah, al) < (i64)V(bh, bl); }}\n\
             static int ref_sat(u32 ah, u32 al) {{\n  i64 v = (i64)V(ah, al);\n  if (v > 0x7fffffff) return 0x7fffffff;\n  \
             if (v < -0x7fffffff - 1) return -0x7fffffff - 1;\n  return (int)v;\n}}\n\
             int main(void) {{\n  static const u32 ws[8] = {{0, 1, 0x7fffffff, 0x80000000, 0xffffffff, 0xfffffffe, 0x12345678, 0x80000001}};\n  \
             int bad = 0;\n  \
             for (int i = 0; i < 8; i++)\n    for (int j = 0; j < 8; j++) {{\n      \
             bad += (int)big(ws[i], ws[j]) != ref_big(ws[i], ws[j]);\n      \
             bad += (int)sat(ws[i], ws[j]) != ref_sat(ws[i], ws[j]);\n      \
             for (int k = 0; k < 8; k++) {{\n        \
             bad += (int)ge64(ws[i], ws[j], ws[k], ws[7 - j]) != ref_ge64(ws[i], ws[j], ws[k], ws[7 - j]);\n        \
             bad += (int)lt64(ws[i], ws[j], ws[k], ws[7 - j]) != ref_lt64(ws[i], ws[j], ws[k], ws[7 - j]);\n      }}\n    }}\n  \
             printf(\"%d\\n\", bad);\n  return 0;\n}}\n"
        );
        bejoin_round_trip(fixture, &src_text, &printed);
    }
}

/// The trade-off `option bejoin` makes. An `int` holding the high word of a
/// 64-bit temporary leaves the stale low half in the second return register
/// and compiles to the same registers as the `long long` returning the whole
/// value: `sum_hi` and `sum64` on PowerPC -O2 (`addc 4,4,6; addze 3,3; add
/// 3,3,5`), `mulhi` and `uwide_mul` on MIPS gcc -O0 (both halves of `multu`
/// left in `$2:$3`). On, every one reads as the `long long`, so the `int`
/// versions print the whole 64-bit value; off, the default, every one reads as
/// the `int`, so the `long long` versions print with their halves swapped.
/// gcc -O0 on MIPS moves `sum_hi`'s high word down with `srl $17,$2,0` for
/// `>> 32`, which works it over, so there `sum_hi` reads as its `int` in both
/// modes. `ltz` returns a flag and reads as its `int` either way. Each mode's
/// readings are compiled with gcc and clang at -O0 and -O2 and run against
/// the source.
#[test]
fn bejoin_reads_an_ints_stale_low_half_as_the_long_long_it_matches() {
    let sp = specs();
    for (fixture, funcs) in [
        ("bejoin_tradeoff_ppc32_O2.o", &["sum64", "sum_hi", "ltz"][..]),
        ("bejoin_tradeoff_mips32_O0.o", &["sum64", "sum_hi", "uwide_mul", "mulhi", "ltz"][..]),
    ] {
        let bin = repo_root()
            .join("decompiler/crates/kuna-analysis/tests/fixtures")
            .join(fixture)
            .to_str()
            .unwrap()
            .to_string();
        for on in [true, false] {
            let mut args = vec!["decompile-all", bin.as_str(), "--sleighpath", sp.as_str()];
            if on {
                args.extend(["--option", "bejoin", "on"]);
            }
            let (stdout, stderr, ok) = run_kuna(&args);
            assert!(ok, "kuna decompile-all {fixture} (bejoin {on}) failed: {stderr}");
            let printed = callrettype_functions(&stdout, funcs);
            let sigs: Vec<&str> = printed.split("// Function: ").skip(1).filter_map(|part| part.lines().nth(1)).collect();
            assert_eq!(sigs.len(), funcs.len(), "{fixture}: every function prints:\n{stdout}");
            let decls: String = sigs.iter().map(|sig| format!("{sig};\n")).collect();
            let ltz_args = if sigs.iter().any(|s| s.contains("ltz(") && bejoin_arity(s) == 1) { "ws[i]" } else { "ws[i], ws[j]" };
            let sum_hi_long = on && !fixture.contains("mips");
            let (sum64_want, sum_hi_want, wide_want, hi_want) = if on {
                (
                    "V(ws[i], ws[j]) + V(ws[k], ws[j ^ 3])",
                    if sum_hi_long {
                        "V(ws[i], ws[j]) + V(ws[k], ws[j ^ 3])"
                    } else {
                        "(u64)(u32)((V(ws[i], ws[j]) + V(ws[k], ws[j ^ 3])) >> 32)"
                    },
                    "(u64)ws[i] * ws[j]",
                    "(u64)ws[i] * ws[j]",
                )
            } else {
                (
                    "SW(V(ws[i], ws[j]) + V(ws[k], ws[j ^ 3]))",
                    "(u64)(u32)((V(ws[i], ws[j]) + V(ws[k], ws[j ^ 3])) >> 32)",
                    "SW((u64)ws[i] * ws[j])",
                    "(u64)(u32)(((u64)ws[i] * ws[j]) >> 32)",
                )
            };
            let mut checks = format!(
                "      bad += (int)ltz({ltz_args}) != ((i64)V(ws[i], ws[j]) < 0);\n      \
                 for (int k = 0; k < 8; k++) {{\n        \
                 bad += (u64)sum64(ws[i], ws[j], ws[k], ws[j ^ 3]) != {sum64_want};\n        \
                 bad += (u64)(u32)sum_hi(ws[i], ws[j], ws[k], ws[j ^ 3]) != (u64)(u32)({sum_hi_want})\n          \
                 || ({sum_hi_long} && (u64)sum_hi(ws[i], ws[j], ws[k], ws[j ^ 3]) != {sum_hi_want});\n      }}\n"
            );
            if funcs.contains(&"mulhi") {
                checks.push_str(&format!(
                    "      bad += (u64)uwide_mul(ws[i], ws[j]) != {wide_want};\n      \
                     bad += (u64)(u32)mulhi(ws[i], ws[j]) != (u64)(u32)({hi_want})\n        \
                     || ({on} && (u64)mulhi(ws[i], ws[j]) != {hi_want});\n"
                ));
            }
            let src_text = format!(
                "#include <stdbool.h>\n#include <stdio.h>\n\
                 typedef unsigned long long u64; typedef long long i64; typedef unsigned u32;\n\
                 #define V(h, l) (((u64)(u32)(h) << 32) | (u32)(l))\n\
                 #define SW(x) ((u64)(x) << 32 | (u64)(x) >> 32)\n\
                 #define CONCAT44(h, l) ((unsigned long long)(unsigned int)(h) << 32 | (unsigned int)(l))\n\
                 #define CARRY4(a, b) ((unsigned int)(a) + (unsigned int)(b) < (unsigned int)(a))\n\
                 {decls}{printed}\n\
                 int main(void) {{\n  static const u32 ws[8] = {{0, 1, 0x7fffffff, 0x80000000, 0xffffffff, 0xfffffffe, 0x12345678, 0x80000001}};\n  \
                 int bad = 0;\n  \
                 for (int i = 0; i < 8; i++)\n    for (int j = 0; j < 8; j++) {{\n{checks}    }}\n  \
                 printf(\"%d\\n\", bad);\n  return 0;\n}}\n"
            );
            bejoin_round_trip(&format!("{fixture}-{}", if on { "on" } else { "off" }), &src_text, &printed);
        }
    }
}

/// Two shapes of `int` on a big-endian ABI that leave a 64-bit value's word in
/// the second return register. A boolean a branch chose: `v == (int)v` on
/// SPARC -O2 is `addcc %i1,%i2,%i1; addxcc %i0,0,%i0; cmp; be` and then
/// `restore %g0,1,%o0` or `restore %g0,%g0,%o0`, so `%o1` keeps the sum, and
/// PowerPC computes the same flag with `li 3,1`/`li 3,0` on two paths, or with
/// `cntlzw 3,3; srwi 3,3,5`. And an `int` truncation of a 64-bit temporary at
/// -O0: `(int)(c ? a : b)` reloads the temporary low word first (`lwz
/// 4,12(31); lwz 3,16(31)` on PowerPC), the reverse of `pick64`'s `long long`
/// (`lwz 3,12(31); lwz 4,16(31)`). Joined in the ABI's order, `fits_int` printed
/// `return a1 + 0x80000000;` and `pick` returned the high word. Every function
/// is compiled with gcc and clang at -O0 and -O2 against `bejoin_trunc.c`, the
/// 64-bit arguments split high word first; `pick64` and `tabs64` keep their
/// fix. PowerPC -O1 and -O2 narrow `pick64` and `tabs64` to one register for a
/// reason unrelated to the join, so they are left out there.
#[test]
fn a_branch_chosen_flag_or_an_int_truncation_keeps_its_int_in_the_first_register() {
    const ALL: [&str; 8] = ["fits_int", "lua_fits", "ovf", "pick", "pick64", "min_len", "tabs", "tabs64"];
    const NARROWED: [&str; 6] = ["fits_int", "lua_fits", "ovf", "pick", "min_len", "tabs"];
    let sp = specs();
    let fixtures = repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures");
    let source = std::fs::read_to_string(fixtures.join("bejoin_trunc.c")).unwrap();
    for (fixture, funcs) in [
        ("bejoin_trunc_sparc32_O0.o", &ALL[..]),
        ("bejoin_trunc_sparc32_O2.o", &ALL[..]),
        ("bejoin_trunc_ppc32_O0.o", &ALL[..]),
        ("bejoin_trunc_ppc32_O1.o", &NARROWED[..]),
        ("bejoin_trunc_ppc32_O2.o", &NARROWED[..]),
        ("bejoin_trunc_arm32_be_O0.o", &ALL[..]),
    ] {
        let bin = fixtures.join(fixture).to_str().unwrap().to_string();
        let (stdout, stderr, ok) = run_kuna(&["decompile-all", &bin, "--sleighpath", &sp, "--option", "bejoin", "on"]);
        assert!(ok, "kuna decompile-all {fixture} failed: {stderr}");
        let printed = callrettype_functions(&stdout, funcs);
        let sigs: Vec<&str> = printed.split("// Function: ").skip(1).filter_map(|part| part.lines().nth(1)).collect();
        assert_eq!(sigs.len(), funcs.len(), "{fixture}: every function prints:\n{stdout}");
        for sig in &sigs {
            let want = if sig.contains("pick") { 5 } else { 2 };
            assert_eq!(bejoin_arity(sig), want, "{fixture}: {sig} keeps the source's word count");
        }
        let decls: String = sigs.iter().map(|sig| format!("{sig};\n")).collect();
        let refs: String = ALL.iter().map(|f| format!("#define {f} ref_{f}\n")).collect();
        let unrefs: String = ALL.iter().map(|f| format!("#undef {f}\n")).collect();
        let (tabs64, pick64) = if funcs.contains(&"pick64") {
            (
                "bad += (u64)tabs64(H(v), L(v)) != (u64)ref_tabs64(v);",
                "bad += (u64)pick64(H(v), L(v), H(w), L(w), c) != (u64)ref_pick64(v, w, c);",
            )
        } else {
            ("", "")
        };
        let src_text = format!(
            "#include <stdbool.h>\n#include <stdio.h>\n\
             typedef unsigned long long u64; typedef long long i64; typedef unsigned u32;\n\
             #define H(x) ((u32)((u64)(x) >> 32))\n\
             #define L(x) ((u32)(u64)(x))\n\
             #define CONCAT44(h, l) ((unsigned long long)(unsigned int)(h) << 32 | (unsigned int)(l))\n\
             #define CARRY4(a, b) ((unsigned int)(a) + (unsigned int)(b) < (unsigned int)(a))\n\
             static inline int SBORROW4(int a, int b) {{ int r; return __builtin_sub_overflow(a, b, &r); }}\n\
             static inline int SCARRY4(int a, int b) {{ int r; return __builtin_add_overflow(a, b, &r); }}\n\
             {refs}{source}{unrefs}{decls}{printed}\n\
             int main(void) {{\n  \
             static const u64 ws[13] = {{0, 1, 0x7fffffff, 0x80000000, 0xffffffff, 0x100000000, 0x17fffffff,\n    \
             0xffffffffffffffff, 0x8000000000000000, 0x123456789abcdef0, 0xfffffffffffffffb, 99, 0xffffffff7fffffff}};\n  \
             static const int xs[8] = {{0, 1, -1, 7, 0x7fffffff, -0x7fffffff - 1, 0x55555553, 123456789}};\n  \
             int bad = 0;\n  \
             for (int i = 0; i < 13; i++) {{\n    i64 v = (i64)ws[i];\n    \
             bad += (int)fits_int(H(v), L(v)) != ref_fits_int(v);\n    \
             bad += (int)lua_fits(H(v), L(v)) != ref_lua_fits(v);\n    \
             bad += (int)min_len(H(v), L(v)) != ref_min_len(v);\n    \
             bad += (int)tabs(H(v), L(v)) != ref_tabs(v);\n    \
             {tabs64}\n    \
             for (int j = 0; j < 13; j++)\n      for (int c = 0; c < 2; c++) {{\n        i64 w = (i64)ws[j];\n        \
             bad += (int)pick(H(v), L(v), H(w), L(w), c) != ref_pick(v, w, c);\n        \
             {pick64}\n      }}\n  }}\n  \
             for (int i = 0; i < 8; i++)\n    for (int j = 0; j < 8; j++)\n      \
             bad += (int)ovf(xs[i], xs[j]) != ref_ovf(xs[i], xs[j]);\n  \
             printf(\"%d\\n\", bad);\n  return 0;\n}}\n"
        );
        bejoin_round_trip(fixture, &src_text, &printed);
    }
}

/// An `int` range or overflow check that returns any literal, or a flag
/// negated, leaves the sum or difference it tested in the second return
/// register. SPARC -O2 `chk_err` is `addcc %i1,%i2,%i1; addxcc %i0,-1,%i0;
/// cmp %i0,-1; bne` and then `restore %g0,34,%o0` or `restore %g0,%g0,%o0`;
/// PowerPC -O1 `fits_neg` is `addc 4,4,5; addze 3,3; cntlzw 3,3; srwi 3,3,5;
/// neg 3,3`; ARM big-endian -O2 `d_code` keeps the `adcs` result in r0 on the
/// path where it is zero (`adcs r0,r0,#0; movne r0,#22`). Joined in the ABI's
/// order, each returned the leftover sum (`chk_err` printed `return a1 +
/// 0x80000000;`). The `long long` twins whose high word is a sum's carry added
/// in, or a literal on another path (`add_k`, `d64`, `mul_k`, `clamp_ll`,
/// `zsum`), keep their fix. Every function is compiled with gcc and clang at
/// -O0 and -O2 against `bejoin_code.c`, the 64-bit arguments split high word
/// first. ARM big-endian -O2 leaves out `ovf_m1` and `clamp_ll`, which print
/// wrong there for reasons unrelated to the join, with the option on or off.
#[test]
fn a_range_check_returning_any_literal_or_flag_keeps_its_int_in_the_first_register() {
    const ONE: &str = "for (int i = 0; i < 13; i++) {\n    i64 v = (i64)ws[i];\n";
    const TWO: &str =
        "for (int i = 0; i < 13; i++)\n    for (int j = 0; j < 13; j++) {\n      i64 v = (i64)ws[i], w = (i64)ws[j];\n";
    const INTS: &str = "for (int i = 0; i < 8; i++)\n    for (int j = 0; j < 8; j++) {\n";
    let checks: [(&str, &str, &str); 11] = [
        ("chk_err", ONE, "bad += (int)chk_err(H(v), L(v)) != ref_chk_err(v);"),
        ("chk_m1", ONE, "bad += (int)chk_m1(H(v), L(v)) != ref_chk_m1(v);"),
        ("fits_neg", ONE, "bad += (int)fits_neg(H(v), L(v)) != ref_fits_neg(v);"),
        ("add_k", ONE, "bad += (u64)add_k(H(v), L(v)) != (u64)ref_add_k(v);"),
        ("clamp_ll", ONE, "bad += (u64)clamp_ll(H(v), L(v)) != (u64)ref_clamp_ll(v);"),
        ("st_code2", TWO, "bad += (int)st_code2(H(v), L(v), H(w), L(w)) != ref_st_code2(v, w);"),
        ("d_code", TWO, "bad += (int)d_code(H(v), L(v), H(w), L(w)) != ref_d_code(v, w);"),
        ("d64", TWO, "bad += (u64)d64(H(v), L(v), H(w), L(w)) != (u64)ref_d64(v, w);"),
        ("ovf_m1", INTS, "bad += (int)ovf_m1(xs[i], xs[j]) != ref_ovf_m1(xs[i], xs[j]);"),
        ("mul_k", INTS, "bad += (u64)mul_k(xs[i], xs[j]) != (u64)ref_mul_k(xs[i], xs[j]);"),
        ("zsum", INTS, "bad += (u64)zsum(xs[i], xs[j]) != (u64)ref_zsum(xs[i], xs[j]);"),
    ];
    let names: Vec<&str> = checks.iter().map(|&(f, _, _)| f).collect();
    let sp = specs();
    let fixtures = repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures");
    let source = std::fs::read_to_string(fixtures.join("bejoin_code.c")).unwrap();
    for (fixture, skip) in [
        ("bejoin_code_sparc32_O2.o", &[][..]),
        ("bejoin_code_ppc32_O1.o", &[][..]),
        ("bejoin_code_ppc32_O2.o", &[][..]),
        ("bejoin_code_arm32_be_O2.o", &["ovf_m1", "clamp_ll"][..]),
    ] {
        let funcs: Vec<&str> = names.iter().copied().filter(|f| !skip.contains(f)).collect();
        let bin = fixtures.join(fixture).to_str().unwrap().to_string();
        let (stdout, stderr, ok) = run_kuna(&["decompile-all", &bin, "--sleighpath", &sp, "--option", "bejoin", "on"]);
        assert!(ok, "kuna decompile-all {fixture} failed: {stderr}");
        let printed = callrettype_functions(&stdout, &funcs);
        let sigs: Vec<&str> = printed
            .split("// Function: ")
            .skip(1)
            .filter_map(|part| part.lines().nth(1))
            .map(|sig| sig.split(" //").next().unwrap_or(sig).trim_end())
            .collect();
        assert_eq!(sigs.len(), funcs.len(), "{fixture}: every function prints:\n{stdout}");
        for sig in &sigs {
            let want = if ["st_code2", "d_code", "d64"].iter().any(|f| sig.contains(f)) { 4 } else { 2 };
            assert_eq!(bejoin_arity(sig), want, "{fixture}: {sig} keeps the source's word count");
        }
        let decls: String = sigs.iter().map(|sig| format!("{sig};\n")).collect();
        let refs: String = names.iter().map(|f| format!("#define {f} ref_{f}\n")).collect();
        let unrefs: String = names.iter().map(|f| format!("#undef {f}\n")).collect();
        let body: String = checks
            .iter()
            .filter(|(f, _, _)| funcs.contains(f))
            .map(|(_, lp, check)| format!("  {lp}      {check}\n    }}\n"))
            .collect();
        let src_text = format!(
            "#include <stdbool.h>\n#include <stdio.h>\n\
             #define H(x) ((u32)((u64)(x) >> 32))\n\
             #define L(x) ((u32)(u64)(x))\n\
             #define CONCAT44(h, l) ((unsigned long long)(unsigned int)(h) << 32 | (unsigned int)(l))\n\
             #define CARRY4(a, b) ((unsigned int)(a) + (unsigned int)(b) < (unsigned int)(a))\n\
             static inline int SBORROW4(int a, int b) {{ int r; return __builtin_sub_overflow(a, b, &r); }}\n\
             static inline int SCARRY4(int a, int b) {{ int r; return __builtin_add_overflow(a, b, &r); }}\n\
             {refs}{source}{unrefs}{decls}{printed}\n\
             int main(void) {{\n  \
             static const u64 ws[13] = {{0, 1, 0x7fffffff, 0x80000000, 0xffffffff, 0x100000000, 0x17fffffff,\n    \
             0xffffffffffffffff, 0x8000000000000000, 0x123456789abcdef0, 0xfffffffffffffffb, 99, 0xffffffff7fffffff}};\n  \
             static const int xs[8] = {{0, 1, -1, 7, 0x7fffffff, -0x7fffffff - 1, 0x55555553, 123456789}};\n  \
             int bad = 0;\n{body}  printf(\"%d\\n\", bad);\n  return 0;\n}}\n"
        );
        bejoin_round_trip(fixture, &src_text, &printed);
    }
}

/// A division by a constant on big-endian ARM multiplies by a magic number
/// and shifts the product's high half down, and `umull`/`smull` leave the
/// product's low half in r1: `a / 10` at -O0 is `umull r1,r0,r2,r3; lsr
/// r0,r0,#3`, `a / 7` on ARMv6 and v7 `umull r1,r2,r0,r1; sub; add ...,lsr
/// #1; lsr r0,r0,#2`. Joined in the ABI's order, each returned that low half
/// (`ud10` printed `CONCAT44(a0 / 10,a0 * -0x33333333)`). A high half shifted
/// down on its way to the first register keeps the first-register-low join,
/// which C's truncation narrows to the quotient. The `long long` products
/// whose high half is returned as it is or with terms added (`m3`, `mk` to
/// `mk5`) keep their fix. Every function is compiled with gcc and clang at -O0
/// and -O2 against `bejoin_div.c`, the 64-bit arguments split high word first.
#[test]
fn a_division_by_a_constant_keeps_its_quotient_in_the_first_register() {
    const UNARY: [&str; 12] =
        ["ud10", "ud100", "ud1000", "ud3600", "ud86400", "ud3", "ud5", "ud7", "ud19", "dv_365", "um7", "mk4"];
    const ALL: [&str; 19] = [
        "ud10", "ud100", "ud1000", "ud3600", "ud86400", "ud3", "ud5", "sd3", "ud7", "ud19", "dv_365", "ud7_ptr", "um7",
        "m3", "mk", "mk2", "mk3", "mk4", "mk5",
    ];
    let sp = specs();
    let fixtures = repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures");
    let source = std::fs::read_to_string(fixtures.join("bejoin_div.c")).unwrap();
    for fixture in ["bejoin_div_arm32_be_O0.o", "bejoin_div_arm32_be_v6_Os.o", "bejoin_div_arm32_be_v7_O2.o"] {
        let bin = fixtures.join(fixture).to_str().unwrap().to_string();
        let (stdout, stderr, ok) = run_kuna(&["decompile-all", &bin, "--sleighpath", &sp, "--option", "bejoin", "on"]);
        assert!(ok, "kuna decompile-all {fixture} failed: {stderr}");
        let printed = callrettype_functions(&stdout, &ALL);
        let sigs: Vec<&str> = printed.split("// Function: ").skip(1).filter_map(|part| part.lines().nth(1)).collect();
        assert_eq!(sigs.len(), ALL.len(), "{fixture}: every function prints:\n{stdout}");
        for sig in &sigs {
            let want = if ["m3(", "mk2(", "mk3(", "mk5("].iter().any(|f| sig.contains(f)) { 2 } else { 1 };
            assert_eq!(bejoin_arity(sig), want, "{fixture}: {sig} keeps the source's word count");
        }
        let decls: String = sigs.iter().map(|sig| format!("{sig};\n")).collect();
        let refs: String = ALL.iter().map(|f| format!("#define {f} ref_{f}\n")).collect();
        let unrefs: String = ALL.iter().map(|f| format!("#undef {f}\n")).collect();
        let unary: String = UNARY
            .iter()
            .map(|f| {
                format!(
                    "    bad += ({f}(x) & 0xffffffffULL) != (ref_{f}(x) & 0xffffffffULL)\n      \
                     || (sizeof(ref_{f}(x)) == 8 && (u64){f}(x) != (u64)ref_{f}(x));\n"
                )
            })
            .collect();
        let src_text = format!(
            "#include <stdbool.h>\n#include <stdio.h>\n\
             #define H(x) ((u32)((u64)(x) >> 32))\n\
             #define L(x) ((u32)(u64)(x))\n\
             #define CONCAT44(h, l) ((unsigned long long)(unsigned int)(h) << 32 | (unsigned int)(l))\n\
             {refs}{source}{unrefs}{decls}{printed}\n\
             int main(void) {{\n  \
             static const u64 ws[13] = {{0, 1, 0x7fffffff, 0x80000000, 0xffffffff, 0x100000000, 0x17fffffff,\n    \
             0xffffffffffffffff, 0x8000000000000000, 0x123456789abcdef0, 0xfffffffffffffffb, 99, 0xffffffff7fffffff}};\n  \
             static const int xs[12] = {{0, 1, -1, 7, -7, 0x7fffffff, -0x7fffffff - 1, 0x55555553, -0x55555553, 123456789, 86399, 365}};\n  \
             int bad = 0;\n  \
             for (int i = 0; i < 12; i++) {{\n    u32 x = (u32)xs[i], c = x;\n{unary}    \
             bad += (int)sd3(xs[i]) != ref_sd3(xs[i]);\n    \
             bad += (u32)ud7_ptr(&c) != ref_ud7_ptr(&c);\n    \
             bad += (u64)mk(x) != ref_mk(x);\n    \
             for (int j = 0; j < 12; j++) {{\n      \
             bad += (u64)mk3(xs[i], xs[j]) != (u64)ref_mk3(xs[i], xs[j]);\n      \
             bad += (u64)mk5(x, (u32)xs[j]) != ref_mk5(x, (u32)xs[j]);\n    }}\n  }}\n  \
             for (int i = 0; i < 13; i++) {{\n    i64 v = (i64)ws[i];\n    \
             bad += (u64)m3(H(v), L(v)) != (u64)ref_m3(v);\n    \
             bad += (u64)mk2(H(v), L(v)) != ref_mk2((u64)v);\n  }}\n  \
             printf(\"%d\\n\", bad);\n  return 0;\n}}\n"
        );
        bejoin_round_trip(fixture, &src_text, &printed);
    }
}

/// Big-endian ARM computes an `int` that is a 64-bit sum, difference or
/// product shifted right by one in the reverse register pair, low half in r0:
/// `(u32)(((u64)a + b) >> 1)` is `adds r0,r1,r0; adc r1,r2,#0; lsrs r1,r1,#1;
/// rrx r0,r0`, which moves bit 0 of r1 into r0 and leaves the stale `r1 >> 1`
/// in the second register. Joined in the ABI's order, that stale word became
/// the low word (`avg_u` printed `(unsigned long long)(...) << 0x20`). A low
/// word that is a right shift whose dropped bits reach the first register
/// keeps the first-register-low join, which C's truncation narrows to r0. A
/// `long long` shifted right moves the bit the other way, and `avg64`,
/// `mshr1_64` and `sshr33` keep their fix. Every function is compiled with gcc
/// and clang at -O0 and -O2 against `bejoin_shr.c`, the 64-bit arguments split
/// high word first.
#[test]
fn a_shift_that_carries_into_the_first_register_keeps_the_int() {
    const NARROW: [&str; 11] =
        ["avg_u", "avg_s", "avg_d", "mshr1", "sshr1", "ashr1", "sub_shr1", "avg_ret", "uq1r", "avg64", "mshr1_64"];
    const ALL: [&str; 15] = [
        "avg_u", "avg_s", "avg_d", "avg3", "mshr1", "sshr1", "mshr1k", "ashr1", "sub_shr1", "avg_ret", "uq1r", "mid64",
        "avg64", "mshr1_64", "sshr33",
    ];
    let sp = specs();
    let fixtures = repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures");
    let source = std::fs::read_to_string(fixtures.join("bejoin_shr.c")).unwrap();
    for fixture in ["bejoin_shr_arm32_be_O0.o", "bejoin_shr_arm32_be_v6_Os.o", "bejoin_shr_arm32_be_v7_O2.o"] {
        let bin = fixtures.join(fixture).to_str().unwrap().to_string();
        let (stdout, stderr, ok) = run_kuna(&["decompile-all", &bin, "--sleighpath", &sp, "--option", "bejoin", "on"]);
        assert!(ok, "kuna decompile-all {fixture} failed: {stderr}");
        let printed = callrettype_functions(&stdout, &ALL);
        let sigs: Vec<&str> = printed.split("// Function: ").skip(1).filter_map(|part| part.lines().nth(1)).collect();
        assert_eq!(sigs.len(), ALL.len(), "{fixture}: every function prints:\n{stdout}");
        let sshr33 = sigs.iter().find(|sig| sig.contains("sshr33(")).copied().unwrap_or("");
        let sshr33_args = if bejoin_arity(sshr33) == 1 { "H(v)" } else { "H(v), L(v)" };
        for sig in &sigs {
            let want = match sig {
                s if s.contains("avg3(") => 3,
                s if s.contains("mid64(") => 4,
                s if s.contains("mshr1k(") || s.contains("sshr33(") => bejoin_arity(sig).clamp(1, 2),
                _ => 2,
            };
            assert_eq!(bejoin_arity(sig), want, "{fixture}: {sig} keeps the source's word count");
        }
        let decls: String = sigs.iter().map(|sig| format!("{sig};\n")).collect();
        let refs: String = ALL.iter().map(|f| format!("#define {f} ref_{f}\n")).collect();
        let unrefs: String = ALL.iter().map(|f| format!("#undef {f}\n")).collect();
        let binary: String = NARROW
            .iter()
            .map(|f| {
                format!(
                    "      bad += ((u64){f}(x, y) & 0xffffffffULL) != ((u64)ref_{f}(x, y) & 0xffffffffULL)\n        \
                     || (sizeof(ref_{f}(x, y)) == 8 && (u64){f}(x, y) != (u64)ref_{f}(x, y));\n"
                )
            })
            .collect();
        let src_text = format!(
            "#include <stdbool.h>\n#include <stdio.h>\n\
             #define H(x) ((u32)((u64)(x) >> 32))\n\
             #define L(x) ((u32)(u64)(x))\n\
             #define CONCAT44(h, l) ((unsigned long long)(unsigned int)(h) << 32 | (unsigned int)(l))\n\
             #define CARRY4(a, b) ((unsigned int)(a) + (unsigned int)(b) < (unsigned int)(a))\n\
             {refs}{source}{unrefs}{decls}{printed}\n\
             int main(void) {{\n  \
             static const u64 ws[13] = {{0, 1, 0x7fffffff, 0x80000000, 0xffffffff, 0x100000000, 0x17fffffff,\n    \
             0xffffffffffffffff, 0x8000000000000000, 0x123456789abcdef0, 0xfffffffffffffffb, 99, 0xffffffff7fffffff}};\n  \
             static const int xs[12] = {{0, 1, -1, 7, -7, 0x7fffffff, -0x7fffffff - 1, 0x55555553, -0x55555553, 123456789, 2, 5}};\n  \
             int bad = 0;\n  \
             for (int i = 0; i < 12; i++) {{\n    u32 x = (u32)xs[i];\n    \
             bad += (u32)mshr1k(x) != ref_mshr1k(x);\n    \
             for (int j = 0; j < 12; j++) {{\n      u32 y = (u32)xs[j];\n{binary}      \
             for (int k = 0; k < 12; k++)\n        \
             bad += (u32)avg3(x, y, (u32)xs[k]) != ref_avg3(x, y, (u32)xs[k]);\n    }}\n  }}\n  \
             for (int i = 0; i < 13; i++) {{\n    i64 v = (i64)ws[i];\n    \
             bad += (u64)sshr33({sshr33_args}) != (u64)ref_sshr33(v);\n    \
             for (int j = 0; j < 13; j++)\n      \
             bad += (u32)mid64(H(v), L(v), H(ws[j]), L(ws[j])) != ref_mid64((u64)v, ws[j]);\n  }}\n  \
             printf(\"%d\\n\", bad);\n  return 0;\n}}\n"
        );
        bejoin_round_trip(fixture, &src_text, &printed);
    }
}

/// gcc copies an argument into `$3` before a switch for the case that passes
/// it on (`move v1,a1`), and a jump table the object leaves to relocations is
/// not recovered, so those cases are not part of the function kuna sees: on
/// the default path, `return "???"` leaves the copy beside the pointer in
/// `$2`, the registers `zb`, which returns its second argument zero-extended,
/// leaves on purpose. Joined in the ABI's order, `pick` returned the argument
/// (`return a1;`). A function with an indirect jump flow could not follow
/// keeps the first-register-low join, the default's, for a low word
/// nothing visible reads and the jump's code can; `zb` keeps its fix and is
/// compiled with gcc and clang at -O0 and -O2. (`pick`'s printed jump-as-call
/// is not C a compiler takes.)
#[test]
fn an_unrecovered_switch_keeps_a_pointers_register() {
    let sp = specs();
    let fixtures = repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures");
    let bin = fixtures.join("bejoin_jump_mips32_O2.o").to_str().unwrap().to_string();
    let mut listings = Vec::new();
    for on in [true, false] {
        let mut args = vec!["decompile-all", bin.as_str(), "--sleighpath", sp.as_str()];
        if on {
            args.extend(["--option", "bejoin", "on"]);
        }
        let (stdout, stderr, ok) = run_kuna(&args);
        assert!(ok, "kuna decompile-all bejoin_jump_mips32_O2.o (bejoin {on}) failed: {stderr}");
        listings.push(stdout);
    }
    let (on, off) = (&listings[0], &listings[1]);
    let pick = callrettype_functions(on, &["pick"]);
    assert!(!pick.is_empty(), "pick prints:\n{on}");
    assert_eq!(pick, callrettype_functions(off, &["pick"]), "pick keeps the first-register-low join");
    let printed = callrettype_functions(on, &["zb"]);
    assert_ne!(printed, callrettype_functions(off, &["zb"]), "zb joins in the ABI's order");
    let sig = printed.split("// Function: ").nth(1).and_then(|part| part.lines().nth(1)).unwrap_or("");
    let src_text = format!(
        "#include <stdio.h>\n\
         typedef unsigned long long u64;\n\
         #define CONCAT44(h, l) ((unsigned long long)(unsigned int)(h) << 32 | (unsigned int)(l))\n\
         {sig};\n{printed}\n\
         int main(void) {{\n  \
         int bad = 0;\n  \
         for (unsigned b = 0; b < 0x100000; b += 0x1357)\n    \
         bad += (u64)zb(7, b) != (u64)b;\n  \
         printf(\"%d\\n\", bad);\n  return 0;\n}}\n"
    );
    bejoin_round_trip("bejoin_jump_mips32_O2.o", &src_text, &printed);
}

/// An `int` worked out from the high half of a 64-bit temporary leaves the low
/// half in the second register: `(u32)((x + y) >> 48)` on PowerPC -O2 is
/// `addc 4,6,4; adde 3,5,3; srwi 3,3,16`, ARM's `umull r1,r0` leaves the low
/// half of `(u32)((x * k) >> 32) & 0xff` in r1, gcc -O0 on MIPS computes
/// `(u32)((x ^ y) >> 32) & 0xffff` in `$2:$3` and masks `$2` alone, and clang
/// -O0 on MIPS reloads a 64-bit hash's spilled `mfhi` into `$3` beside
/// `(u32)x`. Joined in the ABI's order, each returned that stale half as its
/// low word (`xy_shr48` printed `CONCAT44(a2 + a0 + CARRY4(a3,a1) >> 0x10,a3
/// + a1)`). A first register that shifts, masks, offsets, negates or
/// multiplies a sum's high word, or applies such a literal operation to a
/// value the low word never reads, or reads the low half of the product whose
/// high half is the low word, keeps the first-register-low join; the 64-bit
/// sums, differences, negation, product and packing keep their fix. Each
/// function that prints with the source's word count is compiled with gcc and
/// clang at -O0 and -O2 against `bejoin_hiop.c`, 64-bit arguments split high
/// word first.
#[test]
fn an_int_worked_out_from_the_high_half_of_a_temporary_keeps_it() {
    const SHAPES: [(&str, &str); 22] = [
        ("xy_shr48", "WW"),
        ("xb_shr36", "WI"),
        ("xy_hi_shr4", "WW"),
        ("xy_hi_and", "WW"),
        ("xy_hi_add", "WW"),
        ("sxy_hi_neg", "WW"),
        ("ss_shr34", "II"),
        ("s3_hi_mul", "III"),
        ("d_shr40", "II"),
        ("sum3_shr33", "III"),
        ("xk_shr40", "W"),
        ("p_hi_and", "II"),
        ("p_hi_add", "II"),
        ("p_hi_neg", "II"),
        ("xm_hi_and", "WI"),
        ("xx_hi_xor", "WW"),
        ("hash64", "W"),
        ("xy_add", "WW"),
        ("xy_sub", "WW"),
        ("neg64", "W"),
        ("pm", "II"),
        ("pack8", "I"),
    ];
    const WIDE: [&str; 5] = ["xy_add", "xy_sub", "neg64", "pm", "pack8"];
    let fixtures_dir = repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures");
    let source = std::fs::read_to_string(fixtures_dir.join("bejoin_hiop.c")).unwrap();
    let sp = specs();
    for (fixture, skip) in [
        ("bejoin_hiop_ppc32_O2.o", &["xx_hi_xor"][..]),
        ("bejoin_hiop_arm32_be_v7_O2.o", &["xx_hi_xor", "pack8"][..]),
        ("bejoin_hiop_sparc32_O0.o", &["xx_hi_xor", "hash64", "pack8"][..]),
        ("bejoin_hiop_mips32_O0.o", &["xy_hi_and", "xy_add", "xy_sub", "neg64"][..]),
        ("bejoin_hiop_mips32_clang_O0.o", &["xy_hi_and", "xx_hi_xor", "xy_add", "pack8"][..]),
    ] {
        let bin = fixtures_dir.join(fixture).to_str().unwrap().to_string();
        let (stdout, stderr, ok) = run_kuna(&["decompile-all", &bin, "--sleighpath", &sp, "--option", "bejoin", "on"]);
        assert!(ok, "kuna decompile-all {fixture} failed: {stderr}");
        let checked: Vec<(&str, &str)> = SHAPES.iter().copied().filter(|(f, _)| !skip.contains(f)).collect();
        let names: Vec<&str> = checked.iter().map(|&(f, _)| f).collect();
        let printed = callrettype_functions(&stdout, &names);
        let sigs: Vec<&str> = printed.split("// Function: ").skip(1).filter_map(|part| part.lines().nth(1)).collect();
        assert_eq!(sigs.len(), names.len(), "{fixture}: every checked function prints:\n{stdout}");
        for &(f, shape) in &checked {
            let sig = sigs.iter().find(|s| s.contains(&format!(" {f}("))).copied().unwrap_or("");
            let words: usize = shape.chars().map(|c| if c == 'W' { 2 } else { 1 }).sum();
            assert_eq!(bejoin_arity(sig), words, "{fixture}: {sig} keeps the source's word count");
        }
        let decls: String = sigs.iter().map(|sig| format!("{sig};\n")).collect();
        let refs: String = SHAPES.iter().map(|(f, _)| format!("#define {f} ref_{f}\n")).collect();
        let unrefs: String = SHAPES.iter().map(|(f, _)| format!("#undef {f}\n")).collect();
        let checks: String = checked
            .iter()
            .map(|&(f, shape)| {
                let cast = if WIDE.contains(&f) { "u64" } else { "u32" };
                let (loops, args, refargs) = match shape {
                    "WW" => ("for (int i = 0; i < 13; i++) for (int j = 0; j < 13; j++)", "H(ws[i]), L(ws[i]), H(ws[j]), L(ws[j])", "ws[i], ws[j]"),
                    "WI" => ("for (int i = 0; i < 13; i++) for (int j = 0; j < 12; j++)", "H(ws[i]), L(ws[i]), (u32)xs[j]", "ws[i], (u32)xs[j]"),
                    "W" => ("for (int i = 0; i < 13; i++)", "H(ws[i]), L(ws[i])", "ws[i]"),
                    "I" => ("for (int i = 0; i < 12; i++)", "xs[i]", "xs[i]"),
                    "II" => ("for (int i = 0; i < 12; i++) for (int j = 0; j < 12; j++)", "xs[i], xs[j]", "xs[i], xs[j]"),
                    _ => (
                        "for (int i = 0; i < 12; i++) for (int j = 0; j < 12; j++) for (int k = 0; k < 12; k++)",
                        "xs[i], xs[j], xs[k]",
                        "xs[i], xs[j], xs[k]",
                    ),
                };
                format!("  {loops}\n    bad += ({cast}){f}({args}) != ({cast})ref_{f}({refargs});\n")
            })
            .collect();
        let src_text = format!(
            "#include <stdio.h>\n\
             #define H(x) ((u32)((u64)(x) >> 32))\n\
             #define L(x) ((u32)(u64)(x))\n\
             #define CONCAT44(h, l) ((unsigned long long)(unsigned int)(h) << 32 | (unsigned int)(l))\n\
             #define CARRY4(a, b) ((unsigned int)(a) + (unsigned int)(b) < (unsigned int)(a))\n\
             {refs}{source}{unrefs}{decls}{printed}\n\
             int main(void) {{\n  \
             static const u64 ws[13] = {{0, 1, 0x7fffffff, 0x80000000, 0xffffffff, 0x100000000, 0x17fffffff,\n    \
             0xffffffffffffffff, 0x8000000000000000, 0x123456789abcdef0, 0xfffffffffffffffb, 99, 0xffffffff7fffffff}};\n  \
             static const int xs[12] = {{0, 1, -1, 7, -7, 0x7fffffff, -0x7fffffff - 1, 0x55555553, -0x55555553, 123456789, 2, 5}};\n  \
             int bad = 0;\n{checks}  \
             printf(\"%d\\n\", bad);\n  return 0;\n}}\n"
        );
        bejoin_round_trip(fixture, &src_text, &printed);
    }
}

/// `zerofillreturn_a64.o` (clang -O2, AArch64): `scale` returns
/// `*g * 1.5 + a` in `d0`, but the `fmov d0,#1.5` before its `fmadd` zero-fills
/// the upper half of `q0`, and that zero joined the return: `undefined16
/// scale(..)` with `v1._8_8_ = 0`, `recip`'s `return 0.0` as `ZEXT816(0)`, and
/// `floor_add` casting the 16-byte result, `(double)scale(a0,a1)`. The fill is
/// no part of the value: every function returns its `double`, the tail call
/// hands it on, and the printed functions compiled on the host compute what
/// the source does. `zerofillfwd_a64.o` (gcc -O0): `mk` reloads both members
/// of a two-double struct from its frame and joins them in `d1:d0`; `fa`
/// writes its callee's complex result through a pointer reloaded from the frame,
/// so the `d1` it reloads may be what the call left, and keeps `q0` as before.
#[test]
fn an_aarch64_double_return_leaves_out_the_zero_fill() {
    let sp = specs();
    let fixture = repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/zerofillreturn_a64.o");
    let fixture = fixture.to_str().unwrap();
    let (off, stderr, ok) = run_kuna(&["decompile-all", fixture, "--sleighpath", &sp, "--option", "zerofillreturn", "off"]);
    assert!(ok, "kuna decompile-all failed: {stderr}");
    assert!(off.contains("undefined16 scale("), "the fill no longer joins the return with the option off:\n{off}");
    let (stdout, stderr, ok) = run_kuna(&["decompile-all", fixture, "--sleighpath", &sp]);
    assert!(ok, "kuna decompile-all failed: {stderr}");
    for want in ["double scale(int a0,double *a1)", "double recip(int a0)", "double tail(int a0,double *a1)", "return 0.0;"] {
        assert!(stdout.contains(want), "missing `{want}`:\n{stdout}");
    }
    assert!(!stdout.contains("undefined16") && !stdout.contains("(double)scale("), "the fill is still returned:\n{stdout}");
    let printed = printed_functions(&stdout, &["scale ", "recip ", "tail ", "floor_add "]);
    let src = format!(
        "#include <stdio.h>\n{printed}\n\
         int main(void) {{\n  double g = 1.25;\n  \
         printf(\"%g %g %g %d %d\\n\", scale(3, &g), tail(3, &g), recip(4), floor_add(3, &g), floor_add(-2, &g));\n  \
         return 0;\n}}\n"
    );
    for (cc, got) in compile_and_run_each("zerofillreturn-a64", &src) {
        assert_eq!(got, "4.875 5.875 0.25 5 0", "{cc}: the printed C computes something else:\n{printed}");
    }
    let fwd = repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/zerofillfwd_a64.o");
    let (stdout, stderr, ok) = run_kuna(&["decompile-all", fwd.to_str().unwrap(), "--sleighpath", &sp]);
    assert!(ok, "kuna decompile-all failed: {stderr}");
    let fa = printed_functions(&stdout, &["fa "]);
    let mk = printed_functions(&stdout, &["mk "]);
    assert!(mk.contains("// d1:d0") && mk.contains("._8_8_ = a0 * a0;"), "mk's reloaded pair is not joined:\n{mk}");
    assert!(!fa.contains("d1:d0") && fa.contains("._8_8_ = 0;"), "fa joins a d1 its callee may have left:\n{fa}");
}

/// `voidret_complexpick_a64.o` (clang -O2, AArch64): `pick` tail-calls `cpart`,
/// which returns a double, on one path and `cboth`, a complex double recovered
/// as a 16-byte `d1:d0` join, on the other; `user` reads `pick`'s `d0`. The
/// return forced on `pick` for `user` hands back an untyped call result on the
/// `cboth` path, which the float-return check counted as a conversion: `pick`
/// was decompiled again without its return and printed `void`, beside
/// `(double)pick(a0)` in `user`. A value left in a floating register is not an
/// integer converted to a float, with the zero-fill option off or on.
#[test]
fn a_tail_call_wrapper_keeps_the_float_its_readers_read() {
    let sp = specs();
    let fixture = repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/voidret_complexpick_a64.o");
    let fixture = fixture.to_str().unwrap();
    for extra in [&[][..], &["--option", "zerofillreturn", "off"][..]] {
        let mut args = vec!["decompile-all", fixture, "--sleighpath", sp.as_str()];
        args.extend_from_slice(extra);
        let (stdout, stderr, ok) = run_kuna(&args);
        assert!(ok, "kuna decompile-all failed: {stderr}");
        for want in ["double pick(double a0)", "return cpart(a0); // tail-call", "v1 = pick(a0);"] {
            assert!(stdout.contains(want), "{extra:?}: missing `{want}`:\n{stdout}");
        }
    }
}

/// `floatbits_a64.o` (clang -O2, AArch64): `to_bits` and `exponent` move the
/// bits of `getd`'s `double` to an integer register (`fmov x0,d0`), `to_fbits`
/// those of `getf`'s `float` (`fmov w0,s0`), and `store_bits` stores them
/// through an `unsigned long *`. The listing declares `double getd(..)` and
/// `float getf(..)`, and the readers printed `return getf(a0,a1);` from an
/// `unsigned int` function, `(unsigned long)getd(a0,a1) >> 0x34` and `*a2 =
/// getd(a0,a1);`, which C converts by value. The printed functions, compiled
/// on the host, hand back the bits.
#[test]
fn a_float_result_held_as_bits_round_trips() {
    let sp = specs();
    let fixture = repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/floatbits_a64.o");
    let fixture = fixture.to_str().unwrap();
    let (stdout, stderr, ok) = run_kuna(&["decompile-all", fixture, "--sleighpath", &sp]);
    assert!(ok, "kuna decompile-all failed: {stderr}");
    for want in ["double getd(int a0,double *a1)", "float getf(int a0,float *a1)"] {
        assert!(stdout.contains(want), "missing `{want}`:\n{stdout}");
    }
    for bad in ["return getd(", "return getf(", ")getd(", ")getf(", "*a2 = getd("] {
        assert!(!stdout.contains(bad), "a reader converts the float by value (`{bad}`):\n{stdout}");
    }
    let printed = printed_functions(&stdout, &["getd ", "getf ", "to_bits ", "to_fbits ", "exponent ", "store_bits "]);
    let src = format!(
        "#include <stdio.h>\n{printed}\n\
         int main(void) {{\n  double g = 1.25;\n  float f = 1.25f;\n  unsigned long s = 0;\n  \
         store_bits(-3, &g, (void *)&s);\n  \
         printf(\"%lx %lx %x %x %d %d %lx\\n\", (unsigned long)to_bits(3, &g), (unsigned long)to_bits(-3, &g), \
         (unsigned int)to_fbits(3, &f), (unsigned int)to_fbits(-3, &f), (int)exponent(3, &g), (int)exponent(-3, &g), s);\n  \
         return 0;\n}}\n"
    );
    for (cc, got) in compile_and_run_each("floatbits-a64", &src) {
        assert_eq!(
            got, "4011000000000000 c00a000000000000 40880000 c0500000 1025 1024 c00a000000000000",
            "{cc}: the printed C computes something else:\n{printed}"
        );
    }
}
