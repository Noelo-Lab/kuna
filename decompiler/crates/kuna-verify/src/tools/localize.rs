//! diffverify C3: the first rule application after which a function differs.
//!
//! The decompile that differs is rerun once per rule that applied in it, with
//! that rule alone taken out (`kuna_rulecap`); every other rule and every
//! action runs, so each rerun is a complete decompile. A rule without which the
//! function agrees is a culprit, and a binary search over how many times it
//! may apply finds `k` where `k - 1` applications agree and `k` differ: the
//! `k`-th, named by the op address it rewrote. Both ends are measured, so a
//! verdict that flips more than once still yields a pair that brackets a flip.
//! When no single rule's removal helps, the cause is an action, or rules that
//! only go wrong together.

use crate::check::check_function;
use crate::report::{Report, Verdict};
use crate::session::{self, Cap};
use kuna_decomp::kuna_rulecap::{Mode, Record};
use crate::Options;

/// Where a script comes from: a datatest, or an object file and options.
pub enum Source<'a> {
    Datatest(&'a str),
    /// `contract` is a C source and its name (`--contract`), as `check` takes it.
    Binary { path: &'a str, functions: Vec<String>, options: &'a [(String, String)], contract: Option<(&'a str, &'a str)> },
}

pub enum Finding {
    /// The full decompile does not differ.
    NotDiffering(Report),
    /// It differs, and taking out any one rule that applied leaves it
    /// differing (or not checkable): the cause is an action, or rules together.
    NoRule { applied: Vec<(String, u64)>, inconclusive: Vec<String>, report: Report },
    /// Each rule whose removal makes it agree, and where it goes wrong.
    Rules { applied: usize, culprits: Vec<Culprit>, inconclusive: Vec<String>, report: Report },
}

/// Application `k` of `total` of `rule`, at `addr`: with `k - 1` allowed the
/// function agrees over every run, with `k` it differs. Found by bisection,
/// which assumes a later application does not undo a difference, so `k` is a
/// candidate, not a proven first cause.
pub struct Culprit {
    pub rule: String,
    pub k: u64,
    pub total: u64,
    pub addr: Option<u64>,
    /// Where that application falls among all the decompile's applications.
    pub step: usize,
}

pub struct Localized {
    pub function: String,
    /// Which decompile of the script, from 0.
    pub decompile: usize,
    pub finding: Finding,
}

/// Runs past the full decompile's failing one that a probe checks; a culprit's
/// agreeing side is confirmed over every run.
const QUICK_RUNS: u32 = 32;

/// Run the script up to its `decompile`-th decompile under `mode`, and check it.
/// `None` for the report when the capped decompile itself fails.
fn probe(src: &Source, roots: &[String], decompile: usize, mode: Mode, opts: &Options) -> Result<(Option<Report>, Record), String> {
    let mut cap = Cap { decompile, mode, result: None };
    let mut seen = 0usize;
    let mut report = None;
    let (commands, conf) = match src {
        Source::Datatest(path) => {
            let (prog, commands) = session::load_datatest(path, roots)?;
            (commands, Some(prog))
        }
        Source::Binary { path, functions, options, .. } => (session::binary_script(path, functions, options), None),
    };
    let source = match src {
        Source::Binary { contract, .. } => *contract,
        Source::Datatest(_) => None,
    };
    let mut failure = None;
    session::run(commands, conf, roots, true, Some(&mut cap), |prog, fd| {
        if seen == decompile {
            let contract = match source.map(|(src, name)| crate::contract::from_source(src, fd.get_name(), name, crate::abi::c_sizes(prog))) {
                Some(Err(e)) => {
                    failure = Some(e);
                    None
                }
                Some(Ok(c)) => Some(c),
                None => None,
            };
            report = Some(check_function(prog, fd, contract.as_ref(), opts));
        }
        seen += 1;
    })?;
    if let Some(e) = failure {
        return Err(e);
    }
    Ok((report, cap.result.unwrap_or_default()))
}

fn differs(r: &Report) -> bool {
    matches!(r.verdict, Verdict::Differs { .. })
}

/// Localize the `pass`-th decompile (from 1) of `function`, or by default the
/// first of its decompiles that differs.
pub fn localize(
    src: &Source,
    roots: &[String],
    function: &str,
    pass: Option<usize>,
    opts: &Options,
) -> Result<Localized, String> {
    let all = match src {
        Source::Datatest(path) => {
            let r = crate::check_datatest(path, roots, opts);
            if let Some(e) = r.error.filter(|_| r.reports.is_empty()) {
                return Err(e);
            }
            r.reports
        }
        Source::Binary { path, functions, options, contract } => crate::check_binary(path, functions, options, roots, *contract, opts)?,
    };
    let mine: Vec<usize> = all.iter().enumerate().filter(|(_, r)| r.function == function).map(|(i, _)| i).collect();
    if mine.is_empty() {
        return Err(format!("no decompile of {function}"));
    }
    let decompile = match pass {
        Some(p) => *mine.get(p.wrapping_sub(1)).ok_or_else(|| format!("{function} is decompiled {} time(s)", mine.len()))?,
        None => mine.iter().copied().find(|&i| differs(&all[i])).unwrap_or(mine[0]),
    };
    let done = |finding| Ok(Localized { function: function.to_string(), decompile, finding });
    let (full, record) = probe(src, roots, decompile, Mode::Observe, opts)?;
    let full = full.ok_or_else(|| format!("decompile {} of the script fails when counted", decompile + 1))?;
    if !differs(&full) {
        return done(Finding::NotDiffering(full));
    }
    let full_run = match full.verdict {
        Verdict::Differs { run, .. } => run,
        _ => 0,
    };
    let quick = Options { runs: opts.runs.min(full_run.saturating_add(QUICK_RUNS)), ..*opts };
    let mut culprits = Vec::new();
    let mut inconclusive = Vec::new();
    for (rule, &total) in &record.counts {
        let limit = |allow| Mode::Limit { rule: rule.clone(), allow };
        let Some(without) = probe(src, roots, decompile, limit(0), &quick)?.0 else {
            inconclusive.push(format!("{rule}: the decompile fails"));
            continue;
        };
        if differs(&without) {
            if opts.trace {
                let first = without.lines.iter().find(|l| !l.starts_with("input")).cloned().unwrap_or_default();
                inconclusive.push(format!("{rule}: DIFFERS  {first}"));
            }
            continue;
        }
        if !matches!(without.verdict, Verdict::Agrees(_)) {
            inconclusive.push(format!("{rule}: {}", without.verdict.label()));
            continue;
        }
        let (mut lo, mut hi) = (0u64, total);
        let mut addr = None;
        while hi - lo > 1 {
            let mid = lo + (hi - lo) / 2;
            let (r, rec) = probe(src, roots, decompile, limit(mid), &quick)?;
            if r.as_ref().is_some_and(differs) {
                (hi, addr) = (mid, rec.last);
            } else {
                lo = mid;
            }
        }
        if addr.is_none() {
            addr = probe(src, roots, decompile, limit(hi), &quick)?.1.last;
        }
        let agrees = probe(src, roots, decompile, limit(lo), opts)?.0;
        if !agrees.as_ref().is_some_and(|r| matches!(r.verdict, Verdict::Agrees(_))) {
            let label = agrees.map_or_else(|| "the decompile fails".to_string(), |r| r.verdict.label());
            inconclusive.push(format!("{rule}: {label} with {lo} applied, over all runs"));
            continue;
        }
        let step = record.sequence.iter().enumerate().filter(|(_, r)| *r == rule).nth(hi as usize - 1).map_or(0, |(i, _)| i + 1);
        culprits.push(Culprit { rule: rule.clone(), k: hi, total, addr, step });
    }
    culprits.sort_by_key(|c| c.step);
    let applied = record.counts.len();
    if culprits.is_empty() {
        let applied = record.counts.into_iter().collect();
        return done(Finding::NoRule { applied, inconclusive, report: full });
    }
    done(Finding::Rules { applied, culprits, inconclusive, report: full })
}

impl Localized {
    pub fn render(&self) -> String {
        let head = format!("{} (decompile {})", self.function, self.decompile + 1);
        let indent = |r: &Report| r.render().lines().skip(1).map(|l| format!("  {l}\n")).collect::<String>();
        match &self.finding {
            Finding::NotDiffering(r) => format!("{head}  does not differ: {}\n", r.verdict.label()),
            Finding::NoRule { applied, inconclusive, report } => {
                let mut s = format!(
                    "{head}  differs, and still differs without any one of the {} rules that applied: \
                     the cause is an action, or rules together\n{}",
                    applied.len(),
                    indent(report)
                );
                let list: Vec<String> = applied.iter().map(|(r, n)| format!("{r}x{n}")).collect();
                s.push_str(&format!("  applied  {}\n", list.join(" ")));
                for i in inconclusive {
                    s.push_str(&format!("  without {i}\n"));
                }
                s
            }
            Finding::Rules { applied, culprits, inconclusive, report } => {
                let mut s = format!(
                    "{head}  differs; of the {applied} rules that applied, it agrees without these, in the order their last needed application ran (candidates found by bisection):\n"
                );
                for (i, c) in culprits.iter().enumerate() {
                    let at = c.addr.map_or_else(|| "?".to_string(), |a| format!("{a:#x}"));
                    let latest = if i + 1 == culprits.len() { "  <- latest" } else { "" };
                    s.push_str(&format!(
                        "  step {:>5}  {}  application {} of {} at {at} (agrees with {}, differs with {}){latest}\n",
                        c.step, c.rule, c.k, c.total, c.k - 1, c.k
                    ));
                }
                for i in inconclusive {
                    s.push_str(&format!("  without {i}\n"));
                }
                s.push_str(&indent(report));
                s
            }
        }
    }
}
