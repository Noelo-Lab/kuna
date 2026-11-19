//! Raw firmware front end: a Cortex-M flash dump loads with no `--target`,
//! `--base` or `--entry`, from its vector table, and a firmware container from
//! its header; `--svd` types its peripherals; `kuna unpack` writes the modules
//! of a UEFI firmware volume.

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

#[test]
fn unpack_decodes_efi_standard_and_tiano_compressed_files() {
    let fv = common::fixture("uefi_tiano_efi_compressed.fv");
    let (out, err, code) = common::run_kuna(&["unpack", &fv, "--list"]);
    assert_eq!(code, 0, "{err}");
    assert!(out.contains("2 modules in 1 volumes"), "{out}");
    assert!(out.contains("DiskIoDxe.efi") && out.contains("DiskIoDxe_2.efi"), "{out}");
    assert!(!out.contains("undecoded"), "{out}");
}

#[test]
fn unpack_protocols_indexes_what_each_module_installs() {
    let fv = common::fixture("uefi_tiano_efi_compressed.fv");
    let dir = tempfile::tempdir().unwrap();
    let out_dir = dir.path().to_string_lossy().into_owned();
    let (_, err, code) = common::run_kuna(&["unpack", &fv, "-o", &out_dir, "--protocols"]);
    assert_eq!(code, 0, "{err}");
    let index = std::fs::read_to_string(dir.path().join("kuna-uefi.index")).unwrap();
    assert!(index.starts_with("kuna-uefi-index 1\nmodule DiskIoDxe.efi 07\n"), "{index}");
    let binding = "install DiskIoDxe.efi ab31a01843b41a4da5c00c09261e9f71 1920 EFI_DRIVER_BINDING_PROTOCOL \
                   Supported=24f Start=aac Stop=5b9";
    assert!(index.contains(binding), "{index}");
    let module = dir.path().join("DiskIoDxe.efi").to_string_lossy().into_owned();
    let (out, err, code) = common::run_kuna(&["decompile", &module, "_ModuleEntryPoint"]);
    assert_eq!(code, 0, "{err}");
    assert!(out.contains("gEfiDevicePathUtilitiesProtocolGuid,NULL,&mDevicePathUtilities)"), "{out}");
    assert!(!out.contains("installed by"), "nothing in this volume installs the located protocol: {out}");
}

#[test]
fn intel_hex_with_svd_names_peripherals_and_interrupts() {
    let hex = common::fixture("cortexm3_raw.hex");
    let svd = common::fixture("cortexm3_raw.svd");
    let (out, err, code) = common::run_kuna(&["decompile-all", &hex, "--svd", &svd]);
    assert_eq!(code, 0, "{err}");
    assert!(err.contains("Intel HEX: 516 bytes at 0x8000000"), "{err}");
    assert!(out.contains("void USART1_IRQHandler(void)"), "{out}");
    assert!(out.contains("USART1.DR = 0x78;"));
    assert!(out.contains("GPIOA.ODR = GPIOA.ODR ^ 0x20;"));
}

#[test]
fn a_mediatek_md1img_loads_its_modem_rom_with_debug_names() {
    let img = common::fixture("md1img_mipsel_test.img");
    let (out, err, code) = common::run_kuna(&["decompile-all", &img]);
    assert_eq!(code, 0, "{err}");
    assert!(err.contains("MediaTek md1img: 68 bytes at 0x90000000, target MIPS:LE:32:default"), "{err}");
    assert!(out.contains("int scale(int a0)"), "{out}");
    assert!(out.contains("return scale(a0) ^ 0x5a;"), "{out}");
}

#[test]
fn a_bios_image_loads_in_real_mode_at_its_reset_vector() {
    let bin = common::fixture("bios_realmode_test.bin");
    let (out, err, code) = common::run_kuna(&["decompile-all", &bin]);
    assert_eq!(code, 0, "{err}");
    assert!(err.contains("PC BIOS image: 65536 bytes at 0xf0000, target x86:LE:16:Real Mode, entries [ffff0]"), "{err}");
    assert!(out.contains("sub_fe000();"), "a near call resolves inside segment F000: {out}");
}
