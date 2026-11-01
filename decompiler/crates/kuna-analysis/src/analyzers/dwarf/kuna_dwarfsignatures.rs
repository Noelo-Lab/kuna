//! Minimal debug information names functions without describing their signatures.

use std::collections::BTreeMap;

use super::DieSnap;

pub(super) fn has_signature_information(dies: &BTreeMap<usize, DieSnap>) -> bool {
    dies.values().any(|die| {
        die.type_ref.is_some()
            || matches!(die.tag, gimli::DW_TAG_formal_parameter | gimli::DW_TAG_unspecified_parameters)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_and_ranges_do_not_describe_a_void_signature() {
        let mut sub = DieSnap::new(gimli::DW_TAG_subprogram, 1);
        sub.name = "print".into();
        sub.low_pc = Some(0x1000);
        let mut dies = BTreeMap::from([(1, sub)]);
        assert!(!has_signature_information(&dies));
        dies.insert(2, DieSnap::new(gimli::DW_TAG_formal_parameter, 2));
        assert!(has_signature_information(&dies));
        dies.remove(&2);
        dies.get_mut(&1).unwrap().type_ref = Some(3);
        assert!(has_signature_information(&dies));
    }
}
