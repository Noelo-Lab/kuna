use crate::common;
use std::process::Command;

fn image(count: usize, scalar: bool, last: bool) -> Vec<u8> {
    let registers = [0u32, 2, 3, 4, 5, 6, 7, 8];
    let saved = registers[..count]
        .iter()
        .filter(|&&r| r >= 4)
        .fold(0, |mask, &r| mask | (1 << r));
    let mut words = vec![0xe92d4000 | saved, 0xe3001000, 0xe3451000];
    if scalar {
        words.extend(
            registers[..count]
                .iter()
                .enumerate()
                .map(|(i, &r)| 0xe5910000 | (r << 12) | (i as u32 * 4)),
        );
    } else {
        words.push(
            0xe8910000
                | registers[..count]
                    .iter()
                    .fold(0, |mask, &r| mask | (1 << r)),
        );
    }
    if last {
        words.push(0xe1a00000 | registers[count - 1]);
    }
    words.push(0xe8bd8000 | saved);
    words.into_iter().flat_map(u32::to_le_bytes).collect()
}

#[test]
fn unused_multiple_load_destinations_keep_their_volatile_reads() {
    for count in [4, 8] {
        for scalar in [false, true] {
            for last in [false, true] {
                let path = common::scratch_file("arm-volatile-reads", "bin");
                std::fs::write(&path, image(count, scalar, last)).unwrap();
                for marked in [0, 1, 2] {
                    let mut cmd = Command::new(env!("CARGO_BIN_EXE_kuna"));
                    cmd.args([
                        "decompile",
                        path.to_str().unwrap(),
                        "0x1000",
                        "--addr",
                        "--raw-image",
                        "--base",
                        "0x1000",
                        "--target",
                        "ARM:LE:32:v7",
                        "--isa",
                        "arm",
                        "--mode",
                        "aggressive",
                    ]);
                    if marked != 0 {
                        let (start, size) = if marked == 1 {
                            (0x50000000, count * 4)
                        } else {
                            (0x50000008, 4)
                        };
                        cmd.args([
                            "--assert",
                            &format!("volatile 0x{start:x}+{size}"),
                            "--assert-strict",
                        ]);
                    }
                    let result = cmd.output().unwrap();
                    let code = String::from_utf8_lossy(&result.stdout);
                    assert!(
                        result.status.success(),
                        "{code}\n{}",
                        String::from_utf8_lossy(&result.stderr)
                    );
                    let mut previous = None;
                    for i in 0..count {
                        let name = format!("dat_{:08x}", 0x50000000 + i * 4);
                        let should_read = marked == 1
                            || (marked == 2 && i == 2)
                            || i == if last { count - 1 } else { 0 };
                        assert_eq!(
                            code.matches(&name).count(),
                            usize::from(should_read),
                            "count={count} scalar={scalar} last={last} marked={marked}\n{code}"
                        );
                        if marked == 1 {
                            let at = code.find(&name).unwrap();
                            assert!(previous.is_none_or(|p| p < at), "read order: {code}");
                            previous = Some(at);
                        }
                    }
                }
            }
        }
    }
}

fn decompile_raw(hex: &str, target: &str, volatile: Option<&str>) -> String {
    let image: Vec<u8> = (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap())
        .collect();
    let path = common::scratch_file("volatile-reads", "bin");
    std::fs::write(&path, image).unwrap();
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_kuna"));
    cmd.args([
        "decompile",
        path.to_str().unwrap(),
        "0x1000",
        "--addr",
        "--raw-image",
        "--base",
        "0x1000",
        "--target",
        target,
        "--mode",
        "aggressive",
    ]);
    if let Some(range) = volatile {
        cmd.args(["--assert", &format!("volatile {range}"), "--assert-strict"]);
    }
    let result = cmd.output().unwrap();
    let code = String::from_utf8_lossy(&result.stdout).into_owned();
    assert!(
        result.status.success(),
        "{code}\n{}",
        String::from_utf8_lossy(&result.stderr)
    );
    code
}

/// `mov ecx,0x50000000; add rcx,4` three times, then four `mov eax,[rcx-N]`
/// reads that each overwrite the last, then `xor eax,eax; ret`.
#[test]
fn register_addressed_x86_volatile_reads_survive_into_a_dead_register() {
    let bytes = "b9000000504883c1044883c1044883c1048b018b41fc8b41f88b41f431c0c3";
    for marked in [false, true] {
        let code = decompile_raw(
            bytes,
            "x86:LE:64:default",
            marked.then_some("0x50000000+16"),
        );
        let mut previous = None;
        for offset in [0xc, 0x8, 0x4, 0x0] {
            let name = format!("dat_{:08x}", 0x50000000 + offset);
            assert_eq!(
                code.matches(&name).count(),
                usize::from(marked),
                "marked={marked}\n{code}"
            );
            if marked {
                let at = code.find(&name).unwrap();
                assert!(previous.is_none_or(|p| p < at), "read order: {code}");
                previous = Some(at);
            }
        }
    }
}

/// Flag macros re-load a memory operand: x86 `or dword [rcx+4],0x10` and
/// `add eax,[rcx+8]` behind a deep `rcx`, and MSP430 `add #1,8(r15)` into the
/// I/O range its pspec declares volatile. Each operand is one read.
#[test]
fn a_reloaded_operand_is_one_volatile_read() {
    let or = decompile_raw(
        "b9000000504883c1084883c1084883e9108349041031c0c3",
        "x86:LE:64:default",
        Some("0x50000000+16"),
    );
    assert!(or.contains("dat_50000004 = dat_50000004 | 0x10;"), "{or}");
    assert_eq!(or.matches("dat_50000004").count(), 2, "{or}");
    let add = decompile_raw(
        "b9000000504883c1044883c1044883c1044883e90c03410831c0c3",
        "x86:LE:64:default",
        Some("0x50000000+16"),
    );
    assert_eq!(add.matches("= dat_50000008;").count(), 1, "{add}");
    let msp = decompile_raw(
        "3f4018003f5004003f5004003f5004003f800c009f5308000c433041",
        "TI_MSP430:LE:16:default",
        None,
    );
    assert!(msp.contains("dat_20 = dat_20 + 1;"), "{msp}");
    assert_eq!(msp.matches("dat_20").count(), 2, "{msp}");
    let reads = decompile_raw(
        "3f4018003f5004003f5004003f5004003f8004002e4f1d4f02000c430f433041",
        "TI_MSP430:LE:16:default",
        None,
    );
    assert_eq!(reads.matches("= dat_20;").count(), 1, "{reads}");
    assert_eq!(reads.matches("= dat_22;").count(), 1, "{reads}");
}

#[test]
fn a_volatile_data_assertion_keeps_each_read_in_decompile_all() {
    let path = common::scratch_file("volatile-data-all", "bin");
    std::fs::write(
        &path,
        [0x8b, 0x04, 0x25, 0x00, 0x10, 0x60, 0x00, 0x03, 0x04, 0x25, 0x00, 0x10, 0x60, 0x00, 0xc3],
    )
    .unwrap();
    for declaration in ["data 0x601000 volatile int cursor", "data 0x601000 int volatile cursor"] {
        let (out, err, code) = common::run_kuna(&[
            "decompile-all",
            path.to_str().unwrap(),
            "--addr",
            "0x400000",
            "--raw-image",
            "--base",
            "0x400000",
            "--target",
            "x86:LE:64:default:gcc",
            "--assert",
            "function 0x400000=f",
            "--assert",
            declaration,
        ]);
        assert_eq!(code, 0, "{err}");
        assert!(out.contains("v1 = cursor;"), "{declaration}:\n{out}");
        assert!(!out.contains("cursor * 2"), "{declaration}:\n{out}");
    }
}
