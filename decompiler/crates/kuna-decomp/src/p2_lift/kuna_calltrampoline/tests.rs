//! Logic-level tests for `kuna_trampoline_branch_target`, exercising the
//! stack-pointer scan on hand-built raw p-code.

use std::rc::Rc;

use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
    VarnodeStorage,
};
use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;

use super::*;

/// const(0), unique(1), ram(2, IPTR_PROCESSOR), register(3).
fn build_manager() -> AddrSpaceManager {
    let mut m = AddrSpaceManager::new();
    m.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    m.insert_space(Rc::new(UniqueSpace::new(1, 0, false))).unwrap();
    m.insert_space(Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "ram",
        false,
        4,
        1,
        2,
        addrspace_flags::hasphysical,
        1,
        1,
    )))
    .unwrap();
    m.insert_space(Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "register",
        false,
        4,
        1,
        3,
        addrspace_flags::hasphysical,
        0,
        1,
    )))
    .unwrap();
    m
}

struct Spaces {
    cnst: Rc<AddrSpace>,
    uniq: Rc<AddrSpace>,
    ram: Rc<AddrSpace>,
    reg: Rc<AddrSpace>,
}

fn spaces(m: &AddrSpaceManager) -> Spaces {
    Spaces {
        cnst: Rc::clone(m.get_constant_space().unwrap()),
        uniq: Rc::clone(m.get_space_by_name("unique").unwrap()),
        ram: Rc::clone(m.get_space_by_name("ram").unwrap()),
        reg: Rc::clone(m.get_space_by_name("register").unwrap()),
    }
}

fn vn(space: &Rc<AddrSpace>, offset: u64, size: u32) -> VarnodeData {
    VarnodeData { space: Some(Rc::clone(space)), offset, size }
}

/// ESP: register space, offset 0, 4 bytes.
fn esp(s: &Spaces) -> VarnodeData {
    vn(&s.reg, 0, 4)
}

fn sp_storage(s: &Spaces) -> VarnodeStorage {
    VarnodeStorage { space: Some(Rc::clone(&s.reg)), offset: 0, size: 4 }
}

fn op(code: OpCode, out: Option<VarnodeData>, ins: Vec<VarnodeData>) -> RawOp {
    RawOp { code, out, ins }
}

/// `lea esp,[esp+4]` — through a temporary, the way SLEIGH lowers an address
/// computation — then `jmp 0x40d1b4`.
fn lea_form(s: &Spaces) -> Vec<RawOp> {
    let tmp = vn(&s.uniq, 0x100, 4);
    vec![
        op(OpCode::CPUI_INT_ADD, Some(tmp.clone()), vec![esp(s), vn(&s.cnst, 4, 4)]),
        op(OpCode::CPUI_COPY, Some(esp(s)), vec![tmp]),
        op(OpCode::CPUI_BRANCH, None, vec![vn(&s.ram, 0x40d1b4, 1)]),
    ]
}

#[test]
fn lea_discard_then_jump_matches() {
    let m = build_manager();
    let s = spaces(&m);
    let got = kuna_trampoline_branch_target(&lea_form(&s), &sp_storage(&s));
    assert_eq!(got, Some(vn(&s.ram, 0x40d1b4, 1)));
}

#[test]
fn add_esp_4_then_jump_matches() {
    let m = build_manager();
    let s = spaces(&m);
    let ops = vec![
        op(OpCode::CPUI_INT_ADD, Some(esp(&s)), vec![esp(&s), vn(&s.cnst, 4, 4)]),
        op(OpCode::CPUI_BRANCH, None, vec![vn(&s.ram, 0x2000, 1)]),
    ];
    assert_eq!(
        kuna_trampoline_branch_target(&ops, &sp_storage(&s)),
        Some(vn(&s.ram, 0x2000, 1))
    );
}

/// The witness's first fragment: `lea esp,[esp+4]; sub esp,0xcfc; jmp`.  The NET
/// delta is a large allocation — what identifies the shape is that the run
/// *passed through* `entrySP + 4`.
#[test]
fn allocation_after_the_discard_still_matches() {
    let m = build_manager();
    let s = spaces(&m);
    let mut ops = lea_form(&s);
    ops.insert(
        2,
        op(OpCode::CPUI_INT_SUB, Some(esp(&s)), vec![esp(&s), vn(&s.cnst, 0xcfc, 4)]),
    );
    assert!(kuna_trampoline_branch_target(&ops, &sp_storage(&s)).is_some());
}

/// A `pop` reads the word before dropping it; the value read is not tracked, but
/// the stack pointer still passes through `entrySP + 4`.
#[test]
fn pop_then_jump_matches() {
    let m = build_manager();
    let s = spaces(&m);
    let ecx = vn(&s.reg, 8, 4);
    let ops = vec![
        op(OpCode::CPUI_LOAD, Some(ecx), vec![vn(&s.cnst, 2, 4), esp(&s)]),
        op(OpCode::CPUI_INT_ADD, Some(esp(&s)), vec![esp(&s), vn(&s.cnst, 4, 4)]),
        op(OpCode::CPUI_BRANCH, None, vec![vn(&s.ram, 0x2000, 1)]),
    ];
    assert!(kuna_trampoline_branch_target(&ops, &sp_storage(&s)).is_some());
}

#[test]
fn plain_jump_without_a_discard_declines() {
    let m = build_manager();
    let s = spaces(&m);
    let ops = vec![op(OpCode::CPUI_BRANCH, None, vec![vn(&s.ram, 0x2000, 1)])];
    assert_eq!(kuna_trampoline_branch_target(&ops, &sp_storage(&s)), None);
}

/// `add esp,8` never equals one pointer word: two words consumed is not the
/// return-address discard this rule models.
#[test]
fn wrong_sized_discard_declines() {
    let m = build_manager();
    let s = spaces(&m);
    let ops = vec![
        op(OpCode::CPUI_INT_ADD, Some(esp(&s)), vec![esp(&s), vn(&s.cnst, 8, 4)]),
        op(OpCode::CPUI_BRANCH, None, vec![vn(&s.ram, 0x2000, 1)]),
    ];
    assert_eq!(kuna_trampoline_branch_target(&ops, &sp_storage(&s)), None);
}

#[test]
fn a_return_declines() {
    let m = build_manager();
    let s = spaces(&m);
    let ops = vec![
        op(OpCode::CPUI_INT_ADD, Some(esp(&s)), vec![esp(&s), vn(&s.cnst, 4, 4)]),
        op(OpCode::CPUI_RETURN, None, vec![vn(&s.cnst, 0, 4)]),
    ];
    assert_eq!(kuna_trampoline_branch_target(&ops, &sp_storage(&s)), None);
}

#[test]
fn a_conditional_branch_declines() {
    let m = build_manager();
    let s = spaces(&m);
    let ops = vec![
        op(OpCode::CPUI_INT_ADD, Some(esp(&s)), vec![esp(&s), vn(&s.cnst, 4, 4)]),
        op(OpCode::CPUI_CBRANCH, None, vec![vn(&s.ram, 0x2000, 1), vn(&s.reg, 0x20, 1)]),
    ];
    assert_eq!(kuna_trampoline_branch_target(&ops, &sp_storage(&s)), None);
}

#[test]
fn an_indirect_branch_declines() {
    let m = build_manager();
    let s = spaces(&m);
    let ops = vec![
        op(OpCode::CPUI_INT_ADD, Some(esp(&s)), vec![esp(&s), vn(&s.cnst, 4, 4)]),
        op(OpCode::CPUI_BRANCHIND, None, vec![vn(&s.reg, 0, 4)]),
    ];
    assert_eq!(kuna_trampoline_branch_target(&ops, &sp_storage(&s)), None);
}

/// A relative branch inside one instruction's own p-code (a `constant`
/// destination) is not a machine-level transfer.
#[test]
fn a_relative_branch_declines() {
    let m = build_manager();
    let s = spaces(&m);
    let ops = vec![
        op(OpCode::CPUI_INT_ADD, Some(esp(&s)), vec![esp(&s), vn(&s.cnst, 4, 4)]),
        op(OpCode::CPUI_BRANCH, None, vec![vn(&s.cnst, 2, 4)]),
    ];
    assert_eq!(kuna_trampoline_branch_target(&ops, &sp_storage(&s)), None);
}

/// A `leave`-style `esp = ebp` restore leaves the stack pointer unaccountable.
#[test]
fn an_opaque_stack_pointer_write_declines() {
    let m = build_manager();
    let s = spaces(&m);
    let ebp = vn(&s.reg, 0x14, 4);
    let ops = vec![
        op(OpCode::CPUI_COPY, Some(esp(&s)), vec![ebp]),
        op(OpCode::CPUI_INT_ADD, Some(esp(&s)), vec![esp(&s), vn(&s.cnst, 4, 4)]),
        op(OpCode::CPUI_BRANCH, None, vec![vn(&s.ram, 0x2000, 1)]),
    ];
    assert_eq!(kuna_trampoline_branch_target(&ops, &sp_storage(&s)), None);
}

/// A partial write to the low half of the stack pointer invalidates the tracked
/// value even though the locations are not equal.
#[test]
fn a_partial_stack_pointer_write_declines() {
    let m = build_manager();
    let s = spaces(&m);
    let ops = vec![
        op(OpCode::CPUI_INT_ADD, Some(esp(&s)), vec![esp(&s), vn(&s.cnst, 4, 4)]),
        op(OpCode::CPUI_COPY, Some(vn(&s.reg, 0, 2)), vec![vn(&s.cnst, 0, 2)]),
        op(OpCode::CPUI_BRANCH, None, vec![vn(&s.ram, 0x2000, 1)]),
    ];
    assert_eq!(kuna_trampoline_branch_target(&ops, &sp_storage(&s)), None);
}

/// A `call` inside the fragment means the run is a function body, not a
/// trampoline.
#[test]
fn an_inner_call_declines() {
    let m = build_manager();
    let s = spaces(&m);
    let ops = vec![
        op(OpCode::CPUI_INT_ADD, Some(esp(&s)), vec![esp(&s), vn(&s.cnst, 4, 4)]),
        op(OpCode::CPUI_CALL, None, vec![vn(&s.ram, 0x3000, 1)]),
    ];
    assert_eq!(kuna_trampoline_branch_target(&ops, &sp_storage(&s)), None);
}

/// A run that discards but never branches (the decode budget ran out) declines.
#[test]
fn a_run_with_no_branch_declines() {
    let m = build_manager();
    let s = spaces(&m);
    let ops = vec![op(
        OpCode::CPUI_INT_ADD,
        Some(esp(&s)),
        vec![esp(&s), vn(&s.cnst, 4, 4)],
    )];
    assert_eq!(kuna_trampoline_branch_target(&ops, &sp_storage(&s)), None);
}

#[test]
fn an_empty_run_declines() {
    let m = build_manager();
    let s = spaces(&m);
    assert_eq!(kuna_trampoline_branch_target(&[], &sp_storage(&s)), None);
}

/// `sub esp,-4` is `add esp,4` written the other way; the literal is
/// sign-extended from its own width before it is applied.
#[test]
fn a_negative_literal_is_sign_extended() {
    let m = build_manager();
    let s = spaces(&m);
    let ops = vec![
        op(
            OpCode::CPUI_INT_SUB,
            Some(esp(&s)),
            vec![esp(&s), vn(&s.cnst, 0xffff_fffc, 4)],
        ),
        op(OpCode::CPUI_BRANCH, None, vec![vn(&s.ram, 0x2000, 1)]),
    ];
    assert!(kuna_trampoline_branch_target(&ops, &sp_storage(&s)).is_some());
}
