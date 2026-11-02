use super::*;
use crate::context::ArchContext;
use crate::dtype::{flags, DatatypeKind, TypeField};
use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};
use kuna_num::opcodes::OpCode;

fn structure(field: Rc<Datatype>) -> Datatype {
    let mut whole = Datatype::new(16, type_metatype::TYPE_STRUCT);
    whole.kind = DatatypeKind::Struct {
        field: vec![
            TypeField::new(0, 0, "first", Rc::clone(&field)),
            TypeField::new(1, 8, "second", field),
        ],
        bitfield: vec![],
    };
    whole
}

fn slice(big: bool, whole: &Datatype, target: Rc<Datatype>, offset: i64) -> (Funcdata, OpId) {
    let mut manager = AddrSpaceManager::new();
    manager.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    manager
        .insert_space(Rc::new(UniqueSpace::new(1, 0, big)))
        .unwrap();
    let ram = Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "ram",
        big,
        8,
        1,
        2,
        addrspace_flags::hasphysical,
        1,
        1,
    ));
    manager.insert_space(Rc::clone(&ram)).unwrap();
    let arch = Rc::new(ArchContext::new(manager));
    let mut fd = Funcdata::new(
        "slice",
        "slice",
        arch,
        Address::new(Rc::clone(&ram), 0x1000),
        0x1000,
        0x40,
    )
    .unwrap();
    let source = fd.new_varnode(16, &Address::new(ram, 0x2000), Some(Rc::new(whole.clone())));
    let op = fd.new_op(2, fd.get_address().clone());
    fd.op_set_opcode_code(op, OpCode::CPUI_SUBPIECE);
    fd.op_set_input(op, source, 0).unwrap();
    let size = target.get_size();
    let lsb = if big {
        16 - i64::from(size) - offset
    } else {
        offset
    };
    let constant = fd.new_constant(4, lsb as u64);
    fd.op_set_input(op, constant, 1).unwrap();
    let output = fd.new_unique_out(size, op).unwrap();
    fd.vn_update_type(output, target);
    (fd, op)
}

#[test]
fn integer_slices_match_storage_bytes_in_both_orders() {
    let bits = 0xfedc_ba98_7654_3210_u64;
    for big in [false, true] {
        let bytes = if big {
            bits.to_be_bytes()
        } else {
            bits.to_le_bytes()
        };
        for source_kind in [type_metatype::TYPE_INT, type_metatype::TYPE_UINT] {
            let whole = structure(Rc::new(Datatype::new(8, source_kind)));
            for target_kind in [type_metatype::TYPE_INT, type_metatype::TYPE_UINT] {
                for size in [1, 2, 4, 8] {
                    for offset in 0..=8 - size {
                        let target = Rc::new(Datatype::new(size, target_kind));
                        let (fd, op) = slice(big, &whole, target, i64::from(8 + offset));
                        let (index, within) = whole
                            .find_truncation(i64::from(8 + offset), size, op, 1)
                            .unwrap()
                            .unwrap();
                        let extraction = plan(&fd, op, &whole, index, within);
                        let shifted = if let Some(extraction) = extraction {
                            assert_eq!(extraction.target.get_size(), size);
                            assert_eq!(extraction.target.get_metatype(), target_kind);
                            bits >> extraction.shift
                        } else {
                            assert_eq!((size, offset, target_kind), (8, 0, source_kind));
                            bits
                        };
                        let mut expected = 0_u64;
                        for index in 0..size {
                            let byte = bytes[(offset + index) as usize];
                            let significance = if big { size - 1 - index } else { index };
                            expected |= u64::from(byte) << (significance * 8);
                        }
                        let mask = u64::MAX >> (64 - size * 8);
                        assert_eq!(
                            shifted & mask,
                            expected,
                            "big={big} size={size} offset={offset}"
                        );
                    }
                }
            }
        }
    }
}

#[test]
fn unsupported_fields_and_invalid_ranges_keep_the_existing_path() {
    let integer = Rc::new(Datatype::new(8, type_metatype::TYPE_INT));
    let target = Rc::new(Datatype::new(4, type_metatype::TYPE_INT));
    let mut excluded = vec![];
    for kind in [
        type_metatype::TYPE_FLOAT,
        type_metatype::TYPE_PTR,
        type_metatype::TYPE_BOOL,
        type_metatype::TYPE_UNKNOWN,
    ] {
        excluded.push(structure(Rc::new(Datatype::new(8, kind))));
    }
    let mut enumeration = (*integer).clone();
    enumeration.flags |= flags::enumtype;
    excluded.push(structure(Rc::new(enumeration)));
    excluded.push(structure(crate::kuna_typequal::qualified_type(
        Rc::clone(&integer),
        crate::kuna_typequal::VOLATILE,
    )));
    for flag in [flags::has_bitfields, flags::type_incomplete] {
        let mut whole = structure(Rc::clone(&integer));
        whole.flags |= flag;
        excluded.push(whole);
    }
    let mut volatile = structure(Rc::clone(&integer));
    volatile.qualifiers = crate::kuna_typequal::VOLATILE;
    excluded.push(volatile);
    for whole in excluded {
        let (fd, op) = slice(false, &whole, Rc::clone(&target), 8);
        assert!(plan(&fd, op, &whole, 1, 0).is_none());
    }
    let whole = structure(integer);
    let (fd, op) = slice(false, &whole, target, 8);
    for offset in [-1, 5, i64::MAX] {
        assert!(plan(&fd, op, &whole, 1, offset).is_none());
    }
}
