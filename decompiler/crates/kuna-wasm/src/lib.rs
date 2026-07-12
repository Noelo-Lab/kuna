//! kuna-wasm: the in-browser decompiler front-end.
//!
//! This is the *engine's* in-process path (`kuna_console::engine::
//! bootstrap_from_object` → `commit_pending_analysis` → loop
//! `decompile_func_full_with_override_dyn` + `print_c`) wrapped in a tiny,
//! dependency-light CLI that reads its inputs from the (virtual) filesystem and
//! writes JSON/C to stdout — exactly the contract a browser WASI shim provides.
//! It is a faithful, self-contained restatement of `kuna decompile-all`'s core
//! loop (`kuna-cli/src/decompile_all.rs`), reproduced here rather than shared
//! because `kuna-cli` pulls in the subprocess/CLI machinery that cannot compile
//! for `wasm32-wasip1`. The Node-WASI parity test in `integrations/web/test/`
//! pins native-vs-wasm output equality; the `--json` shape matches
//! `kuna decompile-all --json` field-for-field.
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
use kuna_decomp::decompile_drive::{
    decompile_func_full_with_override_dyn, extract_variables, print_c, VarInfo,
};

/// One decompiled function's result (mirrors `decompile_all.rs::FuncResult`).
struct FuncResult {
    name: String,
    address: u64,
    size: i64,
    code: Option<String>,
    error: Option<String>,
    variables: Vec<VarInfo>,
}

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
        other => return Err(format!("unknown command: {other:?} (want `list` or `decompile`)")),
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
            Ok(list_json(binary, &entries))
        }
        _ => {
            let targets = resolve_targets(&prog, &command)?;
            let mut out = Vec::with_capacity(targets.len());
            for (name, entry) in targets {
                out.push(decompile_one(&mut prog, name, entry));
            }
            Ok(result_json(binary, &out))
        }
    }
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
        Cmd::List => unreachable!("List handled by caller"),
    }
}

/// Decompile a single function, mirroring `decompile_all.rs`'s per-function body
/// (DWARF re-seed + `CALL_RETURN` flow overrides for error-noreturn call sites),
/// containing a printer/type-declarator panic to a per-function error record.
fn decompile_one(prog: &mut ConsoleProgram, name: String, entry: Address) -> FuncResult {
    let address = entry.get_offset();
    let mapped = prog.dwarf_locals_for(address);

    let flow_ovr: Vec<(Address, kuna_base::types::uint4)> = match entry.get_space() {
        Some(space) if !prog.arch().error_noreturn_callsites.is_empty() => prog
            .arch()
            .error_noreturn_callsites
            .iter()
            .map(|&off| {
                (
                    Address::new(Rc::clone(space), off),
                    kuna_decomp::overrides::flow_type::CALL_RETURN,
                )
            })
            .collect(),
        _ => Vec::new(),
    };

    match decompile_func_full_with_override_dyn(
        prog.arch_mut(),
        &name,
        entry.clone(),
        0,
        &mapped,
        &[],
        &[],
        None,
        &flow_ovr,
        &[],
        &[],
    ) {
        Ok(fd) => {
            let size = fd.get_size() as i64;
            let rendered = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let code = print_c(prog.arch_mut(), &fd).trim_matches('\n').to_string();
                let variables = extract_variables(prog.arch(), &fd);
                (code, variables)
            }));
            match rendered {
                Ok((code, variables)) => FuncResult {
                    name,
                    address,
                    size,
                    code: Some(code),
                    error: None,
                    variables,
                },
                Err(_) => FuncResult {
                    name,
                    address,
                    size: 0,
                    code: None,
                    error: Some("panic while rendering C / extracting variables".into()),
                    variables: Vec::new(),
                },
            }
        }
        Err(e) => FuncResult {
            name,
            address,
            size: 0,
            code: None,
            error: Some(e.explain().to_string()),
            variables: Vec::new(),
        },
    }
}

/// Parse a `0x`-prefixed (or bare hex) address, else `None` (treat as a name).
fn parse_addr(s: &str) -> Option<u64> {
    let t = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X"))?;
    u64::from_str_radix(t, 16).ok()
}

// --- JSON (self-contained; matches `decompile-all --json` field-for-field) ---

/// The `list` document: `{binary, count, functions:[{name, address, address_hex}]}`.
fn list_json(binary: &str, entries: &[(String, u64)]) -> String {
    let mut s = String::from("{\n");
    s.push_str(&format!("  \"binary\": {},\n", json_str(binary)));
    s.push_str(&format!("  \"count\": {},\n", entries.len()));
    s.push_str("  \"functions\": [");
    for (i, (name, addr)) in entries.iter().enumerate() {
        s.push_str(if i == 0 { "\n" } else { ",\n" });
        s.push_str("    {");
        s.push_str(&format!("\"name\": {}, ", json_str(name)));
        s.push_str(&format!("\"address\": {}, ", addr));
        s.push_str(&format!("\"address_hex\": {}", json_str(&format!("0x{addr:x}"))));
        s.push('}');
    }
    s.push_str(if entries.is_empty() { "]\n}" } else { "\n  ]\n}" });
    s
}

/// The `decompile` document (matches `decompile_all.rs::result_json`).
fn result_json(binary: &str, funcs: &[FuncResult]) -> String {
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
