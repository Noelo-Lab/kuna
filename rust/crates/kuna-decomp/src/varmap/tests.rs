//! Unit tests for `varmap`: the order-determining stack-layout-recovery core.
//!
//! These exercise the parts the porting item flags as output-determining — the
//! `RangeHint` ordering and merge logic, `MapState`'s sort + `reconcileDatatypes`,
//! and `AliasChecker`'s offset gather / alias sort / boundary derivation — on
//! synthetic frames with known-answer assertions transcribed against the C++.

use std::rc::Rc;

use kuna_base::address::{Address, RangeList};
use kuna_base::space::{addrspace_flags, spacetype, AddrSpace};

use crate::dtype::{type_metatype, Datatype, TypeFactoryImpl};

use super::*;

// --- Builders -------------------------------------------------------------

/// A processor space with word-size 1 (so byte<->address is the identity) and
/// 8-byte addresses, matching the synthetic "stack" used below.
fn stack_space() -> Rc<AddrSpace> {
    Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "stack",
        false, // little-endian
        8,     // address size
        1,     // word size
        4,     // index
        addrspace_flags::hasphysical,
        1,
        1,
    ))
}

/// A base data-type of the given size and meta-type (align-size == size).
fn base(size: int4, meta: type_metatype) -> Rc<Datatype> {
    Rc::new(Datatype::new_with_align(size, size, meta))
}

/// A usable type factory (default alignment map, base-type cap of 8), matching
/// the dtype-module test helper.
fn factory() -> TypeFactoryImpl {
    let f = TypeFactoryImpl::new();
    f.set_default_alignment_map();
    f.set_max_basetype_size(8);
    f
}

/// A fixed-size hint at the given signed/unsigned start with a given type.
fn hint(start: uintb, size: int4, ct: Rc<Datatype>, rt: RangeType) -> RangeHint {
    // For these synthetic frames word-size is 1, so sstart == start treated as
    // signed; callers pass small positive offsets where sstart == start.
    RangeHint::new(start, size, start as intb, ct, 0, rt, -1)
}

// --- RangeHint::compare (the stable_sort comparator) ----------------------

#[test]
fn compare_orders_by_signed_start_first() {
    let a = hint(0x10, 4, base(4, type_metatype::TYPE_INT), RangeType::Fixed);
    let b = hint(0x20, 4, base(4, type_metatype::TYPE_INT), RangeType::Fixed);
    assert_eq!(a.compare(&b), -1);
    assert_eq!(b.compare(&a), 1);
}

#[test]
fn compare_small_sizes_come_first() {
    // Same start: smaller size sorts first (varmap.cc:326-327).
    let small = hint(0x10, 2, base(2, type_metatype::TYPE_INT), RangeType::Fixed);
    let big = hint(0x10, 8, base(8, type_metatype::TYPE_INT), RangeType::Fixed);
    assert_eq!(small.compare(&big), -1);
    assert_eq!(big.compare(&small), 1);
}

#[test]
fn compare_rangetype_breaks_size_ties() {
    // Same start+size: fixed(0) < open(1) < endpoint(2).
    let fixed = hint(0x10, 4, base(4, type_metatype::TYPE_INT), RangeType::Fixed);
    let open = hint(0x10, 4, base(4, type_metatype::TYPE_INT), RangeType::Open);
    assert_eq!(fixed.compare(&open), -1);
    assert_eq!(open.compare(&fixed), 1);
}

#[test]
fn compare_flags_then_highind_break_remaining_ties() {
    let mut a = hint(0x10, 4, base(4, type_metatype::TYPE_INT), RangeType::Fixed);
    let mut b = hint(0x10, 4, base(4, type_metatype::TYPE_INT), RangeType::Fixed);
    a.flags = 0;
    b.flags = TYPELOCK;
    assert_eq!(a.compare(&b), -1); // lower flags first

    a.flags = TYPELOCK;
    a.highind = 1;
    b.highind = 3;
    assert_eq!(a.compare(&b), -1); // lower highind first
    a.highind = 3;
    assert_eq!(a.compare(&b), 0); // fully equal — datatype is NOT compared
}

#[test]
fn compare_ignores_datatype() {
    // Identical key fields, different data-types: compare() returns 0.
    let a = hint(0x10, 4, base(4, type_metatype::TYPE_INT), RangeType::Fixed);
    let b = hint(0x10, 4, base(4, type_metatype::TYPE_UINT), RangeType::Fixed);
    assert_eq!(a.compare(&b), 0);
}

#[test]
fn compare_ranges_is_a_strict_weak_ordering_used_by_sort() {
    let mut v = [
        hint(0x30, 4, base(4, type_metatype::TYPE_INT), RangeType::Fixed),
        hint(0x10, 8, base(8, type_metatype::TYPE_INT), RangeType::Fixed),
        hint(0x10, 4, base(4, type_metatype::TYPE_INT), RangeType::Fixed),
        hint(0x20, 4, base(4, type_metatype::TYPE_INT), RangeType::Fixed),
    ];
    v.sort_by(RangeHint::compare_ranges);
    let starts: Vec<(uintb, int4)> = v.iter().map(|h| (h.start, h.size)).collect();
    // start ascending; within 0x10 the size-4 hint precedes the size-8 hint.
    assert_eq!(starts, vec![(0x10, 4), (0x10, 8), (0x20, 4), (0x30, 4)]);
}

// --- contain / isConstAbsorbable / reconcile ------------------------------

#[test]
fn contain_same_start_or_enclosed() {
    let a = hint(0x10, 8, base(8, type_metatype::TYPE_INT), RangeType::Fixed);
    let same_start = hint(0x10, 2, base(2, type_metatype::TYPE_INT), RangeType::Fixed);
    let enclosed = hint(0x12, 2, base(2, type_metatype::TYPE_INT), RangeType::Fixed);
    let overhang = hint(0x14, 8, base(8, type_metatype::TYPE_INT), RangeType::Fixed);
    assert!(a.contain(&same_start)); // sstart == sstart
    assert!(a.contain(&enclosed)); // b end <= a end
    assert!(!a.contain(&overhang)); // b extends past a
}

#[test]
fn const_absorbable_requires_copy_constant_and_primitive() {
    // `this` is an open INT; `b` is a copy-constant of >= size with unknown type.
    let mut this = hint(0x10, 4, base(4, type_metatype::TYPE_INT), RangeType::Open);
    this.highind = -1;
    let mut b = hint(0x10, 4, base(4, type_metatype::TYPE_UNKNOWN), RangeType::Fixed);
    b.flags = COPY_CONSTANT;
    assert!(this.is_const_absorbable(&b));

    // Without copy_constant: not absorbable.
    b.flags = 0;
    assert!(!this.is_const_absorbable(&b));

    // A struct `this` is not a primitive, so never const-absorbs.
    let s = hint(0x10, 4, base(4, type_metatype::TYPE_STRUCT), RangeType::Open);
    let mut bc = hint(0x10, 4, base(4, type_metatype::TYPE_UNKNOWN), RangeType::Fixed);
    bc.flags = COPY_CONSTANT;
    assert!(!s.is_const_absorbable(&bc));
}

#[test]
fn reconcile_equal_sizes_line_up() {
    // Two 4-byte ints at the same start reconcile (align-sizes equal → true).
    let a = hint(0x10, 4, base(4, type_metatype::TYPE_INT), RangeType::Fixed);
    let b = hint(0x10, 4, base(4, type_metatype::TYPE_INT), RangeType::Fixed);
    assert!(a.reconcile(&b).unwrap());
}

#[test]
fn reconcile_typelocked_mismatch_fails() {
    // A 4-byte int containing a 2-byte type-locked int at an offset that does
    // not divide into a matching component → not reconcilable.
    let a = hint(0x10, 4, base(4, type_metatype::TYPE_INT), RangeType::Fixed);
    let mut b = hint(0x11, 2, base(2, type_metatype::TYPE_INT), RangeType::Fixed);
    b.sstart = 0x11;
    b.flags = TYPELOCK;
    assert!(!a.reconcile(&b).unwrap());
}

// --- preferred ------------------------------------------------------------

#[test]
fn preferred_different_start_always_true() {
    let a = hint(0x10, 4, base(4, type_metatype::TYPE_INT), RangeType::Fixed);
    let b = hint(0x14, 4, base(4, type_metatype::TYPE_INT), RangeType::Fixed);
    assert!(a.preferred(&b, false).unwrap());
}

#[test]
fn preferred_locked_beats_unlocked() {
    let mut a = hint(0x10, 4, base(4, type_metatype::TYPE_INT), RangeType::Fixed);
    let b = hint(0x10, 4, base(4, type_metatype::TYPE_INT), RangeType::Fixed);
    a.flags = TYPELOCK;
    // a is locked, b is not → a is preferred.
    assert!(a.preferred(&b, true).unwrap());
    // Flip: now b is locked, a is not → a is NOT preferred.
    let mut a2 = hint(0x10, 4, base(4, type_metatype::TYPE_INT), RangeType::Fixed);
    let mut b2 = hint(0x10, 4, base(4, type_metatype::TYPE_INT), RangeType::Fixed);
    b2.flags = TYPELOCK;
    a2.flags = 0;
    assert!(!a2.preferred(&b2, true).unwrap());
}

#[test]
fn preferred_fixed_bigger_size_wins_without_reconcile() {
    // Two fixed ranges, same start, unreconciled, different size → bigger wins.
    let big = hint(0x10, 8, base(8, type_metatype::TYPE_INT), RangeType::Fixed);
    let small = hint(0x10, 4, base(4, type_metatype::TYPE_INT), RangeType::Fixed);
    assert!(big.preferred(&small, false).unwrap());
    assert!(!small.preferred(&big, false).unwrap());
}

#[test]
fn preferred_open_thrown_out_when_not_reconciled() {
    // a is open, b is fixed, not reconciled → open is thrown out (a not preferred).
    let open = hint(0x10, 4, base(4, type_metatype::TYPE_INT), RangeType::Open);
    let fixed = hint(0x10, 4, base(4, type_metatype::TYPE_INT), RangeType::Fixed);
    assert!(!open.preferred(&fixed, false).unwrap());
    // Reverse roles: a fixed, b open, not reconciled → a is preferred.
    assert!(fixed.preferred(&open, false).unwrap());
}

// --- absorb / attemptJoin -------------------------------------------------

#[test]
fn absorb_extends_open_array_highind() {
    // An open int array [0..0] absorbs a second open int one element later with
    // its own indexing, extending highind.
    let mut a = RangeHint::new(0x10, 4, 0x10, base(4, type_metatype::TYPE_INT), 0, RangeType::Open, 0);
    let b = RangeHint::new(0x14, 4, 0x14, base(4, type_metatype::TYPE_INT), 0, RangeType::Open, 2);
    a.absorb(&b);
    assert_eq!(a.range_type, RangeType::Open);
    // diffsz = (0x14-0x10)/4 = 1; trialhi = b.highind(2)+1 = 3 > a.highind(0)
    assert_eq!(a.highind, 3);
}

#[test]
fn attempt_join_concatenates_matching_array_elements() {
    // Open int array with room (highind=3) joins a later same-type open element.
    // The C++ `attemptJoin` tests data-type identity by pointer (`aTestType !=
    // bTestType`), and the TypeFactory interns identical types to one pointer;
    // model that by sharing the same `Rc` for both hints.
    let int4 = base(4, type_metatype::TYPE_INT);
    let mut a = RangeHint::new(0x10, 4, 0x10, Rc::clone(&int4), 0, RangeType::Open, 3);
    let b = RangeHint::new(0x14, 4, 0x14, int4, 0, RangeType::Open, -1);
    assert!(a.attempt_join(&b));
}

#[test]
fn attempt_join_refuses_endpoint() {
    let mut a = RangeHint::new(0x10, 4, 0x10, base(4, type_metatype::TYPE_INT), 0, RangeType::Open, 3);
    let term = RangeHint::new(0x14, 1, 0x14, base(1, type_metatype::TYPE_UNKNOWN), 0, RangeType::Endpoint, -2);
    assert!(!a.attempt_join(&term)); // Don't merge with bounding range
}

#[test]
fn attempt_join_refuses_non_open_this() {
    let mut a = hint(0x10, 4, base(4, type_metatype::TYPE_INT), RangeType::Fixed);
    let b = hint(0x14, 4, base(4, type_metatype::TYPE_INT), RangeType::Open);
    assert!(!a.attempt_join(&b));
}

// --- merge ----------------------------------------------------------------

#[test]
fn merge_two_typelocked_overlapping_is_an_error() {
    let space = stack_space();
    let factory = factory();
    // Non-contained (b overhangs a) and both locked → the LowlevelError path.
    let mut a = RangeHint::new(0x10, 4, 0x10, base(4, type_metatype::TYPE_INT), TYPELOCK, RangeType::Fixed, -1);
    let b = RangeHint::new(0x12, 8, 0x12, base(8, type_metatype::TYPE_UINT), TYPELOCK, RangeType::Fixed, -1);
    let err = a.merge(&b, &space, &factory).unwrap_err();
    assert!(err.to_string().contains("Overlapping forced variable types"));
}

#[test]
fn merge_confused_types_yields_unknown() {
    let space = stack_space();
    let factory = factory();
    // Non-contained, neither locked → resType 2 (confuse): unknown type, size grows.
    let mut a = RangeHint::new(0x10, 4, 0x10, base(4, type_metatype::TYPE_INT), 0, RangeType::Fixed, -1);
    let b = RangeHint::new(0x12, 4, 0x12, base(4, type_metatype::TYPE_UINT), 0, RangeType::Fixed, -1);
    let ret = a.merge(&b, &space, &factory).unwrap();
    assert!(!ret); // merge always returns false on this path
    assert_eq!(a.type_.get_metatype(), type_metatype::TYPE_UNKNOWN);
    // diff = 0x12-0x10 = 2; diff + b.size(4) = 6 > size(4) → size becomes 6, not
    // a power of two so it collapses to size 1 / open.
    assert_eq!(a.size, 1);
    assert_eq!(a.range_type, RangeType::Open);
}

#[test]
fn merge_contained_prefers_this_when_locked() {
    let space = stack_space();
    let factory = factory();
    // a (locked, 8-byte) contains b (4-byte) at the same start → a absorbs b.
    let mut a = RangeHint::new(0x10, 8, 0x10, base(8, type_metatype::TYPE_INT), TYPELOCK, RangeType::Fixed, -1);
    let b = RangeHint::new(0x10, 4, 0x10, base(4, type_metatype::TYPE_INT), 0, RangeType::Fixed, -1);
    a.merge(&b, &space, &factory).unwrap();
    assert!(a.is_type_lock());
    assert_eq!(a.size, 8);
}

// --- MapState: sort + reconcileDatatypes ----------------------------------

/// Build a one-range analysis window [first,last] over the stack space.
fn window(space: &Rc<AddrSpace>, first: uintb, last: uintb) -> RangeList {
    let mut rl = RangeList::new();
    rl.insert_range(Rc::clone(space), first, last);
    rl
}

#[test]
fn mapstate_initialize_sorts_and_terminates() {
    let space = stack_space();
    let factory = factory();
    let dflt = factory.get_base(1, type_metatype::TYPE_UNKNOWN).unwrap();
    let rn = window(&space, 0, 0x100);
    let pm = RangeList::new();
    let mut state = MapState::new(Rc::clone(&space), &rn, &pm, dflt);

    // Add out-of-order fixed hints.
    state.add_range_pub(0x30, Some(base(4, type_metatype::TYPE_INT)), 0, RangeType::Fixed, -1);
    state.add_range_pub(0x10, Some(base(4, type_metatype::TYPE_INT)), 0, RangeType::Fixed, -1);
    state.add_range_pub(0x20, Some(base(4, type_metatype::TYPE_INT)), 0, RangeType::Fixed, -1);

    assert!(state.initialize().unwrap());

    // Walk the sorted list: 0x10, 0x20, 0x30, then the artificial endpoint at
    // high = last+1 = 0x101.  (C++ idiom: read current, then `getNext()` to
    // advance; `getNext()` returning false means the iterator is now at end and
    // `next()` must not be read again.)
    let mut walk = vec![(state.next().start, state.next().range_type)];
    while state.get_next() {
        walk.push((state.next().start, state.next().range_type));
    }
    let starts: Vec<uintb> = walk.iter().map(|(s, _)| *s).collect();
    assert_eq!(starts, vec![0x10, 0x20, 0x30, 0x101]);
    // The final, artificial range is the endpoint bound.
    assert_eq!(walk.last().unwrap().1, RangeType::Endpoint);
}

#[test]
fn mapstate_reconcile_collapses_same_keyed_duplicates() {
    let space = stack_space();
    let factory = factory();
    let dflt = factory.get_base(1, type_metatype::TYPE_UNKNOWN).unwrap();
    let rn = window(&space, 0, 0x100);
    let pm = RangeList::new();
    let mut state = MapState::new(Rc::clone(&space), &rn, &pm, dflt);

    // Two identical-key 4-byte hints at 0x10 (same start/size/flags), one UINT
    // one INT: reconcileDatatypes keeps a single entry with the more-specific
    // type and drops the duplicate (compare()==0).
    state.add_range_pub(0x10, Some(base(4, type_metatype::TYPE_UINT)), 0, RangeType::Fixed, -1);
    state.add_range_pub(0x10, Some(base(4, type_metatype::TYPE_INT)), 0, RangeType::Fixed, -1);

    assert!(state.initialize().unwrap());

    // First entry is the single reconciled 0x10 hint, then the endpoint.
    let first = state.next().start;
    assert_eq!(first, 0x10);
    let mut count_at_0x10 = 1;
    while state.get_next() {
        if state.next().start == 0x10 {
            count_at_0x10 += 1;
        }
    }
    assert_eq!(count_at_0x10, 1, "duplicate same-keyed hints collapse to one");
}

#[test]
fn mapstate_addrange_drops_out_of_window_hints() {
    let space = stack_space();
    let factory = factory();
    let dflt = factory.get_base(1, type_metatype::TYPE_UNKNOWN).unwrap();
    let rn = window(&space, 0x10, 0x1f); // only [0x10,0x1f] analyzed
    let pm = RangeList::new();
    let mut state = MapState::new(Rc::clone(&space), &rn, &pm, dflt);

    state.add_range_pub(0x10, Some(base(4, type_metatype::TYPE_INT)), 0, RangeType::Fixed, -1); // in
    state.add_range_pub(0x40, Some(base(4, type_metatype::TYPE_INT)), 0, RangeType::Fixed, -1); // out

    assert!(state.initialize().unwrap());
    let mut starts = vec![state.next().start];
    while state.get_next() {
        starts.push(state.next().start);
    }
    // Only the in-window hint plus the endpoint survive (endpoint at last+1=0x20).
    assert_eq!(starts, vec![0x10, 0x20]);
}

#[test]
fn mapstate_empty_does_not_initialize() {
    let space = stack_space();
    let factory = factory();
    let dflt = factory.get_base(1, type_metatype::TYPE_UNKNOWN).unwrap();
    let rn = window(&space, 0, 0x100);
    let pm = RangeList::new();
    let mut state = MapState::new(Rc::clone(&space), &rn, &pm, dflt);
    assert!(!state.initialize().unwrap()); // no hints → false
}

#[test]
fn mapstate_constructor_removes_param_ranges_from_window() {
    // The param range [0x80,0xff] is carved out of the analysis window, so a
    // hint there is dropped even though it is inside the original [0,0xff].
    let space = stack_space();
    let factory = factory();
    let dflt = factory.get_base(1, type_metatype::TYPE_UNKNOWN).unwrap();
    let rn = window(&space, 0, 0xff);
    let pm = window(&space, 0x80, 0xff);
    let mut state = MapState::new(Rc::clone(&space), &rn, &pm, dflt);

    state.add_range_pub(0x10, Some(base(4, type_metatype::TYPE_INT)), 0, RangeType::Fixed, -1); // kept
    state.add_range_pub(0x90, Some(base(4, type_metatype::TYPE_INT)), 0, RangeType::Fixed, -1); // carved out

    assert!(state.initialize().unwrap());
    let mut starts = vec![state.next().start];
    while state.get_next() {
        starts.push(state.next().start);
    }
    assert!(starts.contains(&0x10));
    assert!(!starts.contains(&0x90));
}

// --- MapState::addFixedType partial-type expansion ------------------------

#[test]
fn add_fixed_type_plain_type_is_a_fixed_hint() {
    let space = stack_space();
    let factory = factory();
    let dflt = factory.get_base(1, type_metatype::TYPE_UNKNOWN).unwrap();
    let rn = window(&space, 0, 0x100);
    let pm = RangeList::new();
    let mut state = MapState::new(Rc::clone(&space), &rn, &pm, dflt);

    state.add_fixed_type_pub(0x10, base(4, type_metatype::TYPE_INT), 0, &factory);
    assert!(state.initialize().unwrap());
    assert_eq!(state.next().start, 0x10);
    assert_eq!(state.next().range_type, RangeType::Fixed);
    assert_eq!(state.next().size, 4);
}

// --- AliasChecker: offset gather / sort / boundary ------------------------

/// A mock `AliasGatherSeam` that replays a pre-canned spacebase input,
/// additive-base set, and per-base offset map (so the offset/sort/boundary
/// logic can be tested without the W7 Varnode descend graph).
struct MockGather {
    spacebase: Option<crate::seams::VarnodeId>,
    bases: Vec<AddBase>,
    offsets: std::collections::BTreeMap<u64, uintb>,
}

impl AliasGatherSeam for MockGather {
    fn find_spacebase_input(&self, _space: &Rc<AddrSpace>) -> Option<crate::seams::VarnodeId> {
        self.spacebase
    }
    fn gather_additive_base(&mut self, _startvn: crate::seams::VarnodeId, addbase: &mut Vec<AddBase>) {
        addbase.extend(self.bases.iter().copied());
    }
    fn gather_offset(&mut self, vn: crate::seams::VarnodeId) -> uintb {
        // Key the canned offset by the slotmap key's index bits (stable per id).
        let k = vn_key(vn);
        *self.offsets.get(&k).unwrap_or(&0)
    }
}

/// Stable u64 key for a VarnodeId (its slotmap internal representation).
fn vn_key(id: crate::seams::VarnodeId) -> u64 {
    use slotmap::Key;
    id.data().as_ffi()
}

/// Mint a few distinct VarnodeIds via a throwaway slotmap.
fn mint_varnodes(n: usize) -> Vec<crate::seams::VarnodeId> {
    let mut sm: slotmap::SlotMap<crate::seams::VarnodeId, ()> = slotmap::SlotMap::with_key();
    (0..n).map(|_| sm.insert(())).collect()
}

#[test]
fn alias_gather_converts_and_records_offsets() {
    let space = stack_space();
    let ids = mint_varnodes(3);
    let mut offsets = std::collections::BTreeMap::new();
    offsets.insert(vn_key(ids[1]), 0x40u64); // base ids[1] -> offset 0x40
    offsets.insert(vn_key(ids[2]), 0x80u64);
    let mut mock = MockGather {
        spacebase: Some(ids[0]),
        bases: vec![AddBase::new(ids[1], None), AddBase::new(ids[2], Some(ids[0]))],
        offsets,
    };

    let mut checker = AliasChecker::new();
    // Negative-growth stack, no proto info → localBoundary defaults to 0x1000000.
    checker.gather(Rc::clone(&space), None, false, &mut mock);

    // One alias offset per AddBase, in gather order (word-size 1 → identity).
    assert_eq!(checker.get_alias(), &[0x40, 0x80]);
    assert_eq!(checker.get_add_base().len(), 2);
}

#[test]
fn alias_sort_orders_offsets() {
    let space = stack_space();
    let ids = mint_varnodes(3);
    let mut offsets = std::collections::BTreeMap::new();
    offsets.insert(vn_key(ids[1]), 0x80u64);
    offsets.insert(vn_key(ids[2]), 0x20u64);
    let mut mock = MockGather {
        spacebase: Some(ids[0]),
        bases: vec![AddBase::new(ids[1], None), AddBase::new(ids[2], None)],
        offsets,
    };
    let mut checker = AliasChecker::new();
    checker.gather(Rc::clone(&space), None, false, &mut mock);
    assert_eq!(checker.get_alias(), &[0x80, 0x20]); // gather order
    checker.sort_alias();
    assert_eq!(checker.get_alias(), &[0x20, 0x80]); // sorted
}

#[test]
fn alias_no_spacebase_is_empty() {
    let space = stack_space();
    let mut mock = MockGather {
        spacebase: None,
        bases: vec![],
        offsets: std::collections::BTreeMap::new(),
    };
    let mut checker = AliasChecker::new();
    checker.gather(Rc::clone(&space), None, false, &mut mock);
    assert!(checker.get_alias().is_empty());
}

#[test]
fn has_local_alias_uses_alias_boundary_for_negative_stack() {
    // For a negative-growth stack, `direction == 1`; hasLocalAlias returns true
    // only for offsets at or beyond the shallowest alias boundary.
    let space = stack_space();
    let ids = mint_varnodes(2);
    let mut offsets = std::collections::BTreeMap::new();
    // A local-region alias (>= localBoundary default 0x1000000) lowers the
    // boundary to 0x2000000.
    offsets.insert(vn_key(ids[1]), 0x2000000u64);
    let mut mock = MockGather {
        spacebase: Some(ids[0]),
        bases: vec![AddBase::new(ids[1], None)],
        offsets,
    };
    let mut checker = AliasChecker::new();
    checker.gather(Rc::clone(&space), None, false, &mut mock);

    // At/after the boundary → aliased; before → not.
    assert!(checker.has_local_alias(Some((Rc::clone(&space), 0x2000000)), &mut mock));
    assert!(checker.has_local_alias(Some((Rc::clone(&space), 0x3000000)), &mut mock));
    assert!(!checker.has_local_alias(Some((Rc::clone(&space), 0x1000000)), &mut mock));
    // None varnode → never aliased.
    assert!(!checker.has_local_alias(None, &mut mock));
}

#[test]
fn derive_boundaries_uses_proto_param_last_for_negative_stack() {
    // With a proto giving local-first and param-last, the local/param boundary
    // moves to param_last for a negative-growth stack (direction == 1).
    let space = stack_space();
    let ids = mint_varnodes(2);
    let mut offsets = std::collections::BTreeMap::new();
    // Offset just above the proto param_last (0x100) is in the local region.
    offsets.insert(vn_key(ids[1]), 0x180u64);
    let mut mock = MockGather {
        spacebase: Some(ids[0]),
        bases: vec![AddBase::new(ids[1], None)],
        offsets,
    };
    let bounds = ProtoBoundaries {
        has_local_first: true,
        has_param_last: true,
        param_last: 0x100,
        param_first: 0x80,
    };
    let mut checker = AliasChecker::new();
    checker.gather(Rc::clone(&space), Some(&bounds), false, &mut mock);
    // 0x180 > localBoundary(0x100) → it lowers aliasBoundary to 0x180.
    assert!(checker.has_local_alias(Some((Rc::clone(&space), 0x180)), &mut mock));
    // 0x140 is in the local region (> boundary) but BELOW the alias → not aliased.
    assert!(!checker.has_local_alias(Some((Rc::clone(&space), 0x140)), &mut mock));
}

// --- Recommendation record round-trips ------------------------------------

#[test]
fn recommendation_records_carry_their_fields() {
    let space = stack_space();
    let addr = Address::new(Rc::clone(&space), 0x10);
    let use_addr = Address::new(Rc::clone(&space), 0x20);
    let nr = NameRecommend::new(addr.clone(), use_addr.clone(), 4, "foo".into(), 7);
    assert_eq!(nr.size, 4);
    assert_eq!(nr.name, "foo");
    assert_eq!(nr.symbol_id, 7);

    let dr = DynamicRecommend::new(use_addr.clone(), 0xdead, "bar".into(), 9);
    assert_eq!(dr.hash, 0xdead);
    assert_eq!(dr.symbol_id, 9);

    let tr = TypeRecommend::new(addr, base(4, type_metatype::TYPE_INT));
    assert_eq!(tr.data_type.get_size(), 4);
}
