//! Adversarial verifier tests for item `w4-p0-override-comment` (round 1).
//!
//! These target the hunt-list spots flagged as most fragile for the
//! `override.{cc,hh}` + `comment.{cc,hh}` port:
//!
//!   - `CommentDatabaseInternal::addComment` predecessor probe at/before the
//!     container begin (C++ `if (iter != begin()) --iter;` else iter stays at
//!     begin) vs the Rust `range(..probe).next_back()` (which yields `None`);
//!   - `addCommentNoDuplicate` backwards-walk `break` boundary: a same-address
//!     run must NOT pick up uniq from a comment at a *lower* address;
//!   - `clearType` half-open range bracketing (m_minimal..m_maximal) across two
//!     functions — only the named function's matching-type comments go;
//!   - the `CommentOrder`/`func_key_range` cross-space sentinel ordering;
//!   - `Override::decode` flow branch rejecting an *invalid* address (the C++
//!     `addr.isInvalid()` arm — the porter only covered the NONE-type arm);
//!   - `Override::getFlowOverride` returning NONE for an empty store and the
//!     `Override::decode` deadcodedelay vector growth replaying through a high
//!     space index.
//!
//! Driven through the public `overrides`/`comment` API so the diff stands on
//! its own against the C++ source.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::marshal::{Decoder, IdRegistry, XmlDecode};
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace,
};

use kuna_decomp::comment::{comment_type, CommentDatabase, CommentDatabaseInternal};
use kuna_decomp::overrides::{flow_type, register_ids, Override};

// ---------------------------------------------------------------------------
// Fixtures: const(0), ram(1, 8-byte processor), io(2, 8-byte processor).
// ---------------------------------------------------------------------------

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
    m.insert_space(Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "io",
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

fn ram(m: &AddrSpaceManager, off: u64) -> Address {
    Address::new(Rc::clone(m.get_space(1).unwrap()), off)
}
fn io(m: &AddrSpaceManager, off: u64) -> Address {
    Address::new(Rc::clone(m.get_space(2).unwrap()), off)
}

fn registry() -> IdRegistry {
    let mut reg = IdRegistry::with_base_ids();
    register_ids(&mut reg);
    reg
}

// ---------------------------------------------------------------------------
// 1. addComment when the new comment becomes the container's *first* element.
//
// C++ addComment: `iter = lower_bound(probe@uniq=65535)`; `if (iter != begin())
// --iter;`.  When the probe sorts at/before every element, `iter` stays at
// begin(); the `if (iter != end())` body then inspects the FIRST element of the
// whole set — but it can only bump uniq when that element shares (addr,fad) AND
// has uniq >= 65535, which never holds for real comments.  So the practical
// answer is uniq = 0.  The Rust `range(..probe).next_back()` returns None for
// this case and likewise leaves uniq = 0.  This test pins that both the
// "no predecessor at all" path and the "predecessor at a different, lower
// funcaddr" path yield uniq 0 (no spurious bump from a smaller key).
// ---------------------------------------------------------------------------
#[test]
fn add_comment_first_element_and_lower_funcaddr_predecessor_keep_uniq_zero() {
    let m = manager();
    let mut db = CommentDatabaseInternal::new();

    // Insert a comment for a HIGH funcaddr first; then one for a LOW funcaddr.
    // The low-funcaddr insert becomes the new container begin (no predecessor).
    let hi = ram(&m, 0x9000);
    let lo = ram(&m, 0x1000);
    db.add_comment(comment_type::USER1, &hi, &ram(&m, 0x9010), b"hi");
    // No element precedes the low-funcaddr probe: uniq must be 0.
    db.add_comment(comment_type::USER1, &lo, &ram(&m, 0x1010), b"lo");

    let los: Vec<_> = db.comments_for(&lo).collect();
    assert_eq!(los.len(), 1);
    assert_eq!(los[0].get_uniq(), 0, "first-element insert must keep uniq 0");

    // Now a second comment for the LOW funcaddr at a *higher* address than the
    // first low comment.  Its predecessor (the lo@0x1010 comment) has a
    // DIFFERENT addr, so the (addr,fad) guard fails and uniq stays 0 (a distinct
    // address always restarts the uniq counter).
    db.add_comment(comment_type::USER1, &lo, &ram(&m, 0x1020), b"lo2");
    let los: Vec<_> = db.comments_for(&lo).collect();
    assert_eq!(los.len(), 2);
    assert_eq!(los[0].get_uniq(), 0);
    assert_eq!(los[1].get_uniq(), 0, "different address restarts uniq at 0");
    // And the high funcaddr comment is untouched.
    assert_eq!(db.comments_for(&hi).count(), 1);
}

// ---------------------------------------------------------------------------
// 2. addCommentNoDuplicate backwards-walk `break` boundary.
//
// C++ walks predecessors of lower_bound while each shares (addr,fad); it
// `break`s at the first non-matching one.  A comment at a *lower different
// address* for the same function must therefore NOT contribute its uniq.  Build
// two distinct addresses for one function, fill the higher one to uniq 2, and
// verify a fresh no-dup insert at the higher address bumps to 3 — never picking
// up the lower address's comment.
// ---------------------------------------------------------------------------
#[test]
fn add_comment_no_duplicate_walk_breaks_at_different_address() {
    let m = manager();
    let fad = ram(&m, 0x1000);
    let lower = ram(&m, 0x1010);
    let higher = ram(&m, 0x1020);
    let mut db = CommentDatabaseInternal::new();

    // A comment at the LOWER address (sorts before the higher run).
    db.add_comment(comment_type::USER1, &fad, &lower, b"low");
    // Three comments at the HIGHER address: uniq 0,1,2.
    assert!(db.add_comment_no_duplicate(comment_type::USER1, &fad, &higher, b"h0"));
    assert!(db.add_comment_no_duplicate(comment_type::USER1, &fad, &higher, b"h1"));
    assert!(db.add_comment_no_duplicate(comment_type::USER1, &fad, &higher, b"h2"));

    // A 4th distinct comment at the higher address: backwards walk hits the
    // higher run (uniq 2) and stops at the lower-address comment via `break`,
    // so uniq = 3 (NOT influenced by `low`).
    assert!(db.add_comment_no_duplicate(comment_type::USER1, &fad, &higher, b"h3"));

    let highs: Vec<u32> = db
        .comments_for(&fad)
        .filter(|c| c.get_addr().get_offset() == 0x1020)
        .map(|c| c.get_uniq() as u32)
        .collect();
    assert_eq!(highs, vec![0, 1, 2, 3]);

    // A duplicate-text insert at the higher address is rejected even though it
    // would have to walk past several non-dup higher-address comments first.
    assert!(!db.add_comment_no_duplicate(comment_type::USER1, &fad, &higher, b"h1"));
    // No new entry was added.
    assert_eq!(
        db.comments_for(&fad)
            .filter(|c| c.get_addr().get_offset() == 0x1020)
            .count(),
        4
    );
}

// ---------------------------------------------------------------------------
// 3. clearType half-open range bracketing across two functions AND two spaces.
//
// The C++ probes [lower_bound(fad,m_minimal,0), lower_bound(fad,m_maximal,
// 65535)).  A comment for a DIFFERENT function (even one ordered immediately
// after the cleared range) must survive, as must comments of the same function
// whose type does not intersect the mask.  Use a funcaddr in `io` (higher space
// index) to make sure the m_maximal sentinel brackets across the space boundary
// correctly.
// ---------------------------------------------------------------------------
#[test]
fn clear_type_brackets_one_function_across_space_boundary() {
    let m = manager();
    let fad = ram(&m, 0xF000); // high offset in ram
    let other = io(&m, 0x10); // different function, different (higher) space
    let mut db = CommentDatabaseInternal::new();

    db.add_comment(comment_type::USER1, &fad, &ram(&m, 0xF010), b"u1");
    db.add_comment(comment_type::HEADER, &fad, &ram(&m, 0xF020), b"hdr"); // non-matching type
    db.add_comment(comment_type::USER1, &fad, &io(&m, 0x0), b"u1-io-addr"); // same fn, addr in io
    db.add_comment(comment_type::USER1, &other, &io(&m, 0x20), b"other");

    // Clear USER1 for `fad`.  Both USER1 comments of `fad` go (including the one
    // whose *instruction* address lives in the io space), the HEADER one stays,
    // and `other` (a different function) is untouched.
    db.clear_type(&fad, comment_type::USER1);

    let remaining: Vec<_> = db
        .comments_for(&fad)
        .map(|c| c.get_text().to_vec())
        .collect();
    assert_eq!(remaining, vec![b"hdr".to_vec()], "only non-matching type survives");

    let others: Vec<_> = db
        .comments_for(&other)
        .map(|c| c.get_text().to_vec())
        .collect();
    assert_eq!(others, vec![b"other".to_vec()], "other function untouched");
}

// ---------------------------------------------------------------------------
// 4. CommentOrder / func_key_range cross-space ordering: funcaddrs in different
// spaces sort by space index, and each function's range is isolated.
// ---------------------------------------------------------------------------
#[test]
fn comments_for_isolates_functions_in_different_spaces() {
    let m = manager();
    let f_ram = ram(&m, 0x1000);
    let f_io = io(&m, 0x1000); // same offset, different space
    let mut db = CommentDatabaseInternal::new();

    db.add_comment(comment_type::USER1, &f_ram, &ram(&m, 0x1010), b"ram-fn");
    db.add_comment(comment_type::USER1, &f_io, &io(&m, 0x1010), b"io-fn");

    // Same offset but different funcaddr space => disjoint ranges.
    let ramc: Vec<_> = db.comments_for(&f_ram).map(|c| c.get_text().to_vec()).collect();
    let ioc: Vec<_> = db.comments_for(&f_io).map(|c| c.get_text().to_vec()).collect();
    assert_eq!(ramc, vec![b"ram-fn".to_vec()]);
    assert_eq!(ioc, vec![b"io-fn".to_vec()]);
}

// ---------------------------------------------------------------------------
// 5. Override::decode flow branch rejects an *invalid* address (the C++
// `addr.isInvalid()` arm — distinct from the NONE-type arm the porter tested).
// An <addr> with no space attribute decodes to an invalid Address, so a flow
// override with a valid type but invalid address must error.
// ---------------------------------------------------------------------------
#[test]
fn override_decode_flow_rejects_invalid_address() {
    let m = manager();
    let reg = registry();
    // Valid type "call" but the addr has no space => invalid address.
    let xml = br#"<override><flow type="call"><addr offset="0x40"/></flow></override>"#;
    let mut dec = XmlDecode::new(&m, &reg);
    dec.ingest_stream(xml).unwrap();
    let mut ov = Override::new();
    let err = ov
        .decode(&mut dec, &mut |_d| panic!("no protooverride"))
        .unwrap_err();
    assert!(
        err.to_string().contains("Bad flowoverride tag"),
        "invalid-address flow must throw the same LowlevelError, got: {}",
        err
    );
}

// ---------------------------------------------------------------------------
// 6. Override flow / deadcode round-trip through a HIGH space index, exercising
// the insertDeadcodeDelay vector-growth (`while size <= index push_back(-1)`)
// on decode, and getFlowOverride returning NONE for the empty store before any
// decode.
// ---------------------------------------------------------------------------
#[test]
fn override_decode_deadcode_high_index_grows_vector_and_flow_none_default() {
    let m = manager();

    // Empty store: getFlowOverride is NONE, hasFlowOverride is false.
    let empty = Override::new();
    assert_eq!(empty.get_flow_override(&io(&m, 0x40)), flow_type::NONE);
    assert!(!empty.has_flow_override());

    let reg = registry();
    // deadcodedelay for the "io" space (index 2): decode must grow the vector to
    // length 3 with -1 placeholders at indices 0 and 1, then deadcode_delays()
    // yields exactly (2, 5).
    let xml = br#"<override><deadcodedelay space="io" delay="5"/></override>"#;
    let mut dec = XmlDecode::new(&m, &reg);
    dec.ingest_stream(xml).unwrap();
    let mut ov = Override::new();
    ov.decode(&mut dec, &mut |_d| panic!("no protooverride")).unwrap();

    let delays: Vec<_> = ov.deadcode_delays().collect();
    assert_eq!(delays, vec![(2, 5)], "only the index-2 slot is non-placeholder");
}
