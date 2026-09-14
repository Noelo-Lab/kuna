//! End-to-end gate for `pebnames` through a real object load.
//!
//! The stage tests (`tests/stages/kuna-pebnames*.xml`) cover the rendering under
//! `option pebnames on`. What an XML bytechunk cannot reach is the shipped default,
//! `auto`, whose gate is a fact the object loader records from the PE optional
//! header. This file builds minimal import-less PE images in-test and asserts:
//!
//! * a console PE32+ and a GUI PE32 get the TEB/PEB names with nothing set;
//! * a GUI PE32 function that links an MSVC C++ EH registration record through
//!   `fs:[0]` is left untyped at that default, its three-slot record intact;
//! * a native-subsystem (driver) image with the same bytes does not, because the
//!   segment base holds a KPCR there, and neither does an EFI application;
//! * a raw image under a non-Windows compiler spec is never typed even with
//!   `option pebnames on`: an x86-64 `gs:` reader and an x86 `fs:` reader stay
//!   untyped, while the same bytes under the Windows spec are typed.
//!
//! ## `.sla` precondition
//!
//! Decompiling needs the built x86 `.sla` under `specs/`; when it is absent the
//! bootstrap fails and each test prints a visible skip instead of passing.

mod common;

use std::path::PathBuf;

use kuna_console::engine::{bootstrap_from_object, bootstrap_from_raw, ConsoleProgram};
use kuna_console::ifacedecomp::{execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE};
use kuna_console::ifaceterm::ConsoleCommands;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

/// `mov rax,gs:[0x60]; movzx ecx,byte [rax+2]; test ecx,ecx; jne dbg;
/// mov eax,[rax+0xbc]; and eax,0x70; cmp eax,0x70; je dbg; mov rax,gs:[0x30];
/// mov rax,[rax+0x60]; mov rax,[rax+0x30]; mov eax,[rax+0x70]; ret; dbg: mov eax,1; ret`
const CODE64: &str = "4883ec2865488b0425600000000fb6480285c975278b80bc00000083e07083f870741965488b0425300000\
00488b4060488b40308b40704883c428c3b8010000004883c428c3";

/// The x86 twin through `fs:[0x30]`/`fs:[0x18]`.
const CODE32: &str = "64a1300000000fb6480285c9751b8b486883e17083f970741064a1180000008b40308b40188b4040c3b801000000c3";

/// MSVC's C++ EH prologue and epilogue: `push -1; push handler; push fs:[0]; ...;
/// lea eax,[esp+0xc]; mov fs:[0],eax; ...; mov ecx,[esp+0xc]; mov fs:[0],ecx`.
const EH_FRAME32: &str = "6aff680013400064a100000000505156a11020400031e0508d44240c64a30000000089ce89742408e8d3030000\
c744241400000000c7060021400089f08b4c240c64890d00000000595e83c410c3";

fn hex(s: &str) -> Vec<u8> {
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap()).collect()
}

/// A single-section PE whose `.text` at RVA 0x1000 is `code` and is the entry.
fn pe_image(bits: u32, subsystem: u16, code: &[u8]) -> Vec<u8> {
    const SA: u32 = 0x1000;
    const FA: u32 = 0x200;
    const TEXT: u32 = 0x1000;
    let mut text = code.to_vec();
    text.resize(((code.len() as u32 + FA - 1) / FA * FA) as usize, 0xcc);
    let mut b = vec![0u8; 0x40];
    b[0..2].copy_from_slice(b"MZ");
    b[0x3c..0x40].copy_from_slice(&0x40u32.to_le_bytes());
    b.extend_from_slice(b"PE\0\0");
    let (machine, opt_size, characteristics): (u16, u16, u16) =
        if bits == 64 { (0x8664, 0xf0, 0x22) } else { (0x14c, 0xe0, 0x10f) };
    for v in [machine, 1] {
        b.extend_from_slice(&v.to_le_bytes());
    }
    b.extend_from_slice(&[0u8; 12]);
    b.extend_from_slice(&opt_size.to_le_bytes());
    b.extend_from_slice(&characteristics.to_le_bytes());
    let mut opt: Vec<u8> = Vec::new();
    let u16s = |o: &mut Vec<u8>, v: &[u16]| v.iter().for_each(|x| o.extend_from_slice(&x.to_le_bytes()));
    let u32s = |o: &mut Vec<u8>, v: &[u32]| v.iter().for_each(|x| o.extend_from_slice(&x.to_le_bytes()));
    u16s(&mut opt, &[if bits == 64 { 0x20b } else { 0x10b }]);
    opt.extend_from_slice(&[14, 0]);
    u32s(&mut opt, &[text.len() as u32, 0, 0, TEXT, TEXT]);
    if bits == 64 {
        opt.extend_from_slice(&0x1_4000_0000u64.to_le_bytes());
    } else {
        u32s(&mut opt, &[0x2000, 0x40_0000]);
    }
    u32s(&mut opt, &[SA, FA]);
    u16s(&mut opt, &[6, 0, 0, 0, 6, 0]);
    u32s(&mut opt, &[0, TEXT + SA, FA, 0]);
    u16s(&mut opt, &[subsystem, if bits == 64 { 0x8160 } else { 0x8140 }]);
    for _ in 0..4 {
        if bits == 64 {
            opt.extend_from_slice(&0x1000u64.to_le_bytes());
        } else {
            u32s(&mut opt, &[0x1000]);
        }
    }
    u32s(&mut opt, &[0, 16]);
    opt.resize(opt_size as usize, 0);
    b.extend_from_slice(&opt);
    b.extend_from_slice(b".text\0\0\0");
    u32s(&mut b, &[code.len() as u32, TEXT, text.len() as u32, FA, 0, 0]);
    u16s(&mut b, &[0, 0]);
    u32s(&mut b, &[0x6000_0020]);
    b.resize(FA as usize, 0);
    b.extend_from_slice(&text);
    b
}

fn spec_roots() -> Vec<String> {
    vec![repo_root().join("specs").to_str().unwrap().to_string()]
}

/// Load `path`, run `options` then decompile `select` (a `load` command), and
/// return the transcript. `None` => specs-less skip.
fn decompile(path: &str, options: &[&str], select: &str) -> Option<String> {
    match bootstrap_from_object(path, "", &spec_roots()) {
        Ok(prog) => Some(transcript(prog, options, select)),
        Err(e) => {
            eprintln!("verify_pebnames: skipping (bootstrap failed; build `.sla` with `make specs`): {}", e.explain());
            None
        }
    }
}

/// Load the raw bytes `code` at 0x1000 under `target`, then as [`decompile`].
fn decompile_raw(stem: &str, code: &[u8], target: &str, options: &[&str]) -> Option<String> {
    let path = common::scratch_file(stem, "bin");
    std::fs::write(&path, code).unwrap();
    match bootstrap_from_raw(path.to_str().unwrap(), target, 0x1000, &[0x1000], None, &spec_roots()) {
        Ok(prog) => Some(transcript(prog, options, "load addr 0x1000")),
        Err(e) => {
            eprintln!("verify_pebnames: skipping (raw bootstrap failed; build `.sla` with `make specs`): {}", e.explain());
            None
        }
    }
}

fn transcript(mut prog: ConsoleProgram, options: &[&str], select: &str) -> String {
    prog.commit_pending_analysis().expect("analysis commit succeeds");
    let mut cmds: Vec<String> = options.iter().map(|o| format!("option pebnames {o}")).collect();
    cmds.extend([select.to_string(), "decompile".into(), "print C".into()]);
    let count = cmds.len();
    let mut status = ConsoleCommands::into_status(cmds);
    register_decomp_commands(&mut status);
    {
        let data = status.get_data_mut(DECOMPILE_MODULE).unwrap();
        let dcp = data.as_any_mut().downcast_mut::<IfaceDecompData>().unwrap();
        dcp.conf = Some(prog);
    }
    for _ in 0..count {
        execute(&mut status);
    }
    status.optr.clone()
}

fn write_pe(stem: &str, bits: u32, subsystem: u16, code: &str) -> String {
    let path = common::scratch_file(stem, "exe");
    std::fs::write(&path, pe_image(bits, subsystem, &hex(code))).unwrap();
    path.to_str().unwrap().to_string()
}

#[test]
fn a_console_pe32_plus_is_named_at_the_shipped_default() {
    let path = write_pe("pebnames-cui64", 64, 3, CODE64);
    let Some(out) = decompile(&path, &[], "load addr 0x140001000") else { return };
    assert!(out.contains("TEB *teb; // gs_offset"), "the GS base must be the TEB:\n{out}");
    assert!(out.contains("!teb->ProcessEnvironmentBlock->BeingDebugged"), "{out}");
    assert!(out.contains("teb->ProcessEnvironmentBlock->NtGlobalFlag & 0x70"), "{out}");
    assert!(out.contains("teb->Self->ProcessEnvironmentBlock->ProcessHeap"), "{out}");

    let Some(off) = decompile(&path, &["off"], "load addr 0x140001000") else { return };
    assert!(!off.contains("teb->"), "off must restore the untyped register:\n{off}");
    assert!(off.contains("; // gs_offset"), "{off}");
}

#[test]
fn a_gui_pe32_is_named_at_the_shipped_default() {
    let path = write_pe("pebnames-gui32", 32, 2, CODE32);
    let Some(out) = decompile(&path, &[], "load addr 0x401000") else { return };
    assert!(out.contains("TEB *teb; // fs_offset"), "the FS base must be the TEB:\n{out}");
    assert!(out.contains("teb->ProcessEnvironmentBlock->BeingDebugged"), "{out}");
    assert!(out.contains("teb->Self->ProcessEnvironmentBlock->ProcessHeap"), "{out}");
}

#[test]
fn a_function_that_writes_through_the_base_stays_untyped_at_the_shipped_default() {
    let path = write_pe("pebnames-ehframe32", 32, 2, EH_FRAME32);
    let Some(auto) = decompile(&path, &[], "load addr 0x401000") else { return };
    let Some(off) = decompile(&path, &["off"], "load addr 0x401000") else { return };
    assert!(!auto.contains("teb"), "a function linking an SEH record must stay untyped:\n{auto}");
    assert!(auto.contains("[3]; // stack - 0xc"), "the registration record keeps its extent:\n{auto}");
    let without_option = |t: &str| t.lines().filter(|l| !l.contains("segment-base typing set to")).collect::<Vec<_>>().join("\n");
    assert_eq!(without_option(&off), without_option(&auto), "the default must render exactly as off");
}

#[test]
fn a_driver_or_efi_image_is_left_alone_unless_forced() {
    for (stem, subsystem) in [("pebnames-native64", 1u16), ("pebnames-efi64", 10)] {
        let path = write_pe(stem, 64, subsystem, CODE64);
        let Some(auto) = decompile(&path, &[], "load addr 0x140001000") else { return };
        assert!(!auto.contains("teb->"), "subsystem {subsystem} must not be typed under auto:\n{auto}");
        assert!(auto.contains("; // gs_offset"), "{auto}");
        let Some(on) = decompile(&path, &["on"], "load addr 0x140001000") else { return };
        assert!(on.contains("TEB *teb; // gs_offset"), "`on` trusts the compiler spec:\n{on}");
    }
}

#[test]
fn a_non_windows_compiler_spec_is_never_typed_even_when_forced() {
    const GS64: &[u8] = b"\x65\x48\x8b\x04\x25\x30\x00\x00\x00\x48\x8b\x40\x60\x0f\xb6\x40\x02\xc3";
    const FS32: &[u8] = b"\x64\xa1\x18\x00\x00\x00\x8b\x40\x30\x0f\xb6\x40\x02\xc3";
    for (stem, code, target, reg) in [
        ("pebnames-gcc64", GS64, "x86:LE:64:default", "gs_offset"),
        ("pebnames-gcc32", FS32, "x86:LE:32:default", "fs_offset"),
    ] {
        let Some(gcc) = decompile_raw(stem, code, &format!("{target}:gcc"), &["on"]) else { return };
        assert!(!gcc.contains("teb"), "a {target}:gcc image must never be typed:\n{gcc}");
        assert!(gcc.contains(&format!("; // {reg}")), "{gcc}");
        let Some(win) = decompile_raw(stem, code, &format!("{target}:windows"), &["on"]) else { return };
        assert!(
            win.contains("teb->Self->ProcessEnvironmentBlock->BeingDebugged"),
            "the same bytes under the Windows spec are typed:\n{win}"
        );
    }
}
