// Verifier adversarial tests for item `w6-s5-constseq-prefersplit` (round 1).
//
// Scope of the public surface reachable from an integration test:
//   - `PreferSplitRecord` total-order comparator (the wave's "ParamEntry/ProtoModel
//     sort orders" hunt: antisymmetry, transitivity, and the mixed-space / equal
//     index / size-desc tie-break that drives `initialize`'s sort and
//     `find_record`'s lower_bound).
//   - `PreferSplitManager::find_record` lower_bound + equality probe at the
//     boundaries the C++ `lower_bound`/`templ < *iter` pair is fragile at:
//       * a vn key strictly greater than every record (iter == end()),
//       * a vn key strictly less than every record (templ < *begin),
//       * an exact-key hit sitting in the middle of a same-space run.
//
// `ArraySequence`/`form_byte_array` is `pub(crate)`, so its overflow/order probes
// live alongside the unit tests in `src/constseq/tests.rs` (also tagged with this
// item id) — they cannot be reached from this external crate.

use std::rc::Rc;

use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};
use kuna_base::types::int4;
use kuna_num::pcoderaw::VarnodeData;

use kuna_decomp::dtype::{type_metatype, Datatype};
use kuna_decomp::funcdata::Funcdata;
use kuna_decomp::prefersplit::{PreferSplitManager, PreferSplitRecord};
use kuna_decomp::seams::Architecture;

use kuna_base::address::Address;

// Two distinct processor spaces so we can exercise the space-index leg of the
// comparator (the C++ first key: `storage.space->getIndex()`).
fn build_manager() -> AddrSpaceManager {
    let mut m = AddrSpaceManager::new();
    m.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    m.insert_space(Rc::new(UniqueSpace::new(1, 0, false))).unwrap();
    m.insert_space(Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "ram",
        false,
        8,
        1,
        2,
        addrspace_flags::hasphysical,
        1,
        1,
    )))
    .unwrap();
    m.insert_space(Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "reg",
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
    Funcdata::new("func", "func", glb, addr, 0x1000_0000, 0x40).unwrap()
}

fn space(fd: &Funcdata, name: &str) -> Rc<AddrSpace> {
    Rc::clone(fd.get_arch().manage().get_space_by_name(name).unwrap())
}

fn rec(space: Rc<AddrSpace>, off: u64, size: u32, splitoffset: int4) -> PreferSplitRecord {
    PreferSplitRecord {
        storage: VarnodeData { space: Some(space), offset: off, size },
        splitoffset,
    }
}

fn mk_vn(fd: &mut Funcdata, sp: Rc<AddrSpace>, off: u64, size: int4) -> kuna_decomp::seams::VarnodeId {
    let ct: Rc<Datatype> = Rc::new(Datatype::new(size, type_metatype::TYPE_UNKNOWN));
    fd.vbank_mut().create(size, Address::new(sp, off), ct)
}

// --- w6-s5: PreferSplitRecord comparator totality ----------------------------

/// The comparator must be a strict-weak total order: antisymmetric and
/// transitive, matching C++ `PreferSplitRecord::operator<` (space index, then
/// BIGGER size first, then offset).  We brute-force every ordered pair/triple
/// over a deliberately adversarial fixture that crosses the space-index leg, the
/// size-desc leg (the `Reverse` cast that is easy to flip), and the offset leg.
#[test]
fn w6s5_record_comparator_is_total_order() {
    let fd = build_fd();
    let ram = space(&fd, "ram");
    let reg = space(&fd, "reg");
    // Mix spaces (indices differ), sizes (1,4,8), offsets (0,0x100,0x100).
    let recs = vec![
        rec(Rc::clone(&ram), 0x100, 8, 4),
        rec(Rc::clone(&ram), 0x100, 4, 2),
        rec(Rc::clone(&ram), 0x200, 4, 2),
        rec(Rc::clone(&ram), 0x100, 1, 0),
        rec(Rc::clone(&reg), 0x000, 8, 4),
        rec(Rc::clone(&reg), 0x100, 8, 4),
        rec(Rc::clone(&reg), 0x100, 4, 2),
    ];
    // Antisymmetry: never both a<b and b<a; a<b xor b<a unless equal.
    for a in &recs {
        for b in &recs {
            let ab = a < b;
            let ba = b < a;
            assert!(!(ab && ba), "antisymmetry violated");
            if a != b {
                assert!(ab ^ ba, "two distinct records must be strictly ordered");
            } else {
                assert!(!ab && !ba, "equal records must compare equal");
            }
        }
    }
    // Transitivity: a<b & b<c => a<c.
    for a in &recs {
        for b in &recs {
            for c in &recs {
                if a < b && b < c {
                    assert!(a < c, "transitivity violated");
                }
            }
        }
    }
    // Spot-check the size-desc leg explicitly: within a space+offset, the
    // 8-byte record must sort strictly before the 4-byte and 1-byte ones.
    let big = rec(Rc::clone(&ram), 0x100, 8, 4);
    let mid = rec(Rc::clone(&ram), 0x100, 4, 2);
    let small = rec(Rc::clone(&ram), 0x100, 1, 0);
    assert!(big < mid && mid < small, "bigger size must come first (Reverse leg)");
}

/// `initialize` (a plain `sort`) must lay the table out in exactly the order
/// `find_record`'s lower_bound walks: ascending space index, then DESCENDING
/// size, then ascending offset.  We assert the full materialized order, because
/// a flipped `Reverse` would still "sort" but in the wrong direction silently.
#[test]
fn w6s5_initialize_full_order_across_spaces() {
    let fd = build_fd();
    let ram = space(&fd, "ram");
    let reg = space(&fd, "reg");
    let ram_idx = ram.get_index();
    let reg_idx = reg.get_index();
    assert!(ram_idx < reg_idx, "fixture assumes ram before reg by index");

    let mut recs = vec![
        rec(Rc::clone(&reg), 0x100, 4, 2),
        rec(Rc::clone(&ram), 0x200, 4, 2),
        rec(Rc::clone(&ram), 0x100, 8, 4),
        rec(Rc::clone(&reg), 0x000, 8, 4),
        rec(Rc::clone(&ram), 0x100, 4, 2),
    ];
    PreferSplitManager::initialize(&mut recs);

    let key: Vec<(i32, u32, u64)> = recs
        .iter()
        .map(|r| (r.storage.space.as_ref().unwrap().get_index(), r.storage.size, r.storage.offset))
        .collect();
    assert_eq!(
        key,
        vec![
            (ram_idx, 8, 0x100), // ram first; within ram, size 8 before size 4
            (ram_idx, 4, 0x100), // same size run ordered by ascending offset
            (ram_idx, 4, 0x200),
            (reg_idx, 8, 0x000), // then reg; size 8 first
            (reg_idx, 4, 0x100),
        ],
    );
}

// --- w6-s5: find_record lower_bound + equality probe boundaries ---------------

/// `find_record` must reproduce the C++ `lower_bound` + `if (templ < *iter)`
/// pair at all three boundaries: a key past the end, a key before the start,
/// and an exact hit buried in a same-space run.  A `partition_point` predicate
/// using `<=` instead of `<`, or a missing end()/`templ < *iter` guard, would
/// misfire at exactly these spots.
#[test]
fn w6s5_find_record_lower_bound_boundaries() {
    let mut fd = build_fd();
    let ram = space(&fd, "ram");
    let reg = space(&fd, "reg");

    let mut recs = vec![
        rec(Rc::clone(&ram), 0x100, 8, 7),
        rec(Rc::clone(&ram), 0x100, 4, 5),
        rec(Rc::clone(&ram), 0x200, 4, 3),
        rec(Rc::clone(&reg), 0x100, 4, 1),
    ];
    PreferSplitManager::initialize(&mut recs);
    let mut mgr = PreferSplitManager::default();
    mgr.init(&recs);

    // Buried exact hit: size-4 at ram:0x100 (second record after size-desc sort).
    let vn_mid = mk_vn(&mut fd, Rc::clone(&ram), 0x100, 4);
    assert_eq!(mgr.find_record(&fd, vn_mid).expect("buried hit").splitoffset, 5);

    // Past-the-end key: reg:0x200 sorts after every record -> lower_bound == end
    // -> None (NOT a wrap to records[0]).
    let vn_end = mk_vn(&mut fd, Rc::clone(&reg), 0x200, 4);
    assert!(mgr.find_record(&fd, vn_end).is_none(), "key past end must miss");

    // Before-the-start key: ram:0x100 size 16 sorts before the size-8 record
    // (bigger size first), so lower_bound lands on records[0] but templ < *iter
    // -> None.
    let vn_before = mk_vn(&mut fd, Rc::clone(&ram), 0x100, 16);
    assert!(mgr.find_record(&fd, vn_before).is_none(), "key before start must miss");

    // Genuine hit at the very first slot (size-8 at ram:0x100).
    let vn_first = mk_vn(&mut fd, Rc::clone(&ram), 0x100, 8);
    assert_eq!(mgr.find_record(&fd, vn_first).expect("first hit").splitoffset, 7);

    // Genuine hit at the very last slot (reg:0x100 size 4).
    let vn_last = mk_vn(&mut fd, Rc::clone(&reg), 0x100, 4);
    assert_eq!(mgr.find_record(&fd, vn_last).expect("last hit").splitoffset, 1);
}
