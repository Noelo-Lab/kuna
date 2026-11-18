use crate::common;
use std::process::Command;
use std::time::Duration;

#[path = "fixtures/ppc32_import_fixture.rs"]
mod fixture;

fn decompile(little: bool, words: Vec<u32>) -> String {
    let output = run_decompile(little, words, &[]);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap()
}

fn run_decompile(little: bool, words: Vec<u32>, extra: &[&str]) -> std::process::Output {
    let mut f = fixture::Fixture::new(fixture::Plt::Short);
    f.little = little;
    f.pic_collision = true;
    f.caller = Some(words);
    decompile_fixture(&f, extra)
}

fn decompile_fixture(f: &fixture::Fixture, extra: &[&str]) -> std::process::Output {
    let path = common::scratch_file("ppc32-computed-lr", "elf");
    std::fs::write(&path, f.bytes()).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args([
            "decompile",
            path.to_str().unwrap(),
            "check",
            "--mode",
            "fast",
        ])
        .args(["--assert", "prototype check int check(int a0)"])
        .args(extra)
        .output()
        .unwrap();
    std::fs::remove_file(path).unwrap();
    output
}

#[test]
fn shared_lr_transfer_discovers_its_second_destination() {
    let code = vec![
        0x7d8802a6, 0x38600009, 0x38001014, 0x7c0803a6, 0x4e800020, 0x38001020, 0x7c0803a6,
        0x4bfffff4, 0x3863000e, 0x7d8803a6, 0x4e800020,
    ];
    let common_work = vec![
        0x7d8802a6, 0x38600000, 0x38001018, 0x7c0803a6, 0x38630001, 0x4e800020, 0x38001024,
        0x7c0803a6, 0x4bfffff0, 0x38630014, 0x7d8803a6, 0x4e800020,
    ];
    for order in ["BE", "LE"] {
        for model in ["default", "e500"] {
            let target = format!("PowerPC:{order}:32:{model}");
            for mode in ["fast", "reliable"] {
                for (words, expected) in [(&code, "23"), (&common_work, "22")] {
                    let output = run_decompile(
                        order == "LE",
                        words.clone(),
                        &["--target", &target, "--mode", mode],
                    );
                    assert!(output.status.success(), "{target} {mode}: {output:?}");
                    let c = String::from_utf8(output.stdout).unwrap();
                    assert!(
                        c.contains("return ") && !c.contains("jump-as-call"),
                        "{target} {mode}: {c}"
                    );
                    check_runtime(&c, expected);
                }
            }
        }
    }
}

#[test]
fn shared_lr_tail_call_keeps_the_saved_return_path() {
    for order in ["BE", "LE"] {
        let mut f = fixture::Fixture::new(fixture::Plt::Short);
        f.little = order == "LE";
        f.pic_collision = true;
        f.caller = Some(vec![
            0x7d8802a6, 0x2c030000, 0x38600007, 0x41820010, 0x38001028, 0x7c0803a6, 0x4e800020,
            0x38001030, 0x7c0803a6, 0x4e800020, 0x7d8803a6, 0x4bfffff8,
        ]);
        f.second_caller = Some(vec![0x38600063, 0x7d8803a6, 0x4e800020]);
        for model in ["default", "e500"] {
            let target = format!("PowerPC:{order}:32:{model}");
            for mode in ["fast", "reliable"] {
                let output = decompile_fixture(
                    &f,
                    &[
                        "--target",
                        &target,
                        "--mode",
                        mode,
                        "--assert",
                        "prototype check2 int check2(void)",
                    ],
                );
                assert!(output.status.success(), "{target} {mode}: {output:?}");
                let c = String::from_utf8(output.stdout).unwrap();
                assert!(c.contains("return 7;"), "{target} {mode}: {c}");
                check_runtime(
                    &format!("int check2(void) {{ return 99; }}\n{c}"),
                    "i == 0 ? 99 : 7",
                );
            }
        }
    }
}

#[test]
fn computed_lr_keeps_the_returning_path() {
    let code = vec![
        0x7c0802a6, 0x9421ffe0, 0x90010024, 0x93c10018, 0x3bc02804, 0x2c030000, 0x41820010,
        0x3800102c, 0x7c0803a6, 0x4e800020, 0x3bc02800, 0x480007d5, 0x38600007, 0x83c10018,
        0x80010024, 0x38210020, 0x7c0803a6, 0x4e800020,
    ];
    for little in [false, true] {
        let c = decompile(little, code.clone());
        assert!(c.contains("return 7;"), "little={little}: {c}");
        assert!(!c.contains("return a0;"), "little={little}: {c}");
        assert!(!c.contains("__stack_chk_fail("), "little={little}: {c}");
    }
}

#[test]
fn wide_register_ppc32_lr_transfers_keep_the_target() {
    let code = vec![
        0x7d8802a6, 0x38001010, 0x7c0803a6, 0x4e800020, 0x38600007, 0x7d8803a6, 0x4e800020,
    ];
    let reloaded = vec![
        0x7d8802a6, 0x9421fff0, 0x38001024, 0x90010008, 0x38000000, 0x80010008, 0x7c0803a6,
        0x4e800020, 0x60000000, 0x38600007, 0x38210010, 0x7d8803a6, 0x4e800020,
    ];
    for little in [false, true] {
        let target = if little {
            "PowerPC:LE:32:e500"
        } else {
            "PowerPC:BE:32:e500"
        };
        for words in [&code, &reloaded] {
            let output = run_decompile(little, words.clone(), &["--target", target]);
            assert!(
                output.status.success(),
                "{}",
                String::from_utf8_lossy(&output.stderr)
            );
            check_runtime(&String::from_utf8(output.stdout).unwrap(), "7");
        }
    }
}

fn check_runtime(c: &str, expected: &str) {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        eprintln!("cc unavailable; skipping only emitted-C execution");
        return;
    }
    let source = common::scratch_file("ppc32-lr-output", "c");
    let executable = common::scratch_file("ppc32-lr-output", "bin");
    let source_text = format!("#include <stdbool.h>\ntypedef unsigned int undefined4; typedef unsigned int uint; typedef int int4;\n{c}\nint main(void) {{ for (int i = -4; i <= 4; ++i) if (check(i) != ({expected})) return 1; return 0; }}\n");
    std::fs::write(&source, source_text).unwrap();
    for optimization in ["-O0", "-O2"] {
        let build = Command::new("cc")
            .args([
                optimization,
                "-Werror=implicit-function-declaration",
                "-Werror=return-type",
            ])
            .arg(&source)
            .arg("-o")
            .arg(&executable)
            .output()
            .unwrap();
        assert!(
            build.status.success(),
            "{c}\n{}",
            String::from_utf8_lossy(&build.stderr)
        );
        let run = common::process::output_with_timeout(
            &mut Command::new(&executable),
            Duration::from_secs(5),
            Duration::from_millis(10),
        )
        .unwrap_or_else(|| panic!("emitted C did not return\n{c}"));
        assert!(run.status.success(), "{c}\n{run:?}");
    }
    std::fs::remove_file(source).unwrap();
    std::fs::remove_file(executable).unwrap();
}

#[test]
fn local_and_conditional_lr_transfers_execute_their_targets() {
    let direct = vec![
        0x7d8802a6, 0x38001010, 0x7c0803a6, 0x4e800020, 0x38600007, 0x7d8803a6, 0x4e800020,
    ];
    let conditional = vec![
        0x7d8802a6, 0x38001024, 0x7c0803a6, 0x2c030000, 0x4d820020, 0x38600009, 0x7d8803a6,
        0x4e800020, 0x60000000, 0x38600007, 0x7d8803a6, 0x4e800020,
    ];
    for little in [false, true] {
        for (words, expected) in [(&direct, "7"), (&conditional, "i == 0 ? 7 : 9")] {
            let c = decompile(little, words.clone());
            check_runtime(&c, expected);
        }
    }
}

#[test]
fn shared_lr_transfer_keeps_its_computed_and_returning_contexts() {
    let shared = vec![
        0x7d8802a6, 0x38600007, 0x38001014, 0x7c0803a6, 0x4e800020, 0x7d8803a6, 0x4bfffff8,
    ];
    let common_work = vec![
        0x7d8802a6, 0x38600005, 0x38001018, 0x7c0803a6, 0x38630001, 0x4e800020, 0x3863000a,
        0x7d8803a6, 0x4bfffff0,
    ];
    for little in [false, true] {
        for model in ["default", "e500"] {
            let target = format!("PowerPC:{}:32:{model}", if little { "LE" } else { "BE" });
            for (code, expected) in [(&shared, "7"), (&common_work, "17")] {
                for mode in ["fast", "reliable"] {
                    let output =
                        run_decompile(little, code.clone(), &["--target", &target, "--mode", mode]);
                    assert!(output.status.success(), "{target} {mode}: {output:?}");
                    let c = String::from_utf8(output.stdout).unwrap();
                    assert!(!c.contains("jump-as-call"), "{target} {mode}: {c}");
                    check_runtime(&c, expected);
                }
            }
        }
    }
}

#[test]
fn compared_incoming_lr_values_do_not_become_dispatches() {
    let guarded = vec![
        0x7d8802a6, 0x38600007, 0x7c006000, 0x4c820020, 0x7c0803a6, 0x4e800020,
    ];
    let mut argument = guarded.clone();
    argument[2] = 0x7c046000;
    argument[4] = 0x7c8803a6;
    let difference = vec![
        0x7d8802a6, 0x38600007, 0x7c806278, 0x2c000000, 0x4c820020, 0x7c8803a6, 0x4e800020,
    ];
    let escaped = vec![
        0x7d8802a6, 0x39602804, 0x918b0000, 0x818b0000, 0x38600007, 0x38805004, 0x7c046000,
        0x4c820020, 0x7c8803a6, 0x4e800020,
    ];
    let joined = vec![
        0x7d8802a6, 0x7c651b78, 0x2c050000, 0x41820008, 0x7c8c2378, 0x38600007, 0x38805004,
        0x2c050000, 0x4c820020, 0x7c046000, 0x4c820020, 0x7c8803a6, 0x4e800020,
    ];
    let constant = vec![
        0x7d8802a6, 0x38600007, 0x38805004, 0x7c046000, 0x4c820020, 0x7c8803a6, 0x4e800020,
    ];
    for little in [false, true] {
        for model in ["default", "e500"] {
            let target = format!("PowerPC:{}:32:{model}", if little { "LE" } else { "BE" });
            let plain_restore = vec![0x38600007, 0x7c0803a6, 0x4e800020];
            for code in [
                &guarded,
                &argument,
                &difference,
                &escaped,
                &joined,
                &constant,
                &plain_restore,
            ] {
                let output = run_decompile(little, code.clone(), &["--target", &target]);
                assert!(output.status.success(), "{target}: {output:?}");
                let c = String::from_utf8(output.stdout).unwrap();
                assert!(
                    !c.contains("jump-as-call") && !c.contains("switch"),
                    "{target}: {c}"
                );
                check_runtime(&format!("unsigned int dat_2804;\n{c}"), "7");
            }
        }
    }
}

#[test]
fn unproved_late_lr_context_is_reported() {
    let code = vec![
        0x7d8802a6, 0x38600007, 0x38001014, 0x7c0803a6, 0x4e800020, 0x80040000, 0x7c0803a6,
        0x4bfffff4,
    ];
    for little in [false, true] {
        for model in ["default", "e500"] {
            let target = format!("PowerPC:{}:32:{model}", if little { "LE" } else { "BE" });
            let output = run_decompile(little, code.clone(), &["--target", &target]);
            assert!(!output.status.success(), "{target}: {output:?}");
            let stderr = String::from_utf8(output.stderr).unwrap();
            assert!(
                stderr.contains("Cannot revalidate PPC32 LR"),
                "{target}: {stderr}"
            );
            assert!(!stderr.contains("panicked"), "{target}: {stderr}");
        }
    }
}

#[test]
fn joins_and_stack_reloads_keep_computed_lr_targets() {
    let joined = vec![
        0x7d8802a6, 0x2c030000, 0x4182000c, 0x38001020, 0x48000008, 0x38001020, 0x7c0803a6,
        0x4e800020, 0x38600007, 0x7d8803a6, 0x4e800020,
    ];
    let reloaded = vec![
        0x7d8802a6, 0x9421fff0, 0x38001024, 0x90010008, 0x38000000, 0x80010008, 0x7c0803a6,
        0x4e800020, 0x60000000, 0x38600007, 0x38210010, 0x7d8803a6, 0x4e800020,
    ];
    let frame_alias = vec![
        0x9421ffe0, 0x7c0802a6, 0x90010024, 0x93e1001c, 0x7c3f0b78, 0x907f000c, 0x3d200000,
        0x39291030, 0x7d2803a6, 0x4e800020, 0x3920ff9d, 0x4800000c, 0x813f000c, 0x39290007,
        0x7d234b78, 0x397f0020, 0x800b0004, 0x7c0803a6, 0x83ebfffc, 0x7d615b78, 0x4e800020,
    ];
    for little in [false, true] {
        for words in [&joined, &reloaded] {
            let c = decompile(little, words.clone());
            check_runtime(&c, "7");
        }
        for model in ["default", "e500"] {
            let target = format!("PowerPC:{}:32:{model}", if little { "LE" } else { "BE" });
            let output = run_decompile(little, frame_alias.clone(), &["--target", &target]);
            assert!(output.status.success(), "{target}: {output:?}");
            check_runtime(&String::from_utf8(output.stdout).unwrap(), "i + 7");
        }
    }
}

#[test]
fn incoming_saved_and_conditional_returns_stay_returns() {
    let leaf = vec![0x38600007, 0x4e800020];
    let saved = vec![
        0x7c0802a6, 0x9421fff0, 0x90010014, 0x38600007, 0x80010014, 0x38210010, 0x7c0803a6,
        0x4e800020,
    ];
    let conditional = vec![0x2c030000, 0x4d820020, 0x38600007, 0x4e800020];
    for little in [false, true] {
        for (words, expected) in [
            (&leaf, "7"),
            (&saved, "7"),
            (&conditional, "i == 0 ? 0 : 7"),
        ] {
            let c = decompile(little, words.clone());
            assert!(!c.contains("indirect jump"), "{c}");
            check_runtime(&c, expected);
        }
    }
}

#[test]
fn frame_restore_helper_keeps_the_unproved_lr_return() {
    for little in [false, true] {
        let c = decompile(
            little,
            vec![0x800b0004, 0x83ebfffc, 0x7c0803a6, 0x7d615b78, 0x4e800020],
        );
        assert!(!c.contains("jump-as-call") && !c.contains("switch"), "{c}");
    }
}

#[test]
fn unproved_input_lr_requires_a_branch_assertion() {
    for little in [false, true] {
        for model in ["default", "e500"] {
            let target = format!("PowerPC:{}:32:{model}", if little { "LE" } else { "BE" });
            let code = vec![0x7c6803a6, 0x4e800020];
            let output = run_decompile(little, code.clone(), &["--target", &target]);
            assert!(output.status.success(), "{target}: {output:?}");
            let c = String::from_utf8(output.stdout).unwrap();
            assert!(
                !c.contains("jump-as-call") && !c.contains("switch"),
                "{target}: {c}"
            );
            let output = run_decompile(
                little,
                code,
                &["--target", &target, "--assert", "flow check::0x1004 branch"],
            );
            assert!(output.status.success(), "{target}: {output:?}");
            let c = String::from_utf8(output.stdout).unwrap();
            assert!(
                c.contains("jump-as-call") || c.contains("switch"),
                "{target}: {c}"
            );
        }
    }
}

#[test]
fn memory_loaded_code_address_is_a_transfer() {
    let code = vec![
        0x7d8802a6, 0x39601020, 0x800b0000, 0x7c0803a6, 0x4e800020, 0x60000000, 0x60000000,
        0x60000000, 0x00001024, 0x38600007, 0x7d8803a6, 0x4e800020,
    ];
    for little in [false, true] {
        for model in ["default", "e500"] {
            let target = format!("PowerPC:{}:32:{model}", if little { "LE" } else { "BE" });
            let output = run_decompile(little, code.clone(), &["--target", &target]);
            assert!(output.status.success(), "{output:?}");
            check_runtime(&String::from_utf8(output.stdout).unwrap(), "7");
        }
    }
}

#[test]
fn fixed_address_saved_lr_returns_stay_returns() {
    let code = vec![
        0x7c0802a6, 0x38802804, 0x90040000, 0x38600007, 0x80040000, 0x7c0803a6, 0x4e800020,
    ];
    for little in [false, true] {
        for model in ["default", "e500"] {
            let target = format!("PowerPC:{}:32:{model}", if little { "LE" } else { "BE" });
            let output = run_decompile(little, code.clone(), &["--target", &target]);
            assert!(
                output.status.success(),
                "{}",
                String::from_utf8_lossy(&output.stderr)
            );
            let c = String::from_utf8(output.stdout).unwrap();
            assert!(c.contains("return 7;"), "{target}: {c}");
            assert!(
                !c.contains("jump-as-call") && !c.contains("switch"),
                "{target}: {c}"
            );
            check_runtime(&format!("unsigned int dat_2804;\n{c}"), "7");
        }
    }
}

#[test]
fn mutable_or_volatile_image_loads_stay_unproved() {
    let writable = vec![0x39602804, 0x800b0000, 0x7c0803a6, 0x4e800020];
    let modified = vec![
        0x7c0802a6, 0x39601020, 0x900b0000, 0x800b0000, 0x7c0803a6, 0x4e800020, 0x60000000,
        0x60000000, 0x00001024, 0x38600007, 0x4e800020,
    ];
    let mut aliased = modified.clone();
    aliased[2] = 0x90040000;
    let mut called = modified.clone();
    called[1] = 0x480007fd;
    called[2] = 0x39601020;
    let joined = vec![
        0x7d8802a6, 0x2c030000, 0x41820008, 0x91840000, 0x39601030, 0x800b0000, 0x7c0803a6,
        0x4e800020, 0x60000000, 0x60000000, 0x60000000, 0x60000000, 0x00001034, 0x38600007,
        0x7d8803a6, 0x4e800020,
    ];
    let literal = vec![
        0x7d8802a6, 0x39601020, 0x800b0000, 0x7c0803a6, 0x4e800020, 0x60000000, 0x60000000,
        0x60000000, 0x00001024, 0x38600007, 0x7d8803a6, 0x4e800020,
    ];
    for little in [false, true] {
        for model in ["default", "e500"] {
            let target = format!("PowerPC:{}:32:{model}", if little { "LE" } else { "BE" });
            for code in [&writable, &modified, &aliased, &called, &joined] {
                let output = run_decompile(little, code.clone(), &["--target", &target]);
                assert!(output.status.success(), "{target}: {output:?}");
                let c = String::from_utf8(output.stdout).unwrap();
                assert!(
                    !c.contains("jump-as-call") && !c.contains("switch"),
                    "{target}: {c}"
                );
                assert!(!c.contains("return 7;"), "{target}: {c}");
            }
            let output = run_decompile(
                little,
                literal.clone(),
                &["--target", &target, "--assert", "volatile 0x1023+1"],
            );
            assert!(output.status.success(), "{target}: {output:?}");
            let c = String::from_utf8(output.stdout).unwrap();
            assert!(
                !c.contains("jump-as-call") && !c.contains("switch"),
                "{target}: {c}"
            );
            assert!(!c.contains("return 7;"), "{target}: {c}");
        }
    }
}

#[test]
fn lr_target_low_bits_do_not_change_the_instruction_address() {
    for little in [false, true] {
        let code = vec![
            0x7d8802a6, 0x38001013, 0x7c0803a6, 0x4e800020, 0x38600007, 0x7d8803a6, 0x4e800020,
        ];
        for model in ["default", "e500"] {
            let target = format!("PowerPC:{}:32:{model}", if little { "LE" } else { "BE" });
            let output = run_decompile(little, code.clone(), &["--target", &target]);
            assert!(
                output.status.success(),
                "{}",
                String::from_utf8_lossy(&output.stderr)
            );
            check_runtime(&String::from_utf8(output.stdout).unwrap(), "7");
        }
    }
}

#[test]
fn explicit_return_assertion_retains_precedence() {
    for little in [false, true] {
        let code = vec![
            0x7d8802a6, 0x38001010, 0x7c0803a6, 0x4e800020, 0x38600007, 0x7d8803a6, 0x4e800020,
        ];
        let output = run_decompile(little, code, &["--assert", "flow check::0x100c return"]);
        assert!(!output.status.success());
        assert!(String::from_utf8_lossy(&output.stderr).contains("Could not apply flowoverride"));
        let c = String::from_utf8(output.stdout).unwrap();
        assert!(c.contains("return a0;"), "{c}");
        assert!(!c.contains("return 7;"), "{c}");
    }
}

#[test]
fn clobbered_targets_are_not_followed_as_constants() {
    let call_clobber = vec![
        0x7d8802a6, 0x38001014, 0x480007f9, 0x7c0803a6, 0x4e800020, 0x38600007, 0x7d8803a6,
        0x4e800020,
    ];
    let byte_clobber = vec![
        0x7d8802a6, 0x9421fff0, 0x38001024, 0x90010008, 0x98610008, 0x80010008, 0x7c0803a6,
        0x4e800020, 0x60000000, 0x38600007, 0x38210010, 0x7d8803a6, 0x4e800020,
    ];
    let mut alias_clobber = byte_clobber.clone();
    alias_clobber[4] = 0x90640000; // stw r3,0(r4), which may alias the stack slot
    for little in [false, true] {
        for code in [&call_clobber, &byte_clobber, &alias_clobber] {
            let c = decompile(little, code.clone());
            assert!(!c.contains("return 7;"), "{c}");
        }
    }
}
