//! (kuna `gotypes`) The Go runtime-type harvest gate — walk a Go binary's own
//! `moduledata → typelinks → _type` metadata and intern NAMED struct shells
//! (`main.Point`, `os.File`, …) into the type factory, phase 1 of the
//! `docs/multilang.md` W3 Go provider.
//!
//! A stripped Go binary carries no DWARF, but the runtime needs its type graph
//! for reflection and interface dispatch, so the linker bakes it in. Phase 1
//! recovers the names and struct layouts; it deliberately does NOT yet bind
//! typed globals, resolve interface/itab methods, name generic instantiations
//! beyond their raw strings, or open map/chan internals — those are later
//! consumers of these shells.
//!
//! ## Why this is an env-var bridge, not an `Architecture` flag
//!
//! The shells are interned by the `gotypes` analysis pass at `load file`,
//! upstream of every `option` command — the `dwarfstructs` situation exactly,
//! and the same answer: the gate is a process environment variable
//! ([`GOTYPES_ENV`]), consulted by the pass and exported by the CLI before the
//! load. The companion `Architecture::analysis_gotypes` bool keeps the option
//! catalog-visible and the live `current` honest.
//!
//! Default **on**: the pass is registered only for a Go-detected image (the
//! `gopclntab` precedent), so it is a structural no-op on every other binary,
//! and on a Go binary the shells are pure added knowledge — nothing reads them
//! destructively. `--option gotypes off` restores the shell-free factory.

/// Environment variable that gates the Go runtime-type harvest (read by
/// `kuna-analysis::analyzers::gotypes`). Absent or any value other than the
/// off-tokens ⇒ **enabled**; `off`/`0`/`false` ⇒ disabled.
pub const GOTYPES_ENV: &str = "KUNA_GOTYPES";

/// Whether the Go runtime-type harvest is enabled for this process.
pub fn gotypes_enabled() -> bool {
    crate::options::env_toggle(GOTYPES_ENV, true)
}

/// Bridge a `set_kuna_option("gotypes", val)` toggle to [`GOTYPES_ENV`] so a
/// later `load file` in the same process sees the choice.
pub fn set_gotypes_env(on: bool) {
    std::env::set_var(GOTYPES_ENV, if on { "on" } else { "off" });
}

/// The most typelink entries one image's walk will process. Real Go binaries
/// carry 1,500–4,000 (`vet`, the 6 MB tool, has ~3,400); the ceiling only
/// exists so a corrupt length cannot make the load path spin. Generous rather
/// than tight: every skipped entry is a named type the user never sees.
pub const GOTYPES_MAX_TYPELINKS: usize = 20_000;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn env_gate_default_on_and_off_tokens() {
        std::env::remove_var(GOTYPES_ENV);
        assert!(gotypes_enabled(), "unset env => default on");
        for off in ["off", "0", "false"] {
            std::env::set_var(GOTYPES_ENV, off);
            assert!(!gotypes_enabled(), "`{off}` => disabled");
        }
        for on in ["on", "1", "true", ""] {
            std::env::set_var(GOTYPES_ENV, on);
            assert!(gotypes_enabled(), "`{on}` => enabled");
        }
        std::env::remove_var(GOTYPES_ENV);
    }
}
