//! (kuna) The `msvcfpconst` option gate — recover an MSVC `__real@` floating-point
//! constant COMDAT from its mangled symbol name and let the engine fold it.
//!
//! ## The gap it closes
//!
//! MSVC never encodes a floating-point literal into the instruction stream: x87
//! and SSE both load one from memory, and the compiler emits each literal as a
//! COMDAT whose *name spells the value* (`__real@8@3ffec90fdaa22168c000` is
//! pi/4). COMDAT folding then keeps the definition in exactly one translation
//! unit, so in every other object the symbol is **undefined** — no section, no
//! bytes. kuna binds it to a synthetic extern slot and the value is simply gone,
//! leaving `v6 = (... * dat_402020 + dat_402040) * dat_400ae0` with every operand
//! an opaque address. The *defined* half is present but unfolded, because folding
//! a read-only global is gated by the program-wide `option readonly`.
//!
//! On, the value is decoded from the name, materialised at the extern slot, and
//! both halves are added to the loader's "these ranges fold anyway" list
//! (`ObjectLoadImage::dynreloc_const`) — the same narrow exception `dynrelocs`
//! uses, and for the same reason: the datum is known by construction rather than
//! by policy. A defined COMDAT's bytes are cross-checked against its own name
//! before its range is admitted.
//!
//! ## Why this is an env-var bridge, not an `Architecture` flag
//!
//! The bytes are materialised inside `ObjectLoadImage::from_relocatable`, i.e.
//! inside `load file` — **upstream of every per-function `option` command**, so
//! an `Architecture` bool would be read long after the image was snapshotted. The
//! gate is therefore a process **environment variable** ([`MSVCFPCONST_ENV`]),
//! consulted by the loader and set by the `kuna` CLI on the `decomp_dbg`
//! subprocess (and on its own process for the in-process `decompile-all` /
//! `functions` drivers) *before* the file is loaded. This mirrors the
//! `relocobjects` / `relocrebase` / `dynrelocs` / `symbolnamerepair` loader-option
//! pattern. The companion `Architecture::analysis_msvcfpconst` bool exists only
//! so the option is catalog-visible and `phase catalog`'s live `current` field
//! reflects it.
//!
//! Default **on** (DIV-96): the mangled name IS the datum, so an undefined slot
//! with no bytes is missing information rather than expressing a judgement.
//! `--option msvcfpconst off` (=> env `off`) restores the byte-identical
//! pre-feature behaviour.

/// Environment variable that gates MSVC `__real@` constant recovery (read by
/// `kuna-analysis::loader::kuna_msvcfpconst::plan`). Absent or any value other
/// than the off-tokens => **enabled**; set to `off`/`0`/`false` => disabled.
pub const MSVCFPCONST_ENV: &str = "KUNA_MSVCFPCONST";

/// Whether MSVC `__real@` constant recovery is enabled for this process.
/// Default **on**: only an explicit off-token in [`MSVCFPCONST_ENV`] disables it.
pub fn msvcfpconst_enabled() -> bool {
    match std::env::var(MSVCFPCONST_ENV) {
        Ok(v) => !matches!(v.trim().to_ascii_lowercase().as_str(), "off" | "0" | "false"),
        Err(_) => true, // unset => default-on
    }
}

/// Bridge a `set_kuna_option("msvcfpconst", val)` toggle to [`MSVCFPCONST_ENV`] so
/// a later `load file` in the *same* process sees the choice (the interactive
/// `option msvcfpconst …` before `load file` path, and the in-process CLI
/// drivers).
pub fn set_msvcfpconst_env(on: bool) {
    std::env::set_var(MSVCFPCONST_ENV, if on { "on" } else { "off" });
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The gate defaults on (unset env) and only the off-tokens disable it.
    /// Serialized by being a single test (env is process-global).
    #[test]
    fn env_gate_default_on_and_off_tokens() {
        std::env::remove_var(MSVCFPCONST_ENV);
        assert!(msvcfpconst_enabled(), "unset env => default on");

        for off in ["off", "0", "false", "OFF", "False"] {
            std::env::set_var(MSVCFPCONST_ENV, off);
            assert!(!msvcfpconst_enabled(), "`{off}` => disabled");
        }
        for on in ["on", "1", "true", ""] {
            std::env::set_var(MSVCFPCONST_ENV, on);
            assert!(msvcfpconst_enabled(), "`{on}` => enabled");
        }

        set_msvcfpconst_env(false);
        assert!(!msvcfpconst_enabled());
        set_msvcfpconst_env(true);
        assert!(msvcfpconst_enabled());

        std::env::remove_var(MSVCFPCONST_ENV);
    }
}
