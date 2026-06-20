//! Verifier adversarial tests for item `w1-base-marshal` (round 2).
//!
//! Round 2 targets the THREE sites the repair commit (60bcd4d) touched, at
//! the boundaries the round-1 work-order tests did not pin:
//!
//! - `cxx_num_get_u32_dec` via the public `XmlDecode::get_indexed_attribute_id`
//!   (marshal.cc:243-260): the libstdc++ `num_get::_M_extract_int` branch
//!   structure has THREE distinct outcomes around u32::MAX —
//!   exact-max (no overflow, stored as-is), negative non-overflow (modular
//!   negation), and negative OVERFLOW (stores `__max` = UINT_MAX because the
//!   destination is unsigned: `__negative && __is_signed` is false).  All
//!   expected values below were produced by running the exact C++ statement
//!   sequence (g++ -std=c++11, libstdc++, base id 24 = ATTRIB_VAL):
//!
//!   ```text
//!   suffix "4294967295"  -> val 4294967295 -> id 22   (exact max, no overflow)
//!   suffix "-4294967295" -> val 1          -> id 24   (modular negation)
//!   suffix "-4294967296" -> val 4294967295 -> id 22   (negative overflow -> UINT_MAX)
//!   suffix "-0"          -> val 0          -> LowlevelError
//!   suffix "007"         -> val 7          -> id 30
//!   suffix "1-2"         -> val 1          -> id 24   (sign only valid before digits)
//!   suffix "12.7"        -> val 12         -> id 35   (extraction stops at '.')
//!   suffix ".5"          -> val 0          -> LowlevelError (no digits consumed)
//!   ```
//!
//!   '-' and '.' are XML NameChars, so these suffixes arrive through real
//!   documents, not just synthetic API calls.
//!
//! - `XmlEncode::writeStringIndexed` (marshal.cc:559-566): `<< dec << index+1`
//!   is uint4 arithmetic; index u32::MAX must render "<name>0" (wrapped), and
//!   the resulting attribute must then FAIL to decode (val 0 -> throw), which
//!   is the genuine C++ round-trip behavior for that index.
//!
//! - `PackedEncode::writeStringIndexed` (marshal.cc:1188-1194):
//!   `writeHeader(ATTRIBUTE, attribId.getId() + index)` is uint4 arithmetic;
//!   ATTRIB_VAL (24) + u32::MAX wraps to header id 23, byte 0xc0|23 = 0xd7.

use std::rc::Rc;

use kuna_base::error::KunaError;
use kuna_base::marshal::{
    Decoder, Encoder, IdRegistry, PackedDecode, PackedEncode, XmlDecode, XmlEncode, ATTRIB_UNKNOWN,
    ATTRIB_VAL, ELEM_DATA,
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

/// Drive `<data NAME="x"/>` through XmlDecode up to getIndexedAttributeId
/// (same scaffold as round 1; base name "val", id 24).
fn indexed_attr_id(name: &str) -> Result<u32, KunaError> {
    let manager = AddrSpaceManager::new();
    let registry = IdRegistry::with_base_ids();
    let mut dec = XmlDecode::new(&manager, &registry);
    let doc = format!("<data {name}=\"x\"/>");
    dec.ingest_stream(doc.as_bytes()).unwrap();
    dec.open_element_id(&ELEM_DATA).unwrap();
    let next = dec.get_next_attribute_id().unwrap();
    if name != "val" {
        assert_eq!(next, ATTRIB_UNKNOWN.get_id());
    }
    dec.get_indexed_attribute_id(&ATTRIB_VAL)
}

/// The three num_get outcomes clustered at the u32 boundary.  The negative
/// OVERFLOW case is the subtlest line of the repair: libstdc++ stores
/// `__max` (not the negated magnitude) because the destination is unsigned,
/// so "-4294967296" and "4294967295" land on the SAME id.
#[test]
fn verify_marshal_indexed_attr_negative_overflow_boundary_r2() {
    assert_eq!(
        indexed_attr_id("val4294967295").unwrap(),
        22,
        "exact UINT_MAX magnitude: no overflow, id 24 + (MAX-1) wraps to 22"
    );
    assert_eq!(
        indexed_attr_id("val-4294967295").unwrap(),
        24,
        "modular negation: -(MAX) mod 2^32 = 1 -> id 24 + 0"
    );
    assert_eq!(
        indexed_attr_id("val-4294967296").unwrap(),
        22,
        "negative overflow of an unsigned destination stores UINT_MAX (not min)"
    );
    assert!(
        matches!(indexed_attr_id("val-0").unwrap_err(), KunaError::Lowlevel { .. }),
        "negated zero is still zero -> Bad indexed attribute"
    );
    assert_eq!(indexed_attr_id("val007").unwrap(), 30, "leading zeros are decimal: 7 -> 24+6");
}

/// Partial extraction: digits stop at the first non-digit; a '-' AFTER
/// digits is not a sign; no digits before a NameChar like '.' is a failed
/// extraction (val 0 -> throw).  All oracle-pinned.
#[test]
fn verify_marshal_indexed_attr_suffix_stops_at_first_nondigit_r2() {
    assert_eq!(indexed_attr_id("val1-2").unwrap(), 24, "extracts 1, stops at '-'");
    assert_eq!(indexed_attr_id("val12.7").unwrap(), 35, "extracts 12, stops at '.'");
    assert!(
        matches!(indexed_attr_id("val.5").unwrap_err(), KunaError::Lowlevel { .. }),
        "no digits before '.': failed extraction stores 0 -> throw"
    );
}

/// XmlEncode::writeStringIndexed index wrap, byte-exact: C++ streams
/// `index + 1` in uint4 arithmetic, so u32::MAX renders "val0" and 0 renders
/// "val1".  The wrapped name then fails to decode exactly like C++ (val 0).
#[test]
fn verify_marshal_xml_write_string_indexed_index_wrap_bytes_r2() {
    let mut buf = Vec::new();
    {
        let mut enc = XmlEncode::new(&mut buf);
        enc.open_element(&ELEM_DATA);
        enc.write_string_indexed(&ATTRIB_VAL, u32::MAX, b"x");
        enc.write_string_indexed(&ATTRIB_VAL, 0, b"y");
        enc.close_element(&ELEM_DATA);
    }
    assert_eq!(
        String::from_utf8(buf.clone()).unwrap(),
        "\n<data val0=\"x\" val1=\"y\"/>",
        "C++ `<< dec << index + 1` wraps u32::MAX+1 to 0"
    );
    // Round-trip: the wrapped "val0" attribute is undecodable (C++ throws on
    // val == 0); "val1" decodes back to the base id.
    let manager = AddrSpaceManager::new();
    let registry = IdRegistry::with_base_ids();
    let mut dec = XmlDecode::new(&manager, &registry);
    dec.ingest_stream(&buf).unwrap();
    dec.open_element_id(&ELEM_DATA).unwrap();
    assert_eq!(dec.get_next_attribute_id().unwrap(), ATTRIB_UNKNOWN.get_id());
    assert!(matches!(
        dec.get_indexed_attribute_id(&ATTRIB_VAL).unwrap_err(),
        KunaError::Lowlevel { .. }
    ));
    assert_eq!(dec.get_next_attribute_id().unwrap(), ATTRIB_UNKNOWN.get_id());
    assert_eq!(dec.get_indexed_attribute_id(&ATTRIB_VAL).unwrap(), ATTRIB_VAL.get_id());
}

/// PackedEncode::writeStringIndexed id wrap: ATTRIB_VAL (24) + u32::MAX is
/// uint4 arithmetic -> header id 23 (single-byte header 0xc0|23 = 0xd7); the
/// stream decodes as attribute id 23 carrying the string payload.  Non-wrap
/// sanity: index 2 -> id 26.
#[test]
fn verify_marshal_packed_write_string_indexed_id_wrap_r2() {
    let manager = test_manager();
    let mut buf = Vec::new();
    {
        let mut enc = PackedEncode::new(&mut buf);
        enc.open_element(&ELEM_DATA);
        enc.write_string_indexed(&ATTRIB_VAL, u32::MAX, b"hi");
        enc.write_string_indexed(&ATTRIB_VAL, 2, b"yo");
        enc.close_element(&ELEM_DATA);
    }
    assert_eq!(buf[1], 0xc0 | 23, "wrapped attribute header: ATTRIBUTE | (24 + MAX mod 2^32)");
    let mut dec = PackedDecode::new(&manager);
    dec.ingest_stream(&buf).unwrap();
    let el = dec.open_element_id(&ELEM_DATA).unwrap();
    assert_eq!(dec.get_next_attribute_id().unwrap(), 23, "24 + u32::MAX wraps to 23");
    assert_eq!(dec.read_string().unwrap(), b"hi");
    assert_eq!(dec.get_next_attribute_id().unwrap(), 26, "24 + 2, no wrap");
    assert_eq!(dec.read_string().unwrap(), b"yo");
    dec.close_element(el).unwrap();
}
