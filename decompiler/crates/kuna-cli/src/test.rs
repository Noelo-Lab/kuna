//! `kuna test` — the Rust port of `kuna/run_tests.py`.
//!
//! Wraps the `decomp_test_dbg` harness: unit-test results arrive on **stderr**,
//! datatest results on **stdout**; the two streams are parsed separately.  Does
//! the baseline passing-set diff (scoped to the modes run) and computes the exit
//! code exactly as the Python (number of failures, clamped at 255; forced
//! non-zero on a baseline regression or a file-level datatest error;
//! signal-killed harness -> 128+signal).

use std::collections::BTreeSet;
use std::fmt::Write as _;
use std::io::Read;
use std::path::PathBuf;
use std::process::Command;

use crate::jsonfmt::{dumps_indent2_sorted, Json};
use crate::paths;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    All,
    Unittests,
    Datatests,
}

pub struct TestArgs {
    pub mode: Mode,
    pub names: Vec<String>,
    pub binary: Option<String>,
    pub sleighpath: Option<String>,
    pub datatests_dir: Option<String>,
    pub baseline: Option<String>,
    pub save_baseline: Option<String>,
    pub json: bool,
}

/// Parsed unit-test stream (stderr).
struct UnitResult {
    passing: Vec<String>,
    failing: Vec<(String, String)>,
    footer: Option<(i64, i64)>,
}

/// Parsed datatest stream (stdout).
struct DataResult {
    passing: Vec<String>,
    failing: Vec<String>,
    errors: Vec<String>,
    footer: Option<(i64, i64)>,
}

struct RunResult {
    returncode: i32,
    unit: UnitResult,
    data: DataResult,
}

// --- parsing -----------------------------------------------------------------
//
// Hand parsers mirroring the run_tests.py regexes (no regex dep needed; the
// patterns are simple line shapes).

fn trim_trailing_ws(s: &str) -> &str {
    s.trim_end_matches([' ', '\t'])
}

fn parse_unit(stderr: &str) -> UnitResult {
    let mut passing = Vec::new();
    let mut failing = Vec::new();
    let mut footer = None;
    let mut current: Option<String> = None;

    for raw in stderr.lines() {
        let line = trim_trailing_ws(raw);
        // ^testing : (?P<name>.+) \.\.\.$
        if let Some(rest) = line.strip_prefix("testing : ") {
            if let Some(name) = rest.strip_suffix(" ...") {
                if !name.is_empty() {
                    current = Some(name.to_string());
                    continue;
                }
            }
        }
        if let Some(cur) = current.clone() {
            // ^  passed\.$
            if line == "  passed." {
                passing.push(cur);
                current = None;
                continue;
            }
            // ^  fail(?:: (?P<msg>.*))?$
            if line == "  fail" {
                failing.push((cur, String::new()));
                current = None;
                continue;
            }
            if let Some(msg) = line.strip_prefix("  fail: ") {
                failing.push((cur, msg.to_string()));
                current = None;
                continue;
            }
        }
        // ^(?P<passed>\d+)/(?P<total>\d+) tests passed\.$
        if let Some(f) = parse_footer(line, " tests passed.") {
            footer = Some(f);
        }
    }
    UnitResult { passing, failing, footer }
}

/// Parse `"<a>/<b> <suffix>"` -> `(a, b)`.
fn parse_footer(line: &str, suffix: &str) -> Option<(i64, i64)> {
    let body = line.strip_suffix(suffix)?;
    let (a, b) = body.split_once('/')?;
    let a: i64 = a.trim().parse().ok()?;
    let b: i64 = b.trim().parse().ok()?;
    Some((a, b))
}

fn parse_data(stdout: &str) -> DataResult {
    let mut passing = Vec::new();
    let mut failing = Vec::new();
    let mut errors = Vec::new();
    let mut applied: Option<i64> = None;
    let mut passing_total: Option<i64> = None;

    for raw in stdout.lines() {
        let line = trim_trailing_ws(raw);
        // ^Success -- (?P<name>.+?)$  (.+? non-greedy with trailing-ws already
        // trimmed == the whole remainder).
        if let Some(name) = line.strip_prefix("Success -- ") {
            passing.push(name.to_string());
            continue;
        }
        if let Some(name) = line.strip_prefix("FAIL -- ") {
            failing.push(name.to_string());
            continue;
        }
        if let Some(f) = parse_data_error(line) {
            errors.push(f);
            continue;
        }
        // ^Total tests applied = (?P<n>\d+)$
        if let Some(n) = line.strip_prefix("Total tests applied = ") {
            if let Ok(v) = n.trim().parse() {
                applied = Some(v);
            }
            continue;
        }
        if let Some(n) = line.strip_prefix("Total passing tests = ") {
            if let Ok(v) = n.trim().parse() {
                passing_total = Some(v);
            }
            continue;
        }
    }
    let footer = applied.map(|a| (a, passing_total.unwrap_or(0)));
    // Errors can be reported twice (column 0 + the indented Failures summary);
    // dedup preserving first-seen order (dict.fromkeys).
    let mut seen = BTreeSet::new();
    let errors: Vec<String> = errors
        .into_iter()
        .filter(|e| seen.insert(e.clone()))
        .collect();
    DataResult { passing, failing, errors, footer }
}

/// Match a file-level datatest error line, returning the named `*.xml` file.
/// Port of the `_DATA_ERR` regex (leading whitespace allowed for the indented
/// "Failures:" summary forms; every form names a `*.xml` file).
fn parse_data_error(line: &str) -> Option<String> {
    let t = line.trim_start();
    // "Error parsing <f>.xml: ..." | "Error executing <f>.xml: ..."
    for kw in ["Error parsing ", "Error executing "] {
        if let Some(rest) = t.strip_prefix(kw) {
            if let Some(idx) = rest.find(".xml: ") {
                return Some(rest[..idx + 4].to_string());
            }
        }
    }
    // "Error: Did not apply tests in <f>.xml"
    if let Some(rest) = t.strip_prefix("Error: Did not apply tests in ") {
        if let Some(f) = rest.strip_suffix(".xml") {
            return Some(format!("{f}.xml"));
        }
        // also accept exact ".xml" boundary (no suffix trailing)
        if rest.ends_with(".xml") {
            return Some(rest.to_string());
        }
    }
    // "No output for <f>.xml"
    if let Some(rest) = t.strip_prefix("No output for ") {
        if rest.ends_with(".xml") {
            return Some(rest.to_string());
        }
    }
    // "Execution failed for <f>.xml"
    if let Some(rest) = t.strip_prefix("Execution failed for ") {
        if rest.ends_with(".xml") {
            return Some(rest.to_string());
        }
    }
    None
}

// --- running -----------------------------------------------------------------

/// `os.path.abspath`: join a relative path onto the current working directory
/// and lexically normalize `.`/`..` components (does NOT resolve symlinks, so it
/// matches Python's abspath rather than `fs::canonicalize`).
fn abspath(p: &str) -> PathBuf {
    use std::path::Component;
    let raw = PathBuf::from(p);
    let joined = if raw.is_absolute() {
        raw
    } else {
        std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")).join(raw)
    };
    let mut out = PathBuf::new();
    for comp in joined.components() {
        match comp {
            Component::CurDir => {}
            Component::ParentDir => {
                out.pop();
            }
            other => out.push(other.as_os_str()),
        }
    }
    out
}

fn run(args: &TestArgs) -> Result<RunResult, String> {
    if !args.names.is_empty() && args.mode == Mode::All {
        return Err("names= requires mode='unittests' or mode='datatests'".into());
    }
    let bin_path = match &args.binary {
        Some(b) => PathBuf::from(b),
        None => paths::decomp_test_dbg(),
    };
    if !bin_path.exists() {
        return Err(format!(
            "decomp_test_dbg not built at {} -- run `make binaries`",
            bin_path.display()
        ));
    }
    // Python passes os.path.abspath(...) for -sleighpath / -path; the harness
    // echoes the -path dir verbatim in its file-level error messages, so the
    // absolutized form must match byte-for-byte (the relative form would differ).
    let specs = match &args.sleighpath {
        Some(s) => abspath(s),
        None => paths::specs_dir(),
    };
    let dtdir = match &args.datatests_dir {
        Some(d) => abspath(d),
        None => paths::datatests_dir(),
    };

    let mut argv: Vec<String> = vec![
        "-sleighpath".into(),
        specs.to_string_lossy().to_string(),
        "-path".into(),
        dtdir.to_string_lossy().to_string(),
    ];

    match args.mode {
        Mode::Unittests => {
            argv.push("unittests".into());
            argv.extend(args.names.iter().cloned());
        }
        Mode::Datatests => {
            argv.push("datatests".into());
            for n in &args.names {
                let n2 = if n.ends_with(".xml") {
                    n.clone()
                } else {
                    format!("{n}.xml")
                };
                if !dtdir.join(&n2).exists() {
                    // Python `%r` on a str renders single-quoted; match it byte-for-byte.
                    eprintln!(
                        "warning: datatest '{n2}' not found in {} (will be skipped)",
                        dtdir.display()
                    );
                }
                argv.push(n2);
            }
        }
        Mode::All => {}
    }

    let output = Command::new(&bin_path)
        .args(&argv)
        .output()
        .map_err(|e| format!("failed to run decomp_test_dbg: {e}"))?;

    let returncode = exit_code_of(&output);
    let unit = parse_unit(&String::from_utf8_lossy(&output.stderr));
    let data = parse_data(&String::from_utf8_lossy(&output.stdout));
    Ok(RunResult { returncode, unit, data })
}

/// The process return code, with a signal-killed process reported as a negative
/// value (mirroring Python `subprocess`'s `returncode = -signal`), so the
/// downstream `128 - rc` shell mapping reproduces.
fn exit_code_of(output: &std::process::Output) -> i32 {
    #[cfg(unix)]
    {
        use std::os::unix::process::ExitStatusExt;
        if let Some(sig) = output.status.signal() {
            return -sig;
        }
    }
    output.status.code().unwrap_or(0)
}

// --- baseline keys -----------------------------------------------------------

/// Prefix names into keys, disambiguating repeats by emission order (port of
/// `_dedup_keys`): the first occurrence is `"<prefix><name>"`, the Nth is
/// `"<prefix><name> @dupN"`.
fn dedup_keys(prefix: &str, names: &[String]) -> BTreeSet<String> {
    use std::collections::BTreeMap;
    let mut seen: BTreeMap<String, usize> = BTreeMap::new();
    let mut keys = BTreeSet::new();
    for n in names {
        let count = seen.entry(n.clone()).or_insert(0);
        *count += 1;
        if *count == 1 {
            keys.insert(format!("{prefix}{n}"));
        } else {
            keys.insert(format!("{prefix}{n} @dup{}", *count));
        }
    }
    keys
}

fn pass_keyset(r: &RunResult) -> BTreeSet<String> {
    let mut s = dedup_keys("unit:", &r.unit.passing);
    s.extend(dedup_keys("data:", &r.data.passing));
    s
}

// --- text summary ------------------------------------------------------------

fn summary(r: &RunResult) -> String {
    let mut lines: Vec<String> = Vec::new();
    if let Some((p, t)) = r.unit.footer {
        lines.push(format!("unittests: {p}/{t} passed"));
    } else if !r.unit.passing.is_empty() || !r.unit.failing.is_empty() {
        lines.push(format!(
            "unittests: {} passed, {} failed",
            r.unit.passing.len(),
            r.unit.failing.len()
        ));
    }
    if let Some((applied, passing)) = r.data.footer {
        lines.push(format!("datatests: {passing}/{applied} assertions passed"));
    } else if !r.data.passing.is_empty() || !r.data.failing.is_empty() {
        lines.push(format!(
            "datatests: {} passed, {} failed",
            r.data.passing.len(),
            r.data.failing.len()
        ));
    }
    let mut fails: Vec<String> = Vec::new();
    for (n, m) in &r.unit.failing {
        fails.push(format!("[unit] {n}: {m}"));
    }
    for n in &r.data.failing {
        fails.push(format!("[data] {n}"));
    }
    for f in &r.data.errors {
        fails.push(format!("[data] ERROR in {f}"));
    }
    if !fails.is_empty() {
        lines.push("FAILURES:".into());
        for f in &fails {
            lines.push(format!("  {f}"));
        }
    }
    lines.push(format!("exit: {}", r.returncode));
    lines.join("\n")
}

// --- entry point -------------------------------------------------------------

struct Parity {
    regressed: Vec<String>,
    newly_passing: Vec<String>,
    verdict: &'static str,
}

pub fn run_cmd(args: &TestArgs) -> i32 {
    let result = match run(args) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("error: {e}");
            return 2;
        }
    };

    let mut passkeys: Vec<String> = pass_keyset(&result).into_iter().collect();
    passkeys.sort();

    // The baseline record (also the --save-baseline content).
    let record_json = |passkeys: &[String]| -> Json {
        Json::Object(vec![
            ("unit_footer".into(), footer_json(result.unit.footer)),
            ("data_footer".into(), footer_json(result.data.footer)),
            (
                "passing".into(),
                Json::Array(passkeys.iter().map(|k| Json::Str(k.clone())).collect()),
            ),
            ("returncode".into(), Json::Number(result.returncode.to_string())),
        ])
    };

    if let Some(path) = &args.save_baseline {
        let body = dumps_indent2_sorted(&record_json(&passkeys));
        match std::fs::write(path, body) {
            Ok(()) => eprintln!(
                "baseline written to {} ({} passing keys)",
                path,
                passkeys.len()
            ),
            Err(e) => {
                eprintln!("error: could not write baseline {path}: {e}");
                return 2;
            }
        }
    }

    let mut parity_fail = false;
    let mut parity: Option<Parity> = None;
    if let Some(bl) = &args.baseline {
        let mut content = String::new();
        match std::fs::File::open(bl).and_then(|mut f| f.read_to_string(&mut content)) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("error: could not read baseline {bl}: {e}");
                return 2;
            }
        }
        let base = match crate::jsonfmt::parse(&content) {
            Some(v) => v,
            None => {
                eprintln!("error: could not parse baseline JSON {bl}");
                return 2;
            }
        };
        let base_pass = baseline_passing(&base);
        // Scope the baseline to the modes actually run.
        let run_prefixes: &[&str] = match args.mode {
            Mode::All => &["unit:", "data:"],
            Mode::Unittests => &["unit:"],
            Mode::Datatests => &["data:"],
        };
        let base_pass: BTreeSet<String> = base_pass
            .into_iter()
            .filter(|k| run_prefixes.iter().any(|p| k.starts_with(p)))
            .collect();
        let now_pass: BTreeSet<String> = passkeys.iter().cloned().collect();
        let mut regressed: Vec<String> = base_pass.difference(&now_pass).cloned().collect();
        regressed.sort();
        let mut newly: Vec<String> = now_pass.difference(&base_pass).cloned().collect();
        newly.sort();
        parity_fail = !regressed.is_empty() || result.returncode != 0;
        parity = Some(Parity {
            regressed,
            newly_passing: newly,
            verdict: if parity_fail { "PARITY FAIL" } else { "PARITY OK" },
        });
    }

    let mut text = String::new();
    if args.json {
        let mut obj: Vec<(String, Json)> = match record_json(&passkeys) {
            Json::Object(p) => p,
            _ => unreachable!(),
        };
        // failures sub-object
        let unit_fail = Json::Array(
            result
                .unit
                .failing
                .iter()
                .map(|(n, m)| Json::Array(vec![Json::Str(n.clone()), Json::Str(m.clone())]))
                .collect(),
        );
        let data_fail = Json::Array(
            result.data.failing.iter().map(|n| Json::Str(n.clone())).collect(),
        );
        let data_errors = Json::Array(
            result.data.errors.iter().map(|e| Json::Str(e.clone())).collect(),
        );
        obj.push((
            "failures".into(),
            Json::Object(vec![
                ("unit".into(), unit_fail),
                ("data".into(), data_fail),
                ("data_errors".into(), data_errors),
            ]),
        ));
        if let Some(p) = &parity {
            obj.push((
                "parity".into(),
                Json::Object(vec![
                    (
                        "regressed".into(),
                        Json::Array(p.regressed.iter().map(|k| Json::Str(k.clone())).collect()),
                    ),
                    (
                        "newly_passing".into(),
                        Json::Array(
                            p.newly_passing.iter().map(|k| Json::Str(k.clone())).collect(),
                        ),
                    ),
                    ("verdict".into(), Json::Str(p.verdict.to_string())),
                ]),
            ));
        }
        let _ = writeln!(text, "{}", dumps_indent2_sorted(&Json::Object(obj)));
    } else {
        let _ = writeln!(text, "{}", summary(&result));
        if let Some(p) = &parity {
            let _ = writeln!(text, "\n=== baseline parity ===");
            if !p.regressed.is_empty() {
                let _ = writeln!(text, "REGRESSED ({}):", p.regressed.len());
                for k in &p.regressed {
                    let _ = writeln!(text, "  {k}");
                }
            }
            if !p.newly_passing.is_empty() {
                let shown: Vec<String> = p.newly_passing.iter().take(10).cloned().collect();
                let suffix = if p.newly_passing.len() > 10 { " ..." } else { "" };
                let _ = writeln!(
                    text,
                    "newly passing ({}): {}{}",
                    p.newly_passing.len(),
                    shown.join(", "),
                    suffix
                );
            }
            let _ = writeln!(text, "{}", p.verdict);
        }
    }

    // --- exit code (port of run_tests.main's tail) ---------------------------
    let rc = if parity_fail {
        1
    } else if result.returncode < 0 {
        std::cmp::min(128 - result.returncode, 255)
    } else if result.returncode == 0
        && (!result.data.errors.is_empty()
            || !result.unit.failing.is_empty()
            || !result.data.failing.is_empty())
    {
        1
    } else {
        std::cmp::min(result.returncode, 255)
    };
    crate::output::emit_with_status(&text, rc)
}

fn footer_json(f: Option<(i64, i64)>) -> Json {
    match f {
        Some((a, b)) => Json::Array(vec![
            Json::Number(a.to_string()),
            Json::Number(b.to_string()),
        ]),
        None => Json::Null,
    }
}

/// Read the `passing` string array out of a baseline JSON value.
fn baseline_passing(base: &Json) -> BTreeSet<String> {
    if let Json::Object(pairs) = base {
        for (k, v) in pairs {
            if k == "passing" {
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
    BTreeSet::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_stream_parses_pass_fail_footer() {
        let stderr = "\
testing : Foo ...
  passed.
testing : Bar ...
  fail: boom
3/4 tests passed.
";
        let u = parse_unit(stderr);
        assert_eq!(u.passing, vec!["Foo".to_string()]);
        assert_eq!(u.failing, vec![("Bar".to_string(), "boom".to_string())]);
        assert_eq!(u.footer, Some((3, 4)));
    }

    #[test]
    fn data_stream_parses_success_fail_footer() {
        let stdout = "\
Success -- Test #1
FAIL -- Test #2
Total tests applied = 2
Total passing tests = 1
";
        let d = parse_data(stdout);
        assert_eq!(d.passing, vec!["Test #1".to_string()]);
        assert_eq!(d.failing, vec!["Test #2".to_string()]);
        assert_eq!(d.footer, Some((2, 1)));
    }

    #[test]
    fn data_error_forms_named_xml_files() {
        assert_eq!(
            parse_data_error("Error parsing foo.xml: bad token"),
            Some("foo.xml".to_string())
        );
        assert_eq!(
            parse_data_error("Error executing bar.xml: trap"),
            Some("bar.xml".to_string())
        );
        assert_eq!(
            parse_data_error("Error: Did not apply tests in baz.xml"),
            Some("baz.xml".to_string())
        );
        assert_eq!(
            parse_data_error("  No output for qux.xml"),
            Some("qux.xml".to_string())
        );
        assert_eq!(
            parse_data_error("  Execution failed for quux.xml"),
            Some("quux.xml".to_string())
        );
        // A non-error line is not misclassified.
        assert_eq!(parse_data_error("Success -- a thing"), None);
    }

    #[test]
    fn dedup_keys_disambiguates_repeats_in_order() {
        let names = vec!["A".to_string(), "A".to_string(), "B".to_string()];
        let keys = dedup_keys("data:", &names);
        assert!(keys.contains("data:A"));
        assert!(keys.contains("data:A @dup2"));
        assert!(keys.contains("data:B"));
        assert_eq!(keys.len(), 3);
    }

    #[test]
    fn baseline_json_sorted_record_shape() {
        // The record/baseline JSON is sorted-key indent-2 (matches run_tests.py).
        let rec = Json::Object(vec![
            ("returncode".into(), Json::Number("0".into())),
            (
                "passing".into(),
                Json::Array(vec![Json::Str("data:x".into())]),
            ),
            (
                "data_footer".into(),
                Json::Array(vec![Json::Number("1".into()), Json::Number("1".into())]),
            ),
            ("unit_footer".into(), Json::Null),
        ]);
        let s = dumps_indent2_sorted(&rec);
        // sort_keys => data_footer, passing, returncode, unit_footer.
        let want = "{\n  \"data_footer\": [\n    1,\n    1\n  ],\n  \"passing\": [\n    \"data:x\"\n  ],\n  \"returncode\": 0,\n  \"unit_footer\": null\n}";
        assert_eq!(s, want);
    }
}
