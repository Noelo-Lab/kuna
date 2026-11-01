//! Compiler-derived BE N32/N64 storage contracts, including register exhaustion.
use kuna_base::address::Address;
use kuna_base::xml::DocumentStorage;
use kuna_console::{
    engine::{bootstrap_from_object, bootstrap_program, ConsoleProgram, EntrySelector},
    grammar::{parse_c, DataOrg},
    project::decompile_targets,
};
use kuna_decomp::dtype::type_metatype;
use kuna_decomp::fspec::ParameterPieces;
use std::path::PathBuf;

fn program(variant: &str, n32: bool) -> ConsoleProgram {
    program_with_model(variant, n32, "default")
}

fn program_with_model(variant: &str, n32: bool, model: &str) -> ConsoleProgram {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let mut docs = DocumentStorage::new();
    let image = docs.parse_document(br#"<binaryimage arch="MIPS:BE:64:octeon"><bytechunk space="ram" offset="0x1000">03e0000800000000</bytechunk></binaryimage>"#).unwrap().get_root().clone();
    let target = format!(
        "MIPS:BE:64:{variant}{}:{model}",
        if n32 { "-32addr" } else { "" }
    );
    let prog = bootstrap_program(
        image,
        &target,
        &[root.join("specs").to_string_lossy().into_owned()],
    )
    .unwrap();
    let (addr_size, word_size) = prog.arch().data_org();
    prog.arch()
        .types()
        .get_base_named(16, type_metatype::TYPE_INT, "int128_t")
        .unwrap();
    for decl in [
        "struct Small { int x; };",
        "union SmallUnion { int x; float f; };",
        "struct Pair { long long a, b; };",
        "struct Twelve { int x, y, z; };",
        "struct Quad { long double x; };",
        "struct Layout { char x; long double q; };",
        "struct Floats { float a, b; };",
        "struct Doubles { double a, b; };",
        "struct Mixed { double a; long long b; };",
        "struct Inner { double d; };",
        "struct Nested { struct Inner a; };",
        "struct ArrayDouble { double a[1]; };",
        "struct FloatDouble { float a; double b; };",
        "struct DoubleFloat { double a; float b; };",
        "struct QuadDouble { long double q; double d; };",
        "struct TwoQuads { long double a, b; };",
        "struct DoubleBit { double d; unsigned b:1; };",
        "struct BitDouble { unsigned b:1; double d; };",
    ] {
        parse_c(
            decl,
            prog.arch().types(),
            DataOrg {
                addr_size,
                word_size,
            },
            &[],
            |_, _| Ok(()),
        )
        .unwrap();
    }
    let types = prog.arch().types();
    let shell = types.get_type_struct("AlignedDoubles").unwrap();
    let double = types.get_base(8, type_metatype::TYPE_FLOAT).unwrap();
    types
        .set_fields_struct_raw(
            &shell,
            vec![
                kuna_decomp::dtype::TypeField::new(0, 0, "a", double.clone()),
                kuna_decomp::dtype::TypeField::new(1, 8, "b", double),
            ],
            Vec::new(),
            16,
            16,
            0,
        )
        .unwrap();
    prog
}

fn allocate(prog: &ConsoleProgram, datatype: &str, before: usize) -> Vec<ParameterPieces> {
    allocate_varargs(prog, datatype, before, -1)
}

fn allocate_varargs(
    prog: &ConsoleProgram,
    datatype: &str,
    before: usize,
    first: i32,
) -> Vec<ParameterPieces> {
    let mut args = (0..before)
        .map(|i| format!("long long p{i}"))
        .collect::<Vec<_>>();
    args.extend([format!("{datatype} value"), "long long after".into()]);
    let arch = prog.arch();
    let (addr_size, word_size) = arch.data_org();
    let mut result = Vec::new();
    parse_c(
        &format!("extern {datatype} f({});", args.join(",")),
        arch.types(),
        DataOrg {
            addr_size,
            word_size,
        },
        &[],
        |proto, _| {
            let mut proto = proto.clone();
            proto.first_var_arg_slot = first;
            arch.default_fp().unwrap().assign_parameter_storage(
                &proto,
                &mut result,
                false,
                arch.types(),
                arch.manage(),
            )
        },
    )
    .unwrap();
    result
}

fn storage(prog: &ConsoleProgram, param: &ParameterPieces) -> Vec<(String, u64, u32)> {
    let addr = &param.addr;
    if addr.get_space().unwrap().get_name() == "join" {
        let join = prog.arch().manage().find_join(addr.get_offset()).unwrap();
        (0..join.num_pieces())
            .map(|i| {
                let p = join.get_piece(i);
                (
                    p.space.as_ref().unwrap().get_name().to_string(),
                    p.offset,
                    p.size,
                )
            })
            .collect()
    } else {
        vec![(
            addr.get_space().unwrap().get_name().to_string(),
            addr.get_offset(),
            param.type_.as_ref().unwrap().get_size() as u32,
        )]
    }
}

fn reg(prog: &ConsoleProgram, name: &str, pad: u64, size: u32) -> (String, u64, u32) {
    let r = prog
        .arch()
        .translate()
        .get_register_varnode(name.as_bytes())
        .unwrap();
    ("register".into(), r.offset + pad, size)
}
fn slot(prog: &ConsoleProgram, index: usize, size: u32) -> (String, u64, u32) {
    if index < 8 {
        reg(
            prog,
            ["a0", "a1", "a2", "a3", "t0", "t1", "t2", "t3"][index],
            0,
            size,
        )
    } else {
        ("stack".into(), (index as u64 - 8) * 8, size)
    }
}

#[test]
fn abi_return_registers_do_not_preserve_their_pre_call_values() {
    for n32 in [true, false] {
        let prog = program("octeon", n32);
        let model = prog.arch().default_fp().unwrap();
        for name in ["v0", "v1"] {
            let r = prog
                .arch()
                .translate()
                .get_register_varnode(name.as_bytes())
                .unwrap();
            let addr = Address::new(r.space.unwrap(), r.offset);
            assert_eq!(
                model.has_effect(&addr, 8),
                kuna_decomp::fspec::effect_type::KILLEDBYCALL,
                "n32={n32}: all physical bytes of {name} are caller-clobbered"
            );
        }
    }
}

#[test]
fn hard_float_scalars_share_argument_slots_and_keep_quad_alignment() {
    for n32 in [true, false] {
        let prog = program_with_model("octeon3", n32, "hard");
        for before in [0, 1, 6, 7, 8, 9] {
            for (ty, size) in [("float", 4), ("double", 8), ("long double", 16)] {
                let assigned = allocate(&prog, ty, before);
                let first = if size == 16 {
                    (before + 1) & !1
                } else {
                    before
                };
                let expected = if first >= 8 {
                    vec![("stack".into(), (first as u64 - 8) * 8, size)]
                } else if size == 16 {
                    vec![
                        reg(&prog, &format!("f{}", 12 + first), 0, 8),
                        reg(&prog, &format!("f{}", 13 + first), 0, 8),
                    ]
                } else {
                    vec![reg(
                        &prog,
                        &format!("f{}", 12 + first),
                        (8 - size) as u64,
                        size,
                    )]
                };
                assert_eq!(
                    storage(&prog, &assigned[before + 1]),
                    expected,
                    "n32={n32} {ty} after {before}"
                );
                assert_eq!(
                    storage(&prog, assigned.last().unwrap()),
                    vec![slot(&prog, first + if size == 16 { 2 } else { 1 }, 8)]
                );
                let result = if size == 16 {
                    vec![reg(&prog, "f0", 0, 8), reg(&prog, "f2", 0, 8)]
                } else {
                    vec![reg(&prog, "f0", (8 - size) as u64, size)]
                };
                assert_eq!(
                    storage(&prog, &assigned[0]),
                    result,
                    "n32={n32} return {ty}"
                );
            }
        }
    }
}

#[test]
fn hard_float_aggregate_storage_is_not_scalar_quad_storage() {
    for n32 in [true, false] {
        let prog = program_with_model("octeon3", n32, "hard");
        for (ty, input, output) in [
            (
                "struct Floats",
                vec![reg(&prog, "a0", 0, 8)],
                vec![reg(&prog, "f0", 4, 4), reg(&prog, "f2", 4, 4)],
            ),
            (
                "struct Doubles",
                vec![reg(&prog, "f12", 0, 8), reg(&prog, "f13", 0, 8)],
                vec![reg(&prog, "f0", 0, 8), reg(&prog, "f2", 0, 8)],
            ),
            (
                "struct Mixed",
                vec![reg(&prog, "f12", 0, 8), reg(&prog, "a1", 0, 8)],
                vec![reg(&prog, "v0", 0, 8), reg(&prog, "v1", 0, 8)],
            ),
            (
                "struct Quad",
                vec![reg(&prog, "a0", 0, 8), reg(&prog, "a1", 0, 8)],
                vec![reg(&prog, "f0", 0, 8), reg(&prog, "f1", 0, 8)],
            ),
        ] {
            let assigned = allocate(&prog, ty, 0);
            assert_eq!(storage(&prog, &assigned[1]), input, "n32={n32} input {ty}");
            assert_eq!(
                storage(&prog, &assigned[0]),
                output,
                "n32={n32} output {ty}"
            );
        }
    }
}

#[test]
fn direct_float_fields_use_fprs_but_nested_fields_use_gprs() {
    for n32 in [true, false] {
        let prog = program_with_model("octeon3", n32, "hard");
        for ty in ["struct Nested", "struct ArrayDouble"] {
            let assigned = allocate(&prog, ty, 0);
            assert_eq!(storage(&prog, &assigned[1]), vec![reg(&prog, "a0", 0, 8)]);
            assert_eq!(storage(&prog, &assigned[0]), vec![reg(&prog, "v0", 0, 8)]);
        }
        for (ty, input, output) in [
            ("struct FloatDouble",
                vec![reg(&prog, "a0", 0, 8), reg(&prog, "f13", 0, 8)],
                vec![reg(&prog, "f0", 4, 4), ("const".into(), 0, 4), reg(&prog, "f2", 0, 8)]),
            ("struct DoubleFloat",
                vec![reg(&prog, "f12", 0, 8), reg(&prog, "a1", 0, 8)],
                vec![reg(&prog, "f0", 0, 8), reg(&prog, "f2", 4, 4), ("const".into(), 0, 4)]),
            ("struct DoubleBit",
                vec![reg(&prog, "f12", 0, 8), reg(&prog, "a1", 0, 8)],
                vec![reg(&prog, "v0", 0, 8), reg(&prog, "v1", 0, 8)]),
            ("struct BitDouble",
                vec![reg(&prog, "a0", 0, 8), reg(&prog, "f13", 0, 8)],
                vec![reg(&prog, "v0", 0, 8), reg(&prog, "v1", 0, 8)]),
        ] {
            let assigned = allocate(&prog, ty, 0);
            assert_eq!(storage(&prog, &assigned[1]), input, "{n32} {ty}");
            assert_eq!(storage(&prog, &assigned[0]), output, "{n32} {ty}");
        }
        for before in [0, 1, 5, 6, 7, 8, 9] {
            let mut params: Vec<_> = (0..before).map(|i| format!("long long p{i}")).collect();
            params.extend(["struct QuadDouble x".into(), "long long after".into()]);
            let mut assigned = Vec::new();
            let (addr_size, word_size) = prog.arch().data_org();
            parse_c(&format!("extern void f({});", params.join(",")), prog.arch().types(),
                DataOrg { addr_size, word_size }, &[], |proto, _| {
                    prog.arch().default_fp().unwrap().assign_parameter_storage(
                        &proto, &mut assigned, false, prog.arch().types(), prog.arch().manage())
                }).unwrap();
            let first = (before + 1) & !1;
            let expected = if first >= 8 {
                vec![("stack".into(), ((first - 8) * 8) as u64, 32)]
            } else if first == 6 {
                vec![slot(&prog, 6, 8), slot(&prog, 7, 8), ("stack".into(), 0, 16)]
            } else {
                vec![slot(&prog, first, 8), slot(&prog, first + 1, 8),
                    reg(&prog, &format!("f{}", first + 14), 0, 8), slot(&prog, first + 3, 8)]
            };
            assert_eq!(storage(&prog, &assigned[before + 1]), expected, "{n32} after {before}");
            assert_eq!(storage(&prog, assigned.last().unwrap()), vec![slot(&prog, first + 4, 8)]);
        }
        let assigned = allocate(&prog, "struct TwoQuads", 0);
        assert!(assigned[0].flags & kuna_decomp::fspec::parameter_pieces_flags::INDIRECTSTORAGE != 0, "large floating structs require a hidden return pointer");
    }
}

#[test]
fn hard_float_varargs_use_gprs_after_the_named_parameters() {
    for n32 in [true, false] {
        let prog = program_with_model("octeon3", n32, "hard");
        for before in [0, 1, 7, 8, 9] {
            for (ty, size) in [("double", 8), ("long double", 16), ("struct Doubles", 16)] {
                let assigned = allocate_varargs(&prog, ty, before, 1);
                if before == 0 {
                    assert_eq!(storage(&prog, &assigned[1])[0], reg(&prog, "f12", 0, 8));
                    continue;
                }
                let first = if ty == "long double" {
                    (before + 1) & !1
                } else {
                    before
                };
                let expected = if first >= 8 || size == 8 {
                    vec![slot(&prog, first, size)]
                } else {
                    vec![slot(&prog, first, 8), slot(&prog, first + 1, 8)]
                };
                assert_eq!(
                    storage(&prog, &assigned[before + 1]),
                    expected,
                    "n32={n32} variadic {ty} after {before}"
                );
                assert_eq!(
                    storage(&prog, assigned.last().unwrap()),
                    vec![slot(&prog, first + size as usize / 8, 8)]
                );
            }
        }
    }
}

#[test]
fn floating_structs_with_bitfields_return_through_gprs() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    for n32 in [true, false] {
        for gcc47 in [true, false] {
            for bits_first in [true, false] {
                let adjust = if n32 { "27bd" } else { "67bd" };
                let move_op = if gcc47 { "2d" } else { "25" };
                let body = if bits_first {
                    format!("{adjust}ffe0008010{move_op}f7ad0018dfa3001803e00008{adjust}0020")
                } else {
                    format!("{adjust}ffe000a018{move_op}f7ac0010dfa2001003e00008{adjust}0020")
                };
                let target = format!("MIPS:BE:64:octeon3{}:hard", if n32 { "-32addr" } else { "" });
                let xml = format!("<binaryimage arch=\"{target}\"><bytechunk space=\"ram\" offset=\"0x1000\">{body}</bytechunk><symbol space=\"ram\" offset=\"0x1000\" name=\"probe\"/></binaryimage>");
                let mut docs = DocumentStorage::new();
                let image = docs.parse_document(xml.as_bytes()).unwrap().get_root().clone();
                let mut prog = bootstrap_program(image, &target, &[root.join("specs").to_string_lossy().into_owned()]).unwrap();
                prog.arch_mut().set_kuna_option("ctypes", "on").unwrap();
                let (addr_size, word_size) = prog.arch().data_org();
                let fields = if bits_first { "unsigned b:1; double d;" } else { "double d; unsigned b:1;" };
                parse_c(&format!("struct Bits {{ {fields} }};"), prog.arch().types(),
                    DataOrg { addr_size, word_size }, &[], |_, _| Ok(())).unwrap();
                let mut proto = None;
                parse_c("extern struct Bits probe(struct Bits x);", prog.arch().types(),
                    DataOrg { addr_size, word_size }, &[], |pieces, _| { proto = Some(pieces); Ok(()) }).unwrap();
                prog.set_pending_prototype("probe", proto.unwrap());
                let entry = prog.resolve_entry(&EntrySelector::Name("probe".into())).unwrap();
                let result = decompile_targets(&mut prog, vec![entry], false, false, false);
                assert!(result[0].error.is_none(), "{:?}", result[0].error);
                assert!(result[0].code.as_ref().unwrap().contains("return x;"),
                    "n32={n32} gcc47={gcc47} bits_first={bits_first}: {:?}", result[0].code);
            }
        }
    }
}

#[test]
fn hard_float_saved_register_sets_follow_the_selected_abi() {
    use kuna_decomp::fspec::effect_type::{KILLEDBYCALL, UNAFFECTED};
    for n32 in [true, false] {
        let prog = program_with_model("octeon3", n32, "hard");
        for index in 0..32 {
            let name = format!("f{index}");
            let r = prog
                .arch()
                .translate()
                .get_register_varnode(name.as_bytes())
                .unwrap();
            let addr = Address::new(r.space.unwrap(), r.offset);
            let saved = if n32 {
                index >= 20 && index % 2 == 0
            } else {
                index >= 24
            };
            assert_eq!(
                prog.arch().default_fp().unwrap().has_effect(&addr, 8),
                if saved { UNAFFECTED } else { KILLEDBYCALL },
                "n32={n32} {name}"
            );
        }
    }
}

#[test]
fn aligned_float_aggregates_skip_the_same_slots_in_both_register_classes() {
    for n32 in [true, false] {
        let prog = program_with_model("octeon3", n32, "hard");
        for before in [0, 1, 6, 7, 8, 9] {
            let assigned = allocate(&prog, "struct AlignedDoubles", before);
            let first = (before + 1) & !1;
            let expected = if first < 8 {
                vec![
                    reg(&prog, &format!("f{}", first + 12), 0, 8),
                    reg(&prog, &format!("f{}", first + 13), 0, 8),
                ]
            } else {
                vec![slot(&prog, first, 16)]
            };
            assert_eq!(
                storage(&prog, &assigned[before + 1]),
                expected,
                "n32={n32} before={before}"
            );
            assert_eq!(
                storage(&prog, assigned.last().unwrap()),
                vec![slot(&prog, first + 2, 8)]
            );
        }
    }
}

#[test]
fn octeon_integer_promotions_keep_type_sign_and_the_32_bit_abi_exception() {
    use kuna_decomp::kuna_narrowext::{extension, rules, Widen};
    use kuna_num::{opcodes::OpCode, pcoderaw::VarnodeData};
    for n32 in [true, false] {
        for model in if n32 { vec!["default", "hard", "n32"] } else { vec!["default", "hard"] } {
            let prog = program_with_model("octeon3", n32, model);
            assert_eq!(
                kuna_decomp::kuna_typedcallabi::image_evidence(prog.arch()),
                Some(model == "hard")
            );
            let rules = rules(prog.arch());
            assert_eq!(rules.input, Some(Widen::Register));
            for (ty, size, expected) in [
                ("signed char", 1, OpCode::CPUI_INT_SEXT),
                ("unsigned char", 1, OpCode::CPUI_INT_ZEXT),
                ("unsigned short", 2, OpCode::CPUI_INT_ZEXT),
                ("unsigned int", 4, OpCode::CPUI_INT_SEXT),
                ("int", 4, OpCode::CPUI_INT_SEXT),
            ] {
                let assigned = allocate(&prog, ty, 0);
                let param = &assigned[1];
                let mut physical = VarnodeData::default();
                assert_eq!(
                    extension(
                        rules.input.unwrap(),
                        prog.arch().default_fp().unwrap().input(),
                        &param.addr,
                        size,
                        param.type_.as_ref().unwrap(),
                        &mut physical
                    ),
                    Some(expected),
                    "n32={n32} model={model} {ty}"
                );
                assert_eq!(physical, prog.arch().get_register_varnode(b"a0").unwrap());
            }
        }
    }
}

#[test]
fn independent_hard_float_images_select_and_recover_the_declared_values() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    for compiler in ["gcc73", "gcc47"] {
        for abi in ["n32", "64"] {
            let mut prog = bootstrap_from_object(
                root.join(format!(
                    "tests/fixtures/octeon-elf-abi/abi-{compiler}-{abi}.elf"
                ))
                .to_str()
                .unwrap(),
                "",
                &[root.join("specs").to_string_lossy().into_owned()],
            )
            .unwrap();
            prog.arch_mut().set_kuna_option("ctypes", "on").unwrap();
            prog.commit_pending_analysis().unwrap();
            let types = prog.arch().types();
            let shell = types.get_type_struct("ExplicitAlignedDoubles").unwrap();
            let double = types.get_base(8, type_metatype::TYPE_FLOAT).unwrap();
            types
                .set_fields_struct_raw(
                    &shell,
                    vec![
                        kuna_decomp::dtype::TypeField::new(0, 0, "a", double.clone()),
                        kuna_decomp::dtype::TypeField::new(1, 8, "b", double),
                    ],
                    Vec::new(),
                    16,
                    16,
                    0,
                )
                .unwrap();
            for declaration in [
                "extern struct Pair pair_indirect(struct Pair (*callee)(unsigned long long,unsigned long long),unsigned long long a,unsigned long long b);",
                "extern long double quad_indirect(long double (*callee)(long double),long double x);",
                "extern double aligned_second(unsigned long long a,struct ExplicitAlignedDoubles x,unsigned long long c);",
                "extern unsigned long long aligned_follow(unsigned long long a,struct ExplicitAlignedDoubles x,unsigned long long c);",
            ] {
                let arch = prog.arch();
                let (addr_size, word_size) = arch.data_org();
                let mut pieces = None;
                parse_c(declaration, arch.types(), DataOrg { addr_size, word_size }, &[], |proto, _| {
                    pieces = Some(proto);
                    Ok(())
                }).unwrap();
                let pieces = pieces.unwrap();
                prog.set_pending_prototype(&pieces.name.clone(), pieces);
            }
            for (name, expected) in [
                ("scalar_after", "return b;"),
                ("scalar_stack", "return x;"),
                ("quad", "return x;"),
                ("quad_struct", "return x;"),
                ("quad_after", "return x;"),
                ("after_quad", "return b;"),
                ("floats", "return x;"),
                ("doubles", "return x;"),
                ("mixed", "return x;"),
                ("named_varargs", "return x;"),
                ("indirect", "return (*callee)(x) + x;"),
                ("signed_indirect", "return (*callee)(x) + x;"),
                ("void_indirect", "(*callee)(x);\n  return x;"),
                ("pair_indirect", "return (*callee)(a,b);"),
                ("quad_indirect", "return (*callee)(x);"),
                ("aligned_second", "return x.b;"),
                ("aligned_follow", "return c;"),
                ("narrow_s8", "return (long long)x;"),
                ("narrow_u8", "return x;"),
                ("narrow_u16", "return x;"),
                ("narrow_u32", "return x;"),
            ] {
                let entry = prog
                    .resolve_entry(&EntrySelector::Name(name.into()))
                    .unwrap();
                let result = decompile_targets(&mut prog, vec![entry], false, false, false);
                assert!(
                    result[0].error.is_none(),
                    "{compiler} {abi} {name}: {:?}",
                    result[0].error
                );
                let code = result[0].code.as_ref().unwrap();
                assert!(code.contains(expected)
                    || (abi == "64" && name == "narrow_s8" && code.contains("return (long)x;")),
                    "{compiler} {abi} {name}: {code}");
            }
        }
    }
}

#[test]
fn independent_compilers_recover_direct_nested_and_padded_float_fields() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let mut failures = Vec::new();
    for compiler in ["gcc73", "gcc47"] {
        for abi in ["n32", "64"] {
            let mut prog = bootstrap_from_object(
                root.join(format!("tests/fixtures/octeon-elf-abi/fields-{compiler}-{abi}.elf"))
                    .to_str().unwrap(), "", &[root.join("specs").to_string_lossy().into_owned()],
            ).unwrap();
            prog.arch_mut().set_kuna_option("ctypes", "on").unwrap();
            prog.commit_pending_analysis().unwrap();
            for (name, expected) in [
                ("nested_first", "return x.a.d;"),
                ("nested_identity", "return x;"),
                ("array_first", "return x.a[0];"),
                ("array_identity", "return x;"),
                ("float_double", "x.a;"),
                ("double_float", "x.a;"),
                ("quad_double_last", "return x.d;"),
                ("quad_double_after", "return x.d;"),
                ("quad_double_split", "return x.d;"),
                ("float_double_split", "x.a;"),
            ] {
                let entry = prog.resolve_entry(&EntrySelector::Name(name.into())).unwrap();
                let result = decompile_targets(&mut prog, vec![entry], false, false, false);
                assert!(result[0].error.is_none(), "{compiler} {abi} {name}: {:?}", result[0].error);
                let code = result[0].code.as_ref().unwrap();
                let aggregate = matches!(name, "float_double" | "double_float" | "float_double_split");
                let returned = code.lines().find_map(|line|
                    line.trim().strip_prefix("return ").and_then(|line| line.strip_suffix(';')));
                let identity = aggregate && returned == Some("x");
                assert!(!aggregate || !code.contains("undefined4 "), "dead padding local: {code}");
                let both_fields = !aggregate || identity || (returned.is_some_and(|value|
                    code.contains(&format!("{value}.a = ")) && code.contains(&format!("{value}.b = ")))
                    && !code.contains("0 = 0;") && !code.contains("._"));
                if (!code.contains(expected) && !identity) || !both_fields {
                    failures.push(format!("{compiler} {abi} {name}: {code}"));
                }
            }
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[test]
fn n32_alias_preserves_unsigned_argument_extension_with_explicit_selection() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    for compiler in ["gcc73", "gcc47"] {
        for model in ["default", "n32"] {
            let mut prog = bootstrap_from_object(
                root.join(format!("tests/fixtures/octeon-elf-abi/abi-{compiler}-n32.elf"))
                    .to_str().unwrap(), &format!("MIPS:BE:64:octeon3-32addr:{model}"),
                &[root.join("specs").to_string_lossy().into_owned()],
            ).unwrap();
            assert_eq!(kuna_decomp::kuna_typedcallabi::image_evidence(prog.arch()), Some(false));
            prog.arch_mut().set_kuna_option("ctypes", "on").unwrap();
            prog.commit_pending_analysis().unwrap();
            for name in ["narrow_u8", "narrow_u16", "narrow_u32"] {
                let entry = prog.resolve_entry(&EntrySelector::Name(name.into())).unwrap();
                let result = decompile_targets(&mut prog, vec![entry], false, false, false);
                assert!(result[0].error.is_none());
                assert!(result[0].code.as_ref().unwrap().contains("return x;"),
                    "{compiler} {model} {name}: {:?}", result[0].code);
            }
        }
    }
}

#[test]
fn sectionless_pic_entries_recover_t9_under_the_existing_gate() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    for abi in ["n32", "64"] {
        let mut bytes =
            std::fs::read(root.join(format!("tests/fixtures/octeon-elf-abi/abi-gcc73-{abi}.elf")))
                .unwrap();
        if abi == "n32" {
            bytes[32..36].fill(0);
            bytes[48..52].fill(0);
        } else {
            bytes[40..48].fill(0);
            bytes[60..64].fill(0);
        }
        let path = crate::common::scratch_file("octeon-sectionless", "elf");
        std::fs::write(&path, bytes).unwrap();
        for enabled in [false, true] {
            let mut prog = bootstrap_from_object(
                path.to_str().unwrap(),
                "",
                &[root.join("specs").to_string_lossy().into_owned()],
            )
            .unwrap();
            prog.arch_mut()
                .set_kuna_option("mips_gp", if enabled { "on" } else { "off" })
                .unwrap();
            prog.arch_mut().set_kuna_option("ctypes", "on").unwrap();
            prog.commit_pending_analysis().unwrap();
            let entry = prog
                .resolve_entry(&EntrySelector::Numeric(0x100000))
                .unwrap();
            let result = decompile_targets(&mut prog, vec![entry], false, false, false);
            assert!(result[0].error.is_none(), "{abi}: {:?}", result[0].error);
            let code = result[0].code.as_ref().unwrap();
            assert_eq!(
                code.contains("// t9"),
                !enabled,
                "{abi} enabled={enabled}: {code}"
            );
            if enabled {
                assert!(
                    code.contains(if abi == "n32" { "102018" } else { "102020" }),
                    "{abi}: {code}"
                );
            }
        }
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn pic_imports_use_the_dynamic_name_instead_of_the_linker_stub_label() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    for compiler in ["gcc73", "gcc47"] {
        for abi in ["n32", "64"] {
            let mut prog = bootstrap_from_object(
                root.join(format!(
                    "tests/fixtures/octeon-elf-abi/imports-{compiler}-{abi}.so"
                ))
                .to_str()
                .unwrap(),
                "",
                &[root.join("specs").to_string_lossy().into_owned()],
            )
            .unwrap();
            prog.arch_mut().set_kuna_option("ctypes", "on").unwrap();
            prog.commit_pending_analysis().unwrap();
            let entry = prog
                .resolve_entry(&EntrySelector::Name("imported_call".into()))
                .unwrap();
            let result = decompile_targets(&mut prog, vec![entry], false, false, false);
            assert!(
                result[0].error.is_none(),
                "{compiler} {abi}: {:?}",
                result[0].error
            );
            let code = result[0].code.as_ref().unwrap();
            assert!(code.contains("external_value("), "{compiler} {abi}: {code}");
            assert!(!code.contains("_MIPS_STUBS_("), "{compiler} {abi}: {code}");
            use kuna_console::assertions::{self, Body, Directive};
            prog.set_assertions(vec![Directive {
                raw: "prototype external_value unsigned long long external_value(unsigned long long value)".into(),
                body: Body::Prototype {
                    func: "external_value".into(),
                    decl: "unsigned long long external_value(unsigned long long value)".into(),
                },
            }]);
            assertions::apply_program_scoped(&mut prog);
            assert_eq!(
                prog.assertion_outcomes()[0].status,
                "applied",
                "{:?}",
                prog.assertion_outcomes()
            );
            let entry = prog
                .resolve_entry(&EntrySelector::Name("imported_call".into()))
                .unwrap();
            let result = decompile_targets(&mut prog, vec![entry], false, false, false);
            assert!(
                result[0].error.is_none(),
                "{compiler} {abi}: {:?}",
                result[0].error
            );
            let code = result[0].code.as_ref().unwrap();
            assert!(
                code.contains("return external_value(value) + value;"),
                "{compiler} {abi}: {code}"
            );
        }
    }
}

#[test]
fn mips_entry_tracking_keeps_physical_width_and_explicit_values() {
    use kuna_sleigh::globalcontext::TrackedContext;
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    for abi in ["n32", "64"] {
        let mut prog = bootstrap_from_object(
            root.join(format!("tests/fixtures/octeon-elf-abi/abi-gcc73-{abi}.elf"))
                .to_str()
                .unwrap(),
            "",
            &[root.join("specs").to_string_lossy().into_owned()],
        )
        .unwrap();
        prog.commit_pending_analysis().unwrap();
        let loc = prog.arch().get_register_varnode(b"t9").unwrap();
        let code = prog
            .arch()
            .manage()
            .get_default_code_space()
            .unwrap()
            .clone();
        for raw in [0x100000, 0x81234560] {
            if raw != 0x100000 {
                prog.arch()
                    .translate()
                    .loader_rc()
                    .borrow_mut()
                    .adjust_vma((raw - 0x100000) as i64);
            }
            let entry = Address::new(code.clone(), raw);
            let fd = prog
                .arch_mut()
                .new_funcdata("probe", entry.clone(), 4)
                .unwrap();
            let tracked = fd.get_arch().tracked_sets.get_value(&entry);
            let expected = if abi == "n32" {
                (raw as u32 as i32 as i64) as u64
            } else {
                raw
            };
            assert_eq!(
                tracked.iter().find(|t| t.loc == loc).map(|t| t.val),
                Some(expected)
            );
            let end = Address::new(code.clone(), raw + 1);
            prog.arch().with_context_db_mut(|db| {
                db.create_set(&entry, &end).push(TrackedContext {
                    loc: loc.clone(),
                    val: 0x87654321,
                });
            });
            let fd = prog
                .arch_mut()
                .new_funcdata("probe", entry.clone(), 4)
                .unwrap();
            let tracked = fd.get_arch().tracked_sets.get_value(&entry);
            assert_eq!(
                tracked
                    .iter()
                    .filter(|t| t.loc == loc)
                    .map(|t| t.val)
                    .collect::<Vec<_>>(),
                vec![0x87654321]
            );
        }
    }
    assert!(program("octeon", true).arch().loader_mips_t9.is_none());
}

#[test]
fn an_interior_block_does_not_replace_an_incoming_indirect_target_with_itself() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let mut prog = bootstrap_from_object(
        root.join("decompiler/crates/kuna-analysis/tests/fixtures/floatret_chain_mips_O0")
            .to_str()
            .unwrap(),
        "",
        &[root.join("specs").to_string_lossy().into_owned()],
    )
    .unwrap();
    prog.commit_pending_analysis().unwrap();
    let entry = prog
        .resolve_entry(&EntrySelector::Numeric(0x400f04))
        .unwrap();
    let result = decompile_targets(&mut prog, vec![entry], false, false, false);
    assert!(result[0].error.is_none(), "{:?}", result[0].error);
    let code = result[0].code.as_ref().unwrap();
    assert!(!code.contains("= sub_400f04;"), "{code}");
    assert!(code.contains("// pc | t9"), "{code}");
    let entry = Address::new(
        prog.arch()
            .manage()
            .get_default_code_space()
            .unwrap()
            .clone(),
        0x400f04,
    );
    assert!(prog.arch().loader_mips_t9.as_ref().unwrap()(&entry).is_none());
    let entry = prog.resolve_entry(&EntrySelector::Numeric(0x40057c)).unwrap();
    let result = decompile_targets(&mut prog, vec![entry], false, false, false);
    assert!(result[0].error.is_none(), "{:?}", result[0].error);
    let code = result[0].code.as_ref().unwrap();
    assert!(code.contains("if (") && code.contains("41208c"), "unresolved weak call was folded away: {code}");
}

#[test]
fn explicit_octeon_targets_override_conflicting_or_missing_elf_profiles() {
    use kuna_analysis::loadimage_object::ObjectLoadImage;
    use kuna_sleigh::loadimage::LoadImage;
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    for abi in ["n32", "64"] {
        let n32 = abi == "n32";
        let target = format!(
            "MIPS:BE:64:octeon3{}:hard",
            if n32 { "-32addr" } else { "" }
        );
        let original =
            std::fs::read(root.join(format!("tests/fixtures/octeon-elf-abi/abi-gcc73-{abi}.elf")))
                .unwrap();
        let mut conflict = original.clone();
        let offset = if n32 { 36 } else { 48 };
        let flags = u32::from_be_bytes(conflict[offset..offset + 4].try_into().unwrap());
        conflict[offset..offset + 4]
            .copy_from_slice(&((flags & !0xff0000) | 0x8d0000).to_be_bytes());
        assert!(ObjectLoadImage::from_bytes_silent("conflict", &conflict)
            .err()
            .unwrap()
            .to_string()
            .contains("conflicting Octeon"));
        let selected =
            ObjectLoadImage::from_bytes_with_target("conflict", &conflict, &target).unwrap();
        assert_eq!(selected.get_arch_type(), target.as_bytes());

        let mut legacy =
            std::fs::read(root.join(format!("tests/fixtures/octeon-elf-abi/abi-gcc47-{abi}.elf")))
                .unwrap();
        if n32 {
            legacy[32..36].fill(0);
            legacy[48..52].fill(0);
        } else {
            legacy[40..48].fill(0);
            legacy[60..64].fill(0);
        }
        assert!(ObjectLoadImage::from_bytes_silent("missing", &legacy)
            .err()
            .unwrap()
            .to_string()
            .contains("floating-point ABI is missing"));
        let selected =
            ObjectLoadImage::from_bytes_with_target("missing", &legacy, &target).unwrap();
        assert_eq!(selected.get_arch_type(), target.as_bytes());
    }
}

#[test]
fn aggregates_join_registers_and_keep_leading_return_bytes() {
    for n32 in [true, false] {
        let prog = program("octeon", n32);
        for before in [0, 1, 6, 7, 8, 9] {
            for (ty, size) in [("struct Pair", 16), ("struct Twelve", 12)] {
                let assigned = allocate(&prog, ty, before);
                let expected = if before < 8 {
                    vec![slot(&prog, before, 8), slot(&prog, before + 1, size - 8)]
                } else {
                    vec![slot(&prog, before, size)]
                };
                assert_eq!(
                    storage(&prog, &assigned[before + 1]),
                    expected,
                    "n32={n32} {ty}, slot {before}"
                );
                assert_eq!(
                    storage(&prog, assigned.last().unwrap()),
                    vec![slot(&prog, before + 2, 8)]
                );
                assert_eq!(
                    storage(&prog, &assigned[0]),
                    vec![reg(&prog, "v0", 0, 8), reg(&prog, "v1", 0, size - 8)]
                );
            }
        }
        for ty in ["struct Small", "union SmallUnion"] {
            let assigned = allocate(&prog, ty, 0);
            assert_eq!(storage(&prog, &assigned[1]), vec![reg(&prog, "a0", 0, 4)]);
            assert_eq!(storage(&prog, &assigned[0]), vec![reg(&prog, "v0", 0, 4)]);
        }
    }
}

#[test]
fn quad_alignment_and_return_pair_are_type_specific() {
    for n32 in [true, false] {
        let prog = program("octeon", n32);
        for ty in ["long double", "struct Quad", "int128_t"] {
            for before in [0, 1, 6, 7, 8, 9] {
                let assigned = allocate(&prog, ty, before);
                assert_eq!(
                    assigned[before + 1].type_.as_ref().unwrap().get_alignment(),
                    16
                );
                let first = (before + 1) & !1;
                let expected = if first < 8 {
                    vec![slot(&prog, first, 8), slot(&prog, first + 1, 8)]
                } else {
                    vec![slot(&prog, first, 16)]
                };
                assert_eq!(
                    storage(&prog, &assigned[before + 1]),
                    expected,
                    "n32={n32} {ty}, slot {before}"
                );
                assert_eq!(
                    storage(&prog, assigned.last().unwrap()),
                    vec![slot(&prog, first + 2, 8)]
                );
                assert_eq!(
                    storage(&prog, &assigned[0]),
                    vec![
                        reg(&prog, "v0", 0, 8),
                        reg(&prog, if ty == "long double" { "a0" } else { "v1" }, 0, 8)
                    ]
                );
            }
        }
        let layout = allocate(&prog, "struct Layout", 0);
        assert_eq!(
            layout[layout.len() - 2].type_.as_ref().unwrap().get_size(),
            32
        );
    }
}

#[test]
fn n32_small_arguments_keep_type_specific_padding() {
    let prog = program("octeon", true);
    for before in [0, 7, 8, 9] {
        for (ty, reg_pad, stack_pad) in [
            ("float", 4, 0),
            ("unsigned int", 4, 4),
            ("struct Small", 0, 0),
            ("union SmallUnion", 0, 0),
        ] {
            let assigned = allocate(&prog, ty, before);
            let mut expected = slot(&prog, before, 4);
            expected.1 += if before < 8 { reg_pad } else { stack_pad };
            assert_eq!(
                storage(&prog, &assigned[before + 1]),
                vec![expected],
                "{ty} slot {before}"
            );
            assert_eq!(
                storage(&prog, assigned.last().unwrap()),
                vec![slot(&prog, before + 1, 8)]
            );
        }
    }
}

#[test]
fn independent_gcc_prototypes_recover_aggregate_and_quad_values() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    for variant in ["octeon", "octeonplus", "octeon2", "octeon3"] {
        for abi in ["n32", "64"] {
            let target = format!(
                "MIPS:BE:64:{variant}{}:default",
                if abi == "n32" { "-32addr" } else { "" }
            );
            let mut prog = bootstrap_from_object(
                root.join(format!("tests/fixtures/octeon-abi/arguments-{abi}.elf"))
                    .to_str()
                    .unwrap(),
                &target,
                &[root.join("specs").to_string_lossy().into_owned()],
            )
            .unwrap();
            prog.arch_mut().set_kuna_option("ctypes", "on").unwrap();
            prog.commit_pending_analysis().unwrap();
            for (name, expr) in [
                ("pair_first", "return p.a;"),
                ("pair_second", "return p.b;"),
                ("small_field", "return s.x;"),
                ("small_result", ".from = x"),
                ("union_result", "return x;"),
                ("pair_identity", "return p;"),
                ("float_identity", "return f;"),
                ("quad_after", "return c;"),
                ("quad_identity", "return b;"),
            ] {
                let entry = prog
                    .resolve_entry(&EntrySelector::Name(name.into()))
                    .unwrap();
                let result = decompile_targets(&mut prog, vec![entry], false, false, false);
                let code = result[0].code.as_ref().unwrap();
                assert!(code.contains(expr), "{target} {name}: {code}");
            }
            if abi == "64" {
                for name in ["quad_last_after", "quad_stack_after"] {
                    let entry = prog
                        .resolve_entry(&EntrySelector::Name(name.into()))
                        .unwrap();
                    let result = decompile_targets(&mut prog, vec![entry], false, false, false);
                    let code = result[0].code.as_ref().unwrap();
                    assert!(code.contains("return value;"), "{target} {name}: {code}");
                }
            }
        }
    }
}
