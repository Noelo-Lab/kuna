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
        assert!(function["warnings"].as_array().unwrap().iter()
            .all(|w| w["kind"] == "branch-flip"), "{function}");
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
