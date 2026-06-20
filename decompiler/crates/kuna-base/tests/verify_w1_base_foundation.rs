//! Verifier adversarial tests for item `w1-base-foundation`
//! (docs/rust-port/verification.md, adversarial pass).
//!
//! These tests were written by the item's VERIFIER, independently of the
//! porter.  The expected outputs in `tests/data/w1_*_oracle.out` were
//! produced by an independent C++ harness compiled against the vendored
//! `decompiler/cpp/rangemap.hh` / `partmap.hh` (g++ -std=c++11, libstdc++),
//! re-run under `-fsanitize=address,undefined` with byte-identical output
//! (so the pinned traces are UB-free C++ behavior).  The Rust side rebuilds
//! the exact same printf-format trace and must match byte-for-byte.
//!
//! Deliberately targeted at the hunt-list hot spots:
//! - equal `(last, subsort)` multiset keys (hinted-insert placement order),
//! - the right-boundary duplicate-cell quirk of `insert`,
//! - erase-while-iterating (`tree.erase(low++)`) and the zip/unzip sewing,
//! - `u64` boundary wraps (`a-1` with `a == 0`),
//! - record-list splice order,
//! - a 45-op randomized differential run with a verifier-chosen LCG
//!   (different constants AND seed from the porter's digest test).

use kuna_base::partmap::PartMap;
use kuna_base::rangemap::{RangeMap, RangeRecord, RecordIdx};
use std::fmt::Write as _;

/// Mirrors the C++ harness record: u64 line domain, u64 subsort
/// (`subsorttype(false)` == 0, `subsorttype(true)` == u64::MAX), tag id.
#[derive(Debug, Clone)]
struct TestRecord {
    a: u64,
    b: u64,
    ss: u64,
    tag: u32,
}

impl RangeRecord for TestRecord {
    type LineType = u64;
    type SubsortType = u64;
    type InitType = (u32, u64); // (tag, subsort)

    fn create(data: (u32, u64), a: u64, b: u64) -> Self {
        TestRecord { a, b, ss: data.1, tag: data.0 }
    }
    fn get_first(&self) -> u64 {
        self.a
    }
    fn get_last(&self) -> u64 {
        self.b
    }
    fn get_subsort(&self) -> u64 {
        self.ss
    }
}

type RM = RangeMap<TestRecord>;

const RANGEMAP_ORACLE: &str = include_str!("data/w1_rangemap_oracle.out");
const PARTMAP_ORACLE: &str = include_str!("data/w1_partmap_oracle.out");

/// Extract one `=== name ===` section (header excluded) from an oracle dump.
fn section(all: &str, name: &str) -> String {
    let header = format!("=== {name} ===\n");
    let start = all.find(&header).expect("oracle section missing");
    let rest = &all[start + header.len()..];
    let end = rest.find("\n=== ").map(|i| i + 1).unwrap_or(rest.len());
    rest[..end].to_string()
}

/// Byte-identical replica of the C++ harness `trace()`.
fn trace(rm: &RM, maxpt: u64, out: &mut String) {
    for p in 0..=maxpt {
        write!(out, "F{p}:").unwrap();
        for idx in rm.find(p) {
            write!(out, " {}", rm.record(idx).tag).unwrap();
        }
        out.push('\n');
    }
    out.push_str("L:");
    for (_, r) in rm.records() {
        write!(out, " {}", r.tag).unwrap();
    }
    out.push('\n');
    out.push_str("I:");
    for idx in rm.iter() {
        write!(out, " {}", rm.record(idx).tag).unwrap();
    }
    out.push('\n');
    out.push_str("O:");
    for p in 0..=maxpt {
        match rm.find_overlap(p, p + 3) {
            Some(idx) => write!(out, " {}", rm.record(idx).tag).unwrap(),
            None => out.push_str(" -"),
        }
    }
    out.push('\n');
}

/// Equal `(last, subsort)` multiset keys: six identical [5,10] records with
/// the same subsort.  Pins the libstdc++ hinted-insert placement (hinted
/// in-loop inserts stack at the FRONT of the equal range, the trailing
/// unhinted duplicate appends at the BACK) and exact-node erase from the
/// middle of an equal range.
#[test]
fn verify_rangemap_equal_keys_matches_cpp_oracle() {
    let mut got = String::new();
    let mut rm = RM::new();
    let mut recs: Vec<RecordIdx> = Vec::new();
    for t in 1..=6u32 {
        recs.push(rm.insert((t, 7), 5, 10));
    }
    trace(&rm, 12, &mut got);
    rm.erase(recs[2]); // tag 3
    got.push_str("-- after erase tag3\n");
    trace(&rm, 12, &mut got);
    rm.erase(recs[0]); // tag 1
    got.push_str("-- after erase tag1\n");
    trace(&rm, 12, &mut got);
    assert_eq!(got, section(RANGEMAP_ORACLE, "equal_keys"));
}

/// Overlapping records sharing ONE subsort value: right boundary landing on
/// an existing cell boundary (the duplicate-cell quirk) plus a both-sides
/// unzip, then erase of the bottom record (zip re-sewing with equal keys).
#[test]
fn verify_rangemap_same_subsort_overlap_matches_cpp_oracle() {
    let mut got = String::new();
    let mut rm = RM::new();
    let a = rm.insert((1, 4), 0, 10);
    rm.insert((2, 4), 5, 10); // llast == b, same subsort
    rm.insert((3, 4), 3, 7); // unzip both sides
    trace(&rm, 12, &mut got);
    rm.erase(a);
    got.push_str("-- after erase tag1\n");
    trace(&rm, 12, &mut got);
    assert_eq!(got, section(RANGEMAP_ORACLE, "same_subsort_overlap"));
}

/// `find(point, sub1, sub2)` windows over every non-inverted subsort pair,
/// and the `find_begin`/`find_end` positional walk at every point —
/// `lower_bound`/`upper_bound` boundary-inclusion parity.
#[test]
fn verify_rangemap_subsort_windows_matches_cpp_oracle() {
    let mut got = String::new();
    let mut rm = RM::new();
    rm.insert((1, 10), 5, 10);
    rm.insert((2, 20), 5, 10);
    rm.insert((3, 5), 5, 10);
    rm.insert((4, 20), 8, 15); // overlapping, splits at 7/8
    trace(&rm, 16, &mut got);
    let subs: [u64; 4] = [0, 5, 10, 20];
    for &lo in &subs {
        for &hi in &subs {
            if hi < lo {
                continue; // inverted window is C++ UB; not oracled
            }
            write!(got, "S {lo} {hi}:").unwrap();
            for idx in rm.find_subsorts(9, lo, hi) {
                write!(got, " {}", rm.record(idx).tag).unwrap();
            }
            got.push('\n');
        }
    }
    for p in 0..=16u64 {
        write!(got, "BE{p}:").unwrap();
        let b = rm.find_begin(p);
        let e = rm.find_end(p);
        for idx in rm.iter_between(&b, &e) {
            write!(got, " {}", rm.record(idx).tag).unwrap();
        }
        got.push('\n');
    }
    assert_eq!(got, section(RANGEMAP_ORACLE, "subsort_windows"));
}

/// 45 randomized insert/erase ops driven by a verifier-chosen LCG (different
/// multiplier, increment and seed from the porter's digest test), traced in
/// full after every op.  The pinned C++ trace was sanitizer-clean, so this
/// differentially pins refinement, equal-key ordering, splice order and
/// zip/unzip at scale, independently of the porter's own differential test.
#[test]
#[allow(clippy::manual_is_multiple_of)] // keep the C++ harness's literal `% 3 != 0`
fn verify_rangemap_random45_matches_cpp_oracle() {
    struct Lcg(u64);
    impl Lcg {
        fn next(&mut self) -> u64 {
            self.0 = self
                .0
                .wrapping_mul(2862933555777941757)
                .wrapping_add(3037000493);
            self.0 >> 33
        }
    }
    let mut got = String::new();
    let mut rng = Lcg(0xDEADBEEF12345678);
    let mut rm = RM::new();
    let mut live: Vec<RecordIdx> = Vec::new();
    for op in 0..45u32 {
        let r = rng.next();
        if live.is_empty() || (r % 3) != 0 {
            let a = rng.next() % 20;
            let len = rng.next() % 6;
            let ss = rng.next() % 3;
            live.push(rm.insert((op, ss), a, a + len));
            writeln!(got, "op{op} ins").unwrap();
        } else {
            // cast: slot selection, same `% live.size()` math as the C++ harness
            let idx = (rng.next() % live.len() as u64) as usize;
            rm.erase(live[idx]);
            live.remove(idx);
            writeln!(got, "op{op} del").unwrap();
        }
        trace(&rm, 25, &mut got);
    }
    writeln!(got, "live {}", live.len()).unwrap();
    assert_eq!(got, section(RANGEMAP_ORACLE, "random"));
}

/// partmap: getValue/upper-bound off-by-one, split copy-from-predecessor vs
/// copy-from-default, exact re-split identity, clearRange(10, u64::MAX), and
/// every `bounds` validity code, all diffed against the C++ partmap.hh run.
#[test]
fn verify_partmap_matches_cpp_oracle() {
    const M: u64 = u64::MAX;
    let pts: [u64; 8] = [0, 1, 9, 10, 15, 20, M - 1, M];

    fn show(pm: &PartMap<u64, u32>, pts: &[u64], out: &mut String) {
        out.push_str("V:");
        for p in pts {
            write!(out, " {}", pm.get_value(p)).unwrap();
        }
        out.push_str("\nK:");
        for (k, v) in pm.iter() {
            write!(out, " {k}={v}").unwrap();
        }
        out.push('\n');
        for p in pts {
            let (v, before, after, valid) = pm.bounds(p);
            write!(out, "B{p}: v={v} valid={valid}").unwrap();
            if valid == 0 || valid == 2 {
                write!(out, " before={}", before.unwrap()).unwrap();
            }
            if valid == 0 || valid == 1 {
                write!(out, " after={}", after.unwrap()).unwrap();
            }
            out.push('\n');
        }
    }

    let mut got = String::new();
    let mut pm: PartMap<u64, u32> = PartMap::new(0);
    *pm.default_value_mut() = 7;

    got.push_str("=== empty ===\n");
    show(&pm, &pts, &mut got);

    got.push_str("=== splits ===\n");
    *pm.split(&10) = 1;
    *pm.split(&20) = 2;
    *pm.split(&0) = 5; // before all splits: copies defaultvalue, overwritten
    *pm.split(&M) = 9;
    writeln!(got, "resplit10={}", pm.split(&10)).unwrap(); // exact match: old value
    show(&pm, &pts, &mut got);

    got.push_str("=== clearRange(10,MAX) ===\n");
    let r = pm.clear_range(&10, &M);
    writeln!(got, "ret={r}").unwrap();
    show(&pm, &pts, &mut got);

    got.push_str("=== getValue lower edge ===\n");
    writeln!(got, "split1={}", pm.split(&1)).unwrap(); // copies split-0's value
    show(&pm, &pts, &mut got);

    assert_eq!(got, PARTMAP_ORACLE);
}

/// Pure-Rust pins for the documented UB-divergence paths the C++ cannot
/// oracle: inverted subsort window, inverted iter_between positions, and
/// partmap clear_range with equal boundaries all yield empty/no-op instead
/// of C++ iterator UB; erase of an `a == 0` record exercises the wrapping
/// `a-1` computation without panicking in debug builds.
#[test]
fn verify_defensive_ub_divergence_paths() {
    let mut rm = RM::new();
    rm.insert((1, 10), 5, 10);
    rm.insert((2, 20), 5, 10);
    // NOT inverted despite sub1 > sub2: lower_bound((7,20)) lands on the
    // ss10 cell, and upper_bound((10,10)) is past it — C++ defined behavior,
    // visits tag 1.  (Verified against the C++ harness directly.)
    let tags: Vec<u32> = rm.find_subsorts(7, 20, 10).map(|i| rm.record(i).tag).collect();
    assert_eq!(tags, vec![1]);
    // Genuinely inverted iterator pair: only ss-20 cells exist, so iter1 has
    // subsort 20 > sub2 == 10 and the C++ (iter1, iter2) pair is reversed —
    // C++ iterating it is UB, the Rust port returns empty.
    let mut rm20 = RM::new();
    rm20.insert((9, 20), 5, 10);
    assert!(rm20.find_subsorts(7, 20, 10).next().is_none());
    // Inverted positions — C++ UB, Rust empty.
    let b = rm.find_begin(7);
    let e = rm.find_end(7);
    assert!(rm.iter_between(&e, &b).next().is_none() || b == e);
    // a == 0 erase takes the wrapping aminus1 path in debug builds.  The
    // surviving record's cell is duplicated (right-boundary insert quirk),
    // so find(0) visits it twice — pinned against the C++ ("2 2").
    let mut rm2 = RM::new();
    let z = rm2.insert((1, 0), 0, 4);
    rm2.insert((2, 1), 0, 4);
    rm2.erase(z);
    let tags2: Vec<u32> = rm2.find(0).map(|i| rm2.record(i).tag).collect();
    assert_eq!(tags2, vec![2, 2]);
    // partmap clear_range with pnt1 == pnt2 — C++ UB, Rust erases nothing.
    let mut pm: PartMap<u64, u32> = PartMap::new(9);
    *pm.split(&3) = 1;
    *pm.clear_range(&5, &5) = 55;
    assert_eq!(*pm.get_value(&5), 55);
    assert_eq!(*pm.get_value(&4), 1);
    assert_eq!(*pm.get_value(&2), 9);
}
