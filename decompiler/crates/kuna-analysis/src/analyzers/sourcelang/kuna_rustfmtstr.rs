//! Require the complete x86-64 string forwarding body and exact pad identity.

use object::{Object, ObjectSection, SectionKind};
use std::collections::{BTreeMap, BTreeSet};

pub(super) fn collect(file: &object::File<'_>, symbols: &[(u64, Option<String>)]) -> BTreeSet<u64> {
    if file.architecture() != object::Architecture::X86_64 {
        return BTreeSet::new();
    }
    let mut names: BTreeMap<_, _> = symbols
        .iter()
        .filter_map(|(address, name)| {
            let name = name.as_deref()?;
            matches!(name, "<&T as core::fmt::Display>::fmt" | "core::fmt::Formatter::pad")
                .then_some((*address, name))
        })
        .collect();
    for (address, name) in symbols {
        if names.get(address).is_some_and(|known| Some(*known) != name.as_deref()) {
            names.remove(address);
        }
    }
    names
        .iter()
        .filter_map(|(&address, name)| {
            if *name != "<&T as core::fmt::Display>::fmt" {
                return None;
            }
            let target = file.sections().find_map(|section| {
                if section.kind() != SectionKind::Text {
                    return None;
                }
                let offset = usize::try_from(address.checked_sub(section.address())?).ok()?;
                let bytes = section.data().ok()?.get(offset..offset.checked_add(18)?)?;
                kuna_decomp::kuna_rustfmtstr::forwarded_target(address, bytes)
            })?;
            (names.get(&target) == Some(&"core::fmt::Formatter::pad")).then_some(address)
        })
        .collect()
}
