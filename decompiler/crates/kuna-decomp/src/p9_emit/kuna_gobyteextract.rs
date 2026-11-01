//! Integer extraction from a whole, addressable Go byte-array value.

use std::rc::Rc;

use crate::architecture::Architecture;
use crate::context::{OpId, VarnodeId};
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;

pub(crate) struct Extraction {
    pub input: VarnodeId,
    pub pointer: Rc<Datatype>,
    pub offset: u64,
}

fn memory_offset(container: i32, size: i32, lsb: u64, big: bool) -> Option<u64> {
    if !matches!(size, 1 | 2 | 4 | 8) {
        return None;
    }
    let container = u64::try_from(container).ok()?;
    let end = lsb.checked_add(size as u64)?;
    if end > container {
        return None;
    }
    Some(if big { container - end } else { lsb })
}

fn compatible(source: &Datatype, target: &Datatype, size: i32) -> bool {
    source.get_metatype() == type_metatype::TYPE_ARRAY
        && !crate::kuna_typequal::is_volatile(source)
        && source.get_array_base().is_some_and(|base| {
            base.get_size() == 1
                && base.get_align_size() == 1
                && !crate::kuna_typequal::is_volatile(&base)
                && matches!(
                    base.get_metatype(),
                    type_metatype::TYPE_UINT | type_metatype::TYPE_UNKNOWN
                )
        })
        && matches!(size, 1 | 2 | 4 | 8)
        && target.get_size() == size
        && !crate::kuna_typequal::is_volatile(target)
        && matches!(
            target.get_metatype(),
            type_metatype::TYPE_INT | type_metatype::TYPE_UINT | type_metatype::TYPE_UNKNOWN
        )
}

pub(crate) fn plan(
    fd: &Funcdata,
    arch: &Architecture,
    op: OpId,
    read_op: Option<OpId>,
    source: &Datatype,
    store_type: Option<Rc<Datatype>>,
) -> Option<Extraction> {
    let operation = fd.obank().get(op)?;
    if operation.code() != kuna_num::opcodes::OpCode::CPUI_SUBPIECE {
        return None;
    }
    let input = operation.get_in(0)?;
    let value = fd.vbank().get(input)?;
    if value.is_implied()
        || value.is_constant()
        || value.is_annotation()
        || value.is_persist()
        || value.is_volatile()
        || value.get_size() != source.get_size()
    {
        return None;
    }
    let lsb = fd.vbank().get(operation.get_in(1)?)?;
    let output = operation.get_out()?;
    let result = fd.vbank().get(output)?;
    let size = result.get_size();
    if !lsb.is_constant() || result.is_annotation() || result.is_volatile() {
        return None;
    }
    let offset = memory_offset(
        value.get_size(),
        size,
        lsb.get_offset(),
        value.get_space().is_big_endian(),
    )?;
    let facing = read_op
        .map(|read| fd.vn_type_read_facing(output, read))
        .unwrap_or_else(|| fd.vn_type_def_facing(output));
    let store = read_op
        .and_then(|read| fd.obank().get(read))
        .is_some_and(|read| {
            read.code() == kuna_num::opcodes::OpCode::CPUI_STORE && read.get_in(2) == Some(output)
        });
    let target = if store { store_type? } else { facing.clone() };
    if !compatible(source, &facing, size)
        || !compatible(source, &target, size)
        || ((target.get_metatype() == type_metatype::TYPE_UNKNOWN
            || facing.get_metatype() == type_metatype::TYPE_UNKNOWN)
            && crate::kuna_partialaccess::carries_float(fd, output, read_op.unwrap_or(op)))
    {
        return None;
    }
    let types = arch.types();
    Some(Extraction {
        input,
        pointer: types
            .get_type_pointer(types.get_size_of_pointer(), target, 1)
            .ok()?,
        offset,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dtype::{TypeFactory, TypeFactoryImpl};

    #[test]
    fn extraction_reads_the_same_value_in_both_memory_orders() {
        let value = 0x00112233445566778899aabbccddeeffu128;
        for size in [1, 2, 4, 8] {
            for lsb in 0..=16 - size {
                for big in [false, true] {
                    let bytes = if big {
                        value.to_be_bytes()
                    } else {
                        value.to_le_bytes()
                    };
                    let offset = memory_offset(16, size, lsb as u64, big).unwrap() as usize;
                    let mut actual = 0u64;
                    for i in 0..size as usize {
                        let shift = if big { size as usize - i - 1 } else { i };
                        actual |= u64::from(bytes[offset + i]) << (shift * 8);
                    }
                    let mask = (1u128 << (size * 8)) - 1;
                    assert_eq!(u128::from(actual), (value >> (lsb * 8)) & mask);
                }
            }
        }
        for (container, size, lsb) in [
            (16, 8, 9),
            (16, 3, 0),
            (16, 0, 0),
            (-1, 8, 0),
            (16, 8, u64::MAX),
        ] {
            for big in [false, true] {
                assert_eq!(memory_offset(container, size, lsb, big), None);
            }
        }
    }

    #[test]
    fn only_nonvolatile_byte_storage_and_integer_results_qualify() {
        let types = TypeFactoryImpl::new();
        types.set_default_alignment_map();
        types.set_max_basetype_size(8);
        let byte = types.get_base(1, type_metatype::TYPE_UINT).unwrap();
        let array = types.get_type_array(16, byte.clone()).unwrap();
        let word = types.get_base(8, type_metatype::TYPE_INT).unwrap();
        assert!(compatible(&array, &word, 8));
        assert!(!compatible(&array, &word, 4));
        for metatype in [
            type_metatype::TYPE_FLOAT,
            type_metatype::TYPE_PTR,
            type_metatype::TYPE_BOOL,
        ] {
            assert!(!compatible(&array, &Datatype::new(8, metatype), 8));
        }
        assert!(!compatible(&word, &word, 8));
        let words = types.get_type_array(2, word.clone()).unwrap();
        assert!(!compatible(&words, &word, 8));
        let volatile = |ty| {
            types
                .get_qualified_type(ty, crate::kuna_typequal::VOLATILE)
                .unwrap()
        };
        assert!(!compatible(&volatile(array.clone()), &word, 8));
        assert!(!compatible(&array, &volatile(word.clone()), 8));
        let bytes = types.get_type_array(16, volatile(byte)).unwrap();
        assert!(!compatible(&bytes, &word, 8));
    }
}
