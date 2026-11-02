//! (kuna) MIPS relocations in a relocatable object.
//!
//! A MIPS `.o` names its call targets and its data through relocations whose
//! fields are pieces of instructions: `jal`'s 26-bit word index (`R_MIPS_26`),
//! a branch's word offset (`PC16`), the `lui`/`addiu` halves of an address
//! (`HI16`/`LO16`), and, in the `-mabicalls` code compilers emit by default, a
//! 16-bit offset from `$gp` to a GOT entry (`CALL16`, `GOT16`, `GOT_DISP`,
//! `GOT_PAGE`/`GOT_OFST`) or to small data (`GPREL16`, `GPREL32`). An o32
//! object (REL) keeps each addend in the field it patches, so a `HI16`, or a
//! `GOT16` against a local symbol, takes the low half of its addend from the
//! next `LO16` against the same symbol. An N32/N64 object (RELA) states its
//! addends, and one entry may chain three operations
//! (`%hi(%neg(%gp_rel(f)))` is `GPREL16`, `SUB`, `HI16`), each result the
//! addend of the next.
//!
//! The object has neither `$gp` nor a GOT; the linker makes both. The layout
//! makes what it would: a GOT after the object's other sections, with one entry
//! per symbol (and addend) a GOT relocation names and one per 64 KiB page a
//! local `GOT16` or a `GOT_PAGE` reaches, then the small-data sections, and
//! `_gp` 0x7ff0 past the GOT's start. `_gp_disp` is `_gp` less the place (a PIC
//! prologue adds `$t9`, the function's entry, to it), and `__gnu_local_gp` is
//! `_gp`. The GOT holds what the relocations put in it and nothing writes it,
//! so the loader reports it foldable, and `lw $t9, %call16(f)($gp); jalr $t9`
//! calls `f`.
//!
//! `R_MIPS_JALR` only says which function a `jalr` calls and is not applied.
//! microMIPS, MIPS16, R6 PC-relative and TLS relocations are unsupported.

use std::collections::{BTreeMap, BTreeSet};

use object::read::{Object, ObjectSection, ObjectSymbol};
use object::{Relocation, RelocationFlags, RelocationTarget, SectionIndex};

use super::reloc_apply::RelocationFailure;

const NONE: u32 = 0;
const R32: u32 = 2;
const R26: u32 = 4;
const HI16: u32 = 5;
const LO16: u32 = 6;
const GPREL16: u32 = 7;
const LITERAL: u32 = 8;
const GOT16: u32 = 9;
const PC16: u32 = 10;
const CALL16: u32 = 11;
const GPREL32: u32 = 12;
const R64: u32 = 18;
const GOT_DISP: u32 = 19;
const GOT_PAGE: u32 = 20;
const GOT_OFST: u32 = 21;
const GOT_HI16: u32 = 22;
const GOT_LO16: u32 = 23;
const SUB: u32 = 24;
const HIGHER: u32 = 28;
const HIGHEST: u32 = 29;
const CALL_HI16: u32 = 30;
const CALL_LO16: u32 = 31;
const JALR: u32 = 37;
const PC32: u32 = 248;

/// `_gp` sits this far past the GOT's start, centring the signed 16-bit
/// `$gp` offsets over it.
pub const GP_BIAS: u64 = super::mips_markers::MIPS_GP_BIAS;

/// The most a 16-bit `$gp` offset reaches.
const GOT_WINDOW: u64 = 0x1_0000;

/// The (up to three) operations of one entry: an o32 type has one, an N64
/// type packs `type`, `type2` and `type3` into its low three bytes.
fn ops(r_type: u32) -> [u32; 3] {
    [r_type & 0xff, (r_type >> 8) & 0xff, (r_type >> 16) & 0xff]
}

/// The operations a chain may continue with: each takes the previous result
/// as its addend.
fn chains(op: u32) -> bool {
    matches!(op, SUB | HI16 | LO16 | R64 | R32 | HIGHER | HIGHEST)
}

fn first(op: u32) -> bool {
    matches!(
        op,
        R32 | R26
            | HI16
            | LO16
            | GPREL16
            | LITERAL
            | GOT16
            | PC16
            | CALL16
            | GPREL32
            | R64
            | GOT_DISP
            | GOT_PAGE
            | GOT_OFST
            | GOT_HI16
            | GOT_LO16
            | SUB
            | HIGHER
            | HIGHEST
            | CALL_HI16
            | CALL_LO16
            | PC32
    )
}

/// The width of the field a MIPS relocation patches and whether its target is
/// code; `Ok(None)` for one that patches nothing.
pub(super) fn classify(r_type: u32) -> Result<Option<(usize, bool)>, RelocationFailure> {
    let [t1, t2, t3] = ops(r_type);
    if (t1 == JALR && t2 == NONE && t3 == NONE) || r_type == NONE {
        return Ok(None);
    }
    if !first(t1) || (t2 != NONE && !chains(t2)) || (t3 != NONE && (t2 == NONE || !chains(t3))) {
        return Err(RelocationFailure::Unsupported);
    }
    let last = [t1, t2, t3].into_iter().rfind(|&t| t != NONE).unwrap_or(t1);
    if last == SUB {
        return Err(RelocationFailure::Unsupported);
    }
    Ok(Some((if last == R64 { 8 } else { 4 }, targets_code(r_type))))
}

/// Does the relocation name a function: a jump, a branch, or a call's GOT
/// entry?
pub(super) fn targets_code(r_type: u32) -> bool {
    matches!(ops(r_type)[0], R26 | PC16 | CALL16 | CALL_HI16 | CALL_LO16)
}

/// Does the relocation's first operation want a GOT entry for its symbol
/// (`global`) or for a 64 KiB page (`!global`)? `None` for one without.
fn got_entry(r_type: u32, local: bool) -> Option<bool> {
    match ops(r_type)[0] {
        CALL16 | GOT_DISP | GOT_HI16 | GOT_LO16 | CALL_HI16 | CALL_LO16 => Some(true),
        GOT16 => Some(!local),
        GOT_PAGE => Some(false),
        _ => None,
    }
}

fn r_type_of(reloc: &Relocation) -> Option<u32> {
    match reloc.flags() {
        RelocationFlags::Elf { r_type } => Some(r_type),
        _ => None,
    }
}

/// Is the relocation's target local to the object: a section, or a local
/// symbol? A `GOT16` against one takes a page entry.
pub(super) fn is_local(file: &object::File, target: RelocationTarget) -> bool {
    match target {
        RelocationTarget::Section(_) => true,
        RelocationTarget::Symbol(idx) => file.symbol_by_index(idx).is_ok_and(|s| s.is_local()),
        _ => false,
    }
}

/// Is `name` a small-data section, which the linker places beside the GOT
/// so that `$gp` reaches it?
pub(super) fn is_small_data(name: &str) -> bool {
    [".sdata", ".sbss", ".lit4", ".lit8", ".srdata", ".scommon"]
        .iter()
        .any(|p| name == *p || name.strip_prefix(p).is_some_and(|rest| rest.starts_with('.')))
}

/// How many GOT entries the relocations of the `laid` sections (index, size)
/// can need: one per distinct symbol and addend a symbol entry names, and,
/// when any page entry is wanted, every 64 KiB page those sections span plus
/// one per page relocation against an undefined symbol.
pub(super) fn entry_bound(file: &object::File, laid: &[(SectionIndex, u64)]) -> usize {
    let mut symbols: BTreeSet<(u8, usize, i64)> = BTreeSet::new();
    let mut extern_pages = 0usize;
    let mut pages = false;
    for &(index, _) in laid {
        let Ok(section) = file.section_by_index(index) else { continue };
        for (_, reloc) in section.relocations() {
            let Some(r_type) = r_type_of(&reloc) else { continue };
            let target = reloc.target();
            match got_entry(r_type, is_local(file, target)) {
                Some(true) => {
                    let key = match target {
                        RelocationTarget::Symbol(i) => (0, i.0),
                        RelocationTarget::Section(i) => (1, i.0),
                        _ => (2, 0),
                    };
                    symbols.insert((key.0, key.1, reloc.addend()));
                }
                Some(false) => {
                    pages = true;
                    if let RelocationTarget::Symbol(i) = target {
                        extern_pages += usize::from(file.symbol_by_index(i).is_ok_and(|s| s.is_undefined()));
                    }
                }
                None => {}
            }
        }
    }
    let spans: usize = if pages {
        laid.iter().map(|&(_, size)| (size / GOT_WINDOW) as usize + 2).sum::<usize>() + extern_pages
    } else {
        0
    };
    symbols.len() + spans
}

/// For each REL `HI16`, or `GOT16` against a local symbol, the low half of its
/// addend: the signed 16-bit field of the next `LO16` against the same target
/// in the section's original bytes `data`. A `HI16` without one gets 0, as
/// LLD's does (the GNU linker reports it as an error).
pub(super) fn lo_halves(relocs: &[(u64, Relocation)], data: &[u8], little_endian: bool) -> Vec<Option<i64>> {
    let mut out = vec![None; relocs.len()];
    for (i, (_, reloc)) in relocs.iter().enumerate() {
        if !reloc.has_implicit_addend() || !matches!(r_type_of(reloc).map(|t| ops(t)[0]), Some(HI16 | GOT16)) {
            continue;
        }
        let lo = relocs[i + 1..]
            .iter()
            .find(|(_, r)| r_type_of(r).map(|t| ops(t)[0]) == Some(LO16) && r.target() == reloc.target())
            .and_then(|(off, _)| word(data, *off as usize, little_endian))
            .map_or(0, |w| i64::from(w as u16 as i16));
        out[i] = Some(lo);
    }
    out
}

fn word(data: &[u8], at: usize, little_endian: bool) -> Option<u32> {
    let b: [u8; 4] = data.get(at..at.checked_add(4)?)?.try_into().ok()?;
    Some(if little_endian { u32::from_le_bytes(b) } else { u32::from_be_bytes(b) })
}

/// The symbols the linker defines for `$gp`: `Some(None)` for `_gp_disp`
/// (`_gp` less the place), `Some(Some(gp))` for an undefined `_gp` or
/// `__gnu_local_gp`, `None` for any other.
pub(super) fn special_symbol(file: &object::File, idx: object::SymbolIndex, gp: u64) -> Option<Option<u64>> {
    let sym = file.symbol_by_index(idx).ok()?;
    match sym.name().ok()? {
        "_gp_disp" => Some(None),
        "_gp" | "__gnu_local_gp" if sym.is_undefined() => Some(Some(gp)),
        _ => None,
    }
}

/// What a relocation's symbol resolves to.
#[derive(Clone, Copy, Debug)]
pub(super) struct Target {
    pub address: u64,
    pub local: bool,
    /// The symbol is `_gp_disp`: `_gp` less the place.
    pub gp_disp: bool,
}

/// The synthetic `$gp` and GOT of one object.
#[derive(Debug)]
pub(super) struct Gp {
    base: u64,
    entry: u64,
    capacity: u64,
    entries: BTreeMap<u64, u64>,
    little_endian: bool,
}

impl Gp {
    /// A GOT of `entries` entries of `entry` bytes at `base`.
    pub(super) fn new(base: u64, entry: u64, entries: usize, little_endian: bool) -> Gp {
        let capacity = (entries as u64).min(GOT_WINDOW / entry);
        Gp { base, entry, capacity, entries: BTreeMap::new(), little_endian }
    }

    pub(super) fn gp(&self) -> u64 {
        self.base.wrapping_add(GP_BIAS)
    }

    /// How many bytes the GOT spans.
    pub(super) fn size(&self) -> u64 {
        self.capacity * self.entry
    }

    /// The GOT's contents: each entry's value, the rest zero.
    pub(super) fn bytes(&self) -> Vec<u8> {
        let mut out = vec![0u8; self.size() as usize];
        for (&value, &slot) in &self.entries {
            let at = (slot - self.base) as usize;
            let n = self.entry as usize;
            let bytes = if self.little_endian { value.to_le_bytes() } else { value.to_be_bytes() };
            let src = if self.little_endian { &bytes[..n] } else { &bytes[8 - n..] };
            out[at..at + n].copy_from_slice(src);
        }
        out
    }

    fn slot(&mut self, value: i128) -> Result<i128, RelocationFailure> {
        let mask = if self.entry == 8 { u64::MAX } else { 0xffff_ffff };
        let value = (value as u64) & mask;
        let next = self.entries.len() as u64;
        if let Some(&slot) = self.entries.get(&value) {
            return Ok(slot as i128 - self.gp() as i128);
        }
        if next >= self.capacity {
            return Err(RelocationFailure::OutOfRange);
        }
        let slot = self.base + next * self.entry;
        self.entries.insert(value, slot);
        Ok(slot as i128 - self.gp() as i128)
    }

    /// Apply one relocation of type `r_type` to its `field` at `place`.
    pub(super) fn apply(
        &mut self,
        r_type: u32,
        reloc: &Relocation,
        field: &mut [u8],
        target: Target,
        place: u64,
        lo_half: Option<i64>,
    ) -> Result<(), RelocationFailure> {
        let le = self.little_endian;
        let [t1, t2, t3] = ops(r_type);
        let implicit = reloc.has_implicit_addend();
        if implicit && (t2 != NONE || t3 != NONE) {
            return Err(RelocationFailure::InvalidEncoding);
        }
        let old = read(field, le);
        let a = if implicit { self.implicit(t1, old, target.local, lo_half)? } else { i128::from(reloc.addend()) };
        let mut v = self.value(t1, i128::from(target.address), a, place, target)?;
        let mut last = t1;
        let ssym = r_type >> 24;
        for t in [t2, t3] {
            if t == NONE {
                break;
            }
            let s = match ssym {
                1 => i128::from(self.gp()),
                3 => i128::from(place),
                _ => 0,
            };
            v = if t == SUB { s - v } else { s + v };
            last = t;
        }
        let new = encode(last, old, v, place, field.len())?;
        write(field, le, new);
        Ok(())
    }

    /// The addend an o32 instruction or data word holds for operation `op`.
    fn implicit(&self, op: u32, old: u64, local: bool, lo_half: Option<i64>) -> Result<i128, RelocationFailure> {
        let w = old as u32;
        let half = i128::from(w as u16 as i16);
        Ok(match op {
            R32 | GPREL32 | PC32 => i128::from(w as i32),
            R64 => i128::from(old as i64),
            R26 if local => i128::from((w & 0x03ff_ffff) << 2),
            R26 => sign_extend(i128::from(w & 0x03ff_ffff) << 2, 28),
            HI16 => (i128::from(w & 0xffff) << 16) + i128::from(lo_half.unwrap_or(0)),
            GOT16 if local => (i128::from(w & 0xffff) << 16) + i128::from(lo_half.unwrap_or(0)),
            LO16 | GPREL16 | LITERAL => half,
            PC16 => half << 2,
            GOT16 | CALL16 | GOT_DISP | GOT_PAGE | GOT_OFST | GOT_HI16 | GOT_LO16 | CALL_HI16 | CALL_LO16 => 0,
            _ => return Err(RelocationFailure::InvalidEncoding),
        })
    }

    /// The value operation `op` computes for symbol `s` and addend `a`.
    fn value(&mut self, op: u32, s: i128, a: i128, place: u64, target: Target) -> Result<i128, RelocationFailure> {
        let gp = i128::from(self.gp());
        let p = i128::from(place);
        if target.gp_disp {
            return match op {
                HI16 => Ok(gp - p + a),
                LO16 => Ok(gp - p + 4 + a),
                _ => Err(RelocationFailure::UnresolvedTarget),
            };
        }
        let page = |x: i128| (x + 0x8000) & !0xffff;
        Ok(match op {
            R32 | R64 | R26 | HI16 | LO16 | HIGHER | HIGHEST => s + a,
            GPREL16 | LITERAL | GPREL32 => s + a - gp,
            PC16 | PC32 => s + a - p,
            SUB => s - a,
            GOT16 if target.local => self.slot(page(s + a))?,
            GOT_PAGE => self.slot(page(s + a))?,
            GOT16 | CALL16 | GOT_DISP | GOT_HI16 | GOT_LO16 | CALL_HI16 | CALL_LO16 => self.slot(s + a)?,
            GOT_OFST => (s + a) - page(s + a),
            _ => return Err(RelocationFailure::Unsupported),
        })
    }
}

/// The field `old` with operation `op`'s value `v` written into it.
fn encode(op: u32, old: u64, v: i128, place: u64, width: usize) -> Result<u64, RelocationFailure> {
    let imm = |bits: i128| (old & !0xffff) | (bits as u64 & 0xffff);
    Ok(match op {
        R64 if width == 8 => v as u64,
        R32 => {
            if !(-(1i128 << 31)..(1i128 << 32)).contains(&v) {
                return Err(RelocationFailure::OutOfRange);
            }
            v as u64 & 0xffff_ffff
        }
        GPREL32 | PC32 => {
            require_signed(v, 32)?;
            v as u64 & 0xffff_ffff
        }
        R26 => {
            if v & 3 != 0 {
                return Err(RelocationFailure::Misaligned);
            }
            if ((v as u64) ^ place.wrapping_add(4)) >> 28 != 0 {
                return Err(RelocationFailure::OutOfRange);
            }
            (old & !0x03ff_ffff) | ((v as u64 >> 2) & 0x03ff_ffff)
        }
        HI16 | GOT_HI16 | CALL_HI16 => imm((v + 0x8000) >> 16),
        LO16 | GOT_LO16 | CALL_LO16 => imm(v),
        HIGHER => imm((v + 0x8000_8000) >> 32),
        HIGHEST => imm((v + 0x8000_8000_8000) >> 48),
        GPREL16 | LITERAL | GOT16 | CALL16 | GOT_DISP | GOT_PAGE | GOT_OFST => {
            require_signed(v, 16)?;
            imm(v)
        }
        PC16 => {
            if v & 3 != 0 {
                return Err(RelocationFailure::Misaligned);
            }
            require_signed(v, 18)?;
            imm(v >> 2)
        }
        _ => return Err(RelocationFailure::Unsupported),
    })
}

fn require_signed(v: i128, bits: u32) -> Result<(), RelocationFailure> {
    if (-(1i128 << (bits - 1))..(1i128 << (bits - 1))).contains(&v) {
        Ok(())
    } else {
        Err(RelocationFailure::OutOfRange)
    }
}

fn sign_extend(v: i128, bits: u32) -> i128 {
    let shift = 128 - bits;
    (v << shift) >> shift
}

fn read(field: &[u8], le: bool) -> u64 {
    let fold = |acc: u64, b: &u8| (acc << 8) | u64::from(*b);
    if le {
        field.iter().rev().fold(0, fold)
    } else {
        field.iter().fold(0, fold)
    }
}

fn write(field: &mut [u8], le: bool, value: u64) {
    let n = field.len();
    for (i, b) in field.iter_mut().enumerate() {
        let shift = if le { i * 8 } else { (n - 1 - i) * 8 };
        *b = (value >> shift) as u8;
    }
}

#[cfg(test)]
#[path = "kuna_mipsreloc_tests.rs"]
mod tests;
