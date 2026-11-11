//! N64 soft-float placement from GCC mips_pad_reg_upward/mips_pad_arg_upward.
use std::{path::PathBuf, rc::Rc};

use kuna_base::xml::DocumentStorage;
use kuna_console::{
    engine::{bootstrap_from_object, bootstrap_program, ConsoleProgram, EntrySelector},
    grammar::{parse_c, DataOrg},
    project::decompile_targets,
};
use kuna_decomp::fspec::ParameterPieces;

fn program(variant: &str) -> ConsoleProgram {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let mut store = DocumentStorage::new();
    let image = store.parse_document(b"<binaryimage arch=\"MIPS:BE:64:octeon:default\"><bytechunk space=\"ram\" offset=\"0x1000\">03e0000800000000</bytechunk></binaryimage>").unwrap().get_root().clone();
    let prog = bootstrap_program(
        image,
        &format!("MIPS:BE:64:{variant}:default"),
        &[root.join("specs").to_string_lossy().into_owned()],
    )
    .unwrap();
    let (addr_size, word_size) = prog.arch().data_org();
    for declaration in [
        "struct One { unsigned char x; };",
        "struct Two { unsigned short x; };",
        "struct Four { unsigned int x; };",
        "struct Eight { unsigned int x, y; };",
        "union Word { unsigned int x; float f; };",
    ] {
        parse_c(
            declaration,
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

fn allocate(
    prog: &ConsoleProgram,
    datatype: &str,
    before: usize,
    variadic: bool,
) -> Vec<ParameterPieces> {
    let mut arguments: Vec<_> = (0..before).map(|i| format!("unsigned long p{i}")).collect();
    arguments.push(format!("{datatype} value"));
    arguments.push("unsigned long after".into());
    let declaration = format!("extern {datatype} sample({});", arguments.join(","));
    let arch = prog.arch();
    let (addr_size, word_size) = arch.data_org();
    let mut result = Vec::new();
    parse_c(
        &declaration,
        arch.types(),
        DataOrg {
            addr_size,
            word_size,
        },
        &[],
        |mut pieces, _| {
            pieces.first_var_arg_slot = if variadic { 0 } else { -1 };
            arch.default_fp().unwrap().assign_parameter_storage(
                &pieces,
                &mut result,
                false,
                arch.types(),
                arch.manage(),
            )
        },
    )
    .unwrap();
    assert_location(prog, result.last().unwrap(), before + 1, 0, 0);
    result
}

#[test]
fn fixed_and_variadic_aggregates_keep_their_leading_bytes() {
    for variant in ["octeon", "octeonplus", "octeon2", "octeon3"] {
        let prog = program(variant);
        for datatype in [
            "struct One",
            "struct Two",
            "struct Four",
            "struct Eight",
            "union Word",
        ] {
            for before in [0, 1, 7, 8, 9] {
                for variadic in [false, true] {
                    let assigned = allocate(&prog, datatype, before, variadic);
                    assert_location(&prog, &assigned[before + 1], before, 0, 0);
                    let v0 = prog.arch().translate().get_register_varnode(b"v0").unwrap();
                    assert_eq!(
                        assigned[0].addr.get_offset(),
                        v0.offset,
                        "{variant}: {datatype} return must stay left justified"
                    );
                }
            }
        }
    }
}

fn assert_location(
    prog: &ConsoleProgram,
    parameter: &ParameterPieces,
    before: usize,
    register_pad: u64,
    stack_pad: u64,
) {
    if before < 8 {
        let name = ["a0", "a1", "a2", "a3", "t0", "t1", "t2", "t3"][before];
        let register = prog
            .arch()
            .translate()
            .get_register_varnode(name.as_bytes())
            .unwrap();
        assert!(Rc::ptr_eq(
            parameter.addr.get_space().unwrap(),
            register.space.as_ref().unwrap()
        ));
        assert_eq!(
            parameter.addr.get_offset(),
            register.offset + register_pad,
            "{name} argument placement"
        );
    } else {
        assert_eq!(parameter.addr.get_space().unwrap().get_name(), "stack");
        assert_eq!(
            parameter.addr.get_offset(),
            (before as u64 - 8) * 8 + stack_pad,
            "stack argument placement"
        );
    }
}

#[test]
fn float_register_and_stack_justification_are_distinct() {
    for variant in ["octeon", "octeonplus", "octeon2", "octeon3"] {
        let prog = program(variant);
        for (datatype, register_pad, stack_pad) in
            [("float", 4, 0), ("double", 0, 0), ("unsigned int", 4, 4)]
        {
            for before in [0, 1, 7, 8, 9] {
                let assigned = allocate(&prog, datatype, before, false);
                assert_location(
                    &prog,
                    &assigned[before + 1],
                    before,
                    register_pad,
                    stack_pad,
                );
                let v0 = prog.arch().translate().get_register_varnode(b"v0").unwrap();
                assert_eq!(assigned[0].addr.get_offset(), v0.offset + register_pad);
            }
        }
        for before in [1, 7, 8, 9] {
            let assigned = allocate(&prog, "double", before, true);
            assert_location(&prog, &assigned[before + 1], before, 0, 0);
        }
    }
}

#[test]
fn gcc_dwarf_parameters_reach_the_returned_expression() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let fixture = root.join("tests/fixtures/octeon-n64-arguments/arguments.elf");
    for variant in ["octeon", "octeonplus", "octeon2", "octeon3"] {
        let mut prog = bootstrap_from_object(
            fixture.to_str().unwrap(),
            &format!("MIPS:BE:64:{variant}:default"),
            &[root.join("specs").to_string_lossy().into_owned()],
        )
        .unwrap();
        prog.arch_mut().set_kuna_option("ctypes", "on").unwrap();
        prog.commit_pending_analysis().unwrap();
        for (name, expression) in [
            ("get_one", "(unsigned int)value.x"),
            ("get_two", "(unsigned int)value.x"),
            ("get_four", "value.x"),
            ("get_eight", "value.x"),
            ("get_union", "value.x"),
            ("four_last", "value.x"),
            ("four_stack", "value.x"),
            ("float_identity", "value"),
            ("float_second", "value"),
            ("float_last", "value"),
            ("float_stack", "value"),
            ("float_stack_second", "value"),
            ("double_identity", "value"),
            ("uint_stack", "value"),
        ] {
            let entry = prog
                .resolve_entry(&EntrySelector::Name(name.into()))
                .unwrap();
            let functions = decompile_targets(&mut prog, vec![entry], false, false, false);
            assert!(
                functions[0].error.is_none(),
                "{variant}: {name}: {:?}",
                functions[0].error
            );
            let code = functions[0].code.as_ref().expect("emitted C");
            assert!(
                code.contains(&format!("return {expression};")),
                "{variant}: {name}:\n{code}"
            );
        }
    }
}
