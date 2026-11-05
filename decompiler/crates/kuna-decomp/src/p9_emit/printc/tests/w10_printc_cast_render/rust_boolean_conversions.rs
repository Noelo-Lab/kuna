//! Boolean consumers use the printed byte value without an invalid Rust cast.
use super::*;

fn expression(meta: type_metatype, opcode: OpCode, lang: &str, nocasts: bool) -> String {
    let mut fd = build_fd();
    let source = named("byte_value", 1, meta);
    let target = named("bool", 1, type_metatype::TYPE_BOOL);
    let value = fd.new_varnode(1, &Address::new(ram(&fd), 0x400), Some(source));
    let output = fd.new_varnode(1, &Address::new(ram(&fd), 0x410), Some(target));
    let op = mk_op(&mut fd, 1, 0x20, opcode);
    fd.op_set_input(op, value, 0).unwrap();
    fd.op_set_output(op, output).unwrap();
    fd.set_high_level();
    let high = fd.vbank().get(value).unwrap().get_high().unwrap();
    fd.high_bank_mut().get_mut(high).unwrap().set_kuna_name("value");
    let mut printer = PrintC::new();
    printer.set_name(lang);
    printer.rt_ctx.lang = printer.out_lang;
    printer.rt_ctx.enabled = true;
    printer.rt_ctx.long_is_8 = true;
    printer.rt_ctx.ctypes = true;
    printer.set_no_cast_printing(nocasts);
    printer.set_output_stream();
    printer.op_push_ir(&fd, &bare_arch(), op, None);
    printer.emit_mut().output_str().to_string()
}

#[test]
fn rust_byte_to_boolean_casts_and_assignments_compare_with_zero() {
    for meta in [type_metatype::TYPE_INT, type_metatype::TYPE_UINT, type_metatype::TYPE_UNKNOWN] {
        for opcode in [OpCode::CPUI_CAST, OpCode::CPUI_COPY] {
            assert_eq!(expression(meta, opcode, "rust", false), "value != 0");
            assert_eq!(expression(meta, opcode, "rust", true), "value");
        }
    }
}

#[test]
fn boolean_values_do_not_gain_integer_comparisons() {
    assert_eq!(expression(type_metatype::TYPE_BOOL, OpCode::CPUI_COPY, "rust", false), "value");
    assert_eq!(expression(type_metatype::TYPE_UINT, OpCode::CPUI_CAST, "c", false), "(bool)value");
}

#[test]
fn literal_complements_keep_their_byte_context() {
    let mut fd = build_fd();
    let invert = mk_op(&mut fd, 1, 0x10, OpCode::CPUI_INT_NEGATE);
    let literal = fd.new_constant(1, 0xff);
    fd.op_set_input(invert, literal, 0).unwrap();
    let byte = fd.new_varnode(1, &Address::new(ram(&fd), 0x400),
        Some(named("uint1", 1, type_metatype::TYPE_UINT)));
    fd.op_set_output(invert, byte).unwrap();
    let byte = fd.obank().get(invert).unwrap().get_out().unwrap();
    fd.vbank_mut().get_mut(byte).unwrap().set_implied();
    let cast = mk_op(&mut fd, 1, 0x20, OpCode::CPUI_CAST);
    fd.op_set_input(cast, byte, 0).unwrap();
    let output = fd.new_varnode(1, &Address::new(ram(&fd), 0x410),
        Some(named("bool", 1, type_metatype::TYPE_BOOL)));
    fd.op_set_output(cast, output).unwrap();
    let mut printer = PrintC::new();
    printer.set_name("rust");
    printer.rt_ctx.lang = printer.out_lang;
    printer.rt_ctx.enabled = true;
    printer.rt_ctx.ctypes = true;
    printer.set_output_stream();
    printer.op_push_ir(&fd, &bare_arch(), cast, None);
    assert_eq!(printer.emit_mut().output_str(), "::core::convert::identity::<u8>(!0xff) != 0");
}

#[test]
fn boolean_truncation_keeps_the_low_byte_before_comparison() {
    let mut fd = build_fd();
    let source = fd.new_varnode(8, &Address::new(ram(&fd), 0x400),
        Some(named("uint8", 8, type_metatype::TYPE_UINT)));
    let output = fd.new_varnode(1, &Address::new(ram(&fd), 0x410),
        Some(named("bool", 1, type_metatype::TYPE_BOOL)));
    let op = mk_op(&mut fd, 2, 0x20, OpCode::CPUI_SUBPIECE);
    fd.op_set_input(op, source, 0).unwrap();
    let offset = fd.new_constant(4, 0);
    fd.op_set_input(op, offset, 1).unwrap();
    fd.op_set_output(op, output).unwrap();
    fd.set_high_level();
    let high = fd.vbank().get(source).unwrap().get_high().unwrap();
    fd.high_bank_mut().get_mut(high).unwrap().set_kuna_name("value");
    let mut arch = bare_arch();
    arch.bool_byte = true;
    let types = arch.types_impl();
    types.set_max_basetype_size(8);
    types.set_default_alignment_map();
    types.set_size_of_int(4);
    types.set_size_of_float(4);
    types.set_size_of_long_long(8);
    types.set_size_of_double(8);
    types.set_core_type("uint1", 1, type_metatype::TYPE_UINT, false).unwrap();
    types.cache_core_types().unwrap();
    for mask in [0xff, u64::MAX] {
        fd.vbank_mut().get_mut(source).unwrap().set_nz_mask(mask);
        let mut printer = PrintC::new();
        printer.set_name("rust");
        printer.rt_ctx.lang = printer.out_lang;
        printer.rt_ctx.enabled = true;
        printer.rt_ctx.ctypes = true;
        printer.set_output_stream();
        printer.op_push_ir(&fd, &arch, op, None);
        assert_eq!(printer.emit_mut().output_str(), "value as u8 != 0");
    }
}
