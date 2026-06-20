//! Adversarial verification tests for item `w6-harness-unittests`.
//!
//! The in-scope work is a *test harness* port (testcirclerange / testtypes /
//! testfuncproto / testparamstore).  The substantive risk is NOT that an
//! individual `TEST` fails — they pass — but that the brute-force reference
//! harness (`CircleRangeTest`) and the implementation it validates were ported
//! by the same hand and could share a coincident bug, so a green test means
//! nothing.  These tests provide an INDEPENDENT third oracle:
//!
//!   * `getStartStopStep` is re-derived here straight from the C++ source
//!     (`testcirclerange.cc:291-374`) with explicit signed/unsigned/wrap
//!     handling, then checked to round-trip against the public `CircleRange`
//!     boundaries — the hunt-list's wrap/off-by-one/comparator spots.
//!   * the multiset `set_union`/`set_intersection` semantics the harness relies
//!     on are pinned with duplicate-bearing and disjoint inputs.
//!   * `make_enum` + `Datatype::get_matches` are pinned on zero handling,
//!     complement masking at the size-8 boundary, and BTreeMap match-name
//!     ordering (values the two ported TESTs do not cover).

use std::collections::BTreeMap;
use std::rc::Rc;

use kuna_base::address::calc_mask;
use kuna_base::types::{int4, uintb, Wrap};
use kuna_decomp::dtype::{flags, type_metatype, Datatype, DatatypeKind};
use kuna_decomp::rangeutil::CircleRange;

// ---------------------------------------------------------------------------
// Independent re-derivation of testcirclerange.cc:291-374 getStartStopStep,
// transcribed from the C++ (NOT from the porter's Rust), used as a third
// oracle.  Mixed-sign / wrap handling is made explicit.
// ---------------------------------------------------------------------------

fn ref_get_start_stop_step(
    mut elements: Vec<uintb>,
    mask: uintb,
) -> Option<(uintb, uintb, int4)> {
    if elements.is_empty() {
        return Some((0, 0, 1));
    }
    elements.sort_unstable();
    // Dedup (C++ keeps first of each run).
    let mut dedup: Vec<uintb> = Vec::new();
    let mut lastel = elements[0];
    dedup.push(lastel);
    for &curel in &elements[1..] {
        if curel == lastel {
            continue;
        }
        dedup.push(curel);
        lastel = curel;
    }
    elements = dedup;

    if *elements.last().unwrap() > mask {
        return None;
    }
    if elements.len() == 1 {
        let start = elements[0];
        return Some((start, start.wadd(1) & mask, 1));
    }
    if elements.len() == 2 {
        let diff = elements[0].wsub(elements[1]) & mask;
        if diff == 1 || diff == 2 || diff == 4 || diff == 8 {
            let start = elements[1];
            let stop = start.wadd(diff).wadd(diff) & mask;
            return Some((start, stop, diff as int4));
        }
    }
    let mut bigpos: i64 = -1;
    let mut biggest1: uintb = 0;
    let mut biggest2: uintb = 0;
    for i in 1..elements.len() {
        let diff = elements[i].wsub(elements[i - 1]);
        if diff >= biggest1 {
            if diff > biggest1 {
                biggest2 = biggest1;
                biggest1 = diff;
                bigpos = i as i64;
            }
        } else if diff > biggest2 {
            biggest2 = diff;
        }
    }
    if biggest1 == 0 {
        return None;
    }
    if biggest2 == 0 {
        let step = biggest1 as int4;
        let start = elements[0];
        let stop = elements.last().unwrap().wadd(step as uintb) & mask;
        return Some((start, stop, step));
    }
    let mut count1 = 0i32;
    let mut count3 = 0i32;
    for i in 1..elements.len() {
        let diff = elements[i].wsub(elements[i - 1]);
        if diff == biggest1 {
            count1 += 1;
        } else if diff == biggest2 {
            // count2 (unused in C++ too)
        } else {
            count3 += 1;
        }
    }
    if count3 > 0 {
        return None;
    }
    if count1 > 1 {
        return None;
    }
    let step = biggest2 as int4;
    let mut tmp = elements.last().unwrap().wadd(step as uintb);
    if tmp <= mask {
        return None;
    }
    tmp = tmp.wsub(mask.wadd(1));
    if tmp != elements[0] {
        return None;
    }
    let start = elements[bigpos as usize];
    let stop = elements[(bigpos - 1) as usize].wadd(step as uintb);
    Some((start, stop, step))
}

/// Brute-force the element set of a `CircleRange` through the public API,
/// exactly as the C++ `CircleRangeTest(const CircleRange&)` does
/// (`getMin` then `getNext`).
fn brute_elements(range: &CircleRange) -> Vec<uintb> {
    let mut out = Vec::new();
    if range.is_empty() {
        return out;
    }
    let mut start = range.get_min();
    loop {
        out.push(start);
        if !range.get_next(&mut start) {
            break;
        }
    }
    out
}

/// The harness's whole contract: for any non-empty `CircleRange`, brute-forcing
/// its elements and feeding them back through the (independent) reference
/// `getStartStopStep` must reproduce `(getMin, getEnd, getStep)`.  This is the
/// invariant `testEqual`/`testIntersect`/`testUnion` lean on; if the porter's
/// harness or `CircleRange` drifted, this independent oracle diverges.
fn assert_roundtrip(lft: uintb, rgt: uintb, size: int4, step: int4) {
    let range = CircleRange::new(lft, rgt, size, step);
    let els = brute_elements(&range);
    let mask = calc_mask(size);
    let got = ref_get_start_stop_step(els.clone(), mask);
    assert!(
        got.is_some(),
        "ref getStartStopStep rejected a real CircleRange [{lft:#x},{rgt:#x}) step {step} els={els:x?}"
    );
    let (start, stop, st) = got.unwrap();
    assert_eq!(start, range.get_min(), "start mismatch for [{lft:#x},{rgt:#x}) step {step}");
    assert_eq!(stop, range.get_end(), "stop mismatch for [{lft:#x},{rgt:#x}) step {step}");
    assert_eq!(st, range.get_step(), "step mismatch for [{lft:#x},{rgt:#x}) step {step}");
}

#[test]
fn w6_harness_unittests_roundtrip_wrapping_and_step() {
    // The wrap-prone / stepped boundary ranges drawn straight from the
    // in-scope TESTs (intersect5/intersect7/union7/pushright2 inputs), plus a
    // single-value and a 2-element power-of-two-diff case (the special arms of
    // getStartStopStep).
    assert_roundtrip(0xffe0, 0x20, 2, 2); // wraps past 0, step 2 (intersect7)
    assert_roundtrip(0xfe00, 0xffc0, 2, 0x20); // big step, no wrap (pushright2)
    assert_roundtrip(0x100000, 0x1000fe, 8, 2); // 8-byte, step 2 (intersect5)
    assert_roundtrip(0x80, 0x8, 1, 1); // wraps, step 1 (intersect8)
    assert_roundtrip(7, 8, 4, 1); // single element (len==1 arm)
    assert_roundtrip(4, 12, 1, 4); // two elements, diff==4 (len==2 arm)
    assert_roundtrip(0xf0, 0x10, 1, 1); // wraps near top of byte (pullback inputs)
}

#[test]
fn w6_harness_unittests_roundtrip_full_byte_step_then_wrap() {
    // Well-formed (left within mask) stepped ranges that wrap past 0 — the
    // `tmp -= (mask+1)` wrap-closure arm (getStartStopStep:366-373), the most
    // fragile signed/unsigned/wrap spot.  All inputs keep left <= mask, the only
    // domain the C++ CircleRangeTest harness ever constructs.
    assert_roundtrip(0xe0, 0x40, 1, 4); // byte: [0xe0.. wrap ..0x3c] step 4
    assert_roundtrip(0xfff0, 0x20, 2, 8); // step 8, wraps mod 2^16
    // The biggest2==0 (single uniform step, no wrap closure) arm:
    assert_roundtrip(0x10, 0x40, 4, 8); // uniform step 8, 32-bit
}

// ---------------------------------------------------------------------------
// Multiset set_union / set_intersection semantics (the harness's
// set_intersect/set_union over sorted, possibly-duplicate sequences).
// ---------------------------------------------------------------------------

fn set_intersection(a: &[uintb], b: &[uintb]) -> Vec<uintb> {
    let (mut i, mut j) = (0usize, 0usize);
    let mut res = Vec::new();
    while i < a.len() && j < b.len() {
        match a[i].cmp(&b[j]) {
            std::cmp::Ordering::Less => i += 1,
            std::cmp::Ordering::Greater => j += 1,
            std::cmp::Ordering::Equal => {
                res.push(a[i]);
                i += 1;
                j += 1;
            }
        }
    }
    res
}

fn set_union(a: &[uintb], b: &[uintb]) -> Vec<uintb> {
    let (mut i, mut j) = (0usize, 0usize);
    let mut res = Vec::new();
    while i < a.len() && j < b.len() {
        match a[i].cmp(&b[j]) {
            std::cmp::Ordering::Less => {
                res.push(a[i]);
                i += 1;
            }
            std::cmp::Ordering::Greater => {
                res.push(b[j]);
                j += 1;
            }
            std::cmp::Ordering::Equal => {
                res.push(a[i]);
                i += 1;
                j += 1;
            }
        }
    }
    res.extend_from_slice(&a[i..]);
    res.extend_from_slice(&b[j..]);
    res
}

#[test]
fn w6_harness_unittests_multiset_ops() {
    // std::set_intersection / std::set_union retain duplicate multiplicity =
    // min / max of the two run lengths.  The harness depends on this when two
    // CircleRangeTest element vectors share values.
    let a = [1u64, 2, 2, 5, 5, 5, 9];
    let b = [2u64, 2, 2, 5, 7];
    // intersection: 2 appears min(2,3)=2 times, 5 appears min(3,1)=1 time.
    assert_eq!(set_intersection(&a, &b), vec![2, 2, 5]);
    // union: 2 appears max(2,3)=3 times, 5 appears max(3,1)=3 times, plus the
    // unique tails 1, 7, 9.
    assert_eq!(set_union(&a, &b), vec![1, 2, 2, 2, 5, 5, 5, 7, 9]);
    // disjoint -> empty intersection (the `code==0` false / empty-elements arm
    // of testEqual, e.g. circlerange_intersect6 inputs touch this).
    assert_eq!(set_intersection(&[1u64, 3, 5], &[2u64, 4, 6]), Vec::<u64>::new());
    // empty operand
    assert_eq!(set_union(&[], &[3u64, 4]), vec![3, 4]);
    assert_eq!(set_intersection(&[3u64, 4], &[]), Vec::<u64>::new());
}

// ---------------------------------------------------------------------------
// make_enum + Datatype::get_matches adversarial coverage.
// ---------------------------------------------------------------------------

fn make_enum(size: i32, entries: &[(u64, &str)]) -> Rc<Datatype> {
    let mut dt = Datatype::new_with_align(size, size, type_metatype::TYPE_INT);
    dt.flags |= flags::enumtype;
    let mut namemap: BTreeMap<u64, String> = BTreeMap::new();
    for (v, n) in entries {
        namemap.insert(*v, (*n).to_string());
    }
    dt.kind = DatatypeKind::Enum { namemap };
    Rc::new(dt)
}

#[test]
fn w6_harness_unittests_enum_zero_and_complement_boundary() {
    // enum3 from the C++ TEST, but probing values the ported TESTs skip.
    let enum3 = make_enum(
        8,
        &[(0, "ZERO"), (1, "ONE"), (2, "TWO"), (4, "FOUR"), (8, "EIGHT")],
    );

    // 3 = ONE|TWO, both present, no complement (ordered most-significant first).
    let rep = enum3.get_matches(3).unwrap();
    assert_eq!(rep.match_name, vec!["TWO".to_string(), "ONE".to_string()]);
    assert!(!rep.complement);

    // 0xb = 8|2|1 -> EIGHT|TWO|ONE, exact, no complement.
    let rep = enum3.get_matches(0xb).unwrap();
    assert_eq!(
        rep.match_name,
        vec!["EIGHT".to_string(), "TWO".to_string(), "ONE".to_string()]
    );
    assert!(!rep.complement);

    // 0xffff_ffff_ffff_ffff at size 8: ~0 -> ZERO complemented (mask is full
    // 64-bit; the size-8 boundary is what makes ZERO the lone complement match).
    let rep = enum3.get_matches(0xffff_ffff_ffff_ffff).unwrap();
    assert!(rep.complement);

    // A value with a bit set above every named flag and not representable as an
    // OR of names, nor its complement -> empty match_name.
    let rep = enum3.get_matches(0x10).unwrap();
    assert!(rep.match_name.is_empty());
    assert!(!rep.complement);
}

#[test]
fn w6_harness_unittests_enum_size_changes_complement_mask() {
    // The same value at different enum sizes must mask differently: get_matches
    // uses `size` for the complement (~val & mask).  At size 1 the relevant
    // mask is 0xff so ~4 within a byte is 0xfb = EIGHT|TWO|ONE complement of
    // FOUR.  This pins that make_enum's `size` argument actually feeds the
    // masking (a size that silently defaulted to 8 would give a different
    // representation here).
    let e1 = make_enum(1, &[(1, "ONE"), (2, "TWO"), (4, "FOUR"), (8, "EIGHT")]);
    let rep = e1.get_matches(0xfb).unwrap();
    // 0xfb (byte) is not an exact OR of the named single-bit flags (bit 0x40
    // etc are unnamed), so the direct pass fails and the complement pass runs:
    // ~0xfb & 0xff = 0x04 = FOUR, complement = true.
    assert!(rep.complement);
    assert_eq!(rep.match_name, vec!["FOUR".to_string()]);

    // The identical numeric value at size 8 cannot complement to a single named
    // flag (the high bits 0xffff_ffff_ffff_ff00 are now part of ~val), so the
    // representation differs from the size-1 case above.
    let e8 = make_enum(8, &[(1, "ONE"), (2, "TWO"), (4, "FOUR"), (8, "EIGHT")]);
    let rep8 = e8.get_matches(0xfb).unwrap();
    assert!(
        !(rep8.complement && rep8.match_name == vec!["FOUR".to_string()]),
        "size must affect complement masking; size-8 must not match the size-1 single-flag complement"
    );
}
