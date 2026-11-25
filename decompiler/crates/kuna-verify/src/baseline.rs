//! The corpus baseline (`docs/baseline-verify.json`): each datatest file's
//! verdict classes in script order, and how a run compares with it.
//!
//! A class leaves out what varies without the verdict changing: a `DIFFERS`
//! run, an `UNKNOWN` address. It keeps an `AGREES` run count, so a function
//! that still agrees on fewer than half the runs it agreed on is lost coverage;
//! a smaller change in the count is not reported. Rows pair by function and
//! occurrence, so a file whose function list changed is still compared. A
//! function that now differs where it did not, a new function that differs,
//! or a file that can no longer be checked is a regression; lost coverage is a
//! warning; every other change is reported so the baseline can be re-recorded.

use std::collections::BTreeMap;

use serde_json::Value;

use crate::report::Row;

/// Per file, `(function, class)` in the order the script decompiles them; a
/// file that could not be checked has the class `ERROR` alone.
pub type Baseline = BTreeMap<String, Vec<(String, String)>>;

pub const ERROR: &str = "ERROR";

/// What the baseline records of a verdict.
pub fn class(r: &Row) -> String {
    match r.class.as_str() {
        "AGREES" => r.label.clone(),
        "DIFFERS" => r.class.clone(),
        "UNKNOWN" | "SKIPPED" => format!("{})", r.reason().unwrap_or_else(|| r.class.clone())),
        _ => r.label.clone(),
    }
}

/// One line per file, sorted, so a re-recorded baseline diffs by file.
pub fn render(b: &Baseline) -> String {
    let mut s = String::from("{\n");
    for (k, (file, rows)) in b.iter().enumerate() {
        let items: Vec<String> = rows.iter().map(|(f, c)| format!("{f} {c}")).collect();
        let sep = if k + 1 == b.len() { "" } else { "," };
        s.push_str(&format!("  {}: {}{sep}\n", Value::from(file.as_str()), Value::from(items)));
    }
    s.push_str("}\n");
    s
}

/// `name class`, where a name may hold spaces (`-[A b:]`) and so may the
/// reason of an `UNKNOWN`, `SKIPPED` or `DIVERGES` class (`UNKNOWN(no switch row)`).
fn split_row(r: &str) -> Option<(&str, &str)> {
    let tagged = r.ends_with(')').then(|| [" UNKNOWN(", " SKIPPED(", " DIVERGES(", " AGREES("].iter().filter_map(|t| r.rfind(t)).max()).flatten();
    match tagged {
        Some(i) => Some((&r[..i], &r[i + 1..])),
        None => r.rsplit_once(' '),
    }
}

pub fn parse(text: &str) -> Result<Baseline, String> {
    let v: Value = serde_json::from_str(text).map_err(|e| e.to_string())?;
    let obj = v.as_object().ok_or("the baseline is not a JSON object")?;
    let mut b = Baseline::new();
    for (file, rows) in obj {
        let rows = rows.as_array().ok_or_else(|| format!("{file}: not an array"))?;
        let mut out = Vec::with_capacity(rows.len());
        for r in rows {
            let r = r.as_str().ok_or_else(|| format!("{file}: not a string: {r}"))?;
            let (f, c) = split_row(r).ok_or_else(|| format!("{file}: no class in {r:?}"))?;
            out.push((f.to_string(), c.to_string()));
        }
        b.insert(file.clone(), out);
    }
    Ok(b)
}

#[derive(Default, Debug)]
pub struct Changes {
    /// A function that now differs, a new function that differs, or a file
    /// that can no longer be checked.
    pub regressions: Vec<String>,
    /// Coverage lost without a new difference: a function that agreed and no
    /// longer does, a function or file the baseline has that was not checked.
    pub lost: Vec<String>,
    /// Every other change, a function that no longer differs among them.
    pub other: Vec<String>,
    /// Functions that agree, and functions in all, in the baseline and now.
    pub agreed: (usize, usize),
    pub functions: (usize, usize),
}

impl Changes {
    pub fn is_empty(&self) -> bool {
        self.regressions.is_empty() && self.lost.is_empty() && self.other.is_empty()
    }
}

/// Each row keyed by its function and how many times the file decompiled that
/// function before it, so a function decompiled again under other options
/// pairs with its own earlier verdict.
fn keyed(rows: &[(String, String)]) -> Vec<((String, usize), String)> {
    let mut seen: BTreeMap<&str, usize> = BTreeMap::new();
    rows.iter()
        .map(|(f, c)| {
            let n = seen.entry(f.as_str()).or_default();
            *n += 1;
            ((f.clone(), *n), c.clone())
        })
        .collect()
}

fn name((f, n): &(String, usize)) -> String {
    if *n == 1 { f.clone() } else { format!("{f} (decompile {n})") }
}

/// Whether a class is an agreement, and over how many runs when it says.
fn agrees(k: &str) -> Option<Option<u32>> {
    if k == "AGREES" {
        return Some(None);
    }
    k.strip_prefix("AGREES(").and_then(|r| r.strip_suffix(')')).map(|n| n.parse().ok())
}

pub fn compare(base: &Baseline, now: &Baseline) -> Changes {
    let mut c = Changes::default();
    let count = |b: &Baseline| {
        let rows = b.values().flatten();
        (rows.clone().filter(|(_, k)| agrees(k).is_some()).count(), rows.count())
    };
    let (was_agreed, was_all) = count(base);
    let (now_agreed, now_all) = count(now);
    c.agreed = (was_agreed, now_agreed);
    c.functions = (was_all, now_all);
    for (file, rows) in now {
        let Some(was) = base.get(file) else {
            for (f, k) in rows.iter().filter(|(_, k)| k == "DIFFERS" || k == ERROR) {
                c.regressions.push(format!("{file} {f}: not in the baseline, {k}"));
            }
            c.other.push(format!("{file}: not in the baseline"));
            continue;
        };
        let was = keyed(was);
        let rows = keyed(rows);
        let old: BTreeMap<_, _> = was.iter().cloned().collect();
        let new: BTreeMap<_, _> = rows.iter().cloned().collect();
        for (key, k) in &rows {
            let f = name(key);
            let Some(o) = old.get(key) else {
                let line = format!("{file} {f}: not in the baseline, {k}");
                if k == "DIFFERS" || k == ERROR {
                    c.regressions.push(line);
                } else {
                    c.other.push(line);
                }
                continue;
            };
            if o == k {
                continue;
            }
            let line = format!("{file} {f}: {o} -> {k}");
            if let (Some(was), Some(is)) = (agrees(o), agrees(k)) {
                if let (Some(was), Some(is)) = (was, is) {
                    if is.saturating_mul(2) < was {
                        c.lost.push(format!("{line}, fewer than half the runs"));
                    }
                }
                continue;
            }
            if k == "DIFFERS" || k == ERROR {
                c.regressions.push(line);
            } else if agrees(o).is_some() {
                c.lost.push(line);
            } else {
                c.other.push(line);
            }
        }
        for (key, k) in was.iter().filter(|(key, _)| !new.contains_key(key)) {
            let line = format!("{file} {}: {k}, no longer decompiled", name(key));
            if agrees(k).is_some() {
                c.lost.push(line);
            } else {
                c.other.push(line);
            }
        }
    }
    for file in base.keys().filter(|f| !now.contains_key(*f)) {
        c.lost.push(format!("{file}: in the baseline, not checked"));
    }
    c
}

#[cfg(test)]
mod tests {
    use super::*;

    fn b(entries: &[(&str, &[(&str, &str)])]) -> Baseline {
        entries
            .iter()
            .map(|(f, rows)| (f.to_string(), rows.iter().map(|(a, c)| (a.to_string(), c.to_string())).collect()))
            .collect()
    }

    #[test]
    fn render_round_trips() {
        let x = b(&[
            ("t/a.xml", &[("f", "AGREES"), ("g g", "UNKNOWN(budget)"), ("h", "UNKNOWN(no switch row)"), ("-[A b:]", "UNKNOWN(no switch row)")]),
            ("t/b.xml", &[("", ERROR)]),
        ]);
        assert_eq!(parse(&render(&x)).unwrap(), x);
    }

    #[test]
    fn only_a_new_difference_is_a_regression() {
        let base = b(&[("a", &[("f", "AGREES"), ("g", "DIFFERS"), ("h", "AGREES"), ("i", "UNKNOWN(budget)")])]);
        let now = b(&[("a", &[("f", "DIFFERS"), ("g", "AGREES"), ("h", "UNKNOWN(budget)"), ("i", "UNKNOWN(budget)")])]);
        let c = compare(&base, &now);
        assert_eq!(c.regressions, ["a f: AGREES -> DIFFERS"]);
        assert_eq!(c.lost, ["a h: AGREES -> UNKNOWN(budget)"]);
        assert_eq!(c.other, ["a g: DIFFERS -> AGREES"]);
        assert!(compare(&base, &base).is_empty());
    }

    #[test]
    fn a_file_that_fails_or_changes_its_functions() {
        let base = b(&[("a", &[("f", "AGREES")]), ("b", &[("f", "AGREES")])]);
        let now = b(&[("a", &[("", ERROR)]), ("c", &[("f", "AGREES")])]);
        let c = compare(&base, &now);
        assert_eq!(c.regressions, ["a : not in the baseline, ERROR"]);
        assert_eq!(c.lost, ["a f: AGREES, no longer decompiled", "b: in the baseline, not checked"]);
        assert_eq!(c.other, ["c: not in the baseline"]);
    }

    /// A changed function list still compares the functions both have, and a
    /// new function that differs is a regression even where an old one already
    /// differed.
    #[test]
    fn a_changed_function_list_is_compared_by_name() {
        let base = b(&[("a", &[("f", "DIFFERS"), ("g", "AGREES")])]);
        let now = b(&[("a", &[("f", "DIFFERS"), ("h", "DIFFERS")])]);
        let c = compare(&base, &now);
        assert_eq!(c.regressions, ["a h: not in the baseline, DIFFERS"]);
        assert_eq!(c.lost, ["a g: AGREES, no longer decompiled"]);
        assert_eq!((c.agreed, c.functions), ((1, 0), (2, 2)));
    }

    /// An agreement over far fewer runs is lost coverage; a small change in the
    /// count, or a count against an old class without one, is not reported.
    #[test]
    fn an_agreement_over_far_fewer_runs_is_lost_coverage() {
        let base = b(&[("a", &[("f", "AGREES(256)"), ("g", "AGREES(256)"), ("h", "AGREES")])]);
        let now = b(&[("a", &[("f", "AGREES(1)"), ("g", "AGREES(250)"), ("h", "AGREES(3)")])]);
        let c = compare(&base, &now);
        assert_eq!(c.lost, ["a f: AGREES(256) -> AGREES(1), fewer than half the runs"]);
        assert!(c.regressions.is_empty() && c.other.is_empty());
        assert_eq!(c.agreed, (3, 3));
        assert_eq!(parse(&render(&now)).unwrap(), now);
    }

    /// A function decompiled twice pairs each decompile with its own.
    #[test]
    fn a_function_decompiled_twice_pairs_by_occurrence() {
        let base = b(&[("a", &[("f", "DIFFERS"), ("f", "AGREES")])]);
        let now = b(&[("a", &[("g", "AGREES"), ("f", "DIFFERS"), ("f", "DIFFERS")])]);
        let c = compare(&base, &now);
        assert_eq!(c.regressions, ["a f (decompile 2): AGREES -> DIFFERS"]);
        assert_eq!(c.other, ["a g: not in the baseline, AGREES"]);
    }
}
