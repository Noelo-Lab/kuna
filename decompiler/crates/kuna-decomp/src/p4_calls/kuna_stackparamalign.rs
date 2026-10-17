//! Stack argument alignment can differ from the in-memory type alignment.
use super::fspec::param_entry_flags::IGNORE_TYPE_ALIGN;
use kuna_base::{
    error::{KunaError, KunaResult},
    space::{spacetype, AddrSpace},
    xml::Element,
};

pub(crate) fn decode(
    entry: &Element,
    space: &AddrSpace,
    alignment: i32,
    size: i32,
) -> KunaResult<u32> {
    let value = (0..entry.get_num_attributes())
        .find(|&i| entry.get_attribute_name(i) == "stackalign")
        .map(|i| entry.get_attribute_value_at(i));
    match value {
        None | Some(b"true") => Ok(0),
        Some(b"false")
            if space.get_type() == spacetype::IPTR_SPACEBASE
                && alignment > 0
                && alignment != size =>
        {
            Ok(IGNORE_TYPE_ALIGN)
        }
        Some(b"false") => Err(KunaError::lowlevel(
            "pentry stackalign requires aligned stack storage",
        )),
        Some(_) => Err(KunaError::lowlevel("bad pentry stackalign")),
    }
}

pub(crate) fn effective_alignment(flags: u32, type_alignment: i32, slot_alignment: i32) -> i32 {
    if flags & IGNORE_TYPE_ALIGN != 0 {
        slot_alignment
    } else {
        type_alignment
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn decoder_rejects_non_resources_and_invalid_values() {
        let stack = AddrSpace::new(spacetype::IPTR_SPACEBASE, "stack", false, 4, 1, 5, 0, 0, 0);
        let register = AddrSpace::new(
            spacetype::IPTR_PROCESSOR,
            "register",
            false,
            4,
            1,
            3,
            0,
            0,
            0,
        );
        let mut entry = Element::new();
        assert_eq!(decode(&entry, &stack, 4, 500).unwrap(), 0);
        entry.add_attribute("stackalign", b"false");
        assert_eq!(decode(&entry, &stack, 4, 500).unwrap(), IGNORE_TYPE_ALIGN);
        assert!(decode(&entry, &register, 4, 500).is_err());
        assert!(decode(&entry, &stack, 0, 500).is_err());
        assert!(decode(&entry, &stack, 4, 4).is_err());
        let mut entry = Element::new();
        entry.add_attribute("stackalign", b"true");
        assert_eq!(decode(&entry, &register, 0, 4).unwrap(), 0);
        let mut entry = Element::new();
        entry.add_attribute("stackalign", b"invalid");
        assert!(decode(&entry, &stack, 4, 500).is_err());
    }
}
