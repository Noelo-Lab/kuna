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
        "outgoing_only",
        "outgoing_definition",
        "outgoing_rmw",
        "outgoing_local_use",
        "outgoing_indirect",
        "durable_call_result",
        "stack_outgoing",
        "outgoing_independent_write",
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
        declaration_count, 24,
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
fn parameters_have_no_storage_comments() {
    for (function, setup, parameter_name) in [
        ("parameter_source", vec!["option namestyle angr"], "a0"),
        (
            "parameter_source",
            vec![
                "option namestyle angr",
                "parse line extern int4 parameter_source(int4 *cursor);",
            ],
            "cursor",
        ),
        ("cmov_source", vec!["option namestyle angr"], "a0"),
    ] {
        let output = decompile(function, &setup);
        assert!(
            output.contains(&format!("{function}(")),
            "the function must render:\n{output}"
        );
        assert!(
            output.contains(parameter_name),
            "the parameter must render:\n{output}"
        );
        for parameter in ["a0", "a1", "a2", "cursor"] {
            assert!(
                !output
                    .lines()
                    .any(|line| line.trim_start().starts_with(&format!("// {parameter}:"))),
                "parameter {parameter} must not have a source comment:\n{output}"
            );
        }
    }
}

#[test]
fn outgoing_argument_copies_do_not_add_local_homes() {
    for function in ["outgoing_only", "outgoing_indirect"] {
        let output = decompile(function, &["option namestyle angr"]);
        assert!(
            local_declarations(&output)
                .iter()
                .any(|line| line.ends_with("v1; // r14d")),
            "{function}'s argument-only ESI copy must not become a local home:\n{output}"
        );
        assert!(
            output.contains("v1)"),
            "the call must pass the local:\n{output}"
        );
    }
    for function in ["outgoing_definition", "outgoing_rmw", "outgoing_local_use"] {
        let output = decompile(function, &["option namestyle angr"]);
        assert!(
            local_declarations(&output)
                .iter()
                .any(|line| line.ends_with("v1; // esi | r14d")),
            "{function}'s genuine ESI definition or local use must remain a home:\n{output}"
        );
    }
}

#[test]
fn durable_call_results_keep_callee_saved_homes() {
    let output = decompile("durable_call_result", &["option namestyle angr"]);
    assert!(
        local_declarations(&output).iter().any(|line| line.ends_with("v1; // eax | r14d")),
        "R14D remains a durable local home even when it only forwards to outgoing ESI copies:\n{output}"
    );
    assert_eq!(
        output.matches("call_sink(").count(),
        2,
        "the saved result must cross two calls:\n{output}"
    );
}

#[test]
fn stack_reloads_used_only_as_arguments_do_not_add_register_homes() {
    let output = decompile("stack_outgoing", &["option namestyle angr"]);
    assert!(
        local_declarations(&output).iter().any(|line| line.ends_with("v1; // stack - 0x9")),
        "the outgoing ESI reload must not add ESI or SIL to the helper-written stack local:\n{output}"
    );
    assert!(
        output.contains("write_byte(&v1)"),
        "the helper must write the stack local:\n{output}"
    );
    assert!(
        output.contains("call_sink(a0,"),
        "the reloaded byte must be passed to the call:\n{output}"
    );
}

#[test]
fn independent_computation_after_argument_setup_keeps_its_register_home() {
    let output = decompile("outgoing_independent_write", &["option namestyle angr"]);
    assert!(
        local_declarations(&output).iter().any(|line| line.ends_with("v1; // esi | r14d")),
        "an earlier argument-only ESI write must not hide a later independent value's ESI home:\n{output}"
    );
    assert!(
        output.contains("call_sink(a0,*a0)"),
        "the first ESI write must only prepare an argument:\n{output}"
    );
    assert!(
        output.contains("v1 = (a0[4] * 5 ^ 0x2468aceU) + 1"),
        "the second ESI write must compute an independent local:\n{output}"
    );
    assert_eq!(
        output.matches("call_sink(").count(),
        2,
        "both values must be passed to calls:\n{output}"
    );
}
