//! C++ iostream `double` formatting (the oracle's `operator<<(ostream&,double)`).
//!
//! The C++ decompiler renders floating-point decimals with `ostringstream`
//! (see `FloatFormat::printDecimal` in `decompiler/cpp/float.cc` and the
//! golden generator `decompiler/cpp/kuna_goldengen.cc`).  libstdc++ routes
//! `operator<<(double)` through `vsnprintf`, so the observable behavior is
//! the C `%g` / `%e` conversion family:
//!
//! - default stream state ("default" / general notation, precision 6) is
//!   `%g` with precision 6,
//! - `s.unsetf(ios::floatfield); s.precision(p)` is `%g` with precision `p`,
//! - `s.setf(ios::scientific); s.precision(p)` is `%e` with precision `p`.
//!
//! `%g` semantics with precision P (C17 7.21.6.1, glibc):
//! - a precision of zero is taken as 1; a negative precision means
//!   "missing", i.e. the default 6,
//! - the value is converted as `%e` with precision P-1; let X be the decimal
//!   exponent of that conversion (i.e. AFTER rounding to P significant
//!   digits),
//! - if P > X >= -4 the value is re-rendered in fixed (`%f`) style with
//!   precision P-1-X, otherwise the `%e` form is kept,
//! - trailing zeros are removed from the fractional portion, and a trailing
//!   decimal point is removed (the oracle's streams never set `showpoint`).
//!
//! Infinities and NaNs print as glibc prints them: `inf`, `-inf`, `nan`,
//! `-nan`.
//!
//! Digit generation leans on Rust's `format!`, which performs exact decimal
//! conversion with round-to-nearest, ties-to-even -- the same rounding glibc
//! applies -- so this module only reproduces the C exponent syntax
//! (`e+XX` with a sign and at least two digits) and the `%g` trimming rules.
//!
//! The `dec` rows of `tests/golden/vectors/float.csv` pin
//! [`ostream_default`] against the C++ oracle byte-for-byte.

/// `os << val` with default stream state: general notation, precision 6.
pub fn ostream_default(val: f64) -> String {
    ostream_general(val, 6)
}

/// `os.unsetf(ios::floatfield); os.precision(precision); os << val`
/// (`%g` with the given precision).
pub fn ostream_general(val: f64, precision: i32) -> String {
    if let Some(s) = special_text(val) {
        return s;
    }
    // %g: precision 0 is taken as 1, negative precision as the default 6.
    let p = if precision < 0 {
        6
    } else if precision == 0 {
        1
    } else {
        precision
    };
    // Convert as %e with precision P-1; the exponent of that conversion is
    // the post-rounding decimal exponent X the style decision keys on.
    let e_form = format!("{:.*e}", (p - 1) as usize, val);
    let (mantissa, x) = split_exponent(&e_form);
    if p > x && x >= -4 {
        // Fixed style with precision P-1-X (>= 0 because X <= P-1).  This is
        // a single rounding of the original value to the same decimal place
        // %g rounds to, so the digits agree with the %e conversion above.
        let fixed = format!("{:.*}", (p - 1 - x) as usize, val);
        trim_zeros(&fixed)
    } else {
        format!("{}e{}", trim_zeros(mantissa), exponent_text(x))
    }
}

/// `os.setf(ios::scientific); os.precision(precision); os << val`
/// (`%e` with the given precision: `precision` digits after the decimal
/// point, no trailing-zero trimming).
pub fn ostream_scientific(val: f64, precision: i32) -> String {
    if let Some(s) = special_text(val) {
        return s;
    }
    // %e: a negative precision is treated as missing (default 6).
    let p = if precision < 0 { 6 } else { precision };
    let e_form = format!("{:.*e}", p as usize, val);
    let (mantissa, x) = split_exponent(&e_form);
    format!("{}e{}", mantissa, exponent_text(x))
}

/// glibc text forms for non-finite values (sign included for negatives).
fn special_text(val: f64) -> Option<String> {
    if val.is_nan() {
        return Some(if val.is_sign_negative() { "-nan" } else { "nan" }.to_string());
    }
    if val.is_infinite() {
        return Some(if val.is_sign_negative() { "-inf" } else { "inf" }.to_string());
    }
    None
}

/// Split Rust's `LowerExp` output (`d.ddde-7` / `d.ddde15`) into the mantissa
/// text and the numeric decimal exponent.
fn split_exponent(e_form: &str) -> (&str, i32) {
    let pos = e_form
        .rfind('e')
        .unwrap_or_else(|| panic!("no exponent in {e_form:?}"));
    let mantissa = &e_form[..pos];
    let x: i32 = e_form[pos + 1..]
        .parse()
        .unwrap_or_else(|err| panic!("bad exponent in {e_form:?}: {err}"));
    (mantissa, x)
}

/// C `%e` exponent syntax: explicit sign, at least two digits.
fn exponent_text(x: i32) -> String {
    let sign = if x < 0 { '-' } else { '+' };
    // x is at most +-1074 for f64; unsigned_abs avoids any i32::MIN edge.
    format!("{}{:02}", sign, x.unsigned_abs())
}

/// `%g` trimming: drop trailing zeros of the fractional part, then a
/// trailing decimal point.  Strings without a '.' pass through untouched.
fn trim_zeros(s: &str) -> String {
    if !s.contains('.') {
        return s.to_string();
    }
    s.trim_end_matches('0').trim_end_matches('.').to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::approx_constant)] // literal from kuna_goldengen.cc, not PI
    fn test_cfmt_default_basic() {
        // The dec-row forms of tests/golden/vectors/float.csv.
        assert_eq!(ostream_default(0.0), "0");
        assert_eq!(ostream_default(-0.0), "-0");
        assert_eq!(ostream_default(1.0), "1");
        assert_eq!(ostream_default(-1.0), "-1");
        assert_eq!(ostream_default(0.5), "0.5");
        assert_eq!(ostream_default(3.14159265358979), "3.14159");
        assert_eq!(ostream_default(1e-10), "1e-10");
        assert_eq!(ostream_default(1e10), "1e+10");
    }

    #[test]
    fn test_cfmt_default_boundaries_of_scientific() {
        // %g switches to scientific when X < -4 or X >= P (P == 6).
        assert_eq!(ostream_default(0.0001), "0.0001"); // X == -4: fixed
        assert_eq!(ostream_default(0.00001), "1e-05"); // X == -5: scientific
        assert_eq!(ostream_default(100000.0), "100000"); // X == 5: fixed
        assert_eq!(ostream_default(1000000.0), "1e+06"); // X == 6: scientific
        // Rounding can bump the exponent across the boundary.
        assert_eq!(ostream_default(999999.9), "1e+06");
        assert_eq!(ostream_default(0.000099999999), "0.0001");
    }

    #[test]
    fn test_cfmt_default_subnormals_and_extremes() {
        assert_eq!(ostream_default(f64::MIN_POSITIVE), "2.22507e-308");
        assert_eq!(ostream_default(5e-324), "4.94066e-324"); // smallest subnormal
        assert_eq!(ostream_default(f64::MAX), "1.79769e+308");
        assert_eq!(ostream_default(1.401298464324817e-45), "1.4013e-45");
    }

    #[test]
    fn test_cfmt_special_values() {
        assert_eq!(ostream_default(f64::INFINITY), "inf");
        assert_eq!(ostream_default(f64::NEG_INFINITY), "-inf");
        assert_eq!(ostream_default(f64::NAN), "nan");
        assert_eq!(ostream_default(-f64::NAN), "-nan");
        assert_eq!(ostream_scientific(f64::INFINITY, 14), "inf");
        assert_eq!(ostream_scientific(-f64::NAN, 14), "-nan");
        assert_eq!(ostream_general(f64::NEG_INFINITY, 17), "-inf");
    }

    #[test]
    fn test_cfmt_general_precision_sweep() {
        // The printDecimal precision ladder (float.cc) for f32 third = 1/3.
        let third32 = f64::from(1.0f32 / 3.0f32);
        assert_eq!(ostream_general(third32, 6), "0.333333");
        assert_eq!(ostream_general(third32, 8), "0.33333334");
        // f64 sixth at max double precision.
        assert_eq!(ostream_general(1.0 / 6.0, 17), "0.16666666666666666");
        assert_eq!(ostream_general(f64::MAX, 15), "1.79769313486232e+308");
        assert_eq!(ostream_general(0.1, 15), "0.1");
        assert_eq!(ostream_general(0.25, 6), "0.25");
        // Precision 0 is taken as 1.
        assert_eq!(ostream_general(1234.0, 0), "1e+03");
    }

    #[test]
    fn test_cfmt_scientific_keeps_trailing_zeros() {
        // ios::scientific does NOT trim (testfloatemu's forcesci case).
        assert_eq!(ostream_scientific(0.123, 14), "1.23000000000000e-01");
        assert_eq!(ostream_scientific(1.0, 5), "1.00000e+00");
        assert_eq!(ostream_scientific(-0.0, 2), "-0.00e+00");
        assert_eq!(ostream_scientific(1e100, 0), "1e+100");
    }

    #[test]
    fn test_cfmt_ties_round_to_even() {
        // glibc (and Rust) round the exact decimal expansion to nearest,
        // ties to even.
        assert_eq!(ostream_general(2.5, 1), "2");
        assert_eq!(ostream_general(3.5, 1), "4");
        assert_eq!(ostream_general(0.125, 2), "0.12");
        assert_eq!(ostream_scientific(0.125, 1), "1.2e-01");
    }
}
