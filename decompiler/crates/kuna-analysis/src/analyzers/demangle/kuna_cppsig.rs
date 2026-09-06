//! (kuna `cppsig`) Apply the DEMANGLED C++ signature — the class type for
//! `this`, plus the declared parameter types — to a function whose mangled
//! symbol survives.
//!
//! ## What the name-only demangler leaves on the table
//!
//! [`crate::demangle`] reduces a mangled symbol to its qualified NAME
//! ([`demangle_name`](crate::demangle::demangle_name)), and its own header
//! records the gap: *"It does NOT apply the demangled signature (parameter /
//! return types) … a deferred follow-up."* [`demangle_raw`] has kept the full
//! c++filt form since that PR and had no production caller. This module is that
//! caller.
//!
//! The payoff is a **stripped** binary. A C++ shared library exports its member
//! functions through `.dynsym`, so `_ZN7leveldb12TableBuilder10WriteBlockEPNS_12
//! BlockBuilderEPNS_11BlockHandleE` survives `strip` and carries a full
//! declaration that no amount of data-flow analysis can recover:
//!
//! ```text
//!  before   void WriteBlock(int8 *a0,unsigned long a1,unsigned long a2)
//!  after    void WriteBlock(TableBuilder *this,BlockBuilder *a1,BlockHandle *a2)
//! ```
//!
//! This is the mangled-symbol twin of the DWARF-sourced `cppproto` (#264), and
//! the two are complementary: DWARF is ground truth and wins wherever it reaches
//! (the commit applies `cppsig` FIRST and lets `cppproto` overwrite), while the
//! mangled symbol is all that is left once the debug info is gone.
//!
//! ## The `this` decision is the whole risk
//!
//! Itanium mangling does not distinguish a **static member function** from a
//! **non-static** one, nor either from a **namespaced free function**:
//! `leveldb::Status::OK()` and `leveldb::NewMemEnv(Env*)` mangle with the same
//! `_ZN…E` nested-name shape as `leveldb::Cache::~Cache()`. Guessing wrong does
//! not cost precision — it shifts EVERY parameter by one position, which is
//! strictly worse than leaving the function alone.
//!
//! So the pass computes two sets and lets the option pick:
//!
//! * **`proven`** (the default) — a `this` is only added when the mangling
//!   *entails* it, and only these three shapes do:
//!   1. a **constructor** (`C1`/`C2`/`C3`, demangled as `A::B::B(…)`),
//!   2. a **destructor** (`D0`/`D1`/`D2`, demangled as `A::B::~B(…)`),
//!   3. a **cv- or ref-qualified** member (`_ZNK…`, demangled with a trailing
//!      `const` / `volatile` / `&` / `&&`) — a qualifier that can only ever
//!      attach to an implicit object parameter.
//!   Plus the converse certainty: an **unqualified** name (`_Z3fooi` → `foo(int)`,
//!   no `::` at all) can have no implicit object parameter, so its declared
//!   parameters apply at position 0. Everything else is SKIPPED.
//! * **`inferred`** — additionally decides the ambiguous nested names from
//!   class evidence mined out of the binary's own symbol table: a scope that owns
//!   a constructor, a destructor, a cv-qualified member, or a `_ZTV`/`_ZTI`/`_ZTS`
//!   (vtable / typeinfo / typeinfo-name) symbol is a CLASS, so its members take
//!   `this`; a scope with no such evidence is a namespace, so its functions do
//!   not.
//!
//! Measured on google/leveldb (1329 mangled `.dynsym` FUNC symbols with DWARF
//! ground truth, 915 of which really do take `this`): `proven` is
//! **precision 1.0000 at recall 0.7093**, `inferred` is precision 0.9278 at
//! recall 0.9978. For calibration, Ghidra 12.1's own `this` decision on the same
//! binary — which resolves the ambiguity by comparing the mangled parameter count
//! against the count its analysis recovered (`DemangledFunction.isThisCall`) —
//! runs at precision 0.85.
//!
//! ## Why the string, not the AST
//!
//! Ghidra's GNU demangler support "only pre-filters candidate strings and parses
//! the c++filt-style text the native process prints" (`GnuDemanglerParser`), and
//! this module is the same shape: it parses the [`demangle_raw`] text. That keeps
//! the one demangler dependency the port already documents as a LOSS and avoids a
//! second, deeper coupling to `cpp_demangle`'s substitution-table AST.
//!
//! Overloaded operators are deliberately REJECTED: `operator<`, `operator>` and
//! `operator()` put unbalanced brackets into the demangled text, which is exactly
//! what the depth-tracking parse cannot survive, and the free operator templates
//! (`__gnu_cxx::operator==<…>(…)`) are the densest source of static/free-function
//! false positives.
//!
//! ## Origin (upstream Ghidra)
//!
//! `DemangledFunction.applyTo` + `GnuDemanglerAnalyzer`'s "Apply Function
//! Signatures" option. `resolveReturnType` is ported in spirit: Itanium encodes a
//! return type only for template functions, so upstream returns null and leaves
//! the function's own recovered return type in place — which is why
//! `leveldb::TableBuilder::NumEntries` still renders `undefined8` in Ghidra. Here
//! that is expressed as [`PrototypePieces::outtype`] `= None`, which
//! `Funcdata::apply_locked_prototype` reads as "lock the INPUT half only".

use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use object::read::{Object, ObjectSymbol};
use object::SymbolKind;

use kuna_base::types::{int4, uint4};
use kuna_decomp::dtype::{type_metatype, Datatype, TypeFactory};
use kuna_decomp::fspec::PrototypePieces;

use crate::demangle::demangle_raw;
use crate::pass::{AnalysisCtx, AnalysisOutput, AnalysisPass, CppSigFacts, Phase};

/// How certain the pass is that a symbol's first argument is an implicit `this`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ThisKind {
    /// The mangling entails an implicit object parameter (ctor / dtor /
    /// cv-qualified member / an explicit MSVC `__thiscall`).
    Proven,
    /// The mangling entails there is NO implicit object parameter (an unqualified
    /// name, or an explicit MSVC `__cdecl`/`__stdcall`).
    ProvenNone,
    /// A nested name that could be a non-static member, a static member, or a
    /// namespaced free function. Only `inferred` resolves these.
    Ambiguous,
}

/// One parsed demangled declaration.
#[derive(Debug)]
struct Decl {
    /// The `::`-qualified name (`leveldb::TableBuilder::WriteBlock`).
    qualified: String,
    /// The enclosing scope (`leveldb::TableBuilder`), empty for a global name.
    scope: String,
    /// Declared parameter types, in order, as demangled text.
    params: Vec<String>,
    /// Trailing `...`.
    varargs: bool,
    /// What the mangling proves about the implicit object parameter.
    this_kind: ThisKind,
    /// The unqualified CLASS name a `this` would point at (`TableBuilder`), when
    /// the name is nested. Ghidra names the placeholder structure with the bare
    /// class name and files it under the namespace category path, and the DWARF
    /// ground truth spells it the same way (`TableBuilder *`, not
    /// `leveldb::TableBuilder *`).
    class: String,
}

/// Apply the demangled C++ signature to every function whose mangled symbol the
/// object still carries.
pub struct CppSigPass;

impl AnalysisPass for CppSigPass {
    fn phase(&self) -> Phase {
        Phase::P1
    }

    fn id(&self) -> &'static str {
        "cppsig"
    }

    fn run(&self, ctx: &AnalysisCtx) -> AnalysisOutput {
        let mut out = AnalysisOutput::default();
        let mangled = mangled_function_symbols(ctx.file);
        if mangled.is_empty() {
            return out;
        }
        let decls: Vec<(u64, Decl)> = mangled
            .iter()
            .filter_map(|(addr, raw)| {
                let dem = demangle_raw(raw)?;
                Some((*addr, parse_decl(&dem)?))
            })
            .collect();
        if decls.is_empty() {
            return out;
        }
        let classes = class_scopes(ctx.file, &decls);
        let types = ctx.arch.types();
        let (_addr_size, word_size) = ctx.arch.data_org();
        let ptr = types.get_size_of_pointer();
        for (addr, decl) in &decls {
            let (has_this, proven) = match decl.this_kind {
                ThisKind::Proven => (true, true),
                ThisKind::ProvenNone => (false, true),
                ThisKind::Ambiguous => match infer_this(&decl.scope, &classes) {
                    Some(t) => (t, false),
                    None => continue,
                },
            };
            if has_this && decl.class.is_empty() {
                continue;
            }
            let Some(pieces) = build_pieces(decl, has_this, types, ptr, word_size) else {
                continue;
            };
            if proven {
                out.cpp_sig.proven.push((*addr, pieces));
            } else {
                out.cpp_sig.inferred.push((*addr, pieces));
            }
        }
        out
    }
}

/// Every defined FUNC symbol whose name is C++-mangled, as `(entry vma, raw name)`.
///
/// Both symbol tables are read: `.dynsym` is the one that survives `strip` on a
/// shared library (and carries the whole exported C++ API), `.symtab` adds the
/// internal linkage on an unstripped object. A `@VERSION` suffix is left alone —
/// `demangle_raw`'s `skip()` rejects a versioned name, and a versioned symbol is
/// a libc import, never a C++ definition.
fn mangled_function_symbols(file: &object::File) -> Vec<(u64, String)> {
    let mut seen: HashSet<(u64, String)> = HashSet::new();
    let mut out = Vec::new();
    for sym in file.symbols().chain(file.dynamic_symbols()) {
        if sym.kind() != SymbolKind::Text || sym.address() == 0 {
            continue;
        }
        let Ok(name) = sym.name() else { continue };
        if !(name.starts_with("_Z") || name.starts_with("__Z") || name.starts_with('?')) {
            continue;
        }
        let key = (sym.address(), name.to_string());
        if seen.insert(key.clone()) {
            out.push(key);
        }
    }
    out
}

/// Decide the implicit object parameter for an ambiguous nested name from the
/// class evidence in `classes`, or `None` when the binary says nothing either way
/// and the symbol must be left alone.
///
/// Both answers need POSITIVE evidence. `scope` itself being a known class means
/// its members take `this`. `scope` being a strict prefix of a known class scope
/// (`leveldb` is a prefix of `leveldb::Cache`) means it encloses one, so it is a
/// namespace and its functions do not. Treating "no evidence" as "namespace"
/// instead is what a small binary punishes: a class whose constructor is implicit
/// and whose destructor is trivial emits no witness at all, and assuming its one
/// exported member is a free function drops the `this` and shifts every parameter
/// LEFT — the same damage as inventing one, in the other direction. Measured on
/// google/leveldb this raises recall from 0.9978 to 1.0000 (both misses removed)
/// at unchanged precision, by declining 100 of 1322 symbols instead of guessing.
fn infer_this(scope: &str, classes: &HashSet<String>) -> Option<bool> {
    if scope.is_empty() {
        return Some(false);
    }
    if classes.contains(scope) {
        return Some(true);
    }
    let inner = format!("{scope}::");
    if classes.iter().any(|c| c.starts_with(&inner)) {
        return Some(false);
    }
    None
}

/// The scopes the binary's own symbols PROVE are classes rather than namespaces.
///
/// Three independent witnesses, all of which survive `strip` on a shared library:
/// a constructor or destructor filed under the scope, a cv-qualified member of
/// it, and a `_ZTV`/`_ZTI`/`_ZTS` (vtable / typeinfo / typeinfo-name) symbol
/// naming it. Only `inferred` consults this set.
fn class_scopes(file: &object::File, decls: &[(u64, Decl)]) -> HashSet<String> {
    let mut scopes: HashSet<String> = HashSet::new();
    for (_, d) in decls {
        if d.this_kind == ThisKind::Proven && !d.scope.is_empty() {
            scopes.insert(d.scope.clone());
        }
    }
    for sym in file.symbols().chain(file.dynamic_symbols()) {
        let Ok(name) = sym.name() else { continue };
        if !(name.starts_with("_ZTV") || name.starts_with("_ZTI") || name.starts_with("_ZTS")) {
            continue;
        }
        // `_ZTVN7leveldb5CacheE` demangles to `vtable for leveldb::Cache`; the
        // class is the text after the `for `.
        let Some(dem) = demangle_raw(name) else { continue };
        if let Some((_, cls)) = dem.split_once(" for ") {
            let cls = cls.trim();
            if !cls.is_empty() && !cls.contains('(') {
                scopes.insert(strip_template_args(cls));
            }
        }
    }
    scopes
}

// ---------------------------------------------------------------------------
// Parsing the demangled declaration (the `GnuDemanglerParser` analog)
// ---------------------------------------------------------------------------

/// Parse a full c++filt-style demangled string into a [`Decl`], or `None` when
/// the shape is one this pass refuses to act on.
fn parse_decl(dem: &str) -> Option<Decl> {
    // An overloaded operator puts unbalanced `<`/`>`/`(` into the text, which the
    // depth-tracking split below cannot survive, and the free operator templates
    // are the densest source of false `this` positives. Reject them outright.
    if dem.contains("operator") {
        return None;
    }
    // A `_ZT*`/`_ZG*` special name ("vtable for X", "typeinfo for X", "guard
    // variable for X") is not a function declaration.
    if dem.contains(" for ") {
        return None;
    }
    let (prefix, params_text, suffix) = signature_parens(dem)?;
    if !is_cv_ref_only(suffix) {
        // Anything after the parameter list other than cv/ref qualifiers means
        // the group we found is not the signature (a function returning a
        // function pointer, a pointer-to-member declarator, ...).
        return None;
    }
    let qualified = last_token(prefix.trim());
    // A declarator (`(*foo(int))`) or a leftover type token is not a name.
    if qualified.is_empty()
        || qualified.contains(['(', ')', '*', '&', '[', ']'])
        || qualified.starts_with("::")
    {
        return None;
    }
    let comps = split_top_level(&qualified);
    // An explicit function-template specialization (`pair<X&, Y&, true>(…)`,
    // `maxof<int>(…)`) is refused: its demangled parameter list is not
    // trustworthy. Measured on leveldb, `cpp_demangle` renders
    // `_ZNSt4pairI…EC1IRS1_S4_Lb1EEEOT_OT0_` — a two-parameter forwarding
    // constructor — with ONE parameter, and a short parameter list is exactly
    // what leaves a live argument register undeclared. It is also where the
    // module's known template collision lives (`maxof<int>` and the `double`
    // instantiation both reduce to `maxof`).
    if comps[comps.len() - 1].contains('<') {
        return None;
    }
    let cv = suffix.contains("const") || suffix.contains("volatile") || suffix.contains('&');
    // The MSVC arm answers the whole question outright, unlike Itanium: the
    // demangled form carries the ACCESS SPECIFIER (only a class member has one),
    // the `static` keyword, and the calling convention.  A 32-bit MSVC member
    // (`__thiscall`) passes `this` in ECX rather than as ordinary argument 0, so
    // that one combination is refused rather than mis-placed — selecting the
    // `__thiscall` prototype model (registered by #265) is the follow-up.
    let msvc = prefix.contains("__cdecl")
        || prefix.contains("__thiscall")
        || prefix.contains("__stdcall")
        || prefix.contains("__fastcall")
        || prefix.contains("__vectorcall");
    let msvc_member = prefix.starts_with("public: ")
        || prefix.starts_with("private: ")
        || prefix.starts_with("protected: ");
    let msvc_static = msvc_member && prefix.contains(" static ");
    if msvc && msvc_member && !msvc_static && prefix.contains("__thiscall") {
        return None;
    }

    let (this_kind, class) = if comps.len() < 2 {
        (ThisKind::ProvenNone, String::new())
    } else {
        let func = strip_template_args(comps[comps.len() - 1]);
        let class = strip_template_args(comps[comps.len() - 2]);
        let kind = if msvc {
            if msvc_member && !msvc_static {
                ThisKind::Proven
            } else {
                ThisKind::ProvenNone
            }
        } else if func.starts_with('~') || (!class.is_empty() && func == class) || cv {
            ThisKind::Proven
        } else {
            ThisKind::Ambiguous
        };
        (kind, class)
    };
    let scope = if comps.len() < 2 {
        String::new()
    } else {
        comps[..comps.len() - 1].join("::")
    };

    let mut params: Vec<String> = Vec::new();
    let mut varargs = false;
    for p in split_params(params_text) {
        let p = p.trim();
        if p.is_empty() || p == "void" {
            continue;
        }
        if p == "..." {
            varargs = true;
            continue;
        }
        if varargs {
            // A parameter after `...` is not a C signature.
            return None;
        }
        params.push(p.to_string());
    }

    Some(Decl {
        qualified,
        scope,
        params,
        varargs,
        this_kind,
        class,
    })
}

/// Locate the top-level parameter list: `(prefix, params, suffix)` around the
/// LAST depth-0 `(...)` group, or `None` when the string has none.
fn signature_parens(dem: &str) -> Option<(&str, &str, &str)> {
    let b = dem.as_bytes();
    let mut depth: i32 = 0;
    let mut starts: Vec<usize> = Vec::new();
    let mut last: Option<(usize, usize)> = None;
    for (i, &c) in b.iter().enumerate() {
        match c {
            b'<' | b'[' => depth += 1,
            b'>' | b']' => depth -= 1,
            b'(' => {
                if depth == 0 {
                    starts.push(i);
                }
                depth += 1;
            }
            b')' => {
                depth -= 1;
                if depth == 0 {
                    if let Some(s) = starts.pop() {
                        last = Some((s, i));
                    }
                }
            }
            _ => {}
        }
    }
    let (s, e) = last?;
    Some((&dem[..s], &dem[s + 1..e], &dem[e + 1..]))
}

/// Is `s` nothing but cv/ref qualifiers and whitespace (the legal tail of a
/// member function declaration)?
fn is_cv_ref_only(s: &str) -> bool {
    let mut rest = s.trim();
    while !rest.is_empty() {
        let next = if let Some(r) = rest.strip_prefix("const") {
            r
        } else if let Some(r) = rest.strip_prefix("volatile") {
            r
        } else if let Some(r) = rest.strip_prefix("&&") {
            r
        } else if let Some(r) = rest.strip_prefix('&') {
            r
        } else if let Some(r) = rest.strip_prefix("noexcept") {
            r
        } else {
            return false;
        };
        rest = next.trim_start();
    }
    true
}

/// The last space-separated token at bracket depth 0 — the qualified name, with
/// any return type in front of it dropped.
fn last_token(s: &str) -> String {
    let b = s.as_bytes();
    let mut depth: i32 = 0;
    let mut cut = 0usize;
    for (i, &c) in b.iter().enumerate() {
        match c {
            b'<' | b'(' | b'[' => depth += 1,
            b'>' | b')' | b']' => depth -= 1,
            b' ' if depth == 0 => cut = i + 1,
            _ => {}
        }
    }
    s[cut..].to_string()
}

/// Split a qualified name on every depth-0 `::`.
fn split_top_level(s: &str) -> Vec<&str> {
    let b = s.as_bytes();
    let mut depth: i32 = 0;
    let mut out = Vec::new();
    let mut start = 0usize;
    let mut i = 0usize;
    while i < b.len() {
        match b[i] {
            b'<' | b'(' | b'[' => depth += 1,
            b'>' | b')' | b']' => depth -= 1,
            b':' if depth == 0 && i + 1 < b.len() && b[i + 1] == b':' => {
                out.push(&s[start..i]);
                i += 2;
                start = i;
                continue;
            }
            _ => {}
        }
        i += 1;
    }
    out.push(&s[start..]);
    out
}

/// Split a parameter list on every depth-0 `,`.
fn split_params(s: &str) -> Vec<&str> {
    if s.trim().is_empty() {
        return Vec::new();
    }
    let b = s.as_bytes();
    let mut depth: i32 = 0;
    let mut out = Vec::new();
    let mut start = 0usize;
    for (i, &c) in b.iter().enumerate() {
        match c {
            b'<' | b'(' | b'[' => depth += 1,
            b'>' | b')' | b']' => depth -= 1,
            b',' if depth == 0 => {
                out.push(&s[start..i]);
                start = i + 1;
            }
            _ => {}
        }
    }
    out.push(&s[start..]);
    out
}

/// Drop every balanced `<...>` group, leaving the bare name (`basic_string<char,
/// …>` -> `basic_string`).
fn strip_template_args(s: &str) -> String {
    let mut depth: i32 = 0;
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '<' => depth += 1,
            '>' => depth -= 1,
            _ if depth <= 0 => out.push(c),
            _ => {}
        }
    }
    out.trim().to_string()
}

// ---------------------------------------------------------------------------
// Building the prototype
// ---------------------------------------------------------------------------

/// Assemble the [`PrototypePieces`], or `None` when any declared parameter is a
/// type this pass will not model.
///
/// `outtype` is deliberately `None`: Itanium encodes a return type only for a
/// template function, so there is nothing to apply, and inventing `void` would
/// DELETE the return value kuna's own recovery finds (upstream's
/// `resolveReturnType` returns null here for the same reason). `None` is read by
/// `Funcdata::apply_locked_prototype` as "lock the input half only".
fn build_pieces(
    decl: &Decl,
    has_this: bool,
    types: &dyn TypeFactory,
    ptr: int4,
    word_size: uint4,
) -> Option<PrototypePieces> {
    let mut intypes: Vec<Rc<Datatype>> = Vec::with_capacity(decl.params.len() + 1);
    let mut innames: Vec<String> = Vec::with_capacity(decl.params.len() + 1);
    if has_this {
        let cls = types.get_type_struct(&decl.class).ok()?;
        intypes.push(types.get_type_pointer(ptr, cls, word_size).ok()?);
        innames.push("this".to_string());
    }
    for p in &decl.params {
        intypes.push(build_param_type(p, types, ptr, word_size)?);
        innames.push(String::new());
    }
    if intypes.is_empty() && !decl.varargs {
        // Nothing to say about a `void`-parameter function that kuna does not
        // already derive; locking an empty input list would only suppress its own
        // recovery.
        return None;
    }
    let first_var_arg_slot = if decl.varargs { intypes.len() as int4 } else { -1 };
    Some(PrototypePieces {
        name: decl.qualified.clone(),
        outtype: None,
        intypes,
        innames,
        first_var_arg_slot,
        output_storage: None,
        input_storage: Vec::new(),
    })
}

/// Map one demangled parameter type to a kuna [`Datatype`], or `None` when the
/// declaration describes something this pass refuses to model.
///
/// The accepted grammar is deliberately narrow, because the demangled text
/// carries **declarations, not layouts**:
///
/// * a pointer or a reference of any depth — a reference is a pointer at the ABI
///   level, the same mapping `cppproto` gives `DW_TAG_reference_type`;
/// * a primitive, at its LP64/ILP32 width;
/// * an `enum`-shaped or class-shaped name only as a POINTEE, where a named
///   opaque structure is enough to render `Foo *` (Ghidra's
///   `createPlaceHolderStructure`).
///
/// An aggregate passed **by value** is refused outright: its size is not in the
/// mangling, a zero-width by-value parameter walks off the end of the parameter
/// storage model (the failure `cppproto` measured on
/// `std::vector<int>::_M_realloc_insert`), and getting the width wrong would
/// shift every following parameter. Arrays, function types and pointer-to-member
/// are refused for the same reason. Refusing ONE parameter drops the whole
/// signature — unlike the DWARF path there is no per-parameter fallback that
/// preserves position.
fn build_param_type(
    text: &str,
    types: &dyn TypeFactory,
    ptr: int4,
    word_size: uint4,
) -> Option<Rc<Datatype>> {
    let mut t = text.trim();
    if t.contains('(') || t.contains('[') || t.contains("::*") {
        return None; // function pointer, array, pointer-to-member
    }
    // Peel the declarator suffix: `char const* const*`, `Slice const&`, `int&&`.
    let mut indirection = 0usize;
    loop {
        let before = t.len();
        t = t.trim_end();
        if let Some(r) = t.strip_suffix('*') {
            indirection += 1;
            t = r;
        } else if let Some(r) = t.strip_suffix("&&") {
            indirection += 1;
            t = r;
        } else if let Some(r) = t.strip_suffix('&') {
            indirection += 1;
            t = r;
        } else if let Some(r) = strip_qualifier_word(t, "const") {
            t = r;
        } else if let Some(r) = strip_qualifier_word(t, "volatile") {
            t = r;
        } else if let Some(r) = strip_qualifier_word(t, "restrict") {
            t = r;
        }
        if t.len() == before {
            break;
        }
    }
    let mut base_text = t.trim();
    loop {
        let before = base_text.len();
        for q in ["const ", "volatile ", "restrict "] {
            base_text = base_text.strip_prefix(q).unwrap_or(base_text).trim_start();
        }
        if base_text.len() == before {
            break;
        }
    }
    let base = if let Some(prim) = primitive_type(base_text, types) {
        prim?
    } else if indirection == 0 {
        // A by-value aggregate whose width the mangling does not carry.
        return None;
    } else {
        // A class/struct/enum name, reachable only as a pointee. The bare
        // innermost component is what Ghidra names the placeholder structure and
        // what the DWARF ground truth spells (`Slice *`, not `leveldb::Slice *`).
        let comps = split_top_level(base_text);
        let name = strip_template_args(comps[comps.len() - 1]);
        if name.is_empty() || !is_identifier(&name) {
            return None;
        }
        types.get_type_struct(&name).ok()?
    };
    let mut ty = base;
    for _ in 0..indirection {
        ty = types.get_type_pointer(ptr, ty, word_size).ok()?;
    }
    Some(ty)
}

/// The C/C++ primitive spellings c++filt emits, at their LP64/ILP32 width.
///
/// Returns `None` when `text` is not a primitive at all (so the caller falls
/// through to the aggregate arm) and `Some(None)` when it is a primitive the type
/// factory declined to build (so the caller refuses the signature).
#[allow(clippy::type_complexity)]
fn primitive_type(text: &str, types: &dyn TypeFactory) -> Option<Option<Rc<Datatype>>> {
    let long = types.get_size_of_long();
    let int = types.get_size_of_int();
    let (size, meta) = match text {
        "void" => return Some(types.get_type_void().ok()),
        "bool" => (1, type_metatype::TYPE_BOOL),
        "char" | "signed char" => return Some(types.get_type_char(1).ok()),
        "unsigned char" => (1, type_metatype::TYPE_UINT),
        "wchar_t" => return Some(types.get_type_char(types.get_size_of_wchar()).ok()),
        "char8_t" => (1, type_metatype::TYPE_UINT),
        "char16_t" | "short" | "short int" => (2, type_metatype::TYPE_INT),
        "unsigned short" | "short unsigned int" => (2, type_metatype::TYPE_UINT),
        "char32_t" => (4, type_metatype::TYPE_UINT),
        "int" => (int, type_metatype::TYPE_INT),
        "unsigned int" => (int, type_metatype::TYPE_UINT),
        "long" | "long int" => (long, type_metatype::TYPE_INT),
        "unsigned long" | "long unsigned int" => (long, type_metatype::TYPE_UINT),
        "long long" | "long long int" => (8, type_metatype::TYPE_INT),
        "unsigned long long" | "long long unsigned int" => (8, type_metatype::TYPE_UINT),
        "__int128" => (16, type_metatype::TYPE_INT),
        "unsigned __int128" => (16, type_metatype::TYPE_UINT),
        "float" => (4, type_metatype::TYPE_FLOAT),
        "double" => (8, type_metatype::TYPE_FLOAT),
        "long double" => (16, type_metatype::TYPE_FLOAT),
        _ => return None,
    };
    Some(types.get_base(size, meta).ok())
}

/// Strip a trailing cv qualifier WORD, never a suffix of an identifier: `Slice
/// const` peels to `Slice`, `Xconst` does not peel at all.
fn strip_qualifier_word<'a>(s: &'a str, word: &str) -> Option<&'a str> {
    let r = s.strip_suffix(word)?;
    match r.chars().next_back() {
        None => Some(r),
        Some(c) if !c.is_ascii_alphanumeric() && c != '_' => Some(r),
        _ => None,
    }
}

/// A plain C identifier (what a placeholder structure may be named).
fn is_identifier(s: &str) -> bool {
    !s.is_empty()
        && !s.starts_with(|c: char| c.is_ascii_digit())
        && s.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
}

/// Fold the pass's two certainty tiers into the list the commit boundary applies,
/// under the resolved `--option cppsig` mode.
pub fn select(facts: CppSigFacts, inferred: bool) -> Vec<(u64, PrototypePieces)> {
    let CppSigFacts {
        proven,
        inferred: amb,
    } = facts;
    let mut out = proven;
    if inferred {
        out.extend(amb);
    }
    // Two symbols can alias one entry (a `C1`/`C2` constructor pair); the parked
    // prototype is per address, so keep the first and let the rest fall away
    // rather than re-parking the same function repeatedly.
    let mut seen: HashMap<u64, ()> = HashMap::new();
    out.retain(|(a, _)| seen.insert(*a, ()).is_none());
    out
}

#[cfg(test)]
mod tests;
