//! Verifier adversarial tests for item `w1-harness-unittests` (testmarshal half).
//!
//! These extend the ported C++ unit tests (`tests/testmarshal.rs`,
//! from `decompiler/unittests/testmarshal.cc`) into the boundary cells the
//! C++ suite stops short of.  Every expected behavior below is derived from
//! the vendored C++ sources, not from the Rust implementation:
//! - `PackedDecode::ingestStream`/`getByte`/`getBytePlus1` buffer logic
//!   (marshal.cc:707-733, BUFFER_SIZE pad semantics) — the C++
//!   `marshal_bufferpad` test only pins the exact-fill case (1024 bytes);
//!   here the off-by-one neighbours and the exact-two-buffer case, where a
//!   2-byte bool attribute straddles the 1024-byte chunk boundary.
//! - `PackedEncode::writeSignedInteger` (marshal.cc:2124-area in Rust,
//!   C++ marshal.cc `num = -val`) and `XmlDecode` strtoll parity for
//!   i64::MIN: the C++ negation wraps on x86-64 and round-trips.
//! - `closeElementSkipping` from the document root: C++
//!   PackedDecode::closeElementSkipping (marshal.cc:785, do-while over an
//!   id stack) and XmlDecode::closeElementSkipping (marshal.cc:212, plain
//!   pop) must both leave the stream at end-of-document, where
//!   `peekElement()` and no-arg `openElement()` both return 0
//!   (marshal.cc:126-133 XML / :719-731 packed).

use kuna_base::marshal::{
    Decoder, Encoder, IdRegistry, PackedDecode, PackedEncode, XmlDecode, XmlEncode, ATTRIB_ALIGN,
    ATTRIB_BIGENDIAN, ATTRIB_CONSTRUCTOR, ATTRIB_CONTENT, ATTRIB_DESTRUCTOR, ATTRIB_EXTRAPOP,
    ATTRIB_FORMAT, ATTRIB_ID, ELEM_DATA, ELEM_INPUT, ELEM_OFF, ELEM_OUTPUT, ELEM_SYMBOL,
    ELEM_TARGET,
};
use kuna_base::space::{addrspace_flags, spacetype, AddrSpace, AddrSpaceManager};
use std::rc::Rc;

/// Same fixture as the ported harness (`testmarshal.cc:50-66`).
fn the_environ_build() -> AddrSpaceManager {
    let mut manager = AddrSpaceManager::new();
    let ram = AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "ram",
        false,
        8,
        1,
        3,
        addrspace_flags::hasphysical,
        1,
        1,
    );
    manager.insert_space(Rc::new(ram)).unwrap();
    manager
}

/// Extension of C++ `marshal_bufferpad` (testmarshal.cc:533-555): the C++
/// test pins only N=511 bools = exactly one 1024-byte buffer.  Each bool is
/// 2 bytes and open/close are 1 byte each (per the C++ test's own byte
/// accounting), so N=510/512 are the one-under/one-over cells and N=1023 is
/// exactly two buffers (2048 bytes).  For N>=512 the bool attribute pair at
/// k=511 occupies stream offsets [1023,1024] — its header byte is the last
/// byte of ingest buffer 1 and its data byte the first byte of buffer 2,
/// exercising the cross-buffer getNextByte path the C++ exact-fill test
/// never reaches.
#[test]
fn verify_bufferpad_off_by_one_and_two_buffer_cells() {
    let spc_manager = the_environ_build();
    for &n in &[510i32, 511, 512, 1023] {
        let mut s: Vec<u8> = Vec::new();
        {
            let mut encoder = PackedEncode::new(&mut s);
            encoder.open_element(&ELEM_INPUT);
            for i in 0..n {
                encoder.write_bool(&ATTRIB_CONTENT, (i & 1) == 0);
            }
            encoder.close_element(&ELEM_INPUT);
        }
        // C++ byte accounting: 1 + 2N + 1
        assert_eq!(s.len() as i32, 2 * n + 2, "encoded length for N={n}");
        let mut decoder = PackedDecode::new(&spc_manager);
        decoder.ingest_stream(&s).unwrap();
        let el = decoder.open_element_id(&ELEM_INPUT).unwrap();
        for i in 0..n {
            let attrib_id = decoder.get_next_attribute_id().unwrap();
            assert_eq!(attrib_id, ATTRIB_CONTENT.get_id(), "attrib id N={n} i={i}");
            let val = decoder.read_bool().unwrap();
            assert_eq!(val, (i & 1) == 0, "bool value N={n} i={i}");
        }
        // C++ peekElement at the ELEMENT_END byte returns 0 (marshal.cc:719)
        assert_eq!(decoder.peek_element().unwrap(), 0, "peek before close N={n}");
        decoder.close_element(el).unwrap();
    }
}

/// Signed-integer extremes through both codecs.  The C++ suite tops out at
/// 0x7fffffffffffffff (63 bits, testmarshal.cc:80); the C++ encoder/decoder
/// negation pair (`num = -val` on write, `res = -res` on read) wraps on
/// x86-64 so INT64_MIN round-trips.  The Rust port must reproduce that in
/// BOTH build profiles (no debug-only negation panic, per ADR 0003).
#[test]
fn verify_signed_extremes_roundtrip_both_codecs() {
    let cells: [(i64, &kuna_base::marshal::AttributeId); 6] = [
        (i64::MIN, &ATTRIB_ALIGN),
        (i64::MIN + 1, &ATTRIB_BIGENDIAN),
        (-1, &ATTRIB_CONSTRUCTOR),
        (0, &ATTRIB_DESTRUCTOR),
        (1, &ATTRIB_EXTRAPOP),
        (i64::MAX, &ATTRIB_FORMAT),
    ];
    let spc_manager = the_environ_build();

    // Packed
    let mut packed: Vec<u8> = Vec::new();
    {
        let mut encoder = PackedEncode::new(&mut packed);
        encoder.open_element(&ELEM_DATA);
        for (val, attrib) in cells.iter() {
            encoder.write_signed_integer(attrib, *val);
        }
        encoder.close_element(&ELEM_DATA);
    }
    let mut decoder = PackedDecode::new(&spc_manager);
    decoder.ingest_stream(&packed).unwrap();
    let el = decoder.open_element_id(&ELEM_DATA).unwrap();
    for (val, attrib) in cells.iter() {
        let got = decoder.read_signed_integer_id(attrib).unwrap();
        assert_eq!(got, *val, "packed round-trip of {val:#x}");
    }
    decoder.close_element(el).unwrap();

    // XML
    let registry = IdRegistry::with_base_ids();
    let mut xml: Vec<u8> = Vec::new();
    {
        let mut encoder = XmlEncode::new(&mut xml);
        encoder.open_element(&ELEM_DATA);
        for (val, attrib) in cells.iter() {
            encoder.write_signed_integer(attrib, *val);
        }
        encoder.close_element(&ELEM_DATA);
    }
    let mut decoder = XmlDecode::new(&spc_manager, &registry);
    decoder.ingest_stream(&xml).unwrap();
    let el = decoder.open_element_id(&ELEM_DATA).unwrap();
    for (val, attrib) in cells.iter() {
        let got = decoder.read_signed_integer_id(attrib).unwrap();
        assert_eq!(got, *val, "xml round-trip of {val:#x}");
    }
    decoder.close_element(el).unwrap();
}

/// Encode the full C++ `test_hierarchy` document (testmarshal.cc:315-350).
fn hierarchy_encode<E: Encoder>(encoder: &mut E) {
    encoder.open_element(&ELEM_DATA);
    encoder.write_bool(&ATTRIB_CONTENT, true);
    encoder.open_element(&ELEM_INPUT);
    encoder.open_element(&ELEM_OUTPUT);
    encoder.write_signed_integer(&ATTRIB_ID, 0x1000);
    encoder.open_element(&ELEM_DATA);
    encoder.open_element(&ELEM_DATA);
    encoder.open_element(&ELEM_OFF);
    encoder.close_element(&ELEM_OFF);
    encoder.open_element(&ELEM_OFF);
    encoder.write_string(&ATTRIB_ID, b"blahblah");
    encoder.close_element(&ELEM_OFF);
    encoder.open_element(&ELEM_OFF);
    encoder.close_element(&ELEM_OFF);
    encoder.close_element(&ELEM_DATA);
    encoder.close_element(&ELEM_DATA);
    encoder.open_element(&ELEM_SYMBOL);
    encoder.write_unsigned_integer(&ATTRIB_ID, 17);
    encoder.open_element(&ELEM_TARGET);
    encoder.close_element(&ELEM_TARGET);
    encoder.close_element(&ELEM_SYMBOL);
    encoder.close_element(&ELEM_OUTPUT);
    encoder.close_element(&ELEM_INPUT);
    for _ in 0..6 {
        encoder.open_element(&ELEM_INPUT);
        encoder.close_element(&ELEM_INPUT);
    }
    encoder.close_element(&ELEM_DATA);
}

/// `closeElementSkipping` invoked at the document root, over the deepest
/// document in the C++ suite (6 nesting levels, attributes of every kind in
/// the skipped subtrees).  The C++ tests only skip mid-tree (el3 with one
/// unread sibling subtree, el1 with four unread <input>s,
/// testmarshal.cc:372/378).  After the skip, end-of-document must look
/// identical through both codecs: `peekElement() == 0` (XmlDecode with the
/// root consumed and the stack empty, marshal.cc:129-133; PackedDecode at
/// the pad byte, marshal.cc:719-723) and no-arg `openElement() == 0`
/// (marshal.cc:147-150 / :733-738).
#[test]
fn verify_close_element_skipping_whole_document_parity() {
    let spc_manager = the_environ_build();

    // Packed
    let mut packed: Vec<u8> = Vec::new();
    {
        let mut encoder = PackedEncode::new(&mut packed);
        hierarchy_encode(&mut encoder);
    }
    let mut decoder = PackedDecode::new(&spc_manager);
    decoder.ingest_stream(&packed).unwrap();
    let el1 = decoder.open_element_id(&ELEM_DATA).unwrap();
    decoder.close_element_skipping(el1).unwrap();
    assert_eq!(decoder.peek_element().unwrap(), 0, "packed peek after skip");
    assert_eq!(decoder.open_element().unwrap(), 0, "packed open after skip");

    // XML
    let registry = IdRegistry::with_base_ids();
    let mut xml: Vec<u8> = Vec::new();
    {
        let mut encoder = XmlEncode::new(&mut xml);
        hierarchy_encode(&mut encoder);
    }
    let mut decoder = XmlDecode::new(&spc_manager, &registry);
    decoder.ingest_stream(&xml).unwrap();
    let el1 = decoder.open_element_id(&ELEM_DATA).unwrap();
    decoder.close_element_skipping(el1).unwrap();
    assert_eq!(decoder.peek_element().unwrap(), 0, "xml peek after skip");
    assert_eq!(decoder.open_element().unwrap(), 0, "xml open after skip");
}
