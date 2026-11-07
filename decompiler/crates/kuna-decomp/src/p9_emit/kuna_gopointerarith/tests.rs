use super::*;
use crate::context::ArchContext;
use crate::dtype::{TypeFactory, TypeFactoryImpl};
use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};

fn function() -> Funcdata {
    let mut manager = AddrSpaceManager::new();
    manager.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    manager
        .insert_space(Rc::new(UniqueSpace::new(1, 0, false)))
        .unwrap();
    let ram = Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "ram",
        false,
        8,
        1,
        2,
        addrspace_flags::hasphysical,
        1,
        1,
    ));
    manager.insert_space(Rc::clone(&ram)).unwrap();
    Funcdata::new(
        "words",
        "words",
        Rc::new(ArchContext::new(manager)),
        Address::new(ram, 0x1000),
        0x1000,
        0x40,
    )
    .unwrap()
}

fn expression(fd: &mut Funcdata, code: OpCode, size: i32, inputs: &[VarnodeId]) -> VarnodeId {
    let op = fd.new_op(inputs.len() as i32, fd.get_address().clone());
    fd.op_set_opcode_code(op, code);
    for (index, &input) in inputs.iter().enumerate() {
        fd.op_set_input(op, input, index as i32).unwrap();
    }
    let output = fd.new_unique_out(size, op).unwrap();
    fd.vbank_mut().get_mut(output).unwrap().set_implied();
    output
}

fn types(size: i32) -> (Rc<Datatype>, Rc<Datatype>) {
    let factory = TypeFactoryImpl::new();
    factory.setup_sizes(Some(size), size, size);
    let void = Rc::new(Datatype::new(0, type_metatype::TYPE_VOID));
    let pointer = factory.get_type_pointer(size, void, 1).unwrap();
    let word = Rc::new(Datatype::new(size, type_metatype::TYPE_UINT));
    (pointer, word)
}

#[test]
fn byte_add_keeps_the_pointer_base_through_same_width_casts_in_either_order() {
    let (pointer, word) = types(8);
    for reversed in [false, true] {
        let mut fd = function();
        let base = fd.new_unique(8, Some(Rc::clone(&pointer)));
        let offset = fd.new_unique(8, Some(Rc::clone(&word)));
        let cast = expression(&mut fd, OpCode::CPUI_CAST, 8, &[base]);
        let args = if reversed {
            [offset, cast]
        } else {
            [cast, offset]
        };
        let sum = expression(&mut fd, OpCode::CPUI_INT_ADD, 8, &args);
        let wrapped = expression(&mut fd, OpCode::CPUI_COPY, 8, &[sum]);
        let reader = fd.vbank().get(wrapped).unwrap().get_def().unwrap();
        let result = addition(&fd, wrapped, reader, |vn, _| {
            Rc::clone(if vn == base { &pointer } else { &word })
        })
        .unwrap();
        assert_eq!(result.base.value, base);
        assert_eq!(result.offset.value, offset);
        assert_eq!(
            result.operation,
            fd.vbank().get(sum).unwrap().get_def().unwrap()
        );
    }
}

#[test]
fn high_bit_constant_offsets_use_their_signed_native_word() {
    let (pointer, word) = types(8);
    for bits in [0, 1, i64::MAX as u64, 1u64 << 63, u64::MAX, u64::MAX - 7] {
        let mut fd = function();
        let base = fd.new_unique(8, Some(Rc::clone(&pointer)));
        let offset = fd.new_constant(8, bits);
        let sum = expression(&mut fd, OpCode::CPUI_INT_ADD, 8, &[base, offset]);
        let reader = fd.vbank().get(sum).unwrap().get_def().unwrap();
        let plan = addition(&fd, sum, reader, |vn, _| {
            Rc::clone(if vn == base { &pointer } else { &word })
        })
        .unwrap();
        assert_eq!(
            plan.negative_constant,
            (bits > i64::MAX as u64).then_some(bits as i64)
        );
        assert_eq!(plan.offset.value, offset);
    }
}

#[test]
fn byte_add_refuses_ambiguous_noninteger_effectful_or_narrow_operands() {
    let (pointer, word) = types(8);
    for (size, meta, implied, opcode) in [
        (8, type_metatype::TYPE_PTR, false, OpCode::CPUI_INT_ADD),
        (8, type_metatype::TYPE_FLOAT, false, OpCode::CPUI_INT_ADD),
        (8, type_metatype::TYPE_BOOL, false, OpCode::CPUI_INT_ADD),
        (4, type_metatype::TYPE_UINT, false, OpCode::CPUI_INT_ADD),
        (8, type_metatype::TYPE_UINT, true, OpCode::CPUI_INT_ADD),
        (8, type_metatype::TYPE_UINT, false, OpCode::CPUI_INT_SUB),
    ] {
        let mut fd = function();
        let base = fd.new_unique(8, Some(Rc::clone(&pointer)));
        let ty = if meta == type_metatype::TYPE_PTR {
            Rc::clone(&pointer)
        } else {
            Rc::new(Datatype::new(size, meta))
        };
        let offset = if implied {
            let destination = fd.new_constant(8, 0x1000);
            expression(&mut fd, OpCode::CPUI_CALL, size, &[destination])
        } else {
            fd.new_unique(size, Some(Rc::clone(&ty)))
        };
        let sum = expression(&mut fd, opcode, 8, &[base, offset]);
        let reader = fd.vbank().get(sum).unwrap().get_def().unwrap();
        assert!(
            addition(&fd, sum, reader, |vn, _| {
                Rc::clone(if vn == base {
                    &pointer
                } else if vn == offset {
                    &ty
                } else {
                    &word
                })
            })
            .is_none(),
            "{size} {meta:?} {implied} {opcode:?}"
        );
    }
}

#[test]
fn byte_add_refuses_volatile_constants_and_excessive_cast_depth() {
    let (pointer, word) = types(8);
    for kind in [0, 1, 2, 3, 4, 5] {
        let mut fd = function();
        let base = if kind == 0 {
            fd.new_constant(8, 0x1000)
        } else {
            fd.new_unique(8, Some(Rc::clone(&pointer)))
        };
        if kind == 1 {
            fd.vbank_mut()
                .get_mut(base)
                .unwrap()
                .set_flags_pub(crate::varnode::varnode_flags::volatil);
        }
        if kind == 4 {
            fd.vbank_mut()
                .get_mut(base)
                .unwrap()
                .set_flags_pub(crate::varnode::varnode_flags::persist);
        }
        let offset = if kind == 5 {
            let value = fd.new_unique(8, Some(Rc::clone(&word)));
            fd.vbank_mut()
                .get_mut(value)
                .unwrap()
                .set_flags_pub(crate::varnode::varnode_flags::persist);
            value
        } else {
            fd.new_constant(8, 7)
        };
        let mut sum = expression(&mut fd, OpCode::CPUI_INT_ADD, 8, &[base, offset]);
        if kind == 2 {
            for _ in 0..8 {
                sum = expression(&mut fd, OpCode::CPUI_CAST, 8, &[sum]);
            }
        } else if kind == 3 {
            fd.vbank_mut().get_mut(sum).unwrap().clear_implied();
        }
        let reader = fd.vbank().get(sum).unwrap().get_def().unwrap();
        assert!(
            addition(&fd, sum, reader, |vn, _| {
                Rc::clone(if vn == base { &pointer } else { &word })
            })
            .is_none(),
            "kind {kind}"
        );
    }
}

#[test]
fn native_width_direct_additions_and_negative_offsets_preserve_their_base() {
    for size in [4, 8] {
        let (pointer, word) = types(size);
        let mask = kuna_base::address::calc_mask(size);
        for bits in [0, 1, mask >> 1, (mask >> 1) + 1, mask - 7, mask] {
            let mut fd = function();
            let base = fd.new_unique(size, Some(Rc::clone(&pointer)));
            let offset = fd.new_constant(size, bits);
            let sum = expression(&mut fd, OpCode::CPUI_INT_ADD, size, &[base, offset]);
            fd.vbank_mut().get_mut(sum).unwrap().clear_implied();
            let op = fd.vbank().get(sum).unwrap().get_def().unwrap();
            let plan = operation(&fd, op, |vn, _| {
                Rc::clone(if vn == base { &pointer } else { &word })
            })
            .unwrap();
            let signed = if size == 4 {
                bits as i32 as i64
            } else {
                bits as i64
            };
            assert_eq!(plan.negative_constant, (signed < 0).then_some(signed));
            assert_eq!(plan.base.value, base);
            assert_eq!(plan.offset.value, offset);
        }
    }
}

#[test]
fn pointer_word_size_and_cast_width_must_match_the_addition() {
    for size in [4, 8] {
        for kind in [0, 1, 2] {
            let (pointer, word) = types(size);
            let factory = TypeFactoryImpl::new();
            factory.setup_sizes(Some(size), size, size);
            let base_type = match kind {
                0 => types(12 - size).0,
                1 => factory
                    .get_type_pointer(size, Rc::new(Datatype::new(0, type_metatype::TYPE_VOID)), 2)
                    .unwrap(),
                _ => Rc::clone(&pointer),
            };
            let mut fd = function();
            let base = fd.new_unique(
                if kind == 2 { 12 - size } else { size },
                Some(Rc::clone(&base_type)),
            );
            let cast = expression(&mut fd, OpCode::CPUI_CAST, size, &[base]);
            let offset = fd.new_constant(size, 8);
            let sum = expression(&mut fd, OpCode::CPUI_INT_ADD, size, &[cast, offset]);
            let op = fd.vbank().get(sum).unwrap().get_def().unwrap();
            assert!(
                addition(&fd, sum, op, |vn, _| {
                    Rc::clone(if vn == base { &base_type } else { &word })
                })
                .is_none(),
                "size {size}, kind {kind}"
            );
        }
    }
}
