//! (kuna) The `i386_pie_plt` option gate — decode i386 **PIE** GOT-relative
//! (`jmp *disp(%ebx)`) PLT stubs so dynamic imports (`exit`/`dcgettext`/…) are
//! named and known-no-return functions (`exit`) are flagged.
//!
//! ## Why this is an env-var bridge, not an `Architecture` flag
//!
//! Closing `test_decompiling_nl_i386_pie::usage` (angr) is a **loader-tier**
//! change: the actual decode lives in `kuna-analysis`
//! (`s1_loader/elf_plt.rs::decode_i386`), the i386-PIE analog of the already-
//! shipped `decode_x86_64`/`decode_aarch64` PLT decoders.  The PLT→name map is
//! baked into the `ObjectLoadImage` at `load file`, which runs **upstream of
//! every per-function `option` command** (options are applied *after* the image
//! is opened).  An `Architecture` bool would therefore be read too late — by the
//! time `set_kuna_option` runs, the imports are already resolved (or not).
//!
//! So the gate is a process **environment variable** ([`I386_PIE_PLT_ENV`]),
//! consulted by the loader at decode time and set by the
//! `kuna decompile`/`prdemo`/`timeit` CLI on the `decomp_dbg` subprocess
//! *before* it loads the file.  This mirrors the `relocobjects` loader-option
//! pattern (PR #37).  The companion `Architecture::analysis_i386_pie_plt` bool
//! exists only so the option is catalog-visible and `stage catalog`'s live
//! `current` field reflects it; the loader reads the env var, not the bool.
//!
//! Default **on** (loader fidelity, like the sibling x86-64/aarch64 decoders) —
//! `--option i386_pie_plt off` (⇒ env `off`) restores the byte-identical
//! pre-fix rendering (the non-PIE `FF 25 <abs32>` arm is never touched).

/// Environment variable that gates the i386-PIE `FF A3 <disp32>` PLT-stub decode
/// (read by `kuna-analysis::s1_loader::elf_plt::decode_i386`).  Absent or any
/// value other than the off-tokens ⇒ **enabled** (the default-on fidelity); set
/// to `off`/`0`/`false` ⇒ disabled (pre-fix behavior).
pub const I386_PIE_PLT_ENV: &str = "KUNA_I386_PIE_PLT";

/// Whether the i386-PIE PLT-stub decode is enabled for this process (the loader
/// gate).  Default **on**: only an explicit off-token in [`I386_PIE_PLT_ENV`]
/// disables it.  Consulted in `decode_i386` so that `--option i386_pie_plt off`
/// (bridged to the env var by the CLI) reproduces the pre-fix output.
pub fn i386_pie_plt_enabled() -> bool {
    match std::env::var(I386_PIE_PLT_ENV) {
        Ok(v) => !matches!(v.trim().to_ascii_lowercase().as_str(), "off" | "0" | "false"),
        Err(_) => true, // unset ⇒ default-on
    }
}

/// Bridge a `set_kuna_option("i386_pie_plt", val)` toggle to [`I386_PIE_PLT_ENV`]
/// so a later `load file` in the *same* process sees the choice.  In the usual
/// CLI flow the env var is set on the `decomp_dbg` subprocess directly (before
/// `load file`); this keeps the interactive `option i386_pie_plt …` *before*
/// `load file` path consistent too.
pub fn set_i386_pie_plt_env(on: bool) {
    std::env::set_var(I386_PIE_PLT_ENV, if on { "on" } else { "off" });
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The gate defaults on (unset env) and only the off-tokens disable it.
    /// Serialized by being a single test (env is process-global).
    #[test]
    fn env_gate_default_on_and_off_tokens() {
        std::env::remove_var(I386_PIE_PLT_ENV);
        assert!(i386_pie_plt_enabled(), "unset env ⇒ default on");

        for off in ["off", "0", "false", "OFF", "False"] {
            std::env::set_var(I386_PIE_PLT_ENV, off);
            assert!(!i386_pie_plt_enabled(), "`{off}` ⇒ disabled");
        }
        for on in ["on", "1", "true", ""] {
            std::env::set_var(I386_PIE_PLT_ENV, on);
            assert!(i386_pie_plt_enabled(), "`{on}` ⇒ enabled");
        }

        set_i386_pie_plt_env(false);
        assert!(!i386_pie_plt_enabled());
        set_i386_pie_plt_env(true);
        assert!(i386_pie_plt_enabled());

        std::env::remove_var(I386_PIE_PLT_ENV);
    }
}
