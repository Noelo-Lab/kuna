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

// ===========================================================================
// ROUND 2 adversarial tests.  Round 1 left three fragile spots uncovered:
//   * the assignStringData truncation re-encode path (numChars >= maximumChars),
//     which sets isTruncated=true and goes through writeUnicode (not the raw
//     charsize==1 fast-copy);  the round-1 `getStringData` tests only ever hit
//     numChars < maximumChars (the terminator-within-max requirement makes the
//     boundary unreachable through getStringData, so it must be driven through
//     registerInternalStringData),
//   * the UTF-32 codepoint-range gate + the i32 add in the LE/BE assembly
//     (a wrap-prone spot: each `(buf[k] as int4) << shift` can be negative and
//     the C++ relies on the byte-disjoint sum never carrying),
//   * the F1 fix at its exact boundary plus a control where the terminator lands
//     on the very last byte of maximumChars (proves the fix did not over-clamp).
// ===========================================================================

// ---------------------------------------------------------------------------
// G1: assignStringData truncation re-encode path via registerInternalStringData.
//
// C++ assignStringData (stringmanage.cc:69-85):
//   if (charsize == 1 && numChars < maximumChars) { raw copy }
//   else { writeUnicode -> truncate to <= maximumChars chars; push NUL; }
//   data.isTruncated = (numChars >= maximumChars);
//
// With maximumChars = 3 and a 5-char ASCII payload "ABCDE\0", checkCharacters
// returns 5 (>= 3), so the ELSE branch runs: writeUnicode stops after `count >=
// maximumChars` (3 chars -> "ABC"), a NUL is appended, and isTruncated is true.
// This is the exact truncation behavior that decides how many characters of a
// string literal Ghidra prints.
// ---------------------------------------------------------------------------
#[test]
fn g1_truncation_reencode_path_caps_at_maximum_chars() {
    let mgr = build_manager();
    let ram = mgr.get_space_by_name("ram").unwrap().clone();
    let addr = Address::new(ram, 0xA000);
    let ct = chartype(1);

    let mut sm = StringManager::new(3); // maximumChars = 3

    let payload = b"ABCDE\0"; // 5 chars + terminator, size 6
    let h = sm.register_internal_string_data(&addr, payload, payload.len() as int4, &ct, &mgr);
    assert_ne!(h, 0, "a legal (truncatable) string must register");
    let const_addr = mgr.get_constant(h);

    // ELSE branch: writeUnicode emits the first 3 chars then breaks on
    // `count >= maximumChars`, and a NUL terminator is appended.
    let cached = sm.get_string_data_cached(&const_addr);
    assert_eq!(
        cached, b"ABC\0",
        "truncation re-encode must cap at maximumChars (3) chars + NUL, got {cached:?}"
    );
    // isTruncated = (numChars(5) >= maximumChars(3)) = true.
    let sd = sm.string_map.get(&const_addr).unwrap();
    assert!(
        sd.is_truncated,
        "numChars >= maximumChars must mark the StringData truncated"
    );
}

// ---------------------------------------------------------------------------
// G2: getCodepoint UTF-32 range gate + the byte-disjoint i32 assembly.
//
// stringmanage.cc:393-399 assembles a UTF-32 element as
//   (buf[3]<<24)+(buf[2]<<16)+(buf[1]<<8)+buf[0]   (little endian)
// where each `buf[k]` is a uint1 promoted to int4.  When buf[3] >= 0x80 the
// (buf[3]<<24) term is a NEGATIVE int4 and the whole sum reinterprets to a
// negative int4 -- which the `codepoint < 0` caller-side gate then rejects.
// stringmanage.cc:402-406 also rejects anything in [0xd800,0xdfff] (surrogates)
// and anything > 0x10ffff.  The Rust add is plain `i32 + i32`, which panics on
// debug overflow; this verifies the byte-disjoint sum never overflows and that
// every range branch matches.
// ---------------------------------------------------------------------------
#[test]
fn g2_utf32_range_gate_and_no_add_overflow() {
    // A valid 3-byte codepoint U+20AC (euro) packed as UTF-32 LE: 0xAC,0x20,0,0.
    let mut sk = 0;
    let euro = [0xACu8, 0x20, 0x00, 0x00];
    assert_eq!(StringManager::get_codepoint(&euro, 4, false, &mut sk), 0x20AC);
    assert_eq!(sk, 4, "UTF-32 always consumes 4 bytes");

    // Big-endian assembly of the same codepoint: 0,0,0x20,0xAC.
    let mut skb = 0;
    let euro_be = [0x00u8, 0x00, 0x20, 0xAC];
    assert_eq!(StringManager::get_codepoint(&euro_be, 4, true, &mut skb), 0x20AC);

    // Just past the maximum codepoint (0x110000 > 0x10ffff) -> -1.
    let mut sk2 = 0;
    let too_big = [0x00u8, 0x00, 0x11, 0x00]; // LE 0x00110000
    assert_eq!(
        StringManager::get_codepoint(&too_big, 4, false, &mut sk2),
        -1,
        "codepoint > 0x10ffff must be rejected"
    );

    // A surrogate-range scalar 0xD800 encoded as UTF-32 -> -1 (reserved).
    let mut sk3 = 0;
    let surrogate = [0x00u8, 0xD8, 0x00, 0x00]; // LE 0x0000D800
    assert_eq!(
        StringManager::get_codepoint(&surrogate, 4, false, &mut sk3),
        -1,
        "a UTF-32 value in [0xd800,0xdfff] must be rejected as a reserved surrogate"
    );

    // The high-bit-set assembly must reinterpret to a NEGATIVE int4 and must NOT
    // panic on the i32 additions.  Note: getCodepoint does NOT normalize this to
    // -1 -- the `codepoint >= 0xd800` range gate is skipped for a negative value,
    // so the raw negative codepoint is returned and the CALLER (checkCharacters /
    // writeUnicode) rejects it via `codepoint < 0`.  This mirrors C++ exactly
    // (the value is `(buf[3]<<24)+...` reinterpreted, e.g. 0xFF030201 -> negative).
    let mut sk4 = 0;
    let high = [0x01u8, 0x02, 0x03, 0xFF]; // LE assembles to 0xFF030201 = negative
    let cp = StringManager::get_codepoint(&high, 4, false, &mut sk4);
    assert!(
        cp < 0,
        "a UTF-32 element with the top bit set reinterprets to a negative codepoint \
         (caught by the caller's `codepoint < 0`); got {cp}"
    );
    // It must be the exact C++ bit-pattern reinterpretation, not -1.
    assert_eq!(cp, 0xFF030201u32 as i32, "must be the byte-disjoint sum, reinterpreted");

    // The worst-case all-ones input must also not panic and must reinterpret to
    // the exact bit pattern (-1), proving the byte-disjoint sum never carries.
    let mut sk5 = 0;
    let all_ones = [0xFFu8, 0xFF, 0xFF, 0xFF]; // assembles to 0xFFFFFFFF = -1
    assert_eq!(StringManager::get_codepoint(&all_ones, 4, false, &mut sk5), -1);

    // And confirm the negative codepoint really does gate string emission at the
    // caller: checkCharacters over a UTF-32 buffer whose first element is negative
    // must return -1 (invalid encoding) rather than counting a character.
    assert_eq!(
        StringManager::check_characters(&high, 4, 4, false),
        -1,
        "a negative UTF-32 codepoint must make checkCharacters report an invalid encoding"
    );
}

// ---------------------------------------------------------------------------
// G3: the F1 fix at the boundary + control that the fix did not over-clamp.
//
// (a) No terminator within maximumChars but the buffer is NOT a full multiple of
//     32 (maximumChars = 20): the first fetch clamps newBufferSize to 20,
//     amount = 20 (non-zero), no terminator; the second iteration clamps again,
//     amount = 0 -> C++ returns the EMPTY buffer.  This is the repaired path.
// (b) Control: the very same 20-byte buffer but with a terminator on the LAST
//     byte (index 19).  hasCharTerminator finds it inside the first fetch, so a
//     string IS emitted -- proving the `amount==0` early-return did not swallow a
//     legitimately terminated string sitting exactly at the maximumChars edge.
// ---------------------------------------------------------------------------
#[test]
fn g3_no_terminator_returns_empty_but_edge_terminator_decodes() {
    let mgr = build_manager();
    let ram = mgr.get_space_by_name("ram").unwrap().clone();
    let ct = chartype(1);

    // (a) 20 non-zero bytes, no terminator, maximumChars = 20 (not a /32 size).
    let addr_a = Address::new(ram.clone(), 0xB000);
    let mut sm_a = StringManagerUnicode::new(20);
    let mut loader_a = MockLoad {
        base: 0xB000,
        bytes: vec![b'Z'; 20],
        mapped_len: 64, // plenty mapped; the amount==0 clamp fires first
    };
    let mut tr_a = false;
    let out_a = sm_a
        .get_string_data(&addr_a, &ct, &mut loader_a, &mut tr_a)
        .to_vec();
    assert!(
        out_a.is_empty(),
        "no terminator within maximumChars must yield the EMPTY buffer (F1 fix), got {out_a:?}"
    );
    assert!(!tr_a);

    // (b) Same 20-byte window but a terminator at the last byte (index 19).
    let addr_b = Address::new(ram, 0xC000);
    let mut sm_b = StringManagerUnicode::new(20);
    let mut bytes_b = vec![b'Z'; 19];
    bytes_b.push(0); // terminator at index 19, inside the first fetch
    let mut loader_b = MockLoad {
        base: 0xC000,
        bytes: bytes_b,
        mapped_len: 64,
    };
    let mut tr_b = false;
    let out_b = sm_b
        .get_string_data(&addr_b, &ct, &mut loader_b, &mut tr_b)
        .to_vec();
    // numChars(19) < maximumChars(20) -> raw charsize==1 copy of all 20 bytes.
    assert_eq!(&out_b[..19], &[b'Z'; 19][..], "edge-terminated string must decode");
    assert_eq!(out_b[19], 0, "the terminator byte is preserved by the raw copy");
    assert!(
        !tr_b,
        "a terminator inside maximumChars is not a truncation"
    );
}
