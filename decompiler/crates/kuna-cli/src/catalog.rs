//! Discover, render and validate the registered option catalog.
//!
//! The linked engine supplies static JSON by default; an explicit console
//! override keeps external queries. This module renders a table, indented JSON
//! or the generated Markdown manual. Validation compares the catalog with
//! [`kuna_decomp::options::KUNA_OPTION_NAMES`] to reject missing or stale rows.
//! `--engine jvm|all` adds the JVM engine's rows (`docs/options-jvm.md`); the
//! default query is the native catalog alone.

use std::collections::BTreeSet;
use std::fmt::Write as _;

use kuna_console::kuna_buildstamp;
use kuna_decomp::kuna_phases::{emit_catalog_json, emit_catalog_json_one, emit_catalog_markdown};
use kuna_decomp::options::KUNA_OPTION_NAMES;

use crate::jsonfmt::{dumps_indent2, extract_json_span, parse, Json};
use crate::paths;

/// Read the local static catalog unless the caller selected an external console.
fn run_catalog(option: Option<&str>, pinned_by: Option<&str>) -> Result<Json, String> {
    if pinned_by.is_some() {
        return run_external_catalog(option, pinned_by);
    }
    let out = match option {
        Some(option) => emit_catalog_json_one(option, None)
            .ok_or_else(|| format!("unknown option: {option:?}"))?,
        None => emit_catalog_json(|_| None),
    };
    parse(&out).ok_or_else(|| "could not parse `phase catalog` JSON".to_string())
}

/// Run `decomp_dbg phase catalog [<option>]` for an explicit console override.
fn run_external_catalog(option: Option<&str>, pinned_by: Option<&str>) -> Result<Json, String> {
    let engine = paths::decomp_dbg()?;
    let specs = paths::specs_dir();
    let cmd = match option {
        None => "phase catalog\nquit\n".to_string(),
        Some(o) => format!("phase catalog {o}\nquit\n"),
    };

    let output = engine
        .command()
        .arg("-s")
        .arg(&specs)
        .env("SLEIGHHOME", &specs)
        .env(
            kuna_buildstamp::PARENT_ENV,
            kuna_buildstamp::identity(kuna_buildstamp::ChildKind::Engine),
        )
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
    kuna_buildstamp::report(
        kuna_buildstamp::ChildKind::Engine,
        String::from_utf8_lossy(&output.stderr).into_owned(),
        "decomp_dbg",
        &engine.path,
        pinned_by,
    );

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

/// Which engine's rows a catalog query covers. `Default` (no `--engine`, or
/// the legacy `cpp|rust`) is the native catalog exactly as it always was; a
/// JVM option name given to `--option` still finds its row there. The other
/// three tag every row with `engine` (and `aliases`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Engine {
    Default,
    Native,
    Jvm,
    All,
}

impl Engine {
    pub fn parse(value: Option<&str>) -> Result<Engine, String> {
        match value {
            None | Some("cpp") | Some("rust") => Ok(Engine::Default),
            Some("native") => Ok(Engine::Native),
            Some("jvm") => Ok(Engine::Jvm),
            Some("all") => Ok(Engine::All),
            Some(other) => Err(format!("unknown --engine {other:?} (expected native, jvm or all)")),
        }
    }

    fn native(self) -> bool {
        self != Engine::Jvm
    }
}

#[cfg(feature = "jvm")]
fn jvm_rows(option: Option<&str>) -> Result<Vec<Json>, String> {
    Ok(crate::nonnative::catalog::rows(option))
}

#[cfg(not(feature = "jvm"))]
fn jvm_rows(_option: Option<&str>) -> Result<Vec<Json>, String> {
    Err("this kuna was built without the JVM engine (cargo feature `jvm`)".into())
}

#[cfg(feature = "jvm")]
fn jvm_knows(name: &str) -> bool {
    crate::nonnative::catalog::knows(name)
}

#[cfg(not(feature = "jvm"))]
fn jvm_knows(_name: &str) -> bool {
    false
}

/// `row` with `aliases` (when absent) and `engine` appended.
fn tagged(row: &Json, engine: &str) -> Json {
    let mut row = row.clone();
    if let Json::Object(pairs) = &mut row {
        if !pairs.iter().any(|(k, _)| k == "aliases") {
            pairs.push(("aliases".into(), Json::Array(Vec::new())));
        }
        if !pairs.iter().any(|(k, _)| k == "engine") {
            pairs.push(("engine".into(), Json::Str(engine.into())));
        }
    }
    row
}

/// The rows a query selects, in catalog order: native rows, then JVM rows.
fn selected_rows(option: Option<&str>, pinned_by: Option<&str>, engine: Engine) -> Result<Vec<Json>, String> {
    let jvm_only = match option {
        Some(name) => engine == Engine::Jvm
            || (engine != Engine::Native && emit_catalog_json_one(name, None).is_none() && jvm_knows(name)),
        None => engine == Engine::Jvm,
    };
    let mut rows = Vec::new();
    if engine.native() && !jvm_only {
        let v = run_catalog(option, pinned_by)?;
        for e in entries(&v) {
            rows.push(if engine == Engine::Default { e.clone() } else { tagged(e, "native") });
        }
    }
    if jvm_only || (engine == Engine::All && option.is_none()) {
        let found = jvm_rows(option)?;
        if found.is_empty() {
            if let Some(name) = option {
                return Err(format!("unknown option: {name:?}"));
            }
        }
        rows.extend(found);
    }
    Ok(rows)
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

/// `--json`: byte-identical to `json.dumps(entries, indent=2)`, keeping the rows
/// of `tier` only when one is given.
pub fn cmd_json(option: Option<&str>, tier: Option<&str>, pinned_by: Option<&str>, engine: Engine) -> i32 {
    match selected_rows(option, pinned_by, engine) {
        Ok(rows) => {
            // Python always re-dumps the *list* form (catalog() returns a list,
            // wrapping a single dict in `[data]`).
            let list = Json::Array(
                rows.into_iter()
                    .filter(|e| tier.is_none_or(|want| field(e, "tier") == Some(want)))
                    .collect(),
            );
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
pub fn cmd_markdown(option: Option<&str>, engine: Engine) -> i32 {
    if let Some(o) = option {
        eprintln!("error: --markdown renders the whole catalog (got --option {o}); use --json/--option for one row");
        return 2;
    }
    match engine {
        Engine::Default | Engine::Native => crate::output::emit_with_status(&emit_catalog_markdown(), 0),
        Engine::Jvm => jvm_markdown(),
        Engine::All => {
            eprintln!("error: --markdown renders one engine's catalog: --engine native (docs/options.md) or --engine jvm (docs/options-jvm.md)");
            2
        }
    }
}

#[cfg(feature = "jvm")]
fn jvm_markdown() -> i32 {
    crate::output::emit_with_status(&crate::nonnative::catalog::markdown(), 0)
}

#[cfg(not(feature = "jvm"))]
fn jvm_markdown() -> i32 {
    eprintln!("error: this kuna was built without the JVM engine (cargo feature `jvm`)");
    1
}

/// (default) the human-readable table (port of catalog.main's text branch).
/// `tier` filters to one of transform|analysis|core.
pub fn cmd_text(option: Option<&str>, tier: Option<&str>, pinned_by: Option<&str>, engine: Engine) -> i32 {
    let rows = match selected_rows(option, pinned_by, engine) {
        Ok(rows) => rows,
        Err(e) => {
            eprintln!("error: {e}");
            return 1;
        }
    };
    let mut text = String::new();
    for e in &rows {
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
        let aliases = arr_field(e, "aliases");
        if !aliases.is_empty() {
            let _ = writeln!(text, "    aliases: {}", aliases.join(", "));
        }
    }
    crate::output::emit_with_status(&text, 0)
}

/// `--check`: verify the catalog documents exactly the registered kuna options.
///
/// Compare the selected catalog's option names with this build's registered set
/// (`KUNA_OPTION_NAMES`). A registered option with no row is undiscoverable;
/// a row matching no registered option is stale. An explicit console override
/// selects that binary's catalog; otherwise both sets come from this build.
pub fn cmd_check(pinned_by: Option<&str>) -> i32 {
    let v = match run_catalog(None, pinned_by) {
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

    let (jvm_problems, jvm_ok) = jvm_check();
    problems.extend(jvm_problems);

    if !problems.is_empty() {
        eprintln!("catalog DRIFT:");
        for p in &problems {
            eprintln!("  - {p}");
        }
        return 1;
    }
    crate::output::emit_with_status(
        &format!("catalog OK: documents exactly the registered kuna options\n{jvm_ok}"),
        0,
    )
}

#[cfg(feature = "jvm")]
fn jvm_check() -> (Vec<String>, String) {
    (
        crate::nonnative::catalog::check(),
        format!(
            "catalog OK: {} JVM engine options, each spelled jvm_<name> with a resolvable default\n",
            crate::nonnative::catalog::count()
        ),
    )
}

#[cfg(not(feature = "jvm"))]
fn jvm_check() -> (Vec<String>, String) {
    (Vec::new(), String::new())
}
