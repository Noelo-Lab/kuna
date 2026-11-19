//! Firmware front end: a raw Cortex-M dump, or a firmware container (U-Boot
//! uImage, Intel HEX, S-record, bFLT, Qualcomm MBN, MediaTek
//! md1img, PC BIOS image), gets the `--raw-image --target --base --entry`
//! arguments it would otherwise need by hand
//! (`kuna_analysis::loader::firmware`). `--svd FILE` adds a CMSIS-SVD
//! device's peripherals as typed volatile globals and its interrupt names.
//!
//! Runs before the subcommand parses its arguments, for the subcommands that
//! take a raw image. The binary must be the first argument. A container is
//! flattened into a file in an owned temporary directory, as an archive
//! member is staged, and that path replaces the binary in the arguments.

use std::path::{Path, PathBuf};

use kuna_analysis::loader::firmware::{containers, cortexm, esp, svd};

const RAW_COMMANDS: [&str; 4] = ["decompile", "decompile-all", "decompile-project", "functions"];
const CORTEX_TARGET: &str = "ARM:LE:32:Cortex";
/// The ARMv7-M/ARMv8-M Device memory regions: Peripheral and System (PPB).
const DEVICE_REGIONS: [&str; 2] = ["0x40000000+0x20000000", "0xe0000000+0x20000000"];

/// The image a subcommand will load once a container is peeled.
struct Image {
    bytes: Vec<u8>,
    /// Where the container says the bytes load.
    base: Option<u64>,
    entries: Vec<u64>,
    staged: Option<tempfile::TempDir>,
}

enum Peeled {
    /// The container wrapped an object file, now staged; nothing more applies.
    Object(tempfile::TempDir),
    Raw(Image),
}

fn has(args: &[String], flag: &str) -> bool {
    args.iter().any(|a| a == flag)
}

fn value(args: &[String], flag: &str) -> Option<String> {
    args.iter().position(|a| a == flag).and_then(|i| args.get(i + 1)).cloned()
}

/// Rewrite `args` with the firmware layout filled in; unchanged when nothing
/// applies. The returned guard owns the staged image of a container, if any.
/// An unreadable or malformed `--svd` file is an error, and so is a
/// recognized container whose payload cannot be laid out.
pub(crate) fn prepare(sub: &str, args: &mut Vec<String>) -> Result<Option<tempfile::TempDir>, String> {
    let svd_file = take_flag(args, "--svd");
    if !RAW_COMMANDS.contains(&sub) {
        return match svd_file {
            Some(_) => Err(format!("--svd does not apply to {sub}")),
            None => Ok(None),
        };
    }
    let device = match &svd_file {
        Some(f) => Some(svd::parse(&std::fs::read(f).map_err(|e| format!("could not read {f}: {e}"))?)?),
        None => None,
    };
    let Some(binary) = args.first().filter(|a| !a.starts_with("--")).cloned() else { return Ok(None) };
    if is_object(&binary) {
        return Ok(None);
    }
    let Ok(bytes) = std::fs::read(&binary) else { return Ok(None) };
    if !has(args, "--raw-image") && esp::parse(&bytes).is_some() {
        return Ok(None);
    }
    let image = match peel_container(&binary, bytes, args)? {
        Peeled::Object(dir) => return Ok(Some(dir)),
        Peeled::Raw(image) => image,
    };
    let table = probe_cortex(&image, args);
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
        for addr in handlers.iter().map(|h| h.0).chain(image.entries.iter().copied()) {
            args.extend(["--entry".into(), format!("{addr:#x}")]);
        }
    }
    for (addr, name) in &handlers {
        args.extend(["--define-function".into(), format!("{addr:#x}={name}")]);
    }
    if let Some(device) = &device {
        declare_device(device, args);
    }
    Ok(image.staged)
}

/// Replace a container by what it holds: the wrapped object file, or a flat
/// image staged as a raw image with the processor, base and entries its
/// header states. A file that is no container is the image itself.
fn peel_container(binary: &str, bytes: Vec<u8>, args: &mut Vec<String>) -> Result<Peeled, String> {
    let as_is = |bytes| Ok(Peeled::Raw(Image { bytes, base: None, entries: Vec::new(), staged: None }));
    if has(args, "--raw-image") {
        return as_is(bytes);
    }
    let flat = match containers::detect(&bytes)? {
        None => return as_is(bytes),
        Some(containers::Container::Embedded { format, bytes: inner }) => {
            eprintln!("[kuna firmware] {format}: loading the ELF it wraps ({} bytes)", inner.len());
            let (dir, path) = stage(binary, "elf", &inner)?;
            args[0] = path.display().to_string();
            return Ok(Peeled::Object(dir));
        }
        Some(containers::Container::Flat(flat)) => flat,
    };
    let wants_flash = value(args, "--target").is_some_and(|t| t.starts_with("x86:LE:32") || t.starts_with("x86:LE:64"));
    let flat = if flat.format == "PC BIOS image" && wants_flash { containers::bios_flash(&bytes).unwrap_or(flat) } else { flat };
    if let Some(note) = &flat.note {
        eprintln!("[kuna firmware] {}: {note}", flat.format);
    }
    eprintln!(
        "[kuna firmware] {}: {} bytes at {:#x}, target {}, entries {:x?}",
        flat.format,
        flat.bytes.len(),
        flat.base,
        flat.target.as_deref().unwrap_or("(from --target or the vector table)"),
        flat.entries
    );
    let (dir, path) = stage(binary, "bin", &flat.bytes)?;
    args[0] = path.display().to_string();
    args.push("--raw-image".into());
    if !flat.symbols.is_empty() {
        let list = dir.path().join("functions");
        let text: String = flat.symbols.iter().map(|(a, n)| format!("{a:#x}={n}\n")).collect();
        std::fs::write(&list, text).map_err(|e| format!("could not write {}: {e}", list.display()))?;
        args.extend(["--define-function".into(), format!("@{}", list.display())]);
    }
    if !has(args, "--target") {
        if let Some(t) = &flat.target {
            args.extend(["--target".into(), t.clone()]);
        }
    }
    if !has(args, "--base") && (flat.target.is_some() || has(args, "--target")) {
        args.extend(["--base".into(), format!("{:#x}", flat.base)]);
    }
    if flat.thumb && !has(args, "--isa") {
        args.extend(["--isa".into(), "thumb".into()]);
    }
    Ok(Peeled::Raw(Image { bytes: flat.bytes, base: Some(flat.base), entries: flat.entries, staged: Some(dir) }))
}

/// Probe for a Cortex-M vector table where no base is given and the target is
/// unstated or 32-bit ARM, and state the load it implies.
fn probe_cortex(image: &Image, args: &mut Vec<String>) -> Option<cortexm::VectorTable> {
    let raw = has(args, "--raw-image");
    let target = value(args, "--target");
    let cortex = match &target {
        Some(t) => raw && t.starts_with("ARM:LE:32"),
        None => true,
    };
    if !cortex || has(args, "--base") {
        return None;
    }
    let table = cortexm::probe(&image.bytes, image.base)?;
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
    Some(table)
}

/// Each SVD peripheral: its structure, a typed global at its base, and a
/// volatile range over its registers.
fn declare_device(device: &svd::Device, args: &mut Vec<String>) {
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

/// Write `bytes` as `<binary stem>.<ext>` in a fresh temporary directory.
fn stage(binary: &str, ext: &str, bytes: &[u8]) -> Result<(tempfile::TempDir, PathBuf), String> {
    let dir = tempfile::Builder::new().prefix("kuna-firmware-").tempdir().map_err(|e| format!("could not stage {binary}: {e}"))?;
    let stem = Path::new(binary).file_name().map_or("image".into(), |n| n.to_string_lossy().to_string());
    let path = dir.path().join(format!("{stem}.{ext}"));
    std::fs::write(&path, bytes).map_err(|e| format!("could not write {}: {e}", path.display()))?;
    Ok((dir, path))
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
