use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace};

use super::*;
use crate::context::{ArchContext, TypeOp};
use crate::op::pcodeop_flags;

fn fd() -> Funcdata {
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
    Funcdata::new("lj", "lj", glb, Address::new(ram, 0x1000), 0x10000000, 0x40).unwrap()
}

/// Append an op with `out` (a fresh temporary when true) reading `ins`.
fn op(fd: &mut Funcdata, bl: BlockId, at: u64, opc: OpCode, ins: &[VarnodeId], out: bool) -> (OpId, Option<VarnodeId>) {
    let ram = Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap());
    let o = fd.new_op(ins.len() as i32, Address::new(ram, at));
    let flags = match opc {
        OpCode::CPUI_CBRANCH => pcodeop_flags::branch,
        OpCode::CPUI_MULTIEQUAL => pcodeop_flags::marker,
        _ => 0,
    };
    fd.obank_mut().change_opcode(o, TypeOp::new(opc, flags, format!("{opc:?}")));
    for (i, &v) in ins.iter().enumerate() {
        fd.op_set_input(o, v, i as i32).unwrap();
    }
    let w = out.then(|| fd.new_unique_out(4, o).unwrap());
    fd.op_insert(o, bl, None);
    (o, w)
}

struct Loop {
    fd: Funcdata,
    h: BlockId,
    a: BlockId,
    b: BlockId,
    x: BlockId,
    ops_b: Vec<OpId>,
}

/// `entry -> H; H -> A, B; A -> H | X; B -> X | H`, each latch stepping the
/// pointer by one and testing it against zero; `b_test` is B's comparison, which
/// tests the opposite sense with its targets swapped when it is INT_NOTEQUAL.
fn two_latches(b_test: OpCode) -> Loop {
    let mut fd = fd();
    let root = fd.bblocks_root_pub();
    let [entry, h, a, b, x] = [(); 5].map(|_| fd.bblocks_mut().new_block_basic(root));
    for (s, t) in [(entry, h), (h, a), (h, b), (a, h), (a, x), (b, x), (b, h)] {
        fd.bblocks_mut().add_edge(s, t);
    }
    fd.bblocks_mut().set_start_block(root, entry);
    let zero = fd.new_constant(4, 0);
    let (_, p0) = op(&mut fd, entry, 0x1000, OpCode::CPUI_COPY, &[zero], true);
    let tsrc = fd.new_constant(4, 0x1100);
    let (_, tgt) = op(&mut fd, entry, 0x1004, OpCode::CPUI_COPY, &[tsrc], true);
    let (tgt, p0) = (tgt.unwrap(), p0.unwrap());
    let (phi, p) = op(&mut fd, h, 0x1100, OpCode::CPUI_MULTIEQUAL, &[p0, p0, p0], true);
    let p = p.unwrap();
    let (hc, hv) = {
        let one = fd.new_constant(4, 1);
        op(&mut fd, h, 0x1104, OpCode::CPUI_INT_EQUAL, &[p, one], true)
    };
    let _ = hc;
    op(&mut fd, h, 0x1108, OpCode::CPUI_CBRANCH, &[tgt, hv.unwrap()], false);
    let mut latch = |fd: &mut Funcdata, bl: BlockId, at: u64, test: OpCode| {
        let one = fd.new_constant(4, 1);
        let zero = fd.new_constant(4, 0);
        let (o1, next) = op(fd, bl, at, OpCode::CPUI_INT_ADD, &[p, one], true);
        let (o2, c) = op(fd, bl, at + 4, test, &[next.unwrap(), zero], true);
        let (o3, _) = op(fd, bl, at + 8, OpCode::CPUI_CBRANCH, &[tgt, c.unwrap()], false);
        (next.unwrap(), vec![o1, o2, o3])
    };
    let (pa, _) = latch(&mut fd, a, 0x1200, OpCode::CPUI_INT_EQUAL);
    let (pb, ops_b) = latch(&mut fd, b, 0x1300, b_test);
    let (sa, sb) = (fd.bblocks_ref().block(h).get_in_index(a), fd.bblocks_ref().block(h).get_in_index(b));
    fd.op_set_input(phi, pa, sa).unwrap();
    fd.op_set_input(phi, pb, sb).unwrap();
    let (xa, xb) = (fd.bblocks_ref().block(x).get_in_index(a), fd.bblocks_ref().block(x).get_in_index(b));
    let mut ins = [pa, pa];
    ins[xb as usize] = pb;
    let _ = xa;
    op(&mut fd, x, 0x1400, OpCode::CPUI_MULTIEQUAL, &ins, true);
    fd.structure_reset();
    Loop { fd, h, a, b, x, ops_b }
}

#[test]
fn copies_testing_the_opposite_sense_join_into_one_latch() {
    let mut l = two_latches(OpCode::CPUI_INT_NOTEQUAL);
    let plan = match_latches(&l.fd, l.h, l.a, l.b).expect("the two latches are copies");
    assert_eq!(plan.ops_a.len(), 3);
    let n = l.fd.bblocks_get_size();
    execute(&mut l.fd, &plan);
    assert_eq!(l.fd.bblocks_get_size(), n + 1, "one new latch block");
    assert!(l.ops_b.iter().all(|&o| l.fd.obank().get(o).unwrap().is_dead()), "B's copy is deleted");
    assert_eq!(l.fd.bblocks_ref().block(l.h).size_in(), 2, "the header keeps one back edge");
    assert_eq!(l.fd.bblocks_ref().block(l.x).size_in(), 1, "the exit is entered once");
    let phi = l.fd.bb_op_head(l.h).unwrap();
    assert_eq!(l.fd.obank().get(phi).unwrap().num_input(), 2);
    let exit = l.fd.bb_op_head(l.x).unwrap();
    assert_eq!(l.fd.obank().get(exit).unwrap().code(), OpCode::CPUI_COPY, "a one-input MULTIEQUAL becomes a COPY");
}

#[test]
fn copies_testing_different_things_are_not_joined() {
    let l = two_latches(OpCode::CPUI_INT_SLESS);
    assert!(match_latches(&l.fd, l.h, l.a, l.b).is_err());
}

#[test]
fn a_comparison_and_its_complement() {
    assert_eq!(complement(OpCode::CPUI_INT_EQUAL), Some((OpCode::CPUI_INT_NOTEQUAL, false)));
    assert_eq!(complement(OpCode::CPUI_INT_LESS), Some((OpCode::CPUI_INT_LESSEQUAL, true)));
    assert_eq!(complement(OpCode::CPUI_INT_SLESSEQUAL), Some((OpCode::CPUI_INT_SLESS, true)));
    assert_eq!(complement(OpCode::CPUI_INT_ADD), None);
}

#[test]
fn the_option_parses_its_values() {
    use crate::p8_structure::kuna_dedupitetail::OptionDedupIteTail;
    let latch = OptionDedupIteTail.apply("latch").unwrap();
    assert!(latch.0 && latch.1);
    let on = OptionDedupIteTail.apply("on").unwrap();
    assert!(on.0 && !on.1);
    let off = OptionDedupIteTail.apply("off").unwrap();
    assert!(!off.0 && !off.1);
    assert!(OptionDedupIteTail.apply("sideways").is_err());
}
