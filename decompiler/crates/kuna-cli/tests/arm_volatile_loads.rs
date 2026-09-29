mod common;
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
