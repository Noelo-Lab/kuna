//! `kuna catalog` — the option-catalog surface (discovery half of the LLM control API).
//!
//! Drives `decomp_dbg phase catalog [<option>]`, extracts the embedded JSON from
//! the console transcript, and re-emits it.  Modes:
//!   * (default) a human-readable table,
//!   * `--json`     the catalog as `json.dumps(indent=2)` (byte-identical to the
//!                  Python `--json`),
//!   * `--markdown` the `docs/options.md` body (tier-grouped, symptom-indexed),
//!   * `--check`    verify the catalog documents exactly the registered kuna
//!                  options.  In the Rust-only world the old `check_drift` (which
//!                  parsed the removed `decompiler/cpp/options.cc` + `kuna_*.hh`)
//!                  can no longer run; the faithful replacement cross-checks the
//!                  catalog the binary emits against the in-process registered
//!                  option set ([`kuna_decomp::options::KUNA_OPTION_NAMES`]),
//!                  catching the same drift (a registered option with no catalog
//!                  row, or a stale catalog row).

use std::collections::BTreeSet;
use std::fmt::Write as _;
use std::process::Command;

use kuna_decomp::kuna_phases::emit_catalog_markdown;
use kuna_decomp::options::KUNA_OPTION_NAMES;

use crate::jsonfmt::{dumps_indent2, extract_json_span, parse, Json};
use crate::paths;

/// Run `decomp_dbg phase catalog [<option>]` and return the parsed JSON value
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
        None => "phase catalog\nquit\n".to_string(),
        Some(o) => format!("phase catalog {o}\nquit\n"),
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
            "could not parse `phase catalog` output; decompiler said:\n{}",
            out.trim().chars().take(2000).collect::<String>()
        )
    })?;
    parse(span).ok_or_else(|| "could not parse `phase catalog` JSON".to_string())
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
            crate::output::emit_with_status(&format!("{}\n", dumps_indent2(&list)), 0)
        }
        Err(e) => {
            eprintln!("error: {e}");
            1
        }
    }
}

/// `--markdown`: the body of `docs/options.md`, rendered in-process straight
/// from `SETTABLE_TABLE` (`kuna_phases::emit_catalog_markdown`) -- no live
/// values, byte-stable, freshness-fenced by kuna-decomp's
/// `tests/options_md_fresh.rs`.
pub fn cmd_markdown(option: Option<&str>) -> i32 {
    if let Some(o) = option {
        eprintln!("error: --markdown renders the whole catalog (got --option {o}); use --json/--option for one row");
        return 2;
    }
    crate::output::emit_with_status(&emit_catalog_markdown(), 0)
}

/// (default) the human-readable table (port of catalog.main's text branch).
/// `tier` filters to one of transform|analysis|core.
pub fn cmd_text(option: Option<&str>, tier: Option<&str>) -> i32 {
    let v = match run_catalog(option) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("error: {e}");
            return 1;
        }
    };
    let mut text = String::new();
    for e in entries(&v) {
        if let Some(want) = tier {
            if field(e, "tier") != Some(want) {
                continue;
            }
        }
        let flag = if bool_field(e, "destructive_as_default") {
            "  (opt-in: destructive as global default)"
        } else {
            ""
        };
        let values = format!("{{{}}}", arr_field(e, "values").join("|"));
        let _ = writeln!(
            text,
            "{:<20} {:<20} default={:<10} [{}/{}] tier={}{}",
            field(e, "option").unwrap_or(""),
            values,
            field(e, "default").unwrap_or(""),
            field(e, "phase").unwrap_or(""),
            field(e, "subphase").unwrap_or(""),
            field(e, "tier").unwrap_or("?"),
            flag,
        );
        let _ = writeln!(text, "    {}", field(e, "summary").unwrap_or(""));
        let _ = writeln!(text, "    when: {}", field(e, "use_when").unwrap_or(""));
        let src = field(e, "source_decompiler").unwrap_or("");
        let kind = field(e, "change_kind").unwrap_or("");
        if !src.is_empty() || !kind.is_empty() {
            let _ = writeln!(
                text,
                "    from: {} ({})",
                if src.is_empty() { "?" } else { src },
                if kind.is_empty() { "?" } else { kind }
            );
        }
        let insp = field(e, "inspiration").unwrap_or("");
        if !insp.is_empty() {
            let _ = writeln!(text, "    inspiration: {insp}");
        }
    }
    crate::output::emit_with_status(&text, 0)
}

/// `--check`: verify the catalog documents exactly the registered kuna options.
///
/// The Rust-only replacement for `catalog.check_drift`: compare the option names
/// the binary's `phase catalog` emits against the in-process registered set
/// (`KUNA_OPTION_NAMES`).  A registered option with no catalog row is
/// undiscoverable; a catalog row matching no registered option is stale.  The
/// build-time codegen (`phases.toml` -> `SETTABLE_TABLE`) already fences the
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
    crate::output::emit_with_status(
        "catalog OK: documents exactly the registered kuna options\n",
        0,
    )
}
