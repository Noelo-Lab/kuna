//! (kuna) The `pdatachained` option gate — a PE `.pdata` record whose
//! `UNWIND_INFO` carries `UNW_FLAG_CHAININFO` names a separated code chunk of
//! another function, not a function start.
//!
//! ## The gap it closes
//!
//! MSVC splits one function across several `RUNTIME_FUNCTION` records whenever it
//! shrink-wraps a prologue or moves a cold block out of line. The first record is
//! the function; every later one describes a chunk of it and says so, by pointing
//! at an `UNWIND_INFO` with `UNW_FLAG_CHAININFO` set and a trailing chained
//! `RUNTIME_FUNCTION` naming the primary. kuna's `.pdata` oracle read only
//! `BeginAddress`, so each chunk became a `sub_<addr>` of its own — and because
//! `funcboundflow` truncates a fall-through that reaches a known function entry,
//! the real function was then cut off at its own chunk and emitted as a stub.
//!
//! ## Why this is an env-var bridge, not an `Architecture` flag
//!
//! The candidate reaches the function inventory by two routes that both run
//! inside `load file`: `entry::collect_entries` (the committed facts) and
//! `passes::listing_seeds` (the recursive-descent walk's seeds). Neither has an
//! `&Architecture` in scope, and both run upstream of every per-function `option`
//! command, so a flag on `Architecture` would be read too late. The gate is
//! therefore a process **environment variable** ([`PDATACHAINED_ENV`]), consulted
//! by the oracle and set by the `kuna` CLI on the `decomp_dbg` subprocess (and on
//! its own process for the in-process `functions`/`decompile-all` drivers) before
//! the file is loaded. This mirrors the `relocobjects` / `dynrelocs` /
//! `i386_pie_plt` loader-option pattern. The companion
//! `Architecture::analysis_pdatachained` bool exists only so the option is
//! catalog-visible and `phase catalog`'s live `current` field reflects it.
//!
//! Default **on**: the record itself declares the address to be interior, so
//! claiming a function there contradicts the image's own metadata (Ghidra skips
//! the same records at `ImageRuntimeFunctionEntries_X86.markup`).
//! `--option pdatachained off` restores the byte-identical pre-fix inventory.

/// Environment variable that gates the chained-`UNWIND_INFO` entry skip (read by
/// `kuna-analysis::analyzers::entry::pe_entry`). Absent or any value other than
/// the off-tokens ⇒ **enabled** (the default-on correctness fix); set to
/// `off`/`0`/`false` ⇒ disabled (pre-fix behavior).
pub const PDATACHAINED_ENV: &str = "KUNA_PDATACHAINED";

/// Whether the chained-`UNWIND_INFO` entry skip is enabled for this process.
/// Default **on**: only an explicit off-token in [`PDATACHAINED_ENV`] disables it.
pub fn pdatachained_enabled() -> bool {
    match std::env::var(PDATACHAINED_ENV) {
        Ok(v) => !matches!(v.trim().to_ascii_lowercase().as_str(), "off" | "0" | "false"),
        Err(_) => true, // unset ⇒ default-on
    }
}

/// Bridge a `set_kuna_option("pdatachained", val)` toggle to [`PDATACHAINED_ENV`]
/// so a later `load file` in the *same* process sees the choice (the interactive
/// `option pdatachained …` before `load file` path, and the in-process CLI drivers).
pub fn set_pdatachained_env(on: bool) {
    std::env::set_var(PDATACHAINED_ENV, if on { "on" } else { "off" });
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The gate defaults on (unset env) and only the off-tokens disable it.
    /// Serialized by being a single test (env is process-global).
    #[test]
    fn env_gate_default_on_and_off_tokens() {
        std::env::remove_var(PDATACHAINED_ENV);
        assert!(pdatachained_enabled(), "unset env ⇒ default on");

        for off in ["off", "0", "false", "OFF", "False"] {
            std::env::set_var(PDATACHAINED_ENV, off);
            assert!(!pdatachained_enabled(), "`{off}` ⇒ disabled");
        }
        for on in ["on", "1", "true", ""] {
            std::env::set_var(PDATACHAINED_ENV, on);
            assert!(pdatachained_enabled(), "`{on}` ⇒ enabled");
        }

        set_pdatachained_env(false);
        assert!(!pdatachained_enabled());
        set_pdatachained_env(true);
        assert!(pdatachained_enabled());

        std::env::remove_var(PDATACHAINED_ENV);
    }
}
