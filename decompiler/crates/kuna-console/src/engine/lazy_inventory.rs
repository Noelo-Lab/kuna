//! Compact anonymous entries for the lazy function commit.

use super::*;

impl ConsoleProgram {
    pub(super) fn anonymous_address(&self, name: &str) -> Option<Address> {
        let vma = u64::from_str_radix(name.strip_prefix("sub_")?, 16).ok()?;
        self.anonymous_entries
            .binary_search_by_key(&vma, |entry| entry.0)
            .ok()?;
        let space = self.arch().manage().get_default_code_space()?.clone();
        let addr = Address::new(space, vma);
        (format!("sub_{vma:x}") == name).then_some(addr)
    }

    pub(super) fn inventory_symbols(&self) -> Box<dyn Iterator<Item = &ProgramSymbol> + '_> {
        if self.anonymous_entries.is_empty() {
            return Box::new(self.symbols.iter());
        }
        let records = self.anonymous_symbols.get_or_init(|| {
            let mut records: Vec<_> = self.symbols.iter().cloned().collect();
            let space = self.arch().manage().get_default_code_space().unwrap();
            for &(vma, ordinal) in &self.anonymous_entries {
                let addr = Address::new(Rc::clone(space), vma);
                records.push(ProgramSymbol {
                    ordinal,
                    name: format!("sub_{vma:x}"),
                    addr,
                    object_location: self.object_location_at(vma),
                    binding: None,
                    provenance: if self.object_location_at(vma).is_some() {
                        EntryProvenance::DefinedObject
                    } else {
                        EntryProvenance::Mapped
                    },
                });
            }
            records.sort_unstable_by_key(|record| record.ordinal);
            records
        });
        Box::new(records.iter())
    }

    pub(super) fn anonymous_entry(&self, vma: u64) -> FunctionEntry {
        let space = self.arch().manage().get_default_code_space().unwrap();
        let object_location = self.object_location_at(vma);
        let name = format!("sub_{vma:x}");
        let mut aliases = Vec::new();
        self.push_loader_aliases(vma, &name, &mut aliases);
        FunctionEntry {
            name,
            addr: Address::new(Rc::clone(space), vma),
            aliases,
            provenance: if object_location.is_some() {
                EntryProvenance::DefinedObject
            } else {
                EntryProvenance::Mapped
            },
            object_location,
            binding: None,
            size: 0,
        }
    }

    fn merge_anonymous_name(&self, entry: &mut FunctionEntry, vma: u64) {
        let mut names: Vec<String> = std::iter::once(entry.name.clone())
            .chain(
                entry
                    .aliases
                    .iter()
                    .filter(|name| !self.named_only_by_loader_alias(entry, name))
                    .cloned(),
            )
            .chain([format!("sub_{vma:x}")])
            .collect();
        names.sort_by(|a, b| {
            (self.is_not_declared_name(vma, a), entry_name_rank(a))
                .cmp(&(self.is_not_declared_name(vma, b), entry_name_rank(b)))
                .then_with(|| a.cmp(b))
        });
        names.dedup();
        entry.name = names.remove(0);
        entry.aliases = names;
        self.push_loader_aliases(vma, &entry.name, &mut entry.aliases);
    }

    pub(super) fn append_anonymous_entries(&self, entries: &mut Vec<FunctionEntry>) {
        if self.anonymous_entries.is_empty() {
            return;
        }
        for &(vma, _) in &self.anonymous_entries {
            let vma = self.thumb_normalized(vma);
            match entries.binary_search_by_key(&vma, |entry| entry.addr.get_offset()) {
                Ok(i) => self.merge_anonymous_name(&mut entries[i], vma),
                Err(_) => {}
            }
        }
        let named: std::collections::HashSet<u64> = entries
            .iter()
            .map(|entry| entry.addr.get_offset())
            .collect();
        entries.extend(self.anonymous_entries.iter().filter_map(|&(vma, _)| {
            let vma = self.thumb_normalized(vma);
            (!named.contains(&vma)).then(|| self.anonymous_entry(vma))
        }));
        entries.sort_unstable_by_key(|entry| entry.addr.get_offset());
        crate::funcextent::assign_extents(
            entries,
            &crate::funcextent::spans(&self.sections(), &self.segments()),
        );
        self.bound_entries_by_flow(entries);
        for entry in entries {
            let declared = self.declared_extent(entry.addr.get_offset());
            if declared > 0 {
                entry.size = declared as u64;
            }
        }
    }

    pub(crate) fn entry_offsets(&self) -> Vec<u64> {
        let mut starts: Vec<_> = self
            .symbols
            .iter()
            .filter(|s| s.addr.get_space().is_some())
            .map(|s| self.thumb_normalized(s.addr.get_offset()))
            .chain(
                self.anonymous_entries
                    .iter()
                    .map(|entry| self.thumb_normalized(entry.0)),
            )
            .collect();
        starts.sort_unstable();
        starts.dedup();
        starts
    }

    pub(super) fn find_compact_entry_at(&self, vma: u64) -> Option<FunctionEntry> {
        let mut entry = self
            .build_function_entries_canonical()
            .into_iter()
            .find(|entry| entry.addr.get_offset() == vma);
        if self
            .anonymous_entries
            .binary_search_by_key(&vma, |entry| entry.0)
            .is_ok()
        {
            let anonymous = self.anonymous_entry(vma);
            if let Some(named) = entry.as_mut() {
                self.merge_anonymous_name(named, vma);
            } else {
                entry = Some(anonymous);
            }
        }
        if let Some(entry) = entry.as_mut() {
            entry.size = self.function_extent_at(vma);
        }
        entry
    }

    pub(super) fn compact_extent_at(&self, vma: u64) -> u64 {
        let above = self
            .anonymous_entries
            .partition_point(|entry| entry.0 <= vma);
        let next = self
            .anonymous_entries
            .get(above)
            .map(|entry| entry.0)
            .into_iter()
            .chain(self.symbols.iter().filter_map(|s| {
                s.addr.get_space()?;
                let start = self.thumb_normalized(s.addr.get_offset());
                (start > vma).then_some(start)
            }))
            .min();
        crate::funcextent::extent_at(
            vma,
            &next.into_iter().collect::<Vec<_>>(),
            &crate::funcextent::spans(&self.sections(), &self.segments()),
        )
    }

    pub(super) fn compact_entries_containing(&self, vmas: &[u64]) -> Vec<Option<FunctionEntry>> {
        let starts = self.entry_offsets();
        vmas.iter()
            .map(|&vma| {
                let vma = self.thumb_normalized(vma);
                let below = starts.partition_point(|&start| start <= vma);
                let start = *starts.get(below.checked_sub(1)?)?;
                if start == vma {
                    return None;
                }
                let entry = self.find_compact_entry_at(start)?;
                (vma - start < entry.size).then_some(entry)
            })
            .collect()
    }
}
