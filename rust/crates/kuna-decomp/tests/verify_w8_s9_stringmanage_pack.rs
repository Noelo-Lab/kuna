//! Adversarial verification tests for `w8-s9-stringmanage-pack`
//! (port of `decompiler/cpp/stringmanage.{cc,hh}`, `kuna_naming.{cc,hh}`,
//! `kuna_arraynotation.{cc,hh}`).
//!
//! Written by the independent verifier (verification.md adversarial pass).  The
//! hunt targeted: the `getStringData` do-while no-terminator path
//! (`amount==0` -> C++ `return` vs Rust fall-through), the `<stringmanage>`
//! hex-content marshalling layout (20-byte line break), the UTF-16 surrogate
//! boundary in `getCodepoint`, and `registerInternalStringData` legality gating.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::KunaResult;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};
use kuna_base::marshal::{Decoder, IdRegistry, XmlDecode, XmlEncode};
use kuna_base::types::int4;
use kuna_decomp::dtype::{type_metatype, Datatype};
use kuna_decomp::stringmanage::{
    StringData, StringManager, StringManagerUnicode, ATTRIB_TRUNC, ELEM_BYTES, ELEM_STRING,
    ELEM_STRINGMANAGE,
};
use kuna_sleigh::loadimage::LoadImage;

// --- shared scaffolding (mirrors src/stringmanage/tests.rs) -----------------

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

fn chartype(size: int4) -> Datatype {
    Datatype::new(size, type_metatype::TYPE_INT)
}

/// A loadimage mapping `[base, base+mapped_len)`; bytes past `bytes` read as 0
/// (terminator), out-of-range raises DataUnavail.  Same contract as the inline
/// test `MockLoad`.
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
                return Err(kuna_base::error::KunaError::data_unavail("out of range"));
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

// ===========================================================================
// F1 (MAJOR): getStringData no-terminator path.
//
// C++ getStringData (stringmanage.cc:449-464):
//
//   do {
//     ...
//     if (newBufferSize > maximumChars) {
//       newBufferSize = maximumChars;
//       amount = newBufferSize - curBufferSize;
//       if (amount == 0) {
//         return stringData.byteData;   // Could not find terminator -> EMPTY
//       }
//     }
//     ...
//   } while (!foundTerminator);
//   ... checkCharacters / assignStringData ...
//
// When the mapped image holds `maximumChars` bytes of *valid* encoding but NO
// null terminator, the C++ hits `amount == 0` and RETURNS THE EMPTY BUFFER
// before ever calling checkCharacters/assignStringData -- i.e. it reports "not
// a string here".
//
// The Rust port turns that `return` into `break 'fill Ok(())`, which falls
// through to check_characters + assign_string_data and emits a (truncated)
// string.  That is a literal-emission divergence: C++ emits nothing, Rust emits
// a string.  This test pins the C++ behavior (empty) and therefore FAILS
// against the current port.
// ===========================================================================
#[test]
fn f1_no_terminator_within_max_returns_empty_like_cpp() {
    let mgr = build_manager();
    let ram = mgr.get_space_by_name("ram").unwrap().clone();
    let addr = Address::new(ram, 0x5000);

    // maximumChars == 32 (one full 32-byte fetch).  The section is exactly 32
    // bytes of printable ASCII with NO zero byte anywhere, and the section is
    // exactly `mapped_len == 32` so reads past it would DataUnavail -- but the
    // amount==0 clamp fires first on the second iteration.
    let mut sm = StringManagerUnicode::new(32);
    let ct = chartype(1);
    let bytes = vec![b'A'; 32]; // 32 non-zero ASCII bytes, no terminator
    let mut loader = MockLoad {
        base: 0x5000,
        bytes,
        mapped_len: 32,
    };

    let mut is_trunc = false;
    let out = sm
        .get_string_data(&addr, &ct, &mut loader, &mut is_trunc)
        .to_vec();

    // C++ returns the empty buffer (no terminator found within maximumChars).
    assert!(
        out.is_empty(),
        "C++ getStringData returns the EMPTY buffer when no terminator is found \
         within maximumChars (the `amount==0` early `return`); the Rust port \
         falls through to assignStringData and emits {:?}",
        out
    );
    assert!(!is_trunc);
}

// ===========================================================================
// F2 (control): a terminated string at exactly maximumChars-1 still decodes.
// Confirms the no-terminator finding above is specifically about the missing
// terminator, not a general off-by-one in the loop bound.
// ===========================================================================
#[test]
fn f2_terminated_just_under_max_decodes() {
    let mgr = build_manager();
    let ram = mgr.get_space_by_name("ram").unwrap().clone();
    let addr = Address::new(ram, 0x5100);

    // 31 ASCII bytes then a terminator at index 31 -> the single 32-byte fetch
    // (maximumChars==32) finds the terminator; charsize==1 copies all 32 bytes.
    let mut sm = StringManagerUnicode::new(32);
    let ct = chartype(1);
    let mut bytes = vec![b'B'; 31];
    bytes.push(0);
    let mut loader = MockLoad {
        base: 0x5100,
        bytes,
        mapped_len: 64,
    };

    let mut is_trunc = true;
    let out = sm
        .get_string_data(&addr, &ct, &mut loader, &mut is_trunc)
        .to_vec();
    assert_eq!(&out[..31], &[b'B'; 31][..]);
    assert_eq!(out[31], 0);
    // numChars (31) < maximumChars (32) -> not truncated.
    assert!(!is_trunc);
}

// ===========================================================================
// F3: registerInternalStringData legality gate + constant-address keying.
//
// checkCharacters(buf) < 0 (invalid encoding) must return 0 and register
// nothing; a legal buffer returns a non-zero hash and caches under the constant
// address built from that hash.
// ===========================================================================
#[test]
fn f3_register_internal_legality_and_constant_keying() {
    let mgr = build_manager();
    let ram = mgr.get_space_by_name("ram").unwrap().clone();
    let addr = Address::new(ram, 0x9000);
    let ct = chartype(1);

    let mut sm = StringManager::new(2048);

    // A lone 0xFF byte is an invalid UTF-8 lead -> checkCharacters == -1 -> 0.
    let bad = [0xFFu8];
    let h_bad = sm.register_internal_string_data(&addr, &bad, 1, &ct, &mgr);
    assert_eq!(h_bad, 0, "invalid encoding must return hash 0 and register nothing");
    assert!(sm.string_map.is_empty());

    // A legal NUL-terminated ASCII string -> non-zero hash, cached at the
    // constant address derived from the hash.
    let good = b"hi\0";
    let h = sm.register_internal_string_data(&addr, good, good.len() as int4, &ct, &mgr);
    assert_ne!(h, 0);
    let const_addr = mgr.get_constant(h);
    // charsize==1 && numChars(2) < maximumChars -> raw bytes copied verbatim.
    assert_eq!(sm.get_string_data_cached(&const_addr), b"hi\0");
}

// ===========================================================================
// F4: <stringmanage> hex-content layout -- the 20-byte line break.
//
// encode writes (stringmanage.cc:215-222): a leading '\n', two lowercase hex
// digits per byte, a "\n  " break after every 20 bytes (i % 20 == 19), and a
// trailing '\n'.  decode must round-trip the exact bytes back.  This pins the
// boundary the % 20 break sits on (21 bytes -> exactly one mid-string break).
// ===========================================================================
fn registry() -> IdRegistry {
    let mut reg = IdRegistry::with_base_ids();
    reg.register_attribute(&ATTRIB_TRUNC);
    for e in [&ELEM_BYTES, &ELEM_STRING, &ELEM_STRINGMANAGE] {
        reg.register_element(e);
    }
    reg
}

#[test]
fn f4_stringmanage_roundtrip_and_20byte_break() {
    let mgr = build_manager();
    let ram = mgr.get_space_by_name("ram").unwrap().clone();
    let addr = Address::new(ram.clone(), 0x4000);

    let mut sm = StringManager::new(2048);
    // 21 bytes 0x00..0x14 so the 20th byte (index 19) forces a "\n  " break and
    // byte 21 (index 20) lands after it.  Includes the 0x00..0x0f low range so
    // the two-digit zero-fill (setw(2)) is exercised.
    let payload: Vec<u8> = (0u8..21).collect();
    sm.string_map.insert(
        addr.clone(),
        StringData {
            is_truncated: true,
            byte_data: payload.clone(),
        },
    );

    let mut buf: Vec<u8> = Vec::new();
    {
        let mut enc = XmlEncode::new(&mut buf);
        sm.encode(&mut enc).unwrap();
    }

    // The hex content must contain a line break exactly after 20 hex pairs
    // (40 hex chars) and the 21st pair ("14") on the continuation line.
    let text = String::from_utf8(buf.clone()).unwrap();
    assert!(
        text.contains("\n  14"),
        "byte index 20 (0x14) must follow the 20-byte `\\n  ` break; got:\n{text}"
    );

    // Round-trip: decode must reconstruct the identical byte_data.
    let reg = registry();
    let mut dec = XmlDecode::new(&mgr, &reg);
    dec.ingest_stream(&buf).unwrap();
    let mut sm2 = StringManager::new(2048);
    sm2.decode(&mut dec).unwrap();
    assert_eq!(sm2.string_map.len(), 1);
    let restored = sm2.string_map.get(&addr).unwrap();
    assert_eq!(restored.byte_data, payload);
    assert!(restored.is_truncated);
}

// ===========================================================================
// F5: getCodepoint UTF-16 surrogate-pair boundary.
//
// A valid high+low surrogate pair must decode to the combined codepoint and
// consume 4 bytes; a high surrogate followed by a non-trail must return -1; a
// lone trail surrogate must return -1.  These are the exact branches that gate
// whether a UTF-16 literal is emitted (stringmanage.cc:355-361).
// ===========================================================================
#[test]
fn f5_utf16_surrogate_pair_boundary() {
    // U+1F600 = 0x1F600.  high surrogate 0xD83D, low (trail) surrogate 0xDE00.
    // Little-endian byte order: high = [0x3D,0xD8], trail = [0x00,0xDE].
    let mut skip = 0;
    let le = [0x3D, 0xD8, 0x00, 0xDE];
    let cp = StringManager::get_codepoint(&le, 2, false, &mut skip);
    assert_eq!(cp, 0x1F600, "valid surrogate pair must combine to U+1F600");
    assert_eq!(skip, 4, "a surrogate pair consumes 4 bytes");

    // High surrogate (0xD83D LE) followed by a NON-trail (0x0041 'A') -> -1.
    let mut skip2 = 0;
    let bad_trail = [0x3D, 0xD8, 0x41, 0x00];
    assert_eq!(
        StringManager::get_codepoint(&bad_trail, 2, false, &mut skip2),
        -1,
        "high surrogate not followed by a trail surrogate is invalid"
    );

    // Lone trail surrogate (0xDC00 LE) with no preceding high -> -1.
    let mut skip3 = 0;
    let lone_trail = [0x00, 0xDC, 0x00, 0x00];
    assert_eq!(
        StringManager::get_codepoint(&lone_trail, 2, false, &mut skip3),
        -1,
        "trail surrogate before a high surrogate is invalid"
    );
}
