//! GNU Fortran describes a reference argument through a dereferenced frame slot.

use std::rc::Rc;

use kuna_base::types::uint4;
use kuna_decomp::dtype::{Datatype, TypeFactory};

use super::{read_sleb128, DieSnap, Reader};

pub(super) fn is_fortran(language: Option<gimli::DwLang>) -> bool {
    matches!(
        language,
        Some(
            gimli::DW_LANG_Fortran77
                | gimli::DW_LANG_Fortran90
                | gimli::DW_LANG_Fortran95
                | gimli::DW_LANG_Fortran03
                | gimli::DW_LANG_Fortran08
                | gimli::DW_LANG_Fortran18
        )
    )
}

pub(super) fn indirect_location(loc: &gimli::AttributeValue<Reader<'_>>) -> bool {
    let bytes = match loc {
        gimli::AttributeValue::Exprloc(expr) => expr.0.slice(),
        gimli::AttributeValue::Block(block) => block.slice(),
        _ => return false,
    };
    if bytes.first() != Some(&gimli::DW_OP_fbreg.0) {
        return false;
    }
    let Some((_, consumed)) = read_sleb128(&bytes[1..]) else {
        return false;
    };
    bytes.get(1 + consumed..) == Some(&[gimli::DW_OP_deref.0][..])
}

pub(super) fn parameter_type(
    parameter: &DieSnap,
    value: Rc<Datatype>,
    types: &dyn TypeFactory,
    word_size: uint4,
) -> Option<Rc<Datatype>> {
    if parameter.indirect_parameter {
        types
            .get_type_pointer(types.get_size_of_pointer(), value, word_size)
            .ok()
    } else {
        Some(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_a_single_dereferenced_frame_slot_is_an_indirect_parameter() {
        for (bytes, expected) in [
            (&[0x91, 0x68, 0x06][..], true),
            (&[0x91, 0xc0, 0x7e, 0x06][..], true),
            (&[0x91, 0x68][..], false),
            (&[0x91, 0x68, 0x06, 0x9f][..], false),
            (&[0x91, 0x68, 0x06, 0x06][..], false),
            (&[0x91, 0x80][..], false),
            (&[0x55][..], false),
        ] {
            let expr =
                gimli::Expression(gimli::EndianSlice::new(bytes, gimli::RunTimeEndian::Little));
            assert_eq!(
                indirect_location(&gimli::AttributeValue::Exprloc(expr)),
                expected
            );
        }
    }

    #[test]
    fn other_languages_do_not_inherit_fortran_reference_lowering() {
        for language in [
            None,
            Some(gimli::DW_LANG_C),
            Some(gimli::DW_LANG_C_plus_plus),
        ] {
            assert!(!is_fortran(language));
        }
        for language in [
            gimli::DW_LANG_Fortran77,
            gimli::DW_LANG_Fortran90,
            gimli::DW_LANG_Fortran95,
            gimli::DW_LANG_Fortran03,
            gimli::DW_LANG_Fortran08,
            gimli::DW_LANG_Fortran18,
        ] {
            assert!(is_fortran(Some(language)));
        }
    }
}
