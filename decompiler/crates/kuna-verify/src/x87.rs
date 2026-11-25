//! x87's 80-bit extended format, for the operations that are exact without an
//! 80-bit arithmetic unit: conversions in from integers and IEEE single and
//! double, out to them with round-to-nearest-even, sign operations,
//! comparisons, the NaN test and truncation to an integer. Arithmetic on the
//! format stays unsupported.
//!
//! A value is the low 80 bits of a `u128`: sign (bit 79), a 15-bit biased
//! exponent, and a 64-bit significand whose top bit is the explicit integer bit.

use kuna_num::opcodes::OpCode;

const BIAS: i32 = 16383;
const EXP_MAX: u128 = 0x7fff;

fn parts(v: u128) -> (bool, u128, u128) {
    ((v >> 79) & 1 == 1, (v >> 64) & EXP_MAX, v & u128::from(u64::MAX))
}

fn pack(neg: bool, exp: u128, sig: u128) -> u128 {
    (u128::from(neg) << 79) | (exp << 64) | sig
}

fn is_nan(v: u128) -> bool {
    let (_, e, m) = parts(v);
    e == EXP_MAX && m & !(1u128 << 63) != 0
}

/// An integer magnitude, exactly: every integer of up to 64 bits fits.
fn from_int(neg: bool, mag: u128) -> u128 {
    if mag == 0 {
        return pack(neg, 0, 0);
    }
    let top = 127 - mag.leading_zeros() as i32;
    let sig = if top >= 63 { mag >> (top - 63) } else { mag << (63 - top) };
    pack(neg, (BIAS + top) as u128, sig & u128::from(u64::MAX))
}

/// An IEEE value of `size` bytes, exactly.
fn from_ieee(v: u128, size: u32) -> u128 {
    let (ebits, fbits) = if size == 4 { (8, 23) } else { (11, 52) };
    let neg = (v >> (ebits + fbits)) & 1 == 1;
    let e = (v >> fbits) & ((1 << ebits) - 1);
    let f = v & ((1 << fbits) - 1);
    let emax = (1 << ebits) - 1;
    let bias = (1i32 << (ebits - 1)) - 1;
    if e == emax {
        return pack(neg, EXP_MAX, (1 << 63) | (f << (63 - fbits)));
    }
    if e == 0 {
        if f == 0 {
            return pack(neg, 0, 0);
        }
        let top = 127 - f.leading_zeros() as i32;
        let exp = 1 - bias - fbits as i32 + top;
        return pack(neg, (BIAS + exp) as u128, (f << (63 - top)) & u128::from(u64::MAX));
    }
    pack(neg, (e as i32 - bias + BIAS) as u128, (1 << 63) | (f << (63 - fbits)))
}

/// To an IEEE value of `size` bytes, rounding to nearest, ties to even.
pub fn to_ieee(v: u128, size: u32) -> u128 {
    let (ebits, fbits) = if size == 4 { (8u32, 23i32) } else { (11, 52) };
    let (neg, e, m) = parts(v);
    let sign = u128::from(neg) << (ebits as i32 + fbits);
    let emax = (1u128 << ebits) - 1;
    if e == EXP_MAX {
        let rest = m & !(1 << 63);
        let frac = if rest != 0 { (rest >> (63 - fbits)) | (1 << (fbits - 1)) } else { 0 };
        return sign | (emax << fbits) | frac;
    }
    if m == 0 {
        return sign;
    }
    let bias = (1i32 << (ebits - 1)) - 1;
    let scale = e.max(1) as i32 - BIAS - 63;
    let top = 127 - m.leading_zeros() as i32;
    let quantum = (scale + top - fbits).max(1 - bias - fbits);
    let drop = quantum - scale;
    let mut sig = if drop <= 0 {
        m << (-drop)
    } else if drop >= 66 {
        0
    } else {
        let kept = m >> drop;
        let rest = m & ((1u128 << drop) - 1);
        let half = 1u128 << (drop - 1);
        kept + u128::from(rest > half || (rest == half && kept & 1 == 1))
    };
    let mut quantum = quantum;
    if sig >> (fbits + 1) != 0 {
        sig >>= 1;
        quantum += 1;
    }
    if sig >> fbits != 0 {
        let field = (quantum + fbits + bias) as u128;
        if field >= emax {
            return sign | (emax << fbits);
        }
        sign | (field << fbits) | (sig & ((1 << fbits) - 1))
    } else {
        sign | sig
    }
}

/// A key that orders non-NaN values as the numbers they are; zeros of either
/// sign are equal.
fn order(v: u128) -> i128 {
    let (neg, e, m) = parts(v);
    let mag = if m == 0 { 0 } else { ((e << 64) | m) as i128 };
    if neg { -mag } else { mag }
}

/// Truncate toward zero to a signed integer of `size` bytes; out of range or
/// NaN gives x87's integer indefinite, the smallest integer.
fn trunc(v: u128, size: u32) -> u128 {
    let bits = size * 8;
    let indefinite = 1u128 << (bits - 1);
    let (neg, e, m) = parts(v);
    if e == EXP_MAX {
        return indefinite;
    }
    let shift = e as i32 - BIAS - 63;
    let mag = if shift >= 0 {
        if shift > 64 || (m << shift) >> (bits - 1) != 0 && !(neg && m << shift == indefinite) {
            return indefinite;
        }
        m << shift
    } else if shift <= -64 {
        0
    } else {
        m >> (-shift)
    };
    if mag >> (bits - 1) != 0 && !(neg && mag == indefinite) {
        return indefinite;
    }
    let mask = if bits >= 128 { u128::MAX } else { (1u128 << bits) - 1 };
    (if neg { mag.wrapping_neg() } else { mag }) & mask
}

/// Evaluate `opc` when an input or the output is the 10-byte format; `None`
/// for an operation this module does not do exactly.
pub fn eval(opc: OpCode, out: u32, ins: &[(u128, u32)]) -> Option<u128> {
    let &(a, asz) = ins.first()?;
    let ext = |(v, s): (u128, u32)| match s {
        10 => Some(v & ((1u128 << 80) - 1)),
        4 | 8 => Some(from_ieee(v, s)),
        _ => None,
    };
    let sign = 1u128 << 79;
    match opc {
        OpCode::CPUI_FLOAT_INT2FLOAT if out == 10 && asz <= 8 => {
            let bits = asz * 8;
            let s = ((a << (128 - bits)) as i128) >> (128 - bits);
            Some(from_int(s < 0, s.unsigned_abs()))
        }
        OpCode::CPUI_FLOAT_FLOAT2FLOAT if out == 10 => ext((a, asz)),
        OpCode::CPUI_FLOAT_FLOAT2FLOAT if asz == 10 && (out == 4 || out == 8) => Some(to_ieee(a, out)),
        OpCode::CPUI_FLOAT_NEG if asz == 10 && out == 10 => Some(a ^ sign),
        OpCode::CPUI_FLOAT_ABS if asz == 10 && out == 10 => Some(a & !sign),
        OpCode::CPUI_FLOAT_NAN if asz == 10 => Some(u128::from(is_nan(a))),
        OpCode::CPUI_FLOAT_TRUNC if asz == 10 && out <= 8 => Some(trunc(a, out)),
        OpCode::CPUI_FLOAT_EQUAL | OpCode::CPUI_FLOAT_NOTEQUAL | OpCode::CPUI_FLOAT_LESS | OpCode::CPUI_FLOAT_LESSEQUAL
            if ins.iter().any(|&(_, s)| s == 10) =>
        {
            let (x, y) = (ext(a_in(ins, 0)?)?, ext(a_in(ins, 1)?)?);
            let unordered = is_nan(x) || is_nan(y);
            let (kx, ky) = (order(x), order(y));
            Some(u128::from(match opc {
                OpCode::CPUI_FLOAT_EQUAL => !unordered && kx == ky,
                OpCode::CPUI_FLOAT_NOTEQUAL => unordered || kx != ky,
                OpCode::CPUI_FLOAT_LESS => !unordered && kx < ky,
                _ => !unordered && kx <= ky,
            }))
        }
        _ => None,
    }
}

fn a_in(ins: &[(u128, u32)], k: usize) -> Option<(u128, u32)> {
    ins.get(k).copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn doubles_round_trip_and_integers_are_exact() {
        for d in [0.0f64, -0.0, 1.0, -2.5, 1e300, 5e-324, 2.2250738585072014e-308, f64::INFINITY, 0.1] {
            let x = from_ieee(u128::from(d.to_bits()), 8);
            assert_eq!(to_ieee(x, 8), u128::from(d.to_bits()), "{d}");
        }
        let one = from_int(false, 1);
        assert_eq!(one, (16383u128 << 64) | (1 << 63));
        assert_eq!(to_ieee(from_int(false, (1 << 53) + 1), 8), u128::from(9007199254740992f64.to_bits()));
        assert_eq!(to_ieee(from_int(false, (1 << 53) + 3), 8), u128::from(9007199254740996f64.to_bits()));
        assert_eq!(to_ieee(from_int(true, 3), 4), u128::from((-3.0f32).to_bits()));
        assert_eq!(trunc(from_ieee(u128::from((-7.9f64).to_bits()), 8), 4), (-7i32) as u32 as u128);
        assert_eq!(trunc(from_ieee(u128::from(1e30f64.to_bits()), 8), 4), 0x8000_0000);
        let nan = from_ieee(u128::from(f64::NAN.to_bits()), 8);
        assert!(is_nan(nan));
        assert_eq!(eval(OpCode::CPUI_FLOAT_LESS, 1, &[(from_int(true, 1), 10), (0, 10)]), Some(1));
        assert_eq!(eval(OpCode::CPUI_FLOAT_EQUAL, 1, &[(pack(true, 0, 0), 10), (0, 10)]), Some(1));
        assert_eq!(eval(OpCode::CPUI_FLOAT_EQUAL, 1, &[(nan, 10), (nan, 10)]), Some(0));
    }
}
