// Tests for `kuna_gonewtype` (included into `mod tests` in kuna_gonewtype.rs).
//
// Covered:
//   - `descriptor_offset`: a constant, a COPY of a constant, and the refusals
//     (a non-constant argument, a non-COPY definition).
//   - `contribute` gate: `option gonewtype off` inserts nothing, and a
//     matching callee NAME alone (no resolvable descriptor / no factory) never
//     inserts a lock.
// The resolution against a real harvested `type:` symbol and the declaration
// in the C are pinned end to end by
// `verify_gotypes::stripped_go_newobject_result_takes_descriptor_type`.

use super::*;

use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};

use crate::context::ArchContext;
use crate::fspec::FuncCallSpecs;

fn build_fd(go_new_type: bool) -> Funcdata {
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
    let mut ctx = ArchContext::new(m);
    ctx.go_new_type = go_new_type;
    let glb = Rc::new(ctx);
    let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
    let addr = Address::new(ram, 0x1000);
    Funcdata::new("func", "func", glb, addr, 0x1000_0000, 0x40).unwrap()
}

/// A CALL at `pc` to a callee named `name`, with an 8-byte output and a
/// descriptor argument in slot 1.
fn newobject_call(data: &mut Funcdata, pc: u64, name: &str, arg: crate::context::VarnodeId) {
    let ram = Rc::clone(data.get_arch().manage().get_space_by_name("ram").unwrap());
    let at = Address::new(Rc::clone(&ram), pc);
    let op = data.new_op(2, at);
    data.op_set_opcode_code(op, OpCode::CPUI_CALL);
    let target = data.new_constant(8, 0x2000);
    data.op_set_input(op, target, 0).unwrap();
    data.op_set_input(op, arg, 1).unwrap();
    data.new_unique_out(8, op).unwrap();
    let mut fc = FuncCallSpecs::new(op, Address::new(ram, 0x2000));
    fc.set_funcdata(Address::new(Rc::clone(data.get_arch().manage().get_space_by_name("ram").unwrap()), 0x2000), name)
        .unwrap();
    data.push_call_specs(fc);
}

#[test]
fn descriptor_slot_names_the_allocator_family() {
    assert_eq!(descriptor_slot("runtime.newobject"), Some(1));
    assert_eq!(descriptor_slot("runtime.mallocgc"), Some(2));
    assert_eq!(descriptor_slot("runtime.mallocgcSmallScanNoHeaderSC4"), Some(2));
    assert_eq!(descriptor_slot("runtime.mallocgcTiny"), Some(2));
    assert_eq!(descriptor_slot("runtime.mallocgcLarge"), Some(2));
    assert_eq!(descriptor_slot("runtime.newarray"), None);
    assert_eq!(descriptor_slot("runtime.mallocgc.abi0"), None);
    assert_eq!(descriptor_slot("runtime.mallocgc_cleanup"), None);
    assert_eq!(descriptor_slot("mallocgc"), None);
    assert_eq!(descriptor_slot("main.newobject"), None);
}

#[test]
fn descriptor_offset_takes_a_constant_or_one_copy() {
    let mut data = build_fd(true);
    let c = data.new_constant(8, 0x5040);
    assert_eq!(descriptor_offset(&data, c), Some(0x5040));

    // One COPY away (the lifted `lea`).
    let ram = Rc::clone(data.get_arch().manage().get_space_by_name("ram").unwrap());
    let copy = data.new_op(1, Address::new(Rc::clone(&ram), 0x1100));
    data.op_set_opcode_code(copy, OpCode::CPUI_COPY);
    let src = data.new_constant(8, 0x6080);
    data.op_set_input(copy, src, 0).unwrap();
    let out = data.new_unique_out(8, copy).unwrap();
    assert_eq!(descriptor_offset(&data, out), Some(0x6080));

    // A non-COPY definition refuses.
    let add = data.new_op(2, Address::new(Rc::clone(&ram), 0x1200));
    data.op_set_opcode_code(add, OpCode::CPUI_INT_ADD);
    let a = data.new_constant(8, 1);
    let b = data.new_constant(8, 2);
    data.op_set_input(add, a, 0).unwrap();
    data.op_set_input(add, b, 1).unwrap();
    let sum = data.new_unique_out(8, add).unwrap();
    assert_eq!(descriptor_offset(&data, sum), None);

    // A free (undefined) Varnode refuses.
    let free = data.new_varnode(8, &Address::new(ram, 0x9000), None);
    assert_eq!(descriptor_offset(&data, free), None);
}

#[test]
fn contribute_gates_on_the_option_and_on_resolution() {
    // Option off: a perfect-looking site inserts nothing.
    let mut data = build_fd(false);
    let arg = data.new_constant(8, 0x5040);
    newobject_call(&mut data, 0x1100, NEWOBJECT, arg);
    contribute(&mut data);
    assert!(!data.get_override().has_output_typelocks());

    // Option on, but no global symbol table / type factory behind the
    // constant: the resolution refuses, nothing is inserted.
    let mut data = build_fd(true);
    let arg = data.new_constant(8, 0x5040);
    newobject_call(&mut data, 0x1100, NEWOBJECT, arg);
    let other = data.new_constant(8, 0x5040);
    newobject_call(&mut data, 0x1200, "main.otherAlloc", other);
    contribute(&mut data);
    assert!(!data.get_override().has_output_typelocks());
}
