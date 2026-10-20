use super::*;
use crate::context::ArchContext;
use crate::fspec::ParameterPieces;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, SpacebaseSpace,
    UniqueSpace,
};

fn frame_data() -> (Funcdata, Rc<AddrSpace>) {
    frame_data_with_views(false)
}

fn frame_data_with_views(full_views: bool) -> (Funcdata, Rc<AddrSpace>) {
    frame_data_with_options(full_views, true)
}

fn frame_data_with_options(full_views: bool, local_views: bool) -> (Funcdata, Rc<AddrSpace>) {
    let mut manage = AddrSpaceManager::new();
    manage.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    manage
        .insert_space(Rc::new(UniqueSpace::new(1, 0, false)))
        .unwrap();
    let register = Rc::new(AddrSpace::new(
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
    manage.insert_space(Rc::clone(&register)).unwrap();
    let stack = Rc::new(SpacebaseSpace::new(
        "stack", 3, 8, &register, 1, true, false,
    ));
    manage.insert_space(Rc::clone(&stack)).unwrap();
    let ram = Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "ram",
        false,
        8,
        1,
        4,
        addrspace_flags::hasphysical,
        1,
        1,
    ));
    manage.insert_space(Rc::clone(&ram)).unwrap();
    manage.set_default_code_space(ram.get_index()).unwrap();
    manage.set_default_data_space(ram.get_index()).unwrap();
    let mut context = ArchContext::new(manage);
    context.local_stack_views = local_views;
    context.stack_views = full_views;
    let mut fd = Funcdata::new(
        "early_views",
        "early_views",
        Rc::new(context),
        Address::new(ram, 0x1000),
        0x1000_0000,
        0x40,
    )
    .unwrap();
    let model = Rc::new(crate::fspec::ProtoModel::new(fd.get_arch().manage()));
    fd.get_func_proto_mut().set_model(Some(model));
    fd.get_func_proto_mut()
        .attach_internal_store(Rc::new(Datatype::new(0, type_metatype::TYPE_VOID)));
    (fd, stack)
}

fn store(fd: &mut Funcdata, pc: u64) -> OpId {
    let point = Address::new(
        fd.get_arch()
            .manage()
            .get_space_by_name("ram")
            .unwrap()
            .clone(),
        pc,
    );
    let op = fd.new_op(3, point);
    fd.op_set_opcode_code(op, OpCode::CPUI_STORE);
    let value = fd.new_constant(4, 0);
    fd.op_set_input(op, value, 2).unwrap();
    fd.obank_mut().mark_alive(op);
    op
}

#[test]
fn full_policy_disables_every_local_evidence_entry_point() {
    let (mut fd, stack) = frame_data_with_views(true);
    let address = Address::new(stack, (-32i64) as u64);
    let op = store(&mut fd, 0x1000);
    record_native(&mut fd, op);
    record_store(&mut fd, op, &address, 4);
    assert!(fd.local_stack_views.native.is_empty());
    assert!(fd.local_stack_views.writes.is_empty());
    fd.local_stack_views
        .footprints
        .insert((address.get_offset(), 4), Vec::new());
    assert!(!active(&fd));
    assert!(!preserves_range(&fd, &address, 4));
    assert!(views(&fd).is_empty());
    assert!(!backing(&fd, address.get_offset(), 4));
}

#[test]
fn only_original_live_store_identity_records_physical_extent() {
    let (mut fd, stack) = frame_data();
    let at = Address::new(stack, (-32i64) as u64);
    let original = store(&mut fd, 0x1000);
    let generated = store(&mut fd, 0x1000);
    record_native(&mut fd, original);
    record_store(&mut fd, generated, &at, 4);
    assert!(fd.local_stack_views.writes.is_empty());
    record_store(&mut fd, original, &at, 4);
    assert_eq!(
        fd.local_stack_views.writes.get(&original),
        Some(&(at.get_offset(), 4))
    );
    fd.clear();
    assert!(fd.local_stack_views.native.is_empty());
    assert!(fd.local_stack_views.writes.is_empty());
}

#[test]
fn nominal_view_names_must_remain_stable_c_identifiers() {
    for name in ["Pair", "_Pair2", "fx__Three"] {
        assert!(c_identifier(name));
    }
    for name in ["", "fx::Three", "Pair 2", "2Pair", "Páir", "Pair<int>"] {
        assert!(!c_identifier(name));
    }
}

#[test]
fn missing_formal_store_cannot_admit_local_evidence() {
    let (mut fd, stack) = frame_data();
    let mut proto = crate::fspec::FuncProto::new();
    proto.set_model(Some(Rc::new(crate::fspec::ProtoModel::new(
        fd.get_arch().manage(),
    ))));
    *fd.get_func_proto_mut() = proto;
    let at = Address::new(stack, (-32i64) as u64);
    assert!(!local(&fd, &at, 4));
    let original = store(&mut fd, 0x1000);
    record_native(&mut fd, original);
    record_store(&mut fd, original, &at, 4);
    assert!(fd.local_stack_views.writes.is_empty());
    assert!(fd.local_stack_views.early_window.is_none());
    assert!(!active(&fd));
}

#[test]
fn local_footprints_reject_formals_wrong_space_wrap_and_oversize() {
    let (mut fd, stack) = frame_data();
    let at = Address::new(stack.clone(), (-32i64) as u64);
    assert!(local(&fd, &at, 8));
    for width in [-1, 0, 257] {
        assert!(!local(&fd, &at, width));
    }
    assert!(!local(&fd, &Address::new(stack.clone(), u64::MAX), 2));
    let ram = fd
        .get_arch()
        .manage()
        .get_space_by_name("ram")
        .unwrap()
        .clone();
    assert!(!local(&fd, &Address::new(ram, at.get_offset()), 4));
    fd.get_func_proto_mut().set_param(
        0,
        "neighbor",
        &ParameterPieces {
            addr: Address::new(stack.clone(), (-24i64) as u64),
            type_: Some(Rc::new(Datatype::new(4, type_metatype::TYPE_INT))),
            flags: 0,
        },
    );
    assert!(local(&fd, &at, 8));
    assert!(!local(&fd, &at, 9));
    fd.get_func_proto_mut().set_param(
        1,
        "overlap",
        &ParameterPieces {
            addr: Address::new(stack, (-29i64) as u64),
            type_: Some(Rc::new(Datatype::new(1, type_metatype::TYPE_BOOL))),
            flags: 0,
        },
    );
    assert!(!local(&fd, &at, 4));
}

#[test]
fn provenance_overflow_invalidates_every_prior_candidate() {
    let (mut fd, _) = frame_data();
    fd.local_stack_views
        .footprints
        .insert(((-32i64) as u64, 4), Vec::new());
    for pc in 0..=MAX_RECORDS {
        let op = store(&mut fd, 0x1000 + pc as u64);
        record_native(&mut fd, op);
    }
    assert!(fd.local_stack_views.invalid);
    assert!(fd.local_stack_views.native.is_empty());
    assert!(fd.local_stack_views.footprints.is_empty());
    assert!(!active(&fd));
}

#[test]
fn hazard_extent_must_not_wrap_at_the_stack_address_width() {
    for bits in [4, 8] {
        let stack = AddrSpace::new(
            spacetype::IPTR_SPACEBASE,
            "stack",
            false,
            bits,
            1,
            3,
            addrspace_flags::hasphysical,
            1,
            1,
        );
        assert!(valid_extent(&stack, 0xfffffff0, 16));
        assert!(valid_extent(&stack, 0xffffffff, 1));
        assert_eq!(valid_extent(&stack, 0xfffffff8, 16), bits == 8);
        assert_eq!(valid_extent(&stack, 0xffffffff, 2), bits == 8);
        assert_eq!(valid_extent(&stack, 0x100000000, 1), bits == 8);
        assert!(!valid_extent(&stack, u64::MAX, 2));
        assert!(!valid_extent(&stack, 0, 0));
        assert!(!valid_extent(&stack, 0, -1));
    }
}

#[test]
fn early_access_bounds_use_the_actual_load_width() {
    for width in [4, 8] {
        let (mut fd, stack) = frame_data();
        let start = (-32i64) as u64;
        let op = fd.new_op(2, fd.get_address().clone());
        fd.op_set_opcode_code(op, OpCode::CPUI_LOAD);
        let space = fd.new_constant(8, stack.get_index() as u64);
        let pointer = fd.new_constant(8, start + 4);
        fd.op_set_input(op, space, 0).unwrap();
        fd.op_set_input(op, pointer, 1).unwrap();
        fd.new_unique_out(width, op).unwrap();
        fd.obank_mut().mark_alive(op);
        assert_eq!(
            accesses_fit_early(&fd, start, 8, false),
            if width == 4 {
                Ok(())
            } else {
                Err(Refusal::Crossing)
            }
        );
    }
}

#[test]
fn raw_receiver_requires_an_uninterrupted_same_block_publication() {
    for barrier in ["none", "call", "write", "store", "other-block"] {
        let (mut fd, _) = frame_data();
        let root = fd.bblocks_ref().root.unwrap();
        let block = fd.bblocks_mut().new_block_basic(root);
        let ram = fd
            .get_arch()
            .manage()
            .get_space_by_name("ram")
            .unwrap()
            .clone();
        let receiver = Address::new(ram.clone(), 0x3000);
        let publication = fd.new_op(1, Address::new(ram.clone(), 0x1000));
        fd.op_set_opcode_code(publication, OpCode::CPUI_COPY);
        let value = fd.new_constant(8, 0);
        fd.op_set_input(publication, value, 0).unwrap();
        fd.new_varnode_out(8, &receiver, publication).unwrap();
        fd.op_insert_end(publication, block);
        match barrier {
            "call" => {
                let call = fd.new_op(1, Address::new(ram.clone(), 0x1001));
                fd.op_set_opcode_code(call, OpCode::CPUI_CALL);
                let target = fd.new_constant(8, 0x2000);
                fd.op_set_input(call, target, 0).unwrap();
                fd.op_insert_end(call, block);
            }
            "write" => {
                let copy = fd.new_op(1, Address::new(ram.clone(), 0x1001));
                fd.op_set_opcode_code(copy, OpCode::CPUI_COPY);
                let value = fd.new_constant(4, 0);
                fd.op_set_input(copy, value, 0).unwrap();
                fd.new_varnode_out(4, &Address::new(ram.clone(), 0x3004), copy)
                    .unwrap();
                fd.op_insert_end(copy, block);
            }
            "store" => {
                let op = fd.new_op(3, Address::new(ram.clone(), 0x1001));
                fd.op_set_opcode_code(op, OpCode::CPUI_STORE);
                let value = fd.new_constant(4, 0);
                fd.op_set_input(op, value, 2).unwrap();
                let register = fd
                    .get_arch()
                    .manage()
                    .get_space_by_name("register")
                    .unwrap()
                    .clone();
                let pointer = fd.new_varnode(8, &Address::new(register, 0x20), None);
                fd.op_set_input(op, pointer, 1).unwrap();
                fd.op_insert_end(op, block);
            }
            _ => (),
        }
        let reader = fd.new_op(1, Address::new(ram, 0x1002));
        fd.op_set_opcode_code(reader, OpCode::CPUI_COPY);
        let read_block = if barrier == "other-block" {
            fd.bblocks_mut().new_block_basic(root)
        } else {
            block
        };
        fd.op_insert_end(reader, read_block);
        let p = Publication {
            start: (-32i64) as u64,
            pointee: Rc::new(Datatype::new(4, type_metatype::TYPE_INT)),
            receiver,
            width: 8,
        };
        assert_eq!(
            receiver_reaches(&fd, publication, reader, &p),
            barrier == "none",
            "{barrier}"
        );
    }
}

#[test]
fn locked_layout_refuses_before_preserving_a_containing_scalar() {
    let (mut fd, stack) = frame_data();
    let start = (-32i64) as u64;
    assert!(!locked_layout(&fd, start, 8));
    let scope = fd.get_scope_local_mut().unwrap();
    let symbol = scope
        .add_symbol(
            "locked_value",
            Rc::new(Datatype::new(8, type_metatype::TYPE_UINT)),
            &Address::new(stack, start),
            &Address::new_invalid(),
        )
        .unwrap();
    scope.set_attribute(symbol, crate::varnode::varnode_flags::typelock);
    assert!(locked_layout(&fd, start, 8));
    assert!(locked_layout(&fd, start + 4, 4));
    assert!(!locked_layout(&fd, start - 8, 8));
    assert!(!locked_layout(&fd, start + 8, 8));
    let op = store(&mut fd, 0x1000);
    fd.local_stack_views.writes.insert(op, (start, 8));
    assert_eq!(
        accesses_fit_early(&fd, start + 4, 4, true),
        Err(Refusal::LockedLayout)
    );
}

#[test]
fn nominal_contracts_cannot_bypass_or_reopen_a_refused_physical_footprint() {
    let (mut fd, _) = frame_data();
    let at = (-32i64) as u64;
    let views = ["Pair", "Handle"].map(|name| {
        let mut ty = Datatype::new(8, type_metatype::TYPE_STRUCT);
        ty.name = name.to_owned();
        Rc::new(ty)
    });
    fd.local_stack_views
        .candidates
        .insert((at, 8), views.to_vec());
    let op = store(&mut fd, 0x1000);
    fd.local_stack_views.writes.insert(op, (at, 8));
    fd.local_stack_views
        .admission
        .insert((at, 8), Err(Refusal::Crossing));
    promote(&mut fd, true);
    assert!(fd.local_stack_views.footprints.is_empty());
    fd.local_stack_views.admission.clear();
    promote(&mut fd, false);
    assert_eq!(
        fd.local_stack_views.admission[&(at, 8)],
        Err(Refusal::Unavailable)
    );
    promote(&mut fd, true);
    assert!(fd.local_stack_views.footprints.is_empty());
}

#[test]
fn nominal_extent_cannot_expand_to_a_containing_native_scalar() {
    let (mut fd, stack) = frame_data();
    let start = (-32i64) as u64;
    let op = store(&mut fd, 0x1000);
    let space = fd.new_constant(8, stack.get_index() as u64);
    fd.op_set_input(op, space, 0).unwrap();
    let pointer = fd.new_constant(8, start);
    fd.op_set_input(op, pointer, 1).unwrap();
    let value = fd.new_constant(8, 0);
    fd.op_set_input(op, value, 2).unwrap();
    record_native(&mut fd, op);
    fd.local_stack_views.writes.insert(op, (start, 8));
    assert_eq!(
        accesses_fit_early(&fd, start, 4, false),
        Err(Refusal::Crossing)
    );
    assert_eq!(accesses_fit_early(&fd, start, 4, true), Ok(()));
}

#[test]
fn physical_component_must_cover_all_overlapping_declared_candidates() {
    let (mut fd, stack) = frame_data();
    let start = (-32i64) as u64;
    fd.local_stack_views
        .candidates
        .insert((start, 8), Vec::new());
    fd.local_stack_views
        .candidates
        .insert((start + 4, 8), Vec::new());
    for at in [start, start + 4] {
        assert_eq!(
            accesses_fit_early(&fd, at, 8, false),
            Err(Refusal::Crossing)
        );
        fd.local_stack_views
            .admission
            .insert((at, 8), Err(Refusal::Unavailable));
    }
    fd.local_stack_views.candidates.remove(&(start + 4, 8));
    fd.local_stack_views
        .candidates
        .insert((start + 4, 4), Vec::new());
    assert_eq!(accesses_fit_early(&fd, start, 8, false), Ok(()));
    assert_eq!(
        accesses_fit_early(&fd, start + 4, 4, false),
        Err(Refusal::Crossing)
    );
    let op = store(&mut fd, 0x1000);
    let space = fd.new_constant(8, stack.get_index() as u64);
    fd.op_set_input(op, space, 0).unwrap();
    let pointer = fd.new_constant(8, start);
    fd.op_set_input(op, pointer, 1).unwrap();
    let value = fd.new_constant(8, 0);
    fd.op_set_input(op, value, 2).unwrap();
    record_native(&mut fd, op);
    fd.local_stack_views.writes.insert(op, (start, 8));
    assert_eq!(accesses_fit_early(&fd, start + 4, 4, true), Ok(()));
}

#[test]
fn duplicate_native_and_physical_records_do_not_invalidate_at_capacity() {
    let (mut fd, stack) = frame_data();
    let at = Address::new(stack, (-32i64) as u64);
    let mut last = None;
    for pc in 0..MAX_RECORDS {
        let op = store(&mut fd, 0x1000 + pc as u64);
        record_native(&mut fd, op);
        fd.local_stack_views.writes.insert(op, (at.get_offset(), 4));
        last = Some(op);
    }
    let last = last.unwrap();
    record_native(&mut fd, last);
    record_store(&mut fd, last, &at, 4);
    assert!(!fd.local_stack_views.invalid);
    assert_eq!(fd.local_stack_views.native.len(), MAX_RECORDS);
    assert_eq!(fd.local_stack_views.writes.len(), MAX_RECORDS);
}

#[test]
fn original_literal_store_receiver_survives_lowering_with_exact_width_and_space() {
    let (mut fd, stack) = frame_data();
    let ram = fd
        .get_arch()
        .manage()
        .get_default_data_space()
        .unwrap()
        .clone();
    let address = Address::new(ram.clone(), 0x2000);
    let op = store(&mut fd, 0x1000);
    let space = fd.new_constant(8, ram.get_index() as u64);
    fd.op_set_input(op, space, 0).unwrap();
    let destination = fd.new_constant(8, address.get_offset());
    fd.op_set_input(op, destination, 1).unwrap();
    let value = fd.new_constant(8, 0);
    fd.op_set_input(op, value, 2).unwrap();
    record_native(&mut fd, op);
    assert_eq!(native_receiver(&fd, op), Some((address.clone(), 8)));
    let partial_destination = fd.new_constant(4, address.get_offset());
    fd.op_set_input(op, partial_destination, 1).unwrap();
    assert!(native_receiver(&fd, op).is_none());
    let wrapped_destination = fd.new_constant(8, u64::MAX);
    fd.op_set_input(op, wrapped_destination, 1).unwrap();
    assert!(native_receiver(&fd, op).is_none());
    let destination = fd.new_constant(8, address.get_offset());
    fd.op_set_input(op, destination, 1).unwrap();
    let wrong_space = fd.new_constant(8, stack.get_index() as u64);
    fd.op_set_input(op, wrong_space, 0).unwrap();
    assert!(native_receiver(&fd, op).is_none());
    let space = fd.new_constant(8, ram.get_index() as u64);
    fd.op_set_input(op, space, 0).unwrap();
    let partial = fd.new_constant(4, 0);
    fd.op_set_input(op, partial, 2).unwrap();
    assert!(native_receiver(&fd, op).is_none());
    let value = fd.new_constant(8, 0);
    fd.op_set_input(op, value, 2).unwrap();
    fd.op_remove_input(op, 1);
    fd.op_remove_input(op, 0);
    fd.op_set_opcode_code(op, OpCode::CPUI_COPY);
    let output = fd.new_varnode_out(8, &address, op).unwrap();
    fd.vbank_mut()
        .get_mut(output)
        .unwrap()
        .set_flags_pub(crate::varnode::varnode_flags::persist);
    assert_eq!(native_receiver(&fd, op), Some((address, 8)));
}

#[test]
fn unrepresented_frame_publication_refuses_the_whole_candidate() {
    let (mut fd, _) = frame_data();
    let register = fd
        .get_arch()
        .manage()
        .get_space_by_name("register")
        .unwrap()
        .clone();
    let ram = fd
        .get_arch()
        .manage()
        .get_default_data_space()
        .unwrap()
        .clone();
    let sp = fd.new_varnode(8, &Address::new(register, 0), None);
    let sp = fd.set_input_varnode(sp).unwrap();
    fd.vbank_mut()
        .get_mut(sp)
        .unwrap()
        .set_flags_pub(crate::varnode::varnode_flags::spacebase);
    let add = fd.new_op(2, Address::new(ram.clone(), 0x1000));
    fd.op_set_opcode_code(add, OpCode::CPUI_INT_ADD);
    fd.op_set_input(add, sp, 0).unwrap();
    let offset = fd.new_constant(8, (-32i64) as u64);
    fd.op_set_input(add, offset, 1).unwrap();
    let pointer = fd.new_unique_out(8, add).unwrap();
    fd.obank_mut().mark_alive(add);
    assert!(crate::kuna_stackranges::frame_address(&fd, pointer).is_some());
    let op = store(&mut fd, 0x1001);
    let space = fd.new_constant(8, ram.get_index() as u64);
    fd.op_set_input(op, space, 0).unwrap();
    let destination = fd.new_constant(8, 0x2000);
    fd.op_set_input(op, destination, 1).unwrap();
    fd.op_set_input(op, pointer, 2).unwrap();
    record_native(&mut fd, op);
    assert!(native_receiver(&fd, op).is_some());
    collect(&mut fd);
    assert!(fd.local_stack_views.pending.is_empty());
    assert!(fd.local_stack_views.unrepresented_publication);
    assert_eq!(
        accesses_fit_early(&fd, (-32i64) as u64, 8, false),
        Err(Refusal::Unavailable)
    );
}

#[test]
fn native_marker_copy_requires_exact_live_original_write_inside_admitted_storage() {
    for mode in [
        "valid",
        "unadmitted",
        "partial",
        "clone",
        "origin",
        "space",
        "address",
        "width",
        "dead",
        "flag",
        "off",
        "full",
    ] {
        let (mut fd, stack) = frame_data_with_options(mode == "full", mode != "off");
        let at = Address::new(stack.clone(), (-32i64) as u64);
        let op = store(&mut fd, 0x1000);
        let point = SourcePoint::op(&fd, op).unwrap();
        fd.local_stack_views
            .native
            .insert(op, NativeWrite::Store(point, 4));
        fd.local_stack_views.writes.insert(op, (at.get_offset(), 4));
        if mode != "unadmitted" {
            let (start, width) = if mode == "partial" {
                (at.get_offset() + 2, 2)
            } else {
                (at.get_offset(), 4)
            };
            fd.local_stack_views
                .footprints
                .insert((start, width), Vec::new());
        }
        let copy = if mode == "clone" {
            store(&mut fd, 0x1000)
        } else {
            op
        };
        let space = fd.new_constant(8, stack.get_index() as u64);
        let destination = fd.new_constant(8, at.get_offset());
        fd.op_set_input(copy, space, 0).unwrap();
        fd.op_set_input(copy, destination, 1).unwrap();
        fd.op_remove_input(copy, 1);
        fd.op_remove_input(copy, 0);
        fd.op_set_opcode_code(copy, OpCode::CPUI_COPY);
        let output_address = match mode {
            "space" => Address::new(
                fd.get_arch()
                    .manage()
                    .get_default_data_space()
                    .unwrap()
                    .clone(),
                at.get_offset(),
            ),
            "address" => Address::new(stack.clone(), at.get_offset() + 4),
            _ => at.clone(),
        };
        let output = fd
            .new_varnode_out(if mode == "width" { 8 } else { 4 }, &output_address, copy)
            .unwrap();
        if mode != "flag" {
            fd.vbank_mut().get_mut(output).unwrap().set_stack_store();
        }
        if mode == "origin" {
            let other = store(&mut fd, 0x1004);
            fd.local_stack_views.native.insert(
                op,
                NativeWrite::Store(SourcePoint::op(&fd, other).unwrap(), 4),
            );
        }
        if mode == "dead" {
            fd.obank_mut().mark_dead(copy);
        }
        assert_eq!(native_store(&fd, output), mode == "valid", "{mode}");
    }
}
