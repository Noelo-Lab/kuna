//! (kuna `calleereginputs custom`) A whole-binary x86-32 Windows run recovers a
//! callee-saved register (EBX/ESI/EDI) custom argument when the callee reads it
//! and every direct call site in the image sets it, leaves the counterexample
//! shapes untouched, and keeps callee and callers in agreement on every surface.
//!
//! The fixture `calleeregcustom_pe_i386.exe` is hand-assembled (nasm, recipe in
//! its `.asm`); it reproduces the issue's shape (`mov edi,N; push ...; call`) and
//! the save-only, one-site-unset, address-taken and also-reads-EDX
//! counterexamples. `vcall_pe_i386.exe` checks that `custom` keeps the
//! virtual-call receiver `on` gives.
use crate::common;
use common::process;
use std::process::Command;

fn kuna() -> std::ffi::OsString {
    std::env::var_os("KUNA_TEST_BIN").unwrap_or_else(|| env!("CARGO_BIN_EXE_kuna").into())
}

fn run(args: &[&str]) -> (String, String, bool) {
    let output = Command::new(kuna()).args(args).output().expect("spawn kuna");
    (
        String::from_utf8_lossy(&output.stdout).into_owned(),
        String::from_utf8_lossy(&output.stderr).into_owned(),
        output.status.success(),
    )
}

fn bodies(mode: &str, extra: &[&str]) -> std::collections::HashMap<String, String> {
    bodies_of("calleeregcustom_pe_i386.exe", mode, extra)
}

fn bodies_of(name: &str, mode: &str, extra: &[&str]) -> std::collections::HashMap<String, String> {
    let fixture = common::fixture(name);
    let mut command = Command::new(kuna());
    command.args(["decompile-all", &fixture, "--option", "calleereginputs", mode, "--json"]).args(extra);
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
    let on = bodies("on", &[]);
    assert!(on["0x401000"].contains("int sub_401000(void)"), "on: no register argument:\n{}", on["0x401000"]);
    assert!(on["0x401060"].contains("sub_401000(1,2,3,4)"), "on: the EDI count is dropped:\n{}", on["0x401060"]);

    let custom = bodies("custom", &[]);
    assert!(
        custom["0x401000"].contains("int sub_401000(int a0)"),
        "custom: the EDI count is a parameter:\n{}",
        custom["0x401000"]
    );
    let caller = &custom["0x401060"];
    assert!(
        caller.contains("sub_401000(4,1,2,3,4)") && caller.contains("sub_401000(2,10,"),
        "custom: both calls pass the EDI count first:\n{caller}"
    );

    // edisave saves EDI and never reads it; esiarg has a call site that does not
    // set ESI; takenfn's address is stored in data; ediedx also reads EDX past
    // its first branch, so it keeps the ordinary resolution.
    for (callee, why) in [
        ("0x401020", "save-only"),
        ("0x401030", "one site does not set ESI"),
        ("0x401040", "address-taken"),
        ("0x4010f0", "also reads EDX"),
    ] {
        assert_eq!(custom[callee], on[callee], "{callee} ({why}) must be unchanged");
    }
    assert!(custom["0x401030"].contains("sub_401030(void)"), "esiarg takes no argument:\n{}", custom["0x401030"]);
}

/// `custom` extends `on`: a virtual call through the vtable of the object in
/// ECX keeps its receiver, and the C table and COM calls keep the default.
#[test]
#[cfg(all(target_arch = "x86_64", target_os = "linux"))]
fn custom_keeps_the_virtual_call_receiver() {
    let off = bodies_of("vcall_pe_i386.exe", "off", &[]);
    let on = bodies_of("vcall_pe_i386.exe", "on", &[]);
    let custom = bodies_of("vcall_pe_i386.exe", "custom", &[]);
    assert!(off["0x401000"].contains("(*a0 + 8))(5);"), "off: no receiver:\n{}", off["0x401000"]);
    assert!(on["0x401000"].contains("(*a0 + 8))(a0,5);"), "on: the object is this:\n{}", on["0x401000"]);
    let mut keys: Vec<_> = on.keys().collect();
    keys.sort();
    assert_eq!(keys.len(), custom.len(), "custom decompiles the same functions");
    for k in keys {
        assert_eq!(custom[k], on[k], "{k}: custom must match on");
    }
    assert!(custom["0x401040"].contains("(*a0 + 0x10))(5);"), "C table call:\n{}", custom["0x401040"]);
    assert!(custom["0x401050"].contains("(*a0 + 0x14))(a0,7);"), "COM copy:\n{}", custom["0x401050"]);
    // comesi and crosscall hold the object in ESI, copied from ECX before a call
    // that clobbers ECX: the COM calls keep one interface and the C call no receiver.
    for (k, call) in [("0x401060", "(*v1 + 0x18))(v1,v2);"), ("0x4010a0", "(*v1 + 8))(5);")] {
        assert!(custom[k].contains(call) && off[k] == custom[k], "{k}:\n{}", custom[k]);
    }
    // ecxcopy copies ECX from ESI before loading the vtable from ESI.
    assert!(off["0x4010c0"].contains("(*a0 + 0x24))(6);"), "off ecxcopy:\n{}", off["0x4010c0"]);
    assert!(custom["0x4010c0"].contains("(*a0 + 0x24))(a0,6);"), "ecxcopy:\n{}", custom["0x4010c0"]);
}

#[test]
#[cfg(all(target_arch = "x86_64", target_os = "linux"))]
fn off_is_unchanged_by_the_custom_extension() {
    let off = bodies("off", &[]);
    assert!(off["0x401000"].contains("sub_401000(void)"));
}

#[test]
#[cfg(all(target_arch = "x86_64", target_os = "linux"))]
fn custom_surfaces_agree_or_say_why_not() {
    let fixture = common::fixture("calleeregcustom_pe_i386.exe");

    // The serial project export prints the parameter in the `.c` and the `.h`.
    let dir = common::scratch_file("calleeregcustom", "kuna");
    let dir = dir.to_str().unwrap();
    let (_, stderr, ok) = run(&["decompile-project", &fixture, "-o", dir, "--option", "calleereginputs", "custom"]);
    assert!(ok, "decompile-project failed:\n{stderr}");
    let c = std::fs::read_to_string(format!("{dir}/calleeregcustom_pe_i386.exe.c")).unwrap();
    let h = std::fs::read_to_string(format!("{dir}/calleeregcustom_pe_i386.exe.h")).unwrap();
    assert!(c.contains("int sub_401000(int a0)") && c.contains("sub_401000(4,1,2,3,4)"), "project .c:\n{c}");
    assert!(h.contains("int sub_401000(int a0);"), "project .h:\n{h}");

    // A narrowed run is not the whole-binary decision: neither side changes, and
    // the run says so.
    let narrowed = bodies("custom", &["--addr", "0x401000", "--addr", "0x401060"]);
    assert!(narrowed["0x401000"].contains("int sub_401000(void)"), "narrowed callee:\n{}", narrowed["0x401000"]);
    assert!(narrowed["0x401060"].contains("sub_401000(1,2,3,4)"), "narrowed caller:\n{}", narrowed["0x401060"]);
    let (_, stderr, _) = run(&["decompile-all", &fixture, "--addr", "0x401000", "--option", "calleereginputs", "custom"]);
    assert!(stderr.contains("calleereginputs custom acts only on a whole-binary"), "narrowed note:\n{stderr}");

    // A worker pool cannot hold the whole-image decision, so naming it is refused.
    let (_, stderr, ok) = run(&["decompile-all", &fixture, "--jobs", "2", "--option", "calleereginputs", "custom"]);
    assert!(!ok && stderr.contains("calleereginputs custom and --jobs are exclusive"), "--jobs:\n{stderr}");

    // A streamed export prints in decompile order and warns that `custom` is inert.
    let stream = common::scratch_file("calleeregcustom-stream", "kuna");
    let (_, stderr, ok) = run(&[
        "decompile-project",
        &fixture,
        "-o",
        stream.to_str().unwrap(),
        "--stream",
        "--option",
        "calleereginputs",
        "custom",
    ]);
    assert!(ok && stderr.contains("calleereginputs custom has no effect"), "--stream:\n{stderr}");
}
