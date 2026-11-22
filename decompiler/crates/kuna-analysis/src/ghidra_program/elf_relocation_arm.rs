//! Recompute Ghidra 12.1.2 ARM:LE:32 ELF relocations under the saved section layout.

use object::elf::{R_ARM_THM_PC11 as R_ARM_THM_JUMP11, R_ARM_THM_PC22 as R_ARM_THM_CALL};
use object::{Relocation, RelocationFlags, elf};

pub(super) fn width(kind: u32) -> Option<usize> {
    match kind {
        elf::R_ARM_THM_PC8 | R_ARM_THM_JUMP11 => Some(2),
        elf::R_ARM_ABS32
        | elf::R_ARM_REL32
        | elf::R_ARM_PREL31
        | elf::R_ARM_CALL
        | elf::R_ARM_JUMP24
        | R_ARM_THM_CALL
        | elf::R_ARM_THM_JUMP24
        | elf::R_ARM_THM_MOVW_ABS_NC
        | elf::R_ARM_THM_MOVT_ABS => Some(4),
        _ => None,
    }
}

pub(super) fn encode(
    reloc: &Relocation,
    original: &[u8],
    symbol: u64,
    place: u64,
    pc_bias: bool,
) -> Option<[u8; 4]> {
    let RelocationFlags::Elf { r_type: kind } = reloc.flags() else { return None };
    if width(kind)? != original.len() {
        return None;
    }
    let old = match original {
        &[a, b] => u32::from(u16::from_le_bytes([a, b])),
        &[a, b, c, d] => u32::from_le_bytes([a, b, c, d]),
        _ => return None,
    };
    // Match ARM_ElfRelocationHandler's Java int arithmetic, not ARM ABI rules.
    // Its isThumb tests `% 1 == 1`, so it never sets bit 0. The resolved
    // symbol value is already even for mapped Thumb symbols; do not clear
    // bit 0 on other symbols, such as an odd absolute value.
    let symbol = symbol as i32;
    let place = place as i32;
    let explicit = (!reloc.has_implicit_addend()).then(|| reloc.addend() as i32);
    let relative = |addend, bias| symbol.wrapping_add(addend).wrapping_sub(place).wrapping_sub(bias);
    let thumb_bias = if pc_bias { 4 } else { 0 };
    let new = match kind {
        elf::R_ARM_ABS32 => symbol.wrapping_add(explicit.unwrap_or(old as i32)) as u32,
        elf::R_ARM_REL32 => relative(explicit.unwrap_or(old as i32), 0) as u32,
        elf::R_ARM_PREL31 => {
            let addend = explicit.unwrap_or((old as i32) << 1 >> 1);
            (old & 0x8000_0000) | (relative(addend, 0) as u32 & 0x7fff_ffff)
        }
        elf::R_ARM_CALL | elf::R_ARM_JUMP24 => {
            let addend = explicit.unwrap_or((old as i32) << 8 >> 6);
            let delta = relative(addend, 2 * thumb_bias) as u32;
            let mode = if old & 0xff00_0000 == 0xfb00_0000 {
                (old & 0xfe00_0000) | ((delta & 2) << 23)
            } else {
                old & 0xff00_0000
            };
            mode | ((delta >> 2) & 0x00ff_ffff)
        }
        R_ARM_THM_CALL | elf::R_ARM_THM_JUMP24 => {
            let hi = old & 0xffff;
            let lo = old >> 16;
            let sign = (hi >> 10) & 1;
            let i1 = 1 ^ sign ^ ((lo >> 13) & 1);
            let i2 = 1 ^ sign ^ ((lo >> 11) & 1);
            let encoded =
                (sign << 24) | (i1 << 23) | (i2 << 22) | ((hi & 0x03ff) << 12) | ((lo & 0x07ff) << 1);
            let addend = ((encoded as i32) << 7 >> 7).wrapping_add(explicit.unwrap_or(0));
            // Stock keeps both J bits and the branch mode from the input,
            // even if the new displacement or target mode would change them.
            let delta = (relative(addend, 0) >> 1) as u32;
            let new_hi = (hi & 0xf800) | ((delta >> 11) & 0x07ff);
            let mut new_lo = (lo & 0xf800) | (delta & 0x07ff);
            if lo & 0x1000 == 0 {
                new_lo &= !1; // BLX
            }
            new_hi | (new_lo << 16)
        }
        elf::R_ARM_THM_PC8 => {
            // Unlike the ABI, stock ignores any embedded addend and uses
            // a halfword-scaled delta from the unaligned relocation address.
            (old & 0xff00) | ((relative(explicit.unwrap_or(0), thumb_bias) >> 1) as u32 & 0xff)
        }
        elf::R_ARM_THM_MOVW_ABS_NC | elf::R_ARM_THM_MOVT_ABS => {
            // Stock joins the halfwords in instruction order, widening the
            // second as a signed short (its sign bit is clear in MOVW/MOVT).
            let instruction = (old << 16) | ((old as i32 >> 16) as u32);
            let addend = explicit.unwrap_or_else(|| {
                let imm = ((instruction >> 4) & 0xf700) | ((instruction >> 15) & 0x0800) | (instruction & 0xff);
                imm as i16 as i32
            });
            let value = symbol.wrapping_add(addend);
            let imm = (if kind == elf::R_ARM_THM_MOVT_ABS { value >> 16 } else { value }) as u32;
            let instruction = (instruction & 0xfbf0_8f00)
                | ((imm & 0xf700) << 4)
                | ((imm & 0x0800) << 15)
                | (imm & 0xff);
            instruction.rotate_left(16)
        }
        R_ARM_THM_JUMP11 => {
            let addend = explicit.unwrap_or((old as i32) << 21 >> 20);
            (old & 0xf800) | ((relative(addend, thumb_bias) >> 1) as u32 & 0x07ff)
        }
        _ => return None,
    };
    Some(new.to_le_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;
    use object::write::{Object as Writer, Relocation as WriteRelocation, Symbol, SymbolSection};
    use object::{
        Architecture, BinaryFormat, Endianness, Object, ObjectSection, SectionKind, SymbolFlags, SymbolKind, SymbolScope,
    };

    fn relocation(kind: u32) -> Relocation {
        let mut obj = Writer::new(BinaryFormat::Elf, Architecture::Arm, Endianness::Little);
        let text = obj.add_section(Vec::new(), b".text".to_vec(), SectionKind::Text);
        obj.append_section_data(text, &[0; 4], 4);
        let symbol = obj.add_symbol(Symbol {
            name: b"target".to_vec(),
            value: 0,
            size: 0,
            kind: SymbolKind::Text,
            scope: SymbolScope::Linkage,
            weak: false,
            section: SymbolSection::Undefined,
            flags: SymbolFlags::None,
        });
        obj.add_relocation(
            text,
            WriteRelocation { offset: 0, symbol, addend: 0, flags: RelocationFlags::Elf { r_type: kind } },
        )
        .unwrap();
        let bytes = obj.write().unwrap();
        let file = object::File::parse(&*bytes).unwrap();
        let (_, reloc) = file.section_by_name(".text").unwrap().relocations().next().unwrap();
        assert!(reloc.has_implicit_addend());
        reloc
    }

    fn check(kind: u32, old: u32, symbol: u64, place: u64, pc_bias: bool, expected: u32) {
        let bytes = old.to_le_bytes();
        let original = &bytes[..width(kind).unwrap()];
        assert_eq!(
            encode(&relocation(kind), original, symbol, place, pc_bias),
            Some(expected.to_le_bytes())
        );
    }

    #[test]
    fn data_relocations_use_the_resolved_symbol_value() {
        // A mapped Thumb symbol does not set bit zero; an odd absolute one keeps it.
        check(elf::R_ARM_ABS32, 0xffff_fffc, 0x10000, 0, false, 0xfffc);
        check(elf::R_ARM_ABS32, 0, 0x1235, 0, false, 0x1235);
        check(elf::R_ARM_REL32, 0xffff_fffc, 0x1000, 0x1006, false, 0xffff_fff6);
        check(elf::R_ARM_PREL31, 0xffff_fffc, 0x1010, 0x1004, false, 0x8000_0008);
    }

    #[test]
    fn branch_relocations_keep_stock_mode_bits() {
        // BL with A=-4 and a resolved non-Thumb symbol. No extra PC bias.
        check(R_ARM_THM_CALL, 0xfffe_f7ff, 0x1100, 0x1000, true, 0xf87e_f000);
        // BLX clears the low displacement bit rather than aligning P.
        check(R_ARM_THM_CALL, 0xe800_f000, 0x1014, 0x1002, false, 0xe808_f000);
        // B.W with A=0x401002. The two different J bits stay unchanged.
        check(elf::R_ARM_THM_JUMP24, 0xb001_f001, 0x3010, 0x1000, false, 0xb009_f403);
        check(elf::R_ARM_CALL, 0xebff_fffe, 0x1100, 0x1000, false, 0xeb00_003e);
        check(elf::R_ARM_JUMP24, 0xeaff_fffe, 0x1100, 0x1000, true, 0xea00_003c);
        check(elf::R_ARM_CALL, 0xfb00_0000, 0x1010, 0x1000, false, 0xfa00_0004);
    }

    #[test]
    fn mov_relocations_add_a_signed_immediate() {
        // The original MOVW/MOVT both encode -2; preserve the destination r4.
        check(elf::R_ARM_THM_MOVW_ABS_NC, 0x74fe_f64f, 0x1234_0401, 0, false, 0x34ff_f240);
        check(elf::R_ARM_THM_MOVT_ABS, 0x74fe_f6cf, 0x1234_0000, 0, false, 0x2433_f2c1);
        check(elf::R_ARM_THM_MOVW_ABS_NC, 0x0423_f240, 0xabcd, 0, false, 0x34f0_f64a);
    }

    #[test]
    fn thumb16_relocations_use_exactly_two_bytes() {
        assert_eq!(width(elf::R_ARM_THM_PC8), Some(2));
        assert_eq!(width(R_ARM_THM_JUMP11), Some(2));
        assert_eq!(width(elf::R_ARM_NONE), None);
        assert_eq!(encode(&relocation(R_ARM_THM_JUMP11), &[0; 4], 0, 0, false), None);
        // PC8 ignores the original imm8 and neither aligns P nor scales by four.
        check(elf::R_ARM_THM_PC8, 0x4903, 0x100d, 0x1002, false, 0x4905);
        check(elf::R_ARM_THM_PC8, 0x4903, 0x100d, 0x1002, true, 0x4903);
        // JUMP11 adds the sign-extended imm11 * 2, here -4.
        check(R_ARM_THM_JUMP11, 0xe7fe, 0x1000, 0x1006, false, 0xe7fb);
        check(R_ARM_THM_JUMP11, 0xe7fe, 0x1000, 0x1006, true, 0xe7f9);
    }
}
