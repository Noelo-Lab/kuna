//! Independent VERIFIER adversarial tests for item `w6-s5-type-2`.
//!
//! These are written by the reviewer (not the porter) and target the spots the
//! hunt list flagged as most fragile for this item:
//!
//!   1. `TypeStruct::getFieldIter` / `getLowerBoundField` binary-search boundary
//!      behaviour — fields with *holes* (padding) between them, offsets before
//!      the first field, and the asymmetric `(min+max+1)/2` rounding in
//!      `getLowerBoundField` (type.cc:1754-1921).  Exercised through the public
//!      `get_sub_type` / `get_hole_size` / `find_truncation` surfaces.
//!   2. `TypeEnum::getMatches` predecessor walk over the namemap: the
//!      `else`-branch ("restrict" path `target = curval & ~diff`), the
//!      `iter == begin()` boundary (target smaller than all named values), and
//!      the `--end()` boundary, plus the complement fallback (type.cc:1537-1586).
//!   3. `assignFieldOffsets` / `assignContiguousBitfields` interleaving: a
//!      bit-field group landing *between* two regular fields (the `nextBitPos`
//!      machinery) and the big-endian post-assignment reversal (type.cc:2322-2409).
//!
//! C++ oracle: `decompiler/cpp/type.cc` (the cited line ranges) and
//! `decompiler/cpp/address.cc` (`coveringmask`, `calc_mask`).

use std::collections::BTreeMap;
use std::rc::Rc;

use kuna_decomp::dtype::{
    flags, type_metatype, Datatype, DatatypeKind, TypeBitField, TypeField,
};
use kuna_decomp::context::OpId;

fn scalar(size: i32, align: i32, m: type_metatype) -> Rc<Datatype> {
    let mut dt = Datatype::new_with_align(size, align, m);
    dt.align_size = Datatype::calc_align_size(size, align);
    Rc::new(dt)
}

fn make_struct(size: i32, align: i32, fields: Vec<TypeField>) -> Datatype {
    let mut s = Datatype::new_with_align(size, align, type_metatype::TYPE_STRUCT);
    s.align_size = Datatype::calc_align_size(size, align);
    s.kind = DatatypeKind::Struct { field: fields, bitfield: vec![] };
    s
}

fn make_enum(size: i32, entries: &[(u64, &str)]) -> Datatype {
    let mut dt = Datatype::new_with_align(size, size, type_metatype::TYPE_INT);
    dt.flags |= flags::enumtype;
    let mut namemap: BTreeMap<u64, String> = BTreeMap::new();
    for (v, n) in entries {
        namemap.insert(*v, (*n).to_string());
    }
    dt.kind = DatatypeKind::Enum { namemap };
    dt
}

// ===========================================================================
// (1) Binary-search boundaries through getSubType / getHoleSize / findTruncation
// ===========================================================================

/// A struct with a *hole*: a@[0,4), then c@[8,12), with bytes 4-7 and 12-15
/// unaccounted (size 16).  This stresses both binary searches:
///   - `getFieldIter` must return -1 for an offset inside the hole (8 > a's end,
///     8 not yet reached by c when probing 5).
///   - `getLowerBoundField` must return the *last* field starting at-or-before
///     the offset even when that field does not contain it (the `(min+max+1)/2`
///     upper-rounding selects the right element).
#[test]
fn struct_binary_search_with_holes() {
    let int_t = scalar(4, 4, type_metatype::TYPE_INT);
    let s = make_struct(
        16,
        4,
        vec![
            TypeField::new(0, 0, "a", Rc::clone(&int_t)),
            TypeField::new(8, 8, "c", Rc::clone(&int_t)),
        ],
    );

    // getSubType: inside field a.
    let (sub, newoff) = s.get_sub_type(2).unwrap();
    assert_eq!(sub.unwrap().get_size(), 4);
    assert_eq!(newoff, 2);

    // getSubType at offset 5 — inside the hole: getFieldIter returns -1, base
    // body returns (None, off).
    let (sub, newoff) = s.get_sub_type(5).unwrap();
    assert!(sub.is_none());
    assert_eq!(newoff, 5);

    // getSubType at exactly the start of c.
    let (sub, newoff) = s.get_sub_type(8).unwrap();
    assert_eq!(sub.unwrap().get_size(), 4);
    assert_eq!(newoff, 0);

    // getHoleSize at offset 5: getLowerBoundField -> a (offset 0 <= 5), newOff =
    // 5, 5 < a.size(4)? no -> advance to c, distance = c.offset(8) - 5 = 3.
    assert_eq!(s.get_hole_size(5).unwrap(), 3);

    // getHoleSize at offset 12 (after c): getLowerBoundField -> c (offset 8),
    // newOff = 4, 4 < 4? no -> advance past end -> size(16) - 12 = 4.
    assert_eq!(s.get_hole_size(12).unwrap(), 4);

    // getHoleSize at offset 0: getLowerBoundField -> a, newOff 0 < a.size ->
    // recurse into a (a scalar with no hole) -> 0.
    assert_eq!(s.get_hole_size(0).unwrap(), 0);

    // findTruncation: piece [2,4) fits inside a.
    let r = s.find_truncation(2, 2, OpId::default(), 0).unwrap();
    assert_eq!(r, Some((0, 2)));
    // findTruncation: piece [2,6) spans past a's end -> None (spans >1 field).
    let r = s.find_truncation(2, 4, OpId::default(), 0).unwrap();
    assert_eq!(r, None);
    // findTruncation in the hole -> None.
    let r = s.find_truncation(5, 1, OpId::default(), 0).unwrap();
    assert_eq!(r, None);
}

/// Offset strictly before the first field: `getLowerBoundField` must return -1
/// (no field at-or-before the offset), and `getHoleSize` then reports the
/// distance to the first field.  Empty struct: both searches degenerate
/// (max = -1, loop body never runs).
#[test]
fn struct_lower_bound_before_first_and_empty() {
    let int_t = scalar(4, 4, type_metatype::TYPE_INT);

    // First field starts at offset 4 (a leading hole [0,4)).
    let s = make_struct(8, 4, vec![TypeField::new(4, 4, "a", Rc::clone(&int_t))]);
    // getHoleSize(0): getLowerBoundField(0) -> -1 (only field starts at 4 > 0);
    // i becomes 0, field[0].offset(4) - 0 = 4.
    assert_eq!(s.get_hole_size(0).unwrap(), 4);
    // getHoleSize(2): still before the field -> 4 - 2 = 2.
    assert_eq!(s.get_hole_size(2).unwrap(), 2);
    // getSubType(1): getFieldIter -> -1 (field starts at 4) -> (None, 1).
    let (sub, newoff) = s.get_sub_type(1).unwrap();
    assert!(sub.is_none());
    assert_eq!(newoff, 1);

    // Empty struct: getSubType / getHoleSize on the degenerate search.
    let empty = make_struct(8, 1, vec![]);
    let (sub, newoff) = empty.get_sub_type(3).unwrap();
    assert!(sub.is_none());
    assert_eq!(newoff, 3);
    // getLowerBoundField(empty) -> -1, i becomes 0, 0 < len(0)? no -> size - off.
    assert_eq!(empty.get_hole_size(3).unwrap(), 8 - 3);
}

/// `getLowerBoundField` rounding stress: many fields, probe an offset that lands
/// exactly on a field boundary and one that lands one short of it, to force the
/// `(min + max + 1)/2` mid to converge on the correct (upper) element.
#[test]
fn struct_lower_bound_rounding() {
    let int1 = scalar(1, 1, type_metatype::TYPE_INT);
    // Fields at 0,1,2,3,4,5 (six contiguous 1-byte fields), size 6.
    let mut fields = Vec::new();
    for i in 0..6 {
        fields.push(TypeField::new(i, i, format!("f{i}"), Rc::clone(&int1)));
    }
    let s = make_struct(6, 1, fields);

    // getSubType at each offset hits exactly the field with that offset.
    for off in 0..6i64 {
        let (sub, newoff) = s.get_sub_type(off).unwrap();
        assert!(sub.is_some(), "offset {off} should land in a field");
        assert_eq!(newoff, 0, "offset {off} is the field start");
    }
    // getHoleSize on a fully-packed struct: every byte is inside a 1-byte field
    // whose own hole-size is 0.
    for off in 0..6i32 {
        assert_eq!(s.get_hole_size(off).unwrap(), 0, "no hole at {off}");
    }
}

// ===========================================================================
// (2) TypeEnum::getMatches predecessor-walk corners
// ===========================================================================

/// The "restrict" `else` branch: when the biggest named value <= target has a
/// bit set *below* the highest differing bit, the C++ does not accept it but
/// narrows `target` to `curval & ~diff` and searches again.  Construct
/// { 0x05:"FIVE", 0x02:"TWO" } (size 1) and ask for 0x06:
///   target=0x06 -> pred 0x05, diff=coveringmask(0x03)=0x03, 0x05&0x03=0x01!=0
///   -> else: target = 0x05 & ~0x03 = 0x04
///   target=0x04 -> pred 0x02, diff=coveringmask(0x04)=0x07 >= bitsleft(6) -> break
/// No direct match; complement likewise fails -> empty representation.
#[test]
fn enum_getmatches_restrict_branch() {
    let e = make_enum(1, &[(0x05, "FIVE"), (0x02, "TWO")]);
    let rep = e.get_matches(0x06).unwrap();
    assert!(
        rep.match_name.is_empty(),
        "0x06 has no OR-of-names representation, got {:?}",
        rep.match_name
    );
    assert!(!rep.complement);

    // 0x05 itself is an exact named value -> single name, no complement.
    let rep = e.get_matches(0x05).unwrap();
    assert_eq!(rep.match_name, vec!["FIVE".to_string()]);
    assert!(!rep.complement);

    // 0x07 is NOT representable by the greedy MSB walk even though
    // 0x05 | 0x02 == 0x07: probing target 0x07 finds pred FIVE(0x05) with
    // diff=coveringmask(0x02)=0x03 and (0x05 & 0x03)=0x01 != 0, so FIVE is
    // *rejected* (the restrict branch narrows target to 0x04); the next pred
    // TWO(0x02) then trips `diff >= bitsleft` and breaks.  The faithful C++
    // result is therefore the empty representation, not [FIVE, TWO].  This
    // pins the exact else-branch + break interplay.
    let rep = e.get_matches(0x07).unwrap();
    assert!(
        rep.match_name.is_empty(),
        "0x07 is not greedily representable, got {:?}",
        rep.match_name
    );
    assert!(!rep.complement);
}

/// `iter == namemap.begin()` boundary: the target is smaller than every named
/// value, so `upper_bound` lands on `begin()` and the predecessor walk yields
/// nothing -> immediate break.  { 0x10:"X" } (size 1), value 0x08:
///   direct: target=0x08 -> pred None -> break, bitsleft!=0 -> fail
///   complement: 0x08 ^ 0xff = 0xf7 -> pred 0x10, diff=coveringmask(0xe7)=0xff
///     >= bitsleft -> break -> fail.  Empty representation.
#[test]
fn enum_getmatches_begin_boundary() {
    let e = make_enum(1, &[(0x10, "X")]);
    let rep = e.get_matches(0x08).unwrap();
    assert!(rep.match_name.is_empty());
    assert!(!rep.complement);

    // Exact hit on the single value.
    let rep = e.get_matches(0x10).unwrap();
    assert_eq!(rep.match_name, vec!["X".to_string()]);
    assert!(!rep.complement);

    // Zero with no ZERO entry: handled specially, no match, then complement
    // 0x00 ^ 0xff = 0xff -> pred 0x10, diff=coveringmask(0xff^0x10)=0xff >=
    // bitsleft -> fail.  Empty.
    let rep = e.get_matches(0x00).unwrap();
    assert!(rep.match_name.is_empty());
}

/// `--end()` boundary: target greater than every named value, so `upper_bound`
/// returns `end()` and the predecessor is the *last* (largest) entry.  Verify a
/// value that decomposes into the two largest names walked from the top.
/// { 0x08:"EIGHT", 0x04:"FOUR", 0x01:"ONE" } (size 1), value 0x0d = 8|4|1.
#[test]
fn enum_getmatches_end_boundary() {
    let e = make_enum(1, &[(0x08, "EIGHT"), (0x04, "FOUR"), (0x01, "ONE")]);
    let rep = e.get_matches(0x0d).unwrap();
    assert_eq!(
        rep.match_name,
        vec!["EIGHT".to_string(), "FOUR".to_string(), "ONE".to_string()]
    );
    assert!(!rep.complement);

    // Complement path: value whose complement is exactly EIGHT.
    // 0xf7 ^ 0xff = 0x08 -> "EIGHT", complement = true.
    let rep = e.get_matches(0xf7).unwrap();
    assert_eq!(rep.match_name, vec!["EIGHT".to_string()]);
    assert!(rep.complement);
}

// ===========================================================================
// (3) assignFieldOffsets: bitfields between fields + big-endian reversal
// ===========================================================================

/// A bit-field group placed *between* two regular fields (ident = 1 means
/// "insert before field index 1").  This drives the `nextBitPos` machinery in
/// `assignFieldOffsets` (the `pos == nextBitPos` check fires mid-loop) and the
/// big-endian reversal in `assignContiguousBitfields`.
///
/// Layout: char a@0 (size1) -> offset 1; bitfields {3 bits, 5 bits} sharing
/// ident 1 -> one byte container at offset 1 (byteSize 1); char b@2 (size1) ->
/// offset 3.  Big-endian -> the two bit-fields are reversed after LSB
/// assignment.
#[test]
fn assign_field_offsets_interleaved_bitfields_bigendian() {
    let char_t = scalar(1, 1, type_metatype::TYPE_INT);
    let mut list = vec![
        TypeField::new(-1, -1, "a", Rc::clone(&char_t)),
        TypeField::new(-1, -1, "b", Rc::clone(&char_t)),
    ];
    // Both bit-fields carry ident 1 => contiguous group inserted before field 1.
    let mut bitlist = vec![
        TypeBitField::new(1, 3, /*big_endian=*/ true, "bf_lo", Rc::clone(&char_t)),
        TypeBitField::new(1, 5, /*big_endian=*/ true, "bf_hi", Rc::clone(&char_t)),
    ];

    let (new_size, new_align, fl) =
        Datatype::assign_field_offsets(&mut list, &mut bitlist).unwrap();

    // Regular field offsets straddle the bit-field byte.
    assert_eq!(list[0].offset, 0, "a at 0");
    assert_eq!(list[1].offset, 2, "b after the 1-byte bitfield container");
    assert_eq!(new_align, 1);
    assert_eq!(new_size, 3);
    assert_eq!(fl & flags::has_bitfields, flags::has_bitfields);

    // The contiguous group totals 3+5 = 8 bits = 1 byte at offset 1.
    for bf in &bitlist {
        assert_eq!(bf.byte_offset, 1, "bitfield container offset");
        assert_eq!(bf.byte_size, 1, "bitfield container size");
    }
    // LSBs assigned 0 then 3 (lo then hi) *before* the big-endian reversal, so
    // after reversal the vector is [hi(lsb=3), lo(lsb=0)].
    assert_eq!(bitlist.len(), 2);
    assert_eq!(bitlist[0].least_sig_bit, 3, "reversed: hi (lsb 3) first");
    assert_eq!(bitlist[0].num_bits, 5);
    assert_eq!(bitlist[1].least_sig_bit, 0, "reversed: lo (lsb 0) second");
    assert_eq!(bitlist[1].num_bits, 3);
}

/// Little-endian contiguous bit-fields are *not* reversed: LSB order is
/// preserved.  Same layout, big_endian = false.
#[test]
fn assign_field_offsets_bitfields_littleendian_no_reversal() {
    let char_t = scalar(1, 1, type_metatype::TYPE_INT);
    let mut list = vec![TypeField::new(-1, -1, "a", Rc::clone(&char_t))];
    let mut bitlist = vec![
        TypeBitField::new(1, 3, false, "bf_lo", Rc::clone(&char_t)),
        TypeBitField::new(1, 5, false, "bf_hi", Rc::clone(&char_t)),
    ];
    let (new_size, _new_align, fl) =
        Datatype::assign_field_offsets(&mut list, &mut bitlist).unwrap();

    assert_eq!(list[0].offset, 0);
    // Bit-fields trail field a; byte container at offset 1, size 1, total 2.
    assert_eq!(new_size, 2);
    assert_eq!(fl & flags::has_bitfields, flags::has_bitfields);
    // No reversal: declaration order [lo(lsb0), hi(lsb3)] preserved.
    assert_eq!(bitlist[0].least_sig_bit, 0);
    assert_eq!(bitlist[0].num_bits, 3);
    assert_eq!(bitlist[1].least_sig_bit, 3);
    assert_eq!(bitlist[1].num_bits, 5);
    // idents reset to declaration index.
    assert_eq!(bitlist[0].ident, 0);
    assert_eq!(bitlist[1].ident, 1);
}
