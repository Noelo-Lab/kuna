//! Porter in-module tests for `jumptable.rs`.
//!
//! Exercises the structurally-complete, IR-independent surfaces of the port:
//!
//!   - [`LoadTable::collapse_table`] (sorted-contiguous fast path + the merge-
//!     and-compact slow path) and the `<loadtable>` encode/decode round-trip;
//!   - the [`PathMeld`] container construction (`set_single`/`set_path`/`append`),
//!     its accessors, `mark_paths`, `get_earliest_op`, `get_op_parent`, and
//!     `is_load_in_path` on hand-built op graphs;
//!   - [`GuardRecord::one_off_match`] / `matching_constants` on hand-built ops;
//!   - the [`JumpValuesRange`] / [`JumpValuesRangeDefault`] iterator bookkeeping
//!     (`truncate`, `get_size`, `initialize_for_reading`, default last-value);
//!   - [`IndexPair`] / [`PcodeOpNode`] ordering, and the [`JumpTable`] `<jumptable>`
//!     decode (addresses + labels + loadtables) and encode round-trip.
//!
//! Paths that need the genuinely-absent subsystems (CircleRange value-set ops,
//! emulation, the loader, TypeOp reverse-eval) are seam `Err` shells and are not
//! exercised here.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::marshal::{Decoder, PackedDecode, PackedEncode};
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};
use kuna_base::types::int4;

use crate::dtype::{type_metatype, Datatype};
use crate::op::pcodeop_flags;
use crate::context::{ArchContext, TypeOp};

use super::*;

// ---------------------------------------------------------------------------
// helpers (mirrors funcdata_op/tests.rs)
// ---------------------------------------------------------------------------

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

fn build_fd() -> Funcdata {
    let manage = build_manager();
    let glb = Rc::new(ArchContext::new(manage));
    let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
    let addr = Address::new(ram, 0x1000);
    Funcdata::new("func", "func", glb, addr, 0x10000000, 0x40).unwrap()
}

fn ram_space(fd: &Funcdata) -> Rc<AddrSpace> {
    Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap())
}

fn ram_of(m: &AddrSpaceManager) -> Rc<AddrSpace> {
    Rc::clone(m.get_space_by_name("ram").unwrap())
}

fn dt(size: int4) -> Rc<Datatype> {
    Rc::new(Datatype::new(size, type_metatype::TYPE_UNKNOWN))
}

fn opcode_flags(opc: OpCode) -> kuna_base::types::uint4 {
    match opc {
        OpCode::CPUI_BRANCH | OpCode::CPUI_CBRANCH | OpCode::CPUI_BRANCHIND => pcodeop_flags::branch,
        _ => 0,
    }
}

fn mk_op(fd: &mut Funcdata, inputs: int4, off: u64, opc: OpCode) -> OpId {
    let ram = ram_space(fd);
    let op = fd.new_op(inputs, Address::new(ram, off));
    fd.obank_mut().change_opcode(op, TypeOp::new(opc, opcode_flags(opc), format!("{opc:?}")));
    op
}

fn mk_vn(fd: &mut Funcdata, off: u64) -> VarnodeId {
    let ram = ram_space(fd);
    fd.vbank_mut().create(4, Address::new(ram, off), dt(4))
}

fn mk_const(fd: &mut Funcdata, val: u64) -> VarnodeId {
    fd.new_constant(4, val)
}

/// Create a function-input Varnode (input varnodes may be read by multiple ops;
/// a *free* varnode can have only one descendant, mirroring the C++ rule).
fn mk_input(fd: &mut Funcdata, off: u64) -> VarnodeId {
    let ram = ram_space(fd);
    let vn = fd.new_varnode(4, &Address::new(ram, off), None);
    fd.vbank_mut()
        .set_input(vn, &mut |_: &mut crate::varnode::VarnodeBank, _: VarnodeId, _: VarnodeId| {
            Ok(())
        })
        .unwrap()
}

// ---------------------------------------------------------------------------
// LoadTable::collapse_table
// ---------------------------------------------------------------------------

#[test]
fn collapse_table_empty_is_noop() {
    let mut table: Vec<LoadTable> = Vec::new();
    LoadTable::collapse_table(&mut table);
    assert!(table.is_empty());
}

#[test]
fn collapse_table_sorted_contiguous_fast_path() {
    let m = build_manager();
    let ram = ram_of(&m);
    // 3 entries, size 4, addresses 0x100, 0x104, 0x108 -- contiguous & sorted.
    let mut table = vec![
        LoadTable::full(Address::new(Rc::clone(&ram), 0x100), 4, 1),
        LoadTable::full(Address::new(Rc::clone(&ram), 0x104), 4, 1),
        LoadTable::full(Address::new(Rc::clone(&ram), 0x108), 4, 1),
    ];
    LoadTable::collapse_table(&mut table);
    // Fast path: collapse to a single entry whose num is the sum.
    assert_eq!(table.len(), 1);
    assert_eq!(table[0].addr, Address::new(Rc::clone(&ram), 0x100));
    assert_eq!(table[0].size, 4);
    assert_eq!(table[0].num, 3);
}

#[test]
fn collapse_table_unsorted_merge_path() {
    let m = build_manager();
    let ram = ram_of(&m);
    // Out of order: 0x108, 0x100, 0x104 (size 4 each). After sort the slow path
    // merges all three into one num=3 table at 0x100.
    let mut table = vec![
        LoadTable::full(Address::new(Rc::clone(&ram), 0x108), 4, 1),
        LoadTable::full(Address::new(Rc::clone(&ram), 0x100), 4, 1),
        LoadTable::full(Address::new(Rc::clone(&ram), 0x104), 4, 1),
    ];
    LoadTable::collapse_table(&mut table);
    assert_eq!(table.len(), 1);
    assert_eq!(table[0].addr, Address::new(Rc::clone(&ram), 0x100));
    assert_eq!(table[0].num, 3);
}

#[test]
fn collapse_table_two_disjoint_tables() {
    let m = build_manager();
    let ram = ram_of(&m);
    // Two clusters separated by a gap: {0x100,0x104} and {0x200,0x204}.
    let mut table = vec![
        LoadTable::full(Address::new(Rc::clone(&ram), 0x204), 4, 1),
        LoadTable::full(Address::new(Rc::clone(&ram), 0x100), 4, 1),
        LoadTable::full(Address::new(Rc::clone(&ram), 0x200), 4, 1),
        LoadTable::full(Address::new(Rc::clone(&ram), 0x104), 4, 1),
    ];
    LoadTable::collapse_table(&mut table);
    assert_eq!(table.len(), 2);
    assert_eq!(table[0].addr, Address::new(Rc::clone(&ram), 0x100));
    assert_eq!(table[0].num, 2);
    assert_eq!(table[1].addr, Address::new(Rc::clone(&ram), 0x200));
    assert_eq!(table[1].num, 2);
}

#[test]
fn loadtable_encode_decode_roundtrip() {
    let m = build_manager();
    let ram = ram_of(&m);
    let lt = LoadTable::full(Address::new(Rc::clone(&ram), 0x1234), 4, 7);

    let mut buf = Vec::new();
    {
        let mut enc = PackedEncode::new(&mut buf);
        lt.encode(&mut enc).unwrap();
    }
    let mut dec = PackedDecode::new(&m);
    dec.ingest_stream(&buf).unwrap();
    let mut decoded = LoadTable::single(Address::default(), 0);
    decoded.decode(&mut dec).unwrap();

    assert_eq!(decoded.addr, Address::new(Rc::clone(&ram), 0x1234));
    assert_eq!(decoded.size, 4);
    assert_eq!(decoded.num, 7);
}

// ---------------------------------------------------------------------------
// PcodeOpNode / IndexPair ordering
// ---------------------------------------------------------------------------

#[test]
fn indexpair_orders_by_position_then_index() {
    let a = IndexPair::new(1, 5);
    let b = IndexPair::new(1, 6);
    let c = IndexPair::new(2, 0);
    assert!(a < b); // same position, lower index first
    assert!(b < c); // lower position first
    let mut v = vec![c, b, a];
    v.sort();
    assert_eq!(v, vec![a, b, c]);
}

// ---------------------------------------------------------------------------
// PathMeld construction + accessors
// ---------------------------------------------------------------------------

#[test]
fn pathmeld_set_single_and_accessors() {
    let mut fd = build_fd();
    let op = mk_op(&mut fd, 1, 0x100, OpCode::CPUI_COPY);
    let vn = mk_vn(&mut fd, 0x10);
    fd.op_set_input(op, vn, 0).unwrap();

    let mut pm = PathMeld::new();
    assert!(pm.empty());
    pm.set_single(op, vn);
    assert!(!pm.empty());
    assert_eq!(pm.num_common_varnode(), 1);
    assert_eq!(pm.num_ops(), 1);
    assert_eq!(pm.get_varnode(0), vn);
    assert_eq!(pm.get_op(0), Some(op));
    assert_eq!(pm.get_op_parent(0), vn);
    assert_eq!(pm.get_earliest_op(0), Some(op));
}

#[test]
fn pathmeld_set_path_links_op_inputs() {
    // Build a 2-edge path: add(out) <- mult; the path nodes index op/slot pairs.
    let mut fd = build_fd();
    let add = mk_op(&mut fd, 2, 0x100, OpCode::CPUI_INT_ADD);
    let mult = mk_op(&mut fd, 2, 0x100, OpCode::CPUI_INT_MULT);
    let v_outer = mk_vn(&mut fd, 0x10); // input slot 0 of add
    let v_inner = mk_vn(&mut fd, 0x20); // input slot 0 of mult
    fd.op_set_input(add, v_outer, 0).unwrap();
    fd.op_set_input(mult, v_inner, 0).unwrap();

    let path = vec![PcodeOpNode::new(add, 0), PcodeOpNode::new(mult, 0)];
    let mut pm = PathMeld::new();
    pm.set_path(&fd, &path);

    // common_vn[i] is the input varnode of path[i] at its slot.
    assert_eq!(pm.num_common_varnode(), 2);
    assert_eq!(pm.get_varnode(0), v_outer);
    assert_eq!(pm.get_varnode(1), v_inner);
    assert_eq!(pm.get_op(0), Some(add));
    assert_eq!(pm.get_op(1), Some(mult));
    // root_vn is the index i, so getOpParent(i) == commonVn[i].
    assert_eq!(pm.get_op_parent(0), v_outer);
    assert_eq!(pm.get_op_parent(1), v_inner);
}

#[test]
fn pathmeld_append_renumbers_roots() {
    let mut fd = build_fd();
    // Container A: single op/vn at index 0.
    let op_a = mk_op(&mut fd, 1, 0x100, OpCode::CPUI_COPY);
    let vn_a = mk_vn(&mut fd, 0x10);
    fd.op_set_input(op_a, vn_a, 0).unwrap();
    let mut a = PathMeld::new();
    a.set_single(op_a, vn_a);

    // Container B (the prefix to prepend): single op/vn.
    let op_b = mk_op(&mut fd, 1, 0x200, OpCode::CPUI_COPY);
    let vn_b = mk_vn(&mut fd, 0x20);
    fd.op_set_input(op_b, vn_b, 0).unwrap();
    let mut b = PathMeld::new();
    b.set_single(op_b, vn_b);

    // a.append(b): b's nodes go first; a's nodes shift, roots += b.numCommonVarnode().
    a.append(&b);
    assert_eq!(a.num_common_varnode(), 2);
    assert_eq!(a.num_ops(), 2);
    assert_eq!(a.get_varnode(0), vn_b); // prefix first
    assert_eq!(a.get_varnode(1), vn_a);
    assert_eq!(a.get_op(0), Some(op_b));
    assert_eq!(a.get_op(1), Some(op_a));
    // a's op (now at index 1) had root 0; renumbered to 0 + 1 = 1.
    assert_eq!(a.get_op_parent(1), vn_a);
}

#[test]
fn pathmeld_mark_paths_marks_through_start() {
    let mut fd = build_fd();
    // Two ops, both common to a 2-varnode path (roots 0 and 1).
    let op0 = mk_op(&mut fd, 1, 0x100, OpCode::CPUI_INT_ADD);
    let op1 = mk_op(&mut fd, 1, 0x100, OpCode::CPUI_INT_MULT);
    let v0 = mk_vn(&mut fd, 0x10);
    let v1 = mk_vn(&mut fd, 0x20);
    fd.op_set_input(op0, v0, 0).unwrap();
    fd.op_set_input(op1, v1, 0).unwrap();
    let path = vec![PcodeOpNode::new(op0, 0), PcodeOpNode::new(op1, 0)];
    let mut pm = PathMeld::new();
    pm.set_path(&fd, &path);

    // markPaths(true, startVarnode=0): marks ops from index 0 down to the op
    // whose root_vn == 0 (op0). Only op0 should be marked.
    pm.mark_paths(&mut fd, true, 0);
    assert!(fd.obank().get(op0).unwrap().is_mark());
    assert!(!fd.obank().get(op1).unwrap().is_mark());

    // markPaths(true, startVarnode=1): marks down through op1 (root 1) -> both.
    pm.mark_paths(&mut fd, true, 1);
    assert!(fd.obank().get(op0).unwrap().is_mark());
    assert!(fd.obank().get(op1).unwrap().is_mark());

    // Unmark everything via the start at root 1.
    pm.mark_paths(&mut fd, false, 1);
    assert!(!fd.obank().get(op0).unwrap().is_mark());
    assert!(!fd.obank().get(op1).unwrap().is_mark());
}

// ---------------------------------------------------------------------------
// GuardRecord static logic
// ---------------------------------------------------------------------------

#[test]
fn one_off_match_same_op_and_const() {
    let mut fd = build_fd();
    // Two INT_AND ops with the same input-0 varnode and equal constant input-1.
    let shared = mk_input(&mut fd, 0x10);
    let c1 = mk_const(&mut fd, 0xff);
    let c2 = mk_const(&mut fd, 0xff);
    let op1 = mk_op(&mut fd, 2, 0x100, OpCode::CPUI_INT_AND);
    let op2 = mk_op(&mut fd, 2, 0x200, OpCode::CPUI_INT_AND);
    fd.op_set_input(op1, shared, 0).unwrap();
    fd.op_set_input(op1, c1, 1).unwrap();
    fd.op_set_input(op2, shared, 0).unwrap();
    fd.op_set_input(op2, c2, 1).unwrap();

    assert_eq!(GuardRecord::one_off_match(&fd, op1, op2), 1);
}

#[test]
fn one_off_match_diff_const_no_match() {
    let mut fd = build_fd();
    let shared = mk_input(&mut fd, 0x10);
    let c1 = mk_const(&mut fd, 0xff);
    let c2 = mk_const(&mut fd, 0x0f); // different constant
    let op1 = mk_op(&mut fd, 2, 0x100, OpCode::CPUI_INT_AND);
    let op2 = mk_op(&mut fd, 2, 0x200, OpCode::CPUI_INT_AND);
    fd.op_set_input(op1, shared, 0).unwrap();
    fd.op_set_input(op1, c1, 1).unwrap();
    fd.op_set_input(op2, shared, 0).unwrap();
    fd.op_set_input(op2, c2, 1).unwrap();

    assert_eq!(GuardRecord::one_off_match(&fd, op1, op2), 0);
}

#[test]
fn one_off_match_diff_opcode_no_match() {
    let mut fd = build_fd();
    let shared = mk_input(&mut fd, 0x10);
    let c1 = mk_const(&mut fd, 0xff);
    let c2 = mk_const(&mut fd, 0xff);
    let op1 = mk_op(&mut fd, 2, 0x100, OpCode::CPUI_INT_AND);
    let op2 = mk_op(&mut fd, 2, 0x200, OpCode::CPUI_INT_OR); // different opcode
    fd.op_set_input(op1, shared, 0).unwrap();
    fd.op_set_input(op1, c1, 1).unwrap();
    fd.op_set_input(op2, shared, 0).unwrap();
    fd.op_set_input(op2, c2, 1).unwrap();

    assert_eq!(GuardRecord::one_off_match(&fd, op1, op2), 0);
}

#[test]
fn one_off_match_unhandled_opcode_no_match() {
    let mut fd = build_fd();
    // CPUI_COPY is not in the oneOffMatch switch, so even identical ops -> 0.
    let shared = mk_input(&mut fd, 0x10);
    let op1 = mk_op(&mut fd, 1, 0x100, OpCode::CPUI_COPY);
    let op2 = mk_op(&mut fd, 1, 0x200, OpCode::CPUI_COPY);
    fd.op_set_input(op1, shared, 0).unwrap();
    fd.op_set_input(op2, shared, 0).unwrap();
    assert_eq!(GuardRecord::one_off_match(&fd, op1, op2), 0);
}

// ---------------------------------------------------------------------------
// JumpBasic static helpers
// ---------------------------------------------------------------------------

#[test]
fn ispoint_rejects_constant_and_accepts_register() {
    let mut fd = build_fd();
    let c = mk_const(&mut fd, 7);
    let r = mk_vn(&mut fd, 0x10);
    assert!(!JumpBasic::ispoint(&fd, c)); // constant -> not a switch var
    assert!(JumpBasic::ispoint(&fd, r)); // plain register -> candidate
}

#[test]
fn isprune_unwritten_is_pruned() {
    let mut fd = build_fd();
    let r = mk_vn(&mut fd, 0x10); // never given a def -> not written
    assert!(JumpBasic::isprune(&fd, r));
}

#[test]
fn duplicate_varnodes_detects_uniformity() {
    let mut fd = build_fd();
    let a = mk_vn(&mut fd, 0x10);
    let b = mk_vn(&mut fd, 0x20);
    assert!(JumpBasic::duplicate_varnodes(&[a, a, a]));
    assert!(!JumpBasic::duplicate_varnodes(&[a, b, a]));
}

// ---------------------------------------------------------------------------
// JumpValuesRange iterator bookkeeping
// ---------------------------------------------------------------------------

#[test]
fn jumpvaluesrange_size_and_init_over_explicit_range() {
    // Range [0, 8) step 1, 4-byte mask -> size 8.
    // (Now uses the real CircleRange constructor `new` — w6-s5-rangeutil.)
    let mut jv = JumpValuesRange::new();
    jv.set_range(CircleRange::new(0, 8, 4, 1));
    assert_eq!(jv.get_size(), 8);
    assert!(jv.initialize_for_reading());
    assert_eq!(jv.get_value(), 0); // first value is the range minimum
    assert!(jv.is_reversible());
}

#[test]
fn jumpvaluesrange_empty_does_not_initialize() {
    // A genuinely empty range -> size 0 -> initializeForReading returns false.
    // (In real CircleRange semantics `[n,n)` step 1 is the FULL range, not empty;
    // emptiness is the explicit `new_empty()` state — w6-s5-rangeutil.)
    let mut jv = JumpValuesRange::new();
    jv.set_range(CircleRange::new_empty());
    assert_eq!(jv.get_size(), 0);
    assert!(!jv.initialize_for_reading());
}

#[test]
fn jumpvaluesrange_truncate_resets_extent() {
    // Range [0,16) step 1; truncate to 4 elements -> size 4.
    let mut jv = JumpValuesRange::new();
    jv.set_range(CircleRange::new(0, 16, 4, 1));
    assert_eq!(jv.get_size(), 16);
    JumpValues::truncate(&mut jv, 4);
    assert_eq!(jv.get_size(), 4);
}

#[test]
fn jumpvaluesrangedefault_size_is_base_plus_one() {
    let mut jv = JumpValuesRangeDefault::new();
    jv.set_range(CircleRange::new(0, 8, 4, 1));
    jv.set_extra_value(0xdead);
    // getSize == base range size + 1 (the extra default value).
    assert_eq!(jv.get_size(), 9);
}

#[test]
fn jumpvaluesrangedefault_empty_range_initializes_to_extra() {
    let mut jv = JumpValuesRangeDefault::new();
    jv.set_range(CircleRange::new_empty()); // genuinely empty base range
    jv.set_extra_value(0x42);
    // With an empty base range, initializeForReading yields the extra value and
    // marks lastvalue (so it is not reversible).
    assert!(jv.initialize_for_reading());
    assert_eq!(jv.get_value(), 0x42);
    assert!(!jv.is_reversible()); // the extra value is not reversible
}

// ---------------------------------------------------------------------------
// JumpTable <jumptable> decode + encode round-trip
// ---------------------------------------------------------------------------

/// Build a small recovered JumpTable in-memory by decoding a hand-encoded
/// `<jumptable>` element with two labeled destinations and one loadtable.
fn encode_sample_jumptable(m: &AddrSpaceManager, buf: &mut Vec<u8>) {
    let ram = ram_of(m);
    // Construct directly, then encode (mirrors what a recovered table holds).
    let mut jt = JumpTable::new(Address::new(Rc::clone(&ram), 0x4000));
    jt.set_display_format(0);
    // addresstable + labels (two entries, both labeled).
    jt.addresstable.push(Address::new(Rc::clone(&ram), 0x5000));
    jt.addresstable.push(Address::new(Rc::clone(&ram), 0x5100));
    jt.label.push(0);
    jt.label.push(1);
    // one loadtable record.
    jt.loadpoints.push(LoadTable::full(Address::new(Rc::clone(&ram), 0x6000), 4, 2));

    let mut enc = PackedEncode::new(buf);
    jt.encode(&mut enc).unwrap();
}

#[test]
fn jumptable_decode_recovers_addresses_labels_loads() {
    let m = build_manager();
    let ram = ram_of(&m);
    let mut buf = Vec::new();
    encode_sample_jumptable(&m, &mut buf);

    let mut dec = PackedDecode::new(&m);
    dec.ingest_stream(&buf).unwrap();
    let mut jt = JumpTable::new(Address::default());
    jt.decode(&mut dec).unwrap();

    assert!(jt.is_recovered());
    assert!(jt.is_labelled());
    assert_eq!(jt.num_entries(), 2);
    assert_eq!(jt.get_op_address(), &Address::new(Rc::clone(&ram), 0x4000));
    assert_eq!(jt.get_address_by_index(0), Address::new(Rc::clone(&ram), 0x5000));
    assert_eq!(jt.get_address_by_index(1), Address::new(Rc::clone(&ram), 0x5100));
    assert_eq!(jt.get_label_by_index(0), 0);
    assert_eq!(jt.get_label_by_index(1), 1);
    assert_eq!(jt.load_points().len(), 1);
    assert_eq!(jt.load_points()[0].num, 2);
}

#[test]
fn jumptable_encode_decode_roundtrip() {
    let m = build_manager();
    let ram = ram_of(&m);

    // Decode the sample, re-encode, and decode again: fields must be stable.
    let mut buf1 = Vec::new();
    encode_sample_jumptable(&m, &mut buf1);
    let mut dec1 = PackedDecode::new(&m);
    dec1.ingest_stream(&buf1).unwrap();
    let mut jt1 = JumpTable::new(Address::default());
    jt1.decode(&mut dec1).unwrap();

    let mut buf2 = Vec::new();
    {
        let mut enc = PackedEncode::new(&mut buf2);
        jt1.encode(&mut enc).unwrap();
    }
    let mut dec2 = PackedDecode::new(&m);
    dec2.ingest_stream(&buf2).unwrap();
    let mut jt2 = JumpTable::new(Address::default());
    jt2.decode(&mut dec2).unwrap();

    assert_eq!(jt2.get_op_address(), &Address::new(Rc::clone(&ram), 0x4000));
    assert_eq!(jt2.num_entries(), 2);
    assert_eq!(jt2.get_address_by_index(0), Address::new(Rc::clone(&ram), 0x5000));
    assert_eq!(jt2.get_address_by_index(1), Address::new(Rc::clone(&ram), 0x5100));
    assert_eq!(jt2.get_label_by_index(0), 0);
    assert_eq!(jt2.get_label_by_index(1), 1);
    assert_eq!(jt2.load_points().len(), 1);
}

#[test]
fn jumptable_decode_missing_label_after_labeled_is_error() {
    // Manually craft a <jumptable> with a labeled dest followed by an unlabeled
    // one: the C++ raises "Jumptable entries are missing labels" only when an
    // unlabeled entry PRECEDES a labeled one. Here we check the inverse holds:
    // a labeled-then-unlabeled sequence decodes fine (missedlabel only trips on
    // a label appearing AFTER a missing one).
    let m = build_manager();
    let ram = ram_of(&m);
    let mut jt = JumpTable::new(Address::new(Rc::clone(&ram), 0x4000));
    jt.addresstable.push(Address::new(Rc::clone(&ram), 0x5000));
    jt.addresstable.push(Address::new(Rc::clone(&ram), 0x5100));
    jt.label.push(7); // only the first entry is labeled
    let mut buf = Vec::new();
    {
        let mut enc = PackedEncode::new(&mut buf);
        jt.encode(&mut enc).unwrap();
    }
    let mut dec = PackedDecode::new(&m);
    dec.ingest_stream(&buf).unwrap();
    let mut decoded = JumpTable::new(Address::default());
    // First entry has a label, second does not -> the "label after missing"
    // rule is not violated, so decode succeeds; trailing labels are padded with
    // NO_LABEL up to addresstable size.
    decoded.decode(&mut dec).unwrap();
    assert_eq!(decoded.num_entries(), 2);
    assert_eq!(decoded.get_label_by_index(0), 7);
    assert_eq!(decoded.get_label_by_index(1), NO_LABEL);
}

#[test]
fn label_rows_pair_values_with_rows_in_label_order() {
    let mut range = JumpValuesRange::new();
    range.set_range(CircleRange::new(4, 10, 4, 1));
    let mut basic = JumpBasicModel::new();
    basic.jrange = Some(Box::new(range));
    let rows = LabelRows::new(&basic, &vec![Address::default(); 3], None).unwrap();
    assert_eq!(rows.values, vec![Some(4), Some(5), Some(6)]);

    let mut with_default = JumpValuesRangeDefault::new();
    with_default.set_range(CircleRange::new(0, 2, 4, 1));
    with_default.set_extra_value(9);
    let mut model2 = JumpBasicModel::new_model2();
    model2.jrange = Some(Box::new(with_default));
    let rows = LabelRows::new(&model2, &vec![Address::default(); 4], None).unwrap();
    assert_eq!(rows.values, vec![Some(0), Some(1), None, None]);
}

#[test]
fn label_rows_judge_a_value_by_the_entries_it_reads() {
    let m = build_manager();
    let ram = ram_of(&m);
    let at = |off: u64, size: int4| LoadTable::single(Address::new(Rc::clone(&ram), off), size);
    let table = |off: u64, size: int4, num: int4| {
        LoadTable::full(Address::new(Rc::clone(&ram), off), size, num)
    };
    let mut basic = JumpBasicModel::new();
    let mut range = JumpValuesRange::new();
    range.set_range(CircleRange::new(0, 2, 4, 1));
    basic.jrange = Some(Box::new(range));
    let addrs = vec![Address::default(); 2];
    let read = RowLoads {
        entries: vec![table(0x100, 1, 2), table(0x208, 8, 2)],
        reached: vec![table(0x100, 1, 4), table(0x208, 8, 2), table(0x300, 8, 1)],
        depth: 2,
    };
    let rows = LabelRows::new(&basic, &addrs, Some(&read)).unwrap();
    assert!(rows.may_be_dispatched(&[at(0x101, 1), at(0x210, 8)]));
    assert!(rows.may_be_dispatched(&[at(0x210, 8)]));
    assert!(rows.may_be_dispatched(&[at(0x103, 1), at(0x210, 8)]));
    assert!(!rows.may_be_dispatched(&[at(0x104, 1), at(0x210, 8)]));
    assert!(!rows.may_be_dispatched(&[at(0x103, 1), at(0x300, 8)]));
    assert!(!rows.may_be_dispatched(&[at(0x100, 1), at(0x218, 8)]));
    assert!(!rows.may_be_dispatched(&[at(0x200, 8)]));
    assert!(!rows.may_be_dispatched(&[at(0x20c, 8)]));
    assert!(!rows.may_be_dispatched(&[at(0x208, 4)]));
    assert!(rows.may_be_dispatched(&[]));
    assert!(rows.may_be_dispatched(&[at(0x500, 2), at(0x400, 1), at(0x210, 8)]));
    assert!(!rows.may_be_dispatched(&[at(0x500, 2), at(0x100, 1), at(0x218, 8)]));

    let empty = RowLoads { entries: Vec::new(), reached: Vec::new(), depth: 1 };
    let empty = LabelRows::new(&basic, &addrs, Some(&empty)).unwrap();
    assert!(empty.may_be_dispatched(&[at(0x400, 8)]));
    let unrecorded = LabelRows::new(&basic, &addrs, None).unwrap();
    assert!(unrecorded.may_be_dispatched(&[at(0x400, 8)]));
}

#[test]
fn kept_row_loads_follow_the_sanity_truncation_and_the_dispatched_cut_rows() {
    let m = build_manager();
    let ram = ram_of(&m);
    let at = |off: u64, size: int4| LoadTable::single(Address::new(Rc::clone(&ram), off), size);
    let table = |off: u64, num: int4| LoadTable::full(Address::new(Rc::clone(&ram), off), 4, num);
    let loads = vec![at(0x10, 4), at(0x14, 4), at(0x18, 4), at(0x1c, 4)];
    let kept = JumpTable::kept_row_loads(loads.clone(), &[1, 2, 3, 4], 2, &[]).unwrap();
    assert_eq!(kept.entries, vec![table(0x10, 2)]);
    assert_eq!(kept.reached, vec![table(0x10, 4)]);
    assert_eq!(kept.depth, 1);
    let all = JumpTable::kept_row_loads(loads.clone(), &[1, 2, 3, 4], 2, &[true, true]).unwrap();
    assert_eq!(all.entries, all.reached);
    assert_eq!(all.depth, 1);
    let guarded =
        JumpTable::kept_row_loads(loads.clone(), &[1, 2, 3, 4], 2, &[false, true]).unwrap();
    assert_eq!(guarded.entries, vec![table(0x10, 2), table(0x1c, 1)]);
    let sent_away =
        JumpTable::kept_row_loads(loads.clone(), &[1, 2, 3, 4], 2, &[false, false]).unwrap();
    assert_eq!(sent_away.entries, kept.entries);
    let paired = JumpTable::kept_row_loads(loads.clone(), &[2, 4], 2, &[]).unwrap();
    assert_eq!(paired.entries, vec![table(0x10, 4)]);
    assert_eq!(paired.depth, 2);
    assert!(JumpTable::kept_row_loads(loads.clone(), &[], 2, &[]).is_none());
    assert!(JumpTable::kept_row_loads(loads, &[1, 2], 0, &[true]).is_none());
}

#[test]
fn default_addresses_follow_the_default_block() {
    let m = build_manager();
    let ram = ram_of(&m);
    let at = |off: u64| Address::new(Rc::clone(&ram), off);
    let mut jt = JumpTable::new(at(0x1000));
    jt.addresstable = vec![at(0x10), at(0x20), at(0x30), at(0x20), at(0x10)];
    jt.block2addr = vec![
        IndexPair::new(0, 0),
        IndexPair::new(0, 4),
        IndexPair::new(1, 1),
        IndexPair::new(1, 3),
        IndexPair::new(2, 2),
    ];
    assert!(jt.default_addresses().is_empty());
    jt.set_default_block(0);
    assert_eq!(jt.default_addresses(), vec![at(0x10), at(0x10)]);
    jt.set_default_block(2);
    assert_eq!(jt.default_addresses(), vec![at(0x30)]);
}
