//! (kuna) Swift symbol demangling — a pure-Rust demangler for the stable
//! `$s` mangling and the Objective-C runtime's `_Tt` type names
//! (`docs/multilang.md`, the Swift pack's first prerequisite).
//!
//! ## Why a port and not a crate
//!
//! Every Swift demangler on crates.io builds Apple's C++ demangler
//! (`swift-demangler`, `symbolic-demangle`, `macho-demangle`, `razgad`,
//! `swift-demangle`): a C++ toolchain at build time, which `kuna-wasm`
//! cannot carry. There is no pure-Rust Swift demangler to depend on, so
//! this is the repo's usual move (the MSVC signature recovery precedent):
//! written from the public grammar (`swift/docs/ABI/Mangling.rst`) as the
//! same postfix stack machine Apple's reference implementation uses, and
//! verified against `swift-demangle` byte for byte on 131,000 symbols
//! (the Swift 6.3 stdlib and Foundation, and the symbol tables and
//! Objective-C class names of seven iOS apps). The remaining gap is the
//! autodiff family (`TJ…`), which never appears in app binaries.
//!
//! ## The contract: parse-or-passthrough
//!
//! Every name this module cannot derive with certainty comes back `None`
//! and the symbol keeps its mangled spelling: an unknown operator, a
//! malformed count, a stack that does not reduce, a node kind without a
//! rendering. Nothing here guesses a partial path.
//!
//! ## Two renderings
//!
//! - [`demangle_swift_full`]: the `swift-demangle` form, types included
//!   (`Swift.String.count.getter : Swift.Int`), for display.
//! - [`demangle_swift_name`]: the name-only path kuna installs as the
//!   symbol name — space-free, Swift reference syntax for members
//!   (`Toyota.Foo.init(x:y:)`, `Toyota.Foo.bar(_:)`, `Toyota.Foo.baz()`),
//!   dotted tags for runtime artifacts (`Toyota.Foo.type_metadata`,
//!   `….dispatch_thunk`, `….merged`, `….closure#1`), and `<Type:Protocol>`
//!   for conformances (`<Toyota.Foo:Swift.Hashable>.hash(into:)` is the
//!   protocol witness, `<…>.protocol_witness_table` the table). Private
//!   discriminators, generic signatures and parameter types are dropped;
//!   where a name is meaningless without types (conformances, generic
//!   specializations, reabstraction thunks) a compact spelling appears
//!   (`[Swift.String:Any]`, `(Swift.Int)->Swift.Bool`).
//!
//! The Objective-C runtime names (`_TtC6Toyota3Foo`, nested `_TtCC…`,
//! private `P33_<hash>`, generic `_TtGC…_`, protocol `_TtP…_`) parse into
//! the same tree, so `demangle_swift_name("_TtC6Toyota3Foo")` is
//! `Toyota.Foo`.

pub mod name;
pub mod node;
pub mod oldparse;
pub mod parse;
pub mod print;
pub mod punycode;

#[cfg(test)]
mod tests;

use std::rc::Rc;

use node::{Kind, Node, NodeRef};

/// Strip the Swift prefix. Returns the body and whether the Swift-4.0
/// (`_T0`) function-type convention applies.
fn strip_prefix(raw: &str) -> Option<(&str, bool)> {
    let bare = raw.strip_prefix('_').unwrap_or(raw);
    if let Some(b) = bare.strip_prefix("$s").or_else(|| bare.strip_prefix("$S")) {
        return Some((b, false));
    }
    if let Some(b) = bare.strip_prefix("_T0").or_else(|| raw.strip_prefix("_T0")) {
        return Some((b, true));
    }
    None
}

/// Parse `raw` into its demangling tree.
pub fn demangle_swift_tree(raw: &str) -> Option<NodeRef> {
    if let Some(body) = raw.strip_prefix("_Tt") {
        let ty = oldparse::parse_old_type(body)?;
        return Some(Node::with(Kind::Global, vec![Rc::clone(ty.first()?)]));
    }
    let (body, old) = strip_prefix(raw)?;
    if body.is_empty() || !body.is_ascii() {
        return None;
    }
    parse::Parser::new(body, old).parse_global()
}

/// Parse a mangled type name as the reflection metadata stores it (no
/// prefix; symbolic references with binary payloads resolved by
/// `resolver`) into its `Type` node, or `None` outside the coverage.
pub fn demangle_swift_type(
    mangled: &[u8],
    resolver: &parse::SymbolicResolver<'_>,
) -> Option<NodeRef> {
    if mangled.is_empty() {
        return None;
    }
    parse::Parser::with_resolver(mangled, false, Some(resolver)).parse_type()
}

/// The `swift-demangle` full form, or `None` outside the coverage.
pub fn demangle_swift_full(raw: &str) -> Option<String> {
    let tree = demangle_swift_tree(raw)?;
    print::print_full(&tree)
}

/// kuna's name-only form (`Module.Type.member(labels:)`), or `None` for a
/// name outside the coverage — never a guess.
pub fn demangle_swift_name(raw: &str) -> Option<String> {
    let tree = demangle_swift_tree(raw)?;
    name::print_name(&tree)
}
