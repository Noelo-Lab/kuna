use crate::common;
use common::process;
use std::process::Command;

#[test]
fn complete_element_copies_initialize_a_rust_local_array() {
    let kuna = std::env::var_os("KUNA_TEST_BIN")
        .unwrap_or_else(|| env!("CARGO_BIN_EXE_kuna").into());
    let render = |language: &str| {
        let output = process::required_output(Command::new(&kuna).args([
            "decompile",
            &common::fixture("typedepth_x86_64"),
            "main",
            "--language",
            language,
        ]));
        String::from_utf8(output.stdout).unwrap()
    };
    let rust = render("rust");
    assert!(rust.contains("  let mut counts: [u64; 3]; // stack - 0x58\n"), "{rust}");
    assert!(rust.contains("\n  counts = [1, 2, 3];\n  tail.value = 2;\n"), "{rust}");
    assert!(!rust.contains("counts["), "{rust}");
    assert!(rust.contains("  codes[0].op = 0x1;\n"), "{rust}");
    let c = render("c");
    assert!(c.contains("\n  counts[0] = 1;\n  counts[1] = 2;\n  counts[2] = 3;\n"), "{c}");
}
