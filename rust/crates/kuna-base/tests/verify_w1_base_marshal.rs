//! Verifier adversarial tests for item `w1-base-marshal` (round 1).
//!
//! Targets chosen from the hunt list against decompiler/cpp/marshal.cc:
//!
//! - `XmlDecode::getIndexedAttributeId` (marshal.cc:243-260): the C++ parses
//!   the name suffix with `istringstream >> dec >> val` (base 10 FIXED) into
//!   a `uint4`, then computes `attribId.getId() + (val-1)` in wrapping uint4
//!   arithmetic.  Expected values below were produced by compiling and
//!   running the exact C++ statement sequence (g++ -std=c++11, libstdc++):
//!
//!   ```text
//!   suffix "1"          -> val 1          -> base+0
//!   suffix "23"         -> val 23         -> base+22
//!   suffix "010"        -> val 10 (DEC)   -> base+9
//!   suffix "08"         -> val 8  (DEC)   -> base+7
//!   suffix "0x2"        -> val 0          -> LowlevelError
//!   suffix ""           -> val 0          -> LowlevelError
//!   suffix "abc"        -> val 0          -> LowlevelError
//!   suffix "4294967296" -> val 4294967295 -> base+4294967294 (mod 2^32)
//!   suffix "-5"         -> val 4294967291 -> base+4294967290 (mod 2^32)
//!   ```
//!
//!   (the `..._f1` tests document finding F1: the port parses the suffix
//!   with base-0 strtoul semantics and uses a non-wrapping add).
//! - PackedDecode chunk machinery (marshal.hh:599-646, marshal.cc:954-988):
//!   strings spanning multiple 1024-byte chunks, an extended attribute id
//!   header whose second byte is the first byte of the next chunk
//!   (`getBytePlus1` cross-chunk path), and `ingestStream`'s NUL-terminated
//!   ingestion (marshal.cc:707-717).
//! - `readSignedIntegerExpectString` dual string/integer parse
//!   (marshal.cc:890-922).
//! - XmlEncode tagStatus machine byte-exactness for ATTRIB_CONTENT writes
//!   (marshal.cc:458-598).

use std::rc::Rc;

use kuna_base::error::KunaError;
use kuna_base::marshal::{
    Decoder, Encoder, IdRegistry, PackedDecode, PackedEncode, XmlDecode, XmlEncode, ATTRIB_ALIGN,
    ATTRIB_CONTENT, ATTRIB_FORMAT, ATTRIB_STORAGE, ATTRIB_UNKNOWN, ATTRIB_VAL, ELEM_DATA,
    ELEM_VALUE,
};
use kuna_base::space::{addrspace_flags, spacetype, AddrSpace, AddrSpaceManager};

fn test_manager() -> AddrSpaceManager {
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

/// Drive `<data NAME="x"/>` through XmlDecode up to the point where C++ would
/// call getIndexedAttributeId(ATTRIB_VAL) (base name "val", id 24).
fn indexed_attr_id(name: &str) -> Result<u32, KunaError> {
    let manager = AddrSpaceManager::new();
    let registry = IdRegistry::with_base_ids();
    let mut dec = XmlDecode::new(&manager, &registry);
    let doc = format!("<data {name}=\"x\"/>");
    dec.ingest_stream(doc.as_bytes()).unwrap();
    dec.open_element_id(&ELEM_DATA).unwrap();
    // Position attributeIndex on the attribute (an unrecognized name comes
    // back as ATTRIB_UNKNOWN, the C++ precondition for reinterpreting it as
    // indexed; the exact base name "val" itself resolves to ATTRIB_VAL).
    let next = dec.get_next_attribute_id().unwrap();
    if name != "val" {
        assert_eq!(next, ATTRIB_UNKNOWN.get_id());
    }
    dec.get_indexed_attribute_id(&ATTRIB_VAL)
}

/// Cases where the Rust port already agrees with the C++ oracle.
#[test]
fn verify_marshal_indexed_attr_happy_and_error_paths() {
    assert_eq!(indexed_attr_id("val1").unwrap(), 24);
    assert_eq!(indexed_attr_id("val23").unwrap(), 46);
    // Exact base-name match and non-numeric suffix: val parses to 0 -> throw
    assert!(matches!(indexed_attr_id("val").unwrap_err(), KunaError::Lowlevel { .. }));
    assert!(matches!(indexed_attr_id("valabc").unwrap_err(), KunaError::Lowlevel { .. }));
    // Name not starting with the base name -> ATTRIB_UNKNOWN, no error
    assert_eq!(indexed_attr_id("zzz9").unwrap(), ATTRIB_UNKNOWN.get_id());
}

/// F1: C++ parses the suffix with `>> dec` (decimal only).  Oracle values:
/// "010" is decimal 10 (not octal 8), "08" is decimal 8 (not a parse error),
/// "0x2" fails the decimal parse (val 0 -> LowlevelError, not hex 2).
#[test]
fn verify_marshal_indexed_attr_dec_suffix_matches_cpp_oracle_f1() {
    assert_eq!(
        indexed_attr_id("val010").unwrap(),
        33,
        "C++ `>> dec` parses suffix 010 as decimal 10 -> id 24+9"
    );
    assert_eq!(
        indexed_attr_id("val08").unwrap(),
        31,
        "C++ `>> dec` parses suffix 08 as decimal 8 -> id 24+7"
    );
    assert!(
        matches!(indexed_attr_id("val0x2").unwrap_err(), KunaError::Lowlevel { .. }),
        "C++ `>> dec` extracts 0 from suffix 0x2 -> Bad indexed attribute"
    );
}

/// F1 (wrap half): C++ `uint4 val` saturates the decimal overflow to
/// UINT_MAX (num_get, C++11) and negative suffixes negate modularly; the
/// subsequent `attribId.getId() + (val-1)` wraps mod 2^32.  Oracle (base id
/// 24): "4294967296" -> 22, "-5" -> 18.  The Rust port must not panic.
#[test]
fn verify_marshal_indexed_attr_wraps_like_cpp_on_huge_index_f1() {
    assert_eq!(
        indexed_attr_id("val4294967296").unwrap(),
        22,
        "C++ stores UINT_MAX for the overflowing decimal and wraps the id add"
    );
    assert_eq!(
        indexed_attr_id("val-5").unwrap(),
        18,
        "C++ negates the magnitude modularly (val 4294967291) and wraps the id add"
    );
}

/// A string attribute whose payload spans three input chunks exercises the
/// multi-chunk loop in PackedDecode::readString (marshal.cc:976-987): the
/// per-chunk copy order and lengths must reassemble the C++ string exactly.
#[test]
fn verify_marshal_packed_string_spans_three_chunks() {
    let manager = test_manager();
    // 3000 non-zero bytes (zero would terminate C++ istream ingestion)
    let payload: Vec<u8> = (0..3000u32).map(|i| (i % 255 + 1) as u8).collect();
    let mut buf = Vec::new();
    {
        let mut enc = PackedEncode::new(&mut buf);
        enc.open_element(&ELEM_DATA);
        enc.write_string(&ATTRIB_VAL, &payload);
        enc.write_bool(&ATTRIB_ALIGN, true);
        enc.close_element(&ELEM_DATA);
    }
    assert!(buf.len() > 2 * 1024 && buf.len() < 3 * 1024);
    let mut dec = PackedDecode::new(&manager);
    dec.ingest_stream(&buf).unwrap();
    let el = dec.open_element_id(&ELEM_DATA).unwrap();
    // Skip over the string attribute first via getNextAttributeId, then
    // re-find it by id: exercises skipAttribute's advancePosition across
    // chunk boundaries AND findMatchingAttribute's rescan-from-startPos.
    assert_eq!(dec.get_next_attribute_id().unwrap(), ATTRIB_VAL.get_id());
    assert_eq!(dec.get_next_attribute_id().unwrap(), ATTRIB_ALIGN.get_id());
    assert!(dec.read_bool().unwrap());
    assert_eq!(dec.read_string_id(&ATTRIB_VAL).unwrap(), payload);
    // Attribute traversal must still work after the id-read reset curPos.
    dec.rewind_attributes();
    assert_eq!(dec.get_next_attribute_id().unwrap(), ATTRIB_VAL.get_id());
    assert_eq!(dec.read_string().unwrap(), payload);
    dec.close_element(el).unwrap();
}

/// Position an extended (2-byte) attribute id header so its first byte is
/// the LAST byte of chunk 0 and its extension byte is the FIRST byte of
/// chunk 1: findMatchingAttribute reads the extension via getBytePlus1's
/// cross-chunk path (marshal.hh:599-611).
#[test]
fn verify_marshal_packed_extended_header_straddles_chunk_boundary() {
    let manager = test_manager();
    // <data> (1 byte) + val-string attr: 1 header + 1 type + 2 length bytes
    // + 1018 payload = offset 1023 for the next attribute header.
    let payload = vec![b'x'; 1018];
    let mut buf = Vec::new();
    {
        let mut enc = PackedEncode::new(&mut buf);
        enc.open_element(&ELEM_DATA);
        enc.write_string(&ATTRIB_VAL, &payload);
        enc.write_bool(&ATTRIB_STORAGE, true); // id 149 > 0x1f: extended header
        enc.close_element(&ELEM_DATA);
    }
    assert_eq!(buf.len(), 1027);
    assert_eq!(buf[1023], 0xc0 | 0x20 | (149u32 >> 7) as u8); // header, last byte of chunk 0
    assert_eq!(buf[1024], 0x80 | (149u32 & 0x7f) as u8); // extension, first byte of chunk 1
    let mut dec = PackedDecode::new(&manager);
    dec.ingest_stream(&buf).unwrap();
    let el = dec.open_element_id(&ELEM_DATA).unwrap();
    assert!(dec.read_bool_id(&ATTRIB_STORAGE).unwrap());
    assert_eq!(dec.read_string_id(&ATTRIB_VAL).unwrap(), payload);
    dec.close_element(el).unwrap();
}

/// C++ ingestStream stops at the first NUL byte (`istream::get(...,'\0')` +
/// `peek() > 0`); bytes after a NUL must be invisible to the decoder, and a
/// stream that starts with NUL (or is empty) is "Ended ingestion without any
/// input".
#[test]
fn verify_marshal_packed_ingest_nul_truncation_and_empty() {
    let manager = test_manager();
    let mut buf = Vec::new();
    {
        let mut enc = PackedEncode::new(&mut buf);
        enc.open_element(&ELEM_DATA);
        enc.write_bool(&ATTRIB_ALIGN, false);
        enc.close_element(&ELEM_DATA);
    }
    // Garbage after a NUL: would be a second element if it were ingested.
    let mut poisoned = buf.clone();
    poisoned.push(0);
    poisoned.extend_from_slice(&[0x41, 0x81, 0xff, 0x07]);
    let mut dec = PackedDecode::new(&manager);
    dec.ingest_stream(&poisoned).unwrap();
    let el = dec.open_element_id(&ELEM_DATA).unwrap();
    assert!(!dec.read_bool_id(&ATTRIB_ALIGN).unwrap());
    dec.close_element(el).unwrap();
    assert_eq!(dec.peek_element().unwrap(), 0, "post-NUL bytes must not be ingested");

    let mut dec2 = PackedDecode::new(&manager);
    assert!(matches!(dec2.ingest_stream(&[]).unwrap_err(), KunaError::Decoder { .. }));
    let mut dec3 = PackedDecode::new(&manager);
    assert!(matches!(
        dec3.ingest_stream(&[0x00, 0x41, 0x81]).unwrap_err(),
        KunaError::Decoder { .. }
    ));
}

/// readSignedIntegerExpectString (packed): a matching string yields the
/// sentinel, a mismatching string is a DecoderError, an integer attribute
/// falls through to readSignedInteger; curPos peeking must not consume the
/// attribute (marshal.cc:890-922).
#[test]
fn verify_marshal_packed_expect_string_paths() {
    let manager = test_manager();
    // Case 1+2: string attribute
    let mut buf = Vec::new();
    {
        let mut enc = PackedEncode::new(&mut buf);
        enc.open_element(&ELEM_DATA);
        enc.write_string(&ATTRIB_FORMAT, b"blob");
        enc.close_element(&ELEM_DATA);
    }
    let mut dec = PackedDecode::new(&manager);
    dec.ingest_stream(&buf).unwrap();
    let el = dec.open_element_id(&ELEM_DATA).unwrap();
    assert_eq!(
        dec.read_signed_integer_expect_string_id(&ATTRIB_FORMAT, b"blob", -42).unwrap(),
        -42
    );
    // Re-read with a different expectation: mismatching string is an error
    let err = dec
        .read_signed_integer_expect_string_id(&ATTRIB_FORMAT, b"other", -42)
        .unwrap_err();
    assert!(matches!(err, KunaError::Decoder { .. }));
    // The failed read must not have corrupted element traversal
    assert_eq!(
        dec.read_signed_integer_expect_string_id(&ATTRIB_FORMAT, b"blob", -42).unwrap(),
        -42
    );
    dec.close_element(el).unwrap();

    // Case 3: integer attribute falls through to readSignedInteger
    let mut buf2 = Vec::new();
    {
        let mut enc = PackedEncode::new(&mut buf2);
        enc.open_element(&ELEM_DATA);
        enc.write_signed_integer(&ATTRIB_FORMAT, 1234);
        enc.close_element(&ELEM_DATA);
    }
    let mut dec2 = PackedDecode::new(&manager);
    dec2.ingest_stream(&buf2).unwrap();
    let el2 = dec2.open_element_id(&ELEM_DATA).unwrap();
    assert_eq!(
        dec2.read_signed_integer_expect_string_id(&ATTRIB_FORMAT, b"blob", -42).unwrap(),
        1234
    );
    dec2.close_element(el2).unwrap();
}

/// XmlEncode tagStatus machine, byte-exact (C++ marshal.cc:458-598): content
/// writes close the open tag with '>' exactly once, suppress the closing
/// newline, and escape through xml_escape; unsigned content renders as
/// hex "0x...".  Round-trips back through XmlDecode's ATTRIB_CONTENT reads.
#[test]
fn verify_marshal_xml_content_tagstatus_exact_bytes() {
    let manager = AddrSpaceManager::new();
    let registry = IdRegistry::with_base_ids();

    let mut buf = Vec::new();
    {
        let mut enc = XmlEncode::new(&mut buf);
        enc.open_element(&ELEM_DATA);
        enc.write_bool(&ATTRIB_ALIGN, true);
        enc.write_string(&ATTRIB_CONTENT, b"he<llo&");
        enc.close_element(&ELEM_DATA);
    }
    assert_eq!(
        String::from_utf8(buf.clone()).unwrap(),
        "\n<data align=\"true\">he&lt;llo&amp;</data>"
    );
    let mut dec = XmlDecode::new(&manager, &registry);
    dec.ingest_stream(&buf).unwrap();
    let el = dec.open_element_id(&ELEM_DATA).unwrap();
    assert_eq!(dec.read_string_id(&ATTRIB_CONTENT).unwrap(), b"he<llo&");
    assert!(dec.read_bool_id(&ATTRIB_ALIGN).unwrap());
    dec.close_element(el).unwrap();

    // Nested element with unsigned content; closing </data> takes the
    // newline path (tagStatus == tag_stop after </value>).
    let mut buf2 = Vec::new();
    {
        let mut enc = XmlEncode::new(&mut buf2);
        enc.open_element(&ELEM_DATA);
        enc.open_element(&ELEM_VALUE);
        enc.write_unsigned_integer(&ATTRIB_CONTENT, 0xbeef);
        enc.close_element(&ELEM_VALUE);
        enc.close_element(&ELEM_DATA);
    }
    assert_eq!(
        String::from_utf8(buf2.clone()).unwrap(),
        "\n<data>\n  <value>0xbeef</value>\n</data>"
    );
    let mut dec2 = XmlDecode::new(&manager, &registry);
    dec2.ingest_stream(&buf2).unwrap();
    let el1 = dec2.open_element_id(&ELEM_DATA).unwrap();
    let el2 = dec2.open_element_id(&ELEM_VALUE).unwrap();
    assert_eq!(dec2.read_unsigned_integer_id(&ATTRIB_CONTENT).unwrap(), 0xbeef);
    dec2.close_element(el2).unwrap();
    dec2.close_element(el1).unwrap();
}
