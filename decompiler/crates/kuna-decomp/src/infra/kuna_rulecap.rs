//! (kuna-verify) Counting and capping rule applications in one decompile, for
//! finding the rule application after which the decompiled function computes
//! something else (diffverify C3, `docs/features/diffverify/proposal.md`).
//!
//! Unarmed, the default, every rule applies as usual and the only cost is one
//! thread-local flag read per rule attempt. Armed, every application is
//! counted by rule name, and [`Mode::Limit`] lets one rule apply only its first
//! `allow` times; every other rule and every action runs as usual, so the
//! decompile completes. The address of that rule's last application is kept.

use std::cell::{Cell, RefCell};
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Mode {
    Observe,
    Limit { rule: String, allow: u64 },
}

/// What an armed decompile applied.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Record {
    /// Applications per rule name.
    pub counts: BTreeMap<String, u64>,
    /// [`Mode::Limit`]: the op address of the limited rule's last application.
    pub last: Option<u64>,
    /// Every application's rule, in the order they were made.
    pub sequence: Vec<String>,
}

thread_local! {
    static ARMED: Cell<bool> = const { Cell::new(false) };
    static STATE: RefCell<(Option<Mode>, Record)> = const { RefCell::new((None, Record { counts: BTreeMap::new(), last: None, sequence: Vec::new() })) };
}

pub fn arm(mode: Mode) {
    STATE.with(|s| *s.borrow_mut() = (Some(mode), Record::default()));
    ARMED.with(|a| a.set(true));
}

pub fn disarm() -> Record {
    ARMED.with(|a| a.set(false));
    STATE.with(|s| std::mem::take(&mut s.borrow_mut().1))
}

#[inline]
pub(crate) fn armed() -> bool {
    ARMED.with(Cell::get)
}

/// Whether `rule` may apply once more.
pub(crate) fn allows(rule: &str) -> bool {
    STATE.with(|s| match &s.borrow().0 {
        Some(Mode::Limit { rule: r, allow }) if r == rule => s.borrow().1.counts.get(rule).copied().unwrap_or(0) < *allow,
        _ => true,
    })
}

pub(crate) fn note(rule: &str, addr: u64) {
    STATE.with(|s| {
        let mut s = s.borrow_mut();
        *s.1.counts.entry(rule.to_string()).or_default() += 1;
        s.1.sequence.push(rule.to_string());
        if matches!(&s.0, Some(Mode::Limit { rule: r, .. }) if r == rule) {
            s.1.last = Some(addr);
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_limit_stops_one_rule_and_counts_all() {
        assert!(!armed());
        arm(Mode::Limit { rule: "RuleX".into(), allow: 1 });
        assert!(armed() && allows("RuleX"));
        note("RuleX", 0x40);
        note("RuleY", 0x44);
        assert!(!allows("RuleX") && allows("RuleY"));
        let r = disarm();
        assert_eq!(r.counts.get("RuleX"), Some(&1));
        assert_eq!(r.counts.get("RuleY"), Some(&1));
        assert_eq!(r.last, Some(0x40));
        assert_eq!(r.sequence, ["RuleX", "RuleY"]);
        assert!(!armed());
    }
}
