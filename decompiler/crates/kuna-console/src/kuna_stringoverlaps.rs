//! Preserve overlapping character-array views within one relocatable storage owner.
use std::collections::{BTreeMap, BTreeSet, HashSet};

use kuna_decomp::decompile_drive::GlobalInfo;

pub(crate) fn declarations<'a>(
    globals: impl Iterator<Item = &'a GlobalInfo>,
    occupied: &BTreeSet<String>,
    header: &[&str],
) -> BTreeMap<u64, String> {
    let reserved: BTreeSet<&str> = header
        .iter()
        .flat_map(|text| words(text).map(|(_, word)| word))
        .collect();
    let mut by_address: BTreeMap<u64, Vec<&GlobalInfo>> = BTreeMap::new();
    let mut names: BTreeMap<&str, BTreeSet<u64>> = BTreeMap::new();
    for global in globals {
        by_address.entry(global.address).or_default().push(global);
        names
            .entry(&global.name)
            .or_default()
            .insert(global.address);
    }
    let mut result = BTreeMap::new();
    let mut group = Vec::new();
    let mut reach = 0;
    for (&address, entries) in &by_address {
        if !group.is_empty() && address >= reach {
            declare_group(&group, &names, occupied, &reserved, &mut result);
            group.clear();
            reach = 0;
        }
        let size = entries
            .iter()
            .map(|g| g.size.max(1) as u64)
            .max()
            .unwrap_or(1);
        reach = reach.max(address.saturating_add(size));
        group.push((address, entries.as_slice()));
    }
    declare_group(&group, &names, occupied, &reserved, &mut result);
    result
}

fn declare_group(
    group: &[(u64, &[&GlobalInfo])],
    names: &BTreeMap<&str, BTreeSet<u64>>,
    occupied: &BTreeSet<String>,
    reserved: &BTreeSet<&str>,
    result: &mut BTreeMap<u64, String>,
) {
    if group.len() < 2 {
        return;
    }
    let Some(views) = group
        .iter()
        .map(|(_, entries)| array_view(entries, names))
        .collect::<Option<Vec<_>>>()
    else {
        return;
    };
    let owner = views[0];
    if views
        .iter()
        .any(|view| occupied.contains(&view.name) || reserved.contains(view.name.as_str()))
    {
        return;
    }
    let Some(end) = owner.address.checked_add(owner.size as u64) else {
        return;
    };
    if views.iter().any(|view| {
        view.address
            .checked_add(view.size as u64)
            .is_none_or(|e| e > end)
    }) {
        return;
    }
    result.insert(owner.address, format!("extern {};", owner.declaration));
    for view in &views[1..] {
        result.insert(
            view.address,
            format!(
                "#define {} (*(char (*)[{}])((unsigned char *)&{} + 0x{:x}))",
                view.name,
                view.size,
                owner.name,
                view.address - owner.address,
            ),
        );
    }
}

fn array_view<'a>(
    entries: &[&'a GlobalInfo],
    names: &BTreeMap<&str, BTreeSet<u64>>,
) -> Option<&'a GlobalInfo> {
    let first = *entries.first()?;
    if !crate::kuna_projectfunctionnames::identifier(&first.name)
        || names.get(first.name.as_str())?.len() != 1
        || first.size <= 0
        || first.declaration != format!("char {}[{}]", first.name, first.size)
        || entries.iter().any(|g| {
            g.name != first.name
                || g.declaration != first.declaration
                || g.size != first.size
                || !g.direct
                || g.unknown
                || g.aggregate
                || g.elem
                || g.integer
        })
    {
        return None;
    }
    Some(first)
}

/// Name evidence retained before a streamed export discards function bodies.
#[derive(Default)]
pub struct GlobalAliasNames {
    occupied: BTreeSet<String>,
    observed: BTreeSet<(u64, String)>,
}

impl GlobalAliasNames {
    pub fn observe(&mut self, result: &crate::project::FuncResult) {
        if result.code.is_some() {
            self.observed
                .insert((result.byte_address, result.name.clone()));
        }
        let mut reserve = |text: &str| {
            self.occupied
                .extend(words(text).map(|(_, word)| word.to_string()));
        };
        reserve(result.proto.as_deref().unwrap_or(""));
        for declaration in &result.call_declarations {
            reserve(declaration);
        }
        for variable in &result.variables {
            reserve(&variable.name);
            reserve(&variable.type_name);
        }
        for ty in &result.types {
            reserve(&ty.definition);
        }
        let visible: HashSet<&str> = result.globals.iter().map(|g| g.name.as_str()).collect();
        for line in result.code.as_deref().unwrap_or("").lines() {
            let declaration = line.contains("; //") && !line.contains("// no-return")
                || line.trim_end().ends_with(';')
                    && !line.contains(['=', '('])
                    && !line.trim_start().starts_with("return ");
            for (offset, word) in words(line) {
                let prefix = line[..offset].trim_end();
                if declaration
                    || !visible.contains(word)
                    || prefix.ends_with('.')
                    || prefix.ends_with("->")
                {
                    reserve(word);
                }
            }
        }
    }

    pub(crate) fn from_results(results: &[crate::project::FuncResult]) -> Self {
        let mut names = Self::default();
        for result in results {
            names.observe(result);
        }
        names
    }

    pub(crate) fn covers(&self, results: &[crate::project::FuncResult]) -> bool {
        results.iter().all(|result| {
            result.error.is_some()
                || self
                    .observed
                    .contains(&(result.byte_address, result.name.clone()))
        })
    }

    pub(crate) fn occupied(&self) -> &BTreeSet<String> {
        &self.occupied
    }
}

fn words(text: &str) -> impl Iterator<Item = (usize, &str)> {
    let bytes = text.as_bytes();
    let mut offset = 0;
    std::iter::from_fn(move || {
        while offset < bytes.len()
            && !(bytes[offset].is_ascii_alphabetic() || bytes[offset] == b'_')
        {
            offset += 1;
        }
        let start = offset;
        while offset < bytes.len()
            && (bytes[offset].is_ascii_alphanumeric() || bytes[offset] == b'_')
        {
            offset += 1;
        }
        (offset > start).then(|| (start, &text[start..offset]))
    })
}

#[cfg(test)]
#[path = "kuna_stringoverlaps/tests.rs"]
mod tests;
