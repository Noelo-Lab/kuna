//! Compile and execute emitted C for Carmel raw-image programs.
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
                let data_access = matches!(opcode, "LOAD" | "STORE");
                let transfer = matches!(opcode, "BRANCH" | "CBRANCH" | "CALL")
                    && op["inputs"][0]["space"] == "code";
                (data_access || transfer).then_some(opcode)
            }).collect();
            assert_eq!(ops, expected, "{doc}");
        }
    }
    fs::remove_file(binary).unwrap();
}

#[test]
fn carmel_external_control_samples_the_board_and_keeps_call_link_stores() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let binary = common::scratch_file("carmel-external-control", "bin");
    let source = binary.with_extension("c");
    let executable = binary.with_extension("exe");
    let nop = [0x2b, 0xec, 0];
    for (slot, positive, active_level) in [
        ([[0x28, 0x28, 0x20], nop], true, 1),
        ([[0xc0, 0, 8], [0xce, 0x40, 0x6c]], false, 8),
    ] {
        let mut image = [[nop; 2]; 5];
        image[0] = slot;
        image[1][0] = [0x21, 0xac, 0x20];
        image[4][0] = [0x2b, 0xd8, 0];
        fs::write(&binary, image.as_flattened().as_flattened()).unwrap();
        let output = common::process::required_output(Command::new(env!("CARGO_BIN_EXE_kuna"))
            .arg("decompile").arg(&binary).args([
                "0", "--addr", "--raw-image", "--target", "Carmel:BE:24:default:default", "--base", "0",
                "--json", "--assert-strict", "--assert", "param 0 %A0L unsigned short a",
                "--assert", "param 1 %ECFG unsigned short config",
                "--assert", "return %A0L unsigned short",
            ]));
        let json: Value = serde_json::from_slice(&output.stdout).unwrap();
        let function = &json["functions"][0];
        assert!(function["error"].is_null(), "{function}");
        let emitted = function["code"].as_str().unwrap();
        fs::write(&source, format!(
            "#include <stdbool.h>\n#include <stdint.h>\n\
             static uint8_t levels;\nstatic unsigned samples;\n\
             static uint8_t carmel_sample_gpp(void) {{ ++samples; return levels; }}\n{emitted}\n\
             int main(void) {{\n\
               if (sub_0(41, 0) != {} || samples != 1) return 1;\n\
               levels = {active_level};\n\
               if (sub_0(41, 0) != {} || samples != 2) return 2;\n\
               return 0;\n\
             }}\n",
            if positive { 42 } else { 41 }, if positive { 41 } else { 42 },
        )).unwrap();
        run_emitted_c(&source, &executable, emitted);
    }
    for (slot, call_ops) in [
        ([[0x28, 0xaa, 0x20], nop], &["CALLOTHER", "STORE", "STORE", "CBRANCH", "CALL"][..]),
        ([[0xc0, 0, 8], [0xce, 0x42, 0xec]], &["STORE", "STORE"][..]),
    ] {
        let mut image = [[nop; 2]; 5];
        image[0] = slot;
        fs::write(&binary, image.as_flattened().as_flattened()).unwrap();
        let (stdout, stderr, status) = common::run_kuna(&[
            "raw-pcode", binary.to_str().unwrap(), "0x1000", "--raw-image", "--target",
            "Carmel:BE:24:default:default", "--base", "0x1000", "--count", "1", "--json",
        ]);
        assert_eq!(status, 0, "{stdout}\n{stderr}");
        let doc: Value = serde_json::from_str(&stdout).unwrap();
        let ops = doc["instructions"][0]["ops"].as_array().unwrap();
        assert_eq!(ops.iter().filter_map(|op| {
            let name = op["opcode"].as_str().unwrap();
            matches!(name, "CALLOTHER" | "STORE" | "CBRANCH" | "CALL").then_some(name)
        }).collect::<Vec<_>>(), call_ops, "{doc}");
        if let Some(call) = ops.iter().find(|op| op["opcode"] == "CALL") {
            assert_eq!(call["inputs"][0]["space"], "code", "{doc}");
            assert_eq!(call["inputs"][0]["offset"], 3 * 0x1008, "{doc}");
        }
    }
    for file in [binary, source, executable] {
        fs::remove_file(file).unwrap();
    }
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
    let mixed = [
        0xb8, 0x41, 0x00, 0xcf, 0x18, 0x00,
        0xa1, 0xac, 0x20, 0x21, 0xac, 0x62,
        0x21, 0xac, 0xa4, 0x2b, 0xec, 0x00,
        0xa1, 0xac, 0xe6, 0x21, 0xad, 0x28,
        0x2b, 0xec, 0x00, 0x2b, 0xec, 0x00,
        0x2b, 0xd8, 0x00, 0x2b, 0xec, 0x00,
    ];
    check_carmel_data_repeat_range(&code);
    check_carmel_data_repeat_range(&mixed);
}

fn check_carmel_data_repeat_range(code: &[u8]) {
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
fn carmel_wide_single_repeat_samples_implicit_data_after_no_address_update() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let binary = common::scratch_file("carmel-repeat-data-ffff", "bin");
    let source = binary.with_extension("c");
    let executable = binary.with_extension("exe");
    let nop = [0x2b, 0xec, 0, 0x2b, 0xec, 0];
    let setup = [0x21, 0xac, 0x62, 0x2b, 0xec, 0]; // INC A1L
    let body = [0x21, 0xac, 0x20, 0x2b, 0xec, 0]; // INC A0L
    let ret = [0x2b, 0xd8, 0, 0x2b, 0xec, 0];
    let base = "0x2200";
    for repeat in [[0x95, 0, 0, 0xcf, 0x18, 0xae], [0xa6, 0x0b, 0xc3, 0xcf, 0x18, 0xaf]] {
        let code = [repeat, setup, setup, setup, body, nop, ret];
        fs::write(&binary, code.as_flattened()).unwrap();
        let raw = [
            "raw-pcode", binary.to_str().unwrap(), base, "--raw-image", "--target",
            "Carmel:BE:24:default:default", "--base", base, "--count", "1", "--json",
        ];
        let (stdout, stderr, status) = common::run_kuna(&raw);
        assert_eq!(status, 0, "{stdout}\n{stderr}");
        let doc: Value = serde_json::from_str(&stdout).unwrap();
        let bus_ops: Vec<_> = doc["instructions"][0]["ops"].as_array().unwrap().iter()
            .filter_map(|op| match op["opcode"].as_str().unwrap() {
                name @ ("LOAD" | "STORE") => Some(name),
                _ => None,
            }).collect();
        assert_eq!(bus_ops, ["LOAD"], "{doc}");
        for (output, expression) in [("A0L", "9 + count"), ("A1L", "14")] {
            let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
            kuna.arg("decompile").arg(&binary).args([
                base, "--addr", "--raw-image", "--target", "Carmel:BE:24:default:default",
                "--base", base, "--json", "--assert-strict",
                "--assert", "data [data,0x0] volatile unsigned short wrong_count",
                "--assert", "data [data,0xffff] volatile unsigned short repeat_count",
                "--assert", "param 0 %A0L unsigned short in_a0l",
                "--assert", "param 1 %A1L unsigned short in_a1l",
                "--assert", "param 2 %ECFG unsigned short in_ecfg",
                "--assert", "param 3 %EST unsigned short in_est",
                "--assert", "param 4 %CECFG1 unsigned short in_cecfg1",
                "--assert", "param 5 %CECFG2 unsigned short in_cecfg2",
                "--assert", "param 6 %CETBL unsigned short in_cetbl",
                "--assert", &format!("return %{output} unsigned short"),
            ]);
            let result = common::process::required_output(&mut kuna);
            let json: Value = serde_json::from_slice(&result.stdout).unwrap();
            let function = &json["functions"][0];
            assert!(function["error"].is_null(), "{function}");
            let emitted = function["code"].as_str().unwrap();
            assert!(emitted.contains("repeat_count") && !emitted.contains("wrong_count"), "{emitted}");
            fs::write(&source, format!(
                "#include <stdbool.h>\n#include <stdint.h>\n\
                 volatile unsigned short repeat_count, wrong_count;\n{emitted}\n\
                 int main(void) {{\n\
                   wrong_count = 7;\n\
                   for (unsigned count = 0; count <= 3; ++count) {{\n\
                     repeat_count = count;\n\
                     if (sub_2200(9, 11, 0, 0, 0, 0, 0) != {expression} ||\n\
                         wrong_count != 7 || repeat_count != count) return 1;\n\
                   }}\n\
                   return 0;\n\
                 }}\n"
            )).unwrap();
            run_emitted_c(&source, &executable, emitted);
        }
        let mut split = code;
        split[1][0] |= 0x80;
        fs::write(&binary, split.as_flattened()).unwrap();
        let (stdout, stderr, status) = common::run_kuna(&raw);
        assert_eq!(status, 1, "{stdout}\n{stderr}");
        let doc: Value = serde_json::from_str(&stdout).unwrap();
        assert!(doc["instructions"].as_array().unwrap().is_empty(), "{doc}");
    }
    for file in [binary, source, executable] {
        fs::remove_file(file).unwrap();
    }
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
fn carmel_compact_bank_exchange_executes_in_emitted_c() {
    const TARGET: &str = "Carmel:BE:24:default:default";
    let binary = common::scratch_file("carmel-compact-bank-exchange", "bin");
    let source = binary.with_extension("c");
    let executable = binary.with_extension("exe");
    let nop = [0x2b, 0xec, 0, 0x2b, 0xec, 0];
    let ret = [0x2b, 0xd8, 0, 0x2b, 0xec, 0];
    let qualified = [0x2b, 0xec, 0, 0x2b, 0x80, 3];
    fs::write(&binary, [qualified, ret].as_flattened()).unwrap();
    let (stdout, stderr, status) = common::run_kuna(&[
        "disassemble", binary.to_str().unwrap(), "0", "--raw-image", "--target", TARGET,
        "--base", "0", "--count", "1", "--json",
    ]);
    assert_eq!(status, 0, "{stdout}\n{stderr}");
    let listing: Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(listing["instructions"][0]["text"], "nop || ifexe bankxchg(a3,est);");
    let (_, _, status) = common::run_kuna(&[
        "raw-pcode", binary.to_str().unwrap(), "0", "--raw-image", "--target", TARGET,
        "--base", "0", "--count", "1", "--json",
    ]);
    assert_eq!(status, 1, "qualification must not silently lift");
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        fs::remove_file(binary).unwrap();
        return;
    }

    type Projection = (&'static str, &'static [(&'static str, u16)], u16);
    let right: &[Projection] = &[
        ("A3L", &[("A3LS", 0x4455)], 0x4455),
        ("A3H", &[("A3HS", 0x2233)], 0x2233),
        ("GB3", &[("GB3S", 0x11)], 0x11),
        ("A3LS", &[("A3L", 0)], 0),
        ("A3HS", &[("A3H", 0)], 0),
        ("GB3S", &[("GB3", 0)], 0),
        ("EST", &[("ESTS", 0x234), ("CECFG1", 0x1000), ("CECFG2", 0x1000), ("CETBL", 0)], 0x234),
        ("ESTS", &[("EST", 0)], 0),
    ];
    let left: &[Projection] = &[
        ("PACFG", &[("PACFG", 0x0020), ("PACFGS", 0x5555)], 0x0455),
        ("PACFGS", &[("PACFG", 0x0020), ("PACFGS", 0x5555)], 0x5120),
        ("R0", &[("R0S", 0x5555)], 0x5555),
        ("RN0", &[("RN0S", 0x1234)], 0x1234),
        ("R5", &[("R5S", 0x5558)], 0x5558),
        ("A2L", &[("A2LS", 0x3f29)], 0x3f29),
        ("A2H", &[("A2HS", 0xfb87)], 0xfb87),
        ("GB2", &[("GB2S", 0xf9)], 0xf9),
        ("EST", &[("ESTS", 0x15aa), ("CECFG1", 0x1000), ("CECFG2", 0x1000), ("CETBL", 0xffff)], 0x35aa),
        ("ESTS", &[("EST", 0)], 0),
    ];
    let kind = |reg: &str| if reg.starts_with("GB") { "unsigned char" } else { "unsigned short" };
    let mut emitted = String::from("#include <stdbool.h>\n#include <stdint.h>\n");
    let mut main = String::from("int main(void) {\n");
    for (slot, (base, fetch, rows)) in [
        ("0", [0x2b, 0xec, 0, 0x2b, 0x80, 2], right),
        ("0x1000", [0x2b, 0xb0, 0xbe, 0x2b, 0xec, 0], left),
    ].into_iter().enumerate() {
        fs::write(&binary, [fetch, nop, ret].as_flattened()).unwrap();
        for (i, &(reg, inputs, expected)) in rows.iter().enumerate() {
            let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
            kuna.arg("decompile").arg(&binary).args([
                base, "--addr", "--raw-image", "--target", TARGET, "--base", base, "--json",
                "--assert-strict", "--assert", &format!("return %{reg} {}", kind(reg)),
            ]);
            for (arg, &(input, _)) in inputs.iter().enumerate() {
                kuna.args(["--assert", &format!("param {arg} %{input} {} in_{}", kind(input), input.to_ascii_lowercase())]);
            }
            let output = common::process::required_output(&mut kuna);
            let document: Value = serde_json::from_slice(&output.stdout).unwrap();
            let function = &document["functions"][0];
            assert!(function["error"].is_null() && function["truncated"] != true, "{function}");
            let body = function["code"].as_str().unwrap();
            let name = format!("bank_{slot}_{i}");
            writeln!(emitted, "{}", body.replace(function["name"].as_str().unwrap(), &name)).unwrap();
            let values = inputs.iter().map(|&(_, value)| format!("{value:#x}")).collect::<Vec<_>>().join(", ");
            writeln!(main, "if ({name}({values}) != {expected:#x}) return {};", 16 * slot + i + 1).unwrap();
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

#[test]
fn carmel_wide_discarded_literal_shift_still_reads_its_count() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let scratch = tempfile::tempdir().unwrap();
    let binary = scratch.path().join("discard-shift.bin");
    let source = scratch.path().join("discard-shift.c");
    let executable = scratch.path().join("discard-shift");
    let specs = scratch.path().join("specs");
    tracked_carmel_entry_specs(&specs, &[("R0", 0x40), ("R1", 0x4040), ("PACFG", 0)], 0, 0);
    let explicit = scratch.path().join("specs-explicit");
    tracked_carmel_entry_specs(
        &explicit,
        &[("R1", 0x4005), ("RM0", 0x1007), ("RL0", 0), ("PACFG", 12), ("A0L", 1), ("ECFG", 0)],
        0,
        0,
    );
    let implicit = scratch.path().join("specs-implicit");
    tracked_carmel_entry_specs(
        &implicit,
        &[("R1", 0x4044), ("RM0", 7), ("RL0", 0), ("RM0_ALIGN", 0), ("PACFG", 8), ("A0L", 0x1007), ("ECFG", 0)],
        0,
        0,
    );
    let nop = [0x2b, 0xec, 0, 0x2b, 0xec, 0];
    let mut c = String::from(
        "#include <stdbool.h>\n#include <stdint.h>\nstatic volatile unsigned short value;\nstatic unsigned reads;\n\
         static volatile unsigned short *count_port(void) { ++reads; return &value; }\n\
         #define count_word (*count_port())\n",
    );
    let mut calls = String::new();
    let indexed = [0x51, 0xd5, 0xb7, 0xf1, 0x1f, 0xc0];
    for (row, (packet, address, profile, advanced, writer)) in [
        ([0x60, 0xd5, 0xb7, 0xf1, 0x1f, 0xdc], 0xfff7, &specs, None, None),
        (indexed, 0x4040, &specs, None, None),
        ([0x51, 0xd5, 0xb7, 0xf1, 0x1f, 0xee], 0x4040, &specs, None, None),
        (indexed, 0x4005, &explicit, Some(0x2ffe), None),
        (indexed, 0x4044, &implicit, Some(0x403d), None),
        (indexed, 0x4044, &implicit, Some(0x4045), Some([0x30, 0x84, 0x0e, 0x2b, 0xec, 0])),
        (indexed, 0x4044, &implicit, Some(0x4045), Some([0x90, 0, 0, 0xc2, 0x62, 0xca])),
    ].into_iter().enumerate() {
        let (reg, first) = if advanced.is_some() {
            ("R1", [0x21, 0xac, 0x20, 0x2b, 0xec, 0])
        } else {
            ("EST", nop)
        };
        let mut code = Vec::new();
        if let Some(writer) = writer {
            code.push(writer);
            code.extend([nop; 6]);
        }
        code.extend([first, nop, nop, nop, nop, packet, [0x2b, 0xd8, 0, 0x2b, 0xec, 0], nop]);
        fs::write(&binary, code.as_flattened()).unwrap();
        let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
        kuna.env("KUNA_SPECS", profile).arg("decompile").arg(&binary).args([
            "0", "--addr", "--raw-image", "--target", "Carmel:BE:24:default:default", "--base", "0",
            "--json", "--assert-strict", "--assert", &format!("return %{reg} unsigned short"),
            "--assert", &format!("data [data,{address:#x}] volatile unsigned short count_word"),
        ]);
        for (n, reg) in ["EST", "CECFG1", "CECFG2", "CETBL"].into_iter().enumerate() {
            kuna.args(["--assert", &format!("param {n} %{reg} unsigned short in_{n}")]);
        }
        let output = common::process::required_output(&mut kuna);
        let json: Value = serde_json::from_slice(&output.stdout).unwrap();
        let function = &json["functions"][0];
        assert!(function["error"].is_null(), "{function}");
        writeln!(c, "#define sub_0 discard_{row}\n{}\n#undef sub_0", function["code"].as_str().unwrap()).unwrap();
        for (arm, (table, cfg, reads, count, incoming, carry)) in [
            (0, 0, 0, 0xfff8, 0x269, 1),
            (0xff, 0, 0, 0x11, 0x269, 1),
            (0xff00, 0, 1, 0xfff8, 0x268, 1),
            (0, 0x1000, 1, 0x11, 0x269, 1),
            (0xffff, 0, 1, 0x11, 0x269, 0),
            (0xffff, 0, 1, 0, 0x269, 1),
            (0xffff, 0, 1, 1, 0x268, 1),
            (0xffff, 0, 1, 0x70, 0x268, 1),
            (0xffff, 0, 1, 0x6f, 0x269, 0),
            (0xffff, 0, 1, 0x40, 0x269, 0),
        ].into_iter().enumerate() {
            let expected = match advanced {
                Some(next) if reads != 0 => next,
                Some(_) => address,
                None => 0x268 | carry | ((table & 1) << 12) | ((table & 0x100) << 5),
            };
            writeln!(calls, "value = {count:#x}; reads = 0; \
                             if (discard_{row}({incoming:#x}, 0, {cfg:#x}, {table:#x}) != {expected:#x} \
                                 || reads != {reads} || value != {count:#x}) return {};", row * 10 + arm + 1).unwrap();
        }
    }
    writeln!(c, "int main(void) {{\n{calls}\nreturn 0;\n}}").unwrap();
    fs::write(&source, &c).unwrap();
    run_emitted_c(&source, &executable, &c);
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
fn carmel_original_wide_bitfield_ports_execute_in_emitted_c() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let binary = common::scratch_file("carmel-original-wide-bits", "bin");
    let source = binary.with_extension("c");
    let executable = binary.with_extension("exe");
    let mut emitted = String::from(
        "#include <stdbool.h>\n#include <stdint.h>\n\
         #define CONCAT22(h,l) (((uint32_t)(uint16_t)(h) << 16) | (uint16_t)(l))\n\
         volatile unsigned short word_0000, word_0040, word_0041, word_0048, word_02ca, word_02cb, word_8012,\n\
         word_4048, word_4049, word_804a, word_c048, word_c049, word_c04f, word_ffff;\n",
    );
    let routes: [([u8; 6], Option<(u8, u16)>, &[&str], &[u16], &[&str]); 13] = [
        ([0xe9, 0xb4, 0x0a, 0xf6, 0x31, 0xb1], Some((0, 0x40)),
            &["PACFG", "RM0", "RL0", "FF1", "FF2"], &[0x40, 0xffff], &["EST", "R0"]),
        ([0xb0, 0xe3, 0x11, 0xf2, 0x11, 0xfb], Some((0, 0x40)),
            &["PACFG", "RM0", "RL0", "RN0", "ECFG"], &[0x40, 0xffff], &["EST", "R0"]),
        ([0xf9, 0x05, 0x88, 0xf3, 0x0b, 0x71], None,
            &["RF0", "ASTATUS"], &[], &["EST"]),
        ([0xea, 0x75, 0xbf, 0xf2, 0x21, 0xe9], Some((0, 0x40)),
            &["PACFG", "RM0", "RL0", "RN2", "ECFG"], &[0x40, 0xffff], &["EST", "R0"]),
        ([0xa6, 0x02, 0xca, 0xf3, 0xce, 0xf2], Some((6, 0x8048)),
            &[], &[0x02ca, 0x02cb, 0x8012, 0x804a], &["EST"]),
        ([0xf4, 0x28, 0x9e, 0xf3, 0xa7, 0x8c], Some((4, 0x48)),
            &["PACFG", "RM4", "RL4", "BTR1"], &[0x48], &["EST", "R4"]),
        ([0xed, 0xe2, 0x07, 0xf3, 0x88, 0xef], Some((7, 0xc048)),
            &["PACFG", "RM6", "RL6", "RN5", "ECFG"], &[0xc048, 0xc049, 0xc04f], &["EST", "R7", "CETBL"]),
        ([0xcf, 0x14, 0x20, 0xf7, 0x10, 0x01], Some((0, 0x40)),
            &[], &[0x40, 0x41], &["A0L", "A0H", "EST"]),
        ([0xcf, 0x14, 0x20, 0xf7, 0x10, 0x81], Some((0, 0x40)),
            &[], &[0x40, 0x41], &["A0L", "A0H", "EST"]),
        ([0xcf, 0x14, 0x20, 0xf7, 0x10, 0x8f], Some((7, 0xffff)),
            &[], &[0xffff, 0], &["A0L", "A0H", "R7"]),
        ([0xcf, 0, 0x12, 0xf7, 0x10, 0x01], Some((0, 0x40)),
            &[], &[0x40], &["A0L", "EST"]),
        ([0xed, 0xe2, 0x07, 0xf3, 0x20, 0xef], Some((7, 0x4048)),
            &["PACFG", "RM6", "RL6", "RN5", "ECFG"], &[0x4048, 0x4049], &["A0L", "EST"]),
        ([0xed, 0xe2, 0x07, 0xf3, 0x22, 0xef], Some((7, 0xffff)),
            &["PACFG", "RM6", "RL6", "RN5", "ECFG"], &[0xffff, 0], &["A1L", "R7"]),
    ];
    for (route, (packet, pointer, inputs, cells, outputs)) in routes.into_iter().enumerate() {
        let mut code = Vec::new();
        if let Some((r, address)) = pointer {
            let [hi, lo] = address.to_be_bytes();
            code.extend([0x80, hi, lo, 0xc6, 0x42, 0x80 + 4 * r]);
        }
        code.extend(packet);
        if route == 6 {
            code.extend([0x2b, 0xec, 0, 0x2b, 0xec, 0]); // Sample CETBL on the next issue.
        }
        code.extend([0x2b, 0xd8, 0, 0x2b, 0xec, 0]);
        fs::write(&binary, code).unwrap();
        for reg in outputs {
            let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
            kuna.arg("decompile").arg(&binary).args([
                "0", "--addr", "--raw-image", "--target", "Carmel:BE:24:default:default",
                "--base", "0", "--json", "--assert-strict",
                "--assert", &format!("return %{reg} unsigned short"),
            ]);
            for (i, name) in ["CECFG1", "CECFG2", "CETBL", "EST"].into_iter()
                .chain(inputs.iter().copied()).enumerate()
            {
                kuna.args(["--assert", &format!("param {i} %{name} unsigned short in_{}", name.to_ascii_lowercase())]);
            }
            for address in cells {
                kuna.args(["--assert", &format!("data [data,0x{address:x}] volatile unsigned short word_{address:04x}")]);
            }
            let output = common::process::required_output(&mut kuna);
            let json: Value = serde_json::from_slice(&output.stdout).unwrap();
            let function = &json["functions"][0];
            assert!(function["error"].is_null(), "{function}");
            assert!(json["assertions"].as_array().unwrap().iter().all(|a| a["status"] == "applied"), "{json}");
            let name = format!("route_{route}_{}", reg.to_ascii_lowercase());
            writeln!(emitted, "{}", function["code"].as_str().unwrap().replace("sub_0", &name)).unwrap();
        }
    }
    emitted.push_str(r"
int main(void) {
    word_0040 = 0x1234; word_ffff = 0xffff;
    if (route_0_est(0x1000,0x1000,0,0,0,0,0,0,12) != 8 || word_0040 != 0) return 1;
    if (route_0_est(0x1000,0x1000,0xffff,0,0,0,0,0,0) != 0x3008 ||
        route_0_r0(0x1000,0x1000,0xffff,0,2,3,0,31,31) != 0x43) return 2;
    word_ffff = 0x1234;
    if (route_1_est(0x1000,0x1000,0,0,0,0,3,3,0) != 0x20 || word_0040 != 0x91a0) return 3;
    if (route_1_r0(0x1000,0x1000,0xffff,0,1,0,0,0x80,0) != 0xc0) return 4;
    if (route_2_est(0x1000,0x1000,0,0,0xffff,0x7f) != 1) return 5;
    if (route_2_est(0x1000,0x1000,0xffff,0,0,0x7f) != 0x3000) return 6;
    word_ffff = 0x8001;
    if (route_3_est(0x1000,0x1000,0,0,0,0,0,0x7f,0) != 0x11 || word_0040 != 0xc000) return 7;
    if (route_3_r0(0x1000,0x1000,0xffff,0,3,0,0x40,0x7f,0) != 0x40) return 8;
    word_02ca = 1; word_02cb = 0; word_8012 = 0; word_804a = 0x7f;
    if (route_4_est(0x1000,0x1000,0,0) != 0) return 9;
    word_8012 = 0x7f;
    if (route_4_est(0x1000,0x1000,0xffff,0) != 0x3001) return 10;
    word_02ca = 0; word_02cb = 1; word_8012 = 16;
    if (route_4_est(0x1000,0x1000,0,0) != 1) return 11;
    word_0048 = 0x7f;
    if (route_5_r4(0x1000,0x1000,0,0,0,0,0,0xffff) != 0xffe6 ||
        route_5_est(0x1000,0x1000,0xffff,0,0,0,0,0xffff) != 0x3000) return 12;
    word_c048 = 0x5678; word_c049 = 0x9234; word_c04f = 0x3210;
    if (route_6_cetbl(0x1000,0x1000,0,0,0,0,0,0x7f,0) != 0xc91a) return 13;
    if (route_6_est(0x1000,0x1000,0xffff,0,0,0,0,0x7f,0) != 0x2010) return 14;
    if (route_6_r7(0x1000,0x1000,0,0,0x8000,7,0,0x7f,0) != 0xc047) return 15;
    if (word_c048 != 0x5678 || word_c049 != 0x9234 || word_c04f != 0x3210 ||
        word_ffff != 0x8001 || word_0048 != 0x7f || word_02cb != 1 || word_8012 != 16) return 16;
    word_c048 = 0; word_c049 = 0x4000;
    if (route_6_cetbl(0x1000,0x1000,0,0,0,0,0,1,4) != 0x7fff ||
        route_6_est(0x1000,0x1000,0,0,0,0,0,1,4) != 0x3024) return 17;
    word_c048 = 0x8765; word_c049 = 0x9234;
    if (route_6_cetbl(0x1000,0x1000,0,0,0,0,0,2,4) != 0x8000 ||
        route_6_est(0x1000,0x1000,0,0,0,0,0,2,4) != 0x34) return 27;
    if (route_2_est(0x1000,0x1000,0xffff,9,0xffff,0) != 0x3009) return 18;
    word_0040 = 0x802b; word_0041 = 0x4321;
    if (route_7_a0l(0x1000,0x1000,0,1) != 0x802b || route_7_a0h(0x1000,0x1000,0,1) != 0xf ||
        route_7_est(0x1000,0x1000,0,0xcf37) != 0) return 19;
    if (route_8_a0l(0x1000,0x1000,0,1) != 0x802b || route_8_a0h(0x1000,0x1000,0,1) != 1 ||
        route_8_est(0x1000,0x1000,0xffff,0x15) != 0x3000) return 20;
    word_ffff = 0x712d; word_0000 = 0xcab1;
    if (route_9_a0l(0x1000,0x1000,0,0) != 0x712d || route_9_a0h(0x1000,0x1000,0,0) != 1 ||
        route_9_r7(0x1000,0x1000,0,0) != 0xffff) return 21;
    if (route_10_a0l(0x1000,0x1000,0,0) != 0 ||
        route_10_est(0x1000,0x1000,0xffff,0x15) != 0x3008) return 22;
    word_4048 = 0x8765; word_4049 = 0x9234;
    if (route_11_a0l(0x1000,0x1000,0,0,0,0,0,1,0) != 0x2469 ||
        route_11_est(0x1000,0x1000,0,0,0,0,0,1,0) != 0x31) return 23;
    if (route_11_a0l(0x1000,0x1000,0,0,0,0,0,2,4) != 0x8000 ||
        route_11_est(0x1000,0x1000,0,0,0,0,0,2,4) != 0x34) return 24;
    if (route_11_a0l(0x1000,0x1000,0,0,0,0,0,0x40,0) != 0xffff ||
        route_11_est(0x1000,0x1000,0,0,0,0,0,0x40,0) != 0x11) return 25;
    word_4049 = 0x6dcb;
    if (route_11_a0l(0x1000,0x1000,0,0,0,0,0,2,4) != 0x7fff ||
        route_11_est(0x1000,0x1000,0,0,0,0,0,2,4) != 0x25 ||
        word_4048 != 0x8765 || word_4049 != 0x6dcb) return 28;
    word_ffff = 0x1234; word_0000 = 0xe312;
    if (route_12_a1l(0x1000,0x1000,0,0,0,0,0,7,0) != 0x8909 ||
        route_12_r7(0x1000,0x1000,0,0,0,0,0,7,0) != 6) return 26;
    return 0;
}
");
    fs::write(&source, &emitted).unwrap();
    run_emitted_c(&source, &executable, &emitted);
    for file in [binary, source, executable] {
        fs::remove_file(file).unwrap();
    }
}

#[test]
fn carmel_wide_immediate_ashift_samples_data_address_and_result_lane() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let binary = common::scratch_file("carmel-wide-immediate-ashift", "bin");
    let source = binary.with_extension("c");
    let executable = binary.with_extension("exe");
    let mut c = String::from(
        "#include <stdbool.h>\n#include <stdint.h>\n\
         #define CONCAT12(h,l) (((uint32_t)(uint8_t)(h) << 16) | (uint16_t)(l))\n\
         #define CONCAT14(h,l) (((uint64_t)(uint8_t)(h) << 32) | (uint32_t)(l))\n\
         #define CONCAT22(h,l) (((uint32_t)(uint16_t)(h) << 16) | (uint16_t)(l))\n\
         volatile unsigned short word_0040, word_0041, word_ffff;\n",
    );
    for (name, packet, setup, younger) in [
        ("rmw", [0xcf, 0x08, 0x04, 0xf3, 0x0c, 0x01],
            Some([0x80, 0, 0x40, 0xc6, 0x42, 0x80]), Some([0x80, 0, 0x41, 0xc6, 0x42, 0x80])),
        ("rmw_high", [0xcf, 0, 0x7f, 0xf3, 0x0c, 0x0f],
            Some([0x80, 0xff, 0xff, 0xc6, 0x42, 0x9c]), None),
        ("rmw_limit", [0xcf, 0xff, 0x8f, 0xf3, 0x0c, 0x01],
            Some([0x80, 0xff, 0xff, 0xc6, 0x42, 0x80]), None),
        ("word", [0xcf, 0, 0x40, 0xf3, 0x21, 0x13], None, None),
        ("word_left", [0xcf, 0xa5, 1, 0xf3, 0x21, 0x13], None, None),
        ("word_right", [0xcf, 0xa5, 0x7f, 0xf3, 0x21, 0x13], None, None),
        ("word_limit", [0xcf, 0xff, 0x80, 0xf3, 0x21, 0x13], None, None),
    ] {
        let code: Vec<u8> = setup.into_iter().chain([packet]).chain(younger)
            .chain([[0x2b, 0xd8, 0, 0x2b, 0xec, 0]]).flatten().collect();
        fs::write(&binary, code).unwrap();
        let inputs: &[&str] = if setup.is_some() { &[] } else { &["A1L", "A1H", "GB1"] };
        let outputs: &[&str] = if setup.is_some() { &["EST"] } else { &["EST", "A0L"] };
        for reg in outputs {
            let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
            kuna.arg("decompile").arg(&binary).args([
                "0", "--addr", "--raw-image", "--target", "Carmel:BE:24:default:default",
                "--base", "0", "--json", "--assert-strict",
                "--assert", &format!("return %{reg} unsigned short"),
            ]);
            for (i, input) in ["EST", "ECFG", "CECFG1", "CECFG2", "CETBL"].into_iter()
                .chain(inputs.iter().copied()).enumerate()
            {
                let ty = if input == "GB1" { "unsigned char" } else { "unsigned short" };
                kuna.args(["--assert", &format!("param {i} %{input} {ty} in_{}", input.to_ascii_lowercase())]);
            }
            for addr in [0x40, 0x41, 0xffff] {
                kuna.args(["--assert", &format!("data [data,0x{addr:x}] volatile unsigned short word_{addr:04x}")]);
            }
            let output = common::process::required_output(&mut kuna);
            let json: Value = serde_json::from_slice(&output.stdout).unwrap();
            let function = &json["functions"][0];
            assert!(function["error"].is_null(), "{function}");
            assert!(json["assertions"].as_array().unwrap().iter().all(|a| a["status"] == "applied"), "{json}");
            let emitted = function["code"].as_str().unwrap();
            assert!(!emitted.contains("halt_"), "{function}");
            let name = format!("{name}_{}", reg.to_ascii_lowercase());
            writeln!(c, "{}", emitted.replace("sub_0", &name)).unwrap();
        }
    }
    c.push_str(r"
int main(void) {
    word_0040 = 0x0318; word_0041 = 0x1234; word_ffff = 0x8001;
    if (rmw_est(0,0,0x1000,0x1000,0) != 0 || word_0040 != 0x3180 ||
        word_0041 != 0x1234 || word_ffff != 0x8001) return 1;
    word_0040 = 0x0800;
    if (rmw_est(0,0,0x1000,0x1000,0) != 0x20 || word_0040 != 0x8000) return 2;
    word_0040 = 0x1000;
    if (rmw_est(0,0,0x1000,0x1000,0) != 0x21 || word_0040 != 0) return 3;
    if (rmw_high_est(0,0,0x1000,0x1000,0) != 0x11 || word_ffff != 0xc000 ||
        word_0040 != 0 || word_0041 != 0x1234) return 4;
    word_ffff = 0x95a7;
    if (rmw_limit_est(0,4,0x1000,0x1000,0) != 0x35 || word_ffff != 0x8000) return 5;
    if (word_a0l(0,0,0x1000,0x1000,0,0xabcd,0x8765,0x12) != 0 ||
        word_est(0,0,0x1000,0x1000,0,0xabcd,0x8765,0x12) != 8) return 6;
    if (word_a0l(0,0,0x1000,0x1000,0,0,0,0x80) != 0xffff ||
        word_est(0,0,0x1000,0x1000,0,0,0,0x80) != 0x11) return 7;
    if (word_left_a0l(0,0,0x1000,0x1000,0,0,0x8000,0) != 0 ||
        word_left_est(0,0,0x1000,0x1000,0,0,0x8000,0) != 0x20 ||
        word_left_est(0,0,0x1000,0x1000,0,0,0,0x80) != 9) return 8;
    if (word_right_a0l(0,0,0x1000,0x1000,0,0,2,0) != 1 ||
        word_right_est(0,0,0x1000,0x1000,0,0,1,0) != 9) return 9;
    if (word_limit_a0l(0,4,0x1000,0x1000,0,0,0,0x12) != 0x7fff ||
        word_limit_est(0,4,0x1000,0x1000,0,0,0,0x12) != 0x24) return 10;
    return 0;
}
");
    fs::write(&source, &c).unwrap();
    run_emitted_c(&source, &executable, &c);
    for file in [binary, source, executable] {
        fs::remove_file(file).unwrap();
    }
}

#[test]
fn carmel_wide_extract_selects_descriptor_and_result_ports() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    type Row<'a> = ([u8; 6], &'a [(u8, u16)], &'a [(&'a str, u32)],
                    &'a [(u16, u16, u16)], &'a [(&'a str, u32)]);
    let long_gb2 = [0xce, 0xbe, 0x12, 0xf6, 0x85, 0x04];
    let long_ff2 = [0xb4, 0xa4, 0x27, 0xf7, 0x7d, 0x2e];
    let ce1_gb3 = [0x1e, 0x5f, 0x54, 0xf6, 0x19, 0x07];
    let gb3_inputs = &[("FF1", 1), ("FF2", 7), ("X1", 0xe0),
                       ("RN4", 1), ("PACFG", 0), ("RM4", 0), ("RL4", 0)];
    let routes: &[Row<'_>] = &[
        (long_gb2, &[(2, 0xffff), (6, 0x8048)],
         &[("CECFG1", 0x1000), ("CECFG2", 0x1000), ("CETBL", 0), ("EST", 1),
           ("GB2", 0xf3), ("RN6", 3), ("PACFG", 0), ("RM6", 0), ("RL6", 0)],
         &[(0x8048, 8, 8), (0x8049, 7, 7), (0xffff, 0xabcd, 1), (0, 0x1234, 0)],
         &[("EST", 1), ("R6", 0x804b), ("R2", 0xffff), ("GB2", 0xf3)]),
        (long_ff2, &[(4, 0x48)],
         &[("CECFG1", 0x1000), ("CECFG2", 0x1000), ("CETBL", 0xffff), ("EST", 8),
           ("FF1", 0xa567), ("FF2", 0x8000), ("PACFG", 0x200), ("RM4", 0x3f), ("RL4", 0)],
         &[(0x48, 5, 5), (0x49, 16, 16), (0x2f, 6, 6)],
         &[("FF1", 31), ("R4", 0x2f), ("EST", 0x3000)]),
        ([0x9b, 0x87, 0x83, 0xf5, 0xc9, 0x6e], &[(3, 0xc040)],
         &[("CECFG1", 0x1000), ("CECFG2", 0x1000), ("CETBL", 0xffff), ("EST", 9),
           ("PACFG", 0), ("RM2", 0), ("RL2", 0)],
         &[(0xc040, 31, 31), (0xc041, 7, 7)],
         &[("R3", 0xc03e), ("EST", 0x3009)]),
        ([0xf6, 0xcd, 0x87, 0xf5, 0xa4, 0x17], &[(3, 0xc040)],
         &[("CECFG1", 0x1000), ("CECFG2", 0x1000), ("CETBL", 0), ("EST", 8),
           ("LSA1W", 0x123456), ("RN3", 5), ("PACFG", 0xc0), ("RM2", 7), ("RL2", 0xc03d)],
         &[(0xc040, 0x8001, 0x8001), (0xc03d, 0x1234, 0x1234)],
         &[("R3", 0xc03d), ("LSA1W", 0x123456), ("EST", 8)]),
        ([0xfc, 0x89, 0x61, 0xf5, 0x8e, 0x8b], &[(5, 0x4048)],
         &[("CECFG1", 0x2000), ("CECFG2", 0x2000), ("CETBL", 0), ("EST", 0x24),
           ("FF1", 0x113), ("FF2", 0x105)],
         &[(0x4048, 0x5678, 0x5678), (0x4049, 0xbc92, 0xbc92)],
         &[("BTR2", 0xfff2), ("R5", 0x4048), ("EST", 0x34)]),
        ([0xfe, 0xa5, 0x2f, 0xf5, 0x8e, 0x8b], &[(5, 0xffff)],
         &[("CECFG1", 0x1000), ("CECFG2", 0x1000), ("CETBL", 0xffff), ("EST", 0),
           ("FF1", 0), ("FF2", 20)],
         &[(0xffff, 0, 0), (0, 8, 8)],
         &[("BTR2", 0), ("R5", 0xffff), ("EST", 0x3010)]),
        ([0xfc, 0x89, 0x61, 0xf7, 0x8e, 0x8b], &[(5, 0x4048)],
         &[("CECFG1", 0x2000), ("CECFG2", 0x2000), ("CETBL", 0), ("EST", 0x11),
           ("FF1", 31), ("FF2", 5)],
         &[(0x4048, 0x5678, 0x5678), (0x4049, 0xbc92, 0xbc92)],
         &[("BTR2", 31), ("R5", 0x4048), ("EST", 1)]),
        ([0xfe, 0x89, 0x61, 0xf7, 0x8e, 0x8b], &[(5, 0x4048)],
         &[("CECFG1", 0x1000), ("CECFG2", 0x1000), ("CETBL", 0), ("EST", 0x10),
           ("FF1", 19), ("FF2", 0)],
         &[(0x4048, 0x5678, 0x5678), (0x4049, 0xbc92, 0xbc92)],
         &[("BTR2", 0), ("R5", 0x4048), ("EST", 8)]),
        ([0xfc, 0x89, 0x61, 0xf5, 0x8e, 0x8b], &[(5, 0x4048)],
         &[("CECFG1", 0x0888), ("CECFG2", 0x0888), ("CETBL", 0x8080), ("EST", 8),
           ("FF1", 19), ("FF2", 5)],
         &[(0x4048, 0x5678, 0x5678), (0x4049, 0xbc92, 0xbc92)],
         &[("BTR2", 0xfff2), ("EST", 0x3010)]),
    ];
    let binary = common::scratch_file("carmel-wide-extract-ports", "bin");
    let source = binary.with_extension("c");
    let executable = binary.with_extension("exe");
    for opcode in [0xf5, 0xf7] {
        fs::write(&binary, [0xfd, 0x89, 0x61, opcode, 0x8e, 0x8b]).unwrap();
        let (stdout, stderr, status) = common::run_kuna(&[
            "raw-pcode", binary.to_str().unwrap(), "0x1000", "--addr", "--raw-image", "--target",
            "Carmel:BE:24:default:default", "--base", "0x1000", "--count", "1", "--json",
        ]);
        let doc: Value = serde_json::from_str(&stdout).unwrap_or_else(|_| panic!("{stdout}\n{stderr}"));
        assert_eq!(status, 1, "{doc}");
        assert_eq!(doc["error"]["address"], 0x1000, "{doc}");
        assert!(doc["instructions"].as_array().unwrap().is_empty(), "{doc}");
        assert!(doc["error"]["message"].as_str().unwrap().contains("rejected before issue"), "{doc}");
    }
    let ce_rows = [
        (0x1000, 0x1000, 0xffff, 0, 0xf3, 0x79, 0x49, 0x3000),
        (0x1000, 0x1000, 0, 1, 0xf3, 0x5df9, 0x49, 1),
        (0, 0x1000, 0, 1, 0xf3, 0x5df9, 0x48, 1),
        (0x1888, 0x0888, 0x8001, 0, 0, 0, 0x49, 0x1008),
    ];
    let ce_inputs: Vec<Vec<_>> = ce_rows.iter().map(|&(cfg1, cfg2, table, flags, gb3, ..)| {
        [("CECFG1", cfg1), ("CECFG2", cfg2), ("CETBL", table), ("EST", flags), ("GB3", gb3)]
            .into_iter().chain(gb3_inputs.iter().copied()).collect()
    }).collect();
    let ce_words: Vec<_> = ce_rows.iter().map(|&(_, _, _, _, _, word, _, _)| [(0x48, 0x5df9, word)]).collect();
    let ce_outputs: Vec<_> = ce_rows.iter().map(|&(_, _, _, _, _, _, ptr, flags)| [("R4", ptr), ("EST", flags)]).collect();
    let cases = routes.iter().copied().chain((0..ce_rows.len()).map(|i| {
        (ce1_gb3, &[(4, 0x48)][..], ce_inputs[i].as_slice(), &ce_words[i][..], &ce_outputs[i][..])
    }));
    let mut generated = String::from(
        "#include <stdbool.h>\n#include <stdint.h>\n#include <stdio.h>\n\
         #define CONCAT22(h,l) (((uint32_t)(uint16_t)(h) << 16) | (uint16_t)(l))\n",
    );
    let mut calls = String::from("int main(void) {\n");
    let c_type = |reg: &str| match reg {
        "GB2" | "GB3" => "unsigned char",
        "LSA1W" => "unsigned long",
        _ => "unsigned short",
    };
    for (i, (packet, pointers, inputs, cells, outputs)) in cases.enumerate() {
        let mut code = Vec::new();
        for &(r, address) in pointers {
            let [hi, lo] = address.to_be_bytes();
            code.extend([0x80, hi, lo, 0xc6, 0x42, 0x80 + 4 * r]);
        }
        code.extend(packet);
        code.extend([0x2b, 0xd8, 0, 0x2b, 0xec, 0]);
        fs::write(&binary, code).unwrap();
        let actual = inputs.iter().map(|(_, val)| val.to_string()).collect::<Vec<_>>().join(",");
        for &(address, _, _) in cells {
            writeln!(generated, "volatile unsigned short word_{i}_{address:04x};").unwrap();
        }
        for &(reg, expected) in outputs {
            let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
            kuna.arg("decompile").arg(&binary).args([
                "0x1000", "--addr", "--raw-image", "--target", "Carmel:BE:24:default:default",
                "--base", "0x1000", "--json", "--assert-strict",
                "--assert", &format!("return %{reg} {}", c_type(reg)),
            ]);
            for (j, &(input, _)) in inputs.iter().enumerate() {
                kuna.args(["--assert", &format!("param {j} %{input} {} in_{}", c_type(input), input.to_ascii_lowercase())]);
            }
            for &(address, _, _) in cells {
                kuna.args(["--assert", &format!("data [data,{address:#x}] volatile unsigned short word_{i}_{address:04x}")]);
            }
            let result = common::process::required_output(&mut kuna);
            let json: Value = serde_json::from_slice(&result.stdout).unwrap();
            let function = &json["functions"][0];
            let text = function["code"].as_str().unwrap();
            assert!(function["error"].is_null() && !text.contains("halt_"), "{function}");
            assert!(json["assertions"].as_array().unwrap().iter().all(|a| a["status"] == "applied"), "{json}");
            let name = format!("extract_{i}_{}", reg.to_ascii_lowercase());
            writeln!(generated, "#define sub_1000 {name}\n{text}\n#undef sub_1000").unwrap();
            for &(address, before, _) in cells {
                writeln!(calls, "word_{i}_{address:04x} = {before};").unwrap();
            }
            write!(calls, "if ({name}({actual}) != {expected}").unwrap();
            for &(address, _, after) in cells {
                write!(calls, " || word_{i}_{address:04x} != {after}").unwrap();
            }
            writeln!(calls, ") {{ fputs(\"{name}\\n\", stderr); return 1; }}").unwrap();
        }
    }
    generated.push_str(&calls);
    generated.push_str("return 0;\n}\n");
    fs::write(&source, &generated).unwrap();
    run_emitted_c(&source, &executable, &generated);
    for file in [binary, source, executable] {
        fs::remove_file(file).unwrap();
    }
}

#[test]
fn carmel_wide_shift_ce_samples_old_feedback() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let binary = common::scratch_file("carmel-wide-shift-ce", "bin");
    let source = binary.with_extension("c");
    let executable = binary.with_extension("exe");
    let mut generated = String::from(
        "#include <stdbool.h>\n#include <stdint.h>\n\
         #define CONCAT22(h,l) (((uint32_t)(uint16_t)(h) << 16) | (uint16_t)(l))\n\
         volatile unsigned short shift_low, shift_high;\n",
    );
    for (port_byte, port) in [(0x84, "CECFG1"), (0x86, "CECFG2"), (0x88, "CETBL")] {
        let code = [
            [0x80, 0xc0, 0x48, 0xc6, 0x42, 0x9c], // MOVE R7, 0xc048
            [0xed, 0xe2, 0x07, 0xf3, port_byte, 0xef],
            [0x2b, 0xd8, 0, 0x2b, 0xec, 0],
        ];
        fs::write(&binary, code.as_flattened()).unwrap();
        for (suffix, output) in [("result", port), ("feedback", "EST")] {
            let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
            kuna.arg("decompile").arg(&binary).args([
                "0", "--addr", "--raw-image", "--target", "Carmel:BE:24:default:default",
                "--base", "0", "--json", "--assert-strict",
                "--assert", &format!("return %{output} unsigned short"),
                "--assert", "data [data,0xc048] volatile unsigned short shift_low",
                "--assert", "data [data,0xc049] volatile unsigned short shift_high",
            ]);
            for (i, reg) in ["CECFG1", "CECFG2", "CETBL", "EST", "RN5", "ECFG", "PACFG", "RM6", "RL6"]
                .into_iter().enumerate()
            {
                kuna.args(["--assert", &format!("param {i} %{reg} unsigned short in_{}", reg.to_ascii_lowercase())]);
            }
            let output = common::process::required_output(&mut kuna);
            let json: Value = serde_json::from_slice(&output.stdout).unwrap();
            let function = &json["functions"][0];
            assert!(function["error"].is_null(), "{function}");
            assert!(json["assertions"].as_array().unwrap().iter().all(|a| a["status"] == "applied"), "{json}");
            let label = format!("{}_{suffix}", port.to_ascii_lowercase());
            writeln!(generated, "{}", function["code"].as_str().unwrap().replace("sub_0", &label)).unwrap();
        }
    }
    generated.push_str(r"
int main(void) {
    /* A newly set Z must not index the incoming truth table yet. */
    if (cecfg1_feedback(0x1888,0x1888,0x8080,0,0,0,0,0,0) != 8 ||
        cecfg2_feedback(0x1888,0x1888,0x8080,0,0,0,0,0,0) != 8 ||
        cetbl_feedback(0x1888,0x1888,0x8080,0,0,0,0,0,0) != 8) return 1;
    /* Entering static mode retains the selected bank's old dynamic result. */
    shift_high = 0x2111;
    if (cecfg1_result(0x1888,0x1888,0x0101,0,0,0,0,0,0) != 0x2111 ||
        cecfg2_result(0x1888,0x1888,0x0101,0,0,0,0,0,0) != 0x2111 ||
        cecfg1_feedback(0x1888,0x1888,0x0101,0,0,0,0,0,0) != 0x3000 ||
        cecfg2_feedback(0x1888,0x1888,0x0101,0,0,0,0,0,0) != 0x3000) return 2;
    shift_high = 1;
    if (cetbl_result(0x1000,0x1000,0xff00,0,0,0,0,0,0) != 1 ||
        cetbl_feedback(0x1000,0x1000,0xff00,0,0,0,0,0,0) != 0x2000) return 3;
    return 0;
}
");
    fs::write(&source, &generated).unwrap();
    run_emitted_c(&source, &executable, &generated);
    for file in [binary, source, executable] {
        fs::remove_file(file).unwrap();
    }
}

#[test]
fn carmel_lc1_requires_address_admission_and_later_result_enable() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let binary = common::scratch_file("carmel-lc1-phases", "bin");
    let source = binary.with_extension("c");
    let executable = binary.with_extension("exe");
    let mut generated = String::from(
        "#include <stdbool.h>\n#include <stdint.h>\nvolatile unsigned short phase_word = 0x00ff;\n",
    );
    const NOP: [u8; 6] = [0x2b, 0xec, 0, 0x2b, 0xec, 0];
    for (label, loadce) in [("split", true), ("stable", false)] {
        let mut code = vec![[0x80, 0xc0, 0x40, 0xc6, 0x42, 0x8c]]; // MOVE R3, 0xc040
        code.extend([NOP; 6]);
        if loadce {
            code.push([0x8c, 0, 1, 0xcd, 0xe4, 0x33]); // LOADCE 0x0ccc,1
            code.extend([NOP; 4]);
        }
        code.push([0x13, 0x34, 0x68, 0xc2, 0x2f, 0x50]); // NOT CE1 LC1,*R3++
        code.extend([NOP; 4]);
        code.push([0x2b, 0xd8, 0, 0x2b, 0xec, 0]);
        code.extend([NOP; 2]);
        fs::write(&binary, code.as_flattened()).unwrap();
        if loadce {
            let short = Command::new(env!("CARGO_BIN_EXE_kuna")).arg("decompile").arg(&binary).args([
                "0x1010", "--addr", "--raw-image", "--base", "0x1000",
                "--target", "Carmel:BE:24:default:default", "--json",
            ]).output().unwrap();
            let error: Value = serde_json::from_slice(&short.stdout).unwrap();
            assert!(!short.status.success() && error["functions"][0]["code"].is_null());
        }
        for register in ["R3", "LC1"] {
            let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
            kuna.arg("decompile").arg(&binary).args([
                "0x1000", "--addr", "--raw-image", "--base", "0x1000",
                "--target", "Carmel:BE:24:default:default", "--json", "--assert-strict",
                "--assert", &format!("return %{register} unsigned short"),
                "--assert", "data [data,0xc040] volatile unsigned short phase_word",
            ]);
            for (i, input) in ["R3", "LC1", "EST", "CECFG1", "CECFG2", "CETBL", "PACFG", "RM2", "RL2"]
                .into_iter().enumerate()
            {
                kuna.args(["--assert", &format!("param {i} %{input} unsigned short in_{}", input.to_ascii_lowercase())]);
            }
            let output = common::process::required_output(&mut kuna);
            let json: Value = serde_json::from_slice(&output.stdout).unwrap();
            let function = &json["functions"][0];
            assert!(function["error"].is_null(), "{function}");
            let name = format!("{label}_{}", register.to_ascii_lowercase());
            writeln!(generated, "{}", function["code"].as_str().unwrap().replace("sub_1000", &name)).unwrap();
        }
    }
    generated.push_str(r"
#define INPUTS(table) 0xc040,0x4567,0,0,0,table,0,0,0
int main(void) {
    /* Early admission updates R3, even though the later result is vetoed. */
    if (split_r3(INPUTS(0)) != 0xc041 || split_lc1(INPUTS(0)) != 0x4567) return 1;
    /* A late true rail cannot reopen the earlier veto. */
    if (split_r3(INPUTS(0xffff)) != 0xc040 || split_lc1(INPUTS(0xffff)) != 0x4567) return 2;
    if (stable_r3(INPUTS(0xffff)) != 0xc041 || stable_lc1(INPUTS(0xffff)) != 0xff00) return 3;
    return 0;
}
");
    fs::write(&source, &generated).unwrap();
    run_emitted_c(&source, &executable, &generated);
    for file in [binary, source, executable] {
        fs::remove_file(file).unwrap();
    }
}

#[test]
fn carmel_qualified_wide_transfer_uses_early_address_enable() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    const NOP: [u8; 6] = [0x2b, 0xec, 0, 0x2b, 0xec, 0];
    const INC: [u8; 6] = [0x21, 0xac, 0x20, 0x2b, 0xec, 0];
    const RET: [u8; 6] = [0x2b, 0xd8, 0, 0x2b, 0xec, 0];
    let scratch = tempfile::tempdir().unwrap();
    let binary = scratch.path().join("transfer.bin");
    let source = scratch.path().join("transfer.c");
    let executable = scratch.path().join("transfer");
    let specs = scratch.path().join("specs");
    tracked_carmel_entry_specs(
        &specs,
        &[("A0L", 0xffff), ("ECFG", 0), ("R3", 0xc040), ("PACFG", 0), ("INTBASE", 2), ("RL0", 0x1234)],
        0,
        0,
    );
    let mut c = String::from("#include <stdbool.h>\n#include <stdint.h>\n");
    let mut checks = String::new();
    for (r, (left, tail, reader, output, held, written)) in [
        (0x36, [0xc0, 0xc0, 0x89], false, "R2", 0x6666, 0x21b8),
        (0x76, [0xc0, 0xac, 0x89], false, "R2", 0x6666, 0x21bb),
        (0x36, [0xc0, 0x46, 0xc1], false, "RL0", 0x1234, 0xe1f9),
        (0x36, [0xc0, 0x47, 0x61], true, "R2", 0x21b9, 0x03b2),
    ].into_iter().enumerate() {
        let mut code = vec![INC, NOP, NOP, NOP, NOP, [left, 0x21, 0xb9, tail[0], tail[1], tail[2]]];
        if reader {
            code.push([0x76, 0x21, 0xb9, 0xc0, 0xb0, 0x89]);
        }
        code.extend([NOP, RET, NOP]);
        fs::write(&binary, code.as_flattened()).unwrap();
        let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
        kuna.env("KUNA_SPECS", &specs).arg("decompile").arg(&binary).args([
            "0", "--addr", "--raw-image", "--target", "Carmel:BE:24:default:default", "--base", "0",
            "--json", "--assert-strict", "--assert", &format!("return %{output} unsigned short"),
        ]);
        for (i, reg) in ["R2", "CETBL", "IOSPACE_ADDR", "CECFG1", "CECFG2", "EST"].into_iter().enumerate() {
            kuna.args(["--assert", &format!("param {i} %{reg} unsigned short in_{i}")]);
        }
        let output = common::process::required_output(&mut kuna);
        let json: Value = serde_json::from_slice(&output.stdout).unwrap();
        let function = &json["functions"][0];
        assert!(function["error"].is_null(), "{function}");
        let name = format!("transfer_{r}");
        writeln!(c, "#define sub_0 {name}\n{}\n#undef sub_0", function["code"].as_str().unwrap()).unwrap();
        let early = if left == 0x36 { 1 } else { 0x100 };
        let reader_table = if reader { 0xff00 } else { 0 };
        writeln!(checks,
            "if ({name}(0x6666,{admit},0,0x888,0x888,0) != {written} || \
                 {name}(0x6666,{late},0,0x888,0x888,0) != {held}) return {case};",
            admit = early | reader_table, late = (early << 7) | reader_table, case = r + 1,
        ).unwrap();
    }
    writeln!(c, "int main(void) {{\n{checks}\nreturn 0;\n}}").unwrap();
    fs::write(&source, &c).unwrap();
    run_emitted_c(&source, &executable, &c);
}

#[test]
fn carmel_sequential_ff2_store_separates_pointer_and_writeback() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let scratch = tempfile::tempdir().unwrap();
    let binary = scratch.path().join("sequential-ff2.bin");
    let source = scratch.path().join("sequential-ff2.c");
    let executable = scratch.path().join("sequential-ff2");
    const NOP: [u8; 6] = [0x2b, 0xec, 0, 0x2b, 0xec, 0];
    let mut code = vec![[0x21, 0xac, 0x20, 0x2b, 0xec, 0]];
    code.extend([NOP; 5]);
    code.push([0xbd, 0xac, 0x0e, 0x31, 0x43, 0xbd]);
    code.extend([NOP, [0x2b, 0xd8, 0, 0x2b, 0xec, 0], NOP]);
    let mut c = String::from("#include <stdbool.h>\n#include <stdint.h>\nstatic volatile unsigned short cell;\n");
    let mut checks = String::new();
    const RM7: [u8; 6] = [0x30, 0x8e, 0x0e, 0x2b, 0xec, 0];
    const RM255: [u8; 6] = [0x30, 0x8f, 0xfe, 0x2b, 0xec, 0];
    const BANK: [u8; 6] = [0x2b, 0xa0, 0, 0x2b, 0xec, 0];
    for (r, (ptr, step, cfg, modulo, align, advanced, writers)) in [
        (0x8048, 3, 0, 0, 0, 0x804b, &[][..]),
        (0x4005, 3, 0x3000, 0x1007, 0, 0x3000, &[][..]),
        (0x8010, 0x8000, 0x2000, 0x3f, 0, 0x50, &[][..]),
        (0x8047, 0xffff, 0x2000, 7, 0, 0x804e, &[][..]),
        (0x8047, 0xffff, 0x2000, 7, 0xfff8, 0x8046, &[][..]),
        (0x8047, 0xffff, 0x2000, 0, 0, 0x8046, &[RM7][..]),
        (0x8047, 0xffff, 0x2000, 0, 0, 0x8046, &[RM255, [0xdf, 0, 0, 0x9b, 0x6f, 0xdd]][..]),
        (0x8047, 0xffff, 0x2000, 0x1234, 0, 0x8046, &[[0x77, 0, 0, 0xc3, 0xb2, 0x1a]][..]),
        (0x8047, 0xffff, 0x2000, 0, 0, 0x8046, &[RM7, BANK, RM255, BANK][..]),
    ].into_iter().enumerate() {
        let mut program = Vec::new();
        for &writer in writers {
            program.push(writer);
            program.extend([NOP; 6]);
        }
        program.extend_from_slice(&code);
        fs::write(&binary, program.as_flattened()).unwrap();
        let specs = scratch.path().join(format!("specs-{r}"));
        tracked_carmel_entry_specs(
            &specs,
            &[
                ("R6", ptr), ("RN6", step), ("RM6", modulo), ("RL6", 0), ("RM6_ALIGN", align),
                ("PACFG", cfg), ("A0L", 0xffff), ("A0H", 1),
                ("FF1", 4), ("X2", 3), ("RM6S", 0), ("RL6S", 0),
            ],
            0,
            0,
        );
        let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
        kuna.env("KUNA_SPECS", &specs).arg("decompile").arg(&binary).args([
            "0", "--addr", "--raw-image", "--target", "Carmel:BE:24:default:default", "--base", "0",
            "--json", "--assert-strict", "--assert", "return %R6 unsigned short",
            "--assert", &format!("data [data,{ptr:#x}] volatile unsigned short cell"),
        ]);
        for (i, reg) in ["CECFG1", "CECFG2", "CETBL", "ECFG", "FF2", "EST"].into_iter().enumerate() {
            kuna.args(["--assert", &format!("param {i} %{reg} unsigned short in_{i}")]);
        }
        let output = common::process::required_output(&mut kuna);
        let json: Value = serde_json::from_slice(&output.stdout).unwrap();
        let function = &json["functions"][0];
        assert!(function["error"].is_null(), "{function}");
        writeln!(c, "#define sub_0 store_{r}\n{}\n#undef sub_0", function["code"].as_str().unwrap()).unwrap();
        for (arm, (ce, table, ecfg, new_ptr, data)) in [
            (0x0888, 0x80, 0, advanced, 0x2468),
            (0x0888, 1, 0, ptr, 0x2468),
            (0x0888, 0x81, 0x20, advanced, 0xfb3c),
            (0x1000, 0, 0, advanced, 0x2468),
        ].into_iter().enumerate() {
            writeln!(checks,
                "cell=0x2468; if (store_{r}({ce},0,{table},{ecfg},0xf678,0) != {new_ptr} || cell != {data}) return {};",
                r * 4 + arm + 1, table = table | 0xff00,
            ).unwrap();
        }
    }
    writeln!(c, "int main(void) {{\n{checks}\nreturn 0;\n}}").unwrap();
    fs::write(&source, &c).unwrap();
    run_emitted_c(&source, &executable, &c);
}

#[test]
fn carmel_sequential_subc_flags_only_gate_the_late_intbase_store() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let scratch = tempfile::tempdir().unwrap();
    let binary = scratch.path().join("sequential-subc.bin");
    let source = scratch.path().join("sequential-subc.c");
    let executable = scratch.path().join("sequential-subc");
    const NOP: [u8; 6] = [0x2b, 0xec, 0, 0x2b, 0xec, 0];
    let mut code = vec![NOP; 6];
    code.push([0x21, 0xac, 0x20, 0x2b, 0xec, 0]);
    code.extend([NOP; 5]);
    code.extend([[0x8e, 0x7c, 0xe3, 0x31, 0x5f, 0x0d], NOP, [0x2b, 0xd8, 0, 0x2b, 0xec, 0], NOP]);
    fs::write(&binary, code.as_flattened()).unwrap();
    let mut c = String::from("#include <stdbool.h>\n#include <stdint.h>\n\
        static volatile unsigned short lhs_cell, rhs_cell;\n");
    let mut checks = String::new();
    for (row, (r7, r0, a1, cecfg, table, ecfg, new_r7, new_r0, flags, left, right)) in [
        (0xc048, 0x4040, 0xc048, 0x0888, 0x0080, 0, 0xc04c, 0x403f, 0x1009, 0, 0xd135),
        (0xc048, 0x4040, 0x1234, 0x0888, 0x0080, 0, 0xc04c, 0x403f, 0, 0x51ec, 0x2143),
        (0xc048, 0x4040, 0x1234, 0x1000, 0, 0, 0xc04c, 0x403f, 9, 0x2143, 0x2143),
        (0xffff, 0x4040, 0x7fff, 0, 0xffff, 0x20, 3, 0x403f, 0x3024, 0x4000, 0xe89a),
        (0xc048, 0xc048, 0x1234, 0, 0xffff, 0, 0xc04c, 0xc047, 0x3000, 0xd135, 0x2143),
    ].into_iter().enumerate() {
        let specs = scratch.path().join(format!("subc-specs-{row}"));
        tracked_carmel_entry_specs(&specs, &[
            ("R7", r7), ("R0", r0), ("RN7", 4), ("PACFG", 0), ("EST", 0), ("A0L", 0xffff),
            ("A1L", a1), ("CECFG2", 0), ("INTBASE", 0xd135),
        ], 0, 0);
        for (reg, expected) in [("R7", new_r7), ("R0", new_r0), ("EST", flags)] {
            let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
            kuna.env("KUNA_SPECS", &specs).arg("decompile").arg(&binary).args([
                "0", "--addr", "--raw-image", "--target", "Carmel:BE:24:default:default", "--base", "0",
                "--json", "--assert-strict", "--assert", &format!("return %{reg} unsigned short"),
                "--assert", "param 0 %CECFG1 unsigned short ce", "--assert", "param 1 %CETBL unsigned short table",
                "--assert", "param 2 %ECFG unsigned short ecfg",
                "--assert", &format!("data [data,{r7:#x}] volatile unsigned short lhs_cell"),
            ]);
            if r7 != r0 {
                kuna.args(["--assert", &format!("data [data,{r0:#x}] volatile unsigned short rhs_cell")]);
            }
            let output = common::process::required_output(&mut kuna);
            let json: Value = serde_json::from_slice(&output.stdout).unwrap();
            let function = &json["functions"][0];
            assert!(function["error"].is_null(), "{function}");
            let name = format!("subc_{row}_{reg}");
            writeln!(c, "#define sub_0 {name}\n{}\n#undef sub_0", function["code"].as_str().unwrap()).unwrap();
            writeln!(checks, "lhs_cell = rhs_cell = 0x2143; \
                if ({name}({cecfg}, {table}, {ecfg}) != {expected} || lhs_cell != {left} || rhs_cell != {right}) \
                    return {};", row + 1).unwrap();
        }
    }
    writeln!(c, "int main(void) {{\n{checks}\nreturn 0;\n}}").unwrap();
    fs::write(&source, &c).unwrap();
    run_emitted_c(&source, &executable, &c);
}

#[test]
fn carmel_parallel_short_stores_reserve_ports_before_result_veto() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    const NOP: [u8; 6] = [0x2b, 0xec, 0, 0x2b, 0xec, 0];
    const INC: [u8; 6] = [0x21, 0xac, 0x62, 0x2b, 0xec, 0];
    const MOVE_PAIR: [u8; 6] = [0x31, 0x50, 0x11, 0x31, 0x50, 0x33];
    const RET: [u8; 6] = [0x2b, 0xd8, 0, 0x2b, 0xec, 0];
    let scratch = tempfile::tempdir().unwrap();
    let binary = scratch.path().join("paired-store.bin");
    let source = scratch.path().join("paired-store.c");
    let executable = scratch.path().join("paired-store");
    let mut c = String::from("#include <stdbool.h>\n#include <stdint.h>\n#include <setjmp.h>\n\
        static jmp_buf conflict;\nstatic void carmel_data_write_conflict(void) { longjmp(conflict, 1); }\n\
        static volatile unsigned short mem_left, mem_right;\n");
    let mut checks = String::new();
    for (i, (left, right, gap, cfg1, cfg2, table, ecfg, expected)) in [
        (0x40, 0x40, 0, 0x1000, 0x1000, 0, 0, Some((0xa12b, 0x783c, 0))),
        (0x40, 0x40, 0, 0x1000, 0x1000, 0xffff, 0, None),
        (0x4040, 0x4042, 0, 0x1000, 0x1000, 0, 0, None),
        (0x4040, 0x4041, 0, 0x1000, 0x1000, 0xffff, 32, Some((0xc091, 0x32a1, 0x3000))),
        (0x4040, 0x8040, 0, 0x1000, 0x1000, 0xff00, 0, Some((0xa12b, 0x6543, 0x2000))),
        (0x4040, 0x8040, 3, 0x1888, 0x0888, 0x8080, 0, Some((0x8123, 0x783c, 0x3009))),
        (0x4040, 0x8040, 5, 0x0888, 0x0888, 0x8080, 0, Some((0x8123, 0x6543, 0x3009))),
    ].into_iter().enumerate() {
        let mut code = vec![NOP; 6];
        if gap != 0 {
            code.push(INC);
            code.extend(std::iter::repeat_n(NOP, gap));
        }
        code.extend([MOVE_PAIR, NOP, NOP, NOP, NOP, NOP, RET, NOP]);
        fs::write(&binary, code.as_flattened()).unwrap();
        let specs = scratch.path().join(format!("paired-store-specs-{i}"));
        tracked_carmel_entry_specs(&specs, &[
            ("R0", left), ("R1", right), ("EST", 0),
            ("A0H", 0x8123), ("A1H", 0x6543), ("A1L", 0xffff),
            ("CECFG1", cfg1), ("CECFG2", cfg2), ("CETBL", table), ("ECFG", ecfg),
        ], 0, 0);
        let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
        kuna.env("KUNA_SPECS", &specs).arg("decompile").arg(&binary).args([
            "0", "--addr", "--raw-image", "--target", "Carmel:BE:24:default:default", "--base", "0",
            "--json", "--assert-strict", "--assert", "return %EST unsigned short",
            "--assert", &format!("data [data,{left:#x}] volatile unsigned short mem_left"),
        ]);
        if right != left {
            kuna.args(["--assert", &format!("data [data,{right:#x}] volatile unsigned short mem_right")]);
        }
        let output = common::process::required_output(&mut kuna);
        let json: Value = serde_json::from_slice(&output.stdout).unwrap();
        let f = &json["functions"][0];
        assert!(f["error"].is_null(), "{f}");
        let name = format!("paired_store_{i}");
        writeln!(c, "#define sub_0 {name}\n{}\n#undef sub_0", f["code"].as_str().unwrap()).unwrap();
        writeln!(checks, "mem_left = 0xa12b; mem_right = 0x783c;").unwrap();
        if let Some((lhs, rhs, est)) = expected {
            writeln!(checks, "if ({name}() != {est} || mem_left != {lhs} || mem_right != {rhs}) return {};", i + 1).unwrap();
        } else {
            writeln!(checks, "if (setjmp(conflict) == 0) {{ {name}(); return {}; }}\n\
                if (mem_left != 0xa12b || mem_right != 0x783c) return {};", i + 1, i + 1).unwrap();
        }
    }
    writeln!(c, "int main(void) {{\n{checks}\nreturn 0;\n}}").unwrap();
    fs::write(&source, &c).unwrap();
    run_emitted_c(&source, &executable, &c);
}

#[test]
fn carmel_parallel_long_short_stores_commit_after_shared_port_arbitration() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    const NOP: [u8; 6] = [0x2b, 0xec, 0, 0x2b, 0xec, 0];
    const PAIR: [u8; 6] = [0x48, 0x29, 0x1d, 0x1c, 0x2f, 0x40];
    const RET: [u8; 6] = [0x2b, 0xd8, 0, 0x2b, 0xec, 0];
    let scratch = tempfile::tempdir().unwrap();
    let binary = scratch.path().join("mixed-store.bin");
    let source = scratch.path().join("mixed-store.c");
    let executable = scratch.path().join("mixed-store");
    let mut c = String::from("#include <stdbool.h>\n#include <stdint.h>\n#include <setjmp.h>\n\
        static jmp_buf conflict;\nstatic void carmel_data_write_conflict(void) { longjmp(conflict, 1); }\n\
        static volatile unsigned short mem_0, mem_1, mem_2, mem_3;\n");
    let mut checks = String::new();
    for (i, (long, word, rn6, x1, a5h, ecfg, cfg, table, expected)) in [
        (0x4040u16, 0x8048, 3, 0xba98, 0xfffe, 0, 0x1000, 0xffff, Some((0x4042, 0x3000, [0x2fc8, 2, 3]))),
        (0x7fff, 0x8001, 0xffff, 0x8000, 0xfffe, 32, 0x1000, 0xffff, Some((0x8001, 0x3010, [0xc000, 0xffff, 1]))),
        (0xffff, 0, 3, 0xba98, 2, 0, 0x1000, 0xffff, Some((1, 0x3000, [0x2fc8, 3, 0x9a3d]))),
        (0x4040, 0x8048, 0x8000, 0xffff, 2, 1, 0x1000, 0xffff, Some((0x4042, 0x3034, [0, 1, 3]))),
        (0x4040, 0x8048, 3, 0xba98, 0x8000, 4, 0x1000, 0, Some((0x4042, 0x0900, [0xa12b, 0x783c, 0x7fff]))),
        (0x4040, 0x4048, 3, 0xba98, 2, 0, 0, 0, Some((0x4040, 0, [0xa12b, 0x783c, 3]))),
        (0x7fff, 0x8000, 3, 0xba98, 2, 0, 0x1000, 0, None),
    ].into_iter().enumerate() {
        let mut code = vec![NOP; 6];
        code.extend([PAIR, NOP, NOP, NOP, NOP, NOP, RET, NOP]);
        fs::write(&binary, code.as_flattened()).unwrap();
        let specs = scratch.path().join(format!("mixed-store-specs-{i}"));
        tracked_carmel_entry_specs(&specs, &[
            ("R0", 0x40), ("R1", long), ("R5", word), ("RN1", 2), ("RN6", rn6),
            ("PACFG", 0), ("ECFG", ecfg), ("EST", 0), ("FFTV", 0),
            ("A5H", a5h), ("X1", x1), ("CECFG1", cfg), ("CECFG2", 0), ("CETBL", table),
        ], 0, 0);
        for (reg, result) in [("R1", expected.map(|v| v.0)), ("EST", expected.map(|v| v.1))] {
            let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
            kuna.env("KUNA_SPECS", &specs).arg("decompile").arg(&binary).args([
                "0", "--addr", "--raw-image", "--target", "Carmel:BE:24:default:default", "--base", "0",
                "--json", "--assert-strict", "--assert", &format!("return %{reg} unsigned short"),
            ]);
            let cells = [long, long.wrapping_add(1), word, 0x40];
            for (j, address) in cells.into_iter().enumerate() {
                if !cells[..j].contains(&address) {
                    kuna.args(["--assert", &format!("data [data,{address:#x}] volatile unsigned short mem_{j}")]);
                }
            }
            let output = common::process::required_output(&mut kuna);
            let json: Value = serde_json::from_slice(&output.stdout).unwrap();
            let f = &json["functions"][0];
            assert!(f["error"].is_null(), "{f}");
            let name = format!("mixed_store_{i}_{reg}");
            writeln!(c, "#define sub_0 {name}\n{}\n#undef sub_0", f["code"].as_str().unwrap()).unwrap();
            writeln!(checks, "mem_0 = 0xa12b; mem_1 = 0x783c; mem_2 = 0x9a3d; mem_3 = 1;").unwrap();
            if let (Some(result), Some((_, _, [lo, hi, right]))) = (result, expected) {
                writeln!(checks, "if ({name}() != {result} || mem_0 != {lo} || mem_1 != {hi} || \
                    mem_2 != {right} || mem_3 != 1) return {};", i + 1).unwrap();
            } else {
                writeln!(checks, "if (setjmp(conflict) == 0) {{ {name}(); return {}; }}\n\
                    if (mem_0 != 0xa12b || mem_1 != 0x783c || mem_2 != 0x9a3d || mem_3 != 1) return {};", i + 1, i + 1).unwrap();
            }
        }
    }
    writeln!(c, "int main(void) {{\n{checks}\nreturn 0;\n}}").unwrap();
    fs::write(&source, &c).unwrap();
    run_emitted_c(&source, &executable, &c);
}

#[test]
fn carmel_parallel_long_stores_preserve_latch_and_full_width_extrema() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    const NOP: [u8; 6] = [0x2b, 0xec, 0, 0x2b, 0xec, 0];
    const RET: [u8; 6] = [0x2b, 0xd8, 0, 0x2b, 0xec, 0];
    let scratch = tempfile::tempdir().unwrap();
    let binary = scratch.path().join("long-stores.bin");
    let source = scratch.path().join("long-stores.c");
    let executable = scratch.path().join("long-stores");
    type Regs = &'static [(&'static str, u16)];
    type Words = &'static [(u16, u16, u16)];
    let rows: [([u8; 6], Regs, Regs, Words); 2] = [
        ([0x44, 0x85, 0x0e, 0x44, 0x01, 0x88],
         &[("R0", 0xffff), ("R1", 0xffff), ("R4", 0x48), ("R7", 0xc048), ("RN4", 1),
           ("A0H", 0x8123), ("X1", 0x6a5b), ("ECFG", 0)],
         &[("R4", 0x49), ("X1", 0x5df9), ("EST", 0x3400)],
         &[(0xffff, 0x9a3d, 0x4ac9), (0, 0x24ab, 0xc001), (0x48, 0x5df9, 0x5df9)]),
        ([0x26, 0x62, 0x43, 0x0c, 0x77, 0x61],
         &[("A1L", 2), ("A1H", 0x3401), ("GB1", 0xff), ("A4L", 1), ("A4H", 0x3401),
           ("GB4", 0), ("A5H", 0xd1dd), ("BTR1", 0x8543), ("ECFG", 32),
           ("R0", 0x40), ("R4", 0x48), ("R6", 0x8048), ("RN6", 3)],
         &[("BTR1", 0x0a86), ("R0", 0x41), ("R6", 0x804b), ("EST", 0x3000)],
         &[(0x48, 0xa12b, 0x8001), (0x49, 0x783c, 0x9a00), (0x8048, 0x9a3d, 0x2729),
           (0x40, 0x2030, 0x2030)]),
    ];
    let mut c = String::from("#include <stdbool.h>\n#include <stdint.h>\n\
        static volatile unsigned short mem_0, mem_1, mem_2, mem_3;\n");
    let mut checks = String::new();
    for (i, (packet, seeds, outputs, words)) in rows.into_iter().enumerate() {
        let mut code = vec![NOP; 6];
        code.extend([packet, NOP, NOP, NOP, NOP, NOP, RET, NOP]);
        fs::write(&binary, code.as_flattened()).unwrap();
        let specs = scratch.path().join(format!("long-store-specs-{i}"));
        let mut tracked = vec![("EST", 0), ("PACFG", 0), ("FFTV", 0),
                               ("CECFG1", 0x1000), ("CECFG2", 0x1000), ("CETBL", 0xffff)];
        tracked.extend_from_slice(seeds);
        tracked_carmel_entry_specs(&specs, &tracked, 0, 0);
        for &(reg, result) in outputs {
            let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
            kuna.env("KUNA_SPECS", &specs).arg("decompile").arg(&binary).args([
                "0", "--addr", "--raw-image", "--target", "Carmel:BE:24:default:default", "--base", "0",
                "--json", "--assert-strict", "--assert", &format!("return %{reg} unsigned short"),
            ]);
            for (j, (address, seed, _)) in words.iter().enumerate() {
                kuna.args(["--assert", &format!("data [data,{address:#x}] volatile unsigned short mem_{j}")]);
                writeln!(checks, "mem_{j} = {seed};").unwrap();
            }
            let output = common::process::required_output(&mut kuna);
            let json: Value = serde_json::from_slice(&output.stdout).unwrap();
            let f = &json["functions"][0];
            assert!(f["error"].is_null(), "{f}");
            let name = format!("long_store_{i}_{reg}");
            writeln!(c, "#define sub_0 {name}\n{}\n#undef sub_0", f["code"].as_str().unwrap()).unwrap();
            writeln!(checks, "if ({name}() != {result}) return {};", 1 + i).unwrap();
            for (j, &(_, _, word)) in words.iter().enumerate() {
                writeln!(checks, "if (mem_{j} != {word}) return {};", 3 + i).unwrap();
            }
        }
    }
    writeln!(c, "int main(void) {{\n{checks}\nreturn 0;\n}}").unwrap();
    fs::write(&source, &c).unwrap();
    run_emitted_c(&source, &executable, &c);
}

#[test]
fn carmel_compact_shifted_mac_uses_guard_and_separates_qualification() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let scratch = tempfile::tempdir().unwrap();
    let binary = scratch.path().join("shifted-mac.bin");
    let source = scratch.path().join("shifted-mac.c");
    let executable = scratch.path().join("shifted-mac");
    const NOP: [u8; 6] = [0x2b, 0xec, 0, 0x2b, 0xec, 0];
    const PAIR: [u8; 6] = [0xd3, 0x38, 0xd3, 0x4a, 0xc0, 0x71];
    let mut c = String::from("#include <stdbool.h>\n#include <stdint.h>\nstatic volatile unsigned short word;\n");
    let mut checks = String::new();
    for (row, (guard, ff2, cfg, table, prefix, expected)) in [
        (0, 0x1000, 0, 0xffff, None, [0xfffe, 0x0dac, 0xc048]),
        (0xff, 0x1000, 0, 0xffff, None, [0xfffd, 0x0dac, 0xc048]),
        (0xff, 0x1000, 0, 0xffff, Some([0x12, 0xbc, 0, 0x2b, 0xec, 0]), [0xfffe, 0x0dac, 0xc048]),
        (0, 0x1000, 0x1000, 0, None, [0x1000, 0x1234, 0xc048]),
        (0, 0x1000, 0, 0, None, [0x1000, 0x1234, 0x96a5]),
        (0, 0, 0x0888, 0x80, Some([0x21, 0xac, 0x62, 0x2b, 0xec, 0]), [0xfffe, 0x1234, 0xc048]),
    ].into_iter().enumerate() {
        let mut code = vec![NOP; 6];
        if let Some(prefix) = prefix {
            code.push(prefix);
            code.extend([NOP; 6]);
        }
        code.extend([PAIR, [0x31, 0xaa, 0x4e, 0x2b, 0xec, 0], NOP, [0x2b, 0xd8, 0, 0x2b, 0xec, 0], NOP]);
        fs::write(&binary, code.as_flattened()).unwrap();
        let specs = scratch.path().join(format!("shifted-mac-specs-{row}"));
        tracked_carmel_entry_specs(&specs, &[
            ("FF1", 1), ("FF2", ff2), ("FFG", guard), ("A0L", 0x1234), ("A0H", 0), ("GB0", 0),
            ("A1L", 0xffff), ("X1", 0x96a5), ("R0", 0x40), ("R3", 0xc040), ("R7", 0xc048),
            ("RN1", 0xfffe), ("ECFG", 0), ("EST", 0), ("CECFG1", cfg), ("CECFG2", 0), ("CETBL", table),
        ], 0, 0);
        for (reg, value) in ["A2L", "A0L", "X1"].into_iter().zip(expected) {
            let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
            kuna.env("KUNA_SPECS", &specs).arg("decompile").arg(&binary).args([
                "0", "--addr", "--raw-image", "--target", "Carmel:BE:24:default:default", "--base", "0",
                "--json", "--assert-strict", "--assert", &format!("return %{reg} unsigned short"),
                "--assert", "data [data,0xc040] volatile unsigned short word",
            ]);
            let output = common::process::required_output(&mut kuna);
            let json: Value = serde_json::from_slice(&output.stdout).unwrap();
            let function = &json["functions"][0];
            assert!(function["error"].is_null(), "{function}");
            let name = format!("shifted_mac_{row}_{reg}");
            writeln!(c, "#define sub_0 {name}\n{}\n#undef sub_0", function["code"].as_str().unwrap()).unwrap();
            writeln!(checks, "word = 0xeffe; if ({name}() != {value}) return {};", row + 1).unwrap();
        }
    }
    writeln!(c, "int main(void) {{\n{checks}\nreturn 0;\n}}").unwrap();
    fs::write(&source, &c).unwrap();
    run_emitted_c(&source, &executable, &c);
}

#[test]
fn carmel_indexed_shifted_mac_feeds_carry_to_the_serial_data_store() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let scratch = tempfile::tempdir().unwrap();
    let binary = scratch.path().join("indexed-mac.bin");
    let source = scratch.path().join("indexed-mac.c");
    let executable = scratch.path().join("indexed-mac");
    const NOP: [u8; 6] = [0x2b, 0xec, 0, 0x2b, 0xec, 0];
    let mut c = String::from("#include <stdbool.h>\n#include <stdint.h>\n\
        static volatile unsigned short indexed, post, other;\n\
        #define CONCAT22(h,l) (((uint32_t)(uint16_t)(h) << 16) | (uint16_t)(l))\n");
    let mut calls = String::new();
    for (i, (writer, pacfg, r3, ff2, rhs, a2h, x2, ecfg, new_r2, new_r3, low, high, stored)) in [
        (false, 0, 0x0049, 1, 0xffff, 0x13, 0x27, 0, 0x8043, 0x004d, 0, 0, 0x003b),
        (false, 0x80, 0xc041, 0, 0xffff, 0x13, 0x27, 0, 0x8043, 0xc03d, 0xffff, 0xffff, 0x003a),
        (false, 0x20, 0x8042, 0, 1, 0x7fff, 2, 0x24, 0x803b, 0x8046, 1, 0, 0x3fff),
        (true, 0x20, 0x8042, 0, 1, 0x7fff, 2, 0x24, 0x8043, 0x8046, 1, 0, 0x3fff),
        (true, 0x80, 0xc041, 0, 1, 0x7fff, 2, 0x28, 0x8043, 0xc045, 1, 0, 0x4000),
    ].into_iter().enumerate() {
        let mut code = vec![NOP; 6];
        if writer {
            code.push([0x30, 0x86, 0x0e, 0x2b, 0xec, 0]);
            code.extend([NOP; 6]);
        }
        code.extend([
            [0xd3, 0xad, 0x24, 0x16, 0x6f, 0x5e], [0x31, 0xaa, 0x4e, 0x2b, 0xec, 0],
            NOP, [0x2b, 0xd8, 0, 0x2b, 0xec, 0], NOP,
        ]);
        fs::write(&binary, code.as_flattened()).unwrap();
        let specs = scratch.path().join(format!("indexed-mac-specs-{i}"));
        tracked_carmel_entry_specs(&specs, &[
            ("R2", 0x8042), ("R3", r3), ("R4", 0x48), ("RN3", 4), ("RN4", 1),
            ("PACFG", pacfg), ("RM2", 7), ("RM2_ALIGN", 0), ("RL2", 0), ("FFG", 0), ("FF2", ff2),
            ("A2H", a2h), ("X2", x2), ("ECFG", ecfg), ("EST", 1),
            ("CECFG1", 0), ("CECFG2", 0), ("CETBL", 0xffff),
        ], 0, 0);
        for (reg, expected) in [("FF1", low), ("A2L", high), ("R2", new_r2), ("R3", new_r3)] {
            let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
            kuna.env("KUNA_SPECS", &specs).arg("decompile").arg(&binary).args([
                "0", "--addr", "--raw-image", "--target", "Carmel:BE:24:default:default", "--base", "0",
                "--json", "--assert-strict", "--assert", &format!("return %{reg} unsigned short"),
                "--assert", "data [data,0x0049] volatile unsigned short indexed",
                "--assert", "data [data,0x8042] volatile unsigned short post",
                "--assert", "data [data,0xc041] volatile unsigned short other",
            ]);
            let output = common::process::required_output(&mut kuna);
            let json: Value = serde_json::from_slice(&output.stdout).unwrap();
            let f = &json["functions"][0];
            assert!(f["error"].is_null(), "{f}");
            let name = format!("indexed_mac_{i}_{reg}");
            writeln!(c, "#define sub_0 {name}\n{}\n#undef sub_0", f["code"].as_str().unwrap()).unwrap();
            let [first, second, third] = [(0x0049, 1), (0x8042, rhs), (0xc041, 0xbeef)]
                .map(|(addr, initial)| if addr == r3 { stored } else { initial });
            writeln!(calls, "indexed = 1; post = {rhs}; other = 0xbeef;\n\
                if ({name}() != {expected} || indexed != {first} || post != {second} || other != {third}) return {};",
                i + 1).unwrap();
        }
    }
    writeln!(c, "int main(void) {{\n{calls}\nreturn 0;\n}}").unwrap();
    fs::write(&source, &c).unwrap();
    run_emitted_c(&source, &executable, &c);
}

#[test]
fn carmel_omits_unallocated_right_alu_without_inventing_a_clock() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let scratch = tempfile::tempdir().unwrap();
    let binary = scratch.path().join("omitted-slot.bin");
    let source = scratch.path().join("omitted-slot.c");
    let executable = scratch.path().join("omitted-slot");
    let specs = scratch.path().join("specs");
    tracked_carmel_entry_specs(&specs, &[
        ("FFG", 0), ("FF2", 0x1000), ("X1", 1), ("R7", 0xc048), ("EST", 0), ("ECFG", 0),
        ("CECFG1", 0), ("CECFG2", 0), ("CETBL", 0xffff),
    ], 0, 0);
    const NOP: [u8; 6] = [0x2b, 0xec, 0, 0x2b, 0xec, 0];
    const RET: [u8; 6] = [0x2b, 0xd8, 0, 0x2b, 0xec, 0];
    let decompile = |code: &[[u8; 6]]| {
        fs::write(&binary, code.as_flattened()).unwrap();
        let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
        kuna.env("KUNA_SPECS", &specs).arg("decompile").arg(&binary).args([
            "0", "--addr", "--raw-image", "--target", "Carmel:BE:24:default:default", "--base", "0",
            "--json", "--assert-strict", "--assert", "return %A2L unsigned short",
            "--assert", "data [data,0xc048] volatile unsigned short operand",
        ]);
        common::process::output_with_timeout(&mut kuna, Duration::from_secs(15), Duration::from_millis(10)).unwrap()
    };
    let mut c = String::from("#include <stdbool.h>\n#include <stdint.h>\nstatic volatile unsigned short operand = 0x8000;\n");
    let mut checks = String::new();
    for (i, opcode) in [0x248000u32, 0x25d068, 0x26b068, 0x26ffff].into_iter().enumerate() {
        let [_, a, b, d] = opcode.to_be_bytes();
        let output = decompile(&[[0xb3, 0x31, 0xe8, a, b, d], [0x31, 0xaa, 0x4e, 0x2b, 0xec, 0], NOP, RET, NOP]);
        assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
        let report: Value = serde_json::from_slice(&output.stdout).unwrap();
        let f = &report["functions"][0];
        assert!(f["error"].is_null(), "{f}");
        writeln!(c, "#define sub_0 omitted_{i}\n{}\n#undef sub_0", f["code"].as_str().unwrap()).unwrap();
        writeln!(checks, "if (omitted_{i}() != 0xffff) return {};", i + 1).unwrap();
    }
    writeln!(c, "int main(void) {{\n{checks}\nreturn 0;\n}}").unwrap();
    fs::write(&source, &c).unwrap();
    run_emitted_c(&source, &executable, &c);

    let mut code = vec![NOP; 3];
    code.extend([
        [0xab, 0xec, 0, 0x26, 0xb0, 0x68], [0xd3, 0x38, 0xd3, 0x4a, 0xc0, 0x71], NOP, RET, NOP,
    ]);
    let failed = decompile(&code);
    let error = String::from_utf8_lossy(&failed.stderr);
    assert!(!failed.status.success() && error.contains("4 of 5 clocks"), "{error}");
    let mut branch = vec![NOP; 16];
    branch[0] = [0xab, 0xec, 0, 0x26, 0xb0, 0x68];
    branch[1] = [0xc0, 0x00, 0x18, 0xcf, 0x80, 0x00];
    branch[14] = RET;
    let failed = decompile(&branch);
    let error = String::from_utf8_lossy(&failed.stderr);
    assert!(!failed.status.success() && error.contains("Unproved Carmel control flow after omitted sequential slot"), "{error}");
}

#[test]
fn carmel_reset_clears_control_only_after_a_proved_retirement_window() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let scratch = tempfile::tempdir().unwrap();
    let binary = scratch.path().join("reset.bin");
    let source = scratch.path().join("reset.c");
    let executable = scratch.path().join("reset");
    let nop = [0x2b, 0xec, 0, 0x2b, 0xec, 0];
    let sequential = [0xab, 0xec, 0, 0x2b, 0xec, 0];
    let warm = [sequential; 2];
    let reset = [0xb4, 0xd1, 0x1d, 0xca, 0x74, 0x52];
    let decode = |entry: &str, prefix: &[[u8; 6]], after: &[[u8; 6]], storage: &str| -> Value {
        let mut bytes = prefix.to_vec();
        bytes.push(reset);
        bytes.extend_from_slice(after);
        bytes.extend([nop; 6]);
        bytes.push([0x2b, 0xd8, 0, 0x2b, 0xec, 0]);
        bytes.extend([nop; 2]);
        fs::write(&binary, bytes.as_flattened()).unwrap();
        let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
        kuna.arg("decompile").arg(&binary).args([
            entry, "--addr", "--raw-image", "--base", entry, "--target", "Carmel:BE:24:default:default",
            "--json", "--assert-strict", "--assert", &format!("return %{storage} unsigned short"),
        ]);
        for (n, input) in ["EST", "ECFG", "PACFG", "INTBASE", "PSTATUS", "CECFG1", "CECFG2", "CETBL", "A0L"]
            .into_iter().enumerate()
        {
            kuna.args(["--assert", &format!("param {n} %{input} unsigned short in_{n}")]);
        }
        let output = kuna.output().unwrap();
        let report: Value = serde_json::from_slice(&output.stdout).unwrap_or_else(|err| {
            panic!("{err}: {}", String::from_utf8_lossy(&output.stderr));
        });
        report["functions"][0].clone()
    };
    let mut generated = String::from("#include <stdbool.h>\n#include <stdint.h>\n");
    let mut calls = String::new();
    let rows = [("EST", 0), ("ECFG", 0x3fc0), ("PACFG", 0), ("INTBASE", 0xe000),
        ("PSTATUS", 0xe010), ("CECFG1", 0xd000), ("CECFG2", 0x5000), ("CETBL", 0), ("A0L", 0x4321)];
    for (i, (storage, expected)) in rows.into_iter().enumerate() {
        let (entry, prefix) = if i & 1 == 0 { ("0", &[][..]) } else { ("0x1000", &warm[..]) };
        let fun = decode(entry, prefix, &[], storage);
        assert!(fun["error"].is_null(), "{entry} {storage}: {fun}");
        let symbol = fun["name"].as_str().unwrap();
        let text = fun["code"].as_str().unwrap();
        let name = format!("reset_{i}");
        writeln!(generated, "#define {symbol} {name}\n{text}\n#undef {symbol}").unwrap();
        writeln!(calls, "if ({name}(0x7fff,0x3fff,0x8765,0xeabc,0xe00f,0xffff,0x4fff,0xffff,0x4321) != {expected:#x}) return {};", i + 1).unwrap();
    }
    let inc = [0x21, 0xac, 0x20];
    let idle = [0x2b, 0xec, 0];
    let left = [inc, idle].as_flattened().try_into().unwrap();
    let right = [idle, inc].as_flattened().try_into().unwrap();
    let consume = [[0x31, 0xaa, 0x20, 0x2b, 0xec, 0]]; // MOVE A1L,A0L
    for (i, (prefix, after, storage, expected)) in [
        (&[left][..], &[][..], "A0L", 0x8000),
        (&[left][..], &[][..], "EST", 0x24),
        (&[right][..], &[][..], "EST", 0x900),
        (&[left][..], &consume[..], "A1L", 0x8000),
        (&[left, nop][..], &[][..], "A0L", 0x7fff),
    ].into_iter().enumerate() {
        let fun = decode("0", prefix, after, storage);
        assert!(fun["error"].is_null(), "{fun}");
        let symbol = fun["name"].as_str().unwrap();
        let text = fun["code"].as_str().unwrap();
        let name = format!("reset_live_{i}");
        writeln!(generated, "#define {symbol} {name}\n{text}\n#undef {symbol}").unwrap();
        writeln!(calls, "if ({name}(0xcfff,0x2c,0x8765,0xeabc,0xe00f,0x1000,0x1000,0xffff,0x7fff) != {expected:#x}) return {};", i + 10).unwrap();
    }
    writeln!(generated, "int main(void) {{\n{calls}\nreturn 0;\n}}").unwrap();
    fs::write(&source, &generated).unwrap();
    run_emitted_c(&source, &executable, &generated);
    for (entry, prefix) in [("0x1000", &[][..]), ("0x1000", &[left][..])] {
        let fun = decode(entry, prefix, &[], "EST");
        assert!(fun["error"].to_string().contains("Unproved Carmel RESET retirement window"), "{fun}");
    }
}

#[test]
fn carmel_reset_cancels_data_results_but_keeps_older_address_updates() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let scratch = tempfile::tempdir().unwrap();
    let binary = scratch.path().join("reset-agu.bin");
    let source = scratch.path().join("reset-agu.c");
    let executable = scratch.path().join("reset-agu");
    let nop = [0x2b, 0xec, 0, 0x2b, 0xec, 0];
    let load = [0x31, 0xdc, 0x92, 0x2b, 0xec, 0];
    let specs = scratch.path().join("specs");
    tracked_carmel_entry_specs(&specs,
        &[("R1", 0x0130), ("R6", 0x8049), ("R7", 0xc04b), ("RN1", 1), ("RN6", 2), ("PACFG", 0)],
        0x1000, 0);
    let mut generated = String::from(
        "#include <stdbool.h>\n#include <stdint.h>\nvolatile unsigned short to, from, pre, post;\n",
    );
    let mut calls = String::new();
    for (i, (storage, after, expected)) in [
        ("R6", false, 0x804b), ("R7", false, 0xc04c), ("R1", false, 0x0131),
        ("A2L", false, 0xaaaa), ("A2L", true, 0x9abc), ("R1", true, 0x0132),
    ].into_iter().enumerate() {
        let mut code = vec![
            [0x31, 0x4b, 0xae, 0x2b, 0xec, 0], load,
            [0xb4, 0xd1, 0x1d, 0xca, 0x74, 0x52],
        ];
        if after {
            code.push(load);
        }
        code.extend([nop; 6]);
        code.extend([[0x2b, 0xd8, 0, 0x2b, 0xec, 0], nop, nop]);
        fs::write(&binary, code.as_flattened()).unwrap();
        let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
        kuna.env("KUNA_SPECS", &specs);
        kuna.arg("decompile").arg(&binary).args([
            "0x1000", "--addr", "--raw-image", "--base", "0x1000", "--target", "Carmel:BE:24:default:default",
            "--json", "--assert-strict", "--assert", &format!("return %{storage} unsigned short"),
            "--assert", "data [data,0x8049] volatile unsigned short to",
            "--assert", "data [data,0xc04b] volatile unsigned short from",
            "--assert", "data [data,0x0130] volatile unsigned short pre",
            "--assert", "data [data,0x0131] volatile unsigned short post",
        ]);
        for (n, input) in ["ECFG", "EST", "CECFG1", "CECFG2", "CETBL", "A2L"].into_iter().enumerate() {
            kuna.args(["--assert", &format!("param {n} %{input} unsigned short in_{n}")]);
        }
        let output = common::process::required_output(&mut kuna);
        let report: Value = serde_json::from_slice(&output.stdout).unwrap();
        let fun = &report["functions"][0];
        assert!(fun["error"].is_null(), "{storage}: {fun}");
        let name = format!("reset_agu_{i}");
        let text = fun["code"].as_str().unwrap();
        writeln!(generated, "#define sub_1000 {name}\n{text}\n#undef sub_1000").unwrap();
        writeln!(calls, "to=0xcd01; from=0xd202; pre=0x5678; post=0x9abc; if ({name}(0x20,0,0,0,0,0xaaaa) != {expected:#x} || to != 0xcd01 || from != 0xd202 || pre != 0x5678 || post != 0x9abc) return {};", i + 1).unwrap();
    }
    writeln!(generated, "int main(void) {{\n{calls}\nreturn 0;\n}}").unwrap();
    fs::write(&source, &generated).unwrap();
    run_emitted_c(&source, &executable, &generated);
}

#[test]
fn carmel_reset_retires_staged_wb_data_with_old_config_and_ex_with_new_config() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let scratch = tempfile::tempdir().unwrap();
    let binary = scratch.path().join("reset-wb.bin");
    let source = scratch.path().join("reset-wb.c");
    let executable = scratch.path().join("reset-wb");
    let specs = scratch.path().join("specs");
    tracked_carmel_entry_specs(&specs, &[
        ("R0", 0xc042), ("R1", 0xc041), ("RN1", 1), ("RM0", 3), ("RL0", 0xc040),
        ("R6", 0xc041), ("R7", 0xc043), ("RN6", 3), ("RM6", 3), ("RL6", 0xc040),
        ("PACFG", 0xffff),
    ], 0x1000, 0);
    let nop = [0x2b, 0xec, 0, 0x2b, 0xec, 0];
    let store = [0x80, 0x56, 0x78, 0xc6, 0x40, 0];
    let left = [0x21, 0xac, 0x20, 0x2b, 0xec, 0];
    let right = [0x2b, 0xec, 0, 0x21, 0xac, 0x20];
    let copy = [0x31, 0x4b, 0xae, 0x2b, 0xec, 0];
    let load = [0x31, 0xdc, 0x92, 0x2b, 0xec, 0];
    let reset = [0xb4, 0xd1, 0x1d, 0xca, 0x74, 0x52];
    let mut generated = String::from("#include <stdbool.h>\n#include <stdint.h>\nvolatile unsigned short cell;\n");
    let mut calls = String::new();
    for (i, (inc, storage, expected)) in [
        (left, "A0L", 0x8000), (left, "A2L", 0x2b3c), (left, "R6", 0xc040),
        (left, "R7", 0xc040), (left, "R1", 0xc043), (left, "EST", 4), (right, "EST", 0x900),
    ].into_iter().enumerate() {
        let mut code = vec![store, inc, copy, load, reset, load];
        code.extend([nop; 6]);
        code.extend([[0x2b, 0xd8, 0, 0x2b, 0xec, 0], nop, nop]);
        fs::write(&binary, code.as_flattened()).unwrap();
        let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
        kuna.env("KUNA_SPECS", &specs).arg("decompile").arg(&binary).args([
            "0x1000", "--addr", "--raw-image", "--base", "0x1000", "--target", "Carmel:BE:24:default:default",
            "--json", "--assert-strict", "--assert", &format!("return %{storage} unsigned short"),
            "--assert", "data [data,0xc042] volatile unsigned short cell",
        ]);
        for (n, input) in ["A0L", "ECFG", "EST", "CECFG1", "CECFG2", "CETBL"].into_iter().enumerate() {
            kuna.args(["--assert", &format!("param {n} %{input} unsigned short in_{n}")]);
        }
        let output = common::process::required_output(&mut kuna);
        let report: Value = serde_json::from_slice(&output.stdout).unwrap();
        let fun = &report["functions"][0];
        assert!(fun["error"].is_null(), "{storage}: {fun}");
        let symbol = fun["name"].as_str().unwrap();
        let text = fun["code"].as_str().unwrap();
        let name = format!("reset_ports_{i}");
        writeln!(generated, "#define {symbol} {name}\n{text}\n#undef {symbol}").unwrap();
        writeln!(calls, "cell=0x8042; if ({name}(0x7fff,0x24,0,0x1000,0x1000,0xffff) != {expected:#x} || cell != 0x2b3c) return {};", i + 1).unwrap();
    }
    writeln!(generated, "int main(void) {{\n{calls}\nreturn 0;\n}}").unwrap();
    fs::write(&source, &generated).unwrap();
    run_emitted_c(&source, &executable, &generated);
}

#[test]
fn carmel_reset_preserves_copy_operand_before_overlapping_writeback() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let scratch = tempfile::tempdir().unwrap();
    let binary = scratch.path().join("reset-alias.bin");
    let source = scratch.path().join("reset-alias.c");
    let executable = scratch.path().join("reset-alias");
    let specs = scratch.path().join("specs");
    tracked_carmel_entry_specs(&specs, &[
        ("R0", 0xc042), ("R1", 0xc043), ("RN1", 1),
        ("R6", 0xc044), ("RN6", 1), ("R7", 0xc042), ("PACFG", 0),
    ], 0x1000, 0);
    let nop = [0x2b, 0xec, 0, 0x2b, 0xec, 0];
    let store = [0x80, 0x56, 0x78, 0xc6, 0x40, 0];
    let inc = [0x21, 0xac, 0x20, 0x2b, 0xec, 0];
    let cmp = [0x20, 0x9a, 0x02, 0x2b, 0xec, 0];
    let copy = [0x31, 0x4b, 0xae, 0x2b, 0xec, 0];
    let load = [0x31, 0xdc, 0x92, 0x2b, 0xec, 0];
    let reset = [0xb4, 0xd1, 0x1d, 0xca, 0x74, 0x52];
    let mut generated = String::from(
        "#include <stdbool.h>\n#include <stdint.h>\nvolatile unsigned short from, to;\n",
    );
    let mut calls = String::new();
    for (i, (first, second, after, a0, storage, expected, data)) in [
        (copy, inc, load, 0x7fff, "A0L", 0x7fff, 0xd242),
        (copy, inc, load, 0x7fff, "A2L", 0xd242, 0xd242),
        (cmp, copy, nop, 0xffff, "EST", 0x10, 0xd244),
    ].into_iter().enumerate() {
        let mut code = vec![store, first, second, load, reset, after];
        code.extend([nop; 6]);
        code.extend([[0x2b, 0xd8, 0, 0x2b, 0xec, 0], nop, nop]);
        fs::write(&binary, code.as_flattened()).unwrap();
        let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
        kuna.env("KUNA_SPECS", &specs).arg("decompile").arg(&binary).args([
            "0x1000", "--addr", "--raw-image", "--base", "0x1000", "--target", "Carmel:BE:24:default:default",
            "--json", "--assert-strict", "--assert", &format!("return %{storage} unsigned short"),
            "--assert", "data [data,0xc042] volatile unsigned short from",
            "--assert", "data [data,0xc044] volatile unsigned short to",
        ]);
        for (n, input) in ["A0L", "A1L", "ECFG", "EST", "CECFG1", "CECFG2", "CETBL"].into_iter().enumerate() {
            kuna.args(["--assert", &format!("param {n} %{input} unsigned short in_{n}")]);
        }
        let output = common::process::required_output(&mut kuna);
        let report: Value = serde_json::from_slice(&output.stdout).unwrap();
        let fun = &report["functions"][0];
        assert!(fun["error"].is_null(), "{storage}: {fun}");
        let symbol = fun["name"].as_str().unwrap();
        let text = fun["code"].as_str().unwrap();
        let name = format!("reset_alias_{i}");
        writeln!(generated, "#define {symbol} {name}\n{text}\n#undef {symbol}").unwrap();
        writeln!(calls, "from=0xd242; to=0xd244; if ({name}({a0:#x},0,0x20,0xcfff,0x1000,0x1000,0xffff) != {expected:#x} || from != 0x2b3c || to != {data:#x}) return {};", i + 1).unwrap();
    }
    writeln!(generated, "int main(void) {{\n{calls}\nreturn 0;\n}}").unwrap();
    fs::write(&source, &generated).unwrap();
    run_emitted_c(&source, &executable, &generated);
}

#[test]
fn carmel_reset_keeps_x1_only_with_live_early_admission() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let scratch = tempfile::tempdir().unwrap();
    let binary = scratch.path().join("reset-x1.bin");
    let source = scratch.path().join("reset-x1.c");
    let executable = scratch.path().join("reset-x1");
    let nop = [0x2b, 0xec, 0, 0x2b, 0xec, 0];
    let inc = [0x21, 0xac, 0x20, 0x2b, 0xec, 0];
    let x1 = [0x10, 0xfa, 2, 0xc2, 0x7a, 0x7a];
    let reset = [0xb4, 0xd1, 0x1d, 0xca, 0x74, 0x52];
    let loadce = [0x8c, 0, 1, 0xcd, 0xe4, 0xb3];
    let mut c = String::from("#include <stdbool.h>\n#include <stdint.h>\n");
    let mut calls = String::new();
    for (i, (lead, gap, younger, storage, cases)) in [
        (None, 0, 2, "X1", &[
            (0xffff, 0x0888, 1, 0x7fff, 0xffff),
            (0xffff, 0x0888, 0x80, 0x7fff, 0x55),
            (0xffff, 0x2000, 0xff, 0x7fff, 0x55),
            (0xffff, 0x1000, 0, 0, 0xffff),
        ][..]),
        (None, 0, 2, "EST", &[(0xffff, 0x1000, 0xff, 0x7fff, 0)][..]),
        (Some(inc), 0, 1, "X1", &[(0xffff, 0x1000, 0xff, 0, 0x55)][..]),
        (Some(inc), 4, 2, "X1", &[
            (0xffff, 0x0888, 1, 0, 0),
            (0xffff, 0x0888, 0x80, 0, 0x55),
        ][..]),
        (Some(inc), 5, 2, "X1", &[
            (0xffff, 0x0888, 1, 0, 0x55),
            (0xffff, 0x0888, 0x80, 0, 0),
        ][..]),
        (Some(loadce), 6, 2, "X1", &[(0x8001, 0x1000, 0xffff, 0, 0x8001)][..]),
    ].into_iter().enumerate() {
        let mut code: Vec<_> = lead.into_iter().collect();
        code.extend(std::iter::repeat_n(nop, gap));
        code.push(x1);
        code.extend(std::iter::repeat_n(nop, younger));
        code.extend([reset, nop, nop, nop, nop, [0x2b, 0xd8, 0, 0x2b, 0xec, 0], nop, nop]);
        fs::write(&binary, code.as_flattened()).unwrap();
        let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
        kuna.arg("decompile").arg(&binary).args([
            "0x1000", "--addr", "--raw-image", "--base", "0x1000", "--target", "Carmel:BE:24:default:default",
            "--json", "--assert-strict", "--assert", &format!("return %{storage} unsigned short"),
        ]);
        for (n, input) in ["A0L", "CECFG1", "CECFG2", "CETBL", "EST", "X1", "ECFG"].into_iter().enumerate() {
            kuna.args(["--assert", &format!("param {n} %{input} unsigned short in_{n}")]);
        }
        let out = common::process::required_output(&mut kuna);
        let report: Value = serde_json::from_slice(&out.stdout).unwrap();
        let fun = &report["functions"][0];
        assert!(fun["error"].is_null(), "{i} {storage}: {fun}");
        let symbol = fun["name"].as_str().unwrap();
        let name = format!("reset_x1_{i}");
        writeln!(c, "#define {symbol} {name}\n{}\n#undef {symbol}", fun["code"].as_str().unwrap()).unwrap();
        for &(a0, cfg, table, est, expected) in cases {
            writeln!(calls, "if ({name}({a0:#x},{cfg:#x},0x1000,{table:#x},{est:#x},0x55,0) != {expected:#x}) return {};", i + 1).unwrap();
        }
    }
    writeln!(c, "int main(void) {{\n{calls}\nreturn 0;\n}}").unwrap();
    fs::write(&source, &c).unwrap();
    run_emitted_c(&source, &executable, &c);
}

#[test]
fn carmel_indexed_word_copy_captures_addresses_before_the_late_store() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let scratch = tempfile::tempdir().unwrap();
    let binary = scratch.path().join("copy.bin");
    let source = scratch.path().join("copy.c");
    let executable = scratch.path().join("copy");
    let nop = [0x2b, 0xec, 0, 0x2b, 0xec, 0];
    let loadce = |bypass, table| [0x80, 0, table, 0xcd, 0xe4, if bypass { 0x40 } else { 0 }];
    let rows = [
        ([0x31, 0x4b, 0xae, 0x2b, 0xec, 0], false, 0, 0, true, true, true),
        ([0x2b, 0xec, 0, 0x31, 0x4b, 0xae], false, 0, 0, true, true, false),
        ([0xb1, 0x4b, 0xae, 0x2b, 0xec, 0], false, 0, 0, true, true, false),
        ([0xab, 0xec, 0, 0x31, 0x4b, 0xae], false, 0, 0, true, true, false),
        ([0x31, 0x4b, 0xaf, 0x2b, 0xec, 0], false, 0xff, 0, true, false, false),
        ([0x2b, 0xec, 0, 0x31, 0x4b, 0xaf], false, 0, 0xff, false, false, false),
        ([0xb1, 0x4b, 0xaf, 0x2b, 0xec, 0], true, 0, 0xff, true, true, false),
    ];
    let mut generated = String::from(
        "#include <stdbool.h>\n#include <stdint.h>\nvolatile unsigned short to, from;\n",
    );
    let mut calls = String::new();
    for (i, (packet, bypass, early, late, address, store, shift)) in rows.into_iter().enumerate() {
        let mut code = vec![
            [0x80, 0x80, 0x49, 0xc6, 0x42, 0x98], // R6
            [0x80, 0xc0, 0x4b, 0xc6, 0x42, 0x9c], // R7
            loadce(bypass, early),
        ];
        code.extend([nop; 6]);
        code.push(loadce(false, late));
        code.extend([nop; 3]);
        code.push(packet);
        code.extend([nop; 4]);
        code.push([0x2b, 0xd8, 0, 0x2b, 0xec, 0]);
        code.extend([nop; 2]);
        fs::write(&binary, code.as_flattened()).unwrap();
        let (r6, r7) = if address { (0x804b, 0xc04c) } else { (0x8049, 0xc04b) };
        let word = if !store { 0xcd01 } else if shift { 0xe901 } else { 0xd202 };
        for (j, (storage, expected)) in [("R6", r6), ("R7", r7)].into_iter().enumerate() {
            let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
            kuna.arg("decompile").arg(&binary).args([
                "0x1000", "--addr", "--raw-image", "--base", "0x1000",
                "--target", "Carmel:BE:24:default:default", "--json", "--assert-strict",
                "--assert", &format!("return %{storage} unsigned short"),
                "--assert", "data [data,0x8049] volatile unsigned short to",
                "--assert", "data [data,0xc04b] volatile unsigned short from",
            ]);
            for (n, input) in ["RN6", "RM6", "RL6", "PACFG", "ECFG", "EST", "CECFG1", "CECFG2", "CETBL"]
                .into_iter().enumerate()
            {
                kuna.args(["--assert", &format!("param {n} %{input} unsigned short in_{n}")]);
            }
            let output = common::process::required_output(&mut kuna);
            let json: Value = serde_json::from_slice(&output.stdout).unwrap();
            let fun = &json["functions"][0];
            assert!(fun["error"].is_null(), "{packet:02x?} {storage}: {fun}");
            let text = fun["code"].as_str().unwrap();
            let name = format!("copy_{i}_{j}");
            writeln!(generated, "#define sub_1000 {name}\n{text}\n#undef sub_1000").unwrap();
            writeln!(calls, "to=0xcd01; from=0xd202; if ({name}(2,0,0,0,{},0,0,0,0) != {expected:#x} || to != {word:#x} || from != 0xd202) return {};", u16::from(shift) << 5, 2 * i + j + 1).unwrap();
        }
    }
    writeln!(generated, "int main(void) {{\n{calls}\nreturn 0;\n}}").unwrap();
    fs::write(&source, &generated).unwrap();
    run_emitted_c(&source, &executable, &generated);
}

#[test]
fn carmel_movu_literal_keeps_agu_and_late_flags_independent() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let scratch = tempfile::tempdir().unwrap();
    let binary = scratch.path().join("movu.bin");
    let source = scratch.path().join("movu.c");
    let executable = scratch.path().join("movu");
    let nop = [0x2b, 0xec, 0, 0x2b, 0xec, 0];
    let loadce = |bypass, table| [0x80, 0, table, 0xcd, 0xe4, if bypass { 0x40 } else { 0 }];
    let long = [0x2c, 0xab, 0x23, 0xc6, 0xd1, 0x7c];
    let rows = [
        ([0x96, 0x75, 0x38, 0xc6, 0xdc, 0x2e], false, 0, 0, "R3", 0xc044, 8),
        (long, false, 0xff, 0, "R7", 0xc046, 9),
        (long, false, 0, 0xff, "R7", 0xc048, 0x3009),
        (long, true, 0, 0xff, "R7", 0xc046, 0x3008),
        ([0x4f, 0xd8, 0x03, 0xc6, 0xe6, 0xb0], true, 0, 0xff, "RN4", 0x792a, 0x3008),
        ([0xd4, 0xcc, 0x42, 0xc6, 0xca, 0x53], false, 0xff, 0xff, "A4L", 0x1a2b, 0x3008),
        ([0xd8, 0xe0, 0xbb, 0xc6, 0xd7, 0x6e], false, 0xff, 0xff, "EST", 0x3008, 0x3008),
    ];
    let inputs = [
        "A0L", "A4L", "RN3", "RN4", "R3", "R7", "PACFG", "RM2", "RL2", "RM6", "RL6",
        "EST", "CECFG1", "CECFG2", "CETBL", "ECFG",
    ];
    let mut generated = String::from("#include <stdbool.h>\n#include <stdint.h>\n");
    let mut calls = String::new();
    for (i, (packet, bypass, early, late, reg, value, est)) in rows.into_iter().enumerate() {
        let mut code = vec![[0x21, 0xac, 0x20, 0x2b, 0xec, 0], loadce(bypass, early)];
        code.extend([nop; 6]);
        code.push(loadce(false, late));
        code.extend([nop; 3]);
        code.push(packet);
        code.extend([nop; 4]);
        code.push([0x2b, 0xd8, 0, 0x2b, 0xec, 0]);
        code.extend([nop; 2]);
        fs::write(&binary, code.as_flattened()).unwrap();
        for (j, (storage, expected)) in [(reg, value), ("EST", est)].into_iter().enumerate() {
            if j == 1 && reg == "EST" { continue; }
            let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
            kuna.arg("decompile").arg(&binary).args([
                "0x1000", "--addr", "--raw-image", "--base", "0x1000",
                "--target", "Carmel:BE:24:default:default", "--json", "--assert-strict",
                "--assert", &format!("return %{storage} unsigned short"),
            ]);
            for (n, input) in inputs.into_iter().enumerate() {
                kuna.args(["--assert", &format!("param {n} %{input} unsigned short in_{n}")]);
            }
            let output = common::process::required_output(&mut kuna);
            let json: Value = serde_json::from_slice(&output.stdout).unwrap();
            let fun = &json["functions"][0];
            assert!(fun["error"].is_null(), "{packet:02x?} {storage}: {fun}");
            let text = fun["code"].as_str().unwrap();
            let name = format!("movu_{i}_{j}");
            writeln!(generated, "#define sub_1000 {name}\n{text}\n#undef sub_1000").unwrap();
            writeln!(calls, "if ({name}(0xffff,0x1a2b,4,0x792a,0xc040,0xc048,0,0,0,0,0,0,0,0,0,0) != {expected:#x}) return {};", 2 * i + j + 1).unwrap();
        }
    }
    writeln!(generated, "int main(void) {{\n{calls}\nreturn 0;\n}}").unwrap();
    fs::write(&source, &generated).unwrap();
    run_emitted_c(&source, &executable, &generated);
}

#[test]
fn carmel_mixed_square_keeps_admission_separate_from_result_enable() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let scratch = tempfile::tempdir().unwrap();
    let binary = scratch.path().join("square.bin");
    let source = scratch.path().join("square.c");
    let executable = scratch.path().join("square");
    let nop = [0x2b, 0xec, 0, 0x2b, 0xec, 0];
    let loadce = |bypass, table| [0x80, 0, table, 0xcd, 0xe4, if bypass { 0x40 } else { 0 }];
    let rows = [
        (false, false, 0, 0xff, [0x2345, 12, 0xc048, 0xc04c, 0x3000]),
        (true, false, 0xff, 0, [0x3456, 0xbbbb, 0xc047, 0xc04d, 0]),
        (true, false, 0, 0xff, [0x3456, 0xbbbb, 0xc048, 0xc04c, 0x3000]),
        (true, true, 0, 0xff, [0, 0xc04f, 0xc047, 0xc04d, 0x3008]),
    ];
    let mut generated = String::from(
        "#include <stdbool.h>\n#include <stdint.h>\n\
         volatile unsigned short mem0 = 4, mem5 = 8, mem2 = 0x8000, mem4 = 0xf;\n",
    );
    let mut calls = String::new();
    for (i, (data, bypass, early, late, expected)) in rows.into_iter().enumerate() {
        let mut code = vec![
            [0x80, 0xc0, 0x40, 0xc6, 0x42, 0x80], // R0
            [0x80, 0xc0, 0x44, 0xc6, 0x42, 0x94], // R5
            [0x80, 0xc0, 0x48, 0xc6, 0x42, 0x88], // R2
            [0x80, 0xc0, 0x4c, 0xc6, 0x42, 0x90], // R4
            loadce(bypass, early),
        ];
        code.extend([nop; 6]);
        code.push(loadce(false, late));
        code.extend([nop; 3]);
        code.push(if data { [0x45, 0xd4, 0x75, 0x68, 0xe8, 0xa9] } else { [0x49, 0x52, 0x59, 0x69, 0xd9, 0x40] });
        code.extend([nop; 4]);
        code.push([0x2b, 0xd8, 0, 0x2b, 0xec, 0]);
        code.extend([nop; 2]);
        fs::write(&binary, code.as_flattened()).unwrap();
        let (left, right) = if data { ("A4L", "A5H") } else { ("A2L", "A3H") };
        for (j, (reg, value)) in [left, right, "R2", "R4", "EST"].into_iter().zip(expected).enumerate() {
            let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
            kuna.arg("decompile").arg(&binary).args([
                "0x1000", "--addr", "--raw-image", "--base", "0x1000",
                "--target", "Carmel:BE:24:default:default", "--json", "--assert-strict",
                "--assert", &format!("return %{reg} unsigned short"),
                "--assert", "data [data,0xc040] volatile unsigned short mem0",
                "--assert", "data [data,0xc044] volatile unsigned short mem5",
                "--assert", "data [data,0xc048] volatile unsigned short mem2",
                "--assert", "data [data,0xc04c] volatile unsigned short mem4",
            ]);
            for (n, input) in [left, right, "EST", "PACFG", "RM2", "RL2", "RM4", "RL4", "CECFG1", "CECFG2", "CETBL"]
                .into_iter().enumerate()
            {
                kuna.args(["--assert", &format!("param {n} %{input} unsigned short in_{n}")]);
            }
            let output = common::process::required_output(&mut kuna);
            let json: Value = serde_json::from_slice(&output.stdout).unwrap();
            let fun = &json["functions"][0];
            assert!(fun["error"].is_null(), "{fun}");
            let text = fun["code"].as_str().unwrap();
            let name = format!("square_{i}_{j}");
            writeln!(generated, "#define sub_1000 {name}\n{text}\n#undef sub_1000").unwrap();
            let (initial_left, initial_right) = if data { (0x3456, 0xbbbb) } else { (0x2345, 0xaaaa) };
            writeln!(calls, "if ({name}({initial_left:#x},{initial_right:#x},0,0,0,0,0,0,0,0,0) != {value:#x}) return {};", i * 5 + j + 1).unwrap();
        }
    }
    writeln!(generated, "int main(void) {{\n{calls}\nreturn 0;\n}}").unwrap();
    fs::write(&source, &generated).unwrap();
    run_emitted_c(&source, &executable, &generated);
}

#[test]
fn carmel_rounded_ff_store_uses_early_address_and_of_samples() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let scratch = tempfile::tempdir().unwrap();
    let binary = scratch.path().join("round.bin");
    const NOP: [u8; 6] = [0x2b, 0xec, 0, 0x2b, 0xec, 0];
    const ROUND: [u8; 6] = [0x56, 0xbf, 0xed, 0x2b, 0xec, 0];
    let mut generated = String::from(
        "#include <stdbool.h>\n#include <stdint.h>\n\
        volatile unsigned short sample, status = 8;\n",
    );
    for (layout, instruction, output) in [
        ("move", [0x90, 0x29, 0x83, 0xc3, 0x66, 0x5e], "X1"),
        ("status", [0xbf, 0x12, 0x34, 0xc3, 0x63, 0x04], "R7"),
    ] {
        let mut code = vec![[0x80, 0xc0, 0x80, 0xc6, 0x42, 0x9c]]; // MOVE R7,0xc080
        code.extend([NOP; 6]);
        code.extend([instruction, ROUND]);
        code.extend([NOP; 4]);
        code.push([0x2b, 0xd8, 0, 0x2b, 0xec, 0]);
        code.extend([NOP; 2]);
        fs::write(&binary, code.as_flattened()).unwrap();
        let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
        kuna.arg("decompile").arg(&binary).args([
            "0x1000", "--addr", "--raw-image", "--base", "0x1000",
            "--target", "Carmel:BE:24:default:default", "--json", "--assert-strict",
            "--assert", &format!("return %{output} unsigned short"),
            "--assert", "data [data,0xc080] volatile unsigned short sample",
        ]);
        if layout == "status" {
            kuna.args(["--assert", "data [data,0x1234] volatile unsigned short status"]);
        }
        let inputs = ["A0L", "FF1", "FF2", "X1", "EST", "CECFG1", "CECFG2", "CETBL", "ECFG",
                      "RN7", "PACFG", "RM6", "RL6"];
        for (i, port) in inputs.into_iter().enumerate() {
            kuna.args(["--assert", &format!("param {i} %{port} unsigned short in_{}", port.to_ascii_lowercase())]);
        }
        let result = common::process::required_output(&mut kuna);
        let json: Value = serde_json::from_slice(&result.stdout).unwrap();
        let function = &json["functions"][0];
        assert!(function["error"].is_null(), "{function}");
        assert!(json["assertions"].as_array().unwrap().iter().all(|row| row["status"] == "applied"), "{json}");
        let code = function["code"].as_str().unwrap().replace("sub_1000", &format!("check_{layout}"));
        writeln!(generated, "{code}").unwrap();
    }
    generated.push_str(r"
int main(void) {
    sample = 0x1357;
    /* The very next OF sees the early FF2 MOVE. The stored word uses that value too. */
    if (check_move(0x8000,0x8000,0x1111,0x3333,0,0,0,0xffff,1,3,0,0,0) != 0x8000 ||
        sample != 0x8000) return 1;
    if (check_move(0x8000,0x8000,0x1111,0x3333,0,0,0,0xffff,5,3,0,0,0) != 0x8000 ||
        sample != 0x7fff) return 2;
    if (check_move(0x8000,0x8000,0x1111,0x3333,0,0,0,0xffff,0x29,3,0,0,0) != 0x8000 ||
        sample != 0x4000) return 3;
    sample = 0x1357;
    /* AEN is off. The old CE admits the address, then the loaded Z vetoes the store. */
    if (check_status(0,0x8000,0x8000,0x3333,0,0x0ee8,0x0ee8,0x0101,0,3,0,0,0) != 0xc083 ||
        sample != 0x1357 || status != 8) return 4;
    if (check_status(0,0x8000,0x8000,0x3333,0,0x0ee8,0x0ee8,0x0202,0,3,0,0,0) != 0xc080 ||
        sample != 0x1357 || status != 8) return 5;
    return 0;
}
");
    let source = scratch.path().join("round.c");
    fs::write(&source, &generated).unwrap();
    run_emitted_c(&source, &scratch.path().join("round.exe"), &generated);

    let mut code = vec![NOP; 6];
    code.push(ROUND);
    code.extend([NOP; 4]);
    code.push([0x13, 0x34, 0x68, 0xc2, 0x2f, 0x50]); // NOT CE1 LC1,*R3++
    code.push([0x2b, 0xd8, 0, 0x2b, 0xec, 0]);
    code.extend([NOP; 2]);
    fs::write(&binary, code.as_flattened()).unwrap();
    let result = Command::new(env!("CARGO_BIN_EXE_kuna")).arg("decompile").arg(&binary).args([
        "0x1000", "--addr", "--raw-image", "--base", "0x1000", "--json",
        "--target", "Carmel:BE:24:default:default",
    ]).output().unwrap();
    assert!(!result.status.success());
    assert!(String::from_utf8_lossy(&result.stderr).contains("Unproved Carmel address admission at CODE 0x1016"));
}

#[test]
fn carmel_late_ff_writeback_does_not_reach_the_next_operand_fetch() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let scratch = tempfile::tempdir().unwrap();
    let binary = scratch.path().join("ff.bin");
    const NOP: [u8; 6] = [0x2b, 0xec, 0, 0x2b, 0xec, 0];
    const ROUND: [u8; 6] = [0x56, 0xbf, 0xed, 0x2b, 0xec, 0];
    let mut generated = String::from(
        "#include <stdbool.h>\n#include <stdint.h>\n\
         volatile unsigned short factor_signed = 0x8001;\n\
         volatile unsigned short factor_unsigned = 0x5678;\n\
         volatile unsigned short stored;\n",
    );
    for (label, gap, consumer, output) in [
        ("adjacent", false, ROUND, "X1"),
        ("gap", true, ROUND, "X1"),
        ("move", false, [0x97, 0x29, 0x83, 0xc3, 0x66, 0x22], "A0H"),
        ("or", false, [0xe0, 0x72, 0x25, 0xe2, 0x22, 0x5e], "A0L"),
    ] {
        let mut code = vec![
            [0x80, 0xc0, 0x80, 0xc6, 0x42, 0x9c], // MOVE R7,0xc080
            [0x80, 0, 3, 0xc6, 0x42, 0xbc],       // MOVE RN7,3
            [0x80, 0, 2, 0xc6, 0x42, 0x88],       // MOVE R2,2
            [0xa2, 0xfa, 2, 0xc2, 0x7a, 0xaa],   // MOVE RN2,R2
            [0x80, 0x80, 0x40, 0xc6, 0x42, 0x88], // MOVE R2,0x8040
            [0x80, 0, 0, 0xc6, 0x43, 0x68],       // MOVE PACFG,0
        ];
        code.extend([NOP; 6]);
        code.push([0x47, 0xce, 0xf5, 0x2b, 0xec, 0]); // MPYSU CE1 FF2, *R7+=RN7, *R2+RN2
        if gap {
            code.push(NOP);
        }
        code.push(consumer);
        code.extend([NOP; 4]);
        code.push([0x2b, 0xd8, 0, 0x2b, 0xec, 0]);
        code.extend([NOP; 2]);
        fs::write(&binary, code.as_flattened()).unwrap();
        let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
        kuna.arg("decompile").arg(&binary).args([
            "0x1000", "--addr", "--raw-image", "--base", "0x1000",
            "--target", "Carmel:BE:24:default:default", "--json", "--assert-strict",
            "--assert", &format!("return %{output} unsigned short"),
            "--assert", "data [data,0xc080] volatile unsigned short factor_signed",
            "--assert", "data [data,0x8042] volatile unsigned short factor_unsigned",
            "--assert", "data [data,0xc083] volatile unsigned short stored",
        ]);
        for (i, port) in ["FF1", "FF2", "X1", "EST", "CECFG1", "CECFG2", "CETBL", "ECFG", "R0"].into_iter().enumerate() {
            kuna.args(["--assert", &format!("param {i} %{port} unsigned short in_{}", port.to_ascii_lowercase())]);
        }
        let result = common::process::required_output(&mut kuna);
        let json: Value = serde_json::from_slice(&result.stdout).unwrap();
        let function = &json["functions"][0];
        assert!(function["error"].is_null(), "{function}");
        let code = function["code"].as_str().unwrap().replace("sub_1000", &format!("seen_{label}"));
        writeln!(generated, "{code}").unwrap();
    }
    generated.push_str(r"
#define ARGS 0x4000,0xf678,0xba98,0,0,0,0xffff,0,0x40
int main(void) {
    if (seen_adjacent(ARGS) != 0xf678 || stored != 0xfd9e) return 1;
    if (seen_gap(ARGS) != 0xd4c4 || stored != 0xf531) return 2;
    if (seen_move(ARGS) != 0xffff) return 3;
    if (seen_or(ARGS) != 0xf678) return 4;
    return 0;
}
");
    let source = scratch.path().join("ff.c");
    fs::write(&source, &generated).unwrap();
    run_emitted_c(&source, &scratch.path().join("ff.exe"), &generated);
}

#[test]
fn carmel_qualified_bank_exchange_preserves_independent_address_and_result_effects() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let binary = common::scratch_file("carmel-qualified-bank-exchange", "bin");
    let source = binary.with_extension("c");
    let executable = binary.with_extension("exe");
    const NOP: [u8; 6] = [0x2b, 0xec, 0, 0x2b, 0xec, 0];
    let mut code = vec![NOP; 6];
    code.push([0x8c, 0, 1, 0xcd, 0xe4, 0x33]); // LOADCE 0x0ccc,1
    code.extend([NOP; 3]);
    code.push([0x2b, 0xb4, 7, 0x20, 0xba, 2]); // BANKXCHG CE1 || CMPU A0L,A1L
    code.push([0x2b, 0xd8, 0, 0x2b, 0xec, 0]);
    code.extend([NOP; 2]);
    fs::write(&binary, code.as_flattened()).unwrap();
    let mut generated = String::from("#include <stdbool.h>\n#include <stdint.h>\n");
    for (register, bank) in [
        ("R0", ["R0", "R0S"]), ("A2L", ["A2L", "A2LS"]),
        ("ESTS", ["ESTS", "R0"]), ("EST", ["ESTS", "R0"]),
    ] {
        let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
        kuna.arg("decompile").arg(&binary).args([
            "0x1000", "--addr", "--raw-image", "--base", "0x1000",
            "--target", "Carmel:BE:24:default:default", "--json", "--assert-strict",
            "--assert", &format!("return %{register} unsigned short"),
        ]);
        for (i, input) in bank.into_iter().chain(["EST", "CECFG1", "CECFG2", "CETBL", "A0L", "A1L"]).enumerate() {
            kuna.args(["--assert", &format!("param {i} %{input} unsigned short in_{}", input.to_ascii_lowercase())]);
        }
        let output = common::process::required_output(&mut kuna);
        let json: Value = serde_json::from_slice(&output.stdout).unwrap();
        let function = &json["functions"][0];
        assert!(function["error"].is_null(), "{function}");
        let name = format!("bank_{}", register.to_ascii_lowercase());
        writeln!(generated, "{}", function["code"].as_str().unwrap().replace("sub_1000", &name)).unwrap();
    }
    generated.push_str(r"
#define CLOCK(seed) 0,0x1000,0x1000,seed,4,4
int main(void) {
    if (bank_r0(0x1234,0x4567,CLOCK(0)) != 0x4567 ||
        bank_a2l(0x2222,0x3333,CLOCK(0)) != 0x3333 ||
        bank_ests(0x0048,0,CLOCK(0)) != 0 || bank_est(0x0048,0,CLOCK(0)) != 0x0048) return 1;
    if (bank_r0(0x1234,0x4567,CLOCK(0xffff)) != 0x4567 ||
        bank_a2l(0x2222,0x3333,CLOCK(0xffff)) != 0x2222 ||
        bank_ests(0x0048,0,CLOCK(0xffff)) != 0x0048 ||
        bank_est(0x0048,0,CLOCK(0xffff)) != 0x3240) return 2;
    return 0;
}
");
    fs::write(&source, &generated).unwrap();
    run_emitted_c(&source, &executable, &generated);
    for file in [binary, source, executable] {
        fs::remove_file(file).unwrap();
    }
}

#[test]
fn carmel_direct_redirect_preserves_earlier_issue_admission() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let binary = common::scratch_file("carmel-redirect-admission", "bin");
    let source = binary.with_extension("c");
    let executable = binary.with_extension("exe");
    const NOP: [u8; 6] = [0x2b, 0xec, 0, 0x2b, 0xec, 0];
    let mut generated = String::from("#include <stdbool.h>\n#include <stdint.h>\n");
    for (gap, bank) in [(0, 0), (1, 1)] {
        let mut code = vec![NOP; 6];
        code.push([0x8c, 0, 1, 0xcd, 0xe4, 0x33]); // LOADCE 0x0ccc,1
        code.extend(std::iter::repeat_n(NOP, gap));
        code.push([0x80, 0, 6, 0xcf, 0x80, 0]); // GOTO across two NOP fetches
        code.extend([NOP; 2]);
        code.push([0x36 | (bank << 6), 0x21, 0xb9, 0xc0, 0xac, 0x89]); // TRANSFER R2, INTBASE+0x21b9
        code.push([0x2b, 0xd8, 0, 0x2b, 0xec, 0]);
        code.extend([NOP; 2]);
        fs::write(&binary, code.as_flattened()).unwrap();
        for register in ["R2", "EST"] {
            let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
            kuna.arg("decompile").arg(&binary).args([
                "0x1000", "--addr", "--raw-image", "--base", "0x1000",
                "--target", "Carmel:BE:24:default:default", "--json", "--assert-strict",
                "--assert", &format!("return %{register} unsigned short"),
            ]);
            for (i, input) in ["R2", "INTBASE", "EST", "CECFG1", "CECFG2", "CETBL"].into_iter().enumerate() {
                kuna.args(["--assert", &format!("param {i} %{input} unsigned short in_{}", input.to_ascii_lowercase())]);
            }
            let output = common::process::required_output(&mut kuna);
            let json: Value = serde_json::from_slice(&output.stdout).unwrap();
            let function = &json["functions"][0];
            assert!(function["error"].is_null(), "{function}");
            let name = format!("redirect_{gap}_{}", register.to_ascii_lowercase());
            writeln!(generated, "{}", function["code"].as_str().unwrap().replace("sub_1000", &name)).unwrap();
        }
    }
    generated.push_str(r"
#define CLOCK(seed) 0x6666,2,0,0x1000,0x1000,seed
int main(void) {
    if (redirect_0_r2(CLOCK(0)) != 0x21bb || redirect_0_est(CLOCK(0)) != 0) return 1;
    if (redirect_0_r2(CLOCK(0xffff)) != 0x6666 || redirect_0_est(CLOCK(0xffff)) != 0x3000) return 2;
    if (redirect_1_r2(CLOCK(0)) != 0x6666 || redirect_1_est(CLOCK(0)) != 0x3000) return 3;
    if (redirect_1_r2(CLOCK(0xffff)) != 0x21bb || redirect_1_est(CLOCK(0xffff)) != 0) return 4;
    return 0;
}
");
    fs::write(&source, &generated).unwrap();
    run_emitted_c(&source, &executable, &generated);
    for file in [binary, source, executable] {
        fs::remove_file(file).unwrap();
    }
}

#[test]
fn carmel_conditional_redirect_uses_the_executed_predecessor_clock() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let binary = common::scratch_file("carmel-conditional-redirect-admission", "bin");
    let source = binary.with_extension("c");
    let executable = binary.with_extension("exe");
    const NOP: [u8; 6] = [0x2b, 0xec, 0, 0x2b, 0xec, 0];
    let mut generated = String::from(
        "#include <stdbool.h>\n#include <stdint.h>\nvolatile unsigned short phase_word = 0x00ff;\n",
    );
    for (label, slot) in [
        ("parallel", [0x21, 0xac, 0x20, 0x2b, 0xec, 0]),
        ("sequential", [0xa1, 0xac, 0x20, 0x2b, 0xec, 0]),
        ("unknown", [0x20, 0xba, 0x02, 0x2b, 0xec, 0]),
    ] {
        let mut code = vec![[0x80, 0xc0, 0x40, 0xc6, 0x42, 0x8c]]; // MOVE R3, 0xc040
        code.push([0x2b, 0xec, 0, 0x21, 0xac, 0x62]); // Right INC writes the EQ2 condition
        code.extend([NOP; 7]);
        code.push([0x28, 0x5c, 0x31, 0x2b, 0xec, 0]); // GOTO EQ2 with one delay issue
        code.push(slot);
        code.push(NOP);
        code.push([0x80, 0, 6, 0xcf, 0x80, 0]); // Untaken path reaches the join later
        code.extend([NOP; 2]);
        code.push([0x13, 0x34, 0x68, 0xc2, 0x2f, 0x50]); // NOT CE1 LC1,*R3++
        code.extend([NOP; 4]);
        code.push([0x2b, 0xd8, 0, 0x2b, 0xec, 0]);
        code.extend([NOP; 2]);
        fs::write(&binary, code.as_flattened()).unwrap();
        for register in ["R3", "LC1"] {
            let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
            kuna.arg("decompile").arg(&binary).args([
                "0x1000", "--addr", "--raw-image", "--base", "0x1000",
                "--target", "Carmel:BE:24:default:default", "--json", "--assert-strict",
                "--assert", &format!("return %{register} unsigned short"),
                "--assert", "data [data,0xc040] volatile unsigned short phase_word",
            ]);
            for (i, input) in ["R3", "LC1", "EST", "CECFG1", "CECFG2", "CETBL",
                "PACFG", "RM2", "RL2", "A0L", "A1L", "ECFG"].into_iter().enumerate() {
                kuna.args(["--assert", &format!("param {i} %{input} unsigned short in_{}", input.to_ascii_lowercase())]);
            }
            let output = kuna.output().unwrap();
            let json: Value = serde_json::from_slice(&output.stdout).unwrap();
            let function = &json["functions"][0];
            if label == "unknown" {
                assert!(!output.status.success() && function["code"].is_null(), "{function}");
                break;
            }
            assert!(output.status.success() && function["error"].is_null(), "{function}");
            let name = format!("{label}_{}", register.to_ascii_lowercase());
            writeln!(generated, "{}", function["code"].as_str().unwrap().replace("sub_1000", &name)).unwrap();
        }
    }
    generated.push_str(r"
#define INPUTS(a1) 0xc040,0x4567,0,0x888,0x888,0x0180,0,0,0,0xffff,a1,0
int main(void) {
    if (parallel_r3(INPUTS(0xffff)) != 0xc040 || parallel_lc1(INPUTS(0xffff)) != 0x4567) return 1;
    if (parallel_r3(INPUTS(2)) != 0xc041 || parallel_lc1(INPUTS(2)) != 0xff00) return 2;
    if (sequential_r3(INPUTS(0xffff)) != 0xc040 || sequential_lc1(INPUTS(0xffff)) != 0x4567) return 3;
    if (sequential_r3(INPUTS(2)) != 0xc041 || sequential_lc1(INPUTS(2)) != 0xff00) return 4;
    return 0;
}
");
    fs::write(&source, &generated).unwrap();
    run_emitted_c(&source, &executable, &generated);
    for file in [binary, source, executable] {
        fs::remove_file(file).unwrap();
    }
}

#[test]
fn carmel_absolute_est_load_records_rails_before_its_writeback() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let binary = common::scratch_file("carmel-est-load-admission", "bin");
    let source = binary.with_extension("c");
    let executable = binary.with_extension("exe");
    const NOP: [u8; 6] = [0x2b, 0xec, 0, 0x2b, 0xec, 0];
    let mut generated = String::from(
        "#include <stdbool.h>\n#include <stdint.h>\nvolatile unsigned short status_word = 0x8008;\n",
    );
    for (gap, bank) in [(0, 0), (4, 0), (5, 1)] {
        let mut code = vec![NOP; 6];
        code.push([0xbf, 0x12, 0x34, 0xc3, 0x63, 0x04]); // LOAD EST,*0x1234
        code.extend(std::iter::repeat_n(NOP, gap));
        code.push([0x36 | (bank << 6), 0x21, 0xb9, 0xc0, 0xac, 0x89]);
        code.push([0x2b, 0xd8, 0, 0x2b, 0xec, 0]);
        code.extend([NOP; 2]);
        fs::write(&binary, code.as_flattened()).unwrap();
        let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
        kuna.arg("decompile").arg(&binary).args([
            "0x1000", "--addr", "--raw-image", "--base", "0x1000",
            "--target", "Carmel:BE:24:default:default", "--json", "--assert-strict",
            "--assert", "return %R2 unsigned short",
            "--assert", "data [data,0x1234] volatile unsigned short status_word",
        ]);
        for (i, input) in ["R2", "INTBASE", "EST", "CECFG1", "CECFG2", "CETBL"].into_iter().enumerate() {
            kuna.args(["--assert", &format!("param {i} %{input} unsigned short in_{}", input.to_ascii_lowercase())]);
        }
        let output = common::process::required_output(&mut kuna);
        let json: Value = serde_json::from_slice(&output.stdout).unwrap();
        let function = &json["functions"][0];
        assert!(function["error"].is_null(), "{function}");
        let name = format!("load_gap_{gap}");
        writeln!(generated, "{}", function["code"].as_str().unwrap().replace("sub_1000", &name)).unwrap();
    }
    generated.push_str(r"
#define CLOCK(table) 0x6666,2,0,0x888,0x888,table
int main(void) {
    if (load_gap_0(CLOCK(0x0101)) != 0x21bb || load_gap_0(CLOCK(0x8080)) != 0x6666) return 1;
    if (load_gap_4(CLOCK(0x0101)) != 0x21bb || load_gap_4(CLOCK(0x8080)) != 0x6666) return 2;
    if (load_gap_5(CLOCK(0x0101)) != 0x6666 || load_gap_5(CLOCK(0x8080)) != 0x21bb) return 3;
    if (status_word != 0x8008) return 4;
    return 0;
}
");
    fs::write(&source, &generated).unwrap();
    run_emitted_c(&source, &executable, &generated);
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
fn carmel_wide_packed_arithmetic_routes_independent_lanes_to_emitted_c() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let binary = common::scratch_file("carmel-wide-packed-arithmetic", "bin");
    let source = binary.with_extension("c");
    let executable = binary.with_extension("exe");
    let registers = [
        "A0L", "A0H", "GB0", "A1L", "A1H", "GB1", "A2L", "A2H", "GB2", "FF1", "FF2",
        "BTR1", "BTR2", "EST", "CECFG1", "CECFG2", "CETBL",
    ];
    let seeds = [
        [0xffff, 0x8001, 0xf0, 2, 0x7fff, 0x20, 0x1234, 0xff7f, 0x54, 0xabcd, 0x7fff, 0x8115, 0x005a, 0, 0, 0, 0],
        [0, 3, 1, 0xffff, 3, 0xee, 0x1234, 0xff7f, 0x54, 0xabcd, 5, 0x8115, 0x005a, 0, 0, 0, 0],
    ];
    let rows: [([u8; 6], bool, &[(&str, u16, u16)]); 12] = [
        // Each expected pair corresponds to one seed above, including carry, borrow and ties.
        ([0x09, 0, 0, 0xc4, 0x4a, 0x91], false, &[("A2L", 1, 0xffff), ("A2H", 0, 6), ("GB2", 0x11, 0xef)]),
        ([0x09, 0, 0, 0xc4, 0xca, 0x91], false, &[("A2L", 0xfffd, 1), ("A2H", 2, 0), ("GB2", 0xd0, 0x13)]),
        ([0x09, 0, 0, 0xc4, 0xd2, 0x91], false, &[("A2L", 2, 0), ("A2H", 0xff7f, 0xff7f), ("GB2", 0x54, 0x54)]),
        ([0x09, 0, 0, 0xc5, 0x4a, 0x91], false, &[("A2L", 2, 0), ("A2H", 0x7fff, 3), ("GB2", 0, 0), ("BTR1", 0x022a, 0x022a), ("BTR2", 0xb4, 0xb5)]),
        ([0x09, 0, 0, 0xc5, 0xca, 0x91], false, &[("A2L", 0xffff, 0xffff), ("A2H", 0x8001, 3), ("GB2", 0xff, 0), ("BTR1", 0x022b, 0x022a), ("BTR2", 0xb5, 0xb4)]),
        ([0x09, 0, 0, 0xc5, 0xc0, 0x91], false, &[("GB0", 0xf0, 1), ("BTR1", 0x022b, 0x022a), ("BTR2", 0xb5, 0xb4)]),
        ([0x00, 0x92, 0x34, 0xc4, 0x4a, 0x9f], true, &[("A2L", 0xfff1, 0xf2), ("A2H", 0xffff, 0), ("GB2", 0xff, 0)]),
        ([0x0f, 0x92, 0x34, 0xc4, 0x4a, 0x81], false, &[("A2L", 0x9324, 0x9235), ("GB2", 0xff, 0xff)]),
        ([0x09, 0x89, 0xfc, 0xc4, 0xcf, 0x91], false, &[("A0L", 0xffff, 0), ("A1H", 0x7fff, 3), ("EST", 0x3000, 0x3000)]),
        ([0x09, 0x89, 0xfc, 0xc4, 0xfe, 0x91], false, &[("FF1", 2, 0)]),
        ([0x01, 0x89, 0xfc, 0xc4, 0xc6, 0x81], false, &[("GB0", 0, 0), ("A0H", 0x8001, 3)]),
        ([0x08, 0x89, 0xfc, 0xc4, 0xc7, 0xaf], false, &[("A0L", 0x8000, 5), ("A0H", 0x7fff, 0xfffd), ("GB0", 0x0f, 0xfe)]),
    ];
    let c_type = |register: &str| if register.starts_with("GB") { "unsigned char" } else { "unsigned short" };
    let mut emitted = String::from(
        "#include <stdbool.h>\n#include <stdint.h>\n\
         #define CONCAT12(h,l) (((uint32_t)(uint8_t)(h) << 16) | (uint16_t)(l))\n\
         #define CONCAT22(h,l) (((uint32_t)(uint16_t)(h) << 16) | (uint16_t)(l))\n\
         volatile unsigned short data_tail;\n",
    );
    let mut main = String::from("int main(void) {\n");
    for (i, (packet, data, outputs)) in rows.into_iter().enumerate() {
        fs::write(&binary, [packet, [0x2b, 0xd8, 0, 0x2b, 0xec, 0]].as_flattened()).unwrap();
        for &(register, first, second) in outputs {
            let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
            kuna.arg("decompile").arg(&binary).args([
                "0", "--addr", "--raw-image", "--target", "Carmel:BE:24:default:default", "--base", "0",
                "--json", "--assert-strict", "--assert", &format!("return %{register} {}", c_type(register)),
            ]);
            for (index, input) in registers.into_iter().enumerate() {
                kuna.args(["--assert", &format!("param {index} %{input} {} in_{index}", c_type(input))]);
            }
            if data {
                kuna.args(["--assert", "data [data,0xffff] volatile unsigned short data_tail"]);
            }
            let result = common::process::required_output(&mut kuna);
            let json: Value = serde_json::from_slice(&result.stdout).unwrap();
            let [function] = json["functions"].as_array().unwrap().as_slice() else { panic!("{json}"); };
            let code = function["code"].as_str().unwrap();
            assert!(function["error"].is_null() && !code.contains("halt_"), "{function}");
            assert!(json["assertions"].as_array().unwrap().iter().all(|a| a["status"] == "applied"), "{json}");
            let name = format!("packed_{i}_{register}");
            writeln!(emitted, "#define sub_0 {name}\n{code}\n#undef sub_0").unwrap();
            let port = registers.iter().position(|name| *name == register).unwrap();
            for ((mut values, expected), word) in seeds.into_iter().zip([first, second]).zip([0xff01, 0x00f1]) {
                for (gate, wanted) in [(0, values[port]), (0xffff, expected)] {
                    *values.last_mut().unwrap() = gate;
                    let args = values.map(|value| format!("{value:#x}")).join(",");
                    writeln!(main, "data_tail = {word}; if ({name}({args}) != {wanted} || data_tail != {word}) return {};", i + 1).unwrap();
                }
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

#[test]
fn carmel_compact_signed_long_data_uses_the_connected_ports() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let binary = common::scratch_file("carmel-compact-signed-long-data", "bin");
    let source = binary.with_extension("c");
    let executable = binary.with_extension("exe");
    let mut emitted = String::from(
        "#include <stdbool.h>\n#include <stdint.h>\n\
         #define CONCAT22(h,l) (((uint32_t)(uint16_t)(h) << 16) | (uint16_t)(l))\n\
         volatile unsigned short low_word, high_word;\n",
    );
    for (name, packet, storage, ty) in [
        ("store_ce", [0x2b, 0xec, 0, 0x31, 0xea, 0x61], "R6", "unsigned short"),
        ("store_routed_ea", [0x2b, 0xec, 0, 0x31, 0xea, 0xe0], "R6", "unsigned short"),
        ("store_routed_eb", [0x2b, 0xec, 0, 0x31, 0xeb, 0xe0], "R6", "unsigned short"),
        ("load_guard", [0x31, 0xd8, 0x6c, 0x2b, 0xec, 0], "GB0", "unsigned char"),
        ("store_address", [0x31, 0xe4, 0xc4, 0x2b, 0xec, 0], "R6", "unsigned short"),
        ("load_address", [0x31, 0xf1, 0x4d, 0x2b, 0xec, 0], "LSAL", "unsigned short"),
        ("load_address_high", [0x31, 0xf1, 0x4d, 0x2b, 0xec, 0], "LSA1H", "unsigned char"),
    ] {
        let image = [
            [0x80, 0x80, 0x48, 0xc6, 0x42, 0x98], // MOVE R6,0x8048
            [0x80, 0, 3, 0xc6, 0x42, 0xb8],       // MOVE RN6,3
            packet,
            [0x2b, 0xd8, 0, 0x2b, 0xec, 0],
        ];
        fs::write(&binary, image.as_flattened()).unwrap();
        let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
        kuna.arg("decompile").arg(&binary).args([
            "0", "--addr", "--raw-image", "--target", "Carmel:BE:24:default:default", "--base",
            "0", "--json", "--assert-strict", "--assert", &format!("return %{storage} {ty}"),
            "--assert", "data [data,0x804b] volatile unsigned short low_word",
            "--assert", "data [data,0x804c] volatile unsigned short high_word",
        ]);
        for (i, (input, ty)) in [
            ("A0L", "unsigned short"), ("A0H", "unsigned short"), ("GB0", "unsigned char"),
            ("LSA1L", "unsigned short"), ("LSA1H", "unsigned char"), ("EST", "unsigned short"),
            ("ECFG", "unsigned short"), ("CECFG1", "unsigned short"), ("CECFG2", "unsigned short"),
            ("CETBL", "unsigned short"),
        ].into_iter().enumerate() {
            kuna.args(["--assert", &format!("param {i} %{input} {ty} in_{}", input.to_ascii_lowercase())]);
        }
        let output = common::process::required_output(&mut kuna);
        let json: Value = serde_json::from_slice(&output.stdout).unwrap();
        let function = &json["functions"][0];
        assert!(function["error"].is_null(), "{function}");
        let c = function["code"].as_str().unwrap();
        assert!(!c.contains("halt_"), "{function}");
        writeln!(emitted, "{}", c.replace("sub_0", name)).unwrap();
    }
    emitted.push_str(
        "#define INPUTS(config,table) 0x1234,0x5678,0xea,0x4567,0x9a,0,config,0x1000,0x1000,table\n\
         int main(void) {\n\
         low_word = 0x81ab; high_word = 0xeefa;\n\
         if (store_ce(INPUTS(0,0xff)) != 0x8048 || low_word != 0x81ab || high_word != 0xeefa) return 1;\n\
         if (load_guard(INPUTS(0,0)) != 0xff || low_word != 0x81ab || high_word != 0xeefa) return 2;\n\
         if (load_address(INPUTS(0,0)) != 0x81aa || load_address_high(INPUTS(0,0)) != 0xfa) return 3;\n\
         if (store_ce(INPUTS(0,0xff00)) != 0x8048 || low_word != 0x1234 || high_word != 0x5678) return 4;\n\
         if (store_ce(INPUTS(8,0xff00)) != 0x8048 || low_word != 0 || high_word != 0x8000) return 5;\n\
         if (store_address(INPUTS(0,0)) != 0x8048 || low_word != 0x4567 || high_word != 0x9a) return 6;\n\
         if (store_routed_ea(INPUTS(0,0)) != 0x8048 || low_word != 0x1234 || high_word != 0x5678) return 7;\n\
         low_word = 0x81ab; high_word = 0xeefa;\n\
         if (store_routed_eb(INPUTS(0,0)) != 0x8048 || low_word != 0x1234 || high_word != 0x5678) return 8;\n\
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

#[test]
fn carmel_wide_io_disconnected_destinations_keep_the_guarded_read() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let binary = common::scratch_file("carmel-wide-io-disconnected", "bin");
    let source = binary.with_extension("c");
    let executable = binary.with_extension("exe");
    let mut emitted = String::from(
        "#include <stdbool.h>\n#include <stdint.h>\n\
         static volatile unsigned short device_word = 0x96b5;\n\
         static unsigned accesses;\n\
         static volatile unsigned short *device(void) { ++accesses; return &device_word; }\n\
         #define io_cell (*device())\n",
    );
    let mut calls = String::from("int main(void) {\n");
    for bank in 0..2 {
        for selector in [0x46, 0x47] {
            let packet = [bank << 6, 1, 2, 0xc6, selector, 0xc0];
            let nop = [0x2b, 0xec, 0, 0x2b, 0xec, 0];
            let body = [packet, nop, nop, nop, nop, [0x2b, 0xd8, 0, 0x2b, 0xec, 0]];
            fs::write(&binary, body.as_flattened()).unwrap();
            let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
            kuna.arg("decompile").arg(&binary).args([
                "0", "--addr", "--raw-image", "--target", "Carmel:BE:24:default:default", "--base", "0",
                "--json", "--assert-strict",
                "--assert", &format!("param 0 %CECFG{} unsigned short config", bank + 1),
                "--assert", "param 1 %EST unsigned short status",
                "--assert", "param 2 %CETBL unsigned short table",
                "--assert", "param 3 %RL0 unsigned short old_rl0",
                "--assert", "return %RL0 unsigned short",
                "--assert", "data [io,0x102] volatile unsigned short io_cell",
            ]);
            let output = common::process::required_output(&mut kuna);
            let json: Value = serde_json::from_slice(&output.stdout).unwrap();
            let function = &json["functions"][0];
            assert!(function["error"].is_null(), "{function}");
            let c = function["code"].as_str().unwrap();
            assert!(!c.contains("halt_") && c.contains("io_cell"), "{function}");
            let name = format!("read_io_{bank}_{selector}");
            writeln!(emitted, "#define sub_0 {name}\n{c}\n#undef sub_0").unwrap();
            let selected = 0xff << (8 * bank);
            let other = 0xffff ^ selected;
            for (test, (config, table, expected)) in [
                (0, 0, 0), (0, other, 0), (0, selected, 1), (0x1000, 0, 1),
                (0x1000, selected, 1),
            ].into_iter().enumerate() {
                writeln!(calls,
                    "accesses = 0;\n\
                     if ({name}({config:#x},0,{table:#x},0x8732) != 0x8732 || \
                         accesses != {expected} || device_word != 0x96b5) return {};",
                    1 + 10 * usize::from(bank) + 5 * usize::from(selector - 0x46) + test,
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
fn carmel_compact_mac_preg_ports_execute_in_emitted_c() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let binary = common::scratch_file("carmel-compact-mac-preg-ports", "bin");
    let source = binary.with_extension("c");
    let executable = binary.with_extension("exe");
    let mut emitted = String::from(
        "#include <stdbool.h>\n#include <stdint.h>\n\
         volatile unsigned short cell_4040, cell_8048, cell_804b, cell_c048;\n",
    );
    let mut calls = String::from("int main(void) {\n");
    for (route, (packet, outputs)) in [
        ([0x5b, 0xa2, 0xe6, 0x6c, 0x77, 0xef], &["EST", "R3", "R6", "R7"][..]),
        ([0x5b, 0xa2, 0xe6, 0x00, 0x0f, 0xd4], &["EST", "R2"][..]),
        ([0x2b, 0xec, 0x00, 0x6c, 0x77, 0xcf], &["EST", "R6"][..]),
    ].into_iter().enumerate() {
        let nop = [0x2b, 0xec, 0, 0x2b, 0xec, 0];
        let code = [
            [0x80, 0x80, 0x48, 0xc6, 0x42, 0x98], // MOVE R6,0x8048
            [0x80, 0xc0, 0x48, 0xc6, 0x42, 0x9c], // MOVE R7,0xc048
            nop, nop, nop, packet, nop, nop, nop,
            [0x2b, 0xd8, 0, 0x2b, 0xec, 0],
        ];
        fs::write(&binary, code.as_flattened()).unwrap();
        for (port, register) in outputs.iter().enumerate() {
            let name = format!("preg_{route}_{port}");
            let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
            kuna.arg("decompile").arg(&binary).args([
                "0", "--addr", "--raw-image", "--target", "Carmel:BE:24:default:default", "--base", "0",
                "--json", "--assert-strict", "--assert", &format!("return %{register} unsigned short"),
            ]);
            for address in [0x4040, 0x8048, 0x804b, 0xc048] {
                kuna.args(["--assert", &format!("data [data,{address:#x}] volatile unsigned short cell_{address:x}")]);
            }
            let params = [
                "A1L", "A1H", "GB1", "EST", "CECFG1", "CECFG2", "CETBL", "R1", "R2", "R3",
                "RN2", "RN3", "RN6", "RN7", "PACFG", "RM2", "RL2", "RM6", "RL6",
            ];
            for (i, reg) in params.into_iter().enumerate() {
                let ty = if reg == "GB1" { "unsigned char" } else { "unsigned short" };
                kuna.args(["--assert", &format!("param {i} %{reg} {ty} in_{}", reg.to_ascii_lowercase())]);
            }
            let output = common::process::required_output(&mut kuna);
            let json: Value = serde_json::from_slice(&output.stdout).unwrap();
            let function = &json["functions"][0];
            assert!(function["error"].is_null(), "{function}");
            let c = function["code"].as_str().unwrap();
            assert!(!c.contains("halt_"), "{function}");
            writeln!(emitted, "{}", c.replace("sub_0", &name)).unwrap();

            // CE1 and CE2 differ in the qualified trials. A CE2 veto with
            // its override still advances the AGU without driving WB.
            let trials: &[(u16, u16, u16, u16, &[u16])] = match route {
                0 => &[
                    (0x00ff, 0x1000, 0x1234, 0x4321, &[0x1020, 0xc041, 0x804b, 0xc049]),
                    (0xff00, 0x1000, 0x1234, 0x4321, &[0x2220, 0xc041, 0x804b, 0xc049]),
                    (0x00ff, 0, 0x1234, 0x4321, &[0x1020, 0xc041, 0x8048, 0xc048]),
                ],
                1 => &[(0, 0, 0x1234, 0x4321, &[0x260, 0x8043])],
                _ => &[
                    (0x00ff, 0x1000, 0x1234, 0x4321, &[0x1000, 0x804b]),
                    (0xff00, 0x1000, 0x1234, 0x1234, &[0x2000, 0x804b]),
                    (0xff00, 0x1000, 0, 0, &[0x2200, 0x804b]),
                ],
            };
            for (trial, &(table, cfg, input, data, results)) in trials.iter().enumerate() {
                let expected = results[port];
                writeln!(calls,
                    "cell_4040=0x3210; cell_8048=0x4321; cell_804b=0x7654; cell_c048={input:#x};\n\
                     if ({name}(0xa8d0,0x8383,0x6a,0,0x1000,{cfg:#x},{table:#x},\
                                0x4040,0x8040,0xc040,3,4,3,4,0,0,0,0,0) != {expected:#x} || \
                         cell_4040 != 0x3210 || cell_8048 != {data:#x} || \
                         cell_804b != 0x7654 || cell_c048 != {input:#x}) return {};",
                    1 + 16 * route + 4 * port + trial,
                ).unwrap();
            }
        }
    }
    emitted.push_str(&calls);
    emitted.push_str("return 0;\n}\n");
    fs::write(&source, &emitted).unwrap();
    run_emitted_c(&source, &executable, &emitted);

    for packet in [
        [0x5b, 0xa2, 0xe6, 0x6c, 0x72, 0x8f], // RN2 reconnects the third DATA read.
        [0x61, 0x8e, 0xe9, 0x6c, 0x77, 0xcf], // ALU peer still duplicates reads.
        [0x61, 0x8e, 0xe9, 0, 0x0f, 0x95],
    ] {
        fs::write(&binary, packet).unwrap();
        let (stdout, stderr, status) = common::run_kuna(&[
            "raw-pcode", binary.to_str().unwrap(), "0", "--raw-image", "--target",
            "Carmel:BE:24:default:default", "--base", "0", "--count", "1", "--json",
        ]);
        let doc: Value = serde_json::from_str(&stdout).unwrap_or_else(|_| panic!("{stdout}\n{stderr}"));
        assert_eq!(status, 1, "{doc}");
        assert_eq!(doc["error"]["address"], 0, "{doc}");
        assert!(doc["instructions"].as_array().unwrap().is_empty(), "{doc}");
    }
    for file in [binary, source, executable] {
        fs::remove_file(file).unwrap();
    }
}

#[test]
fn carmel_compact_indexed_multiply_routes_execute_in_emitted_c() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let binary = common::scratch_file("carmel-compact-indexed-multiply", "bin");
    let source = binary.with_extension("c");
    let executable = binary.with_extension("exe");
    let cells = [0x100, 0x101, 0x4112, 0x8100, 0x8101, 0xc110];
    let mut emitted = String::from(
        "#include <stdbool.h>\n#include <stdint.h>\n\
         #define CONCAT12(h,l) (((uint32_t)(uint8_t)(h) << 16) | (uint16_t)(l))\n\
         #define CONCAT22(h,l) (((uint32_t)(uint16_t)(h) << 16) | (uint16_t)(l))\n\
         #define CONCAT44(h,l) (((uint64_t)(uint32_t)(h) << 32) | (uint32_t)(l))\n",
    );
    for address in cells {
        writeln!(emitted, "volatile unsigned short cell_{address:x};").unwrap();
    }
    let mut calls = String::from("int main(void) {\n");
    for (route, (packet, ports, taken, vetoed)) in [
        (
            [0xb6, 0xf3, 0x20, 0x4c, 0xe0, 0x5b],
            ["X1", "X2", "A3L", "R7", "EST"],
            [0xfffd, 0x5678, 30, 0xc111, 0x3000],
            [0xfffd, 0x5678, 30, 0xc111, 0],
        ),
        (
            [0x3d, 0xcb, 0x94, 0x48, 0x51, 0x36],
            ["X1", "X2", "A4H", "R7", "EST"],
            [0x1234, 7, 0xfc00, 0xc111, 0x3411],
            [0x1234, 7, 0xfc00, 0xc111, 0x0411],
        ),
    ]
    .into_iter()
    .enumerate()
    {
        let nop = [0x2b, 0xec, 0, 0x2b, 0xec, 0];
        let code = [
            [0x80, 0x01, 0, 0xc6, 0x42, 0x80], // R0 = 0x0100
            [0x80, 0x81, 0, 0xc6, 0x42, 0x88], // R2 = 0x8100
            [0x80, 0x41, 0x10, 0xc6, 0x42, 0x94], // R5 = 0x4110
            [0x80, 0xc1, 0x10, 0xc6, 0x42, 0x9c], // R7 = 0xc110
            [0x80, 0, 2, 0xc6, 0x42, 0xb4], // RN5 = 2
            nop,
            nop,
            nop,
            packet,
            nop,
            nop,
            nop,
            [0x2b, 0xd8, 0, 0x2b, 0xec, 0],
        ];
        fs::write(&binary, code.as_flattened()).unwrap();
        for (port, register) in ports.into_iter().enumerate() {
            let name = format!("indexed_{route}_{port}");
            let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
            kuna.arg("decompile").arg(&binary).args([
                "0", "--addr", "--raw-image", "--target", "Carmel:BE:24:default:default", "--base", "0",
                "--json", "--assert-strict", "--assert", &format!("return %{register} unsigned short"),
            ]);
            for address in cells {
                kuna.args(["--assert", &format!("data [data,{address:#x}] volatile unsigned short cell_{address:x}")]);
            }
            for (i, reg) in [
                "CETBL", "CECFG1", "CECFG2", "EST", "ECFG", "PACFG", "RM6", "RL6",
                "X1", "X2", "FF2", "A1L", "A1H", "GB1", "A4H",
            ].into_iter().enumerate() {
                let ty = if reg == "GB1" { "unsigned char" } else { "unsigned short" };
                kuna.args(["--assert", &format!("param {i} %{reg} {ty} in_{}", reg.to_ascii_lowercase())]);
            }
            let output = common::process::required_output(&mut kuna);
            let json: Value = serde_json::from_slice(&output.stdout).unwrap();
            let function = &json["functions"][0];
            assert!(function["error"].is_null(), "{function}");
            let c = function["code"].as_str().unwrap();
            assert!(!c.contains("halt_"), "{function}");
            writeln!(emitted, "{}", c.replace("sub_0", &name)).unwrap();
            for (trial, (table, results)) in [(0, vetoed), (0xffff, taken)].into_iter().enumerate() {
                let (dest, upper, expected_low, expected_high) = match (route, table) {
                    (0, 0) => ("cell_100", "cell_101", 5, 4),
                    (0, _) => ("cell_100", "cell_101", 0xffeb, 6),
                    _ => ("cell_8100", "cell_8101", 0xffeb, 0xffff),
                };
                writeln!(calls,
                    "cell_100=5; cell_101=4; cell_4112=7; cell_8100=11; cell_8101=13; cell_c110=0xfffd;\n\
                     if ({name}({table:#x},0x1000,0x1000,0,0,0,0,0,0x1234,0x5678,0xffff,0x1e0,0,0,0) != {:#x} || \
                         {dest} != {expected_low:#x} || {upper} != {expected_high:#x} || \
                         cell_4112 != 7 || cell_c110 != 0xfffd) return {};",
                    results[port], 1 + 10 * route + 2 * port + trial,
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
fn carmel_compact_dual_arithmetic_preg_executes_in_emitted_c() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let binary = common::scratch_file("carmel-dual-arithmetic-preg", "bin");
    let specs = binary.with_extension("specs");
    let source = binary.with_extension("c");
    let executable = binary.with_extension("exe");
    let mut emitted = String::from(
        "#include <stdbool.h>\n#include <stdint.h>\n\
         volatile unsigned short cell_2010, cell_2011, cell_4010, cell_4020, cell_8030, \
         cell_9010, cell_c010, cell_e020;\n",
    );
    let mut calls = String::from("int main(void) {\n");
    let nop = [0x2b, 0xec, 0, 0x2b, 0xec, 0];
    let preg = [0x1d, 0xcc, 0x44, 0x08, 0xd7, 0xa6];
    let direct = [0x1d, 0xcc, 0x44, 0, 0x0f, 0x95];
    let outputs = ["A1H", "A2H", "A3H", "EST", "R1", "R2", "R3"];
    let inputs = ["A1H", "A2H", "A3H", "EST", "CECFG1", "CECFG2", "CETBL", "ECFG", "RN2", "R3"];
    let cells = [(0x2010, 0x4372), (0x2011, 0xfe56), (0x4010, 0x4372), (0x4020, 0x0271),
                 (0x8030, 0x0271), (0x9010, 0x0271), (0xc010, 0x4372), (0xe020, 0xa4c3)];
    for (row, (program, r1, r2, route, disabled, enabled_est)) in [
        (&[preg][..], 0x4010, 0x4020, 0, [0, 0xffff, 0xcdef, 0x408, 0x4011, 0x4020, 0xe021], 0x3408),
        (&[[0x01, 0xcc, 0x44, 0x08, 0xdf, 0xe6]], 0xc010, 0x8030, 0,
         [0, 0x89ab, 0xffff, 0x409, 0xc011, 0x8030, 0xe021], 0x3409),
        (&[[0x1d, 0xcc, 0x44, 0, 0x0f, 0xd5]], 0x4010, 0x9010, 2,
         [0x4372, 0x89ab, 0xcdef, 0, 0x4011, 0x9013, 0xe020], 0x3240),
        (&[[0x01, 0xcc, 0x44, 0, 0x0f, 0x95]], 0x4010, 0x9010, 2,
         [0x4372, 0x89ab, 0xcdef, 1, 0x4011, 0x9013, 0xe020], 0x3241),
        (&[[0x05, 0xcc, 0x44, 0, 0x0f, 0x95]], 0x4010, 0x9010, 2,
         [0xbc8e, 0x89ab, 0xcdef, 0x10, 0x4011, 0x9013, 0xe020], 0x3250),
        (&[preg, direct], 0x2010, 0x9010, 2,
         [0x01aa, 0xffff, 0xcdef, 0x400, 0x2012, 0x9013, 0xe021], 0x3240),
        (&[preg, nop, direct], 0x2010, 0x9010, 2,
         [0x01aa, 0xffff, 0xcdef, 0x400, 0x2012, 0x9013, 0xe021], 0x3240),
    ].into_iter().enumerate() {
        tracked_carmel_entry_specs(&specs, &[("R1", r1), ("R2", r2), ("PACFG", 0)],
                                   2 * (program.len() as u32 - 1), route);
        let mut image = program.to_vec();
        image.extend_from_slice(&[nop, nop, nop, [0x2b, 0xd8, 0, 0x2b, 0xec, 0]]);
        fs::write(&binary, image.as_flattened()).unwrap();
        for (port, register) in outputs.into_iter().enumerate() {
            let name = format!("dual_preg_{row}_{port}");
            let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
            kuna.env("KUNA_SPECS", &specs);
            kuna.arg("decompile").arg(&binary).args([
                "0", "--addr", "--raw-image", "--target", "Carmel:BE:24:default:default", "--base", "0",
                "--json", "--assert-strict", "--assert", &format!("return %{register} unsigned short"),
            ]);
            for (i, input) in inputs.into_iter().enumerate() {
                kuna.args(["--assert", &format!("param {i} %{input} unsigned short in_{}", input.to_ascii_lowercase())]);
            }
            for (address, _) in cells {
                kuna.args(["--assert", &format!("data [data,{address:#x}] volatile unsigned short cell_{address:x}")]);
            }
            let result = common::process::required_output(&mut kuna);
            let json: Value = serde_json::from_slice(&result.stdout).unwrap();
            let function = &json["functions"][0];
            assert!(function["error"].is_null(), "{function}");
            let c = function["code"].as_str().unwrap();
            assert!(!c.contains("halt_"), "{function}");
            writeln!(emitted, "{}", c.replace("sub_0", &name)).unwrap();
            for table in [0, 0xffff] {
                let expected = if register == "EST" && table != 0 { enabled_est } else { disabled[port] };
                for (address, value) in cells {
                    write!(calls, "cell_{address:x}={value:#x}; ").unwrap();
                }
                writeln!(calls,
                    "if ({name}(0x7654,0x89ab,0xcdef,0,0x1000,0x1000,{table:#x},0,3,0xe020) != {expected:#x}) return {};",
                    1 + row * outputs.len() + port,
                ).unwrap();
                for (address, value) in cells {
                    writeln!(calls, "if (cell_{address:x} != {value:#x}) return {};", 100 + row).unwrap();
                }
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
    fs::remove_dir_all(specs).unwrap();
}

#[test]
fn carmel_compact_dual_arithmetic_preg_unbound_data_stays_unlifted() {
    let binary = common::scratch_file("carmel-dual-arithmetic-preg-unbound", "bin");
    fs::write(&binary, [0x1d, 0xcc, 0x44, 0, 0x0f, 0x95]).unwrap();
    let (stdout, stderr, status) = common::run_kuna(&[
        "raw-pcode", binary.to_str().unwrap(), "0", "--raw-image", "--target",
        "Carmel:BE:24:default:default", "--base", "0", "--count", "1", "--json",
    ]);
    let doc: Value = serde_json::from_str(&stdout).unwrap_or_else(|_| panic!("{stdout}\n{stderr}"));
    assert_eq!(status, 1, "{doc}");
    assert_eq!(doc["error"]["address"], 0, "{doc}");
    assert!(doc["instructions"].as_array().unwrap().is_empty(), "{doc}");
    fs::remove_file(binary).unwrap();
}

#[test]
fn carmel_wide_minmax_address_ports_execute_in_emitted_c() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    struct Route {
        packet: [u8; 6],
        inputs: &'static [(&'static str, u16)],
        results: &'static [(&'static str, [u16; 2])],
    }
    let routes = [
        Route {
            // A positive whole source wins the signed comparison; its high
            // word, not the low word or a sign bit, reaches the result.
            packet: [0xe7, 0x3e, 0x30, 0xca, 0xe8, 0x30],
            inputs: &[("A0L", 0x09a4), ("A4L", 0xaf94), ("A4H", 0xf4c5), ("GB4", 0), ("R7", 0xc048)],
            results: &[("A0L", [0xf4c5; 2]), ("MINMAX", [0xc078; 2]), ("EST", [0x25, 0x3025])],
        },
        Route {
            packet: [0xe7, 0x3e, 0x30, 0xca, 0xf1, 0xcc],
            inputs: &[("GB3", 0x8d), ("A3L", 0x9f26), ("R7", 0xc048)],
            results: &[("GB3", [0, 0]), ("MINMAX", [0xc078; 2]), ("EST", [0x25, 0x3025])],
        },
        Route {
            packet: [0x7b, 0x85, 0x57, 0xca, 0xe2, 0x6c],
            inputs: &[("GB4", 0x80), ("A3H", 0x0def)],
            results: &[("GB4", [0x80, 0]), ("MINMAX", [0x9876, 0x8557]), ("EST", [0x25, 0x3025])],
        },
        Route {
            packet: [0x80, 0xdf, 0xea, 0xca, 0xfb, 0xde],
            inputs: &[("FF2", 0xf678), ("GB0", 0xea)],
            results: &[("FF2", [0; 2]), ("MINMAX", [0x9876; 2]), ("EST", [0x25, 0x3025])],
        },
        Route {
            packet: [0x3f, 0x26, 0x65, 0xca, 0xda, 0x65],
            inputs: &[("A4L", 0xaf94), ("A1H", 0x834c)],
            results: &[("A4L", [0xaf94, 0]), ("MINMAX", [0x9876, 0x2665]), ("EST", [0x25, 0x3024])],
        },
        Route {
            packet: [0x34, 0x07, 0x2d, 0xca, 0xf5, 0xe9],
            inputs: &[("A3L", 0x9f26), ("A3H", 0x0def), ("GB3", 0x8d), ("A2H", 0xb008),
                      ("R4", 0x48), ("PACFG", 0), ("RM4", 0), ("RL4", 0)],
            results: &[("A3L", [0x9f26; 2]), ("A3H", [0x0def; 2]), ("GB3", [0x8d; 2]),
                       ("R4", [0x75; 2]), ("MINMAX", [0x9876; 2]), ("EST", [0x25, 0x3024])],
        },
        Route {
            packet: [0xe7, 0x3e, 0x30, 0xca, 0xe8, 0x30],
            inputs: &[("A0L", 0x09a4), ("A4L", 0xaf94), ("A4H", 0xf4c5), ("GB4", 0x80), ("R7", 0xc048)],
            results: &[("A0L", [0; 2]), ("MINMAX", [0x9876; 2]), ("EST", [0x35, 0x3035])],
        },
        Route {
            packet: [0x90, 0, 1, 0xca, 0xff, 0x45],
            inputs: &[("X1", 0xba98), ("A1L", 0xecd0), ("R0", 0x40), ("PACFG", 0), ("RM0", 0), ("RL0", 0)],
            results: &[("X1", [0; 2]), ("R0", [0x42; 2]), ("MINMAX", [0x40; 2]), ("EST", [0x25, 0x3025])],
        },
    ];
    let c_type = |reg: &str| if reg.starts_with("GB") { "unsigned char" } else { "unsigned short" };
    let binary = common::scratch_file("carmel-wide-minmax-address", "bin");
    let source = binary.with_extension("c");
    let executable = binary.with_extension("exe");
    let mut emitted = String::from("#include <stdbool.h>\n#include <stdint.h>\n");
    let mut calls = String::from("int main(void) {\n");
    for (index, route) in routes.into_iter().enumerate() {
        let code = [route.packet, [0x2b, 0xd8, 0, 0x2b, 0xec, 0], [0x2b, 0xec, 0, 0x2b, 0xec, 0]];
        fs::write(&binary, code.as_flattened()).unwrap();
        let inputs = [("CETBL", 0), ("CECFG1", 0x1000), ("CECFG2", 0x1000),
                      ("EST", 0x25), ("ECFG", 0), ("MINMAX", 0x9876)]
            .into_iter().chain(route.inputs.iter().copied());
        for &(register, results) in route.results {
            let name = format!("minmax_{index}_{}", register.to_ascii_lowercase());
            let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
            kuna.arg("decompile").arg(&binary).args([
                "0", "--addr", "--raw-image", "--target", "Carmel:BE:24:default:default", "--base", "0",
                "--json", "--assert-strict", "--assert", &format!("return %{register} {}", c_type(register)),
            ]);
            for (i, (reg, _)) in inputs.clone().enumerate() {
                kuna.args(["--assert", &format!("param {i} %{reg} {} in_{}", c_type(reg), reg.to_ascii_lowercase())]);
            }
            let output = common::process::required_output(&mut kuna);
            let json: Value = serde_json::from_slice(&output.stdout).unwrap();
            let function = &json["functions"][0];
            assert!(function["error"].is_null(), "{function}");
            let code = function["code"].as_str().unwrap();
            assert!(!code.contains("halt_"), "{code}");
            writeln!(emitted, "{}", code.replace("sub_0", &name)).unwrap();
            for (enabled, expected) in results.into_iter().enumerate() {
                let values = inputs.clone().map(|(reg, value)| {
                    if reg == "CETBL" && enabled == 1 { "65535".into() } else { value.to_string() }
                }).collect::<Vec<_>>();
                writeln!(calls, "if ({name}({}) != {expected:#x}) return {};",
                    values.join(","), index * 2 + enabled + 1).unwrap();
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
fn carmel_compact_word_stores_keep_agu_and_data_lanes_separate() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let binary = common::scratch_file("carmel-compact-word-stores", "bin");
    let specs = binary.with_extension("specs");
    tracked_carmel_entry_specs(&specs, &[("R3", 0x4047)], 0, 0);
    let source = binary.with_extension("c");
    let executable = binary.with_extension("exe");
    let nop = [0x2b, 0xec, 0, 0x2b, 0xec, 0];
    let ret = [0x2b, 0xd8, 0, 0x2b, 0xec, 0];
    let inputs = ["A0H", "A1H", "CECFG1", "CECFG2", "CETBL", "EST", "ECFG", "PACFG", "RM2", "RL2"];
    let routes = [
        ([0x31, 0x52, 0x71, 0x2b, 0xec, 0], &[
            (0, 0, 0, 0x20, 0, 0x4047, 0xa007, 0xa008),
            (0x1000, 0, 0, 0x20, 0, 0x4048, 0xa007, 0xa008),
            (0, 0, 0x00ff, 0x20, 0, 0x4048, 0xff6e, 0xa008),
        ][..]),
        ([0x2b, 0xec, 0, 0x31, 0x52, 0x73], &[
            (0, 0, 0xff00, 0, 0, 0x4048, 0x2468, 0xa008),
        ][..]),
        ([0x31, 0x52, 0x70, 0x31, 0x52, 0x72], &[
            (0, 0, 0, 0x20, 0, 0x4049, 0xff6e, 0x1234),
            (0, 0, 0, 0, 0xc0, 0x4041, 0xfedc, 0x2468),
        ][..]),
    ];
    let mut code = String::from("#include <stdbool.h>\nvolatile unsigned short first, second, bound;\n");
    let mut calls = String::from("int main(void) {\n");
    let mut failure = 0;
    for (i, (packet, rows)) in routes.into_iter().enumerate() {
        fs::write(&binary, [packet, nop, nop, ret].as_flattened()).unwrap();
        let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
        kuna.env("KUNA_SPECS", &specs).arg("decompile").arg(&binary).args([
            "0", "--addr", "--raw-image", "--target", "Carmel:BE:24:default:default",
            "--base", "0", "--json", "--assert-strict", "--assert", "return %R3 unsigned short",
            "--assert", "data [data,0x4047] volatile unsigned short first",
            "--assert", "data [data,0x4048] volatile unsigned short second",
            "--assert", "data [data,0x4040] volatile unsigned short bound",
        ]);
        for (index, input) in inputs.into_iter().enumerate() {
            kuna.args(["--assert", &format!("param {index} %{input} unsigned short in_{}", input.to_ascii_lowercase())]);
        }
        let output = common::process::required_output(&mut kuna);
        let json: Value = serde_json::from_slice(&output.stdout).unwrap();
        let function = &json["functions"][0];
        assert!(function["error"].is_null(), "{function}");
        let lift = function["code"].as_str().unwrap();
        writeln!(code, "{}", lift.replace("sub_0", &format!("store_{i}"))).unwrap();
        for &(ce1, ce2, table, ecfg, pacfg, pointer, low, high) in rows {
            failure += 1;
            writeln!(calls,
                "first = 0xa007; second = 0xa008; bound = 0xa000;\n\
                 if (store_{i}(0xfedc,0x2468,{ce1},{ce2},{table},0,{ecfg},{pacfg},7,0x4040) != {pointer} || \
                     first != {low} || second != {high} || bound != 0xa000) return {failure};").unwrap();
        }
    }
    code.push_str(&calls);
    code.push_str("return 0;\n}\n");
    fs::write(&source, &code).unwrap();
    run_emitted_c(&source, &executable, &code);
    fs::remove_dir_all(specs).unwrap();
    for file in [binary, source, executable] {
        fs::remove_file(file).unwrap();
    }
}

#[test]
fn carmel_wide_displaced_moves_use_distinct_source_and_write_ports() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let binary = common::scratch_file("carmel-wide-displaced-moves", "bin");
    let specs = binary.with_extension("specs");
    tracked_carmel_entry_specs(&specs, &[("R0", 0x4320), ("R4", 0x4000), ("R7", 0x410a)], 0, 0);
    let source = binary.with_extension("c");
    let executable = binary.with_extension("exe");
    let nop = [0x2b, 0xec, 0, 0x2b, 0xec, 0];
    let ret = [0x2b, 0xd8, 0, 0x2b, 0xec, 0];
    let routes = [
        ([0xa4, 0xd7, 4, 0xc3, 0x62, 0x90], "R4", &[(0, 0x7a36, 0x3412)][..]),
        ([0xe7, 0xd4, 0xf6, 0xc3, 0x62, 0x80], "R0", &[(0, 0x8123, 0x3412)][..]),
        ([0xe7, 0xf1, 0xf6, 0xc3, 0x60, 0x20], "R7", &[(0, 0x410a, 0x8123), (0x28, 0x410a, 0xc091)][..]),
    ];
    let mut code = String::from("#include <stdbool.h>\nvolatile unsigned short positive, negative, sink, upper;\n");
    let mut calls = String::from("int main(void) {\n");
    let mut failure = 0;
    for (i, (packet, result, rows)) in routes.into_iter().enumerate() {
        fs::write(&binary, [packet, nop, nop, ret].as_flattened()).unwrap();
        let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
        kuna.env("KUNA_SPECS", &specs).arg("decompile").arg(&binary).args([
            "0", "--addr", "--raw-image", "--target", "Carmel:BE:24:default:default",
            "--base", "0", "--json", "--assert-strict", "--assert", &format!("return %{result} unsigned short"),
            "--assert", "param 0 %ECFG unsigned short mode",
            "--assert", "data [data,0x4004] volatile unsigned short positive",
            "--assert", "data [data,0x4100] volatile unsigned short negative",
            "--assert", "data [data,0x4320] volatile unsigned short sink",
            "--assert", "data [data,0x4321] volatile unsigned short upper",
        ]);
        let output = common::process::required_output(&mut kuna);
        let doc: Value = serde_json::from_slice(&output.stdout).unwrap();
        let function = &doc["functions"][0];
        assert!(function["error"].is_null(), "{function}");
        let lift = function["code"].as_str().unwrap();
        writeln!(code, "{}", lift.replace("sub_0", &format!("move_{i}"))).unwrap();
        for &(mode, returned, sink) in rows {
            failure += 1;
            writeln!(calls,
                "positive = 0x7a36; negative = 0x8123; sink = 0x3412; upper = 0xabcd;\n\
                 if (move_{i}({mode:#x}) != {returned:#x} || sink != {sink:#x} || upper != 0xabcd || \
                     positive != 0x7a36 || negative != 0x8123) return {failure};").unwrap();
        }
    }
    code.push_str(&calls);
    code.push_str("return 0;\n}\n");
    fs::write(&source, &code).unwrap();
    run_emitted_c(&source, &executable, &code);
    fs::remove_dir_all(specs).unwrap();
    for file in [binary, source, executable] {
        fs::remove_file(file).unwrap();
    }
}

#[test]
fn carmel_wide_stored_extrema_keep_the_data_and_literal_ports_distinct() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let binary = common::scratch_file("carmel-wide-stored-extrema", "bin");
    let specs = binary.with_extension("specs");
    tracked_carmel_entry_specs(&specs, &[("R7", 0xc048)], 0, 0);
    let source = binary.with_extension("c");
    let executable = binary.with_extension("exe");
    let mut emitted = String::from(
        "#include <stdbool.h>\n#include <stdint.h>\n\
         volatile unsigned short out_low, out_high, source_word, wrap_word, literal_cell;\n",
    );
    let mut calls = String::from("int main(void) {\nwrap_word = 0xcafe; literal_cell = 0x5678;\n");
    let inputs = ["A4H", "A1L", "BTR2", "RN7", "PACFG", "RM6", "RL6", "CECFG1", "CECFG2", "CETBL", "EST", "ECFG"];
    let routes = [
        ([0x7c, 0x99, 0x23, 0xc8, 0xc4, 0xf7], 0x9a12, 0xffee, 0x3010, 0xffee, 0xffff, 0xc04c, 7),
        ([0x7c, 0x99, 0x23, 0xc8, 0xc0, 0xf7], 0x2468, 0xffee, 0x3000, 0x2468, 0x9876, 0xc04c, 7),
        ([0x6f, 0x6b, 0x48, 0xc8, 0xc5, 0xf7], 0x2468, 0xffee, 0x3010, 0x6b48, 0, 0xc046, 7),
        ([0x6f, 0x99, 0x23, 0xc8, 0xc5, 0xf8], 0x2468, 0xffee, 0x3000, 0xecd0, 0xffff, 0xc046, 7),
        ([0x7c, 0xa4, 0x54, 0xc8, 0xa4, 0xf7], 0xa454, 0xa454, 0x3008, 0xa454, 0xffff, 0xc04c, 14),
        ([0x7c, 0x99, 0x23, 0xc8, 0xd4, 0xf7], 0x2468, 0xffee, 0x3000, 0xffee, 0, 0xc04c, 7),
        ([0x7c, 0x99, 0x23, 0xc8, 0xa4, 0xf7], 0x8afe, 0xffee, 0x3010, 0x8afe, 0xffff, 0xc04c, 15),
    ];
    let nop = [0x2b, 0xec, 0, 0x2b, 0xec, 0];
    let ret = [0x2b, 0xd8, 0, 0x2b, 0xec, 0];
    for (index, (packet, data, a4h, flags, low, high, address, trace)) in routes.into_iter().enumerate() {
        let trace_seed = if index == 6 { 0x8007 } else { 7 };
        fs::write(&binary, [packet, nop, nop, nop, ret].as_flattened()).unwrap();
        for (reg, expected) in [("EST", flags), ("R7", address), ("BTR2", trace)] {
            let name = format!("extrema_{index}_{}", reg.to_ascii_lowercase());
            let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
            kuna.env("KUNA_SPECS", &specs).arg("decompile").arg(&binary).args([
                "0", "--addr", "--raw-image", "--target", "Carmel:BE:24:default:default",
                "--base", "0", "--json", "--assert-strict", "--assert", &format!("return %{reg} unsigned short"),
                "--assert", "data [data,0xc048] volatile unsigned short out_low",
                "--assert", "data [data,0xc049] volatile unsigned short out_high",
                "--assert", "data [data,0xffff] volatile unsigned short source_word",
                "--assert", "data [data,0] volatile unsigned short wrap_word",
                "--assert", "data [data,0x9923] volatile unsigned short literal_cell",
            ]);
            for (i, input) in inputs.into_iter().enumerate() {
                kuna.args(["--assert", &format!("param {i} %{input} unsigned short in_{}", input.to_ascii_lowercase())]);
            }
            let output = common::process::required_output(&mut kuna);
            let json: Value = serde_json::from_slice(&output.stdout).unwrap();
            let function = &json["functions"][0];
            assert!(function["error"].is_null(), "{function}");
            let c = function["code"].as_str().unwrap();
            assert!(!c.contains("halt_"), "{function}");
            writeln!(emitted, "{}", c.replace("sub_0", &name)).unwrap();
            let failure = index + 1;
            writeln!(calls,
                "source_word = {data:#x}; out_low = 0x1234; out_high = 0x9876;\n\
                 if ({name}({a4h:#x},0xecd0,{trace_seed:#x},4,0,0,0,0x1000,0x1000,0xffff,0,0) != {expected:#x} ||\
                     out_low != {low:#x} || out_high != {high:#x} || source_word != {data:#x} ||\
                     wrap_word != 0xcafe || literal_cell != 0x5678) return {failure};").unwrap();
            let veto = match reg { "R7" => address, "BTR2" => trace_seed, _ => 0 };
            writeln!(calls,
                "out_low = 0x1234; out_high = 0x9876;\n\
                 if ({name}({a4h:#x},0xecd0,{trace_seed:#x},4,0,0,0,0x1000,0x1000,0,0,0) != {veto:#x} ||\
                     out_low != 0x1234 || out_high != 0x9876) return {failure};").unwrap();
            if index == 0 && reg == "R7" {
                writeln!(calls,
                    "if ({name}(0xffee,0xecd0,7,9,0,0,0,0x1000,0,0,0,0) != 0xc048 ||\
                         out_low != 0x1234 || out_high != 0x9876) return 7;\n\
                     if ({name}(0xffee,0xecd0,7,9,0x4000,0,0,0x1000,0x1000,0xffff,0,0) != 0xc045 ||\
                         out_low != 0xffee || out_high != 0xffff) return 8;\n\
                     if ({name}(0xffee,0xecd0,7,9,0xc000,0xc040,15,0x1000,0x1000,0xffff,0,0) != 0x10 ||\
                         out_low != 0xffee || out_high != 0xffff) return 9;").unwrap();
            }
        }
    }
    emitted.push_str(&calls);
    emitted.push_str("return 0;\n}\n");
    fs::write(&source, &emitted).unwrap();
    run_emitted_c(&source, &executable, &emitted);
    fs::remove_dir_all(specs).unwrap();
    for file in [binary, source, executable] {
        fs::remove_file(file).unwrap();
    }
}

#[test]
fn carmel_compact_indexed_word_load_keeps_address_and_result_qualification_separate() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let binary = common::scratch_file("carmel-compact-indexed-word", "bin");
    let specs = binary.with_extension("specs");
    tracked_carmel_entry_specs(&specs, &[("R0", 0x4000)], 0, 0);
    let source = binary.with_extension("c");
    let executable = binary.with_extension("exe");
    let nop = [0x2b, 0xec, 0, 0x2b, 0xec, 0];
    let ret = [0x2b, 0xd8, 0, 0x2b, 0xec, 0];
    let inputs = [
        ("A0L", "unsigned short"), ("A0H", "unsigned short"), ("GB0", "unsigned char"),
        ("EST", "unsigned short"), ("CECFG1", "unsigned short"), ("CECFG2", "unsigned short"),
        ("CETBL", "unsigned short"), ("RN0", "unsigned short"), ("PACFG", "unsigned short"),
        ("RM0", "unsigned short"), ("RL0", "unsigned short"),
    ];
    let routes = [
        ([0x31, 0xdc, 0x10, 0x2b, 0xec, 0], &[
            (0, 0xffff, 0x8123, [0x2000, 0x8123, 0xffff, 0xff, 0x3010]),
        ][..]),
        ([0x31, 0xdc, 0x11, 0x2b, 0xec, 0], &[
            (0, 0, 0x8123, [0x4000, 0x4321, 0x8765, 0x12, 0]),
            (0x1000, 0, 0x8123, [0x2000, 0x4321, 0x8765, 0x12, 0]),
        ][..]),
        ([0x2b, 0xec, 0, 0x31, 0xdc, 0x10], &[
            (0, 0xffff, 0, [0x2000, 0, 0, 0, 0x3200]),
        ][..]),
    ];
    let mut code = String::from("#include <stdbool.h>\n#include <stdint.h>\nvolatile unsigned short cell;\n");
    let mut calls = String::from("int main(void) {\n");
    let mut failure = 0;
    for (i, (packet, rows)) in routes.into_iter().enumerate() {
        fs::write(&binary, [packet, nop, nop, ret].as_flattened()).unwrap();
        for (port, reg) in ["R0", "A0L", "A0H", "GB0", "EST"].into_iter().enumerate() {
            let kind = if reg == "GB0" { "unsigned char" } else { "unsigned short" };
            let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
            kuna.env("KUNA_SPECS", &specs).arg("decompile").arg(&binary).args([
                "0", "--addr", "--raw-image", "--target", "Carmel:BE:24:default:default",
                "--base", "0", "--json", "--assert-strict", "--assert", &format!("return %{reg} {kind}"),
                "--assert", "data [data,0x4000] volatile unsigned short cell",
            ]);
            for (j, (input, ty)) in inputs.into_iter().enumerate() {
                kuna.args(["--assert", &format!("param {j} %{input} {ty} in_{}", input.to_ascii_lowercase())]);
            }
            let output = common::process::required_output(&mut kuna);
            let doc: Value = serde_json::from_slice(&output.stdout).unwrap();
            let function = &doc["functions"][0];
            assert!(function["error"].is_null(), "{function}");
            let lift = function["code"].as_str().unwrap();
            let name = format!("load_{i}_{port}");
            writeln!(code, "{}", lift.replace("sub_0", &name)).unwrap();
            for &(ce1, table, word, result) in rows {
                failure += 1;
                writeln!(calls,
                    "cell = {word:#x}; if ({name}(0x4321,0x8765,0x12,0,{ce1:#x},0,{table:#x},0x4000,1,7,0x4000) \
                     != {:#x} || cell != {word:#x}) return {failure};", result[port]).unwrap();
            }
        }
    }
    code.push_str(&calls);
    code.push_str("return 0;\n}\n");
    fs::write(&source, &code).unwrap();
    run_emitted_c(&source, &executable, &code);
    fs::remove_dir_all(specs).unwrap();
    for file in [binary, source, executable] {
        fs::remove_file(file).unwrap();
    }
}

#[test]
fn carmel_wide_adjust_immediates_align_the_literal_and_keep_40_bit_status() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let binary = common::scratch_file("carmel-wide-fractional-immediate", "bin");
    let source = binary.with_extension("c");
    let executable = binary.with_extension("exe");
    let mut emitted = String::from(
        "#include <stdbool.h>\n#include <stdint.h>\n\
         #define CONCAT22(h,l) (((uint32_t)(uint16_t)(h) << 16) | (uint16_t)(l))\n",
    );
    let mut calls = String::from("int main(void) {\n");
    let inputs = [
        ("A0L", "unsigned short"), ("A0H", "unsigned short"), ("GB0", "unsigned char"),
        ("EST", "unsigned short"), ("ECFG", "unsigned short"),
        ("CECFG1", "unsigned short"), ("CECFG2", "unsigned short"), ("CETBL", "unsigned short"),
    ];
    let cases = [
        ((0xffffu16, 0x12), &[(0u64, 0, 0xffff, 0)][..]),
        ((0xffff, 0x16), &[(0, 0, 0xffffff0000, 0x10)][..]),
        ((0xffff, 0x02), &[(0, 0, 0xffffff0001, 0x10)][..]),
        ((0xffff, 0x06), &[(0, 0, 0x10000, 0)][..]),
        ((1, 0x12), &[
            (0x7fffffff, 0, 0x80000000, 0x20),
            (0x7fffffffff, 0, 0x8000000000, 0x36),
            (0x7fffffffff, 4, 0x8000000000, 0x32),
            (0x7fffffffff, 8, 0x8000000000, 0x32),
        ][..]),
        ((1, 0x06), &[
            (0x8000000000, 0, 0x7fffff0000, 0x27),
            (0x8000000000, 4, 0x7fffff0000, 0x23),
        ][..]),
    ];
    let nop = [0x2b, 0xec, 0, 0x2b, 0xec, 0];
    let ret = [0x2b, 0xd8, 0, 0x2b, 0xec, 0];
    for (index, ((literal, opcode), rows)) in cases.into_iter().enumerate() {
        let [hi, lo] = literal.to_be_bytes();
        fs::write(&binary, [[0x8f, hi, lo, 0xc2, opcode, 0x22], nop, nop, ret].as_flattened()).unwrap();
        for (reg, c_type, shift, mask) in [
            ("A0L", "unsigned short", 0, 0xffffu64),
            ("A0H", "unsigned short", 16, 0xffff),
            ("GB0", "unsigned char", 32, 0xff),
            ("EST", "unsigned short", 0, 0xffff),
        ] {
            let name = format!("adjust_{index}_{}", reg.to_ascii_lowercase());
            let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
            kuna.arg("decompile").arg(&binary).args([
                "0", "--addr", "--raw-image", "--target", "Carmel:BE:24:default:default",
                "--base", "0", "--json", "--assert-strict", "--assert", &format!("return %{reg} {c_type}"),
            ]);
            for (i, (input, ty)) in inputs.into_iter().enumerate() {
                kuna.args(["--assert", &format!("param {i} %{input} {ty} in_{}", input.to_ascii_lowercase())]);
            }
            let output = common::process::required_output(&mut kuna);
            let json: Value = serde_json::from_slice(&output.stdout).unwrap();
            let function = &json["functions"][0];
            assert!(function["error"].is_null(), "{function}");
            let c = function["code"].as_str().unwrap();
            assert!(!c.contains("halt_"), "{function}");
            writeln!(emitted, "{}", c.replace("sub_0", &name)).unwrap();
            for &(seed, cfg, result, status) in rows {
                let low = seed & 0xffff;
                let high = seed >> 16 & 0xffff;
                let guard = seed >> 32;
                let expected = if reg == "EST" { status } else { (result >> shift) & mask };
                writeln!(calls, "if ({name}({low:#x},{high:#x},{guard:#x},0,{cfg},4096,4096,0) != {expected:#x}) return {};",
                         index + 1).unwrap();
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
fn carmel_wide_alu_aliases_keep_result_and_flag_ports_distinct() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let binary = common::scratch_file("carmel-wide-alu-aliases", "bin");
    let source = binary.with_extension("c");
    let executable = binary.with_extension("exe");
    let of_specs = [0u8, 1, 2, 3].map(|route| binary.with_extension(format!("of{route}")));
    let tracked = [STATIC_CE[0], STATIC_CE[1], ("R0", 0x100)];
    for (route, specs) in of_specs.iter().enumerate() {
        let count = if route == 0 { 2 } else { 3 };
        tracked_carmel_entry_specs(specs, &tracked[..count], 0, route as u8);
    }
    let mut emitted = String::from(
        "#include <stdbool.h>\n#include <stdint.h>\n\
         #define CONCAT22(h,l) (((uint32_t)(uint16_t)(h) << 16) | (uint16_t)(l))\n\
         volatile unsigned short in_low, in_high, out_low, out_high, absolute_word, redirected_word;\n",
    );
    let no_prefix = &[][..];
    let r0 = &[[0x80, 0x01, 0x00, 0xc6, 0x42, 0x80]][..];
    let r4 = &[[0x80, 0x00, 0x40, 0xc6, 0x42, 0x90]][..];
    let nop = [0x2b, 0xec, 0, 0x2b, 0xec, 0];
    let cmp = [0x20, 0x9a, 2, 0x2b, 0xec, 0];
    let arithmetic = ["R0", "EST", "ECFG"];
    let long_read = ["A2L", "EST", "PACFG", "RN4", "RM4", "RL4"];
    let absolute = ["A3L", "EST", "ECFG"];
    let rmw = ["A0L", "A1L", "EST", "ECFG", "PACFG", "RN0", "RM0", "RL0"];
    for (name, reg, packet, setup, compare, inputs) in [
        ("all_ones", "EST", [0xa0, 0, 0, 0x91, 0x13, 0xcd], no_prefix, false, arithmetic.as_slice()),
        ("add_ff1", "A1L", [0xa0, 0, 0, 0x91, 0x13, 0x7d], no_prefix, false, &["R0", "FF1", "ECFG"]),
        ("ff1_flags", "EST", [0xa0, 0, 0, 0x91, 0x13, 0x7d], no_prefix, false, &["R0", "FF1", "EST", "ECFG"]),
        ("xor_long_flags", "EST", [0x9c, 0xc8, 0x23, 0xea, 0x7a, 0x49], r4, false, &long_read),
        ("xor_long_word", "X1", [0x9c, 0xc8, 0x23, 0xea, 0x7a, 0x49], r4, false, &long_read),
        ("xor_store", "EST", [0x93, 1, 0x10, 0xe8, 3, 0xba], no_prefix, false, &absolute),
        ("add_acc", "EST", [0x93, 1, 0x10, 0x91, 0x11, 0xcf], no_prefix, false, &absolute),
        ("add_long", "EST", [0x93, 1, 0x10, 0x90, 0x81, 0xcd], no_prefix, false, &absolute),
        ("xor_disconnected", "EST", [0x93, 1, 0x10, 0xe8, 3, 0xba], no_prefix, false, &absolute),
        ("add_redirected", "EST", [0x93, 1, 0x10, 0x90, 0x81, 0xcd], no_prefix, false, &absolute),
        ("guard_flags", "EST", [0x91, 0x0a, 0xbc, 0x91, 8, 0x21], r0, true, &rmw),
        ("xor_rmw", "A1L", [0x91, 0x0a, 0xbc, 0xea, 0x38, 0x42], r0, true, &rmw),
        ("xor_rmw_flags", "EST", [0x91, 0x0a, 0xbc, 0xea, 0x38, 0x42], r0, true, &rmw),
        ("add_rmw_flags", "EST", [0x91, 0x0a, 0xbc, 0x91, 0x0e, 0x21], r0, true, &rmw),
    ] {
        let mut image = setup.to_vec();
        if !setup.is_empty() {
            image.extend_from_slice(&[nop; 3]);
        }
        if compare {
            image.extend_from_slice(&[cmp, nop, nop]);
        }
        image.extend_from_slice(&[packet, nop, nop, nop, [0x2b, 0xd8, 0, 0x2b, 0xec, 0]]);
        fs::write(&binary, image.as_flattened()).unwrap();
        let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
        let route = match name {
            "xor_disconnected" => 1,
            "add_acc" | "xor_store" | "add_long" => 2,
            "add_redirected" => 3,
            _ => 0,
        };
        kuna.env("KUNA_SPECS", &of_specs[route]);
        kuna.arg("decompile").arg(&binary).args([
            "0", "--addr", "--raw-image", "--target", "Carmel:BE:24:default:default",
            "--base", "0", "--json", "--assert-strict", "--assert", &format!("return %{reg} unsigned short"),
            "--assert", "data [data,0x40] volatile unsigned short in_low",
            "--assert", "data [data,0x41] volatile unsigned short in_high",
            "--assert", "data [data,0x100] volatile unsigned short out_low",
            "--assert", "data [data,0x101] volatile unsigned short out_high",
            "--assert", "data [data,0xffff] volatile unsigned short absolute_word",
            "--assert", "data [data,0] volatile unsigned short redirected_word",
        ]);
        for (i, input) in inputs.iter().enumerate() {
            kuna.args(["--assert", &format!("param {i} %{input} unsigned short in_{}", input.to_ascii_lowercase())]);
        }
        let output = common::process::required_output(&mut kuna);
        let json: Value = serde_json::from_slice(&output.stdout).unwrap();
        let function = &json["functions"][0];
        assert!(function["error"].is_null(), "{function}");
        let c = function["code"].as_str().unwrap();
        assert!(!c.contains("halt_"), "{function}");
        writeln!(emitted, "{}", c.replace("sub_0", name)).unwrap();
    }
    emitted.push_str(
        "int main(void) {\n\
         if (all_ones(3,0,0) != 1 || add_ff1(3,0x3456,0) != 0x3459) return 1;\n\
         if (all_ones(0x8003,0,0) != 0x11 || ff1_flags(0x8003,0x8001,0,0) != 0x11) return 11;\n\
         in_low = 0x5df9; in_high = 0xabcd;\n\
         if (xor_long_word(0x8002,0,0,2,0,0) != 0xddfb || xor_long_flags(0x8002,0,0,2,0,0) != 0x20) return 2;\n\
         if (in_low != 0x5df9 || in_high != 0xabcd) return 3;\n\
         in_low = 0x8002; in_high = 0;\n\
         if (xor_long_flags(0x8002,0,0,2,0,0) != 8) return 4;\n\
         in_high = 0x8000;\n\
         if (xor_long_flags(0x8002,0,0,2,0,0) != 0x20) return 5;\n\
         absolute_word = 0xf076; out_low = 0x375f; out_high = 0x2468;\n\
         if (add_acc(0x2233,0,0) != 1 || add_acc(0,0,0) != 0x10 || out_low != 0x375f || out_high != 0x2468) return 6;\n\
         if (xor_store(0xf076,0,0) != 8 || out_low != 0 || out_high != 0x2468) return 7;\n\
         if (add_long(0x2233,0,0) != 0 || out_low != 0x12a9 || out_high != 1 || absolute_word != 0xf076) return 8;\n\
         redirected_word = 0x5678;\n\
         if (xor_disconnected(0x2233,0,0) != 0 || out_low != 0x2233 || out_high != 1) return 13;\n\
         if (add_redirected(0x2233,0,0) != 0 || out_low != 0x78ab || out_high != 0x5678 || redirected_word != 0x5678) return 14;\n\
         out_low = 1;\n\
         if (guard_flags(1,0x7fff,0,0,0,0,0,0) != 0x10 || add_rmw_flags(1,0x7fff,0,0,0,0,0,0) != 0x24 || out_low != 1) return 9;\n\
         out_low = 0x8000;\n\
         if (xor_rmw(1,0x8000,0,0,0,0,0,0) != 0 || add_rmw_flags(1,0x8000,0,0,0,0,0,0) != 0x35 || out_low != 0x8000) return 10;\n\
         out_low = 0x8001;\n\
         if (guard_flags(0x8000,0x8001,0,0,0,0,0,0) != 0x10 || xor_rmw_flags(0x8000,0x8001,0,0,0,0,0,0) != 8 || out_low != 0x8001) return 12;\n\
         return 0;\n\
         }\n",
    );
    fs::write(&source, &emitted).unwrap();
    run_emitted_c(&source, &executable, &emitted);
    fs::write(&binary, [0x93, 1, 0x10, 0xe8, 3, 0xba]).unwrap();
    let (stdout, stderr, status) = common::run_kuna(&[
        "raw-pcode", binary.to_str().unwrap(), "0", "--raw-image", "--target",
        "Carmel:BE:24:default:default", "--base", "0", "--count", "1", "--json",
    ]);
    let unknown: Value = serde_json::from_str(&stdout).unwrap_or_else(|_| panic!("{stdout}\n{stderr}"));
    assert_eq!(status, 1, "{unknown}");
    assert_eq!(unknown["error"]["address"], 0, "{unknown}");
    assert!(unknown["instructions"].as_array().unwrap().is_empty(), "{unknown}");
    for specs in of_specs { fs::remove_dir_all(specs).unwrap(); }
    for file in [binary, source, executable] {
        fs::remove_file(file).unwrap();
    }
}

const STATIC_CE: [(&str, u16); 2] = [("CECFG1", 0x2000), ("CECFG2", 0x2000)];

fn tracked_carmel_entry_specs(destination: &Path, tracked: &[(&str, u16)], site: u32, route: u8) {
    let languages = Path::new("Ghidra/Processors/Carmel/data/languages");
    let installed = std::env::var_os("KUNA_SPECS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../specs"))
        .join(languages);
    let fixture = destination.join(languages);
    fs::create_dir_all(&fixture).unwrap();
    for file in ["carmel.ldefs", "carmel.cspec", "carmel.sla"] {
        fs::copy(installed.join(file), fixture.join(file)).unwrap();
    }
    let processor_spec = fs::read_to_string(installed.join("carmel.pspec")).unwrap();
    let site = 3 * site; // Processor context bounds use byte offsets, unlike Carmel CLI addresses.
    let mut entry = format!(
        "<context_data><context_set space=\"code\" first=\"{site:#x}\" last=\"{site:#x}\">\n\
         <set name=\"carmel_of_route\" val=\"{route}\"/></context_set><tracked_set space=\"code\">\n"
    );
    for (register, value) in tracked {
        writeln!(entry, "<set name=\"{register}\" val=\"{value:#x}\"/>").unwrap();
    }
    entry.push_str("</tracked_set></context_data>\n</processor_spec>");
    fs::write(fixture.join("carmel.pspec"), processor_spec.replace("</processor_spec>", &entry)).unwrap();
}

#[test]
fn carmel_wide_logic_extended_input_banks() {
    const TARGET: &str = "Carmel:BE:24:default:default";
    const BASE: &str = "0x1860";
    let binary = common::scratch_file("carmel-wide-logic-input", "bin");
    let ret = [0x2b, 0xd8, 0, 0x2b, 0xec, 0];
    let split = [0x6d, 0x21, 0x9b, 0xe9, 0x3b, 0xed];
    fs::write(&binary, [split, ret].as_flattened()).unwrap();
    for base in ["0x1000", BASE] {
        let (stdout, stderr, status) = common::run_kuna(&[
            "raw-pcode", binary.to_str().unwrap(), base, "--addr", "--raw-image", "--target", TARGET,
            "--base", base, "--count", "1", "--json",
        ]);
        let out: Value = serde_json::from_str(&stdout).unwrap_or_else(|_| panic!("{stdout}\n{stderr}"));
        assert_eq!(status, 1, "{out}");
        assert!(out["instructions"].as_array().unwrap().is_empty(), "{out}");
    }
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        fs::remove_file(binary).unwrap();
        return;
    }

    let mut c = String::from("#include <stdbool.h>\n#include <stdint.h>\n");
    let mut calls = String::from("int main(void) {\n");
    for (name, packet, output, regs, checks) in [
        ("qualified_or", [0x36, 0x74, 0x9a, 0xe2, 0xd3, 0xda], "RL4",
            &["RM4", "RL4", "EST", "CECFG1", "CECFG2", "CETBL"][..],
            &[("0x745, 0x2345, 0, 0x1000, 0x1000, 0xffff", 0x745),
              ("0x745, 0x2345, 0, 0x1000, 0x1000, 0", 0x2345)][..]),
        ("address_bank", [0x91, 0xcd, 0x6a, 0xea, 0xd3, 0x9a], "RL4",
            &["A1L", "RM4"][..], &[("0x60, 0x745", 0x725)][..]),
        ("config_bank", [0x91, 0xcd, 0x6a, 0xeb, 0x03, 0x92], "ECFG",
            &["A1L", "CETBL"][..], &[("0x60, 0xffff", 0xff9f)][..]),
    ] {
        fs::write(&binary, [packet, ret].as_flattened()).unwrap();
        let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
        kuna.arg("decompile").arg(&binary).args([
            BASE, "--addr", "--raw-image", "--target", TARGET, "--base", BASE, "--json",
            "--assert-strict", "--assert", &format!("return %{output} unsigned short"),
        ]);
        for (i, register) in regs.iter().enumerate() {
            kuna.args(["--assert", &format!("param {i} %{register} unsigned short in_{i}")]);
        }
        let run = common::process::required_output(&mut kuna);
        let json: Value = serde_json::from_slice(&run.stdout).unwrap();
        let function = &json["functions"][0];
        let code = function["code"].as_str().unwrap();
        assert!(function["error"].is_null() && !code.contains("halt_"), "{function}");
        assert!(json["assertions"].as_array().unwrap().iter().all(|a| a["status"] == "applied"), "{json}");
        writeln!(c, "#define sub_1860 logic_{name}\n{code}\n#undef sub_1860").unwrap();
        for (args, expected) in checks {
            writeln!(calls, "if (logic_{name}({args}) != {expected}) return 1;").unwrap();
        }
    }
    c.push_str(&calls);
    c.push_str("return 0;\n}\n");
    let source = binary.with_extension("c");
    let executable = binary.with_extension("exe");
    fs::write(&source, &c).unwrap();
    run_emitted_c(&source, &executable, &c);
    for file in [binary, source, executable] {
        fs::remove_file(file).unwrap();
    }
}

#[test]
fn carmel_held_out_wide_arithmetic_ports_execute_in_emitted_c() {
    const TARGET: &str = "Carmel:BE:24:default:default";
    let binary = common::scratch_file("carmel-held-out-wide-arithmetic", "bin");
    let nop = [0x2b, 0xec, 0, 0x2b, 0xec, 0];
    let ret = [0x2b, 0xd8, 0, 0x2b, 0xec, 0];
    let reverse = [0x2b, 0x1f, 0xb3, 0x86, 0xe3, 0xea];
    let postmodify = [0xd4, 0x27, 0x14, 0x84, 0x75, 0xf4];
    for (packet, base) in [(reverse, "0"), (reverse, "0x1000"), (postmodify, "0x1000")] {
        fs::write(&binary, [packet, ret].as_flattened()).unwrap();
        let (stdout, stderr, status) = common::run_kuna(&[
            "raw-pcode", binary.to_str().unwrap(), base, "--raw-image", "--target", TARGET,
            "--base", base, "--count", "1", "--json",
        ]);
        let out: Value = serde_json::from_str(&stdout).unwrap_or_else(|_| panic!("{stdout}\n{stderr}"));
        assert_eq!(status, 1, "{out}");
        assert!(out["instructions"].as_array().unwrap().is_empty(), "{out}");
    }
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        fs::remove_file(binary).unwrap();
        return;
    }

    let tracked = [("R0", 0x40), ("R1", 0x4040), ("R3", 0xc040), ("RN3", 4), ("R4", 0x48)];
    let specs = [0, 2].map(|route| binary.with_extension(format!("of{route}")));
    tracked_carmel_entry_specs(&specs[0], &tracked, 0x1000, 0);
    tracked_carmel_entry_specs(&specs[1], &tracked, 0x1000, 2);
    let mut emitted = String::from(
        "#include <stdbool.h>\n#include <stdint.h>\n\
         #define CONCAT12(h,l) (((uint32_t)(uint8_t)(h) << 16) | (uint16_t)(l))\n\
         #define CONCAT14(h,l) (((uint64_t)(uint8_t)(h) << 32) | (uint32_t)(l))\n\
         #define CONCAT22(h,l) (((uint32_t)(uint16_t)(h) << 16) | (uint16_t)(l))\n",
    );
    let mut main = String::from("int main(void) {\n");
    type Cell<'a> = (u16, &'a str, u16, [u16; 2]);
    type Vector<'a> = (&'a str, [u8; 6], bool, &'a [(&'a str, [u16; 2])], &'a [(&'a str, u16)], &'a [Cell<'a>]);
    let vectors: [Vector<'_>; 8] = [
        ("guard", [0xc1, 0x4d, 0xb5, 0x9a, 0x77, 0x75], false,
         &[("EST", [0x10, 0x3010])], &[("MINMAX", 0xff00), ("GB1", 0x80)],
         &[(0x4db5, "guard_out", 0x7777, [0xffff; 2])]),
        ("reverse", reverse, true,
         &[("EST", [0, 0x3010]), ("R1", [0x3ff3; 2])], &[("PACFG", 0), ("RM0", 0), ("RL0", 0)],
         &[(0x4040, "reverse_low", 0xc460, [0xc460, 0xe5a4]), (0x4041, "reverse_high", 0x6385, [0x6385, 0xfffd]),
           (0xc044, "reverse_input", 0x1b5b, [0x1b5b; 2]), (0, "reverse_zero", 2, [2; 2]),
           (0xffff, "reverse_ffff", 0x100, [0x100; 2])]),
        ("stack", [0xb0, 0x27, 0xc3, 0x97, 0x77, 0xcb], false,
         &[("EST", [0, 0x3000]), ("R6", [10; 2])], &[("RL6", 0xfff9), ("RL0", 3)], &[]),
        ("modulo", [0xdf, 0x92, 0x0c, 0x9b, 0x6f, 0xdd], false,
         &[("EST", [0, 0x3000]), ("RM6", [0x5555; 2])], &[("FF1", 0x3333), ("X2", 0x2222)], &[]),
        ("post", postmodify, true,
         &[("EST", [1, 0x3001]), ("R4", [0x49; 2])], &[("PACFG", 0), ("RN4", 1), ("RM4", 0), ("RL4", 0)],
         &[(0x2714, "post_out", 0, [0x5cf8; 2]), (0x48, "post_input", 0x5df9, [0x5df9; 2]),
           (0xffff, "post_ffff", 0x100, [0x100; 2])]),
        ("long_sub", [0xca, 0x06, 0xe0, 0x86, 0xb8, 0x81], false,
         &[("EST", [0x34, 0x3034]), ("R4", [0x46; 2])],
         &[("A2L", 0x4b74), ("A2H", 0xb008), ("GB2", 0x87), ("PACFG", 0), ("RN4", 1), ("RM4", 0), ("RL4", 0)],
         &[(0x40, "long_input_low", 0xa258, [0xa258; 2]), (0x41, "long_input_high", 0xecd6, [0xecd6; 2]),
           (0x48, "long_output_low", 0x5df9, [0xa91b; 2]), (0x49, "long_output_high", 0xebe7, [0xc331; 2])]),
        ("status_sink", [0xaf, 0x74, 0x44, 0x81, 0xaa, 0x61], false,
         &[("EST", [0, 0x3000]), ("R0", [0x84; 2]), ("PSTATUS", [0x10; 2])],
         &[("PSTATUS", 0x10), ("PACFG", 0), ("RM0", 0), ("RL0", 0)],
         &[(0x40, "sink_input", 0xa258, [0xa258; 2])]),
        ("mac_control", [0x83, 0x76, 0xe4, 0xac, 0xd6, 0xf8], false,
         &[("EST", [0, 0x3000]), ("X1", [3; 2]), ("INTBASE", [0x41; 2])], &[("RF4", 3), ("INTBASE", 0x42)],
         &[(0xc040, "mac_input", 0x786, [0x786; 2])]),
    ];
    let mut check = 0;
    for (label, packet, cold, outputs, extra_inputs, cells) in vectors {
        let mut image = vec![packet, nop, nop, nop];
        if label == "stack" {
            image.push([0x80, 0, 0, 0xc0, 0x76, 0x98]); // TRANSFER R6,SP
        }
        image.extend([[0x80, 0x41, 0, 0xc6, 0x42, 0x9c], // MOVE R7,0x4100
                      [0x80, 0, 0, 0xc0, 0x4e, 0xec], // TRANSFER SP,R7
                      ret]);
        fs::write(&binary, image.as_flattened()).unwrap();
        for (_, name, _, _) in cells {
            writeln!(emitted, "volatile unsigned short {name};").unwrap();
        }
        let mut inputs = vec![("EST", 0), ("ECFG", 0), ("CECFG1", 0x1000), ("CECFG2", 0x1000), ("CETBL", 0)];
        inputs.extend_from_slice(extra_inputs);
        for (output, expected) in outputs {
            let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
            kuna.env("KUNA_SPECS", &specs[usize::from(cold)]);
            kuna.arg("decompile").arg(&binary).args([
                "0x1000", "--addr", "--raw-image", "--target", TARGET, "--base", "0x1000",
                "--json", "--assert-strict", "--assert", &format!("return %{output} unsigned short"),
            ]);
            for (i, (reg, _)) in inputs.iter().enumerate() {
                let typ = if reg.starts_with("GB") { "unsigned char" } else { "unsigned short" };
                kuna.args(["--assert", &format!("param {i} %{reg} {typ} in_{}", reg.to_ascii_lowercase())]);
            }
            for (address, name, _, _) in cells {
                kuna.args(["--assert", &format!("data [data,{address:#x}] volatile unsigned short {name}")]);
            }
            let result = common::process::required_output(&mut kuna);
            let json: Value = serde_json::from_slice(&result.stdout).unwrap();
            let function = &json["functions"][0];
            assert!(function["error"].is_null(), "{function}");
            let c = function["code"].as_str().unwrap();
            assert!(!c.contains("halt_"), "{function}");
            let name = format!("wide_{label}_{}", output.to_ascii_lowercase());
            writeln!(emitted, "{}", c.replace("sub_1000", &name)).unwrap();
            for (predicate, table) in [0, 0xffff].into_iter().enumerate() {
                for (_, cell, before, _) in cells {
                    writeln!(main, "{cell} = {before};").unwrap();
                }
                let args = inputs.iter().map(|(reg, value)| if *reg == "CETBL" { table } else { *value })
                    .map(|value| value.to_string()).collect::<Vec<_>>().join(",");
                write!(main, "if ({name}({args}) != {}", expected[predicate]).unwrap();
                for (_, cell, _, after) in cells {
                    write!(main, " || {cell} != {}", after[predicate]).unwrap();
                }
                check += 1;
                writeln!(main, ") return {check};").unwrap();
            }
        }
    }
    emitted.push_str(&main);
    emitted.push_str("return 0;\n}\n");
    let source = binary.with_extension("c");
    let executable = binary.with_extension("exe");
    fs::write(&source, &emitted).unwrap();
    run_emitted_c(&source, &executable, &emitted);
    for file in [binary, source, executable] {
        fs::remove_file(file).unwrap();
    }
    for fixture in specs {
        fs::remove_dir_all(fixture).unwrap();
    }
}

#[test]
fn carmel_wide_paired_pop_fallback_ports() {
    use kuna_console::engine::Bootstrap;

    const TARGET: &str = "Carmel:BE:24:default:default";
    let binary = common::scratch_file("carmel-wide-paired-pop", "bin");
    let return_fetch = [0x2b, 0xd8, 0, 0x2b, 0xec, 0];
    let specs = [Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../../specs")
        .display()
        .to_string()];
    for stop in [
        [0x90, 0x45, 0x69, 0xca, 0x20, 0xae],
        [0xe8, 0x63, 0x4f, 0xca, 0x29, 0x30],
    ] {
        fs::write(&binary, [stop, return_fetch].as_flattened()).unwrap();
        let mut program = Bootstrap::new(&specs)
            .target(TARGET)
            .raw(binary.to_str().unwrap(), 0x1000, &[0x1000])
            .unwrap();
        program.commit_pending_analysis().unwrap();
        assert!(program.raw_pcode_at(0x3000).is_err(), "{stop:02x?}");
    }
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        fs::remove_file(binary).unwrap();
        return;
    }

    let fixture_specs = binary.with_extension("specs");
    tracked_carmel_entry_specs(&fixture_specs, &[
        ("SP", 0x4040), ("R3", 0xc040), ("R7", 0xc048),
        ("RL6", 0x8150), ("A0L", 0x09a4), ("R0", 0x0040),
    ], 0x1000, 2);

    let source = binary.with_extension("c");
    let executable = binary.with_extension("exe");
    let mut emitted = String::from("#include <stdbool.h>\n\
        volatile unsigned short source_word, latch_word, stack_word, low_word, high_word, wrap_word;\n");
    let mut main = String::from("int main(void) {\n");
    for (row, (packet, output, expected, low, high)) in [
        ([0xe8, 0x63, 0x0f, 0xca, 0x29, 0x30], "R7", 0xc04c, 0xf123, 0x217c),
        ([0xe8, 0x23, 0x4f, 0xca, 0x29, 0x30], "R6", 0x4042, 0xf123, 0xffff),
        ([0xf6, 0x9e, 0x73, 0xca, 0x0e, 0x87], "X1", 0x0786, 0xf123, 0xffff),
        ([0xb6, 0xa3, 0xfc, 0xca, 0x31, 0xb6], "RN4", 0x6745, 0xaacc, 0x217c),
        ([0xb6, 0xb4, 0xe3, 0xca, 0x31, 0xb6], "RM2", 0x0745, 0xaacc, 0x217c),
    ].into_iter().enumerate() {
        let code = [
            packet,
            [0x80, 0, 0, 0xc0, 0x76, 0x98],       // TRANSFER R6,SP
            [0x80, 0x41, 0, 0xc6, 0x42, 0x80],   // MOVE R0,0x4100
            [0x80, 0, 0, 0xc0, 0x40, 0xec],       // TRANSFER SP,R0
            return_fetch,
        ];
        fs::write(&binary, code.as_flattened()).unwrap();
        let name = format!("pop_port_{row}");
        let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
        kuna.env("KUNA_SPECS", &fixture_specs);
        kuna.arg("decompile").arg(&binary).args([
            "0x1000", "--addr", "--raw-image", "--target", TARGET,
            "--base", "0x1000", "--json", "--assert-strict",
            "--assert", &format!("return %{output} unsigned short"),
            "--assert", "data [data,0x8150] volatile unsigned short source_word",
            "--assert", "data [data,0xc040] volatile unsigned short latch_word",
            "--assert", "data [data,0x4041] volatile unsigned short stack_word",
            "--assert", "data [data,0xffff] volatile unsigned short low_word",
            "--assert", "data [data,0x09a4] volatile unsigned short high_word",
            "--assert", "data [data,0] volatile unsigned short wrap_word",
        ]);
        for (i, input) in ["CECFG1", "CECFG2", "CETBL", "EST", "ECFG", "RN7", "PACFG", "RM6", "RM2", "RL2"]
            .into_iter().enumerate()
        {
            kuna.args(["--assert", &format!("param {i} %{input} unsigned short in_{}", input.to_ascii_lowercase())]);
        }
        let result = common::process::required_output(&mut kuna);
        let json: Value = serde_json::from_slice(&result.stdout).unwrap();
        let function = &json["functions"][0];
        assert!(function["error"].is_null(), "{function}");
        let c = function["code"].as_str().unwrap();
        assert!(!c.contains("halt_"), "{function}");
        writeln!(emitted, "{}", c.replace("sub_1000", &name)).unwrap();
        for (table, predicate) in [(0, 0), (0xffff, 1)] {
            writeln!(main,
                "source_word = 0xf123; latch_word = 0x0786; stack_word = 0x6745;\n\
                 low_word = 0xaacc; high_word = 0x217c; wrap_word = 0x5a5a;\n\
                 if ({name}(0x1000,0x1000,{table},0,0,4,0,0,0,0) != {expected} || low_word != {low} || high_word != {high}\n\
                     || source_word != 0xf123 || latch_word != 0x0786 || stack_word != 0x6745 || wrap_word != 0x5a5a) return {};",
                2 * row + predicate + 1,
            ).unwrap();
        }
    }
    emitted.push_str(&main);
    emitted.push_str("return 0;\n}\n");
    fs::write(&source, &emitted).unwrap();
    run_emitted_c(&source, &executable, &emitted);
    for file in [binary, source, executable] {
        fs::remove_file(file).unwrap();
    }
    fs::remove_dir_all(fixture_specs).unwrap();
}

#[test]
fn carmel_single_wide_pop_routes_data_and_falls_through() {
    use kuna_console::engine::Bootstrap;

    const TARGET: &str = "Carmel:BE:24:default:default";
    let binary = common::scratch_file("carmel-single-wide-pop", "bin");
    let source = binary.with_extension("c");
    let executable = binary.with_extension("exe");
    let fixture_specs = binary.with_extension("specs");
    let installed = [std::env::var("KUNA_SPECS").unwrap_or_else(|_| {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../specs").display().to_string()
    })];
    let cells = [
        (0, 0x5a5a), (0x40f1, 0x0765), (0x40f2, 0xfc34), (0x5002, 0x1a99), (0x5003, 0x2b22),
        (0x6004, 0x3c33), (0x9032, 0x4e44), (0x9040, 0x8401), (0x9041, 0x8202), (0xffff, 0x5f55),
    ];
    let rows = [
        (0x17, 0x38, "ifexe(ce1) pop(*r7++);", [0xffff, 0]),
        (0x57, 0x38, "ifexe(ce2) pop(*r7++);", [0x5002, 0]),
        (0x17, 0x39, "ifexe(ce1) pop(*(long*)r7++);", [0xffff, 0x5002]),
        (0x57, 0x39, "ifexe(ce2) pop(*(long*)r7++);", [0x5002, 0x6004]),
        (0x97, 0x39, "pop(*(long*)r7++);", [0x9040, 0x9041]),
    ];
    let cc = common::process::optional_output(Command::new("cc").arg("--version")).is_some();
    if cc {
        tracked_carmel_entry_specs(&fixture_specs, &[
            ("SP", 0x40f0), ("R7", 0x9040), ("RN7", 23), ("A0L", 0x5002), ("A1L", 0x6004),
            ("PACFG", 0xc000), ("RM6", 15), ("RL6", 0x9032),
        ], 0x1000, 2);
    }
    let mut emitted = String::from("#include <stdbool.h>\n#include <stdint.h>\n");
    for (address, _) in cells {
        writeln!(emitted, "volatile unsigned short word_{address:04x};").unwrap();
    }
    let mut main = String::from("int main(void) {\n");
    for (i, (prefix, opcode, listing, destinations)) in rows.into_iter().enumerate() {
        let body = [
            [prefix, 0xe4, 0x81, 0xcb, 0x2e, opcode],
            [0x80, 0, 0, 0xc0, 0x76, 0x98],       // TRANSFER R6,SP
            [0x80, 0x41, 0, 0xc6, 0x42, 0x80],   // MOVE R0,0x4100
            [0x80, 0, 0, 0xc0, 0x40, 0xec],       // TRANSFER SP,R0
            [0x2b, 0xd8, 0, 0x2b, 0xec, 0],       // RETURN
        ];
        fs::write(&binary, body.as_flattened()).unwrap();
        let (stdout, stderr, status) = common::run_kuna(&[
            "disassemble", binary.to_str().unwrap(), "0x1000", "--raw-image", "--target", TARGET,
            "--base", "0x1000", "--count", "2", "--json",
        ]);
        assert_eq!(status, 0, "{stdout}\n{stderr}");
        let listing_json: Value = serde_json::from_str(&stdout).unwrap();
        assert_eq!(listing_json["instructions"][0]["text"], listing);
        assert_eq!(listing_json["instructions"][1]["address"], 0x1002);
        if prefix != 0x97 {
            let mut program = Bootstrap::new(&installed).target(TARGET)
                .raw(binary.to_str().unwrap(), 0x1000, &[0x1000]).unwrap();
            program.commit_pending_analysis().unwrap();
            assert!(program.raw_pcode_at(0x3000).is_err(), "qualified POP must not guess its route");
        }
        if !cc {
            continue;
        }
        let long = opcode == 0x39;
        let marked = prefix == 0x97;
        let input = if marked { [0x0765, 0xfc34] } else { [0x8401, 0x8202] };
        for register in ["R6", "R7", "EST"] {
            let name = format!("pop_{i}_{}", register.to_ascii_lowercase());
            let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
            kuna.env("KUNA_SPECS", &fixture_specs).arg("decompile").arg(&binary).args([
                "0x1000", "--addr", "--raw-image", "--target", TARGET,
                "--base", "0x1000", "--json", "--assert-strict",
                "--assert", &format!("return %{register} unsigned short"),
            ]);
            for (index, input) in ["CECFG1", "CECFG2", "CETBL", "EST", "ECFG"].into_iter().enumerate() {
                kuna.args(["--assert", &format!("param {index} %{input} unsigned short in_{}", input.to_ascii_lowercase())]);
            }
            for (address, _) in cells {
                kuna.args(["--assert", &format!("data [data,{address:#x}] volatile unsigned short word_{address:04x}")]);
            }
            let result = common::process::required_output(&mut kuna);
            let json: Value = serde_json::from_slice(&result.stdout).unwrap();
            let function = &json["functions"][0];
            assert!(function["error"].is_null(), "{function}");
            let c = function["code"].as_str().unwrap();
            assert!(!c.contains("halt_"), "{function}");
            writeln!(emitted, "{}", c.replace("sub_1000", &name)).unwrap();
            for (table, config, ecfg) in [(0, 0, 0x30), (0, 0x1000, 0x30),
                                         (0xffff, 0, 0), (0xffff, 0x1000, 0x30)] {
                let writes = marked || table != 0;
                let advances = writes || config != 0;
                let expected = match register {
                    "R6" => 0x40f0 + if advances { if long { 2 } else { 1 } } else { 0 },
                    "R7" => if advances { if long { 0x9032 } else { 0x9041 } } else { 0x9040 },
                    "EST" => 9 | if table != 0 { 0x3000 } else { 0 } | if writes && ecfg != 0 { 0x4000 } else { 0 },
                    _ => unreachable!(),
                };
                for (address, value) in cells {
                    writeln!(main, "word_{address:04x} = {value};").unwrap();
                }
                write!(main, "if ({name}({config},{config},{table},9,{ecfg}) != {expected}").unwrap();
                for (address, original) in cells {
                    let value = if writes && address == destinations[0] {
                        input[0]
                    } else if writes && long && address == destinations[1] {
                        input[1]
                    } else {
                        original
                    };
                    write!(main, " || word_{address:04x} != {value}").unwrap();
                }
                writeln!(main, ") return {};", i + 1).unwrap();
            }
        }
    }
    fs::remove_file(binary).unwrap();
    if cc {
        emitted.push_str(&main);
        emitted.push_str("return 0;\n}\n");
        fs::write(&source, &emitted).unwrap();
        run_emitted_c(&source, &executable, &emitted);
        fs::remove_file(source).unwrap();
        fs::remove_file(executable).unwrap();
        fs::remove_dir_all(fixture_specs).unwrap();
    }
}

#[test]
fn carmel_paired_pop_separates_address_latch_and_result_gates() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let binary = common::scratch_file("carmel-paired-pop-routes", "bin");
    let specs = binary.with_extension("specs");
    tracked_carmel_entry_specs(&specs, &[
        ("SP", 0x4040), ("R4", 0x48), ("RN4", 1), ("RL6", 0x100), ("A0L", 0x7007),
    ], 0x1000, 2);
    let cells = [
        (0x48, "old_pointer", 0x9876), (0x49, "indexed_pointer", 0x789b),
        (0x100, "fallback_source", 0x82ef), (0x4041, "stack_lower", 0x1234),
        (0x4042, "stack_upper", 0x9678), (0x7007, "high_result", 0x1a99),
        (0xffff, "low_result", 0x3c33),
    ];
    let inputs = ["CECFG1", "CECFG2", "CETBL", "EST", "ECFG", "PACFG", "RM4", "RL4", "A4L", "FF1"];
    let mut emitted = String::from("#include <stdbool.h>\n#include <stdint.h>\n");
    for (_, cell, _) in cells {
        writeln!(emitted, "volatile unsigned short {cell};").unwrap();
    }
    let mut main = String::from("int main(void) {\n");
    let mut check = 0;
    for (packet, results, fallback) in [
        ([0x52, 0x96, 0x4c, 0xca, 0x08, 0x65], &[("FF1", [0x9876, 0x9876]), ("R4", [0x49, 0x49])][..], true),
        ([0x52, 0x90, 0x2c, 0xca, 0x08, 0x65], &[("A0L", [0x7007, 0x789b]), ("A4L", [0xaf94, 0x9678]), ("EST", [0, 0x3400])][..], false),
    ] {
        fs::write(&binary, [
            packet,
            [0x80, 0x41, 0, 0xc6, 0x42, 0x80], // MOVE R0,0x4100
            [0x80, 0, 0, 0xc0, 0x40, 0xec],     // TRANSFER SP,R0
            [0x2b, 0xd8, 0, 0x2b, 0xec, 0],
        ].as_flattened()).unwrap();
        for (output, expected) in results {
            let name = format!("pop_{}", output.to_ascii_lowercase());
            let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
            kuna.env("KUNA_SPECS", &specs).arg("decompile").arg(&binary).args([
                "0x1000", "--addr", "--raw-image", "--target", "Carmel:BE:24:default:default",
                "--base", "0x1000", "--json", "--assert-strict",
                "--assert", &format!("return %{output} unsigned short"),
            ]);
            for (i, input) in inputs.into_iter().enumerate() {
                kuna.args(["--assert", &format!("param {i} %{input} unsigned short in_{}", input.to_ascii_lowercase())]);
            }
            for (address, cell, _) in cells {
                kuna.args(["--assert", &format!("data [data,{address:#x}] volatile unsigned short {cell}")]);
            }
            let result = common::process::required_output(&mut kuna);
            let json: Value = serde_json::from_slice(&result.stdout).unwrap();
            let function = &json["functions"][0];
            assert!(function["error"].is_null(), "{function}");
            let c = function["code"].as_str().unwrap();
            assert!(!c.contains("halt_"), "{function}");
            writeln!(emitted, "{}", c.replace("sub_1000", &name)).unwrap();
            for (i, table) in [0, 0xffff].into_iter().enumerate() {
                for (_, cell, before) in cells {
                    writeln!(main, "{cell} = {before};").unwrap();
                }
                write!(main, "if ({name}(0x1000,0x1000,{table},0,0,0,0,0,0xaf94,0x1234) != {}", expected[i]).unwrap();
                for (address, cell, before) in cells {
                    let after = match (fallback && i == 1, address) {
                        (true, 0xffff) => 0x82ef,
                        (true, 0x7007) => 0xffff,
                        _ => before,
                    };
                    write!(main, " || {cell} != {after}").unwrap();
                }
                check += 1;
                writeln!(main, ") return {check};").unwrap();
            }
        }
    }
    emitted.push_str(&main);
    emitted.push_str("return 0;\n}\n");
    let source = binary.with_extension("c");
    let executable = binary.with_extension("exe");
    fs::write(&source, &emitted).unwrap();
    run_emitted_c(&source, &executable, &emitted);
    for file in [binary, source, executable] {
        fs::remove_file(file).unwrap();
    }
    fs::remove_dir_all(specs).unwrap();
}

#[test]
fn carmel_mac_and_parallel_add_result_ports_execute_in_emitted_c() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    struct Route {
        packet: [u8; 6],
        results: &'static [(&'static str, [u16; 2])],
        stores: [Option<(u16, u16)>; 2],
    }
    // Exercise both CETBL settings with DATA words live in the compiled C.
    let routes = [
        Route {
            packet: [0xd2, 0x23, 0xfe, 0x13, 0x10, 0xc6],
            results: &[("A0L", [0xfd1f; 2]), ("A0H", [0x0def; 2]), ("GB0", [0x8d; 2]),
                       ("A4L", [0xf4c5; 2]), ("A4H", [0xff80; 2]), ("GB4", [0xff; 2]),
                       ("X1", [0xa25e; 2]), ("X2", [0x5678; 2]), ("R4", [0x49; 2]),
                       ("R7", [0xc04c; 2]), ("EST", [0x30, 0x3030])],
            stores: [None; 2],
        },
        Route {
            packet: [0xd1, 0x22, 0xbf, 0x51, 0xb6, 0x91],
            results: &[("A3L", [0x9f26, 0]), ("A3H", [0x0def; 2]), ("GB3", [0x8d; 2]),
                       ("A4L", [0xaf94, 0]), ("A4H", [0xf4c5, 0xf4c6]), ("GB4", [0x80; 2]),
                       ("X1", [0x5fee; 2]), ("X2", [0x5678; 2]), ("R2", [0x8043; 2]),
                       ("EST", [0, 0x3031])],
            stores: [None; 2],
        },
        Route {
            packet: [0xc3, 0x78, 0x50, 0x14, 0x11, 0x29],
            results: &[("X1", [0xc460; 2]), ("X2", [0x15d0; 2]), ("R0", [0x41; 2]),
                       ("R2", [0x8040; 2]), ("R4", [0x49; 2]), ("EST", [0, 0x3000])],
            stores: [None, Some((0x8040, 0x5e41))],
        },
        Route {
            packet: [0x51, 0x97, 0x4c, 0x5e, 0x33, 0xe1],
            results: &[("A3L", [0; 2]), ("A3H", [0x1aac; 2]), ("GB3", [0x8d; 2]),
                       ("FF1", [0x1234, 0xbffc]), ("FF2", [0xf678, 0xf05a]),
                       ("X1", [0x5df9; 2]), ("X2", [0xa25e; 2]), ("R4", [0x49; 2]),
                       ("R7", [0xc04c; 2]), ("EST", [0x30, 0x3870])],
            stores: [None; 2],
        },
        Route {
            packet: [0x94, 0xfd, 0x3c, 0x49, 0x6a, 0x21],
            results: &[("A5L", [0x24b5, 0x6900]), ("A5H", [0xd1dd, 0x04d4]), ("GB5", [0x89, 0]),
                       ("X1", [0x8040; 2]), ("X2", [0x515d; 2]), ("R6", [0x8047; 2]),
                       ("EST", [0x35, 0x3004])],
            stores: [None; 2],
        },
        Route {
            packet: [0xdc, 0x0c, 0xd6, 0xb2, 0xfd, 0xae],
            results: &[("X1", [0xf678; 2]), ("X2", [0x5678; 2]), ("R4", [0x47; 2]),
                       ("FF2", [0xf678; 2]), ("EST", [0, 0x3000])],
            stores: [None; 2],
        },
        Route {
            packet: [0x4e, 0xd2, 0x20, 0x1f, 0x7c, 0xef],
            results: &[("A1L", [0xecd0; 2]), ("A1H", [0x04d4; 2]), ("GB1", [0x6a; 2]),
                       ("X1", [0x8040; 2]), ("X2", [0x5678, 0xcfc7]), ("EST", [0, 0x3400])],
            stores: [None; 2],
        },
        Route {
            packet: [0x11, 0x7a, 0xe2, 0x4b, 0x9d, 0x34],
            results: &[("X1", [0xba98; 2]), ("X2", [0x7289; 2]), ("R1", [0x4042; 2]),
                       ("R2", [0x803f; 2]), ("R7", [0xc04c; 2]), ("EST", [1, 0x3001])],
            stores: [Some((0x4040, 0x1419)); 2],
        },
        Route {
            packet: [0x2b, 0xec, 0, 0x4e, 0xde, 0x20],
            results: &[("X1", [0xba98; 2]), ("X2", [0x04d4; 2]), ("EST", [0, 0x3000])],
            stores: [None; 2],
        },
    ];
    let data = [
        [
            0xac5f, 0x55ab, 0x6896, 0x36f3, 0xbd19, 0x9520, 0x994f, 0x59e1,
            0x0ff5, 0x58ef, 0xaebd, 0xb9ec, 0x7c52, 0x07ca, 0xb5ca, 0xbd95,
            0xf4ba, 0x396e, 0x4ab5, 0x9a7c, 0x3b25, 0xd79c, 0xdae3, 0x62cd,
            0x3cb4, 0xc633, 0x9fa7, 0xed37, 0xf3b4, 0xb134, 0xb16f, 0x4d08,
            0xa258, 0xecd6, 0xcbc4, 0x72db, 0x0745, 0xf961, 0x7343, 0xcaff,
            0x5df9, 0xebe7, 0x8301, 0x5df1, 0xd244, 0x0ce5, 0xe5eb, 0xc196,
            0x240d, 0x12ff, 0x3fac, 0x95c6, 0x57f2, 0x8cb7, 0x2d9b, 0x55a4,
            0x7e24, 0x763a, 0x1279, 0x1798, 0x5932, 0x0ee6, 0x26a9, 0x4ee3,
        ],
        [
            0x753b, 0xb9f5, 0x671e, 0x7d2a, 0xcb4b, 0x2f20, 0x93ef, 0xf4d7,
            0x038e, 0x864c, 0x93da, 0x1e09, 0xadc8, 0xc3bf, 0x5979, 0x85c9,
            0xc9af, 0xdd80, 0xe2f6, 0x7c6f, 0xce0a, 0xb80c, 0xdda4, 0x69bd,
            0xa66b, 0x632b, 0x10d7, 0x6096, 0x1af5, 0xd93c, 0x3d9d, 0x338c,
            0xc460, 0x6385, 0x59f6, 0x82c9, 0x25de, 0xd3e5, 0x86e4, 0x8540,
            0x71bb, 0x03a3, 0x5dda, 0xcbfe, 0x8389, 0x7541, 0x9a46, 0x4768,
            0x4733, 0x17e4, 0xfcf6, 0x774c, 0x8e61, 0x66e7, 0x925e, 0x2258,
            0x19fe, 0x59b4, 0xb861, 0xfbc7, 0xd6a4, 0x9734, 0x93f2, 0x421e,
        ],
        [
            0xef73, 0x481e, 0x41ca, 0xd4eb, 0xb9fb, 0xdb4b, 0x8ee4, 0x2423,
            0x2905, 0xf50c, 0xb782, 0x7ba8, 0x6a96, 0x3d63, 0xd2e1, 0x22f0,
            0xf224, 0x0149, 0x4ccf, 0xcca0, 0xe3ca, 0x49ab, 0xd634, 0xaa48,
            0xb0af, 0x4ab3, 0x24e0, 0xe329, 0x466c, 0x6bba, 0x47b6, 0xfa64,
            0x7c4b, 0x20fa, 0x1a62, 0x8be1, 0xef6e, 0x73d1, 0x5fee, 0x5d36,
            0xa1c9, 0x1b2f, 0x4176, 0xdd4f, 0x1c16, 0x2c86, 0x203b, 0xfcfe,
            0x20af, 0xc816, 0xf1e1, 0xd70d, 0x9b97, 0xfde3, 0xf007, 0xe512,
            0x3410, 0x2355, 0x858c, 0xf90c, 0xba4c, 0x1103, 0x8210, 0x8218,
        ],
        [
            0x807d, 0xdbab, 0x96df, 0x54e5, 0xdda2, 0x02b7, 0x86e4, 0xf00c,
            0x8f40, 0x2941, 0x1659, 0x65f9, 0x3828, 0xe736, 0x7c28, 0x44a1,
            0xa62e, 0x4153, 0xfe45, 0xf73d, 0x8f1b, 0x2b97, 0xd186, 0x7332,
            0x1018, 0x3085, 0xd1f3, 0x7035, 0xd49a, 0x38c7, 0xc9da, 0x17f3,
            0x0786, 0x5455, 0x5240, 0x5863, 0x1b5b, 0x0e31, 0xb4a9, 0x1aec,
            0xa25e, 0x8a5a, 0x640c, 0xdfee, 0xcfc4, 0x09c1, 0xb5dd, 0x42ff,
            0x629d, 0x6c80, 0xe3e1, 0xcea6, 0x1449, 0x0a27, 0x7fa7, 0xc841,
            0xb26d, 0x3fee, 0x1380, 0x2d95, 0xba77, 0xdea6, 0x1ed4, 0x8d7d,
        ],
    ];
    let specs = common::scratch_file("carmel-raw-mac-witness", "specs");
    let relative = Path::new("Ghidra/Processors/Carmel/data/languages");
    let installed = std::env::var_os("KUNA_SPECS").map(std::path::PathBuf::from)
        .unwrap_or_else(|| common::repo_root().join("specs")).join(relative);
    let target = specs.join(relative);
    fs::create_dir_all(&target).unwrap();
    for file in ["carmel.ldefs", "carmel.sla", "carmel.cspec"] {
        fs::copy(installed.join(file), target.join(file)).unwrap();
    }
    let mut tracked = String::from("<context_data><tracked_set space=\"code\">\n");
    // Map the known entry addresses without inserting any setup instruction.
    // DATA remains writable; no operand or result word is tracked here.
    for (register, value) in [
        ("R0", 0x40), ("R1", 0x4040), ("R2", 0x8040), ("R4", 0x48),
        ("R5", 0x4048), ("R6", 0x8048), ("R7", 0xc048),
        ("RN0", 1), ("RN1", 2), ("RN2", 3), ("RN4", 1), ("RN6", 3), ("RN7", 4),
        ("PACFG", 0), ("CECFG1", 0x1000),
    ] {
        writeln!(tracked, "<set name=\"{register}\" val=\"{value:#x}\"/>").unwrap();
    }
    tracked.push_str("</tracked_set></context_data>\n");
    let pspec = fs::read_to_string(installed.join("carmel.pspec")).unwrap();
    fs::write(target.join("carmel.pspec"),
        pspec.replace("</processor_spec>", &(tracked + "</processor_spec>"))).unwrap();

    let binary = common::scratch_file("carmel-raw-mac-witness", "bin");
    let source = binary.with_extension("c");
    let executable = binary.with_extension("exe");
    let mut emitted = String::from(
        "#include <stdbool.h>\n#include <stdint.h>\n#include <stdio.h>\n\
         #define CONCAT12(h,l) (((uint32_t)(uint8_t)(h) << 16) | (uint16_t)(l))\n\
         #define CONCAT22(h,l) (((uint32_t)(uint16_t)(h) << 16) | (uint16_t)(l))\n",
    );
    let cells = (0..4).flat_map(|bank| (0x20..0x60).map(move |word| bank * 0x4000 + word));
    for address in cells.clone() {
        writeln!(emitted, "volatile unsigned short cell_{address:04x};").unwrap();
    }
    emitted.push_str("static volatile unsigned short *const cells[] = {\n");
    for address in cells.clone() {
        write!(emitted, "&cell_{address:04x},").unwrap();
    }
    emitted.push_str("};\nstatic const unsigned short seed[] = {\n");
    for word in data.into_iter().flatten() {
        write!(emitted, "{word:#x},").unwrap();
    }
    emitted.push_str(
        "};\nstatic void reset_data(void) {\n\
         for (unsigned i = 0; i < 256; ++i) *cells[i] = seed[i];\n}\n\
         static bool check_data(unsigned index, unsigned short value) {\n\
         for (unsigned i = 0; i < 256; ++i)\n\
           if (*cells[i] != (i == index ? value : seed[i])) return false;\n\
         return true;\n}\n",
    );
    let params = [
        ("CETBL", 0), ("CECFG2", 0x1000), ("EST", 0), ("ECFG", 0),
        ("A0L", 0x09a4), ("A0H", 0x4338), ("GB0", 0xea),
        ("A1L", 0xecd0), ("A1H", 0x834c), ("GB1", 0x6a),
        ("A3L", 0x9f26), ("A3H", 0x0def), ("GB3", 0x8d),
        ("A4L", 0xaf94), ("A4H", 0xf4c5), ("GB4", 0x80),
        ("A5L", 0x24b5), ("A5H", 0xd1dd), ("GB5", 0x89),
        ("FF1", 0x1234), ("FF2", 0xf678), ("X1", 0xba98), ("X2", 0x5678),
    ];
    let reg_type = |reg: &str| if reg.starts_with("GB") { "unsigned char" } else { "unsigned short" };
    let nop = [0x2b, 0xec, 0, 0x2b, 0xec, 0];
    let mut calls = String::from("int main(void) {\n");
    for (route_id, route) in routes.into_iter().enumerate() {
        let code = [route.packet, nop, nop, nop, nop, nop, [0x2b, 0xd8, 0, 0x2b, 0xec, 0]];
        fs::write(&binary, code.as_flattened()).unwrap();
        for &(reg, results) in route.results {
            let name = format!("mac_{route_id}_{}", reg.to_ascii_lowercase());
            let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
            kuna.env("KUNA_SPECS", &specs).arg("decompile").arg(&binary).args([
                "0x1000", "--addr", "--raw-image", "--target", "Carmel:BE:24:default:default",
                "--base", "0x1000", "--json", "--assert-strict", "--assert",
                &format!("return %{reg} {}", reg_type(reg)),
            ]);
            for (index, (param, _)) in params.into_iter().enumerate() {
                kuna.args(["--assert", &format!("param {index} %{param} {} in_{}",
                    reg_type(param), param.to_ascii_lowercase())]);
            }
            for address in cells.clone() {
                kuna.args(["--assert", &format!("data [data,{address:#x}] volatile unsigned short cell_{address:04x}")]);
            }
            let output = common::process::required_output(&mut kuna);
            let json: Value = serde_json::from_slice(&output.stdout).unwrap();
            let function = &json["functions"][0];
            assert!(function["error"].is_null(), "{function}");
            assert_eq!(function["name"], "sub_1000");
            let c = function["code"].as_str().unwrap();
            assert!(!c.contains("halt_"), "{c}");
            writeln!(emitted, "{}", c.replace("sub_1000", &name)).unwrap();
            for (trial, expected) in results.into_iter().enumerate() {
                let args = params.iter().map(|&(param, value)| {
                    let value = if param == "CETBL" && trial == 1 { 0xffff } else { value };
                    format!("{value:#x}")
                }).collect::<Vec<_>>().join(",");
                let (index, value) = route.stores[trial].map_or((256, 0), |(address, value)| {
                    ((address >> 14) * 64 + (address & 0x3fff) - 0x20, value)
                });
                writeln!(calls,
                    "reset_data();\n\
                     if ({name}({args}) != {expected:#x} || !check_data({index},{value:#x})) {{\n\
                     fputs(\"{name}, predicate {trial}\\n\", stderr); return 1; }}",
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
    fs::remove_dir_all(specs).unwrap();
}

#[test]
fn carmel_wide_subtract_requires_cold_operand_fetch_and_reuses_the_data_word() {
    const TARGET: &str = "Carmel:BE:24:default:default";
    const RETURN: [u8; 6] = [0x2b, 0xd8, 0, 0x2b, 0xec, 0];
    let binary = common::scratch_file("carmel-wide-subtract-absolute", "bin");
    let packet = |tail: [u8; 2]| [0x02, 0xa2, 0x77, 0x81, tail[0], tail[1]];
    fs::write(&binary, [packet([0x1d, 0xce]), RETURN].as_flattened()).unwrap();
    let (stdout, stderr, status) = common::run_kuna(&[
        "raw-pcode", binary.to_str().unwrap(), "0", "--raw-image", "--target", TARGET,
        "--base", "0", "--count", "1", "--json",
    ]);
    let unknown: Value = serde_json::from_str(&stdout).unwrap_or_else(|_| panic!("{stdout}\n{stderr}"));
    assert_eq!(status, 1, "{unknown}");
    assert_eq!(unknown["error"]["address"], 0, "{unknown}");
    assert!(unknown["instructions"].as_array().unwrap().is_empty(), "{unknown}");
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        fs::remove_file(binary).unwrap();
        return;
    }

    let source = binary.with_extension("c");
    let executable = binary.with_extension("exe");
    let specs = binary.with_extension("specs");
    tracked_carmel_entry_specs(&specs, &[("R2", 0x8040)], 0x1000, 2);
    let mut generated = String::from(
        "#include <stdbool.h>\n#include <stdint.h>\n\
         volatile unsigned short source_word, reused_word, next_word, extension_word;\n",
    );
    let mut calls = String::from("int main(void) {\n");
    let params = [
        ("CECFG1", "unsigned short", 0x1000),
        ("CECFG2", "unsigned short", 0x1000),
        ("CETBL", "unsigned short", 0),
        ("ECFG", "unsigned short", 0),
        ("EST", "unsigned short", 0),
        ("A0L", "unsigned short", 0x09a4),
        ("A0H", "unsigned short", 0x4338),
        ("GB0", "unsigned char", 0xea),
        ("FF1", "unsigned short", 0x1234),
    ];
    let accumulator = &[("A0L", 0xe94c), ("A0H", 0xffff), ("GB0", 0xff)][..];
    for (index, (tail, store, results)) in [
        ([0x1d, 0xce], true, &[][..]),
        ([0x1d, 0xee], true, &[][..]),
        ([0x1d, 0x1e], true, &[][..]),
        ([0x11, 0xce], false, accumulator),
        ([0x11, 0x1e], false, accumulator),
        ([0x7d, 0x1e], false, &[("FF1", 0xe94c)][..]),
        ([0x83, 0x1e], false, &[][..]),
    ].into_iter().enumerate() {
        fs::write(&binary, [packet(tail), RETURN].as_flattened()).unwrap();
        for (reg, result) in [("EST", 0x3010)].into_iter().chain(results.iter().copied()) {
            let (_, ty, initial) = params.iter().copied().find(|p| p.0 == reg).unwrap();
            let name = format!("sub_{index}_{}", reg.to_ascii_lowercase());
            let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
            kuna.arg("decompile").arg(&binary).env("KUNA_SPECS", &specs).args([
                "0x1000", "--addr", "--raw-image", "--target", TARGET,
                "--base", "0x1000", "--json", "--assert-strict",
                "--assert", &format!("return %{reg} {ty}"),
                "--assert", "data [data,0xffff] volatile unsigned short source_word",
                "--assert", "data [data,0x8040] volatile unsigned short reused_word",
                "--assert", "data [data,0x8041] volatile unsigned short next_word",
                "--assert", "data [data,0xa277] volatile unsigned short extension_word",
            ]);
            for (i, (reg, ty, _)) in params.into_iter().enumerate() {
                kuna.args(["--assert", &format!("param {i} %{reg} {ty} in_{}", reg.to_ascii_lowercase())]);
            }
            let output = common::process::required_output(&mut kuna);
            let json: Value = serde_json::from_slice(&output.stdout).unwrap();
            let function = &json["functions"][0];
            assert!(function["error"].is_null(), "{function}");
            let code = function["code"].as_str().unwrap();
            assert!(!code.contains("halt_"), "{function}");
            writeln!(generated, "{}", code.replace("sub_1000", &name)).unwrap();
            for (table, edge) in [(0, false), (0xffff, false), (0xffff, true)] {
                let enabled = table != 0;
                let args = params.map(|(r, _, v)| format!("{:#x}", if r == "CETBL" { table } else { v }));
                // 32767 - (-32768) extends the word lane but fits the long lane.
                let result = if !enabled { initial } else if edge {
                    match reg {
                        "EST" if results == accumulator => 0x3000,
                        "EST" => 0x3024,
                        "A0H" | "GB0" => 0,
                        _ => 0xffff,
                    }
                } else { result };
                let (lhs, rhs) = if edge { (0x7fff, 0x8000) } else { (0x6597, 0x7c4b) };
                let data = if enabled && store { if edge { 0xffff } else { 0xe94c } } else { rhs };
                writeln!(calls,
                    "source_word = {lhs:#x}; reused_word = {rhs:#x}; next_word = 0x20fa; extension_word = 0x1133;\n\
                     if ({name}({}) != {result:#x} || reused_word != {data:#x} || source_word != {lhs:#x} ||\n\
                         next_word != 0x20fa || extension_word != 0x1133) return {};",
                    args.join(","), 1 + index * 2 + usize::from(enabled),
                ).unwrap();
            }
        }
    }
    generated.push_str(&calls);
    generated.push_str("return 0;\n}\n");
    fs::write(&source, &generated).unwrap();
    run_emitted_c(&source, &executable, &generated);
    fs::remove_dir_all(specs).unwrap();
    for file in [binary, source, executable] {
        fs::remove_file(file).unwrap();
    }
}

#[test]
fn carmel_wide_mac_data_ports_separate_latching_and_writeback() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let image = common::scratch_file("carmel-wide-mac-data", "xml");
    let specs = image.with_extension("specs");
    let source = image.with_extension("c");
    let executable = image.with_extension("exe");
    tracked_carmel_entry_specs(&specs, &[("R0", 0x100), ("R1", 0x120), ("PACFG", 0),
        ("CECFG1", 0), ("CECFG2", 0), ("CETBL", 0), ("EST", 0)], 0, 0);
    let mut c = String::from("#include <stdbool.h>\n#include <stdint.h>\n\
        #define CONCAT12(h,l) (((uint32_t)(uint8_t)(h) << 16) | (uint16_t)(l))\n\
        #define CONCAT22(h,l) (((uint32_t)(uint16_t)(h) << 16) | (uint16_t)(l))\n\
        volatile unsigned short data_r0 = 0x8001, data_r1 = 0x8000;\n");
    let mut calls = String::from("int main(void) {\n");
    for (row, (packet, bits, latch, r0, r1, flags)) in [
        ("d10cd6b28820", 0xff7fff0000_u64, 0x8001, 0x101, 0x121, 0x30),
        ("d10cd6b288a2", 0xffcc000000, 0x11, 0x100, 0x121, 0x10),
        ("c85189cb8888", 0xffcc129c00, 0x8001, 0x101, 0x120, 0x10),
        ("e89089cb8888", 0xffcc129c00, 0x11, 0x101, 0x120, 0x10),
    ].into_iter().enumerate() {
        fs::write(&image, format!(
            "<binaryimage arch=\"Carmel:BE:24:default:default\">\n\
             <bytechunk space=\"code\" offset=\"0\" readonly=\"true\">{packet}2bd8002bec00</bytechunk>\n\
             <symbol space=\"code\" offset=\"0\" name=\"wide_mac\"/>\n</binaryimage>\n"
        )).unwrap();
        for (column, (reg, expected, typ)) in [
            ("A0L", bits as u16, "unsigned short"), ("A0H", (bits >> 16) as u16, "unsigned short"),
            ("GB0", (bits >> 32) as u16, "unsigned char"), ("X1", latch, "unsigned short"),
            ("R0", r0, "unsigned short"), ("R1", r1, "unsigned short"), ("EST", flags, "unsigned short"),
        ].into_iter().enumerate() {
            let output = common::process::required_output(Command::new(env!("CARGO_BIN_EXE_kuna"))
                .env("KUNA_SPECS", &specs).arg("decompile").arg(&image)
                .args(["wide_mac", "--target", "Carmel:BE:24:default:default", "--assert-strict",
                    "--assert", "param 0 %A1L unsigned short factor", "--assert", "param 1 %A1H unsigned short high",
                    "--assert", "param 2 %GB1 unsigned char guard", "--assert", "param 3 %ECFG unsigned short mode",
                    "--assert", "param 4 %X1 unsigned short latch",
                    "--assert", "data [data,0x100] volatile unsigned short data_r0",
                    "--assert", "data [data,0x120] volatile unsigned short data_r1",
                    "--assert", &format!("return %{reg} {typ}")]));
            let emitted = String::from_utf8(output.stdout).unwrap();
            assert!(!emitted.contains("halt_") && !emitted.to_lowercase().contains("warning:"), "{emitted}");
            let name = format!("mac_{row}_{column}");
            writeln!(c, "#define wide_mac {name}\n{emitted}\n#undef wide_mac").unwrap();
            writeln!(calls, "if ({name}(0x3400,0x12,0,5,0x11) != {expected:#x}) return {};", row * 7 + column + 1)
                .unwrap();
        }
    }
    c.push_str(&calls);
    c.push_str("return 0;\n}\n");
    fs::write(&source, &c).unwrap();
    run_emitted_c(&source, &executable, &c);
    for file in [image, source, executable] {
        fs::remove_file(file).unwrap();
    }
    fs::remove_dir_all(specs).unwrap();
}

#[test]
fn carmel_wide_mixed_multiply_samples_ff_at_operand_fetch() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let scratch = tempfile::tempdir().unwrap();
    let image = scratch.path().join("ff.xml");
    let specs = scratch.path().join("specs");
    let source = scratch.path().join("ff.c");
    let executable = scratch.path().join("ff");
    tracked_carmel_entry_specs(&specs, &[("R0", 0x100), ("ECFG", 0), ("EST", 0),
        ("CECFG1", 0), ("CECFG2", 0), ("CETBL", 0xffff)], 0, 0);
    let mut c = String::from("#include <stdbool.h>\n#include <stdint.h>\n\
        #define CONCAT12(h,l) (((uint32_t)(uint8_t)(h) << 16) | (uint16_t)(l))\n\
        #define CONCAT22(h,l) (((uint32_t)(uint16_t)(h) << 16) | (uint16_t)(l))\n\
        volatile unsigned short data_r0 = 0x65;\n");
    let mut calls = String::from("int main(void) {\n");
    for (row, (prefix, ff)) in [
        ("", 0x8516_u16),
        ("8f1800f77dc5", 0x8516),
        ("8f1800f77dc52bec002bec00", 0x7123),
        ("312f802bec00", 0x65),
        ("10fa02c27a5a", 0x4117),
    ].into_iter().enumerate() {
        for (reversed, packet) in [(false, "d20cd6b288ad"), (true, "d60cd6b288a5")] {
            let product = 9 * if reversed { i64::from(ff as i16) } else { i64::from(ff) };
            fs::write(&image, format!(
                "<binaryimage arch=\"Carmel:BE:24:default:default\">\n\
                 <bytechunk space=\"code\" offset=\"0\" readonly=\"true\">{prefix}{packet}2bd8002bec00</bytechunk>\n\
                 <symbol space=\"code\" offset=\"0\" name=\"wide_ff\"/>\n</binaryimage>\n"
            )).unwrap();
            for (column, (reg, expected, typ)) in [
                ("A0L", product as u16, "unsigned short"),
                ("A0H", (product >> 16) as u16, "unsigned short"),
                ("GB0", u16::from((product >> 32) as u8), "unsigned char"),
                ("X1", if reversed { 0x11 } else { ff }, "unsigned short"),
            ].into_iter().enumerate() {
                let output = common::process::required_output(Command::new(env!("CARGO_BIN_EXE_kuna"))
                    .env("KUNA_SPECS", &specs).arg("decompile").arg(&image)
                    .args(["wide_ff", "--target", "Carmel:BE:24:default:default", "--assert-strict",
                        "--assert", "param 0 %FF1 unsigned short old_ff",
                        "--assert", "param 1 %RM0 unsigned short bounds",
                        "--assert", "param 2 %A0L unsigned short old_a0",
                        "--assert", "param 3 %A2L unsigned short factor",
                        "--assert", "param 4 %X1 unsigned short latch",
                        "--assert", "data [data,0x100] volatile unsigned short data_r0",
                        "--assert", &format!("return %{reg} {typ}")]));
                let code = String::from_utf8(output.stdout).unwrap();
                assert!(!code.contains("halt_") && !code.to_lowercase().contains("warning:"), "{code}");
                let index = row * 8 + usize::from(reversed) * 4 + column + 1;
                let name = format!("ff_{index}");
                writeln!(c, "#define wide_ff {name}\n{code}\n#undef wide_ff").unwrap();
                writeln!(calls, "if ({name}(0x8516,0x7123,0x4117,9,0x11) != {expected:#x}) return {index};")
                    .unwrap();
            }
        }
    }
    c.push_str(&calls);
    c.push_str("return 0;\n}\n");
    fs::write(&source, &c).unwrap();
    run_emitted_c(&source, &executable, &c);
}

#[test]
fn carmel_wide_mac_guard_and_control_ports_execute_in_emitted_c() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    const TARGET: &str = "Carmel:BE:24:default:default";
    const RETURN: [u8; 6] = [0x2b, 0xd8, 0, 0x2b, 0xec, 0];
    let binary = common::scratch_file("carmel-wide-mac-guard", "bin");
    let source = binary.with_extension("c");
    let executable = binary.with_extension("exe");
    let specs = binary.with_extension("specs");
    tracked_carmel_entry_specs(&specs, &[("R0", 0x40), ("R3", 0xc040), ("R6", 0x8048)], 0x1000, 0);
    let params = [
        ("A0L", 0x4000), ("A1L", 4), ("GB3", 1), ("A3L", 0x3456), ("A3H", 0x5812),
        ("GB5", 0x89), ("MINMAX", 0x200), ("EST", 0x0844), ("ECFG", 0),
        ("CECFG1", 0x1000), ("CECFG2", 0x1000), ("CETBL", 0xffff),
        ("PACFG", 0), ("X1", 0xba98), ("RL0", 0), ("RM0", 0x222), ("RL2", 0x123),
        ("RM2", 0), ("R5", 0x4000), ("RN3", 3), ("R7", 2), ("RN1", 2),
    ];
    let cells = [
        (0x40, "data40", 0x8000), (0x3f, "data3f", 0x7777),
        (0xc040, "indexed", 0x2233), (0xc043, "updated", 0x4455),
        (0x80c1, "displaced", 0x4000), (0x80af, "other_displacement", 0x2000),
        (0x703b, "absolute", 0x9000), (0x70, "absolute_high", 0x2000),
        (0x3b, "absolute_low", 0x3000),
    ];
    let c_type = |reg: &str| if reg.starts_with("GB") { "unsigned char" } else { "unsigned short" };
    let mut generated = String::from(
        "#include <stdbool.h>\n#include <stdint.h>\n\
         #define CONCAT12(h,l) (((uint32_t)(uint8_t)(h) << 16) | (uint16_t)(l))\n\
         #define CONCAT22(h,l) (((uint32_t)(uint16_t)(h) << 16) | (uint16_t)(l))\n",
    );
    for (_, name, word) in cells {
        writeln!(generated, "volatile unsigned short {name} = {word:#x};").unwrap();
    }
    let mut calls = String::from("int main(void) {\n");
    type Ports<'a> = &'a [(&'a str, u16)];
    let vectors: [([u8; 6], Ports<'_>, Ports<'_>); 9] = [
        ([0x91, 0, 0, 0xa7, 0x83, 0xa1], &[("ECFG", 1)],
         &[("GB3", 2), ("A3L", 0x3456), ("A3H", 0x5812), ("X1", 0xba98)]),
        ([0x91, 0, 0, 0xab, 0x83, 0xa1], &[("ECFG", 4), ("A0L", 0xffff), ("A1L", 0xffff)],
         &[("GB3", 0xff)]),
        ([0xd8, 0xc9, 0xc7, 0xa7, 0x85, 0xca], &[],
         &[("GB5", 0), ("R0", 0x3f), ("X1", 0x4000)]),
        ([0x7a, 0x35, 0xfc, 0xa7, 0x83, 0x9d], &[], &[("GB3", 4), ("X1", 0x200)]),
        ([0x91, 0, 0, 0xa8, 0xb1, 0xa1], &[("ECFG", 5), ("A0L", 0x8000), ("A1L", 0x8000), ("RL2", 0)],
         &[("RL2", 0x8000)]),
        ([0x91, 0, 0, 0xa8, 0xc2, 0xa1], &[("A0L", 0x8000), ("A1L", 0x7fff)], &[("CECFG1", 0xd000)]),
        ([0xf2, 0x9a, 0xd8, 0xae, 0xc0, 0x17], &[("ECFG", 0x15)],
         &[("ECFG", 0xff83), ("R3", 0xc043), ("X1", 0x2233)]),
        ([0xe7, 0x70, 0x79, 0xa0, 0xb1, 0x4d], &[("ECFG", 2), ("RL2", 0x122)],
         &[("RL2", 0x123), ("X1", 0x4000)]),
        ([0x29, 0x70, 0x3b, 0xaa, 0xb1, 0x7d], &[], &[("RL2", 0x124), ("X1", 0x9000)]),
    ];
    for (index, (packet, overrides, expected)) in vectors.into_iter().enumerate() {
        fs::write(&binary, [packet, RETURN].as_flattened()).unwrap();
        for (reg, value) in expected.iter().copied().chain([("EST", 0x3844)]) {
            let name = format!("mac_{index}_{}", reg.to_ascii_lowercase());
            let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
            kuna.arg("decompile").arg(&binary).env("KUNA_SPECS", &specs).args([
                "0x1000", "--addr", "--raw-image", "--target", TARGET,
                "--base", "0x1000", "--json", "--assert-strict",
                "--assert", &format!("return %{reg} {}", c_type(reg)),
            ]);
            for (i, (reg, _)) in params.into_iter().enumerate() {
                kuna.args(["--assert", &format!("param {i} %{reg} {} in_{}", c_type(reg), reg.to_ascii_lowercase())]);
            }
            for (address, name, _) in cells {
                kuna.args(["--assert", &format!("data [data,{address:#x}] volatile unsigned short {name}")]);
            }
            let output = common::process::required_output(&mut kuna);
            let doc: Value = serde_json::from_slice(&output.stdout).unwrap();
            let function = &doc["functions"][0];
            assert!(function["error"].is_null(), "{function}");
            assert!(function["warnings"].as_array().unwrap().iter().all(|warning| {
                matches!(warning["kind"].as_str(), Some("branch-flip" | "early-return" | "return-dupe" | "ternary"))
            }), "{function}");
            let emitted = function["code"].as_str().unwrap();
            assert!(!emitted.contains("halt_"), "{function}");
            writeln!(generated, "{}", emitted.replace("sub_1000", &name)).unwrap();
            let args = params.map(|(reg, initial)| {
                let value = overrides.iter().find(|(r, _)| *r == reg).map_or(initial, |(_, v)| *v);
                format!("{value:#x}")
            });
            writeln!(calls, "if ({name}({}) != {value:#x}) return {};", args.join(","), index + 1).unwrap();
            for (_, cell, word) in cells {
                writeln!(calls, "if ({cell} != {word:#x}) return {};", index + 1).unwrap();
            }
        }
    }
    generated.push_str(&calls);
    generated.push_str("return 0;\n}\n");
    fs::write(&source, &generated).unwrap();
    run_emitted_c(&source, &executable, &generated);
    fs::remove_dir_all(specs).unwrap();
    for file in [binary, source, executable] {
        fs::remove_file(file).unwrap();
    }
}

#[test]
fn carmel_wide_mac_flag_sink_and_fallback_word_execute_in_emitted_c() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    const TARGET: &str = "Carmel:BE:24:default:default";
    const RETURN: [u8; 6] = [0x2b, 0xd8, 0, 0x2b, 0xec, 0];
    let binary = common::scratch_file("carmel-wide-mac-fallback", "bin");
    let source = binary.with_extension("c");
    let executable = binary.with_extension("exe");
    let mut c = String::from("#include <stdbool.h>\n#include <stdint.h>\nvolatile unsigned short word;\n");
    let mut calls = String::from("int main(void) {\n");
    let ports = ["A0L", "A1L", "ECFG", "EST", "CECFG1", "CECFG2", "CETBL"];
    let rows = [
        (0xa8, 0x8e, 0, 2, 1, 0x8000, 0x8000, 0x3001),
        (0xa8, 0x8f, 0, 0, 1, 0x8000, 0xffff, 0x3010),
        (0xa8, 0x8f, 9, 0x8000, 0x8000, 0, 0x7fff, 0x3024),
        (0xa8, 0x8f, 0x21, 0x8000, 0x8000, 0, 0x4000, 0x3024),
        (0xa9, 0x8f, 5, 0x7fff, 0xffff, 0, 0xfffd, 0x3020),
        (0xac, 0x8f, 1, 0x4000, 4, 1, 0xffff, 0x3010),
        (0xa0, 0x8f, 0x22, 0x4000, 4, 0, 1, 0x3000),
    ];
    for (i, (op, dst, cfg, left, right, old, stored, est)) in rows.into_iter().enumerate() {
        let body = [[0x91, 0x4a, 0xc3, op, dst, 0xa1], RETURN];
        fs::write(&binary, body.as_flattened()).unwrap();
        let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
        kuna.arg("decompile").arg(&binary).args([
            "0x1000", "--addr", "--raw-image", "--target", TARGET,
            "--base", "0x1000", "--json", "--assert-strict",
            "--assert", "return %EST unsigned short",
            "--assert", "data [data,0xffff] volatile unsigned short word",
        ]);
        for (j, reg) in ports.iter().enumerate() {
            kuna.args(["--assert", &format!("param {j} %{reg} unsigned short in_{}", reg.to_ascii_lowercase())]);
        }
        let output = common::process::required_output(&mut kuna);
        let doc: Value = serde_json::from_slice(&output.stdout).unwrap();
        let function = &doc["functions"][0];
        let emitted = function["code"].as_str().unwrap();
        assert!(function["error"].is_null() && !emitted.contains("halt_"), "{function}");
        writeln!(c, "#define sub_1000 mac_{i}\n{emitted}\n#undef sub_1000").unwrap();
        writeln!(calls, "word = {old}; if (mac_{i}({left},{right},{cfg},0,4096,4096,65535) != {est} || word != {stored}) return {};",
                 i + 1).unwrap();
    }
    c.push_str(&calls);
    c.push_str("return 0;\n}\n");
    fs::write(&source, &c).unwrap();
    run_emitted_c(&source, &executable, &c);
    for file in [binary, source, executable] {
        fs::remove_file(file).unwrap();
    }
}

#[test]
fn carmel_wide_msu_result_mux_executes_in_emitted_c() {
    let binary = common::scratch_file("carmel-wide-msu-mux", "bin");
    let specs = binary.with_extension("specs");
    let source = binary.with_extension("c");
    let executable = binary.with_extension("exe");
    tracked_carmel_entry_specs(&specs, &[
        ("R5", 0x4048), ("PACFG", 0), ("FF1", 1), ("A0L", 1), ("A5L", 9),
        ("A5H", 2), ("GB5", 0), ("ECFG", 0), ("EST", 0), ("CECFG1", 0x1000),
        ("CECFG2", 0x1000), ("CETBL", 0xffff),
    ], 0x1000, 0);
    let rows = [
        (0x8d, "A5L", 0x6000, 8, 0x6000),
        (0x8e, "EST", 0x6000, 0x3011, 0x6000),
        (0x8f, "EST", 0, 0x3010, 0xffff),
        (0x8f, "X1", 1, 1, 0),
        (0x90, "A0L", 0x6000, 0, 0x6000),
        (0x90, "R5", 0x6000, 0x4049, 0x6000),
    ];
    let mut c = String::from("#include <stdbool.h>\n#include <stdint.h>\n\
                             volatile unsigned short input_word = 1, fallback_word;\n");
    let mut calls = String::from("int main(void) {\n");
    for (i, (dst, result, old, expected, stored)) in rows.into_iter().enumerate() {
        let body = [[0x95, 0x87, 0x65, 0xac, dst, 0xfc], [0x2b, 0xd8, 0, 0x2b, 0xec, 0]];
        fs::write(&binary, body.as_flattened()).unwrap();
        let output = common::process::required_output(Command::new(env!("CARGO_BIN_EXE_kuna"))
            .env("KUNA_SPECS", &specs).arg("decompile").arg(&binary).args([
                "0x1000", "--addr", "--raw-image", "--target", "Carmel:BE:24:default:default",
                "--base", "0x1000", "--json", "--assert-strict",
                "--assert", &format!("return %{result} unsigned short"),
                "--assert", "data [data,0x4048] volatile unsigned short input_word",
                "--assert", "data [data,0xffff] volatile unsigned short fallback_word",
            ]));
        let doc: Value = serde_json::from_slice(&output.stdout).unwrap();
        let function = &doc["functions"][0];
        let text = function["code"].as_str().unwrap();
        assert!(function["error"].is_null() && !text.contains("halt_"), "{function}");
        writeln!(c, "#define sub_1000 msu_{i}\n{text}\n#undef sub_1000").unwrap();
        writeln!(calls,
            "fallback_word = {old}; if (msu_{i}() != {expected} || fallback_word != {stored} || input_word != 1) return {};",
            i + 1).unwrap();
    }
    c.push_str(&calls);
    c.push_str("return 0;\n}\n");
    fs::write(&source, &c).unwrap();
    run_emitted_c(&source, &executable, &c);
    fs::remove_dir_all(&specs).unwrap();
    for file in [binary, source, executable] {
        fs::remove_file(file).unwrap();
    }
}

#[test]
fn carmel_compact_parallel_logic_ports_execute_in_emitted_c() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    struct Route<'a> {
        packet: [u8; 6],
        retires_with_ce: bool,
        results: &'a [(&'static str, [u16; 2])],
    }
    let a2_unchanged = [("A2L", [0x4b74; 2]), ("A2H", [0xb008; 2]), ("GB2", [0x87; 2])];
    let overlap = &[
        a2_unchanged[0], a2_unchanged[1], a2_unchanged[2],
        ("R5", [0x4049; 2]), ("R6", [0x8049; 2]), ("EST", [0; 2]),
    ];
    let routes = [
        Route { packet: [0x6a, 0xd5, 0x85, 0x69, 0xd1, 0xeb], retires_with_ce: false, results: overlap },
        Route { packet: [0x69, 0xd1, 0xeb, 0x6a, 0xd5, 0x85], retires_with_ce: false, results: overlap },
        Route { packet: [0x6a, 0xd5, 0x85, 0x69, 0x91, 0xeb], retires_with_ce: false, results: overlap },
        Route {
            packet: [0x69, 0xd1, 0xeb, 0x69, 0x91, 0xeb], retires_with_ce: false,
            results: &[a2_unchanged[0], a2_unchanged[1], a2_unchanged[2], ("R5", [0x404a; 2]), ("EST", [0; 2])],
        },
        Route {
            packet: [0x6a, 0xd5, 0x85, 0x69, 0xc1, 0xeb], retires_with_ce: true,
            results: &[("A2L", [0x4b74, 0xa1c9]), ("A2H", [0xb008, 0]), ("GB2", [0x87, 0]),
                       ("A0L", [0x09a4; 2]), ("A0H", [0x4338, 0xf3ff]), ("GB0", [0xea; 2]),
                       ("R5", [0x4049; 2]), ("R6", [0x8049; 2]), ("EST", [0, 0x3000])],
        },
        Route {
            packet: [0x6d, 0x83, 0x2d, 0x6c, 0xf9, 0xfb], retires_with_ce: false,
            results: &[("A0L", [0x09a4; 2]), ("A0H", [0x4338; 2]), ("GB0", [0xea; 2]), ("X2", [0x5678; 2]),
                       ("R4", [0x49; 2]), ("R5", [0x4047; 2]), ("R6", [0x8049; 2]), ("EST", [0; 2])],
        },
        Route {
            packet: [0x6d, 0xc3, 0x2d, 0x6c, 0xf1, 0xfb], retires_with_ce: false,
            results: &[("A0L", [0x09a4; 2]), ("A0H", [0x4338; 2]), ("GB0", [0xea; 2]), ("X2", [0x5678; 2]),
                       ("R4", [0x49; 2]), ("R5", [0x4047; 2]), ("R6", [0x8049; 2]), ("EST", [0; 2])],
        },
        Route {
            packet: [0x6d, 0x8b, 0x2d, 0x6c, 0xf9, 0xfb], retires_with_ce: true,
            results: &[("A1L", [0xecd0, 0xfc30]), ("A1H", [0x834c; 2]), ("GB1", [0x6a; 2]), ("X2", [0x5678, 0xb1f3]),
                       ("R4", [0x49; 2]), ("R5", [0x4047; 2]), ("R6", [0x8049; 2]), ("EST", [0, 0x3000])],
        },
        Route {
            packet: [0x6a, 0x3a, 0x80, 0x6d, 0x83, 0x2d], retires_with_ce: true,
            results: &[("X1", [0xafb4; 2]), ("A0L", [0x09a4, 0xfc30]), ("A0H", [0x4338; 2]), ("GB0", [0xea; 2]),
                       ("R4", [0x49; 2]), ("R6", [0x8049; 2]), ("EST", [0, 0x3000])],
        },
        Route {
            packet: [0x6a, 0x3e, 0x80, 0x12, 0x03, 0x1d], retires_with_ce: false,
            results: &[("X2", [0xafb4; 2]), ("A0L", [0x09a4; 2]), ("A0H", [0x4338; 2]),
                       ("GB0", [0xea; 2]), ("EST", [0; 2])],
        },
        Route {
            packet: [0x6a, 0x1e, 0x80, 0x52, 0x39, 0xce], retires_with_ce: true,
            results: &[("FF1", [0x20e8; 2]), ("FF2", [0xb9ad; 2]), ("X1", [0xba98; 2]),
                       ("X2", [0xa25e; 2]), ("EST", [0x400, 0x3400])],
        },
        Route {
            packet: [0x52, 0x39, 0xce, 0x6a, 0x1a, 0x80], retires_with_ce: true,
            results: &[("FF1", [0x20e8; 2]), ("FF2", [0xb9ad; 2]), ("X1", [0xa25e; 2]),
                       ("X2", [0x5678; 2]), ("EST", [0x10, 0x3010])],
        },
        Route {
            packet: [0x6a, 0xf5, 0x85, 0x6a, 0xf5, 0x85], retires_with_ce: false,
            results: &[("FF1", [0x1234; 2]), ("FF2", [0xf678; 2]), ("R6", [0x804a; 2]), ("EST", [0; 2])],
        },
    ];
    let data = [(0x0048, 0x5df9), (0x0049, 0xebe7), (0x4047, 0x8540), (0x4048, 0x71bb),
                (0x4049, 0x03a3), (0x404a, 0x5dda), (0x8048, 0xa1c9), (0x8049, 0x1b2f),
                (0x804a, 0x4176), (0xc048, 0xa25e), (0xc049, 0x8a5a)];
    let tracked = [("R2", 0x8040), ("R4", 0x48), ("R5", 0x4048), ("R6", 0x8048), ("R7", 0xc048),
                   ("RN4", 1), ("PACFG", 0), ("RM4", 0), ("RL4", 0), ("RM6", 0), ("RL6", 0),
                   ("CECFG1", 0x1000), ("CECFG2", 0x1000), ("FFG", 0)];
    let params = [("CETBL", 0), ("EST", 0), ("ECFG", 0), ("A0L", 0x09a4), ("A0H", 0x4338), ("GB0", 0xea),
                  ("A1L", 0xecd0), ("A1H", 0x834c), ("GB1", 0x6a), ("A2L", 0x4b74), ("A2H", 0xb008),
                  ("GB2", 0x87), ("A4L", 0xaf94), ("FF1", 0x1234), ("FF2", 0xf678), ("X1", 0xba98), ("X2", 0x5678)];
    let c_type = |reg: &str| if reg.starts_with("GB") { "unsigned char" } else { "unsigned short" };
    let binary = common::scratch_file("carmel-parallel-logic", "bin");
    let source = binary.with_extension("c");
    let executable = binary.with_extension("exe");
    let specs = binary.with_extension("specs");
    let relative = Path::new("Ghidra/Processors/Carmel/data/languages");
    let src = std::env::var_os("KUNA_SPECS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| common::repo_root().join("specs"))
        .join(relative);
    let dst = specs.join(relative);
    fs::create_dir_all(&dst).unwrap();
    for file in ["carmel.ldefs", "carmel.sla", "carmel.cspec"] {
        fs::copy(src.join(file), dst.join(file)).unwrap();
    }
    let mut entry = String::from("<context_data><tracked_set space=\"code\">\n");
    for (register, value) in tracked {
        writeln!(entry, "<set name=\"{register}\" val=\"{value:#x}\"/>").unwrap();
    }
    entry.push_str("</tracked_set></context_data>\n</processor_spec>");
    let pspec = fs::read_to_string(src.join("carmel.pspec")).unwrap().replace("</processor_spec>", &entry);
    fs::write(dst.join("carmel.pspec"), pspec).unwrap();
    let mut emitted = String::from(
        "#include <stdbool.h>\n#include <stdint.h>\n#include <stdlib.h>\n\
         static _Noreturn void carmel_register_write_conflict(void) { abort(); }\n",
    );
    for (addr, value) in data {
        writeln!(emitted, "volatile unsigned short cell_{addr:04x} = {value:#x};").unwrap();
    }
    let unchanged = data.map(|(addr, value)| format!("cell_{addr:04x} != {value:#x}")).join(" || ");
    let mut calls = String::from("int main(void) {\n");
    let mut cases = 0;
    for (index, route) in routes.into_iter().enumerate() {
        let nop = [0x2b, 0xec, 0, 0x2b, 0xec, 0];
        let body = [route.packet, nop, nop, nop, nop, nop, [0x2b, 0xd8, 0, 0x2b, 0xec, 0]];
        fs::write(&binary, body.as_flattened()).unwrap();
        let count = 1 + usize::from(route.retires_with_ce);
        cases += count;
        for (port, &(register, results)) in route.results.iter().enumerate() {
            let name = format!("logic_{index}_{port}");
            let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
            kuna.env("KUNA_SPECS", &specs).arg("decompile").arg(&binary).args([
                "0x1000", "--addr", "--raw-image", "--target", "Carmel:BE:24:default:default", "--base", "0x1000",
                "--json", "--assert-strict", "--assert", &format!("return %{register} {}", c_type(register)),
            ]);
            for (i, (reg, _)) in params.into_iter().enumerate() {
                kuna.args(["--assert", &format!("param {i} %{reg} {} in_{}", c_type(reg), reg.to_ascii_lowercase())]);
            }
            for (addr, _) in data {
                kuna.args(["--assert", &format!("data [data,{addr:#x}] volatile unsigned short cell_{addr:04x}")]);
            }
            let output = common::process::required_output(&mut kuna);
            let json: Value = serde_json::from_slice(&output.stdout).unwrap();
            let function = &json["functions"][0];
            assert!(function["error"].is_null(), "{function}");
            let code = function["code"].as_str().unwrap();
            assert!(!code.contains("halt_") && code.contains("sub_1000("), "{code}");
            writeln!(emitted, "{}", code.replace("sub_1000(", &format!("{name}("))).unwrap();
            for (ce, expected) in results.into_iter().take(count).enumerate() {
                let values = params.map(|(reg, value)| {
                    if reg == "CETBL" && ce == 1 { "65535".into() } else { value.to_string() }
                });
                writeln!(calls, "if ({name}({}) != {expected:#x} || {unchanged}) return {};",
                    values.join(","), 1 + index * 16 + port * 2 + ce).unwrap();
            }
        }
    }
    assert_eq!(cases, 18);
    emitted.push_str(&calls);
    emitted.push_str("return 0;\n}\n");
    fs::write(&source, &emitted).unwrap();
    run_emitted_c(&source, &executable, &emitted);

    fs::write(&binary, [0x6a, 0x1e, 0x80, 0x6a, 0x1e, 0x80]).unwrap();
    let (stdout, stderr, status) = common::run_kuna(&[
        "raw-pcode", binary.to_str().unwrap(), "0x1000", "--raw-image", "--target",
        "Carmel:BE:24:default:default", "--base", "0x1000", "--count", "1", "--json",
    ]);
    let json: Value = serde_json::from_str(&stdout).unwrap_or_else(|_| panic!("{stdout}\n{stderr}"));
    assert_eq!(status, 1, "{json}");
    assert_eq!(json["error"]["address"], 0x1000, "{json}");
    assert!(json["instructions"].as_array().unwrap().is_empty(), "{json}");
    for file in [binary, source, executable] {
        fs::remove_file(file).unwrap();
    }
    fs::remove_dir_all(specs).unwrap();
}

#[test]
fn carmel_uncommon_native_ports_execute_in_emitted_c() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    type Registers = &'static [(&'static str, u16)];
    type Cell = (&'static str, u32, u16, u16);
    struct Route {
        packet: [u8; 6],
        tracked: Registers,
        inputs: Registers,
        results: Registers,
        cells: &'static [Cell],
        bus_access: Option<(usize, usize)>,
    }
    let routes = [
        Route {
            packet: [0x80, 0x92, 0x34, 0xb9, 0x91, 0x24],
            tracked: &[],
            inputs: &[("A0L", 0x4321), ("A0H", 0x8765), ("GB0", 0xab),
                      ("A1L", 0xbbcc), ("A1H", 0xddaa), ("GB1", 0xee), ("EST", 0)],
            results: &[("A0L", 0x0300), ("A0H", 0x8520), ("GB0", 0xaa), ("EST", 0x30)],
            cells: &[], bus_access: None,
        },
        Route {
            packet: [0x80, 0, 0, 0xb8, 0xb1, 0x24],
            tracked: &[],
            inputs: &[("A0H", 0x8765), ("A1L", 0xbbcc), ("A1H", 0xddaa), ("GB1", 0xee), ("EST", 0)],
            results: &[("A0H", 0xbfed), ("EST", 0x30)],
            cells: &[], bus_access: None,
        },
        Route {
            packet: [0x80, 0, 0, 0xb8, 0x81, 0x24],
            tracked: &[],
            inputs: &[("GB0", 0x76), ("A1H", 0x23b9), ("EST", 0x21)],
            results: &[("GB0", 0xb9), ("EST", 0x21)],
            cells: &[], bus_access: None,
        },
        Route {
            packet: [0x80, 0, 0, 0xba, 0x11, 0x40],
            tracked: &[("R0", 0x1000)],
            inputs: &[("A0L", 0x4321), ("RN0", 1), ("PACFG", 0), ("RM0", 0), ("RL0", 0), ("EST", 0)],
            results: &[("R0", 0x1001), ("EST", 0)],
            cells: &[("data", 0x1000, 0x8abc, 0xc99d), ("data", 0x1001, 0xcdee, 0xcdee)],
            bus_access: None,
        },
        Route {
            packet: [0x80, 0, 0, 0xba, 0x10, 0],
            tracked: &[("R0", 0x1000)],
            inputs: &[("RN0", 1), ("PACFG", 0), ("RM0", 0), ("RL0", 0), ("EST", 0)],
            results: &[("R0", 0x1001), ("EST", 8)],
            cells: &[("data", 0x1000, 0x8abc, 0), ("data", 0x1001, 0xcdee, 0xcdee)],
            bus_access: None,
        },
        Route {
            packet: [0x70, 0x0b, 0x17, 0xc2, 0x15, 4],
            tracked: &[("R0", 0x4038)],
            inputs: &[("R1", 0x4048), ("PACFG", 0), ("RM0", 0), ("RL0", 0),
                      ("CECFG1", 0x1000), ("CECFG2", 0x1000), ("CETBL", 0), ("EST", 0)],
            results: &[("R0", 0x404f), ("EST", 0)],
            cells: &[("data", 0x4038, 0x8010, 0x8010), ("data", 0x4048, 0xbc23, 0xbc23)],
            bus_access: None,
        },
        Route {
            packet: [0x70, 0x0b, 0x17, 0xc2, 0x11, 0xc4],
            tracked: &[("R0", 0x4038), ("R1", 0x4048)],
            inputs: &[("PACFG", 0), ("RM0", 0), ("RL0", 0), ("CECFG1", 0x1000),
                      ("CECFG2", 0x1000), ("CETBL", 0xffff), ("EST", 0)],
            results: &[("R0", 0x4043), ("R1", 0x405f), ("EST", 0x3000)],
            cells: &[("data", 0x4038, 0x8010, 0x8010), ("data", 0x4048, 0xbc23, 0xbc23)],
            bus_access: None,
        },
        Route {
            packet: [0x40, 0x0b, 0x17, 0xc2, 0x16, 0x21],
            tracked: &[("R0", 0x4038)],
            inputs: &[("A0L", 0xa031), ("A0H", 0x2020), ("GB0", 0x20), ("CECFG1", 0x1000),
                      ("CECFG2", 0x1000), ("CETBL", 0xffff), ("EST", 0)],
            results: &[("A0L", 0x2041), ("A0H", 0x9f55), ("GB0", 0x20), ("EST", 0x3020)],
            cells: &[("data", 0x4038, 0x8010, 0x8010), ("data", 0x4039, 0x7f34, 0x7f34)],
            bus_access: None,
        },
        Route {
            packet: [0x4f, 0x40, 0x48, 0xc2, 0x01, 0x46],
            tracked: &[],
            inputs: &[("R1", 0x4048), ("PACFG", 0), ("RM0", 0), ("RL0", 0), ("CECFG1", 0x1000),
                      ("CECFG2", 0x1000), ("CETBL", 0xffff), ("EST", 0)],
            results: &[("R1", 0x404a), ("EST", 0x3009)],
            cells: &[("data", 0x4048, 0xbc23, 0xbc23), ("data", 0x4049, 0xc567, 0xc567)],
            bus_access: None,
        },
        Route {
            packet: [0x81, 1, 2, 0xc6, 0x46, 0x40],
            tracked: &[], inputs: &[], results: &[("A0L", 0xd615)],
            cells: &[("io", 0x102, 0x6d15, 0x6d15), ("io", 0x10102, 0xd615, 0xd615)],
            bus_access: Some((1, 1)),
        },
        Route {
            packet: [0x80, 1, 2, 0xc6, 0x44, 0x20],
            tracked: &[("R0", 0x1000)],
            inputs: &[("RN0", 1), ("PACFG", 0), ("RM0", 0), ("RL0", 0)],
            results: &[("R0", 0x1001)],
            cells: &[("io", 0x102, 0x6d15, 0x6d15), ("data", 0x1000, 0x1111, 0x6d15),
                      ("data", 0x1001, 0x2222, 0x2222)],
            bus_access: Some((0, 1)),
        },
        Route {
            packet: [0x80, 1, 2, 0xc6, 0x80, 0x20],
            tracked: &[("R0", 0x1000)],
            inputs: &[("RN0", 1), ("PACFG", 0), ("RM0", 0), ("RL0", 0)],
            results: &[("R0", 0x1001)],
            cells: &[("io", 0x102, 0x6d15, 0x1111), ("data", 0x1000, 0x1111, 0x1111),
                      ("data", 0x1001, 0x2222, 0x2222)],
            bus_access: Some((0, 1)),
        },
        Route {
            packet: [0x80, 1, 2, 0xc6, 0x47, 0x18],
            tracked: &[], inputs: &[("EST", 0x172)], results: &[("EST", 0x172)],
            cells: &[("io", 0x102, 0x6d15, 0x6d15), ("data", 0x102, 0x1111, 0x1111)],
            bus_access: Some((0, 1)),
        },
        Route {
            packet: [0x29, 0xee, 0, 0xc6, 0xbb, 0xc0],
            tracked: &[], inputs: &[("CECFG1", 0x1000), ("CECFG2", 0x1000), ("CETBL", 0x00ff), ("EST", 0x20)],
            results: &[("EST", 0x1020)],
            cells: &[("io", 0xee00, 0x6d15, 0x6d15), ("io", 0x29ee00, 0xd615, 0)],
            bus_access: Some((1, 1)),
        },
        Route {
            packet: [0x29, 0xee, 0, 0xc6, 0xbb, 0xc0],
            tracked: &[], inputs: &[("CECFG1", 0x1000), ("CECFG2", 0x1000), ("CETBL", 0xff00), ("EST", 0x20)],
            results: &[("EST", 0x2020)],
            cells: &[("io", 0xee00, 0x6d15, 0x6d15), ("io", 0x29ee00, 0xd615, 0xd615)],
            bus_access: Some((1, 0)),
        },
        Route {
            packet: [0x82, 0, 0x10, 0xcc, 0x81, 4],
            tracked: &[("R0", 0x1000)], inputs: &[("EST", 0x20)], results: &[("EST", 0x20)],
            cells: &[("io", 0x102, 0x6d15, 0x6d15), ("ext", 0x102, 0xaaaa, 0xaaaa),
                      ("data", 0x1000, 0x1111, 0x1111), ("data", 0x1010, 0x2222, 0xaaaa)],
            bus_access: Some((1, 1)),
        },
        Route {
            packet: [0x46, 0x43, 0xf8, 0xcc, 0x9d, 0x97],
            tracked: &[], inputs: &[("CECFG2", 0x1000), ("CETBL", 0), ("EST", 0x20)],
            results: &[("EST", 0x20)],
            cells: &[("ext", 0xc6, 0x1111, 0x1111), ("ext", 0x1c6, 0x2222, 0x2222),
                      ("data", 0x43f8, 0x3333, 0x3333)],
            bus_access: Some((1, 1)),
        },
        Route {
            packet: [0xc6, 0x43, 0xf8, 0xcc, 0x99, 0xff],
            tracked: &[], inputs: &[("EST", 0x20)], results: &[("EST", 0x20)],
            cells: &[("ext", 0xc6, 0x1111, 0x1111), ("ext", 0x1c6, 0x2222, 0x2222),
                      ("data", 0x43f8, 0x3333, 0x2222), ("data", 0x43f9, 0x4444, 0x4444)],
            bus_access: Some((1, 1)),
        },
    ];
    let binary = common::scratch_file("carmel-uncommon-ports", "bin");
    let source = binary.with_extension("c");
    let executable = binary.with_extension("exe");
    let mut c = String::from(
        "#include <stdbool.h>\n#include <stdint.h>\n\
         #define CONCAT12(h,l) (((uint32_t)(uint8_t)(h) << 16) | (uint16_t)(l))\n\
         #define CONCAT22(h,l) (((uint32_t)(uint16_t)(h) << 16) | (uint16_t)(l))\n\
         #define CONCAT32(h,l) (((uint64_t)((uint32_t)(h) & 0xffffffu) << 16) | (uint16_t)(l))\n\
         #define CONCAT44(h,l) (((uint64_t)(uint32_t)(h) << 32) | (uint32_t)(l))\n\
         static unsigned accesses;\n\
         static volatile unsigned short *port(volatile unsigned short *word) { ++accesses; return word; }\n",
    );
    let mut calls = String::from("int main(void) {\n");
    let mut spec_dirs = Vec::new();
    let c_type = |reg: &str| if reg.starts_with("GB") { "unsigned char" } else { "unsigned short" };
    for (row, route) in routes.into_iter().enumerate() {
        fs::write(&binary, [
            route.packet, [0x2b, 0xec, 0, 0x2b, 0xec, 0], [0x2b, 0xd8, 0, 0x2b, 0xec, 0],
        ].as_flattened()).unwrap();
        let mut tracked = Vec::from(route.tracked);
        for (name, value) in STATIC_CE {
            if !route.tracked.iter().chain(route.inputs).any(|&(reg, _)| reg == name) {
                tracked.push((name, value));
            }
        }
        let specs = binary.with_extension(format!("specs-{row}"));
        tracked_carmel_entry_specs(&specs, &tracked, 0, 0);
        for (i, _) in route.cells.iter().enumerate() {
            writeln!(c, "static volatile unsigned short backing_{row}_{i};").unwrap();
            if route.bus_access.is_some_and(|(cell, _)| cell == i) {
                writeln!(c, "#define cell_{row}_{i} (*port(&backing_{row}_{i}))").unwrap();
            } else {
                writeln!(c, "#define cell_{row}_{i} backing_{row}_{i}").unwrap();
            }
        }
        for &(register, expected) in route.results {
            let name = format!("uncommon_{row}_{}", register.to_ascii_lowercase());
            let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
            kuna.env("KUNA_SPECS", &specs);
            kuna.arg("decompile").arg(&binary).args([
                "0", "--addr", "--raw-image", "--target", "Carmel:BE:24:default:default", "--base", "0",
                "--json", "--assert-strict", "--assert", &format!("return %{register} {}", c_type(register)),
            ]);
            for (i, (reg, _)) in route.inputs.iter().enumerate() {
                kuna.args(["--assert", &format!("param {i} %{reg} {} in_{}", c_type(reg), reg.to_ascii_lowercase())]);
            }
            for (i, (space, address, _, _)) in route.cells.iter().enumerate() {
                kuna.args(["--assert", &format!("data [{space},{address:#x}] volatile unsigned short cell_{row}_{i}")]);
            }
            let output = common::process::required_output(&mut kuna);
            let json: Value = serde_json::from_slice(&output.stdout).unwrap();
            let function = &json["functions"][0];
            let text = function["code"].as_str().unwrap();
            assert!(function["error"].is_null() && !text.contains("halt_"), "{function}");
            assert!(json["assertions"].as_array().unwrap().iter().all(|a| a["status"] == "applied"), "{json}");
            writeln!(c, "#define sub_0 {name}\n{text}\n#undef sub_0").unwrap();
            for (i, (_, _, before, _)) in route.cells.iter().enumerate() {
                writeln!(calls, "backing_{row}_{i} = {before:#x};").unwrap();
            }
            let arguments = route.inputs.iter().map(|&(_, n)| n.to_string()).collect::<Vec<_>>().join(",");
            writeln!(calls, "accesses = 0;\nif ({name}({arguments}) != {expected:#x}) return {};", row + 1).unwrap();
            if let Some((_, expected_accesses)) = route.bus_access {
                writeln!(calls, "if (accesses != {expected_accesses}) return {};", row + 32).unwrap();
            }
            for (i, (_, _, _, after)) in route.cells.iter().enumerate() {
                writeln!(calls, "if (backing_{row}_{i} != {after:#x}) return {};", row + 64).unwrap();
            }
        }
        spec_dirs.push(specs);
    }
    c.push_str(&calls);
    c.push_str("return 0;\n}\n");
    fs::write(&source, &c).unwrap();
    run_emitted_c(&source, &executable, &c);
    for path in spec_dirs {
        fs::remove_dir_all(path).unwrap();
    }
    for file in [binary, source, executable] {
        fs::remove_file(file).unwrap();
    }
}

#[test]
fn carmel_wide_divide_absolute_views_execute_in_emitted_c() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let binary = common::scratch_file("carmel-wide-divide", "bin");
    let source = binary.with_extension("c");
    let executable = binary.with_extension("exe");
    let mut c = String::from(
        "#include <stdbool.h>\n#include <stdint.h>\n\
         #define CONCAT12(h,l) (((uint32_t)(uint8_t)(h) << 16) | (uint16_t)(l))\n\
         #define CONCAT22(h,l) (((uint32_t)(uint16_t)(h) << 16) | (uint16_t)(l))\n\
         #define CONCAT44(h,l) (((uint64_t)(uint32_t)(h) << 32) | (uint32_t)(l))\n",
    );
    let mut calls = String::from("int main(void) {\n");
    let routes = [
        (0x4858u16, 0x55, [0x89d1dd24b5, 0, 0], [0x13a3ba496a, 0, 0]),
        (0x4858, 0x01, [0, 0xea433809a4, 0x6a834cecd0], [0, 0xd4433809a4, 0x6a000000a0]),
        (0x4858, 0x15, [0x89d1dd24b5, 0, 0], [0x13d1dd24b5, 0, 0]),
        (0x4858, 0x61, [0, 0xea433809a4, 0x6a834cecd0], [0, 0xd4867109a4, 0x6a834c0000]),
        (0x4858, 0x75, [0x89d1dd24b5, 0, 0], [0x13a3bb24b5, 0, 0]),
        (1, 0x54, [0x1012345678, 0, 0], [0x1f2468acf1, 0, 0]),
        (1, 0x55, [0x1012345678, 0, 0], [0x202467acf1, 0, 0]),
    ];
    for (row, (address, selector, before, after)) in routes.into_iter().enumerate() {
        let [address_hi, address_lo] = address.to_be_bytes();
        fs::write(&binary, [
            [0x3c, address_hi, address_lo, 0xc3, 0x16, selector],
            [0x2b, 0xd8, 0, 0x2b, 0xec, 0],
        ].as_flattened()).unwrap();
        let mut inputs = Vec::new();
        let mut results = Vec::new();
        if before[0] != 0 {
            inputs.extend([("A5L", "unsigned short"), ("A5H", "unsigned short"), ("GB5", "unsigned char")]);
            results.extend((0..3).map(|slot| (slot, 0)));
        } else {
            inputs.extend([
                ("A0L", "unsigned short"), ("A0H", "unsigned short"), ("GB0", "unsigned char"),
                ("A1L", "unsigned short"), ("A1H", "unsigned short"), ("GB1", "unsigned char"),
            ]);
            results.extend((0..6).map(|slot| (slot, 1 + slot / 3)));
        }
        inputs.extend([("EST", "unsigned short"), ("CECFG1", "unsigned short"),
                       ("CECFG2", "unsigned short"), ("CETBL", "unsigned short")]);
        let low = format!("divide_{row}_low");
        let high = format!("divide_{row}_high");
        writeln!(c, "volatile unsigned short {low} = {};", u8::from(address == 1)).unwrap();
        if selector != 0x54 {
            writeln!(c, "volatile unsigned short {high};").unwrap();
        }
        let lane = |value: u64, slot: usize| (value >> ((slot % 3) * 16)) as u16;
        for (slot, bank) in results.into_iter().chain([(inputs.len() - 4, 3)]) {
            let (reg, ty) = inputs[slot];
            let name = format!("divide_{row}_{reg}");
            let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
            kuna.arg("decompile").arg(&binary).args([
                "0", "--addr", "--raw-image", "--target", "Carmel:BE:24:default:default",
                "--base", "0", "--json", "--assert-strict", "--assert", &format!("return %{reg} {ty}"),
                "--assert", &format!("data [data,{address:#x}] volatile unsigned short {low}"),
            ]);
            if selector != 0x54 {
                kuna.args(["--assert", &format!("data [data,{:#x}] volatile unsigned short {high}", address + 1)]);
            }
            for (i, (input, ty)) in inputs.iter().enumerate() {
                kuna.args(["--assert", &format!("param {i} %{input} {ty} in_{}", input.to_ascii_lowercase())]);
            }
            let output = common::process::required_output(&mut kuna);
            let json: Value = serde_json::from_slice(&output.stdout).unwrap();
            let function = &json["functions"][0];
            let text = function["code"].as_str().unwrap();
            assert!(function["error"].is_null() && !text.contains("halt_"), "{function}");
            assert!(json["assertions"].as_array().unwrap().iter().all(|a| a["status"] == "applied"), "{json}");
            writeln!(c, "#define sub_0 {name}\n{text}\n#undef sub_0").unwrap();
            for table in [0, 0xffff] {
                let accumulator_inputs = inputs.len() - 4;
                let arguments = (0..accumulator_inputs)
                    .map(|i| lane(before[if accumulator_inputs == 3 { 0 } else { 1 + i / 3 }], i))
                    .chain([0, 0x1000, 0x1000, table])
                    .map(|v| v.to_string()).collect::<Vec<_>>().join(", ");
                let expected = if bank == 3 { table & 0x3000 }
                    else { lane(if table == 0 { before[bank] } else { after[bank] }, slot) };
                writeln!(calls, "if ({name}({arguments}) != {expected:#x}) return {};", row + 1).unwrap();
            }
        }
    }
    c.push_str(&calls);
    c.push_str("return 0;\n}\n");
    fs::write(&source, &c).unwrap();
    run_emitted_c(&source, &executable, &c);
    for file in [binary, source, executable] {
        fs::remove_file(file).unwrap();
    }
}

#[test]
fn carmel_block_repeat_publishes_each_bank_and_retires_nested_tails() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    const TARGET: &str = "Carmel:BE:24:default:default";
    let binary = common::scratch_file("carmel-block-repeat", "bin");
    let specs = binary.with_extension("specs");
    let source = binary.with_extension("c");
    let executable = binary.with_extension("exe");
    let languages = Path::new("Ghidra/Processors/Carmel/data/languages");
    let installed = std::env::var_os("KUNA_SPECS").map(std::path::PathBuf::from)
        .unwrap_or_else(|| Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../specs")).join(languages);
    let fixture = specs.join(languages);
    fs::create_dir_all(&fixture).unwrap();
    for file in ["carmel.ldefs", "carmel.cspec", "carmel.sla"] {
        fs::copy(installed.join(file), fixture.join(file)).unwrap();
    }
    let mut entry = String::from("<context_data><tracked_set space=\"code\">\n");
    for (name, value) in [("PSTATUS", 0x10), ("LOOP_FRAME", 0), ("LOOP_REDIRECT", 0),
                          ("LOOP_REDIRECT_AT", 0), ("LOOP_TARGET", 0)] {
        writeln!(entry, "<set name=\"{name}\" val=\"{value:#x}\"/>").unwrap();
    }
    for bank in 1..=4 {
        for (name, value) in [
            (format!("LC{bank}"), 0), (format!("LSA{bank}W"), 0), (format!("LEA{bank}W"), 0),
            (format!("LOOP_PENDING{bank}"), 0), (format!("LOOP_FIRST_WORD{bank}"), 0),
            (format!("LOOP_LAST_WORD{bank}"), 0), (format!("LOOP_FIRST_AT{bank}"), 0x04000000),
            (format!("LOOP_LAST_AT{bank}"), 0x04000000),
        ] {
            writeln!(entry, "<set name=\"{name}\" val=\"{value:#x}\"/>").unwrap();
        }
    }
    entry.push_str("</tracked_set></context_data>\n</processor_spec>");
    let pspec = fs::read_to_string(installed.join("carmel.pspec")).unwrap();
    fs::write(fixture.join("carmel.pspec"), pspec.replace("</processor_spec>", &entry)).unwrap();

    let packet = |[a, b, c]: [u8; 3]| [a, b, c, 0x2b, 0xec, 0];
    let nop = packet([0x2b, 0xec, 0]);
    let ret = packet([0x2b, 0xd8, 0]);
    let a0 = packet([0x21, 0xac, 0x20]);
    let a1 = packet([0x21, 0xac, 0x62]);
    let dynamic = [[0x90, 0, 0x0a, 0xce, 0x84, 2], a0, a1, a0, a1, a0, ret];
    let mut register = [nop; 11];
    register[0] = packet([0x2a, 0xa6, 0x7b]); // REPEAT R3, word 15, delay 3
    register[4] = a0;
    register[7] = [0xa1, 0xac, 0x20, 0x21, 0xac, 0x62];
    register[8] = a0;
    register[10] = ret;
    let mut unreachable_odd_tail = register;
    unreachable_odd_tail[7] = a0;
    let nested = [[0xa0, 0, 0x0e, 0xce, 4, 0], [0xa0, 0, 6, 0xce, 4, 0], a0, nop, nop, nop, nop, nop, ret];
    let mut coincident = [nop; 12];
    coincident[0] = [0xa0, 0, 0x12, 0xce, 0x0c, 0];
    coincident[1] = [0xa0, 0, 0x0e, 0xce, 4, 0];
    coincident[11] = ret;
    let continue_issue = packet([0x2b, 0x40, 0x62]);
    let break_issue = packet([0x29, 0x1e, 0x31]);
    let transfer = |control| [
        packet([0x2a, 3, 0x42]), nop, nop, control, a0, a1,
        packet([0x21, 0xac, 0xa4]), packet([0x21, 0xac, 0xe6]),
        packet([0x21, 0xad, 0x28]), packet([0x21, 0xad, 0x6a]), nop, ret,
    ];
    let continuing = transfer(continue_issue);
    let breaking = transfer(break_issue);
    let transfer_inputs = &["A0L", "A1L", "A2L", "A3L", "A4L", "A5L", "ECFG", "EST"][..];
    let mut emitted = String::from(
        "#include <stdbool.h>\n#include <stdint.h>\n#include <stdlib.h>\nvoid carmel_zero_repeat_count(void) { abort(); }\n"
    );
    let mut calls = String::from("int main(void) {\n");
    for (n, (code, base, params, reg, checks)) in [
        (&dynamic[..], "0", &["A0L", "ECFG", "R1"][..], "A0L", &[("10,0,1", 13), ("10,0,3", 19)][..]),
        (&register[..], "0", &["A0L", "ECFG", "R3"][..], "A0L", &[("0,0,1", 3), ("0,0,2", 5)][..]),
        (&register[..], "0", &["A1L", "ECFG", "R3"][..], "A1L", &[("0,0,2", 2)][..]),
        (&unreachable_odd_tail[..], "0", &["R3"][..], "LC1", &[("2", 2)][..]),
        (&nested[..], "0", &["A0L", "ECFG"][..], "A0L", &[("10,0", 14)][..]),
        (&nested[..], "0", &[][..], "LC1", &[("", 0)][..]),
        (&nested[..], "0", &[][..], "LC2", &[("", 0)][..]),
        (&nested[..], "0", &[][..], "PSTATUS", &[("", 0x10)][..]),
        (&nested[..], "0", &[][..], "LOOP_FRAME", &[("", 0)][..]),
        (&coincident[..], "0x1000", &[][..], "LSA1L", &[("", 0x1004)][..]),
        (&coincident[..], "0x1000", &[][..], "LSA2L", &[("", 0x1004)][..]),
        (&coincident[..], "0x1000", &[][..], "LSAL", &[("", 0x1004)][..]),
        (&continuing[..], "0", transfer_inputs, "A0L", &[("0,0,0,0,0,0,0,0", 3)][..]),
        (&continuing[..], "0", transfer_inputs, "LC1", &[("0,0,0,0,0,0,0,0", 0)][..]),
        (&continuing[..], "0", transfer_inputs, "A5L", &[("0,0,0,0,0,0,0,0", 1)][..]),
        (&continuing[..], "0", transfer_inputs, "LOOP_FRAME", &[("0,0,0,0,0,0,0,0", 0)][..]),
        (&breaking[..], "0", transfer_inputs, "A0L", &[("0,0,0,0,0,0,0,0", 1)][..]),
        (&breaking[..], "0", transfer_inputs, "LC1", &[("0,0,0,0,0,0,0,0", 3)][..]),
        (&breaking[..], "0", transfer_inputs, "A5L", &[("0,0,0,0,0,0,0,0", 1)][..]),
        (&breaking[..], "0", transfer_inputs, "LOOP_FRAME", &[("0,0,0,0,0,0,0,0", 0)][..]),
    ].into_iter().enumerate() {
        fs::write(&binary, code.as_flattened()).unwrap();
        let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
        let register_type = if reg == "LOOP_FRAME" { "unsigned char" } else { "unsigned short" };
        kuna.env("KUNA_SPECS", &specs).arg("decompile").arg(&binary).args([
            base, "--raw-image", "--target", TARGET, "--base", base, "--addr", "--json", "--assert-strict",
            "--assert", &format!("return %{reg} {register_type}"),
        ]);
        if code.get(3) == Some(&continue_issue) {
            kuna.args(["--assert", "jumptable 6 6"]);
        }
        for (i, input) in params.iter().enumerate() {
            kuna.args(["--assert", &format!("param {i} %{input} unsigned short in_{i}")]);
        }
        let out = common::process::required_output(&mut kuna);
        let json: Value = serde_json::from_slice(&out.stdout).unwrap();
        let [fun] = json["functions"].as_array().unwrap().as_slice() else { panic!("{json}"); };
        assert!(fun["error"].is_null(), "{fun}");
        let name = fun["name"].as_str().unwrap();
        let code = fun["code"].as_str().unwrap();
        assert!(!code.contains("halt_"), "{code}");
        writeln!(emitted, "#define {name} loop_{n}\n{code}\n#undef {name}").unwrap();
        for (args, expected) in checks {
            writeln!(calls, "if (loop_{n}({args}) != {expected}) return {};", n + 1).unwrap();
        }
    }
    emitted.push_str(&calls);
    emitted.push_str("return 0;\n}\n");
    fs::write(&source, &emitted).unwrap();
    run_emitted_c(&source, &executable, &emitted);
    fs::remove_dir_all(specs).unwrap();
    for file in [binary, source, executable] {
        fs::remove_file(file).unwrap();
    }
}

#[test]
fn carmel_compact_ff12_data_and_stack_select_physical_words() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let binary = common::scratch_file("carmel-compact-ff12-stack", "bin");
    let source = binary.with_extension("c");
    let executable = binary.with_extension("exe");
    let mut emitted = String::from("#include <stdbool.h>\n#include <stdint.h>\n");
    let mut main = String::from("int main(void) {\n");
    let nop = 0x2bec00;
    let cells = [0x100, 0x101, 0x4103, 0x4104, 0x4105, 0x4106, 0x7ff1, 0x7ff2];
    let initial = [0x1122, 0x3344, 0xc103, 0xc104, 0xc105, 0xc106, 0x26, 0];
    for cell in cells {
        writeln!(emitted, "volatile unsigned short cell_{cell:x};").unwrap();
    }
    type Case = ([u32; 2], u16, [u16; 4], &'static [(&'static str, u16)]);
    let rows: [Case; 16] = [
        ([0x20d80c, nop], 0, [0, 0x3344, 0xc103, 0xc104], &[("EST", 0x34)]),
        ([0x20dc0c, nop], 0, [0x7fff, 0, 0xc103, 0xc104], &[("EST", 0x34)]),
        ([0x20fc0c, nop], 0, [0x8001, 0xffff, 0xc103, 0xc104], &[("EST", 0x24)]),
        ([0x303400, nop], 0, [0x1122, 0x3344, 0x4321, 0x100], &[("R7", 0x4102)]),
        ([nop, 0x303000], 0, [0x1122, 0x3344, 0xc103, 0xc104],
            &[("R7", 0x4106), ("R0", 0xc106), ("R4", 0xc105)]),
        ([0xb0378c, nop], 0, [0x1122, 0x3344, 0xffff, 0x8001], &[("R7", 0x4102)]),
        ([0x30338c, nop], 0, [0x1122, 0x3344, 0xc103, 0xc104],
            &[("R7", 0x4106), ("FF1", 0xc106), ("FF2", 0xc105)]),
        ([0x30341a, nop], 0, [0x1122, 0x3344, 1, 0x100], &[("R7", 0x4102)]),
        ([0x3031a0, nop], 0, [0x1122, 0x3344, 0xc103, 0xc104],
            &[("R7", 0x4106), ("ASTATUS", 2), ("R4", 0xc105)]),
        ([0x30301a, nop], 0, [0x1122, 0x3344, 0xc103, 0xc104],
            &[("R7", 0x4106), ("R0", 0xc106), ("ASTATUS", 1)]),
        ([0x3035be, nop], 0, [0x1122, 0x3344, 0xffff, 1], &[("R7", 0x4102)]),
        ([0x3031c0, nop], 0, [0x1122, 0x3344, 0xc103, 0xc104],
            &[("R7", 0x4106), ("R4", 0xc105), ("ASTATUS", 1)]),
        ([0x20d80c, nop], 0x20, [0x8000, 0x3344, 0xc103, 0xc104], &[("EST", 0x34)]),
        ([0x20dc0c, nop], 0x20, [0x3fff, 0x8000, 0xc103, 0xc104], &[("EST", 0x34)]),
        ([0x20fc0c, nop], 0x04, [0xffff, 0x7fff, 0xc103, 0xc104], &[("EST", 0x24)]),
        ([0x20dc0c, nop], 0x08, [0, 0x8000, 0xc103, 0xc104], &[("EST", 0x34)]),
    ];
    let guard_rows: [(Case, u8, u16); 5] = [
        (([0x20d80c, nop], 32, [0, 0x3344, 0xc103, 0xc104], &[("EST", 8)]), 0xff, 1),
        (([0x20dc0c, nop], 4, [0xffff, 0, 0xc103, 0xc104], &[("EST", 0)]), 0xff, 1),
        (([0x20fc0c, nop], 0, [0xffff, 0, 0xc103, 0xc104], &[("EST", 0)]), 0xff, 1),
        (([0x20fc0c, nop], 32, [0x7fff, 0, 0xc103, 0xc104], &[("EST", 0)]), 0xff, 1),
        (([0x20fc0c, nop], 0, [1, 0xffff, 0xc103, 0xc104], &[("EST", 0x24)]), 0, 1),
    ];
    let mut check = 0;
    for (row, ((packet, config, changed, outputs), guard, ff1)) in rows.into_iter()
        .map(|case| (case, 0, 0x8001)).chain(guard_rows).enumerate()
    {
        let words = [
            0x804104, 0xc64298, // MOVE R6,0x4104
            0x800000, 0xc04cec, // TRANSFER SP,R6
            0x800100, 0xc64280, // MOVE R0,0x100
            packet[0], packet[1],
            0x800000, 0xc0769c, // TRANSFER R7,SP
            0x807ff0, 0xc64298, // MOVE R6,0x7ff0
            0x800000, 0xc04cec, // TRANSFER SP,R6
            0x2bd800, nop, // RETURN || NOP
        ];
        let code: Vec<_> = words.into_iter().flat_map(|word| {
            let [_, a, b, c] = word.to_be_bytes();
            [a, b, c]
        }).collect();
        fs::write(&binary, code).unwrap();
        for &(reg, expected) in outputs {
            let name = format!("compact_{row}_{}", reg.to_ascii_lowercase());
            let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
            kuna.arg("decompile").arg(&binary).args([
                "0", "--addr", "--raw-image", "--target", "Carmel:BE:24:default:default", "--base", "0",
                "--json", "--assert-strict", "--assert", &format!("return %{reg} unsigned short"),
            ]);
            for (i, input) in ["EST", "ECFG", "FF1", "FF2", "R4", "ASTATUS"].into_iter().enumerate() {
                kuna.args(["--assert", &format!("param {i} %{input} unsigned short in_{}", input.to_ascii_lowercase())]);
            }
            kuna.args(["--assert", "param 6 %FFG unsigned char in_ffg"]);
            for cell in cells {
                kuna.args(["--assert", &format!("data [data,{cell:#x}] volatile unsigned short cell_{cell:x}")]);
            }
            let output = common::process::required_output(&mut kuna);
            let json: Value = serde_json::from_slice(&output.stdout).unwrap();
            let function = &json["functions"][0];
            let c = function["code"].as_str().unwrap();
            assert!(function["error"].is_null() && !c.contains("halt_"), "{function}");
            writeln!(emitted, "{}", c.replace("sub_0", &name)).unwrap();
            for (cell, value) in cells.into_iter().zip(initial) {
                writeln!(main, "cell_{cell:x} = {value:#x};").unwrap();
            }
            writeln!(main, "value = {name}(0,{config},{ff1},0xffff,0x4321,1,{guard});").unwrap();
            if reg == "R7" && cells.contains(&u32::from(expected)) {
                writeln!(main, "if (value == (unsigned short)(uintptr_t)&cell_{expected:x}) value = {expected:#x};").unwrap();
            }
            check += 1;
            writeln!(main, "if (value != {expected:#x}) return {check};").unwrap();
            for (cell, value) in cells.into_iter().zip(changed.into_iter().chain(initial[4..].iter().copied())) {
                writeln!(main, "if (cell_{cell:x} != {value:#x}) return {check};").unwrap();
            }
        }
    }
    emitted.push_str("static unsigned short value;\n");
    emitted.push_str(&main);
    emitted.push_str("return 0;\n}\n");
    fs::write(&source, &emitted).unwrap();
    run_emitted_c(&source, &executable, &emitted);
    for file in [binary, source, executable] {
        fs::remove_file(file).unwrap();
    }
}

#[test]
fn carmel_compact_move_literal_and_ff_ports_execute_in_emitted_c() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let scratch = tempfile::tempdir().unwrap();
    let binary = scratch.path().join("compact-move.bin");
    let source = scratch.path().join("compact-move.c");
    let executable = scratch.path().join("compact-move");
    let specs = scratch.path().join("specs");
    tracked_carmel_entry_specs(&specs, &[
        ("R0", 0x40), ("R6", 0x8048), ("RN6", 3), ("PACFG", 0),
    ], 0x1000, 0);
    let nop = [0x2b, 0xec, 0];
    let inc = [0x21, 0xac, 0x20];
    let zero = [0x31, 0x9a, 0];
    let seq = [0xb1, 0x9a, 0x3e];
    let read = [0x31, 0x2f, 0x80];
    let store = [0x31, 0x43, 0xbc];
    let packet = |left: [u8; 3], right: [u8; 3]| [left[0], left[1], left[2], right[0], right[1], right[2]];
    let mut c = String::from("#include <stdbool.h>\n#include <stdint.h>\nvolatile unsigned short src, dst;\n");
    let mut calls = String::new();
    for (i, (first, second, reg, expected, expected_dst, config)) in [
        (packet(zero, nop), packet(nop, nop), "EST", 0x0c, 0x1111, 4),
        (packet(nop, zero), packet(nop, nop), "EST", 0x224, 0x1111, 4),
        (packet(seq, nop), packet(nop, nop), "A5H", 0x1f, 0x1111, 4),
        (packet(read, nop), packet(store, nop), "FF1", 0xb679, 0xb679, 4),
        (packet(nop, read), packet(nop, store), "FF2", 0xb679, 0xdb3c, 0x24),
        (packet(nop, read), packet(nop, store), "FF1", 0x1234, 0xdb3c, 0x24),
        (packet(read, nop), packet(store, nop), "R6", 0x804b, 0xdb3c, 0x24),
        (packet(read, nop), packet(store, nop), "EST", 0x24, 0xdb3c, 0x24),
    ].into_iter().enumerate() {
        let code = [packet(inc, nop), packet(nop, nop), first, second, packet(nop, nop),
                    packet([0x2b, 0xd8, 0], nop), packet(nop, nop), packet(nop, nop)];
        fs::write(&binary, code.as_flattened()).unwrap();
        let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
        kuna.env("KUNA_SPECS", &specs).arg("decompile").arg(&binary).args([
            "0x1000", "--addr", "--raw-image", "--base", "0x1000", "--target", "Carmel:BE:24:default:default",
            "--json", "--assert-strict", "--assert", &format!("return %{reg} unsigned short"),
            "--assert", "data [data,0x40] volatile unsigned short src",
            "--assert", "data [data,0x8048] volatile unsigned short dst",
        ]);
        for (n, input) in ["A0L", "EST", "ECFG", "CECFG1", "CECFG2", "CETBL", "FF1", "FF2"].into_iter().enumerate() {
            kuna.args(["--assert", &format!("param {n} %{input} unsigned short in_{n}")]);
        }
        let out = common::process::required_output(&mut kuna);
        let json: Value = serde_json::from_slice(&out.stdout).unwrap();
        let fun = &json["functions"][0];
        assert!(fun["error"].is_null(), "{reg}: {fun}");
        let name = fun["name"].as_str().unwrap();
        writeln!(c, "#define {name} move_port_{i}\n{}\n#undef {name}", fun["code"].as_str().unwrap()).unwrap();
        writeln!(calls, "src=0xb679; dst=0x1111; if (move_port_{i}(0x7fff,0,{config:#x},0x1000,0x1000,0,0x1234,0xf678) != {expected:#x} || src != 0xb679 || dst != {expected_dst:#x}) return {};", i + 1).unwrap();
    }
    writeln!(c, "int main(void) {{\n{calls}\nreturn 0;\n}}").unwrap();
    fs::write(&source, &c).unwrap();
    run_emitted_c(&source, &executable, &c);
}

#[test]
fn carmel_compact_saved_lea_store_uses_index_without_postmodify() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let scratch = tempfile::tempdir().unwrap();
    let binary = scratch.path().join("saved-lea.bin");
    let source = scratch.path().join("saved-lea.c");
    let executable = scratch.path().join("saved-lea");
    let nop = [0x2b, 0xec, 0];
    let mut c = String::from(
        "#include <stdint.h>\n\
         typedef uint32_t undefined3;\n\
         #define CONCAT12(h,l) (((uint32_t)(uint8_t)(h) << 16) | (uint16_t)(l))\n\
         volatile unsigned short lo, hi;\n",
    );
    let mut calls = String::new();
    for (row, (packet, reg, index, base, step)) in [
        ([[0x31, 0xe4, 0x26], nop], "R1", "RN1", 0x4043u16, 0u16),
        ([[0xab, 0xec, 0], [0x31, 0xe4, 0xe6]], "R7", "RN7", 0xfffd, 2),
        ([nop, [0x31, 0xe4, 0x26]], "R1", "RN1", 0x4043, 0xfffe),
    ].into_iter().enumerate() {
        let address = base.wrapping_add(step);
        let specs = scratch.path().join(format!("specs-{row}"));
        tracked_carmel_entry_specs(&specs, &[(reg, base.into()), (index, step.into())], 0, 0);
        fs::write(&binary, [packet[0], packet[1], [0x2b, 0xd8, 0], nop].as_flattened()).unwrap();
        let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
        kuna.env("KUNA_SPECS", &specs).arg("decompile").arg(&binary).args([
            "0", "--addr", "--raw-image", "--target", "Carmel:BE:24:default:default", "--base", "0",
            "--json", "--assert-strict",
            "--assert", "param 0 %LEA1H unsigned char saved_hi",
            "--assert", "param 1 %LEA1L unsigned short saved_lo",
            "--assert", "param 2 %ECFG unsigned short ecfg",
            "--assert", &format!("return %{reg} unsigned short"),
            "--assert", &format!("data [data,{address:#x}] volatile unsigned short lo"),
            "--assert", &format!("data [data,{:#x}] volatile unsigned short hi", address.wrapping_add(1)),
        ]);
        let output = common::process::required_output(&mut kuna);
        let json: Value = serde_json::from_slice(&output.stdout).unwrap();
        let function = &json["functions"][0];
        assert!(function["error"].is_null(), "{function}");
        writeln!(c, "#define sub_0 saved_lea_{row}\n{}\n#undef sub_0", function["code"].as_str().unwrap()).unwrap();
        writeln!(calls, "lo = hi = 0x1111; \
                        if (saved_lea_{row}(0x80, 0xabcd, 0) != {base} || lo != 0xabcd || hi != 0x80) return {}; \
                        lo = hi = 0x1111; \
                        if (saved_lea_{row}(0x80, 0xabcd, 0x20) != {base} || lo != 0x55e6 || hi != 0x40) return {};",
                 2 * row + 1, 2 * row + 2).unwrap();
    }
    writeln!(c, "int main(void) {{\n{calls}\nreturn 0;\n}}").unwrap();
    fs::write(&source, &c).unwrap();
    run_emitted_c(&source, &executable, &c);
}

#[test]
fn carmel_sequential_addabs_commits_wrapped_accumulator_after_address_admission() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let scratch = tempfile::tempdir().unwrap();
    let binary = scratch.path().join("addabs.bin");
    let source = scratch.path().join("addabs.c");
    let executable = scratch.path().join("addabs");
    let specs = scratch.path().join("specs");
    tracked_carmel_entry_specs(&specs, &[
        ("R0", 0x40), ("R1", 0x4040), ("RN1", 2), ("PACFG", 0), ("EST", 0), ("CECFG2", 0),
        ("A3L", 0xffff), ("A3H", 0xffff), ("GB3", 0x7f), ("GB5", 0x89),
        ("LEA1L", 0xbcde), ("LEA1H", 0x8a),
    ], 0, 0);
    const NOP: [u8; 6] = [0x2b, 0xec, 0, 0x2b, 0xec, 0];
    let mut code = vec![NOP; 6];
    code.extend([[0x9b, 0x1b, 0x87, 0x31, 0xe4, 0x26], NOP, [0x2b, 0xd8, 0, 0x2b, 0xec, 0], NOP]);
    fs::write(&binary, code.as_flattened()).unwrap();
    let mut c = String::from("#include <stdbool.h>\n#include <stdint.h>\n\
        static volatile unsigned short word, lo, hi;\n");
    for (reg, name, ty) in [
        ("R0", "pointer", "unsigned short"), ("EST", "flags", "unsigned short"),
        ("GB5", "guard", "unsigned char"),
    ] {
        let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
        kuna.env("KUNA_SPECS", &specs).arg("decompile").arg(&binary).args([
            "0", "--addr", "--raw-image", "--target", "Carmel:BE:24:default:default", "--base", "0",
            "--json", "--assert-strict", "--assert", &format!("return %{reg} {ty}"),
            "--assert", "data [data,0x40] volatile unsigned short word",
            "--assert", "data [data,0x4042] volatile unsigned short lo",
            "--assert", "data [data,0x4043] volatile unsigned short hi",
        ]);
        for (n, reg) in ["CECFG1", "CETBL", "ECFG"].into_iter().enumerate() {
            kuna.args(["--assert", &format!("param {n} %{reg} unsigned short in_{n}")]);
        }
        let output = common::process::required_output(&mut kuna);
        let json: Value = serde_json::from_slice(&output.stdout).unwrap();
        let function = &json["functions"][0];
        assert!(function["error"].is_null(), "{function}");
        writeln!(c, "#define sub_0 {name}\n{}\n#undef sub_0", function["code"].as_str().unwrap()).unwrap();
    }
    c.push_str("int main(void) {\n\
        word = 1;\n\
        if (pointer(0, 0, 0) != 0x40 || guard(0, 0, 0) != 0x89) return 1;\n\
        if (pointer(0x1000, 0, 0) != 0x3f || guard(0x1000, 0, 0) != 0x89) return 2;\n\
        if (pointer(0, 0xffff, 0) != 0x3f || guard(0, 0xffff, 0) != 0x80) return 3;\n\
        if (flags(0, 0xffff, 0) != 0x3036 || flags(0, 0xffff, 4) != 0x3032) return 4;\n\
        word = 0xffff;\n\
        if (guard(0, 0xffff, 0) != 0x7f || flags(0, 0xffff, 0) != 0x3021) return 5;\n\
        if (lo != 0xbcde || hi != 0x8a) return 6;\n\
        pointer(0, 0, 0x20);\n\
        if (lo != 0x5e6f || hi != 0x45) return 7;\n\
        return 0;\n}\n");
    fs::write(&source, &c).unwrap();
    run_emitted_c(&source, &executable, &c);
}

#[test]
fn carmel_compact_shift_lane_ports_execute_in_emitted_c() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    const NOP: [u8; 3] = [0x2b, 0xec, 0];
    const SEQUENTIAL_NOP: [u8; 3] = [0xab, 0xec, 0];
    const WORD_SHIFT: [u8; 3] = [0x39, 0x0a, 0x08];
    const RN_SHIFT: [u8; 3] = [0x38, 0x08, 0x09];
    const ARITHMETIC: [[u8; 3]; 2] = [[0x3e, 0xe7, 0x2c], NOP];
    const DATA_SHIFT: [[u8; 3]; 2] = [[0x3b, 0x1d, 0x2d], NOP];
    let binary = common::scratch_file("carmel-compact-shift-ports", "bin");
    let source = binary.with_extension("c");
    let executable = binary.with_extension("exe");
    let specs = [0, 1, 2].map(|profile| binary.with_extension(format!("specs-{profile}")));
    tracked_carmel_entry_specs(&specs[0], &STATIC_CE, 0, 0);
    tracked_carmel_entry_specs(&specs[1], &STATIC_CE[1..], 0, 0);
    tracked_carmel_entry_specs(&specs[2], &[STATIC_CE[1], ("R7", 0xc048)], 0, 0);
    let mut c = String::from(
        "#include <stdbool.h>\n#include <stdint.h>\n\
         #define CONCAT12(h,l) (((uint32_t)(uint8_t)(h) << 16) | (uint16_t)(l))\n\
         #define CONCAT14(h,l) (((uint64_t)(uint8_t)(h) << 32) | (uint32_t)(l))\n\
         #define CONCAT22(h,l) (((uint32_t)(uint16_t)(h) << 16) | (uint16_t)(l))\n\
         static volatile unsigned short below, word, above;\n\
         static unsigned accesses;\n\
         static volatile unsigned short *port(void) { ++accesses; return &word; }\n\
         #define shift_word (*port())\n",
    );
    let mut main = String::from("int main(void) {\n");
    let mut check = 0;
    let mut project = |packet: [[u8; 3]; 2],
                       output: &str,
                       inputs: &[&str],
                       states: &[(&[u16], u16, Option<(u16, u16)>)],
                       addr: Option<u16>| {
        fs::write(&binary, [packet[0], packet[1], [0x2b, 0xd8, 0], NOP].as_flattened()).unwrap();
        let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
        let profile = if addr == Some(0xc048) { 2 } else { usize::from(inputs.contains(&"CECFG1")) };
        kuna.env("KUNA_SPECS", &specs[profile]);
        let c_type = |reg: &str| if reg.starts_with("GB") { "unsigned char" } else { "unsigned short" };
        kuna.arg("decompile").arg(&binary).args([
            "0", "--addr", "--raw-image", "--target", "Carmel:BE:24:default:default", "--base", "0",
            "--json", "--assert-strict", "--assert", &format!("return %{output} {}", c_type(output)),
        ]);
        for (i, reg) in inputs.iter().enumerate() {
            kuna.args(["--assert", &format!("param {i} %{reg} {} in_{}", c_type(reg), reg.to_ascii_lowercase())]);
        }
        if let Some(address) = addr {
            for (delta, cell) in [(u16::MAX, "below"), (0, "shift_word"), (1, "above")] {
                kuna.args([
                    "--assert", &format!("data [data,{:#x}] volatile unsigned short {cell}", address.wrapping_add(delta)),
                ]);
            }
        }
        let result = common::process::required_output(&mut kuna);
        let json: Value = serde_json::from_slice(&result.stdout).unwrap();
        assert!(json["assertions"].as_array().unwrap().iter().all(|a| a["status"] == "applied"), "{json}");
        let function = &json["functions"][0];
        let text = function["code"].as_str().unwrap();
        assert!(function["error"].is_null() && !text.contains("halt_"), "{function}");
        let name = format!("shift_port_{check}");
        writeln!(c, "#define sub_0 {name}\n{text}\n#undef sub_0").unwrap();
        for (args, expected, memory) in states {
            let arguments = args.iter().map(u16::to_string).collect::<Vec<_>>().join(",");
            let (num_accesses, data_after) = memory.unwrap_or((0, 0xcdef));
            check += 1;
            writeln!(main, "below = 0x57ac; word = 0xcdef; above = 0xcd61; accesses = 0;").unwrap();
            writeln!(main, "if ({name}({arguments}) != {expected:#x} || accesses != {num_accesses} || \
                            below != 0x57ac || word != {data_after:#x} || above != 0xcd61) return {check};").unwrap();
        }
    };
    for (packet, lo, flags) in [
        ([WORD_SHIFT, NOP], 0x9ab0, 0x3020),
        ([NOP, WORD_SHIFT], 0x789a, 0x3000),
        ([SEQUENTIAL_NOP, WORD_SHIFT], 0x9ab0, 0x3020),
    ] {
        let inputs = ["A0L", "A4L", "EST"];
        let state = [0x789a, 0x89ab, 0x3000];
        project(packet, "A0L", &inputs, &[(&state, lo, None)], None);
        project(packet, "EST", &inputs, &[(&state, flags, None)], None);
    }
    for (packet, lo, guard) in [
        ([RN_SHIFT, NOP], 0x20, 0),
        ([NOP, RN_SHIFT], 0x789a, 0x12),
        ([SEQUENTIAL_NOP, RN_SHIFT], 0x20, 0),
    ] {
        let inputs = ["A0L", "GB0", "RN0", "EST", "CECFG1", "CETBL"];
        let enabled = [0x789a, 0x12, 2, 0x3000, 0, 0xffff];
        let disabled = [0x789a, 0x12, 2, 0x2000, 0x1000, 0xff00];
        project(packet, "A0L", &inputs, &[(&enabled, lo, None), (&disabled, 0x789a, None)], None);
        project(packet, "GB0", &inputs, &[(&enabled, guard, None)], None);
    }
    let inputs = ["A1H", "FF1", "ECFG", "EST"];
    let shifted = [0x8765, 0x7f, 0, 0x3000];
    let left_large = [0x8765, 0x20, 0, 0x3000];
    let right_large = [0x8765, 0x40, 0, 0x3000];
    let limited = [0x8003, 1, 4, 0x3000];
    project(ARITHMETIC, "A1H", &inputs, &[
        (&shifted, 0xc3b2, None), (&left_large, 0, None),
        (&right_large, 0xffff, None), (&limited, 0x8000, None),
    ], None);
    project(ARITHMETIC, "EST", &inputs, &[
        (&shifted, 0x3011, None), (&left_large, 0x3008, None),
        (&right_large, 0x3011, None), (&limited, 0x3035, None),
    ], None);
    let inputs = ["A0H", "A2H", "A2L", "GB2", "ECFG", "EST"];
    let negative = [0x8001, 0xb008, 0x4b74, 0x87, 4, 0x3202];
    let positive = [0x7001, 0xb008, 0x4b74, 0x87, 4, 0x3202];
    let shift = [0x3d, 0xac, 0x0e];
    for (packet, negative_result, positive_result, flags) in [
        ([[0xbd, 0xac, 0x0e], NOP], 0x8000, 0x7fff, 0x3236),
        ([SEQUENTIAL_NOP, shift], 0x8000, 0x7fff, 0x3236),
        ([shift, NOP], 0x8000, 0x7fff, 0x3236),
        ([NOP, shift], 0xb008, 0xb008, 0x3202),
    ] {
        project(packet, "A2H", &inputs, &[(&negative, negative_result, None), (&positive, positive_result, None)], None);
        project(packet, "EST", &inputs, &[(&negative, flags, None)], None);
    }
    let arithmetic_right = [[0x3d, 0xac, 0x7e], NOP];
    project(arithmetic_right, "A2H", &inputs, &[(&negative, 0xc000, None)], None);
    project(arithmetic_right, "A2L", &inputs, &[(&negative, 0x4b74, None)], None);
    project(arithmetic_right, "GB2", &inputs, &[(&negative, 0x87, None)], None);
    project(DATA_SHIFT, "EST", &["RN1", "FF1", "EST", "CECFG1", "CETBL"], &[
        (&[2, 32, 0x3000, 0, 0xffff], 0x3008, Some((1, 0))),
        (&[2, 64, 0x3000, 0, 0xffff], 0x3008, Some((1, 0))),
        (&[2, 127, 0x3000, 0, 0xffff], 0x3000, Some((1, 1))),
        (&[0x8003, 127, 0x2000, 0x1000, 0xff00], 0x2000, None),
    ], Some(0xc048));
    project([NOP, [0x3f, 0xca, 0x22]], "R2", &["R2", "RN2", "PACFG", "RM2", "RL2"], &[
        (&[0x8040, 3, 0, 7, 0x8040], 0x8043, None),
        (&[0x8040, 3, 0x20, 7, 0x8040], 0x803b, None),
    ], Some(0x8040));
    c.push_str(&main);
    c.push_str("return 0;\n}\n");
    fs::write(&source, &c).unwrap();
    run_emitted_c(&source, &executable, &c);
    for path in specs {
        fs::remove_dir_all(path).unwrap();
    }
    for file in [binary, source, executable] {
        fs::remove_file(file).unwrap();
    }
}

#[test]
fn carmel_wide_byte_limit_ports_execute_in_emitted_c() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let binary = common::scratch_file("carmel-wide-byte-limit", "bin");
    let source = binary.with_extension("c");
    let executable = binary.with_extension("exe");
    let mut emitted = String::from("#include <stdbool.h>\n#include <stdint.h>\n");
    let nop = [0x2b, 0xec, 0, 0x2b, 0xec, 0];
    let ret = [0x2b, 0xd8, 0, 0x2b, 0xec, 0];
    for (name, packet, output) in [
        ("extended_flags", [0x77, 0xd4, 0x08, 0xc3, 0xb3, 0xd6], "EST"),
        ("guard_flags", [0x77, 0xd4, 0x08, 0xc3, 0xb2, 0x02], "EST"),
        ("short_flags", [0x50, 0xd4, 0x08, 0xc3, 0xb3, 0xd6], "EST"),
        ("alias_result", [0x77, 0xd4, 0x08, 0xc3, 0xb2, 0x1a], "RM6"),
        ("ff_result", [0x77, 0xd4, 0x08, 0xc3, 0xb2, 0xfa], "FF1"),
        ("loop_result", [0x77, 0xd4, 0x08, 0xc3, 0xb3, 0x52], "LC1"),
        ("loop_flags", [0x77, 0xd4, 0x08, 0xc3, 0xb3, 0x52], "EST"),
    ] {
        fs::write(&binary, [packet, nop, nop, nop, nop, ret].as_flattened()).unwrap();
        let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
        kuna.arg("decompile").arg(&binary).args([
            "0", "--addr", "--raw-image", "--target", "Carmel:BE:24:default:default",
            "--base", "0", "--json", "--assert-strict", "--assert",
            &format!("return %{output} unsigned short"),
        ]);
        for (i, input) in ["RM6", "A0L", "FF1", "LC1", "EST", "CECFG1", "CECFG2", "CETBL"]
            .into_iter().enumerate()
        {
            kuna.args(["--assert", &format!("param {i} %{input} unsigned short in_{i}")]);
        }
        let output = common::process::required_output(&mut kuna);
        let json: Value = serde_json::from_slice(&output.stdout).unwrap();
        let [fun] = json["functions"].as_array().unwrap().as_slice() else { panic!("{json}"); };
        assert!(fun["error"].is_null(), "{fun}");
        let code = fun["code"].as_str().unwrap();
        assert!(!code.contains("halt_"), "{fun}");
        writeln!(emitted, "#define sub_0 {name}\n{code}\n#undef sub_0").unwrap();
    }
    emitted.push_str(
        "int main(void) {\n\
         if (extended_flags(0x527,0x23,0x1234,9,0,0x1000,0x1000,0x100) != 0x2004) return 1;\n\
         if (extended_flags(0x527,0x23,0x1234,9,0,0x1000,0x1000,1) != 0x1000) return 2;\n\
         if (guard_flags(0x8000,0x23,0x1234,9,0,0x1000,0x1000,0xffff) != 0x3004) return 3;\n\
         if (short_flags(0x527,0xff,0x1234,9,0x20,0x1000,0x1000,0xffff) != 0x3020) return 4;\n\
         if (short_flags(0x527,0xff,0x1234,9,4,0x1000,0x1000,0xffff) != 0x3004) return 5;\n\
         if (alias_result(0x527,0x23,0x1234,9,0,0x1000,0x1000,0x100) != 0xff) return 6;\n\
         if (alias_result(0x527,0x23,0x1234,9,0,0x1000,0x1000,1) != 0x527) return 7;\n\
         if (ff_result(0x8000,0x23,0x1234,9,0,0x1000,0x1000,0x100) != 0) return 8;\n\
         if (ff_result(0x8000,0x23,0x1234,9,0,0x1000,0x1000,1) != 0x1234) return 9;\n\
         if (loop_result(0xfe,0x23,0x1234,9,0,0x1000,0x1000,0x100) != 0xfe) return 10;\n\
         if (loop_result(0xfe,0x23,0x1234,9,0,0x1000,0x1000,1) != 9) return 11;\n\
         if (loop_flags(0x100,0x23,0x1234,9,0,0x1000,0x1000,0x100) != 0x2004) return 12;\n\
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
fn carmel_wide_lim8_clamps_pointer_and_increment_sources() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let binary = common::scratch_file("carmel-lim8-pointer", "bin");
    let source = binary.with_extension("c");
    let executable = binary.with_extension("exe");
    let mut c = String::from("#include <stdbool.h>\n#include <stdint.h>\n");
    let mut checks = String::from("int main(void) {\n");
    for (i, (selector, register)) in [
        (0xa0, "R0"), (0xa1, "R1"), (0xa6, "R6"), (0xa8, "RN0"), (0xaf, "RN7"), (0xbf, "RN7"),
    ].into_iter().enumerate() {
        fs::write(&binary, [
            [selector, 0xfd, 0xfe, 0xc3, 0xa2, 0xb3],
            [0x2b, 0xd8, 0, 0x2b, 0xec, 0],
        ].as_flattened()).unwrap();
        for result in ["RN4", "EST"] {
            let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
            kuna.arg("decompile").arg(&binary).args([
                "0x1000", "--addr", "--raw-image", "--target", "Carmel:BE:24:default:default",
                "--base", "0x1000", "--json", "--assert-strict",
                "--assert", &format!("return %{result} unsigned short"),
            ]);
            for (slot, input) in [register, "RN4", "EST", "CECFG1", "CECFG2", "CETBL"].into_iter().enumerate() {
                kuna.args(["--assert", &format!("param {slot} %{input} unsigned short in_{slot}")]);
            }
            let output = common::process::required_output(&mut kuna);
            let doc: Value = serde_json::from_slice(&output.stdout).unwrap();
            let function = &doc["functions"][0];
            let code = function["code"].as_str().unwrap();
            assert!(function["error"].is_null() && !code.contains("halt_"), "{doc}");
            writeln!(c, "#define sub_1000 lim8_{i}_{result}\n{code}\n#undef sub_1000").unwrap();
        }
        for (v, table, word, status) in [
            (0x67u16, 0u16, 0x67, 0),
            (0xff, 0xffff, 0xff, 0x3000),
            (0x100, 0, 0xff, 4),
            (0x8000, 0xffff, 0, 0x3004),
        ] {
            writeln!(checks, "if (lim8_{i}_RN4({v},0x7878,0,0,0,{table}) != {word} || \
                lim8_{i}_EST({v},0x7878,0,0,0,{table}) != {status}) return {};", i + 1).unwrap();
        }
    }
    c.push_str(&checks);
    c.push_str("return 0;\n}\n");
    fs::write(&source, &c).unwrap();
    run_emitted_c(&source, &executable, &c);
    for file in [binary, source, executable] {
        fs::remove_file(file).unwrap();
    }
}

#[test]
fn carmel_wide_control_alu_routes_execute_in_emitted_c() {
    const TARGET: &str = "Carmel:BE:24:default:default";
    let binary = common::scratch_file("carmel-wide-control-alu", "bin");
    let source = binary.with_extension("c");
    let executable = binary.with_extension("exe");
    let specs = binary.with_extension("specs");
    let nop = [0x2b, 0xec, 0, 0x2b, 0xec, 0];
    let ret = [0x2b, 0xd8, 0, 0x2b, 0xec, 0];
    let sub_flags = [0xaf, 0x74, 0x44, 0x81, 0xb0, 0x61];
    let msu_lc = [0x91, 0x76, 0xe4, 0xac, 0xd4, 0xa1];
    fs::write(&binary, [sub_flags, msu_lc, ret].as_flattened()).unwrap();
    let (stdout, stderr, status) = common::run_kuna(&[
        "disassemble", binary.to_str().unwrap(), "0x1000", "--raw-image", "--target", TARGET,
        "--base", "0x1000", "--count", "2", "--json",
    ]);
    assert_eq!(status, 0, "{stdout}\n{stderr}");
    let listing: Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(listing["instructions"][0]["text"], "io_space = *(r0+=0x44) - rn7;");
    assert_eq!(listing["instructions"][1]["text"], "lc -= a0l * a1l;");
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        fs::remove_file(binary).unwrap();
        return;
    }
    tracked_carmel_entry_specs(&specs, &[("R0", 0x40), ("R3", 0xc040), ("R4", 0x48)], 0x1000, 0);

    type Regs = &'static [(&'static str, u16)];
    type Cells = &'static [(u16, u16, u16)];
    let agu: Regs = &[("RN7", 4), ("PACFG", 0), ("RM0", 0), ("RL0", 0)];
    let r0_read: Cells = &[(0x40, 0xa258, 0xa258)];
    let r3_read: Cells = &[(0xc040, 0x786, 0x786)];
    let mac_latch: Regs = &[("RF4", 0), ("X1", 0xba98)];
    let routes: [([u8; 6], Regs, Regs, Cells); 16] = [
        ([0xaf, 0x74, 0x44, 0x81, 0xae, 0x61],
         &[("PACFG", 0), ("RM0", 0), ("RL0", 0), ("USER_IO", 5)],
         &[("EST", 0x3000), ("USER_IO", 5), ("R0", 0x84)], r0_read),
        ([0xaf, 0x74, 0x44, 0x81, 0xac, 0x61], agu,
         &[("EST", 0x3011), ("INTBASE", 0xa254), ("R0", 0x84)], r0_read),
        (sub_flags, agu, &[("EST", 0x3011), ("R0", 0x84)], r0_read),
        ([0xaf, 0x74, 0x44, 0x81, 0x0d, 0xab], &[("PSTATUS", 0x10), ("RN7", 4)],
         &[("EST", 0x3000), ("PSTATUS", 0)], &[]),
        ([0x88, 0x74, 0x44, 0x81, 0x0d, 0xaf],
         &[("USER_IO", 5), ("A0L", 0x09a4), ("A0H", 0x4338), ("GB0", 0xea)],
         &[("EST", 0x3000), ("USER_IO", 0xbcc7)], &[]),
        ([0xaf, 0x74, 0x44, 0x81, 0x0c, 0x21], agu,
         &[("EST", 0x3011), ("R0", 0x41)], &[(0x40, 0xa258, 0xa254), (0x41, 0xecd6, 0xecd6)]),
        ([0xcf, 0xa1, 0x3c, 0xeb, 0x5f, 0x5e], &[("USER_IO", 5)],
         &[("EST", 0x3000), ("USER_IO", 5)], &[]),
        ([0xcf, 0xa1, 0x3c, 0xeb, 0x57, 0x5e], &[("USER_IO", 5), ("PSTATUS", 0x10)],
         &[("EST", 0x3000), ("PSTATUS", 0x10)], &[]),
        ([0xcf, 0xa1, 0x3c, 0xeb, 0x5b, 0x5e], &[("USER_IO", 5), ("INTBASE", 2)],
         &[("EST", 0x3000), ("INTBASE", 0xa139)], &[]),
        ([0x83, 0x76, 0xe4, 0xac, 0xd7, 0xf8], &[("RF4", 0), ("X1", 0xba98), ("USER_IO", 5)],
         &[("EST", 0x3000), ("USER_IO", 5), ("X1", 0)], r3_read),
        (msu_lc, &[("A0L", 0x09a4), ("A1L", 0xecd0), ("LC", 0), ("LC1", 0)],
         &[("EST", 0x3000), ("LC", 0), ("LC1", 0xb8)], &[]),
        ([0x83, 0x76, 0xe4, 0xac, 0x90, 0xf8],
         &[("RF4", 0), ("X1", 0xba98), ("A0L", 0x09a4), ("A0H", 0x4338), ("GB0", 0xea)],
         &[("EST", 0x3009), ("A0L", 0), ("A0H", 0x4338), ("GB0", 0xea), ("X1", 0)], r3_read),
        ([0x88, 0x74, 0x44, 0x81, 0x0d, 0xab],
         &[("PSTATUS", 0x10), ("A0L", 0), ("A0H", 0xffcd), ("GB0", 0xff)],
         &[("EST", 0x3000), ("PSTATUS", 0x30)], &[]),
        ([0xaf, 0x74, 0x44, 0x81, 0x0d, 0xaf], &[("USER_IO", 5), ("RN7", 6)],
         &[("EST", 0x3000), ("USER_IO", 0xffff)], &[]),
        (msu_lc, &[("A0L", 0x09a4), ("A1L", 0xecd0), ("LC", 0), ("LC1", 0x600)],
         &[("EST", 0x3000), ("LC", 0), ("LC1", 0x6b8)], &[]),
        ([0x83, 0x76, 0xe4, 0xac, 0x04, 0xf8], mac_latch,
         &[("EST", 0x3009), ("X1", 0)], &[(0x48, 1, 0), (0xc040, 0x786, 0x786)]),
    ];
    let mut c = String::from(
        "#include <stdbool.h>\n#include <stdint.h>\n#include <stdio.h>\n\
         #define CONCAT12(h,l) (((uint32_t)(uint8_t)(h) << 16) | (uint16_t)(l))\n\
         #define CONCAT14(h,l) (((uint64_t)(uint8_t)(h) << 32) | (uint32_t)(l))\n\
         #define CONCAT22(h,l) (((uint32_t)(uint16_t)(h) << 16) | (uint16_t)(l))\n\
         #define CONCAT32(h,l) (((uint64_t)((uint32_t)(h) & 0xffffffu) << 16) | (uint16_t)(l))\n",
    );
    let mut calls = String::from("int main(void) {\n");
    let reg_type = |reg: &str| if reg.starts_with("GB") { "unsigned char" } else { "unsigned short" };
    for (row, (packet, extra_inputs, outputs, cells)) in routes.into_iter().enumerate() {
        fs::write(&binary, [packet, nop, nop, nop, ret].as_flattened()).unwrap();
        let mut inputs = vec![("EST", 0), ("ECFG", 0), ("CECFG1", 0x1000), ("CECFG2", 0x1000), ("CETBL", 0xffff)];
        inputs.extend_from_slice(extra_inputs);
        let arguments = inputs.iter().map(|(_, value)| value.to_string()).collect::<Vec<_>>().join(",");
        for &(address, _, _) in cells {
            writeln!(c, "volatile unsigned short data_{row}_{address:04x};").unwrap();
        }
        for &(register, expected) in outputs {
            let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
            kuna.env("KUNA_SPECS", &specs).arg("decompile").arg(&binary).args([
                "0x1000", "--addr", "--raw-image", "--target", TARGET, "--base", "0x1000",
                "--json", "--assert-strict", "--assert",
                &format!("return %{register} {}", reg_type(register)),
            ]);
            for (i, (reg, _)) in inputs.iter().enumerate() {
                kuna.args(["--assert", &format!("param {i} %{reg} {} in_{}", reg_type(reg), reg.to_ascii_lowercase())]);
            }
            for &(address, _, _) in cells {
                kuna.args(["--assert", &format!("data [data,{address:#x}] volatile unsigned short data_{row}_{address:04x}")]);
            }
            let result = common::process::required_output(&mut kuna);
            let json: Value = serde_json::from_slice(&result.stdout).unwrap();
            let function = &json["functions"][0];
            let text = function["code"].as_str().unwrap();
            assert!(function["error"].is_null() && !text.contains("halt_"), "{function}");
            assert!(json["assertions"].as_array().unwrap().iter().all(|a| a["status"] == "applied"), "{json}");
            let name = format!("control_alu_{row}_{}", register.to_ascii_lowercase());
            writeln!(c, "#define sub_1000 {name}\n{text}\n#undef sub_1000").unwrap();
            for &(address, before, _) in cells {
                writeln!(calls, "data_{row}_{address:04x} = {before};").unwrap();
            }
            writeln!(calls, "if ({name}({arguments}) != {expected}) {{ fputs(\"{name}\\n\", stderr); return 1; }}").unwrap();
            for &(address, _, after) in cells {
                writeln!(calls, "if (data_{row}_{address:04x} != {after}) {{ fputs(\"{name} DATA[{address:#x}]\\n\", stderr); return 1; }}").unwrap();
            }
        }
    }
    c.push_str(&calls);
    c.push_str("return 0;\n}\n");
    fs::write(&source, &c).unwrap();
    run_emitted_c(&source, &executable, &c);
    fs::remove_dir_all(specs).unwrap();
    for file in [binary, source, executable] {
        fs::remove_file(file).unwrap();
    }
}

#[test]
fn carmel_wide_short_sources_keep_carry_data_width_and_veto_separate() {
    const TARGET: &str = "Carmel:BE:24:default:default";
    let binary = common::scratch_file("carmel-wide-short-address", "bin");
    let nop = [0x2b, 0xec, 0, 0x2b, 0xec, 0];
    let ret = [0x2b, 0xd8, 0, 0x2b, 0xec, 0];
    for packet in [
        [0x3d, 0x91, 0x26, 0x80, 0x21, 0x23],
        [0x3f, 0x91, 0x26, 0x80, 0x21, 0x23],
        [0x3f, 0x91, 0x26, 0x90, 0x84, 0xcb],
        [0x30, 0x91, 0x26, 0x87, 0x11, 0x7f],
    ] {
        fs::write(&binary, [packet, ret].as_flattened()).unwrap();
        let (stdout, stderr, status) = common::run_kuna(&[
            "raw-pcode", binary.to_str().unwrap(), "0x1000", "--raw-image", "--target", TARGET,
            "--base", "0x1000", "--count", "1", "--json",
        ]);
        let out: Value = serde_json::from_str(&stdout).unwrap_or_else(|_| panic!("{stdout}\n{stderr}"));
        assert_eq!(status, 1, "{out}");
        assert!(out["instructions"].as_array().unwrap().is_empty(), "{out}");
    }
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        fs::remove_file(binary).unwrap();
        return;
    }
    let specs = binary.with_extension("specs");
    tracked_carmel_entry_specs(&specs, &[("R0", 0x40), ("R2", 0x8040), ("R5", 0x4048)], 0x1000, 0);
    let mut c = String::from(
        "#include <stdbool.h>\n#include <stdint.h>\n\
         #define CONCAT22(h,l) (((uint32_t)(uint16_t)(h) << 16) | (uint16_t)(l))\n\
         volatile unsigned short old, next, low, high, in_low, in_high;\n",
    );
    let mut calls = String::from("int main(void) {\nin_low = 0xacf1; in_high = 0xde23;\n");
    let carry = [0x0d, 0, 0, 0x91, 0x17, 0x19];
    let carry2 = [0x2b, 0xec, 0, 0x12, 0x0e, 0x9f];
    let word = [0x3f, 0, 0, 0x86, 0x21, 0x23];
    let ff = [0x3e, 0, 0x26, 0x86, 0x84, 0xcb];
    for (index, (packet, prefix, reg, check)) in [
        (word, Some(carry), "R0", "ret == 0x41 && old == 0x6946 && next == 0x2345"),
        (word, None, "EST", "ret == 0x3000 && old == 0x6945 && next == 0x2345"),
        (word, None, "R0", "ret == 0x41 && old == 0xabcd && next == 0x2345"),
        (word, None, "R0", "ret == 0x40 && old == 0xabcd && next == 0x2345"),
        (ff, None, "EST", "ret == 0x3001 && low == 0xda73 && high == 0x21db"),
        (ff, None, "EST", "ret == 0x3001 && low == 0xed39 && high == 0x10ed"),
        ([0x3e, 0, 0x26, 0x86, 0x04, 0xcb], None, "EST", "ret == 0x3001 && low == 0x21db && high == 0xdf23"),
        ([0x30, 0, 0, 0x87, 0x11, 0x23], None, "A0L", "ret == 0x7b79"),
        ([0x3f, 0, 0, 0x8f, 0x11, 0x23], Some(carry), "A0L", "ret == 0x6945"),
        ([0x3f, 0, 0, 0x8f, 0x11, 0x23], Some(carry2), "A0L", "ret == 0x6946"),
        ([0x3f, 0, 0, 0x87, 0x11, 0x23], Some(carry2), "A0L", "ret == 0x6945"),
        ([0x3f, 0, 0, 0x8f, 0x11, 0x23], Some(carry2), "EST", "ret == 0x3240"),
        ([0x7f, 0, 0, 0x86, 0x21, 0x23], None, "R0", "ret == 0x41 && old == 0xabcd && next == 0x2345"),
    ].into_iter().enumerate() {
        let mut image = prefix.into_iter().collect::<Vec<_>>();
        image.extend_from_slice(&[packet, nop, nop, nop, ret]);
        fs::write(&binary, image.as_flattened()).unwrap();
        let mut args = Command::new(env!("CARGO_BIN_EXE_kuna"));
        args.env("KUNA_SPECS", &specs).arg("decompile").arg(&binary).args([
            "0x1000", "--addr", "--raw-image", "--target", TARGET, "--base", "0x1000", "--json",
            "--assert-strict", "--assert", &format!("return %{reg} unsigned short"),
            "--assert", "data [data,0x40] volatile unsigned short old",
            "--assert", "data [data,0x41] volatile unsigned short next",
            "--assert", "data [data,0x8040] volatile unsigned short low",
            "--assert", "data [data,0x8041] volatile unsigned short high",
            "--assert", "data [data,0x406e] volatile unsigned short in_low",
            "--assert", "data [data,0x406f] volatile unsigned short in_high",
        ]);
        let mut inputs = vec![
            ("EST", 0), ("ECFG", if index == 5 { 32 } else { 0 }),
            ("CECFG1", if matches!(index, 3 | 12) { 0 } else { 0x1000 }), ("CECFG2", 0x1000),
            ("CETBL", if index == 12 { 0x00ff } else if matches!(index, 2 | 3) { 0 } else { 0xffff }),
            ("A1L", 0x96ba), ("FF1", 0x8765), ("FF2", if index == 5 { 0x1234 } else { 0x5432 }),
            ("PACFG", 0), ("RM0", 0), ("RL0", 0x1234), ("A0L", 0x8721),
        ];
        if prefix == Some(carry) {
            inputs.extend_from_slice(&[("A4L", 0xffff), ("A4H", 0xffff), ("GB4", 0xff),
                                       ("A5L", 1), ("A5H", 0), ("GB5", 0)]);
        } else if prefix == Some(carry2) {
            inputs.extend_from_slice(&[("A4L", 0xffff), ("X2", 1)]);
        }
        for (i, (input, _)) in inputs.iter().enumerate() {
            let ty = if input.starts_with("GB") { "unsigned char" } else { "unsigned short" };
            args.args(["--assert", &format!("param {i} %{input} {ty} p_{}", input.to_ascii_lowercase())]);
        }
        let output = common::process::required_output(&mut args);
        let out: Value = serde_json::from_slice(&output.stdout).unwrap();
        let function = &out["functions"][0];
        assert!(function["error"].is_null(), "{function}");
        let emitted = function["code"].as_str().unwrap();
        assert!(!emitted.contains("halt_"), "{function}");
        writeln!(c, "{}", emitted.replace("sub_1000", &format!("route_{index}"))).unwrap();
        let values = inputs.iter().map(|(_, value)| value.to_string()).collect::<Vec<_>>().join(",");
        writeln!(calls, "{{ old = 0xabcd; next = 0x2345; low = 0xa321; high = 0xdf23; \
                  unsigned ret = route_{index}({values}); if (!({check})) return {}; }}", index + 1).unwrap();
    }
    c.push_str(&calls);
    c.push_str("return 0;\n}\n");
    let source = binary.with_extension("c");
    let executable = binary.with_extension("exe");
    fs::write(&source, &c).unwrap();
    run_emitted_c(&source, &executable, &c);
    fs::remove_dir_all(specs).unwrap();
    for file in [binary, source, executable] {
        fs::remove_file(file).unwrap();
    }
}

#[test]
fn carmel_trap_flushes_the_next_issue_or_follows_its_vector_and_stacks_the_resume() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    const BASE: usize = 0x10800;
    const TARGET: &str = "Carmel:BE:24:default:default";
    let nop = [0x2b, 0xec, 0];
    let ret = [[0x2b, 0xd8, 0], nop];
    let a0 = [0x21, 0xac, 0x20];
    let a1 = [0x21, 0xac, 0x62];
    let a2 = [0x21, 0xac, 0xa4];
    let a3 = [0x21, 0xac, 0xe6];
    let binary = common::scratch_file("carmel-trap-entry", "bin");
    let source = binary.with_extension("c");
    let executable = binary.with_extension("exe");
    let specs = binary.with_extension("specs");
    let languages = Path::new("Ghidra/Processors/Carmel/data/languages");
    let installed = std::env::var_os("KUNA_SPECS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| common::repo_root().join("specs"))
        .join(languages);
    let fixture = specs.join(languages);
    fs::create_dir_all(&fixture).unwrap();
    for file in ["carmel.ldefs", "carmel.cspec", "carmel.sla"] {
        fs::copy(installed.join(file), fixture.join(file)).unwrap();
    }
    let pspec = fs::read_to_string(installed.join("carmel.pspec")).unwrap();
    fs::write(fixture.join("carmel.pspec"), pspec.replace("</processor_spec>",
        "<context_data><tracked_set space=\"code\">
        <set name=\"SP\" val=\"0x4040\"/><set name=\"TRAPCTX\" val=\"0\"/>
        </tracked_set></context_data></processor_spec>",
    )).unwrap();

    let sequential = |[a, b, c]: [u8; 3]| [a | 0x80, b, c];
    let mut c = String::from(
        "#include <stdbool.h>\n#include <stdint.h>\n\
         volatile unsigned short link_lo, link_hi;\n",
    );
    let mut calls = String::from("int main(void) {\n");
    let base = format!("{BASE:#x}");
    // Placement 0 is parallel, 1 is sequential left, and 2 is sequential right.
    for (i, (vector, placement, want)) in [
        (1u8, 0, [0, 0, 0, 1]),
        (2, 0, [0, 0, 1, 0]),
        (2, 1, [0, 1, 1, 0]),
        (3, 2, [0, 1, 1, 0]),
        (4, 2, [0, 1, 0, 1]),
        (255, 1, [0, 0, 0, 1]),
    ].into_iter().enumerate() {
        let trap = [0x2b, 0xd0, vector];
        let vectored = !matches!(vector, 2 | 3);
        let vector_offset = usize::from(vector) * 8;
        let mut code = vec![[nop; 2]; if vectored { vector_offset / 2 + 2 } else { 3 }];
        code[0] = match placement {
            0 => [trap, nop],
            1 => [sequential(trap), a0],
            _ => [sequential(a1), trap],
        };
        code[1] = [sequential(a1), a2];
        code[2] = ret;
        if vectored {
            code[vector_offset / 2] = [a3, nop];
            code[vector_offset / 2 + 1] = ret;
        }
        fs::write(&binary, code.as_slice().as_flattened().as_flattened()).unwrap();
        let owner = BASE + usize::from(placement == 2);
        let saved = (BASE & 0xffff) + if placement == 1 { 1 } else { 2 };
        for (r, expected) in ["A0L", "A1L", "A2L", "A3L"].into_iter().zip(want) {
            let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
            kuna.env("KUNA_SPECS", &specs).arg("decompile").arg(&binary).args([
                &base, "--addr", "--raw-image", "--target", TARGET, "--base", &base,
                "--json", "--assert-strict", "--assert", &format!("return %{r} unsigned short"),
                "--assert", "data [data,0x403f] volatile unsigned short link_lo",
                "--assert", "data [data,0x4040] volatile unsigned short link_hi",
            ]);
            for (i, reg) in ["A0L", "A1L", "A2L", "A3L", "EST", "ECFG", "INTBASE"].into_iter().enumerate() {
                kuna.args(["--assert", &format!("param {i} %{reg} unsigned short in_{i}")]);
            }
            if vectored {
                kuna.args(["--assert", &format!("jumptable {owner:#x} {:#x}", BASE + vector_offset)]);
            }
            let out = common::process::required_output(&mut kuna);
            let report: Value = serde_json::from_slice(&out.stdout).unwrap();
            let [fun] = report["functions"].as_array().unwrap().as_slice() else { panic!("{report}"); };
            let text = fun["code"].as_str().unwrap();
            assert!(fun["error"].is_null() && !text.contains("halt_"), "{fun}");
            assert!(report["assertions"].as_array().unwrap().iter().all(|a| a["status"] == "applied"), "{report}");
            let name = format!("trap_{i}_{r}");
            writeln!(c, "#define sub_10800 {name}\n{text}\n#undef sub_10800").unwrap();
            writeln!(calls,
                "link_lo = 0xabcd; link_hi = 0x5678;\
                 if ({name}(0, 0, 0, 0, 0, 0, 0x21) != {expected} || link_lo != {saved} || link_hi != 1) return {};",
                i + 1,
            ).unwrap();
        }
    }
    c.push_str("volatile unsigned short inner_lo, inner_hi;\n");
    let mut project = |entry: usize, reg: &str, name: &str, vector: bool| {
        let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
        kuna.env("KUNA_SPECS", &specs).arg("decompile").arg(&binary).args([
            &format!("{entry:#x}"), "--addr", "--raw-image", "--target", TARGET, "--base", &base,
            "--json", "--assert-strict", "--assert", &format!("return %{reg} unsigned short"),
            "--assert", "param 0 %A0L unsigned short in_a0",
            "--assert", "param 1 %EST unsigned short in_est",
            "--assert", "param 2 %ECFG unsigned short in_ecfg",
            "--assert", "param 3 %INTBASE unsigned short in_intbase",
            "--assert", "data [data,0x403d] volatile unsigned short inner_lo",
            "--assert", "data [data,0x403e] volatile unsigned short inner_hi",
            "--assert", "data [data,0x403f] volatile unsigned short link_lo",
            "--assert", "data [data,0x4040] volatile unsigned short link_hi",
        ]);
        if vector {
            kuna.args(["--assert", &format!("jumptable {BASE:#x} {:#x}", BASE + 8)]);
        }
        let out = common::process::required_output(&mut kuna);
        let report: Value = serde_json::from_slice(&out.stdout).unwrap();
        let [fun] = report["functions"].as_array().unwrap().as_slice() else { panic!("{report}"); };
        let text = fun["code"].as_str().unwrap();
        assert!(fun["error"].is_null() && !text.contains("halt_"), "{fun}");
        assert!(report["assertions"].as_array().unwrap().iter().all(|a| a["status"] == "applied"), "{report}");
        writeln!(c, "#define sub_{entry:x} {name}\n{text}\n#undef sub_{entry:x}").unwrap();
    };
    for first in 1..=3u8 {
        for second in 1..=3u8 {
            let handler = if first == 1 { 8 } else { 4 };
            let accepted = first != second && second != 3;
            let mut code = [[nop; 2]; 8];
            code[0][0] = [0x2b, 0xd0, first];
            code[handler / 2][0] = [0x2b, 0xd0, second];
            code[handler / 2 + 1][0] = a0;
            code[7] = ret;
            fs::write(&binary, code.as_flattened().as_flattened()).unwrap();
            let name = format!("nested_{first}_{second}");
            project(BASE, "A0L", &name, first == 1);
            let inner_lo = if accepted { (BASE + handler + 2) & 0xffff } else { 1 };
            let inner_hi = if accepted { BASE >> 16 } else { 2 };
            writeln!(calls,
                "inner_lo = 1; inner_hi = 2; link_lo = 3; link_hi = 4;\
                 if ({name}(0, 0, 0, 0x21) != {} || inner_lo != {inner_lo} || inner_hi != {inner_hi} ||\
                 link_lo != 0x802 || link_hi != 1) return {};",
                usize::from(!accepted), 10 + 3 * first + second,
            ).unwrap();
        }
    }
    // A left sequential TRAP saves an odd CODE word. Keep the whole fetch
    // mapped so the ordinary framer recognizes the saved right slot.
    let mut code = [[nop; 2]; 8];
    code[0] = [sequential([0x2b, 0xd0, 1]), a0];
    code[1] = ret;
    code[4][0] = [0x2b, 0xd8, 0x10];
    code[7] = ret;
    fs::write(&binary, code.as_flattened().as_flattened()).unwrap();
    project(BASE, "SP", "iret_frame", true);
    project(BASE + 1, "A0L", "iret_resume", false);
    writeln!(calls,
        "link_lo = 3; link_hi = 4;\
         if (iret_frame(0, 0, 0, 0x21) != 0x4040 || ((uint32_t)link_hi << 16 | link_lo) != {:#x}) return 30;\
         if (iret_resume(0, 0, 0, 0x21) != 1) return 31;",
        BASE + 1,
    ).unwrap();
    c.push_str(&calls);
    c.push_str("return 0;\n}\n");
    fs::write(&source, &c).unwrap();
    run_emitted_c(&source, &executable, &c);
    fs::remove_dir_all(specs).unwrap();
    for file in [binary, source, executable] {
        fs::remove_file(file).unwrap();
    }
}

#[test]
fn carmel_compact_alu_x_selector_follows_the_physical_lane() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let binary = common::scratch_file("carmel-compact-alu-x", "bin");
    let source = binary.with_extension("c");
    let executable = binary.with_extension("exe");
    let specs = binary.with_extension("specs");
    let languages = Path::new("Ghidra/Processors/Carmel/data/languages");
    let installed = std::env::var_os("KUNA_SPECS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| common::repo_root().join("specs"))
        .join(languages);
    let fixture = specs.join(languages);
    fs::create_dir_all(&fixture).unwrap();
    for file in ["carmel.ldefs", "carmel.cspec", "carmel.pspec", "carmel.sla"] {
        fs::copy(installed.join(file), fixture.join(file)).unwrap();
    }
    let target = "Carmel:BE:24:default:default";
    let nop = [0x2b, 0xec, 0];
    let add = [0x12, 0x03, 0x1d];
    let vectors = [
        ([add, nop], [0x567b, 0x5678, 0x12, 0x3000]),
        ([nop, add], [0x567d, 0x5678, 0x12, 0x3000]),
        ([[0xab, 0xec, 0], add], [0x567b, 0x5678, 0x12, 0x3000]),
        ([[0x12, 0x03, 0x1f], nop], [0x567d, 0x5678, 0x12, 0x3000]),
        ([nop, [0x12, 0x83, 0x93]], [0xaaaf, 0xffff, 0xff, 0x3400]),
        ([nop, [0x6a, 0x83, 0x93]], [0xaaaf, 0, 0, 0x3000]),
    ];
    let registers = ["A0L", "A0H", "GB0", "EST", "X1", "X2", "RN1"];
    let inputs = [0x1234, 0x5678, 0x12, 0, 3, 5, 0xaaaa];
    let negative = [0x8888, 0x8002, 0xab, 0, 0x3fda, 0x894a, 2];
    let mut c = String::from("#include <stdbool.h>\n#include <stdint.h>\n");
    let mut calls = String::from("int main(void) {\n");
    for (route, (packet, enabled)) in vectors.into_iter().enumerate() {
        let body = [packet, [nop, nop], [nop, nop], [nop, nop], [nop, nop], [[0x2b, 0xd8, 0], nop]];
        fs::write(&binary, body.as_flattened().as_flattened()).unwrap();
        for (i, reg) in registers.into_iter().enumerate() {
            let typ = if reg == "GB0" { "unsigned char" } else { "unsigned short" };
            let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
            kuna.env("KUNA_SPECS", &specs).arg("decompile").arg(&binary).args([
                "0", "--addr", "--raw-image", "--target", target, "--base", "0",
                "--json", "--assert-strict", "--assert", &format!("return %{reg} {typ}"),
            ]);
            for (j, input) in registers.into_iter().chain(["CECFG1", "CECFG2", "CETBL", "ECFG"]).enumerate() {
                let input_type = if input == "GB0" { "unsigned char" } else { "unsigned short" };
                kuna.args(["--assert", &format!("param {j} %{input} {input_type} in_{j}")]);
            }
            let out = common::process::required_output(&mut kuna);
            let report: Value = serde_json::from_slice(&out.stdout).unwrap();
            let [fun] = report["functions"].as_array().unwrap().as_slice() else { panic!("{report}"); };
            let text = fun["code"].as_str().unwrap();
            assert!(fun["error"].is_null() && !text.contains("halt_"), "{fun}");
            assert!(report["assertions"].as_array().unwrap().iter().all(|a| a["status"] == "applied"), "{report}");
            let name = format!("x_route_{route}_{i}");
            writeln!(c, "#define sub_0 {name}\n{text}\n#undef sub_0").unwrap();
            let mut tests = vec![
                (inputs, 0, 0, inputs[i]),
                (inputs, 0xffff, 0, enabled.get(i).copied().unwrap_or(inputs[i])),
            ];
            match route {
                0 => tests.push((inputs, 0xff00, 0, if reg == "EST" { 0x2000 } else { inputs[i] })),
                1 => tests.extend([
                    (inputs, 0xff, 0, if reg == "EST" { 0x1000 } else { inputs[i] }),
                    (negative, 0xffff, 0, [0x094c, 0x8002, 0xab, 0x3d40, 0x3fda, 0x894a, 2][i]),
                    (negative, 0xffff, 4, [0x8000, 0x8002, 0xab, 0x3d40, 0x3fda, 0x894a, 2][i]),
                    (negative, 0xffff, 8, [0x094c, 0x8002, 0xab, 0x3c40, 0x3fda, 0x894a, 2][i]),
                ]),
                2 => tests.push((inputs, 0xff, 0, if reg == "EST" { 0x1000 } else {
                    enabled.get(i).copied().unwrap_or(inputs[i])
                })),
                _ => {}
            }
            for (sample, (before, truth, ecfg, expected)) in tests.into_iter().enumerate() {
                let args = before.map(|v| format!("{v:#x}")).join(", ");
                writeln!(calls,
                    "if ({name}({args}, 0x1000, 0x1000, {truth:#x}, {ecfg:#x}) != {expected:#x}) return {};",
                    1 + route * 42 + i * 6 + sample,
                ).unwrap();
            }
        }
    }
    c.push_str(&calls);
    c.push_str("return 0;\n}\n");
    fs::write(&source, &c).unwrap();
    run_emitted_c(&source, &executable, &c);
    fs::remove_dir_all(specs).unwrap();
    for file in [binary, source, executable] {
        fs::remove_file(file).unwrap();
    }
}

#[test]
fn carmel_wide_mac_loop_ports_and_ce1_address_results_execute_in_emitted_c() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    const TARGET: &str = "Carmel:BE:24:default:default";
    let binary = common::scratch_file("carmel-wide-mac-loop-ports", "bin");
    let source = binary.with_extension("c");
    let executable = binary.with_extension("exe");
    let nop = [0x2b, 0xec, 0, 0x2b, 0xec, 0];
    let ret = [0x2b, 0xd8, 0, 0x2b, 0xec, 0];
    type Regs = &'static [(&'static str, u32)];
    type Tracked = &'static [(&'static str, u16)];
    type Row = ([u8; 6], Tracked, (u16, u16), Regs, Regs, u16, u16, u16);
    let r0: Tracked = &[("R0", 0x40)];
    let r4: Tracked = &[("R4", 0x48)];
    let mac_word = (0x40, 0xa258);
    let add_word = (0x48, 0x5df9);
    let mac_lsa = [0x90, 0xef, 0xa7, 0xa3, 0xd2, 0xc6];
    let mac_lea = [0x90, 0xef, 0xa7, 0xa9, 0xd3, 0xc6];
    let mac_lc = [0x90, 0xef, 0xa7, 0xa0, 0xd4, 0xc6];
    let mac_status = [0xaa, 0xa6, 0xbb, 0xaa, 0xd5, 0xc8];
    let mac_ff2 = [0x90, 0xef, 0xa7, 0xa0, 0x97, 0xc6];
    let mac_rn7 = [0xb5, 0xc3, 0xe8, 0xaf, 0xaf, 0x71];
    let add_lsa = [0x34, 0xef, 0xec, 0x91, 0x49, 0xa4];
    let add_lea = [0x34, 0xef, 0xec, 0x91, 0x49, 0xa6];
    let add_lc = [0x34, 0xef, 0xec, 0x91, 0x49, 0xa8];
    let add_status = [0x34, 0xef, 0xec, 0x91, 0x49, 0xaa];
    let add_user = [0x34, 0xef, 0xec, 0x91, 0x49, 0xae];
    let vectors: &[Row] = &[
        (mac_lsa, r0, mac_word, &[("R3", 0xc040), ("LSA1W", 0x561234)],
         &[("LSA1W", 0x56b9aa), ("X1", 0xc040), ("R0", 0x41)], 0, 0xffff, 0x3000),
        (mac_lea, r0, mac_word, &[("R3", 0xc040), ("LEA1W", 0x787654)],
         &[("LEA1W", 0x78d80b), ("X1", 0xc040), ("R0", 0x41)], 0, 0xffff, 0x3000),
        (mac_lc, r0, mac_word, &[("R3", 0xc040), ("LC1", 0x600), ("LC", 0)],
         &[("LC1", 0x1d53), ("LC", 0), ("X1", 0xc040), ("R0", 0x41)], 0, 0xffff, 0x3000),
        (mac_status, &[("R2", 0x8040), ("RN2", 3), ("LOOP_FRAME", 0)], (0x8043, 0x8be1),
         &[("PSTATUS", 0x10), ("R4", 0x48), ("LC1", 0), ("LSA1W", 0), ("LEA1W", 0)],
         &[("PSTATUS", 0xffeb), ("X1", 0x48)], 0, 0xffff, 0x3000),
        ([0x90, 0xef, 0xa7, 0xa3, 0xd6, 0xc6], r0, mac_word, &[("R3", 0xc040), ("INTBASE", 2)],
         &[("INTBASE", 0xb9aa), ("X1", 0xc040), ("R0", 0x41)], 0, 0xffff, 0x3000),
        ([0x90, 0xef, 0xa7, 0xa3, 0xd7, 0xc6], r0, mac_word, &[("R3", 0xc040), ("USER_IO", 5)],
         &[("USER_IO", 0xb9aa), ("X1", 0xc040), ("R0", 0x41)], 0, 0xffff, 0x3000),
        ([0xa0, 0xef, 0xb7, 0xa9, 0xb3, 0x0f], &[("R7", 0xc048)], (0xc048, 0xa25e),
         &[("RM2", 0), ("R0", 0x40)], &[("RM2", 0xffe8), ("X1", 0xa25e)], 0, 0xffff, 0x3000),
        (mac_rn7, &[], (0xc3e8, 0), &[("RN7", 4), ("RL6", 0)], &[("RN7", 4), ("X1", 0)], 0, 0xffff, 0x3000),
        (mac_ff2, r0, mac_word, &[("R3", 0xc040), ("FF2", 0xf678)],
         &[("FF2", 0x0dcb), ("X1", 0xc040), ("R0", 0x41)], 0, 0xffff, 0x3000),
        (add_lsa, r4, add_word, &[("LSA1W", 0)], &[("R4", 0)], 0, 0xffff, 0x3008),
        (add_lea, r4, add_word, &[("LEA1W", 0)], &[("R4", 0)], 0, 0xffff, 0x3008),
        (add_status, r4, add_word, &[("PSTATUS", 0x10)], &[("R4", 0)], 0, 0xffff, 0x3008),
        (add_user, r4, add_word, &[("USER_IO", 5)], &[("R4", 0)], 0, 0xffff, 0x3008),
        (add_lc, r4, add_word, &[("LC1", 0)], &[("R4", 0x5df9)], 0, 0xffff, 0x3000),
        (mac_lsa, &[("R0", 0x42)], (0x42, 0xffff), &[("R3", 1), ("LSA1W", 0xfe6789)],
         &[("LSA1W", 0xfe0000), ("X1", 1), ("R0", 0x43)], 0, 0xffff, 0x3000),
        (mac_lea, &[("R0", 0x42)], (0x42, 0x8001), &[("R3", 0x7634), ("LEA1W", 0xad89ab)],
         &[("LEA1W", 0xad76e2)], 7, 0xffff, 0x3000),
        (mac_lc, &[("R0", 0x42)], (0x42, 0x8000), &[("R3", 1), ("LC1", 1)],
         &[("LC1", 1)], 2, 0xffff, 0x3000),
        (mac_rn7, &[], (0xc3e8, 0x8071), &[("RN7", 0x837b), ("RL6", 0x6a79)],
         &[("RN7", 0x18a4), ("X1", 0x8071)], 7, 0xffff, 0x3000),
        (mac_status, &[("R2", 0x8041), ("RN2", 4), ("LOOP_FRAME", 0)], (0x8045, 0x8123),
         &[("PSTATUS", 0x90), ("R4", 0x4a), ("LC1", 0x3344), ("LSA1W", 0xfe6789), ("LEA1W", 0xad89ab)],
         &[("PSTATUS", 0x40), ("LC", 0x3344), ("LSAH", 0xfe), ("LSAL", 0x6789),
           ("LEAH", 0xad), ("LEAL", 0x89ab), ("X1", 0x4a)], 7, 0xffff, 0x3000),
        (add_lsa, &[("R4", 0x4a)], (0x4a, 0x8003), &[("LSA1W", 0xfe6789)],
         &[("R4", 0xfd)], 7, 0xffff, 0x3001),
        (add_lsa, &[("R4", 0x4a)], (0x4a, 0x8003), &[("LSA1W", 0xfe6789)],
         &[("R4", 0x36)], 7, 0, 0),
        (add_status, &[("R4", 0x4a)], (0x4a, 0xffff), &[("PSTATUS", 2)],
         &[("R4", 0)], 0, 0xffff, 0x3009),
        (add_user, &[("R4", 0x4a)], (0x4a, 0x8003), &[("USER_IO", 0x8123)],
         &[("R4", 0xffff)], 7, 0xffff, 0x3011),
        (add_lc, &[("R4", 0x4a)], (0x4a, 0x8003), &[("LC", 0x2222), ("LC1", 0x3344)],
         &[("R4", 0xb347)], 7, 0xffff, 0x3010),
    ];
    let c_type = |reg: &str| match reg {
        "LSA1W" | "LEA1W" => "unsigned long",
        "LSAH" | "LEAH" => "unsigned char",
        _ => "unsigned short",
    };
    let mut c = String::from(
        "#include <stdbool.h>\n#include <stdint.h>\n#include <stdio.h>\n\
         #define CONCAT12(h,l) (((uint32_t)(uint8_t)(h) << 16) | (uint16_t)(l))\n\
         #define CONCAT22(h,l) (((uint32_t)(uint16_t)(h) << 16) | (uint16_t)(l))\n",
    );
    let mut calls = String::from("int main(void) {\n");
    let mut entry_specs = std::collections::BTreeMap::new();
    for (row, &(packet, tracked, (address, word), extra, outputs, ecfg, table, flags)) in vectors.iter().enumerate() {
        fs::write(&binary, [packet, nop, nop, nop, nop, nop, ret].as_flattened()).unwrap();
        let specs = entry_specs.entry(tracked).or_insert_with(|| {
            let root = binary.with_extension(format!("specs-{row}"));
            tracked_carmel_entry_specs(&root, tracked, 0x1000, 0);
            root
        });
        let mut inputs = vec![("EST", 0), ("ECFG", u32::from(ecfg)), ("CECFG1", 0x1000),
                              ("CECFG2", 0x1000), ("CETBL", u32::from(table)),
                              ("PACFG", 0), ("RM0", 0), ("RL0", 0), ("RM4", 0), ("RL4", 0), ("X1", 0xba98)];
        inputs.extend_from_slice(extra);
        let args = inputs.iter().map(|(_, value)| value.to_string()).collect::<Vec<_>>().join(",");
        let cell = format!("mac_word_{row}");
        writeln!(c, "volatile unsigned short {cell};").unwrap();
        for (reg, expected) in std::iter::once(("EST", u32::from(flags))).chain(outputs.iter().copied()) {
            let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
            kuna.env("KUNA_SPECS", &*specs).arg("decompile").arg(&binary).args([
                "0x1000", "--addr", "--raw-image", "--target", TARGET, "--base", "0x1000",
                "--json", "--assert-strict", "--assert", &format!("return %{reg} {}", c_type(reg)),
                "--assert", &format!("data [data,{address:#x}] volatile unsigned short {cell}"),
            ]);
            for (i, (reg, _)) in inputs.iter().enumerate() {
                kuna.args(["--assert", &format!("param {i} %{reg} {} in_{}", c_type(reg), reg.to_ascii_lowercase())]);
            }
            let result = common::process::required_output(&mut kuna);
            let json: Value = serde_json::from_slice(&result.stdout).unwrap();
            let function = &json["functions"][0];
            let text = function["code"].as_str().unwrap();
            assert!(function["error"].is_null() && !text.contains("halt_"), "{function}");
            assert!(json["assertions"].as_array().unwrap().iter().all(|a| a["status"] == "applied"), "{json}");
            let name = format!("mac_loop_{row}_{}", reg.to_ascii_lowercase());
            writeln!(c, "#define sub_1000 {name}\n{text}\n#undef sub_1000").unwrap();
            writeln!(calls, "{cell} = {word};\nif ({name}({args}) != {expected} || {cell} != {word}) \
                             {{ fputs(\"{name}\\n\", stderr); return 1; }}").unwrap();
        }
    }
    for specs in entry_specs.into_values() {
        fs::remove_dir_all(specs).unwrap();
    }
    c.push_str(&calls);
    c.push_str("return 0;\n}\n");
    fs::write(&source, &c).unwrap();
    run_emitted_c(&source, &executable, &c);
    for file in [binary, source, executable] {
        fs::remove_file(file).unwrap();
    }
}

#[test]
fn carmel_pstatus_mac_refreshes_the_active_nested_frame_in_emitted_c() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    const TARGET: &str = "Carmel:BE:24:default:default";
    let binary = common::scratch_file("carmel-pstatus-mac-nesting", "bin");
    let specs = binary.with_extension("specs");
    let source = binary.with_extension("c");
    let executable = binary.with_extension("exe");
    let nop = [0x2b, 0xec, 0, 0x2b, 0xec, 0];
    let mut image = [nop; 34];
    image[0] = [0x2a, 0x03, 0xc0, 0x2b, 0xec, 0];
    image[5] = [0x2a, 0x07, 0x80, 0x2b, 0xec, 0];
    image[10] = [0xaa, 0xa6, 0xbb, 0xaa, 0xd5, 0xc8];
    image[16] = [0x2b, 0xd8, 0, 0x2b, 0xec, 0];
    fs::write(&binary, image.as_flattened()).unwrap();
    tracked_carmel_entry_specs(&specs, &[
        ("PSTATUS", 0x10), ("LOOP_FRAME", 0), ("R2", 0x8040), ("RN2", 3),
        ("R4", 0x48), ("EST", 0), ("ECFG", 0), ("CECFG1", 0x1000), ("CECFG2", 0x1000), ("CETBL", 0xffff),
        ("LOOP_PENDING1", 0), ("LOOP_PENDING2", 0), ("LOOP_PENDING3", 0), ("LOOP_PENDING4", 0),
        ("LC3", 0), ("LC4", 0), ("LSA3W", 0), ("LEA3W", 0), ("LSA4W", 0), ("LEA4W", 0),
    ], 0x1000, 0);
    let mut c = String::from(
        "#include <stdbool.h>\n#include <stdint.h>\n\
         #define CONCAT12(h,l) (((uint32_t)(uint8_t)(h) << 16) | (uint16_t)(l))\n\
         #define CONCAT22(h,l) (((uint32_t)(uint16_t)(h) << 16) | (uint16_t)(l))\n\
         volatile unsigned short mac_data;\n",
    );
    let mut calls = String::from("int main(void) {\n");
    for (i, (reg, expected)) in [("PSTATUS", 0xfff9), ("LC", 7), ("LSAL", 0x100c), ("LEAL", 0x102a),
                                ("LC1", 3), ("LC2", 7), ("X1", 0x48)].into_iter().enumerate() {
        let result = common::process::required_output(Command::new(env!("CARGO_BIN_EXE_kuna"))
            .env("KUNA_SPECS", &specs).arg("decompile").arg(&binary).args([
                "0x1000", "--addr", "--raw-image", "--target", TARGET, "--base", "0x1000",
                "--json", "--assert-strict", "--assert", &format!("return %{reg} unsigned short"),
                "--assert", "data [data,0x8043] volatile unsigned short mac_data",
            ]));
        let json: Value = serde_json::from_slice(&result.stdout).unwrap();
        let function = &json["functions"][0];
        let text = function["code"].as_str().unwrap();
        assert!(function["error"].is_null() && !text.contains("halt_"), "{function}");
        assert!(json["assertions"].as_array().unwrap().iter().all(|a| a["status"] == "applied"), "{json}");
        writeln!(c, "#define sub_1000 mac_loop_{reg}\n{text}\n#undef sub_1000").unwrap();
        writeln!(calls, "mac_data = 0x8be1; if (mac_loop_{reg}() != {expected} || mac_data != 0x8be1) return {};", i + 1)
            .unwrap();
    }
    c.push_str(&calls);
    c.push_str("return 0;\n}\n");
    fs::write(&source, &c).unwrap();
    run_emitted_c(&source, &executable, &c);
    fs::remove_dir_all(specs).unwrap();
    for file in [binary, source, executable] {
        fs::remove_file(file).unwrap();
    }
}

#[test]
fn carmel_wide_round_captures_split_data_and_gates_writeback() {
    const TARGET: &str = "Carmel:BE:24:default:default";
    const ENTRY: u32 = 0x1000;
    const NOP: [u8; 6] = [0x2b, 0xec, 0, 0x2b, 0xec, 0];
    const RET: [u8; 6] = [0x2b, 0xd8, 0, 0x2b, 0xec, 0];
    let binary = common::scratch_file("carmel-wide-round", "bin");
    let cold = [0xae, 0, 0, 0xc2, 0xd9, 0x27];
    let qualified = [0x2e, 0x80, 0xa1, 0xc2, 0xd9, 0x27];
    fs::write(&binary, [cold, RET].as_flattened()).unwrap();
    let (stdout, stderr, status) = common::run_kuna(&[
        "raw-pcode", binary.to_str().unwrap(), "0x1000", "--addr", "--raw-image",
        "--target", TARGET, "--base", "0x1000", "--count", "1", "--json",
    ]);
    let unknown: Value = serde_json::from_str(&stdout).unwrap_or_else(|_| panic!("{stdout}\n{stderr}"));
    assert_eq!(status, 1, "{unknown}");
    assert_eq!(unknown["error"]["address"], ENTRY, "{unknown}");
    assert!(unknown["instructions"].as_array().unwrap().is_empty(), "{unknown}");
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        fs::remove_file(binary).unwrap();
        return;
    }

    let specs = binary.with_extension("specs");
    let source = binary.with_extension("c");
    let executable = binary.with_extension("exe");
    let cells = [0u16, 0xffff, 0x7000, 0x80a1, 0x4040, 0x4041, 0x4042, 0x4043];
    let mut emitted = String::from("#include <stdbool.h>\n#include <stdint.h>\n");
    for addr in cells {
        writeln!(emitted, "volatile unsigned short cell_{addr:04x};").unwrap();
    }
    for (name, packet, warm, route, r1) in [
        ("cold", cold, false, 2, 0x4040),
        ("warm", [0xae, 0x80, 0xa1, 0xc2, 0xd9, 0x27], true, 3, 0x4040),
        ("short", [0xae, 0x80, 0xa1, 0xc2, 0xd8, 0x27], false, 0, 0x4040),
        ("wrap", [0x81, 0x80, 0xa1, 0xc2, 0xd9, 0x25], false, 0, 0xffff),
        ("ce1", qualified, false, 2, 0x4040),
        ("ce2", [0x6e, 0x80, 0xa1, 0xc2, 0xd9, 0x27], false, 2, 0x4040),
    ] {
        tracked_carmel_entry_specs(&specs, &[("R1", r1), ("A0L", 0x7000)], ENTRY + 2 * u32::from(warm), route);
        let mut body = Vec::new();
        if warm {
            body.extend_from_slice(&NOP);
        }
        body.extend([packet, NOP, NOP, NOP, NOP, RET].into_iter().flatten());
        fs::write(&binary, body).unwrap();
        for (register, suffix) in [("EST", "est"), ("R1", "r1")] {
            let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
            kuna.arg("decompile").arg(&binary).env("KUNA_SPECS", &specs).args([
                "0x1000", "--addr", "--raw-image", "--target", TARGET, "--base", "0x1000",
                "--json", "--assert-strict", "--assert", &format!("return %{register} unsigned short"),
            ]);
            for (i, register) in ["RN1", "RM0", "RL0", "PACFG", "ECFG", "EST", "CECFG1", "CECFG2", "CETBL"]
                .into_iter().enumerate()
            {
                kuna.args(["--assert", &format!("param {i} %{register} unsigned short in_{i}")]);
            }
            for addr in cells {
                kuna.args(["--assert", &format!("data [data,{addr:#x}] volatile unsigned short cell_{addr:04x}")]);
            }
            let output = common::process::required_output(&mut kuna);
            let json: Value = serde_json::from_slice(&output.stdout).unwrap();
            let [function] = json["functions"].as_array().unwrap().as_slice() else { panic!("{json}"); };
            assert!(function["error"].is_null(), "{function}");
            assert!(json["assertions"].as_array().unwrap().iter().all(|entry| entry["status"] == "applied"), "{json}");
            let code = function["code"].as_str().unwrap();
            assert!(!code.contains("halt_"), "{function}");
            writeln!(emitted, "{}", code.replace("sub_1000", &format!("{name}_{suffix}"))).unwrap();
        }
    }
    emitted.push_str("int main(void) {\n");
    // Each pair of calls starts with fresh DATA, including the aliased wrap destination.
    for (i, (name, cfg, table, ecfg, low, high, zero, est, r1, writes)) in [
        ("cold", 0, 0, 0, 0x8123, 0x83ef, 0x2495, 0x10, 0x4043, &[(0x4040, 0), (0x4041, 0x83f0)][..]),
        ("cold", 0, 0, 0, 0x8000, 0xffff, 0x2495, 9, 0x4043, &[(0x4040, 0), (0x4041, 0)]),
        ("cold", 0, 0, 0x20, 0x8000, 0xfffe, 0x2495, 0x10, 0x4043, &[(0x4040, 0), (0x4041, 0xffff)]),
        ("cold", 0, 0, 0, 0x8000, 0, 0x2495, 0, 0x4043, &[(0x4040, 0), (0x4041, 1)]),
        ("cold", 0, 0, 2, 0x8000, 0, 0x2495, 8, 0x4043, &[(0x4040, 0), (0x4041, 0)]),
        ("cold", 0, 0, 8, 0x8000, 0x7fff, 0x2495, 0x24, 0x4043, &[(0x4040, 0), (0x4041, 0x7fff)]),
        ("cold", 0, 0, 0x24, 0x8123, 0x7fff, 0x2495, 0x24, 0x4043, &[(0x4040, 0), (0x4041, 0x3fff)]),
        ("warm", 0, 0, 0, 0x8123, 0x83ef, 0x2495, 0, 0x4043, &[(0x4040, 0), (0x4041, 0x2496)]),
        ("short", 0, 0, 0x24, 0x8123, 0x83ef, 0x2495, 0x10, 0x4043, &[(0x4040, 0x8123)]),
        ("wrap", 0, 0, 0, 0x8123, 0x83ef, 0x1357, 0, 2, &[(0xffff, 0), (0, 0x1358)]),
        ("ce1", 0, 0, 0, 0x8123, 0x83ef, 0x2495, 0, 0x4040, &[]),
        ("ce1", 0x1000, 0, 0, 0x8123, 0x83ef, 0x2495, 0, 0x4043, &[]),
        ("ce1", 0, 1, 0, 0x8123, 0x83ef, 0x2495, 0x1010, 0x4043, &[(0x4040, 0), (0x4041, 0x83f0)]),
        ("ce2", 0, 0x100, 0, 0x8123, 0x83ef, 0x2495, 0x2010, 0x4043, &[(0x4040, 0), (0x4041, 0x83f0)]),
    ].into_iter().enumerate() {
        let before = [zero, low, high, 0xfedc, 0x5725, 0x9a7b, 0x81e7, 0x55aa];
        for (j, (suffix, result)) in [("est", est), ("r1", r1)].into_iter().enumerate() {
            for (addr, value) in cells.into_iter().zip(before) {
                writeln!(emitted, "cell_{addr:04x} = {value:#x};").unwrap();
            }
            write!(emitted, "if ({name}_{suffix}(3,0,0,0,{ecfg:#x},0,{cfg:#x},{cfg:#x},{table:#x}) != {result:#x}").unwrap();
            for (addr, value) in cells.into_iter().zip(before) {
                let after = writes.iter().find(|(cell, _)| *cell == addr).map_or(value, |(_, v)| *v);
                write!(emitted, " || cell_{addr:04x} != {after:#x}").unwrap();
            }
            writeln!(emitted, ") return {};", 1 + i * 2 + j).unwrap();
        }
    }
    emitted.push_str("return 0;\n}\n");
    fs::write(&source, &emitted).unwrap();
    run_emitted_c(&source, &executable, &emitted);
    fs::remove_dir_all(specs).unwrap();
    for file in [binary, source, executable] {
        fs::remove_file(file).unwrap();
    }
}

#[test]
fn carmel_wide_round_and_minu_distinguish_disconnected_results() {
    const TARGET: &str = "Carmel:BE:24:default:default";
    const NOP: [u8; 6] = [0x2b, 0xec, 0, 0x2b, 0xec, 0];
    const RET: [u8; 6] = [0x2b, 0xd8, 0, 0x2b, 0xec, 0];
    let binary = common::scratch_file("carmel-round-minu-ports", "bin");
    let specs = binary.with_extension("specs");
    let source = binary.with_extension("c");
    let executable = binary.with_extension("exe");
    let have_cc = common::process::optional_output(Command::new("cc").arg("--version")).is_some();
    let routes = [
        ("round_sink", [0x81, 0xda, 0x21, 0xc2, 0xdb, 0xd1], "A2L"),
        ("round_word", [0x81, 0x80, 0xa1, 0xc2, 0xda, 0x49], "A2L"),
        ("round_ce1", [0x01, 0x80, 0xa1, 0xc2, 0xda, 0x49], "A2L"),
        ("minu_sink_f", [0xcb, 0x6a, 0x45, 0xc8, 0x5f, 0x4d], "R4"),
        ("minu_sink_e", [0xcb, 0x6a, 0x45, 0xc8, 0x5e, 0x4d], "R4"),
        ("minu_long", [0xcb, 0x6a, 0x45, 0xc8, 0x5a, 0x4d], "R4"),
        ("minu_word", [0xdb, 0x6a, 0x45, 0xc8, 0x5a, 0x4d], "R4"),
    ];
    let mut c = String::from(
        "#include <stdbool.h>\n#include <stdint.h>\nvolatile unsigned short low, high;\n",
    );
    if have_cc {
        tracked_carmel_entry_specs(&specs, &[("R1", 0x4040)], 0x1000, 0);
    }
    for (name, packet, result) in routes {
        fs::write(&binary, [packet, NOP, NOP, NOP, NOP, RET].as_flattened()).unwrap();
        let (stdout, stderr, status) = common::run_kuna(&[
            "raw-pcode", binary.to_str().unwrap(), "0x1000", "--addr", "--raw-image", "--target",
            TARGET, "--base", "0x1000", "--count", "1", "--json",
        ]);
        assert_eq!(status, 0, "{stdout}\n{stderr}");
        let json: Value = serde_json::from_str(&stdout).unwrap();
        let ops = json["instructions"][0]["ops"].as_array().unwrap();
        let reads = if name.starts_with("round") { 2 } else { 0 };
        assert_eq!(ops.iter().filter(|op| op["opcode"] == "LOAD").count(), reads, "{json}");
        assert!(ops.iter().all(|op| op["opcode"] != "STORE"), "{json}");
        if !have_cc {
            continue;
        }

        for (suffix, output) in [("result", result), ("est", "EST")] {
            let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
            kuna.env("KUNA_SPECS", &specs).arg("decompile").arg(&binary).args([
                "0x1000", "--addr", "--raw-image", "--target", TARGET, "--base", "0x1000",
                "--json", "--assert-strict", "--assert", &format!("return %{output} unsigned short"),
                "--assert", "data [data,0x4040] volatile unsigned short low",
                "--assert", "data [data,0x4041] volatile unsigned short high",
            ]);
            for (i, input) in ["A2L", "R4", "A2H", "GB3", "A3H", "A3L", "ECFG", "EST",
                "CECFG1", "CECFG2", "CETBL"].into_iter().enumerate()
            {
                let kind = if input == "GB3" { "unsigned char" } else { "unsigned short" };
                kuna.args(["--assert", &format!("param {i} %{input} {kind} in_{i}")]);
            }
            let output = common::process::required_output(&mut kuna);
            let json: Value = serde_json::from_slice(&output.stdout).unwrap();
            let function = &json["functions"][0];
            let code = function["code"].as_str().unwrap();
            assert!(function["error"].is_null() && !code.contains("halt_"), "{function}");
            assert!(json["assertions"].as_array().unwrap().iter().all(|a| a["status"] == "applied"), "{json}");
            writeln!(c, "{}", code.replace("sub_1000", &format!("{name}_{suffix}"))).unwrap();
        }
    }
    fs::remove_file(binary).unwrap();
    if !have_cc {
        return;
    }
    c.push_str("int main(void) {\n");
    for (i, (name, lo, hi, ecfg, table, a2h, a3g, a3h, a3l, result, est)) in [
        ("round_sink", 0x8001, 0xffff, 0, 0xffff, 0, 0, 0, 0, 0x4321, 0x3000),
        ("round_word", 0x8000, 0x1022, 2, 0, 0, 0, 0, 0, 0x1022, 0),
        ("round_word", 0x8000, 0x1023, 2, 0xffff, 0, 0, 0, 0, 0x1024, 0x3000),
        ("round_word", 0x8001, 0xffff, 0, 0xffff, 0, 0, 0, 0, 0, 0x3009),
        ("round_word", 0x8001, 0x7fff, 4, 0xffff, 0, 0, 0, 0, 0x7fff, 0x3024),
        ("round_word", 0x8001, 0x7fff, 8, 0xffff, 0, 0, 0, 0, 0x8000, 0x3020),
        ("round_ce1", 0xabc9, 0xa102, 0, 0, 0, 0, 0, 0, 0x4321, 0),
        ("round_ce1", 0xabc9, 0xa102, 0, 0xffff, 0, 0, 0, 0, 0xa103, 0x3010),
        ("minu_sink_f", 0x0123, 0x8123, 0, 0xffff, 0x1234, 0, 0, 0x1234, 0x48, 0x3000),
        ("minu_sink_e", 0x0123, 0x8123, 0, 0, 0x9876, 0, 0, 0x1234, 0x48, 0),
        ("minu_long", 0x0123, 0x8123, 0, 0xffff, 0x1234, 0, 0, 0x1234, 0, 0x3009),
        ("minu_long", 0x0123, 0x8123, 0, 0xffff, 0x1234, 0x87, 0, 0x1234, 0, 0x3000),
        ("minu_long", 0x0123, 0x8123, 0, 0, 0x9876, 0, 0, 0x1234, 0, 1),
        ("minu_word", 0x0123, 0x8123, 0, 0xffff, 0xffec, 0x14, 0x1234, 0xffed, 0x1234, 0x3001),
    ].into_iter().enumerate() {
        let args = format!("0x4321,0x48,{a2h:#x},{a3g:#x},{a3h:#x},{a3l:#x},{ecfg:#x},0,0x1000,0x1000,{table:#x}");
        for (j, (suffix, expected)) in [("result", result), ("est", est)].into_iter().enumerate() {
            writeln!(c,
                "low = {lo:#x}; high = {hi:#x};\n\
                 if ({name}_{suffix}({args}) != {expected:#x} || low != {lo:#x} || high != {hi:#x}) return {};",
                1 + 2 * i + j).unwrap();
        }
    }
    c.push_str(
        "low = 0x0123; high = 0x8123;\n\
         if (round_sink_est(0x4321,0x48,0,0,0,0,0,0x24,0x1000,0x1000,0xffff) != 0x3024 ||\n\
             round_word_est(0x4321,0x48,0,0,0,0,0,0x24,0x1000,0x1000,0xffff) != 0x3014 ||\n\
             minu_sink_f_est(0,0x48,0x1234,0,0,0x1234,0,0x24,0x1000,0x1000,0xffff) != 0x3024 ||\n\
             minu_long_est(0,0x48,0x1234,0,0,0x1234,0,0x24,0x1000,0x1000,0xffff) != 0x302d) return 29;\n",
    );
    c.push_str("return 0;\n}\n");
    fs::write(&source, &c).unwrap();
    run_emitted_c(&source, &executable, &c);
    fs::remove_dir_all(specs).unwrap();
    for file in [source, executable] {
        fs::remove_file(file).unwrap();
    }
}

#[test]
fn carmel_compact_secondary_rotate_only_updates_its_address_generators() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let binary = common::scratch_file("carmel-compact-rotate-agu", "bin");
    let source = binary.with_extension("c");
    let executable = binary.with_extension("exe");
    let nop = [0x2b, 0xec, 0, 0x2b, 0xec, 0];
    let ret = [0x2b, 0xd8, 0, 0x2b, 0xec, 0];
    let or = [0x6a, 0xa3, 0xe4];
    let xor = [0x6c, 0xe5, 0x1f];
    let rol = [0x21, 0xc1, 0x17];
    let sequential_rol = [[0xa1, 0xc1, 0x17], [0x2b, 0xec, 0]];
    let add_carry = [0x12, 0x03, 0x1d, 0x2b, 0xec, 0];
    let variants = [
        (nop, [or, rol], 0xc040, 0xc048, 0x1000, 0, [0xc044, 0x48, 0xc048, 0xfedd, 0x4567, 0x8888, 0, 0x5678]),
        (nop, [[0x2b, 0xec, 0], [0x21, 0xc1, 0x57]], 0xc040, 0xc048, 0, 0xff00, [0xc044, 0x48, 0xc048, 0x789a, 0x4567, 0x8888, 0x2000, 0x5678]),
        (nop, [rol, or], 0xc040, 0xc048, 0x1000, 0xffff, [0xc044, 0x48, 0xc048, 0xfedd, 0x4567, 0x8888, 0x3001, 2]),
        (nop, sequential_rol, 0xc040, 0xc048, 0x1000, 0xffff, [0xc044, 0x48, 0xc048, 0x789a, 0x4567, 0x8888, 0x3001, 2]),
        (nop, [xor, [0x21, 0xc3, 0x17]], 0x4040, 0x4040, 0x1000, 0xffff, [0x4042, 0x4b, 0x4042, 0x789a, 0, 0x8888, 0x3008, 0x5678]),
        (nop, [xor, [0x21, 0xe1, 0x17]], 0x4040, 0x4040, 0, 0xff00, [0x4042, 0x48, 0x4040, 0x789a, 0x4567, 0x8888, 0x2000, 0x5678]),
        ([0x2b, 0xec, 0, 0x12, 0x03, 0x1d], [[0x2b, 0xec, 0], rol], 0xc040, 0xc048, 0x1000, 0xffff, [0xc044, 0x48, 0xc048, 0x789a, 0x4567, 0x094c, 0x3d40, 0x5678]),
        (add_carry, sequential_rol, 0xc040, 0xc048, 0x1000, 0xffff, [0xc044, 0x48, 0xc048, 0x789a, 0x4567, 1, 0x3001, 3]),
    ];
    let projections: [&[usize]; 8] = [
        &[0, 3, 6], &[0, 6], &[0, 3, 6], &[0, 6],
        &[0, 1, 2, 4, 6], &[0, 1, 2, 4, 6], &[0, 5, 6], &[0, 5, 6],
    ];
    let mut c = String::from(
        "#include <stdbool.h>\n#include <stdint.h>\n\
         volatile unsigned short rotate_in, rotate_out, shared_word, or_word, other_out;\n",
    );
    let mut calls = String::from("int main(void) {\n");
    let registers = ["R3", "R4", "R7", "A1L", "A4H", "A0L", "EST"];
    for (i, (prefix, packet, r3, r7, cfg, table, after)) in variants.into_iter().enumerate() {
        let specs = binary.with_extension(format!("specs-{i}"));
        tracked_carmel_entry_specs(&specs, &[("R3", r3), ("R4", 0x48), ("R5", 0x4048), ("R7", r7),
            ("RN3", if r3 == 0x4040 { 2 } else { 4 }), ("RN4", 3),
            ("RN7", if r7 == 0x4040 { 2 } else { 4 }), ("PACFG", 0)], 0x1000, 0);
        let [left, right] = packet;
        let mut program = vec![prefix, [left[0], left[1], left[2], right[0], right[1], right[2]]];
        program.extend([nop, nop, nop, nop, ret]);
        fs::write(&binary, program.as_slice().as_flattened()).unwrap();
        let with_carry = prefix == add_carry;
        let input = [("A0L", 0x8888), ("A0H", if with_carry { 0xffff } else { 0x8002 }), ("GB0", 0xab),
                     ("A1L", 0x789a), ("A2L", 0xfedc), ("A4L", 0x89ab), ("A4H", 0x4567),
                     ("X1", 2), ("X2", 0x894a), ("EST", 0), ("ECFG", 0), ("CECFG1", cfg),
                     ("CECFG2", cfg), ("CETBL", table)];
        let args = input.iter().map(|(_, value)| value.to_string()).collect::<Vec<_>>().join(", ");
        for &j in projections[i] {
            let reg = registers[j];
            let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
            kuna.env("KUNA_SPECS", &specs).arg("decompile").arg(&binary).args([
                "0x1000", "--addr", "--raw-image", "--target", "Carmel:BE:24:default:default", "--base", "0x1000",
                "--json", "--assert-strict", "--assert", &format!("return %{reg} unsigned short"),
                "--assert", "data [data,0xc040] volatile unsigned short rotate_in",
                "--assert", "data [data,0x48] volatile unsigned short rotate_out",
                "--assert", "data [data,0x4040] volatile unsigned short shared_word",
                "--assert", "data [data,0xc04c] volatile unsigned short or_word",
                "--assert", "data [data,0x4048] volatile unsigned short other_out",
            ]);
            for (k, (field, _)) in input.into_iter().enumerate() {
                let ty = if field == "GB0" { "unsigned char" } else { "unsigned short" };
                kuna.args(["--assert", &format!("param {k} %{field} {ty} in_{k}")]);
            }
            let result = common::process::required_output(&mut kuna);
            let json: Value = serde_json::from_slice(&result.stdout).unwrap();
            let function = &json["functions"][0];
            let text = function["code"].as_str().unwrap();
            assert!(function["error"].is_null() && !text.contains("halt_"), "{function}");
            assert!(json["assertions"].as_array().unwrap().iter().all(|a| a["status"] == "applied"), "{json}");
            let name = format!("rotate_{i}_{j}");
            writeln!(c, "#define sub_1000 {name}\n{text}\n#undef sub_1000").unwrap();
            writeln!(calls,
                "rotate_in = 0x8001; rotate_out = 0x5678; shared_word = 0x89ab; or_word = 0x2005; other_out = 0x22cc;\n\
                 if ({name}({args}) != {} || rotate_out != {} || rotate_in != 0x8001 || shared_word != 0x89ab ||\
                     or_word != 0x2005 || other_out != 0x22cc) return {};",
                after[j], after[7], i * 7 + j + 1).unwrap();
        }
        fs::remove_dir_all(specs).unwrap();
    }
    c.push_str(&calls);
    c.push_str("return 0;\n}\n");
    fs::write(&source, &c).unwrap();
    run_emitted_c(&source, &executable, &c);
    for file in [binary, source, executable] {
        fs::remove_file(file).unwrap();
    }
}

#[test]
fn carmel_compact_mpy_samples_physical_inputs_and_publishes_x1() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    type Registers = &'static [(&'static str, u16)];
    const R0_POST: Registers = &[("R0", 0x100), ("PACFG", 0), ("RM0", 0), ("RL0", 0)];
    const R1_POST: Registers = &[("R1", 0x120), ("PACFG", 0), ("RM0", 0), ("RL0", 0)];
    const BOTH_POST: Registers = &[("R0", 0x100), ("R1", 0x120), ("PACFG", 0), ("RM0", 0), ("RL0", 0)];
    const R2_POST: Registers = &[("R2", 0x140), ("PACFG", 0), ("RM2", 0), ("RL2", 0)];
    const R0_INDIRECT: Registers = &[("R0", 0x100)];
    let nop = [0x2b, 0xec, 0, 0x2b, 0xec, 0];
    let ret = [0x2b, 0xd8, 0, 0x2b, 0xec, 0];
    let cells = [(0x100, 101), (0x101, 0x4444), (0x103, 104), (0x120, 117),
                 (0x121, 0x5555), (0x124, 121), (0x140, 0xffff), (0x141, 0x6666)];
    let seeds = [("A0L", 0x4321), ("A0H", 5), ("GB0", 0), ("A2L", 9), ("X1", 17), ("X2", 19),
                 ("R0", 0x100), ("R1", 0x120), ("R2", 0x140), ("RN2", 5), ("FF1", 11), ("FF2", 13),
                 ("EST", 0), ("ECFG", 0), ("CECFG1", 0), ("CECFG2", 0), ("CETBL", 0)];
    // Raw selector, tracked AGU seed, live register overrides, DATA[R0], product (or
    // preserved A0), X1, pointer updates as a bitmask, EST, and count of DATA reads.
    let rows: [(u32, Registers, Registers, u16, u64, u16, u8, u16, u8); 24] = [
        (0x416024, &[], &[], 101, 2880, 320, 0, 0, 0), // R2, not DATA[R2].
        (0x4160a4, &[], &[], 101, 45, 5, 0, 0, 0), // RN2.
        (0x416164, &[], &[], 101, 99, 11, 0, 0, 0), // FF1.
        (0x416144, &[], &[], 101, 153, 17, 0, 0, 0), // Incoming X1.
        (0x410400, R0_POST, &[], 101, 25856, 101, 1, 0, 1), // DATA[old R0] * old R0.
        (0x410428, R0_POST, &[], 101, 1717, 101, 1, 0, 1), // DATA[old R0] * old X1.
        (0x41042e, R0_POST, &[], 101, 1313, 101, 1, 0, 1), // Second FF2 does not latch.
        (0x418422, BOTH_POST, &[], 101, 11817, 101, 3, 0, 2),
        (0x41d022, R1_POST, &[], 101, 1989, 17, 2, 0, 1),
        (0x414024, &[("R0", 0x100), ("RN0", 3)], &[], 101, 936, 104, 0, 0, 1),
        (0x41f402, &[("R1", 0x120), ("RN1", 4)], &[], 101, 2057, 17, 0, 0, 1),
        (0x41506c, &[], &[], 101, 121, 11, 0, 0, 0), // OP3 squares FF1.
        (0x41d064, R2_POST, &[], 101, 1, 0xffff, 4, 0, 1), // Signed square of -1.
        (0x4dd064, R2_POST, &[], 101, 0xfffe0001, 0xffff, 4, 0x20, 1),
        (0x4dd064, R2_POST, &[("ECFG", 5)], 101, 0x1_fffc_0002, 0xffff, 4, 0x20, 1),
        (0x4dd064, R2_POST, &[("EST", 0x4fff)], 101, 0xfffe0001, 0xffff, 4, 0x4fe4, 1),
        (0x4561e4, &[], &[], 101, 0, 17, 0, 8, 0), // Mixed-sign OP1 does not connect the other input.
        (0x4961f4, &[], &[("A2L", 0x8000)], 101, 0, 17, 0, 8, 0), // Neither does OP2.
        (0x427024, R0_INDIRECT, &[("A2L", 0x1234)], 101, 0x54321, 7, 0, 0, 1),
        (0x410400, R0_POST, &[("EST", 0x1b7)], 0xff00, 0xff_ffff_0000, 0xff00, 1, 0x10, 1),
        (0x410428, R0_POST, &[("EST", 0x1b7), ("X1", 0xffff)], 0xff00, 256, 0xff00, 1, 0, 1),
        (0x427024, R0_INDIRECT, &[("A2L", 1), ("EST", 0xcfc4), ("CETBL", 0xffff)],
            1, 0x54321, 0, 0, 0x3008, 1), // Narrow result is zero, full product is not.
        (0x427024, R0_INDIRECT, &[("A2L", 0x8000), ("ECFG", 1)], 0x8000, 0x54321, 0x8000, 0, 0x24, 1),
        (0x427024, R0_INDIRECT, &[("A2L", 0x8000), ("ECFG", 5)], 0x8000, 0x54321, 0x7fff, 0, 0x24, 1),
    ];
    let binary = common::scratch_file("carmel-compact-mpy-latch", "bin");
    let source = binary.with_extension("c");
    let executable = binary.with_extension("exe");
    let mut c = String::from(
        "#include <stdbool.h>\n#include <stdint.h>\n\
         #define CONCAT12(h,l) (((uint32_t)(uint8_t)(h) << 16) | (uint16_t)(l))\n\
         #define CONCAT22(h,l) (((uint32_t)(uint16_t)(h) << 16) | (uint16_t)(l))\n\
         static unsigned reads;\n\
         static volatile unsigned short *port(volatile unsigned short *p) { ++reads; return p; }\n",
    );
    let mut calls = String::from("int main(void) {\n");
    for (address, _) in cells {
        writeln!(c, "static volatile unsigned short backing_{address:x};\n\
                     #define cell_{address:x} (*port(&backing_{address:x}))").unwrap();
    }
    let mut spec_dirs = Vec::new();
    let c_type = |reg: &str| if reg.starts_with("GB") { "unsigned char" } else { "unsigned short" };
    for (i, (raw, tracked, inputs, data0, a0, x1, post, est, access_count)) in rows.into_iter().enumerate() {
        let word = raw.to_be_bytes();
        let packet = [word[1], word[2], word[3], 0x2b, 0xec, 0];
        let specs = if tracked.is_empty() {
            None
        } else {
            let path = binary.with_extension(format!("specs-{i}"));
            tracked_carmel_entry_specs(&path, tracked, 0, 0);
            Some(path)
        };
        let params = seeds.iter().filter(|(reg, _)| !tracked.iter().any(|(r, _)| reg == r))
            .map(|&(reg, seed)| (reg, inputs.iter().find(|&&(r, _)| r == reg).map_or(seed, |&(_, n)| n)))
            .collect::<Vec<_>>();
        let arguments = params.iter().map(|&(_, n)| n.to_string()).collect::<Vec<_>>().join(",");
        let outputs = [("A0L", a0 as u16), ("A0H", (a0 >> 16) as u16), ("GB0", (a0 >> 32) as u16),
                       ("X1", x1), ("X2", 19), ("R0", 0x100 + u16::from(post & 1)),
                       ("R1", 0x120 + u16::from((post >> 1) & 1)),
                       ("R2", 0x140 + u16::from((post >> 2) & 1)), ("EST", est)];
        let body = [packet, nop, ret];
        // Independent copies let one load emit each return projection.
        fs::write(&binary, body.as_flattened().repeat(outputs.len())).unwrap();
        let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
        if let Some(path) = &specs {
            kuna.env("KUNA_SPECS", path);
        }
        kuna.arg("decompile-all").arg(&binary).args([
            "--raw-image", "--target", "Carmel:BE:24:default:default", "--base", "0",
            "--json", "--jobs", "1", "--assert-strict",
        ]);
        for (addr, _) in cells {
            kuna.args(["--assert", &format!("data [data,{addr:#x}] volatile unsigned short cell_{addr:x}")]);
        }
        let names = std::array::from_fn::<_, 9, _>(|projection| {
            let addr = projection * body.as_flattened().len() / 3;
            let function = format!("sub_{addr:x}");
            let (reg, _) = outputs[projection];
            kuna.args(["--addr", &format!("{addr:#x}"), "--assert",
                       &format!("return {function}::%{reg} {}", c_type(reg))]);
            for (n, &(input, _)) in params.iter().enumerate() {
                kuna.args(["--assert", &format!("param {function}::{n} %{input} {} in_{n}", c_type(input))]);
            }
            function
        });
        let out = common::process::required_output(&mut kuna);
        let json: Value = serde_json::from_slice(&out.stdout).unwrap();
        let functions = json["functions"].as_array().unwrap();
        assert_eq!(functions.len(), outputs.len(), "{json}");
        assert!(json["assertions"].as_array().unwrap().iter().all(|a| a["status"] == "applied"), "{json}");
        for (((reg, expected), old_name), fun) in outputs.into_iter().zip(names).zip(functions) {
            assert_eq!(fun["name"], old_name, "{fun}");
            let text = fun["code"].as_str().unwrap();
            assert!(fun["error"].is_null() && !text.contains("halt_"), "{raw:06x} {reg}: {fun}");
            let name = format!("mpy_{i}_{reg}");
            writeln!(c, "#define {old_name} {name}\n{text}\n#undef {old_name}").unwrap();
            for (addr, seed) in cells {
                let value = if addr == 0x100 { data0 } else { seed };
                writeln!(calls, "backing_{addr:x} = {value:#x};").unwrap();
            }
            writeln!(calls, "reads = 0;\nif ({name}({arguments}) != {expected:#x}) return {};", i + 1).unwrap();
            if reg == if raw == 0x427024 { "X1" } else { "A0L" } {
                writeln!(calls, "if (reads != {access_count}) return {};", i + 32).unwrap();
            }
            for (addr, seed) in cells {
                let value = if addr == 0x100 { data0 } else { seed };
                writeln!(calls, "if (backing_{addr:x} != {value:#x}) return {};", i + 64).unwrap();
            }
        }
        spec_dirs.extend(specs);
    }
    c.push_str(&calls);
    c.push_str("return 0;\n}\n");
    fs::write(&source, &c).unwrap();
    run_emitted_c(&source, &executable, &c);
    for path in spec_dirs {
        fs::remove_dir_all(path).unwrap();
    }
    for file in [binary, source, executable] {
        fs::remove_file(file).unwrap();
    }
}

#[test]
fn carmel_compact_secondary_extract_does_not_consume_the_data_ports() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let binary = common::scratch_file("carmel-compact-extract-agu", "bin");
    let source = binary.with_extension("c");
    let executable = binary.with_extension("exe");
    let specs = binary.with_extension("specs");
    tracked_carmel_entry_specs(&specs, &[("R3", 0x4040), ("R5", 0x4048), ("R7", 0x4040),
        ("RN3", 4), ("RN5", 3), ("RN7", 2), ("PACFG", 0)], 0x1000, 0);
    let nop = [0x2b, 0xec, 0, 0x2b, 0xec, 0];
    let ret = [0x2b, 0xd8, 0, 0x2b, 0xec, 0];
    let primary = [0x36, 0xc4, 0xa7, 0x2b, 0xec, 0];
    let variants = [
        ([0x6c, 0xe5, 0x1f, 0x36, 0xc4, 0xb7], 0x1000, 0, 4, 12, [0x4044, 0x4048, 0x4042, 0x9999, 0x4567, 0xf123, 0xa5, 0]),
        ([0x6c, 0xe5, 0x1f, 0x36, 0x44, 0xa7], 0, 0xff00, 4, 12, [0x4040, 0x404b, 0x4040, 0x9999, 0x4567, 0xf123, 0xa5, 0x2000]),
        ([0x6c, 0xe5, 0x1f, 0x34, 0xc4, 0xa7], 0x1000, 0xffff, 4, 12, [0x4040, 0x4048, 0x4042, 0x78a5, 0x4567, 0xf123, 0xa5, 0x3000]),
        (primary, 0x1000, 0xffff, 4, 12, [0x4040, 0x4048, 0x4040, 0x9999, 9, 0, 0, 0x3000]),
        (primary, 0x1000, 0xffff, 16, 16, [0x4040, 0x4048, 0x4040, 0x9999, 0xffff, 0, 0, 0x3000]),
        (primary, 0x1000, 0, 16, 16, [0x4040, 0x4048, 0x4040, 0x9999, 0x4567, 0xf123, 0xa5, 0]),
    ];
    let projections: [&[usize]; 6] = [
        &[0, 2, 3, 4], &[1, 2, 4, 7], &[0, 3, 4, 7],
        &[4, 5, 6, 7], &[4, 5, 6, 7], &[4, 5, 6, 7],
    ];
    let outputs = ["R3", "R5", "R7", "A4H", "A2L", "A2H", "GB2", "EST"];
    let mut c = String::from(
        "#include <stdbool.h>\n#include <stdint.h>\n\
         volatile unsigned short width_word, offset_word, short_input;\n",
    );
    let mut calls = String::from("int main(void) {\n");
    for (i, (packet, cfg, table, width, offset, after)) in variants.into_iter().enumerate() {
        fs::write(&binary, [packet, nop, nop, nop, nop, ret].as_flattened()).unwrap();
        let inputs = [("A2L", 0x4567), ("A2H", 0xf123), ("GB2", 0xa5), ("A4L", 0x78a1),
                      ("A4H", 0x9999), ("A5L", 0x91ae), ("EST", 0), ("ECFG", 0),
                      ("CECFG1", cfg), ("CECFG2", cfg), ("CETBL", table)];
        let args = inputs.iter().map(|(_, value)| value.to_string()).collect::<Vec<_>>().join(", ");
        for &j in projections[i] {
            let reg = outputs[j];
            let ty = if reg == "GB2" { "unsigned char" } else { "unsigned short" };
            let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
            kuna.env("KUNA_SPECS", &specs).arg("decompile").arg(&binary).args([
                "0x1000", "--addr", "--raw-image", "--target", "Carmel:BE:24:default:default", "--base", "0x1000",
                "--json", "--assert-strict", "--assert", &format!("return %{reg} {ty}"),
                "--assert", "data [data,0x4040] volatile unsigned short width_word",
                "--assert", "data [data,0x4041] volatile unsigned short offset_word",
                "--assert", "data [data,0x4048] volatile unsigned short short_input",
            ]);
            for (k, (field, _)) in inputs.into_iter().enumerate() {
                let ty = if field == "GB2" { "unsigned char" } else { "unsigned short" };
                kuna.args(["--assert", &format!("param {k} %{field} {ty} in_{k}")]);
            }
            let result = common::process::required_output(&mut kuna);
            let json: Value = serde_json::from_slice(&result.stdout).unwrap();
            let fun = &json["functions"][0];
            let text = fun["code"].as_str().unwrap();
            assert!(fun["error"].is_null() && !text.contains("halt_"), "{fun}");
            assert!(json["assertions"].as_array().unwrap().iter().all(|a| a["status"] == "applied"), "{json}");
            let name = format!("extract_{i}_{j}");
            writeln!(c, "#define sub_1000 {name}\n{text}\n#undef sub_1000").unwrap();
            writeln!(calls,
                "width_word = {width}; offset_word = {offset}; short_input = 0x8181;\n\
                 if ({name}({args}) != {} || width_word != {width} || offset_word != {offset} || short_input != 0x8181) return {};",
                after[j], i * 8 + j + 1).unwrap();
        }
    }
    c.push_str(&calls);
    c.push_str("return 0;\n}\n");
    fs::write(&source, &c).unwrap();
    run_emitted_c(&source, &executable, &c);
    fs::remove_dir_all(specs).unwrap();
    for file in [binary, source, executable] {
        fs::remove_file(file).unwrap();
    }
}

#[test]
fn carmel_wide_load_execution_ports_and_retired_status_execute_in_emitted_c() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    const TARGET: &str = "Carmel:BE:24:default:default";
    const RET: [u8; 6] = [0x2b, 0xd8, 0, 0x2b, 0xec, 0];
    let binary = common::scratch_file("carmel-load-execution-ports", "bin");
    let source = binary.with_extension("c");
    let executable = binary.with_extension("exe");
    let ff1 = [0x10, 0xfa, 2, 0xc2, 0x7a, 0x5a];
    let x1 = [0x10, 0xfa, 2, 0xc2, 0x7a, 0x7a];
    let btr1 = [0x10, 0xfa, 2, 0xc2, 0x7b, 0x1a];
    let early_est = [0x10, 0xfa, 2, 0xc2, 0x7b, 0x06];
    let inc = [0x21, 0xac, 0x20, 0x2b, 0xec, 0];
    let mut c = String::from("#include <stdbool.h>\n#include <stdint.h>\n");
    for (body, regs, projections) in [
        (
            &[ff1, x1, btr1, RET][..],
            &["A0L", "CECFG1", "CECFG2", "CETBL", "EST", "PACFG", "FF1", "X1", "BTR1"][..],
            &["FF1", "X1", "BTR1", "PACFG"][..],
        ),
        (
            &[early_est, inc, RET][..],
            &["A0L", "CECFG1", "CECFG2", "CETBL", "EST", "ECFG"][..],
            &["A0L", "EST"][..],
        ),
    ] {
        fs::write(&binary, body.as_flattened()).unwrap();
        for reg in projections {
            let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
            kuna.arg("decompile").arg(&binary).args([
                "0x1000", "--addr", "--raw-image", "--target", TARGET, "--base", "0x1000",
                "--json", "--assert-strict", "--assert", &format!("return %{reg} unsigned short"),
            ]);
            for (i, input) in regs.iter().enumerate() {
                kuna.args(["--assert", &format!("param {i} %{input} unsigned short in_{i}")]);
            }
            let output = common::process::required_output(&mut kuna);
            let report: Value = serde_json::from_slice(&output.stdout).unwrap();
            let [function] = report["functions"].as_array().unwrap().as_slice() else { panic!("{report}"); };
            assert!(function["error"].is_null(), "{function}");
            assert!(report["assertions"].as_array().unwrap().iter().all(|r| r["status"] == "applied"), "{report}");
            let code = function["code"].as_str().unwrap();
            assert!(!code.contains("halt_"), "{function}");
            writeln!(c, "{}", code.replace("sub_1000", &format!("load_{reg}"))).unwrap();
        }
    }
    c.push_str("int main(void) {\n");
    for (i, (cfg, table, ff, x, btr, pacfg)) in [
        (0x1000, 0, 0xb679, 0xb679, 0x2222, 0x2000),
        (0x1000, 1, 0xb679, 0xb679, 0xb679, 0x2000),
        (0, 0, 0x0033, 0x0055, 0x2222, 0),
    ].into_iter().enumerate() {
        let args = format!("0xb679,{cfg:#x},0x1000,{table:#x},0,0,0x33,0x55,0x2222");
        writeln!(c, "if (load_FF1({args}) != {ff:#x} || load_X1({args}) != {x:#x} || \
                     load_BTR1({args}) != {btr:#x} || load_PACFG({args}) != {pacfg:#x}) return {};", i + 1).unwrap();
    }
    c.push_str("if (load_A0L(0xb679,0x1000,0x1000,0xffff,0,1) != 0xb67a || \
                    load_EST(0xb679,0x1000,0x1000,0xffff,0,1) != 0x3010) return 4;\n\
                if (load_A0L(0xffff,0x1000,0x1000,1,0,1) != 0 || \
                    load_EST(0xffff,0x1000,0x1000,1,0,1) != 0x1009) return 5;\n\
                return 0;\n}\n");
    fs::write(&source, &c).unwrap();
    run_emitted_c(&source, &executable, &c);
    for file in [binary, source, executable] {
        fs::remove_file(file).unwrap();
    }
}

#[test]
fn carmel_wide_limit_source_lanes_execute_in_emitted_c() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    const TARGET: &str = "Carmel:BE:24:default:default";
    const NOP: [u8; 6] = [0x2b, 0xec, 0, 0x2b, 0xec, 0];
    const RET: [u8; 6] = [0x2b, 0xd8, 0, 0x2b, 0xec, 0];
    const INPUTS: [&str; 14] = [
        "A0L", "A0H", "GB0", "A1L", "A1H", "GB1", "FF2", "EST",
        "ECFG", "CECFG1", "CECFG2", "CETBL", "LC", "LC1",
    ];
    let binary = common::scratch_file("carmel-wide-limit", "bin");
    let specs = binary.with_extension("specs");
    let source = binary.with_extension("c");
    let executable = binary.with_extension("exe");
    let tracked = [
        ("R1", 0x4040), ("R3", 0xc044), ("PACFG", 0),
        ("RM0", 0), ("RL0", 0), ("RM2", 0), ("RL2", 0),
    ];
    let implicit = [0x8f, 0x23, 0xe1, 0xc2, 0xec, 0xc6];
    fs::write(&binary, [implicit, RET].as_flattened()).unwrap();
    let (stdout, stderr, status) = common::run_kuna(&[
        "raw-pcode", binary.to_str().unwrap(), "0x1000", "--addr", "--raw-image",
        "--target", TARGET, "--base", "0x1000", "--count", "1", "--json",
    ]);
    let unknown: Value = serde_json::from_str(&stdout).unwrap_or_else(|_| panic!("{stdout}\n{stderr}"));
    assert_eq!(status, 1, "{unknown}");
    assert_eq!(unknown["error"]["address"], 0x1000, "{unknown}");
    assert!(unknown["instructions"].as_array().unwrap().is_empty(), "{unknown}");
    let mut emitted = String::from(
        "#include <stdbool.h>\n#include <stdint.h>\n\
         #define CONCAT12(h,l) (((uint32_t)(uint8_t)(h) << 16) | (uint16_t)(l))\n\
         #define CONCAT14(h,l) (((uint64_t)(uint8_t)(h) << 32) | (uint32_t)(l))\n\
         #define CONCAT22(h,l) (((uint32_t)(uint16_t)(h) << 16) | (uint16_t)(l))\n\
         #define CONCAT44(h,l) (((uint64_t)(uint32_t)(h) << 32) | (uint32_t)(l))\n\
         volatile unsigned short destination, old_source, advanced_source, implicit_ffff, implicit_zero;\n",
    );
    let c_type = |register: &str| {
        if register.starts_with("GB") { "unsigned char" } else { "unsigned short" }
    };
    for (name, packet, route, outputs) in [
        ("half", [0x88, 0x41, 0x23, 0xc2, 0xee, 0x46], 0, &["A1L", "A1H", "GB1", "EST"][..]),
        ("whole", [0x88, 0x41, 0x23, 0xc2, 0xee, 0x26], 0, &["A1L", "A1H", "GB1", "EST"][..]),
        ("word", [0x97, 0x41, 0x23, 0xc2, 0xee, 0x26], 0, &["A1L", "A1H", "GB1", "EST"][..]),
        ("cold", implicit, 2, &["R1", "EST"][..]),
        ("routed", implicit, 3, &["R1", "EST"][..]),
        ("sink", [0x08, 0x41, 0x23, 0xc2, 0xe3, 0xa2], 0, &["EST"][..]),
        ("loop", [0x08, 0x41, 0x23, 0xc2, 0xe3, 0x52], 0, &["LC", "LC1", "EST"][..]),
        ("pointer", [0x73, 0x11, 0xff, 0xc2, 0xe3, 0x60], 0, &["R3", "EST"][..]),
    ] {
        tracked_carmel_entry_specs(&specs, &tracked, 0x1000, route);
        fs::write(&binary, [packet, NOP, NOP, NOP, NOP, RET].as_flattened()).unwrap();
        for register in outputs {
            let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
            kuna.arg("decompile").arg(&binary).env("KUNA_SPECS", &specs).args([
                "0x1000", "--addr", "--raw-image", "--target", TARGET, "--base", "0x1000",
                "--json", "--assert-strict", "--assert", &format!("return %{register} {}", c_type(register)),
            ]);
            for (i, input) in INPUTS.into_iter().enumerate() {
                kuna.args(["--assert", &format!("param {i} %{input} {} in_{i}", c_type(input))]);
            }
            for (addr, name) in [(0x4040, "destination"), (0xc044, "old_source"), (0xc043, "advanced_source"),
                                 (0xffff, "implicit_ffff"), (0, "implicit_zero")] {
                kuna.args(["--assert", &format!("data [data,{addr:#x}] volatile unsigned short {name}")]);
            }
            let output = common::process::required_output(&mut kuna);
            let json: Value = serde_json::from_slice(&output.stdout).unwrap();
            let [function] = json["functions"].as_array().unwrap().as_slice() else { panic!("{json}"); };
            assert!(function["error"].is_null(), "{function}");
            assert!(json["assertions"].as_array().unwrap().iter().all(|a| a["status"] == "applied"), "{json}");
            let code = function["code"].as_str().unwrap();
            assert!(!code.contains("halt_"), "{function}");
            writeln!(emitted, "#define sub_1000 {name}_{register}\n{code}\n#undef sub_1000").unwrap();
        }
    }
    emitted.push_str("int main(void) {\n");
    let mut check = 0;
    let mut project = |name: &str, results: &[(&str, u16)], acc: u64, ff2: u16,
                       est: u16, cfg2: u16, table: u16, stored: u16| {
        let address_zero = ff2 ^ 0x8000;
        let flag_word = if name == "word" { ff2 } else { ff2 ^ 0x4a93 };
        let args = [
            acc as u16, (acc >> 16) as u16, (acc >> 32) as u16, 0x61a5, 0xbeef, 0xac,
            flag_word, est, 0x24, 0, cfg2, table, 0x1234, 0xa69c,
        ].map(|v| v.to_string()).join(", ");
        for &(register, expected) in results {
            check += 1;
            writeln!(emitted, "destination = 0x557e; old_source = 0x9a73; advanced_source = 0x3012;\n\
                implicit_ffff = {ff2:#x}; implicit_zero = {address_zero:#x};\n\
                if ({name}_{register}({args}) != {expected:#x} || destination != {stored:#x} || \
                old_source != 0x9a73 || advanced_source != 0x3012 || implicit_ffff != {ff2:#x} || \
                implicit_zero != {address_zero:#x}) return {check};").unwrap();
        }
    };
    for (acc, half, whole, half_flags, whole_flags) in [
        (1, 0, 1u64, 8, 0),
        (0x7fff_fffe, 0x7fff, 0x7fff_fffe, 0, 0),
        (0x7fff_ffff, 0x7fff, 0x7fff_ffff, 0, 0),
        (0x8000_0000, 0x7fff, 0x7fff_ffff, 0x24, 0x24),
        (0xff_7fff_ffff, 0x8000, 0xff_8000_0000, 0x34, 0x34),
        (0xff_8000_0000, 0x8000, 0xff_8000_0000, 0x10, 0x10),
        (0xff_8000_0001, 0x8000, 0xff_8000_0001, 0x10, 0x10),
    ] {
        project("half", &[("A1L", half), ("A1H", 0xbeef), ("GB1", 0xac), ("EST", 0x3000 | half_flags)],
                acc, 0x8765, 0, 0, 0xffff, 0x557e);
        project("whole", &[("A1L", whole as u16), ("A1H", (whole >> 16) as u16),
                            ("GB1", (whole >> 32) as u16), ("EST", 0x3000 | whole_flags)],
                acc, 0x8765, 0, 0, 0xffff, 0x557e);
    }
    project("word", &[("A1L", 0x8765), ("A1H", 0xffff), ("GB1", 0xff), ("EST", 0x3010)],
            0x01_2345_6789, 0x8765, 0, 0, 0xffff, 0x557e);
    project("cold", &[("R1", 0x4021), ("EST", 0x3008)], 0x01_2345_6789, 0, 0, 0, 0xffff, 0);
    project("cold", &[("R1", 0x4021), ("EST", 0x3010)], 0x01_2345_6789, 0x924b, 0, 0, 0xffff, 0x924b);
    project("routed", &[("R1", 0x4021), ("EST", 0x3000)], 0x01_2345_6789, 0x924b, 0, 0, 0xffff, 0x124b);
    project("sink", &[("EST", 0x1014)], 0xff_8000_0000, 0, 0x34, 0, 1, 0x557e);
    project("sink", &[("EST", 0x101c)], 0, 0, 0x3f, 0, 0xff, 0x557e);
    project("sink", &[("EST", 0x1024)], 0x80_0000_0000, 0, 0, 0, 1, 0x557e);
    project("loop", &[("LC", 0x1234), ("LC1", 0x7fff), ("EST", 0x1024)],
            0x8000_0000, 0, 0, 0, 1, 0x557e);
    project("loop", &[("LC", 0x1234), ("LC1", 0xa69c), ("EST", 0x2000)],
            0x8000_0000, 0, 0, 0, 0x100, 0x557e);
    project("pointer", &[("R3", 0xc043), ("EST", 0x2010)], 0, 0, 0, 0, 0x100, 0x557e);
    project("pointer", &[("R3", 0xc044), ("EST", 0x1000)], 0, 0, 0, 0, 1, 0x557e);
    project("pointer", &[("R3", 0xc043), ("EST", 0x1000)], 0, 0, 0, 0x1000, 1, 0x557e);
    emitted.push_str("return 0;\n}\n");
    fs::write(&source, &emitted).unwrap();
    run_emitted_c(&source, &executable, &emitted);
    fs::remove_dir_all(specs).unwrap();
    for file in [binary, source, executable] {
        fs::remove_file(file).unwrap();
    }
}

#[test]
fn carmel_raw_limit_and_round_data_ports_execute_in_emitted_c() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    const NOP: [u8; 6] = [0x2b, 0xec, 0, 0x2b, 0xec, 0];
    const RET: [u8; 6] = [0x2b, 0xd8, 0, 0x2b, 0xec, 0];
    let binary = common::scratch_file("carmel-raw-limit-round", "bin");
    let (specs, source, executable) =
        (binary.with_extension("specs"), binary.with_extension("c"), binary.with_extension("exe"));
    tracked_carmel_entry_specs(&specs, &[
        ("R5", 0x4048), ("RN5", 2), ("R6", 0x8048), ("RN6", 3), ("A0L", 0x09a4), ("PACFG", 0),
    ], 0x1000, 2);
    let cells = [
        (0x4048, "dst_low"), (0x4049, "dst_high"), (0xffff, "src_low"),
        (0x09a4, "src_high"), (0x8048, "unindexed"), (0x804b, "indexed"),
    ];
    let mut c = String::from("#include <stdbool.h>\n#include <stdint.h>\n");
    for (_, name) in cells {
        writeln!(c, "volatile unsigned short {name};").unwrap();
    }
    for (name, packet, result) in [
        ("short", [0xe0, 0x46, 0xf5, 0xc2, 0xe0, 0x37], "R5"),
        ("long", [0xe7, 0x46, 0xf5, 0xc2, 0xe1, 0x37], "R5"),
        ("round", [0x2e, 0x49, 0xeb, 0xc2, 0xde, 0xf8], "FF1"),
    ] {
        fs::write(&binary, [packet, NOP, NOP, NOP, NOP, RET].as_flattened()).unwrap();
        for output in ["EST", result] {
            let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
            kuna.arg("decompile").arg(&binary).env("KUNA_SPECS", &specs).args([
                "0x1000", "--addr", "--raw-image", "--target", "Carmel:BE:24:default:default",
                "--base", "0x1000", "--json", "--assert-strict",
                "--assert", &format!("return %{output} unsigned short"),
            ]);
            for (i, input) in ["FF1", "ECFG", "EST", "CECFG1", "CECFG2", "CETBL"].into_iter().enumerate() {
                kuna.args(["--assert", &format!("param {i} %{input} unsigned short in_{i}")]);
            }
            for (address, cell) in cells {
                kuna.args(["--assert", &format!("data [data,{address:#x}] volatile unsigned short {cell}")]);
            }
            let output_json = common::process::required_output(&mut kuna);
            let document: Value = serde_json::from_slice(&output_json.stdout).unwrap();
            let function = &document["functions"][0];
            let code = function["code"].as_str().unwrap();
            assert!(function["error"].is_null() && !code.contains("halt_"), "{document}");
            writeln!(c, "#define sub_1000 {name}_{output}\n{code}\n#undef sub_1000").unwrap();
        }
    }
    c.push_str("int main(void) {\n\
        src_low = 0x8888; src_high = 0x7fff; dst_high = 0xbeef;\n\
        if (short_R5(0,0x20,0,0,0,0) != 0x404a || dst_low != 0x8888 || dst_high != 0xbeef) return 1;\n\
        if (short_EST(0,0x20,0,0,0,0xffff) != 0x3010) return 2;\n\
        if (long_R5(0,0x20,0,0,0,0) != 0x404a || dst_low != 0x8888 || dst_high != 0x7fff) return 3;\n\
        if (long_EST(0,9,0,0,0,0xffff) != 0x3000) return 4;\n\
        src_low = 0; src_high = 0;\n\
        if (long_EST(0,9,0,0,0,0xffff) != 0x3008 || dst_low != 0 || dst_high != 0) return 5;\n\
        unindexed = 0x1231; indexed = 0xc331;\n\
        if (round_FF1(0x1234,0,0,0,0,0) != 0x1234 || round_EST(0x1234,0,0,0,0,0) != 0) return 6;\n\
        if (round_FF1(0x1234,0x20,0,0,0,0xffff) != 0xc331 || round_EST(0x1234,0,0,0,0,0xffff) != 0x3010) return 7;\n\
        if (unindexed != 0x1231 || indexed != 0xc331) return 8;\n\
        return 0;\n}\n");
    fs::write(&source, &c).unwrap();
    run_emitted_c(&source, &executable, &c);
    fs::remove_dir_all(specs).unwrap();
    for file in [binary, source, executable] {
        fs::remove_file(file).unwrap();
    }
}

#[test]
fn carmel_raw_divide_separates_read_and_address_only_ports() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    const TARGET: &str = "Carmel:BE:24:default:default";
    const NOP: [u8; 6] = [0x2b, 0xec, 0, 0x2b, 0xec, 0];
    const RET: [u8; 6] = [0x2b, 0xd8, 0, 0x2b, 0xec, 0];
    let binary = common::scratch_file("carmel-raw-divide-effects", "bin");
    let (specs, source, executable) =
        (binary.with_extension("specs"), binary.with_extension("c"), binary.with_extension("exe"));
    tracked_carmel_entry_specs(&specs, &[("R2", 0x8040), ("RN2", 3), ("PACFG", 0)], 0x1000, 0);
    let mut c = String::from(
        "#include <stdbool.h>\n#include <stdint.h>\n\
         volatile unsigned short low_divisor, high_divisor;\n",
    );
    for (name, packet, result, reads) in [
        ("agu", [0x7e, 0x0d, 0x36, 0xc3, 0x18, 0x6a], "R2", 0),
        ("sink_e7", [0xaa, 0xc4, 0x15, 0xc3, 0x03, 0x9d], "EST", 2),
        ("sink_ff", [0xaa, 0xc4, 0x15, 0xc3, 0x03, 0xfd], "EST", 2),
    ] {
        fs::write(&binary, [packet, NOP, NOP, NOP, NOP, RET].as_flattened()).unwrap();
        let (stdout, stderr, status) = common::run_kuna(&[
            "raw-pcode", binary.to_str().unwrap(), "0x1000", "--addr", "--raw-image",
            "--target", TARGET, "--base", "0x1000", "--count", "1", "--json",
        ]);
        assert_eq!(status, 0, "{stdout}\n{stderr}");
        let doc: Value = serde_json::from_str(&stdout).unwrap();
        let ops = doc["instructions"][0]["ops"].as_array().unwrap();
        assert_eq!(ops.iter().filter(|op| op["opcode"] == "LOAD").count(), reads, "{doc}");
        assert!(ops.iter().all(|op| op["opcode"] != "STORE"), "{doc}");

        let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
        kuna.arg("decompile").arg(&binary).env("KUNA_SPECS", &specs).args([
            "0x1000", "--addr", "--raw-image", "--target", TARGET, "--base", "0x1000",
            "--json", "--assert-strict", "--assert", &format!("return %{result} unsigned short"),
            "--assert", "data [data,0x8043] volatile unsigned short low_divisor",
            "--assert", "data [data,0x8044] volatile unsigned short high_divisor",
        ]);
        for (i, input) in ["EST", "CECFG1", "CECFG2", "CETBL"].into_iter().enumerate() {
            kuna.args(["--assert", &format!("param {i} %{input} unsigned short in_{i}")]);
        }
        let output = common::process::required_output(&mut kuna);
        let doc: Value = serde_json::from_slice(&output.stdout).unwrap();
        let function = &doc["functions"][0];
        let code = function["code"].as_str().unwrap();
        assert!(function["error"].is_null() && !code.contains("halt_"), "{doc}");
        for cell in ["low_divisor", "high_divisor"] {
            assert_eq!(code.contains(cell), reads != 0, "{code}");
        }
        writeln!(c, "#define sub_1000 {name}\n{code}\n#undef sub_1000").unwrap();
    }
    c.push_str("int main(void) {\n\
        if (agu(0,0x1000,0,0) != 0x8040) return 1;\n\
        if (agu(0,0,0x1000,0) != 0x803f || agu(0,0,0,0xffff) != 0x803f) return 2;\n\
        if (sink_e7(0x42,0,0,0) != 0x42 || sink_ff(0x42,0,0,0xffff) != 0x3042) return 3;\n\
        return 0;\n}\n");
    fs::write(&source, &c).unwrap();
    run_emitted_c(&source, &executable, &c);
    fs::remove_dir_all(specs).unwrap();
    for file in [binary, source, executable] {
        fs::remove_file(file).unwrap();
    }
}

#[test]
fn carmel_compact_extrema_keep_compare_width_and_record_only_strict_improvements() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let binary = common::scratch_file("carmel-compact-extrema", "bin");
    let source = binary.with_extension("c");
    let executable = binary.with_extension("exe");
    let specs = binary.with_extension("specs");
    tracked_carmel_entry_specs(&specs, &[("R0", 0x100), ("PACFG", 0)], 0x1000, 0);
    let nop = [0x2b, 0xec, 0, 0x2b, 0xec, 0];
    let ret = [0x2b, 0xd8, 0, 0x2b, 0xec, 0];
    let operands = [
        ("A0L", "unsigned short"), ("A0H", "unsigned short"), ("GB0", "unsigned char"),
        ("A1L", "unsigned short"), ("A1H", "unsigned short"), ("GB1", "unsigned char"),
        ("BTR1", "unsigned short"), ("MINMAX", "unsigned short"),
        ("EST", "unsigned short"), ("ECFG", "unsigned short"), ("CECFG1", "unsigned short"),
        ("CECFG2", "unsigned short"), ("CETBL", "unsigned short"),
    ];
    let min = ([0xffff, 0xffff, 0x7f, 0, 0, 0x80], 0, 0, [0, 0x80, 0xcccc, 0x7777, 6, 0x100]);
    let min_saturated = (min.0, 4, 0, [0, 0x80, 0xcccc, 0x7777, 2, 0x100]);
    let max = ([0, 0, 0x80, 0xffff, 0xffff, 0x7f], 0, 0, [0xffff, 0x7f, 0xcccc, 0x7777, 0x16, 0x100]);
    let equal = ([5, 0, 0, 5, 0, 0], 0, 0, [5, 0, 0x9998, 0x7777, 8, 0x100]);
    let smaller = ([5, 0, 0, 3, 0, 0], 0, 0, [3, 0, 0x9999, 0x7777, 0x10, 0x100]);
    let data_win = ([0xffff, 0x1234, 0xff, 0, 0, 0], 0, 1, [1, 0xff, 0xcccc, 0x100, 0, 0x101]);
    let data_tie = ([0xffff, 0x1234, 0xff, 0, 0, 0], 0, 0xffff, [0xffff, 0xff, 0xcccc, 0x7777, 8, 0x101]);
    let variants = [
        ([0x22, 0x8f, 0x82], &[min, min_saturated][..], &[0, 1, 4][..]),
        ([0x23, 0x8f, 0x82], &[max][..], &[0, 1, 4][..]),
        ([0x22, 0x0f, 0x82], &[equal, smaller][..], &[0, 2, 4][..]),
        ([0x23, 0xd8, 0x20], &[data_win, data_tie][..], &[0, 1, 3, 4, 5][..]),
    ];
    let outputs = ["A0L", "GB0", "BTR1", "MINMAX", "EST", "R0"];
    let mut c = String::from(
        "#include <stdbool.h>\n#include <stdint.h>\n\
         #define CONCAT22(h,l) (((uint32_t)(uint16_t)(h) << 16) | (uint16_t)(l))\n\
         volatile unsigned short old_word;\n",
    );
    let mut calls = String::from("int main(void) {\n");
    for (i, ([a, b, d], cases, projections)) in variants.into_iter().enumerate() {
        fs::write(&binary, [[a, b, d, 0x2b, 0xec, 0], nop, nop, nop, nop, ret].as_flattened()).unwrap();
        for &j in projections {
            let reg = outputs[j];
            let ty = if reg == "GB0" { "unsigned char" } else { "unsigned short" };
            let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
            kuna.env("KUNA_SPECS", &specs).arg("decompile").arg(&binary).args([
                "0x1000", "--addr", "--raw-image", "--target", "Carmel:BE:24:default:default", "--base", "0x1000",
                "--json", "--assert-strict", "--assert", &format!("return %{reg} {ty}"),
                "--assert", "data [data,0x100] volatile unsigned short old_word",
            ]);
            for (k, (field, c_type)) in operands.into_iter().enumerate() {
                kuna.args(["--assert", &format!("param {k} %{field} {c_type} in_{k}")]);
            }
            let out = common::process::required_output(&mut kuna);
            let json: Value = serde_json::from_slice(&out.stdout).unwrap();
            let fun = &json["functions"][0];
            let body = fun["code"].as_str().unwrap();
            assert!(fun["error"].is_null() && !body.contains("halt_"), "{fun}");
            assert!(json["assertions"].as_array().unwrap().iter().all(|a| a["status"] == "applied"), "{json}");
            let name = format!("extremum_{i}_{j}");
            writeln!(c, "#define sub_1000 {name}\n{body}\n#undef sub_1000").unwrap();
            for &(input, cfg, word, expected) in cases {
                let args = input.map(|value: u16| format!("{value:#x}")).join(", ");
                writeln!(calls,
                    "old_word = {word}; if ({name}({args}, 0xcccc, 0x7777, 0, {cfg}, 0x1000, 0x1000, 0) != {} || old_word != {word}) return {};",
                    expected[j], 1 + i * outputs.len() + j).unwrap();
            }
        }
    }
    c.push_str(&calls);
    c.push_str("return 0;\n}\n");
    fs::write(&source, &c).unwrap();
    run_emitted_c(&source, &executable, &c);
    fs::remove_dir_all(specs).unwrap();
    for file in [binary, source, executable] {
        fs::remove_file(file).unwrap();
    }
}

#[test]
fn carmel_compact_load_routes_address_config_and_pointer_writeback() {
    const NOP: [u8; 6] = [0x2b, 0xec, 0, 0x2b, 0xec, 0];
    const RET: [u8; 6] = [0x2b, 0xd8, 0, 0x2b, 0xec, 0];
    let left = [0x2c, 0, 0xad, 0x2b, 0xec, 0];
    let right = [0x2b, 0xec, 0, 0x2c, 0, 0xad];
    let rows = [
        (left, 0x1000, 0x1000, 0, [0x5559, 0xb99a, 0x8048, 0x7122, 0]),
        ([0xab, 0xec, 0, 0x2c, 0, 0xad], 0x1000, 0, 0, [0x5559, 0xb99a, 0x8048, 0x7122, 0]),
        (right, 0, 0x1000, 0, [0x5559, 0xb99a, 0x8048, 0x7122, 0]),
        (left, 0, 0x1000, 0, [0x5555, 0x4048, 0x8048, 0x7122, 0]),
        ([0x2c, 0, 0xab, 0x2c, 1, 0x2d], 0, 0x1000, 0, [0x5565, 0x4048, 0xb99a, 0x7122, 0]),
        (left, 0, 0x1000, 1, [0x5559, 0xb99a, 0x8048, 0x7122, 0x1000]),
        (right, 0x1000, 0, 0x0100, [0x5559, 0xb99a, 0x8048, 0x7122, 0x2000]),
        ([0x2c, 3, 0xec, 0x2b, 0xec, 0], 0, 0, 0, [0x9555, 0x4048, 0x8048, 0x4f23, 0]),
        ([0x2c, 0, 0xab, 0x2c, 1, 0x2d], 0x1000, 0x1000, 0, [0x5565, 0, 0, 0, 0]),
    ];
    let binary = common::scratch_file("carmel-compact-load", "bin");
    let source = binary.with_extension("c");
    let executable = binary.with_extension("exe");
    let live = [("PACFG", 0x5555), ("R1", 0x4048), ("R2", 0x8048), ("R7", 0x7122),
                ("A5L", 0xb99a), ("A5H", 0x4f23), ("EST", 0)];
    let values = live.iter().map(|(_, val)| val.to_string()).collect::<Vec<_>>().join(",");
    let mut c = String::from(
        "#include <stdint.h>\n#include <stdbool.h>\n#include <stdlib.h>\n\
         static unsigned short expected_conflict;\n\
         _Noreturn static void carmel_address_config_conflict(unsigned short pacfg) {\n\
             exit(expected_conflict && pacfg == expected_conflict ? 0 : 71);\n\
         }\n",
    );
    let mut calls = String::from("int main(void) {\n");
    for (i, (fetch, ce1, ce2, table, expected)) in rows.into_iter().enumerate() {
        let body = [fetch, NOP, NOP, NOP, NOP, RET];
        let contention = i + 1 == rows.len();
        let outputs = ["PACFG", "R1", "R2", "R7", "EST"];
        let count = if contention { 1 } else { outputs.len() };
        fs::write(&binary, body.as_flattened().repeat(count)).unwrap();
        let specs = binary.with_extension(format!("specs-{i}"));
        tracked_carmel_entry_specs(&specs, &[("CECFG1", ce1), ("CECFG2", ce2), ("CETBL", table)], 0x1000, 0);
        let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
        kuna.env("KUNA_SPECS", &specs).arg("decompile-all").arg(&binary).args([
            "--raw-image", "--target", "Carmel:BE:24:default:default", "--base", "0x1000",
            "--json", "--jobs", "1", "--assert-strict",
        ]);
        let mut names = Vec::with_capacity(count);
        for (j, reg) in outputs.into_iter().take(count).enumerate() {
            let addr = 0x1000 + j * body.as_flattened().len() / 3;
            let name = format!("sub_{addr:x}");
            kuna.args(["--addr", &format!("{addr:#x}"), "--assert", &format!("return {name}::%{reg} unsigned short")]);
            for (k, (input, _)) in live.into_iter().enumerate() {
                kuna.args(["--assert", &format!("param {name}::{k} %{input} unsigned short in_{k}")]);
            }
            names.push(name);
        }
        let result = common::process::required_output(&mut kuna);
        let json: Value = serde_json::from_slice(&result.stdout).unwrap();
        let functions = json["functions"].as_array().unwrap();
        assert_eq!(functions.len(), count, "{json}");
        assert!(json["assertions"].as_array().unwrap().iter().all(|a| a["status"] == "applied"), "{json}");
        for (j, ((function, name), want)) in functions.iter().zip(names).zip(expected).enumerate() {
            assert_eq!(function["name"], name, "{function}");
            let text = function["code"].as_str().unwrap();
            assert!(function["error"].is_null() && !text.contains("halt_"), "{function}");
            let unique = format!("load_{i}_{j}");
            writeln!(c, "#define {name} {unique}\n{text}\n#undef {name}").unwrap();
            if contention {
                writeln!(calls, "expected_conflict = {want:#x}; {unique}({values}); return 72;").unwrap();
            } else {
                writeln!(calls, "if ({unique}({values}) != {want:#x}) return {};", i * outputs.len() + j + 1).unwrap();
            }
        }
        fs::remove_dir_all(&specs).unwrap();
    }
    calls.push_str("}\n");
    c.push_str(&calls);
    fs::write(&source, &c).unwrap();
    run_emitted_c(&source, &executable, &c);
    for file in [binary, source, executable] {
        fs::remove_file(file).unwrap();
    }
}

#[test]
fn carmel_wide_signed_immediate_move_keeps_address_and_retired_result_ports_separate() {
    const NOP: [u8; 6] = [0x2b, 0xec, 0, 0x2b, 0xec, 0];
    const RET: [u8; 6] = [0x2b, 0xd8, 0, 0x2b, 0xec, 0];
    let rows: &[(u32, u8, u16, u16, u16, &[(&str, u16)], u16)] = &[
        (0x00b679, 0x08, 0x1000, 0, 0, &[("R0", 0x48)], 0xaaaa),
        (0x00b679, 0x08, 0, 0, 0, &[("R0", 0x45)], 0xaaaa),
        (0x80b679, 0x08, 0x1000, 0, 0, &[("R0", 0x48)], 0xdb3c),
        (0x92b679, 0x8e, 0x1000, 0, 0, &[("EST", 0x10), ("A0L", 0x4567)], 0xaaaa),
        (0x00b679, 0xba, 0x1000, 0, 0, &[("MINMAX", 0xb679)], 0xaaaa),
        (0x00b679, 0xc0, 0x1000, 0, 0, &[("ECFG", 0x39)], 0xaaaa),
        (0x00b679, 0xc1, 0x1000, 0, 0x14, &[("EST", 0x14)], 0xaaaa),
        (0x80b679, 0xc2, 0x1000, 0xffff, 0, &[("CECFG1", 0x3679), ("EST", 0x2000)], 0xaaaa),
        (0x00b679, 0xc4, 0x1000, 0, 0, &[("CETBL", 0xb679), ("EST", 0x1000)], 0xaaaa),
        (0x00b679, 0xd5, 0x1000, 0, 0, &[("PSTATUS", 0x19), ("LC", 0x1234), ("LSAL", 0x2233), ("LEAL", 0x7745)], 0xaaaa),
        (0x00b679, 0xd7, 0x1000, 0, 0, &[("USER_IO", 0xb679)], 0xaaaa),
        (0x00b679, 0xd7, 0, 0, 0, &[("USER_IO", 7)], 0xaaaa),
        (0x92b679, 0xda, 0, 0, 0, &[("PACFG", 0xb679)], 0xaaaa),
    ];
    let binary = common::scratch_file("carmel-wide-signed-imm", "bin");
    let source = binary.with_extension("c");
    let executable = binary.with_extension("exe");
    let specs = binary.with_extension("specs");
    let mut c = String::from("#include <stdbool.h>\n#include <stdint.h>\nvolatile unsigned short data_word;\n");
    let mut calls = String::from("int main(void) {\n");
    for (i, &(prefix, destination, ce1, table, est, outputs, memory)) in rows.iter().enumerate() {
        let left = prefix.to_be_bytes();
        let right = (0xc65000 | (u32::from(destination) << 2)).to_be_bytes();
        let fetch = [left[1], left[2], left[3], right[1], right[2], right[3]];
        let body = [fetch, NOP, NOP, NOP, NOP, RET];
        fs::write(&binary, body.as_flattened().repeat(outputs.len())).unwrap();
        tracked_carmel_entry_specs(&specs, &[
            ("CECFG1", ce1), ("CECFG2", 0), ("CETBL", table), ("ECFG", 0x20), ("R0", 0x45),
            ("RN0", 3), ("RM0", 0), ("RL0", 0), ("PACFG", 0), ("PSTATUS", 0x10), ("LC1", 0x1234),
            ("LSA1H", 0x11), ("LSA1L", 0x2233), ("LEA1H", 0x3a), ("LEA1L", 0x7745),
            ("A0L", 0x4567), ("USER_IO", 7), ("LOOP_FRAME", 0),
        ], 0x1000, 0);
        let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
        kuna.env("KUNA_SPECS", &specs).arg("decompile-all").arg(&binary).args([
            "--raw-image", "--target", "Carmel:BE:24:default:default", "--base", "0x1000",
            "--json", "--jobs", "1", "--assert-strict",
            "--assert", "data [data,0x45] volatile unsigned short data_word",
        ]);
        let mut names = Vec::with_capacity(outputs.len());
        for (j, (reg, _)) in outputs.iter().enumerate() {
            let addr = 0x1000 + j * body.as_flattened().len() / 3;
            let name = format!("sub_{addr:x}");
            kuna.args(["--addr", &format!("{addr:#x}"), "--assert", &format!("return {name}::%{reg} unsigned short"),
                       "--assert", &format!("param {name}::0 %EST unsigned short flags")]);
            names.push(name);
        }
        let result = common::process::required_output(&mut kuna);
        let json: Value = serde_json::from_slice(&result.stdout).unwrap();
        let functions = json["functions"].as_array().unwrap();
        assert_eq!(functions.len(), outputs.len(), "{json}");
        assert!(json["assertions"].as_array().unwrap().iter().all(|a| a["status"] == "applied"), "{json}");
        for (j, ((function, name), (_, want))) in functions.iter().zip(names).zip(outputs).enumerate() {
            assert_eq!(function["name"], name, "{function}");
            let text = function["code"].as_str().unwrap();
            assert!(function["error"].is_null() && !text.contains("halt_"), "{function}");
            let unique = format!("movs_{i}_{j}");
            writeln!(c, "#define {name} {unique}\n{text}\n#undef {name}").unwrap();
            writeln!(calls, "data_word = 0xaaaa; if ({unique}({est:#x}) != {want:#x} || data_word != {memory:#x}) return {};",
                     i * 4 + j + 1).unwrap();
        }
        fs::remove_dir_all(&specs).unwrap();
    }
    c.push_str(&calls);
    c.push_str("return 0;\n}\n");
    fs::write(&source, &c).unwrap();
    run_emitted_c(&source, &executable, &c);
    for file in [binary, source, executable] {
        fs::remove_file(file).unwrap();
    }
}

#[test]
fn carmel_wide_rotate_routes_through_carry_and_physical_writeback() {
    const NOP: [u8; 6] = [0x2b, 0xec, 0, 0x2b, 0xec, 0];
    const RET: [u8; 6] = [0x2b, 0xd8, 0, 0x2b, 0xec, 0];
    let rows: &[([u8; 6], bool, u16, u16, &[(&str, u16)])] = &[
        ([0xc1, 0x77, 0x6d, 0xc2, 0x3e, 0x89], false, 0xffff, 0x11, &[("R2", 0), ("EST", 0x3011)]),
        ([0xd1, 0x77, 0x6d, 0xc2, 0x3e, 0x85], false, 0, 1, &[("R1", 0), ("EST", 1)]),
        ([0xd1, 0x77, 0x6d, 0xc2, 0x3e, 0x84], false, 0, 1, &[("R1", 0xa340), ("EST", 0x11)]),
        ([0xc1, 0x77, 0x6d, 0xc2, 0x3e, 0x25], false, 0, 1,
            &[("A1L", 0xa340), ("A1H", 0xc091), ("GB1", 0xff), ("EST", 0x11)]),
        ([0xc1, 0x77, 0x6d, 0xc2, 0x3a, 0x89], false, 0, 1, &[("R2", 0x247), ("EST", 1)]),
        ([0xc1, 0x77, 0x6d, 0xc2, 0x3a, 0x89], false, 0, 0, &[("R2", 0x246), ("EST", 1)]),
        ([0xc1, 0x77, 0x6d, 0xc2, 0x3e, 0x89], true, 0, 0, &[("R2", 0), ("A0L", 0xecd0), ("EST", 0x10)]),
    ];
    let binary = common::scratch_file("carmel-wide-rotate-ports", "bin");
    let source = binary.with_extension("c");
    let executable = binary.with_extension("exe");
    let specs = binary.with_extension("specs");
    let mut c = String::from("#include <stdint.h>\n#include <stdbool.h>\nvolatile unsigned short low_word, high_word;\n");
    let mut calls = String::from("int main(void) {\n");
    for (i, &(fetch, warm, table, est, outputs)) in rows.iter().enumerate() {
        let mut body = Vec::new();
        if warm {
            body.push([0x8f, 0x10, 0, 0xf5, 0x11, 0x13]);
            body.extend([NOP; 9]);
        }
        body.extend([fetch, NOP, NOP, NOP, NOP, RET]);
        fs::write(&binary, body.as_slice().as_flattened().repeat(outputs.len())).unwrap();
        tracked_carmel_entry_specs(&specs, &[("R1", 0x4040), ("R2", 0x8040), ("PACFG", 0), ("RM0", 0), ("RL0", 0),
            ("RN1", 2), ("CETBL", table), ("CECFG1", 0), ("CECFG2", 0),
            ("A1L", 0xecd0), ("A1H", 0x834c), ("GB1", 0x6a)], 0x1000, 0);
        let mut cmd = Command::new(env!("CARGO_BIN_EXE_kuna"));
        cmd.env("KUNA_SPECS", &specs).arg("decompile-all").arg(&binary).args([
            "--raw-image", "--target", "Carmel:BE:24:default:default", "--base", "0x1000",
            "--json", "--jobs", "1", "--assert-strict",
            "--assert", "data [data,0x4040] volatile unsigned short low_word",
            "--assert", "data [data,0x4041] volatile unsigned short high_word",
        ]);
        let mut names = Vec::with_capacity(outputs.len());
        for (j, (reg, _)) in outputs.iter().enumerate() {
            let addr = 0x1000 + 2 * j * body.len();
            let name = format!("sub_{addr:x}");
            let ty = if *reg == "GB1" { "unsigned char" } else { "unsigned short" };
            cmd.args(["--addr", &format!("{addr:#x}"), "--assert", &format!("return {name}::%{reg} {ty}"),
                      "--assert", &format!("param {name}::0 %EST unsigned short initial_est")]);
            names.push(name);
        }
        let result = common::process::required_output(&mut cmd);
        let json: Value = serde_json::from_slice(&result.stdout).unwrap();
        let functions = json["functions"].as_array().unwrap();
        assert_eq!(functions.len(), outputs.len(), "{json}");
        assert!(json["assertions"].as_array().unwrap().iter().all(|a| a["status"] == "applied"), "{json}");
        for (j, ((function, name), (_, expected))) in functions.iter().zip(names).zip(outputs).enumerate() {
            assert_eq!(function["name"], name, "{function}");
            let text = function["code"].as_str().unwrap();
            assert!(function["error"].is_null() && !text.contains("halt_"), "{function}");
            let unique = format!("rotate_wide_{i}_{j}");
            writeln!(c, "#define {name} {unique}\n{text}\n#undef {name}").unwrap();
            writeln!(calls, "low_word = 0x4681; high_word = 0x8123;\n\
                      if ({unique}({est}) != {expected} || low_word != 0x4681 || high_word != 0x8123) return {};",
                      i * 4 + j + 1).unwrap();
        }
        fs::remove_dir_all(&specs).unwrap();
    }
    c.push_str(&calls);
    c.push_str("return 0;\n}\n");
    fs::write(&source, &c).unwrap();
    run_emitted_c(&source, &executable, &c);
    for file in [binary, source, executable] {
        fs::remove_file(file).unwrap();
    }
}

#[test]
fn carmel_cliw_parallel_reads_execute_in_emitted_c() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let image = common::scratch_file("carmel-cliw-parallel", "xml");
    let source = image.with_extension("c");
    let executable = image.with_extension("exe");
    let target = "Carmel:BE:24:default:default";
    // A0 is overwritten by MAC1, but ALU1 doubles the incoming A0.
    // The two LOADs use separate physical DATA banks.
    fs::write(&image, format!(
        "<binaryimage arch=\"{target}\">\n\
         <bytechunk space=\"code\" offset=\"0\" readonly=\"true\">804000c6428080c400c642908c6000dc01022bd8002bec00</bytechunk>\n\
         <bytechunk space=\"cliw\" offset=\"0\" readonly=\"true\">844fe10421184047f282529d</bytechunk>\n\
         <symbol space=\"code\" offset=\"0\" name=\"cliw_pair\"/>\n</binaryimage>\n"
    )).unwrap();
    let mut c = String::from("#include <stdbool.h>\n\
        volatile unsigned short bank0 = 0x1111, bank4 = 0xaaaa;\n");
    for reg in ["A0L", "A1L", "FF1", "FF2"] {
        let output = common::process::required_output(Command::new(env!("CARGO_BIN_EXE_kuna"))
            .arg("decompile").arg(&image).args(["cliw_pair", "--target", target, "--assert-strict",
                "--assert", "param 0 %A0L unsigned short a0",
                "--assert", "param 1 %A0H unsigned short high0",
                "--assert", "param 2 %GB0 unsigned char guard0",
                "--assert", "param 3 %ECFG unsigned short ecfg",
                "--assert", "data [data,0x4000] volatile unsigned short bank0",
                "--assert", "data [data,0xc400] volatile unsigned short bank4",
                "--assert", &format!("return %{reg} unsigned short")]));
        let code = String::from_utf8(output.stdout).unwrap();
        assert!(!code.contains("halt_") && !code.to_lowercase().contains("warning:"), "{code}");
        writeln!(c, "#define cliw_pair projection_{reg}\n{code}\n#undef cliw_pair").unwrap();
    }
    c.push_str("int main(void) {\n\
        if (projection_A0L(7, 0, 0, 0) != 0x9f4a) return 1;\n\
        if (projection_A1L(7, 0, 0, 0) != 14) return 2;\n\
        if (projection_FF1(7, 0, 0, 0) != 0x1111) return 3;\n\
        if (projection_FF2(7, 0, 0, 0) != 0xaaaa) return 4;\n\
        return 0;\n}\n");
    fs::write(&source, &c).unwrap();
    run_emitted_c(&source, &executable, &c);
    for file in [image, source, executable] {
        fs::remove_file(file).unwrap();
    }
}

#[test]
fn carmel_cliw_mac1_fetches_the_selected_continuation() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let target = "Carmel:BE:24:default:default";
    let image = common::scratch_file("carmel-cliw-second-mac", "xml");
    let source = image.with_extension("c");
    let executable = image.with_extension("exe");
    let mut c = String::from("#include <stdbool.h>\n\
        volatile unsigned short first_word = 0xa123, continuation = 3, short_word = 0xfffd;\n");
    let mut calls = String::from("int main(void) {\n");
    for (row, (prefix, packet, template, out_pointer, low, high, latch, pointer)) in [
        ("80ffffc64280", "8c6000dd7142", "864fe0fffff03fffff7ffff8", "R0", 0x369c, 0, 0x1234, 1),
        ("80ffffc64280", "8c6000dd7142", "84cfe0fffff03fffff7ffff8", "R0", 9, 0, 3, 1),
        ("80ffffc64290", "8e2000dd61c2", "865fe0fffff03fffff7ffff8", "R4", 0x369c, 0, 0x1234, 1),
        ("800040c64280804041c64284", "8c6000df2048", "8647e0fffff03fffff7ffff8", "R1",
         0xc964, 0xffff, 0x1234, 0x4042),
        ("80ffffc64290308c0e2bec00", "8e2000dd61c2", "865fe0fffff03fffff7ffff8", "R4", 0x369c, 0, 0x1234, 1),
    ].into_iter().enumerate() {
        let gap = "2bec002bec00".repeat(6);
        let (rm, rl, align) = if out_pointer == "R4" {
            ("RM4", "RL4", "RM4_ALIGN")
        } else {
            ("RM0", "RL0", "RM0_ALIGN")
        };
        fs::write(&image, format!(
            "<binaryimage arch=\"{target}\">\n\
             <bytechunk space=\"code\" offset=\"0\" readonly=\"true\">{prefix}{gap}{packet}2bd8002bec00</bytechunk>\n\
             <bytechunk space=\"cliw\" offset=\"0\" readonly=\"true\">{template}</bytechunk>\n\
             <symbol space=\"code\" offset=\"0\" name=\"cliw_mac\"/>\n</binaryimage>\n"
        )).unwrap();
        for (column, (reg, expected)) in [("A0L", low), ("A0H", high), ("X1", latch), (out_pointer, pointer)]
            .into_iter().enumerate()
        {
            let output = common::process::required_output(Command::new(env!("CARGO_BIN_EXE_kuna"))
                .arg("decompile").arg(&image).args(["cliw_mac", "--target", target, "--assert-strict",
                    "--assert", "param 0 %ECFG unsigned short ecfg",
                    "--assert", "param 1 %FF1 unsigned short factor",
                    "--assert", "param 2 %PACFG unsigned short address_mode",
                    "--assert", &format!("param 3 %{rm} unsigned short modulo"),
                    "--assert", &format!("param 4 %{rl} unsigned short bound"),
                    "--assert", &format!("param 5 %{align} unsigned short alignment"),
                    "--assert", "data [data,0xffff] volatile unsigned short first_word",
                    "--assert", "data [data,0] volatile unsigned short continuation",
                    "--assert", "data [data,0x40] volatile unsigned short short_word",
                    "--assert", &format!("return %{reg} unsigned short")]));
            let code = String::from_utf8(output.stdout).unwrap();
            assert!(!code.contains("halt_") && !code.to_lowercase().contains("warning:"), "{code}");
            let name = format!("project_{row}_{column}");
            writeln!(c, "#define cliw_mac {name}\n{code}\n#undef cliw_mac").unwrap();
            writeln!(calls, "if ({name}(0, 0x1234, 0, 7, 0xfff8, 0) != {expected:#x}) return {};",
                     row * 4 + column + 1).unwrap();
            if row < 3 && column == 0 {
                let doubled = expected * 2;
                writeln!(calls, "if ({name}(1, 0x1234, 0, 7, 0xfff8, 0) != {doubled:#x}) return {};",
                         row * 4 + column + 17).unwrap();
            }
            if row < 3 && column == 3 {
                let shift = if out_pointer == "R4" { 8 } else { 0 };
                writeln!(calls, "if ({name}(0, 0x1234, {:#x}, 7, 0xfff8, 0) != 0xfff9) return {};",
                         3 << shift, row * 4 + column + 29).unwrap();
            }
            if row == 4 && column == 3 {
                writeln!(calls, "if ({name}(0, 0x1234, 0x200, 0, 0, 0) != 0xfff9) return 41;").unwrap();
            }
        }
    }
    c.push_str(&calls);
    c.push_str("return 0;\n}\n");
    fs::write(&source, &c).unwrap();
    run_emitted_c(&source, &executable, &c);
    for file in [image, source, executable] {
        fs::remove_file(file).unwrap();
    }
}

#[test]
fn carmel_cliw_chained_short_reads_keep_distinct_effective_addresses() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let arch = "Carmel:BE:24:default:default";
    let image = common::scratch_file("carmel-cliw-chained", "xml");
    let specs = image.with_extension("specs");
    let source = image.with_extension("c");
    let executable = image.with_extension("exe");
    tracked_carmel_entry_specs(&specs, &[("R2", 0x7fff), ("PACFG", 0), ("EST", 0),
                                         ("CECFG1", 0), ("CECFG2", 0)], 0, 0);
    let mut c = String::from("#include <stdbool.h>\n\
        volatile unsigned short old_word=0x8001, patched=0x8123, next=0x0605, prior=0xffff;\n");
    let mut calls = String::from("int main(void) {\n");
    for (row, (packet, first, second, pointer)) in [
        ("0c614fdfa972", 0x8123, 0x0605, 0x7fff),
        ("8c614fdfa972", 0x8001, 0x0605, 0x7fff),
        ("0c614fdfa952", 0x8123, 0x0605, 0x8001),
        ("0c614fdfb932", 0x8123, 0xffff, 0x8004),
    ].into_iter().enumerate() {
        fs::write(&image, format!(
            "<binaryimage arch=\"{arch}\">\n\
             <bytechunk space=\"code\" offset=\"0\" readonly=\"true\">{packet}2bd8002bec00</bytechunk>\n\
             <bytechunk space=\"cliw\" offset=\"0xfb4\" readonly=\"true\">7fffff2601003fffff932088</bytechunk>\n\
             <symbol space=\"code\" offset=\"0\" name=\"cliw_chain\"/>\n</binaryimage>\n"
        )).unwrap();
        let guard1 = if second & 0x8000 != 0 { 0xff } else { 0 };
        for (column, (reg, expected, typ)) in [
            ("A0L", first, "unsigned short"), ("A1L", second, "unsigned short"),
            ("GB0", 0xff, "unsigned char"), ("GB1", guard1, "unsigned char"),
            ("R2", pointer, "unsigned short"),
        ].into_iter().enumerate() {
            let output = common::process::required_output(Command::new(env!("CARGO_BIN_EXE_kuna"))
                .env("KUNA_SPECS", &specs).arg("decompile").arg(&image)
                .args(["cliw_chain", "--target", arch, "--assert-strict",
                    "--assert", "param 0 %RN2 unsigned short increment",
                    "--assert", "param 1 %CETBL unsigned short condition_table",
                    "--assert", "data [data,0x7fff] volatile unsigned short old_word",
                    "--assert", "data [data,0x8003] volatile unsigned short patched",
                    "--assert", "data [data,0x8000] volatile unsigned short next",
                    "--assert", "data [data,0x7ffe] volatile unsigned short prior",
                    "--assert", &format!("return %{reg} {typ}")]));
            let code = String::from_utf8(output.stdout).unwrap();
            assert!(!code.contains("halt_") && !code.to_lowercase().contains("warning:"), "{code}");
            let name = format!("project_{row}_{column}");
            writeln!(c, "#define cliw_chain {name}\n{code}\n#undef cliw_chain").unwrap();
            let index = row * 5 + column + 1;
            writeln!(calls, "if ({name}(5, 0) != {expected:#x} || {name}(5, 0xffff) != {expected:#x}) return {index};")
                .unwrap();
        }
    }
    c.push_str(&calls);
    c.push_str("return 0;\n}\n");
    fs::write(&source, &c).unwrap();
    run_emitted_c(&source, &executable, &c);
    for file in [image, source, executable] {
        fs::remove_file(file).unwrap();
    }
    fs::remove_dir_all(specs).unwrap();
}

#[test]
fn carmel_cliw_unmarked_ma1_uses_the_trailing_address_width() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let arch = "Carmel:BE:24:default:default";
    let image = common::scratch_file("carmel-cliw-ma1-width", "xml");
    let source = image.with_extension("c");
    let exe = image.with_extension("exe");
    let mut c = String::from("#include <stdbool.h>\n#include <stdint.h>\n");
    let mut calls = String::from("int main(void) {\n");
    // The terminal indirect record is four bytes; the indexed one is three.
    // Neither may change the marked packet's encoded R2 postdecrement.
    for (row, (left, delta)) in [
        ("770004", 12), ("774004", 11), ("f70004", -1), ("f74004", -1),
    ].into_iter().enumerate() {
        let name = format!("cliw_width_{row}");
        fs::write(&image, format!(
            "<binaryimage arch=\"{arch}\">\n\
             <bytechunk space=\"code\" offset=\"0\" readonly=\"true\">{left}d3384b2bd8002bec00</bytechunk>\n\
             <bytechunk space=\"cliw\" offset=\"0x30\" readonly=\"true\">7fffff227f003fffff7ffff8</bytechunk>\n\
             <symbol space=\"code\" offset=\"0\" name=\"{name}\"/>\n</binaryimage>\n"
        )).unwrap();
        let output = common::process::required_output(Command::new(env!("CARGO_BIN_EXE_kuna"))
            .arg("decompile").arg(&image)
            .args([&name, "--target", arch, "--assert-strict",
                "--assert", "param 0 %R2 unsigned short pointer",
                "--assert", "param 1 %PACFG unsigned short config",
                "--assert", "param 2 %RM2 unsigned short modulo",
                "--assert", "param 3 %RL2 unsigned short bound",
                "--assert", "return %R2 unsigned short"]));
        let emitted = String::from_utf8(output.stdout).unwrap();
        assert!(!emitted.contains("halt_") && !emitted.to_ascii_lowercase().contains("warning:"),
                "{name}: {emitted}");
        writeln!(c, "{emitted}").unwrap();
        writeln!(calls, "if ({name}(0x8160, 0, 0, 0) != {:#x}) return {};",
                 0x8160 + delta, row + 1).unwrap();
        writeln!(calls, "if ({name}(0x8190, 0, 0, 0) != {:#x}) return {};",
                 0x8190 + delta, row + 1).unwrap();
    }
    c.push_str(&calls);
    c.push_str("return 0;\n}\n");
    fs::write(&source, &c).unwrap();
    run_emitted_c(&source, &exe, &c);
    for file in [image, source, exe] {
        fs::remove_file(file).unwrap();
    }
}

#[test]
fn carmel_cliw_merged_ma1_store_is_independent_of_the_late_add() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let scratch = tempfile::tempdir().unwrap();
    let image = scratch.path().join("cliw.xml");
    let specs = scratch.path().join("specs");
    let source = scratch.path().join("cliw.c");
    let executable = scratch.path().join("cliw");
    let arch = "Carmel:BE:24:default:default";
    tracked_carmel_entry_specs(&specs, &[
        ("R3", 0xc040), ("R7", 0xc048), ("PACFG", 0), ("EST", 0),
        ("CECFG1", 0), ("CECFG2", 0), ("ECFG", 0),
        ("A0L", 0x09a4), ("A1L", 0xecd0), ("A1H", 0x834c), ("GB1", 0x6a),
        ("A2L", 0x4b74), ("A2H", 0xb008), ("GB2", 0x87),
    ], 0, 0);
    let cells = [0xc040_u16, 0xc04b, 0xc04c, 0xc04e, 0xc04f];
    let mut c = String::from("#include <stdbool.h>\n#include <stdint.h>\n");
    for address in cells {
        writeln!(c, "volatile unsigned short data_{address:04x};").unwrap();
    }
    let mut calls = String::from("int main(void) {\n");
    for (row, (packet, cleared, pointer)) in [
        ("37bc16dd8e89", 0xc04b, 0xc046), ("77bc16dd8e89", 0xc04f, 0xc046),
        ("379c16dd8e89", 0xc04b, 0xc04c), ("779c16dd8e89", 0xc04e, 0xc04c),
        ("b7bc16dd8e89", 0xc040, 0xc046), ("f7bc16dd8e89", 0xc040, 0xc046),
    ].into_iter().enumerate() {
        fs::write(&image, format!(
            "<binaryimage arch=\"{arch}\">\n\
             <bytechunk space=\"code\" offset=\"0\" readonly=\"true\">{packet}2bd8002bec00</bytechunk>\n\
             <bytechunk space=\"cliw\" offset=\"0x108\" readonly=\"true\">7fffff0465003fffff7ffff8</bytechunk>\n\
             <symbol space=\"code\" offset=\"0\" name=\"cliw_merged\"/>\n</binaryimage>\n"
        )).unwrap();
        for (column, (reg, active, vetoed)) in [("A0L", 0x3844, 0x09a4), ("R7", pointer, pointer)].into_iter().enumerate() {
            let mut command = Command::new(env!("CARGO_BIN_EXE_kuna"));
            command.env("KUNA_SPECS", &specs).arg("decompile").arg(&image)
                .args(["cliw_merged", "--target", arch, "--assert-strict",
                    "--assert", "param 0 %RN7 unsigned short stride",
                    "--assert", "param 1 %CETBL unsigned short table",
                    "--assert", &format!("return %{reg} unsigned short")]);
            for address in cells {
                command.args(["--assert", &format!("data [data,{address:#x}] volatile unsigned short data_{address:04x}")]);
            }
            let output = common::process::required_output(&mut command);
            let code = String::from_utf8(output.stdout).unwrap();
            assert!(!code.contains("halt_") && !code.to_lowercase().contains("warning:"), "{code}");
            let name = format!("merged_{row}_{column}");
            writeln!(c, "#define cliw_merged {name}\n{code}\n#undef cliw_merged").unwrap();
            let fail = row * 2 + column + 1;
            for (table, expected) in [(1, active), (0x100, vetoed)] {
                for address in cells {
                    writeln!(calls, "data_{address:04x} = 0x1234;").unwrap();
                }
                writeln!(calls, "if ({name}(4,{table:#x}) != {expected:#x}) return {fail};").unwrap();
                for address in cells {
                    let expected = if address == cleared { 0 } else { 0x1234 };
                    writeln!(calls, "if (data_{address:04x} != {expected:#x}) return {fail};").unwrap();
                }
            }
        }
    }
    c.push_str(&calls);
    c.push_str("return 0;\n}\n");
    fs::write(&source, &c).unwrap();
    run_emitted_c(&source, &executable, &c);
}

#[test]
fn carmel_cliw_extended_source_ports_execute_in_emitted_c() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let image = common::scratch_file("carmel-cliw-extended", "xml");
    let source = image.with_extension("c");
    let executable = image.with_extension("exe");
    // One immutable template per image, with the same index-zero invocation.
    // The divide pair needs a positive trial as well as distinct A0/A1/A2.
    let rows = [
        ("7fffff23ff00438bf17ffff8", 0x14u64, 0xfffffc16acu64, 0x14, 0x400),
        ("7fffff23ff00c38bf17ffff8", 0xffffffff15, 0x2df59b, 0xff15, 0x10),
        ("7fffff23ff01438bf17ffff8", 0x16, 0xfffffbb28a, 0x16, 0x400),
        ("7fffff23ff01c38bf17ffff8", 0xffffffff17, 0x2d9179, 0xff17, 0x10),
        ("27fde0fffff03fffff7ffff8", 0xffff050015, 0xff9000abaa, 7, 0x10),
        ("2f8ce0fffff03fffff7ffff8", 0x1233abff, 0xff9000abaa, 7, 1),
        ("478ce0fffff03fffff7ffff8", 0x123454ab, 0xff9000abaa, 7, 0),
        ("7fffff7465003fffff7ffff8", 0x32350000, 0xff9000abaa, 7, 1),
        ("7ffffe9465003fffff7ffff8", 0xfeedcc0000, 0xff9000abaa, 7, 0x31),
        ("7ffffe9c65003fffff7ffff8", 0x112340000, 0xff9000abaa, 7, 0x20),
        ("7ffffe3c65003fffff7ffff8", 0x40040, 0x20020, 7, 0),
        ("7ffffe3465003fffff7ffff8", 0x2003e, 0x20020, 7, 0),
    ];
    let declarations = "#include <stdbool.h>\n#include <stdint.h>\n";
    let mut c = String::from(declarations);
    let mut calls = String::from("int main(void) {\n");
    let target = "Carmel:BE:24:default:default";
    let ty = |reg: &str| if reg.starts_with("GB") { "unsigned char" } else { "unsigned short" };
    for (row, (record, a0, a1, x2, est)) in rows.into_iter().enumerate() {
        let div = row >= 10;
        fs::write(&image, format!(
            "<binaryimage arch=\"{target}\">\n\
             <bytechunk space=\"code\" offset=\"0\" readonly=\"true\">8c6000dc61c22bd8002bec00</bytechunk>\n\
             <bytechunk space=\"cliw\" offset=\"0\" readonly=\"true\">{record}</bytechunk>\n\
             <symbol space=\"code\" offset=\"0\" name=\"cliw_sample\"/>\n</binaryimage>\n"
        )).unwrap();
        let seeds: &[(&str, u16)] = match row {
            0..=3 => &[("RN4", 0x14), ("RN5", 0xff15), ("RN6", 0x16), ("RN7", 0xff17), ("A2L", 0xcdef)],
            4 => &[("FF1", 0xff05), ("FF2", 0x15)],
            5..=6 => &[("A3L", 0x55), ("A3H", 0x1234), ("GB3", 0), ("A1L", 0xabaa)],
            7..=9 => &[("A1L", 0xabaa), ("A1H", 0x9000), ("GB1", 0xff),
                        ("A2L", 0xcdef), ("A2H", 0xa234), ("GB2", 0)],
            _ => &[("A1L", 0x20), ("A1H", 2), ("GB1", 0),
                   ("A2L", 1), ("A2H", 1), ("GB2", 0), ("BTR1", 0x8001)],
        };
        let mut outputs = vec![("A0L", a0 as u16), ("A0H", (a0 >> 16) as u16),
                               ("GB0", (a0 >> 32) as u16), ("EST", est)];
        if row < 4 {
            outputs.extend([("A1L", a1 as u16), ("A1H", (a1 >> 16) as u16),
                            ("GB1", (a1 >> 32) as u16), ("X2", x2)]);
        } else if div {
            outputs.push(("BTR1", if row == 10 { 0x8001 } else { 3 }));
        }
        for (reg, expected) in outputs {
            let mut inputs = seeds.to_vec();
            inputs.extend([("EST", 0), ("ECFG", 0), ("CECFG1", 0x1000), ("CECFG2", 0x1000), ("CETBL", 0)]);
            let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
            kuna.arg("decompile").arg(&image).args(["cliw_sample", "--target", target, "--assert-strict"]);
            for (i, (input, _)) in inputs.iter().enumerate() {
                kuna.args(["--assert", &format!("param {i} %{input} {} in_{}", ty(input), input.to_ascii_lowercase())]);
            }
            kuna.args(["--assert", &format!("return %{reg} {}", ty(reg))]);
            let output = common::process::required_output(&mut kuna);
            let code = String::from_utf8(output.stdout).unwrap();
            assert!(code.contains("cliw_sample(") && !code.contains("halt_") &&
                    !code.to_ascii_lowercase().contains("warning:") &&
                    !String::from_utf8_lossy(&output.stderr).to_ascii_lowercase().contains("warning:"),
                    "{record} {reg}\n{code}");
            let name = format!("cliw_{row}_{reg}");
            let arguments = inputs.iter().map(|(_, value)| value.to_string()).collect::<Vec<_>>().join(", ");
            writeln!(c, "#define cliw_sample {name}\n{code}\n#undef cliw_sample").unwrap();
            let signature = inputs.iter().map(|(input, _)| ty(input)).collect::<Vec<_>>().join(", ");
            writeln!(c, "typedef {} (*sig_{name})({signature});", ty(reg)).unwrap();
            writeln!(c, "_Static_assert(_Generic(&{name}, sig_{name}: 1, default: 0), \"{reg} ABI\");").unwrap();
            writeln!(calls, "if ({name}({arguments}) != {expected:#x}) return {};", row + 1).unwrap();
        }
    }
    c.push_str(&calls);
    c.push_str("return 0;\n}\n");
    fs::write(&source, &c).unwrap();
    run_emitted_c(&source, &executable, &c);
    for file in [image, source, executable] {
        fs::remove_file(file).unwrap();
    }
}

#[test]
fn carmel_loadce_writes_address_qualified_banks_and_samples_feedback_once() {
    const NOP: [u8; 6] = [0x2b, 0xec, 0, 0x2b, 0xec, 0];
    const RET: [u8; 6] = [0x2b, 0xd8, 0, 0x2b, 0xec, 0];
    let ce1 = [0x1d, 0xf8, 0x4b, 0xcd, 0xf9, 0x7b];
    let ce2 = [0x08, 0, 1, 0xcd, 0xe2, 0x09];
    let not = [0x2f, 0, 1, 0xcd, 0xe6, 0x3f];
    let ce1_addr = [0x40, 0, 1, 0xcd, 0xe0, 0x80];
    let latch_both = [0x8c, 0, 1, 0xcd, 0xe4, 0xb3];
    let markers = [[0, 0x12, 0x34, 0xc6, 0x42, 0x40], [0x40, 0x23, 0x45, 0xc6, 0x42, 0x44]];
    let cases = [
        (ce1, 0, 0x1000, 0, [0x1edd, 0x1000, 0x004b, 0x1000]),
        (ce1, 3, 0x1000, 0xffff, [0x1edd, 0x1000, 0xff4b, 0x3000]),
        (ce2, 0, 0x1000, 0, [0x1000, 0x0248, 0x0100, 0x2000]),
        (ce2, 3, 0x1000, 0xffff, [0x1000, 0x0248, 0x01ff, 0x3000]),
        (not, 0, 0x1000, 0, [0x0fef, 0x0fef, 0xfe01, 0x1000]),
        (ce1_addr, 0, 0x0ccc, 0x8000, [0x1000, 0x0ccc, 0x8000, 0]),
        (ce1_addr, 0, 0x1ccc, 0x8000, [0x2000, 0x1ccc, 0x8001, 0x3000]),
        (ce1_addr, 10, 0x0ccc, 0x8001, [0x2000, 0x0ccc, 0x8001, 0x3000]),
        (latch_both, 0, 0x1000, 0xffff, [0x2ccc, 0x2ccc, 0x0101, 0x3000]),
        (latch_both, 3, 0x1000, 0xffff, [0x2ccc, 0x2ccc, 0x0101, 0]),
    ];
    let binary = common::scratch_file("carmel-loadce-feedback", "bin");
    let source = binary.with_extension("c");
    let executable = binary.with_extension("exe");
    let specs = binary.with_extension("specs");
    let mut c = String::from("#include <stdint.h>\n#include <stdbool.h>\n");
    let mut calls = String::from("int main(void) {\n");
    for (i, (fetch, warm, cfg2, table, expected)) in cases.into_iter().enumerate() {
        let outputs = ["CECFG1", "CECFG2", "CETBL", "EST", "A0L", "A1L"];
        let mut body = vec![NOP; warm];
        body.push(fetch);
        let mut accumulator = [0, 0];
        if fetch == latch_both {
            body.extend(markers);
            if warm == 0 {
                accumulator = [0x1234, 0x2345];
            }
        }
        body.extend([NOP, NOP, NOP, NOP, RET]);
        fs::write(&binary, body.as_slice().as_flattened().repeat(outputs.len())).unwrap();
        tracked_carmel_entry_specs(&specs, &[("CECFG1", 0x1000), ("CECFG2", cfg2),
            ("CETBL", table), ("EST", 0), ("A0L", 0), ("A1L", 0)], 0x1000, 0);
        let mut kuna = Command::new(env!("CARGO_BIN_EXE_kuna"));
        kuna.env("KUNA_SPECS", &specs).arg("decompile-all").arg(&binary).args([
            "--raw-image", "--target", "Carmel:BE:24:default:default", "--base", "0x1000",
            "--json", "--jobs", "1", "--assert-strict",
        ]);
        let names = std::array::from_fn::<_, 6, _>(|j| {
            let addr = 0x1000 + 2 * j * body.len();
            let name = format!("sub_{addr:x}");
            kuna.args(["--addr", &format!("{addr:#x}"), "--assert",
                       &format!("return {name}::%{} unsigned short", outputs[j])]);
            name
        });
        let output = common::process::required_output(&mut kuna);
        let json: Value = serde_json::from_slice(&output.stdout).unwrap();
        let functions = json["functions"].as_array().unwrap();
        assert_eq!(functions.len(), outputs.len(), "{json}");
        assert!(json["assertions"].as_array().unwrap().iter().all(|a| a["status"] == "applied"), "{json}");
        let expected = expected.into_iter().chain(accumulator);
        for (j, ((function, name), value)) in functions.iter().zip(names).zip(expected).enumerate() {
            assert_eq!(function["name"], name, "{function}");
            let text = function["code"].as_str().unwrap();
            assert!(function["error"].is_null() && !text.contains("halt_"), "{function}");
            let unique = format!("loadce_{i}_{j}");
            writeln!(c, "#define {name} {unique}\n{text}\n#undef {name}").unwrap();
            writeln!(calls, "if ({unique}() != {value:#x}) return {};", i * outputs.len() + j + 1).unwrap();
        }
        fs::remove_dir_all(&specs).unwrap();
    }
    c.push_str(&calls);
    c.push_str("return 0;\n}\n");
    fs::write(&source, &c).unwrap();
    run_emitted_c(&source, &executable, &c);
    for file in [binary, source, executable] {
        fs::remove_file(file).unwrap();
    }
}

#[test]
fn carmel_secondary_shifter_reserves_physical_preg_reads() {
    let binary = common::scratch_file("carmel-secondary-shifter", "bin");
    let specs = binary.with_extension("specs");
    let source = binary.with_extension("c");
    let exe = binary.with_extension("exe");
    let mut c = String::from("#include <stdint.h>\n#include <stdbool.h>\n\
                             volatile unsigned short first = 0x4567, second = 0x2345;\n");
    tracked_carmel_entry_specs(&specs, &[
        ("R4", 0x40), ("R0", 0xc040), ("PACFG", 0), ("ECFG", 0),
        ("CECFG1", 0x1000), ("CECFG2", 0x1000), ("CETBL", 0xffff), ("EST", 0),
    ], 0x1000, 0);
    for (selector, name) in [(0x4f, "preg"), (0x4b, "ff")] {
        fs::write(&binary, [0x19, 0x17, 1, 0x39, selector, 0x26, 0x2b, 0xd8, 0, 0x2b, 0xec, 0]).unwrap();
        let result = common::process::required_output(Command::new(env!("CARGO_BIN_EXE_kuna"))
            .env("KUNA_SPECS", &specs).arg("decompile").arg(&binary).args([
                "0x1000", "--addr", "--raw-image", "--target", "Carmel:BE:24:default:default",
                "--base", "0x1000", "--json", "--assert-strict", "--assert", "return %A2L unsigned short",
                "--assert", "data [data,0x40] volatile unsigned short first",
                "--assert", "data [data,0xc040] volatile unsigned short second",
            ]));
        let json: Value = serde_json::from_slice(&result.stdout).unwrap();
        let function = &json["functions"][0];
        assert!(function["error"].is_null(), "{json}");
        let text = function["code"].as_str().unwrap();
        assert!(!text.contains("halt_") && !text.contains("carmel_data_read_conflict"), "{text}");
        writeln!(c, "#define sub_1000 {name}\n{text}\n#undef sub_1000").unwrap();
    }
    c.push_str("int main(void) { return preg() != 0x4567 || ff() != 0x68ac; }\n");
    fs::write(&source, &c).unwrap();
    run_emitted_c(&source, &exe, &c);
    fs::remove_dir_all(specs).unwrap();
    for file in [binary, source, exe] {
        fs::remove_file(file).unwrap();
    }
}

#[test]
fn carmel_sequential_address_update_resolves_the_right_port_from_the_fetch() {
    let binary = common::scratch_file("carmel-sequential-address-update", "bin");
    for (left, right, implemented) in [
        ([0xab, 0xec, 0], [0x2c, 0xb4, 0x20], true),
        ([0xab, 0xec, 0], [0x2c, 0xf4, 0x60], true),
        ([0xb0, 0x37, 0x80], [0x2c, 0xb4, 0x20], false),
        ([0x2b, 0xec, 0], [0x2c, 0xb4, 0x20], false),
    ] {
        fs::write(&binary, [left, right, [0x2b, 0xec, 0], [0x2b, 0xec, 0]].as_flattened()).unwrap();
        let (stdout, stderr, status) = common::run_kuna(&[
            "raw-pcode", binary.to_str().unwrap(), if left[0] == 0x2b { "0x1000" } else { "0x1001" },
            "--addr", "--raw-image", "--target", "Carmel:BE:24:default:default", "--base", "0x1000",
            "--count", "1", "--json",
        ]);
        let doc: Value = serde_json::from_str(&stdout).unwrap_or_else(|_| panic!("{stdout}\n{stderr}"));
        if !implemented {
            assert_eq!(status, 1, "{doc}");
            assert!(doc["instructions"].as_array().unwrap().is_empty(), "{doc}");
            continue;
        }
        assert_eq!(status, 0, "{doc}\n{stderr}");
        let [issue] = doc["instructions"].as_array().unwrap().as_slice() else { panic!("{doc}"); };
        assert_eq!(issue["address"], 0x1001, "{doc}");
        assert_eq!(issue["size"], 3, "{doc}");
        let ops = issue["ops"].as_array().unwrap();
        let mut writes = ops.iter().filter_map(|op| {
            assert!(!matches!(op["opcode"].as_str(), Some("LOAD" | "STORE")), "{doc}");
            let out = &op["output"];
            (out["space"] == "register").then(|| (out["offset"].as_u64().unwrap(), out["size"].as_u64().unwrap()))
        }).collect::<Vec<_>>();
        writes.sort_unstable();
        writes.dedup();
        assert_eq!(writes, [(0xb4, 2), (0xd0, 2)], "{doc}");
    }
    fs::remove_file(binary).unwrap();
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
