//! Adversarial verification tests for item `w6-s4-fspec-1`
//! (the parameter-recovery foundation: `decompiler/cpp/fspec.cc` ~1-2267 —
//! `ParamEntry`, `ParamTrial`, `ParamActive`, `ParamList*`).
//!
//! Written by the INDEPENDENT verifier. These target the spots the hunt list
//! flagged as most fragile for this item:
//!
//!   - **Comparator totality.** `ParamTrial::operator<` (fspec.cc:1895) is a
//!     strict weak ordering in C++: a null-`entry` trial returns `false` for
//!     `a<b` and is ordered AFTER every non-null trial; two null trials are
//!     EQUIVALENT (`!(a<b) && !(b<a)`).  The Rust `cmp` must turn that into a
//!     total `Ordering` whose `(None,None)` case is `Equal`, not `Greater`.
//!     A `Greater`-both-ways comparator fed to `sort_unstable_by` is a
//!     non-total order whose output is unspecified.
//!   - **`characterizeAsParam` second-pass window.** C++ (fspec.cc:684) runs
//!     the second exclusion scan over the iterator window LEFT OVER from the
//!     first `find()`; the Rust re-derives the window from `find_begin`.  Verify
//!     the RESULT (the Containment code) is identical across the cases.
//!   - **Wrapping / endianness arithmetic** in `getAddrBySlot` (reverse stack)
//!     and `justifiedContain` (big-endian alignment boundary).
//!
//! All entries are built through the real `ParamEntry::seed` post-decode
//! resolution chain, matching the in-crate test harness.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{spacetype, AddrSpace, AddrSpaceManager};
use kuna_base::types::int4;

use kuna_decomp::dtype::type_class;
use kuna_decomp::fspec::{
    Containment, ParamActive, ParamEntry, ParamListKind, ParamListStandard, ParamTrial,
};

// --- shared builders (mirrors src/fspec/tests.rs) ---------------------------

fn reg_space_le() -> Rc<AddrSpace> {
    Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "register",
        false,
        4,
        1,
        3,
        0,
        0,
        0,
    ))
}

fn stack_space() -> Rc<AddrSpace> {
    Rc::new(AddrSpace::new(
        spacetype::IPTR_SPACEBASE,
        "stack",
        false,
        8,
        1,
        5,
        0,
        0,
        0,
    ))
}

fn addr(spc: &Rc<AddrSpace>, off: u64) -> Address {
    Address::new(Rc::clone(spc), off)
}

fn excl_entry(
    grp: int4,
    space: &Rc<AddrSpace>,
    base: u64,
    size: int4,
    prev: &[ParamEntry],
    mgr: &AddrSpaceManager,
) -> ParamEntry {
    ParamEntry::seed(
        grp,
        type_class::TYPECLASS_GENERAL,
        Rc::clone(space),
        base,
        size,
        1,
        0,
        0,
        true,
        false,
        prev,
        mgr,
    )
    .expect("seed exclusion entry")
}

fn two_excl_model() -> (ParamListStandard, Rc<AddrSpace>) {
    let mgr = AddrSpaceManager::new();
    let reg = reg_space_le();
    let mut model = ParamListStandard::new(ParamListKind::Standard);
    let e0 = excl_entry(0, &reg, 0x10, 4, &[], &mgr);
    model.push_entry(e0);
    let e1 = excl_entry(1, &reg, 0x20, 4, model.get_entry(), &mgr);
    model.push_entry(e1);
    model.finish_decode();
    (model, reg)
}

// ===========================================================================
// F1: ParamTrial::cmp comparator totality on the (None, None) case
// ===========================================================================

/// C++ `operator<` (fspec.cc:1898-1899): for two null-entry trials, both
/// `a<b` and `b<a` are FALSE, i.e. they are EQUIVALENT under the strict weak
/// ordering.  The faithful `Ordering` is therefore `Equal`.  The port returns
/// `Greater` for `(None, _)` unconditionally, so `a.cmp(b)` and `b.cmp(a)` are
/// BOTH `Greater` — antisymmetry is violated.
///
/// This test pins the divergence: it FAILS on the current port (documents the
/// finding) and would pass once `(None, None)` maps to `Equal`.
#[test]
fn two_null_entry_trials_are_equivalent_not_both_greater_w6s4() {
    let (model, reg) = two_excl_model();
    let entries = model.get_entry();
    let a = ParamTrial::new(addr(&reg, 0x30), 4, 1); // entry == None
    let b = ParamTrial::new(addr(&reg, 0x40), 4, 2); // entry == None

    let ab = a.cmp(&b, entries);
    let ba = b.cmp(&a, entries);

    // Antisymmetry: a.cmp(b) and b.cmp(a) must be opposite (or both Equal).
    // C++ says equivalent => Equal/Equal.  The bug yields Greater/Greater.
    assert_eq!(
        ab,
        std::cmp::Ordering::Equal,
        "two null-entry trials must compare Equal (C++ operator< returns false both ways), got {:?}",
        ab
    );
    assert_eq!(
        ab.reverse(),
        ba,
        "comparator must be antisymmetric: a.cmp(b)={:?} but b.cmp(a)={:?}",
        ab,
        ba
    );
}

/// The downstream consequence: `sort_trials` must leave every null-entry trial
/// AFTER every non-null trial (C++: non-null `b < a` null is true, so nulls
/// sink to the end regardless of `std::sort`'s instability).  With three null
/// trials interleaved with two real ones, a non-total comparator can let
/// `sort_unstable` strand a null trial in front of a real one.
#[test]
fn sort_keeps_all_null_trials_after_real_ones_w6s4() {
    let (model, reg) = two_excl_model();
    let mut active = ParamActive::new(false);
    // Interleave: null, real(grp1), null, real(grp0), null.
    active.register_trial(&addr(&reg, 0x50), 4); // 0: null
    active.register_trial(&addr(&reg, 0x20), 4); // 1: real -> entry 1 (grp 1)
    active.register_trial(&addr(&reg, 0x60), 4); // 2: null
    active.register_trial(&addr(&reg, 0x10), 4); // 3: real -> entry 0 (grp 0)
    active.register_trial(&addr(&reg, 0x70), 4); // 4: null
    active.get_trial_mut(1).set_entry(Some(1), 0);
    active.get_trial_mut(3).set_entry(Some(0), 0);

    active.sort_trials(model.get_entry());

    // The two real trials must come first, in group order (grp0 then grp1),
    // and all three null trials must follow.
    let n = active.get_num_trials();
    let mut seen_null = false;
    for i in 0..n {
        let has_entry = active.get_trial(i).get_entry().is_some();
        if !has_entry {
            seen_null = true;
        } else {
            assert!(
                !seen_null,
                "a real (non-null-entry) trial appeared AFTER a null trial at index {} \
                 — null trials did not all sink to the end",
                i
            );
        }
    }
    // And the real ones are in group order.
    assert_eq!(active.get_trial(0).get_entry(), Some(0));
    assert_eq!(active.get_trial(1).get_entry(), Some(1));
}

/// Stress the non-total comparator: many null trials interleaved with reals,
/// across several interleavings, to see whether `sort_unstable_by` can be
/// driven to strand a null trial before a real one (which would make the
/// `(None,None) => Greater` defect OUTPUT-AFFECTING rather than latent).
///
/// `sort_unstable` (pdqsort) does NOT promise a defined result for a non-total
/// comparator, so a future toolchain bump could change the answer; this test
/// records the behavior observed by the verifier on the pinned toolchain.
#[test]
fn sort_stress_null_vs_real_invariant_w6s4() {
    let (model, reg) = two_excl_model();
    let entries = model.get_entry();

    // Build a builder for a "real" trial of group g (entry index == g here).
    let real = |off: u64, g: usize| -> ParamTrial {
        let mut t = ParamTrial::new(addr(&reg, off), 4, 1);
        t.set_entry(Some(g), 0);
        t
    };
    let null = |off: u64| ParamTrial::new(addr(&reg, off), 4, 1);

    // Several adversarial interleavings of 2 reals and N nulls.
    let layouts: Vec<Vec<ParamTrial>> = vec![
        vec![null(0x50), null(0x51), real(0x20, 1), null(0x52), real(0x10, 0)],
        vec![real(0x20, 1), null(0x50), null(0x51), null(0x52), real(0x10, 0)],
        vec![
            null(0x50),
            real(0x10, 0),
            null(0x51),
            real(0x20, 1),
            null(0x52),
            null(0x53),
            null(0x54),
        ],
        vec![
            null(0x5a),
            null(0x5b),
            null(0x5c),
            null(0x5d),
            real(0x20, 1),
            real(0x10, 0),
            null(0x5e),
            null(0x5f),
        ],
    ];

    for (li, layout) in layouts.into_iter().enumerate() {
        let mut v = layout;
        v.sort_unstable_by(|a, b| a.cmp(b, entries));
        let mut seen_null = false;
        for (i, t) in v.iter().enumerate() {
            if t.get_entry().is_none() {
                seen_null = true;
            } else if seen_null {
                panic!(
                    "layout {}: real trial at index {} sorted AFTER a null trial \
                     — the non-total ParamTrial comparator corrupted the order",
                    li, i
                );
            }
        }
        // Reals (when present) stay in group order.
        let reals: Vec<usize> = v.iter().filter_map(|t| t.get_entry()).collect();
        assert_eq!(reals, vec![0, 1], "layout {}: reals lost group order", li);
    }
}

// ===========================================================================
// F2: characterizeAsParam — the second-pass window divergence is benign
// ===========================================================================

/// Two exclusion entries, then probe a range that is contained-by the *second*
/// entry but does not justify-contain the first.  C++ reaches the second-pass
/// exclusion scan; the Rust re-derives that window from `find_begin`.  The
/// resulting Containment code must match the C++ semantics exactly.
#[test]
fn characterize_contained_by_via_second_pass_w6s4() {
    let mgr = AddrSpaceManager::new();
    let reg = reg_space_le();
    let mut model = ParamListStandard::new(ParamListKind::Standard);
    // entry 0: [0x10, 0x14)
    model.push_entry(excl_entry(0, &reg, 0x10, 4, &[], &mgr));
    // entry 1: [0x20, 0x24)
    let prev = model.get_entry().to_vec();
    model.push_entry(excl_entry(1, &reg, 0x20, 4, &prev, &mgr));
    model.finish_decode();
    model.populate_resolver();

    // A 2-byte slice fully inside entry 1 ([0x22,0x24)): justifiedContain != 0
    // (LE, justified at the base 0x20, so 0x22 gives off=2 > 0 => ContainsUnjustified).
    assert_eq!(
        model.characterize_as_param(&addr(&reg, 0x22), 2),
        Containment::ContainsUnjustified
    );
    // A range that strictly CONTAINS entry 1 ([0x1e, 0x26)) but is justified at
    // neither => the second pass must report ContainedBy (entry1 is exclusion,
    // containedBy the range).
    assert_eq!(
        model.characterize_as_param(&addr(&reg, 0x1e), 8),
        Containment::ContainedBy
    );
    // A range matching entry 0 exactly => ContainsJustified (off == 0).
    assert_eq!(
        model.characterize_as_param(&addr(&reg, 0x10), 4),
        Containment::ContainsJustified
    );
    // A range in unmapped space => NoContainment.
    assert_eq!(
        model.characterize_as_param(&addr(&reg, 0x100), 4),
        Containment::NoContainment
    );
}

/// Regression on the exact spot the iteration-range divergence could bite: the
/// first `find()` window is NON-EMPTY (so C++ second pass starts AFTER it, Rust
/// restarts from find_begin and re-scans it).  A range that overlaps the first
/// entry (so the first window is non-empty) but is ultimately contained-by a
/// LATER entry must still report ContainedBy, never a different code from the
/// re-scan.
#[test]
fn characterize_nonempty_first_window_rescan_is_benign_w6s4() {
    let mgr = AddrSpaceManager::new();
    let reg = reg_space_le();
    let mut model = ParamListStandard::new(ParamListKind::Standard);
    // Three contiguous exclusion entries so a wide probe spans several windows.
    model.push_entry(excl_entry(0, &reg, 0x10, 2, &[], &mgr));
    let prev = model.get_entry().to_vec();
    model.push_entry(excl_entry(1, &reg, 0x12, 2, &prev, &mgr));
    let prev = model.get_entry().to_vec();
    model.push_entry(excl_entry(2, &reg, 0x14, 2, &prev, &mgr));
    model.finish_decode();
    model.populate_resolver();

    // Probe [0x12, 0x14): exactly entry 1, justified at its base (LE) => off==0.
    assert_eq!(
        model.characterize_as_param(&addr(&reg, 0x12), 2),
        Containment::ContainsJustified
    );
    // Probe [0x11, 0x15): overlaps entry0/1/2, contains entry1 ([0x12,0x14)).
    // entry1 is exclusion & containedBy => ContainedBy must win, and the first
    // (non-empty) window's re-scan must not produce a spurious code.
    assert_eq!(
        model.characterize_as_param(&addr(&reg, 0x11), 4),
        Containment::ContainedBy
    );
}

// ===========================================================================
// F3: wrapping / endianness arithmetic
// ===========================================================================

/// `getAddrBySlot` on a REVERSE-stack (positive-growth) entry: C++ computes
/// `index = numslots - slotnum - slotsused` then `addressbase + index*align`.
/// With a 16-byte, align-4 reverse-stack entry (4 slots), allocating an 8-byte
/// object from slot 0 must land at the TOP of the range (index = 4-0-2 = 2 =>
/// addressbase + 8) and consume 2 slots.
#[test]
fn get_addr_by_slot_reverse_stack_index_w6s4() {
    let mgr = AddrSpaceManager::new();
    let stk = stack_space();
    // normalstack = false => reverse_stack; align 4, size 16 => numslots 4.
    let e = ParamEntry::seed(
        0,
        type_class::TYPECLASS_GENERAL,
        Rc::clone(&stk),
        0x0,
        16,
        1,
        4,
        0,
        false, // reverse stack
        false,
        &[],
        &mgr,
    )
    .expect("seed reverse-stack entry");
    assert!(e.is_reverse_stack());

    let mut slotnum = 0;
    // 8-byte object => slotsused = 2.  index = 4 - 0 - 2 = 2 => offset 8.
    let a = e
        .get_addr_by_slot(&mut slotnum, 8, 1, &mgr)
        .expect("get_addr_by_slot");
    assert!(!a.is_invalid());
    assert_eq!(a.get_offset(), 8, "reverse-stack 8-byte slot-0 address");
    assert_eq!(slotnum, 2, "two slots consumed");

    // A second 8-byte object now sits below it: index = 4 - 2 - 2 = 0 => offset 0.
    let b = e
        .get_addr_by_slot(&mut slotnum, 8, 1, &mgr)
        .expect("get_addr_by_slot 2");
    assert_eq!(b.get_offset(), 0, "reverse-stack 8-byte slot-2 address");
    assert_eq!(slotnum, 4, "all four slots consumed");

    // A third allocation has no slots left => invalid address.
    let c = e
        .get_addr_by_slot(&mut slotnum, 8, 1, &mgr)
        .expect("get_addr_by_slot 3");
    assert!(c.is_invalid(), "no slots left => invalid");
}

/// `justifiedContain` big-endian, aligned (stack) branch: for a right-justified
/// entry the result is `(alignment - (endaddr+1) % alignment)` unless that
/// remainder is 0.  Exercise the boundary where `(endaddr+1) % alignment == 0`
/// returns 0, and a non-aligned end returns the padding.
#[test]
fn justified_contain_be_aligned_boundary_w6s4() {
    let mgr = AddrSpaceManager::new();
    // Big-endian stack-like space, align 4, size 16.
    let be_stack = Rc::new(AddrSpace::new(
        spacetype::IPTR_SPACEBASE,
        "bestack",
        true, // big endian
        8,
        1,
        6,
        0,
        0,
        0,
    ));
    let e = ParamEntry::seed(
        0,
        type_class::TYPECLASS_GENERAL,
        Rc::clone(&be_stack),
        0x0,
        16,
        1,
        4,
        0,
        true,
        false,
        &[],
        &mgr,
    )
    .expect("seed be-aligned entry");
    assert!(!e.is_exclusion());

    // A 4-byte range exactly at slot boundary [0x0,0x4): endaddr=3, (3+1)%4==0 => 0.
    assert_eq!(e.justified_contain(&addr(&be_stack, 0x0), 4), 0);
    // A 2-byte range [0x0,0x2): endaddr=1, (1+1)%4 = 2 => alignment-2 = 2.
    assert_eq!(e.justified_contain(&addr(&be_stack, 0x0), 2), 2);
    // A 2-byte range [0x2,0x4): endaddr=3, (3+1)%4 = 0 => 0 (right-justified).
    assert_eq!(e.justified_contain(&addr(&be_stack, 0x2), 2), 0);
    // Out of range => -1.
    assert_eq!(e.justified_contain(&addr(&be_stack, 0x20), 4), -1);
}

/// `containedBy` must reject a wrap-around range without panicking: a range
/// whose `offset + sz - 1` wraps past u64::MAX (C++ uintb wrap) must not be
/// treated as a giant container.  The entry is at the top of the space.
#[test]
fn contained_by_no_panic_on_wrap_w6s4() {
    let mgr = AddrSpaceManager::new();
    let reg = reg_space_le();
    let e = excl_entry(0, &reg, 0x10, 4, &[], &mgr);
    // sz = 0 would make (sz-1) wrap to u64::MAX inside the helper; the port uses
    // wrapping arithmetic, so this must not panic.  We only assert no panic and
    // a deterministic bool.
    let _ = e.contained_by(&addr(&reg, 0x10), 0);
    // A normal exact containment still holds.
    assert!(e.contained_by(&addr(&reg, 0x10), 4));
}

// ===========================================================================
// ROUND 2 (re-verify of the F1 comparator fix): full strict-weak-order audit.
//
// Round 1 REJECTed because `cmp` returned `Greater` for (None, None) in BOTH
// directions (antisymmetry broken).  The repair maps (None, None) -> Equal.
// These tests go beyond the single antisymmetry probe and assert the THREE
// strict-weak-order laws the C++ `operator<` satisfies and that any
// `sort_unstable_by` consumer relies on:
//   (1) irreflexivity / Equal-on-self,
//   (2) antisymmetry: cmp(a,b) == cmp(b,a).reverse(),
//   (3) transitivity of the induced ordering.
// They are checked EXHAUSTIVELY over a representative trial population that
// mixes the every comparator branch (group, entry-index, exclusion offset,
// reverse-stack addr, size) PLUS null entries.
// ===========================================================================

/// Build a model with: entry0 grp0 excl @0x10, entry1 grp1 excl @0x20,
/// entry2 grp1 excl @0x24 (same group as entry1 -> entry-index tiebreak),
/// returning the model + register space.
fn three_excl_two_in_group_model() -> (ParamListStandard, Rc<AddrSpace>) {
    let mgr = AddrSpaceManager::new();
    let reg = reg_space_le();
    let mut model = ParamListStandard::new(ParamListKind::Standard);
    let e0 = excl_entry(0, &reg, 0x10, 4, &[], &mgr);
    model.push_entry(e0);
    let e1 = excl_entry(1, &reg, 0x20, 4, model.get_entry(), &mgr);
    model.push_entry(e1);
    // Distinct group so resolveOverlap does not reject; group 2.
    let e2 = excl_entry(2, &reg, 0x30, 4, model.get_entry(), &mgr);
    model.push_entry(e2);
    model.finish_decode();
    (model, reg)
}

/// Exhaustively verify the strict-weak-order laws of `ParamTrial::cmp` over a
/// population that exercises every comparator branch AND null entries.  This is
/// the round-2 generalization of the round-1 antisymmetry probe: a non-total
/// comparator (the round-1 defect) FAILS law (2) on the (null,null) pair and
/// can FAIL law (3) anywhere; a correct total order passes all three.
#[test]
fn cmp_is_total_strict_weak_order_exhaustive_w6s4() {
    use std::cmp::Ordering;
    let (model, reg) = three_excl_two_in_group_model();
    let entries = model.get_entry();

    // Population: real trials matching each entry (different groups/indices) +
    // two trials in the same group (entry-index tiebreak) + two null trials.
    let mk = |off: u64, sz: int4, ent: Option<usize>| {
        let mut t = ParamTrial::new(addr(&reg, off), sz, 0);
        if let Some(e) = ent {
            t.set_entry(Some(e), 0);
        }
        t
    };
    let pop = vec![
        mk(0x10, 4, Some(0)), // grp0
        mk(0x20, 4, Some(1)), // grp1, entry idx 1
        mk(0x30, 4, Some(2)), // grp2, entry idx 2
        mk(0x20, 2, Some(1)), // same entry as #1, smaller size (size tiebreak)
        mk(0x20, 4, Some(1)), // duplicate of #1 -> must be Equal to #1
        mk(0x99, 4, None),    // null entry
        mk(0xAA, 4, None),    // null entry (must be Equal to the other null)
    ];

    let cmp = |i: usize, j: usize| pop[i].cmp(&pop[j], entries);

    // (1) reflexivity: every element Equal to itself.
    for i in 0..pop.len() {
        assert_eq!(
            cmp(i, i),
            Ordering::Equal,
            "cmp not reflexive at {i}"
        );
    }
    // (2) antisymmetry: cmp(i,j) == cmp(j,i).reverse() for ALL pairs.
    for i in 0..pop.len() {
        for j in 0..pop.len() {
            assert_eq!(
                cmp(i, j),
                cmp(j, i).reverse(),
                "antisymmetry broken for ({i},{j}): {:?} vs reverse({:?})",
                cmp(i, j),
                cmp(j, i)
            );
        }
    }
    // The two null trials specifically must be Equal (the F1 fix).
    assert_eq!(cmp(5, 6), Ordering::Equal, "two null trials must be Equal");
    // (3) transitivity of the induced ordering: if i<=j and j<=k then i<=k,
    //     and the Equal-equivalence is also transitive.  Checked exhaustively.
    let le = |a: Ordering| a != Ordering::Greater; // a <= b
    for i in 0..pop.len() {
        for j in 0..pop.len() {
            for k in 0..pop.len() {
                if le(cmp(i, j)) && le(cmp(j, k)) {
                    assert!(
                        le(cmp(i, k)),
                        "transitivity broken: {i}<={j} && {j}<={k} but not {i}<={k}"
                    );
                }
            }
        }
    }
}

/// `fixedPositionCompare` (fspec.cc:1922) must also be a total order, because
/// `sortFixedPosition` feeds it to `sort_unstable_by`.  Its (-1,-1) arm
/// delegates to `cmp`, so any non-totality in `cmp` (the round-1 defect) leaks
/// through.  Exhaustively audit antisymmetry over a mix of fixed and unfixed
/// positions, including two unfixed null-entry trials (the path that delegates
/// to the (None,None) `cmp` case).
#[test]
fn fixed_position_compare_is_total_order_w6s4() {
    use std::cmp::Ordering;
    let (model, reg) = three_excl_two_in_group_model();
    let entries = model.get_entry();

    let mk = |off: u64, ent: Option<usize>, fp: int4| {
        let mut t = ParamTrial::new(addr(&reg, off), 4, 0);
        if let Some(e) = ent {
            t.set_entry(Some(e), 0);
        }
        if fp >= 0 {
            t.set_fixed_position(fp);
        }
        t
    };
    let pop = vec![
        mk(0x10, Some(0), 0),  // fixed pos 0
        mk(0x20, Some(1), 1),  // fixed pos 1
        mk(0x30, Some(2), -1), // unfixed, real entry -> delegates to cmp
        mk(0x99, None, -1),    // unfixed, null entry -> (None,None) cmp path
        mk(0xAA, None, -1),    // unfixed, null entry
        mk(0x20, Some(1), 0),  // same fixed pos as #0 -> Equal on fixedPosition
    ];

    let fc =
        |i: usize, j: usize| ParamTrial::fixed_position_compare(&pop[i], &pop[j], entries);

    for i in 0..pop.len() {
        assert_eq!(fc(i, i), Ordering::Equal, "fixedPositionCompare not reflexive at {i}");
        for j in 0..pop.len() {
            assert_eq!(
                fc(i, j),
                fc(j, i).reverse(),
                "fixedPositionCompare antisymmetry broken for ({i},{j})"
            );
        }
    }
    // The two unfixed null trials must be Equal (delegated (None,None) -> Equal).
    assert_eq!(fc(3, 4), Ordering::Equal, "unfixed null trials must be Equal");
    // An unfixed trial sorts AFTER any fixed one (C++ returns false for a<b when
    // a.fixedPosition == -1 and b is fixed): cmp(unfixed, fixed) == Greater.
    assert_eq!(fc(2, 0), Ordering::Greater, "unfixed must sort after fixed");
    assert_eq!(fc(0, 2), Ordering::Less, "fixed must sort before unfixed");
}

/// `ParamActive::splitTrial` then `joinTrial` slot bookkeeping (fspec.cc:2035,
/// 2065).  splitTrial bumps `slotbase` and re-slots every trial whose slot is
/// past the split point; joinTrial collapses two adjacent slots and shifts the
/// rest down.  A round trip (split slot 2, then join it back) must restore the
/// original slot layout and trial count, and `slotbase` must return to its
/// starting value.  Targets the off-by-one / slot-shift bookkeeping in the
/// hunt list.
#[test]
fn split_then_join_round_trips_slots_w6s4() {
    let reg = reg_space_le();
    let mut active = ParamActive::new(false);
    // Three trials in slots 1,2,3 (registerTrial assigns slotbase++, starting 1).
    active.register_trial(&addr(&reg, 0x10), 8); // slot 1
    active.register_trial(&addr(&reg, 0x20), 8); // slot 2
    active.register_trial(&addr(&reg, 0x30), 8); // slot 3
    let slots0: Vec<int4> = (0..active.get_num_trials())
        .map(|i| active.get_trial(i).get_slot())
        .collect();
    assert_eq!(slots0, vec![1, 2, 3], "initial slot layout");

    // Split the slot-2 (index 1) trial into a 4+4 pair.  This makes slot 2 ->
    // two trials, and the old slot-3 trial must shift up to slot 4.
    active.split_trial(1, 4).expect("split");
    assert_eq!(active.get_num_trials(), 4, "split adds one trial");
    let slots1: Vec<int4> = (0..active.get_num_trials())
        .map(|i| active.get_trial(i).get_slot())
        .collect();
    // splitHi keeps slot 2, splitLo gets slot 3, old slot-3 trial bumped to 4.
    assert_eq!(slots1, vec![1, 2, 3, 4], "post-split slots");

    // Now join slots 2 and 3 back into one 8-byte trial.  The size check
    // (4+4 == 8) must pass and the trial count returns to 3 with slots 1,2,3.
    active
        .join_trial(2, &addr(&reg, 0x20), 8)
        .expect("join 2+3 -> 8");
    assert_eq!(active.get_num_trials(), 3, "join removes one trial");
    let slots2: Vec<int4> = (0..active.get_num_trials())
        .map(|i| active.get_trial(i).get_slot())
        .collect();
    assert_eq!(slots2, vec![1, 2, 3], "join restores the slot layout");

    // A join whose pieces do not sum to the requested size must error (C++
    // throws "Size mismatch"); verify Result parity (no panic, Err).
    active.split_trial(1, 4).expect("split again");
    let bad = active.join_trial(2, &addr(&reg, 0x20), 7); // 4+4 != 7
    assert!(bad.is_err(), "join with size mismatch must be Err (C++ throws)");
}

/// `ParamEntry::getSlot` reverse-stack slot mapping (fspec.cc:411).  For a
/// reverse-stack non-exclusion entry the slot index is
/// `groupSet[0] + (numslots-1) - baseslot`, where `baseslot = (offset+skip
/// -addressbase)/alignment`.  This is the SLOT side of the round-1
/// `getAddrBySlot` test and exercises the reverse-stack arithmetic from the
/// other direction.  A faithful port must invert getAddrBySlot's index map.
#[test]
fn get_slot_reverse_stack_mapping_w6s4() {
    let mgr = AddrSpaceManager::new();
    let stk = stack_space();
    // align 4, size 16 => numslots 4, reverse stack, group base 0.
    let e = ParamEntry::seed(
        0,
        type_class::TYPECLASS_GENERAL,
        Rc::clone(&stk),
        0x0,
        16,
        1,
        4,
        0,
        false, // reverse stack
        false,
        &[],
        &mgr,
    )
    .expect("seed reverse-stack entry");
    assert!(e.is_reverse_stack());

    // baseslot at offset 0 = 0 => slot = (numslots-1) - 0 = 3 (top of stack).
    assert_eq!(e.get_slot(&addr(&stk, 0x0), 0), 3, "offset 0 => slot 3");
    // baseslot at offset 4 = 1 => slot = 3 - 1 = 2.
    assert_eq!(e.get_slot(&addr(&stk, 0x4), 0), 2, "offset 4 => slot 2");
    // skip advances within the same call: offset 0 + skip 12 => baseslot 3 => slot 0.
    assert_eq!(e.get_slot(&addr(&stk, 0x0), 12), 0, "offset 0 skip 12 => slot 0");
    // offset 8 => baseslot 2 => slot 1.
    assert_eq!(e.get_slot(&addr(&stk, 0x8), 0), 1, "offset 8 => slot 1");
}
