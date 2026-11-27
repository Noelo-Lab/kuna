//! Stack argument alignment can differ from the in-memory type alignment.
use super::fspec::param_entry_flags::{FLOAT_LEFT, IGNORE_TYPE_ALIGN};
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
    let aligned_stack =
        space.get_type() == spacetype::IPTR_SPACEBASE && alignment > 0 && alignment != size;
    let mut flags = 0;
    for i in 0..entry.get_num_attributes() {
        match (entry.get_attribute_name(i), entry.get_attribute_value_at(i)) {
            ("stackalign", b"false") if aligned_stack => flags |= IGNORE_TYPE_ALIGN,
            ("floatleft", b"true") if aligned_stack => flags |= FLOAT_LEFT,
            ("stackalign", b"true") | ("floatleft", b"false") => {}
            ("stackalign" | "floatleft", _) => {
                return Err(KunaError::lowlevel(
                    "invalid stack placement attribute on pentry",
                ))
            }
            _ => {}
        }
    }
    Ok(flags)
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
        let mut entry = Element::new();
        entry.add_attribute("floatleft", b"true");
        entry.add_attribute("stackalign", b"false");
        assert_eq!(decode(&entry, &stack, 8, 500).unwrap(), FLOAT_LEFT | IGNORE_TYPE_ALIGN);
        assert!(decode(&entry, &register, 8, 500).is_err());
    }
}
