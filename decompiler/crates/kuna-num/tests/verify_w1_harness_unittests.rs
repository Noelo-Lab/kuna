//! Division output-state checks, including cells from upstream
//! `decompiler/unittests/testmultiprec.cc`. Sentinel-filled outputs verify
//! that successful calls write every limb and failures leave them untouched.

use kuna_num::multiprecision::udiv128;

const SENTINEL: u64 = 0xdead_beef_dead_beef;

/// (numerator, denominator, expected quotient, expected remainder)
type UdivCase = ([u64; 2], [u64; 2], [u64; 2], [u64; 2]);

// Upstream testmultiprec.cc:21-26, 30-57.
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

#[test]
fn zero_division_preserves_outputs_on_both_failure_paths() {
    let quotient = [0xaabb_ccdd_eeff_0011, 0x1122_3344_5566_7788];
    let remainder = [0xdead_beef, 0xfedc_ba98];
    for numer in [[0, 0], [1, 0], [u64::MAX, 0], [0, 1], [u64::MAX; 2]] {
        let (mut q, mut r) = (quotient, remainder);
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            udiv128(&numer, &[0, 0], &mut q, &mut r)
        }));
        if numer[1] == 0 {
            let payload = result.expect_err("narrow zero division must panic");
            let message = payload.downcast_ref::<String>().map(String::as_str)
                .or_else(|| payload.downcast_ref::<&str>().copied());
            assert_eq!(message, Some("attempt to divide by zero"));
        } else {
            assert_eq!(
                result.expect("wide zero division must return an error"),
                Err(kuna_base::error::KunaError::lowlevel("divide by 0"))
            );
        }
        assert_eq!(q, quotient);
        assert_eq!(r, remainder);
    }
}

#[test]
fn zero_equal_and_smaller_division_write_every_output_limb() {
    let cases: [UdivCase; 6] = [
        ([0, 0], [1, 0], [0, 0], [0, 0]),
        ([0, 0], [0, 1], [0, 0], [0, 0]),
        ([u64::MAX, 0], [u64::MAX, 0], [1, 0], [0, 0]),
        ([u64::MAX; 2], [u64::MAX; 2], [1, 0], [0, 0]),
        ([1, 1], [2, 1], [0, 0], [1, 1]),
        ([u64::MAX, 0], [0, 1], [0, 0], [u64::MAX, 0]),
    ];
    for (numer, denom, quotient, remainder) in cases {
        let (mut q, mut r) = ([SENTINEL; 2], [SENTINEL; 2]);
        udiv128(&numer, &denom, &mut q, &mut r).unwrap();
        assert_eq!(q, quotient);
        assert_eq!(r, remainder);
    }
}
