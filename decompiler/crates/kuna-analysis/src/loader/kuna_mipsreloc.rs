//! (kuna) MIPS relocations in a relocatable object.
//!
//! A MIPS `.o` names its call targets and its data through relocations whose
//! fields are pieces of instructions: `jal`'s 26-bit word index (`R_MIPS_26`),
//! a branch's word offset (`PC16`), the `lui`/`addiu` halves of an address
//! (`HI16`/`LO16`), and, in the `-mabicalls` code compilers emit by default, a
//! 16-bit offset from `$gp` to a GOT entry (`CALL16`, `GOT16`, `GOT_DISP`,
//! `GOT_PAGE`/`GOT_OFST`) or to small data (`GPREL16`, `GPREL32`). An o32
//! or N32 REL object keeps each addend in the field it patches, so a `HI16`, or
//! a `GOT16` against a local symbol, takes the low half of its addend from the
//! next `LO16` against the same symbol; an N32/N64 RELA object states its
//! addends. Either way, consecutive records at one offset are one expression,
//! as GNU ld chains them: each result is the next operation's addend, a REL
//! record's field is read once for the first operation, and only the last
//! writes. `%hi(%neg(%gp_rel(f)))` is `GPREL16`, `SUB`, `HI16`, which GNU as
//! writes as one N32 record per operation at the same offset; an N64 record
//! packs up to three, its second operation taking the `r_ssym` special symbol
//! (`0`, `$gp`, the input `$gp` or the place), and the chain continues into
//! the next record at that offset until an `R_MIPS_NONE` ends it. Each
//! operation computes what GNU ld does, including a 32-bit result truncated
//! before a 64-bit one; only the last must fit its field. An expression with
//! an operation that cannot be applied writes nothing, and a packed record
//! whose operations resume after a `NONE` is malformed: it writes nothing and
//! neither continues nor starts a chain.
//!
//! The object has neither `$gp` nor a GOT; the linker makes both. The layout
//! makes what it would: a GOT after the object's other sections, with one entry
//! per symbol (and addend) a GOT relocation names and one per 64 KiB page a
//! local `GOT16` or a `GOT_PAGE` reaches, then the small-data sections, and
//! `_gp` 0x7ff0 past the GOT's start; an object's own definition of `_gp` is
//! overridden, as the default linker script's assignment overrides it.
//! `_gp_disp` is `_gp` less the place (a PIC prologue adds `$t9`, the
//! function's entry, to it), and `__gnu_local_gp` is `_gp`. A GP-relative value against a local symbol, and every `GPREL32`,
//! adds back the input `$gp` a partial link recorded in `.reginfo` or
//! `.MIPS.options`. The GOT holds what the relocations put in it and nothing
//! writes it, so the loader reports it foldable, and
//! `lw $t9, %call16(f)($gp); jalr $t9` calls `f`.
//!
//! `R_MIPS_JALR` only says which function a `jalr` calls and is not applied.
//! microMIPS, MIPS16, R6 PC-relative and TLS relocations are unsupported.

use std::collections::{BTreeMap, BTreeSet};

use object::read::{Object, ObjectSection, ObjectSymbol};
use object::{Relocation, RelocationFlags, RelocationTarget, SectionIndex};

use super::reloc_apply::RelocationFailure;

const NONE: u32 = 0;
const R16: u32 = 1;
const R32: u32 = 2;
const REL32: u32 = 3;
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

pub(super) fn r_type_of(reloc: &Relocation) -> Option<u32> {
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
/// (`_gp` less the place), `Some(Some(gp))` for `_gp`, which the linker
/// script's assignment defines over any definition the object has, and for an
/// undefined `__gnu_local_gp`; `None` for any other.
pub(super) fn special_symbol(file: &object::File, idx: object::SymbolIndex, gp: u64) -> Option<Option<u64>> {
    let sym = file.symbol_by_index(idx).ok()?;
    match sym.name().ok()? {
        "_gp_disp" => Some(None),
        "_gp" => Some(Some(gp)),
        "__gnu_local_gp" if sym.is_undefined() => Some(Some(gp)),
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

    /// Where the GOT starts.
    pub(super) fn base(&self) -> u64 {
        self.base
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

    /// Apply a section's relocations to its bytes `data` laid at `base`: each
    /// expression in record order, reading an implicit addend from the bytes
    /// earlier expressions wrote. `lo_halves` is [`lo_halves`]'s, per record,
    /// and `input_gp` the object's own recorded `$gp` ([`input_gp`]). A failed
    /// expression writes nothing and reports each of its records.
    pub(super) fn apply_section(
        &mut self,
        arch: object::Architecture,
        data: &mut [u8],
        base: u64,
        records: &[(u64, Relocation)],
        lo_halves: &[Option<i64>],
        input_gp: Result<u64, RelocationFailure>,
        resolve: &mut dyn FnMut(RelocationTarget) -> Option<Target>,
    ) -> Applied {
        let mut applied = Applied::default();
        let input = Inputs { arch, base, input_gp };
        let relocs: Vec<&Relocation> = records.iter().map(|(_, r)| r).collect();
        for expression in expressions(arch, records) {
            let first = expression[0].record;
            let result = (|| {
                let offset = usize::try_from(records[first].0).map_err(|_| RelocationFailure::PastSectionEnd)?;
                let width = field_width(expression.last().map_or(NONE, |op| op.op))?;
                let end = offset.checked_add(width).ok_or(RelocationFailure::PastSectionEnd)?;
                let field = data.get(offset..end).ok_or(RelocationFailure::PastSectionEnd)?;
                let lo = lo_halves.get(first).copied().flatten();
                let (value, callee) = self.evaluate(&input, field, offset, &relocs, &expression, lo, resolve)?;
                Ok((offset..end, value, callee))
            })();
            match result {
                Ok((range, value, callee)) => {
                    write(&mut data[range], self.little_endian, value);
                    applied.callees.extend(callee);
                }
                Err(reason) => {
                    let mut seen = BTreeSet::new();
                    for op in &expression {
                        if seen.insert(op.record) {
                            applied.failures.push((op.record, reason));
                        }
                    }
                }
            }
        }
        applied
    }

    /// The bits one expression writes into `field`, and the function it calls
    /// when its result is a call or branch to exactly one symbol.
    #[allow(clippy::too_many_arguments)]
    fn evaluate(
        &mut self,
        input: &Inputs,
        field: &[u8],
        offset: usize,
        relocs: &[&Relocation],
        expression: &[Op],
        lo_half: Option<i64>,
        resolve: &mut dyn FnMut(RelocationTarget) -> Option<Target>,
    ) -> Result<(u64, Option<RelocationTarget>), RelocationFailure> {
        let le = self.little_endian;
        let old = read(field, le);
        let place = input.base.wrapping_add(offset as u64);
        let first = relocs[expression[0].record];
        let first_target = resolve(first.target()).ok_or(RelocationFailure::UnresolvedTarget)?;
        let mut value = if first.has_implicit_addend() {
            self.implicit(expression[0].op, old, first_target.local, lo_half)? as u64
        } else {
            first.addend() as u64
        };
        let mut origin = Origin::Constant;
        let mut callee = None;
        for (index, op) in expression.iter().enumerate() {
            let reloc = relocs[op.record];
            let (mut target, named) = match (index, op.special) {
                (0, _) => (first_target, Some(first.target())),
                (_, None) => (resolve(reloc.target()).ok_or(RelocationFailure::UnresolvedTarget)?, Some(reloc.target())),
                (_, Some(ssym)) => (self.special(ssym, place, input)?, None),
            };
            if input.arch != object::Architecture::Mips64 {
                target.address = target.address as u32 as i32 as i64 as u64;
            }
            let last = index + 1 == expression.len();
            let next = self.operate(op.op, target, value, place, input, last)?;
            let symbol = match named {
                Some(t @ RelocationTarget::Symbol(_)) if !target.gp_disp => Origin::Symbol(t, 1),
                _ => Origin::Constant,
            };
            let sum = origin.add(symbol);
            if last && targets_code(op.op) {
                callee = sum.callee();
            }
            origin = match op.op {
                R16 | R64 => sum,
                R32 => sum.narrow(value.wrapping_add(target.address), next, input.arch),
                REL32 => origin.narrow(value, next, input.arch),
                SUB => symbol.add(origin.negate()),
                _ => sum.transformed(),
            };
            value = next;
        }
        let mask = field_mask(expression.last().map_or(NONE, |op| op.op));
        Ok(((old & !mask) | (value & mask), callee))
    }

    /// The value of an N64 `r_ssym` special symbol at `place`.
    fn special(&self, ssym: u8, place: u64, input: &Inputs) -> Result<Target, RelocationFailure> {
        let address = match ssym {
            RSS_UNDEF => 0,
            RSS_GP => self.gp(),
            RSS_GP0 => input.input_gp?,
            RSS_LOC => place,
            _ => return Err(RelocationFailure::Unsupported),
        };
        Ok(Target { address, local: true, gp_disp: false })
    }

    /// The addend an o32 instruction or data word holds for operation `op`.
    fn implicit(&self, op: u32, old: u64, local: bool, lo_half: Option<i64>) -> Result<i128, RelocationFailure> {
        let w = old as u32;
        let half = i128::from(w as u16 as i16);
        Ok(match op {
            R32 | GPREL32 | PC32 | REL32 => i128::from(w as i32),
            R64 | SUB => i128::from(old as i64),
            R26 if local => i128::from((w & 0x03ff_ffff) << 2),
            R26 => sign_extend(i128::from(w & 0x03ff_ffff) << 2, 28),
            HI16 => (i128::from(w & 0xffff) << 16) + i128::from(lo_half.unwrap_or(0)),
            GOT16 if local => (i128::from(w & 0xffff) << 16) + i128::from(lo_half.unwrap_or(0)),
            R16 | LO16 | GPREL16 | LITERAL => half,
            PC16 => half << 2,
            GOT_PAGE | GOT_OFST => i128::from(w & 0xffff),
            GOT16 | CALL16 | GOT_DISP | GOT_HI16 | GOT_LO16 | CALL_HI16 | CALL_LO16 => 0,
            _ => return Err(RelocationFailure::InvalidEncoding),
        })
    }

    /// The result of operation `op` on symbol `t` and addend `a`, as the field
    /// bits it would write; a later operation of the expression takes it as its
    /// addend. Range checks apply only to the last operation; PC32 always
    /// keeps the low 32 bits of the displacement.
    fn operate(&mut self, op: u32, t: Target, a: u64, place: u64, input: &Inputs, last: bool) -> Result<u64, RelocationFailure> {
        let gp = self.gp();
        let s = t.address;
        let sum = s.wrapping_add(a);
        let fits = |v: u64, bits: u32| sign_extend(i128::from(v as i64), bits) == i128::from(v as i64);
        let checked = |v: u64, bits: u32| if last && !fits(v, bits) { Err(RelocationFailure::OutOfRange) } else { Ok(v) };
        if t.gp_disp && !matches!(op, HI16 | LO16) {
            return Err(RelocationFailure::UnresolvedTarget);
        }
        let page = |x: u64| x.wrapping_add(0x8000) & !0xffff;
        let got = |this: &mut Self, value: u64| this.slot(i128::from(value as i64)).map(|v| v as u64);
        Ok(match op {
            R16 => checked(sum, 16)?,
            R32 => sum & 0xffff_ffff,
            R64 => sum,
            REL32 => a & 0xffff_ffff,
            PC32 => sum.wrapping_sub(place) & 0xffff_ffff,
            SUB => s.wrapping_sub(a),
            HI16 | LO16 => {
                let v = if t.gp_disp {
                    gp.wrapping_add(a).wrapping_sub(place).wrapping_add(if op == LO16 { 4 } else { 0 })
                } else {
                    sum
                };
                if op == HI16 { v.wrapping_add(0x8000) >> 16 & 0xffff } else { v & 0xffff }
            }
            HIGHER => sum.wrapping_add(0x8000_8000) >> 32 & 0xffff,
            HIGHEST => sum.wrapping_add(0x8000_8000_8000) >> 48 & 0xffff,
            R26 => {
                if sum & 3 != 0 {
                    return Err(RelocationFailure::Misaligned);
                }
                if last && (sum ^ place.wrapping_add(4)) >> 28 != 0 {
                    return Err(RelocationFailure::OutOfRange);
                }
                sum >> 2 & 0x03ff_ffff
            }
            PC16 => {
                let v = sum.wrapping_sub(place);
                if v & 3 != 0 {
                    return Err(RelocationFailure::Misaligned);
                }
                checked(v, 18)? >> 2 & 0xffff
            }
            GPREL16 | LITERAL | GPREL32 => {
                let gp0 = if t.local || op == GPREL32 { input.input_gp? } else { 0 };
                let v = sum.wrapping_add(gp0).wrapping_sub(gp);
                if op == GPREL32 { v & if last { 0xffff_ffff } else { u64::MAX } } else { checked(v, 16)? }
            }
            GOT16 if t.local => checked(got(self, page(sum))?, 16)?,
            GOT_PAGE => checked(got(self, page(sum))?, 16)?,
            GOT16 | CALL16 | GOT_DISP => checked(got(self, sum)?, 16)?,
            GOT_HI16 | CALL_HI16 => got(self, sum)?.wrapping_add(0x8000) >> 16 & 0xffff,
            GOT_LO16 | CALL_LO16 => got(self, sum)? & 0xffff,
            GOT_OFST => checked(sum.wrapping_sub(page(sum)), 16)?,
            _ => return Err(RelocationFailure::Unsupported),
        })
    }
}

/// What a section's expressions read besides their records.
struct Inputs {
    arch: object::Architecture,
    base: u64,
    input_gp: Result<u64, RelocationFailure>,
}

/// One operation of an expression: its type, the record it came from, and
/// for a later operation of an N64 record the `r_ssym` special symbol it uses
/// in place of the record's.
#[derive(Clone, Copy, Debug)]
struct Op {
    record: usize,
    op: u32,
    special: Option<u8>,
}

/// What [`Gp::apply_section`] did: the records of each expression it could
/// not apply, and the symbols its calls and branches reach.
#[derive(Debug, Default)]
pub(super) struct Applied {
    pub failures: Vec<(usize, RelocationFailure)>,
    pub callees: Vec<RelocationTarget>,
}

const RSS_UNDEF: u8 = 0;
const RSS_GP: u8 = 1;
const RSS_GP0: u8 = 2;
const RSS_LOC: u8 = 3;
/// An operation that cannot be applied: a packed record resuming after `NONE`,
/// or a record without an ELF type.
const MALFORMED: u32 = u32::MAX;

/// The symbol source of operation `i` of an N64 record: the record's own for
/// the first, `r_ssym` for the second, none for the third.
fn special_of(i: usize, r_type: u32) -> Option<u8> {
    match i {
        0 => None,
        1 => Some((r_type >> 24) as u8),
        _ => Some(RSS_UNDEF),
    }
}

/// The expressions of a section's relocation records, in order, as GNU ld
/// chains them: consecutive records at one offset are one expression, each
/// operation taking the previous result as its addend, until the offset changes
/// or an `R_MIPS_NONE` ends it; an N64 record contributes its up to three
/// packed operations in turn. A packed record whose operations resume after a
/// `NONE`, or a record without an ELF type, is malformed: it ends the chain
/// before it and is an expression of its own that cannot be applied and that
/// nothing continues. A JALR alone is only a hint and is dropped.
fn expressions(arch: object::Architecture, records: &[(u64, Relocation)]) -> Vec<Vec<Op>> {
    let packed = arch == object::Architecture::Mips64;
    let mut out: Vec<Vec<Op>> = Vec::new();
    let mut current: Vec<Op> = Vec::new();
    for (record, (offset, reloc)) in records.iter().enumerate() {
        let r_type = r_type_of(reloc).unwrap_or(MALFORMED);
        let kinds = if packed { ops(r_type) } else { [r_type, NONE, NONE] };
        let malformed = r_type == MALFORMED
            || packed && (kinds[0] == NONE && kinds[1..] != [NONE, NONE] || kinds[1] == NONE && kinds[2] != NONE);
        if malformed {
            out.extend((!current.is_empty()).then(|| std::mem::take(&mut current)));
            out.push(vec![Op { record, op: MALFORMED, special: None }]);
            continue;
        }
        for (i, &op) in kinds.iter().enumerate().take(if packed { 3 } else { 1 }) {
            if op == NONE || current.first().is_some_and(|start| records[start.record].0 != *offset) {
                out.extend((!current.is_empty()).then(|| std::mem::take(&mut current)));
            }
            if op != NONE {
                current.push(Op { record, op, special: if packed { special_of(i, r_type) } else { None } });
            }
        }
    }
    out.extend((!current.is_empty()).then_some(current));
    out.retain(|e| !(e.len() == 1 && e[0].op == JALR));
    out
}

/// The object's own `$gp` as an input to the link: `ri_gp_value` of its
/// `.reginfo`, or of an `ODK_REGINFO` entry in `.MIPS.options`. A partial link
/// records the `$gp` it already subtracted from local GP-relative values. An
/// object without either records none, which the linker reads as 0; a
/// malformed or conflicting record is an error for the expressions that use it.
pub(super) fn input_gp(file: &object::File) -> Result<u64, RelocationFailure> {
    let wide = file.is_64();
    let le = file.is_little_endian();
    let size = if wide { 32 } else { 24 };
    let gp_of = |data: &[u8]| -> Option<u64> {
        if wide {
            let b: [u8; 8] = data.get(24..32)?.try_into().ok()?;
            Some(if le { u64::from_le_bytes(b) } else { u64::from_be_bytes(b) })
        } else {
            word(data, 20, le).map(|w| w as i32 as i64 as u64)
        }
    };
    let mut found: Option<u64> = None;
    let mut note = |value: Option<u64>| match (value, found) {
        (Some(v), None) => {
            found = Some(v);
            Ok(())
        }
        (Some(v), Some(f)) if v == f => Ok(()),
        _ => Err(RelocationFailure::UnresolvedTarget),
    };
    for section in file.sections() {
        let Ok(data) = section.data() else { continue };
        match section.name() {
            Ok(".reginfo") => note((data.len() == size).then(|| gp_of(data)).flatten())?,
            Ok(".MIPS.options") => {
                let mut rest = data;
                while !rest.is_empty() {
                    let len = usize::from(*rest.get(1).ok_or(RelocationFailure::UnresolvedTarget)?);
                    let entry = rest.get(..len).filter(|_| len >= 8).ok_or(RelocationFailure::UnresolvedTarget)?;
                    if entry[0] == 1 {
                        note((len == 8 + size).then(|| gp_of(&entry[8..])).flatten())?;
                    }
                    rest = &rest[len..];
                }
            }
            _ => {}
        }
    }
    Ok(found.unwrap_or(0))
}

/// How many bytes an expression ending in operation `op` patches.
fn field_width(op: u32) -> Result<usize, RelocationFailure> {
    match op {
        R64 | SUB => Ok(8),
        R16 | R32 | REL32 | R26 | HI16 | LO16 | GPREL16 | LITERAL | GOT16 | PC16 | CALL16 | GPREL32 | GOT_DISP
        | GOT_PAGE | GOT_OFST | GOT_HI16 | GOT_LO16 | HIGHER | HIGHEST | CALL_HI16 | CALL_LO16 | PC32 => Ok(4),
        _ => Err(RelocationFailure::Unsupported),
    }
}

/// The bits of its field an expression ending in operation `op` replaces.
fn field_mask(op: u32) -> u64 {
    match op {
        R64 | SUB => u64::MAX,
        R32 | REL32 | GPREL32 | PC32 => 0xffff_ffff,
        R26 => 0x03ff_ffff,
        _ => 0xffff,
    }
}

/// Whether an expression's value is a symbol, a constant, or neither, so a
/// call through a composed expression is still named after its callee.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Origin {
    Constant,
    Symbol(RelocationTarget, i64),
    Unknown,
}

impl Origin {
    fn add(self, other: Self) -> Self {
        match (self, other) {
            (Self::Constant, v) | (v, Self::Constant) => v,
            (Self::Symbol(a, x), Self::Symbol(b, y)) if a == b => match x.checked_add(y) {
                Some(0) => Self::Constant,
                Some(n) => Self::Symbol(a, n),
                None => Self::Unknown,
            },
            _ => Self::Unknown,
        }
    }

    fn negate(self) -> Self {
        match self {
            Self::Symbol(t, n) => n.checked_neg().map_or(Self::Unknown, |n| Self::Symbol(t, n)),
            v => v,
        }
    }

    /// Unchanged when truncation kept the value (or, outside N64, its sign
    /// extension); otherwise no longer a symbol.
    fn narrow(self, before: u64, after: u64, arch: object::Architecture) -> Self {
        let kept = before == after
            || arch != object::Architecture::Mips64 && before == after as u32 as i32 as i64 as u64;
        if kept { self } else { self.transformed() }
    }

    fn transformed(self) -> Self {
        if self == Self::Constant { self } else { Self::Unknown }
    }

    fn callee(self) -> Option<RelocationTarget> {
        match self {
            Self::Symbol(t, 1) => Some(t),
            _ => None,
        }
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
