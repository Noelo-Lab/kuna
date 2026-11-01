//! Compiler-derived BE N32/N64 storage contracts, including register exhaustion.
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
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let mut docs = DocumentStorage::new();
    let image = docs.parse_document(br#"<binaryimage arch="MIPS:BE:64:octeon"><bytechunk space="ram" offset="0x1000">03e0000800000000</bytechunk></binaryimage>"#).unwrap().get_root().clone();
    let target = format!(
        "MIPS:BE:64:{variant}{}:default",
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
    prog
}

fn allocate(prog: &ConsoleProgram, datatype: &str, before: usize) -> Vec<ParameterPieces> {
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
