//! Port of `decompiler/cpp/multiprecision.{hh,cc}` -- multi-precision
//! integers (item w1-num-float-multiprec).
//!
//! The C++ passes 128-bit values as `uint8*` arrays of 2 little-endian
//! 64-bit words (word 0 = least significant); the port uses `&[u64; 2]`
//! references with the same word order.  The internal helpers are generic
//! over `num` words like the C++ statics, even though only `num == 2` is
//! exercised today.
//!
//! Integer semantics follow ADR 0003: arithmetic that can legitimately wrap
//! uses the `Wrap` helpers; in-range index arithmetic stays on `i32` like
//! the C++ `int4` loops.

use kuna_base::error::{KunaError, KunaResult};
use kuna_base::types::Wrap;

/// `count_leading_zeros(uintb val)` (address.cc): number of leading zero
/// bits, by the upstream mask-halving loop (returns 64 for zero).  The C++
/// file re-declares this itself as an `extern`; the canonical public port
/// belongs to the kuna-base address module (a different port item), so a
/// private transcription lives here too.
fn count_leading_zeros(val: u64) -> i32 {
    if val == 0 {
        return 8 * 8; // 8*sizeof(uintb)
    }
    let mut mask: u64 = !0u64;
    let mut mask_size: i32 = 4 * 8; // 4*sizeof(uintb)
    mask &= mask << mask_size;
    let mut bit = 0;
    loop {
        if (mask & val) == 0 {
            bit += mask_size;
            mask_size >>= 1;
            mask |= mask >> mask_size;
        } else {
            mask_size >>= 1;
            mask &= mask << mask_size;
        }
        if mask_size == 0 {
            break;
        }
    }
    bit
}

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

/// Compare two multi-precision unsigned integers.
///
/// -1, 0, or 1 is returned depending on if the first integer is less than,
/// equal to, or greater than the second integer.
fn ucompare(num: i32, in1: &[u64], in2: &[u64]) -> i32 {
    let mut i = num - 1;
    while i >= 0 {
        if in1[i as usize] != in2[i as usize] {
            return if in1[i as usize] < in2[i as usize] { -1 } else { 1 };
        }
        i -= 1;
    }
    0
}

/// 128-bit INT_LESS operation: true if the first value is less than the
/// second value.
pub fn uless128(in1: &[u64; 2], in2: &[u64; 2]) -> bool {
    ucompare(2, in1, in2) < 0
}

/// 128-bit INT_LESSEQUAL operation: true if the first value is less than or
/// equal to the second value.
pub fn ulessequal128(in1: &[u64; 2], in2: &[u64; 2]) -> bool {
    ucompare(2, in1, in2) <= 0
}

/// Multi-precision add operation: `out = in1 + in2` over `num` words.
fn add(num: i32, in1: &[u64], in2: &[u64], out: &mut [u64]) {
    let mut carry: u64 = 0;
    for i in 0..num {
        let i = i as usize;
        let tmp = in2[i].wadd(carry);
        let tmp2 = in1[i].wadd(tmp);
        out[i] = tmp2;
        carry = u64::from(tmp < in2[i] || tmp2 < tmp);
    }
}

/// 128-bit INT_ADD operation: `out` holds the 128-bit result of
/// `in1 + in2` (each as 2 64-bit words).
pub fn add128(in1: &[u64; 2], in2: &[u64; 2], out: &mut [u64; 2]) {
    add(2, in1, in2, out);
}

/// Multi-precision subtract operation: `out = in1 - in2` over `num` words.
fn subtract(num: i32, in1: &[u64], in2: &[u64], out: &mut [u64]) {
    let mut borrow: u64 = 0;
    for i in 0..num {
        let i = i as usize;
        let tmp = in2[i].wadd(borrow);
        borrow = u64::from(tmp < in2[i] || in1[i] < tmp);
        out[i] = in1[i].wsub(tmp);
    }
}

/// 128-bit INT_SUB operation: `out` holds the 128-bit result of
/// `in1 - in2` (each as 2 64-bit words).
pub fn subtract128(in1: &[u64; 2], in2: &[u64; 2], out: &mut [u64; 2]) {
    subtract(2, in1, in2, out);
}

/// Split an array of 64-bit words into an array of 32-bit words.
///
/// The least significant half of each 64-bit word is put into the 32-bit
/// word array first.  The index of the most significant non-zero 32-bit
/// word is calculated and returned as the *effective size* of the resulting
/// array.
fn split64_32(num: i32, val: &[u64], res: &mut [u32]) -> i32 {
    let mut m: i32 = 0;
    for i in 0..num {
        let iu = i as usize;
        let hi: u32 = (val[iu] >> 32) as u32;
        let lo: u32 = (val[iu] & 0xffffffff) as u32;
        if hi != 0 {
            m = i * 2 + 2;
        } else if lo != 0 {
            m = i * 2 + 1;
        }
        res[iu * 2] = lo;
        res[iu * 2 + 1] = hi;
    }
    m
}

/// Pack an array of 32-bit words into an array of 64-bit words.
///
/// The 64-bit word array is padded out with zeroes if the specified size
/// (`num` 64-bit words) exceeds the provided number of 32-bit words (`max`).
fn pack32_64(num: i32, max: i32, out: &mut [u64], in_: &[u32]) {
    let mut j = num * 2 - 1;
    let mut i = num - 1;
    while i >= 0 {
        let mut val: u64 = if j < max { u64::from(in_[j as usize]) } else { 0 };
        val <<= 32;
        j -= 1;
        if j < max {
            val |= u64::from(in_[j as usize]);
        }
        j -= 1;
        out[i as usize] = val;
        i -= 1;
    }
}

/// Logical shift left for an extended integer in 32-bit word arrays.
/// `size` is the number of words in the array; `sa` the number of bits
/// (0 <= sa < 32) to shift.
fn shift_left(arr: &mut [u32], size: i32, sa: i32) {
    if sa == 0 {
        return;
    }
    let mut i = size - 1;
    while i > 0 {
        arr[i as usize] = (arr[i as usize] << sa) | (arr[i as usize - 1] >> (32 - sa));
        i -= 1;
    }
    arr[0] <<= sa;
}

/// Logical shift right for an extended integer in 32-bit word arrays.
/// `size` is the number of words in the array; `sa` the number of bits
/// (0 <= sa < 32) to shift.
fn shift_right(arr: &mut [u32], size: i32, sa: i32) {
    if sa == 0 {
        return;
    }
    for i in 0..size - 1 {
        let iu = i as usize;
        arr[iu] = (arr[iu] >> sa) | (arr[iu + 1] << (32 - sa));
    }
    arr[(size - 1) as usize] >>= sa;
}

/// Knuth's algorithm d, for integer division.
///
/// The numerator and denominator, expressed in 32-bit *digits*, are
/// provided.  The algorithm calculates the quotient and the remainder is
/// left in the array originally containing the numerator.  We assume
/// m > n > 0 and u[n-1] >= v[n-1] > 0.
/// `m` is the number of 32-bit digits in the numerator; `n` the number of
/// 32-bit digits in the denominator; `u` is the numerator and will hold the
/// remainder; `v` is the denominator; `q` will hold the final quotient.
fn knuth_algorithm_d(m: i32, n: i32, u: &mut [u32], v: &mut [u32], q: &mut [u32]) {
    // count_leading_zeros takes a uintb: the uint4 digit arrives
    // zero-extended, and 8*(sizeof(uintb)-sizeof(uint4)) == 32 removes the
    // high-word zeros from the count.
    let s = count_leading_zeros(u64::from(v[(n - 1) as usize])) - 32;
    shift_left(v, n, s);
    shift_left(u, m, s);

    let mut j = m - n - 1;
    while j >= 0 {
        let ju = j as usize;
        let nu = n as usize;
        let mut tmp: u64 = (u64::from(u[nu + ju]) << 32) + u64::from(u[nu - 1 + ju]);
        let mut qhat: u64 = tmp / u64::from(v[nu - 1]);
        let mut rhat: u64 = tmp % u64::from(v[nu - 1]);
        loop {
            // qhat*v[n-2] is guarded by qhat <= 0xffffffff (short-circuit),
            // so the 64-bit product cannot wrap.
            if qhat <= 0xffffffff
                && qhat * u64::from(v[nu - 2]) <= (rhat << 32) + u64::from(u[nu - 2 + ju])
            {
                break;
            }
            qhat -= 1;
            rhat += u64::from(v[nu - 1]);
            if rhat > 0xffffffff {
                break;
            }
        }

        let mut carry: u64 = 0;
        let mut t: i64;
        for i in 0..n {
            let iu = i as usize;
            // qhat <= 2^32+1 after the loop above, so qhat*v[i] fits in u64;
            // C++ computes it as a (wrapping) uint64 multiply regardless.
            tmp = qhat.wmul(u64::from(v[iu]));
            // C++: t = u[i+j] - carry - (tmp & 0xffffffff), evaluated in
            // wrapping uint64 arithmetic, then converted to int8 (wraps).
            t = u64::from(u[iu + ju]).wsub(carry).wsub(tmp & 0xffffffff) as i64;
            u[iu + ju] = t as u32; // truncating uint4 store
                                   // C++: carry = (tmp >> 32) - (t >> 32); t >> 32 is an arithmetic
                                   // shift on int8, converted back to uint64 for the subtraction.
            carry = (tmp >> 32).wsub((t >> 32) as u64);
        }
        t = u64::from(u[ju + nu]).wsub(carry) as i64;
        u[ju + nu] = t as u32;

        q[ju] = qhat as u32; // truncating uint4 store
        if t < 0 {
            q[ju] = q[ju].wsub(1);
            carry = 0;
            for i in 0..n {
                let iu = i as usize;
                // C++: tmp = u[i+j] + (v[i] + carry) in uint64.
                tmp = u64::from(u[iu + ju]).wadd(u64::from(v[iu]).wadd(carry));
                u[iu + ju] = tmp as u32; // truncating uint4 store
                carry = tmp >> 32;
            }
            // C++: u[j+n] += carry; the uint4 addition truncates.  carry is
            // 0 or 1 here, so adding the truncated carry is identical.
            u[ju + nu] = u[ju + nu].wadd(carry as u32);
        }
        j -= 1;
    }
    shift_right(u, m, s);
}

/// 128-bit INT_DIV: divide `numer` by `denom` (each 2 64-bit words),
/// producing the quotient and remainder.
///
/// Returns the C++ `LowlevelError("divide by 0")` when the full 128-bit
/// denominator is zero on the multi-digit path.
pub fn udiv128(
    numer: &[u64; 2],
    denom: &[u64; 2],
    quotient_res: &mut [u64; 2],
    remainder_res: &mut [u64; 2],
) -> KunaResult<()> {
    if numer[1] == 0 && denom[1] == 0 {
        // (kuna) C++ divides directly here; a zero denom[0] is a hardware
        // trap (SIGFPE) in the oracle, i.e. an internal invariant violation
        // -- the Rust division panic is the analogous failure (ADR 0004).
        quotient_res[0] = numer[0] / denom[0];
        quotient_res[1] = 0;
        remainder_res[0] = numer[0] % denom[0];
        remainder_res[1] = 0;
        return Ok(());
    }
    let mut v = [0u32; 4];
    let mut u = [0u32; 5]; // Array needs one more entry for normalization overflow
    let mut q = [0u32; 4];
    let n = split64_32(2, denom, &mut v);
    if n == 0 {
        return Err(KunaError::lowlevel("divide by 0"));
    }
    let mut m = split64_32(2, numer, &mut u);
    if m < n || ((n == m) && u[(n - 1) as usize] < v[(n - 1) as usize]) {
        // denominator is smaller than the numerator, quotient is 0
        quotient_res[0] = 0;
        quotient_res[1] = 0;
        remainder_res[0] = numer[0];
        remainder_res[1] = numer[1];
        return Ok(());
    }
    if n == 1 {
        let d = v[0];
        let mut rem: u32 = 0;
        let mut i = m - 1;
        while i >= 0 {
            let iu = i as usize;
            let tmp: u64 = (u64::from(rem) << 32) + u64::from(u[iu]);
            // tmp/d < 2^32 because rem < d, so the uint4 store is exact.
            q[iu] = (tmp / u64::from(d)) as u32;
            u[iu] = 0;
            rem = (tmp % u64::from(d)) as u32;
            i -= 1;
        }
        u[0] = rem; // Last carry is final remainder
    } else {
        u[m as usize] = 0;
        m += 1; // Temporarily extend u array by 1 to allow for normalization
        knuth_algorithm_d(m, n, &mut u, &mut v, &mut q);
        m -= 1; // Remove the extension
    }
    pack32_64(2, m - n + 1, quotient_res, &q);
    pack32_64(2, m, remainder_res, &u);
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
        // Each (numer, denom) pair drives a distinct udiv128 code path:
        // fast 64/64, n==1 with 128-bit numerator, quotient-zero, and the
        // full Knuth path (n in {2,3,4}), including an add-back candidate.
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
    fn test_edge_count_leading_zeros_loop() {
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
