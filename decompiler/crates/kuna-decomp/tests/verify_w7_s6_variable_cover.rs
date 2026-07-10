//! Adversarial verifier tests for item `w7-s6-variable-cover` (round 1).
//!
//! These tests target the spots the hunt list flagged as most fragile for the
//! `cover.{hh,cc}` / `variable.{hh,cc}` port:
//!
//!   * `CoverBlock` two-piece (wrap-around) intersect/contain geometry — the
//!     `ustart > ustop` branches of `cover.cc:59-120` that the in-module unit
//!     tests do not cover.
//!   * `CoverBlock::merge` producing a whole-block (`setAll`) cover and the
//!     disjoint-interval branch (`cover.cc:147-184`).
//!   * `Cover::intersect_op_set` traversal with a boundary op excluded from the
//!     secondary `affectsTest` (`cover.cc:342-382`).
//!   * The `VariableGroup` duplicate-piece guard: C++ `pieceSet` is a
//!     `set<VariablePiece*, PieceCompareByOffset>` keyed on (offset,size), so
//!     `addPiece` throws `LowlevelError("Duplicate VariablePiece")` when a second
//!     piece shares an (offset,size).  The Rust `PieceKey` adds an `id` tiebreak,
//!     so the `BTreeSet` insert always succeeds and the guard never fires — see
//!     finding F1.
//!
//! Anchors: cpp `decompiler/cpp/cover.cc`, `decompiler/cpp/variable.cc`;
//! rust `decompiler/crates/kuna-decomp/src/cover.rs`, `.../variable.rs`.

use kuna_base::types::{int4, uintm};
use kuna_decomp::cover::{Cover, CoverBlock, CoverPoint, PcodeOpSet, PcodeOpSetEntry};
use kuna_decomp::context::{OpId, VarnodeId};
use kuna_decomp::variable::HighVariableBank;
use kuna_num::opcodes::OpCode;
use slotmap::KeyData;

fn op_point(uindex: uintm) -> CoverPoint {
    CoverPoint::Op {
        id: OpId::from(KeyData::from_ffi(u64::from(uindex) + 1)),
        uindex,
        code: OpCode::CPUI_COPY,
    }
}

fn block(start: uintm, stop: uintm) -> CoverBlock {
    let mut cb = CoverBlock::new();
    cb.set_begin(Some(op_point(start)));
    cb.set_end(Some(op_point(stop)));
    cb
}

/// Hunt: off-by-one / reverse-iterator analogue — the `ustart > ustop`
/// "two-piece" (wrap-around) CoverBlock, where the covered range wraps past the
/// end of the block back to the beginning (`cover.cc:117-119`, `:90-100`).
///
/// A two-piece block [stop=5 .. start=20] covers points <=5 OR >=20.  A
/// one-piece block fully inside the *gap* (6..19) must NOT intersect; one that
/// reaches into the covered region must.
#[test]
fn coverblock_two_piece_wraparound_geometry() {
    // wrap: start=20, stop=5  => ustart(20) > ustop(5) => covers p<=5 || p>=20
    let wrap = block(20, 5);
    // contain: inside covered tail
    assert!(wrap.contain(Some(op_point(3))));
    // contain: inside covered head
    assert!(wrap.contain(Some(op_point(25))));
    // NOT contained: in the gap
    assert!(!wrap.contain(Some(op_point(10))));

    // one-piece [8,15] sits entirely in the gap => no intersection (0)
    let gap = block(8, 15);
    assert_eq!(wrap.intersect(&gap), 0);
    assert_eq!(gap.intersect(&wrap), 0);

    // one-piece [3,8] overlaps the covered tail => interval intersection (2)
    let over = block(3, 8);
    assert_eq!(wrap.intersect(&over), 2);
    assert_eq!(over.intersect(&wrap), 2);

    // Two two-piece blocks always intersect as an interval (the C++
    // "If both are two-pieces, then the intersection must be an interval"
    // fall-through at cover.cc:99-101).
    let wrap2 = block(18, 7);
    assert_eq!(wrap.intersect(&wrap2), 2);
}

/// Hunt: merge / setAll boundary — `CoverBlock::merge` (`cover.cc:147-184`).
/// Two abutting/overlapping internal intervals whose union is the whole block
/// must collapse to `setAll()` (start=None, stop=End); a disjoint pair keeps the
/// earliest start and the latest stop.
#[test]
fn coverblock_merge_full_block_and_disjoint() {
    // [begin..10] merged with [10..end] => whole block.
    // begin is the (PcodeOp*)0 sentinel (set via set_all then set_end), end is
    // the (PcodeOp*)1 sentinel.
    let mut a = CoverBlock::new();
    a.set_all(); // start=None(begin), stop=End
    a.set_end(Some(op_point(10))); // now [begin .. 10]
    let mut b = CoverBlock::new();
    b.set_begin(Some(op_point(10))); // [10 .. end] (set_begin seeds stop=End)
    a.merge(&b);
    // union covers the entire block => setAll: start None, stop End
    assert_eq!(a.get_start(), None);
    assert_eq!(a.get_stop(), Some(CoverPoint::End));

    // Disjoint [5,10] U [20,25] => earliest start kept, latest stop taken.
    let mut c = block(5, 10);
    let d = block(20, 25);
    c.merge(&d);
    assert_eq!(c.get_start(), Some(op_point(5)));
    assert_eq!(c.get_stop(), Some(op_point(25)));

    // Reverse order: [20,25] merge [5,10] => start picked as the other's (5).
    let mut e = block(20, 25);
    let f = block(5, 10);
    e.merge(&f);
    assert_eq!(e.get_start(), Some(op_point(5)));
    assert_eq!(e.get_stop(), Some(op_point(25)));
}

/// A `Cover` holding exactly one block index -> the range `[start,stop]`, built
/// through the public `add_def_point` + `merge` surface (the only public Cover
/// constructors).  `add_def_point` seeds a point block `[start,start]`; merging a
/// second point cover `[stop,stop]` (disjoint, `start < stop`) collapses to the
/// `[start,stop]` interval per `CoverBlock::merge` (`cover.cc:175-180`).
fn single_block_cover(idx: int4, cb: CoverBlock) -> Cover {
    single_block_cover_range(
        idx,
        cb.get_start().expect("start"),
        cb.get_stop().expect("stop"),
    )
}

/// Hunt: iteration-order provenance + lower_bound boundary — the two-cursor
/// block walk of `Cover::intersect` / `intersect_list` visits block indices in
/// ascending order (BTreeMap == std::map).  A shared block intersecting as an
/// interval (2) short-circuits `intersect` to 2; a point-only (1) on one block
/// and interval (2) on a later block must classify each independently in
/// `intersect_list`.
#[test]
fn cover_two_cursor_walk_orders_and_classifies() {
    // Cover A: block 1 point [10,10], block 4 interval [5,20]
    let mut a = Cover::new();
    a.merge(&single_block_cover(1, block(10, 10)));
    a.merge(&single_block_cover(4, block(5, 20)));
    // Cover B: block 1 point [10,10], block 4 interval [10,30], block 9 [1,2]
    let mut b = Cover::new();
    b.merge(&single_block_cover(1, block(10, 10)));
    b.merge(&single_block_cover(4, block(10, 30)));
    b.merge(&single_block_cover(9, block(1, 2)));

    // block 1 -> point (1), block 4 -> interval (2) => intersect short-circuits 2
    assert_eq!(a.intersect(&b), 2);

    // intersect_list at level 2 lists only the interval block (4); the absent
    // block 9 (not in A) is never visited (ascending two-cursor walk).
    let mut out = Vec::new();
    a.intersect_list(&mut out, &b, 2);
    assert_eq!(out, vec![4]);
    // at level 1, both shared blocks (1 then 4) in ascending order.
    a.intersect_list(&mut out, &b, 1);
    assert_eq!(out, vec![1, 4]);
}

/// Hunt: `do {} while` at-least-once + boundary exclusion — `intersect_op_set`
/// (`cover.cc:342-382`).  An op that lands exactly on the cover's *boundary*
/// (`boundary != 0`) must be skipped for the secondary `affectsTest`; an op in
/// the interior must trigger it.
#[test]
fn intersect_op_set_skips_boundary_ops() {
    use std::cell::RefCell;
    use std::rc::Rc;

    // Cover: block 2 = [10,20].  Op at 15 is interior (boundary 0 -> tested);
    // op at 10 is the start boundary (boundary 2 -> NOT tested); op at 20 is the
    // tail (boundary 1 -> NOT tested).
    let cover = single_block_cover_range(2, op_point(10), op_point(20));

    let tested: Rc<RefCell<Vec<u64>>> = Rc::new(RefCell::new(Vec::new()));
    let tested_c = Rc::clone(&tested);
    let affects = move |op: OpId, _addr: &kuna_base::address::Address| {
        use slotmap::Key;
        tested_c.borrow_mut().push(op.data().as_ffi());
        true // claim every interior op affects => first interior op returns true
    };
    let entry = |u: uintm| PcodeOpSetEntry {
        id: OpId::from(KeyData::from_ffi(u64::from(u) + 1)),
        block_index: 2,
        point: op_point(u),
        order: u,
    };
    let populate = move || vec![entry(10), entry(15), entry(20)];
    let mut op_set = PcodeOpSet::new(Box::new(populate), Box::new(affects));
    op_set.populate();

    let rep_addr = kuna_base::address::Address::new_invalid();
    let res = cover.intersect_op_set(&op_set, &rep_addr);
    // The interior op (15) is contained and not on a boundary => affectsTest
    // fires and returns true.
    assert!(res);
    // Only the interior op (uindex 15) was ever handed to affectsTest; the
    // boundary ops (10 start, 20 tail) were contained but boundary != 0 so
    // skipped.  Compare against the slotmap `as_ffi()` of each op id (the ffi
    // round-trip stamps a version in the high bits, so we cannot use the raw
    // construction value).
    let ffi = |u: uintm| -> u64 {
        use slotmap::Key;
        OpId::from(KeyData::from_ffi(u64::from(u) + 1)).data().as_ffi()
    };
    let seen = tested.borrow();
    assert!(seen.contains(&ffi(15)), "interior op must be tested, saw {seen:?}");
    assert!(!seen.contains(&ffi(10)), "start-boundary op must be skipped");
    assert!(!seen.contains(&ffi(20)), "tail-boundary op must be skipped");
}

/// Build a one-block cover [start,stop] at the given index through the public
/// add_def_point + addRefPoint-free merge surface, by merging two point covers.
fn single_block_cover_range(idx: int4, start: CoverPoint, stop: CoverPoint) -> Cover {
    let mut out = Cover::new();
    out.add_def_point(Some((idx, start)), false);
    let mut endc = Cover::new();
    endc.add_def_point(Some((idx, stop)), false);
    out.merge(&endc);
    out
}

/// FINDING F1 (major) — the C++ `VariableGroup::pieceSet` is a
/// `set<VariablePiece*, PieceCompareByOffset>` keyed only on (offset,size), so
/// `addPiece` (`variable.cc:43-52`) throws `LowlevelError("Duplicate
/// VariablePiece")` when a second piece shares an (offset,size).  The Rust
/// `PieceKey` (`variable.rs:174-202`) appends a unique `id` to the `Ord`, so the
/// `BTreeSet::insert` ALWAYS succeeds and `group_add_piece`'s duplicate branch is
/// dead.  This test demonstrates the divergence: grouping two HighVariables at
/// the SAME offset 0 with EQUAL sizes — which fully overlap — would throw in C++
/// (`new_piece(A,0,sA,Some(X))` collides with `X@(0,sX)` when sA==sX) but the
/// Rust `group_with` returns `Ok(())`.
#[test]
fn group_with_equal_offset_size_should_error_like_cpp_but_does_not() {
    let mut bank = HighVariableBank::new();
    let a = bank.new_high(VarnodeId::from(KeyData::from_ffi(1)));
    let x = bank.new_high(VarnodeId::from(KeyData::from_ffi(2)));
    // Both pieces at offset 0 with identical size 4 => identical (offset,size)
    // key.  C++ pieceSet.insert(second).second == false => addPiece throws.
    let res = bank.group_with(a, 0, x, /*first_size=*/ 4, /*hi2_first_size=*/ 4);
    // EXPECTED (C++ parity): Err("Duplicate VariablePiece").
    // ACTUAL (Rust): Ok(()) — the id-tiebreak masks the std::set dedup, so the
    // guard never fires.  This assertion documents the live (divergent) behavior;
    // flip to `is_err()` once the (offset,size) set semantics are restored.
    assert!(
        res.is_ok(),
        "regression witness: if this now errors, the F1 duplicate-piece guard \
         was restored — update the verdict"
    );
}
