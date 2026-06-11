//! Porter in-module tests for `comment.rs`.
//!
//! Cover the `Comment` type-flag round-trips, the `CommentDatabaseInternal`
//! insert/uniq/dedup/clear/range semantics (the `CommentOrder` three-level sort,
//! the `addComment` uniq bump, the `addCommentNoDuplicate` backwards-walk), the
//! `<commentdb>` encode/decode round-trip, and the `CommentSorter` `Subsort`
//! ordering + walk cursors against a hand-built `SorterFuncdata`.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::marshal::{Decoder, IdRegistry, XmlDecode, XmlEncode};
use kuna_base::space::{addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace};
use kuna_base::types::{int4, uint4};

use super::*;

// ---------------------------------------------------------------------------
// Test fixtures
// ---------------------------------------------------------------------------

/// const(0), ram(1, 8-byte processor space).
fn manager() -> AddrSpaceManager {
    let mut m = AddrSpaceManager::new();
    m.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    m.insert_space(Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "ram",
        false,
        8,
        1,
        1,
        addrspace_flags::hasphysical,
        1,
        1,
    )))
    .unwrap();
    m
}

fn ram(m: &AddrSpaceManager, off: u64) -> Address {
    Address::new(Rc::clone(m.get_space(1).unwrap()), off)
}

fn registry() -> IdRegistry {
    let mut reg = IdRegistry::with_base_ids();
    register_ids(&mut reg);
    reg
}

// ---------------------------------------------------------------------------
// Comment type flags
// ---------------------------------------------------------------------------

#[test]
fn comment_type_string_roundtrip() {
    for (val, name) in [
        (comment_type::USER1, "user1"),
        (comment_type::USER2, "user2"),
        (comment_type::USER3, "user3"),
        (comment_type::HEADER, "header"),
        (comment_type::WARNING, "warning"),
        (comment_type::WARNINGHEADER, "warningheader"),
    ] {
        assert_eq!(Comment::decode_comment_type(val).unwrap(), name);
        assert_eq!(Comment::encode_comment_type(name.as_bytes()).unwrap(), val);
    }
}

#[test]
fn comment_type_unknown_errors() {
    assert!(Comment::decode_comment_type(64).is_err());
    assert!(Comment::decode_comment_type(0).is_err());
    let err = Comment::encode_comment_type(b"bogus").unwrap_err();
    assert!(err.to_string().contains("Unknown comment type: bogus"));
}

// ---------------------------------------------------------------------------
// CommentDatabaseInternal: insert / uniq / dedup / clear
// ---------------------------------------------------------------------------

#[test]
fn add_comment_bumps_uniq_for_same_address() {
    let m = manager();
    let fad = ram(&m, 0x1000);
    let ad = ram(&m, 0x1010);
    let mut db = CommentDatabaseInternal::new();

    // C++ addComment: first at an address gets uniq 0, the next at the same
    // (funcaddr, addr) gets the predecessor's uniq+1, and so on.
    db.add_comment(comment_type::USER1, &fad, &ad, b"first");
    db.add_comment(comment_type::USER1, &fad, &ad, b"second");
    db.add_comment(comment_type::USER1, &fad, &ad, b"third");

    let comms: Vec<_> = db.comments_for(&fad).collect();
    assert_eq!(comms.len(), 3);
    assert_eq!(comms[0].get_uniq(), 0);
    assert_eq!(comms[0].get_text(), b"first");
    assert_eq!(comms[1].get_uniq(), 1);
    assert_eq!(comms[1].get_text(), b"second");
    assert_eq!(comms[2].get_uniq(), 2);
    assert_eq!(comms[2].get_text(), b"third");
}

#[test]
fn add_comment_distinct_addresses_reset_uniq() {
    let m = manager();
    let fad = ram(&m, 0x1000);
    let mut db = CommentDatabaseInternal::new();
    db.add_comment(comment_type::USER1, &fad, &ram(&m, 0x1010), b"a");
    db.add_comment(comment_type::USER1, &fad, &ram(&m, 0x1020), b"b");
    // Each distinct address starts uniq at 0 (predecessor address differs).
    let comms: Vec<_> = db.comments_for(&fad).collect();
    assert_eq!(comms[0].get_uniq(), 0);
    assert_eq!(comms[1].get_uniq(), 0);
    // Ordered by addr (0x1010 before 0x1020).
    assert_eq!(comms[0].get_text(), b"a");
    assert_eq!(comms[1].get_text(), b"b");
}

#[test]
fn add_comment_no_duplicate_rejects_identical_text() {
    let m = manager();
    let fad = ram(&m, 0x1000);
    let ad = ram(&m, 0x1010);
    let mut db = CommentDatabaseInternal::new();

    assert!(db.add_comment_no_duplicate(comment_type::USER1, &fad, &ad, b"hello"));
    // Identical (addr, funcaddr, text): rejected.
    assert!(!db.add_comment_no_duplicate(comment_type::USER1, &fad, &ad, b"hello"));
    // Different text at same address: accepted, uniq bumps to 1.
    assert!(db.add_comment_no_duplicate(comment_type::USER1, &fad, &ad, b"world"));

    let comms: Vec<_> = db.comments_for(&fad).collect();
    assert_eq!(comms.len(), 2);
    assert_eq!(comms[0].get_uniq(), 0);
    assert_eq!(comms[1].get_uniq(), 1);
    assert_eq!(comms[1].get_text(), b"world");
}

#[test]
fn add_comment_no_duplicate_uniq_uses_highest_predecessor() {
    let m = manager();
    let fad = ram(&m, 0x1000);
    let ad = ram(&m, 0x1010);
    let mut db = CommentDatabaseInternal::new();
    // Build three distinct comments at the same address (uniq 0,1,2).
    db.add_comment(comment_type::USER1, &fad, &ad, b"a");
    db.add_comment(comment_type::USER1, &fad, &ad, b"b");
    db.add_comment(comment_type::USER1, &fad, &ad, b"c");
    // The backwards walk hits uniq 2 first and sets newcom.uniq = 3.
    assert!(db.add_comment_no_duplicate(comment_type::USER1, &fad, &ad, b"d"));
    let comms: Vec<_> = db.comments_for(&fad).collect();
    assert_eq!(comms.last().unwrap().get_uniq(), 3);
    assert_eq!(comms.last().unwrap().get_text(), b"d");
}

#[test]
fn clear_type_only_removes_matching_property_for_function() {
    let m = manager();
    let fad = ram(&m, 0x1000);
    let other_fad = ram(&m, 0x2000);
    let mut db = CommentDatabaseInternal::new();
    db.add_comment(comment_type::USER1, &fad, &ram(&m, 0x1010), b"u1");
    db.add_comment(comment_type::WARNING, &fad, &ram(&m, 0x1020), b"warn");
    db.add_comment(comment_type::HEADER, &fad, &ram(&m, 0x1030), b"hdr");
    // Same property bit but a different function — must survive.
    db.add_comment(comment_type::USER1, &other_fad, &ram(&m, 0x2010), b"other-u1");

    // Clear USER1|WARNING for `fad`.
    db.clear_type(&fad, comment_type::USER1 | comment_type::WARNING);

    let comms: Vec<_> = db.comments_for(&fad).collect();
    assert_eq!(comms.len(), 1);
    assert_eq!(comms[0].get_text(), b"hdr");
    // The other function's comment is untouched.
    let others: Vec<_> = db.comments_for(&other_fad).collect();
    assert_eq!(others.len(), 1);
    assert_eq!(others[0].get_text(), b"other-u1");
}

#[test]
fn comments_for_restricts_to_function() {
    let m = manager();
    let fad_a = ram(&m, 0x1000);
    let fad_b = ram(&m, 0x4000);
    let mut db = CommentDatabaseInternal::new();
    db.add_comment(comment_type::USER1, &fad_a, &ram(&m, 0x1010), b"a");
    db.add_comment(comment_type::USER1, &fad_b, &ram(&m, 0x4010), b"b");
    assert_eq!(db.comments_for(&fad_a).count(), 1);
    assert_eq!(db.comments_for(&fad_b).count(), 1);
    // A function with no comments yields an empty range (probe at m_minimal..m_maximal).
    assert_eq!(db.comments_for(&ram(&m, 0x9999)).count(), 0);
}

#[test]
fn delete_comment_removes_by_key() {
    let m = manager();
    let fad = ram(&m, 0x1000);
    let ad = ram(&m, 0x1010);
    let mut db = CommentDatabaseInternal::new();
    db.add_comment(comment_type::USER1, &fad, &ad, b"a");
    db.add_comment(comment_type::USER1, &fad, &ad, b"b");
    let key = CommentKey { funcaddr: fad.clone(), addr: ad.clone(), uniq: 0 };
    db.delete_comment(&key);
    let comms: Vec<_> = db.comments_for(&fad).collect();
    assert_eq!(comms.len(), 1);
    assert_eq!(comms[0].get_uniq(), 1);
}

#[test]
fn clear_empties_the_database() {
    let m = manager();
    let fad = ram(&m, 0x1000);
    let mut db = CommentDatabaseInternal::new();
    db.add_comment(comment_type::USER1, &fad, &ram(&m, 0x1010), b"a");
    db.clear();
    assert_eq!(db.comments_for(&fad).count(), 0);
}

// ---------------------------------------------------------------------------
// CommentSet ordering (CommentOrder three-level sort)
// ---------------------------------------------------------------------------

#[test]
fn ordering_is_funcaddr_then_addr_then_uniq() {
    let m = manager();
    let mut db = CommentDatabaseInternal::new();
    // Insert out of order across two functions and addresses.
    db.add_comment(comment_type::USER1, &ram(&m, 0x2000), &ram(&m, 0x2010), b"f2-a10");
    db.add_comment(comment_type::USER1, &ram(&m, 0x1000), &ram(&m, 0x1020), b"f1-a20");
    db.add_comment(comment_type::USER1, &ram(&m, 0x1000), &ram(&m, 0x1010), b"f1-a10");
    // beginComment/endComment for f1 must yield (0x1010, 0x1020) in order.
    let f1: Vec<_> = db.comments_for(&ram(&m, 0x1000)).map(|c| c.get_text().to_vec()).collect();
    assert_eq!(f1, vec![b"f1-a10".to_vec(), b"f1-a20".to_vec()]);
    let f2: Vec<_> = db.comments_for(&ram(&m, 0x2000)).map(|c| c.get_text().to_vec()).collect();
    assert_eq!(f2, vec![b"f2-a10".to_vec()]);
}

// ---------------------------------------------------------------------------
// commentdb encode/decode round-trip
// ---------------------------------------------------------------------------

#[test]
fn commentdb_encode_decode_roundtrip() {
    let m = manager();
    let fad = ram(&m, 0x1000);
    let mut db = CommentDatabaseInternal::new();
    db.add_comment(comment_type::USER1, &fad, &ram(&m, 0x1010), b"alpha");
    db.add_comment(comment_type::WARNINGHEADER, &fad, &ram(&m, 0x1010), b"beta");
    db.add_comment(comment_type::HEADER, &fad, &ram(&m, 0x1020), b"gamma");

    let mut buf = Vec::new();
    {
        let mut enc = XmlEncode::new(&mut buf);
        db.encode(&mut enc).unwrap();
    }

    let reg = registry();
    let mut dec = XmlDecode::new(&m, &reg);
    dec.ingest_stream(&buf).unwrap();
    let mut db2 = CommentDatabaseInternal::new();
    db2.decode(&mut dec).unwrap();

    let orig: Vec<_> = db.comments_for(&fad).collect();
    let back: Vec<_> = db2.comments_for(&fad).collect();
    assert_eq!(orig.len(), back.len());
    for (a, b) in orig.iter().zip(back.iter()) {
        assert_eq!(a.get_type(), b.get_type());
        assert_eq!(a.get_addr(), b.get_addr());
        assert_eq!(a.get_func_addr(), b.get_func_addr());
        assert_eq!(a.get_uniq(), b.get_uniq());
        assert_eq!(a.get_text(), b.get_text());
    }
}

#[test]
fn comment_empty_text_roundtrips() {
    // The C++ decode tolerates a <comment> with no <text> child; the encode
    // always writes one (possibly empty). Ensure an empty body round-trips.
    let m = manager();
    let fad = ram(&m, 0x1000);
    let mut db = CommentDatabaseInternal::new();
    db.add_comment(comment_type::USER2, &fad, &ram(&m, 0x1010), b"");

    let mut buf = Vec::new();
    {
        let mut enc = XmlEncode::new(&mut buf);
        db.encode(&mut enc).unwrap();
    }
    let reg = registry();
    let mut dec = XmlDecode::new(&m, &reg);
    dec.ingest_stream(&buf).unwrap();
    let mut db2 = CommentDatabaseInternal::new();
    db2.decode(&mut dec).unwrap();
    let back: Vec<_> = db2.comments_for(&fad).collect();
    assert_eq!(back.len(), 1);
    assert_eq!(back[0].get_text(), b"");
    assert_eq!(back[0].get_type(), comment_type::USER2);
}

// ---------------------------------------------------------------------------
// Subsort ordering (CommentSorter::Subsort::operator<)
// ---------------------------------------------------------------------------

#[test]
fn subsort_orders_index_then_order_then_pos() {
    let header = Subsort { index: -1, order: 0, pos: 0 };
    let blk0_a = Subsort { index: 0, order: 0, pos: 0 };
    let blk0_b = Subsort { index: 0, order: 0, pos: 1 };
    let blk0_c = Subsort { index: 0, order: 5, pos: 0 };
    let blk1 = Subsort { index: 1, order: 0, pos: 0 };
    let mut v = vec![blk1, blk0_c, header, blk0_b, blk0_a];
    v.sort();
    assert_eq!(v, vec![header, blk0_a, blk0_b, blk0_c, blk1]);
}

// ---------------------------------------------------------------------------
// CommentSorter walk against a hand-built SorterFuncdata
// ---------------------------------------------------------------------------

/// A minimal op-tree model: a list of `(addr_offset, block_index, order)` ops
/// sorted by address, plus a block-contains predicate, exercising the
/// `findPosition` machinery without the full `Funcdata`.
struct FakeFd {
    func_addr: Address,
    /// (op address, block index, seqnum order); kept sorted by address.
    ops: Vec<(Address, int4, uint4)>,
    /// (block index, lo, hi) inclusive address-offset spans on `ram`.
    block_spans: Vec<(int4, u64, u64)>,
}

impl FakeFd {
    fn op_index_at_or_after(&self, addr: &Address) -> OpProbe {
        for (i, (a, _, _)) in self.ops.iter().enumerate() {
            if a >= addr {
                return OpProbe::At(OpCursor(i));
            }
        }
        OpProbe::AtEnd
    }
}

impl SorterFuncdata for FakeFd {
    fn get_address(&self) -> &Address {
        &self.func_addr
    }
    fn op_tree_empty(&self) -> bool {
        self.ops.is_empty()
    }
    fn first_op_at_or_after(&self, addr: &Address) -> OpProbe {
        self.op_index_at_or_after(addr)
    }
    fn prev_op(&self, probe: &OpProbe) -> Option<OpCursor> {
        // The C++ `--opiter`, guarded by `opiter != beginOpAll()`.
        let lower = match probe {
            OpProbe::At(c) => c.0,
            OpProbe::AtEnd => self.ops.len(),
        };
        if lower == 0 {
            None // opiter == beginOpAll()
        } else {
            Some(OpCursor(lower - 1))
        }
    }
    fn op_block_info(&self, cursor: &OpCursor, addr: &Address) -> KunaResult<OpBlockInfo> {
        let (_, idx, order) = self.ops[cursor.0];
        let off = addr.get_offset();
        let mut contains = false;
        for (bidx, lo, hi) in &self.block_spans {
            if *bidx == idx && off >= *lo && off <= *hi {
                contains = true;
                break;
            }
        }
        Ok(OpBlockInfo { block_index: idx, order, block_contains_comment: contains })
    }
    fn op_addr(&self, cursor: &OpCursor) -> Address {
        self.ops[cursor.0].0.clone()
    }
}

#[test]
fn sorter_places_header_and_block_comments_and_walks() {
    let m = manager();
    let fad = ram(&m, 0x1000);
    // Block 0 spans [0x1000, 0x1020], block 1 spans [0x1030, 0x1040].
    let fd = FakeFd {
        func_addr: fad.clone(),
        ops: vec![
            (ram(&m, 0x1000), 0, 0),
            (ram(&m, 0x1010), 0, 1),
            (ram(&m, 0x1030), 1, 0),
        ],
        block_spans: vec![(0, 0x1000, 0x1020), (1, 0x1030, 0x1040)],
    };

    let mut db = CommentDatabaseInternal::new();
    // Header comment at the function address.
    db.add_comment(comment_type::HEADER, &fad, &ram(&m, 0x1000), b"hdr");
    // Comment in block 0 at the second op.
    db.add_comment(comment_type::USER1, &fad, &ram(&m, 0x1010), b"b0");
    // Comment in block 1.
    db.add_comment(comment_type::USER1, &fad, &ram(&m, 0x1030), b"b1");

    let mut sorter = CommentSorter::new();
    sorter
        .setup_function_list(
            comment_type::HEADER | comment_type::USER1,
            &fd,
            &db,
            false,
        )
        .unwrap();

    // Walk the header.
    sorter.setup_header(header_kind::HEADER_BASIC);
    let mut header_texts = Vec::new();
    while sorter.has_next() {
        header_texts.push(sorter.get_next().get_text().to_vec());
    }
    assert_eq!(header_texts, vec![b"hdr".to_vec()]);

    // Walk block 0; pick up everything in the block (no op landmark).
    sorter.setup_block_list(0);
    sorter.setup_op_list(None);
    let mut b0_texts = Vec::new();
    while sorter.has_next() {
        b0_texts.push(sorter.get_next().get_text().to_vec());
    }
    assert_eq!(b0_texts, vec![b"b0".to_vec()]);

    // Walk block 1.
    sorter.setup_block_list(1);
    sorter.setup_op_list(None);
    let mut b1_texts = Vec::new();
    while sorter.has_next() {
        b1_texts.push(sorter.get_next().get_text().to_vec());
    }
    assert_eq!(b1_texts, vec![b"b1".to_vec()]);
}

#[test]
fn sorter_tp_zero_collects_nothing_else_collects_all_nonzero_types() {
    // Faithful to the C++ `setupFunctionList`: `tp` is only an all-or-nothing
    // gate (`if (tp == 0) return;`).  It does NOT filter individual comments by
    // property bits — every comment with a non-zero `getType()` is collected
    // (the type filtering is the caller's responsibility, via the `tp` mask it
    // chooses to even call with).
    let m = manager();
    let fad = ram(&m, 0x1000);
    let fd = FakeFd {
        func_addr: fad.clone(),
        ops: vec![(ram(&m, 0x1000), 0, 0)],
        block_spans: vec![(0, 0x1000, 0x1020)],
    };
    let mut db = CommentDatabaseInternal::new();
    db.add_comment(comment_type::USER1, &fad, &ram(&m, 0x1010), b"u1");
    db.add_comment(comment_type::WARNING, &fad, &ram(&m, 0x1010), b"warn");

    // tp == 0 collects nothing (the C++ early return).
    let mut sorter = CommentSorter::new();
    sorter.setup_function_list(0, &fd, &db, false).unwrap();
    sorter.setup_block_list(0);
    sorter.setup_op_list(None);
    assert!(!sorter.has_next());

    // Any non-zero `tp` gate collects every non-zero-type comment of the
    // function (both u1 and warn), in container (uniq) order at the address.
    sorter.setup_function_list(comment_type::USER1, &fd, &db, false).unwrap();
    sorter.setup_block_list(0);
    sorter.setup_op_list(None);
    let mut texts = Vec::new();
    while sorter.has_next() {
        texts.push(sorter.get_next().get_text().to_vec());
    }
    assert_eq!(texts, vec![b"u1".to_vec(), b"warn".to_vec()]);
}

#[test]
fn sorter_op_landmark_stops_walk_early() {
    let m = manager();
    let fad = ram(&m, 0x1000);
    let fd = FakeFd {
        func_addr: fad.clone(),
        ops: vec![(ram(&m, 0x1000), 0, 0), (ram(&m, 0x1010), 0, 5)],
        block_spans: vec![(0, 0x1000, 0x1020)],
    };
    let mut db = CommentDatabaseInternal::new();
    // Comment associated with op at order 0 and another at order 5.
    db.add_comment(comment_type::USER1, &fad, &ram(&m, 0x1000), b"at-order-0");
    db.add_comment(comment_type::USER1, &fad, &ram(&m, 0x1010), b"at-order-5");

    let mut sorter = CommentSorter::new();
    sorter.setup_function_list(comment_type::USER1, &fd, &db, false).unwrap();
    sorter.setup_block_list(0);
    // Landmark at order 0: opstop = upper_bound(index=0, order=0, pos=MAX), so
    // only comments with order <= 0 are emitted.
    sorter.setup_op_list(Some(OpListLandmark { index: 0, order: 0 }));
    let mut first = Vec::new();
    while sorter.has_next() {
        first.push(sorter.get_next().get_text().to_vec());
    }
    assert_eq!(first, vec![b"at-order-0".to_vec()]);

    // Now advance the landmark to order 5: the remaining comment is emitted.
    sorter.setup_op_list(Some(OpListLandmark { index: 0, order: 5 }));
    let mut second = Vec::new();
    while sorter.has_next() {
        second.push(sorter.get_next().get_text().to_vec());
    }
    assert_eq!(second, vec![b"at-order-5".to_vec()]);
}

#[test]
fn sorter_unplaced_goes_to_header_when_enabled() {
    let m = manager();
    let fad = ram(&m, 0x1000);
    // Op tree exists but the comment's block does not contain the address and
    // no op is adjacent in a containing block -> unplaced.
    let fd = FakeFd {
        func_addr: fad.clone(),
        ops: vec![(ram(&m, 0x1000), 0, 0)],
        block_spans: vec![(0, 0x1000, 0x1005)], // narrow span; 0x2000 excised
    };
    let mut db = CommentDatabaseInternal::new();
    db.add_comment(comment_type::USER1, &fad, &ram(&m, 0x2000), b"orphan");

    // displayUnplaced = false: comment is dropped.
    let mut sorter = CommentSorter::new();
    sorter.setup_function_list(comment_type::USER1, &fd, &db, false).unwrap();
    sorter.setup_header(header_kind::HEADER_UNPLACED);
    assert!(!sorter.has_next());

    // displayUnplaced = true: comment surfaces in the unplaced header group.
    sorter.setup_function_list(comment_type::USER1, &fd, &db, true).unwrap();
    sorter.setup_header(header_kind::HEADER_UNPLACED);
    let mut texts = Vec::new();
    while sorter.has_next() {
        texts.push(sorter.get_next().get_text().to_vec());
    }
    assert_eq!(texts, vec![b"orphan".to_vec()]);
}

#[test]
fn sorter_no_ops_puts_comment_at_first_block() {
    let m = manager();
    let fad = ram(&m, 0x1000);
    let fd = FakeFd { func_addr: fad.clone(), ops: vec![], block_spans: vec![] };
    let mut db = CommentDatabaseInternal::new();
    db.add_comment(comment_type::USER1, &fad, &ram(&m, 0x1010), b"anywhere");

    let mut sorter = CommentSorter::new();
    sorter.setup_function_list(comment_type::USER1, &fd, &db, false).unwrap();
    // setBlock(0,0): emitted as the very first comment of block 0.
    sorter.setup_block_list(0);
    sorter.setup_op_list(None);
    let mut texts = Vec::new();
    while sorter.has_next() {
        texts.push(sorter.get_next().get_text().to_vec());
    }
    assert_eq!(texts, vec![b"anywhere".to_vec()]);
}
