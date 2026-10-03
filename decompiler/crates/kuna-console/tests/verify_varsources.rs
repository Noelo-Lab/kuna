//! Machine-storage comments survive copy elimination, merges, and naming.

use std::path::PathBuf;

use kuna_console::engine::{bootstrap_from_file, ConsoleProgram};
use kuna_console::ifacedecomp::{
    execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE,
};
use kuna_console::ifaceterm::ConsoleCommands;

fn bootstrap() -> ConsoleProgram {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let fixture = root.join("tests/stages/kuna-varsources.xml");
    let specs = root.join("specs");
    bootstrap_from_file(
        fixture.to_str().unwrap(),
        "",
        &[specs.to_str().unwrap().to_string()],
    )
    .expect("bootstrap variable-source fixture with built processor specs")
}

fn decompile(name: &str, setup: &[&str]) -> String {
    let mut commands: Vec<String> = setup.iter().map(|s| s.to_string()).collect();
    commands.extend([
        format!("load function {name}"),
        "decompile".into(),
        "print C".into(),
    ]);
    let count = commands.len();
    let mut status = ConsoleCommands::into_status(commands);
    register_decomp_commands(&mut status);
    let data = status.get_data_mut(DECOMPILE_MODULE).unwrap();
    data.as_any_mut()
        .downcast_mut::<IfaceDecompData>()
        .unwrap()
        .conf = Some(bootstrap());
    for _ in 0..count {
        execute(&mut status);
    }
    status.optr.clone()
}

fn local_declarations(output: &str) -> Vec<&str> {
    output
        .lines()
        .filter(|line| {
            let line = line.trim();
            let first = line.split_whitespace().next().unwrap_or("");
            ["char", "int", "int4", "uint1", "uint4", "int8", "unsigned"].contains(&first)
                && line.contains(';')
                && !line.contains('(')
        })
        .collect()
}

#[test]
fn every_fixture_local_has_a_source() {
    let mut declaration_count = 0;
    for function in [
        "copy_home",
        "loop_home",
        "merged_homes",
        "arithmetic_home",
        "constant_home",
        "joined_home",
        "joined_helper",
        "stack_home",
        "byte_source",
        "cmov_source",
    ] {
        let output = decompile(function, &["option namestyle angr"]);
        let declarations = local_declarations(&output);
        assert!(
            !declarations.is_empty(),
            "{function} must exercise local declarations:\n{output}"
        );
        declaration_count += declarations.len();
        for declaration in declarations {
            let (_, source) = declaration.split_once("; // ").unwrap_or_else(|| {
                panic!("{function} has a local with no source: {declaration}\n{output}")
            });
            assert!(
                !source.trim().is_empty(),
                "{function} has an empty source:\n{output}"
            );
        }
    }
    assert_eq!(
        declaration_count, 16,
        "all synthetic locals must be checked"
    );
}

#[test]
fn comments_describe_value_homes_without_borrowing_arithmetic_inputs() {
    let cases: &[(&str, &[&str])] = &[
        ("copy_home", &["v1; // eax"]),
        (
            "loop_home",
            &["v1; // ecx", "v2; // eax | r8d", "v3; // r9d"],
        ),
        ("merged_homes", &["v1; // eax | ebx"]),
        ("arithmetic_home", &["v1; // eax", "v2; // ecx"]),
        ("constant_home", &["v1; // eax | ebx"]),
        ("joined_home", &["v1 [16]; // rdx:rax"]),
        ("joined_helper", &["v1 [16]; // rdx:rax"]),
        (
            "byte_source",
            &["v1; // r15b | r8b", "v2; // cl", "v3; // cl", "v4; // cl"],
        ),
        ("cmov_source", &["v1; // eax | r13d"]),
    ];
    for &(function, expected) in cases {
        let output = decompile(function, &["option namestyle angr"]);
        for &expected in expected {
            assert!(
                output.lines().any(|line| line.ends_with(expected)),
                "{function} must contain declaration ending {expected:?}:\n{output}"
            );
        }
    }
}

#[test]
fn ghidra_naming_omits_storage_comments() {
    let output = decompile("arithmetic_home", &["option namestyle ghidra"]);
    let declarations = local_declarations(&output);
    assert_eq!(
        declarations.len(),
        2,
        "the control must have both locals:\n{output}"
    );
    assert!(
        declarations.iter().all(|line| !line.contains("//")),
        "Ghidra naming preserves its declaration presentation:\n{output}"
    );
}

#[test]
fn copied_parameters_keep_abi_and_register_homes() {
    let output = decompile("parameter_source", &["option namestyle angr"]);
    assert!(
        output.lines().any(|line| line.trim() == "// a0: rbx | rdi"),
        "the parameter must retain both its ABI and copied register homes:\n{output}"
    );
    let output = decompile(
        "parameter_source",
        &[
            "option namestyle angr",
            "parse line extern int4 parameter_source(int4 *cursor);",
        ],
    );
    assert!(
        output
            .lines()
            .any(|line| line.trim() == "// cursor: rbx | rdi"),
        "an explicit parameter name must retain the same homes:\n{output}"
    );
    let output = decompile("cmov_source", &["option namestyle angr"]);
    assert!(
        output
            .lines()
            .any(|line| line.trim() == "// a0: edi | r13d"),
        "the arithmetic input has different homes from the CMOV result:\n{output}"
    );
}
