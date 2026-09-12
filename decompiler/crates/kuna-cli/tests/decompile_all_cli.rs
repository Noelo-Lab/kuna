//! CLI end-to-end gate for `kuna decompile-all` / `kuna functions` — drives the
//! built `kuna` binary over the real vendored `fauxware` ELF and asserts the
//! machine-readable JSON surface decbench and an LLM driver consume.
//!
//! ## `.sla` precondition
//!
//! Bootstrapping needs the built `x86` `.sla` under `specs/` (gitignored;
//! `make specs`).  When it is absent the command fails to build an architecture;
//! the test prints that and returns early (a specs-less CI is a visible skip,
//! never a false green).

mod common;

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

/// Parse the `"count": N` field out of the decompile-all `--json` header.
fn json_count(stdout: &str) -> Option<usize> {
    let i = stdout.find("\"count\":")? + "\"count\":".len();
    stdout[i..].trim_start().split(|c: char| !c.is_ascii_digit()).next()?.parse().ok()
}

/// Every entry address in a `functions` / `decompile-all` `--json` document, in
/// document (address) order.  `"address_hex"` is a different key, so the `":"` in
/// the pattern is what keeps it out.
fn json_addresses(stdout: &str) -> Vec<u64> {
    stdout
        .match_indices("\"address\":")
        .filter_map(|(i, m)| {
            stdout[i + m.len()..]
                .trim_start()
                .split(|c: char| !c.is_ascii_digit())
                .next()?
                .parse()
                .ok()
        })
        .collect()
}

/// Run `kuna <cmd> <bin> --mode reliable --json` and return its entry addresses,
/// or `None` on a missing-`.sla` skip.
fn run_json_addrs(cmd: &str, bin: &str, sp: &str, extra: &[&str]) -> Option<Vec<u64>> {
    let mut args = vec![cmd, bin, "--json", "--sleighpath", sp, "--mode", "reliable"];
    args.extend_from_slice(extra);
    let (stdout, stderr, ok) = run_kuna(&args);
    if !ok {
        if is_specs_skip(&stderr) {
            return None;
        }
        panic!("kuna {cmd} failed on {bin}: {stderr}");
    }
    Some(json_addresses(&stdout))
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

/// Run the built `kuna` binary, returning `(stdout, stderr, success)`.
fn run_kuna(args: &[&str]) -> (String, String, bool) {
    let out = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .env_remove("KUNA_DECOMP_DBG")
        .env_remove("KUNA_DECOMP_TEST")
        .env_remove("KUNA_SLACOMP")
        .args(args)
        .output()
        .expect("failed to spawn the kuna binary");
    (
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
        out.status.success(),
    )
}

/// Run the built `kuna` binary with a hard outer wall-clock `cap`, returning
/// `Some((stdout, stderr, success))` if it exited in time, `None` if it had to
/// be killed.  The outer cap is the regression guard for the watchdog itself:
/// without `--max-fn-seconds` the hang-repro invocation would spin forever.
fn run_kuna_with_timeout(args: &[&str], cap: Duration) -> Option<(String, String, bool)> {
    use std::io::Read;
    let mut child = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .env_remove("KUNA_DECOMP_DBG")
        .env_remove("KUNA_DECOMP_TEST")
        .env_remove("KUNA_SLACOMP")
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to spawn the kuna binary");
    // Drain the pipes on reader threads so a chatty child can never block on a
    // full pipe while we poll for exit.
    let mut stdout_pipe = child.stdout.take().expect("stdout piped");
    let mut stderr_pipe = child.stderr.take().expect("stderr piped");
    let out_thread = std::thread::spawn(move || {
        let mut buf = Vec::new();
        let _ = stdout_pipe.read_to_end(&mut buf);
        buf
    });
    let err_thread = std::thread::spawn(move || {
        let mut buf = Vec::new();
        let _ = stderr_pipe.read_to_end(&mut buf);
        buf
    });
    let deadline = Instant::now() + cap;
    let status = loop {
        match child.try_wait().expect("try_wait on the kuna binary") {
            Some(st) => break Some(st),
            None if Instant::now() >= deadline => {
                let _ = child.kill();
                let _ = child.wait();
                break None;
            }
            None => std::thread::sleep(Duration::from_millis(200)),
        }
    };
    let stdout = String::from_utf8_lossy(&out_thread.join().expect("stdout reader")).into_owned();
    let stderr = String::from_utf8_lossy(&err_thread.join().expect("stderr reader")).into_owned();
    status.map(|st| (stdout, stderr, st.success()))
}

/// `true` when the failure is a missing-`.sla` bootstrap failure (a legitimate
/// skip), not a real bug.
fn is_specs_skip(stderr: &str) -> bool {
    stderr.contains("could not build an architecture")
        || stderr.contains("SLEIGH")
        || stderr.contains("Could not discover")
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
    if is_specs_skip(&stderr) {
        eprintln!("decompile_all_iat: skipping (no `.sla`; run `make specs`): {stderr}");
        return;
    }
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
        if is_specs_skip(&stderr) {
            eprintln!("decompile_all_cli: skipping (no `.sla`; run `make specs`): {stderr}");
            return;
        }
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
    let run = |extra: &[&str]| -> Option<String> {
        let mut args =
            vec!["decompile-all", bin.as_str(), "--json", "--no-vars", "--sleighpath", sp.as_str()];
        args.extend_from_slice(extra);
        let (stdout, stderr, ok) = run_kuna(&args);
        if !ok {
            if is_specs_skip(&stderr) {
                return None;
            }
            panic!("kuna decompile-all failed on the ARM fixture: {stderr}");
        }
        Some(stdout)
    };

    let Some(fast) = run(&["--mode", "fast"]) else {
        eprintln!("fast mode: skipping (no `.sla`; run `make specs`)");
        return;
    };
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
    ])
    .expect("explicit fast-equivalent run");
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
    let run = |mode: Option<&str>| -> Option<String> {
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
            if is_specs_skip(&stderr) {
                return None;
            }
            panic!("kuna functions failed for mode {mode:?}: {stderr}");
        }
        Some(stdout)
    };

    let Some(omitted) = run(None) else {
        eprintln!("auto mode: skipping (no `.sla`; run `make specs`)");
        return;
    };
    assert_eq!(omitted, run(Some("auto")).expect("explicit auto"));
    assert_eq!(omitted, run(Some("aggressive")).expect("explicit aggressive"));
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
    let run = |extra: &[&str]| -> Option<usize> {
        let mut args = vec![
            "decompile-all", bin.as_str(), "--json", "--sleighpath", sp.as_str(),
            "--mode", "reliable",
        ];
        args.extend_from_slice(extra);
        let (stdout, stderr, ok) = run_kuna(&args);
        if !ok {
            if is_specs_skip(&stderr) {
                return None;
            }
            panic!("kuna decompile-all failed on the ARM fixture: {stderr}");
        }
        Some(json_count(&stdout).expect("count in json"))
    };
    let Some(default_cnt) = run(&[]) else {
        eprintln!("arm funcstart default: skipping (no `.sla`; run `make specs`)");
        return;
    };
    let off_cnt = run(&["--option", "funcstart_patterns", "off"]).expect("second run");
    let on_cnt = run(&["--option", "funcstart_patterns", "on"]).expect("third run");
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
    let run = |extra: &[&str]| -> Option<usize> {
        let mut args = vec![
            "decompile-all", bin.as_str(), "--json", "--sleighpath", sp.as_str(),
            "--mode", "reliable",
        ];
        args.extend_from_slice(extra);
        let (stdout, stderr, ok) = run_kuna(&args);
        if !ok {
            if is_specs_skip(&stderr) {
                return None;
            }
            panic!("kuna decompile-all failed on the ARM fixture: {stderr}");
        }
        Some(json_count(&stdout).expect("count in json"))
    };
    let Some(default_cnt) = run(&[]) else {
        eprintln!("arm aif default: skipping (no `.sla`; run `make specs`)");
        return;
    };
    let off_cnt = run(&["--option", "aif", "off"]).expect("second run");
    let on_cnt = run(&["--option", "aif", "on"]).expect("third run");
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
    let run = |extra: &[&str]| -> Option<usize> {
        let mut args = vec![
            "decompile-all", bin.as_str(), "--json", "--sleighpath", sp.as_str(),
            "--mode", "reliable",
        ];
        args.extend_from_slice(extra);
        let (stdout, stderr, ok) = run_kuna(&args);
        if !ok {
            if is_specs_skip(&stderr) {
                return None;
            }
            panic!("kuna decompile-all failed on the ARM fixture: {stderr}");
        }
        Some(json_count(&stdout).expect("count in json"))
    };
    let Some(default_cnt) = run(&[]) else {
        eprintln!("arm raw-prologue default: skipping (no `.sla`; run `make specs`)");
        return;
    };
    // `funcstart_patterns off` disables the whole recursive-discovery tier (the raw
    // Thumb-prologue seed is gated on the same flag), so the default (with the raw
    // seed active) must never discover fewer.
    let off_cnt = run(&["--option", "funcstart_patterns", "off"]).expect("second run");
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
    let Some(inventory) = run_json_addrs("functions", &bin, &sp, &[]) else {
        eprintln!("arm functions parity: skipping (no `.sla`; run `make specs`)");
        return;
    };
    let decompiled =
        run_json_addrs("decompile-all", &bin, &sp, &["--no-vars"]).expect("second run");

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
    )
    .expect("third run");
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
    let Some(inventory) = run_json_addrs("functions", &bin, &sp, &[]) else {
        eprintln!("x86-64 functions parity: skipping (no `.sla`; run `make specs`)");
        return;
    };
    let no_listing =
        run_json_addrs("functions", &bin, &sp, &["--option", "listing", "off"]).expect("second run");
    assert_eq!(
        inventory, no_listing,
        "x86-64 `kuna functions` must not build the Listing — the DIV-15 default is the \
         decompiling surfaces'"
    );
    let decompiled =
        run_json_addrs("decompile-all", &bin, &sp, &["--no-vars"]).expect("third run");
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
        if is_specs_skip(&stderr) {
            eprintln!("entry dedup: skipping (no `.sla`; run `make specs`)");
            return;
        }
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
        if is_specs_skip(&stderr) {
            eprintln!("alias lookup: skipping (no `.sla`; run `make specs`)");
            return;
        }
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
        if is_specs_skip(&stderr) {
            eprintln!("thumb --addr: skipping (no `.sla`; run `make specs`)");
            return;
        }
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
        if is_specs_skip(&stderr) {
            return;
        }
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
        if is_specs_skip(&stderr) {
            eprintln!("ARM PE CLI: skipping (no ARM `.sla`)");
            return;
        }
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
    if !ok && is_specs_skip(&stderr) {
        eprintln!("te_image CLI: skipping (no ARM `.sla`): {stderr}");
        let _ = std::fs::remove_file(path);
        return;
    }
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
        if !ok && matches!(command, "functions" | "decompile-all") && is_specs_skip(&stderr) {
            eprintln!("TE {command} filters: skipping (no ARM `.sla`): {stderr}");
            continue;
        }
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
        if is_specs_skip(&stderr) {
            eprintln!("decompile_all_cli: skipping (no `.sla`; run `make specs`): {stderr}");
            return;
        }
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
        if is_specs_skip(&stderr) {
            eprintln!("decompile_all_cli: skipping (no `.sla`; run `make specs`): {stderr}");
            return;
        }
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
        if is_specs_skip(&stderr) {
            eprintln!("decompile_all_cli: skipping (no `.sla`; run `make specs`): {stderr}");
            return;
        }
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
        if is_specs_skip(&stderr) {
            eprintln!("decompile_all_cli: skipping (no `.sla`; run `make specs`): {stderr}");
            return;
        }
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
    let code = |extra: &[&str]| -> Option<String> {
        let mut a: Vec<&str> =
            vec!["decompile-all", &bin, "--addr", "0x4011c0", "--json", "--sleighpath", &sp];
        a.extend_from_slice(extra);
        let (stdout, stderr, ok) = run_kuna(&a);
        if !ok {
            eprintln!("decompile-all failed (likely a specs-less environment): {stderr}");
            return None;
        }
        Some(stdout)
    };
    // OFF: err_fatal's flow walks past `call error(2)` into the following functions.
    // `funcboundflow` (default-on, DIV-67) is a SECOND, name-independent bound that
    // stops the same overrun at `compute`'s entry, so it must also be off to expose
    // the pre-fix overrun this test isolates.
    let Some(off) = code(&["--option", "noreturn_error", "off", "--option", "funcboundflow", "off"])
    else {
        return; // specs-less skip
    };
    // ON (default): the CALL_RETURN prune stops err_fatal at the no-return call.
    let on = code(&[]).expect("second run succeeds if the first did");
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
    let code = |extra: &[&str]| -> Option<String> {
        let mut a: Vec<&str> = vec!["decompile", &bin, "0x4011c0", "--addr", "--sleighpath", &sp];
        a.extend_from_slice(extra);
        let (stdout, stderr, ok) = run_kuna(&a);
        if !ok {
            eprintln!("kuna decompile failed (likely a specs-less environment): {stderr}");
            return None;
        }
        Some(stdout)
    };
    // `err_warn` belongs to `compute_warn`, a DIFFERENT function — it appears in err_fatal's
    // output ONLY if the flow overran past `call error(2)`.  `funcboundflow` (default-on,
    // DIV-67) is a second, name-independent bound at `compute`'s entry, so it too must be
    // off to expose the pre-fix overrun.
    let Some(off) = code(&["--option", "noreturn_error", "off", "--option", "funcboundflow", "off"])
    else {
        return; // specs-less skip
    };
    let on = code(&[]).expect("second run succeeds if the first did");
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
    let code = |extra: &[&str]| -> Option<String> {
        let mut a: Vec<&str> =
            vec!["decompile", &bin, "elaborate_debug_symbol", "--sleighpath", &sp];
        a.extend_from_slice(extra);
        let (stdout, stderr, ok) = run_kuna(&a);
        if !ok {
            if is_specs_skip(&stderr) {
                return None;
            }
            panic!("kuna decompile failed for {extra:?}: {stderr}");
        }
        Some(stdout)
    };

    let Some(default) = code(&[]) else {
        return; // specs-less skip
    };
    assert!(
        default.contains("elaborate_debug_symbol"),
        "expected the function body, got:\n{default}"
    );
    assert!(
        !default.contains("/* debug_symbol.c:"),
        "the default (auto -> aggressive here) must NOT annotate source lines:\n{default}"
    );

    let aggressive = code(&["--mode", "aggressive"]).expect("second run succeeds");
    assert!(
        !aggressive.contains("/* debug_symbol.c:"),
        "--mode aggressive must NOT annotate source lines:\n{aggressive}"
    );

    // Named explicitly, the pass still works — and outranks the mode.
    let opted_in = code(&["--option", "dwarf_lines", "on"]).expect("third run succeeds");
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
    if !spec.exists() {
        eprintln!("raw_image CLI: skipping (no ARM `.sla`)");
        let _ = std::fs::remove_file(path);
        return;
    }

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

#[test]
fn raw_image_decompile_scales_word_addressed_selector() {
    let path = common::scratch_file("raw-avr-return", "bin");
    std::fs::write(&path, [0, 0, 0x08, 0x95]).unwrap();
    let binary = path.to_string_lossy().into_owned();
    let sp = specs();
    let spec = PathBuf::from(&sp).join("Ghidra/Processors/Atmel/data/languages/avr8.sla");
    if !spec.exists() {
        eprintln!("raw_image CLI: skipping (no AVR8 `.sla`)");
        let _ = std::fs::remove_file(path);
        return;
    }

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
    if !spec.exists() {
        eprintln!("raw_image CLI: skipping (no AVR8 `.sla`)");
        let _ = std::fs::remove_file(path);
        return;
    }

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
    if !avr_spec.exists() || !arm_spec.exists() {
        eprintln!("raw_image CLI: skipping (no AVR8 or ARM `.sla`)");
        return;
    }

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
    if !spec.exists() {
        eprintln!("raw_image CLI: skipping (no x86-64 `.sla`)");
        let _ = std::fs::remove_file(path);
        return;
    }

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

/// Every `"size": N` in a `--json` document, in document order.
fn json_sizes(stdout: &str) -> Vec<u64> {
    stdout
        .match_indices("\"size\":")
        .filter_map(|(i, key)| {
            stdout[i + key.len()..]
                .trim_start()
                .split(|c: char| !c.is_ascii_digit())
                .next()?
                .parse()
                .ok()
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
        if is_specs_skip(&stderr) {
            eprintln!("functions_json_carries_a_ranking_extent: skipping (no `.sla`): {stderr}");
            return;
        }
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
        if is_specs_skip(&stderr) {
            eprintln!("functions_and_decompile_all_agree_on_size: skipping: {stderr}");
            return;
        }
        panic!("kuna functions failed: {stderr}");
    }
    let (decompiled, stderr, ok) =
        run_kuna(&["decompile-all", &bin, "--json", "--sleighpath", &sp]);
    if !ok {
        if is_specs_skip(&stderr) {
            eprintln!("functions_and_decompile_all_agree_on_size: skipping: {stderr}");
            return;
        }
        panic!("kuna decompile-all failed: {stderr}");
    }
    // Both documents are address-ordered over the same entry set, so the size
    // columns line up positionally.
    let want = json_sizes(&inventory);
    // `decompile-all` also emits a `size` per recovered VARIABLE; keep only the
    // per-function ones by pairing each with the entry address that precedes it.
    let got: Vec<u64> = json_addresses(&decompiled)
        .iter()
        .map(|addr| {
            let rec = decompiled
                .split(&format!("\"address\": {addr},"))
                .nth(1)
                .expect("each entry address must open a record");
            json_sizes(rec).first().copied().expect("each record must carry `size`")
        })
        .collect();
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
        if is_specs_skip(&stderr) {
            eprintln!("instruction_budget_overrun_truncates_instead_of_failing: skipping: {stderr}");
            return;
        }
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
    assert!(ok, "kuna decompile-all failed: {stderr}");
    assert!(
        failed.contains("Flow exceeded maximum allowable instructions")
            && failed.contains("\"code\": null"),
        "`--option errortoomanyinstructions on` must restore the hard failure:\n{failed}"
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
        if is_specs_skip(&stderr) {
            eprintln!("fast_discovery_finds_the_pointer_only_target: skipping (no `.sla`)");
            return;
        }
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

/// The pool's whole contract: the document must not depend on how many processes
/// produced it, or on how the work was cut up between them.  Every job count and
/// chunk size here has to agree with the serial run byte for byte — dispatch
/// order is longest-first, which is deliberately not output order, so a
/// positional merge is the only thing that can make this hold.
#[test]
fn jobs_output_is_byte_identical_to_serial() {
    let bin = fauxware();
    let sp = specs();
    let (want, stderr, ok) =
        run_kuna(&["decompile-all", &bin, "--json", "--max-fn-seconds", "0", "--sleighpath", &sp]);
    if !ok {
        if is_specs_skip(&stderr) {
            eprintln!("jobs: skipping (no `.sla`; run `make specs`): {stderr}");
            return;
        }
        panic!("kuna decompile-all failed: {stderr}");
    }
    assert!(want.contains("\"name\": \"main\""), "the serial run decompiled nothing:\n{want}");

    for (jobs, chunk) in [("2", None), ("3", Some("1")), ("4", Some("7")), ("8", Some("1000"))] {
        let mut args = vec![
            "decompile-all",
            &bin,
            "--json",
            "--max-fn-seconds",
            "0",
            "--sleighpath",
            &sp,
            "--jobs",
            jobs,
        ];
        if let Some(chunk) = chunk {
            args.extend_from_slice(&["--jobs-chunk", chunk]);
        }
        let (got, stderr, ok) = run_kuna(&args);
        assert!(ok, "kuna decompile-all --jobs {jobs} failed: {stderr}");
        assert_eq!(got, want, "--jobs {jobs} (chunk {chunk:?}) moved the document");
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
        if is_specs_skip(&stderr) {
            eprintln!("jobs full-load: skipping (no `.sla`; run `make specs`): {stderr}");
            return;
        }
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
        if is_specs_skip(&stderr) {
            eprintln!("jobs c++ names: skipping (no `.sla`; run `make specs`): {stderr}");
            return;
        }
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
        if is_specs_skip(&stderr) {
            eprintln!("jobs auto: skipping (no `.sla`; run `make specs`): {stderr}");
            return;
        }
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
    if !status.success() {
        eprintln!("jobs scratch: skipping (the run failed; likely no `.sla`)");
        return;
    }
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
    fn children_of(pid: u32) -> Vec<u32> {
        std::fs::read_to_string(format!("/proc/{pid}/task/{pid}/children"))
            .unwrap_or_default()
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
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
            eprintln!("jobs cancellation: skipping (the pool never came up; likely no `.sla`)");
            return;
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
