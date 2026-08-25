//! (kuna) The `symbolnamerepair` option gate — stop one degenerate qualified
//! symbol name from aborting the load of an entire binary.
//!
//! ## The failure it removes
//!
//! [`Database::find_create_scope_from_symbol_name`] splits a loader symbol name
//! on every `::` and nests one Scope per component. An **empty** component —
//! `a::::b`, `::b` — reaches `Database::attach_scope`, which rejects it
//! with the `Non-global scope has empty name` `LowlevelError`. That is raised
//! while `read_loader_symbols` is installing the symbol table, i.e. *inside*
//! `bootstrap_from_object`, so it does not cost one symbol: it escapes the
//! architecture build, and every command — `decompile`, `decompile-all`,
//! `functions`, `decompile-project` — answers
//! `could not build an architecture for <binary>` and produces nothing at all.
//!
//! One symbol denying analysis of a whole binary is a poor trade at the best of
//! times, and a symbol name is attacker-controlled data. A hostile binary can
//! carry a name shaped to hit exactly this seam for the price of a few
//! `.strtab`/`.dynstr` bytes, and unlike a malformed header it survives every
//! sanity check the loader applies, because a name is never parsed until it is
//! split here. On the sample that motivated the gate — a MinGW-built malware
//! DLL — the trigger was not even adversarial: 1184 ordinary libstdc++ symbols
//! sat in a C++ anonymous namespace, whose demangling used to reduce to an empty
//! component (fixed at its own root in `kuna-analysis`'s
//! `analyzers/demangle`; this gate is the backstop for every *other* way a name
//! can arrive degenerate).
//!
//! With the gate on the empty component is skipped and the symbol keeps the rest
//! of its scope path; with it off the historical hard error stands, which is what
//! someone *investigating* a binary's symbol table wants to see.
//!
//! ## Why an env-var bridge, not an `Architecture` flag
//!
//! The symbol table is installed inside `load file`, **upstream of every
//! per-function `option` command**, so an `Architecture` bool would be read long
//! after the load had already failed. This mirrors the `relocobjects` /
//! `i386_pie_plt` / `relocrebase` / `typedepth` loader-option pattern: the gate is
//! a process environment variable ([`SYMBOLNAMEREPAIR_ENV`]), set by the `kuna`
//! CLI before `bootstrap_from_object` (and on the `decomp_dbg` subprocess). The
//! companion `Architecture::analysis_symbolnamerepair` bool exists only so the
//! option is catalog-visible and `phase catalog`'s live `current` field reflects
//! it.
//!
//! Default **on**: the gate can only ever fire where the alternative is a total
//! load failure, so it turns *no* output into output and can change no working
//! result.
//!
//! [`Database::find_create_scope_from_symbol_name`]: crate::database::Database::find_create_scope_from_symbol_name

/// Environment variable that gates degenerate-symbol-name repair (read by
/// `Database::find_create_scope_from_symbol_name`). Absent or any value other
/// than the off-tokens ⇒ **enabled** (the default); set to `off`/`0`/`false` ⇒
/// disabled (the historical hard error).
pub const SYMBOLNAMEREPAIR_ENV: &str = "KUNA_SYMBOLNAMEREPAIR";

/// Whether degenerate-symbol-name repair is enabled for this process.
/// Default **on**: only an explicit off-token in [`SYMBOLNAMEREPAIR_ENV`]
/// disables it.
pub fn symbolnamerepair_enabled() -> bool {
    match std::env::var(SYMBOLNAMEREPAIR_ENV) {
        Ok(v) => !matches!(v.trim().to_ascii_lowercase().as_str(), "off" | "0" | "false"),
        Err(_) => true, // unset ⇒ default-on
    }
}

/// Whether `component` — one `::`-delimited piece of a qualified symbol name —
/// should be skipped rather than nested as a Scope.
///
/// Only the empty component is degenerate: every other string names a Scope
/// perfectly well, however strange it looks, and dropping more would silently
/// discard namespace structure the binary really carries. Note that a TRAILING
/// `::` never reaches here — `a::` yields the non-empty component `a` and an
/// empty *basename*, which the loop returns rather than nesting.
///
/// Skipping is a lossy repair and is meant to be: `a::::b` installs `b` one
/// scope shallower than the intact `a::X::b` would have, because the component
/// that named `X` is simply not in the string any more. That is the right trade
/// only because the alternative is failing the whole load — where the lost
/// component is *recoverable*, it is recovered at the source instead, which is
/// what the anonymous-namespace fix in `kuna-analysis`'s `analyzers/demangle`
/// does rather than leaning on this gate.
pub fn skip_scope_component(component: &str) -> bool {
    component.is_empty() && symbolnamerepair_enabled()
}

/// Bridge a `set_kuna_option("symbolnamerepair", val)` toggle to
/// [`SYMBOLNAMEREPAIR_ENV`] so a later `load file` in the *same* process sees the
/// choice (the interactive `option symbolnamerepair …` before `load file` path,
/// and the in-process CLI drivers).
pub fn set_symbolnamerepair_env(on: bool) {
    std::env::set_var(SYMBOLNAMEREPAIR_ENV, if on { "on" } else { "off" });
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The gate defaults on (unset env) and only the off-tokens disable it.
    /// Serialized by being a single test (env is process-global).
    #[test]
    fn env_gate_default_on_and_off_tokens() {
        std::env::remove_var(SYMBOLNAMEREPAIR_ENV);
        assert!(symbolnamerepair_enabled(), "unset env ⇒ default on");

        for off in ["off", "0", "false", "OFF", "False"] {
            std::env::set_var(SYMBOLNAMEREPAIR_ENV, off);
            assert!(!symbolnamerepair_enabled(), "`{off}` ⇒ disabled");
            assert!(!skip_scope_component(""), "off ⇒ the empty component is fatal again");
        }
        for on in ["on", "1", "true", ""] {
            std::env::set_var(SYMBOLNAMEREPAIR_ENV, on);
            assert!(symbolnamerepair_enabled(), "`{on}` ⇒ enabled");
            assert!(skip_scope_component(""), "on ⇒ the empty component is skipped");
        }

        set_symbolnamerepair_env(false);
        assert!(!symbolnamerepair_enabled());
        set_symbolnamerepair_env(true);
        assert!(symbolnamerepair_enabled());

        // Only the EMPTY component is degenerate, gate or no gate.
        for keep in ["a", "std", "anonymous_namespace", "operator[]", " "] {
            assert!(!skip_scope_component(keep), "`{keep}` names a scope");
        }

        std::env::remove_var(SYMBOLNAMEREPAIR_ENV);
    }
}
