//! Fixed-width integer arithmetic, from `decompiler/cpp/multiprecision.{hh,cc}`.
//!
//! Public operations use two little-endian `u64` limbs. Division uses native
//! `u128` arithmetic while preserving the narrow-input path and zero-divisor
//! behavior.

use kuna_base::error::{KunaError, KunaResult};

/// Multi-precision logical left shift by a constant amount.
///
/// In C++ the `in` and `out` arrays can point to the same storage; the loop
/// orders are written so that aliasing is safe.  The Rust port takes
/// separate borrows (callers needing in-place shift copy first).
/// `num` is the number of 64-bit words in the extended precision integers;
/// `sa` is the number of bits to shift.
fn leftshift(num: i32, in_: &[u64], out: &mut [u64], sa: i32) {
    let mut in_index = num - 1 - sa / 64;
    let sa = sa % 64;
    let mut out_index = num - 1;
    if sa == 0 {
        while in_index >= 0 {
            out[out_index as usize] = in_[in_index as usize];
            out_index -= 1;
            in_index -= 1;
        }
        while out_index >= 0 {
            out[out_index as usize] = 0;
            out_index -= 1;
        }
    } else {
        while in_index > 0 {
            out[out_index as usize] =
                (in_[in_index as usize] << sa) | (in_[in_index as usize - 1] >> (64 - sa));
            out_index -= 1;
            in_index -= 1;
        }
        out[out_index as usize] = in_[0] << sa;
        out_index -= 1;
        while out_index >= 0 {
            out[out_index as usize] = 0;
            out_index -= 1;
        }
    }
}

/// 128-bit INT_LEFT operation with constant shift amount.
/// `in_` is the 128-bit value to shift (as 2 64-bit words); `out` is the
/// container for the 128-bit result; `sa` is the number of bits to shift.
pub fn leftshift128(in_: &[u64; 2], out: &mut [u64; 2], sa: i32) {
    leftshift(2, in_, out, sa);
}

fn from_limbs(value: &[u64; 2]) -> u128 {
    u128::from(value[0]) | (u128::from(value[1]) << 64)
}

fn to_limbs(value: u128) -> [u64; 2] {
    [value as u64, (value >> 64) as u64]
}

/// 128-bit INT_LESS operation: true if the first value is less than the
/// second value.
pub fn uless128(in1: &[u64; 2], in2: &[u64; 2]) -> bool {
    from_limbs(in1) < from_limbs(in2)
}

/// 128-bit INT_LESSEQUAL operation: true if the first value is less than or
/// equal to the second value.
pub fn ulessequal128(in1: &[u64; 2], in2: &[u64; 2]) -> bool {
    from_limbs(in1) <= from_limbs(in2)
}

/// 128-bit INT_ADD operation: `out` holds the wrapping sum of `in1` and `in2`.
pub fn add128(in1: &[u64; 2], in2: &[u64; 2], out: &mut [u64; 2]) {
    *out = to_limbs(from_limbs(in1).wrapping_add(from_limbs(in2)));
}

/// 128-bit INT_SUB operation: `out` holds the wrapping difference of `in1` and `in2`.
pub fn subtract128(in1: &[u64; 2], in2: &[u64; 2], out: &mut [u64; 2]) {
    *out = to_limbs(from_limbs(in1).wrapping_sub(from_limbs(in2)));
}

/// Divide two little-endian 128-bit values, writing quotient and remainder.
/// A zero divisor panics for a 64-bit numerator and returns an error otherwise.
pub fn udiv128(
    numer: &[u64; 2],
    denom: &[u64; 2],
    quotient_res: &mut [u64; 2],
    remainder_res: &mut [u64; 2],
) -> KunaResult<()> {
    if numer[1] == 0 && denom[1] == 0 {
        quotient_res[0] = numer[0] / denom[0];
        quotient_res[1] = 0;
        remainder_res[0] = numer[0] % denom[0];
        remainder_res[1] = 0;
        return Ok(());
    }
    udiv128_wide(numer, denom, quotient_res, remainder_res)
}

/// Keep wide arithmetic out of the narrow-input path.
#[inline(never)]
fn udiv128_wide(
    numer: &[u64; 2],
    denom: &[u64; 2],
    quotient_res: &mut [u64; 2],
    remainder_res: &mut [u64; 2],
) -> KunaResult<()> {
    let n = from_limbs(numer);
    let d = from_limbs(denom);
    if d == 0 {
        return Err(KunaError::lowlevel("divide by 0"));
    }
    if n < d {
        *quotient_res = [0, 0];
        *remainder_res = *numer;
        return Ok(());
    }
    let quotient = n / d;
    let remainder = n % d;
    *quotient_res = to_limbs(quotient);
    *remainder_res = to_limbs(remainder);
    Ok(())
}

/// Set a 128-bit value (2 64-bit words) from a 64-bit value.
pub fn set_u128(res: &mut [u64; 2], val: u64) {
    res[0] = val;
    res[1] = 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    // Edge tests owned by this item.  The C++ TEST(...) suite of
    // testmultiprec.cc is ported by a different item; names here are
    // deliberately distinct.

    /// Reference 128-bit arithmetic via Rust's native u128.
    fn to_u128(w: &[u64; 2]) -> u128 {
        (u128::from(w[1]) << 64) | u128::from(w[0])
    }
    fn from_u128(v: u128) -> [u64; 2] {
        [v as u64, (v >> 64) as u64]
    }

    #[test]
    fn test_edge_add_sub_carry_chains() {
        let mut out = [0u64; 2];
        // Carry across the word boundary.
        add128(&[u64::MAX, 0], &[1, 0], &mut out);
        assert_eq!(out, [0, 1]);
        // Full wraparound: max + 1 == 0.
        add128(&[u64::MAX, u64::MAX], &[1, 0], &mut out);
        assert_eq!(out, [0, 0]);
        // Borrow across the word boundary.
        subtract128(&[0, 1], &[1, 0], &mut out);
        assert_eq!(out, [u64::MAX, 0]);
        // Borrow wraps below zero: 0 - 1 == max.
        subtract128(&[0, 0], &[1, 0], &mut out);
        assert_eq!(out, [u64::MAX, u64::MAX]);
    }

    #[test]
    fn test_edge_leftshift_boundaries() {
        let v = [0x8000_0000_0000_0001u64, 0x1];
        let mut out = [0u64; 2];
        leftshift128(&v, &mut out, 0);
        assert_eq!(out, v);
        leftshift128(&v, &mut out, 1);
        assert_eq!(out, from_u128(to_u128(&v) << 1));
        leftshift128(&v, &mut out, 63);
        assert_eq!(out, from_u128(to_u128(&v) << 63));
        leftshift128(&v, &mut out, 64);
        assert_eq!(out, from_u128(to_u128(&v) << 64));
        leftshift128(&v, &mut out, 65);
        assert_eq!(out, from_u128(to_u128(&v) << 65));
        leftshift128(&v, &mut out, 127);
        assert_eq!(out, [0, 0x8000_0000_0000_0000]);
    }

    #[test]
    fn test_edge_compare_word_order() {
        // High word dominates.
        assert!(uless128(&[u64::MAX, 0], &[0, 1]));
        assert!(!uless128(&[0, 1], &[u64::MAX, 0]));
        // Equal values.
        let a = [0xdead_beefu64, 0x1234_5678u64];
        assert!(!uless128(&a, &a));
        assert!(ulessequal128(&a, &a));
        // Low word decides when high words match.
        assert!(uless128(&[1, 5], &[2, 5]));
        assert!(ulessequal128(&[1, 5], &[2, 5]));
        assert!(!ulessequal128(&[3, 5], &[2, 5]));
    }

    #[test]
    fn test_edge_udiv_paths_against_u128() {
        // Narrow and wide inputs, smaller numerators and quotient boundaries.
        let cases: [([u64; 2], [u64; 2]); 9] = [
            ([12345, 0], [7, 0]),                                  // fast path
            ([0x1234_5678_9abc_def0, 5], [3, 0]),                  // n == 1
            ([7, 0], [0, 1]),                                      // quotient 0
            ([u64::MAX, u64::MAX], [0, 1]),                        // power-of-two high word
            ([0x89a7_32a9_fb15_7c4d, 0x4ead_a203_9e48_443e], [0xbabf_3b71, 1]), // n == 3
            ([u64::MAX, u64::MAX], [u64::MAX, 0x7fff_ffff_ffff_ffff]), // m == n+? close magnitudes
            ([0, 0x8000_0000_0000_0000], [1, 0x4000_0000_0000_0000]), // add-back prone shape
            ([u64::MAX, u64::MAX], [2, u64::MAX >> 1]),            // qhat correction loop
            ([0xf7df_0315_d584_ad8d, 0xb9d5_5c0d_1d5c_fbbd], [0x8aa7_97db_ccee_6e96, 0x64_6be9]),
        ];
        for (numer, denom) in cases {
            let mut q = [0u64; 2];
            let mut r = [0u64; 2];
            udiv128(&numer, &denom, &mut q, &mut r).unwrap();
            let nn = to_u128(&numer);
            let dd = to_u128(&denom);
            assert_eq!(to_u128(&q), nn / dd, "quotient of {nn:#x} / {dd:#x}");
            assert_eq!(to_u128(&r), nn % dd, "remainder of {nn:#x} % {dd:#x}");
        }
    }

    #[test]
    fn test_edge_udiv_divide_by_zero_error() {
        // The multi-digit path reports the C++ LowlevelError("divide by 0").
        let mut q = [0u64; 2];
        let mut r = [0u64; 2];
        let err = udiv128(&[0, 1], &[0, 0], &mut q, &mut r).unwrap_err();
        assert_eq!(err, KunaError::lowlevel("divide by 0"));
        assert!(err.is_lowlevel());
    }

    #[test]
    fn test_edge_set_u128() {
        let mut res = [0xffu64, 0xffu64];
        set_u128(&mut res, 0xdead_beef_dead_beef);
        assert_eq!(res, [0xdead_beef_dead_beef, 0]);
    }

    #[test]
    fn test_edge_count_leading_zeros() {
        use kuna_base::address::count_leading_zeros;

        assert_eq!(count_leading_zeros(0), 64);
        assert_eq!(count_leading_zeros(1), 63);
        assert_eq!(count_leading_zeros(u64::MAX), 0);
        assert_eq!(count_leading_zeros(0x8000_0000_0000_0000), 0);
        assert_eq!(count_leading_zeros(0x1_0000_0000), 31);
        assert_eq!(count_leading_zeros(0xffff_ffff), 32);
        for bit in 0..64u32 {
            assert_eq!(count_leading_zeros(1u64 << bit), 63 - bit as i32);
        }
    }
}
