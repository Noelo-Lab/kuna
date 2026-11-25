//! Mutation testing of the certificate (kuna-verify's `mutate`, turned on the
//! learned machine): plant one fault in the hypothesis and see which check
//! refuses it.

use serde_json::{json, Value};

use crate::engine::{closure, conformance, words, Closure, Config, Learned, Machine};
use crate::learn::Hypothesis;

/// Every single-fault mutant, by kind. A transition is the set of symbols of
/// one state with one result and one target.
pub fn mutants(h: &Hypothesis) -> Vec<(&'static str, String, Hypothesis)> {
    let mut out = Vec::new();
    for q in 0..h.states {
        let mut groups: Vec<((u64, usize), Vec<usize>)> = Vec::new();
        for (a, key) in h.delta[q].iter().enumerate() {
            match groups.iter_mut().find(|(k, _)| k == key) {
                Some((_, syms)) => syms.push(a),
                None => groups.push((*key, vec![a])),
            }
        }
        for (g, ((o, to), syms)) in groups.iter().enumerate() {
            let edit = |f: &dyn Fn(&mut (u64, usize))| {
                let mut m = h.clone();
                for a in syms {
                    f(&mut m.delta[q][*a]);
                }
                m
            };
            for t in (0..h.states).filter(|t| t != to) {
                out.push((
                    "retarget",
                    format!("q{q} -[{g}]-> q{to} now q{t}"),
                    edit(&|d| d.1 = t),
                ));
            }
            out.push((
                "result",
                format!("q{q} -[{g}]-> result {o:#x} ^ 1"),
                edit(&|d| d.0 ^= 1),
            ));
            for (h2, (other, _)) in groups.iter().enumerate().filter(|(h2, _)| *h2 != g) {
                let mut m = h.clone();
                m.delta[q][syms[0]] = *other;
                out.push((
                    "guard",
                    format!("q{q}: one input of [{g}] moved to [{h2}]"),
                    m,
                ));
            }
        }
    }
    for gone in 1..h.states {
        for keep in (0..h.states).filter(|k| *k != gone) {
            let mut m = h.clone();
            for row in &mut m.delta {
                for d in row.iter_mut() {
                    if d.1 == gone {
                        d.1 = keep;
                    }
                }
            }
            out.push(("merge", format!("q{gone} merged into q{keep}"), m));
        }
    }
    out
}

/// Each mutant through the path-closure certificate and the two random
/// replays; kills per kind.
pub fn run(l: &Learned, m: &mut Machine, cfg: &Config) -> Result<Value, String> {
    let h = l.hypothesis.as_ref().ok_or("no hypothesis to mutate")?;
    let uniform = words(cfg, &l.tree.alphabet, false);
    let biased = words(cfg, &l.tree.alphabet, true);
    let check =
        |m: &mut Machine, h: &Hypothesis| -> Result<(bool, Option<usize>, Option<usize>), String> {
            let refused = closure(h, &l.states) != Closure::Proved;
            let u = conformance(m, h, &l.states, &uniform)?.map(|w| w.len());
            let b = conformance(m, h, &l.states, &biased)?.map(|w| w.len());
            Ok((refused, u, b))
        };
    let control = check(m, h)?;
    let mut rows = Vec::new();
    let mut totals: std::collections::BTreeMap<&str, [usize; 4]> =
        std::collections::BTreeMap::new();
    for (kind, label, mutant) in mutants(h) {
        let (refused, u, b) = check(m, &mutant)?;
        for k in [kind, "all"] {
            let t = totals.entry(k).or_default();
            t[0] += 1;
            t[1] += usize::from(refused);
            t[2] += usize::from(u.is_some());
            t[3] += usize::from(b.is_some());
        }
        rows.push(
            json!({ "kind": kind, "label": label, "closure_refuses": refused,
            "uniform_kills_at": u, "alphabet_kills_at": b }),
        );
    }
    Ok(json!({
        "control": { "closure_refuses": control.0, "uniform_kills_at": control.1, "alphabet_kills_at": control.2 },
        "totals": totals.iter().map(|(k, t)| (k.to_string(), json!({
            "mutants": t[0], "closure_refuses": t[1], "uniform_kills": t[2], "alphabet_kills": t[3] }))).collect::<serde_json::Map<_, _>>(),
        "rows": rows,
    }))
}
