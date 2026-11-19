//! Firmware front end: a raw Cortex-M dump gets the `--raw-image --target
//! --base --isa --entry` arguments it would otherwise need by hand, from its
//! vector table (`kuna_analysis::loader::firmware::cortexm`).
//!
//! Runs before the subcommand parses its arguments, for the subcommands that
//! take a raw image. The binary must be the first argument.

use kuna_analysis::loader::firmware::cortexm;

const RAW_COMMANDS: [&str; 4] = ["decompile", "decompile-all", "decompile-project", "functions"];
const CORTEX_TARGET: &str = "ARM:LE:32:Cortex";
/// The ARMv7-M/ARMv8-M Device memory regions: Peripheral and System (PPB).
const DEVICE_REGIONS: [&str; 2] = ["0x40000000+0x20000000", "0xe0000000+0x20000000"];

/// Rewrite `args` with the firmware layout filled in; unchanged when nothing
/// applies.
pub(crate) fn prepare(sub: &str, args: &mut Vec<String>) {
    if !RAW_COMMANDS.contains(&sub) {
        return;
    }
    let Some(binary) = args.first().filter(|a| !a.starts_with("--")).cloned() else { return };
    if is_object(&binary) {
        return;
    }
    let Ok(bytes) = std::fs::read(&binary) else { return };
    let has = |args: &[String], flag: &str| args.iter().any(|a| a == flag);
    let value = |args: &[String], flag: &str| args.iter().position(|a| a == flag).and_then(|i| args.get(i + 1)).cloned();
    let raw = has(args, "--raw-image");
    let target = value(args, "--target");
    let cortex = match &target {
        Some(t) => raw && t.starts_with("ARM:LE:32"),
        None => true,
    };
    if !cortex || has(args, "--base") {
        return;
    }
    let Some(table) = cortexm::probe(&bytes, None) else { return };
    if !raw {
        args.push("--raw-image".into());
    }
    if target.is_none() {
        args.extend(["--target".into(), CORTEX_TARGET.into()]);
    }
    args.extend(["--base".into(), format!("{:#x}", table.base)]);
    if !has(args, "--isa") {
        args.extend(["--isa".into(), "thumb".into()]);
    }
    for device in DEVICE_REGIONS {
        args.extend(["--assert".into(), format!("volatile {device}")]);
    }
    eprintln!(
        "[kuna firmware] Cortex-M vector table: base {:#x}, initial SP {:#x}, {} handlers, reset {:#x}",
        table.base,
        table.initial_sp,
        table.handlers.len(),
        table.reset()
    );
    if sub != "decompile" && !has(args, "--entry") && !has(args, "--addr") {
        for h in &table.handlers {
            args.extend(["--entry".into(), format!("{:#x}", h.addr)]);
        }
    }
    for h in &table.handlers {
        args.extend(["--define-function".into(), format!("{:#x}={}", h.addr, h.name)]);
    }
}

/// Formats the loader parses itself, by their magic; an unreadable file too,
/// so the subcommand reports it.
fn is_object(path: &str) -> bool {
    use std::io::Read as _;
    let mut magic = [0u8; 4];
    let Ok(mut file) = std::fs::File::open(path) else { return true };
    let Ok(n) = file.read(&mut magic) else { return true };
    let magic = &magic[..n];
    magic == b"\x7fELF"
        || magic.starts_with(b"MZ")
        || magic.starts_with(b"VZ")
        || matches!(magic, [0xfe | 0xce | 0xcf | 0xca, 0xed | 0xfa | 0xfe, ..])
        || magic == b"dyld"
        || magic == b"!<ar"
}
