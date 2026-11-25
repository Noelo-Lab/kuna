use super::*;

fn render_wide_add(lang: &str, nocasts: bool) -> String {
    let mut fd = build_fd();
    let signed = named("signed_wide", 16, type_metatype::TYPE_INT);
    let unsigned = named("unsigned_wide", 16, type_metatype::TYPE_UINT);
    let space = ram(&fd);
    let a = fd.new_varnode(16, &Address::new(space.clone(), 0x400), Some(signed));
    let b = fd.new_varnode(
        16,
        &Address::new(space.clone(), 0x410),
        Some(unsigned.clone()),
    );
    let out = fd.new_varnode(16, &Address::new(space, 0x420), Some(unsigned));
    let add = mk_op(&mut fd, 2, 0x20, OpCode::CPUI_INT_ADD);
    fd.op_set_input(add, a, 0).unwrap();
    fd.op_set_input(add, b, 1).unwrap();
    fd.op_set_output(add, out).unwrap();
    fd.set_high_level();
    for (id, name) in [(a, "a"), (b, "b")] {
        let high = fd.vbank().get(id).unwrap().get_high().unwrap();
        fd.high_bank_mut()
            .get_mut(high)
            .unwrap()
            .set_kuna_name(name);
    }
    let mut printer = PrintC::new();
    printer.set_name(lang);
    printer.rt_ctx.lang = printer.out_lang;
    printer.rt_ctx.enabled = true;
    printer.rt_ctx.long_is_8 = true;
    printer.rt_ctx.ctypes = true;
    printer.set_no_cast_printing(nocasts);
    printer.set_output_stream();
    printer.op_push_ir(&fd, &bare_arch(), add, None);
    printer.emit_mut().output_str().to_string()
}

#[test]
fn wide_rust_wrapping_arithmetic_retains_width_and_operand_order() {
    assert_eq!(
        render_wide_add("rust", false),
        "u128::wrapping_add(a as u128,b)"
    );
    assert_eq!(render_wide_add("rust", true), "a + b");
    assert_eq!(render_wide_add("c", false), "a + b");
}
