//! Verifier (round 2) adversarial tests for item `w2-sleigh-core`, focused on
//! the *walk-back loop* of `SleighBase::getRegisterName`
//! (decompiler/cpp/sleighbase.cc:144-168).
//!
//! Round 1 rejected this item for F1 (the `--upper_bound(key)` -> greatest
//! element `<= key` idiom mistranslated as a `Bound::Excluded(&key)` back-walk,
//! which skipped exact matches).  The round-1 fix switched the starting point to
//! `Bound::Included(&key)`.  Those tests pinned the *exact-match* and
//! *first-byte containment* cases, but did **not** exercise the subsequent
//! `while (iter != begin()) { --iter; ... }` walk-back, which fires when the
//! greatest-`<= key` register does not itself contain the query and the
//! algorithm must step back to a larger, lower-offset register at the same
//! `offbase`.
//!
//! These tests assert the genuine C++ outputs (computed by a standalone C++
//! re-implementation of the exact algorithm over the same register clusters)
//! against the real x86-64 engine, whose register cross-reference map and
//! comparator (`VarnodeData::operator<`, BIG sizes first) are the C++ ones.
//!
//! Expected values were computed by a standalone C++ re-implementation of the
//! exact `getRegisterName` algorithm fed the *complete* 1440-entry x86-64
//! register map dumped from this engine; over a 73,728-query sweep (0x1200
//! offsets x 16 sizes) the engine and the C++ oracle agreed byte-for-byte, so
//! these spot cases are a readable subset of that equivalence (including the
//! sub-registers AH@0x1, CL@0x8, MM0_Bb@0x1101, ST0h@0x1108, BND0_UB@0x748 that
//! shape the C++ `offbase`-constrained walk-back).

use kuna_base::address::Address;
use kuna_base::error::KunaResult;
use kuna_base::space::AddrSpace;
use std::rc::Rc;

use kuna_sleigh::globalcontext::ContextInternal;
use kuna_sleigh::loadimage::LoadImage;
use kuna_sleigh::sleigh::Sleigh;

struct DummyImg;
impl LoadImage for DummyImg {
    fn get_file_name(&self) -> &str {
        "dummy"
    }
    fn load_fill(&mut self, _ptr: &mut [u8], _addr: &Address) -> KunaResult<()> {
        Err(kuna_base::error::KunaError::data_unavail("dummy"))
    }
    fn get_arch_type(&self) -> Vec<u8> {
        Vec::new()
    }
    fn adjust_vma(&mut self, _adjust: i64) {}
}

fn repo_root() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .unwrap()
}

fn x86_64_engine() -> Sleigh {
    let sla = std::fs::read(
        repo_root().join("specs/Ghidra/Processors/x86/data/languages/x86-64.sla"),
    )
    .expect("read x86-64.sla");
    let ctx = Box::new(ContextInternal::new());
    let mut sleigh = Sleigh::new(Box::new(DummyImg), ctx);
    sleigh.initialize_from_sla(&sla).expect("initialize from sla");
    sleigh
}

/// Locate the register space (the one that holds RAX etc.) by finding the
/// space carrying the entry named "RAX".
fn register_space(sleigh: &Sleigh) -> Rc<AddrSpace> {
    let regs = sleigh.base().get_all_registers();
    for (st, name) in regs.iter() {
        if name == b"RAX" {
            return Rc::clone(st.space.as_ref().unwrap());
        }
    }
    panic!("x86-64 register map has no RAX entry");
}

fn name(sleigh: &Sleigh, space: &Rc<AddrSpace>, off: u64, size: i32) -> String {
    String::from_utf8_lossy(&sleigh.base().get_register_name(space, off, size)).into_owned()
}

// ---------------------------------------------------------------------------
// Walk-back across the RAX/EAX/AX/AL cluster.  Each non-trivial case forces the
// `while (iter != begin())` loop to step back to a larger, lower-offset register
// (or to exhaust and return "").  Expected values are the C++ oracle outputs.
// ---------------------------------------------------------------------------
#[test]
fn verify_w2core_walkback_rax_cluster() {
    let sleigh = x86_64_engine();
    let sp = register_space(&sleigh);

    // (off, size, expected) straight from the C++ oracle over the full map.
    let cases: &[(u64, i32, &str)] = &[
        (0x0, 1, "AL"),
        (0x0, 2, "AX"),
        (0x0, 3, "EAX"), // upper_bound lands EAX, contains directly
        (0x0, 4, "EAX"),
        (0x0, 5, "RAX"), // EAX(4) misses, same-offbase walk back to RAX(8)
        (0x0, 6, "RAX"),
        (0x0, 8, "RAX"),
        (0x0, 9, ""), // no register that large at off 0 -> walk-back exhausts
        // Interior-byte queries.  AH lives at 0x1, so the greatest-<=key entry
        // is at a *different* offbase than RAX/EAX — the C++ walk-back is
        // constrained to `point.offset == offbase`, so it stops and returns ""
        // exactly where the real map's sub-registers break the chain.
        (0x1, 1, "AH"),  // AH is the exact byte at offset 1
        (0x2, 2, ""),    // greatest-<= is AH(1,1); offbase 1, walk-back hits off 0 -> stop
        (0x4, 4, ""),    // no reg at offbase 4 covers; chain breaks
        (0x4, 5, ""),
        (0x7, 1, ""),    // byte 7: greatest-<= is at a sub-offset, chain breaks
        (0x8, 1, "CL"),  // first byte past RAX is CL (next cluster)
    ];

    let mut fails = Vec::new();
    for &(off, size, want) in cases {
        let got = name(&sleigh, &sp, off, size);
        if got != want {
            fails.push(format!("(off=0x{off:x},size={size}) got {got:?} want {want:?}"));
        }
    }
    assert!(
        fails.is_empty(),
        "getRegisterName walk-back diverged from the C++ oracle:\n{}",
        fails.join("\n")
    );
}

// ---------------------------------------------------------------------------
// MM0 cluster: five sizes (1,2,4,8,10) at one offbase — the deepest walk-back
// chain available.  Probes both whole-register and interior-byte queries.
// ---------------------------------------------------------------------------
#[test]
fn verify_w2core_walkback_mm0_cluster() {
    let sleigh = x86_64_engine();
    let sp = register_space(&sleigh);

    let base = 0x1100u64;
    let cases: &[(u64, i32, &str)] = &[
        (base, 1, "MM0_Ba"),
        (base, 3, "MM0_Da"), // upper_bound -> MM0_Da(4), contains
        (base, 9, "ST0"),    // MM0(8) misses, same-offbase walk back to ST0(10)
        (base, 10, "ST0"),
        (base, 11, ""),          // nothing 11 wide at this offbase -> exhaust
        (base + 1, 1, "MM0_Bb"), // exact sub-register byte at 0x1101
        (base + 8, 2, "ST0h"),   // exact sub-register (high word) at 0x1108
        (base + 9, 1, "ST0h"),   // interior of ST0h(2), walk back to ST0h
        (base + 10, 1, ""),      // first byte past the ST0 group
    ];

    let mut fails = Vec::new();
    for &(off, size, want) in cases {
        let got = name(&sleigh, &sp, off, size);
        if got != want {
            fails.push(format!("(off=0x{off:x},size={size}) got {got:?} want {want:?}"));
        }
    }
    assert!(
        fails.is_empty(),
        "MM0 walk-back diverged from the C++ oracle:\n{}",
        fails.join("\n")
    );
}

// ---------------------------------------------------------------------------
// BND cluster (8 then 16 at the same offbase) — a different size pair, plus the
// `off > offbase` interior-walk-back and the past-the-end miss.  Also asserts
// the `point.space != base` short-circuit by probing the constant space, which
// holds no registers (must return "").
// ---------------------------------------------------------------------------
#[test]
fn verify_w2core_walkback_bnd_and_space_mismatch() {
    let sleigh = x86_64_engine();
    let sp = register_space(&sleigh);

    let base = 0x740u64;
    let cases: &[(u64, i32, &str)] = &[
        (base, 8, "BND0_LB"),
        (base, 16, "BND0"),       // BND0_LB(8) misses, walk back to BND0(16)
        (base, 17, ""),           // nothing 17 wide -> exhaust
        (base + 4, 4, "BND0_LB"), // bytes 4..8 -> BND0_LB(8) interior walk-back
        (base + 8, 8, "BND0_UB"), // exact upper-bound sub-register at 0x748
        (base + 8, 9, ""),        // 8..17 exceeds every reg at offbase 0x748 -> exhaust
    ];

    let mut fails = Vec::new();
    for &(off, size, want) in cases {
        let got = name(&sleigh, &sp, off, size);
        if got != want {
            fails.push(format!("(off=0x{off:x},size={size}) got {got:?} want {want:?}"));
        }
    }
    assert!(
        fails.is_empty(),
        "BND walk-back diverged from the C++ oracle:\n{}",
        fails.join("\n")
    );

    // A space that holds no registers must always yield "" (the very first
    // `next_back()` is None, or the `point.space != base` guard fires).  Use the
    // constant space, which never appears in the register xref map.
    let const_space = Rc::clone(
        sleigh
            .base()
            .manager()
            .get_constant_space()
            .expect("constant space registered"),
    );
    for &(off, size) in &[(0u64, 1i32), (0x0, 8), (0x100, 4)] {
        let got = name(&sleigh, &const_space, off, size);
        assert_eq!(
            got, "",
            "constant space holds no registers; getRegisterName must be empty \
             at (off=0x{off:x},size={size})"
        );
    }
}
