//! (kuna) The `peordinal` option gate: name a PE import-by-ordinal from a
//! built-in export table for the system DLLs whose ordinals are fixed
//! (`OLEAUT32`, `WS2_32`, `WSOCK32`, `MSVBVM60`), instead of synthesizing
//! `<DLL>_Ordinal_<n>` (`kuna-analysis::loader::kuna_peordinal`).
//!
//! Import names are resolved inside `load file`, upstream of every per-function
//! `option` command, so the gate is a process environment variable
//! ([`PEORDINAL_ENV`]) set by the `kuna` CLI before the file is loaded, the same
//! pattern as `rexthunk`. `Architecture::analysis_peordinal` exists only so the
//! option is catalog-visible and `phase catalog`'s live `current` field reflects it.
//!
//! Default **on**; `--option peordinal off` restores the synthesized names.

/// Environment variable that gates ordinal-import naming. Absent or any value
/// other than the off-tokens means **enabled**; `off`/`0`/`false` means disabled.
pub const PEORDINAL_ENV: &str = "KUNA_PEORDINAL";

/// Whether ordinal-import naming is enabled for this process.
pub fn peordinal_enabled() -> bool {
    match std::env::var(PEORDINAL_ENV) {
        Ok(v) => !matches!(v.trim().to_ascii_lowercase().as_str(), "off" | "0" | "false"),
        Err(_) => true,
    }
}

/// Bridge a `set_kuna_option("peordinal", val)` toggle to [`PEORDINAL_ENV`] so a
/// later `load file` in the same process sees the choice.
pub fn set_peordinal_env(on: bool) {
    std::env::set_var(PEORDINAL_ENV, if on { "on" } else { "off" });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn env_gate_default_on_and_off_tokens() {
        std::env::remove_var(PEORDINAL_ENV);
        assert!(peordinal_enabled(), "unset env => default on");
        for off in ["off", "0", "false", "OFF", "False"] {
            std::env::set_var(PEORDINAL_ENV, off);
            assert!(!peordinal_enabled(), "`{off}` => disabled");
        }
        for on in ["on", "1", "true", ""] {
            std::env::set_var(PEORDINAL_ENV, on);
            assert!(peordinal_enabled(), "`{on}` => enabled");
        }
        set_peordinal_env(false);
        assert!(!peordinal_enabled());
        set_peordinal_env(true);
        assert!(peordinal_enabled());
        std::env::remove_var(PEORDINAL_ENV);
    }
}
