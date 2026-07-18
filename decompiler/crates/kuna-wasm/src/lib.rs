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
//! `kuna decompile-all --json` fields plus a kuna-wasm-only `"kind"`
//! (`"func"` | `"plt"` | `"thunk"` — see `classify.rs`) on every function entry.
//!
//! # Why WASI
//! The decompiler touches the outside world only through plain `std::fs` path
//! reads (the binary via `LoadImage`, the SLEIGH `.sla`/`.pspec`/`.cspec`/
//! `.ldefs` via `scan_language_database`). Those map 1:1 onto a WASI virtual
//! filesystem, so this front-end runs in the browser with **zero** engine
//! changes. See `docs/web-integration.md`.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_console::engine::{bootstrap_from_object, ConsoleProgram};
use kuna_console::project::{
    build_asm, build_c, build_header, build_readme, collect_dat_addrs, decompile_targets,
    FuncResult,
};
use kuna_decomp::decompile_drive::{print_c_recompile_prelude, print_c_types};

mod classify;
use classify::Classifier;

/// Parsed command.
enum Cmd {
    /// Enumerate functions only (cheap: no per-function decompile, Listing off).
    List,
    /// Decompile every function.
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

/// Run the front-end. `binary` and `spec_root` are (virtual) filesystem paths;
/// `cmd`/`arg` come from argv. Returns the stdout payload (JSON) on success.
pub fn run(binary: &str, spec_root: &str, cmd: &str, arg: Option<&str>) -> Result<String, String> {
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

    let want_decompile = !matches!(command, Cmd::List);
    let mut prog = load_program(binary, spec_root, want_decompile)?;

    match command {
        Cmd::List => {
            let mut entries: Vec<(String, u64)> = prog
                .function_entries()
                .map(|(n, a)| (n.to_string(), a.get_offset()))
                .collect();
            entries.sort_by(|a, b| a.1.cmp(&b.1).then_with(|| a.0.cmp(&b.0)));
            entries.dedup();
            let classifier =
                Classifier::new(&prog, binary, entries.iter().map(|&(_, vma)| vma));
            let kinds: Vec<&'static str> =
                entries.iter().map(|(n, vma)| classifier.kind(&prog, n, *vma)).collect();
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
                prog.function_entries().map(|(_, a)| a.get_offset()),
            );
            let out = decompile_targets(&mut prog, targets, /* no_vars= */ false,
                /* want_proto= */ false);
            let kinds: Vec<&'static str> =
                out.iter().map(|f| classifier.kind(&prog, &f.name, f.address)).collect();
            Ok(result_json(binary, &out, &kinds))
        }
    }
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
        decompile_targets(prog, targets, /* no_vars= */ false, /* want_proto= */ true);
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
/// in-process `load file` + `read symbols`. `default_listing` injects the
/// `decompile-all` surface's defaults (Listing on; `funcstart_patterns` on for
/// non-x86-64) so the output matches `kuna decompile-all` (DIV-15 / DIV-20).
fn load_program(
    binary: &str,
    spec_root: &str,
    default_listing: bool,
) -> Result<ConsoleProgram, String> {
    let spec_roots = vec![spec_root.to_string()];
    let mut prog = bootstrap_from_object(binary, "", &spec_roots)
        .map_err(|e| format!("could not build an architecture for {binary}: {}", e.explain()))?;

    if default_listing {
        prog.arch_mut()
            .set_kuna_option("listing", "on")
            .map_err(|e| format!("option listing: {}", e.explain()))?;

        // (kuna, decbench ARM / DIV-20) `funcstart_patterns` is the primary
        // discovery source on a stripped **non-x86-64** binary; default it on
        // there, exactly as `decompile-all` does. x86-64 keeps it off.
        if let Ok(bytes) = std::fs::read(binary) {
            use object::Object;
            let non_x86_64 = object::File::parse(&*bytes)
                .map(|f| f.architecture() != object::Architecture::X86_64)
                .unwrap_or(false);
            if non_x86_64 {
                prog.arch_mut()
                    .set_kuna_option("funcstart_patterns", "on")
                    .map_err(|e| format!("option funcstart_patterns: {}", e.explain()))?;
            }
        }
    }

    prog.commit_pending_analysis()
        .map_err(|e| format!("read symbols (analysis commit) failed: {}", e.explain()))?;
    Ok(prog)
}

/// Resolve the `(name, entry)` decompile targets for a `decompile` command.
fn resolve_targets(
    prog: &ConsoleProgram,
    command: &Cmd,
) -> Result<Vec<(String, Address)>, String> {
    let code_space = prog
        .arch()
        .manage()
        .get_default_code_space()
        .ok_or("no default code space")?;

    let all: Vec<(String, Address)> = {
        let mut v: Vec<(String, Address)> = prog
            .function_entries()
            .map(|(n, a)| (n.to_string(), a.clone()))
            .collect();
        v.sort_by(|a, b| {
            a.1.get_offset().cmp(&b.1.get_offset()).then_with(|| a.0.cmp(&b.0))
        });
        v.dedup_by(|a, b| a.0 == b.0 && a.1.get_offset() == b.1.get_offset());
        v
    };

    match command {
        Cmd::DecompileAll => Ok(all),
        Cmd::DecompileName(want) => match all.iter().find(|(n, _)| n == want) {
            Some((n, a)) => Ok(vec![(n.clone(), a.clone())]),
            None => Err(format!("no function named {want:?} in the binary")),
        },
        Cmd::DecompileAddr(vma) => {
            let addr = Address::new(Rc::clone(&code_space), *vma);
            let name = prog
                .function_named_at(*vma)
                .unwrap_or_else(|| prog.arch().name_function(&addr));
            Ok(vec![(name, addr)])
        }
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
/// `{binary, count, functions:[{name, address, address_hex, kind}]}`.
/// `kinds` is parallel to `entries` (the classifier's verdict per entry).
fn list_json(binary: &str, entries: &[(String, u64)], kinds: &[&'static str]) -> String {
    let mut s = String::from("{\n");
    s.push_str(&format!("  \"binary\": {},\n", json_str(binary)));
    s.push_str(&format!("  \"count\": {},\n", entries.len()));
    s.push_str("  \"functions\": [");
    for (i, (name, addr)) in entries.iter().enumerate() {
        s.push_str(if i == 0 { "\n" } else { ",\n" });
        s.push_str("    {");
        s.push_str(&format!("\"name\": {}, ", json_str(name)));
        s.push_str(&format!("\"address\": {}, ", addr));
        s.push_str(&format!("\"address_hex\": {}, ", json_str(&format!("0x{addr:x}"))));
        s.push_str(&format!("\"kind\": {}", json_str(kinds[i])));
        s.push('}');
    }
    s.push_str(if entries.is_empty() { "]\n}" } else { "\n  ]\n}" });
    s
}

/// The `decompile` document (`decompile_all.rs::result_json`'s fields with the
/// kuna-wasm-only `kind` after `address_hex`). `kinds` is parallel to `funcs`.
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
