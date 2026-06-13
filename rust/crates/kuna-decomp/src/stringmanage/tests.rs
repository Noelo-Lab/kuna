//! Tests for the `stringmanage` port: the unicode encode/decode + escape
//! matrices (the fidelity-critical static functions), the `<stringmanage>`
//! marshalling round-trip, internal-string registration, and the
//! loadimage-backed `getStringData` happy/sad paths.

use super::*;
use kuna_base::address::Address;
use kuna_base::marshal::{IdRegistry, XmlDecode, XmlEncode};
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};
use kuna_sleigh::loadimage::LoadImage;
use std::rc::Rc;

use crate::dtype::{type_metatype, Datatype};

// --- helpers ---------------------------------------------------------------

/// A character data-type of the given encoding size (1=UTF8, 2=UTF16, 4=UTF32).
fn chartype(size: int4) -> Datatype {
    Datatype::new(size, type_metatype::TYPE_INT)
}

/// `write_utf8` into a fresh Vec.
fn utf8(cp: int4) -> Vec<u8> {
    let mut v = Vec::new();
    StringManager::write_utf8(&mut v, cp);
    v
}

// --- write_utf8: the 1/2/3/4-byte encoder ----------------------------------

#[test]
fn write_utf8_one_byte_ascii() {
    // codepoint < 128 -> single byte, verbatim.
    assert_eq!(utf8(0), vec![0x00]);
    assert_eq!(utf8(0x41), vec![0x41]); // 'A'
    assert_eq!(utf8(0x7f), vec![0x7f]);
}

#[test]
fn write_utf8_two_bytes() {
    // U+00A2 CENT SIGN -> 0xc2 0xa2 (bits=8 < 12)
    assert_eq!(utf8(0xa2), vec![0xc2, 0xa2]);
    // U+0080 (first 2-byte) and U+07FF (last 2-byte, bits=11 < 12).
    assert_eq!(utf8(0x80), vec![0xc2, 0x80]);
    assert_eq!(utf8(0x7ff), vec![0xdf, 0xbf]);
}

#[test]
fn write_utf8_three_bytes() {
    // U+20AC EURO SIGN -> 0xe2 0x82 0xac (bits=14, 12<=bits<17)
    assert_eq!(utf8(0x20ac), vec![0xe2, 0x82, 0xac]);
    // U+0800 (first 3-byte, bits=12) and U+FFFF (last 3-byte, bits=16).
    assert_eq!(utf8(0x800), vec![0xe0, 0xa0, 0x80]);
    assert_eq!(utf8(0xffff), vec![0xef, 0xbf, 0xbf]);
}

#[test]
fn write_utf8_four_bytes() {
    // U+10000 (first 4-byte, bits=17) and U+10FFFF (max, bits=21).
    assert_eq!(utf8(0x10000), vec![0xf0, 0x90, 0x80, 0x80]);
    assert_eq!(utf8(0x10ffff), vec![0xf4, 0x8f, 0xbf, 0xbf]);
}

#[test]
#[should_panic(expected = "Negative unicode codepoint")]
fn write_utf8_negative_panics() {
    let mut v = Vec::new();
    StringManager::write_utf8(&mut v, -1);
}

#[test]
#[should_panic(expected = "Bad unicode codepoint")]
fn write_utf8_too_big_panics() {
    // bits > 21 (>= 2^21) throws "Bad unicode codepoint".
    let mut v = Vec::new();
    StringManager::write_utf8(&mut v, 0x200000);
}

// --- read_utf16 ------------------------------------------------------------

#[test]
fn read_utf16_endianness() {
    let buf = [0x12u8, 0x34];
    assert_eq!(StringManager::read_utf16(&buf, true), 0x1234); // big endian
    assert_eq!(StringManager::read_utf16(&buf, false), 0x3412); // little endian
}

// --- get_codepoint: UTF-8 decode -------------------------------------------

#[test]
fn get_codepoint_utf8_ascii() {
    let mut skip = 0;
    assert_eq!(StringManager::get_codepoint(&[0x41], 1, false, &mut skip), 0x41);
    assert_eq!(skip, 1);
}

#[test]
fn get_codepoint_utf8_multibyte() {
    let mut skip = 0;
    // 2-byte: U+00A2
    assert_eq!(StringManager::get_codepoint(&[0xc2, 0xa2], 1, false, &mut skip), 0xa2);
    assert_eq!(skip, 2);
    // 3-byte: U+20AC
    assert_eq!(
        StringManager::get_codepoint(&[0xe2, 0x82, 0xac], 1, false, &mut skip),
        0x20ac
    );
    assert_eq!(skip, 3);
    // 4-byte: U+10348
    assert_eq!(
        StringManager::get_codepoint(&[0xf0, 0x90, 0x8d, 0x88], 1, false, &mut skip),
        0x10348
    );
    assert_eq!(skip, 4);
}

#[test]
fn get_codepoint_utf8_invalid_continuation() {
    let mut skip = 0;
    // 2-byte lead with bad continuation (high bits not 10).
    assert_eq!(StringManager::get_codepoint(&[0xc2, 0x00], 1, false, &mut skip), -1);
    // 3-byte lead with one bad continuation.
    assert_eq!(
        StringManager::get_codepoint(&[0xe2, 0x82, 0x00], 1, false, &mut skip),
        -1
    );
    // Standalone continuation byte (0x80) is not a valid lead.
    assert_eq!(StringManager::get_codepoint(&[0x80], 1, false, &mut skip), -1);
}

// --- get_codepoint: UTF-16 decode incl surrogates --------------------------

#[test]
fn get_codepoint_utf16_bmp() {
    let mut skip = 0;
    // U+0041 'A' little-endian.
    assert_eq!(StringManager::get_codepoint(&[0x41, 0x00], 2, false, &mut skip), 0x41);
    assert_eq!(skip, 2);
}

#[test]
fn get_codepoint_utf16_surrogate_pair() {
    let mut skip = 0;
    // U+1D11E (musical G clef): high D834, low DD1E, little-endian bytes.
    let buf = [0x34, 0xd8, 0x1e, 0xdd];
    assert_eq!(StringManager::get_codepoint(&buf, 2, false, &mut skip), 0x1d11e);
    assert_eq!(skip, 4);
}

#[test]
fn get_codepoint_utf16_bad_surrogate() {
    let mut skip = 0;
    // High surrogate D800 followed by a non-trail (0x0041) -> invalid.
    let buf = [0x00, 0xd8, 0x41, 0x00];
    assert_eq!(StringManager::get_codepoint(&buf, 2, false, &mut skip), -1);
    // Trail surrogate DC00 appearing before a high surrogate -> invalid.
    let buf2 = [0x00, 0xdc];
    assert_eq!(StringManager::get_codepoint(&buf2, 2, false, &mut skip), -1);
}

// --- get_codepoint: UTF-32 decode ------------------------------------------

#[test]
fn get_codepoint_utf32() {
    let mut skip = 0;
    // U+1F600 little-endian.
    let le = [0x00, 0xf6, 0x01, 0x00];
    assert_eq!(StringManager::get_codepoint(&le, 4, false, &mut skip), 0x1f600);
    assert_eq!(skip, 4);
    // big-endian
    let be = [0x00, 0x01, 0xf6, 0x00];
    assert_eq!(StringManager::get_codepoint(&be, 4, true, &mut skip), 0x1f600);
    // Beyond max codepoint -> invalid.
    let bad = [0x00, 0x00, 0x11, 0x00]; // 0x110000
    assert_eq!(StringManager::get_codepoint(&bad, 4, false, &mut skip), -1);
}

#[test]
fn get_codepoint_reserved_surrogate_range_invalid() {
    let mut skip = 0;
    // A lone surrogate codepoint encoded as UTF-32 is rejected (0xD800..=0xDFFF).
    let buf = [0x00, 0xd8, 0x00, 0x00];
    assert_eq!(StringManager::get_codepoint(&buf, 4, false, &mut skip), -1);
}

#[test]
fn get_codepoint_bad_charsize() {
    let mut skip = 0;
    assert_eq!(StringManager::get_codepoint(&[0x41, 0, 0], 3, false, &mut skip), -1);
}

// --- check_characters / has_char_terminator --------------------------------

#[test]
fn check_characters_counts_until_terminator() {
    // "Hi\0" UTF-8.
    let buf = b"Hi\0";
    assert_eq!(StringManager::check_characters(buf, buf.len() as int4, 1, false), 2);
}

#[test]
fn check_characters_rejects_bad_encoding() {
    // A stray continuation byte makes the whole buffer illegal.
    let buf = [0x80u8, 0x00];
    assert_eq!(StringManager::check_characters(&buf, 2, 1, false), -1);
}

#[test]
fn check_characters_empty_buffer_is_invalid() {
    // C++ `buf == 0` -> -1; an empty slice models that here.
    assert_eq!(StringManager::check_characters(&[], 0, 1, false), -1);
}

#[test]
fn has_char_terminator_finds_full_width_zero() {
    // UTF-16: terminator is two zero bytes.
    let buf = [0x41u8, 0x00, 0x00, 0x00];
    assert!(StringManager::has_char_terminator(&buf, 4, 2));
    // No terminator: a non-zero byte in every character cell.
    let buf2 = [0x41u8, 0x00, 0x42, 0x00];
    assert!(!StringManager::has_char_terminator(&buf2, 4, 2));
}

// --- encode / decode round-trip --------------------------------------------

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

fn registry() -> IdRegistry {
    let mut reg = IdRegistry::with_base_ids();
    reg.register_attribute(&ATTRIB_TRUNC);
    for e in [&ELEM_BYTES, &ELEM_STRING, &ELEM_STRINGMANAGE] {
        reg.register_element(e);
    }
    reg
}

#[test]
fn encode_decode_roundtrip() {
    let mgr = build_manager();
    let ram = mgr.get_space_by_name("ram").unwrap().clone();

    let mut sm = StringManager::new(2048);
    // Two strings at distinct addresses; one truncated, one not.  The byte data
    // includes a >= 20-byte run to exercise the `\n  ` content break.
    let a1 = Address::new(ram.clone(), 0x1000);
    sm.string_map.insert(
        a1.clone(),
        StringData {
            is_truncated: false,
            byte_data: b"hello world\0".to_vec(),
        },
    );
    let a2 = Address::new(ram.clone(), 0x2000);
    sm.string_map.insert(
        a2.clone(),
        StringData {
            is_truncated: true,
            byte_data: (0u8..25).collect(),
        },
    );

    let mut buf: Vec<u8> = Vec::new();
    {
        let mut enc = XmlEncode::new(&mut buf);
        sm.encode(&mut enc).unwrap();
    }

    let reg = registry();
    let mut dec = XmlDecode::new(&mgr, &reg);
    dec.ingest_stream(&buf).unwrap();
    let mut out = StringManager::new(2048);
    out.decode(&mut dec).unwrap();

    assert_eq!(out.string_map.len(), 2);
    let d1 = out.string_map.get(&a1).unwrap();
    assert!(!d1.is_truncated);
    assert_eq!(d1.byte_data, b"hello world\0");
    let d2 = out.string_map.get(&a2).unwrap();
    assert!(d2.is_truncated);
    assert_eq!(d2.byte_data, (0u8..25).collect::<Vec<u8>>());
}

// --- register_internal_string_data -----------------------------------------

#[test]
fn register_internal_legal_string() {
    let mgr = build_manager();
    let ram = mgr.get_space_by_name("ram").unwrap().clone();
    let addr = Address::new(ram, 0x4010);
    let mut sm = StringManager::new(2048);

    let ct = chartype(1);
    let bytes = b"abc\0";
    let hash = sm.register_internal_string_data(&addr, bytes, bytes.len() as int4, &ct, &mgr);
    assert_ne!(hash, 0);

    // The string is retrievable via the constant address built from the hash.
    let const_addr = mgr.get_constant(hash);
    // charsize==1 && numChars(3) < maximumChars copies the raw `size` bytes
    // verbatim, INCLUDING the terminator (C++ `byteData.assign(buf, buf+size)`).
    assert_eq!(sm.get_string_data_cached(&const_addr), b"abc\0");
    assert!(sm.is_string_cached(&const_addr));
}

#[test]
fn register_internal_illegal_returns_zero() {
    let mgr = build_manager();
    let ram = mgr.get_space_by_name("ram").unwrap().clone();
    let addr = Address::new(ram, 0x4020);
    let mut sm = StringManager::new(2048);

    let ct = chartype(1);
    // A lone continuation byte is an illegal UTF-8 encoding -> checkCharacters<0.
    let bytes = [0x80u8, 0x00];
    let hash = sm.register_internal_string_data(&addr, &bytes, 2, &ct, &mgr);
    assert_eq!(hash, 0);
}

// --- StringManagerUnicode::get_string_data ---------------------------------

/// A loadimage modelling one mapped section `[base, base+mapped_len)`: addresses
/// inside the section that are past `bytes` read as zero (terminator), the way a
/// real loader zero-fills a `.bss`/`.data` tail; addresses outside the section
/// raise DataUnavail (C++ `DataUnavailError`).
struct MockLoad {
    base: u64,
    bytes: Vec<u8>,
    mapped_len: u64,
}

impl LoadImage for MockLoad {
    fn get_file_name(&self) -> &str {
        "mock"
    }
    fn load_fill(&mut self, ptr: &mut [u8], addr: &Address) -> KunaResult<()> {
        let off = addr.get_offset();
        for (i, slot) in ptr.iter_mut().enumerate() {
            let idx = off.wrapping_add(i as u64).wrapping_sub(self.base);
            if idx >= self.mapped_len {
                return Err(KunaError::data_unavail("out of range"));
            }
            *slot = self.bytes.get(idx as usize).copied().unwrap_or(0);
        }
        Ok(())
    }
    fn get_arch_type(&self) -> Vec<u8> {
        Vec::new()
    }
    fn adjust_vma(&mut self, _adjust: i64) {}
}

#[test]
fn get_string_data_terminated_ascii() {
    let mgr = build_manager();
    let ram = mgr.get_space_by_name("ram").unwrap().clone();
    let addr = Address::new(ram, 0x5000);
    let mut sm = StringManagerUnicode::new(2048);
    let ct = chartype(1);
    // 40 bytes of ascii then a terminator, so the 32-at-a-time fetch loops once.
    let mut data: Vec<u8> = b"the quick brown fox jumps over a lazydog".to_vec();
    data.push(0);
    let mut loader = MockLoad {
        base: 0x5000,
        bytes: data.clone(),
        mapped_len: 256,
    };

    let mut is_trunc = true;
    let out = sm
        .get_string_data(&addr, &ct, &mut loader, &mut is_trunc)
        .to_vec();
    assert!(!is_trunc);
    // charsize==1 path copies the raw fetched bytes verbatim, up to and including
    // the terminator the 32-at-a-time loop stops on (here the full 41 bytes of
    // the 64-byte second fetch are copied, so the string + terminator survive).
    assert_eq!(&out[..40], &data[..40]);
    assert_eq!(out[40], 0); // terminator copied through
}

#[test]
fn get_string_data_caches_second_lookup() {
    let mgr = build_manager();
    let ram = mgr.get_space_by_name("ram").unwrap().clone();
    let addr = Address::new(ram, 0x6000);
    let mut sm = StringManagerUnicode::new(2048);
    let ct = chartype(1);
    let loader_bytes = b"hi\0".to_vec();
    let mut loader = MockLoad {
        base: 0x6000,
        bytes: loader_bytes,
        mapped_len: 64,
    };

    let mut is_trunc = false;
    let first = sm
        .get_string_data(&addr, &ct, &mut loader, &mut is_trunc)
        .to_vec();
    // The first 32-byte fetch finds the terminator at byte 2; the charsize==1
    // path copies all `curBufferSize` (32) fetched bytes, so the string + its
    // terminator lead a zero-filled tail.
    assert_eq!(&first[..3], b"hi\0");
    // Second lookup hits the cache; the result is identical even with a loader
    // that would now fail (verifies the cache short-circuit before load_fill).
    let mut empty_loader = MockLoad {
        base: 0x9999,
        bytes: Vec::new(),
        mapped_len: 0,
    };
    let second = sm
        .get_string_data(&addr, &ct, &mut empty_loader, &mut is_trunc)
        .to_vec();
    assert_eq!(second, first);
}

#[test]
fn get_string_data_unavailable_returns_empty() {
    let mgr = build_manager();
    let ram = mgr.get_space_by_name("ram").unwrap().clone();
    let addr = Address::new(ram, 0x7000);
    let mut sm = StringManagerUnicode::new(2048);
    let ct = chartype(1);
    // A short mapped section of non-zero bytes (no terminator); the first
    // 32-byte fetch runs past the 8-byte section and load_fill throws
    // DataUnavail, so the result is the empty buffer.
    let mut loader = MockLoad {
        base: 0x7000,
        bytes: vec![0x41u8; 8],
        mapped_len: 8,
    };

    let mut is_trunc = false;
    let out = sm
        .get_string_data(&addr, &ct, &mut loader, &mut is_trunc)
        .to_vec();
    assert!(out.is_empty());
    assert!(!is_trunc);
}

#[test]
fn get_string_data_opaque_string_returns_empty() {
    let mgr = build_manager();
    let ram = mgr.get_space_by_name("ram").unwrap().clone();
    let addr = Address::new(ram, 0x8000);
    let mut sm = StringManagerUnicode::new(2048);
    // An opaque-string char type short-circuits before any load_fill.
    let mut ct = chartype(1);
    ct.flags |= crate::dtype::flags::opaque_string;
    let mut loader = MockLoad {
        base: 0,
        bytes: Vec::new(),
        mapped_len: 0,
    };
    let mut is_trunc = true;
    let out = sm
        .get_string_data(&addr, &ct, &mut loader, &mut is_trunc)
        .to_vec();
    assert!(out.is_empty());
    assert!(!is_trunc);
}
