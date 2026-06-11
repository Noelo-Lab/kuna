//! Verifier adversarial tests for item `w2-sleigh-context` (round 1).
//!
//! Written by the independent verifier against the C++ oracle semantics of
//! `decompiler/cpp/globalcontext.{hh,cc}` (and the `partmap.hh` machinery it
//! rides on).  Each test targets a hunt-list spot the port could plausibly
//! have gotten wrong while still passing the porter's own tests.

use std::rc::Rc;

use kuna_base::marshal::{Decoder, IdRegistry, XmlDecode, XmlEncode};
use kuna_base::space::{addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace};
use kuna_base::address::Address;
use kuna_num::pcoderaw::VarnodeData;
use kuna_sleigh::globalcontext::{
    register_globalcontext_ids, ContextCache, ContextDatabase, ContextInternal, TrackedContext,
};

fn test_manager() -> AddrSpaceManager {
    let mut manager = AddrSpaceManager::new();
    manager.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    for (i, (name, bigend)) in [("ram", false), ("reg", false), ("breg", true)].iter().enumerate() {
        let spc = AddrSpace::new(
            spacetype::IPTR_PROCESSOR,
            name,
            *bigend,
            4,
            1,
            (i + 1) as i32, // cast: small test index
            addrspace_flags::hasphysical,
            1,
            1,
        );
        manager.insert_space(Rc::new(spc)).unwrap();
    }
    manager
}

fn space(manager: &AddrSpaceManager, nm: &str) -> Rc<AddrSpace> {
    Rc::clone(manager.get_space_by_name(nm).unwrap())
}

fn addr(manager: &AddrSpaceManager, off: u64) -> Address {
    Address::new(space(manager, "ram"), off)
}

fn test_registry() -> IdRegistry {
    let mut registry = IdRegistry::with_base_ids();
    register_globalcontext_ids(&mut registry);
    registry
}

fn get(db: &ContextInternal, manager: &AddrSpaceManager, nm: &[u8], off: u64) -> u32 {
    db.get_variable_value(nm, &addr(manager, off)).unwrap()
}

/// C++ `decodeContext` (globalcontext.cc:354-358) re-zeroes the ENTIRE
/// default blob for EVERY `<set>` child before painting that child's value:
/// with two children, the second child's zeroing wipes the first child's
/// value.  A port that hoisted the zeroing out of the loop would keep both.
#[test]
fn verify_w2ctx_default_pointset_rezeroed_per_set_child() {
    let manager = test_manager();
    let registry = test_registry();
    let mut db = ContextInternal::new();
    db.register_variable(b"fldA", 0, 7).unwrap();
    db.register_variable(b"fldB", 8, 15).unwrap();
    db.set_variable_default(b"fldA", 0x55).unwrap();

    let mut decoder = XmlDecode::new(&manager, &registry);
    decoder
        .ingest_stream(
            b"<context_points><context_pointset>\
              <set name=\"fldA\" val=\"2\"/>\
              <set name=\"fldB\" val=\"3\"/>\
            </context_pointset></context_points>",
        )
        .unwrap();
    db.decode(&mut decoder).unwrap();
    // fldA=2 was painted by the first child, then WIPED by the second
    // child's wholesale re-zeroing; only fldB=3 survives.
    assert_eq!(db.get_default_value_by_name(b"fldA").unwrap(), 0);
    assert_eq!(db.get_default_value_by_name(b"fldB").unwrap(), 3);
}

/// C++ `getRegionToChangePoint` (globalcontext.cc:446-468) marks the
/// explicitly-set mask ONLY on the first entry of a paint; intermediate
/// split points receive the painted value but no mask, so later paints of
/// the same variable flow THROUGH them and stop only at genuine
/// change points of that variable.
#[test]
fn verify_w2ctx_change_point_first_entry_only_mask() {
    let manager = test_manager();
    let mut db = ContextInternal::new();
    db.register_variable(b"fldA", 0, 7).unwrap();
    db.register_variable(b"fldB", 8, 15).unwrap();

    db.set_variable(b"fldA", &addr(&manager, 0x100), 1).unwrap(); // marks A @0x100
    db.set_variable(b"fldB", &addr(&manager, 0x80), 2).unwrap(); // marks B @0x80
    // paints entries 0x40 (marks A) and 0x80 (value only), stops at 0x100
    db.set_variable(b"fldA", &addr(&manager, 0x40), 3).unwrap();
    assert_eq!(get(&db, &manager, b"fldA", 0x3f), 0);
    assert_eq!(get(&db, &manager, b"fldA", 0x40), 3);
    assert_eq!(get(&db, &manager, b"fldA", 0x80), 3); // flowed through B's split
    assert_eq!(get(&db, &manager, b"fldA", 0xff), 3);
    assert_eq!(get(&db, &manager, b"fldA", 0x100), 1); // stopped at A's mask

    // a paint starting between existing splits: splits 0x60 (clone, mask
    // zeroed per FreeArray operator=), marks A there, flows through 0x80
    db.set_variable(b"fldA", &addr(&manager, 0x60), 4).unwrap();
    assert_eq!(get(&db, &manager, b"fldA", 0x5f), 3);
    assert_eq!(get(&db, &manager, b"fldA", 0x60), 4);
    assert_eq!(get(&db, &manager, b"fldA", 0x90), 4);
    assert_eq!(get(&db, &manager, b"fldA", 0x100), 1);

    // a paint below 0x60 must now STOP at 0x60 (A's mask was set there as
    // the first entry of its own paint)
    db.set_variable(b"fldA", &addr(&manager, 0x50), 5).unwrap();
    assert_eq!(get(&db, &manager, b"fldA", 0x4f), 3);
    assert_eq!(get(&db, &manager, b"fldA", 0x50), 5);
    assert_eq!(get(&db, &manager, b"fldA", 0x5f), 5);
    assert_eq!(get(&db, &manager, b"fldA", 0x60), 4);

    // ... and one below 0x40 stops at 0x40
    db.set_variable(b"fldA", &addr(&manager, 0x20), 6).unwrap();
    assert_eq!(get(&db, &manager, b"fldA", 0x1f), 0);
    assert_eq!(get(&db, &manager, b"fldA", 0x20), 6);
    assert_eq!(get(&db, &manager, b"fldA", 0x3f), 6);
    assert_eq!(get(&db, &manager, b"fldA", 0x40), 3);

    // B was never disturbed: 0 below 0x80 (clones of pre-B entries), 2 after
    assert_eq!(get(&db, &manager, b"fldB", 0x7f), 0);
    assert_eq!(get(&db, &manager, b"fldB", 0x80), 2);
    assert_eq!(get(&db, &manager, b"fldB", 0x150), 2);
}

/// C++ `getRegionForSet(addr,addr,...)` paints NOTHING (aiter == biter
/// immediately) and — crucially — does NOT mark the variable's mask at the
/// split point it forces, so a later change-point paint flows through it.
/// A port that marked the mask before testing the stop condition would
/// wrongly stop the later paint at 0x100.
#[test]
fn verify_w2ctx_equal_range_region_set_is_noop_and_unmasked() {
    let manager = test_manager();
    let mut db = ContextInternal::new();
    db.register_variable(b"fldA", 0, 7).unwrap();

    let a = addr(&manager, 0x100);
    db.set_variable_region(b"fldA", &a, &a, 9).unwrap();
    assert_eq!(get(&db, &manager, b"fldA", 0x100), 0); // nothing painted

    // the forced split at 0x100 carries NO fldA mask: paint from below
    // flows through it to the end of the space
    db.set_variable(b"fldA", &addr(&manager, 0x50), 7).unwrap();
    assert_eq!(get(&db, &manager, b"fldA", 0x4f), 0);
    assert_eq!(get(&db, &manager, b"fldA", 0x50), 7);
    assert_eq!(get(&db, &manager, b"fldA", 0x100), 7);
    assert_eq!(get(&db, &manager, b"fldA", 0xffff), 7);
}

/// `getTrackedValue` endpoint arithmetic is C++ unsigned wraparound
/// (globalcontext.cc:228-247): a query straddling the top of the offset
/// space wraps `endoff` to a small value, which makes the containment test
/// pass and the (defined, < 64) endian shift produce a real value.  A port
/// using checked/saturating arithmetic would panic or skip the entry.
#[test]
fn verify_w2ctx_tracked_value_wrapped_endoff_containment() {
    let manager = test_manager();
    let mut db = ContextInternal::new();
    db.register_variable(b"fldA", 0, 7).unwrap();
    let point = addr(&manager, 0x1000);
    {
        let tset = db.get_tracked_default();
        tset.push(TrackedContext {
            loc: VarnodeData {
                space: Some(space(&manager, "reg")),
                offset: u64::MAX - 7,
                size: 8,
            },
            val: 0x1122_3344_5566_7788,
        });
        tset.push(TrackedContext {
            loc: VarnodeData {
                space: Some(space(&manager, "breg")),
                offset: u64::MAX - 7,
                size: 8,
            },
            val: 0x1122_3344_5566_7788,
        });
    }
    // mem.offset + mem.size - 1 == u64::MAX + 2 - 1 wraps to 0:
    //   tendoff(u64::MAX) < endoff(0) is FALSE, so containment passes.
    // little endian: res >>= 8*(MAX - (MAX-7)) = 56 -> 0x11, & 0xffff
    let mem = VarnodeData { space: Some(space(&manager, "reg")), offset: u64::MAX, size: 2 };
    assert_eq!(db.get_tracked_value(&mem, &point), 0x11);
    // big endian: endoff(0) != tendoff(MAX) -> res >>= 8*(tendoff-mem.offset)
    //   = 0 -> untouched, & 0xffff
    let mem = VarnodeData { space: Some(space(&manager, "breg")), offset: u64::MAX, size: 2 };
    assert_eq!(db.get_tracked_value(&mem, &point), 0x7788);
    // non-wrapping sanity pins on the same registers
    let mem = VarnodeData { space: Some(space(&manager, "reg")), offset: u64::MAX, size: 1 };
    assert_eq!(db.get_tracked_value(&mem, &point), 0x11);
    let mem = VarnodeData { space: Some(space(&manager, "breg")), offset: u64::MAX, size: 1 };
    assert_eq!(db.get_tracked_value(&mem, &point), 0x88);
}

/// `encodeContext` iterates `map<string,ContextBitRange>` — std::string
/// ordering is unsigned-byte lexicographic (memcmp), so a name starting
/// with byte 0xC3 sorts AFTER all ASCII names.  A signed-char comparison
/// (or any other key order) flips the emitted `<set>` sequence.
#[test]
fn verify_w2ctx_encode_variable_order_unsigned_byte_lex() {
    let manager = test_manager();
    let mut db = ContextInternal::new();
    // registration order deliberately scrambled
    db.register_variable("\u{e9}".as_bytes(), 8, 15).unwrap(); // bytes C3 A9
    db.register_variable(b"z", 16, 23).unwrap();
    db.register_variable(b"A", 0, 7).unwrap();
    db.set_variable(b"A", &addr(&manager, 0x100), 1).unwrap();

    let mut stream: Vec<u8> = Vec::new();
    {
        let mut encoder = XmlEncode::new(&mut stream);
        db.encode(&mut encoder).unwrap();
    }
    let s = stream;
    let pos = |needle: &[u8]| -> usize {
        s.windows(needle.len())
            .position(|w| w == needle)
            .unwrap_or_else(|| panic!("missing {:?}", String::from_utf8_lossy(needle)))
    };
    let pa = pos(b"name=\"A\"");
    let pz = pos(b"name=\"z\"");
    let pe = pos("name=\"\u{e9}\"".as_bytes());
    assert!(pa < pz, "A(0x41) must precede z(0x7a): {pa} vs {pz}");
    assert!(pz < pe, "z(0x7a) must precede \\xc3\\xa9 (unsigned order): {pz} vs {pe}");
}

/// LOSS canary: the port's ContextCache re-fetches the blob on a cache hit
/// instead of holding the C++ raw pointer.  When the database is mutated
/// DIRECTLY (not through the cache) and a new split lands inside the live
/// cached range, C++ serves the stale pre-split blob (here: 7); the port
/// serves the fresh value (9).  Pins the documented divergence.
#[test]
fn verify_w2ctx_cache_direct_mutation_divergence_canary() {
    let manager = test_manager();
    let mut db = ContextInternal::new();
    db.register_variable(b"fldA", 0, 7).unwrap();
    let var = db.get_variable(b"fldA").unwrap();
    db.set_variable(b"fldA", &addr(&manager, 0x200), 7).unwrap();

    let mut cache = ContextCache::new();
    let mut buf = [0u32; 1];
    cache.get_context(&db, &addr(&manager, 0x300), &mut buf); // cache [0x200, highest]
    assert_eq!(var.get_value(&buf), 7);

    // direct database mutation: split at 0x280 inside the cached range
    db.set_variable(b"fldA", &addr(&manager, 0x280), 9).unwrap();

    // through-the-cache read at 0x300: C++ would serve the stale 0x200 blob
    // (value 7); the Rust port re-fetches and serves 9.
    cache.get_context(&db, &addr(&manager, 0x300), &mut buf);
    assert_eq!(
        var.get_value(&buf),
        9,
        "loss canary: port intentionally serves fresh value where C++ serves stale"
    );
    // below the new split the old paint still governs — both engines agree
    cache.get_context(&db, &addr(&manager, 0x250), &mut buf);
    assert_eq!(var.get_value(&buf), 7);
}
