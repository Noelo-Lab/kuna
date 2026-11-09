//! (kuna) The `macho-arm64e` option gate: load an arm64e Mach-O under the
//! `AARCH64:LE:64:AppleSilicon` pointer-authentication spec instead of the
//! generic `v8A`.
//!
//! Spec selection is a **load-time** decision (`language_id_for` in
//! `kuna-analysis`), taken before any console `option` command runs, so like
//! [`ifuncfpret`](crate::kuna_ifuncfpret) the gate is a process environment
//! variable: the CLI exports it onto the `decomp_dbg` subprocess, the console
//! `option` arm sets it for a later `load file` in the same process, and the
//! loader reads it when it picks the language. Unset means **on**; only an
//! explicit off-token turns the AppleSilicon selection off. The companion
//! `Architecture::macho_arm64e` bool records the requested state for the
//! catalog's `current` field.

/// Environment variable the Mach-O loader reads to decide whether an arm64e
/// image selects the AppleSilicon spec. Absent or an on-token ⇒ enabled; an
/// off-token (`off`, `0`, `false`) ⇒ the generic `v8A` spec.
pub const MACHO_ARM64E_ENV: &str = "KUNA_MACHO_ARM64E";

/// Whether an arm64e Mach-O selects the AppleSilicon spec in this process.
pub fn macho_arm64e_enabled() -> bool {
    crate::options::env_toggle(MACHO_ARM64E_ENV, true)
}

/// Bridge a `set_kuna_option("macho-arm64e", val)` toggle to
/// [`MACHO_ARM64E_ENV`] so a later `load file` in the same process sees it.
pub fn set_macho_arm64e_env(on: bool) {
    std::env::set_var(MACHO_ARM64E_ENV, if on { "on" } else { "off" });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn env_gate_default_on_and_off_tokens() {
        std::env::remove_var(MACHO_ARM64E_ENV);
        assert!(macho_arm64e_enabled(), "unset env ⇒ default on");

        for on in ["on", "1", "true", "ON", "True", ""] {
            std::env::set_var(MACHO_ARM64E_ENV, on);
            assert!(macho_arm64e_enabled(), "`{on}` ⇒ enabled");
        }
        for off in ["off", "0", "false"] {
            std::env::set_var(MACHO_ARM64E_ENV, off);
            assert!(!macho_arm64e_enabled(), "`{off}` ⇒ disabled");
        }

        set_macho_arm64e_env(false);
        assert!(!macho_arm64e_enabled());
        set_macho_arm64e_env(true);
        assert!(macho_arm64e_enabled());

        std::env::remove_var(MACHO_ARM64E_ENV);
    }
}
