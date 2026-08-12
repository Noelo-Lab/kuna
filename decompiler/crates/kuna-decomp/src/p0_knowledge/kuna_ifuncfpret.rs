//! (kuna) The `ifuncfpret` option gate — name the x86-64 `.plt.sec`/`.iplt`
//! **IFUNC** (`R_X86_64_IRELATIVE`) PLT stubs so a tail `jmp` to one is recovered
//! as a tail call (`tailcalljump`) and the callee's return type flows out.
//!
//! ## Why this is an env-var bridge, not an `Architecture` flag
//!
//! Like [`i386_pie_plt`](crate::kuna_i386_pie_plt), this is a **loader-tier**
//! change: the stub naming lives in `kuna-analysis`
//! (`loader/elf_plt.rs::resolve_plt_imports`), which is baked into the
//! `ObjectLoadImage` at `load file` — **upstream of every per-function `option`
//! command**. An `Architecture` bool would be read too late. So the gate is a
//! process environment variable ([`IFUNCFPRET_ENV`]), consulted by the loader at
//! decode time and set by the CLI on the `decomp_dbg` subprocess before it loads
//! the file. The companion `Architecture::analysis_ifuncfpret` bool exists only
//! so the option is catalog-visible and `phase catalog`'s live `current` field
//! reflects it.
//!
//! ## Why default **off**
//!
//! An IFUNC IRELATIVE slot carries no symbol, so a wrapper that tail-jumps to its
//! stub (a glibc `log`/`memcpy`/`strlen` dispatcher) flows into the stub and
//! renders `(*dat_...)(...)`, losing the callee return type — but the stub the
//! naming would create is a *synthetic* `ifunc.<resolver>` entry with no real
//! prototype, so its downstream value (return-type recovery) depends on the rest
//! of the `ifuncfpret` chain. Kept opt-in until that chain is validated; when on,
//! the tail jump becomes a named tail call.

/// Environment variable that gates the x86-64 IFUNC PLT-stub naming (read by
/// `kuna-analysis::loader::elf_plt::resolve_plt_imports`). Absent or an off-token
/// ⇒ **disabled** (default-off, opt-in); any on-token ⇒ enabled.
pub const IFUNCFPRET_ENV: &str = "KUNA_IFUNCFPRET";

/// Whether IFUNC PLT-stub naming is enabled for this process (the loader gate).
/// Default **off**: only an explicit on-token in [`IFUNCFPRET_ENV`] enables it.
pub fn ifuncfpret_enabled() -> bool {
    match std::env::var(IFUNCFPRET_ENV) {
        Ok(v) => matches!(v.trim().to_ascii_lowercase().as_str(), "on" | "1" | "true" | ""),
        Err(_) => false, // unset ⇒ default-off (opt-in)
    }
}

/// Bridge a `set_kuna_option("ifuncfpret", val)` toggle to [`IFUNCFPRET_ENV`] so a
/// later `load file` in the *same* process sees the choice.
pub fn set_ifuncfpret_env(on: bool) {
    std::env::set_var(IFUNCFPRET_ENV, if on { "on" } else { "off" });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn env_gate_default_off_and_on_tokens() {
        std::env::remove_var(IFUNCFPRET_ENV);
        assert!(!ifuncfpret_enabled(), "unset env ⇒ default off");

        for on in ["on", "1", "true", "ON", "True", ""] {
            std::env::set_var(IFUNCFPRET_ENV, on);
            assert!(ifuncfpret_enabled(), "`{on}` ⇒ enabled");
        }
        for off in ["off", "0", "false"] {
            std::env::set_var(IFUNCFPRET_ENV, off);
            assert!(!ifuncfpret_enabled(), "`{off}` ⇒ disabled");
        }

        set_ifuncfpret_env(true);
        assert!(ifuncfpret_enabled());
        set_ifuncfpret_env(false);
        assert!(!ifuncfpret_enabled());

        std::env::remove_var(IFUNCFPRET_ENV);
    }
}
