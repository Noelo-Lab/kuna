//! `--option NAME` name validation, shared by every CLI surface that takes one.
//!
//! The settable-option namespace is the LLM control surface, so a misspelled
//! name is an evidence bug, not a papercut: an agent that writes
//! `--option LOWEREDSWITCH off` and sees no change concludes the decision point
//! is not the cause, when in fact it never flipped.  Until this module existed
//! `kuna decompile` accepted any name at all — it lowers each pair into an
//! `option NAME VALUE` line for `decomp_dbg`, and the console reports an
//! unrecognized name as an `Execution error:` on **stdout** with exit status 0,
//! which the driver has no reason to read (an unrelated console diagnostic must
//! not change the verdict; see `decompile.rs (check_errors)`).
//!
//! So the name is checked here instead, before anything is loaded, against the
//! same two tables the engine dispatches on: the kuna stage-model options
//! ([`KUNA_OPTION_NAMES`]) and the upstream `OptionDatabase` element ids
//! ([`UPSTREAM_OPTION_ELEMENTS`]).  The in-process surfaces already rejected an
//! unknown name at `decompile_all.rs (apply_one_option)`; checking up front
//! makes every surface answer the same way, and answer before the load.

use kuna_decomp::options::{UPSTREAM_OPTION_ELEMENTS, KUNA_OPTION_NAMES};

/// Whether `name` is a settable option name the engine will dispatch on.
pub(crate) fn is_known(name: &str) -> bool {
    KUNA_OPTION_NAMES.contains(&name)
        || UPSTREAM_OPTION_ELEMENTS.iter().any(|e| e.get_name() == name)
        || crate::decompile_all::is_loadtime_gate(name)
}

/// Check one `--option NAME VALUE` pair's name, returning the CLI error text
/// for an unrecognized one.
///
/// The wording keeps the in-process surfaces' `option <name>: Unknown option`
/// prefix so the two paths read the same, and adds the nearest catalogued
/// spelling when there is one — the reported misses were `LOWEREDSWITCH`,
/// `lowered_switch` and `loweredswitc` for `loweredswitch`, all of which a
/// suggestion answers directly.
pub(crate) fn check(name: &str) -> Result<(), String> {
    if is_known(name) {
        return Ok(());
    }
    let mut msg = format!("option {name}: Unknown option");
    if let Some(near) = nearest(name) {
        msg.push_str(&format!(" (did you mean {near:?}?)"));
    }
    msg.push_str("; `kuna catalog` lists every settable name");
    Err(msg)
}

/// Every settable name, for the suggestion search.
fn all_names() -> impl Iterator<Item = &'static str> {
    KUNA_OPTION_NAMES
        .iter()
        .copied()
        .chain(UPSTREAM_OPTION_ELEMENTS.iter().map(|e| e.get_name()))
}

/// Lowercase and drop `_`/`-`, so a case or separator slip compares equal.
fn squash(name: &str) -> String {
    name.chars().filter(|c| *c != '_' && *c != '-').flat_map(char::to_lowercase).collect()
}

/// The catalogued name closest to `name`: an exact match after case and
/// separator normalization first, else the single nearest edit-distance
/// neighbour within a length-scaled budget.
fn nearest(name: &str) -> Option<&'static str> {
    let squashed = squash(name);
    if let Some(hit) = all_names().find(|candidate| squash(candidate) == squashed) {
        return Some(hit);
    }
    // One slip in a short name, two in a long one; beyond that the "suggestion"
    // is noise (`zzzznotanoption` is within 9 edits of nothing worth printing).
    let budget = if squashed.len() >= 10 { 2 } else { 1 };
    all_names()
        .map(|candidate| (distance(&squashed, &squash(candidate)), candidate))
        .filter(|(d, _)| *d <= budget)
        .min_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(b.1)))
        .map(|(_, candidate)| candidate)
}

/// Levenshtein edit distance over `char`s (the two-row form; option names are
/// short enough that the allocation is irrelevant).
fn distance(a: &str, b: &str) -> usize {
    let b: Vec<char> = b.chars().collect();
    let mut prev: Vec<usize> = (0..=b.len()).collect();
    let mut cur = vec![0usize; b.len() + 1];
    for (i, ca) in a.chars().enumerate() {
        cur[0] = i + 1;
        for (j, cb) in b.iter().enumerate() {
            let sub = prev[j] + usize::from(ca != *cb);
            cur[j + 1] = sub.min(prev[j + 1] + 1).min(cur[j] + 1);
        }
        std::mem::swap(&mut prev, &mut cur);
    }
    prev[b.len()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_catalogued_name_from_either_table_is_accepted() {
        assert!(is_known("loweredswitch"), "a kuna stage-model option");
        assert!(is_known("setlanguage"), "an upstream OptionDatabase option");
        assert!(is_known("readonly"), "an upstream option whose element id lives elsewhere");
        assert!(is_known("relocobjects"), "a load-time gate");
    }

    #[test]
    fn every_name_the_engine_dispatches_on_is_accepted() {
        for name in all_names() {
            assert!(is_known(name), "{name} is dispatched on but rejected up front");
        }
    }

    #[test]
    fn an_unrecognized_name_is_rejected_and_names_itself() {
        let err = check("zzzznotanoption").unwrap_err();
        assert!(err.contains("zzzznotanoption"), "{err}");
        assert!(err.contains("Unknown option"), "{err}");
        assert!(!err.contains("did you mean"), "nothing is near it: {err}");
    }

    #[test]
    fn the_reported_near_misses_each_suggest_the_real_name() {
        for miss in ["LOWEREDSWITCH", "lowered_switch", "loweredswitc", "lowered-switch"] {
            let err = check(miss).unwrap_err();
            assert!(err.contains("did you mean \"loweredswitch\"?"), "{miss}: {err}");
        }
    }
}
