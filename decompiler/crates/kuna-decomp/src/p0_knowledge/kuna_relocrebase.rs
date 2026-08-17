//! (kuna) The `relocrebase` option gate — rebase the LOAD-TIME analysis facts of
//! a synthetically laid-out relocatable object (an ELF `ET_REL` `.o`, a COFF
//! `.obj`) into the loaded image's address space.
//!
//! ## Why this is an env-var bridge, not an `Architecture` flag
//!
//! The whole analyzer tier (`kuna_analysis::passes::run_default_analyses_per_pass`)
//! runs inside `load file`, which is **upstream of every per-function `option`
//! command** — an `Architecture` bool would be read after the facts were already
//! computed. And unlike the commit-gated passes (`datasyms`, `itaniumrtti`, …)
//! this gate cannot be deferred to the commit boundary: it changes the *inputs*
//! every pass reads, so honouring it at commit would mean running the whole pass
//! list twice.
//!
//! So the gate is a process **environment variable** ([`RELOCREBASE_ENV`]),
//! consulted by the analysis tier when it builds the rebased view and set by the
//! `kuna` CLI on the `decomp_dbg` subprocess (and on its own process for the
//! in-process `decompile-all`/`functions` drivers) *before* the file is loaded.
//! This mirrors the `relocobjects` / `i386_pie_plt` loader-option pattern. The
//! companion `Architecture::analysis_relocrebase` bool exists only so the option
//! is catalog-visible and `phase catalog`'s live `current` field reflects it.
//!
//! Default **on**: the pre-link addresses it replaces are provably wrong (they
//! name a different address space than the one the engine decodes in), so `on` is
//! a correctness fix, not a judgement call. `--option relocrebase off` (⇒ env
//! `off`) restores the byte-identical pre-fix behaviour.

/// Environment variable that gates the relocatable-object analysis rebase (read
/// by `kuna-analysis::loader::kuna_relocrebase::rebased_view`). Absent or any
/// value other than the off-tokens ⇒ **enabled** (the default-on correctness
/// fix); set to `off`/`0`/`false` ⇒ disabled (pre-fix behavior).
pub const RELOCREBASE_ENV: &str = "KUNA_RELOCREBASE";

/// Whether the relocatable-object analysis rebase is enabled for this process.
/// Default **on**: only an explicit off-token in [`RELOCREBASE_ENV`] disables it.
pub fn relocrebase_enabled() -> bool {
    match std::env::var(RELOCREBASE_ENV) {
        Ok(v) => !matches!(v.trim().to_ascii_lowercase().as_str(), "off" | "0" | "false"),
        Err(_) => true, // unset ⇒ default-on
    }
}

/// Bridge a `set_kuna_option("relocrebase", val)` toggle to [`RELOCREBASE_ENV`]
/// so a later `load file` in the *same* process sees the choice (the interactive
/// `option relocrebase … ` before `load file` path, and the in-process CLI
/// drivers).
pub fn set_relocrebase_env(on: bool) {
    std::env::set_var(RELOCREBASE_ENV, if on { "on" } else { "off" });
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The gate defaults on (unset env) and only the off-tokens disable it.
    /// Serialized by being a single test (env is process-global).
    #[test]
    fn env_gate_default_on_and_off_tokens() {
        std::env::remove_var(RELOCREBASE_ENV);
        assert!(relocrebase_enabled(), "unset env ⇒ default on");

        for off in ["off", "0", "false", "OFF", "False"] {
            std::env::set_var(RELOCREBASE_ENV, off);
            assert!(!relocrebase_enabled(), "`{off}` ⇒ disabled");
        }
        for on in ["on", "1", "true", ""] {
            std::env::set_var(RELOCREBASE_ENV, on);
            assert!(relocrebase_enabled(), "`{on}` ⇒ enabled");
        }

        set_relocrebase_env(false);
        assert!(!relocrebase_enabled());
        set_relocrebase_env(true);
        assert!(relocrebase_enabled());

        std::env::remove_var(RELOCREBASE_ENV);
    }
}
