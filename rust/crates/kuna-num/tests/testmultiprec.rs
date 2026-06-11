//! Port of the C++ unit tests in `decompiler/unittests/testmultiprec.cc`.
//!
//! One C++ `TEST(name)` -> one `#[test] fn name()`, assertion for
//! assertion, same constants, same order.
//!
//! The C++ file-static `uint8 name[2]` vectors (testmultiprec.cc:21-28)
//! become `const` arrays (renamed to UPPER_CASE per Rust const naming;
//! values unchanged).  C++ passes the raw arrays as pointers; the Rust API
//! takes `&[u64; 2]` / `&mut [u64; 2]`, and `udiv128` surfaces the C++
//! `LowlevelError("divide by 0")` as a `Result` -- none of these inputs
//! divide by zero, so each call site `.unwrap()`s.

use kuna_num::multiprecision::{
    add128, leftshift128, subtract128, udiv128, uless128, ulessequal128,
};

// C++ statics num1/denom1/num2/denom2/num3/denom3/a/b (testmultiprec.cc:21-28)
const NUM1: [u64; 2] = [0xffffffffffffffff, 0xffffffffffffffff];
const DENOM1: [u64; 2] = [1, 0];
const NUM2: [u64; 2] = [0x89a732a9fb157c4d, 0x4eada2039e48443e];
const DENOM2: [u64; 2] = [0xbabf3b71, 0];
const NUM3: [u64; 2] = [0xf7df0315d584ad8d, 0xb9d55c0d1d5cfbbd];
const DENOM3: [u64; 2] = [0x8aa797dbccee6e96, 0x646be9];
const A: [u64; 2] = [0x1a309a9df2ce836a, 0xd66f2248906d1bdf];
const B: [u64; 2] = [0xf6c190704eb1763e, 0xa05c42212dfba7c6];

#[test]
fn multiprec_udiv() {
    // C++ declares `uint8 q[2]; uint8 r[2];` uninitialized; Rust zero-fills.
    let mut q: [u64; 2] = [0; 2];
    let mut r: [u64; 2] = [0; 2];

    udiv128(&NUM1, &DENOM1, &mut q, &mut r).unwrap();
    assert_eq!(q[0], 0xffffffffffffffff);
    assert_eq!(q[1], 0xffffffffffffffff);
    assert_eq!(r[0], 0);
    assert_eq!(r[1], 0);

    udiv128(&NUM2, &DENOM2, &mut q, &mut r).unwrap();
    assert_eq!(q[0], 0x2a21eef2058d7e9a);
    assert_eq!(q[1], 0x6bdaed99);
    assert_eq!(r[0], 0x928d1c53);
    assert_eq!(r[1], 0);

    udiv128(&NUM2, &NUM1, &mut q, &mut r).unwrap();
    assert_eq!(q[0], 0);
    assert_eq!(q[1], 0);
    assert_eq!(r[0], NUM2[0]);
    assert_eq!(r[1], NUM2[1]);

    udiv128(&NUM3, &DENOM3, &mut q, &mut r).unwrap();
    assert_eq!(q[0], 0x1d9bc949e24);
    assert_eq!(q[1], 0);
    assert_eq!(r[0], 0x2e78197dc5048c75);
    assert_eq!(r[1], 0x24d9cc);
}

#[test]
fn multiprec_add() {
    let mut res: [u64; 2] = [0; 2];
    add128(&A, &B, &mut res);
    assert_eq!(res[0], 0x10f22b0e417ff9a8);
    assert_eq!(res[1], 0x76cb6469be68c3a6);
}

#[test]
fn multiprec_sub() {
    let mut res: [u64; 2] = [0; 2];
    subtract128(&A, &B, &mut res);
    assert_eq!(res[0], 0x236F0A2DA41D0D2C);
    assert_eq!(res[1], 0x3612E02762717418);
}

#[test]
fn multiprec_left() {
    let mut res: [u64; 2] = [0; 2];
    leftshift128(&NUM2, &mut res, 51);
    assert_eq!(res[0], 0xe268000000000000);
    assert_eq!(res[1], 0x21f44d39954fd8ab);
}

#[test]
fn multiprec_less() {
    assert!(!uless128(&A, &A));
    assert!(uless128(&NUM2, &NUM3));
    assert!(uless128(&DENOM1, &DENOM2));
    assert!(ulessequal128(&A, &A));
    assert!(ulessequal128(&NUM2, &NUM3));
    assert!(ulessequal128(&DENOM2, &DENOM2));
}
