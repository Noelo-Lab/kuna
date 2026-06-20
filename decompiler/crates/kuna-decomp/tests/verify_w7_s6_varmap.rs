//! Verifier adversarial tests for item `w7-s6-varmap` (varmap stack-layout core).
//!
//! These are written by the INDEPENDENT verifier (not the porter) against the
//! C++ oracle `decompiler/cpp/varmap.cc`.  They target the hunt-list spots the
//! re-derivation flagged as most fragile for this item:
//!
//!   * the `sstart` (signed-start) scale chain in `MapState::addRange` /
//!     `MapState::initialize` — the PRIMARY sort key of `RangeHint::compare`,
//!     which is exercised by the gate only with word-size 1 (where the bug is
//!     masked).  `verify_addrange_sstart_scale_wordsize2` drives it with a
//!     word-addressed space (word-size 2) where the C++ `addressToByte`
//!     multiply and the ported `byteToAddressInt` divide diverge.
//!   * the `RangeHint::compare` total order under wrap-prone (huge-offset /
//!     negative-sstart) inputs.
//!   * `reconcileDatatypes` run-collapse on a degenerate all-equal-key list
//!     (empty/boundary container behaviour).
//!   * `merge` resType==2 ("confuse") size growth at the power-of-two boundary.

use std::rc::Rc;

use kuna_base::address::RangeList;
use kuna_base::space::{addrspace_flags, spacetype, AddrSpace};

use kuna_decomp::dtype::{type_metatype, Datatype, TypeFactory, TypeFactoryImpl};
use kuna_decomp::varmap::{MapState, RangeHint, RangeType, COPY_CONSTANT, TYPELOCK};

// --- builders -------------------------------------------------------------

/// Stack space with a configurable word-size.  8-byte addresses.
fn space_ws(ws: u32) -> Rc<AddrSpace> {
    Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "stack",
        false, // little-endian
        8,     // address size
        ws,    // word size
        4,     // index
        addrspace_flags::hasphysical,
        1,
        1,
    ))
}

fn base(size: i32, meta: type_metatype) -> Rc<Datatype> {
    Rc::new(Datatype::new_with_align(size, size, meta))
}

fn factory() -> TypeFactoryImpl {
    let f = TypeFactoryImpl::new();
    f.set_default_alignment_map();
    f.set_max_basetype_size(8);
    f
}

fn window(space: &Rc<AddrSpace>, first: u64, last: u64) -> RangeList {
    let mut rl = RangeList::new();
    rl.insert_range(Rc::clone(space), first, last);
    rl
}

// ===========================================================================
// F1 (MAJOR): sstart scale chain uses the wrong AddrSpace conversion.
//
// C++ `MapState::addRange` (varmap.cc:904-906):
//     intb sst = (intb)byteToAddress(st, ws);
//     sst = sign_extend(sst, addrSize*8-1);
//     sst = (intb)addressToByte(sst, ws);          // <-- MULTIPLY by ws
// The Rust port (varmap.rs:924-926) ends the chain with
//     sst = byte_to_address_int(sst, ws);          // <-- DIVIDE by ws
// For ws==1 both are the identity (the gate only ever uses ws==1), so the bug
// is fully masked there.  For ws==2 (a word-addressed space) the C++ value and
// the ported value diverge, and `sstart` is the PRIMARY key of
// `RangeHint::compare`, so the recovered stack layout order is corrupted.
//
// Oracle (ws=2, st=0x10): byteToAddress=0x10/2=0x8; sign_extend(+) = 0x8;
//   addressToByte = 0x8*2 = 0x10  ==>  C++ sstart == 0x10.
// Ported:           byteToAddress=0x8; byteToAddressInt = 0x8/2 = 0x4
//                                       ==>  Rust sstart == 0x4  (WRONG).
// ===========================================================================
#[test]
fn verify_addrange_sstart_scale_wordsize2() {
    let space = space_ws(2);
    let factory = factory();
    let dflt = factory.get_base(1, type_metatype::TYPE_UNKNOWN).unwrap();
    // Window covers up through the endpoint; word-size 2 means addressable
    // offsets, in-range check is byte based.
    let rn = window(&space, 0, 0x100);
    let pm = RangeList::new();
    let mut state = MapState::new(Rc::clone(&space), &rn, &pm, dflt);

    // Single 2-byte hint at byte offset 0x10.
    state.add_range_pub(0x10, Some(base(2, type_metatype::TYPE_INT)), 0, RangeType::Fixed, -1);
    assert!(state.initialize().unwrap());

    let got = state.next().sstart;
    // The C++ oracle computes sstart = addressToByte(byteToAddress(0x10,2),2)
    //                                 = (0x10/2)*2 = 0x10.
    assert_eq!(
        got, 0x10,
        "sstart scale chain must end in addressToByte (multiply by word-size); \
         C++ varmap.cc:906 == 0x10, ported divide gives {got:#x}"
    );
}

// A companion that pins the C++ semantics directly as arithmetic, independent
// of the port, so the expected value cannot be argued to be the verifier's
// misreading.  byteToAddress then addressToByte with the SAME word-size is the
// "round to a word multiple" identity-ish map (drops sub-word bits), NOT a
// divide.
#[test]
fn verify_sstart_oracle_is_word_rounding_not_divide() {
    let space = space_ws(2);
    let factory = factory();
    let dflt = factory.get_base(1, type_metatype::TYPE_UNKNOWN).unwrap();
    let rn = window(&space, 0, 0x100);
    let pm = RangeList::new();
    let mut state = MapState::new(Rc::clone(&space), &rn, &pm, dflt);

    // 0x11: C++ (0x11/2)*2 = 0x8*2 = 0x10 (low bit dropped, scale preserved).
    // ported divide: (0x11/2)/2 = 0x8/2 = 0x4.
    state.add_range_pub(0x11, Some(base(2, type_metatype::TYPE_INT)), 0, RangeType::Fixed, -1);
    assert!(state.initialize().unwrap());
    assert_eq!(
        state.next().sstart,
        0x10,
        "C++ word-scale round of byte 0x11 with ws=2 is 0x10"
    );
}

// ===========================================================================
// RangeHint::compare totality under wrap-prone / negative-sstart inputs.
// `sstart` is `intb` (i64); the comparator must be a strict total order even
// when one sstart is deeply negative (stack-grows-negative frames sign-extend
// high byte offsets to negative).  Anti-symmetry + the size/rangeType/flags/
// highind tie-break chain must all hold.
// ===========================================================================
#[test]
fn verify_compare_total_order_with_negative_sstarts() {
    // sstart from i64::MIN region up through a positive value.
    let mk = |sstart: i64, size: i32, rt: RangeType| {
        RangeHint::new(sstart as u64, size, sstart, base(size, type_metatype::TYPE_INT), 0, rt, -1)
    };
    let items = [
        mk(i64::MIN, 4, RangeType::Fixed),
        mk(-0x4000_0000, 8, RangeType::Open),
        mk(-1, 1, RangeType::Fixed),
        mk(0, 4, RangeType::Fixed),
        mk(0, 4, RangeType::Open), // same sstart+size, rangeType tie-break
        mk(0x10, 2, RangeType::Fixed),
        mk(i64::MAX, 4, RangeType::Endpoint),
    ];

    // Anti-symmetry and consistency: compare(a,b) == -compare(b,a) for all pairs.
    for a in &items {
        for b in &items {
            let ab = a.compare(b);
            let ba = b.compare(a);
            assert_eq!(ab, -ba, "compare must be anti-symmetric");
            if ab == 0 {
                // equality must be symmetric (it is, trivially via -0).
                assert_eq!(ba, 0);
            }
        }
    }
    // Transitivity spot-check on the sorted order: sorting must be stable and
    // monotone in sstart-then-size-then-rangeType.
    let mut v: Vec<RangeHint> = items.to_vec();
    v.sort_by(RangeHint::compare_ranges);
    let keys: Vec<(i64, i32, RangeType)> =
        v.iter().map(|h| (h.sstart, h.size, h.range_type)).collect();
    assert_eq!(
        keys,
        vec![
            (i64::MIN, 4, RangeType::Fixed),
            (-0x4000_0000, 8, RangeType::Open),
            (-1, 1, RangeType::Fixed),
            (0, 4, RangeType::Fixed),
            (0, 4, RangeType::Open),
            (0x10, 2, RangeType::Fixed),
            (i64::MAX, 4, RangeType::Endpoint),
        ]
    );
}

// ===========================================================================
// reconcileDatatypes run-collapse over a degenerate list: many hints that are
// compare()-equal (same start/size/flags/rangeType/highind) but differing
// data-types must collapse to a SINGLE surviving entry carrying the most
// specific type, and the endpoint must remain.  This stresses the `delete`/
// keep branch and the back-fill of `startDatatype` over the kept prefix.
// ===========================================================================
#[test]
fn verify_reconcile_collapses_many_equal_keyed_to_one() {
    let space = space_ws(1);
    let factory = factory();
    let dflt = factory.get_base(1, type_metatype::TYPE_UNKNOWN).unwrap();
    let rn = window(&space, 0, 0x100);
    let pm = RangeList::new();
    let mut state = MapState::new(Rc::clone(&space), &rn, &pm, dflt);

    // Five identical-key 4-byte hints at 0x20: UNKNOWN, INT, UINT, INT, UNKNOWN.
    // typeOrder ranks INT/UINT more specific than UNKNOWN; only one entry kept.
    for meta in [
        type_metatype::TYPE_UNKNOWN,
        type_metatype::TYPE_INT,
        type_metatype::TYPE_UINT,
        type_metatype::TYPE_INT,
        type_metatype::TYPE_UNKNOWN,
    ] {
        state.add_range_pub(0x20, Some(base(4, meta)), 0, RangeType::Fixed, -1);
    }
    assert!(state.initialize().unwrap());

    // Walk every surviving hint.
    let mut at_0x20 = 0usize;
    let mut starts = vec![state.next().start];
    if state.next().start == 0x20 {
        at_0x20 += 1;
    }
    while state.get_next() {
        let s = state.next().start;
        starts.push(s);
        if s == 0x20 {
            at_0x20 += 1;
        }
    }
    assert_eq!(at_0x20, 1, "all compare()-equal hints collapse to a single entry");
    // The artificial endpoint at last+1 = 0x101 must still be present.
    assert_eq!(*starts.last().unwrap(), 0x101);
    // The kept entry must carry a concrete (non-UNKNOWN) most-specific type.
    let kept_meta = state_first_meta(&space, &factory);
    assert_ne!(kept_meta, type_metatype::TYPE_UNKNOWN);
}

/// Helper: rebuild the same state and return the meta-type of the first entry
/// (the kept 0x20 hint) after initialize().
fn state_first_meta(space: &Rc<AddrSpace>, factory: &TypeFactoryImpl) -> type_metatype {
    let dflt = factory.get_base(1, type_metatype::TYPE_UNKNOWN).unwrap();
    let rn = window(space, 0, 0x100);
    let pm = RangeList::new();
    let mut state = MapState::new(Rc::clone(space), &rn, &pm, dflt);
    for meta in [
        type_metatype::TYPE_UNKNOWN,
        type_metatype::TYPE_INT,
        type_metatype::TYPE_UINT,
    ] {
        state.add_range_pub(0x20, Some(base(4, meta)), 0, RangeType::Fixed, -1);
    }
    state.initialize().unwrap();
    state.next().type_.get_metatype()
}

// ===========================================================================
// merge resType==2 ("confuse") size-growth at the power-of-two boundary.
// C++ varmap.cc:298-312: diff = b.sstart - sstart; if diff+b.size > size grow;
// if size not in {1,2,4,8} -> size=1, rangeType=open, type=UNKNOWN(1).
// Pick offsets so the grown size lands EXACTLY on 8 (stays fixed) vs 6
// (collapses to 1/open) to pin the boundary.
// ===========================================================================
#[test]
fn verify_merge_confuse_size_boundary() {
    let space = space_ws(1);
    let factory = factory();

    // Case A: grown size == 8 (a power of two) -> stays fixed, UNKNOWN(8).
    // a at 0, size 4; b at sstart 4, size 4 -> diff=4, 4+4=8 > 4 -> size 8.
    let mut a = RangeHint::new(0, 4, 0, base(4, type_metatype::TYPE_INT), 0, RangeType::Fixed, -1);
    let b = RangeHint::new(4, 4, 4, base(4, type_metatype::TYPE_UINT), 0, RangeType::Fixed, -1);
    let ret = a.merge(&b, &space, &factory).unwrap();
    assert!(!ret);
    assert_eq!(a.size, 8, "8 is a power of two: size stays grown");
    assert_eq!(a.range_type, RangeType::Fixed);
    assert_eq!(a.type_.get_metatype(), type_metatype::TYPE_UNKNOWN);

    // Case B: grown size == 6 (not a power of two) -> collapses to size 1/open.
    let mut a2 = RangeHint::new(0, 4, 0, base(4, type_metatype::TYPE_INT), 0, RangeType::Fixed, -1);
    let b2 = RangeHint::new(2, 4, 2, base(4, type_metatype::TYPE_UINT), 0, RangeType::Fixed, -1);
    let ret2 = a2.merge(&b2, &space, &factory).unwrap();
    assert!(!ret2);
    assert_eq!(a2.size, 1, "6 is not a power of two: collapses to 1");
    assert_eq!(a2.range_type, RangeType::Open);
}

// ===========================================================================
// merge resType==1 (prefer b): when `this` contains b but b's data-type is
// preferred, `this` adopts b's type/flags/rangeType/highind/size and absorbs a
// COPY of its former self.  Verify the field adoption.  (Both unlocked, same
// start so `contain` is true, b a more-specific type, fixed/fixed different
// size with reconcile=false makes the bigger preferred -- we instead force the
// b-preferred branch via a locked b.)
// ===========================================================================
#[test]
fn verify_merge_prefers_locked_b() {
    let space = space_ws(1);
    let factory = factory();
    // a unlocked 8-byte contains b locked 4-byte at the same start.
    // contain==true (same start); reconcile likely false; b is typelocked and a
    // is not -> preferred(b) returns false -> resType 1 -> adopt b.
    let mut a = RangeHint::new(0x10, 8, 0x10, base(8, type_metatype::TYPE_INT), 0, RangeType::Fixed, -1);
    let b = RangeHint::new(0x10, 4, 0x10, base(4, type_metatype::TYPE_UINT), TYPELOCK, RangeType::Fixed, -1);
    a.merge(&b, &space, &factory).unwrap();
    assert!(a.is_type_lock(), "adopted b's locked flags");
    assert_eq!(a.size, 4, "adopted b's size");
    assert_eq!(a.type_.get_metatype(), type_metatype::TYPE_UINT, "adopted b's type");
}

// ===========================================================================
// isConstAbsorbable highind-array end computation (uses align-size, wrap-safe).
// `this` open INT[ ] with highind>0: end = sstart + highind*alignSize, and a
// copy-constant b is absorbable iff its sstart <= that end.  Pin the boundary.
// ===========================================================================
#[test]
fn verify_const_absorbable_highind_end_boundary() {
    // this: open int (align 4), sstart 0x10, highind 2 -> end = 0x10 + 2*4 = 0x18.
    let mut this = RangeHint::new(0x10, 4, 0x10, base(4, type_metatype::TYPE_INT), 0, RangeType::Open, 2);
    this.highind = 2;

    // b exactly at end (0x18): absorbable.
    let mut b_at = RangeHint::new(0x18, 4, 0x18, base(4, type_metatype::TYPE_UNKNOWN), COPY_CONSTANT, RangeType::Fixed, -1);
    b_at.flags = COPY_CONSTANT;
    assert!(this.is_const_absorbable(&b_at), "b at end is absorbable (<=)");

    // b one byte past end (0x19): NOT absorbable.
    let mut b_past = RangeHint::new(0x19, 4, 0x19, base(4, type_metatype::TYPE_UNKNOWN), COPY_CONSTANT, RangeType::Fixed, -1);
    b_past.flags = COPY_CONSTANT;
    assert!(!this.is_const_absorbable(&b_past), "b past end is not absorbable");
}
