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

    // A non-additive reader of the SP input (a STORE through it), inside a basic
    // block (annotateRawStackPtr splices a PTRSUB *before* the reader op, so the
    // reader must be placed in a block).  CPUI_STORE's slot 1 is the pointer;
    // reading the SP there is a "raw stack pointer" use.
    let root = fd.bblocks_root_pub();
    let bl = fd.bblocks_mut().new_block_basic(root);
    let store = fd.new_op(3, reg_addr(&fd, 0x1000));
    fd.op_set_opcode_code(store, OpCode::CPUI_STORE);
    // slot 0 = space id (a const), slot 1 = pointer (the SP), slot 2 = value.
    let spaceconst = fd.new_constant(8, 0);
    let value = fd.new_varnode(4, &reg_addr(&fd, 0x100), None);
    fd.op_set_input(store, spaceconst, 0).unwrap();
    fd.op_set_input(store, sp_in, 1).unwrap();
    fd.op_set_input(store, value, 2).unwrap();
    fd.bb_insert_op(store, bl, None);

    let stackspc = Rc::clone(fd.get_arch().manage().get_stack_space().unwrap());

    // Pre: the STORE reads the SP input directly at slot 1.
    assert_eq!(fd.obank().get(store).unwrap().get_in(1), Some(sp_in));

    fd.annotate_raw_stack_ptr(&stackspc);

    // Post: slot 1 now reads a fresh Varnode defined by PTRSUB(sp, #0).
    let new_in1 = fd.obank().get(store).unwrap().get_in(1).unwrap();
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
