//! `kuna catalog` — the Rust port of `kuna/catalog.py`.
//!
//! Drives `decomp_dbg stage catalog [<option>]`, extracts the embedded JSON from
//! the console transcript, and re-emits it.  Modes:
//!   * (default) a human-readable table,
//!   * `--json`     the catalog as `json.dumps(indent=2)` (byte-identical to the
//!                  Python `--json`),
//!   * `--markdown` the `docs/assertions.md` body (the Python `to_markdown`),
//!   * `--check`    verify the catalog documents exactly the registered kuna
//!                  options.  In the Rust-only world the old `check_drift` (which
//!                  parsed the removed `decompiler/cpp/options.cc` + `kuna_*.hh`)
//!                  can no longer run; the faithful replacement cross-checks the
//!                  catalog the binary emits against the in-process registered
//!                  option set ([`kuna_decomp::options::KUNA_OPTION_NAMES`]),
//!                  catching the same drift (a registered option with no catalog
//!                  row, or a stale catalog row).

use std::collections::BTreeSet;
use std::process::Command;

use kuna_decomp::options::KUNA_OPTION_NAMES;

use crate::jsonfmt::{dumps_indent2, extract_json_span, parse, Json};
use crate::paths;

/// Run `decomp_dbg stage catalog [<option>]` and return the parsed JSON value
/// (an array, or a single object when `option` is given).
fn run_catalog(option: Option<&str>) -> Result<Json, String> {
    let bin = paths::decomp_dbg();
    if !bin.exists() {
        return Err(format!(
            "decomp_dbg not built at {} -- run `make binaries`",
            bin.display()
        ));
    }
    let specs = paths::specs_dir();
    let cmd = match option {
        None => "stage catalog\nquit\n".to_string(),
        Some(o) => format!("stage catalog {o}\nquit\n"),
    };

    let output = Command::new(&bin)
        .arg("-s")
        .arg(&specs)
        .env("SLEIGHHOME", &specs)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            use std::io::Write;
            if let Some(mut sin) = child.stdin.take() {
                let _ = sin.write_all(cmd.as_bytes());
            }
            child.wait_with_output()
        })
        .map_err(|e| format!("failed to run decomp_dbg: {e}"))?;

    let out = String::from_utf8_lossy(&output.stdout);
    if out.contains("Unknown settable option") {
        return Err(format!("unknown option: {:?}", option.unwrap_or("")));
    }
    let span = extract_json_span(&out).ok_or_else(|| {
        format!(
            "could not parse `stage catalog` output; decompiler said:\n{}",
            out.trim().chars().take(2000).collect::<String>()
        )
    })?;
    parse(span).ok_or_else(|| "could not parse `stage catalog` JSON".to_string())
}

/// Normalize to a `Vec` of object entries (the catalog is an array; a single
/// option comes back as one object).
fn entries(v: &Json) -> Vec<&Json> {
    match v {
        Json::Array(items) => items.iter().collect(),
        obj @ Json::Object(_) => vec![obj],
        _ => Vec::new(),
    }
}

/// Look up a string field on an object entry.
fn field<'a>(entry: &'a Json, key: &str) -> Option<&'a str> {
    if let Json::Object(pairs) = entry {
        for (k, v) in pairs {
            if k == key {
                if let Json::Str(s) = v {
                    return Some(s);
                }
            }
        }
    }
    None
}

fn bool_field(entry: &Json, key: &str) -> bool {
    if let Json::Object(pairs) = entry {
        for (k, v) in pairs {
            if k == key {
                return matches!(v, Json::Bool(true));
            }
        }
    }
    false
}

/// String array field (e.g. `values`).
fn arr_field(entry: &Json, key: &str) -> Vec<String> {
    if let Json::Object(pairs) = entry {
        for (k, v) in pairs {
            if k == key {
                if let Json::Array(items) = v {
                    return items
                        .iter()
                        .filter_map(|i| match i {
                            Json::Str(s) => Some(s.clone()),
                            _ => None,
                        })
                        .collect();
                }
            }
        }
    }
    Vec::new()
}

/// `--json`: byte-identical to `json.dumps(entries, indent=2)`.
pub fn cmd_json(option: Option<&str>) -> i32 {
    match run_catalog(option) {
        Ok(v) => {
            // Python always re-dumps the *list* form (catalog() returns a list,
            // wrapping a single dict in `[data]`).
            let list = match &v {
                Json::Array(_) => v.clone(),
                obj => Json::Array(vec![obj.clone()]),
            };
            println!("{}", dumps_indent2(&list));
            0
        }
        Err(e) => {
            eprintln!("error: {e}");
            1
        }
    }
}

/// `--markdown`: the body of `docs/assertions.md` (port of `to_markdown`).
pub fn cmd_markdown(option: Option<&str>) -> i32 {
    let v = match run_catalog(option) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("error: {e}");
            return 1;
        }
    };
    let mut lines: Vec<String> = Vec::new();
    lines.push("# kuna assertion catalog".into());
    lines.push(String::new());
    lines.push(
        "**Generated** from the decompiler's `stage catalog` command \
         (`python -m kuna.catalog --markdown`) -- do not edit by hand; edit \
         `settableTable` in `decompiler/cpp/kuna_stages.cc` and regenerate."
            .into(),
    );
    lines.push(String::new());
    lines.push(
        "These are the kuna stage-model sub-stage decisions an operator (human or LLM) \
         can flip per decompilation. Defaults are the shipped values (post-DIV-2, see \
         `docs/divergences.md`). Set any of them with \
         `python -m kuna.decompile <bin> <fn> --option <name> <value>` (or, per \
         function, `--kassert \"<stage> <substage> ...\"`); revert any one with its \
         `off`/`canonical` value."
            .into(),
    );
    lines.push(String::new());
    lines.push(
        "| Option | Values | Default | Stage / sub-stage | Source | Kind | Decision | When to flip |"
            .into(),
    );
    lines.push("|---|---|---|---|---|---|---|---|".into());
    for e in entries(&v) {
        let vals = arr_field(e, "values").join(" \\| ");
        let mut default = field(e, "default").unwrap_or("").to_string();
        if bool_field(e, "destructive_as_default") {
            default.push_str(" \u{26a0}\u{fe0f} opt-in");
        }
        let stage = format!(
            "{} / {}",
            field(e, "stage").unwrap_or(""),
            field(e, "substage").unwrap_or("")
        );
        lines.push(format!(
            "| `{}` | {} | `{}` | {} | {} | {} | {} | {} |",
            field(e, "option").unwrap_or(""),
            vals,
            default,
            stage,
            field(e, "source_decompiler").unwrap_or(""),
            field(e, "change_kind").unwrap_or(""),
            field(e, "summary").unwrap_or(""),
            field(e, "use_when").unwrap_or(""),
        ));
    }
    lines.push(String::new());
    lines.push("## Programmatic use".into());
    lines.push(String::new());
    lines.push("```bash".into());
    lines.push("# discover (machine-readable):".into());
    lines.push("python -m kuna.catalog --json".into());
    lines.push(String::new());
    lines.push("# decompile with an assertion flipped (repeatable):".into());
    lines.push("python -m kuna.decompile ./a.out main --option compareform canonical".into());
    lines.push("python -m kuna.decompile ./sparc.elf main --option returnpair single".into());
    lines.push("```".into());
    lines.push(String::new());
    lines.push(
        "The `\u{26a0}\u{fe0f} opt-in` defaults (`returnpair`, `v850indirectbranch`) are \
         documented as destructive global defaults and ship off; apply them per \
         function / per program only (see each row's *When to flip*)."
            .into(),
    );
    lines.push(String::new());
    println!("{}", lines.join("\n"));
    0
}

/// (default) the human-readable table (port of catalog.main's text branch).
pub fn cmd_text(option: Option<&str>) -> i32 {
    let v = match run_catalog(option) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("error: {e}");
            return 1;
        }
    };
    for e in entries(&v) {
        let flag = if bool_field(e, "destructive_as_default") {
            "  (opt-in: destructive as global default)"
        } else {
            ""
        };
        let values = format!("{{{}}}", arr_field(e, "values").join("|"));
        println!(
            "{:<20} {:<20} default={:<10} [{}/{}]{}",
            field(e, "option").unwrap_or(""),
            values,
            field(e, "default").unwrap_or(""),
            field(e, "stage").unwrap_or(""),
            field(e, "substage").unwrap_or(""),
            flag,
        );
        println!("    {}", field(e, "summary").unwrap_or(""));
        println!("    when: {}", field(e, "use_when").unwrap_or(""));
        let src = field(e, "source_decompiler").unwrap_or("");
        let kind = field(e, "change_kind").unwrap_or("");
        if !src.is_empty() || !kind.is_empty() {
            println!(
                "    from: {} ({})",
                if src.is_empty() { "?" } else { src },
                if kind.is_empty() { "?" } else { kind }
            );
        }
        let insp = field(e, "inspiration").unwrap_or("");
        if !insp.is_empty() {
            println!("    inspiration: {insp}");
        }
    }
    0
}

/// `--check`: verify the catalog documents exactly the registered kuna options.
///
/// The Rust-only replacement for `catalog.check_drift`: compare the option names
/// the binary's `stage catalog` emits against the in-process registered set
/// (`KUNA_OPTION_NAMES`).  A registered option with no catalog row is
/// undiscoverable; a catalog row matching no registered option is stale.  The
/// build-time codegen (`stages.toml` -> `SETTABLE_TABLE`) already fences the
/// catalog/table consistency at compile time; this end-to-end check fences the
/// catalog (what the binary emits) against the registration list.
pub fn cmd_check() -> i32 {
    let v = match run_catalog(None) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("error: {e}");
            return 1;
        }
    };
    let cat: BTreeSet<String> = entries(&v)
        .iter()
        .filter_map(|e| field(e, "option").map(|s| s.to_string()))
        .collect();
    let registered: BTreeSet<String> = KUNA_OPTION_NAMES.iter().map(|s| s.to_string()).collect();

    let mut problems: Vec<String> = Vec::new();
    for name in registered.difference(&cat) {
        problems.push(format!(
            "registered kuna option {name:?} has no catalog row (undiscoverable)"
        ));
    }
    for name in cat.difference(&registered) {
        problems.push(format!(
            "catalog row {name:?} matches no registered kuna option (stale)"
        ));
    }

    if !problems.is_empty() {
        eprintln!("catalog DRIFT:");
        for p in &problems {
            eprintln!("  - {p}");
        }
        return 1;
    }
    println!("catalog OK: documents exactly the registered kuna options");
    0
}
