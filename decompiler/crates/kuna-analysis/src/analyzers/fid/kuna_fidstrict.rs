//! Evidence gate for optional strict FID renames.

use super::db::FidRecord;
use super::hash::FidHashQuad;

pub(super) fn supports_rename(records: &[FidRecord], query: &FidHashQuad) -> bool {
    records.iter().any(|record| {
        record.specific_hash == query.specific_hash && record.code_unit_size == query.code_unit_size
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn record(hash: u64, count: i16) -> FidRecord {
        FidRecord { full_hash: 42, specific_hash: hash, code_unit_size: count,
            specific_addl: 0, flags: 0, name: "candidate".into() }
    }

    #[test]
    fn requires_both_facts_from_one_record() {
        let query = FidHashQuad { full_hash: 42, specific_hash: 9,
            code_unit_size: 5, specific_addl_size: 0 };
        assert!(!supports_rename(&[], &query));
        assert!(!supports_rename(&[record(9, 6), record(8, 5)], &query));
        assert!(supports_rename(&[record(8, 5), record(9, 5)], &query));
    }
}
