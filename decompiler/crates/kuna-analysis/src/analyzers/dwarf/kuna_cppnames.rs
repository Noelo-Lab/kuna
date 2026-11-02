//! Keep address-proven namespace names from acquiring a redundant global alias.

use std::collections::BTreeMap;

use kuna_decomp::kuna_namespaces::plain_scope;

use crate::pass::{SymFact, SymKind};

/// Fold enabled C++ DWARF symbols into the generic stream. Only a unique
/// qualified function name can replace its exact unqualified leaf at that address.
pub fn merge_symbols(symbols: &mut Vec<SymFact>, cpp: Vec<SymFact>) {
    if cpp.is_empty() {
        return;
    }
    let mut names: BTreeMap<u64, Option<&str>> = BTreeMap::new();
    for fact in cpp.iter().filter(|fact| fact.kind == SymKind::Function) {
        names
            .entry(fact.addr)
            .and_modify(|name| {
                if *name != Some(fact.name.as_str()) {
                    *name = None;
                }
            })
            .or_insert(Some(&fact.name));
    }
    for fact in symbols
        .iter()
        .filter(|fact| fact.kind == SymKind::Function && fact.name.contains("::"))
    {
        if let Some(name) = names.get_mut(&fact.addr) {
            if *name != Some(fact.name.as_str()) {
                *name = None;
            }
        }
    }
    symbols.retain(|fact| {
        if fact.kind != SymKind::Function || fact.name.contains("::") {
            return true;
        }
        let Some(Some(name)) = names.get(&fact.addr) else {
            return true;
        };
        let Some((_, leaf)) = name.rsplit_once("::") else {
            return true;
        };
        let valid = name.split("::").all(plain_scope);
        !valid || leaf != fact.name
    });
    symbols.extend(cpp);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn function(addr: u64, name: &str) -> SymFact {
        SymFact {
            addr,
            name: name.into(),
            kind: SymKind::Function,
        }
    }

    #[test]
    fn qualified_identity_replaces_only_the_same_address_and_leaf() {
        let mut symbols = vec![
            function(1, "target"),
            function(2, "target"),
            function(1, "alias"),
            function(3, "other::target"),
            SymFact {
                addr: 1,
                name: "target".into(),
                kind: SymKind::Data,
            },
        ];
        merge_symbols(&mut symbols, vec![function(1, "left::deep::target")]);
        assert_eq!(
            symbols
                .iter()
                .map(|s| (s.addr, s.name.as_str(), s.kind))
                .collect::<Vec<_>>(),
            vec![
                (2, "target", SymKind::Function),
                (1, "alias", SymKind::Function),
                (3, "other::target", SymKind::Function),
                (1, "target", SymKind::Data),
                (1, "left::deep::target", SymKind::Function),
            ]
        );
    }

    #[test]
    fn conflicting_cpp_names_preserve_the_generic_fact_in_either_order() {
        for conflict in ["right::target", "target", "left::alias"] {
            for reverse in [false, true] {
                let mut cpp = vec![function(1, "left::target"), function(1, conflict)];
                if reverse {
                    cpp.reverse();
                }
                let mut symbols = vec![function(1, "target")];
                merge_symbols(&mut symbols, cpp);
                assert_eq!(symbols.len(), 3);
                assert_eq!(symbols[0].name, "target");
            }
        }
    }

    #[test]
    fn repeated_identical_proof_is_not_ambiguous() {
        let mut symbols = vec![function(1, "target")];
        merge_symbols(
            &mut symbols,
            vec![function(1, "left::target"), function(1, "left::target")],
        );
        assert_eq!(symbols.len(), 2);
        assert!(symbols.iter().all(|s| s.name == "left::target"));
    }

    #[test]
    fn conflicting_qualified_generic_fact_preserves_both_aliases() {
        let mut symbols = vec![function(1, "target"), function(1, "other::target")];
        merge_symbols(&mut symbols, vec![function(1, "left::target")]);
        assert_eq!(symbols.len(), 3);
        assert_eq!(symbols[0].name, "target");
        assert_eq!(symbols[1].name, "other::target");
    }

    #[test]
    fn unqualified_or_malformed_paths_do_not_replace_generic_names() {
        for name in [
            "target",
            "::target",
            "left::::target",
            "left:::target",
            "left::",
            " left::target",
            "left:: target",
            "raw_scope::union::target",
            "left::class::target",
            "left::Box<int>::target",
            "left::&T::target",
            "left::*mut T::target",
            "left::(anonymous namespace)::target",
            "left::é::target",
            "left::1scope::target",
        ] {
            let mut symbols = vec![function(1, "target")];
            merge_symbols(&mut symbols, vec![function(1, name)]);
            assert_eq!(symbols.len(), 2, "{name}");
            assert_eq!(symbols[0].name, "target");
        }
    }

    #[test]
    fn data_and_empty_cpp_streams_cannot_replace_function_facts() {
        let mut symbols = vec![function(1, "target")];
        merge_symbols(&mut symbols, Vec::new());
        assert_eq!(symbols.len(), 1);
        merge_symbols(
            &mut symbols,
            vec![SymFact {
                addr: 1,
                name: "left::target".into(),
                kind: SymKind::Data,
            }],
        );
        assert_eq!(symbols.len(), 2);
        assert_eq!(symbols[0].name, "target");
    }
}
