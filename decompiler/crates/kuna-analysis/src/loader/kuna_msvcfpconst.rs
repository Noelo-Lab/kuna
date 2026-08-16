//! (kuna) MSVC `__real@` floating-point constant COMDATs: recover the value from
//! the mangled symbol name.
//!
//! # The defect
//!
//! MSVC does not encode a floating-point literal into the instruction stream —
//! x87 and SSE both load one from memory. It emits the literal as a COMDAT with
//! the value spelled out in the symbol name:
//!
//! ```text
//! __real@8@3fe08000000200000000     (sec  0)   <- UNDEFINED in this object
//! __real@8@4000c000000000000000     (sec 13)   <- defined here
//! ```
//!
//! COMDAT folding means only one translation unit keeps the definition, so in
//! any *other* object the symbol is **undefined**: no section, no bytes, nothing
//! for the loader to map. kuna binds it to a synthetic extern slot so a
//! reference resolves, and the decompiled C then reads
//! `... * dat_402020 + dat_402040`, where every operand of the floating-point
//! expression is an opaque address.
//!
//! # The recovery
//!
//! The name is not a label, it is the value. `__real@<size>@<hex>` spells the
//! constant as an x87 80-bit extended datum — a 16-bit sign/exponent field
//! followed by a 64-bit mantissa **with its explicit integer bit** — and the
//! `<size>` between the `@`s is the storage width the program actually loads (4
//! for `float`, 8 for `double`). The older `__real@<hex>` form spells the IEEE
//! double bits directly. Decoding the name therefore reconstructs the datum
//! exactly, with no need for the defining object.
//!
//! Verified against the constants in a Command & Conquer `bounce.obj`:
//! `3ffec90fdaa22168c000` is π/4, `4001c90fdaa22168c000` is 2π,
//! `3fe08000000200000000` is 2⁻³¹ (the scale that turns a 31-bit random integer
//! into a fraction), `3fef8000800000000000` is 2⁻¹⁶.
//!
//! # Scope
//!
//! Only `__real@`. MSVC's `__xmm@…` (16-byte SSE) and `__ymm@…` constants use
//! the same idea with a different payload and are deliberately not handled here:
//! a wrong 16-byte datum is worse than an honest `dat_<addr>`.

/// A decoded MSVC floating-point constant: the little-endian bytes to store, at
/// the storage width the program loads.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FpConst {
    /// The datum as it appears in memory (little-endian), `size` bytes long.
    pub bytes: Vec<u8>,
    /// Storage width: 4 (`float`) or 8 (`double`).
    pub size: usize,
}

/// Decode an x87 80-bit extended datum, spelled big-endian as 20 hex digits:
/// a 16-bit sign/exponent field then a 64-bit mantissa carrying its explicit
/// integer bit.
///
/// Returned as `f64`, which is lossless for every literal a C program can write:
/// the source constant was a `float` or a `double` before the assembler widened
/// it to the x87 format, and the `<size>` field says which.
fn x87_extended_to_f64(hex: &str) -> Option<f64> {
    if hex.len() != 20 {
        return None;
    }
    let se = u16::from_str_radix(&hex[..4], 16).ok()?;
    let mantissa = u64::from_str_radix(&hex[4..], 16).ok()?;
    let sign = if se & 0x8000 != 0 { -1.0 } else { 1.0 };
    let exponent = (se & 0x7fff) as i32;
    if exponent == 0 && mantissa == 0 {
        return Some(0.0 * sign);
    }
    // The mantissa is an unnormalized 64-bit integer whose top bit is the
    // explicit leading 1, so its scale is 2^(exp - bias - 63).
    Some(sign * (mantissa as f64) * (2.0f64).powi(exponent - 16383 - 63))
}

/// Decode an MSVC `__real@…` symbol name, or `None` if it is not one.
///
/// Two spellings:
///   * `__real@<size>@<20 hex>` — an x87 80-bit datum plus the storage width
///     the program loads it at (`4` = float, `8` = double).
///   * `__real@<16 hex>` — the IEEE-754 double bits directly, big-endian.
pub fn decode(name: &str) -> Option<FpConst> {
    let rest = name.strip_prefix("__real@")?;
    if let Some((size_str, hex)) = rest.split_once('@') {
        let size: usize = size_str.parse().ok()?;
        if size != 4 && size != 8 {
            return None;
        }
        if !hex.chars().all(|c| c.is_ascii_hexdigit()) {
            return None;
        }
        let v = x87_extended_to_f64(hex)?;
        let bytes = if size == 4 {
            (v as f32).to_le_bytes().to_vec()
        } else {
            v.to_le_bytes().to_vec()
        };
        return Some(FpConst { bytes, size });
    }
    // `__real@<16 hex>`: the IEEE double bits, big-endian.
    if rest.len() == 16 && rest.chars().all(|c| c.is_ascii_hexdigit()) {
        let bits = u64::from_str_radix(rest, 16).ok()?;
        return Some(FpConst { bytes: bits.to_le_bytes().to_vec(), size: 8 });
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Round-trip the decoded bytes back to the number they encode.
    fn value_of(c: &FpConst) -> f64 {
        if c.size == 4 {
            f32::from_le_bytes(c.bytes[..4].try_into().unwrap()) as f64
        } else {
            f64::from_le_bytes(c.bytes[..8].try_into().unwrap())
        }
    }

    /// The real constants out of a Command & Conquer `bounce.obj`. These are not
    /// synthetic: each is a recognisable literal from the original source, which
    /// is what makes them a sound oracle for the decoder.
    #[test]
    fn decodes_the_constants_from_a_real_msvc_object() {
        let cases: &[(&str, f64)] = &[
            ("__real@8@3fff8000000000000000", 1.0),
            ("__real@8@4000a000000000000000", 2.5),
            ("__real@8@4000c000000000000000", 3.0),
            ("__real@8@00000000000000000000", 0.0),
            ("__real@8@3ffec90fdaa22168c000", std::f64::consts::FRAC_PI_4),
            ("__real@8@4001c90fdaa22168c000", std::f64::consts::TAU),
            // 2^-31: the scale that turns a 31-bit random integer into [0,1).
            ("__real@8@3fe08000000200000000", 4.656612877414201e-10),
        ];
        for (name, want) in cases {
            let c = decode(name).unwrap_or_else(|| panic!("{name} must decode"));
            assert_eq!(c.size, 8, "{name}");
            let got = value_of(&c);
            assert!(
                (got - want).abs() <= want.abs() * 1e-12 + 1e-300,
                "{name}: got {got}, want {want}"
            );
        }
    }

    /// The `@4@` form stores a 4-byte `float`, so the datum must be narrowed --
    /// storing 8 bytes would overwrite the neighbouring slot and read back as a
    /// wildly different number.
    #[test]
    fn the_size_field_selects_the_storage_width() {
        // 2^-16, spelled as an 80-bit datum but stored as a float.
        let c = decode("__real@4@3fef8000800000000000").expect("must decode");
        assert_eq!(c.size, 4);
        assert_eq!(c.bytes.len(), 4);
        assert!((value_of(&c) - 1.5259021896696368e-05).abs() < 1e-11);

        let c = decode("__real@8@40069600000000000000").expect("must decode");
        assert_eq!(c.size, 8);
        assert_eq!(c.bytes.len(), 8);
        assert_eq!(value_of(&c), 150.0);
    }

    /// Sign is carried by the top bit of the exponent field.
    #[test]
    fn negative_constants_decode() {
        let c = decode("__real@8@bff39d495182a9930800").expect("must decode");
        assert!((value_of(&c) - -0.0003).abs() < 1e-15, "got {}", value_of(&c));
    }

    /// The older direct-IEEE spelling.
    #[test]
    fn the_bare_ieee_double_form_decodes() {
        let c = decode("__real@3ff0000000000000").expect("must decode");
        assert_eq!(c.size, 8);
        assert_eq!(value_of(&c), 1.0);
    }

    /// Anything that is not this mangling must be refused rather than guessed --
    /// a wrong constant is worse than an opaque address. `__xmm@` in particular
    /// is a real MSVC symbol with a different payload.
    #[test]
    fn non_fp_constant_symbols_are_refused() {
        for name in [
            "__ftol",
            "main",
            "__real",
            "__real@",
            "__real@8@",                        // no payload
            "__real@8@3fff80000000000000",      // 18 digits, not 20
            "__real@8@zzzzzzzzzzzzzzzzzzzz",    // not hex
            "__real@16@3fff8000000000000000",   // no 16-byte float storage
            "__xmm@00000000000000000000000000000000",
            "__real@3ff00000000000",            // 14 digits, not 16
        ] {
            assert!(decode(name).is_none(), "{name} must NOT decode");
        }
    }
}
