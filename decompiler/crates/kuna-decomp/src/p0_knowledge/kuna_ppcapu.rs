//! (kuna) The `ppcapu` option gate: select a compatible embedded language for a 32-bit
//! PowerPC ELF image from its APU requirements and embedded ABI flag.
//!
//! The language is chosen while the image loads, before any `option` command
//! runs, so the gate is the process environment variable [`PPCAPU_ENV`], set by
//! the CLI before loading. `Architecture::analysis_ppcapu` exists only for the
//! catalog's live `current` field.

/// Environment variable read by `kuna-analysis::loader::kuna_ppcaputarget`.
/// Enabled by default.
pub const PPCAPU_ENV: &str = "KUNA_PPCAPU";

/// Whether APU-based PowerPC language selection is enabled for this process.
pub fn ppcapu_enabled() -> bool {
    crate::kuna_loadoptions::scoped(|o| o.ppcapu).unwrap_or_else(crate::kuna_loadoptions::env::ppcapu)
}

/// Bridge a `set_kuna_option("ppcapu", val)` toggle to [`PPCAPU_ENV`] so a later
/// `load file` in the same process sees the choice.
pub fn set_ppcapu_env(on: bool) {
    std::env::set_var(PPCAPU_ENV, if on { "on" } else { "off" });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn env_gate_default_on_and_off_tokens() {
        std::env::remove_var(PPCAPU_ENV);
        assert!(ppcapu_enabled(), "unset env ⇒ default on");
        for on in ["on", "1", "true", "ON", ""] {
            std::env::set_var(PPCAPU_ENV, on);
            assert!(ppcapu_enabled(), "`{on}` ⇒ enabled");
        }
        for off in ["off", "0", "false", "invalid"] {
            std::env::set_var(PPCAPU_ENV, off);
            assert!(!ppcapu_enabled(), "`{off}` ⇒ disabled");
        }
        set_ppcapu_env(true);
        assert!(ppcapu_enabled());
        set_ppcapu_env(false);
        assert!(!ppcapu_enabled());
        std::env::remove_var(PPCAPU_ENV);
    }
}
