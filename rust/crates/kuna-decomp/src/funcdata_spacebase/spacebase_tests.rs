// Positive unit tests for the front of the stack-var promotion chain:
// `Funcdata::spacebase` — the Varnode `spacebase` mark (chain link 1).  The mark
// fires independently of the type factory (the type-set is gated on a successful
// `getTypeSpacebase`, but the `setFlags(spacebase)` on every base register is
// unconditional), so a hand-built fixture with a stack spacebase suffices to pin
// it.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, SpacebaseSpace,
    UniqueSpace, VarnodeStorage,
};
use kuna_base::types::int4;

use crate::funcdata::Funcdata;
use crate::seams::{Architecture, VarnodeId};

const SP_OFF: u64 = 0x20;
const SP_SIZE: int4 = 8;

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
    m.insert_space(Rc::new(SpacebaseSpace::new("stack", 3, 8, &regspc, 1, true, false)))
        .unwrap();
    m
}

fn build_fd() -> Funcdata {
    let manage = build_manager();
    let regspc = Rc::clone(manage.get_space_by_name("register").unwrap());
    let stackspc = Rc::clone(manage.get_stack_space().unwrap());
    let sp_data = VarnodeStorage { space: Some(regspc), offset: SP_OFF, size: SP_SIZE as u32 };
    manage.add_spacebase_pointer(&stackspc, &sp_data, SP_SIZE, true).unwrap();
    let glb = Rc::new(Architecture::new(manage));
    let code = Rc::clone(glb.manage().get_space_by_name("register").unwrap());
    let entry = Address::new(code, 0x1000);
    Funcdata::new("func", "func", glb, entry, 0x1000_0000, 0x40).unwrap()
}

fn reg_addr(fd: &Funcdata, off: u64) -> Address {
    Address::new(Rc::clone(fd.get_arch().manage().get_space_by_name("register").unwrap()), off)
}

fn make_sp_in(fd: &mut Funcdata) -> VarnodeId {
    let vn = fd.new_varnode(SP_SIZE, &reg_addr(fd, SP_OFF), None);
    fd.set_input_varnode(vn).unwrap()
}

#[test]
fn spacebase_marks_input_stack_pointer() {
    let mut fd = build_fd();
    let sp_in = make_sp_in(&mut fd);

    // Before: the SP input is not yet a spacebase.
    assert!(!fd.vbank().get(sp_in).unwrap().is_spacebase());

    fd.spacebase();

    // After: Funcdata::spacebase has marked the SP-input Varnode spacebase.
    assert!(
        fd.vbank().get(sp_in).unwrap().is_spacebase(),
        "the input stack-pointer Varnode must be marked spacebase"
    );
}

#[test]
fn spacebase_leaves_free_varnodes_unmarked() {
    let mut fd = build_fd();
    // A free Varnode at the SP location (not input, not written) must be skipped
    // (C++ `if (vn->isFree()) continue;`).
    let free_vn = fd.new_varnode(SP_SIZE, &reg_addr(&fd, SP_OFF), None);
    assert!(fd.vbank().get(free_vn).unwrap().is_free());

    fd.spacebase();

    assert!(
        !fd.vbank().get(free_vn).unwrap().is_spacebase(),
        "a free Varnode at the SP location must not be marked spacebase"
    );
}

// --- ScopeLocal restructure tail (w10-stacklocal-typing) -------------------

/// `ScopeLocal::annotateRawStackPtr` (varmap.cc:386): a *non-additive* read of
/// the input stack pointer gets a `PTRSUB(sp, #0)` placeholder spliced in, so a
/// raw stack-pointer use renders `&local` rather than the bare register.
#[test]
fn annotate_raw_stack_ptr_splices_ptrsub_zero_before_nonadditive_use() {
    use kuna_num::opcodes::OpCode;

    let mut fd = build_fd();
    let sp_in = make_sp_in(&mut fd);
    fd.spacebase(); // marks + types the SP input
    assert!(fd.start_type_recovery());

    // A non-additive, non-special reader of the SP input, inside a basic block
    // (annotateRawStackPtr splices a PTRSUB *before* the reader op, so the reader
    // must be placed in a block).  C++ `annotateRawStackPtr` SKIPS `special`
    // non-call ops (STORE/LOAD/CALL...), so the reader must be an ordinary
    // (non-special) op: `INT_NEGATE` is a unary read of the SP — a genuine "raw
    // stack pointer" use that is neither additive (INT_ADD/PTRSUB/PTRADD) nor
    // special.
    let root = fd.bblocks_root_pub();
    let bl = fd.bblocks_mut().new_block_basic(root);
    let store = fd.new_op(1, reg_addr(&fd, 0x1000));
    fd.op_set_opcode_code(store, OpCode::CPUI_INT_NEGATE);
    let out = fd.new_varnode(8, &reg_addr(&fd, 0x200), None);
    fd.op_set_output(store, out).unwrap();
    fd.op_set_input(store, sp_in, 0).unwrap();
    fd.bb_insert_op(store, bl, None);

    let stackspc = Rc::clone(fd.get_arch().manage().get_stack_space().unwrap());

    // Pre: the reader reads the SP input directly at slot 0.
    assert_eq!(fd.obank().get(store).unwrap().get_in(0), Some(sp_in));

    fd.annotate_raw_stack_ptr(&stackspc);

    // Post: slot 0 now reads a fresh Varnode defined by PTRSUB(sp, #0).
    let new_in1 = fd.obank().get(store).unwrap().get_in(0).unwrap();
    assert_ne!(new_in1, sp_in, "the raw SP read must be replaced by the PTRSUB out");
    let def = fd.vbank().get(new_in1).unwrap().get_def().expect("PTRSUB out must be written");
    assert_eq!(fd.obank().get(def).unwrap().code(), OpCode::CPUI_PTRSUB);
    // PTRSUB(spVn, const#0)
    assert_eq!(fd.obank().get(def).unwrap().get_in(0), Some(sp_in));
    let c = fd.obank().get(def).unwrap().get_in(1).unwrap();
    assert!(fd.vbank().get(c).unwrap().is_constant());
    assert_eq!(fd.vbank().get(c).unwrap().get_offset(), 0);
}

/// `annotateRawStackPtr` skips *additive* uses (INT_ADD/PTRSUB/PTRADD): those
/// are already pointer arithmetic into the frame and must not get a placeholder.
#[test]
fn annotate_raw_stack_ptr_skips_additive_use() {
    use kuna_num::opcodes::OpCode;

    let mut fd = build_fd();
    let sp_in = make_sp_in(&mut fd);
    fd.spacebase();
    assert!(fd.start_type_recovery());

    // An INT_ADD off the SP (additive use) — must be left untouched.
    let add = fd.new_op(2, reg_addr(&fd, 0x1000));
    fd.op_set_opcode_code(add, OpCode::CPUI_INT_ADD);
    let off = fd.new_constant(SP_SIZE, 0x10);
    fd.op_set_input(add, sp_in, 0).unwrap();
    fd.op_set_input(add, off, 1).unwrap();

    let stackspc = Rc::clone(fd.get_arch().manage().get_stack_space().unwrap());
    fd.annotate_raw_stack_ptr(&stackspc);

    // The INT_ADD still reads the SP directly at slot 0 (no PTRSUB spliced).
    assert_eq!(fd.obank().get(add).unwrap().get_in(0), Some(sp_in));
}

/// `ScopeLocal::addTypeRecommendation` / `hasTypeRecommendations`
/// (varmap.cc:1590, varmap.hh:259) and the `Funcdata::applyTypeRecommendations`
/// (varmap.cc:1574) round-trip: a recommendation for an input Varnode's storage
/// locks the recommended type onto it.
#[test]
fn apply_type_recommendations_locks_input_varnode_type() {
    use crate::dtype::type_metatype;

    let mut fd = build_fd();
    // A plain 8-byte input Varnode at a register slot (not the SP, so its initial
    // type is unlocked).
    let in_addr = reg_addr(&fd, 0x80);
    let in_vn = fd.new_varnode(8, &in_addr, None);
    let in_vn = fd.set_input_varnode(in_vn).unwrap();
    assert!(!fd.vbank().get(in_vn).unwrap().is_type_lock());

    // Build an explicit 8-byte INT recommendation at the input's storage (a
    // standalone factory suffices; the recommendation only carries an `Rc<Datatype>`).
    use crate::dtype::TypeFactory as _;
    let factory = crate::dtype::TypeFactoryImpl::new();
    factory.set_default_alignment_map();
    factory.set_max_basetype_size(8);
    let dt = factory.get_base(8, type_metatype::TYPE_INT).unwrap();

    assert!(!fd.get_scope_local().unwrap().has_type_recommendations());
    fd.get_scope_local_mut().unwrap().add_type_recommendation(in_addr, dt);
    assert!(fd.get_scope_local().unwrap().has_type_recommendations());

    // apply locks the recommended type onto the matching input Varnode
    // (`findVarnodeInput(dt->getSize(), addr)` -> `updateType(dt, true, false)`).
    fd.apply_type_recommendations();
    assert!(
        fd.vbank().get(in_vn).unwrap().is_type_lock(),
        "applyTypeRecommendations must type-lock the input Varnode at the recommended address"
    );
    assert_eq!(
        fd.vbank().get(in_vn).unwrap().get_type().get_metatype(),
        type_metatype::TYPE_INT
    );
}

// === VERIFIER ADVERSARIAL TESTS (item w10-stacklocal-typing) ===============
//
// These pin the conservative arms of the restructure tail the porter's
// positive tests do NOT exercise: the early-return guards and the
// size/metatype discrimination that prevent the chain from fabricating a type
// or a placeholder when the preconditions are not met.

/// `ScopeLocal::annotateRawStackPtr` (varmap.cc:389): the FIRST line is
/// `if (!fd->hasTypeRecoveryStarted()) return;`.  Before type recovery starts,
/// a non-additive read of the SP must be left UNTOUCHED (no PTRSUB spliced).
/// The porter's positive test always calls `start_type_recovery()` first, so
/// this OFF path is unverified — a port that dropped the guard would silently
/// rewrite IR too early.
#[test]
fn w10_stacklocal_typing_annotate_raw_stack_ptr_noop_before_type_recovery() {
    use kuna_num::opcodes::OpCode;

    let mut fd = build_fd();
    let sp_in = make_sp_in(&mut fd);
    fd.spacebase();
    // NOTE: deliberately do NOT call start_type_recovery().
    assert!(!fd.has_type_recovery_started());

    let root = fd.bblocks_root_pub();
    let bl = fd.bblocks_mut().new_block_basic(root);
    let store = fd.new_op(3, reg_addr(&fd, 0x1000));
    fd.op_set_opcode_code(store, OpCode::CPUI_STORE);
    let spaceconst = fd.new_constant(8, 0);
    let value = fd.new_varnode(4, &reg_addr(&fd, 0x100), None);
    fd.op_set_input(store, spaceconst, 0).unwrap();
    fd.op_set_input(store, sp_in, 1).unwrap();
    fd.op_set_input(store, value, 2).unwrap();
    fd.bb_insert_op(store, bl, None);

    let stackspc = Rc::clone(fd.get_arch().manage().get_stack_space().unwrap());
    fd.annotate_raw_stack_ptr(&stackspc);

    // The STORE still reads the raw SP at slot 1 — no placeholder before recovery.
    assert_eq!(
        fd.obank().get(store).unwrap().get_in(1),
        Some(sp_in),
        "annotateRawStackPtr must early-return when type recovery has not started"
    );
}

/// `applyTypeRecommendations` -> `fd->findVarnodeInput(dt->getSize(), addr)`
/// (varmap.cc:1580): the lookup is keyed on the recommendation's *size* as well
/// as its address.  A recommendation whose size does not match the input
/// Varnode at that address must NOT lock it (the `findVarnodeInput` returns 0).
/// The porter's test only covers the size-MATCH case; a port that ignored the
/// size in the lookup would type-lock the wrong-sized Varnode.
#[test]
fn w10_stacklocal_typing_apply_type_recommendations_size_mismatch_no_lock() {
    use crate::dtype::type_metatype;

    let mut fd = build_fd();
    // An 8-byte input Varnode at a register slot.
    let in_addr = reg_addr(&fd, 0x80);
    let in_vn = fd.new_varnode(8, &in_addr, None);
    let in_vn = fd.set_input_varnode(in_vn).unwrap();
    assert!(!fd.vbank().get(in_vn).unwrap().is_type_lock());

    // A *4-byte* INT recommendation at the same address — size mismatch vs the
    // 8-byte input, so findVarnodeInput(4, addr) finds nothing.
    use crate::dtype::TypeFactory as _;
    let factory = crate::dtype::TypeFactoryImpl::new();
    factory.set_default_alignment_map();
    factory.set_max_basetype_size(8);
    let dt4 = factory.get_base(4, type_metatype::TYPE_INT).unwrap();
    fd.get_scope_local_mut().unwrap().add_type_recommendation(in_addr, dt4);

    fd.apply_type_recommendations();

    assert!(
        !fd.vbank().get(in_vn).unwrap().is_type_lock(),
        "a size-mismatched recommendation must not lock the input Varnode \
         (findVarnodeInput keys on dt->getSize())"
    );
}

/// `applyTypeRecommendations` -> `vn->updateType(dt, true, false)` ->
/// `updateType` (varnode.cc:496): an UNKNOWN data-type is ALWAYS unlocked, so a
/// recommendation of an unknown type cannot fabricate a type-lock.  This pins
/// the UNKNOWN-unlock rule on the recommendation path (the porter's test uses a
/// concrete INT and never exercises it).
#[test]
fn w10_stacklocal_typing_apply_type_recommendations_unknown_stays_unlocked() {
    use crate::dtype::type_metatype;

    let mut fd = build_fd();
    let in_addr = reg_addr(&fd, 0x80);
    let in_vn = fd.new_varnode(4, &in_addr, None);
    let in_vn = fd.set_input_varnode(in_vn).unwrap();

    use crate::dtype::TypeFactory as _;
    let factory = crate::dtype::TypeFactoryImpl::new();
    factory.set_default_alignment_map();
    factory.set_max_basetype_size(8);
    let unk = factory.get_base(4, type_metatype::TYPE_UNKNOWN).unwrap();
    fd.get_scope_local_mut().unwrap().add_type_recommendation(in_addr, unk);

    fd.apply_type_recommendations();

    assert!(
        !fd.vbank().get(in_vn).unwrap().is_type_lock(),
        "an UNKNOWN-typed recommendation must never type-lock (updateType forces \
         lock=false for TYPE_UNKNOWN)"
    );
}

// --- L5 checkUnaliasedReturn alias-overlap decision ----------------------
//
// Pins the C++ `ScopeLocal::checkUnaliasedReturn` lower_bound/overlap rule
// (varmap.cc:421-425) that decides whether a return slot stays mapped (an alias
// reaches into it) or gets unmapped (none does).  Purely offset arithmetic — no
// register/name/value special-casing.

#[test]
fn adv_l5_alias_reaching_return_slot_keeps_it_mapped() {
    // Return slot [0x10, 0x18); an alias starts exactly at 0x10 -> reaches in.
    assert!(
        Funcdata::alias_reaches_return_slot(&[0x4, 0x10, 0x40], 0x10, 8),
        "an alias at the slot start reaches into the return storage (keep mapped)"
    );
    // Alias strictly inside the slot (0x14 in [0x10,0x18)).
    assert!(
        Funcdata::alias_reaches_return_slot(&[0x14], 0x10, 8),
        "an alias inside the slot reaches in"
    );
}

#[test]
fn adv_l5_alias_past_return_slot_unmaps_it() {
    // Nearest alias >= 0x10 is 0x18, which is just past the slot end (0x17).
    assert!(
        !Funcdata::alias_reaches_return_slot(&[0x4, 0x18, 0x40], 0x10, 8),
        "an alias one byte past the slot end does NOT reach in (unmap)"
    );
    // No alias at or above the slot offset at all.
    assert!(
        !Funcdata::alias_reaches_return_slot(&[0x0, 0x4, 0x8], 0x10, 8),
        "no alias >= the slot offset -> nothing reaches in (unmap)"
    );
    // Empty alias list -> nothing reaches in.
    assert!(
        !Funcdata::alias_reaches_return_slot(&[], 0x10, 8),
        "an empty alias list never reaches the slot"
    );
}

#[test]
fn adv_l5_alias_overlap_size_minus_one_is_inclusive() {
    // The C++ compares against `off + size - 1` (inclusive end).  A 1-byte slot at
    // 0x10 has last == 0x10; an alias exactly at 0x10 reaches it, 0x11 does not.
    assert!(
        Funcdata::alias_reaches_return_slot(&[0x10], 0x10, 1),
        "alias == off reaches a 1-byte slot (inclusive end off+size-1 == off)"
    );
    assert!(
        !Funcdata::alias_reaches_return_slot(&[0x11], 0x10, 1),
        "alias == off+1 is past a 1-byte slot"
    );
}
