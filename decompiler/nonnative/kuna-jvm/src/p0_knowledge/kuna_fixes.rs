//! kuna-only fixes of Vineflower defects, and the switch that turns them all off.
//!
//! The port prints what Vineflower 1.12.0 prints, failures included. A fix is
//! a deliberate departure from that: each is a [`KunaFix`], always on, named
//! by an internal id ([`KunaFix::id`]) rather than an option, and every site
//! where kuna behaves differently from the Java asks [`enabled`] and keeps the
//! Java's behaviour verbatim in the other branch. The one option is
//! `jvm_vineflower_compat` (off by default): on, it disables every fix, which
//! gives Vineflower's exact text; the Vineflower-oracle corpus runs that way.
//!
//! `Fernflower::new` takes the switch off its property list (so the option
//! map stays Vineflower's), stores the resolved set in the decompilation
//! context, and installing a context makes its set the one [`enabled`]
//! answers from on this thread.

use std::cell::Cell;

/// The option that disables every kuna-only fix.
pub const VINEFLOWER_COMPAT: &str = "jvm_vineflower_compat";

/// One kuna-only fix; its id is [`KunaFix::id`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum KunaFix {
    /// `report_to_kuna`: the failure comment asks for reports to kuna's tracker.
    ReportToKuna,
    /// `lambda_wildcard_return`: a method reference on a receiver
    /// whose type argument is `?` no longer fails type inference.
    LambdaWildcardReturn,
    /// `twr_unary_condition`: a `catch (Throwable)` handler that starts
    /// with `if (!flag)` is not taken for a try-with-resources null check.
    TwrUnaryCondition,
    /// `if_denest_fallthrough`: if-else-chain denesting no longer
    /// moves the code after an if into its then-branch while the if's body
    /// still reaches that code.
    IfDenestFallthrough,
    /// `switch_yield_const_retype`: a constant a switch expression
    /// yields keeps its own type when the switch's type is one no constant has.
    SwitchYieldConstRetype,
    /// `catch_loop_back_successor`: a try/catch is not built at a
    /// level where its successor would be its own try block.
    CatchLoopBackSuccessor,
    /// `kt_synthetic_anonymous_object`: an anonymous class with only
    /// synthetic-class Kotlin metadata is written as an `object` whose
    /// supertypes come from the class file.
    KtSyntheticAnonymousObject,
    /// `kt_local_as_extension_receiver`: a `$this$` local of a
    /// method without parameters is not taken for an extension receiver.
    KtLocalAsExtensionReceiver,
    /// `kt_default_args_super_call`: a Kotlin primary constructor's
    /// supertype call through a default-arguments constructor names the
    /// constructor it stands for.
    KtDefaultArgsSuperCall,
    /// `kt_default_arg_compound_branch`: a default value computed by
    /// more than one statement is left unrecovered instead of failing the class.
    KtDefaultArgCompoundBranch,
}

impl KunaFix {
    /// Every fix, in declaration order.
    pub const ALL: &'static [KunaFix] = &[
        KunaFix::ReportToKuna,
        KunaFix::LambdaWildcardReturn,
        KunaFix::TwrUnaryCondition,
        KunaFix::IfDenestFallthrough,
        KunaFix::SwitchYieldConstRetype,
        KunaFix::CatchLoopBackSuccessor,
        KunaFix::KtSyntheticAnonymousObject,
        KunaFix::KtLocalAsExtensionReceiver,
        KunaFix::KtDefaultArgsSuperCall,
        KunaFix::KtDefaultArgCompoundBranch,
    ];

    /// The fix's id, as the docs and `tests/jvm/kuna-fixes/cases.json` name it.
    pub fn id(self) -> &'static str {
        match self {
            KunaFix::ReportToKuna => "report_to_kuna",
            KunaFix::LambdaWildcardReturn => "lambda_wildcard_return",
            KunaFix::TwrUnaryCondition => "twr_unary_condition",
            KunaFix::IfDenestFallthrough => "if_denest_fallthrough",
            KunaFix::SwitchYieldConstRetype => "switch_yield_const_retype",
            KunaFix::CatchLoopBackSuccessor => "catch_loop_back_successor",
            KunaFix::KtSyntheticAnonymousObject => "kt_synthetic_anonymous_object",
            KunaFix::KtLocalAsExtensionReceiver => "kt_local_as_extension_receiver",
            KunaFix::KtDefaultArgsSuperCall => "kt_default_args_super_call",
            KunaFix::KtDefaultArgCompoundBranch => "kt_default_arg_compound_branch",
        }
    }

    /// The fix whose id is `id`.
    pub fn from_id(id: &str) -> Option<KunaFix> {
        KunaFix::ALL.iter().copied().find(|f| f.id() == id)
    }

    const fn bit(self) -> u64 {
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

    /// The set for option pairs (later pairs win): every fix, or none while
    /// `jvm_vineflower_compat` is on.
    pub fn from_options<'a>(pairs: impl IntoIterator<Item = (&'a str, &'a str)>) -> KunaFixes {
        let compat = pairs
            .into_iter()
            .filter(|(key, _)| *key == VINEFLOWER_COMPAT)
            .last()
            .is_some_and(|(_, value)| switch_value(value));
        if compat {
            KunaFixes::COMPAT
        } else {
            KunaFixes::DEFAULT
        }
    }

    /// This set without `fix`: a test hook for isolating one fix.
    #[doc(hidden)]
    pub const fn without(self, fix: KunaFix) -> KunaFixes {
        KunaFixes(self.0 & !fix.bit())
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

/// Whether `key` is `jvm_vineflower_compat`.
pub fn is_switch(key: &str) -> bool {
    key == VINEFLOWER_COMPAT
}

/// Splits `jvm_vineflower_compat` off a custom property list: the resulting
/// set and the other pairs, in order.
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
