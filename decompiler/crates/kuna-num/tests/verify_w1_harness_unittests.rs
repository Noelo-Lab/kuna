//! Verifier adversarial tests for item `w1-harness-unittests`
//! (testmultiprec half, `decompiler/unittests/testmultiprec.cc`).
//!
//! The ported `multiprec_udiv` test zero-initializes its `q`/`r` output
//! arrays (the C++ declares them uninitialized, testmultiprec.cc:31-32).
//! Zero-init can MASK a port bug the C++ test would (nondeterministically)
//! catch: a `udiv128` that fails to write an output limb whose expected
//! value is 0 (q[1] in three of the four C++ cases, r[1] in two) would
//! still pass the ported test.  Here every output limb is pre-filled with
//! a sentinel, so each expected limb value — including the zeros — is
//! proven to be *written* by `udiv128`, per the C++ contract that
//! `quotient_res`/`remainder_res` "will hold the 2 words"
//! (multiprecision.cc:284-287; pack32_64 zero-fills the tail).

use kuna_num::multiprecision::udiv128;

const SENTINEL: u64 = 0xdead_beef_dead_beef;

/// (numerator, denominator, expected quotient, expected remainder)
type UdivCase = ([u64; 2], [u64; 2], [u64; 2], [u64; 2]);

// The four C++ udiv cells (testmultiprec.cc:21-26, 30-57), exercising the
// 64-bit fast path is not among them; these hit: full-width quotient,
// n==1 single-word divisor, numerator < denominator early return, and the
// Knuth m>n path with a 2-word remainder.
const CASES: [UdivCase; 4] = [
    (
        [0xffffffffffffffff, 0xffffffffffffffff], // num1
        [1, 0],                                   // denom1
        [0xffffffffffffffff, 0xffffffffffffffff], // q
        [0, 0],                                   // r
    ),
    (
        [0x89a732a9fb157c4d, 0x4eada2039e48443e], // num2
        [0xbabf3b71, 0],                          // denom2
        [0x2a21eef2058d7e9a, 0x6bdaed99],
        [0x928d1c53, 0],
    ),
    (
        [0x89a732a9fb157c4d, 0x4eada2039e48443e], // num2
        [0xffffffffffffffff, 0xffffffffffffffff], // num1 as denominator
        [0, 0],
        [0x89a732a9fb157c4d, 0x4eada2039e48443e],
    ),
    (
        [0xf7df0315d584ad8d, 0xb9d55c0d1d5cfbbd], // num3
        [0x8aa797dbccee6e96, 0x646be9],           // denom3
        [0x1d9bc949e24, 0],
        [0x2e78197dc5048c75, 0x24d9cc],
    ),
];

#[test]
fn verify_udiv128_writes_every_output_limb() {
    for (i, (num, den, expect_q, expect_r)) in CASES.iter().enumerate() {
        let mut q: [u64; 2] = [SENTINEL; 2];
        let mut r: [u64; 2] = [SENTINEL; 2];
        udiv128(num, den, &mut q, &mut r).unwrap();
        assert_eq!(q, *expect_q, "quotient limbs, case {i}");
        assert_eq!(r, *expect_r, "remainder limbs, case {i}");
        assert_ne!(q[1], SENTINEL, "q[1] left unwritten, case {i}");
        assert_ne!(r[1], SENTINEL, "r[1] left unwritten, case {i}");
    }
}
