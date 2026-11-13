use std::rc::Rc;

use kuna_base::space::{AddrSpace, ConstantSpace, OtherSpace, UniqueSpace};

use super::*;

type Op = (OpCode, Option<VarnodeData>, Vec<VarnodeData>);

struct Fx {
    cst: Rc<AddrSpace>,
    reg: Rc<AddrSpace>,
    tmp: Rc<AddrSpace>,
    regs: Regs,
}

fn vn(space: &Rc<AddrSpace>, offset: u64, size: u32) -> VarnodeData {
    VarnodeData { space: Some(space.clone()), offset, size }
}

fn fx() -> Fx {
    let cst = Rc::new(ConstantSpace::new());
    let reg = Rc::new(OtherSpace::new(1));
    let tmp = Rc::new(UniqueSpace::new(2, 0, false));
    let regs = Regs { sp: vn(&reg, 0x10, 4), bp: vn(&reg, 0x14, 4) };
    Fx { cst, reg, tmp, regs }
}

fn at_entry() -> Frame {
    Frame { sp: Some(0), bp: None }
}

/// `mov eax, [esp+off]`
fn load_sp(f: &Fx, off: u64) -> Vec<Op> {
    let t = vn(&f.tmp, 0x100, 4);
    vec![
        (OpCode::CPUI_INT_ADD, Some(t.clone()), vec![f.regs.sp.clone(), vn(&f.cst, off, 4)]),
        (OpCode::CPUI_LOAD, Some(vn(&f.reg, 0, 4)), vec![vn(&f.cst, 3, 8), t]),
    ]
}

#[test]
fn reading_the_first_argument_uses_it() {
    let f = fx();
    assert_eq!(step(&load_sp(&f, 4), &f.regs, at_entry()).arg_end, 8);
    assert_eq!(step(&load_sp(&f, 0), &f.regs, at_entry()).arg_end, 4, "the return address is not an argument");
}

#[test]
fn a_push_moves_the_frame_and_a_local_read_is_no_use() {
    let f = fx();
    let push = vec![
        (OpCode::CPUI_INT_SUB, Some(f.regs.sp.clone()), vec![f.regs.sp.clone(), vn(&f.cst, 4, 4)]),
        (OpCode::CPUI_STORE, None, vec![vn(&f.cst, 3, 8), f.regs.sp.clone(), vn(&f.reg, 0x18, 4)]),
    ];
    let s = step(&push, &f.regs, at_entry());
    assert_eq!(s.frame.sp, Some(-4));
    assert_eq!(s.arg_end, 4);
    assert_eq!(step(&load_sp(&f, 8), &f.regs, s.frame).arg_end, 8, "[esp+8] after one push is the first argument");
    assert_eq!(step(&load_sp(&f, 4), &f.regs, s.frame).arg_end, 4, "[esp+4] after one push is the return address");
}

#[test]
fn an_ebp_frame_is_followed() {
    let f = fx();
    let mov = vec![(OpCode::CPUI_COPY, Some(f.regs.bp.clone()), vec![f.regs.sp.clone()])];
    let s = step(&mov, &f.regs, Frame { sp: Some(-4), bp: None });
    assert_eq!(s.frame.bp, Some(-4));
    let t = vn(&f.tmp, 0x100, 4);
    let load = vec![
        (OpCode::CPUI_INT_ADD, Some(t.clone()), vec![f.regs.bp.clone(), vn(&f.cst, 8, 4)]),
        (OpCode::CPUI_LOAD, Some(vn(&f.reg, 0, 4)), vec![vn(&f.cst, 3, 8), t]),
    ];
    assert_eq!(step(&load, &f.regs, s.frame).arg_end, 8, "[ebp+8] is the first argument");
}

#[test]
fn an_unplaced_stack_pointer_is_untracked() {
    let f = fx();
    let lost = Frame { sp: None, bp: None };
    assert!(step(&load_sp(&f, 4), &f.regs, lost).untracked);
}

#[test]
fn an_argument_address_handed_on_escapes() {
    let f = fx();
    let lea = vec![(OpCode::CPUI_INT_ADD, Some(vn(&f.reg, 0, 4)), vec![f.regs.sp.clone(), vn(&f.cst, 8, 4)])];
    assert!(step(&lea, &f.regs, at_entry()).escaped);
}

#[test]
fn the_third_argument_ends_at_sixteen() {
    let f = fx();
    assert_eq!(step(&load_sp(&f, 0xc), &f.regs, at_entry()).arg_end, 0x10);
}
