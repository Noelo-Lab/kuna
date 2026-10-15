//! (kuna `stackstruct`) Lay out an address-taken frame object that the function
//! accesses at several offsets with different widths as one synthesized record.
//!
//! A local record whose address is handed to a callee (`read(fd, &hdr, 8)`)
//! keeps no type in a stripped binary. The frame layout makes one scalar per
//! slot it finds, so the byte and short members inside a slot print as
//! Ghidra's partial-symbol tokens (`v1._4_2_`) and the slots after the first
//! become unrelated locals. This pass recognizes the object instead:
//!
//! * its start is a frame address that reaches a call as an argument, with no
//!   index added to it;
//! * its extent is what single accesses prove belongs to it: an access that
//!   starts inside the object and ends past the current end extends it, the
//!   writes one instruction made at contiguous offsets count as one access (a
//!   vector store the lane split cut into words), and a buffer call's byte count
//!   for the address (`memset(&h, 0, 0x18)`) covers that many bytes. Slots that
//!   merely sit next to the object are not part of it, and no single access
//!   joins two addresses that each reach a call at their own start;
//! * its members are the accesses inside the extent. An access that strictly
//!   contains another one is an initializer or a copy and claims nothing. It is
//!   a record only when at least two members of different widths remain.
//!
//! The object declines when an access crosses its start, a hint over it is
//! locked, shared or already a composite, a write stores text (a string built
//! in place), the bytes also hold one scalar value (a register-shared slot), or
//! its members cannot be laid out at their own alignment with a size that is a
//! multiple of the widest. The structure is minted
//! through the `structsynth` ledger, whose answer must be exactly the object's
//! size, and the object's hints are replaced by one fixed hint of it; the
//! existing member rendering then prints `v1.field_0x4`.
//!
//! The first layout waits until type propagation has settled, so a member's
//! type is the one its accesses agree on; later passes keep that structure while
//! no access crosses its bounds, so the layout cannot alternate between passes
//! and is measured once.

use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

use kuna_base::address::{sign_extend, Address};
use kuna_base::space::AddrSpace;
use kuna_base::types::{int4, intb, uintb};
use kuna_num::opcodes::OpCode;

use crate::context::{OpId, VarnodeId};
use crate::dtype::{type_metatype, Datatype, TypeFactory, TypeField};
use crate::funcdata::Funcdata;
use crate::varmap::{MapState, RangeHint, RangeType, SHARED_STORAGE, TYPELOCK};

/// The widest object this pass lays out.
const MAX_OBJECT: intb = 0x1000;

/// The member widths a record may have.
const FIELD_WIDTHS: [int4; 4] = [1, 2, 4, 8];

/// What a write stored, for the string-buffer decline.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Stored {
    Zero,
    Text,
    Other,
}

/// One stack access: its signed frame offset, width, the instruction that wrote
/// it (a write only), what it stored, whether an operation other than a
/// SUBPIECE reads it whole, the type it carried, and whether that type is
/// unknown over a float value.
struct Access {
    off: intb,
    size: int4,
    writer: Option<u64>,
    stored: Stored,
    whole_read: bool,
    ctype: Rc<Datatype>,
    float: bool,
}

impl Access {
    fn end(&self) -> intb {
        self.off + self.size as intb
    }
}

fn is_plumbing(code: OpCode) -> bool {
    matches!(code, OpCode::CPUI_INDIRECT | OpCode::CPUI_MULTIEQUAL)
}

/// What a frame write stored: zero, the bytes of text, or anything else.
fn stored(fd: &Funcdata, def: OpId, width: int4) -> Stored {
    let Some(op) = fd.obank().get(def) else { return Stored::Other };
    if op.code() != OpCode::CPUI_COPY {
        return Stored::Other;
    }
    let Some(v) = op.get_in(0).and_then(|i| fd.vbank().get(i)) else { return Stored::Other };
    if !v.is_constant() {
        return Stored::Other;
    }
    let n = width.clamp(0, 8) as u32;
    let bytes: Vec<u8> = (0..n).map(|i| ((v.get_offset() >> (8 * i)) & 0xff) as u8).collect();
    match crate::kuna_structsynth::bytes_text(&bytes) {
        None => Stored::Zero,
        Some(true) => Stored::Text,
        Some(false) => Stored::Other,
    }
}

/// Every stack Varnode in `[lo, hi)` some real operation writes or reads.
fn accesses(fd: &Funcdata, space: &Rc<AddrSpace>, lo: intb, hi: intb) -> Vec<Access> {
    let bits = space.get_addr_size() as int4 * 8 - 1;
    let from = Address::new(Rc::clone(space), space.wrap_offset(lo as uintb));
    let to = Address::new(Rc::clone(space), space.wrap_offset(hi as uintb));
    let mut out = Vec::new();
    for vn in fd.vbank().iter_loc_addr_range(&from, &to) {
        let Some(v) = fd.vbank().get(vn) else { continue };
        if v.is_free() || v.get_space().get_index() != space.get_index() {
            continue;
        }
        let off = sign_extend(v.get_offset() as intb, bits);
        if off < lo || off >= hi {
            continue;
        }
        let def = v.get_def().filter(|d| fd.obank().get(*d).is_some_and(|o| !is_plumbing(o.code())));
        let reader_ops: Vec<OpId> = v
            .descend_iter()
            .filter(|d| fd.obank().get(*d).is_some_and(|o| !is_plumbing(o.code())))
            .collect();
        let readers: Vec<OpCode> = reader_ops.iter().filter_map(|d| fd.obank().get(*d)).map(|o| o.code()).collect();
        if def.is_none() && readers.is_empty() {
            continue;
        }
        let float = v.get_type().get_metatype() == type_metatype::TYPE_UNKNOWN
            && def.iter().chain(reader_ops.iter()).any(|o| crate::kuna_partialaccess::carries_float(fd, vn, *o));
        out.push(Access {
            off,
            size: v.get_size(),
            writer: def.and_then(|d| fd.obank().get(d)).map(|o| o.get_addr().get_offset()),
            stored: def.map_or(Stored::Other, |d| stored(fd, d, v.get_size())),
            whole_read: readers.iter().any(|c| *c != OpCode::CPUI_SUBPIECE),
            ctype: Rc::clone(v.get_type()),
            float,
        });
    }
    out
}

/// The byte-count arguments of the library calls that touch a whole buffer:
/// `(name, pointer argument, size arguments multiplied)`. A `_chk` variant's
/// last size is the compiler's own object size.
const SIZED_CALLS: &[(&str, usize, &[usize])] = &[
    ("memset", 0, &[2]),
    ("memcpy", 0, &[2]),
    ("memcpy", 1, &[2]),
    ("memmove", 0, &[2]),
    ("memmove", 1, &[2]),
    ("memcmp", 0, &[2]),
    ("memcmp", 1, &[2]),
    ("bzero", 0, &[1]),
    ("explicit_bzero", 0, &[1]),
    ("read", 1, &[2]),
    ("write", 1, &[2]),
    ("recv", 1, &[2]),
    ("send", 1, &[2]),
    ("fread", 0, &[1, 2]),
    ("fwrite", 0, &[1, 2]),
    ("__memset_chk", 0, &[3]),
    ("__memcpy_chk", 0, &[3]),
    ("__memmove_chk", 0, &[3]),
    ("__read_chk", 1, &[3]),
    ("__fread_chk", 0, &[1]),
];

/// The calls the frame address `base` reaches as an argument: `None` when it
/// reaches none, else the largest byte count a buffer call states for it
/// (0 when no call states one).
fn call_extent(fd: &Funcdata, base: VarnodeId) -> Option<intb> {
    let v = fd.vbank().get(base)?;
    let mut called = false;
    let mut size: intb = 0;
    for d in v.descend_iter() {
        let Some(o) = fd.obank().get(d) else { continue };
        if !matches!(o.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND) {
            continue;
        }
        let slots: Vec<usize> = (1..o.num_input()).filter(|s| o.get_in(*s) == Some(base)).map(|s| s as usize - 1).collect();
        if slots.is_empty() {
            continue;
        }
        called = true;
        let Some(name) = fd.get_call_specs_index(d).map(|i| fd.get_call_specs(i).get_name().to_string()) else { continue };
        for (callee, ptr, sizes) in SIZED_CALLS {
            if *callee != name || !slots.contains(ptr) {
                continue;
            }
            let mut n: intb = 1;
            for arg in sizes.iter() {
                let c = o
                    .get_in(*arg as int4 + 1)
                    .and_then(|i| fd.vbank().get(i))
                    .filter(|c| c.is_constant())
                    .map(|c| c.get_offset() as intb);
                n = match c {
                    Some(c) if c > 0 && c <= MAX_OBJECT => n.saturating_mul(c),
                    _ => 0,
                };
            }
            size = size.max(n);
        }
    }
    called.then_some(size)
}

/// The byte ranges single accesses prove are one object: each access alone,
/// and each contiguous run of writes one instruction made. A range that
/// straddles the start of another escaped address is left out: a compiler
/// zeroes or reloads two adjacent locals with one access (an AArch64 `stp` of
/// `xzr`, an 8-byte reload of two 4-byte slots), and each of the two addresses
/// reaching a call of its own says they are two objects.
fn spans(acc: &[Access], bases: &[intb]) -> Vec<(intb, intb)> {
    let mut out: Vec<(intb, intb)> = acc.iter().map(|a| (a.off, a.end())).collect();
    let mut by_insn: BTreeMap<u64, Vec<(intb, intb)>> = BTreeMap::new();
    for a in acc {
        if let Some(w) = a.writer {
            by_insn.entry(w).or_default().push((a.off, a.end()));
        }
    }
    for (_, mut runs) in by_insn {
        if runs.len() < 2 {
            continue;
        }
        runs.sort();
        let (mut lo, mut hi) = runs[0];
        for (l, h) in runs.into_iter().skip(1) {
            if l <= hi {
                hi = hi.max(h);
            } else {
                out.push((lo, hi));
                lo = l;
                hi = h;
            }
        }
        out.push((lo, hi));
    }
    out.retain(|&(lo, hi)| !bases.iter().any(|b| lo < *b && *b < hi));
    out
}

/// Does an access cross the boundary at `at`?
fn crosses(spans: &[(intb, intb)], at: intb) -> bool {
    spans.iter().any(|&(lo, hi)| lo < at && hi > at)
}

/// The end of the object that starts at `start` and is at least `sized` bytes,
/// or `None` when an access crosses its start or nothing proves any extent.
fn extent(spans: &[(intb, intb)], start: intb, sized: intb) -> Option<intb> {
    if crosses(spans, start) {
        return None;
    }
    let mut end = start + sized;
    loop {
        let before = end;
        for &(lo, hi) in spans {
            if (lo == start || (lo > start && lo < end)) && hi > end {
                end = hi;
            }
        }
        if end == before {
            return (end > start).then_some(end);
        }
    }
}

/// One claimed member: its width, the type every access of it agreed on, and
/// whether an access of it carried a float its unknown type does not say.
type Claims = BTreeMap<intb, (int4, Option<Rc<Datatype>>, bool)>;

/// The members the accesses inside `[start, end)` claim, keyed by object
/// offset, or `None` when two claims overlap.
fn claims(acc: &[Access], start: intb, end: intb) -> Option<Claims> {
    let inside: Vec<&Access> = acc.iter().filter(|a| a.off >= start && a.end() <= end).collect();
    let contains = |a: &Access, b: &Access| {
        a.off <= b.off && a.end() >= b.end() && (a.off, a.size) != (b.off, b.size)
    };
    let mut out = Claims::new();
    for a in inside.iter().filter(|a| !inside.iter().any(|b| contains(a, b))) {
        let key = a.off - start;
        match out.get_mut(&key) {
            Some((w, t, f)) => {
                if *w != a.size {
                    return None;
                }
                if t.as_ref().is_some_and(|t| !Rc::ptr_eq(t, &a.ctype)) {
                    *t = None;
                }
                *f |= a.float;
            }
            None => {
                out.insert(key, (a.size, Some(Rc::clone(&a.ctype)), a.float));
            }
        }
    }
    let mut last = intb::MIN;
    for (off, (w, _, _)) in &out {
        if *off < last {
            return None;
        }
        last = off + *w as intb;
    }
    Some(out)
}

/// Do the claims describe a record rather than one scalar or a run of equal
/// elements?
fn is_record(claimed: &Claims) -> bool {
    let widths: BTreeSet<int4> = claimed.values().map(|(w, _, _)| *w).collect();
    claimed.len() >= 2 && widths.len() >= 2
}

/// Do the bytes also hold one scalar value?  A wide access over members that
/// stores anything but a constant initializer copies a value in, and a read of
/// the whole object uses it as one value; a register-shared slot does both and
/// a record does neither.
fn is_scalar_too(acc: &[Access], start: intb, end: intb) -> bool {
    let inside: Vec<&Access> = acc.iter().filter(|a| a.off >= start && a.end() <= end).collect();
    inside.iter().any(|a| {
        let wide = inside
            .iter()
            .any(|b| a.off <= b.off && a.end() >= b.end() && (a.off, a.size) != (b.off, b.size));
        let whole = a.off == start && a.end() == end;
        (wide && a.writer.is_some() && a.stored == Stored::Other) || (whole && a.whole_read)
    })
}

/// Does a write store the bytes of text: a string built in place, which a
/// compiler splits into words of whatever widths fit?
fn is_text(acc: &[Access], start: intb, end: intb) -> bool {
    acc.iter()
        .any(|a| a.off >= start && a.end() <= end && a.writer.is_some() && a.stored == Stored::Text)
}

/// A member type that spells exactly its own width, a float for an unknown that
/// carries one, or the honest unknown.
fn member_type(types: &dyn TypeFactory, width: int4, ct: Option<&Rc<Datatype>>, float: bool) -> Option<Rc<Datatype>> {
    let scalar = |t: &&Rc<Datatype>| {
        t.get_size() == width
            && matches!(
                t.get_metatype(),
                type_metatype::TYPE_INT
                    | type_metatype::TYPE_UINT
                    | type_metatype::TYPE_BOOL
                    | type_metatype::TYPE_FLOAT
            )
    };
    match ct.filter(scalar) {
        Some(t) => Some(Rc::clone(t)),
        None if float && matches!(width, 4 | 8) => types.get_base(width, type_metatype::TYPE_FLOAT).ok(),
        None => types.get_base(width, type_metatype::TYPE_UNKNOWN).ok(),
    }
}

/// Cover `[from, to)`: alignment padding with unknown members of the widest
/// width each offset allows, a longer gap with one `undefined1[N]` member.
fn fill(types: &dyn TypeFactory, fields: &mut Vec<TypeField>, from: int4, to: int4) -> Option<()> {
    if to - from > 8 {
        let byte = types.get_base(1, type_metatype::TYPE_UNKNOWN).ok()?;
        let ct = types.get_type_array(to - from, byte).ok()?;
        fields.push(TypeField::new(fields.len() as int4, from, format!("field_0x{from:x}"), ct));
        return Some(());
    }
    let mut at = from;
    while at < to {
        let width = FIELD_WIDTHS
            .iter()
            .rev()
            .copied()
            .find(|w| at % w == 0 && at + w <= to)
            .unwrap_or(1);
        let ct = types.get_base(width, type_metatype::TYPE_UNKNOWN).ok()?;
        fields.push(TypeField::new(fields.len() as int4, at, format!("field_0x{at:x}"), ct));
        at += width;
    }
    Some(())
}

/// The dense member list for an object of `size` bytes, or `None` when a claim
/// is not a naturally aligned 1, 2, 4 or 8 bytes or the size is not a multiple
/// of the widest member (a compiler reading the definition back would place it
/// elsewhere).
fn layout(types: &dyn TypeFactory, claimed: &Claims, size: int4) -> Option<Vec<TypeField>> {
    let mut fields: Vec<TypeField> = Vec::new();
    let mut next: int4 = 0;
    let mut align: int4 = 1;
    for (off, (width, ct, float)) in claimed {
        let off = int4::try_from(*off).ok()?;
        if !FIELD_WIDTHS.contains(width) || off % *width != 0 {
            return None;
        }
        fill(types, &mut fields, next, off)?;
        let ct = member_type(types, *width, ct.as_ref(), *float)?;
        fields.push(TypeField::new(fields.len() as int4, off, format!("field_0x{off:x}"), ct));
        align = align.max(*width);
        next = off + *width;
    }
    if next > size || size % align != 0 {
        return None;
    }
    fill(types, &mut fields, next, size)?;
    Some(fields)
}

/// Is a hint over the object already a decision this pass must not override:
/// locked, shared storage, or a composite some other evidence typed?
fn is_decided(h: &RangeHint) -> bool {
    if h.flags & (TYPELOCK | SHARED_STORAGE) != 0 || (h.range_type == RangeType::Open && h.highind >= 0) {
        return true;
    }
    match h.type_.get_metatype() {
        type_metatype::TYPE_STRUCT | type_metatype::TYPE_UNION => {
            !crate::kuna_structsynth::ledger::is_minted(&h.type_)
        }
        type_metatype::TYPE_ARRAY => h
            .type_
            .get_array_base()
            .is_none_or(|e| e.get_metatype() != type_metatype::TYPE_UNKNOWN || e.get_size() != 1),
        _ => false,
    }
}

/// A frame record a pass laid out: its signed start, size and structure.
#[derive(Clone)]
pub struct StackRecord {
    pub start: intb,
    pub size: int4,
    pub record: Rc<Datatype>,
}

/// The measured layout of the record at `start`, or `None` when the evidence
/// declines it.
fn measure(
    types: &dyn TypeFactory,
    acc: &[Access],
    spans: &[(intb, intb)],
    start: intb,
    sized: intb,
    frame_end: intb,
) -> Option<(intb, Vec<TypeField>)> {
    let end = extent(spans, start, sized)?;
    if end > frame_end || end - start > MAX_OBJECT || is_text(acc, start, end) || is_scalar_too(acc, start, end) {
        return None;
    }
    let claimed = claims(acc, start, end)?;
    if !is_record(&claimed) {
        return None;
    }
    Some((end, layout(types, &claimed, (end - start) as int4)?))
}

/// Replace the hints of every recognized frame record with one fixed hint of
/// its synthesized structure.
pub(super) fn coalesce(fd: &Funcdata, state: &mut MapState, space: &Rc<AddrSpace>) {
    if !fd.get_arch().stack_struct || space.get_word_size() != 1 {
        return;
    }
    let Some(types) = fd.get_arch().types_rc() else { return };
    let frame_end = state.end_sstart().unwrap_or(0).min(0);
    let bits = space.get_addr_size() as int4 * 8 - 1;
    let mut bases: BTreeMap<intb, intb> = BTreeMap::new();
    {
        let checker = state.checker_mut();
        for (off, b) in checker.get_alias().iter().copied().zip(checker.get_add_base()) {
            let start = sign_extend(space.wrap_offset(off) as intb, bits);
            if b.index.is_some() || start >= frame_end {
                continue;
            }
            if let Some(sized) = call_extent(fd, b.base) {
                let e = bases.entry(start).or_insert(0);
                *e = (*e).max(sized);
            }
        }
    }
    let Some(&lowest) = bases.keys().next() else { return };
    let known = fd.stack_records();
    let settled =
        fd.has_type_recovery_started() && !fd.is_type_recovery_exceeded() && fd.kuna_infertypes_settled();
    if known.is_empty() && !settled {
        return;
    }
    let acc = accesses(fd, space, lowest, frame_end);
    let escaped: Vec<intb> = bases.keys().copied().collect();
    let spans = spans(&acc, &escaped);
    let mut taken_end = intb::MIN;
    for (start, sized) in bases {
        if start < taken_end {
            continue;
        }
        let kept = known.iter().find(|r| r.start == start).filter(|r| {
            let end = start + r.size as intb;
            !crosses(&spans, start) && !crosses(&spans, end) && end <= frame_end
        });
        let found = match kept {
            Some(k) => Some(k.clone()),
            None if settled => measure(types.as_ref(), &acc, &spans, start, sized, frame_end)
                .and_then(|(end, fields)| mint(fd, types.as_ref(), start, end, fields)),
            None => None,
        };
        let Some(found) = found else { continue };
        let end = start + found.size as intb;
        let at: uintb = space.wrap_offset(start as uintb);
        let overlaps = |h: &RangeHint| h.sstart < end && h.sstart + h.size as intb > start;
        if !state.covers(at, found.size) || state.hints_mut().iter().any(|h| overlaps(h) && is_decided(h)) {
            continue;
        }
        state.hints_mut().retain(|h| !overlaps(h));
        state.add_range_pub(at, Some(Rc::clone(&found.record)), 0, RangeType::Fixed, -1);
        fd.record_stack_record(found);
        taken_end = end;
    }
}

/// The program-wide structure for a measured layout, through the `structsynth`
/// ledger, which must answer with exactly the object's size.
fn mint(
    fd: &Funcdata,
    types: &dyn TypeFactory,
    start: intb,
    end: intb,
    fields: Vec<TypeField>,
) -> Option<StackRecord> {
    let size = (end - start) as int4;
    let record = crate::kuna_structsynth::answer_frame(fd, types, fields, size)?;
    Some(StackRecord { start, size, record })
}
