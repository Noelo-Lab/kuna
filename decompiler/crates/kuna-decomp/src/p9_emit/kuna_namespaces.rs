//! (kuna) Which namespaces to print in front of a symbol's name.
//!
//! The port of `PrintC::pushSymbolScope`/`emitSymbolScope` (`printc.cc:202-256`)
//! and `Symbol::getResolutionDepth` (`database.cc:324`), computed over namespace
//! NAME paths: kuna's scopes for a symbol Ghidra describes over the wire, or that
//! a `::`-qualified console name implies, are not the live `Scope` tree upstream
//! walks, so both sides arrive as lists of names.

use crate::printlanguage::NamespaceStrategy;

/// `Symbol::getResolutionDepth(useScope)`: how many of the symbol's enclosing
/// scopes must be printed for its name to resolve where it is used.
///
/// `sym_scope_path` is the symbol's namespace chain, innermost first, global
/// excluded. `use_ns_path` is the namespace of the function being printed,
/// outermost first; its local scope sits below it. `name_used(name, levels)`
/// answers `useScope->isNameUsed(name, terminatingScope)`, where the
/// terminating scope is the symbol's ancestor `levels` namespaces below
/// global; a hit costs one more level. A depth one past `sym_scope_path`
/// reaches the global scope, which prints as a bare `::`.
pub fn resolution_depth(
    base: &str,
    sym_scope_path: &[String],
    use_ns_path: &[String],
    name_used: &dyn Fn(&str, usize) -> bool,
) -> usize {
    let mut sym_path: Vec<&str> = Vec::with_capacity(sym_scope_path.len() + 1);
    sym_path.push("");
    sym_path.extend(sym_scope_path.iter().rev().map(String::as_str));
    // The local scope marker never equals a namespace name.
    let mut use_path: Vec<&str> = Vec::with_capacity(use_ns_path.len() + 2);
    use_path.push("");
    use_path.extend(use_ns_path.iter().map(String::as_str));
    use_path.push("\0local");

    // Scope::findDistinguishingScope (database.cc:1486) on the name paths.
    let min = sym_path.len().min(use_path.len());
    let distinguish = match (0..min).find(|&i| sym_path[i] != use_path[i]) {
        Some(i) => Some(i),
        None if min < sym_path.len() => Some(min),
        None if min < use_path.len() => None,
        None => Some(sym_path.len() - 1),
    };
    let (mut depth, distinguish_name, terminating) = match distinguish {
        None => (0, base, sym_scope_path.len()),
        Some(i) => (sym_path.len() - i, sym_path[i], i - 1),
    };
    if name_used(distinguish_name, terminating) {
        depth += 1;
    }
    depth
}

/// The scope names to print in front of a symbol used inside the function
/// whose namespace is `use_ns_path`, outermost first; empty when the strategy
/// needs none. `""` stands for the global scope.
pub fn scope_prefix(
    strategy: NamespaceStrategy,
    base: &str,
    sym_scope_path: &[String],
    use_ns_path: &[String],
    name_used: &dyn Fn(&str, usize) -> bool,
) -> Vec<String> {
    let depth = match strategy {
        NamespaceStrategy::MinimalNamespaces => {
            resolution_depth(base, sym_scope_path, use_ns_path, name_used)
        }
        NamespaceStrategy::AllNamespaces => sym_scope_path.len(),
        NamespaceStrategy::NoNamespaces => 0,
    };
    let mut chain: Vec<&str> = sym_scope_path.iter().map(String::as_str).collect();
    chain.push("");
    chain[..depth.min(chain.len())].iter().rev().map(|s| s.to_string()).collect()
}

/// The scope names in front of the function's own name in its declaration.
/// `emitFunctionDeclaration` runs before the printer enters any scope, so the
/// path resolves from the global scope: the whole namespace chain, unless the
/// strategy prints none.
pub fn declaration_prefix(strategy: NamespaceStrategy, sym_scope_path: &[String]) -> Vec<String> {
    match strategy {
        NamespaceStrategy::NoNamespaces => Vec::new(),
        _ => sym_scope_path.iter().rev().cloned().collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn path(v: &[&str]) -> Vec<String> {
        v.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn minimal_prints_what_the_use_site_cannot_see() {
        let unused = |_: &str, _: usize| false;
        let minimal = NamespaceStrategy::MinimalNamespaces;
        // A sibling class's method, called from a constructor.
        assert_eq!(
            scope_prefix(minimal, "instance", &path(&["UIThemeManager"]),
                &path(&["ArticleListWidget"]), &unused),
            path(&["UIThemeManager"])
        );
        // A method of the same class needs nothing.
        assert!(scope_prefix(minimal, "setupLayout", &path(&["ArticleListWidget"]),
            &path(&["ArticleListWidget"]), &unused).is_empty());
        // Two levels, outermost first.
        assert_eq!(
            scope_prefix(minimal, "smallIconSize", &path(&["Gui", "Utils"]),
                &path(&["ArticleListWidget"]), &unused),
            path(&["Utils", "Gui"])
        );
        // A global function from inside a class.
        assert!(scope_prefix(minimal, "free", &[], &path(&["ArticleListWidget"]), &unused).is_empty());
        // A global the function shadows with a local of the same name.
        assert_eq!(
            scope_prefix(minimal, "count", &[], &[], &|n: &str, _: usize| n == "count"),
            path(&[""])
        );
    }

    #[test]
    fn all_and_none() {
        let unused = |_: &str, _: usize| false;
        let sym = path(&["Gui", "Utils"]);
        assert_eq!(
            scope_prefix(NamespaceStrategy::AllNamespaces, "f", &sym, &path(&["Utils", "Gui"]), &unused),
            path(&["Utils", "Gui"])
        );
        assert!(scope_prefix(NamespaceStrategy::NoNamespaces, "f", &sym, &[], &unused).is_empty());
        assert_eq!(declaration_prefix(NamespaceStrategy::MinimalNamespaces, &sym), path(&["Utils", "Gui"]));
        assert!(declaration_prefix(NamespaceStrategy::NoNamespaces, &sym).is_empty());
    }
}
