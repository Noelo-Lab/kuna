//! (kuna) The `debuglink` option gate: name a stripped ELF's functions, and read
//! its DWARF, from the separate debug file its build-id or `.gnu_debuglink`
//! names. The symbols are read inside `load file`, before any `option` command
//! runs, so the gate is the process environment variable [`DEBUGLINK_ENV`].
//! `Architecture::analysis_debuglink` exists for the catalog's live value.

/// Environment variable gating the companion lookup. Absent or an on-token
/// means enabled; an off-token disables it.
pub const DEBUGLINK_ENV: &str = "KUNA_DEBUGLINK";

/// Whether the companion lookup is enabled for this load.
pub fn debuglink_enabled() -> bool {
    crate::kuna_loadoptions::scoped(|o| o.debuglink).unwrap_or_else(crate::kuna_loadoptions::env::debuglink)
}

/// Bridge `option debuglink on|off` to [`DEBUGLINK_ENV`] for a later `load file`.
pub fn set_debuglink_env(on: bool) {
    std::env::set_var(DEBUGLINK_ENV, if on { "on" } else { "off" });
}
