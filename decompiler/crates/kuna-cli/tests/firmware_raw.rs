//! Raw firmware front end: a Cortex-M flash dump loads with no `--target`,
//! `--base` or `--entry`, from its vector table.

use crate::common;

#[test]
fn cortexm_dump_is_configured_from_its_vector_table() {
    let fixture = common::fixture("cortexm3_raw.bin");
    let (out, err, code) = common::run_kuna(&["decompile-all", &fixture]);
    assert_eq!(code, 0, "{err}");
    assert!(err.contains("Cortex-M vector table: base 0x8000000"), "{err}");
    assert!(out.contains("// Function: Reset_Handler @ 0x8000174"), "{out}");
    assert!(out.contains("void SysTick_Handler(void)"));
    assert!(out.contains("// Function: IRQ37_Handler @ 0x8000108"));
    assert!(out.contains("dat_40011004 = 0x78;"), "a peripheral store survives");
}

#[test]
fn an_explicit_base_is_kept() {
    let fixture = common::fixture("cortexm3_raw.bin");
    let (_, err, code) = common::run_kuna(&[
        "functions", &fixture, "--raw-image", "--target", "ARM:LE:32:Cortex", "--base", "0x8000000",
        "--isa", "thumb", "--entry", "0x8000174",
    ]);
    assert_eq!(code, 0, "{err}");
    assert!(!err.contains("Cortex-M vector table"));
}
