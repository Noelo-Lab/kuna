//! Where a subprogram's frame base is, and which locals besides its direct
//! children with a single `DW_OP_fbreg` expression are placed.
//!
//! `DW_OP_fbreg` is relative to `DW_AT_frame_base`. gcc states the frame base
//! as `DW_OP_call_frame_cfa`; clang at -O0 states the frame-pointer register
//! (`DW_OP_reg6` RBP on x86-64, `DW_OP_reg5` EBP on i386, `DW_OP_reg29` on
//! AArch64, `DW_OP_reg8` s0 on RISC-V), which the standard prologue leaves at a
//! fixed distance from the CFA. Any other register frame base moves with the
//! function body, so its locals are not placed at all.
//!
//! A local nested in `DW_TAG_lexical_block` or `DW_TAG_inlined_subroutine`
//! scopes is collected like a direct child. A variable whose location is a
//! location list takes the stack slot when every frame-relative entry of the
//! list names the same one (a value spilled to one slot for part of its life),
//! and a variable held in one register is reported with that register.

use std::collections::BTreeMap;

use super::{kuna_dwarflocation, Reader};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FrameBase {
    Cfa,
    Register(u16),
    Other,
}

pub fn frame_base(value: &gimli::AttributeValue<Reader<'_>>) -> Option<FrameBase> {
    let bytes = match value {
        gimli::AttributeValue::Exprloc(e) => e.0.slice(),
        gimli::AttributeValue::Block(b) => b.slice(),
        _ => return Some(FrameBase::Other),
    };
    Some(match bytes {
        [0x9c] => FrameBase::Cfa,
        [op @ 0x50..=0x6f] => FrameBase::Register(u16::from(op - 0x50)),
        [0x90, n] if *n < 0x80 => FrameBase::Register(u16::from(*n)),
        _ => FrameBase::Other,
    })
}

/// The stack offset of the frame base, given the target's CFA constant.
pub fn frame_base_offset(base: Option<FrameBase>, cfa: i64) -> Option<i64> {
    match base {
        None | Some(FrameBase::Cfa) => Some(cfa),
        Some(FrameBase::Register(reg)) => match (cfa, reg) {
            (8, 6) => Some(cfa - 16),
            (4, 5) => Some(cfa - 8),
            (0, 29) => Some(cfa - 16),
            (8, 8) => Some(cfa),
            _ => None,
        },
        Some(FrameBase::Other) => None,
    }
}

/// The one frame-base offset every frame-relative entry of a location list
/// names, when the location is a list.
pub fn loclist_frame_offset<'a>(
    dwarf: &gimli::Dwarf<Reader<'a>>,
    unit: &gimli::Unit<Reader<'a>>,
    location: &gimli::AttributeValue<Reader<'a>>,
) -> Option<i64> {
    if matches!(location, gimli::AttributeValue::Exprloc(_) | gimli::AttributeValue::Block(_)) {
        return None;
    }
    let mut list = dwarf.attr_locations(unit, location.clone()).ok()??;
    let mut found = None;
    for _ in 0..4096 {
        let Some(entry) = list.next().ok()? else { return found };
        if let Some(off) = kuna_dwarflocation::frame_offset(entry.data.0.slice()) {
            if found.is_some_and(|f| f != off) {
                return None;
            }
            found = Some(off);
        }
    }
    None
}

/// The variables and parameters of `sub` in document order: its direct
/// children and those of nested lexical blocks and inlined subroutines.
pub(super) fn scope_vars<'a>(sub: &'a super::DieSnap, dies: &'a BTreeMap<usize, super::DieSnap>) -> Vec<&'a super::DieSnap> {
    let mut out = Vec::new();
    let mut pending: Vec<usize> = sub.children.iter().rev().copied().collect();
    while let Some(off) = pending.pop() {
        let Some(child) = dies.get(&off) else { continue };
        if matches!(child.tag, gimli::DW_TAG_lexical_block | gimli::DW_TAG_inlined_subroutine) {
            pending.extend(child.children.iter().rev().copied());
            continue;
        }
        out.push(child);
    }
    out
}

/// Names already given in one function; a repeat gets `_2`, `_3`, ...
#[derive(Default)]
pub(super) struct LocalNames {
    seen: BTreeMap<String, usize>,
}

impl LocalNames {
    pub(super) fn unique(&mut self, name: &str) -> String {
        let n = self.seen.entry(name.to_string()).or_insert(0);
        *n += 1;
        if *n == 1 { name.to_string() } else { format!("{name}_{n}") }
    }
}

/// `DW_OP_reg<n>` / `DW_OP_regx <n>` alone.
pub fn single_register(bytes: &[u8]) -> Option<u16> {
    match bytes {
        [op @ 0x50..=0x6f] => Some(u16::from(op - 0x50)),
        [0x90, rest @ ..] => {
            let mut r = gimli::EndianSlice::new(rest, gimli::LittleEndian);
            let n = gimli::Reader::read_uleb128(&mut r).ok()?;
            gimli::Reader::is_empty(&r).then_some(n as u16)
        }
        _ => None,
    }
}

/// A variable held in one register: the whole-function `DW_OP_reg<n>`, or the
/// longest location-list range that is a single register (its start PC).
pub(super) fn register_location<'a>(
    dwarf: &gimli::Dwarf<Reader<'a>>,
    unit: &gimli::Unit<Reader<'a>>,
    location: &gimli::AttributeValue<Reader<'a>>,
) -> Option<(u16, Option<u64>)> {
    match location {
        gimli::AttributeValue::Exprloc(e) => return single_register(e.0.slice()).map(|r| (r, None)),
        gimli::AttributeValue::Block(b) => return single_register(b.slice()).map(|r| (r, None)),
        _ => {}
    }
    let mut list = dwarf.attr_locations(unit, location.clone()).ok()??;
    let mut best: Option<(u64, u16, u64)> = None;
    for _ in 0..4096 {
        let Some(entry) = list.next().ok()? else { break };
        let Some(reg) = single_register(entry.data.0.slice()) else { continue };
        let len = entry.range.end.saturating_sub(entry.range.begin);
        if best.is_none_or(|(l, ..)| len > l) {
            best = Some((len, reg, entry.range.begin));
        }
    }
    best.map(|(_, reg, lo)| (reg, Some(lo)))
}

/// A composite location whose pieces are consecutive frame slots
/// (`DW_OP_fbreg o; DW_OP_piece n; DW_OP_fbreg o+n; DW_OP_piece m; ...`): the
/// first slot's offset.
pub fn piece_frame_offset(bytes: &[u8]) -> Option<i64> {
    let mut r = gimli::EndianSlice::new(bytes, gimli::LittleEndian);
    let mut start: Option<i64> = None;
    let mut next: i64 = 0;
    let mut pieces = 0;
    while !gimli::Reader::is_empty(&r) {
        if gimli::Reader::read_u8(&mut r).ok()? != gimli::DW_OP_fbreg.0 {
            return None;
        }
        let off = gimli::Reader::read_sleb128(&mut r).ok()?;
        if gimli::Reader::read_u8(&mut r).ok()? != gimli::DW_OP_piece.0 {
            return None;
        }
        let size = gimli::Reader::read_uleb128(&mut r).ok()? as i64;
        match start {
            None => start = Some(off),
            Some(_) if off != next => return None,
            Some(_) => {}
        }
        next = off + size;
        pieces += 1;
    }
    (pieces >= 2).then_some(start?)
}

/// The SLEIGH name of DWARF register `reg` on `arch` (the psABI numbering).
pub fn register_name(arch: object::Architecture, reg: u16) -> Option<String> {
    use object::Architecture as A;
    let r = reg as usize;
    Some(match arch {
        A::X86_64 | A::X86_64_X32 => match r {
            0..=7 => ["RAX", "RDX", "RCX", "RBX", "RSI", "RDI", "RBP", "RSP"][r].to_string(),
            8..=15 => format!("R{r}"),
            17..=32 => format!("XMM{}", r - 17),
            _ => return None,
        },
        A::I386 => ["EAX", "ECX", "EDX", "EBX", "ESP", "EBP", "ESI", "EDI"].get(r)?.to_string(),
        A::Aarch64 => match r {
            0..=30 => format!("x{r}"),
            31 => "sp".into(),
            64..=95 => format!("q{}", r - 64),
            _ => return None,
        },
        A::Arm => match r {
            0..=12 => format!("r{r}"),
            13 => "sp".into(),
            14 => "lr".into(),
            _ => return None,
        },
        A::Riscv32 | A::Riscv64 => {
            const X: [&str; 32] = [
                "zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2", "s0", "s1", "a0", "a1", "a2", "a3", "a4", "a5", "a6",
                "a7", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11", "t3", "t4", "t5", "t6",
            ];
            match r {
                0..=31 => X[r].to_string(),
                32..=63 => format!("f{}", r - 32),
                _ => return None,
            }
        }
        A::Mips | A::Mips64 => {
            const X: [&str; 32] = [
                "zero", "at", "v0", "v1", "a0", "a1", "a2", "a3", "t0", "t1", "t2", "t3", "t4", "t5", "t6", "t7", "s0",
                "s1", "s2", "s3", "s4", "s5", "s6", "s7", "t8", "t9", "k0", "k1", "gp", "sp", "s8", "ra",
            ];
            X.get(r)?.to_string()
        }
        A::PowerPc | A::PowerPc64 => match r {
            0..=31 => format!("r{r}"),
            _ => return None,
        },
        _ => return None,
    })
}

/// Every parameter of `sub` whose location at entry is
/// one register: the compiler's convention for the function, which differs from
/// the default model for an IPA-optimized or `regparm` callee. The slot index
/// rides in [`RegLocalFact::start`](crate::pass::RegLocalFact).
pub(super) fn entry_storage(
    sub: &super::DieSnap,
    func_addr: u64,
    dies: &BTreeMap<usize, super::DieSnap>,
    pieces: &kuna_decomp::fspec::PrototypePieces,
    out: &mut Vec<crate::pass::RegLocalFact>,
) {
    if sub.nocall {
        return;
    }
    let params = sub.children.iter().filter_map(|c| dies.get(c)).filter(|d| d.tag == gimli::DW_TAG_formal_parameter);
    for (index, param) in params.enumerate() {
        let (Some(reg), Some(ty)) = (param.entry_register, pieces.intypes.get(index)) else { continue };
        out.push(crate::pass::RegLocalFact {
            func_addr,
            name: pieces.innames.get(index).cloned().unwrap_or_default(),
            type_: std::rc::Rc::clone(ty),
            dwarf_reg: reg,
            reg: String::new(),
            start: index as u64,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registers_and_contiguous_pieces_decode() {
        assert_eq!(single_register(&[0x55]), Some(5));
        assert_eq!(single_register(&[0x90, 0x11]), Some(17));
        assert_eq!(single_register(&[0x55, 0x93, 0x08]), None);
        assert_eq!(piece_frame_offset(&[0x91, 0x60, 0x93, 0x08, 0x91, 0x68, 0x93, 0x08]), Some(-32));
        assert_eq!(piece_frame_offset(&[0x91, 0x60, 0x93, 0x08, 0x91, 0x70, 0x93, 0x08]), None);
        assert_eq!(register_name(object::Architecture::X86_64, 3).as_deref(), Some("RBX"));
        assert_eq!(register_name(object::Architecture::X86_64, 12).as_deref(), Some("R12"));
    }

    #[test]
    fn frame_pointer_bases_sit_below_the_cfa() {
        assert_eq!(frame_base_offset(Some(FrameBase::Register(6)), 8), Some(-8));
        assert_eq!(frame_base_offset(Some(FrameBase::Register(5)), 4), Some(-4));
        assert_eq!(frame_base_offset(Some(FrameBase::Register(29)), 0), Some(-16));
        assert_eq!(frame_base_offset(Some(FrameBase::Cfa), 8), Some(8));
        assert_eq!(frame_base_offset(Some(FrameBase::Register(7)), 8), None);
    }
}
