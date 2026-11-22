use std::borrow::Cow;
use std::collections::{btree_map::Entry, BTreeMap, HashSet};

use kuna_analysis::pass::{AnalysisOutput, SymKind};
use kuna_decomp::kuna_symbolnamebound::bound_scope_path;
use kuna_decomp::kuna_symbolnamechars::{sanitize_symbol_name, symbolnamechars_mode};

use super::{ConsoleProgram, ProgramContainer, is_generic_placeholder_name};

#[derive(Default)]
pub(super) struct SignatureNames {
    provided_entries: HashSet<u64>,
    loader_names: HashSet<String>,
}

impl SignatureNames {
    /// Protect supplied names, not the placeholders already installed for raw entries.
    pub(super) fn collect(
        prog: &ConsoleProgram,
        out: &AnalysisOutput,
        zero_size_data_names: Vec<String>,
    ) -> Self {
        if !out.fid_names.iter().any(|m| m.by_signature) {
            return Self::default();
        }
        let provided_entries = prog
            .symbols
            .iter()
            .filter(|s| {
                prog.container != ProgramContainer::Raw
                    || !is_generic_placeholder_name(&s.name)
                    || !prog.is_not_declared_name(s.addr.get_offset(), &s.name)
            })
            .map(|s| s.addr.get_offset())
            .chain(prog.loader_aliases.keys().copied())
            .chain(
                out.symbols
                    .iter()
                    .filter(|s| s.kind == SymKind::Function)
                    .map(|s| s.addr),
            )
            .chain(
                out.fid_names
                    .iter()
                    .filter(|m| !m.by_signature)
                    .map(|m| m.addr),
            )
            .map(|addr| prog.thumb_normalized(addr))
            .collect();
        let data_names = zero_size_data_names
            .into_iter()
            .map(|name| bound_scope_path(&name, "::").into_owned());
        let loader_names = prog
            .loader_aliases
            .values()
            .flatten()
            .cloned()
            .chain(data_names)
            .collect();
        Self {
            provided_entries,
            loader_names,
        }
    }

    pub(super) fn blocks(&self, prog: &ConsoleProgram, addr: u64, name: &str) -> bool {
        self.provided_entries.contains(&prog.thumb_normalized(addr))
            || name_keys(name).any(|key| self.is_owned(prog, &key))
    }

    fn is_owned(&self, prog: &ConsoleProgram, name: &str) -> bool {
        if prog.lookup_symbol(name).is_some()
            || self.loader_names.contains(name)
            || prog.lookup_data_alias(name).is_some()
        {
            return true;
        }
        let symbols = &prog.arch().symboltab;
        let (scope, basename) = symbols.resolve_scope_from_symbol_name(name, "::", None);
        scope.is_some_and(|scope| !symbols.find_by_name(scope, &basename).is_empty())
    }
}

fn name_keys(name: &str) -> impl Iterator<Item = Cow<'_, str>> {
    let raw = bound_scope_path(name, "::");
    let canonical = std::iter::once_with(move || {
        let name = kuna_analysis::demangle::demangle_name(name)?;
        let name = sanitize_symbol_name(&name, symbolnamechars_mode());
        Some(Cow::Owned(bound_scope_path(&name, "::").into_owned()))
    });
    std::iter::once(raw).chain(canonical.flatten())
}

/// Return every eligible entry staking a name also proposed at a different entry.
pub(super) fn contested_addresses<'a>(
    proposals: impl IntoIterator<Item = (u64, &'a str)>,
) -> HashSet<u64> {
    let mut owners = BTreeMap::new();
    let mut contested = HashSet::new();
    for (addr, name) in proposals {
        for key in name_keys(name) {
            match owners.entry(key) {
                Entry::Vacant(owner) => {
                    owner.insert(addr);
                }
                Entry::Occupied(owner) if *owner.get() != addr => {
                    contested.extend([*owner.get(), addr]);
                }
                Entry::Occupied(_) => {}
            }
        }
    }
    contested
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn competing_signature_spellings_refuse_all_entries_in_either_order() {
        for (first, second) in [
            ("scale_b", "scale_b"),
            ("lib::scale_b", "_ZN3lib7scale_bEv"),
            ("_ZN3lib7scale_bEv", "_ZN3lib7scale_bEi"),
        ] {
            let mut matches = [(0x100, first), (0x200, second), (0x300, "wrap_a")];
            let contested = HashSet::from([0x100, 0x200]);
            assert_eq!(contested_addresses(matches), contested);
            matches.reverse();
            assert_eq!(contested_addresses(matches), contested);
        }
    }

    #[test]
    fn repeated_keys_for_one_address_are_not_cross_address_conflicts() {
        assert!(
            contested_addresses([(0x100, "lib::scale_b"), (0x100, "_ZN3lib7scale_bEv")]).is_empty()
        );
    }
}
