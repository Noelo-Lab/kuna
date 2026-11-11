//! **fpuemu** — decode an 8087-emulator program's `INT 34h`-`INT 3Dh` as the
//! x87 instructions they stand for.
//!
//! A DOS program built by Microsoft's or Borland's compilers for the 8087
//! emulator carries no x87 opcodes: the linker turns each `WAIT; ESC` pair into
//! a software interrupt the emulator services (or patches back to the real
//! instruction once it finds a coprocessor). `INT 34h+n` stands for the ESC
//! opcode `D8h+n` and is followed by the instruction's ModR/M and displacement;
//! `INT 3Ch` is followed by a byte holding the segment override (bits 6-7: DS,
//! SS, CS, ES) and the ESC opcode (bits 0-2), then the ModR/M; `INT 3Dh` is
//! `FWAIT`. Read as plain x86 the operand bytes become garbage instructions:
//! `CD 35 E8` (`FLD1`) is `INT 35h` and a near `CALL`, so the program grows
//! phantom functions and bad-instruction warnings where it does float math.
//!
//! `x86-16.sla` decodes the emulator forms as the x87 instruction, with its
//! p-code, wherever the context variable `fpuEmu` is 1 (default 0). This module
//! paints that variable over the whole code space: `on` always, `off` never,
//! and `auto` (the default) when the loader found the emulator's code in the
//! image ([`emulator_evidence`]; MS-DOS executables only). Languages without
//! `fpuEmu` ignore the option.

use kuna_base::address::Address;

use crate::architecture::Architecture;

/// The `x86-16.sla` context variable that turns the emulator forms on.
pub const FPU_EMU: &[u8] = b"fpuEmu";

/// How many emulated instructions must sit next to an emulated `FWAIT` before
/// [`emulator_evidence`] calls an image emulator code.
pub const EVIDENCE_WAIT_PAIRS: usize = 2;

/// `option fpuemu`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FpuEmuMode {
    /// Decode `INT 34h`-`3Dh` as interrupts.
    Off,
    /// Decode them as x87 instructions when the image carries emulator code.
    #[default]
    Auto,
    /// Decode them as x87 instructions in every 16-bit x86 image.
    On,
}

impl FpuEmuMode {
    /// The catalog `current` token.
    pub fn as_str(self) -> &'static str {
        match self {
            FpuEmuMode::Off => "off",
            FpuEmuMode::Auto => "auto",
            FpuEmuMode::On => "on",
        }
    }

    /// Stored form for the `Architecture` field.
    pub fn as_u8(self) -> u8 {
        match self {
            FpuEmuMode::Off => 0,
            FpuEmuMode::Auto => 1,
            FpuEmuMode::On => 2,
        }
    }

    /// Inverse of [`FpuEmuMode::as_u8`] (unknown values read as `Off`).
    pub fn from_u8(v: u8) -> FpuEmuMode {
        match v {
            1 => FpuEmuMode::Auto,
            2 => FpuEmuMode::On,
            _ => FpuEmuMode::Off,
        }
    }
}

/// Parse an `option fpuemu` value.
pub fn parse_fpuemu_mode(value: &str) -> Result<(FpuEmuMode, String), String> {
    let mode = match value.trim().to_ascii_lowercase().as_str() {
        "off" | "0" | "false" => FpuEmuMode::Off,
        "auto" | "" => FpuEmuMode::Auto,
        "on" | "1" | "true" => FpuEmuMode::On,
        other => return Err(format!("unknown fpuemu value {other:?} (expected off, auto or on)")),
    };
    Ok((mode, format!("8087-emulator decode set to {}", mode.as_str())))
}

/// The length of the emulator instruction at `at`, or `None` when the bytes
/// there are not one: `INT 34h`-`3Bh` plus a ModR/M and its 16-bit-addressing
/// displacement, `INT 3Ch` plus the segment byte, ModR/M and displacement, or
/// the two-byte `INT 3Dh`.
pub fn emulated_len(bytes: &[u8], at: usize) -> Option<usize> {
    if *bytes.get(at)? != 0xcd {
        return None;
    }
    let (modrm_at, head) = match *bytes.get(at + 1)? {
        0x34..=0x3b => (at + 2, 3),
        0x3c => (at + 3, 4),
        0x3d => return Some(2),
        _ => return None,
    };
    let modrm = *bytes.get(modrm_at)?;
    let disp = match (modrm >> 6, modrm & 7) {
        (0, 6) | (2, _) => 2,
        (1, _) => 1,
        _ => 0,
    };
    let len = head + disp;
    (at + len <= bytes.len()).then_some(len)
}

/// Whether `module` holds 8087-emulator code: at least
/// [`EVIDENCE_WAIT_PAIRS`] emulated instructions that run straight into an
/// emulated `FWAIT`, or follow one.
///
/// A compiler ends an emulated float sequence with `FWAIT` before the CPU reads
/// the result, so emulator code is full of these pairs, while the chance of one
/// in data is about one in 10^9 per byte. A table of `INT n; RET` thunks (a
/// generic interrupt-call helper) holds every `INT 34h`-`3Dh` back to back but
/// no pair: each `RET` byte reads as a register ModR/M.
pub fn emulator_evidence(module: &[u8]) -> bool {
    let mut pairs = 0usize;
    let mut at = 0usize;
    while at + 1 < module.len() {
        let Some(len) = emulated_len(module, at) else {
            at += 1;
            continue;
        };
        let next = at + len;
        if let Some(next_len) = emulated_len(module, next) {
            let wait_here = module[at + 1] == 0x3d;
            let wait_next = module[next + 1] == 0x3d;
            if wait_here != wait_next {
                pairs += 1;
                if pairs >= EVIDENCE_WAIT_PAIRS {
                    return true;
                }
                at = next + next_len;
                continue;
            }
        }
        at += 1;
    }
    false
}

/// Paint `fpuEmu` over the default code space from the architecture's
/// `fpuemu` mode and the loader's evidence. Returns whether the language has
/// the variable; a language without it is left untouched.
pub fn apply(arch: &Architecture) -> bool {
    let Some(space) = arch.manage().get_default_code_space().cloned() else {
        return false;
    };
    let on = match FpuEmuMode::from_u8(arch.fpu_emu) {
        FpuEmuMode::Off => false,
        FpuEmuMode::Auto => arch.fpu_emu_evidence,
        FpuEmuMode::On => true,
    };
    let at = Address::new(space, 0);
    let Ok(current) = arch.with_context_db(|db| db.get_variable_value(FPU_EMU, &at)) else {
        return false;
    };
    if current != u32::from(on) {
        arch.with_context_db_mut(|db| {
            let _ = db.set_variable(FPU_EMU, &at, u32::from(on));
        });
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn emulated_lengths_follow_the_modrm() {
        assert_eq!(emulated_len(&[0xcd, 0x35, 0xe8], 0), Some(3));
        assert_eq!(emulated_len(&[0xcd, 0x35, 0x06, 0x10, 0x20], 0), Some(5));
        assert_eq!(emulated_len(&[0xcd, 0x37, 0x46, 0xea], 0), Some(4));
        assert_eq!(emulated_len(&[0xcd, 0x39, 0x87, 0x10, 0x20], 0), Some(5));
        assert_eq!(emulated_len(&[0xcd, 0x3c, 0xd9, 0x07], 0), Some(4));
        assert_eq!(emulated_len(&[0xcd, 0x3d], 0), Some(2));
        assert_eq!(emulated_len(&[0xcd, 0x21], 0), None);
        assert_eq!(emulated_len(&[0xcd, 0x3e, 0xee, 0x90], 0), None);
        assert_eq!(emulated_len(&[0xcd, 0x35, 0x06, 0x10], 0), None);
    }

    #[test]
    fn emulator_code_is_evidence() {
        let code = [
            0xcd, 0x37, 0x46, 0xea, 0xcd, 0x3d, 0x83, 0xc4, 0x04, 0xcd, 0x38, 0x0e, 0xa0, 0x04,
            0x83, 0xec, 0x08, 0xcd, 0x39, 0x5e, 0xf8, 0xcd, 0x3d,
        ];
        assert!(emulator_evidence(&code));
        assert!(!emulator_evidence(&code[..9]));
    }

    #[test]
    fn an_interrupt_thunk_table_is_not_evidence() {
        let table: Vec<u8> = (0u8..=0xff).flat_map(|n| [0xcd, n, 0xc3]).collect();
        assert!(!emulator_evidence(&table));
    }

    #[test]
    fn modes_parse_and_round_trip() {
        for mode in [FpuEmuMode::Off, FpuEmuMode::Auto, FpuEmuMode::On] {
            assert_eq!(parse_fpuemu_mode(mode.as_str()).unwrap().0, mode);
            assert_eq!(FpuEmuMode::from_u8(mode.as_u8()), mode);
        }
        assert!(parse_fpuemu_mode("always").is_err());
        assert_eq!(FpuEmuMode::default(), FpuEmuMode::Auto);
    }
}
