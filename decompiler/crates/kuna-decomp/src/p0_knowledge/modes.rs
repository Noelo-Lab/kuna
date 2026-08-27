//! P0 -- decompiler MODES: named presets over the runtime option surface.
//!
//! A concrete *mode* is a named, ordered list of `(option, value)` overrides
//! layered on top of the shipped defaults, applied *before* any explicit user
//! `--option` (last-write-wins, so a user `--option` always overrides the
//! mode). A mode is **not** a `[[settable]]` row -- it references existing
//! option names and lives entirely in this table, so it never touches
//! `phases.toml` / `SETTABLE_TABLE` / the catalog count+tier gates. File
//! frontends resolve an omitted mode as the size-driven `auto` policy.
//!
//! Four modes ship:
//!   - **`auto`** -- a frontend policy that selects a concrete preset from the
//!     input binary's file size. It is not itself applicable to an
//!     [`Architecture`](crate::architecture::Architecture), which has no input
//!     file metadata.
//!   - **`reliable`** -- the shipped, well-tested defaults (a stable, named
//!     alias with an empty override list). Anchors the "give me the safe output"
//!     product surface and future-proofs the preset if defaults later drift.
//!   - **`aggressive`** -- turn on every off-by-default quality/recovery/analysis
//!     pass for the most-recovered (and slowest, most speculative) output. Two
//!     options are excluded (see [`AGGRESSIVE_OVERRIDES`]).
//!   - **`fast`** -- replace exhaustive discovery with a rooted recursive pass
//!     plus conservative pointer validation while retaining per-function transforms.
//!
//! Concrete presets are applied through `Architecture::apply_mode` (which fans
//! out to `set_kuna_option`). File frontends first resolve `auto` from binary
//! metadata; the interactive console has no file-size policy and accepts only
//! concrete presets.

/// A named preset over the option surface.
pub struct Mode {
    /// The mode token (`auto`, `reliable`, `aggressive`, `fast`).
    pub name: &'static str,
    /// One-line human/LLM description.
    pub summary: &'static str,
    /// Whether this is a frontend policy that must first select a concrete
    /// preset using input metadata.
    pub automatic: bool,
    /// Ordered `(option, value)` overrides. Applied in order; a later
    /// `set_kuna_option` (another override, or a user `--option`) wins. An empty
    /// slice means either the shipped defaults (`reliable`) or a dynamic policy
    /// with no direct overrides (`auto`); consult [`Self::automatic`].
    pub overrides: &'static [(&'static str, &'static str)],
}

/// First file size at which `auto` stops selecting `aggressive`.
pub const AUTO_RELIABLE_MIN_BYTES: u64 = 500 * 1024;

/// First file size at which `auto` selects `fast`.
pub const AUTO_FAST_MIN_BYTES: u64 = 2 * 1024 * 1024;

/// The four shipped modes.
pub const MODE_TABLE: &[Mode] = &[
    Mode {
        name: "auto",
        summary: "Select aggressive below 500 KiB, reliable from 500 KiB up to \
                  2 MiB, and fast at 2 MiB or larger, using the input file size.",
        automatic: true,
        overrides: &[],
    },
    Mode {
        name: "reliable",
        summary: "The shipped, well-tested defaults -- the safe, stable baseline \
                  with no extra transform overrides.",
        automatic: false,
        overrides: RELIABLE_OVERRIDES,
    },
    Mode {
        name: "aggressive",
        summary: "Maximum recovery: turn on every off-by-default quality, \
                  structuring, and analysis pass except v850indirectbranch and \
                  dwarf_lines. Slower and more speculative (may over-recover); \
                  best for readability and the benchmark ceiling, not for \
                  guaranteed faithfulness.",
        automatic: false,
        overrides: AGGRESSIVE_OVERRIDES,
    },
    Mode {
        name: "fast",
        summary: "Speed-first whole-binary analysis: replace exhaustive Listing, \
                  prologue-pattern, and AIF discovery with rooted recursive \
                  function discovery plus conservative pointer validation.",
        automatic: false,
        overrides: FAST_OVERRIDES,
    },
];

/// `reliable` = the shipped defaults. Deliberately empty: pinning any option
/// here (e.g. `listing off`) would *change* behaviour vs the defaults -- in
/// particular `decompile` and `decompile-all` auto-enable the Listing, so an
/// explicit `listing off` would change them. An empty list leaves every surface
/// default untouched.
const RELIABLE_OVERRIDES: &[(&str, &str)] = &[];

/// `aggressive` = every off-by-default option flipped ON, **except**
/// `v850indirectbranch` and `dwarf_lines`. `returndup` is listed even though
/// DIV-54 made it a shipped default: the preset owns the option outright, so a
/// user `--option returndup off` earlier on the command line is re-enabled by
/// the preset and the contract survives any later default drift.
///
/// The default-off options are safe to blanket-enable except those two. Unlike
/// the format-gated no-ops (`rtti`/`pdb`=PE, `itaniumrtti`=ELF-with-RTTI,
/// `objc`/`macho-arm64e`=Mach-O, `sparcstructret`=SPARC `unimp`-trap idiom -- all
/// inert off their target):
///
///   - `v850indirectbranch`'s predicate matches *any* register-indirect `CALLIND`
///     (`kuna_is_v850_indirect_jmp`, `p2_lift/kuna_v850indbranch.rs`), so on
///     x86-64 / ARM it would reclassify every `call reg` into an indirect branch
///     -- corruption, not recovery.
///   - `dwarf_lines` recovers nothing. It annotates every instruction with its
///     `.debug_line` `file:line` and those comments survive into the C body, so
///     on any binary built with `-g` it buries the code under interleaved
///     `/* src.c:NNN */` lines (`auto` picks `aggressive` under 500 KiB, which
///     made that the *default* rendering for small debug binaries).
///   - `formatstring` costs a **second full decompile** of any caller whose
///     printf/scanf call sites yield a varargs override, and those are the
///     expensive functions, so on a whole binary it is far over the 5% speed
///     budget: `decompile-all` measured +77.5% (cronie `crontab`), +55.1%
///     (gnutls `psktool`), +43.7% (`gzip`). Only ~5-15% of functions re-decompile;
///     they simply carry most of the time. Held to a per-run opt-in by standing
///     requirement 4 (DIV-66) -- the recovery it buys is real (decbench C
///     `type_match` perfects 80 -> 88 of 1,133) and `--option formatstring on`
///     still gets it, on every surface.
///
/// All three stay manual per-run opt-ins (`--option v850indirectbranch on`,
/// `--option dwarf_lines on`, `--option formatstring on`) even under
/// `--mode aggressive`.
const AGGRESSIVE_OVERRIDES: &[(&str, &str)] = &[
    // transform-tier default-off recovery/structuring passes.
    ("switchmodbound", "on"),
    ("switchguardbound", "on"), // speed-expensive
    ("unrolledguard", "on"),    // speed-expensive
    ("stackalias", "on"),
    ("sparcstructret", "on"), // SPARC-idiom-gated; no-op off-SPARC
    ("regionedgeorder", "on"),
    ("returndup", "on"), // default-ON since DIV-54; kept so the preset owns it
    ("orchain", "on"),   // the returndup gate DIV-69 measured; must follow returndup
    ("iteexpr", "on"), // angr-style ?: recovery over computed-expression arms (readability)
    // (kuna) Valid per-architecture C type spelling. The shipped catalog default
    // is OFF because 42 XML datatest assertions pin the Ghidra spellings, and the
    // parity harness applies no mode -- so preset membership is what makes valid C
    // the default RENDERING for every real binary without touching the corpus.
    ("ctypes", "on"),
    // analysis-tier default-off discovery/markup passes. `listing` is the master
    // gate that enables the Listing-consuming passes (fid/aif/discovered-noreturn).
    ("listing", "on"),
    ("fast_funcdisc", "on"),
    ("eh_frame_full", "on"),
    ("funcstart_patterns", "on"),
    ("addrtable", "on"),
    ("operand_refs", "on"),
    ("fid", "on"),
    ("rtti", "on"),          // PE-only; no-op off-PE
    ("itaniumrtti", "on"),   // ELF-only, and inert without __cxxabiv1 typeinfo relocs
    ("aif", "on"),           // speculative gap-walk ("may create bad code")
    // (kuna, GH-299) The aligned gap cursor. Every number in its catalog row was
    // measured under this preset, and it is a net win in BOTH directions there
    // (mid-body entries -38.9%, recall +344 over 110 stripped non-x86-64 binaries),
    // but it misses the acceptance bar pre-registered for becoming the shipped
    // default, so preset membership is how it reaches the default path.
    ("aifstrict", "on"),
    // (kuna, DIV-93) The four-step ARM entry-discovery sequence (#248 / #255 /
    // #259 / #278). `cortexmvectors`, `ptrentry` and `tailcallentry` carry an
    // explicit `object::Architecture::Arm` early return; `poolentry` has no arch
    // gate but is ARM-only IN EFFECT, because it keys on PC-relative literal
    // pools, which x86-64 (RIP-relative loads target .rodata, not .text
    // interstices) and i386 (no PC-relative addressing) do not produce. Either
    // way the no-op is proven rather than assumed: over 90 x86-64 decbench twins
    // and the 12 i386 PE images inside the ARM corpus the entry sets are
    // IDENTICAL, and emitted C over 8 x86-64 binaries is byte-identical. They
    // COMPOSE, so preset
    // membership was one joint evaluation, measured over the 110 stripped
    // non-x86-64 decbench twins (50,724 symbol-table function starts):
    // recall 44,957 -> 47,330 (88.63% -> 93.31%, +2,373) while mid-body false
    // entries FALL 8,333 -> 7,117. 98.8% of the 2,402 added entries are real
    // function starts and ZERO ground-truth entries are lost -- `poolentry`
    // retires 1,217 phantom entries, none of them ground truth.
    // `ptrentry`/`tailcallentry`/`poolentry` are Listing consumers and are inert
    // without `listing` + `aif`; the preset supplies both, which is what makes
    // the flip deliver anything at all (these are flag writes, so their position
    // in this list carries no ordering semantics -- the dependency is at run time).
    ("cortexmvectors", "on"),
    ("ptrentry", "on"),
    ("tailcallentry", "on"),
    ("poolentry", "on"),
    ("objc", "on"),          // Mach-O-only; no-op off-Mach-O
    ("pdb", "on"),           // PE-only; no-op off-PE
    ("macho-arm64e", "on"),  // Mach-O arm64e-only; no-op elsewhere
];

/// `fast` = avoid the three exhaustive program-wide discovery paths that dominate
/// large-binary latency, while retaining rooted recursive discovery and
/// conservative pointer validation for a usable whole-binary project.
const FAST_OVERRIDES: &[(&str, &str)] = &[
    ("listing", "off"),
    ("funcstart_patterns", "off"),
    ("aif", "off"),
    ("fast_funcdisc", "on"),
];

/// Select the concrete preset for `size_bytes` under the `auto` policy.
pub fn auto_mode_for_size(size_bytes: u64) -> &'static str {
    if size_bytes < AUTO_RELIABLE_MIN_BYTES {
        "aggressive"
    } else if size_bytes < AUTO_FAST_MIN_BYTES {
        "reliable"
    } else {
        "fast"
    }
}

/// Resolve an omitted or named frontend mode to a concrete preset.
///
/// Omission selects `auto`. Explicit concrete presets ignore `size_bytes`.
/// Returns `None` for an unknown name.
pub fn resolve_mode_for_size(
    requested: Option<&str>,
    size_bytes: u64,
) -> Option<&'static str> {
    let requested = requested.unwrap_or("auto");
    let mode = MODE_TABLE.iter().find(|mode| mode.name == requested)?;
    if mode.automatic {
        Some(auto_mode_for_size(size_bytes))
    } else {
        Some(mode.name)
    }
}

/// Whether `name` identifies a frontend policy rather than a static preset.
pub fn mode_is_automatic(name: &str) -> bool {
    MODE_TABLE.iter().any(|m| m.name == name && m.automatic)
}

/// The override list for a static `name`, or `None` if `name` is unknown or
/// automatic. Frontends must resolve `auto` with [`resolve_mode_for_size`]
/// first.
pub fn mode_overrides(name: &str) -> Option<&'static [(&'static str, &'static str)]> {
    MODE_TABLE.iter().find(|m| m.name == name && !m.automatic).map(|m| m.overrides)
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
    fn auto_uses_exact_binary_size_boundaries() {
        assert_eq!(auto_mode_for_size(0), "aggressive");
        assert_eq!(auto_mode_for_size(AUTO_RELIABLE_MIN_BYTES - 1), "aggressive");
        assert_eq!(auto_mode_for_size(AUTO_RELIABLE_MIN_BYTES), "reliable");
        assert_eq!(auto_mode_for_size(AUTO_FAST_MIN_BYTES - 1), "reliable");
        assert_eq!(auto_mode_for_size(AUTO_FAST_MIN_BYTES), "fast");
        assert_eq!(auto_mode_for_size(u64::MAX), "fast");
    }

    #[test]
    fn frontend_resolution_defaults_to_auto_and_preserves_concrete_modes() {
        assert_eq!(resolve_mode_for_size(None, 0), Some("aggressive"));
        assert_eq!(
            resolve_mode_for_size(Some("auto"), AUTO_RELIABLE_MIN_BYTES),
            Some("reliable")
        );
        assert_eq!(resolve_mode_for_size(Some("fast"), 0), Some("fast"));
        assert_eq!(
            resolve_mode_for_size(Some("aggressive"), u64::MAX),
            Some("aggressive")
        );
        assert_eq!(resolve_mode_for_size(Some("turbo"), 0), None);
    }

    #[test]
    fn auto_is_dynamic_not_a_static_override_list() {
        assert!(mode_is_automatic("auto"));
        assert!(!mode_is_automatic("reliable"));
        assert!(mode_overrides("auto").is_none());
    }

    #[test]
    fn aggressive_excludes_v850_and_dwarf_lines_but_includes_the_rest() {
        let agg = mode_overrides("aggressive").unwrap();
        // The two intentional exclusions: corruption off-V850, and source-line
        // comment noise over every statement of a `-g` binary.
        assert!(
            !agg.iter().any(|(o, _)| *o == "v850indirectbranch"),
            "aggressive must NOT enable v850indirectbranch (reclassifies x86-64 call reg)"
        );
        assert!(
            !agg.iter().any(|(o, _)| *o == "dwarf_lines"),
            "aggressive must NOT enable dwarf_lines (buries `-g` output in /* src.c:N */)"
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

    /// The preset's headline contract, enforced rather than sampled: **every**
    /// default-off option is in `aggressive` except the two documented exclusions.
    ///
    /// The sampled test above passes whether or not a newly added default-off
    /// option was ever put in the list, and a default-off option outside the list
    /// is *unreachable in practice* — `auto` selects `aggressive` for anything
    /// under 500 KiB, which is the default path for essentially every invocation,
    /// so such an option never runs for `decompile-all`, the web front-end, or the
    /// benchmark. That is exactly how `itaniumrtti` was first shipped inert. This
    /// test makes the omission a build failure instead of a silent no-feature.
    #[test]
    fn aggressive_carries_every_default_off_option() {
        use crate::kuna_phases::{kuna_num_settables, kuna_settable_by_index};

        /// The DELIBERATE exclusions, each with its reason recorded on
        /// `AGGRESSIVE_OVERRIDES`: `v850indirectbranch` corrupts non-V850 targets,
        /// `dwarf_lines` buries a `-g` binary's C under per-instruction
        /// `/* src.c:N */` comments, and `formatstring` costs a second full
        /// decompile of every caller that yields a varargs override -- measured
        /// +43.7% to +77.5% on whole-binary `decompile-all`, far over the 5% speed
        /// budget, so standing requirement 4 holds it to an opt-in (DIV-66).
        /// `ifuncfpret` is Stage A of the IFUNC FP-return chain: on its own it only
        /// renames x86-64 IFUNC stubs to synthetic `ifunc_<resolver>` names and
        /// recovers a tail call to them -- it does NOT yet fix the `xmm0`-return the
        /// feature exists for (that needs the Ghidra-divergent Stage B), so it is an
        /// explicit opt-in until the chain is complete rather than a rendering default.
        /// `aifcorroborate` (GH-313) is upstream's second AIF fingerprint test, and it
        /// was MEASURED OUT of the preset rather than left unevaluated: over the same
        /// 110 stripped non-x86-64 binaries `aifstrict` was measured on, it cuts
        /// mid-body entries 6,728 -> 4,653 but costs 850 of 44,957 recovered
        /// functions, raises recall on zero of the 110 images, and takes 84 / 141 real
        /// functions off the two u-boot A32 images DIV-20 exists for.
        const EXCLUDED_ON_PURPOSE: &[&str] =
            &["v850indirectbranch", "dwarf_lines", "formatstring", "ifuncfpret",
              "aifcorroborate"];

        /// Default-off options that predate this test and are **not** in the preset,
        /// i.e. are currently unreachable on the default path. Each is a genuine open
        /// question rather than a decision: preset membership changes emitted C on
        /// the binaries the option fires for, so each needs its own sweep, speed
        /// measurement and PR (`paramcopyhoist`'s own catalog row already records
        /// that its flip regresses a stage assertion and was deferred). Listed here
        /// so the invariant can be enforced for *new* options without silently
        /// flipping the existing ones; shrinking this list is the follow-up.
        ///
        /// The four ARM entry options (`cortexmvectors`, `ptrentry`, `tailcallentry`,
        /// `poolentry`) came OFF this list in DIV-93. The joint evaluation this entry
        /// asked for was run over the 110 stripped non-x86-64 decbench twins and all
        /// four passed it, so they are in `AGGRESSIVE_OVERRIDES` above.
        const UNEVALUATED: &[&str] = &[
            "paramcopyhoist",
            // `guardarm` / `loopcondhoist` land as opt-ins with their whole-O0
            // bidirectional sweep already recorded in their catalog rows; preset
            // membership makes them the default output, which is a DIV-recorded
            // default change and gets its own PR (the 0/675 ablation and the speed
            // number are the two things that PR has to add).
            "guardarm",
            "loopcondhoist",
        ];

        let agg = mode_overrides("aggressive").unwrap();
        let mut missing: Vec<&str> = Vec::new();
        for i in 0..kuna_num_settables() {
            let st = kuna_settable_by_index(i);
            // Only ON/OFF options with a shipped default of `off` are in scope;
            // a multi-valued option (e.g. `cppsig = off|proven|inferred`) has no
            // single "on" the preset could pin.
            if st.shipped != "off" || st.values != "on|off" {
                continue;
            }
            if EXCLUDED_ON_PURPOSE.contains(&st.option) {
                assert!(
                    !agg.iter().any(|(o, _)| *o == st.option),
                    "`{}` is a documented exclusion and must NOT be in aggressive",
                    st.option
                );
                continue;
            }
            if UNEVALUATED.contains(&st.option) {
                continue;
            }
            if !agg.iter().any(|(o, v)| *o == st.option && *v == "on") {
                missing.push(st.option);
            }
        }
        assert!(
            missing.is_empty(),
            "these default-off options are absent from AGGRESSIVE_OVERRIDES and are \
             therefore unreachable on the default path (`auto` picks `aggressive` \
             under 500 KiB, so `decompile-all`, the web front-end and the benchmark \
             never run them): {missing:?}. Either add each to the preset -- with a \
             sweep and a speed measurement -- or list it in EXCLUDED_ON_PURPOSE / \
             UNEVALUATED here with the reason."
        );
        // The bookkeeping lists must stay honest: an entry that IS in the preset,
        // or that names no option at all, is stale.
        for name in EXCLUDED_ON_PURPOSE.iter().chain(UNEVALUATED) {
            assert!(
                (0..kuna_num_settables()).any(|i| kuna_settable_by_index(i).option == *name),
                "`{name}` is listed here but is not a registered option"
            );
        }
        for name in UNEVALUATED {
            assert!(
                !agg.iter().any(|(o, _)| o == name),
                "`{name}` is now IN the preset -- remove it from UNEVALUATED"
            );
        }
    }

    #[test]
    fn fast_replaces_exhaustive_with_bounded_discovery() {
        assert_eq!(
            mode_overrides("fast").unwrap(),
            &[
                ("listing", "off"),
                ("funcstart_patterns", "off"),
                ("aif", "off"),
                ("fast_funcdisc", "on")
            ]
        );
    }

    #[test]
    fn unknown_mode_is_none() {
        assert!(mode_overrides("turbo").is_none());
        assert!(mode_overrides("").is_none());
    }

    #[test]
    fn mode_names_lists_all() {
        let names: Vec<_> = mode_names().collect();
        assert_eq!(names, vec!["auto", "reliable", "aggressive", "fast"]);
    }
}
