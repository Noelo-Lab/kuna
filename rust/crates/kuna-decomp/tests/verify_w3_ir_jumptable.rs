//! Independent verifier adversarial tests for item `w3-ir-jumptable`
//! (`decompiler/cpp/jumptable.{cc,hh}`).
//!
//! These tests were written by the VERIFIER (not the porter) and target the
//! spots the hunt list flagged as most fragile for this item:
//!
//!   - integer-width / overflow fidelity in `JumpValuesRange::truncate`
//!     (`step * nm`, C++ `int4 * int4` vs the Rust `u64 * u64`);
//!   - `LoadTable::collapse_table`'s two-iterator in-place compaction
//!     (overlap-skip branch, exact-contiguity tie, multi-table split);
//!   - `IndexPair` comparator totality / `block2addr` ordering provenance;
//!   - `JumpValuesRangeDefault` iteration order (the extra value comes LAST and
//!     is not reversible);
//!   - `JumpTable::decode` label-ordering rule (a labeled entry after an
//!     unlabeled one is an error; the inverse is tolerated and back-filled).
//!
//! They land with the verdict regardless of outcome (verifier protocol §
//! "Adversarial pass").

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::marshal::{Decoder, PackedDecode, PackedEncode};
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};

use kuna_base::marshal::Encoder;
use kuna_decomp::jumptable::{
    CircleRange, IndexPair, JumpTable, JumpValues, JumpValuesRange, JumpValuesRangeDefault,
    LoadTable, ATTRIB_LABEL, ELEM_DEST, ELEM_JUMPTABLE, NO_LABEL,
};

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
    m.set_default_code_space(2).unwrap();
    m
}

fn ram_of(m: &AddrSpaceManager) -> Rc<AddrSpace> {
    Rc::clone(m.get_space_by_name("ram").unwrap())
}

// ===========================================================================
// HUNT: integer width / overflow in JumpValuesRange::truncate
//
// C++ (jumptable.cc:268):  uintb right = (left + step * nm) & range.getMask();
//   `step` and `nm` are int4 -> `step * nm` is 32-bit `int` arithmetic.
// Rust (jumptable.rs:1106): left.wadd((step as uintb).wmul(nm as uintb))
//   computes the product in full 64-bit.
//
// For step*nm that fits in 32 bits the two agree.  The first test pins the
// common (faithful) case.  The second documents the divergence at a product
// that overflows int32 -- a CONCRETE DIVERGENCE TRACE against the C++ oracle.
// ===========================================================================

#[test]
fn truncate_small_product_matches_cpp_int32() {
    // step=4, nm=10 -> product 40 fits int32; right = (0 + 40) & mask.
    // size 6 cases at step 4 over [0,40).  getSize = (40-0)&mask / 4 = 10.
    let mut jv = JumpValuesRange::new();
    jv.set_range(CircleRange::new(0, 1024, 4, 4)); // mask = 2^32-1, step 4
    jv.truncate(10);
    assert_eq!(jv.get_size(), 10, "10 cases of step 4 after truncate");
}

#[test]
fn truncate_step_times_nm_overflows_int32_diverges_from_cpp() {
    // Adversarial: step * nm overflows a 32-bit int.
    //   step = 0x10000, nm = 0x10000  ->  C++ int32 product = 0 (wraps mod 2^32)
    //                                  ->  Rust u64 product   = 0x1_0000_0000
    // Over a 48-bit mask (size 6) these give DIFFERENT ranges, hence different
    // getSize().  We compute the C++-oracle value by hand and show the Rust port
    // disagrees.
    let step: u64 = 0x10000;
    let nm: i32 = 0x10000;
    let left: u64 = 0;
    let mask: u64 = (1u64 << 48) - 1; // size 6

    // C++ oracle: (left + (int4)(step*nm)) & mask, with step*nm in int32.
    let cpp_prod_i32 = (step as i32).wrapping_mul(nm); // == 0
    let cpp_right = left.wrapping_add(cpp_prod_i32 as i64 as u64) & mask;
    let cpp_size = if step == 0 { 0 } else { (cpp_right.wrapping_sub(left) & mask) / step };
    assert_eq!(cpp_size, 0, "C++ int32 product wraps to 0 -> empty range");

    // Rust port behaviour through the public API.  (The initial range value is
    // immaterial — `truncate` reads only get_min/get_step/get_mask and overwrites
    // the bounds via set_range; `new` matches the real CircleRange constructor.)
    let mut jv = JumpValuesRange::new();
    jv.set_range(CircleRange::new(left, left, 6, step as i32));
    jv.truncate(nm);
    let rust_size = jv.get_size();

    // The divergence: Rust computes the product in 64 bits, so the range is NOT
    // empty.  If this assertion ever flips (rust_size == cpp_size), the port has
    // been corrected to match the C++ 32-bit overflow and this test should be
    // updated.  Recorded as a confirmed divergence (see review / losses).
    assert_ne!(
        rust_size, cpp_size,
        "DIVERGENCE: Rust truncate uses 64-bit step*nm; C++ uses int32 (oracle=0)"
    );
    assert_eq!(rust_size, 0x10000, "Rust 64-bit product yields full-step range");
}

// ===========================================================================
// HUNT: LoadTable::collapse_table two-iterator in-place compaction
// ===========================================================================

#[test]
fn collapse_table_overlap_entry_is_skipped() {
    // Sorted input, but the middle entry OVERLAPS the running nextaddr
    // (nextaddr > addr, same size): the C++ slow path falls through both the
    // merge `if` and the new-table `else if`, dropping the overlapping entry.
    let m = build_manager();
    let ram = ram_of(&m);
    let a = |off: u64| Address::new(Rc::clone(&ram), off);

    // entry0: [0x1000, size 4, num 4]  -> covers 0x1000..0x1010, nextaddr 0x1010
    // entry1: [0x1008, size 4, num 1]  -> 0x1008 < 0x1010 (overlap) -> SKIPPED
    // entry2: [0x1010, size 4, num 2]  -> contiguous with nextaddr -> merged
    let mut table = vec![
        LoadTable::full(a(0x1000), 4, 4),
        LoadTable::full(a(0x1008), 4, 1),
        LoadTable::full(a(0x1010), 4, 2),
    ];
    LoadTable::collapse_table(&mut table);

    // Net: a single table [0x1000, size 4] with num 4 + 2 = 6 (overlap dropped).
    assert_eq!(table.len(), 1, "overlap entry dropped, rest merged into one");
    assert_eq!(table[0].addr, a(0x1000));
    assert_eq!(table[0].size, 4);
    assert_eq!(table[0].num, 6, "0x1008 overlap contributed nothing");
}

#[test]
fn collapse_table_size_change_starts_new_table() {
    // Different entry sizes must start a new table even when addresses are
    // contiguous-by-bytes; exercises the `(*iter).size != (*lastiter).size`
    // branch of the new-table `else if` and the in-place `*lastiter = *iter`.
    let m = build_manager();
    let ram = ram_of(&m);
    let a = |off: u64| Address::new(Rc::clone(&ram), off);

    // entry0: [0x2000, size 4, num 2] -> nextaddr 0x2008
    // entry1: [0x2008, size 2, num 3] -> addr==nextaddr but size differs -> new
    let mut table =
        vec![LoadTable::full(a(0x2000), 4, 2), LoadTable::full(a(0x2008), 2, 3)];
    LoadTable::collapse_table(&mut table);

    assert_eq!(table.len(), 2, "size change forces a second table");
    assert_eq!(table[0], LoadTable::full(a(0x2000), 4, 2));
    assert_eq!(table[1], LoadTable::full(a(0x2008), 2, 3));
}

#[test]
fn collapse_table_unsorted_input_is_sorted_then_merged() {
    // Reverse-sorted, contiguous-once-sorted: the fast "issorted" probe fails on
    // the very first comparison, so the slow path sorts by address then merges.
    let m = build_manager();
    let ram = ram_of(&m);
    let a = |off: u64| Address::new(Rc::clone(&ram), off);

    let mut table = vec![
        LoadTable::full(a(0x3008), 4, 1),
        LoadTable::full(a(0x3004), 4, 1),
        LoadTable::full(a(0x3000), 4, 1),
    ];
    LoadTable::collapse_table(&mut table);

    assert_eq!(table.len(), 1, "sorted then merged into one contiguous run");
    assert_eq!(table[0].addr, a(0x3000), "merged table starts at lowest addr");
    assert_eq!(table[0].num, 3);
}

// ===========================================================================
// HUNT: IndexPair comparator totality / sort provenance
// ===========================================================================

#[test]
fn indexpair_total_order_position_then_index() {
    // operator< compares blockPosition first, addressIndex second; equal pairs
    // compare Equal.  A sort of mixed pairs must group by position, ascending
    // index within each position -- the order numIndicesByBlock/getIndexByBlock
    // rely on for their linear scan to match the C++ equal_range/lower_bound.
    let mut v = vec![
        IndexPair::new(2, 9),
        IndexPair::new(0, 5),
        IndexPair::new(2, 1),
        IndexPair::new(0, 1),
        IndexPair::new(1, 7),
    ];
    v.sort();
    let got: Vec<(i32, i32)> = v.iter().map(|p| (p.block_position, p.address_index)).collect();
    assert_eq!(
        got,
        vec![(0, 1), (0, 5), (1, 7), (2, 1), (2, 9)],
        "grouped by position, ascending index within position"
    );

    // Totality spot-checks: antisymmetry + reflexivity.
    let p = IndexPair::new(3, 4);
    assert_eq!(p.cmp(&p), std::cmp::Ordering::Equal);
    assert_eq!(IndexPair::new(1, 0).cmp(&IndexPair::new(1, 2)), std::cmp::Ordering::Less);
    assert_eq!(IndexPair::new(2, 0).cmp(&IndexPair::new(1, 99)), std::cmp::Ordering::Greater);
}

// ===========================================================================
// HUNT: JumpValuesRangeDefault iteration order (extra value LAST, not reversible)
// ===========================================================================

#[test]
fn default_range_iterates_base_then_extra_value_last() {
    // Base range [10,13) step 1 -> values 10,11,12, then the extra default value
    // 0x99 comes LAST and is flagged not-reversible.  Reproduces the C++
    // next()/getValue()/isReversible() sequencing (jumptable.cc:354-362).
    let mut jv = JumpValuesRangeDefault::new();
    jv.set_range(CircleRange::new(10, 13, 4, 1));
    jv.set_extra_value(0x99);

    assert_eq!(jv.get_size(), 4, "3 base values + 1 extra");
    assert!(jv.initialize_for_reading());
    assert_eq!(jv.get_value(), 10);
    assert!(jv.is_reversible(), "base values are reversible");

    // The bookkeeping of the LAST step is observable: once lastvalue is set the
    // iterator yields the extra value and reports not-reversible.  Drive that
    // state via the empty-base path (a genuinely empty CircleRange — under the
    // real port `[n,n)` step 1 is the FULL range, so emptiness is `new_empty()`):
    let mut last = JumpValuesRangeDefault::new();
    last.set_range(CircleRange::new_empty()); // empty base
    last.set_extra_value(0x99);
    assert!(last.initialize_for_reading());
    assert_eq!(last.get_value(), 0x99, "empty base -> straight to extra value");
    assert!(!last.is_reversible(), "the extra/default value is not reversible");
    assert_eq!(last.next().unwrap(), false, "no value after the extra (lastvalue)");
}

// ===========================================================================
// HUNT: JumpTable::decode label-ordering rule + NO_LABEL back-fill
// ===========================================================================

/// Hand-encode a `<jumptable>` element with two `<dest>` children, controlling
/// independently whether each carries an ATTRIB_LABEL.  This mirrors exactly
/// what `JumpTable::encode` emits (opaddress, then per-dest space attributes +
/// optional label), so it drives `JumpTable::decode` on the wire format.
fn encode_two_dests(m: &AddrSpaceManager, first_labeled: bool, second_labeled: bool) -> Vec<u8> {
    let ram = ram_of(m);
    let mut buf = Vec::new();
    let mut enc = PackedEncode::new(&mut buf);
    enc.open_element(&ELEM_JUMPTABLE);
    // opaddress
    Address::new(Rc::clone(&ram), 0x4000).encode(&mut enc).unwrap();
    // dest 0
    enc.open_element(&ELEM_DEST);
    ram.encode_attributes(&mut enc, 0x5000).unwrap();
    if first_labeled {
        enc.write_unsigned_integer(&ATTRIB_LABEL, 7);
    }
    enc.close_element(&ELEM_DEST);
    // dest 1
    enc.open_element(&ELEM_DEST);
    ram.encode_attributes(&mut enc, 0x5100).unwrap();
    if second_labeled {
        enc.write_unsigned_integer(&ATTRIB_LABEL, 8);
    }
    enc.close_element(&ELEM_DEST);
    enc.close_element(&ELEM_JUMPTABLE);
    buf
}

#[test]
fn decode_labeled_then_unlabeled_backfills_no_label() {
    // First dest labeled, second NO_LABEL: encode omits the second label; decode
    // sets missedlabel after the unlabeled entry and back-fills NO_LABEL so
    // label.len() == addresstable.len() (jumptable.cc:2993-2996).
    let m = build_manager();
    let buf = encode_two_dests(&m, true, false);

    let mut dec = PackedDecode::new(&m);
    dec.ingest_stream(&buf).unwrap();
    let mut jt = JumpTable::new(Address::default());
    jt.decode(&mut dec).unwrap();

    assert_eq!(jt.num_entries(), 2);
    assert!(jt.is_labelled());
    assert_eq!(jt.get_label_by_index(0), 7);
    assert_eq!(jt.get_label_by_index(1), NO_LABEL, "trailing entry back-filled NO_LABEL");
}

#[test]
fn decode_unlabeled_then_labeled_is_an_error() {
    // First dest NO_LABEL (omitted), second labeled: decode encounters a label
    // AFTER missedlabel was set -> hard error "Jumptable entries are missing
    // labels" (jumptable.cc:2968-2969).  Adversarial ordering inversion.
    let m = build_manager();
    let buf = encode_two_dests(&m, false, true);

    let mut dec = PackedDecode::new(&m);
    dec.ingest_stream(&buf).unwrap();
    let mut jt = JumpTable::new(Address::default());
    let err = jt.decode(&mut dec);
    assert!(err.is_err(), "labeled entry after an unlabeled one must be rejected");
}
