//! Tests for installing a lowered switch along the raw paths of its compare tree.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace};
use kuna_num::opcodes::OpCode;

use super::*;
use crate::context::{ArchContext, TypeOp};

fn build_fd() -> Funcdata {
    let mut m = AddrSpaceManager::new();
    m.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    m.insert_space(Rc::new(UniqueSpace::new(1, 0, false))).unwrap();
    m.insert_space(Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "ram",
        false,
        8,
        1,
        2,
        addrspace_flags::hasphysical,
        1,
        1,
    )))
    .unwrap();
    let glb = Rc::new(ArchContext::new(m));
    let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
    Funcdata::new("func", "func", glb, Address::new(ram, 0x1000), 0x10000000, 0x40).unwrap()
}

fn ram(fd: &Funcdata, off: u64) -> Address {
    Address::new(Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap()), off)
}

fn new_block(fd: &mut Funcdata, off: u64) -> BlockId {
    let root = fd.bblocks_root_pub();
    let bl = fd.bblocks_mut().new_block_basic(root);
    let a = ram(fd, off);
    fd.set_basic_block_range(bl, &a, &a);
    bl
}

/// `ram:out = COPY #val` appended to `bl`.
fn set_const(fd: &mut Funcdata, bl: BlockId, off: u64, out: u64, val: u64) {
    let op = fd.new_op(1, ram(fd, off));
    fd.op_set_opcode(op, TypeOp::new(OpCode::CPUI_COPY, 0, "COPY"));
    let c = fd.new_constant(4, val);
    fd.op_set_input(op, c, 0).unwrap();
    let a = ram(fd, out);
    fd.new_varnode_out(4, &a, op).unwrap();
    fd.op_insert_end(op, bl);
}

/// `if (ram:0x38 == c) goto true_out` appended to `bl`, reading the selector afresh.
fn test_eq(fd: &mut Funcdata, bl: BlockId, off: u64, c: u64) {
    let cmp = fd.new_op(2, ram(fd, off));
    fd.op_set_opcode(cmp, TypeOp::new(OpCode::CPUI_INT_EQUAL, 0, "cmp"));
    let sel = ram(fd, 0x38);
    let v = fd.new_varnode(4, &sel, None);
    fd.op_set_input(cmp, v, 0).unwrap();
    let cvn = fd.new_constant(4, c);
    fd.op_set_input(cmp, cvn, 1).unwrap();
    let b = fd.new_unique_out(1, cmp).unwrap();
    fd.op_insert_end(cmp, bl);
    let cb = fd.new_op(2, ram(fd, off + 1));
    fd.op_set_opcode(cb, TypeOp::new(OpCode::CPUI_CBRANCH, 0, "CBRANCH"));
    let dest = fd.new_constant(8, 0);
    fd.op_set_input(cb, dest, 0).unwrap();
    fd.op_set_input(cb, b, 1).unwrap();
    fd.op_insert_end(cb, bl);
}

struct Hoisted {
    head: BlockId,
    b1: BlockId,
    b2: BlockId,
    ret: BlockId,
    dflt: BlockId,
}

/// `v = 10; if (x == 1) goto R; v = 20; if (x == 2) goto R; v = 30; if (x == 3)
/// goto R; D: v = 0; R: use(v)`.
fn hoisted(fd: &mut Funcdata) -> Hoisted {
    let head = new_block(fd, 0x1000);
    let b1 = new_block(fd, 0x1100);
    let b2 = new_block(fd, 0x1200);
    let dflt = new_block(fd, 0x3000);
    let ret = new_block(fd, 0x4000);
    for (bl, off, val, c) in [(head, 0x1000, 10, 1), (b1, 0x1100, 20, 2), (b2, 0x1200, 30, 3)] {
        set_const(fd, bl, off, 0x40, val);
        test_eq(fd, bl, off + 4, c);
    }
    set_const(fd, dflt, 0x3000, 0x40, 0);
    let a = ram(fd, 0x40);
    let read = fd.new_varnode(4, &a, None);
    let op = fd.new_op(1, ram(fd, 0x4000));
    fd.op_set_opcode(op, TypeOp::new(OpCode::CPUI_COPY, 0, "COPY"));
    fd.op_set_input(op, read, 0).unwrap();
    let out = ram(fd, 0x48);
    fd.new_varnode_out(4, &out, op).unwrap();
    fd.op_insert_end(op, ret);
    for (bl, next) in [(head, b1), (b1, b2), (b2, dflt)] {
        fd.bblocks_mut().add_edge(bl, next);
        fd.bblocks_mut().add_edge(bl, ret);
    }
    fd.bblocks_mut().add_edge(dflt, ret);
    Hoisted { head, b1, b2, ret, dflt }
}

fn spine(fd: &Funcdata) -> Vec<Address> {
    [0x1005, 0x1105, 0x1205].iter().map(|&o| ram(fd, o)).collect()
}

#[test]
fn each_value_runs_the_assignments_on_its_path() {
    let mut fd = build_fd();
    let h = hoisted(&mut fd);
    let sel = Selector::Storage(ram(&fd, 0x38), 4);
    let returns = vec![(ram(&fd, 0x40), 4)];
    let arms = plan(&fd, h.head, &spine(&fd), &sel, 4, &[1, 2, 3], &returns).expect("planned");
    let shape: Vec<(Vec<BlockId>, BlockId, Vec<u64>)> = arms.iter().map(|a| (a.via.clone(), a.target, a.labels.clone())).collect();
    assert_eq!(
        shape,
        vec![
            (vec![], h.ret, vec![1]),
            (vec![h.b1], h.ret, vec![2]),
            (vec![h.b1, h.b2], h.ret, vec![3]),
            (vec![h.b1, h.b2], h.dflt, vec![]),
        ]
    );
    let copies = |a: &PathArm| -> Vec<u64> {
        a.ops
            .iter()
            .filter_map(|&op| fd.obank().get(op))
            .filter(|o| o.code() == OpCode::CPUI_COPY)
            .filter_map(|o| o.get_in(0).and_then(|v| fd.vbank().get(v)).map(|v| v.get_offset()))
            .collect()
    };
    assert!(arms[0].ops.is_empty(), "the head already loaded the first value");
    assert_eq!(copies(&arms[1]), vec![20]);
    assert_eq!(copies(&arms[2]), vec![30], "the overwritten 20 is not repeated");
    assert_eq!(copies(&arms[3]), Vec::<u64>::new(), "the default overwrites the value before it is read");
}

#[test]
fn a_value_the_tree_cannot_decide_declines() {
    let mut fd = build_fd();
    let h = hoisted(&mut fd);
    let sel = Selector::Storage(ram(&fd, 0x50), 4);
    assert!(plan(&fd, h.head, &spine(&fd), &sel, 4, &[1, 2, 3], &[]).is_none());
}

#[test]
fn an_arm_block_copies_the_assignments_and_leaves_for_the_target() {
    let mut fd = build_fd();
    let h = hoisted(&mut fd);
    let ops: Vec<OpId> = fd.bb_ops(h.b1).into_iter().filter(|&op| fd.obank().get(op).unwrap().code() != OpCode::CPUI_CBRANCH).collect();
    let arm = PathArm { via: vec![h.b1], target: h.ret, labels: vec![2], ops };
    let blocks = build_arm_blocks(&mut fd, std::slice::from_ref(&arm)).unwrap();
    let nb = blocks[0];
    assert_ne!(nb, h.ret);
    let codes: Vec<OpCode> = fd.bb_ops(nb).iter().map(|&op| fd.obank().get(op).unwrap().code()).collect();
    assert_eq!(codes, vec![OpCode::CPUI_COPY, OpCode::CPUI_INT_EQUAL]);
    assert_eq!(fd.bblocks_ref().block(nb).get_out(0), h.ret);
}

#[test]
fn x86_style_flags_decide_the_branch() {
    let mut fd = build_fd();
    let bl = new_block(&mut fd, 0x1000);
    let sub = fd.new_op(2, ram(&fd, 0x1000));
    fd.op_set_opcode(sub, TypeOp::new(OpCode::CPUI_INT_SBORROW, 0, "SBORROW"));
    let sel = ram(&fd, 0x38);
    let a = fd.new_varnode(4, &sel, None);
    let c = fd.new_constant(4, 5);
    fd.op_set_input(sub, a, 0).unwrap();
    fd.op_set_input(sub, c, 1).unwrap();
    let of = fd.new_unique_out(1, sub).unwrap();
    fd.op_insert_end(sub, bl);
    let mut st = State::default();
    st.write(&fd, &sel, 4, Some(0x8000_0000));
    assert_eq!(eval(&fd, &st, sub), Some(1));
    st.write(&fd, &sel, 4, Some(7));
    assert_eq!(eval(&fd, &st, sub), Some(0));
    let _ = of;
}
