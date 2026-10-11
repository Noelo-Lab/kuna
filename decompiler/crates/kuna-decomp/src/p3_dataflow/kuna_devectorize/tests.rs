use super::*;
use crate::{context::ArchContext, context::TypeOp, dtype::type_metatype as Meta, dtype::Datatype};
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
    let mut fd = Funcdata::new(
        "devectorize",
        "devectorize",
        Rc::new(ArchContext::new(manager)),
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

fn build(fd: &mut Funcdata, anchor: OpId, code: OpCode, width: i32, inputs: &[VarnodeId]) -> VarnodeId {
    let addr = fd.obank().get(anchor).unwrap().get_addr().clone();
    let op = fd.new_op(inputs.len() as i32, addr);
    fd.op_set_all_input(op, inputs).unwrap();
    fd.op_set_opcode(op, TypeOp::new(code, 0, format!("{code:?}")));
    let out = fd.new_unique_out(width, op).unwrap();
    fd.op_insert_before(op, anchor);
    out
}

fn load(fd: &mut Funcdata, anchor: OpId, width: i32, addr: VarnodeId) -> VarnodeId {
    let ram = fd.get_arch().manage().get_space_by_name("ram").unwrap().get_index() as u64;
    let space = fd.new_constant(8, ram);
    build(fd, anchor, CPUI_LOAD, width, &[space, addr])
}

fn lane_of(fd: &Funcdata, v: VarnodeId) -> Option<(VarnodeId, i64, i64)> {
    lane(fd, v).map(|(op, n, e)| (fd.obank().get(op).unwrap().get_out().unwrap(), n, e))
}

#[test]
fn every_zero_extended_lane_form_names_its_bytes() {
    let (mut fd, anchor) = fixture();
    let addr = input(&mut fd, 8, 0x200);
    let word = load(&mut fd, anchor, 4, addr);
    let c = |fd: &mut Funcdata, v: u64| fd.new_constant(4, v);
    let k8 = c(&mut fd, 8);
    let k24 = c(&mut fd, 24);
    let byte = c(&mut fd, 0xff);
    let half = c(&mut fd, 0xffff);
    let two = c(&mut fd, 2);

    let low = build(&mut fd, anchor, CPUI_INT_AND, 4, &[word, byte]);
    assert_eq!(lane_of(&fd, low), Some((word, 0, 1)));
    let shifted = build(&mut fd, anchor, CPUI_INT_RIGHT, 4, &[word, k8]);
    let second = build(&mut fd, anchor, CPUI_INT_AND, 4, &[shifted, byte]);
    assert_eq!(lane_of(&fd, second), Some((word, 1, 1)));
    let top = build(&mut fd, anchor, CPUI_INT_RIGHT, 4, &[word, k24]);
    assert_eq!(lane_of(&fd, top), Some((word, 3, 1)));
    let piece = build(&mut fd, anchor, CPUI_SUBPIECE, 1, &[word, two]);
    let widened = build(&mut fd, anchor, CPUI_INT_ZEXT, 4, &[piece]);
    assert_eq!(lane_of(&fd, widened), Some((word, 2, 1)));
    let pair = build(&mut fd, anchor, CPUI_INT_AND, 4, &[shifted, half]);
    assert_eq!(lane_of(&fd, pair), Some((word, 1, 2)));

    let odd = c(&mut fd, 0x7f);
    let partial = build(&mut fd, anchor, CPUI_INT_AND, 4, &[word, odd]);
    assert_eq!(lane_of(&fd, partial), None);
    let signed = build(&mut fd, anchor, CPUI_INT_SRIGHT, 4, &[word, k24]);
    assert_eq!(lane_of(&fd, signed), None);
}

#[test]
fn addresses_split_into_base_index_scale_and_offset() {
    let (mut fd, anchor) = fixture();
    let base = input(&mut fd, 8, 0x200);
    let idx = input(&mut fd, 8, 0x300);
    let other = input(&mut fd, 8, 0x400);
    let four = fd.new_constant(8, 4);
    let one = fd.new_constant(8, 1);
    let two = fd.new_constant(8, 2);
    let six = fd.new_constant(8, 6);

    let check = |fd: &Funcdata, v: VarnodeId| {
        let mut a = Addr::default();
        decompose(fd, v, idx, &mut a).then_some((a.base, a.scale, a.off))
    };
    let plain = build(&mut fd, anchor, CPUI_INT_ADD, 8, &[base, idx]);
    assert_eq!(check(&fd, plain), Some((Some(base), Some(1), 0)));
    let bumped = build(&mut fd, anchor, CPUI_INT_ADD, 8, &[idx, four]);
    let later = build(&mut fd, anchor, CPUI_INT_ADD, 8, &[base, bumped]);
    assert_eq!(check(&fd, later), Some((Some(base), Some(1), 4)));
    let ptr = build(&mut fd, anchor, CPUI_PTRADD, 8, &[base, bumped, one]);
    assert_eq!(check(&fd, ptr), Some((Some(base), Some(1), 4)));
    let scaled = build(&mut fd, anchor, CPUI_PTRADD, 8, &[base, idx, two]);
    assert_eq!(check(&fd, scaled), Some((Some(base), Some(2), 0)));
    let doubled = build(&mut fd, anchor, CPUI_INT_LEFT, 8, &[idx, one]);
    let inner = build(&mut fd, anchor, CPUI_INT_ADD, 8, &[base, doubled]);
    let strided = build(&mut fd, anchor, CPUI_INT_ADD, 8, &[inner, six]);
    assert_eq!(check(&fd, strided), Some((Some(base), Some(2), 6)));

    let two_bases = build(&mut fd, anchor, CPUI_INT_ADD, 8, &[plain, other]);
    assert_eq!(check(&fd, two_bases), None);
    let twice = build(&mut fd, anchor, CPUI_INT_ADD, 8, &[plain, idx]);
    assert_eq!(check(&fd, twice), None);
}
