//! (kuna `libctypes`) The named-libc-aggregate gate — spell a libc/POSIX
//! aggregate pointer in the built-in prototype tables (`FILE *`, `stat *`,
//! `DIR *`, `option *`, …) instead of the width-stable `void *` those slots
//! carry today.
//!
//! The prototype tables' [`Ty`](../../../kuna_analysis/protos) vocabulary is
//! deliberately width-stable, so every aggregate pointer in them is a `void *`.
//! That is honest about the width and silent about the pointee: a `fopen`
//! result, a `fclose` argument and an `__uflow` parameter are all `void *`, and
//! nothing downstream can tell them apart. The table entry is the only place the
//! pointee is known — `int fclose(FILE *)` is not an inference, it is the
//! declaration — so this gate teaches the table to say so.
//!
//! ## Why this is an env-var bridge, not an `Architecture` flag
//!
//! The named shells are interned into the type factory by the analysis pass at
//! `load file`, which runs **upstream of every `option` command** and, in
//! `kuna decompile-all`, upstream of `apply_runtime_options`: an
//! `Architecture` bool read inside the pass would see the constructor default
//! whatever the user asked for. So the gate is a process environment variable
//! ([`LIBCTYPES_ENV`]), consulted by the pass and by the declared-name lookup,
//! exported by the `kuna decompile`/`decompile-all` CLI before the load and set
//! by the console's `option libctypes …` arm, exactly as the sibling
//! `dwarfstructs`/`typedepth` gates do. The companion
//! `Architecture::analysis_libctypes` bool keeps the option catalog-visible and
//! makes `phase catalog`'s live `current` field reflect it.
//!
//! With the gate off the pass does not run at all, so not one named shell is
//! interned and the output is byte-identical to the `void *` tables alone.
//!
//! Default **opaque**: the table's own declaration is the strongest evidence
//! about a libc pointee that exists, and `void *` discards it. The 675
//! datatest assertions are structurally untouched (no datatest loads a file);
//! the evidence for the default is the corpus sweep recorded in
//! `docs/features/libctypes/record.json`. `--option libctypes off` restores the
//! byte-identical `void *` tables.

/// Environment variable that gates the named libc aggregate types (read by
/// `kuna-analysis::analyzers::protos::kuna_libctypes`). Absent or any value
/// other than the off-tokens ⇒ **enabled**; set to `off`/`0`/`false` ⇒ disabled
/// (the `void *` tables).
pub const LIBCTYPES_ENV: &str = "KUNA_LIBCTYPES";

/// Whether the named libc aggregate types are enabled for this process.
/// Default **on**: only an explicit off-token in [`LIBCTYPES_ENV`] disables it.
pub fn libctypes_enabled() -> bool {
    match std::env::var(LIBCTYPES_ENV) {
        Ok(v) => !matches!(v.trim().to_ascii_lowercase().as_str(), "off" | "0" | "false"),
        Err(_) => true, // unset => default-on
    }
}

/// Bridge a `set_kuna_option("libctypes", val)` toggle to [`LIBCTYPES_ENV`] so a
/// later `load file` in the *same* process sees the choice (the interactive
/// `option libctypes opaque` before `load file` path; the CLI sets the env var
/// on the subprocess directly).
pub fn set_libctypes_env(on: bool) {
    std::env::set_var(LIBCTYPES_ENV, if on { "opaque" } else { "off" });
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The gate defaults on (unset env) and only the off-tokens disable it.
    /// Serialized by being a single test (env is process-global).
    #[test]
    fn libctypes_env_gate() {
        std::env::remove_var(LIBCTYPES_ENV);
        assert!(libctypes_enabled(), "unset => default-on");
        for off in ["off", "0", "false", "OFF", " off "] {
            std::env::set_var(LIBCTYPES_ENV, off);
            assert!(!libctypes_enabled(), "`{off}` should disable");
        }
        for on in ["opaque", "on", "1", "true", "OPAQUE", " opaque ", ""] {
            std::env::set_var(LIBCTYPES_ENV, on);
            assert!(libctypes_enabled(), "`{on}` should enable");
        }
        set_libctypes_env(false);
        assert!(!libctypes_enabled());
        set_libctypes_env(true);
        assert!(libctypes_enabled());
        std::env::remove_var(LIBCTYPES_ENV);
    }
}
