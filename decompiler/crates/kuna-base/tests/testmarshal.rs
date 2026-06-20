//! Port of the C++ unit tests in `decompiler/unittests/testmarshal.cc`.
//!
//! One C++ `TEST(name)` -> one `#[test] fn name()`, assertion for
//! assertion, same constants, same order.
//!
//! Scaffolding mapping (testmarshal.cc:21-66):
//! - `DummyTranslate` exists in C++ only to seed `AddrSpace` construction
//!   (`t->isBigEndian()` is `Translate`'s default `false`); the Rust
//!   `AddrSpace::new` takes the values directly, so no translator object is
//!   needed.
//! - `TestAddrSpaceManager` inserts one "ram" space:
//!   `AddrSpace(this,t,IPTR_PROCESSOR,"ram",false,8,1,3,hasphysical,1,1)`.
//! - The static `theEnviron` / `spcManager` pair becomes [`the_environ_build`],
//!   called per test (each Rust test runs in its own process group, so the
//!   shared static is rebuilt instead).
//!
//! The shared C++ helpers (`test_signed_attributes`, ...) each take the
//! encoder, the decoder, AND the output stream; in Rust the encoder mutably
//! borrows the output buffer, so each helper is split in two at its
//! `istringstream inStream(outStream.str())` line: an `*_encode` half
//! (everything before) and a `*_decode` half (everything after).  Statement
//! order inside each half is unchanged.

use std::rc::Rc;

use kuna_base::address::ELEM_ADDR;
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::marshal::{
    Decoder, Encoder, IdRegistry, PackedDecode, PackedEncode, XmlDecode, XmlEncode, ATTRIB_ALIGN,
    ATTRIB_BIGENDIAN, ATTRIB_CONSTRUCTOR, ATTRIB_CONTENT, ATTRIB_DESTRUCTOR, ATTRIB_EXTRAPOP,
    ATTRIB_FORMAT, ATTRIB_ID, ATTRIB_INDEX, ATTRIB_METATYPE, ATTRIB_MODEL, ATTRIB_NAME,
    ATTRIB_SPACE, ATTRIB_VAL, ATTRIB_VALUE, ELEM_DATA, ELEM_INPUT, ELEM_OFF, ELEM_OUTPUT,
    ELEM_SYMBOL, ELEM_TARGET,
};
use kuna_base::space::{addrspace_flags, spacetype, AddrSpace, AddrSpaceManager};

/// C++ `TestAddrSpaceManager::TestAddrSpaceManager` + `MarshalTestEnvironment::build`
fn the_environ_build() -> AddrSpaceManager {
    let mut manager = AddrSpaceManager::new();
    let ram = AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "ram",
        false, // DummyTranslate: Translate's default endianness
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

// C++ `test_signed_attributes` (testmarshal.cc:68-137), encode half
fn test_signed_attributes_encode<E: Encoder>(encoder: &mut E) {
    encoder.open_element(&ELEM_ADDR);
    encoder.write_signed_integer(&ATTRIB_ALIGN, 3); // 7-bits
    encoder.write_signed_integer(&ATTRIB_BIGENDIAN, -0x100); // 14-bits
    encoder.write_signed_integer(&ATTRIB_CONSTRUCTOR, 0x1fffff); // 21-bits
    encoder.write_signed_integer(&ATTRIB_DESTRUCTOR, -0xabcdefa); // 28-bits
    encoder.write_signed_integer(&ATTRIB_EXTRAPOP, 0x300000000); // 35-bits
    encoder.write_signed_integer(&ATTRIB_FORMAT, -0x30101010101); // 42-bits
    encoder.write_signed_integer(&ATTRIB_ID, 0x123456789011); // 49-bits
    encoder.write_signed_integer(&ATTRIB_INDEX, -0xf0f0f0f0f0f0f0); // 56-bits
    encoder.write_signed_integer(&ATTRIB_METATYPE, 0x7fffffffffffffff); // 63-bits
    encoder.close_element(&ELEM_ADDR);
}

// C++ `test_signed_attributes`, decode half
fn test_signed_attributes_decode<D: Decoder>(decoder: &mut D, out_stream: &[u8]) {
    decoder.ingest_stream(out_stream).unwrap();
    let el = decoder.open_element_id(&ELEM_ADDR).unwrap();
    let mut flags: u32 = 0;
    loop {
        let attrib_id = decoder.get_next_attribute_id().unwrap();
        if attrib_id == 0 {
            break;
        }
        if attrib_id == ATTRIB_ALIGN {
            let val: i64 = decoder.read_signed_integer().unwrap();
            flags |= 1;
            assert_eq!(val, 3);
        } else if attrib_id == ATTRIB_BIGENDIAN {
            let val: i64 = decoder.read_signed_integer().unwrap();
            flags |= 2;
            assert_eq!(val, -0x100);
        } else if attrib_id == ATTRIB_CONSTRUCTOR {
            let val: i64 = decoder.read_signed_integer().unwrap();
            flags |= 4;
            assert_eq!(val, 0x1fffff);
        } else if attrib_id == ATTRIB_DESTRUCTOR {
            let val: i64 = decoder.read_signed_integer().unwrap();
            flags |= 8;
            assert_eq!(val, -0xabcdefa);
        } else if attrib_id == ATTRIB_EXTRAPOP {
            let val: i64 = decoder.read_signed_integer().unwrap();
            flags |= 0x10;
            assert_eq!(val, 0x300000000);
        } else if attrib_id == ATTRIB_FORMAT {
            let val: i64 = decoder.read_signed_integer().unwrap();
            flags |= 0x20;
            assert_eq!(val, -0x30101010101);
        } else if attrib_id == ATTRIB_ID {
            let val: i64 = decoder.read_signed_integer().unwrap();
            flags |= 0x40;
            assert_eq!(val, 0x123456789011);
        } else if attrib_id == ATTRIB_INDEX {
            let val: i64 = decoder.read_signed_integer().unwrap();
            flags |= 0x80;
            assert_eq!(val, -0xf0f0f0f0f0f0f0);
        } else if attrib_id == ATTRIB_METATYPE {
            let val: i64 = decoder.read_signed_integer().unwrap();
            flags |= 0x100;
            assert_eq!(val, 0x7fffffffffffffff);
        }
    }
    decoder.close_element(el).unwrap();
    assert_eq!(flags, 0x1ff);
}

// C++ `test_unsigned_attributes` (testmarshal.cc:139-178), encode half
fn test_unsigned_attributes_encode<E: Encoder>(encoder: &mut E) {
    encoder.open_element(&ELEM_ADDR);
    encoder.write_unsigned_integer(&ATTRIB_ALIGN, 3); // 7-bits
    encoder.write_unsigned_integer(&ATTRIB_BIGENDIAN, 0x100); // 14-bits
    encoder.write_unsigned_integer(&ATTRIB_CONSTRUCTOR, 0x1fffff); // 21-bits
    encoder.write_unsigned_integer(&ATTRIB_DESTRUCTOR, 0xabcdefa); // 28-bits
    encoder.write_unsigned_integer(&ATTRIB_EXTRAPOP, 0x300000000); // 35-bits
    encoder.write_unsigned_integer(&ATTRIB_FORMAT, 0x30101010101); // 42-bits
    encoder.write_unsigned_integer(&ATTRIB_ID, 0x123456789011); // 49-bits
    encoder.write_unsigned_integer(&ATTRIB_INDEX, 0xf0f0f0f0f0f0f0); // 56-bits
    encoder.write_unsigned_integer(&ATTRIB_METATYPE, 0x7fffffffffffffff); // 63-bits
    encoder.write_unsigned_integer(&ATTRIB_MODEL, 0x8000000000000000); // 64-bits
    encoder.close_element(&ELEM_ADDR);
}

// C++ `test_unsigned_attributes`, decode half
fn test_unsigned_attributes_decode<D: Decoder>(decoder: &mut D, out_stream: &[u8]) {
    decoder.ingest_stream(out_stream).unwrap();
    let el = decoder.open_element_id(&ELEM_ADDR).unwrap();
    let mut val: u64 = decoder.read_unsigned_integer_id(&ATTRIB_ALIGN).unwrap();
    assert_eq!(val, 3);
    val = decoder.read_unsigned_integer_id(&ATTRIB_BIGENDIAN).unwrap();
    assert_eq!(val, 0x100);
    val = decoder.read_unsigned_integer_id(&ATTRIB_CONSTRUCTOR).unwrap();
    assert_eq!(val, 0x1fffff);
    val = decoder.read_unsigned_integer_id(&ATTRIB_DESTRUCTOR).unwrap();
    assert_eq!(val, 0xabcdefa);
    val = decoder.read_unsigned_integer_id(&ATTRIB_EXTRAPOP).unwrap();
    assert_eq!(val, 0x300000000);
    val = decoder.read_unsigned_integer_id(&ATTRIB_FORMAT).unwrap();
    assert_eq!(val, 0x30101010101);
    val = decoder.read_unsigned_integer_id(&ATTRIB_ID).unwrap();
    assert_eq!(val, 0x123456789011);
    val = decoder.read_unsigned_integer_id(&ATTRIB_INDEX).unwrap();
    assert_eq!(val, 0xf0f0f0f0f0f0f0);
    val = decoder.read_unsigned_integer_id(&ATTRIB_METATYPE).unwrap();
    assert_eq!(val, 0x7fffffffffffffff);
    val = decoder.read_unsigned_integer_id(&ATTRIB_MODEL).unwrap();
    assert_eq!(val, 0x8000000000000000);
    decoder.close_element(el).unwrap();
}

#[test]
fn marshal_signed_packed() {
    let mut out_stream: Vec<u8> = Vec::new();

    let spc_manager = the_environ_build();
    {
        let mut encoder = PackedEncode::new(&mut out_stream);
        test_signed_attributes_encode(&mut encoder);
    }
    let mut decoder = PackedDecode::new(&spc_manager);
    test_signed_attributes_decode(&mut decoder, &out_stream);
}

#[test]
fn marshal_signed_xml() {
    let mut out_stream: Vec<u8> = Vec::new();

    let spc_manager = the_environ_build();
    let registry = IdRegistry::with_base_ids();
    {
        let mut encoder = XmlEncode::new(&mut out_stream);
        test_signed_attributes_encode(&mut encoder);
    }
    let mut decoder = XmlDecode::new(&spc_manager, &registry);
    test_signed_attributes_decode(&mut decoder, &out_stream);
}

#[test]
fn marshal_unsigned_packed() {
    let mut out_stream: Vec<u8> = Vec::new();

    let spc_manager = the_environ_build();
    {
        let mut encoder = PackedEncode::new(&mut out_stream);
        test_unsigned_attributes_encode(&mut encoder);
    }
    let mut decoder = PackedDecode::new(&spc_manager);
    test_unsigned_attributes_decode(&mut decoder, &out_stream);
}

#[test]
fn marshal_unsigned_xml() {
    let mut out_stream: Vec<u8> = Vec::new();

    let spc_manager = the_environ_build();
    let registry = IdRegistry::with_base_ids();
    {
        let mut encoder = XmlEncode::new(&mut out_stream);
        test_unsigned_attributes_encode(&mut encoder);
    }
    let mut decoder = XmlDecode::new(&spc_manager, &registry);
    test_unsigned_attributes_decode(&mut decoder, &out_stream);
}

// C++ `test_mixed_attributes` (testmarshal.cc:216-239), encode half
fn test_mixed_attributes_encode<E: Encoder>(encoder: &mut E) {
    encoder.open_element(&ELEM_ADDR);
    encoder.write_signed_integer(&ATTRIB_ALIGN, 456);
    encoder.write_string(&ATTRIB_EXTRAPOP, b"unknown");
    encoder.close_element(&ELEM_ADDR);
}

// C++ `test_mixed_attributes`, decode half
fn test_mixed_attributes_decode<D: Decoder>(decoder: &mut D, out_stream: &[u8]) {
    decoder.ingest_stream(out_stream).unwrap();
    let mut align_val: i32 = -1;
    let mut extrapop_val: i32 = -1;
    let el = decoder.open_element_id(&ELEM_ADDR).unwrap();
    loop {
        let attrib_id = decoder.get_next_attribute_id().unwrap();
        if attrib_id == 0 {
            break;
        }
        if attrib_id == ATTRIB_ALIGN {
            // C++ assigns the int8 result into int4 (truncating store)
            align_val = decoder
                .read_signed_integer_expect_string(b"00blah", 700)
                .unwrap() as i32;
        } else if attrib_id == ATTRIB_EXTRAPOP {
            extrapop_val = decoder
                .read_signed_integer_expect_string(b"unknown", 800)
                .unwrap() as i32;
        }
    }
    decoder.close_element(el).unwrap();
    assert_eq!(align_val, 456);
    assert_eq!(extrapop_val, 800);
}

#[test]
fn marshal_mixed_packed() {
    let mut out_stream: Vec<u8> = Vec::new();

    let spc_manager = the_environ_build();
    {
        let mut encoder = PackedEncode::new(&mut out_stream);
        test_mixed_attributes_encode(&mut encoder);
    }
    let mut decoder = PackedDecode::new(&spc_manager);
    test_mixed_attributes_decode(&mut decoder, &out_stream);
}

#[test]
fn marshal_mixed_xml() {
    let mut out_stream: Vec<u8> = Vec::new();

    let spc_manager = the_environ_build();
    let registry = IdRegistry::with_base_ids();
    {
        let mut encoder = XmlEncode::new(&mut out_stream);
        test_mixed_attributes_encode(&mut encoder);
    }
    let mut decoder = XmlDecode::new(&spc_manager, &registry);
    test_mixed_attributes_decode(&mut decoder, &out_stream);
}

/// C++ `longString` (testmarshal.cc:271-273)
const LONG_STRING: &[u8] = b"one to three four five six seven eight nine ten eleven twelve thirteen \
fourteen fifteen sixteen seventeen eighteen nineteen twenty twenty one \
blahblahblahblahblahblahblahblahblahblahblahblahblahblahblahblahblahblah";

// C++ `test_attributes` (testmarshal.cc:260-294), encode half.  The C++
// helper reads the space to encode from the global `spcManager`.
fn test_attributes_encode<E: Encoder>(encoder: &mut E, spc_manager: &AddrSpaceManager) {
    encoder.open_element(&ELEM_DATA);
    encoder.write_bool(&ATTRIB_ALIGN, true);
    encoder.write_bool(&ATTRIB_BIGENDIAN, false);
    let spc = spc_manager.get_space(3).unwrap();
    encoder.write_space(&ATTRIB_SPACE, spc);
    encoder.write_string(&ATTRIB_VAL, b""); // Empty string
    encoder.write_string(&ATTRIB_VALUE, b"hello");
    encoder.write_string(&ATTRIB_CONSTRUCTOR, b"<<\xe2\x82\xac>>&\"bl a  h'\\bleh\n\t");
    encoder.write_string(&ATTRIB_DESTRUCTOR, LONG_STRING);
    encoder.close_element(&ELEM_DATA);
}

// C++ `test_attributes`, decode half
fn test_attributes_decode<D: Decoder>(
    decoder: &mut D,
    out_stream: &[u8],
    spc_manager: &AddrSpaceManager,
) {
    decoder.ingest_stream(out_stream).unwrap();
    let el = decoder.open_element_id(&ELEM_DATA).unwrap();
    let bval = decoder.read_bool_id(&ATTRIB_ALIGN).unwrap();
    assert!(bval);
    let bval = decoder.read_bool_id(&ATTRIB_BIGENDIAN).unwrap();
    assert!(!bval);
    let spc = decoder.read_space_id(&ATTRIB_SPACE).unwrap();
    // C++ ASSERT_EQUALS compares AddrSpace pointers; Rc identity is the
    // pointer-equality analogue.
    assert!(Rc::ptr_eq(&spc, spc_manager.get_space(3).unwrap()));
    let val = decoder.read_string_id(&ATTRIB_VAL).unwrap();
    assert_eq!(val, b"");
    let val = decoder.read_string_id(&ATTRIB_VALUE).unwrap();
    assert_eq!(val, b"hello");
    let val = decoder.read_string_id(&ATTRIB_CONSTRUCTOR).unwrap();
    assert_eq!(val, b"<<\xe2\x82\xac>>&\"bl a  h'\\bleh\n\t");
    let val = decoder.read_string_id(&ATTRIB_DESTRUCTOR).unwrap();
    assert_eq!(val, LONG_STRING);
    decoder.close_element(el).unwrap();
}

#[test]
fn marshal_attribs_packed() {
    let mut out_stream: Vec<u8> = Vec::new();
    let spc_manager = the_environ_build();
    {
        let mut encoder = PackedEncode::new(&mut out_stream);
        test_attributes_encode(&mut encoder, &spc_manager);
    }
    let mut decoder = PackedDecode::new(&spc_manager);
    test_attributes_decode(&mut decoder, &out_stream, &spc_manager);
}

#[test]
fn marshal_attribs_xml() {
    let mut out_stream: Vec<u8> = Vec::new();
    let spc_manager = the_environ_build();
    let registry = IdRegistry::with_base_ids();
    {
        let mut encoder = XmlEncode::new(&mut out_stream);
        test_attributes_encode(&mut encoder, &spc_manager);
    }
    let mut decoder = XmlDecode::new(&spc_manager, &registry);
    test_attributes_decode(&mut decoder, &out_stream, &spc_manager);
}

// C++ `test_hierarchy` (testmarshal.cc:312-379), encode half
fn test_hierarchy_encode<E: Encoder>(encoder: &mut E) {
    encoder.open_element(&ELEM_DATA); // el1
    encoder.write_bool(&ATTRIB_CONTENT, true);
    encoder.open_element(&ELEM_INPUT); // el2
    encoder.open_element(&ELEM_OUTPUT); // el3
    encoder.write_signed_integer(&ATTRIB_ID, 0x1000);
    encoder.open_element(&ELEM_DATA); // el4
    encoder.open_element(&ELEM_DATA); // el5
    encoder.open_element(&ELEM_OFF); // el6
    encoder.close_element(&ELEM_OFF);
    encoder.open_element(&ELEM_OFF); // el6
    encoder.write_string(&ATTRIB_ID, b"blahblah");
    encoder.close_element(&ELEM_OFF);
    encoder.open_element(&ELEM_OFF); // el6
    encoder.close_element(&ELEM_OFF);
    encoder.close_element(&ELEM_DATA); // close el5
    encoder.close_element(&ELEM_DATA); // close el4
    encoder.open_element(&ELEM_SYMBOL); // skip4
    encoder.write_unsigned_integer(&ATTRIB_ID, 17);
    encoder.open_element(&ELEM_TARGET); // skip5
    encoder.close_element(&ELEM_TARGET); // close skip5
    encoder.close_element(&ELEM_SYMBOL); // close skip4
    encoder.close_element(&ELEM_OUTPUT); // close el3
    encoder.close_element(&ELEM_INPUT); // close el2
    encoder.open_element(&ELEM_INPUT); // el2
    encoder.close_element(&ELEM_INPUT);
    encoder.open_element(&ELEM_INPUT); // el2
    encoder.close_element(&ELEM_INPUT);
    encoder.open_element(&ELEM_INPUT); // el2
    encoder.close_element(&ELEM_INPUT);
    encoder.open_element(&ELEM_INPUT); // el2
    encoder.close_element(&ELEM_INPUT);
    encoder.open_element(&ELEM_INPUT); // el2
    encoder.close_element(&ELEM_INPUT);
    encoder.open_element(&ELEM_INPUT); // el2
    encoder.close_element(&ELEM_INPUT);
    encoder.close_element(&ELEM_DATA); // close el1
}

// C++ `test_hierarchy`, decode half
fn test_hierarchy_decode<D: Decoder>(decoder: &mut D, out_stream: &[u8]) {
    decoder.ingest_stream(out_stream).unwrap();
    let el1 = decoder.open_element_id(&ELEM_DATA).unwrap();
    // Skip over the bool
    let el2 = decoder.open_element_id(&ELEM_INPUT).unwrap();
    let el3 = decoder.open_element_id(&ELEM_OUTPUT).unwrap();
    // C++ assigns the int8 result into int4 (truncating store)
    let val = decoder.read_signed_integer_id(&ATTRIB_ID).unwrap() as i32;
    assert_eq!(val, 0x1000);
    let el4 = decoder.peek_element().unwrap();
    assert_eq!(el4, ELEM_DATA.get_id());
    decoder.open_element().unwrap();
    let el5 = decoder.open_element().unwrap();
    assert_eq!(el5, ELEM_DATA.get_id());
    let el6 = decoder.open_element_id(&ELEM_OFF).unwrap();
    decoder.close_element(el6).unwrap();
    let el6 = decoder.open_element_id(&ELEM_OFF).unwrap();
    decoder.close_element(el6).unwrap();
    let el6 = decoder.open_element_id(&ELEM_OFF).unwrap();
    decoder.close_element(el6).unwrap();
    decoder.close_element(el5).unwrap();
    decoder.close_element(el4).unwrap();
    decoder.close_element_skipping(el3).unwrap();
    decoder.close_element(el2).unwrap();
    let el2 = decoder.open_element_id(&ELEM_INPUT).unwrap();
    decoder.close_element(el2).unwrap();
    let el2 = decoder.open_element_id(&ELEM_INPUT).unwrap();
    decoder.close_element(el2).unwrap();
    decoder.close_element_skipping(el1).unwrap();
}

#[test]
fn marshal_hierarchy_packed() {
    let mut out_stream: Vec<u8> = Vec::new();
    let spc_manager = the_environ_build();
    {
        let mut encoder = PackedEncode::new(&mut out_stream);
        test_hierarchy_encode(&mut encoder);
    }
    let mut decoder = PackedDecode::new(&spc_manager);
    test_hierarchy_decode(&mut decoder, &out_stream);
}

#[test]
fn marshal_hierarchy_xml() {
    let mut out_stream: Vec<u8> = Vec::new();
    let spc_manager = the_environ_build();
    let registry = IdRegistry::with_base_ids();
    {
        let mut encoder = XmlEncode::new(&mut out_stream);
        test_hierarchy_encode(&mut encoder);
    }
    let mut decoder = XmlDecode::new(&spc_manager, &registry);
    test_hierarchy_decode(&mut decoder, &out_stream);
}

// C++ `test_unexpected_eof` (testmarshal.cc:397-416), encode half.  Note the
// outer <data> element is deliberately never closed.
fn test_unexpected_eof_encode<E: Encoder>(encoder: &mut E) {
    encoder.open_element(&ELEM_DATA);
    encoder.open_element(&ELEM_INPUT);
    encoder.write_string(&ATTRIB_NAME, b"hello");
    encoder.close_element(&ELEM_INPUT);
}

// C++ `test_unexpected_eof`, decode half.  The C++ try block becomes a
// `?`-propagating closure; `catch(DecoderError &err)` becomes the
// `KunaError::Decoder` match (any other error variant would leave the flag
// unset and fail the assert, exactly like an uncaught C++ exception).
fn test_unexpected_eof_decode<D: Decoder>(decoder: &mut D, out_stream: &[u8]) {
    let result = (|| -> KunaResult<()> {
        decoder.ingest_stream(out_stream)?;
        let el1 = decoder.open_element_id(&ELEM_DATA)?;
        let el2 = decoder.open_element_id(&ELEM_INPUT)?;
        decoder.close_element(el2)?;
        decoder.close_element(el1)?;
        Ok(())
    })();
    let saw_unexpected_error = matches!(result, Err(KunaError::Decoder { .. }));
    assert!(saw_unexpected_error);
}

#[test]
fn marshal_unexpected_packed() {
    let mut out_stream: Vec<u8> = Vec::new();
    let spc_manager = the_environ_build();
    {
        let mut encoder = PackedEncode::new(&mut out_stream);
        test_unexpected_eof_encode(&mut encoder);
    }
    let mut decoder = PackedDecode::new(&spc_manager);
    test_unexpected_eof_decode(&mut decoder, &out_stream);
}

#[test]
fn marshal_unexpected_xml() {
    let mut out_stream: Vec<u8> = Vec::new();
    let spc_manager = the_environ_build();
    let registry = IdRegistry::with_base_ids();
    {
        let mut encoder = XmlEncode::new(&mut out_stream);
        test_unexpected_eof_encode(&mut encoder);
    }
    let mut decoder = XmlDecode::new(&spc_manager, &registry);
    test_unexpected_eof_decode(&mut decoder, &out_stream);
}

// Shared encode half for C++ `test_noremaining` / `test_openmismatch` /
// `test_closemismatch` (all three encode the identical document:
// <input><off/></input>)
fn test_input_off_encode<E: Encoder>(encoder: &mut E) {
    encoder.open_element(&ELEM_INPUT);
    encoder.open_element(&ELEM_OFF);
    encoder.close_element(&ELEM_OFF);
    encoder.close_element(&ELEM_INPUT);
}

// C++ `test_noremaining` (testmarshal.cc:434-453), decode half
fn test_noremaining_decode<D: Decoder>(decoder: &mut D, out_stream: &[u8]) {
    decoder.ingest_stream(out_stream).unwrap();
    decoder.open_element_id(&ELEM_INPUT).unwrap();
    let el2 = decoder.open_element_id(&ELEM_OFF).unwrap();
    decoder.close_element(el2).unwrap();
    // C++: try { el2 = decoder.openElement(ELEM_OFF); } catch(DecoderError&)
    let saw_no_remaining = matches!(
        decoder.open_element_id(&ELEM_OFF),
        Err(KunaError::Decoder { .. })
    );
    assert!(saw_no_remaining);
}

// C++ `test_openmismatch` (testmarshal.cc:455-472), decode half
fn test_openmismatch_decode<D: Decoder>(decoder: &mut D, out_stream: &[u8]) {
    decoder.ingest_stream(out_stream).unwrap();
    decoder.open_element_id(&ELEM_INPUT).unwrap();
    // C++: try { decoder.openElement(ELEM_OUTPUT); } catch(DecoderError&)
    let saw_open_mismatch = matches!(
        decoder.open_element_id(&ELEM_OUTPUT),
        Err(KunaError::Decoder { .. })
    );
    assert!(saw_open_mismatch);
}

// C++ `test_closemismatch` (testmarshal.cc:474-491), decode half
fn test_closemismatch_decode<D: Decoder>(decoder: &mut D, out_stream: &[u8]) {
    decoder.ingest_stream(out_stream).unwrap();
    let el1 = decoder.open_element_id(&ELEM_INPUT).unwrap();
    // C++: try { decoder.closeElement(el1); } catch(DecoderError&)
    let saw_close_mismatch = matches!(
        decoder.close_element(el1),
        Err(KunaError::Decoder { .. })
    );
    assert!(saw_close_mismatch);
}

#[test]
fn marshal_noremaining_packed() {
    let mut out_stream: Vec<u8> = Vec::new();
    let spc_manager = the_environ_build();
    {
        let mut encoder = PackedEncode::new(&mut out_stream);
        test_input_off_encode(&mut encoder);
    }
    let mut decoder = PackedDecode::new(&spc_manager);
    test_noremaining_decode(&mut decoder, &out_stream);
}

#[test]
fn marshal_noremaining_xml() {
    let mut out_stream: Vec<u8> = Vec::new();
    let spc_manager = the_environ_build();
    let registry = IdRegistry::with_base_ids();
    {
        let mut encoder = XmlEncode::new(&mut out_stream);
        test_input_off_encode(&mut encoder);
    }
    let mut decoder = XmlDecode::new(&spc_manager, &registry);
    test_noremaining_decode(&mut decoder, &out_stream);
}

#[test]
fn marshal_openmismatch_packed() {
    let mut out_stream: Vec<u8> = Vec::new();
    let spc_manager = the_environ_build();
    {
        let mut encoder = PackedEncode::new(&mut out_stream);
        test_input_off_encode(&mut encoder);
    }
    let mut decoder = PackedDecode::new(&spc_manager);
    test_openmismatch_decode(&mut decoder, &out_stream);
}

#[test]
fn marshal_openmismatch_xml() {
    let mut out_stream: Vec<u8> = Vec::new();
    let spc_manager = the_environ_build();
    let registry = IdRegistry::with_base_ids();
    {
        let mut encoder = XmlEncode::new(&mut out_stream);
        test_input_off_encode(&mut encoder);
    }
    let mut decoder = XmlDecode::new(&spc_manager, &registry);
    test_openmismatch_decode(&mut decoder, &out_stream);
}

#[test]
fn marshal_closemismatch_packed() {
    let mut out_stream: Vec<u8> = Vec::new();
    let spc_manager = the_environ_build();
    {
        let mut encoder = PackedEncode::new(&mut out_stream);
        test_input_off_encode(&mut encoder);
    }
    let mut decoder = PackedDecode::new(&spc_manager);
    test_closemismatch_decode(&mut decoder, &out_stream);
}

#[test]
fn marshal_bufferpad() {
    assert_eq!(PackedDecode::BUFFER_SIZE, 1024);
    let mut s: Vec<u8> = Vec::new();
    {
        let mut encoder = PackedEncode::new(&mut s);
        encoder.open_element(&ELEM_INPUT); // 1-byte
        for i in 0..511i32 {
            // 1022-bytes
            encoder.write_bool(&ATTRIB_CONTENT, (i & 1) == 0);
        }
        encoder.close_element(&ELEM_INPUT);
    }
    assert_eq!(s.len(), 1024); // Encoding should exactly fill one buffer
    // (C++ reads the static spcManager left over from the earlier tests'
    // theEnviron.build(); the Rust test builds it explicitly.)
    let spc_manager = the_environ_build();
    let mut decoder = PackedDecode::new(&spc_manager);
    decoder.ingest_stream(&s).unwrap();
    let el = decoder.open_element_id(&ELEM_INPUT).unwrap();
    for i in 0..511i32 {
        let attrib_id = decoder.get_next_attribute_id().unwrap();
        assert_eq!(attrib_id, ATTRIB_CONTENT.get_id());
        let val = decoder.read_bool().unwrap();
        assert_eq!(val, (i & 1) == 0);
    }
    let nextel = decoder.peek_element().unwrap();
    assert_eq!(nextel, 0);
    decoder.close_element(el).unwrap();
}
