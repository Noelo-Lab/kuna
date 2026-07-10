//! Symbol-name demangling — the kuna analog of Ghidra's `GnuDemanglerAnalyzer`
//! ("Demangler GNU").
//!
//! ## Why a crate, not a hand-port
//!
//! Ghidra's GNU/Itanium demangler is **not** Java logic to transcribe:
//! `GnuDemanglerNativeProcess` shells out to a native `demangle` executable
//! (libiberty `cplus_demangle`); the Java side only pre-filters candidate
//! strings and parses the c++filt-style text the native process prints. There
//! is no Itanium grammar in Ghidra Java to mirror. So the faithful port is
//! "call a third-party demangler and consume its string" — exactly the same
//! dependency-substitution LOSS the BFD object model takes (GPL BFD → the
//! `object` crate; see [`crate::loadimage_object`] and
//! `docs/rust-port/losses.md`). Here Ghidra's native `demangle`/libiberty
//! dependency is replaced by the permissive [`cpp_demangle`] (Itanium C++) and
//! [`rustc_demangle`] (Rust legacy `_ZN` + v0 `_R`) crates.
//!
//! ## The MSVC arm (multi-format loader PR-9)
//!
//! Itanium (`cpp_demangle`) covers ELF/Mach-O C++ and MinGW-PE; Rust
//! (`rustc_demangle`) covers Rust. Neither recognizes **MSVC** mangling — the
//! `cl.exe` scheme that starts with `?` (`?foo@Bar@@QEAAXXZ`), carried by MSVC
//! PE/COFF objects (`docs/history/multiformat-loader-design.md` §5.5). Ghidra demangles
//! it with `MicrosoftDemangler` (the hand-rolled `MDMang` C++ grammar, again not
//! Java to transcribe), so the faithful move is identical: substitute a
//! permissive crate — here [`msvc_demangler`] — and consume its string. This is
//! the same dependency-substitution LOSS as the Itanium/Rust deps, recorded in
//! `docs/rust-port/losses.md`. The MSVC arm fires only when `raw` starts with
//! `?`, which no Itanium/Rust/C symbol ever does, so it is a pure additive arm
//! (ELF output byte-identical). `msvc_demangler`'s `NAME_ONLY` flag yields the
//! qualified-name-only form directly — already free of the signature/template
//! `::` the scope splitter must not see (the same name-only contract below).
//!
//! Origin (upstream Ghidra, the tree kuna was ported from):
//! - analyzer: `Ghidra/Features/GnuDemangler/.../GnuDemanglerAnalyzer.java`
//! - the pre-filter ported here: `GnuDemangler.skip`
//!   (`Ghidra/Features/GnuDemangler/.../GnuDemangler.java:186-227`) +
//!   `isGnu2Or3Pattern` (`:274-283`).
//!
//! ## Where this runs
//!
//! Applied in [`crate::loadimage_object`]`::from_bytes`, to each loader funcsym
//! name (`.symtab` / PLT / `.dynsym`) *after* `@VERSION` stripping and *before*
//! the name is installed as a `FunctionSymbol`. So a mangled `_ZN3foo3barEv`
//! becomes `foo::bar`, kuna's `::`-namespace splitter
//! (`Database::find_create_scope_from_symbol_name`) nests it, and calls render
//! with readable names.
//!
//! ## The name-only contract (MANDATORY)
//!
//! kuna's scope splitter splits a symbol name on **every** `::` into nested
//! scopes. A full c++filt form carries a signature (`ns::f(std::string)`) and
//! template args (`vector<std::pair<...>>`), both of which contain `::` that
//! would create junk scopes. So the loader consumes [`demangle_name`], which
//! reduces the demangled form to the **qualified name only** — signature tail
//! and all bracketed template/array/signature groups removed (see
//! [`strip_bracket_groups`]). [`demangle_raw`] keeps the full c++filt-like form
//! for callers that want it (and for the roadmap-documented `_Z3fooi` ->
//! `foo(int)` shape).
//!
//! ## Scope (PARTIAL — names + namespaces only)
//!
//! This delivers Ghidra's **name + namespace** application — the same depth as
//! the PLT pass ("correct names"). It does NOT apply the demangled
//! **signature** (parameter / return types): kuna recovers parameter types from
//! usage in S4/S5, so re-seeding libiberty's signature is out of scope here (a
//! deferred follow-up, like the PLT external-location/thunk object model).
//!
//! ## Output spelling (minor divergence, not a parity break)
//!
//! The demangled string spelling can differ slightly from libiberty c++filt
//! (e.g. `cpp_demangle` renders `std::__cxx11::basic_string`, where some c++filt
//! builds elide `__cxx11`). For the name-only path this only affects which
//! intermediate `::` scopes are created, not correctness of the base name.

/// `GnuDemangler.GLOBAL_PREFIX` (`GnuDemangler.java:35`).
const GLOBAL_PREFIX: &str = "_GLOBAL_";

/// Port of `GnuDemangler.skip` (`GnuDemangler.java:186-227`) +
/// `isGnu2Or3Pattern` (`:274-283`): `true` when `raw` should NOT be handed to a
/// demangler. We always gate to known patterns (the Ghidra
/// `demangleOnlyKnownPatterns()` branch) so plain C names (`puts`, `main`) and
/// other false positives are never mangled.
fn skip(raw: &str) -> bool {
    // MSVC C++ (multi-format loader PR-9): the `cl.exe` scheme is the only one
    // that starts with `?` (`?foo@Bar@@QEAAXXZ`). It must be recognized BEFORE
    // the `@`-version check below — MSVC names embed `@` structurally (namespace
    // / type separators), so the version heuristic would otherwise reject every
    // MSVC symbol. No Itanium/Rust/C symbol ever starts with `?`, so this is a
    // clean early gate that never reorders the existing behavior.
    if raw.starts_with('?') {
        return false;
    }

    // Ignore versioned symbols (`foo@GLIBC_2.2.5`), generally duplicated at the
    // same address. `indexOf("@") > 0`: an '@' must be present and not leading.
    if let Some(p) = raw.find('@') {
        if p > 0 {
            return true;
        }
    }

    // Not a mangled symbol, but the demangler will try anyway — don't let it.
    if raw.starts_with("___") {
        return true;
    }

    // The known GNU start patterns (the `demangleOnlyKnownPatterns()` branch).
    // `_GLOBAL_<...>_Z<...>` (a static (de)init thunk wrapping a mangled name).
    if raw.starts_with(GLOBAL_PREFIX) {
        if let Some(idx) = raw.find("_Z") {
            if idx > 0 {
                return false;
            }
        }
    }
    if raw.starts_with("_Z") {
        return false;
    }
    if raw.starts_with("__Z") {
        return false;
    }
    // Rust v0 mangling (`_R...`). Not in Ghidra's Itanium-only `skip`, but kuna
    // also feeds `rustc_demangle`, which owns this prefix; gate it the same way.
    if raw.starts_with("_R") {
        return false;
    }
    if is_gnu2_or_3_pattern(raw) {
        return false;
    }

    true
}

/// Port of `GnuDemangler.isGnu2Or3Pattern` (`GnuDemangler.java:274-283`): the
/// Gnu2/3 static-(de)init thunk constructs.
fn is_gnu2_or_3_pattern(raw: &str) -> bool {
    raw.starts_with("_GLOBAL_.I.")
        || raw.starts_with("_GLOBAL_.D.")
        || raw.starts_with("_GLOBAL__I__Z")
        || raw.starts_with("_GLOBAL__D__Z")
}

/// Demangle `raw` to the **full** c++filt-like form (signature included), or
/// `None` if `raw` is not a (recognized, demangleable) mangled symbol.
///
/// Itanium C++ ([`cpp_demangle`]) is tried first, then Rust
/// ([`rustc_demangle`], `{:#}` to drop the v0/legacy hash). A result that is
/// empty or unchanged (`== raw`) means "not actually mangled" → `None`.
pub fn demangle_raw(raw: &str) -> Option<String> {
    if skip(raw) {
        return None;
    }

    // MSVC C++ (PR-9): a leading `?` is exclusive to the `cl.exe` scheme, which
    // the Itanium/Rust crates reject, so try it first when present.
    // `DemangleFlags::COMPLETE` gives the full `public: void __cdecl Bar::foo(void)`
    // form (the MSVC analog of the c++filt-like Itanium output).
    if raw.starts_with('?') {
        if let Ok(d) = msvc_demangler::demangle(raw, msvc_demangler::DemangleFlags::COMPLETE) {
            if !d.is_empty() && d != raw {
                return Some(d);
            }
        }
        return None;
    }

    // Itanium C++ first.
    if let Ok(sym) = cpp_demangle::Symbol::new(raw) {
        if let Ok(d) = sym.demangle() {
            if !d.is_empty() && d != raw {
                return Some(d);
            }
        }
    }

    // Then Rust (legacy `_ZN..` and v0 `_R..`); `{:#}` drops the trailing hash.
    if let Ok(d) = rustc_demangle::try_demangle(raw) {
        let s = format!("{:#}", d);
        if !s.is_empty() && s != raw {
            return Some(s);
        }
    }

    None
}

/// Demangle `raw` to the **qualified name only** (signature + template/bracket
/// groups removed), or `None` if `raw` is not a (recognized) mangled symbol.
///
/// This is what the loader installs as a symbol name. The reduction is
/// MANDATORY: kuna's scope splitter splits on every `::`, and a leftover
/// signature (`ns::f(std::string)`) or template arg list
/// (`vector<std::pair<...>>`) would otherwise create junk scopes. For Itanium we
/// ask [`cpp_demangle`] for the name-only form
/// (`DemangleOptions::no_params().no_return_type()`) and *additionally* strip
/// any residual bracketed group (`cpp_demangle` keeps `<...>` template args on
/// the qualified name even with `no_params`); Rust names already carry no
/// signature, but [`strip_bracket_groups`] is applied uniformly for safety.
pub fn demangle_name(raw: &str) -> Option<String> {
    if skip(raw) {
        return None;
    }

    // MSVC C++ (PR-9): a leading `?` is exclusive to the `cl.exe` scheme.
    // `DemangleFlags::NAME_ONLY` returns the qualified-name-only form directly
    // (`?foo@Bar@@QEAAXXZ` -> `Bar::foo`), already free of signature/template
    // text; `strip_bracket_groups` is applied uniformly for safety (a templated
    // name-only form could still carry `<...>`).
    if raw.starts_with('?') {
        if let Ok(d) = msvc_demangler::demangle(raw, msvc_demangler::DemangleFlags::NAME_ONLY) {
            let reduced = strip_bracket_groups(&d);
            if !reduced.is_empty() && reduced != raw {
                return Some(reduced);
            }
        }
        return None;
    }

    // Itanium C++ first, name-only options.  A legacy Rust `_ZN..` symbol is
    // *also* valid Itanium, so cpp_demangle handles it but keeps the trailing
    // `::h<hex>` mangling hash as a junk scope; strip it (the `{:#}` hash-drop a
    // pure rustc_demangle path would give).
    if let Ok(sym) = cpp_demangle::Symbol::new(raw) {
        let opts = cpp_demangle::DemangleOptions::new()
            .no_params()
            .no_return_type();
        if let Ok(d) = sym.demangle_with_options(&opts) {
            let reduced = strip_legacy_rust_hash(&strip_bracket_groups(&d));
            if !reduced.is_empty() && reduced != raw {
                return Some(reduced);
            }
        }
    }

    // Then Rust (`{:#}` already drops the hash; the extra strip is defensive).
    if let Ok(d) = rustc_demangle::try_demangle(raw) {
        let reduced = strip_legacy_rust_hash(&strip_bracket_groups(&format!("{:#}", d)));
        if !reduced.is_empty() && reduced != raw {
            return Some(reduced);
        }
    }

    None
}

/// Remove every balanced bracketed group — `<...>` (template args), `(...)`
/// (signature), `[...]` (array/attribute) — from a demangled name, keeping only
/// the qualified-name text. Depth-tracked so nested groups
/// (`vector<pair<int, int> >`) are dropped whole; unbalanced trailing brackets
/// are tolerated. Leaves only the `::`-qualified name (e.g.
/// `std::vector<int, std::allocator<int> >::push_back(int const&)` ->
/// `std::vector::push_back`).
fn strip_bracket_groups(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut depth: i32 = 0;
    for c in s.chars() {
        match c {
            '<' | '(' | '[' => depth += 1,
            '>' | ')' | ']' => {
                if depth > 0 {
                    depth -= 1;
                }
            }
            _ => {
                if depth == 0 {
                    out.push(c);
                }
            }
        }
    }
    out.trim().to_string()
}

/// Strip a trailing legacy-Rust mangling-hash scope (`::h<16 lowercase hex>`)
/// from a reduced name. Legacy Rust `_ZN..` mangling is valid Itanium, so
/// [`cpp_demangle`] demangles it but keeps the disambiguator hash as a final
/// `::h<hex>` component; this drops it, matching the `{:#}` hash-elision a pure
/// [`rustc_demangle`] path gives. Conservative: only a final component that is
/// exactly `h` + 16 hex digits is removed (the rustc legacy hash shape), so a
/// real C++ name is unaffected.
fn strip_legacy_rust_hash(name: &str) -> String {
    if let Some(last) = name.rfind("::") {
        let tail = &name[last + 2..];
        let is_legacy_hash = tail.len() == 17
            && tail.starts_with('h')
            && tail[1..].bytes().all(|b| b.is_ascii_hexdigit());
        if is_legacy_hash {
            return name[..last].to_string();
        }
    }
    name.to_string()
}

/// Recover the **C++ class name** carried by an MSVC RTTI `TypeDescriptor` (RTTI0)
/// mangled type-info string, or `None` if `typeinfo` is not a recognized `.?A…@@`
/// type-info name.
///
/// MSVC's RTTI0 `TypeDescriptor` stores its class as a *type-info* mangled string,
/// `.?A<kind><name>@@` (`kind` = `V` class / `U` struct / `T` union / `W` enum) —
/// e.g. `.?AVShape@@`, `.?AUBox@@`, `.?AVInner@ns@@`, `.?AV?$Vec@H@@`. This is NOT
/// a symbol name and the `?`-symbol arm of [`demangle_name`] does not handle it (it
/// would yield the raw `AVShape`/`AUBox` with the type-code prefix attached). The
/// faithful recovery — Ghidra's `RttiUtil`/`MicrosoftDemangler` recipe — is to wrap
/// the type-info string in the RTTI0 descriptor symbol form `??_R0<name>@8` and
/// demangle *that*, which the MSVC scheme renders as
/// `<qualified-class>::`RTTI Type Descriptor'`; the class name is the prefix.
///
/// Reuses the **same** [`msvc_demangler`] crate the MSVC arm of [`demangle_name`]
/// already drives (no new demangler dependency); `NO_MS_KEYWORDS | NO_CLASS_TYPE`
/// drop the leading `class`/`struct` keyword so the result is the bare qualified
/// name (`ns::Inner`, `Vec<int>`). The recovered name is reduced through
/// [`strip_bracket_groups`] for the same name-only `::`-scope-splitter contract the
/// rest of this module enforces (a templated class like `Vec<int>` → `Vec`).
///
/// Returns `None` for any string that does not demangle to the
/// `…::`RTTI Type Descriptor'` shape — never panics.
pub fn demangle_rtti_class_name(typeinfo: &str) -> Option<String> {
    // RTTI0 type-info names carry a leading `.`; the wrapped symbol form does not.
    let core = typeinfo.strip_prefix('.').unwrap_or(typeinfo);
    if !core.starts_with("?A") {
        return None;
    }
    let wrapped = format!("??_R0{core}@8");
    let flags = msvc_demangler::DemangleFlags::NAME_ONLY
        | msvc_demangler::DemangleFlags::NO_MS_KEYWORDS
        | msvc_demangler::DemangleFlags::NO_CLASS_TYPE;
    let d = msvc_demangler::demangle(&wrapped, flags).ok()?;
    // `d` is `<qualified-class>::`RTTI Type Descriptor'`; take the class prefix.
    let class = d.strip_suffix("::`RTTI Type Descriptor'")?;
    let reduced = strip_bracket_groups(class);
    if reduced.is_empty() {
        None
    } else {
        Some(reduced)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rtti_class_name_msvc_typeinfo() {
        // The RTTI0 `.?A<kind><name>@@` type-info forms recover their bare class
        // name (the Ghidra RttiUtil recipe), kind-agnostic (V class / U struct).
        assert_eq!(demangle_rtti_class_name(".?AVShape@@").as_deref(), Some("Shape"));
        assert_eq!(demangle_rtti_class_name(".?AUBox@@").as_deref(), Some("Box"));
        // A namespaced class keeps its `::` qualification (the scope splitter nests it).
        assert_eq!(demangle_rtti_class_name(".?AVInner@ns@@").as_deref(), Some("ns::Inner"));
        // A templated class is reduced to the bare name (the name-only contract).
        assert_eq!(demangle_rtti_class_name(".?AV?$Vec@H@@").as_deref(), Some("Vec"));
        // The leading `.` is optional (some readers strip it before calling).
        assert_eq!(demangle_rtti_class_name("?AVShape@@").as_deref(), Some("Shape"));
        // Non-type-info strings are rejected (no panic, no junk name).
        assert_eq!(demangle_rtti_class_name("Shape"), None);
        assert_eq!(demangle_rtti_class_name(".?AB"), None);
        assert_eq!(demangle_rtti_class_name(""), None);
    }

    #[test]
    fn name_only_nested_itanium() {
        // `_ZN3foo3barEv` -> `foo::bar` (void f()). Name-only, nested namespace.
        assert_eq!(demangle_name("_ZN3foo3barEv"), Some("foo::bar".to_string()));
    }

    #[test]
    fn raw_keeps_full_signature() {
        // The literal roadmap form: full c++filt-like signature is retained.
        assert_eq!(demangle_raw("_Z3fooi"), Some("foo(int)".to_string()));
        // ...while the name-only form drops it.
        assert_eq!(demangle_name("_Z3fooi"), Some("foo".to_string()));
    }

    #[test]
    fn plain_c_names_not_demangled() {
        // Not mangled → None (the skip() known-pattern gate).
        assert_eq!(demangle_name("puts"), None);
        assert_eq!(demangle_name("main"), None);
        assert_eq!(demangle_raw("printf"), None);
    }

    #[test]
    fn versioned_symbol_skipped() {
        // An '@' (versioned symbol) is skipped before any demangle attempt.
        assert_eq!(demangle_name("_Z3fooi@GLIBC_2.2.5"), None);
        assert_eq!(demangle_raw("_Z3fooi@@GLIBC_2.2.5"), None);
    }

    #[test]
    fn triple_underscore_skipped() {
        // `___foo` → None (`GnuDemangler.skip`: "the demangler will try anyway").
        assert_eq!(demangle_name("___foo"), None);
        assert_eq!(demangle_name("___ZN3foo3barEv"), None);
    }

    #[test]
    fn rust_v0_symbol_demangles() {
        // Rust v0 (`_R..`) is owned by rustc_demangle, falls through from Itanium.
        let raw = "_RNvCs15kBYyAo9fc_7mycrate7example";
        let n = demangle_name(raw);
        assert!(n.is_some(), "rust v0 must demangle: {n:?}");
        let n = n.unwrap();
        assert_eq!(n, "mycrate::example");
        // legacy rust `_ZN..` mangling, hash dropped by `{:#}`.
        let legacy = "_ZN3std2io5stdio6_print17h0123456789abcdefE";
        assert_eq!(demangle_name(legacy), Some("std::io::stdio::_print".to_string()));
    }

    #[test]
    fn templated_symbol_reduces_to_clean_name() {
        // `std::vector<int, std::allocator<int> >::push_back(int const&)`.
        // The name-only reduction MUST leave no unmatched `<` / `(` and no
        // signature/template `::` leakage that would corrupt the scope splitter.
        let raw = "_ZNSt6vectorIiSaIiEE9push_backERKi";
        let n = demangle_name(raw).expect("templated symbol demangles");
        assert_eq!(n, "std::vector::push_back");
        assert!(!n.contains('<'), "no template-arg leakage: {n}");
        assert!(!n.contains('>'), "no template-arg leakage: {n}");
        assert!(!n.contains('('), "no signature leakage: {n}");
        assert!(!n.contains(')'), "no signature leakage: {n}");
    }

    #[test]
    fn legacy_rust_hash_is_stripped() {
        // `::h<16 hex>` (the legacy rustc disambiguator) is dropped...
        assert_eq!(
            strip_legacy_rust_hash("std::io::stdio::_print::h0123456789abcdef"),
            "std::io::stdio::_print"
        );
        // ...but a normal trailing scope is kept (not a 17-char `h<hex>`).
        assert_eq!(strip_legacy_rust_hash("foo::bar"), "foo::bar");
        assert_eq!(strip_legacy_rust_hash("foo::helper"), "foo::helper");
    }

    // ---- MSVC C++ arm (multi-format loader PR-9) ----------------------------
    //
    // These are the merge gate: `cl.exe` is unavailable on Linux, so the MSVC
    // path is proven entirely by these hermetic asserts on known mangled strings
    // (the raw forms come straight from `clang -target x86_64-pc-windows-msvc`'s
    // COFF symtab — `?foo@Bar@@QEAAXXZ` etc.).

    #[test]
    fn msvc_member_function_name_only() {
        // `?foo@Bar@@QEAAXXZ` = `public: void __cdecl Bar::foo(void)`.
        // The headline before -> after: raw mangled -> `Bar::foo` (name-only).
        assert_eq!(demangle_name("?foo@Bar@@QEAAXXZ"), Some("Bar::foo".to_string()));
    }

    #[test]
    fn msvc_raw_keeps_full_form() {
        // `demangle_raw` retains the full MSVC form (the c++filt-like analog).
        assert_eq!(
            demangle_raw("?foo@Bar@@QEAAXXZ"),
            Some("public: void __cdecl Bar::foo(void)".to_string())
        );
    }

    #[test]
    fn msvc_free_function() {
        // `?freefunc@@YAXH@Z` = `void __cdecl freefunc(int)` — a non-namespaced
        // free function reduces to the bare name (no junk `::` scope).
        assert_eq!(demangle_name("?freefunc@@YAXH@Z"), Some("freefunc".to_string()));
        assert_eq!(
            demangle_raw("?freefunc@@YAXH@Z"),
            Some("void __cdecl freefunc(int)".to_string())
        );
    }

    #[test]
    fn msvc_namespaced_function() {
        // `?g@ns@@YAHHH@Z` = `int __cdecl ns::g(int, int)`. The MSVC `@`-encoded
        // namespace nests as a single `::` scope; the params (which carry the
        // structural `@`s) are dropped by NAME_ONLY.
        assert_eq!(demangle_name("?g@ns@@YAHHH@Z"), Some("ns::g".to_string()));
    }

    #[test]
    fn msvc_deeply_nested_and_constructor() {
        // Deep nesting `?baz@A@B@C@@QEAAHHH@Z` = `C::B::A::baz(int, int)` — every
        // `@`-segment becomes a `::` scope; no signature leakage.
        let n = demangle_name("?baz@A@B@C@@QEAAHHH@Z").expect("nested MSVC demangles");
        assert_eq!(n, "C::B::A::baz");
        assert!(!n.contains('('), "no signature leakage: {n}");
        assert!(!n.contains('@'), "no raw `@` leakage into the scope name: {n}");
        // A constructor `??0Bar@@QEAA@XZ` = `Bar::Bar(void)`.
        assert_eq!(demangle_name("??0Bar@@QEAA@XZ"), Some("Bar::Bar".to_string()));
    }

    #[test]
    fn msvc_data_symbol() {
        // `?x@@3HA` = `int x` — a global variable (not a function). NAME_ONLY
        // yields the bare name; the raw form keeps the type.
        assert_eq!(demangle_name("?x@@3HA"), Some("x".to_string()));
        assert_eq!(demangle_raw("?x@@3HA"), Some("int x".to_string()));
    }

    #[test]
    fn msvc_question_mark_not_routed_to_itanium() {
        // Regression guard: a leading `?` is recognized by `skip()` as MSVC (NOT
        // treated as a versioned `@` symbol, even though MSVC names embed `@`),
        // and the Itanium/Rust crates never see it. A non-MSVC `?`-garbage string
        // still yields None (msvc-demangler rejects it), never a panic.
        assert!(!skip("?foo@Bar@@QEAAXXZ"), "MSVC name must not be skipped");
        assert_eq!(demangle_name("?not a real mangled name"), None);
        assert_eq!(demangle_raw("?"), None);
    }

    #[test]
    fn strip_bracket_groups_handles_nested_and_unbalanced() {
        assert_eq!(strip_bracket_groups("foo<int>::bar(int)"), "foo::bar");
        assert_eq!(
            strip_bracket_groups("a::b<c::d<e> >::f"),
            "a::b::f"
        );
        // tolerate an unbalanced trailing bracket (defensive)
        assert_eq!(strip_bracket_groups("foo::bar<"), "foo::bar");
    }
}
