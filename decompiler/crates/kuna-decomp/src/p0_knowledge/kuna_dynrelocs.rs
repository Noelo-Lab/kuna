//! (kuna) The `dynrelocs` option gate — apply a **linked** ELF image's dynamic
//! relocations (`.rela.dyn`/`.rel.dyn`/`.rela.plt`) into the loaded image, and
//! mark the slots that `PT_GNU_RELRO` freezes as constant.
//!
//! ## The gap it closes
//!
//! kuna's ELF loader maps `PT_LOAD` bytes verbatim. For a PIE (`ET_DYN`) or a
//! relocatable-linked `ET_EXEC` that is not the run-time image: every GOT slot
//! filled by an `R_*_RELATIVE` / `GLOB_DAT` / `JUMP_SLOT` relocation reads back
//! as **0**, because the linker leaves the value to the dynamic loader. A call
//! through such a slot can never resolve, so kuna renders
//! `(*dat_e0dc8)(…)` where the callee is a named function in the very same
//! image. Zero is not a judgement call — it is a wrong byte.
//!
//! ## Why this is an env-var bridge, not an `Architecture` flag
//!
//! The relocations are applied inside `ObjectLoadImage::from_bytes`, i.e. inside
//! `load file` — **upstream of every per-function `option` command**, so an
//! `Architecture` bool would be read long after the image bytes were snapshotted.
//! The gate is therefore a process **environment variable**
//! ([`DYNRELOCS_ENV`]), consulted by the loader and set by the `kuna` CLI on the
//! `decomp_dbg` subprocess (and on its own process for the in-process
//! `decompile-all`/`functions` drivers) *before* the file is loaded. This mirrors
//! the `relocobjects` / `relocrebase` / `i386_pie_plt` loader-option pattern. The
//! companion `Architecture::analysis_dynrelocs` bool exists only so the option is
//! catalog-visible and `phase catalog`'s live `current` field reflects it.
//!
//! Default **on**: an unrelocated slot holds a value the run-time image never
//! holds, so `on` is a correctness fix. `--option dynrelocs off` (⇒ env `off`)
//! restores the byte-identical pre-fix behaviour.

/// Environment variable that gates the linked-image dynamic-relocation pass
/// (read by `kuna-analysis::loader::kuna_dynrelocs::resolve`). Absent or any
/// value other than the off-tokens ⇒ **enabled** (the default-on correctness
/// fix); set to `off`/`0`/`false` ⇒ disabled (pre-fix behavior).
pub const DYNRELOCS_ENV: &str = "KUNA_DYNRELOCS";

/// Whether the linked-image dynamic-relocation pass is enabled for this process.
/// Default **on**: only an explicit off-token in [`DYNRELOCS_ENV`] disables it.
pub fn dynrelocs_enabled() -> bool {
    match std::env::var(DYNRELOCS_ENV) {
        Ok(v) => !matches!(v.trim().to_ascii_lowercase().as_str(), "off" | "0" | "false"),
        Err(_) => true, // unset ⇒ default-on
    }
}

/// Bridge a `set_kuna_option("dynrelocs", val)` toggle to [`DYNRELOCS_ENV`] so a
/// later `load file` in the *same* process sees the choice (the interactive
/// `option dynrelocs …` before `load file` path, and the in-process CLI drivers).
pub fn set_dynrelocs_env(on: bool) {
    std::env::set_var(DYNRELOCS_ENV, if on { "on" } else { "off" });
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The gate defaults on (unset env) and only the off-tokens disable it.
    /// Serialized by being a single test (env is process-global).
    #[test]
    fn env_gate_default_on_and_off_tokens() {
        std::env::remove_var(DYNRELOCS_ENV);
        assert!(dynrelocs_enabled(), "unset env ⇒ default on");

        for off in ["off", "0", "false", "OFF", "False"] {
            std::env::set_var(DYNRELOCS_ENV, off);
            assert!(!dynrelocs_enabled(), "`{off}` ⇒ disabled");
        }
        for on in ["on", "1", "true", ""] {
            std::env::set_var(DYNRELOCS_ENV, on);
            assert!(dynrelocs_enabled(), "`{on}` ⇒ enabled");
        }

        set_dynrelocs_env(false);
        assert!(!dynrelocs_enabled());
        set_dynrelocs_env(true);
        assert!(dynrelocs_enabled());

        std::env::remove_var(DYNRELOCS_ENV);
    }
}
