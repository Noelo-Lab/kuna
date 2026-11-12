use std::rc::Rc;

use kuna_base::space::{ConstantSpace, OtherSpace, UniqueSpace};

use super::*;

type Op = (OpCode, Option<VarnodeData>, Vec<VarnodeData>);

struct Spaces {
    cst: Rc<AddrSpace>,
    reg: Rc<AddrSpace>,
    ram: Rc<AddrSpace>,
}

fn spaces() -> Spaces {
    Spaces {
        cst: Rc::new(ConstantSpace::new()),
        reg: Rc::new(OtherSpace::new(1)),
        ram: Rc::new(UniqueSpace::new(2, 0, false)),
    }
}

fn vn(space: &Rc<AddrSpace>, offset: u64, size: u32) -> VarnodeData {
    VarnodeData { space: Some(space.clone()), offset, size }
}

fn esp(s: &Spaces) -> VarnodeData {
    vn(&s.reg, 0x10, 4)
}

fn add_esp(s: &Spaces, c: u64) -> Op {
    (OpCode::CPUI_INT_ADD, Some(esp(s)), vec![esp(s), vn(&s.cst, c, 4)])
}

#[test]
fn ret_imm16_states_its_extrapop() {
    let s = spaces();
    let eip = vn(&s.reg, 0x284, 4);
    let ops = vec![
        (OpCode::CPUI_LOAD, Some(eip.clone()), vec![vn(&s.cst, 3, 8), esp(&s)]),
        add_esp(&s, 4),
        add_esp(&s, 0x14),
        (OpCode::CPUI_RETURN, None, vec![eip]),
    ];
    let fx = effect(&ops, &esp(&s));
    assert_eq!(fx.returns, Some(0x18));
    assert!(!fx.falls_through);
}

#[test]
fn plain_ret_pops_only_the_return_address() {
    let s = spaces();
    let eip = vn(&s.reg, 0x284, 4);
    let ops = vec![add_esp(&s, 4), (OpCode::CPUI_RETURN, None, vec![eip])];
    assert_eq!(effect(&ops, &esp(&s)).returns, Some(4));
}

#[test]
fn direct_jump_is_followed_and_ends_the_path() {
    let s = spaces();
    let fx = effect(&[(OpCode::CPUI_BRANCH, None, vec![vn(&s.ram, 0x5fa1c0, 4)])], &esp(&s));
    assert_eq!(fx.targets, vec![0x5fa1c0]);
    assert!(!fx.falls_through);
    assert_eq!(fx.returns, None);
}

#[test]
fn conditional_branch_keeps_the_fall_through() {
    let s = spaces();
    let cond = vn(&s.reg, 0x206, 1);
    let fx = effect(&[(OpCode::CPUI_CBRANCH, None, vec![vn(&s.ram, 0x401020, 4), cond])], &esp(&s));
    assert_eq!(fx.targets, vec![0x401020]);
    assert!(fx.falls_through);
}

#[test]
fn intra_instruction_branch_and_indirect_jump() {
    let s = spaces();
    let rel = effect(&[(OpCode::CPUI_BRANCH, None, vec![vn(&s.cst, 2, 4)])], &esp(&s));
    assert!(rel.targets.is_empty() && rel.falls_through);
    let ind = effect(&[(OpCode::CPUI_BRANCHIND, None, vec![vn(&s.reg, 0, 4)])], &esp(&s));
    assert!(!ind.falls_through && ind.targets.is_empty());
}
