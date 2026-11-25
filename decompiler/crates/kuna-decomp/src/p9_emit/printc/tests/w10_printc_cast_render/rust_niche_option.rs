use super::*;
use crate::dtype::TypeField;
use crate::kuna_dwarfvariants::{VariantFacet, VariantLayout};
use crate::kuna_sourcelang::SourceLang;

const OPTION: &str = "core::option::Option<&u64>";

fn setup(
    opcode: OpCode,
    flip: bool,
    negate: bool,
    reverse: bool,
    referent_kind: u8,
) -> (
    Funcdata,
    crate::architecture::Architecture,
    PrintC,
    OpId,
    VarnodeId,
    VarnodeId,
) {
    let mut arch = bare_arch();
    arch.archid = "x86:LE:64:default:gcc".into();
    arch.source_lang_forced = Some(SourceLang::Rust);
    arch.types_impl().set_max_basetype_size(8);
    arch.types_impl().set_default_alignment_map();
    let types = arch.types();
    let referent = types
        .get_base(
            8,
            if referent_kind == 1 {
                type_metatype::TYPE_INT
            } else {
                type_metatype::TYPE_UINT
            },
        )
        .unwrap();
    let pointer = if referent_kind == 2 {
        types
            .get_type_pointer_rel_full(8, referent.clone(), referent, 1, 8, "relative")
            .unwrap()
    } else {
        types.get_type_pointer(8, referent, 1).unwrap()
    };
    let payload_name = format!("{OPTION}::Some");
    let payload = types.get_type_struct(&payload_name).unwrap();
    let payload = types
        .set_fields_struct_raw(
            &payload,
            vec![TypeField::new(0, 0, "__0", pointer)],
            vec![],
            8,
            8,
            0,
        )
        .unwrap();
    let owner = types.get_type_union(OPTION).unwrap();
    let owner = types
        .set_fields_union_raw(&owner, vec![TypeField::new(0, 0, "Some", payload)], 8, 8)
        .unwrap();
    types.kuna_record_variant_layout(VariantLayout {
        type_name: OPTION.into(),
        size: 8,
        tag_offset: 0,
        tag_size: 8,
        payload_offset: 0,
        niche: true,
        union_type: OPTION.into(),
        variants: vec![
            VariantFacet {
                name: "None".into(),
                discr: Some(0),
                label: String::new(),
                payload_type: String::new(),
                fields: vec![],
            },
            VariantFacet {
                name: "Some".into(),
                discr: None,
                label: "Some".into(),
                payload_type: payload_name,
                fields: vec![("__0".into(), 0)],
            },
        ],
    });
    let mut fd = build_fd();
    let address = Address::new(ram(&fd), 0x400);
    let value = fd.new_varnode(8, &address, Some(owner.clone()));
    let value = fd.set_input_varnode(value).unwrap();
    let model = Rc::new(crate::fspec::ProtoModel::new(fd.get_arch().manage()));
    let proto = fd.get_func_proto_mut();
    proto.set_internal(model, named("void", 0, type_metatype::TYPE_VOID));
    proto.set_param(
        0,
        "value",
        &crate::fspec::ParameterPieces {
            addr: address,
            type_: Some(owner.clone()),
            flags: 0,
        },
    );
    proto.set_input_lock(true);
    let compare = mk_op(&mut fd, 2, 0x20, opcode);
    let zero = fd.new_constant(8, 0);
    fd.op_set_input(compare, value, i32::from(reverse)).unwrap();
    fd.op_set_input(compare, zero, i32::from(!reverse)).unwrap();
    let condition = fd.new_varnode(
        1,
        &Address::new(ram(&fd), 0x420),
        Some(named("bool", 1, type_metatype::TYPE_BOOL)),
    );
    fd.op_set_output(compare, condition).unwrap();
    fd.vbank_mut().get_mut(condition).unwrap().set_implied();
    let final_condition = if negate {
        let op = mk_op(&mut fd, 1, 0x25, OpCode::CPUI_BOOL_NEGATE);
        fd.op_set_input(op, condition, 0).unwrap();
        let out = fd.new_varnode(
            1,
            &Address::new(ram(&fd), 0x430),
            Some(named("bool", 1, type_metatype::TYPE_BOOL)),
        );
        fd.op_set_output(op, out).unwrap();
        fd.vbank_mut().get_mut(out).unwrap().set_implied();
        out
    } else {
        condition
    };
    let branch = mk_op(&mut fd, 2, 0x30, OpCode::CPUI_CBRANCH);
    let target = fd.new_constant(8, 0x50);
    fd.op_set_input(branch, target, 0).unwrap();
    fd.op_set_input(branch, final_condition, 1).unwrap();
    if flip {
        fd.obank_mut()
            .get_mut(branch)
            .unwrap()
            .set_flag(crate::op::pcodeop_flags::boolean_flip);
    }
    fd.set_high_level();
    let high = fd.vbank().get(value).unwrap().get_high().unwrap();
    let high = fd.high_bank_mut().get_mut(high).unwrap();
    high.set_kuna_name("value");
    high.set_symbol_offset(0);
    high.set_symbol_type(owner);
    let mut printer = PrintC::new();
    printer.set_name("rust");
    printer.set_output_stream();
    (fd, arch, printer, branch, value, condition)
}

#[test]
fn named_reference_niche_follows_final_boolean_polarity() {
    for opcode in [OpCode::CPUI_INT_EQUAL, OpCode::CPUI_INT_NOTEQUAL] {
        for flip in [false, true] {
            for negate in [false, true] {
                for reverse in [false, true] {
                    let (fd, arch, mut printer, branch, _, _) =
                        setup(opcode, flip, negate, reverse, 0);
                    assert!(printer.emit_rust_niche_option_condition(&fd, &arch, branch));
                    let some = (opcode == OpCode::CPUI_INT_NOTEQUAL) ^ flip ^ negate;
                    assert_eq!(
                        printer.emit_mut().output_str(),
                        format!(
                            "let ::core::option::Option::{} = value",
                            if some { "Some(_)" } else { "None" }
                        )
                    );
                }
            }
        }
    }
}

#[test]
fn named_reference_niche_requires_the_complete_recorded_layout() {
    for case in 0..14 {
        let (fd, arch, mut printer, branch, _, _) = setup(
            OpCode::CPUI_INT_NOTEQUAL,
            false,
            false,
            false,
            if case == 12 {
                1
            } else if case == 13 {
                2
            } else {
                0
            },
        );
        let mut layout = (*arch.types().kuna_variant_layout(OPTION).unwrap()).clone();
        match case {
            0 => layout.size = 16,
            1 => layout.tag_offset = 1,
            2 => layout.tag_size = 4,
            3 => layout.payload_offset = 8,
            4 => layout.niche = false,
            5 => layout.union_type = "other".into(),
            6 => layout.variants[0].discr = Some(1),
            7 => layout.variants[0].fields.push(("hidden".into(), 0)),
            8 => layout.variants[1].discr = Some(1),
            9 => layout.variants[1].fields[0].1 = 4,
            10 => layout.variants[1].payload_type = "missing".into(),
            11 => layout.variants.push(layout.variants[0].clone()),
            12 | 13 => {}
            _ => unreachable!(),
        }
        arch.types().kuna_record_variant_layout(layout);
        assert!(
            !printer.emit_rust_niche_option_condition(&fd, &arch, branch),
            "case {case}"
        );
        assert_eq!(printer.emit_mut().output_str(), "");
    }
}

#[test]
fn named_reference_niche_refuses_consumed_saved_partial_and_foreign_values() {
    for case in 0..19 {
        let (mut fd, mut arch, mut printer, branch, value, condition) =
            setup(OpCode::CPUI_INT_NOTEQUAL, false, false, false, 0);
        let high = fd.vbank().get(value).unwrap().get_high().unwrap();
        match case {
            0 => printer.set_name("c"),
            1 => printer.set_name("go"),
            2 => arch.source_lang_forced = Some(SourceLang::C),
            3 => arch.archid = "AARCH64:LE:64:v8A:default".into(),
            4 => fd.vbank_mut().get_mut(condition).unwrap().clear_implied(),
            5 => fd
                .vbank_mut()
                .get_mut(value)
                .unwrap()
                .set_flags_pub(crate::varnode::varnode_flags::volatil),
            6 => fd
                .vbank_mut()
                .get_mut(value)
                .unwrap()
                .set_flags_pub(crate::varnode::varnode_flags::persist),
            7 => fd
                .high_bank_mut()
                .get_mut(high)
                .unwrap()
                .set_symbol_offset(4),
            8 => fd
                .high_bank_mut()
                .get_mut(high)
                .unwrap()
                .set_kuna_global(true),
            9 => {
                let other = mk_op(&mut fd, 1, 0x40, OpCode::CPUI_COPY);
                fd.op_set_input(other, value, 0).unwrap();
            }
            10 => {
                let comparison = fd.vbank().get(condition).unwrap().get_def().unwrap();
                let wrong = fd.new_constant(8, 1);
                fd.op_set_input(comparison, wrong, 1).unwrap();
            }
            11 => {
                let address = fd.vbank().get(value).unwrap().get_addr().clone();
                fd.get_func_proto_mut().set_param(
                    0,
                    "value",
                    &crate::fspec::ParameterPieces {
                        addr: address,
                        type_: Some(named("uint8", 8, type_metatype::TYPE_UINT)),
                        flags: 0,
                    },
                );
            }
            12 => fd.get_func_proto_mut().remove_param(0),
            13 | 14 => {
                let address = fd.vbank().get(value).unwrap().get_addr().clone();
                let original = fd
                    .get_func_proto()
                    .get_param(0)
                    .unwrap()
                    .get_type()
                    .unwrap()
                    .clone();
                let address = if case == 14 {
                    Address::new(ram(&fd), 0x408)
                } else {
                    address
                };
                fd.get_func_proto_mut().set_param(
                    0,
                    "value",
                    &crate::fspec::ParameterPieces {
                        addr: address,
                        type_: Some(if case == 13 {
                            named("uint4", 4, type_metatype::TYPE_UINT)
                        } else {
                            original
                        }),
                        flags: 0,
                    },
                );
            }
            15 => *fd.get_func_proto_mut() = crate::fspec::FuncProto::new(),
            16..=18 => {
                let address = fd.vbank().get(value).unwrap().get_addr().clone();
                let ty = fd
                    .get_func_proto()
                    .get_param(0)
                    .unwrap()
                    .get_type()
                    .unwrap()
                    .clone();
                let flags = match case {
                    17 => crate::fspec::parameter_pieces_flags::HIDDENRETPARM,
                    18 => crate::fspec::parameter_pieces_flags::INDIRECTSTORAGE,
                    _ => 0,
                };
                fd.get_func_proto_mut().set_param(
                    if case == 16 { 1 } else { 0 },
                    "other",
                    &crate::fspec::ParameterPieces {
                        addr: address,
                        type_: Some(ty),
                        flags,
                    },
                );
            }
            _ => unreachable!(),
        }
        assert!(
            !printer.emit_rust_niche_option_condition(&fd, &arch, branch),
            "case {case}"
        );
        assert_eq!(printer.emit_mut().output_str(), "");
    }
}

#[test]
fn named_reference_niche_uses_the_emitted_name_and_refuses_aliases() {
    let (fd, arch, mut printer, branch, value, _) =
        setup(OpCode::CPUI_INT_NOTEQUAL, false, false, false, 0);
    let high = fd.vbank().get(value).unwrap().get_high().unwrap();
    printer
        .local_name_overrides
        .insert(high, "input_option".into());
    assert!(printer.emit_rust_niche_option_condition(&fd, &arch, branch));
    assert_eq!(
        printer.emit_mut().output_str(),
        "let ::core::option::Option::Some(_) = input_option"
    );
    printer.local_name_aliases.insert(high, high);
    assert!(!printer.emit_rust_niche_option_condition(&fd, &arch, branch));
    printer.local_name_aliases.clear();
    for name in [
        "",
        "_",
        "self",
        "Self",
        "super",
        "crate",
        "module::value",
        "type",
        "match",
        "gen",
        "r#type",
        "2value",
        "value.field",
        "é",
    ] {
        printer.local_name_overrides.insert(high, name.into());
        assert!(!printer.emit_rust_niche_option_condition(&fd, &arch, branch));
    }
}
