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
//! The per-function decompile runs the *same* step as the console `decompile`
//! command (`IfcDecompile`) — one shared
//! `kuna_console::decompile_step::decompile_one`, so the two surfaces cannot
//! drift again (DIV-66; they had, and this one was the weaker). It re-seeds the
//! function's DWARF stack locals via [`ConsoleProgram::dwarf_locals_for`] (so a
//! `-g` binary renders DWARF names), and a per-function pipeline abort (the
//! decompile drive already catches panics / un-ported seams and returns `Err`) is
//! recorded as that function's `error` rather than aborting the whole binary.
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
//!
//! Both surfaces share ONE discovery policy ([`DriverDefaults`], DIV-68): the
//! non-x86-64 `funcstart_patterns` + `aif` defaults (DIV-20) and the Listing that
//! gates them are injected for `functions` exactly as for `decompile-all`, so the
//! inventory can never omit an entry the whole-binary run decompiles.
//!
//! An unfiltered run that discovers ZERO functions **fails** — non-zero exit, the
//! reason on stderr and in the document's run-level `error` field — because a
//! silent `count: 0` is indistinguishable from a file that genuinely has no
//! functions, and the caller acts on the difference. [`zero_discovery_error`]
//! draws the line at executable content, so a data-only object still answers
//! with an honest empty inventory and exit 0.
//!
//! [`render_result_json`] and [`decompile_entries`] are also `kuna decompile
//! --json`'s (`decompile.rs`) — one schema and one decompile policy across the
//! single-function and whole-binary surfaces.

use kuna_console::engine::{
    bootstrap_from_object, ConsoleProgram, EntryLookupError, EntrySelector, FunctionEntry,
    ObjectLocation,
};
use std::ffi::{OsStr, OsString};
use std::fmt::Write as _;
// The decompile loop + result shape live in the shared decompile-project core
// (`kuna_console::project` — also reused by the `kuna_wasm` front-end).
use kuna_console::project::{
    decompile_targets, default_fn_budget_seconds, render_c, FuncResult,
};
// `File::architecture()` (the ARM-discovery default, decbench) plus the
// section/segment walks the zero-discovery diagnosis reads.
use object::{Object, ObjectSection, ObjectSegment};
use kuna_decomp::decompile_drive::{LineMapping, VarInfo};
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
    /// `--addr 0xVMA|.section+0xOFFSET|SECTION_INDEX:0xOFFSET` (repeatable).
    /// Combined with `--functions` if both are given.
    pub(crate) addrs: Vec<EntrySelector>,
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
            // An UNFILTERED run that produced nothing discovered nothing, which
            // is a failed run wearing a successful one's clothes. A run narrowed
            // by `--functions`/`--addr` that matched nothing is a different
            // condition (already warned about, per target) and keeps its status.
            let unfiltered = args.names.is_none() && args.addrs.is_empty();
            let discovery_error = (funcs.is_empty() && unfiltered)
                .then(|| zero_discovery_error(&args.binary))
                .flatten();
            let text = if args.json {
                render_result_json(&args.binary, &funcs, &args.options, discovery_error.as_deref())
            } else {
                render_c(&funcs)
            };
            emit_with_discovery_error(&text, discovery_error.as_deref())
        }
        Err(e) => {
            eprintln!("error: {e}");
            1
        }
    }
}

/// Emit `text`, then report a discovery failure (stdout before stderr, as
/// `kuna decompile` orders them) and answer with the run's verdict.
fn emit_with_discovery_error(text: &str, discovery_error: Option<&str>) -> i32 {
    let status = crate::output::emit_with_status(text, i32::from(discovery_error.is_some()));
    if let Some(message) = discovery_error {
        eprintln!("error: {message}");
    }
    status
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
            // `functions` takes no selection filter, so an empty inventory IS a
            // total discovery failure.
            let discovery_error = entries
                .is_empty()
                .then(|| zero_discovery_error(&args.binary))
                .flatten();
            let text = if args.json {
                functions_json(&args.binary, &entries, discovery_error.as_deref())
            } else {
                let mut text = String::new();
                for e in &entries {
                    // Alias names follow the canonical one on the same line, so
                    // the plain listing stays one line per function.
                    let extra = if e.aliases.is_empty() {
                        String::new()
                    } else {
                        format!("\t({})", e.aliases.join(", "))
                    };
                    let _ = writeln!(text, "0x{:x}\t{}{extra}", e.addr.get_offset(), e.name);
                }
                text
            };
            emit_with_discovery_error(&text, discovery_error.as_deref())
        }
        Err(e) => {
            eprintln!("error: {e}");
            1
        }
    }
}

/// Load + analyze the binary once, then decompile every selected function.
fn decompile_all(args: &Args) -> Result<Vec<FuncResult>, String> {
    let mut prog = load_program(args, DriverDefaults::Decompile)?;
    let targets = resolve_targets(&prog, args)?;
    Ok(decompile_entries(&mut prog, args, targets))
}

/// Arm the per-function watchdog and run the decompile loop over `targets` — the
/// body `decompile-all` and `kuna decompile --json` share, so the two answer with
/// one policy as well as one schema.
///
/// The watchdog (`--max-fn-seconds`, default 10 for an unfiltered fast batch and
/// 120 otherwise, 0 disables) is driver policy, not a stage-model option: the
/// decompile drive arms a cooperative deadline from this budget for EACH
/// function, so one pathological function becomes a per-function `error` record
/// instead of hanging the whole batch.
pub(crate) fn decompile_entries(
    prog: &mut ConsoleProgram,
    args: &Args,
    targets: Vec<FunctionEntry>,
) -> Vec<FuncResult> {
    if args.max_fn_seconds > 0 {
        prog.arch_mut().kuna_fn_budget =
            Some(std::time::Duration::from_secs(args.max_fn_seconds));
    }

    decompile_targets(
        prog,
        targets,
        args.no_vars,
        /* want_proto= */ false,
        /* want_provenance= */ args.json,
    )
}

/// Enumerate the program's full callable-symbol inventory, one
/// [`FunctionEntry`] per entry address (the `functions` command).
fn list_functions(args: &Args) -> Result<Vec<FunctionEntry>, String> {
    let prog = load_program(args, DriverDefaults::Inventory)?;
    // One record per entry address, address-ordered, alias names carried as data
    // (issue #197 — this used to dedup by (address, name), so a function the
    // loader and an analysis pass both named was listed twice).
    Ok(prog.function_entries_canonical())
}

/// Which bundle of driver defaults a surface takes in [`load_program`].
///
/// Both variants take the same DISCOVERY policy (DIV-20/DIV-68); they differ only
/// in whether the Listing is built on an architecture where it discovers nothing.
pub(crate) enum DriverDefaults {
    /// `kuna functions` — enumeration only.
    Inventory,
    /// `kuna decompile-all` / `kuna decompile-project` — enumeration plus bodies.
    Decompile,
}

impl DriverDefaults {
    /// Does this surface decompile the entries it discovers (so the Listing's
    /// no-return facts change its output, not just its inventory)?
    fn decompiles(&self) -> bool {
        matches!(self, DriverDefaults::Decompile)
    }
}

/// Bootstrap the architecture from the binary and run the analysis commit (the
/// in-process `load file` + `read symbols`), applying load-time env gates and
/// `--option`s in the correct order.  `defaults` selects the driver-default
/// bundle (DIV-15/DIV-20/DIV-68): the discovery passes are shared by every
/// surface, the Listing-for-no-return default is the decompiling surfaces'.
pub(crate) fn load_program(
    args: &Args,
    defaults: DriverDefaults,
) -> Result<ConsoleProgram, String> {
    let binary = std::fs::canonicalize(&args.binary)
        .map_err(|_| format!("binary not found: {}", args.binary))?
        .to_string_lossy()
        .into_owned();
    // Load-time loader gates are read by `bootstrap_from_object` itself, so they
    // must be exported BEFORE it runs (the same gates `kuna decompile` threads to
    // the subprocess env). Keep the restoration guard alive through runtime
    // option recording too: `relocobjects`, `i386_pie_plt`, `relocrebase`,
    // `typedepth` and `dwarfstructs` update their env bridges again inside
    // `set_kuna_option` and must not leak into a later load.
    let _loadtime_env = apply_loadtime_env(&args.options, args.slice.as_deref());

    let spec_roots = spec_roots(args.sleighpath.as_deref());
    let target = args.target.as_deref().unwrap_or("");
    let mut prog = bootstrap_from_object(&binary, target, &spec_roots)
        .map_err(|e| format!("could not build an architecture for {binary}: {}", e.explain()))?;

    let named = |name: &str| args.options.iter().any(|(option, _)| option == name);
    // A non-x86-64 image takes the DIV-20 discovery bundle below; read the file
    // once for both of its gates (and for the Listing gate above them).
    let non_x86_64 = std::fs::read(&binary)
        .ok()
        .and_then(|bytes| {
            object::File::parse(&*bytes)
                .ok()
                .map(|file| file.architecture() != object::Architecture::X86_64)
        })
        .unwrap_or(false);

    // (kuna, decbench F1) Default the program-wide Listing ON, unless the caller
    // set it explicitly (`--option listing on|off` still wins — the injection is
    // skipped whenever the caller names `listing` at all).  Two independent
    // reasons, one per surface:
    //
    //   * A DECOMPILING surface needs it on every architecture.  The Listing
    //     feeds the default-on `noreturn_propagate` consumer (the angr-style
    //     call-graph no-return fixpoint, DIV-14): without it the pass is a
    //     structural no-op, so a call to an unnamed internal exit/fatal wrapper
    //     in a STRIPPED binary is treated as returning and the decompiler runs
    //     past it, swallowing every following function into the caller (the
    //     decbench `noreturn-propagation-stripped` family, e.g. coreutils
    //     `xalloc_die`: 118 LOC / 2 gotos swallowed vs the true 4-instruction
    //     body).  See DIV-15.
    //   * On a NON-x86-64 image every surface needs it, `kuna functions`
    //     included, because the Listing is the master gate of the DIV-20
    //     discovery bundle below — `funcstart_patterns` and `aif` both walk the
    //     Listing's code units and are inert without it, and those two passes
    //     ARE the discovery on a stripped ARM/AArch64/MIPS/PPC/RISC-V binary.
    //     See DIV-68.
    //
    // x86-64 enumeration keeps the cheap path: the Listing is measured
    // entry-neutral there (identical entry sets on 40 sampled stripped x86-64
    // ELFs), so building it would only make `kuna functions` slower.  Only this
    // driver changes: the engine default (`analysis_listing = false`) and the
    // subprocess surfaces (`kuna decompile` → `decomp_dbg`, the datatest
    // harness) are untouched, and a selected mode can still name Listing.
    if (defaults.decompiles() || non_x86_64) && !named("listing") {
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
    // the code — so default it ON for non-x86-64 on every whole-binary surface, the
    // `functions` inventory included (DIV-68), unless the caller named it explicitly.
    // x86-64 keeps it OFF (oracle 5 + the entry oracles suffice, and the aggressive scan
    // can over-produce there); only this driver changes, the engine default
    // (`analysis_funcstart_patterns = false`) and the console/datatest surfaces are
    // untouched.  See DIV-20 (`docs/divergences.md`).
    if non_x86_64 && !named("funcstart_patterns") {
        prog.arch_mut()
            .set_kuna_option("funcstart_patterns", "on")
            .map_err(|e| format!("option funcstart_patterns: {}", e.explain()))?;
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
    // it bridges those disconnected components.  Default it ON for non-x86-64 on every
    // whole-binary surface alongside `funcstart_patterns` (crazyflie cf2.elf 1430 -> 2700
    // functions, 45% -> 82% of angr's discovered set), unless the caller named it.  x86-64
    // keeps it OFF (the entry+prologue oracles suffice and the aggressive gap-walk can
    // over-produce there); only this driver changes — the engine default (`analysis_aif =
    // false`) and the console/datatest surfaces are untouched.  Extra non-ground-truth
    // functions are harmless to the GED benchmark (it scores per ground-truth function, matched
    // by name).  See DIV-20 (`docs/divergences.md`).
    if non_x86_64 && !named("aif") {
        prog.arch_mut()
            .set_kuna_option("aif", "on")
            .map_err(|e| format!("option aif: {}", e.explain()))?;
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
    let mut targets: Vec<FunctionEntry> = Vec::new();

    // Resolve every address form through the program's shared selector model.
    for selector in &args.addrs {
        targets.push(prog.resolve_entry(selector).map_err(|error| error.to_string())?);
    }

    // `--functions a,b,c`: intersect names with the enumerated set.  An ALIAS
    // resolves too — collapsing the enumeration must not make a name that used
    // to select a function stop working (the decbench name-narrowing looks up
    // generated `sub_<addr>` names).
    if let Some(names) = &args.names {
        for want in names {
            match prog.resolve_entry(&EntrySelector::Name(want.clone())) {
                Ok(entry) => targets.push(entry),
                Err(EntryLookupError::NotFound { .. }) => {
                    eprintln!("warning: no function named {want:?} in {}", args.binary)
                }
                Err(error) => return Err(error.to_string()),
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
    matches!(
        name,
        "relocobjects"
            | "i386_pie_plt"
            | "relocrebase"
            | "dynrelocs"
            | "macho-arm64e"
            | "typedepth"
            | "dwarfstructs"
            | "ifuncfpret"
            | "symbolnamerepair"
            | "symbolnamechars"
            | "symbolnamebound"
            | "msvcfpconst"
    )
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
    // (kuna, GH-289) The relocatable-object analysis rebase runs inside `load
    // file` (the whole analyzer tier does), so the gate must be exported before
    // `bootstrap_from_object`. Default-on: only an off-token disables it.
    if let Some(value) = last_option_value(options, "relocrebase") {
        let on = !matches!(
            value.trim().to_ascii_lowercase().as_str(),
            "off" | "0" | "false"
        );
        env.set(
            kuna_decomp::kuna_relocrebase::RELOCREBASE_ENV,
            if on { "on" } else { "off" },
        );
    }
    // (kuna, DIV-84) Same timing for the linked-image dynamic relocations: they
    // are applied inside the loader's own snapshot of the image bytes.
    if let Some(value) = last_option_value(options, "dynrelocs") {
        let on = !matches!(
            value.trim().to_ascii_lowercase().as_str(),
            "off" | "0" | "false"
        );
        env.set(
            kuna_decomp::kuna_dynrelocs::DYNRELOCS_ENV,
            if on { "on" } else { "off" },
        );
    }
    // (kuna, DIV-96) Same timing for the MSVC `__real@` constants: the decoded
    // bytes are materialised while the loader lays the object out.
    if let Some(value) = last_option_value(options, "msvcfpconst") {
        let on = !matches!(
            value.trim().to_ascii_lowercase().as_str(),
            "off" | "0" | "false"
        );
        env.set(
            kuna_decomp::kuna_msvcfpconst::MSVCFPCONST_ENV,
            if on { "on" } else { "off" },
        );
    }
    // (kuna) The symbol table is installed inside `load file`, so the gate must be
    // exported before `bootstrap_from_object` -- turning it off after the fact
    // would arrive long after the load it was meant to fail. Default-on: only an
    // off-token disables it.
    if let Some(value) = last_option_value(options, "symbolnamerepair") {
        let on = !matches!(
            value.trim().to_ascii_lowercase().as_str(),
            "off" | "0" | "false"
        );
        env.set(
            kuna_decomp::kuna_symbolnamerepair::SYMBOLNAMEREPAIR_ENV,
            if on { "on" } else { "off" },
        );
    }
    // (kuna, GH-340) Symbol names are minted inside `load file` (the loader's
    // symbol walks and the analysis passes both run there), so the sanitizer's
    // mode must be exported before `bootstrap_from_object`. An unrecognized
    // token falls back to the shipped `safe` rather than silently to `off`.
    if let Some(value) = last_option_value(options, "symbolnamechars") {
        let mode = kuna_decomp::kuna_symbolnamechars::NameChars::parse(value).unwrap_or_default();
        env.set(
            kuna_decomp::kuna_symbolnamechars::SYMBOLNAMECHARS_ENV,
            mode.as_str(),
        );
    }
    // (kuna) The scope ceiling is spent while the symbol table is installed
    // inside `load file`, so it must be exported before `bootstrap_from_object`.
    // Valued: the token goes through verbatim.
    if let Some(value) = last_option_value(options, "symbolnamebound") {
        env.set(kuna_decomp::kuna_symbolnamebound::SYMBOLNAMEBOUND_ENV, value.trim());
    }
    if let Some(value) = last_option_value(options, "ifuncfpret") {
        // default-off, opt-in: only an on-token enables it.
        let on = matches!(
            value.trim().to_ascii_lowercase().as_str(),
            "on" | "1" | "true" | ""
        );
        env.set("KUNA_IFUNCFPRET", if on { "on" } else { "off" });
    }
    if let Some(value) = last_option_value(options, "typedepth") {
        let on = !matches!(
            value.trim().to_ascii_lowercase().as_str(),
            "off" | "0" | "false"
        );
        env.set(kuna_decomp::kuna_typedepth::TYPEDEPTH_ENV, if on { "on" } else { "off" });
    }
    if let Some(value) = last_option_value(options, "dwarfstructs") {
        let on = !matches!(
            value.trim().to_ascii_lowercase().as_str(),
            "off" | "0" | "false"
        );
        env.set(
            kuna_decomp::kuna_dwarfstructs::DWARFSTRUCTS_ENV,
            if on { "on" } else { "off" },
        );
    }
    if let Some(value) = last_option_value(options, "dwarfvariants") {
        let on = !matches!(
            value.trim().to_ascii_lowercase().as_str(),
            "off" | "0" | "false"
        );
        env.set(
            kuna_decomp::kuna_dwarfvariants::DWARFVARIANTS_ENV,
            if on { "on" } else { "off" },
        );
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

/// (kuna outlang) The output language to use when the caller named none.
///
/// A Rust binary rendered as C is strictly worse than the same binary rendered
/// as Rust: the types are wrong in a way a reader has to undo by hand. kuna
/// already knows which it is -- `sourcelang::detect_compiler` is the port of
/// Ghidra's `SourceLanguageAnalyzer`, and it reports `Rustc` from the `.comment`
/// `rustc version` record, a `.rodata` signature, or a Rust-mangled symbol -- so
/// the default follows the binary rather than making every user of a Rust binary
/// remember a flag.
///
/// Detection is high-precision, not heuristic, and `--language c` always wins.
/// Returns `None` when the file is not a Rust binary or cannot be parsed, which
/// leaves the C default in place: this can only ever ADD a language, never take
/// one away.
pub fn detected_output_language(binary: &str) -> Option<&'static str> {
    let bytes = std::fs::read(binary).ok()?;
    let file = object::File::parse(&*bytes).ok()?;
    match kuna_analysis::sourcelang::detect_compiler(&file, &bytes) {
        kuna_analysis::sourcelang::Compiler::Rustc => Some("rust-language"),
        _ => None,
    }
}

/// Resolve a `--language` value, or `None` for the auto policy.
///
/// `auto` is the default and the only value that is not a language name.
pub fn parse_language_flag(v: &str) -> Result<Option<&'static str>, String> {
    if v.eq_ignore_ascii_case("auto") {
        return Ok(None);
    }
    kuna_decomp::kuna_lang::OutLang::from_print_name(v)
        .map(|l| Some(l.print_name()))
        .ok_or_else(|| {
            format!(
                "unknown output language {v:?} (expected auto, or one of: {})",
                kuna_decomp::kuna_lang::OutLang::names().join(", ")
            )
        })
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

// --- discovery-failure diagnosis --------------------------------------------

/// Why a run that discovered ZERO functions failed, or `None` when an empty
/// inventory is the honest answer for this image.
///
/// A total discovery failure used to be reported in a successful run's voice —
/// `count: 0`, exit 0, silent stderr — which an agent cannot tell apart from
/// "this file genuinely has no functions". The distinction this makes is
/// EXECUTABLE CONTENT: a data-only relocatable object or a resource-only PE has
/// no functions to find, and failing those would turn a correct answer into an
/// error. An image that does carry code and yielded nothing is a failed run, and
/// the message names the cause it can prove, because a packed image is the one
/// an agent can act on.
pub(crate) fn zero_discovery_error(binary: &str) -> Option<String> {
    let bytes = std::fs::read(binary).unwrap_or_default();
    if !bytes.is_empty() && !image_has_executable_content(&bytes) {
        return None;
    }
    Some(match detect_packer(&bytes) {
        Some(packer) => format!(
            "no functions discovered in {binary}: image appears {packer}-packed; \
             try `kuna unpack`"
        ),
        None => format!("no functions discovered in {binary}"),
    })
}

/// The packer whose signature `bytes` carries.
///
/// UPX is the one that matters: it is what `kuna unpack` targets, and every UPX
/// build stamps the `UPX!` magic into its stub and into each packed block
/// header, so a whole-image search is both cheap (this runs only once a run has
/// already failed) and precise enough to name in a diagnostic.
fn detect_packer(bytes: &[u8]) -> Option<&'static str> {
    bytes.windows(4).any(|w| w == b"UPX!").then_some("UPX")
}

/// Does this image carry executable content at all?
///
/// Section flags first (the per-format executable test `kuna-analysis`'s entry
/// analyzers use), then the ELF program headers — a section-header-stripped PIE
/// has no section table at all, and the program header is what the loader obeys.
/// An image `object` cannot parse (a raw blob, a `<binaryimage>` document)
/// answers `true`: nothing there clears the run, so it stays a failure.
fn image_has_executable_content(bytes: &[u8]) -> bool {
    // ELF section header flag SHF_EXECINSTR; the Mach-O instruction attributes.
    const SHF_EXECINSTR: u64 = 0x4;
    const S_ATTR_PURE_INSTRUCTIONS: u32 = 0x8000_0000;
    const S_ATTR_SOME_INSTRUCTIONS: u32 = 0x0000_0400;
    // ELF program header flag PF_X.
    const PF_X: u32 = 0x1;

    let Ok(file) = object::File::parse(bytes) else {
        return true;
    };
    let executable_section = file.sections().any(|sec| {
        sec.size() != 0
            && match sec.flags() {
                object::SectionFlags::Elf { sh_flags } => sh_flags & SHF_EXECINSTR != 0,
                object::SectionFlags::Coff { characteristics } => {
                    characteristics & object::pe::IMAGE_SCN_MEM_EXECUTE != 0
                }
                object::SectionFlags::MachO { flags } => {
                    flags & (S_ATTR_PURE_INSTRUCTIONS | S_ATTR_SOME_INSTRUCTIONS) != 0
                        || sec.kind() == object::SectionKind::Text
                }
                _ => sec.kind() == object::SectionKind::Text,
            }
    });
    executable_section
        || file.segments().any(|seg| {
            seg.size() != 0
                && matches!(
                    seg.flags(),
                    object::SegmentFlags::Elf { p_flags } if p_flags & PF_X != 0
                )
        })
}

// --- output rendering --------------------------------------------------------

/// Render the `--json` document for a decompile run.
///
/// `kuna decompile --json` renders through here too (its `functions` array holds
/// the one function it was asked for), so the single-function and whole-binary
/// surfaces cannot drift into two shapes for one record.
pub(crate) fn render_result_json(
    binary: &str,
    funcs: &[FuncResult],
    options: &[(String, String)],
    error: Option<&str>,
) -> String {
    let language = last_option_value(options, "setlanguage").unwrap_or("c-language");
    format!("{}\n", dumps_indent2(&result_json(binary, funcs, language, error)))
}

/// The `functions --json` document.
fn functions_json(binary: &str, entries: &[FunctionEntry], error: Option<&str>) -> String {
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
                    (
                        "object_location".into(),
                        object_location_json(e.object_location.as_ref()),
                    ),
                    ("size".into(), Json::Number(e.size.to_string())),
                ])
            })
            .collect(),
    );
    format!(
        "{}\n",
        dumps_indent2(&Json::Object(vec![
            ("binary".into(), Json::Str(binary.to_string())),
            ("count".into(), Json::Number(entries.len().to_string())),
            ("error".into(), error_json(error)),
            ("functions".into(), arr),
        ]))
    )
}

/// The run-level `error` field. Always present (`null` on a healthy run) so a
/// consumer can read it unconditionally, exactly as [`aliases_json`] is.
fn error_json(error: Option<&str>) -> Json {
    error.map(|e| Json::Str(e.to_string())).unwrap_or(Json::Null)
}

/// Build the `decompile-all --json` document.
fn result_json(
    binary: &str,
    funcs: &[FuncResult],
    language: &str,
    error: Option<&str>,
) -> Json {
    let functions = Json::Array(
        funcs
            .iter()
            .map(|f| {
                let vars = Json::Array(f.variables.iter().map(var_json).collect());
                let line_mappings =
                    Json::Array(f.line_mappings.iter().map(line_mapping_json).collect());
                Json::Object(vec![
                    ("name".into(), Json::Str(f.name.clone())),
                    ("address".into(), Json::Number(f.address.to_string())),
                    ("address_hex".into(), Json::Str(format!("0x{:x}", f.address))),
                    ("aliases".into(), aliases_json(&f.aliases)),
                    (
                        "object_location".into(),
                        object_location_json(f.object_location.as_ref()),
                    ),
                    ("size".into(), Json::Number(f.size.to_string())),
                    (
                        "code".into(),
                        f.code.clone().map(Json::Str).unwrap_or(Json::Null),
                    ),
                    (
                        "error".into(),
                        f.error.clone().map(Json::Str).unwrap_or(Json::Null),
                    ),
                    ("line_mappings".into(), line_mappings),
                    ("variables".into(), vars),
                ])
            })
            .collect(),
    );
    Json::Object(vec![
        ("binary".into(), Json::Str(binary.to_string())),
        // (kuna outlang) The auto language policy resolves inside the engine, so
        // the document has to say which language `code` is in -- otherwise a
        // consumer cannot tell a Rust body from a C one without guessing.
        ("language".into(), Json::Str(language.to_string())),
        ("count".into(), Json::Number(funcs.len().to_string())),
        // The RUN-level error channel, set exactly when the command exits
        // non-zero (a total discovery failure here; the aborted function on
        // `kuna decompile --json`). A single function that failed inside a
        // whole-binary run is that record's own `error`, not this one.
        ("error".into(), error_json(error)),
        ("functions".into(), functions),
    ])
}

fn line_mapping_json(mapping: &LineMapping) -> Json {
    Json::Object(vec![
        ("line_number".into(), Json::Number(mapping.line_number.to_string())),
        (
            "addresses".into(),
            Json::Array(
                mapping
                    .addresses
                    .iter()
                    .map(|address| Json::Number(address.to_string()))
                    .collect(),
            ),
        ),
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

fn object_location_json(location: Option<&ObjectLocation>) -> Json {
    match location {
        Some(location) => Json::Object(vec![
            (
                "section_index".into(),
                Json::Number(location.section_index.to_string()),
            ),
            ("section".into(), Json::Str(location.section.clone())),
            ("offset".into(), Json::Number(location.offset.to_string())),
            (
                "offset_hex".into(),
                Json::Str(format!("0x{:x}", location.offset)),
            ),
        ]),
        None => Json::Null,
    }
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
        (
            "line_numbers".into(),
            Json::Array(
                v.line_numbers
                    .iter()
                    .map(|line| Json::Number(line.to_string()))
                    .collect(),
            ),
        ),
        (
            "addresses".into(),
            Json::Array(
                v.addresses
                    .iter()
                    .map(|address| Json::Number(address.to_string()))
                    .collect(),
            ),
        ),
    ])
}

#[cfg(test)]
mod provenance_json_tests {
    use super::*;

    #[test]
    fn result_schema_adds_line_and_variable_provenance() {
        let function = FuncResult {
            name: "f".into(),
            address: 0x401000,
            size: 12,
            code: Some("int f(int x)\n{\n  return x;\n}".into()),
            error: None,
            proto: None,
            variables: vec![VarInfo {
                name: "x".into(),
                type_name: "int".into(),
                stack_offset: None,
                size: 4,
                is_param: true,
                arg_index: Some(0),
                line_numbers: vec![3],
                addresses: vec![0x401004],
            }],
            line_mappings: vec![LineMapping {
                line_number: 3,
                addresses: vec![0x401004, 0x401008],
            }],
            aliases: Vec::new(),
            object_location: None,
        };

        let rendered = dumps_indent2(&result_json("fixture", &[function], "c-language", None));
        assert!(rendered.contains("\"address\": 4198400"));
        assert!(rendered.contains("\"code\": \"int f(int x)\\n{\\n  return x;\\n}\""));
        assert!(rendered.contains("\"line_mappings\": ["));
        assert!(rendered.contains("\"line_number\": 3"));
        assert!(rendered.contains("\"line_numbers\": [\n            3"));
        assert!(rendered.contains("\"addresses\": [\n            4198404"));
    }
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
    let mut addrs: Vec<EntrySelector> = Vec::new();
    let mut no_vars = false;
    let mut max_fn_seconds: Option<u64> = None;
    let mut options: Vec<(String, String)> = Vec::new();
    let mut mode: Option<String> = None;
    let mut slice: Option<String> = None;
    let mut target: Option<String> = None;
    let mut sleighpath: Option<String> = None;
    let mut saw_language = false;

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
                addrs.push(parse_entry_selector(&v)?);
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
            // (kuna outlang) `--language` is the first-class surface for the
            // output language; it lowers to the upstream `setlanguage` option, so
            // it reaches every downstream consumer (the console script here, the
            // in-process option applier in decompile-all) with no new plumbing.
            // Pushed in argv order, so a later `--option setlanguage` still wins.
            "--language" => {
                let v = take(argv, &mut i, "--language")?;
                if let Some(lang) = parse_language_flag(&v)? {
                    options.push(("setlanguage".into(), lang.into()));
                }
                saw_language = true;
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

    // (kuna outlang, DIV-80) The auto policy: with no `--language` and no
    // explicit `--option setlanguage`, follow the binary. `decompile-project` is
    // excluded -- its `.c`/`.h`/`.asm` export is C-shaped end to end and refuses
    // any other language, so auto-selecting one there would turn a working
    // export into an error.
    if !saw_language
        && cmd != "decompile-project"
        && !options.iter().any(|(n, _)| n == "setlanguage")
    {
        if let Some(lang) = detected_output_language(&binary) {
            options.push(("setlanguage".into(), lang.into()));
        }
    }

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

fn parse_entry_selector(s: &str) -> Result<EntrySelector, String> {
    match EntrySelector::parse(s) {
        selector @ (EntrySelector::Numeric(_)
        | EntrySelector::SectionOffset { .. }
        | EntrySelector::SectionIndexOffset { .. }) => Ok(selector),
        EntrySelector::Name(_) => parse_hex(s).map(EntrySelector::Numeric),
    }
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
         2 MiB, and fast at 2 MiB or larger. Explicit --option values win.\n\
         An unfiltered run that discovers no function at all exits 1 with the\n\
         reason on stderr and in the document's run-level `error` field."
    );
}

fn usage_functions() {
    eprintln!(
        "usage: kuna functions <binary> [--json] [--mode auto|reliable|aggressive|fast] [--slice ARCH] [--target T] [--sleighpath D]\n\
         \n\
         List every function kuna discovers in a binary as `<addr>\\t<name>` (or\n\
         --json: {{binary,count,functions:[{{name,address}}]}}).\n\
         Shares decompile-all's discovery policy, so the inventory always contains\n\
         every function a whole-binary run would decompile; on a non-x86-64 binary\n\
         that means a full prologue-pattern + gap-walk discovery pass.\n\
         Discovering no function at all exits 1 with the reason on stderr and in\n\
         the document's `error` field (a packed image is named as such)."
    );
}

#[cfg(test)]
mod discovery_tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    static TEMP_ID: AtomicU64 = AtomicU64::new(0);

    /// A minimal ELF64 executable: one `PF_X` `PT_LOAD` and NO section table —
    /// the section-header-stripped PIE shape of the witness binary, where the
    /// program header is the only evidence the image holds code.
    fn stripped_executable(payload: &[u8]) -> Vec<u8> {
        const EHDR: usize = 64;
        const PHDR: usize = 56;
        let mut out = vec![0u8; EHDR + PHDR + payload.len()];
        out[..4].copy_from_slice(b"\x7fELF");
        out[4] = 2; // ELFCLASS64
        out[5] = 1; // ELFDATA2LSB
        out[6] = 1; // EV_CURRENT
        out[16..18].copy_from_slice(&2u16.to_le_bytes()); // e_type = ET_EXEC
        out[18..20].copy_from_slice(&62u16.to_le_bytes()); // e_machine = EM_X86_64
        out[20..24].copy_from_slice(&1u32.to_le_bytes()); // e_version
        out[32..40].copy_from_slice(&(EHDR as u64).to_le_bytes()); // e_phoff
        out[52..54].copy_from_slice(&(EHDR as u16).to_le_bytes()); // e_ehsize
        out[54..56].copy_from_slice(&(PHDR as u16).to_le_bytes()); // e_phentsize
        out[56..58].copy_from_slice(&1u16.to_le_bytes()); // e_phnum
        let total = (EHDR + PHDR + payload.len()) as u64;
        let p = EHDR;
        out[p..p + 4].copy_from_slice(&1u32.to_le_bytes()); // p_type = PT_LOAD
        out[p + 4..p + 8].copy_from_slice(&5u32.to_le_bytes()); // p_flags = PF_R|PF_X
        out[p + 16..p + 24].copy_from_slice(&0x40_0000u64.to_le_bytes()); // p_vaddr
        out[p + 32..p + 40].copy_from_slice(&total.to_le_bytes()); // p_filesz
        out[p + 40..p + 48].copy_from_slice(&total.to_le_bytes()); // p_memsz
        out[p + 48..p + 56].copy_from_slice(&0x1000u64.to_le_bytes()); // p_align
        out[EHDR + PHDR..].copy_from_slice(payload);
        out
    }

    /// A minimal ET_REL ELF64 whose only allocated section is `.data` — an
    /// object file that legitimately holds no functions at all.
    fn data_only_object() -> Vec<u8> {
        const EHDR: usize = 64;
        const SHDR: usize = 64;
        let names: &[u8] = b"\0.data\0.shstrtab\0";
        let shoff = EHDR;
        let names_off = shoff + 3 * SHDR;
        let data_off = names_off + names.len();
        let mut out = vec![0u8; data_off + 4];
        out[..4].copy_from_slice(b"\x7fELF");
        out[4] = 2;
        out[5] = 1;
        out[6] = 1;
        out[16..18].copy_from_slice(&1u16.to_le_bytes()); // e_type = ET_REL
        out[18..20].copy_from_slice(&62u16.to_le_bytes());
        out[20..24].copy_from_slice(&1u32.to_le_bytes());
        out[40..48].copy_from_slice(&(shoff as u64).to_le_bytes()); // e_shoff
        out[52..54].copy_from_slice(&(EHDR as u16).to_le_bytes());
        out[58..60].copy_from_slice(&(SHDR as u16).to_le_bytes()); // e_shentsize
        out[60..62].copy_from_slice(&3u16.to_le_bytes()); // e_shnum
        out[62..64].copy_from_slice(&2u16.to_le_bytes()); // e_shstrndx
        out[names_off..data_off].copy_from_slice(names);

        fn shdr(out: &mut [u8], at: usize, name: u32, kind: u32, flags: u64, off: u64, size: u64) {
            out[at..at + 4].copy_from_slice(&name.to_le_bytes());
            out[at + 4..at + 8].copy_from_slice(&kind.to_le_bytes());
            out[at + 8..at + 16].copy_from_slice(&flags.to_le_bytes());
            out[at + 24..at + 32].copy_from_slice(&off.to_le_bytes());
            out[at + 32..at + 40].copy_from_slice(&size.to_le_bytes());
        }
        // `.data`: SHT_PROGBITS, SHF_ALLOC|SHF_WRITE — allocated, never executed.
        shdr(&mut out, shoff + SHDR, 1, 1, 0x3, data_off as u64, 4);
        shdr(&mut out, shoff + 2 * SHDR, 7, 3, 0, names_off as u64, names.len() as u64);
        out
    }

    fn temp_image(tag: &str, bytes: &[u8]) -> std::path::PathBuf {
        let id = TEMP_ID.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "kuna-discovery-{tag}-{}-{id}.bin",
            std::process::id()
        ));
        std::fs::write(&path, bytes).expect("write the discovery fixture");
        path
    }

    #[test]
    fn the_upx_magic_is_recognized_and_nothing_else_is() {
        assert_eq!(detect_packer(b"....UPX!....."), Some("UPX"));
        assert_eq!(detect_packer(b"UPX!"), Some("UPX"));
        assert_eq!(detect_packer(b"a plain unpacked image, UPX-free"), None);
        assert_eq!(detect_packer(b"UPX"), None, "the magic is four bytes");
        assert_eq!(detect_packer(b""), None);
    }

    /// A section-header-stripped executable still shows its code through the
    /// program headers, so zero functions there is a failure — and a packed one
    /// is named, because that is the cause the caller can act on.
    #[test]
    fn a_packed_image_names_the_packer_in_the_failure() {
        let packed = temp_image("packed", &stripped_executable(b"UPX!\x00\x00\x00\x00"));
        let message = zero_discovery_error(packed.to_str().unwrap())
            .expect("an executable image that yielded nothing is a failure");
        assert!(message.contains("no functions"), "{message}");
        assert!(message.contains("UPX-packed"), "{message}");
        assert!(message.contains("kuna unpack"), "{message}");
        std::fs::remove_file(packed).expect("remove the discovery fixture");

        let plain = temp_image("plain", &stripped_executable(b"\x55\x48\x89\xe5\x5d\xc3"));
        let message = zero_discovery_error(plain.to_str().unwrap())
            .expect("an executable image that yielded nothing is a failure");
        assert!(message.contains("no functions"), "{message}");
        assert!(!message.contains("packed"), "no packer, no packer claim: {message}");
        std::fs::remove_file(plain).expect("remove the discovery fixture");
    }

    /// The legitimate empty case: an image with no executable content has no
    /// functions to find, so the empty inventory stays a success.
    #[test]
    fn an_image_with_no_code_keeps_its_honest_empty_answer() {
        let bytes = data_only_object();
        assert!(!image_has_executable_content(&bytes));
        let path = temp_image("dataonly", &bytes);
        assert_eq!(zero_discovery_error(path.to_str().unwrap()), None);
        std::fs::remove_file(path).expect("remove the discovery fixture");
    }

    /// Anything `object` cannot parse is not evidence of innocence: a run that
    /// found nothing in it still failed.
    #[test]
    fn an_unparseable_image_is_still_a_failure() {
        assert!(image_has_executable_content(b"not an object file at all"));
        assert!(image_has_executable_content(&[]));
    }

    /// The checked-in x86-64 fixture the acceptance probes use: real code, real
    /// sections — the shape that must fail loudly if discovery ever returns
    /// nothing for it.
    #[test]
    fn a_real_fixture_carries_executable_content() {
        let fixture = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../kuna-analysis/tests/fixtures/aif_gap_x86_64");
        let bytes = std::fs::read(fixture).expect("the fixture is checked in");
        assert!(image_has_executable_content(&bytes));
        assert_eq!(detect_packer(&bytes), None);
    }

    /// The run-level `error` field is present on every document, so a consumer
    /// reads it unconditionally rather than inferring failure from `count`.
    #[test]
    fn the_run_level_error_field_is_always_present() {
        let healthy = functions_json("fixture", &[], None);
        assert!(healthy.contains("\"error\": null"), "{healthy}");
        let failed = functions_json("fixture", &[], Some("no functions discovered in fixture"));
        assert!(
            failed.contains("\"error\": \"no functions discovered in fixture\""),
            "{failed}"
        );
        let decompiled = render_result_json("fixture", &[], &[], Some("boom"));
        assert!(decompiled.contains("\"error\": \"boom\""), "{decompiled}");
        assert!(
            render_result_json("fixture", &[], &[], None).contains("\"error\": null")
        );
    }

    /// `--language` reaches the document through the same last-write-wins lookup
    /// every other option uses.
    #[test]
    fn the_document_reports_the_selected_language() {
        let options = vec![
            ("setlanguage".into(), "rust-language".into()),
            ("setlanguage".into(), "c-language".into()),
        ];
        assert!(render_result_json("f", &[], &options, None).contains("\"c-language\""));
        assert!(render_result_json("f", &[], &[], None).contains("\"c-language\""));
    }
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
