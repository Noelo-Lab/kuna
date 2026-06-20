//! Round-2 verifier adversarial tests for item `w7-s6-varmap`.
//!
//! Round 1 REJECTed on F1 (the `sstart` scale chain ended in `byteToAddressInt`
//! (divide) instead of the C++ `addressToByte` (multiply), `varmap.cc:906`/`:1073`).
//! The porter's repair commit (`f4de4e1`) switched both call sites to
//! `address_to_byte_int`.  These round-2 tests re-derive that fix INDEPENDENTLY
//! against the C++ oracle, and additionally exercise the F2 site (round-1 minor)
//! that the repair did NOT touch, so the verdict carries a live reproduction.
//!
//! Coverage targets (hunt-list spots, distinct from the round-1 file):
//!   * F1 reached via the REAL driving path `gather_symbols` (not the test-only
//!     `add_range_pub`), with word-size 2 — confirms the multiply is in the
//!     production code path, not just patched at one entry.
//!   * F1 in `initialize`'s terminating-endpoint sstart (varmap.cc:1073) with
//!     word-size 2 — the second of the two repaired sites.
//!   * the combined sign_extend+multiply chain on a NEGATIVE word-addressed
//!     stack offset feeding the `compare` sort, end-to-end through `initialize`.
//!   * F2 (minor, still open): `merge` resType==2 grows `size` with a bare
//!     `int4 + int4` (`diff + b.size`, varmap.rs:495-496) where the C++ relies on
//!     silent signed wrap.  A frame whose start distance truncates near i32::MAX
//!     makes the bare `+` overflow — a debug panic in Rust, a wrap in C++.

use std::rc::Rc;

use kuna_base::address::RangeList;
use kuna_base::space::{addrspace_flags, spacetype, AddrSpace};

use kuna_decomp::dtype::{type_metatype, Datatype, TypeFactory, TypeFactoryImpl};
use kuna_decomp::varmap::{MapState, RangeHint, RangeType};

fn space_ws(ws: u32) -> Rc<AddrSpace> {
    Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "stack",
        false,
        8, // address size
        ws,
        4,
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
// F1 (repaired): the multiply must be reached through the REAL driving path.
//
// `add_range` is private; production code reaches it via `gather_symbols`
// (varmap.cc:1058 -> addRange) and `addFixedType`, NOT the test-only
// `add_range_pub`.  Drive `gather_symbols` with ws=2 and assert the C++
// oracle value (addressToByte(byteToAddress(st,2),2) = round-to-word).
//
// Oracle for st=0x20, ws=2:  byteToAddress = 0x20/2 = 0x10; sign_extend(+)=0x10;
//   addressToByte = 0x10*2 = 0x20.   The pre-repair divide gave (0x10)/2 = 0x8.
// ===========================================================================
#[test]
fn verify_r2_gather_symbols_sstart_multiply_ws2() {
    let space = space_ws(2);
    let factory = factory();
    let dflt = factory.get_base(1, type_metatype::TYPE_UNKNOWN).unwrap();
    let rn = window(&space, 0, 0x100);
    let pm = RangeList::new();
    let mut state = MapState::new(Rc::clone(&space), &rn, &pm, dflt);

    // gatherSymbols path: (start_offset, type, type_locked).
    let syms = [(0x20u64, base(4, type_metatype::TYPE_INT), false)];
    state.gather_symbols(&syms);
    assert!(state.initialize().unwrap());

    assert_eq!(
        state.next().sstart,
        0x20,
        "gather_symbols -> add_range sstart chain must end in addressToByte \
         (multiply); C++ varmap.cc:906 gives 0x20, a divide would give 0x8"
    );
}

// ===========================================================================
// F1 (repaired) at the SECOND site: the terminating endpoint built in
// `MapState::initialize` (varmap.cc:1071-1073) also runs the sstart chain.
// With ws=2 and a window last = 0xFE, high = wrapOffset(0xFE+1) = 0xFF;
//   byteToAddress = 0xFF/2 = 0x7F; sign_extend(+) = 0x7F;
//   addressToByte = 0x7F*2 = 0xFE.   Pre-repair divide: 0x7F/2 = 0x3F.
// The endpoint is the LAST hint after the stable_sort (largest sstart), so we
// can read it off the tail of the iteration.
// ===========================================================================
#[test]
fn verify_r2_initialize_endpoint_sstart_multiply_ws2() {
    let space = space_ws(2);
    let factory = factory();
    let dflt = factory.get_base(1, type_metatype::TYPE_UNKNOWN).unwrap();
    // window last = 0xFE so high = 0xFF.
    let rn = window(&space, 0, 0xFE);
    let pm = RangeList::new();
    let mut state = MapState::new(Rc::clone(&space), &rn, &pm, dflt);

    // One ordinary hint well below the endpoint so the endpoint sorts last.
    state.gather_symbols(&[(0x10u64, base(2, type_metatype::TYPE_INT), false)]);
    assert!(state.initialize().unwrap());

    // Walk to the final (endpoint) hint.
    let mut last_sstart = state.next().sstart;
    let mut last_start = state.next().start;
    while state.get_next() {
        last_sstart = state.next().sstart;
        last_start = state.next().start;
    }
    assert_eq!(last_start, 0xFF, "endpoint start is wrapOffset(last+1)=0xFF");
    assert_eq!(
        last_sstart, 0xFE,
        "endpoint sstart chain must multiply by word-size: C++ (0xFF/2)*2 = 0xFE; \
         a divide would give 0x3F"
    );
}

// ===========================================================================
// Combined sign_extend + multiply on a NEGATIVE word-addressed offset, driven
// end-to-end through `initialize` so the sort key is the real recovered one.
//
// addrSize=8 so sign_extend at bit 63 is a no-op on a u64 already in i64 range;
// to get a negative sstart we use a high byte offset whose /ws value's top bit,
// after the addrSize-1 sign extension, is set.  Here we instead pin the simpler,
// fully-determined property: with ws=2, two hints at bytes 0x10 and 0x40 must
// sort by their MULTIPLIED sstarts (0x10 < 0x40), i.e. the relative order is
// preserved and the absolute keys are the word-rounded byte offsets — a divide
// would still preserve order here but produce the WRONG absolute keys (0x8,0x20)
// that later size/contain math reads.  We assert the absolute keys.
// ===========================================================================
#[test]
fn verify_r2_initialize_sorts_on_multiplied_keys_ws2() {
    let space = space_ws(2);
    let factory = factory();
    let dflt = factory.get_base(1, type_metatype::TYPE_UNKNOWN).unwrap();
    let rn = window(&space, 0, 0x100);
    let pm = RangeList::new();
    let mut state = MapState::new(Rc::clone(&space), &rn, &pm, dflt);

    state.gather_symbols(&[
        (0x40u64, base(2, type_metatype::TYPE_INT), false),
        (0x10u64, base(2, type_metatype::TYPE_INT), false),
    ]);
    assert!(state.initialize().unwrap());

    // Collect the sstart keys in sorted order (excluding the endpoint tail).
    let mut keys = vec![state.next().sstart];
    while state.get_next() {
        keys.push(state.next().sstart);
    }
    // First two keys are the two hints (sorted ascending), then the endpoint.
    assert_eq!(keys[0], 0x10, "first hint key is word-rounded byte 0x10");
    assert_eq!(keys[1], 0x40, "second hint key is word-rounded byte 0x40");
    // Monotone non-decreasing (compare's primary key is sstart).
    assert!(keys.windows(2).all(|w| w[0] <= w[1]), "initialize sort is by sstart");
}

// ===========================================================================
// F2 (minor, STILL OPEN after the repair): `merge` resType==2 size growth.
//
// C++ varmap.cc:301-303:
//     int4 diff = (int4)(b->sstart - sstart);
//     if (diff + b->size > size)            // int4 + int4, silent signed wrap
//       size = diff + b->size;
// Rust varmap.rs:494-497 transcribes the `diff` truncation with `wsub ... as
// int4` (correct), but the subsequent `diff + b.size` and `diff + b.size` are
// BARE `int4 + int4`.  Per ADR 0003 a legitimately-wrapping C++ op must be a
// `w*` call; a bare `+` is allowed only where overflow is a genuine bug.  Here
// the C++ wraps on purpose (the "confuse" path tolerates garbage), so the bare
// `+` is a latent debug-build panic / release divergence.
//
// Construct the resType==2 branch with a start distance that truncates to a
// near-i32::MAX `diff`, so `diff + b.size` overflows i32.
//   - a: unlocked, sstart 0, size 4   -> not type-locked
//   - b: sstart 0x7FFF_FFFE, size 4    -> diff = (int4)0x7FFF_FFFE = i32::MAX-1
// contain(b): sstart != b.sstart, and b_end (0x7FFF_FFFE+4-1) > a_end (0+4-1),
// so contain is false -> resType = (a locked? 0 : 2) = 2.
// Then `diff + b.size` = (i32::MAX-1) + 4  ==> i32 overflow.
//
// In C++ this wraps to a large negative and the `> size` test is false, so size
// is left at 4 and (4 is a power of two) the range stays fixed UNKNOWN(4).  The
// faithful Rust port must reproduce that (no panic, size stays 4).  A bare `+`
// panics in this debug test build instead -> the test FAILS, pinning F2.
// ===========================================================================
#[test]
fn verify_r2_merge_confuse_diff_plus_size_wraps_like_cpp() {
    let space = space_ws(1);
    let factory = factory();

    let mut a = RangeHint::new(0, 4, 0, base(4, type_metatype::TYPE_INT), 0, RangeType::Fixed, -1);
    // sstart chosen so (int4)b.sstart == i32::MAX - 1; size 4 pushes the add over.
    let b_sstart: i64 = 0x7FFF_FFFE;
    let b = RangeHint::new(
        b_sstart as u64,
        4,
        b_sstart,
        base(4, type_metatype::TYPE_UINT),
        0,
        RangeType::Fixed,
        -1,
    );

    // C++ oracle: diff = i32::MAX-1; diff + 4 wraps to a negative int4, which is
    // NOT > size(4), so size is unchanged (4).  4 is a power of two, so the range
    // stays Fixed with UNKNOWN(4).  A faithful port must NOT panic here.
    let ret = a.merge(&b, &space, &factory).unwrap();
    assert!(!ret, "merge always returns false on the resType==2 path");
    assert_eq!(
        a.size, 4,
        "C++ silent wrap of (i32::MAX-1)+4 is negative, so `> size` is false and \
         size stays 4; a bare Rust `+` would panic in debug instead"
    );
    assert_eq!(a.range_type, RangeType::Fixed, "size 4 is a power of two: stays fixed");
    assert_eq!(a.type_.get_metatype(), type_metatype::TYPE_UNKNOWN);
}
