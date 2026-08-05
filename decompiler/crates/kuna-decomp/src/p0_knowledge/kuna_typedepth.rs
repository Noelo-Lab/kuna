//! (kuna) The `typedepth` option gate — resolve a DWARF type DIE chain to its
//! full depth, guarded by upstream's per-DIE **cycle** counter instead of a fixed
//! hop budget.
//!
//! kuna's DWARF type mapper reduced Ghidra's `DWARFDataTypeImporter.trackRecursion`
//! (a per-DIE-offset re-entry counter that fires only on a genuine type cycle) to
//! a flat `MAX_TYPE_DEPTH = 3` counted over EVERY hop, transparent
//! `typedef`/`const`/`volatile`/`restrict` links included. Ordinary C
//! declarations exhaust that budget — `const char **`, `const size_t *`,
//! `char ***`, `char *const []` — and fall back to `void`, so a `-g` binary's
//! stack locals, globals and (three pointer hops in) prototypes rendered
//! `void *` where the debug info names a concrete type.
//!
//! ## Why this is an env-var bridge, not an `Architecture` flag
//!
//! The types are baked into the analysis facts by `DwarfPass` at `load file`,
//! which runs **upstream of every `option` command** — an `Architecture` bool
//! would be read after the mapping already happened. (The sibling `cppproto`
//! gate can defer to the analysis COMMIT because it selects between two fact
//! sets; this one changes how a single fact set is built.) So the gate is a
//! process environment variable ([`TYPEDEPTH_ENV`]), consulted by the mapper and
//! exported by the `kuna decompile`/`decompile-all` CLI before the load, exactly
//! as the `relocobjects`/`i386_pie_plt` loader gates do. The companion
//! `Architecture::analysis_typedepth` bool keeps the option catalog-visible and
//! makes `phase catalog`'s live `current` field reflect it.
//!
//! Default **on** (DIV-63): the depth budget was never the cycle guard it claimed
//! to be, and every truncation it produced was a concrete DWARF type replaced by
//! `void`. `--option typedepth off` restores the byte-identical pre-fix mapping.

/// Environment variable that selects the DWARF type mapper's recursion guard
/// (read by `kuna-analysis::analyzers::dwarf::kuna_typedepth::TypeWalk`). Absent
/// or any value other than the off-tokens ⇒ **enabled** (the cycle counter); set
/// to `off`/`0`/`false` ⇒ disabled (the legacy depth-3 budget).
pub const TYPEDEPTH_ENV: &str = "KUNA_TYPEDEPTH";

/// Whether full-depth DWARF type resolution is enabled for this process.
/// Default **on**: only an explicit off-token in [`TYPEDEPTH_ENV`] disables it.
pub fn typedepth_enabled() -> bool {
    match std::env::var(TYPEDEPTH_ENV) {
        Ok(v) => !matches!(v.trim().to_ascii_lowercase().as_str(), "off" | "0" | "false"),
        Err(_) => true, // unset ⇒ default-on
    }
}

/// Bridge a `set_kuna_option("typedepth", val)` toggle to [`TYPEDEPTH_ENV`] so a
/// later `load file` in the *same* process sees the choice (the interactive
/// `option typedepth … ` before `load file` path; the CLI sets the env var on the
/// subprocess directly).
pub fn set_typedepth_env(on: bool) {
    std::env::set_var(TYPEDEPTH_ENV, if on { "on" } else { "off" });
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The gate defaults on (unset env) and only the off-tokens disable it.
    /// Serialized by being a single test (env is process-global).
    #[test]
    fn typedepth_env_gate() {
        std::env::remove_var(TYPEDEPTH_ENV);
        assert!(typedepth_enabled(), "unset ⇒ default-on");
        for off in ["off", "0", "false", "OFF", " off "] {
            std::env::set_var(TYPEDEPTH_ENV, off);
            assert!(!typedepth_enabled(), "`{off}` should disable");
        }
        for on in ["on", "1", "true", "anything"] {
            std::env::set_var(TYPEDEPTH_ENV, on);
            assert!(typedepth_enabled(), "`{on}` should enable");
        }
        set_typedepth_env(false);
        assert!(!typedepth_enabled());
        set_typedepth_env(true);
        assert!(typedepth_enabled());
        std::env::remove_var(TYPEDEPTH_ENV);
    }
}
