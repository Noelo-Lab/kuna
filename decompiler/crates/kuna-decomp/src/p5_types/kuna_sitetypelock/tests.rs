// Tests for `kuna_sitetypelock` (included into `mod tests` in
// kuna_sitetypelock.rs).
//
// Covered:
//   - the Override store: insert/find/replace/clear round-trip.
//   - `apply_pending`: pins the lock onto the call output, survives a later
//     `update_type` (the write-back plane), is idempotent, and refuses a
//     pre-locked output, a missing output and a size mismatch.
// The end-to-end path (a real stripped Go binary, the declaration in the C)
// is `verify_gotypes::stripped_go_newobject_result_takes_descriptor_type`.

use super::*;

use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};
use kuna_num::opcodes::OpCode;

use crate::context::{ArchContext, OpId};
use crate::dtype::{type_metatype, Datatype};
use crate::fspec::FuncCallSpecs;

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
    let ctx = ArchContext::new(m);
    let glb = Rc::new(ctx);
    let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
    let addr = Address::new(ram, 0x1000);
    Funcdata::new("func", "func", glb, addr, 0x1000_0000, 0x40).unwrap()
}

/// A CALL op at `pc` with a constant target and (optionally) an 8-byte
/// unique output, registered as a call spec.
fn call_at(data: &mut Funcdata, pc: u64, with_out: bool) -> (OpId, Address) {
    let ram = Rc::clone(data.get_arch().manage().get_space_by_name("ram").unwrap());
    let at = Address::new(ram, pc);
    let op = data.new_op(1, at.clone());
    data.op_set_opcode_code(op, OpCode::CPUI_CALL);
    let target = data.new_constant(8, 0x2000);
    data.op_set_input(op, target, 0).unwrap();
    if with_out {
        data.new_unique_out(8, op).unwrap();
    }
    let fc = FuncCallSpecs::new(op, Address::new(Rc::clone(&data.get_arch().manage().get_space_by_name("ram").unwrap()), 0x2000));
    data.push_call_specs(fc);
    (op, at)
}

fn int8_type() -> Rc<Datatype> {
    Rc::new(Datatype::new(8, type_metatype::TYPE_INT))
}

#[test]
fn override_store_round_trips() {
    let mut data = build_fd();
    let ram = Rc::clone(data.get_arch().manage().get_space_by_name("ram").unwrap());
    let site = Address::new(Rc::clone(&ram), 0x1100);
    assert!(!data.get_override().has_output_typelocks());
    assert!(data.get_override().find_output_typelock(&site).is_none());

    let first = int8_type();
    data.get_override_mut().insert_output_typelock(site.clone(), Rc::clone(&first));
    assert!(data.get_override().has_output_typelocks());
    assert!(Rc::ptr_eq(data.get_override().find_output_typelock(&site).unwrap(), &first));
    assert!(data
        .get_override()
        .find_output_typelock(&Address::new(ram, 0x1101))
        .is_none());

    // A later insert at the same call point replaces the earlier one.
    let second = Rc::new(Datatype::new(8, type_metatype::TYPE_UINT));
    data.get_override_mut().insert_output_typelock(site.clone(), Rc::clone(&second));
    assert!(Rc::ptr_eq(data.get_override().find_output_typelock(&site).unwrap(), &second));

    data.get_override_mut().clear();
    assert!(!data.get_override().has_output_typelocks());
}

#[test]
fn apply_pins_the_output_and_survives_update_type() {
    let mut data = build_fd();
    let (op, site) = call_at(&mut data, 0x1100, true);
    let ct = int8_type();
    data.get_override_mut().insert_output_typelock(site, Rc::clone(&ct));

    assert!(apply_pending(&mut data));
    let out = data.obank().get(op).unwrap().get_out().unwrap();
    {
        let v = data.vbank().get(out).unwrap();
        assert!(v.is_type_lock(), "the output Varnode is type-locked");
        assert!(Rc::ptr_eq(v.get_type(), &ct));
    }

    // The write-back plane cannot replace a locked type, and re-applying the
    // same lock changes nothing.
    let other = Rc::new(Datatype::new(8, type_metatype::TYPE_UINT));
    assert!(!data.vbank_mut().get_mut(out).unwrap().update_type(other));
    assert!(Rc::ptr_eq(data.vbank().get(out).unwrap().get_type(), &ct));
    assert!(!apply_pending(&mut data));
}

#[test]
fn apply_refuses_prelocked_missing_output_and_size_mismatch() {
    let mut data = build_fd();

    // No entries at all: the empty-store fast path.
    assert!(!apply_pending(&mut data));

    // A call with no output Varnode is left alone.
    let (_dead, dead_site) = call_at(&mut data, 0x1100, false);
    data.get_override_mut().insert_output_typelock(dead_site, int8_type());
    assert!(!apply_pending(&mut data));

    // A pre-locked output (mapped-symbol/DWARF evidence) is never replaced.
    let (locked_op, locked_site) = call_at(&mut data, 0x1200, true);
    let prior = Rc::new(Datatype::new(8, type_metatype::TYPE_UINT));
    let locked_out = data.obank().get(locked_op).unwrap().get_out().unwrap();
    assert!(data.vn_update_type_locked(locked_out, Rc::clone(&prior), true, false));
    data.get_override_mut().insert_output_typelock(locked_site, int8_type());
    assert!(!apply_pending(&mut data));
    assert!(Rc::ptr_eq(data.vbank().get(locked_out).unwrap().get_type(), &prior));

    // A lock whose size differs from the output's is refused.
    let (short_op, short_site) = call_at(&mut data, 0x1300, true);
    data.get_override_mut()
        .insert_output_typelock(short_site, Rc::new(Datatype::new(4, type_metatype::TYPE_INT)));
    assert!(!apply_pending(&mut data));
    let short_out = data.obank().get(short_op).unwrap().get_out().unwrap();
    assert!(!data.vbank().get(short_out).unwrap().is_type_lock());
}
