//! kuna-wasm: the in-browser decompiler front-end.
//!
//! This is the *engine's* in-process path (`kuna_console::engine::
//! bootstrap_from_image` → `commit_pending_analysis` → loop
//! `decompile_func_full_with_override_dyn` + `print_c`) wrapped in a tiny,
//! dependency-light CLI that reads its inputs from the (virtual) filesystem and
//! writes JSON/C to stdout — exactly the contract a browser WASI shim provides.
//! The decompile loop and the project-export artifact builders are the shared
//! decompile-project core (`kuna_console::project`, also behind `kuna
//! decompile-all` / `kuna decompile-project`); this crate keeps only the
//! wasm-safe wrapper (`kuna-cli`'s subprocess/CLI machinery cannot compile for
//! `wasm32-wasip1`). The Node-WASI parity test in `integrations/web/test/`
//! pins native-vs-wasm output equality; the `--json` shape is the
//! `kuna decompile-all --json` fields plus a `"kind"`
//! (`"func"` | `"plt"` | `"thunk"` — [`kuna_console::classify`], shared with
//! `kuna decompile-graph`) on every function entry.
//!
//! Every command takes repeatable `--assert <directive>` — the CLI's override
//! plane ([`kuna_console::assertsyntax`] / [`kuna_console::assertions`]), applied
//! in the CLI's order — and reports what became of each directive in an
//! `assertions` array. `inspect` and `read` are the study view's documents
//! ([`inspect`]).
//!
//! # Why WASI
//! The decompiler touches the outside world only through plain `std::fs` path
//! reads (the binary via `LoadImage`, the SLEIGH `.sla`/`.pspec`/`.cspec`/
//! `.ldefs` via `scan_language_database`). Those map 1:1 onto a WASI virtual
//! filesystem, so this front-end runs in the browser with **zero** engine
//! changes. See `docs/web-integration.md`.

mod inspect;
mod json;

use kuna_console::assertions::{Directive, Outcome};
use kuna_console::engine::{
    bootstrap_from_image, ConsoleProgram, EntrySelector, FunctionEntry,
};
use kuna_console::project::{
    build_asm, build_c, build_header, build_readme, collect_dat_addrs,
    decompile_export_targets, decompile_targets_with, DecompileOptions,
    FuncResult, FAST_WHOLE_BINARY_FN_BUDGET_SECONDS,
};
use kuna_decomp::decompile_drive::{print_c_recompile_prelude, print_c_types};
// The per-function `kind` annotation, shared with `kuna decompile-graph`.
use kuna_console::classify::Classifier;

use json::{
    arr, assertions_json, json_object_location, json_opt_num, json_opt_str, json_str,
    json_str_array, Obj,
};

/// How a command names its function.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Selector {
    Name(String),
    Addr(u64),
}

impl Selector {
    fn parse(arg: &str) -> Self {
        match parse_addr(arg) {
            Some(vma) => Selector::Addr(vma),
            None => Selector::Name(arg.to_string()),
        }
    }
}

/// Parsed command.
#[derive(Debug)]
enum Cmd {
    /// Enumerate functions only (no per-function decompile; analysis follows
    /// the selected concrete mode).
    List,
    /// Decompile every CODE-backed function.
    DecompileAll,
    /// Decompile one function, selected by name.
    DecompileName(String),
    /// Decompile one function, selected by entry VMA.
    DecompileAddr(u64),
    /// Whole-binary project export (`.c`/`.h`/`.asm`/`README.md` as one JSON
    /// document); the payload is the display name the artifacts are named
    /// after. Whole binary only — no `--functions` subset on this surface.
    Project(String),
    /// One function with its token source map and instruction listing.
    Inspect(Selector),
    /// Raw image bytes (discovery off).
    Read { addr: u64, len: u64 },
}

/// One front-end invocation: the argv of `kuna_wasm` as data.
#[derive(Debug, Clone, Copy)]
pub struct Request<'a> {
    pub binary: &'a str,
    pub spec_root: &'a str,
    pub cmd: &'a str,
    /// The command's positional arguments (at most two).
    pub args: &'a [String],
    pub mode: Option<&'a str>,
    pub language: Option<&'a str>,
    /// `--assert` values, in order: directives or `@FILE`s of them.
    pub asserts: &'a [String],
}

/// Run the front-end with the automatic size-based mode policy.
///
/// `binary` and `spec_root` are (virtual) filesystem paths; `cmd`/`arg` come
/// from argv. Returns the stdout payload (JSON) on success.
pub fn run(binary: &str, spec_root: &str, cmd: &str, arg: Option<&str>) -> Result<String, String> {
    run_with_mode(binary, spec_root, cmd, arg, None)
}

/// Run the front-end with an explicit mode, no output language (the `auto`
/// policy applies).
pub fn run_with_mode(
    binary: &str,
    spec_root: &str,
    cmd: &str,
    arg: Option<&str>,
    requested_mode: Option<&str>,
) -> Result<String, String> {
    run_with(binary, spec_root, cmd, arg, requested_mode, None)
}

/// Run the front-end with an optional explicit mode and output language, and
/// no assertions ([`run_request`]).
pub fn run_with(
    binary: &str,
    spec_root: &str,
    cmd: &str,
    arg: Option<&str>,
    requested_mode: Option<&str>,
    requested_language: Option<&str>,
) -> Result<String, String> {
    let args: Vec<String> = arg.map(str::to_string).into_iter().collect();
    run_request(&Request {
        binary,
        spec_root,
        cmd,
        args: &args,
        mode: requested_mode,
        language: requested_language,
        asserts: &[],
    })
}

/// Parse the `--assert` values into directives. A directive is one line; a
/// value that does not parse is the whole run's error, spelled as the CLI
/// spells it (`--assert "<directive>": <why>`).
fn parse_asserts(values: &[String]) -> Result<Vec<Directive>, String> {
    let mut out = Vec::new();
    for value in values {
        if value.contains('\n') || value.contains('\r') {
            return Err(format!("--assert {value:?}: a directive is one line"));
        }
        out.extend(kuna_console::assertsyntax::parse_directive_flag(value)?);
    }
    Ok(out)
}

fn parse_command(cmd: &str, args: &[String], binary: &str) -> Result<Cmd, String> {
    let at_most = |n: usize| {
        if args.len() > n {
            Err(format!("`{cmd}` takes at most {n} argument(s), got {}", args.len()))
        } else {
            Ok(())
        }
    };
    let arg = args.first().map(String::as_str);
    Ok(match cmd {
        "list" => {
            at_most(1)?;
            Cmd::List
        }
        "decompile" => {
            at_most(1)?;
            match arg {
                None => Cmd::DecompileAll,
                Some(a) => match Selector::parse(a) {
                    Selector::Addr(vma) => Cmd::DecompileAddr(vma),
                    Selector::Name(name) => Cmd::DecompileName(name),
                },
            }
        }
        "project" => {
            at_most(1)?;
            // Display name defaults to the binary's basename (the CLI's
            // `<binary-filename>.kuna/` convention).
            let display = arg.map(str::to_string).unwrap_or_else(|| {
                std::path::Path::new(binary)
                    .file_name()
                    .map(|s| s.to_string_lossy().into_owned())
                    .unwrap_or_else(|| binary.to_string())
            });
            Cmd::Project(display)
        }
        "inspect" => {
            at_most(1)?;
            Cmd::Inspect(Selector::parse(arg.ok_or("`inspect` needs a function name or 0x address")?))
        }
        "read" => {
            let [addr, len] = args else {
                return Err("`read` needs <0xADDR> <LEN>".to_string());
            };
            let addr = parse_addr(addr)
                .ok_or_else(|| format!("`read`: {addr:?} is not a 0x-prefixed address"))?;
            let len = parse_len(len)
                .filter(|&n| n > 0 && n <= inspect::READ_MAX)
                .ok_or_else(|| {
                    format!("`read`: length {len:?} must be 1..={} bytes", inspect::READ_MAX)
                })?;
            Cmd::Read { addr, len }
        }
        other => {
            return Err(format!(
                "unknown command: {other:?} (want `list`, `decompile`, `project`, `inspect` or `read`)"
            ))
        }
    })
}

/// Run one request.
///
/// For the mode, `None` and `Some("auto")` select `aggressive` below 500 KiB,
/// `reliable` from 500 KiB through just below 2 MiB, and `fast` at 2 MiB and
/// above.
///
/// (kuna outlang) For the language, `None` and `Some("auto")` follow the binary:
/// a Rust binary renders as Rust. `project` is excluded from that policy -- its
/// `.c`/`.h`/`.asm` export is C-shaped end to end -- so an explicit non-C
/// language is an error there rather than a broken export.
pub fn run_request(req: &Request) -> Result<String, String> {
    let binary = req.binary;
    let command = parse_command(req.cmd, req.args, binary)?;
    let directives = parse_asserts(req.asserts)?;

    let want_fast_funcdisc = command_wants_fast_funcdisc(&command);
    let requested = req.mode.unwrap_or("auto");
    let binary_size = if kuna_decomp::modes::mode_is_automatic(requested) {
        std::fs::metadata(binary)
            .map_err(|e| format!("could not read binary metadata for {binary}: {e}"))?
            .len()
    } else {
        0
    };
    let mode = resolve_mode(req.mode, binary_size)?;
    let language = resolve_language(binary, req.language, &command)?;
    let discovery = !matches!(command, Cmd::Read { .. });
    let mut prog = load_program(
        binary,
        req.spec_root,
        mode,
        want_fast_funcdisc,
        language,
        &directives,
        discovery,
    )?;
    if let Some(seconds) = command_fn_budget_seconds(&command, mode) {
        prog.arch_mut().kuna_fn_budget = Some(std::time::Duration::from_secs(seconds));
    }
    let language = prog.arch().print().out_lang().print_name();

    match command {
        Cmd::List => {
            // One record per entry address, alias names carried as data
            // (issue #197 — this used to dedup by (address, name), so one
            // function was listed once per name it carried).
            let entries = prog.function_entries_canonical();
            let classifier =
                Classifier::new(&prog, binary, entries.iter().map(|e| e.addr.get_offset()));
            let kinds: Vec<&'static str> = entries
                .iter()
                .map(|e| classifier.kind(&prog, &e.name, e.addr.get_offset()))
                .collect();
            Ok(list_json(binary, &prog, language, &entries, &kinds))
        }
        Cmd::Project(display) => project(binary, &mut prog, &display),
        Cmd::Read { addr, len } => {
            Ok(inspect::read_json(binary, &prog, addr, len, &prog.assertion_outcomes()))
        }
        Cmd::Inspect(ref selector) => {
            let targets = resolve_targets(&prog, &command)?;
            let classifier = Classifier::new(
                &prog,
                binary,
                prog.function_entries_canonical().iter().map(|e| e.addr.get_offset()),
            );
            let opts = DecompileOptions {
                want_proto: true,
                want_provenance: true,
                want_tokens: true,
                single_target: true,
                ..DecompileOptions::default()
            };
            let out = decompile_targets_with(&mut prog, targets, &opts);
            let f = out
                .first()
                .ok_or_else(|| format!("no function selected by {selector:?}"))?;
            let kind = classifier.kind(&prog, &f.name, f.byte_address);
            Ok(inspect::inspect_json(binary, language, &prog, f, kind, &prog.assertion_outcomes()))
        }
        _ => {
            let targets = resolve_targets(&prog, &command)?;
            // Classify against the FULL deduped entry set (a single-function
            // decompile still needs every entry for the thunk-target test).
            let classifier = Classifier::new(
                &prog,
                binary,
                prog.function_entries_canonical().iter().map(|e| e.addr.get_offset()),
            );
            let opts = DecompileOptions {
                want_provenance: true,
                single_target: targets.len() == 1,
                ..DecompileOptions::default()
            };
            let out = decompile_targets_with(&mut prog, targets, &opts);
            let kinds: Vec<&'static str> =
                out.iter()
                    .map(|f| classifier.kind(&prog, &f.name, f.byte_address))
                    .collect();
            Ok(result_json(binary, language, &out, &kinds, &prog.assertion_outcomes()))
        }
    }
}

fn command_wants_fast_funcdisc(command: &Cmd) -> bool {
    !matches!(
        command,
        Cmd::DecompileAddr(_) | Cmd::Inspect(Selector::Addr(_)) | Cmd::Read { .. }
    )
}

fn command_fn_budget_seconds(command: &Cmd, mode: &str) -> Option<u64> {
    if mode == "fast" && matches!(command, Cmd::DecompileAll | Cmd::Project(_)) {
        Some(FAST_WHOLE_BINARY_FN_BUDGET_SECONDS)
    } else {
        None
    }
}

fn resolve_mode(requested: Option<&str>, binary_size: u64) -> Result<&'static str, String> {
    kuna_decomp::modes::resolve_mode_for_size(requested, binary_size).ok_or_else(|| {
        let requested = requested.unwrap_or("auto");
        let known: Vec<&str> = kuna_decomp::modes::mode_names().collect();
        format!("unknown mode {requested:?} (known: {})", known.join(", "))
    })
}

/// The `project` command: the `kuna decompile-project` flow
/// (`decompile_project.rs::decompile_project`) with the folder write replaced
/// by one JSON document of the four artifacts. Whole binary only; the display
/// name (default: the binary's basename) names the artifacts. No
/// `canonicalize()` — WASI virtual paths are used as given.
fn project(binary: &str, prog: &mut ConsoleProgram, display: &str) -> Result<String, String> {
    let targets = resolve_targets(prog, &Cmd::DecompileAll)?;
    if targets.is_empty() {
        return Err(format!("no functions discovered in {binary}"));
    }

    let mut results = decompile_export_targets(prog, targets);
    // Every artifact is address-ordered (the CLI's convention).
    results.sort_by(|a, b| a.address.cmp(&b.address).then_with(|| a.name.cmp(&b.name)));

    // `print_c_types` AFTER the decompile loop: user-defined types are interned
    // into the factory as functions decompile.
    let prelude = print_c_recompile_prelude(prog.arch());
    let types = print_c_types(prog.arch_mut());

    let header = build_header(display, &prelude, &types, &results);
    let c_file = build_c(display, &results);
    let dat_addrs = collect_dat_addrs(&results);
    let asm = build_asm(prog, &results, &dat_addrs, display);
    // The wasm surface labels the README with the virtual display name (the
    // CLI prints its canonicalized on-disk path there).
    let readme = build_readme(std::path::Path::new(binary), display, display, prog, &results);

    let ok = results.iter().filter(|r| r.error.is_none()).count();
    Ok(project_json(
        binary,
        display,
        results.len(),
        ok,
        &[
            (format!("{display}.c"), &c_file),
            (format!("{display}.h"), &header),
            (format!("{display}.asm"), &asm),
            ("README.md".to_string(), &readme),
        ],
        &prog.assertion_outcomes(),
    ))
}

/// The output language for this run: an explicit name, or the auto policy.
///
/// The auto policy follows the binary through `sourcelang::detect_compiler`, the
/// port of Ghidra's `SourceLanguageAnalyzer`. Detection is high-precision (a
/// `.comment` `rustc version` record, a `.rodata` signature, or a Rust-mangled
/// symbol), and a failure to parse the file leaves the C default in place -- the
/// policy can only ever ADD a language, never take one away.
fn resolve_language(
    binary: &str,
    requested: Option<&str>,
    command: &Cmd,
) -> Result<Option<&'static str>, String> {
    let explicit = match requested {
        None | Some("auto") | Some("") => None,
        Some(name) => Some(
            kuna_decomp::kuna_lang::OutLang::from_print_name(name)
                .ok_or_else(|| {
                    format!(
                        "unknown output language {name:?} (expected auto, or one of: {})",
                        kuna_decomp::kuna_lang::OutLang::names().join(", ")
                    )
                })?
                .print_name(),
        ),
    };
    if matches!(command, Cmd::Project(_)) {
        return match explicit {
            Some(name) if name != "c-language" => Err(format!(
                "project export is C-only in this release (got {name}); use `decompile`"
            )),
            // Never auto-select for the project export: it would turn a working
            // export into an error on every Rust binary.
            _ => Ok(None),
        };
    }
    if explicit.is_some() {
        return Ok(explicit);
    }
    let Ok(bytes) = kuna_analysis::loader::elf_shdr::read_image(binary) else {
        return Ok(None);
    };
    let Ok(file) = kuna_analysis::loadimage_object::parse_object(&*bytes) else {
        return Ok(None);
    };
    Ok(match kuna_analysis::sourcelang::detect_compiler(&file, &bytes) {
        kuna_analysis::sourcelang::Compiler::Rustc => Some("rust-language"),
        _ => None,
    })
}

/// Bootstrap the architecture from the binary and run the analysis commit — the
/// in-process `load file` + `read symbols`, then inject the `decompile-all`
/// surface's discovery defaults unless the selected mode owns those options
/// (matching the CLI's mode-then-explicit-default ordering).
///
/// EVERY command but `read` gets the injections, `list` included: in the
/// browser the inventory is a *product surface* (the sidebar is the only way to
/// reach a function), so an inventory that disagrees with the `project` export
/// is a missing-function bug, not a saved analysis. `kuna functions` makes the
/// opposite trade — cheap enumeration — because a native caller can always ask
/// `decompile-all` for the full set (DIV-53, `docs/web-integration.md` §2).
/// `read` (`discovery` false) needs no inventory at all, so it turns the
/// discovery walk off, as the CLI's caller-bounded listing does.
///
/// The directives land in the CLI's order: read-only propagation when a
/// `readonly` range implies it, the image-scoped ones (byte overlays) before
/// the analysis commit, the program-scoped ones after it; the function- and
/// symbol-scoped ones are dispatched by the decompile loop.
fn load_program(
    binary: &str,
    spec_root: &str,
    mode: &str,
    want_fast_funcdisc: bool,
    language: Option<&str>,
    directives: &[Directive],
    discovery: bool,
) -> Result<ConsoleProgram, String> {
    let overrides = kuna_decomp::modes::mode_overrides(mode)
        .ok_or_else(|| format!("unknown mode {mode:?}"))?;
    let owns_arm64e = overrides
        .iter()
        .any(|(option, _)| *option == "macho-arm64e");
    let previous_arm64e = std::env::var_os("KUNA_MACHO_ARM64E");
    if owns_arm64e {
        std::env::remove_var("KUNA_MACHO_ARM64E");
    }
    if overrides
        .iter()
        .any(|(option, value)| *option == "macho-arm64e" && *value == "on")
    {
        std::env::set_var("KUNA_MACHO_ARM64E", "1");
    }

    let spec_roots = vec![spec_root.to_string()];
    let bootstrap = bootstrap_from_image(binary, "", &spec_roots);
    if owns_arm64e {
        match previous_arm64e {
            Some(value) => std::env::set_var("KUNA_MACHO_ARM64E", value),
            None => std::env::remove_var("KUNA_MACHO_ARM64E"),
        }
    }
    let mut prog = bootstrap
        .map_err(|e| format!("could not build an architecture for {binary}: {}", e.explain()))?;

    prog.arch_mut()
        .apply_mode(mode)
        .map_err(|e| format!("mode {mode}: {}", e.explain()))?;
    let set = |prog: &mut ConsoleProgram, name: &str, value: &str| {
        prog.arch_mut()
            .set_kuna_option(name, value)
            .map_err(|e| format!("option {name}: {}", e.explain()))
    };
    if !want_fast_funcdisc {
        set(&mut prog, "fast_funcdisc", "off")?;
    }
    let mode_owns = |name: &str| overrides.iter().any(|(option, _)| *option == name);

    if !discovery {
        set(&mut prog, "listing", "off")?;
    } else if !mode_owns("listing") {
        set(&mut prog, "listing", "on")?;
    }
    if let Some(name) = language {
        prog.arch_mut()
            .set_print_language_checked(name)
            .map_err(|e| e.explain().to_string())?;
    }

    use object::Object;
    let non_x86_64 = if discovery && (!mode_owns("funcstart_patterns") || !mode_owns("aif")) {
        kuna_analysis::loader::elf_shdr::read_image(binary)
            .ok()
            .and_then(|bytes| {
                kuna_analysis::loadimage_object::parse_object(&*bytes)
                    .ok()
                    .map(|file| file.architecture() != object::Architecture::X86_64)
            })
            .unwrap_or(false)
    } else {
        false
    };
    if non_x86_64 && !mode_owns("funcstart_patterns") {
        set(&mut prog, "funcstart_patterns", "on")?;
    }
    if non_x86_64 && !mode_owns("aif") {
        set(&mut prog, "aif", "on")?;
    }

    if kuna_console::assertions::implies_readonly_propagation(directives) {
        prog.arch_mut().readonlypropagate = true;
    }
    if !directives.is_empty() {
        prog.set_assertions(directives.to_vec());
        kuna_console::assertions::apply_image_scoped(&mut prog);
    }
    prog.commit_pending_analysis()
        .map_err(|e| format!("read symbols (analysis commit) failed: {}", e.explain()))?;
    if !directives.is_empty() {
        kuna_console::assertions::apply_program_scoped(&mut prog);
    }
    Ok(prog)
}

/// Resolve the `(name, entry)` decompile targets for a `decompile` command.
fn resolve_targets(
    prog: &ConsoleProgram,
    command: &Cmd,
) -> Result<Vec<FunctionEntry>, String> {
    let one = |selector: &EntrySelector| {
        prog.resolve_body_entry(selector)
            .map(|entry| vec![entry])
            .map_err(|error| error.to_string())
    };
    match command {
        // Automatic whole-binary runs target code, not import pointer slots.
        Cmd::DecompileAll => Ok(prog.function_entries_executable()),
        // An ALIAS resolves too — collapsing the enumeration must not make a
        // name that used to select a function stop working.
        Cmd::DecompileName(want) | Cmd::Inspect(Selector::Name(want)) => {
            one(&EntrySelector::parse(want))
        }
        Cmd::DecompileAddr(vma) | Cmd::Inspect(Selector::Addr(vma)) => {
            one(&EntrySelector::Numeric(*vma))
        }
        Cmd::List | Cmd::Project(_) | Cmd::Read { .. } => {
            Err("this command selects no decompile targets".to_string())
        }
    }
}

/// Parse a `0x`-prefixed (or bare hex) address, else `None` (treat as a name).
fn parse_addr(s: &str) -> Option<u64> {
    let t = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X"))?;
    u64::from_str_radix(t, 16).ok()
}

/// A byte count: decimal, or `0x` hex.
fn parse_len(s: &str) -> Option<u64> {
    match s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
        Some(hex) => u64::from_str_radix(hex, 16).ok(),
        None => s.parse().ok(),
    }
}

// --- JSON (self-contained; the `decompile-all --json` fields + `kind`) ------

/// The `list` document:
/// `{binary, language, target, count, sections, known_types, assertions,
/// functions:[{name, address, address_hex, aliases, object_location, size, kind}]}`.
/// `size` is the entry's byte extent (`kuna_console::funcextent` — an upper
/// bound), so the browser inventory can rank its rows by weight without
/// decompiling every function.
/// `kinds` is parallel to `entries` (the classifier's verdict per entry).
fn list_json(
    binary: &str,
    prog: &ConsoleProgram,
    language: &str,
    entries: &[FunctionEntry],
    kinds: &[&'static str],
) -> String {
    let mut s = String::from("{\n");
    s.push_str(&format!("  \"binary\": {},\n", json_str(binary)));
    s.push_str(&format!("  \"language\": {},\n", json_str(language)));
    s.push_str(&format!("  \"target\": {},\n", inspect::target_json(prog)));
    s.push_str(&format!("  \"count\": {},\n", entries.len()));
    s.push_str(&format!(
        "  \"sections\": {},\n",
        inspect::sections_json(&kuna_console::inspect::section_rows(prog))
    ));
    s.push_str(&format!("  \"known_types\": {},\n", inspect::known_types_json(prog)));
    s.push_str(&format!("  \"assertions\": {},\n", assertions_json(&prog.assertion_outcomes())));
    s.push_str("  \"functions\": [");
    for (i, e) in entries.iter().enumerate() {
        let addr = e.addr.get_offset();
        s.push_str(if i == 0 { "\n" } else { ",\n" });
        s.push_str("    {");
        s.push_str(&format!("\"name\": {}, ", json_str(&e.name)));
        s.push_str(&format!("\"address\": {}, ", addr));
        s.push_str(&format!("\"address_hex\": {}, ", json_str(&format!("0x{addr:x}"))));
        s.push_str(&format!("\"aliases\": {}, ", json_str_array(&e.aliases)));
        s.push_str(&format!(
            "\"object_location\": {}, ",
            json_object_location(e.object_location.as_ref())
        ));
        s.push_str(&format!("\"size\": {}, ", e.size));
        s.push_str(&format!("\"kind\": {}", json_str(kinds[i])));
        s.push('}');
    }
    s.push_str(if entries.is_empty() { "]\n}" } else { "\n  ]\n}" });
    s
}

/// The `decompile` document (`decompile_all.rs::result_json`'s fields with
/// `kind` after `address_hex`). `kinds` is parallel to `funcs`.
fn result_json(
    binary: &str,
    language: &str,
    funcs: &[FuncResult],
    kinds: &[&'static str],
    assertions: &[Outcome],
) -> String {
    let mut s = String::from("{\n");
    s.push_str(&format!("  \"binary\": {},\n", json_str(binary)));
    s.push_str(&format!("  \"language\": {},\n", json_str(language)));
    s.push_str(&format!("  \"count\": {},\n", funcs.len()));
    s.push_str(&format!("  \"assertions\": {},\n", assertions_json(assertions)));
    s.push_str("  \"functions\": [");
    for (i, f) in funcs.iter().enumerate() {
        s.push_str(if i == 0 { "\n" } else { ",\n" });
        s.push_str("    {\n");
        s.push_str(&format!("      \"name\": {},\n", json_str(&f.name)));
        s.push_str(&format!("      \"address\": {},\n", f.address));
        s.push_str(&format!("      \"address_hex\": {},\n", json_str(&format!("0x{:x}", f.address))));
        s.push_str(&format!("      \"aliases\": {},\n", json_str_array(&f.aliases)));
        s.push_str(&format!(
            "      \"object_location\": {},\n",
            json_object_location(f.object_location.as_ref())
        ));
        s.push_str(&format!("      \"kind\": {},\n", json_str(kinds[i])));
        s.push_str(&format!("      \"size\": {},\n", f.size));
        s.push_str(&format!("      \"code\": {},\n", json_opt_str(f.code.as_deref())));
        s.push_str(&format!("      \"error\": {},\n", json_opt_str(f.error.as_deref())));
        s.push_str(&format!("      \"unstructured_gotos\": {},\n", f.unstructured_gotos()));
        s.push_str(&format!(
            "      \"line_mappings\": {},\n",
            arr(f.line_mappings.iter().map(|m| {
                Obj::new()
                    .num("line_number", m.line_number)
                    .raw("addresses", &arr(m.addresses.iter().map(u64::to_string)))
                    .end()
            }))
        ));
        s.push_str(&format!(
            "      \"types\": {},\n",
            arr(f.types.iter().map(|t| {
                Obj::new().str("name", &t.name).str("definition", &t.definition).num("size", t.size).end()
            }))
        ));
        s.push_str("      \"variables\": [");
        for (j, v) in f.variables.iter().enumerate() {
            s.push_str(if j == 0 { "\n" } else { ",\n" });
            s.push_str("        {");
            s.push_str(&format!("\"name\": {}, ", json_str(&v.name)));
            s.push_str(&format!("\"type\": {}, ", json_str(&v.type_name)));
            s.push_str(&format!("\"kind\": {}, ", json_str(if v.is_param { "arg" } else { "stack" })));
            s.push_str(&format!("\"arg_index\": {}, ", json_opt_num(v.arg_index.map(|i| i as i64))));
            s.push_str(&format!("\"stack_offset\": {}, ", json_opt_num(v.stack_offset)));
            s.push_str(&format!("\"size\": {}, ", v.size));
            s.push_str(&format!(
                "\"line_numbers\": {}, ",
                arr(v.line_numbers.iter().map(usize::to_string))
            ));
            s.push_str(&format!("\"addresses\": {}", arr(v.addresses.iter().map(u64::to_string))));
            s.push('}');
        }
        s.push_str(if f.variables.is_empty() { "]\n" } else { "\n      ]\n" });
        s.push_str("    }");
    }
    s.push_str(if funcs.is_empty() { "]\n}" } else { "\n  ]\n}" });
    s
}

/// The `project` document:
/// `{binary, name, count, ok, failed, assertions, files:{"<display>.c":…,
/// "<display>.h":…, "<display>.asm":…, "README.md":…}}` — the four artifact
/// bodies as (large) JSON strings, `json_str`-escaped.
fn project_json(
    binary: &str,
    display: &str,
    count: usize,
    ok: usize,
    files: &[(String, &String)],
    assertions: &[Outcome],
) -> String {
    let mut s = String::from("{\n");
    s.push_str(&format!("  \"binary\": {},\n", json_str(binary)));
    s.push_str(&format!("  \"name\": {},\n", json_str(display)));
    s.push_str(&format!("  \"count\": {},\n", count));
    s.push_str(&format!("  \"ok\": {},\n", ok));
    s.push_str(&format!("  \"failed\": {},\n", count - ok));
    s.push_str(&format!("  \"assertions\": {},\n", assertions_json(assertions)));
    s.push_str("  \"files\": {");
    for (i, (name, text)) in files.iter().enumerate() {
        s.push_str(if i == 0 { "\n" } else { ",\n" });
        s.push_str(&format!("    {}: {}", json_str(name), json_str(text)));
    }
    s.push_str(if files.is_empty() { "}\n}" } else { "\n  }\n}" });
    s
}

#[cfg(test)]
mod study_tests;

#[cfg(test)]
mod tests {
    use super::{command_fn_budget_seconds, command_wants_fast_funcdisc, resolve_mode, Cmd, Selector};
    use kuna_console::project::FAST_WHOLE_BINARY_FN_BUDGET_SECONDS;
    use kuna_decomp::modes::{AUTO_FAST_MIN_BYTES, AUTO_RELIABLE_MIN_BYTES};
    use std::path::PathBuf;

    #[test]
    fn auto_mode_uses_exact_browser_size_boundaries() {
        assert_eq!(
            resolve_mode(Some("auto"), AUTO_RELIABLE_MIN_BYTES - 1).unwrap(),
            "aggressive"
        );
        assert_eq!(
            resolve_mode(Some("auto"), AUTO_RELIABLE_MIN_BYTES).unwrap(),
            "reliable"
        );
        assert_eq!(resolve_mode(None, AUTO_FAST_MIN_BYTES - 1).unwrap(), "reliable");
        assert_eq!(resolve_mode(None, AUTO_FAST_MIN_BYTES).unwrap(), "fast");
    }

    #[test]
    fn explicit_mode_overrides_binary_size() {
        assert_eq!(resolve_mode(Some("fast"), 1).unwrap(), "fast");
        assert_eq!(
            resolve_mode(Some("aggressive"), 4 * AUTO_FAST_MIN_BYTES).unwrap(),
            "aggressive"
        );
        assert!(resolve_mode(Some("turbo"), 1).unwrap_err().contains("unknown mode"));
    }

    #[test]
    fn only_address_selection_skips_fast_discovery() {
        assert!(!command_wants_fast_funcdisc(&Cmd::DecompileAddr(0x1234)));
        assert!(command_wants_fast_funcdisc(&Cmd::DecompileName("sub_1234".into())));
        assert!(command_wants_fast_funcdisc(&Cmd::DecompileAll));
        assert!(command_wants_fast_funcdisc(&Cmd::Project("binary".into())));
        assert!(command_wants_fast_funcdisc(&Cmd::List));
        assert!(!command_wants_fast_funcdisc(&Cmd::Inspect(Selector::Addr(0x1234))));
        assert!(command_wants_fast_funcdisc(&Cmd::Inspect(Selector::Name("main".into()))));
        assert!(!command_wants_fast_funcdisc(&Cmd::Read { addr: 0x1234, len: 4 }));
    }

    #[test]
    fn fast_whole_binary_commands_use_the_short_watchdog() {
        assert_eq!(command_fn_budget_seconds(&Cmd::List, "fast"), None);
        assert_eq!(
            command_fn_budget_seconds(&Cmd::DecompileAll, "fast"),
            Some(FAST_WHOLE_BINARY_FN_BUDGET_SECONDS)
        );
        assert_eq!(
            command_fn_budget_seconds(&Cmd::Project("binary".into()), "fast"),
            Some(FAST_WHOLE_BINARY_FN_BUDGET_SECONDS)
        );
        assert_eq!(command_fn_budget_seconds(&Cmd::DecompileAddr(0x1234), "fast"), None);
        assert_eq!(
            command_fn_budget_seconds(&Cmd::DecompileName("main".into()), "fast"),
            None
        );
        assert_eq!(command_fn_budget_seconds(&Cmd::DecompileAll, "reliable"), None);
        assert_eq!(
            command_fn_budget_seconds(&Cmd::Project("binary".into()), "aggressive"),
            None
        );
    }

    #[test]
    fn fast_project_exports_discovered_wasm_bodies() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../..")
            .canonicalize()
            .unwrap();
        let binary = root.join("decompiler/crates/kuna-analysis/tests/fixtures/pdb_prog.exe");
        let specs = root.join("specs");
        let result = super::run_with_mode(
            binary.to_str().unwrap(),
            specs.to_str().unwrap(),
            "project",
            Some("pdb_prog.exe"),
            Some("fast"),
        );
        let json = match result {
            Ok(json) => json,
            Err(error)
                if error.contains("could not build an architecture")
                    || error.contains("SLEIGH")
                    || error.contains("Could not discover") =>
            {
                eprintln!("fast_project_exports_discovered_wasm_bodies: skipping: {error}");
                return;
            }
            Err(error) => panic!("WASM fast project failed: {error}"),
        };
        assert!(
            json.contains("@ 0x140001000"),
            "hidden direct callee missing: {json}"
        );
        assert!(
            json.contains("return a1 * 7 + a0 * 3;"),
            "hidden direct callee has no real body: {json}"
        );
    }

    /// Direct browser decompilation uses body-bearing selection too; inventory
    /// remains free to retain this same import symbol for call binding.
    #[test]
    fn wasm_direct_decompile_refuses_an_executable_section_iat_slot() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../..")
            .canonicalize()
            .unwrap();
        let binary =
            root.join("decompiler/crates/kuna-analysis/tests/fixtures/pe_iatincode_i386.exe");
        let specs = root.join("specs");
        let expected = "identifies import VirtualAlloc at 0x401000; \
                        the IAT slot contains a loader-written pointer, not a function body";
        for selector in ["VirtualAlloc", "0x401000"] {
            match super::run_with_mode(
                binary.to_str().unwrap(),
                specs.to_str().unwrap(),
                "decompile",
                Some(selector),
                Some("reliable"),
            ) {
                Err(error)
                    if error.contains("could not build an architecture")
                        || error.contains("SLEIGH")
                        || error.contains("Could not discover") =>
                {
                    eprintln!("wasm_direct_decompile_iat: skipping: {error}");
                    return;
                }
                Err(error) => {
                    assert!(error.contains(expected), "{selector}: {error}");
                    assert!(!error.contains("CARRY1("), "{selector}: {error}");
                }
                Ok(json) => panic!("{selector} unexpectedly emitted a WASM result: {json}"),
            }
        }
    }

    /// The browser sidebar is built from `list`, so anything `project` exports
    /// but `list` omits is unreachable in the UI. `list` used to skip the
    /// discovery injections (`kuna functions`' cheap-enumeration trade), which
    /// on a non-x86-64 target hid every prologue-scan/AIF-found function: a
    /// 1.1 MiB i386 PE listed 308 entries while its own project export carried
    /// 3015 functions.
    #[test]
    fn reliable_list_inventory_covers_the_project_export() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../..")
            .canonicalize()
            .unwrap();
        // Non-x86-64: the funcstart-pattern and AIF injections are arch-gated,
        // and this is the family the browser was losing functions on.
        let binary = root.join("decompiler/crates/kuna-analysis/tests/fixtures/entrymain_arm");
        let specs = root.join("specs");
        let run = |cmd: &str, arg: Option<&str>| {
            super::run_with_mode(
                binary.to_str().unwrap(),
                specs.to_str().unwrap(),
                cmd,
                arg,
                Some("reliable"),
            )
        };
        let skip = |error: &str| {
            error.contains("could not build an architecture")
                || error.contains("SLEIGH")
                || error.contains("Could not discover")
        };
        let (list, project) = match (run("list", None), run("project", Some("entrymain_arm"))) {
            (Ok(list), Ok(project)) => (list, project),
            (Err(error), _) | (_, Err(error)) if skip(&error) => {
                eprintln!("reliable_list_inventory_covers_the_project_export: skipping: {error}");
                return;
            }
            (Err(error), _) | (_, Err(error)) => panic!("WASM reliable run failed: {error}"),
        };

        let listed: Vec<&str> = list.match_indices("\"address_hex\": \"").map(|(i, m)| {
            let rest = &list[i + m.len()..];
            &rest[..rest.find('"').unwrap()]
        }).collect();
        let exported: Vec<&str> = project.match_indices("@ 0x").map(|(i, _)| {
            let rest = &project[i + 2..];
            let end = rest.find(|c: char| !c.is_ascii_hexdigit() && c != 'x').unwrap();
            &rest[..end]
        }).collect();

        assert!(!exported.is_empty(), "project exported no functions: {project}");
        for addr in &exported {
            assert!(
                listed.contains(addr),
                "{addr} is in the project export but not in the sidebar inventory \
                 (listed {}, exported {})",
                listed.len(),
                exported.len()
            );
        }
    }

    #[test]
    fn relocatable_selectors_and_object_locations_reach_wasm() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../..")
            .canonicalize()
            .unwrap();
        let binary = root.join(
            "decompiler/crates/kuna-analysis/tests/fixtures/entry_selectors_x86_64.o",
        );
        let specs = root.join("specs");
        let run = |cmd: &str, arg: Option<&str>| {
            super::run_with_mode(
                binary.to_str().unwrap(),
                specs.to_str().unwrap(),
                cmd,
                arg,
                Some("reliable"),
            )
        };
        let skip = |error: &str| {
            error.contains("could not build an architecture")
                || error.contains("SLEIGH")
                || error.contains("Could not discover")
        };

        let list = match run("list", None) {
            Ok(list) => list,
            Err(error) if skip(&error) => {
                eprintln!("relocatable_selectors_and_object_locations_reach_wasm: skipping: {error}");
                return;
            }
            Err(error) => panic!("WASM relocatable list failed: {error}"),
        };
        assert_eq!(list.matches("\"name\": \"duplicate_local\"").count(), 2);
        assert!(
            list.contains(
                "\"object_location\": {\"section_index\": 4, \"section\": \".text.selector_a\", \"offset\": 0, \"offset_hex\": \"0x0\"}"
            ),
            "{list}"
        );
        assert!(
            list.contains(
                "\"object_location\": {\"section_index\": 6, \"section\": \".text.selector_b\", \"offset\": 0, \"offset_hex\": \"0x0\"}"
            ),
            "{list}"
        );

        let error = run("decompile", Some("duplicate_local"))
            .expect_err("duplicate WASM name must be ambiguous");
        assert!(error.contains("ambiguous"), "{error}");
        assert!(error.contains(".text.selector_a+0x0"), "{error}");
        assert!(error.contains(".text.selector_b+0x0"), "{error}");

        let selected = run("decompile", Some("6:0x0"))
            .expect("WASM section-index selector must decompile");
        assert!(selected.contains("\"count\": 1"), "{selected}");
        assert!(
            selected.contains(
                "\"object_location\": {\"section_index\": 6, \"section\": \".text.selector_b\", \"offset\": 0, \"offset_hex\": \"0x0\"}"
            ),
            "{selected}"
        );
        assert!(selected.contains("return 2;"), "{selected}");
    }
}
