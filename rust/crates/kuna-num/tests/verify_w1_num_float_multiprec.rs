//! Verifier adversarial tests for item w1-num-float-multiprec
//! (round 1 review; see docs/rust-port/reviews/w1-num-float-multiprec.md).
//!
//! Oracle values were produced by linking the REAL vendored
//! `decompiler/cpp/float.cc` (g++ -O2, x86-64 glibc host) into a standalone
//! harness -- not by re-deriving them on paper.  Multiprecision oracle
//! values come from Rust's native u128 arithmetic.

use kuna_base::error::KunaError;
use kuna_num::float::{floatclass, FloatFormat};
use kuna_num::multiprecision::{add128, leftshift128, subtract128, udiv128, uless128};

fn to_u128(w: &[u64; 2]) -> u128 {
    (u128::from(w[1]) << 64) | u128::from(w[0])
}

/// FINDING F1 (this test FAILS in a debug build -- the REJECT evidence).
///
/// C++ float.cc:334-344 (getEncoding) / 407-417 (convertEncoding): in the
/// denormal path, a round-to-nearest-even carry out of bit 63 sets
/// `signif = 1<<63; exp += 1`.  When exp was 0 (host value at/just below the
/// target's smallest normal), exp becomes 1 and the subsequent
/// `signif >> (-exp)` shifts by -1: C++ UB that on the x86-64 oracle masks
/// the count to 63, so the fraction code collapses to 0 and the function
/// returns the (signed) zero encoding.  The C++ oracle harness confirms:
///
///   getEncoding(fmt4, f64::from_bits(0x380fffffffffffff)) == 0x0
///   getEncoding(fmt4, -same)                              == 0x80000000
///   convertEncoding(fmt4<-fmt8, 0x380fffffffffffff)       == 0x0
///
/// The Rust port executes the same `signif >> (-exp)` (float.rs:513 / :592),
/// which PANICS in debug ("attempt to shift right with overflow") and only
/// matches the oracle in release by the accident of LLVM shift masking --
/// a debug/release divergence ADR 0003 explicitly forbids.
#[test]
fn verify_denormal_carry_to_normal_matches_cpp_oracle() {
    let fmt4 = FloatFormat::new(4);
    let fmt8 = FloatFormat::new(8);
    // (1 - 2^-53) * 2^-126: all double mantissa bits set, just below the f32
    // smallest normal.
    let b1 = 0x380f_ffff_ffff_ffffu64;
    // (1 - 2^-24) * 2^-126 == 2^-126 - 2^-150: the exact tie between the f32
    // max denormal and min normal.
    let b3 = 0x380f_ffff_e000_0000u64;
    assert_eq!(fmt4.get_encoding(f64::from_bits(b1)), 0x0);
    assert_eq!(fmt4.get_encoding(-f64::from_bits(b1)), 0x8000_0000);
    assert_eq!(fmt4.get_encoding(f64::from_bits(b3)), 0x0);
    // Same cell through FLOAT_FLOAT2FLOAT constant folding (8 -> 4).
    assert_eq!(fmt4.convert_encoding(b1, &fmt8), 0x0);
    assert_eq!(fmt8.op_float2float(b1, &fmt4), 0x0);
}

/// Denormal rounding cells AWAY from the broken carry boundary, pinned to
/// the C++ oracle harness output (these pass and fence the fix).
#[test]
fn verify_denormal_rounding_grid_oracle() {
    let fmt4 = FloatFormat::new(4);
    let fmt8 = FloatFormat::new(8);
    let cells: [(u64, u64); 7] = [
        (0x36b0_0000_0000_0000, 0x2), // 2^-148 = 2 * min_denorm
        (0x36b8_0000_0000_0000, 0x3), // 1.5 * 2^-148 -> 3 * min_denorm
        (0x36b7_0000_0000_0000, 0x3), // 2.875 * 2^-149 rounds up to 3
        (0x36a0_0000_0000_0000, 0x1), // 2^-149 exactly = min_denorm
        (0x36a0_0000_2000_0000, 0x1), // 2^-149 * (1 + eps)
        (0x36af_ffff_f000_0000, 0x2), // just below 2^-148 rounds up
        (0x3690_0000_0000_0000, 0x0), // 2^-150: tie with zero -> even -> 0
    ];
    for (bits, expect) in cells {
        assert_eq!(
            fmt4.get_encoding(f64::from_bits(bits)),
            expect,
            "get_encoding(fmt4, {bits:#018x})"
        );
        // The same value via convertEncoding 8->4 (lowbitpos == 64 cell for
        // the last row: lowbitmask must drop to 0, not shift by 64).
        assert_eq!(
            fmt4.convert_encoding(bits, &fmt8),
            expect,
            "convert_encoding(fmt4<-fmt8, {bits:#018x})"
        );
    }
}

/// udiv128 driven through the Knuth add-back (`t < 0`) and qhat-correction
/// paths, the riskiest signedness/wrap cluster of multiprecision.cc
/// (uint4 -> uint64 promotions, int8 reinterpretation, arithmetic-shift
/// carries).  Trigger inputs were located by exhaustively instrumenting a
/// transcription of the algorithm; results checked against native u128.
#[test]
fn verify_udiv128_addback_and_qhat_correction() {
    let addback: [([u64; 2], [u64; 2]); 6] = [
        ([0x7fff_ffff_7689_1ccc, 0x8000_0000_7140_bf7e], [0xe281_7efd, 0x1]),
        ([0x7fff_ffff_6964_999e, 0x8000_0000_02b1_402c], [0x0562_8059, 0x1]),
        ([0x0f84_4fee_4593_cb55, 0x8000_0000_0000_6712], [0x6712_0f84_4fef, 0x8000_0000]),
        ([0x7fff_ffff_d85f_3c29, 0x8000_0000_3e4b_3fbc], [0x7c96_7f79, 0x1]),
        ([0x71e2_1b2b_6499_4a57, 0x8000_0000], [0xe3c4_3657, 0x1]),
        ([0x0b5c_ea69_905e_88cb, 0x8000_0000_0000_b572], [0xb572_0b5c_ea6a, 0x8000_0000]),
    ];
    let qhat_correction: [([u64; 2], [u64; 2]); 4] = [
        ([0x96da_1dab_b8c8_7e70, 0x8000_0000], [0x8000_0000_96da_1dac, 0x0]),
        ([0x7fff_ffff_f4e0_6e9d, 0x8000_674f], [0x1_0000_ce9f, 0x0]),
        ([0xffff_ffff_2509_e5da, 0x8000_0000_146d_3f30], [0x8000_0000_146d_3f31, 0x0]),
        ([0x445e_b202_c557_f22c, 0x8000_0000], [0x88bd_6407, 0x1]),
    ];
    for (numer, denom) in addback.iter().chain(qhat_correction.iter()) {
        let mut q = [0u64; 2];
        let mut r = [0u64; 2];
        udiv128(numer, denom, &mut q, &mut r).unwrap();
        let nn = to_u128(numer);
        let dd = to_u128(denom);
        assert_eq!(to_u128(&q), nn / dd, "quotient {nn:#x} / {dd:#x}");
        assert_eq!(to_u128(&r), nn % dd, "remainder {nn:#x} % {dd:#x}");
    }
}

/// Randomized differential sweep of the 128-bit primitives against native
/// u128 (deterministic LCG, no external state).
#[test]
fn verify_mp_random_differential_u128() {
    let mut state = 0xdead_beef_cafe_f00du64;
    let mut next = move || {
        state = state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        state
    };
    for i in 0..20_000 {
        let a = [next(), next() >> (next() % 64)];
        let b = [next(), next() >> (next() % 64)];
        let (aa, bb) = (to_u128(&a), to_u128(&b));
        let mut out = [0u64; 2];
        add128(&a, &b, &mut out);
        assert_eq!(to_u128(&out), aa.wrapping_add(bb), "add #{i}");
        subtract128(&a, &b, &mut out);
        assert_eq!(to_u128(&out), aa.wrapping_sub(bb), "sub #{i}");
        assert_eq!(uless128(&a, &b), aa < bb, "less #{i}");
        let sa = (next() % 128) as i32;
        leftshift128(&a, &mut out, sa);
        assert_eq!(to_u128(&out), aa << sa, "shl #{i} by {sa}");
        if bb != 0 && aa >= bb {
            let mut q = [0u64; 2];
            let mut r = [0u64; 2];
            udiv128(&a, &b, &mut q, &mut r).unwrap();
            assert_eq!(to_u128(&q), aa / bb, "div #{i}");
            assert_eq!(to_u128(&r), aa % bb, "rem #{i}");
        }
    }
    // Error-path parity: 128-bit-wide numerator over zero denominator is the
    // C++ LowlevelError("divide by 0") (the 64-bit fast path is a host trap).
    let mut q = [0u64; 2];
    let mut r = [0u64; 2];
    assert_eq!(
        udiv128(&[1, 1], &[0, 0], &mut q, &mut r).unwrap_err(),
        KunaError::lowlevel("divide by 0")
    );
}

/// printDecimal cells pinned from the C++ float.cc oracle harness (the
/// golden CSV's `dec` rows only cover ostream default precision, not the
/// precision ladder).
#[test]
fn verify_print_decimal_cpp_oracle_cells() {
    let fmt4 = FloatFormat::new(4);
    let fmt8 = FloatFormat::new(8);
    // f32 2^-148 (denormal): host double is exact.
    let twe_m148 = f64::from_bits(0x36b0_0000_0000_0000);
    assert_eq!(fmt4.print_decimal(twe_m148, true), "2.80260e-45");
    assert_eq!(fmt4.print_decimal(twe_m148, false), "2.8026e-45");
    // f64 min denormal needs the full 15-digit start of the ladder.
    assert_eq!(
        fmt8.print_decimal(f64::from_bits(1), false),
        "4.94065645841247e-324"
    );
    assert_eq!(fmt8.print_decimal(1e22, false), "1e+22");
    assert_eq!(fmt8.print_decimal(123456789.0, false), "123456789");
    // 2^53 + 2: 15 digits do not roundtrip, 16 do (fixed style, X < P).
    assert_eq!(
        fmt8.print_decimal(9007199254740994.0, false),
        "9007199254740994"
    );
}

/// opTrunc boundary cells pinned from the C++ oracle harness (cvttsd2si
/// semantics: NaN/inf/out-of-range -> 0x8000000000000000; the largest double
/// below 2^63 is in range).
#[test]
fn verify_trunc_boundary_cpp_oracle_cells() {
    let fmt8 = FloatFormat::new(8);
    let cells: [(u64, u64); 8] = [
        (0x43df_ffff_ffff_ffff, 0x7fff_ffff_ffff_fc00), // 2^63 - 1024
        (0x43e0_0000_0000_0000, 0x8000_0000_0000_0000), // 2^63 (out of range)
        (0xc3e0_0000_0000_0000, 0x8000_0000_0000_0000), // -2^63 (in range!)
        (0xc3e0_0000_0000_0001, 0x8000_0000_0000_0000), // below -2^63
        (0x4340_0000_0000_0001, 0x0020_0000_0000_0002), // 2^53 + 2
        (0xbfe0_0000_0000_0000, 0x0),                   // -0.5 truncates to 0
        (0x3fe0_0000_0000_0000, 0x0),                   // 0.5 truncates to 0
        (0xc024_0000_0000_0000, 0xffff_ffff_ffff_fff6), // -10.0
    ];
    for (enc, expect) in cells {
        assert_eq!(fmt8.op_trunc(enc, 8), expect, "op_trunc({enc:#018x}, 8)");
    }
    assert_eq!(fmt8.op_trunc(0xc024_0000_0000_0000, 2), 0xfff6);
    // floatclass plumbing on the same encodings.
    assert_eq!(fmt8.get_class(0x43e0_0000_0000_0000), floatclass::normalized);
}
