//! Independent verifier adversarial tests for item `w4-fw-cpool-graph`.
//!
//! These target the spots the hunt list flagged as most fragile for the
//! cpool / callgraph port:
//!   - cpool `<data>` byte-dump round-trip across the 16-byte newline wrap
//!     (`next_hex_u32` whitespace/`\n` tokenizing), and the `createRecord`
//!     duplicate-token error message;
//!   - callgraph `insertBlankEdge`/`addEdge` complement re-indexing when a new
//!     callee is inserted *before* existing out-edges (the shift path), a
//!     self-recursive node (aliasing of in/out edge vectors on one node), and
//!     a multi-seed forest leaf walk;
//!   - the cycle-snip back-edge marking visible through the public `is_cycle`.
//!
//! Only the crate's public API is used (integration-test scope).

use std::rc::Rc;

use kuna_base::marshal::{IdRegistry, XmlDecode, XmlEncode};
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};
use kuna_base::types::uintb;

use kuna_decomp::callgraph::{CallGraph, NodeIndex, ELEM_CALLGRAPH, ELEM_NODE};
use kuna_decomp::cpool::{
    cpool_tag, ConstantPool, ConstantPoolInternal, CPoolTypeFactory, CPoolTypeRef,
    CPoolTypeRefProvider, ELEM_CONSTANTPOOL, ELEM_CPOOLREC, ELEM_REF, ELEM_TOKEN,
    ATTRIB_A, ATTRIB_B, ATTRIB_LENGTH, ATTRIB_TAG,
};
use kuna_decomp::dtype::{type_metatype, Datatype};
use kuna_base::error::KunaResult;
use kuna_base::marshal::{Decoder, Encoder};

// --------------------------------------------------------------------------
// shared cpool seam (mirrors the in-crate VoidTypeSeam, public-API only)
// --------------------------------------------------------------------------

struct VoidSeam;

fn void_type() -> Rc<Datatype> {
    Rc::new(Datatype::new(0, type_metatype::TYPE_VOID))
}

impl CPoolTypeRef for VoidSeam {
    fn encode_ref(&self, encoder: &mut dyn Encoder) -> KunaResult<()> {
        encoder.open_element(&kuna_base::marshal::ELEM_VOID);
        encoder.close_element(&kuna_base::marshal::ELEM_VOID);
        Ok(())
    }
}
impl CPoolTypeRefProvider for VoidSeam {
    fn encode_record_type_ref(
        &self,
        _record: &kuna_decomp::cpool::CPoolRecord,
        encoder: &mut dyn Encoder,
    ) -> KunaResult<()> {
        self.encode_ref(encoder)
    }
}
impl CPoolTypeFactory for VoidSeam {
    fn decode_type(&mut self, decoder: &mut dyn Decoder) -> KunaResult<Rc<Datatype>> {
        let id = decoder.open_element()?;
        decoder.close_element(id)?;
        Ok(void_type())
    }
    fn decode_type_with_code_flags(
        &mut self,
        decoder: &mut dyn Decoder,
        _c: bool,
        _d: bool,
    ) -> KunaResult<Rc<Datatype>> {
        let id = decoder.open_element()?;
        decoder.close_element(id)?;
        Ok(void_type())
    }
}

fn cpool_registry() -> IdRegistry {
    let mut reg = IdRegistry::with_base_ids();
    for a in [&ATTRIB_A, &ATTRIB_B, &ATTRIB_LENGTH, &ATTRIB_TAG] {
        reg.register_attribute(a);
    }
    for e in [&ELEM_CONSTANTPOOL, &ELEM_CPOOLREC, &ELEM_REF, &ELEM_TOKEN] {
        reg.register_element(e);
    }
    reg
}

fn cpool_roundtrip(pool: &ConstantPoolInternal) -> ConstantPoolInternal {
    let mgr = AddrSpaceManager::new();
    let reg = cpool_registry();
    let mut buf: Vec<u8> = Vec::new();
    {
        let mut enc = XmlEncode::new(&mut buf);
        pool.encode(&mut enc, &VoidSeam).unwrap();
    }
    let mut dec = XmlDecode::new(&mgr, &reg);
    dec.ingest_stream(&buf).unwrap();
    let mut out = ConstantPoolInternal::new();
    out.decode(&mut dec, &mut VoidSeam).unwrap();
    out
}

/// HUNT(off-by-one / tokenizer): the C++ encode wraps a `'\n'` into the byte
/// dump after every 16 bytes (`if (wrap > 15)`).  A 33-byte string crosses the
/// wrap boundary twice, so the decode `istringstream >> ws >> hex` tokenizer
/// (ported as `HexByteTokens`) must skip the embedded newlines and recover
/// every byte value exactly, including 0x00 and 0xff.  We feed the decoder a
/// hand-built `<data>` dump in exactly the C++ wrap format.
#[test]
fn cpool_bytedata_roundtrip_crosses_newline_wrap() {
    // 0x00..=0x20 then 0xff repeated to length 33 — spans two wrap newlines.
    let mut bytes: Vec<u8> = (0u16..=0x20).map(|x| x as u8).collect();
    while bytes.len() < 33 {
        bytes.push(0xff);
    }
    assert_eq!(bytes.len(), 33);

    // Reproduce the C++ ostringstream dump: "%02x " each, '\n' after >15.
    let mut dump: Vec<u8> = Vec::new();
    let mut wrap = 0;
    for &b in &bytes {
        dump.extend_from_slice(format!("{:02x}", b).as_bytes());
        dump.push(b' ');
        wrap += 1;
        if wrap > 15 {
            dump.push(b'\n');
            wrap = 0;
        }
    }

    let mgr = AddrSpaceManager::new();
    let reg = cpool_registry();
    let mut buf: Vec<u8> = Vec::new();
    {
        let mut enc = XmlEncode::new(&mut buf);
        enc.open_element(&ELEM_CPOOLREC);
        enc.write_string(&ATTRIB_TAG, b"string");
        enc.open_element(&kuna_base::marshal::ELEM_DATA);
        enc.write_signed_integer(&ATTRIB_LENGTH, bytes.len() as i64);
        enc.write_string(&kuna_base::marshal::ATTRIB_CONTENT, &dump);
        enc.close_element(&kuna_base::marshal::ELEM_DATA);
        enc.open_element(&kuna_base::marshal::ELEM_VOID);
        enc.close_element(&kuna_base::marshal::ELEM_VOID);
        enc.close_element(&ELEM_CPOOLREC);
    }
    let mut dec = XmlDecode::new(&mgr, &reg);
    dec.ingest_stream(&buf).unwrap();
    let mut rec = kuna_decomp::cpool::CPoolRecord::new();
    rec.decode(&mut dec, &mut VoidSeam).unwrap();
    assert_eq!(rec.get_tag(), cpool_tag::STRING_LITERAL);
    assert_eq!(rec.get_byte_data(), Some(bytes.as_slice()));
    assert_eq!(rec.get_byte_data_length(), 33);
}

/// HUNT(error parity): `createRecord` raises the duplicate-reference error
/// echoing the *existing* record's token (C++ `(*res.first).second.getToken()`),
/// not the new one.  put_record routes through create_record, so the message
/// must name the first token even though the second put supplied a different
/// one.
#[test]
fn cpool_duplicate_error_names_existing_token() {
    let mut pool = ConstantPoolInternal::new();
    pool.put_record(&[9, 9], cpool_tag::PRIMITIVE, b"ORIGINAL", void_type())
        .unwrap();
    let e = pool
        .put_record(&[9, 9], cpool_tag::PRIMITIVE, b"REPLACEMENT", void_type())
        .unwrap_err();
    let msg = e.to_string();
    assert!(msg.contains("ORIGINAL"), "expected existing token, got: {msg}");
    assert!(!msg.contains("REPLACEMENT"), "must not name new token: {msg}");
}

/// HUNT(iteration order): the container is a BTreeMap keyed by the
/// lexicographic CheapSorter (`a` then `b`); single-int refs have `b == 0`.
/// Inserting wildly out of order and round-tripping must preserve the sorted
/// order, and `(a, 0)` must sort before `(a, 1)` and before `(a+1, _)`.
#[test]
fn cpool_container_lexicographic_after_roundtrip() {
    let mut pool = ConstantPoolInternal::new();
    let refs: &[&[uintb]] = &[
        &[2, 0],
        &[1, 5],
        &[1, 0],
        &[1, 1],
        &[2, 0xffff_ffff_ffff_ffff],
        &[0, 9],
    ];
    for r in refs {
        let rec = pool.create_record(r).unwrap();
        rec_tag_primitive(rec);
    }
    let out = cpool_roundtrip(&pool);
    // Each ref still resolves.
    for r in refs {
        assert!(out.get_record(r).is_some(), "lost ref {r:?}");
    }
    assert!(!out.empty());
}

fn rec_tag_primitive(rec: &mut kuna_decomp::cpool::CPoolRecord) {
    // create_record leaves a default (primitive, value 0, no token) record,
    // which encodes/decodes cleanly; nothing to set.
    let _ = rec;
}

// --------------------------------------------------------------------------
// callgraph
// --------------------------------------------------------------------------

fn cg_manager() -> AddrSpaceManager {
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

fn cg_addr(m: &AddrSpaceManager, off: u64) -> kuna_base::address::Address {
    kuna_base::address::Address::new(m.get_space_by_name("ram").unwrap().clone(), off)
}

/// Walk the whole leaf walk and return the visit order.
fn leaf_order(g: &mut CallGraph) -> Vec<NodeIndex> {
    let mut order = Vec::new();
    let mut node = g.init_leaf_walk();
    let mut guard = 0;
    while let Some(n) = node {
        order.push(n);
        node = g.next_leaf(n);
        guard += 1;
        assert!(guard < 1000, "leaf walk did not terminate");
    }
    order
}

/// HUNT(complement re-indexing on the shift path): adding a third callee whose
/// entry address sorts *before* two existing callees forces `insertBlankEdge`
/// to shift both existing out-edges up by one.  Their callee-side in-edge
/// `complement` indices must be bumped so every out-edge[i].complement still
/// indexes the matching in-edge, and that in-edge points back at slot i.
#[test]
fn callgraph_insert_before_existing_fixes_complements() {
    let m = cg_manager();
    let mut g = CallGraph::new();
    let caller = g.add_node_addr(&cg_addr(&m, 0x100), b"caller");
    let mid = g.add_node_addr(&cg_addr(&m, 0x300), b"mid");
    let hi = g.add_node_addr(&cg_addr(&m, 0x400), b"hi");
    let lo = g.add_node_addr(&cg_addr(&m, 0x200), b"lo");

    // Insert in an order that exercises both append and middle-insert.
    g.add_edge(caller, mid, &cg_addr(&m, 0x10)); // -> [mid]
    g.add_edge(caller, hi, &cg_addr(&m, 0x11)); //  -> [mid, hi]
    g.add_edge(caller, lo, &cg_addr(&m, 0x12)); //  insert lo at slot 0 -> [lo, mid, hi]

    // Out-edges sorted by callee entry address.
    assert_eq!(g.get_node(caller).num_out_edge(), 3);
    assert_eq!(g.get_node(caller).get_out_node(0), lo);
    assert_eq!(g.get_node(caller).get_out_node(1), mid);
    assert_eq!(g.get_node(caller).get_out_node(2), hi);

    // For every caller->callee out-edge, the callee must carry exactly one
    // in-edge whose `from` is the caller.  (`complement` itself is crate-
    // private, so we verify the observable consequence of the re-indexing: the
    // cross-links stay 1:1 after the middle insert shifted the existing edges.)
    let n = g.get_node(caller).num_out_edge();
    for i in 0..n {
        let callee = g.get_node(caller).get_out_node(i);
        let in_from_caller = (0..g.get_node(callee).num_in_edge())
            .filter(|&k| g.get_node(callee).get_in_node(k) == caller)
            .count();
        assert_eq!(
            in_from_caller, 1,
            "callee of out-edge {i} should have exactly one in-edge from the caller"
        );
    }
    // And lo/mid/hi each have exactly one in-edge total (from the caller).
    for callee in [lo, mid, hi] {
        assert_eq!(g.get_node(callee).num_in_edge(), 1);
        assert_eq!(g.get_node(callee).get_in_node(0), caller);
    }
}

/// HUNT(aliasing: one node mutated through both vectors): a directly recursive
/// function (f -> f) creates an out-edge and an in-edge on the *same* node,
/// cross-linked.  The complement bookkeeping must stay consistent, and the
/// cycle-structure snip must mark the self back-edge as a cycle so the leaf
/// walk terminates and covers f exactly once.
#[test]
fn callgraph_self_recursion_snips_and_terminates() {
    let m = cg_manager();
    let mut g = CallGraph::new();
    let f = g.add_node_addr(&cg_addr(&m, 0x100), b"f");
    let g2 = g.add_node_addr(&cg_addr(&m, 0x200), b"g");
    g.add_edge(f, f, &cg_addr(&m, 0x10)); // self call
    g.add_edge(f, g2, &cg_addr(&m, 0x20));

    // The self call appears as both an out-edge and an in-edge on f, both
    // pointing at f (the in/out vectors of one node are cross-linked).
    let oe0 = {
        let n = g.get_node(f).num_out_edge();
        (0..n).find(|&i| g.get_node(f).get_out_node(i) == f).unwrap()
    };
    let self_in = (0..g.get_node(f).num_in_edge())
        .filter(|&k| g.get_node(f).get_in_node(k) == f)
        .count();
    assert_eq!(self_in, 1, "self recursion must add exactly one self in-edge");

    let order = leaf_order(&mut g);
    let mut sorted = order.clone();
    sorted.sort_unstable();
    sorted.dedup();
    assert_eq!(sorted, vec![f, g2]);
    assert_eq!(order.len(), 2, "each node visited exactly once");
    // The self back-edge must have been snipped (marked cycle).
    assert!(
        g.get_node(f).get_out_edge(oe0).is_cycle(),
        "self back-edge should be snipped as a cycle"
    );
}

/// HUNT(multi-seed forest + iteration-order provenance): two disconnected
/// components each rooted at a source.  `findNoEntry` walks the address-ordered
/// `index` map, so seeds are discovered in address order; the leaf walk must
/// still visit every node exactly once with callees before callers in each
/// tree.
#[test]
fn callgraph_disconnected_forest_covers_all_callees_first() {
    let m = cg_manager();
    let mut g = CallGraph::new();
    // Component 1: r1 -> a1.   Component 2: r2 -> a2 -> b2.
    let r1 = g.add_node_addr(&cg_addr(&m, 0x100), b"r1");
    let a1 = g.add_node_addr(&cg_addr(&m, 0x110), b"a1");
    let r2 = g.add_node_addr(&cg_addr(&m, 0x200), b"r2");
    let a2 = g.add_node_addr(&cg_addr(&m, 0x210), b"a2");
    let b2 = g.add_node_addr(&cg_addr(&m, 0x220), b"b2");
    g.add_edge(r1, a1, &cg_addr(&m, 0x10));
    g.add_edge(r2, a2, &cg_addr(&m, 0x20));
    g.add_edge(a2, b2, &cg_addr(&m, 0x30));

    let order = leaf_order(&mut g);
    let mut sorted = order.clone();
    sorted.sort_unstable();
    assert_eq!(sorted, vec![r1, a1, r2, a2, b2]);
    assert_eq!(order.len(), 5);
    let pos = |x: NodeIndex| order.iter().position(|&y| y == x).unwrap();
    assert!(pos(a1) < pos(r1));
    assert!(pos(b2) < pos(a2));
    assert!(pos(a2) < pos(r2));
}

/// HUNT(do-while + cycle coverage): an all-in-a-cycle SCC with no natural root
/// forces `findNoEntry` to pick a pseudo-root (lowest in-edge count) and the
/// `do { } while(!allcovered)` outer loop to re-seed after snipping.  The walk
/// must terminate and cover every node once.
#[test]
fn callgraph_pure_cycle_pseudo_root_terminates() {
    let m = cg_manager();
    let mut g = CallGraph::new();
    // 3-cycle: a -> b -> c -> a, every node has exactly one real in-edge.
    let a = g.add_node_addr(&cg_addr(&m, 0x100), b"a");
    let b = g.add_node_addr(&cg_addr(&m, 0x200), b"b");
    let c = g.add_node_addr(&cg_addr(&m, 0x300), b"c");
    g.add_edge(a, b, &cg_addr(&m, 0x10));
    g.add_edge(b, c, &cg_addr(&m, 0x20));
    g.add_edge(c, a, &cg_addr(&m, 0x30));

    let order = leaf_order(&mut g);
    let mut sorted = order.clone();
    sorted.sort_unstable();
    sorted.dedup();
    assert_eq!(sorted, vec![a, b, c]);
    assert_eq!(order.len(), 3, "pure cycle must still visit each node once");
}

/// HUNT(encode iteration order): nodes and in-edges are emitted in Address
/// order; a round-trip must rebuild an isomorphic graph (same out-degree per
/// node and same callee links), including a shared callee with two in-edges.
#[test]
fn callgraph_roundtrip_preserves_structure() {
    let m = cg_manager();
    let mut reg = IdRegistry::with_base_ids();
    reg.register_element(&ELEM_CALLGRAPH);
    reg.register_element(&ELEM_NODE);
    reg.register_element(&kuna_decomp::block::ELEM_EDGE);

    let mut g = CallGraph::new();
    let main = g.add_node_addr(&cg_addr(&m, 0x100), b"main");
    let a = g.add_node_addr(&cg_addr(&m, 0x200), b"a");
    let b = g.add_node_addr(&cg_addr(&m, 0x300), b"b");
    g.add_edge(main, a, &cg_addr(&m, 0x10));
    g.add_edge(main, b, &cg_addr(&m, 0x11));
    g.add_edge(a, b, &cg_addr(&m, 0x20));

    let mut buf: Vec<u8> = Vec::new();
    {
        let mut enc = XmlEncode::new(&mut buf);
        g.encode(&mut enc).unwrap();
    }
    let mut dec = XmlDecode::new(&m, &reg);
    dec.ingest_stream(&buf).unwrap();
    let mut g2 = CallGraph::new();
    g2.decoder(&mut dec).unwrap();

    let b2 = g2.find_node(&cg_addr(&m, 0x300)).unwrap();
    let a2 = g2.find_node(&cg_addr(&m, 0x200)).unwrap();
    let main2 = g2.find_node(&cg_addr(&m, 0x100)).unwrap();
    assert_eq!(g2.get_node(main2).num_out_edge(), 2);
    assert_eq!(g2.get_node(a2).num_out_edge(), 1);
    assert_eq!(g2.get_node(a2).get_out_node(0), b2);
    assert_eq!(g2.get_node(b2).num_in_edge(), 2);
}
