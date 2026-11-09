//! (kuna `objcivars`) The load-time half of the instance-layout gate.
//!
//! The commit decides whether `self` is typed and the offset variables fold
//! (`Architecture::analysis_objcivars`); the pass that builds the structs runs
//! while the loader snapshots the image, upstream of every console `option`,
//! so the same switch also travels as a process environment variable the way
//! `dynrelocs` does. Off, no class struct is interned at all.

/// Environment variable carrying the `objcivars` choice into the load path.
/// Unset means on.
pub const OBJCIVARS_ENV: &str = "KUNA_OBJCIVARS";

/// Whether the pass builds the instance layouts.
pub fn objcivars_enabled() -> bool {
    match std::env::var(OBJCIVARS_ENV) {
        Ok(v) => !matches!(v.trim().to_ascii_lowercase().as_str(), "off" | "0" | "false" | "no"),
        Err(_) => true,
    }
}

/// Bridge a `set_kuna_option("objcivars", val)` choice to [`OBJCIVARS_ENV`] so
/// a later `load file` in the same process sees it.
pub fn set_objcivars_env(on: bool) {
    std::env::set_var(OBJCIVARS_ENV, if on { "on" } else { "off" });
}
