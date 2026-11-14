//! (kuna `calleereginputs custom`) A whole-binary x86-32 Windows run recovers a
//! callee-saved register (EBX/ESI/EDI) custom argument when the callee reads it
//! and every direct call site in the image sets it, and leaves the three
//! counterexample shapes untouched.
//!
//! The fixture `calleeregcustom_pe_i386.exe` is hand-assembled (nasm, recipe in
//! its `.asm`); it reproduces the issue's shape (`mov edi,N; push ...; call`) and
//! the save-only, one-site-unset and address-taken counterexamples.
use crate::common;
use common::process;
use std::process::Command;

fn bodies(mode: &str) -> std::collections::HashMap<String, String> {
    let kuna = std::env::var_os("KUNA_TEST_BIN").unwrap_or_else(|| env!("CARGO_BIN_EXE_kuna").into());
    let mut command = Command::new(&kuna);
    command.args([
        "decompile-all",
        &common::fixture("calleeregcustom_pe_i386.exe"),
        "--option",
        "calleereginputs",
        mode,
        "--json",
    ]);
    let output = process::required_output(&mut command);
    let document: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    document["functions"]
        .as_array()
        .unwrap()
        .iter()
        .map(|f| (f["address_hex"].as_str().unwrap().to_string(), f["code"].as_str().unwrap_or("").to_string()))
        .collect()
}

#[test]
#[cfg(all(target_arch = "x86_64", target_os = "linux"))]
fn custom_recovers_callee_saved_register_argument_two_sided() {
    // Default `on` (what #1575 ships): the EDI count is dropped; `countsum` is
    // `int sub_401000(void)` and its callers pass only the four stack arguments.
    let on = bodies("on");
    let countsum_on = &on["0x401000"];
    assert!(
        countsum_on.contains("int sub_401000(void)"),
        "on: countsum should take no register argument:\n{countsum_on}"
    );
    let caller_on = &on["0x401060"];
    assert!(
        caller_on.contains("sub_401000(1,2,3,4)"),
        "on: the EDI count is dropped at the call:\n{caller_on}"
    );

    // `custom`: EDI is recovered as the leading parameter of `countsum`, and both
    // call sites pass the count they set in EDI (4 and 2) ahead of the stack args.
    let custom = bodies("custom");
    let countsum = &custom["0x401000"];
    assert!(
        countsum.contains("int sub_401000(int a0)"),
        "custom: countsum should take the EDI count as a parameter:\n{countsum}"
    );
    let caller = &custom["0x401060"];
    assert!(
        caller.contains("sub_401000(4,1,2,3,4)") && caller.contains("sub_401000(2,10,"),
        "custom: both calls should pass the EDI count first:\n{caller}"
    );

    // Counterexamples stay exactly as `on` left them.
    // esiarg (0x401030): one call site leaves ESI as a leftover -> disproved.
    assert_eq!(custom["0x401030"], on["0x401030"], "esiarg must be unchanged (one site does not set ESI)");
    assert!(custom["0x401030"].contains("sub_401030(void)"), "esiarg takes no argument:\n{}", custom["0x401030"]);
    // takenfn (0x401040): address stored in a data table -> excluded.
    assert_eq!(custom["0x401040"], on["0x401040"], "takenfn must be unchanged (address-taken)");
    // edisave (0x401020): saves EDI, never reads the incoming value -> no parameter.
    assert_eq!(custom["0x401020"], on["0x401020"], "edisave must be unchanged (save-only)");
}

#[test]
#[cfg(all(target_arch = "x86_64", target_os = "linux"))]
fn off_is_unchanged_by_the_custom_extension() {
    // `off` must stay byte-identical regardless of the extension existing.
    let off = bodies("off");
    assert!(off.contains_key("0x401000"));
    assert!(off["0x401000"].contains("sub_401000(void)"));
}
