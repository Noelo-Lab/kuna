mod common;
use std::process::Command;

fn decompile(words: &[u32], parameter: &str) -> String {
    let bytes: Vec<u8> = words.iter().flat_map(|word| word.to_le_bytes()).collect();
    decompile_bytes(&bytes, parameter, "ARM:LE:32:v5t:default")
}

fn decompile_bytes(bytes: &[u8], parameter: &str, target: &str) -> String {
    let path = common::scratch_file("volatile-qualifiers", "bin");
    std::fs::write(&path, bytes).unwrap();
    let binary = env!("CARGO_BIN_EXE_kuna");
    let mut command = match std::env::var_os("KUNA_WRAPPER") {
        Some(wrapper) => {
            let mut command = Command::new(wrapper);
            command.env("KUNA_BINARY", binary);
            command
        }
        None => Command::new(binary),
    };
    let assertion = format!("prototype ReadTwice uint4 ReadTwice({parameter})");
    command.args([
        "decompile-all",
        path.to_str().unwrap(),
        "--addr",
        "0x1000",
        "--raw-image",
        "--base",
        "0x1000",
        "--target",
        target,
        "--mode",
        "reliable",
        "--jobs",
        "1",
        "--assert",
        "function 0x1000=ReadTwice",
        "--assert",
        &assertion,
        "--assert-strict",
    ]);
    if target.starts_with("ARM:") {
        command.args(["--isa", "arm"]);
    }
    let result = command.output().unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    String::from_utf8(result.stdout).unwrap()
}

#[test]
fn x86_flag_macros_do_not_add_volatile_reads() {
    let code = decompile_bytes(
        &[0x8b, 0x07, 0x03, 0x07, 0xc3],
        "volatile uint4 *control",
        "x86:LE:64:default:gcc",
    );
    assert_eq!(code.matches("*control").count(), 3, "{code}");
}

#[test]
fn pointee_and_pointer_qualifiers_stay_on_their_declared_layer() {
    let words = [0xe5901000, 0xe5900000, 0xe0810000, 0xe12fff1e];
    for parameter in [
        "volatile uint4 *control",
        "uint4 * volatile control",
        "const uint4 *control",
        "uint4 * const control",
        "uint4 *control",
    ] {
        let code = decompile(&words, parameter);
        assert!(code.contains(&format!("ReadTwice({parameter})")), "{code}");
        assert_eq!(
            code.matches("*control").count(),
            2 + usize::from(parameter.contains("*control")),
            "{code}"
        );
    }
}

#[test]
fn pointer_arithmetic_keeps_the_volatile_pointee() {
    let words = [0xe2802004, 0xe5921000, 0xe5900000, 0xe0810000, 0xe12fff1e];
    let code = decompile(&words, "volatile uint4 *control");
    assert!(
        code.contains("ReadTwice(volatile uint4 *control)"),
        "{code}"
    );
    assert!(code.contains("control[1]"), "{code}");
    assert!(code.contains("*control"), "{code}");
    assert!(!code.contains("(uint4 *)"), "{code}");
}

#[test]
fn unused_volatile_pointee_reads_remain_live() {
    let words = [0xe5901000, 0xe5901000, 0xe3a00000, 0xe12fff1e];
    let plain = decompile(&words, "uint4 *control");
    assert_eq!(plain.matches("*control").count(), 1, "{plain}");
    let qualified = decompile(&words, "volatile uint4 *control");
    assert_eq!(qualified.matches("*control").count(), 3, "{qualified}");
}

#[test]
fn casts_for_access_width_keep_the_volatile_pointee() {
    let words = [0xe5901000, 0xe5900000, 0xe0810000, 0xe12fff1e];
    let code = decompile(&words, "volatile void *control");
    assert!(code.contains("ReadTwice(volatile void *control)"), "{code}");
    assert_eq!(
        code.matches("(volatile ").count(),
        3,
        "{code}"
    );
}

#[test]
fn writes_cast_away_const_and_keep_volatile_access() {
    let words = [0xe3a01001, 0xe5801000, 0xe3a00000, 0xe12fff1e];
    for parameter in ["const uint4 *control", "volatile void *control"] {
        let code = decompile(&words, parameter);
        assert_eq!(code.matches(")control").count(), 1, "{code}");
        if parameter.starts_with("volatile") {
            assert_eq!(code.matches("(volatile ").count(), 2, "{code}");
        } else {
            assert!(!code.contains("*(const "), "{code}");
        }
    }
}
