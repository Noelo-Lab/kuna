// Positive unit tests for the stack-pointer flow analysis
// (`coreaction_stackptr`): the clog fold (LOAD -> COPY), `is_stack_relative`,
// and the `StackSolver` linear solve.  These pin the in-scope, realizable
// behavior of the third stack-var-promotion chain link directly on a real
// `Funcdata` carrying a stack spacebase, rather than relying on the datatest
// corpus (which also needs the out-of-scope proto-recovery / structuring links).

use super::*;

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, SpacebaseSpace,
    UniqueSpace, VarnodeStorage,
};

use kuna_base::types::int4;

use kuna_num::opcodes::OpCode;

use crate::seams::{Architecture, BlockId, TypeOp};

// SP register location (in the "register" processor space) and size.
const SP_OFF: u64 = 0x20;
const SP_SIZE: int4 = 8;

// ---------------------------------------------------------------------------
// Fixtures: a manager with const/unique/register/stack-spacebase spaces, and a
// Funcdata whose stack space's base register is the SP register.
// ---------------------------------------------------------------------------

fn build_manager() -> AddrSpaceManager {
    let mut m = AddrSpaceManager::new();
    m.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    m.insert_space(Rc::new(UniqueSpace::new(1, 0, false))).unwrap();
    let regspc = Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "register",
        false,
        8,
        1,
        2,
        addrspace_flags::hasphysical,
        1,
        1,
    ));
    m.insert_space(Rc::clone(&regspc)).unwrap();
    // The "stack" spacebase space (index 3) contained in "register" — becomes
    // get_stack_space().
    m.insert_space(Rc::new(SpacebaseSpace::new("stack", 3, 8, &regspc, 1, true, false)))
        .unwrap();
    m
}

fn register_space(m: &AddrSpaceManager) -> Rc<AddrSpace> {
    Rc::clone(m.get_space_by_name("register").unwrap())
}

fn build_fd() -> Funcdata {
    let manage = build_manager();
    let regspc = register_space(&manage);
    let stackspc = Rc::clone(manage.get_stack_space().unwrap());
    let sp_data = VarnodeStorage { space: Some(regspc), offset: SP_OFF, size: SP_SIZE as u32 };
    manage.add_spacebase_pointer(&stackspc, &sp_data, SP_SIZE, true).unwrap();

    let glb = Rc::new(Architecture::new(manage));
    let code = Rc::clone(glb.manage().get_space_by_name("register").unwrap());
    let entry = Address::new(code, 0x1000);
    Funcdata::new("func", "func", glb, entry, 0x1000_0000, 0x40).unwrap()
}

fn stack_space(fd: &Funcdata) -> Rc<AddrSpace> {
    Rc::clone(fd.get_arch().manage().get_stack_space().unwrap())
}

fn reg_addr(fd: &Funcdata, off: u64) -> Address {
    Address::new(register_space(fd.get_arch().manage()), off)
}

fn ram_addr(fd: &Funcdata, off: u64) -> Address {
    // Use the unique space for non-spacebase intermediate outputs.
    Address::new(
        Rc::clone(fd.get_arch().manage().get_space_by_name("unique").unwrap()),
        off,
    )
}

fn new_block(fd: &mut Funcdata) -> BlockId {
    let root = fd.bblocks_root_pub();
    fd.bblocks_mut().new_block_basic(root)
}

fn mk_op(fd: &mut Funcdata, bl: BlockId, off: u64, ninput: int4, opc: OpCode) -> OpId {
    let op = fd.new_op(ninput, reg_addr(fd, off));
    fd.op_set_opcode(op, TypeOp::new(opc, 0, format!("{opc:?}")));
    fd.op_insert_end(op, bl);
    op
}

/// The spacebase input Varnode at the SP register location.
fn make_sp_in(fd: &mut Funcdata) -> VarnodeId {
    let vn = fd.new_varnode(SP_SIZE, &reg_addr(fd, SP_OFF), None);
    fd.set_input_varnode(vn).unwrap()
}

// ---------------------------------------------------------------------------
// is_stack_relative
// ---------------------------------------------------------------------------

#[test]
fn is_stack_relative_recognizes_base_and_offset() {
    let mut fd = build_fd();
    let bl = new_block(&mut fd);
    let sp_in = make_sp_in(&mut fd);

    // The base pointer itself is stack relative with offset 0.
    assert_eq!(super::is_stack_relative(&fd, sp_in, sp_in), Some(0));

    // ptr = INT_ADD(sp_in, -0x10)  -> stack relative with that constant.
    let addop = mk_op(&mut fd, bl, 0x10, 2, OpCode::CPUI_INT_ADD);
    let c = fd.new_constant(SP_SIZE, 0xffff_ffff_ffff_fff0);
    fd.op_set_input(addop, sp_in, 0).unwrap();
    fd.op_set_input(addop, c, 1).unwrap();
    let ptr = fd.new_varnode(SP_SIZE, &reg_addr(&fd, 0x40), None);
    fd.op_set_output(addop, ptr).unwrap();

    assert_eq!(
        super::is_stack_relative(&fd, sp_in, ptr),
        Some(0xffff_ffff_ffff_fff0)
    );
}

#[test]
fn is_stack_relative_rejects_non_relative() {
    let mut fd = build_fd();
    let bl = new_block(&mut fd);
    let sp_in = make_sp_in(&mut fd);

    // other = INT_ADD(notSp, const) — base is not the SP input.
    let other = fd.new_varnode(SP_SIZE, &reg_addr(&fd, 0x80), None);
    let other_in = fd.set_input_varnode(other).unwrap();
    let addop = mk_op(&mut fd, bl, 0x14, 2, OpCode::CPUI_INT_ADD);
    let c = fd.new_constant(SP_SIZE, 0x10);
    fd.op_set_input(addop, other_in, 0).unwrap();
    fd.op_set_input(addop, c, 1).unwrap();
    let outv = fd.new_varnode(SP_SIZE, &reg_addr(&fd, 0x48), None);
    fd.op_set_output(addop, outv).unwrap();

    assert_eq!(super::is_stack_relative(&fd, sp_in, outv), None);

    // A non-constant addend is also rejected (the addend must be constant).
    let addop2 = mk_op(&mut fd, bl, 0x18, 2, OpCode::CPUI_INT_ADD);
    fd.op_set_input(addop2, sp_in, 0).unwrap();
    fd.op_set_input(addop2, other_in, 1).unwrap();
    let outv2 = fd.new_varnode(SP_SIZE, &reg_addr(&fd, 0x50), None);
    fd.op_set_output(addop2, outv2).unwrap();
    assert_eq!(super::is_stack_relative(&fd, sp_in, outv2), None);
}

// ---------------------------------------------------------------------------
// StackSolver
// ---------------------------------------------------------------------------

#[test]
fn stacksolver_solves_add_chain() {
    // sp_in (var0) ; v1 = INT_ADD(sp_in, 8) (var1) ; v2 = INT_ADD(v1, 4)
    // but v2 is at the SP location too, so it is var2 with equation v2-v1=4.
    let mut fd = build_fd();
    let bl = new_block(&mut fd);
    let sp_in = make_sp_in(&mut fd);

    // v1 at SP location: a stack-pointer reference (e.g. an aligned push).
    let add1 = mk_op(&mut fd, bl, 0x10, 2, OpCode::CPUI_INT_ADD);
    let c8 = fd.new_constant(SP_SIZE, 8);
    fd.op_set_input(add1, sp_in, 0).unwrap();
    fd.op_set_input(add1, c8, 1).unwrap();
    let v1 = fd.new_varnode(SP_SIZE, &reg_addr(&fd, SP_OFF), None);
    fd.op_set_output(add1, v1).unwrap();

    let stackspc = stack_space(&fd);
    let mut solver = super::StackSolver::new();
    assert!(solver.build(&fd, &stackspc, 0).is_ok());
    // Two spacebase references: the input (var0) and v1 (var1).
    assert_eq!(solver.vnlist.len(), 2);
    solver.solve();
    // Input solves to 0; v1 = sp_in + 8 -> solution 8.
    assert_eq!(solver.soln[0], 0);
    assert_eq!(solver.soln[1], 8);
}

#[test]
fn stacksolver_propagate_uses_guess_for_indirect() {
    // An INDIRECT-defined spacebase ref with no call spec -> the rhs=4 guess.
    let mut fd = build_fd();
    let bl = new_block(&mut fd);
    let sp_in = make_sp_in(&mut fd);

    // ind = INDIRECT(sp_in, iop)  with output at the SP location.
    let indop = mk_op(&mut fd, bl, 0x10, 2, OpCode::CPUI_INDIRECT);
    fd.op_set_input(indop, sp_in, 0).unwrap();
    // second input is the iop marker; a const placeholder is sufficient here
    // (the call-spec lookup is the seam — no fc found -> guess path).
    let iop = fd.new_constant(SP_SIZE, 0);
    fd.op_set_input(indop, iop, 1).unwrap();
    let ind = fd.new_varnode(SP_SIZE, &reg_addr(&fd, SP_OFF), None);
    fd.op_set_output(indop, ind).unwrap();

    let stackspc = stack_space(&fd);
    let mut solver = super::StackSolver::new();
    assert!(solver.build(&fd, &stackspc, 0).is_ok());
    assert_eq!(solver.vnlist.len(), 2);
    // No known equation for var1, only a guess (rhs=4) -> solve propagates it.
    solver.solve();
    assert_eq!(solver.soln[0], 0);
    assert_eq!(solver.soln[1], 4);
}

// ---------------------------------------------------------------------------
// check_clog: the LOAD -> COPY fold
// ---------------------------------------------------------------------------

#[test]
fn check_clog_folds_load_to_copy() {
    // Build the clog shape in one block, in order:
    //   STORE(stack, sp_in+0, K)          ; constant store at the stack pointer
    //   ld   = LOAD(stack, sp_in+0)       ; load from the same stack slot
    //   newsp= INT_ADD(sp_in, ld)         ; RSP += *(RSP)  -- the clog
    // check_clog should rewrite `ld` to COPY(K).
    let mut fd = build_fd();
    let bl = new_block(&mut fd);
    let sp_in = make_sp_in(&mut fd);
    let stackspc = stack_space(&fd);
    // The LOAD/STORE space-id operand (slot 0) is never read by check_clog /
    // repair / adjust_load (they only use the pointer (slot 1) and stored value
    // (slot 2)); a placeholder const suffices.
    let stack_idx = stackspc.get_index() as u64;

    // STORE(spaceid, sp_in, K)
    let storeop = mk_op(&mut fd, bl, 0x10, 3, OpCode::CPUI_STORE);
    let sid_store = fd.new_constant(8, stack_idx);
    let kconst = fd.new_constant(SP_SIZE, 0x1234);
    fd.op_set_input(storeop, sid_store, 0).unwrap();
    fd.op_set_input(storeop, sp_in, 1).unwrap();
    fd.op_set_input(storeop, kconst, 2).unwrap();

    // ld = LOAD(spaceid, sp_in)
    let loadop = mk_op(&mut fd, bl, 0x14, 2, OpCode::CPUI_LOAD);
    let sid_load = fd.new_constant(8, stack_idx);
    fd.op_set_input(loadop, sid_load, 0).unwrap();
    fd.op_set_input(loadop, sp_in, 1).unwrap();
    let ld = fd.new_varnode(SP_SIZE, &ram_addr(&fd, 0x100), None);
    fd.op_set_output(loadop, ld).unwrap();

    // newsp = INT_ADD(sp_in, ld)  output at the SP location.
    let addop = mk_op(&mut fd, bl, 0x18, 2, OpCode::CPUI_INT_ADD);
    fd.op_set_input(addop, sp_in, 0).unwrap();
    fd.op_set_input(addop, ld, 1).unwrap();
    let newsp = fd.new_varnode(SP_SIZE, &reg_addr(&fd, SP_OFF), None);
    fd.op_set_output(addop, newsp).unwrap();

    let repaired = super::check_clog(&mut fd, &stackspc, 0);
    assert_eq!(repaired, 1, "exactly one clog should be repaired");

    // The LOAD is now a COPY of the stored constant value (0x1234).
    let code = fd.obank().get(loadop).map(|o| o.code());
    assert_eq!(code, Some(OpCode::CPUI_COPY), "LOAD folded to COPY");
    let in0 = fd.obank().get(loadop).and_then(|o| o.get_in(0)).unwrap();
    let v = fd.vbank().get(in0).unwrap();
    assert!(v.is_constant());
    assert_eq!(v.get_offset(), 0x1234);
    // The COPY has a single input (the size/space operand was removed).
    assert_eq!(fd.obank().get(loadop).map(|o| o.num_input()), Some(1));
}

#[test]
fn check_clog_no_store_is_noop() {
    // A LOAD-fed clog with no matching STORE before it: nothing to fold.
    let mut fd = build_fd();
    let bl = new_block(&mut fd);
    let sp_in = make_sp_in(&mut fd);
    let stackspc = stack_space(&fd);
    let stack_idx = stackspc.get_index() as u64;

    let loadop = mk_op(&mut fd, bl, 0x14, 2, OpCode::CPUI_LOAD);
    let sid_load = fd.new_constant(8, stack_idx);
    fd.op_set_input(loadop, sid_load, 0).unwrap();
    fd.op_set_input(loadop, sp_in, 1).unwrap();
    let ld = fd.new_varnode(SP_SIZE, &ram_addr(&fd, 0x100), None);
    fd.op_set_output(loadop, ld).unwrap();

    let addop = mk_op(&mut fd, bl, 0x18, 2, OpCode::CPUI_INT_ADD);
    fd.op_set_input(addop, sp_in, 0).unwrap();
    fd.op_set_input(addop, ld, 1).unwrap();
    let newsp = fd.new_varnode(SP_SIZE, &reg_addr(&fd, SP_OFF), None);
    fd.op_set_output(addop, newsp).unwrap();

    let repaired = super::check_clog(&mut fd, &stackspc, 0);
    assert_eq!(repaired, 0);
    // The LOAD is unchanged.
    assert_eq!(fd.obank().get(loadop).map(|o| o.code()), Some(OpCode::CPUI_LOAD));
}
