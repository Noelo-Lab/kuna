//! Port of `decompiler/cpp/float.{hh,cc}` -- FloatFormat (item
//! w1-num-float-multiprec).
//!
//! Encoding information for a single floating-point format.  This supports
//! manipulation of a single floating-point encoding: an encoding can be
//! converted to and from the host format, and convenience methods allow
//! p-code floating-point operations to be performed on natively encoded
//! operands.  This follows the IEEE 754 standards.
//!
//! Host-semantics notes (tests/golden/vectors/README.md caveats):
//!
//! - Exactly where the C++ computes on host `double`, this port computes on
//!   host `f64` (same IEEE 754 binary64 arithmetic, same hardware ops on the
//!   oracle's x86 host).  IEEE-754 leaves the *sign* of a NaN produced by an
//!   invalid operation (`sqrt(-1)`, `0/0`, `inf-inf`, `0*inf`, …) host-defined:
//!   the x86 FPU yields the negative "real indefinite" QNaN (`0xffc00000`),
//!   Apple-Silicon/ARM the positive default NaN (`0x7fc00000`); Ghidra's C++
//!   inherits whichever the build host produces (float.cc `signbit(host)`).
//!   To stay deterministic across build hosts *and* byte-identical to the
//!   Linux/x86 golden oracle everywhere, the arithmetic ops pin the x86 sign
//!   for a *generated* NaN (see [`FloatFormat::encode_generated`]); a
//!   *propagated* NaN (an input was already NaN) keeps its host-deterministic
//!   sign.  The payload is canonicalized by `get_nan_encoding`.
//! - `opTrunc`'s `(intb)double` cast is the host x86 `cvttsd2si` cast, NOT
//!   Rust's saturating `as`; see [`host_double_to_int64`].
//! - C++ `ldexp`/`frexp` have no Rust std equivalent; [`ldexp`] / [`frexp`]
//!   are bit-exact transcriptions of the standard (musl-style) constructions,
//!   correctly rounded like the libm functions the oracle calls.

use kuna_base::cfmt;

/// The various classes of floating-point encodings
/// (C++ `FloatFormat::floatclass`; name kept per the porting conventions).
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum floatclass {
    /// A normal floating-point number
    normalized = 0,
    /// An encoding representing an infinite value
    infinity = 1,
    /// An encoding of the value zero
    zero = 2,
    /// An invalid encoding, Not-a-Number
    nan = 3,
    /// A denormalized encoding (for very small values)
    denormalized = 4,
}

/// Encoding information for a single floating-point format.
///
/// Port of C++ `FloatFormat` (float.hh).
#[derive(Debug, Clone)]
pub struct FloatFormat {
    /// Size of float in bytes (this format)
    size: i32,
    /// Bit position of sign bit
    signbit_pos: i32,
    /// (lowest) bit position of fractional part
    frac_pos: i32,
    /// Number of bits in fractional part
    frac_size: i32,
    /// (lowest) bit position of exponent
    exp_pos: i32,
    /// Number of bits in exponent
    exp_size: i32,
    /// What to add to real exponent to get encoding
    bias: i32,
    /// Maximum possible exponent
    maxexponent: i32,
    /// Minimum decimal digits of precision guaranteed by the format
    decimal_min_precision: i32,
    /// Maximum decimal digits of precision needed to uniquely represent value
    decimal_max_precision: i32,
    /// Set to true if integer bit of 1 is assumed
    jbitimplied: bool,
}

// ---------------------------------------------------------------------------
// Private transcriptions of the address.hh/address.cc helpers float.cc calls.
// The canonical public ports belong to the kuna-base address module (a
// different port item); these stay private to avoid cross-module ownership
// edits, mirroring how multiprecision.cc re-declares its own extern.
// ---------------------------------------------------------------------------

/// `uintbmasks` table (address.cc): mask for each byte size, 0 through 8.
const UINTBMASKS: [u64; 9] = [
    0,
    0xff,
    0xffff,
    0xff_ffff,
    0xffff_ffff,
    0xff_ffff_ffff,
    0xffff_ffff_ffff,
    0xff_ffff_ffff_ffff,
    0xffff_ffff_ffff_ffff,
];

/// `calc_mask(int4 size)` (address.hh): `uintbmasks[((uint4)size) < 8 ? size : 8]`.
/// The C++ compares `size` as unsigned (mixed signed/unsigned), so a negative
/// size wraps to a huge value and selects the full mask (index 8), as do
/// sizes greater than 8.
fn calc_mask(size: i32) -> u64 {
    UINTBMASKS[if (size as u32) < 8 { size as usize } else { 8 }]
}

/// `sign_extend(intb val, int4 bit)` (address.hh): sign extend `val` starting
/// at `bit` (0 = least significant bit).  The C++ `(val << sa) >> sa` left
/// shift of a signed value can formally overflow but compiles to plain
/// shifts; transcribed with wrapping shifts.
fn sign_extend(val: i64, bit: i32) -> i64 {
    let sa = 8 * 8 - (bit + 1); // 8*sizeof(intb) - (bit+1)
    (val.wrapping_shl(sa as u32)).wrapping_shr(sa as u32)
}

/// `count_leading_zeros(uintb val)` (address.cc): number of leading zero
/// bits, by the upstream mask-halving loop (returns 64 for zero).
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

// ---------------------------------------------------------------------------
// Host floating-point plumbing the C++ gets from libm / the x86 host.
// ---------------------------------------------------------------------------

/// 2^1023 (largest exactly representable power-of-two normal double).
const TWO_P1023: f64 = f64::from_bits(0x7FE0_0000_0000_0000);
/// 2^-969 == 0x1p-1022 * 0x1p53 (the combined subnormal step factor).
const TWO_PM969: f64 = f64::from_bits(0x0360_0000_0000_0000);
/// 2^64 (for scaling subnormals in frexp).
const TWO_P64: f64 = f64::from_bits(0x43F0_0000_0000_0000);

/// `ldexp(x, n)` = x * 2^n, correctly rounded (including into and out of the
/// subnormal range).  Standard scalbn construction: clamp the scale into
/// exactly representable power-of-two factors so every intermediate multiply
/// is exact except the last, which performs the single IEEE rounding -- the
/// same result as the glibc `ldexp` the C++ calls.
fn ldexp(x: f64, n: i32) -> f64 {
    let mut y = x;
    let mut n = n;
    if n > 1023 {
        y *= TWO_P1023;
        n -= 1023;
        if n > 1023 {
            y *= TWO_P1023;
            n -= 1023;
            if n > 1023 {
                n = 1023;
            }
        }
    } else if n < -1022 {
        y *= TWO_PM969;
        n += 1022 - 53;
        if n < -1022 {
            y *= TWO_PM969;
            n += 1022 - 53;
            if n < -1022 {
                n = -1022;
            }
        }
    }
    // 2^n for n in [-1022, 1023], built directly from the exponent bits.
    y * f64::from_bits(((0x3ff + n) as u64) << 52)
}

/// `frexp(x)` -> (mantissa in [0.5, 1), exponent), such that
/// x == mantissa * 2^exponent.  Bit-manipulation transcription of the
/// standard implementation; only finite nonzero values reach it from
/// `extract_exp_sig`, but the full behavior is kept for fidelity.
fn frexp(x: f64) -> (f64, i32) {
    let bits = x.to_bits();
    let ee = ((bits >> 52) & 0x7ff) as i32;
    if ee == 0 {
        if x != 0.0 {
            // Subnormal: scale by 2^64 (exact) and adjust the exponent.
            let (m, e) = frexp(x * TWO_P64);
            return (m, e - 64);
        }
        return (x, 0);
    } else if ee == 0x7ff {
        return (x, 0); // inf/NaN pass through (C leaves *e undefined)
    }
    let e = ee - 0x3fe;
    let mbits = (bits & 0x800F_FFFF_FFFF_FFFF) | 0x3FE0_0000_0000_0000;
    (f64::from_bits(mbits), e)
}

/// Host `(int64_t)double` cast as the C++ oracle's x86 host performs it
/// (`cvttsd2si`): truncation toward zero; NaN, infinities, and values whose
/// truncation falls outside the int64 range all produce 0x8000000000000000.
/// Rust's saturating `as` differs on exactly those cells, hence this helper
/// (tests/golden/vectors/README.md determinism caveat).
#[allow(clippy::manual_range_contains)] // explicit asymmetric overflow bounds
fn host_double_to_int64(val: f64) -> i64 {
    if val.is_nan() {
        return i64::MIN;
    }
    // 2^63 is exactly representable; anything >= it, or below -2^63, is out
    // of range (-2^63 itself is in range and yields i64::MIN legitimately).
    if val >= 9_223_372_036_854_775_808.0 || val < -9_223_372_036_854_775_808.0 {
        return i64::MIN;
    }
    val as i64 // in range: `as` truncates toward zero, same as cvttsd2si
}

impl FloatFormat {
    /// Construct default IEEE 754 standard settings for a given encoding
    /// size `sz` in bytes.
    ///
    /// The C++ constructor leaves every field uninitialized for sizes other
    /// than 4 and 8 (undefined behavior downstream); the port treats that as
    /// an internal invariant violation and panics (ADR 0004).
    pub fn new(sz: i32) -> FloatFormat {
        let (signbit_pos, exp_pos, exp_size, frac_pos, frac_size, bias, jbitimplied) = if sz == 4 {
            (31, 23, 8, 0, 23, 127, true)
        } else if sz == 8 {
            (63, 52, 11, 0, 52, 1023, true)
        } else {
            panic!("FloatFormat: unsupported encoding size {sz}");
        };
        let mut res = FloatFormat {
            size: sz,
            signbit_pos,
            frac_pos,
            frac_size,
            exp_pos,
            exp_size,
            bias,
            maxexponent: (1 << exp_size) - 1,
            decimal_min_precision: 0,
            decimal_max_precision: 0,
            jbitimplied,
        };
        res.calc_precision();
        res
    }

    /// Get the size of the encoding in bytes.
    pub fn get_size(&self) -> i32 {
        self.size
    }

    /// Create a double given sign, fractional, and exponent.
    fn create_float(sign: bool, signif: u64, exp: i32) -> f64 {
        let signif = signif >> 1; // Throw away 1 bit of precision we will
                                  // lose anyway, to make sure highbit is 0
        let precis: i32 = 8 * 8 - 1; // fullword - 1 we threw away
        let res = signif as f64; // (double)signif rounds to nearest, like C++
        let expchange = exp - precis + 1; // change in exponent is precis
                                          // -1 integer bit
        let res = ldexp(res, expchange);
        if sign {
            #[allow(clippy::neg_multiply)] // C++ writes res = res * -1.0
            {
                res * -1.0
            }
        } else {
            res
        }
    }

    /// Extract the sign, fractional, and exponent from a given
    /// floating-point value.
    ///
    /// Returns (class, sign, fractional part, exponent).  The C++ leaves the
    /// significand/exponent outputs unwritten for the zero/infinity/nan
    /// classes; callers never read them, so zeros are returned here.
    fn extract_exp_sig(x: f64) -> (floatclass, bool, u64, i32) {
        let sgn = x.is_sign_negative(); // signbit(x)
        if x == 0.0 {
            return (floatclass::zero, sgn, 0, 0);
        }
        if x.is_infinite() {
            return (floatclass::infinity, sgn, 0, 0);
        }
        if x.is_nan() {
            return (floatclass::nan, sgn, 0, 0);
        }
        let x = if sgn { -x } else { x };
        let (norm, e) = frexp(x); // norm is between 1/2 and 1
        let norm = ldexp(norm, 8 * 8 - 1); // norm between 2^62 and 2^63

        // (uintb)norm: norm is integral and < 2^63 here, so the C cast and
        // Rust `as` agree (exact truncation).
        let mut signif = norm as u64; // Convert to normalized integer
        signif <<= 1;

        let e = e - 1; // Consider normalization between 1 and 2
        (floatclass::normalized, sgn, signif, e)
    }

    /// Extract the fraction part of the encoding,
    /// aligned to the top of the word.
    pub fn extract_fractional_code(&self, x: u64) -> u64 {
        let x = x >> self.frac_pos; // Eliminate bits below
        x << (8 * 8 - self.frac_size) // Align with top of word
    }

    /// Extract the sign bit from the encoding.
    pub fn extract_sign(&self, x: u64) -> bool {
        let x = x >> self.signbit_pos;
        (x & 1) != 0
    }

    /// Extract the (signed) exponent from the encoding.
    pub fn extract_exponent_code(&self, x: u64) -> i32 {
        let x = x >> self.exp_pos;
        let mut mask: u64 = 1;
        mask = (mask << self.exp_size) - 1;
        (x & mask) as i32
    }

    /// Get the exponent of an encoding.
    pub fn get_exponent(&self, x: u64) -> i32 {
        self.extract_exponent_code(x) - self.bias
    }

    /// Set the fractional part of an encoded value
    /// (`x` has its fraction part set to zero).
    fn set_fractional_code(&self, x: u64, code: u64) -> u64 {
        // Align with bottom of word, also drops bits of precision
        // we don't have room for
        let code = code >> (8 * 8 - self.frac_size);
        let code = code << self.frac_pos; // Move bits into position;
        x | code
    }

    /// Set the sign bit of an encoded value (`x` has sign set to zero).
    fn set_sign(&self, x: u64, sign: bool) -> u64 {
        if !sign {
            return x; // Assume bit is already zero
        }
        let mut mask: u64 = 1;
        mask <<= self.signbit_pos;
        x | mask // Stick in the bit
    }

    /// Set the exponent of an encoded value (`x` has exponent set to zero).
    fn set_exponent_code(&self, x: u64, code: u64) -> u64 {
        let code = code << self.exp_pos; // Move bits into position
        x | code
    }

    /// Get an encoded zero value; `sgn` is true for negative zero.
    fn get_zero_encoding(&self, sgn: bool) -> u64 {
        let mut res: u64 = 0;
        // Use IEEE 754 standard for zero encoding
        res = self.set_fractional_code(res, 0);
        res = self.set_exponent_code(res, 0);
        self.set_sign(res, sgn)
    }

    /// Get an encoded infinite value; `sgn` is true for negative infinity.
    fn get_infinity_encoding(&self, sgn: bool) -> u64 {
        let mut res: u64 = 0;
        // Use IEEE 754 standard for infinity encoding
        res = self.set_fractional_code(res, 0);
        res = self.set_exponent_code(res, self.maxexponent as u64);
        self.set_sign(res, sgn)
    }

    /// Get an encoded NaN value; `sgn` is true for negative NaN.
    fn get_nan_encoding(&self, sgn: bool) -> u64 {
        let mut res: u64 = 0;
        // Use IEEE 754 standard for NaN encoding
        let mut mask: u64 = 1;
        mask <<= 8 * 8 - 1; // Create "quiet" NaN
        res = self.set_fractional_code(res, mask);
        res = self.set_exponent_code(res, self.maxexponent as u64);
        self.set_sign(res, sgn)
    }

    /// Calculate the decimal precision of this format.
    // The C++ uses the literal 0.30103 (an approximation of log10(2)); the
    // exact f64::consts::LOG10_2 would be a different constant and could
    // change the computed precision, so the literal is kept.
    #[allow(clippy::approx_constant)]
    fn calc_precision(&mut self) {
        self.decimal_min_precision = (f64::from(self.frac_size) * 0.30103).floor() as i32;
        // Precision needed to guarantee IEEE 754 binary -> decimal -> binary
        // round trip conversion
        self.decimal_max_precision =
            ((f64::from(self.frac_size) + 1.0) * 0.30103).ceil() as i32 + 1;
    }

    /// Get the class of an encoding: either zero, infinity, denormalized,
    /// nan, or normalized.
    pub fn get_class(&self, encoding: u64) -> floatclass {
        let exp = self.extract_exponent_code(encoding);
        if exp == 0 {
            if self.extract_fractional_code(encoding) == 0 {
                return floatclass::zero;
            }
            return floatclass::denormalized;
        }
        if exp == self.maxexponent {
            if self.extract_fractional_code(encoding) == 0 {
                return floatclass::infinity;
            }
            return floatclass::nan;
        }
        floatclass::normalized
    }

    /// Convert an encoding into the host's double.  Returns the equivalent
    /// double value and the floating-point class (which the C++ passes back
    /// through a pointer).
    pub fn get_host_float(&self, encoding: u64) -> (f64, floatclass) {
        let sgn = self.extract_sign(encoding);
        let mut frac = self.extract_fractional_code(encoding);
        let mut exp = self.extract_exponent_code(encoding);
        let mut normal = true;
        let tp: floatclass;

        if exp == 0 {
            if frac == 0 {
                // Floating point zero
                return (if sgn { -0.0 } else { 0.0 }, floatclass::zero);
            }
            tp = floatclass::denormalized;
            // Number is denormalized
            normal = false;
        } else if exp == self.maxexponent {
            if frac == 0 {
                // Floating point infinity
                let infinity = f64::INFINITY;
                return (
                    if sgn { -infinity } else { infinity },
                    floatclass::infinity,
                );
            }
            // encoding is "Not a Number" NaN
            let nan = f64::NAN;
            return (if sgn { -nan } else { nan }, floatclass::nan); // Sign is usually ignored
        } else {
            tp = floatclass::normalized;
        }

        // Get "true" exponent and fractional
        exp -= self.bias;
        if normal && self.jbitimplied {
            frac >>= 1; // Make room for 1 jbit
            let highbit: u64 = 1u64 << (8 * 8 - 1);
            frac |= highbit; // Stick bit in at top
        }
        (Self::create_float(sgn, frac, exp), tp)
    }

    /// Round a floating point value to the nearest even.
    ///
    /// `signif` is the significant bits of a floating point value;
    /// `lowbitpos` is the position in signif of the floating point.
    /// Returns true if we rounded up.
    fn round_to_nearest_even(signif: &mut u64, lowbitpos: i32) -> bool {
        // C++ compares int4 lowbitpos against the size_t 8*sizeof(uintb)
        // (mixed signed/unsigned, promoting lowbitpos to unsigned); lowbitpos
        // is always in [11, 64] at the call sites, so a plain signed compare
        // is equivalent.
        let lowbitmask: u64 = if lowbitpos < 8 * 8 { 1u64 << lowbitpos } else { 0 };
        let midbitmask: u64 = 1u64 << (lowbitpos - 1);
        let epsmask = midbitmask - 1;
        let odd = (*signif & lowbitmask) != 0;
        if (*signif & midbitmask) != 0 && ((*signif & epsmask) != 0 || odd) {
            // The add can carry out of bit 63 and wrap (C++ unsigned wrap);
            // callers detect that through the cleared high bit.
            *signif = signif.wrapping_add(midbitmask);
            return true;
        }
        false
    }

    /// Convert the host's double into this encoding.
    pub fn get_encoding(&self, host: f64) -> u64 {
        let (tp, sgn, mut signif, mut exp) = Self::extract_exp_sig(host);
        if tp == floatclass::zero {
            return self.get_zero_encoding(sgn);
        } else if tp == floatclass::infinity {
            return self.get_infinity_encoding(sgn);
        } else if tp == floatclass::nan {
            return self.get_nan_encoding(sgn);
        }

        // convert exponent and fractional to their encodings
        exp += self.bias;

        if exp < -self.frac_size {
            // Exponent is too small to represent
            return self.get_zero_encoding(sgn); // TODO handle round to non-zero
        }

        if exp < 1 {
            // Must be denormalized
            if Self::round_to_nearest_even(&mut signif, 8 * 8 - self.frac_size - exp) {
                // TODO handle round to normal case
                if (signif >> (8 * 8 - 1)) == 0 {
                    signif = 1u64 << (8 * 8 - 1);
                    exp += 1;
                }
            }
            let res = self.get_zero_encoding(sgn);
            // The round-carry-to-normal fixup above can leave exp == 1 (host
            // value at/just below this format's smallest normal), making the
            // C++ `signif >> (-exp)` a shift by -1: undefined behavior that
            // the x86-64 oracle resolves by masking the count to 63 (SHR), so
            // setFractionalCode drops the fraction to 0 and the signed zero
            // encoding is returned.  Upstream bug, but oracle-observable:
            // getEncoding(fmt4, bits 0x380fffffffffffff) == 0x0.
            // wrapping_shr applies the identical count mask (& 63), keeping
            // debug and release builds identical (ADR 0003).
            return self.set_fractional_code(res, signif.wrapping_shr((-exp) as u32));
        }

        if Self::round_to_nearest_even(&mut signif, 8 * 8 - self.frac_size - 1) {
            // if high bit is clear, then the add overflowed. Increase exp and set
            // signif to 1.
            if (signif >> (8 * 8 - 1)) == 0 {
                signif = 1u64 << (8 * 8 - 1);
                exp += 1;
            }
        }

        if exp >= self.maxexponent {
            // Exponent is too big to represent
            return self.get_infinity_encoding(sgn);
        }

        if self.jbitimplied && (exp != 0) {
            signif <<= 1; // Cut off top bit (which should be 1)
        }

        let mut res: u64 = 0;
        res = self.set_fractional_code(res, signif);
        res = self.set_exponent_code(res, exp as u64);
        self.set_sign(res, sgn)
    }

    /// Convert between two different formats: `encoding` is a value in the
    /// *other* FloatFormat `formin`; the result is the equivalent value in
    /// this FloatFormat.
    pub fn convert_encoding(&self, encoding: u64, formin: &FloatFormat) -> u64 {
        let sgn = formin.extract_sign(encoding);
        let mut signif = formin.extract_fractional_code(encoding);
        let mut exp = formin.extract_exponent_code(encoding);

        if exp == formin.maxexponent {
            // NaN or INFINITY encoding
            if signif != 0 {
                return self.get_nan_encoding(sgn);
            } else {
                return self.get_infinity_encoding(sgn);
            }
        }

        if exp == 0 {
            // incoming is subnormal
            if signif == 0 {
                return self.get_zero_encoding(sgn);
            }

            // normalize
            let lz = count_leading_zeros(signif);
            signif <<= lz;
            exp = -formin.bias - lz;
        } else {
            // incoming is normal
            exp -= formin.bias;
            if self.jbitimplied {
                signif = (1u64 << (8 * 8 - 1)) | (signif >> 1);
            }
        }

        exp += self.bias;

        if exp < -self.frac_size {
            // Exponent is too small to represent
            return self.get_zero_encoding(sgn); // TODO handle round to non-zero
        }

        if exp < 1 {
            // Must be denormalized
            if Self::round_to_nearest_even(&mut signif, 8 * 8 - self.frac_size - exp) {
                // TODO handle carry to normal case
                if (signif >> (8 * 8 - 1)) == 0 {
                    signif = 1u64 << (8 * 8 - 1);
                    exp += 1;
                }
            }
            let res = self.get_zero_encoding(sgn);
            // Same upstream UB cell as in get_encoding: the carry fixup above
            // can leave exp == 1 and the C++ `signif >> (-exp)` shifts by -1;
            // the x86-64 oracle masks the count to 63 so the signed zero
            // encoding comes back (convertEncoding fmt4<-fmt8 of
            // 0x380fffffffffffff == 0x0).  wrapping_shr reproduces the masked
            // shift, identical in debug and release (ADR 0003).
            return self.set_fractional_code(res, signif.wrapping_shr((-exp) as u32));
        }

        if Self::round_to_nearest_even(&mut signif, 8 * 8 - self.frac_size - 1) {
            // if high bit is clear, then the add overflowed. Increase exp and set
            // signif to 1.
            if (signif >> (8 * 8 - 1)) == 0 {
                signif = 1u64 << (8 * 8 - 1);
                exp += 1;
            }
        }

        if exp >= self.maxexponent {
            // Exponent is too big to represent
            return self.get_infinity_encoding(sgn);
        }

        if self.jbitimplied && (exp != 0) {
            signif <<= 1; // Cut off top bit (which should be 1)
        }

        let mut res: u64 = 0;
        res = self.set_fractional_code(res, signif);
        res = self.set_exponent_code(res, exp as u64);
        self.set_sign(res, sgn)
    }

    /// Print given value as a decimal string.
    ///
    /// The string should be printed with the minimum number of digits to
    /// uniquely specify the underlying binary value.  This currently only
    /// works for the 32-bit and 64-bit IEEE 754 formats.  If `forcesci` is
    /// true, the string will always be printed using scientific notation.
    /// `host` is the given value already converted to the host's double
    /// format.
    pub fn print_decimal(&self, host: f64, forcesci: bool) -> String {
        let res: String;
        let mut prec = self.decimal_min_precision;
        loop {
            let s = if forcesci {
                // Set to scientific notation; scientific doesn't include
                // first digit in precision count
                cfmt::ostream_scientific(host, prec - 1)
            } else {
                // Use "default" notation to allow fewer digits to be printed
                // if possible
                cfmt::ostream_general(host, prec)
            };
            if prec == self.decimal_max_precision {
                return s;
            }
            // C++ reads the string back with istream >> float / >> double
            // (strtof/strtod semantics, correctly rounded, same as
            // str::parse).  An extraction failure leaves the C++ roundtrip
            // variable at 0.0; Rust's parse additionally accepts "inf",
            // which short-circuits to the same final string.
            let roundtrip: f64 = if self.size <= 4 {
                f64::from(s.parse::<f32>().unwrap_or(0.0))
            } else {
                s.parse::<f64>().unwrap_or(0.0)
            };
            if roundtrip == host {
                res = s;
                break;
            }
            prec += 1;
        }
        res
    }

    // Currently we emulate floating point operations on the target
    // By converting the encoding to the host's encoding and then
    // performing the operation using the host's floating point unit
    // then the host's encoding is converted back to the targets encoding

    /// Equality comparison (==).
    pub fn op_equal(&self, a: u64, b: u64) -> u64 {
        let (val1, _type) = self.get_host_float(a);
        let (val2, _type) = self.get_host_float(b);
        u64::from(val1 == val2)
    }

    /// Inequality comparison (!=).
    pub fn op_not_equal(&self, a: u64, b: u64) -> u64 {
        let (val1, _type) = self.get_host_float(a);
        let (val2, _type) = self.get_host_float(b);
        u64::from(val1 != val2)
    }

    /// Less-than comparison (<).
    pub fn op_less(&self, a: u64, b: u64) -> u64 {
        let (val1, _type) = self.get_host_float(a);
        let (val2, _type) = self.get_host_float(b);
        u64::from(val1 < val2)
    }

    /// Less-than-or-equal comparison (<=).
    pub fn op_less_equal(&self, a: u64, b: u64) -> u64 {
        let (val1, _type) = self.get_host_float(a);
        let (val2, _type) = self.get_host_float(b);
        u64::from(val1 <= val2)
    }

    /// Test if Not-a-Number (NaN).
    pub fn op_nan(&self, a: u64) -> u64 {
        let (_val, tp) = self.get_host_float(a);
        u64::from(tp == floatclass::nan)
    }

    /// Encode a host FP result, pinning the sign of an *invalid-operation-
    /// generated* NaN to the x86 "real indefinite" (`0xffc00000` / sign set),
    /// so the engine is deterministic across build hosts and byte-identical to
    /// the Linux/x86 golden oracle everywhere (see the module header).
    ///
    /// `input_was_nan` is true when any operand already decoded to a NaN — then
    /// the result is a *propagated* NaN whose sign is host-deterministic (it
    /// follows the operand: the `0x7fc00000` propagation rows of the golden
    /// vectors), so it is left untouched.  On the x86 oracle host the fixup is a
    /// no-op (the host already yields `0xffc00000`), so x86 output is unchanged.
    fn encode_generated(&self, host: f64, input_was_nan: bool) -> u64 {
        let enc = self.get_encoding(host);
        if !input_was_nan && self.get_class(enc) == floatclass::nan {
            return self.get_nan_encoding(true);
        }
        enc
    }

    /// Addition (+).
    pub fn op_add(&self, a: u64, b: u64) -> u64 {
        let (val1, t1) = self.get_host_float(a);
        let (val2, t2) = self.get_host_float(b);
        self.encode_generated(val1 + val2, t1 == floatclass::nan || t2 == floatclass::nan)
    }

    /// Division (/).
    pub fn op_div(&self, a: u64, b: u64) -> u64 {
        let (val1, t1) = self.get_host_float(a);
        let (val2, t2) = self.get_host_float(b);
        self.encode_generated(val1 / val2, t1 == floatclass::nan || t2 == floatclass::nan)
    }

    /// Multiplication (*).
    pub fn op_mult(&self, a: u64, b: u64) -> u64 {
        let (val1, t1) = self.get_host_float(a);
        let (val2, t2) = self.get_host_float(b);
        self.encode_generated(val1 * val2, t1 == floatclass::nan || t2 == floatclass::nan)
    }

    /// Subtraction (-).
    pub fn op_sub(&self, a: u64, b: u64) -> u64 {
        let (val1, t1) = self.get_host_float(a);
        let (val2, t2) = self.get_host_float(b);
        self.encode_generated(val1 - val2, t1 == floatclass::nan || t2 == floatclass::nan)
    }

    /// Unary negate.
    pub fn op_neg(&self, a: u64) -> u64 {
        let (val, _type) = self.get_host_float(a);
        self.get_encoding(-val)
    }

    /// Absolute value (abs).
    pub fn op_abs(&self, a: u64) -> u64 {
        let (val, _type) = self.get_host_float(a);
        self.get_encoding(val.abs())
    }

    /// Square root (sqrt).
    pub fn op_sqrt(&self, a: u64) -> u64 {
        let (val, tp) = self.get_host_float(a);
        self.encode_generated(val.sqrt(), tp == floatclass::nan)
    }

    /// Convert integer to floating-point: `a` is a signed integer value,
    /// `sizein` is the number of bytes in the integer encoding.
    pub fn op_int2float(&self, a: u64, sizein: i32) -> u64 {
        // C++ passes the uintb through an implicit intb conversion into
        // sign_extend (two's-complement reinterpretation, i.e. `as i64`).
        let ival = sign_extend(a as i64, 8 * sizein - 1);
        let val = ival as f64; // Convert integer to float
        self.get_encoding(val)
    }

    /// Convert between floating-point precisions.
    pub fn op_float2float(&self, a: u64, outformat: &FloatFormat) -> u64 {
        outformat.convert_encoding(a, self)
    }

    /// Convert floating-point to integer; `sizeout` is the desired encoding
    /// size of the output.
    pub fn op_trunc(&self, a: u64, sizeout: i32) -> u64 {
        let (val, _type) = self.get_host_float(a);
        let ival = host_double_to_int64(val); // Convert to integer
        let mut res = ival as u64; // Convert to unsigned
        res &= calc_mask(sizeout); // Truncate to proper size
        res
    }

    /// Ceiling (ceil).
    pub fn op_ceil(&self, a: u64) -> u64 {
        let (val, _type) = self.get_host_float(a);
        self.get_encoding(val.ceil())
    }

    /// Floor (floor).
    pub fn op_floor(&self, a: u64) -> u64 {
        let (val, _type) = self.get_host_float(a);
        self.get_encoding(val.floor())
    }

    /// Round.
    pub fn op_round(&self, a: u64) -> u64 {
        let (val, _type) = self.get_host_float(a);
        self.get_encoding(val.round()) // round half away from zero
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Edge tests owned by this item.  The C++ TEST(...) suites of
    // testfloatemu.cc are ported by a different item; names here are
    // deliberately distinct.

    #[test]
    #[allow(clippy::approx_constant)] // literal from kuna_goldengen.cc, not PI
    fn test_edge_ldexp_frexp_roundtrip() {
        // frexp/ldexp transcriptions: exact roundtrip across the range,
        // including subnormals.
        for &v in &[
            1.0,
            0.5,
            3.14159265358979,
            1e-300,
            1e300,
            f64::MIN_POSITIVE,
            5e-324,
            f64::MAX,
            1.401298464324817e-45,
        ] {
            let (m, e) = frexp(v);
            assert!((0.5..1.0).contains(&m), "frexp({v}) mantissa {m}");
            assert_eq!(ldexp(m, e), v, "roundtrip of {v}");
        }
        // ldexp rounding into the subnormal range: 2^-1074 is the smallest
        // subnormal.  1.0*2^-1075 is an exact tie -> rounds to even (zero);
        // 1.5*2^-1075 is above the midpoint -> rounds up to 2^-1074.
        assert_eq!(ldexp(1.0, -1074), 5e-324);
        assert_eq!(ldexp(1.0, -1075), 0.0);
        assert_eq!(ldexp(1.5, -1075), 5e-324);
        assert_eq!(ldexp(1.0, 1024), f64::INFINITY);
        assert_eq!(ldexp(-1.0, 1024), f64::NEG_INFINITY);
    }

    #[test]
    fn test_edge_both_zeros() {
        let fmt4 = FloatFormat::new(4);
        let fmt8 = FloatFormat::new(8);
        assert_eq!(fmt4.get_encoding(0.0), 0x0000_0000);
        assert_eq!(fmt4.get_encoding(-0.0), 0x8000_0000);
        assert_eq!(fmt8.get_encoding(-0.0), 0x8000_0000_0000_0000);
        assert_eq!(fmt4.get_class(0x8000_0000), floatclass::zero);
        let (v, tp) = fmt4.get_host_float(0x8000_0000);
        assert_eq!(tp, floatclass::zero);
        assert!(v == 0.0 && v.is_sign_negative());
        // -0 + +0 == +0 ; -0 + -0 == -0 (IEEE round-to-nearest)
        assert_eq!(fmt4.op_add(0x8000_0000, 0x0000_0000), 0x0000_0000);
        assert_eq!(fmt4.op_add(0x8000_0000, 0x8000_0000), 0x8000_0000);
        // 0 == -0 compares equal
        assert_eq!(fmt4.op_equal(0x0, 0x8000_0000), 1);
        assert_eq!(fmt4.op_less(0x8000_0000, 0x0), 0);
    }

    #[test]
    fn test_edge_subnormal_encode_decode() {
        let fmt4 = FloatFormat::new(4);
        // Smallest and largest f32 subnormals roundtrip through the host.
        for enc in [0x1u64, 0x7f_ffffu64, 0x2u64, 0x40_0000u64] {
            let (host, tp) = fmt4.get_host_float(enc);
            assert_eq!(tp, floatclass::denormalized, "enc {enc:#x}");
            assert_eq!(fmt4.get_encoding(host), enc, "enc {enc:#x}");
        }
        // f64 subnormals
        let fmt8 = FloatFormat::new(8);
        for enc in [0x1u64, 0xf_ffff_ffff_ffffu64] {
            let (host, tp) = fmt8.get_host_float(enc);
            assert_eq!(tp, floatclass::denormalized, "enc {enc:#x}");
            assert_eq!(fmt8.get_encoding(host), enc, "enc {enc:#x}");
        }
        // A double too small for float collapses to (signed) zero.
        assert_eq!(fmt4.get_encoding(5e-324), 0x0);
        assert_eq!(fmt4.get_encoding(-5e-324), 0x8000_0000);
        // float2float 8->4 of a value inside f32's subnormal range.
        let sub32 = fmt8.get_encoding(f64::from(f32::from_bits(0x0040_0000)));
        assert_eq!(fmt8.op_float2float(sub32, &fmt4), 0x0040_0000);
    }

    #[test]
    fn test_edge_inf_nan_arithmetic() {
        let fmt4 = FloatFormat::new(4);
        const PINF: u64 = 0x7f80_0000;
        const NINF: u64 = 0xff80_0000;
        const QNAN: u64 = 0x7fc0_0000;
        const NQNAN: u64 = 0xffc0_0000;
        // inf + inf = inf ; inf + -inf = NaN (x86 default QNaN, negative)
        assert_eq!(fmt4.op_add(PINF, PINF), PINF);
        assert_eq!(fmt4.op_add(PINF, NINF), NQNAN);
        assert_eq!(fmt4.op_sub(PINF, PINF), NQNAN);
        // 0 * inf, 0/0, inf/inf are NaN
        assert_eq!(fmt4.op_mult(0x0, PINF), NQNAN);
        assert_eq!(fmt4.op_div(0x0, 0x0), NQNAN);
        assert_eq!(fmt4.op_div(PINF, PINF), NQNAN);
        // 1/0 = inf, -1/0 = -inf
        let one = fmt4.get_encoding(1.0);
        let neg_one = fmt4.get_encoding(-1.0);
        assert_eq!(fmt4.op_div(one, 0x0), PINF);
        assert_eq!(fmt4.op_div(neg_one, 0x0), NINF);
        // sqrt(-1) is the negative default QNaN; sqrt(-0) is -0.
        assert_eq!(fmt4.op_sqrt(neg_one), NQNAN);
        assert_eq!(fmt4.op_sqrt(0x8000_0000), 0x8000_0000);
        // NaN propagates (canonicalized, sign preserved through the host)
        assert_eq!(fmt4.op_add(QNAN, one), QNAN);
        assert_eq!(fmt4.op_nan(QNAN), 1);
        assert_eq!(fmt4.op_nan(PINF), 0);
        // NaN compares: == false, != true, < false, <= false
        assert_eq!(fmt4.op_equal(QNAN, QNAN), 0);
        assert_eq!(fmt4.op_not_equal(QNAN, QNAN), 1);
        assert_eq!(fmt4.op_less(QNAN, one), 0);
        assert_eq!(fmt4.op_less_equal(QNAN, QNAN), 0);
    }

    #[test]
    fn test_edge_trunc_x86_cast_semantics() {
        let fmt8 = FloatFormat::new(8);
        let fmt4 = FloatFormat::new(4);
        // NaN / inf / out-of-range all produce 0x8000000000000000 (cvttsd2si)
        assert_eq!(fmt8.op_trunc(0x7ff8_0000_0000_0000, 8), 0x8000_0000_0000_0000);
        assert_eq!(fmt8.op_trunc(0x7ff0_0000_0000_0000, 8), 0x8000_0000_0000_0000);
        assert_eq!(fmt8.op_trunc(0xfff0_0000_0000_0000, 8), 0x8000_0000_0000_0000);
        assert_eq!(fmt8.op_trunc(fmt8.get_encoding(1e300), 8), 0x8000_0000_0000_0000);
        assert_eq!(fmt4.op_trunc(0x7f7f_ffff, 8), 0x8000_0000_0000_0000); // 3.4e38
        // ... and the same bit pattern masked for smaller outputs.
        assert_eq!(fmt8.op_trunc(0x7ff8_0000_0000_0000, 4), 0x0);
        assert_eq!(fmt8.op_trunc(0xfff0_0000_0000_0000, 1), 0x0);
        // In-range truncation goes toward zero; negatives wrap into the mask.
        assert_eq!(fmt8.op_trunc(fmt8.get_encoding(-1.0), 2), 0xffff);
        assert_eq!(fmt8.op_trunc(fmt8.get_encoding(-2.7), 8), (-2i64) as u64);
        assert_eq!(fmt8.op_trunc(fmt8.get_encoding(2.7), 1), 0x2);
        // -2^63 is exactly representable and in range.
        assert_eq!(
            fmt8.op_trunc(fmt8.get_encoding(-9.223372036854776e18), 8),
            0x8000_0000_0000_0000
        );
    }

    #[test]
    fn test_edge_int2float_sign_extension() {
        let fmt4 = FloatFormat::new(4);
        // 0xff as a 1-byte signed integer is -1 ...
        assert_eq!(fmt4.op_int2float(0xff, 1), fmt4.get_encoding(-1.0));
        // ... but as a 2-byte signed integer it is 255.
        assert_eq!(fmt4.op_int2float(0xff, 2), fmt4.get_encoding(255.0));
        // i64 min
        let fmt8 = FloatFormat::new(8);
        assert_eq!(
            fmt8.op_int2float(0x8000_0000_0000_0000, 8),
            fmt8.get_encoding(-9.223372036854776e18)
        );
    }

    #[test]
    fn test_edge_print_decimal_roundtrip() {
        // Shortest-roundtrip selection (testfloatemu-style values, distinct
        // assertion set -- that suite is ported by another item).
        let ff4 = FloatFormat::new(4);
        assert_eq!(ff4.print_decimal(f64::from(0.25f32), false), "0.25");
        assert_eq!(
            ff4.print_decimal(f64::from(1.0f32 / 3.0f32), false),
            "0.33333334"
        );
        let ff8 = FloatFormat::new(8);
        assert_eq!(ff8.print_decimal(0.1, false), "0.1");
        assert_eq!(ff8.print_decimal(1.0 / 6.0, false), "0.16666666666666666");
        assert_eq!(ff8.print_decimal(0.123, true), "1.23000000000000e-01");
        // Non-finite values format as glibc text in every iteration.
        assert_eq!(ff8.print_decimal(f64::INFINITY, false), "inf");
        assert_eq!(ff8.print_decimal(f64::NAN, false), "nan");
    }

    #[test]
    fn test_edge_get_class_table() {
        let fmt4 = FloatFormat::new(4);
        assert_eq!(fmt4.get_class(0x0), floatclass::zero);
        assert_eq!(fmt4.get_class(0x8000_0000), floatclass::zero);
        assert_eq!(fmt4.get_class(0x1), floatclass::denormalized);
        assert_eq!(fmt4.get_class(0x0080_0000), floatclass::normalized);
        assert_eq!(fmt4.get_class(0x7f80_0000), floatclass::infinity);
        assert_eq!(fmt4.get_class(0x7fc0_0000), floatclass::nan);
        assert_eq!(fmt4.get_class(0xff80_0001), floatclass::nan); // signaling-bit pattern
        assert_eq!(fmt4.get_exponent(0x3f80_0000), 0);
    }
}
