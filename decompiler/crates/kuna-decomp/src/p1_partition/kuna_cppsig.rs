//! (kuna `cppsig`) The three-valued gate on applying a DEMANGLED C++ signature.
//!
//! The recovering pass lives in the analyzer tier
//! (`kuna_analysis::demangle::kuna_cppsig`); what lives here is the *decision*
//! the option names, because the option is not a plain on/off.
//!
//! Itanium mangling does not distinguish a **static** member function from a
//! **non-static** one — `leveldb::Status::OK()` and
//! `leveldb::TableBuilder::WriteBlock(…)` have the same nested-name shape — so a
//! `this` parameter is sometimes entailed by the mangling and sometimes only
//! inferable. Adding one that is not there does not merely lose precision, it
//! shifts every following parameter by a position. The option exposes exactly
//! that trade:
//!
//! | value | what it applies | measured on google/leveldb |
//! |---|---|---|
//! | `off` | nothing | — |
//! | `proven` | only the shapes the mangling entails: constructors, destructors, cv-/ref-qualified members, unqualified globals, explicit MSVC conventions | precision 1.0000, recall 0.7093 |
//! | `inferred` | additionally decides ambiguous nested names from class evidence in the binary's own symbols | precision 0.9278, recall 0.9978 |
//!
//! (Ghidra 12.1's own `this` decision on the same binary runs at precision 0.85.)

use kuna_base::error::{KunaError, KunaResult};

/// The resolved `option cppsig off|proven|inferred` mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum CppSigMode {
    /// Apply nothing; every mangled symbol keeps only its demangled NAME.
    Off,
    /// Apply only the prototypes whose implicit-object-parameter question the
    /// mangling answers on its own.
    #[default]
    Proven,
    /// Also apply the prototypes whose `this`-ness was inferred from class
    /// evidence mined out of the binary's own symbol table.
    Inferred,
}

impl CppSigMode {
    /// Is any demangled signature applied at all?
    pub fn enabled(self) -> bool {
        !matches!(self, CppSigMode::Off)
    }

    /// Should the inferred (class-evidence) tier be applied as well?
    pub fn inferred(self) -> bool {
        matches!(self, CppSigMode::Inferred)
    }

    /// The `option cppsig <p1>` token for this mode.
    pub fn as_str(self) -> &'static str {
        match self {
            CppSigMode::Off => "off",
            CppSigMode::Proven => "proven",
            CppSigMode::Inferred => "inferred",
        }
    }
}

/// Parse the `option cppsig off|proven|inferred` argument, producing the
/// resolved mode plus the confirmation message (the established value-option
/// idiom — cf. `parse_return_pair_form`).
pub fn parse_cppsig_mode(p1: &str) -> KunaResult<(CppSigMode, String)> {
    let mode = match p1 {
        "off" => CppSigMode::Off,
        "proven" => CppSigMode::Proven,
        "inferred" => CppSigMode::Inferred,
        _ => return Err(KunaError::parse("Must specify off, proven or inferred")),
    };
    Ok((mode, format!("Demangled C++ signature application set to {p1}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_the_three_values_and_rejects_the_rest() {
        assert_eq!(parse_cppsig_mode("off").unwrap().0, CppSigMode::Off);
        assert_eq!(parse_cppsig_mode("proven").unwrap().0, CppSigMode::Proven);
        assert_eq!(parse_cppsig_mode("inferred").unwrap().0, CppSigMode::Inferred);
        // `on` is deliberately NOT an alias: the caller must say which tier.
        assert!(parse_cppsig_mode("on").is_err());
        assert!(parse_cppsig_mode("").is_err());
    }

    #[test]
    fn the_tiers_compose() {
        assert!(!CppSigMode::Off.enabled());
        assert!(CppSigMode::Proven.enabled());
        assert!(!CppSigMode::Proven.inferred());
        assert!(CppSigMode::Inferred.enabled());
        assert!(CppSigMode::Inferred.inferred());
        assert_eq!(CppSigMode::Inferred.as_str(), "inferred");
    }
}
