//! Logic-level tests for `kuna_call_pops_return_address`, exercising the
//! stack-slot scan on hand-built raw p-code.

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

/// EAX: register space, offset 0x10, 4 bytes.
fn eax(s: &Spaces) -> VarnodeData {
    vn(&s.reg, 0x10, 4)
}

fn sp_storage(s: &Spaces) -> VarnodeStorage {
    VarnodeStorage { space: Some(Rc::clone(&s.reg)), offset: 0, size: 4 }
}

fn op(code: OpCode, out: Option<VarnodeData>, ins: Vec<VarnodeData>) -> RawOp {
    RawOp { code, out, ins }
}

/// `dest = *:4 ESP` — the LOAD half of a `pop`/`ret`.
fn load_from_sp(s: &Spaces, dest: VarnodeData) -> RawOp {
    op(
        OpCode::CPUI_LOAD,
        Some(dest),
        vec![vn(&s.cnst, s.ram.get_index() as u64, 4), esp(s)],
    )
}

/// `ESP = ESP + k`.
fn add_sp(s: &Spaces, k: u64) -> RawOp {
    op(OpCode::CPUI_INT_ADD, Some(esp(s)), vec![esp(s), vn(&s.cnst, k, 4)])
}

/// The `ret` tail: `tmp = *:4 ESP; ESP = ESP + 4; return [tmp]`.
fn ret_tail(s: &Spaces) -> Vec<RawOp> {
    let tmp = vn(&s.uniq, 0x100, 4);
    vec![
        load_from_sp(s, tmp.clone()),
        add_sp(s, 4),
        op(OpCode::CPUI_RETURN, None, vec![tmp]),
    ]
}

/// `pop eax; inc eax; ret` — the round-12 witness at 0x4f708d.
#[test]
fn pop_adjust_ret_discards_the_return_address() {
    let m = build_manager();
    let s = spaces(&m);
    let mut ops = vec![load_from_sp(&s, eax(&s)), add_sp(&s, 4)];
    // `inc eax` plus a flag write the scan cannot account for.
    ops.push(op(OpCode::CPUI_INT_ADD, Some(eax(&s)), vec![eax(&s), vn(&s.cnst, 1, 4)]));
    ops.push(op(OpCode::CPUI_INT_SLESS, Some(vn(&s.reg, 0x40, 1)), vec![eax(&s), vn(&s.cnst, 0, 4)]));
    ops.extend(ret_tail(&s));
    assert!(kuna_call_pops_return_address(&ops, &sp_storage(&s)));
}

/// `add esp,4; ret` — the same transfer with no register captured.
#[test]
fn bare_stack_adjust_then_ret_matches() {
    let m = build_manager();
    let s = spaces(&m);
    let mut ops = vec![add_sp(&s, 4)];
    ops.extend(ret_tail(&s));
    assert!(kuna_call_pops_return_address(&ops, &sp_storage(&s)));
}

/// `mov ebx,[esp]; ret` — the `__x86.get_pc_thunk` family reads the return
/// address without consuming it, so the caller DOES get control back.
#[test]
fn get_pc_thunk_returns_to_the_call_site() {
    let m = build_manager();
    let s = spaces(&m);
    let mut ops = vec![load_from_sp(&s, vn(&s.reg, 0x20, 4))];
    ops.extend(ret_tail(&s));
    assert!(!kuna_call_pops_return_address(&ops, &sp_storage(&s)));
}

/// A bare `ret` loads and returns through the call site's own return address.
#[test]
fn bare_ret_returns_to_the_call_site() {
    let m = build_manager();
    let s = spaces(&m);
    assert!(!kuna_call_pops_return_address(&ret_tail(&s), &sp_storage(&s)));
}

/// `ret 8` still returns through the call site's own return address; its larger
/// stack adjustment happens only after loading that destination.
#[test]
fn ret_immediate_returns_to_the_call_site() {
    let m = build_manager();
    let s = spaces(&m);
    let tmp = vn(&s.uniq, 0x100, 4);
    let ops = vec![
        load_from_sp(&s, tmp.clone()),
        add_sp(&s, 12),
        op(OpCode::CPUI_RETURN, None, vec![tmp]),
    ];
    assert!(!kuna_call_pops_return_address(&ops, &sp_storage(&s)));
}

/// A balanced stack frame returns through slot zero and never discards the
/// call site's return address before loading it.
#[test]
fn balanced_frame_returns_to_the_call_site() {
    let m = build_manager();
    let s = spaces(&m);
    let mut ops = vec![
        op(
            OpCode::CPUI_INT_SUB,
            Some(esp(&s)),
            vec![esp(&s), vn(&s.cnst, 0x10, 4)],
        ),
        add_sp(&s, 0x10),
    ];
    ops.extend(ret_tail(&s));
    assert!(!kuna_call_pops_return_address(&ops, &sp_storage(&s)));
}

/// A callee that pops twice returns through a word the caller's frame owns;
/// the rule declines rather than model that.
#[test]
fn two_pops_return_past_the_callers_frame() {
    let m = build_manager();
    let s = spaces(&m);
    let mut ops =
        vec![load_from_sp(&s, eax(&s)), add_sp(&s, 4), load_from_sp(&s, eax(&s)), add_sp(&s, 4)];
    ops.extend(ret_tail(&s));
    assert!(!kuna_call_pops_return_address(&ops, &sp_storage(&s)));
}

/// An ordinary prologue lowers the stack pointer and never reaches `entrySP + 4`.
#[test]
fn ordinary_prologue_never_discards() {
    let m = build_manager();
    let s = spaces(&m);
    let mut ops = vec![op(
        OpCode::CPUI_INT_SUB,
        Some(esp(&s)),
        vec![esp(&s), vn(&s.cnst, 0x10, 4)],
    )];
    ops.extend(ret_tail(&s));
    assert!(!kuna_call_pops_return_address(&ops, &sp_storage(&s)));
}

/// The `jmp` flavour belongs to `calltrampoline`, not here.
#[test]
fn trampoline_flavour_is_not_claimed() {
    let m = build_manager();
    let s = spaces(&m);
    let ops = vec![
        add_sp(&s, 4),
        op(OpCode::CPUI_BRANCH, None, vec![vn(&s.ram, 0x40d1b4, 4)]),
    ];
    assert!(!kuna_call_pops_return_address(&ops, &sp_storage(&s)));
}

#[test]
fn conditional_branch_ends_the_scan() {
    let m = build_manager();
    let s = spaces(&m);
    let ops = vec![
        add_sp(&s, 4),
        op(OpCode::CPUI_CBRANCH, None, vec![vn(&s.ram, 0x40d1b4, 4)]),
    ];
    assert!(!kuna_call_pops_return_address(&ops, &sp_storage(&s)));
}

#[test]
fn indirect_branch_ends_the_scan() {
    let m = build_manager();
    let s = spaces(&m);
    let ops = vec![add_sp(&s, 4), op(OpCode::CPUI_BRANCHIND, None, vec![eax(&s)])];
    assert!(!kuna_call_pops_return_address(&ops, &sp_storage(&s)));
}

/// Constant-space branches are internal to one instruction's p-code, but the
/// bounded linear scan does not model their alternate path and must decline.
#[test]
fn internal_pcode_branch_ends_the_scan() {
    let m = build_manager();
    let s = spaces(&m);
    let ops = vec![
        add_sp(&s, 4),
        op(OpCode::CPUI_BRANCH, None, vec![vn(&s.cnst, 2, 4)]),
    ];
    assert!(!kuna_call_pops_return_address(&ops, &sp_storage(&s)));
}

/// A store can rewrite the slot the `ret` is about to read; the scan does not
/// model memory, so it declines.
#[test]
fn a_store_ends_the_scan() {
    let m = build_manager();
    let s = spaces(&m);
    let mut ops = vec![
        add_sp(&s, 4),
        op(
            OpCode::CPUI_STORE,
            None,
            vec![vn(&s.cnst, s.ram.get_index() as u64, 4), esp(&s), eax(&s)],
        ),
    ];
    ops.extend(ret_tail(&s));
    assert!(!kuna_call_pops_return_address(&ops, &sp_storage(&s)));
}

/// A call in the middle of the fragment ends the scan.
#[test]
fn an_inner_call_ends_the_scan() {
    let m = build_manager();
    let s = spaces(&m);
    let mut ops =
        vec![add_sp(&s, 4), op(OpCode::CPUI_CALL, None, vec![vn(&s.ram, 0x401000, 4)])];
    ops.extend(ret_tail(&s));
    assert!(!kuna_call_pops_return_address(&ops, &sp_storage(&s)));
}

/// An unaccountable write to the stack pointer ends the scan even though the
/// displacement passed through `entrySP + 4` first.
#[test]
fn an_opaque_stack_pointer_write_ends_the_scan() {
    let m = build_manager();
    let s = spaces(&m);
    let mut ops = vec![
        add_sp(&s, 4),
        op(OpCode::CPUI_INT_AND, Some(esp(&s)), vec![esp(&s), vn(&s.cnst, 0xfffffff0, 4)]),
    ];
    ops.extend(ret_tail(&s));
    assert!(!kuna_call_pops_return_address(&ops, &sp_storage(&s)));
}

/// An empty decode is not the idiom.
#[test]
fn an_empty_run_declines() {
    let m = build_manager();
    let s = spaces(&m);
    assert!(!kuna_call_pops_return_address(&[], &sp_storage(&s)));
}

/// A link-register architecture: the `call` pushes nothing and the return's
/// destination is a register, so the rule declines twice over.
#[test]
fn link_register_return_declines() {
    let m = build_manager();
    let s = spaces(&m);
    let lr = vn(&s.reg, 0x30, 4);
    let ops = vec![op(OpCode::CPUI_RETURN, None, vec![lr])];
    assert!(!kuna_call_pops_return_address(&ops, &sp_storage(&s)));
}
