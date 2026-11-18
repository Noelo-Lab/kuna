//! Logic-level tests for [`site_producer`] and [`rewrite`] on hand-built
//! dead-list IR, in the shapes x86 `ud2` and AArch64 `brk #0x800` lift to.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace};
use kuna_num::opcodes::OpCode;

use crate::context::ArchContext;

use super::*;

const INVALID_USEROP: u32 = 0x60;
const BRK_USEROP: u32 = 0x54;
const PC: u64 = 0x0;

fn build_fd() -> Funcdata {
    let mut m = AddrSpaceManager::new();
    m.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    m.insert_space(Rc::new(UniqueSpace::new(1, 0, false))).unwrap();
    for (index, name) in [(2, "ram"), (3, "register")] {
        m.insert_space(Rc::new(AddrSpace::new(
            spacetype::IPTR_PROCESSOR,
            name,
            false,
            8,
            1,
            index,
            addrspace_flags::hasphysical,
            1,
            1,
        )))
        .unwrap();
    }
    let glb = Rc::new(ArchContext::new(m));
    let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
    Funcdata::new("func", "func", glb, Address::new(ram, 0x1000), 0x1000_0000, 0x40).unwrap()
}

fn addr(fd: &Funcdata, space: &str, off: u64) -> Address {
    Address::new(Rc::clone(fd.get_arch().manage().get_space_by_name(space).unwrap()), off)
}

/// `pc = CALLOTHER(#userop, operands..)` at 0x1000.
fn callother(fd: &mut Funcdata, userop: u32, operands: &[(u64, int4)]) -> OpId {
    let pc = addr(fd, "ram", 0x1000);
    let op = fd.new_op(1 + operands.len() as int4, pc);
    fd.op_set_opcode_code(op, OpCode::CPUI_CALLOTHER);
    let id = fd.new_constant(4, userop as u64);
    fd.op_set_input(op, id, 0).unwrap();
    for (slot, &(value, size)) in operands.iter().enumerate() {
        let vn = fd.new_constant(size, value);
        fd.op_set_input(op, vn, slot as int4 + 1).unwrap();
    }
    let out = addr(fd, "register", PC);
    fd.new_varnode_out(8, &out, op).unwrap();
    op
}

/// `goto [pc]` at 0x1000, after the ops already there.
fn branchind(fd: &mut Funcdata) -> OpId {
    let pc = addr(fd, "ram", 0x1000);
    let op = fd.new_op(1, pc);
    fd.op_set_opcode_code(op, OpCode::CPUI_BRANCHIND);
    let target = addr(fd, "register", PC);
    let vn = fd.new_varnode(8, &target, None);
    fd.op_set_input(op, vn, 0).unwrap();
    op
}

/// x86: `target = invalidInstructionException(); goto [target];`.
fn ud2(fd: &mut Funcdata) -> (OpId, OpId) {
    let producer = callother(fd, INVALID_USEROP, &[]);
    (producer, branchind(fd))
}

/// AArch64: `pc = SoftwareBreakpoint(imm, inst_next); goto [pc];`.
fn brk(fd: &mut Funcdata, imm: u64) -> (OpId, OpId) {
    let producer = callother(fd, BRK_USEROP, &[(imm, 2), (0x1004, 8)]);
    (producer, branchind(fd))
}

fn inputs(fd: &Funcdata, op: OpId) -> Vec<(u64, int4)> {
    let o = fd.obank().get(op).unwrap();
    (0..o.num_input())
        .map(|i| fd.vbank().get(o.get_in(i).unwrap()).unwrap())
        .map(|v| (v.get_offset(), v.get_size()))
        .collect()
}

#[test]
fn site_lookup_is_by_exact_address() {
    let sites = vec![
        KernelBugSite { addr: 0x400035, warning: true, file: Some(0x4010a0), line: Some(9) },
        KernelBugSite { addr: 0x40004a, warning: false, file: Some(0x4010a0), line: Some(11) },
    ];
    assert_eq!(site_at(&sites, 0x40004a).map(|s| s.warning), Some(false));
    assert_eq!(site_at(&sites, 0x400035).map(|s| s.line), Some(Some(9)));
    assert!(site_at(&sites, 0x400036).is_none());
}

#[test]
fn an_x86_ud2_names_its_producer() {
    let mut fd = build_fd();
    let (producer, branch) = ud2(&mut fd);
    assert_eq!(site_producer(&fd, branch, Some(INVALID_USEROP), None), Some(producer));
    assert_eq!(site_producer(&fd, branch, Some(INVALID_USEROP + 1), None), None);
    assert_eq!(site_producer(&fd, branch, None, Some(BRK_USEROP)), None);
}

#[test]
fn only_the_kernel_brk_immediate_names_its_producer() {
    let mut fd = build_fd();
    let (producer, branch) = brk(&mut fd, LINUX_BUG_BRK_IMM);
    assert_eq!(site_producer(&fd, branch, None, Some(BRK_USEROP)), Some(producer));
    let mut fd = build_fd();
    let (_, branch) = brk(&mut fd, 0x1);
    assert_eq!(site_producer(&fd, branch, Some(INVALID_USEROP), Some(BRK_USEROP)), None);
}

#[test]
fn a_warn_site_becomes_warn_with_file_and_line() {
    let mut fd = build_fd();
    let (producer, branch) = ud2(&mut fd);
    let site = KernelBugSite { addr: 0x1000, warning: true, file: Some(0x2000), line: Some(42) };
    rewrite(&mut fd, branch, producer, &site).unwrap();
    assert_eq!(inputs(&fd, producer), vec![(BUILTIN_KERNEL_WARN as u64, 4), (0x2000, 8), (42, 4)]);
}

#[test]
fn a_bare_table_prints_warn_without_arguments() {
    let mut fd = build_fd();
    let (producer, branch) = ud2(&mut fd);
    let site = KernelBugSite { addr: 0x1000, warning: true, file: None, line: None };
    rewrite(&mut fd, branch, producer, &site).unwrap();
    assert_eq!(inputs(&fd, producer), vec![(BUILTIN_KERNEL_WARN as u64, 4)]);
}

#[test]
fn a_bug_site_becomes_bug_and_a_brk_sheds_its_operands() {
    let mut fd = build_fd();
    let (producer, branch) = brk(&mut fd, LINUX_BUG_BRK_IMM);
    let site = KernelBugSite { addr: 0x1000, warning: false, file: Some(0x2000), line: Some(7) };
    rewrite(&mut fd, branch, producer, &site).unwrap();
    assert_eq!(inputs(&fd, producer), vec![(BUILTIN_KERNEL_BUG as u64, 4)]);
}
