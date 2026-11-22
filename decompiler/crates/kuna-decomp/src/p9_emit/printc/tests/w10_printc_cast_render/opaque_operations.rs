//! Functional CALLOTHER naming and argument preservation in Rust output.
use super::*;
use crate::userop::UserPcodeOp;

fn render(
    arch: &crate::architecture::Architecture,
    lang: &str,
    index: u64,
    names: &[&str],
) -> String {
    let mut fd = build_fd();
    let op = mk_op(
        &mut fd,
        (1 + names.len()) as int4,
        0x10,
        OpCode::CPUI_CALLOTHER,
    );
    let selector = fd.new_constant(8, index);
    fd.op_set_input(op, selector, 0).unwrap();
    let word = named("uint4", 4, type_metatype::TYPE_UINT);
    let mut operands = Vec::new();
    for (i, &name) in names.iter().enumerate() {
        let addr = Address::new(ram(&fd), 0x400 + i as u64 * 4);
        let operand = fd.new_varnode(4, &addr, Some(word.clone()));
        fd.op_set_input(op, operand, i as int4 + 1).unwrap();
        operands.push((operand, name));
    }
    fd.set_high_level();
    for (operand, name) in operands {
        let high = fd.vbank().get(operand).unwrap().get_high().unwrap();
        fd.high_bank_mut()
            .get_mut(high)
            .unwrap()
            .set_kuna_name(name);
    }
    let mut printer = PrintC::new();
    printer.set_name(lang);
    printer.rt_ctx.lang = printer.out_lang;
    printer.rt_ctx.enabled = true;
    printer.set_output_stream();
    printer.op_push_ir(&fd, arch, op, None);
    printer.emit_mut().output_str().to_owned()
}

#[test]
fn dotted_and_unregistered_operations_keep_identifiers_and_operands() {
    let mut arch = bare_arch();
    arch.userops
        .register_op(UserPcodeOp::new_unspecialized(b"DPA.W.PH", 0))
        .unwrap();
    arch.userops
        .register_op(UserPcodeOp::new_unspecialized(b"DPA_W.PH", 1))
        .unwrap();
    arch.userops
        .register_op(UserPcodeOp::new_unspecialized(
            b"__kuna_userop_0_DPA_W_PH",
            2,
        ))
        .unwrap();
    let args = ["left", "right"];
    assert_eq!(
        render(&arch, "rust", 0, &args),
        "__kuna_userop_0_DPA_W_PH(left,right)"
    );
    assert_eq!(
        render(&arch, "rust", 1, &args),
        "__kuna_userop_1_DPA_W_PH(left,right)"
    );
    assert_eq!(
        render(&arch, "rust", 2, &[]),
        "__kuna_userop_2___kuna_userop_0_DPA_W_PH()"
    );
    assert_eq!(render(&arch, "rust", 3, &[]), "__kuna_userop_3()");
    assert_eq!(
        render(&arch, "rust", 1 << 32, &[]),
        "__kuna_userop_100000000()"
    );
    for lang in ["c", "go"] {
        assert_eq!(render(&arch, lang, 0, &args), "DPA.W.PH(left,right)");
        assert_eq!(render(&arch, lang, 3, &[]), "CALLOTHER[0x3]()");
    }
}

#[test]
fn unicode_operations_keep_distinct_bindings_and_all_four_operands() {
    let mut arch = bare_arch();
    for (index, name) in ["é", "e\u{301}", "__kuna_userop_0__"].iter().enumerate() {
        arch.userops
            .register_op(UserPcodeOp::new_unspecialized(name.as_bytes(), index as int4))
            .unwrap();
    }
    let args = ["first", "second", "third", "fourth"];
    assert_eq!(
        render(&arch, "rust", 0, &args),
        "__kuna_userop_0__(first,second,third,fourth)"
    );
    assert_eq!(
        render(&arch, "rust", 1, &args),
        "__kuna_userop_1_e_(first,second,third,fourth)"
    );
    assert_eq!(
        render(&arch, "rust", 2, &args),
        "__kuna_userop_2___kuna_userop_0__(first,second,third,fourth)"
    );
    for lang in ["c", "go"] {
        assert_eq!(render(&arch, lang, 0, &args), "é(first,second,third,fourth)");
        assert_eq!(render(&arch, lang, 1, &args), "e\u{301}(first,second,third,fourth)");
    }
}

#[test]
fn acle_intrinsics_keep_precedence_for_a_resolved_userop() {
    let mut arch = bare_arch();
    arch.archid = "AARCH64:LE:64:v8A:default".into();
    arch.acle_intrinsics = true;
    arch.userops
        .register_op(UserPcodeOp::new_unspecialized(b"WaitForInterrupt", 0))
        .unwrap();
    for lang in ["c", "go", "rust"] {
        assert_eq!(render(&arch, lang, 0, &[]), "__wfi()");
    }
    assert_eq!(
        render(&arch, "rust", 1 << 32, &[]),
        "__kuna_userop_100000000()"
    );
    arch.acle_intrinsics = false;
    for lang in ["c", "go", "rust"] {
        assert_eq!(render(&arch, lang, 0, &[]), "WaitForInterrupt()");
    }
}
