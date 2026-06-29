//! `kuna decompile-all` / `kuna functions` — in-process **whole-binary**
//! decompilation.
//!
//! ```text
//!   kuna decompile-all <binary> [--json] [--functions a,b,..] [--addr 0xVMA].. \
//!                       [--no-vars] [--option N V].. [--slice ARCH] [--target T] [--sleighpath D]
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

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_console::engine::{bootstrap_from_object, ConsoleProgram};
use kuna_decomp::decompile_drive::{
    decompile_func_full_with_override_dyn, extract_variables, print_c, VarInfo,
};
use kuna_decomp::options::{OptionDatabase, KUNA_OPTION_NAMES, RELOC_OBJECTS_ENV};

use crate::jsonfmt::{dumps_indent2, Json};
use crate::paths;

/// Parsed `decompile-all` / `functions` arguments (the two share a loader).
struct Args {
    binary: String,
    json: bool,
    /// `--functions a,b,c`: restrict to these names (None ⇒ every function).
    names: Option<Vec<String>>,
    /// `--addr 0xVMA` (repeatable): decompile specific entry addresses, even if
    /// unnamed (stripped / LLM use).  Combined with `--functions` if both given.
    addrs: Vec<u64>,
    /// `--no-vars`: skip the per-function variable extraction (faster; drops the
    /// `variables` array used by decbench's `type_match`).
    no_vars: bool,
    options: Vec<(String, String)>,
    slice: Option<String>,
    target: Option<String>,
    sleighpath: Option<String>,
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

/// One decompiled function's result (success carries `code`; failure carries
/// `error`).
struct FuncResult {
    name: String,
    address: u64,
    size: i64,
    code: Option<String>,
    error: Option<String>,
    variables: Vec<VarInfo>,
}

/// Load + analyze the binary once, then decompile every selected function.
fn decompile_all(args: &Args) -> Result<Vec<FuncResult>, String> {
    let mut prog = load_program(args)?;
    let targets = resolve_targets(&prog, args)?;

    let mut out = Vec::with_capacity(targets.len());
    for (name, entry) in targets {
        let address = entry.get_offset();
        // Mirror IfcDecompile: re-seed this function's DWARF stack locals (so a
        // `-g` binary renders DWARF names/types) and decompile.  The drive itself
        // catches un-ported-seam panics and returns Err, so a single bad function
        // degrades to an `error` record instead of aborting the binary.
        let mapped = prog.dwarf_locals_for(address);
        match decompile_func_full_with_override_dyn(
            prog.arch_mut(),
            &name,
            entry,
            0, // UNBOUNDED: the function's natural flow extent
            &mapped,
            &[],
            &[],
            None,
            &[],
            &[],
            &[],
        ) {
            Ok(fd) => {
                let size = fd.get_size() as i64;
                // Render + extract under `catch_unwind`: the decompile drive only
                // guards the pipeline (decompile_drive.rs), so a fail-fast invariant
                // in the printer / type declarator on an exotic recovered function
                // would otherwise abort the WHOLE binary and discard every function
                // already decompiled. Containing it here honors the per-function
                // isolation contract (one bad function → one `error` record).
                let no_vars = args.no_vars;
                let rendered = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    // Trim the surrounding newlines the same way `kuna decompile`
                    // does (`decompile.rs::trim_newlines`), so the per-function
                    // `code` is byte-identical to the single-shot path.
                    let code = print_c(prog.arch_mut(), &fd).trim_matches('\n').to_string();
                    let variables =
                        if no_vars { Vec::new() } else { extract_variables(prog.arch(), &fd) };
                    (code, variables)
                }));
                match rendered {
                    Ok((code, variables)) => out.push(FuncResult {
                        name,
                        address,
                        size,
                        code: Some(code),
                        error: None,
                        variables,
                    }),
                    Err(_) => out.push(FuncResult {
                        name,
                        address,
                        size: 0,
                        code: None,
                        error: Some("panic while rendering C / extracting variables".into()),
                        variables: Vec::new(),
                    }),
                }
            }
            Err(e) => out.push(FuncResult {
                name,
                address,
                size: 0,
                code: None,
                error: Some(e.explain().to_string()),
                variables: Vec::new(),
            }),
        }
    }
    Ok(out)
}

/// Enumerate the program's functions as `(name, address)` (the `functions`
/// command + the default `decompile-all` target set).
fn list_functions(args: &Args) -> Result<Vec<(String, u64)>, String> {
    let prog = load_program(args)?;
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
/// `--option`s in the correct order.
fn load_program(args: &Args) -> Result<ConsoleProgram, String> {
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
fn resolve_targets(prog: &ConsoleProgram, args: &Args) -> Result<Vec<(String, Address)>, String> {
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

/// Render the functions as concatenated C with `// Function:` headers (the human
/// output, mirroring `DecompilationResult.to_c_file`).
fn render_c(funcs: &[FuncResult]) -> String {
    let mut out = String::new();
    for f in funcs {
        match (&f.code, &f.error) {
            (Some(code), _) => {
                out.push_str(&format!("// Function: {} @ 0x{:x}\n", f.name, f.address));
                out.push_str(code);
                out.push_str("\n\n");
            }
            (None, Some(err)) => {
                out.push_str(&format!(
                    "// Function: {} @ 0x{:x}  (error: {})\n\n",
                    f.name, f.address, err
                ));
            }
            (None, None) => {}
        }
    }
    out
}

// --- argument parsing --------------------------------------------------------

fn parse_args(argv: &[String], cmd: &str) -> Result<Args, String> {
    let mut binary: Option<String> = None;
    let mut json = false;
    let mut names: Option<Vec<String>> = None;
    let mut addrs: Vec<u64> = Vec::new();
    let mut no_vars = false;
    let mut options: Vec<(String, String)> = Vec::new();
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
            "--option" => {
                if i + 2 >= argv.len() {
                    return Err("--option requires NAME VALUE".into());
                }
                options.push((argv[i + 1].clone(), argv[i + 2].clone()));
                i += 2;
            }
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
    Ok(Args { binary, json, names, addrs, no_vars, options, slice, target, sleighpath })
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
         \x20                   [--no-vars] [--option N V].. [--slice ARCH] [--target T] [--sleighpath D]\n\
         \n\
         Decompile every function of a binary in one in-process load (load-once,\n\
         decompile-many).  --json emits {{binary,count,functions:[{{name,address,code,variables,..}}]}};\n\
         without it, concatenated C with `// Function:` headers."
    );
}

fn usage_functions() {
    eprintln!(
        "usage: kuna functions <binary> [--json] [--slice ARCH] [--target T] [--sleighpath D]\n\
         \n\
         List every function kuna discovers in a binary as `<addr>\\t<name>` (or\n\
         --json: {{binary,count,functions:[{{name,address}}]}})."
    );
}
