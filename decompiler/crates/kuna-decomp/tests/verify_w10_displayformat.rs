//! Independent-verifier adversarial tests for item `rport/w10-displayformat`
//! (`force varnode <varnode> <format>`: IfcForceFormat -> readVarnode ->
//! Funcdata::buildDynamicSymbol (equate Symbol) -> Scope::setDisplayFormat ->
//! the per-symbol display-format honoring in PrintC::push_integer).
//!
//! Written by the VERIFIER (not the porter).  The load-bearing behavior this
//! item introduces is the C++ `push_integer` decision (printc.cc:1370-1405):
//! once an equate Symbol is bound to the constant's HighVariable, its
//! `getDisplayFormat()` value is threaded in as `displayFormat`, and
//!
//!     if (displayFormat != 0) { /* forced by the Symbol */ }
//!     else if (force_hex) ...
//!     else if (val<=10 || force_dec) ...
//!     else mostNaturalBase(...)
//!
//! i.e. a non-zero forced format MUST win over the `val<=10` / `mostNaturalBase`
//! default *and* over the printer's own force_hex/force_dec mods.  These tests
//! exercise that fork directly through the public, deterministic, engine-free
//! `printc::resolve_integer_format` (the fork) and `printc::format_integer_token`
//! (the rendering of each of the five encodeIntegerFormat codes), grounded
//! against what the C++ `push_integer` formatter (`t << ...`) emits.
//!
//! Hunt-list spots targeted for THIS item:
//!   F1. The forced-format short-circuit: a non-NONE `display_fmt_in` must win
//!       over both the natural-base default and the FORCE_HEX/FORCE_DEC mods —
//!       the whole point of `force varnode`.  A port that let the mods or the
//!       `val<=10` branch override would silently ignore the forced format.
//!   F2. Encode-mapping totality (encodeIntegerFormat, type.cc:732): hex=1,
//!       dec=2, oct=3, bin=4, char=5 — each renders its own prefix/shape; no
//!       value/format is special-cased.
//!   F3. force_char interaction with the signed path: C++ suppresses the
//!       signed-magnitude flip when `displayFormat == force_char`
//!       (`sign && displayFormat != Symbol::force_char`), so a high-bit char
//!       value is NOT two's-complemented.
//!   F4. Width/wrap edges: an 8-byte 0xFFFF_FFFF_FFFF_FFFF forced to dec must
//!       print the full unsigned magnitude (no signed flip, no wrap panic).

use kuna_decomp::printc::{display_format, format_integer_token, resolve_integer_format};

// ---------------------------------------------------------------------------
// F1: the forced format wins over the natural-base default AND the mods.
// ---------------------------------------------------------------------------

/// A value whose natural default is HEX (255 -> mostNaturalBase==16) forced to
/// DEC must render decimal "255"; a value whose natural default is DEC (7 ->
/// val<=10) forced to HEX must render "0x7".  This is the `force varnode`
/// override doing its job against `resolve_integer_format`'s own default arm.
#[test]
fn forced_format_overrides_natural_base_default() {
    // 255: with no override (NONE) the natural base is hex -> "0xff".
    let (neg, val, fmt) =
        resolve_integer_format(255, 4, false, display_format::NONE, false, false);
    assert!(!neg);
    assert_eq!(val, 255);
    assert_eq!(fmt, display_format::FORCE_HEX, "255 is hex-natural by default");
    assert_eq!(
        format_integer_token(false, 255, fmt, 4, false, false, false, ""),
        "0xff"
    );

    // 255 with the equate Symbol forcing DEC (code 2) -> the short-circuit keeps
    // dec; render "255".
    let (_, _, fmt_dec) =
        resolve_integer_format(255, 4, false, display_format::FORCE_DEC, false, false);
    assert_eq!(fmt_dec, display_format::FORCE_DEC, "forced dec must win");
    assert_eq!(
        format_integer_token(false, 255, fmt_dec, 4, false, false, false, ""),
        "255"
    );

    // 7: natural default is decimal (val<=10); forcing HEX (code 1) must win.
    let (_, _, fmt_hex) =
        resolve_integer_format(7, 4, false, display_format::FORCE_HEX, false, false);
    assert_eq!(fmt_hex, display_format::FORCE_HEX, "forced hex must win over val<=10");
    assert_eq!(
        format_integer_token(false, 7, fmt_hex, 4, false, false, false, ""),
        "0x7"
    );
}

/// The forced Symbol format must also win over the printer's own `force_hex` /
/// `force_dec` mods.  In C++ `push_integer` the `if (displayFormat != 0)` arm is
/// checked BEFORE the `(mods & force_hex)` / `(mods & force_dec)` arms, so a
/// constant the user forced to DEC stays decimal even while the global hex mod
/// is active (and vice-versa).
#[test]
fn forced_format_beats_force_hex_and_force_dec_mods() {
    // Symbol forces DEC while the global force_hex mod is ON: dec must win.
    let (_, _, fmt) =
        resolve_integer_format(255, 4, false, display_format::FORCE_DEC, true, false);
    assert_eq!(
        fmt,
        display_format::FORCE_DEC,
        "Symbol's forced dec must beat the force_hex mod"
    );

    // Symbol forces HEX while the global force_dec mod is ON: hex must win.
    let (_, _, fmt2) =
        resolve_integer_format(255, 4, false, display_format::FORCE_HEX, false, true);
    assert_eq!(
        fmt2,
        display_format::FORCE_HEX,
        "Symbol's forced hex must beat the force_dec mod"
    );

    // Control: with NO forced format (NONE), the force_hex mod DOES take effect
    // even on a val<=10 value (proving the override path, not a coincidence).
    let (_, _, fmt3) = resolve_integer_format(7, 4, false, display_format::NONE, true, false);
    assert_eq!(fmt3, display_format::FORCE_HEX, "force_hex mod applies when unforced");
}

// ---------------------------------------------------------------------------
// F2: encodeIntegerFormat totality — every code renders its own shape.
// ---------------------------------------------------------------------------

/// Each of the five `Datatype::encodeIntegerFormat` codes (hex=1, dec=2, oct=3,
/// bin=4, char=5) must produce the exact C++ `push_integer` formatter output:
///   force_hex -> "0x"+hex, force_dec -> dec, force_oct -> "0"+oct,
///   force_bin -> "0b"+binary, force_char -> 'c'.
/// Same value (65), five formats, five distinct renderings — no special-casing,
/// no format collapsing.
#[test]
fn each_encode_integer_format_renders_its_own_shape() {
    let v: u64 = 65; // 'A', 0x41, 0o101, 0b1000001
    // hex (code 1)
    assert_eq!(
        format_integer_token(false, v, display_format::FORCE_HEX, 4, false, false, false, ""),
        "0x41"
    );
    // dec (code 2)
    assert_eq!(
        format_integer_token(false, v, display_format::FORCE_DEC, 4, false, false, false, ""),
        "65"
    );
    // oct (code 3): C++ `t << oct << '0' << val`
    assert_eq!(
        format_integer_token(false, v, display_format::FORCE_OCT, 4, false, false, false, ""),
        "0101"
    );
    // bin (code 4): C++ `t << "0b"; formatBinary(...)`.  formatBinary pads up to
    // the next byte boundary (mostsigbit 6 <= 7 -> 8 bits), so 65 -> "01000001".
    assert_eq!(
        format_integer_token(false, v, display_format::FORCE_BIN, 4, false, false, false, ""),
        "0b01000001"
    );
    // char (code 5), size 1, printable -> 'A'
    assert_eq!(
        format_integer_token(false, v, display_format::FORCE_CHAR, 1, false, false, false, ""),
        "'A'"
    );
}

// ---------------------------------------------------------------------------
// F3: force_char suppresses the signed two's-complement flip.
// ---------------------------------------------------------------------------

/// C++ `push_integer`: `if (sign && displayFormat != Symbol::force_char)` does
/// the magnitude flip.  When the forced format is `char`, a signed request must
/// NOT flip a high-bit byte to a negative magnitude — it stays the raw value and
/// renders as the (escaped) character.  Compare against a NON-char signed format
/// where the flip DOES happen.
#[test]
fn force_char_suppresses_signed_flip() {
    // size 1, value 0xFF, signed request, forced CHAR: no flip, print_negsign
    // stays false, val unchanged (0xFF), and a >=0x80 sz-1 char hex-escapes.
    let (neg, val, fmt) =
        resolve_integer_format(0xFF, 1, true, display_format::FORCE_CHAR, false, false);
    assert!(!neg, "force_char must suppress the signed-magnitude flip");
    assert_eq!(val, 0xFF, "force_char must not two's-complement the value");
    assert_eq!(fmt, display_format::FORCE_CHAR);
    // sz==1 && val>=0x80 -> printCharHexEscape (C++ printc.cc:1417).
    assert_eq!(
        format_integer_token(false, val, fmt, 1, false, false, false, ""),
        "'\\xff'"
    );

    // Contrast: same value, signed, forced DEC -> the flip DOES fire
    // (0xFF in size 1 is -1), so print_negsign=true and val becomes 1.
    let (neg2, val2, fmt2) =
        resolve_integer_format(0xFF, 1, true, display_format::FORCE_DEC, false, false);
    assert!(neg2, "a signed non-char high-bit byte flips to negative");
    assert_eq!(val2, 1, "0xFF size-1 signed magnitude is 1 (prints as -1)");
    assert_eq!(
        format_integer_token(neg2, val2, fmt2, 1, false, false, false, ""),
        "-1"
    );
}

// ---------------------------------------------------------------------------
// F4: 64-bit width / wrap edge — forced dec on the all-ones unsigned value.
// ---------------------------------------------------------------------------

/// An 8-byte 0xFFFF_FFFF_FFFF_FFFF constant forced to DEC (unsigned, no signed
/// request) must print the full unsigned magnitude 18446744073709551615 with no
/// two's-complement flip and no debug-build wrap panic on the `flip+1` path
/// (the path is not taken because `sign==false`).
#[test]
fn forced_dec_on_all_ones_uintb_prints_full_magnitude() {
    let all_ones: u64 = u64::MAX;
    let (neg, val, fmt) =
        resolve_integer_format(all_ones, 8, false, display_format::FORCE_DEC, false, false);
    assert!(!neg);
    assert_eq!(val, all_ones);
    assert_eq!(fmt, display_format::FORCE_DEC);
    assert_eq!(
        format_integer_token(false, val, fmt, 8, false, false, false, ""),
        "18446744073709551615"
    );

    // And the same all-ones value forced to HEX renders the full 16 nibbles.
    let (_, _, fmt_h) =
        resolve_integer_format(all_ones, 8, false, display_format::FORCE_HEX, false, false);
    assert_eq!(
        format_integer_token(false, all_ones, fmt_h, 8, false, false, false, ""),
        "0xffffffffffffffff"
    );
}
