//! P0 -- decompiler MODES: named presets over the runtime option surface.
//!
//! A *mode* is a named, ordered list of `(option, value)` overrides layered on
//! top of the shipped defaults, applied *before* any explicit user `--option`
//! (last-write-wins, so a user `--option` always overrides the mode). A mode is
//! **not** a `[[settable]]` row -- it references existing option names and lives
//! entirely in this table, so it never touches `phases.toml` / `SETTABLE_TABLE`
//! / the catalog count+tier gates. With no mode selected, behaviour is
//! byte-identical to the shipped defaults.
//!
//! Two modes ship:
//!   - **`reliable`** -- the shipped, well-tested defaults (a stable, named
//!     alias with an empty override list). Anchors the "give me the safe output"
//!     product surface and future-proofs the preset if defaults later drift.
//!   - **`aggressive`** -- turn on every off-by-default quality/recovery/analysis
//!     pass for the most-recovered (and slowest, most speculative) output.
//!
//! The mode is applied through `Architecture::apply_mode` (which fans out to
//! `set_kuna_option`), reachable from `kuna decompile`, `kuna decompile-all`,
//! and the interactive console `mode <name>` command.

/// A named preset over the option surface.
pub struct Mode {
    /// The mode token (`reliable`, `aggressive`).
    pub name: &'static str,
    /// One-line human/LLM description.
    pub summary: &'static str,
    /// Ordered `(option, value)` overrides. Applied in order; a later
    /// `set_kuna_option` (another override, or a user `--option`) wins. An empty
    /// slice = the shipped defaults (a no-op alias).
    pub overrides: &'static [(&'static str, &'static str)],
}

/// The two shipped modes.
pub const MODE_TABLE: &[Mode] = &[
    Mode {
        name: "reliable",
        summary: "The shipped, well-tested defaults -- the safe, stable baseline \
                  (no extra transforms). Byte-identical to running with no mode.",
        overrides: RELIABLE_OVERRIDES,
    },
    Mode {
        name: "aggressive",
        summary: "Maximum recovery: turn on every off-by-default quality, \
                  structuring, and analysis pass. Slower and more speculative \
                  (may over-recover); best for readability and the benchmark \
                  ceiling, not for guaranteed faithfulness.",
        overrides: AGGRESSIVE_OVERRIDES,
    },
];

/// `reliable` = the shipped defaults. Deliberately empty: pinning any option
/// here (e.g. `listing off`) would *change* behaviour vs the defaults -- in
/// particular `decompile-all` auto-enables the Listing (DIV-15), so an explicit
/// `listing off` would regress it. An empty list leaves every default untouched.
const RELIABLE_OVERRIDES: &[(&str, &str)] = &[];

/// `aggressive` = every off-by-default option flipped ON, **except**
/// `v850indirectbranch`.
///
/// All 21 default-off options are safe to blanket-enable except that one: unlike
/// the format-gated no-ops (`rtti`/`pdb`=PE, `objc`/`macho-arm64e`=Mach-O,
/// `sparcstructret`=SPARC `unimp`-trap idiom -- all inert off their target),
/// `v850indirectbranch`'s predicate matches *any* register-indirect `CALLIND`
/// (`kuna_is_v850_indirect_jmp`, `p2_lift/kuna_v850indbranch.rs`), so on x86-64 /
/// ARM it would reclassify every `call reg` into an indirect branch -- corruption,
/// not recovery. It stays a manual per-target opt-in (`--option
/// v850indirectbranch on`) even under `--mode aggressive`.
const AGGRESSIVE_OVERRIDES: &[(&str, &str)] = &[
    // transform-tier default-off recovery/structuring passes.
    ("switchmodbound", "on"),
    ("switchguardbound", "on"), // speed-expensive
    ("unrolledguard", "on"),    // speed-expensive
    ("stackalias", "on"),
    ("sparcstructret", "on"), // SPARC-idiom-gated; no-op off-SPARC
    ("regionedgeorder", "on"),
    ("returndup", "on"),
    // analysis-tier default-off discovery/markup passes. `listing` is the master
    // gate that enables the Listing-consuming passes (fid/aif/discovered-noreturn).
    ("listing", "on"),
    ("eh_frame_full", "on"),
    ("funcstart_patterns", "on"),
    ("dwarf_lines", "on"),
    ("addrtable", "on"),
    ("operand_refs", "on"),
    ("formatstring", "on"),
    ("fid", "on"),
    ("rtti", "on"),          // PE-only; no-op off-PE
    ("aif", "on"),           // speculative gap-walk ("may create bad code")
    ("objc", "on"),          // Mach-O-only; no-op off-Mach-O
    ("pdb", "on"),           // PE-only; no-op off-PE
    ("macho-arm64e", "on"),  // Mach-O arm64e-only; no-op elsewhere
];

/// The override list for `name`, or `None` if `name` is not a known mode.
pub fn mode_overrides(name: &str) -> Option<&'static [(&'static str, &'static str)]> {
    MODE_TABLE.iter().find(|m| m.name == name).map(|m| m.overrides)
}

/// The shipped mode names, in table order.
pub fn mode_names() -> impl Iterator<Item = &'static str> {
    MODE_TABLE.iter().map(|m| m.name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::options::KUNA_OPTION_NAMES;

    #[test]
    fn every_override_names_a_real_option() {
        for m in MODE_TABLE {
            for (opt, _val) in m.overrides {
                assert!(
                    KUNA_OPTION_NAMES.contains(opt),
                    "mode `{}` references unknown option `{}`",
                    m.name,
                    opt
                );
            }
        }
    }

    #[test]
    fn reliable_is_a_no_op_alias() {
        assert!(mode_overrides("reliable").unwrap().is_empty());
    }

    #[test]
    fn aggressive_excludes_v850_but_includes_the_rest() {
        let agg = mode_overrides("aggressive").unwrap();
        // The one intentional exclusion: it corrupts non-V850 targets.
        assert!(
            !agg.iter().any(|(o, _)| *o == "v850indirectbranch"),
            "aggressive must NOT enable v850indirectbranch (reclassifies x86-64 call reg)"
        );
        // Representative members that MUST be on.
        for want in ["listing", "aif", "switchguardbound", "returndup", "sparcstructret"] {
            assert!(
                agg.iter().any(|(o, v)| *o == want && *v == "on"),
                "aggressive must enable `{want}`"
            );
        }
        // Every aggressive override turns its option on.
        assert!(agg.iter().all(|(_, v)| *v == "on"));
    }

    #[test]
    fn unknown_mode_is_none() {
        assert!(mode_overrides("turbo").is_none());
        assert!(mode_overrides("").is_none());
    }

    #[test]
    fn mode_names_lists_both() {
        let names: Vec<_> = mode_names().collect();
        assert_eq!(names, vec!["reliable", "aggressive"]);
    }
}
