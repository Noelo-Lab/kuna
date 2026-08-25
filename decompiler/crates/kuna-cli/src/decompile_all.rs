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
                let language = args
                    .options
                    .iter()
                    .rev()
                    .find(|(n, _)| n == "setlanguage")
                    .map(|(_, v)| v.as_str())
                    .unwrap_or("c-language");
                println!("{}", dumps_indent2(&result_json(&args.binary, &funcs, language)));
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
    let mut prog = load_program(args, DriverDefaults::Decompile)?;
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

    Ok(decompile_targets(
        &mut prog,
        targets,
        args.no_vars,
        /* want_proto= */ false,
        /* want_provenance= */ args.json,
    ))
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

// --- output rendering --------------------------------------------------------

/// Build the `decompile-all --json` document.
fn result_json(binary: &str, funcs: &[FuncResult], language: &str) -> Json {
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
        };

        let rendered = dumps_indent2(&result_json("fixture", &[function], "c-language"));
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
    let mut addrs: Vec<u64> = Vec::new();
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
         --json: {{binary,count,functions:[{{name,address}}]}}).\n\
         Shares decompile-all's discovery policy, so the inventory always contains\n\
         every function a whole-binary run would decompile; on a non-x86-64 binary\n\
         that means a full prologue-pattern + gap-walk discovery pass."
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
