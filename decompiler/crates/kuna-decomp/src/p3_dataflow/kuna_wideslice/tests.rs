use super::*;
use crate::{context::ArchContext, dtype::Datatype};
use kuna_base::{
    address::Address,
    space::{addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace},
};
use std::rc::Rc;

fn fixture() -> (Funcdata, OpId) {
    let mut manager = AddrSpaceManager::new();
    manager.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    manager.insert_space(Rc::new(UniqueSpace::new(1, 0, false))).unwrap();
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
    manager.insert_space(ram.clone()).unwrap();
    let mut arch = ArchContext::new(manager);
    arch.wide_slice_reduce = true;
    let mut fd = Funcdata::new(
        "wide_slice",
        "wide_slice",
        Rc::new(arch),
        Address::new(ram.clone(), 0x1000),
        0x10000000,
        0x40,
    )
    .unwrap();
    let root = fd.bblocks_ref().root.unwrap();
    let block = fd.bblocks_mut().new_block_basic(root);
    let anchor = fd.new_op(1, Address::new(ram, 0x1000));
    fd.op_set_opcode(anchor, TypeOp::new(CPUI_RETURN, 0, "RETURN"));
    fd.op_insert(anchor, block, None);
    (fd, anchor)
}

fn input(fd: &mut Funcdata, width: i32, offset: u64) -> VarnodeId {
    let ram = fd.get_arch().manage().get_space_by_name("ram").unwrap().clone();
    let vn = fd.vbank_mut().create(
        width,
        Address::new(ram, offset),
        Rc::new(Datatype::new(width, Meta::TYPE_UNKNOWN)),
    );
    fd.vbank_mut().set_input(vn, &mut |_, _, _| Ok(())).unwrap()
}

fn mask(width: i32) -> u128 {
    u128::MAX >> ((16 - width) * 8)
}

// Independent bitvector evaluator, including bits above the engine's 64-bit
// constant representation.
fn evaluate(fd: &Funcdata, vn: VarnodeId, inputs: &[(VarnodeId, u128)]) -> u128 {
    let node = fd.vbank().get(vn).unwrap();
    let value = if node.is_constant() {
        node.get_offset() as u128
    } else if let Some((_, value)) = inputs.iter().find(|(id, _)| *id == vn) {
        *value
    } else {
        let op = fd.obank().get(node.get_def().unwrap()).unwrap();
        let a = evaluate(fd, op.get_in(0).unwrap(), inputs);
        let b = op.get_in(1).map(|v| evaluate(fd, v, inputs)).unwrap_or(0);
        match op.code() {
            CPUI_COPY | CPUI_INT_ZEXT => a,
            CPUI_PIECE => (a << (8 * size(fd, op.get_in(1).unwrap()))) | b,
            CPUI_SUBPIECE => a >> (b * 8),
            CPUI_INT_LEFT => {
                if b >= 128 {
                    0
                } else {
                    a << b
                }
            }
            CPUI_INT_RIGHT => {
                if b >= 128 {
                    0
                } else {
                    a >> b
                }
            }
            CPUI_INT_AND => a & b,
            CPUI_INT_OR => a | b,
            CPUI_INT_XOR => a ^ b,
            code => panic!("unexpected {code:?}"),
        }
    };
    value & mask(node.get_size())
}

fn check_slice(
    fd: &mut Funcdata,
    anchor: OpId,
    value: VarnodeId,
    offset: i32,
    width: i32,
    inputs: &[(VarnodeId, u128)],
) {
    let off = fd.new_constant(4, offset as u64);
    let result = insert(fd, anchor, CPUI_SUBPIECE, width, &[value, off]);
    let expected = evaluate(fd, result, inputs);
    let op = fd.vbank().get(result).unwrap().get_def().unwrap();
    assert_eq!(RuleWideSlice::new("analysis").apply_op(op, fd), 1);
    assert_eq!(evaluate(fd, result, inputs), expected, "offset={offset}, width={width}");
}

#[test]
fn slices_across_wide_piece_boundaries_preserve_all_bits() {
    for (hi_size, lo_size) in [(8, 5), (5, 8), (1, 12), (7, 5), (7, 8)] {
        for offset in 0..hi_size + lo_size {
            for width in 1..=hi_size + lo_size - offset {
                if width == hi_size + lo_size {
                    continue;
                }
                let (mut fd, anchor) = fixture();
                let hi = input(&mut fd, hi_size, 0x200);
                let lo = input(&mut fd, lo_size, 0x300);
                let value = insert(&mut fd, anchor, CPUI_PIECE, hi_size + lo_size, &[hi, lo]);
                for bits in [0, u128::MAX, 0x0123456789abcdeffedcba9876543210] {
                    check_slice(&mut fd, anchor, value, offset, width, &[(hi, bits), (lo, !bits)]);
                }
            }
        }
    }
}

#[test]
fn wide_shift_slices_preserve_zero_fill_on_both_boundaries() {
    for code in [CPUI_INT_LEFT, CPUI_INT_RIGHT] {
        for total in [9, 10, 12, 13, 15] {
            for shift in [0, 1, 3, 8, 12, 16, 32] {
                let (mut fd, anchor) = fixture();
                let source = input(&mut fd, total, 0x200);
                let amount = fd.new_constant(4, shift * 8);
                let value = insert(&mut fd, anchor, code, total, &[source, amount]);
                for offset in 0..total {
                    for width in 1..=total - offset {
                        if width == total {
                            continue;
                        }
                        check_slice(
                            &mut fd,
                            anchor,
                            value,
                            offset,
                            width,
                            &[(source, 0xf123456789abcdeffedcba9876543210)],
                        );
                    }
                }
            }
        }
    }
}

#[test]
fn wide_bitwise_slices_preserve_both_operands() {
    for code in [CPUI_INT_AND, CPUI_INT_OR, CPUI_INT_XOR] {
        for total in [9, 10, 12, 13, 15] {
            let (mut fd, anchor) = fixture();
            let left = input(&mut fd, total, 0x200);
            let right = input(&mut fd, total, 0x300);
            let value = insert(&mut fd, anchor, code, total, &[left, right]);
            let inputs = [
                (left, 0x89abcdef01234567fedcba9876543210),
                (right, 0x0f0f0f0f0f0f0f0ff0f0f0f0f0f0f0f0),
            ];
            for offset in 0..total {
                for width in 1..total - offset {
                    check_slice(&mut fd, anchor, value, offset, width, &inputs);
                }
            }
        }
    }
}

#[test]
fn native_width_non_byte_and_arithmetic_operations_are_left_intact() {
    for (total, code, shift) in [
        (8, CPUI_INT_LEFT, 8),
        (16, CPUI_INT_LEFT, 8),
        (16, CPUI_INT_LEFT, 7),
        (16, CPUI_INT_RIGHT, 9),
        (16, CPUI_INT_SRIGHT, 8),
        (13, CPUI_INT_LEFT, 7),
        (13, CPUI_INT_RIGHT, 9),
        (13, CPUI_INT_SRIGHT, 8),
    ] {
        let (mut fd, anchor) = fixture();
        let source = input(&mut fd, total, 0x200);
        let amount = fd.new_constant(4, shift);
        let value = insert(&mut fd, anchor, code, total, &[source, amount]);
        let offset = fd.new_constant(4, 2);
        let result = insert(&mut fd, anchor, CPUI_SUBPIECE, 4, &[value, offset]);
        let op = fd.vbank().get(result).unwrap().get_def().unwrap();
        assert_eq!(RuleWideSlice::new("analysis").apply_op(op, &mut fd), 0);
        assert_eq!(fd.obank().get(op).unwrap().get_in(0), Some(value));
    }
}
