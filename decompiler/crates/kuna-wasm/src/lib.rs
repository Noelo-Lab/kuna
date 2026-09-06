//! kuna-wasm: the in-browser decompiler front-end.
//!
//! This is the *engine's* in-process path (`kuna_console::engine::
//! bootstrap_from_object` → `commit_pending_analysis` → loop
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
//! # Why WASI
//! The decompiler touches the outside world only through plain `std::fs` path
//! reads (the binary via `LoadImage`, the SLEIGH `.sla`/`.pspec`/`.cspec`/
//! `.ldefs` via `scan_language_database`). Those map 1:1 onto a WASI virtual
//! filesystem, so this front-end runs in the browser with **zero** engine
//! changes. See `docs/web-integration.md`.

use kuna_console::engine::{
    bootstrap_from_object, ConsoleProgram, EntrySelector, FunctionEntry, ObjectLocation,
};
use kuna_console::project::{
    build_asm, build_c, build_header, build_readme, collect_dat_addrs, decompile_targets,
    FuncResult, FAST_WHOLE_BINARY_FN_BUDGET_SECONDS,
};
use kuna_decomp::decompile_drive::{print_c_recompile_prelude, print_c_types};
// The per-function `kind` annotation, shared with `kuna decompile-graph`.
use kuna_console::classify::Classifier;

/// Parsed command.
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

/// Run the front-end with an optional explicit mode and output language.
///
/// For the mode, `None` and `Some("auto")` select `aggressive` below 500 KiB,
/// `reliable` from 500 KiB through just below 2 MiB, and `fast` at 2 MiB and
/// above.
///
/// (kuna outlang) For the language, `None` and `Some("auto")` follow the binary:
/// a Rust binary renders as Rust. `project` is excluded from that policy -- its
/// `.c`/`.h`/`.asm` export is C-shaped end to end -- so an explicit non-C
/// language is an error there rather than a broken export.
pub fn run_with(
    binary: &str,
    spec_root: &str,
    cmd: &str,
    arg: Option<&str>,
    requested_mode: Option<&str>,
    requested_language: Option<&str>,
) -> Result<String, String> {
    let command = match cmd {
        "list" => Cmd::List,
        "decompile" => match arg {
            None => Cmd::DecompileAll,
            Some(a) => match parse_addr(a) {
                Some(vma) => Cmd::DecompileAddr(vma),
                None => Cmd::DecompileName(a.to_string()),
            },
        },
        "project" => {
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
        other => {
            return Err(format!(
                "unknown command: {other:?} (want `list`, `decompile` or `project`)"
            ))
        }
    };

    let want_fast_funcdisc = command_wants_fast_funcdisc(&command);
    let requested = requested_mode.unwrap_or("auto");
    let binary_size = if kuna_decomp::modes::mode_is_automatic(requested) {
        std::fs::metadata(binary)
            .map_err(|e| format!("could not read binary metadata for {binary}: {e}"))?
            .len()
    } else {
        0
    };
    let mode = resolve_mode(requested_mode, binary_size)?;
    let language = resolve_language(binary, requested_language, &command)?;
    let mut prog = load_program(binary, spec_root, mode, want_fast_funcdisc, language)?;
    if let Some(seconds) = command_fn_budget_seconds(&command, mode) {
        prog.arch_mut().kuna_fn_budget = Some(std::time::Duration::from_secs(seconds));
    }

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
            Ok(list_json(binary, &entries, &kinds))
        }
        Cmd::Project(display) => project(binary, &mut prog, &display),
        _ => {
            let targets = resolve_targets(&prog, &command)?;
            // Classify against the FULL deduped entry set (a single-function
            // decompile still needs every entry for the thunk-target test).
            let classifier = Classifier::new(
                &prog,
                binary,
                prog.function_entries_canonical().iter().map(|e| e.addr.get_offset()),
            );
            let out = decompile_targets(
                &mut prog,
                targets,
                /* no_vars= */ false,
                /* want_proto= */ false,
                /* want_provenance= */ false,
            );
            let kinds: Vec<&'static str> =
                out.iter().map(|f| classifier.kind(&prog, &f.name, f.address)).collect();
            Ok(result_json(binary, &out, &kinds))
        }
    }
}

fn command_wants_fast_funcdisc(command: &Cmd) -> bool {
    !matches!(command, Cmd::DecompileAddr(_))
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

    let mut results =
        decompile_targets(
            prog,
            targets,
            /* no_vars= */ false,
            /* want_proto= */ true,
            /* want_provenance= */ false,
        );
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
    ))
}

/// Bootstrap the architecture from the binary and run the analysis commit — the
/// in-process `load file` + `read symbols`, then inject the `decompile-all`
/// surface's discovery defaults unless the selected mode owns those options
/// (matching the CLI's mode-then-explicit-default ordering).
///
/// EVERY command gets the injections, `list` included: in the browser the
/// inventory is a *product surface* (the sidebar is the only way to reach a
/// function), so an inventory that disagrees with the `project` export is a
/// missing-function bug, not a saved analysis. `kuna functions` makes the
/// opposite trade — cheap enumeration — because a native caller can always ask
/// `decompile-all` for the full set (DIV-53, `docs/web-integration.md` §2).
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
    let Ok(bytes) = std::fs::read(binary) else { return Ok(None) };
    let Ok(file) = kuna_analysis::loadimage_object::parse_object(&*bytes) else {
        return Ok(None);
    };
    Ok(match kuna_analysis::sourcelang::detect_compiler(&file, &bytes) {
        kuna_analysis::sourcelang::Compiler::Rustc => Some("rust-language"),
        _ => None,
    })
}

fn load_program(
    binary: &str,
    spec_root: &str,
    mode: &str,
    want_fast_funcdisc: bool,
    language: Option<&str>,
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
    let bootstrap = bootstrap_from_object(binary, "", &spec_roots);
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
    if !want_fast_funcdisc {
        prog.arch_mut()
            .set_kuna_option("fast_funcdisc", "off")
            .map_err(|e| format!("option fast_funcdisc: {}", e.explain()))?;
    }
    let mode_owns = |name: &str| overrides.iter().any(|(option, _)| *option == name);

    if !mode_owns("listing") {
        prog.arch_mut()
            .set_kuna_option("listing", "on")
            .map_err(|e| format!("option listing: {}", e.explain()))?;
    }
    if let Some(name) = language {
        prog.arch_mut()
            .set_print_language_checked(name)
            .map_err(|e| e.explain().to_string())?;
    }

    use object::Object;
    let non_x86_64 = if !mode_owns("funcstart_patterns") || !mode_owns("aif") {
        std::fs::read(binary)
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
        prog.arch_mut()
            .set_kuna_option("funcstart_patterns", "on")
            .map_err(|e| format!("option funcstart_patterns: {}", e.explain()))?;
    }
    if non_x86_64 && !mode_owns("aif") {
        prog.arch_mut()
            .set_kuna_option("aif", "on")
            .map_err(|e| format!("option aif: {}", e.explain()))?;
    }

    prog.commit_pending_analysis()
        .map_err(|e| format!("read symbols (analysis commit) failed: {}", e.explain()))?;
    Ok(prog)
}

/// Resolve the `(name, entry)` decompile targets for a `decompile` command.
fn resolve_targets(
    prog: &ConsoleProgram,
    command: &Cmd,
) -> Result<Vec<FunctionEntry>, String> {
    match command {
        // Automatic whole-binary runs target code, not import pointer slots.
        Cmd::DecompileAll => Ok(prog.function_entries_executable()),
        // An ALIAS resolves too — collapsing the enumeration must not make a
        // name that used to select a function stop working.
        Cmd::DecompileName(want) => prog
            .resolve_entry(&EntrySelector::parse(want))
            .map(|entry| vec![entry])
            .map_err(|error| error.to_string()),
        Cmd::DecompileAddr(vma) => prog
            .resolve_entry(&EntrySelector::Numeric(*vma))
            .map(|entry| vec![entry])
            .map_err(|error| error.to_string()),
        Cmd::List | Cmd::Project(_) => unreachable!("List/Project handled by caller"),
    }
}

/// Parse a `0x`-prefixed (or bare hex) address, else `None` (treat as a name).
fn parse_addr(s: &str) -> Option<u64> {
    let t = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X"))?;
    u64::from_str_radix(t, 16).ok()
}

// --- JSON (self-contained; the `decompile-all --json` fields + `kind`) ------

/// The `list` document:
/// `{binary, count, functions:[{name, address, address_hex, aliases, size, kind}]}`.
/// `size` is the entry's byte extent (`kuna_console::funcextent` — an upper
/// bound), so the browser inventory can rank its rows by weight without
/// decompiling every function.
/// `kinds` is parallel to `entries` (the classifier's verdict per entry).
fn list_json(binary: &str, entries: &[FunctionEntry], kinds: &[&'static str]) -> String {
    let mut s = String::from("{\n");
    s.push_str(&format!("  \"binary\": {},\n", json_str(binary)));
    s.push_str(&format!("  \"count\": {},\n", entries.len()));
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
fn result_json(binary: &str, funcs: &[FuncResult], kinds: &[&'static str]) -> String {
    let mut s = String::from("{\n");
    s.push_str(&format!("  \"binary\": {},\n", json_str(binary)));
    s.push_str(&format!("  \"count\": {},\n", funcs.len()));
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
        s.push_str("      \"variables\": [");
        for (j, v) in f.variables.iter().enumerate() {
            s.push_str(if j == 0 { "\n" } else { ",\n" });
            s.push_str("        {");
            s.push_str(&format!("\"name\": {}, ", json_str(&v.name)));
            s.push_str(&format!("\"type\": {}, ", json_str(&v.type_name)));
            s.push_str(&format!("\"kind\": {}, ", json_str(if v.is_param { "arg" } else { "stack" })));
            s.push_str(&format!("\"arg_index\": {}, ", json_opt_num(v.arg_index.map(|i| i as i64))));
            s.push_str(&format!("\"stack_offset\": {}, ", json_opt_num(v.stack_offset)));
            s.push_str(&format!("\"size\": {}", v.size));
            s.push('}');
        }
        s.push_str(if f.variables.is_empty() { "]\n" } else { "\n      ]\n" });
        s.push_str("    }");
    }
    s.push_str(if funcs.is_empty() { "]\n}" } else { "\n  ]\n}" });
    s
}

/// The `project` document:
/// `{binary, name, count, ok, failed, files:{"<display>.c":…, "<display>.h":…,
/// "<display>.asm":…, "README.md":…}}` — the four artifact bodies as (large)
/// JSON strings, `json_str`-escaped.
fn project_json(
    binary: &str,
    display: &str,
    count: usize,
    ok: usize,
    files: &[(String, &String)],
) -> String {
    let mut s = String::from("{\n");
    s.push_str(&format!("  \"binary\": {},\n", json_str(binary)));
    s.push_str(&format!("  \"name\": {},\n", json_str(display)));
    s.push_str(&format!("  \"count\": {},\n", count));
    s.push_str(&format!("  \"ok\": {},\n", ok));
    s.push_str(&format!("  \"failed\": {},\n", count - ok));
    s.push_str("  \"files\": {");
    for (i, (name, text)) in files.iter().enumerate() {
        s.push_str(if i == 0 { "\n" } else { ",\n" });
        s.push_str(&format!("    {}: {}", json_str(name), json_str(text)));
    }
    s.push_str(if files.is_empty() { "}\n}" } else { "\n  }\n}" });
    s
}

fn json_opt_str(v: Option<&str>) -> String {
    match v {
        Some(s) => json_str(s),
        None => "null".to_string(),
    }
}

fn json_opt_num(v: Option<i64>) -> String {
    match v {
        Some(n) => n.to_string(),
        None => "null".to_string(),
    }
}

/// (kuna, issue #197) A JSON array of strings — the `aliases` field: every
/// OTHER name the reported entry carries.  Always present (`[]` when the entry
/// has exactly one name).
fn json_str_array(items: &[String]) -> String {
    let mut out = String::from("[");
    for (i, s) in items.iter().enumerate() {
        if i > 0 {
            out.push_str(", ");
        }
        out.push_str(&json_str(s));
    }
    out.push(']');
    out
}

fn json_object_location(location: Option<&ObjectLocation>) -> String {
    match location {
        Some(location) => format!(
            "{{\"section_index\": {}, \"section\": {}, \"offset\": {}, \"offset_hex\": {}}}",
            location.section_index,
            json_str(&location.section),
            location.offset,
            json_str(&format!("0x{:x}", location.offset))
        ),
        None => "null".to_string(),
    }
}

/// Encode a Rust string as a JSON string literal (RFC 8259 escaping).
fn json_str(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\u{08}' => out.push_str("\\b"),
            '\u{0c}' => out.push_str("\\f"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

#[cfg(test)]
mod tests {
    use super::{command_fn_budget_seconds, command_wants_fast_funcdisc, resolve_mode, Cmd};
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
