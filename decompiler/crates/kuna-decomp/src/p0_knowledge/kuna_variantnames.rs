//! (kuna `variantnames`) The gate for naming a Rust enum's discriminant
//! VALUES in the emitted C (`docs/multilang.md`, the Rust pack's first
//! `VariantLayout` consumer).
//!
//! `dwarfvariants` (DIV-87) recovers a `DW_TAG_variant_part`'s geometry — the
//! tag member, every `DW_AT_discr_value`, every variant name — and installs
//! `struct E { tag; union E::payload {…} }`, but the `tag` field keeps the
//! plain integer type DWARF declares for it, so every test of it renders as a
//! bare constant: `if ((*e).tag == 1)`. The compiler SAID what `1` is. With
//! this gate on, the importer gives the tag field a synthesized kuna enum
//! built from the variant-part's own discriminant→name map, and the existing
//! enum-constant rendering (the `TYPE_ENUM_*` namemap path the C-enum import
//! already uses) prints `if ((*e).tag == Err)` with no new emission machinery
//! at all.
//!
//! Like its parent gate this is an env-var bridge, not only an `Architecture`
//! bool: the types are baked into the analysis facts inside `load file`,
//! upstream of every `option` command, so the CLI binds [`VARIANTNAMES_ENV`]
//! onto the process before loading (the `dwarfstructs`/`dwarfvariants`
//! pattern). Default **on** — the renamed constant is the compiler's own
//! vocabulary, the gate acts only where a variant part was imported at all
//! (full `-g` debug info), and `dwarfvariants off` suppresses it wholesale.

/// Environment variable gating the synthesized discriminant enum (read by
/// `kuna-analysis::analyzers::dwarf::kuna_variantnames`). Absent or any value
/// other than the off-tokens ⇒ **enabled**; `off`/`0`/`false` ⇒ disabled.
pub const VARIANTNAMES_ENV: &str = "KUNA_VARIANTNAMES";

/// Whether discriminant-value naming is enabled for this process.
pub fn variantnames_enabled() -> bool {
    crate::options::env_toggle(VARIANTNAMES_ENV, true)
}

/// Bridge a `set_kuna_option("variantnames", val)` toggle to
/// [`VARIANTNAMES_ENV`] so a later `load file` in the same process sees it.
pub fn set_variantnames_env(on: bool) {
    std::env::set_var(VARIANTNAMES_ENV, if on { "on" } else { "off" });
}
