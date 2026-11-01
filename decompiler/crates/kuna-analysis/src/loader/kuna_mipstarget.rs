//! ELF instruction-set and compiler-model declarations beyond the container's
//! address size. Explicit language targets bypass this selection.
use kuna_base::error::{KunaError, KunaResult};
use object::{elf::*, Architecture, Object};

const MIPS_32BITMODE: u32 = 0x100;
const MIPS_FP64: u32 = 0x200;
const MIPS_MICRO: u32 = 0x0200_0000;
const MIPS16: u32 = 0x0400_0000;

#[derive(Clone, Copy, Default)]
struct MipsFloat {
    registers: Option<u32>,
    abi: Option<u32>,
}

fn agree(a: Option<u32>, b: Option<u32>) -> KunaResult<Option<u32>> {
    if a.zip(b).is_some_and(|(a, b)| a != b) {
        return Err(unsupported("conflicting MIPS floating-point declarations"));
    }
    Ok(a.or(b))
}

fn mips_float(
    file: &object::File<'_>,
    record: Option<super::kuna_mipsabi::AbiFlags>,
) -> KunaResult<MipsFloat> {
    let registers = match record.map(|r| r.0[5]).unwrap_or(0) {
        0 => None,
        1 => Some(4),
        2 => Some(8),
        _ => return Err(unsupported("unsupported MIPS floating-point register size")),
    };
    let abi = super::kuna_mipsabi::fp_abi(file, record).map_err(unsupported)?;
    Ok(MipsFloat { registers, abi })
}

fn flags(file: &object::File<'_>) -> Option<u32> {
    match file.flags() {
        object::FileFlags::Elf { e_flags, .. } => Some(e_flags),
        _ => None,
    }
}

fn unsupported(message: &str) -> KunaError {
    KunaError::lowlevel(format!("{message}; pass an explicit --target language id"))
}

fn mips_id(flags: u32, elf64: bool, endian: &str, fp: MipsFloat) -> KunaResult<String> {
    let isa = flags & EF_MIPS_ARCH;
    let isa64 = match isa {
        EF_MIPS_ARCH_3 | EF_MIPS_ARCH_4 | EF_MIPS_ARCH_5 | EF_MIPS_ARCH_64 | EF_MIPS_ARCH_64R2
        | EF_MIPS_ARCH_64R6 => true,
        EF_MIPS_ARCH_1 | EF_MIPS_ARCH_2 | EF_MIPS_ARCH_32 | EF_MIPS_ARCH_32R2
        | EF_MIPS_ARCH_32R6 => false,
        _ => return Err(unsupported("unsupported MIPS ELF ISA flags")),
    };
    let abi = flags & EF_MIPS_ABI;
    let n32 = flags & EF_MIPS_ABI2 != 0;
    if (n32 && (elf64 || abi != 0 || flags & MIPS_32BITMODE != 0))
        || (elf64 && (abi != 0 || flags & MIPS_32BITMODE != 0))
        || (abi == EF_MIPS_ABI_O64 && flags & MIPS_32BITMODE != 0)
    {
        return Err(unsupported("conflicting MIPS ELF class and ABI flags"));
    }
    let wide = elf64
        || n32
        || abi == EF_MIPS_ABI_O64
        || (isa64 && flags & MIPS_32BITMODE == 0 && abi != EF_MIPS_ABI_EABI32);
    if wide && !isa64 && isa != EF_MIPS_ARCH_1 {
        return Err(unsupported(
            "MIPS ELF ABI requires 64-bit registers but declares a 32-bit ISA",
        ));
    }
    let model = match (abi, n32) {
        (0, true) => "n32",
        (0, false) => "default",
        (EF_MIPS_ABI_O32, false) if wide => "o32",
        (EF_MIPS_ABI_O32, false) => "default",
        (EF_MIPS_ABI_O64, false) => "o64",
        (EF_MIPS_ABI_EABI32, false) => "eabi",
        _ => return Err(unsupported("unsupported MIPS ELF ABI")),
    };
    let r6 = matches!(isa, EF_MIPS_ARCH_32R6 | EF_MIPS_ARCH_64R6);
    let micro = flags & MIPS_MICRO != 0;
    let mips16 = flags & MIPS16 != 0;
    if mips16 && (micro || r6) {
        return Err(unsupported(
            "unsupported combination of MIPS ELF instruction modes",
        ));
    }
    let abi_width = match fp.abi {
        Some(1) => Some(if !wide || model == "o32" { 4 } else { 8 }),
        Some(4 | 6 | 7) => Some(8),
        None | Some(0 | 2 | 3 | 5) => None,
        _ => return Err(unsupported("unsupported MIPS floating-point ABI")),
    };
    let fpr = agree(fp.registers, abi_width)?;
    let fpr = agree(fpr, (flags & MIPS_FP64 != 0).then_some(8))?
        .unwrap_or(if r6 || (wide && model != "o32") { 8 } else { 4 });
    if (r6 && fpr != 8) || (wide && fpr == 4 && model != "o32") || (!wide && !r6 && fpr == 8) {
        return Err(unsupported(
            "no bundled MIPS language for the declared ISA, ABI and FPR width",
        ));
    }
    let variant = match (wide, elf64, r6, micro, fpr) {
        (true, false, false, true, 4) => "micro64-32addr-fp32",
        (true, false, false, false, 4) => "64-32addr-fp32",
        (true, false, true, _, _) => "64-32R6addr",
        (true, false, _, true, _) => "micro64-32addr",
        (true, false, _, _, _) => "64-32addr",
        (_, _, true, _, _) => "R6",
        (_, _, _, true, _) => "micro",
        _ => "default",
    };
    if model == "eabi" && variant != "default" {
        return Err(unsupported(
            "no bundled MIPS EABI compiler model for this instruction mode",
        ));
    }
    Ok(format!(
        "MIPS:{endian}:{}:{variant}:{model}",
        if wide { 64 } else { 32 }
    ))
}

pub(crate) fn language_id(file: &object::File<'_>) -> KunaResult<Option<String>> {
    let Some(flags) = flags(file) else {
        return Ok(None);
    };
    let endian = if file.is_little_endian() { "LE" } else { "BE" };
    match file.architecture() {
        Architecture::Mips | Architecture::Mips64 | Architecture::Mips64_N32 => {
            if flags & 0x00ff_0000 != 0 {
                return Ok(None);
            }
            let record = super::kuna_mipsabi::abi_flags(file).map_err(unsupported)?;
            if record.is_some_and(|r| r.extension(file.is_little_endian()) != 0) {
                return Ok(None);
            }
            mips_id(flags, file.is_64(), endian, mips_float(file, record)?).map(Some)
        }
        _ => Ok(None),
    }
}

pub(crate) fn notes(file: &object::File<'_>, language: &str, explicit: bool) -> Vec<String> {
    let Some(flags) = flags(file) else {
        return Vec::new();
    };
    let mut notes = Vec::new();
    match file.architecture() {
        Architecture::Mips | Architecture::Mips64 | Architecture::Mips64_N32 => {
            if !explicit
                && !file.is_64()
                && flags & (EF_MIPS_ABI | EF_MIPS_ABI2) == 0
                && language.contains(":64:")
            {
                notes.push(format!("64-bit MIPS instructions in ELF32 with no ABI declaration; assuming O64 ({language}); use --target to specify another ABI"));
            }
        }
        _ => {}
    }
    notes
}

pub(crate) fn language_ids() -> Vec<String> {
    let mut ids = std::collections::BTreeSet::new();
    for endian in ["BE", "LE"] {
        for elf64 in [false, true] {
            for isa in 0..=10 {
                for abi in [
                    0,
                    EF_MIPS_ABI2,
                    EF_MIPS_ABI_O32,
                    EF_MIPS_ABI_O64,
                    EF_MIPS_ABI_EABI32,
                ] {
                    for mode in [0, MIPS_32BITMODE, MIPS_MICRO, MIPS16] {
                        for registers in [None, Some(4), Some(8)] {
                            let fp = MipsFloat {
                                registers,
                                abi: None,
                            };
                            if let Ok(id) = mips_id((isa << 28) | abi | mode, elf64, endian, fp) {
                                ids.insert(id);
                            }
                        }
                    }
                }
            }
        }
    }
    ids.into_iter().collect()
}
