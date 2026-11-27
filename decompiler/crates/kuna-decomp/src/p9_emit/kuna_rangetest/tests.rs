use super::*;

fn merged(x: u64, a: u64, h: u64, bits: u32) -> bool {
    let mask = u64::MAX >> (64 - bits);
    x.wrapping_sub(a) & mask <= h
}

/// The C value of the `bits`-bit pattern `x` printed in a type of `bits` bits.
fn c_value(x: u64, bits: u32, unsigned: bool) -> i128 {
    if unsigned || x >> (bits - 1) == 0 {
        x as i128
    } else {
        x as i128 - (1i128 << bits)
    }
}

#[test]
fn apply_parses_each_value_and_reports() {
    assert_eq!(OptionRangeTest.apply("merged").unwrap().0, RangeTestMode::Merged);
    assert_eq!(OptionRangeTest.apply("split").unwrap().0, RangeTestMode::Split);
    let (mode, msg) = OptionRangeTest.apply("auto").unwrap();
    assert_eq!(mode, RangeTestMode::Auto);
    assert_eq!(msg, "Range test rendering set to auto");
    assert_eq!(OptionRangeTest.apply("").unwrap().0, RangeTestMode::Auto);
    assert!(OptionRangeTest.apply("on").is_err());
    assert_eq!(OptionRangeTest::NAME, "rangetest");
}

#[test]
fn orient_reads_each_printed_comparison() {
    assert_eq!(orient("<", false, 10, 0xff), Some((true, 9)));
    assert_eq!(orient("<=", false, 9, 0xff), Some((true, 9)));
    assert_eq!(orient(">", false, 9, 0xff), Some((false, 9)));
    assert_eq!(orient(">=", false, 10, 0xff), Some((false, 9)));
    assert_eq!(orient("<=", true, 10, 0xff), Some((false, 9)));
    assert_eq!(orient("<", true, 9, 0xff), Some((false, 9)));
    assert_eq!(orient(">", true, 10, 0xff), Some((true, 9)));
    assert_eq!(orient(">=", true, 9, 0xff), Some((true, 9)));
    assert_eq!(orient("<", false, 0, 0xff), None);
    assert_eq!(orient("<=", false, 0xff, 0xff), None);
    assert_eq!(orient("==", false, 3, 0xff), None);
}

#[test]
fn interval_refuses_a_wrapping_range_and_a_zero_offset() {
    assert_eq!(interval(0x30, 9, 8), Some((0x30, 0x39)));
    assert_eq!(interval(0xfe, 3, 8), None);
    assert_eq!(interval(0, 9, 8), None);
    assert_eq!(interval(1, u64::MAX - 2, 64), Some((1, u64::MAX - 1)));
    assert_eq!(interval(2, u64::MAX - 1, 64), None);
}

#[test]
fn exact_bounds_the_signed_and_unsigned_reading() {
    assert!(exact(0x7f, 8, false));
    assert!(!exact(0x80, 8, false));
    assert!(exact(0xff, 8, true));
    assert!(exact(u64::MAX, 64, true));
    assert!(!exact(1 << 63, 64, false));
}

/// Every accepted 8-bit case, in both signednesses: the two printed
/// comparisons agree with the merged test on every input, inside and outside.
#[test]
fn every_accepted_byte_split_is_the_merged_test() {
    let bits = 8;
    for a in 0..256u64 {
        for h in 0..255u64 {
            let Some((lo, hi)) = interval(a, h, bits) else { continue };
            for unsigned in [true, false] {
                if !exact(hi, bits, unsigned) {
                    continue;
                }
                for x in 0..256u64 {
                    let v = c_value(x, bits, unsigned);
                    let inside = lo as i128 <= v && v <= hi as i128;
                    let outside = v < lo as i128 || (hi as i128) < v;
                    assert_eq!(inside, merged(x, a, h, bits), "a={a} h={h} x={x} unsigned={unsigned}");
                    assert_eq!(outside, !merged(x, a, h, bits));
                }
            }
        }
    }
}

/// Wider types at their boundaries: `A - 1`, `A`, `A + B`, `A + B + 1`, `0`
/// and the type's extremes.
#[test]
fn wide_splits_agree_at_the_boundaries() {
    for bits in [16u32, 32, 64] {
        let mask = u64::MAX >> (64 - bits);
        let cases = [(1u64, 1u64), (0x30, 9), (0x41, 25), (1, mask - 2), (2, mask - 3), (0x1000, 0xff000), (1, (mask >> 1) - 1)];
        for (a, h) in cases {
            let Some((lo, hi)) = interval(a, h, bits) else { continue };
            for unsigned in [true, false] {
                if !exact(hi, bits, unsigned) {
                    continue;
                }
                let probes = [a.wrapping_sub(1) & mask, a, hi, (hi + 1) & mask, 0, mask, mask >> 1, (mask >> 1) + 1];
                for x in probes {
                    let v = c_value(x, bits, unsigned);
                    assert_eq!(lo as i128 <= v && v <= hi as i128, merged(x, a, h, bits), "bits={bits} a={a} h={h} x={x}");
                }
            }
        }
    }
}
