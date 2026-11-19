//! Calls retain the low result bytes of their emitted callee's wider integer.
use super::*;
use crate::kuna_callrettype::StatedReturn;

fn call(source: Rc<Datatype>, target: Rc<Datatype>, withdrawn: bool) -> (Funcdata, super::super::OpId) {
    let mut fd = build_fd();
    let op = mk_op(&mut fd, 1, 0x20, OpCode::CPUI_CALL);
    let entry = Address::new(ram(&fd), 0x2000);
    let callee = fd.new_varnode(8, &entry, None);
    fd.op_set_input(op, callee, 0).unwrap();
    let mut specs = crate::fspec::FuncCallSpecs::new(op, entry.clone());
    specs.set_funcdata(entry, "wide_value").unwrap();
    fd.push_call_specs(specs);
    let storage = Address::new(ram(&fd), 0x400);
    let output = fd.new_varnode(target.get_size(), &storage, Some(target));
    fd.op_set_output(op, output).unwrap();
    let key = (ram(&fd).get_index(), 0x2000);
    let statement = Rc::new(StatedReturn { addr: storage, size: source.get_size(), ct: source });
    if withdrawn { fd.kuna_set_callret_withdrawn(key, statement); }
    else { fd.kuna_set_callret_type(key, statement); }
    (fd, op)
}

fn render(fd: &Funcdata, op: super::super::OpId, lang: &str, nocasts: bool) -> String {
    let arch = bare_arch();
    let types = arch.types_impl();
    types.set_max_basetype_size(8);
    types.set_default_alignment_map();
    types.set_size_of_int(4);
    types.set_size_of_float(4);
    types.set_size_of_long_long(8);
    types.set_size_of_double(8);
    types.set_core_type("uint1", 1, type_metatype::TYPE_UINT, false).unwrap();
    types.cache_core_types().unwrap();
    let mut printer = PrintC::new();
    printer.set_name(lang);
    printer.rt_ctx.lang = printer.out_lang;
    printer.rt_ctx.enabled = true;
    printer.rt_ctx.long_is_8 = true;
    printer.rt_ctx.ctypes = true;
    printer.set_no_cast_printing(nocasts);
    printer.set_output_stream();
    printer.op_push_ir(fd, &arch, op, None);
    printer.emit_mut().output_str().to_string()
}

fn retarget(fd: &mut Funcdata, op: super::super::OpId, entry: &Address) {
    let mut specs = crate::fspec::FuncCallSpecs::new(op, entry.clone());
    specs.set_funcdata(entry.clone(), "wide_value").unwrap();
    *fd.get_call_specs_mut(0) = specs;
    let target = fd.new_varnode(8, entry, None);
    fd.op_set_input(op, target, 0).unwrap();
}

#[test]
fn recovered_call_results_narrow_in_go_and_rust() {
    for withdrawn in [false, true] {
        for width in [1, 2, 4] {
            for meta in [type_metatype::TYPE_UINT, type_metatype::TYPE_INT] {
                let (fd, op) = call(named("uint8", 8, type_metatype::TYPE_UINT),
                    named("target", width, meta), withdrawn);
                let signed = meta == type_metatype::TYPE_INT;
                assert_eq!(render(&fd, op, "rust", false),
                    format!("wide_value() as {}{}", if signed { "i" } else { "u" }, width * 8));
                let go = if width == 1 && !signed { "byte".into() }
                    else { format!("{}int{}", if signed { "" } else { "u" }, width * 8) };
                assert_eq!(render(&fd, op, "go", false), format!("{go}(wide_value())"));
                assert_eq!(render(&fd, op, "c", false), "wide_value()");
                assert_eq!(render(&fd, op, "rust", true), "wide_value()");
            }
        }
    }
}

#[test]
fn boolean_call_results_narrow_before_the_zero_test() {
    let (fd, op) = call(named("uint8", 8, type_metatype::TYPE_UINT),
        named("bool", 1, type_metatype::TYPE_BOOL), false);
    assert_eq!(render(&fd, op, "rust", false), "wide_value() as u8 != 0");
    assert_eq!(render(&fd, op, "go", false),
        "func(value byte) bool { return value != 0 }(byte(wide_value()))");
}

#[test]
fn implied_byte_call_keeps_its_conversion_in_boolean_consumers() {
    let (mut fd, op) = call(named("uint8", 8, type_metatype::TYPE_UINT),
        named("uint1", 1, type_metatype::TYPE_UINT), false);
    let output = fd.obank().get(op).unwrap().get_out().unwrap();
    fd.vbank_mut().get_mut(output).unwrap().set_implied();
    let cast = mk_op(&mut fd, 1, 0x30, OpCode::CPUI_CAST);
    fd.op_set_input(cast, output, 0).unwrap();
    let boolean = fd.new_varnode(1, &Address::new(ram(&fd), 0x410),
        Some(named("bool", 1, type_metatype::TYPE_BOOL)));
    fd.op_set_output(cast, boolean).unwrap();
    assert_eq!(render(&fd, cast, "rust", false), "wide_value() as u8 != 0");
}

#[test]
fn noninteger_returns_and_unchanged_or_wider_storage_are_not_narrowed() {
    for meta in [type_metatype::TYPE_FLOAT, type_metatype::TYPE_PTR,
        type_metatype::TYPE_STRUCT, type_metatype::TYPE_BOOL]
    {
        let (fd, op) = call(named("source", 8, meta), named("uint1", 1, type_metatype::TYPE_UINT), false);
        assert_eq!(render(&fd, op, "rust", false), "wide_value()");
    }
    for (source, target) in [(8, 8), (4, 8), (16, 1)] {
        let (fd, op) = call(named("source", source, type_metatype::TYPE_UINT),
            named("target", target, type_metatype::TYPE_UINT), false);
        assert_eq!(render(&fd, op, "rust", false), "wide_value()");
    }
    let mut enumeration = Datatype::new_with_align(8, 8, type_metatype::TYPE_UINT);
    enumeration.flags |= crate::dtype::flags::enumtype;
    let (fd, op) = call(Rc::new(enumeration), named("uint1", 1, type_metatype::TYPE_UINT), false);
    assert_eq!(render(&fd, op, "rust", false), "wide_value()");
}

#[test]
fn inconsistent_types_and_recursive_statements_are_ignored() {
    let source = named("uint8", 8, type_metatype::TYPE_UINT);
    let target = named("uint1", 1, type_metatype::TYPE_UINT);
    let (mut fd, op) = call(source.clone(), target.clone(), false);
    let key = (ram(&fd).get_index(), 0x2000);
    fd.kuna_set_callret_type(key, Rc::new(StatedReturn {
        addr: Address::new(ram(&fd), 0x400), size: 4, ct: source.clone(),
    }));
    assert_eq!(render(&fd, op, "rust", false), "wide_value()");

    let (mut fd, op) = call(source.clone(), target.clone(), false);
    let output = fd.obank().get(op).unwrap().get_out().unwrap();
    fd.vbank_mut().get_mut(output).unwrap().update_type(source.clone());
    assert_eq!(render(&fd, op, "rust", false), "wide_value()");

    let (mut fd, op) = call(source.clone(), target, false);
    let entry = fd.get_address().clone();
    let key = (entry.get_space().unwrap().get_index(), entry.get_offset());
    retarget(&mut fd, op, &entry);
    fd.kuna_set_callret_type(key, Rc::new(StatedReturn {
        addr: Address::new(ram(&fd), 0x400), size: 8, ct: source,
    }));
    assert_eq!(render(&fd, op, "rust", false), "wide_value()");
}

#[test]
fn explicit_and_aggregate_prototypes_keep_their_own_conversion_paths() {
    for (meta, locked) in [(type_metatype::TYPE_UINT, true),
        (type_metatype::TYPE_STRUCT, false), (type_metatype::TYPE_UINT, false)]
    {
        let (mut fd, op) = call(named("uint8", 8, type_metatype::TYPE_UINT),
            named("uint1", 1, type_metatype::TYPE_UINT), false);
        let model = Rc::new(crate::fspec::ProtoModel::new(fd.get_arch().manage()));
        let storage = Address::new(ram(&fd), 0x400);
        let proto = fd.get_call_specs_mut(0).proto_mut();
        proto.set_internal(model, named("void", 0, type_metatype::TYPE_VOID));
        proto.set_output(&crate::fspec::ParameterPieces {
            addr: storage, type_: Some(named("result", if locked { 8 } else { 4 }, meta)), flags: 0,
        });
        proto.set_output_lock(locked);
        assert_eq!(render(&fd, op, "rust", false), "wide_value()");
        assert_eq!(render(&fd, op, "go", false), "wide_value()");
    }
}

#[test]
fn computed_untyped_returns_use_the_parameter_recovery_snapshot() {
    let (mut fd, op) = call(named("uint8", 8, type_metatype::TYPE_UINT),
        named("uint1", 1, type_metatype::TYPE_UINT), false);
    let entry = Address::new(ram(&fd), 0x3000);
    retarget(&mut fd, op, &entry);
    let storage = Address::new(ram(&fd), 0x400);
    let mut recovered = crate::kuna_protoorder::RecoveredTypes {
        inputs: Vec::new(), arity_sound: false, output: None,
        result: Some((storage.clone(), 8)), vararg_tail: Vec::new(),
        reads: Vec::new(), passes: Vec::new(), callers_vote: false,
    };
    fd.kuna_set_protoorder_types(&entry, Rc::new(recovered.clone()));
    assert_eq!(render(&fd, op, "rust", false), "wide_value()");
    recovered.output = Some((storage, 8, named("unknown8", 8, type_metatype::TYPE_UNKNOWN)));
    fd.kuna_set_protoorder_types(&entry, Rc::new(recovered));
    assert_eq!(render(&fd, op, "rust", false), "wide_value() as u8");
    assert_eq!(render(&fd, op, "go", false), "byte(wide_value())");
}
