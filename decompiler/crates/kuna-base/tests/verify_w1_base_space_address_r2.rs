//! VERIFIER round-2 adversarial tests for item `w1-base-space-address`
//! (space.cc/.hh + address.cc/.hh).  Written by the independent verifier,
//! NOT the porter.  Round 2 targets the areas round 1 left unpinned:
//! the whole BitRange class (address.cc:630-868, untouched by the round-1
//! verifier tests), the uintb sign_extend / signbit_negative helpers with
//! out-of-contract sizes, and the RangeList merge / getLastSignedRange /
//! non-chaining-inRange semantics.  Expected values marked "C++ oracle"
//! were produced by a standalone micro-oracle transcribing the verbatim
//! upstream code (g++ -std=c++11 -O0, x86-64).

use std::rc::Rc;

use kuna_base::address::{
    sign_extend_sized, signbit_negative, Address, BitRange, RangeList,
};
use kuna_base::space::{addrspace_flags, spacetype, AddrSpace};

fn proc_space(name: &str, index: i32, addr_size: u32, wordsize: u32) -> AddrSpace {
    AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        name,
        false,
        addr_size,
        wordsize,
        index,
        addrspace_flags::hasphysical,
        0,
        0,
    )
}

/// BitRange::getMask (address.cc:810-824): the `numBits >= sizeof(uintb)*8`
/// comparison is int4-vs-size_t, so a *negative* numBits converts huge and
/// takes the res=0 branch; the final `res <<= leastSigBit` is then a C++ UB
/// shift for the undefined range (leastSigBit == -1) that x86 resolves by
/// masking the count to 63.
/// C++ oracle: getMask(undefined)        == 0x8000000000000000
///             getMask(nb=64, lsb=0)     == u64::MAX
///             getMask(nb=70, lsb=8)     == 0xffffffffffffff00
/// Also: BitRange::empty() is `numBits <= 0` (address.hh:269), so the
/// undefined range (numBits == -1) IS empty.
#[test]
fn verify_bitrange_getmask_undefined_and_oversize_cpp_oracle() {
    let undef = BitRange::new();
    assert_eq!(undef.get_mask(), 0x8000000000000000);
    assert!(undef.empty());

    let b64 = BitRange::with_bits(0, 8, 0, 64, false);
    assert_eq!(b64.get_mask(), u64::MAX);
    let b70 = BitRange::with_bits(0, 16, 8, 70, false);
    assert_eq!(b70.get_mask(), 0xffffffffffffff00);

    // in-range sanity row pinned alongside the oracle run
    let b = BitRange::with_bits(0, 4, 8, 4, false);
    assert_eq!(b.get_mask(), 0xf00);
}

/// BitRange::translateLSB/overlapTest (address.cc:660-705): the full
/// big-endian return-code matrix and a cross-container translation.
/// C++ oracle (container (0,4) BE, field lsb=8 nb=16):
///   vs (0,4,lsb=0,nb=8)   -> -1   (below)
///   vs (0,4,lsb=24,nb=8)  ->  1   (above)
///   vs (0,4,lsb=8,nb=16)  ->  0   (same)
///   vs (0,4,lsb=4,nb=24)  ->  2   (this contained in op2)
///   vs (0,4,lsb=12,nb=4)  ->  3   (op2 contained in this)
///   vs (0,4,lsb=4,nb=8)   ->  4   (partial)
///   vs (4,4,lsb=8,nb=16)  -> -1   (BE translation: op2Sig = 8 + 8*(4-8) = -24)
/// Cross-container BE: this=(0,8,lsb=16,nb=16), op2=(2,2,lsb=3,nb=5):
///   translateLSB == 35, overlapTest == 1.
#[test]
fn verify_bitrange_overlap_matrix_big_endian_cpp_oracle() {
    let be = BitRange::with_bits(0, 4, 8, 16, true);
    assert_eq!(be.overlap_test(&BitRange::with_bits(0, 4, 0, 8, true)), -1);
    assert_eq!(be.overlap_test(&BitRange::with_bits(0, 4, 24, 8, true)), 1);
    assert_eq!(be.overlap_test(&BitRange::with_bits(0, 4, 8, 16, true)), 0);
    assert_eq!(be.overlap_test(&BitRange::with_bits(0, 4, 4, 24, true)), 2);
    assert_eq!(be.overlap_test(&BitRange::with_bits(0, 4, 12, 4, true)), 3);
    assert_eq!(be.overlap_test(&BitRange::with_bits(0, 4, 4, 8, true)), 4);
    assert_eq!(be.overlap_test(&BitRange::with_bits(4, 4, 8, 16, true)), -1);

    let big = BitRange::with_bits(0, 8, 16, 16, true);
    let small = BitRange::with_bits(2, 2, 3, 5, true);
    assert_eq!(big.translate_lsb(&small), 35);
    assert_eq!(big.overlap_test(&small), 1);
}

/// BitRange mutators (address.cc:709-868), all pinned against the C++
/// oracle as (byteOffset, byteSize, leastSigBit, numBits) tuples:
///   intersection -> empty clamps to lsb=0 nb=0
///   intersection partial (cross-container LE)
///   intersectMask minimal-cover semantics
///   shift left/right clamping at both container edges
///   truncateLeast/MostSigBytes for both endians
///   minimizeContainer for both endians
///   the recontainment constructor (BE)
#[test]
fn verify_bitrange_mutators_cpp_oracle() {
    fn tuple(b: &BitRange) -> (i32, i32, i32, i32) {
        (b.byte_offset, b.byte_size, b.least_sig_bit, b.num_bits)
    }

    // intersection going empty: op2 lives 64 bits above this range
    let mut ix = BitRange::with_bits(0, 4, 8, 8, false);
    ix.intersection(&BitRange::with_bits(8, 4, 0, 8, false));
    assert_eq!(tuple(&ix), (0, 4, 0, 0));

    // partial cross-container intersection: op2Sig = 2 + 8*(1-0) = 10
    let mut ix2 = BitRange::with_bits(0, 4, 4, 16, false);
    ix2.intersection(&BitRange::with_bits(1, 2, 2, 10, false));
    assert_eq!(tuple(&ix2), (0, 4, 10, 10));

    // intersectMask with sparse bits {3,4,5,17}: minimal cover is [3,17]
    let mut im = BitRange::with_bits(0, 4, 0, 32, false);
    im.intersect_mask(0x20038);
    assert_eq!(tuple(&im), (0, 4, 3, 15));

    // intersectMask with a mask disjoint from the range zeroes it
    let mut im2 = BitRange::with_bits(0, 4, 8, 4, false);
    im2.intersect_mask(0x1);
    assert_eq!(tuple(&im2), (0, 4, 0, 0));

    // shift clamping: at the low edge, the high edge, and through-zero
    let mut sh = BitRange::with_bits(0, 4, 8, 16, false);
    sh.shift(-12);
    assert_eq!(tuple(&sh), (0, 4, 0, 12));
    let mut sh2 = BitRange::with_bits(0, 4, 8, 16, false);
    sh2.shift(12);
    assert_eq!(tuple(&sh2), (0, 4, 20, 12));
    let mut sh3 = BitRange::with_bits(0, 4, 8, 16, false);
    sh3.shift(-30);
    assert_eq!(tuple(&sh3), (0, 4, 0, 0));

    // truncateLeastSigBytes: LE moves byteOffset, BE does not; cutting into
    // the range clamps at zero
    let mut tl = BitRange::with_bits(0, 8, 4, 16, false);
    tl.truncate_least_sig_bytes(2);
    assert_eq!(tuple(&tl), (2, 6, 0, 4));
    let mut tl2 = BitRange::with_bits(0, 8, 4, 16, true);
    tl2.truncate_least_sig_bytes(3);
    assert_eq!(tuple(&tl2), (0, 5, 0, 0));

    // truncateMostSigBytes: BE moves byteOffset, LE does not
    let mut tm = BitRange::with_bits(0, 8, 4, 16, true);
    tm.truncate_most_sig_bytes(6);
    assert_eq!(tuple(&tm), (6, 2, 4, 12));
    let mut tm2 = BitRange::with_bits(0, 8, 52, 12, false);
    tm2.truncate_most_sig_bytes(1);
    assert_eq!(tuple(&tm2), (0, 7, 52, 4));

    // minimizeContainer: trunc = lsb/8 trims opposite ends per endianness
    let mut mc = BitRange::with_bits(0, 8, 21, 7, false);
    mc.minimize_container();
    assert_eq!(tuple(&mc), (2, 2, 5, 7));
    let mut mc2 = BitRange::with_bits(0, 8, 21, 7, true);
    mc2.minimize_container();
    assert_eq!(tuple(&mc2), (4, 2, 5, 7));

    // recontainment ctor (address.cc:630-638), BE: lsb = 8 + 8*((2+8)-(4+4))
    let src = BitRange::with_bits(4, 4, 8, 12, true);
    let rc = BitRange::recontained(&src, 2, 8);
    assert_eq!(tuple(&rc), (2, 8, 24, 12));
    assert!(rc.is_big_endian);
}

/// uintb sign_extend (address.cc:906-915) and signbit_negative
/// (address.cc:880-886) with out-of-contract sizes.  The C++ size clamp
/// `sizein < sizeof(uintb)` is an int4-vs-size_t comparison (negative sizes
/// convert huge and clamp to 8), and the inner shifts go negative /
/// oversized (C++ UB) which x86 resolves by masking the count — matching
/// the port's wrapping shifts.  All rows from the C++ oracle (-O0):
#[test]
fn verify_sign_extend_sized_and_signbit_negative_cpp_oracle() {
    assert_eq!(sign_extend_sized(0x80, 1, 4), 0x00000000ffffff80);
    assert_eq!(sign_extend_sized(0x80, 1, 8), 0xffffffffffffff80);
    assert_eq!(sign_extend_sized(0x7f, 1, 3), 0x7f);
    assert_eq!(sign_extend_sized(0x8000, 2, 2), 0x8000);
    // negative sizein clamps to 8 via the huge conversion -> result 0
    assert_eq!(sign_extend_sized(0x1234, -1, 4), 0);
    // negative sizeout clamps to 8 -> plain zero-keep of the 4-byte value
    assert_eq!(sign_extend_sized(0x1234, 4, -1), 0x1234);
    // sizeout < sizein: negative shift count, x86-masked to 48 -> 0
    assert_eq!(sign_extend_sized(0xffee, 4, 2), 0);
    // sizein == 0: <<64 masks to <<0, then >>32 twice -> 0
    assert_eq!(sign_extend_sized(0xaa, 0, 4), 0);

    // signbit_negative: size 0 shifts by -8 (x86: 56) so 0x80 tests bit 63;
    // size 9 shifts by 64 (x86: 0) so the mask stays 0x80
    assert!(!signbit_negative(0x80, 0));
    assert!(signbit_negative(0x8000000000000000, 0));
    assert!(!signbit_negative(0x8000000000000000, 9));
    assert!(signbit_negative(0x80, 9));
    assert!(signbit_negative(0x80, 1));
    assert!(!signbit_negative(0x7f, 1));
}

/// RangeList::merge (address.cc:451-462) re-walks op2 through insertRange,
/// so a bridging range coalesces with both neighbours; ranges from other
/// spaces stay separate and the tree orders by (space index, first).
/// RangeList::getLastSignedRange (address.cc:562-587): the midway probe
/// returns the latest range starting in the "positive" half if one exists,
/// falls back to the latest "negative" range, and respects space identity.
/// RangeList::inRange (address.cc:468-487) does NOT chain across adjacent
/// unmerged ranges even though longestFit does.
#[test]
fn verify_rangelist_merge_last_signed_and_nonchaining_inrange() {
    let ram = Rc::new(proc_space("ram", 3, 4, 1)); // highest = 0xffffffff
    let io = Rc::new(proc_space("io", 5, 4, 1));

    // merge: [5,25] bridges [0,10] and [20,30] into [0,30]; the io range
    // stays its own element, ordered after ram (index 3 < 5)
    let mut a = RangeList::new();
    a.insert_range(Rc::clone(&ram), 0, 10);
    a.insert_range(Rc::clone(&ram), 20, 30);
    let mut b = RangeList::new();
    b.insert_range(Rc::clone(&ram), 5, 25);
    b.insert_range(Rc::clone(&io), 0, 5);
    a.merge(&b);
    assert_eq!(a.num_ranges(), 2);
    let mut s = String::new();
    a.print_bounds(&mut s);
    assert_eq!(s, "ram: 0-1e\nio: 0-5\n");

    // getLastSignedRange: a positive-half range wins over a negative one
    let mut list = RangeList::new();
    list.insert_range(Rc::clone(&ram), 0x1000, 0x1fff);
    list.insert_range(Rc::clone(&ram), 0x8000_0000, 0x8fff_ffff);
    let r = list.get_last_signed_range(&ram).unwrap();
    assert_eq!((r.get_first(), r.get_last()), (0x1000, 0x1fff));

    // ...with only a negative-half range, the second probe finds it
    let mut list = RangeList::new();
    list.insert_range(Rc::clone(&ram), 0x8000_0000, 0x8fff_ffff);
    let r = list.get_last_signed_range(&ram).unwrap();
    assert_eq!((r.get_first(), r.get_last()), (0x8000_0000, 0x8fff_ffff));

    // ...and ranges belonging to other spaces never match: surround io's
    // key window with ranges from spaces of lower and higher index
    let high = Rc::new(proc_space("high", 9, 4, 1));
    let mut list = RangeList::new();
    list.insert_range(Rc::clone(&ram), 0x1000, 0x1fff);
    list.insert_range(Rc::clone(&high), 0x1000, 0x1fff);
    assert!(list.get_last_signed_range(&io).is_none());
    assert!(RangeList::new().get_last_signed_range(&ram).is_none());

    // inRange does not chain across adjacent (unmerged) ranges; longestFit
    // does, and stops early once sizeres >= maxsize
    let mut list = RangeList::new();
    list.insert_range(Rc::clone(&ram), 0x100, 0x1ff);
    list.insert_range(Rc::clone(&ram), 0x200, 0x2ff);
    assert_eq!(list.num_ranges(), 2);
    let addr = Address::new(Rc::clone(&ram), 0x180);
    assert!(!list.in_range(&addr, 0x100)); // spans both pieces -> false
    assert!(list.in_range(&addr, 0x80)); // within the first piece -> true
    assert_eq!(list.longest_fit(&addr, 0x10000), 0x180);
    assert_eq!(list.longest_fit(&addr, 0x80), 0x80); // maxsize early break
}
