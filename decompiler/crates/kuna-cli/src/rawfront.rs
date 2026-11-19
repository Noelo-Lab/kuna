//! Firmware front end: a raw Cortex-M dump gets the `--raw-image --target
//! --base --isa --entry` arguments it would otherwise need by hand, from its
//! vector table (`kuna_analysis::loader::firmware::cortexm`). `--svd FILE`
//! adds a CMSIS-SVD device's peripherals as typed volatile globals and its
//! interrupt names.
//!
//! Runs before the subcommand parses its arguments, for the subcommands that
//! take a raw image. The binary must be the first argument.

use kuna_analysis::loader::firmware::{cortexm, svd};

const RAW_COMMANDS: [&str; 4] = ["decompile", "decompile-all", "decompile-project", "functions"];
const CORTEX_TARGET: &str = "ARM:LE:32:Cortex";
/// The ARMv7-M/ARMv8-M Device memory regions: Peripheral and System (PPB).
const DEVICE_REGIONS: [&str; 2] = ["0x40000000+0x20000000", "0xe0000000+0x20000000"];

/// Rewrite `args` with the firmware layout filled in; unchanged when nothing
/// applies. An unreadable or malformed `--svd` file is an error.
pub(crate) fn prepare(sub: &str, args: &mut Vec<String>) -> Result<(), String> {
    let svd_file = take_flag(args, "--svd");
    if !RAW_COMMANDS.contains(&sub) {
        return match svd_file {
            Some(_) => Err(format!("--svd does not apply to {sub}")),
            None => Ok(()),
        };
    }
    let device = match &svd_file {
        Some(f) => Some(svd::parse(&std::fs::read(f).map_err(|e| format!("could not read {f}: {e}"))?)?),
        None => None,
    };
    let Some(binary) = args.first().filter(|a| !a.starts_with("--")).cloned() else { return Ok(()) };
    if is_object(&binary) {
        return Ok(());
    }
    let Ok(bytes) = std::fs::read(&binary) else { return Ok(()) };
    let has = |args: &[String], flag: &str| args.iter().any(|a| a == flag);
    let value = |args: &[String], flag: &str| args.iter().position(|a| a == flag).and_then(|i| args.get(i + 1)).cloned();
    let raw = has(args, "--raw-image");
    let target = value(args, "--target");
    let cortex = match &target {
        Some(t) => raw && t.starts_with("ARM:LE:32"),
        None => true,
    };
    let table = if cortex && !has(args, "--base") { cortexm::probe(&bytes, None) } else { None };
    if let Some(table) = &table {
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
    }
    let handlers: Vec<(u64, String)> = table
        .iter()
        .flat_map(|t| &t.handlers)
        .map(|h| {
            let irq = h.name.strip_prefix("IRQ").and_then(|n| n.strip_suffix("_Handler")).and_then(|n| n.parse::<u32>().ok());
            let named = irq.and_then(|n| device.as_ref()?.interrupts.get(&n)).map(|n| format!("{n}_IRQHandler"));
            (h.addr, named.unwrap_or_else(|| h.name.clone()))
        })
        .collect();
    if sub != "decompile" && !has(args, "--entry") && !has(args, "--addr") && has(args, "--raw-image") {
        for (addr, _) in &handlers {
            args.extend(["--entry".into(), format!("{addr:#x}")]);
        }
    }
    for (addr, name) in &handlers {
        args.extend(["--define-function".into(), format!("{addr:#x}={name}")]);
    }
    if let Some(device) = &device {
        let mut declared = std::collections::BTreeSet::new();
        for p in device.peripherals.iter().filter(|p| !p.registers.is_empty()) {
            if declared.insert(p.type_name.clone()) {
                args.extend(["--assert".into(), format!("typedef {}", p.c_struct())]);
            }
            args.extend(["--assert".into(), format!("data {:#x} struct {} {}", p.base, p.type_name, p.name)]);
            args.extend(["--assert".into(), format!("volatile {:#x}+{:#x}", p.base, p.extent())]);
        }
        eprintln!(
            "[kuna firmware] SVD {}: {} peripherals, {} interrupts",
            device.name,
            device.peripherals.len(),
            device.interrupts.len()
        );
    }
    Ok(())
}

/// Remove `flag VALUE` from `args`, returning the value.
fn take_flag(args: &mut Vec<String>, flag: &str) -> Option<String> {
    let i = args.iter().position(|a| a == flag)?;
    let value = args.get(i + 1).cloned();
    args.drain(i..(i + 2).min(args.len()));
    value
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
