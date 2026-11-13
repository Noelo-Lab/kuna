//! (kuna) Degenerate-symbol-name repair — stop one degenerate qualified
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
//! `analyzers/demangle`; this repair is the backstop for every *other* way a name
//! can arrive degenerate).
//!
//! The empty component is skipped and the symbol keeps the rest of its scope
//! path. The repair can only ever fire where the alternative is a total load
//! failure, so it turns *no* output into output and can change no working
//! result.
//!
//! [`Database::find_create_scope_from_symbol_name`]: crate::database::Database::find_create_scope_from_symbol_name

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
/// does rather than leaning on this repair.
pub fn skip_scope_component(component: &str) -> bool {
    component.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Only the EMPTY component is degenerate.
    #[test]
    fn only_the_empty_component_is_skipped() {
        assert!(skip_scope_component(""));
        for keep in ["a", "std", "anonymous_namespace", "operator[]", " "] {
            assert!(!skip_scope_component(keep), "`{keep}` names a scope");
        }
    }
}
