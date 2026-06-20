//! Verifier adversarial tests for item `w2-harness-floatemu` (round 1).
//!
//! Independent of the ported `testfloatemu.rs`: fixtures and expected
//! values here are re-derived from the C++ source
//! (`decompiler/unittests/testfloatemu.cc`) and from a standalone C++
//! oracle probe (g++ -std=c++11, x86-64 glibc/libstdc++ host — verbatim
//! transcription of `FloatFormat::printDecimal` float.cc:446-477 plus the
//! `(int64_t)float` guard of TEST(float_opTrunc_to_int)), NOT from the
//! porter's helper functions.

use kuna_num::float::FloatFormat;

/// The 18 fixture bit patterns of the C++ `float_test_values` vector
/// (testfloatemu.cc:86-112), re-derived independently:
/// -0.0, +0.0, -1.0, +1.0, -1.234f, +1.234f, -denorm_min, +denorm_min,
/// min-denorm (largest subnormal), min, min+denorm, -min+denorm, -min,
/// -min-denorm, FLT_MAX, qNaN, -inf, +inf.
const FIXTURE_BITS: [u32; 18] = [
    0x8000_0000, // -0.0f
    0x0000_0000, // +0.0f
    0xbf80_0000, // -1.0f
    0x3f80_0000, // +1.0f
    0xbf9d_f3b6, // -1.234f
    0x3f9d_f3b6, // +1.234f
    0x8000_0001, // -denorm_min
    0x0000_0001, // +denorm_min
    0x007f_ffff, // min - denorm_min (largest subnormal)
    0x0080_0000, // min (FLT_MIN)
    0x0080_0001, // min + denorm_min
    0x807f_ffff, // -min + denorm_min (one ulp toward zero: -(largest subnormal))
    0x8080_0000, // -min
    0x8080_0001, // -min - denorm_min (one ulp away from zero)
    0x7f7f_ffff, // FLT_MAX
    0x7fc0_0000, // quiet_NaN (x86 host)
    0xff80_0000, // -inf
    0x7f80_0000, // +inf
];

/// Sanity: the bit patterns above really are the C++ constant-folded
/// values (one-ulp steps off FLT_MIN, exact in f32 arithmetic).
#[test]
fn verify_fixture_bits_rederivation() {
    let denorm = f32::from_bits(1);
    assert_eq!((-1.234f32).to_bits(), 0xbf9d_f3b6);
    assert_eq!((f32::MIN_POSITIVE - denorm).to_bits(), 0x007f_ffff);
    assert_eq!((f32::MIN_POSITIVE + denorm).to_bits(), 0x0080_0001);
    assert_eq!((-f32::MIN_POSITIVE + denorm).to_bits(), 0x807f_ffff);
    assert_eq!((-f32::MIN_POSITIVE - denorm).to_bits(), 0x8080_0001);
    assert_eq!(f32::MAX.to_bits(), 0x7f7f_ffff);
}

/// Every op test in the harness feeds `format.get_encoding(f64::from(f))`
/// as the op input where the C++ feeds `format.getEncoding(f)` (float
/// promoted to double).  That is only faithful if the promotion+re-encode
/// is bit-exact for ALL fixture values — including the subnormal boundary
/// cells (the w1 denormal-carry repair) and the NaN rows (payload through
/// cvtss2sd).  Pin it.
#[test]
fn verify_fixture_encodings_bit_exact() {
    let format = FloatFormat::new(4);
    for bits in FIXTURE_BITS {
        let f = f32::from_bits(bits);
        let enc = format.get_encoding(f64::from(f));
        assert_eq!(
            enc,
            u64::from(bits),
            "get_encoding(f64::from(f)) not bit-exact for fixture 0x{bits:08x}"
        );
    }
}

/// The `assert_float_encoding(f: f64)` rewrite of ASSERT_FLOAT_ENCODING
/// derives `true_encoding` via an f32->f64->f32 roundtrip that the C++
/// macro does not perform for float-typed arguments (it takes the float
/// bits directly).  Pin that the roundtrip is bit-exact for the NaN call
/// sites (sign + payload through cvtss2sd/cvtsd2ss on this host) and that
/// the widened doubles equal the x86 numeric_limits NaNs.
#[test]
fn verify_float_encoding_macro_nan_roundtrip() {
    for bits in [0x7fc0_0000u32, 0xffc0_0000u32] {
        let f = f32::from_bits(bits);
        let widened = f64::from(f);
        // cast: the macro rewrite's double -> float narrowing under test
        let narrowed = (widened as f32).to_bits();
        assert_eq!(narrowed, bits, "f32->f64->f32 NaN roundtrip changed bits");
    }
    assert_eq!(
        f64::from(f32::from_bits(0x7fc0_0000)).to_bits(),
        0x7ff8_0000_0000_0000u64
    );
    assert_eq!(
        f64::from(f32::from_bits(0xffc0_0000)).to_bits(),
        0xfff8_0000_0000_0000u64
    );
}

/// TEST(float_opTrunc_to_int)'s `(int64_t)f` guard is UB in C++ for NaN /
/// +-inf / FLT_MAX; the oracle host's cvttss2si yields i64::MIN for all
/// four (probe-verified: each prints -9223372036854775808 -> skipped), so
/// the C++ loop skips exactly those 4 of the 18 fixtures.  Re-derive the
/// skip set independently and check op_trunc on every KEPT cell against
/// `((uintb)(int32_t)f) & 0xffffffff` (sign-extension visible in the
/// mask: -1.234 truncates to -1 -> 0xffffffff).
#[test]
fn verify_trunc_guard_skip_set_and_kept_cells() {
    let format = FloatFormat::new(4);
    let skipped: [u32; 4] = [
        0x7f7f_ffff, // FLT_MAX: cvttss2si overflow -> i64::MIN -> skipped
        0x7fc0_0000, // qNaN: cvttss2si invalid -> i64::MIN -> skipped
        0xff80_0000, // -inf
        0x7f80_0000, // +inf
    ];
    let mut kept = 0;
    for bits in FIXTURE_BITS {
        if skipped.contains(&bits) {
            continue;
        }
        kept += 1;
        let f = f32::from_bits(bits);
        // independent expected value: truncate toward zero in f64 (exact
        // for every kept fixture), then C++ int32_t -> uintb sign extension
        let t = f64::from(f).trunc();
        assert!(t >= f64::from(i32::MIN) && t <= f64::from(i32::MAX));
        // cast: t is integral and in i32 range (asserted above)
        let expected = (t as i32 as i64 as u64) & 0xffff_ffff;
        let enc = format.get_encoding(f64::from(f));
        assert_eq!(
            format.op_trunc(enc, 4),
            expected,
            "op_trunc mismatch for fixture 0x{bits:08x}"
        );
    }
    assert_eq!(kept, 14, "C++ guard keeps exactly 14 of the 18 fixtures");
    // the sign-extension cell explicitly: -1.234 -> -1 -> 0xffffffff
    let enc = format.get_encoding(f64::from(f32::from_bits(0xbf9d_f3b6)));
    assert_eq!(format.op_trunc(enc, 4), 0xffff_ffff);
}

/// The five non-overflow rows of TEST(double_decimal_precision), pinned
/// from the standalone C++ oracle probe (identical strings to
/// testfloatemu.cc:184-195).  Proves the #[ignore]d test's divergence is
/// exactly ONE assertion (the DBL_MAX row), not six.
#[test]
fn verify_double_decimal_precision_non_overflow_rows() {
    let ff = FloatFormat::new(8);
    let cells: [(u64, bool, &str); 5] = [
        (0x3fc5555555555555, false, "0.16666666666666666"),
        (0x3fd555555c7dda4b, false, "0.33333334"),
        (0x3fd0000000000000, false, "0.25"),
        (0x3fb999999999999a, false, "0.1"),
        (0x3fbf7ced916872b0, true, "1.23000000000000e-01"),
    ];
    for (bits, forcesci, expect) in cells {
        assert_eq!(ff.print_decimal(f64::from_bits(bits), forcesci), expect);
    }
}

/// LOSS-025 divergence trace, DBL_MAX row of TEST(double_decimal_precision)
/// (testfloatemu.cc:186-187).
///
/// Root cause (stable Rust-stdlib fact, asserted first): the precision-15
/// rendering "1.79769313486232e+308" overflows; C++11 num_get stores
/// numeric_limits<double>::max() on ERANGE (probe-verified on this host's
/// libstdc++: reads back == DBL_MAX with failbit), so the C++ roundtrip
/// matches and printDecimal returns the 15-digit string.  Rust's
/// str::parse returns +inf instead, so float.rs print_decimal's ladder
/// never matches and falls through to decimal_max_precision (17 digits).
/// The same gap exists on the size<=4 path (parse::<f32> -> inf vs the
/// num_get FLT_MAX store), also asserted below.
///
/// The final assertion accepts EITHER the current divergent output (loss
/// open) or the C++ oracle string (loss repaired — then un-ignore
/// testfloatemu.rs double_decimal_precision and drop this trace).
#[test]
fn verify_print_decimal_dbl_max_loss_trace() {
    // root cause: Rust parse overflows to inf where C++11 num_get stores MAX
    assert_eq!("1.79769313486232e+308".parse::<f64>(), Ok(f64::INFINITY));
    assert_eq!("3.402824e+38".parse::<f32>(), Ok(f32::INFINITY));

    let ff = FloatFormat::new(8);
    assert_eq!(f64::MAX.to_bits(), 0x7fefffffffffffff);
    let got = ff.print_decimal(f64::MAX, false);
    let cpp_oracle = "1.79769313486232e+308"; // probe + C++ test baseline
    let rust_current = "1.7976931348623157e+308"; // 17-digit fallthrough
    assert!(
        got == cpp_oracle || got == rust_current,
        "unexpected third behavior for printDecimal(DBL_MAX): {got:?}"
    );
}
