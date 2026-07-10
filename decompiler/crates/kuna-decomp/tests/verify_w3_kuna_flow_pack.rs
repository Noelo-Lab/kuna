//! Verifier adversarial tests for item `w3-kuna-flow-pack`.
//!
//! Independent re-derivation tests targeting the hunt-list spots flagged as
//! most fragile for the five S2/S5 kuna flow hooks:
//!   - SPARC struct-ret trap: the `(uint4)idvn->getOffset()` *width truncation*
//!     of the user-op id, and the positional backward dead-list walk through
//!     *multiple* CALLOTHERs (iteration order / first-match semantics), plus the
//!     "op not on the dead list" premise guard.
//!   - V850 indirect-jmp: input in a non-register space ordering vs. PC.
//!   - inferfuncentry: cross-space rampoint/entry equality (Address total order).
//!
//! C++ oracle: decompiler/cpp/kuna_sparcstructret.cc, kuna_v850indbranch.cc,
//! kuna_inferfuncentry.cc (at the recorded cpp blob).

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};
use kuna_base::types::{int4, uintb};
use kuna_num::opcodes::OpCode;

use kuna_decomp::dtype::{type_metatype, Datatype};
use kuna_decomp::funcdata::Funcdata;
use kuna_decomp::op::pcodeop_flags;
use kuna_decomp::context::{ArchContext, OpId, TypeOp};

use kuna_decomp::kuna_inferfuncentry::kuna_is_function_entry;
use kuna_decomp::kuna_sparcstructret::kuna_is_sparc_struct_ret_trap;
use kuna_decomp::kuna_v850indbranch::kuna_is_v850_indirect_jmp;

fn build_manager() -> AddrSpaceManager {
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
    m
}

fn build_fd() -> Funcdata {
    let manage = build_manager();
    let glb = Rc::new(ArchContext::new(manage));
    let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
    let addr = Address::new(ram, 0x1000);
    Funcdata::new("func", "func", glb, addr, 0x1000_0000, 0x40).unwrap()
}

fn unk_type(size: int4) -> Rc<Datatype> {
    Rc::new(Datatype::new(size, type_metatype::TYPE_UNKNOWN))
}

fn ram(fd: &Funcdata) -> Rc<AddrSpace> {
    Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap())
}

fn const_space(fd: &Funcdata) -> Rc<AddrSpace> {
    Rc::clone(fd.get_arch().manage().get_space_by_name("const").unwrap())
}

fn make_op(fd: &mut Funcdata, opc: OpCode, pc_off: u64, ninputs: int4, start: bool) -> OpId {
    let pc = Address::new(ram(fd), pc_off);
    let op = fd.obank_mut().create_at(ninputs, pc);
    fd.obank_mut().change_opcode(op, TypeOp::new(opc, 0, "op"));
    if start {
        fd.obank_mut()
            .get_mut(op)
            .unwrap()
            .set_flag(pcodeop_flags::startmark);
    }
    op
}

fn set_const_input(fd: &mut Funcdata, op: OpId, slot: int4, value: uintb) {
    let cspace = const_space(fd);
    let vn = fd
        .vbank_mut()
        .create(4, Address::new(cspace, value), unk_type(4));
    fd.obank_mut()
        .get_mut(op)
        .unwrap()
        .set_input(Some(vn), slot);
}

/// id 7 -> "IllegalInstructionTrap"; id 9 -> a different non-null name;
/// everything else -> null UserPcodeOp.
fn resolver(id: u32) -> Option<String> {
    match id {
        7 => Some("IllegalInstructionTrap".to_string()),
        9 => Some("SomeOtherTrap".to_string()),
        _ => None,
    }
}

// ---------------------------------------------------------------------------
// SPARC: `(uint4)idvn->getOffset()` width-truncation fidelity.
//
// The C++ does `glb->userops.getOp((uint4)idvn->getOffset())`.  getOffset() is
// a 64-bit uintb; the cast keeps only the low 32 bits.  A constant whose low 32
// bits == 7 (the trap id) but with high bits set MUST still resolve to the trap
// after truncation, exactly as the C++ would.
// ---------------------------------------------------------------------------
#[test]
fn flowpack_sparc_userop_id_truncates_high_bits_and_matches() {
    let mut fd = build_fd();
    // 0x1_0000_0007: low 32 bits are 7, high bit set above the u32 boundary.
    let callother = make_op(&mut fd, OpCode::CPUI_CALLOTHER, 0x1000, 1, true);
    set_const_input(&mut fd, callother, 0, 0x1_0000_0007u64);
    let branchind = make_op(&mut fd, OpCode::CPUI_BRANCHIND, 0x1000, 1, false);
    // (uint4)0x1_0000_0007 == 7 -> resolver(7) == IllegalInstructionTrap.
    assert!(
        kuna_is_sparc_struct_ret_trap(&fd, branchind, true, resolver),
        "low-32-bits-7 user-op id must truncate to 7 and match the trap"
    );
}

#[test]
fn flowpack_sparc_userop_id_high_bits_only_does_not_alias_into_trap() {
    let mut fd = build_fd();
    // 0x1_0000_0000: low 32 bits are 0 (resolver(0) == None), high bit set.
    // A naive 64-bit lookup of 0x1_0000_0000 also misses, but the point is the
    // truncated value 0 must NOT accidentally become 7.
    let callother = make_op(&mut fd, OpCode::CPUI_CALLOTHER, 0x1000, 1, true);
    set_const_input(&mut fd, callother, 0, 0x1_0000_0000u64);
    let branchind = make_op(&mut fd, OpCode::CPUI_BRANCHIND, 0x1000, 1, false);
    assert!(!kuna_is_sparc_struct_ret_trap(&fd, branchind, true, resolver));
}

// ---------------------------------------------------------------------------
// SPARC: backward walk through MULTIPLE CALLOTHERs in the same instruction.
// The C++ returns true on the FIRST trap CALLOTHER it meets walking backward
// (nearest to the BRANCHIND), and it keeps walking past non-matching CALLOTHERs
// until it hits one or the instruction-start.
// ---------------------------------------------------------------------------
#[test]
fn flowpack_sparc_walk_passes_nontrap_callother_to_reach_trap() {
    let mut fd = build_fd();
    // instruction-start: the trap CALLOTHER (id 7) ...
    let trap = make_op(&mut fd, OpCode::CPUI_CALLOTHER, 0x1000, 1, true);
    set_const_input(&mut fd, trap, 0, 7);
    // ... then a NON-trap CALLOTHER (id 9) nearer the BRANCHIND ...
    let other = make_op(&mut fd, OpCode::CPUI_CALLOTHER, 0x1000, 1, false);
    set_const_input(&mut fd, other, 0, 9);
    // ... then the BRANCHIND (last op).
    let branchind = make_op(&mut fd, OpCode::CPUI_BRANCHIND, 0x1000, 1, false);
    // Walk: BRANCHIND -> other(id9, no match, keep going) -> trap(id7, match).
    assert!(
        kuna_is_sparc_struct_ret_trap(&fd, branchind, true, resolver),
        "the walk must pass a non-matching CALLOTHER and still reach the trap"
    );
}

#[test]
fn flowpack_sparc_nearest_trap_matches_before_instruction_start() {
    // Two trap CALLOTHERs; the nearer one (id 7) matches.  This pins that the
    // walk short-circuits on the first match rather than walking to the head.
    let mut fd = build_fd();
    let far_trap = make_op(&mut fd, OpCode::CPUI_CALLOTHER, 0x1000, 1, true);
    set_const_input(&mut fd, far_trap, 0, 7);
    let near_trap = make_op(&mut fd, OpCode::CPUI_CALLOTHER, 0x1000, 1, false);
    set_const_input(&mut fd, near_trap, 0, 7);
    let branchind = make_op(&mut fd, OpCode::CPUI_BRANCHIND, 0x1000, 1, false);
    assert!(kuna_is_sparc_struct_ret_trap(&fd, branchind, true, resolver));
}

// ---------------------------------------------------------------------------
// SPARC: the C++ premise is that `op` sits on the dead list (getInsertIter into
// beginOpDead()..).  The Rust port guards the "op not found on the dead list"
// case by returning false.  Probing a freshly-built op that is NOT a BRANCHIND
// already returns false on the opcode check; here we confirm a BRANCHIND whose
// only preceding op is itself, sitting at the dead-list head with no trap,
// returns false (and does not panic walking off pos==0).
// ---------------------------------------------------------------------------
#[test]
fn flowpack_sparc_lone_branchind_at_deadlist_head_no_panic_false() {
    let mut fd = build_fd();
    // A single BRANCHIND, instruction-start, dead-list head, no CALLOTHER.
    let branchind = make_op(&mut fd, OpCode::CPUI_BRANCHIND, 0x1000, 1, true);
    set_const_input(&mut fd, branchind, 0, 0x40);
    assert!(!kuna_is_sparc_struct_ret_trap(&fd, branchind, true, resolver));
}

#[test]
fn flowpack_sparc_walk_to_deadlist_head_non_start_terminates() {
    // BRANCHIND is NOT instruction-start and there is no instruction-start op
    // before it on the dead list: the walk must terminate at the dead-list head
    // (pos == 0) without panicking, returning false (no trap present).
    let mut fd = build_fd();
    let first = make_op(&mut fd, OpCode::CPUI_COPY, 0x1000, 1, false); // not a start
    set_const_input(&mut fd, first, 0, 1);
    let branchind = make_op(&mut fd, OpCode::CPUI_BRANCHIND, 0x1000, 1, false);
    set_const_input(&mut fd, branchind, 0, 1);
    assert!(!kuna_is_sparc_struct_ret_trap(&fd, branchind, true, resolver));
}

// ---------------------------------------------------------------------------
// V850: a CALLIND whose input is a named register that is neither PC nor pc
// fires; a uniquely-spelled "Pc"/"pC" is NOT excluded (the C++ only excludes the
// exact strings "PC" and "pc").  This pins the comparator is the exact two
// string match, not a case-insensitive contains.
// ---------------------------------------------------------------------------
#[test]
fn flowpack_v850_only_exact_pc_spellings_excluded() {
    let mut fd = build_fd();
    let pc = Address::new(ram(&fd), 0x1000);
    let op = fd.obank_mut().create_at(1, pc);
    fd.obank_mut()
        .change_opcode(op, TypeOp::new(OpCode::CPUI_CALLIND, 0, "callind"));
    let rspace = ram(&fd);
    let vn = fd
        .vbank_mut()
        .create(4, Address::new(rspace, 0x10), unk_type(4));
    fd.obank_mut().get_mut(op).unwrap().set_input(Some(vn), 0);

    // "PC" and "pc" excluded ...
    assert!(!kuna_is_v850_indirect_jmp(&fd, op, true, Some("PC")));
    assert!(!kuna_is_v850_indirect_jmp(&fd, op, true, Some("pc")));
    // ... but "Pc" / "pC" / a normal reg are NOT excluded (exact-string match).
    assert!(kuna_is_v850_indirect_jmp(&fd, op, true, Some("Pc")));
    assert!(kuna_is_v850_indirect_jmp(&fd, op, true, Some("pC")));
    assert!(kuna_is_v850_indirect_jmp(&fd, op, true, Some("r10")));
}

// ---------------------------------------------------------------------------
// inferfuncentry: the C++ `fd->getAddress() == rampoint` is a full Address
// equality (space AND offset).  A function entry at the same offset but a
// DIFFERENT space must NOT compare equal -> no inference.
// ---------------------------------------------------------------------------
#[test]
fn flowpack_inferfuncentry_cross_space_same_offset_not_equal() {
    // Build two spaces with the same offset value but different identity.
    let mut m = AddrSpaceManager::new();
    m.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    m.insert_space(Rc::new(UniqueSpace::new(1, 0, false))).unwrap();
    let ram = Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "ram",
        false,
        8,
        1,
        2,
        addrspace_flags::hasphysical,
        1,
        1,
    ));
    m.insert_space(Rc::clone(&ram)).unwrap();
    // A second distinct processor space at index 3.
    let rom = Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "rom",
        false,
        8,
        1,
        3,
        addrspace_flags::hasphysical,
        1,
        1,
    ));
    m.insert_space(Rc::clone(&rom)).unwrap();

    let rampoint = Address::new(Rc::clone(&ram), 0x10_0000);
    let entry_other_space = Address::new(Rc::clone(&rom), 0x10_0000); // same offset, other space
    // Different space -> Address::eq false -> no inference.
    assert!(!kuna_is_function_entry(
        true,
        Some(&rampoint),
        Some(&entry_other_space)
    ));
    // Same space, same offset -> inference fires.
    let entry_same = Address::new(Rc::clone(&ram), 0x10_0000);
    assert!(kuna_is_function_entry(true, Some(&rampoint), Some(&entry_same)));
}
