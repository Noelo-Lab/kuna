//! Unit tests for `kuna_boolbranch.rs`.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace};
use kuna_num::opcodes::OpCode;

use super::{arms_only_assign, branches_on_nan, is_branch_instruction, only_tested, parse_bool_branch};
use crate::context::{ArchContext, BlockId, OpId, TypeOp, VarnodeId};
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;
use crate::varnode::DefOpInfo;

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
    m.set_default_code_space(2).unwrap();
    let glb = Rc::new(ArchContext::new(m));
    let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
    Funcdata::new("f", "f", glb, Address::new(ram, 0x1000), 0x10000000, 0x40).unwrap()
}

fn ram(fd: &Funcdata, off: u64) -> Address {
    Address::new(Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap()), off)
}

fn op_at(fd: &mut Funcdata, bl: BlockId, off: u64, opc: OpCode) -> OpId {
    let op = fd.new_op(1, ram(fd, off));
    fd.obank_mut().change_opcode(op, TypeOp::new(opc, 0, format!("{opc:?}")));
    fd.op_insert_end(op, bl);
    op
}

fn dt(size: i32) -> Rc<Datatype> {
    Rc::new(Datatype::new(size, type_metatype::TYPE_UNKNOWN))
}

/// The output of `op`, a register-like varnode at `off`.
fn out_of(fd: &mut Funcdata, op: OpId, off: u64) -> VarnodeId {
    let def = DefOpInfo { id: op, seqnum: fd.obank().get(op).unwrap().get_seq_num().clone() };
    let addr = ram(fd, off);
    let mut no_collision =
        |_: &mut crate::varnode::VarnodeBank, _: VarnodeId, _: VarnodeId| -> kuna_base::error::KunaResult<()> {
            panic!("out_of: collision")
        };
    let vn = fd.vbank_mut().create_def(4, addr, dt(4), def, &mut no_collision).unwrap();
    fd.obank_mut().get_mut(op).unwrap().set_output(Some(vn));
    vn
}

fn read(fd: &mut Funcdata, reader: OpId, vn: VarnodeId) {
    fd.obank_mut().get_mut(reader).unwrap().set_input(Some(vn), 0);
    fd.vbank_mut().add_descend(vn, reader).unwrap();
}

fn constant(fd: &mut Funcdata, val: u64) -> VarnodeId {
    let c = Rc::clone(fd.get_arch().manage().get_space(0).unwrap());
    fd.vbank_mut().create(4, Address::new(c, val), dt(4))
}

/// A basic block covering the instructions at `start..=stop`.
fn block(fd: &mut Funcdata, start: u64, stop: u64) -> BlockId {
    let root = fd.bblocks_root_pub();
    let bl = fd.bblocks_mut().new_block_basic(root);
    let (beg, end) = (ram(fd, start), ram(fd, stop));
    fd.set_basic_block_range(bl, &beg, &end);
    bl
}

/// One arm: the address its block starts at and the address of the COPY it
/// holds (`None` leaves it empty).
type Arm = (u64, Option<u64>);

/// A root block ending in a CBRANCH at `cbr`, plus two arm blocks.
fn diamond(cbr: u64, arms: [Arm; 2]) -> (Funcdata, OpId, BlockId, [BlockId; 2]) {
    let mut fd = build_fd();
    let root = block(&mut fd, 0x1000, cbr);
    let cbranch = op_at(&mut fd, root, cbr, OpCode::CPUI_CBRANCH);
    let blocks = arms.map(|(start, copy)| {
        let bl = block(&mut fd, start, start);
        if let Some(off) = copy {
            op_at(&mut fd, bl, off, OpCode::CPUI_COPY);
        }
        bl
    });
    (fd, cbranch, root, blocks)
}

/// x86 `jle L; mov eax,1; jmp E; L: mov eax,0`: both arms are other instructions.
#[test]
fn jump_diamond_is_a_branch_instruction() {
    let (fd, cbr, root, arms) = diamond(0x100b, [(0x100d, Some(0x100d)), (0x1014, Some(0x1014))]);
    assert!(is_branch_instruction(&fd, cbr, root, arms));
}

/// Thumb `ite hi; movhi r3,#1; movls r3,#0` before conditional-execution merging
/// is applied to the arms: the `movhi` arm holds an op of the guard's instruction.
#[test]
fn it_block_arm_op_is_conditional_execution() {
    let (fd, cbr, root, arms) = diamond(0x1006, [(0x1006, Some(0x1006)), (0x1008, Some(0x1008))]);
    assert!(!is_branch_instruction(&fd, cbr, root, arms));
    assert!(!is_branch_instruction(&fd, cbr, root, [arms[1], arms[0]]));
}

/// After `ActionConditionalExe` merges the pair, both COPYs can sit at the
/// `movls` address; the `movhi` arm block still starts at the guard's address.
#[test]
fn merged_it_block_is_conditional_execution() {
    let (fd, cbr, root, arms) = diamond(0x1006, [(0x1006, Some(0x1008)), (0x1008, Some(0x1008))]);
    assert!(!is_branch_instruction(&fd, cbr, root, arms));
}

/// x86 `cmovg` triangle: one input comes straight from the root, the guarded
/// COPY is the rest of the CBRANCH's instruction.
#[test]
fn cmov_triangle_is_conditional_execution() {
    let (fd, cbr, root, arms) = diamond(0x1009, [(0x1009, Some(0x1009)), (0x100c, None)]);
    assert!(!is_branch_instruction(&fd, cbr, root, [root, arms[0]]));
}

/// A jump triangle (`mov eax,0; test; je L; mov eax,1; L:`), where the root's
/// own CBRANCH does not count, and an empty arm.
#[test]
fn jump_triangle_and_empty_arm_are_branch_instructions() {
    let (fd, cbr, root, arms) = diamond(0x100c, [(0x100e, Some(0x100e)), (0x1013, None)]);
    assert!(is_branch_instruction(&fd, cbr, root, [root, arms[0]]));
    assert!(is_branch_instruction(&fd, cbr, root, arms));
}

/// The select's value reaching each of `readers` (through a COPY for the first).
fn select_read_by(readers: &[OpCode]) -> bool {
    let (mut fd, ..) = diamond(0x100b, [(0x100d, None), (0x1014, None)]);
    let join = block(&mut fd, 0x1020, 0x1020);
    let multi = op_at(&mut fd, join, 0x1020, OpCode::CPUI_MULTIEQUAL);
    let out = out_of(&mut fd, multi, 0x40);
    let copy = op_at(&mut fd, join, 0x1022, OpCode::CPUI_COPY);
    read(&mut fd, copy, out);
    let copied = out_of(&mut fd, copy, 0x48);
    for (i, &opc) in readers.iter().enumerate() {
        let reader = op_at(&mut fd, join, 0x1024 + i as u64, opc);
        read(&mut fd, reader, if i == 0 { copied } else { out });
    }
    only_tested(&fd, out)
}

/// A value that only feeds branches and boolean tests is a materialized flag;
/// a returned, stored or computed-with value is not.
#[test]
fn only_tested_follows_copies_to_tests() {
    assert!(select_read_by(&[OpCode::CPUI_CBRANCH]));
    assert!(select_read_by(&[OpCode::CPUI_INT_EQUAL, OpCode::CPUI_BOOL_NEGATE]));
    assert!(!select_read_by(&[]));
    assert!(!select_read_by(&[OpCode::CPUI_CBRANCH, OpCode::CPUI_RETURN]));
    assert!(!select_read_by(&[OpCode::CPUI_INT_ADD]));
}

/// Arms that only assign the constant (the `mov $1,%ecx` of an overflow check)
/// against an arm that also does work (`set = 1; noeval++;`).
#[test]
fn arms_only_assign_rejects_other_work() {
    let (mut fd, _, root, arms) = diamond(0x100b, [(0x100d, None), (0x1014, None)]);
    let one = op_at(&mut fd, arms[0], 0x100d, OpCode::CPUI_COPY);
    let c1 = constant(&mut fd, 1);
    read(&mut fd, one, c1);
    op_at(&mut fd, arms[0], 0x1012, OpCode::CPUI_BRANCH);
    assert!(arms_only_assign(&fd, root, arms));
    assert!(arms_only_assign(&fd, root, [root, arms[0]]));
    op_at(&mut fd, arms[1], 0x1014, OpCode::CPUI_STORE);
    assert!(!arms_only_assign(&fd, root, arms));
    let reg = op_at(&mut fd, arms[0], 0x1010, OpCode::CPUI_COPY);
    let x = out_of(&mut fd, one, 0x50);
    read(&mut fd, reg, x);
    assert!(!arms_only_assign(&fd, root, [root, arms[0]]));
}

/// A CBRANCH on `!NAN(x)` (the x86 `jp` of a float `!=`) is the compiler's; one
/// on an integer comparison is not.
#[test]
fn branch_on_nan_is_recognized_through_a_negation() {
    for (first, expect) in [(OpCode::CPUI_FLOAT_NAN, true), (OpCode::CPUI_INT_EQUAL, false)] {
        let (mut fd, _, root, _) = diamond(0x100e, [(0x1010, None), (0x1018, None)]);
        let test = op_at(&mut fd, root, 0x100a, first);
        let flag = out_of(&mut fd, test, 0x60);
        let neg = op_at(&mut fd, root, 0x100e, OpCode::CPUI_BOOL_NEGATE);
        read(&mut fd, neg, flag);
        let cond = out_of(&mut fd, neg, 0x68);
        let cbr = fd.new_op(2, ram(&fd, 0x100e));
        fd.obank_mut().change_opcode(cbr, TypeOp::new(OpCode::CPUI_CBRANCH, 0, "CBRANCH".to_string()));
        fd.obank_mut().get_mut(cbr).unwrap().set_input(Some(cond), 1);
        fd.vbank_mut().add_descend(cond, cbr).unwrap();
        fd.op_insert_end(cbr, root);
        assert_eq!(branches_on_nan(&fd, cbr), expect, "{first:?}");
    }
}

#[test]
fn parses_fold_keep_and_return() {
    let flags = |v: &str| parse_bool_branch(v).map(|(keep, ret, _)| (keep, ret)).unwrap();
    assert_eq!(flags("return"), (true, true));
    assert_eq!(flags("keep"), (true, false));
    assert_eq!(flags("fold"), (false, false));
    assert!(parse_bool_branch("on").is_err());
}
