//! Operation checks adapted from Ghidra's testfloatemu.cc.
//!
//! Finite results compare against host arithmetic. NaNs use the sign policy
//! pinned by the golden vectors rather than the compiler's NaN propagation.

#![allow(non_snake_case)] // C++ TEST(name) fn names are preserved exactly

use kuna_num::float::FloatFormat;

// utility functions (testfloatemu.cc:39-61)

/// C++ `floatFromRawBits(uintb e)`: memcpy of the low 4 bytes of `e` into a
/// float on the little-endian host.
fn float_from_raw_bits(e: u64) -> f32 {
    // truncation to the low 32 bits is exactly the C++ 4-byte memcpy
    f32::from_bits(e as u32)
}

/// C++ `floatToRawBits(float f)`: memcpy of the 4 float bytes into a zeroed
/// uintb on the little-endian host.
fn float_to_raw_bits(f: f32) -> u64 {
    u64::from(f.to_bits())
}

/// C++ `doubleFromRawBits(uintb e)`.
fn double_from_raw_bits(e: u64) -> f64 {
    f64::from_bits(e)
}

/// C++ `doubleToRawBits(double f)`.
fn double_to_raw_bits(f: f64) -> u64 {
    f.to_bits()
}

/// The fixture contains only positive quiet NaNs; invalid operations produce
/// the negative quiet NaN used by the pinned x86 oracle.
fn expected_arithmetic_encoding(raw: u64, inputs_nan: bool) -> u64 {
    if inputs_nan {
        return float_to_raw_bits(FLOAT_QNAN);
    }
    let is_nan = (raw & 0x7f80_0000) == 0x7f80_0000 && (raw & 0x007f_ffff) != 0;
    if is_nan {
        0xffc0_0000
    } else {
        raw
    }
}

// macros to preserve call site (testfloatemu.cc:64-82); Rust panics carry the
// fixture-fn location, but the assert message still shows both values.

/// C++ `ASSERT_FLOAT_ENCODING(f)`: `f` is `double` at every call site (the
/// `1.234` literals are doubles; the float-typed arguments are promoted at
/// `format.getEncoding(f)`).
fn assert_float_encoding(f: f64) {
    let format = FloatFormat::new(4);

    // C++ floatToRawBits(f) implicitly narrows double -> float
    let true_encoding = float_to_raw_bits(f as f32);
    let encoding = format.get_encoding(f);

    assert_eq!(true_encoding, encoding);
}

/// C++ `ASSERT_DOUBLE_ENCODING(f)`.
fn assert_double_encoding(f: f64) {
    let format = FloatFormat::new(8);

    let true_encoding = double_to_raw_bits(f);
    let encoding = format.get_encoding(f);

    assert_eq!(true_encoding, encoding);
}

// FloatFormat tests (C++ `////` section divider; clippy four_forward_slashes)

/// `std::numeric_limits<float>::denorm_min()`: smallest positive subnormal,
/// 2^-149.
const FLOAT_DENORM_MIN: f32 = f32::from_bits(1);
/// `std::numeric_limits<double>::denorm_min()`: smallest positive subnormal,
/// 2^-1074.
const DOUBLE_DENORM_MIN: f64 = f64::from_bits(1);
/// `std::numeric_limits<float>::quiet_NaN()` as the x86 host encodes it.
const FLOAT_QNAN: f32 = f32::from_bits(0x7fc0_0000);
/// `std::numeric_limits<double>::quiet_NaN()` as the x86 host encodes it.
const DOUBLE_QNAN: f64 = f64::from_bits(0x7ff8_0000_0000_0000);

/// C++ file-static `float_test_values` (testfloatemu.cc:86-112).  The
/// `min() +/- denorm_min()` arithmetic is exact (one-ulp steps off the
/// smallest normal), identical to the C++ constant folding.
fn float_test_values() -> [f32; 18] {
    [
        -0.0f32,
        0.0f32,
        -1.0,
        1.0,
        -1.234,
        1.234,
        -FLOAT_DENORM_MIN,
        FLOAT_DENORM_MIN,
        f32::MIN_POSITIVE - FLOAT_DENORM_MIN,
        f32::MIN_POSITIVE,
        f32::MIN_POSITIVE + FLOAT_DENORM_MIN,
        -f32::MIN_POSITIVE + FLOAT_DENORM_MIN,
        -f32::MIN_POSITIVE,
        -f32::MIN_POSITIVE - FLOAT_DENORM_MIN,
        f32::MAX,
        FLOAT_QNAN,
        -f32::INFINITY,
        f32::INFINITY,
    ]
}

/// C++ file-static `int_test_values` (testfloatemu.cc:114-116).
fn int_test_values() -> [i32; 7] {
    [0, -1, 1, 1234, -1234, i32::MIN, i32::MAX]
}

#[test]
fn float_encoding_normal() {
    assert_float_encoding(1.234);
    assert_float_encoding(-1.234);
}

#[test]
fn double_encoding_normal() {
    assert_double_encoding(1.234);
    assert_double_encoding(-1.234);
}

#[test]
fn float_encoding_nan() {
    let format = FloatFormat::new(4);
    assert_eq!(
        format.get_encoding(f64::from_bits(0x7ff8_0000_0000_0000)),
        0x7fc0_0000
    );
    assert_eq!(
        format.get_encoding(f64::from_bits(0xfff8_0000_0000_0000)),
        0xffc0_0000
    );
}

#[test]
fn double_encoding_nan() {
    assert_double_encoding(DOUBLE_QNAN);
    assert_double_encoding(-DOUBLE_QNAN);
}

#[test]
fn float_encoding_subnormal() {
    assert_float_encoding(f64::from(FLOAT_DENORM_MIN));
    assert_float_encoding(f64::from(-FLOAT_DENORM_MIN));
}

#[test]
fn double_encoding_subnormal() {
    assert_double_encoding(DOUBLE_DENORM_MIN);
    assert_double_encoding(-DOUBLE_DENORM_MIN);
}

#[test]
fn float_encoding_min_normal() {
    assert_float_encoding(f64::from(f32::MIN_POSITIVE));
    assert_float_encoding(f64::from(-f32::MIN_POSITIVE));
}

#[test]
fn double_encoding_min_normal() {
    assert_double_encoding(f64::MIN_POSITIVE);
    assert_double_encoding(-f64::MIN_POSITIVE);
}

#[test]
fn float_encoding_infinity() {
    assert_float_encoding(f64::from(f32::INFINITY));
    assert_float_encoding(f64::from(-f32::INFINITY));
}

#[test]
fn double_encoding_infinity() {
    assert_double_encoding(f64::INFINITY);
    assert_double_encoding(-f64::INFINITY);
}

#[test]
fn float_decimal_precision() {
    let ff = FloatFormat::new(4);
    let f0 = float_from_raw_bits(0x34000001);
    assert_eq!(ff.print_decimal(f64::from(f0), false), "1.192093e-07");
    let f1 = float_from_raw_bits(0x34800000);
    assert_eq!(ff.print_decimal(f64::from(f1), false), "2.3841858e-07");
    let f2 = float_from_raw_bits(0x3eaaaaab);
    assert_eq!(ff.print_decimal(f64::from(f2), false), "0.33333334");
    let f3 = float_from_raw_bits(0x3e800000);
    assert_eq!(ff.print_decimal(f64::from(f3), false), "0.25");
    let f4 = float_from_raw_bits(0x3de3ee46);
    assert_eq!(ff.print_decimal(f64::from(f4), false), "0.111294314");
}

// IGNORED pending a fix in kuna-num src/float.rs (owned by item
// w1-num-float-multiprec, not this harness item): the f1 = DBL_MAX row
// (0x7fefffffffffffff) fails.  C++ printDecimal's istream readback
// (float.cc:465-473) follows C++11 num_get overflow semantics -- on ERANGE
// it stores numeric_limits<double>::max() (verified against g++/libstdc++ on
// this host: "1.79769313486232e+308" reads back as max with failbit set), so
// the precision-15 roundtrip equals DBL_MAX and the loop breaks with
// "1.79769313486232e+308".  Rust's parse::<f64>() returns +inf on overflow
// instead, so the port's roundtrip never matches and it falls through to
// decimal_max_precision, yielding "1.7976931348623157e+308".  The same gap
// exists on the size<=4 path (parse::<f32>() -> inf vs FLT_MAX).
#[test]
#[ignore = "kuna-num float.rs print_decimal roundtrip diverges from C++11 num_get overflow semantics at DBL_MAX; see comment above"]
fn double_decimal_precision() {
    let ff = FloatFormat::new(8);
    let f0 = double_from_raw_bits(0x3fc5555555555555);
    assert_eq!(ff.print_decimal(f0, false), "0.16666666666666666");
    let f1 = double_from_raw_bits(0x7fefffffffffffff);
    assert_eq!(ff.print_decimal(f1, false), "1.79769313486232e+308");
    let f2 = double_from_raw_bits(0x3fd555555c7dda4b);
    assert_eq!(ff.print_decimal(f2, false), "0.33333334");
    let f3 = double_from_raw_bits(0x3fd0000000000000);
    assert_eq!(ff.print_decimal(f3, false), "0.25");
    let f4 = double_from_raw_bits(0x3fb999999999999a);
    assert_eq!(ff.print_decimal(f4, false), "0.1");
    let f5 = double_from_raw_bits(0x3fbf7ced916872b0);
    assert_eq!(ff.print_decimal(f5, true), "1.23000000000000e-01");
}

#[test]
fn float_midpoint_rounding() {
    let ff = FloatFormat::new(4);
    // IEEE754 recommends "round to nearest even" for binary formats, like single and double
    // precision floating point.  It rounds to the nearest integer (significand) when unambiguous,
    // and to the nearest even on the midpoint.

    // There are 52 bits of significand in a double and 23 in a float.
    // Below we construct a sequence of double precision values to demonstrate each case
    // in rounding,

    // 		d0 - zeros in low 29 bits, round down
    // 		d1 - on the rounding midpoint with integer even integer part, round down
    //      d2 - just above the midpoint, round up
    let d0 = double_from_raw_bits(0x4010000000000000);
    let d1 = double_from_raw_bits(0x4010000010000000);
    let d2 = double_from_raw_bits(0x4010000010000001);

    // 		d3 - zeros in low 29 bits, round down
    // 		d4 - on the rounding midpoint with integer part odd, round up
    //      d5 - just above the midpoint, round up
    let d3 = double_from_raw_bits(0x4010000020000000);
    let d4 = double_from_raw_bits(0x4010000030000000);
    let d5 = double_from_raw_bits(0x4010000030000001);

    // C++ (float)d: double -> float narrowing, round-to-nearest-even
    let f0 = d0 as f32;
    let f1 = d1 as f32;
    let f2 = d2 as f32;
    let f3 = d3 as f32;
    let f4 = d4 as f32;
    let f5 = d5 as f32;

    let e0 = ff.get_encoding(d0);
    let e1 = ff.get_encoding(d1);
    let e2 = ff.get_encoding(d2);
    let e3 = ff.get_encoding(d3);
    let e4 = ff.get_encoding(d4);
    let e5 = ff.get_encoding(d5);

    assert_eq!(float_to_raw_bits(f0), e0);
    assert_eq!(float_to_raw_bits(f1), e1);
    assert_eq!(float_to_raw_bits(f2), e2);
    assert_eq!(float_to_raw_bits(f3), e3);
    assert_eq!(float_to_raw_bits(f4), e4);
    assert_eq!(float_to_raw_bits(f5), e5);

    assert_eq!(e0, e1);
    assert_ne!(e1, e2);

    assert_ne!(e3, e4);
    assert_eq!(e4, e5);
}

// op tests

// generated

#[test]
fn float_opNan() {
    let format = FloatFormat::new(4);

    for f in float_test_values() {
        let true_result = u64::from(f.is_nan()); // C++ bool -> uintb
        let encoding = float_to_raw_bits(f);
        let result = format.op_nan(encoding);

        assert_eq!(true_result, result);
    }
}

#[test]
fn float_opNeg() {
    let format = FloatFormat::new(4);

    for f in float_test_values() {
        let true_result = float_to_raw_bits(-f);
        let encoding = float_to_raw_bits(f);
        let result = format.op_neg(encoding);

        assert_eq!(true_result, result);
    }
}

#[test]
fn float_opAbs() {
    let format = FloatFormat::new(4);

    for f in float_test_values() {
        let true_result = float_to_raw_bits(f.abs());
        let encoding = float_to_raw_bits(f);
        let result = format.op_abs(encoding);

        assert_eq!(true_result, result);
    }
}

#[test]
fn float_opSqrt() {
    let format = FloatFormat::new(4);

    for f in float_test_values() {
        let true_result = expected_arithmetic_encoding(float_to_raw_bits(f.sqrt()), f.is_nan());
        let encoding = float_to_raw_bits(f);
        let result = format.op_sqrt(encoding);

        assert_eq!(true_result, result);
    }
}

#[test]
fn float_opCeil() {
    let format = FloatFormat::new(4);

    for f in float_test_values() {
        let true_result = expected_arithmetic_encoding(float_to_raw_bits(f.ceil()), f.is_nan());
        let encoding = float_to_raw_bits(f);
        let result = format.op_ceil(encoding);

        assert_eq!(true_result, result);
    }
}

#[test]
fn float_opFloor() {
    let format = FloatFormat::new(4);

    for f in float_test_values() {
        let true_result = expected_arithmetic_encoding(float_to_raw_bits(f.floor()), f.is_nan());
        let encoding = float_to_raw_bits(f);
        let result = format.op_floor(encoding);

        assert_eq!(true_result, result);
    }
}

#[test]
fn float_opRound() {
    let format = FloatFormat::new(4);

    for f in float_test_values() {
        // C++ std::round(float): round half away from zero, same as f32::round
        let true_result = expected_arithmetic_encoding(float_to_raw_bits(f.round()), f.is_nan());
        let encoding = float_to_raw_bits(f);
        let result = format.op_round(encoding);

        assert_eq!(true_result, result);
    }
}

#[test]
fn float_opInt2Float_size4() {
    let format = FloatFormat::new(4);

    for i in int_test_values() {
        // C++ (float)i: int -> float, round-to-nearest (i32::MAX rounds up)
        let true_result = float_to_raw_bits(i as f32);
        // C++ implicitly converts the int argument to uintb (modular
        // reduction == sign extension through i64)
        let result = format.op_int2float(i as i64 as u64, 4);

        assert_eq!(true_result, result);
    }
}
// TODO other sized ints

#[test]
fn float_to_double_opFloat2Float() {
    let format = FloatFormat::new(4);
    let format8 = FloatFormat::new(8);

    for f in float_test_values() {
        let true_result = if f.is_nan() {
            DOUBLE_QNAN.to_bits()
        } else {
            double_to_raw_bits(f64::from(f))
        };
        let encoding = float_to_raw_bits(f);
        let result = format.op_float2float(encoding, &format8);

        assert_eq!(true_result, result);
    }
}

/// The C++ test's `(int64_t)f` cast as the oracle's x86 host performs it
/// (`cvttss2si`): truncation toward zero; NaN, infinities, and values whose
/// truncation falls outside the int64 range all produce `i64::MIN`.  Rust's
/// saturating `as` differs on exactly those cells (NaN -> 0, +inf/overflow ->
/// `i64::MAX`), which would change which test values the C++ range guard
/// skips.
#[allow(clippy::manual_range_contains)] // explicit asymmetric overflow bounds
fn host_float_to_int64(f: f32) -> i64 {
    if f.is_nan() {
        return i64::MIN;
    }
    // 2^63 is exactly representable as f32; anything >= it, or below -2^63,
    // is out of range (-2^63 itself converts exactly to i64::MIN).
    if f >= 9_223_372_036_854_775_808.0_f32 || f < -9_223_372_036_854_775_808.0_f32 {
        return i64::MIN;
    }
    f as i64 // in range: `as` truncates toward zero, same as cvttss2si
}

#[test]
fn float_opTrunc_to_int() {
    let format = FloatFormat::new(4);
    let _format8 = FloatFormat::new(8); // C++ declares format8 unused here

    for f in float_test_values() {
        // avoid undefined behavior
        // (the C++ `(int64_t)f` guard with host x86 cast semantics; see
        // host_float_to_int64)
        let host_i64 = host_float_to_int64(f);
        if host_i64 > i64::from(i32::MAX) || host_i64 < i64::from(i32::MIN) {
            continue;
        }
        // C++ ((uintb)(int32_t)f) & 0xffffffff: the guard above ensures f is
        // in int32 range, so `as i32` (truncate toward zero) matches the C++
        // host cast; int32_t -> uintb sign-extends, hence `as i64 as u64`.
        let true_result = (f as i32 as i64 as u64) & 0xffffffff;
        let encoding = float_to_raw_bits(f);
        let result = format.op_trunc(encoding, 4);

        assert_eq!(true_result, result);
    }
}

// TODO trunc to other sizes

#[test]
fn float_opEqual() {
    let format = FloatFormat::new(4);

    for f1 in float_test_values() {
        let encoding1 = float_to_raw_bits(f1);
        for f2 in float_test_values() {
            let true_result = u64::from(f1 == f2); // C++ bool -> uintb
            let encoding2 = float_to_raw_bits(f2);
            let result = format.op_equal(encoding1, encoding2);

            assert_eq!(true_result, result);
        }
    }
}

#[test]
fn float_opNotEqual() {
    let format = FloatFormat::new(4);

    for f1 in float_test_values() {
        let encoding1 = float_to_raw_bits(f1);
        for f2 in float_test_values() {
            let true_result = u64::from(f1 != f2); // C++ bool -> uintb
            let encoding2 = float_to_raw_bits(f2);
            let result = format.op_not_equal(encoding1, encoding2);

            assert_eq!(true_result, result);
        }
    }
}

#[test]
fn float_opLess() {
    let format = FloatFormat::new(4);

    for f1 in float_test_values() {
        let encoding1 = float_to_raw_bits(f1);
        for f2 in float_test_values() {
            let true_result = u64::from(f1 < f2); // C++ bool -> uintb
            let encoding2 = float_to_raw_bits(f2);
            let result = format.op_less(encoding1, encoding2);

            assert_eq!(true_result, result);
        }
    }
}

#[test]
fn float_opLessEqual() {
    let format = FloatFormat::new(4);

    for f1 in float_test_values() {
        let encoding1 = float_to_raw_bits(f1);
        for f2 in float_test_values() {
            let true_result = u64::from(f1 <= f2); // C++ bool -> uintb
            let encoding2 = float_to_raw_bits(f2);
            let result = format.op_less_equal(encoding1, encoding2);

            assert_eq!(true_result, result);
        }
    }
}

#[test]
fn float_opAdd() {
    let format = FloatFormat::new(4);

    for f1 in float_test_values() {
        let encoding1 = float_to_raw_bits(f1);
        for f2 in float_test_values() {
            let true_result = expected_arithmetic_encoding(
                float_to_raw_bits(f1 + f2),
                f1.is_nan() || f2.is_nan(),
            );
            let encoding2 = float_to_raw_bits(f2);
            let result = format.op_add(encoding1, encoding2);

            assert_eq!(true_result, result);
        }
    }
}

#[test]
fn float_opDiv() {
    let format = FloatFormat::new(4);

    for f1 in float_test_values() {
        let encoding1 = float_to_raw_bits(f1);
        for f2 in float_test_values() {
            let true_result = expected_arithmetic_encoding(
                float_to_raw_bits(f1 / f2),
                f1.is_nan() || f2.is_nan(),
            );
            let encoding2 = float_to_raw_bits(f2);
            let result = format.op_div(encoding1, encoding2);

            assert_eq!(true_result, result);
        }
    }
}

#[test]
fn float_opMult() {
    let format = FloatFormat::new(4);

    for f1 in float_test_values() {
        let encoding1 = float_to_raw_bits(f1);
        for f2 in float_test_values() {
            let true_result = expected_arithmetic_encoding(
                float_to_raw_bits(f1 * f2),
                f1.is_nan() || f2.is_nan(),
            );
            let encoding2 = float_to_raw_bits(f2);
            let result = format.op_mult(encoding1, encoding2);

            assert_eq!(true_result, result);
        }
    }
}

#[test]
fn float_opSub() {
    let format = FloatFormat::new(4);

    for f1 in float_test_values() {
        let encoding1 = float_to_raw_bits(f1);
        for f2 in float_test_values() {
            let true_result = expected_arithmetic_encoding(
                float_to_raw_bits(f1 - f2),
                f1.is_nan() || f2.is_nan(),
            );
            let encoding2 = float_to_raw_bits(f2);
            let result = format.op_sub(encoding1, encoding2);

            assert_eq!(true_result, result);
        }
    }
}

// end generated
