//! Adversarial verifier tests for item `rport/w10-lanedivide`
//! (port of `ActionLaneDivide` + the `LaneDivide` engine + the laned-register
//! access map, `coreaction.cc:516-637`, `subflow.cc:3533-4143`,
//! `architecture.cc:291/313/933`, `funcdata_varnode.cc:300`).
//!
//! These tests drive the *public* lane-lookup surface that the ported
//! `ActionLaneDivide`/`checkForLanedRegister` flow pivots on, hitting the spots
//! the hunt list flagged as most fragile for this diff:
//!
//!   * **`Architecture::getLanedRegister` binary search** (architecture.cc:291).
//!     `int4 max = lanerecords.size() - 1` underflows to `-1` on an *empty*
//!     table (C++ relies on `min(0) <= max(-1)` being false).  The Rust writes
//!     `len() as int4 - 1`; an accidental `usize` subtraction would *panic in
//!     debug* on the empty table instead of returning `None`.  The exact-hit /
//!     size-miss branches (`sz < size` vs `size < sz`) are copy-paste prone.
//!   * **`getMinimumLanedRegisterSize`** (architecture.cc:313) — on the real
//!     `Architecture` it is empty → `-1`, else `lanerecords[0].getWholeSize()`
//!     (records size-sorted, `[0]` smallest).  On the per-function seam
//!     `ArchHandle` it returns the *cached* `min_laned_register_size` that
//!     `build_arch_handle` copied off the real arch (the access map is driven by
//!     that cache + the carried `lanerecords`); these tests drive the seam form.
//!   * **`LanedRegister` size-bit-mask boundary** (transform.hh / transform.cc) —
//!     `addLaneSize(16)` sets *bit 16* (`1u << 16`), `allowedLane`/`getSizeBitMask`
//!     must read it back; the x86 XMM whole size is 16, so an `as`-narrowing or a
//!     wrong shift width silently drops the real lane.
//!
//! C++ oracle anchors are cited inline.  `LanedKey` (the `lanedMap` ordering) is
//! `pub(crate)`, so its total-order transcription of `VarnodeData::operator<`
//! (space asc, offset asc, size *desc*) is exercised by the in-crate unit test
//! `funcdata::tests` rather than from here; these integration tests cover the
//! arch-side lookup the access map feeds.

use std::rc::Rc;

use kuna_base::error::{KunaError, KunaResult};
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};
use kuna_base::address::Address;
use kuna_base::types::int4;

use kuna_decomp::context::ArchContext;
use kuna_decomp::transform::LanedRegister;

use kuna_sleigh::globalcontext::ContextInternal;
use kuna_sleigh::loadimage::LoadImage;
use kuna_sleigh::sleigh::Sleigh;

struct DummyImg;
impl LoadImage for DummyImg {
    fn get_file_name(&self) -> &str {
        "dummy"
    }
    fn load_fill(&mut self, _ptr: &mut [u8], _addr: &Address) -> KunaResult<()> {
        Err(KunaError::data_unavail("dummy"))
    }
    fn get_arch_type(&self) -> Vec<u8> {
        Vec::new()
    }
    fn adjust_vma(&mut self, _adjust: i64) {}
}

fn bare_sleigh() -> Sleigh {
    Sleigh::new(Box::new(DummyImg), Box::new(ContextInternal::new()))
}

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
    m
}

fn ram_addr(arch: &ArchContext, off: u64) -> Address {
    let ram = Rc::clone(arch.manage().get_space_by_name("ram").unwrap());
    Address::new(ram, off)
}

/// HUNT(integer-width / wrapping): an empty `lanerecords` table must drive the
/// `int4 max = size()-1 = -1` underflow path of the C++ binary search, returning
/// `None` *without panicking*.  A `usize` subtraction (`0usize - 1`) would panic
/// in debug; a wrong empty-guard would deref `lanerecords[0]` out of bounds.
/// C++ architecture.cc:291 (`getLanedRegister`) + :313 (`getMinimumLanedRegisterSize`).
#[test]
fn w10_lanedivide_empty_table_lookup_is_none_not_panic() {
    let arch = ArchContext::new(build_manager());
    assert!(arch.lanerecords.is_empty());
    // getLanedRegister over an empty table is the C++ (LanedRegister*)0 == None,
    // for any size, with NO out-of-bounds access / debug panic on the
    // `int4 max = size()-1 == -1` underflow path (the whole point of `as int4`
    // instead of a `usize` subtraction).
    let addr = ram_addr(&arch, 0x100);
    for sz in [0, 1, 4, 8, 16, 32, 64, 1024] {
        assert!(
            arch.get_laned_register(&addr, sz).is_none(),
            "empty table must return None for size {sz}"
        );
    }
}

/// HUNT(off-by-one / comparator totality): a multi-record size-sorted table (the
/// x86-64 shape: whole sizes 16/32/64) must binary-search to an *exact* size hit
/// and return `None` for any size between / outside the records.  The `sz < size`
/// vs `size < sz` branch ordering and the `(min+max)/2` mid are the fragile bits.
/// The record is keyed on *size only* (the `loc` Address is unused), exactly as
/// the C++.  architecture.cc:291.
#[test]
fn w10_lanedivide_binary_search_exact_and_miss() {
    let mut arch = ArchContext::new(build_manager());
    // Size-sorted ascending, one record per whole size — the invariant
    // `decodeRegisterData` establishes (maskList walked by ascending index).
    arch.lanerecords = vec![
        LanedRegister::with_mask(16, 1u32 << 8),  // XMM: 8-byte lanes
        LanedRegister::with_mask(32, 1u32 << 8),  // YMM
        LanedRegister::with_mask(64, 1u32 << 8),  // ZMM
    ];
    let addr = ram_addr(&arch, 0x200);
    // The `loc` Address is unused: a different address still hits on size.
    let other = ram_addr(&arch, 0xdead_beef);

    // Exact hits — every record is reachable (left, middle, right of the search).
    for &sz in &[16, 32, 64] {
        let hit = arch
            .get_laned_register(&addr, sz)
            .unwrap_or_else(|| panic!("exact size {sz} must hit"));
        assert_eq!(hit.get_whole_size(), sz, "hit must be the record for {sz}");
        // Address is irrelevant to the lookup.
        assert!(arch.get_laned_register(&other, sz).is_some());
    }
    // Misses: below, between, and above the records all return None.
    for &sz in &[1, 8, 15, 17, 24, 31, 33, 48, 63, 65, 128] {
        assert!(
            arch.get_laned_register(&addr, sz).is_none(),
            "non-record size {sz} must miss"
        );
    }
}

/// HUNT(integer width / shift): a lane size of 16 (the x86 XMM *whole* size) sets
/// bit 16 of the 32-bit `sizeBitMask` (`1u32 << 16`), and `allowedLane`/
/// `getSizeBitMask` must read it back.  A narrowing `as` on the `1 << size` or a
/// wrong shift type would silently lose the real lane.  This is the size gate
/// `ActionLaneDivide::collectLaneSizes` consults via `allowedLanes.allowedLane`.
/// transform.cc (`addLaneSize`/`allowedLane`) + coreaction.cc:524.
#[test]
fn w10_lanedivide_lane_size_mask_boundary_16() {
    let mut lr = LanedRegister::new();
    // No lanes initially: nothing is allowed, mask is empty.
    assert_eq!(lr.get_size_bit_mask(), 0);
    assert!(!lr.allowed_lane(16));

    lr.add_lane_size(8);
    lr.add_lane_size(16);
    // Bits 8 and 16 set, nothing else.
    assert_eq!(lr.get_size_bit_mask(), (1u32 << 8) | (1u32 << 16));
    assert!(lr.allowed_lane(8));
    assert!(lr.allowed_lane(16));
    // Neighbours of the set bits are NOT allowed (no off-by-one in the shift).
    assert!(!lr.allowed_lane(7));
    assert!(!lr.allowed_lane(9));
    assert!(!lr.allowed_lane(15));
    assert!(!lr.allowed_lane(17));
    assert!(!lr.allowed_lane(0));

    // iter_sizes (LanedIterator::normalize) yields the set sizes ascending,
    // least-to-greatest, hitting the LOWEST set bit first (not bit 0).
    let sizes: Vec<int4> = lr.iter_sizes().collect();
    assert_eq!(sizes, vec![8, 16]);
}

/// HUNT(empty-guard, real arch): the *real* `architecture::Architecture`
/// `getMinimumLanedRegisterSize` computes from `lanerecords` directly
/// (architecture.rs:605): empty → `-1`, else `lanerecords[0].getWholeSize()`.
/// A freshly built arch (no pspec `<register_data>` decoded) has an empty table,
/// so the cached seam min (which `build_arch_handle` copies off this) is `-1`,
/// i.e. `checkForLanedRegister`'s `s >= minLanedSize` gate never fires and the
/// lanedMap stays empty — the correct no-vector behavior.  architecture.cc:313.
#[test]
fn w10_lanedivide_real_arch_empty_min_size_is_minus_one() {
    let arch = kuna_decomp::architecture::Architecture::new("t", bare_sleigh());
    assert_eq!(
        arch.get_minimum_laned_register_size(),
        -1,
        "no <register_data> parsed => empty lanerecords => -1"
    );
}
