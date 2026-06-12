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
