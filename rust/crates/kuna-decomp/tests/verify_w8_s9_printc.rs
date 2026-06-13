//! Adversarial verifier tests for `w8-s9-printc`.
//!
//! Protocol: docs/rust-port/verification.md.  These target the hunt-list spots
//! the verifier flagged as most fragile for the ported half of `PrintC` (the
//! data tables + the self-contained constant/char/float formatters), with the
//! C++ oracle re-derived from `decompiler/cpp/printc.cc` and
//! `decompiler/cpp/stringmanage.cc`:
//!
//!   - **`print_char_hex_escape` signedness / bit-pattern** (printc.cc:1580-1591).
//!     C++ prints `int4 val` through `<< hex`, which for a NEGATIVE int4 emits
//!     the 32-bit two's-complement (`ffffffff`-style), NOT a `-` sign.  Rust's
//!     `{:x}` on `i32` must reproduce the same bit pattern (it does — `LowerHex`
//!     formats the raw bits).  This is reachable: `print_unicode` routes a
//!     negative codepoint (escaped by `unicode_needs_escape`, which returns true
//!     for `< 0x20`) through the generic-escape branch to `print_char_hex_escape`.
//!     A `-` sign here would be silent output corruption.
//!   - **`resolve_integer_format` two's-complement magnitude wrap** (printc.cc:
//!     1384-1391).  `val = flip+1` is unsigned arithmetic; at the signed minimum
//!     (`0x80…00`) and at the all-ones encoding it must wrap exactly like C++.
//!   - **`print_unicode` raw-emit branch UTF-8 byte parity** (printc.cc:1537 ->
//!     `StringManager::writeUtf8`, stringmanage.cc:124-158).  For every codepoint
//!     that actually reaches the raw branch (i.e. `!unicode_needs_escape`), the
//!     bytes must equal C++ `writeUtf8`'s 2/3-byte UTF-8 encoding.  `char::from_u32`
//!     must never return `None` for a reachable codepoint (surrogates and the
//!     `>= 0x2fa20` range are pre-escaped).
//!   - **`format_float_token` `.0` fix-up boundary** (printc.cc:1477-1487).  The
//!     "looks like a float" test keys on the bytes `.`/`e`; values like `"100"`
//!     (no dot, no e) must get `.0`, but `"1e5"`/`"0.0"` must not, and the
//!     `force_scinote` path must bypass the fix-up entirely.
//!   - **`format_integer_token` FORCE_CHAR high-byte / wide split** (printc.cc:
//!     1417-1424): `sz==1 && val>=0x80` -> hex escape; else `print_unicode`.

use kuna_base::types::{int4, uintb};
use kuna_decomp::printc::{
    display_format, format_float_token, format_integer_token, print_char_hex_escape,
    print_unicode, resolve_integer_format, FloatClass,
};
use kuna_decomp::printlanguage::unicode_needs_escape;

/// Oracle: C++ `printCharHexEscape(ostream&, int4 val)` (printc.cc:1580-1591)
/// — `s << "\x" << setfill('0') << setw(W) << hex << val`, where the width W is
/// 2 if `val < 256`, 4 if `val < 65536`, else 8; `setw` is a MINIMUM, and
/// `<< hex` on a signed value prints its raw 32-bit two's-complement.
fn oracle_char_hex_escape(val: i32) -> String {
    // Width selection is on the SIGNED comparison `val < 256` etc. (C++ int4).
    let width = if val < 256 {
        2
    } else if val < 65536 {
        4
    } else {
        8
    };
    // `<< hex` reinterprets the bits as the unsigned 32-bit value.
    let bits = val as u32;
    format!("\\x{bits:0width$x}", width = width)
}

#[test]
fn printc_w8_char_hex_escape_negative_matches_cpp_bit_pattern() {
    // A negative codepoint reaches print_char_hex_escape via print_unicode's
    // generic-escape branch (unicode_needs_escape(<0) == true, switch misses).
    // C++ `<< hex << (int4)-1` -> "ffffffff" (8 digits, since setw(2) is a min
    // and the full value is 8 hex digits).  A `-` sign would be corruption.
    for &v in &[-1i32, -2, -128, -256, i32::MIN, -0x1234] {
        let mut got = String::new();
        print_char_hex_escape(&mut got, v as int4);
        let want = oracle_char_hex_escape(v);
        assert_eq!(got, want, "char_hex_escape mismatch for {v:#x}");
        assert!(!got.contains('-'), "negative codepoint must not print a '-': {got:?}");
        assert!(
            got.starts_with("\\x") && got.len() == 2 + 8,
            "negative int4 escapes to 8 hex digits: {got:?}"
        );
    }

    // Positive width boundaries: setw(2)/setw(4)/setw(8) zero-fill minimums.
    let cases = [(0x0, "\\x00"), (0x7, "\\x07"), (0xff, "\\xff"), (0x100, "\\x0100"),
                 (0xffff, "\\xffff"), (0x1_0000, "\\x00010000"), (0xdead_beefu32 as i32, "\\xdeadbeef")];
    for (v, want) in cases {
        let mut got = String::new();
        print_char_hex_escape(&mut got, v as int4);
        assert_eq!(got, want, "boundary width mismatch for {v:#x}");
        assert_eq!(got, oracle_char_hex_escape(v), "oracle disagreement for {v:#x}");
    }
}

/// Oracle: C++ `push_integer` sign block (printc.cc:1384-1391).
/// `mask = calc_mask(sz); flip = val^mask; print_negsign = flip < val;
///  if (print_negsign) val = flip+1;` — unsigned, wraps.
fn oracle_resolve_sign(val: uintb, sz: int4) -> (bool, uintb) {
    let mask: uintb = if sz >= 8 { u64::MAX } else { (1u64 << (sz * 8)) - 1 };
    let flip = val ^ mask;
    let neg = flip < val;
    if neg {
        (true, flip.wrapping_add(1))
    } else {
        (false, val)
    }
}

#[test]
fn printc_w8_resolve_integer_twos_complement_wrap_boundaries() {
    // Signed minima for each size: 0x80, 0x8000, 0x8000_0000, 0x8000…00.
    // At INT_MIN, flip = 0x7f…ff, flip+1 = 0x80…00 == val (magnitude == val), no wrap.
    let cases: &[(uintb, int4)] = &[
        (0x80, 1),                  // int8 min
        (0x8000, 2),                // int16 min
        (0x8000_0000, 4),           // int32 min
        (0x8000_0000_0000_0000, 8), // int64 min
        (0xff, 1),                  // -1 in 1 byte -> magnitude 1
        (0xffff_ffff_ffff_ffff, 8), // -1 in 8 bytes -> magnitude 1
        (0x7fff_ffff, 4),           // INT32_MAX, positive -> no negsign
        (0, 4),                     // zero
    ];
    for &(val, sz) in cases {
        let (neg, mag, _fmt) = resolve_integer_format(val, sz, true, 0, false, false);
        let (oneg, omag) = oracle_resolve_sign(val, sz);
        assert_eq!(neg, oneg, "negsign mismatch for val={val:#x} sz={sz}");
        assert_eq!(mag, omag, "magnitude mismatch for val={val:#x} sz={sz}");
    }

    // FORCE_CHAR suppresses sign handling entirely (printc.cc:1384 guard
    // `displayFormat != Symbol::force_char`): even a top-bit-set value stays as is.
    let (neg, mag, fmt) =
        resolve_integer_format(0xff, 1, true, display_format::FORCE_CHAR, false, false);
    assert!(!neg, "force_char must suppress negsign");
    assert_eq!(mag, 0xff, "force_char must not strip the sign magnitude");
    assert_eq!(fmt, display_format::FORCE_CHAR);
}

/// Oracle: C++ `StringManager::writeUtf8(ostream&, int4 codepoint)`
/// (stringmanage.cc:124-158) for the in-range, non-throwing path.
fn oracle_writeutf8(codepoint: i32) -> Vec<u8> {
    assert!(codepoint >= 0);
    if codepoint < 128 {
        return vec![codepoint as u8];
    }
    let cp = codepoint as u32;
    // bits = mostsigbit_set + 1; encoding length by `bits` thresholds.
    let bits = 32 - cp.leading_zeros(); // == mostsigbit_set(cp)+1 for cp>0
    if bits < 12 {
        vec![
            (0xc0 ^ ((cp >> 6) & 0x1f)) as u8,
            (0x80 ^ (cp & 0x3f)) as u8,
        ]
    } else if bits < 17 {
        vec![
            (0xe0 ^ ((cp >> 12) & 0xf)) as u8,
            (0x80 ^ ((cp >> 6) & 0x3f)) as u8,
            (0x80 ^ (cp & 0x3f)) as u8,
        ]
    } else {
        vec![
            (0xf0 ^ ((cp >> 18) & 7)) as u8,
            (0x80 ^ ((cp >> 12) & 0x3f)) as u8,
            (0x80 ^ ((cp >> 6) & 0x3f)) as u8,
            (0x80 ^ (cp & 0x3f)) as u8,
        ]
    }
}

#[test]
fn printc_w8_print_unicode_raw_branch_matches_writeutf8_for_all_reachable() {
    // Walk every codepoint that can REACH the raw-emit branch (i.e. is NOT
    // escaped) across the full range printUnicode is ever called with
    // (0..0x2fa20, since >= 0x2fa20 is always escaped), plus a slice above it to
    // confirm those are escaped.  For each reachable codepoint, print_unicode's
    // bytes must equal C++ writeUtf8's encoding, and char::from_u32 must succeed.
    let mut reachable_checked = 0u64;
    for cp in 0i32..0x3_0000 {
        let escaped = unicode_needs_escape(cp);
        let mut got = String::new();
        print_unicode(&mut got, cp as int4);
        if escaped {
            // Escaped codepoints never use the raw UTF-8 path; they are an escape
            // sequence (covered by the other tests). Just assert no raw multi-byte
            // emission of the codepoint itself for surrogates (the danger zone).
            if (0xd800..=0xdfff).contains(&cp) {
                // Surrogates are escaped -> must NOT be emitted as raw bytes.
                assert!(
                    got.starts_with("\\x") || got.starts_with('\\'),
                    "surrogate {cp:#x} must be escaped, got {got:?}"
                );
            }
            continue;
        }
        // Reachable raw branch: char::from_u32 must succeed (no silent drop) and
        // the bytes must match writeUtf8.
        assert!(
            char::from_u32(cp as u32).is_some(),
            "reachable codepoint {cp:#x} must be a valid scalar (no silent drop)"
        );
        let want = oracle_writeutf8(cp);
        assert_eq!(got.as_bytes(), want.as_slice(), "utf8 byte mismatch at {cp:#x}");
        reachable_checked += 1;
    }
    // Sanity: we actually exercised a large body of the raw branch (not vacuous).
    assert!(reachable_checked > 1000, "expected many reachable codepoints, got {reachable_checked}");

    // A specific multi-byte boundary: U+00E9 'é' (2-byte) and U+0800 (3-byte,
    // first 3-byte codepoint), both in printable/non-escaped ranges.
    for &cp in &[0x00e9i32, 0x0800, 0x2fa1f /* just under the escape cliff */] {
        if unicode_needs_escape(cp) {
            continue;
        }
        let mut got = String::new();
        print_unicode(&mut got, cp as int4);
        assert_eq!(got.as_bytes(), oracle_writeutf8(cp).as_slice(), "multi-byte mismatch {cp:#x}");
    }
}

#[test]
fn printc_w8_format_float_dot_zero_and_scinote_boundary() {
    // looksLikeFloat keys on bytes '.' or 'e' (printc.cc:1480-1483).
    // No '.'/'e' -> ".0" appended.
    assert_eq!(format_float_token(FloatClass::Normal, false, "100", false), "100.0");
    assert_eq!(format_float_token(FloatClass::Normal, false, "0", false), "0.0");
    assert_eq!(format_float_token(FloatClass::Normal, false, "-7", false), "-7.0");
    // Has '.' -> untouched.
    assert_eq!(format_float_token(FloatClass::Normal, false, "3.5", false), "3.5");
    // Has 'e' -> untouched (no double-suffix).
    assert_eq!(format_float_token(FloatClass::Normal, false, "1e9", false), "1e9");
    // 'E' uppercase is NOT in the C++ test (only lowercase 'e'); FloatFormat
    // emits lowercase, but verify we don't accidentally treat 'E' as float-like.
    assert_eq!(format_float_token(FloatClass::Normal, false, "1E9", false), "1E9.0");
    // force_scinote bypasses the fix-up entirely (printc.cc:1473-1475): the
    // string is used verbatim even if it somehow lacks '.'/'e'.
    assert_eq!(format_float_token(FloatClass::Normal, false, "100", true), "100");
    // Special classes ignore the decimal entirely.
    assert_eq!(format_float_token(FloatClass::Infinity, true, "ignored", false), "-INFINITY");
    assert_eq!(format_float_token(FloatClass::Nan, false, "ignored", true), "NAN");
    assert_eq!(format_float_token(FloatClass::Unknown, true, "ignored", false), "FLOAT_UNKNOWN");
}

#[test]
fn printc_w8_format_integer_char_high_byte_and_wide_split() {
    // printc.cc:1419-1423: sz==1 && val>=0x80 -> printCharHexEscape (the raw
    // byte, NOT a unicode interpretation); else printUnicode.
    // 0x80 in 1 byte -> '\x80'.
    assert_eq!(
        format_integer_token(false, 0x80, display_format::FORCE_CHAR, 1, false, false, false, ""),
        "'\\x80'"
    );
    // 0xff in 1 byte -> '\xff' (a high byte, escaped, not é).
    assert_eq!(
        format_integer_token(false, 0xff, display_format::FORCE_CHAR, 1, false, false, false, ""),
        "'\\xff'"
    );
    // 0xe9 in 2 bytes (sz>1) does NOT take the high-byte escape branch (guard is
    // `sz == 1`): it goes through printUnicode, emitting raw é (U+00E9 printable).
    assert_eq!(
        format_integer_token(false, 0xe9, display_format::FORCE_CHAR, 2, false, false, false, ""),
        "'é'"
    );
    // Wide prefix only when doEmitWideCharPrefix && sz>1.
    assert_eq!(
        format_integer_token(false, b'A' as u64, display_format::FORCE_CHAR, 2, false, false, true, ""),
        "L'A'"
    );
    // wide_char_prefix true but sz==1 -> no 'L' (guard `sz > 1`).
    assert_eq!(
        format_integer_token(false, b'A' as u64, display_format::FORCE_CHAR, 1, false, false, true, ""),
        "'A'"
    );
    // A control char inside the char literal escapes via print_unicode.
    assert_eq!(
        format_integer_token(false, 0x09, display_format::FORCE_CHAR, 1, false, false, false, ""),
        "'\\t'"
    );
}
