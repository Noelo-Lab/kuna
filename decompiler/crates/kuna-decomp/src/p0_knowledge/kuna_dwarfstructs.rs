//! (kuna `dwarfstructs`) The DWARF aggregate-LAYOUT gate — import a
//! `DW_TAG_structure_type`/`union_type`/`class_type`'s `DW_AT_byte_size` and its
//! `DW_TAG_member` children, instead of interning it as a named, EMPTY,
//! zero-size shell.
//!
//! kuna's DWARF type mapper recovered aggregates at name level only: every
//! aggregate DIE became `get_type_struct(name)` — a *named opaque* of size 0,
//! which is enough for `struct foo *p` to render and nothing else. Two
//! consequences follow from the size being zero rather than merely the fields
//! being absent:
//!
//! * a struct passed or returned **by value** has no width the prototype
//!   machinery can classify, so the slot degrades to a raw integer
//!   (`int take_struct(unsigned long,int)` for `take_struct(P8,int)`), and an
//!   8-byte struct return is misclassified as a hidden-return-pointer call —
//!   emitting a `rethidden` parameter that the body then does arithmetic on;
//! * every field access renders as a cast-and-offset
//!   (`*(int *)((long)n + 4)` for `n->inner.a`).
//!
//! ## Why this is an env-var bridge, not an `Architecture` flag
//!
//! The types are baked into the analysis facts by `DwarfPass` at `load file`,
//! which runs **upstream of every `option` command** — an `Architecture` bool
//! would be read after the mapping already happened. So the gate is a process
//! environment variable ([`DWARFSTRUCTS_ENV`]), consulted by the mapper and
//! exported by the `kuna decompile`/`decompile-all` CLI before the load, exactly
//! as the sibling `typedepth` gate does. The companion
//! `Architecture::analysis_dwarfstructs` bool keeps the option catalog-visible
//! and makes `phase catalog`'s live `current` field reflect it.
//!
//! Default **on** (DIV): a zero-size type is not a conservative answer — it is a
//! wrong one that the ABI classifier acts on. `--option dwarfstructs off`
//! restores the byte-identical name-only mapping.

/// Environment variable that gates DWARF aggregate-layout import (read by
/// `kuna-analysis::analyzers::dwarf::kuna_dwarfstructs`). Absent or any value
/// other than the off-tokens ⇒ **enabled**; set to `off`/`0`/`false` ⇒ disabled
/// (the pre-fix named-opaque mapping).
pub const DWARFSTRUCTS_ENV: &str = "KUNA_DWARFSTRUCTS";

/// Whether DWARF aggregate-layout import is enabled for this process.
/// Default **on**: only an explicit off-token in [`DWARFSTRUCTS_ENV`] disables it.
pub fn dwarfstructs_enabled() -> bool {
    match std::env::var(DWARFSTRUCTS_ENV) {
        Ok(v) => !matches!(v.trim().to_ascii_lowercase().as_str(), "off" | "0" | "false"),
        Err(_) => true, // unset => default-on
    }
}

/// Bridge a `set_kuna_option("dwarfstructs", val)` toggle to
/// [`DWARFSTRUCTS_ENV`] so a later `load file` in the *same* process sees the
/// choice (the interactive `option dwarfstructs ...` before `load file` path;
/// the CLI sets the env var on the subprocess directly).
pub fn set_dwarfstructs_env(on: bool) {
    std::env::set_var(DWARFSTRUCTS_ENV, if on { "on" } else { "off" });
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The gate defaults on (unset env) and only the off-tokens disable it.
    /// Serialized by being a single test (env is process-global).
    #[test]
    fn dwarfstructs_env_gate() {
        std::env::remove_var(DWARFSTRUCTS_ENV);
        assert!(dwarfstructs_enabled(), "unset => default-on");
        for off in ["off", "0", "false", "OFF", " off "] {
            std::env::set_var(DWARFSTRUCTS_ENV, off);
            assert!(!dwarfstructs_enabled(), "`{off}` should disable");
        }
        for on in ["on", "1", "true", "anything"] {
            std::env::set_var(DWARFSTRUCTS_ENV, on);
            assert!(dwarfstructs_enabled(), "`{on}` should enable");
        }
        set_dwarfstructs_env(false);
        assert!(!dwarfstructs_enabled());
        set_dwarfstructs_env(true);
        assert!(dwarfstructs_enabled());
        std::env::remove_var(DWARFSTRUCTS_ENV);
    }
}
