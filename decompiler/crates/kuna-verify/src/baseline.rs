//! The corpus baseline (`docs/baseline-verify.json`): each datatest file's
//! verdict classes in script order, and how a run compares with it.
//!
//! A class leaves out what varies without the verdict changing: an `AGREES`
//! run count, a `DIFFERS` run, an `UNKNOWN` address. Only a function that now
//! differs where it did not, or a file that can no longer be checked, is a
//! regression; every other change is reported so the baseline can be
//! re-recorded.

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
        "AGREES" | "DIFFERS" => r.class.clone(),
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
    let tagged = r.ends_with(')').then(|| [" UNKNOWN(", " SKIPPED(", " DIVERGES("].iter().filter_map(|t| r.rfind(t)).max()).flatten();
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
    /// A function that now differs, or a file that can no longer be checked.
    pub regressions: Vec<String>,
    /// A function that agreed and no longer does, without differing.
    pub lost: Vec<String>,
    /// Every other change, a function that no longer differs among them.
    pub other: Vec<String>,
}

impl Changes {
    pub fn is_empty(&self) -> bool {
        self.regressions.is_empty() && self.lost.is_empty() && self.other.is_empty()
    }
}

pub fn compare(base: &Baseline, now: &Baseline) -> Changes {
    let mut c = Changes::default();
    for (file, rows) in now {
        let Some(was) = base.get(file) else {
            c.other.push(format!("{file}: not in the baseline"));
            continue;
        };
        let names = |r: &[(String, String)]| r.iter().map(|(f, _)| f.clone()).collect::<Vec<_>>();
        if names(was) != names(rows) {
            let differs = rows.iter().any(|(_, k)| k == "DIFFERS" || k == ERROR);
            let line = format!("{file}: decompiles other functions than the baseline lists");
            if differs && was.iter().all(|(_, k)| k != "DIFFERS" && k != ERROR) {
                c.regressions.push(line);
            } else {
                c.other.push(line);
            }
            continue;
        }
        for ((f, old), (_, new)) in was.iter().zip(rows) {
            if old == new {
                continue;
            }
            let line = format!("{file} {f}: {old} -> {new}");
            if new == "DIFFERS" || new == ERROR {
                c.regressions.push(line);
            } else if old == "AGREES" {
                c.lost.push(line);
            } else {
                c.other.push(line);
            }
        }
    }
    for file in base.keys().filter(|f| !now.contains_key(*f)) {
        c.other.push(format!("{file}: in the baseline, not checked"));
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
        assert_eq!(c.regressions, ["a: decompiles other functions than the baseline lists"]);
        assert_eq!(c.other, ["c: not in the baseline", "b: in the baseline, not checked"]);
    }
}
