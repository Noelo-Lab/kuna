//! Architecture-aware relocation encoding for pre-link objects.
//!
//! `object::Relocation::size()` describes a simple contiguous field. Several
//! ELF instruction relocations consequently report size zero: their bits are
//! scattered through an instruction and must be decoded from `r_type`. This
//! module classifies those entries and applies the ABI formula without touching
//! opcode bits outside the relocation field.
//!
//! Supported instruction families are ARM `CALL`/`JUMP24`, Thumb call/jump,
//! `REL32`, and `PREL31`; AArch64 branch, ADRP page, ADD low-12, and scaled load/
//! store low-12 forms; and PowerPC64 `REL24`, TOC16 variants, and TOC64. Generic
//! absolute, relative, PLT-relative, and image-offset relocations support
//! 8/16/32/64-bit fields. Application preserves REL implicit addends and object
//! endianness, and returns a specific failure classification rather than
//! partially writing a field that is unresolved, malformed, misaligned, outside
//! its range or section, or dependent on a linker veneer or missing TOC.

use object::{elf, Architecture, Relocation, RelocationEncoding, RelocationFlags, RelocationKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RelocationFailure {
    Unsupported,
    UnresolvedTarget,
    MissingToc,
    PastSectionEnd,
    RequiresVeneer,
    Misaligned,
    OutOfRange,
    InvalidEncoding,
}

impl RelocationFailure {
    pub fn label(self) -> &'static str {
        match self {
            Self::Unsupported => "unsupported relocation",
            Self::UnresolvedTarget => "unresolved target",
            Self::MissingToc => "missing .toc section",
            Self::PastSectionEnd => "field extends past section end",
            Self::RequiresVeneer => "interworking target requires a veneer",
            Self::Misaligned => "target displacement is misaligned",
            Self::OutOfRange => "relocation value is out of range",
            Self::InvalidEncoding => "instruction does not match relocation encoding",
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Specific {
    ArmBranch24,
    ArmThumbBranch24,
    ArmPrel31,
    PcRelativeData,
    Aarch64Branch26,
    Aarch64AdrPage { checked: bool },
    Aarch64AddLo12,
    Aarch64LdstLo12 { scale: u8 },
    Ppc64Rel24,
    Ppc64Toc16 { form: PpcHalf },
    Ppc64Toc64,
    Generic { bits: u8 },
}

#[derive(Debug, Clone, Copy)]
enum PpcHalf {
    Plain,
    Lo,
    Hi,
    Ha,
    Ds,
    LoDs,
}

/// A classified relocation. Classification determines the actual byte width,
/// including instruction relocations whose generic size is zero.
#[derive(Debug, Clone, Copy)]
pub struct RelocationSpec {
    specific: Specific,
    width: usize,
    call: bool,
}

#[derive(Debug, Clone, Copy)]
struct RelocationInfo {
    kind: RelocationKind,
    encoding: RelocationEncoding,
    size: u8,
    addend: i64,
    implicit_addend: bool,
    r_type: Option<u32>,
}

impl From<&Relocation> for RelocationInfo {
    fn from(reloc: &Relocation) -> Self {
        Self {
            kind: reloc.kind(),
            encoding: reloc.encoding(),
            size: reloc.size(),
            addend: reloc.addend(),
            implicit_addend: reloc.has_implicit_addend(),
            r_type: match reloc.flags() {
                RelocationFlags::Elf { r_type } => Some(r_type),
                _ => None,
            },
        }
    }
}

impl RelocationSpec {
    pub fn width(self) -> usize {
        self.width
    }

    pub fn is_call(self) -> bool {
        self.call
    }

    /// Refine relocation callness from the instruction field when the ELF
    /// relocation type is shared by branch and call encodings.
    pub fn refine_call_from_field(mut self, field: &[u8], little_endian: bool) -> Self {
        if matches!(self.specific, Specific::ArmBranch24) && !self.call && field.len() == 4 {
            let instruction = read_u32(field, little_endian);
            if instruction & 0x0f00_0000 == 0x0b00_0000 {
                self.call = true;
            }
        }
        if matches!(self.specific, Specific::Ppc64Rel24) && field.len() == 4 {
            self.call = read_u32(field, little_endian) & 1 != 0;
        }
        self
    }

    pub fn requires_symbol(self) -> bool {
        !matches!(self.specific, Specific::Ppc64Toc64)
    }

    /// Whether the patched field is a branch or call instruction, so its target
    /// is code. A tail call to an undefined symbol is spelled as a plain jump
    /// relocation, which is not a call but still needs an extern function slot.
    pub fn targets_code(self) -> bool {
        self.is_call()
            || matches!(
                self.specific,
                Specific::ArmBranch24
                    | Specific::ArmThumbBranch24
                    | Specific::Aarch64Branch26
                    | Specific::Ppc64Rel24
            )
    }
}

/// `Ok(None)` means a no-op relocation (`R_*_NONE`).
pub fn classify(
    arch: Architecture,
    reloc: &Relocation,
) -> Result<Option<RelocationSpec>, RelocationFailure> {
    classify_info(arch, RelocationInfo::from(reloc))
}

fn classify_info(
    arch: Architecture,
    reloc: RelocationInfo,
) -> Result<Option<RelocationSpec>, RelocationFailure> {
    let r_type = reloc.r_type;

    let specific = match (arch, r_type) {
        (Architecture::Arm, Some(elf::R_ARM_NONE))
        | (Architecture::Arm, Some(elf::R_ARM_V4BX))
        | (Architecture::Aarch64, Some(elf::R_AARCH64_NONE))
        | (Architecture::PowerPc64, Some(elf::R_PPC64_NONE)) => return Ok(None),

        (Architecture::Arm, Some(elf::R_ARM_CALL)) => Some((Specific::ArmBranch24, true)),
        (Architecture::Arm, Some(elf::R_ARM_JUMP24)) => Some((Specific::ArmBranch24, false)),
        // The ABI's modern name for relocation 10 is R_ARM_THM_CALL; object
        // retains the historical R_ARM_THM_PC22 spelling for the same value.
        (Architecture::Arm, Some(elf::R_ARM_THM_PC22)) => Some((Specific::ArmThumbBranch24, true)),
        (Architecture::Arm, Some(elf::R_ARM_THM_JUMP24)) => {
            Some((Specific::ArmThumbBranch24, false))
        }
        (Architecture::Arm, Some(elf::R_ARM_REL32)) => Some((Specific::PcRelativeData, false)),
        (Architecture::Arm, Some(elf::R_ARM_PREL31)) => Some((Specific::ArmPrel31, false)),

        (Architecture::Aarch64, Some(elf::R_AARCH64_CALL26)) => {
            Some((Specific::Aarch64Branch26, true))
        }
        (Architecture::Aarch64, Some(elf::R_AARCH64_JUMP26)) => {
            Some((Specific::Aarch64Branch26, false))
        }
        (Architecture::Aarch64, Some(elf::R_AARCH64_ADR_PREL_PG_HI21)) => {
            Some((Specific::Aarch64AdrPage { checked: true }, false))
        }
        (Architecture::Aarch64, Some(elf::R_AARCH64_ADR_PREL_PG_HI21_NC)) => {
            Some((Specific::Aarch64AdrPage { checked: false }, false))
        }
        (Architecture::Aarch64, Some(elf::R_AARCH64_ADD_ABS_LO12_NC)) => {
            Some((Specific::Aarch64AddLo12, false))
        }
        (Architecture::Aarch64, Some(elf::R_AARCH64_LDST8_ABS_LO12_NC)) => {
            Some((Specific::Aarch64LdstLo12 { scale: 0 }, false))
        }
        (Architecture::Aarch64, Some(elf::R_AARCH64_LDST16_ABS_LO12_NC)) => {
            Some((Specific::Aarch64LdstLo12 { scale: 1 }, false))
        }
        (Architecture::Aarch64, Some(elf::R_AARCH64_LDST32_ABS_LO12_NC)) => {
            Some((Specific::Aarch64LdstLo12 { scale: 2 }, false))
        }
        (Architecture::Aarch64, Some(elf::R_AARCH64_LDST64_ABS_LO12_NC)) => {
            Some((Specific::Aarch64LdstLo12 { scale: 3 }, false))
        }
        (Architecture::Aarch64, Some(elf::R_AARCH64_LDST128_ABS_LO12_NC)) => {
            Some((Specific::Aarch64LdstLo12 { scale: 4 }, false))
        }

        (Architecture::PowerPc64, Some(elf::R_PPC64_REL24)) => Some((Specific::Ppc64Rel24, true)),
        (Architecture::PowerPc64, Some(elf::R_PPC64_TOC16)) => Some((
            Specific::Ppc64Toc16 {
                form: PpcHalf::Plain,
            },
            false,
        )),
        (Architecture::PowerPc64, Some(elf::R_PPC64_TOC16_LO)) => {
            Some((Specific::Ppc64Toc16 { form: PpcHalf::Lo }, false))
        }
        (Architecture::PowerPc64, Some(elf::R_PPC64_TOC16_HI)) => {
            Some((Specific::Ppc64Toc16 { form: PpcHalf::Hi }, false))
        }
        (Architecture::PowerPc64, Some(elf::R_PPC64_TOC16_HA)) => {
            Some((Specific::Ppc64Toc16 { form: PpcHalf::Ha }, false))
        }
        (Architecture::PowerPc64, Some(elf::R_PPC64_TOC16_DS)) => {
            Some((Specific::Ppc64Toc16 { form: PpcHalf::Ds }, false))
        }
        (Architecture::PowerPc64, Some(elf::R_PPC64_TOC16_LO_DS)) => Some((
            Specific::Ppc64Toc16 {
                form: PpcHalf::LoDs,
            },
            false,
        )),
        (Architecture::PowerPc64, Some(elf::R_PPC64_TOC)) => Some((Specific::Ppc64Toc64, false)),
        _ => None,
    };

    if let Some((specific, call)) = specific {
        let width = match specific {
            Specific::Ppc64Toc16 { .. } => 2,
            Specific::Ppc64Toc64 => 8,
            _ => 4,
        };
        return Ok(Some(RelocationSpec {
            specific,
            width,
            call,
        }));
    }

    if reloc.kind == RelocationKind::None {
        return Ok(None);
    }
    let bits = reloc.size;
    if !matches!(bits, 8 | 16 | 32 | 64) {
        return Err(RelocationFailure::Unsupported);
    }
    if !matches!(
        reloc.kind,
        RelocationKind::Absolute
            | RelocationKind::Relative
            | RelocationKind::PltRelative
            | RelocationKind::ImageOffset
    ) {
        return Err(RelocationFailure::Unsupported);
    }
    Ok(Some(RelocationSpec {
        specific: Specific::Generic { bits },
        width: usize::from(bits / 8),
        call: matches!(reloc.kind, RelocationKind::PltRelative),
    }))
}

/// Apply a classified relocation to an exactly `spec.width()`-byte field.
pub fn apply(
    spec: RelocationSpec,
    reloc: &Relocation,
    field: &mut [u8],
    little_endian: bool,
    symbol: u64,
    place: u64,
    target_thumb: Option<bool>,
    toc: Option<u64>,
) -> Result<(), RelocationFailure> {
    apply_info(
        spec,
        RelocationInfo::from(reloc),
        field,
        little_endian,
        symbol,
        place,
        target_thumb,
        toc,
    )
}

fn apply_info(
    spec: RelocationSpec,
    reloc: RelocationInfo,
    field: &mut [u8],
    little_endian: bool,
    symbol: u64,
    place: u64,
    target_thumb: Option<bool>,
    toc: Option<u64>,
) -> Result<(), RelocationFailure> {
    debug_assert_eq!(field.len(), spec.width);
    match spec.specific {
        Specific::ArmBranch24 => arm_branch24(
            field,
            little_endian,
            reloc,
            symbol,
            place,
            spec.call,
            target_thumb,
        ),
        Specific::ArmThumbBranch24 => arm_thumb_branch24(
            field,
            little_endian,
            reloc,
            symbol,
            place,
            spec.call,
            target_thumb,
        ),
        Specific::ArmPrel31 => arm_prel31(field, little_endian, reloc, symbol, place),
        Specific::PcRelativeData => pc_relative_data(field, little_endian, reloc, symbol, place),
        Specific::Aarch64Branch26 => aarch64_branch26(field, little_endian, reloc, symbol, place),
        Specific::Aarch64AdrPage { checked } => {
            aarch64_adr_page(field, little_endian, reloc, symbol, place, checked)
        }
        Specific::Aarch64AddLo12 => aarch64_lo12(field, little_endian, reloc, symbol, 0, false),
        Specific::Aarch64LdstLo12 { scale } => {
            aarch64_lo12(field, little_endian, reloc, symbol, scale, true)
        }
        Specific::Ppc64Rel24 => ppc64_rel24(field, little_endian, reloc, symbol, place),
        Specific::Ppc64Toc16 { form } => {
            ppc64_toc16(field, little_endian, reloc, symbol, toc, form)
        }
        Specific::Ppc64Toc64 => ppc64_toc64(field, little_endian, reloc, toc),
        Specific::Generic { bits } => generic(field, little_endian, reloc, symbol, place, bits),
    }
}

fn explicit_plus_implicit(reloc: RelocationInfo, implicit: i128) -> i128 {
    reloc.addend as i128 + if reloc.implicit_addend { implicit } else { 0 }
}

fn arm_branch24(
    field: &mut [u8],
    le: bool,
    reloc: RelocationInfo,
    symbol: u64,
    place: u64,
    call: bool,
    target_thumb: Option<bool>,
) -> Result<(), RelocationFailure> {
    let mut insn = read_u32(field, le);
    if insn & 0x0e00_0000 != 0x0a00_0000 {
        return Err(RelocationFailure::InvalidEncoding);
    }
    let is_blx = insn & 0xfe00_0000 == 0xfa00_0000;
    let is_bl = insn >> 28 != 0xf && insn & 0x0f00_0000 == 0x0b00_0000;
    if call && !is_blx && !is_bl {
        return Err(RelocationFailure::InvalidEncoding);
    }
    if !call && target_thumb == Some(true) {
        return Err(RelocationFailure::RequiresVeneer);
    }
    // BLX <imm> carries the halfword bit of its displacement in bit 24.
    let blx_halfword = if is_blx {
        (((insn >> 24) & 1) as i128) << 1
    } else {
        0
    };
    let implicit = sign_extend((((insn & 0x00ff_ffff) as i128) << 2) | blx_halfword, 26);
    let addend = explicit_plus_implicit(reloc, implicit);
    let target = symbol & !1;
    let value = target as i128 + addend - place as i128;

    let conditional_bl = is_bl && insn >> 28 != 0xe;
    if conditional_bl && target_thumb == Some(true) {
        return Err(RelocationFailure::RequiresVeneer);
    }
    let write_blx = call && target_thumb.unwrap_or(is_blx);
    if write_blx {
        require_aligned(value, 2)?;
        require_signed(value, 26)?;
        insn = 0xfa00_0000 | (((value as u32) & 2) << 23) | (((value >> 2) as u32) & 0x00ff_ffff);
    } else {
        require_aligned(value, 4)?;
        require_signed(value, 26)?;
        if call && target_thumb == Some(false) && is_blx {
            insn = 0xeb00_0000 | (insn & 0x00ff_ffff);
        }
        insn = (insn & !0x00ff_ffff) | (((value >> 2) as u32) & 0x00ff_ffff);
    }
    write_u32(field, le, insn);
    Ok(())
}

fn arm_thumb_branch24(
    field: &mut [u8],
    le: bool,
    reloc: RelocationInfo,
    symbol: u64,
    place: u64,
    call: bool,
    target_thumb: Option<bool>,
) -> Result<(), RelocationFailure> {
    let mut hi = read_u16(&field[..2], le);
    let mut lo = read_u16(&field[2..], le);
    // BL/BLX set both of the low halfword's top bits; the T4 `B.W` this
    // relocation pair otherwise targets sets bit 15 and bit 12 only. A T3
    // conditional `B.W` matches neither and uses a different immediate layout.
    let form_matches = if call {
        lo & 0xc000 == 0xc000
    } else {
        lo & 0xd000 == 0x9000
    };
    if hi & 0xf800 != 0xf000 || !form_matches {
        return Err(RelocationFailure::InvalidEncoding);
    }
    let is_blx = lo & 0x1000 == 0;
    if !call && target_thumb == Some(false) {
        return Err(RelocationFailure::RequiresVeneer);
    }
    let s = ((hi >> 10) & 1) as i128;
    let j1 = ((lo >> 13) & 1) as i128;
    let j2 = ((lo >> 11) & 1) as i128;
    let i1 = (!(j1 ^ s)) & 1;
    let i2 = (!(j2 ^ s)) & 1;
    let encoded = (s << 24)
        | (i1 << 23)
        | (i2 << 22)
        | (((hi & 0x03ff) as i128) << 12)
        | (((lo & 0x07ff) as i128) << 1);
    let implicit = sign_extend(encoded, 25);
    let addend = explicit_plus_implicit(reloc, implicit);
    let mut value = (symbol & !1) as i128 + addend - place as i128;
    require_aligned(value, 2)?;
    let write_blx = call && !target_thumb.unwrap_or(!is_blx);
    if write_blx {
        // Thumb BLX branches from Align(PC, 4), even when its own address is
        // only halfword-aligned. Adjust the halfword-aligned displacement to
        // that architectural base (a correction of 0 or 2) before encoding it.
        value += (-value).rem_euclid(4);
        lo &= !0x1000;
    } else if call {
        lo |= 0x1000;
    }
    require_signed(value, 25)?;
    let raw = value as u32;
    let new_s = (raw >> 24) & 1;
    let new_i1 = (raw >> 23) & 1;
    let new_i2 = (raw >> 22) & 1;
    let new_j1 = (!(new_i1 ^ new_s)) & 1;
    let new_j2 = (!(new_i2 ^ new_s)) & 1;
    hi = (hi & !0x07ff) | ((new_s as u16) << 10) | (((raw >> 12) as u16) & 0x03ff);
    lo = (lo & !0x2fff)
        | ((new_j1 as u16) << 13)
        | ((new_j2 as u16) << 11)
        | (((raw >> 1) as u16) & 0x07ff);
    write_u16(&mut field[..2], le, hi);
    write_u16(&mut field[2..], le, lo);
    Ok(())
}

fn arm_prel31(
    field: &mut [u8],
    le: bool,
    reloc: RelocationInfo,
    symbol: u64,
    place: u64,
) -> Result<(), RelocationFailure> {
    let old = read_u32(field, le);
    let implicit = sign_extend((old & 0x7fff_ffff) as i128, 31);
    let value = symbol as i128 + explicit_plus_implicit(reloc, implicit) - place as i128;
    require_signed(value, 31)?;
    write_u32(
        field,
        le,
        (old & 0x8000_0000) | ((value as u32) & 0x7fff_ffff),
    );
    Ok(())
}

/// `R_ARM_REL32`. AAELF32 classifies it as a no-overflow-check relocation, so
/// the result is truncated to the field width rather than range-checked.
fn pc_relative_data(
    field: &mut [u8],
    le: bool,
    reloc: RelocationInfo,
    symbol: u64,
    place: u64,
) -> Result<(), RelocationFailure> {
    let value =
        symbol as i128 + explicit_plus_implicit(reloc, read_signed(field, le)) - place as i128;
    write_integer(field, le, value as u64);
    Ok(())
}

fn aarch64_branch26(
    field: &mut [u8],
    le: bool,
    reloc: RelocationInfo,
    symbol: u64,
    place: u64,
) -> Result<(), RelocationFailure> {
    let mut insn = read_u32(field, le);
    if insn & 0x7c00_0000 != 0x1400_0000 {
        return Err(RelocationFailure::InvalidEncoding);
    }
    let implicit = sign_extend(((insn & 0x03ff_ffff) as i128) << 2, 28);
    let value = symbol as i128 + explicit_plus_implicit(reloc, implicit) - place as i128;
    require_aligned(value, 4)?;
    require_signed(value, 28)?;
    insn = (insn & !0x03ff_ffff) | (((value >> 2) as u32) & 0x03ff_ffff);
    write_u32(field, le, insn);
    Ok(())
}

fn aarch64_adr_page(
    field: &mut [u8],
    le: bool,
    reloc: RelocationInfo,
    symbol: u64,
    place: u64,
    checked: bool,
) -> Result<(), RelocationFailure> {
    let mut insn = read_u32(field, le);
    if insn & 0x9f00_0000 != 0x9000_0000 {
        return Err(RelocationFailure::InvalidEncoding);
    }
    let encoded = ((((insn >> 5) & 0x7ffff) as i128) << 2) | (((insn >> 29) & 3) as i128);
    let implicit_pages = sign_extend(encoded, 21);
    let implicit = implicit_pages << 12;
    let target = symbol as i128 + explicit_plus_implicit(reloc, implicit);
    let pages = (target >> 12) - ((place as i128) >> 12);
    if checked {
        require_signed(pages, 21)?;
    }
    let raw = pages as u32;
    insn = (insn & !0x60ff_ffe0) | ((raw & 3) << 29) | (((raw >> 2) & 0x7ffff) << 5);
    write_u32(field, le, insn);
    Ok(())
}

/// `*_ABS_LO12_NC`: the ABI writes bits 11:`scale` of `S + A`, so the low 12
/// bits are taken first and the scaled field is what remains.
fn aarch64_lo12(
    field: &mut [u8],
    le: bool,
    reloc: RelocationInfo,
    symbol: u64,
    scale: u8,
    ldst: bool,
) -> Result<(), RelocationFailure> {
    let mut insn = read_u32(field, le);
    if !lo12_encoding_matches(insn, scale, ldst) {
        return Err(RelocationFailure::InvalidEncoding);
    }
    let old_imm = ((insn >> 10) & 0xfff) as i128;
    let implicit = old_imm << scale;
    let value = symbol as i128 + explicit_plus_implicit(reloc, implicit);
    let mask = (1i128 << scale) - 1;
    if value & mask != 0 {
        return Err(RelocationFailure::Misaligned);
    }
    let imm = ((value & 0xfff) >> scale) as u32;
    insn = (insn & !(0xfff << 10)) | (imm << 10);
    write_u32(field, le, insn);
    Ok(())
}

/// ADD (immediate) for the ADD form; load/store register (unsigned immediate)
/// with a size field matching `scale` for the LDST forms. A SIMD 128-bit access
/// spells its scale as `opc<1>:size`.
fn lo12_encoding_matches(insn: u32, scale: u8, ldst: bool) -> bool {
    if !ldst {
        return insn & 0x7f80_0000 == 0x1100_0000;
    }
    if insn & 0x3b00_0000 != 0x3900_0000 {
        return false;
    }
    let size = insn >> 30;
    let encoded = if insn & 0x0400_0000 != 0 {
        (((insn >> 23) & 1) << 2) | size
    } else {
        size
    };
    encoded == u32::from(scale)
}

fn ppc64_rel24(
    field: &mut [u8],
    le: bool,
    reloc: RelocationInfo,
    symbol: u64,
    place: u64,
) -> Result<(), RelocationFailure> {
    let mut insn = read_u32(field, le);
    if insn & 0xfc00_0000 != 0x4800_0000 {
        return Err(RelocationFailure::InvalidEncoding);
    }
    let implicit = sign_extend((insn & 0x03ff_fffc) as i128, 26);
    let value = symbol as i128 + explicit_plus_implicit(reloc, implicit) - place as i128;
    require_aligned(value, 4)?;
    require_signed(value, 26)?;
    insn = (insn & !0x03ff_fffc) | ((value as u32) & 0x03ff_fffc);
    write_u32(field, le, insn);
    Ok(())
}

fn ppc64_toc16(
    field: &mut [u8],
    le: bool,
    reloc: RelocationInfo,
    symbol: u64,
    toc: Option<u64>,
    form: PpcHalf,
) -> Result<(), RelocationFailure> {
    let toc = toc.ok_or(RelocationFailure::MissingToc)?;
    // PPC64 uses RELA for these split fields. Reconstructing a full implicit
    // addend from one half is impossible, so reject a malformed REL spelling.
    if reloc.implicit_addend {
        return Err(RelocationFailure::InvalidEncoding);
    }
    let value = symbol as i128 + reloc.addend as i128 - toc as i128;
    let old = read_u16(field, le);
    let encoded = match form {
        PpcHalf::Plain => {
            require_signed(value, 16)?;
            value as u16
        }
        PpcHalf::Lo => value as u16,
        PpcHalf::Hi => (value >> 16) as u16,
        PpcHalf::Ha => ((value + 0x8000) >> 16) as u16,
        PpcHalf::Ds => {
            require_signed(value, 16)?;
            require_aligned(value, 4)?;
            (old & 3) | ((value as u16) & !3)
        }
        PpcHalf::LoDs => {
            require_aligned(value, 4)?;
            (old & 3) | ((value as u16) & !3)
        }
    };
    write_u16(field, le, encoded);
    Ok(())
}

fn ppc64_toc64(
    field: &mut [u8],
    le: bool,
    reloc: RelocationInfo,
    toc: Option<u64>,
) -> Result<(), RelocationFailure> {
    let toc = toc.ok_or(RelocationFailure::MissingToc)?;
    let value = toc as i128 + explicit_plus_implicit(reloc, read_signed(field, le));
    write_integer(field, le, value as u64);
    Ok(())
}

fn generic(
    field: &mut [u8],
    le: bool,
    reloc: RelocationInfo,
    symbol: u64,
    place: u64,
    bits: u8,
) -> Result<(), RelocationFailure> {
    let addend = explicit_plus_implicit(reloc, read_signed(field, le));
    let value = match reloc.kind {
        RelocationKind::Absolute => symbol as i128 + addend,
        RelocationKind::Relative | RelocationKind::PltRelative => {
            symbol as i128 + addend - place as i128
        }
        RelocationKind::ImageOffset => {
            symbol as i128 + addend - super::reloc_object::RELOC_BASE as i128
        }
        _ => return Err(RelocationFailure::Unsupported),
    };
    if matches!(
        reloc.kind,
        RelocationKind::Relative | RelocationKind::PltRelative
    ) || matches!(reloc.encoding, RelocationEncoding::X86Signed)
    {
        require_signed(value, bits)?;
    } else if value < 0 || (bits < 64 && value >= (1i128 << bits)) {
        return Err(RelocationFailure::OutOfRange);
    }
    write_integer(field, le, value as u64);
    Ok(())
}

fn require_aligned(value: i128, align: i128) -> Result<(), RelocationFailure> {
    if value.rem_euclid(align) == 0 {
        Ok(())
    } else {
        Err(RelocationFailure::Misaligned)
    }
}

fn require_signed(value: i128, bits: u8) -> Result<(), RelocationFailure> {
    let min = -(1i128 << (bits - 1));
    let max = (1i128 << (bits - 1)) - 1;
    if (min..=max).contains(&value) {
        Ok(())
    } else {
        Err(RelocationFailure::OutOfRange)
    }
}

fn sign_extend(value: i128, bits: u8) -> i128 {
    let shift = 128 - bits;
    (value << shift) >> shift
}

fn read_signed(field: &[u8], le: bool) -> i128 {
    let raw = read_integer(field, le);
    sign_extend(raw as i128, (field.len() * 8) as u8)
}

fn read_integer(field: &[u8], le: bool) -> u64 {
    let mut value = 0u64;
    if le {
        for byte in field.iter().rev() {
            value = (value << 8) | u64::from(*byte);
        }
    } else {
        for byte in field {
            value = (value << 8) | u64::from(*byte);
        }
    }
    value
}

fn write_integer(field: &mut [u8], le: bool, value: u64) {
    let len = field.len();
    for (i, byte) in field.iter_mut().enumerate() {
        let shift = if le { i * 8 } else { (len - i - 1) * 8 };
        *byte = (value >> shift) as u8;
    }
}

fn read_u16(field: &[u8], le: bool) -> u16 {
    let bytes: [u8; 2] = field.try_into().expect("u16 relocation field");
    if le {
        u16::from_le_bytes(bytes)
    } else {
        u16::from_be_bytes(bytes)
    }
}

fn write_u16(field: &mut [u8], le: bool, value: u16) {
    let bytes = if le {
        value.to_le_bytes()
    } else {
        value.to_be_bytes()
    };
    field.copy_from_slice(&bytes);
}

fn read_u32(field: &[u8], le: bool) -> u32 {
    let bytes: [u8; 4] = field.try_into().expect("u32 relocation field");
    if le {
        u32::from_le_bytes(bytes)
    } else {
        u32::from_be_bytes(bytes)
    }
}

fn write_u32(field: &mut [u8], le: bool, value: u32) {
    let bytes = if le {
        value.to_le_bytes()
    } else {
        value.to_be_bytes()
    };
    field.copy_from_slice(&bytes);
}

#[cfg(test)]
mod tests {
    use super::*;
    use object::{RelocationEncoding, RelocationKind};

    fn reloc(r_type: u32, implicit: bool) -> RelocationInfo {
        RelocationInfo {
            kind: RelocationKind::Unknown,
            encoding: RelocationEncoding::Unknown,
            size: 0,
            addend: 0,
            implicit_addend: implicit,
            r_type: Some(r_type),
        }
    }

    fn generic_reloc(
        kind: RelocationKind,
        encoding: RelocationEncoding,
        size: u8,
        addend: i64,
        implicit_addend: bool,
    ) -> RelocationInfo {
        RelocationInfo {
            kind,
            encoding,
            size,
            addend,
            implicit_addend,
            r_type: None,
        }
    }

    #[test]
    fn no_op_and_generic_classification_cover_the_supported_widths() {
        for (arch, r_type) in [
            (Architecture::Arm, elf::R_ARM_NONE),
            (Architecture::Arm, elf::R_ARM_V4BX),
            (Architecture::Aarch64, elf::R_AARCH64_NONE),
            (Architecture::PowerPc64, elf::R_PPC64_NONE),
        ] {
            assert!(classify_info(arch, reloc(r_type, false)).unwrap().is_none());
        }
        assert!(classify_info(
            Architecture::X86_64,
            generic_reloc(
                RelocationKind::None,
                RelocationEncoding::Generic,
                0,
                0,
                false,
            ),
        )
        .unwrap()
        .is_none());

        for bits in [8, 16, 32, 64] {
            let info = generic_reloc(
                RelocationKind::Absolute,
                RelocationEncoding::Generic,
                bits,
                0,
                false,
            );
            let spec = classify_info(Architecture::X86_64, info).unwrap().unwrap();
            assert_eq!(spec.width(), usize::from(bits / 8));
        }

        let unsupported = generic_reloc(
            RelocationKind::Absolute,
            RelocationEncoding::Generic,
            24,
            0,
            false,
        );
        assert!(matches!(
            classify_info(Architecture::X86_64, unsupported),
            Err(RelocationFailure::Unsupported)
        ));
        let unsupported = generic_reloc(
            RelocationKind::SectionOffset,
            RelocationEncoding::Generic,
            32,
            0,
            false,
        );
        assert!(matches!(
            classify_info(Architecture::X86_64, unsupported),
            Err(RelocationFailure::Unsupported)
        ));
    }

    #[test]
    fn arm_call_preserves_opcode_and_encodes_implicit_addend() {
        let r = reloc(elf::R_ARM_CALL, true);
        let spec = classify_info(Architecture::Arm, r).unwrap().unwrap();
        let mut bytes = 0xebff_fffeu32.to_le_bytes(); // BL with implicit A=-8
        apply_info(spec, r, &mut bytes, true, 0x400100, 0x400000, None, None).unwrap();
        assert_eq!(u32::from_le_bytes(bytes), 0xeb00_003e);
    }

    #[test]
    fn arm_jump_out_of_range_does_not_mutate_opcode() {
        let r = reloc(elf::R_ARM_JUMP24, true);
        let spec = classify_info(Architecture::Arm, r).unwrap().unwrap();
        let mut bytes = 0xeaff_fffeu32.to_le_bytes();
        let before = bytes;
        assert_eq!(
            apply_info(spec, r, &mut bytes, true, 0x9000_0000, 0x400000, None, None,),
            Err(RelocationFailure::OutOfRange)
        );
        assert_eq!(bytes, before);
    }

    #[test]
    fn arm_thumb_call_encodes_split_immediate_without_touching_opcode_bits() {
        let r = reloc(elf::R_ARM_THM_PC22, false);
        let spec = classify_info(Architecture::Arm, r).unwrap().unwrap();
        let mut bytes = [0x00, 0xf0, 0x00, 0xf8];
        apply_info(spec, r, &mut bytes, true, 0x400100, 0x400000, None, None).unwrap();
        assert_eq!(bytes, [0x00, 0xf0, 0x80, 0xf8]);
    }

    #[test]
    fn arm_data_relocations_preserve_addends_and_prel31_flag_bit() {
        let rel32 = reloc(elf::R_ARM_REL32, true);
        let rel32_spec = classify_info(Architecture::Arm, rel32).unwrap().unwrap();
        let mut relative = (-4i32).to_le_bytes();
        apply_info(
            rel32_spec,
            rel32,
            &mut relative,
            true,
            0x400100,
            0x400000,
            None,
            None,
        )
        .unwrap();
        assert_eq!(i32::from_le_bytes(relative), 0xfc);

        let prel31 = reloc(elf::R_ARM_PREL31, true);
        let prel31_spec = classify_info(Architecture::Arm, prel31).unwrap().unwrap();
        let mut flagged = 0x8000_0004u32.to_le_bytes();
        apply_info(
            prel31_spec,
            prel31,
            &mut flagged,
            true,
            0x400100,
            0x400000,
            None,
            None,
        )
        .unwrap();
        assert_eq!(u32::from_le_bytes(flagged), 0x8000_0104);

        let mut out_of_range = 0x8000_0000u32.to_le_bytes();
        let before = out_of_range;
        assert_eq!(
            apply_info(
                prel31_spec,
                prel31,
                &mut out_of_range,
                true,
                0x8000_0000,
                0,
                None,
                None,
            ),
            Err(RelocationFailure::OutOfRange)
        );
        assert_eq!(out_of_range, before);
    }

    #[test]
    fn arm_calls_convert_between_bl_and_blx_for_typed_targets() {
        let r = reloc(elf::R_ARM_CALL, false);
        let spec = classify_info(Architecture::Arm, r).unwrap().unwrap();

        let mut to_thumb = 0xeb00_0000u32.to_le_bytes();
        apply_info(
            spec,
            r,
            &mut to_thumb,
            true,
            0x400103,
            0x400000,
            Some(true),
            None,
        )
        .unwrap();
        assert_eq!(u32::from_le_bytes(to_thumb), 0xfb00_0040);

        let mut to_arm = 0xfa00_0000u32.to_le_bytes();
        apply_info(
            spec,
            r,
            &mut to_arm,
            true,
            0x400100,
            0x400000,
            Some(false),
            None,
        )
        .unwrap();
        assert_eq!(u32::from_le_bytes(to_arm), 0xeb00_0040);
    }

    #[test]
    fn thumb_call_converts_bl_to_blx_and_aligns_its_pc_base() {
        let r = reloc(elf::R_ARM_THM_PC22, false);
        let spec = classify_info(Architecture::Arm, r).unwrap().unwrap();
        let mut bytes = [0x00, 0xf0, 0x00, 0xf8];
        apply_info(
            spec,
            r,
            &mut bytes,
            true,
            0x400100,
            0x400002,
            Some(false),
            None,
        )
        .unwrap();
        assert_eq!(bytes, [0x00, 0xf0, 0x80, 0xe8]);
    }

    #[test]
    fn cross_state_jumps_require_a_veneer_without_mutating_code() {
        let arm = reloc(elf::R_ARM_JUMP24, false);
        let arm_spec = classify_info(Architecture::Arm, arm).unwrap().unwrap();
        let mut arm_bytes = 0xea00_0000u32.to_le_bytes();
        let arm_before = arm_bytes;
        assert_eq!(
            apply_info(
                arm_spec,
                arm,
                &mut arm_bytes,
                true,
                0x400101,
                0x400000,
                Some(true),
                None,
            ),
            Err(RelocationFailure::RequiresVeneer)
        );
        assert_eq!(arm_bytes, arm_before);

        let thumb = reloc(elf::R_ARM_THM_JUMP24, false);
        let thumb_spec = classify_info(Architecture::Arm, thumb).unwrap().unwrap();
        let mut thumb_bytes = [0x00, 0xf0, 0x00, 0xb8];
        let thumb_before = thumb_bytes;
        assert_eq!(
            apply_info(
                thumb_spec,
                thumb,
                &mut thumb_bytes,
                true,
                0x400100,
                0x400000,
                Some(false),
                None,
            ),
            Err(RelocationFailure::RequiresVeneer)
        );
        assert_eq!(thumb_bytes, thumb_before);
    }

    #[test]
    fn aarch64_call_and_page_pair_keep_opcode_register_bits() {
        let call = reloc(elf::R_AARCH64_CALL26, false);
        let call_spec = classify_info(Architecture::Aarch64, call).unwrap().unwrap();
        let mut branch = 0x9400_0000u32.to_le_bytes();
        apply_info(
            call_spec,
            call,
            &mut branch,
            true,
            0x401000,
            0x400000,
            None,
            None,
        )
        .unwrap();
        assert_eq!(u32::from_le_bytes(branch), 0x9400_0400);

        let adrp = reloc(elf::R_AARCH64_ADR_PREL_PG_HI21, false);
        let adrp_spec = classify_info(Architecture::Aarch64, adrp).unwrap().unwrap();
        let mut page = 0x9000_0013u32.to_le_bytes();
        apply_info(
            adrp_spec, adrp, &mut page, true, 0x412345, 0x400004, None, None,
        )
        .unwrap();
        assert_eq!(u32::from_le_bytes(page) & 0x1f, 0x13);
        assert_eq!(u32::from_le_bytes(page) & 0x9f00_0000, 0x9000_0000);
    }

    #[test]
    fn aarch64_call_checks_alignment_and_range() {
        let r = reloc(elf::R_AARCH64_CALL26, false);
        let spec = classify_info(Architecture::Aarch64, r).unwrap().unwrap();
        let mut bytes = 0x9400_0000u32.to_le_bytes();
        assert_eq!(
            apply_info(spec, r, &mut bytes, true, 0x400002, 0x400000, None, None,),
            Err(RelocationFailure::Misaligned)
        );
        assert_eq!(
            apply_info(spec, r, &mut bytes, true, 0x4000_0000, 0x400000, None, None,),
            Err(RelocationFailure::OutOfRange)
        );
    }

    #[test]
    fn aarch64_jump_unchecked_page_and_every_low12_scale_apply() {
        let jump = reloc(elf::R_AARCH64_JUMP26, false);
        let jump_spec = classify_info(Architecture::Aarch64, jump).unwrap().unwrap();
        assert!(!jump_spec.is_call());
        let mut branch = 0x1400_0000u32.to_le_bytes();
        apply_info(
            jump_spec,
            jump,
            &mut branch,
            true,
            0x400100,
            0x400000,
            None,
            None,
        )
        .unwrap();
        assert_eq!(u32::from_le_bytes(branch), 0x1400_0040);

        let checked = reloc(elf::R_AARCH64_ADR_PREL_PG_HI21, false);
        let checked_spec = classify_info(Architecture::Aarch64, checked)
            .unwrap()
            .unwrap();
        let unchecked = reloc(elf::R_AARCH64_ADR_PREL_PG_HI21_NC, false);
        let unchecked_spec = classify_info(Architecture::Aarch64, unchecked)
            .unwrap()
            .unwrap();
        let mut checked_page = 0x9000_0007u32.to_le_bytes();
        assert_eq!(
            apply_info(
                checked_spec,
                checked,
                &mut checked_page,
                true,
                0x1_0000_0000,
                0,
                None,
                None,
            ),
            Err(RelocationFailure::OutOfRange)
        );
        let mut unchecked_page = 0x9000_0007u32.to_le_bytes();
        apply_info(
            unchecked_spec,
            unchecked,
            &mut unchecked_page,
            true,
            0x1_0000_0000,
            0,
            None,
            None,
        )
        .unwrap();
        assert_eq!(u32::from_le_bytes(unchecked_page), 0x9080_0007);

        for (r_type, scale, opcode) in [
            (elf::R_AARCH64_ADD_ABS_LO12_NC, 0, 0x9100_0083u32),
            (elf::R_AARCH64_LDST8_ABS_LO12_NC, 0, 0x3940_0083),
            (elf::R_AARCH64_LDST16_ABS_LO12_NC, 1, 0x7940_0083),
            (elf::R_AARCH64_LDST32_ABS_LO12_NC, 2, 0xb940_0083),
            (elf::R_AARCH64_LDST64_ABS_LO12_NC, 3, 0xf940_0083),
            (elf::R_AARCH64_LDST128_ABS_LO12_NC, 4, 0x3dc0_0083),
        ] {
            let info = reloc(r_type, false);
            let spec = classify_info(Architecture::Aarch64, info).unwrap().unwrap();
            // Symbols whose page offset is identical but whose page differs:
            // the encoded immediate must depend only on the low 12 bits.
            for symbol in [0x7f0u64, 0x1040, 0x2ff8, 0x3010, 0x40_4040, 0x40_8040] {
                if symbol & ((1u64 << scale) - 1) != 0 {
                    continue;
                }
                let mut field = opcode.to_le_bytes();
                apply_info(spec, info, &mut field, true, symbol, 0, None, None).unwrap();
                let relocated = u32::from_le_bytes(field);
                assert_eq!(relocated & !(0xfff << 10), opcode & !(0xfff << 10));
                assert_eq!(
                    (relocated >> 10) & 0xfff,
                    ((symbol & 0xfff) >> scale) as u32,
                    "r_type={r_type} symbol={symbol:#x}"
                );
            }

            if scale != 0 {
                let mut misaligned = opcode.to_le_bytes();
                let before = misaligned;
                assert_eq!(
                    apply_info(
                        spec,
                        info,
                        &mut misaligned,
                        true,
                        (1u64 << scale) - 1,
                        0,
                        None,
                        None,
                    ),
                    Err(RelocationFailure::Misaligned),
                    "r_type={r_type}"
                );
                assert_eq!(misaligned, before);
            }
        }
    }

    #[test]
    fn aarch64_low12_rejects_instructions_outside_its_encoding() {
        for (r_type, opcode) in [
            (elf::R_AARCH64_ADD_ABS_LO12_NC, 0xb940_0083u32), // LDR, not ADD
            (elf::R_AARCH64_LDST32_ABS_LO12_NC, 0x9100_0083), // ADD, not LDR
            (elf::R_AARCH64_LDST32_ABS_LO12_NC, 0xf940_0083), // 64-bit LDR at scale 2
            (elf::R_AARCH64_LDST64_ABS_LO12_NC, 0x3dc0_0083), // 128-bit LDR at scale 3
        ] {
            let info = reloc(r_type, false);
            let spec = classify_info(Architecture::Aarch64, info).unwrap().unwrap();
            let mut field = opcode.to_le_bytes();
            let before = field;
            assert_eq!(
                apply_info(spec, info, &mut field, true, 0x40, 0, None, None),
                Err(RelocationFailure::InvalidEncoding),
                "r_type={r_type} opcode={opcode:#x}"
            );
            assert_eq!(field, before);
        }
    }

    #[test]
    fn arm_blx_with_the_halfword_bit_set_is_not_a_conditional_bl() {
        let r = reloc(elf::R_ARM_CALL, true);
        let spec = classify_info(Architecture::Arm, r).unwrap().unwrap();
        let mut bytes = 0xfbff_fffeu32.to_le_bytes(); // BLX with H=1, addend -6
        apply_info(
            spec,
            r,
            &mut bytes,
            true,
            0x400101,
            0x400000,
            Some(true),
            None,
        )
        .unwrap();
        assert_eq!(u32::from_le_bytes(bytes), 0xfb00_003e);
    }

    #[test]
    fn thumb_branch_forms_are_validated_before_the_immediate_is_rewritten() {
        let call = reloc(elf::R_ARM_THM_PC22, false);
        let call_spec = classify_info(Architecture::Arm, call).unwrap().unwrap();
        let mut conditional = [0x00, 0xf0, 0x00, 0x80]; // B<c>.W (T3)
        let before = conditional;
        assert_eq!(
            apply_info(
                call_spec,
                call,
                &mut conditional,
                true,
                0x400100,
                0x400000,
                None,
                None,
            ),
            Err(RelocationFailure::InvalidEncoding)
        );
        assert_eq!(conditional, before);

        let jump = reloc(elf::R_ARM_THM_JUMP24, false);
        let jump_spec = classify_info(Architecture::Arm, jump).unwrap().unwrap();
        let mut as_call = [0x00, 0xf0, 0x00, 0xf8]; // BL (T1)
        let before = as_call;
        assert_eq!(
            apply_info(
                jump_spec,
                jump,
                &mut as_call,
                true,
                0x400100,
                0x400000,
                None,
                None,
            ),
            Err(RelocationFailure::InvalidEncoding)
        );
        assert_eq!(as_call, before);
    }

    #[test]
    fn thumb_blx_reports_a_misaligned_displacement_instead_of_rounding_it() {
        let r = reloc(elf::R_ARM_THM_PC22, false);
        let spec = classify_info(Architecture::Arm, r).unwrap().unwrap();
        let mut bytes = [0x00, 0xf0, 0x00, 0xf8];
        let before = bytes;
        assert_eq!(
            apply_info(
                spec,
                r,
                &mut bytes,
                true,
                0x400100,
                0x400001,
                Some(false),
                None,
            ),
            Err(RelocationFailure::Misaligned)
        );
        assert_eq!(bytes, before);
    }

    #[test]
    fn arm_rel32_truncates_instead_of_reporting_an_overflow() {
        let r = reloc(elf::R_ARM_REL32, true);
        let spec = classify_info(Architecture::Arm, r).unwrap().unwrap();
        let mut field = 0u32.to_le_bytes();
        apply_info(spec, r, &mut field, true, 0x9000_0000, 0x100, None, None).unwrap();
        assert_eq!(u32::from_le_bytes(field), 0x8fff_ff00);
    }

    #[test]
    fn ppc64_rel24_is_big_endian_and_preserves_link_bit() {
        let r = reloc(elf::R_PPC64_REL24, false);
        let spec = classify_info(Architecture::PowerPc64, r).unwrap().unwrap();
        let mut bytes = 0x4800_0001u32.to_be_bytes();
        apply_info(
            spec,
            r,
            &mut bytes,
            false,
            0x400100,
            0x400000,
            None,
            Some(0x408000),
        )
        .unwrap();
        assert_eq!(u32::from_be_bytes(bytes), 0x4800_0101);
    }

    #[test]
    fn ppc64_rel24_callness_comes_from_the_link_bit() {
        let r = reloc(elf::R_PPC64_REL24, false);
        let spec = classify_info(Architecture::PowerPc64, r).unwrap().unwrap();
        assert!(!spec
            .refine_call_from_field(&0x4800_0000u32.to_be_bytes(), false)
            .is_call());
        assert!(spec
            .refine_call_from_field(&0x4800_0001u32.to_be_bytes(), false)
            .is_call());
    }

    #[test]
    fn ppc64_toc_split_fields_are_big_endian() {
        let r = reloc(elf::R_PPC64_TOC16_HA, false);
        let spec = classify_info(Architecture::PowerPc64, r).unwrap().unwrap();
        let mut bytes = [0xaa, 0xaa];
        apply_info(
            spec,
            r,
            &mut bytes,
            false,
            0x40_0000,
            0,
            None,
            Some(0x40_8000),
        )
        .unwrap();
        assert_eq!(bytes, [0x00, 0x00]);

        let r = reloc(elf::R_PPC64_TOC16_LO_DS, false);
        let spec = classify_info(Architecture::PowerPc64, r).unwrap().unwrap();
        let mut bytes = [0x00, 0x02];
        apply_info(
            spec,
            r,
            &mut bytes,
            false,
            0x40_0000,
            0,
            None,
            Some(0x40_8000),
        )
        .unwrap();
        assert_eq!(bytes, [0x80, 0x02]);
    }

    #[test]
    fn ppc64_rel24_and_checked_toc_ds_reject_out_of_range_values() {
        let r = reloc(elf::R_PPC64_REL24, false);
        let spec = classify_info(Architecture::PowerPc64, r).unwrap().unwrap();
        let mut branch = 0x4800_0001u32.to_be_bytes();
        assert_eq!(
            apply_info(
                spec,
                r,
                &mut branch,
                false,
                0x8000_0000,
                0x400000,
                None,
                None,
            ),
            Err(RelocationFailure::OutOfRange)
        );

        let r = reloc(elf::R_PPC64_TOC16_DS, false);
        let spec = classify_info(Architecture::PowerPc64, r).unwrap().unwrap();
        let mut field = [0x00, 0x02];
        assert_eq!(
            apply_info(
                spec,
                r,
                &mut field,
                false,
                0x41_0000,
                0,
                None,
                Some(0x40_8000),
            ),
            Err(RelocationFailure::OutOfRange)
        );
    }

    #[test]
    fn ppc64_toc_family_covers_every_form_and_missing_toc() {
        let toc = 0x40_0000;
        for (r_type, symbol, initial, expected) in [
            (elf::R_PPC64_TOC16, toc + 0x1234, 0u16, 0x1234u16),
            (elf::R_PPC64_TOC16_LO, toc + 0x1234_5678, 0, 0x5678),
            (elf::R_PPC64_TOC16_HI, toc + 0x1234_5678, 0, 0x1234),
            (elf::R_PPC64_TOC16_HA, toc + 0x1234_8000, 0, 0x1235),
            (elf::R_PPC64_TOC16_DS, toc - 0x100, 2, 0xff02),
            (elf::R_PPC64_TOC16_LO_DS, toc + 0x1234_5678, 2, 0x567a),
        ] {
            let info = reloc(r_type, false);
            let spec = classify_info(Architecture::PowerPc64, info)
                .unwrap()
                .unwrap();
            let mut field = initial.to_be_bytes();
            apply_info(spec, info, &mut field, false, symbol, 0, None, Some(toc)).unwrap();
            assert_eq!(u16::from_be_bytes(field), expected, "r_type={r_type}");
        }

        let toc64 = reloc(elf::R_PPC64_TOC, false);
        let toc64_spec = classify_info(Architecture::PowerPc64, toc64)
            .unwrap()
            .unwrap();
        assert!(!toc64_spec.requires_symbol());
        let mut field = [0; 8];
        apply_info(
            toc64_spec,
            toc64,
            &mut field,
            false,
            0,
            0,
            None,
            Some(0x40_8000),
        )
        .unwrap();
        assert_eq!(u64::from_be_bytes(field), 0x40_8000);

        let toc16 = reloc(elf::R_PPC64_TOC16, false);
        let toc16_spec = classify_info(Architecture::PowerPc64, toc16)
            .unwrap()
            .unwrap();
        let mut short = [0; 2];
        assert_eq!(
            apply_info(toc16_spec, toc16, &mut short, false, toc, 0, None, None),
            Err(RelocationFailure::MissingToc)
        );
        let mut wide = [0; 8];
        assert_eq!(
            apply_info(toc64_spec, toc64, &mut wide, false, 0, 0, None, None),
            Err(RelocationFailure::MissingToc)
        );

        let malformed = reloc(elf::R_PPC64_TOC16_LO, true);
        let malformed_spec = classify_info(Architecture::PowerPc64, malformed)
            .unwrap()
            .unwrap();
        assert_eq!(
            apply_info(
                malformed_spec,
                malformed,
                &mut short,
                false,
                toc,
                0,
                None,
                Some(toc),
            ),
            Err(RelocationFailure::InvalidEncoding)
        );
    }

    #[test]
    fn generic_relocations_cover_kinds_endianness_and_bounds() {
        for (bits, little_endian, symbol, expected) in [
            (8, true, 0x12, vec![0x12]),
            (16, false, 0x1234, vec![0x12, 0x34]),
            (32, true, 0x1234_5678, vec![0x78, 0x56, 0x34, 0x12]),
            (
                64,
                false,
                0x0123_4567_89ab_cdef,
                vec![0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef],
            ),
        ] {
            let info = generic_reloc(
                RelocationKind::Absolute,
                RelocationEncoding::Generic,
                bits,
                0,
                false,
            );
            let spec = classify_info(Architecture::X86_64, info).unwrap().unwrap();
            let mut field = vec![0; spec.width()];
            apply_info(spec, info, &mut field, little_endian, symbol, 0, None, None).unwrap();
            assert_eq!(field, expected, "bits={bits}");
        }

        for kind in [RelocationKind::Relative, RelocationKind::PltRelative] {
            let info = generic_reloc(kind, RelocationEncoding::Generic, 16, 0, false);
            let spec = classify_info(Architecture::X86_64, info).unwrap().unwrap();
            assert_eq!(spec.is_call(), kind == RelocationKind::PltRelative);
            let mut field = [0; 2];
            apply_info(spec, info, &mut field, true, 0x1010, 0x1000, None, None).unwrap();
            assert_eq!(i16::from_le_bytes(field), 0x10);
        }

        let image = generic_reloc(
            RelocationKind::ImageOffset,
            RelocationEncoding::Generic,
            32,
            0,
            false,
        );
        let image_spec = classify_info(Architecture::X86_64, image).unwrap().unwrap();
        let mut image_field = [0; 4];
        apply_info(
            image_spec,
            image,
            &mut image_field,
            true,
            super::super::reloc_object::RELOC_BASE + 0x34,
            0,
            None,
            None,
        )
        .unwrap();
        assert_eq!(u32::from_le_bytes(image_field), 0x34);

        let signed = generic_reloc(
            RelocationKind::Absolute,
            RelocationEncoding::X86Signed,
            8,
            -1,
            false,
        );
        let signed_spec = classify_info(Architecture::X86_64, signed)
            .unwrap()
            .unwrap();
        let mut signed_field = [0];
        apply_info(
            signed_spec,
            signed,
            &mut signed_field,
            true,
            0,
            0,
            None,
            None,
        )
        .unwrap();
        assert_eq!(signed_field, [0xff]);

        let with_addends = generic_reloc(
            RelocationKind::Absolute,
            RelocationEncoding::Generic,
            16,
            2,
            true,
        );
        let with_addends_spec = classify_info(Architecture::X86_64, with_addends)
            .unwrap()
            .unwrap();
        let mut with_addends_field = 3u16.to_le_bytes();
        apply_info(
            with_addends_spec,
            with_addends,
            &mut with_addends_field,
            true,
            5,
            0,
            None,
            None,
        )
        .unwrap();
        assert_eq!(u16::from_le_bytes(with_addends_field), 10);

        let unsigned = generic_reloc(
            RelocationKind::Absolute,
            RelocationEncoding::Generic,
            8,
            0,
            false,
        );
        let unsigned_spec = classify_info(Architecture::X86_64, unsigned)
            .unwrap()
            .unwrap();
        let mut overflow = [0x5a];
        assert_eq!(
            apply_info(
                unsigned_spec,
                unsigned,
                &mut overflow,
                true,
                0x100,
                0,
                None,
                None
            ),
            Err(RelocationFailure::OutOfRange)
        );
        assert_eq!(overflow, [0x5a]);
    }

    #[test]
    fn invalid_instruction_encodings_do_not_mutate_fields() {
        for (arch, r_type, little_endian, mut field) in [
            (Architecture::Arm, elf::R_ARM_CALL, true, vec![0; 4]),
            (Architecture::Arm, elf::R_ARM_THM_PC22, true, vec![0; 4]),
            (
                Architecture::Aarch64,
                elf::R_AARCH64_CALL26,
                true,
                vec![0; 4],
            ),
            (
                Architecture::Aarch64,
                elf::R_AARCH64_ADR_PREL_PG_HI21,
                true,
                vec![0; 4],
            ),
            (
                Architecture::PowerPc64,
                elf::R_PPC64_REL24,
                false,
                vec![0; 4],
            ),
        ] {
            let info = reloc(r_type, false);
            let spec = classify_info(arch, info).unwrap().unwrap();
            let before = field.clone();
            assert_eq!(
                apply_info(spec, info, &mut field, little_endian, 0, 0, None, None),
                Err(RelocationFailure::InvalidEncoding),
                "arch={arch:?} r_type={r_type}"
            );
            assert_eq!(field, before);
        }
    }
}
