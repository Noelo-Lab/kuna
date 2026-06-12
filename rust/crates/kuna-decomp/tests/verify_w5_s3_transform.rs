//! Adversarial verifier tests for item `w5-s3-transform`
//! (port of `decompiler/cpp/transform.{cc,hh}`).
//!
//! These exercise the spots the hunt list flagged as most fragile:
//!
//!   * **`LaneDescription::getBoundary` binary search** (transform.cc:100) — the
//!     `min/max` are `int4`; on an empty/short `lanePosition` the `size()-1`
//!     underflows to `-1` and the `(min+max)/2` mid must still behave; the
//!     `bytePos == wholeSize → numLanes` branch and the non-boundary `-1` return
//!     are off-by-one prone.
//!   * **`LaneDescription::subset`** (transform.cc:72) — clears `lanePosition`
//!     *before* the rebuild loop while still reading the *old* `laneSize[i]`; a
//!     transposed clear or an aliasing slip silently corrupts the trimmed lanes.
//!   * **`restriction` / `extension`** (transform.cc:133/158) — the `+bytePos`
//!     vs `-bytePos` sign is the only difference between the two; a copy-paste
//!     would invert it.
//!   * **`LanedRegister::parseSizes` + `LanedIterator::normalize`**
//!     (transform.cc:300/284) — comma tokenization (trailing-comma empty token →
//!     parse-fail → throw), the `[0,16]` range gate, and `normalize` skipping to
//!     the lowest *set* mask bit (not bit 0).
//!   * **`pieceMap` ascending-create-index visit order** (transform.cc:684) —
//!     the C++ `map<int4,...>` walks pieces in ascending big-Varnode create-index
//!     order regardless of *insertion* order; `apply()`'s materialization
//!     allocation order (downstream-observable) depends on it.
//!   * **`newConstant` masking** (transform.cc:399) — `(val >> lsbOffset) &
//!     calc_mask(size)`, with an offset that strips past the value's precision.
//!
//! C++ oracle anchors are cited inline. The transform module's struct fields are
//! private, so these drive only the public factory/query/apply surface (the same
//! surface `LaneDivide`/`SplitFlow` use).

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, IopSpace, UniqueSpace,
};
use kuna_base::types::int4;

use kuna_decomp::dtype::{type_metatype, Datatype};
use kuna_decomp::funcdata::Funcdata;
use kuna_decomp::seams::{Architecture, VarnodeId};
use kuna_decomp::transform::{
    tvar_flags, tvar_type, LaneDescription, LanedRegister, TVarRef, TransformManager,
};

// --- harness (mirrors the in-crate transform test harness) --------------------

fn build_manager() -> AddrSpaceManager {
    let mut m = AddrSpaceManager::new();
    m.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    m.insert_space(Rc::new(UniqueSpace::new(1, 0, false))).unwrap();
    m.insert_space(Rc::new(IopSpace::new(2))).unwrap();
    m.insert_space(Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "ram",
        false,
        8,
        1,
        3,
        addrspace_flags::hasphysical,
        1,
        1,
    )))
    .unwrap();
    m
}

fn build_fd() -> Funcdata {
    let manage = build_manager();
    let glb = Rc::new(Architecture::new(manage));
    let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
    let addr = Address::new(ram, 0x1000);
    Funcdata::new("func", "func", glb, addr, 0x10000000, 0x40).unwrap()
}

fn ram_space(fd: &Funcdata) -> Rc<AddrSpace> {
    Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap())
}

fn dt(size: int4) -> Rc<Datatype> {
    Rc::new(Datatype::new(size, type_metatype::TYPE_UNKNOWN))
}

fn mk_vn(fd: &mut Funcdata, off: u64, s: int4) -> VarnodeId {
    let ram = ram_space(fd);
    fd.vbank_mut().create(s, Address::new(ram, off), dt(s))
}

fn mk_const(fd: &mut Funcdata, val: u64, s: int4) -> VarnodeId {
    fd.new_constant(s, val)
}

// =============================================================================
// F-area 1: getBoundary binary search corner cases (transform.cc:100)
// =============================================================================

#[test]
fn w5s3_get_boundary_single_lane_and_whole_size() {
    // A one-lane description: lanePosition = [0], wholeSize = sz.
    // max = lanePosition.size()-1 = 0, so the binary search has a 1-element range.
    let d = LaneDescription::uniform(4, 4); // 1 lane of 4 bytes at pos 0
    assert_eq!(d.get_num_lanes(), 1);
    assert_eq!(d.get_boundary(0), 0); // start of the only lane
    assert_eq!(d.get_boundary(4), 1); // == wholeSize → numLanes (==1)
    assert_eq!(d.get_boundary(2), -1); // interior, not a boundary
    assert_eq!(d.get_boundary(-1), -1); // bytePos < 0
    assert_eq!(d.get_boundary(5), -1); // bytePos > wholeSize
}

#[test]
fn w5s3_get_boundary_non_uniform_two_lane() {
    // two_lane(8, lo=3, hi=5): positions [0,3], wholeSize 8.
    // Exercises the `pos < bytePos` branch landing exactly on the 2nd lane.
    let d = LaneDescription::two_lane(8, 3, 5);
    assert_eq!(d.get_boundary(0), 0);
    assert_eq!(d.get_boundary(3), 1);
    assert_eq!(d.get_boundary(8), 2); // wholeSize → numLanes
    // Positions inside either lane are not boundaries.
    assert_eq!(d.get_boundary(1), -1);
    assert_eq!(d.get_boundary(2), -1);
    assert_eq!(d.get_boundary(4), -1);
    assert_eq!(d.get_boundary(7), -1);
}

// =============================================================================
// F-area 2: subset clear-then-rebuild aliasing (transform.cc:72)
// =============================================================================

#[test]
fn w5s3_subset_trims_suffix_and_rebases_positions() {
    // 16-byte whole, 4-byte lanes → positions 0,4,8,12.
    // Trim to the last two lanes: lsbOffset 8, size 8 → lanes 2,3 rebased to 0,4.
    // This stresses that the loop reads OLD laneSize[i] AFTER lanePosition.clear().
    let mut d = LaneDescription::uniform(16, 4);
    assert!(d.subset(8, 8));
    assert_eq!(d.get_whole_size(), 8);
    assert_eq!(d.get_num_lanes(), 2);
    assert_eq!(d.get_size(0), 4);
    assert_eq!(d.get_size(1), 4);
    assert_eq!(d.get_position(0), 0); // rebased from 8
    assert_eq!(d.get_position(1), 4); // rebased from 12
}

#[test]
fn w5s3_subset_prefix_single_lane() {
    // Trim to just the first lane (offset 0, size 4 of an 16/4 desc).
    let mut d = LaneDescription::uniform(16, 4);
    assert!(d.subset(0, 4));
    assert_eq!(d.get_whole_size(), 4);
    assert_eq!(d.get_num_lanes(), 1);
    assert_eq!(d.get_position(0), 0);
    assert_eq!(d.get_size(0), 4);
}

#[test]
fn w5s3_subset_partial_intersection_fails_unmodified_check() {
    // offset 2 is interior to lane 0 (positions 0,4,...) → false.
    let mut d = LaneDescription::uniform(16, 4);
    assert!(!d.subset(2, 8));
    // And a size that ends interior to a lane also fails.
    let mut d2 = LaneDescription::uniform(16, 4);
    assert!(!d2.subset(0, 6));
}

// =============================================================================
// F-area 3: restriction vs extension sign (transform.cc:133 vs 158)
// =============================================================================

#[test]
fn w5s3_restriction_and_extension_differ_only_in_sign() {
    // positions 0,2,4,6 ; wholeSize 8.
    let d = LaneDescription::uniform(8, 2);

    // restriction: ADD bytePos. skip=0 (pos 0), bytePos=2 → resSkip from pos 2 = lane 1.
    // size=4 → final from pos 2+4=6 = lane 3 → resNum = 3-1 = 2.
    assert_eq!(d.restriction(4, 0, 2, 4), Some((2, 1)));

    // extension: SUBTRACT bytePos. skip=2 (pos 4), bytePos=2 → resSkip from pos 4-2=2 = lane 1.
    // size=4 → final from pos 2+4=6 = lane 3 → resNum = 2.
    assert_eq!(d.extension(4, 2, 2, 4), Some((2, 1)));

    // The sign genuinely matters: at skip=0, extension would compute pos 0-2 = -2 → -1 → None,
    // whereas restriction at skip=0 lands inside.
    assert_eq!(d.extension(4, 0, 2, 4), None);
    // A restriction whose truncation splits a lane (size 3 from a boundary) → None.
    assert_eq!(d.restriction(4, 0, 2, 3), None);
    // A zero-lane result (resNumLanes == 0) → None even though both boundaries are valid.
    // skip=0 pos 0, bytePos 0, size 0 → resSkip lane 0, final lane 0 → resNum 0 → None.
    assert_eq!(d.restriction(4, 0, 0, 0), None);
}

// =============================================================================
// F-area 4: LanedRegister parse + iterator normalize (transform.cc:300/284)
// =============================================================================

#[test]
fn w5s3_parse_sizes_range_gate_and_trailing_comma() {
    // Valid list.
    let mut lr = LanedRegister::new();
    lr.parse_sizes(8, "1,2,4").unwrap();
    assert!(lr.allowed_lane(1) && lr.allowed_lane(2) && lr.allowed_lane(4));
    assert!(!lr.allowed_lane(3));

    // Boundary value 16 is allowed; 17 throws.
    let mut ok16 = LanedRegister::new();
    assert!(ok16.parse_sizes(20, "16").is_ok());
    let mut bad17 = LanedRegister::new();
    assert!(bad17.parse_sizes(20, "17").is_err());

    // 0 is allowed by the [0,16] gate (C++ `sz < 0` is the lower bound).
    let mut zero = LanedRegister::new();
    assert!(zero.parse_sizes(8, "0").is_ok());

    // Trailing comma → empty final token → parse fails → sz stays -1 → throw.
    let mut trail = LanedRegister::new();
    assert!(trail.parse_sizes(8, "1,").is_err());

    // A leading empty token (",1") is likewise a failed parse on the first token.
    let mut lead = LanedRegister::new();
    assert!(lead.parse_sizes(8, ",1").is_err());
}

#[test]
fn w5s3_iter_normalize_skips_to_lowest_set_bit() {
    // Only bit 5 set: normalize() must advance `size` 0→5 before yielding.
    let lr = LanedRegister::with_mask(32, 1u32 << 5);
    let sizes: Vec<int4> = lr.iter_sizes().collect();
    assert_eq!(sizes, vec![5]);

    // Sparse mask: bits 1 and 7 → yields 1 then 7 (skipping the gap).
    let lr2 = LanedRegister::with_mask(32, (1u32 << 1) | (1u32 << 7));
    let sizes2: Vec<int4> = lr2.iter_sizes().collect();
    assert_eq!(sizes2, vec![1, 7]);

    // bit 0 set (lane size 0) must be yielded, not skipped.
    let lr3 = LanedRegister::with_mask(8, 1u32 | (1u32 << 3));
    let sizes3: Vec<int4> = lr3.iter_sizes().collect();
    assert_eq!(sizes3, vec![0, 3]);
}

// =============================================================================
// F-area 5: pieceMap ascending-create-index visit order (transform.cc:684)
// =============================================================================

#[test]
fn w5s3_apply_visits_pieces_in_create_index_order_not_insertion_order() {
    // Two constant varnodes; split the HIGHER-create-index one FIRST, then the
    // lower one. The C++ pieceMap (map<int4,...>) is keyed by big-Varnode
    // create-index, so apply() must materialize the LOWER-create-index split's
    // lanes first regardless of the insertion order — a HashMap would not.
    let mut fd = build_fd();
    let lo_vn = mk_const(&mut fd, 0x11223344, 4); // created first → lower create-index
    let hi_vn = mk_const(&mut fd, 0x55667788, 4); // created second → higher create-index
    let lo_ci = fd.vbank().get(lo_vn).unwrap().get_create_index() as int4;
    let hi_ci = fd.vbank().get(hi_vn).unwrap().get_create_index() as int4;
    assert!(lo_ci < hi_ci);

    let mut tm = TransformManager::new();
    let desc = LaneDescription::uniform(4, 2);
    // Insert the HIGH one first, the LOW one second (reverse of create-index order).
    tm.new_split(&fd, hi_vn, &desc);
    tm.new_split(&fd, lo_vn, &desc);

    tm.apply(&mut fd).expect("constant-only split apply should succeed (no seams)");

    let lo_lane0 = TVarRef::Piece { key: lo_ci, idx: 0 };
    let hi_lane0 = TVarRef::Piece { key: hi_ci, idx: 0 };
    let lo_repl = tm.var(lo_lane0).get_replacement().unwrap();
    let hi_repl = tm.var(hi_lane0).get_replacement().unwrap();
    let lo_repl_ci = fd.vbank().get(lo_repl).unwrap().get_create_index();
    let hi_repl_ci = fd.vbank().get(hi_repl).unwrap().get_create_index();
    // The lower-keyed split's lanes were materialized first → lower replacement
    // create-index, matching map<int4,...> ascending iteration.
    assert!(
        lo_repl_ci < hi_repl_ci,
        "pieceMap walked in ascending create-index order: lo={lo_repl_ci} hi={hi_repl_ci}"
    );
}

// =============================================================================
// F-area 6: newConstant masking past precision (transform.cc:399)
// =============================================================================

#[test]
fn w5s3_new_constant_masks_and_strips_beyond_precision() {
    let mut tm = TransformManager::new();

    // size 1 (mask 0xff), lsb_offset 0 → keeps the low byte only.
    let a = tm.new_constant(1, 0, 0xDEAD_BEEF);
    assert_eq!(tm.var(a).get_val(), 0xEF);

    // size 2 (mask 0xffff), lsb_offset 16 → (v >> 16) & 0xffff = 0xDEAD.
    let b = tm.new_constant(2, 16, 0xDEAD_BEEF);
    assert_eq!(tm.var(b).get_val(), 0xDEAD);

    // lsb_offset that strips PAST all the bits → 0.
    let c = tm.new_constant(2, 48, 0x0000_BEEF);
    assert_eq!(tm.var(c).get_val(), 0);

    // size 8 (full width mask), no strip → whole value preserved.
    let d = tm.new_constant(8, 0, 0xFEDC_BA98_7654_3210);
    assert_eq!(tm.var(d).get_val(), 0xFEDC_BA98_7654_3210);
}

// =============================================================================
// F-area 7: preexistingGuard truth table (transform.hh:246)
// =============================================================================

#[test]
fn w5s3_preexisting_guard_full_truth_table() {
    let mut fd = build_fd();
    let vn = mk_vn(&mut fd, 0x200, 8);
    let mut tm = TransformManager::new();
    let piece = tm.new_piece(&fd, vn, 16, 0); // piece (preserveAddress true on ram)
    let uniq = tm.new_unique(4); // normal_temp
    let konst = tm.new_constant(4, 0, 0x10); // constant

    // slot 0: always build.
    assert!(tm.preexisting_guard(0, piece));
    assert!(tm.preexisting_guard(0, uniq));
    assert!(tm.preexisting_guard(0, konst));

    // slot != 0: build iff the OTHER input is NOT a piece/piece_temp.
    assert!(!tm.preexisting_guard(1, piece)); // piece → don't build (visited on slot 0)
    assert!(tm.preexisting_guard(1, uniq)); // non-piece → build
    assert!(tm.preexisting_guard(1, konst)); // constant → build

    // A piece_temp also suppresses (unique-space piece is piece_temp).
    let upiece = {
        let uvn = fd.vbank_mut().create_unique(8, dt(8));
        tm.new_piece(&fd, uvn, 16, 0)
    };
    assert_eq!(tm.var(upiece).get_type(), tvar_type::piece_temp);
    assert!(!tm.preexisting_guard(1, upiece));
}

// =============================================================================
// F-area 8: input_duplicate handling (transform.cc:684) — second input marked
// =============================================================================

#[test]
fn w5s3_split_terminator_marks_only_last_lane() {
    // Defensive: the split_terminator flag (which bounds createVarnodes' inner
    // loop) must be on the LAST lane only, for every lane count.
    let mut fd = build_fd();
    let vn = mk_vn(&mut fd, 0x300, 8);
    let ci = fd.vbank().get(vn).unwrap().get_create_index() as int4;
    let mut tm = TransformManager::new();
    let desc = LaneDescription::uniform(8, 2); // 4 lanes
    tm.new_split(&fd, vn, &desc);
    for idx in 0..4usize {
        let r = TVarRef::Piece { key: ci, idx };
        let is_term = (tm.var(r).get_flags() & tvar_flags::split_terminator) != 0;
        assert_eq!(is_term, idx == 3, "only lane 3 is the terminator (idx={idx})");
    }
}
