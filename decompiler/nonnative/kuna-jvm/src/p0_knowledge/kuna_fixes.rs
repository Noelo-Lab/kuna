//! kuna-only fixes of Vineflower defects, and the switch that turns them all off.
//!
//! The port prints what Vineflower 1.12.0 prints, failures included. A fix is
//! a deliberate departure from that: each is a [`KunaFix`] with its own
//! option row (`jvm_fix_<slug>`, on by default), and every site where kuna
//! behaves differently from the Java asks [`enabled`] and keeps the Java's
//! behaviour verbatim in the other branch. `jvm_vineflower_compat` (off by
//! default) disables every fix at once, which gives Vineflower's exact text;
//! the Vineflower-oracle corpus runs that way.
//!
//! `Fernflower::new` takes the switches off its property list (so the option
//! map stays Vineflower's), stores the resolved set in the decompilation
//! context, and installing a context makes its set the one [`enabled`]
//! answers from on this thread.

use std::cell::Cell;

/// The option that disables every kuna-only fix.
pub const VINEFLOWER_COMPAT: &str = "jvm_vineflower_compat";

/// The prefix of every fix's option.
pub const FIX_PREFIX: &str = "jvm_fix_";

/// One kuna-only fix; its option is [`KunaFix::option`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum KunaFix {
    /// `jvm_fix_report_to_kuna`: the failure comment asks for reports to kuna's tracker.
    ReportToKuna,
    /// `jvm_fix_lambda_wildcard_return`: a method reference on a receiver
    /// whose type argument is `?` no longer fails type inference.
    LambdaWildcardReturn,
    /// `jvm_fix_twr_unary_condition`: a `catch (Throwable)` handler that starts
    /// with `if (!flag)` is not taken for a try-with-resources null check.
    TwrUnaryCondition,
    /// `jvm_fix_statement_as_expression`: a statement-only exprent (an
    /// `assert`) is not written as a lambda's expression body or a `for` update.
    StatementAsExpression,
}

impl KunaFix {
    /// Every fix, in declaration order.
    pub const ALL: &'static [KunaFix] = &[
        KunaFix::ReportToKuna,
        KunaFix::LambdaWildcardReturn,
        KunaFix::TwrUnaryCondition,
        KunaFix::StatementAsExpression,
    ];

    /// The option row that turns this fix on or off.
    pub fn option(self) -> &'static str {
        match self {
            KunaFix::ReportToKuna => "jvm_fix_report_to_kuna",
            KunaFix::LambdaWildcardReturn => "jvm_fix_lambda_wildcard_return",
            KunaFix::TwrUnaryCondition => "jvm_fix_twr_unary_condition",
            KunaFix::StatementAsExpression => "jvm_fix_statement_as_expression",
        }
    }

    /// The fix whose option is `name`.
    pub fn from_option(name: &str) -> Option<KunaFix> {
        KunaFix::ALL.iter().copied().find(|f| f.option() == name)
    }

    fn bit(self) -> u64 {
        1 << (self as u32)
    }
}

/// A set of active fixes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct KunaFixes(u64);

impl KunaFixes {
    /// Every fix active: the engine's default.
    pub const DEFAULT: KunaFixes = KunaFixes(u64::MAX);
    /// No fix active: Vineflower's behaviour.
    pub const COMPAT: KunaFixes = KunaFixes(0);

    pub fn contains(self, fix: KunaFix) -> bool {
        self.0 & fix.bit() != 0
    }

    /// The set for option pairs (later pairs win): every fix whose option is
    /// not off, or none while `jvm_vineflower_compat` is on.
    pub fn from_options<'a>(pairs: impl IntoIterator<Item = (&'a str, &'a str)>) -> KunaFixes {
        let mut compat = false;
        let mut off = 0u64;
        for (key, value) in pairs {
            if key == VINEFLOWER_COMPAT {
                compat = switch_value(value);
            } else if let Some(fix) = KunaFix::from_option(key) {
                if switch_value(value) {
                    off &= !fix.bit();
                } else {
                    off |= fix.bit();
                }
            }
        }
        if compat {
            KunaFixes::COMPAT
        } else {
            KunaFixes(KunaFixes::DEFAULT.0 & !off)
        }
    }

    /// The active fixes, in [`KunaFix::ALL`] order.
    pub fn list(self) -> Vec<KunaFix> {
        KunaFix::ALL
            .iter()
            .copied()
            .filter(|f| self.contains(*f))
            .collect()
    }
}

/// Whether a switch value turns its option on (`0`, `off` and `false` turn it off).
pub fn switch_value(value: &str) -> bool {
    !(value == "0" || value.eq_ignore_ascii_case("off") || value.eq_ignore_ascii_case("false"))
}

/// Whether `key` is `jvm_vineflower_compat` or a fix's option.
pub fn is_switch(key: &str) -> bool {
    key == VINEFLOWER_COMPAT || KunaFix::from_option(key).is_some()
}

/// Splits the compat switch and the fix options off a custom property list:
/// the resulting set and the other pairs, in order.
pub fn take_switches(custom_properties: &[(String, String)]) -> (KunaFixes, Vec<(String, String)>) {
    let fixes = KunaFixes::from_options(
        custom_properties
            .iter()
            .filter(|(k, _)| is_switch(k))
            .map(|(k, v)| (k.as_str(), v.as_str())),
    );
    let rest = custom_properties
        .iter()
        .filter(|(k, _)| !is_switch(k))
        .cloned()
        .collect();
    (fixes, rest)
}

thread_local! {
    static ACTIVE: Cell<KunaFixes> = const { Cell::new(KunaFixes::DEFAULT) };
}

/// Whether `fix` is active for the engine instance running on this thread.
pub fn enabled(fix: KunaFix) -> bool {
    ACTIVE.with(|a| a.get().contains(fix))
}

/// The fixes [`enabled`] answers from on this thread.
pub fn active() -> KunaFixes {
    ACTIVE.with(Cell::get)
}

/// Makes `fixes` the set [`enabled`] answers from on this thread; installing
/// a decompilation context does this with the context's set.
pub fn activate(fixes: KunaFixes) {
    ACTIVE.with(|a| a.set(fixes))
}
