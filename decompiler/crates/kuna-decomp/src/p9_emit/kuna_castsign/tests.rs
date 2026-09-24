//! Tests for the arithmetic `castsign` rests on.
//!
//! Which declaration a function prints needs a decompile and is covered end to
//! end by `tests/stages/kuna-castsign.xml` (pass 1 off, pass 2 on) and by the
//! compiled round trip in `kuna-cli/tests/decompile_all_cli.rs`
//! (`a_signed_only_variable_round_trips_through_the_printed_c`).  What is pinned
//! here is why each relaxation leaves the computed value alone.

/// C's conversion of `v` to an integer of `bits` width and the given signedness
/// (C11 6.3.1.3): kept when representable, else reduced modulo 2^bits -- the
/// unsigned rule, and what gcc and clang define for the signed case.
fn convert(v: i128, bits: u32, signed: bool) -> i128 {
    let m = v.rem_euclid(1i128 << bits);
    if signed && m >= 1i128 << (bits - 1) {
        m - (1i128 << bits)
    } else {
        m
    }
}

fn samples(bits: u32) -> Vec<i128> {
    let top = 1i128 << bits;
    vec![0, 1, 2, 0x7f, 0x80, 0xff, 0x7fff, 0x8000, top / 2 - 1, top / 2, top - 1, -1, -2, -(top / 2)]
}

#[test]
fn a_cast_to_the_old_declaration_is_the_assignment_to_the_new_one() {
    // `lhs = (T)e;` with `lhs` re-declared from T to D of the same width: the
    // value stored is the same for every operand value and every operand width.
    for bits in [8u32, 16, 32, 64] {
        for src_bits in [8u32, 16, 32, 64] {
            for src_signed in [false, true] {
                for t_signed in [false, true] {
                    for v in samples(src_bits) {
                        let e = convert(v, src_bits, src_signed);
                        let via_cast = convert(convert(e, bits, t_signed), bits, !t_signed);
                        let direct = convert(e, bits, !t_signed);
                        assert_eq!(via_cast, direct, "{src_bits}->{bits} {e}");
                    }
                }
            }
        }
    }
}

#[test]
fn a_comparison_the_new_declaration_reads_is_the_one_the_cast_spelled() {
    // `(long)v < k` over `unsigned long v` and `v < k` over `long v` compare the
    // same value: the cast is exactly the conversion the new declaration makes.
    for bits in [32u32, 64] {
        for v in samples(bits) {
            let stored = convert(v, bits, false);
            let old_reading = convert(stored, bits, true);
            let new_reading = convert(convert(v, bits, true), bits, true);
            assert_eq!(old_reading, new_reading);
        }
    }
}

#[test]
fn a_pointer_width_index_addresses_the_same_byte_either_way() {
    // `p[v]` at pointer width: the address is `p + v * size` modulo 2^64 whether
    // `v` is read signed or unsigned.
    let p: u64 = 0x5555_0000_1000;
    for v in samples(64) {
        for size in [1u64, 2, 4, 8, 24] {
            let as_signed = convert(v, 64, true) as i64;
            let as_unsigned = convert(v, 64, false) as u64;
            let a = p.wrapping_add((as_signed as u64).wrapping_mul(size));
            let b = p.wrapping_add(as_unsigned.wrapping_mul(size));
            assert_eq!(a, b, "{v} * {size}");
        }
    }
}

#[test]
fn a_same_width_conversion_to_a_pointer_keeps_the_bits_either_way() {
    // gcc and clang convert an integer to a pointer of the same width by keeping
    // its bits, so `(char *)v` is the same pointer under either declaration.
    for v in samples(64) {
        let signed_bits = convert(v, 64, true) as i64 as u64;
        let unsigned_bits = convert(v, 64, false) as u64;
        assert_eq!(signed_bits, unsigned_bits);
    }
}
