//! `kuna decompile-all` / `kuna functions` — in-process **whole-binary**
//! decompilation.
//!
//! ```text
//!   kuna decompile-all <binary> [--json] [--functions a,b,..] [--addr 0xVMA].. \
//!                       [--no-vars] [--max-fn-seconds N] [--option N V].. \
//!                       [--slice ARCH] [--target T] [--sleighpath D]
//!   kuna functions <binary> [--json] [--slice ARCH] [--target T] [--sleighpath D]
//! ```
//!
//! Unlike `kuna decompile` (which spawns a fresh `decomp_dbg` subprocess **per
//! function** — re-parsing the SLEIGH spec and re-running the whole-binary
//! analysis tier every time), this loads and analyzes the binary **once**
//! in-process (`bootstrap_from_object` → `commit_pending_analysis`, i.e. the
//! `load file` + `read symbols` seam), then loops `decompile` + `print C` over
//! every function.  The marginal per-function cost drops from a full image load
//! to just the IR build + pipeline — the load-once shape benchmark harnesses
//! (decbench) and an LLM driver need.
//!
//! The per-function decompile mirrors the console `decompile` command
//! (`IfcDecompile`): it re-seeds the function's DWARF stack locals via
//! [`ConsoleProgram::dwarf_locals_for`] (so a `-g` binary renders DWARF names),
//! and a per-function pipeline abort (the decompile drive already catches panics
//! / un-ported seams and returns `Err`) is recorded as that function's `error`
//! rather than aborting the whole binary.
//!
//! `--json` emits a machine-readable object (the decbench / LLM surface); without
//! it the command prints concatenated C with `// Function: <name> @ <addr>`
//! headers (the human surface).
//!
//! **The Listing is on by default for `decompile-all`** (decbench F1, DIV-15):
//! the decompile surface injects `--option listing on` unless the caller names
//! `listing` explicitly, so the default-on `noreturn_propagate` consumer (the
//! angr-style call-graph no-return fixpoint) actually fires and a call to an
//! unnamed internal exit/fatal wrapper in a stripped binary terminates the
//! caller instead of swallowing the following functions.  Opt out with
//! `--option listing off`.  `kuna functions` and the subprocess
//! `kuna decompile` / `decomp_dbg` surfaces keep the engine default
//! (listing off).

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_console::engine::{bootstrap_from_object, ConsoleProgram};
// The decompile loop + result shape live in the shared decompile-project core
// (`kuna_console::project` — also reused by the `kuna_wasm` front-end).
use kuna_console::project::{decompile_targets, render_c, FuncResult};
use object::Object; // `File::architecture()` (ARM-discovery default, decbench)
use kuna_decomp::decompile_drive::VarInfo;
use kuna_decomp::options::{OptionDatabase, KUNA_OPTION_NAMES, RELOC_OBJECTS_ENV};

use crate::jsonfmt::{dumps_indent2, Json};
use crate::paths;

/// Parsed `decompile-all` / `functions` arguments (the two share a loader;
/// `decompile-project` reuses the same parse via its own wrapper).
pub(crate) struct Args {
    pub(crate) binary: String,
    pub(crate) json: bool,
    /// `--functions a,b,c`: restrict to these names (None ⇒ every function).
    pub(crate) names: Option<Vec<String>>,
    /// `--addr 0xVMA` (repeatable): decompile specific entry addresses, even if
    /// unnamed (stripped / LLM use).  Combined with `--functions` if both given.
    pub(crate) addrs: Vec<u64>,
    /// `--no-vars`: skip the per-function variable extraction (faster; drops the
    /// `variables` array used by decbench's `type_match`).
    pub(crate) no_vars: bool,
    /// `--max-fn-seconds N` (decompile-all / decompile-project): per-function
    /// decompile watchdog budget in seconds; `0` disables.  A function that
    /// exceeds it is recorded as that function's `error` (the batch continues)
    /// instead of hanging the whole run — the defensive cap for the known
    /// stripped-ELF non-convergence hang (`tests/hang-repro/`).  Default 120.
    pub(crate) max_fn_seconds: u64,
    pub(crate) options: Vec<(String, String)>,
    pub(crate) slice: Option<String>,
    pub(crate) target: Option<String>,
    pub(crate) sleighpath: Option<String>,
}

/// `kuna decompile-all` entry point.
pub fn run(argv: &[String]) -> i32 {
    let args = match parse_args(argv, "decompile-all") {
        Ok(a) => a,
        Err(e) => {
            eprintln!("error: {e}");
            usage_decompile_all();
            return 2;
        }
    };
    match decompile_all(&args) {
        Ok(funcs) => {
            if args.json {
                println!("{}", dumps_indent2(&result_json(&args.binary, &funcs)));
            } else {
                print!("{}", render_c(&funcs));
            }
            0
        }
        Err(e) => {
            eprintln!("error: {e}");
            1
        }
    }
}

/// `kuna functions` entry point (enumeration only — no decompile).
pub fn run_functions(argv: &[String]) -> i32 {
    let args = match parse_args(argv, "functions") {
        Ok(a) => a,
        Err(e) => {
            eprintln!("error: {e}");
            usage_functions();
            return 2;
        }
    };
    match list_functions(&args) {
        Ok(entries) => {
            if args.json {
                let arr = Json::Array(
                    entries
                        .iter()
                        .map(|(n, a)| {
                            Json::Object(vec![
                                ("name".into(), Json::Str(n.clone())),
                                ("address".into(), Json::Number(a.to_string())),
                                ("address_hex".into(), Json::Str(format!("0x{a:x}"))),
                            ])
                        })
                        .collect(),
                );
                println!(
                    "{}",
                    dumps_indent2(&Json::Object(vec![
                        ("binary".into(), Json::Str(args.binary.clone())),
                        ("count".into(), Json::Number(entries.len().to_string())),
                        ("functions".into(), arr),
                    ]))
                );
            } else {
                for (name, addr) in &entries {
                    println!("0x{addr:x}\t{name}");
                }
            }
            0
        }
        Err(e) => {
            eprintln!("error: {e}");
            1
        }
    }
}

/// Load + analyze the binary once, then decompile every selected function.
fn decompile_all(args: &Args) -> Result<Vec<FuncResult>, String> {
    let mut prog = load_program(args, /* default_listing= */ true)?;
    let targets = resolve_targets(&prog, args)?;

    // Per-function watchdog (`--max-fn-seconds`, default 120, 0 disables):
    // driver policy, not a stage-model option — the decompile drive arms a
    // cooperative deadline from this budget for EACH function, so one
    // pathological non-converging function becomes a per-function `error`
    // record instead of hanging the whole batch (see `tests/hang-repro/`).
    if args.max_fn_seconds > 0 {
        prog.arch_mut().kuna_fn_budget =
            Some(std::time::Duration::from_secs(args.max_fn_seconds));
    }

    Ok(decompile_targets(&mut prog, targets, args.no_vars, /* want_proto= */ false))
}

/// Enumerate the program's functions as `(name, address)` (the `functions`
/// command + the default `decompile-all` target set).
fn list_functions(args: &Args) -> Result<Vec<(String, u64)>, String> {
    let prog = load_program(args, /* default_listing= */ false)?;
    let mut entries: Vec<(String, u64)> = prog
        .function_entries()
        .map(|(n, a)| (n.to_string(), a.get_offset()))
        .collect();
    // Deduplicate by (address, name) — a symbol can appear in both the loader set
    // and a later register — and sort by address for a stable, readable order.
    entries.sort_by(|a, b| a.1.cmp(&b.1).then_with(|| a.0.cmp(&b.0)));
    entries.dedup();
    Ok(entries)
}

/// Bootstrap the architecture from the binary and run the analysis commit (the
/// in-process `load file` + `read symbols`), applying load-time env gates and
/// `--option`s in the correct order.  `default_listing` injects the Listing
/// default of the `decompile-all` surface (decbench F1, DIV-15) — `true` from
/// [`decompile_all`], `false` from [`list_functions`] (enumeration stays cheap).
pub(crate) fn load_program(args: &Args, default_listing: bool) -> Result<ConsoleProgram, String> {
    let binary = std::fs::canonicalize(&args.binary)
        .map_err(|_| format!("binary not found: {}", args.binary))?
        .to_string_lossy()
        .into_owned();
    // Load-time loader gates are read by `bootstrap_from_object` itself, so they
    // must be exported BEFORE it runs (the same gates `kuna decompile` threads to
    // the subprocess env).
    apply_loadtime_env(&args.options, args.slice.as_deref());

    let spec_roots = spec_roots(args.sleighpath.as_deref());
    let target = args.target.as_deref().unwrap_or("");
    let mut prog = bootstrap_from_object(&binary, target, &spec_roots)
        .map_err(|e| format!("could not build an architecture for {binary}: {}", e.explain()))?;

    // (kuna, decbench F1) Default the program-wide Listing ON for the
    // `decompile-all` surface, unless the caller set it explicitly
    // (`--option listing on|off` still wins — the injection is skipped whenever
    // the caller names `listing` at all).  The Listing feeds the default-on
    // `noreturn_propagate` consumer (the angr-style call-graph no-return
    // fixpoint, DIV-14): without it the pass is a structural no-op, so a call
    // to an unnamed internal exit/fatal wrapper in a STRIPPED binary is treated
    // as returning and the decompiler runs past it, swallowing every following
    // function into the caller (the decbench `noreturn-propagation-stripped`
    // family, e.g. coreutils `xalloc_die`: 118 LOC / 2 gotos swallowed vs the
    // true 4-instruction body).  Only this driver changes: the engine default
    // (`analysis_listing = false`) and the subprocess surfaces (`kuna
    // decompile` → `decomp_dbg`, the datatest harness) are untouched.  `kuna
    // functions` also keeps the engine default: its default output cannot
    // change (no-return facts never add or remove entries — only the
    // default-OFF `aif`/`fid` consumers do that) and the Listing build would
    // turn the cheap enumeration into a whole-program decode (measured 0.21 s →
    // 5.7 s on a stripped tar).  See DIV-15 (`docs/divergences.md`).
    if default_listing && !args.options.iter().any(|(name, _)| name == "listing") {
        prog.arch_mut()
            .set_kuna_option("listing", "on")
            .map_err(|e| format!("option listing: {}", e.explain()))?;
    }

    // (kuna, decbench ARM) Oracle 5 — the always-on prologue-pattern scan folded into
    // function discovery — is x86-64-only, so on a STRIPPED **non-x86-64** binary the ELF
    // entry point is the ONLY discovered function (ARM Cortex-M `betaflight`: 1 of ~469;
    // it has no recursive-descent Listing sweep at the analyzer tier).  The
    // `funcstart_patterns` pass IS the primary discovery source there — it applies the
    // full ARM/AArch64/MIPS/PPC/RISC-V `<patternpairs>` (pre/post) prologue matcher over
    // the code — so default it ON for non-x86-64 on the `decompile-all` surface (the
    // benchmark/LLM path), unless the caller named it explicitly.  x86-64 keeps it OFF
    // (oracle 5 + the entry oracles suffice, and the aggressive scan can over-produce
    // there); only this driver changes, the engine default
    // (`analysis_funcstart_patterns = false`) and the console/datatest surfaces are
    // untouched.  See DIV-20 (`docs/divergences.md`).
    if default_listing && !args.options.iter().any(|(name, _)| name == "funcstart_patterns") {
        let non_x86_64 = std::fs::read(&binary)
            .ok()
            .and_then(|b| {
                object::File::parse(&*b).ok().map(|f| f.architecture() != object::Architecture::X86_64)
            })
            .unwrap_or(false);
        if non_x86_64 {
            prog.arch_mut()
                .set_kuna_option("funcstart_patterns", "on")
                .map_err(|e| format!("option funcstart_patterns: {}", e.explain()))?;
        }
    }

    // (kuna, decbench ARM) `funcstart_patterns` above only seeds a candidate when a matching
    // EPILOGUE prepattern (Ghidra `<patternpairs>`) sits immediately before it, so ~70% of a
    // stripped Cortex-M firmware's functions — those preceded by literal pools / data /
    // padding and living in call-graph components reachable only through indirect calls /
    // function-pointer tables — are never seeded, and the recursive-descent walk (direct
    // CALL/BL only) structurally cannot reach them (crazyflie: 87% of the missed functions
    // have NO direct-call edge from what kuna found).  The ported Aggressive Instruction
    // Finder (`aif`, Ghidra `ArmAggressiveInstructionFinderAnalyzer`) gap-walks the UNDEFINED
    // regions the walk left uncovered, gating each candidate on a prologue-fingerprint
    // histogram learned from the already-discovered functions + `check_valid_subroutine`, so
    // it bridges those disconnected components.  Default it ON for non-x86-64 on the
    // `decompile-all` surface alongside `funcstart_patterns` (crazyflie cf2.elf 1430 -> 2700
    // functions, 45% -> 82% of angr's discovered set), unless the caller named it.  x86-64
    // keeps it OFF (the entry+prologue oracles suffice and the aggressive gap-walk can
    // over-produce there); only this driver changes — the engine default (`analysis_aif =
    // false`) and the console/datatest surfaces are untouched.  Extra non-ground-truth
    // functions are harmless to the GED benchmark (it scores per ground-truth function, matched
    // by name).  See DIV-20 (`docs/divergences.md`).
    if default_listing && !args.options.iter().any(|(name, _)| name == "aif") {
        let non_x86_64 = std::fs::read(&binary)
            .ok()
            .and_then(|b| {
                object::File::parse(&*b).ok().map(|f| f.architecture() != object::Architecture::X86_64)
            })
            .unwrap_or(false);
        if non_x86_64 {
            prog.arch_mut()
                .set_kuna_option("aif", "on")
                .map_err(|e| format!("option aif: {}", e.explain()))?;
        }
    }

    // Analysis-/printer-tier `--option`s must be applied to the architecture
    // BEFORE the gated analysis commit (the `option` < `read symbols` ordering
    // the script path enforces), so a per-pass gate takes effect.
    apply_runtime_options(&mut prog, &args.options)?;
    prog.commit_pending_analysis()
        .map_err(|e| format!("read symbols (analysis commit) failed: {}", e.explain()))?;
    Ok(prog)
}

/// Build the target `(name, Address)` list from the filters: `--addr` entries
/// (named via the symbol table, else `name_function`), `--functions` names, or —
/// with no filter — every enumerated function.
pub(crate) fn resolve_targets(
    prog: &ConsoleProgram,
    args: &Args,
) -> Result<Vec<(String, Address)>, String> {
    let code_space = prog
        .arch()
        .manage()
        .get_default_code_space()
        .ok_or("no default code space")?;

    // The full enumerated set as (name -> Address), used by the name filter and
    // the no-filter default.
    let all: Vec<(String, Address)> = {
        let mut v: Vec<(String, Address)> =
            prog.function_entries().map(|(n, a)| (n.to_string(), a.clone())).collect();
        v.sort_by(|a, b| a.1.get_offset().cmp(&b.1.get_offset()).then_with(|| a.0.cmp(&b.0)));
        v.dedup_by(|a, b| a.0 == b.0 && a.1.get_offset() == b.1.get_offset());
        v
    };

    let mut targets: Vec<(String, Address)> = Vec::new();

    // `--addr 0xVMA`: build the Address directly; name it from the symbol table
    // (or the default `FUN_<addr>` name) so the print/proto path has a name.
    for &vma in &args.addrs {
        let addr = Address::new(Rc::clone(code_space), vma);
        let name = prog
            .function_named_at(vma)
            .unwrap_or_else(|| prog.arch().name_function(&addr));
        targets.push((name, addr));
    }

    // `--functions a,b,c`: intersect names with the enumerated set.
    if let Some(names) = &args.names {
        for want in names {
            match all.iter().find(|(n, _)| n == want) {
                Some((n, a)) => targets.push((n.clone(), a.clone())),
                None => eprintln!("warning: no function named {want:?} in {}", args.binary),
            }
        }
    }

    // Dedup the explicitly-selected targets by entry offset: `--addr 0xX` and
    // `--functions f` can resolve to the same function, and decompiling it twice
    // just wastes work + duplicates the JSON entry.
    let mut seen = std::collections::HashSet::new();
    targets.retain(|(_, a)| seen.insert(a.get_offset()));

    // No filter at all ⇒ every function (the `all` set is already deduped by
    // (name, offset), preserving distinct alias names at one address for the
    // decbench name-narrowing).
    if args.addrs.is_empty() && args.names.is_none() {
        targets = all;
    }
    Ok(targets)
}

// --- option / env-gate handling ---------------------------------------------

/// Is `name` a load-time loader gate (read inside `bootstrap_from_object`)?  Such
/// gates are exported as env vars BEFORE the bootstrap; the matching `option`
/// line is still applied afterward (for the catalog record), exactly as
/// `kuna decompile` does.
fn is_loadtime_gate(name: &str) -> bool {
    matches!(name, "relocobjects" | "i386_pie_plt" | "macho-arm64e")
}

/// Export the load-time loader gates (and the Mach-O slice) onto this process's
/// environment before `bootstrap_from_object` reads them — the in-process analog
/// of the `Command::env(...)` calls in `decompile.rs`.
fn apply_loadtime_env(options: &[(String, String)], slice: Option<&str>) {
    if let Some(slice) = slice.filter(|s| !s.trim().is_empty()) {
        std::env::set_var("KUNA_MACHO_SLICE", slice);
    }
    for (name, value) in options {
        match name.as_str() {
            "relocobjects" => {
                let off = matches!(value.trim(), "0" | "off" | "false" | "no" | "OFF");
                std::env::set_var(RELOC_OBJECTS_ENV, if off { "0" } else { "1" });
            }
            "i386_pie_plt" => {
                let on = !matches!(
                    value.trim().to_ascii_lowercase().as_str(),
                    "off" | "0" | "false"
                );
                std::env::set_var("KUNA_I386_PIE_PLT", if on { "on" } else { "off" });
            }
            "macho-arm64e" => {
                if matches!(value.trim().to_ascii_lowercase().as_str(), "on" | "true" | "1" | "yes") {
                    std::env::set_var("KUNA_MACHO_ARM64E", "1");
                }
            }
            _ => {}
        }
    }
}

/// Apply each `--option NAME VALUE` to the live architecture, mirroring the
/// console `option` command (`IfcOption`): kuna stage-model options route to
/// `set_kuna_option`, upstream options to the `OptionDatabase`.  Load-time gates
/// are still applied here (so they are recorded) after their env export above.
fn apply_runtime_options(prog: &mut ConsoleProgram, options: &[(String, String)]) -> Result<(), String> {
    for (name, value) in options {
        if KUNA_OPTION_NAMES.contains(&name.as_str()) {
            prog.arch_mut()
                .set_kuna_option(name, value)
                .map_err(|e| format!("option {name}: {}", e.explain()))?;
            continue;
        }
        let id = prog.registry().find_element(name, 0);
        if id == 0 {
            // A load-time gate may not be a registered upstream option but is a
            // valid kuna gate already handled via env; don't fail on it.
            if is_loadtime_gate(name) {
                continue;
            }
            return Err(format!("unknown option: {name}"));
        }
        let db = OptionDatabase::new();
        db.set(prog.arch_mut(), id, value, "", "")
            .map_err(|e| format!("option {name}: {}", e.explain()))?;
    }
    Ok(())
}

/// The SLEIGH spec roots (an explicit `--sleighpath` wins, else `SLEIGHHOME` +
/// the repo `specs/`), matching `kuna fid`'s resolution.
fn spec_roots(sleighpath: Option<&str>) -> Vec<String> {
    let mut roots: Vec<String> = Vec::new();
    if let Some(p) = sleighpath.filter(|s| !s.is_empty()) {
        roots.push(p.to_string());
        return roots;
    }
    if let Ok(home) = std::env::var("SLEIGHHOME") {
        if !home.is_empty() {
            roots.push(home);
        }
    }
    let specs = paths::specs_dir().to_string_lossy().into_owned();
    if !roots.contains(&specs) {
        roots.push(specs);
    }
    roots
}

// --- output rendering --------------------------------------------------------

/// Build the `decompile-all --json` document.
fn result_json(binary: &str, funcs: &[FuncResult]) -> Json {
    let functions = Json::Array(
        funcs
            .iter()
            .map(|f| {
                let vars = Json::Array(f.variables.iter().map(var_json).collect());
                Json::Object(vec![
                    ("name".into(), Json::Str(f.name.clone())),
                    ("address".into(), Json::Number(f.address.to_string())),
                    ("address_hex".into(), Json::Str(format!("0x{:x}", f.address))),
                    ("size".into(), Json::Number(f.size.to_string())),
                    (
                        "code".into(),
                        f.code.clone().map(Json::Str).unwrap_or(Json::Null),
                    ),
                    (
                        "error".into(),
                        f.error.clone().map(Json::Str).unwrap_or(Json::Null),
                    ),
                    ("variables".into(), vars),
                ])
            })
            .collect(),
    );
    Json::Object(vec![
        ("binary".into(), Json::Str(binary.to_string())),
        ("count".into(), Json::Number(funcs.len().to_string())),
        ("functions".into(), functions),
    ])
}

/// One `VariableInfo`-shaped JSON object (the fields decbench's `type_match`
/// consumes).
fn var_json(v: &VarInfo) -> Json {
    Json::Object(vec![
        ("name".into(), Json::Str(v.name.clone())),
        ("type".into(), Json::Str(v.type_name.clone())),
        (
            "kind".into(),
            Json::Str(if v.is_param { "arg" } else { "stack" }.into()),
        ),
        (
            "arg_index".into(),
            v.arg_index.map(|i| Json::Number(i.to_string())).unwrap_or(Json::Null),
        ),
        (
            "stack_offset".into(),
            v.stack_offset.map(|o| Json::Number(o.to_string())).unwrap_or(Json::Null),
        ),
        ("size".into(), Json::Number(v.size.to_string())),
    ])
}

// --- argument parsing --------------------------------------------------------

/// Expand a decompiler *mode* name (`reliable` | `aggressive`) into its owned
/// `(option, value)` overrides. Callers PREPEND these before the user's
/// `--option` pairs so an explicit `--option` still wins (last-write, matching
/// the console's `mode` then `option` ordering). Errors on an unknown mode.
pub fn mode_override_pairs(mode: &str) -> Result<Vec<(String, String)>, String> {
    match kuna_decomp::modes::mode_overrides(mode) {
        Some(ovr) => Ok(ovr.iter().map(|(o, v)| ((*o).to_string(), (*v).to_string())).collect()),
        None => {
            let known: Vec<&str> = kuna_decomp::modes::mode_names().collect();
            Err(format!("unknown mode {mode:?} (known: {})", known.join(", ")))
        }
    }
}

pub(crate) fn parse_args(argv: &[String], cmd: &str) -> Result<Args, String> {
    let mut binary: Option<String> = None;
    let mut json = false;
    let mut names: Option<Vec<String>> = None;
    let mut addrs: Vec<u64> = Vec::new();
    let mut no_vars = false;
    let mut max_fn_seconds: u64 = 120;
    let mut options: Vec<(String, String)> = Vec::new();
    let mut mode: Option<String> = None;
    let mut slice: Option<String> = None;
    let mut target: Option<String> = None;
    let mut sleighpath: Option<String> = None;

    let mut i = 0;
    while i < argv.len() {
        let a = argv[i].as_str();
        match a {
            "--json" => json = true,
            "--no-vars" => no_vars = true,
            "--functions" => {
                let v = take(argv, &mut i, "--functions")?;
                names = Some(v.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect());
            }
            "--addr" => {
                let v = take(argv, &mut i, "--addr")?;
                addrs.push(parse_hex(&v)?);
            }
            "--max-fn-seconds" if cmd == "decompile-all" || cmd == "decompile-project" => {
                let v = take(argv, &mut i, "--max-fn-seconds")?;
                max_fn_seconds = v
                    .trim()
                    .parse::<u64>()
                    .map_err(|_| format!("invalid --max-fn-seconds value {v:?}"))?;
            }
            "--option" => {
                if i + 2 >= argv.len() {
                    return Err("--option requires NAME VALUE".into());
                }
                options.push((argv[i + 1].clone(), argv[i + 2].clone()));
                i += 2;
            }
            "--mode" => mode = Some(take(argv, &mut i, "--mode")?),
            "--slice" => slice = Some(take(argv, &mut i, "--slice")?),
            "--target" => target = Some(take(argv, &mut i, "--target")?),
            "--sleighpath" => sleighpath = Some(take(argv, &mut i, "--sleighpath")?),
            "-h" | "--help" => {
                if cmd == "functions" {
                    usage_functions();
                } else {
                    usage_decompile_all();
                }
                std::process::exit(0);
            }
            s if s.starts_with("--") => return Err(format!("unknown option {s}")),
            _ => {
                if binary.is_none() {
                    binary = Some(a.to_string());
                } else {
                    return Err(format!("unexpected argument {a:?}"));
                }
            }
        }
        i += 1;
    }

    let binary = binary.ok_or_else(|| format!("{cmd} requires <binary>"))?;

    // Expand a selected `--mode` into its option overrides, PREPENDED so an
    // explicit `--option` still wins (last-write). Every downstream consumer
    // (`apply_loadtime_env`, the listing/funcstart auto-inject skips,
    // `apply_runtime_options`) reads `args.options`, so this is the single wire
    // point for both `decompile-all` and `functions`.
    if let Some(m) = mode {
        let mut merged = mode_override_pairs(&m)?;
        merged.extend(options);
        options = merged;
    }

    Ok(Args { binary, json, names, addrs, no_vars, max_fn_seconds, options, slice, target, sleighpath })
}

fn take(argv: &[String], i: &mut usize, flag: &str) -> Result<String, String> {
    if *i + 1 < argv.len() {
        *i += 1;
        Ok(argv[*i].clone())
    } else {
        Err(format!("{flag} requires a value"))
    }
}

fn parse_hex(s: &str) -> Result<u64, String> {
    let t = s.trim();
    let body = t.strip_prefix("0x").or_else(|| t.strip_prefix("0X")).unwrap_or(t);
    u64::from_str_radix(body, 16).map_err(|_| format!("invalid address {s:?}"))
}

fn usage_decompile_all() {
    eprintln!(
        "usage: kuna decompile-all <binary> [--json] [--functions a,b,..] [--addr 0xVMA].. \\\n\
         \x20                   [--no-vars] [--max-fn-seconds N] [--mode reliable|aggressive] \\\n\
         \x20                   [--option N V].. [--slice ARCH] [--target T] [--sleighpath D]\n\
         \n\
         Decompile every function of a binary in one in-process load (load-once,\n\
         decompile-many).  --json emits {{binary,count,functions:[{{name,address,code,variables,..}}]}};\n\
         without it, concatenated C with `// Function:` headers.\n\
         --max-fn-seconds N caps ONE function's decompile at N seconds (default 120,\n\
         0 disables); a function over budget becomes its own `error` record and the\n\
         batch continues (the stripped-ELF hang watchdog, see tests/hang-repro/).\n\
         The Listing analysis tier is ON by default on this surface (so the\n\
         no-return call-graph fixpoint `noreturn_propagate` fires on stripped\n\
         binaries); opt out with `--option listing off`."
    );
}

fn usage_functions() {
    eprintln!(
        "usage: kuna functions <binary> [--json] [--mode reliable|aggressive] [--slice ARCH] [--target T] [--sleighpath D]\n\
         \n\
         List every function kuna discovers in a binary as `<addr>\\t<name>` (or\n\
         --json: {{binary,count,functions:[{{name,address}}]}})."
    );
}
