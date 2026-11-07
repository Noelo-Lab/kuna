//! Typed call boundaries preserve zero extensions removed by subvariable flow.
use super::*;

fn argument(source: Rc<Datatype>, target: Rc<Datatype>, zext: bool) -> (Funcdata, super::super::OpId) {
    let mut fd = build_fd();
    let operation = mk_op(&mut fd, 2, 0x20, OpCode::CPUI_CALL);
    let entry = Address::new(ram(&fd), 0x2000);
    let callee = fd.new_varnode(8, &entry, None);
    fd.op_set_input(operation, callee, 0).unwrap();
    let width = source.get_size();
    let input = fd.new_varnode(width, &Address::new(ram(&fd), 0x500), Some(source));
    fd.op_set_input(operation, input, 1).unwrap();
    fd.set_high_level();
    let high = fd.vbank().get(input).unwrap().get_high().unwrap();
    fd.high_bank_mut().get_mut(high).unwrap().set_kuna_name("value");
    let storage = Address::new(ram(&fd), 0x400);
    let mut call = crate::fspec::FuncCallSpecs::new(operation, entry.clone());
    call.set_funcdata(entry.clone(), "consume").unwrap();
    call.set_final_input_storage(vec![(storage.clone(), 8)]);
    call.note_trimmed_input(1, width, 2, zext);
    fd.push_call_specs(call);
    let recovered = crate::kuna_protoorder::RecoveredTypes {
        inputs: vec![(storage, target.get_size(), target)], arity_sound: false,
        output: None, result: None, vararg_tail: Vec::new(), reads: Vec::new(), passes: Vec::new(),
    };
    fd.kuna_set_protoorder_types(&entry, Rc::new(recovered));
    (fd, operation)
}

fn render(fd: &Funcdata, operation: super::super::OpId, language: &str, nocasts: bool) -> String {
    let arch = bare_arch();
    let types = arch.types_impl();
    types.set_max_basetype_size(8);
    types.set_default_alignment_map();
    types.set_size_of_int(4);
    types.set_core_type("uint1", 1, type_metatype::TYPE_UINT, false).unwrap();
    types.cache_core_types().unwrap();
    let mut printer = PrintC::new();
    printer.set_name(language);
    printer.rt_ctx.lang = printer.out_lang;
    printer.rt_ctx.enabled = true;
    printer.rt_ctx.long_is_8 = true;
    printer.rt_ctx.ctypes = true;
    printer.set_no_cast_printing(nocasts);
    printer.set_output_stream();
    printer.op_push_ir(fd, &arch, operation, None);
    printer.emit_mut().output_str().to_string()
}

#[test]
fn trimmed_values_zero_extend_before_entering_wider_parameters() {
    for input in [1, 2, 4] {
        for output in [2, 4, 8].into_iter().filter(|size| *size > input) {
            for source_signed in [false, true] {
                for target_signed in [false, true] {
                    let meta = |signed| if signed { type_metatype::TYPE_INT } else { type_metatype::TYPE_UINT };
                    let (fd, op) = argument(named("input", input, meta(source_signed)),
                        named("output", output, meta(target_signed)), true);
                    assert_eq!(render(&fd, op, "rust", false), format!("consume((value as u{}) as {}{})",
                        input * 8, if target_signed { "i" } else { "u" }, output * 8));
                    let narrow = if input == 1 { "byte".into() } else { format!("uint{}", input * 8) };
                    assert_eq!(render(&fd, op, "go", false), format!("consume({}int{}({narrow}(value)))",
                        if target_signed { "" } else { "u" }, output * 8));
                    assert_eq!(render(&fd, op, "c", false), "consume(value)");
                    assert_eq!(render(&fd, op, "rust", true), "consume(value)");
                }
            }
        }
    }
}

#[test]
fn missing_or_stale_zero_extension_evidence_does_not_change_arguments() {
    let input = named("input", 4, type_metatype::TYPE_INT);
    let output = named("output", 8, type_metatype::TYPE_UINT);
    for mutation in 0..6 {
        let (mut fd, op) = argument(input.clone(), output.clone(), true);
        let address = Address::new(ram(&fd), if mutation == 4 { 0x408 } else { 0x400 });
        let call = fd.get_call_specs_mut(0);
        match mutation {
            0 => call.note_trimmed_input(1, 4, 2, false),
            1 => call.note_trimmed_input(1, 2, 2, true),
            2 => call.note_trimmed_input(1, 4, 3, true),
            3 => call.set_final_input_storage(Vec::new()),
            4 => call.set_final_input_storage(vec![(address, 8)]),
            5 => call.set_final_input_storage(vec![(address, 2)]),
            _ => unreachable!(),
        }
        assert_eq!(render(&fd, op, "go", false), "consume(value)");
        assert_eq!(render(&fd, op, "rust", false), "consume(value)");
    }
}

#[test]
fn noninteger_and_nonwidening_contracts_keep_their_existing_paths() {
    for meta in [type_metatype::TYPE_FLOAT, type_metatype::TYPE_PTR,
        type_metatype::TYPE_STRUCT, type_metatype::TYPE_BOOL]
    {
        let (fd, op) = argument(named("input", 4, type_metatype::TYPE_UINT), named("output", 8, meta), true);
        assert_eq!(render(&fd, op, "rust", false), "consume(value)");
        let (fd, op) = argument(named("input", 4, meta), named("output", 8, type_metatype::TYPE_UINT), true);
        assert_eq!(render(&fd, op, "rust", false), "consume(value)");
    }
    for (input, output) in [(4, 4), (4, 2), (8, 8), (8, 16)] {
        let (fd, op) = argument(named("input", input, type_metatype::TYPE_UINT), named("output", output, type_metatype::TYPE_UINT), true);
        assert_eq!(render(&fd, op, "rust", false), "consume(value)");
    }
}

#[test]
fn constants_use_their_retained_bits_without_a_negative_unsigned_literal() {
    for input in [1, 2, 4] {
        let (mut fd, op) = argument(named("input", input, type_metatype::TYPE_INT),
            named("output", 8, type_metatype::TYPE_UINT), true);
        let value = fd.new_constant(input, kuna_base::address::calc_mask(input));
        fd.op_set_input(op, value, 1).unwrap();
        for lang in ["go", "rust"] {
            assert_eq!(render(&fd, op, lang, false), format!("consume({})", kuna_base::address::calc_mask(input)));
        }
    }
}

#[test]
fn declared_prototypes_take_precedence_over_recovered_parameter_snapshots() {
    let (mut fd, op) = argument(named("input", 4, type_metatype::TYPE_UINT),
        named("output", 8, type_metatype::TYPE_UINT), true);
    let model = Rc::new(crate::fspec::ProtoModel::new(fd.get_arch().manage()));
    let address = Address::new(ram(&fd), 0x400);
    let proto = fd.get_call_specs_mut(0).proto_mut();
    proto.set_internal(model, named("void", 0, type_metatype::TYPE_VOID));
    proto.set_param(0, "value", &crate::fspec::ParameterPieces {
        addr: address, type_: Some(named("uint4", 4, type_metatype::TYPE_UINT)), flags: 0,
    });
    proto.set_input_lock(true);
    for lang in ["go", "rust"] { assert_eq!(render(&fd, op, lang, false), "consume(value)"); }
}

#[test]
fn same_width_arguments_match_recovered_signedness_without_trim_evidence() {
    for size in [1, 2, 4, 8] {
        for signed in [false, true] {
            let source = if signed { type_metatype::TYPE_UINT } else { type_metatype::TYPE_INT };
            let target = if signed { type_metatype::TYPE_INT } else { type_metatype::TYPE_UINT };
            let (fd, op) = argument(named("input", size, source), named("output", size, target), false);
            assert_eq!(render(&fd, op, "rust", false), format!("consume(value as {}{})",
                if signed { "i" } else { "u" }, size * 8));
            let ty = if size == 1 && !signed { "byte".into() }
                else { format!("{}int{}", if signed { "" } else { "u" }, size * 8) };
            assert_eq!(render(&fd, op, "go", false), format!("consume({ty}(value))"));
            assert_eq!(render(&fd, op, "c", false), "consume(value)");
            assert_eq!(render(&fd, op, "rust", true), "consume(value)");
        }
    }
}

#[test]
fn declared_same_width_signedness_uses_the_declared_parameter() {
    let (mut fd, op) = argument(named("input", 4, type_metatype::TYPE_INT),
        named("output", 8, type_metatype::TYPE_INT), true);
    let model = Rc::new(crate::fspec::ProtoModel::new(fd.get_arch().manage()));
    let address = Address::new(ram(&fd), 0x400);
    let proto = fd.get_call_specs_mut(0).proto_mut();
    proto.set_internal(model, named("void", 0, type_metatype::TYPE_VOID));
    proto.set_param(0, "value", &crate::fspec::ParameterPieces {
        addr: address, type_: Some(named("uint4", 4, type_metatype::TYPE_UINT)), flags: 0,
    });
    proto.set_input_lock(true);
    assert_eq!(render(&fd, op, "rust", false), "consume(value as u32)");
    assert_eq!(render(&fd, op, "go", false), "consume(uint32(value))");
    assert_eq!(render(&fd, op, "c", false), "consume(value)");
}

#[test]
fn indirect_and_recursive_calls_do_not_reuse_recovered_contracts() {
    for indirect in [false, true] {
        let (mut fd, op) = argument(named("input", 4, type_metatype::TYPE_INT),
            named("output", 8, type_metatype::TYPE_UINT), true);
        if indirect {
            fd.op_set_opcode(op, TypeOp::new(OpCode::CPUI_CALLIND, 0, "CALLIND"));
        } else {
            let entry = fd.get_address().clone();
            let previous = fd.get_call_specs(0).get_entry_address().clone();
            let recovered = fd.kuna_protoorder_types(&previous).unwrap().clone();
            fd.kuna_set_protoorder_types(&entry, Rc::new(recovered));
            let storage = fd.get_call_specs(0).final_input_storage().to_vec();
            let mut call = crate::fspec::FuncCallSpecs::new(op, entry.clone());
            call.set_funcdata(entry, "consume").unwrap();
            call.set_final_input_storage(storage);
            call.note_trimmed_input(1, 4, 2, true);
            *fd.get_call_specs_mut(0) = call;
        }
        let input = fd.obank().get(op).unwrap().get_in(1).unwrap();
        for lang in ["rust", "go"] {
            let mut printer = PrintC::new();
            printer.set_name(lang);
            assert!(!printer.push_extended_call_argument(&fd, &bare_arch(), input, op, 1));
        }
    }
}

#[test]
fn overflowing_recovered_storage_ranges_are_not_integer_contracts() {
    let (mut fd, op) = argument(named("input", 4, type_metatype::TYPE_INT),
        named("output", 8, type_metatype::TYPE_UINT), true);
    let entry = fd.get_call_specs(0).get_entry_address().clone();
    let mut recovered = fd.kuna_protoorder_types(&entry).unwrap().clone();
    let storage = Address::new(ram(&fd), u64::MAX - 3);
    recovered.inputs[0].0 = storage.clone();
    fd.kuna_set_protoorder_types(&entry, Rc::new(recovered));
    fd.get_call_specs_mut(0).set_final_input_storage(vec![(storage, 8)]);
    for lang in ["go", "rust"] { assert_eq!(render(&fd, op, lang, false), "consume(value)"); }
}

#[test]
fn matching_recovered_type_overrides_an_unlocked_callsite_guess() {
    let (mut fd, op) = argument(named("input", 4, type_metatype::TYPE_UINT),
        named("output", 4, type_metatype::TYPE_UINT), false);
    let model = Rc::new(crate::fspec::ProtoModel::new(fd.get_arch().manage()));
    let address = Address::new(ram(&fd), 0x400);
    let proto = fd.get_call_specs_mut(0).proto_mut();
    proto.set_internal(model, named("void", 0, type_metatype::TYPE_VOID));
    proto.set_param(0, "value", &crate::fspec::ParameterPieces {
        addr: address, type_: Some(named("int4", 4, type_metatype::TYPE_INT)), flags: 0,
    });
    for lang in ["go", "rust", "c"] {
        assert_eq!(render(&fd, op, lang, false), "consume(value)");
    }
}
