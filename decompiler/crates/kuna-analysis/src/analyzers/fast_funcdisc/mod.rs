//! Fast function discovery for latency-sensitive whole-binary exports.
//!
//! The recursive Listing walk supplies direct-call closure from loader-backed
//! roots. This module adds conservative validation of indirect-only roots from
//! absolute address tables, accepting a target only when AIF's established
//! fingerprint and valid-subroutine predicates both corroborate it.

use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

use kuna_base::space::AddrSpace;
use kuna_sleigh::translate::Translate;
use object::Object;

use crate::listing::Listing;

const MAX_POINTER_TABLE_ENTRIES: usize = 256;
const MAX_SINGLE_REFERENCE_CANDIDATES: usize = 512;
const MAX_VALIDATED_POINTER_CANDIDATES: usize = 4096;

fn pointer_candidates(tables: Vec<crate::addrtable::AddrTable>) -> Vec<u64> {
    let mut references = BTreeMap::<u64, usize>::new();
    for table in tables
        .into_iter()
        .filter(|table| table.len <= MAX_POINTER_TABLE_ENTRIES)
    {
        let distinct: BTreeSet<u64> = table.elements.into_iter().collect();
        for target in distinct {
            *references.entry(target).or_default() += 1;
        }
    }
    let require_corroboration = references.len() > MAX_SINGLE_REFERENCE_CANDIDATES;
    let mut ranked: Vec<(u64, usize)> = references
        .into_iter()
        .filter(|(_, count)| !require_corroboration || *count >= 2)
        .collect();
    ranked.sort_by(|(target_a, refs_a), (target_b, refs_b)| {
        refs_b.cmp(refs_a).then_with(|| target_a.cmp(target_b))
    });
    ranked.truncate(MAX_VALIDATED_POINTER_CANDIDATES);
    ranked.into_iter().map(|(target, _)| target).collect()
}

pub fn pointer_table_seeds(
    file: &object::File,
    listing: &Listing,
    translate: &dyn Translate,
    code_space: Rc<AddrSpace>,
) -> Vec<u64> {
    if file.architecture() == object::Architecture::Arm {
        return crate::aif::code_pointer_table_seeds(
            file,
            listing,
            translate,
            code_space,
            listing.exec_ranges(),
        );
    }

    let ptr_size = if file.is_64() { 8 } else { 4 };
    let tables = crate::addrtable::scan_address_tables(file, 2, ptr_size);
    crate::aif::validate_pointer_targets(
        listing,
        translate,
        code_space,
        listing.exec_ranges(),
        pointer_candidates(tables),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::addrtable::AddrTable;

    fn table(addr: u64, elements: Vec<u64>) -> AddrTable {
        AddrTable {
            addr,
            len: elements.len(),
            elements,
        }
    }

    #[test]
    fn large_inventories_require_distinct_table_references() {
        let mut tables = vec![table(0, vec![0x1000, 0x1000])];
        tables.extend(
            (0..MAX_SINGLE_REFERENCE_CANDIDATES)
                .map(|i| table(i as u64 + 1, vec![0x2000 + i as u64])),
        );
        assert!(!pointer_candidates(tables.clone()).contains(&0x1000));
        tables.push(table(0x1000, vec![0x1000]));
        assert!(pointer_candidates(tables).contains(&0x1000));
    }

    #[test]
    fn pointer_validation_work_has_a_global_cap() {
        let mut tables = Vec::new();
        for i in 0..=MAX_VALIDATED_POINTER_CANDIDATES {
            let target = 0x1000 + i as u64;
            tables.push(table(i as u64 * 2, vec![target]));
            tables.push(table(i as u64 * 2 + 1, vec![target]));
        }
        let candidates = pointer_candidates(tables);
        assert_eq!(candidates.len(), MAX_VALIDATED_POINTER_CANDIDATES);
        assert_eq!(candidates[0], 0x1000);
    }

    #[test]
    fn oversized_tables_are_ignored() {
        let elements = (0..=MAX_POINTER_TABLE_ENTRIES)
            .map(|i| 0x1000 + i as u64)
            .collect();
        assert!(pointer_candidates(vec![table(0, elements)]).is_empty());
    }
}
