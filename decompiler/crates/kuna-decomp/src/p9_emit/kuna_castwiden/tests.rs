//! Tests for the arithmetic `castwiden` rests on.
//!
//! Which cast a function prints needs a decompile and is covered end to end by
//! `tests/stages/kuna-castwiden.xml` (off, on, literal) and by the compiled round
//! trip in `kuna-cli/tests/decompile_all_cli.rs`
//! (`an_implied_widening_round_trips_through_the_printed_c`).  What is pinned here
//! is the rule every removal rests on: `fits` accepts a widening only where C's
//! usual arithmetic conversions give the same type and the same value without it.

use super::*;

/// A promoted C integer type as (bits, signed).
fn shape(t: u8) -> (u32, bool) {
    match t {
        INT => (32, true),
        UINT => (32, false),
        LONG => (64, true),
        _ => (64, false),
    }
}

/// C's conversion of the value `v` to the promoted type `t` (C11 6.3.1.3; gcc and
/// clang define the signed case modulo 2^N).
fn conv(v: i128, t: u8) -> i128 {
    let (bits, signed) = shape(t);
    let m = v.rem_euclid(1i128 << bits);
    if signed && m >= 1i128 << (bits - 1) {
        m - (1i128 << bits)
    } else {
        m
    }
}

/// `a op b` evaluated in the promoted type `t` (wrapping; the unsigned rule, and
/// what the binary computes).
fn eval(op: char, a: i128, b: i128, t: u8) -> Option<i128> {
    let (bits, signed) = shape(t);
    let r = match op {
        '+' => a + b,
        '-' => a - b,
        '*' => a.wrapping_mul(b),
        '&' => (a.rem_euclid(1i128 << bits) & b.rem_euclid(1i128 << bits)),
        '|' => (a.rem_euclid(1i128 << bits) | b.rem_euclid(1i128 << bits)),
        '^' => (a.rem_euclid(1i128 << bits) ^ b.rem_euclid(1i128 << bits)),
        '/' | '%' => {
            if b == 0 || (signed && b == -1) {
                return None;
            }
            let q = a / b;
            if op == '/' {
                q
            } else {
                a - q * b
            }
        }
        _ => unreachable!(),
    };
    Some(conv(r, t))
}

fn samples(t: u8) -> Vec<i128> {
    let (bits, signed) = shape(t);
    let (lo, hi) = if signed {
        (-(1i128 << (bits - 1)), (1i128 << (bits - 1)) - 1)
    } else {
        (0, (1i128 << bits) - 1)
    };
    let mut v = vec![lo, lo + 1, -7, -1, 0, 1, 3, 0x7fff_ffff, 0x8000_0000, 0xffff_ffff, 0x1_0000_0000, hi - 1, hi];
    v.retain(|x| (lo..=hi).contains(x));
    v
}

#[test]
fn fits_is_exactly_an_unchanged_type_and_value() {
    for to in [LONG, ULONG] {
        for from in [INT, UINT] {
            // The extension C performs on `from`: a value `from` keeps under `to`
            // unless `to` cannot hold its negative values.
            let keeps = !(from == INT && to == ULONG);
            let w = Widening { op: OpId::default(), to, from, keeps };
            for y in [INT, UINT, LONG, ULONG] {
                let with_t = usual(to, y);
                let bare_t = usual(from, y);
                let mut same = with_t == bare_t;
                for &x in &samples(from) {
                    for &b in &samples(y) {
                        for op in ['+', '-', '*', '&', '|', '^', '/', '%'] {
                            let with = eval(op, conv(conv(x, to), with_t), conv(b, with_t), with_t);
                            let bare = eval(op, conv(x, bare_t), conv(b, bare_t), bare_t);
                            if with != bare {
                                same = false;
                            }
                        }
                    }
                }
                assert_eq!(fits(&w, y), same, "to {to} from {from} other {y}");
            }
        }
    }
}

#[test]
fn of_several_possible_types_every_one_must_fit() {
    let w = Widening { op: OpId::default(), to: LONG, from: INT, keeps: true };
    assert!(fits(&w, LONG));
    assert!(fits(&w, ULONG));
    assert!(!fits(&w, LONG | INT));
    assert!(!fits(&w, 0));
}

#[test]
fn a_literal_is_typed_in_both_bases() {
    assert_eq!(token_types("8", 8), Some(INT));
    assert_eq!(token_types("0xc", 8), Some(INT));
    assert_eq!(token_types("8L", 8), Some(LONG));
    assert_eq!(token_types("0xcL", 8), Some(LONG));
    assert_eq!(token_types("-4UL", 8), Some(ULONG));
    assert_eq!(token_types("3UL", 8), Some(ULONG));
    // Printed in decimal `3000000000` is a `long`; in hex, `0xb2d05e00` is an
    // `unsigned int`.  The printer picks the base, so both are allowed for.
    assert_eq!(token_types("3000000000", 8), Some(LONG | UINT));
    assert_eq!(token_types("0xb2d05e00", 8), Some(LONG | UINT));
    assert_eq!(token_types("-0x80000000", 8), Some(LONG | UINT));
    assert_eq!(token_types("0x100000000", 8), Some(LONG));
    // Too wide for a decimal literal of any signed type: no C type at all.
    assert_eq!(token_types("18446744073709551615", 8), None);
    assert_eq!(token_types("0x100000000L", 8), Some(LONG));
    assert_eq!(token_types("'a'", 8), Some(INT));
}

#[test]
fn a_long_suffix_where_long_is_four_bytes_is_long_long() {
    assert_eq!(token_types("8LL", 4), Some(LONG));
    assert_eq!(token_types("8ULL", 4), Some(ULONG));
    assert_eq!(token_types("8L", 4), Some(INT));
}
