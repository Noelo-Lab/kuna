use std::rc::Rc;

use kuna_base::space::{AddrSpace, ConstantSpace, OtherSpace, UniqueSpace};

use super::*;

type Op = (OpCode, Option<VarnodeData>, Vec<VarnodeData>);

struct Fx {
    cst: Rc<AddrSpace>,
    reg: Rc<AddrSpace>,
    ram: Rc<AddrSpace>,
    regs: Regs,
}

fn vn(space: &Rc<AddrSpace>, offset: u64, size: u32) -> VarnodeData {
    VarnodeData { space: Some(space.clone()), offset, size }
}

fn fx() -> Fx {
    let cst = Rc::new(ConstantSpace::new());
    let reg = Rc::new(OtherSpace::new(1));
    let ram = Rc::new(UniqueSpace::new(2, 0, false));
    let tracked = [0x0, 0x4, 0x8, 0xc, 0x18, 0x1c].iter().map(|&o| vn(&reg, o, 4)).collect();
    let regs = Regs { tracked, sp: vn(&reg, 0x10, 4) };
    Fx { cst, reg, ram, regs }
}

const EAX: u64 = 0x0;
const ECXO: u64 = 0x4;
const EDXO: u64 = 0x8;
const ESI: u64 = 0x18;

fn copy(f: &Fx, to: u64, from: u64) -> Op {
    (OpCode::CPUI_COPY, Some(vn(&f.reg, to, 4)), vec![vn(&f.reg, from, 4)])
}

#[test]
fn mov_esi_ecx_reads_this() {
    let f = fx();
    let s = step(&[copy(&f, ESI, ECXO)], &f.regs, [0; 6]);
    assert_eq!(s.read, ECX);
    assert!(!s.ends);
}

#[test]
fn a_push_saves_and_a_clear_writes_without_reading() {
    let f = fx();
    let push = (OpCode::CPUI_STORE, None, vec![vn(&f.cst, 3, 8), f.regs.sp.clone(), vn(&f.reg, ESI, 4)]);
    assert_eq!(step(&[push], &f.regs, [0; 6]).read, 0);
    let clear = (OpCode::CPUI_INT_XOR, Some(vn(&f.reg, ECXO, 4)), vec![vn(&f.reg, ECXO, 4), vn(&f.reg, ECXO, 4)]);
    let s = step(&[clear], &f.regs, [0; 6]);
    assert_eq!((s.read, s.written[1]), (0, 0xf));
}

#[test]
fn a_push_through_a_temporary_is_a_save() {
    let f = fx();
    let tmp = vn(&f.ram, 0x41500, 4);
    let ops = vec![
        (OpCode::CPUI_COPY, Some(tmp.clone()), vec![vn(&f.reg, ECXO, 4)]),
        (OpCode::CPUI_INT_SUB, Some(f.regs.sp.clone()), vec![f.regs.sp.clone(), vn(&f.cst, 4, 4)]),
        (OpCode::CPUI_STORE, None, vec![vn(&f.cst, 3, 8), f.regs.sp.clone(), tmp]),
    ];
    assert_eq!(step(&ops, &f.regs, [0; 6]).read, 0);
}

#[test]
fn a_temporary_copy_used_otherwise_is_a_read() {
    let f = fx();
    let tmp = vn(&f.ram, 0x100, 4);
    let ops = vec![
        (OpCode::CPUI_COPY, Some(tmp.clone()), vec![vn(&f.reg, ECXO, 4)]),
        (OpCode::CPUI_COPY, Some(vn(&f.reg, ESI, 4)), vec![tmp]),
    ];
    assert_eq!(step(&ops, &f.regs, [0; 6]).read, ECX);
}

#[test]
fn a_register_written_first_is_not_an_input() {
    let f = fx();
    let load = (OpCode::CPUI_COPY, Some(vn(&f.reg, ECXO, 4)), vec![vn(&f.cst, 5, 4)]);
    let s1 = step(&[load], &f.regs, [0; 6]);
    let s2 = step(&[copy(&f, EAX, ECXO)], &f.regs, s1.written);
    assert_eq!(s2.read, 0);
}

#[test]
fn a_byte_written_first_is_not_an_input() {
    let f = fx();
    let ops = vec![
        (OpCode::CPUI_COPY, Some(vn(&f.reg, ECXO, 1)), vec![vn(&f.cst, 4, 1)]),
        (OpCode::CPUI_INT_AND, Some(vn(&f.reg, EAX, 1)), vec![vn(&f.reg, EAX, 1), vn(&f.reg, ECXO, 1)]),
    ];
    assert_eq!(step(&ops, &f.regs, [0; 6]).read & ECX, 0);
}

#[test]
fn a_register_partly_written_first_is_not_an_input() {
    let f = fx();
    let ops = vec![
        (OpCode::CPUI_COPY, Some(vn(&f.reg, ECXO, 2)), vec![vn(&f.cst, 4, 2)]),
        (OpCode::CPUI_COPY, Some(vn(&f.reg, ESI, 4)), vec![vn(&f.reg, ECXO, 4)]),
    ];
    assert_eq!(step(&ops, &f.regs, [0; 6]).read, 0);
}

#[test]
fn a_partial_read_counts() {
    let f = fx();
    let cl = (OpCode::CPUI_COPY, Some(vn(&f.reg, EAX, 1)), vec![vn(&f.reg, EDXO, 1)]);
    assert_eq!(step(&[cl], &f.regs, [0; 6]).read, 1 << 2);
}

#[test]
fn the_line_follows_a_jump_and_ends_at_a_branch_or_call() {
    let f = fx();
    let jmp = step(&[(OpCode::CPUI_BRANCH, None, vec![vn(&f.ram, 0x5fa1c0, 4)])], &f.regs, [0; 6]);
    assert_eq!((jmp.ends, jmp.jump), (true, Some(0x5fa1c0)));
    let call = step(&[(OpCode::CPUI_CALL, None, vec![vn(&f.ram, 0x401000, 4)])], &f.regs, [0; 6]);
    assert_eq!((call.ends, call.jump), (true, None));
}

#[test]
fn a_copy_onto_itself_neither_reads_nor_writes() {
    let f = fx();
    let s = step(&[copy(&f, ECXO, ECXO)], &f.regs, [0; 6]);
    assert_eq!((s.read, s.written[1]), (0, 0));
}

#[test]
fn or_all_ones_and_and_zero_write_without_reading() {
    let f = fx();
    let or = (OpCode::CPUI_INT_OR, Some(vn(&f.reg, ECXO, 4)), vec![vn(&f.reg, ECXO, 4), vn(&f.cst, 0xffff_ffff, 4)]);
    let s = step(&[or], &f.regs, [0; 6]);
    assert_eq!((s.read, s.written[1]), (0, 0xf));
    let and = (OpCode::CPUI_INT_AND, Some(vn(&f.reg, ECXO, 4)), vec![vn(&f.reg, ECXO, 4), vn(&f.cst, 0, 4)]);
    assert_eq!(step(&[and], &f.regs, [0; 6]).read, 0);
    let mask = (OpCode::CPUI_INT_AND, Some(vn(&f.reg, ECXO, 4)), vec![vn(&f.reg, ECXO, 4), vn(&f.cst, 0xff, 4)]);
    assert_eq!(step(&[mask], &f.regs, [0; 6]).read, ECX);
}

#[test]
fn the_inputs_choose_thiscall_fastcall_or_nothing() {
    assert_eq!(model_name(ECX), Some("__thiscall"));
    assert_eq!(model_name(ECX | EDX), Some("__fastcall"));
    assert_eq!(model_name(EDX), None);
    assert_eq!(model_name(ECX | 1), None, "EAX in: a register convention no model states");
    assert_eq!(model_name(ECX | EDX | (1 << 5)), None, "EDI read: a register capture");
}
