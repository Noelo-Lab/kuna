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
    use std::io::Read;
    let mut child = kuna_command(env)
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
    if !spec.exists() {
        eprintln!("raw_image CLI: skipping (no Cortus APS3 `.sla`)");
        let _ = std::fs::remove_file(path);
        return;
    }

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
    if is_specs_skip(&stderr) {
        eprintln!("aggregate_exit_distinguishes_all_failed_from_partial_success: skipping: {stderr}");
        return;
    }
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
            if is_specs_skip(&stderr) {
                eprintln!("jobs: skipping (no `.sla`; run `make specs`): {stderr}");
                return;
            }
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
        if is_specs_skip(&stderr) {
            eprintln!("jobs: skipping (no `.sla`; run `make specs`): {stderr}");
            return;
        }
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
        if is_specs_skip(&stderr) {
            eprintln!("protoorder: skipping (no `.sla`; run `make specs`): {stderr}");
            return;
        }
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
            if is_specs_skip(&stderr) {
                eprintln!("protoorder cycles: skipping (no `.sla`; run `make specs`): {stderr}");
                return;
            }
            panic!("kuna decompile-all --option protoorder {value} failed: {stderr}");
        }
        for f in ["wrap", "wrap2"] {
            assert!(got.contains(&format!("void {f}{param}")), "{value}: {f}{param} missing:\n{got}");
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
            if is_specs_skip(&stderr) {
                eprintln!("calleevote frame: skipping (no `.sla`; run `make specs`): {stderr}");
                return;
            }
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
            if is_specs_skip(&stderr) {
                eprintln!("protoorder cyclestruct: skipping (no `.sla`; run `make specs`): {stderr}");
                return;
            }
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
    if !ok && is_specs_skip(&stderr) {
        eprintln!("protoorder float-in-GPR: skipping (no `.sla`; run `make specs`)");
        return;
    }
    assert!(ok, "kuna decompile-all failed: {stderr}");
    for want in [
        "return (int)(float)h(a0,v1) + v1 + 3;",
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

    if Command::new("cc").arg("--version").output().map(|o| !o.status.success()).unwrap_or(true) {
        eprintln!("protoorder float-in-GPR round trip: no `cc`, spelling checked only");
        return;
    }
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
             r20 = g20(7, b, &s); r22 = g22(7, b, c); r24 = (int)g24(7, b, q);\n  \
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
    let cc = Command::new("cc")
        .args(["-std=gnu11", "-w", "-fno-pie", "-no-pie", "-o", exe.to_str().unwrap(), src.to_str().unwrap()])
        .output()
        .expect("spawn cc");
    assert!(cc.status.success(), "the printed callers did not compile:\n{}", String::from_utf8_lossy(&cc.stderr));
    let run = Command::new(&exe).output().expect("run the round trip");
    let got = String::from_utf8_lossy(&run.stdout).to_string();
    let _ = std::fs::remove_dir_all(&dir);
    let lines: Vec<&str> = got.lines().collect();
    assert_eq!(lines.len(), 2, "{got}");
    assert_eq!(lines[0], lines[1], "the printed callers compute different values:\n{printed}");
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
    let have_cc = Command::new("cc").arg("--version").output().map(|o| o.status.success()).unwrap_or(false);
    for off in [false, true] {
        let mut args = vec!["decompile-all", bin.as_str(), "--option", "structdefs", "on", "--sleighpath", &sp];
        if off {
            args.extend_from_slice(&["--option", "protoorder", "off"]);
        }
        let (stdout, stderr, ok) = run_kuna(&args);
        if !ok && is_specs_skip(&stderr) {
            eprintln!("protoorder float pointee: skipping (no `.sla`; run `make specs`)");
            return;
        }
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
        let run = Command::new(&exe).output().expect("run the round trip");
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
        if !ok && is_specs_skip(&stderr) {
            eprintln!("protoorder byte pointee: skipping (no `.sla`; run `make specs`)");
            return;
        }
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
    if !ok && is_specs_skip(&stderr) {
        eprintln!("protoorder structsynth sweep: skipping (no `.sla`; run `make specs`)");
        return;
    }
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
        if !ok && is_specs_skip(&stderr) {
            eprintln!("protoorder word fills: skipping (no `.sla`; run `make specs`)");
            return;
        }
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
                if is_specs_skip(&stderr) {
                    eprintln!("jobs structsynth: skipping (no `.sla`; run `make specs`): {stderr}");
                    return;
                }
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
        if is_specs_skip(&stderr) {
            eprintln!("jobs structsynth install: skipping (no `.sla`; run `make specs`): {stderr}");
            return;
        }
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
        if is_specs_skip(&stderr) {
            eprintln!("jobs structsynth peb: skipping (no `.sla`; run `make specs`): {stderr}");
            return;
        }
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
        if is_specs_skip(&stderr) {
            eprintln!("jobs structsynth off: skipping (no `.sla`; run `make specs`): {stderr}");
            return;
        }
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
            let sla =
                PathBuf::from(specs()).join("Ghidra/Processors/x86/data/languages/x86-64.sla");
            assert!(!sla.exists(), "the pool never came up although {} is built", sla.display());
            eprintln!("jobs cancellation: skipping (the pool never came up; no `.sla`)");
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

// --- `--jobs N`: a dead worker costs one function --------------------------

/// `decompile-all --json` records, split at their own indentation, so a record
/// can be compared whole and named without a JSON parser.
fn json_records(doc: &str) -> Vec<&str> {
    doc.split("\n    {\n").skip(1).map(|r| r.split("\n    }").next().unwrap_or(r)).collect()
}

fn record_name(record: &str) -> &str {
    let at = record.find("\"name\": \"").expect("a record has a name") + "\"name\": \"".len();
    &record[at..at + record[at..].find('"').expect("a terminated name")]
}

/// The count a `[kuna <tag>] N function(s) left unfinished ... recovered.` line
/// reports, or `None` when the run printed no such line.
fn recovered_count(stderr: &str) -> Option<usize> {
    let line = stderr.lines().find(|l| l.contains("left unfinished by a failed worker process"))?;
    line.split("] ").nth(1)?.split(' ').next()?.parse().ok()
}

/// The serial `decompile-all --json` document of `fauxware`, or `None` on a
/// specs-less skip.
fn fauxware_serial_json() -> Option<String> {
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
        if is_specs_skip(&stderr) {
            eprintln!("jobs faults: skipping (no `.sla`; run `make specs`): {stderr}");
            return None;
        }
        panic!("kuna decompile-all failed: {stderr}");
    }
    Some(want)
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
        if !lost.contains(&record_name(w)) {
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
    let Some(serial) = fauxware_serial_json() else { return };
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
    let Some(serial) = fauxware_serial_json() else { return };
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
    let Some(serial) = fauxware_serial_json() else { return };
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
    for r in json_records(&got).into_iter().filter(|r| lost.contains(&record_name(r))) {
        assert!(r.contains(stalled), "{r}");
        let marked = r.contains("; not re-run: two functions re-run from its chunk stalled\"");
        assert_eq!(marked, not_rerun.contains(&record_name(r)), "{r}");
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
    let Some(serial) = fauxware_serial_json() else { return };
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
    let Some(serial) = fauxware_serial_json() else { return };
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
        if is_specs_skip(&want_err) {
            eprintln!("jobs decode: skipping (no `.sla`; run `make specs`): {want_err}");
            return;
        }
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
        if is_specs_skip(&stderr) {
            eprintln!("jobs decode arm: skipping (no `.sla`; run `make specs`)");
            return;
        }
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
        if is_specs_skip(&stderr) {
            eprintln!("jobs decode fault: skipping (no `.sla`; run `make specs`)");
            return;
        }
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
        if is_specs_skip(&stderr) {
            eprintln!("jobs decode all: skipping (no `.sla`; run `make specs`)");
            return;
        }
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
        if is_specs_skip(&stderr) {
            eprintln!("jobs decode spawn: skipping (no `.sla`; run `make specs`)");
            return;
        }
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
    if !ok && is_specs_skip(&stderr) {
        eprintln!("jobs decode raw: skipping (no `.sla`; run `make specs`)");
        return;
    }
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
    if !ok && is_specs_skip(&stderr) {
        eprintln!("jobs decode assert: skipping (no `.sla`; run `make specs`)");
        return;
    }
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
    if !ok && is_specs_skip(&stderr) {
        eprintln!("signfield round trip: skipping (no `.sla`; run `make specs`)");
        return;
    }
    assert!(ok, "kuna decompile-all failed: {stderr}");
    assert!(stdout.contains("unsigned short field_0xc;"), "{stdout}");
    assert!(stdout.contains("sink(a0->field_0xc);"), "{stdout}");

    if Command::new("cc").arg("--version").output().map(|o| !o.status.success()).unwrap_or(true) {
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
    let run = Command::new(&exe).output().expect("run the round trip");
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
    if !ok && is_specs_skip(&stderr) {
        eprintln!("unionfield round trip: skipping (no `.sla`; run `make specs`)");
        return;
    }
    assert!(ok, "kuna decompile-all failed: {stderr}");
    assert!(stdout.contains("char field_0x8[8];"), "{stdout}");
    assert!(stdout.contains("return *(double *)a0->field_0x8;"), "{stdout}");

    if Command::new("cc").arg("--version").output().map(|o| !o.status.success()).unwrap_or(true) {
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
    let run = Command::new(&exe).output().expect("run the round trip");
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
        if !ok && is_specs_skip(&stderr) {
            eprintln!("expandload round trip: skipping (no `.sla`; run `make specs`)");
            return;
        }
        assert!(ok, "kuna decompile-all failed: {stderr}");
        assert!(stdout.contains(call), "{stdout}");

        if Command::new("cc").arg("--version").output().map(|o| !o.status.success()).unwrap_or(true) {
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
        let run = Command::new(&exe).output().expect("run the round trip");
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
        ("v1 = (long)*(char *)(a0 + v2);", "v1 = *(char *)(a0 + v2);"),
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
        .filter(|cc| Command::new(cc).arg("--version").output().is_ok_and(|o| o.status.success()))
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
                "--option", "castimplied", opt,
            ];
            let (stdout, stderr, ok) = run_kuna(&args);
            if !ok && is_specs_skip(&stderr) {
                eprintln!("castimplied round trip: skipping (no `.sla`; run `make specs`)");
                return;
            }
            assert!(ok, "kuna decompile-all failed: {stderr}");
            for want in kept {
                assert!(stdout.contains(want), "{fixture} option {opt} lost `{want}`:\n{stdout}");
            }
            for (off, on) in changed {
                let want = if opt == "on" { on } else { off };
                assert!(stdout.contains(want), "{fixture} option {opt} does not print `{want}`:\n{stdout}");
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
                let run = Command::new(&exe).output().expect("run the round trip");
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

/// The text of one function in a `decompile-all` listing, from its `// Function:`
/// header to the next.
fn castsign_function<'a>(listing: &'a str, name: &str) -> &'a str {
    let head = format!("// Function: {name} @");
    let start = listing.find(&head).unwrap_or_else(|| panic!("{name} not printed:\n{listing}"));
    let rest = &listing[start + head.len()..];
    &listing[start..start + head.len() + rest.find("// Function: ").unwrap_or(rest.len())]
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
            &[(
                "v1 = (unsigned long)*(unsigned int *)(a0 + (long)a2 * 4);",
                "v1 = *(unsigned int *)(a0 + (long)a2 * 4);",
            )],
            OLD,
        ),
    ];
    let sp = specs();
    let compilers: Vec<&str> = ["gcc", "clang"]
        .into_iter()
        .filter(|cc| Command::new(cc).arg("--version").output().is_ok_and(|o| o.status.success()))
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
                "--option", "castsign", opt,
            ];
            let (stdout, stderr, ok) = run_kuna(&args);
            if !ok && is_specs_skip(&stderr) {
                eprintln!("castsign round trip: skipping (no `.sla`; run `make specs`)");
                return;
            }
            assert!(ok, "kuna decompile-all failed: {stderr}");
            for (off, on) in lines {
                let want = if opt == "on" { on } else { off };
                assert!(stdout.contains(want), "{fixture} option {opt} does not print `{want}`:\n{stdout}");
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
                    let run = Command::new(&exe).output().expect("run the round trip");
                    let _ = std::fs::remove_dir_all(&dir);
                    assert_eq!(
                        String::from_utf8_lossy(&run.stdout),
                        want,
                        "{fixture} printed with option {opt} and built by {cc} {level} computes a different value:\n{stdout}"
                    );
                }
            }
            printed.push(stdout);
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
        if !ok && is_specs_skip(&stderr) {
            eprintln!("castsign locked declaration: skipping (no `.sla`; run `make specs`)");
            return;
        }
        assert!(ok, "kuna decompile failed: {stderr}");
        for want in unlocked {
            assert!(stdout.contains(want), "{name} {func} unlocked does not print `{want}`:\n{stdout}");
        }
        for opt in ["on", "off"] {
            let args = [
                "decompile", bin.as_str(), func, "--sleighpath", sp.as_str(), "--assert", assertion,
                "--option", "castsign", opt,
            ];
            let (stdout, stderr, ok) = run_kuna(&args);
            assert!(ok, "kuna decompile --assert failed: {stderr}");
            for want in locked {
                assert!(
                    stdout.contains(want),
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
        for want in locked {
            assert!(stdout.contains(want), "the DWARF local, option {opt}, does not print `{want}`:\n{stdout}");
        }
    }
    let dir = std::env::temp_dir().join(format!("kuna-castsign-lock-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let stripped = dir.join("sign_of");
    let strip = Command::new("objcopy")
        .args(["--strip-debug", dwarf.as_str(), stripped.to_str().unwrap()])
        .output();
    if strip.is_ok_and(|o| o.status.success()) {
        let args = [
            "decompile-all", stripped.to_str().unwrap(), "--functions", "sign_of", "--sleighpath",
            sp.as_str(), "--option", "castsign", "on",
        ];
        let (stdout, stderr, ok) = run_kuna(&args);
        assert!(ok, "kuna decompile-all (debug info stripped) failed: {stderr}");
        for want in signed_stack {
            assert!(stdout.contains(want), "stripped, the slot does not print `{want}`:\n{stdout}");
        }
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
    let have_cc = Command::new("cc").arg("--version").output().map(|o| o.status.success()).unwrap_or(false);
    for build in ["gcc_O0", "clang_O0", "gcc_O2"] {
        let bin = fx.join(format!("castarith_{build}_x86_64"));
        let bin = bin.to_str().unwrap();
        for arm in ["on", "off"] {
            let args = [
                "decompile-all", bin, "--sleighpath", sp.as_str(),
                "--option", "structdefs", "on", "--option", "castarith", arm,
            ];
            let (stdout, stderr, ok) = run_kuna(&args);
            if !ok && is_specs_skip(&stderr) {
                eprintln!("castarith round trip: skipping (no `.sla`; run `make specs`)");
                return;
            }
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
            let expected = Command::new(bin).output().expect("run the fixture");
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
            let got = Command::new(&exe).output().expect("run the round trip");
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
    let have_cc = Command::new("cc").arg("--version").output().map(|o| o.status.success()).unwrap_or(false);
    for build in ["gcc_O0", "clang_O0", "gcc_O2"] {
        let bin = fx.join(format!("castindex_{build}_x86_64"));
        let bin = bin.to_str().unwrap();
        for arm in ["on", "off"] {
            let args = ["decompile-all", bin, "--sleighpath", sp.as_str(), "--option", "castindex", arm];
            let (stdout, stderr, ok) = run_kuna(&args);
            if !ok && is_specs_skip(&stderr) {
                eprintln!("castindex round trip: skipping (no `.sla`; run `make specs`)");
                return;
            }
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
            for w in want.iter().chain(kept) {
                assert!(body.contains(w), "{build} castindex {arm}: expected `{w}`\n{body}");
            }
            if arm == "on" {
                assert!(!body.contains("(long)b64_table"), "{build}: the table lookup kept its round trip\n{body}");
            }
            if !runs_here || !have_cc {
                eprintln!("castindex round trip: no x86-64 host or no `cc`, spelling checked only");
                continue;
            }
            let expected = Command::new(bin).output().expect("run the fixture");
            let dir = std::env::temp_dir()
                .join(format!("kuna-castindex-rt-{}-{build}-{arm}", std::process::id()));
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
            let got = Command::new(&exe).output().expect("run the round trip");
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
    let have_cc = Command::new("cc").arg("--version").output().map(|o| o.status.success()).unwrap_or(false);
    let decompile = |bin: &str, arm: &str| -> Option<String> {
        let args = [
            "decompile-all", bin, "--sleighpath", sp.as_str(),
            "--option", "structdefs", "on", "--option", "castarith", arm,
        ];
        let (stdout, stderr, ok) = run_kuna(&args);
        if !ok && is_specs_skip(&stderr) {
            eprintln!("castarith enum round trip: skipping (no `.sla`; run `make specs`)");
            return None;
        }
        assert!(ok, "kuna decompile-all failed: {stderr}");
        Some(stdout)
    };
    let cxx = fx.join("castarith_enumclass_gpp_O2_x86_64");
    let Some(out) = decompile(cxx.to_str().unwrap(), "on") else { return };
    let rd = out.split("// Function: rd_enum_class ").nth(1).expect("rd_enum_class printed");
    let rd = rd.split("// Function: ").next().unwrap();
    for w in ["use_kind(*(Kind *)((long)p + 5));", "use_op(*(Op *)((long)p + 6));"] {
        assert!(rd.contains(w), "enum class: expected `{w}`\n{rd}");
    }
    for build in ["gcc_O0", "gcc_O2"] {
        let bin = fx.join(format!("castarith_enum_{build}_x86_64"));
        let bin = bin.to_str().unwrap();
        for arm in ["on", "off"] {
            let Some(stdout) = decompile(bin, arm) else { return };
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
            let expected = Command::new(bin).output().expect("run the fixture");
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
            let got = Command::new(&exe).output().expect("run the round trip");
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
    let expected = Command::new(&bin).output().map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string());
    let Ok(expected) = expected else {
        eprintln!("globalref round trip: the x86-64 fixture does not run here, spelling checked only");
        return;
    };
    assert_eq!(expected, "210 21 27 121 1 0 4 229 1 -8608764254683430263 2", "the fixture itself");
    let dir = std::env::temp_dir().join(format!("kuna-globalref-rt-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
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
        if !ok && is_specs_skip(&stderr) {
            eprintln!("globalref round trip: skipping (no `.sla`; run `make specs`)");
            return;
        }
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
        for cc in ["gcc", "clang"] {
            if Command::new(cc).arg("--version").output().map(|o| !o.status.success()).unwrap_or(true) {
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
            let run = Command::new(&exe).output().expect("run the round trip");
            let got = String::from_utf8_lossy(&run.stdout).trim().to_string();
            assert_eq!(got, expected, "{arm}/{cc}: the printed callers compute something else:\n{printed}");
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
        if !ok && is_specs_skip(&stderr) {
            eprintln!("foldcallret short-circuit round trip: skipping (no `.sla`; run `make specs`)");
            return;
        }
        assert!(ok, "kuna decompile-all failed on {name}: {stderr}");
        for w in want {
            assert!(stdout.contains(w), "{name}: missing `{w}`:\n{stdout}");
        }
        for n in never {
            assert!(!stdout.contains(n), "{name}: the call was folded into a right-hand operand (`{n}`):\n{stdout}");
        }

        if Command::new("cc").arg("--version").output().map(|o| !o.status.success()).unwrap_or(true) {
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
        let run = Command::new(&exe).output().expect("run the round trip");
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
    if !ok && is_specs_skip(&stderr) {
        eprintln!("aliasoverlap round trip: skipping (no `.sla`; run `make specs`)");
        return;
    }
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
        ("next", "*(unsigned int *)(a0 + 4 + a1 * 4) = ", "return *(unsigned int *)(a0 + a1 * 4);"),
    ];
    for (name, first, second) in ordered {
        let b = body(name);
        let (i, j) = (b.find(first), b.find(second));
        assert!(i.is_some() && j.is_some() && i < j, "{name}: `{first}` must print before `{second}`:\n{b}");
    }

    if Command::new("cc").arg("--version").output().map(|o| !o.status.success()).unwrap_or(true) {
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
    let run = Command::new(&exe).output().expect("run the round trip");
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
    if !ok && is_specs_skip(&stderr) {
        eprintln!("splitload round trip: skipping (no `.sla`; run `make specs`)");
        return;
    }
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

    if Command::new("cc").arg("--version").output().map(|o| !o.status.success()).unwrap_or(true) {
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
    let run = Command::new(&exe).output().expect("run the round trip");
    let got = String::from_utf8_lossy(&run.stdout).to_string();
    let _ = std::fs::remove_dir_all(&dir);
    assert_eq!(got.lines().last(), Some("0"), "the printed functions read different bytes:\n{got}\n{printed}");
}
