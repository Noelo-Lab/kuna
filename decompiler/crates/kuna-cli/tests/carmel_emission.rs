//! Compile and execute emitted C for original-witnessed Carmel issues.
#![cfg(all(target_os = "linux", target_pointer_width = "64"))]

use crate::common;
use serde_json::Value;
use std::{fmt::Write as _, fs, path::Path, process::Command, time::Duration};

#[test]
fn carmel_odd_eq_has_two_delay_issues_and_skips_the_rest_only_when_taken() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    // INC A0L ; GOTO EQ1 +11 delay 2. The two delayed increments affect A1L
    // and A2L; A3L is the first fallthrough-only result. RETURN is at word 12.
    let code = [
        0xa1, 0xac, 0x20, 0x28, 0x1c, 0x2e,
        0x21, 0xac, 0x62, 0x2b, 0xec, 0,
        0x21, 0xac, 0xa4, 0x2b, 0xec, 0,
        0x21, 0xac, 0xe6, 0x2b, 0xec, 0,
        0x21, 0xad, 0x28, 0x2b, 0xec, 0,
        0x21, 0xad, 0x6a, 0x2b, 0xec, 0,
        0x2b, 0xd8, 0, 0x2b, 0xec, 0,
    ];
    let binary = common::scratch_file("carmel-conditional-delay", "bin");
    fs::write(&binary, code).unwrap();
    let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
    kuna.arg("decompile").arg(&binary).args([
        "0", "--addr", "--raw-image", "--target", "Carmel:BE:24:default:default", "--base", "0",
        "--json", "--assert-strict", "--assert", "return %A3L unsigned short",
    ]);
    for (index, reg) in ["A0L", "A1L", "A2L", "A3L", "A4L", "A5L", "EST", "ECFG"].into_iter().enumerate() {
        kuna.args(["--assert", &format!("param {index} %{reg} unsigned short in_{}", reg.to_ascii_lowercase())]);
    }
    let output = common::process::required_output(&mut kuna);
    let json: Value = serde_json::from_slice(&output.stdout).unwrap();
    let function = &json["functions"][0];
    assert!(function["error"].is_null(), "{function}");
    assert_eq!(function["name"], "sub_0");
    let emitted = function["code"].as_str().unwrap();
    let source = binary.with_extension("c");
    let executable = binary.with_extension("exe");
    fs::write(&source, format!(
        "#include <stdbool.h>\n#include <stdint.h>\n{emitted}\n\
         int main(void) {{\n\
           if (sub_0(0, 0, 0, 0, 0, 0, 0, 0) != 1) return 1;\n\
           if (sub_0(0xffff, 0, 0, 0, 0, 0, 0, 0) != 0) return 2;\n\
           return 0;\n\
         }}\n"
    )).unwrap();
    run_emitted_c(&source, &executable, emitted);
    for file in [binary, source, executable] {
        fs::remove_file(file).unwrap();
    }
}

#[test]
fn carmel_delay_refuses_unmodeled_data_order_without_discarding_the_slot() {
    let nop = [0x2b, 0xec, 0];
    let store = [[0x80, 0x56, 0x78], [0xc6, 0x40, 0]];
    let binary = common::scratch_file("carmel-delay-data-order", "bin");
    let goto = [0x2b, 0x00, 0x52];
    let repeat = [0x2b, 0xc0, 0x03];
    let call = [0x28, 0x82, 0x61];

    for (owner, slot, expected) in [
        (goto, store, &["STORE", "BRANCH"][..]),
        (call, [nop; 2], &["STORE", "STORE", "CBRANCH", "CALL", "BRANCH"][..]),
        (repeat, store, &[][..]),
        (call, store, &[][..]),
    ] {
        let mut code = [[nop; 2]; 16];
        code[0] = [owner, nop];
        code[1] = slot;
        fs::write(&binary, code.as_flattened().as_flattened()).unwrap();
        let (stdout, stderr, status) = common::run_kuna(&[
            "raw-pcode", binary.to_str().unwrap(), "0", "--raw-image", "--target",
            "Carmel:BE:24:default:default", "--base", "0", "--count", "1", "--json",
        ]);
        let doc: Value = serde_json::from_str(&stdout).unwrap_or_else(|_| panic!("{stdout}\n{stderr}"));
        let instructions = doc["instructions"].as_array().unwrap();
        if expected.is_empty() {
            assert_eq!(status, 1, "{doc}");
            assert_eq!(doc["error"]["address"], 0, "{doc}");
            assert!(instructions.is_empty(), "{doc}");
        } else {
            assert_eq!(status, 0, "{doc}\n{stderr}");
            let [instruction] = instructions.as_slice() else { panic!("{doc}"); };
            let ops: Vec<_> = instruction["ops"].as_array().unwrap().iter().filter_map(|op| {
                let opcode = op["opcode"].as_str().unwrap();
                matches!(opcode, "LOAD" | "STORE" | "BRANCH" | "CBRANCH" | "CALL").then_some(opcode)
            }).collect();
            assert_eq!(ops, expected, "{doc}");
        }
    }
    fs::remove_file(binary).unwrap();
}

#[test]
fn carmel_backtrace_projections_preserve_the_other_lane_and_the_vetoed_write() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let binary = common::scratch_file("carmel-backtrace", "bin");
    let source = binary.with_extension("c");
    let executable = binary.with_extension("exe");
    let mut generated = String::from("#include <stdbool.h>\n#include <stdint.h>\n");
    let mut main = String::from("int main(void) {\n");
    for (case, (fetch, expected, predicated)) in [
        ([0x2da022u32, 0x2bec00], [0, 0xbf32], false),
        ([0x2bec00, 0x2dbffe], [0xacdb, 0], false),
        ([0xabec00, 0x2da022], [0, 0xbf32], false),
        ([0x2da003, 0x2bec00], [0xacdb, 0xbf32], true),
    ].into_iter().enumerate() {
        let mut code = [0; 12];
        for (dst, word) in code.chunks_exact_mut(3)
            .zip(fetch.into_iter().chain([0x2bd800, 0x2bec00]))
        {
            dst.copy_from_slice(&word.to_be_bytes()[1..]);
        }
        fs::write(&binary, code).unwrap();
        let params = ["BTR1", "BTR2", "CECFG1", "EST", "CETBL"];
        for (port, want) in expected.into_iter().enumerate() {
            let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
            kuna.arg("decompile").arg(&binary).args([
                "0", "--addr", "--raw-image", "--target", "Carmel:BE:24:default:default", "--base", "0",
                "--json", "--assert-strict", "--assert", &format!("return %{} unsigned short", params[port]),
            ]);
            for (index, reg) in params[..if predicated { 5 } else { 2 }].iter().enumerate() {
                kuna.args(["--assert", &format!("param {index} %{reg} unsigned short in_{}", reg.to_ascii_lowercase())]);
            }
            let output = common::process::required_output(&mut kuna);
            let json: Value = serde_json::from_slice(&output.stdout).unwrap();
            let function = &json["functions"][0];
            assert!(function["error"].is_null(), "{function}");
            assert_eq!(function["name"], "sub_0");
            let name = format!("clrbtr_{case}_{port}");
            let code = function["code"].as_str().unwrap();
            writeln!(generated, "#define sub_0 {name}\n{code}\n#undef sub_0\n").unwrap();
            let extra = if predicated { ", 0, 0, 0" } else { "" };
            writeln!(main, "if ({name}(0xacdb, 0xbf32{extra}) != {want}) return {};", 1 + 2 * case + port).unwrap();
        }
    }
    generated.push_str(&main);
    generated.push_str("return 0;\n}\n");
    fs::write(&source, &generated).unwrap();
    run_emitted_c(&source, &executable, &generated);
    for file in [binary, source, executable] {
        fs::remove_file(file).unwrap();
    }
}

#[test]
fn carmel_wide_logic_writes_saved_loop_count_without_replacing_active_count() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let binary = common::scratch_file("carmel-wide-logic-loop-count", "bin");
    fs::write(&binary, [
        0xe0, 0x72, 0x25, 0xeb, 0x52, 0xae, // XOR LC1,RN3,R0
        0x2b, 0xd8, 0x00, 0x2b, 0xec, 0x00,
    ]).unwrap();
    let source = binary.with_extension("c");
    let executable = binary.with_extension("exe");
    let mut emitted = String::new();
    for (register, name) in [("LC1", "saved_count"), ("LC", "active_count")] {
        let output = common::process::required_output(Command::new(env!("CARGO_BIN_EXE_kuna"))
            .arg("decompile").arg(&binary).args([
                "0", "--addr", "--raw-image", "--target", "Carmel:BE:24:default:default",
                "--base", "0", "--json", "--assert-strict",
                "--assert", "param 0 %LC unsigned short active",
                "--assert", "param 1 %RN3 unsigned short step",
                "--assert", "param 2 %R0 unsigned short index",
                "--assert", &format!("return %{register} unsigned short"),
            ]));
        let json: Value = serde_json::from_slice(&output.stdout).unwrap();
        let function = &json["functions"][0];
        assert!(function["error"].is_null(), "{function}");
        emitted.push_str(&function["code"].as_str().unwrap().replace("sub_0", name));
        emitted.push('\n');
    }
    fs::write(&source, format!(
        "{emitted}\nint main(void) {{\n\
         return saved_count(0x1234, 0xe28, 0x8c44) != 0x826c ||\n\
                saved_count(0x1234, 0x8c44, 0x8c44) != 0 ||\n\
                active_count(0x1234, 0xe28, 0x8c44) != 0x1234;\n\
         }}\n",
    )).unwrap();
    run_emitted_c(&source, &executable, &emitted);
    for file in [binary, source, executable] {
        fs::remove_file(file).unwrap();
    }
}

#[test]
fn carmel_scalar_stack_pointer_returns_the_popped_word_address() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let binary = common::scratch_file("carmel-scalar-sp", "bin");
    fs::write(&binary, [0x2b, 0xd8, 0, 0x2b, 0xec, 0]).unwrap();
    let mut command = Command::new(env!("CARGO_BIN_EXE_kuna"));
    command.arg("decompile").arg(&binary).args([
        "0", "--addr", "--raw-image", "--target", "Carmel:BE:24:default:default",
        "--base", "0", "--json", "--assert-strict",
        "--assert", "param 0 %SP unsigned short input_sp",
        "--assert", "return %SP unsigned short",
    ]);
    let output = common::process::required_output(&mut command);
    let report: Value = serde_json::from_slice(&output.stdout).unwrap();
    let function = &report["functions"][0];
    assert!(function["error"].is_null(), "{function}");
    let emitted = function["code"].as_str().unwrap();
    let source = binary.with_extension("c");
    let executable = binary.with_extension("exe");
    fs::write(&source, format!("{emitted}\nint main(void) {{\n\
        return sub_0(0x4020) != 0x4022 || sub_0(0x4070) != 0x4072;\n\
    }}\n")).unwrap();
    run_emitted_c(&source, &executable, emitted);
    for file in [binary, source, executable] {
        fs::remove_file(file).unwrap();
    }
}

#[test]
fn carmel_data_repeat_replays_the_range_and_skips_only_its_first_issue_at_zero() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let code = [
        0xb8, 0x41, 0x00, 0xcf, 0x20, 0x00, // REP *0x4100 delay 4
        0x21, 0xac, 0x20, 0x2b, 0xec, 0x00, // setup: INC A0L
        0x21, 0xac, 0x62, 0x2b, 0xec, 0x00, //        INC A1L
        0x21, 0xac, 0xa4, 0x2b, 0xec, 0x00, //        INC A2L
        0x21, 0xac, 0xe6, 0x2b, 0xec, 0x00, // body:  INC A3L
        0x21, 0xad, 0x28, 0x2b, 0xec, 0x00, //        INC A4L
        0x2b, 0xd8, 0x00, 0x2b, 0xec, 0x00,
    ];
    let binary = common::scratch_file("carmel-data-repeat", "bin");
    fs::write(&binary, code).unwrap();
    for (output, zero, three) in [("A3L", 7, 10), ("A4L", 8, 10)] {
        let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
        kuna.arg("decompile").arg(&binary).args([
            "0", "--addr", "--raw-image", "--target", "Carmel:BE:24:default:default", "--base", "0",
            "--json", "--assert-strict",
            "--assert", "data [data,0x4100] volatile unsigned short repeat_count",
            "--assert", "param 0 %A3L unsigned short in_a3l",
            "--assert", "param 1 %A4L unsigned short in_a4l",
            "--assert", "param 2 %ECFG unsigned short in_ecfg",
            "--assert", &format!("return %{output} unsigned short"),
        ]);
        let result = common::process::required_output(&mut kuna);
        let json: Value = serde_json::from_slice(&result.stdout).unwrap();
        let function = &json["functions"][0];
        assert!(function["error"].is_null(), "{function}");
        let emitted = function["code"].as_str().unwrap();
        let source = binary.with_extension("c");
        let executable = binary.with_extension("exe");
        fs::write(&source, format!(
            "#include <stdbool.h>\nvolatile unsigned short repeat_count;\n{emitted}\n\
             int main(void) {{\n\
               repeat_count = 0;\n\
               if (sub_0(7, 7, 0) != {zero} || repeat_count != 0) return 1;\n\
               repeat_count = 3;\n\
               return sub_0(7, 7, 0) != {three} || repeat_count != 3;\n\
             }}\n"
        )).unwrap();
        run_emitted_c(&source, &executable, emitted);
        for file in [source, executable] {
            fs::remove_file(file).unwrap();
        }
    }
    fs::remove_file(binary).unwrap();
}

#[test]
fn carmel_postmodify_returns_the_word_index_not_the_host_object_address() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let code = [
        0x80, 0x41, 0x48, 0xc6, 0x42, 0x84, // MOVE R1,0x4148
        0x31, 0xdc, 0x92, 0x2b, 0xec, 0x00, // MOVE A2,*R1++RN1
        0x2b, 0xd8, 0x00, 0x2b, 0xec, 0x00,
    ];
    let binary = common::scratch_file("carmel-postmodify-word", "bin");
    fs::write(&binary, code).unwrap();
    let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
    kuna.arg("decompile").arg(&binary).args([
        "0", "--addr", "--raw-image", "--target", "Carmel:BE:24:default:default", "--base", "0",
        "--json", "--assert-strict", "--assert", "return %R1 unsigned short",
        "--assert", "data [data,0x4148] volatile unsigned short word_4148",
        "--assert", "param 0 %RN1 unsigned short step",
        "--assert", "param 1 %PACFG unsigned short cfg",
        "--assert", "param 2 %RM0 unsigned short bound",
        "--assert", "param 3 %RL0 unsigned short lower",
    ]);
    let output = common::process::required_output(&mut kuna);
    let json: Value = serde_json::from_slice(&output.stdout).unwrap();
    let function = &json["functions"][0];
    assert!(function["error"].is_null(), "{function}");
    let emitted = function["code"].as_str().unwrap();
    let source = binary.with_extension("c");
    let executable = binary.with_extension("exe");
    fs::write(&source, format!(
        "#include <stdbool.h>\nvolatile unsigned short word_4148 = 0x3333;\n{emitted}\n\
         int main(void) {{\n\
           if (sub_0(3, 0, 0, 0) != 0x414b) return 1;\n\
           return sub_0(0xffe8, 0, 0, 0) != 0x4130 || word_4148 != 0x3333;\n\
         }}\n"
    )).unwrap();
    run_emitted_c(&source, &executable, emitted);
    for file in [binary, source, executable] {
        fs::remove_file(file).unwrap();
    }
}

#[test]
fn carmel_predicated_call_keeps_its_encoded_resume_inside_the_caller() {
    let nop = [0x2b, 0xec, 0x00];
    let inc = [0x21, 0xac, 0x20];
    let ret = [0x2b, 0xd8, 0x00];
    let mut code = [[nop; 2]; 15];
    code[..6].copy_from_slice(&[
        [inc, nop], [[0x28, 0x82, 0x5b], nop], [inc, nop],
        [inc, nop], [inc, nop], [ret, nop],
    ]);
    code[12..].copy_from_slice(&[[inc, nop], [inc, nop], [ret, nop]]);
    let binary = common::scratch_file("carmel-call-resume", "bin");
    fs::write(&binary, code.as_flattened().as_flattened()).unwrap();
    let mut inventory = Command::new(env!("CARGO_BIN_EXE_kuna"));
    inventory.arg("functions").arg(&binary).args([
        "--raw-image", "--target", "Carmel:BE:24:default:default", "--base", "0x1000",
        "--entry", "0x1000", "--json",
    ]);
    let listing: Value =
        serde_json::from_slice(&common::process::required_output(&mut inventory).stdout).unwrap();
    let functions = listing["functions"].as_array().unwrap();
    let entries: Vec<_> = functions.iter().map(|f| f["address"].as_u64().unwrap()).collect();
    // Word 0x100a is the caller's saved link, not a second callee.
    assert_eq!(entries, [0x1000, 0x1018], "{listing}");

    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        fs::remove_file(binary).unwrap();
        return;
    }
    let emit = |entry| {
        let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
        kuna.arg("decompile").arg(&binary).args([
            entry, "--addr", "--raw-image", "--target", "Carmel:BE:24:default:default", "--base",
            "0x1000", "--json", "--define-function", "0x1018=fixture_callee", "--assert-strict",
            "--assert", "param 0 %A0L short input", "--assert", "param 1 %ECFG unsigned short ecfg",
            "--assert", "return %A0L short",
            "--assert", "param fixture_callee::0 %A0L short input",
            "--assert", "param fixture_callee::1 %ECFG unsigned short ecfg",
            "--assert", "return fixture_callee::%A0L short",
        ]);
        let output = common::process::required_output(&mut kuna);
        let json: Value = serde_json::from_slice(&output.stdout).unwrap();
        let function = &json["functions"][0];
        assert!(function["error"].is_null(), "{function}");
        function["code"].as_str().unwrap().to_owned()
    };
    let emitted = format!("{}\n{}", emit("0x1018"), emit("0x1000"));
    let source = binary.with_extension("c");
    let executable = binary.with_extension("exe");
    fs::write(&source, format!(
        "#include <stdbool.h>\n{emitted}\n\
         int main(void) {{ return sub_1000(0, 0) != 5 || sub_1000(-1, 0) != 3; }}\n"
    )).unwrap();
    run_emitted_c(&source, &executable, &emitted);
    for file in [binary, source, executable] {
        fs::remove_file(file).unwrap();
    }
}

#[test]
fn carmel_wide_logic_control_alias_writes_high_word_and_masks_repeat_depth() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let binary = common::scratch_file("carmel-wide-logic-control", "bin");
    let source = binary.with_extension("c");
    let executable = binary.with_extension("exe");
    for (operator, expected) in [(0xe2, [0x3450, 0xf1ab, 0x000b]),
                                 (0xe6, [0, 0, 0]),
                                 (0xea, [0x3450, 0xf1ab, 0x000b])] {
        fs::write(&binary, [
            0xc8, 0xa1, 0x3c, operator, 0x1b, 0x56, // PSTATUS, PSTATUS, A0
            0x2b, 0xd8, 0x00, 0x2b, 0xec, 0x00, // RETURN
        ]).unwrap();
        let output = common::process::required_output(Command::new(env!("CARGO_BIN_EXE_kuna"))
            .arg("decompile").arg(&binary).args([
                "0", "--addr", "--raw-image", "--target", "Carmel:BE:24:default:default",
                "--base", "0", "--json", "--assert-strict",
                "--assert", "return %PSTATUS unsigned short",
                "--assert", "param 0 %A0H unsigned short a_hi",
            ]));
        let json: Value = serde_json::from_slice(&output.stdout).unwrap();
        let function = &json["functions"][0];
        assert!(function["error"].is_null(), "{function}");
        let emitted = function["code"].as_str().unwrap();
        let [a, b, c] = expected;
        fs::write(&source, format!(
            "{emitted}\nint main(void) {{\n\
             return sub_0(0x3456) != {a} || sub_0(0xf1ab) != {b} || sub_0(0x000f) != {c};\n\
             }}\n",
        )).unwrap();
        run_emitted_c(&source, &executable, emitted);
    }
    for file in [binary, source, executable] {
        fs::remove_file(file).unwrap();
    }
}

#[test]
fn carmel_wide_logic_separates_conditional_results_from_address_updates() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let binary = common::scratch_file("carmel-wide-logic-ce", "bin");
    let source = binary.with_extension("c");
    let executable = binary.with_extension("exe");
    let params = ["INTBASE", "X1", "R6", "PACFG", "RM6", "RL6", "CECFG1", "CECFG2", "CETBL", "EST"];
    let mut emitted = String::from("#include <stdbool.h>\n#include <stdint.h>\n");
    for (instruction, projections) in [
        ([0x1e, 0x85, 0xa8, 0xe3, 0x5b, 0xe2], [("INTBASE", "intbase"), ("EST", "or_flags")]),
        ([0x5e, 0x68, 0x5e, 0xe7, 0x5f, 0xe1], [("R6", "pointer"), ("EST", "and_flags")]),
    ] {
        let mut program = instruction.to_vec();
        program.extend([0x2b, 0xd8, 0, 0x2b, 0xec, 0]);
        fs::write(&binary, program).unwrap();
        for (register, name) in projections {
            let mut command = Command::new(env!("CARGO_BIN_EXE_kuna"));
            command.arg("decompile").arg(&binary).args([
                "0", "--addr", "--raw-image", "--target", "Carmel:BE:24:default:default",
                "--base", "0", "--json", "--assert-strict",
                "--assert", &format!("return %{register} unsigned short"),
            ]);
            for (index, reg) in params.into_iter().enumerate() {
                command.args(["--assert", &format!("param {index} %{reg} unsigned short in_{}", reg.to_ascii_lowercase())]);
            }
            let output = common::process::required_output(&mut command);
            let json: Value = serde_json::from_slice(&output.stdout).unwrap();
            let function = &json["functions"][0];
            assert!(function["error"].is_null(), "{function}");
            emitted.push_str(&function["code"].as_str().unwrap().replace("sub_0", name));
            emitted.push('\n');
        }
    }
    emitted.push_str(
        "int main(void) {\n\
         if (intbase(0x3210,0x8001,0,0,0,0,0,0,0xffff,0x10) != 0x8001) return 1;\n\
         if (intbase(0x3210,0x8001,0,0,0,0,0,0,0,0x10) != 0x3210) return 2;\n\
         if (or_flags(0,0,0,0,0,0,0,0,0xffff,0x10) != 0x3008) return 3;\n\
         if (or_flags(0,0,0,0,0,0,0,0,0,0x10) != 0x10) return 4;\n\
         if (pointer(0,0,0x8048,0,7,0,0,0,0,0x10) != 0x8048) return 5;\n\
         if (pointer(0,0,0x8048,0,7,0,0,0x1000,0,0x10) != 0x8046) return 6;\n\
         if (pointer(0,0,0x8044,0x2000,7,0x8040,0,0,0xffff,0x10) != 0x804a) return 7;\n\
         if (pointer(0,0,0x8044,0x3000,7,0x8040,0,0,0xffff,0x10) != 0x8042) return 8;\n\
         if (and_flags(0,0,0,0,0,0,0,0,0xffff,0x10) != 0x3010) return 9;\n\
         return 0;\n\
         }\n",
    );
    fs::write(&source, &emitted).unwrap();
    run_emitted_c(&source, &executable, &emitted);
    for file in [binary, source, executable] {
        fs::remove_file(file).unwrap();
    }
}

#[test]
fn carmel_wide_logic_tests_the_full_saved_loop_address_before_word_writeback() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let binary = common::scratch_file("carmel-logic-loop-address", "bin");
    let source = binary.with_extension("c");
    let executable = binary.with_extension("exe");
    fs::write(&binary, [
        0x1e, 0x85, 0xa8, 0xe3, 0x5b, 0x8a, // OR CE1 INTBASE,LSA1,X1
        0x2b, 0xd8, 0, 0x2b, 0xec, 0,
    ]).unwrap();
    let mut emitted = String::from("#include <stdbool.h>\n#include <stdint.h>\n");
    for (reg, name) in [("INTBASE", "word"), ("EST", "flags")] {
        let output = common::process::required_output(Command::new(env!("CARGO_BIN_EXE_kuna"))
            .arg("decompile").arg(&binary).args([
                "0", "--addr", "--raw-image", "--target", "Carmel:BE:24:default:default",
                "--base", "0", "--json", "--assert-strict",
                "--assert", &format!("return %{reg} unsigned short"),
                "--assert", "param 0 %LSA1W unsigned long saved_start",
                "--assert", "param 1 %X1 unsigned short input_word",
                "--assert", "param 2 %INTBASE unsigned short in_base",
                "--assert", "param 3 %EST unsigned short in_est",
                "--assert", "param 4 %CETBL unsigned short table",
                "--assert", "param 5 %CECFG1 unsigned short ce1",
                "--assert", "param 6 %CECFG2 unsigned short ce2",
            ]));
        let json: Value = serde_json::from_slice(&output.stdout).unwrap();
        let function = &json["functions"][0];
        assert!(function["error"].is_null(), "{function}");
        emitted.push_str(&function["code"].as_str().unwrap().replace("sub_0", name));
        emitted.push('\n');
    }
    emitted.push_str(
        "int main(void) {\n\
         if (word(0x120000,0,0x2468,0x0010,0xffff,0,0) != 0) return 1;\n\
         if (flags(0x120000,0,0x2468,0x0010,0xffff,0,0) != 0x3000) return 2;\n\
         if (flags(0,0,0x2468,0x0010,0xffff,0,0) != 0x3008) return 3;\n\
         if (word(0x120000,0,0x2468,0x0010,0,0,0) != 0x2468) return 4;\n\
         return 0;\n\
         }\n",
    );
    fs::write(&source, &emitted).unwrap();
    run_emitted_c(&source, &executable, &emitted);
    for file in [binary, source, executable] {
        fs::remove_file(file).unwrap();
    }
}

#[test]
fn carmel_predicated_multiply_updates_its_latch_without_vetoed_writeback() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let binary = common::scratch_file("carmel-qualified-mul", "bin");
    let source = binary.with_extension("c");
    let executable = binary.with_extension("exe");
    let regs = ["R0", "X2", "A0L", "A0H", "A1L", "EST", "CECFG1", "CECFG2", "CETBL", "ECFG", "PACFG", "RM0", "RL0"];
    let mut emitted = String::from("#include <stdbool.h>\n#include <stdint.h>\nvolatile unsigned short cell_0100;\n");
    for (issue, results) in [
        ([0x2b, 0xec, 0, 0x41, 0x04, 0x23], [("R0", "pointer"), ("X2", "latch"), ("A0L", "low"), ("EST", "flags")]),
        ([0x51, 0x0c, 0xd6, 0xb2, 0x88, 0x21], [("R0", "wide_pointer"), ("X2", "wide_wrong_latch"), ("A0H", "wide_high"), ("EST", "wide_flags")]),
    ] {
        let mut code = vec![0x80, 0x01, 0, 0xc6, 0x42, 0x80];
        code.extend(issue);
        code.extend([0x2b, 0xd8, 0, 0x2b, 0xec, 0]);
        fs::write(&binary, code).unwrap();
        for (reg, name) in results {
            let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
            kuna.arg("decompile").arg(&binary).args([
                "0", "--addr", "--raw-image", "--target", "Carmel:BE:24:default:default",
                "--base", "0", "--json", "--assert-strict",
                "--assert", "data [data,0x100] volatile unsigned short cell_0100",
                "--assert", &format!("return %{reg} unsigned short"),
            ]);
            for (i, reg) in regs.into_iter().enumerate() {
                kuna.args(["--assert", &format!("param {i} %{reg} unsigned short in_{}", reg.to_ascii_lowercase())]);
            }
            let output = common::process::required_output(&mut kuna);
            let json: Value = serde_json::from_slice(&output.stdout).unwrap();
            let function = &json["functions"][0];
            assert!(function["error"].is_null(), "{function}");
            emitted.push_str(&function["code"].as_str().unwrap().replace("sub_0", name));
            emitted.push('\n');
        }
    }
    emitted.push_str(
        "int main(void) {\n\
         cell_0100 = 101;\n\
         if (pointer(0x100,19,0x4321,5,3,0,0,0,0,0,0,0,0) != 0x100) return 1;\n\
         if (pointer(0x100,19,0x4321,5,3,0,0,0x1000,0,0,0,0,0) != 0x101) return 2;\n\
         if (latch(0x100,19,0x4321,5,3,0,0,0x1000,0,0,0,0,0) != 101) return 3;\n\
         if (low(0x100,19,0x4321,5,3,0,0,0x1000,0,0,0,0,0) != 0x4321) return 4;\n\
         if (low(0x100,19,0x4321,5,3,0,0,0,0xff00,0,0,0,0) != 303) return 5;\n\
         if (flags(0x100,19,0x4321,5,3,0,0,0,0xff00,0,0,0,0) != 0x2000) return 11;\n\
         cell_0100 = 0xfffe;\n\
         if (flags(0x100,19,0x4321,5,3,0,0,0,0xff00,0,0,0,0) != 0x2400) return 12;\n\
         cell_0100 = 0x8000;\n\
         if (flags(0x100,19,0x4321,5,0x8000,0,0,0,0xff00,5,0,0,0) != 0x2900) return 6;\n\
         cell_0100 = 0xffff;\n\
         if (wide_pointer(0x100,19,0x4321,5,0x7fff,0,0,0,0xff00,5,0,0,0) != 0x101) return 7;\n\
         if (wide_wrong_latch(0x100,19,0x4321,5,0x7fff,0,0,0,0xff00,5,0,0,0) != 19) return 8;\n\
         if (wide_high(0x100,19,0x4321,5,0x7fff,0,0,0,0xff00,5,0,0,0) != 0xfffd) return 9;\n\
         if (wide_flags(0x100,19,0x4321,5,0x7fff,0,0,0,0xff00,5,0,0,0) != 0x2020) return 10;\n\
         return cell_0100 != 0xffff;\n\
         }\n",
    );
    fs::write(&source, &emitted).unwrap();
    run_emitted_c(&source, &executable, &emitted);
    for file in [binary, source, executable] {
        fs::remove_file(file).unwrap();
    }
}

#[test]
fn carmel_compact_rmw_result_and_flags_follow_the_physical_lane() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let binary = common::scratch_file("carmel-compact-rmw", "bin");
    let nop = [0x2b, 0xec, 0];
    let add = [0x10, 0xb7, 0x57];
    let mut emitted = String::from("#include <stdbool.h>\n#include <stdint.h>\nvolatile unsigned short cell_c040;\n");
    for (name, reg, packet) in [
        ("primary_half", "A5H", [add, nop]),
        ("secondary_half", "A5H", [nop, add]),
        ("primary_flags", "EST", [add, nop]),
        ("secondary_flags", "EST", [nop, add]),
        ("data_flags", "EST", [[0x10, 0xbf, 0x57], nop]),
        ("data_pointer", "R3", [[0x10, 0xbf, 0x57], nop]),
    ] {
        let code = [
            [[0x80, 0xc0, 0x40], [0xc6, 0x42, 0x8c]], // MOVE R3, 0xc040
            packet,
            [[0x2b, 0xd8, 0], nop],
        ];
        fs::write(&binary, code.as_flattened().as_flattened()).unwrap();
        let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
        kuna.arg("decompile").arg(&binary).args([
            "0", "--addr", "--raw-image", "--target", "Carmel:BE:24:default:default", "--base", "0",
            "--json", "--assert-strict", "--assert", &format!("return %{reg} unsigned short"),
            "--assert", "data [data,0xc040] volatile unsigned short cell_c040",
        ]);
        for (i, input) in ["A5H", "CECFG1", "CECFG2", "CETBL", "EST", "ECFG", "RN3", "PACFG", "RM2", "RL2"].into_iter().enumerate() {
            kuna.args(["--assert", &format!("param {i} %{input} unsigned short in_{}", input.to_ascii_lowercase())]);
        }
        let output = common::process::required_output(&mut kuna);
        let json: Value = serde_json::from_slice(&output.stdout).unwrap();
        let function = &json["functions"][0];
        assert!(function["error"].is_null(), "{function}");
        writeln!(emitted, "{}", function["code"].as_str().unwrap().replace("sub_0", name)).unwrap();
    }
    emitted.push_str(
        "int main(void) {\n\
         cell_c040 = 0x8021;\n\
         if (primary_half(0x3215,0,0,0xffff,0,0,4,0,0,0) != 0x3215) return 1;\n\
         if (secondary_half(0x3215,0,0,0xffff,0,0,4,0,0,0) != 0xb236) return 2;\n\
         if (primary_flags(0x3215,0,0,0xffff,0,0,4,0,0,0) != 0x3010) return 3;\n\
         if (secondary_flags(0x3215,0,0,0xffff,0,0,4,0,0,0) != 0x3400) return 4;\n\
         if (data_pointer(0x3215,0x1000,0,0,0,0,4,0,0,0) != 0xc044 || cell_c040 != 0x8021) return 5;\n\
         if (data_flags(0x3215,0,0,0xffff,0,0,4,0,0,0) != 0x3010 || cell_c040 != 0xb236) return 6;\n\
         cell_c040 = 0x6000;\n\
         if (data_flags(0x7000,0,0,0xffff,0,4,4,0,0,0) != 0x3024 || cell_c040 != 0x7fff) return 7;\n\
         return 0;\n\
         }\n",
    );
    let source = binary.with_extension("c");
    let executable = binary.with_extension("exe");
    fs::write(&source, &emitted).unwrap();
    run_emitted_c(&source, &executable, &emitted);
    for file in [binary, source, executable] {
        fs::remove_file(file).unwrap();
    }
}

#[test]
fn carmel_compact_extract_separates_address_updates_and_result_ports() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let binary = common::scratch_file("carmel-compact-extract", "bin");
    let nop = [0x2b, 0xec, 0];
    let extract = [0x37, 0x2a, 0xde];
    let mut emitted = String::from("#include <stdbool.h>\n#include <stdint.h>\nvolatile unsigned short cell_4200, descriptor_count, descriptor_offset, source_word;\n");
    for (name, reg, packet) in [
        ("write_field", "EST", [extract, nop]),
        ("update_address", "R5", [nop, extract]),
        ("direct_flags", "A1L", [[0x37, 0xc1, 0x8c], nop]),
        ("swapped_flags", "A1L", [[0x37, 0xc1, 0x8e], nop]),
        ("long_descriptor", "X2", [[0x36, 0x5f, 0x52], nop]),
    ] {
        let code = [
            [[0x80, 0x08, 0x06], [0xc6, 0x42, 0x88]], // MOVE R2, 0x0806
            [[0xa2, 0xfa, 0x02], [0xc2, 0x7a, 0xaa]], // MOVE RN2, R2
            [[0x80, 0x81, 0x00], [0xc6, 0x42, 0x88]], // MOVE R2, 0x8100
            [[0x80, 0x41, 0x00], [0xc6, 0x42, 0x84]], // MOVE R1, 0x4100
            [[0x80, 0x42, 0], [0xc6, 0x42, 0x94]], // MOVE R5, 0x4200
            packet,
            [[0x2b, 0xd8, 0], nop],
        ];
        fs::write(&binary, code.as_flattened().as_flattened()).unwrap();
        let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
        kuna.arg("decompile").arg(&binary).args([
            "0", "--addr", "--raw-image", "--target", "Carmel:BE:24:default:default", "--base", "0",
            "--json", "--assert-strict", "--assert", &format!("return %{reg} unsigned short"),
            "--assert", "data [data,0x4200] volatile unsigned short cell_4200",
            "--assert", "data [data,0x4100] volatile unsigned short descriptor_count",
            "--assert", "data [data,0x4101] volatile unsigned short descriptor_offset",
            "--assert", "data [data,0x8906] volatile unsigned short source_word",
        ]);
        for (i, input) in ["EST", "CETBL", "CECFG1", "CECFG2", "A0H", "FF1", "FF2", "RN5", "PACFG", "RM4", "RL4", "RN1", "RM0", "RL0"].into_iter().enumerate() {
            kuna.args(["--assert", &format!("param {i} %{input} unsigned short in_{}", input.to_ascii_lowercase())]);
        }
        let output = common::process::required_output(&mut kuna);
        let json: Value = serde_json::from_slice(&output.stdout).unwrap();
        let function = &json["functions"][0];
        assert!(function["error"].is_null(), "{function}");
        writeln!(emitted, "{}", function["code"].as_str().unwrap().replace("sub_0", name)).unwrap();
    }
    emitted.push_str(
        "int main(void) {\n\
         cell_4200 = 0x8bf1;\n\
         if (update_address(0,0xffff,0,0,0,0x831,0x789,0x806,0,0,0,0,0,0) != 0x4a06 || cell_4200 != 0x8bf1) return 1;\n\
         if (write_field(0,0xffff,0,0,0,0x831,0x789,0x806,0,0,0,0,0,0) != 0x3000 || cell_4200 != 0x1ff) return 2;\n\
         if (direct_flags(0,0xffff,0,0,0x3456,6,8,0,0,0,0,0,0,0) != 0x34) return 3;\n\
         if (swapped_flags(0,0xffff,0,0,0x3456,6,8,0,0,0,0,0,0,0) != 0xd1) return 4;\n\
         descriptor_count = 4; descriptor_offset = 8; source_word = 0x0123;\n\
         if (long_descriptor(0,0xffff,0,0,0,0,0,0,0,0,0,1,0,0) != 1) return 5;\n\
         source_word = 0xf0dd;\n\
         if (long_descriptor(0,0xffff,0,0,0,0,0,0,0,0,0,1,0,0) != 0) return 6;\n\
         return 0;\n\
         }\n",
    );
    let source = binary.with_extension("c");
    let executable = binary.with_extension("exe");
    fs::write(&source, &emitted).unwrap();
    run_emitted_c(&source, &executable, &emitted);
    for file in [binary, source, executable] {
        fs::remove_file(file).unwrap();
    }
}

#[test]
fn carmel_wide_unary_fetch_updates_only_the_connected_status_and_address() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let binary = common::scratch_file("carmel-wide-unary-data", "bin");
    let mut emitted = String::from("#include <stdbool.h>\n#include <stdint.h>\nvolatile unsigned short low_word, high_word;\n");
    for (name, reg, encoding) in [
        ("rotate_left", "EST", [0xc2, 0x3b, 0xcd]),
        ("rotate_right", "EST", [0xc2, 0x3f, 0xcd]),
        ("subtract_unsigned", "EST", [0xc2, 0x03, 0xcd]),
        ("advance", "R4", [0xc2, 0x2f, 0xcd]),
    ] {
        let code = [
            [[0x80, 0, 0x40], [0xc6, 0x42, 0x90]], // MOVE R4, 0x40
            [[0x14, 0xfc, 0x44], encoding],
            [[0x2b, 0xd8, 0], [0x2b, 0xec, 0]],
        ];
        fs::write(&binary, code.as_flattened().as_flattened()).unwrap();
        let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
        kuna.arg("decompile").arg(&binary).args([
            "0", "--addr", "--raw-image", "--target", "Carmel:BE:24:default:default", "--base", "0",
            "--json", "--assert-strict", "--assert", &format!("return %{reg} unsigned short"),
            "--assert", "data [data,0x40] volatile unsigned short low_word",
            "--assert", "data [data,0x41] volatile unsigned short high_word",
        ]);
        for (i, input) in ["CECFG1", "CECFG2", "CETBL", "EST", "PACFG", "RM4", "RL4"].into_iter().enumerate() {
            kuna.args(["--assert", &format!("param {i} %{input} unsigned short in_{}", input.to_ascii_lowercase())]);
        }
        let output = common::process::required_output(&mut kuna);
        let json: Value = serde_json::from_slice(&output.stdout).unwrap();
        let function = &json["functions"][0];
        assert!(function["error"].is_null(), "{function}");
        writeln!(emitted, "{}", function["code"].as_str().unwrap().replace("sub_0", name)).unwrap();
    }
    emitted.push_str(
        "int main(void) {\n\
         low_word = 1; high_word = 0;\n\
         if (rotate_left(0,0,0xffff,0,0,0,0) != 0x3000) return 1;\n\
         if (rotate_right(0,0,0xffff,0,0,0,0) != 0x3001) return 2;\n\
         if (subtract_unsigned(0,0,0xffff,0,0,0,0) != 0x3011) return 3;\n\
         low_word = 0; high_word = 0x8000;\n\
         if (rotate_left(0,0,0xffff,0,0,0,0) != 0x3001) return 4;\n\
         if (rotate_right(0,0,0xffff,0,0,0,0) != 0x3000) return 5;\n\
         if (subtract_unsigned(0,0,0xffff,0,0,0,0) != 0x3031) return 6;\n\
         if (advance(0,0,0,0,0,0,0) != 0x40 || advance(0x1000,0,0,0,0,0,0) != 0x42) return 7;\n\
         return 0;\n\
         }\n",
    );
    let source = binary.with_extension("c");
    let executable = binary.with_extension("exe");
    fs::write(&source, &emitted).unwrap();
    run_emitted_c(&source, &executable, &emitted);
    for file in [binary, source, executable] {
        fs::remove_file(file).unwrap();
    }
}

#[test]
fn carmel_wide_shift_result_lane_selects_carry_and_zero() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let binary = common::scratch_file("carmel-wide-shift-ports", "bin");
    let mut emitted = String::from(
        "#include <stdbool.h>\n#include <stdint.h>\n\
         volatile unsigned short out_low, out_high, count_word, in_low, in_high;\n\
         #define CONCAT22(h,l) (((uint32_t)(uint16_t)(h) << 16) | (uint16_t)(l))\n",
    );
    for (name, right) in [
        ("word_from_word", [0xf0, 0x00, 0x04]),
        ("word_from_long", [0xf0, 0x00, 0x84]),
        ("long_from_long", [0xf0, 0x80, 0x84]),
        ("acc_from_long", [0xf1, 0x10, 0x84]),
    ] {
        let code = [
            [[0x80, 0, 0x40], [0xc6, 0x42, 0x80]], // MOVE R0, 0x40
            [[0x80, 0x40, 0x40], [0xc6, 0x42, 0x84]], // MOVE R1, 0x4040
            [[0x80, 0x80, 0x40], [0xc6, 0x42, 0x88]], // MOVE R2, 0x8040
            [[0xc1, 0, 0], right],
            [[0x2b, 0xd8, 0], [0x2b, 0xec, 0]],
        ];
        fs::write(&binary, code.as_flattened().as_flattened()).unwrap();
        let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
        kuna.arg("decompile").arg(&binary).args([
            "0", "--addr", "--raw-image", "--target", "Carmel:BE:24:default:default",
            "--base", "0", "--json", "--assert-strict",
            "--assert", "return %EST unsigned short",
            "--assert", "data [data,0x40] volatile unsigned short out_low",
            "--assert", "data [data,0x41] volatile unsigned short out_high",
            "--assert", "data [data,0x4040] volatile unsigned short count_word",
            "--assert", "data [data,0x8040] volatile unsigned short in_low",
            "--assert", "data [data,0x8041] volatile unsigned short in_high",
        ]);
        for (i, input) in ["EST", "CECFG1", "CECFG2", "CETBL"].into_iter().enumerate() {
            kuna.args(["--assert", &format!("param {i} %{input} unsigned short in_{}", input.to_ascii_lowercase())]);
        }
        let output = common::process::required_output(&mut kuna);
        let json: Value = serde_json::from_slice(&output.stdout).unwrap();
        let function = &json["functions"][0];
        assert!(function["error"].is_null(), "{function}");
        writeln!(emitted, "{}", function["code"].as_str().unwrap().replace("sub_0", name)).unwrap();
    }
    emitted.push_str(
        "int main(void) {\n\
         count_word = 4; in_low = 1u << 11;\n\
         if (word_from_word(0,0x1000,0x1000,0) != 0x20 || out_low != 0x8000) return 1;\n\
         in_low = 1u << 12;\n\
         if (word_from_word(0,0x1000,0x1000,0) != 0x21 || out_low != 0) return 2;\n\
         count_word = (unsigned short)-7; in_low = 1u << 6;\n\
         if (word_from_word(0,0x1000,0x1000,0) != 9 || out_low != 0) return 3;\n\
         in_low = 1u << 7;\n\
         if (word_from_word(0,0x1000,0x1000,0) != 0 || out_low != 1) return 4;\n\
         count_word = (unsigned short)-16; in_low = 0x8000; in_high = 0;\n\
         if (word_from_long(0,0x1000,0x1000,0) != 8 || out_low != 0) return 5;\n\
         out_low = 0xffff; out_high = 0xffff;\n\
         if (long_from_long(0,0x1000,0x1000,0) != 9 || out_low != 0 || out_high != 0) return 6;\n\
         if (acc_from_long(0,0x1000,0x1000,0) != 9) return 7;\n\
         in_low = 0; in_high = 0x8000;\n\
         if (word_from_long(0,0x1000,0x1000,0) != 9 || out_low != 0) return 8;\n\
         if (long_from_long(0,0x1000,0x1000,0) != 0 || out_low != 0x8000 || out_high != 0) return 9;\n\
         count_word = 36; in_low = 1u << 4; in_high = 0;\n\
         if (word_from_long(0,0x1000,0x1000,0) != 8 || out_low != 0) return 10;\n\
         if (acc_from_long(0,0x1000,0x1000,0) != 9) return 11;\n\
         return 0;\n\
         }\n",
    );
    let source = binary.with_extension("c");
    let executable = binary.with_extension("exe");
    fs::write(&source, &emitted).unwrap();
    run_emitted_c(&source, &executable, &emitted);
    for file in [binary, source, executable] {
        fs::remove_file(file).unwrap();
    }
}

#[test]
fn carmel_compact_logic_rmw_routes_execute_in_emitted_c() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let binary = common::scratch_file("carmel-compact-logic-ports", "bin");
    let source = binary.with_extension("c");
    let executable = binary.with_extension("exe");
    let mut emitted = String::from(
        "#include <stdbool.h>\n#include <stdint.h>\n\
         volatile unsigned short cell_4048, cell_404a, cell_8040, cell_c044;\n",
    );
    let mut calls = String::from("int main(void) {\n");
    let vectors = [
        ([0x17, 0x5b, 0xca, 0x6b, 0x6c, 0x26], "R2", [0, 0]),
        ([0x2b, 0xec, 0, 0x6b, 0x6c, 0x27], "R2", [0x8040, 0]),
        ([0x6b, 0x6c, 0xa7, 0x2b, 0xec, 0], "RN2", [3, 0]),
        ([0x2b, 0xec, 0, 0x68, 0xb5, 0x85], "FF1", [0x1234, 0]),
        ([0x2b, 0xec, 0, 0x63, 0x6d, 0x27], "A2L", [0x4b74, 0x0234]),
        ([0x6f, 0x6d, 0x27, 0x2b, 0xec, 0], "A2L", [0x4b74, 0x5940]),
        ([0xe8, 0xb4, 0x85, 0x2b, 0xec, 0], "A2L", [0x4b74, 0x7f7f]),
        ([0x2b, 0xec, 0, 0x6b, 0x6e, 0x27], "R2", [0x8040, 0x8040]),
    ];
    for (route, (packet, output, expected)) in vectors.into_iter().enumerate() {
        for (enabled, (truth, result)) in [0, 0xff].into_iter().zip(expected).enumerate() {
            // Keep the address calculations in the lifted issue, while making DATA
            // addresses resolvable to separate word objects by the decompiler.
            let code = [
                [0x80, 0, 4, 0xc6, 0x42, 0xac], // MOVE RN3,4
                [0x80, 0, 2, 0xc6, 0x42, 0xb4], // MOVE RN5,2
                [0x80, 0x80, 0x40, 0xc6, 0x42, 0x88], // MOVE R2,0x8040
                [0x80, 0xc0, 0x40, 0xc6, 0x42, 0x8c], // MOVE R3,0xc040
                [0x80, 0x40, 0x48, 0xc6, 0x42, 0x94], // MOVE R5,0x4048
                if route == 0 {
                    [0x80, 0, truth, 0xcd, 0xe4, 0xc0]
                } else {
                    // LOADCE.NOT enables exactly one physical lane, so using
                    // the other lane's predicate cannot pass the regression.
                    let truth = if matches!(route, 1 | 3 | 4 | 7) { !truth } else { truth };
                    [0x80, 0, truth, 0xcd, 0xe6, 0xc0]
                },
                packet,
                [0x2b, 0xd8, 0, 0x2b, 0xec, 0],
            ];
            fs::write(&binary, code.as_flattened()).unwrap();
            let projections: &[(&str, u16)] = if route == 0 {
                &[(output, result), ("R5", 0x404a), ("EST", 1 + (enabled as u16) * 0x3000)]
            } else {
                &[(output, result)]
            };
            for &(register, expected) in projections {
                let name = format!("route_{route}_{enabled}_{}", register.to_ascii_lowercase());
                let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
                kuna.arg("decompile").arg(&binary).args([
                    "0", "--addr", "--raw-image", "--target", "Carmel:BE:24:default:default", "--base", "0",
                    "--json", "--assert-strict", "--assert", &format!("return %{register} unsigned short"),
                ]);
                for address in [0x4048, 0x404a, 0x8040, 0xc044] {
                    kuna.args(["--assert", &format!("data [data,{address:#x}] volatile unsigned short cell_{address:x}")]);
                }
                for (i, reg) in ["A2L", "A4H", "FF1", "RN2", "EST", "ECFG", "PACFG", "RM4", "RL4"].into_iter().enumerate() {
                    kuna.args(["--assert", &format!("param {i} %{reg} unsigned short in_{}", reg.to_ascii_lowercase())]);
                }
                let output = common::process::required_output(&mut kuna);
                let json: Value = serde_json::from_slice(&output.stdout).unwrap();
                let function = &json["functions"][0];
                assert!(function["error"].is_null(), "{function}");
                writeln!(emitted, "{}", function["code"].as_str().unwrap().replace("sub_0", &name)).unwrap();
                let left_data = if route == 0 { 0x529f } else { 0x71bb };
                let right_data = if route == 7 && enabled == 1 { 0x9274 } else { 0x1234 };
                writeln!(calls,
                    "cell_4048=0x71bb; cell_404a=0x5dda; cell_8040=0x7c4b; cell_c044=0x1234;\n\
                     if ({name}(0x4b74,0xf4c5,0x1234,3,0,0,0,0,0) != {expected:#x} || \
                     cell_4048 != {left_data:#x} || cell_c044 != {right_data:#x} || \
                     cell_404a != 0x5dda || cell_8040 != 0x7c4b) return {};",
                    1 + route * 2 + enabled,
                ).unwrap();
            }
        }
    }
    emitted.push_str(&calls);
    emitted.push_str("return 0;\n}\n");
    fs::write(&source, &emitted).unwrap();
    run_emitted_c(&source, &executable, &emitted);
    for file in [binary, source, executable] {
        fs::remove_file(file).unwrap();
    }
}

#[test]
fn carmel_packed_short_results_and_extrema_sinks_preserve_independent_lanes() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let binary = common::scratch_file("carmel-packed-extrema", "bin");
    let mut emitted = String::from(
        "#include <stdbool.h>\n#include <stdint.h>\nvolatile unsigned short low_word, high_word;\n",
    );
    for (name, reg, packet, data) in [
        ("add_half", "A2H", [0x59, 0x2d, 0xa8, 0xc4, 0x46, 0xb5], false),
        ("trace_high", "BTR1", [0xd9, 0x85, 0x8d, 0xc5, 0xc6, 0x65], true),
        ("trace_low", "BTR2", [0xf2, 0x85, 0x8d, 0xc5, 0xce, 0xb4], true),
        ("postmodify", "R2", [0xf2, 0x85, 0x8d, 0xc5, 0xce, 0xb4], true),
    ] {
        let mut image = Vec::new();
        if data {
            image.extend_from_slice(&[0x80, 0x80, 0x40, 0xc6, 0x42, 0x88]); // MOVE R2, 0x8040
        }
        image.extend_from_slice(&packet);
        image.extend_from_slice(&[0x2b, 0xd8, 0, 0x2b, 0xec, 0]);
        fs::write(&binary, image).unwrap();
        let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
        kuna.arg("decompile").arg(&binary).args([
            "0", "--addr", "--raw-image", "--target", "Carmel:BE:24:default:default", "--base",
            "0", "--json", "--assert-strict", "--assert", &format!("return %{reg} unsigned short"),
        ]);
        for (i, input) in [
            "A2H", "A1H", "BTR1", "BTR2", "CETBL", "CECFG1", "CECFG2", "EST", "PACFG", "RM2", "RL2",
        ].into_iter().enumerate() {
            kuna.args(["--assert", &format!("param {i} %{input} unsigned short in_{}", input.to_ascii_lowercase())]);
        }
        if data {
            kuna.args([
                "--assert", "data [data,0x8040] volatile unsigned short low_word",
                "--assert", "data [data,0x8041] volatile unsigned short high_word",
            ]);
        }
        let output = common::process::required_output(&mut kuna);
        let json: Value = serde_json::from_slice(&output.stdout).unwrap();
        let function = &json["functions"][0];
        assert!(function["error"].is_null(), "{function}");
        let code = function["code"].as_str().unwrap();
        assert!(!code.contains("halt_"), "{code}");
        writeln!(emitted, "{}", code.replace("sub_0", name)).unwrap();
    }
    emitted.push_str(
        "#define INPUTS(table) 0x29cd,0x2112,0x5252,0x3434,table,0x1000,0x1000,0,0,0,0\n\
         int main(void) {\n\
         low_word = 0x7c4b; high_word = 0x20fa;\n\
         if (add_half(INPUTS(0)) != 0x29cd || add_half(INPUTS(0xffff)) != 0x4adf) return 1;\n\
         if (trace_high(INPUTS(0xffff)) != 0xa4a4 || trace_low(INPUTS(0)) != 0x6869) return 2;\n\
         if (postmodify(INPUTS(0)) != 0x7fcd || postmodify(INPUTS(0xffff)) != 0x7fcd) return 3;\n\
         if (low_word != 0x7c4b || high_word != 0x20fa) return 4;\n\
         return 0;\n\
         }\n",
    );
    let source = binary.with_extension("c");
    let executable = binary.with_extension("exe");
    fs::write(&source, &emitted).unwrap();
    run_emitted_c(&source, &executable, &emitted);
    for file in [binary, source, executable] {
        fs::remove_file(file).unwrap();
    }
}

#[test]
fn carmel_wide_paired_stack_separates_stride_and_operand_ports() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let binary = common::scratch_file("carmel-wide-paired-stack", "bin");
    let source = binary.with_extension("c");
    let executable = binary.with_extension("exe");
    let mut emitted = String::from(
        "#include <stdbool.h>\n\
         volatile unsigned short below, old_sp, top, next_word;\n",
    );
    let mut main = String::from("int main(void) {\n");
    let rows = [
        ([0x02, 0x56, 0xac, 0xca, 0x13, 0x49], 2, 0x6385, 0x9f26, [0xc460, 0x338c]),
        ([0x02, 0x16, 0xac, 0xca, 0x13, 0x49], 1, 0x6385, 0x9f26, [0xc460, 0x338c]),
        ([0x02, 0x56, 0x93, 0xca, 0x13, 0x49], 2, 1, 0x6385, [0xc460, 0x338c]),
        ([0x4f, 0xa3, 0x93, 0xca, 0x07, 0xb1], 0, 1, 0x9f26, [0xffff, 0x9f26]),
        ([0x4f, 0x92, 0xa3, 0xca, 0x07, 0xb1], 0, 1, 0x9f26, [0x4b74, 0x4b74]),
        ([0x4f, 0x56, 0x93, 0xca, 0x07, 0xb1], 2, 1, 0x9f26, [0x9f26, 0xffff]),
        ([0x4f, 0x16, 0x93, 0xca, 0x07, 0xb1], 1, 1, 0x9f26, [0x9f26, 0xffff]),
        ([0xf7, 0x2d, 0x99, 0xca, 0x1e, 0x64], 0, 1, 0x9f26, [0x834c, 0xffff]),
    ];
    let mut error = 0;
    for (row, (packet, stride, rn4, enabled_a3l, stored)) in rows.into_iter().enumerate() {
        let code = [
            [0x80, 0x40, 0x40, 0xc6, 0x42, 0x80], // MOVE R0,0x4040
            [0x80, 0, 0, 0xc0, 0x40, 0xec],       // TRANSFER SP,R0
            [0x80, 0x80, 0x48, 0xc6, 0x42, 0x98], // MOVE R6,0x8048
            packet,
            [0x80, 0, 0, 0xc0, 0x76, 0x9c], // TRANSFER R7,SP
            [0x80, 0x41, 0, 0xc6, 0x42, 0x80], // MOVE R0,0x4100
            [0x80, 0, 0, 0xc0, 0x40, 0xec], // TRANSFER SP,R0
            [0x2b, 0xd8, 0, 0x2b, 0xec, 0], // RETURN || NOP
        ];
        fs::write(&binary, code.as_flattened()).unwrap();
        let sp = if row < 3 { 0x4042 } else { 0x403e };
        for (reg, disabled, enabled) in [
            ("R7", sp, sp),
            ("R6", 0x8048 + stride, 0x8048 + stride),
            ("RN4", rn4, rn4),
            ("A3L", 0x9f26, enabled_a3l),
            ("EST", 0, 0x3000),
        ] {
            let name = format!("pair_{row}_{}", reg.to_ascii_lowercase());
            let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
            kuna.arg("decompile").arg(&binary).args([
                "0x1000", "--addr", "--raw-image", "--target", "Carmel:BE:24:default:default",
                "--base", "0x1000", "--json", "--assert-strict",
                "--assert", &format!("return %{reg} unsigned short"),
                "--assert", "data [data,0x403f] volatile unsigned short below",
                "--assert", "data [data,0x4040] volatile unsigned short old_sp",
                "--assert", "data [data,0x4041] volatile unsigned short top",
                "--assert", "data [data,0x4042] volatile unsigned short next_word",
            ]);
            for (i, input) in [
                "A1H", "A2L", "A3L", "RN4", "CECFG1", "CECFG2", "CETBL",
                "EST", "ECFG", "PACFG", "RN6", "RM6", "RL6",
            ].into_iter().enumerate() {
                kuna.args(["--assert", &format!("param {i} %{input} unsigned short in_{}", input.to_ascii_lowercase())]);
            }
            let output = common::process::required_output(&mut kuna);
            let json: Value = serde_json::from_slice(&output.stdout).unwrap();
            let function = &json["functions"][0];
            assert!(function["error"].is_null(), "{function}");
            let c = function["code"].as_str().unwrap();
            assert!(!c.contains("halt_"), "{function}");
            writeln!(emitted, "{}", c.replace("sub_1000", &name)).unwrap();
            for (table, expected) in [(0, disabled), (0xffff, enabled)] {
                let [old_sp, below] = if table != 0 || row == 7 { stored } else { [0xc460, 0x338c] };
                error += 1;
                writeln!(main,
                    "below = 0x338c; old_sp = 0xc460; top = 0x6385; next_word = 0x59f6;\n\
                     if ({name}(0x834c,0x4b74,0x9f26,1,0x1000,0x1000,{table},0,0,0,3,0,0) != {expected}\n\
                         || old_sp != {old_sp} || below != {below} || top != 0x6385 || next_word != 0x59f6) return {error};",
                ).unwrap();
            }
        }
    }
    emitted.push_str(&main);
    emitted.push_str("return 0;\n}\n");
    fs::write(&source, &emitted).unwrap();
    run_emitted_c(&source, &executable, &emitted);
    for file in [binary, source, executable] {
        fs::remove_file(file).unwrap();
    }
}

fn run_emitted_c(source: &Path, executable: &Path, emitted: &str) {
    for opt in ["-O0", "-O2"] {
        common::process::required_output(Command::new("cc")
            .args([opt, "-std=c11", "-Werror=implicit-function-declaration", "-Werror=uninitialized",
                   "-fsanitize=undefined", "-fno-sanitize-recover=undefined"])
            .arg(source).arg("-o").arg(executable));
        let result = common::process::output_with_timeout(
            &mut Command::new(executable), Duration::from_secs(10), Duration::from_millis(10),
        ).unwrap();
        assert!(result.status.success(), "{opt}: {:?}\n{}\n{emitted}", result.status,
                String::from_utf8_lossy(&result.stderr));
    }
}
