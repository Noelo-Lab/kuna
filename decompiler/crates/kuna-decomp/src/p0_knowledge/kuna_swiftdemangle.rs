//! (kuna) The `swiftdemangle` option gate — render Swift-mangled symbol names
//! (and the ObjC runtime's `_Tt` class names) through kuna's Swift demangler
//! (`kuna-analysis::analyzers::demangle::kuna_swift`).
//!
//! ## Why an env-var bridge, not an `Architecture` flag
//!
//! Symbol names are demangled while the loader installs the symbol table,
//! inside `load file`, **upstream of every per-function `option` command** —
//! the `rexthunk`/`symbolnamerepair` loadtime pattern. The gate is therefore a
//! process environment variable ([`SWIFTDEMANGLE_ENV`]), set by the `kuna` CLI
//! before the file is loaded; the companion
//! `Architecture::analysis_swiftdemangle` bool exists only so the option is
//! catalog-visible and `phase catalog`'s live `current` reflects it.
//!
//! The demangler itself is additionally **shape-gated**: only names carrying a
//! Swift mangling prefix (`$s…`/`_$s…`/`$S…`/`_T0…`/`_Tt…` — shapes no other
//! scheme owns, tested by `sourcelang::is_swift_mangled`) ever reach it, so
//! the option is structurally inert on every non-Swift binary.
//!
//! Default **on**; `--option swiftdemangle off` keeps the raw mangled
//! spellings for auditing. A name the demangler cannot derive with certainty
//! passes through unchanged either way (parse-or-passthrough).

/// Environment variable gating Swift symbol demangling. Absent or any value
/// other than the off-tokens means **enabled**; `off`/`0`/`false` disables.
pub const SWIFTDEMANGLE_ENV: &str = "KUNA_SWIFTDEMANGLE";

/// Whether Swift demangling is enabled for this process.
pub fn swiftdemangle_enabled() -> bool {
    crate::options::env_toggle(SWIFTDEMANGLE_ENV, true)
}

/// Bridge a `set_kuna_option("swiftdemangle", val)` toggle to
/// [`SWIFTDEMANGLE_ENV`] so a later `load file` in the same process sees it.
pub fn set_swiftdemangle_env(on: bool) {
    std::env::set_var(SWIFTDEMANGLE_ENV, if on { "on" } else { "off" });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn env_gate_default_on_and_off_tokens() {
        std::env::remove_var(SWIFTDEMANGLE_ENV);
        assert!(swiftdemangle_enabled(), "unset env => default on");
        for off in ["off", "0", "false"] {
            std::env::set_var(SWIFTDEMANGLE_ENV, off);
            assert!(!swiftdemangle_enabled(), "`{off}` => disabled");
        }
        for on in ["on", "1", "true", ""] {
            std::env::set_var(SWIFTDEMANGLE_ENV, on);
            assert!(swiftdemangle_enabled(), "`{on}` => enabled");
        }
        std::env::remove_var(SWIFTDEMANGLE_ENV);
    }
}
