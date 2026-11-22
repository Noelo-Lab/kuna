//! AAELF32 Thumb literal loads, ADR, MOVW/MOVT and narrow branches.

use object::elf;

use super::{
    explicit_plus_implicit, read_u16, require_aligned, require_signed, sign_extend, write_u16,
    RelocationFailure, RelocationInfo,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ThumbRelocation {
    Pc8,
    MovwAbs,
    MovtAbs,
    AluPrel11_0,
    Pc12,
    Jump11,
}

impl ThumbRelocation {
    pub(super) fn from_type(kind: u32) -> Option<Self> {
        Some(match kind {
            elf::R_ARM_THM_PC8 => Self::Pc8,
            elf::R_ARM_THM_MOVW_ABS_NC => Self::MovwAbs,
            elf::R_ARM_THM_MOVT_ABS => Self::MovtAbs,
            elf::R_ARM_THM_ALU_PREL_11_0 => Self::AluPrel11_0,
            elf::R_ARM_THM_PC12 => Self::Pc12,
            // `object` uses the legacy name for R_ARM_THM_JUMP11.
            elf::R_ARM_THM_PC11 => Self::Jump11,
            _ => return None,
        })
    }

    pub(super) fn width(self) -> usize {
        match self {
            Self::Pc8 | Self::Jump11 => 2,
            Self::MovwAbs | Self::MovtAbs | Self::AluPrel11_0 | Self::Pc12 => 4,
        }
    }

    pub(super) fn targets_code(self) -> bool {
        matches!(self, Self::Jump11)
    }

    pub(super) fn apply(
        self,
        field: &mut [u8],
        le: bool,
        reloc: RelocationInfo,
        symbol: u64,
        place: u64,
        target_thumb: Option<bool>,
    ) -> Result<(), RelocationFailure> {
        // For a Thumb function, ELF st_value carries the mode in bit 0, but
        // S is the untagged address. Keep bit 0 of ordinary data addresses.
        let t = u32::from(target_thumb == Some(true));
        let s = symbol as u32 & !t;
        let s_plus_a = |implicit| s.wrapping_add(explicit_plus_implicit(reloc, implicit) as u32);
        let p = place as u32;
        let pa = p & !3;
        let hi = read_u16(&field[..2], le);

        // The object producer already encoded Thumb's PC bias into A, in
        // either the REL instruction or the explicit RELA addend.
        match self {
            Self::Pc8 => {
                if !matches!(hi & 0xf800, 0x4800 | 0xa000) {
                    return Err(RelocationFailure::InvalidEncoding);
                }
                // Unsigned word offset, with the top value (1020) encoding
                // the initial addend -4 rather than +1020 in a REL object.
                let imm = i128::from(hi & 0xff) << 2;
                let value = s_plus_a(((imm + 4) & 0x3ff) - 4).wrapping_sub(pa) as i32;
                require_aligned(value.into(), 4)?;
                if !(0..=0x3fc).contains(&value) {
                    return Err(RelocationFailure::OutOfRange);
                }
                write_u16(field, le, (hi & !0xff) | (value as u16 >> 2));
                return Ok(());
            }
            Self::Jump11 => {
                if hi & 0xf800 != 0xe000 {
                    return Err(RelocationFailure::InvalidEncoding);
                }
                if target_thumb == Some(false) {
                    return Err(RelocationFailure::RequiresVeneer);
                }
                let implicit = sign_extend(i128::from(hi & 0x7ff) << 1, 12);
                let value = i128::from(s_plus_a(implicit).wrapping_sub(p) as i32);
                require_aligned(value, 2)?;
                require_signed(value, 12)?;
                write_u16(field, le, (hi & !0x7ff) | (((value >> 1) as u16) & 0x7ff));
                return Ok(());
            }
            _ => {}
        }

        // A wide Thumb instruction is two endian-aware halfwords in execution
        // order, not one endian-aware u32.
        let lo = read_u16(&field[2..], le);
        let (hi, lo) = match self {
            Self::MovwAbs | Self::MovtAbs => {
                let movt = self == Self::MovtAbs;
                let opcode = if movt { 0xf2c0 } else { 0xf240 };
                if hi & 0xfbf0 != opcode || lo & 0x8000 != 0 || lo & 0xf00 == 0xf00 {
                    return Err(RelocationFailure::InvalidEncoding);
                }
                let imm = ((hi & 0xf) << 12) | decode_imm12(hi, lo);
                // Both initial immediates are signed 16-bit values, even MOVT.
                let value = s_plus_a(sign_extend(imm.into(), 16));
                let imm = if movt { value >> 16 } else { value | t } as u16;
                encode_imm12((hi & !0xf) | (imm >> 12), lo, imm)
            }
            Self::AluPrel11_0 => {
                // ADR.W is ADDW or SUBW with Rn = PC.
                let add = hi & !0x0400 == 0xf20f;
                if !(add || hi & !0x0400 == 0xf2af) || lo & 0x8000 != 0 || lo & 0xf00 == 0xf00 {
                    return Err(RelocationFailure::InvalidEncoding);
                }
                let imm = i128::from(decode_imm12(hi, lo));
                let value = ((s_plus_a(if add { imm } else { -imm }) | t).wrapping_sub(pa)) as i32;
                let magnitude = magnitude12(value)?;
                // Select ADDW/SUBW from the result, not the initial opcode.
                let hi = (hi & !0x00a0) | if value < 0 { 0x00a0 } else { 0 };
                encode_imm12(hi, lo, magnitude)
            }
            Self::Pc12 => {
                let opcode = hi & !0x0080;
                if !matches!(opcode, 0xf81f | 0xf83f | 0xf85f | 0xf91f | 0xf93f)
                    // Only word loads can target PC; the other encodings
                    // are memory hints or reserved, not literal loads.
                    || (opcode != 0xf85f && lo & 0xf000 == 0xf000)
                {
                    return Err(RelocationFailure::InvalidEncoding);
                }
                let imm = i128::from(lo & 0xfff);
                let value = s_plus_a(if hi & 0x0080 != 0 { imm } else { -imm }).wrapping_sub(pa)
                    as i32;
                let hi = (hi & !0x0080) | if value >= 0 { 0x0080 } else { 0 };
                (hi, (lo & !0xfff) | magnitude12(value)?)
            }
            Self::Pc8 | Self::Jump11 => unreachable!(),
        };
        write_u16(&mut field[..2], le, hi);
        write_u16(&mut field[2..], le, lo);
        Ok(())
    }
}

// MOVW, MOVT and ADR.W share i:imm3:imm8; MOVW and MOVT also use imm4.
fn decode_imm12(hi: u16, lo: u16) -> u16 {
    ((hi & 0x0400) << 1) | ((lo & 0x7000) >> 4) | (lo & 0x00ff)
}

fn encode_imm12(hi: u16, lo: u16, imm: u16) -> (u16, u16) {
    (
        (hi & !0x0400) | ((imm & 0x0800) >> 1),
        (lo & !0x70ff) | ((imm & 0x0700) << 4) | (imm & 0x00ff),
    )
}

fn magnitude12(value: i32) -> Result<u16, RelocationFailure> {
    let magnitude = value.unsigned_abs();
    if magnitude > 0xfff {
        return Err(RelocationFailure::OutOfRange);
    }
    Ok(magnitude as u16)
}

#[cfg(test)]
mod tests {
    use super::*;
    use object::{RelocationEncoding, RelocationKind};

    fn addend(value: i64, implicit: bool) -> RelocationInfo {
        RelocationInfo {
            kind: RelocationKind::Unknown,
            encoding: RelocationEncoding::Unknown,
            size: 0,
            addend: value,
            implicit_addend: implicit,
            r_type: None,
        }
    }

    #[test]
    fn pc8_uses_scaled_addend_only_for_rel_and_rejects_unrepresentable_values() {
        let kind = ThumbRelocation::Pc8;
        let rel = addend(0, true);

        let mut ldr = 0x4bffu16.to_le_bytes();
        kind.apply(&mut ldr, true, rel, 0x1020, 0x1002, None)
            .unwrap();
        assert_eq!(ldr, 0x4b07u16.to_le_bytes());

        let mut adr = 0xa3feu16.to_le_bytes();
        kind.apply(&mut adr, true, rel, 0x1004, 0x1002, None)
            .unwrap();
        assert_eq!(adr, 0xa3ffu16.to_le_bytes());
        kind.apply(&mut adr, true, addend(-4, false), 0x1400, 0x1002, None)
            .unwrap();
        assert_eq!(adr, 0xa3ffu16.to_le_bytes());

        for (symbol, reason) in [
            (0x1000, RelocationFailure::OutOfRange),
            (0x1022, RelocationFailure::Misaligned),
            (0x1404, RelocationFailure::OutOfRange),
        ] {
            let original = 0x4bffu16.to_le_bytes();
            let mut field = original;
            assert_eq!(
                kind.apply(&mut field, true, rel, symbol, 0x1002, None),
                Err(reason)
            );
            assert_eq!(field, original);
        }
    }

    #[test]
    fn pc12_encodes_backward_loads_and_keeps_non_thumb_data_addresses() {
        let kind = ThumbRelocation::Pc12;
        let rel = addend(0, true);
        // Big-endian LDRSH with a positive implicit addend of 0x234.
        let mut field = [0xf9, 0xbf, 0x72, 0x34];
        kind.apply(&mut field, false, rel, 0x1001, 0x1802, None)
            .unwrap();
        assert_eq!(field, [0xf9, 0x3f, 0x75, 0xcb]);

        let original = [0xdf, 0xf8, 0x15, 0x30];
        let mut field = original;
        let rela = addend(-4, false);
        assert_eq!(
            kind.apply(&mut field, true, rela, 0x1004, 0x2002, None),
            Err(RelocationFailure::OutOfRange)
        );
        assert_eq!(field, original);
        kind.apply(&mut field, true, rela, 0x0ff0, 0x1002, None)
            .unwrap();
        assert_eq!(field, [0x5f, 0xf8, 0x14, 0x30]);
    }

    #[test]
    fn adr_sets_the_thumb_bit_before_subtracting_place() {
        let mut field = [0x0f, 0xf2, 0x11, 0x04];
        ThumbRelocation::AluPrel11_0
            .apply(&mut field, true, addend(-4, false), 0x0ff1, 0x1000, Some(true))
            .unwrap();
        assert_eq!(field, [0xaf, 0xf2, 0x13, 0x04]);
    }
}
