//! (kuna `msvcfpconst`) MSVC `__real@` floating-point constant COMDATs: recover
//! the value from the mangled symbol name, and let the engine fold it.
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
//! expression is an opaque address. The *defined* half is no better at the
//! shipped defaults: its bytes are present and read-only, but folding a
//! read-only global into a literal is gated by the program-wide `option
//! readonly`, which is default-off.
//!
//! # The recovery
//!
//! The name is not a label, it is the value, in one of three spellings:
//!
//! | spelling | payload | width |
//! |---|---|---|
//! | `__real@<size>@<20 hex>` | an x87 80-bit extended datum | `<size>`: 4 = `float`, 8 = `double` |
//! | `__real@<16 hex>` | IEEE-754 **double** bits, big-endian | 8 |
//! | `__real@<8 hex>` | IEEE-754 **float** bits, big-endian | 4 |
//!
//! The 20-hex form is the VC6-era one (a 16-bit sign/exponent field followed by
//! a 64-bit mantissa carrying its **explicit** integer bit, with `<size>` giving
//! the storage width the program actually loads); the bare-bits forms are what
//! modern MSVC emits, and today's compiler spells `1.0f` `__real@3f800000` — 8
//! hex digits of raw `float` bits, not 16. Decoding the name reconstructs the
//! datum exactly, with no need for the defining object.
//!
//! Verified against the constants in a Command & Conquer `bounce.obj` — every
//! one a recognisable literal from the original source, which is what makes it a
//! sound oracle. See the module tests.
//!
//! # What is done with it
//!
//! [`plan`] returns two things per object:
//!
//! * **writes** — the decoded bytes for each *undefined* `__real@` symbol, to be
//!   materialised at the synthetic extern slot the layout bound it to. Without
//!   them that address has no backing at all and every read of it fails.
//! * **const_ranges** — the `[start, stop]` (inclusive) extent of each such slot
//!   *and* of each **defined** `__real@` COMDAT, pushed onto
//!   `ObjectLoadImage::dynreloc_const`: the "these particular ranges fold even
//!   with `readonlypropagate` off" exception list. Both halves must be on the
//!   list or the output is half-folded — one operand a literal, its neighbour
//!   still `dat_<addr>`, which reads worse than either extreme.
//!
//! A defined COMDAT's bytes are **cross-checked against its own name** before its
//! range is admitted. That is also what keeps the ET_REL fidelity hazard (a
//! read-only section in a `.o` holds *pre*-relocation bytes) away from this
//! path: a `__real@` COMDAT carries no relocation, and if the bytes and the name
//! ever disagree the range is dropped rather than folded.
//!
//! # Scope
//!
//! Only `__real@`. MSVC's `__xmm@…` (16-byte SSE) and `__ymm@…` constants use
//! the same idea with a different payload and are deliberately not handled here:
//! a wrong 16-byte datum is worse than an honest `dat_<addr>`. Everything that
//! is not exactly one of the three spellings above is refused, including an x87
//! Inf/NaN or (pseudo-)denormal encoding, which has no faithful `f64` image.
//!
//! Gated by `--option msvcfpconst on|off` (default **on**, DIV-96) through the
//! [`kuna_decomp::kuna_msvcfpconst`] env bridge — the bytes are materialised
//! inside `load file`, upstream of the per-function option machinery.

use object::read::{Object, ObjectSection, ObjectSymbol};
use object::SymbolSection;

use super::reloc_object::RelocLayout;

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
/// Only the two encodings a compiler ever emits are accepted — a true zero and a
/// **normalized** finite value (biased exponent `1..=0x7ffe` with the integer bit
/// set). Every other encoding class is refused rather than approximated:
///
/// * `exponent == 0x7fff` is an Inf/NaN/pseudo-infinity, which has no faithful
///   `f64` image (the naive scaling silently produced `+inf`);
/// * `exponent == 0` with a non-zero mantissa is a denormal or a deprecated
///   pseudo-denormal, whose true scale is `2^(-16382-63)` — one binade away from
///   the normalized formula, so the naive scaling was wrong by a factor of two;
/// * an *unnormal* (non-zero exponent, integer bit clear) is invalid on every
///   x87 since the 387.
///
/// Returned as `f64`, which is lossless for every literal a C program can write:
/// the source constant was a `float` or a `double` before the assembler widened
/// it to the x87 format, so at most 53 of the mantissa's 64 bits are set.
fn x87_extended_to_f64(hex: &str) -> Option<f64> {
    if hex.len() != 20 {
        return None;
    }
    let se = u16::from_str_radix(&hex[..4], 16).ok()?;
    let mantissa = u64::from_str_radix(&hex[4..], 16).ok()?;
    let sign = if se & 0x8000 != 0 { -1.0 } else { 1.0 };
    let exponent = (se & 0x7fff) as i32;
    if exponent == 0 {
        return if mantissa == 0 { Some(0.0 * sign) } else { None };
    }
    if exponent == 0x7fff || mantissa >> 63 == 0 {
        return None;
    }
    // The mantissa is a 64-bit integer whose top bit is the explicit leading 1,
    // so its scale is 2^(exp - bias - 63).
    let value = sign * (mantissa as f64) * (2.0f64).powi(exponent - 16383 - 63);
    // Out of `f64` range in either direction: `powi` saturates to inf or 0, and
    // a saturated 0 would be a wrong datum rather than a refusal.
    if !value.is_finite() || value == 0.0 {
        return None;
    }
    Some(value)
}

/// Decode an MSVC `__real@…` symbol name, or `None` if it is not one.
///
/// Three spellings, all big-endian: `__real@<size>@<20 hex>` (an x87 80-bit
/// datum plus the storage width the program loads it at, `4` = float, `8` =
/// double), `__real@<16 hex>` (IEEE-754 double bits) and `__real@<8 hex>`
/// (IEEE-754 float bits — what today's MSVC emits for a `float` literal).
pub fn decode(name: &str) -> Option<FpConst> {
    let rest = name.strip_prefix("__real@")?;
    if let Some((size_str, hex)) = rest.split_once('@') {
        let size: usize = match size_str {
            "4" => 4,
            "8" => 8,
            _ => return None, // no other storage width is a float or a double
        };
        if !hex.chars().all(|c| c.is_ascii_hexdigit()) {
            return None;
        }
        let v = x87_extended_to_f64(hex)?;
        if size == 4 {
            let narrow = v as f32;
            // A value outside `float` range narrows to inf, and a value below
            // its smallest subnormal flushes to zero — both wrong data.
            if !narrow.is_finite() || (narrow == 0.0 && v != 0.0) {
                return None;
            }
            return Some(FpConst { bytes: narrow.to_le_bytes().to_vec(), size });
        }
        return Some(FpConst { bytes: v.to_le_bytes().to_vec(), size });
    }
    if !rest.chars().all(|c| c.is_ascii_hexdigit()) {
        return None;
    }
    // The bare-bits forms are copied through verbatim, so they need no value
    // interpretation at all (an Inf or a NaN literal round-trips exactly).
    match rest.len() {
        16 => {
            let bits = u64::from_str_radix(rest, 16).ok()?;
            Some(FpConst { bytes: bits.to_le_bytes().to_vec(), size: 8 })
        }
        8 => {
            let bits = u32::from_str_radix(rest, 16).ok()?;
            Some(FpConst { bytes: bits.to_le_bytes().to_vec(), size: 4 })
        }
        _ => None,
    }
}

/// What [`plan`] found in one relocatable object.
#[derive(Default)]
pub struct FpConstPlan {
    /// `(vma, little-endian bytes)` to materialise at a synthetic extern slot
    /// that has no backing today.
    pub writes: Vec<(u64, Vec<u8>)>,
    /// `[start, stop]` (inclusive) ranges the engine may fold with the
    /// program-wide `option readonly` still off — the materialised slots plus
    /// the defined COMDATs whose bytes match their own name.
    pub const_ranges: Vec<(u64, u64)>,
    /// Non-fatal diagnostics (a defined COMDAT whose bytes contradict its name).
    pub warnings: Vec<String>,
}

/// Find every `__real@` constant in a relocatable object and place it in the
/// loaded image: decoded bytes for the undefined half, foldable ranges for both
/// halves. Empty when the gate is off or the object carries no such symbol.
///
/// Addresses are deduped: an aliased symbol must not plant a second segment over
/// the first, and the range list the engine binary-searches has to stay disjoint.
pub fn plan(file: &object::File, layout: &RelocLayout) -> FpConstPlan {
    let mut out = FpConstPlan::default();
    if !kuna_decomp::kuna_msvcfpconst::msvcfpconst_enabled() {
        return out;
    }
    let mut seen: std::collections::HashSet<u64> = std::collections::HashSet::new();
    for sym in file.symbols() {
        let Ok(name) = sym.name() else { continue };
        let Some(c) = decode(name) else { continue };
        match sym.section() {
            // Undefined: bound to a synthetic extern slot with no bytes behind
            // it. Materialise the value the linker would have supplied.
            SymbolSection::Undefined | SymbolSection::Common => {
                let Some(&addr) = layout.extern_addr.get(&sym.index()) else { continue };
                if !seen.insert(addr) {
                    continue;
                }
                out.const_ranges.push((addr, addr + c.size as u64 - 1));
                out.writes.push((addr, c.bytes));
            }
            // Defined here: the bytes are already mapped and read-only, but the
            // fold is gated. Admit the range only once the bytes confirm the
            // name — a disagreement means one of the two is not what it claims.
            SymbolSection::Section(sec_idx) => {
                let Some(&base) = layout.section_vma.get(&sec_idx) else { continue };
                let off = sym.address() as usize;
                let Ok(sec) = file.section_by_index(sec_idx) else { continue };
                let Ok(data) = sec.data() else { continue };
                // Checked: `st_value` is attacker-controlled data no header
                // validates, and a wrapped `off + size` would index backwards.
                let Some(end) = off.checked_add(c.size) else { continue };
                if end > data.len() || data[off..end] != c.bytes[..] {
                    out.warnings.push(format!(
                        "{name}: defined bytes contradict the mangled value (not folded)"
                    ));
                    continue;
                }
                let addr = base.wrapping_add(sym.address());
                if seen.insert(addr) {
                    out.const_ranges.push((addr, addr + c.size as u64 - 1));
                }
            }
            _ => {}
        }
    }
    out
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
    ///
    /// The expected values are **exact** — each was re-derived with rational
    /// arithmetic and is the `f64` the datum denotes to the last bit, so the
    /// comparison is `==` rather than a tolerance. Two are not the round numbers
    /// they look like: `3fe08000000200000000` is 2^-31 + 2^-61, not 2^-31, and
    /// `3fef8000800000000000` is 2^-16 + 2^-32, not 2^-16 (both are the leading
    /// terms of a reciprocal the compiler folded).
    #[test]
    fn decodes_the_constants_from_a_real_msvc_object() {
        let cases: &[(&str, f64)] = &[
            ("__real@8@3fff8000000000000000", 1.0),
            ("__real@8@4000a000000000000000", 2.5),
            ("__real@8@4000c000000000000000", 3.0),
            ("__real@8@00000000000000000000", 0.0),
            ("__real@8@3ffec90fdaa22168c000", std::f64::consts::FRAC_PI_4),
            ("__real@8@4001c90fdaa22168c000", std::f64::consts::TAU),
            // 2^-31 + 2^-61: the scale turning a 31-bit random integer into [0,1).
            ("__real@8@3fe08000000200000000", 4.656612877414201e-10),
            ("__real@8@bff39d495182a9930800", -0.0003),
            ("__real@8@bff2d1b71758e2196800", -0.0002),
        ];
        for (name, want) in cases {
            let c = decode(name).unwrap_or_else(|| panic!("{name} must decode"));
            assert_eq!(c.size, 8, "{name}");
            assert_eq!(value_of(&c), *want, "{name}");
        }
    }

    /// The `@4@` form stores a 4-byte `float`, so the datum must be narrowed --
    /// storing 8 bytes would overwrite the neighbouring slot and read back as a
    /// wildly different number.
    #[test]
    fn the_size_field_selects_the_storage_width() {
        // 2^-16 + 2^-32, spelled as an 80-bit datum but stored as a float. Both
        // terms survive the narrowing (17 significand bits, `float` has 24), so
        // this is exact too.
        let c = decode("__real@4@3fef8000800000000000").expect("must decode");
        assert_eq!(c.size, 4);
        assert_eq!(c.bytes.len(), 4);
        assert_eq!(value_of(&c), 1.5259021893143654e-05);

        let c = decode("__real@4@40069600000000000000").expect("must decode");
        assert_eq!(c.size, 4);
        assert_eq!(value_of(&c), 150.0);

        let c = decode("__real@8@40069600000000000000").expect("must decode");
        assert_eq!(c.size, 8);
        assert_eq!(c.bytes.len(), 8);
        assert_eq!(value_of(&c), 150.0);
    }

    /// Sign is carried by the top bit of the exponent field.
    #[test]
    fn negative_constants_decode() {
        let c = decode("__real@8@bff39d495182a9930800").expect("must decode");
        assert_eq!(value_of(&c), -0.0003);
    }

    /// The bare-bits spellings: 16 hex digits are `double` bits, 8 are `float`
    /// bits. The 8-hex form is what MSVC has emitted for a `float` literal since
    /// VS2005, so refusing it drops every float constant in a modern object.
    #[test]
    fn the_bare_ieee_forms_decode() {
        let c = decode("__real@3ff0000000000000").expect("must decode");
        assert_eq!(c.size, 8);
        assert_eq!(value_of(&c), 1.0);

        let c = decode("__real@3f800000").expect("must decode");
        assert_eq!(c.size, 4);
        assert_eq!(c.bytes, vec![0x00, 0x00, 0x80, 0x3f]);
        assert_eq!(value_of(&c), 1.0);

        let c = decode("__real@bf000000").expect("must decode");
        assert_eq!(c.size, 4);
        assert_eq!(value_of(&c), -0.5);
    }

    /// The x87 encoding classes that have no faithful `f64` image are refused,
    /// not approximated: an Inf/NaN exponent used to scale silently to `+inf`,
    /// and a denormal used to decode two times too large.
    #[test]
    fn unrepresentable_x87_encodings_are_refused() {
        for name in [
            "__real@8@7fff8000000000000000", // +inf
            "__real@8@ffff8000000000000000", // -inf
            "__real@8@7fffc000000000000000", // quiet NaN
            "__real@8@00000000000000000001", // denormal (true scale 2^(-16382-63))
            "__real@8@00008000000000000000", // pseudo-denormal
            "__real@8@40060000000000000000", // unnormal: integer bit clear
            "__real@8@7ffe8000000000000000", // 2^16383 -- overflows f64
            "__real@4@414b8000000000000000", // 2^332 -- fine as f64, inf as f32
            "__real@4@3f378000000000000000", // 2^-200 -- fine as f64, flushes to 0 as f32
        ] {
            assert!(decode(name).is_none(), "{name} must NOT decode");
        }
        // A signed zero is a zero in both encodings, and stays decodable.
        assert_eq!(value_of(&decode("__real@8@80000000000000000000").unwrap()), 0.0);
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
            "__real@8@",                      // no payload
            "__real@8@3fff80000000000000",    // 18 digits, not 20
            "__real@8@zzzzzzzzzzzzzzzzzzzz",  // not hex
            "__real@16@3fff8000000000000000", // no 16-byte float storage
            "__real@08@3fff8000000000000000", // the width is spelled `8`, exactly
            "__xmm@00000000000000000000000000000000",
            "__real@3ff00000000000",   // 14 digits: neither 8 nor 16
            "__real@3ff000000000000",  // 15 digits
            "__real@3f80000",          // 7 digits
            "__real@3f8000000",        // 9 digits
        ] {
            assert!(decode(name).is_none(), "{name} must NOT decode");
        }
    }
}
