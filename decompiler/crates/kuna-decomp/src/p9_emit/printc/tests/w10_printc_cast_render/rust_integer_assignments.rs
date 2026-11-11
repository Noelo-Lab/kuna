//! Rust integer assignment conversions and their deliberate exclusions.
use super::*;

fn copy_expression(source: Rc<Datatype>, target: Rc<Datatype>, nocasts: bool) -> String {
    let mut fd = build_fd();
    let input = fd.new_varnode(source.get_size(), &Address::new(ram(&fd), 0x400), Some(source));
    let output = fd.new_varnode(target.get_size(), &Address::new(ram(&fd), 0x410), Some(target));
    let op = mk_op(&mut fd, 1, 0x20, OpCode::CPUI_COPY);
    fd.op_set_input(op, input, 0).unwrap();
    fd.op_set_output(op, output).unwrap();
    fd.set_high_level();
    let high = fd.vbank().get(input).unwrap().get_high().unwrap();
    fd.high_bank_mut().get_mut(high).unwrap().set_kuna_name("value");
    let mut printer = PrintC::new();
    printer.set_name("rust");
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
fn copy_conversions_follow_primitive_spelling_and_cast_suppression() {
    for width in [1, 2, 4, 8] {
        let signed = named("signed_word", width, type_metatype::TYPE_INT);
        let unsigned = named("unsigned_word", width, type_metatype::TYPE_UINT);
        assert_eq!(copy_expression(signed.clone(), unsigned.clone(), false), format!("value as u{}", width * 8));
        assert_eq!(copy_expression(unsigned.clone(), signed.clone(), false), format!("value as i{}", width * 8));
        assert_eq!(copy_expression(signed.clone(), unsigned.clone(), true), "value");
        assert_eq!(copy_expression(unsigned.clone(), signed.clone(), true), "value");
        assert_eq!(copy_expression(signed.clone(), signed, false), "value");
        assert_eq!(copy_expression(unsigned.clone(), unsigned, false), "value");
    }
    let mut text = Datatype::new_with_align(1, 1, type_metatype::TYPE_INT);
    text.flags |= crate::dtype::flags::chartype | crate::dtype::flags::coretype;
    assert_eq!(copy_expression(Rc::new(text), named("signed_byte", 1, type_metatype::TYPE_INT), false), "value as i8");
}

#[test]
fn unsupported_assignment_carriers_do_not_gain_integer_casts() {
    for meta in [type_metatype::TYPE_STRUCT, type_metatype::TYPE_ARRAY, type_metatype::TYPE_BOOL] {
        let source = named("Carrier", 8, meta);
        let target = named("unsigned_word", 8, type_metatype::TYPE_UINT);
        assert_eq!(copy_expression(source, target, false), "value");
    }
    let mut enumeration = Datatype::new_with_align(8, 8, type_metatype::TYPE_INT);
    enumeration.flags |= crate::dtype::flags::enumtype;
    enumeration.name = "Choice".into();
    enumeration.display_name = "Choice".into();
    assert_eq!(copy_expression(Rc::new(enumeration), named("unsigned_word", 8, type_metatype::TYPE_UINT), false), "value");
    for (source, target) in [(4, 8), (8, 4), (16, 16)] {
        assert_eq!(copy_expression(named("signed_word", source, type_metatype::TYPE_INT), named("unsigned_word", target, type_metatype::TYPE_UINT), false), "value");
    }
}
