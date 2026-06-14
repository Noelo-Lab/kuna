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
