//! Raw firmware front end: a Cortex-M flash dump loads with no `--target`,
//! `--base` or `--entry`, from its vector table; `--svd` types its peripherals.

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

#[test]
fn svd_names_peripherals_and_interrupts() {
    let bin = common::fixture("cortexm3_raw.bin");
    let svd = common::fixture("cortexm3_raw.svd");
    let (out, err, code) = common::run_kuna(&["decompile-all", &bin, "--svd", &svd]);
    assert_eq!(code, 0, "{err}");
    assert!(err.contains("SVD MINI_F4: 3 peripherals, 1 interrupts"), "{err}");
    assert!(out.contains("void USART1_IRQHandler(void)"), "{out}");
    assert!(out.contains("USART1.DR = 0x78;"), "{out}");
    assert!(out.contains("GPIOA.ODR = GPIOA.ODR ^ 0x20;"), "{out}");
}

#[test]
fn svd_is_refused_where_it_does_not_apply() {
    let bin = common::fixture("cortexm3_raw.bin");
    let svd = common::fixture("cortexm3_raw.svd");
    let (_, err, code) = common::run_kuna(&["strings", &bin, "--svd", &svd]);
    assert_eq!(code, 2, "{err}");
    assert!(err.contains("--svd does not apply to strings"), "{err}");
}
