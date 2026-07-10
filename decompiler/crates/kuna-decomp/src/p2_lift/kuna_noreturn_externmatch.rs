//! (kuna) Name-matched no-return for unresolved externs — `option
//! noreturn_externmatch` (angr port: `test_decompiling_incorrect_duplication_chcon_main`;
//! angr CLE/CFGFast extern no-return tagging; S2 flow follow).
//!
//! ## The gap
//!
//! On a normal dynamically-linked ELF, the `noreturn_known` analysis pass
//! (`kuna-analysis` `loader/noreturn.rs`, the kuna analog of Ghidra's
//! `NoReturnFunctionAnalyzer`) marks `exit`/`abort`/`__stack_chk_fail`/… no-return
//! by scanning the symbol table and emitting a `NoReturnFact` keyed by the
//! function's **address**. The commit sets the `FunctionSymbol` proto's
//! no-return flag, so `FlowInfo::queryCall` plants an `artificialHalt(noreturn)`
//! after the call and flow never runs into the bytes past it.
//!
//! In an **ET_REL `.o`** that breaks down: `__stack_chk_fail` is an *undefined*
//! extern — its `.symtab` entry has `address()==0` and there is no PLT stub — so
//! the address-keyed scan emits no fact for it. The `relocobjects` loader
//! (default-on, DIV-7) binds the extern to a *synthetic* call target and installs
//! a `FunctionSymbol` named `__stack_chk_fail` there (the call site **prints**
//! `__stack_chk_fail()`), but no no-return flag is ever set on that synthetic
//! target. Flow then runs off the end of the function into the inter-function
//! alignment padding (`00 00 …` decoded as `add byte ptr [rax], al`), and the
//! decompiler emits dozens of garbage `*v = *v + c;` lines after the call — the
//! `chcon.o::main` symptom angr's `test_decompiling_incorrect_duplication_chcon_main`
//! does not have.
//!
//! ## What this option does
//!
//! When `option noreturn_externmatch on`, the `FlowEnvironment::query_call_no_return`
//! hook (`infra/decompile_drive.rs`) additionally consults the callee **name** at
//! the direct-call entry: if the name (after stripping leading `_`, honoring the
//! upstream global/`std` namespace guard) matches the same vendored ELF
//! known-no-return list `noreturn_known` uses, the hook reports no-return. flow.rs
//! ORs this query with the proto flag at the artificial-halt site
//! (`let no_return = is_no_return || query_call_no_return(entry)`), so a name match
//! plants the halt and the trailing padding is never decoded — closing the gap by
//! name where the address-keyed scan could not.
//!
//! On a normal ELF the proto flag is already set by `noreturn_known`, so the OR is
//! a no-op (byte-identical output); the only behavioral change is on ET_REL
//! undefined externs the address-keyed scan missed. The list is `include_str!`'d
//! from the canonical `kuna-analysis/data/ElfFunctionsThatDoNotReturn` (a
//! build-time string include, *not* a crate dependency) so it can never drift from
//! `noreturn_known`.
//!
//! Ships **default-ON** (DIV-13): a clean 0/675 datatest ablation (the corpus is
//! either real ELFs, where `noreturn_known` already set the proto flag and this is
//! a no-op, or bytechunks with no name-matched returning extern) and a ~23% speed
//! *win* on the target (less dead padding to decompile). It carries no risk class
//! beyond the already-default-on `noreturn_known`: it applies the *same* vendored
//! name list and the *same* global/`std` namespace guard, just at the flow query
//! boundary to reach the ET_REL extern that the address-keyed scan structurally
//! misses. Set `option noreturn_externmatch off` to restore the prior
//! byte-identical rendering (the dead `add`-padding after the call reappears).

use kuna_base::marshal::ElementId;

/// Marshaling element `<noreturn_externmatch>` (kuna). ElementIds live in the
/// 4000+ range; 4103 (`noreturn_extern`, PR #90) is the previous max.
pub const ELEM_NORETURN_EXTERNMATCH: ElementId = ElementId::new("noreturn_externmatch", 4104);

/// The known-no-return name list, `include_str!`'d from the canonical vendored
/// file the `noreturn_known` analysis pass uses
/// (`kuna-analysis/data/ElfFunctionsThatDoNotReturn`, itself vendored verbatim
/// from Ghidra `Ghidra/Features/Base/data/ElfFunctionsThatDoNotReturn`). A
/// build-time string include — kuna-decomp does *not* depend on kuna-analysis;
/// this keeps a single source of truth so the two matchers can never drift.
const ELF_NORETURN_LIST: &str =
    include_str!("../../../kuna-analysis/data/ElfFunctionsThatDoNotReturn");

/// Strip **all** leading underscores (faithful to
/// `NoReturnFunctionAnalyzer.java:84-91` — a loop, not a single strip: so
/// `__stack_chk_fail` -> `stack_chk_fail` and `_exit` -> `exit`).
fn strip_leading_underscores(s: &str) -> &str {
    s.trim_start_matches('_')
}

/// Split a (possibly namespaced) symbol name into `(base, namespace)`. kuna
/// installs `::`-split namespaces, so a demangled `Menu::exit` has base `exit` in
/// namespace `Menu`; a flat C import `exit` has no namespace.
fn base_and_namespace(name: &str) -> (&str, Option<&str>) {
    match name.rfind("::") {
        Some(i) => (&name[i + 2..], Some(&name[..i])),
        None => (name, None),
    }
}

/// `true` if `name` matches the vendored ELF known-no-return list, mirroring the
/// `noreturn_known` matcher (`loader/noreturn.rs::name_matches`): the namespace
/// guard (only global / `std`, never a C++ class method like `Menu::_exit`),
/// leading-`_` strip, then exact or trailing-`*` wildcard-prefix match.
///
/// The parse is recomputed per call (the list is ~20 short lines; this is invoked
/// only from the gated `query_call_no_return` hook on direct CALLs). It mirrors
/// `parse_list`: skip `#`/blank lines, strip leading `_`, a trailing `*` makes a
/// wildcard prefix entry (the shipped ELF list has no wildcards, but the parse is
/// faithful).
pub fn is_known_noreturn_name(name: &str) -> bool {
    let (base, ns) = base_and_namespace(name);
    // Namespace guard: skip class methods. Global (no namespace) is allowed; the
    // sole allowed non-global namespace is exactly `std`.
    if let Some(ns) = ns {
        if ns != "std" {
            return false;
        }
    }
    let stripped = strip_leading_underscores(base);
    if stripped.is_empty() {
        return false;
    }
    for line in ELF_NORETURN_LIST.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let entry = strip_leading_underscores(line);
        if let Some(prefix) = entry.strip_suffix('*') {
            if stripped.starts_with(prefix) {
                return true;
            }
        } else if entry == stripped {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_canary_and_abort_family() {
        // The chcon.o::main target and its neighbours in the vendored list.
        assert!(is_known_noreturn_name("__stack_chk_fail"));
        assert!(is_known_noreturn_name("stack_chk_fail"));
        assert!(is_known_noreturn_name("abort"));
        assert!(is_known_noreturn_name("exit"));
        assert!(is_known_noreturn_name("_exit"));
        assert!(is_known_noreturn_name("__assert_fail"));
    }

    #[test]
    fn namespace_guard_rejects_class_methods_but_allows_std() {
        assert!(!is_known_noreturn_name("Menu::exit"));
        assert!(!is_known_noreturn_name("Menu::_exit"));
        // The guard allows the `std` namespace; `exit` is in the vendored list.
        assert!(is_known_noreturn_name("std::exit"));
    }

    #[test]
    fn rejects_ordinary_returning_names() {
        assert!(!is_known_noreturn_name("process_files"));
        assert!(!is_known_noreturn_name("main"));
        assert!(!is_known_noreturn_name("error"));
        assert!(!is_known_noreturn_name(""));
        assert!(!is_known_noreturn_name("_"));
    }
}
