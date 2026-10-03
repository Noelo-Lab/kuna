#![cfg(all(target_arch = "x86_64", target_os = "linux"))]

mod common;

use common::process;
use std::path::{Path, PathBuf};
use std::process::Command;

fn compiler(name: &str) -> bool {
    process::optional_output(Command::new(name).arg("--version")).is_some()
}

fn fixture() -> PathBuf {
    PathBuf::from(common::fixture("globalorder_x86_64.c"))
}

fn build(command: &mut Command) {
    process::required_output(command);
}

fn prepare(cc: &str, level: &str, dir: &Path) -> (PathBuf, PathBuf, PathBuf) {
    std::fs::create_dir_all(dir).unwrap();
    let source = fixture();
    let sample = dir.join("sample.o");
    let object = dir.join("original.o");
    let original = dir.join("original");
    build(
        Command::new(cc)
            .args(["-O0", "-DGLOBALORDER_SAMPLE", "-c"])
            .arg(&source)
            .arg("-o")
            .arg(&sample),
    );
    build(
        Command::new(cc)
            .args([level, "-fPIE", "-fno-asynchronous-unwind-tables", "-c"])
            .arg(&source)
            .arg("-o")
            .arg(&object),
    );
    build(
        Command::new(cc)
            .args([level, "-DGLOBALORDER_MAIN"])
            .arg(&source)
            .arg(&sample)
            .arg("-o")
            .arg(&original),
    );
    (original, object, sample)
}

fn decompile(binary: &Path, functions: &str) -> String {
    let specs = common::repo_root().join("specs");
    let (printed, stderr, rc) = common::run_kuna(&[
        "decompile-all",
        binary.to_str().unwrap(),
        "--functions",
        functions,
        "--sleighpath",
        specs.to_str().unwrap(),
        "--assert",
        "prototype sample void sample(void)",
        "--assert",
        "prototype pointer_store int pointer_store(int *p, int b)",
        "--assert",
        "prototype uid_sequence unsigned long uid_sequence(unsigned int *p)",
        "--assert",
        "prototype scaled_sequence long scaled_sequence(long *p)",
    ]);
    assert_eq!(rc, 0, "{stderr}");
    printed
}

#[test]
fn a_computed_global_write_stays_after_observations_and_on_its_paths() {
    for cc in ["gcc", "clang"].into_iter().filter(|cc| compiler(cc)) {
        for level in ["-O0", "-O2"] {
            let dir = common::scratch_file("globalorder-value", "dir");
            let (original, _, sample) = prepare(cc, level, &dir);
            let want = process::required_output(&mut Command::new(&original)).stdout;
            let printed = decompile(
                &original,
                "computed,called,conditional,before,repeated,nonalias,pointer_store,earlier_branch,earlier_loop,pick",
            );
            let source = dir.join("printed.c");
            std::fs::write(&source, format!("#include <stdbool.h>\nextern int gi,other,seen;\nextern volatile int sink;\nvoid sample(void);\n{printed}")).unwrap();
            for out_cc in ["gcc", "clang"].into_iter().filter(|cc| compiler(cc)) {
                for out_level in ["-O0", "-O2"] {
                    let exe = dir.join(format!("printed-{out_cc}-{out_level}"));
                    build(
                        Command::new(out_cc)
                            .args([out_level, "-w", "-fwrapv", "-DGLOBALORDER_HARNESS"])
                            .arg(fixture())
                            .arg(&source)
                            .arg(&sample)
                            .arg("-o")
                            .arg(&exe),
                    );
                    let got = process::required_output(&mut Command::new(&exe)).stdout;
                    assert_eq!(
                        String::from_utf8_lossy(&got),
                        String::from_utf8_lossy(&want),
                        "{cc} {level}, rebuilt by {out_cc} {out_level}:\n{printed}"
                    );
                }
            }
            std::fs::remove_dir_all(dir).unwrap();
        }
    }
}

#[test]
fn switch_global_stores_keep_their_actual_access_sequence() {
    let modes = [
        0u32,
        29,
        30,
        31,
        32,
        33,
        34,
        35,
        36,
        37,
        38,
        65566,
        65567,
        65568,
        65573,
        u32::MAX,
    ];
    for cc in ["gcc", "clang"].into_iter().filter(|cc| compiler(cc)) {
        for level in ["-O0", "-O2"] {
            let dir = common::scratch_file("globalorder-trace", "dir");
            let (original, object, sample) = prepare(cc, level, &dir);
            let printed = decompile(&original, "pick");
            let source = dir.join("printed.c");
            std::fs::write(&source, format!("volatile int sink;\n{printed}")).unwrap();
            let trace = PathBuf::from(common::fixture("globalorder_trace_x86_64.c"));
            let native = dir.join("trace-native");
            build(
                Command::new(cc)
                    .arg("-O0")
                    .arg(&trace)
                    .arg(&object)
                    .arg(&sample)
                    .arg("-o")
                    .arg(&native),
            );
            for out_cc in ["gcc", "clang"].into_iter().filter(|cc| compiler(cc)) {
                for out_level in ["-O0", "-O2"] {
                    let exe = dir.join(format!("trace-{out_cc}-{out_level}"));
                    build(
                        Command::new(out_cc)
                            .args([out_level, "-w", "-fwrapv"])
                            .arg(&trace)
                            .arg(&source)
                            .arg("-o")
                            .arg(&exe),
                    );
                    for mode in modes {
                        for accesses in [false, true] {
                            let mut native_run = Command::new(&native);
                            let mut printed_run = Command::new(&exe);
                            native_run.arg(mode.to_string());
                            printed_run.arg(mode.to_string());
                            if accesses {
                                native_run.arg("rw");
                                printed_run.arg("rw");
                            }
                            let want = process::required_output(&mut native_run).stdout;
                            let got = process::required_output(&mut printed_run).stdout;
                            assert_eq!(String::from_utf8_lossy(&got), String::from_utf8_lossy(&want), "{cc} {level}, mode {mode}, read/write {accesses}, rebuilt by {out_cc} {out_level}:\n{printed}");
                        }
                    }
                }
            }
            std::fs::remove_dir_all(dir).unwrap();
        }
    }
}

#[test]
fn computed_multi_global_writes_keep_the_instruction_order() {
    for cc in ["gcc", "clang"].into_iter().filter(|cc| compiler(cc)) {
        for level in ["-O0", "-O2"] {
            let dir = common::scratch_file("globalorder-multi-trace", "dir");
            let (original, object, sample) = prepare(cc, level, &dir);
            let printed = decompile(&original, "uid_sequence,scaled_sequence");
            let source = dir.join("printed.c");
            std::fs::write(&source, format!("int gi,other,seen;\nvolatile int sink;\nvolatile unsigned char flag_a,flag_b;\nvolatile unsigned long uid_a,uid_b;\nvolatile long scaled;\n{printed}")).unwrap();
            let trace = PathBuf::from(common::fixture("globalorder_trace_x86_64.c"));
            let native = dir.join("trace-native");
            build(
                Command::new(cc)
                    .args(["-O0", "-DGLOBALORDER_MULTIGLOBAL"])
                    .arg(&trace)
                    .arg(&object)
                    .arg(&sample)
                    .arg("-o")
                    .arg(&native),
            );
            for out_cc in ["gcc", "clang"].into_iter().filter(|cc| compiler(cc)) {
                for out_level in ["-O0", "-O2"] {
                    let exe = dir.join(format!("trace-{out_cc}-{out_level}"));
                    build(
                        Command::new(out_cc)
                            .args([out_level, "-w", "-fwrapv", "-DGLOBALORDER_MULTIGLOBAL"])
                            .arg(&trace)
                            .arg(&source)
                            .arg(&sample)
                            .arg("-o")
                            .arg(&exe),
                    );
                    for mode in ["0", "1"] {
                        for value in ["0", "17", "4294967295"] {
                            let want =
                                process::required_output(Command::new(&native).args([mode, value]))
                                    .stdout;
                            let got =
                                process::required_output(Command::new(&exe).args([mode, value]))
                                    .stdout;
                            assert_eq!(String::from_utf8_lossy(&got), String::from_utf8_lossy(&want), "{cc} {level}, mode {mode}, value {value}, rebuilt by {out_cc} {out_level}:\n{printed}");
                        }
                    }
                }
            }
            std::fs::remove_dir_all(dir).unwrap();
        }
    }
}
