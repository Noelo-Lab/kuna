//! Keep the rounding an integer-to-float conversion performs when a later
//! float conversion or a narrower `SubfloatFlow` trace would fold it away
//! (`option intfloatround`). Folding is exact only when the format the value
//! passes through holds every integer the source can be.

use crate::context::ArchContext;
use kuna_base::types::{int4, uintb};

/// Significant bits of a `size`-byte float. The 80-bit extended format SLEIGH
/// gives x87 and 68k registers has no FloatFormat entry but carries 64.
fn float_precision(arch: &ArchContext, size: int4) -> Option<int4> {
    arch.get_float_format(size).map(|f| f.get_precision()).or((size == 10).then_some(64))
}

/// Can a binary format with `precision` significant bits hold every value a
/// signed `size`-byte integer converts from, or the one `constant` it is?
fn int_converts_exactly(precision: int4, size: int4, constant: Option<uintb>) -> bool {
    match constant {
        Some(raw) if (1..=8).contains(&size) => {
            let shift = 64 - size as u32 * 8;
            let magnitude = (((raw << shift) as i64) >> shift).unsigned_abs();
            magnitude == 0
                || 64 - (magnitude >> magnitude.trailing_zeros()).leading_zeros() <= precision as u32
        }
        _ => precision >= size * 8 - 1,
    }
}

/// May `RuleFloatCast` turn FLOAT2FLOAT(INT2FLOAT(x)) into one INT2FLOAT of
/// the `int_size`-byte `x` straight to `out_size`, skipping the `mid_size`
/// float the binary rounds through?
pub(crate) fn combines(arch: &ArchContext, int_size: int4, mid_size: int4, out_size: int4) -> bool {
    !arch.int_float_round
        || mid_size == out_size
        || float_precision(arch, mid_size).is_some_and(|bits| int_converts_exactly(bits, int_size, None))
}

/// May `SubfloatFlow` re-issue an INT2FLOAT of an `int_size`-byte value (or
/// `constant`) at its narrower `precision` instead of `original_size`? The
/// replacement is never wider than the original, so its own exactness is
/// the whole condition.
pub(crate) fn lowers(arch: &ArchContext, int_size: int4, constant: Option<uintb>, original_size: int4, precision: int4) -> bool {
    !arch.int_float_round
        || original_size == precision
        || float_precision(arch, precision).is_some_and(|bits| int_converts_exactly(bits, int_size, constant))
}

#[cfg(test)]
mod tests {
    use super::int_converts_exactly;

    #[test]
    fn constants_need_only_their_own_significant_bits() {
        assert!(int_converts_exactly(24, 4, Some(1)));
        assert!(int_converts_exactly(24, 4, Some(0)));
        assert!(int_converts_exactly(24, 4, Some(0xffff_ffff)));
        assert!(int_converts_exactly(24, 4, Some(0x4000_0000)));
        assert!(!int_converts_exactly(24, 4, Some(0x0100_0001)));
        assert!(int_converts_exactly(53, 8, Some(0x8000_0000_0000_0000)));
        assert!(!int_converts_exactly(53, 8, Some(0x7fff_ffff_ffff_ffff)));
    }

    #[test]
    fn variables_need_the_width_less_the_sign() {
        assert!(int_converts_exactly(24, 3, None));
        assert!(!int_converts_exactly(24, 4, None));
        assert!(int_converts_exactly(53, 4, None));
        assert!(!int_converts_exactly(53, 8, None));
        assert!(int_converts_exactly(64, 8, None));
    }
}
