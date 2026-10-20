//! GNU Fortran reference arguments retain their entry-time address storage.

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

/// By-value argument sizes an ABI passes through a hidden address in an argument register.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) enum HiddenValues {
    #[default]
    Never,
    Above(i32),
    UnlessRegisterSized,
}

impl HiddenValues {
    fn may_hide(self, size: i32) -> bool {
        match self {
            HiddenValues::Never => false,
            HiddenValues::Above(limit) => size > limit,
            HiddenValues::UnlessRegisterSized => !matches!(size, 1 | 2 | 4 | 8),
        }
    }
}

/// Incoming integer argument registers (DWARF numbers) of an image's ABI.
#[derive(Clone, Copy, Default)]
pub(super) struct ArgumentAbi {
    pub(super) registers: &'static [u16],
    pub(super) hidden: HiddenValues,
}

pub(super) fn argument_abi(
    format: object::BinaryFormat,
    architecture: object::Architecture,
) -> ArgumentAbi {
    use object::{Architecture::*, BinaryFormat::*};
    const RISCV: &[u16] = &[10, 11, 12, 13, 14, 15, 16, 17];
    let (registers, hidden): (&'static [u16], _) = match (format, architecture) {
        (Pe | Coff, X86_64) => (&[2, 1, 8, 9], HiddenValues::UnlessRegisterSized),
        (Elf | MachO, X86_64) => (&[5, 4, 1, 2, 8, 9], HiddenValues::Never),
        (Elf, Arm) => (&[0, 1, 2, 3], HiddenValues::Never),
        (Elf, Aarch64) => (&[0, 1, 2, 3, 4, 5, 6, 7], HiddenValues::Above(16)),
        (Elf, Riscv32) => (RISCV, HiddenValues::Above(8)),
        (Elf, Riscv64) => (RISCV, HiddenValues::Above(16)),
        _ => (&[], HiddenValues::Never),
    };
    ArgumentAbi { registers, hidden }
}

pub(super) fn location_at_entry<'a>(
    dwarf: &gimli::Dwarf<Reader<'a>>,
    unit: &gimli::Unit<Reader<'a>>,
    location: &gimli::AttributeValue<Reader<'a>>,
    entry: Option<u64>,
) -> Option<gimli::Expression<Reader<'a>>> {
    match location {
        gimli::AttributeValue::Exprloc(expression) => return Some(expression.clone()),
        gimli::AttributeValue::Block(bytes) => return Some(gimli::Expression(bytes.clone())),
        _ => {}
    }
    let entry = entry?;
    let mut locations = dwarf.attr_locations(unit, location.clone()).ok()??;
    let mut found = None;
    for _ in 0..4096 {
        let Some(location) = locations.next().ok()? else {
            return found;
        };
        if location.range.begin <= entry && entry < location.range.end {
            if found.is_some() {
                return None;
            }
            found = Some(location.data);
        }
    }
    None
}

pub(super) fn memory_argument_register(
    expression: &gimli::Expression<Reader<'_>>,
    encoding: gimli::Encoding,
    argument_registers: &[u16],
) -> bool {
    let mut operations = expression.operations(encoding);
    matches!(operations.next(), Ok(Some(gimli::Operation::RegisterOffset { register, offset: 0, base_type }))
        if base_type.0 == 0 && argument_registers.contains(&register.0))
        && matches!(operations.next(), Ok(None))
}

pub(super) fn parameter_type(
    parameter: &DieSnap,
    value: Rc<Datatype>,
    types: &dyn TypeFactory,
    word_size: uint4,
) -> Option<Rc<Datatype>> {
    use kuna_decomp::dtype::type_metatype::{TYPE_CODE, TYPE_VOID};
    let register_reference = parameter.register_indirect_parameter.is_some_and(|hidden| {
        value.get_size() > 0
            && !matches!(value.get_metatype(), TYPE_VOID | TYPE_CODE)
            && !hidden.may_hide(value.get_size())
    });
    if parameter.indirect_parameter || register_reference {
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
    fn register_memory_requires_an_incoming_argument_address() {
        let encoding = gimli::Encoding {
            format: gimli::Format::Dwarf32,
            version: 5,
            address_size: 8,
        };
        for (bytes, expected) in [
            (&[0x75, 0x00][..], true),
            (&[0x92, 0x05, 0x00][..], true),
            (&[0x55][..], false),
            (&[0x75, 0x01][..], false),
            (&[0x75, 0x00, 0x9f][..], false),
            (&[0x75, 0x00, 0x06][..], false),
            (&[0x77, 0x00][..], false),
            (&[0x76, 0x00][..], false),
            (&[0x75][..], false),
        ] {
            let expression =
                gimli::Expression(gimli::EndianSlice::new(bytes, gimli::RunTimeEndian::Little));
            assert_eq!(
                memory_argument_register(&expression, encoding, &[5]),
                expected
            );
            assert!(!memory_argument_register(&expression, encoding, &[]));
        }
    }

    #[test]
    fn location_lists_require_one_complete_entry_covering_the_entry_pc() {
        fn selected(
            ranges: &[(u64, u64, &[u8])],
            pc: Option<u64>,
            terminate: bool,
        ) -> Option<Vec<u8>> {
            let mut locations = Vec::new();
            for &(begin, end, bytes) in ranges {
                locations.extend_from_slice(&begin.to_le_bytes());
                locations.extend_from_slice(&end.to_le_bytes());
                locations.extend_from_slice(&(bytes.len() as u16).to_le_bytes());
                locations.extend_from_slice(bytes);
            }
            if terminate {
                locations.extend_from_slice(&[0; 16]);
            } else {
                locations.push(1);
            }
            let sections = gimli::DwarfSections::load(|id| -> Result<Vec<u8>, gimli::Error> {
                Ok(match id {
                    gimli::SectionId::DebugInfo => vec![8, 0, 0, 0, 4, 0, 0, 0, 0, 0, 8, 1],
                    gimli::SectionId::DebugAbbrev => vec![1, 0x11, 0, 0, 0, 0],
                    gimli::SectionId::DebugLoc => locations.clone(),
                    _ => Vec::new(),
                })
            })
            .unwrap();
            let dwarf = sections
                .borrow(|bytes| gimli::EndianSlice::new(bytes, gimli::RunTimeEndian::Little));
            let header = dwarf.units().next().unwrap().unwrap();
            let unit = dwarf.unit(header).unwrap();
            let attribute = gimli::AttributeValue::LocationListsRef(gimli::LocationListsOffset(0));
            location_at_entry(&dwarf, &unit, &attribute, pc)
                .map(|expression| expression.0.slice().to_vec())
        }
        let ranges = &[(0x110, 0x120, &[0x55][..]), (0x100, 0x110, &[0x75, 0][..])];
        assert_eq!(selected(ranges, Some(0x100), true), Some(vec![0x75, 0]));
        assert_eq!(selected(ranges, Some(0x110), true), Some(vec![0x55]));
        assert_eq!(selected(ranges, Some(0x120), true), None);
        assert_eq!(selected(ranges, None, true), None);
        assert_eq!(selected(ranges, Some(0x100), false), None);
        assert_eq!(
            selected(
                &[(0x100, 0x120, &[0x75, 0]), (0x100, 0x110, &[0x55])],
                Some(0x100),
                true
            ),
            None
        );
    }

    #[test]
    fn register_addresses_become_pointers_unless_the_abi_may_pass_the_value_hidden() {
        use kuna_decomp::dtype::{type_metatype::*, TypeFactoryImpl};
        use object::{Architecture::*, BinaryFormat::*};
        let types = TypeFactoryImpl::new();
        types.set_default_alignment_map();
        types.set_max_basetype_size(16);
        types.setup_sizes(Some(8), 8, 4);
        types.cache_core_types().unwrap();
        let float = types.get_base(4, TYPE_FLOAT).unwrap();
        let values = [
            types.get_base(1, TYPE_INT).unwrap(),
            types.get_base(2, TYPE_UINT).unwrap(),
            float.clone(),
            types.get_base(8, TYPE_FLOAT).unwrap(),
            types.get_type_array(2, float.clone()).unwrap(),
            types.get_base(10, TYPE_FLOAT).unwrap(),
            types.get_type_array(3, float.clone()).unwrap(),
            types.get_base(16, TYPE_FLOAT).unwrap(),
            types.get_type_array(5, float.clone()).unwrap(),
            types.get_type_array(512, float).unwrap(),
            types.get_type_struct("value_record").unwrap(),
        ];
        for (format, architecture, pointers) in [
            (Elf, X86_64, 10),
            (MachO, X86_64, 10),
            (Elf, Arm, 10),
            (Pe, X86_64, 5),
            (Elf, Aarch64, 8),
            (Elf, Riscv64, 8),
            (Elf, Riscv32, 5),
        ] {
            let mut parameter = DieSnap::new(gimli::DW_TAG_formal_parameter, 2);
            parameter.register_indirect_parameter = Some(argument_abi(format, architecture).hidden);
            for (index, value) in values.iter().enumerate() {
                let result = parameter_type(&parameter, value.clone(), &types, 1).unwrap();
                if index < pointers {
                    assert_eq!(
                        result.get_metatype(),
                        TYPE_PTR,
                        "{format:?} {architecture:?}"
                    );
                    assert_eq!(result.get_size(), 8);
                    assert!(Rc::ptr_eq(&result.get_ptr_to().unwrap(), value));
                } else {
                    assert!(Rc::ptr_eq(&result, value), "{format:?} {architecture:?}");
                }
            }
        }
        let parameter = DieSnap::new(gimli::DW_TAG_formal_parameter, 2);
        for value in &values {
            let result = parameter_type(&parameter, value.clone(), &types, 1).unwrap();
            assert!(Rc::ptr_eq(&result, value));
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
