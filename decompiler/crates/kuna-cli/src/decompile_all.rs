//! `kuna decompile-all` / `kuna functions` — in-process **whole-binary**
//! decompilation.
//!
//! ```text
//!   kuna decompile-all <binary> [--json] [--functions a,b,..] [--addr 0xVMA].. \
//!                       [--no-vars] [--max-fn-seconds N] [--mode MODE] \
//!                       [--option N V].. \
//!                       [--slice ARCH] [--target T] [--sleighpath D]
//!   kuna functions <binary> [--json] [--mode MODE] [--slice ARCH] [--target T]
//!                  [--sleighpath D]
//! ```
//!
//! Unlike `kuna decompile` (which spawns a fresh `decomp_dbg` subprocess **per
//! function** — re-parsing the SLEIGH spec and re-running the whole-binary
//! analysis tier every time), this loads and analyzes the binary **once**
//! in-process (`bootstrap_from_object` → `commit_pending_analysis`, i.e. the
//! `load file` + `read symbols` seam), then loops `decompile` + `print C` over
//! every executable entry. The marginal per-function cost drops from a full image load
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
//! Omitted `--mode` resolves the size-based `auto` policy. Under its concrete
//! `reliable` preset, the decompile surface injects `--option listing on`
//! (decbench F1, DIV-15) unless the caller names `listing`, so the default-on
//! `noreturn_propagate` consumer fires and an unnamed internal exit/fatal
//! wrapper cannot swallow following functions. `aggressive` names Listing on;
//! `fast` names it off. A later explicit `--option` always wins.

use std::ffi::{OsStr, OsString};
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_console::engine::{bootstrap_from_object, ConsoleProgram, FunctionEntry};
// The decompile loop + result shape live in the shared decompile-project core
// (`kuna_console::project` — also reused by the `kuna_wasm` front-end).
use kuna_console::project::{
    decompile_targets, default_fn_budget_seconds, render_c, FuncResult,
};
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
    /// `--functions a,b,c`: restrict to these names (None ⇒ CODE-backed entries).
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
    /// stripped-ELF non-convergence hang (`tests/hang-repro/`). Defaults to 10
    /// for an unfiltered fast whole-binary run and 120 otherwise.
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
                        .map(|e| {
                            let a = e.addr.get_offset();
                            Json::Object(vec![
                                ("name".into(), Json::Str(e.name.clone())),
                                ("address".into(), Json::Number(a.to_string())),
                                ("address_hex".into(), Json::Str(format!("0x{a:x}"))),
                                ("aliases".into(), aliases_json(&e.aliases)),
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
                for e in &entries {
                    // Alias names follow the canonical one on the same line, so
                    // the plain listing stays one line per function.
                    let extra = if e.aliases.is_empty() {
                        String::new()
                    } else {
                        format!("\t({})", e.aliases.join(", "))
                    };
                    println!("0x{:x}\t{}{extra}", e.addr.get_offset(), e.name);
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

    // Per-function watchdog (`--max-fn-seconds`, default 10 for an unfiltered
    // fast batch and 120 otherwise, 0 disables): driver policy, not a
    // stage-model option — the decompile drive arms a cooperative deadline
    // from this budget for EACH function, so one pathological function becomes
    // a per-function `error` record instead of hanging the whole batch.
    if args.max_fn_seconds > 0 {
        prog.arch_mut().kuna_fn_budget =
            Some(std::time::Duration::from_secs(args.max_fn_seconds));
    }

    Ok(decompile_targets(&mut prog, targets, args.no_vars, /* want_proto= */ false))
}

/// Enumerate the program's full callable-symbol inventory, one
/// [`FunctionEntry`] per entry address (the `functions` command).
fn list_functions(args: &Args) -> Result<Vec<FunctionEntry>, String> {
    let prog = load_program(args, /* default_listing= */ false)?;
    // One record per entry address, address-ordered, alias names carried as data
    // (issue #197 — this used to dedup by (address, name), so a function the
    // loader and an analysis pass both named was listed twice).
    Ok(prog.function_entries_canonical())
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
    // the subprocess env). Keep the restoration guard alive through runtime
    // option recording too: `relocobjects` and `i386_pie_plt` update their env
    // bridges again inside `set_kuna_option` and must not leak into a later load.
    let _loadtime_env = apply_loadtime_env(&args.options, args.slice.as_deref());

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
    // decompile` → `decomp_dbg`, the datatest harness) are untouched. `kuna
    // functions` does not add this driver fallback: no-return facts never add
    // or remove entries, while the Listing build would turn cheap enumeration
    // into a whole-program decode (measured 0.21 s → 5.7 s on a stripped tar).
    // A selected mode can still name Listing explicitly. See DIV-15
    // (`docs/divergences.md`).
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

/// Build the target [`FunctionEntry`] list from the filters: `--addr` entries
/// (the canonical record at that address, else a record named via the symbol
/// table / `name_function`), `--functions` names or aliases, or — with no filter
/// — every CODE-backed entry, each exactly once.
pub(crate) fn resolve_targets(
    prog: &ConsoleProgram,
    args: &Args,
) -> Result<Vec<FunctionEntry>, String> {
    let code_space = prog
        .arch()
        .manage()
        .get_default_code_space()
        .ok_or("no default code space")?;

    let mut targets: Vec<FunctionEntry> = Vec::new();

    // `--addr 0xVMA`: build the Address directly; name it from the symbol table
    // (or the default `FUN_<addr>` name) so the print/proto path has a name.
    // Prefer the canonical record when the enumeration knows this entry, so an
    // explicitly-addressed function reports the same name + aliases it would in
    // a whole-binary run.
    for &vma in &args.addrs {
        match prog.find_entry_at(vma) {
            Some(e) => targets.push(e),
            None => {
                let addr = Address::new(Rc::clone(code_space), vma);
                let name = prog
                    .function_named_at(vma)
                    .unwrap_or_else(|| prog.arch().name_function(&addr));
                targets.push(FunctionEntry { name, addr, aliases: Vec::new() });
            }
        }
    }

    // `--functions a,b,c`: intersect names with the enumerated set.  An ALIAS
    // resolves too — collapsing the enumeration must not make a name that used
    // to select a function stop working (the decbench name-narrowing looks up
    // generated `sub_<addr>` names).
    if let Some(names) = &args.names {
        for want in names {
            match prog.find_entry_by_name(want) {
                Some(e) => targets.push(e),
                None => eprintln!("warning: no function named {want:?} in {}", args.binary),
            }
        }
    }

    // Dedup the explicitly-selected targets by entry offset: `--addr 0xX` and
    // `--functions f` can resolve to the same function, and decompiling it twice
    // just wastes work + duplicates the JSON entry.
    let mut seen = std::collections::HashSet::new();
    targets.retain(|e| seen.insert(e.addr.get_offset()));

    // No filter at all ⇒ every executable function, exactly once. Import pointer
    // slots remain explicitly selectable, but are data rather than function
    // bodies.
    if args.addrs.is_empty() && args.names.is_none() {
        targets = prog.function_entries_executable();
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

fn last_option_value<'a>(options: &'a [(String, String)], name: &str) -> Option<&'a str> {
    options
        .iter()
        .rev()
        .find(|(option_name, _)| option_name == name)
        .map(|(_, value)| value.as_str())
}

#[derive(Default)]
struct LoadtimeEnv {
    previous: Vec<(&'static str, Option<OsString>)>,
}

impl LoadtimeEnv {
    fn set(&mut self, name: &'static str, value: impl AsRef<OsStr>) {
        self.previous.push((name, std::env::var_os(name)));
        std::env::set_var(name, value);
    }

    fn remove(&mut self, name: &'static str) {
        self.previous.push((name, std::env::var_os(name)));
        std::env::remove_var(name);
    }
}

impl Drop for LoadtimeEnv {
    fn drop(&mut self) {
        for (name, previous) in self.previous.drain(..).rev() {
            if let Some(value) = previous {
                std::env::set_var(name, value);
            } else {
                std::env::remove_var(name);
            }
        }
    }
}

/// Export the load-time loader gates (and the Mach-O slice) onto this process's
/// environment before `bootstrap_from_object` reads them — the in-process analog
/// of the `Command::env(...)` calls in `decompile.rs`.
fn apply_loadtime_env(options: &[(String, String)], slice: Option<&str>) -> LoadtimeEnv {
    let mut env = LoadtimeEnv::default();
    if let Some(slice) = slice.filter(|s| !s.trim().is_empty()) {
        env.set("KUNA_MACHO_SLICE", slice);
    }

    if let Some(value) = last_option_value(options, "relocobjects") {
        let off = matches!(
            value.trim().to_ascii_lowercase().as_str(),
            "0" | "off" | "false" | "no"
        );
        env.set(RELOC_OBJECTS_ENV, if off { "0" } else { "1" });
    }
    if let Some(value) = last_option_value(options, "i386_pie_plt") {
        let on = !matches!(
            value.trim().to_ascii_lowercase().as_str(),
            "off" | "0" | "false"
        );
        env.set("KUNA_I386_PIE_PLT", if on { "on" } else { "off" });
    }
    if let Some(value) = last_option_value(options, "macho-arm64e") {
        if matches!(
            value.trim().to_ascii_lowercase().as_str(),
            "on" | "true" | "1" | "yes"
        ) {
            env.set("KUNA_MACHO_ARM64E", "1");
        } else {
            env.remove("KUNA_MACHO_ARM64E");
        }
    }
    env
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
                    ("aliases".into(), aliases_json(&f.aliases)),
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

/// (kuna, issue #197) The `aliases` array: every OTHER name the reported entry
/// carries.  Always present (`[]` when the entry has exactly one name) so a
/// consumer can read the field unconditionally.  Additive — no existing field
/// changes shape, and the names that used to appear as extra top-level records
/// are all still here, one level down.
fn aliases_json(aliases: &[String]) -> Json {
    Json::Array(aliases.iter().map(|a| Json::Str(a.clone())).collect())
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

/// Expand a concrete decompiler mode into its owned `(option, value)`
/// overrides. Callers PREPEND these before the user's `--option` pairs so an
/// explicit `--option` still wins (last-write, matching the console's `mode`
/// then `option` ordering). `auto` must first be resolved from binary metadata
/// by [`mode_options_for_binary`].
pub fn mode_override_pairs(mode: &str) -> Result<Vec<(String, String)>, String> {
    if kuna_decomp::modes::mode_is_automatic(mode) {
        return Err("mode `auto` requires input binary size".into());
    }
    match kuna_decomp::modes::mode_overrides(mode) {
        Some(ovr) => Ok(ovr.iter().map(|(o, v)| ((*o).to_string(), (*v).to_string())).collect()),
        None => {
            let known: Vec<&str> = kuna_decomp::modes::mode_names().collect();
            Err(format!("unknown mode {mode:?} (known: {})", known.join(", ")))
        }
    }
}

/// Resolve an omitted or explicit `auto` mode from `binary_size`; explicit
/// concrete modes ignore the size.
#[cfg(test)]
fn mode_override_pairs_for_size(
    mode: Option<&str>,
    binary_size: u64,
) -> Result<Vec<(String, String)>, String> {
    let concrete = kuna_decomp::modes::resolve_mode_for_size(mode, binary_size).ok_or_else(|| {
        let requested = mode.unwrap_or("auto");
        let known: Vec<&str> = kuna_decomp::modes::mode_names().collect();
        format!("unknown mode {requested:?} (known: {})", known.join(", "))
    })?;
    mode_override_pairs(concrete)
}

/// Resolve the frontend mode policy and prepend its overrides to the user's
/// explicit options. Omission is `auto`; file metadata is only read for an
/// omitted or explicitly automatic mode.
pub(crate) fn mode_options_for_binary(
    mode: Option<&str>,
    binary: &str,
    explicit: Vec<(String, String)>,
) -> Result<Vec<(String, String)>, String> {
    Ok(mode_and_options_for_binary(mode, binary, explicit)?.1)
}

fn mode_and_options_for_binary(
    mode: Option<&str>,
    binary: &str,
    explicit: Vec<(String, String)>,
) -> Result<(&'static str, Vec<(String, String)>), String> {
    let concrete = concrete_mode_for_binary(mode, binary)?;
    let mut merged = mode_override_pairs(concrete)?;
    merged.extend(explicit);
    Ok((concrete, merged))
}

fn concrete_mode_for_binary(
    mode: Option<&str>,
    binary: &str,
) -> Result<&'static str, String> {
    let requested = mode.unwrap_or("auto");
    if kuna_decomp::modes::mode_is_automatic(requested) {
        let size = std::fs::metadata(binary)
            .map_err(|e| format!("cannot read input binary metadata for mode auto: {binary}: {e}"))?
            .len();
        kuna_decomp::modes::resolve_mode_for_size(Some(requested), size).ok_or_else(|| {
            let known: Vec<&str> = kuna_decomp::modes::mode_names().collect();
            format!("unknown mode {requested:?} (known: {})", known.join(", "))
        })
    } else {
        kuna_decomp::modes::resolve_mode_for_size(Some(requested), 0).ok_or_else(|| {
            let known: Vec<&str> = kuna_decomp::modes::mode_names().collect();
            format!("unknown mode {requested:?} (known: {})", known.join(", "))
        })
    }
}

pub(crate) fn parse_args(argv: &[String], cmd: &str) -> Result<Args, String> {
    let mut binary: Option<String> = None;
    let mut json = false;
    let mut names: Option<Vec<String>> = None;
    let mut addrs: Vec<u64> = Vec::new();
    let mut no_vars = false;
    let mut max_fn_seconds: Option<u64> = None;
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
                max_fn_seconds = Some(
                    v.trim()
                        .parse::<u64>()
                        .map_err(|_| format!("invalid --max-fn-seconds value {v:?}"))?,
                );
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

    let explicit_fast_funcdisc = options.iter().any(|(name, _)| name == "fast_funcdisc");
    // Omitted mode is the size-driven `auto` policy. Mode overrides are
    // PREPENDED so an explicit `--option` still wins (last-write). Every
    // downstream consumer (`apply_loadtime_env`, the listing/funcstart
    // auto-inject skips, `apply_runtime_options`) reads `args.options`, so this
    // is the single wire point for decompile-all, decompile-project, and
    // functions.
    let (concrete_mode, merged) =
        mode_and_options_for_binary(mode.as_deref(), &binary, options)?;
    options = merged;
    if names.is_none() && !addrs.is_empty() && !explicit_fast_funcdisc {
        options.push(("fast_funcdisc".into(), "off".into()));
    }
    let whole_binary = (cmd == "decompile-all" || cmd == "decompile-project")
        && names.is_none()
        && addrs.is_empty();
    let max_fn_seconds = max_fn_seconds
        .unwrap_or_else(|| default_fn_budget_seconds(concrete_mode, whole_binary));

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
         \x20                   [--no-vars] [--max-fn-seconds N] [--mode auto|reliable|aggressive|fast] \\\n\
         \x20                   [--option N V].. [--slice ARCH] [--target T] [--sleighpath D]\n\
         \n\
         Decompile every CODE-backed function in one in-process load (load-once,\n\
         decompile-many).  --json emits {{binary,count,functions:[{{name,address,code,variables,..}}]}};\n\
         without it, concatenated C with `// Function:` headers.\n\
         --max-fn-seconds N caps ONE function's decompile at N seconds (default 10\n\
         for unfiltered fast runs, 120 otherwise; 0 disables); a function over\n\
         budget becomes its own `error` record and the batch continues.\n\
         Omitted --mode uses auto: aggressive below 500 KiB, reliable below\n\
         2 MiB, and fast at 2 MiB or larger. Explicit --option values win."
    );
}

fn usage_functions() {
    eprintln!(
        "usage: kuna functions <binary> [--json] [--mode auto|reliable|aggressive|fast] [--slice ARCH] [--target T] [--sleighpath D]\n\
         \n\
         List every function kuna discovers in a binary as `<addr>\\t<name>` (or\n\
         --json: {{binary,count,functions:[{{name,address}}]}})."
    );
}

#[cfg(test)]
mod mode_tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    static TEMP_ID: AtomicU64 = AtomicU64::new(0);

    fn sparse_binary(size: u64) -> std::path::PathBuf {
        let id = TEMP_ID.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "kuna-auto-mode-{}-{id}.bin",
            std::process::id()
        ));
        let file = std::fs::File::create(&path).expect("create auto-mode fixture");
        file.set_len(size).expect("size auto-mode fixture");
        path
    }

    #[test]
    fn omitted_and_explicit_auto_select_the_same_concrete_presets() {
        for (size, concrete) in [
            (0, "aggressive"),
            (kuna_decomp::modes::AUTO_RELIABLE_MIN_BYTES, "reliable"),
            (kuna_decomp::modes::AUTO_FAST_MIN_BYTES, "fast"),
        ] {
            let expected = mode_override_pairs(concrete).unwrap();
            assert_eq!(mode_override_pairs_for_size(None, size).unwrap(), expected);
            assert_eq!(mode_override_pairs_for_size(Some("auto"), size).unwrap(), expected);
        }
    }

    #[test]
    fn explicit_concrete_mode_ignores_binary_metadata() {
        let missing = std::env::temp_dir().join(format!(
            "kuna-auto-mode-missing-{}.bin",
            std::process::id()
        ));
        let options =
            mode_options_for_binary(Some("fast"), missing.to_str().unwrap(), Vec::new()).unwrap();
        assert_eq!(options, mode_override_pairs("fast").unwrap());
    }

    #[test]
    fn shared_parsers_default_to_auto_and_keep_user_options_last() {
        let path = sparse_binary(kuna_decomp::modes::AUTO_FAST_MIN_BYTES);
        let binary = path.to_string_lossy().into_owned();
        for cmd in ["decompile-all", "decompile-project", "functions"] {
            let argv = vec![
                binary.clone(),
                "--option".into(),
                "listing".into(),
                "on".into(),
            ];
            let args = parse_args(&argv, cmd).unwrap();
            let listing: Vec<&str> = args
                .options
                .iter()
                .filter(|(name, _)| name == "listing")
                .map(|(_, value)| value.as_str())
                .collect();
            assert_eq!(listing, vec!["off", "on"], "{cmd} precedence");
        }
        std::fs::remove_file(path).expect("remove auto-mode fixture");
    }

    #[test]
    fn unfiltered_fast_batches_default_to_ten_seconds_per_function() {
        let path = sparse_binary(kuna_decomp::modes::AUTO_FAST_MIN_BYTES);
        let binary = path.to_string_lossy().into_owned();
        for cmd in ["decompile-all", "decompile-project"] {
            let args = parse_args(std::slice::from_ref(&binary), cmd).unwrap();
            assert_eq!(
                args.max_fn_seconds,
                kuna_console::project::FAST_WHOLE_BINARY_FN_BUDGET_SECONDS,
                "{cmd}"
            );

            let selected = parse_args(
                &[
                    binary.clone(),
                    "--addr".into(),
                    "0x1234".into(),
                    "--mode".into(),
                    "fast".into(),
                ],
                cmd,
            )
            .unwrap();
            assert_eq!(
                selected.max_fn_seconds,
                kuna_console::project::DEFAULT_FN_BUDGET_SECONDS,
                "{cmd} selected"
            );
            let named = parse_args(
                &[
                    binary.clone(),
                    "--functions".into(),
                    "sub_1234".into(),
                    "--mode".into(),
                    "fast".into(),
                ],
                cmd,
            )
            .unwrap();
            assert_eq!(
                named.max_fn_seconds,
                kuna_console::project::DEFAULT_FN_BUDGET_SECONDS,
                "{cmd} named"
            );

            let disabled = parse_args(
                &[
                    binary.clone(),
                    "--mode".into(),
                    "fast".into(),
                    "--max-fn-seconds".into(),
                    "0".into(),
                ],
                cmd,
            )
            .unwrap();
            assert_eq!(disabled.max_fn_seconds, 0, "{cmd} explicit override");
            let explicit = parse_args(
                &[
                    binary.clone(),
                    "--mode".into(),
                    "fast".into(),
                    "--max-fn-seconds".into(),
                    "17".into(),
                ],
                cmd,
            )
            .unwrap();
            assert_eq!(explicit.max_fn_seconds, 17, "{cmd} explicit budget");
        }

        for mode in ["reliable", "aggressive"] {
            let args = parse_args(
                &[binary.clone(), "--mode".into(), mode.into()],
                "decompile-all",
            )
            .unwrap();
            assert_eq!(
                args.max_fn_seconds,
                kuna_console::project::DEFAULT_FN_BUDGET_SECONDS,
                "{mode}"
            );
        }
        let functions = parse_args(&[binary], "functions").unwrap();
        assert_eq!(
            functions.max_fn_seconds,
            kuna_console::project::DEFAULT_FN_BUDGET_SECONDS
        );
        std::fs::remove_file(path).expect("remove auto-mode fixture");
    }

    #[test]
    fn address_selection_skips_preset_discovery_but_names_do_not() {
        let path = sparse_binary(kuna_decomp::modes::AUTO_FAST_MIN_BYTES);
        let binary = path.to_string_lossy().into_owned();
        let address_args = parse_args(
            &[
                binary.clone(),
                "--addr".into(),
                "0x1234".into(),
                "--mode".into(),
                "fast".into(),
            ],
            "decompile-project",
        )
        .unwrap();
        assert_eq!(
            last_option_value(&address_args.options, "fast_funcdisc"),
            Some("off")
        );

        let explicit_args = parse_args(
            &[
                binary.clone(),
                "--addr".into(),
                "0x1234".into(),
                "--mode".into(),
                "fast".into(),
                "--option".into(),
                "fast_funcdisc".into(),
                "on".into(),
            ],
            "decompile-project",
        )
        .unwrap();
        assert_eq!(
            last_option_value(&explicit_args.options, "fast_funcdisc"),
            Some("on")
        );

        let named_args = parse_args(
            &[
                binary,
                "--functions".into(),
                "sub_1234".into(),
                "--mode".into(),
                "fast".into(),
            ],
            "decompile-project",
        )
        .unwrap();
        assert_eq!(
            last_option_value(&named_args.options, "fast_funcdisc"),
            Some("on")
        );
        std::fs::remove_file(path).expect("remove auto-mode fixture");
    }

    #[test]
    fn loadtime_gates_use_the_last_named_option() {
        let options = vec![
            ("relocobjects".into(), "on".into()),
            ("i386_pie_plt".into(), "on".into()),
            ("macho-arm64e".into(), "on".into()),
            ("relocobjects".into(), "off".into()),
            ("i386_pie_plt".into(), "off".into()),
            ("macho-arm64e".into(), "off".into()),
        ];
        assert_eq!(last_option_value(&options, "relocobjects"), Some("off"));
        assert_eq!(last_option_value(&options, "i386_pie_plt"), Some("off"));
        assert_eq!(last_option_value(&options, "macho-arm64e"), Some("off"));
    }
}
