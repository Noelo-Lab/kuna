//! A managed .NET assembly is read from its CIL method bodies: the inventory
//! is its methods, each as long as its body, and `cil off` or an explicit
//! target reads the PE's native stub instead.

use crate::common;

const SUM: &str = "Program::<<Main>$>g__Sum|0_0";

fn hello() -> String {
    common::fixture("cil_hello.dll")
}

#[test]
fn the_inventory_is_the_method_bodies_at_their_own_sizes() {
    let (out, err, code) = common::run_kuna(&["functions", &hello()]);
    assert_eq!(code, 0, "{err}");
    assert_eq!(
        out,
        "0x402051\tProgram::<Main>$\n0x40206e\tProgram::.ctor\n0x402084\tProgram::<<Main>$>g__Sum|0_0\n"
    );
    assert_eq!(err, "", "nothing sends the reader to another tool");

    let (json, err, code) = common::run_kuna(&["functions", &hello(), "--json"]);
    assert_eq!(code, 0, "{err}");
    assert_eq!(json.matches("\"size\": 28").count(), 2, "the entry point and the local function\n{json}");
    assert_eq!(json.matches("\"size\": 7").count(), 1, "the constructor, not the gap up to the next header\n{json}");
}

#[test]
fn a_method_decompiles_to_its_loop_and_its_caller_names_it() {
    let (out, err, code) = common::run_kuna(&["decompile", &hello(), SUM]);
    assert_eq!(code, 0, "{err}");
    assert!(out.starts_with("int Program::<<Main>$>g__Sum|0_0(int *a)\n"), "{out}");
    let body = "  for (v1 = 0; v1 < (int)ldlen(a); v1 = v1 + 1) {\n    v2 += a[v1];\n  }\n  return v2;\n";
    assert!(out.contains(body), "{out}");

    let (out, err, code) = common::run_kuna(&["decompile", &hello(), "0x402051", "--addr"]);
    assert_eq!(code, 0, "{err}");
    assert!(out.contains("  System.Console::WriteLine(Program::<<Main>$>g__Sum|0_0(a));\n"), "{out}");
}

#[test]
fn a_method_lists_only_its_own_instructions() {
    let (out, err, code) = common::run_kuna(&["disassemble", &hello(), "0x40206e", "--addr"]);
    assert_eq!(code, 0, "{err}");
    assert!(out.starts_with("# 3 instructions at Program::.ctor @ 0x40206e (0x40206e..0x402075, 7 bytes)\n"), "{out}");
    assert_eq!(err, "", "a stated method end is not a flow that stopped short");
}

#[test]
fn cil_off_and_an_explicit_target_read_the_native_stub() {
    let binary = hello();
    let views: [&[&str]; 2] = [&["--option", "cil", "off"], &["--target", "x86:LE:32:default"]];
    for view in views {
        let mut args = vec!["functions", binary.as_str()];
        args.extend(view);
        let (out, err, code) = common::run_kuna(&args);
        assert_eq!(code, 0, "{view:?}: {err}");
        assert!(out.contains("_CorExeMain") && !out.contains("Program::"), "{view:?}: {out}");
        assert!(err.contains("native decompilation shows only the stub"), "{view:?}: {err}");
    }
}
