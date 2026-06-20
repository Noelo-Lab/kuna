//! VERIFIER adversarial tests for `rport/w10-float-cluster` — the float-lane
//! `getNameRepresentative` / `handleSymbolConflict` naming seam plus the
//! `PrintC::push_float` float-constant render that together unblock the
//! Floating-point cast / Float print / Long double clusters.
//!
//! Protocol: docs/rust-port/verification.md.  The cross-crate-reachable,
//! special-case-free surface this stage changed is:
//!
//!   1. `PrintC::push_float` (printc.cc:1448) — a `TYPE_FLOAT` constant renders
//!      as a DECIMAL LITERAL via the `FloatFormat` decode, never as raw hex.
//!      The two halves are exercised here:
//!        * [`format_float_token`] — the `INFINITY`/`NAN`/`FLOAT_UNKNOWN`
//!          sentinels and the `.0` fix-up (printc.cc:1454-1487), and
//!        * the [`kuna_num::float::FloatFormat`] decode the caller feeds it
//!          (`getHostFloat`/`extractSign`/`printDecimal`) — the value is
//!          *computed from the IEEE-754 encoding*, NOT a hardcoded constant.
//!   2. `Translate::setDefaultFloatFormats` (translate.cc:980) — the gate that
//!      makes `getFloatFormat(4)`/`getFloatFormat(8)` resolve; without it
//!      `push_float` has no format and emits `FLOAT_UNKNOWN`.
//!
//! The naming seam (`handleSymbolConflict`, funcdata_varnode.cc:1018) lives in
//! private fns reached only through the full decompile pipeline; it is verified
//! end-to-end by the `floatcast`/`floatprint` datatests (0→4 / 0→14).  The
//! ANTI-SPECIAL-CASING property checked here is that the literal a float8
//! constant renders to is produced by the format decode applied to its raw
//! encoding — feed a different encoding, get a different number — so no
//! function name, address, or constant value is hardcoded anywhere on the path.

use kuna_decomp::printc::{format_float_token, FloatClass};
use kuna_num::float::{floatclass, FloatFormat};

/// `format_float_token` reproduces the C++ `push_float` sentinel/`.0` body
/// (printc.cc:1454-1487) for EVERY class, with the sign honoured on
/// infinity/NaN.  These are the exact tokens the decompiler must emit.
#[test]
fn w10fc_format_float_token_covers_all_classes() {
    // No FloatFormat for the size -> FLOAT_UNKNOWN (printc.cc:1454).
    assert_eq!(format_float_token(FloatClass::Unknown, false, "", false), "FLOAT_UNKNOWN");
    assert_eq!(format_float_token(FloatClass::Unknown, true, "ignored", false), "FLOAT_UNKNOWN");

    // Infinity / NaN carry the sign (printc.cc:1458-1469).
    assert_eq!(format_float_token(FloatClass::Infinity, false, "", false), "INFINITY");
    assert_eq!(format_float_token(FloatClass::Infinity, true, "", false), "-INFINITY");
    assert_eq!(format_float_token(FloatClass::Nan, false, "", false), "NAN");
    assert_eq!(format_float_token(FloatClass::Nan, true, "", false), "-NAN");

    // Normal: a decimal that already looks like a float is passed through
    // verbatim; one that does NOT gets the `.0` fix-up (printc.cc:1477-1487).
    assert_eq!(format_float_token(FloatClass::Normal, false, "1.1234567812345", false), "1.1234567812345");
    assert_eq!(format_float_token(FloatClass::Normal, false, "3", false), "3.0");
    assert_eq!(format_float_token(FloatClass::Normal, false, "1e10", false), "1e10");
    // force_scinote: passed through verbatim (printc.cc:1472-1474), no `.0`.
    assert_eq!(format_float_token(FloatClass::Normal, false, "5e0", true), "5e0");
}

/// The float8 constant the `floatcast` oracle renders, `1.1234567812345`, is
/// DECODED from its raw IEEE-754 encoding `0x3ff1f9add15df217`, not hardcoded.
/// This is the anti-special-casing core: the rendered literal is a pure
/// function of the encoding, so a verifier grep for the value string finds it
/// nowhere on the path.
#[test]
fn w10fc_float8_constant_decoded_from_encoding_not_hardcoded() {
    let fmt = FloatFormat::new(8);
    // The two prec_conditions readonly constants (dat_100080 / dat_100088).
    let enc_a: u64 = 0x3ff1f9add15df217; // -> 1.1234567812345
    let enc_b: u64 = 0x3ff1f9add1091c89; // -> 1.12345678
    let (val_a, class_a) = fmt.get_host_float(enc_a);
    let (val_b, class_b) = fmt.get_host_float(enc_b);
    assert_eq!(class_a, floatclass::normalized);
    assert_eq!(class_b, floatclass::normalized);
    assert!(!fmt.extract_sign(enc_a), "positive constant");

    // The shortest-round-trip decimal IS the oracle's literal (printDecimal,
    // float.cc:446-479) — computed, not a string constant in the source.
    let tok_a = format_float_token(
        FloatClass::Normal,
        fmt.extract_sign(enc_a),
        &fmt.print_decimal(val_a, false),
        false,
    );
    let tok_b = format_float_token(
        FloatClass::Normal,
        fmt.extract_sign(enc_b),
        &fmt.print_decimal(val_b, false),
        false,
    );
    assert_eq!(tok_a, "1.1234567812345");
    assert_eq!(tok_b, "1.12345678");

    // Adversarial: a DIFFERENT encoding yields a DIFFERENT literal (proves the
    // value tracks the bits — not a constant pinned to one test input).
    let enc_other: u64 = 0x4023c0ca45ddccdb; // prec_comparison's 9.8765432198765
    let (val_other, _) = fmt.get_host_float(enc_other);
    let tok_other =
        format_float_token(FloatClass::Normal, false, &fmt.print_decimal(val_other, false), false);
    assert_eq!(tok_other, "9.8765432198765");
    assert_ne!(tok_other, tok_a, "distinct encodings must render distinct literals");
}

/// The `getEncoding`/`getHostFloat` round-trip is exact for the readonly float8
/// constants: re-encoding the decoded host double reproduces the original bits.
/// This guards the decode the render path depends on against a lossy/by-value
/// shortcut.
#[test]
fn w10fc_float8_decode_roundtrips_encoding() {
    let fmt = FloatFormat::new(8);
    for enc in [0x3ff1f9add15df217u64, 0x3ff1f9add1091c89u64, 0x4023c0ca45ddccdbu64] {
        let (host, class) = fmt.get_host_float(enc);
        assert_eq!(class, floatclass::normalized);
        assert_eq!(fmt.get_encoding(host), enc, "host->encoding must reproduce the bits for {enc:#x}");
    }
}

/// Infinity / NaN float8 encodings render to the sentinel tokens (not a stray
/// decimal): the special IEEE classes are classified by the decode, then routed
/// by `push_float`'s class switch.  Encodings, not magic numbers.
#[test]
fn w10fc_float8_special_classes_render_sentinels() {
    let fmt = FloatFormat::new(8);
    let pos_inf: u64 = 0x7ff0000000000000;
    let neg_inf: u64 = 0xfff0000000000000;
    let a_nan: u64 = 0x7ff8000000000000;

    let (_, c_pi) = fmt.get_host_float(pos_inf);
    let (_, c_ni) = fmt.get_host_float(neg_inf);
    let (_, c_nan) = fmt.get_host_float(a_nan);
    assert_eq!(c_pi, floatclass::infinity);
    assert_eq!(c_ni, floatclass::infinity);
    assert_eq!(c_nan, floatclass::nan);

    assert_eq!(
        format_float_token(FloatClass::Infinity, fmt.extract_sign(pos_inf), "", false),
        "INFINITY"
    );
    assert_eq!(
        format_float_token(FloatClass::Infinity, fmt.extract_sign(neg_inf), "", false),
        "-INFINITY"
    );
    assert_eq!(format_float_token(FloatClass::Nan, fmt.extract_sign(a_nan), "", false), "NAN");
}

/// `FloatFormat::new(4)` and `new(8)` are the IEEE-754 defaults
/// `setDefaultFloatFormats` installs (translate.cc:983-985) — both resolve and
/// classify a normal value, which is the gate that lets `push_float` produce a
/// literal instead of `FLOAT_UNKNOWN` (the regression this stage removed).
#[test]
fn w10fc_default_float_formats_decode_both_sizes() {
    let f4 = FloatFormat::new(4);
    let f8 = FloatFormat::new(8);
    assert_eq!(f4.get_size(), 4);
    assert_eq!(f8.get_size(), 8);
    // A float4 1.0 (0x3f800000) and a float8 1.0 (0x3ff0000000000000) both
    // classify normal — i.e. a registered format actually decodes.
    let (v4, c4) = f4.get_host_float(0x3f80_0000);
    let (v8, c8) = f8.get_host_float(0x3ff0_0000_0000_0000);
    assert_eq!(c4, floatclass::normalized);
    assert_eq!(c8, floatclass::normalized);
    assert_eq!(format_float_token(FloatClass::Normal, false, &f4.print_decimal(v4, false), false), "1.0");
    assert_eq!(format_float_token(FloatClass::Normal, false, &f8.print_decimal(v8, false), false), "1.0");
}
