use crate::common;
use std::process::Command;

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
    let source_text = format!("typedef unsigned int undefined4; typedef unsigned int uint;\n{c}\nint main(void) {{ for (int i = -4; i <= 4; ++i) if (check(i) != ({expected})) return 1; return 0; }}\n");
    std::fs::write(&source, source_text).unwrap();
    for optimization in ["-O0", "-O2"] {
        let build = Command::new("cc")
            .args([optimization, "-Werror=implicit-function-declaration"])
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
        let run = Command::new(&executable).output().unwrap();
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
fn joins_and_stack_reloads_keep_computed_lr_targets() {
    let joined = vec![
        0x7d8802a6, 0x2c030000, 0x4182000c, 0x38001020, 0x48000008, 0x38001020, 0x7c0803a6,
        0x4e800020, 0x38600007, 0x7d8803a6, 0x4e800020,
    ];
    let reloaded = vec![
        0x7d8802a6, 0x9421fff0, 0x38001024, 0x90010008, 0x38000000, 0x80010008, 0x7c0803a6,
        0x4e800020, 0x60000000, 0x38600007, 0x38210010, 0x7d8803a6, 0x4e800020,
    ];
    for little in [false, true] {
        for words in [&joined, &reloaded] {
            let c = decompile(little, words.clone());
            check_runtime(&c, "7");
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
fn unresolved_register_dispatch_is_not_an_abi_return() {
    for little in [false, true] {
        let c = decompile(little, vec![0x7c6803a6, 0x4e800020]);
        assert!(c.contains("jump-as-call") || c.contains("switch"), "{c}");
        assert!(!c.contains("return a0;"), "{c}");
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
