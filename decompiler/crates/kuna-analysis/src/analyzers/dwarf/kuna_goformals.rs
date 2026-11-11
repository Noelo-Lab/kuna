//! Go result formals require the Go ABI importer, not a C-style prototype.

use super::Reader;

pub(super) fn generic_prototype_allowed(unit: &gimli::Unit<Reader<'_>>) -> bool {
    let mut entries = unit.entries();
    !matches!(entries.next_dfs(), Ok(Some(entry)) if matches!(
        entry.attr_value(gimli::DW_AT_language),
        Some(gimli::AttributeValue::Language(gimli::DW_LANG_Go))
    ))
}
