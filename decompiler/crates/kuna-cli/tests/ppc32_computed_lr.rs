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

// The caller resumes at 0x1040 and increments the callee's result.
const SAVED_LR_THROUGH_ALIAS: [u32; 26] = [
    0x9421fff0, 0x7d8802a6, 0x9181000c, 0x8164fffc, 0x38210010, 0x38600007, 0x38a01040, 0x7c055800,
    0x4c820020, 0x7ca803a6, 0x4e800020, 0, 0, 0x39401060, 0x7c240b78, 0x4bffffc5, 0x38630001,
    0x7d4803a6, 0x4e800020, 0, 0, 0, 0, 0, 0x38000001, 0x44000002,
];

fn decompile_saved_lr(
    words: &[u32],
    order: &str,
    model: &str,
    mode: &str,
    extra: &[&str],
) -> String {
    let image = common::scratch_file("ppc32-saved-lr", "bin");
    let bytes: Vec<u8> = words
        .iter()
        .flat_map(|word| {
            if order == "LE" {
                word.to_le_bytes()
            } else {
                word.to_be_bytes()
            }
        })
        .collect();
    std::fs::write(&image, bytes).unwrap();
    let target = format!("PowerPC:{order}:32:{model}");
    let output = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .arg("decompile")
        .arg(&image)
        .args([
            "0x1000",
            "--raw-image",
            "--base",
            "0x1000",
            "--target",
            &target,
            "--mode",
            mode,
        ])
        .args(extra)
        .output()
        .unwrap();
    std::fs::remove_file(image).unwrap();
    assert!(output.status.success(), "{target} {mode}: {output:?}");
    String::from_utf8(output.stdout).unwrap()
}

#[test]
fn saved_lr_loaded_through_a_stack_alias_stays_a_return() {
    for order in ["BE", "LE"] {
        for model in ["default", "e500"] {
            for mode in ["fast", "reliable"] {
                let c = decompile_saved_lr(
                    &SAVED_LR_THROUGH_ALIAS,
                    order,
                    model,
                    mode,
                    &[
                        "--assert",
                        "prototype 0x1000 int sub_1000(int a0, void *alias)",
                    ],
                );
                assert!(
                    !c.contains("case 0x1040:") && !c.contains("return 8;"),
                    "{order} {model} {mode}: {c}"
                );
                let caller = "int check(int i) { int frame[2] = {i == 0 ? 0x1040 : 0, 0}; return sub_1000(0, frame + 1) + 1; }";
                check_runtime(&format!("{c}\n{caller}"), "8");
            }
        }
    }
}

#[test]
fn saved_lr_byte_used_by_a_guard_stays_unproved() {
    let mut words = SAVED_LR_THROUGH_ALIAS;
    words[2] = 0x9981000c; // stb r12,12(r1)
    words[3] = 0x8961000c; // lbz r11,12(r1)
    words[7] = 0x2c0b0040; // cmpwi r11,0x40
    for order in ["BE", "LE"] {
        for model in ["default", "e500"] {
            for mode in ["fast", "reliable"] {
                let c = decompile_saved_lr(&words, order, model, mode, &[]);
                assert!(
                    c.contains("return 7;")
                        && !c.contains("case 0x1040:")
                        && !c.contains("return 8;"),
                    "{order} {model} {mode}: {c}"
                );
            }
        }
    }
}

#[test]
fn lr_memory_addresses_and_derived_guards_keep_caller_code_separate() {
    let direct_store = [
        0x7d8802a6, 0x3d606000, 0x916c0000, 0x60000000, 0x60000000, 0x38a01040, 0x80c50000,
        0x38600007, 0x7c065800, 0x4c820020, 0x7ca803a6, 0x4e800020, 0, 0x39401060, 0x38600000,
        0x4bffffc5, 0x7c000378, 0x38630001, 0x7d4803a6, 0x4e800020, 0, 0, 0, 0, 0x38000001,
        0x44000002,
    ];
    let mut derived_store = direct_store;
    derived_store[2..5].copy_from_slice(&[0x392c0004, 0x3929fffc, 0x91690000]);
    let mut derived_guard = direct_store;
    derived_guard[1] = 0x38cc0004; // addi r6,r12,4
    derived_guard[2] = 0x39601044; // li r11,0x1044
    derived_guard[6] = 0x60000000;
    for order in ["BE", "LE"] {
        for model in ["default", "e500"] {
            for mode in ["fast", "reliable"] {
                for (name, words) in [
                    ("direct store", &direct_store),
                    ("derived store", &derived_store),
                    ("derived guard", &derived_guard),
                ] {
                    let c = decompile_saved_lr(words, order, model, mode, &[]);
                    assert!(
                        c.contains("return 7;")
                            && !c.contains("case 0x1040:")
                            && !c.contains("return 8;"),
                        "{order} {model} {mode} {name}: {c}"
                    );
                }
            }
        }
    }
}

#[test]
fn storing_incoming_lr_preserves_a_later_constant_jump() {
    let code = [
        0x7d8802a6, // mflr r12
        0x91840000, // stw r12,0(r4)
        0x38600003, // li r3,3
        0x38001018, // li r0,0x1018
        0x7c0803a6, // mtlr r0
        0x4e800020, // blr
        0x38600007, // li r3,7
        0x7d8803a6, // mtlr r12
        0x4e800020, // blr
    ];
    let revisited = [
        0x7d8802a6, 0x91840000, 0x38600009, 0x38001018, 0x7c0803a6, 0x4e800020, 0x38001024,
        0x7c0803a6, 0x4bfffff4, 0x3863000e, 0x7d8803a6, 0x4e800020,
    ];
    for order in ["BE", "LE"] {
        for model in ["default", "e500"] {
            for mode in ["fast", "reliable"] {
                for option in ["on", "off"] {
                    for (words, reached, early_return) in [
                        (&code[..], "return 7;", "return 3;"),
                        (&revisited[..], "return 0x17;", "return 9;"),
                    ] {
                        let c = decompile_saved_lr(
                            words,
                            order,
                            model,
                            mode,
                            &[
                                "--assert",
                                "prototype 0x1000 int sub_1000(int i, int *slot)",
                                "--option",
                                "returnbranch",
                                option,
                            ],
                        );
                        assert!(
                            c.contains("*slot = ") && c.contains(reached) && !c.contains(early_return),
                            "{order} {model} {mode} returnbranch={option}: {c}"
                        );
                    }
                }
            }
        }
    }
}

#[test]
fn unused_lr_arithmetic_preserves_computed_transfers() {
    let code = [
        0x7d8802a6, 0x388c0004, 0x2c030000, 0x41820010, 0x38001028, 0x7c0803a6, 0x4e800020,
        0x38600007, 0x7d8803a6, 0x4e800020, 0x38600007, 0x7d8803a6, 0x4e800020,
    ];
    for order in ["BE", "LE"] {
        for model in ["default", "e500"] {
            for mode in ["fast", "reliable"] {
                let c = decompile_saved_lr(
                    &code,
                    order,
                    model,
                    mode,
                    &["--assert", "prototype 0x1000 int sub_1000(int i)"],
                );
                check_runtime(
                    &format!("{c}\nint check(int i) {{ return sub_1000(i); }}"),
                    "7",
                );
            }
        }
    }
}

#[test]
fn reusing_a_saved_lr_slot_keeps_computed_transfers() {
    let reused = [
        0x9421fff0, 0x7d8802a6, 0x9181000c, 0x8181000c, 0x9061000c, 0x38210010, 0x2c030000,
        0x41820010, 0x38001038, 0x7c0803a6, 0x4e800020, 0x38600007, 0x7d8803a6, 0x4e800020,
        0x38600007, 0x7d8803a6, 0x4e800020,
    ];
    let mut adjacent = reused;
    adjacent[4] = 0x90610008;
    let mut wider = reused;
    wider[4] = 0xd8210008; // stfd f1,8(r1), covering the spill at 12(r1)
    let reload_target = [
        0x9421fff0, 0x7d8802a6, 0x9181000c, 0x8181000c, 0x38001024, 0x9001000c, 0x8001000c,
        0x7c0803a6, 0x4e800020, 0x38600007, 0x38210010, 0x7d8803a6, 0x4e800020,
    ];
    for order in ["BE", "LE"] {
        for model in ["default", "e500"] {
            for mode in ["fast", "reliable"] {
                for (words, case) in [
                    (reused.as_slice(), None),
                    (adjacent.as_slice(), None),
                    (wider.as_slice(), None),
                    (reload_target.as_slice(), Some("case 0x1024:")),
                ] {
                    let c = decompile_saved_lr(
                        words,
                        order,
                        model,
                        mode,
                        &["--assert", "prototype 0x1000 int sub_1000(int i)"],
                    );
                    let program = format!("{c}\nint check(int i) {{ return sub_1000(i); }}");
                    if let Some(case) = case.filter(|_| c.contains("switch(")) {
                        assert!(c.contains(case), "{order} {model} {mode}: {c}");
                        assert_eq!(c.matches("case ").count(), 1, "{order} {model} {mode}: {c}");
                        check_runtime_with_flags(&program, "7", &["-Wno-error=return-type"]);
                    } else {
                        check_runtime(&program, "7");
                    }
                }
            }
        }
    }
}

#[test]
fn partial_overwrite_can_leave_saved_lr_bytes_in_a_guard() {
    let mut words = [
        0x9421fff0, 0x7d8802a6, 0x9181000c, 0x9861000c, 0x8161000c, 0x38210010, 0x38600007,
        0x38a01040, 0x7c055800, 0x4c820020, 0x7ca803a6, 0x4e800020, 0, 0x39401060, 0x38600000,
        0x4bffffc5, 0x38630001, 0x7d4803a6, 0x4e800020, 0, 0, 0, 0, 0, 0x38000001, 0x44000002,
    ];
    for order in ["BE", "LE"] {
        // Replace the high byte of LR and keep the other three bytes intact.
        words[3] = if order == "LE" {
            0x9861000f
        } else {
            0x9861000c
        };
        for model in ["default", "e500"] {
            for mode in ["fast", "reliable"] {
                let c = decompile_saved_lr(&words, order, model, mode, &[]);
                assert!(
                    c.contains("return 7;")
                        && !c.contains("case 0x1040:")
                        && !c.contains("return 8;"),
                    "{order} {model} {mode}: {c}"
                );
            }
        }
    }
}

#[test]
fn joined_lr_operands_keep_their_path_correlation() {
    let same_register = [
        0x7d8802a6, 0x2c030000, 0x4182000c, 0x38800004, 0x48000008, 0x38800014, 0x7ca42050,
        0x38a51040, 0x7ca803a6, 0x4e800020, 0, 0, 0x7c6803a6, 0x4bfffff0, 0, 0, 0x60000000,
        0x7d8803a6, 0x4e800020, 0, 0x4bffffd4,
    ];
    let copied_register = [
        0x7d8802a6, 0x2c030000, 0x4182000c, 0x38800004, 0x48000008, 0x38800014, 0x7c862378,
        0x7ca62050, 0x38a51040, 0x7ca803a6, 0x4e800020, 0, 0x7c6803a6, 0x4bfffff4, 0, 0,
        0x60000000, 0x7d8803a6, 0x4e800020, 0, 0x4bffffd8,
    ];
    let mut known_condition = vec![
        0x7d8802a6, 0x2c030000, 0x4182000c, 0x38800004, 0x48000008, 0x38800014, 0x2c040004,
        0x4182000c, 0x38c00014, 0x48000008, 0x38c00004, 0x7ca43050, 0x38a51080, 0x7ca803a6,
        0x4e800020,
    ];
    known_condition.resize(0x70 / 4, 0);
    known_condition.extend([
        0x7c6803a6, 0x4bffffc4, 0, 0, 0x7d8803a6, 0x4e800020, 0, 0, 0x4bffffa8,
    ]);
    let distinct_targets = [
        0x7d8802a6, 0x2c030000, 0x41820010, 0x38800004, 0x38c00014, 0x4800000c, 0x38800014,
        0x38c00004, 0x7ca62050, 0x38a51040, 0x7ca803a6, 0x4e800020, 0x38800009, 0x48000020, 0, 0,
        0x7c6803a6, 0x4bffffe8, 0, 0, 0x38800007, 0x2c030000, 0x4180000c, 0x7c632214, 0x48000008,
        0x7c641850, 0x7d8803a6, 0x4e800020,
    ];
    let cases = [
        ("same register", same_register.as_slice(), "i"),
        ("copied register", copied_register.as_slice(), "i"),
        ("known condition", known_condition.as_slice(), "i"),
        (
            "distinct targets",
            distinct_targets.as_slice(),
            "i == 0 ? 7 : i < 0 ? i - 9 : i + 9",
        ),
    ];
    for order in ["BE", "LE"] {
        for model in ["default", "e500"] {
            let target = format!("PowerPC:{order}:32:{model}");
            for mode in ["fast", "reliable"] {
                for (case, words, expected) in cases {
                    let output = run_decompile(
                        order == "LE",
                        words.to_vec(),
                        &["--target", &target, "--mode", mode],
                    );
                    assert!(
                        output.status.success(),
                        "{target} {mode} {case}: {output:?}"
                    );
                    let c = String::from_utf8(output.stdout).unwrap();
                    assert!(
                        !c.contains("halt_missing") && !c.contains("jump-as-call"),
                        "{target} {mode} {case}: {c}"
                    );
                    if case == "distinct targets" {
                        assert!(!c.contains("case 0x1040:"), "{target} {mode}: {c}");
                    }
                    if case == "known condition" && c.contains("switch(") {
                        assert!(c.contains("case 0x1080:"), "{target} {mode}: {c}");
                        assert_eq!(c.matches("case ").count(), 1, "{target} {mode}: {c}");
                        // A C compiler cannot prove that this case exhausts the switch.
                        check_runtime_with_flags(&c, expected, &["-Wno-error=return-type"]);
                    } else {
                        check_runtime(&c, expected);
                    }
                }
            }
        }
    }
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
    check_runtime_with_flags(c, expected, &[]);
}

fn check_runtime_with_flags(c: &str, expected: &str, flags: &[&str]) {
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
            .args(flags)
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
    let saved_lr = vec![
        0x9421fff0, 0x7d8802a6, 0x9181000c, 0x39601028, 0x800b0000, 0x7c0803a6, 0x4e800020, 0, 0,
        0, 0x0000102c, 0x38600007, 0x8181000c, 0x38210010, 0x7d8803a6, 0x4e800020,
    ];
    for little in [false, true] {
        for model in ["default", "e500"] {
            let target = format!("PowerPC:{}:32:{model}", if little { "LE" } else { "BE" });
            for words in [&code, &saved_lr] {
                let output = run_decompile(little, words.clone(), &["--target", &target]);
                assert!(output.status.success(), "{output:?}");
                check_runtime(&String::from_utf8(output.stdout).unwrap(), "7");
            }
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
