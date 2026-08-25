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
//! ## Scope (names + namespaces here; the signature in [`kuna_cppsig`])
//!
//! This module delivers Ghidra's **name + namespace** application — the same
//! depth as the PLT pass ("correct names"). Applying the demangled **signature**
//! (the class type for `this` plus the declared parameter types) was the
//! deferred follow-up, and it now lives beside it in [`kuna_cppsig`]
//! (`--option cppsig`), which consumes [`demangle_raw`] and parks a prototype by
//! entry address. The split is deliberate: the name reduction is mandatory and
//! unconditional (the `::`-scope splitter depends on it), while the signature is
//! a *declaration* that can disagree with the code and is therefore gated.
//!
//! ## Output spelling (minor divergence, not a parity break)
//!
//! The demangled string spelling can differ slightly from libiberty c++filt
//! (e.g. `cpp_demangle` renders `std::__cxx11::basic_string`, where some c++filt
//! builds elide `__cxx11`). For the name-only path this only affects which
//! intermediate `::` scopes are created, not correctness of the base name.

/// (kuna `cppsig`) The signature arm — apply the demangled parameter types and
/// the class type for `this` to a function whose mangled symbol survives. This
/// is the "deferred follow-up" the Scope section above records, and the first
/// production consumer of [`demangle_raw`].
pub mod kuna_cppsig;

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
/// Rust is tried first when the symbol carries a Rust marker, then Itanium C++
/// ([`cpp_demangle`]), then Rust again for anything the marker test missed.
/// A result that is empty or unchanged (`== raw`) means "not actually mangled"
/// → `None`.
///
/// # Why the Rust-first arm exists
///
/// Rust's **legacy** scheme reuses the Itanium `_ZN…E` envelope, escaping the
/// characters Itanium identifiers cannot hold (`$LT$` for `<`, `$C$` for `,`,
/// `..` for `::`). A C++ demangler therefore *succeeds* on one — it sees a
/// perfectly well-formed nested-name whose components happen to contain dollar
/// signs — and returns the escapes verbatim:
///
/// ```text
/// _ZN4core3ptr100drop_in_place$LT$std..io..error..ErrorData$LT$…$GT$$GT$17h07f6…E
///   Itanium: core::ptr::drop_in_place$LT$std..io..error..ErrorData$LT$…$GT$$GT$
///   Rust:    core::ptr::drop_in_place<std::io::error::ErrorData<…>>
/// ```
///
/// Trying Itanium first therefore did not fall through to Rust — it produced a
/// wrong answer confidently, and every Rust binary rendered its own names as
/// escape soup. [`crate::sourcelang::is_rust_mangled`] identifies the two Rust
/// schemes exactly (a `_R` prefix, or the legacy `17h<16 hex>E` hash tail), so
/// the ordering is decided by a marker rather than by which crate answers first.
/// A C symbol carries neither marker, so this arm is unreachable for one.
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

    // Rust first when the symbol says it is Rust (see the fn docs: a legacy Rust
    // symbol is a well-formed Itanium name, so Itanium does not decline it).
    if crate::sourcelang::is_rust_mangled(raw) {
        if let Some(d) = demangle_rust(raw) {
            return Some(d);
        }
    }

    // Itanium C++.
    if let Ok(sym) = cpp_demangle::Symbol::new(raw) {
        if let Ok(d) = sym.demangle() {
            if !d.is_empty() && d != raw {
                return Some(d);
            }
        }
    }

    // Then Rust for anything the marker test missed.
    demangle_rust(raw)
}

/// Rust legacy (`_ZN…`) and v0 (`_R…`); `{:#}` drops the trailing hash.
fn demangle_rust(raw: &str) -> Option<String> {
    let d = rustc_demangle::try_demangle(raw).ok()?;
    let s = format!("{:#}", d);
    (!s.is_empty() && s != raw).then_some(s)
}

/// Reduce a demangled **Rust** path to a plain `::`-separated one.
///
/// Rust demangles to text that is a *type expression*, not an identifier path:
/// generic arguments (`drop_in_place<Vec<u8>>`) and trait-impl qualifiers
/// (`<aes::Aes256 as crypto_common::KeyInit>::new`) are part of the name. Both
/// have to go before the name can be a symbol, and they have to go *differently*:
///
/// * a generic argument list carries no path, so the whole `<…>` group is
///   dropped (with a preceding `::` if the demangler emitted a turbofish);
/// * a trait qualifier carries TWO paths, and dropping the group would leave a
///   leading `::` — an empty scope component, which the symbol table rejects
///   outright ("Non-global scope has empty name"). `<X as Y>` keeps the
///   **type** `X`, so the method stays attached to the type that defines it;
///   `<impl X as Y>` keeps the **trait** `Y`, matching the reference
///   implementation (SEFCOM Oxidizer's `normalize`, `angr/rust/utils/demangler.py`).
///
/// Angle brackets nest, so this is a depth-tracking scan rather than a regex: an
/// ` as ` separator only counts at depth 1 of the group being resolved.
/// Iterated to a fixed point, because resolving one group can expose another.
#[derive(Clone, Copy)]
enum Sep {
    /// `<Type as Trait>` -- a trait-qualified method.
    As,
    /// `<impl Trait for Type>` -- an inherent impl block.
    For,
}

pub fn normalize_rust_name(name: &str) -> String {
    let mut cur = name.to_string();
    for _ in 0..16 {
        let Some(open) = cur.find('<') else { break };
        let bytes = cur.as_bytes();
        let mut depth = 0usize;
        let mut close = None;
        let mut sep: Option<(usize, usize, Sep)> = None;
        for (i, &b) in bytes.iter().enumerate().skip(open) {
            match b {
                b'<' => depth += 1,
                b'>' => {
                    depth -= 1;
                    if depth == 0 {
                        close = Some(i);
                        break;
                    }
                }
                    _ => {}
            }
            // ` as ` (trait qualifier) or ` for ` (inherent impl) at depth 1
            // separates the two paths the group carries.
            if depth == 1 && sep.is_none() {
                if cur[i..].starts_with(" as ") {
                    sep = Some((i, 4, Sep::As));
                } else if cur[i..].starts_with(" for ") {
                    sep = Some((i, 5, Sep::For));
                }
            }
        }
        // An unbalanced `<` cannot be resolved; leave the rest alone.
        let Some(close) = close else { break };
        let inner = &cur[open + 1..close];
        let replacement = match sep {
            // `<X as Y>::m` keeps the TYPE, so the method stays attached to the
            // type that defines it; `<impl X as Y>::m` keeps the trait, matching
            // the reference implementation.
            Some((a, w, Sep::As)) => {
                let lhs = cur[open + 1..a].trim();
                let rhs = cur[a + w..close].trim();
                if lhs.starts_with("impl ") { rhs.to_string() } else { lhs.to_string() }
            }
            // `<impl Trait for Type>::m` keeps the TYPE, for the same reason.
            Some((a, w, Sep::For)) => cur[a + w..close].trim().to_string(),
            None => {
                let _ = inner;
                String::new()
            }
        };
        // A generic list may be introduced by a turbofish; drop that too so the
        // result does not end in a dangling `::`.
        let mut start = open;
        if replacement.is_empty() && cur[..open].ends_with("::") {
            start = open - 2;
        }
        cur = format!("{}{}{}", &cur[..start], replacement, &cur[close + 1..]);
    }
    // Any empty component left behind (a group that resolved to nothing in the
    // middle of a path) would be an empty scope.
    let joined = cur
        .split("::")
        .filter(|c| !c.trim().is_empty())
        .collect::<Vec<_>>()
        .join("::");
    joined
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
            let reduced = strip_bracket_groups(&name_anonymous_namespaces(&d));
            if !reduced.is_empty() && reduced != raw {
                return Some(reduced);
            }
        }
        return None;
    }

    // Rust first when the symbol says it is Rust. A legacy Rust `_ZN..` symbol is
    // *also* a well-formed Itanium name, so cpp_demangle does not decline it --
    // it returns the Rust escapes verbatim (`$LT$` for `<`, `$C$` for `,`, `..`
    // for `::`), which is a wrong answer rather than a missing one, and which
    // `strip_bracket_groups` then cannot see because there are no real brackets
    // to strip. See `demangle_raw`'s docs for the worked example.
    if crate::sourcelang::is_rust_mangled(raw) {
        if let Ok(d) = rustc_demangle::try_demangle(raw) {
            // `normalize_rust_name`, not `strip_bracket_groups`: a trait-impl
            // qualifier has to KEEP one of its two paths, and deleting the group
            // would leave a leading `::` the symbol table rejects.
            let reduced = strip_legacy_rust_hash(&normalize_rust_name(&format!("{:#}", d)));
            if !reduced.is_empty() && reduced != raw {
                return Some(reduced);
            }
        }
    }

    // Itanium C++, name-only options.
    if let Ok(sym) = cpp_demangle::Symbol::new(raw) {
        let opts = cpp_demangle::DemangleOptions::new()
            .no_params()
            .no_return_type();
        if let Ok(d) = sym.demangle_with_options(&opts) {
            let reduced = strip_legacy_rust_hash(&strip_bracket_groups(&name_anonymous_namespaces(&d)));
            if !reduced.is_empty() && reduced != raw {
                return Some(reduced);
            }
        }
    }

    // Then Rust (`{:#}` already drops the hash; the extra strip is defensive, and
    // so is the anonymous-namespace rewrite -- no Rust rendering spells one, but a
    // legacy `_ZN..` symbol reaches the Itanium arm above first either way).
    if let Ok(d) = rustc_demangle::try_demangle(raw) {
        let reduced = strip_legacy_rust_hash(&strip_bracket_groups(&name_anonymous_namespaces(
            &format!("{:#}", d),
        )));
        if !reduced.is_empty() && reduced != raw {
            return Some(reduced);
        }
    }

    None
}

/// The identifier kuna nests an **anonymous namespace** under, matching the
/// spelling `analyzers/rtti/kuna_itaniumrtti.rs::sanitize_class_name` already
/// gives the same construct (`(anonymous namespace)::Hidden` ->
/// `anonymous_namespace::Hidden`).
const ANONYMOUS_NAMESPACE: &str = "anonymous_namespace";

/// The two spellings a demangler renders an anonymous namespace with: Itanium
/// (`_GLOBAL__N_…`, gcc/clang/MinGW) parenthesizes it, MSVC (`?A0x…@`)
/// backtick-quotes it.
const ANONYMOUS_NAMESPACE_SPELLINGS: [&str; 2] =
    ["(anonymous namespace)", "`anonymous namespace'"];

/// Rewrite every anonymous-namespace spelling in a demangled name to the
/// [`ANONYMOUS_NAMESPACE`] identifier.
///
/// An anonymous namespace is a genuine **name component** —
/// `leveldb::(anonymous namespace)::HandleDumpCommand` is three of them — but
/// neither spelling is an identifier, and both break the name-only contract in
/// their own way:
///
/// - The Itanium spelling is a parenthesized group, so [`strip_bracket_groups`]
///   deletes it whole and leaves an **empty** component
///   (`leveldb::::HandleDumpCommand`). `Database::find_create_scope_from_symbol_name`
///   splits on every `::` and rejects the empty piece outright — a
///   `Non-global scope has empty name` `LowlevelError` that escapes
///   `bootstrap_from_object` and aborts the load of the **entire binary**, so a
///   single such symbol makes every command (`decompile`, `decompile-all`,
///   `functions`, `decompile-project`) fail on it. An anonymous namespace is the
///   ordinary way C++ gives a definition internal linkage, so this took out a
///   large share of real unstripped C++ binaries.
/// - The MSVC spelling survives stripping (backticks are not a bracket group),
///   but reaches emitted C as a scope name carrying a space and two backticks.
///
/// Applied to the name-only form only; [`demangle_raw`] keeps the faithful
/// c++filt/`cl.exe` text it is asked for.
fn name_anonymous_namespaces(name: &str) -> String {
    let mut out = name.to_string();
    for spelling in ANONYMOUS_NAMESPACE_SPELLINGS {
        if out.contains(spelling) {
            out = out.replace(spelling, ANONYMOUS_NAMESPACE);
        }
    }
    out
}

/// The C++ operator names spelled with bracket characters, longest first so a
/// prefix never shadows a longer spelling (`<<=` before `<<` before `<`).
///
/// These are *part of the name* — `operator[]` and `operator()` are as much a
/// function's identity as `push_back` — but they are made of the same
/// characters [`strip_bracket_groups`] removes, so they need the exemption
/// below.
const BRACKET_OPERATORS: &[&str] =
    &["<=>", "<<=", ">>=", "->*", "()", "[]", "<<", ">>", "<=", ">=", "->", "<", ">"];

/// Whether `out` ends with the head of a C++ operator name, so a bracket run
/// starting here spells the operator rather than opening a group. The three
/// heads are `operator`, `operator new` and `operator delete` (the latter two
/// take a `[]` of their own). Requires a word boundary, so an identifier that
/// merely ends in "operator" (`my_operator`) is not treated as one.
fn ends_with_operator_head(out: &str) -> bool {
    let tail = out.trim_end();
    ["operator new", "operator delete", "operator"].iter().any(|head| {
        tail.strip_suffix(head).is_some_and(|before| {
            before.chars().next_back().is_none_or(|c| !c.is_alphanumeric() && c != '_')
        })
    })
}

/// Remove every balanced bracketed group — `<...>` (template args), `(...)`
/// (signature), `[...]` (array/attribute) — from a demangled name, keeping only
/// the qualified-name text. Depth-tracked so nested groups
/// (`vector<pair<int, int> >`) are dropped whole; unbalanced trailing brackets
/// are tolerated. Leaves only the `::`-qualified name (e.g.
/// `std::vector<int, std::allocator<int> >::push_back(int const&)` ->
/// `std::vector::push_back`).
///
/// **Operator names are exempt.** `MapClass::operator[](Cell &)` reduces to
/// `MapClass::operator[]`, not to `MapClass::operator`: the brackets after an
/// operator head spell the operator, and eating them collapses every
/// bracket-spelled overload a class has — `[]`, `()`, `<`, `<<`, `>`, `->` —
/// onto one indistinguishable name. The exemption copies the operator's own
/// spelling verbatim and then resumes stripping, so the parameter list that
/// follows it still goes (`operator[](Cell &)` -> `operator[]`). A `<` or `>`
/// immediately followed by an identifier character is left to the generic path,
/// where it is the start of a template-argument list rather than the operator.
fn strip_bracket_groups(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut depth: i32 = 0;
    let mut i = 0usize;
    while i < s.len() {
        if depth == 0 && ends_with_operator_head(&out) {
            if let Some(spelling) = BRACKET_OPERATORS.iter().find(|sp| s[i..].starts_with(**sp)) {
                let rest = &s[i + spelling.len()..];
                let opens_template =
                    rest.chars().next().is_some_and(|c| c.is_alphanumeric() || c == '_');
                if !opens_template {
                    out.push_str(spelling);
                    i += spelling.len();
                    continue;
                }
            }
        }
        // cast: `i` indexes a char boundary by construction (the operator
        // spellings are ASCII and the fallback advances by one whole char).
        let c = s[i..].chars().next().expect("i is a char boundary");
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
        i += c.len_utf8();
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

/// Recover the **C++ class name** carried by an Itanium (GCC/Clang) `std::type_info`
/// object's mangled type-name string, or `None` if `typeinfo_name` does not name a
/// class.
///
/// The Itanium C++ ABI (§2.9.5) stores a polymorphic class's type name as the *bare
/// `<mangled-name>` component* — `6Widget`, `N7leveldb3EnvE`, `4VecIiE` — pointed at
/// by the second word of the `_ZTI…` typeinfo object, and separately symboled
/// `_ZTS…`. That component is not a symbol and no demangler accepts it alone. The
/// faithful recovery is the exact analog of the MSVC recipe
/// ([`demangle_rtti_class_name`], which wraps the `.?A…@@` string in `??_R0…@8`):
/// wrap the component back into the typeinfo-name symbol form `_ZTS<component>` and
/// demangle *that*, which Itanium renders as `typeinfo name for <qualified-class>`.
///
/// Reuses the **same** [`cpp_demangle`] crate the Itanium arm of [`demangle_name`]
/// already drives (no new demangler dependency).
///
/// Unlike [`demangle_name`] this returns the class name **verbatim**, template
/// arguments and all (`Vec<int>`, `(anonymous namespace)::Hidden`) — it does NOT
/// apply [`strip_bracket_groups`]. Reducing here would be lossy in a way that
/// matters for the caller: `Vec<int>` and `Vec<double>` are two distinct classes
/// with two distinct vtables, and collapsing both to `Vec` silently drops one
/// class's recovery. The caller owns turning this into an identifier
/// (`analyzers/rtti/kuna_itaniumrtti.rs::sanitize_class_name`), which is a labelling
/// concern rather than a demangling one.
///
/// A **leading `*`** is stripped first. The Itanium ABI (§2.9.1) marks a type whose
/// identity is local to one translation unit — an anonymous-namespace class, a class
/// defined inside a function — by prefixing its type-name string with `*`, telling
/// the runtime to compare `type_info`s by pointer rather than by string. The `*` is
/// not part of the mangled name, and leaving it on makes every such class
/// undemangleable; on a real C++ binary those are a large share of the concrete
/// implementation classes (`(anonymous namespace)::Hidden`).
///
/// Returns `None` for any string that does not demangle to the
/// `typeinfo name for …` shape — never panics.
pub fn demangle_typeinfo_name(typeinfo_name: &str) -> Option<String> {
    /// The prefix Itanium renders an `_ZTS…` symbol with.
    const RENDERED: &str = "typeinfo name for ";
    let typeinfo_name = typeinfo_name.strip_prefix('*').unwrap_or(typeinfo_name);
    if typeinfo_name.is_empty() {
        return None;
    }
    let sym = cpp_demangle::Symbol::new(format!("_ZTS{typeinfo_name}")).ok()?;
    let d = sym.demangle().ok()?;
    let class = d.strip_prefix(RENDERED)?.trim();
    if class.is_empty() {
        None
    } else {
        Some(class.to_string())
    }
}

#[cfg(test)]
mod tests {
    /// The two group kinds a Rust demangling carries, and the reason they cannot
    /// be handled the same way: a generic list is dropped, but a trait or impl
    /// qualifier has to KEEP one of its two paths, or the result begins with `::`
    /// and the symbol table rejects the empty scope.
    #[test]
    fn normalize_rust_name_resolves_generics_and_qualifiers() {
        for (input, want) in [
            ("core::ptr::drop_in_place<Vec<u8>>", "core::ptr::drop_in_place"),
            ("<aes::autodetect::Aes256 as crypto_common::KeyInit>::new", "aes::autodetect::Aes256::new"),
            ("<impl core::fmt::Debug for u8>::fmt", "u8::fmt"),
            ("alloc::vec::Vec<T,A>::resize", "alloc::vec::Vec::resize"),
            ("core::ptr::drop_in_place<core::result::Result<(),std::io::error::Error>>", "core::ptr::drop_in_place"),
            ("<cbc::decrypt::Decryptor<C> as crypto_common::InnerIvInit>::inner_iv_init", "cbc::decrypt::Decryptor::inner_iv_init"),
            ("plain::name", "plain::name"),
            // A name that resolves to nothing in the middle must not leave an
            // empty component behind.
            ("a::<T>::b", "a::b"),
        ] {
            assert_eq!(super::normalize_rust_name(input), want, "input={input}");
        }
    }

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

    /// The Itanium `_ZTS…` type-name component recovers its class VERBATIM — template
    /// arguments retained, so two instantiations stay two classes.
    #[test]
    fn typeinfo_name_itanium() {
        assert_eq!(demangle_typeinfo_name("5Shape").as_deref(), Some("Shape"));
        assert_eq!(demangle_typeinfo_name("N7leveldb3EnvE").as_deref(), Some("leveldb::Env"));
        assert_eq!(demangle_typeinfo_name("3VecIiE").as_deref(), Some("Vec<int>"));
        assert_eq!(demangle_typeinfo_name("3VecIdE").as_deref(), Some("Vec<double>"));
        // ABI §2.9.1: a leading `*` marks a TU-local type; it is not part of the name.
        assert_eq!(
            demangle_typeinfo_name("*N12_GLOBAL__N_16HiddenE").as_deref(),
            Some("(anonymous namespace)::Hidden")
        );
        // Non-type-name strings are rejected (no panic, no junk name).
        assert_eq!(demangle_typeinfo_name(""), None);
        assert_eq!(demangle_typeinfo_name("*"), None);
        assert_eq!(demangle_typeinfo_name("not a mangled name"), None);
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

    /// An anonymous namespace survives the name-only reduction as an
    /// identifier component. Before this, `strip_bracket_groups` deleted the
    /// whole `(anonymous namespace)` group and left an EMPTY `::` component,
    /// which `Database::find_create_scope_from_symbol_name` rejects with
    /// `Non-global scope has empty name` — aborting the load of the entire
    /// binary, for every command.
    #[test]
    fn anonymous_namespace_is_named_not_deleted() {
        // Nested under a real namespace (leveldb, and the MinGW libstdc++ shim
        // that took out a whole malware DLL).
        assert_eq!(
            demangle_name("_ZN7leveldb12_GLOBAL__N_117HandleDumpCommandEiPPc"),
            Some("leveldb::anonymous_namespace::HandleDumpCommand".to_string())
        );
        assert_eq!(
            demangle_name("_ZNKSt13__facet_shims12_GLOBAL__N_112collate_shimIcE10do_compareEPKcS4_S4_S4_"),
            Some("std::__facet_shims::anonymous_namespace::collate_shim::do_compare".to_string())
        );
        // At the top level the group was the FIRST component, so the reduction
        // used to yield a leading `::`.
        assert_eq!(
            demangle_name("_ZN12_GLOBAL__N_122get_locale_cache_mutexEv"),
            Some("anonymous_namespace::get_locale_cache_mutex".to_string())
        );
        // A function-local static inside one (`_ZZ..E..`).
        assert_eq!(
            demangle_name("_ZZN12_GLOBAL__N_122get_locale_cache_mutexEvE18locale_cache_mutex"),
            Some("anonymous_namespace::get_locale_cache_mutex::locale_cache_mutex".to_string())
        );
        // No component is empty and no bracket text leaks, for any of them.
        for raw in [
            "_ZN7leveldb12_GLOBAL__N_117HandleDumpCommandEiPPc",
            "_ZN12_GLOBAL__N_14PoolIiE5allocEi",
            "_ZN12_GLOBAL__N_1L16get_locale_mutexEv",
        ] {
            let n = demangle_name(raw).expect("demangles");
            assert!(!n.split("::").any(|c| c.is_empty()), "empty component: {n}");
            assert!(!n.contains('('), "bracket leakage: {n}");
            assert!(!n.contains(')'), "bracket leakage: {n}");
        }
        // The full c++filt form is NOT rewritten — it is asked for verbatim.
        assert_eq!(
            demangle_raw("_ZN7leveldb12_GLOBAL__N_117HandleDumpCommandEiPPc"),
            Some("leveldb::(anonymous namespace)::HandleDumpCommand(int, char**)".to_string())
        );
    }

    /// The three symbols `g++` really emits for the checked-in
    /// `anon_namespace_x86_64` fixture — a top-level anonymous namespace, one
    /// nested inside a named namespace, and a class defined inside that one.
    #[test]
    fn anonymous_namespace_fixture_symbols() {
        assert_eq!(
            demangle_name("_ZN12_GLOBAL__N_110top_helperEi"),
            Some("anonymous_namespace::top_helper".to_string())
        );
        assert_eq!(
            demangle_name("_ZN5outer12_GLOBAL__N_113nested_helperEi"),
            Some("outer::anonymous_namespace::nested_helper".to_string())
        );
        assert_eq!(
            demangle_name("_ZN5outer12_GLOBAL__N_16Widget4emitEi"),
            Some("outer::anonymous_namespace::Widget::emit".to_string())
        );
    }

    /// Two translation units that each define `helper` in an anonymous namespace
    /// demangle to the SAME name, and that is intended, not a defect to fix: the
    /// Itanium mangling itself is identical for both (`_ZN12_GLOBAL__N_16helperEv`
    /// either way), so no demangler can separate them. The name-only contract
    /// already collides this way on templates (`Vec<int>` and `Vec<double>` both
    /// reduce to `Vec`), and the addresses stay distinct, which is what every
    /// resolver keys on.
    #[test]
    fn anonymous_namespace_collides_across_units_by_design() {
        // The collision is upstream of any demangler: `helper` in an anonymous
        // namespace mangles to this one string in EVERY translation unit, so the
        // one name below is what all of them reduce to.
        assert_eq!(
            demangle_name("_ZN12_GLOBAL__N_16helperEv"),
            Some("anonymous_namespace::helper".to_string())
        );
        // What must NOT collide is two anonymous namespaces with different
        // parents -- the rewrite names the component, it does not flatten the
        // path, so these stay two distinct scopes.
        assert_ne!(
            demangle_name("_ZN12_GLOBAL__N_16helperEv"),
            demangle_name("_ZN5outer12_GLOBAL__N_16helperEv")
        );
        assert_eq!(
            demangle_name("_ZN5outer12_GLOBAL__N_16helperEv"),
            Some("outer::anonymous_namespace::helper".to_string())
        );
        // A name that already spells the identifier round-trips unchanged, so a
        // second pass over a rewritten name is a no-op.
        assert_eq!(name_anonymous_namespaces("anonymous_namespace::f"), "anonymous_namespace::f");
        assert_eq!(
            name_anonymous_namespaces("(anonymous namespace)::f"),
            "anonymous_namespace::f"
        );
    }

    /// MSVC spells the same construct with backticks, which no bracket-group
    /// strip touches: it survived, but as a scope name carrying a space and two
    /// backticks. It collapses to the same identifier the Itanium form does, so
    /// one binary's anonymous namespace is spelled like another's.
    #[test]
    fn msvc_anonymous_namespace_is_the_same_identifier() {
        assert_eq!(
            demangle_name("?foo@Bar@?A0x12345678@@QEAAXXZ"),
            Some("anonymous_namespace::Bar::foo".to_string())
        );
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

    /// An operator's own brackets are part of its name and survive; the
    /// parameter list that follows them does not. Without the exemption every
    /// bracket-spelled overload of a class collapsed onto the same
    /// `Class::operator`.
    #[test]
    fn strip_bracket_groups_keeps_operator_spellings() {
        for (input, want) in [
            ("MapClass::operator[](Cell &)", "MapClass::operator[]"),
            ("Random2Class::operator()(int, int)", "Random2Class::operator()"),
            ("Foo::operator<<(int)", "Foo::operator<<"),
            ("Foo::operator<(Foo const&)", "Foo::operator<"),
            ("Foo::operator>=(Foo const&)", "Foo::operator>="),
            ("Foo::operator->()", "Foo::operator->"),
            ("Foo::operator->*(int)", "Foo::operator->*"),
            ("operator new[](unsigned int)", "operator new[]"),
            ("operator delete[](void *)", "operator delete[]"),
            // A `<` that opens a template argument list is NOT the operator,
            // even directly after the keyword.
            ("Foo::operator<int>", "Foo::operator"),
            // An identifier merely ending in "operator" keeps the old behavior.
            ("Foo::my_operator<int>(int)", "Foo::my_operator"),
            // Non-bracket operators were never affected.
            ("Foo::operator==(Foo const&)", "Foo::operator=="),
        ] {
            assert_eq!(strip_bracket_groups(input), want, "input {input:?}");
        }
    }

    /// End-to-end through both manglings: the same overload set, mangled by
    /// MSVC and by the Itanium ABI, reduces to distinct operator names.
    #[test]
    fn demangle_name_keeps_operator_spellings() {
        for (raw, want) in [
            // MSVC (`cl.exe`)
            ("??AMapClass@@QBEAAVCellClass@@ABVCell@@@Z", "MapClass::operator[]"),
            ("??RRandom2Class@@QAEHHH@Z", "Random2Class::operator()"),
            ("??6Foo@@QAEAAV0@H@Z", "Foo::operator<<"),
            ("??MFoo@@QBE_NABV0@@Z", "Foo::operator<"),
            ("??_U@YAPAXI@Z", "operator new[]"),
            ("??_V@YAXPAX@Z", "operator delete[]"),
            // Itanium (gcc/clang)
            ("_ZN3FooixEi", "Foo::operator[]"),
            ("_ZN3FooclEv", "Foo::operator()"),
            ("_ZN3FoolsEi", "Foo::operator<<"),
            // A plain method is untouched by the exemption.
            ("_ZN3Foo5plainEi", "Foo::plain"),
        ] {
            assert_eq!(demangle_name(raw).as_deref(), Some(want), "raw {raw:?}");
        }
    }
}
